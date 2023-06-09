// AUTOGENERATED by tools/gen-syscall-enum

/// Linux syscalls.
///
/// Note that this enum can have different variants for different processor architechtures.
#[allow(non_camel_case_types)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub enum Syscall {
    /// <https://man7.org/linux/man-pages/man2/_sysctl.2.html>
    _sysctl = 156,
    /// <https://man7.org/linux/man-pages/man2/accept.2.html>
    accept = 43,
    /// <https://man7.org/linux/man-pages/man2/accept4.2.html>
    accept4 = 288,
    /// <https://man7.org/linux/man-pages/man2/access.2.html>
    access = 21,
    /// <https://man7.org/linux/man-pages/man2/acct.2.html>
    acct = 163,
    /// <https://man7.org/linux/man-pages/man2/add_key.2.html>
    add_key = 248,
    /// <https://man7.org/linux/man-pages/man2/adjtimex.2.html>
    adjtimex = 159,
    /// <https://man7.org/linux/man-pages/man2/afs_syscall.2.html>
    afs_syscall = 183,
    /// <https://man7.org/linux/man-pages/man2/alarm.2.html>
    alarm = 37,
    /// <https://man7.org/linux/man-pages/man2/arch_prctl.2.html>
    arch_prctl = 158,
    /// <https://man7.org/linux/man-pages/man2/bind.2.html>
    bind = 49,
    /// <https://man7.org/linux/man-pages/man2/bpf.2.html>
    bpf = 321,
    /// <https://man7.org/linux/man-pages/man2/brk.2.html>
    brk = 12,
    /// <https://man7.org/linux/man-pages/man2/capget.2.html>
    capget = 125,
    /// <https://man7.org/linux/man-pages/man2/capset.2.html>
    capset = 126,
    /// <https://man7.org/linux/man-pages/man2/chdir.2.html>
    chdir = 80,
    /// <https://man7.org/linux/man-pages/man2/chmod.2.html>
    chmod = 90,
    /// <https://man7.org/linux/man-pages/man2/chown.2.html>
    chown = 92,
    /// <https://man7.org/linux/man-pages/man2/chroot.2.html>
    chroot = 161,
    /// <https://man7.org/linux/man-pages/man2/clock_adjtime.2.html>
    clock_adjtime = 305,
    /// <https://man7.org/linux/man-pages/man2/clock_getres.2.html>
    clock_getres = 229,
    /// <https://man7.org/linux/man-pages/man2/clock_gettime.2.html>
    clock_gettime = 228,
    /// <https://man7.org/linux/man-pages/man2/clock_nanosleep.2.html>
    clock_nanosleep = 230,
    /// <https://man7.org/linux/man-pages/man2/clock_settime.2.html>
    clock_settime = 227,
    /// <https://man7.org/linux/man-pages/man2/clone.2.html>
    clone = 56,
    /// <https://man7.org/linux/man-pages/man2/clone3.2.html>
    clone3 = 435,
    /// <https://man7.org/linux/man-pages/man2/close.2.html>
    close = 3,
    /// <https://man7.org/linux/man-pages/man2/close_range.2.html>
    close_range = 436,
    /// <https://man7.org/linux/man-pages/man2/connect.2.html>
    connect = 42,
    /// <https://man7.org/linux/man-pages/man2/copy_file_range.2.html>
    copy_file_range = 326,
    /// <https://man7.org/linux/man-pages/man2/creat.2.html>
    creat = 85,
    /// <https://man7.org/linux/man-pages/man2/create_module.2.html>
    create_module = 174,
    /// <https://man7.org/linux/man-pages/man2/delete_module.2.html>
    delete_module = 176,
    /// <https://man7.org/linux/man-pages/man2/dup.2.html>
    dup = 32,
    /// <https://man7.org/linux/man-pages/man2/dup2.2.html>
    dup2 = 33,
    /// <https://man7.org/linux/man-pages/man2/dup3.2.html>
    dup3 = 292,
    /// <https://man7.org/linux/man-pages/man2/epoll_create.2.html>
    epoll_create = 213,
    /// <https://man7.org/linux/man-pages/man2/epoll_create1.2.html>
    epoll_create1 = 291,
    /// <https://man7.org/linux/man-pages/man2/epoll_ctl.2.html>
    epoll_ctl = 233,
    /// <https://man7.org/linux/man-pages/man2/epoll_ctl_old.2.html>
    epoll_ctl_old = 214,
    /// <https://man7.org/linux/man-pages/man2/epoll_pwait.2.html>
    epoll_pwait = 281,
    /// <https://man7.org/linux/man-pages/man2/epoll_pwait2.2.html>
    epoll_pwait2 = 441,
    /// <https://man7.org/linux/man-pages/man2/epoll_wait.2.html>
    epoll_wait = 232,
    /// <https://man7.org/linux/man-pages/man2/epoll_wait_old.2.html>
    epoll_wait_old = 215,
    /// <https://man7.org/linux/man-pages/man2/eventfd.2.html>
    eventfd = 284,
    /// <https://man7.org/linux/man-pages/man2/eventfd2.2.html>
    eventfd2 = 290,
    /// <https://man7.org/linux/man-pages/man2/execve.2.html>
    execve = 59,
    /// <https://man7.org/linux/man-pages/man2/execveat.2.html>
    execveat = 322,
    /// <https://man7.org/linux/man-pages/man2/exit.2.html>
    exit = 60,
    /// <https://man7.org/linux/man-pages/man2/exit_group.2.html>
    exit_group = 231,
    /// <https://man7.org/linux/man-pages/man2/faccessat.2.html>
    faccessat = 269,
    /// <https://man7.org/linux/man-pages/man2/faccessat2.2.html>
    faccessat2 = 439,
    /// <https://man7.org/linux/man-pages/man2/fadvise64.2.html>
    fadvise64 = 221,
    /// <https://man7.org/linux/man-pages/man2/fallocate.2.html>
    fallocate = 285,
    /// <https://man7.org/linux/man-pages/man2/fanotify_init.2.html>
    fanotify_init = 300,
    /// <https://man7.org/linux/man-pages/man2/fanotify_mark.2.html>
    fanotify_mark = 301,
    /// <https://man7.org/linux/man-pages/man2/fchdir.2.html>
    fchdir = 81,
    /// <https://man7.org/linux/man-pages/man2/fchmod.2.html>
    fchmod = 91,
    /// <https://man7.org/linux/man-pages/man2/fchmodat.2.html>
    fchmodat = 268,
    /// <https://man7.org/linux/man-pages/man2/fchown.2.html>
    fchown = 93,
    /// <https://man7.org/linux/man-pages/man2/fchownat.2.html>
    fchownat = 260,
    /// <https://man7.org/linux/man-pages/man2/fcntl.2.html>
    fcntl = 72,
    /// <https://man7.org/linux/man-pages/man2/fdatasync.2.html>
    fdatasync = 75,
    /// <https://man7.org/linux/man-pages/man2/fgetxattr.2.html>
    fgetxattr = 193,
    /// <https://man7.org/linux/man-pages/man2/finit_module.2.html>
    finit_module = 313,
    /// <https://man7.org/linux/man-pages/man2/flistxattr.2.html>
    flistxattr = 196,
    /// <https://man7.org/linux/man-pages/man2/flock.2.html>
    flock = 73,
    /// <https://man7.org/linux/man-pages/man2/fork.2.html>
    fork = 57,
    /// <https://man7.org/linux/man-pages/man2/fremovexattr.2.html>
    fremovexattr = 199,
    /// <https://man7.org/linux/man-pages/man2/fsconfig.2.html>
    fsconfig = 431,
    /// <https://man7.org/linux/man-pages/man2/fsetxattr.2.html>
    fsetxattr = 190,
    /// <https://man7.org/linux/man-pages/man2/fsmount.2.html>
    fsmount = 432,
    /// <https://man7.org/linux/man-pages/man2/fsopen.2.html>
    fsopen = 430,
    /// <https://man7.org/linux/man-pages/man2/fspick.2.html>
    fspick = 433,
    /// <https://man7.org/linux/man-pages/man2/fstat.2.html>
    fstat = 5,
    /// <https://man7.org/linux/man-pages/man2/fstatfs.2.html>
    fstatfs = 138,
    /// <https://man7.org/linux/man-pages/man2/fsync.2.html>
    fsync = 74,
    /// <https://man7.org/linux/man-pages/man2/ftruncate.2.html>
    ftruncate = 77,
    /// <https://man7.org/linux/man-pages/man2/futex.2.html>
    futex = 202,
    /// <https://man7.org/linux/man-pages/man2/futex_waitv.2.html>
    futex_waitv = 449,
    /// <https://man7.org/linux/man-pages/man2/futimesat.2.html>
    futimesat = 261,
    /// <https://man7.org/linux/man-pages/man2/get_kernel_syms.2.html>
    get_kernel_syms = 177,
    /// <https://man7.org/linux/man-pages/man2/get_mempolicy.2.html>
    get_mempolicy = 239,
    /// <https://man7.org/linux/man-pages/man2/get_robust_list.2.html>
    get_robust_list = 274,
    /// <https://man7.org/linux/man-pages/man2/get_thread_area.2.html>
    get_thread_area = 211,
    /// <https://man7.org/linux/man-pages/man2/getcpu.2.html>
    getcpu = 309,
    /// <https://man7.org/linux/man-pages/man2/getcwd.2.html>
    getcwd = 79,
    /// <https://man7.org/linux/man-pages/man2/getdents.2.html>
    getdents = 78,
    /// <https://man7.org/linux/man-pages/man2/getdents64.2.html>
    getdents64 = 217,
    /// <https://man7.org/linux/man-pages/man2/getegid.2.html>
    getegid = 108,
    /// <https://man7.org/linux/man-pages/man2/geteuid.2.html>
    geteuid = 107,
    /// <https://man7.org/linux/man-pages/man2/getgid.2.html>
    getgid = 104,
    /// <https://man7.org/linux/man-pages/man2/getgroups.2.html>
    getgroups = 115,
    /// <https://man7.org/linux/man-pages/man2/getitimer.2.html>
    getitimer = 36,
    /// <https://man7.org/linux/man-pages/man2/getpeername.2.html>
    getpeername = 52,
    /// <https://man7.org/linux/man-pages/man2/getpgid.2.html>
    getpgid = 121,
    /// <https://man7.org/linux/man-pages/man2/getpgrp.2.html>
    getpgrp = 111,
    /// <https://man7.org/linux/man-pages/man2/getpid.2.html>
    getpid = 39,
    /// <https://man7.org/linux/man-pages/man2/getpmsg.2.html>
    getpmsg = 181,
    /// <https://man7.org/linux/man-pages/man2/getppid.2.html>
    getppid = 110,
    /// <https://man7.org/linux/man-pages/man2/getpriority.2.html>
    getpriority = 140,
    /// <https://man7.org/linux/man-pages/man2/getrandom.2.html>
    getrandom = 318,
    /// <https://man7.org/linux/man-pages/man2/getresgid.2.html>
    getresgid = 120,
    /// <https://man7.org/linux/man-pages/man2/getresuid.2.html>
    getresuid = 118,
    /// <https://man7.org/linux/man-pages/man2/getrlimit.2.html>
    getrlimit = 97,
    /// <https://man7.org/linux/man-pages/man2/getrusage.2.html>
    getrusage = 98,
    /// <https://man7.org/linux/man-pages/man2/getsid.2.html>
    getsid = 124,
    /// <https://man7.org/linux/man-pages/man2/getsockname.2.html>
    getsockname = 51,
    /// <https://man7.org/linux/man-pages/man2/getsockopt.2.html>
    getsockopt = 55,
    /// <https://man7.org/linux/man-pages/man2/gettid.2.html>
    gettid = 186,
    /// <https://man7.org/linux/man-pages/man2/gettimeofday.2.html>
    gettimeofday = 96,
    /// <https://man7.org/linux/man-pages/man2/getuid.2.html>
    getuid = 102,
    /// <https://man7.org/linux/man-pages/man2/getxattr.2.html>
    getxattr = 191,
    /// <https://man7.org/linux/man-pages/man2/init_module.2.html>
    init_module = 175,
    /// <https://man7.org/linux/man-pages/man2/inotify_add_watch.2.html>
    inotify_add_watch = 254,
    /// <https://man7.org/linux/man-pages/man2/inotify_init.2.html>
    inotify_init = 253,
    /// <https://man7.org/linux/man-pages/man2/inotify_init1.2.html>
    inotify_init1 = 294,
    /// <https://man7.org/linux/man-pages/man2/inotify_rm_watch.2.html>
    inotify_rm_watch = 255,
    /// <https://man7.org/linux/man-pages/man2/io_cancel.2.html>
    io_cancel = 210,
    /// <https://man7.org/linux/man-pages/man2/io_destroy.2.html>
    io_destroy = 207,
    /// <https://man7.org/linux/man-pages/man2/io_getevents.2.html>
    io_getevents = 208,
    /// <https://man7.org/linux/man-pages/man2/io_pgetevents.2.html>
    io_pgetevents = 333,
    /// <https://man7.org/linux/man-pages/man2/io_setup.2.html>
    io_setup = 206,
    /// <https://man7.org/linux/man-pages/man2/io_submit.2.html>
    io_submit = 209,
    /// <https://man7.org/linux/man-pages/man2/io_uring_enter.2.html>
    io_uring_enter = 426,
    /// <https://man7.org/linux/man-pages/man2/io_uring_register.2.html>
    io_uring_register = 427,
    /// <https://man7.org/linux/man-pages/man2/io_uring_setup.2.html>
    io_uring_setup = 425,
    /// <https://man7.org/linux/man-pages/man2/ioctl.2.html>
    ioctl = 16,
    /// <https://man7.org/linux/man-pages/man2/ioperm.2.html>
    ioperm = 173,
    /// <https://man7.org/linux/man-pages/man2/iopl.2.html>
    iopl = 172,
    /// <https://man7.org/linux/man-pages/man2/ioprio_get.2.html>
    ioprio_get = 252,
    /// <https://man7.org/linux/man-pages/man2/ioprio_set.2.html>
    ioprio_set = 251,
    /// <https://man7.org/linux/man-pages/man2/kcmp.2.html>
    kcmp = 312,
    /// <https://man7.org/linux/man-pages/man2/kexec_file_load.2.html>
    kexec_file_load = 320,
    /// <https://man7.org/linux/man-pages/man2/kexec_load.2.html>
    kexec_load = 246,
    /// <https://man7.org/linux/man-pages/man2/keyctl.2.html>
    keyctl = 250,
    /// <https://man7.org/linux/man-pages/man2/kill.2.html>
    kill = 62,
    /// <https://man7.org/linux/man-pages/man2/landlock_add_rule.2.html>
    landlock_add_rule = 445,
    /// <https://man7.org/linux/man-pages/man2/landlock_create_ruleset.2.html>
    landlock_create_ruleset = 444,
    /// <https://man7.org/linux/man-pages/man2/landlock_restrict_self.2.html>
    landlock_restrict_self = 446,
    /// <https://man7.org/linux/man-pages/man2/lchown.2.html>
    lchown = 94,
    /// <https://man7.org/linux/man-pages/man2/lgetxattr.2.html>
    lgetxattr = 192,
    /// <https://man7.org/linux/man-pages/man2/link.2.html>
    link = 86,
    /// <https://man7.org/linux/man-pages/man2/linkat.2.html>
    linkat = 265,
    /// <https://man7.org/linux/man-pages/man2/listen.2.html>
    listen = 50,
    /// <https://man7.org/linux/man-pages/man2/listxattr.2.html>
    listxattr = 194,
    /// <https://man7.org/linux/man-pages/man2/llistxattr.2.html>
    llistxattr = 195,
    /// <https://man7.org/linux/man-pages/man2/lookup_dcookie.2.html>
    lookup_dcookie = 212,
    /// <https://man7.org/linux/man-pages/man2/lremovexattr.2.html>
    lremovexattr = 198,
    /// <https://man7.org/linux/man-pages/man2/lseek.2.html>
    lseek = 8,
    /// <https://man7.org/linux/man-pages/man2/lsetxattr.2.html>
    lsetxattr = 189,
    /// <https://man7.org/linux/man-pages/man2/lstat.2.html>
    lstat = 6,
    /// <https://man7.org/linux/man-pages/man2/madvise.2.html>
    madvise = 28,
    /// <https://man7.org/linux/man-pages/man2/mbind.2.html>
    mbind = 237,
    /// <https://man7.org/linux/man-pages/man2/membarrier.2.html>
    membarrier = 324,
    /// <https://man7.org/linux/man-pages/man2/memfd_create.2.html>
    memfd_create = 319,
    /// <https://man7.org/linux/man-pages/man2/memfd_secret.2.html>
    memfd_secret = 447,
    /// <https://man7.org/linux/man-pages/man2/migrate_pages.2.html>
    migrate_pages = 256,
    /// <https://man7.org/linux/man-pages/man2/mincore.2.html>
    mincore = 27,
    /// <https://man7.org/linux/man-pages/man2/mkdir.2.html>
    mkdir = 83,
    /// <https://man7.org/linux/man-pages/man2/mkdirat.2.html>
    mkdirat = 258,
    /// <https://man7.org/linux/man-pages/man2/mknod.2.html>
    mknod = 133,
    /// <https://man7.org/linux/man-pages/man2/mknodat.2.html>
    mknodat = 259,
    /// <https://man7.org/linux/man-pages/man2/mlock.2.html>
    mlock = 149,
    /// <https://man7.org/linux/man-pages/man2/mlock2.2.html>
    mlock2 = 325,
    /// <https://man7.org/linux/man-pages/man2/mlockall.2.html>
    mlockall = 151,
    /// <https://man7.org/linux/man-pages/man2/mmap.2.html>
    mmap = 9,
    /// <https://man7.org/linux/man-pages/man2/modify_ldt.2.html>
    modify_ldt = 154,
    /// <https://man7.org/linux/man-pages/man2/mount.2.html>
    mount = 165,
    /// <https://man7.org/linux/man-pages/man2/mount_setattr.2.html>
    mount_setattr = 442,
    /// <https://man7.org/linux/man-pages/man2/move_mount.2.html>
    move_mount = 429,
    /// <https://man7.org/linux/man-pages/man2/move_pages.2.html>
    move_pages = 279,
    /// <https://man7.org/linux/man-pages/man2/mprotect.2.html>
    mprotect = 10,
    /// <https://man7.org/linux/man-pages/man2/mq_getsetattr.2.html>
    mq_getsetattr = 245,
    /// <https://man7.org/linux/man-pages/man2/mq_notify.2.html>
    mq_notify = 244,
    /// <https://man7.org/linux/man-pages/man2/mq_open.2.html>
    mq_open = 240,
    /// <https://man7.org/linux/man-pages/man2/mq_timedreceive.2.html>
    mq_timedreceive = 243,
    /// <https://man7.org/linux/man-pages/man2/mq_timedsend.2.html>
    mq_timedsend = 242,
    /// <https://man7.org/linux/man-pages/man2/mq_unlink.2.html>
    mq_unlink = 241,
    /// <https://man7.org/linux/man-pages/man2/mremap.2.html>
    mremap = 25,
    /// <https://man7.org/linux/man-pages/man2/msgctl.2.html>
    msgctl = 71,
    /// <https://man7.org/linux/man-pages/man2/msgget.2.html>
    msgget = 68,
    /// <https://man7.org/linux/man-pages/man2/msgrcv.2.html>
    msgrcv = 70,
    /// <https://man7.org/linux/man-pages/man2/msgsnd.2.html>
    msgsnd = 69,
    /// <https://man7.org/linux/man-pages/man2/msync.2.html>
    msync = 26,
    /// <https://man7.org/linux/man-pages/man2/munlock.2.html>
    munlock = 150,
    /// <https://man7.org/linux/man-pages/man2/munlockall.2.html>
    munlockall = 152,
    /// <https://man7.org/linux/man-pages/man2/munmap.2.html>
    munmap = 11,
    /// <https://man7.org/linux/man-pages/man2/name_to_handle_at.2.html>
    name_to_handle_at = 303,
    /// <https://man7.org/linux/man-pages/man2/nanosleep.2.html>
    nanosleep = 35,
    /// <https://man7.org/linux/man-pages/man2/newfstatat.2.html>
    newfstatat = 262,
    /// <https://man7.org/linux/man-pages/man2/nfsservctl.2.html>
    nfsservctl = 180,
    /// <https://man7.org/linux/man-pages/man2/open.2.html>
    open = 2,
    /// <https://man7.org/linux/man-pages/man2/open_by_handle_at.2.html>
    open_by_handle_at = 304,
    /// <https://man7.org/linux/man-pages/man2/open_tree.2.html>
    open_tree = 428,
    /// <https://man7.org/linux/man-pages/man2/openat.2.html>
    openat = 257,
    /// <https://man7.org/linux/man-pages/man2/openat2.2.html>
    openat2 = 437,
    /// <https://man7.org/linux/man-pages/man2/pause.2.html>
    pause = 34,
    /// <https://man7.org/linux/man-pages/man2/perf_event_open.2.html>
    perf_event_open = 298,
    /// <https://man7.org/linux/man-pages/man2/personality.2.html>
    personality = 135,
    /// <https://man7.org/linux/man-pages/man2/pidfd_getfd.2.html>
    pidfd_getfd = 438,
    /// <https://man7.org/linux/man-pages/man2/pidfd_open.2.html>
    pidfd_open = 434,
    /// <https://man7.org/linux/man-pages/man2/pidfd_send_signal.2.html>
    pidfd_send_signal = 424,
    /// <https://man7.org/linux/man-pages/man2/pipe.2.html>
    pipe = 22,
    /// <https://man7.org/linux/man-pages/man2/pipe2.2.html>
    pipe2 = 293,
    /// <https://man7.org/linux/man-pages/man2/pivot_root.2.html>
    pivot_root = 155,
    /// <https://man7.org/linux/man-pages/man2/pkey_alloc.2.html>
    pkey_alloc = 330,
    /// <https://man7.org/linux/man-pages/man2/pkey_free.2.html>
    pkey_free = 331,
    /// <https://man7.org/linux/man-pages/man2/pkey_mprotect.2.html>
    pkey_mprotect = 329,
    /// <https://man7.org/linux/man-pages/man2/poll.2.html>
    poll = 7,
    /// <https://man7.org/linux/man-pages/man2/ppoll.2.html>
    ppoll = 271,
    /// <https://man7.org/linux/man-pages/man2/prctl.2.html>
    prctl = 157,
    /// <https://man7.org/linux/man-pages/man2/pread64.2.html>
    pread64 = 17,
    /// <https://man7.org/linux/man-pages/man2/preadv.2.html>
    preadv = 295,
    /// <https://man7.org/linux/man-pages/man2/preadv2.2.html>
    preadv2 = 327,
    /// <https://man7.org/linux/man-pages/man2/prlimit64.2.html>
    prlimit64 = 302,
    /// <https://man7.org/linux/man-pages/man2/process_madvise.2.html>
    process_madvise = 440,
    /// <https://man7.org/linux/man-pages/man2/process_mrelease.2.html>
    process_mrelease = 448,
    /// <https://man7.org/linux/man-pages/man2/process_vm_readv.2.html>
    process_vm_readv = 310,
    /// <https://man7.org/linux/man-pages/man2/process_vm_writev.2.html>
    process_vm_writev = 311,
    /// <https://man7.org/linux/man-pages/man2/pselect6.2.html>
    pselect6 = 270,
    /// <https://man7.org/linux/man-pages/man2/ptrace.2.html>
    ptrace = 101,
    /// <https://man7.org/linux/man-pages/man2/putpmsg.2.html>
    putpmsg = 182,
    /// <https://man7.org/linux/man-pages/man2/pwrite64.2.html>
    pwrite64 = 18,
    /// <https://man7.org/linux/man-pages/man2/pwritev.2.html>
    pwritev = 296,
    /// <https://man7.org/linux/man-pages/man2/pwritev2.2.html>
    pwritev2 = 328,
    /// <https://man7.org/linux/man-pages/man2/query_module.2.html>
    query_module = 178,
    /// <https://man7.org/linux/man-pages/man2/quotactl.2.html>
    quotactl = 179,
    /// <https://man7.org/linux/man-pages/man2/quotactl_fd.2.html>
    quotactl_fd = 443,
    /// <https://man7.org/linux/man-pages/man2/read.2.html>
    read = 0,
    /// <https://man7.org/linux/man-pages/man2/readahead.2.html>
    readahead = 187,
    /// <https://man7.org/linux/man-pages/man2/readlink.2.html>
    readlink = 89,
    /// <https://man7.org/linux/man-pages/man2/readlinkat.2.html>
    readlinkat = 267,
    /// <https://man7.org/linux/man-pages/man2/readv.2.html>
    readv = 19,
    /// <https://man7.org/linux/man-pages/man2/reboot.2.html>
    reboot = 169,
    /// <https://man7.org/linux/man-pages/man2/recvfrom.2.html>
    recvfrom = 45,
    /// <https://man7.org/linux/man-pages/man2/recvmmsg.2.html>
    recvmmsg = 299,
    /// <https://man7.org/linux/man-pages/man2/recvmsg.2.html>
    recvmsg = 47,
    /// <https://man7.org/linux/man-pages/man2/remap_file_pages.2.html>
    remap_file_pages = 216,
    /// <https://man7.org/linux/man-pages/man2/removexattr.2.html>
    removexattr = 197,
    /// <https://man7.org/linux/man-pages/man2/rename.2.html>
    rename = 82,
    /// <https://man7.org/linux/man-pages/man2/renameat.2.html>
    renameat = 264,
    /// <https://man7.org/linux/man-pages/man2/renameat2.2.html>
    renameat2 = 316,
    /// <https://man7.org/linux/man-pages/man2/request_key.2.html>
    request_key = 249,
    /// <https://man7.org/linux/man-pages/man2/restart_syscall.2.html>
    restart_syscall = 219,
    /// <https://man7.org/linux/man-pages/man2/rmdir.2.html>
    rmdir = 84,
    /// <https://man7.org/linux/man-pages/man2/rseq.2.html>
    rseq = 334,
    /// <https://man7.org/linux/man-pages/man2/rt_sigaction.2.html>
    rt_sigaction = 13,
    /// <https://man7.org/linux/man-pages/man2/rt_sigpending.2.html>
    rt_sigpending = 127,
    /// <https://man7.org/linux/man-pages/man2/rt_sigprocmask.2.html>
    rt_sigprocmask = 14,
    /// <https://man7.org/linux/man-pages/man2/rt_sigqueueinfo.2.html>
    rt_sigqueueinfo = 129,
    /// <https://man7.org/linux/man-pages/man2/rt_sigreturn.2.html>
    rt_sigreturn = 15,
    /// <https://man7.org/linux/man-pages/man2/rt_sigsuspend.2.html>
    rt_sigsuspend = 130,
    /// <https://man7.org/linux/man-pages/man2/rt_sigtimedwait.2.html>
    rt_sigtimedwait = 128,
    /// <https://man7.org/linux/man-pages/man2/rt_tgsigqueueinfo.2.html>
    rt_tgsigqueueinfo = 297,
    /// <https://man7.org/linux/man-pages/man2/sched_get_priority_max.2.html>
    sched_get_priority_max = 146,
    /// <https://man7.org/linux/man-pages/man2/sched_get_priority_min.2.html>
    sched_get_priority_min = 147,
    /// <https://man7.org/linux/man-pages/man2/sched_getaffinity.2.html>
    sched_getaffinity = 204,
    /// <https://man7.org/linux/man-pages/man2/sched_getattr.2.html>
    sched_getattr = 315,
    /// <https://man7.org/linux/man-pages/man2/sched_getparam.2.html>
    sched_getparam = 143,
    /// <https://man7.org/linux/man-pages/man2/sched_getscheduler.2.html>
    sched_getscheduler = 145,
    /// <https://man7.org/linux/man-pages/man2/sched_rr_get_interval.2.html>
    sched_rr_get_interval = 148,
    /// <https://man7.org/linux/man-pages/man2/sched_setaffinity.2.html>
    sched_setaffinity = 203,
    /// <https://man7.org/linux/man-pages/man2/sched_setattr.2.html>
    sched_setattr = 314,
    /// <https://man7.org/linux/man-pages/man2/sched_setparam.2.html>
    sched_setparam = 142,
    /// <https://man7.org/linux/man-pages/man2/sched_setscheduler.2.html>
    sched_setscheduler = 144,
    /// <https://man7.org/linux/man-pages/man2/sched_yield.2.html>
    sched_yield = 24,
    /// <https://man7.org/linux/man-pages/man2/seccomp.2.html>
    seccomp = 317,
    /// <https://man7.org/linux/man-pages/man2/security.2.html>
    security = 185,
    /// <https://man7.org/linux/man-pages/man2/select.2.html>
    select = 23,
    /// <https://man7.org/linux/man-pages/man2/semctl.2.html>
    semctl = 66,
    /// <https://man7.org/linux/man-pages/man2/semget.2.html>
    semget = 64,
    /// <https://man7.org/linux/man-pages/man2/semop.2.html>
    semop = 65,
    /// <https://man7.org/linux/man-pages/man2/semtimedop.2.html>
    semtimedop = 220,
    /// <https://man7.org/linux/man-pages/man2/sendfile.2.html>
    sendfile = 40,
    /// <https://man7.org/linux/man-pages/man2/sendmmsg.2.html>
    sendmmsg = 307,
    /// <https://man7.org/linux/man-pages/man2/sendmsg.2.html>
    sendmsg = 46,
    /// <https://man7.org/linux/man-pages/man2/sendto.2.html>
    sendto = 44,
    /// <https://man7.org/linux/man-pages/man2/set_mempolicy.2.html>
    set_mempolicy = 238,
    /// <https://man7.org/linux/man-pages/man2/set_mempolicy_home_node.2.html>
    set_mempolicy_home_node = 450,
    /// <https://man7.org/linux/man-pages/man2/set_robust_list.2.html>
    set_robust_list = 273,
    /// <https://man7.org/linux/man-pages/man2/set_thread_area.2.html>
    set_thread_area = 205,
    /// <https://man7.org/linux/man-pages/man2/set_tid_address.2.html>
    set_tid_address = 218,
    /// <https://man7.org/linux/man-pages/man2/setdomainname.2.html>
    setdomainname = 171,
    /// <https://man7.org/linux/man-pages/man2/setfsgid.2.html>
    setfsgid = 123,
    /// <https://man7.org/linux/man-pages/man2/setfsuid.2.html>
    setfsuid = 122,
    /// <https://man7.org/linux/man-pages/man2/setgid.2.html>
    setgid = 106,
    /// <https://man7.org/linux/man-pages/man2/setgroups.2.html>
    setgroups = 116,
    /// <https://man7.org/linux/man-pages/man2/sethostname.2.html>
    sethostname = 170,
    /// <https://man7.org/linux/man-pages/man2/setitimer.2.html>
    setitimer = 38,
    /// <https://man7.org/linux/man-pages/man2/setns.2.html>
    setns = 308,
    /// <https://man7.org/linux/man-pages/man2/setpgid.2.html>
    setpgid = 109,
    /// <https://man7.org/linux/man-pages/man2/setpriority.2.html>
    setpriority = 141,
    /// <https://man7.org/linux/man-pages/man2/setregid.2.html>
    setregid = 114,
    /// <https://man7.org/linux/man-pages/man2/setresgid.2.html>
    setresgid = 119,
    /// <https://man7.org/linux/man-pages/man2/setresuid.2.html>
    setresuid = 117,
    /// <https://man7.org/linux/man-pages/man2/setreuid.2.html>
    setreuid = 113,
    /// <https://man7.org/linux/man-pages/man2/setrlimit.2.html>
    setrlimit = 160,
    /// <https://man7.org/linux/man-pages/man2/setsid.2.html>
    setsid = 112,
    /// <https://man7.org/linux/man-pages/man2/setsockopt.2.html>
    setsockopt = 54,
    /// <https://man7.org/linux/man-pages/man2/settimeofday.2.html>
    settimeofday = 164,
    /// <https://man7.org/linux/man-pages/man2/setuid.2.html>
    setuid = 105,
    /// <https://man7.org/linux/man-pages/man2/setxattr.2.html>
    setxattr = 188,
    /// <https://man7.org/linux/man-pages/man2/shmat.2.html>
    shmat = 30,
    /// <https://man7.org/linux/man-pages/man2/shmctl.2.html>
    shmctl = 31,
    /// <https://man7.org/linux/man-pages/man2/shmdt.2.html>
    shmdt = 67,
    /// <https://man7.org/linux/man-pages/man2/shmget.2.html>
    shmget = 29,
    /// <https://man7.org/linux/man-pages/man2/shutdown.2.html>
    shutdown = 48,
    /// <https://man7.org/linux/man-pages/man2/sigaltstack.2.html>
    sigaltstack = 131,
    /// <https://man7.org/linux/man-pages/man2/signalfd.2.html>
    signalfd = 282,
    /// <https://man7.org/linux/man-pages/man2/signalfd4.2.html>
    signalfd4 = 289,
    /// <https://man7.org/linux/man-pages/man2/socket.2.html>
    socket = 41,
    /// <https://man7.org/linux/man-pages/man2/socketpair.2.html>
    socketpair = 53,
    /// <https://man7.org/linux/man-pages/man2/splice.2.html>
    splice = 275,
    /// <https://man7.org/linux/man-pages/man2/stat.2.html>
    stat = 4,
    /// <https://man7.org/linux/man-pages/man2/statfs.2.html>
    statfs = 137,
    /// <https://man7.org/linux/man-pages/man2/statx.2.html>
    statx = 332,
    /// <https://man7.org/linux/man-pages/man2/swapoff.2.html>
    swapoff = 168,
    /// <https://man7.org/linux/man-pages/man2/swapon.2.html>
    swapon = 167,
    /// <https://man7.org/linux/man-pages/man2/symlink.2.html>
    symlink = 88,
    /// <https://man7.org/linux/man-pages/man2/symlinkat.2.html>
    symlinkat = 266,
    /// <https://man7.org/linux/man-pages/man2/sync.2.html>
    sync = 162,
    /// <https://man7.org/linux/man-pages/man2/sync_file_range.2.html>
    sync_file_range = 277,
    /// <https://man7.org/linux/man-pages/man2/syncfs.2.html>
    syncfs = 306,
    /// <https://man7.org/linux/man-pages/man2/sysfs.2.html>
    sysfs = 139,
    /// <https://man7.org/linux/man-pages/man2/sysinfo.2.html>
    sysinfo = 99,
    /// <https://man7.org/linux/man-pages/man2/syslog.2.html>
    syslog = 103,
    /// <https://man7.org/linux/man-pages/man2/tee.2.html>
    tee = 276,
    /// <https://man7.org/linux/man-pages/man2/tgkill.2.html>
    tgkill = 234,
    /// <https://man7.org/linux/man-pages/man2/time.2.html>
    time = 201,
    /// <https://man7.org/linux/man-pages/man2/timer_create.2.html>
    timer_create = 222,
    /// <https://man7.org/linux/man-pages/man2/timer_delete.2.html>
    timer_delete = 226,
    /// <https://man7.org/linux/man-pages/man2/timer_getoverrun.2.html>
    timer_getoverrun = 225,
    /// <https://man7.org/linux/man-pages/man2/timer_gettime.2.html>
    timer_gettime = 224,
    /// <https://man7.org/linux/man-pages/man2/timer_settime.2.html>
    timer_settime = 223,
    /// <https://man7.org/linux/man-pages/man2/timerfd_create.2.html>
    timerfd_create = 283,
    /// <https://man7.org/linux/man-pages/man2/timerfd_gettime.2.html>
    timerfd_gettime = 287,
    /// <https://man7.org/linux/man-pages/man2/timerfd_settime.2.html>
    timerfd_settime = 286,
    /// <https://man7.org/linux/man-pages/man2/times.2.html>
    times = 100,
    /// <https://man7.org/linux/man-pages/man2/tkill.2.html>
    tkill = 200,
    /// <https://man7.org/linux/man-pages/man2/truncate.2.html>
    truncate = 76,
    /// <https://man7.org/linux/man-pages/man2/tuxcall.2.html>
    tuxcall = 184,
    /// <https://man7.org/linux/man-pages/man2/umask.2.html>
    umask = 95,
    /// <https://man7.org/linux/man-pages/man2/umount2.2.html>
    umount2 = 166,
    /// <https://man7.org/linux/man-pages/man2/uname.2.html>
    uname = 63,
    /// <https://man7.org/linux/man-pages/man2/unlink.2.html>
    unlink = 87,
    /// <https://man7.org/linux/man-pages/man2/unlinkat.2.html>
    unlinkat = 263,
    /// <https://man7.org/linux/man-pages/man2/unshare.2.html>
    unshare = 272,
    /// <https://man7.org/linux/man-pages/man2/uselib.2.html>
    uselib = 134,
    /// <https://man7.org/linux/man-pages/man2/userfaultfd.2.html>
    userfaultfd = 323,
    /// <https://man7.org/linux/man-pages/man2/ustat.2.html>
    ustat = 136,
    /// <https://man7.org/linux/man-pages/man2/utime.2.html>
    utime = 132,
    /// <https://man7.org/linux/man-pages/man2/utimensat.2.html>
    utimensat = 280,
    /// <https://man7.org/linux/man-pages/man2/utimes.2.html>
    utimes = 235,
    /// <https://man7.org/linux/man-pages/man2/vfork.2.html>
    vfork = 58,
    /// <https://man7.org/linux/man-pages/man2/vhangup.2.html>
    vhangup = 153,
    /// <https://man7.org/linux/man-pages/man2/vmsplice.2.html>
    vmsplice = 278,
    /// <https://man7.org/linux/man-pages/man2/vserver.2.html>
    vserver = 236,
    /// <https://man7.org/linux/man-pages/man2/wait4.2.html>
    wait4 = 61,
    /// <https://man7.org/linux/man-pages/man2/waitid.2.html>
    waitid = 247,
    /// <https://man7.org/linux/man-pages/man2/write.2.html>
    write = 1,
    /// <https://man7.org/linux/man-pages/man2/writev.2.html>
    writev = 20,
}
