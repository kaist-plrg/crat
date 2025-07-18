use ::libc;
extern "C" {
    pub type _IO_wide_data;
    pub type _IO_codecvt;
    pub type _IO_marker;
    pub type MDB_env;
    static mut stderr: *mut FILE;
    fn fprintf(_: *mut FILE, _: *const libc::c_char, _: ...) -> libc::c_int;
    fn printf(_: *const libc::c_char, _: ...) -> libc::c_int;
    fn exit(_: libc::c_int) -> !;
    fn signal(__sig: libc::c_int, __handler: __sighandler_t) -> __sighandler_t;
    fn mdb_strerror(err: libc::c_int) -> *mut libc::c_char;
    fn mdb_env_create(env: *mut *mut MDB_env) -> libc::c_int;
    fn mdb_env_open(
        env: *mut MDB_env,
        path: *const libc::c_char,
        flags: libc::c_uint,
        mode: mdb_mode_t,
    ) -> libc::c_int;
    fn mdb_env_copy2(
        env: *mut MDB_env,
        path: *const libc::c_char,
        flags: libc::c_uint,
    ) -> libc::c_int;
    fn mdb_env_copyfd2(
        env: *mut MDB_env,
        fd: mdb_filehandle_t,
        flags: libc::c_uint,
    ) -> libc::c_int;
    fn mdb_env_close(env: *mut MDB_env);
}
pub type size_t = libc::c_ulong;
pub type __mode_t = libc::c_uint;
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
pub type mode_t = __mode_t;
pub type __sighandler_t = Option::<unsafe extern "C" fn(libc::c_int) -> ()>;
pub type mdb_mode_t = mode_t;
pub type mdb_filehandle_t = libc::c_int;
unsafe extern "C" fn sighandle(mut sig: libc::c_int) {}
unsafe fn main_0(
    mut argc: libc::c_int,
    mut argv: *mut *mut libc::c_char,
) -> libc::c_int {
    let mut rc: libc::c_int = 0;
    let mut env: *mut MDB_env = 0 as *mut MDB_env;
    let mut progname: *const libc::c_char = *argv.offset(0 as libc::c_int as isize);
    let mut act: *const libc::c_char = 0 as *const libc::c_char;
    let mut flags: libc::c_uint = 0x20000 as libc::c_int as libc::c_uint;
    let mut cpflags: libc::c_uint = 0 as libc::c_int as libc::c_uint;
    while argc > 1 as libc::c_int
        && *(*argv.offset(1 as libc::c_int as isize)).offset(0 as libc::c_int as isize)
            as libc::c_int == '-' as i32
    {
        if *(*argv.offset(1 as libc::c_int as isize)).offset(1 as libc::c_int as isize)
            as libc::c_int == 'n' as i32
            && *(*argv.offset(1 as libc::c_int as isize))
                .offset(2 as libc::c_int as isize) as libc::c_int == '\0' as i32
        {
            flags |= 0x4000 as libc::c_int as libc::c_uint;
        } else if *(*argv.offset(1 as libc::c_int as isize))
            .offset(1 as libc::c_int as isize) as libc::c_int == 'v' as i32
            && *(*argv.offset(1 as libc::c_int as isize))
                .offset(2 as libc::c_int as isize) as libc::c_int == '\0' as i32
        {
            flags |= 0x2000000 as libc::c_int as libc::c_uint;
        } else if *(*argv.offset(1 as libc::c_int as isize))
            .offset(1 as libc::c_int as isize) as libc::c_int == 'c' as i32
            && *(*argv.offset(1 as libc::c_int as isize))
                .offset(2 as libc::c_int as isize) as libc::c_int == '\0' as i32
        {
            cpflags |= 0x1 as libc::c_int as libc::c_uint;
        } else if *(*argv.offset(1 as libc::c_int as isize))
            .offset(1 as libc::c_int as isize) as libc::c_int == 'V' as i32
            && *(*argv.offset(1 as libc::c_int as isize))
                .offset(2 as libc::c_int as isize) as libc::c_int == '\0' as i32
        {
            printf(
                b"%s\n\0" as *const u8 as *const libc::c_char,
                b"LMDB 0.9.70: (December 19, 2015)\0" as *const u8 as *const libc::c_char,
            );
            exit(0 as libc::c_int);
        } else {
            argc = 0 as libc::c_int;
        }
        argc -= 1;
        argc;
        argv = argv.offset(1);
        argv;
    }
    if argc < 2 as libc::c_int || argc > 3 as libc::c_int {
        fprintf(
            stderr,
            b"usage: %s [-V] [-c] [-n] [-v] srcpath [dstpath]\n\0" as *const u8
                as *const libc::c_char,
            progname,
        );
        exit(1 as libc::c_int);
    }
    signal(
        13 as libc::c_int,
        Some(sighandle as unsafe extern "C" fn(libc::c_int) -> ()),
    );
    signal(1 as libc::c_int, Some(sighandle as unsafe extern "C" fn(libc::c_int) -> ()));
    signal(2 as libc::c_int, Some(sighandle as unsafe extern "C" fn(libc::c_int) -> ()));
    signal(
        15 as libc::c_int,
        Some(sighandle as unsafe extern "C" fn(libc::c_int) -> ()),
    );
    act = b"opening environment\0" as *const u8 as *const libc::c_char;
    rc = mdb_env_create(&mut env);
    if rc == 0 as libc::c_int {
        rc = mdb_env_open(
            env,
            *argv.offset(1 as libc::c_int as isize),
            flags,
            0o600 as libc::c_int as mdb_mode_t,
        );
    }
    if rc == 0 as libc::c_int {
        act = b"copying\0" as *const u8 as *const libc::c_char;
        if argc == 2 as libc::c_int {
            rc = mdb_env_copyfd2(env, 1 as libc::c_int, cpflags);
        } else {
            rc = mdb_env_copy2(env, *argv.offset(2 as libc::c_int as isize), cpflags);
        }
    }
    if rc != 0 {
        fprintf(
            stderr,
            b"%s: %s failed, error %d (%s)\n\0" as *const u8 as *const libc::c_char,
            progname,
            act,
            rc,
            mdb_strerror(rc),
        );
    }
    mdb_env_close(env);
    return if rc != 0 { 1 as libc::c_int } else { 0 as libc::c_int };
}
pub fn main() {
    let mut args: Vec::<*mut libc::c_char> = Vec::new();
    for arg in ::std::env::args_os() {
        args.push(
            (::std::ffi::CString::new(
                ::std::os::unix::ffi::OsStrExt::as_bytes(arg.as_os_str()),
            ))
                .unwrap()
                .into_raw(),
        );
    }
    args.push(::std::ptr::null_mut());
    unsafe {
        ::std::process::exit(
            main_0(
                (args.len() - 1) as libc::c_int,
                args.as_mut_ptr() as *mut *mut libc::c_char,
            ) as i32,
        )
    }
}
