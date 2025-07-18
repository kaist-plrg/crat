use ::libc;
extern "C" {
    pub type _IO_wide_data;
    pub type _IO_codecvt;
    pub type _IO_marker;
    fn malloc(_: libc::c_ulong) -> *mut libc::c_void;
    fn free(__ptr: *mut libc::c_void);
    static mut stdout: *mut FILE;
    fn fopen(_: *const libc::c_char, _: *const libc::c_char) -> *mut FILE;
    fn fprintf(_: *mut FILE, _: *const libc::c_char, _: ...) -> libc::c_int;
    fn printf(_: *const libc::c_char, _: ...) -> libc::c_int;
    fn fgets(
        __s: *mut libc::c_char,
        __n: libc::c_int,
        __stream: *mut FILE,
    ) -> *mut libc::c_char;
    fn strlen(_: *const libc::c_char) -> libc::c_ulong;
    fn strncmp(
        _: *const libc::c_char,
        _: *const libc::c_char,
        _: libc::c_ulong,
    ) -> libc::c_int;
    fn strncpy(
        _: *mut libc::c_char,
        _: *const libc::c_char,
        _: libc::c_ulong,
    ) -> *mut libc::c_char;
    fn exit(_: libc::c_int) -> !;
    static mut osip_malloc_func: Option::<osip_malloc_func_t>;
    static mut osip_free_func: Option::<osip_free_func_t>;
    fn osip_www_authenticate_init(
        header: *mut *mut osip_www_authenticate_t,
    ) -> libc::c_int;
    fn osip_www_authenticate_parse(
        header: *mut osip_www_authenticate_t,
        hvalue: *const libc::c_char,
    ) -> libc::c_int;
    fn osip_www_authenticate_to_str(
        header: *const osip_www_authenticate_t,
        dest: *mut *mut libc::c_char,
    ) -> libc::c_int;
    fn osip_www_authenticate_free(header: *mut osip_www_authenticate_t);
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
pub type osip_malloc_func_t = unsafe extern "C" fn(size_t) -> *mut libc::c_void;
pub type osip_free_func_t = unsafe extern "C" fn(*mut libc::c_void) -> ();
#[derive(Copy, Clone)]
#[repr(C)]
pub struct osip_www_authenticate {
    pub auth_type: *mut libc::c_char,
    pub realm: *mut libc::c_char,
    pub domain: *mut libc::c_char,
    pub nonce: *mut libc::c_char,
    pub opaque: *mut libc::c_char,
    pub stale: *mut libc::c_char,
    pub algorithm: *mut libc::c_char,
    pub qop_options: *mut libc::c_char,
    pub version: *mut libc::c_char,
    pub targetname: *mut libc::c_char,
    pub gssapi_data: *mut libc::c_char,
    pub auth_param: *mut libc::c_char,
}
pub type osip_www_authenticate_t = osip_www_authenticate;
unsafe fn main_0(
    mut argc: libc::c_int,
    mut argv: *mut *mut libc::c_char,
) -> libc::c_int {
    let mut wwwauthenticates_file: *mut FILE = 0 as *mut FILE;
    let mut wwwauthenticate: *mut osip_www_authenticate_t = 0
        as *mut osip_www_authenticate_t;
    let mut a_wwwauthenticate: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut dest: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut res: *mut libc::c_char = 0 as *mut libc::c_char;
    wwwauthenticates_file = fopen(
        *argv.offset(1 as libc::c_int as isize),
        b"r\0" as *const u8 as *const libc::c_char,
    );
    if wwwauthenticates_file.is_null() {
        fprintf(
            stdout,
            b"Failed to open %s file.\nUsage: twwwauthenticate wwwauthenticates.txt\n\0"
                as *const u8 as *const libc::c_char,
            *argv.offset(1 as libc::c_int as isize),
        );
        exit(0 as libc::c_int);
    }
    a_wwwauthenticate = (if osip_malloc_func.is_some() {
        osip_malloc_func.unwrap()(200 as libc::c_int as size_t)
    } else {
        malloc(200 as libc::c_int as libc::c_ulong)
    }) as *mut libc::c_char;
    res = fgets(a_wwwauthenticate, 200 as libc::c_int, wwwauthenticates_file);
    while !res.is_null() {
        let mut errcode: libc::c_int = 0;
        strncpy(
            a_wwwauthenticate
                .offset(strlen(a_wwwauthenticate) as isize)
                .offset(-(1 as libc::c_int as isize)),
            b"\0\0" as *const u8 as *const libc::c_char,
            1 as libc::c_int as libc::c_ulong,
        );
        if 0 as libc::c_int
            != strncmp(
                a_wwwauthenticate,
                b"#\0" as *const u8 as *const libc::c_char,
                1 as libc::c_int as libc::c_ulong,
            )
        {
            osip_www_authenticate_init(&mut wwwauthenticate);
            printf(
                b"=================================================\n\0" as *const u8
                    as *const libc::c_char,
            );
            printf(
                b"WWWAUTHENTICATE TO PARSE: |%s|\n\0" as *const u8
                    as *const libc::c_char,
                a_wwwauthenticate,
            );
            errcode = osip_www_authenticate_parse(wwwauthenticate, a_wwwauthenticate);
            if errcode != -(1 as libc::c_int) {
                if osip_www_authenticate_to_str(wwwauthenticate, &mut dest)
                    != -(1 as libc::c_int)
                {
                    printf(
                        b"result:                   |%s|\n\0" as *const u8
                            as *const libc::c_char,
                        dest,
                    );
                    if !dest.is_null() {
                        if osip_free_func.is_some() {
                            osip_free_func.unwrap()(dest as *mut libc::c_void);
                        } else {
                            free(dest as *mut libc::c_void);
                        }
                    }
                }
            } else {
                printf(
                    b"Bad wwwauthenticate format: %s\n\0" as *const u8
                        as *const libc::c_char,
                    a_wwwauthenticate,
                );
            }
            osip_www_authenticate_free(wwwauthenticate);
            printf(
                b"=================================================\n\0" as *const u8
                    as *const libc::c_char,
            );
        }
        res = fgets(a_wwwauthenticate, 200 as libc::c_int, wwwauthenticates_file);
    }
    if !a_wwwauthenticate.is_null() {
        if osip_free_func.is_some() {
            osip_free_func.unwrap()(a_wwwauthenticate as *mut libc::c_void);
        } else {
            free(a_wwwauthenticate as *mut libc::c_void);
        }
    }
    return 0 as libc::c_int;
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
