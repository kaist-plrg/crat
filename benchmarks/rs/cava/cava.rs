use ::libc;
extern "C" {
    pub type _IO_wide_data;
    pub type _IO_codecvt;
    pub type _IO_marker;
    pub type __dirstream;
    pub type fftw_plan_s;
    pub type ldat;
    fn setlocale(
        __category: libc::c_int,
        __locale: *const libc::c_char,
    ) -> *mut libc::c_char;
    fn open(__file: *const libc::c_char, __oflag: libc::c_int, _: ...) -> libc::c_int;
    fn pow(_: libc::c_double, _: libc::c_double) -> libc::c_double;
    static mut stdout: *mut FILE;
    static mut stderr: *mut FILE;
    fn fclose(__stream: *mut FILE) -> libc::c_int;
    fn fopen(_: *const libc::c_char, _: *const libc::c_char) -> *mut FILE;
    fn fprintf(_: *mut FILE, _: *const libc::c_char, _: ...) -> libc::c_int;
    fn printf(_: *const libc::c_char, _: ...) -> libc::c_int;
    fn snprintf(
        _: *mut libc::c_char,
        _: libc::c_ulong,
        _: *const libc::c_char,
        _: ...
    ) -> libc::c_int;
    fn fileno(__stream: *mut FILE) -> libc::c_int;
    fn malloc(_: libc::c_ulong) -> *mut libc::c_void;
    fn free(__ptr: *mut libc::c_void);
    fn abort() -> !;
    fn exit(_: libc::c_int) -> !;
    fn getenv(__name: *const libc::c_char) -> *mut libc::c_char;
    fn system(__command: *const libc::c_char) -> libc::c_int;
    fn opendir(__name: *const libc::c_char) -> *mut DIR;
    fn closedir(__dirp: *mut DIR) -> libc::c_int;
    static mut optarg: *mut libc::c_char;
    fn getopt(
        ___argc: libc::c_int,
        ___argv: *const *mut libc::c_char,
        __shortopts: *const libc::c_char,
    ) -> libc::c_int;
    fn signal(__sig: libc::c_int, __handler: __sighandler_t) -> __sighandler_t;
    fn raise(__sig: libc::c_int) -> libc::c_int;
    fn sigaction(
        __sig: libc::c_int,
        __act: *const sigaction,
        __oact: *mut sigaction,
    ) -> libc::c_int;
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
    fn strcpy(_: *mut libc::c_char, _: *const libc::c_char) -> *mut libc::c_char;
    fn strcmp(_: *const libc::c_char, _: *const libc::c_char) -> libc::c_int;
    fn strncmp(
        _: *const libc::c_char,
        _: *const libc::c_char,
        _: libc::c_ulong,
    ) -> libc::c_int;
    fn strlen(_: *const libc::c_char) -> libc::c_ulong;
    fn mkfifo(__path: *const libc::c_char, __mode: __mode_t) -> libc::c_int;
    fn nanosleep(
        __requested_time: *const timespec,
        __remaining: *mut timespec,
    ) -> libc::c_int;
    fn access(__name: *const libc::c_char, __type: libc::c_int) -> libc::c_int;
    fn ttyname(__fd: libc::c_int) -> *mut libc::c_char;
    fn cava_init(
        number_of_bars: libc::c_int,
        rate: libc::c_uint,
        channels: libc::c_int,
        autosens: libc::c_int,
        noise_reduction: libc::c_double,
        low_cut_off: libc::c_int,
        high_cut_off: libc::c_int,
    ) -> *mut cava_plan;
    fn cava_execute(
        cava_in: *mut libc::c_double,
        new_samples: libc::c_int,
        cava_out: *mut libc::c_double,
        plan: *mut cava_plan,
    );
    fn cava_destroy(plan: *mut cava_plan);
    fn load_config(
        configPath: *mut libc::c_char,
        p_0: *mut config_params,
        colorsOnly: bool,
        error: *mut error_s,
    ) -> bool;
    fn init_terminal_ncurses(
        fg_color_string: *mut libc::c_char,
        bg_color_string: *mut libc::c_char,
        predef_fg_color: libc::c_int,
        predef_bg_color: libc::c_int,
        gradient: libc::c_int,
        gradient_count: libc::c_int,
        gradient_colors: *mut *mut libc::c_char,
        width: *mut libc::c_int,
        height: *mut libc::c_int,
    );
    fn draw_terminal_ncurses(
        is_tty: libc::c_int,
        terminal_height: libc::c_int,
        terminal_width: libc::c_int,
        bars_count: libc::c_int,
        bar_width: libc::c_int,
        bar_spacing: libc::c_int,
        rest: libc::c_int,
        bars: *const libc::c_int,
        previous_frame: *mut libc::c_int,
        gradient: libc::c_int,
        x_axis_info: libc::c_int,
    ) -> libc::c_int;
    fn cleanup_terminal_ncurses();
    fn mvprintw(
        _: libc::c_int,
        _: libc::c_int,
        _: *const libc::c_char,
        _: ...
    ) -> libc::c_int;
    fn wgetch(_: *mut WINDOW) -> libc::c_int;
    static mut stdscr: *mut WINDOW;
    fn print_ntk_out(
        bars_count: libc::c_int,
        fd: libc::c_int,
        bit_format: libc::c_int,
        bar_width: libc::c_int,
        bar_spacing: libc::c_int,
        bar_height: libc::c_int,
        f: *const libc::c_int,
    ) -> libc::c_int;
    fn print_raw_out(
        bars_count: libc::c_int,
        fd: libc::c_int,
        is_binary: libc::c_int,
        bit_format: libc::c_int,
        ascii_range: libc::c_int,
        bar_delim: libc::c_char,
        frame_delim: libc::c_char,
        f: *const libc::c_int,
    ) -> libc::c_int;
    fn init_terminal_noncurses(
        inAtty: libc::c_int,
        col: libc::c_int,
        bgcol: libc::c_int,
        w: libc::c_int,
        h: libc::c_int,
        bar_width: libc::c_int,
    ) -> libc::c_int;
    fn get_terminal_dim_noncurses(w: *mut libc::c_int, h: *mut libc::c_int);
    fn draw_terminal_noncurses(
        inAtty: libc::c_int,
        lines: libc::c_int,
        width: libc::c_int,
        number_of_bars: libc::c_int,
        bar_width: libc::c_int,
        bar_spacing: libc::c_int,
        rest: libc::c_int,
        bars: *mut libc::c_int,
        previous_frame: *mut libc::c_int,
        x_axis_info: libc::c_int,
    ) -> libc::c_int;
    fn cleanup_terminal_noncurses();
    fn input_alsa(data: *mut libc::c_void) -> *mut libc::c_void;
    fn pthread_create(
        __newthread: *mut pthread_t,
        __attr: *const pthread_attr_t,
        __start_routine: Option::<
            unsafe extern "C" fn(*mut libc::c_void) -> *mut libc::c_void,
        >,
        __arg: *mut libc::c_void,
    ) -> libc::c_int;
    fn pthread_join(
        __th: pthread_t,
        __thread_return: *mut *mut libc::c_void,
    ) -> libc::c_int;
    fn pthread_mutex_lock(__mutex: *mut pthread_mutex_t) -> libc::c_int;
    fn pthread_mutex_unlock(__mutex: *mut pthread_mutex_t) -> libc::c_int;
    fn input_fifo(data: *mut libc::c_void) -> *mut libc::c_void;
    fn input_portaudio(audiodata: *mut libc::c_void) -> *mut libc::c_void;
    fn input_pulse(data: *mut libc::c_void) -> *mut libc::c_void;
    fn getPulseDefaultSink(data: *mut libc::c_void);
    fn input_shmem(data: *mut libc::c_void) -> *mut libc::c_void;
}
pub type size_t = libc::c_ulong;
pub type __uint32_t = libc::c_uint;
pub type __uid_t = libc::c_uint;
pub type __mode_t = libc::c_uint;
pub type __off_t = libc::c_long;
pub type __off64_t = libc::c_long;
pub type __pid_t = libc::c_int;
pub type __clock_t = libc::c_long;
pub type __time_t = libc::c_long;
pub type __syscall_slong_t = libc::c_long;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct timespec {
    pub tv_sec: __time_t,
    pub tv_nsec: __syscall_slong_t,
}
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
pub type wchar_t = libc::c_int;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct __sigset_t {
    pub __val: [libc::c_ulong; 16],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct __pthread_internal_list {
    pub __prev: *mut __pthread_internal_list,
    pub __next: *mut __pthread_internal_list,
}
pub type __pthread_list_t = __pthread_internal_list;
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
pub type pthread_t = libc::c_ulong;
#[derive(Copy, Clone)]
#[repr(C)]
pub union pthread_attr_t {
    pub __size: [libc::c_char; 56],
    pub __align: libc::c_long,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union pthread_mutex_t {
    pub __data: __pthread_mutex_s,
    pub __size: [libc::c_char; 40],
    pub __align: libc::c_long,
}
pub type DIR = __dirstream;
#[derive(Copy, Clone)]
#[repr(C)]
pub union sigval {
    pub sival_int: libc::c_int,
    pub sival_ptr: *mut libc::c_void,
}
pub type __sigval_t = sigval;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct siginfo_t {
    pub si_signo: libc::c_int,
    pub si_errno: libc::c_int,
    pub si_code: libc::c_int,
    pub __pad0: libc::c_int,
    pub _sifields: C2RustUnnamed,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed {
    pub _pad: [libc::c_int; 28],
    pub _kill: C2RustUnnamed_8,
    pub _timer: C2RustUnnamed_7,
    pub _rt: C2RustUnnamed_6,
    pub _sigchld: C2RustUnnamed_5,
    pub _sigfault: C2RustUnnamed_2,
    pub _sigpoll: C2RustUnnamed_1,
    pub _sigsys: C2RustUnnamed_0,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_0 {
    pub _call_addr: *mut libc::c_void,
    pub _syscall: libc::c_int,
    pub _arch: libc::c_uint,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_1 {
    pub si_band: libc::c_long,
    pub si_fd: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_2 {
    pub si_addr: *mut libc::c_void,
    pub si_addr_lsb: libc::c_short,
    pub _bounds: C2RustUnnamed_3,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_3 {
    pub _addr_bnd: C2RustUnnamed_4,
    pub _pkey: __uint32_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_4 {
    pub _lower: *mut libc::c_void,
    pub _upper: *mut libc::c_void,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_5 {
    pub si_pid: __pid_t,
    pub si_uid: __uid_t,
    pub si_status: libc::c_int,
    pub si_utime: __clock_t,
    pub si_stime: __clock_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_6 {
    pub si_pid: __pid_t,
    pub si_uid: __uid_t,
    pub si_sigval: __sigval_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_7 {
    pub si_tid: libc::c_int,
    pub si_overrun: libc::c_int,
    pub si_sigval: __sigval_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_8 {
    pub si_pid: __pid_t,
    pub si_uid: __uid_t,
}
pub type __sighandler_t = Option::<unsafe extern "C" fn(libc::c_int) -> ()>;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct sigaction {
    pub __sigaction_handler: C2RustUnnamed_9,
    pub sa_mask: __sigset_t,
    pub sa_flags: libc::c_int,
    pub sa_restorer: Option::<unsafe extern "C" fn() -> ()>,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_9 {
    pub sa_handler: __sighandler_t,
    pub sa_sigaction: Option::<
        unsafe extern "C" fn(libc::c_int, *mut siginfo_t, *mut libc::c_void) -> (),
    >,
}
pub type uint32_t = __uint32_t;
pub type fftw_complex = [libc::c_double; 2];
pub type fftw_plan = *mut fftw_plan_s;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct cava_plan {
    pub FFTbassbufferSize: libc::c_int,
    pub FFTmidbufferSize: libc::c_int,
    pub FFTtreblebufferSize: libc::c_int,
    pub number_of_bars: libc::c_int,
    pub audio_channels: libc::c_int,
    pub input_buffer_size: libc::c_int,
    pub rate: libc::c_int,
    pub bass_cut_off_bar: libc::c_int,
    pub treble_cut_off_bar: libc::c_int,
    pub height: libc::c_int,
    pub sens_init: libc::c_int,
    pub autosens: libc::c_int,
    pub frame_skip: libc::c_int,
    pub sens: libc::c_double,
    pub g: libc::c_double,
    pub framerate: libc::c_double,
    pub average_max: libc::c_double,
    pub noise_reduction: libc::c_double,
    pub p_bass_l: fftw_plan,
    pub p_bass_r: fftw_plan,
    pub p_mid_l: fftw_plan,
    pub p_mid_r: fftw_plan,
    pub p_treble_l: fftw_plan,
    pub p_treble_r: fftw_plan,
    pub out_bass_l: *mut fftw_complex,
    pub out_bass_r: *mut fftw_complex,
    pub out_mid_l: *mut fftw_complex,
    pub out_mid_r: *mut fftw_complex,
    pub out_treble_l: *mut fftw_complex,
    pub out_treble_r: *mut fftw_complex,
    pub bass_multiplier: *mut libc::c_double,
    pub mid_multiplier: *mut libc::c_double,
    pub treble_multiplier: *mut libc::c_double,
    pub in_bass_r_raw: *mut libc::c_double,
    pub in_bass_l_raw: *mut libc::c_double,
    pub in_mid_r_raw: *mut libc::c_double,
    pub in_mid_l_raw: *mut libc::c_double,
    pub in_treble_r_raw: *mut libc::c_double,
    pub in_treble_l_raw: *mut libc::c_double,
    pub in_bass_r: *mut libc::c_double,
    pub in_bass_l: *mut libc::c_double,
    pub in_mid_r: *mut libc::c_double,
    pub in_mid_l: *mut libc::c_double,
    pub in_treble_r: *mut libc::c_double,
    pub in_treble_l: *mut libc::c_double,
    pub prev_cava_out: *mut libc::c_double,
    pub cava_mem: *mut libc::c_double,
    pub input_buffer: *mut libc::c_double,
    pub cava_peak: *mut libc::c_double,
    pub eq: *mut libc::c_double,
    pub cut_off_frequency: *mut libc::c_float,
    pub FFTbuffer_lower_cut_off: *mut libc::c_int,
    pub FFTbuffer_upper_cut_off: *mut libc::c_int,
    pub cava_fall: *mut libc::c_int,
}
pub type input_method = libc::c_uint;
pub const INPUT_MAX: input_method = 6;
pub const INPUT_SHMEM: input_method = 5;
pub const INPUT_SNDIO: input_method = 4;
pub const INPUT_PULSE: input_method = 3;
pub const INPUT_ALSA: input_method = 2;
pub const INPUT_PORTAUDIO: input_method = 1;
pub const INPUT_FIFO: input_method = 0;
pub type output_method = libc::c_uint;
pub const OUTPUT_NOT_SUPORTED: output_method = 5;
pub const OUTPUT_NORITAKE: output_method = 4;
pub const OUTPUT_SDL: output_method = 3;
pub const OUTPUT_RAW: output_method = 2;
pub const OUTPUT_NONCURSES: output_method = 1;
pub const OUTPUT_NCURSES: output_method = 0;
pub type mono_option = libc::c_uint;
pub const AVERAGE: mono_option = 2;
pub const RIGHT: mono_option = 1;
pub const LEFT: mono_option = 0;
pub type xaxis_scale = libc::c_uint;
pub const NOTE: xaxis_scale = 2;
pub const FREQUENCY: xaxis_scale = 1;
pub const NONE: xaxis_scale = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct config_params {
    pub color: *mut libc::c_char,
    pub bcolor: *mut libc::c_char,
    pub raw_target: *mut libc::c_char,
    pub audio_source: *mut libc::c_char,
    pub gradient_colors: *mut *mut libc::c_char,
    pub data_format: *mut libc::c_char,
    pub bar_delim: libc::c_char,
    pub frame_delim: libc::c_char,
    pub monstercat: libc::c_double,
    pub integral: libc::c_double,
    pub gravity: libc::c_double,
    pub ignore: libc::c_double,
    pub sens: libc::c_double,
    pub noise_reduction: libc::c_double,
    pub lower_cut_off: libc::c_uint,
    pub upper_cut_off: libc::c_uint,
    pub userEQ: *mut libc::c_double,
    pub input: input_method,
    pub output: output_method,
    pub xaxis: xaxis_scale,
    pub mono_opt: mono_option,
    pub userEQ_keys: libc::c_int,
    pub userEQ_enabled: libc::c_int,
    pub col: libc::c_int,
    pub bgcol: libc::c_int,
    pub autobars: libc::c_int,
    pub stereo: libc::c_int,
    pub raw_format: libc::c_int,
    pub ascii_range: libc::c_int,
    pub bit_format: libc::c_int,
    pub gradient: libc::c_int,
    pub gradient_count: libc::c_int,
    pub fixedbars: libc::c_int,
    pub framerate: libc::c_int,
    pub bar_width: libc::c_int,
    pub bar_spacing: libc::c_int,
    pub bar_height: libc::c_int,
    pub autosens: libc::c_int,
    pub overshoot: libc::c_int,
    pub waves: libc::c_int,
    pub fifoSample: libc::c_int,
    pub fifoSampleBits: libc::c_int,
    pub sleep_timer: libc::c_int,
    pub sdl_width: libc::c_int,
    pub sdl_height: libc::c_int,
    pub sdl_x: libc::c_int,
    pub sdl_y: libc::c_int,
    pub draw_and_quit: libc::c_int,
    pub zero_test: libc::c_int,
    pub non_zero_test: libc::c_int,
    pub reverse: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct error_s {
    pub message: [libc::c_char; 1024],
    pub length: libc::c_int,
}
pub type chtype = libc::c_uint;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _win_st {
    pub _cury: libc::c_short,
    pub _curx: libc::c_short,
    pub _maxy: libc::c_short,
    pub _maxx: libc::c_short,
    pub _begy: libc::c_short,
    pub _begx: libc::c_short,
    pub _flags: libc::c_short,
    pub _attrs: attr_t,
    pub _bkgd: chtype,
    pub _notimeout: bool,
    pub _clear: bool,
    pub _leaveok: bool,
    pub _scroll: bool,
    pub _idlok: bool,
    pub _idcok: bool,
    pub _immed: bool,
    pub _sync: bool,
    pub _use_keypad: bool,
    pub _delay: libc::c_int,
    pub _line: *mut ldat,
    pub _regtop: libc::c_short,
    pub _regbottom: libc::c_short,
    pub _parx: libc::c_int,
    pub _pary: libc::c_int,
    pub _parent: *mut WINDOW,
    pub _pad: pdat,
    pub _yoffset: libc::c_short,
    pub _bkgrnd: cchar_t,
    pub _color: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct cchar_t {
    pub attr: attr_t,
    pub chars: [wchar_t; 5],
    pub ext_color: libc::c_int,
}
pub type attr_t = chtype;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct pdat {
    pub _pad_y: libc::c_short,
    pub _pad_x: libc::c_short,
    pub _pad_top: libc::c_short,
    pub _pad_left: libc::c_short,
    pub _pad_bottom: libc::c_short,
    pub _pad_right: libc::c_short,
}
pub type WINDOW = _win_st;
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
pub static mut output_mode: libc::c_int = 0;
pub static mut should_reload: libc::c_int = 0 as libc::c_int;
pub static mut reload_colors: libc::c_int = 0 as libc::c_int;
pub static mut should_quit: libc::c_int = 0 as libc::c_int;
pub static mut p: config_params = config_params {
    color: 0 as *const libc::c_char as *mut libc::c_char,
    bcolor: 0 as *const libc::c_char as *mut libc::c_char,
    raw_target: 0 as *const libc::c_char as *mut libc::c_char,
    audio_source: 0 as *const libc::c_char as *mut libc::c_char,
    gradient_colors: 0 as *const *mut libc::c_char as *mut *mut libc::c_char,
    data_format: 0 as *const libc::c_char as *mut libc::c_char,
    bar_delim: 0,
    frame_delim: 0,
    monstercat: 0.,
    integral: 0.,
    gravity: 0.,
    ignore: 0.,
    sens: 0.,
    noise_reduction: 0.,
    lower_cut_off: 0,
    upper_cut_off: 0,
    userEQ: 0 as *const libc::c_double as *mut libc::c_double,
    input: INPUT_FIFO,
    output: OUTPUT_NCURSES,
    xaxis: NONE,
    mono_opt: LEFT,
    userEQ_keys: 0,
    userEQ_enabled: 0,
    col: 0,
    bgcol: 0,
    autobars: 0,
    stereo: 0,
    raw_format: 0,
    ascii_range: 0,
    bit_format: 0,
    gradient: 0,
    gradient_count: 0,
    fixedbars: 0,
    framerate: 0,
    bar_width: 0,
    bar_spacing: 0,
    bar_height: 0,
    autosens: 0,
    overshoot: 0,
    waves: 0,
    fifoSample: 0,
    fifoSampleBits: 0,
    sleep_timer: 0,
    sdl_width: 0,
    sdl_height: 0,
    sdl_x: 0,
    sdl_y: 0,
    draw_and_quit: 0,
    zero_test: 0,
    non_zero_test: 0,
    reverse: 0,
};
pub unsafe extern "C" fn cleanup() {
    if output_mode == OUTPUT_NCURSES as libc::c_int {
        cleanup_terminal_ncurses();
    } else if output_mode == OUTPUT_NONCURSES as libc::c_int {
        cleanup_terminal_noncurses();
    } else {
        output_mode == OUTPUT_SDL as libc::c_int;
    };
}
pub unsafe extern "C" fn sig_handler(mut sig_no: libc::c_int) {
    if sig_no == 10 as libc::c_int {
        should_reload = 1 as libc::c_int;
        return;
    }
    if sig_no == 12 as libc::c_int {
        reload_colors = 1 as libc::c_int;
        return;
    }
    cleanup();
    if sig_no == 2 as libc::c_int {
        printf(b"CTRL-C pressed -- goodbye\n\0" as *const u8 as *const libc::c_char);
    }
    signal(sig_no, None);
    raise(sig_no);
}
unsafe extern "C" fn is_loop_device_for_sure(mut text: *const libc::c_char) -> bool {
    let LOOPBACK_DEVICE_PREFIX: *const libc::c_char = b"hw:Loopback,\0" as *const u8
        as *const libc::c_char;
    return strncmp(text, LOOPBACK_DEVICE_PREFIX, strlen(LOOPBACK_DEVICE_PREFIX))
        == 0 as libc::c_int;
}
unsafe extern "C" fn directory_exists(mut path: *const libc::c_char) -> bool {
    let dir: *mut DIR = opendir(path);
    if dir.is_null() {
        return 0 as libc::c_int != 0;
    }
    closedir(dir);
    return 1 as libc::c_int != 0;
}
pub unsafe extern "C" fn monstercat_filter(
    mut bars: *mut libc::c_int,
    mut number_of_bars: libc::c_int,
    mut waves: libc::c_int,
    mut monstercat: libc::c_double,
) -> *mut libc::c_int {
    let mut z: libc::c_int = 0;
    let mut m_y: libc::c_int = 0;
    let mut de: libc::c_int = 0;
    if waves > 0 as libc::c_int {
        z = 0 as libc::c_int;
        while z < number_of_bars {
            *bars
                .offset(
                    z as isize,
                ) = (*bars.offset(z as isize) as libc::c_double / 1.25f64)
                as libc::c_int;
            m_y = z - 1 as libc::c_int;
            while m_y >= 0 as libc::c_int {
                de = z - m_y;
                *bars
                    .offset(
                        m_y as isize,
                    ) = ({
                    let mut _a: libc::c_double = *bars.offset(z as isize)
                        as libc::c_double
                        - pow(de as libc::c_double, 2 as libc::c_int as libc::c_double);
                    let mut _b: libc::c_int = *bars.offset(m_y as isize);
                    if _a > _b as libc::c_double { _a } else { _b as libc::c_double }
                }) as libc::c_int;
                m_y -= 1;
                m_y;
            }
            m_y = z + 1 as libc::c_int;
            while m_y < number_of_bars {
                de = m_y - z;
                *bars
                    .offset(
                        m_y as isize,
                    ) = ({
                    let mut _a: libc::c_double = *bars.offset(z as isize)
                        as libc::c_double
                        - pow(de as libc::c_double, 2 as libc::c_int as libc::c_double);
                    let mut _b: libc::c_int = *bars.offset(m_y as isize);
                    if _a > _b as libc::c_double { _a } else { _b as libc::c_double }
                }) as libc::c_int;
                m_y += 1;
                m_y;
            }
            z += 1;
            z;
        }
    } else if monstercat > 0 as libc::c_int as libc::c_double {
        z = 0 as libc::c_int;
        while z < number_of_bars {
            m_y = z - 1 as libc::c_int;
            while m_y >= 0 as libc::c_int {
                de = z - m_y;
                *bars
                    .offset(
                        m_y as isize,
                    ) = ({
                    let mut _a: libc::c_double = *bars.offset(z as isize)
                        as libc::c_double / pow(monstercat, de as libc::c_double);
                    let mut _b: libc::c_int = *bars.offset(m_y as isize);
                    if _a > _b as libc::c_double { _a } else { _b as libc::c_double }
                }) as libc::c_int;
                m_y -= 1;
                m_y;
            }
            m_y = z + 1 as libc::c_int;
            while m_y < number_of_bars {
                de = m_y - z;
                *bars
                    .offset(
                        m_y as isize,
                    ) = ({
                    let mut _a: libc::c_double = *bars.offset(z as isize)
                        as libc::c_double / pow(monstercat, de as libc::c_double);
                    let mut _b: libc::c_int = *bars.offset(m_y as isize);
                    if _a > _b as libc::c_double { _a } else { _b as libc::c_double }
                }) as libc::c_int;
                m_y += 1;
                m_y;
            }
            z += 1;
            z;
        }
    }
    return bars;
}
unsafe fn main_0(
    mut argc: libc::c_int,
    mut argv: *mut *mut libc::c_char,
) -> libc::c_int {
    printf(
        b"%c]0;%s%c\0" as *const u8 as *const libc::c_char,
        '\u{1b}' as i32,
        b"cava\0" as *const u8 as *const libc::c_char,
        '\u{7}' as i32,
    );
    let mut configPath: [libc::c_char; 4096] = [0; 4096];
    configPath[0 as libc::c_int as usize] = '\0' as i32 as libc::c_char;
    let mut action: sigaction = sigaction {
        __sigaction_handler: C2RustUnnamed_9 {
            sa_handler: None,
        },
        sa_mask: __sigset_t { __val: [0; 16] },
        sa_flags: 0,
        sa_restorer: None,
    };
    memset(
        &mut action as *mut sigaction as *mut libc::c_void,
        0 as libc::c_int,
        ::std::mem::size_of::<sigaction>() as libc::c_ulong,
    );
    action
        .__sigaction_handler
        .sa_handler = Some(sig_handler as unsafe extern "C" fn(libc::c_int) -> ());
    sigaction(2 as libc::c_int, &mut action, 0 as *mut sigaction);
    sigaction(15 as libc::c_int, &mut action, 0 as *mut sigaction);
    sigaction(10 as libc::c_int, &mut action, 0 as *mut sigaction);
    sigaction(12 as libc::c_int, &mut action, 0 as *mut sigaction);
    let mut usage: *mut libc::c_char = b"\nUsage : cava [options]\nVisualize audio input in terminal. \n\nOptions:\n\t-p          path to config file\n\t-v          print version\n\nKeys:\n        Up        Increase sensitivity\n        Down      Decrease sensitivity\n        Left      Decrease number of bars\n        Right     Increase number of bars\n        r         Reload config\n        c         Reload colors only\n        f         Cycle foreground color\n        b         Cycle background color\n        q         Quit\n\nas of 0.4.0 all options are specified in config file, see in '/home/username/.config/cava/' \n\0"
        as *const u8 as *const libc::c_char as *mut libc::c_char;
    let mut c: libc::c_int = 0;
    loop {
        c = getopt(argc, argv, b"p:vh\0" as *const u8 as *const libc::c_char);
        if !(c != -(1 as libc::c_int)) {
            break;
        }
        match c {
            112 => {
                snprintf(
                    configPath.as_mut_ptr(),
                    ::std::mem::size_of::<[libc::c_char; 4096]>() as libc::c_ulong,
                    b"%s\0" as *const u8 as *const libc::c_char,
                    optarg,
                );
            }
            104 => {
                printf(b"%s\0" as *const u8 as *const libc::c_char, usage);
                return 1 as libc::c_int;
            }
            63 => {
                printf(b"%s\0" as *const u8 as *const libc::c_char, usage);
                return 1 as libc::c_int;
            }
            118 => {
                printf(b"cava 0.8.0\n\0" as *const u8 as *const libc::c_char);
                return 0 as libc::c_int;
            }
            _ => {
                abort();
            }
        }
    }
    loop {
        let mut error: error_s = error_s {
            message: [0; 1024],
            length: 0,
        };
        error.length = 0 as libc::c_int;
        if !load_config(
            configPath.as_mut_ptr(),
            &mut p,
            0 as libc::c_int != 0,
            &mut error,
        ) {
            fprintf(
                stderr,
                b"Error loading config. %s\0" as *const u8 as *const libc::c_char,
                (error.message).as_mut_ptr(),
            );
            exit(1 as libc::c_int);
        }
        let mut inAtty: libc::c_int = 0;
        output_mode = p.output as libc::c_int;
        if output_mode != OUTPUT_RAW as libc::c_int
            && output_mode != OUTPUT_NORITAKE as libc::c_int
        {
            inAtty = 0 as libc::c_int;
            if strncmp(
                ttyname(0 as libc::c_int),
                b"/dev/tty\0" as *const u8 as *const libc::c_char,
                8 as libc::c_int as libc::c_ulong,
            ) == 0 as libc::c_int
                || strcmp(
                    ttyname(0 as libc::c_int),
                    b"/dev/console\0" as *const u8 as *const libc::c_char,
                ) == 0 as libc::c_int
            {
                inAtty = 1 as libc::c_int;
            }
            if strncmp(
                ttyname(0 as libc::c_int),
                b"/dev/ttys\0" as *const u8 as *const libc::c_char,
                9 as libc::c_int as libc::c_ulong,
            ) == 0 as libc::c_int
            {
                inAtty = 0 as libc::c_int;
            }
            if inAtty != 0 {
                let mut font_file: *mut FILE = 0 as *mut FILE;
                font_file = fopen(
                    b"/usr/local/share/consolefonts/cava.psf\0" as *const u8
                        as *const libc::c_char,
                    b"r\0" as *const u8 as *const libc::c_char,
                );
                if !font_file.is_null() {
                    fclose(font_file);
                    system(
                        b"setfont /usr/local/share/consolefonts/cava.psf  >/dev/null 2>&1\0"
                            as *const u8 as *const libc::c_char,
                    );
                } else {
                    system(
                        b"setfont cava.psf  >/dev/null 2>&1\0" as *const u8
                            as *const libc::c_char,
                    );
                }
                system(b"setterm -blank 0\0" as *const u8 as *const libc::c_char);
            }
            if (getenv(b"LANG\0" as *const u8 as *const libc::c_char)).is_null() {
                setlocale(
                    6 as libc::c_int,
                    b"en_US.utf8\0" as *const u8 as *const libc::c_char,
                );
            } else {
                setlocale(6 as libc::c_int, b"\0" as *const u8 as *const libc::c_char);
            }
        }
        let mut audio: audio_data = audio_data {
            cava_in: 0 as *mut libc::c_double,
            input_buffer_size: 0,
            cava_buffer_size: 0,
            format: 0,
            rate: 0,
            channels: 0,
            source: 0 as *mut libc::c_char,
            im: 0,
            terminate: 0,
            error_message: [0; 1024],
            samples_counter: 0,
            lock: pthread_mutex_t {
                __data: __pthread_mutex_s {
                    __lock: 0,
                    __count: 0,
                    __owner: 0,
                    __nusers: 0,
                    __kind: 0,
                    __spins: 0,
                    __elision: 0,
                    __list: __pthread_list_t {
                        __prev: 0 as *mut __pthread_internal_list,
                        __next: 0 as *mut __pthread_internal_list,
                    },
                },
            },
        };
        memset(
            &mut audio as *mut audio_data as *mut libc::c_void,
            0 as libc::c_int,
            ::std::mem::size_of::<audio_data>() as libc::c_ulong,
        );
        audio
            .source = malloc(
            (1 as libc::c_int as libc::c_ulong).wrapping_add(strlen(p.audio_source)),
        ) as *mut libc::c_char;
        strcpy(audio.source, p.audio_source);
        audio.format = -(1 as libc::c_int);
        audio.rate = 0 as libc::c_int as libc::c_uint;
        audio.samples_counter = 0 as libc::c_int;
        audio.channels = 2 as libc::c_int as libc::c_uint;
        audio
            .input_buffer_size = (1024 as libc::c_int as libc::c_uint)
            .wrapping_mul(audio.channels) as libc::c_int;
        audio.cava_buffer_size = audio.input_buffer_size * 4 as libc::c_int;
        audio
            .cava_in = malloc(
            (audio.cava_buffer_size as libc::c_ulong)
                .wrapping_mul(::std::mem::size_of::<libc::c_double>() as libc::c_ulong),
        ) as *mut libc::c_double;
        memset(
            audio.cava_in as *mut libc::c_void,
            0 as libc::c_int,
            (::std::mem::size_of::<libc::c_int>() as libc::c_ulong)
                .wrapping_mul(audio.cava_buffer_size as libc::c_ulong),
        );
        audio.terminate = 0 as libc::c_int;
        let mut bars_left: *mut libc::c_int = 0 as *mut libc::c_int;
        let mut bars_right: *mut libc::c_int = 0 as *mut libc::c_int;
        let mut cava_out: *mut libc::c_double = 0 as *mut libc::c_double;
        bars_left = malloc(
            (1024 as libc::c_int as libc::c_ulong)
                .wrapping_mul(::std::mem::size_of::<libc::c_int>() as libc::c_ulong),
        ) as *mut libc::c_int;
        bars_right = malloc(
            (1024 as libc::c_int as libc::c_ulong)
                .wrapping_mul(::std::mem::size_of::<libc::c_int>() as libc::c_ulong),
        ) as *mut libc::c_int;
        cava_out = malloc(
            ((1024 as libc::c_int as libc::c_uint).wrapping_mul(audio.channels)
                as libc::c_ulong)
                .wrapping_mul(::std::mem::size_of::<libc::c_double>() as libc::c_ulong),
        ) as *mut libc::c_double;
        let mut p_thread: pthread_t = 0;
        let mut timeout_counter: libc::c_int = 0 as libc::c_int;
        let mut total_bar_height: libc::c_int = 0 as libc::c_int;
        let mut timeout_timer: timespec = {
            let mut init = timespec {
                tv_sec: 0 as libc::c_int as __time_t,
                tv_nsec: 1000000 as libc::c_int as __syscall_slong_t,
            };
            init
        };
        let mut thr_id: libc::c_int = 0;
        match p.input as libc::c_uint {
            2 => {
                if is_loop_device_for_sure(audio.source) {
                    if directory_exists(b"/sys/\0" as *const u8 as *const libc::c_char) {
                        if !directory_exists(
                            b"/sys/module/snd_aloop/\0" as *const u8
                                as *const libc::c_char,
                        ) {
                            cleanup();
                            fprintf(
                                stderr,
                                b"Linux kernel module \"snd_aloop\" does not seem to  be loaded.\nMaybe run \"sudo modprobe snd_aloop\".\n\0"
                                    as *const u8 as *const libc::c_char,
                            );
                            exit(1 as libc::c_int);
                        }
                    }
                }
                thr_id = pthread_create(
                    &mut p_thread,
                    0 as *const pthread_attr_t,
                    Some(
                        input_alsa
                            as unsafe extern "C" fn(
                                *mut libc::c_void,
                            ) -> *mut libc::c_void,
                    ),
                    &mut audio as *mut audio_data as *mut libc::c_void,
                );
                timeout_counter = 0 as libc::c_int;
                while audio.format == -(1 as libc::c_int)
                    || audio.rate == 0 as libc::c_int as libc::c_uint
                {
                    nanosleep(&mut timeout_timer, 0 as *mut timespec);
                    timeout_counter += 1;
                    timeout_counter;
                    if timeout_counter > 2000 as libc::c_int {
                        cleanup();
                        fprintf(
                            stderr,
                            b"could not get rate and/or format, problems with audio thread? quiting...\n\0"
                                as *const u8 as *const libc::c_char,
                        );
                        exit(1 as libc::c_int);
                    }
                }
            }
            0 => {
                thr_id = pthread_create(
                    &mut p_thread,
                    0 as *const pthread_attr_t,
                    Some(
                        input_fifo
                            as unsafe extern "C" fn(
                                *mut libc::c_void,
                            ) -> *mut libc::c_void,
                    ),
                    &mut audio as *mut audio_data as *mut libc::c_void,
                );
                audio.rate = p.fifoSample as libc::c_uint;
                audio.format = p.fifoSampleBits;
            }
            3 => {
                if strcmp(audio.source, b"auto\0" as *const u8 as *const libc::c_char)
                    == 0 as libc::c_int
                {
                    getPulseDefaultSink(
                        &mut audio as *mut audio_data as *mut libc::c_void,
                    );
                }
                thr_id = pthread_create(
                    &mut p_thread,
                    0 as *const pthread_attr_t,
                    Some(
                        input_pulse
                            as unsafe extern "C" fn(
                                *mut libc::c_void,
                            ) -> *mut libc::c_void,
                    ),
                    &mut audio as *mut audio_data as *mut libc::c_void,
                );
                audio.rate = 44100 as libc::c_int as libc::c_uint;
            }
            5 => {
                thr_id = pthread_create(
                    &mut p_thread,
                    0 as *const pthread_attr_t,
                    Some(
                        input_shmem
                            as unsafe extern "C" fn(
                                *mut libc::c_void,
                            ) -> *mut libc::c_void,
                    ),
                    &mut audio as *mut audio_data as *mut libc::c_void,
                );
                timeout_counter = 0 as libc::c_int;
                while audio.rate == 0 as libc::c_int as libc::c_uint {
                    nanosleep(&mut timeout_timer, 0 as *mut timespec);
                    timeout_counter += 1;
                    timeout_counter;
                    if timeout_counter > 2000 as libc::c_int {
                        cleanup();
                        fprintf(
                            stderr,
                            b"could not get rate and/or format, problems with audio thread? quiting...\n\0"
                                as *const u8 as *const libc::c_char,
                        );
                        exit(1 as libc::c_int);
                    }
                }
            }
            1 => {
                thr_id = pthread_create(
                    &mut p_thread,
                    0 as *const pthread_attr_t,
                    Some(
                        input_portaudio
                            as unsafe extern "C" fn(
                                *mut libc::c_void,
                            ) -> *mut libc::c_void,
                    ),
                    &mut audio as *mut audio_data as *mut libc::c_void,
                );
                audio.rate = 44100 as libc::c_int as libc::c_uint;
            }
            _ => {
                exit(1 as libc::c_int);
            }
        }
        if p.upper_cut_off > (audio.rate).wrapping_div(2 as libc::c_int as libc::c_uint)
        {
            cleanup();
            fprintf(
                stderr,
                b"higher cuttoff frequency can't be higher than sample rate / 2\0"
                    as *const u8 as *const libc::c_char,
            );
            exit(1 as libc::c_int);
        }
        let mut bars: [libc::c_int; 1024] = [0; 1024];
        let mut previous_frame: [libc::c_int; 1024] = [0; 1024];
        let mut height: libc::c_int = 0;
        let mut lines: libc::c_int = 0;
        let mut width: libc::c_int = 0;
        let mut remainder: libc::c_int = 0;
        let mut fp: libc::c_int = 0;
        let mut reloadConf: bool = 0 as libc::c_int != 0;
        while !reloadConf {
            let mut n: libc::c_int = 0 as libc::c_int;
            while n < 1024 as libc::c_int {
                previous_frame[n as usize] = 0 as libc::c_int;
                bars[n as usize] = 0 as libc::c_int;
                *cava_out.offset(n as isize) = 0 as libc::c_int as libc::c_double;
                *cava_out
                    .offset(
                        (n + 1024 as libc::c_int) as isize,
                    ) = 0 as libc::c_int as libc::c_double;
                n += 1;
                n;
            }
            if p.xaxis as libc::c_uint == FREQUENCY as libc::c_int as libc::c_uint
                && p.bar_width < 4 as libc::c_int
            {
                p.bar_width = 4 as libc::c_int;
            }
            match output_mode {
                0 => {
                    init_terminal_ncurses(
                        p.color,
                        p.bcolor,
                        p.col,
                        p.bgcol,
                        p.gradient,
                        p.gradient_count,
                        p.gradient_colors,
                        &mut width,
                        &mut lines,
                    );
                    if p.xaxis as libc::c_uint != NONE as libc::c_int as libc::c_uint {
                        lines -= 1;
                        lines;
                    }
                    height = lines * 8 as libc::c_int;
                }
                1 => {
                    get_terminal_dim_noncurses(&mut width, &mut lines);
                    if p.xaxis as libc::c_uint != NONE as libc::c_int as libc::c_uint {
                        lines -= 1;
                        lines;
                    }
                    init_terminal_noncurses(
                        inAtty,
                        p.col,
                        p.bgcol,
                        width,
                        lines,
                        p.bar_width,
                    );
                    height = lines * 8 as libc::c_int;
                }
                2 | 4 => {
                    if strcmp(
                        p.raw_target,
                        b"/dev/stdout\0" as *const u8 as *const libc::c_char,
                    ) != 0 as libc::c_int
                    {
                        let mut fptest: libc::c_int = 0;
                        if access(p.raw_target, 0 as libc::c_int) != -(1 as libc::c_int)
                        {
                            fptest = open(
                                p.raw_target,
                                0 as libc::c_int | 0o4000 as libc::c_int,
                                0o644 as libc::c_int,
                            );
                            if fptest == -(1 as libc::c_int) {
                                fprintf(
                                    stderr,
                                    b"could not open file %s for writing\n\0" as *const u8
                                        as *const libc::c_char,
                                    p.raw_target,
                                );
                                exit(1 as libc::c_int);
                            }
                        } else {
                            printf(
                                b"creating fifo %s\n\0" as *const u8 as *const libc::c_char,
                                p.raw_target,
                            );
                            if mkfifo(p.raw_target, 0o664 as libc::c_int as __mode_t)
                                == -(1 as libc::c_int)
                            {
                                fprintf(
                                    stderr,
                                    b"could not create fifo %s\n\0" as *const u8
                                        as *const libc::c_char,
                                    p.raw_target,
                                );
                                exit(1 as libc::c_int);
                            }
                            fptest = open(
                                p.raw_target,
                                0 as libc::c_int | 0o4000 as libc::c_int,
                                0o644 as libc::c_int,
                            );
                        }
                        fp = open(
                            p.raw_target,
                            0o1 as libc::c_int | 0o4000 as libc::c_int
                                | 0o100 as libc::c_int,
                            0o644 as libc::c_int,
                        );
                    } else {
                        fp = fileno(stdout);
                    }
                    if fp == -(1 as libc::c_int) {
                        fprintf(
                            stderr,
                            b"could not open file %s for writing\n\0" as *const u8
                                as *const libc::c_char,
                            p.raw_target,
                        );
                        exit(1 as libc::c_int);
                    }
                    width = 1024 as libc::c_int;
                    if strcmp(
                        p.data_format,
                        b"ascii\0" as *const u8 as *const libc::c_char,
                    ) != 0 as libc::c_int
                    {
                        height = (pow(
                            2 as libc::c_int as libc::c_double,
                            p.bit_format as libc::c_double,
                        ) - 1 as libc::c_int as libc::c_double) as libc::c_int;
                    } else {
                        height = p.ascii_range;
                    }
                }
                _ => {
                    exit(1 as libc::c_int);
                }
            }
            if p.fixedbars != 0 {
                p.autobars = 0 as libc::c_int;
                if p.fixedbars * p.bar_width + p.fixedbars * p.bar_spacing
                    - p.bar_spacing > width
                {
                    p.autobars = 1 as libc::c_int;
                }
            }
            let mut number_of_bars: libc::c_int = p.fixedbars;
            if p.autobars == 1 as libc::c_int {
                number_of_bars = (width + p.bar_spacing) / (p.bar_width + p.bar_spacing);
            }
            if number_of_bars < 1 as libc::c_int {
                number_of_bars = 1 as libc::c_int;
            }
            if number_of_bars > 1024 as libc::c_int {
                number_of_bars = 1024 as libc::c_int;
            }
            let mut output_channels: libc::c_int = 1 as libc::c_int;
            if p.stereo != 0 {
                output_channels = 2 as libc::c_int;
                if number_of_bars % 2 as libc::c_int != 0 as libc::c_int {
                    number_of_bars -= 1;
                    number_of_bars;
                }
            }
            remainder = (width - number_of_bars * p.bar_width
                - number_of_bars * p.bar_spacing + p.bar_spacing) / 2 as libc::c_int;
            if remainder < 0 as libc::c_int {
                remainder = 0 as libc::c_int;
            }
            let mut plan: *mut cava_plan = cava_init(
                number_of_bars / output_channels,
                audio.rate,
                audio.channels as libc::c_int,
                p.autosens,
                p.noise_reduction,
                p.lower_cut_off as libc::c_int,
                p.upper_cut_off as libc::c_int,
            );
            let mut center_frequencies: [libc::c_double; 1024] = [0.; 1024];
            let mut x_axis_info: libc::c_int = 0 as libc::c_int;
            if p.xaxis as libc::c_uint != NONE as libc::c_int as libc::c_uint {
                x_axis_info = 1 as libc::c_int;
                let mut center_frequency: libc::c_double = 0.;
                if output_mode == OUTPUT_NONCURSES as libc::c_int {
                    printf(
                        b"\r\x1B[%dB\0" as *const u8 as *const libc::c_char,
                        lines + 1 as libc::c_int,
                    );
                    if remainder != 0 {
                        printf(
                            b"\x1B[%dC\0" as *const u8 as *const libc::c_char,
                            remainder,
                        );
                    }
                }
                let mut n_0: libc::c_int = 0 as libc::c_int;
                while n_0 < number_of_bars {
                    let mut upper_cut_off_frequency: libc::c_float = 10000 as libc::c_int
                        as libc::c_float;
                    if n_0 < number_of_bars - 1 as libc::c_int {
                        upper_cut_off_frequency = *((*plan).cut_off_frequency)
                            .offset((n_0 + 1 as libc::c_int) as isize);
                    }
                    center_frequencies[n_0
                        as usize] = pow(
                        (*((*plan).cut_off_frequency).offset(n_0 as isize)
                            * upper_cut_off_frequency) as libc::c_double,
                        0.5f64,
                    );
                    if p.stereo != 0 {
                        if n_0 < number_of_bars / 2 as libc::c_int {
                            center_frequency = center_frequencies[(number_of_bars
                                / 2 as libc::c_int - 1 as libc::c_int - n_0) as usize];
                        } else {
                            center_frequency = center_frequencies[(n_0
                                - number_of_bars / 2 as libc::c_int) as usize];
                        }
                    } else {
                        center_frequency = center_frequencies[n_0 as usize];
                    }
                    let mut freq_kilohz: libc::c_float = (center_frequency
                        / 1000 as libc::c_int as libc::c_double) as libc::c_float;
                    let mut freq_floor: libc::c_int = center_frequency as libc::c_int;
                    if output_mode == OUTPUT_NCURSES as libc::c_int {
                        if center_frequency < 1000 as libc::c_int as libc::c_double {
                            mvprintw(
                                lines,
                                n_0 * (p.bar_width + p.bar_spacing) + remainder,
                                b"%-4d\0" as *const u8 as *const libc::c_char,
                                freq_floor,
                            );
                        } else if center_frequency
                            > 1000 as libc::c_int as libc::c_double
                            && center_frequency < 10000 as libc::c_int as libc::c_double
                        {
                            mvprintw(
                                lines,
                                n_0 * (p.bar_width + p.bar_spacing) + remainder,
                                b"%.2f\0" as *const u8 as *const libc::c_char,
                                freq_kilohz as libc::c_double,
                            );
                        } else {
                            mvprintw(
                                lines,
                                n_0 * (p.bar_width + p.bar_spacing) + remainder,
                                b"%.1f\0" as *const u8 as *const libc::c_char,
                                freq_kilohz as libc::c_double,
                            );
                        }
                    } else if output_mode == OUTPUT_NONCURSES as libc::c_int {
                        if center_frequency < 1000 as libc::c_int as libc::c_double {
                            printf(
                                b"%-4d\0" as *const u8 as *const libc::c_char,
                                freq_floor,
                            );
                        } else if center_frequency
                            > 1000 as libc::c_int as libc::c_double
                            && center_frequency < 10000 as libc::c_int as libc::c_double
                        {
                            printf(
                                b"%.2f\0" as *const u8 as *const libc::c_char,
                                freq_kilohz as libc::c_double,
                            );
                        } else {
                            printf(
                                b"%.1f\0" as *const u8 as *const libc::c_char,
                                freq_kilohz as libc::c_double,
                            );
                        }
                        if n_0 < number_of_bars - 1 as libc::c_int {
                            printf(
                                b"\x1B[%dC\0" as *const u8 as *const libc::c_char,
                                p.bar_width + p.bar_spacing - 4 as libc::c_int,
                            );
                        }
                    }
                    n_0 += 1;
                    n_0;
                }
                printf(
                    b"\r\x1B[%dA\0" as *const u8 as *const libc::c_char,
                    lines + 1 as libc::c_int,
                );
            }
            let mut resizeTerminal: bool = 0 as libc::c_int != 0;
            let mut frame_time_msec: libc::c_int = (1 as libc::c_int as libc::c_float
                / p.framerate as libc::c_float * 1000 as libc::c_int as libc::c_float)
                as libc::c_int;
            let mut framerate_timer: timespec = {
                let mut init = timespec {
                    tv_sec: 0 as libc::c_int as __time_t,
                    tv_nsec: 0 as libc::c_int as __syscall_slong_t,
                };
                init
            };
            if p.framerate <= 1 as libc::c_int {
                framerate_timer
                    .tv_sec = (frame_time_msec * 1000 as libc::c_int) as __time_t;
            } else {
                framerate_timer.tv_sec = 0 as libc::c_int as __time_t;
                framerate_timer
                    .tv_nsec = (frame_time_msec as libc::c_double * 1e6f64)
                    as __syscall_slong_t;
            }
            let mut sleep_counter: libc::c_int = 0 as libc::c_int;
            let mut silence: bool = 0 as libc::c_int != 0;
            let mut ch: libc::c_char = '\0' as i32 as libc::c_char;
            let mut sleep_mode_timer: timespec = {
                let mut init = timespec {
                    tv_sec: 1 as libc::c_int as __time_t,
                    tv_nsec: 0 as libc::c_int as __syscall_slong_t,
                };
                init
            };
            let mut total_frames: libc::c_int = 0 as libc::c_int;
            while !resizeTerminal {
                if output_mode == OUTPUT_NCURSES as libc::c_int {
                    ch = wgetch(stdscr) as libc::c_char;
                }
                match ch as libc::c_int {
                    65 => {
                        p.sens = p.sens * 1.05f64;
                    }
                    66 => {
                        p.sens = p.sens * 0.95f64;
                    }
                    68 => {
                        p.bar_width += 1;
                        p.bar_width;
                        resizeTerminal = 1 as libc::c_int != 0;
                    }
                    67 => {
                        if p.bar_width > 1 as libc::c_int {
                            p.bar_width -= 1;
                            p.bar_width;
                        }
                        resizeTerminal = 1 as libc::c_int != 0;
                    }
                    114 => {
                        should_reload = 1 as libc::c_int;
                    }
                    99 => {
                        reload_colors = 1 as libc::c_int;
                    }
                    102 => {
                        if p.col < 7 as libc::c_int {
                            p.col += 1;
                            p.col;
                        } else {
                            p.col = 0 as libc::c_int;
                        }
                        resizeTerminal = 1 as libc::c_int != 0;
                    }
                    98 => {
                        if p.bgcol < 7 as libc::c_int {
                            p.bgcol += 1;
                            p.bgcol;
                        } else {
                            p.bgcol = 0 as libc::c_int;
                        }
                        resizeTerminal = 1 as libc::c_int != 0;
                    }
                    113 => {
                        should_reload = 1 as libc::c_int;
                        should_quit = 1 as libc::c_int;
                    }
                    _ => {}
                }
                if should_reload != 0 {
                    reloadConf = 1 as libc::c_int != 0;
                    resizeTerminal = 1 as libc::c_int != 0;
                    should_reload = 0 as libc::c_int;
                }
                if reload_colors != 0 {
                    let mut error_0: error_s = error_s {
                        message: [0; 1024],
                        length: 0,
                    };
                    error_0.length = 0 as libc::c_int;
                    if !load_config(
                        configPath.as_mut_ptr(),
                        &mut p as *mut config_params as *mut libc::c_void
                            as *mut config_params,
                        1 as libc::c_int != 0,
                        &mut error_0,
                    ) {
                        cleanup();
                        fprintf(
                            stderr,
                            b"Error loading config. %s\0" as *const u8
                                as *const libc::c_char,
                            (error_0.message).as_mut_ptr(),
                        );
                        exit(1 as libc::c_int);
                    }
                    resizeTerminal = 1 as libc::c_int != 0;
                    reload_colors = 0 as libc::c_int;
                }
                silence = 1 as libc::c_int != 0;
                let mut n_1: libc::c_int = 0 as libc::c_int;
                while n_1 < audio.input_buffer_size * 4 as libc::c_int {
                    if *(audio.cava_in).offset(n_1 as isize) != 0. {
                        silence = 0 as libc::c_int != 0;
                        break;
                    } else {
                        n_1 += 1;
                        n_1;
                    }
                }
                if output_mode != OUTPUT_SDL as libc::c_int {
                    if p.sleep_timer != 0 {
                        if silence as libc::c_int != 0
                            && sleep_counter <= p.framerate * p.sleep_timer
                        {
                            sleep_counter += 1;
                            sleep_counter;
                        } else if !silence {
                            sleep_counter = 0 as libc::c_int;
                        }
                        if sleep_counter > p.framerate * p.sleep_timer {
                            nanosleep(&mut sleep_mode_timer, 0 as *mut timespec);
                            continue;
                        }
                    }
                }
                pthread_mutex_lock(&mut audio.lock);
                cava_execute(audio.cava_in, audio.samples_counter, cava_out, plan);
                if audio.samples_counter > 0 as libc::c_int {
                    audio.samples_counter = 0 as libc::c_int;
                }
                pthread_mutex_unlock(&mut audio.lock);
                let mut n_2: uint32_t = 0 as libc::c_int as uint32_t;
                while n_2 < (number_of_bars as libc::c_uint).wrapping_mul(audio.channels)
                {
                    if p.autosens != 0 {
                        *cava_out.offset(n_2 as isize) *= height as libc::c_double;
                    } else {
                        *cava_out.offset(n_2 as isize) *= p.sens;
                    }
                    n_2 = n_2.wrapping_add(1);
                    n_2;
                }
                let mut n_3: uint32_t = 0 as libc::c_int as uint32_t;
                while n_3
                    < (number_of_bars as libc::c_uint)
                        .wrapping_mul(audio.channels)
                        .wrapping_div(output_channels as libc::c_uint)
                {
                    *bars_right
                        .offset(
                            n_3 as isize,
                        ) = *cava_out.offset(n_3 as isize) as libc::c_int;
                    if audio.channels == 2 as libc::c_int as libc::c_uint {
                        *bars_left
                            .offset(
                                n_3 as isize,
                            ) = *cava_out
                            .offset(
                                n_3
                                    .wrapping_add(
                                        (number_of_bars / output_channels) as libc::c_uint,
                                    ) as isize,
                            ) as libc::c_int;
                    }
                    n_3 = n_3.wrapping_add(1);
                    n_3;
                }
                if p.monstercat != 0. {
                    if p.stereo != 0 {
                        bars_left = monstercat_filter(
                            bars_left,
                            number_of_bars / 2 as libc::c_int,
                            p.waves,
                            p.monstercat,
                        );
                        bars_right = monstercat_filter(
                            bars_right,
                            number_of_bars / 2 as libc::c_int,
                            p.waves,
                            p.monstercat,
                        );
                    } else {
                        bars_left = monstercat_filter(
                            bars_left,
                            number_of_bars,
                            p.waves,
                            p.monstercat,
                        );
                    }
                }
                if audio.channels == 2 as libc::c_int as libc::c_uint {
                    if p.stereo != 0 {
                        let mut n_4: libc::c_int = 0 as libc::c_int;
                        while n_4 < number_of_bars {
                            if n_4 < number_of_bars / 2 as libc::c_int {
                                if p.reverse != 0 {
                                    bars[n_4 as usize] = *bars_left.offset(n_4 as isize);
                                } else {
                                    bars[n_4
                                        as usize] = *bars_left
                                        .offset(
                                            (number_of_bars / 2 as libc::c_int - n_4 - 1 as libc::c_int)
                                                as isize,
                                        );
                                }
                            } else if p.reverse != 0 {
                                bars[n_4
                                    as usize] = *bars_right
                                    .offset((number_of_bars - n_4 - 1 as libc::c_int) as isize);
                            } else {
                                bars[n_4
                                    as usize] = *bars_right
                                    .offset((n_4 - number_of_bars / 2 as libc::c_int) as isize);
                            }
                            n_4 += 1;
                            n_4;
                        }
                    } else {
                        let mut n_5: libc::c_int = 0 as libc::c_int;
                        while n_5 < number_of_bars {
                            if p.reverse != 0 {
                                if p.mono_opt as libc::c_uint
                                    == AVERAGE as libc::c_int as libc::c_uint
                                {
                                    bars[(number_of_bars - n_5 - 1 as libc::c_int)
                                        as usize] = (*bars_left.offset(n_5 as isize)
                                        + *bars_right.offset(n_5 as isize)) / 2 as libc::c_int;
                                } else if p.mono_opt as libc::c_uint
                                    == LEFT as libc::c_int as libc::c_uint
                                {
                                    bars[(number_of_bars - n_5 - 1 as libc::c_int)
                                        as usize] = *bars_left.offset(n_5 as isize);
                                } else if p.mono_opt as libc::c_uint
                                    == RIGHT as libc::c_int as libc::c_uint
                                {
                                    bars[(number_of_bars - n_5 - 1 as libc::c_int)
                                        as usize] = *bars_right.offset(n_5 as isize);
                                }
                            } else if p.mono_opt as libc::c_uint
                                == AVERAGE as libc::c_int as libc::c_uint
                            {
                                bars[n_5
                                    as usize] = (*bars_left.offset(n_5 as isize)
                                    + *bars_right.offset(n_5 as isize)) / 2 as libc::c_int;
                            } else if p.mono_opt as libc::c_uint
                                == LEFT as libc::c_int as libc::c_uint
                            {
                                bars[n_5 as usize] = *bars_left.offset(n_5 as isize);
                            } else if p.mono_opt as libc::c_uint
                                == RIGHT as libc::c_int as libc::c_uint
                            {
                                bars[n_5 as usize] = *bars_right.offset(n_5 as isize);
                            }
                            n_5 += 1;
                            n_5;
                        }
                    }
                } else {
                    let mut n_6: libc::c_int = 0 as libc::c_int;
                    while n_6 < number_of_bars {
                        bars[n_6 as usize] = *bars_left.offset(n_6 as isize);
                        n_6 += 1;
                        n_6;
                    }
                }
                let mut n_7: libc::c_int = 0 as libc::c_int;
                while n_7 < number_of_bars {
                    if output_mode != OUTPUT_RAW as libc::c_int
                        && output_mode != OUTPUT_NORITAKE as libc::c_int
                        && bars[n_7 as usize] < 1 as libc::c_int
                    {
                        bars[n_7 as usize] = 1 as libc::c_int;
                    }
                    n_7 += 1;
                    n_7;
                }
                let mut rc: libc::c_int = 0;
                match output_mode {
                    0 => {
                        rc = draw_terminal_ncurses(
                            inAtty,
                            lines,
                            width,
                            number_of_bars,
                            p.bar_width,
                            p.bar_spacing,
                            remainder,
                            bars.as_mut_ptr() as *const libc::c_int,
                            previous_frame.as_mut_ptr(),
                            p.gradient,
                            x_axis_info,
                        );
                    }
                    1 => {
                        rc = draw_terminal_noncurses(
                            inAtty,
                            lines,
                            width,
                            number_of_bars,
                            p.bar_width,
                            p.bar_spacing,
                            remainder,
                            bars.as_mut_ptr(),
                            previous_frame.as_mut_ptr(),
                            x_axis_info,
                        );
                    }
                    2 => {
                        rc = print_raw_out(
                            number_of_bars,
                            fp,
                            p.raw_format,
                            p.bit_format,
                            p.ascii_range,
                            p.bar_delim,
                            p.frame_delim,
                            bars.as_mut_ptr() as *const libc::c_int,
                        );
                    }
                    4 => {
                        rc = print_ntk_out(
                            number_of_bars,
                            fp,
                            p.bit_format,
                            p.bar_width,
                            p.bar_spacing,
                            p.bar_height,
                            bars.as_mut_ptr() as *const libc::c_int,
                        );
                    }
                    _ => {
                        exit(1 as libc::c_int);
                    }
                }
                if rc == -(1 as libc::c_int) {
                    resizeTerminal = 1 as libc::c_int != 0;
                }
                if rc == -(2 as libc::c_int) {
                    resizeTerminal = 1 as libc::c_int != 0;
                    reloadConf = 1 as libc::c_int != 0;
                    should_quit = 1 as libc::c_int;
                }
                memcpy(
                    previous_frame.as_mut_ptr() as *mut libc::c_void,
                    bars.as_mut_ptr() as *const libc::c_void,
                    (1024 as libc::c_int as libc::c_ulong)
                        .wrapping_mul(
                            ::std::mem::size_of::<libc::c_int>() as libc::c_ulong,
                        ),
                );
                if audio.terminate == 1 as libc::c_int {
                    cleanup();
                    fprintf(
                        stderr,
                        b"Audio thread exited unexpectedly. %s\n\0" as *const u8
                            as *const libc::c_char,
                        (audio.error_message).as_mut_ptr(),
                    );
                    exit(1 as libc::c_int);
                }
                if output_mode != OUTPUT_SDL as libc::c_int {
                    nanosleep(&mut framerate_timer, 0 as *mut timespec);
                }
                if !(p.draw_and_quit > 0 as libc::c_int) {
                    continue;
                }
                total_frames += 1;
                total_frames;
                if !(total_frames >= p.draw_and_quit) {
                    continue;
                }
                let mut n_8: libc::c_int = 0 as libc::c_int;
                while n_8 < number_of_bars {
                    if output_mode != OUTPUT_RAW as libc::c_int
                        && output_mode != OUTPUT_NORITAKE as libc::c_int
                        && bars[n_8 as usize] == 1 as libc::c_int
                    {
                        bars[n_8 as usize] = 0 as libc::c_int;
                    }
                    total_bar_height += bars[n_8 as usize];
                    n_8 += 1;
                    n_8;
                }
                resizeTerminal = 1 as libc::c_int != 0;
                reloadConf = 1 as libc::c_int != 0;
                should_quit = 1 as libc::c_int;
                break;
            }
            cava_destroy(plan);
            free(plan as *mut libc::c_void);
        }
        audio.terminate = 1 as libc::c_int;
        pthread_join(p_thread, 0 as *mut *mut libc::c_void);
        if p.userEQ_enabled != 0 {
            free(p.userEQ as *mut libc::c_void);
        }
        free(audio.source as *mut libc::c_void);
        free(audio.cava_in as *mut libc::c_void);
        free(cava_out as *mut libc::c_void);
        free(bars_left as *mut libc::c_void);
        free(bars_right as *mut libc::c_void);
        cleanup();
        if should_quit != 0 {
            if p.zero_test != 0 && total_bar_height > 0 as libc::c_int {
                fprintf(
                    stderr,
                    b"Test mode: expected total bar height to be zero, but was: %d\n\0"
                        as *const u8 as *const libc::c_char,
                    total_bar_height,
                );
                return 1 as libc::c_int;
            } else if p.non_zero_test != 0 && total_bar_height == 0 as libc::c_int {
                fprintf(
                    stderr,
                    b"Test mode: expected total bar height to be non-zero, but was zero\n\0"
                        as *const u8 as *const libc::c_char,
                );
                return 1 as libc::c_int;
            } else {
                return 0 as libc::c_int
            }
        }
    };
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
