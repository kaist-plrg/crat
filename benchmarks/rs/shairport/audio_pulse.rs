use ::libc;
extern "C" {
    pub type _IO_wide_data;
    pub type _IO_codecvt;
    pub type _IO_marker;
    pub type pa_simple;
    static mut stderr: *mut FILE;
    fn fprintf(_: *mut FILE, _: *const libc::c_char, _: ...) -> libc::c_int;
    fn printf(_: *const libc::c_char, _: ...) -> libc::c_int;
    static mut optarg: *mut libc::c_char;
    static mut optind: libc::c_int;
    fn getopt(
        ___argc: libc::c_int,
        ___argv: *const *mut libc::c_char,
        __shortopts: *const libc::c_char,
    ) -> libc::c_int;
    fn pa_simple_drain(s: *mut pa_simple, error: *mut libc::c_int) -> libc::c_int;
    fn pa_simple_write(
        s: *mut pa_simple,
        data: *const libc::c_void,
        bytes: size_t,
        error: *mut libc::c_int,
    ) -> libc::c_int;
    fn pa_simple_new(
        server: *const libc::c_char,
        name: *const libc::c_char,
        dir: pa_stream_direction_t,
        dev: *const libc::c_char,
        stream_name: *const libc::c_char,
        ss: *const pa_sample_spec,
        map: *const pa_channel_map,
        attr: *const pa_buffer_attr,
        error: *mut libc::c_int,
    ) -> *mut pa_simple;
    fn pa_simple_free(s: *mut pa_simple);
    fn pa_strerror(error: libc::c_int) -> *const libc::c_char;
    static mut config: shairport_cfg;
    fn die(format: *mut libc::c_char, _: ...);
}
pub type size_t = libc::c_ulong;
pub type __uint8_t = libc::c_uchar;
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
pub type uint8_t = __uint8_t;
pub type uint32_t = __uint32_t;
pub type pa_sample_format = libc::c_int;
pub const PA_SAMPLE_INVALID: pa_sample_format = -1;
pub const PA_SAMPLE_MAX: pa_sample_format = 13;
pub const PA_SAMPLE_S24_32BE: pa_sample_format = 12;
pub const PA_SAMPLE_S24_32LE: pa_sample_format = 11;
pub const PA_SAMPLE_S24BE: pa_sample_format = 10;
pub const PA_SAMPLE_S24LE: pa_sample_format = 9;
pub const PA_SAMPLE_S32BE: pa_sample_format = 8;
pub const PA_SAMPLE_S32LE: pa_sample_format = 7;
pub const PA_SAMPLE_FLOAT32BE: pa_sample_format = 6;
pub const PA_SAMPLE_FLOAT32LE: pa_sample_format = 5;
pub const PA_SAMPLE_S16BE: pa_sample_format = 4;
pub const PA_SAMPLE_S16LE: pa_sample_format = 3;
pub const PA_SAMPLE_ULAW: pa_sample_format = 2;
pub const PA_SAMPLE_ALAW: pa_sample_format = 1;
pub const PA_SAMPLE_U8: pa_sample_format = 0;
pub type pa_sample_format_t = pa_sample_format;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct pa_sample_spec {
    pub format: pa_sample_format_t,
    pub rate: uint32_t,
    pub channels: uint8_t,
}
pub type pa_channel_position = libc::c_int;
pub const PA_CHANNEL_POSITION_MAX: pa_channel_position = 51;
pub const PA_CHANNEL_POSITION_TOP_REAR_CENTER: pa_channel_position = 50;
pub const PA_CHANNEL_POSITION_TOP_REAR_RIGHT: pa_channel_position = 49;
pub const PA_CHANNEL_POSITION_TOP_REAR_LEFT: pa_channel_position = 48;
pub const PA_CHANNEL_POSITION_TOP_FRONT_CENTER: pa_channel_position = 47;
pub const PA_CHANNEL_POSITION_TOP_FRONT_RIGHT: pa_channel_position = 46;
pub const PA_CHANNEL_POSITION_TOP_FRONT_LEFT: pa_channel_position = 45;
pub const PA_CHANNEL_POSITION_TOP_CENTER: pa_channel_position = 44;
pub const PA_CHANNEL_POSITION_AUX31: pa_channel_position = 43;
pub const PA_CHANNEL_POSITION_AUX30: pa_channel_position = 42;
pub const PA_CHANNEL_POSITION_AUX29: pa_channel_position = 41;
pub const PA_CHANNEL_POSITION_AUX28: pa_channel_position = 40;
pub const PA_CHANNEL_POSITION_AUX27: pa_channel_position = 39;
pub const PA_CHANNEL_POSITION_AUX26: pa_channel_position = 38;
pub const PA_CHANNEL_POSITION_AUX25: pa_channel_position = 37;
pub const PA_CHANNEL_POSITION_AUX24: pa_channel_position = 36;
pub const PA_CHANNEL_POSITION_AUX23: pa_channel_position = 35;
pub const PA_CHANNEL_POSITION_AUX22: pa_channel_position = 34;
pub const PA_CHANNEL_POSITION_AUX21: pa_channel_position = 33;
pub const PA_CHANNEL_POSITION_AUX20: pa_channel_position = 32;
pub const PA_CHANNEL_POSITION_AUX19: pa_channel_position = 31;
pub const PA_CHANNEL_POSITION_AUX18: pa_channel_position = 30;
pub const PA_CHANNEL_POSITION_AUX17: pa_channel_position = 29;
pub const PA_CHANNEL_POSITION_AUX16: pa_channel_position = 28;
pub const PA_CHANNEL_POSITION_AUX15: pa_channel_position = 27;
pub const PA_CHANNEL_POSITION_AUX14: pa_channel_position = 26;
pub const PA_CHANNEL_POSITION_AUX13: pa_channel_position = 25;
pub const PA_CHANNEL_POSITION_AUX12: pa_channel_position = 24;
pub const PA_CHANNEL_POSITION_AUX11: pa_channel_position = 23;
pub const PA_CHANNEL_POSITION_AUX10: pa_channel_position = 22;
pub const PA_CHANNEL_POSITION_AUX9: pa_channel_position = 21;
pub const PA_CHANNEL_POSITION_AUX8: pa_channel_position = 20;
pub const PA_CHANNEL_POSITION_AUX7: pa_channel_position = 19;
pub const PA_CHANNEL_POSITION_AUX6: pa_channel_position = 18;
pub const PA_CHANNEL_POSITION_AUX5: pa_channel_position = 17;
pub const PA_CHANNEL_POSITION_AUX4: pa_channel_position = 16;
pub const PA_CHANNEL_POSITION_AUX3: pa_channel_position = 15;
pub const PA_CHANNEL_POSITION_AUX2: pa_channel_position = 14;
pub const PA_CHANNEL_POSITION_AUX1: pa_channel_position = 13;
pub const PA_CHANNEL_POSITION_AUX0: pa_channel_position = 12;
pub const PA_CHANNEL_POSITION_SIDE_RIGHT: pa_channel_position = 11;
pub const PA_CHANNEL_POSITION_SIDE_LEFT: pa_channel_position = 10;
pub const PA_CHANNEL_POSITION_FRONT_RIGHT_OF_CENTER: pa_channel_position = 9;
pub const PA_CHANNEL_POSITION_FRONT_LEFT_OF_CENTER: pa_channel_position = 8;
pub const PA_CHANNEL_POSITION_SUBWOOFER: pa_channel_position = 7;
pub const PA_CHANNEL_POSITION_LFE: pa_channel_position = 7;
pub const PA_CHANNEL_POSITION_REAR_RIGHT: pa_channel_position = 6;
pub const PA_CHANNEL_POSITION_REAR_LEFT: pa_channel_position = 5;
pub const PA_CHANNEL_POSITION_REAR_CENTER: pa_channel_position = 4;
pub const PA_CHANNEL_POSITION_CENTER: pa_channel_position = 3;
pub const PA_CHANNEL_POSITION_RIGHT: pa_channel_position = 2;
pub const PA_CHANNEL_POSITION_LEFT: pa_channel_position = 1;
pub const PA_CHANNEL_POSITION_FRONT_CENTER: pa_channel_position = 3;
pub const PA_CHANNEL_POSITION_FRONT_RIGHT: pa_channel_position = 2;
pub const PA_CHANNEL_POSITION_FRONT_LEFT: pa_channel_position = 1;
pub const PA_CHANNEL_POSITION_MONO: pa_channel_position = 0;
pub const PA_CHANNEL_POSITION_INVALID: pa_channel_position = -1;
pub type pa_channel_position_t = pa_channel_position;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct pa_channel_map {
    pub channels: uint8_t,
    pub map: [pa_channel_position_t; 32],
}
pub type pa_stream_direction = libc::c_uint;
pub const PA_STREAM_UPLOAD: pa_stream_direction = 3;
pub const PA_STREAM_RECORD: pa_stream_direction = 2;
pub const PA_STREAM_PLAYBACK: pa_stream_direction = 1;
pub const PA_STREAM_NODIRECTION: pa_stream_direction = 0;
pub type pa_stream_direction_t = pa_stream_direction;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct pa_buffer_attr {
    pub maxlength: uint32_t,
    pub tlength: uint32_t,
    pub prebuf: uint32_t,
    pub minreq: uint32_t,
    pub fragsize: uint32_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct audio_output {
    pub help: Option::<unsafe extern "C" fn() -> ()>,
    pub name: *mut libc::c_char,
    pub init: Option::<
        unsafe extern "C" fn(libc::c_int, *mut *mut libc::c_char) -> libc::c_int,
    >,
    pub deinit: Option::<unsafe extern "C" fn() -> ()>,
    pub start: Option::<unsafe extern "C" fn(libc::c_int) -> ()>,
    pub play: Option::<unsafe extern "C" fn(*mut libc::c_short, libc::c_int) -> ()>,
    pub stop: Option::<unsafe extern "C" fn() -> ()>,
    pub volume: Option::<unsafe extern "C" fn(libc::c_double) -> ()>,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct mdns_backend {
    pub name: *mut libc::c_char,
    pub mdns_register: Option::<
        unsafe extern "C" fn(*mut libc::c_char, libc::c_int) -> libc::c_int,
    >,
    pub mdns_unregister: Option::<unsafe extern "C" fn() -> ()>,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct shairport_cfg {
    pub password: *mut libc::c_char,
    pub apname: *mut libc::c_char,
    pub hw_addr: [uint8_t; 6],
    pub port: libc::c_int,
    pub output_name: *mut libc::c_char,
    pub output: *mut audio_output,
    pub mdns_name: *mut libc::c_char,
    pub mdns: *mut mdns_backend,
    pub buffer_start_fill: libc::c_int,
    pub daemonise: libc::c_int,
    pub cmd_start: *mut libc::c_char,
    pub cmd_stop: *mut libc::c_char,
    pub cmd_blocking: libc::c_int,
    pub meta_dir: *mut libc::c_char,
    pub pidfile: *mut libc::c_char,
    pub logfile: *mut libc::c_char,
    pub errfile: *mut libc::c_char,
}
static mut pa_dev: *mut pa_simple = 0 as *const pa_simple as *mut pa_simple;
static mut pa_error: libc::c_int = 0;
unsafe extern "C" fn help() {
    printf(
        b"    -a server           set the server name\n    -s sink             set the output sink\n    -n name             set the application name, as seen by PulseAudio\n                            defaults to the access point name\n\0"
            as *const u8 as *const libc::c_char,
    );
}
unsafe extern "C" fn init(
    mut argc: libc::c_int,
    mut argv: *mut *mut libc::c_char,
) -> libc::c_int {
    let mut pa_server: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut pa_sink: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut pa_appname: *mut libc::c_char = config.apname;
    optind = 1 as libc::c_int;
    argv = argv.offset(-1);
    argv;
    argc += 1;
    argc;
    let mut opt: libc::c_int = 0;
    loop {
        opt = getopt(argc, argv, b"a:s:n:\0" as *const u8 as *const libc::c_char);
        if !(opt > 0 as libc::c_int) {
            break;
        }
        match opt {
            97 => {
                pa_server = optarg;
            }
            115 => {
                pa_sink = optarg;
            }
            110 => {
                pa_appname = optarg;
            }
            _ => {
                help();
                die(
                    b"Invalid audio option -%c specified\0" as *const u8
                        as *const libc::c_char as *mut libc::c_char,
                    opt,
                );
            }
        }
    }
    if optind < argc {
        die(
            b"Invalid audio argument: %s\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
            *argv.offset(optind as isize),
        );
    }
    static mut ss: pa_sample_spec = {
        let mut init = pa_sample_spec {
            format: PA_SAMPLE_S16LE,
            rate: 44100 as libc::c_int as uint32_t,
            channels: 2 as libc::c_int as uint8_t,
        };
        init
    };
    pa_dev = pa_simple_new(
        pa_server,
        pa_appname,
        PA_STREAM_PLAYBACK,
        pa_sink,
        b"Shairport Stream\0" as *const u8 as *const libc::c_char,
        &ss,
        0 as *const pa_channel_map,
        0 as *const pa_buffer_attr,
        &mut pa_error,
    );
    if pa_dev.is_null() {
        die(
            b"Could not connect to pulseaudio server: %s\0" as *const u8
                as *const libc::c_char as *mut libc::c_char,
            pa_strerror(pa_error),
        );
    }
    return 0 as libc::c_int;
}
unsafe extern "C" fn deinit() {
    if !pa_dev.is_null() {
        pa_simple_free(pa_dev);
    }
    pa_dev = 0 as *mut pa_simple;
}
unsafe extern "C" fn start(mut sample_rate: libc::c_int) {
    if sample_rate != 44100 as libc::c_int {
        die(
            b"unexpected sample rate!\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
        );
    }
}
unsafe extern "C" fn play(mut buf: *mut libc::c_short, mut samples: libc::c_int) {
    if pa_simple_write(
        pa_dev,
        buf as *mut libc::c_char as *const libc::c_void,
        (samples as size_t).wrapping_mul(4 as libc::c_int as libc::c_ulong),
        &mut pa_error,
    ) < 0 as libc::c_int
    {
        fprintf(
            stderr,
            b"audio_pulse.c: pa_simple_write() failed: %s\n\0" as *const u8
                as *const libc::c_char,
            pa_strerror(pa_error),
        );
    }
}
unsafe extern "C" fn stop() {
    if pa_simple_drain(pa_dev, &mut pa_error) < 0 as libc::c_int {
        fprintf(
            stderr,
            b"audio_pulse.c: pa_simple_drain() failed: %s\n\0" as *const u8
                as *const libc::c_char,
            pa_strerror(pa_error),
        );
    }
}
pub static mut audio_pulse: audio_output = {
    let mut init = audio_output {
        help: Some(help as unsafe extern "C" fn() -> ()),
        name: b"pulse\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        init: Some(
            init
                as unsafe extern "C" fn(
                    libc::c_int,
                    *mut *mut libc::c_char,
                ) -> libc::c_int,
        ),
        deinit: Some(deinit as unsafe extern "C" fn() -> ()),
        start: Some(start as unsafe extern "C" fn(libc::c_int) -> ()),
        play: Some(play as unsafe extern "C" fn(*mut libc::c_short, libc::c_int) -> ()),
        stop: Some(stop as unsafe extern "C" fn() -> ()),
        volume: None,
    };
    init
};
