use ::libc;
extern "C" {
    fn rand() -> libc::c_int;
    fn malloc(_: libc::c_ulong) -> *mut libc::c_void;
    fn free(__ptr: *mut libc::c_void);
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
    fn AES_set_decrypt_key(
        userKey: *const libc::c_uchar,
        bits: libc::c_int,
        key: *mut AES_KEY,
    ) -> libc::c_int;
    fn AES_cbc_encrypt(
        in_0: *const libc::c_uchar,
        out: *mut libc::c_uchar,
        length: size_t,
        key: *const AES_KEY,
        ivec: *mut libc::c_uchar,
        enc: libc::c_int,
    );
    fn cos(_: libc::c_double) -> libc::c_double;
    fn sin(_: libc::c_double) -> libc::c_double;
    fn pow(_: libc::c_double, _: libc::c_double) -> libc::c_double;
    fn fabs(_: libc::c_double) -> libc::c_double;
    fn __assert_fail(
        __assertion: *const libc::c_char,
        __file: *const libc::c_char,
        __line: libc::c_uint,
        __function: *const libc::c_char,
    ) -> !;
    fn die(format: *mut libc::c_char, _: ...);
    fn warn(format: *mut libc::c_char, _: ...);
    fn debug(level: libc::c_int, format: *mut libc::c_char, _: ...);
    fn command_start();
    fn command_stop();
    static mut config: shairport_cfg;
    fn rtp_request_resend(first: seq_t, last: seq_t);
    fn alac_create(samplesize: libc::c_int, numchannels: libc::c_int) -> *mut alac_file;
    fn alac_decode_frame(
        alac: *mut alac_file,
        inbuffer: *mut libc::c_uchar,
        outbuffer: *mut libc::c_void,
        outputsize: *mut libc::c_int,
    );
    fn alac_allocate_buffers(alac: *mut alac_file);
    fn alac_free(alac: *mut alac_file);
}
pub type size_t = libc::c_ulong;
pub type __uint8_t = libc::c_uchar;
pub type __int16_t = libc::c_short;
pub type __uint16_t = libc::c_ushort;
pub type __int32_t = libc::c_int;
pub type __uint32_t = libc::c_uint;
pub type int16_t = __int16_t;
pub type int32_t = __int32_t;
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
pub type C2RustUnnamed = libc::c_uint;
pub const PTHREAD_MUTEX_DEFAULT: C2RustUnnamed = 0;
pub const PTHREAD_MUTEX_ERRORCHECK: C2RustUnnamed = 2;
pub const PTHREAD_MUTEX_RECURSIVE: C2RustUnnamed = 1;
pub const PTHREAD_MUTEX_NORMAL: C2RustUnnamed = 0;
pub const PTHREAD_MUTEX_ADAPTIVE_NP: C2RustUnnamed = 3;
pub const PTHREAD_MUTEX_ERRORCHECK_NP: C2RustUnnamed = 2;
pub const PTHREAD_MUTEX_RECURSIVE_NP: C2RustUnnamed = 1;
pub const PTHREAD_MUTEX_TIMED_NP: C2RustUnnamed = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct aes_key_st {
    pub rd_key: [libc::c_uint; 60],
    pub rounds: libc::c_int,
}
pub type AES_KEY = aes_key_st;
pub type uint8_t = __uint8_t;
pub type uint16_t = __uint16_t;
pub type uint32_t = __uint32_t;
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct stream_cfg {
    pub aesiv: [uint8_t; 16],
    pub aeskey: [uint8_t; 16],
    pub fmtp: [int32_t; 12],
}
pub type seq_t = uint16_t;
pub type abuf_t = audio_buffer_entry;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct audio_buffer_entry {
    pub ready: libc::c_int,
    pub data: *mut libc::c_short,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct biquad_t {
    pub hist: [libc::c_double; 2],
    pub a: [libc::c_double; 2],
    pub b: [libc::c_double; 3],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct alac_file {
    pub input_buffer: *mut libc::c_uchar,
    pub input_buffer_bitaccumulator: libc::c_int,
    pub samplesize: libc::c_int,
    pub numchannels: libc::c_int,
    pub bytespersample: libc::c_int,
    pub predicterror_buffer_a: *mut int32_t,
    pub predicterror_buffer_b: *mut int32_t,
    pub outputsamples_buffer_a: *mut int32_t,
    pub outputsamples_buffer_b: *mut int32_t,
    pub uncompressed_bytes_buffer_a: *mut int32_t,
    pub uncompressed_bytes_buffer_b: *mut int32_t,
    pub setinfo_max_samples_per_frame: uint32_t,
    pub setinfo_7a: uint8_t,
    pub setinfo_sample_size: uint8_t,
    pub setinfo_rice_historymult: uint8_t,
    pub setinfo_rice_initialhistory: uint8_t,
    pub setinfo_rice_kmodifier: uint8_t,
    pub setinfo_7f: uint8_t,
    pub setinfo_80: uint16_t,
    pub setinfo_82: uint32_t,
    pub setinfo_86: uint32_t,
    pub setinfo_8a_rate: uint32_t,
}
#[inline]
unsafe extern "C" fn seq_diff(mut a: seq_t, mut b: seq_t) -> uint16_t {
    let mut diff: int16_t = (b as libc::c_int - a as libc::c_int) as int16_t;
    return diff as uint16_t;
}
static mut aesiv: *mut libc::c_uchar = 0 as *const libc::c_uchar as *mut libc::c_uchar;
static mut aes: AES_KEY = AES_KEY {
    rd_key: [0; 60],
    rounds: 0,
};
static mut sampling_rate: libc::c_int = 0;
static mut frame_size: libc::c_int = 0;
static mut player_thread: pthread_t = 0;
static mut please_stop: libc::c_int = 0;
static mut decoder_info: *mut alac_file = 0 as *const alac_file as *mut alac_file;
static mut volume: libc::c_double = 1.0f64;
static mut fix_volume: libc::c_int = 0x10000 as libc::c_int;
static mut vol_mutex: pthread_mutex_t = pthread_mutex_t {
    __data: {
        let mut init = __pthread_mutex_s {
            __lock: 0 as libc::c_int,
            __count: 0 as libc::c_int as libc::c_uint,
            __owner: 0 as libc::c_int,
            __nusers: 0 as libc::c_int as libc::c_uint,
            __kind: PTHREAD_MUTEX_TIMED_NP as libc::c_int,
            __spins: 0 as libc::c_int as libc::c_short,
            __elision: 0 as libc::c_int as libc::c_short,
            __list: {
                let mut init = __pthread_internal_list {
                    __prev: 0 as *const __pthread_internal_list
                        as *mut __pthread_internal_list,
                    __next: 0 as *const __pthread_internal_list
                        as *mut __pthread_internal_list,
                };
                init
            },
        };
        init
    },
};
static mut audio_buffer: [abuf_t; 512] = [abuf_t {
    ready: 0,
    data: 0 as *const libc::c_short as *mut libc::c_short,
}; 512];
static mut ab_read: seq_t = 0;
static mut ab_write: seq_t = 0;
static mut ab_buffering: libc::c_int = 1 as libc::c_int;
static mut ab_synced: libc::c_int = 0 as libc::c_int;
static mut ab_mutex: pthread_mutex_t = pthread_mutex_t {
    __data: {
        let mut init = __pthread_mutex_s {
            __lock: 0 as libc::c_int,
            __count: 0 as libc::c_int as libc::c_uint,
            __owner: 0 as libc::c_int,
            __nusers: 0 as libc::c_int as libc::c_uint,
            __kind: PTHREAD_MUTEX_TIMED_NP as libc::c_int,
            __spins: 0 as libc::c_int as libc::c_short,
            __elision: 0 as libc::c_int as libc::c_short,
            __list: {
                let mut init = __pthread_internal_list {
                    __prev: 0 as *const __pthread_internal_list
                        as *mut __pthread_internal_list,
                    __next: 0 as *const __pthread_internal_list
                        as *mut __pthread_internal_list,
                };
                init
            },
        };
        init
    },
};
unsafe extern "C" fn ab_resync() {
    let mut i: libc::c_int = 0;
    i = 0 as libc::c_int;
    while i < 512 as libc::c_int {
        audio_buffer[i as usize].ready = 0 as libc::c_int;
        i += 1;
        i;
    }
    ab_synced = 0 as libc::c_int;
    ab_buffering = 1 as libc::c_int;
}
#[inline]
unsafe extern "C" fn seq_order(mut a: seq_t, mut b: seq_t) -> libc::c_int {
    let mut d: libc::c_short = (b as libc::c_int - a as libc::c_int) as libc::c_short;
    return (d as libc::c_int > 0 as libc::c_int) as libc::c_int;
}
unsafe extern "C" fn alac_decode(
    mut dest: *mut libc::c_short,
    mut buf: *mut uint8_t,
    mut len: libc::c_int,
) {
    let mut packet: [libc::c_uchar; 2048] = [0; 2048];
    if len <= 2048 as libc::c_int {} else {
        __assert_fail(
            b"len<=MAX_PACKET\0" as *const u8 as *const libc::c_char,
            b"player.c\0" as *const u8 as *const libc::c_char,
            112 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 42],
                &[libc::c_char; 42],
            >(b"void alac_decode(short *, uint8_t *, int)\0"))
                .as_ptr(),
        );
    }
    'c_16564: {
        if len <= 2048 as libc::c_int {} else {
            __assert_fail(
                b"len<=MAX_PACKET\0" as *const u8 as *const libc::c_char,
                b"player.c\0" as *const u8 as *const libc::c_char,
                112 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 42],
                    &[libc::c_char; 42],
                >(b"void alac_decode(short *, uint8_t *, int)\0"))
                    .as_ptr(),
            );
        }
    };
    let mut iv: [libc::c_uchar; 16] = [0; 16];
    let mut aeslen: libc::c_int = len & !(0xf as libc::c_int);
    memcpy(
        iv.as_mut_ptr() as *mut libc::c_void,
        aesiv as *const libc::c_void,
        ::std::mem::size_of::<[libc::c_uchar; 16]>() as libc::c_ulong,
    );
    AES_cbc_encrypt(
        buf,
        packet.as_mut_ptr(),
        aeslen as size_t,
        &mut aes,
        iv.as_mut_ptr(),
        0 as libc::c_int,
    );
    memcpy(
        packet.as_mut_ptr().offset(aeslen as isize) as *mut libc::c_void,
        buf.offset(aeslen as isize) as *const libc::c_void,
        (len - aeslen) as libc::c_ulong,
    );
    let mut outsize: libc::c_int = 0;
    alac_decode_frame(
        decoder_info,
        packet.as_mut_ptr(),
        dest as *mut libc::c_void,
        &mut outsize,
    );
    if outsize == 4 as libc::c_int * frame_size {} else {
        __assert_fail(
            b"outsize == FRAME_BYTES(frame_size)\0" as *const u8 as *const libc::c_char,
            b"player.c\0" as *const u8 as *const libc::c_char,
            124 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 42],
                &[libc::c_char; 42],
            >(b"void alac_decode(short *, uint8_t *, int)\0"))
                .as_ptr(),
        );
    }
    'c_16434: {
        if outsize == 4 as libc::c_int * frame_size {} else {
            __assert_fail(
                b"outsize == FRAME_BYTES(frame_size)\0" as *const u8
                    as *const libc::c_char,
                b"player.c\0" as *const u8 as *const libc::c_char,
                124 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 42],
                    &[libc::c_char; 42],
                >(b"void alac_decode(short *, uint8_t *, int)\0"))
                    .as_ptr(),
            );
        }
    };
}
unsafe extern "C" fn init_decoder(mut fmtp: *mut int32_t) -> libc::c_int {
    let mut alac: *mut alac_file = 0 as *mut alac_file;
    frame_size = *fmtp.offset(1 as libc::c_int as isize);
    sampling_rate = *fmtp.offset(11 as libc::c_int as isize);
    let mut sample_size: libc::c_int = *fmtp.offset(3 as libc::c_int as isize);
    if sample_size != 16 as libc::c_int {
        die(
            b"only 16-bit samples supported!\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
        );
    }
    alac = alac_create(sample_size, 2 as libc::c_int);
    if alac.is_null() {
        return 1 as libc::c_int;
    }
    decoder_info = alac;
    (*alac).setinfo_max_samples_per_frame = frame_size as uint32_t;
    (*alac).setinfo_7a = *fmtp.offset(2 as libc::c_int as isize) as uint8_t;
    (*alac).setinfo_sample_size = sample_size as uint8_t;
    (*alac)
        .setinfo_rice_historymult = *fmtp.offset(4 as libc::c_int as isize) as uint8_t;
    (*alac)
        .setinfo_rice_initialhistory = *fmtp.offset(5 as libc::c_int as isize)
        as uint8_t;
    (*alac).setinfo_rice_kmodifier = *fmtp.offset(6 as libc::c_int as isize) as uint8_t;
    (*alac).setinfo_7f = *fmtp.offset(7 as libc::c_int as isize) as uint8_t;
    (*alac).setinfo_80 = *fmtp.offset(8 as libc::c_int as isize) as uint16_t;
    (*alac).setinfo_82 = *fmtp.offset(9 as libc::c_int as isize) as uint32_t;
    (*alac).setinfo_86 = *fmtp.offset(10 as libc::c_int as isize) as uint32_t;
    (*alac).setinfo_8a_rate = *fmtp.offset(11 as libc::c_int as isize) as uint32_t;
    alac_allocate_buffers(alac);
    return 0 as libc::c_int;
}
unsafe extern "C" fn free_decoder() {
    alac_free(decoder_info);
}
unsafe extern "C" fn init_buffer() {
    let mut i: libc::c_int = 0;
    i = 0 as libc::c_int;
    while i < 512 as libc::c_int {
        audio_buffer[i as usize]
            .data = malloc(
            (4 as libc::c_int * (frame_size + 3 as libc::c_int)) as libc::c_ulong,
        ) as *mut libc::c_short;
        i += 1;
        i;
    }
    ab_resync();
}
unsafe extern "C" fn free_buffer() {
    let mut i: libc::c_int = 0;
    i = 0 as libc::c_int;
    while i < 512 as libc::c_int {
        free(audio_buffer[i as usize].data as *mut libc::c_void);
        i += 1;
        i;
    }
}
pub unsafe extern "C" fn player_put_packet(
    mut seqno: seq_t,
    mut data: *mut uint8_t,
    mut len: libc::c_int,
) {
    let mut abuf: *mut abuf_t = 0 as *mut abuf_t;
    let mut buf_fill: int16_t = 0;
    pthread_mutex_lock(&mut ab_mutex);
    if ab_synced == 0 {
        debug(
            2 as libc::c_int,
            b"syncing to first seqno %04X\n\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
            seqno as libc::c_int,
        );
        ab_write = (seqno as libc::c_int - 1 as libc::c_int) as seq_t;
        ab_read = seqno;
        ab_synced = 1 as libc::c_int;
    }
    if seq_diff(ab_write, seqno) as libc::c_int == 1 as libc::c_int {
        abuf = audio_buffer
            .as_mut_ptr()
            .offset((seqno as libc::c_int % 512 as libc::c_int) as isize);
        ab_write = seqno;
    } else if seq_order(ab_write, seqno) != 0 {
        rtp_request_resend(
            (ab_write as libc::c_int + 1 as libc::c_int) as seq_t,
            (seqno as libc::c_int - 1 as libc::c_int) as seq_t,
        );
        abuf = audio_buffer
            .as_mut_ptr()
            .offset((seqno as libc::c_int % 512 as libc::c_int) as isize);
        ab_write = seqno;
    } else if seq_order(ab_read, seqno) != 0 {
        abuf = audio_buffer
            .as_mut_ptr()
            .offset((seqno as libc::c_int % 512 as libc::c_int) as isize);
    } else {
        debug(
            1 as libc::c_int,
            b"late packet %04X (%04X:%04X)\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
            seqno as libc::c_int,
            ab_read as libc::c_int,
            ab_write as libc::c_int,
        );
    }
    buf_fill = seq_diff(ab_read, ab_write) as int16_t;
    pthread_mutex_unlock(&mut ab_mutex);
    if !abuf.is_null() {
        alac_decode((*abuf).data, data, len);
        (*abuf).ready = 1 as libc::c_int;
    }
    pthread_mutex_lock(&mut ab_mutex);
    if ab_buffering != 0 && buf_fill as libc::c_int >= config.buffer_start_fill {
        debug(
            1 as libc::c_int,
            b"buffering over. starting play\n\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
        );
        ab_buffering = 0 as libc::c_int;
        bf_est_reset(buf_fill);
    }
    pthread_mutex_unlock(&mut ab_mutex);
}
unsafe extern "C" fn lcg_rand() -> libc::c_short {
    static mut lcg_prev: libc::c_ulong = 12345 as libc::c_int as libc::c_ulong;
    lcg_prev = lcg_prev
        .wrapping_mul(69069 as libc::c_int as libc::c_ulong)
        .wrapping_add(3 as libc::c_int as libc::c_ulong);
    return (lcg_prev & 0xffff as libc::c_int as libc::c_ulong) as libc::c_short;
}
#[inline]
unsafe extern "C" fn dithered_vol(mut sample: libc::c_short) -> libc::c_short {
    static mut rand_a: libc::c_short = 0;
    static mut rand_b: libc::c_short = 0;
    let mut out: libc::c_long = 0;
    out = sample as libc::c_long * fix_volume as libc::c_long;
    if fix_volume < 0x10000 as libc::c_int {
        rand_b = rand_a;
        rand_a = lcg_rand();
        out += rand_a as libc::c_long;
        out -= rand_b as libc::c_long;
    }
    return (out >> 16 as libc::c_int) as libc::c_short;
}
unsafe extern "C" fn biquad_init(
    mut bq: *mut biquad_t,
    mut a: *mut libc::c_double,
    mut b: *mut libc::c_double,
) {
    (*bq).hist[1 as libc::c_int as usize] = 0.0f64;
    (*bq).hist[0 as libc::c_int as usize] = (*bq).hist[1 as libc::c_int as usize];
    memcpy(
        ((*bq).a).as_mut_ptr() as *mut libc::c_void,
        a as *const libc::c_void,
        (2 as libc::c_int as libc::c_ulong)
            .wrapping_mul(::std::mem::size_of::<libc::c_double>() as libc::c_ulong),
    );
    memcpy(
        ((*bq).b).as_mut_ptr() as *mut libc::c_void,
        b as *const libc::c_void,
        (3 as libc::c_int as libc::c_ulong)
            .wrapping_mul(::std::mem::size_of::<libc::c_double>() as libc::c_ulong),
    );
}
unsafe extern "C" fn biquad_lpf(
    mut bq: *mut biquad_t,
    mut freq: libc::c_double,
    mut Q: libc::c_double,
) {
    let mut w0: libc::c_double = 2.0f64 * 3.14159265358979323846f64 * freq
        * frame_size as libc::c_double / sampling_rate as libc::c_double;
    let mut alpha: libc::c_double = sin(w0) / (2.0f64 * Q);
    let mut a_0: libc::c_double = 1.0f64 + alpha;
    let mut b: [libc::c_double; 3] = [0.; 3];
    let mut a: [libc::c_double; 2] = [0.; 2];
    b[0 as libc::c_int as usize] = (1.0f64 - cos(w0)) / (2.0f64 * a_0);
    b[1 as libc::c_int as usize] = (1.0f64 - cos(w0)) / a_0;
    b[2 as libc::c_int as usize] = b[0 as libc::c_int as usize];
    a[0 as libc::c_int as usize] = -2.0f64 * cos(w0) / a_0;
    a[1 as libc::c_int as usize] = (1 as libc::c_int as libc::c_double - alpha) / a_0;
    biquad_init(bq, a.as_mut_ptr(), b.as_mut_ptr());
}
unsafe extern "C" fn biquad_filt(
    mut bq: *mut biquad_t,
    mut in_0: libc::c_double,
) -> libc::c_double {
    let mut w: libc::c_double = in_0
        - (*bq).a[0 as libc::c_int as usize] * (*bq).hist[0 as libc::c_int as usize]
        - (*bq).a[1 as libc::c_int as usize] * (*bq).hist[1 as libc::c_int as usize];
    let mut out: libc::c_double = (*bq).b[1 as libc::c_int as usize]
        * (*bq).hist[0 as libc::c_int as usize]
        + (*bq).b[2 as libc::c_int as usize] * (*bq).hist[1 as libc::c_int as usize]
        + (*bq).b[0 as libc::c_int as usize] * w;
    (*bq).hist[1 as libc::c_int as usize] = (*bq).hist[0 as libc::c_int as usize];
    (*bq).hist[0 as libc::c_int as usize] = w;
    return out;
}
static mut bf_playback_rate: libc::c_double = 1.0f64;
static mut bf_est_drift: libc::c_double = 0.0f64;
static mut bf_drift_lpf: biquad_t = biquad_t {
    hist: [0.; 2],
    a: [0.; 2],
    b: [0.; 3],
};
static mut bf_est_err: libc::c_double = 0.0f64;
static mut bf_last_err: libc::c_double = 0.;
static mut bf_err_lpf: biquad_t = biquad_t {
    hist: [0.; 2],
    a: [0.; 2],
    b: [0.; 3],
};
static mut bf_err_deriv_lpf: biquad_t = biquad_t {
    hist: [0.; 2],
    a: [0.; 2],
    b: [0.; 3],
};
static mut desired_fill: libc::c_double = 0.;
static mut fill_count: libc::c_int = 0;
unsafe extern "C" fn bf_est_reset(mut fill: libc::c_short) {
    biquad_lpf(&mut bf_drift_lpf, 1.0f64 / 180.0f64, 0.3f64);
    biquad_lpf(&mut bf_err_lpf, 1.0f64 / 10.0f64, 0.25f64);
    biquad_lpf(&mut bf_err_deriv_lpf, 1.0f64 / 2.0f64, 0.2f64);
    fill_count = 0 as libc::c_int;
    bf_playback_rate = 1.0f64;
    bf_last_err = 0 as libc::c_int as libc::c_double;
    bf_est_err = bf_last_err;
    fill_count = 0 as libc::c_int;
    desired_fill = fill_count as libc::c_double;
}
unsafe extern "C" fn bf_est_update(mut fill: libc::c_short) {
    if fill_count < 1000 as libc::c_int {
        desired_fill += fill as libc::c_double / 1000.0f64;
        fill_count += 1;
        fill_count;
        return;
    } else if fill_count == 1000 as libc::c_int {
        debug(
            1 as libc::c_int,
            b"established desired fill of %f frames, so output chain buffered about %f frames\n\0"
                as *const u8 as *const libc::c_char as *mut libc::c_char,
            desired_fill,
            config.buffer_start_fill as libc::c_double - desired_fill,
        );
        fill_count += 1;
        fill_count;
    }
    let mut buf_delta: libc::c_double = fill as libc::c_int as libc::c_double
        - desired_fill;
    bf_est_err = biquad_filt(&mut bf_err_lpf, buf_delta);
    let mut err_deriv: libc::c_double = biquad_filt(
        &mut bf_err_deriv_lpf,
        bf_est_err - bf_last_err,
    );
    let mut adj_error: libc::c_double = 1e-4f64 * bf_est_err;
    bf_est_drift = biquad_filt(
        &mut bf_drift_lpf,
        1e-1f64 * (adj_error + err_deriv) + bf_est_drift,
    );
    debug(
        3 as libc::c_int,
        b"bf %d err %f drift %f desiring %f ed %f estd %f\n\0" as *const u8
            as *const libc::c_char as *mut libc::c_char,
        fill as libc::c_int,
        bf_est_err,
        bf_est_drift,
        desired_fill,
        err_deriv,
        err_deriv + adj_error,
    );
    bf_playback_rate = 1.0f64 + adj_error + bf_est_drift;
    bf_last_err = bf_est_err;
}
unsafe extern "C" fn buffer_get_frame() -> *mut libc::c_short {
    let mut buf_fill: int16_t = 0;
    let mut read: seq_t = 0;
    let mut next: seq_t = 0;
    let mut abuf: *mut abuf_t = 0 as *mut abuf_t;
    let mut i: libc::c_int = 0;
    if ab_buffering != 0 {
        return 0 as *mut libc::c_short;
    }
    pthread_mutex_lock(&mut ab_mutex);
    buf_fill = seq_diff(ab_read, ab_write) as int16_t;
    if (buf_fill as libc::c_int) < 1 as libc::c_int || ab_synced == 0 {
        if (buf_fill as libc::c_int) < 1 as libc::c_int {
            warn(
                b"underrun.\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            );
        }
        ab_buffering = 1 as libc::c_int;
        pthread_mutex_unlock(&mut ab_mutex);
        return 0 as *mut libc::c_short;
    }
    if buf_fill as libc::c_int >= 512 as libc::c_int {
        warn(b"overrun.\0" as *const u8 as *const libc::c_char as *mut libc::c_char);
        ab_read = (ab_write as libc::c_int - config.buffer_start_fill) as seq_t;
    }
    read = ab_read;
    ab_read = ab_read.wrapping_add(1);
    ab_read;
    buf_fill = seq_diff(ab_read, ab_write) as int16_t;
    bf_est_update(buf_fill);
    if ab_buffering == 0 {
        i = 16 as libc::c_int;
        while i < config.buffer_start_fill / 2 as libc::c_int {
            next = (ab_read as libc::c_int + i) as seq_t;
            abuf = audio_buffer
                .as_mut_ptr()
                .offset((next as libc::c_int % 512 as libc::c_int) as isize);
            if (*abuf).ready == 0 {
                rtp_request_resend(next, next);
            }
            i = i * 2 as libc::c_int;
        }
    }
    let mut curframe: *mut abuf_t = audio_buffer
        .as_mut_ptr()
        .offset((read as libc::c_int % 512 as libc::c_int) as isize);
    if (*curframe).ready == 0 {
        debug(
            1 as libc::c_int,
            b"missing frame %04X.\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
            read as libc::c_int,
        );
        memset(
            (*curframe).data as *mut libc::c_void,
            0 as libc::c_int,
            (4 as libc::c_int * frame_size) as libc::c_ulong,
        );
    }
    (*curframe).ready = 0 as libc::c_int;
    pthread_mutex_unlock(&mut ab_mutex);
    return (*curframe).data;
}
unsafe extern "C" fn stuff_buffer(
    mut playback_rate: libc::c_double,
    mut inptr: *mut libc::c_short,
    mut outptr: *mut libc::c_short,
) -> libc::c_int {
    let mut i: libc::c_int = 0;
    let mut stuffsamp: libc::c_int = frame_size;
    let mut stuff: libc::c_int = 0 as libc::c_int;
    let mut p_stuff: libc::c_double = 0.;
    p_stuff = 1.0f64
        - pow(1.0f64 - fabs(playback_rate - 1.0f64), frame_size as libc::c_double);
    if (rand() as libc::c_double) < p_stuff * 2147483647 as libc::c_int as libc::c_double
    {
        stuff = if playback_rate > 1.0f64 {
            -(1 as libc::c_int)
        } else {
            1 as libc::c_int
        };
        stuffsamp = rand() % (frame_size - 1 as libc::c_int);
    }
    pthread_mutex_lock(&mut vol_mutex);
    i = 0 as libc::c_int;
    while i < stuffsamp {
        let fresh0 = inptr;
        inptr = inptr.offset(1);
        let fresh1 = outptr;
        outptr = outptr.offset(1);
        *fresh1 = dithered_vol(*fresh0);
        let fresh2 = inptr;
        inptr = inptr.offset(1);
        let fresh3 = outptr;
        outptr = outptr.offset(1);
        *fresh3 = dithered_vol(*fresh2);
        i += 1;
        i;
    }
    if stuff != 0 {
        if stuff == 1 as libc::c_int {
            debug(
                2 as libc::c_int,
                b"+++++++++\n\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            );
            let fresh4 = outptr;
            outptr = outptr.offset(1);
            *fresh4 = dithered_vol(
                (*inptr.offset(-(2 as libc::c_int) as isize) as libc::c_long
                    + *inptr.offset(0 as libc::c_int as isize) as libc::c_long
                    >> 1 as libc::c_int) as libc::c_short,
            );
            let fresh5 = outptr;
            outptr = outptr.offset(1);
            *fresh5 = dithered_vol(
                (*inptr.offset(-(1 as libc::c_int) as isize) as libc::c_long
                    + *inptr.offset(1 as libc::c_int as isize) as libc::c_long
                    >> 1 as libc::c_int) as libc::c_short,
            );
        } else if stuff == -(1 as libc::c_int) {
            debug(
                2 as libc::c_int,
                b"---------\n\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            );
            inptr = inptr.offset(1);
            inptr;
            inptr = inptr.offset(1);
            inptr;
        }
        i = stuffsamp;
        while i < frame_size + stuff {
            let fresh6 = inptr;
            inptr = inptr.offset(1);
            let fresh7 = outptr;
            outptr = outptr.offset(1);
            *fresh7 = dithered_vol(*fresh6);
            let fresh8 = inptr;
            inptr = inptr.offset(1);
            let fresh9 = outptr;
            outptr = outptr.offset(1);
            *fresh9 = dithered_vol(*fresh8);
            i += 1;
            i;
        }
    }
    pthread_mutex_unlock(&mut vol_mutex);
    return frame_size + stuff;
}
unsafe extern "C" fn player_thread_func(
    mut arg: *mut libc::c_void,
) -> *mut libc::c_void {
    let mut play_samples: libc::c_int = 0;
    let mut inbuf: *mut libc::c_short = 0 as *mut libc::c_short;
    let mut outbuf: *mut libc::c_short = 0 as *mut libc::c_short;
    let mut silence: *mut libc::c_short = 0 as *mut libc::c_short;
    outbuf = malloc(
        (4 as libc::c_int * (frame_size + 3 as libc::c_int)) as libc::c_ulong,
    ) as *mut libc::c_short;
    silence = malloc(
        (4 as libc::c_int * (frame_size + 3 as libc::c_int)) as libc::c_ulong,
    ) as *mut libc::c_short;
    memset(
        silence as *mut libc::c_void,
        0 as libc::c_int,
        (4 as libc::c_int * (frame_size + 3 as libc::c_int)) as libc::c_ulong,
    );
    while please_stop == 0 {
        inbuf = buffer_get_frame();
        if inbuf.is_null() {
            inbuf = silence;
        }
        play_samples = stuff_buffer(bf_playback_rate, inbuf, outbuf);
        ((*config.output).play).unwrap()(outbuf, play_samples);
    }
    return 0 as *mut libc::c_void;
}
pub unsafe extern "C" fn player_volume(mut f: libc::c_double) {
    let mut linear_volume: libc::c_double = pow(10.0f64, 0.05f64 * f);
    if ((*config.output).volume).is_some() {
        ((*config.output).volume).unwrap()(linear_volume);
    } else {
        pthread_mutex_lock(&mut vol_mutex);
        volume = linear_volume;
        fix_volume = (65536.0f64 * volume) as libc::c_int;
        pthread_mutex_unlock(&mut vol_mutex);
    };
}
pub unsafe extern "C" fn player_flush() {
    pthread_mutex_lock(&mut ab_mutex);
    ab_resync();
    pthread_mutex_unlock(&mut ab_mutex);
}
pub unsafe extern "C" fn player_play(mut stream: *mut stream_cfg) -> libc::c_int {
    if config.buffer_start_fill > 512 as libc::c_int {
        die(
            b"specified buffer starting fill %d > buffer size %d\0" as *const u8
                as *const libc::c_char as *mut libc::c_char,
            config.buffer_start_fill,
            512 as libc::c_int,
        );
    }
    AES_set_decrypt_key(((*stream).aeskey).as_mut_ptr(), 128 as libc::c_int, &mut aes);
    aesiv = ((*stream).aesiv).as_mut_ptr();
    init_decoder(((*stream).fmtp).as_mut_ptr());
    init_buffer();
    please_stop = 0 as libc::c_int;
    command_start();
    ((*config.output).start).unwrap()(sampling_rate);
    pthread_create(
        &mut player_thread,
        0 as *const pthread_attr_t,
        Some(
            player_thread_func
                as unsafe extern "C" fn(*mut libc::c_void) -> *mut libc::c_void,
        ),
        0 as *mut libc::c_void,
    );
    return 0 as libc::c_int;
}
pub unsafe extern "C" fn player_stop() {
    please_stop = 1 as libc::c_int;
    pthread_join(player_thread, 0 as *mut *mut libc::c_void);
    ((*config.output).stop).unwrap()();
    command_stop();
    free_buffer();
    free_decoder();
}
