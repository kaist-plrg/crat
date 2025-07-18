use ::libc;
extern "C" {
    fn fatalError(message: *const libc::c_char);
    fn fatalErrorValue(message: *const libc::c_char, value: *const libc::c_char);
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
    fn printf(_: *const libc::c_char, _: ...) -> libc::c_int;
    fn strtol(
        _: *const libc::c_char,
        _: *mut *mut libc::c_char,
        _: libc::c_int,
    ) -> libc::c_long;
    fn malloc(_: libc::c_ulong) -> *mut libc::c_void;
    fn free(__ptr: *mut libc::c_void);
    fn execv(
        __path: *const libc::c_char,
        __argv: *const *mut libc::c_char,
    ) -> libc::c_int;
    fn fork() -> __pid_t;
    fn waitpid(
        __pid: __pid_t,
        __stat_loc: *mut libc::c_int,
        __options: libc::c_int,
    ) -> __pid_t;
}
pub type __pid_t = libc::c_int;
pub type pid_t = __pid_t;
pub type pthread_t = libc::c_ulong;
#[derive(Copy, Clone)]
#[repr(C)]
pub union pthread_attr_t {
    pub __size: [libc::c_char; 56],
    pub __align: libc::c_long,
}
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
pub static mut snoopyTestCli_action_stress_threadsexec_runCmdAndArgv: *mut *mut libc::c_char = 0
    as *const *mut libc::c_char as *mut *mut libc::c_char;
pub static mut snoopyTestCli_action_stress_threadsexec_tRepo: [pthread_t; 10000] = [0; 10000];
pub unsafe extern "C" fn snoopyTestCli_action_stress_threadsexec_showHelp() {
    let mut helpContent: *mut libc::c_char = b"Snoopy TEST SUITE CLI utility :: Action `stress` :: Subsystem `threadsexec`\n\nUsage:\n    snoopy-test stress threadsexec THREAD_COUNT CMD [CMD_ARGS]\n\nDescription:\n    Stresses Snoopy's threading implementation by creating and destroying THREAD_COUNT\n    threads as fast as possible and executing CMD from those threads.\n\nArguments:\n    THREAD_COUNT       Number of threads to create and destroy\n    CMD                External command to execute from each newly created thread\n    [CMD_ARGS]         Optional argument(s) for the external command\n\nOutput:\n    Various threading-related messages are shown, followed by a word \"SUCCESS!\".\n\n\0"
        as *const u8 as *const libc::c_char as *mut libc::c_char;
    printf(b"%s\0" as *const u8 as *const libc::c_char, helpContent);
}
pub unsafe extern "C" fn snoopyTestCli_action_stress_threadsexec(
    mut argc: libc::c_int,
    mut argv: *mut *mut libc::c_char,
) -> libc::c_int {
    let mut threadsToCreate: libc::c_int = 0;
    let mut i: libc::c_int = 0;
    let mut retVal: libc::c_int = 0 as libc::c_int;
    if argc < 1 as libc::c_int {
        snoopyTestCli_action_stress_threadsexec_showHelp();
        fatalError(
            b"Missing argument: number of threads to run\0" as *const u8
                as *const libc::c_char,
        );
    }
    threadsToCreate = atoi(*argv.offset(0 as libc::c_int as isize));
    if threadsToCreate < 1 as libc::c_int || threadsToCreate > 10000 as libc::c_int {
        snoopyTestCli_action_stress_threadsexec_showHelp();
        fatalErrorValue(
            b"Invalid number of threads to create (min 1, max THREAD_COUNT_MAX)\0"
                as *const u8 as *const libc::c_char,
            *argv.offset(0 as libc::c_int as isize),
        );
    }
    if argc < 2 as libc::c_int {
        snoopyTestCli_action_stress_threadsexec_showHelp();
        fatalError(
            b"Missing argument: external command to run\0" as *const u8
                as *const libc::c_char,
        );
    }
    snoopyTestCli_action_stress_threadsexec_runCmdAndArgv = &mut *argv
        .offset(1 as libc::c_int as isize) as *mut *mut libc::c_char;
    printf(b"M: Starting threads:\n\0" as *const u8 as *const libc::c_char);
    i = 0 as libc::c_int;
    while i < threadsToCreate {
        let mut tArgs: *mut tData_t = malloc(
            ::std::mem::size_of::<tData_t>() as libc::c_ulong,
        ) as *mut tData_t;
        (*tArgs).seqNr = i;
        printf(
            b" M: Starting thread #%d:\n\0" as *const u8 as *const libc::c_char,
            i + 1 as libc::c_int,
        );
        retVal = pthread_create(
            &mut *snoopyTestCli_action_stress_threadsexec_tRepo
                .as_mut_ptr()
                .offset(i as isize),
            0 as *const pthread_attr_t,
            Some(
                snoopyTestCli_action_stress_threadsexec_threadMain
                    as unsafe extern "C" fn(*mut libc::c_void) -> *mut libc::c_void,
            ),
            tArgs as *mut libc::c_void,
        );
        i += 1;
        i;
    }
    printf(b"M: All threads started\n\0" as *const u8 as *const libc::c_char);
    printf(
        b"M: Waiting for all threads to finish:\n\0" as *const u8 as *const libc::c_char,
    );
    i = 0 as libc::c_int;
    while i < threadsToCreate {
        pthread_join(
            snoopyTestCli_action_stress_threadsexec_tRepo[i as usize],
            0 as *mut *mut libc::c_void,
        );
        printf(
            b" M: Thread #%d joined.\n\0" as *const u8 as *const libc::c_char,
            i + 1 as libc::c_int,
        );
        i += 1;
        i;
    }
    printf(b"M: All threads have finished.\n\0" as *const u8 as *const libc::c_char);
    printf(b"SUCCESS!\n\0" as *const u8 as *const libc::c_char);
    return retVal;
}
pub unsafe extern "C" fn snoopyTestCli_action_stress_threadsexec_threadMain(
    mut args: *mut libc::c_void,
) -> *mut libc::c_void {
    let mut tArgs: *mut tData_t = args as *mut tData_t;
    let mut cmd: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut argv: *mut *mut libc::c_char = 0 as *mut *mut libc::c_char;
    printf(
        b"  t%d : Hello from thread #%d.\n\0" as *const u8 as *const libc::c_char,
        (*tArgs).seqNr + 1 as libc::c_int,
        (*tArgs).seqNr + 1 as libc::c_int,
    );
    let mut pid: pid_t = fork();
    if pid > 0 as libc::c_int {
        printf(
            b"  t%dp: Hello from parent proc\n\0" as *const u8 as *const libc::c_char,
            (*tArgs).seqNr + 1 as libc::c_int,
        );
        let mut status: *mut libc::c_int = 0 as *mut libc::c_int;
        waitpid(pid, status, 0 as libc::c_int);
        printf(
            b"  t%dp: Child proc has finished\n\0" as *const u8 as *const libc::c_char,
            (*tArgs).seqNr + 1 as libc::c_int,
        );
    } else if pid == 0 as libc::c_int {
        printf(
            b"  t%dc: Hello from child proc\n\0" as *const u8 as *const libc::c_char,
            (*tArgs).seqNr + 1 as libc::c_int,
        );
        cmd = *snoopyTestCli_action_stress_threadsexec_runCmdAndArgv
            .offset(0 as libc::c_int as isize);
        argv = &mut *snoopyTestCli_action_stress_threadsexec_runCmdAndArgv
            .offset(0 as libc::c_int as isize) as *mut *mut libc::c_char;
        printf(
            b"  t%dc: running cmd %s %s\n\0" as *const u8 as *const libc::c_char,
            (*tArgs).seqNr + 1 as libc::c_int,
            cmd,
            *argv.offset(0 as libc::c_int as isize),
        );
        execv(cmd, argv as *const *mut libc::c_char);
    } else {
        printf(
            b"  t%d : Fork failed!\n\0" as *const u8 as *const libc::c_char,
            (*tArgs).seqNr + 1 as libc::c_int,
        );
    }
    free(tArgs as *mut libc::c_void);
    return 0 as *mut libc::c_void;
}
