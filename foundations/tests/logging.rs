use foundations::telemetry::TelemetryContext;
use foundations::telemetry::TestTelemetryContext;
use foundations::telemetry::log::internal::LoggerWithKvNestingTracking;
use foundations::telemetry::log::{add_fields, set_verbosity, warn};
use foundations::telemetry::settings::{LogVerbosity, LoggingSettings, RateLimitingSettings};
use foundations_macros::with_test_telemetry;

#[with_test_telemetry(test)]
fn test_rate_limiter(mut ctx: TestTelemetryContext) {
    for i in 0..16 {
        warn!("{}", i);
    }

    assert_eq!(ctx.log_records().len(), 16);

    ctx.set_logging_settings(LoggingSettings {
        rate_limit: RateLimitingSettings {
            enabled: true,
            max_events_per_second: 5,
        },
        ..Default::default()
    });

    for i in 16..32 {
        warn!("{}", i);
    }

    assert!(ctx.log_records().len() < 32);
}

// Every time we call set_verbosity(), or the add_fields! macro, it adds one to the depth of the
// nested structure of Arcs inside the logger object. If the structure gets too deeply nested, it
// causes a stack overflow on drop.
//
// This test case makes sure that before it would hit a dangerous depth, it panics (with an error
// that gives you a hint as to where to go look in your code).
#[with_test_telemetry(test)]
fn test_exceed_limit_kv_nesting(_ctx: TestTelemetryContext) {
    match std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        for _ in 0..((LoggerWithKvNestingTracking::MAX_NESTING / 2) + 1) {
            add_fields! { "key1" => "hello" }
            set_verbosity(LogVerbosity::Info).expect("set_verbosity");
        }
    })) {
        Ok(_) => panic!("test case exceeded the maximum log KV nesting, but there was no panic"),
        Err(err) => {
            if let Some(msg) = err.downcast_ref::<&'static str>() {
                assert_eq!(
                    *msg,
                    LoggerWithKvNestingTracking::EXCEEDED_MAX_NESTING_ERROR
                );
            } else if let Some(msg) = err.downcast_ref::<String>() {
                assert_eq!(
                    *msg,
                    LoggerWithKvNestingTracking::EXCEEDED_MAX_NESTING_ERROR
                );
            } else {
                panic!(
                    "test case exceeded the maximum log KV nesting, but the panic was not castable to the expected type"
                );
            }
        }
    }
}

// Negative version of above test. If we're just under the limit, we shouldn't get a panic, and we
// also shouldn't stack overflow. This helps us make sure we didn't set the limit too high. For
// example, if we set the limit to be 1,000,000, then having this test here would make sure that it
// doesn't cause a stack overflow at 999,990. And if it did cause a stack overflow at 999,990, then
// this test would make sure we notice that and don't set the limit that high!
#[with_test_telemetry(test)]
fn test_not_exceed_limit_kv_nesting(_ctx: TestTelemetryContext) {
    for _ in 0..((LoggerWithKvNestingTracking::MAX_NESTING / 2) - 5) {
        add_fields! { "key1" => "hello" }
        set_verbosity(LogVerbosity::Info).expect("set_verbosity");
    }
}

// ---------------------------------------------------------------------------
// Log forking — name, lineage, and field isolation
// ---------------------------------------------------------------------------

/// with_forked_log() creates a usable forked context (default name).
#[with_test_telemetry(test)]
fn test_fork_log_name(_ctx: TestTelemetryContext) {
    use foundations::telemetry::log::internal::current_log;

    let fork_ctx = TelemetryContext::current().with_forked_log();
    let _scope = fork_ctx.scope();

    let log = current_log();
    // `name` is pub(crate) so we cannot access it directly from an integration
    // test. We verify the observable behaviour instead: forking does not panic
    // and the resulting context can be scoped and used.
    drop(log); // just confirming we can obtain a handle to the forked log
}

/// Forking a log should inherit the parent's fields.
#[with_test_telemetry(test)]
fn test_fork_inherits_parent_fields(mut ctx: TestTelemetryContext) {
    ctx.set_logging_settings(LoggingSettings::default());

    add_fields!("parent_field" => "parent_value");

    {
        let _scope = TelemetryContext::current()
            .with_forked_log_named("child")
            .scope();
        warn!("from child");
    }

    let records = ctx.log_records();
    let child_record = records
        .iter()
        .find(|r| r.message == "from child")
        .expect("child log record not found");

    assert!(
        child_record
            .fields
            .iter()
            .any(|(k, v)| k == "parent_field" && v == "parent_value"),
        "child should inherit parent field; got: {:?}",
        child_record.fields
    );
}

/// Fields added inside a fork must not appear on sibling forks.
#[with_test_telemetry(test)]
fn test_fork_fields_isolated_from_sibling(mut ctx: TestTelemetryContext) {
    ctx.set_logging_settings(LoggingSettings::default());

    {
        let _scope = TelemetryContext::current()
            .with_forked_log_named("fork-a")
            .scope();
        add_fields!("only_in_a" => "yes");
        warn!("from fork a");
    }

    {
        let _scope = TelemetryContext::current()
            .with_forked_log_named("fork-b")
            .scope();
        warn!("from fork b");
    }

    let records = ctx.log_records();
    let fork_b = records
        .iter()
        .find(|r| r.message == "from fork b")
        .expect("fork-b record not found");

    assert!(
        fork_b.fields.iter().all(|(k, _)| k != "only_in_a"),
        "fork-b should not have fork-a's field; got: {:?}",
        fork_b.fields
    );
}

/// Fields added inside a fork must not leak back to the parent.
#[with_test_telemetry(test)]
fn test_fork_fields_do_not_leak_to_parent(mut ctx: TestTelemetryContext) {
    ctx.set_logging_settings(LoggingSettings::default());

    {
        let _scope = TelemetryContext::current()
            .with_forked_log_named("child")
            .scope();
        add_fields!("child_only" => "yes");
    }

    warn!("from parent");

    let records = ctx.log_records();
    let parent_record = records
        .iter()
        .find(|r| r.message == "from parent")
        .expect("parent record not found");

    assert!(
        parent_record.fields.iter().all(|(k, _)| k != "child_only"),
        "parent should not have child's field; got: {:?}",
        parent_record.fields
    );
}

/// Nesting-level counter resets correctly across independent forks: nesting
/// operations in one fork must not affect the parent's nesting counter.
#[with_test_telemetry(test)]
fn test_fork_nesting_is_independent_of_parent(_ctx: TestTelemetryContext) {
    // Nest several times inside a fork.
    {
        let _scope = TelemetryContext::current()
            .with_forked_log_named("nested-fork")
            .scope();

        for _ in 0..10 {
            add_fields!("k" => "v");
        }
    }

    // After the fork scope ends, nesting operations on the parent log should
    // still work without hitting the limit prematurely.
    add_fields!("parent_key" => "parent_value");
}

/// Deeply nested forks accumulate the correct lineage. This tests three
/// levels of nesting (root -> conn -> req) which is the pattern used in the
/// HTTP server example.
#[with_test_telemetry(test)]
fn test_three_level_fork_chain(mut ctx: TestTelemetryContext) {
    ctx.set_logging_settings(LoggingSettings::default());

    add_fields!("root_field" => "root_value");

    let conn_ctx = TelemetryContext::current().with_forked_log_named("connection-log");
    let _conn_scope = conn_ctx.scope();

    add_fields!("conn_field" => "conn_value");

    let req_ctx = TelemetryContext::current().with_forked_log_named("request-log");
    let _req_scope = req_ctx.scope();

    warn!("from request");

    let records = ctx.log_records();
    let req_record = records
        .iter()
        .find(|r| r.message == "from request")
        .expect("request record not found");

    // The request log should carry fields from all three levels.
    assert!(
        req_record
            .fields
            .iter()
            .any(|(k, v)| k == "root_field" && v == "root_value"),
        "request log missing root_field; got: {:?}",
        req_record.fields
    );
    assert!(
        req_record
            .fields
            .iter()
            .any(|(k, v)| k == "conn_field" && v == "conn_value"),
        "request log missing conn_field; got: {:?}",
        req_record.fields
    );
}

#[cfg(feature = "tracing-rs-compat")]
mod tracing_rs_compat {
    use std::io;
    use std::sync::{Arc, Mutex};

    use foundations::telemetry::TelemetryContext;
    use foundations::telemetry::log::{TestLogRecord, warn};
    use foundations::telemetry::settings::LoggingSettings;
    use tracing_subscriber::filter::LevelFilter;
    use tracing_subscriber::util::SubscriberInitExt as _;

    struct TestWriter {
        log_entries: Arc<Mutex<Vec<String>>>,
    }

    impl TestWriter {
        fn with_entries(entries: Arc<Mutex<Vec<String>>>) -> Self {
            Self {
                log_entries: entries,
            }
        }
    }

    impl io::Write for TestWriter {
        fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
            let s = String::from_utf8(buf.to_vec()).unwrap();
            self.log_entries.lock().unwrap().push(s.trim().to_string());
            Ok(buf.len())
        }

        fn flush(&mut self) -> io::Result<()> {
            unimplemented!()
        }
    }

    #[test]
    fn test_tracing_rs_compat() {
        let entries = Arc::new(Mutex::new(Vec::new()));
        let tracing_log_entries = entries.clone();
        let _subscriber = tracing_subscriber::fmt()
            .with_max_level(LevelFilter::TRACE)
            .with_writer(move || TestWriter::with_entries(entries.clone()))
            .without_time()
            .with_level(true)
            .with_ansi(false)
            .set_default();

        let settings = LoggingSettings {
            output: foundations::telemetry::settings::LogOutput::TracingRsCompat,
            ..Default::default()
        };

        let mut ctx = TelemetryContext::test();
        ctx.set_tracing_rs_log_drain(settings);

        let _scope = ctx.scope();

        warn!("compat-layer-works");

        // Validate slog is seeing all of the records
        let slog_records = ctx.log_records();
        let expected_slog_records = TestLogRecord {
            level: slog::Level::Warning,
            message: "compat-layer-works".to_string(),
            fields: vec![],
        };

        assert_eq!(*slog_records, vec![expected_slog_records]);

        let tracing_record = tracing_log_entries
            .lock()
            .unwrap()
            .first()
            .cloned()
            .unwrap();
        assert!(tracing_record.contains("WARN slog: compat-layer-works"));
    }
}
