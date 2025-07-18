use ::libc;
extern "C" {
    pub type _IO_wide_data;
    pub type _IO_codecvt;
    pub type _IO_marker;
    static mut stderr: *mut FILE;
    fn fprintf(_: *mut FILE, _: *const libc::c_char, _: ...) -> libc::c_int;
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
    fn malloc(_: libc::c_ulong) -> *mut libc::c_void;
    fn realloc(_: *mut libc::c_void, _: libc::c_ulong) -> *mut libc::c_void;
    fn exit(_: libc::c_int) -> !;
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
pub static mut lt__alloc_die: Option::<unsafe extern "C" fn() -> ()> = unsafe {
    Some(alloc_die_default as unsafe extern "C" fn() -> ())
};
unsafe extern "C" fn alloc_die_default() {
    fprintf(stderr, b"Out of memory.\n\0" as *const u8 as *const libc::c_char);
    exit(1 as libc::c_int);
}
pub unsafe extern "C" fn lt__malloc(mut n: size_t) -> *mut libc::c_void {
    let mut mem: *mut libc::c_void = 0 as *mut libc::c_void;
    mem = malloc(n);
    if mem.is_null() {
        (Some(lt__alloc_die.unwrap())).unwrap()();
    }
    return mem;
}
pub unsafe extern "C" fn lt__zalloc(mut n: size_t) -> *mut libc::c_void {
    let mut mem: *mut libc::c_void = 0 as *mut libc::c_void;
    mem = lt__malloc(n);
    if !mem.is_null() {
        memset(mem, 0 as libc::c_int, n);
    }
    return mem;
}
pub unsafe extern "C" fn lt__realloc(
    mut mem: *mut libc::c_void,
    mut n: size_t,
) -> *mut libc::c_void {
    mem = realloc(mem, n);
    if mem.is_null() {
        (Some(lt__alloc_die.unwrap())).unwrap()();
    }
    return mem;
}
pub unsafe extern "C" fn lt__memdup(
    mut mem: *const libc::c_void,
    mut n: size_t,
) -> *mut libc::c_void {
    let mut newmem: *mut libc::c_void = 0 as *mut libc::c_void;
    newmem = lt__malloc(n);
    if !newmem.is_null() {
        return memcpy(newmem, mem, n);
    }
    return 0 as *mut libc::c_void;
}
pub unsafe extern "C" fn lt__strdup(
    mut string: *const libc::c_char,
) -> *mut libc::c_char {
    return lt__memdup(
        string as *const libc::c_void,
        (strlen(string)).wrapping_add(1 as libc::c_int as libc::c_ulong),
    ) as *mut libc::c_char;
}
