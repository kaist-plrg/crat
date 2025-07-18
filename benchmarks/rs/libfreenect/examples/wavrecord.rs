use ::libc;
extern "C" {
    pub type _freenect_context;
    pub type _freenect_device;
    pub type _IO_wide_data;
    pub type _IO_codecvt;
    pub type _IO_marker;
    fn freenect_init(
        ctx: *mut *mut freenect_context,
        usb_ctx: *mut libc::c_void,
    ) -> libc::c_int;
    fn freenect_shutdown(ctx: *mut freenect_context) -> libc::c_int;
    fn freenect_set_log_level(ctx: *mut freenect_context, level: freenect_loglevel);
    fn freenect_process_events(ctx: *mut freenect_context) -> libc::c_int;
    fn freenect_num_devices(ctx: *mut freenect_context) -> libc::c_int;
    fn freenect_select_subdevices(
        ctx: *mut freenect_context,
        subdevs: freenect_device_flags,
    );
    fn freenect_open_device(
        ctx: *mut freenect_context,
        dev: *mut *mut freenect_device,
        index: libc::c_int,
    ) -> libc::c_int;
    fn freenect_set_user(dev: *mut freenect_device, user: *mut libc::c_void);
    fn freenect_get_user(dev: *mut freenect_device) -> *mut libc::c_void;
    fn freenect_set_audio_in_callback(
        dev: *mut freenect_device,
        callback: freenect_audio_in_cb,
    );
    fn freenect_start_audio(dev: *mut freenect_device) -> libc::c_int;
    fn fclose(__stream: *mut FILE) -> libc::c_int;
    fn fopen(_: *const libc::c_char, _: *const libc::c_char) -> *mut FILE;
    fn printf(_: *const libc::c_char, _: ...) -> libc::c_int;
    fn fwrite(
        _: *const libc::c_void,
        _: libc::c_ulong,
        _: libc::c_ulong,
        _: *mut FILE,
    ) -> libc::c_ulong;
    fn fseek(
        __stream: *mut FILE,
        __off: libc::c_long,
        __whence: libc::c_int,
    ) -> libc::c_int;
    fn signal(__sig: libc::c_int, __handler: __sighandler_t) -> __sighandler_t;
}
pub type __int16_t = libc::c_short;
pub type __int32_t = libc::c_int;
pub type __off_t = libc::c_long;
pub type __off64_t = libc::c_long;
pub type int16_t = __int16_t;
pub type int32_t = __int32_t;
pub type freenect_device_flags = libc::c_uint;
pub const FREENECT_DEVICE_AUDIO: freenect_device_flags = 4;
pub const FREENECT_DEVICE_CAMERA: freenect_device_flags = 2;
pub const FREENECT_DEVICE_MOTOR: freenect_device_flags = 1;
pub type freenect_context = _freenect_context;
pub type freenect_device = _freenect_device;
pub type freenect_usb_context = ();
pub type freenect_loglevel = libc::c_uint;
pub const FREENECT_LOG_FLOOD: freenect_loglevel = 7;
pub const FREENECT_LOG_SPEW: freenect_loglevel = 6;
pub const FREENECT_LOG_DEBUG: freenect_loglevel = 5;
pub const FREENECT_LOG_INFO: freenect_loglevel = 4;
pub const FREENECT_LOG_NOTICE: freenect_loglevel = 3;
pub const FREENECT_LOG_WARNING: freenect_loglevel = 2;
pub const FREENECT_LOG_ERROR: freenect_loglevel = 1;
pub const FREENECT_LOG_FATAL: freenect_loglevel = 0;
pub type freenect_audio_in_cb = Option::<
    unsafe extern "C" fn(
        *mut freenect_device,
        libc::c_int,
        *mut int32_t,
        *mut int32_t,
        *mut int32_t,
        *mut int32_t,
        *mut int16_t,
        *mut libc::c_void,
    ) -> (),
>;
pub type size_t = libc::c_ulong;
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
pub type __sighandler_t = Option::<unsafe extern "C" fn(libc::c_int) -> ()>;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct capture {
    pub logfiles: [*mut FILE; 4],
    pub samples: libc::c_int,
}
static mut f_ctx: *mut freenect_context = 0 as *const freenect_context
    as *mut freenect_context;
static mut f_dev: *mut freenect_device = 0 as *const freenect_device
    as *mut freenect_device;
pub static mut die: libc::c_int = 0 as libc::c_int;
pub static mut wavheader: [libc::c_char; 44] = [
    0x52 as libc::c_int as libc::c_char,
    0x49 as libc::c_int as libc::c_char,
    0x46 as libc::c_int as libc::c_char,
    0x46 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    0x57 as libc::c_int as libc::c_char,
    0x41 as libc::c_int as libc::c_char,
    0x56 as libc::c_int as libc::c_char,
    0x45 as libc::c_int as libc::c_char,
    0x66 as libc::c_int as libc::c_char,
    0x6d as libc::c_int as libc::c_char,
    0x74 as libc::c_int as libc::c_char,
    0x20 as libc::c_int as libc::c_char,
    0x10 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    0x1 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    0x1 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    0x80 as libc::c_int as libc::c_char,
    0x3e as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    0xfa as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    0x4 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    0x20 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    0x64 as libc::c_int as libc::c_char,
    0x61 as libc::c_int as libc::c_char,
    0x74 as libc::c_int as libc::c_char,
    0x61 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
];
pub unsafe extern "C" fn in_callback(
    mut dev: *mut freenect_device,
    mut num_samples: libc::c_int,
    mut mic1: *mut int32_t,
    mut mic2: *mut int32_t,
    mut mic3: *mut int32_t,
    mut mic4: *mut int32_t,
    mut cancelled: *mut int16_t,
    mut unknown: *mut libc::c_void,
) {
    let mut c: *mut capture = freenect_get_user(dev) as *mut capture;
    fwrite(
        mic1 as *const libc::c_void,
        1 as libc::c_int as libc::c_ulong,
        (num_samples as libc::c_ulong)
            .wrapping_mul(::std::mem::size_of::<int32_t>() as libc::c_ulong),
        (*c).logfiles[0 as libc::c_int as usize],
    );
    fwrite(
        mic2 as *const libc::c_void,
        1 as libc::c_int as libc::c_ulong,
        (num_samples as libc::c_ulong)
            .wrapping_mul(::std::mem::size_of::<int32_t>() as libc::c_ulong),
        (*c).logfiles[1 as libc::c_int as usize],
    );
    fwrite(
        mic3 as *const libc::c_void,
        1 as libc::c_int as libc::c_ulong,
        (num_samples as libc::c_ulong)
            .wrapping_mul(::std::mem::size_of::<int32_t>() as libc::c_ulong),
        (*c).logfiles[2 as libc::c_int as usize],
    );
    fwrite(
        mic4 as *const libc::c_void,
        1 as libc::c_int as libc::c_ulong,
        (num_samples as libc::c_ulong)
            .wrapping_mul(::std::mem::size_of::<int32_t>() as libc::c_ulong),
        (*c).logfiles[3 as libc::c_int as usize],
    );
    (*c).samples += num_samples;
    printf(
        b"Sample received.  Total samples recorded: %d\n\0" as *const u8
            as *const libc::c_char,
        (*c).samples,
    );
}
pub unsafe extern "C" fn cleanup(mut sig: libc::c_int) {
    printf(b"Caught SIGINT, cleaning up\n\0" as *const u8 as *const libc::c_char);
    die = 1 as libc::c_int;
}
unsafe fn main_0(
    mut argc: libc::c_int,
    mut argv: *mut *mut libc::c_char,
) -> libc::c_int {
    if freenect_init(&mut f_ctx, 0 as *mut libc::c_void) < 0 as libc::c_int {
        printf(b"freenect_init() failed\n\0" as *const u8 as *const libc::c_char);
        return 1 as libc::c_int;
    }
    freenect_set_log_level(f_ctx, FREENECT_LOG_SPEW);
    freenect_select_subdevices(f_ctx, FREENECT_DEVICE_AUDIO);
    let mut nr_devices: libc::c_int = freenect_num_devices(f_ctx);
    printf(
        b"Number of devices found: %d\n\0" as *const u8 as *const libc::c_char,
        nr_devices,
    );
    if nr_devices < 1 as libc::c_int {
        freenect_shutdown(f_ctx);
        return 1 as libc::c_int;
    }
    let mut user_device_number: libc::c_int = 0 as libc::c_int;
    if freenect_open_device(f_ctx, &mut f_dev, user_device_number) < 0 as libc::c_int {
        printf(b"Could not open device\n\0" as *const u8 as *const libc::c_char);
        freenect_shutdown(f_ctx);
        return 1 as libc::c_int;
    }
    let mut state: capture = capture {
        logfiles: [0 as *mut FILE; 4],
        samples: 0,
    };
    state.samples = 0 as libc::c_int;
    state
        .logfiles[0 as libc::c_int
        as usize] = fopen(
        b"channel1.wav\0" as *const u8 as *const libc::c_char,
        b"wb\0" as *const u8 as *const libc::c_char,
    );
    state
        .logfiles[1 as libc::c_int
        as usize] = fopen(
        b"channel2.wav\0" as *const u8 as *const libc::c_char,
        b"wb\0" as *const u8 as *const libc::c_char,
    );
    state
        .logfiles[2 as libc::c_int
        as usize] = fopen(
        b"channel3.wav\0" as *const u8 as *const libc::c_char,
        b"wb\0" as *const u8 as *const libc::c_char,
    );
    state
        .logfiles[3 as libc::c_int
        as usize] = fopen(
        b"channel4.wav\0" as *const u8 as *const libc::c_char,
        b"wb\0" as *const u8 as *const libc::c_char,
    );
    fwrite(
        wavheader.as_mut_ptr() as *const libc::c_void,
        1 as libc::c_int as libc::c_ulong,
        44 as libc::c_int as libc::c_ulong,
        state.logfiles[0 as libc::c_int as usize],
    );
    fwrite(
        wavheader.as_mut_ptr() as *const libc::c_void,
        1 as libc::c_int as libc::c_ulong,
        44 as libc::c_int as libc::c_ulong,
        state.logfiles[1 as libc::c_int as usize],
    );
    fwrite(
        wavheader.as_mut_ptr() as *const libc::c_void,
        1 as libc::c_int as libc::c_ulong,
        44 as libc::c_int as libc::c_ulong,
        state.logfiles[2 as libc::c_int as usize],
    );
    fwrite(
        wavheader.as_mut_ptr() as *const libc::c_void,
        1 as libc::c_int as libc::c_ulong,
        44 as libc::c_int as libc::c_ulong,
        state.logfiles[3 as libc::c_int as usize],
    );
    freenect_set_user(f_dev, &mut state as *mut capture as *mut libc::c_void);
    freenect_set_audio_in_callback(
        f_dev,
        Some(
            in_callback
                as unsafe extern "C" fn(
                    *mut freenect_device,
                    libc::c_int,
                    *mut int32_t,
                    *mut int32_t,
                    *mut int32_t,
                    *mut int32_t,
                    *mut int16_t,
                    *mut libc::c_void,
                ) -> (),
        ),
    );
    freenect_start_audio(f_dev);
    signal(2 as libc::c_int, Some(cleanup as unsafe extern "C" fn(libc::c_int) -> ()));
    while die == 0 && freenect_process_events(f_ctx) >= 0 as libc::c_int {}
    let mut i: libc::c_int = 0;
    i = 0 as libc::c_int;
    while i < 4 as libc::c_int {
        let mut buf: [libc::c_char; 4] = [0; 4];
        fseek(
            state.logfiles[i as usize],
            4 as libc::c_int as libc::c_long,
            0 as libc::c_int,
        );
        let mut chunksize: libc::c_int = state.samples * 4 as libc::c_int
            + 36 as libc::c_int;
        buf[0 as libc::c_int
            as usize] = (chunksize & 0xff as libc::c_int) as libc::c_char;
        buf[1 as libc::c_int
            as usize] = ((chunksize & 0xff00 as libc::c_int) >> 8 as libc::c_int)
            as libc::c_char;
        buf[2 as libc::c_int
            as usize] = ((chunksize & 0xff0000 as libc::c_int) >> 16 as libc::c_int)
            as libc::c_char;
        buf[3 as libc::c_int
            as usize] = ((chunksize as libc::c_uint & 0xff000000 as libc::c_uint)
            >> 24 as libc::c_int) as libc::c_char;
        fwrite(
            buf.as_mut_ptr() as *const libc::c_void,
            1 as libc::c_int as libc::c_ulong,
            4 as libc::c_int as libc::c_ulong,
            state.logfiles[i as usize],
        );
        fseek(
            state.logfiles[i as usize],
            40 as libc::c_int as libc::c_long,
            0 as libc::c_int,
        );
        let mut subchunk2size: libc::c_int = state.samples * 4 as libc::c_int;
        buf[0 as libc::c_int
            as usize] = (subchunk2size & 0xff as libc::c_int) as libc::c_char;
        buf[1 as libc::c_int
            as usize] = ((subchunk2size & 0xff00 as libc::c_int) >> 8 as libc::c_int)
            as libc::c_char;
        buf[2 as libc::c_int
            as usize] = ((subchunk2size & 0xff0000 as libc::c_int) >> 16 as libc::c_int)
            as libc::c_char;
        buf[3 as libc::c_int
            as usize] = ((subchunk2size as libc::c_uint & 0xff000000 as libc::c_uint)
            >> 24 as libc::c_int) as libc::c_char;
        fwrite(
            buf.as_mut_ptr() as *const libc::c_void,
            1 as libc::c_int as libc::c_ulong,
            4 as libc::c_int as libc::c_ulong,
            state.logfiles[i as usize],
        );
        fclose(state.logfiles[i as usize]);
        i += 1;
        i;
    }
    freenect_shutdown(f_ctx);
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
