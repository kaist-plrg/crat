use ::libc;
extern "C" {
    pub type _IO_wide_data;
    pub type _IO_codecvt;
    pub type _IO_marker;
    pub type bc_struct;
    static mut stdout: *mut FILE;
    fn getc(__stream: *mut FILE) -> libc::c_int;
    fn fwrite(
        _: *const libc::c_void,
        _: libc::c_ulong,
        _: libc::c_ulong,
        _: *mut FILE,
    ) -> libc::c_ulong;
    fn realloc(_: *mut libc::c_void, _: libc::c_ulong) -> *mut libc::c_void;
    fn free(__ptr: *mut libc::c_void);
    fn memcpy(
        _: *mut libc::c_void,
        _: *const libc::c_void,
        _: libc::c_ulong,
    ) -> *mut libc::c_void;
    fn dc_malloc(_: size_t) -> *mut libc::c_void;
    fn dc_memfail();
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
pub type ptrdiff_t = libc::c_long;
pub type dc_discard = libc::c_uint;
pub const DC_KEEP: dc_discard = 1;
pub const DC_TOSS: dc_discard = 0;
pub type dc_value_type = libc::c_uint;
pub const DC_STRING: dc_value_type = 2;
pub const DC_NUMBER: dc_value_type = 1;
pub const DC_UNINITIALIZED: dc_value_type = 0;
pub type dc_num = *mut bc_struct;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct dc_string {
    pub s_ptr: *mut libc::c_char,
    pub s_len: size_t,
    pub s_refs: libc::c_int,
}
pub type dc_str = *mut dc_string;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct dc_data {
    pub dc_type: dc_value_type,
    pub v: C2RustUnnamed,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed {
    pub number: dc_num,
    pub string: dc_str,
}
pub unsafe extern "C" fn dc_dup_str(mut value: dc_str) -> dc_data {
    let mut result: dc_data = dc_data {
        dc_type: DC_UNINITIALIZED,
        v: C2RustUnnamed {
            number: 0 as *mut bc_struct,
        },
    };
    (*value).s_refs += 1;
    (*value).s_refs;
    result.v.string = value;
    result.dc_type = DC_STRING;
    return result;
}
pub unsafe extern "C" fn dc_free_str(mut value: *mut dc_str) {
    let mut string: *mut dc_string = *value;
    (*string).s_refs -= 1;
    if (*string).s_refs < 1 as libc::c_int {
        free((*string).s_ptr as *mut libc::c_void);
        free(string as *mut libc::c_void);
    }
}
pub unsafe extern "C" fn dc_out_str(mut value: dc_str, mut discard_flag: dc_discard) {
    fwrite(
        (*value).s_ptr as *const libc::c_void,
        (*value).s_len,
        ::std::mem::size_of::<libc::c_char>() as libc::c_ulong,
        stdout,
    );
    if discard_flag as libc::c_uint == DC_TOSS as libc::c_int as libc::c_uint {
        dc_free_str(&mut value);
    }
}
pub unsafe extern "C" fn dc_makestring(
    mut s: *const libc::c_char,
    mut len: size_t,
) -> dc_data {
    let mut result: dc_data = dc_data {
        dc_type: DC_UNINITIALIZED,
        v: C2RustUnnamed {
            number: 0 as *mut bc_struct,
        },
    };
    let mut string: *mut dc_string = 0 as *mut dc_string;
    string = dc_malloc(::std::mem::size_of::<dc_string>() as libc::c_ulong)
        as *mut dc_string;
    (*string)
        .s_ptr = dc_malloc(len.wrapping_add(1 as libc::c_int as libc::c_ulong))
        as *mut libc::c_char;
    memcpy((*string).s_ptr as *mut libc::c_void, s as *const libc::c_void, len);
    *((*string).s_ptr).offset(len as isize) = '\0' as i32 as libc::c_char;
    (*string).s_len = len;
    (*string).s_refs = 1 as libc::c_int;
    result.v.string = string;
    result.dc_type = DC_STRING;
    return result;
}
pub unsafe extern "C" fn dc_readstring(
    mut fp: *mut FILE,
    mut ldelim: libc::c_int,
    mut rdelim: libc::c_int,
) -> dc_data {
    static mut line_buf: *mut libc::c_char = 0 as *const libc::c_char
        as *mut libc::c_char;
    static mut buflen: size_t = 0 as libc::c_int as size_t;
    let mut depth: libc::c_int = 1 as libc::c_int;
    let mut c: libc::c_int = 0;
    let mut p: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut end: *const libc::c_char = 0 as *const libc::c_char;
    if line_buf.is_null() {
        buflen = 2016 as libc::c_int as size_t;
        line_buf = dc_malloc(buflen) as *mut libc::c_char;
    }
    p = line_buf;
    end = line_buf.offset(buflen as isize);
    loop {
        c = getc(fp);
        if c == -(1 as libc::c_int) {
            break;
        }
        if c == rdelim
            && {
                depth -= 1;
                depth < 1 as libc::c_int
            }
        {
            break;
        }
        if c == ldelim {
            depth += 1;
            depth;
        }
        if p >= end as *mut libc::c_char {
            let mut offset: ptrdiff_t = p.offset_from(line_buf) as libc::c_long;
            buflen = (buflen as libc::c_ulong)
                .wrapping_add(2048 as libc::c_int as libc::c_ulong) as size_t as size_t;
            line_buf = realloc(line_buf as *mut libc::c_void, buflen)
                as *mut libc::c_char;
            if line_buf.is_null() {
                dc_memfail();
            }
            p = line_buf.offset(offset as isize);
            end = line_buf.offset(buflen as isize);
        }
        let fresh0 = p;
        p = p.offset(1);
        *fresh0 = c as libc::c_char;
    }
    return dc_makestring(line_buf, p.offset_from(line_buf) as libc::c_long as size_t);
}
pub unsafe extern "C" fn dc_str2charp(mut value: dc_str) -> *const libc::c_char {
    return (*value).s_ptr;
}
pub unsafe extern "C" fn dc_strlen(mut value: dc_str) -> size_t {
    return (*value).s_len;
}
pub unsafe extern "C" fn dc_string_init() {}
