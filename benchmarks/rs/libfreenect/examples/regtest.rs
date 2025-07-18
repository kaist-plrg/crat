use ::libc;
extern "C" {
    pub type _IO_wide_data;
    pub type _IO_codecvt;
    pub type _IO_marker;
    static mut stderr: *mut FILE;
    fn fclose(__stream: *mut FILE) -> libc::c_int;
    fn fopen(_: *const libc::c_char, _: *const libc::c_char) -> *mut FILE;
    fn fprintf(_: *mut FILE, _: *const libc::c_char, _: ...) -> libc::c_int;
    fn printf(_: *const libc::c_char, _: ...) -> libc::c_int;
    fn fwrite(
        _: *const libc::c_void,
        _: libc::c_ulong,
        _: libc::c_ulong,
        _: *mut FILE,
    ) -> libc::c_ulong;
    fn exit(_: libc::c_int) -> !;
    fn freenect_sync_get_video(
        video: *mut *mut libc::c_void,
        timestamp: *mut uint32_t,
        index: libc::c_int,
        fmt: freenect_video_format,
    ) -> libc::c_int;
    fn freenect_sync_get_depth(
        depth: *mut *mut libc::c_void,
        timestamp: *mut uint32_t,
        index: libc::c_int,
        fmt: freenect_depth_format,
    ) -> libc::c_int;
}
pub type size_t = libc::c_ulong;
pub type __uint32_t = libc::c_uint;
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
pub type uint32_t = __uint32_t;
pub type freenect_video_format = libc::c_uint;
pub const FREENECT_VIDEO_DUMMY: freenect_video_format = 2147483647;
pub const FREENECT_VIDEO_YUV_RAW: freenect_video_format = 6;
pub const FREENECT_VIDEO_YUV_RGB: freenect_video_format = 5;
pub const FREENECT_VIDEO_IR_10BIT_PACKED: freenect_video_format = 4;
pub const FREENECT_VIDEO_IR_10BIT: freenect_video_format = 3;
pub const FREENECT_VIDEO_IR_8BIT: freenect_video_format = 2;
pub const FREENECT_VIDEO_BAYER: freenect_video_format = 1;
pub const FREENECT_VIDEO_RGB: freenect_video_format = 0;
pub type freenect_depth_format = libc::c_uint;
pub const FREENECT_DEPTH_DUMMY: freenect_depth_format = 2147483647;
pub const FREENECT_DEPTH_MM: freenect_depth_format = 5;
pub const FREENECT_DEPTH_REGISTERED: freenect_depth_format = 4;
pub const FREENECT_DEPTH_10BIT_PACKED: freenect_depth_format = 3;
pub const FREENECT_DEPTH_11BIT_PACKED: freenect_depth_format = 2;
pub const FREENECT_DEPTH_10BIT: freenect_depth_format = 1;
pub const FREENECT_DEPTH_11BIT: freenect_depth_format = 0;
pub unsafe extern "C" fn open_dump(mut filename: *const libc::c_char) -> *mut FILE {
    let mut fp: *mut FILE = fopen(filename, b"w\0" as *const u8 as *const libc::c_char);
    if fp.is_null() {
        fprintf(
            stderr,
            b"Error: Cannot open file [%s]\n\0" as *const u8 as *const libc::c_char,
            filename,
        );
        exit(1 as libc::c_int);
    }
    printf(b"%s\n\0" as *const u8 as *const libc::c_char, filename);
    return fp;
}
pub unsafe extern "C" fn dump_depth(
    mut fp: *mut FILE,
    mut data: *mut libc::c_void,
    mut width: libc::c_uint,
    mut height: libc::c_uint,
) {
    fprintf(
        fp,
        b"P5 %u %u 65535\n\0" as *const u8 as *const libc::c_char,
        width,
        height,
    );
    fwrite(
        data,
        width.wrapping_mul(height).wrapping_mul(2 as libc::c_int as libc::c_uint)
            as libc::c_ulong,
        1 as libc::c_int as libc::c_ulong,
        fp,
    );
}
pub unsafe extern "C" fn dump_rgb(
    mut fp: *mut FILE,
    mut data: *mut libc::c_void,
    mut width: libc::c_uint,
    mut height: libc::c_uint,
) {
    fprintf(fp, b"P6 %u %u 255\n\0" as *const u8 as *const libc::c_char, width, height);
    fwrite(
        data,
        width.wrapping_mul(height).wrapping_mul(3 as libc::c_int as libc::c_uint)
            as libc::c_ulong,
        1 as libc::c_int as libc::c_ulong,
        fp,
    );
}
pub unsafe extern "C" fn no_kinect_quit() {
    fprintf(
        stderr,
        b"Error: Kinect not connected?\n\0" as *const u8 as *const libc::c_char,
    );
    exit(1 as libc::c_int);
}
unsafe fn main_0() -> libc::c_int {
    let mut depth: *mut libc::c_short = 0 as *mut libc::c_short;
    let mut rgb: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut ts: uint32_t = 0;
    let mut fp: *mut FILE = 0 as *mut FILE;
    let mut ret: libc::c_int = 0;
    ret = freenect_sync_get_video(
        &mut rgb as *mut *mut libc::c_char as *mut *mut libc::c_void,
        &mut ts,
        0 as libc::c_int,
        FREENECT_VIDEO_RGB,
    );
    if ret < 0 as libc::c_int {
        no_kinect_quit();
    }
    fp = open_dump(b"registration_test_rgb.ppm\0" as *const u8 as *const libc::c_char);
    dump_rgb(
        fp,
        rgb as *mut libc::c_void,
        640 as libc::c_int as libc::c_uint,
        480 as libc::c_int as libc::c_uint,
    );
    fclose(fp);
    ret = freenect_sync_get_depth(
        &mut depth as *mut *mut libc::c_short as *mut *mut libc::c_void,
        &mut ts,
        0 as libc::c_int,
        FREENECT_DEPTH_11BIT,
    );
    if ret < 0 as libc::c_int {
        no_kinect_quit();
    }
    fp = open_dump(
        b"registration_test_depth_raw.pgm\0" as *const u8 as *const libc::c_char,
    );
    dump_depth(
        fp,
        depth as *mut libc::c_void,
        640 as libc::c_int as libc::c_uint,
        480 as libc::c_int as libc::c_uint,
    );
    fclose(fp);
    ret = freenect_sync_get_depth(
        &mut depth as *mut *mut libc::c_short as *mut *mut libc::c_void,
        &mut ts,
        0 as libc::c_int,
        FREENECT_DEPTH_REGISTERED,
    );
    if ret < 0 as libc::c_int {
        no_kinect_quit();
    }
    fp = open_dump(
        b"registration_test_depth_registered.pgm\0" as *const u8 as *const libc::c_char,
    );
    dump_depth(
        fp,
        depth as *mut libc::c_void,
        640 as libc::c_int as libc::c_uint,
        480 as libc::c_int as libc::c_uint,
    );
    fclose(fp);
    ret = freenect_sync_get_depth(
        &mut depth as *mut *mut libc::c_short as *mut *mut libc::c_void,
        &mut ts,
        0 as libc::c_int,
        FREENECT_DEPTH_MM,
    );
    if ret < 0 as libc::c_int {
        no_kinect_quit();
    }
    fp = open_dump(
        b"registration_test_depth_mm.pgm\0" as *const u8 as *const libc::c_char,
    );
    dump_depth(
        fp,
        depth as *mut libc::c_void,
        640 as libc::c_int as libc::c_uint,
        480 as libc::c_int as libc::c_uint,
    );
    fclose(fp);
    return 0 as libc::c_int;
}
pub fn main() {
    unsafe { ::std::process::exit(main_0() as i32) }
}
