use ::libc;
extern "C" {
    pub type _snd_pcm;
    pub type _snd_pcm_hw_params;
    pub type _IO_wide_data;
    pub type _IO_codecvt;
    pub type _IO_marker;
    fn fprintf(_: *mut FILE, _: *const libc::c_char, _: ...) -> libc::c_int;
    static mut stderr: *mut FILE;
    fn memset(
        _: *mut libc::c_void,
        _: libc::c_int,
        _: libc::c_ulong,
    ) -> *mut libc::c_void;
    fn write_to_cava_input_buffers(
        size: int16_t,
        buf: *mut int16_t,
        data: *mut libc::c_void,
    ) -> libc::c_int;
    fn malloc(_: libc::c_ulong) -> *mut libc::c_void;
    fn exit(_: libc::c_int) -> !;
    fn abs(_: libc::c_int) -> libc::c_int;
    fn free(__ptr: *mut libc::c_void);
    fn snd_pcm_hw_params_sizeof() -> size_t;
    fn snd_pcm_hw_params_any(
        pcm: *mut snd_pcm_t,
        params: *mut snd_pcm_hw_params_t,
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
    fn snd_pcm_hw_params(
        pcm: *mut snd_pcm_t,
        params: *mut snd_pcm_hw_params_t,
    ) -> libc::c_int;
    fn snd_strerror(errnum: libc::c_int) -> *const libc::c_char;
    fn snd_pcm_open(
        pcm: *mut *mut snd_pcm_t,
        name: *const libc::c_char,
        stream: snd_pcm_stream_t,
        mode: libc::c_int,
    ) -> libc::c_int;
    fn snd_pcm_hw_params_get_format(
        params: *const snd_pcm_hw_params_t,
        val: *mut snd_pcm_format_t,
    ) -> libc::c_int;
    fn snd_pcm_hw_params_get_rate(
        params: *const snd_pcm_hw_params_t,
        val: *mut libc::c_uint,
        dir: *mut libc::c_int,
    ) -> libc::c_int;
    fn snd_pcm_hw_params_get_period_size(
        params: *const snd_pcm_hw_params_t,
        frames: *mut snd_pcm_uframes_t,
        dir: *mut libc::c_int,
    ) -> libc::c_int;
    fn snd_pcm_get_params(
        pcm: *mut snd_pcm_t,
        buffer_size: *mut snd_pcm_uframes_t,
        period_size: *mut snd_pcm_uframes_t,
    ) -> libc::c_int;
    fn snd_pcm_readi(
        pcm: *mut snd_pcm_t,
        buffer: *mut libc::c_void,
        size: snd_pcm_uframes_t,
    ) -> snd_pcm_sframes_t;
    fn snd_pcm_prepare(pcm: *mut snd_pcm_t) -> libc::c_int;
    fn snd_pcm_close(pcm: *mut snd_pcm_t) -> libc::c_int;
    fn snd_pcm_hw_params_set_access(
        pcm: *mut snd_pcm_t,
        params: *mut snd_pcm_hw_params_t,
        _access: snd_pcm_access_t,
    ) -> libc::c_int;
    fn pow(_: libc::c_double, _: libc::c_double) -> libc::c_double;
}
pub type snd_pcm_t = _snd_pcm;
pub type snd_pcm_uframes_t = libc::c_ulong;
pub type int16_t = __int16_t;
pub type __int16_t = libc::c_short;
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
pub type uint16_t = __uint16_t;
pub type __uint16_t = libc::c_ushort;
pub type snd_pcm_sframes_t = libc::c_long;
pub type int32_t = __int32_t;
pub type __int32_t = libc::c_int;
pub type snd_pcm_hw_params_t = _snd_pcm_hw_params;
pub type snd_pcm_format_t = _snd_pcm_format;
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
pub type snd_pcm_stream_t = _snd_pcm_stream;
pub type _snd_pcm_stream = libc::c_uint;
pub const SND_PCM_STREAM_LAST: _snd_pcm_stream = 1;
pub const SND_PCM_STREAM_CAPTURE: _snd_pcm_stream = 1;
pub const SND_PCM_STREAM_PLAYBACK: _snd_pcm_stream = 0;
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
pub type size_t = libc::c_ulong;
pub type __off64_t = libc::c_long;
pub type _IO_lock_t = ();
pub type __off_t = libc::c_long;
pub type snd_pcm_access_t = _snd_pcm_access;
pub type _snd_pcm_access = libc::c_uint;
pub const SND_PCM_ACCESS_LAST: _snd_pcm_access = 4;
pub const SND_PCM_ACCESS_RW_NONINTERLEAVED: _snd_pcm_access = 4;
pub const SND_PCM_ACCESS_RW_INTERLEAVED: _snd_pcm_access = 3;
pub const SND_PCM_ACCESS_MMAP_COMPLEX: _snd_pcm_access = 2;
pub const SND_PCM_ACCESS_MMAP_NONINTERLEAVED: _snd_pcm_access = 1;
pub const SND_PCM_ACCESS_MMAP_INTERLEAVED: _snd_pcm_access = 0;
unsafe extern "C" fn initialize_audio_parameters(
    mut handle: *mut *mut snd_pcm_t,
    mut audio: *mut audio_data,
    mut frames: *mut snd_pcm_uframes_t,
) {
    let mut err: libc::c_int = snd_pcm_open(
        handle,
        (*audio).source,
        SND_PCM_STREAM_CAPTURE,
        0 as libc::c_int,
    );
    if err < 0 as libc::c_int {
        fprintf(
            stderr,
            b"error opening stream: %s\n\0" as *const u8 as *const libc::c_char,
            snd_strerror(err),
        );
        exit(1 as libc::c_int);
    }
    let mut params: *mut snd_pcm_hw_params_t = 0 as *mut snd_pcm_hw_params_t;
    let mut fresh0 = ::std::vec::from_elem(0, snd_pcm_hw_params_sizeof() as usize);
    params = fresh0.leak().as_mut_ptr() as *mut snd_pcm_hw_params_t;
    memset(params as *mut libc::c_void, 0 as libc::c_int, snd_pcm_hw_params_sizeof());
    snd_pcm_hw_params_any(*handle, params);
    snd_pcm_hw_params_set_access(*handle, params, SND_PCM_ACCESS_RW_INTERLEAVED);
    snd_pcm_hw_params_set_format(*handle, params, SND_PCM_FORMAT_S16_LE);
    snd_pcm_hw_params_set_channels(*handle, params, 2 as libc::c_int as libc::c_uint);
    let mut sample_rate: libc::c_uint = 44100 as libc::c_int as libc::c_uint;
    snd_pcm_hw_params_set_rate_near(
        *handle,
        params,
        &mut sample_rate,
        0 as *mut libc::c_int,
    );
    snd_pcm_hw_params_set_period_size_near(
        *handle,
        params,
        frames,
        0 as *mut libc::c_int,
    );
    err = snd_pcm_hw_params(*handle, params);
    if err < 0 as libc::c_int {
        fprintf(
            stderr,
            b"unable to set hw parameters: %s\n\0" as *const u8 as *const libc::c_char,
            snd_strerror(err),
        );
        exit(1 as libc::c_int);
    }
    err = snd_pcm_prepare(*handle);
    if err < 0 as libc::c_int {
        fprintf(
            stderr,
            b"cannot prepare audio interface for use (%s)\n\0" as *const u8
                as *const libc::c_char,
            snd_strerror(err),
        );
        exit(1 as libc::c_int);
    }
    snd_pcm_hw_params_get_format(
        params,
        &mut sample_rate as *mut libc::c_uint as *mut snd_pcm_format_t,
    );
    if sample_rate <= 5 as libc::c_int as libc::c_uint {
        (*audio).format = 16 as libc::c_int;
    } else if sample_rate <= 9 as libc::c_int as libc::c_uint {
        (*audio).format = 24 as libc::c_int;
    } else {
        (*audio).format = 32 as libc::c_int;
    }
    snd_pcm_hw_params_get_rate(params, &mut (*audio).rate, 0 as *mut libc::c_int);
    snd_pcm_hw_params_get_period_size(params, frames, 0 as *mut libc::c_int);
}
unsafe extern "C" fn get_certain_frame(
    mut buffer: *mut libc::c_schar,
    mut buffer_index: libc::c_int,
    mut adjustment: libc::c_int,
) -> libc::c_int {
    let mut temp: libc::c_int = (*buffer
        .offset((buffer_index + adjustment - 1 as libc::c_int) as isize) as libc::c_int)
        << 2 as libc::c_int;
    let mut lo: libc::c_int = *buffer
        .offset((buffer_index + adjustment - 2 as libc::c_int) as isize) as libc::c_int
        >> 6 as libc::c_int;
    if lo < 0 as libc::c_int {
        lo = abs(lo) + 1 as libc::c_int;
    }
    if temp >= 0 as libc::c_int {
        temp += lo;
    } else {
        temp -= lo;
    }
    return temp;
}
pub unsafe extern "C" fn input_alsa(mut data: *mut libc::c_void) -> *mut libc::c_void {
    let mut err: libc::c_int = 0;
    let mut audio: *mut audio_data = data as *mut audio_data;
    let mut handle: *mut snd_pcm_t = 0 as *mut snd_pcm_t;
    let mut buffer_size: snd_pcm_uframes_t = 0;
    let mut period_size: snd_pcm_uframes_t = 0;
    let mut frames: snd_pcm_uframes_t = ((*audio).input_buffer_size / 2 as libc::c_int)
        as snd_pcm_uframes_t;
    initialize_audio_parameters(&mut handle, audio, &mut frames);
    snd_pcm_get_params(handle, &mut buffer_size, &mut period_size);
    let mut radj: libc::c_int = (*audio).format / 4 as libc::c_int;
    let mut ladj: libc::c_int = (*audio).format / 8 as libc::c_int;
    let vla = period_size as usize;
    let mut buf: Vec::<int16_t> = ::std::vec::from_elem(0, vla);
    let vla_0 = period_size as usize;
    let mut buffer32: Vec::<int32_t> = ::std::vec::from_elem(0, vla_0);
    frames = period_size
        .wrapping_div(
            ((*audio).format / 8 as libc::c_int * 2 as libc::c_int) as libc::c_ulong,
        );
    let mut buffer: *mut libc::c_schar = malloc(period_size) as *mut libc::c_schar;
    while (*audio).terminate == 0 {
        match (*audio).format {
            16 => {
                err = snd_pcm_readi(
                    handle,
                    buf.as_mut_ptr() as *mut libc::c_void,
                    frames,
                ) as libc::c_int;
            }
            32 => {
                err = snd_pcm_readi(
                    handle,
                    buffer32.as_mut_ptr() as *mut libc::c_void,
                    frames,
                ) as libc::c_int;
                let mut i: uint16_t = 0 as libc::c_int as uint16_t;
                while (i as libc::c_ulong)
                    < frames.wrapping_mul(2 as libc::c_int as libc::c_ulong)
                {
                    *buf
                        .as_mut_ptr()
                        .offset(
                            i as isize,
                        ) = (*buffer32.as_mut_ptr().offset(i as isize) as libc::c_double
                        / pow(
                            2 as libc::c_int as libc::c_double,
                            16 as libc::c_int as libc::c_double,
                        )) as int16_t;
                    i = i.wrapping_add(1);
                    i;
                }
            }
            _ => {
                err = snd_pcm_readi(handle, buffer as *mut libc::c_void, frames)
                    as libc::c_int;
                let mut i_0: uint16_t = 0 as libc::c_int as uint16_t;
                while (i_0 as libc::c_ulong)
                    < period_size.wrapping_mul(2 as libc::c_int as libc::c_ulong)
                {
                    *buf
                        .as_mut_ptr()
                        .offset(
                            i_0 as isize,
                        ) = get_certain_frame(buffer, i_0 as libc::c_int, ladj)
                        as int16_t;
                    *buf
                        .as_mut_ptr()
                        .offset(
                            (i_0 as libc::c_int + 1 as libc::c_int) as isize,
                        ) = get_certain_frame(buffer, i_0 as libc::c_int, radj)
                        as int16_t;
                    i_0 = (i_0 as libc::c_int + ladj * 2 as libc::c_int) as uint16_t;
                }
            }
        }
        if err == -(32 as libc::c_int) {
            snd_pcm_prepare(handle);
        } else if !(err < 0 as libc::c_int) {
            err != frames as libc::c_int;
        }
        write_to_cava_input_buffers(
            frames.wrapping_mul(2 as libc::c_int as libc::c_ulong) as int16_t,
            buf.as_mut_ptr(),
            data,
        );
    }
    free(buffer as *mut libc::c_void);
    snd_pcm_close(handle);
    return 0 as *mut libc::c_void;
}
