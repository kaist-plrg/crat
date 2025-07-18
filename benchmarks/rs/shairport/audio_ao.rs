use ::libc;
extern "C" {
    pub type ao_device;
    fn printf(_: *const libc::c_char, _: ...) -> libc::c_int;
    static mut optarg: *mut libc::c_char;
    static mut optind: libc::c_int;
    fn getopt(
        ___argc: libc::c_int,
        ___argv: *const *mut libc::c_char,
        __shortopts: *const libc::c_char,
    ) -> libc::c_int;
    fn memset(
        _: *mut libc::c_void,
        _: libc::c_int,
        _: libc::c_ulong,
    ) -> *mut libc::c_void;
    fn strchr(_: *const libc::c_char, _: libc::c_int) -> *mut libc::c_char;
    fn ao_default_driver_id() -> libc::c_int;
    fn ao_driver_id(short_name: *const libc::c_char) -> libc::c_int;
    fn ao_close(device: *mut ao_device) -> libc::c_int;
    fn ao_play(
        device: *mut ao_device,
        output_samples: *mut libc::c_char,
        num_bytes: uint_32,
    ) -> libc::c_int;
    fn ao_open_live(
        driver_id: libc::c_int,
        format: *mut ao_sample_format,
        option: *mut ao_option,
    ) -> *mut ao_device;
    fn ao_append_option(
        options: *mut *mut ao_option,
        key: *const libc::c_char,
        value: *const libc::c_char,
    ) -> libc::c_int;
    fn ao_shutdown();
    fn ao_initialize();
    fn die(format: *mut libc::c_char, _: ...);
}
pub type uint_32 = libc::c_uint;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ao_sample_format {
    pub bits: libc::c_int,
    pub rate: libc::c_int,
    pub channels: libc::c_int,
    pub byte_format: libc::c_int,
    pub matrix: *mut libc::c_char,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ao_option {
    pub key: *mut libc::c_char,
    pub value: *mut libc::c_char,
    pub next: *mut ao_option,
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
pub static mut dev: *mut ao_device = 0 as *const ao_device as *mut ao_device;
unsafe extern "C" fn help() {
    printf(
        b"    -d driver           set the output driver\n    -o name=value       set an arbitrary ao option\n    -i id               shorthand for -o id=<id>\n    -n name             shorthand for -o dev=<name> -o dsp=<name>\n\0"
            as *const u8 as *const libc::c_char,
    );
}
unsafe extern "C" fn init(
    mut argc: libc::c_int,
    mut argv: *mut *mut libc::c_char,
) -> libc::c_int {
    ao_initialize();
    let mut driver: libc::c_int = ao_default_driver_id();
    let mut ao_opts: *mut ao_option = 0 as *mut ao_option;
    optind = 1 as libc::c_int;
    argv = argv.offset(-1);
    argv;
    argc += 1;
    argc;
    let mut opt: libc::c_int = 0;
    let mut mid: *mut libc::c_char = 0 as *mut libc::c_char;
    loop {
        opt = getopt(argc, argv, b"d:i:n:o:\0" as *const u8 as *const libc::c_char);
        if !(opt > 0 as libc::c_int) {
            break;
        }
        match opt {
            100 => {
                driver = ao_driver_id(optarg);
                if driver < 0 as libc::c_int {
                    die(
                        b"could not find ao driver %s\0" as *const u8
                            as *const libc::c_char as *mut libc::c_char,
                        optarg,
                    );
                }
            }
            105 => {
                ao_append_option(
                    &mut ao_opts,
                    b"id\0" as *const u8 as *const libc::c_char,
                    optarg,
                );
            }
            110 => {
                ao_append_option(
                    &mut ao_opts,
                    b"dev\0" as *const u8 as *const libc::c_char,
                    optarg,
                );
                ao_append_option(
                    &mut ao_opts,
                    b"dsp\0" as *const u8 as *const libc::c_char,
                    optarg,
                );
            }
            111 => {
                mid = strchr(optarg, '=' as i32);
                if mid.is_null() {
                    die(
                        b"Expected an = in audio option %s\0" as *const u8
                            as *const libc::c_char as *mut libc::c_char,
                        optarg,
                    );
                }
                *mid = 0 as libc::c_int as libc::c_char;
                ao_append_option(
                    &mut ao_opts,
                    optarg,
                    mid.offset(1 as libc::c_int as isize),
                );
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
    let mut fmt: ao_sample_format = ao_sample_format {
        bits: 0,
        rate: 0,
        channels: 0,
        byte_format: 0,
        matrix: 0 as *mut libc::c_char,
    };
    memset(
        &mut fmt as *mut ao_sample_format as *mut libc::c_void,
        0 as libc::c_int,
        ::std::mem::size_of::<ao_sample_format>() as libc::c_ulong,
    );
    fmt.bits = 16 as libc::c_int;
    fmt.rate = 44100 as libc::c_int;
    fmt.channels = 2 as libc::c_int;
    fmt.byte_format = 4 as libc::c_int;
    dev = ao_open_live(driver, &mut fmt, ao_opts);
    return if !dev.is_null() { 0 as libc::c_int } else { 1 as libc::c_int };
}
unsafe extern "C" fn deinit() {
    if !dev.is_null() {
        ao_close(dev);
    }
    dev = 0 as *mut ao_device;
    ao_shutdown();
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
    ao_play(dev, buf as *mut libc::c_char, (samples * 4 as libc::c_int) as uint_32);
}
unsafe extern "C" fn stop() {}
pub static mut audio_ao: audio_output = {
    let mut init = audio_output {
        help: Some(help as unsafe extern "C" fn() -> ()),
        name: b"ao\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
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
