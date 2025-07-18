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
    fn osip_list_eol(li: *const osip_list_t, pos: libc::c_int) -> libc::c_int;
    fn osip_list_get(li: *const osip_list_t, pos: libc::c_int) -> *mut libc::c_void;
    fn osip_contact_init(header: *mut *mut osip_contact_t) -> libc::c_int;
    fn osip_contact_free(header: *mut osip_contact_t);
    fn osip_contact_parse(
        header: *mut osip_contact_t,
        hvalue: *const libc::c_char,
    ) -> libc::c_int;
    fn osip_contact_to_str(
        header: *const osip_contact_t,
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
pub type osip_generic_param_t = osip_uri_param_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct osip_from {
    pub displayname: *mut libc::c_char,
    pub url: *mut osip_uri_t,
    pub gen_params: osip_list_t,
}
pub type osip_from_t = osip_from;
pub type osip_contact_t = osip_from_t;
unsafe fn main_0(
    mut argc: libc::c_int,
    mut argv: *mut *mut libc::c_char,
) -> libc::c_int {
    let mut contacts_file: *mut FILE = 0 as *mut FILE;
    let mut contact: *mut osip_contact_t = 0 as *mut osip_contact_t;
    let mut a_contact: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut dest: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut res: *mut libc::c_char = 0 as *mut libc::c_char;
    contacts_file = fopen(
        *argv.offset(1 as libc::c_int as isize),
        b"r\0" as *const u8 as *const libc::c_char,
    );
    if contacts_file.is_null() {
        fprintf(
            stdout,
            b"Failed to open %s file.\nUsage: tcontact contacts.txt\n\0" as *const u8
                as *const libc::c_char,
            *argv.offset(1 as libc::c_int as isize),
        );
        exit(0 as libc::c_int);
    }
    a_contact = (if osip_malloc_func.is_some() {
        osip_malloc_func.unwrap()(500 as libc::c_int as size_t)
    } else {
        malloc(500 as libc::c_int as libc::c_ulong)
    }) as *mut libc::c_char;
    res = fgets(a_contact, 500 as libc::c_int, contacts_file);
    while !res.is_null() {
        let mut errcode: libc::c_int = 0;
        strncpy(
            a_contact
                .offset(strlen(a_contact) as isize)
                .offset(-(1 as libc::c_int as isize)),
            b"\0\0" as *const u8 as *const libc::c_char,
            1 as libc::c_int as libc::c_ulong,
        );
        if 0 as libc::c_int
            != strncmp(
                a_contact,
                b"#\0" as *const u8 as *const libc::c_char,
                1 as libc::c_int as libc::c_ulong,
            )
        {
            osip_contact_init(&mut contact);
            printf(
                b"=================================================\n\0" as *const u8
                    as *const libc::c_char,
            );
            printf(
                b"CONTACT TO PARSE: |%s|\n\0" as *const u8 as *const libc::c_char,
                a_contact,
            );
            errcode = osip_contact_parse(contact, a_contact);
            if errcode != -(1 as libc::c_int) {
                if osip_contact_to_str(contact, &mut dest) != -(1 as libc::c_int) {
                    printf(
                        b"result:           |%s|\n\0" as *const u8
                            as *const libc::c_char,
                        dest,
                    );
                    let mut pos: libc::c_int = 0 as libc::c_int;
                    let mut u_param: *mut osip_generic_param_t = 0
                        as *mut osip_generic_param_t;
                    while osip_list_eol(&mut (*contact).gen_params, pos) == 0 {
                        u_param = osip_list_get(&mut (*contact).gen_params, pos)
                            as *mut osip_generic_param_t;
                        if ((*u_param).gvalue).is_null() {
                            printf(
                                b"result:       ;%s\n\0" as *const u8
                                    as *const libc::c_char,
                                (*u_param).gname,
                            );
                        } else {
                            printf(
                                b"result:       ;%s=%s\n\0" as *const u8
                                    as *const libc::c_char,
                                (*u_param).gname,
                                (*u_param).gvalue,
                            );
                        }
                        pos += 1;
                        pos;
                    }
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
                    b"Bad contact format: %s\n\0" as *const u8 as *const libc::c_char,
                    a_contact,
                );
            }
            osip_contact_free(contact);
            printf(
                b"=================================================\n\0" as *const u8
                    as *const libc::c_char,
            );
        }
        res = fgets(a_contact, 500 as libc::c_int, contacts_file);
    }
    if !a_contact.is_null() {
        if osip_free_func.is_some() {
            osip_free_func.unwrap()(a_contact as *mut libc::c_void);
        } else {
            free(a_contact as *mut libc::c_void);
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
