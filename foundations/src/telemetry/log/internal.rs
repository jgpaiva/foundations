use super::init::LogHarness;
use crate::telemetry::scope::Scope;
use slog::{Logger, OwnedKV, SendSyncRefUnwindSafeKV};
use std::backtrace::Backtrace;
use std::ops::Deref;
use std::sync::Arc;

// NOTE: we intentionally use a lock without poisoning here to not
// panic the threads if they just share telemetry with failed thread.
pub(crate) type SharedLog = Arc<parking_lot::RwLock<LoggerWithKvNestingTracking>>;

#[derive(Debug, Clone)]
pub struct LoggerWithKvNestingTracking {
    // The logger itself. This is the most important part of this struct. (We implement Deref to
    // let you go straight to this field, in contexts where you need a &Logger)
    pub(crate) inner: Logger,

    // KV nesting level. You should increment this (using the inc_nesting_level() method) every
    // time you replace the logger with a child of itself. You should likewise set this back to
    // zero if you replace the logger with a "root" logger that doesn't have any nested KVs in it.
    // (That said, accuracy is not critical, as this is only used as a safety check)
    pub(crate) nesting_level: u32,

    // One stacktrace captured per nesting increment. Stored as Arc so that cloning this struct
    // (e.g. in fork_log) is cheap and does not deep-copy backtrace data.
    pub(crate) nesting_stacktraces: Vec<Arc<(Backtrace, String)>>,

    // The name assigned to this logger, used in diagnostic output. The root logger is named
    // "ROOT"; each fork takes the name passed to fork_log().
    pub(crate) name: String,

    // The chain of log names from the root down to this logger, in creation order. Always starts
    // with "ROOT" and each call to fork_log() appends the new fork's name. Emitted alongside the
    // nesting-exceeded error so that the full fork ancestry is visible.
    pub(crate) lineage: Vec<String>,
}

impl LoggerWithKvNestingTracking {
    pub const MAX_NESTING: u32 = 200;
    pub const EXCEEDED_MAX_NESTING_ERROR: &'static str = "foundations: maximum logger KV nesting exceeded (are add_fields! or set_verbosity being called in a loop?)";

    /// Create a new LoggerWithKvNestingTracking based on a fresh logger. The KV nesting level is
    /// initialized to zero.
    pub(crate) fn new(logger: Logger) -> Self {
        Self {
            inner: logger,
            nesting_level: 0,
            nesting_stacktraces: Vec::new(),
            name: "ROOT".into(),
            lineage: vec!["ROOT".to_string()],
        }
    }

    /// Increment the KV nesting level. You should call this before any time you're going to replace the
    /// logger with a child of itself.
    ///
    /// If this returns None, it will consume the logger lock, and you should not nest any further.
    /// If panic_on_too_much_logger_nesting is enabled, instead of returning None this will free the
    /// logger lock and then panic.
    pub(crate) fn check_nesting_level(
        mut current_log_lock: parking_lot::lock_api::RwLockWriteGuard<
            parking_lot::RawRwLock,
            LoggerWithKvNestingTracking,
        >,
    ) -> Option<
        parking_lot::lock_api::RwLockWriteGuard<
            parking_lot::RawRwLock,
            LoggerWithKvNestingTracking,
        >,
    > {
        current_log_lock.nesting_level = current_log_lock.nesting_level.saturating_add(1);
        let log_name = current_log_lock.name.clone();
        current_log_lock
            .nesting_stacktraces
            .push(Arc::new((Backtrace::capture(), log_name)));

        match current_log_lock.nesting_level {
            0..Self::MAX_NESTING => Some(current_log_lock), // continue with operation
            Self::MAX_NESTING => {
                // Drop the lock guard before panicking
                if cfg!(feature = "panic_on_too_much_logger_nesting") {
                    drop(current_log_lock);
                    panic!("{}", Self::EXCEEDED_MAX_NESTING_ERROR);
                } else {
                    slog::error!(current_log_lock, "{}", Self::EXCEEDED_MAX_NESTING_ERROR;
                        "lineage" => format!("{:?}", current_log_lock.lineage));
                    // Use a bare logger rooted directly on the drain so that the
                    // stacktrace records carry no accumulated KV context from the
                    // current log (that context is already in the error above).
                    let bare_logger =
                        Logger::root(Arc::clone(&LogHarness::get().root_drain), slog::o!());
                    for (i, bt) in current_log_lock.nesting_stacktraces.iter().enumerate() {
                        let bt = Arc::clone(bt);
                        // Strip the leading internal frames up to and including
                        // `check_nesting_level` itself — the caller already knows
                        // where the backtrace was captured; those frames only add
                        // noise.
                        let bt_str = bt.0.to_string();
                        let trimmed = trim_backtrace_to_caller(&bt_str);
                        slog::error!(bare_logger,
                            "nesting bt d={} l={}", i + 1, bt.1;
                            "backtrace" => trimmed,
                        );
                    }
                    None // avoid further nesting
                }
            }
            _ => None, // avoid further nesting
        }
    }
}

impl Deref for LoggerWithKvNestingTracking {
    type Target = Logger;

    #[inline]
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

#[must_use]
pub(crate) struct LogScope {
    _inner: Scope<SharedLog>,
}

impl LogScope {
    #[inline]
    pub(crate) fn new(log: SharedLog) -> Self {
        Self {
            _inner: Scope::new(&LogHarness::get().log_scope_stack, log),
        }
    }
}

pub fn add_log_fields<T>(fields: OwnedKV<T>)
where
    T: SendSyncRefUnwindSafeKV + 'static,
{
    let log = current_log();
    let log_lock = log.write();

    let Some(mut log_lock) = LoggerWithKvNestingTracking::check_nesting_level(log_lock) else {
        return; // avoid changes, nesting level was beyond threshold
    };

    log_lock.inner = log_lock.inner.new(fields);
}

pub fn current_log() -> SharedLog {
    let harness = LogHarness::get();
    let log = harness.log_scope_stack.current();

    log.unwrap_or_else(|| Arc::clone(&harness.root_log))
}

pub(crate) fn fork_log(log_name: &str) -> SharedLog {
    let parent = current_log();
    let mut log = parent.read().clone();
    log.name = log_name.into();
    log.lineage.push(log_name.to_string());

    Arc::new(parking_lot::RwLock::new(log))
}

/// Replace a cargo git checkout path of the form
/// `<prefix>/.cargo/git/checkouts/foundations-<hash>/<hash>/foundations`
/// with the bare token `foundations`.
///
/// The two hash segments are variable, so this matches by finding the fixed
/// anchor `.cargo/git/checkouts/` and then scanning forward for the
/// `/foundations` boundary that ends the checkout path.
fn collapse_cargo_git_checkout_path(s: &str) -> String {
    const ANCHOR: &str = ".cargo/git/checkouts/";
    const SUFFIX: &str = "/foundations";

    let mut result = String::with_capacity(s.len());
    let mut rest = s;

    while let Some(anchor_pos) = rest.find(ANCHOR) {
        // Find where the segment containing the anchor begins: walk backwards
        // to the last whitespace (or start of string) so we strip the host
        // path (e.g. `./home/gitlab-runner/`) along with the checkout path.
        let segment_start = rest[..anchor_pos]
            .rfind(|c: char| c.is_whitespace())
            .map(|p| p + 1)
            .unwrap_or(0);
        result.push_str(&rest[..segment_start]);

        let after_anchor = &rest[anchor_pos + ANCHOR.len()..];

        // Find the `/foundations` boundary that ends the checkout path.
        // The structure is `<crate-name>-<hash>/<rev-hash>/foundations`.
        if let Some(suffix_pos) = after_anchor.find(SUFFIX) {
            result.push_str("foundations");
            rest = &after_anchor[suffix_pos + SUFFIX.len()..];
        } else {
            // No closing /foundations found; emit unchanged and stop.
            result.push_str(ANCHOR);
            rest = after_anchor;
        }
    }

    result.push_str(rest);
    result
}

/// Clean up a captured backtrace string for diagnostic output:
///
/// - Skips all lines up to and including the `check_nesting_level` frame (frame
///   0 in practice), since the reader already knows where the backtrace was
///   captured.
/// - Strips leading whitespace from every line.
/// - Removes the `./cfsetup_build/` path prefix that the build system injects
///   into source locations, leaving only the relative path within the crate.
///
/// If the `check_nesting_level` marker is not found (e.g. backtraces are
/// disabled or the symbol was inlined) the full string is returned after
/// applying the whitespace and path cleaning steps.
fn trim_backtrace_to_caller(bt: &str) -> String {
    // Each backtrace frame spans two lines: the symbol name line and the
    // `at file:line:col` line beneath it. We want to drop both lines of the
    // check_nesting_level frame, so we track whether we are on the symbol line
    // or the following `at` line.
    let mut skip_next = false;
    bt.lines()
        .skip_while(|line| {
            if skip_next {
                // This is the `at` line belonging to check_nesting_level; drop
                // it too, then stop skipping.
                skip_next = false;
                return true;
            }
            if line.contains("check_nesting_level") {
                skip_next = true;
                return true;
            }
            false
        })
        .map(|line| {
            let line = line.trim_start();
            // Strip build-system-injected path prefix so source locations are
            // readable without the noise of an absolute build sandbox path.
            let line = line.replace("./cfsetup_build/", "");
            // Collapse the cargo git checkout path
            // (e.g. `./home/.../.cargo/git/checkouts/foundations-<hash>/<hash>/foundations`)
            // down to just `foundations`, regardless of the specific hashes.
            collapse_cargo_git_checkout_path(&line)
        })
        .collect::<Vec<_>>()
        .join("\n")
}

#[cfg(test)]
mod tests {
    #[cfg(not(feature = "panic_on_too_much_logger_nesting"))]
    use super::LoggerWithKvNestingTracking;
    use crate::telemetry::TelemetryContext;
    use crate::telemetry::log::{self, internal::current_log};
    use foundations_macros::with_test_telemetry;

    // -------------------------------------------------------------------------
    // trim_backtrace_to_caller
    // -------------------------------------------------------------------------

    /// Verifies all transformations applied by trim_backtrace_to_caller using a
    /// representative slice of real backtrace output:
    ///   1. Lines up to and including the check_nesting_level frame are dropped.
    ///   2. Leading whitespace is stripped from every remaining line.
    ///   3. The ./cfsetup_build/ path prefix is removed from source locations.
    ///   4. Cargo git checkout paths (with variable hashes) are collapsed to
    ///      just `foundations`.
    #[test]
    fn trim_backtrace_to_caller_drops_internal_frames_and_cleans_output() {
        let input = concat!(
            "   0: foundations::telemetry::log::internal::LoggerWithKvNestingTracking::check_nesting_level\n",
            "             at ./home/gitlab-runner/.cargo/git/checkouts/foundations-9131e3ca38eb3824/838767a/foundations/src/telemetry/log/internal.rs:63:28\n",
            "   1: foundations::telemetry::log::internal::add_log_fields\n",
            "             at ./home/gitlab-runner/.cargo/git/checkouts/foundations-9131e3ca38eb3824/838767a/foundations/src/telemetry/log/internal.rs:119:30\n",
            "   2: gateway_firewall::body_scanning::client::scan_hyper_body::{{closure}}::{{closure}}\n",
            "             at ./cfsetup_build/lib/gateway-firewall/src/body_scanning/client.rs:227:9\n",
            "   3: <foundations::telemetry::telemetry_context::WithTelemetryContext<T> as core::future::future::Future>::poll\n",
            "             at ./home/gitlab-runner/.cargo/git/checkouts/foundations-9131e3ca38eb3824/838767a/foundations/src/telemetry/telemetry_context.rs:42:29",
        );

        let expected = concat!(
            "1: foundations::telemetry::log::internal::add_log_fields\n",
            "at foundations/src/telemetry/log/internal.rs:119:30\n",
            "2: gateway_firewall::body_scanning::client::scan_hyper_body::{{closure}}::{{closure}}\n",
            "at lib/gateway-firewall/src/body_scanning/client.rs:227:9\n",
            "3: <foundations::telemetry::telemetry_context::WithTelemetryContext<T> as core::future::future::Future>::poll\n",
            "at foundations/src/telemetry/telemetry_context.rs:42:29",
        );

        assert_eq!(super::trim_backtrace_to_caller(input), expected);
    }

    /// When the check_nesting_level marker is absent (e.g. backtraces disabled
    /// or symbol inlined), the function still strips whitespace and paths.
    #[test]
    fn trim_backtrace_to_caller_falls_back_when_marker_absent() {
        let input = concat!(
            "   0: some::other::function\n",
            "             at ./cfsetup_build/lib/foo/src/bar.rs:10:5",
        );

        let expected = concat!("0: some::other::function\n", "at lib/foo/src/bar.rs:10:5",);

        assert_eq!(super::trim_backtrace_to_caller(input), expected);
    }

    // -------------------------------------------------------------------------
    // Fork name
    // -------------------------------------------------------------------------

    /// A freshly initialised root log should be named "ROOT".
    #[with_test_telemetry(test, crate_path = "crate")]
    fn root_log_has_root_name(_ctx: crate::telemetry::TestTelemetryContext) {
        let log = current_log();
        let guard = log.read();
        assert_eq!(guard.name, "ROOT");
    }

    /// After forking with an explicit name, the fork's name should equal the given name.
    #[with_test_telemetry(test, crate_path = "crate")]
    fn forked_log_takes_given_name(_ctx: crate::telemetry::TestTelemetryContext) {
        let fork_ctx = TelemetryContext::current().with_forked_log_named("my-fork");
        let _scope = fork_ctx.scope();

        let log = current_log();
        let guard = log.read();
        assert_eq!(guard.name, "my-fork");
    }

    /// Forking must not change the name of the parent log.
    #[with_test_telemetry(test, crate_path = "crate")]
    fn forking_does_not_change_parent_name(_ctx: crate::telemetry::TestTelemetryContext) {
        let parent_log = current_log();

        let _fork_ctx = TelemetryContext::current().with_forked_log_named("child");

        let guard = parent_log.read();
        assert_eq!(guard.name, "ROOT");
    }

    // -------------------------------------------------------------------------
    // Fork lineage
    // -------------------------------------------------------------------------

    /// The root log's lineage is exactly ["ROOT"].
    #[with_test_telemetry(test, crate_path = "crate")]
    fn root_log_has_single_element_lineage(_ctx: crate::telemetry::TestTelemetryContext) {
        let log = current_log();
        let guard = log.read();
        assert_eq!(guard.lineage, vec!["ROOT".to_string()]);
    }

    /// A single fork appends its name to the root lineage.
    #[with_test_telemetry(test, crate_path = "crate")]
    fn single_fork_appends_to_lineage(_ctx: crate::telemetry::TestTelemetryContext) {
        let fork_ctx = TelemetryContext::current().with_forked_log_named("level-1");
        let _scope = fork_ctx.scope();

        let log = current_log();
        let guard = log.read();
        assert_eq!(
            guard.lineage,
            vec!["ROOT".to_string(), "level-1".to_string()]
        );
    }

    /// Forking a fork produces a lineage that includes every step in the chain.
    #[with_test_telemetry(test, crate_path = "crate")]
    fn nested_forks_accumulate_lineage(_ctx: crate::telemetry::TestTelemetryContext) {
        let lvl1_ctx = TelemetryContext::current().with_forked_log_named("level-1");
        let _scope1 = lvl1_ctx.scope();

        let lvl2_ctx = TelemetryContext::current().with_forked_log_named("level-2");
        let _scope2 = lvl2_ctx.scope();

        let log = current_log();
        let guard = log.read();
        assert_eq!(
            guard.lineage,
            vec![
                "ROOT".to_string(),
                "level-1".to_string(),
                "level-2".to_string(),
            ]
        );
    }

    /// Forking must not modify the parent log's lineage.
    #[with_test_telemetry(test, crate_path = "crate")]
    fn forking_does_not_mutate_parent_lineage(_ctx: crate::telemetry::TestTelemetryContext) {
        let parent_log = current_log();

        // Create a fork but don't activate its scope.
        let _fork_ctx = TelemetryContext::current().with_forked_log_named("child");

        let guard = parent_log.read();
        assert_eq!(guard.lineage, vec!["ROOT".to_string()]);
    }

    // -------------------------------------------------------------------------
    // Nesting limit: stacktrace log records emitted on breach
    // -------------------------------------------------------------------------

    /// When the nesting limit is reached without `panic_on_too_much_logger_nesting`
    /// enabled, the logger must emit one error record for the summary message and
    /// one error record per nesting increment, each carrying the correct
    /// `nesting_depth` and `log_name` fields.
    ///
    /// This test only exercises the log-and-continue path and is therefore
    /// gated on the feature being absent. Run it with:
    ///
    /// ```text
    /// cargo nextest run -p foundations --no-default-features \
    ///     --features settings,telemetry,testing --lib \
    ///     nesting_limit_emits_stacktrace_records_with_log_name
    /// ```
    #[cfg(not(feature = "panic_on_too_much_logger_nesting"))]
    #[with_test_telemetry(test, crate_path = "crate")]
    fn nesting_limit_emits_stacktrace_records_with_log_name(
        ctx: crate::telemetry::TestTelemetryContext,
    ) {
        use crate::telemetry::log::set_verbosity;
        use crate::telemetry::settings::LogVerbosity;

        let fork_ctx = TelemetryContext::current().with_forked_log_named("overflowing-fork");
        let _scope = fork_ctx.scope();

        // Each iteration increments nesting twice (add_fields + set_verbosity),
        // so MAX_NESTING / 2 + 1 iterations is enough to hit the limit.
        let _ = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            for _ in 0..=(LoggerWithKvNestingTracking::MAX_NESTING / 2) {
                log::add_fields!("k" => "v");
                let _ = set_verbosity(LogVerbosity::Info);
            }
        }));

        let records = ctx.log_records();

        // The summary error record carries the lineage but no per-frame detail.
        let summary = records
            .iter()
            .find(|r| r.message == LoggerWithKvNestingTracking::EXCEEDED_MAX_NESTING_ERROR)
            .expect("summary nesting-exceeded error record not found");
        assert_eq!(summary.level, slog::Level::Error);

        // There should be MAX_NESTING stacktrace records, one per nesting
        // increment up to the limit. The depth and log name are encoded in the
        // message rather than as separate fields.
        let stacktrace_records: Vec<_> = records
            .iter()
            .filter(|r| r.message.starts_with("nesting bt d="))
            .collect();
        assert_eq!(
            stacktrace_records.len(),
            LoggerWithKvNestingTracking::MAX_NESTING as usize,
            "expected one stacktrace record per nesting increment"
        );

        // Every stacktrace record must embed the fork name and the correct
        // depth in its message, and must NOT carry log_name/nesting_depth as
        // separate fields (those were moved into the message).
        //
        // Critically, the records must also be free of the accumulated KV
        // context fields (e.g. "k") that were added to the forked log. Their
        // absence confirms the bare logger (rooted directly on the drain, with
        // no KV context) was used rather than the context-laden current log.
        for (i, record) in stacktrace_records.iter().enumerate() {
            let expected_msg = format!("nesting bt d={} l=overflowing-fork", i + 1);
            assert_eq!(
                record.message, expected_msg,
                "stacktrace record {} has unexpected message",
                i
            );
            assert!(
                record.fields.iter().all(|(k, _)| k != "log_name"),
                "stacktrace record {} should not have a log_name field; got: {:?}",
                i,
                record.fields
            );
            assert!(
                record.fields.iter().all(|(k, _)| k != "nesting_depth"),
                "stacktrace record {} should not have a nesting_depth field; got: {:?}",
                i,
                record.fields
            );
            assert!(
                record.fields.iter().all(|(k, _)| k != "k"),
                "stacktrace record {} should not carry the fork's KV context (bare logger expected); got: {:?}",
                i,
                record.fields
            );
        }
    }

    // -------------------------------------------------------------------------
    // Nesting stacktrace log_name tracking
    // -------------------------------------------------------------------------

    /// Each nesting stacktrace entry must record the name of the log that was
    /// current at the time the nesting increment occurred.
    #[with_test_telemetry(test, crate_path = "crate")]
    fn nesting_stacktrace_records_log_name(_ctx: crate::telemetry::TestTelemetryContext) {
        let fork_ctx = TelemetryContext::current().with_forked_log_named("named-fork");
        let _scope = fork_ctx.scope();

        // Trigger one nesting increment by adding a field.
        log::add_fields!("k" => "v");

        let log = current_log();
        let guard = log.read();
        assert_eq!(guard.nesting_stacktraces.len(), 1);
        assert_eq!(
            guard.nesting_stacktraces[0].1, "named-fork",
            "stacktrace entry should carry the fork name"
        );
    }

    /// When the same logger is nested multiple times its name appears in every
    /// stacktrace entry, making it easy to identify the offending fork in the
    /// nesting-exceeded error output.
    #[with_test_telemetry(test, crate_path = "crate")]
    fn nesting_stacktrace_consistently_records_fork_name(
        _ctx: crate::telemetry::TestTelemetryContext,
    ) {
        let fork_ctx = TelemetryContext::current().with_forked_log_named("consistent-fork");
        let _scope = fork_ctx.scope();

        log::add_fields!("k1" => "v1");
        log::add_fields!("k2" => "v2");

        let log = current_log();
        let guard = log.read();
        assert_eq!(guard.nesting_stacktraces.len(), 2);
        for entry in &guard.nesting_stacktraces {
            assert_eq!(
                entry.1, "consistent-fork",
                "every stacktrace entry should record the fork name"
            );
        }
    }
}
