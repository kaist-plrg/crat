use ::libc;
extern "C" {
    pub type _IO_wide_data;
    pub type _IO_codecvt;
    pub type _IO_marker;
    fn strchr(_: *const libc::c_char, _: libc::c_int) -> *mut libc::c_char;
    static mut stdout: *mut FILE;
    fn fopen(_: *const libc::c_char, _: *const libc::c_char) -> *mut FILE;
    fn fprintf(_: *mut FILE, _: *const libc::c_char, _: ...) -> libc::c_int;
    fn printf(_: *const libc::c_char, _: ...) -> libc::c_int;
    fn fgets(
        __s: *mut libc::c_char,
        __n: libc::c_int,
        __stream: *mut FILE,
    ) -> *mut libc::c_char;
    fn strncmp(
        _: *const libc::c_char,
        _: *const libc::c_char,
        _: libc::c_ulong,
    ) -> libc::c_int;
    fn malloc(_: libc::c_ulong) -> *mut libc::c_void;
    fn free(__ptr: *mut libc::c_void);
    fn exit(_: libc::c_int) -> !;
    fn strlen(_: *const libc::c_char) -> libc::c_ulong;
    fn osip_list_eol(li: *const osip_list_t, pos: libc::c_int) -> libc::c_int;
    fn osip_list_get(li: *const osip_list_t, pos: libc::c_int) -> *mut libc::c_void;
    static mut osip_malloc_func: Option::<osip_malloc_func_t>;
    static mut osip_free_func: Option::<osip_free_func_t>;
    fn osip_strncpy(
        dest: *mut libc::c_char,
        src: *const libc::c_char,
        length: size_t,
    ) -> *mut libc::c_char;
    fn osip_uri_init(url: *mut *mut osip_uri_t) -> libc::c_int;
    fn osip_uri_free(url: *mut osip_uri_t);
    fn osip_uri_parse(url: *mut osip_uri_t, buf: *const libc::c_char) -> libc::c_int;
    fn osip_uri_to_str(
        url: *const osip_uri_t,
        dest: *mut *mut libc::c_char,
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct __node {
    pub next: *mut __node_t,
    pub element: *mut libc::c_void,
}
pub type __node_t = __node;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct osip_list {
    pub nb_elt: libc::c_int,
    pub node: *mut __node_t,
}
pub type osip_list_t = osip_list;
pub type osip_malloc_func_t = unsafe extern "C" fn(size_t) -> *mut libc::c_void;
pub type osip_free_func_t = unsafe extern "C" fn(*mut libc::c_void) -> ();
#[derive(Copy, Clone)]
#[repr(C)]
pub struct osip_uri_param {
    pub gname: *mut libc::c_char,
    pub gvalue: *mut libc::c_char,
}
pub type osip_uri_param_t = osip_uri_param;
pub type osip_uri_header_t = osip_uri_param_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct osip_uri {
    pub scheme: *mut libc::c_char,
    pub username: *mut libc::c_char,
    pub password: *mut libc::c_char,
    pub host: *mut libc::c_char,
    pub port: *mut libc::c_char,
    pub url_params: osip_list_t,
    pub url_headers: osip_list_t,
    pub string: *mut libc::c_char,
}
pub type osip_uri_t = osip_uri;
unsafe fn main_0(
    mut argc: libc::c_int,
    mut argv: *mut *mut libc::c_char,
) -> libc::c_int {
    let mut urls_file: *mut FILE = 0 as *mut FILE;
    let mut url: *mut osip_uri_t = 0 as *mut osip_uri_t;
    let mut a_url: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut dest: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut res: *mut libc::c_char = 0 as *mut libc::c_char;
    urls_file = fopen(
        *argv.offset(1 as libc::c_int as isize),
        b"r\0" as *const u8 as *const libc::c_char,
    );
    if urls_file.is_null() {
        fprintf(
            stdout,
            b"Failed to open %s file.\nUsage: turls urls.txt\n\0" as *const u8
                as *const libc::c_char,
            *argv.offset(1 as libc::c_int as isize),
        );
        exit(0 as libc::c_int);
    }
    a_url = (if osip_malloc_func.is_some() {
        osip_malloc_func.unwrap()(200 as libc::c_int as size_t)
    } else {
        malloc(200 as libc::c_int as libc::c_ulong)
    }) as *mut libc::c_char;
    res = fgets(a_url, 200 as libc::c_int, urls_file);
    while !res.is_null() {
        let mut errcode: libc::c_int = 0;
        osip_strncpy(
            a_url.offset(strlen(a_url) as isize).offset(-(1 as libc::c_int as isize)),
            b"\0\0" as *const u8 as *const libc::c_char,
            1 as libc::c_int as size_t,
        );
        if 0 as libc::c_int
            != strncmp(
                a_url,
                b"#\0" as *const u8 as *const libc::c_char,
                1 as libc::c_int as libc::c_ulong,
            )
        {
            osip_uri_init(&mut url);
            printf(
                b"=================================================\n\0" as *const u8
                    as *const libc::c_char,
            );
            printf(b"URL TO PARSE: |%s|\n\0" as *const u8 as *const libc::c_char, a_url);
            errcode = osip_uri_parse(url, a_url);
            if errcode != -(1 as libc::c_int) {
                if osip_uri_to_str(url, &mut dest) != -(1 as libc::c_int) {
                    printf(
                        b"result:       |%s|\n\0" as *const u8 as *const libc::c_char,
                        dest,
                    );
                    osip_uri_test_accessor_api(url);
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
                    b"Bad url format: %s\n\0" as *const u8 as *const libc::c_char,
                    a_url,
                );
            }
            osip_uri_free(url);
            printf(
                b"=================================================\n\0" as *const u8
                    as *const libc::c_char,
            );
        }
        res = fgets(a_url, 200 as libc::c_int, urls_file);
    }
    if !a_url.is_null() {
        if osip_free_func.is_some() {
            osip_free_func.unwrap()(a_url as *mut libc::c_void);
        } else {
            free(a_url as *mut libc::c_void);
        }
    }
    return 0 as libc::c_int;
}
pub unsafe extern "C" fn osip_uri_test_accessor_api(
    mut url: *mut osip_uri_t,
) -> libc::c_int {
    if !((*url).scheme).is_null() {
        fprintf(stdout, b"%s:\0" as *const u8 as *const libc::c_char, (*url).scheme);
    }
    if !((*url).string).is_null() {
        fprintf(stdout, b"|%s\0" as *const u8 as *const libc::c_char, (*url).string);
        fprintf(stdout, b"\n\0" as *const u8 as *const libc::c_char);
        return 0 as libc::c_int;
    }
    if !((*url).username).is_null() {
        fprintf(stdout, b"%s|\0" as *const u8 as *const libc::c_char, (*url).username);
    }
    if !((*url).password).is_null() && !((*url).username).is_null() {
        fprintf(stdout, b":%s|\0" as *const u8 as *const libc::c_char, (*url).password);
    }
    if !((*url).username).is_null() {
        fprintf(stdout, b"@|\0" as *const u8 as *const libc::c_char);
    }
    if !((*url).host).is_null() {
        if !(strchr((*url).host, ':' as i32)).is_null() {
            fprintf(stdout, b"[%s]|\0" as *const u8 as *const libc::c_char, (*url).host);
        } else {
            fprintf(stdout, b"%s|\0" as *const u8 as *const libc::c_char, (*url).host);
        }
    }
    if !((*url).port).is_null() {
        fprintf(stdout, b":%s|\0" as *const u8 as *const libc::c_char, (*url).port);
    }
    fprintf(stdout, b"\nuri-params\n\0" as *const u8 as *const libc::c_char);
    let mut pos: libc::c_int = 0 as libc::c_int;
    let mut u_param: *mut osip_uri_param_t = 0 as *mut osip_uri_param_t;
    while osip_list_eol(&mut (*url).url_params, pos) == 0 {
        u_param = osip_list_get(&mut (*url).url_params, pos) as *mut osip_uri_param_t;
        if !((*u_param).gvalue).is_null() {
            fprintf(
                stdout,
                b";%s|=|%s|\0" as *const u8 as *const libc::c_char,
                (*u_param).gname,
                (*u_param).gvalue,
            );
        } else {
            fprintf(
                stdout,
                b";%s|\0" as *const u8 as *const libc::c_char,
                (*u_param).gname,
            );
        }
        pos += 1;
        pos;
    }
    fprintf(stdout, b"\nheaders\n\0" as *const u8 as *const libc::c_char);
    let mut pos_0: libc::c_int = 0 as libc::c_int;
    let mut u_header: *mut osip_uri_header_t = 0 as *mut osip_uri_header_t;
    while osip_list_eol(&mut (*url).url_headers, pos_0) == 0 {
        u_header = osip_list_get(&mut (*url).url_headers, pos_0)
            as *mut osip_uri_header_t;
        if pos_0 == 0 as libc::c_int {
            fprintf(
                stdout,
                b"?%s|=|%s|\0" as *const u8 as *const libc::c_char,
                (*u_header).gname,
                (*u_header).gvalue,
            );
        } else {
            fprintf(
                stdout,
                b"&%s|=|%s|\0" as *const u8 as *const libc::c_char,
                (*u_header).gname,
                (*u_header).gvalue,
            );
        }
        pos_0 += 1;
        pos_0;
    }
    fprintf(stdout, b"\n\0" as *const u8 as *const libc::c_char);
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
