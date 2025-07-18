use ::libc;
extern "C" {
    pub type _IO_wide_data;
    pub type _IO_codecvt;
    pub type _IO_marker;
    fn getc(__stream: *mut FILE) -> libc::c_int;
    fn rpl_free(ptr: *mut libc::c_void);
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
    fn strlen(_: *const libc::c_char) -> libc::c_ulong;
    fn x2nrealloc(p: *mut libc::c_void, pn: *mut size_t, s: size_t) -> *mut libc::c_void;
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
pub struct dynamic_string {
    pub ds_size: size_t,
    pub ds_idx: size_t,
    pub ds_string: *mut libc::c_char,
}
pub unsafe extern "C" fn ds_init(mut string: *mut dynamic_string) {
    memset(
        string as *mut libc::c_void,
        0 as libc::c_int,
        ::std::mem::size_of::<dynamic_string>() as libc::c_ulong,
    );
}
pub unsafe extern "C" fn ds_free(mut string: *mut dynamic_string) {
    rpl_free((*string).ds_string as *mut libc::c_void);
}
pub unsafe extern "C" fn ds_resize(mut string: *mut dynamic_string, mut len: size_t) {
    while len.wrapping_add((*string).ds_idx) >= (*string).ds_size {
        (*string)
            .ds_string = x2nrealloc(
            (*string).ds_string as *mut libc::c_void,
            &mut (*string).ds_size,
            1 as libc::c_int as size_t,
        ) as *mut libc::c_char;
    }
}
pub unsafe extern "C" fn ds_reset(mut s: *mut dynamic_string, mut len: size_t) {
    ds_resize(s, len);
    (*s).ds_idx = len;
}
pub unsafe extern "C" fn ds_fgetstr(
    mut f: *mut FILE,
    mut s: *mut dynamic_string,
    mut eos: libc::c_char,
) -> *mut libc::c_char {
    let mut next_ch: libc::c_int = 0;
    (*s).ds_idx = 0 as libc::c_int as size_t;
    loop {
        next_ch = getc(f);
        if !(next_ch != eos as libc::c_int && next_ch != -(1 as libc::c_int)) {
            break;
        }
        ds_resize(s, 0 as libc::c_int as size_t);
        let fresh0 = (*s).ds_idx;
        (*s).ds_idx = ((*s).ds_idx).wrapping_add(1);
        *((*s).ds_string).offset(fresh0 as isize) = next_ch as libc::c_char;
    }
    ds_resize(s, 0 as libc::c_int as size_t);
    *((*s).ds_string).offset((*s).ds_idx as isize) = '\0' as i32 as libc::c_char;
    if (*s).ds_idx == 0 as libc::c_int as libc::c_ulong && next_ch == -(1 as libc::c_int)
    {
        return 0 as *mut libc::c_char
    } else {
        return (*s).ds_string
    };
}
pub unsafe extern "C" fn ds_append(mut s: *mut dynamic_string, mut c: libc::c_int) {
    ds_resize(s, 0 as libc::c_int as size_t);
    *((*s).ds_string).offset((*s).ds_idx as isize) = c as libc::c_char;
    if c != 0 {
        (*s).ds_idx = ((*s).ds_idx).wrapping_add(1);
        (*s).ds_idx;
        ds_resize(s, 0 as libc::c_int as size_t);
        *((*s).ds_string)
            .offset((*s).ds_idx as isize) = 0 as libc::c_int as libc::c_char;
    }
}
pub unsafe extern "C" fn ds_concat(
    mut s: *mut dynamic_string,
    mut str: *const libc::c_char,
) {
    let mut len: size_t = strlen(str);
    ds_resize(s, len);
    memcpy(
        ((*s).ds_string).offset((*s).ds_idx as isize) as *mut libc::c_void,
        str as *const libc::c_void,
        len,
    );
    (*s).ds_idx = ((*s).ds_idx as libc::c_ulong).wrapping_add(len) as size_t as size_t;
    *((*s).ds_string).offset((*s).ds_idx as isize) = 0 as libc::c_int as libc::c_char;
}
pub unsafe extern "C" fn ds_fgets(
    mut f: *mut FILE,
    mut s: *mut dynamic_string,
) -> *mut libc::c_char {
    return ds_fgetstr(f, s, '\n' as i32 as libc::c_char);
}
pub unsafe extern "C" fn ds_fgetname(
    mut f: *mut FILE,
    mut s: *mut dynamic_string,
) -> *mut libc::c_char {
    return ds_fgetstr(f, s, '\0' as i32 as libc::c_char);
}
pub unsafe extern "C" fn ds_endswith(
    mut s: *mut dynamic_string,
    mut c: libc::c_int,
) -> libc::c_int {
    return ((*s).ds_idx > 0 as libc::c_int as libc::c_ulong
        && *((*s).ds_string)
            .offset(
                ((*s).ds_idx).wrapping_sub(1 as libc::c_int as libc::c_ulong) as isize,
            ) as libc::c_int == c) as libc::c_int;
}
