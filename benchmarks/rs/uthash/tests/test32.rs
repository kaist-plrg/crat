use ::libc;
extern "C" {
    pub type _IO_wide_data;
    pub type _IO_codecvt;
    pub type _IO_marker;
    fn fclose(__stream: *mut FILE) -> libc::c_int;
    fn fopen(_: *const libc::c_char, _: *const libc::c_char) -> *mut FILE;
    fn printf(_: *const libc::c_char, _: ...) -> libc::c_int;
    fn perror(__s: *const libc::c_char);
    fn fgets(
        __s: *mut libc::c_char,
        __n: libc::c_int,
        __stream: *mut FILE,
    ) -> *mut libc::c_char;
    fn malloc(_: libc::c_ulong) -> *mut libc::c_void;
    fn exit(_: libc::c_int) -> !;
    fn strcpy(_: *mut libc::c_char, _: *const libc::c_char) -> *mut libc::c_char;
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
pub struct el {
    pub bname: [libc::c_char; 20],
    pub next: *mut el,
    pub prev: *mut el,
}
unsafe fn main_0() -> libc::c_int {
    let mut name: *mut el = 0 as *mut el;
    let mut tmp: *mut el = 0 as *mut el;
    let mut head: *mut el = 0 as *mut el;
    let mut linebuf: [libc::c_char; 20] = [0; 20];
    let mut file: *mut FILE = 0 as *mut FILE;
    file = fopen(
        b"test11.dat\0" as *const u8 as *const libc::c_char,
        b"r\0" as *const u8 as *const libc::c_char,
    );
    if file.is_null() {
        perror(b"can't open: \0" as *const u8 as *const libc::c_char);
        exit(-(1 as libc::c_int));
    }
    while !(fgets(linebuf.as_mut_ptr(), 20 as libc::c_int, file)).is_null() {
        name = malloc(::std::mem::size_of::<el>() as libc::c_ulong) as *mut el;
        if name.is_null() {
            exit(-(1 as libc::c_int));
        }
        strcpy(((*name).bname).as_mut_ptr(), linebuf.as_mut_ptr());
        (*name).next = head;
        if !head.is_null() {
            (*name).prev = (*head).prev;
            (*head).prev = name;
        } else {
            (*name).prev = name;
        }
        head = name;
    }
    tmp = head;
    while !tmp.is_null() {
        printf(b"%s\0" as *const u8 as *const libc::c_char, ((*tmp).bname).as_mut_ptr());
        tmp = (*tmp).next;
    }
    fclose(file);
    return 0 as libc::c_int;
}
pub fn main() {
    unsafe { ::std::process::exit(main_0() as i32) }
}
