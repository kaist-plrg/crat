use ::libc;
use ::c2rust_bitfields;
extern "C" {
    pub type AVBuffer;
    pub type AVDictionary;
    pub type AVCodecInternal;
    pub type MpegEncContext;
    pub type AVCodecHWConfigInternal;
    pub type AVCodecDefault;
    pub type AVFilterCommand;
    pub type AVFilterInternal;
    pub type AVFilterChannelLayouts;
    pub type AVFilterFormats;
    pub type AVFilterPad;
    pub type AVFilterGraphInternal;
    pub type ao_device;
    pub type AVFormatInternal;
    pub type AVStreamInternal;
    pub type AVDeviceCapabilitiesQuery;
    pub type AVDeviceInfoList;
    pub type AVCodecTag;
    fn memset(
        _: *mut libc::c_void,
        _: libc::c_int,
        _: libc::c_ulong,
    ) -> *mut libc::c_void;
    fn __xstat(
        __ver: libc::c_int,
        __filename: *const libc::c_char,
        __stat_buf: *mut stat,
    ) -> libc::c_int;
    fn avcodec_alloc_context3(codec: *const AVCodec) -> *mut AVCodecContext;
    fn avcodec_parameters_to_context(
        codec: *mut AVCodecContext,
        par: *const AVCodecParameters,
    ) -> libc::c_int;
    fn avcodec_open2(
        avctx: *mut AVCodecContext,
        codec: *const AVCodec,
        options: *mut *mut AVDictionary,
    ) -> libc::c_int;
    fn avcodec_close(avctx: *mut AVCodecContext) -> libc::c_int;
    fn av_packet_alloc() -> *mut AVPacket;
    fn av_packet_free(pkt: *mut *mut AVPacket);
    fn av_packet_unref(pkt: *mut AVPacket);
    fn avcodec_find_decoder(id: AVCodecID) -> *mut AVCodec;
    fn av_strerror(
        errnum: libc::c_int,
        errbuf: *mut libc::c_char,
        errbuf_size: size_t,
    ) -> libc::c_int;
    fn av_log_set_level(level: libc::c_int);
    fn av_get_sample_fmt_name(sample_fmt: AVSampleFormat) -> *const libc::c_char;
    fn av_get_bytes_per_sample(sample_fmt: AVSampleFormat) -> libc::c_int;
    fn av_get_channel_layout_nb_channels(channel_layout: uint64_t) -> libc::c_int;
    fn av_dict_set(
        pm: *mut *mut AVDictionary,
        key: *const libc::c_char,
        value: *const libc::c_char,
        flags: libc::c_int,
    ) -> libc::c_int;
    fn av_frame_alloc() -> *mut AVFrame;
    fn av_frame_free(frame: *mut *mut AVFrame);
    fn av_frame_unref(frame: *mut AVFrame);
    fn snprintf(
        _: *mut libc::c_char,
        _: libc::c_ulong,
        _: *const libc::c_char,
        _: ...
    ) -> libc::c_int;
    fn avcodec_send_packet(
        avctx: *mut AVCodecContext,
        avpkt: *const AVPacket,
    ) -> libc::c_int;
    fn avcodec_receive_frame(
        avctx: *mut AVCodecContext,
        frame: *mut AVFrame,
    ) -> libc::c_int;
    fn avfilter_link(
        src: *mut AVFilterContext,
        srcpad: libc::c_uint,
        dst: *mut AVFilterContext,
        dstpad: libc::c_uint,
    ) -> libc::c_int;
    fn avfilter_get_by_name(name: *const libc::c_char) -> *const AVFilter;
    fn avfilter_graph_alloc() -> *mut AVFilterGraph;
    fn avfilter_graph_create_filter(
        filt_ctx: *mut *mut AVFilterContext,
        filt: *const AVFilter,
        name: *const libc::c_char,
        args: *const libc::c_char,
        opaque: *mut libc::c_void,
        graph_ctx: *mut AVFilterGraph,
    ) -> libc::c_int;
    fn avfilter_graph_config(
        graphctx: *mut AVFilterGraph,
        log_ctx: *mut libc::c_void,
    ) -> libc::c_int;
    fn avfilter_graph_free(graph: *mut *mut AVFilterGraph);
    fn avfilter_graph_send_command(
        graph: *mut AVFilterGraph,
        target: *const libc::c_char,
        cmd: *const libc::c_char,
        arg: *const libc::c_char,
        res: *mut libc::c_char,
        res_len: libc::c_int,
        flags: libc::c_int,
    ) -> libc::c_int;
    fn av_buffersink_get_time_base(ctx: *const AVFilterContext) -> AVRational;
    fn av_buffersink_get_frame(
        ctx: *mut AVFilterContext,
        frame: *mut AVFrame,
    ) -> libc::c_int;
    fn av_buffersrc_write_frame(
        ctx: *mut AVFilterContext,
        frame: *const AVFrame,
    ) -> libc::c_int;
    fn av_buffersrc_add_frame(
        ctx: *mut AVFilterContext,
        frame: *mut AVFrame,
    ) -> libc::c_int;
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
    fn pthread_mutex_init(
        __mutex: *mut pthread_mutex_t,
        __mutexattr: *const pthread_mutexattr_t,
    ) -> libc::c_int;
    fn pthread_mutex_destroy(__mutex: *mut pthread_mutex_t) -> libc::c_int;
    fn pthread_mutex_lock(__mutex: *mut pthread_mutex_t) -> libc::c_int;
    fn pthread_mutex_unlock(__mutex: *mut pthread_mutex_t) -> libc::c_int;
    fn pthread_cond_init(
        __cond: *mut pthread_cond_t,
        __cond_attr: *const pthread_condattr_t,
    ) -> libc::c_int;
    fn pthread_cond_destroy(__cond: *mut pthread_cond_t) -> libc::c_int;
    fn pthread_cond_broadcast(__cond: *mut pthread_cond_t) -> libc::c_int;
    fn pthread_cond_wait(
        __cond: *mut pthread_cond_t,
        __mutex: *mut pthread_mutex_t,
    ) -> libc::c_int;
    fn ao_initialize();
    fn ao_shutdown();
    fn ao_open_live(
        driver_id: libc::c_int,
        format: *mut ao_sample_format,
        option: *mut ao_option,
    ) -> *mut ao_device;
    fn ao_open_file(
        driver_id: libc::c_int,
        filename: *const libc::c_char,
        overwrite: libc::c_int,
        format: *mut ao_sample_format,
        option: *mut ao_option,
    ) -> *mut ao_device;
    fn ao_play(
        device: *mut ao_device,
        output_samples: *mut libc::c_char,
        num_bytes: uint_32,
    ) -> libc::c_int;
    fn ao_close(device: *mut ao_device) -> libc::c_int;
    fn ao_driver_id(short_name: *const libc::c_char) -> libc::c_int;
    fn ao_default_driver_id() -> libc::c_int;
    fn avformat_alloc_context() -> *mut AVFormatContext;
    fn avformat_open_input(
        ps: *mut *mut AVFormatContext,
        url: *const libc::c_char,
        fmt: *mut AVInputFormat,
        options: *mut *mut AVDictionary,
    ) -> libc::c_int;
    fn avformat_find_stream_info(
        ic: *mut AVFormatContext,
        options: *mut *mut AVDictionary,
    ) -> libc::c_int;
    fn av_find_best_stream(
        ic: *mut AVFormatContext,
        type_0: AVMediaType,
        wanted_stream_nb: libc::c_int,
        related_stream: libc::c_int,
        decoder_ret: *mut *mut AVCodec,
        flags: libc::c_int,
    ) -> libc::c_int;
    fn av_read_frame(s: *mut AVFormatContext, pkt: *mut AVPacket) -> libc::c_int;
    fn av_seek_frame(
        s: *mut AVFormatContext,
        stream_index: libc::c_int,
        timestamp: int64_t,
        flags: libc::c_int,
    ) -> libc::c_int;
    fn avformat_close_input(s: *mut *mut AVFormatContext);
    fn BarUiMsg(_: *const BarSettings_t, _: BarUiMsg_t, _: *const libc::c_char, _: ...);
}
pub type __int8_t = libc::c_schar;
pub type __uint8_t = libc::c_uchar;
pub type __uint16_t = libc::c_ushort;
pub type __uint32_t = libc::c_uint;
pub type __int64_t = libc::c_long;
pub type __uint64_t = libc::c_ulong;
pub type __dev_t = libc::c_ulong;
pub type __uid_t = libc::c_uint;
pub type __gid_t = libc::c_uint;
pub type __ino_t = libc::c_ulong;
pub type __mode_t = libc::c_uint;
pub type __nlink_t = libc::c_ulong;
pub type __off_t = libc::c_long;
pub type __time_t = libc::c_long;
pub type __blksize_t = libc::c_long;
pub type __blkcnt_t = libc::c_long;
pub type __syscall_slong_t = libc::c_long;
pub type __sig_atomic_t = libc::c_int;
pub type size_t = libc::c_ulong;
pub type int8_t = __int8_t;
pub type int64_t = __int64_t;
pub type uint8_t = __uint8_t;
pub type uint16_t = __uint16_t;
pub type uint32_t = __uint32_t;
pub type uint64_t = __uint64_t;
pub type uintptr_t = libc::c_ulong;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct timespec {
    pub tv_sec: __time_t,
    pub tv_nsec: __syscall_slong_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct stat {
    pub st_dev: __dev_t,
    pub st_ino: __ino_t,
    pub st_nlink: __nlink_t,
    pub st_mode: __mode_t,
    pub st_uid: __uid_t,
    pub st_gid: __gid_t,
    pub __pad0: libc::c_int,
    pub st_rdev: __dev_t,
    pub st_size: __off_t,
    pub st_blksize: __blksize_t,
    pub st_blocks: __blkcnt_t,
    pub st_atim: timespec,
    pub st_mtim: timespec,
    pub st_ctim: timespec,
    pub __glibc_reserved: [__syscall_slong_t; 3],
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct __pthread_cond_s {
    pub c2rust_unnamed: C2RustUnnamed_1,
    pub c2rust_unnamed_0: C2RustUnnamed,
    pub __g_refs: [libc::c_uint; 2],
    pub __g_size: [libc::c_uint; 2],
    pub __g1_orig_size: libc::c_uint,
    pub __wrefs: libc::c_uint,
    pub __g_signals: [libc::c_uint; 2],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed {
    pub __g1_start: libc::c_ulonglong,
    pub __g1_start32: C2RustUnnamed_0,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_0 {
    pub __low: libc::c_uint,
    pub __high: libc::c_uint,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_1 {
    pub __wseq: libc::c_ulonglong,
    pub __wseq32: C2RustUnnamed_2,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_2 {
    pub __low: libc::c_uint,
    pub __high: libc::c_uint,
}
pub type pthread_t = libc::c_ulong;
#[derive(Copy, Clone)]
#[repr(C)]
pub union pthread_mutexattr_t {
    pub __size: [libc::c_char; 4],
    pub __align: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union pthread_condattr_t {
    pub __size: [libc::c_char; 4],
    pub __align: libc::c_int,
}
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
#[derive(Copy, Clone)]
#[repr(C)]
pub union pthread_cond_t {
    pub __data: __pthread_cond_s,
    pub __size: [libc::c_char; 48],
    pub __align: libc::c_longlong,
}
pub type AVMediaType = libc::c_int;
pub const AVMEDIA_TYPE_NB: AVMediaType = 5;
pub const AVMEDIA_TYPE_ATTACHMENT: AVMediaType = 4;
pub const AVMEDIA_TYPE_SUBTITLE: AVMediaType = 3;
pub const AVMEDIA_TYPE_DATA: AVMediaType = 2;
pub const AVMEDIA_TYPE_AUDIO: AVMediaType = 1;
pub const AVMEDIA_TYPE_VIDEO: AVMediaType = 0;
pub const AVMEDIA_TYPE_UNKNOWN: AVMediaType = -1;
pub type AVPictureType = libc::c_uint;
pub const AV_PICTURE_TYPE_BI: AVPictureType = 7;
pub const AV_PICTURE_TYPE_SP: AVPictureType = 6;
pub const AV_PICTURE_TYPE_SI: AVPictureType = 5;
pub const AV_PICTURE_TYPE_S: AVPictureType = 4;
pub const AV_PICTURE_TYPE_B: AVPictureType = 3;
pub const AV_PICTURE_TYPE_P: AVPictureType = 2;
pub const AV_PICTURE_TYPE_I: AVPictureType = 1;
pub const AV_PICTURE_TYPE_NONE: AVPictureType = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct AVRational {
    pub num: libc::c_int,
    pub den: libc::c_int,
}
pub type AVClassCategory = libc::c_uint;
pub const AV_CLASS_CATEGORY_NB: AVClassCategory = 46;
pub const AV_CLASS_CATEGORY_DEVICE_INPUT: AVClassCategory = 45;
pub const AV_CLASS_CATEGORY_DEVICE_OUTPUT: AVClassCategory = 44;
pub const AV_CLASS_CATEGORY_DEVICE_AUDIO_INPUT: AVClassCategory = 43;
pub const AV_CLASS_CATEGORY_DEVICE_AUDIO_OUTPUT: AVClassCategory = 42;
pub const AV_CLASS_CATEGORY_DEVICE_VIDEO_INPUT: AVClassCategory = 41;
pub const AV_CLASS_CATEGORY_DEVICE_VIDEO_OUTPUT: AVClassCategory = 40;
pub const AV_CLASS_CATEGORY_SWRESAMPLER: AVClassCategory = 10;
pub const AV_CLASS_CATEGORY_SWSCALER: AVClassCategory = 9;
pub const AV_CLASS_CATEGORY_BITSTREAM_FILTER: AVClassCategory = 8;
pub const AV_CLASS_CATEGORY_FILTER: AVClassCategory = 7;
pub const AV_CLASS_CATEGORY_DECODER: AVClassCategory = 6;
pub const AV_CLASS_CATEGORY_ENCODER: AVClassCategory = 5;
pub const AV_CLASS_CATEGORY_DEMUXER: AVClassCategory = 4;
pub const AV_CLASS_CATEGORY_MUXER: AVClassCategory = 3;
pub const AV_CLASS_CATEGORY_OUTPUT: AVClassCategory = 2;
pub const AV_CLASS_CATEGORY_INPUT: AVClassCategory = 1;
pub const AV_CLASS_CATEGORY_NA: AVClassCategory = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct AVOptionRanges {
    pub range: *mut *mut AVOptionRange,
    pub nb_ranges: libc::c_int,
    pub nb_components: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct AVOptionRange {
    pub str_0: *const libc::c_char,
    pub value_min: libc::c_double,
    pub value_max: libc::c_double,
    pub component_min: libc::c_double,
    pub component_max: libc::c_double,
    pub is_range: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct AVClass {
    pub class_name: *const libc::c_char,
    pub item_name: Option::<
        unsafe extern "C" fn(*mut libc::c_void) -> *const libc::c_char,
    >,
    pub option: *const AVOption,
    pub version: libc::c_int,
    pub log_level_offset_offset: libc::c_int,
    pub parent_log_context_offset: libc::c_int,
    pub child_next: Option::<
        unsafe extern "C" fn(*mut libc::c_void, *mut libc::c_void) -> *mut libc::c_void,
    >,
    pub child_class_next: Option::<
        unsafe extern "C" fn(*const AVClass) -> *const AVClass,
    >,
    pub category: AVClassCategory,
    pub get_category: Option::<
        unsafe extern "C" fn(*mut libc::c_void) -> AVClassCategory,
    >,
    pub query_ranges: Option::<
        unsafe extern "C" fn(
            *mut *mut AVOptionRanges,
            *mut libc::c_void,
            *const libc::c_char,
            libc::c_int,
        ) -> libc::c_int,
    >,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct AVOption {
    pub name: *const libc::c_char,
    pub help: *const libc::c_char,
    pub offset: libc::c_int,
    pub type_0: AVOptionType,
    pub default_val: C2RustUnnamed_3,
    pub min: libc::c_double,
    pub max: libc::c_double,
    pub flags: libc::c_int,
    pub unit: *const libc::c_char,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_3 {
    pub i64_0: int64_t,
    pub dbl: libc::c_double,
    pub str_0: *const libc::c_char,
    pub q: AVRational,
}
pub type AVOptionType = libc::c_uint;
pub const AV_OPT_TYPE_BOOL: AVOptionType = 18;
pub const AV_OPT_TYPE_CHANNEL_LAYOUT: AVOptionType = 17;
pub const AV_OPT_TYPE_COLOR: AVOptionType = 16;
pub const AV_OPT_TYPE_DURATION: AVOptionType = 15;
pub const AV_OPT_TYPE_VIDEO_RATE: AVOptionType = 14;
pub const AV_OPT_TYPE_SAMPLE_FMT: AVOptionType = 13;
pub const AV_OPT_TYPE_PIXEL_FMT: AVOptionType = 12;
pub const AV_OPT_TYPE_IMAGE_SIZE: AVOptionType = 11;
pub const AV_OPT_TYPE_CONST: AVOptionType = 10;
pub const AV_OPT_TYPE_UINT64: AVOptionType = 9;
pub const AV_OPT_TYPE_DICT: AVOptionType = 8;
pub const AV_OPT_TYPE_BINARY: AVOptionType = 7;
pub const AV_OPT_TYPE_RATIONAL: AVOptionType = 6;
pub const AV_OPT_TYPE_STRING: AVOptionType = 5;
pub const AV_OPT_TYPE_FLOAT: AVOptionType = 4;
pub const AV_OPT_TYPE_DOUBLE: AVOptionType = 3;
pub const AV_OPT_TYPE_INT64: AVOptionType = 2;
pub const AV_OPT_TYPE_INT: AVOptionType = 1;
pub const AV_OPT_TYPE_FLAGS: AVOptionType = 0;
pub type AVPixelFormat = libc::c_int;
pub const AV_PIX_FMT_NB: AVPixelFormat = 193;
pub const AV_PIX_FMT_NV42: AVPixelFormat = 192;
pub const AV_PIX_FMT_NV24: AVPixelFormat = 191;
pub const AV_PIX_FMT_YUVA444P12LE: AVPixelFormat = 190;
pub const AV_PIX_FMT_YUVA444P12BE: AVPixelFormat = 189;
pub const AV_PIX_FMT_YUVA422P12LE: AVPixelFormat = 188;
pub const AV_PIX_FMT_YUVA422P12BE: AVPixelFormat = 187;
pub const AV_PIX_FMT_GRAYF32LE: AVPixelFormat = 186;
pub const AV_PIX_FMT_GRAYF32BE: AVPixelFormat = 185;
pub const AV_PIX_FMT_GRAY14LE: AVPixelFormat = 184;
pub const AV_PIX_FMT_GRAY14BE: AVPixelFormat = 183;
pub const AV_PIX_FMT_OPENCL: AVPixelFormat = 182;
pub const AV_PIX_FMT_DRM_PRIME: AVPixelFormat = 181;
pub const AV_PIX_FMT_GBRAPF32LE: AVPixelFormat = 180;
pub const AV_PIX_FMT_GBRAPF32BE: AVPixelFormat = 179;
pub const AV_PIX_FMT_GBRPF32LE: AVPixelFormat = 178;
pub const AV_PIX_FMT_GBRPF32BE: AVPixelFormat = 177;
pub const AV_PIX_FMT_GRAY9LE: AVPixelFormat = 176;
pub const AV_PIX_FMT_GRAY9BE: AVPixelFormat = 175;
pub const AV_PIX_FMT_D3D11: AVPixelFormat = 174;
pub const AV_PIX_FMT_P016BE: AVPixelFormat = 173;
pub const AV_PIX_FMT_P016LE: AVPixelFormat = 172;
pub const AV_PIX_FMT_GRAY10LE: AVPixelFormat = 171;
pub const AV_PIX_FMT_GRAY10BE: AVPixelFormat = 170;
pub const AV_PIX_FMT_GRAY12LE: AVPixelFormat = 169;
pub const AV_PIX_FMT_GRAY12BE: AVPixelFormat = 168;
pub const AV_PIX_FMT_MEDIACODEC: AVPixelFormat = 167;
pub const AV_PIX_FMT_GBRAP10LE: AVPixelFormat = 166;
pub const AV_PIX_FMT_GBRAP10BE: AVPixelFormat = 165;
pub const AV_PIX_FMT_GBRAP12LE: AVPixelFormat = 164;
pub const AV_PIX_FMT_GBRAP12BE: AVPixelFormat = 163;
pub const AV_PIX_FMT_P010BE: AVPixelFormat = 162;
pub const AV_PIX_FMT_P010LE: AVPixelFormat = 161;
pub const AV_PIX_FMT_VIDEOTOOLBOX: AVPixelFormat = 160;
pub const AV_PIX_FMT_AYUV64BE: AVPixelFormat = 159;
pub const AV_PIX_FMT_AYUV64LE: AVPixelFormat = 158;
pub const AV_PIX_FMT_YUV440P12BE: AVPixelFormat = 157;
pub const AV_PIX_FMT_YUV440P12LE: AVPixelFormat = 156;
pub const AV_PIX_FMT_YUV440P10BE: AVPixelFormat = 155;
pub const AV_PIX_FMT_YUV440P10LE: AVPixelFormat = 154;
pub const AV_PIX_FMT_XVMC: AVPixelFormat = 153;
pub const AV_PIX_FMT_BAYER_GRBG16BE: AVPixelFormat = 152;
pub const AV_PIX_FMT_BAYER_GRBG16LE: AVPixelFormat = 151;
pub const AV_PIX_FMT_BAYER_GBRG16BE: AVPixelFormat = 150;
pub const AV_PIX_FMT_BAYER_GBRG16LE: AVPixelFormat = 149;
pub const AV_PIX_FMT_BAYER_RGGB16BE: AVPixelFormat = 148;
pub const AV_PIX_FMT_BAYER_RGGB16LE: AVPixelFormat = 147;
pub const AV_PIX_FMT_BAYER_BGGR16BE: AVPixelFormat = 146;
pub const AV_PIX_FMT_BAYER_BGGR16LE: AVPixelFormat = 145;
pub const AV_PIX_FMT_BAYER_GRBG8: AVPixelFormat = 144;
pub const AV_PIX_FMT_BAYER_GBRG8: AVPixelFormat = 143;
pub const AV_PIX_FMT_BAYER_RGGB8: AVPixelFormat = 142;
pub const AV_PIX_FMT_BAYER_BGGR8: AVPixelFormat = 141;
pub const AV_PIX_FMT_YUVJ411P: AVPixelFormat = 140;
pub const AV_PIX_FMT_GBRP14LE: AVPixelFormat = 139;
pub const AV_PIX_FMT_GBRP14BE: AVPixelFormat = 138;
pub const AV_PIX_FMT_GBRP12LE: AVPixelFormat = 137;
pub const AV_PIX_FMT_GBRP12BE: AVPixelFormat = 136;
pub const AV_PIX_FMT_YUV444P14LE: AVPixelFormat = 135;
pub const AV_PIX_FMT_YUV444P14BE: AVPixelFormat = 134;
pub const AV_PIX_FMT_YUV444P12LE: AVPixelFormat = 133;
pub const AV_PIX_FMT_YUV444P12BE: AVPixelFormat = 132;
pub const AV_PIX_FMT_YUV422P14LE: AVPixelFormat = 131;
pub const AV_PIX_FMT_YUV422P14BE: AVPixelFormat = 130;
pub const AV_PIX_FMT_YUV422P12LE: AVPixelFormat = 129;
pub const AV_PIX_FMT_YUV422P12BE: AVPixelFormat = 128;
pub const AV_PIX_FMT_YUV420P14LE: AVPixelFormat = 127;
pub const AV_PIX_FMT_YUV420P14BE: AVPixelFormat = 126;
pub const AV_PIX_FMT_YUV420P12LE: AVPixelFormat = 125;
pub const AV_PIX_FMT_YUV420P12BE: AVPixelFormat = 124;
pub const AV_PIX_FMT_BGR0: AVPixelFormat = 123;
pub const AV_PIX_FMT_0BGR: AVPixelFormat = 122;
pub const AV_PIX_FMT_RGB0: AVPixelFormat = 121;
pub const AV_PIX_FMT_0RGB: AVPixelFormat = 120;
pub const AV_PIX_FMT_CUDA: AVPixelFormat = 119;
pub const AV_PIX_FMT_D3D11VA_VLD: AVPixelFormat = 118;
pub const AV_PIX_FMT_MMAL: AVPixelFormat = 117;
pub const AV_PIX_FMT_QSV: AVPixelFormat = 116;
pub const AV_PIX_FMT_GBRAP16LE: AVPixelFormat = 115;
pub const AV_PIX_FMT_GBRAP16BE: AVPixelFormat = 114;
pub const AV_PIX_FMT_GBRAP: AVPixelFormat = 113;
pub const AV_PIX_FMT_YA16LE: AVPixelFormat = 112;
pub const AV_PIX_FMT_YA16BE: AVPixelFormat = 111;
pub const AV_PIX_FMT_YVYU422: AVPixelFormat = 110;
pub const AV_PIX_FMT_BGRA64LE: AVPixelFormat = 109;
pub const AV_PIX_FMT_BGRA64BE: AVPixelFormat = 108;
pub const AV_PIX_FMT_RGBA64LE: AVPixelFormat = 107;
pub const AV_PIX_FMT_RGBA64BE: AVPixelFormat = 106;
pub const AV_PIX_FMT_NV20BE: AVPixelFormat = 105;
pub const AV_PIX_FMT_NV20LE: AVPixelFormat = 104;
pub const AV_PIX_FMT_NV16: AVPixelFormat = 103;
pub const AV_PIX_FMT_XYZ12BE: AVPixelFormat = 102;
pub const AV_PIX_FMT_XYZ12LE: AVPixelFormat = 101;
pub const AV_PIX_FMT_VDPAU: AVPixelFormat = 100;
pub const AV_PIX_FMT_YUVA444P16LE: AVPixelFormat = 99;
pub const AV_PIX_FMT_YUVA444P16BE: AVPixelFormat = 98;
pub const AV_PIX_FMT_YUVA422P16LE: AVPixelFormat = 97;
pub const AV_PIX_FMT_YUVA422P16BE: AVPixelFormat = 96;
pub const AV_PIX_FMT_YUVA420P16LE: AVPixelFormat = 95;
pub const AV_PIX_FMT_YUVA420P16BE: AVPixelFormat = 94;
pub const AV_PIX_FMT_YUVA444P10LE: AVPixelFormat = 93;
pub const AV_PIX_FMT_YUVA444P10BE: AVPixelFormat = 92;
pub const AV_PIX_FMT_YUVA422P10LE: AVPixelFormat = 91;
pub const AV_PIX_FMT_YUVA422P10BE: AVPixelFormat = 90;
pub const AV_PIX_FMT_YUVA420P10LE: AVPixelFormat = 89;
pub const AV_PIX_FMT_YUVA420P10BE: AVPixelFormat = 88;
pub const AV_PIX_FMT_YUVA444P9LE: AVPixelFormat = 87;
pub const AV_PIX_FMT_YUVA444P9BE: AVPixelFormat = 86;
pub const AV_PIX_FMT_YUVA422P9LE: AVPixelFormat = 85;
pub const AV_PIX_FMT_YUVA422P9BE: AVPixelFormat = 84;
pub const AV_PIX_FMT_YUVA420P9LE: AVPixelFormat = 83;
pub const AV_PIX_FMT_YUVA420P9BE: AVPixelFormat = 82;
pub const AV_PIX_FMT_YUVA444P: AVPixelFormat = 81;
pub const AV_PIX_FMT_YUVA422P: AVPixelFormat = 80;
pub const AV_PIX_FMT_GBRP16LE: AVPixelFormat = 79;
pub const AV_PIX_FMT_GBRP16BE: AVPixelFormat = 78;
pub const AV_PIX_FMT_GBRP10LE: AVPixelFormat = 77;
pub const AV_PIX_FMT_GBRP10BE: AVPixelFormat = 76;
pub const AV_PIX_FMT_GBRP9LE: AVPixelFormat = 75;
pub const AV_PIX_FMT_GBRP9BE: AVPixelFormat = 74;
pub const AV_PIX_FMT_GBR24P: AVPixelFormat = 73;
pub const AV_PIX_FMT_GBRP: AVPixelFormat = 73;
pub const AV_PIX_FMT_YUV422P9LE: AVPixelFormat = 72;
pub const AV_PIX_FMT_YUV422P9BE: AVPixelFormat = 71;
pub const AV_PIX_FMT_YUV444P10LE: AVPixelFormat = 70;
pub const AV_PIX_FMT_YUV444P10BE: AVPixelFormat = 69;
pub const AV_PIX_FMT_YUV444P9LE: AVPixelFormat = 68;
pub const AV_PIX_FMT_YUV444P9BE: AVPixelFormat = 67;
pub const AV_PIX_FMT_YUV422P10LE: AVPixelFormat = 66;
pub const AV_PIX_FMT_YUV422P10BE: AVPixelFormat = 65;
pub const AV_PIX_FMT_YUV420P10LE: AVPixelFormat = 64;
pub const AV_PIX_FMT_YUV420P10BE: AVPixelFormat = 63;
pub const AV_PIX_FMT_YUV420P9LE: AVPixelFormat = 62;
pub const AV_PIX_FMT_YUV420P9BE: AVPixelFormat = 61;
pub const AV_PIX_FMT_BGR48LE: AVPixelFormat = 60;
pub const AV_PIX_FMT_BGR48BE: AVPixelFormat = 59;
pub const AV_PIX_FMT_GRAY8A: AVPixelFormat = 58;
pub const AV_PIX_FMT_Y400A: AVPixelFormat = 58;
pub const AV_PIX_FMT_YA8: AVPixelFormat = 58;
pub const AV_PIX_FMT_BGR444BE: AVPixelFormat = 57;
pub const AV_PIX_FMT_BGR444LE: AVPixelFormat = 56;
pub const AV_PIX_FMT_RGB444BE: AVPixelFormat = 55;
pub const AV_PIX_FMT_RGB444LE: AVPixelFormat = 54;
pub const AV_PIX_FMT_DXVA2_VLD: AVPixelFormat = 53;
pub const AV_PIX_FMT_YUV444P16BE: AVPixelFormat = 52;
pub const AV_PIX_FMT_YUV444P16LE: AVPixelFormat = 51;
pub const AV_PIX_FMT_YUV422P16BE: AVPixelFormat = 50;
pub const AV_PIX_FMT_YUV422P16LE: AVPixelFormat = 49;
pub const AV_PIX_FMT_YUV420P16BE: AVPixelFormat = 48;
pub const AV_PIX_FMT_YUV420P16LE: AVPixelFormat = 47;
pub const AV_PIX_FMT_VAAPI: AVPixelFormat = 46;
pub const AV_PIX_FMT_VAAPI_VLD: AVPixelFormat = 46;
pub const AV_PIX_FMT_VAAPI_IDCT: AVPixelFormat = 45;
pub const AV_PIX_FMT_VAAPI_MOCO: AVPixelFormat = 44;
pub const AV_PIX_FMT_BGR555LE: AVPixelFormat = 43;
pub const AV_PIX_FMT_BGR555BE: AVPixelFormat = 42;
pub const AV_PIX_FMT_BGR565LE: AVPixelFormat = 41;
pub const AV_PIX_FMT_BGR565BE: AVPixelFormat = 40;
pub const AV_PIX_FMT_RGB555LE: AVPixelFormat = 39;
pub const AV_PIX_FMT_RGB555BE: AVPixelFormat = 38;
pub const AV_PIX_FMT_RGB565LE: AVPixelFormat = 37;
pub const AV_PIX_FMT_RGB565BE: AVPixelFormat = 36;
pub const AV_PIX_FMT_RGB48LE: AVPixelFormat = 35;
pub const AV_PIX_FMT_RGB48BE: AVPixelFormat = 34;
pub const AV_PIX_FMT_YUVA420P: AVPixelFormat = 33;
pub const AV_PIX_FMT_YUVJ440P: AVPixelFormat = 32;
pub const AV_PIX_FMT_YUV440P: AVPixelFormat = 31;
pub const AV_PIX_FMT_GRAY16LE: AVPixelFormat = 30;
pub const AV_PIX_FMT_GRAY16BE: AVPixelFormat = 29;
pub const AV_PIX_FMT_BGRA: AVPixelFormat = 28;
pub const AV_PIX_FMT_ABGR: AVPixelFormat = 27;
pub const AV_PIX_FMT_RGBA: AVPixelFormat = 26;
pub const AV_PIX_FMT_ARGB: AVPixelFormat = 25;
pub const AV_PIX_FMT_NV21: AVPixelFormat = 24;
pub const AV_PIX_FMT_NV12: AVPixelFormat = 23;
pub const AV_PIX_FMT_RGB4_BYTE: AVPixelFormat = 22;
pub const AV_PIX_FMT_RGB4: AVPixelFormat = 21;
pub const AV_PIX_FMT_RGB8: AVPixelFormat = 20;
pub const AV_PIX_FMT_BGR4_BYTE: AVPixelFormat = 19;
pub const AV_PIX_FMT_BGR4: AVPixelFormat = 18;
pub const AV_PIX_FMT_BGR8: AVPixelFormat = 17;
pub const AV_PIX_FMT_UYYVYY411: AVPixelFormat = 16;
pub const AV_PIX_FMT_UYVY422: AVPixelFormat = 15;
pub const AV_PIX_FMT_YUVJ444P: AVPixelFormat = 14;
pub const AV_PIX_FMT_YUVJ422P: AVPixelFormat = 13;
pub const AV_PIX_FMT_YUVJ420P: AVPixelFormat = 12;
pub const AV_PIX_FMT_PAL8: AVPixelFormat = 11;
pub const AV_PIX_FMT_MONOBLACK: AVPixelFormat = 10;
pub const AV_PIX_FMT_MONOWHITE: AVPixelFormat = 9;
pub const AV_PIX_FMT_GRAY8: AVPixelFormat = 8;
pub const AV_PIX_FMT_YUV411P: AVPixelFormat = 7;
pub const AV_PIX_FMT_YUV410P: AVPixelFormat = 6;
pub const AV_PIX_FMT_YUV444P: AVPixelFormat = 5;
pub const AV_PIX_FMT_YUV422P: AVPixelFormat = 4;
pub const AV_PIX_FMT_BGR24: AVPixelFormat = 3;
pub const AV_PIX_FMT_RGB24: AVPixelFormat = 2;
pub const AV_PIX_FMT_YUYV422: AVPixelFormat = 1;
pub const AV_PIX_FMT_YUV420P: AVPixelFormat = 0;
pub const AV_PIX_FMT_NONE: AVPixelFormat = -1;
pub type AVColorPrimaries = libc::c_uint;
pub const AVCOL_PRI_NB: AVColorPrimaries = 23;
pub const AVCOL_PRI_JEDEC_P22: AVColorPrimaries = 22;
pub const AVCOL_PRI_SMPTE432: AVColorPrimaries = 12;
pub const AVCOL_PRI_SMPTE431: AVColorPrimaries = 11;
pub const AVCOL_PRI_SMPTEST428_1: AVColorPrimaries = 10;
pub const AVCOL_PRI_SMPTE428: AVColorPrimaries = 10;
pub const AVCOL_PRI_BT2020: AVColorPrimaries = 9;
pub const AVCOL_PRI_FILM: AVColorPrimaries = 8;
pub const AVCOL_PRI_SMPTE240M: AVColorPrimaries = 7;
pub const AVCOL_PRI_SMPTE170M: AVColorPrimaries = 6;
pub const AVCOL_PRI_BT470BG: AVColorPrimaries = 5;
pub const AVCOL_PRI_BT470M: AVColorPrimaries = 4;
pub const AVCOL_PRI_RESERVED: AVColorPrimaries = 3;
pub const AVCOL_PRI_UNSPECIFIED: AVColorPrimaries = 2;
pub const AVCOL_PRI_BT709: AVColorPrimaries = 1;
pub const AVCOL_PRI_RESERVED0: AVColorPrimaries = 0;
pub type AVColorTransferCharacteristic = libc::c_uint;
pub const AVCOL_TRC_NB: AVColorTransferCharacteristic = 19;
pub const AVCOL_TRC_ARIB_STD_B67: AVColorTransferCharacteristic = 18;
pub const AVCOL_TRC_SMPTEST428_1: AVColorTransferCharacteristic = 17;
pub const AVCOL_TRC_SMPTE428: AVColorTransferCharacteristic = 17;
pub const AVCOL_TRC_SMPTEST2084: AVColorTransferCharacteristic = 16;
pub const AVCOL_TRC_SMPTE2084: AVColorTransferCharacteristic = 16;
pub const AVCOL_TRC_BT2020_12: AVColorTransferCharacteristic = 15;
pub const AVCOL_TRC_BT2020_10: AVColorTransferCharacteristic = 14;
pub const AVCOL_TRC_IEC61966_2_1: AVColorTransferCharacteristic = 13;
pub const AVCOL_TRC_BT1361_ECG: AVColorTransferCharacteristic = 12;
pub const AVCOL_TRC_IEC61966_2_4: AVColorTransferCharacteristic = 11;
pub const AVCOL_TRC_LOG_SQRT: AVColorTransferCharacteristic = 10;
pub const AVCOL_TRC_LOG: AVColorTransferCharacteristic = 9;
pub const AVCOL_TRC_LINEAR: AVColorTransferCharacteristic = 8;
pub const AVCOL_TRC_SMPTE240M: AVColorTransferCharacteristic = 7;
pub const AVCOL_TRC_SMPTE170M: AVColorTransferCharacteristic = 6;
pub const AVCOL_TRC_GAMMA28: AVColorTransferCharacteristic = 5;
pub const AVCOL_TRC_GAMMA22: AVColorTransferCharacteristic = 4;
pub const AVCOL_TRC_RESERVED: AVColorTransferCharacteristic = 3;
pub const AVCOL_TRC_UNSPECIFIED: AVColorTransferCharacteristic = 2;
pub const AVCOL_TRC_BT709: AVColorTransferCharacteristic = 1;
pub const AVCOL_TRC_RESERVED0: AVColorTransferCharacteristic = 0;
pub type AVColorSpace = libc::c_uint;
pub const AVCOL_SPC_NB: AVColorSpace = 15;
pub const AVCOL_SPC_ICTCP: AVColorSpace = 14;
pub const AVCOL_SPC_CHROMA_DERIVED_CL: AVColorSpace = 13;
pub const AVCOL_SPC_CHROMA_DERIVED_NCL: AVColorSpace = 12;
pub const AVCOL_SPC_SMPTE2085: AVColorSpace = 11;
pub const AVCOL_SPC_BT2020_CL: AVColorSpace = 10;
pub const AVCOL_SPC_BT2020_NCL: AVColorSpace = 9;
pub const AVCOL_SPC_YCOCG: AVColorSpace = 8;
pub const AVCOL_SPC_YCGCO: AVColorSpace = 8;
pub const AVCOL_SPC_SMPTE240M: AVColorSpace = 7;
pub const AVCOL_SPC_SMPTE170M: AVColorSpace = 6;
pub const AVCOL_SPC_BT470BG: AVColorSpace = 5;
pub const AVCOL_SPC_FCC: AVColorSpace = 4;
pub const AVCOL_SPC_RESERVED: AVColorSpace = 3;
pub const AVCOL_SPC_UNSPECIFIED: AVColorSpace = 2;
pub const AVCOL_SPC_BT709: AVColorSpace = 1;
pub const AVCOL_SPC_RGB: AVColorSpace = 0;
pub type AVColorRange = libc::c_uint;
pub const AVCOL_RANGE_NB: AVColorRange = 3;
pub const AVCOL_RANGE_JPEG: AVColorRange = 2;
pub const AVCOL_RANGE_MPEG: AVColorRange = 1;
pub const AVCOL_RANGE_UNSPECIFIED: AVColorRange = 0;
pub type AVChromaLocation = libc::c_uint;
pub const AVCHROMA_LOC_NB: AVChromaLocation = 7;
pub const AVCHROMA_LOC_BOTTOM: AVChromaLocation = 6;
pub const AVCHROMA_LOC_BOTTOMLEFT: AVChromaLocation = 5;
pub const AVCHROMA_LOC_TOP: AVChromaLocation = 4;
pub const AVCHROMA_LOC_TOPLEFT: AVChromaLocation = 3;
pub const AVCHROMA_LOC_CENTER: AVChromaLocation = 2;
pub const AVCHROMA_LOC_LEFT: AVChromaLocation = 1;
pub const AVCHROMA_LOC_UNSPECIFIED: AVChromaLocation = 0;
pub type AVSampleFormat = libc::c_int;
pub const AV_SAMPLE_FMT_NB: AVSampleFormat = 12;
pub const AV_SAMPLE_FMT_S64P: AVSampleFormat = 11;
pub const AV_SAMPLE_FMT_S64: AVSampleFormat = 10;
pub const AV_SAMPLE_FMT_DBLP: AVSampleFormat = 9;
pub const AV_SAMPLE_FMT_FLTP: AVSampleFormat = 8;
pub const AV_SAMPLE_FMT_S32P: AVSampleFormat = 7;
pub const AV_SAMPLE_FMT_S16P: AVSampleFormat = 6;
pub const AV_SAMPLE_FMT_U8P: AVSampleFormat = 5;
pub const AV_SAMPLE_FMT_DBL: AVSampleFormat = 4;
pub const AV_SAMPLE_FMT_FLT: AVSampleFormat = 3;
pub const AV_SAMPLE_FMT_S32: AVSampleFormat = 2;
pub const AV_SAMPLE_FMT_S16: AVSampleFormat = 1;
pub const AV_SAMPLE_FMT_U8: AVSampleFormat = 0;
pub const AV_SAMPLE_FMT_NONE: AVSampleFormat = -1;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct AVBufferRef {
    pub buffer: *mut AVBuffer,
    pub data: *mut uint8_t,
    pub size: libc::c_int,
}
pub type AVFrameSideDataType = libc::c_uint;
pub const AV_FRAME_DATA_REGIONS_OF_INTEREST: AVFrameSideDataType = 20;
pub const AV_FRAME_DATA_DYNAMIC_HDR_PLUS: AVFrameSideDataType = 19;
pub const AV_FRAME_DATA_S12M_TIMECODE: AVFrameSideDataType = 18;
pub const AV_FRAME_DATA_QP_TABLE_DATA: AVFrameSideDataType = 17;
pub const AV_FRAME_DATA_QP_TABLE_PROPERTIES: AVFrameSideDataType = 16;
pub const AV_FRAME_DATA_ICC_PROFILE: AVFrameSideDataType = 15;
pub const AV_FRAME_DATA_CONTENT_LIGHT_LEVEL: AVFrameSideDataType = 14;
pub const AV_FRAME_DATA_SPHERICAL: AVFrameSideDataType = 13;
pub const AV_FRAME_DATA_GOP_TIMECODE: AVFrameSideDataType = 12;
pub const AV_FRAME_DATA_MASTERING_DISPLAY_METADATA: AVFrameSideDataType = 11;
pub const AV_FRAME_DATA_AUDIO_SERVICE_TYPE: AVFrameSideDataType = 10;
pub const AV_FRAME_DATA_SKIP_SAMPLES: AVFrameSideDataType = 9;
pub const AV_FRAME_DATA_MOTION_VECTORS: AVFrameSideDataType = 8;
pub const AV_FRAME_DATA_AFD: AVFrameSideDataType = 7;
pub const AV_FRAME_DATA_DISPLAYMATRIX: AVFrameSideDataType = 6;
pub const AV_FRAME_DATA_REPLAYGAIN: AVFrameSideDataType = 5;
pub const AV_FRAME_DATA_DOWNMIX_INFO: AVFrameSideDataType = 4;
pub const AV_FRAME_DATA_MATRIXENCODING: AVFrameSideDataType = 3;
pub const AV_FRAME_DATA_STEREO3D: AVFrameSideDataType = 2;
pub const AV_FRAME_DATA_A53_CC: AVFrameSideDataType = 1;
pub const AV_FRAME_DATA_PANSCAN: AVFrameSideDataType = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct AVFrameSideData {
    pub type_0: AVFrameSideDataType,
    pub data: *mut uint8_t,
    pub size: libc::c_int,
    pub metadata: *mut AVDictionary,
    pub buf: *mut AVBufferRef,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct AVFrame {
    pub data: [*mut uint8_t; 8],
    pub linesize: [libc::c_int; 8],
    pub extended_data: *mut *mut uint8_t,
    pub width: libc::c_int,
    pub height: libc::c_int,
    pub nb_samples: libc::c_int,
    pub format: libc::c_int,
    pub key_frame: libc::c_int,
    pub pict_type: AVPictureType,
    pub sample_aspect_ratio: AVRational,
    pub pts: int64_t,
    pub pkt_pts: int64_t,
    pub pkt_dts: int64_t,
    pub coded_picture_number: libc::c_int,
    pub display_picture_number: libc::c_int,
    pub quality: libc::c_int,
    pub opaque: *mut libc::c_void,
    pub error: [uint64_t; 8],
    pub repeat_pict: libc::c_int,
    pub interlaced_frame: libc::c_int,
    pub top_field_first: libc::c_int,
    pub palette_has_changed: libc::c_int,
    pub reordered_opaque: int64_t,
    pub sample_rate: libc::c_int,
    pub channel_layout: uint64_t,
    pub buf: [*mut AVBufferRef; 8],
    pub extended_buf: *mut *mut AVBufferRef,
    pub nb_extended_buf: libc::c_int,
    pub side_data: *mut *mut AVFrameSideData,
    pub nb_side_data: libc::c_int,
    pub flags: libc::c_int,
    pub color_range: AVColorRange,
    pub color_primaries: AVColorPrimaries,
    pub color_trc: AVColorTransferCharacteristic,
    pub colorspace: AVColorSpace,
    pub chroma_location: AVChromaLocation,
    pub best_effort_timestamp: int64_t,
    pub pkt_pos: int64_t,
    pub pkt_duration: int64_t,
    pub metadata: *mut AVDictionary,
    pub decode_error_flags: libc::c_int,
    pub channels: libc::c_int,
    pub pkt_size: libc::c_int,
    pub qscale_table: *mut int8_t,
    pub qstride: libc::c_int,
    pub qscale_type: libc::c_int,
    pub qp_table_buf: *mut AVBufferRef,
    pub hw_frames_ctx: *mut AVBufferRef,
    pub opaque_ref: *mut AVBufferRef,
    pub crop_top: size_t,
    pub crop_bottom: size_t,
    pub crop_left: size_t,
    pub crop_right: size_t,
    pub private_ref: *mut AVBufferRef,
}
pub type AVCodecID = libc::c_uint;
pub const AV_CODEC_ID_WRAPPED_AVFRAME: AVCodecID = 135169;
pub const AV_CODEC_ID_FFMETADATA: AVCodecID = 135168;
pub const AV_CODEC_ID_MPEG4SYSTEMS: AVCodecID = 131073;
pub const AV_CODEC_ID_MPEG2TS: AVCodecID = 131072;
pub const AV_CODEC_ID_PROBE: AVCodecID = 102400;
pub const AV_CODEC_ID_BIN_DATA: AVCodecID = 100359;
pub const AV_CODEC_ID_TIMED_ID3: AVCodecID = 100358;
pub const AV_CODEC_ID_DVD_NAV: AVCodecID = 100357;
pub const AV_CODEC_ID_SMPTE_KLV: AVCodecID = 100356;
pub const AV_CODEC_ID_OTF: AVCodecID = 100355;
pub const AV_CODEC_ID_IDF: AVCodecID = 100354;
pub const AV_CODEC_ID_XBIN: AVCodecID = 100353;
pub const AV_CODEC_ID_BINTEXT: AVCodecID = 100352;
pub const AV_CODEC_ID_SCTE_35: AVCodecID = 98305;
pub const AV_CODEC_ID_TTF: AVCodecID = 98304;
pub const AV_CODEC_ID_FIRST_UNKNOWN: AVCodecID = 98304;
pub const AV_CODEC_ID_ARIB_CAPTION: AVCodecID = 96272;
pub const AV_CODEC_ID_TTML: AVCodecID = 96271;
pub const AV_CODEC_ID_HDMV_TEXT_SUBTITLE: AVCodecID = 96270;
pub const AV_CODEC_ID_ASS: AVCodecID = 96269;
pub const AV_CODEC_ID_PJS: AVCodecID = 96268;
pub const AV_CODEC_ID_VPLAYER: AVCodecID = 96267;
pub const AV_CODEC_ID_MPL2: AVCodecID = 96266;
pub const AV_CODEC_ID_WEBVTT: AVCodecID = 96265;
pub const AV_CODEC_ID_SUBRIP: AVCodecID = 96264;
pub const AV_CODEC_ID_SUBVIEWER: AVCodecID = 96263;
pub const AV_CODEC_ID_SUBVIEWER1: AVCodecID = 96262;
pub const AV_CODEC_ID_STL: AVCodecID = 96261;
pub const AV_CODEC_ID_REALTEXT: AVCodecID = 96260;
pub const AV_CODEC_ID_SAMI: AVCodecID = 96259;
pub const AV_CODEC_ID_JACOSUB: AVCodecID = 96258;
pub const AV_CODEC_ID_EIA_608: AVCodecID = 96257;
pub const AV_CODEC_ID_MICRODVD: AVCodecID = 96256;
pub const AV_CODEC_ID_SRT: AVCodecID = 94216;
pub const AV_CODEC_ID_DVB_TELETEXT: AVCodecID = 94215;
pub const AV_CODEC_ID_HDMV_PGS_SUBTITLE: AVCodecID = 94214;
pub const AV_CODEC_ID_MOV_TEXT: AVCodecID = 94213;
pub const AV_CODEC_ID_SSA: AVCodecID = 94212;
pub const AV_CODEC_ID_XSUB: AVCodecID = 94211;
pub const AV_CODEC_ID_TEXT: AVCodecID = 94210;
pub const AV_CODEC_ID_DVB_SUBTITLE: AVCodecID = 94209;
pub const AV_CODEC_ID_DVD_SUBTITLE: AVCodecID = 94208;
pub const AV_CODEC_ID_FIRST_SUBTITLE: AVCodecID = 94208;
pub const AV_CODEC_ID_HCOM: AVCodecID = 88085;
pub const AV_CODEC_ID_ATRAC9: AVCodecID = 88084;
pub const AV_CODEC_ID_SBC: AVCodecID = 88083;
pub const AV_CODEC_ID_APTX_HD: AVCodecID = 88082;
pub const AV_CODEC_ID_APTX: AVCodecID = 88081;
pub const AV_CODEC_ID_DOLBY_E: AVCodecID = 88080;
pub const AV_CODEC_ID_ATRAC3PAL: AVCodecID = 88079;
pub const AV_CODEC_ID_ATRAC3AL: AVCodecID = 88078;
pub const AV_CODEC_ID_DST: AVCodecID = 88077;
pub const AV_CODEC_ID_XMA2: AVCodecID = 88076;
pub const AV_CODEC_ID_XMA1: AVCodecID = 88075;
pub const AV_CODEC_ID_INTERPLAY_ACM: AVCodecID = 88074;
pub const AV_CODEC_ID_4GV: AVCodecID = 88073;
pub const AV_CODEC_ID_DSD_MSBF_PLANAR: AVCodecID = 88072;
pub const AV_CODEC_ID_DSD_LSBF_PLANAR: AVCodecID = 88071;
pub const AV_CODEC_ID_DSD_MSBF: AVCodecID = 88070;
pub const AV_CODEC_ID_DSD_LSBF: AVCodecID = 88069;
pub const AV_CODEC_ID_SMV: AVCodecID = 88068;
pub const AV_CODEC_ID_EVRC: AVCodecID = 88067;
pub const AV_CODEC_ID_SONIC_LS: AVCodecID = 88066;
pub const AV_CODEC_ID_SONIC: AVCodecID = 88065;
pub const AV_CODEC_ID_FFWAVESYNTH: AVCodecID = 88064;
pub const AV_CODEC_ID_CODEC2: AVCodecID = 86083;
pub const AV_CODEC_ID_DSS_SP: AVCodecID = 86082;
pub const AV_CODEC_ID_ON2AVC: AVCodecID = 86081;
pub const AV_CODEC_ID_PAF_AUDIO: AVCodecID = 86080;
pub const AV_CODEC_ID_METASOUND: AVCodecID = 86079;
pub const AV_CODEC_ID_TAK: AVCodecID = 86078;
pub const AV_CODEC_ID_COMFORT_NOISE: AVCodecID = 86077;
pub const AV_CODEC_ID_OPUS: AVCodecID = 86076;
pub const AV_CODEC_ID_ILBC: AVCodecID = 86075;
pub const AV_CODEC_ID_IAC: AVCodecID = 86074;
pub const AV_CODEC_ID_RALF: AVCodecID = 86073;
pub const AV_CODEC_ID_BMV_AUDIO: AVCodecID = 86072;
pub const AV_CODEC_ID_8SVX_FIB: AVCodecID = 86071;
pub const AV_CODEC_ID_8SVX_EXP: AVCodecID = 86070;
pub const AV_CODEC_ID_G729: AVCodecID = 86069;
pub const AV_CODEC_ID_G723_1: AVCodecID = 86068;
pub const AV_CODEC_ID_CELT: AVCodecID = 86067;
pub const AV_CODEC_ID_QDMC: AVCodecID = 86066;
pub const AV_CODEC_ID_AAC_LATM: AVCodecID = 86065;
pub const AV_CODEC_ID_BINKAUDIO_DCT: AVCodecID = 86064;
pub const AV_CODEC_ID_BINKAUDIO_RDFT: AVCodecID = 86063;
pub const AV_CODEC_ID_ATRAC1: AVCodecID = 86062;
pub const AV_CODEC_ID_MP4ALS: AVCodecID = 86061;
pub const AV_CODEC_ID_TRUEHD: AVCodecID = 86060;
pub const AV_CODEC_ID_TWINVQ: AVCodecID = 86059;
pub const AV_CODEC_ID_MP1: AVCodecID = 86058;
pub const AV_CODEC_ID_SIPR: AVCodecID = 86057;
pub const AV_CODEC_ID_EAC3: AVCodecID = 86056;
pub const AV_CODEC_ID_ATRAC3P: AVCodecID = 86055;
pub const AV_CODEC_ID_WMALOSSLESS: AVCodecID = 86054;
pub const AV_CODEC_ID_WMAPRO: AVCodecID = 86053;
pub const AV_CODEC_ID_WMAVOICE: AVCodecID = 86052;
pub const AV_CODEC_ID_SPEEX: AVCodecID = 86051;
pub const AV_CODEC_ID_MUSEPACK8: AVCodecID = 86050;
pub const AV_CODEC_ID_NELLYMOSER: AVCodecID = 86049;
pub const AV_CODEC_ID_APE: AVCodecID = 86048;
pub const AV_CODEC_ID_ATRAC3: AVCodecID = 86047;
pub const AV_CODEC_ID_GSM_MS: AVCodecID = 86046;
pub const AV_CODEC_ID_MLP: AVCodecID = 86045;
pub const AV_CODEC_ID_MUSEPACK7: AVCodecID = 86044;
pub const AV_CODEC_ID_IMC: AVCodecID = 86043;
pub const AV_CODEC_ID_DSICINAUDIO: AVCodecID = 86042;
pub const AV_CODEC_ID_WAVPACK: AVCodecID = 86041;
pub const AV_CODEC_ID_QCELP: AVCodecID = 86040;
pub const AV_CODEC_ID_SMACKAUDIO: AVCodecID = 86039;
pub const AV_CODEC_ID_TTA: AVCodecID = 86038;
pub const AV_CODEC_ID_TRUESPEECH: AVCodecID = 86037;
pub const AV_CODEC_ID_COOK: AVCodecID = 86036;
pub const AV_CODEC_ID_QDM2: AVCodecID = 86035;
pub const AV_CODEC_ID_GSM: AVCodecID = 86034;
pub const AV_CODEC_ID_WESTWOOD_SND1: AVCodecID = 86033;
pub const AV_CODEC_ID_ALAC: AVCodecID = 86032;
pub const AV_CODEC_ID_SHORTEN: AVCodecID = 86031;
pub const AV_CODEC_ID_MP3ON4: AVCodecID = 86030;
pub const AV_CODEC_ID_MP3ADU: AVCodecID = 86029;
pub const AV_CODEC_ID_FLAC: AVCodecID = 86028;
pub const AV_CODEC_ID_VMDAUDIO: AVCodecID = 86027;
pub const AV_CODEC_ID_MACE6: AVCodecID = 86026;
pub const AV_CODEC_ID_MACE3: AVCodecID = 86025;
pub const AV_CODEC_ID_WMAV2: AVCodecID = 86024;
pub const AV_CODEC_ID_WMAV1: AVCodecID = 86023;
pub const AV_CODEC_ID_DVAUDIO: AVCodecID = 86022;
pub const AV_CODEC_ID_VORBIS: AVCodecID = 86021;
pub const AV_CODEC_ID_DTS: AVCodecID = 86020;
pub const AV_CODEC_ID_AC3: AVCodecID = 86019;
pub const AV_CODEC_ID_AAC: AVCodecID = 86018;
pub const AV_CODEC_ID_MP3: AVCodecID = 86017;
pub const AV_CODEC_ID_MP2: AVCodecID = 86016;
pub const AV_CODEC_ID_GREMLIN_DPCM: AVCodecID = 83969;
pub const AV_CODEC_ID_SDX2_DPCM: AVCodecID = 83968;
pub const AV_CODEC_ID_SOL_DPCM: AVCodecID = 81923;
pub const AV_CODEC_ID_XAN_DPCM: AVCodecID = 81922;
pub const AV_CODEC_ID_INTERPLAY_DPCM: AVCodecID = 81921;
pub const AV_CODEC_ID_ROQ_DPCM: AVCodecID = 81920;
pub const AV_CODEC_ID_RA_288: AVCodecID = 77825;
pub const AV_CODEC_ID_RA_144: AVCodecID = 77824;
pub const AV_CODEC_ID_AMR_WB: AVCodecID = 73729;
pub const AV_CODEC_ID_AMR_NB: AVCodecID = 73728;
pub const AV_CODEC_ID_ADPCM_AGM: AVCodecID = 71690;
pub const AV_CODEC_ID_ADPCM_MTAF: AVCodecID = 71689;
pub const AV_CODEC_ID_ADPCM_IMA_DAT4: AVCodecID = 71688;
pub const AV_CODEC_ID_ADPCM_AICA: AVCodecID = 71687;
pub const AV_CODEC_ID_ADPCM_PSX: AVCodecID = 71686;
pub const AV_CODEC_ID_ADPCM_THP_LE: AVCodecID = 71685;
pub const AV_CODEC_ID_ADPCM_G726LE: AVCodecID = 71684;
pub const AV_CODEC_ID_ADPCM_IMA_RAD: AVCodecID = 71683;
pub const AV_CODEC_ID_ADPCM_DTK: AVCodecID = 71682;
pub const AV_CODEC_ID_ADPCM_IMA_OKI: AVCodecID = 71681;
pub const AV_CODEC_ID_ADPCM_AFC: AVCodecID = 71680;
pub const AV_CODEC_ID_ADPCM_VIMA: AVCodecID = 69662;
pub const AV_CODEC_ID_ADPCM_IMA_APC: AVCodecID = 69661;
pub const AV_CODEC_ID_ADPCM_G722: AVCodecID = 69660;
pub const AV_CODEC_ID_ADPCM_IMA_ISS: AVCodecID = 69659;
pub const AV_CODEC_ID_ADPCM_EA_MAXIS_XA: AVCodecID = 69658;
pub const AV_CODEC_ID_ADPCM_EA_XAS: AVCodecID = 69657;
pub const AV_CODEC_ID_ADPCM_IMA_EA_EACS: AVCodecID = 69656;
pub const AV_CODEC_ID_ADPCM_IMA_EA_SEAD: AVCodecID = 69655;
pub const AV_CODEC_ID_ADPCM_EA_R2: AVCodecID = 69654;
pub const AV_CODEC_ID_ADPCM_EA_R3: AVCodecID = 69653;
pub const AV_CODEC_ID_ADPCM_EA_R1: AVCodecID = 69652;
pub const AV_CODEC_ID_ADPCM_IMA_AMV: AVCodecID = 69651;
pub const AV_CODEC_ID_ADPCM_THP: AVCodecID = 69650;
pub const AV_CODEC_ID_ADPCM_SBPRO_2: AVCodecID = 69649;
pub const AV_CODEC_ID_ADPCM_SBPRO_3: AVCodecID = 69648;
pub const AV_CODEC_ID_ADPCM_SBPRO_4: AVCodecID = 69647;
pub const AV_CODEC_ID_ADPCM_YAMAHA: AVCodecID = 69646;
pub const AV_CODEC_ID_ADPCM_SWF: AVCodecID = 69645;
pub const AV_CODEC_ID_ADPCM_CT: AVCodecID = 69644;
pub const AV_CODEC_ID_ADPCM_G726: AVCodecID = 69643;
pub const AV_CODEC_ID_ADPCM_EA: AVCodecID = 69642;
pub const AV_CODEC_ID_ADPCM_ADX: AVCodecID = 69641;
pub const AV_CODEC_ID_ADPCM_XA: AVCodecID = 69640;
pub const AV_CODEC_ID_ADPCM_4XM: AVCodecID = 69639;
pub const AV_CODEC_ID_ADPCM_MS: AVCodecID = 69638;
pub const AV_CODEC_ID_ADPCM_IMA_SMJPEG: AVCodecID = 69637;
pub const AV_CODEC_ID_ADPCM_IMA_WS: AVCodecID = 69636;
pub const AV_CODEC_ID_ADPCM_IMA_DK4: AVCodecID = 69635;
pub const AV_CODEC_ID_ADPCM_IMA_DK3: AVCodecID = 69634;
pub const AV_CODEC_ID_ADPCM_IMA_WAV: AVCodecID = 69633;
pub const AV_CODEC_ID_ADPCM_IMA_QT: AVCodecID = 69632;
pub const AV_CODEC_ID_PCM_VIDC: AVCodecID = 67588;
pub const AV_CODEC_ID_PCM_F24LE: AVCodecID = 67587;
pub const AV_CODEC_ID_PCM_F16LE: AVCodecID = 67586;
pub const AV_CODEC_ID_PCM_S64BE: AVCodecID = 67585;
pub const AV_CODEC_ID_PCM_S64LE: AVCodecID = 67584;
pub const AV_CODEC_ID_PCM_S16BE_PLANAR: AVCodecID = 65566;
pub const AV_CODEC_ID_PCM_S32LE_PLANAR: AVCodecID = 65565;
pub const AV_CODEC_ID_PCM_S24LE_PLANAR: AVCodecID = 65564;
pub const AV_CODEC_ID_PCM_S8_PLANAR: AVCodecID = 65563;
pub const AV_CODEC_ID_S302M: AVCodecID = 65562;
pub const AV_CODEC_ID_PCM_LXF: AVCodecID = 65561;
pub const AV_CODEC_ID_PCM_BLURAY: AVCodecID = 65560;
pub const AV_CODEC_ID_PCM_F64LE: AVCodecID = 65559;
pub const AV_CODEC_ID_PCM_F64BE: AVCodecID = 65558;
pub const AV_CODEC_ID_PCM_F32LE: AVCodecID = 65557;
pub const AV_CODEC_ID_PCM_F32BE: AVCodecID = 65556;
pub const AV_CODEC_ID_PCM_DVD: AVCodecID = 65555;
pub const AV_CODEC_ID_PCM_S16LE_PLANAR: AVCodecID = 65554;
pub const AV_CODEC_ID_PCM_ZORK: AVCodecID = 65553;
pub const AV_CODEC_ID_PCM_S24DAUD: AVCodecID = 65552;
pub const AV_CODEC_ID_PCM_U24BE: AVCodecID = 65551;
pub const AV_CODEC_ID_PCM_U24LE: AVCodecID = 65550;
pub const AV_CODEC_ID_PCM_S24BE: AVCodecID = 65549;
pub const AV_CODEC_ID_PCM_S24LE: AVCodecID = 65548;
pub const AV_CODEC_ID_PCM_U32BE: AVCodecID = 65547;
pub const AV_CODEC_ID_PCM_U32LE: AVCodecID = 65546;
pub const AV_CODEC_ID_PCM_S32BE: AVCodecID = 65545;
pub const AV_CODEC_ID_PCM_S32LE: AVCodecID = 65544;
pub const AV_CODEC_ID_PCM_ALAW: AVCodecID = 65543;
pub const AV_CODEC_ID_PCM_MULAW: AVCodecID = 65542;
pub const AV_CODEC_ID_PCM_U8: AVCodecID = 65541;
pub const AV_CODEC_ID_PCM_S8: AVCodecID = 65540;
pub const AV_CODEC_ID_PCM_U16BE: AVCodecID = 65539;
pub const AV_CODEC_ID_PCM_U16LE: AVCodecID = 65538;
pub const AV_CODEC_ID_PCM_S16BE: AVCodecID = 65537;
pub const AV_CODEC_ID_PCM_S16LE: AVCodecID = 65536;
pub const AV_CODEC_ID_FIRST_AUDIO: AVCodecID = 65536;
pub const AV_CODEC_ID_VP4: AVCodecID = 32813;
pub const AV_CODEC_ID_LSCR: AVCodecID = 32812;
pub const AV_CODEC_ID_AGM: AVCodecID = 32811;
pub const AV_CODEC_ID_ARBC: AVCodecID = 32810;
pub const AV_CODEC_ID_HYMT: AVCodecID = 32809;
pub const AV_CODEC_ID_RASC: AVCodecID = 32808;
pub const AV_CODEC_ID_WCMV: AVCodecID = 32807;
pub const AV_CODEC_ID_MWSC: AVCodecID = 32806;
pub const AV_CODEC_ID_PROSUMER: AVCodecID = 32805;
pub const AV_CODEC_ID_IMM4: AVCodecID = 32804;
pub const AV_CODEC_ID_FITS: AVCodecID = 32803;
pub const AV_CODEC_ID_GDV: AVCodecID = 32802;
pub const AV_CODEC_ID_SVG: AVCodecID = 32801;
pub const AV_CODEC_ID_SRGC: AVCodecID = 32800;
pub const AV_CODEC_ID_MSCC: AVCodecID = 32799;
pub const AV_CODEC_ID_BITPACKED: AVCodecID = 32798;
pub const AV_CODEC_ID_AV1: AVCodecID = 32797;
pub const AV_CODEC_ID_XPM: AVCodecID = 32796;
pub const AV_CODEC_ID_CLEARVIDEO: AVCodecID = 32795;
pub const AV_CODEC_ID_SCPR: AVCodecID = 32794;
pub const AV_CODEC_ID_FMVC: AVCodecID = 32793;
pub const AV_CODEC_ID_SPEEDHQ: AVCodecID = 32792;
pub const AV_CODEC_ID_PIXLET: AVCodecID = 32791;
pub const AV_CODEC_ID_PSD: AVCodecID = 32790;
pub const AV_CODEC_ID_YLC: AVCodecID = 32789;
pub const AV_CODEC_ID_SHEERVIDEO: AVCodecID = 32788;
pub const AV_CODEC_ID_MAGICYUV: AVCodecID = 32787;
pub const AV_CODEC_ID_M101: AVCodecID = 32786;
pub const AV_CODEC_ID_TRUEMOTION2RT: AVCodecID = 32785;
pub const AV_CODEC_ID_CFHD: AVCodecID = 32784;
pub const AV_CODEC_ID_DAALA: AVCodecID = 32783;
pub const AV_CODEC_ID_APNG: AVCodecID = 32782;
pub const AV_CODEC_ID_SMVJPEG: AVCodecID = 32781;
pub const AV_CODEC_ID_SNOW: AVCodecID = 32780;
pub const AV_CODEC_ID_XFACE: AVCodecID = 32779;
pub const AV_CODEC_ID_CPIA: AVCodecID = 32778;
pub const AV_CODEC_ID_AVRN: AVCodecID = 32777;
pub const AV_CODEC_ID_YUV4: AVCodecID = 32776;
pub const AV_CODEC_ID_V408: AVCodecID = 32775;
pub const AV_CODEC_ID_V308: AVCodecID = 32774;
pub const AV_CODEC_ID_TARGA_Y216: AVCodecID = 32773;
pub const AV_CODEC_ID_AYUV: AVCodecID = 32772;
pub const AV_CODEC_ID_AVUI: AVCodecID = 32771;
pub const AV_CODEC_ID_012V: AVCodecID = 32770;
pub const AV_CODEC_ID_AVRP: AVCodecID = 32769;
pub const AV_CODEC_ID_Y41P: AVCodecID = 32768;
pub const AV_CODEC_ID_AVS2: AVCodecID = 192;
pub const AV_CODEC_ID_RSCC: AVCodecID = 191;
pub const AV_CODEC_ID_SCREENPRESSO: AVCodecID = 190;
pub const AV_CODEC_ID_DXV: AVCodecID = 189;
pub const AV_CODEC_ID_DDS: AVCodecID = 188;
pub const AV_CODEC_ID_HAP: AVCodecID = 187;
pub const AV_CODEC_ID_HQ_HQA: AVCodecID = 186;
pub const AV_CODEC_ID_TDSC: AVCodecID = 185;
pub const AV_CODEC_ID_HQX: AVCodecID = 184;
pub const AV_CODEC_ID_MVC2: AVCodecID = 183;
pub const AV_CODEC_ID_MVC1: AVCodecID = 182;
pub const AV_CODEC_ID_SGIRLE: AVCodecID = 181;
pub const AV_CODEC_ID_SANM: AVCodecID = 180;
pub const AV_CODEC_ID_VP7: AVCodecID = 179;
pub const AV_CODEC_ID_EXR: AVCodecID = 178;
pub const AV_CODEC_ID_PAF_VIDEO: AVCodecID = 177;
pub const AV_CODEC_ID_BRENDER_PIX: AVCodecID = 176;
pub const AV_CODEC_ID_ALIAS_PIX: AVCodecID = 175;
pub const AV_CODEC_ID_FIC: AVCodecID = 174;
pub const AV_CODEC_ID_HEVC: AVCodecID = 173;
pub const AV_CODEC_ID_HNM4_VIDEO: AVCodecID = 172;
pub const AV_CODEC_ID_WEBP: AVCodecID = 171;
pub const AV_CODEC_ID_G2M: AVCodecID = 170;
pub const AV_CODEC_ID_ESCAPE130: AVCodecID = 169;
pub const AV_CODEC_ID_AIC: AVCodecID = 168;
pub const AV_CODEC_ID_VP9: AVCodecID = 167;
pub const AV_CODEC_ID_MSS2: AVCodecID = 166;
pub const AV_CODEC_ID_CLLC: AVCodecID = 165;
pub const AV_CODEC_ID_MTS2: AVCodecID = 164;
pub const AV_CODEC_ID_TSCC2: AVCodecID = 163;
pub const AV_CODEC_ID_MSA1: AVCodecID = 162;
pub const AV_CODEC_ID_MSS1: AVCodecID = 161;
pub const AV_CODEC_ID_ZEROCODEC: AVCodecID = 160;
pub const AV_CODEC_ID_XBM: AVCodecID = 159;
pub const AV_CODEC_ID_CDXL: AVCodecID = 158;
pub const AV_CODEC_ID_XWD: AVCodecID = 157;
pub const AV_CODEC_ID_V410: AVCodecID = 156;
pub const AV_CODEC_ID_DXTORY: AVCodecID = 155;
pub const AV_CODEC_ID_VBLE: AVCodecID = 154;
pub const AV_CODEC_ID_BMV_VIDEO: AVCodecID = 153;
pub const AV_CODEC_ID_UTVIDEO: AVCodecID = 152;
pub const AV_CODEC_ID_VC1IMAGE: AVCodecID = 151;
pub const AV_CODEC_ID_WMV3IMAGE: AVCodecID = 150;
pub const AV_CODEC_ID_DFA: AVCodecID = 149;
pub const AV_CODEC_ID_JV: AVCodecID = 148;
pub const AV_CODEC_ID_PRORES: AVCodecID = 147;
pub const AV_CODEC_ID_LAGARITH: AVCodecID = 146;
pub const AV_CODEC_ID_MXPEG: AVCodecID = 145;
pub const AV_CODEC_ID_R10K: AVCodecID = 144;
pub const AV_CODEC_ID_A64_MULTI5: AVCodecID = 143;
pub const AV_CODEC_ID_A64_MULTI: AVCodecID = 142;
pub const AV_CODEC_ID_ANSI: AVCodecID = 141;
pub const AV_CODEC_ID_PICTOR: AVCodecID = 140;
pub const AV_CODEC_ID_VP8: AVCodecID = 139;
pub const AV_CODEC_ID_YOP: AVCodecID = 138;
pub const AV_CODEC_ID_KGV1: AVCodecID = 137;
pub const AV_CODEC_ID_IFF_ILBM: AVCodecID = 136;
pub const AV_CODEC_ID_BINKVIDEO: AVCodecID = 135;
pub const AV_CODEC_ID_ANM: AVCodecID = 134;
pub const AV_CODEC_ID_R210: AVCodecID = 133;
pub const AV_CODEC_ID_CDGRAPHICS: AVCodecID = 132;
pub const AV_CODEC_ID_FLASHSV2: AVCodecID = 131;
pub const AV_CODEC_ID_FRWU: AVCodecID = 130;
pub const AV_CODEC_ID_MAD: AVCodecID = 129;
pub const AV_CODEC_ID_DPX: AVCodecID = 128;
pub const AV_CODEC_ID_V210: AVCodecID = 127;
pub const AV_CODEC_ID_TMV: AVCodecID = 126;
pub const AV_CODEC_ID_V210X: AVCodecID = 125;
pub const AV_CODEC_ID_AURA2: AVCodecID = 124;
pub const AV_CODEC_ID_AURA: AVCodecID = 123;
pub const AV_CODEC_ID_TQI: AVCodecID = 122;
pub const AV_CODEC_ID_TGQ: AVCodecID = 121;
pub const AV_CODEC_ID_TGV: AVCodecID = 120;
pub const AV_CODEC_ID_MOTIONPIXELS: AVCodecID = 119;
pub const AV_CODEC_ID_CMV: AVCodecID = 118;
pub const AV_CODEC_ID_BFI: AVCodecID = 117;
pub const AV_CODEC_ID_DIRAC: AVCodecID = 116;
pub const AV_CODEC_ID_ESCAPE124: AVCodecID = 115;
pub const AV_CODEC_ID_RL2: AVCodecID = 114;
pub const AV_CODEC_ID_MIMIC: AVCodecID = 113;
pub const AV_CODEC_ID_INDEO5: AVCodecID = 112;
pub const AV_CODEC_ID_INDEO4: AVCodecID = 111;
pub const AV_CODEC_ID_SUNRAST: AVCodecID = 110;
pub const AV_CODEC_ID_PCX: AVCodecID = 109;
pub const AV_CODEC_ID_VB: AVCodecID = 108;
pub const AV_CODEC_ID_AMV: AVCodecID = 107;
pub const AV_CODEC_ID_VP6A: AVCodecID = 106;
pub const AV_CODEC_ID_TXD: AVCodecID = 105;
pub const AV_CODEC_ID_PTX: AVCodecID = 104;
pub const AV_CODEC_ID_BETHSOFTVID: AVCodecID = 103;
pub const AV_CODEC_ID_C93: AVCodecID = 102;
pub const AV_CODEC_ID_SGI: AVCodecID = 101;
pub const AV_CODEC_ID_THP: AVCodecID = 100;
pub const AV_CODEC_ID_DNXHD: AVCodecID = 99;
pub const AV_CODEC_ID_DXA: AVCodecID = 98;
pub const AV_CODEC_ID_GIF: AVCodecID = 97;
pub const AV_CODEC_ID_TIFF: AVCodecID = 96;
pub const AV_CODEC_ID_TIERTEXSEQVIDEO: AVCodecID = 95;
pub const AV_CODEC_ID_DSICINVIDEO: AVCodecID = 94;
pub const AV_CODEC_ID_TARGA: AVCodecID = 93;
pub const AV_CODEC_ID_VP6F: AVCodecID = 92;
pub const AV_CODEC_ID_VP6: AVCodecID = 91;
pub const AV_CODEC_ID_VP5: AVCodecID = 90;
pub const AV_CODEC_ID_VMNC: AVCodecID = 89;
pub const AV_CODEC_ID_JPEG2000: AVCodecID = 88;
pub const AV_CODEC_ID_CAVS: AVCodecID = 87;
pub const AV_CODEC_ID_FLASHSV: AVCodecID = 86;
pub const AV_CODEC_ID_KMVC: AVCodecID = 85;
pub const AV_CODEC_ID_NUV: AVCodecID = 84;
pub const AV_CODEC_ID_SMACKVIDEO: AVCodecID = 83;
pub const AV_CODEC_ID_AVS: AVCodecID = 82;
pub const AV_CODEC_ID_ZMBV: AVCodecID = 81;
pub const AV_CODEC_ID_MMVIDEO: AVCodecID = 80;
pub const AV_CODEC_ID_CSCD: AVCodecID = 79;
pub const AV_CODEC_ID_BMP: AVCodecID = 78;
pub const AV_CODEC_ID_TRUEMOTION2: AVCodecID = 77;
pub const AV_CODEC_ID_FRAPS: AVCodecID = 76;
pub const AV_CODEC_ID_INDEO2: AVCodecID = 75;
pub const AV_CODEC_ID_AASC: AVCodecID = 74;
pub const AV_CODEC_ID_WNV1: AVCodecID = 73;
pub const AV_CODEC_ID_LOCO: AVCodecID = 72;
pub const AV_CODEC_ID_WMV3: AVCodecID = 71;
pub const AV_CODEC_ID_VC1: AVCodecID = 70;
pub const AV_CODEC_ID_RV40: AVCodecID = 69;
pub const AV_CODEC_ID_RV30: AVCodecID = 68;
pub const AV_CODEC_ID_FFVHUFF: AVCodecID = 67;
pub const AV_CODEC_ID_PAM: AVCodecID = 66;
pub const AV_CODEC_ID_PGMYUV: AVCodecID = 65;
pub const AV_CODEC_ID_PGM: AVCodecID = 64;
pub const AV_CODEC_ID_PBM: AVCodecID = 63;
pub const AV_CODEC_ID_PPM: AVCodecID = 62;
pub const AV_CODEC_ID_PNG: AVCodecID = 61;
pub const AV_CODEC_ID_QPEG: AVCodecID = 60;
pub const AV_CODEC_ID_VIXL: AVCodecID = 59;
pub const AV_CODEC_ID_QDRAW: AVCodecID = 58;
pub const AV_CODEC_ID_ULTI: AVCodecID = 57;
pub const AV_CODEC_ID_TSCC: AVCodecID = 56;
pub const AV_CODEC_ID_QTRLE: AVCodecID = 55;
pub const AV_CODEC_ID_ZLIB: AVCodecID = 54;
pub const AV_CODEC_ID_MSZH: AVCodecID = 53;
pub const AV_CODEC_ID_VMDVIDEO: AVCodecID = 52;
pub const AV_CODEC_ID_TRUEMOTION1: AVCodecID = 51;
pub const AV_CODEC_ID_FLIC: AVCodecID = 50;
pub const AV_CODEC_ID_SMC: AVCodecID = 49;
pub const AV_CODEC_ID_8BPS: AVCodecID = 48;
pub const AV_CODEC_ID_IDCIN: AVCodecID = 47;
pub const AV_CODEC_ID_MSVIDEO1: AVCodecID = 46;
pub const AV_CODEC_ID_MSRLE: AVCodecID = 45;
pub const AV_CODEC_ID_WS_VQA: AVCodecID = 44;
pub const AV_CODEC_ID_CINEPAK: AVCodecID = 43;
pub const AV_CODEC_ID_RPZA: AVCodecID = 42;
pub const AV_CODEC_ID_XAN_WC4: AVCodecID = 41;
pub const AV_CODEC_ID_XAN_WC3: AVCodecID = 40;
pub const AV_CODEC_ID_INTERPLAY_VIDEO: AVCodecID = 39;
pub const AV_CODEC_ID_ROQ: AVCodecID = 38;
pub const AV_CODEC_ID_MDEC: AVCodecID = 37;
pub const AV_CODEC_ID_CLJR: AVCodecID = 36;
pub const AV_CODEC_ID_VCR1: AVCodecID = 35;
pub const AV_CODEC_ID_4XM: AVCodecID = 34;
pub const AV_CODEC_ID_FFV1: AVCodecID = 33;
pub const AV_CODEC_ID_ASV2: AVCodecID = 32;
pub const AV_CODEC_ID_ASV1: AVCodecID = 31;
pub const AV_CODEC_ID_THEORA: AVCodecID = 30;
pub const AV_CODEC_ID_VP3: AVCodecID = 29;
pub const AV_CODEC_ID_INDEO3: AVCodecID = 28;
pub const AV_CODEC_ID_H264: AVCodecID = 27;
pub const AV_CODEC_ID_CYUV: AVCodecID = 26;
pub const AV_CODEC_ID_HUFFYUV: AVCodecID = 25;
pub const AV_CODEC_ID_DVVIDEO: AVCodecID = 24;
pub const AV_CODEC_ID_SVQ3: AVCodecID = 23;
pub const AV_CODEC_ID_SVQ1: AVCodecID = 22;
pub const AV_CODEC_ID_FLV1: AVCodecID = 21;
pub const AV_CODEC_ID_H263I: AVCodecID = 20;
pub const AV_CODEC_ID_H263P: AVCodecID = 19;
pub const AV_CODEC_ID_WMV2: AVCodecID = 18;
pub const AV_CODEC_ID_WMV1: AVCodecID = 17;
pub const AV_CODEC_ID_MSMPEG4V3: AVCodecID = 16;
pub const AV_CODEC_ID_MSMPEG4V2: AVCodecID = 15;
pub const AV_CODEC_ID_MSMPEG4V1: AVCodecID = 14;
pub const AV_CODEC_ID_RAWVIDEO: AVCodecID = 13;
pub const AV_CODEC_ID_MPEG4: AVCodecID = 12;
pub const AV_CODEC_ID_JPEGLS: AVCodecID = 11;
pub const AV_CODEC_ID_SP5X: AVCodecID = 10;
pub const AV_CODEC_ID_LJPEG: AVCodecID = 9;
pub const AV_CODEC_ID_MJPEGB: AVCodecID = 8;
pub const AV_CODEC_ID_MJPEG: AVCodecID = 7;
pub const AV_CODEC_ID_RV20: AVCodecID = 6;
pub const AV_CODEC_ID_RV10: AVCodecID = 5;
pub const AV_CODEC_ID_H263: AVCodecID = 4;
pub const AV_CODEC_ID_H261: AVCodecID = 3;
pub const AV_CODEC_ID_MPEG2VIDEO: AVCodecID = 2;
pub const AV_CODEC_ID_MPEG1VIDEO: AVCodecID = 1;
pub const AV_CODEC_ID_NONE: AVCodecID = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct AVCodecDescriptor {
    pub id: AVCodecID,
    pub type_0: AVMediaType,
    pub name: *const libc::c_char,
    pub long_name: *const libc::c_char,
    pub props: libc::c_int,
    pub mime_types: *const *const libc::c_char,
    pub profiles: *const AVProfile,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct AVProfile {
    pub profile: libc::c_int,
    pub name: *const libc::c_char,
}
pub type AVDiscard = libc::c_int;
pub const AVDISCARD_ALL: AVDiscard = 48;
pub const AVDISCARD_NONKEY: AVDiscard = 32;
pub const AVDISCARD_NONINTRA: AVDiscard = 24;
pub const AVDISCARD_BIDIR: AVDiscard = 16;
pub const AVDISCARD_NONREF: AVDiscard = 8;
pub const AVDISCARD_DEFAULT: AVDiscard = 0;
pub const AVDISCARD_NONE: AVDiscard = -16;
pub type AVAudioServiceType = libc::c_uint;
pub const AV_AUDIO_SERVICE_TYPE_NB: AVAudioServiceType = 9;
pub const AV_AUDIO_SERVICE_TYPE_KARAOKE: AVAudioServiceType = 8;
pub const AV_AUDIO_SERVICE_TYPE_VOICE_OVER: AVAudioServiceType = 7;
pub const AV_AUDIO_SERVICE_TYPE_EMERGENCY: AVAudioServiceType = 6;
pub const AV_AUDIO_SERVICE_TYPE_COMMENTARY: AVAudioServiceType = 5;
pub const AV_AUDIO_SERVICE_TYPE_DIALOGUE: AVAudioServiceType = 4;
pub const AV_AUDIO_SERVICE_TYPE_HEARING_IMPAIRED: AVAudioServiceType = 3;
pub const AV_AUDIO_SERVICE_TYPE_VISUALLY_IMPAIRED: AVAudioServiceType = 2;
pub const AV_AUDIO_SERVICE_TYPE_EFFECTS: AVAudioServiceType = 1;
pub const AV_AUDIO_SERVICE_TYPE_MAIN: AVAudioServiceType = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct RcOverride {
    pub start_frame: libc::c_int,
    pub end_frame: libc::c_int,
    pub qscale: libc::c_int,
    pub quality_factor: libc::c_float,
}
pub type AVPacketSideDataType = libc::c_uint;
pub const AV_PKT_DATA_NB: AVPacketSideDataType = 27;
pub const AV_PKT_DATA_AFD: AVPacketSideDataType = 26;
pub const AV_PKT_DATA_ENCRYPTION_INFO: AVPacketSideDataType = 25;
pub const AV_PKT_DATA_ENCRYPTION_INIT_INFO: AVPacketSideDataType = 24;
pub const AV_PKT_DATA_A53_CC: AVPacketSideDataType = 23;
pub const AV_PKT_DATA_CONTENT_LIGHT_LEVEL: AVPacketSideDataType = 22;
pub const AV_PKT_DATA_SPHERICAL: AVPacketSideDataType = 21;
pub const AV_PKT_DATA_MASTERING_DISPLAY_METADATA: AVPacketSideDataType = 20;
pub const AV_PKT_DATA_MPEGTS_STREAM_ID: AVPacketSideDataType = 19;
pub const AV_PKT_DATA_METADATA_UPDATE: AVPacketSideDataType = 18;
pub const AV_PKT_DATA_WEBVTT_SETTINGS: AVPacketSideDataType = 17;
pub const AV_PKT_DATA_WEBVTT_IDENTIFIER: AVPacketSideDataType = 16;
pub const AV_PKT_DATA_MATROSKA_BLOCKADDITIONAL: AVPacketSideDataType = 15;
pub const AV_PKT_DATA_SUBTITLE_POSITION: AVPacketSideDataType = 14;
pub const AV_PKT_DATA_STRINGS_METADATA: AVPacketSideDataType = 13;
pub const AV_PKT_DATA_JP_DUALMONO: AVPacketSideDataType = 12;
pub const AV_PKT_DATA_SKIP_SAMPLES: AVPacketSideDataType = 11;
pub const AV_PKT_DATA_CPB_PROPERTIES: AVPacketSideDataType = 10;
pub const AV_PKT_DATA_FALLBACK_TRACK: AVPacketSideDataType = 9;
pub const AV_PKT_DATA_QUALITY_STATS: AVPacketSideDataType = 8;
pub const AV_PKT_DATA_AUDIO_SERVICE_TYPE: AVPacketSideDataType = 7;
pub const AV_PKT_DATA_STEREO3D: AVPacketSideDataType = 6;
pub const AV_PKT_DATA_DISPLAYMATRIX: AVPacketSideDataType = 5;
pub const AV_PKT_DATA_REPLAYGAIN: AVPacketSideDataType = 4;
pub const AV_PKT_DATA_H263_MB_INFO: AVPacketSideDataType = 3;
pub const AV_PKT_DATA_PARAM_CHANGE: AVPacketSideDataType = 2;
pub const AV_PKT_DATA_NEW_EXTRADATA: AVPacketSideDataType = 1;
pub const AV_PKT_DATA_PALETTE: AVPacketSideDataType = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct AVPacketSideData {
    pub data: *mut uint8_t,
    pub size: libc::c_int,
    pub type_0: AVPacketSideDataType,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct AVPacket {
    pub buf: *mut AVBufferRef,
    pub pts: int64_t,
    pub dts: int64_t,
    pub data: *mut uint8_t,
    pub size: libc::c_int,
    pub stream_index: libc::c_int,
    pub flags: libc::c_int,
    pub side_data: *mut AVPacketSideData,
    pub side_data_elems: libc::c_int,
    pub duration: int64_t,
    pub pos: int64_t,
    pub convergence_duration: int64_t,
}
pub type AVFieldOrder = libc::c_uint;
pub const AV_FIELD_BT: AVFieldOrder = 5;
pub const AV_FIELD_TB: AVFieldOrder = 4;
pub const AV_FIELD_BB: AVFieldOrder = 3;
pub const AV_FIELD_TT: AVFieldOrder = 2;
pub const AV_FIELD_PROGRESSIVE: AVFieldOrder = 1;
pub const AV_FIELD_UNKNOWN: AVFieldOrder = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct AVCodecContext {
    pub av_class: *const AVClass,
    pub log_level_offset: libc::c_int,
    pub codec_type: AVMediaType,
    pub codec: *const AVCodec,
    pub codec_id: AVCodecID,
    pub codec_tag: libc::c_uint,
    pub priv_data: *mut libc::c_void,
    pub internal: *mut AVCodecInternal,
    pub opaque: *mut libc::c_void,
    pub bit_rate: int64_t,
    pub bit_rate_tolerance: libc::c_int,
    pub global_quality: libc::c_int,
    pub compression_level: libc::c_int,
    pub flags: libc::c_int,
    pub flags2: libc::c_int,
    pub extradata: *mut uint8_t,
    pub extradata_size: libc::c_int,
    pub time_base: AVRational,
    pub ticks_per_frame: libc::c_int,
    pub delay: libc::c_int,
    pub width: libc::c_int,
    pub height: libc::c_int,
    pub coded_width: libc::c_int,
    pub coded_height: libc::c_int,
    pub gop_size: libc::c_int,
    pub pix_fmt: AVPixelFormat,
    pub draw_horiz_band: Option::<
        unsafe extern "C" fn(
            *mut AVCodecContext,
            *const AVFrame,
            *mut libc::c_int,
            libc::c_int,
            libc::c_int,
            libc::c_int,
        ) -> (),
    >,
    pub get_format: Option::<
        unsafe extern "C" fn(*mut AVCodecContext, *const AVPixelFormat) -> AVPixelFormat,
    >,
    pub max_b_frames: libc::c_int,
    pub b_quant_factor: libc::c_float,
    pub b_frame_strategy: libc::c_int,
    pub b_quant_offset: libc::c_float,
    pub has_b_frames: libc::c_int,
    pub mpeg_quant: libc::c_int,
    pub i_quant_factor: libc::c_float,
    pub i_quant_offset: libc::c_float,
    pub lumi_masking: libc::c_float,
    pub temporal_cplx_masking: libc::c_float,
    pub spatial_cplx_masking: libc::c_float,
    pub p_masking: libc::c_float,
    pub dark_masking: libc::c_float,
    pub slice_count: libc::c_int,
    pub prediction_method: libc::c_int,
    pub slice_offset: *mut libc::c_int,
    pub sample_aspect_ratio: AVRational,
    pub me_cmp: libc::c_int,
    pub me_sub_cmp: libc::c_int,
    pub mb_cmp: libc::c_int,
    pub ildct_cmp: libc::c_int,
    pub dia_size: libc::c_int,
    pub last_predictor_count: libc::c_int,
    pub pre_me: libc::c_int,
    pub me_pre_cmp: libc::c_int,
    pub pre_dia_size: libc::c_int,
    pub me_subpel_quality: libc::c_int,
    pub me_range: libc::c_int,
    pub slice_flags: libc::c_int,
    pub mb_decision: libc::c_int,
    pub intra_matrix: *mut uint16_t,
    pub inter_matrix: *mut uint16_t,
    pub scenechange_threshold: libc::c_int,
    pub noise_reduction: libc::c_int,
    pub intra_dc_precision: libc::c_int,
    pub skip_top: libc::c_int,
    pub skip_bottom: libc::c_int,
    pub mb_lmin: libc::c_int,
    pub mb_lmax: libc::c_int,
    pub me_penalty_compensation: libc::c_int,
    pub bidir_refine: libc::c_int,
    pub brd_scale: libc::c_int,
    pub keyint_min: libc::c_int,
    pub refs: libc::c_int,
    pub chromaoffset: libc::c_int,
    pub mv0_threshold: libc::c_int,
    pub b_sensitivity: libc::c_int,
    pub color_primaries: AVColorPrimaries,
    pub color_trc: AVColorTransferCharacteristic,
    pub colorspace: AVColorSpace,
    pub color_range: AVColorRange,
    pub chroma_sample_location: AVChromaLocation,
    pub slices: libc::c_int,
    pub field_order: AVFieldOrder,
    pub sample_rate: libc::c_int,
    pub channels: libc::c_int,
    pub sample_fmt: AVSampleFormat,
    pub frame_size: libc::c_int,
    pub frame_number: libc::c_int,
    pub block_align: libc::c_int,
    pub cutoff: libc::c_int,
    pub channel_layout: uint64_t,
    pub request_channel_layout: uint64_t,
    pub audio_service_type: AVAudioServiceType,
    pub request_sample_fmt: AVSampleFormat,
    pub get_buffer2: Option::<
        unsafe extern "C" fn(
            *mut AVCodecContext,
            *mut AVFrame,
            libc::c_int,
        ) -> libc::c_int,
    >,
    pub refcounted_frames: libc::c_int,
    pub qcompress: libc::c_float,
    pub qblur: libc::c_float,
    pub qmin: libc::c_int,
    pub qmax: libc::c_int,
    pub max_qdiff: libc::c_int,
    pub rc_buffer_size: libc::c_int,
    pub rc_override_count: libc::c_int,
    pub rc_override: *mut RcOverride,
    pub rc_max_rate: int64_t,
    pub rc_min_rate: int64_t,
    pub rc_max_available_vbv_use: libc::c_float,
    pub rc_min_vbv_overflow_use: libc::c_float,
    pub rc_initial_buffer_occupancy: libc::c_int,
    pub coder_type: libc::c_int,
    pub context_model: libc::c_int,
    pub frame_skip_threshold: libc::c_int,
    pub frame_skip_factor: libc::c_int,
    pub frame_skip_exp: libc::c_int,
    pub frame_skip_cmp: libc::c_int,
    pub trellis: libc::c_int,
    pub min_prediction_order: libc::c_int,
    pub max_prediction_order: libc::c_int,
    pub timecode_frame_start: int64_t,
    pub rtp_callback: Option::<
        unsafe extern "C" fn(
            *mut AVCodecContext,
            *mut libc::c_void,
            libc::c_int,
            libc::c_int,
        ) -> (),
    >,
    pub rtp_payload_size: libc::c_int,
    pub mv_bits: libc::c_int,
    pub header_bits: libc::c_int,
    pub i_tex_bits: libc::c_int,
    pub p_tex_bits: libc::c_int,
    pub i_count: libc::c_int,
    pub p_count: libc::c_int,
    pub skip_count: libc::c_int,
    pub misc_bits: libc::c_int,
    pub frame_bits: libc::c_int,
    pub stats_out: *mut libc::c_char,
    pub stats_in: *mut libc::c_char,
    pub workaround_bugs: libc::c_int,
    pub strict_std_compliance: libc::c_int,
    pub error_concealment: libc::c_int,
    pub debug: libc::c_int,
    pub err_recognition: libc::c_int,
    pub reordered_opaque: int64_t,
    pub hwaccel: *const AVHWAccel,
    pub hwaccel_context: *mut libc::c_void,
    pub error: [uint64_t; 8],
    pub dct_algo: libc::c_int,
    pub idct_algo: libc::c_int,
    pub bits_per_coded_sample: libc::c_int,
    pub bits_per_raw_sample: libc::c_int,
    pub lowres: libc::c_int,
    pub coded_frame: *mut AVFrame,
    pub thread_count: libc::c_int,
    pub thread_type: libc::c_int,
    pub active_thread_type: libc::c_int,
    pub thread_safe_callbacks: libc::c_int,
    pub execute: Option::<
        unsafe extern "C" fn(
            *mut AVCodecContext,
            Option::<
                unsafe extern "C" fn(
                    *mut AVCodecContext,
                    *mut libc::c_void,
                ) -> libc::c_int,
            >,
            *mut libc::c_void,
            *mut libc::c_int,
            libc::c_int,
            libc::c_int,
        ) -> libc::c_int,
    >,
    pub execute2: Option::<
        unsafe extern "C" fn(
            *mut AVCodecContext,
            Option::<
                unsafe extern "C" fn(
                    *mut AVCodecContext,
                    *mut libc::c_void,
                    libc::c_int,
                    libc::c_int,
                ) -> libc::c_int,
            >,
            *mut libc::c_void,
            *mut libc::c_int,
            libc::c_int,
        ) -> libc::c_int,
    >,
    pub nsse_weight: libc::c_int,
    pub profile: libc::c_int,
    pub level: libc::c_int,
    pub skip_loop_filter: AVDiscard,
    pub skip_idct: AVDiscard,
    pub skip_frame: AVDiscard,
    pub subtitle_header: *mut uint8_t,
    pub subtitle_header_size: libc::c_int,
    pub vbv_delay: uint64_t,
    pub side_data_only_packets: libc::c_int,
    pub initial_padding: libc::c_int,
    pub framerate: AVRational,
    pub sw_pix_fmt: AVPixelFormat,
    pub pkt_timebase: AVRational,
    pub codec_descriptor: *const AVCodecDescriptor,
    pub pts_correction_num_faulty_pts: int64_t,
    pub pts_correction_num_faulty_dts: int64_t,
    pub pts_correction_last_pts: int64_t,
    pub pts_correction_last_dts: int64_t,
    pub sub_charenc: *mut libc::c_char,
    pub sub_charenc_mode: libc::c_int,
    pub skip_alpha: libc::c_int,
    pub seek_preroll: libc::c_int,
    pub debug_mv: libc::c_int,
    pub chroma_intra_matrix: *mut uint16_t,
    pub dump_separator: *mut uint8_t,
    pub codec_whitelist: *mut libc::c_char,
    pub properties: libc::c_uint,
    pub coded_side_data: *mut AVPacketSideData,
    pub nb_coded_side_data: libc::c_int,
    pub hw_frames_ctx: *mut AVBufferRef,
    pub sub_text_format: libc::c_int,
    pub trailing_padding: libc::c_int,
    pub max_pixels: int64_t,
    pub hw_device_ctx: *mut AVBufferRef,
    pub hwaccel_flags: libc::c_int,
    pub apply_cropping: libc::c_int,
    pub extra_hw_frames: libc::c_int,
    pub discard_damaged_percentage: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct AVHWAccel {
    pub name: *const libc::c_char,
    pub type_0: AVMediaType,
    pub id: AVCodecID,
    pub pix_fmt: AVPixelFormat,
    pub capabilities: libc::c_int,
    pub alloc_frame: Option::<
        unsafe extern "C" fn(*mut AVCodecContext, *mut AVFrame) -> libc::c_int,
    >,
    pub start_frame: Option::<
        unsafe extern "C" fn(
            *mut AVCodecContext,
            *const uint8_t,
            uint32_t,
        ) -> libc::c_int,
    >,
    pub decode_params: Option::<
        unsafe extern "C" fn(
            *mut AVCodecContext,
            libc::c_int,
            *const uint8_t,
            uint32_t,
        ) -> libc::c_int,
    >,
    pub decode_slice: Option::<
        unsafe extern "C" fn(
            *mut AVCodecContext,
            *const uint8_t,
            uint32_t,
        ) -> libc::c_int,
    >,
    pub end_frame: Option::<unsafe extern "C" fn(*mut AVCodecContext) -> libc::c_int>,
    pub frame_priv_data_size: libc::c_int,
    pub decode_mb: Option::<unsafe extern "C" fn(*mut MpegEncContext) -> ()>,
    pub init: Option::<unsafe extern "C" fn(*mut AVCodecContext) -> libc::c_int>,
    pub uninit: Option::<unsafe extern "C" fn(*mut AVCodecContext) -> libc::c_int>,
    pub priv_data_size: libc::c_int,
    pub caps_internal: libc::c_int,
    pub frame_params: Option::<
        unsafe extern "C" fn(*mut AVCodecContext, *mut AVBufferRef) -> libc::c_int,
    >,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct AVCodec {
    pub name: *const libc::c_char,
    pub long_name: *const libc::c_char,
    pub type_0: AVMediaType,
    pub id: AVCodecID,
    pub capabilities: libc::c_int,
    pub supported_framerates: *const AVRational,
    pub pix_fmts: *const AVPixelFormat,
    pub supported_samplerates: *const libc::c_int,
    pub sample_fmts: *const AVSampleFormat,
    pub channel_layouts: *const uint64_t,
    pub max_lowres: uint8_t,
    pub priv_class: *const AVClass,
    pub profiles: *const AVProfile,
    pub wrapper_name: *const libc::c_char,
    pub priv_data_size: libc::c_int,
    pub next: *mut AVCodec,
    pub init_thread_copy: Option::<
        unsafe extern "C" fn(*mut AVCodecContext) -> libc::c_int,
    >,
    pub update_thread_context: Option::<
        unsafe extern "C" fn(*mut AVCodecContext, *const AVCodecContext) -> libc::c_int,
    >,
    pub defaults: *const AVCodecDefault,
    pub init_static_data: Option::<unsafe extern "C" fn(*mut AVCodec) -> ()>,
    pub init: Option::<unsafe extern "C" fn(*mut AVCodecContext) -> libc::c_int>,
    pub encode_sub: Option::<
        unsafe extern "C" fn(
            *mut AVCodecContext,
            *mut uint8_t,
            libc::c_int,
            *const AVSubtitle,
        ) -> libc::c_int,
    >,
    pub encode2: Option::<
        unsafe extern "C" fn(
            *mut AVCodecContext,
            *mut AVPacket,
            *const AVFrame,
            *mut libc::c_int,
        ) -> libc::c_int,
    >,
    pub decode: Option::<
        unsafe extern "C" fn(
            *mut AVCodecContext,
            *mut libc::c_void,
            *mut libc::c_int,
            *mut AVPacket,
        ) -> libc::c_int,
    >,
    pub close: Option::<unsafe extern "C" fn(*mut AVCodecContext) -> libc::c_int>,
    pub send_frame: Option::<
        unsafe extern "C" fn(*mut AVCodecContext, *const AVFrame) -> libc::c_int,
    >,
    pub receive_packet: Option::<
        unsafe extern "C" fn(*mut AVCodecContext, *mut AVPacket) -> libc::c_int,
    >,
    pub receive_frame: Option::<
        unsafe extern "C" fn(*mut AVCodecContext, *mut AVFrame) -> libc::c_int,
    >,
    pub flush: Option::<unsafe extern "C" fn(*mut AVCodecContext) -> ()>,
    pub caps_internal: libc::c_int,
    pub bsfs: *const libc::c_char,
    pub hw_configs: *mut *const AVCodecHWConfigInternal,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct AVSubtitle {
    pub format: uint16_t,
    pub start_display_time: uint32_t,
    pub end_display_time: uint32_t,
    pub num_rects: libc::c_uint,
    pub rects: *mut *mut AVSubtitleRect,
    pub pts: int64_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct AVSubtitleRect {
    pub x: libc::c_int,
    pub y: libc::c_int,
    pub w: libc::c_int,
    pub h: libc::c_int,
    pub nb_colors: libc::c_int,
    pub pict: AVPicture,
    pub data: [*mut uint8_t; 4],
    pub linesize: [libc::c_int; 4],
    pub type_0: AVSubtitleType,
    pub text: *mut libc::c_char,
    pub ass: *mut libc::c_char,
    pub flags: libc::c_int,
}
pub type AVSubtitleType = libc::c_uint;
pub const SUBTITLE_ASS: AVSubtitleType = 3;
pub const SUBTITLE_TEXT: AVSubtitleType = 2;
pub const SUBTITLE_BITMAP: AVSubtitleType = 1;
pub const SUBTITLE_NONE: AVSubtitleType = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct AVPicture {
    pub data: [*mut uint8_t; 8],
    pub linesize: [libc::c_int; 8],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct AVCodecParameters {
    pub codec_type: AVMediaType,
    pub codec_id: AVCodecID,
    pub codec_tag: uint32_t,
    pub extradata: *mut uint8_t,
    pub extradata_size: libc::c_int,
    pub format: libc::c_int,
    pub bit_rate: int64_t,
    pub bits_per_coded_sample: libc::c_int,
    pub bits_per_raw_sample: libc::c_int,
    pub profile: libc::c_int,
    pub level: libc::c_int,
    pub width: libc::c_int,
    pub height: libc::c_int,
    pub sample_aspect_ratio: AVRational,
    pub field_order: AVFieldOrder,
    pub color_range: AVColorRange,
    pub color_primaries: AVColorPrimaries,
    pub color_trc: AVColorTransferCharacteristic,
    pub color_space: AVColorSpace,
    pub chroma_location: AVChromaLocation,
    pub video_delay: libc::c_int,
    pub channel_layout: uint64_t,
    pub channels: libc::c_int,
    pub sample_rate: libc::c_int,
    pub block_align: libc::c_int,
    pub frame_size: libc::c_int,
    pub initial_padding: libc::c_int,
    pub trailing_padding: libc::c_int,
    pub seek_preroll: libc::c_int,
}
pub type AVPictureStructure = libc::c_uint;
pub const AV_PICTURE_STRUCTURE_FRAME: AVPictureStructure = 3;
pub const AV_PICTURE_STRUCTURE_BOTTOM_FIELD: AVPictureStructure = 2;
pub const AV_PICTURE_STRUCTURE_TOP_FIELD: AVPictureStructure = 1;
pub const AV_PICTURE_STRUCTURE_UNKNOWN: AVPictureStructure = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct AVCodecParserContext {
    pub priv_data: *mut libc::c_void,
    pub parser: *mut AVCodecParser,
    pub frame_offset: int64_t,
    pub cur_offset: int64_t,
    pub next_frame_offset: int64_t,
    pub pict_type: libc::c_int,
    pub repeat_pict: libc::c_int,
    pub pts: int64_t,
    pub dts: int64_t,
    pub last_pts: int64_t,
    pub last_dts: int64_t,
    pub fetch_timestamp: libc::c_int,
    pub cur_frame_start_index: libc::c_int,
    pub cur_frame_offset: [int64_t; 4],
    pub cur_frame_pts: [int64_t; 4],
    pub cur_frame_dts: [int64_t; 4],
    pub flags: libc::c_int,
    pub offset: int64_t,
    pub cur_frame_end: [int64_t; 4],
    pub key_frame: libc::c_int,
    pub convergence_duration: int64_t,
    pub dts_sync_point: libc::c_int,
    pub dts_ref_dts_delta: libc::c_int,
    pub pts_dts_delta: libc::c_int,
    pub cur_frame_pos: [int64_t; 4],
    pub pos: int64_t,
    pub last_pos: int64_t,
    pub duration: libc::c_int,
    pub field_order: AVFieldOrder,
    pub picture_structure: AVPictureStructure,
    pub output_picture_number: libc::c_int,
    pub width: libc::c_int,
    pub height: libc::c_int,
    pub coded_width: libc::c_int,
    pub coded_height: libc::c_int,
    pub format: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct AVCodecParser {
    pub codec_ids: [libc::c_int; 5],
    pub priv_data_size: libc::c_int,
    pub parser_init: Option::<
        unsafe extern "C" fn(*mut AVCodecParserContext) -> libc::c_int,
    >,
    pub parser_parse: Option::<
        unsafe extern "C" fn(
            *mut AVCodecParserContext,
            *mut AVCodecContext,
            *mut *const uint8_t,
            *mut libc::c_int,
            *const uint8_t,
            libc::c_int,
        ) -> libc::c_int,
    >,
    pub parser_close: Option::<unsafe extern "C" fn(*mut AVCodecParserContext) -> ()>,
    pub split: Option::<
        unsafe extern "C" fn(
            *mut AVCodecContext,
            *const uint8_t,
            libc::c_int,
        ) -> libc::c_int,
    >,
    pub next: *mut AVCodecParser,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct AVFilterContext {
    pub av_class: *const AVClass,
    pub filter: *const AVFilter,
    pub name: *mut libc::c_char,
    pub input_pads: *mut AVFilterPad,
    pub inputs: *mut *mut AVFilterLink,
    pub nb_inputs: libc::c_uint,
    pub output_pads: *mut AVFilterPad,
    pub outputs: *mut *mut AVFilterLink,
    pub nb_outputs: libc::c_uint,
    pub priv_0: *mut libc::c_void,
    pub graph: *mut AVFilterGraph,
    pub thread_type: libc::c_int,
    pub internal: *mut AVFilterInternal,
    pub command_queue: *mut AVFilterCommand,
    pub enable_str: *mut libc::c_char,
    pub enable: *mut libc::c_void,
    pub var_values: *mut libc::c_double,
    pub is_disabled: libc::c_int,
    pub hw_device_ctx: *mut AVBufferRef,
    pub nb_threads: libc::c_int,
    pub ready: libc::c_uint,
    pub extra_hw_frames: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct AVFilterGraph {
    pub av_class: *const AVClass,
    pub filters: *mut *mut AVFilterContext,
    pub nb_filters: libc::c_uint,
    pub scale_sws_opts: *mut libc::c_char,
    pub resample_lavr_opts: *mut libc::c_char,
    pub thread_type: libc::c_int,
    pub nb_threads: libc::c_int,
    pub internal: *mut AVFilterGraphInternal,
    pub opaque: *mut libc::c_void,
    pub execute: Option::<avfilter_execute_func>,
    pub aresample_swr_opts: *mut libc::c_char,
    pub sink_links: *mut *mut AVFilterLink,
    pub sink_links_count: libc::c_int,
    pub disable_auto_convert: libc::c_uint,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct AVFilterLink {
    pub src: *mut AVFilterContext,
    pub srcpad: *mut AVFilterPad,
    pub dst: *mut AVFilterContext,
    pub dstpad: *mut AVFilterPad,
    pub type_0: AVMediaType,
    pub w: libc::c_int,
    pub h: libc::c_int,
    pub sample_aspect_ratio: AVRational,
    pub channel_layout: uint64_t,
    pub sample_rate: libc::c_int,
    pub format: libc::c_int,
    pub time_base: AVRational,
    pub in_formats: *mut AVFilterFormats,
    pub out_formats: *mut AVFilterFormats,
    pub in_samplerates: *mut AVFilterFormats,
    pub out_samplerates: *mut AVFilterFormats,
    pub in_channel_layouts: *mut AVFilterChannelLayouts,
    pub out_channel_layouts: *mut AVFilterChannelLayouts,
    pub request_samples: libc::c_int,
    pub init_state: C2RustUnnamed_4,
    pub graph: *mut AVFilterGraph,
    pub current_pts: int64_t,
    pub current_pts_us: int64_t,
    pub age_index: libc::c_int,
    pub frame_rate: AVRational,
    pub partial_buf: *mut AVFrame,
    pub partial_buf_size: libc::c_int,
    pub min_samples: libc::c_int,
    pub max_samples: libc::c_int,
    pub channels: libc::c_int,
    pub flags: libc::c_uint,
    pub frame_count_in: int64_t,
    pub frame_count_out: int64_t,
    pub frame_pool: *mut libc::c_void,
    pub frame_wanted_out: libc::c_int,
    pub hw_frames_ctx: *mut AVBufferRef,
    pub reserved: [libc::c_char; 61440],
}
pub type C2RustUnnamed_4 = libc::c_uint;
pub const AVLINK_INIT: C2RustUnnamed_4 = 2;
pub const AVLINK_STARTINIT: C2RustUnnamed_4 = 1;
pub const AVLINK_UNINIT: C2RustUnnamed_4 = 0;
pub type avfilter_execute_func = unsafe extern "C" fn(
    *mut AVFilterContext,
    Option::<avfilter_action_func>,
    *mut libc::c_void,
    *mut libc::c_int,
    libc::c_int,
) -> libc::c_int;
pub type avfilter_action_func = unsafe extern "C" fn(
    *mut AVFilterContext,
    *mut libc::c_void,
    libc::c_int,
    libc::c_int,
) -> libc::c_int;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct AVFilter {
    pub name: *const libc::c_char,
    pub description: *const libc::c_char,
    pub inputs: *const AVFilterPad,
    pub outputs: *const AVFilterPad,
    pub priv_class: *const AVClass,
    pub flags: libc::c_int,
    pub preinit: Option::<unsafe extern "C" fn(*mut AVFilterContext) -> libc::c_int>,
    pub init: Option::<unsafe extern "C" fn(*mut AVFilterContext) -> libc::c_int>,
    pub init_dict: Option::<
        unsafe extern "C" fn(*mut AVFilterContext, *mut *mut AVDictionary) -> libc::c_int,
    >,
    pub uninit: Option::<unsafe extern "C" fn(*mut AVFilterContext) -> ()>,
    pub query_formats: Option::<
        unsafe extern "C" fn(*mut AVFilterContext) -> libc::c_int,
    >,
    pub priv_size: libc::c_int,
    pub flags_internal: libc::c_int,
    pub next: *mut AVFilter,
    pub process_command: Option::<
        unsafe extern "C" fn(
            *mut AVFilterContext,
            *const libc::c_char,
            *const libc::c_char,
            *mut libc::c_char,
            libc::c_int,
            libc::c_int,
        ) -> libc::c_int,
    >,
    pub init_opaque: Option::<
        unsafe extern "C" fn(*mut AVFilterContext, *mut libc::c_void) -> libc::c_int,
    >,
    pub activate: Option::<unsafe extern "C" fn(*mut AVFilterContext) -> libc::c_int>,
}
pub type sig_atomic_t = __sig_atomic_t;
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
pub struct AVIOInterruptCB {
    pub callback: Option::<unsafe extern "C" fn(*mut libc::c_void) -> libc::c_int>,
    pub opaque: *mut libc::c_void,
}
pub type AVIODataMarkerType = libc::c_uint;
pub const AVIO_DATA_MARKER_FLUSH_POINT: AVIODataMarkerType = 5;
pub const AVIO_DATA_MARKER_TRAILER: AVIODataMarkerType = 4;
pub const AVIO_DATA_MARKER_UNKNOWN: AVIODataMarkerType = 3;
pub const AVIO_DATA_MARKER_BOUNDARY_POINT: AVIODataMarkerType = 2;
pub const AVIO_DATA_MARKER_SYNC_POINT: AVIODataMarkerType = 1;
pub const AVIO_DATA_MARKER_HEADER: AVIODataMarkerType = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct AVIOContext {
    pub av_class: *const AVClass,
    pub buffer: *mut libc::c_uchar,
    pub buffer_size: libc::c_int,
    pub buf_ptr: *mut libc::c_uchar,
    pub buf_end: *mut libc::c_uchar,
    pub opaque: *mut libc::c_void,
    pub read_packet: Option::<
        unsafe extern "C" fn(*mut libc::c_void, *mut uint8_t, libc::c_int) -> libc::c_int,
    >,
    pub write_packet: Option::<
        unsafe extern "C" fn(*mut libc::c_void, *mut uint8_t, libc::c_int) -> libc::c_int,
    >,
    pub seek: Option::<
        unsafe extern "C" fn(*mut libc::c_void, int64_t, libc::c_int) -> int64_t,
    >,
    pub pos: int64_t,
    pub eof_reached: libc::c_int,
    pub write_flag: libc::c_int,
    pub max_packet_size: libc::c_int,
    pub checksum: libc::c_ulong,
    pub checksum_ptr: *mut libc::c_uchar,
    pub update_checksum: Option::<
        unsafe extern "C" fn(
            libc::c_ulong,
            *const uint8_t,
            libc::c_uint,
        ) -> libc::c_ulong,
    >,
    pub error: libc::c_int,
    pub read_pause: Option::<
        unsafe extern "C" fn(*mut libc::c_void, libc::c_int) -> libc::c_int,
    >,
    pub read_seek: Option::<
        unsafe extern "C" fn(
            *mut libc::c_void,
            libc::c_int,
            int64_t,
            libc::c_int,
        ) -> int64_t,
    >,
    pub seekable: libc::c_int,
    pub maxsize: int64_t,
    pub direct: libc::c_int,
    pub bytes_read: int64_t,
    pub seek_count: libc::c_int,
    pub writeout_count: libc::c_int,
    pub orig_buffer_size: libc::c_int,
    pub short_seek_threshold: libc::c_int,
    pub protocol_whitelist: *const libc::c_char,
    pub protocol_blacklist: *const libc::c_char,
    pub write_data_type: Option::<
        unsafe extern "C" fn(
            *mut libc::c_void,
            *mut uint8_t,
            libc::c_int,
            AVIODataMarkerType,
            int64_t,
        ) -> libc::c_int,
    >,
    pub ignore_boundary_point: libc::c_int,
    pub current_type: AVIODataMarkerType,
    pub last_time: int64_t,
    pub short_seek_get: Option::<unsafe extern "C" fn(*mut libc::c_void) -> libc::c_int>,
    pub written: int64_t,
    pub buf_ptr_max: *mut libc::c_uchar,
    pub min_packet_size: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct AVFormatContext {
    pub av_class: *const AVClass,
    pub iformat: *mut AVInputFormat,
    pub oformat: *mut AVOutputFormat,
    pub priv_data: *mut libc::c_void,
    pub pb: *mut AVIOContext,
    pub ctx_flags: libc::c_int,
    pub nb_streams: libc::c_uint,
    pub streams: *mut *mut AVStream,
    pub filename: [libc::c_char; 1024],
    pub url: *mut libc::c_char,
    pub start_time: int64_t,
    pub duration: int64_t,
    pub bit_rate: int64_t,
    pub packet_size: libc::c_uint,
    pub max_delay: libc::c_int,
    pub flags: libc::c_int,
    pub probesize: int64_t,
    pub max_analyze_duration: int64_t,
    pub key: *const uint8_t,
    pub keylen: libc::c_int,
    pub nb_programs: libc::c_uint,
    pub programs: *mut *mut AVProgram,
    pub video_codec_id: AVCodecID,
    pub audio_codec_id: AVCodecID,
    pub subtitle_codec_id: AVCodecID,
    pub max_index_size: libc::c_uint,
    pub max_picture_buffer: libc::c_uint,
    pub nb_chapters: libc::c_uint,
    pub chapters: *mut *mut AVChapter,
    pub metadata: *mut AVDictionary,
    pub start_time_realtime: int64_t,
    pub fps_probe_size: libc::c_int,
    pub error_recognition: libc::c_int,
    pub interrupt_callback: AVIOInterruptCB,
    pub debug: libc::c_int,
    pub max_interleave_delta: int64_t,
    pub strict_std_compliance: libc::c_int,
    pub event_flags: libc::c_int,
    pub max_ts_probe: libc::c_int,
    pub avoid_negative_ts: libc::c_int,
    pub ts_id: libc::c_int,
    pub audio_preload: libc::c_int,
    pub max_chunk_duration: libc::c_int,
    pub max_chunk_size: libc::c_int,
    pub use_wallclock_as_timestamps: libc::c_int,
    pub avio_flags: libc::c_int,
    pub duration_estimation_method: AVDurationEstimationMethod,
    pub skip_initial_bytes: int64_t,
    pub correct_ts_overflow: libc::c_uint,
    pub seek2any: libc::c_int,
    pub flush_packets: libc::c_int,
    pub probe_score: libc::c_int,
    pub format_probesize: libc::c_int,
    pub codec_whitelist: *mut libc::c_char,
    pub format_whitelist: *mut libc::c_char,
    pub internal: *mut AVFormatInternal,
    pub io_repositioned: libc::c_int,
    pub video_codec: *mut AVCodec,
    pub audio_codec: *mut AVCodec,
    pub subtitle_codec: *mut AVCodec,
    pub data_codec: *mut AVCodec,
    pub metadata_header_padding: libc::c_int,
    pub opaque: *mut libc::c_void,
    pub control_message_cb: av_format_control_message,
    pub output_ts_offset: int64_t,
    pub dump_separator: *mut uint8_t,
    pub data_codec_id: AVCodecID,
    pub open_cb: Option::<
        unsafe extern "C" fn(
            *mut AVFormatContext,
            *mut *mut AVIOContext,
            *const libc::c_char,
            libc::c_int,
            *const AVIOInterruptCB,
            *mut *mut AVDictionary,
        ) -> libc::c_int,
    >,
    pub protocol_whitelist: *mut libc::c_char,
    pub io_open: Option::<
        unsafe extern "C" fn(
            *mut AVFormatContext,
            *mut *mut AVIOContext,
            *const libc::c_char,
            libc::c_int,
            *mut *mut AVDictionary,
        ) -> libc::c_int,
    >,
    pub io_close: Option::<
        unsafe extern "C" fn(*mut AVFormatContext, *mut AVIOContext) -> (),
    >,
    pub protocol_blacklist: *mut libc::c_char,
    pub max_streams: libc::c_int,
    pub skip_estimate_duration_from_pts: libc::c_int,
}
pub type av_format_control_message = Option::<
    unsafe extern "C" fn(
        *mut AVFormatContext,
        libc::c_int,
        *mut libc::c_void,
        size_t,
    ) -> libc::c_int,
>;
pub type AVDurationEstimationMethod = libc::c_uint;
pub const AVFMT_DURATION_FROM_BITRATE: AVDurationEstimationMethod = 2;
pub const AVFMT_DURATION_FROM_STREAM: AVDurationEstimationMethod = 1;
pub const AVFMT_DURATION_FROM_PTS: AVDurationEstimationMethod = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct AVChapter {
    pub id: libc::c_int,
    pub time_base: AVRational,
    pub start: int64_t,
    pub end: int64_t,
    pub metadata: *mut AVDictionary,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct AVProgram {
    pub id: libc::c_int,
    pub flags: libc::c_int,
    pub discard: AVDiscard,
    pub stream_index: *mut libc::c_uint,
    pub nb_stream_indexes: libc::c_uint,
    pub metadata: *mut AVDictionary,
    pub program_num: libc::c_int,
    pub pmt_pid: libc::c_int,
    pub pcr_pid: libc::c_int,
    pub pmt_version: libc::c_int,
    pub start_time: int64_t,
    pub end_time: int64_t,
    pub pts_wrap_reference: int64_t,
    pub pts_wrap_behavior: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct AVStream {
    pub index: libc::c_int,
    pub id: libc::c_int,
    pub codec: *mut AVCodecContext,
    pub priv_data: *mut libc::c_void,
    pub time_base: AVRational,
    pub start_time: int64_t,
    pub duration: int64_t,
    pub nb_frames: int64_t,
    pub disposition: libc::c_int,
    pub discard: AVDiscard,
    pub sample_aspect_ratio: AVRational,
    pub metadata: *mut AVDictionary,
    pub avg_frame_rate: AVRational,
    pub attached_pic: AVPacket,
    pub side_data: *mut AVPacketSideData,
    pub nb_side_data: libc::c_int,
    pub event_flags: libc::c_int,
    pub r_frame_rate: AVRational,
    pub recommended_encoder_configuration: *mut libc::c_char,
    pub codecpar: *mut AVCodecParameters,
    pub info: *mut C2RustUnnamed_5,
    pub pts_wrap_bits: libc::c_int,
    pub first_dts: int64_t,
    pub cur_dts: int64_t,
    pub last_IP_pts: int64_t,
    pub last_IP_duration: libc::c_int,
    pub probe_packets: libc::c_int,
    pub codec_info_nb_frames: libc::c_int,
    pub need_parsing: AVStreamParseType,
    pub parser: *mut AVCodecParserContext,
    pub last_in_packet_buffer: *mut AVPacketList,
    pub probe_data: AVProbeData,
    pub pts_buffer: [int64_t; 17],
    pub index_entries: *mut AVIndexEntry,
    pub nb_index_entries: libc::c_int,
    pub index_entries_allocated_size: libc::c_uint,
    pub stream_identifier: libc::c_int,
    pub program_num: libc::c_int,
    pub pmt_version: libc::c_int,
    pub pmt_stream_idx: libc::c_int,
    pub interleaver_chunk_size: int64_t,
    pub interleaver_chunk_duration: int64_t,
    pub request_probe: libc::c_int,
    pub skip_to_keyframe: libc::c_int,
    pub skip_samples: libc::c_int,
    pub start_skip_samples: int64_t,
    pub first_discard_sample: int64_t,
    pub last_discard_sample: int64_t,
    pub nb_decoded_frames: libc::c_int,
    pub mux_ts_offset: int64_t,
    pub pts_wrap_reference: int64_t,
    pub pts_wrap_behavior: libc::c_int,
    pub update_initial_durations_done: libc::c_int,
    pub pts_reorder_error: [int64_t; 17],
    pub pts_reorder_error_count: [uint8_t; 17],
    pub last_dts_for_order_check: int64_t,
    pub dts_ordered: uint8_t,
    pub dts_misordered: uint8_t,
    pub inject_global_side_data: libc::c_int,
    pub display_aspect_ratio: AVRational,
    pub internal: *mut AVStreamInternal,
}
#[derive(Copy, Clone, BitfieldStruct)]
#[repr(C)]
pub struct AVIndexEntry {
    pub pos: int64_t,
    pub timestamp: int64_t,
    #[bitfield(name = "flags", ty = "libc::c_int", bits = "0..=1")]
    #[bitfield(name = "size", ty = "libc::c_int", bits = "2..=31")]
    pub flags_size: [u8; 4],
    pub min_distance: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct AVProbeData {
    pub filename: *const libc::c_char,
    pub buf: *mut libc::c_uchar,
    pub buf_size: libc::c_int,
    pub mime_type: *const libc::c_char,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct AVPacketList {
    pub pkt: AVPacket,
    pub next: *mut AVPacketList,
}
pub type AVStreamParseType = libc::c_uint;
pub const AVSTREAM_PARSE_FULL_RAW: AVStreamParseType = 5;
pub const AVSTREAM_PARSE_FULL_ONCE: AVStreamParseType = 4;
pub const AVSTREAM_PARSE_TIMESTAMPS: AVStreamParseType = 3;
pub const AVSTREAM_PARSE_HEADERS: AVStreamParseType = 2;
pub const AVSTREAM_PARSE_FULL: AVStreamParseType = 1;
pub const AVSTREAM_PARSE_NONE: AVStreamParseType = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_5 {
    pub last_dts: int64_t,
    pub duration_gcd: int64_t,
    pub duration_count: libc::c_int,
    pub rfps_duration_sum: int64_t,
    pub duration_error: *mut [[libc::c_double; 399]; 2],
    pub codec_info_duration: int64_t,
    pub codec_info_duration_fields: int64_t,
    pub frame_delay_evidence: libc::c_int,
    pub found_decoder: libc::c_int,
    pub last_duration: int64_t,
    pub fps_first_dts: int64_t,
    pub fps_first_dts_idx: libc::c_int,
    pub fps_last_dts: int64_t,
    pub fps_last_dts_idx: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct AVOutputFormat {
    pub name: *const libc::c_char,
    pub long_name: *const libc::c_char,
    pub mime_type: *const libc::c_char,
    pub extensions: *const libc::c_char,
    pub audio_codec: AVCodecID,
    pub video_codec: AVCodecID,
    pub subtitle_codec: AVCodecID,
    pub flags: libc::c_int,
    pub codec_tag: *const *const AVCodecTag,
    pub priv_class: *const AVClass,
    pub next: *mut AVOutputFormat,
    pub priv_data_size: libc::c_int,
    pub write_header: Option::<
        unsafe extern "C" fn(*mut AVFormatContext) -> libc::c_int,
    >,
    pub write_packet: Option::<
        unsafe extern "C" fn(*mut AVFormatContext, *mut AVPacket) -> libc::c_int,
    >,
    pub write_trailer: Option::<
        unsafe extern "C" fn(*mut AVFormatContext) -> libc::c_int,
    >,
    pub interleave_packet: Option::<
        unsafe extern "C" fn(
            *mut AVFormatContext,
            *mut AVPacket,
            *mut AVPacket,
            libc::c_int,
        ) -> libc::c_int,
    >,
    pub query_codec: Option::<
        unsafe extern "C" fn(AVCodecID, libc::c_int) -> libc::c_int,
    >,
    pub get_output_timestamp: Option::<
        unsafe extern "C" fn(
            *mut AVFormatContext,
            libc::c_int,
            *mut int64_t,
            *mut int64_t,
        ) -> (),
    >,
    pub control_message: Option::<
        unsafe extern "C" fn(
            *mut AVFormatContext,
            libc::c_int,
            *mut libc::c_void,
            size_t,
        ) -> libc::c_int,
    >,
    pub write_uncoded_frame: Option::<
        unsafe extern "C" fn(
            *mut AVFormatContext,
            libc::c_int,
            *mut *mut AVFrame,
            libc::c_uint,
        ) -> libc::c_int,
    >,
    pub get_device_list: Option::<
        unsafe extern "C" fn(*mut AVFormatContext, *mut AVDeviceInfoList) -> libc::c_int,
    >,
    pub create_device_capabilities: Option::<
        unsafe extern "C" fn(
            *mut AVFormatContext,
            *mut AVDeviceCapabilitiesQuery,
        ) -> libc::c_int,
    >,
    pub free_device_capabilities: Option::<
        unsafe extern "C" fn(
            *mut AVFormatContext,
            *mut AVDeviceCapabilitiesQuery,
        ) -> libc::c_int,
    >,
    pub data_codec: AVCodecID,
    pub init: Option::<unsafe extern "C" fn(*mut AVFormatContext) -> libc::c_int>,
    pub deinit: Option::<unsafe extern "C" fn(*mut AVFormatContext) -> ()>,
    pub check_bitstream: Option::<
        unsafe extern "C" fn(*mut AVFormatContext, *const AVPacket) -> libc::c_int,
    >,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct AVInputFormat {
    pub name: *const libc::c_char,
    pub long_name: *const libc::c_char,
    pub flags: libc::c_int,
    pub extensions: *const libc::c_char,
    pub codec_tag: *const *const AVCodecTag,
    pub priv_class: *const AVClass,
    pub mime_type: *const libc::c_char,
    pub next: *mut AVInputFormat,
    pub raw_codec_id: libc::c_int,
    pub priv_data_size: libc::c_int,
    pub read_probe: Option::<unsafe extern "C" fn(*const AVProbeData) -> libc::c_int>,
    pub read_header: Option::<unsafe extern "C" fn(*mut AVFormatContext) -> libc::c_int>,
    pub read_packet: Option::<
        unsafe extern "C" fn(*mut AVFormatContext, *mut AVPacket) -> libc::c_int,
    >,
    pub read_close: Option::<unsafe extern "C" fn(*mut AVFormatContext) -> libc::c_int>,
    pub read_seek: Option::<
        unsafe extern "C" fn(
            *mut AVFormatContext,
            libc::c_int,
            int64_t,
            libc::c_int,
        ) -> libc::c_int,
    >,
    pub read_timestamp: Option::<
        unsafe extern "C" fn(
            *mut AVFormatContext,
            libc::c_int,
            *mut int64_t,
            int64_t,
        ) -> int64_t,
    >,
    pub read_play: Option::<unsafe extern "C" fn(*mut AVFormatContext) -> libc::c_int>,
    pub read_pause: Option::<unsafe extern "C" fn(*mut AVFormatContext) -> libc::c_int>,
    pub read_seek2: Option::<
        unsafe extern "C" fn(
            *mut AVFormatContext,
            libc::c_int,
            int64_t,
            int64_t,
            int64_t,
            libc::c_int,
        ) -> libc::c_int,
    >,
    pub get_device_list: Option::<
        unsafe extern "C" fn(*mut AVFormatContext, *mut AVDeviceInfoList) -> libc::c_int,
    >,
    pub create_device_capabilities: Option::<
        unsafe extern "C" fn(
            *mut AVFormatContext,
            *mut AVDeviceCapabilitiesQuery,
        ) -> libc::c_int,
    >,
    pub free_device_capabilities: Option::<
        unsafe extern "C" fn(
            *mut AVFormatContext,
            *mut AVDeviceCapabilitiesQuery,
        ) -> libc::c_int,
    >,
}
pub type PianoAudioQuality_t = libc::c_uint;
pub const PIANO_AQ_HIGH: PianoAudioQuality_t = 3;
pub const PIANO_AQ_MEDIUM: PianoAudioQuality_t = 2;
pub const PIANO_AQ_LOW: PianoAudioQuality_t = 1;
pub const PIANO_AQ_UNKNOWN: PianoAudioQuality_t = 0;
pub type BarStationSorting_t = libc::c_uint;
pub const BAR_SORT_COUNT: BarStationSorting_t = 6;
pub const BAR_SORT_QUICKMIX_10_NAME_ZA: BarStationSorting_t = 5;
pub const BAR_SORT_QUICKMIX_10_NAME_AZ: BarStationSorting_t = 4;
pub const BAR_SORT_QUICKMIX_01_NAME_ZA: BarStationSorting_t = 3;
pub const BAR_SORT_QUICKMIX_01_NAME_AZ: BarStationSorting_t = 2;
pub const BAR_SORT_NAME_ZA: BarStationSorting_t = 1;
pub const BAR_SORT_NAME_AZ: BarStationSorting_t = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct BarMsgFormatStr_t {
    pub prefix: *mut libc::c_char,
    pub postfix: *mut libc::c_char,
}
pub type BarUiMsg_t = libc::c_uint;
pub const MSG_COUNT: BarUiMsg_t = 7;
pub const MSG_LIST: BarUiMsg_t = 6;
pub const MSG_QUESTION: BarUiMsg_t = 5;
pub const MSG_ERR: BarUiMsg_t = 4;
pub const MSG_TIME: BarUiMsg_t = 3;
pub const MSG_PLAYING: BarUiMsg_t = 2;
pub const MSG_INFO: BarUiMsg_t = 1;
pub const MSG_NONE: BarUiMsg_t = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct BarSettings_t {
    pub autoselect: bool,
    pub history: libc::c_uint,
    pub maxRetry: libc::c_uint,
    pub timeout: libc::c_uint,
    pub bufferSecs: libc::c_uint,
    pub volume: libc::c_int,
    pub gainMul: libc::c_float,
    pub sortOrder: BarStationSorting_t,
    pub audioQuality: PianoAudioQuality_t,
    pub username: *mut libc::c_char,
    pub password: *mut libc::c_char,
    pub passwordCmd: *mut libc::c_char,
    pub controlProxy: *mut libc::c_char,
    pub proxy: *mut libc::c_char,
    pub bindTo: *mut libc::c_char,
    pub autostartStation: *mut libc::c_char,
    pub eventCmd: *mut libc::c_char,
    pub loveIcon: *mut libc::c_char,
    pub banIcon: *mut libc::c_char,
    pub tiredIcon: *mut libc::c_char,
    pub atIcon: *mut libc::c_char,
    pub npSongFormat: *mut libc::c_char,
    pub npStationFormat: *mut libc::c_char,
    pub listSongFormat: *mut libc::c_char,
    pub timeFormat: *mut libc::c_char,
    pub fifo: *mut libc::c_char,
    pub rpcHost: *mut libc::c_char,
    pub rpcTlsPort: *mut libc::c_char,
    pub partnerUser: *mut libc::c_char,
    pub partnerPassword: *mut libc::c_char,
    pub device: *mut libc::c_char,
    pub inkey: *mut libc::c_char,
    pub outkey: *mut libc::c_char,
    pub caBundle: *mut libc::c_char,
    pub audioPipe: *mut libc::c_char,
    pub keys: [libc::c_char; 30],
    pub sampleRate: libc::c_int,
    pub msgFormat: [BarMsgFormatStr_t; 7],
}
pub type BarPlayerMode = libc::c_uint;
pub const PLAYER_FINISHED: BarPlayerMode = 3;
pub const PLAYER_PLAYING: BarPlayerMode = 2;
pub const PLAYER_WAITING: BarPlayerMode = 1;
pub const PLAYER_DEAD: BarPlayerMode = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct player_t {
    pub lock: pthread_mutex_t,
    pub aoplayLock: pthread_mutex_t,
    pub cond: pthread_cond_t,
    pub aoplayCond: pthread_cond_t,
    pub doQuit: bool,
    pub doPause: bool,
    pub songDuration: libc::c_uint,
    pub songPlayed: libc::c_uint,
    pub mode: BarPlayerMode,
    pub fvolume: *mut AVFilterContext,
    pub fgraph: *mut AVFilterGraph,
    pub fctx: *mut AVFormatContext,
    pub st: *mut AVStream,
    pub cctx: *mut AVCodecContext,
    pub fbufsink: *mut AVFilterContext,
    pub fabuf: *mut AVFilterContext,
    pub streamIdx: libc::c_int,
    pub lastTimestamp: int64_t,
    pub interrupted: sig_atomic_t,
    pub aoDev: *mut ao_device,
    pub gain: libc::c_double,
    pub url: *mut libc::c_char,
    pub settings: *const BarSettings_t,
}
pub type C2RustUnnamed_6 = libc::c_uint;
pub const PLAYER_RET_SOFTFAIL: C2RustUnnamed_6 = 2;
pub const PLAYER_RET_HARDFAIL: C2RustUnnamed_6 = 1;
pub const PLAYER_RET_OK: C2RustUnnamed_6 = 0;
pub type C2RustUnnamed_7 = libc::c_uint;
pub const DONE: C2RustUnnamed_7 = 2;
pub const DRAIN: C2RustUnnamed_7 = 1;
pub const FILL: C2RustUnnamed_7 = 0;
#[inline]
unsafe extern "C" fn stat(
    mut __path: *const libc::c_char,
    mut __statbuf: *mut stat,
) -> libc::c_int {
    return __xstat(1 as libc::c_int, __path, __statbuf);
}
#[inline]
unsafe extern "C" fn av_q2d(mut a: AVRational) -> libc::c_double {
    return a.num as libc::c_double / a.den as libc::c_double;
}
pub static mut avformat: AVSampleFormat = AV_SAMPLE_FMT_S16;
unsafe extern "C" fn printError(
    settings: *const BarSettings_t,
    msg: *const libc::c_char,
    mut ret: libc::c_int,
) {
    let mut avmsg: [libc::c_char; 128] = [0; 128];
    av_strerror(
        ret,
        avmsg.as_mut_ptr(),
        ::std::mem::size_of::<[libc::c_char; 128]>() as libc::c_ulong,
    );
    BarUiMsg(
        settings,
        MSG_ERR,
        b"%s (%s)\n\0" as *const u8 as *const libc::c_char,
        msg,
        avmsg.as_mut_ptr(),
    );
}
pub unsafe extern "C" fn BarPlayerInit(
    p: *mut player_t,
    settings: *const BarSettings_t,
) {
    ao_initialize();
    av_log_set_level(8 as libc::c_int);
    pthread_mutex_init(&mut (*p).lock, 0 as *const pthread_mutexattr_t);
    pthread_cond_init(&mut (*p).cond, 0 as *const pthread_condattr_t);
    pthread_mutex_init(&mut (*p).aoplayLock, 0 as *const pthread_mutexattr_t);
    pthread_cond_init(&mut (*p).aoplayCond, 0 as *const pthread_condattr_t);
    BarPlayerReset(p);
    (*p).settings = settings;
}
pub unsafe extern "C" fn BarPlayerDestroy(p: *mut player_t) {
    pthread_cond_destroy(&mut (*p).cond);
    pthread_mutex_destroy(&mut (*p).lock);
    pthread_cond_destroy(&mut (*p).aoplayCond);
    pthread_mutex_destroy(&mut (*p).aoplayLock);
    ao_shutdown();
}
pub unsafe extern "C" fn BarPlayerReset(p: *mut player_t) {
    (*p).doQuit = 0 as libc::c_int != 0;
    (*p).doPause = 0 as libc::c_int != 0;
    (*p).songDuration = 0 as libc::c_int as libc::c_uint;
    (*p).songPlayed = 0 as libc::c_int as libc::c_uint;
    (*p).mode = PLAYER_DEAD;
    (*p).fvolume = 0 as *mut AVFilterContext;
    (*p).fgraph = 0 as *mut AVFilterGraph;
    (*p).fctx = 0 as *mut AVFormatContext;
    (*p).st = 0 as *mut AVStream;
    (*p).cctx = 0 as *mut AVCodecContext;
    (*p).fbufsink = 0 as *mut AVFilterContext;
    (*p).fabuf = 0 as *mut AVFilterContext;
    (*p).streamIdx = -(1 as libc::c_int);
    (*p).lastTimestamp = 0 as libc::c_int as int64_t;
    (*p).interrupted = 0 as libc::c_int;
    (*p).aoDev = 0 as *mut ao_device;
}
pub unsafe extern "C" fn BarPlayerSetVolume(player: *mut player_t) {
    if (*player).mode as libc::c_uint != PLAYER_PLAYING as libc::c_int as libc::c_uint {
        return;
    }
    let mut ret: libc::c_int = 0;
    let mut strbuf: [libc::c_char; 16] = [0; 16];
    snprintf(
        strbuf.as_mut_ptr(),
        ::std::mem::size_of::<[libc::c_char; 16]>() as libc::c_ulong,
        b"%fdB\0" as *const u8 as *const libc::c_char,
        (*(*player).settings).volume as libc::c_double
            + (*player).gain * (*(*player).settings).gainMul as libc::c_double,
    );
    ret = avfilter_graph_send_command(
        (*player).fgraph,
        b"volume\0" as *const u8 as *const libc::c_char,
        b"volume\0" as *const u8 as *const libc::c_char,
        strbuf.as_mut_ptr(),
        0 as *mut libc::c_char,
        0 as libc::c_int,
        0 as libc::c_int,
    );
    if ret < 0 as libc::c_int {
        printError(
            (*player).settings,
            b"Cannot set volume\0" as *const u8 as *const libc::c_char,
            ret,
        );
    }
}
unsafe extern "C" fn intCb(data: *mut libc::c_void) -> libc::c_int {
    let player: *mut player_t = data as *mut player_t;
    if (*player).interrupted > 1 as libc::c_int {
        pthread_mutex_lock(&mut (*player).lock);
        (*player).doQuit = 1 as libc::c_int != 0;
        pthread_mutex_unlock(&mut (*player).lock);
        return 1 as libc::c_int;
    } else if (*player).interrupted != 0 as libc::c_int {
        (*player).interrupted = 0 as libc::c_int;
        return 1 as libc::c_int;
    } else {
        return 0 as libc::c_int
    };
}
unsafe extern "C" fn openStream(player: *mut player_t) -> bool {
    let mut ret: libc::c_int = 0;
    (*player).fctx = avformat_alloc_context();
    (*(*player).fctx)
        .interrupt_callback
        .callback = Some(
        intCb as unsafe extern "C" fn(*mut libc::c_void) -> libc::c_int,
    );
    (*(*player).fctx).interrupt_callback.opaque = player as *mut libc::c_void;
    let mut timeout: libc::c_ulong = ((*(*player).settings).timeout)
        .wrapping_mul(1000000 as libc::c_int as libc::c_uint) as libc::c_ulong;
    let mut timeoutStr: [libc::c_char; 16] = [0; 16];
    ret = snprintf(
        timeoutStr.as_mut_ptr(),
        ::std::mem::size_of::<[libc::c_char; 16]>() as libc::c_ulong,
        b"%lu\0" as *const u8 as *const libc::c_char,
        timeout,
    );
    let mut options: *mut AVDictionary = 0 as *mut AVDictionary;
    av_dict_set(
        &mut options,
        b"timeout\0" as *const u8 as *const libc::c_char,
        timeoutStr.as_mut_ptr(),
        0 as libc::c_int,
    );
    ret = avformat_open_input(
        &mut (*player).fctx,
        (*player).url,
        0 as *mut AVInputFormat,
        &mut options,
    );
    if ret < 0 as libc::c_int {
        printError(
            (*player).settings,
            b"Unable to open audio file\0" as *const u8 as *const libc::c_char,
            ret,
        );
        return 0 as libc::c_int != 0;
    }
    ret = avformat_find_stream_info((*player).fctx, 0 as *mut *mut AVDictionary);
    if ret < 0 as libc::c_int {
        printError(
            (*player).settings,
            b"find_stream_info\0" as *const u8 as *const libc::c_char,
            ret,
        );
        return 0 as libc::c_int != 0;
    }
    let mut i: size_t = 0 as libc::c_int as size_t;
    while i < (*(*player).fctx).nb_streams as libc::c_ulong {
        (**((*(*player).fctx).streams).offset(i as isize)).discard = AVDISCARD_ALL;
        i = i.wrapping_add(1);
        i;
    }
    (*player)
        .streamIdx = av_find_best_stream(
        (*player).fctx,
        AVMEDIA_TYPE_AUDIO,
        -(1 as libc::c_int),
        -(1 as libc::c_int),
        0 as *mut *mut AVCodec,
        0 as libc::c_int,
    );
    if (*player).streamIdx < 0 as libc::c_int {
        printError(
            (*player).settings,
            b"find_best_stream\0" as *const u8 as *const libc::c_char,
            ret,
        );
        return 0 as libc::c_int != 0;
    }
    (*player).st = *((*(*player).fctx).streams).offset((*player).streamIdx as isize);
    (*(*player).st).discard = AVDISCARD_DEFAULT;
    (*player).cctx = avcodec_alloc_context3(0 as *const AVCodec);
    if ((*player).cctx).is_null() {
        printError(
            (*player).settings,
            b"avcodec_alloc_context3\0" as *const u8 as *const libc::c_char,
            ret,
        );
        return 0 as libc::c_int != 0;
    }
    let cp: *const AVCodecParameters = (*(*player).st).codecpar;
    ret = avcodec_parameters_to_context((*player).cctx, cp);
    if ret < 0 as libc::c_int {
        printError(
            (*player).settings,
            b"avcodec_parameters_to_context\0" as *const u8 as *const libc::c_char,
            ret,
        );
        return 0 as libc::c_int != 0;
    }
    let decoder: *mut AVCodec = avcodec_find_decoder((*cp).codec_id);
    if decoder.is_null() {
        printError(
            (*player).settings,
            b"find_decoder\0" as *const u8 as *const libc::c_char,
            ret,
        );
        return 0 as libc::c_int != 0;
    }
    ret = avcodec_open2((*player).cctx, decoder, 0 as *mut *mut AVDictionary);
    if ret < 0 as libc::c_int {
        printError(
            (*player).settings,
            b"codec_open2\0" as *const u8 as *const libc::c_char,
            ret,
        );
        return 0 as libc::c_int != 0;
    }
    if (*player).lastTimestamp > 0 as libc::c_int as libc::c_long {
        av_seek_frame(
            (*player).fctx,
            (*player).streamIdx,
            (*player).lastTimestamp,
            0 as libc::c_int,
        );
    }
    let songDuration: libc::c_uint = (av_q2d((*(*player).st).time_base)
        * (*(*player).st).duration as libc::c_double) as libc::c_uint;
    pthread_mutex_lock(&mut (*player).lock);
    (*player).songPlayed = 0 as libc::c_int as libc::c_uint;
    (*player).songDuration = songDuration;
    pthread_mutex_unlock(&mut (*player).lock);
    return 1 as libc::c_int != 0;
}
unsafe extern "C" fn getSampleRate(player: *const player_t) -> libc::c_int {
    let cp: *const AVCodecParameters = (*(*player).st).codecpar;
    return if (*(*player).settings).sampleRate == 0 as libc::c_int {
        (*cp).sample_rate
    } else {
        (*(*player).settings).sampleRate
    };
}
unsafe extern "C" fn openFilter(player: *mut player_t) -> bool {
    let mut strbuf: [libc::c_char; 256] = [0; 256];
    let mut ret: libc::c_int = 0 as libc::c_int;
    let cp: *mut AVCodecParameters = (*(*player).st).codecpar;
    (*player).fgraph = avfilter_graph_alloc();
    if ((*player).fgraph).is_null() {
        printError(
            (*player).settings,
            b"graph_alloc\0" as *const u8 as *const libc::c_char,
            ret,
        );
        return 0 as libc::c_int != 0;
    }
    let mut time_base: AVRational = (*(*player).st).time_base;
    snprintf(
        strbuf.as_mut_ptr(),
        ::std::mem::size_of::<[libc::c_char; 256]>() as libc::c_ulong,
        b"time_base=%d/%d:sample_rate=%d:sample_fmt=%s:channel_layout=0x%lx\0"
            as *const u8 as *const libc::c_char,
        time_base.num,
        time_base.den,
        (*cp).sample_rate,
        av_get_sample_fmt_name((*(*player).cctx).sample_fmt),
        (*cp).channel_layout,
    );
    ret = avfilter_graph_create_filter(
        &mut (*player).fabuf,
        avfilter_get_by_name(b"abuffer\0" as *const u8 as *const libc::c_char),
        b"source\0" as *const u8 as *const libc::c_char,
        strbuf.as_mut_ptr(),
        0 as *mut libc::c_void,
        (*player).fgraph,
    );
    if ret < 0 as libc::c_int {
        printError(
            (*player).settings,
            b"create_filter abuffer\0" as *const u8 as *const libc::c_char,
            ret,
        );
        return 0 as libc::c_int != 0;
    }
    ret = avfilter_graph_create_filter(
        &mut (*player).fvolume,
        avfilter_get_by_name(b"volume\0" as *const u8 as *const libc::c_char),
        b"volume\0" as *const u8 as *const libc::c_char,
        b"0dB\0" as *const u8 as *const libc::c_char,
        0 as *mut libc::c_void,
        (*player).fgraph,
    );
    if ret < 0 as libc::c_int {
        printError(
            (*player).settings,
            b"create_filter volume\0" as *const u8 as *const libc::c_char,
            ret,
        );
        return 0 as libc::c_int != 0;
    }
    let mut fafmt: *mut AVFilterContext = 0 as *mut AVFilterContext;
    snprintf(
        strbuf.as_mut_ptr(),
        ::std::mem::size_of::<[libc::c_char; 256]>() as libc::c_ulong,
        b"sample_fmts=%s:sample_rates=%d\0" as *const u8 as *const libc::c_char,
        av_get_sample_fmt_name(avformat),
        getSampleRate(player),
    );
    ret = avfilter_graph_create_filter(
        &mut fafmt,
        avfilter_get_by_name(b"aformat\0" as *const u8 as *const libc::c_char),
        b"format\0" as *const u8 as *const libc::c_char,
        strbuf.as_mut_ptr(),
        0 as *mut libc::c_void,
        (*player).fgraph,
    );
    if ret < 0 as libc::c_int {
        printError(
            (*player).settings,
            b"create_filter aformat\0" as *const u8 as *const libc::c_char,
            ret,
        );
        return 0 as libc::c_int != 0;
    }
    ret = avfilter_graph_create_filter(
        &mut (*player).fbufsink,
        avfilter_get_by_name(b"abuffersink\0" as *const u8 as *const libc::c_char),
        b"sink\0" as *const u8 as *const libc::c_char,
        0 as *const libc::c_char,
        0 as *mut libc::c_void,
        (*player).fgraph,
    );
    if ret < 0 as libc::c_int {
        printError(
            (*player).settings,
            b"create_filter abuffersink\0" as *const u8 as *const libc::c_char,
            ret,
        );
        return 0 as libc::c_int != 0;
    }
    if avfilter_link(
        (*player).fabuf,
        0 as libc::c_int as libc::c_uint,
        (*player).fvolume,
        0 as libc::c_int as libc::c_uint,
    ) != 0 as libc::c_int
        || avfilter_link(
            (*player).fvolume,
            0 as libc::c_int as libc::c_uint,
            fafmt,
            0 as libc::c_int as libc::c_uint,
        ) != 0 as libc::c_int
        || avfilter_link(
            fafmt,
            0 as libc::c_int as libc::c_uint,
            (*player).fbufsink,
            0 as libc::c_int as libc::c_uint,
        ) != 0 as libc::c_int
    {
        printError(
            (*player).settings,
            b"filter_link\0" as *const u8 as *const libc::c_char,
            ret,
        );
        return 0 as libc::c_int != 0;
    }
    ret = avfilter_graph_config((*player).fgraph, 0 as *mut libc::c_void);
    if ret < 0 as libc::c_int {
        printError(
            (*player).settings,
            b"graph_config\0" as *const u8 as *const libc::c_char,
            ret,
        );
        return 0 as libc::c_int != 0;
    }
    return 1 as libc::c_int != 0;
}
unsafe extern "C" fn openDevice(player: *mut player_t) -> bool {
    let cp: *const AVCodecParameters = (*(*player).st).codecpar;
    let mut aoFmt: ao_sample_format = ao_sample_format {
        bits: 0,
        rate: 0,
        channels: 0,
        byte_format: 0,
        matrix: 0 as *mut libc::c_char,
    };
    memset(
        &mut aoFmt as *mut ao_sample_format as *mut libc::c_void,
        0 as libc::c_int,
        ::std::mem::size_of::<ao_sample_format>() as libc::c_ulong,
    );
    aoFmt.bits = av_get_bytes_per_sample(avformat) * 8 as libc::c_int;
    aoFmt.channels = (*cp).channels;
    aoFmt.rate = getSampleRate(player);
    aoFmt.byte_format = 4 as libc::c_int;
    let mut driver: libc::c_int = -(1 as libc::c_int);
    if !((*(*player).settings).audioPipe).is_null() {
        let mut st: stat = stat {
            st_dev: 0,
            st_ino: 0,
            st_nlink: 0,
            st_mode: 0,
            st_uid: 0,
            st_gid: 0,
            __pad0: 0,
            st_rdev: 0,
            st_size: 0,
            st_blksize: 0,
            st_blocks: 0,
            st_atim: timespec { tv_sec: 0, tv_nsec: 0 },
            st_mtim: timespec { tv_sec: 0, tv_nsec: 0 },
            st_ctim: timespec { tv_sec: 0, tv_nsec: 0 },
            __glibc_reserved: [0; 3],
        };
        if stat((*(*player).settings).audioPipe, &mut st) != 0 {
            BarUiMsg(
                (*player).settings,
                MSG_ERR,
                b"Cannot stat audio pipe file.\n\0" as *const u8 as *const libc::c_char,
            );
            return 0 as libc::c_int != 0;
        }
        if !(st.st_mode & 0o170000 as libc::c_int as libc::c_uint
            == 0o10000 as libc::c_int as libc::c_uint)
        {
            BarUiMsg(
                (*player).settings,
                MSG_ERR,
                b"File is not a pipe, error.\n\0" as *const u8 as *const libc::c_char,
            );
            return 0 as libc::c_int != 0;
        }
        driver = ao_driver_id(b"raw\0" as *const u8 as *const libc::c_char);
        (*player)
            .aoDev = ao_open_file(
            driver,
            (*(*player).settings).audioPipe,
            1 as libc::c_int,
            &mut aoFmt,
            0 as *mut ao_option,
        );
        if ((*player).aoDev).is_null() {
            BarUiMsg(
                (*player).settings,
                MSG_ERR,
                b"Cannot open audio pipe file.\n\0" as *const u8 as *const libc::c_char,
            );
            return 0 as libc::c_int != 0;
        }
    } else {
        driver = ao_default_driver_id();
        (*player).aoDev = ao_open_live(driver, &mut aoFmt, 0 as *mut ao_option);
        if ((*player).aoDev).is_null() {
            BarUiMsg(
                (*player).settings,
                MSG_ERR,
                b"Cannot open audio device.\n\0" as *const u8 as *const libc::c_char,
            );
            return 0 as libc::c_int != 0;
        }
    }
    return 1 as libc::c_int != 0;
}
unsafe extern "C" fn shouldQuit(player: *mut player_t) -> bool {
    pthread_mutex_lock(&mut (*player).lock);
    let ret: bool = (*player).doQuit;
    pthread_mutex_unlock(&mut (*player).lock);
    return ret;
}
unsafe extern "C" fn changeMode(player: *mut player_t, mut mode: libc::c_uint) {
    pthread_mutex_lock(&mut (*player).lock);
    (*player).mode = mode as BarPlayerMode;
    pthread_mutex_unlock(&mut (*player).lock);
}
pub unsafe extern "C" fn BarPlayerGetMode(player: *mut player_t) -> BarPlayerMode {
    pthread_mutex_lock(&mut (*player).lock);
    let ret: BarPlayerMode = (*player).mode;
    pthread_mutex_unlock(&mut (*player).lock);
    return ret;
}
unsafe extern "C" fn play(player: *mut player_t) -> libc::c_int {
    let minBufferHealth: int64_t = (*(*player).settings).bufferSecs as int64_t;
    let cctx: *mut AVCodecContext = (*player).cctx;
    let mut pkt: *mut AVPacket = av_packet_alloc();
    (*pkt).data = 0 as *mut uint8_t;
    (*pkt).size = 0 as libc::c_int;
    let mut frame: *mut AVFrame = 0 as *mut AVFrame;
    frame = av_frame_alloc();
    let mut aoplaythread: pthread_t = 0;
    pthread_create(
        &mut aoplaythread,
        0 as *const pthread_attr_t,
        Some(
            BarAoPlayThread
                as unsafe extern "C" fn(*mut libc::c_void) -> *mut libc::c_void,
        ),
        player as *mut libc::c_void,
    );
    let mut drainMode: C2RustUnnamed_7 = FILL;
    let mut ret: libc::c_int = 0 as libc::c_int;
    let timeBase: libc::c_double = av_q2d((*(*player).st).time_base);
    while !shouldQuit(player)
        && drainMode as libc::c_uint != DONE as libc::c_int as libc::c_uint
    {
        if drainMode as libc::c_uint == FILL as libc::c_int as libc::c_uint {
            ret = av_read_frame((*player).fctx, pkt);
            if ret
                == -((('E' as i32 | ('O' as i32) << 8 as libc::c_int
                    | ('F' as i32) << 16 as libc::c_int) as libc::c_uint
                    | (' ' as i32 as libc::c_uint) << 24 as libc::c_int) as libc::c_int)
            {
                drainMode = DRAIN;
                avcodec_send_packet(cctx, 0 as *const AVPacket);
            } else if (*pkt).stream_index != (*player).streamIdx {
                av_packet_unref(pkt);
                continue;
            } else if ret < 0 as libc::c_int {
                pthread_mutex_lock(&mut (*player).aoplayLock);
                let rt: libc::c_int = av_buffersrc_add_frame(
                    (*player).fabuf,
                    0 as *mut AVFrame,
                );
                pthread_cond_broadcast(&mut (*player).aoplayCond);
                pthread_mutex_unlock(&mut (*player).aoplayLock);
                break;
            } else {
                avcodec_send_packet(cctx, pkt);
            }
        }
        while !shouldQuit(player) {
            ret = avcodec_receive_frame(cctx, frame);
            if ret
                == -((('E' as i32 | ('O' as i32) << 8 as libc::c_int
                    | ('F' as i32) << 16 as libc::c_int) as libc::c_uint
                    | (' ' as i32 as libc::c_uint) << 24 as libc::c_int) as libc::c_int)
            {
                drainMode = DONE;
                pthread_mutex_lock(&mut (*player).aoplayLock);
                let rt_0: libc::c_int = av_buffersrc_add_frame(
                    (*player).fabuf,
                    0 as *mut AVFrame,
                );
                pthread_cond_broadcast(&mut (*player).aoplayCond);
                pthread_mutex_unlock(&mut (*player).aoplayLock);
                break;
            } else {
                if ret != 0 as libc::c_int {
                    break;
                }
                if (*frame).pts == 0x8000000000000000 as libc::c_ulong as int64_t {
                    (*frame).pts = 0 as libc::c_int as int64_t;
                }
                pthread_mutex_lock(&mut (*player).aoplayLock);
                ret = av_buffersrc_write_frame((*player).fabuf, frame);
                pthread_mutex_unlock(&mut (*player).aoplayLock);
                let mut bufferHealth: int64_t = 0 as libc::c_int as int64_t;
                loop {
                    pthread_mutex_lock(&mut (*player).aoplayLock);
                    bufferHealth = (timeBase
                        * ((*frame).pts - (*player).lastTimestamp) as libc::c_double)
                        as int64_t;
                    if bufferHealth > minBufferHealth {
                        pthread_cond_broadcast(&mut (*player).aoplayCond);
                        pthread_cond_wait(
                            &mut (*player).aoplayCond,
                            &mut (*player).aoplayLock,
                        );
                    }
                    pthread_mutex_unlock(&mut (*player).aoplayLock);
                    if !(bufferHealth > minBufferHealth) {
                        break;
                    }
                }
            }
        }
        av_packet_unref(pkt);
    }
    av_frame_free(&mut frame);
    av_packet_free(&mut pkt);
    pthread_join(aoplaythread, 0 as *mut *mut libc::c_void);
    return ret;
}
unsafe extern "C" fn finish(player: *mut player_t) {
    ao_close((*player).aoDev);
    (*player).aoDev = 0 as *mut ao_device;
    if !((*player).fgraph).is_null() {
        avfilter_graph_free(&mut (*player).fgraph);
        (*player).fgraph = 0 as *mut AVFilterGraph;
    }
    if !((*player).cctx).is_null() {
        avcodec_close((*player).cctx);
        (*player).cctx = 0 as *mut AVCodecContext;
    }
    if !((*player).fctx).is_null() {
        avformat_close_input(&mut (*player).fctx);
    }
}
pub unsafe extern "C" fn BarPlayerThread(
    mut data: *mut libc::c_void,
) -> *mut libc::c_void {
    let player: *mut player_t = data as *mut player_t;
    let mut pret: uintptr_t = PLAYER_RET_OK as libc::c_int as uintptr_t;
    let mut retry: bool = false;
    loop {
        retry = 0 as libc::c_int != 0;
        if openStream(player) {
            if openFilter(player) as libc::c_int != 0
                && openDevice(player) as libc::c_int != 0
            {
                changeMode(player, PLAYER_PLAYING as libc::c_int as libc::c_uint);
                BarPlayerSetVolume(player);
                retry = play(player)
                    == -((('I' as i32 | ('N' as i32) << 8 as libc::c_int
                        | ('D' as i32) << 16 as libc::c_int) as libc::c_uint
                        | ('A' as i32 as libc::c_uint) << 24 as libc::c_int)
                        as libc::c_int) && (*player).interrupted == 0;
            } else {
                pret = PLAYER_RET_HARDFAIL as libc::c_int as uintptr_t;
            }
        } else {
            pret = PLAYER_RET_SOFTFAIL as libc::c_int as uintptr_t;
        }
        changeMode(player, PLAYER_WAITING as libc::c_int as libc::c_uint);
        finish(player);
        if !retry {
            break;
        }
    }
    changeMode(player, PLAYER_FINISHED as libc::c_int as libc::c_uint);
    return pret as *mut libc::c_void;
}
pub unsafe extern "C" fn BarAoPlayThread(
    mut data: *mut libc::c_void,
) -> *mut libc::c_void {
    let player: *mut player_t = data as *mut player_t;
    let mut filteredFrame: *mut AVFrame = 0 as *mut AVFrame;
    filteredFrame = av_frame_alloc();
    let mut ret: libc::c_int = 0;
    let timeBase: libc::c_double = av_q2d(
        av_buffersink_get_time_base((*player).fbufsink),
    );
    let timeBaseSt: libc::c_double = av_q2d((*(*player).st).time_base);
    while !shouldQuit(player) {
        pthread_mutex_lock(&mut (*player).aoplayLock);
        ret = av_buffersink_get_frame((*player).fbufsink, filteredFrame);
        if ret
            == -((('E' as i32 | ('O' as i32) << 8 as libc::c_int
                | ('F' as i32) << 16 as libc::c_int) as libc::c_uint
                | (' ' as i32 as libc::c_uint) << 24 as libc::c_int) as libc::c_int)
            || shouldQuit(player) as libc::c_int != 0
        {
            pthread_mutex_unlock(&mut (*player).aoplayLock);
            break;
        } else if ret < 0 as libc::c_int {
            pthread_cond_broadcast(&mut (*player).aoplayCond);
            pthread_cond_wait(&mut (*player).aoplayCond, &mut (*player).aoplayLock);
            pthread_mutex_unlock(&mut (*player).aoplayLock);
        } else {
            pthread_mutex_unlock(&mut (*player).aoplayLock);
            let numChannels: libc::c_int = av_get_channel_layout_nb_channels(
                (*filteredFrame).channel_layout,
            );
            let bps: libc::c_int = av_get_bytes_per_sample(
                (*filteredFrame).format as AVSampleFormat,
            );
            ao_play(
                (*player).aoDev,
                (*filteredFrame).data[0 as libc::c_int as usize] as *mut libc::c_char,
                ((*filteredFrame).nb_samples * numChannels * bps) as uint_32,
            );
            let timestamp: libc::c_double = (*filteredFrame).pts as libc::c_double
                * timeBase;
            let songPlayed: libc::c_uint = timestamp as libc::c_uint;
            pthread_mutex_lock(&mut (*player).lock);
            (*player).songPlayed = songPlayed;
            if (*player).doPause {
                loop {
                    pthread_cond_wait(&mut (*player).cond, &mut (*player).lock);
                    if !(*player).doPause {
                        break;
                    }
                }
            }
            pthread_mutex_unlock(&mut (*player).lock);
            let lastTimestamp: int64_t = (timestamp / timeBaseSt) as int64_t;
            pthread_mutex_lock(&mut (*player).aoplayLock);
            (*player).lastTimestamp = lastTimestamp;
            pthread_cond_broadcast(&mut (*player).aoplayCond);
            pthread_mutex_unlock(&mut (*player).aoplayLock);
            av_frame_unref(filteredFrame);
        }
    }
    av_frame_free(&mut filteredFrame);
    return 0 as *mut libc::c_void;
}
