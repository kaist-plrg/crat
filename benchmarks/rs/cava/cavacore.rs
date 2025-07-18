use ::libc;
extern "C" {
    pub type _IO_wide_data;
    pub type _IO_codecvt;
    pub type _IO_marker;
    pub type fftw_plan_s;
    fn fftw_execute(p: fftw_plan);
    fn fftw_plan_dft_r2c_1d(
        n: libc::c_int,
        in_0: *mut libc::c_double,
        out: *mut fftw_complex,
        flags: libc::c_uint,
    ) -> fftw_plan;
    fn fftw_destroy_plan(p: fftw_plan);
    fn fftw_alloc_real(n: size_t) -> *mut libc::c_double;
    fn fftw_alloc_complex(n: size_t) -> *mut fftw_complex;
    fn fftw_free(p: *mut libc::c_void);
    fn fprintf(_: *mut FILE, _: *const libc::c_char, _: ...) -> libc::c_int;
    static mut stderr: *mut FILE;
    fn cos(_: libc::c_double) -> libc::c_double;
    fn log10(_: libc::c_double) -> libc::c_double;
    fn log2(_: libc::c_double) -> libc::c_double;
    fn pow(_: libc::c_double, _: libc::c_double) -> libc::c_double;
    fn hypot(_: libc::c_double, _: libc::c_double) -> libc::c_double;
    fn malloc(_: libc::c_ulong) -> *mut libc::c_void;
    fn free(__ptr: *mut libc::c_void);
    fn exit(_: libc::c_int) -> !;
    fn memset(
        _: *mut libc::c_void,
        _: libc::c_int,
        _: libc::c_ulong,
    ) -> *mut libc::c_void;
}
pub type __uint16_t = libc::c_ushort;
pub type __off_t = libc::c_long;
pub type __off64_t = libc::c_long;
pub type uint16_t = __uint16_t;
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
pub unsafe extern "C" fn cava_init(
    mut number_of_bars: libc::c_int,
    mut rate: libc::c_uint,
    mut channels: libc::c_int,
    mut autosens: libc::c_int,
    mut noise_reduction: libc::c_double,
    mut low_cut_off: libc::c_int,
    mut high_cut_off: libc::c_int,
) -> *mut cava_plan {
    if channels < 1 as libc::c_int || channels > 2 as libc::c_int {
        fprintf(
            stderr,
            b"cava_init called with illegal number of channels: %d, number of channels supported are 1 and 2\n\0"
                as *const u8 as *const libc::c_char,
            channels,
        );
        exit(1 as libc::c_int);
    }
    if rate < 1 as libc::c_int as libc::c_uint
        || rate > 384000 as libc::c_int as libc::c_uint
    {
        fprintf(
            stderr,
            b"cava_init called with illegal sample rate: %d\n\0" as *const u8
                as *const libc::c_char,
            rate,
        );
        exit(1 as libc::c_int);
    }
    let mut treble_buffer_size: libc::c_int = 128 as libc::c_int;
    if rate > 8125 as libc::c_int as libc::c_uint
        && rate <= 16250 as libc::c_int as libc::c_uint
    {
        treble_buffer_size = 256 as libc::c_int;
    } else if rate > 16250 as libc::c_int as libc::c_uint
        && rate <= 32500 as libc::c_int as libc::c_uint
    {
        treble_buffer_size = 512 as libc::c_int;
    } else if rate > 32500 as libc::c_int as libc::c_uint
        && rate <= 75000 as libc::c_int as libc::c_uint
    {
        treble_buffer_size = 1024 as libc::c_int;
    } else if rate > 75000 as libc::c_int as libc::c_uint
        && rate <= 150000 as libc::c_int as libc::c_uint
    {
        treble_buffer_size = 2048 as libc::c_int;
    } else if rate > 150000 as libc::c_int as libc::c_uint
        && rate <= 300000 as libc::c_int as libc::c_uint
    {
        treble_buffer_size = 4096 as libc::c_int;
    } else if rate > 300000 as libc::c_int as libc::c_uint {
        treble_buffer_size = 8096 as libc::c_int;
    }
    if number_of_bars < 1 as libc::c_int {
        fprintf(
            stderr,
            b"cava_init called with illegal number of bars: %d, number of channels must be positive integer\n\0"
                as *const u8 as *const libc::c_char,
            number_of_bars,
        );
        exit(1 as libc::c_int);
    }
    if number_of_bars > treble_buffer_size / 2 as libc::c_int + 1 as libc::c_int {
        fprintf(
            stderr,
            b"cava_init called with illegal number of bars: %d, for %d sample rate number of bars can't be more than %d positive integer\n\0"
                as *const u8 as *const libc::c_char,
            number_of_bars,
            rate,
            treble_buffer_size / 2 as libc::c_int + 1 as libc::c_int,
        );
        exit(1 as libc::c_int);
    }
    if low_cut_off < 0 as libc::c_int || high_cut_off < 0 as libc::c_int {
        fprintf(
            stderr,
            b"low_cut_off must be a positive value\n\0" as *const u8
                as *const libc::c_char,
        );
        exit(1 as libc::c_int);
    }
    if low_cut_off >= high_cut_off {
        fprintf(
            stderr,
            b"high_cut_off must be a higher than low_cut_off\n\0" as *const u8
                as *const libc::c_char,
        );
        exit(1 as libc::c_int);
    }
    if high_cut_off as libc::c_uint > rate.wrapping_div(2 as libc::c_int as libc::c_uint)
    {
        fprintf(
            stderr,
            b"high_cut_off can't be higher than sample rate / 2. (Nyquist Sampling Theorem)\n\0"
                as *const u8 as *const libc::c_char,
        );
        exit(1 as libc::c_int);
    }
    let mut p: *mut cava_plan = malloc(
        ::std::mem::size_of::<cava_plan>() as libc::c_ulong,
    ) as *mut cava_plan;
    (*p).number_of_bars = number_of_bars;
    (*p).audio_channels = channels;
    (*p).rate = rate as libc::c_int;
    (*p).autosens = 1 as libc::c_int;
    (*p).sens = 1 as libc::c_int as libc::c_double;
    (*p).autosens = autosens;
    (*p).framerate = 75 as libc::c_int as libc::c_double;
    (*p).frame_skip = 1 as libc::c_int;
    (*p).average_max = 0 as libc::c_int as libc::c_double;
    (*p).noise_reduction = noise_reduction;
    (*p).g = log10((*p).height as libc::c_float as libc::c_double) * 0.05f64;
    (*p).FFTbassbufferSize = treble_buffer_size * 4 as libc::c_int;
    (*p).FFTmidbufferSize = treble_buffer_size * 2 as libc::c_int;
    (*p).FFTtreblebufferSize = treble_buffer_size;
    (*p).input_buffer_size = (*p).FFTbassbufferSize * channels;
    (*p)
        .input_buffer = malloc(
        ((*p).input_buffer_size as libc::c_ulong)
            .wrapping_mul(::std::mem::size_of::<libc::c_double>() as libc::c_ulong),
    ) as *mut libc::c_double;
    (*p)
        .FFTbuffer_lower_cut_off = malloc(
        ((number_of_bars + 1 as libc::c_int) as libc::c_ulong)
            .wrapping_mul(::std::mem::size_of::<libc::c_int>() as libc::c_ulong),
    ) as *mut libc::c_int;
    (*p)
        .FFTbuffer_upper_cut_off = malloc(
        ((number_of_bars + 1 as libc::c_int) as libc::c_ulong)
            .wrapping_mul(::std::mem::size_of::<libc::c_int>() as libc::c_ulong),
    ) as *mut libc::c_int;
    (*p)
        .eq = malloc(
        ((number_of_bars + 1 as libc::c_int) as libc::c_ulong)
            .wrapping_mul(::std::mem::size_of::<libc::c_double>() as libc::c_ulong),
    ) as *mut libc::c_double;
    (*p)
        .cut_off_frequency = malloc(
        ((number_of_bars + 1 as libc::c_int) as libc::c_ulong)
            .wrapping_mul(::std::mem::size_of::<libc::c_float>() as libc::c_ulong),
    ) as *mut libc::c_float;
    (*p)
        .cava_fall = malloc(
        ((number_of_bars * channels) as libc::c_ulong)
            .wrapping_mul(::std::mem::size_of::<libc::c_int>() as libc::c_ulong),
    ) as *mut libc::c_int;
    (*p)
        .cava_mem = malloc(
        ((number_of_bars * channels) as libc::c_ulong)
            .wrapping_mul(::std::mem::size_of::<libc::c_double>() as libc::c_ulong),
    ) as *mut libc::c_double;
    (*p)
        .cava_peak = malloc(
        ((number_of_bars * channels) as libc::c_ulong)
            .wrapping_mul(::std::mem::size_of::<libc::c_double>() as libc::c_ulong),
    ) as *mut libc::c_double;
    (*p)
        .prev_cava_out = malloc(
        ((number_of_bars * channels) as libc::c_ulong)
            .wrapping_mul(::std::mem::size_of::<libc::c_double>() as libc::c_ulong),
    ) as *mut libc::c_double;
    (*p)
        .bass_multiplier = malloc(
        ((*p).FFTbassbufferSize as libc::c_ulong)
            .wrapping_mul(::std::mem::size_of::<libc::c_double>() as libc::c_ulong),
    ) as *mut libc::c_double;
    (*p)
        .mid_multiplier = malloc(
        ((*p).FFTmidbufferSize as libc::c_ulong)
            .wrapping_mul(::std::mem::size_of::<libc::c_double>() as libc::c_ulong),
    ) as *mut libc::c_double;
    (*p)
        .treble_multiplier = malloc(
        ((*p).FFTtreblebufferSize as libc::c_ulong)
            .wrapping_mul(::std::mem::size_of::<libc::c_double>() as libc::c_ulong),
    ) as *mut libc::c_double;
    let mut i: libc::c_int = 0 as libc::c_int;
    while i < (*p).FFTbassbufferSize {
        *((*p).bass_multiplier)
            .offset(
                i as isize,
            ) = 0.5f64
            * (1 as libc::c_int as libc::c_double
                - cos(
                    2 as libc::c_int as libc::c_double * 3.14159265358979323846f64
                        * i as libc::c_double
                        / ((*p).FFTbassbufferSize - 1 as libc::c_int) as libc::c_double,
                ));
        i += 1;
        i;
    }
    let mut i_0: libc::c_int = 0 as libc::c_int;
    while i_0 < (*p).FFTmidbufferSize {
        *((*p).mid_multiplier)
            .offset(
                i_0 as isize,
            ) = 0.5f64
            * (1 as libc::c_int as libc::c_double
                - cos(
                    2 as libc::c_int as libc::c_double * 3.14159265358979323846f64
                        * i_0 as libc::c_double
                        / ((*p).FFTmidbufferSize - 1 as libc::c_int) as libc::c_double,
                ));
        i_0 += 1;
        i_0;
    }
    let mut i_1: libc::c_int = 0 as libc::c_int;
    while i_1 < (*p).FFTtreblebufferSize {
        *((*p).treble_multiplier)
            .offset(
                i_1 as isize,
            ) = 0.5f64
            * (1 as libc::c_int as libc::c_double
                - cos(
                    2 as libc::c_int as libc::c_double * 3.14159265358979323846f64
                        * i_1 as libc::c_double
                        / ((*p).FFTtreblebufferSize - 1 as libc::c_int) as libc::c_double,
                ));
        i_1 += 1;
        i_1;
    }
    (*p).in_bass_l = fftw_alloc_real((*p).FFTbassbufferSize as size_t);
    (*p).in_bass_l_raw = fftw_alloc_real((*p).FFTbassbufferSize as size_t);
    (*p)
        .out_bass_l = fftw_alloc_complex(
        ((*p).FFTbassbufferSize / 2 as libc::c_int + 1 as libc::c_int) as size_t,
    );
    (*p)
        .p_bass_l = fftw_plan_dft_r2c_1d(
        (*p).FFTbassbufferSize,
        (*p).in_bass_l,
        (*p).out_bass_l,
        0 as libc::c_uint,
    );
    (*p).in_mid_l = fftw_alloc_real((*p).FFTmidbufferSize as size_t);
    (*p).in_mid_l_raw = fftw_alloc_real((*p).FFTmidbufferSize as size_t);
    (*p)
        .out_mid_l = fftw_alloc_complex(
        ((*p).FFTmidbufferSize / 2 as libc::c_int + 1 as libc::c_int) as size_t,
    );
    (*p)
        .p_mid_l = fftw_plan_dft_r2c_1d(
        (*p).FFTmidbufferSize,
        (*p).in_mid_l,
        (*p).out_mid_l,
        0 as libc::c_uint,
    );
    (*p).in_treble_l = fftw_alloc_real((*p).FFTtreblebufferSize as size_t);
    (*p).in_treble_l_raw = fftw_alloc_real((*p).FFTtreblebufferSize as size_t);
    (*p)
        .out_treble_l = fftw_alloc_complex(
        ((*p).FFTtreblebufferSize / 2 as libc::c_int + 1 as libc::c_int) as size_t,
    );
    (*p)
        .p_treble_l = fftw_plan_dft_r2c_1d(
        (*p).FFTtreblebufferSize,
        (*p).in_treble_l,
        (*p).out_treble_l,
        0 as libc::c_uint,
    );
    memset(
        (*p).in_bass_l as *mut libc::c_void,
        0 as libc::c_int,
        (::std::mem::size_of::<libc::c_double>() as libc::c_ulong)
            .wrapping_mul((*p).FFTbassbufferSize as libc::c_ulong),
    );
    memset(
        (*p).in_mid_l as *mut libc::c_void,
        0 as libc::c_int,
        (::std::mem::size_of::<libc::c_double>() as libc::c_ulong)
            .wrapping_mul((*p).FFTmidbufferSize as libc::c_ulong),
    );
    memset(
        (*p).in_treble_l as *mut libc::c_void,
        0 as libc::c_int,
        (::std::mem::size_of::<libc::c_double>() as libc::c_ulong)
            .wrapping_mul((*p).FFTtreblebufferSize as libc::c_ulong),
    );
    memset(
        (*p).in_bass_l_raw as *mut libc::c_void,
        0 as libc::c_int,
        (::std::mem::size_of::<libc::c_double>() as libc::c_ulong)
            .wrapping_mul((*p).FFTbassbufferSize as libc::c_ulong),
    );
    memset(
        (*p).in_mid_l_raw as *mut libc::c_void,
        0 as libc::c_int,
        (::std::mem::size_of::<libc::c_double>() as libc::c_ulong)
            .wrapping_mul((*p).FFTmidbufferSize as libc::c_ulong),
    );
    memset(
        (*p).in_treble_l_raw as *mut libc::c_void,
        0 as libc::c_int,
        (::std::mem::size_of::<libc::c_double>() as libc::c_ulong)
            .wrapping_mul((*p).FFTtreblebufferSize as libc::c_ulong),
    );
    memset(
        (*p).out_bass_l as *mut libc::c_void,
        0 as libc::c_int,
        (((*p).FFTbassbufferSize / 2 as libc::c_int + 1 as libc::c_int) as libc::c_ulong)
            .wrapping_mul(::std::mem::size_of::<fftw_complex>() as libc::c_ulong),
    );
    memset(
        (*p).out_mid_l as *mut libc::c_void,
        0 as libc::c_int,
        (((*p).FFTmidbufferSize / 2 as libc::c_int + 1 as libc::c_int) as libc::c_ulong)
            .wrapping_mul(::std::mem::size_of::<fftw_complex>() as libc::c_ulong),
    );
    memset(
        (*p).out_treble_l as *mut libc::c_void,
        0 as libc::c_int,
        (((*p).FFTtreblebufferSize / 2 as libc::c_int + 1 as libc::c_int)
            as libc::c_ulong)
            .wrapping_mul(::std::mem::size_of::<fftw_complex>() as libc::c_ulong),
    );
    if (*p).audio_channels == 2 as libc::c_int {
        (*p).in_bass_r = fftw_alloc_real((*p).FFTbassbufferSize as size_t);
        (*p).in_bass_r_raw = fftw_alloc_real((*p).FFTbassbufferSize as size_t);
        (*p)
            .out_bass_r = fftw_alloc_complex(
            ((*p).FFTbassbufferSize / 2 as libc::c_int + 1 as libc::c_int) as size_t,
        );
        (*p)
            .p_bass_r = fftw_plan_dft_r2c_1d(
            (*p).FFTbassbufferSize,
            (*p).in_bass_r,
            (*p).out_bass_r,
            0 as libc::c_uint,
        );
        (*p).in_mid_r = fftw_alloc_real((*p).FFTmidbufferSize as size_t);
        (*p).in_mid_r_raw = fftw_alloc_real((*p).FFTmidbufferSize as size_t);
        (*p)
            .out_mid_r = fftw_alloc_complex(
            ((*p).FFTmidbufferSize / 2 as libc::c_int + 1 as libc::c_int) as size_t,
        );
        (*p)
            .p_mid_r = fftw_plan_dft_r2c_1d(
            (*p).FFTmidbufferSize,
            (*p).in_mid_r,
            (*p).out_mid_r,
            0 as libc::c_uint,
        );
        (*p).in_treble_r = fftw_alloc_real((*p).FFTtreblebufferSize as size_t);
        (*p).in_treble_r_raw = fftw_alloc_real((*p).FFTtreblebufferSize as size_t);
        (*p)
            .out_treble_r = fftw_alloc_complex(
            ((*p).FFTtreblebufferSize / 2 as libc::c_int + 1 as libc::c_int) as size_t,
        );
        (*p)
            .p_treble_r = fftw_plan_dft_r2c_1d(
            (*p).FFTtreblebufferSize,
            (*p).in_treble_r,
            (*p).out_treble_r,
            0 as libc::c_uint,
        );
        memset(
            (*p).in_bass_r as *mut libc::c_void,
            0 as libc::c_int,
            (::std::mem::size_of::<libc::c_double>() as libc::c_ulong)
                .wrapping_mul((*p).FFTbassbufferSize as libc::c_ulong),
        );
        memset(
            (*p).in_mid_r as *mut libc::c_void,
            0 as libc::c_int,
            (::std::mem::size_of::<libc::c_double>() as libc::c_ulong)
                .wrapping_mul((*p).FFTmidbufferSize as libc::c_ulong),
        );
        memset(
            (*p).in_treble_r as *mut libc::c_void,
            0 as libc::c_int,
            (::std::mem::size_of::<libc::c_double>() as libc::c_ulong)
                .wrapping_mul((*p).FFTtreblebufferSize as libc::c_ulong),
        );
        memset(
            (*p).in_bass_r_raw as *mut libc::c_void,
            0 as libc::c_int,
            (::std::mem::size_of::<libc::c_double>() as libc::c_ulong)
                .wrapping_mul((*p).FFTbassbufferSize as libc::c_ulong),
        );
        memset(
            (*p).in_mid_r_raw as *mut libc::c_void,
            0 as libc::c_int,
            (::std::mem::size_of::<libc::c_double>() as libc::c_ulong)
                .wrapping_mul((*p).FFTmidbufferSize as libc::c_ulong),
        );
        memset(
            (*p).in_treble_r_raw as *mut libc::c_void,
            0 as libc::c_int,
            (::std::mem::size_of::<libc::c_double>() as libc::c_ulong)
                .wrapping_mul((*p).FFTtreblebufferSize as libc::c_ulong),
        );
        memset(
            (*p).out_bass_r as *mut libc::c_void,
            0 as libc::c_int,
            (((*p).FFTbassbufferSize / 2 as libc::c_int + 1 as libc::c_int)
                as libc::c_ulong)
                .wrapping_mul(::std::mem::size_of::<fftw_complex>() as libc::c_ulong),
        );
        memset(
            (*p).out_mid_r as *mut libc::c_void,
            0 as libc::c_int,
            (((*p).FFTmidbufferSize / 2 as libc::c_int + 1 as libc::c_int)
                as libc::c_ulong)
                .wrapping_mul(::std::mem::size_of::<fftw_complex>() as libc::c_ulong),
        );
        memset(
            (*p).out_treble_r as *mut libc::c_void,
            0 as libc::c_int,
            (((*p).FFTtreblebufferSize / 2 as libc::c_int + 1 as libc::c_int)
                as libc::c_ulong)
                .wrapping_mul(::std::mem::size_of::<fftw_complex>() as libc::c_ulong),
        );
    }
    memset(
        (*p).input_buffer as *mut libc::c_void,
        0 as libc::c_int,
        (::std::mem::size_of::<libc::c_double>() as libc::c_ulong)
            .wrapping_mul((*p).input_buffer_size as libc::c_ulong),
    );
    memset(
        (*p).cava_fall as *mut libc::c_void,
        0 as libc::c_int,
        (::std::mem::size_of::<libc::c_int>() as libc::c_ulong)
            .wrapping_mul(number_of_bars as libc::c_ulong)
            .wrapping_mul(channels as libc::c_ulong),
    );
    memset(
        (*p).cava_mem as *mut libc::c_void,
        0 as libc::c_int,
        (::std::mem::size_of::<libc::c_double>() as libc::c_ulong)
            .wrapping_mul(number_of_bars as libc::c_ulong)
            .wrapping_mul(channels as libc::c_ulong),
    );
    memset(
        (*p).cava_peak as *mut libc::c_void,
        0 as libc::c_int,
        (::std::mem::size_of::<libc::c_double>() as libc::c_ulong)
            .wrapping_mul(number_of_bars as libc::c_ulong)
            .wrapping_mul(channels as libc::c_ulong),
    );
    memset(
        (*p).prev_cava_out as *mut libc::c_void,
        0 as libc::c_int,
        (::std::mem::size_of::<libc::c_double>() as libc::c_ulong)
            .wrapping_mul(number_of_bars as libc::c_ulong)
            .wrapping_mul(channels as libc::c_ulong),
    );
    let mut lower_cut_off: libc::c_int = low_cut_off;
    let mut upper_cut_off: libc::c_int = high_cut_off;
    let mut bass_cut_off: libc::c_int = 150 as libc::c_int;
    let mut treble_cut_off: libc::c_int = 2500 as libc::c_int;
    let mut frequency_constant: libc::c_double = log10(
        (lower_cut_off as libc::c_float / upper_cut_off as libc::c_float)
            as libc::c_double,
    )
        / (1 as libc::c_int as libc::c_float
            / ((*p).number_of_bars as libc::c_float + 1 as libc::c_int as libc::c_float)
            - 1 as libc::c_int as libc::c_float) as libc::c_double;
    let vla = (*p).FFTtreblebufferSize as usize;
    let mut relative_cut_off: Vec::<libc::c_float> = ::std::vec::from_elem(0., vla);
    (*p).bass_cut_off_bar = -(1 as libc::c_int);
    (*p).treble_cut_off_bar = -(1 as libc::c_int);
    let mut first_bar: libc::c_int = 1 as libc::c_int;
    let mut first_treble_bar: libc::c_int = 0 as libc::c_int;
    let vla_0 = ((*p).number_of_bars + 1 as libc::c_int) as usize;
    let mut bar_buffer: Vec::<libc::c_int> = ::std::vec::from_elem(0, vla_0);
    let mut n: libc::c_int = 0 as libc::c_int;
    while n < (*p).number_of_bars + 1 as libc::c_int {
        let mut bar_distribution_coefficient: libc::c_double = frequency_constant
            * -(1 as libc::c_int) as libc::c_double;
        bar_distribution_coefficient
            += ((n as libc::c_float + 1 as libc::c_int as libc::c_float)
                / ((*p).number_of_bars as libc::c_float
                    + 1 as libc::c_int as libc::c_float)) as libc::c_double
                * frequency_constant;
        *((*p).cut_off_frequency)
            .offset(
                n as isize,
            ) = (upper_cut_off as libc::c_double
            * pow(10 as libc::c_int as libc::c_double, bar_distribution_coefficient))
            as libc::c_float;
        if n > 0 as libc::c_int {
            if *((*p).cut_off_frequency).offset((n - 1 as libc::c_int) as isize)
                >= *((*p).cut_off_frequency).offset(n as isize)
                && *((*p).cut_off_frequency).offset((n - 1 as libc::c_int) as isize)
                    > bass_cut_off as libc::c_float
            {
                *((*p).cut_off_frequency)
                    .offset(
                        n as isize,
                    ) = *((*p).cut_off_frequency).offset((n - 1 as libc::c_int) as isize)
                    + (*((*p).cut_off_frequency).offset((n - 1 as libc::c_int) as isize)
                        - *((*p).cut_off_frequency)
                            .offset((n - 2 as libc::c_int) as isize));
            }
        }
        *relative_cut_off
            .as_mut_ptr()
            .offset(
                n as isize,
            ) = *((*p).cut_off_frequency).offset(n as isize)
            / ((*p).rate / 2 as libc::c_int) as libc::c_float;
        *((*p).eq)
            .offset(
                n as isize,
            ) = pow(
            *((*p).cut_off_frequency).offset(n as isize) as libc::c_double,
            1 as libc::c_int as libc::c_double,
        );
        *((*p).eq).offset(n as isize)
            /= pow(
                2 as libc::c_int as libc::c_double,
                20 as libc::c_int as libc::c_double,
            );
        *((*p).eq).offset(n as isize) /= log2((*p).FFTbassbufferSize as libc::c_double);
        if *((*p).cut_off_frequency).offset(n as isize) < bass_cut_off as libc::c_float {
            *bar_buffer.as_mut_ptr().offset(n as isize) = 1 as libc::c_int;
            *((*p).FFTbuffer_lower_cut_off)
                .offset(
                    n as isize,
                ) = (*relative_cut_off.as_mut_ptr().offset(n as isize)
                * ((*p).FFTbassbufferSize / 2 as libc::c_int) as libc::c_float)
                as libc::c_int;
            (*p).bass_cut_off_bar += 1;
            (*p).bass_cut_off_bar;
            (*p).treble_cut_off_bar += 1;
            (*p).treble_cut_off_bar;
            if (*p).bass_cut_off_bar > 0 as libc::c_int {
                first_bar = 0 as libc::c_int;
            }
            *((*p).eq).offset(n as isize)
                *= log2((*p).FFTbassbufferSize as libc::c_double);
            if *((*p).FFTbuffer_lower_cut_off).offset(n as isize)
                > (*p).FFTbassbufferSize / 2 as libc::c_int
            {
                *((*p).FFTbuffer_lower_cut_off)
                    .offset(n as isize) = (*p).FFTbassbufferSize / 2 as libc::c_int;
            }
        } else if *((*p).cut_off_frequency).offset(n as isize)
            > bass_cut_off as libc::c_float
            && *((*p).cut_off_frequency).offset(n as isize)
                < treble_cut_off as libc::c_float
        {
            *bar_buffer.as_mut_ptr().offset(n as isize) = 2 as libc::c_int;
            *((*p).FFTbuffer_lower_cut_off)
                .offset(
                    n as isize,
                ) = (*relative_cut_off.as_mut_ptr().offset(n as isize)
                * ((*p).FFTmidbufferSize / 2 as libc::c_int) as libc::c_float)
                as libc::c_int;
            (*p).treble_cut_off_bar += 1;
            (*p).treble_cut_off_bar;
            if (*p).treble_cut_off_bar - (*p).bass_cut_off_bar == 1 as libc::c_int {
                first_bar = 1 as libc::c_int;
                if n > 0 as libc::c_int {
                    *((*p).FFTbuffer_upper_cut_off)
                        .offset(
                            (n - 1 as libc::c_int) as isize,
                        ) = (*relative_cut_off.as_mut_ptr().offset(n as isize)
                        * ((*p).FFTbassbufferSize / 2 as libc::c_int) as libc::c_float)
                        as libc::c_int;
                }
            } else {
                first_bar = 0 as libc::c_int;
            }
            *((*p).eq).offset(n as isize)
                *= log2((*p).FFTmidbufferSize as libc::c_double);
            if *((*p).FFTbuffer_lower_cut_off).offset(n as isize)
                > (*p).FFTmidbufferSize / 2 as libc::c_int
            {
                *((*p).FFTbuffer_lower_cut_off)
                    .offset(n as isize) = (*p).FFTmidbufferSize / 2 as libc::c_int;
            }
        } else {
            *bar_buffer.as_mut_ptr().offset(n as isize) = 3 as libc::c_int;
            *((*p).FFTbuffer_lower_cut_off)
                .offset(
                    n as isize,
                ) = (*relative_cut_off.as_mut_ptr().offset(n as isize)
                * ((*p).FFTtreblebufferSize / 2 as libc::c_int) as libc::c_float)
                as libc::c_int;
            first_treble_bar += 1;
            first_treble_bar;
            if first_treble_bar == 1 as libc::c_int {
                first_bar = 1 as libc::c_int;
                if n > 0 as libc::c_int {
                    *((*p).FFTbuffer_upper_cut_off)
                        .offset(
                            (n - 1 as libc::c_int) as isize,
                        ) = (*relative_cut_off.as_mut_ptr().offset(n as isize)
                        * ((*p).FFTmidbufferSize / 2 as libc::c_int) as libc::c_float)
                        as libc::c_int;
                }
            } else {
                first_bar = 0 as libc::c_int;
            }
            *((*p).eq).offset(n as isize)
                *= log2((*p).FFTtreblebufferSize as libc::c_double);
            if *((*p).FFTbuffer_lower_cut_off).offset(n as isize)
                > (*p).FFTtreblebufferSize / 2 as libc::c_int
            {
                *((*p).FFTbuffer_lower_cut_off)
                    .offset(n as isize) = (*p).FFTtreblebufferSize / 2 as libc::c_int;
            }
        }
        if n > 0 as libc::c_int {
            if first_bar == 0 {
                *((*p).FFTbuffer_upper_cut_off)
                    .offset(
                        (n - 1 as libc::c_int) as isize,
                    ) = *((*p).FFTbuffer_lower_cut_off).offset(n as isize)
                    - 1 as libc::c_int;
                if *((*p).FFTbuffer_lower_cut_off).offset(n as isize)
                    <= *((*p).FFTbuffer_lower_cut_off)
                        .offset((n - 1 as libc::c_int) as isize)
                {
                    let mut room_for_more: libc::c_int = 0 as libc::c_int;
                    if *bar_buffer.as_mut_ptr().offset(n as isize) == 1 as libc::c_int {
                        if (*((*p).FFTbuffer_lower_cut_off)
                            .offset((n - 1 as libc::c_int) as isize) + 1 as libc::c_int)
                            < (*p).FFTbassbufferSize / 2 as libc::c_int
                                + 1 as libc::c_int
                        {
                            room_for_more = 1 as libc::c_int;
                        }
                    } else if *bar_buffer.as_mut_ptr().offset(n as isize)
                        == 2 as libc::c_int
                    {
                        if (*((*p).FFTbuffer_lower_cut_off)
                            .offset((n - 1 as libc::c_int) as isize) + 1 as libc::c_int)
                            < (*p).FFTmidbufferSize / 2 as libc::c_int + 1 as libc::c_int
                        {
                            room_for_more = 1 as libc::c_int;
                        }
                    } else if *bar_buffer.as_mut_ptr().offset(n as isize)
                        == 3 as libc::c_int
                    {
                        if (*((*p).FFTbuffer_lower_cut_off)
                            .offset((n - 1 as libc::c_int) as isize) + 1 as libc::c_int)
                            < (*p).FFTtreblebufferSize / 2 as libc::c_int
                                + 1 as libc::c_int
                        {
                            room_for_more = 1 as libc::c_int;
                        }
                    }
                    if room_for_more != 0 {
                        *((*p).FFTbuffer_lower_cut_off)
                            .offset(
                                n as isize,
                            ) = *((*p).FFTbuffer_lower_cut_off)
                            .offset((n - 1 as libc::c_int) as isize) + 1 as libc::c_int;
                        *((*p).FFTbuffer_upper_cut_off)
                            .offset(
                                (n - 1 as libc::c_int) as isize,
                            ) = *((*p).FFTbuffer_lower_cut_off).offset(n as isize)
                            - 1 as libc::c_int;
                        if *bar_buffer.as_mut_ptr().offset(n as isize)
                            == 1 as libc::c_int
                        {
                            *relative_cut_off
                                .as_mut_ptr()
                                .offset(
                                    n as isize,
                                ) = *((*p).FFTbuffer_lower_cut_off).offset(n as isize)
                                as libc::c_float
                                / ((*p).FFTbassbufferSize as libc::c_float
                                    / 2 as libc::c_int as libc::c_float);
                        } else if *bar_buffer.as_mut_ptr().offset(n as isize)
                            == 2 as libc::c_int
                        {
                            *relative_cut_off
                                .as_mut_ptr()
                                .offset(
                                    n as isize,
                                ) = *((*p).FFTbuffer_lower_cut_off).offset(n as isize)
                                as libc::c_float
                                / ((*p).FFTmidbufferSize as libc::c_float
                                    / 2 as libc::c_int as libc::c_float);
                        } else if *bar_buffer.as_mut_ptr().offset(n as isize)
                            == 3 as libc::c_int
                        {
                            *relative_cut_off
                                .as_mut_ptr()
                                .offset(
                                    n as isize,
                                ) = *((*p).FFTbuffer_lower_cut_off).offset(n as isize)
                                as libc::c_float
                                / ((*p).FFTtreblebufferSize as libc::c_float
                                    / 2 as libc::c_int as libc::c_float);
                        }
                        *((*p).cut_off_frequency)
                            .offset(
                                n as isize,
                            ) = *relative_cut_off.as_mut_ptr().offset(n as isize)
                            * ((*p).rate as libc::c_float
                                / 2 as libc::c_int as libc::c_float);
                    }
                }
            } else if *((*p).FFTbuffer_upper_cut_off)
                .offset((n - 1 as libc::c_int) as isize)
                <= *((*p).FFTbuffer_lower_cut_off)
                    .offset((n - 1 as libc::c_int) as isize)
            {
                *((*p).FFTbuffer_upper_cut_off)
                    .offset(
                        (n - 1 as libc::c_int) as isize,
                    ) = *((*p).FFTbuffer_lower_cut_off)
                    .offset((n - 1 as libc::c_int) as isize) + 1 as libc::c_int;
            }
        }
        n += 1;
        n;
    }
    return p;
}
pub unsafe extern "C" fn cava_execute(
    mut cava_in: *mut libc::c_double,
    mut new_samples: libc::c_int,
    mut cava_out: *mut libc::c_double,
    mut p: *mut cava_plan,
) {
    if new_samples > (*p).input_buffer_size {
        new_samples = (*p).input_buffer_size;
    }
    let mut silence: libc::c_int = 1 as libc::c_int;
    if new_samples > 0 as libc::c_int {
        (*p).framerate -= (*p).framerate / 64 as libc::c_int as libc::c_double;
        (*p).framerate
            += ((*p).rate * (*p).audio_channels * (*p).frame_skip / new_samples)
                as libc::c_double / 64 as libc::c_int as libc::c_double;
        (*p).frame_skip = 1 as libc::c_int;
        let mut n: uint16_t = ((*p).input_buffer_size - 1 as libc::c_int) as uint16_t;
        while n as libc::c_int >= new_samples {
            *((*p).input_buffer)
                .offset(
                    n as isize,
                ) = *((*p).input_buffer)
                .offset((n as libc::c_int - new_samples) as isize);
            n = n.wrapping_sub(1);
            n;
        }
        let mut n_0: uint16_t = 0 as libc::c_int as uint16_t;
        while (n_0 as libc::c_int) < new_samples {
            *((*p).input_buffer)
                .offset(
                    (new_samples - n_0 as libc::c_int - 1 as libc::c_int) as isize,
                ) = *cava_in.offset(n_0 as isize);
            if *cava_in.offset(n_0 as isize) != 0. {
                silence = 0 as libc::c_int;
            }
            n_0 = n_0.wrapping_add(1);
            n_0;
        }
    } else {
        (*p).frame_skip += 1;
        (*p).frame_skip;
    }
    let mut n_1: uint16_t = 0 as libc::c_int as uint16_t;
    while (n_1 as libc::c_int) < (*p).FFTbassbufferSize {
        if (*p).audio_channels == 2 as libc::c_int {
            *((*p).in_bass_l_raw)
                .offset(
                    n_1 as isize,
                ) = *((*p).input_buffer)
                .offset((n_1 as libc::c_int * 2 as libc::c_int) as isize);
            *((*p).in_bass_r_raw)
                .offset(
                    n_1 as isize,
                ) = *((*p).input_buffer)
                .offset(
                    (n_1 as libc::c_int * 2 as libc::c_int + 1 as libc::c_int) as isize,
                );
        } else {
            *((*p).in_bass_l_raw)
                .offset(n_1 as isize) = *((*p).input_buffer).offset(n_1 as isize);
        }
        n_1 = n_1.wrapping_add(1);
        n_1;
    }
    let mut n_2: uint16_t = 0 as libc::c_int as uint16_t;
    while (n_2 as libc::c_int) < (*p).FFTmidbufferSize {
        if (*p).audio_channels == 2 as libc::c_int {
            *((*p).in_mid_l_raw)
                .offset(
                    n_2 as isize,
                ) = *((*p).input_buffer)
                .offset((n_2 as libc::c_int * 2 as libc::c_int) as isize);
            *((*p).in_mid_r_raw)
                .offset(
                    n_2 as isize,
                ) = *((*p).input_buffer)
                .offset(
                    (n_2 as libc::c_int * 2 as libc::c_int + 1 as libc::c_int) as isize,
                );
        } else {
            *((*p).in_mid_l_raw)
                .offset(n_2 as isize) = *((*p).input_buffer).offset(n_2 as isize);
        }
        n_2 = n_2.wrapping_add(1);
        n_2;
    }
    let mut n_3: uint16_t = 0 as libc::c_int as uint16_t;
    while (n_3 as libc::c_int) < (*p).FFTtreblebufferSize {
        if (*p).audio_channels == 2 as libc::c_int {
            *((*p).in_treble_l_raw)
                .offset(
                    n_3 as isize,
                ) = *((*p).input_buffer)
                .offset((n_3 as libc::c_int * 2 as libc::c_int) as isize);
            *((*p).in_treble_r_raw)
                .offset(
                    n_3 as isize,
                ) = *((*p).input_buffer)
                .offset(
                    (n_3 as libc::c_int * 2 as libc::c_int + 1 as libc::c_int) as isize,
                );
        } else {
            *((*p).in_treble_l_raw)
                .offset(n_3 as isize) = *((*p).input_buffer).offset(n_3 as isize);
        }
        n_3 = n_3.wrapping_add(1);
        n_3;
    }
    let mut i: libc::c_int = 0 as libc::c_int;
    while i < (*p).FFTbassbufferSize {
        *((*p).in_bass_l)
            .offset(
                i as isize,
            ) = *((*p).bass_multiplier).offset(i as isize)
            * *((*p).in_bass_l_raw).offset(i as isize);
        if (*p).audio_channels == 2 as libc::c_int {
            *((*p).in_bass_r)
                .offset(
                    i as isize,
                ) = *((*p).bass_multiplier).offset(i as isize)
                * *((*p).in_bass_r_raw).offset(i as isize);
        }
        i += 1;
        i;
    }
    let mut i_0: libc::c_int = 0 as libc::c_int;
    while i_0 < (*p).FFTmidbufferSize {
        *((*p).in_mid_l)
            .offset(
                i_0 as isize,
            ) = *((*p).mid_multiplier).offset(i_0 as isize)
            * *((*p).in_mid_l_raw).offset(i_0 as isize);
        if (*p).audio_channels == 2 as libc::c_int {
            *((*p).in_mid_r)
                .offset(
                    i_0 as isize,
                ) = *((*p).mid_multiplier).offset(i_0 as isize)
                * *((*p).in_mid_r_raw).offset(i_0 as isize);
        }
        i_0 += 1;
        i_0;
    }
    let mut i_1: libc::c_int = 0 as libc::c_int;
    while i_1 < (*p).FFTtreblebufferSize {
        *((*p).in_treble_l)
            .offset(
                i_1 as isize,
            ) = *((*p).treble_multiplier).offset(i_1 as isize)
            * *((*p).in_treble_l_raw).offset(i_1 as isize);
        if (*p).audio_channels == 2 as libc::c_int {
            *((*p).in_treble_r)
                .offset(
                    i_1 as isize,
                ) = *((*p).treble_multiplier).offset(i_1 as isize)
                * *((*p).in_treble_r_raw).offset(i_1 as isize);
        }
        i_1 += 1;
        i_1;
    }
    fftw_execute((*p).p_bass_l);
    fftw_execute((*p).p_mid_l);
    fftw_execute((*p).p_treble_l);
    if (*p).audio_channels == 2 as libc::c_int {
        fftw_execute((*p).p_bass_r);
        fftw_execute((*p).p_mid_r);
        fftw_execute((*p).p_treble_r);
    }
    let mut n_4: libc::c_int = 0 as libc::c_int;
    while n_4 < (*p).number_of_bars {
        let mut temp_l: libc::c_double = 0 as libc::c_int as libc::c_double;
        let mut temp_r: libc::c_double = 0 as libc::c_int as libc::c_double;
        let mut i_2: libc::c_int = *((*p).FFTbuffer_lower_cut_off).offset(n_4 as isize);
        while i_2 <= *((*p).FFTbuffer_upper_cut_off).offset(n_4 as isize) {
            if n_4 <= (*p).bass_cut_off_bar {
                temp_l
                    += hypot(
                        (*((*p).out_bass_l)
                            .offset(i_2 as isize))[0 as libc::c_int as usize],
                        (*((*p).out_bass_l)
                            .offset(i_2 as isize))[1 as libc::c_int as usize],
                    );
                if (*p).audio_channels == 2 as libc::c_int {
                    temp_r
                        += hypot(
                            (*((*p).out_bass_r)
                                .offset(i_2 as isize))[0 as libc::c_int as usize],
                            (*((*p).out_bass_r)
                                .offset(i_2 as isize))[1 as libc::c_int as usize],
                        );
                }
            } else if n_4 > (*p).bass_cut_off_bar && n_4 <= (*p).treble_cut_off_bar {
                temp_l
                    += hypot(
                        (*((*p).out_mid_l)
                            .offset(i_2 as isize))[0 as libc::c_int as usize],
                        (*((*p).out_mid_l)
                            .offset(i_2 as isize))[1 as libc::c_int as usize],
                    );
                if (*p).audio_channels == 2 as libc::c_int {
                    temp_r
                        += hypot(
                            (*((*p).out_mid_r)
                                .offset(i_2 as isize))[0 as libc::c_int as usize],
                            (*((*p).out_mid_r)
                                .offset(i_2 as isize))[1 as libc::c_int as usize],
                        );
                }
            } else if n_4 > (*p).treble_cut_off_bar {
                temp_l
                    += hypot(
                        (*((*p).out_treble_l)
                            .offset(i_2 as isize))[0 as libc::c_int as usize],
                        (*((*p).out_treble_l)
                            .offset(i_2 as isize))[1 as libc::c_int as usize],
                    );
                if (*p).audio_channels == 2 as libc::c_int {
                    temp_r
                        += hypot(
                            (*((*p).out_treble_r)
                                .offset(i_2 as isize))[0 as libc::c_int as usize],
                            (*((*p).out_treble_r)
                                .offset(i_2 as isize))[1 as libc::c_int as usize],
                        );
                }
            }
            i_2 += 1;
            i_2;
        }
        temp_l
            /= (*((*p).FFTbuffer_upper_cut_off).offset(n_4 as isize)
                - *((*p).FFTbuffer_lower_cut_off).offset(n_4 as isize)
                + 1 as libc::c_int) as libc::c_double;
        temp_l *= *((*p).eq).offset(n_4 as isize);
        *cava_out.offset(n_4 as isize) = temp_l;
        if (*p).audio_channels == 2 as libc::c_int {
            temp_r
                /= (*((*p).FFTbuffer_upper_cut_off).offset(n_4 as isize)
                    - *((*p).FFTbuffer_lower_cut_off).offset(n_4 as isize)
                    + 1 as libc::c_int) as libc::c_double;
            temp_r *= *((*p).eq).offset(n_4 as isize);
            *cava_out.offset((n_4 + (*p).number_of_bars) as isize) = temp_r;
        }
        n_4 += 1;
        n_4;
    }
    let mut n_5: libc::c_int = 0 as libc::c_int;
    while n_5 < (*p).number_of_bars * (*p).audio_channels {
        if (*p).autosens != 0 {
            *cava_out.offset(n_5 as isize) *= (*p).sens;
        } else if *cava_out.offset(n_5 as isize) > (*p).average_max {
            (*p).average_max -= (*p).average_max / 64 as libc::c_int as libc::c_double;
            (*p).average_max
                += *cava_out.offset(n_5 as isize) / 64 as libc::c_int as libc::c_double;
        }
        n_5 += 1;
        n_5;
    }
    let mut overshoot: libc::c_int = 0 as libc::c_int;
    let mut gravity_mod: libc::c_double = pow(
        60 as libc::c_int as libc::c_double / (*p).framerate,
        2.5f64,
    ) * 1.54f64 / (*p).noise_reduction;
    if gravity_mod < 1 as libc::c_int as libc::c_double {
        gravity_mod = 1 as libc::c_int as libc::c_double;
    }
    let mut n_6: libc::c_int = 0 as libc::c_int;
    while n_6 < (*p).number_of_bars * (*p).audio_channels {
        if *cava_out.offset(n_6 as isize) < *((*p).prev_cava_out).offset(n_6 as isize)
            && (*p).noise_reduction > 0.1f64
        {
            *cava_out
                .offset(
                    n_6 as isize,
                ) = *((*p).cava_peak).offset(n_6 as isize)
                * (1000 as libc::c_int as libc::c_double
                    - (*((*p).cava_fall).offset(n_6 as isize)
                        * *((*p).cava_fall).offset(n_6 as isize)) as libc::c_double
                        * gravity_mod) / 1000 as libc::c_int as libc::c_double;
            if *cava_out.offset(n_6 as isize) < 0 as libc::c_int as libc::c_double {
                *cava_out.offset(n_6 as isize) = 0 as libc::c_int as libc::c_double;
            }
            let ref mut fresh0 = *((*p).cava_fall).offset(n_6 as isize);
            *fresh0 += 1;
            *fresh0;
        } else {
            *((*p).cava_peak).offset(n_6 as isize) = *cava_out.offset(n_6 as isize);
            *((*p).cava_fall).offset(n_6 as isize) = 0 as libc::c_int;
        }
        *((*p).prev_cava_out).offset(n_6 as isize) = *cava_out.offset(n_6 as isize);
        *cava_out
            .offset(
                n_6 as isize,
            ) = *((*p).cava_mem).offset(n_6 as isize) * (*p).noise_reduction
            + *cava_out.offset(n_6 as isize);
        *((*p).cava_mem).offset(n_6 as isize) = *cava_out.offset(n_6 as isize);
        if (*p).autosens != 0 {
            let mut diff: libc::c_double = 1000 as libc::c_int as libc::c_double
                - *cava_out.offset(n_6 as isize);
            if diff < 0 as libc::c_int as libc::c_double {
                diff = 0 as libc::c_int as libc::c_double;
            }
            let mut div: libc::c_double = 1 as libc::c_int as libc::c_double
                / (diff + 1 as libc::c_int as libc::c_double);
            *((*p).cava_mem)
                .offset(
                    n_6 as isize,
                ) = *((*p).cava_mem).offset(n_6 as isize)
                * (1 as libc::c_int as libc::c_double
                    - div / 20 as libc::c_int as libc::c_double);
            if *cava_out.offset(n_6 as isize) > 1000 as libc::c_int as libc::c_double {
                overshoot = 1 as libc::c_int;
            }
            *cava_out.offset(n_6 as isize) /= 1000 as libc::c_int as libc::c_double;
        }
        n_6 += 1;
        n_6;
    }
    if (*p).autosens != 0 {
        if overshoot != 0 {
            (*p).sens = (*p).sens * 0.98f64;
            (*p).sens_init = 0 as libc::c_int;
        } else if silence == 0 {
            (*p).sens = (*p).sens * 1.001f64;
            if (*p).sens_init != 0 {
                (*p).sens = (*p).sens * 1.1f64;
            }
        }
    }
}
pub unsafe extern "C" fn cava_destroy(mut p: *mut cava_plan) {
    fftw_free((*p).in_bass_l as *mut libc::c_void);
    fftw_free((*p).in_bass_l_raw as *mut libc::c_void);
    fftw_free((*p).out_bass_l as *mut libc::c_void);
    fftw_destroy_plan((*p).p_bass_l);
    fftw_free((*p).in_mid_l as *mut libc::c_void);
    fftw_free((*p).in_mid_l_raw as *mut libc::c_void);
    fftw_free((*p).out_mid_l as *mut libc::c_void);
    fftw_destroy_plan((*p).p_mid_l);
    fftw_free((*p).in_treble_l as *mut libc::c_void);
    fftw_free((*p).in_treble_l_raw as *mut libc::c_void);
    fftw_free((*p).out_treble_l as *mut libc::c_void);
    fftw_destroy_plan((*p).p_treble_l);
    if (*p).audio_channels == 2 as libc::c_int {
        fftw_free((*p).in_bass_r as *mut libc::c_void);
        fftw_free((*p).in_bass_r_raw as *mut libc::c_void);
        fftw_free((*p).out_bass_r as *mut libc::c_void);
        fftw_destroy_plan((*p).p_bass_r);
        fftw_free((*p).in_mid_r as *mut libc::c_void);
        fftw_free((*p).in_mid_r_raw as *mut libc::c_void);
        fftw_free((*p).out_mid_r as *mut libc::c_void);
        fftw_destroy_plan((*p).p_mid_r);
        fftw_free((*p).in_treble_r as *mut libc::c_void);
        fftw_free((*p).out_treble_r as *mut libc::c_void);
        fftw_free((*p).in_treble_r_raw as *mut libc::c_void);
        fftw_destroy_plan((*p).p_treble_r);
    }
    free((*p).input_buffer as *mut libc::c_void);
    free((*p).bass_multiplier as *mut libc::c_void);
    free((*p).mid_multiplier as *mut libc::c_void);
    free((*p).treble_multiplier as *mut libc::c_void);
    free((*p).eq as *mut libc::c_void);
    free((*p).cut_off_frequency as *mut libc::c_void);
    free((*p).FFTbuffer_lower_cut_off as *mut libc::c_void);
    free((*p).FFTbuffer_upper_cut_off as *mut libc::c_void);
    free((*p).cava_fall as *mut libc::c_void);
    free((*p).cava_mem as *mut libc::c_void);
    free((*p).cava_peak as *mut libc::c_void);
    free((*p).prev_cava_out as *mut libc::c_void);
}
