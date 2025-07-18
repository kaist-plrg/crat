use ::libc;
extern "C" {
    pub type _snd_pcm_hw_params;
    pub type _snd_pcm;
    pub type _snd_mixer;
    pub type _snd_mixer_class;
    pub type _snd_mixer_elem;
    pub type _snd_mixer_selem_id;
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
    fn strcmp(_: *const libc::c_char, _: *const libc::c_char) -> libc::c_int;
    fn strtol(
        _: *const libc::c_char,
        _: *mut *mut libc::c_char,
        _: libc::c_int,
    ) -> libc::c_long;
    fn snd_strerror(errnum: libc::c_int) -> *const libc::c_char;
    fn snd_pcm_open(
        pcm: *mut *mut snd_pcm_t,
        name: *const libc::c_char,
        stream: snd_pcm_stream_t,
        mode: libc::c_int,
    ) -> libc::c_int;
    fn snd_pcm_close(pcm: *mut snd_pcm_t) -> libc::c_int;
    fn snd_pcm_hw_params(
        pcm: *mut snd_pcm_t,
        params: *mut snd_pcm_hw_params_t,
    ) -> libc::c_int;
    fn snd_pcm_drain(pcm: *mut snd_pcm_t) -> libc::c_int;
    fn snd_pcm_writei(
        pcm: *mut snd_pcm_t,
        buffer: *const libc::c_void,
        size: snd_pcm_uframes_t,
    ) -> snd_pcm_sframes_t;
    fn snd_pcm_recover(
        pcm: *mut snd_pcm_t,
        err: libc::c_int,
        silent: libc::c_int,
    ) -> libc::c_int;
    fn snd_pcm_hw_params_any(
        pcm: *mut snd_pcm_t,
        params: *mut snd_pcm_hw_params_t,
    ) -> libc::c_int;
    fn snd_pcm_hw_params_sizeof() -> size_t;
    fn snd_pcm_hw_params_set_access(
        pcm: *mut snd_pcm_t,
        params: *mut snd_pcm_hw_params_t,
        _access: snd_pcm_access_t,
    ) -> libc::c_int;
    fn snd_pcm_hw_params_set_format(
        pcm: *mut snd_pcm_t,
        params: *mut snd_pcm_hw_params_t,
        val: snd_pcm_format_t,
    ) -> libc::c_int;
    fn snd_pcm_hw_params_set_channels(
        pcm: *mut snd_pcm_t,
        params: *mut snd_pcm_hw_params_t,
        val: libc::c_uint,
    ) -> libc::c_int;
    fn snd_pcm_hw_params_set_rate_near(
        pcm: *mut snd_pcm_t,
        params: *mut snd_pcm_hw_params_t,
        val: *mut libc::c_uint,
        dir: *mut libc::c_int,
    ) -> libc::c_int;
    fn snd_pcm_hw_params_set_period_size_near(
        pcm: *mut snd_pcm_t,
        params: *mut snd_pcm_hw_params_t,
        val: *mut snd_pcm_uframes_t,
        dir: *mut libc::c_int,
    ) -> libc::c_int;
    fn snd_mixer_open(mixer: *mut *mut snd_mixer_t, mode: libc::c_int) -> libc::c_int;
    fn snd_mixer_close(mixer: *mut snd_mixer_t) -> libc::c_int;
    fn snd_mixer_attach(
        mixer: *mut snd_mixer_t,
        name: *const libc::c_char,
    ) -> libc::c_int;
    fn snd_mixer_load(mixer: *mut snd_mixer_t) -> libc::c_int;
    fn snd_mixer_selem_register(
        mixer: *mut snd_mixer_t,
        options: *mut snd_mixer_selem_regopt,
        classp: *mut *mut snd_mixer_class_t,
    ) -> libc::c_int;
    fn snd_mixer_find_selem(
        mixer: *mut snd_mixer_t,
        id: *const snd_mixer_selem_id_t,
    ) -> *mut snd_mixer_elem_t;
    fn snd_mixer_selem_set_playback_volume_all(
        elem: *mut snd_mixer_elem_t,
        value: libc::c_long,
    ) -> libc::c_int;
    fn snd_mixer_selem_get_playback_volume_range(
        elem: *mut snd_mixer_elem_t,
        min: *mut libc::c_long,
        max: *mut libc::c_long,
    ) -> libc::c_int;
    fn snd_mixer_selem_id_sizeof() -> size_t;
    fn snd_mixer_selem_id_set_name(
        obj: *mut snd_mixer_selem_id_t,
        val: *const libc::c_char,
    );
    fn snd_mixer_selem_id_set_index(obj: *mut snd_mixer_selem_id_t, val: libc::c_uint);
    fn die(format: *mut libc::c_char, _: ...);
}
pub type size_t = libc::c_ulong;
pub type snd_pcm_hw_params_t = _snd_pcm_hw_params;
pub type _snd_pcm_stream = libc::c_uint;
pub const SND_PCM_STREAM_LAST: _snd_pcm_stream = 1;
pub const SND_PCM_STREAM_CAPTURE: _snd_pcm_stream = 1;
pub const SND_PCM_STREAM_PLAYBACK: _snd_pcm_stream = 0;
pub type snd_pcm_stream_t = _snd_pcm_stream;
pub type _snd_pcm_access = libc::c_uint;
pub const SND_PCM_ACCESS_LAST: _snd_pcm_access = 4;
pub const SND_PCM_ACCESS_RW_NONINTERLEAVED: _snd_pcm_access = 4;
pub const SND_PCM_ACCESS_RW_INTERLEAVED: _snd_pcm_access = 3;
pub const SND_PCM_ACCESS_MMAP_COMPLEX: _snd_pcm_access = 2;
pub const SND_PCM_ACCESS_MMAP_NONINTERLEAVED: _snd_pcm_access = 1;
pub const SND_PCM_ACCESS_MMAP_INTERLEAVED: _snd_pcm_access = 0;
pub type snd_pcm_access_t = _snd_pcm_access;
pub type _snd_pcm_format = libc::c_int;
pub const SND_PCM_FORMAT_U20: _snd_pcm_format = 27;
pub const SND_PCM_FORMAT_S20: _snd_pcm_format = 25;
pub const SND_PCM_FORMAT_IEC958_SUBFRAME: _snd_pcm_format = 18;
pub const SND_PCM_FORMAT_FLOAT64: _snd_pcm_format = 16;
pub const SND_PCM_FORMAT_FLOAT: _snd_pcm_format = 14;
pub const SND_PCM_FORMAT_U32: _snd_pcm_format = 12;
pub const SND_PCM_FORMAT_S32: _snd_pcm_format = 10;
pub const SND_PCM_FORMAT_U24: _snd_pcm_format = 8;
pub const SND_PCM_FORMAT_S24: _snd_pcm_format = 6;
pub const SND_PCM_FORMAT_U16: _snd_pcm_format = 4;
pub const SND_PCM_FORMAT_S16: _snd_pcm_format = 2;
pub const SND_PCM_FORMAT_LAST: _snd_pcm_format = 52;
pub const SND_PCM_FORMAT_DSD_U32_BE: _snd_pcm_format = 52;
pub const SND_PCM_FORMAT_DSD_U16_BE: _snd_pcm_format = 51;
pub const SND_PCM_FORMAT_DSD_U32_LE: _snd_pcm_format = 50;
pub const SND_PCM_FORMAT_DSD_U16_LE: _snd_pcm_format = 49;
pub const SND_PCM_FORMAT_DSD_U8: _snd_pcm_format = 48;
pub const SND_PCM_FORMAT_G723_40_1B: _snd_pcm_format = 47;
pub const SND_PCM_FORMAT_G723_40: _snd_pcm_format = 46;
pub const SND_PCM_FORMAT_G723_24_1B: _snd_pcm_format = 45;
pub const SND_PCM_FORMAT_G723_24: _snd_pcm_format = 44;
pub const SND_PCM_FORMAT_U18_3BE: _snd_pcm_format = 43;
pub const SND_PCM_FORMAT_U18_3LE: _snd_pcm_format = 42;
pub const SND_PCM_FORMAT_S18_3BE: _snd_pcm_format = 41;
pub const SND_PCM_FORMAT_S18_3LE: _snd_pcm_format = 40;
pub const SND_PCM_FORMAT_U20_3BE: _snd_pcm_format = 39;
pub const SND_PCM_FORMAT_U20_3LE: _snd_pcm_format = 38;
pub const SND_PCM_FORMAT_S20_3BE: _snd_pcm_format = 37;
pub const SND_PCM_FORMAT_S20_3LE: _snd_pcm_format = 36;
pub const SND_PCM_FORMAT_U24_3BE: _snd_pcm_format = 35;
pub const SND_PCM_FORMAT_U24_3LE: _snd_pcm_format = 34;
pub const SND_PCM_FORMAT_S24_3BE: _snd_pcm_format = 33;
pub const SND_PCM_FORMAT_S24_3LE: _snd_pcm_format = 32;
pub const SND_PCM_FORMAT_SPECIAL: _snd_pcm_format = 31;
pub const SND_PCM_FORMAT_U20_BE: _snd_pcm_format = 28;
pub const SND_PCM_FORMAT_U20_LE: _snd_pcm_format = 27;
pub const SND_PCM_FORMAT_S20_BE: _snd_pcm_format = 26;
pub const SND_PCM_FORMAT_S20_LE: _snd_pcm_format = 25;
pub const SND_PCM_FORMAT_GSM: _snd_pcm_format = 24;
pub const SND_PCM_FORMAT_MPEG: _snd_pcm_format = 23;
pub const SND_PCM_FORMAT_IMA_ADPCM: _snd_pcm_format = 22;
pub const SND_PCM_FORMAT_A_LAW: _snd_pcm_format = 21;
pub const SND_PCM_FORMAT_MU_LAW: _snd_pcm_format = 20;
pub const SND_PCM_FORMAT_IEC958_SUBFRAME_BE: _snd_pcm_format = 19;
pub const SND_PCM_FORMAT_IEC958_SUBFRAME_LE: _snd_pcm_format = 18;
pub const SND_PCM_FORMAT_FLOAT64_BE: _snd_pcm_format = 17;
pub const SND_PCM_FORMAT_FLOAT64_LE: _snd_pcm_format = 16;
pub const SND_PCM_FORMAT_FLOAT_BE: _snd_pcm_format = 15;
pub const SND_PCM_FORMAT_FLOAT_LE: _snd_pcm_format = 14;
pub const SND_PCM_FORMAT_U32_BE: _snd_pcm_format = 13;
pub const SND_PCM_FORMAT_U32_LE: _snd_pcm_format = 12;
pub const SND_PCM_FORMAT_S32_BE: _snd_pcm_format = 11;
pub const SND_PCM_FORMAT_S32_LE: _snd_pcm_format = 10;
pub const SND_PCM_FORMAT_U24_BE: _snd_pcm_format = 9;
pub const SND_PCM_FORMAT_U24_LE: _snd_pcm_format = 8;
pub const SND_PCM_FORMAT_S24_BE: _snd_pcm_format = 7;
pub const SND_PCM_FORMAT_S24_LE: _snd_pcm_format = 6;
pub const SND_PCM_FORMAT_U16_BE: _snd_pcm_format = 5;
pub const SND_PCM_FORMAT_U16_LE: _snd_pcm_format = 4;
pub const SND_PCM_FORMAT_S16_BE: _snd_pcm_format = 3;
pub const SND_PCM_FORMAT_S16_LE: _snd_pcm_format = 2;
pub const SND_PCM_FORMAT_U8: _snd_pcm_format = 1;
pub const SND_PCM_FORMAT_S8: _snd_pcm_format = 0;
pub const SND_PCM_FORMAT_UNKNOWN: _snd_pcm_format = -1;
pub type snd_pcm_format_t = _snd_pcm_format;
pub type snd_pcm_uframes_t = libc::c_ulong;
pub type snd_pcm_sframes_t = libc::c_long;
pub type snd_pcm_t = _snd_pcm;
pub type snd_mixer_t = _snd_mixer;
pub type snd_mixer_class_t = _snd_mixer_class;
pub type snd_mixer_elem_t = _snd_mixer_elem;
pub type snd_mixer_selem_regopt_abstract = libc::c_uint;
pub const SND_MIXER_SABSTRACT_BASIC: snd_mixer_selem_regopt_abstract = 1;
pub const SND_MIXER_SABSTRACT_NONE: snd_mixer_selem_regopt_abstract = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct snd_mixer_selem_regopt {
    pub ver: libc::c_int,
    pub abstract_0: snd_mixer_selem_regopt_abstract,
    pub device: *const libc::c_char,
    pub playback_pcm: *mut snd_pcm_t,
    pub capture_pcm: *mut snd_pcm_t,
}
pub type snd_mixer_selem_id_t = _snd_mixer_selem_id;
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
pub static mut audio_alsa: audio_output = {
    let mut init = audio_output {
        help: Some(help as unsafe extern "C" fn() -> ()),
        name: b"alsa\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
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
static mut alsa_handle: *mut snd_pcm_t = 0 as *const snd_pcm_t as *mut snd_pcm_t;
static mut alsa_params: *mut snd_pcm_hw_params_t = 0 as *const snd_pcm_hw_params_t
    as *mut snd_pcm_hw_params_t;
static mut alsa_mix_handle: *mut snd_mixer_t = 0 as *const snd_mixer_t
    as *mut snd_mixer_t;
static mut alsa_mix_elem: *mut snd_mixer_elem_t = 0 as *const snd_mixer_elem_t
    as *mut snd_mixer_elem_t;
static mut alsa_mix_sid: *mut snd_mixer_selem_id_t = 0 as *const snd_mixer_selem_id_t
    as *mut snd_mixer_selem_id_t;
static mut alsa_mix_minv: libc::c_long = 0;
static mut alsa_mix_range: libc::c_long = 0;
static mut alsa_out_dev: *mut libc::c_char = b"default\0" as *const u8
    as *const libc::c_char as *mut libc::c_char;
static mut alsa_mix_dev: *mut libc::c_char = 0 as *const libc::c_char
    as *mut libc::c_char;
static mut alsa_mix_ctrl: *mut libc::c_char = b"Master\0" as *const u8
    as *const libc::c_char as *mut libc::c_char;
static mut alsa_mix_index: libc::c_int = 0 as libc::c_int;
unsafe extern "C" fn help() {
    printf(
        b"    -d output-device    set the output device [default*|...]\n    -t mixer-type       set the mixer type [software*|hardware]\n    -m mixer-device     set the mixer device ['output-device'*|...]\n    -c mixer-control    set the mixer control [Master*|...]\n    -i mixer-index      set the mixer index [0*|...]\n    *) default option\n\0"
            as *const u8 as *const libc::c_char,
    );
}
unsafe extern "C" fn init(
    mut argc: libc::c_int,
    mut argv: *mut *mut libc::c_char,
) -> libc::c_int {
    let mut hardware_mixer: libc::c_int = 0 as libc::c_int;
    optind = 1 as libc::c_int;
    argv = argv.offset(-1);
    argv;
    argc += 1;
    argc;
    let mut opt: libc::c_int = 0;
    loop {
        opt = getopt(argc, argv, b"d:t:m:c:i:\0" as *const u8 as *const libc::c_char);
        if !(opt > 0 as libc::c_int) {
            break;
        }
        match opt {
            100 => {
                alsa_out_dev = optarg;
            }
            116 => {
                if strcmp(optarg, b"hardware\0" as *const u8 as *const libc::c_char)
                    == 0 as libc::c_int
                {
                    hardware_mixer = 1 as libc::c_int;
                }
            }
            109 => {
                alsa_mix_dev = optarg;
            }
            99 => {
                alsa_mix_ctrl = optarg;
            }
            105 => {
                alsa_mix_index = strtol(
                    optarg,
                    0 as *mut *mut libc::c_char,
                    10 as libc::c_int,
                ) as libc::c_int;
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
    if hardware_mixer == 0 {
        return 0 as libc::c_int;
    }
    if alsa_mix_dev.is_null() {
        alsa_mix_dev = alsa_out_dev;
    }
    audio_alsa.volume = Some(volume as unsafe extern "C" fn(libc::c_double) -> ());
    let mut ret: libc::c_int = 0 as libc::c_int;
    let mut alsa_mix_maxv: libc::c_long = 0;
    let mut fresh0 = ::std::vec::from_elem(0, snd_mixer_selem_id_sizeof() as usize);
    alsa_mix_sid = fresh0.leak().as_mut_ptr() as *mut snd_mixer_selem_id_t;
    memset(
        alsa_mix_sid as *mut libc::c_void,
        0 as libc::c_int,
        snd_mixer_selem_id_sizeof(),
    );
    snd_mixer_selem_id_set_index(alsa_mix_sid, alsa_mix_index as libc::c_uint);
    snd_mixer_selem_id_set_name(alsa_mix_sid, alsa_mix_ctrl);
    if snd_mixer_open(&mut alsa_mix_handle, 0 as libc::c_int) < 0 as libc::c_int {
        die(
            b"Failed to open mixer\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
        );
    }
    if snd_mixer_attach(alsa_mix_handle, alsa_mix_dev) < 0 as libc::c_int {
        die(
            b"Failed to attach mixer\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
        );
    }
    if snd_mixer_selem_register(
        alsa_mix_handle,
        0 as *mut snd_mixer_selem_regopt,
        0 as *mut *mut snd_mixer_class_t,
    ) < 0 as libc::c_int
    {
        die(
            b"Failed to register mixer element\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
        );
    }
    ret = snd_mixer_load(alsa_mix_handle);
    if ret < 0 as libc::c_int {
        die(
            b"Failed to load mixer element\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
        );
    }
    alsa_mix_elem = snd_mixer_find_selem(alsa_mix_handle, alsa_mix_sid);
    if alsa_mix_elem.is_null() {
        die(
            b"Failed to find mixer element\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
        );
    }
    snd_mixer_selem_get_playback_volume_range(
        alsa_mix_elem,
        &mut alsa_mix_minv,
        &mut alsa_mix_maxv,
    );
    alsa_mix_range = alsa_mix_maxv - alsa_mix_minv;
    return 0 as libc::c_int;
}
unsafe extern "C" fn deinit() {
    stop();
    if !alsa_mix_handle.is_null() {
        snd_mixer_close(alsa_mix_handle);
    }
}
unsafe extern "C" fn start(mut sample_rate: libc::c_int) {
    if sample_rate != 44100 as libc::c_int {
        die(
            b"Unexpected sample rate!\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
        );
    }
    let mut ret: libc::c_int = 0;
    let mut dir: libc::c_int = 0 as libc::c_int;
    let mut frames: snd_pcm_uframes_t = 64 as libc::c_int as snd_pcm_uframes_t;
    ret = snd_pcm_open(
        &mut alsa_handle,
        alsa_out_dev,
        SND_PCM_STREAM_PLAYBACK,
        0 as libc::c_int,
    );
    if ret < 0 as libc::c_int {
        die(
            b"Alsa initialization failed: unable to open pcm device: %s\n\0" as *const u8
                as *const libc::c_char as *mut libc::c_char,
            snd_strerror(ret),
        );
    }
    let mut fresh1 = ::std::vec::from_elem(0, snd_pcm_hw_params_sizeof() as usize);
    alsa_params = fresh1.leak().as_mut_ptr() as *mut snd_pcm_hw_params_t;
    memset(
        alsa_params as *mut libc::c_void,
        0 as libc::c_int,
        snd_pcm_hw_params_sizeof(),
    );
    snd_pcm_hw_params_any(alsa_handle, alsa_params);
    snd_pcm_hw_params_set_access(
        alsa_handle,
        alsa_params,
        SND_PCM_ACCESS_RW_INTERLEAVED,
    );
    snd_pcm_hw_params_set_format(alsa_handle, alsa_params, SND_PCM_FORMAT_S16);
    snd_pcm_hw_params_set_channels(
        alsa_handle,
        alsa_params,
        2 as libc::c_int as libc::c_uint,
    );
    snd_pcm_hw_params_set_rate_near(
        alsa_handle,
        alsa_params,
        &mut sample_rate as *mut libc::c_int as *mut libc::c_uint,
        &mut dir,
    );
    snd_pcm_hw_params_set_period_size_near(
        alsa_handle,
        alsa_params,
        &mut frames,
        &mut dir,
    );
    ret = snd_pcm_hw_params(alsa_handle, alsa_params);
    if ret < 0 as libc::c_int {
        die(
            b"unable to set hw parameters: %s\n\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
            snd_strerror(ret),
        );
    }
}
unsafe extern "C" fn play(mut buf: *mut libc::c_short, mut samples: libc::c_int) {
    let mut err: libc::c_int = snd_pcm_writei(
        alsa_handle,
        buf as *mut libc::c_char as *const libc::c_void,
        samples as snd_pcm_uframes_t,
    ) as libc::c_int;
    if err < 0 as libc::c_int {
        err = snd_pcm_recover(alsa_handle, err, 0 as libc::c_int);
    }
    if err < 0 as libc::c_int {
        die(
            b"Failed to write to PCM device: %s\n\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
            snd_strerror(err),
        );
    }
}
unsafe extern "C" fn stop() {
    if !alsa_handle.is_null() {
        snd_pcm_drain(alsa_handle);
        snd_pcm_close(alsa_handle);
        alsa_handle = 0 as *mut snd_pcm_t;
    }
}
unsafe extern "C" fn volume(mut vol: libc::c_double) {
    let mut alsa_volume: libc::c_long = (vol * alsa_mix_range as libc::c_double
        + alsa_mix_minv as libc::c_double) as libc::c_long;
    if snd_mixer_selem_set_playback_volume_all(alsa_mix_elem, alsa_volume)
        != 0 as libc::c_int
    {
        die(
            b"Failed to set playback volume\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
        );
    }
}
