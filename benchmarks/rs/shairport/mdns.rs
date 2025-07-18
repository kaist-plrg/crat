use ::libc;
extern "C" {
    fn strcpy(_: *mut libc::c_char, _: *const libc::c_char) -> *mut libc::c_char;
    fn strcmp(_: *const libc::c_char, _: *const libc::c_char) -> libc::c_int;
    fn strlen(_: *const libc::c_char) -> libc::c_ulong;
    fn printf(_: *const libc::c_char, _: ...) -> libc::c_int;
    fn sprintf(_: *mut libc::c_char, _: *const libc::c_char, _: ...) -> libc::c_int;
    fn malloc(_: libc::c_ulong) -> *mut libc::c_void;
    fn warn(format: *mut libc::c_char, _: ...);
    fn die(format: *mut libc::c_char, _: ...);
    static mut config: shairport_cfg;
    static mut mdns_external_avahi: mdns_backend;
    static mut mdns_external_dns_sd: mdns_backend;
    static mut mdns_tinysvcmdns: mdns_backend;
}
pub type __uint8_t = libc::c_uchar;
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
static mut mdns_backends: [*mut mdns_backend; 4] = unsafe {
    [
        &mdns_external_avahi as *const mdns_backend as *mut mdns_backend,
        &mdns_external_dns_sd as *const mdns_backend as *mut mdns_backend,
        &mdns_tinysvcmdns as *const mdns_backend as *mut mdns_backend,
        0 as *const mdns_backend as *mut mdns_backend,
    ]
};
pub unsafe extern "C" fn mdns_register() {
    let mut mdns_apname: *mut libc::c_char = malloc(
        (strlen(config.apname)).wrapping_add(14 as libc::c_int as libc::c_ulong),
    ) as *mut libc::c_char;
    let mut p: *mut libc::c_char = mdns_apname;
    let mut i: libc::c_int = 0;
    i = 0 as libc::c_int;
    while i < 6 as libc::c_int {
        sprintf(
            p,
            b"%02X\0" as *const u8 as *const libc::c_char,
            config.hw_addr[i as usize] as libc::c_int,
        );
        p = p.offset(2 as libc::c_int as isize);
        i += 1;
        i;
    }
    let fresh0 = p;
    p = p.offset(1);
    *fresh0 = '@' as i32 as libc::c_char;
    strcpy(p, config.apname);
    let mut b: *mut *mut mdns_backend = 0 as *mut *mut mdns_backend;
    if !(config.mdns_name).is_null() {
        b = mdns_backends.as_mut_ptr();
        while !(*b).is_null() {
            if strcmp((**b).name, config.mdns_name) != 0 as libc::c_int {
                b = b.offset(1);
                b;
            } else {
                let mut error: libc::c_int = ((**b).mdns_register)
                    .unwrap()(mdns_apname, config.port);
                if error >= 0 as libc::c_int {
                    config.mdns = *b;
                }
                break;
            }
        }
        if (*b).is_null() {
            warn(
                b"%s mDNS backend not found\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
            );
        }
    } else {
        b = mdns_backends.as_mut_ptr();
        while !(*b).is_null() {
            let mut error_0: libc::c_int = ((**b).mdns_register)
                .unwrap()(mdns_apname, config.port);
            if error_0 >= 0 as libc::c_int {
                config.mdns = *b;
                break;
            } else {
                b = b.offset(1);
                b;
            }
        }
    }
    if (config.mdns).is_null() {
        die(
            b"Could not establish mDNS advertisement!\0" as *const u8
                as *const libc::c_char as *mut libc::c_char,
        );
    }
}
pub unsafe extern "C" fn mdns_unregister() {
    if !(config.mdns).is_null() {
        ((*config.mdns).mdns_unregister).unwrap()();
    }
}
pub unsafe extern "C" fn mdns_ls_backends() {
    let mut b: *mut *mut mdns_backend = 0 as *mut *mut mdns_backend;
    printf(b"Available mDNS backends: \n\0" as *const u8 as *const libc::c_char);
    b = mdns_backends.as_mut_ptr();
    while !(*b).is_null() {
        printf(b"    %s\n\0" as *const u8 as *const libc::c_char, (**b).name);
        b = b.offset(1);
        b;
    }
}
