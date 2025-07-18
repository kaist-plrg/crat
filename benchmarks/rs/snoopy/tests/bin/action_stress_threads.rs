use ::libc;
extern "C" {
    fn fatalError(message: *const libc::c_char);
    fn close(__fd: libc::c_int) -> libc::c_int;
    fn read(__fd: libc::c_int, __buf: *mut libc::c_void, __nbytes: size_t) -> ssize_t;
    fn snoopy_entrypoint_test_cli_threads_init();
    fn snoopy_entrypoint_test_cli_exit();
    fn snoopy_configuration_preinit_disableConfigFileParsing();
    fn snoopy_datasourceregistry_getCount() -> libc::c_int;
    fn snoopy_datasourceregistry_getName(datasourceId: libc::c_int) -> *mut libc::c_char;
    fn snoopy_datasourceregistry_callById(
        datasourceId: libc::c_int,
        result: *mut libc::c_char,
        datasourceArg: *const libc::c_char,
    ) -> libc::c_int;
    fn pthread_create(
        __newthread: *mut pthread_t,
        __attr: *const pthread_attr_t,
        __start_routine: Option::<
            unsafe extern "C" fn(*mut libc::c_void) -> *mut libc::c_void,
        >,
        __arg: *mut libc::c_void,
    ) -> libc::c_int;
    fn pthread_join(
        __th: pthread_t,
        __thread_return: *mut *mut libc::c_void,
    ) -> libc::c_int;
    fn pthread_self() -> pthread_t;
    fn pthread_mutex_lock(__mutex: *mut pthread_mutex_t) -> libc::c_int;
    fn pthread_mutex_unlock(__mutex: *mut pthread_mutex_t) -> libc::c_int;
    fn snoopy_tsrm_get_threadCount() -> libc::c_int;
    fn nanosleep(
        __requested_time: *const timespec,
        __remaining: *mut timespec,
    ) -> libc::c_int;
    fn open(__file: *const libc::c_char, __oflag: libc::c_int, _: ...) -> libc::c_int;
    fn printf(_: *const libc::c_char, _: ...) -> libc::c_int;
    fn strtol(
        _: *const libc::c_char,
        _: *mut *mut libc::c_char,
        _: libc::c_int,
    ) -> libc::c_long;
    fn malloc(_: libc::c_ulong) -> *mut libc::c_void;
    fn free(__ptr: *mut libc::c_void);
    fn strcmp(_: *const libc::c_char, _: *const libc::c_char) -> libc::c_int;
}
pub type __time_t = libc::c_long;
pub type __ssize_t = libc::c_long;
pub type __syscall_slong_t = libc::c_long;
pub type ssize_t = __ssize_t;
pub type size_t = libc::c_ulong;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct timespec {
    pub tv_sec: __time_t,
    pub tv_nsec: __syscall_slong_t,
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
pub type pthread_t = libc::c_ulong;
#[derive(Copy, Clone)]
#[repr(C)]
pub union pthread_attr_t {
    pub __size: [libc::c_char; 56],
    pub __align: libc::c_long,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union pthread_mutex_t {
    pub __data: __pthread_mutex_s,
    pub __size: [libc::c_char; 40],
    pub __align: libc::c_long,
}
pub type C2RustUnnamed = libc::c_uint;
pub const PTHREAD_MUTEX_DEFAULT: C2RustUnnamed = 0;
pub const PTHREAD_MUTEX_ERRORCHECK: C2RustUnnamed = 2;
pub const PTHREAD_MUTEX_RECURSIVE: C2RustUnnamed = 1;
pub const PTHREAD_MUTEX_NORMAL: C2RustUnnamed = 0;
pub const PTHREAD_MUTEX_ADAPTIVE_NP: C2RustUnnamed = 3;
pub const PTHREAD_MUTEX_ERRORCHECK_NP: C2RustUnnamed = 2;
pub const PTHREAD_MUTEX_RECURSIVE_NP: C2RustUnnamed = 1;
pub const PTHREAD_MUTEX_TIMED_NP: C2RustUnnamed = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct tData_t {
    pub seqNr: libc::c_int,
}
#[inline]
unsafe extern "C" fn atoi(mut __nptr: *const libc::c_char) -> libc::c_int {
    return strtol(
        __nptr,
        0 as *mut libc::c_void as *mut *mut libc::c_char,
        10 as libc::c_int,
    ) as libc::c_int;
}
pub static mut snoopyTestCli_action_stress_threads_tRepo: [pthread_t; 10000] = [0; 10000];
pub static mut threadCountMutex: pthread_mutex_t = pthread_mutex_t {
    __data: {
        let mut init = __pthread_mutex_s {
            __lock: 0 as libc::c_int,
            __count: 0 as libc::c_int as libc::c_uint,
            __owner: 0 as libc::c_int,
            __nusers: 0 as libc::c_int as libc::c_uint,
            __kind: PTHREAD_MUTEX_TIMED_NP as libc::c_int,
            __spins: 0 as libc::c_int as libc::c_short,
            __elision: 0 as libc::c_int as libc::c_short,
            __list: {
                let mut init = __pthread_internal_list {
                    __prev: 0 as *const __pthread_internal_list
                        as *mut __pthread_internal_list,
                    __next: 0 as *const __pthread_internal_list
                        as *mut __pthread_internal_list,
                };
                init
            },
        };
        init
    },
};
pub static mut threadCountCreated: libc::c_int = 0 as libc::c_int;
pub static mut threadCountAliveNow: libc::c_int = 0 as libc::c_int;
pub static mut threadCountAliveMax: libc::c_int = 0 as libc::c_int;
pub static mut verbose: libc::c_int = 0;
pub unsafe extern "C" fn snoopyTestCli_action_stress_threads_showHelp() {
    let mut helpContent: *mut libc::c_char = b"Snoopy TEST SUITE CLI utility :: Action `stress` :: Subsystem `threads`\n\nUsage:\n    snoopy-test stress threads THREAD_COUNT [-v]\n\nDescription:\n    Stresses Snoopy's threading implementation by creating and destroying THREAD_COUNT\n    threads as fast as possible.\n\nArguments:\n    THREAD_COUNT       Number of threads to create and destroy\n    -v                 Verbose debugging output\n\nOutput:\n    Various threading-related messages and some statistics at the end.\n\n\0"
        as *const u8 as *const libc::c_char as *mut libc::c_char;
    printf(b"%s\0" as *const u8 as *const libc::c_char, helpContent);
}
pub unsafe extern "C" fn snoopyTestCli_action_stress_threads(
    mut argc: libc::c_int,
    mut argv: *mut *mut libc::c_char,
) -> libc::c_int {
    let mut threadsToCreate: libc::c_int = 0;
    let mut maxConcurrentThreadsObserved: libc::c_int = 0 as libc::c_int;
    let mut retVal: libc::c_int = 0 as libc::c_int;
    if argc < 1 as libc::c_int {
        snoopyTestCli_action_stress_threads_showHelp();
        fatalError(
            b"Missing argument: number of threads to create\0" as *const u8
                as *const libc::c_char,
        );
    }
    threadsToCreate = atoi(*argv.offset(0 as libc::c_int as isize));
    if threadsToCreate < 1 as libc::c_int || threadsToCreate > 10000 as libc::c_int {
        snoopyTestCli_action_stress_threads_showHelp();
        fatalError(
            b"Invalid number of threads to create (min 1, max THREAD_COUNT_MAX)\0"
                as *const u8 as *const libc::c_char,
        );
    }
    if argc >= 2 as libc::c_int
        && 0 as libc::c_int
            == strcmp(
                *argv.offset(1 as libc::c_int as isize),
                b"-v\0" as *const u8 as *const libc::c_char,
            )
    {
        verbose = 1 as libc::c_int;
    } else {
        verbose = 0 as libc::c_int;
    }
    snoopy_configuration_preinit_disableConfigFileParsing();
    printf(b"M: Starting threads... \0" as *const u8 as *const libc::c_char);
    let mut i: libc::c_int = 0 as libc::c_int;
    while i < threadsToCreate {
        let mut tArgs: *mut tData_t = malloc(
            ::std::mem::size_of::<tData_t>() as libc::c_ulong,
        ) as *mut tData_t;
        (*tArgs).seqNr = i;
        if verbose != 0 {
            printf(
                b" M: Starting thread #%d:\n\0" as *const u8 as *const libc::c_char,
                i + 1 as libc::c_int,
            );
        }
        retVal = pthread_create(
            &mut *snoopyTestCli_action_stress_threads_tRepo
                .as_mut_ptr()
                .offset(i as isize),
            0 as *const pthread_attr_t,
            Some(
                snoopyTestCli_action_stress_threads_threadMain
                    as unsafe extern "C" fn(*mut libc::c_void) -> *mut libc::c_void,
            ),
            tArgs as *mut libc::c_void,
        );
        i += 1;
        i;
    }
    printf(b"done.\n\0" as *const u8 as *const libc::c_char);
    printf(
        b"M: Threads alive right after thread creation was completed: %d\n\0"
            as *const u8 as *const libc::c_char,
        threadCountAliveNow,
    );
    if verbose != 0 {
        let mut ts_sleep: timespec = timespec { tv_sec: 0, tv_nsec: 0 };
        ts_sleep.tv_sec = 0 as libc::c_int as __time_t;
        ts_sleep.tv_nsec = 200000000 as libc::c_int as __syscall_slong_t;
        nanosleep(&mut ts_sleep, 0 as *mut timespec);
        maxConcurrentThreadsObserved = snoopy_tsrm_get_threadCount();
        printf(
            b"M: Threads after first sleep: %d\n\0" as *const u8 as *const libc::c_char,
            maxConcurrentThreadsObserved,
        );
        nanosleep(&mut ts_sleep, 0 as *mut timespec);
        printf(
            b"M: Threads after all threads are supposedly finished: %d\n\0" as *const u8
                as *const libc::c_char,
            snoopy_tsrm_get_threadCount(),
        );
    }
    printf(
        b"M: Waiting for all threads to finish... \0" as *const u8 as *const libc::c_char,
    );
    let mut i_0: libc::c_int = 0 as libc::c_int;
    while i_0 < threadsToCreate {
        pthread_join(
            snoopyTestCli_action_stress_threads_tRepo[i_0 as usize],
            0 as *mut *mut libc::c_void,
        );
        if verbose != 0 {
            printf(
                b"M: Thread joined: #%d\n\0" as *const u8 as *const libc::c_char,
                i_0 + 1 as libc::c_int,
            );
            printf(
                b" M: Thread #%d joined.\n\0" as *const u8 as *const libc::c_char,
                i_0 + 1 as libc::c_int,
            );
        }
        i_0 += 1;
        i_0;
    }
    printf(b"done.\n\0" as *const u8 as *const libc::c_char);
    printf(
        b"M: Number of threads created:        %d\n\0" as *const u8
            as *const libc::c_char,
        threadCountCreated,
    );
    printf(
        b"M: Max threads alive simultaneously: %d\n\0" as *const u8
            as *const libc::c_char,
        threadCountAliveMax,
    );
    if verbose != 0 {
        printf(
            b"M: Threads after all threads, except main, have finished: %d\n\0"
                as *const u8 as *const libc::c_char,
            snoopy_tsrm_get_threadCount(),
        );
    }
    if verbose != 0 {
        printf(
            b"M: Threads after all threads have finished: %d\n\0" as *const u8
                as *const libc::c_char,
            snoopy_tsrm_get_threadCount(),
        );
    }
    if verbose != 0 {
        printf(
            b"SUCCESS! Expected Snoopy threads count reached: %d\n\0" as *const u8
                as *const libc::c_char,
            maxConcurrentThreadsObserved,
        );
    }
    return retVal;
}
pub unsafe extern "C" fn snoopyTestCli_action_stress_threads_threadMain(
    mut args: *mut libc::c_void,
) -> *mut libc::c_void {
    let mut tArgs: *mut tData_t = args as *mut tData_t;
    let mut seqNr: libc::c_int = (*tArgs).seqNr;
    let mut seqNrPub: libc::c_int = seqNr + 1 as libc::c_int;
    let mut dsCount: libc::c_int = 0;
    let mut dsId: libc::c_int = 0;
    let mut dsName: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut dsArg: *const libc::c_char = b"\0" as *const u8 as *const libc::c_char;
    let mut dsResult: [libc::c_char; 2048] = [0; 2048];
    let mut retVal: libc::c_int = 0;
    pthread_mutex_lock(&mut threadCountMutex);
    threadCountCreated += 1;
    threadCountCreated;
    threadCountAliveNow += 1;
    threadCountAliveNow;
    if threadCountAliveNow > threadCountAliveMax {
        threadCountAliveMax = threadCountAliveNow;
    }
    pthread_mutex_unlock(&mut threadCountMutex);
    if verbose != 0 {
        printf(
            b"    t%d %llu : Hello from thread #%d\n\0" as *const u8
                as *const libc::c_char,
            seqNrPub,
            pthread_self() as libc::c_ulonglong,
            seqNrPub,
        );
    }
    if verbose != 0 {
        printf(
            b"    t%d %llu : Threads before snoopy_init():    %d\n\0" as *const u8
                as *const libc::c_char,
            seqNrPub,
            pthread_self() as libc::c_ulonglong,
            snoopy_tsrm_get_threadCount(),
        );
    }
    snoopy_entrypoint_test_cli_threads_init();
    if verbose != 0 {
        printf(
            b"    t%d %llu : Threads after  snoopy_init():    %d\n\0" as *const u8
                as *const libc::c_char,
            seqNrPub,
            pthread_self() as libc::c_ulonglong,
            snoopy_tsrm_get_threadCount(),
        );
    }
    dsCount = snoopy_datasourceregistry_getCount();
    dsId = snoopyTestCli_action_stress_threads_randomNumberInclusive(
        0 as libc::c_int,
        dsCount - 1 as libc::c_int,
    );
    dsName = snoopy_datasourceregistry_getName(dsId);
    retVal = snoopy_datasourceregistry_callById(dsId, dsResult.as_mut_ptr(), dsArg);
    if 0 as libc::c_int > retVal {
        printf(
            b"    t%d %llu : Datasource %s returned negative result: %d\n\0" as *const u8
                as *const libc::c_char,
            seqNrPub,
            pthread_self() as libc::c_ulonglong,
            dsName,
            retVal,
        );
    } else {
        printf(
            b"    t%d %llu : DS result: %30s = %s\n\0" as *const u8
                as *const libc::c_char,
            seqNrPub,
            pthread_self() as libc::c_ulonglong,
            dsName,
            dsResult.as_mut_ptr(),
        );
    }
    if verbose != 0 {
        printf(
            b"    t%d %llu : Threads before snoopy_cleanup(): %d\n\0" as *const u8
                as *const libc::c_char,
            seqNrPub,
            pthread_self() as libc::c_ulonglong,
            snoopy_tsrm_get_threadCount(),
        );
    }
    snoopy_entrypoint_test_cli_exit();
    if verbose != 0 {
        printf(
            b"    t%d %llu : Threads after  snoopy_cleanup(): %d\n\0" as *const u8
                as *const libc::c_char,
            seqNrPub,
            pthread_self() as libc::c_ulonglong,
            snoopy_tsrm_get_threadCount(),
        );
    }
    if verbose != 0 {
        printf(
            b"    t%d %llu : Thread exiting: #%d\n\0" as *const u8
                as *const libc::c_char,
            seqNrPub,
            pthread_self() as libc::c_ulonglong,
            seqNrPub,
        );
    }
    pthread_mutex_lock(&mut threadCountMutex);
    threadCountAliveNow -= 1;
    threadCountAliveNow;
    pthread_mutex_unlock(&mut threadCountMutex);
    free(tArgs as *mut libc::c_void);
    return 0 as *mut libc::c_void;
}
pub unsafe extern "C" fn snoopyTestCli_action_stress_threads_randomNumberInclusive(
    mut nMin: libc::c_int,
    mut nMax: libc::c_int,
) -> libc::c_int {
    let mut randomNrRaw: libc::c_int = 0 as libc::c_int;
    let mut randomNr: libc::c_int = 0;
    let mut bytesRead: ssize_t = 0 as libc::c_int as ssize_t;
    let mut buffer: [libc::c_uchar; 4] = [0; 4];
    let mut fd: libc::c_int = open(
        b"/dev/urandom\0" as *const u8 as *const libc::c_char,
        0 as libc::c_int,
    );
    if -(1 as libc::c_int) == fd {
        printf(
            b"ERROR: Unable to open /dev/urandom.\n\0" as *const u8
                as *const libc::c_char,
        );
        return -(1 as libc::c_int);
    }
    bytesRead = read(
        fd,
        buffer.as_mut_ptr() as *mut libc::c_void,
        ::std::mem::size_of::<libc::c_int>() as libc::c_ulong,
    );
    close(fd);
    if bytesRead as libc::c_ulong
        != ::std::mem::size_of::<libc::c_int>() as libc::c_ulong
    {
        printf(
            b"ERROR: Unable to read %lu bytes from /dev/urandom, only got %li bytes.\n\0"
                as *const u8 as *const libc::c_char,
            ::std::mem::size_of::<libc::c_int>() as libc::c_ulong,
            bytesRead,
        );
        return -(1 as libc::c_int);
    }
    let mut i: libc::c_uint = 0 as libc::c_int as libc::c_uint;
    while (i as libc::c_ulong) < ::std::mem::size_of::<libc::c_int>() as libc::c_ulong {
        randomNrRaw
            += (buffer[i as usize] as libc::c_int)
                << i.wrapping_mul(8 as libc::c_int as libc::c_uint);
        i = i.wrapping_add(1);
        i;
    }
    if randomNrRaw < 0 as libc::c_int {
        randomNrRaw = -randomNrRaw;
    }
    randomNr = randomNrRaw % (nMax - nMin + 1 as libc::c_int) + nMin;
    return randomNr;
}
