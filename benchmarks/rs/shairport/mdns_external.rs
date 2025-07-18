use ::libc;
extern "C" {
    fn close(__fd: libc::c_int) -> libc::c_int;
    fn read(__fd: libc::c_int, __buf: *mut libc::c_void, __nbytes: size_t) -> ssize_t;
    fn write(__fd: libc::c_int, __buf: *const libc::c_void, __n: size_t) -> ssize_t;
    fn pipe(__pipedes: *mut libc::c_int) -> libc::c_int;
    fn execvp(
        __file: *const libc::c_char,
        __argv: *const *mut libc::c_char,
    ) -> libc::c_int;
    fn _exit(_: libc::c_int) -> !;
    fn fork() -> __pid_t;
    fn kill(__pid: __pid_t, __sig: libc::c_int) -> libc::c_int;
    fn fcntl(__fd: libc::c_int, __cmd: libc::c_int, _: ...) -> libc::c_int;
    fn __errno_location() -> *mut libc::c_int;
    fn sprintf(_: *mut libc::c_char, _: *const libc::c_char, _: ...) -> libc::c_int;
    static mut config: shairport_cfg;
    fn debug(level: libc::c_int, format: *mut libc::c_char, _: ...);
}
pub type __uint8_t = libc::c_uchar;
pub type __pid_t = libc::c_int;
pub type __ssize_t = libc::c_long;
pub type ssize_t = __ssize_t;
pub type size_t = libc::c_ulong;
pub type uint8_t = __uint8_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct audio_output {
    pub help: Option::<unsafe extern "C" fn() -> ()>,
    pub name: *mut libc::c_char,
    pub init: Option::<
        unsafe extern "C" fn(libc::c_int, *mut *mut libc::c_char) -> libc::c_int,
    >,
    pub deinit: Option::<unsafe extern "C" fn() -> ()>,
    pub start: Option::<unsafe extern "C" fn(libc::c_int) -> ()>,
    pub play: Option::<unsafe extern "C" fn(*mut libc::c_short, libc::c_int) -> ()>,
    pub stop: Option::<unsafe extern "C" fn() -> ()>,
    pub volume: Option::<unsafe extern "C" fn(libc::c_double) -> ()>,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct mdns_backend {
    pub name: *mut libc::c_char,
    pub mdns_register: Option::<
        unsafe extern "C" fn(*mut libc::c_char, libc::c_int) -> libc::c_int,
    >,
    pub mdns_unregister: Option::<unsafe extern "C" fn() -> ()>,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct shairport_cfg {
    pub password: *mut libc::c_char,
    pub apname: *mut libc::c_char,
    pub hw_addr: [uint8_t; 6],
    pub port: libc::c_int,
    pub output_name: *mut libc::c_char,
    pub output: *mut audio_output,
    pub mdns_name: *mut libc::c_char,
    pub mdns: *mut mdns_backend,
    pub buffer_start_fill: libc::c_int,
    pub daemonise: libc::c_int,
    pub cmd_start: *mut libc::c_char,
    pub cmd_stop: *mut libc::c_char,
    pub cmd_blocking: libc::c_int,
    pub meta_dir: *mut libc::c_char,
    pub pidfile: *mut libc::c_char,
    pub logfile: *mut libc::c_char,
    pub errfile: *mut libc::c_char,
}
pub static mut mdns_pid: libc::c_int = 0 as libc::c_int;
unsafe extern "C" fn fork_execvp(
    mut file: *const libc::c_char,
    mut argv: *const *mut libc::c_char,
) -> libc::c_int {
    let mut execpipe: [libc::c_int; 2] = [0; 2];
    let mut pid: libc::c_int = 0 as libc::c_int;
    if pipe(execpipe.as_mut_ptr()) < 0 as libc::c_int {
        return -(1 as libc::c_int);
    }
    if fcntl(
        execpipe[1 as libc::c_int as usize],
        2 as libc::c_int,
        fcntl(execpipe[1 as libc::c_int as usize], 1 as libc::c_int) | 1 as libc::c_int,
    ) < 0 as libc::c_int
    {
        close(execpipe[0 as libc::c_int as usize]);
        close(execpipe[1 as libc::c_int as usize]);
        return -(1 as libc::c_int);
    }
    pid = fork();
    if pid < 0 as libc::c_int {
        close(execpipe[0 as libc::c_int as usize]);
        close(execpipe[1 as libc::c_int as usize]);
        return -(1 as libc::c_int);
    } else if pid == 0 as libc::c_int {
        close(execpipe[0 as libc::c_int as usize]);
        execvp(file, argv);
        write(
            execpipe[1 as libc::c_int as usize],
            __errno_location() as *const libc::c_void,
            ::std::mem::size_of::<libc::c_int>() as libc::c_ulong,
        );
        _exit(-(1 as libc::c_int));
    } else {
        close(execpipe[1 as libc::c_int as usize]);
        let mut childErrno: libc::c_int = 0;
        if read(
            execpipe[0 as libc::c_int as usize],
            &mut childErrno as *mut libc::c_int as *mut libc::c_void,
            ::std::mem::size_of::<libc::c_int>() as libc::c_ulong,
        ) as libc::c_ulong == ::std::mem::size_of::<libc::c_int>() as libc::c_ulong
        {
            *__errno_location() = childErrno;
            return -(1 as libc::c_int);
        } else {
            return pid
        }
    };
}
unsafe extern "C" fn mdns_external_avahi_register(
    mut apname: *mut libc::c_char,
    mut port: libc::c_int,
) -> libc::c_int {
    let mut mdns_port: [libc::c_char; 6] = [0; 6];
    sprintf(
        mdns_port.as_mut_ptr(),
        b"%d\0" as *const u8 as *const libc::c_char,
        config.port,
    );
    let mut argv: [*mut libc::c_char; 18] = [
        0 as *mut libc::c_char,
        apname,
        b"_raop._tcp\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        mdns_port.as_mut_ptr(),
        b"tp=UDP\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        b"sm=false\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        b"ek=1\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        b"et=0,1\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        b"cn=0,1\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        b"ch=2\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        b"ss=16\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        b"sr=44100\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        b"vn=3\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        b"txtvers=1\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        b"da=true\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        b"md=0,1,2\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        (if !(config.password).is_null() {
            b"pw=true\0" as *const u8 as *const libc::c_char
        } else {
            b"pw=false\0" as *const u8 as *const libc::c_char
        }) as *mut libc::c_char,
        0 as *mut libc::c_char,
    ];
    argv[0 as libc::c_int
        as usize] = b"avahi-publish-service\0" as *const u8 as *const libc::c_char
        as *mut libc::c_char;
    let mut pid: libc::c_int = fork_execvp(
        argv[0 as libc::c_int as usize],
        argv.as_mut_ptr() as *const *mut libc::c_char,
    );
    if pid >= 0 as libc::c_int {
        mdns_pid = pid;
        return 0 as libc::c_int;
    } else {
        debug(
            1 as libc::c_int,
            b"Calling %s failed\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
            argv[0 as libc::c_int as usize],
        );
    }
    argv[0 as libc::c_int
        as usize] = b"mDNSPublish\0" as *const u8 as *const libc::c_char
        as *mut libc::c_char;
    pid = fork_execvp(
        argv[0 as libc::c_int as usize],
        argv.as_mut_ptr() as *const *mut libc::c_char,
    );
    if pid >= 0 as libc::c_int {
        mdns_pid = pid;
        return 0 as libc::c_int;
    } else {
        debug(
            1 as libc::c_int,
            b"Calling %s failed\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
            argv[0 as libc::c_int as usize],
        );
    }
    return -(1 as libc::c_int);
}
unsafe extern "C" fn mdns_external_dns_sd_register(
    mut apname: *mut libc::c_char,
    mut port: libc::c_int,
) -> libc::c_int {
    let mut mdns_port: [libc::c_char; 6] = [0; 6];
    sprintf(
        mdns_port.as_mut_ptr(),
        b"%d\0" as *const u8 as *const libc::c_char,
        config.port,
    );
    let mut argv: [*mut libc::c_char; 20] = [
        b"dns-sd\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        b"-R\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        apname,
        b"_raop._tcp\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        b".\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        mdns_port.as_mut_ptr(),
        b"tp=UDP\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        b"sm=false\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        b"ek=1\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        b"et=0,1\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        b"cn=0,1\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        b"ch=2\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        b"ss=16\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        b"sr=44100\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        b"vn=3\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        b"txtvers=1\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        b"da=true\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        b"md=0,1,2\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        (if !(config.password).is_null() {
            b"pw=true\0" as *const u8 as *const libc::c_char
        } else {
            b"pw=false\0" as *const u8 as *const libc::c_char
        }) as *mut libc::c_char,
        0 as *mut libc::c_char,
    ];
    let mut pid: libc::c_int = fork_execvp(
        argv[0 as libc::c_int as usize],
        argv.as_mut_ptr() as *const *mut libc::c_char,
    );
    if pid >= 0 as libc::c_int {
        mdns_pid = pid;
        return 0 as libc::c_int;
    } else {
        debug(
            1 as libc::c_int,
            b"Calling %s failed\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
            argv[0 as libc::c_int as usize],
        );
    }
    return -(1 as libc::c_int);
}
unsafe extern "C" fn kill_mdns_child() {
    if mdns_pid != 0 {
        kill(mdns_pid, 15 as libc::c_int);
    }
    mdns_pid = 0 as libc::c_int;
}
pub static mut mdns_external_avahi: mdns_backend = unsafe {
    {
        let mut init = mdns_backend {
            name: b"external-avahi\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
            mdns_register: Some(
                mdns_external_avahi_register
                    as unsafe extern "C" fn(
                        *mut libc::c_char,
                        libc::c_int,
                    ) -> libc::c_int,
            ),
            mdns_unregister: Some(kill_mdns_child as unsafe extern "C" fn() -> ()),
        };
        init
    }
};
pub static mut mdns_external_dns_sd: mdns_backend = unsafe {
    {
        let mut init = mdns_backend {
            name: b"external-dns-sd\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
            mdns_register: Some(
                mdns_external_dns_sd_register
                    as unsafe extern "C" fn(
                        *mut libc::c_char,
                        libc::c_int,
                    ) -> libc::c_int,
            ),
            mdns_unregister: Some(kill_mdns_child as unsafe extern "C" fn() -> ()),
        };
        init
    }
};
