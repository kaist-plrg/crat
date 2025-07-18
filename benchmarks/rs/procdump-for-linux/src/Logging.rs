use ::libc;
extern "C" {
    fn malloc(_: libc::c_ulong) -> *mut libc::c_void;
    fn free(__ptr: *mut libc::c_void);
    fn pthread_mutex_lock(__mutex: *mut pthread_mutex_t) -> libc::c_int;
    fn pthread_mutex_unlock(__mutex: *mut pthread_mutex_t) -> libc::c_int;
    fn sprintf(_: *mut libc::c_char, _: *const libc::c_char, _: ...) -> libc::c_int;
    fn vsprintf(
        _: *mut libc::c_char,
        _: *const libc::c_char,
        _: ::std::ffi::VaList,
    ) -> libc::c_int;
    fn snprintf(
        _: *mut libc::c_char,
        _: libc::c_ulong,
        _: *const libc::c_char,
        _: ...
    ) -> libc::c_int;
    fn vsnprintf(
        _: *mut libc::c_char,
        _: libc::c_ulong,
        _: *const libc::c_char,
        _: ::std::ffi::VaList,
    ) -> libc::c_int;
    fn puts(__s: *const libc::c_char) -> libc::c_int;
    fn localtime(__timer: *const time_t) -> *mut tm;
    fn strftime(
        __s: *mut libc::c_char,
        __maxsize: size_t,
        __format: *const libc::c_char,
        __tp: *const tm,
    ) -> size_t;
    fn time(__timer: *mut time_t) -> time_t;
    fn syslog(__pri: libc::c_int, __fmt: *const libc::c_char, _: ...);
    static mut g_config: ProcDumpConfiguration;
}
pub type __builtin_va_list = [__va_list_tag; 1];
#[derive(Copy, Clone)]
#[repr(C)]
pub struct __va_list_tag {
    pub gp_offset: libc::c_uint,
    pub fp_offset: libc::c_uint,
    pub overflow_arg_area: *mut libc::c_void,
    pub reg_save_area: *mut libc::c_void,
}
pub type va_list = __builtin_va_list;
pub type __pid_t = libc::c_int;
pub type __time_t = libc::c_long;
pub type size_t = libc::c_ulong;
pub type time_t = __time_t;
pub type pid_t = __pid_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct tm {
    pub tm_sec: libc::c_int,
    pub tm_min: libc::c_int,
    pub tm_hour: libc::c_int,
    pub tm_mday: libc::c_int,
    pub tm_mon: libc::c_int,
    pub tm_year: libc::c_int,
    pub tm_wday: libc::c_int,
    pub tm_yday: libc::c_int,
    pub tm_isdst: libc::c_int,
    pub tm_gmtoff: libc::c_long,
    pub tm_zone: *const libc::c_char,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct __pthread_internal_list {
    pub __prev: *mut __pthread_internal_list,
    pub __next: *mut __pthread_internal_list,
}
pub type __pthread_list_t = __pthread_internal_list;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct __pthread_mutex_s {
    pub __lock: libc::c_int,
    pub __count: libc::c_uint,
    pub __owner: libc::c_int,
    pub __nusers: libc::c_uint,
    pub __kind: libc::c_int,
    pub __spins: libc::c_short,
    pub __elision: libc::c_short,
    pub __list: __pthread_list_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct __pthread_cond_s {
    pub c2rust_unnamed: C2RustUnnamed_1,
    pub c2rust_unnamed_0: C2RustUnnamed,
    pub __g_refs: [libc::c_uint; 2],
    pub __g_size: [libc::c_uint; 2],
    pub __g1_orig_size: libc::c_uint,
    pub __wrefs: libc::c_uint,
    pub __g_signals: [libc::c_uint; 2],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed {
    pub __g1_start: libc::c_ulonglong,
    pub __g1_start32: C2RustUnnamed_0,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_0 {
    pub __low: libc::c_uint,
    pub __high: libc::c_uint,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_1 {
    pub __wseq: libc::c_ulonglong,
    pub __wseq32: C2RustUnnamed_2,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_2 {
    pub __low: libc::c_uint,
    pub __high: libc::c_uint,
}
pub type pthread_t = libc::c_ulong;
#[derive(Copy, Clone)]
#[repr(C)]
pub union pthread_mutex_t {
    pub __data: __pthread_mutex_s,
    pub __size: [libc::c_char; 40],
    pub __align: libc::c_long,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union pthread_cond_t {
    pub __data: __pthread_cond_s,
    pub __size: [libc::c_char; 48],
    pub __align: libc::c_longlong,
}
pub type LogLevel = libc::c_uint;
pub const error: LogLevel = 4;
pub const crit: LogLevel = 3;
pub const warn: LogLevel = 2;
pub const info: LogLevel = 1;
pub const debug: LogLevel = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ProcDumpConfiguration {
    pub ProcessId: pid_t,
    pub ProcessName: *mut libc::c_char,
    pub SystemInfo: sysinfo,
    pub NumberOfDumpsCollecting: libc::c_int,
    pub NumberOfDumpsCollected: libc::c_int,
    pub bTerminated: bool,
    pub nQuit: libc::c_int,
    pub evtQuit: Handle,
    pub bTriggerThenSnoozeCPU: bool,
    pub bTriggerThenSnoozeMemory: bool,
    pub bTriggerThenSnoozeTimer: bool,
    pub CpuThreshold: libc::c_int,
    pub bCpuTriggerBelowValue: bool,
    pub MemoryThreshold: libc::c_int,
    pub bMemoryTriggerBelowValue: bool,
    pub ThresholdSeconds: libc::c_int,
    pub bTimerThreshold: bool,
    pub NumberOfDumpsToCollect: libc::c_int,
    pub WaitingForProcessName: bool,
    pub DiagnosticsLoggingEnabled: bool,
    pub ThreadThreshold: libc::c_int,
    pub FileDescriptorThreshold: libc::c_int,
    pub SignalNumber: libc::c_int,
    pub PollingInterval: libc::c_int,
    pub CoreDumpPath: *mut libc::c_char,
    pub CoreDumpName: *mut libc::c_char,
    pub nThreads: libc::c_int,
    pub Threads: [pthread_t; 3],
    pub semAvailableDumpSlots: Handle,
    pub evtCtrlHandlerCleanupComplete: Handle,
    pub evtBannerPrinted: Handle,
    pub evtConfigurationPrinted: Handle,
    pub evtDebugThreadInitialized: Handle,
    pub evtStartMonitoring: Handle,
    pub gcorePid: pid_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct Handle {
    pub c2rust_unnamed: C2RustUnnamed_3,
    pub type_0: EHandleType,
}
pub type EHandleType = libc::c_uint;
pub const SEMAPHORE: EHandleType = 1;
pub const EVENT: EHandleType = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_3 {
    pub event: Event,
    pub semaphore: sem_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union sem_t {
    pub __size: [libc::c_char; 32],
    pub __align: libc::c_long,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct Event {
    pub mutex: pthread_mutex_t,
    pub cond: pthread_cond_t,
    pub bTriggered: bool,
    pub bManualReset: bool,
    pub Name: [libc::c_char; 64],
    pub nWaiters: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct sysinfo {
    pub uptime: __kernel_long_t,
    pub loads: [__kernel_ulong_t; 3],
    pub totalram: __kernel_ulong_t,
    pub freeram: __kernel_ulong_t,
    pub sharedram: __kernel_ulong_t,
    pub bufferram: __kernel_ulong_t,
    pub totalswap: __kernel_ulong_t,
    pub freeswap: __kernel_ulong_t,
    pub procs: __u16,
    pub pad: __u16,
    pub totalhigh: __kernel_ulong_t,
    pub freehigh: __kernel_ulong_t,
    pub mem_unit: __u32,
    pub _f: [libc::c_char; 0],
}
pub type __u32 = libc::c_uint;
pub type __kernel_ulong_t = libc::c_ulong;
pub type __u16 = libc::c_ushort;
pub type __kernel_long_t = libc::c_long;
static mut LogLevelStrings: [*const libc::c_char; 5] = [
    b"DEBUG\0" as *const u8 as *const libc::c_char,
    b"INFO\0" as *const u8 as *const libc::c_char,
    b"WARN\0" as *const u8 as *const libc::c_char,
    b"CRITICAL\0" as *const u8 as *const libc::c_char,
    b"ERROR\0" as *const u8 as *const libc::c_char,
];
pub static mut LoggerLock: pthread_mutex_t = pthread_mutex_t {
    __data: __pthread_mutex_s {
        __lock: 0,
        __count: 0,
        __owner: 0,
        __nusers: 0,
        __kind: 0,
        __spins: 0,
        __elision: 0,
        __list: __pthread_list_t {
            __prev: 0 as *const __pthread_internal_list as *mut __pthread_internal_list,
            __next: 0 as *const __pthread_internal_list as *mut __pthread_internal_list,
        },
    },
};
pub unsafe extern "C" fn LogFormatter(
    mut logLevel: LogLevel,
    mut message: *const libc::c_char,
    mut args: ::std::ffi::VaList,
) {
    let mut timeBuff: [libc::c_char; 64] = [0; 64];
    let mut rawTime: time_t = 0;
    let mut timeInfo: *mut tm = 0 as *mut tm;
    let mut trace: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut copy: ::std::ffi::VaListImpl;
    copy = args.clone();
    pthread_mutex_lock(&mut LoggerLock);
    rawTime = time(0 as *mut time_t);
    timeInfo = localtime(&mut rawTime);
    strftime(
        timeBuff.as_mut_ptr(),
        64 as libc::c_int as size_t,
        b"%T\0" as *const u8 as *const libc::c_char,
        timeInfo,
    );
    let mut traceLen: libc::c_int = snprintf(
        0 as *mut libc::c_char,
        0 as libc::c_int as libc::c_ulong,
        b"[%s - %s]: \0" as *const u8 as *const libc::c_char,
        timeBuff.as_mut_ptr(),
        LogLevelStrings[logLevel as usize],
    );
    let mut argsLen: libc::c_int = vsnprintf(
        0 as *mut libc::c_char,
        0 as libc::c_int as libc::c_ulong,
        message,
        copy.as_va_list(),
    );
    trace = malloc((traceLen + argsLen + 1 as libc::c_int) as libc::c_ulong)
        as *mut libc::c_char;
    if trace.is_null() {
        pthread_mutex_unlock(&mut LoggerLock);
        return;
    }
    sprintf(
        trace,
        b"[%s - %s]: \0" as *const u8 as *const libc::c_char,
        timeBuff.as_mut_ptr(),
        LogLevelStrings[logLevel as usize],
    );
    vsprintf(trace.offset(traceLen as isize), message, args.as_va_list());
    if logLevel as libc::c_uint != debug as libc::c_int as libc::c_uint {
        puts(trace);
    }
    syslog(7 as libc::c_int, b"%s\0" as *const u8 as *const libc::c_char, trace);
    free(trace as *mut libc::c_void);
    pthread_mutex_unlock(&mut LoggerLock);
}
pub unsafe extern "C" fn Log(
    mut logLevel: LogLevel,
    mut message: *const libc::c_char,
    mut args: ...
) {
    let mut args_0: ::std::ffi::VaListImpl;
    args_0 = args.clone();
    LogFormatter(logLevel, message, args_0.as_va_list());
}
pub unsafe extern "C" fn DiagTrace(mut message: *const libc::c_char, mut args: ...) {
    let mut args_0: ::std::ffi::VaListImpl;
    args_0 = args.clone();
    if g_config.DiagnosticsLoggingEnabled {
        LogFormatter(debug, message, args_0.as_va_list());
    }
}
