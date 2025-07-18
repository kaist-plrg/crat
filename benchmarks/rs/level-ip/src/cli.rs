use ::libc;
extern "C" {
    pub type _IO_wide_data;
    pub type _IO_codecvt;
    pub type _IO_marker;
    fn exit(_: libc::c_int) -> !;
    static mut stderr: *mut FILE;
    fn fprintf(_: *mut FILE, _: *const libc::c_char, _: ...) -> libc::c_int;
    static mut optind: libc::c_int;
    fn getopt(
        ___argc: libc::c_int,
        ___argv: *const *mut libc::c_char,
        __shortopts: *const libc::c_char,
    ) -> libc::c_int;
}
pub type size_t = libc::c_ulong;
pub type __off_t = libc::c_long;
pub type __off64_t = libc::c_long;
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
pub static mut debug: libc::c_int = 0 as libc::c_int;
unsafe extern "C" fn usage(mut app: *mut libc::c_char) {
    fprintf(stderr, b"Usage: %s\n\0" as *const u8 as *const libc::c_char, app);
    fprintf(stderr, b"\n\0" as *const u8 as *const libc::c_char);
    fprintf(
        stderr,
        b"Linux TCP/IP stack implemented with TUN/TAP devices.\n\0" as *const u8
            as *const libc::c_char,
    );
    fprintf(
        stderr,
        b"Requires the CAP_NET_ADMIN capability. See capabilities(7).\n\0" as *const u8
            as *const libc::c_char,
    );
    fprintf(
        stderr,
        b"See https://www.kernel.org/doc/Documentation/networking/tuntap.txt\n\0"
            as *const u8 as *const libc::c_char,
    );
    fprintf(stderr, b"\n\0" as *const u8 as *const libc::c_char);
    fprintf(stderr, b"Options:\n\0" as *const u8 as *const libc::c_char);
    fprintf(
        stderr,
        b"  -d Debug logging and tracing\n\0" as *const u8 as *const libc::c_char,
    );
    fprintf(stderr, b"  -h Print usage\n\0" as *const u8 as *const libc::c_char);
    fprintf(stderr, b"\n\0" as *const u8 as *const libc::c_char);
    exit(1 as libc::c_int);
}
unsafe extern "C" fn parse_opts(
    mut argc: *mut libc::c_int,
    mut argv: *mut *mut *mut libc::c_char,
) -> libc::c_int {
    let mut opt: libc::c_int = 0;
    loop {
        opt = getopt(*argc, *argv, b"hd\0" as *const u8 as *const libc::c_char);
        if !(opt != -(1 as libc::c_int)) {
            break;
        }
        match opt {
            100 => {
                debug = 1 as libc::c_int;
            }
            104 | _ => {
                usage(**argv.offset(0 as libc::c_int as isize));
            }
        }
    }
    *argc -= optind;
    *argv = (*argv).offset(optind as isize);
    return optind;
}
pub unsafe extern "C" fn parse_cli(
    mut argc: libc::c_int,
    mut argv: *mut *mut libc::c_char,
) {
    parse_opts(&mut argc, &mut argv);
}
