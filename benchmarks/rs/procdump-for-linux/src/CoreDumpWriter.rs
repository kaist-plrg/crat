use ::libc;
extern "C" {
    pub type _IO_wide_data;
    pub type _IO_codecvt;
    pub type _IO_marker;
    fn __ctype_b_loc() -> *mut *const libc::c_ushort;
    fn time(__timer: *mut time_t) -> time_t;
    fn strftime(
        __s: *mut libc::c_char,
        __maxsize: size_t,
        __format: *const libc::c_char,
        __tp: *const tm,
    ) -> size_t;
    fn localtime(__timer: *const time_t) -> *mut tm;
    fn pthread_setcanceltype(
        __type: libc::c_int,
        __oldtype: *mut libc::c_int,
    ) -> libc::c_int;
    fn sem_post(__sem: *mut sem_t) -> libc::c_int;
    fn access(__name: *const libc::c_char, __type: libc::c_int) -> libc::c_int;
    fn close(__fd: libc::c_int) -> libc::c_int;
    fn pipe(__pipedes: *mut libc::c_int) -> libc::c_int;
    fn sleep(__seconds: libc::c_uint) -> libc::c_uint;
    fn dup2(__fd: libc::c_int, __fd2: libc::c_int) -> libc::c_int;
    fn execl(
        __path: *const libc::c_char,
        __arg: *const libc::c_char,
        _: ...
    ) -> libc::c_int;
    fn setpgid(__pid: __pid_t, __pgid: __pid_t) -> libc::c_int;
    fn fork() -> __pid_t;
    fn unlink(__name: *const libc::c_char) -> libc::c_int;
    fn socket(
        __domain: libc::c_int,
        __type: libc::c_int,
        __protocol: libc::c_int,
    ) -> libc::c_int;
    fn connect(
        __fd: libc::c_int,
        __addr: *const sockaddr,
        __len: socklen_t,
    ) -> libc::c_int;
    fn send(
        __fd: libc::c_int,
        __buf: *const libc::c_void,
        __n: size_t,
        __flags: libc::c_int,
    ) -> ssize_t;
    fn recv(
        __fd: libc::c_int,
        __buf: *mut libc::c_void,
        __n: size_t,
        __flags: libc::c_int,
    ) -> ssize_t;
    fn memcpy(
        _: *mut libc::c_void,
        _: *const libc::c_void,
        _: libc::c_ulong,
    ) -> *mut libc::c_void;
    fn memset(
        _: *mut libc::c_void,
        _: libc::c_int,
        _: libc::c_ulong,
    ) -> *mut libc::c_void;
    fn strncpy(
        _: *mut libc::c_char,
        _: *const libc::c_char,
        _: libc::c_ulong,
    ) -> *mut libc::c_char;
    fn strncmp(
        _: *const libc::c_char,
        _: *const libc::c_char,
        _: libc::c_ulong,
    ) -> libc::c_int;
    fn strdup(_: *const libc::c_char) -> *mut libc::c_char;
    fn strstr(_: *const libc::c_char, _: *const libc::c_char) -> *mut libc::c_char;
    fn strtok(_: *mut libc::c_char, _: *const libc::c_char) -> *mut libc::c_char;
    fn strlen(_: *const libc::c_char) -> libc::c_ulong;
    fn malloc(_: libc::c_ulong) -> *mut libc::c_void;
    fn free(__ptr: *mut libc::c_void);
    fn exit(_: libc::c_int) -> !;
    fn getenv(__name: *const libc::c_char) -> *mut libc::c_char;
    fn fclose(__stream: *mut FILE) -> libc::c_int;
    fn fopen(_: *const libc::c_char, _: *const libc::c_char) -> *mut FILE;
    fn fdopen(__fd: libc::c_int, __modes: *const libc::c_char) -> *mut FILE;
    fn snprintf(
        _: *mut libc::c_char,
        _: libc::c_ulong,
        _: *const libc::c_char,
        _: ...
    ) -> libc::c_int;
    fn fgets(
        __s: *mut libc::c_char,
        __n: libc::c_int,
        __stream: *mut FILE,
    ) -> *mut libc::c_char;
    fn pclose(__stream: *mut FILE) -> libc::c_int;
    fn __errno_location() -> *mut libc::c_int;
    fn SetEvent(Event: *mut Event) -> bool;
    fn Log(logLevel: LogLevel, message: *const libc::c_char, _: ...);
    fn DiagTrace(message: *const libc::c_char, _: ...);
    fn WaitForQuitOrEvent(
        self_0: *mut ProcDumpConfiguration,
        handle: *mut Handle,
        milliseconds: libc::c_int,
    ) -> libc::c_int;
}
pub type __uint8_t = libc::c_uchar;
pub type __uint16_t = libc::c_ushort;
pub type __int32_t = libc::c_int;
pub type __off_t = libc::c_long;
pub type __off64_t = libc::c_long;
pub type __pid_t = libc::c_int;
pub type __time_t = libc::c_long;
pub type __ssize_t = libc::c_long;
pub type __socklen_t = libc::c_uint;
pub type C2RustUnnamed = libc::c_uint;
pub const _ISalnum: C2RustUnnamed = 8;
pub const _ISpunct: C2RustUnnamed = 4;
pub const _IScntrl: C2RustUnnamed = 2;
pub const _ISblank: C2RustUnnamed = 1;
pub const _ISgraph: C2RustUnnamed = 32768;
pub const _ISprint: C2RustUnnamed = 16384;
pub const _ISspace: C2RustUnnamed = 8192;
pub const _ISxdigit: C2RustUnnamed = 4096;
pub const _ISdigit: C2RustUnnamed = 2048;
pub const _ISalpha: C2RustUnnamed = 1024;
pub const _ISlower: C2RustUnnamed = 512;
pub const _ISupper: C2RustUnnamed = 256;
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
    pub c2rust_unnamed: C2RustUnnamed_2,
    pub c2rust_unnamed_0: C2RustUnnamed_0,
    pub __g_refs: [libc::c_uint; 2],
    pub __g_size: [libc::c_uint; 2],
    pub __g1_orig_size: libc::c_uint,
    pub __wrefs: libc::c_uint,
    pub __g_signals: [libc::c_uint; 2],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_0 {
    pub __g1_start: libc::c_ulonglong,
    pub __g1_start32: C2RustUnnamed_1,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_1 {
    pub __low: libc::c_uint,
    pub __high: libc::c_uint,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_2 {
    pub __wseq: libc::c_ulonglong,
    pub __wseq32: C2RustUnnamed_3,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_3 {
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
pub type C2RustUnnamed_4 = libc::c_uint;
pub const PTHREAD_CANCEL_ASYNCHRONOUS: C2RustUnnamed_4 = 1;
pub const PTHREAD_CANCEL_DEFERRED: C2RustUnnamed_4 = 0;
pub type ssize_t = __ssize_t;
pub type int32_t = __int32_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub union sem_t {
    pub __size: [libc::c_char; 32],
    pub __align: libc::c_long,
}
pub type socklen_t = __socklen_t;
pub type __socket_type = libc::c_uint;
pub const SOCK_NONBLOCK: __socket_type = 2048;
pub const SOCK_CLOEXEC: __socket_type = 524288;
pub const SOCK_PACKET: __socket_type = 10;
pub const SOCK_DCCP: __socket_type = 6;
pub const SOCK_SEQPACKET: __socket_type = 5;
pub const SOCK_RDM: __socket_type = 4;
pub const SOCK_RAW: __socket_type = 3;
pub const SOCK_DGRAM: __socket_type = 2;
pub const SOCK_STREAM: __socket_type = 1;
pub type sa_family_t = libc::c_ushort;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct sockaddr {
    pub sa_family: sa_family_t,
    pub sa_data: [libc::c_char; 14],
}
pub type __kernel_long_t = libc::c_long;
pub type __kernel_ulong_t = libc::c_ulong;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct sockaddr_un {
    pub sun_family: sa_family_t,
    pub sun_path: [libc::c_char; 108],
}
pub type uint8_t = __uint8_t;
pub type uint16_t = __uint16_t;
pub type wchar_t = libc::c_int;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _IO_FILE {
    pub _flags: libc::c_int,
    pub _IO_read_ptr: *mut libc::c_char,
    pub _IO_read_end: *mut libc::c_char,
    pub _IO_read_base: *mut libc::c_char,
    pub _IO_write_base: *mut libc::c_char,
    pub _IO_write_ptr: *mut libc::c_char,
    pub _IO_write_end: *mut libc::c_char,
    pub _IO_buf_base: *mut libc::c_char,
    pub _IO_buf_end: *mut libc::c_char,
    pub _IO_save_base: *mut libc::c_char,
    pub _IO_backup_base: *mut libc::c_char,
    pub _IO_save_end: *mut libc::c_char,
    pub _markers: *mut _IO_marker,
    pub _chain: *mut _IO_FILE,
    pub _fileno: libc::c_int,
    pub _flags2: libc::c_int,
    pub _old_offset: __off_t,
    pub _cur_column: libc::c_ushort,
    pub _vtable_offset: libc::c_schar,
    pub _shortbuf: [libc::c_char; 1],
    pub _lock: *mut libc::c_void,
    pub _offset: __off64_t,
    pub _codecvt: *mut _IO_codecvt,
    pub _wide_data: *mut _IO_wide_data,
    pub _freeres_list: *mut _IO_FILE,
    pub _freeres_buf: *mut libc::c_void,
    pub __pad5: size_t,
    pub _mode: libc::c_int,
    pub _unused2: [libc::c_char; 20],
}
pub type _IO_lock_t = ();
pub type FILE = _IO_FILE;
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
pub type LogLevel = libc::c_uint;
pub const error: LogLevel = 4;
pub const crit: LogLevel = 3;
pub const warn: LogLevel = 2;
pub const info: LogLevel = 1;
pub const debug: LogLevel = 0;
pub type EHandleType = libc::c_uint;
pub const SEMAPHORE: EHandleType = 1;
pub const EVENT: EHandleType = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct Handle {
    pub c2rust_unnamed: C2RustUnnamed_5,
    pub type_0: EHandleType,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_5 {
    pub event: Event,
    pub semaphore: sem_t,
}
pub type __u16 = libc::c_ushort;
pub type __u32 = libc::c_uint;
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
pub struct MagicVersion {
    pub Magic: [uint8_t; 14],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct IpcHeader {
    pub c2rust_unnamed: C2RustUnnamed_6,
    pub Size: uint16_t,
    pub CommandSet: uint8_t,
    pub CommandId: uint8_t,
    pub Reserved: uint16_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_6 {
    pub _magic: MagicVersion,
    pub Magic: [uint8_t; 14],
}
pub type ECoreDumpType = libc::c_uint;
pub const MANUAL: ECoreDumpType = 6;
pub const TIME: ECoreDumpType = 5;
pub const SIGNAL: ECoreDumpType = 4;
pub const FILEDESC: ECoreDumpType = 3;
pub const THREAD: ECoreDumpType = 2;
pub const CPU: ECoreDumpType = 1;
pub const COMMIT: ECoreDumpType = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct CoreDumpWriter {
    pub Config: *mut ProcDumpConfiguration,
    pub Type: ECoreDumpType,
}
static mut CoreDumpTypeStrings: [*const libc::c_char; 7] = [
    b"commit\0" as *const u8 as *const libc::c_char,
    b"cpu\0" as *const u8 as *const libc::c_char,
    b"thread\0" as *const u8 as *const libc::c_char,
    b"filedesc\0" as *const u8 as *const libc::c_char,
    b"signal\0" as *const u8 as *const libc::c_char,
    b"time\0" as *const u8 as *const libc::c_char,
    b"manual\0" as *const u8 as *const libc::c_char,
];
pub unsafe extern "C" fn NewCoreDumpWriter(
    mut type_0: ECoreDumpType,
    mut config: *mut ProcDumpConfiguration,
) -> *mut CoreDumpWriter {
    let mut writer: *mut CoreDumpWriter = malloc(
        ::std::mem::size_of::<CoreDumpWriter>() as libc::c_ulong,
    ) as *mut CoreDumpWriter;
    if writer.is_null() {
        Log(
            error,
            b"Internal Error has occurred. If problem continues to occur run procdump with -d flag to trace issue\0"
                as *const u8 as *const libc::c_char,
        );
        DiagTrace(
            b"NewCoreDumpWriter: failed to allocate memory. %s\0" as *const u8
                as *const libc::c_char,
            b"in src/CoreDumpWriter.c, at line 39\0" as *const u8 as *const libc::c_char,
        );
        exit(-(1 as libc::c_int));
    }
    (*writer).Config = config;
    (*writer).Type = type_0;
    return writer;
}
pub unsafe extern "C" fn GetPath(mut lineBuf: *mut libc::c_char) -> *mut libc::c_char {
    let mut delim: [libc::c_char; 2] = *::std::mem::transmute::<
        &[u8; 2],
        &mut [libc::c_char; 2],
    >(b" \0");
    let mut ptr: *mut libc::c_char = strtok(lineBuf, delim.as_mut_ptr());
    let mut i: libc::c_int = 0 as libc::c_int;
    while i < 7 as libc::c_int {
        ptr = strtok(0 as *mut libc::c_char, delim.as_mut_ptr());
        i += 1;
        i;
    }
    if !ptr.is_null() {
        *ptr
            .offset(
                (strlen(ptr)).wrapping_sub(1 as libc::c_int as libc::c_ulong) as isize,
            ) = '\0' as i32 as libc::c_char;
    }
    return ptr;
}
pub unsafe extern "C" fn IsCoreClrProcess(
    mut self_0: *mut CoreDumpWriter,
    mut socketName: *mut *mut libc::c_char,
) -> bool {
    let mut bRet: bool = 0 as libc::c_int != 0;
    *socketName = 0 as *mut libc::c_char;
    let mut procFile: *mut FILE = 0 as *mut FILE;
    let mut lineBuf: [libc::c_char; 4096] = [0; 4096];
    let mut tmpFolder: [libc::c_char; 4096] = [0; 4096];
    let mut prefixTmpFolder: *mut libc::c_char = 0 as *mut libc::c_char;
    prefixTmpFolder = getenv(b"TMPDIR\0" as *const u8 as *const libc::c_char);
    if prefixTmpFolder.is_null() {
        snprintf(
            tmpFolder.as_mut_ptr(),
            4096 as libc::c_int as libc::c_ulong,
            b"/tmp/dotnet-diagnostic-%d\0" as *const u8 as *const libc::c_char,
            (*(*self_0).Config).ProcessId,
        );
    } else {
        strncpy(
            tmpFolder.as_mut_ptr(),
            prefixTmpFolder,
            4096 as libc::c_int as libc::c_ulong,
        );
    }
    procFile = fopen(
        b"/proc/net/unix\0" as *const u8 as *const libc::c_char,
        b"r\0" as *const u8 as *const libc::c_char,
    );
    if !procFile.is_null() {
        fgets(
            lineBuf.as_mut_ptr(),
            ::std::mem::size_of::<[libc::c_char; 4096]>() as libc::c_ulong
                as libc::c_int,
            procFile,
        );
        while !(fgets(lineBuf.as_mut_ptr(), 4096 as libc::c_int, procFile)).is_null() {
            let mut ptr: *mut libc::c_char = GetPath(lineBuf.as_mut_ptr());
            if ptr.is_null() {
                continue;
            }
            if !(strncmp(ptr, tmpFolder.as_mut_ptr(), strlen(tmpFolder.as_mut_ptr()))
                == 0 as libc::c_int)
            {
                continue;
            }
            *socketName = malloc(
                (::std::mem::size_of::<libc::c_char>() as libc::c_ulong)
                    .wrapping_mul(strlen(ptr))
                    .wrapping_add(1 as libc::c_int as libc::c_ulong),
            ) as *mut libc::c_char;
            if (*socketName).is_null() {
                continue;
            }
            memset(
                *socketName as *mut libc::c_void,
                0 as libc::c_int,
                (::std::mem::size_of::<libc::c_char>() as libc::c_ulong)
                    .wrapping_mul(strlen(ptr))
                    .wrapping_add(1 as libc::c_int as libc::c_ulong),
            );
            if !(strncpy(
                *socketName,
                ptr,
                (::std::mem::size_of::<libc::c_char>() as libc::c_ulong)
                    .wrapping_mul(strlen(ptr))
                    .wrapping_add(1 as libc::c_int as libc::c_ulong),
            ))
                .is_null()
            {
                DiagTrace(
                    b"CoreCLR diagnostics socket: %s %s\0" as *const u8
                        as *const libc::c_char,
                    socketName,
                    b"in src/CoreDumpWriter.c, at line 140\0" as *const u8
                        as *const libc::c_char,
                );
                bRet = 1 as libc::c_int != 0;
            }
            break;
        }
    } else {
        DiagTrace(
            b"Failed to open /proc/net/unix [%d]. %s\0" as *const u8
                as *const libc::c_char,
            *__errno_location(),
            b"in src/CoreDumpWriter.c, at line 151\0" as *const u8 as *const libc::c_char,
        );
    }
    if !procFile.is_null() {
        fclose(procFile);
        procFile = 0 as *mut FILE;
    }
    if !(*socketName).is_null() && bRet as libc::c_int == 0 as libc::c_int {
        free(*socketName as *mut libc::c_void);
        *socketName = 0 as *mut libc::c_char;
    }
    return bRet;
}
pub unsafe extern "C" fn WriteCoreDump(mut self_0: *mut CoreDumpWriter) -> libc::c_int {
    let mut rc: libc::c_int = 0 as libc::c_int;
    rc = WaitForQuitOrEvent(
        (*self_0).Config,
        &mut (*(*self_0).Config).semAvailableDumpSlots,
        -(1 as libc::c_int),
    );
    if rc == 0 as libc::c_int {
        Log(
            error,
            b"Internal Error has occurred. If problem continues to occur run procdump with -d flag to trace issue\0"
                as *const u8 as *const libc::c_char,
        );
        DiagTrace(
            b"WriteCoreDump: failed WaitForQuitOrEvent. %s\0" as *const u8
                as *const libc::c_char,
            b"in src/CoreDumpWriter.c, at line 189\0" as *const u8 as *const libc::c_char,
        );
        exit(-(1 as libc::c_int));
    }
    if pthread_setcanceltype(
        PTHREAD_CANCEL_DEFERRED as libc::c_int,
        0 as *mut libc::c_int,
    ) != 0 as libc::c_int
    {
        Log(
            error,
            b"Internal Error has occurred. If problem continues to occur run procdump with -d flag to trace issue\0"
                as *const u8 as *const libc::c_char,
        );
        DiagTrace(
            b"WriteCoreDump: failed pthread_setcanceltype. %s\0" as *const u8
                as *const libc::c_char,
            b"in src/CoreDumpWriter.c, at line 194\0" as *const u8 as *const libc::c_char,
        );
        exit(-(1 as libc::c_int));
    }
    match rc {
        0 => {}
        1 => {
            let mut socketName: *mut libc::c_char = 0 as *mut libc::c_char;
            IsCoreClrProcess(self_0, &mut socketName);
            rc = WriteCoreDumpInternal(self_0, socketName);
            if rc == 0 as libc::c_int {
                if sem_post(
                    &mut (*(*self_0).Config)
                        .semAvailableDumpSlots
                        .c2rust_unnamed
                        .semaphore,
                ) == -(1 as libc::c_int)
                {
                    Log(
                        error,
                        b"Internal Error has occurred. If problem continues to occur run procdump with -d flag to trace issue\0"
                            as *const u8 as *const libc::c_char,
                    );
                    DiagTrace(
                        b"WriteCoreDump: failed sem_post. %s\0" as *const u8
                            as *const libc::c_char,
                        b"in src/CoreDumpWriter.c, at line 208\0" as *const u8
                            as *const libc::c_char,
                    );
                    if !socketName.is_null() {
                        free(socketName as *mut libc::c_void);
                    }
                    exit(-(1 as libc::c_int));
                }
            }
            if !socketName.is_null() {
                free(socketName as *mut libc::c_void);
            }
        }
        128 => {}
        _ => {
            DiagTrace(
                b"WriteCoreDump: Error in default case %s\0" as *const u8
                    as *const libc::c_char,
                b"in src/CoreDumpWriter.c, at line 219\0" as *const u8
                    as *const libc::c_char,
            );
        }
    }
    if pthread_setcanceltype(
        PTHREAD_CANCEL_ASYNCHRONOUS as libc::c_int,
        0 as *mut libc::c_int,
    ) != 0 as libc::c_int
    {
        Log(
            error,
            b"Internal Error has occurred. If problem continues to occur run procdump with -d flag to trace issue\0"
                as *const u8 as *const libc::c_char,
        );
        DiagTrace(
            b"WriteCoreDump: failed pthread_setcanceltype. %s\0" as *const u8
                as *const libc::c_char,
            b"in src/CoreDumpWriter.c, at line 224\0" as *const u8 as *const libc::c_char,
        );
        exit(-(1 as libc::c_int));
    }
    return rc;
}
pub unsafe extern "C" fn GetUint16(mut buffer: *mut libc::c_char) -> *mut uint16_t {
    let mut dumpFileNameW: *mut uint16_t = 0 as *mut uint16_t;
    if !buffer.is_null() {
        dumpFileNameW = malloc(
            (strlen(buffer))
                .wrapping_add(1 as libc::c_int as libc::c_ulong)
                .wrapping_mul(::std::mem::size_of::<uint16_t>() as libc::c_ulong),
        ) as *mut uint16_t;
        let mut i: libc::c_int = 0 as libc::c_int;
        while (i as libc::c_ulong)
            < (strlen(buffer)).wrapping_add(1 as libc::c_int as libc::c_ulong)
        {
            *dumpFileNameW.offset(i as isize) = *buffer.offset(i as isize) as uint16_t;
            i += 1;
            i;
        }
    }
    return dumpFileNameW;
}
pub unsafe extern "C" fn GenerateCoreClrDump(
    mut socketName: *mut libc::c_char,
    mut dumpFileName: *mut libc::c_char,
) -> bool {
    let mut bRet: bool = 0 as libc::c_int != 0;
    let mut addr: sockaddr_un = {
        let mut init = sockaddr_un {
            sun_family: 0 as libc::c_int as sa_family_t,
            sun_path: [0; 108],
        };
        init
    };
    let mut fd: libc::c_int = 0 as libc::c_int;
    let mut dumpFileNameW: *mut uint16_t = 0 as *mut uint16_t;
    let mut temp_buffer: *mut libc::c_void = 0 as *mut libc::c_void;
    dumpFileNameW = GetUint16(dumpFileName);
    if !dumpFileNameW.is_null() {
        fd = socket(1 as libc::c_int, SOCK_STREAM as libc::c_int, 0 as libc::c_int);
        if fd == -(1 as libc::c_int) {
            DiagTrace(
                b"Failed to create socket for .NET Core dump generation. %s\0"
                    as *const u8 as *const libc::c_char,
                b"in src/CoreDumpWriter.c, at line 276\0" as *const u8
                    as *const libc::c_char,
            );
        } else {
            memset(
                &mut addr as *mut sockaddr_un as *mut libc::c_void,
                0 as libc::c_int,
                ::std::mem::size_of::<sockaddr_un>() as libc::c_ulong,
            );
            addr.sun_family = 1 as libc::c_int as sa_family_t;
            strncpy(
                (addr.sun_path).as_mut_ptr(),
                socketName,
                (::std::mem::size_of::<[libc::c_char; 108]>() as libc::c_ulong)
                    .wrapping_sub(1 as libc::c_int as libc::c_ulong),
            );
            if connect(
                fd,
                &mut addr as *mut sockaddr_un as *mut sockaddr,
                ::std::mem::size_of::<sockaddr_un>() as libc::c_ulong as socklen_t,
            ) == -(1 as libc::c_int)
            {
                DiagTrace(
                    b"Failed to connect to socket for .NET Core dump generation. %s\0"
                        as *const u8 as *const libc::c_char,
                    b"in src/CoreDumpWriter.c, at line 287\0" as *const u8
                        as *const libc::c_char,
                );
            } else {
                let mut dumpFileNameLen: libc::c_uint = (strlen(dumpFileName))
                    .wrapping_add(1 as libc::c_int as libc::c_ulong) as libc::c_uint;
                let mut payloadSize: libc::c_int = ::std::mem::size_of::<libc::c_uint>()
                    as libc::c_ulong as libc::c_int;
                payloadSize = (payloadSize as libc::c_ulong)
                    .wrapping_add(
                        (dumpFileNameLen as libc::c_ulong)
                            .wrapping_mul(
                                ::std::mem::size_of::<wchar_t>() as libc::c_ulong,
                            ),
                    ) as libc::c_int as libc::c_int;
                let mut dumpType: libc::c_uint = 4 as libc::c_int as libc::c_uint;
                payloadSize = (payloadSize as libc::c_ulong)
                    .wrapping_add(::std::mem::size_of::<libc::c_uint>() as libc::c_ulong)
                    as libc::c_int as libc::c_int;
                let mut diagnostics: libc::c_uint = 0 as libc::c_int as libc::c_uint;
                payloadSize = (payloadSize as libc::c_ulong)
                    .wrapping_add(::std::mem::size_of::<libc::c_uint>() as libc::c_ulong)
                    as libc::c_int as libc::c_int;
                let mut totalPacketSize: uint16_t = (::std::mem::size_of::<IpcHeader>()
                    as libc::c_ulong)
                    .wrapping_add(payloadSize as libc::c_ulong) as uint16_t;
                temp_buffer = malloc(totalPacketSize as libc::c_ulong);
                if !temp_buffer.is_null() {
                    memset(
                        temp_buffer,
                        0 as libc::c_int,
                        totalPacketSize as libc::c_ulong,
                    );
                    let mut dumpHeader: IpcHeader = {
                        let mut init = IpcHeader {
                            c2rust_unnamed: C2RustUnnamed_6 {
                                _magic: {
                                    let mut init = MagicVersion {
                                        Magic: *::std::mem::transmute::<
                                            &[u8; 14],
                                            &mut [uint8_t; 14],
                                        >(b"DOTNET_IPC_V1\0"),
                                    };
                                    init
                                },
                            },
                            Size: totalPacketSize,
                            CommandSet: 0x1 as libc::c_int as uint8_t,
                            CommandId: 0x1 as libc::c_int as uint8_t,
                            Reserved: 0 as libc::c_int as uint16_t,
                        };
                        init
                    };
                    let mut temp_buffer_cur: *mut libc::c_void = temp_buffer;
                    memcpy(
                        temp_buffer_cur,
                        &mut dumpHeader as *mut IpcHeader as *const libc::c_void,
                        ::std::mem::size_of::<IpcHeader>() as libc::c_ulong,
                    );
                    temp_buffer_cur = temp_buffer_cur
                        .offset(
                            ::std::mem::size_of::<IpcHeader>() as libc::c_ulong as isize,
                        );
                    memcpy(
                        temp_buffer_cur,
                        &mut dumpFileNameLen as *mut libc::c_uint as *const libc::c_void,
                        ::std::mem::size_of::<libc::c_uint>() as libc::c_ulong,
                    );
                    temp_buffer_cur = temp_buffer_cur
                        .offset(
                            ::std::mem::size_of::<libc::c_uint>() as libc::c_ulong
                                as isize,
                        );
                    memcpy(
                        temp_buffer_cur,
                        dumpFileNameW as *const libc::c_void,
                        (dumpFileNameLen as libc::c_ulong)
                            .wrapping_mul(
                                ::std::mem::size_of::<uint16_t>() as libc::c_ulong,
                            ),
                    );
                    temp_buffer_cur = temp_buffer_cur
                        .offset(
                            (dumpFileNameLen as libc::c_ulong)
                                .wrapping_mul(
                                    ::std::mem::size_of::<uint16_t>() as libc::c_ulong,
                                ) as isize,
                        );
                    memcpy(
                        temp_buffer_cur,
                        &mut dumpType as *mut libc::c_uint as *const libc::c_void,
                        ::std::mem::size_of::<libc::c_uint>() as libc::c_ulong,
                    );
                    temp_buffer_cur = temp_buffer_cur
                        .offset(
                            ::std::mem::size_of::<libc::c_uint>() as libc::c_ulong
                                as isize,
                        );
                    memcpy(
                        temp_buffer_cur,
                        &mut diagnostics as *mut libc::c_uint as *const libc::c_void,
                        ::std::mem::size_of::<libc::c_uint>() as libc::c_ulong,
                    );
                    if send(fd, temp_buffer, totalPacketSize as size_t, 0 as libc::c_int)
                        == -(1 as libc::c_int) as libc::c_long
                    {
                        DiagTrace(
                            b"Failed sending packet to diagnostics server [%d] %s\0"
                                as *const u8 as *const libc::c_char,
                            *__errno_location(),
                            b"in src/CoreDumpWriter.c, at line 336\0" as *const u8
                                as *const libc::c_char,
                        );
                    } else {
                        let mut retHeader: IpcHeader = IpcHeader {
                            c2rust_unnamed: C2RustUnnamed_6 {
                                _magic: MagicVersion { Magic: [0; 14] },
                            },
                            Size: 0,
                            CommandSet: 0,
                            CommandId: 0,
                            Reserved: 0,
                        };
                        if recv(
                            fd,
                            &mut retHeader as *mut IpcHeader as *mut libc::c_void,
                            ::std::mem::size_of::<IpcHeader>() as libc::c_ulong,
                            0 as libc::c_int,
                        ) == -(1 as libc::c_int) as libc::c_long
                        {
                            DiagTrace(
                                b"Failed receiving response header from diagnostics server [%d] %s\0"
                                    as *const u8 as *const libc::c_char,
                                *__errno_location(),
                                b"in src/CoreDumpWriter.c, at line 344\0" as *const u8
                                    as *const libc::c_char,
                            );
                        } else if retHeader.Size as libc::c_int != 24 as libc::c_int {
                            DiagTrace(
                                b"Failed validating header size in response header from diagnostics server [%d != 24] %s\0"
                                    as *const u8 as *const libc::c_char,
                                retHeader.Size as libc::c_int,
                                b"in src/CoreDumpWriter.c, at line 351\0" as *const u8
                                    as *const libc::c_char,
                            );
                        } else {
                            let mut res: int32_t = -(1 as libc::c_int);
                            if recv(
                                fd,
                                &mut res as *mut int32_t as *mut libc::c_void,
                                ::std::mem::size_of::<int32_t>() as libc::c_ulong,
                                0 as libc::c_int,
                            ) == -(1 as libc::c_int) as libc::c_long
                            {
                                DiagTrace(
                                    b"Failed receiving result code from response payload from diagnostics server [%d] %s\0"
                                        as *const u8 as *const libc::c_char,
                                    *__errno_location(),
                                    b"in src/CoreDumpWriter.c, at line 359\0" as *const u8
                                        as *const libc::c_char,
                                );
                            } else if res == 0 as libc::c_int {
                                bRet = 1 as libc::c_int != 0;
                            }
                        }
                    }
                }
            }
        }
    }
    if !temp_buffer.is_null() {
        free(temp_buffer);
        temp_buffer = 0 as *mut libc::c_void;
    }
    if fd != 0 as libc::c_int {
        close(fd);
        fd = 0 as libc::c_int;
    }
    if !dumpFileNameW.is_null() {
        free(dumpFileNameW as *mut libc::c_void);
        dumpFileNameW = 0 as *mut uint16_t;
    }
    return bRet;
}
pub unsafe extern "C" fn WriteCoreDumpInternal(
    mut self_0: *mut CoreDumpWriter,
    mut socketName: *mut libc::c_char,
) -> libc::c_int {
    let mut date: [libc::c_char; 26] = [0; 26];
    let mut command: [libc::c_char; 1024] = [0; 1024];
    let mut outputBuffer: *mut *mut libc::c_char = 0 as *mut *mut libc::c_char;
    let mut lineBuffer: [libc::c_char; 1024] = [0; 1024];
    let mut gcorePrefixName: [libc::c_char; 1024] = [0; 1024];
    let mut coreDumpFileName: [libc::c_char; 1024] = [0; 1024];
    let mut lineLength: libc::c_int = 0;
    let mut i: libc::c_int = 0;
    let mut rc: libc::c_int = 0 as libc::c_int;
    let mut rawTime: time_t = 0;
    let mut gcorePid: pid_t = 0;
    let mut timerInfo: *mut tm = 0 as *mut tm;
    let mut commandPipe: *mut FILE = 0 as *mut FILE;
    let mut desc: *const libc::c_char = CoreDumpTypeStrings[(*self_0).Type as usize];
    let mut name: *mut libc::c_char = sanitize((*(*self_0).Config).ProcessName);
    let mut pid: pid_t = (*(*self_0).Config).ProcessId;
    outputBuffer = malloc(
        (::std::mem::size_of::<*mut libc::c_char>() as libc::c_ulong)
            .wrapping_mul(15 as libc::c_int as libc::c_ulong),
    ) as *mut *mut libc::c_char;
    if outputBuffer.is_null() {
        Log(
            error,
            b"Internal Error has occurred. If problem continues to occur run procdump with -d flag to trace issue\0"
                as *const u8 as *const libc::c_char,
        );
        DiagTrace(
            b"WriteCoreDumpInternal: failed gcore output buffer allocation %s\0"
                as *const u8 as *const libc::c_char,
            b"in src/CoreDumpWriter.c, at line 428\0" as *const u8 as *const libc::c_char,
        );
        exit(-(1 as libc::c_int));
    }
    rawTime = time(0 as *mut time_t);
    timerInfo = localtime(&mut rawTime);
    if timerInfo.is_null() {
        Log(
            error,
            b"Internal Error has occurred. If problem continues to occur run procdump with -d flag to trace issue\0"
                as *const u8 as *const libc::c_char,
        );
        DiagTrace(
            b"WriteCoreDumpInternal: failed localtime %s\0" as *const u8
                as *const libc::c_char,
            b"in src/CoreDumpWriter.c, at line 436\0" as *const u8 as *const libc::c_char,
        );
        exit(-(1 as libc::c_int));
    }
    strftime(
        date.as_mut_ptr(),
        26 as libc::c_int as size_t,
        b"%Y-%m-%d_%H:%M:%S\0" as *const u8 as *const libc::c_char,
        timerInfo,
    );
    if !((*(*self_0).Config).CoreDumpName).is_null() {
        if snprintf(
            gcorePrefixName.as_mut_ptr(),
            1024 as libc::c_int as libc::c_ulong,
            b"%s/%s_%d\0" as *const u8 as *const libc::c_char,
            (*(*self_0).Config).CoreDumpPath,
            (*(*self_0).Config).CoreDumpName,
            (*(*self_0).Config).NumberOfDumpsCollected,
        ) < 0 as libc::c_int
        {
            Log(
                error,
                b"Internal Error has occurred. If problem continues to occur run procdump with -d flag to trace issue\0"
                    as *const u8 as *const libc::c_char,
            );
            DiagTrace(
                b"WriteCoreDumpInternal: failed sprintf custom output file name %s\0"
                    as *const u8 as *const libc::c_char,
                b"in src/CoreDumpWriter.c, at line 448\0" as *const u8
                    as *const libc::c_char,
            );
            exit(-(1 as libc::c_int));
        }
    } else if snprintf(
        gcorePrefixName.as_mut_ptr(),
        1024 as libc::c_int as libc::c_ulong,
        b"%s/%s_%s_%s\0" as *const u8 as *const libc::c_char,
        (*(*self_0).Config).CoreDumpPath,
        name,
        desc,
        date.as_mut_ptr(),
    ) < 0 as libc::c_int
    {
        Log(
            error,
            b"Internal Error has occurred. If problem continues to occur run procdump with -d flag to trace issue\0"
                as *const u8 as *const libc::c_char,
        );
        DiagTrace(
            b"WriteCoreDumpInternal: failed sprintf default output file name %s\0"
                as *const u8 as *const libc::c_char,
            b"in src/CoreDumpWriter.c, at line 455\0" as *const u8 as *const libc::c_char,
        );
        exit(-(1 as libc::c_int));
    }
    if snprintf(
        command.as_mut_ptr(),
        1024 as libc::c_int as libc::c_ulong,
        b"gcore -o %s %d 2>&1\0" as *const u8 as *const libc::c_char,
        gcorePrefixName.as_mut_ptr(),
        pid,
    ) < 0 as libc::c_int
    {
        Log(
            error,
            b"Internal Error has occurred. If problem continues to occur run procdump with -d flag to trace issue\0"
                as *const u8 as *const libc::c_char,
        );
        DiagTrace(
            b"WriteCoreDumpInternal: failed sprintf gcore command %s\0" as *const u8
                as *const libc::c_char,
            b"in src/CoreDumpWriter.c, at line 463\0" as *const u8 as *const libc::c_char,
        );
        exit(-(1 as libc::c_int));
    }
    if snprintf(
        coreDumpFileName.as_mut_ptr(),
        1024 as libc::c_int as libc::c_ulong,
        b"%s.%d\0" as *const u8 as *const libc::c_char,
        gcorePrefixName.as_mut_ptr(),
        pid,
    ) < 0 as libc::c_int
    {
        Log(
            error,
            b"Internal Error has occurred. If problem continues to occur run procdump with -d flag to trace issue\0"
                as *const u8 as *const libc::c_char,
        );
        DiagTrace(
            b"WriteCoreDumpInternal: failed sprintf core file name %s\0" as *const u8
                as *const libc::c_char,
            b"in src/CoreDumpWriter.c, at line 470\0" as *const u8 as *const libc::c_char,
        );
        exit(-(1 as libc::c_int));
    }
    if access((*(*self_0).Config).CoreDumpPath, 2 as libc::c_int) < 0 as libc::c_int {
        Log(
            error,
            b"Internal Error has occurred. If problem continues to occur run procdump with -d flag to trace issue\0"
                as *const u8 as *const libc::c_char,
        );
        DiagTrace(
            b"WriteCoreDumpInternal: no write permission to core dump target file %s %s\0"
                as *const u8 as *const libc::c_char,
            coreDumpFileName.as_mut_ptr(),
            b"in src/CoreDumpWriter.c, at line 478\0" as *const u8 as *const libc::c_char,
        );
        exit(-(1 as libc::c_int));
    }
    if !socketName.is_null() {
        if GenerateCoreClrDump(socketName, coreDumpFileName.as_mut_ptr()) as libc::c_int
            == 0 as libc::c_int
        {
            Log(
                error,
                b"An error occurred while generating the core dump for .NET 3.x+ process\0"
                    as *const u8 as *const libc::c_char,
            );
        } else {
            Log(
                info,
                b"Core dump %d generated: %s\0" as *const u8 as *const libc::c_char,
                (*(*self_0).Config).NumberOfDumpsCollected,
                coreDumpFileName.as_mut_ptr(),
            );
            (*(*self_0).Config).NumberOfDumpsCollected += 1;
            (*(*self_0).Config).NumberOfDumpsCollected;
            if (*(*self_0).Config).NumberOfDumpsCollected
                >= (*(*self_0).Config).NumberOfDumpsToCollect
            {
                SetEvent(&mut (*(*self_0).Config).evtQuit.c2rust_unnamed.event);
                rc = 1 as libc::c_int;
            }
        }
        free(outputBuffer as *mut libc::c_void);
    } else {
        commandPipe = popen2(
            command.as_mut_ptr(),
            b"r\0" as *const u8 as *const libc::c_char,
            &mut gcorePid,
        );
        (*(*self_0).Config).gcorePid = gcorePid;
        if commandPipe.is_null() {
            Log(
                error,
                b"An error occurred while generating the core dump\0" as *const u8
                    as *const libc::c_char,
            );
            DiagTrace(
                b"WriteCoreDumpInternal: Failed to open pipe to gcore %s\0" as *const u8
                    as *const libc::c_char,
                b"in src/CoreDumpWriter.c, at line 512\0" as *const u8
                    as *const libc::c_char,
            );
            exit(1 as libc::c_int);
        }
        i = 0 as libc::c_int;
        while i < 15 as libc::c_int
            && !(fgets(
                lineBuffer.as_mut_ptr(),
                ::std::mem::size_of::<[libc::c_char; 1024]>() as libc::c_ulong
                    as libc::c_int,
                commandPipe,
            ))
                .is_null()
        {
            lineLength = strlen(lineBuffer.as_mut_ptr()) as libc::c_int;
            let ref mut fresh0 = *outputBuffer.offset(i as isize);
            *fresh0 = malloc(
                (::std::mem::size_of::<libc::c_char>() as libc::c_ulong)
                    .wrapping_mul(lineLength as libc::c_ulong),
            ) as *mut libc::c_char;
            if !(*outputBuffer.offset(i as isize)).is_null() {
                strncpy(
                    *outputBuffer.offset(i as isize),
                    lineBuffer.as_mut_ptr(),
                    (lineLength - 1 as libc::c_int) as libc::c_ulong,
                );
                *(*outputBuffer.offset(i as isize))
                    .offset(
                        (lineLength - 1 as libc::c_int) as isize,
                    ) = '\0' as i32 as libc::c_char;
            } else {
                Log(
                    error,
                    b"Internal Error has occurred. If problem continues to occur run procdump with -d flag to trace issue\0"
                        as *const u8 as *const libc::c_char,
                );
                DiagTrace(
                    b"WriteCoreDumpInternal: failed to allocate gcore error message buffer %s\0"
                        as *const u8 as *const libc::c_char,
                    b"in src/CoreDumpWriter.c, at line 527\0" as *const u8
                        as *const libc::c_char,
                );
                exit(-(1 as libc::c_int));
            }
            i += 1;
            i;
        }
        (*(*self_0).Config).gcorePid = 2147483647 as libc::c_int;
        pclose(commandPipe);
        if !(strstr(
            *outputBuffer.offset((i - 1 as libc::c_int) as isize),
            b"gcore: failed\0" as *const u8 as *const libc::c_char,
        ))
            .is_null()
        {
            Log(
                error,
                b"An error occurred while generating the core dump\0" as *const u8
                    as *const libc::c_char,
            );
            let mut j: libc::c_int = 0 as libc::c_int;
            while j < i {
                if !(*outputBuffer.offset(j as isize)).is_null() {
                    Log(
                        error,
                        b"GCORE - %s\0" as *const u8 as *const libc::c_char,
                        *outputBuffer.offset(j as isize),
                    );
                }
                j += 1;
                j;
            }
            exit(1 as libc::c_int);
        }
        let mut j_0: libc::c_int = 0 as libc::c_int;
        while j_0 < i {
            free(*outputBuffer.offset(j_0 as isize) as *mut libc::c_void);
            j_0 += 1;
            j_0;
        }
        free(outputBuffer as *mut libc::c_void);
        sleep(1 as libc::c_int as libc::c_uint);
        if access(coreDumpFileName.as_mut_ptr(), 0 as libc::c_int) != -(1 as libc::c_int)
        {
            if (*(*self_0).Config).nQuit != 0 {
                let mut ret: libc::c_int = unlink(coreDumpFileName.as_mut_ptr());
                if ret < 0 as libc::c_int && *__errno_location() != 2 as libc::c_int {
                    DiagTrace(
                        b"WriteCoreDumpInternal: Failed to remove partial core dump %s\0"
                            as *const u8 as *const libc::c_char,
                        b"in src/CoreDumpWriter.c, at line 564\0" as *const u8
                            as *const libc::c_char,
                    );
                    exit(-(1 as libc::c_int));
                }
            } else {
                Log(
                    info,
                    b"Core dump %d generated: %s\0" as *const u8 as *const libc::c_char,
                    (*(*self_0).Config).NumberOfDumpsCollected,
                    coreDumpFileName.as_mut_ptr(),
                );
                (*(*self_0).Config).NumberOfDumpsCollected += 1;
                (*(*self_0).Config).NumberOfDumpsCollected;
                if (*(*self_0).Config).NumberOfDumpsCollected
                    >= (*(*self_0).Config).NumberOfDumpsToCollect
                {
                    SetEvent(&mut (*(*self_0).Config).evtQuit.c2rust_unnamed.event);
                    rc = 1 as libc::c_int;
                }
            }
        }
    }
    free(name as *mut libc::c_void);
    return rc;
}
pub unsafe extern "C" fn popen2(
    mut command: *const libc::c_char,
    mut type_0: *const libc::c_char,
    mut pid: *mut pid_t,
) -> *mut FILE {
    let mut pipefd: [libc::c_int; 2] = [0; 2];
    let mut childPid: pid_t = 0;
    if pipe(pipefd.as_mut_ptr()) == -(1 as libc::c_int) {
        Log(
            error,
            b"Internal Error has occurred. If problem continues to occur run procdump with -d flag to trace issue\0"
                as *const u8 as *const libc::c_char,
        );
        DiagTrace(
            b"popen2: unable to open pipe %s\0" as *const u8 as *const libc::c_char,
            b"in src/CoreDumpWriter.c, at line 605\0" as *const u8 as *const libc::c_char,
        );
        exit(-(1 as libc::c_int));
    }
    childPid = fork();
    if childPid == -(1 as libc::c_int) {
        Log(
            error,
            b"Internal Error has occurred. If problem continues to occur run procdump with -d flag to trace issue\0"
                as *const u8 as *const libc::c_char,
        );
        DiagTrace(
            b"popen2: unable to fork process %s\0" as *const u8 as *const libc::c_char,
            b"in src/CoreDumpWriter.c, at line 612\0" as *const u8 as *const libc::c_char,
        );
        exit(-(1 as libc::c_int));
    }
    if childPid == 0 as libc::c_int {
        setpgid(0 as libc::c_int, 0 as libc::c_int);
        if *type_0.offset(0 as libc::c_int as isize) as libc::c_int == 'r' as i32 {
            close(pipefd[0 as libc::c_int as usize]);
            dup2(pipefd[1 as libc::c_int as usize], 1 as libc::c_int);
        } else {
            close(pipefd[1 as libc::c_int as usize]);
            dup2(pipefd[0 as libc::c_int as usize], 0 as libc::c_int);
        }
        execl(
            b"/bin/bash\0" as *const u8 as *const libc::c_char,
            b"bash\0" as *const u8 as *const libc::c_char,
            b"-c\0" as *const u8 as *const libc::c_char,
            command,
            0 as *mut libc::c_void as *mut libc::c_char,
        );
        return 0 as *mut FILE;
    } else {
        setpgid(childPid, childPid);
        *pid = childPid;
        if *type_0.offset(0 as libc::c_int as isize) as libc::c_int == 'r' as i32 {
            close(pipefd[1 as libc::c_int as usize]);
            return fdopen(
                pipefd[0 as libc::c_int as usize],
                b"r\0" as *const u8 as *const libc::c_char,
            );
        } else {
            close(pipefd[0 as libc::c_int as usize]);
            return fdopen(
                pipefd[1 as libc::c_int as usize],
                b"w\0" as *const u8 as *const libc::c_char,
            );
        }
    };
}
pub unsafe extern "C" fn sanitize(
    mut processName: *mut libc::c_char,
) -> *mut libc::c_char {
    if processName.is_null() {
        Log(error, b"NULL process name.\n\0" as *const u8 as *const libc::c_char);
        exit(-(1 as libc::c_int));
    }
    let mut sanitizedProcessName: *mut libc::c_char = strdup(processName);
    let mut i: libc::c_int = 0 as libc::c_int;
    while (i as libc::c_ulong) < strlen(sanitizedProcessName) {
        if *(*__ctype_b_loc())
            .offset(*sanitizedProcessName.offset(i as isize) as libc::c_int as isize)
            as libc::c_int & _ISalnum as libc::c_int as libc::c_ushort as libc::c_int
            == 0
        {
            *sanitizedProcessName.offset(i as isize) = '_' as i32 as libc::c_char;
        }
        i += 1;
        i;
    }
    return sanitizedProcessName;
}
