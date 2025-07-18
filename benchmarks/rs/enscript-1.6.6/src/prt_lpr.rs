use ::libc;
extern "C" {
    pub type _IO_wide_data;
    pub type _IO_codecvt;
    pub type _IO_marker;
    fn pclose(__stream: *mut FILE) -> libc::c_int;
    fn popen(__command: *const libc::c_char, __modes: *const libc::c_char) -> *mut FILE;
    fn buffer_init(buffer: *mut Buffer);
    fn buffer_uninit(buffer: *mut Buffer);
    fn buffer_append(buffer: *mut Buffer, data: *const libc::c_char);
    fn buffer_ptr(buffer: *mut Buffer) -> *mut libc::c_char;
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
pub struct buffer_st {
    pub data: *mut libc::c_char,
    pub allocated: size_t,
    pub len: size_t,
}
pub type Buffer = buffer_st;
pub unsafe extern "C" fn printer_open(
    mut cmd: *mut libc::c_char,
    mut options: *mut libc::c_char,
    mut queue_param: *mut libc::c_char,
    mut printer_name: *mut libc::c_char,
    mut context_return: *mut *mut libc::c_void,
) -> *mut FILE {
    let mut pipe_cmd: Buffer = Buffer {
        data: 0 as *mut libc::c_char,
        allocated: 0,
        len: 0,
    };
    let mut fp: *mut FILE = 0 as *mut FILE;
    buffer_init(&mut pipe_cmd);
    buffer_append(&mut pipe_cmd, cmd);
    buffer_append(&mut pipe_cmd, b" \0" as *const u8 as *const libc::c_char);
    if !options.is_null() {
        buffer_append(&mut pipe_cmd, options);
        buffer_append(&mut pipe_cmd, b" \0" as *const u8 as *const libc::c_char);
    }
    if !printer_name.is_null() {
        buffer_append(&mut pipe_cmd, queue_param);
        buffer_append(&mut pipe_cmd, printer_name);
    }
    fp = popen(buffer_ptr(&mut pipe_cmd), b"w\0" as *const u8 as *const libc::c_char);
    buffer_uninit(&mut pipe_cmd);
    *context_return = fp as *mut libc::c_void;
    return fp;
}
pub unsafe extern "C" fn printer_close(mut context: *mut libc::c_void) {
    let mut fp: *mut FILE = context as *mut FILE;
    pclose(fp);
}
