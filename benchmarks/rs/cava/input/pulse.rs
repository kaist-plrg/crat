use ::libc;
extern "C" {
    pub type pa_simple;
    pub type pa_mainloop;
    pub type _IO_wide_data;
    pub type _IO_codecvt;
    pub type _IO_marker;
    pub type pa_context;
    pub type pa_operation;
    pub type pa_defer_event;
    pub type pa_time_event;
    pub type pa_io_event;
    fn malloc(_: libc::c_ulong) -> *mut libc::c_void;
    fn free(__ptr: *mut libc::c_void);
    fn exit(_: libc::c_int) -> !;
    fn pthread_exit(__retval: *mut libc::c_void) -> !;
    fn strcpy(_: *mut libc::c_char, _: *const libc::c_char) -> *mut libc::c_char;
    fn strcat(_: *mut libc::c_char, _: *const libc::c_char) -> *mut libc::c_char;
    fn fprintf(_: *mut FILE, _: *const libc::c_char, _: ...) -> libc::c_int;
    static mut stderr: *mut FILE;
    fn sprintf(_: *mut libc::c_char, _: *const libc::c_char, _: ...) -> libc::c_int;
    fn write_to_cava_input_buffers(
        size: int16_t,
        buf: *mut int16_t,
        data: *mut libc::c_void,
    ) -> libc::c_int;
    fn pthread_mutex_unlock(__mutex: *mut pthread_mutex_t) -> libc::c_int;
    fn pthread_mutex_lock(__mutex: *mut pthread_mutex_t) -> libc::c_int;
    fn pa_strerror(error: libc::c_int) -> *const libc::c_char;
    fn pa_context_new(
        mainloop: *mut pa_mainloop_api,
        name: *const libc::c_char,
    ) -> *mut pa_context;
    fn pa_mainloop_get_api(m: *mut pa_mainloop) -> *mut pa_mainloop_api;
    fn pa_mainloop_new() -> *mut pa_mainloop;
    fn pa_context_connect(
        c: *mut pa_context,
        server: *const libc::c_char,
        flags: pa_context_flags_t,
        api: *const pa_spawn_api,
    ) -> libc::c_int;
    fn pa_context_set_state_callback(
        c: *mut pa_context,
        cb_0: pa_context_notify_cb_t,
        userdata: *mut libc::c_void,
    );
    fn pa_context_get_state(c: *const pa_context) -> pa_context_state_t;
    fn pa_operation_unref(o: *mut pa_operation);
    fn pa_context_get_server_info(
        c: *mut pa_context,
        cb_0: pa_server_info_cb_t,
        userdata: *mut libc::c_void,
    ) -> *mut pa_operation;
    fn pa_context_disconnect(c: *mut pa_context);
    fn pa_context_unref(c: *mut pa_context);
    fn pa_mainloop_free(m: *mut pa_mainloop);
    fn pa_mainloop_quit(m: *mut pa_mainloop, retval: libc::c_int);
    fn pa_mainloop_iterate(
        m: *mut pa_mainloop,
        block: libc::c_int,
        retval: *mut libc::c_int,
    ) -> libc::c_int;
    fn pa_mainloop_run(m: *mut pa_mainloop, retval: *mut libc::c_int) -> libc::c_int;
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
    fn pa_simple_read(
        s: *mut pa_simple,
        data: *mut libc::c_void,
        bytes: size_t,
        error: *mut libc::c_int,
    ) -> libc::c_int;
}
pub type int16_t = __int16_t;
pub type __int16_t = libc::c_short;
pub type uint16_t = __uint16_t;
pub type __uint16_t = libc::c_ushort;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct audio_data {
    pub cava_in: *mut libc::c_double,
    pub input_buffer_size: libc::c_int,
    pub cava_buffer_size: libc::c_int,
    pub format: libc::c_int,
    pub rate: libc::c_uint,
    pub channels: libc::c_uint,
    pub source: *mut libc::c_char,
    pub im: libc::c_int,
    pub terminate: libc::c_int,
    pub error_message: [libc::c_char; 1024],
    pub samples_counter: libc::c_int,
    pub lock: pthread_mutex_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union pthread_mutex_t {
    pub __data: __pthread_mutex_s,
    pub __size: [libc::c_char; 40],
    pub __align: libc::c_long,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct __pthread_mutex_s {
    pub __lock: libc::c_int,
    pub __count: libc::c_uint,
    pub __owner: libc::c_int,
    pub __nusers: libc::c_uint,
    pub __kind: libc::c_int,
    pub __spins: libc::c_short,
    pub __elision: libc::c_short,
    pub __list: __pthread_list_t,
}
pub type __pthread_list_t = __pthread_internal_list;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct __pthread_internal_list {
    pub __prev: *mut __pthread_internal_list,
    pub __next: *mut __pthread_internal_list,
}
pub type size_t = libc::c_ulong;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct pa_buffer_attr {
    pub maxlength: uint32_t,
    pub tlength: uint32_t,
    pub prebuf: uint32_t,
    pub minreq: uint32_t,
    pub fragsize: uint32_t,
}
pub type uint32_t = __uint32_t;
pub type __uint32_t = libc::c_uint;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct pa_channel_map {
    pub channels: uint8_t,
    pub map: [pa_channel_position_t; 32],
}
pub type pa_channel_position_t = pa_channel_position;
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
pub type uint8_t = __uint8_t;
pub type __uint8_t = libc::c_uchar;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct pa_sample_spec {
    pub format: pa_sample_format_t,
    pub rate: uint32_t,
    pub channels: uint8_t,
}
pub type pa_sample_format_t = pa_sample_format;
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
pub type pa_stream_direction_t = pa_stream_direction;
pub type pa_stream_direction = libc::c_uint;
pub const PA_STREAM_UPLOAD: pa_stream_direction = 3;
pub const PA_STREAM_RECORD: pa_stream_direction = 2;
pub const PA_STREAM_PLAYBACK: pa_stream_direction = 1;
pub const PA_STREAM_NODIRECTION: pa_stream_direction = 0;
pub type FILE = _IO_FILE;
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
pub type __off64_t = libc::c_long;
pub type _IO_lock_t = ();
pub type __off_t = libc::c_long;
pub const PA_CONTEXT_TERMINATED: pa_context_state = 6;
pub const PA_CONTEXT_FAILED: pa_context_state = 5;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct pa_server_info {
    pub user_name: *const libc::c_char,
    pub host_name: *const libc::c_char,
    pub server_version: *const libc::c_char,
    pub server_name: *const libc::c_char,
    pub sample_spec: pa_sample_spec,
    pub default_sink_name: *const libc::c_char,
    pub default_source_name: *const libc::c_char,
    pub cookie: uint32_t,
    pub channel_map: pa_channel_map,
}
pub type pa_server_info_cb_t = Option::<
    unsafe extern "C" fn(*mut pa_context, *const pa_server_info, *mut libc::c_void) -> (),
>;
pub const PA_CONTEXT_READY: pa_context_state = 4;
pub const PA_CONTEXT_SETTING_NAME: pa_context_state = 3;
pub const PA_CONTEXT_AUTHORIZING: pa_context_state = 2;
pub const PA_CONTEXT_CONNECTING: pa_context_state = 1;
pub const PA_CONTEXT_UNCONNECTED: pa_context_state = 0;
pub type pa_context_state_t = pa_context_state;
pub type pa_context_state = libc::c_uint;
pub type pa_context_notify_cb_t = Option::<
    unsafe extern "C" fn(*mut pa_context, *mut libc::c_void) -> (),
>;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct pa_spawn_api {
    pub prefork: Option::<unsafe extern "C" fn() -> ()>,
    pub postfork: Option::<unsafe extern "C" fn() -> ()>,
    pub atfork: Option::<unsafe extern "C" fn() -> ()>,
}
pub type pa_context_flags_t = pa_context_flags;
pub type pa_context_flags = libc::c_uint;
pub const PA_CONTEXT_NOFAIL: pa_context_flags = 2;
pub const PA_CONTEXT_NOAUTOSPAWN: pa_context_flags = 1;
pub const PA_CONTEXT_NOFLAGS: pa_context_flags = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct pa_mainloop_api {
    pub userdata: *mut libc::c_void,
    pub io_new: Option::<
        unsafe extern "C" fn(
            *mut pa_mainloop_api,
            libc::c_int,
            pa_io_event_flags_t,
            pa_io_event_cb_t,
            *mut libc::c_void,
        ) -> *mut pa_io_event,
    >,
    pub io_enable: Option::<
        unsafe extern "C" fn(*mut pa_io_event, pa_io_event_flags_t) -> (),
    >,
    pub io_free: Option::<unsafe extern "C" fn(*mut pa_io_event) -> ()>,
    pub io_set_destroy: Option::<
        unsafe extern "C" fn(*mut pa_io_event, pa_io_event_destroy_cb_t) -> (),
    >,
    pub time_new: Option::<
        unsafe extern "C" fn(
            *mut pa_mainloop_api,
            *const timeval,
            pa_time_event_cb_t,
            *mut libc::c_void,
        ) -> *mut pa_time_event,
    >,
    pub time_restart: Option::<
        unsafe extern "C" fn(*mut pa_time_event, *const timeval) -> (),
    >,
    pub time_free: Option::<unsafe extern "C" fn(*mut pa_time_event) -> ()>,
    pub time_set_destroy: Option::<
        unsafe extern "C" fn(*mut pa_time_event, pa_time_event_destroy_cb_t) -> (),
    >,
    pub defer_new: Option::<
        unsafe extern "C" fn(
            *mut pa_mainloop_api,
            pa_defer_event_cb_t,
            *mut libc::c_void,
        ) -> *mut pa_defer_event,
    >,
    pub defer_enable: Option::<
        unsafe extern "C" fn(*mut pa_defer_event, libc::c_int) -> (),
    >,
    pub defer_free: Option::<unsafe extern "C" fn(*mut pa_defer_event) -> ()>,
    pub defer_set_destroy: Option::<
        unsafe extern "C" fn(*mut pa_defer_event, pa_defer_event_destroy_cb_t) -> (),
    >,
    pub quit: Option::<unsafe extern "C" fn(*mut pa_mainloop_api, libc::c_int) -> ()>,
}
pub type pa_defer_event_destroy_cb_t = Option::<
    unsafe extern "C" fn(
        *mut pa_mainloop_api,
        *mut pa_defer_event,
        *mut libc::c_void,
    ) -> (),
>;
pub type pa_defer_event_cb_t = Option::<
    unsafe extern "C" fn(
        *mut pa_mainloop_api,
        *mut pa_defer_event,
        *mut libc::c_void,
    ) -> (),
>;
pub type pa_time_event_destroy_cb_t = Option::<
    unsafe extern "C" fn(
        *mut pa_mainloop_api,
        *mut pa_time_event,
        *mut libc::c_void,
    ) -> (),
>;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct timeval {
    pub tv_sec: __time_t,
    pub tv_usec: __suseconds_t,
}
pub type __suseconds_t = libc::c_long;
pub type __time_t = libc::c_long;
pub type pa_time_event_cb_t = Option::<
    unsafe extern "C" fn(
        *mut pa_mainloop_api,
        *mut pa_time_event,
        *const timeval,
        *mut libc::c_void,
    ) -> (),
>;
pub type pa_io_event_destroy_cb_t = Option::<
    unsafe extern "C" fn(*mut pa_mainloop_api, *mut pa_io_event, *mut libc::c_void) -> (),
>;
pub type pa_io_event_flags_t = pa_io_event_flags;
pub type pa_io_event_flags = libc::c_uint;
pub const PA_IO_EVENT_ERROR: pa_io_event_flags = 8;
pub const PA_IO_EVENT_HANGUP: pa_io_event_flags = 4;
pub const PA_IO_EVENT_OUTPUT: pa_io_event_flags = 2;
pub const PA_IO_EVENT_INPUT: pa_io_event_flags = 1;
pub const PA_IO_EVENT_NULL: pa_io_event_flags = 0;
pub type pa_io_event_cb_t = Option::<
    unsafe extern "C" fn(
        *mut pa_mainloop_api,
        *mut pa_io_event,
        libc::c_int,
        pa_io_event_flags_t,
        *mut libc::c_void,
    ) -> (),
>;
pub static mut m_pulseaudio_mainloop: *mut pa_mainloop = 0 as *const pa_mainloop
    as *mut pa_mainloop;
pub unsafe extern "C" fn cb(
    mut pulseaudio_context: *mut pa_context,
    mut i: *const pa_server_info,
    mut userdata: *mut libc::c_void,
) {
    let mut audio: *mut audio_data = userdata as *mut audio_data;
    pthread_mutex_lock(&mut (*audio).lock);
    free((*audio).source as *mut libc::c_void);
    (*audio)
        .source = malloc(
        (::std::mem::size_of::<libc::c_char>() as libc::c_ulong)
            .wrapping_mul(1024 as libc::c_int as libc::c_ulong),
    ) as *mut libc::c_char;
    strcpy((*audio).source, (*i).default_sink_name);
    (*audio)
        .source = strcat(
        (*audio).source,
        b".monitor\0" as *const u8 as *const libc::c_char,
    );
    pthread_mutex_unlock(&mut (*audio).lock);
    pa_context_disconnect(pulseaudio_context);
    pa_context_unref(pulseaudio_context);
    pa_mainloop_quit(m_pulseaudio_mainloop, 0 as libc::c_int);
    pa_mainloop_free(m_pulseaudio_mainloop);
}
pub unsafe extern "C" fn pulseaudio_context_state_callback(
    mut pulseaudio_context: *mut pa_context,
    mut userdata: *mut libc::c_void,
) {
    match pa_context_get_state(pulseaudio_context) as libc::c_uint {
        1 => {}
        2 => {}
        3 => {}
        4 => {
            pa_operation_unref(
                pa_context_get_server_info(
                    pulseaudio_context,
                    Some(
                        cb
                            as unsafe extern "C" fn(
                                *mut pa_context,
                                *const pa_server_info,
                                *mut libc::c_void,
                            ) -> (),
                    ),
                    userdata,
                ),
            );
        }
        5 => {
            fprintf(
                stderr,
                b"failed to connect to pulseaudio server\n\0" as *const u8
                    as *const libc::c_char,
            );
            exit(1 as libc::c_int);
        }
        6 => {
            pa_mainloop_quit(m_pulseaudio_mainloop, 0 as libc::c_int);
        }
        0 | _ => {}
    };
}
pub unsafe extern "C" fn getPulseDefaultSink(mut data: *mut libc::c_void) {
    let mut audio: *mut audio_data = data as *mut audio_data;
    let mut mainloop_api: *mut pa_mainloop_api = 0 as *mut pa_mainloop_api;
    let mut pulseaudio_context: *mut pa_context = 0 as *mut pa_context;
    let mut ret: libc::c_int = 0;
    m_pulseaudio_mainloop = pa_mainloop_new();
    mainloop_api = pa_mainloop_get_api(m_pulseaudio_mainloop);
    pulseaudio_context = pa_context_new(
        mainloop_api,
        b"cava device list\0" as *const u8 as *const libc::c_char,
    );
    pa_context_connect(
        pulseaudio_context,
        0 as *const libc::c_char,
        PA_CONTEXT_NOFLAGS,
        0 as *const pa_spawn_api,
    );
    pa_context_set_state_callback(
        pulseaudio_context,
        Some(
            pulseaudio_context_state_callback
                as unsafe extern "C" fn(*mut pa_context, *mut libc::c_void) -> (),
        ),
        audio as *mut libc::c_void,
    );
    ret = pa_mainloop_iterate(m_pulseaudio_mainloop, 0 as libc::c_int, &mut ret);
    if ret == 0 {
        fprintf(
            stderr,
            b"Could not open pulseaudio mainloop to find default device name: %d\ncheck if pulseaudio is running\n\0"
                as *const u8 as *const libc::c_char,
            ret,
        );
        exit(1 as libc::c_int);
    }
    pa_mainloop_run(m_pulseaudio_mainloop, &mut ret);
}
pub unsafe extern "C" fn input_pulse(mut data: *mut libc::c_void) -> *mut libc::c_void {
    let mut audio: *mut audio_data = data as *mut audio_data;
    let mut frames: uint16_t = ((*audio).input_buffer_size / 2 as libc::c_int)
        as uint16_t;
    let mut channels: libc::c_int = 2 as libc::c_int;
    let vla = (frames as libc::c_int * channels) as usize;
    let mut buf: Vec::<int16_t> = ::std::vec::from_elem(0, vla);
    static mut ss: pa_sample_spec = {
        let mut init = pa_sample_spec {
            format: PA_SAMPLE_S16LE,
            rate: 44100 as libc::c_int as uint32_t,
            channels: 2 as libc::c_int as uint8_t,
        };
        init
    };
    (*audio).format = 16 as libc::c_int;
    let frag_size: libc::c_int = frames as libc::c_int * channels * (*audio).format
        / 8 as libc::c_int * 2 as libc::c_int;
    let mut pb: pa_buffer_attr = {
        let mut init = pa_buffer_attr {
            maxlength: -(1 as libc::c_int) as uint32_t,
            tlength: 0,
            prebuf: 0,
            minreq: 0,
            fragsize: frag_size as uint32_t,
        };
        init
    };
    let mut s: *mut pa_simple = 0 as *mut pa_simple;
    let mut error: libc::c_int = 0;
    s = pa_simple_new(
        0 as *const libc::c_char,
        b"cava\0" as *const u8 as *const libc::c_char,
        PA_STREAM_RECORD,
        (*audio).source,
        b"audio for cava\0" as *const u8 as *const libc::c_char,
        &ss,
        0 as *const pa_channel_map,
        &mut pb,
        &mut error,
    );
    if s.is_null() {
        sprintf(
            ((*audio).error_message).as_mut_ptr(),
            b"input/pulse.c: Could not open pulseaudio source: %s, %s. \t\tTo find a list of your pulseaudio sources run 'pacmd list-sources'\n\0"
                as *const u8 as *const libc::c_char,
            (*audio).source,
            pa_strerror(error),
        );
        (*audio).terminate = 1 as libc::c_int;
        pthread_exit(0 as *mut libc::c_void);
    }
    while (*audio).terminate == 0 {
        if pa_simple_read(
            s,
            buf.as_mut_ptr() as *mut libc::c_void,
            (vla * ::std::mem::size_of::<int16_t>()) as libc::c_ulong,
            &mut error,
        ) < 0 as libc::c_int
        {
            sprintf(
                ((*audio).error_message).as_mut_ptr(),
                b"input/pulse.c: pa_simple_read() failed: %s\n\0" as *const u8
                    as *const libc::c_char,
                pa_strerror(error),
            );
            (*audio).terminate = 1 as libc::c_int;
        }
        write_to_cava_input_buffers(
            (frames as libc::c_int * channels) as int16_t,
            buf.as_mut_ptr(),
            data,
        );
    }
    pa_simple_free(s);
    pthread_exit(0 as *mut libc::c_void);
}
