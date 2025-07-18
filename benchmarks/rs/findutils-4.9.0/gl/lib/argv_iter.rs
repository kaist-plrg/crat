use ::libc;
extern "C" {
    pub type _IO_wide_data;
    pub type _IO_codecvt;
    pub type _IO_marker;
    fn getdelim(
        __lineptr: *mut *mut libc::c_char,
        __n: *mut size_t,
        __delimiter: libc::c_int,
        __stream: *mut FILE,
    ) -> __ssize_t;
    fn feof(__stream: *mut FILE) -> libc::c_int;
    fn malloc(_: libc::c_ulong) -> *mut libc::c_void;
    fn rpl_free(ptr: *mut libc::c_void);
}
pub type size_t = libc::c_ulong;
pub type __off_t = libc::c_long;
pub type __off64_t = libc::c_long;
pub type __ssize_t = libc::c_long;
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
pub type ssize_t = __ssize_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct argv_iterator {
    pub fp: *mut FILE,
    pub item_idx: size_t,
    pub tok: *mut libc::c_char,
    pub buf_len: size_t,
    pub arg_list: *mut *mut libc::c_char,
    pub p: *mut *mut libc::c_char,
}
pub type argv_iter_err = libc::c_uint;
pub const AI_ERR_READ: argv_iter_err = 4;
pub const AI_ERR_MEM: argv_iter_err = 3;
pub const AI_ERR_EOF: argv_iter_err = 2;
pub const AI_ERR_OK: argv_iter_err = 1;
pub unsafe extern "C" fn argv_iter_init_argv(
    mut argv: *mut *mut libc::c_char,
) -> *mut argv_iterator {
    let mut ai: *mut argv_iterator = malloc(
        ::std::mem::size_of::<argv_iterator>() as libc::c_ulong,
    ) as *mut argv_iterator;
    if ai.is_null() {
        return 0 as *mut argv_iterator;
    }
    (*ai).fp = 0 as *mut FILE;
    (*ai).arg_list = argv;
    (*ai).p = argv;
    return ai;
}
pub unsafe extern "C" fn argv_iter_init_stream(mut fp: *mut FILE) -> *mut argv_iterator {
    let mut ai: *mut argv_iterator = malloc(
        ::std::mem::size_of::<argv_iterator>() as libc::c_ulong,
    ) as *mut argv_iterator;
    if ai.is_null() {
        return 0 as *mut argv_iterator;
    }
    (*ai).fp = fp;
    (*ai).tok = 0 as *mut libc::c_char;
    (*ai).buf_len = 0 as libc::c_int as size_t;
    (*ai).item_idx = 0 as libc::c_int as size_t;
    (*ai).arg_list = 0 as *mut *mut libc::c_char;
    return ai;
}
pub unsafe extern "C" fn argv_iter(
    mut ai: *mut argv_iterator,
    mut err: *mut argv_iter_err,
) -> *mut libc::c_char {
    if !((*ai).fp).is_null() {
        let mut len: ssize_t = getdelim(
            &mut (*ai).tok,
            &mut (*ai).buf_len,
            '\0' as i32,
            (*ai).fp,
        );
        if len < 0 as libc::c_int as libc::c_long {
            *err = (if feof((*ai).fp) != 0 {
                AI_ERR_EOF as libc::c_int
            } else {
                AI_ERR_READ as libc::c_int
            }) as argv_iter_err;
            return 0 as *mut libc::c_char;
        }
        *err = AI_ERR_OK;
        (*ai).item_idx = ((*ai).item_idx).wrapping_add(1);
        (*ai).item_idx;
        return (*ai).tok;
    } else if (*(*ai).p).is_null() {
        *err = AI_ERR_EOF;
        return 0 as *mut libc::c_char;
    } else {
        *err = AI_ERR_OK;
        let fresh0 = (*ai).p;
        (*ai).p = ((*ai).p).offset(1);
        return *fresh0;
    };
}
pub unsafe extern "C" fn argv_iter_n_args(mut ai: *const argv_iterator) -> size_t {
    return if !((*ai).fp).is_null() {
        (*ai).item_idx
    } else {
        ((*ai).p).offset_from((*ai).arg_list) as libc::c_long as libc::c_ulong
    };
}
pub unsafe extern "C" fn argv_iter_free(mut ai: *mut argv_iterator) {
    if !((*ai).fp).is_null() {
        rpl_free((*ai).tok as *mut libc::c_void);
    }
    rpl_free(ai as *mut libc::c_void);
}
