use ::libc;
extern "C" {
    pub type _IO_wide_data;
    pub type _IO_codecvt;
    pub type _IO_marker;
    pub type afm_handle_st;
    pub type afm_font_private_data_st;
    pub type stringhash_st;
    fn strtod(_: *const libc::c_char, _: *mut *mut libc::c_char) -> libc::c_double;
    fn strtol(
        _: *const libc::c_char,
        _: *mut *mut libc::c_char,
        _: libc::c_int,
    ) -> libc::c_long;
    fn malloc(_: libc::c_ulong) -> *mut libc::c_void;
    fn free(__ptr: *mut libc::c_void);
    fn exit(_: libc::c_int) -> !;
    fn getenv(__name: *const libc::c_char) -> *mut libc::c_char;
    fn memcpy(
        _: *mut libc::c_void,
        _: *const libc::c_void,
        _: libc::c_ulong,
    ) -> *mut libc::c_void;
    fn strcpy(_: *mut libc::c_char, _: *const libc::c_char) -> *mut libc::c_char;
    fn strncpy(
        _: *mut libc::c_char,
        _: *const libc::c_char,
        _: libc::c_ulong,
    ) -> *mut libc::c_char;
    fn strcmp(_: *const libc::c_char, _: *const libc::c_char) -> libc::c_int;
    fn strncmp(
        _: *const libc::c_char,
        _: *const libc::c_char,
        _: libc::c_ulong,
    ) -> libc::c_int;
    fn strchr(_: *const libc::c_char, _: libc::c_int) -> *mut libc::c_char;
    fn strrchr(_: *const libc::c_char, _: libc::c_int) -> *mut libc::c_char;
    fn strtok(_: *mut libc::c_char, _: *const libc::c_char) -> *mut libc::c_char;
    fn strlen(_: *const libc::c_char) -> libc::c_ulong;
    fn strerror(_: libc::c_int) -> *mut libc::c_char;
    fn getcwd(__buf: *mut libc::c_char, __size: size_t) -> *mut libc::c_char;
    fn gethostname(__name: *mut libc::c_char, __len: size_t) -> libc::c_int;
    fn fopen(_: *const libc::c_char, _: *const libc::c_char) -> *mut FILE;
    fn fflush(__stream: *mut FILE) -> libc::c_int;
    fn fclose(__stream: *mut FILE) -> libc::c_int;
    fn fwrite(
        _: *const libc::c_void,
        _: libc::c_ulong,
        _: libc::c_ulong,
        _: *mut FILE,
    ) -> libc::c_ulong;
    fn fread(
        _: *mut libc::c_void,
        _: libc::c_ulong,
        _: libc::c_ulong,
        _: *mut FILE,
    ) -> libc::c_ulong;
    fn ungetc(__c: libc::c_int, __stream: *mut FILE) -> libc::c_int;
    fn fputs(__s: *const libc::c_char, __stream: *mut FILE) -> libc::c_int;
    static mut stderr: *mut FILE;
    fn fgets(
        __s: *mut libc::c_char,
        __n: libc::c_int,
        __stream: *mut FILE,
    ) -> *mut libc::c_char;
    fn fputc(__c: libc::c_int, __stream: *mut FILE) -> libc::c_int;
    fn __xstat(
        __ver: libc::c_int,
        __filename: *const libc::c_char,
        __stat_buf: *mut stat,
    ) -> libc::c_int;
    fn fgetc(__stream: *mut FILE) -> libc::c_int;
    fn sprintf(_: *mut libc::c_char, _: *const libc::c_char, _: ...) -> libc::c_int;
    fn fprintf(_: *mut FILE, _: *const libc::c_char, _: ...) -> libc::c_int;
    fn pclose(__stream: *mut FILE) -> libc::c_int;
    fn popen(__command: *const libc::c_char, __modes: *const libc::c_char) -> *mut FILE;
    fn __errno_location() -> *mut libc::c_int;
    fn strftime(
        __s: *mut libc::c_char,
        __maxsize: size_t,
        __format: *const libc::c_char,
        __tp: *const tm,
    ) -> size_t;
    fn __ctype_b_loc() -> *mut *const libc::c_ushort;
    fn getpwnam(__name: *const libc::c_char) -> *mut passwd;
    fn dcgettext(
        __domainname: *const libc::c_char,
        __msgid: *const libc::c_char,
        __category: libc::c_int,
    ) -> *mut libc::c_char;
    fn afm_error_to_string(error: AFMError, buf: *mut libc::c_char);
    fn afm_font_prefix(
        handle: AFMHandle,
        fontname: *const libc::c_char,
        prefix_return: *mut *const libc::c_char,
    ) -> AFMError;
    fn afm_open_font(
        handle: AFMHandle,
        info_level: libc::c_uint,
        name: *const libc::c_char,
        font_return: *mut AFMFont,
    ) -> AFMError;
    fn afm_open_default_font(handle: AFMHandle, font_return: *mut AFMFont) -> AFMError;
    fn afm_close_font(font: AFMFont) -> AFMError;
    fn afm_font_charwidth(
        font: AFMFont,
        ptsize: AFMNumber,
        ch: libc::c_char,
        w0x_return: *mut AFMNumber,
        w0y_return: *mut AFMNumber,
    ) -> AFMError;
    fn afm_font_encode(
        font: AFMFont,
        code: libc::c_uchar,
        name: *mut libc::c_char,
        flags: libc::c_uint,
    ) -> AFMError;
    fn afm_font_encoding(
        font: AFMFont,
        enc: AFMEncoding,
        flags: libc::c_uint,
    ) -> AFMError;
    fn strhash_put(
        hash: StringHashPtr,
        key: *mut libc::c_char,
        keylen: libc::c_int,
        data: *mut libc::c_void,
        old_data_return: *mut *mut libc::c_void,
    ) -> libc::c_int;
    fn strhash_get(
        hash: StringHashPtr,
        key: *const libc::c_char,
        keylen: libc::c_int,
        data_return: *mut *mut libc::c_void,
    ) -> libc::c_int;
    fn strhash_delete(
        hash: StringHashPtr,
        key: *const libc::c_char,
        keylen: libc::c_int,
        data_return: *mut *mut libc::c_void,
    ) -> libc::c_int;
    fn strhash_get_first(
        hash: StringHashPtr,
        key_return: *mut *mut libc::c_char,
        keylen_return: *mut libc::c_int,
        data_return: *mut *mut libc::c_void,
    ) -> libc::c_int;
    fn strhash_get_next(
        hash: StringHashPtr,
        key_return: *mut *mut libc::c_char,
        keylen_return: *mut libc::c_int,
        data_return: *mut *mut libc::c_void,
    ) -> libc::c_int;
    fn xmalloc(size: size_t) -> *mut libc::c_void;
    fn xcalloc(num: size_t, size: size_t) -> *mut libc::c_void;
    fn xrealloc(ptr: *mut libc::c_void, size: size_t) -> *mut libc::c_void;
    fn xfree(ptr: *mut libc::c_void);
    fn xstrdup(_: *mut libc::c_char) -> *mut libc::c_char;
    static mut program: *mut libc::c_char;
    static mut ofp: *mut FILE;
    static mut run_tm: tm;
    static mut mod_tm: tm;
    static mut passwd: *mut passwd;
    static mut libpath: *mut libc::c_char;
    static mut afm_path: *mut libc::c_char;
    static mut media_names: *mut MediaEntry;
    static mut no_job_header_switch: *mut libc::c_char;
    static mut output_first_line: *mut libc::c_char;
    static mut queue_param: *mut libc::c_char;
    static mut spooler_command: *mut libc::c_char;
    static mut current_pagenum: libc::c_uint;
    static mut input_filenum: libc::c_uint;
    static mut current_file_linenum: libc::c_uint;
    static mut fname: *mut libc::c_char;
    static mut total_pages: libc::c_int;
    static mut d_header_h: libc::c_int;
    static mut d_footer_h: libc::c_int;
    static mut res_fonts: StringHashPtr;
    static mut download_fonts: StringHashPtr;
    static mut pagedevice: StringHashPtr;
    static mut statusdict: StringHashPtr;
    static mut user_strings: StringHashPtr;
    static mut afm_cache: StringHashPtr;
    static mut afm_info_cache: StringHashPtr;
    static mut afm: AFMHandle;
    static mut Fname: *mut libc::c_char;
    static mut Fpt: FontPoint;
    static mut font_widths: [libc::c_double; 0];
    static mut font_ctype: [libc::c_char; 0];
    static mut font_is_fixed: libc::c_int;
    static mut font_bbox_lly: libc::c_double;
    static mut encodings: [EncodingRegistry; 0];
    static mut printer: *mut libc::c_char;
    static mut verbose: libc::c_int;
    static mut title: *mut libc::c_char;
    static mut quiet: libc::c_int;
    static mut fancy_header_default: *mut libc::c_char;
    static mut output_file: *mut libc::c_char;
    static mut encoding: InputEncoding;
    static mut media_name: *mut libc::c_char;
    static mut encoding_name: *mut libc::c_char;
    static mut escape_char: libc::c_int;
    static mut baselineskip: libc::c_double;
    static mut ul_ptsize: FontPoint;
    static mut ul_gray: libc::c_double;
    static mut ul_font: *mut libc::c_char;
    static mut underlay: *mut libc::c_char;
    static mut ul_position: *mut libc::c_char;
    static mut ul_angle: libc::c_double;
    static mut ul_style_str: *mut libc::c_char;
    static mut ul_position_p: libc::c_int;
    static mut ul_angle_p: libc::c_int;
    static mut page_label_format: *mut libc::c_char;
    static mut mark_wrapped_lines_style_name: *mut libc::c_char;
    static mut npf_name: *mut libc::c_char;
    static mut clean_7bit: libc::c_int;
    static mut append_ctrl_D: libc::c_int;
    static mut highlight_bars: libc::c_uint;
    static mut highlight_bar_gray: libc::c_double;
    static mut page_prefeed: libc::c_int;
    static mut accept_composites: libc::c_int;
    static mut formfeed_type: FormFeedType;
    static mut input_filter_stdin: *mut libc::c_char;
    static mut toc: libc::c_int;
    static mut toc_fmt_string: *mut libc::c_char;
    static mut slicing: libc::c_int;
    static mut slice: libc::c_uint;
    static mut states_binary: *mut libc::c_char;
    static mut states_color: libc::c_int;
    static mut states_config_file: *mut libc::c_char;
    static mut states_highlight_style: *mut libc::c_char;
    static mut states_path: *mut libc::c_char;
    static mut generate_PageSize: libc::c_int;
    static mut pslevel: libc::c_uint;
}
pub type size_t = libc::c_ulong;
pub type __dev_t = libc::c_ulong;
pub type __uid_t = libc::c_uint;
pub type __gid_t = libc::c_uint;
pub type __ino_t = libc::c_ulong;
pub type __mode_t = libc::c_uint;
pub type __nlink_t = libc::c_ulong;
pub type __off_t = libc::c_long;
pub type __off64_t = libc::c_long;
pub type __time_t = libc::c_long;
pub type __blksize_t = libc::c_long;
pub type __blkcnt_t = libc::c_long;
pub type __syscall_slong_t = libc::c_long;
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
pub struct tm {
    pub tm_sec: libc::c_int,
    pub tm_min: libc::c_int,
    pub tm_hour: libc::c_int,
    pub tm_mday: libc::c_int,
    pub tm_mon: libc::c_int,
    pub tm_year: libc::c_int,
    pub tm_wday: libc::c_int,
    pub tm_yday: libc::c_int,
    pub tm_isdst: libc::c_int,
    pub tm_gmtoff: libc::c_long,
    pub tm_zone: *const libc::c_char,
}
pub type C2RustUnnamed = libc::c_uint;
pub const _ISalnum: C2RustUnnamed = 8;
pub const _ISpunct: C2RustUnnamed = 4;
pub const _IScntrl: C2RustUnnamed = 2;
pub const _ISblank: C2RustUnnamed = 1;
pub const _ISgraph: C2RustUnnamed = 32768;
pub const _ISprint: C2RustUnnamed = 16384;
pub const _ISspace: C2RustUnnamed = 8192;
pub const _ISxdigit: C2RustUnnamed = 4096;
pub const _ISdigit: C2RustUnnamed = 2048;
pub const _ISalpha: C2RustUnnamed = 1024;
pub const _ISlower: C2RustUnnamed = 512;
pub const _ISupper: C2RustUnnamed = 256;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct passwd {
    pub pw_name: *mut libc::c_char,
    pub pw_passwd: *mut libc::c_char,
    pub pw_uid: __uid_t,
    pub pw_gid: __gid_t,
    pub pw_gecos: *mut libc::c_char,
    pub pw_dir: *mut libc::c_char,
    pub pw_shell: *mut libc::c_char,
}
pub type AFMString = *mut libc::c_char;
pub type AFMName = *mut libc::c_char;
pub type AFMNumber = libc::c_double;
pub type AFMInteger = libc::c_long;
pub type AFMBoolean = libc::c_uint;
pub const AFMTrue: AFMBoolean = 1;
pub const AFMFalse: AFMBoolean = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct afm_array_st {
    pub num_items: AFMNumber,
    pub items: *mut AFMNode,
}
pub type AFMNode = afm_node_st;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct afm_node_st {
    pub type_0: libc::c_int,
    pub u: C2RustUnnamed_0,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_0 {
    pub string: AFMString,
    pub name: AFMName,
    pub number: AFMNumber,
    pub integer: AFMInteger,
    pub array: AFMArray,
    pub boolean: AFMBoolean,
}
pub type AFMArray = *mut afm_array_st;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct global_font_information_st {
    pub FontName: AFMString,
    pub FullName: AFMString,
    pub FamilyName: AFMString,
    pub Weight: AFMString,
    pub FontBBox_llx: AFMNumber,
    pub FontBBox_lly: AFMNumber,
    pub FontBBox_urx: AFMNumber,
    pub FontBBox_ury: AFMNumber,
    pub Version: AFMString,
    pub Notice: AFMString,
    pub EncodingScheme: AFMString,
    pub MappingScheme: AFMInteger,
    pub EscChar: AFMInteger,
    pub CharacterSet: AFMString,
    pub Characters: AFMInteger,
    pub IsBaseFont: AFMBoolean,
    pub VVector_0: AFMNumber,
    pub VVector_1: AFMNumber,
    pub IsFixedV: AFMBoolean,
    pub CapHeight: AFMNumber,
    pub XHeight: AFMNumber,
    pub Ascender: AFMNumber,
    pub Descender: AFMNumber,
    pub BlendAxisTypes: AFMArray,
    pub BlendDesignPositions: AFMArray,
    pub BlendDesignMap: AFMArray,
    pub WeightVector: AFMArray,
}
pub type AFMGlobalFontInformation = global_font_information_st;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct writing_direction_metrics_st {
    pub is_valid: AFMBoolean,
    pub UnderlinePosition: AFMNumber,
    pub UnderlineThickness: AFMNumber,
    pub ItalicAngle: AFMNumber,
    pub CharWidth_x: AFMNumber,
    pub CharWidth_y: AFMNumber,
    pub IsFixedPitch: AFMBoolean,
}
pub type AFMWritingDirectionMetrics = writing_direction_metrics_st;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ligature_st {
    pub successor: AFMName,
    pub ligature: AFMName,
}
pub type AFMLigature = ligature_st;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct individual_character_metrics_st {
    pub character_code: AFMInteger,
    pub w0x: AFMNumber,
    pub w0y: AFMNumber,
    pub w1x: AFMNumber,
    pub w1y: AFMNumber,
    pub name: AFMName,
    pub vv_x: AFMNumber,
    pub vv_y: AFMNumber,
    pub llx: AFMNumber,
    pub lly: AFMNumber,
    pub urx: AFMNumber,
    pub ury: AFMNumber,
    pub num_ligatures: AFMNumber,
    pub ligatures: *mut AFMLigature,
}
pub type AFMIndividualCharacterMetrics = individual_character_metrics_st;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct track_kern_st {
    pub degree: AFMInteger,
    pub min_ptsize: AFMNumber,
    pub min_kern: AFMNumber,
    pub max_ptsize: AFMNumber,
    pub max_kern: AFMNumber,
}
pub type AFMTrackKern = track_kern_st;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct pair_wise_kerning_st {
    pub name1: AFMName,
    pub name2: AFMName,
    pub kx: AFMNumber,
    pub ky: AFMNumber,
}
pub type AFMPairWiseKerning = pair_wise_kerning_st;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct composite_component_st {
    pub name: AFMName,
    pub deltax: AFMNumber,
    pub deltay: AFMNumber,
}
pub type AFMCompositeComponent = composite_component_st;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct composite_st {
    pub name: AFMName,
    pub num_components: AFMInteger,
    pub components: *mut AFMCompositeComponent,
}
pub type AFMComposite = composite_st;
pub type AFMError = libc::c_uint;
pub type AFMHandle = *mut afm_handle_st;
pub type AFMEncoding = libc::c_uint;
pub const AFM_ENCODING_KOI8: AFMEncoding = 14;
pub const AFM_ENCODING_HP8: AFMEncoding = 13;
pub const AFM_ENCODING_VMS: AFMEncoding = 12;
pub const AFM_ENCODING_MAC: AFMEncoding = 11;
pub const AFM_ENCODING_ASCII: AFMEncoding = 10;
pub const AFM_ENCODING_IBMPC: AFMEncoding = 9;
pub const AFM_ENCODING_ISO_8859_10: AFMEncoding = 8;
pub const AFM_ENCODING_ISO_8859_9: AFMEncoding = 7;
pub const AFM_ENCODING_ISO_8859_7: AFMEncoding = 6;
pub const AFM_ENCODING_ISO_8859_5: AFMEncoding = 5;
pub const AFM_ENCODING_ISO_8859_4: AFMEncoding = 4;
pub const AFM_ENCODING_ISO_8859_3: AFMEncoding = 3;
pub const AFM_ENCODING_ISO_8859_2: AFMEncoding = 2;
pub const AFM_ENCODING_ISO_8859_1: AFMEncoding = 1;
pub const AFM_ENCODING_DEFAULT: AFMEncoding = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct afm_font_st {
    pub private: *mut afm_font_private_data_st,
    pub version: AFMNumber,
    pub info_level: libc::c_uint,
    pub encoding: [*mut AFMIndividualCharacterMetrics; 256],
    pub global_info: AFMGlobalFontInformation,
    pub writing_direction_metrics: [AFMWritingDirectionMetrics; 2],
    pub num_character_metrics: AFMInteger,
    pub character_metrics: *mut AFMIndividualCharacterMetrics,
    pub num_composites: AFMInteger,
    pub composites: *mut AFMComposite,
    pub num_kern_pairs: AFMInteger,
    pub kern_pairs: *mut AFMPairWiseKerning,
    pub num_track_kerns: AFMInteger,
    pub track_kerns: *mut AFMTrackKern,
}
pub type AFMFont = *mut afm_font_st;
pub type StringHashPtr = *mut stringhash_st;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct media_entry_st {
    pub next: *mut media_entry_st,
    pub name: *mut libc::c_char,
    pub w: libc::c_int,
    pub h: libc::c_int,
    pub llx: libc::c_int,
    pub lly: libc::c_int,
    pub urx: libc::c_int,
    pub ury: libc::c_int,
}
pub type MediaEntry = media_entry_st;
pub type InputEncoding = libc::c_uint;
pub const ENC_PS: InputEncoding = 16;
pub const ENC_KOI8: InputEncoding = 15;
pub const ENC_HP8: InputEncoding = 14;
pub const ENC_VMS: InputEncoding = 13;
pub const ENC_MAC: InputEncoding = 12;
pub const ENC_IBMPC: InputEncoding = 11;
pub const ENC_ASCII_DKNO: InputEncoding = 10;
pub const ENC_ASCII_FISE: InputEncoding = 9;
pub const ENC_ASCII: InputEncoding = 8;
pub const ENC_ISO_8859_10: InputEncoding = 7;
pub const ENC_ISO_8859_9: InputEncoding = 6;
pub const ENC_ISO_8859_7: InputEncoding = 5;
pub const ENC_ISO_8859_5: InputEncoding = 4;
pub const ENC_ISO_8859_4: InputEncoding = 3;
pub const ENC_ISO_8859_3: InputEncoding = 2;
pub const ENC_ISO_8859_2: InputEncoding = 1;
pub const ENC_ISO_8859_1: InputEncoding = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct encoding_registry_st {
    pub names: [*mut libc::c_char; 3],
    pub encoding: InputEncoding,
    pub nl: libc::c_int,
    pub bs: libc::c_int,
}
pub type EncodingRegistry = encoding_registry_st;
pub type FormFeedType = libc::c_uint;
pub const FORMFEED_HCOLUMN: FormFeedType = 2;
pub const FORMFEED_PAGE: FormFeedType = 1;
pub const FORMFEED_COLUMN: FormFeedType = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct buffer_st {
    pub data: *mut libc::c_char,
    pub allocated: size_t,
    pub len: size_t,
}
pub type Buffer = buffer_st;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct file_lookup_ctx_st {
    pub name: *mut libc::c_char,
    pub suffix: *mut libc::c_char,
    pub fullname: *mut Buffer,
}
pub type FileLookupCtx = file_lookup_ctx_st;
pub type PathWalkProc = Option::<
    unsafe extern "C" fn(*mut libc::c_char, *mut libc::c_void) -> libc::c_int,
>;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct input_stream_st {
    pub is_pipe: libc::c_int,
    pub fp: *mut FILE,
    pub buf: [libc::c_uchar; 4096],
    pub data_in_buf: libc::c_uint,
    pub bufpos: libc::c_uint,
    pub nreads: libc::c_uint,
    pub unget_ch: *mut libc::c_uchar,
    pub unget_pos: libc::c_uint,
    pub unget_alloc: libc::c_uint,
}
pub type InputStream = input_stream_st;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct font_point_st {
    pub w: libc::c_double,
    pub h: libc::c_double,
}
pub type FontPoint = font_point_st;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct cached_font_info_st {
    pub font_widths: [libc::c_double; 256],
    pub font_ctype: [libc::c_char; 256],
    pub font_is_fixed: AFMBoolean,
    pub font_bbox_lly: AFMNumber,
}
pub type CachedFontInfo = cached_font_info_st;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_1 {
    pub code: libc::c_int,
    pub name: *mut libc::c_char,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_2 {
    pub code: libc::c_int,
    pub name: *mut libc::c_char,
}
#[inline]
unsafe extern "C" fn atof(mut __nptr: *const libc::c_char) -> libc::c_double {
    return strtod(__nptr, 0 as *mut libc::c_void as *mut *mut libc::c_char);
}
#[inline]
unsafe extern "C" fn atoi(mut __nptr: *const libc::c_char) -> libc::c_int {
    return strtol(
        __nptr,
        0 as *mut libc::c_void as *mut *mut libc::c_char,
        10 as libc::c_int,
    ) as libc::c_int;
}
#[inline]
unsafe extern "C" fn stat(
    mut __path: *const libc::c_char,
    mut __statbuf: *mut stat,
) -> libc::c_int {
    return __xstat(1 as libc::c_int, __path, __statbuf);
}
static mut enc_7bit_ascii_fise: [C2RustUnnamed_2; 7] = [
    {
        let mut init = C2RustUnnamed_2 {
            code: '{' as i32,
            name: b"adieresis\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = C2RustUnnamed_2 {
            code: '|' as i32,
            name: b"odieresis\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = C2RustUnnamed_2 {
            code: '}' as i32,
            name: b"aring\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = C2RustUnnamed_2 {
            code: '[' as i32,
            name: b"Adieresis\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = C2RustUnnamed_2 {
            code: '\\' as i32,
            name: b"Odieresis\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = C2RustUnnamed_2 {
            code: ']' as i32,
            name: b"Aring\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = C2RustUnnamed_2 {
            code: 0 as libc::c_int,
            name: 0 as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
];
static mut enc_7bit_ascii_dkno: [C2RustUnnamed_1; 7] = [
    {
        let mut init = C2RustUnnamed_1 {
            code: '{' as i32,
            name: b"ae\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = C2RustUnnamed_1 {
            code: '|' as i32,
            name: b"oslash\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = C2RustUnnamed_1 {
            code: '}' as i32,
            name: b"aring\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = C2RustUnnamed_1 {
            code: '[' as i32,
            name: b"AE\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = C2RustUnnamed_1 {
            code: '\\' as i32,
            name: b"Oslash\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = C2RustUnnamed_1 {
            code: ']' as i32,
            name: b"Aring\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = C2RustUnnamed_1 {
            code: 0 as libc::c_int,
            name: 0 as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
];
pub unsafe extern "C" fn read_config(
    mut path: *mut libc::c_char,
    mut file: *mut libc::c_char,
) -> libc::c_int {
    let mut fp: *mut FILE = 0 as *mut FILE;
    let mut fname_0: Buffer = Buffer {
        data: 0 as *mut libc::c_char,
        allocated: 0,
        len: 0,
    };
    let mut buf: [libc::c_char; 4096] = [0; 4096];
    let mut token: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut token2: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut line: libc::c_int = 0 as libc::c_int;
    buffer_init(&mut fname_0);
    buffer_append(&mut fname_0, path);
    buffer_append(&mut fname_0, b"/\0" as *const u8 as *const libc::c_char);
    buffer_append(&mut fname_0, file);
    fp = fopen(buffer_ptr(&mut fname_0), b"r\0" as *const u8 as *const libc::c_char);
    if fp.is_null() {
        buffer_uninit(&mut fname_0);
        return 0 as libc::c_int;
    }
    while !(fgets(
        buf.as_mut_ptr(),
        ::std::mem::size_of::<[libc::c_char; 4096]>() as libc::c_ulong as libc::c_int,
        fp,
    ))
        .is_null()
    {
        line += 1;
        line;
        if buf[0 as libc::c_int as usize] as libc::c_int == '#' as i32 {
            continue;
        }
        token = strtok(buf.as_mut_ptr(), b" \t\n\0" as *const u8 as *const libc::c_char);
        if token.is_null() {
            continue;
        }
        if strcmp(
            token,
            b"AcceptCompositeCharacters:\0" as *const u8 as *const libc::c_char,
        ) == 0 as libc::c_int
        {
            token2 = strtok(
                0 as *mut libc::c_char,
                b" \t\n\0" as *const u8 as *const libc::c_char,
            );
            if token2.is_null() {
                fprintf(
                    stderr,
                    b"%s:%s:%d: \0" as *const u8 as *const libc::c_char,
                    program,
                    buffer_ptr(&mut fname_0),
                    line,
                );
                fprintf(
                    stderr,
                    dcgettext(
                        0 as *const libc::c_char,
                        b"missing argument: %s\0" as *const u8 as *const libc::c_char,
                        5 as libc::c_int,
                    ),
                    token,
                );
                fprintf(stderr, b"\n\0" as *const u8 as *const libc::c_char);
                fflush(stderr);
                exit(1 as libc::c_int);
            }
            accept_composites = atoi(token2);
        } else if strcmp(token, b"AFMPath:\0" as *const u8 as *const libc::c_char)
            == 0 as libc::c_int
        {
            token2 = strtok(
                0 as *mut libc::c_char,
                b" \t\n\0" as *const u8 as *const libc::c_char,
            );
            if token2.is_null() {
                fprintf(
                    stderr,
                    b"%s:%s:%d: \0" as *const u8 as *const libc::c_char,
                    program,
                    buffer_ptr(&mut fname_0),
                    line,
                );
                fprintf(
                    stderr,
                    dcgettext(
                        0 as *const libc::c_char,
                        b"missing argument: %s\0" as *const u8 as *const libc::c_char,
                        5 as libc::c_int,
                    ),
                    token,
                );
                fprintf(stderr, b"\n\0" as *const u8 as *const libc::c_char);
                fflush(stderr);
                exit(1 as libc::c_int);
            }
            xfree(afm_path as *mut libc::c_void);
            afm_path = xstrdup(token2);
        } else if strcmp(token, b"AppendCtrlD:\0" as *const u8 as *const libc::c_char)
            == 0 as libc::c_int
        {
            token2 = strtok(
                0 as *mut libc::c_char,
                b" \t\n\0" as *const u8 as *const libc::c_char,
            );
            if token2.is_null() {
                fprintf(
                    stderr,
                    b"%s:%s:%d: \0" as *const u8 as *const libc::c_char,
                    program,
                    buffer_ptr(&mut fname_0),
                    line,
                );
                fprintf(
                    stderr,
                    dcgettext(
                        0 as *const libc::c_char,
                        b"missing argument: %s\0" as *const u8 as *const libc::c_char,
                        5 as libc::c_int,
                    ),
                    token,
                );
                fprintf(stderr, b"\n\0" as *const u8 as *const libc::c_char);
                fflush(stderr);
                exit(1 as libc::c_int);
            }
            append_ctrl_D = atoi(token2);
        } else if strcmp(token, b"Clean7Bit:\0" as *const u8 as *const libc::c_char)
            == 0 as libc::c_int
        {
            token2 = strtok(
                0 as *mut libc::c_char,
                b" \t\n\0" as *const u8 as *const libc::c_char,
            );
            if token2.is_null() {
                fprintf(
                    stderr,
                    b"%s:%s:%d: \0" as *const u8 as *const libc::c_char,
                    program,
                    buffer_ptr(&mut fname_0),
                    line,
                );
                fprintf(
                    stderr,
                    dcgettext(
                        0 as *const libc::c_char,
                        b"missing argument: %s\0" as *const u8 as *const libc::c_char,
                        5 as libc::c_int,
                    ),
                    token,
                );
                fprintf(stderr, b"\n\0" as *const u8 as *const libc::c_char);
                fflush(stderr);
                exit(1 as libc::c_int);
            }
            clean_7bit = atoi(token2);
        } else if strcmp(
            token,
            b"DefaultEncoding:\0" as *const u8 as *const libc::c_char,
        ) == 0 as libc::c_int
        {
            token2 = strtok(
                0 as *mut libc::c_char,
                b" \t\n\0" as *const u8 as *const libc::c_char,
            );
            if token2.is_null() {
                fprintf(
                    stderr,
                    b"%s:%s:%d: \0" as *const u8 as *const libc::c_char,
                    program,
                    buffer_ptr(&mut fname_0),
                    line,
                );
                fprintf(
                    stderr,
                    dcgettext(
                        0 as *const libc::c_char,
                        b"missing argument: %s\0" as *const u8 as *const libc::c_char,
                        5 as libc::c_int,
                    ),
                    token,
                );
                fprintf(stderr, b"\n\0" as *const u8 as *const libc::c_char);
                fflush(stderr);
                exit(1 as libc::c_int);
            }
            xfree(encoding_name as *mut libc::c_void);
            encoding_name = xstrdup(token2);
        } else if strcmp(
            token,
            b"DefaultFancyHeader:\0" as *const u8 as *const libc::c_char,
        ) == 0 as libc::c_int
        {
            token2 = strtok(
                0 as *mut libc::c_char,
                b" \t\n\0" as *const u8 as *const libc::c_char,
            );
            if token2.is_null() {
                fprintf(
                    stderr,
                    b"%s:%s:%d: \0" as *const u8 as *const libc::c_char,
                    program,
                    buffer_ptr(&mut fname_0),
                    line,
                );
                fprintf(
                    stderr,
                    dcgettext(
                        0 as *const libc::c_char,
                        b"missing argument: %s\0" as *const u8 as *const libc::c_char,
                        5 as libc::c_int,
                    ),
                    token,
                );
                fprintf(stderr, b"\n\0" as *const u8 as *const libc::c_char);
                fflush(stderr);
                exit(1 as libc::c_int);
            }
            xfree(fancy_header_default as *mut libc::c_void);
            fancy_header_default = xstrdup(token2);
        } else if strcmp(token, b"DefaultMedia:\0" as *const u8 as *const libc::c_char)
            == 0 as libc::c_int
        {
            token2 = strtok(
                0 as *mut libc::c_char,
                b" \t\n\0" as *const u8 as *const libc::c_char,
            );
            if token2.is_null() {
                fprintf(
                    stderr,
                    b"%s:%s:%d: \0" as *const u8 as *const libc::c_char,
                    program,
                    buffer_ptr(&mut fname_0),
                    line,
                );
                fprintf(
                    stderr,
                    dcgettext(
                        0 as *const libc::c_char,
                        b"missing argument: %s\0" as *const u8 as *const libc::c_char,
                        5 as libc::c_int,
                    ),
                    token,
                );
                fprintf(stderr, b"\n\0" as *const u8 as *const libc::c_char);
                fflush(stderr);
                exit(1 as libc::c_int);
            }
            xfree(media_name as *mut libc::c_void);
            media_name = xstrdup(token2);
        } else if strcmp(
            token,
            b"DefaultOutputMethod:\0" as *const u8 as *const libc::c_char,
        ) == 0 as libc::c_int
        {
            token2 = strtok(
                0 as *mut libc::c_char,
                b" \t\n\0" as *const u8 as *const libc::c_char,
            );
            if token2.is_null() {
                fprintf(
                    stderr,
                    b"%s:%s:%d: \0" as *const u8 as *const libc::c_char,
                    program,
                    buffer_ptr(&mut fname_0),
                    line,
                );
                fprintf(
                    stderr,
                    dcgettext(
                        0 as *const libc::c_char,
                        b"missing argument: %s\0" as *const u8 as *const libc::c_char,
                        5 as libc::c_int,
                    ),
                    token,
                );
                fprintf(stderr, b"\n\0" as *const u8 as *const libc::c_char);
                fflush(stderr);
                exit(1 as libc::c_int);
            }
            if strcmp(token2, b"printer\0" as *const u8 as *const libc::c_char)
                == 0 as libc::c_int
            {
                output_file = 0 as *mut libc::c_char;
            } else if strcmp(token2, b"stdout\0" as *const u8 as *const libc::c_char)
                == 0 as libc::c_int
            {
                output_file = 1 as libc::c_int as *mut libc::c_char;
            } else {
                fprintf(
                    stderr,
                    b"%s:%s:%d: \0" as *const u8 as *const libc::c_char,
                    program,
                    buffer_ptr(&mut fname_0),
                    line,
                );
                fprintf(
                    stderr,
                    dcgettext(
                        0 as *const libc::c_char,
                        b"illegal value \"%s\" for option %s\0" as *const u8
                            as *const libc::c_char,
                        5 as libc::c_int,
                    ),
                    token2,
                    token,
                );
                fprintf(stderr, b"\n\0" as *const u8 as *const libc::c_char);
                fflush(stderr);
                exit(1 as libc::c_int);
            }
        } else if strcmp(token, b"DownloadFont:\0" as *const u8 as *const libc::c_char)
            == 0 as libc::c_int
        {
            token2 = strtok(
                0 as *mut libc::c_char,
                b" \t\n\0" as *const u8 as *const libc::c_char,
            );
            if token2.is_null() {
                fprintf(
                    stderr,
                    b"%s:%s:%d: \0" as *const u8 as *const libc::c_char,
                    program,
                    buffer_ptr(&mut fname_0),
                    line,
                );
                fprintf(
                    stderr,
                    dcgettext(
                        0 as *const libc::c_char,
                        b"missing argument: %s\0" as *const u8 as *const libc::c_char,
                        5 as libc::c_int,
                    ),
                    token,
                );
                fprintf(stderr, b"\n\0" as *const u8 as *const libc::c_char);
                fflush(stderr);
                exit(1 as libc::c_int);
            }
            strhash_put(
                download_fonts,
                token2,
                (strlen(token2)).wrapping_add(1 as libc::c_int as libc::c_ulong)
                    as libc::c_int,
                0 as *mut libc::c_void,
                0 as *mut *mut libc::c_void,
            );
        } else if strcmp(token, b"EscapeChar:\0" as *const u8 as *const libc::c_char)
            == 0 as libc::c_int
        {
            token2 = strtok(
                0 as *mut libc::c_char,
                b" \t\n\0" as *const u8 as *const libc::c_char,
            );
            if token2.is_null() {
                fprintf(
                    stderr,
                    b"%s:%s:%d: \0" as *const u8 as *const libc::c_char,
                    program,
                    buffer_ptr(&mut fname_0),
                    line,
                );
                fprintf(
                    stderr,
                    dcgettext(
                        0 as *const libc::c_char,
                        b"missing argument: %s\0" as *const u8 as *const libc::c_char,
                        5 as libc::c_int,
                    ),
                    token,
                );
                fprintf(stderr, b"\n\0" as *const u8 as *const libc::c_char);
                fflush(stderr);
                exit(1 as libc::c_int);
            }
            escape_char = atoi(token2);
            if escape_char < 0 as libc::c_int || escape_char > 255 as libc::c_int {
                fprintf(
                    stderr,
                    b"%s:%s:%d: \0" as *const u8 as *const libc::c_char,
                    program,
                    buffer_ptr(&mut fname_0),
                    line,
                );
                fprintf(
                    stderr,
                    dcgettext(
                        0 as *const libc::c_char,
                        b"invalid value \"%s\" for option %s\0" as *const u8
                            as *const libc::c_char,
                        5 as libc::c_int,
                    ),
                    token2,
                    token,
                );
                fprintf(stderr, b"\n\0" as *const u8 as *const libc::c_char);
                fflush(stderr);
                exit(1 as libc::c_int);
            }
        } else if strcmp(token, b"FormFeedType:\0" as *const u8 as *const libc::c_char)
            == 0 as libc::c_int
        {
            token2 = strtok(
                0 as *mut libc::c_char,
                b" \t\n\0" as *const u8 as *const libc::c_char,
            );
            if token2.is_null() {
                fprintf(
                    stderr,
                    b"%s:%s:%d: \0" as *const u8 as *const libc::c_char,
                    program,
                    buffer_ptr(&mut fname_0),
                    line,
                );
                fprintf(
                    stderr,
                    dcgettext(
                        0 as *const libc::c_char,
                        b"missing argument: %s\0" as *const u8 as *const libc::c_char,
                        5 as libc::c_int,
                    ),
                    token,
                );
                fprintf(stderr, b"\n\0" as *const u8 as *const libc::c_char);
                fflush(stderr);
                exit(1 as libc::c_int);
            }
            if strcmp(token2, b"column\0" as *const u8 as *const libc::c_char)
                == 0 as libc::c_int
            {
                formfeed_type = FORMFEED_COLUMN;
            } else if strcmp(token2, b"page\0" as *const u8 as *const libc::c_char)
                == 0 as libc::c_int
            {
                formfeed_type = FORMFEED_PAGE;
            } else {
                fprintf(
                    stderr,
                    b"%s:%s:%d: \0" as *const u8 as *const libc::c_char,
                    program,
                    buffer_ptr(&mut fname_0),
                    line,
                );
                fprintf(
                    stderr,
                    dcgettext(
                        0 as *const libc::c_char,
                        b"illegal value \"%s\" for option %s\0" as *const u8
                            as *const libc::c_char,
                        5 as libc::c_int,
                    ),
                    token2,
                    token,
                );
                fprintf(stderr, b"\n\0" as *const u8 as *const libc::c_char);
                fflush(stderr);
                exit(1 as libc::c_int);
            }
        } else if strcmp(
            token,
            b"GeneratePageSize:\0" as *const u8 as *const libc::c_char,
        ) == 0 as libc::c_int
        {
            token2 = strtok(
                0 as *mut libc::c_char,
                b" \t\n\0" as *const u8 as *const libc::c_char,
            );
            if token2.is_null() {
                fprintf(
                    stderr,
                    b"%s:%s:%d: \0" as *const u8 as *const libc::c_char,
                    program,
                    buffer_ptr(&mut fname_0),
                    line,
                );
                fprintf(
                    stderr,
                    dcgettext(
                        0 as *const libc::c_char,
                        b"missing argument: %s\0" as *const u8 as *const libc::c_char,
                        5 as libc::c_int,
                    ),
                    token,
                );
                fprintf(stderr, b"\n\0" as *const u8 as *const libc::c_char);
                fflush(stderr);
                exit(1 as libc::c_int);
            }
            generate_PageSize = atoi(token2);
        } else if strcmp(
            token,
            b"HighlightBarGray:\0" as *const u8 as *const libc::c_char,
        ) == 0 as libc::c_int
        {
            token2 = strtok(
                0 as *mut libc::c_char,
                b" \t\n\0" as *const u8 as *const libc::c_char,
            );
            if token2.is_null() {
                fprintf(
                    stderr,
                    b"%s:%s:%d: \0" as *const u8 as *const libc::c_char,
                    program,
                    buffer_ptr(&mut fname_0),
                    line,
                );
                fprintf(
                    stderr,
                    dcgettext(
                        0 as *const libc::c_char,
                        b"missing argument: %s\0" as *const u8 as *const libc::c_char,
                        5 as libc::c_int,
                    ),
                    token,
                );
                fprintf(stderr, b"\n\0" as *const u8 as *const libc::c_char);
                fflush(stderr);
                exit(1 as libc::c_int);
            }
            highlight_bar_gray = atof(token2);
        } else if strcmp(token, b"HighlightBars:\0" as *const u8 as *const libc::c_char)
            == 0 as libc::c_int
        {
            token2 = strtok(
                0 as *mut libc::c_char,
                b" \t\n\0" as *const u8 as *const libc::c_char,
            );
            if token2.is_null() {
                fprintf(
                    stderr,
                    b"%s:%s:%d: \0" as *const u8 as *const libc::c_char,
                    program,
                    buffer_ptr(&mut fname_0),
                    line,
                );
                fprintf(
                    stderr,
                    dcgettext(
                        0 as *const libc::c_char,
                        b"missing argument: %s\0" as *const u8 as *const libc::c_char,
                        5 as libc::c_int,
                    ),
                    token,
                );
                fprintf(stderr, b"\n\0" as *const u8 as *const libc::c_char);
                fflush(stderr);
                exit(1 as libc::c_int);
            }
            highlight_bars = atoi(token2) as libc::c_uint;
        } else if strcmp(token, b"LibraryPath:\0" as *const u8 as *const libc::c_char)
            == 0 as libc::c_int
        {
            token2 = strtok(
                0 as *mut libc::c_char,
                b" \t\n\0" as *const u8 as *const libc::c_char,
            );
            if token2.is_null() {
                fprintf(
                    stderr,
                    b"%s:%s:%d: \0" as *const u8 as *const libc::c_char,
                    program,
                    buffer_ptr(&mut fname_0),
                    line,
                );
                fprintf(
                    stderr,
                    dcgettext(
                        0 as *const libc::c_char,
                        b"missing argument: %s\0" as *const u8 as *const libc::c_char,
                        5 as libc::c_int,
                    ),
                    token,
                );
                fprintf(stderr, b"\n\0" as *const u8 as *const libc::c_char);
                fflush(stderr);
                exit(1 as libc::c_int);
            }
            xfree(libpath as *mut libc::c_void);
            libpath = xstrdup(token2);
        } else if strcmp(
            token,
            b"MarkWrappedLines:\0" as *const u8 as *const libc::c_char,
        ) == 0 as libc::c_int
        {
            token2 = strtok(
                0 as *mut libc::c_char,
                b" \t\n\0" as *const u8 as *const libc::c_char,
            );
            if token2.is_null() {
                fprintf(
                    stderr,
                    b"%s:%s:%d: \0" as *const u8 as *const libc::c_char,
                    program,
                    buffer_ptr(&mut fname_0),
                    line,
                );
                fprintf(
                    stderr,
                    dcgettext(
                        0 as *const libc::c_char,
                        b"missing argument: %s\0" as *const u8 as *const libc::c_char,
                        5 as libc::c_int,
                    ),
                    token,
                );
                fprintf(stderr, b"\n\0" as *const u8 as *const libc::c_char);
                fflush(stderr);
                exit(1 as libc::c_int);
            }
            xfree(mark_wrapped_lines_style_name as *mut libc::c_void);
            mark_wrapped_lines_style_name = xstrdup(token2);
        } else if strcmp(token, b"Media:\0" as *const u8 as *const libc::c_char)
            == 0 as libc::c_int
        {
            let mut name: *mut libc::c_char = 0 as *mut libc::c_char;
            let mut w: libc::c_int = 0;
            let mut h: libc::c_int = 0;
            let mut llx: libc::c_int = 0;
            let mut lly: libc::c_int = 0;
            let mut urx: libc::c_int = 0;
            let mut ury: libc::c_int = 0;
            token2 = strtok(
                0 as *mut libc::c_char,
                b" \t\n\0" as *const u8 as *const libc::c_char,
            );
            if token2.is_null() {
                fprintf(
                    stderr,
                    b"%s:%s:%d: \0" as *const u8 as *const libc::c_char,
                    program,
                    buffer_ptr(&mut fname_0),
                    line,
                );
                fprintf(
                    stderr,
                    dcgettext(
                        0 as *const libc::c_char,
                        b"missing argument: %s\0" as *const u8 as *const libc::c_char,
                        5 as libc::c_int,
                    ),
                    token,
                );
                fprintf(stderr, b"\n\0" as *const u8 as *const libc::c_char);
                fflush(stderr);
                exit(1 as libc::c_int);
            }
            name = token2;
            token2 = strtok(
                0 as *mut libc::c_char,
                b" \t\n\0" as *const u8 as *const libc::c_char,
            );
            if token2.is_null() {
                fprintf(
                    stderr,
                    b"%s:%s:%d: \0" as *const u8 as *const libc::c_char,
                    program,
                    buffer_ptr(&mut fname_0),
                    line,
                );
                fprintf(
                    stderr,
                    dcgettext(
                        0 as *const libc::c_char,
                        b"missing argument: %s\0" as *const u8 as *const libc::c_char,
                        5 as libc::c_int,
                    ),
                    token,
                );
                fprintf(stderr, b"\n\0" as *const u8 as *const libc::c_char);
                fflush(stderr);
                exit(1 as libc::c_int);
            }
            w = atoi(token2);
            token2 = strtok(
                0 as *mut libc::c_char,
                b" \t\n\0" as *const u8 as *const libc::c_char,
            );
            if token2.is_null() {
                fprintf(
                    stderr,
                    b"%s:%s:%d: \0" as *const u8 as *const libc::c_char,
                    program,
                    buffer_ptr(&mut fname_0),
                    line,
                );
                fprintf(
                    stderr,
                    dcgettext(
                        0 as *const libc::c_char,
                        b"missing argument: %s\0" as *const u8 as *const libc::c_char,
                        5 as libc::c_int,
                    ),
                    token,
                );
                fprintf(stderr, b"\n\0" as *const u8 as *const libc::c_char);
                fflush(stderr);
                exit(1 as libc::c_int);
            }
            h = atoi(token2);
            token2 = strtok(
                0 as *mut libc::c_char,
                b" \t\n\0" as *const u8 as *const libc::c_char,
            );
            if token2.is_null() {
                fprintf(
                    stderr,
                    b"%s:%s:%d: \0" as *const u8 as *const libc::c_char,
                    program,
                    buffer_ptr(&mut fname_0),
                    line,
                );
                fprintf(
                    stderr,
                    dcgettext(
                        0 as *const libc::c_char,
                        b"missing argument: %s\0" as *const u8 as *const libc::c_char,
                        5 as libc::c_int,
                    ),
                    token,
                );
                fprintf(stderr, b"\n\0" as *const u8 as *const libc::c_char);
                fflush(stderr);
                exit(1 as libc::c_int);
            }
            llx = atoi(token2);
            token2 = strtok(
                0 as *mut libc::c_char,
                b" \t\n\0" as *const u8 as *const libc::c_char,
            );
            if token2.is_null() {
                fprintf(
                    stderr,
                    b"%s:%s:%d: \0" as *const u8 as *const libc::c_char,
                    program,
                    buffer_ptr(&mut fname_0),
                    line,
                );
                fprintf(
                    stderr,
                    dcgettext(
                        0 as *const libc::c_char,
                        b"missing argument: %s\0" as *const u8 as *const libc::c_char,
                        5 as libc::c_int,
                    ),
                    token,
                );
                fprintf(stderr, b"\n\0" as *const u8 as *const libc::c_char);
                fflush(stderr);
                exit(1 as libc::c_int);
            }
            lly = atoi(token2);
            token2 = strtok(
                0 as *mut libc::c_char,
                b" \t\n\0" as *const u8 as *const libc::c_char,
            );
            if token2.is_null() {
                fprintf(
                    stderr,
                    b"%s:%s:%d: \0" as *const u8 as *const libc::c_char,
                    program,
                    buffer_ptr(&mut fname_0),
                    line,
                );
                fprintf(
                    stderr,
                    dcgettext(
                        0 as *const libc::c_char,
                        b"missing argument: %s\0" as *const u8 as *const libc::c_char,
                        5 as libc::c_int,
                    ),
                    token,
                );
                fprintf(stderr, b"\n\0" as *const u8 as *const libc::c_char);
                fflush(stderr);
                exit(1 as libc::c_int);
            }
            urx = atoi(token2);
            token2 = strtok(
                0 as *mut libc::c_char,
                b" \t\n\0" as *const u8 as *const libc::c_char,
            );
            if token2.is_null() {
                fprintf(
                    stderr,
                    b"%s:%s:%d: \0" as *const u8 as *const libc::c_char,
                    program,
                    buffer_ptr(&mut fname_0),
                    line,
                );
                fprintf(
                    stderr,
                    dcgettext(
                        0 as *const libc::c_char,
                        b"missing argument: %s\0" as *const u8 as *const libc::c_char,
                        5 as libc::c_int,
                    ),
                    token,
                );
                fprintf(stderr, b"\n\0" as *const u8 as *const libc::c_char);
                fflush(stderr);
                exit(1 as libc::c_int);
            }
            ury = atoi(token2);
            add_media(name, w, h, llx, lly, urx, ury);
        } else if strcmp(
            token,
            b"NoJobHeaderSwitch:\0" as *const u8 as *const libc::c_char,
        ) == 0 as libc::c_int
        {
            token2 = strtok(
                0 as *mut libc::c_char,
                b"\n\0" as *const u8 as *const libc::c_char,
            );
            if token2.is_null() {
                fprintf(
                    stderr,
                    b"%s:%s:%d: \0" as *const u8 as *const libc::c_char,
                    program,
                    buffer_ptr(&mut fname_0),
                    line,
                );
                fprintf(
                    stderr,
                    dcgettext(
                        0 as *const libc::c_char,
                        b"missing argument: %s\0" as *const u8 as *const libc::c_char,
                        5 as libc::c_int,
                    ),
                    token,
                );
                fprintf(stderr, b"\n\0" as *const u8 as *const libc::c_char);
                fflush(stderr);
                exit(1 as libc::c_int);
            }
            xfree(no_job_header_switch as *mut libc::c_void);
            no_job_header_switch = xstrdup(token2);
        } else if strcmp(
            token,
            b"NonPrintableFormat:\0" as *const u8 as *const libc::c_char,
        ) == 0 as libc::c_int
        {
            token2 = strtok(
                0 as *mut libc::c_char,
                b" \t\n\0" as *const u8 as *const libc::c_char,
            );
            if token2.is_null() {
                fprintf(
                    stderr,
                    b"%s:%s:%d: \0" as *const u8 as *const libc::c_char,
                    program,
                    buffer_ptr(&mut fname_0),
                    line,
                );
                fprintf(
                    stderr,
                    dcgettext(
                        0 as *const libc::c_char,
                        b"missing argument: %s\0" as *const u8 as *const libc::c_char,
                        5 as libc::c_int,
                    ),
                    token,
                );
                fprintf(stderr, b"\n\0" as *const u8 as *const libc::c_char);
                fflush(stderr);
                exit(1 as libc::c_int);
            }
            xfree(npf_name as *mut libc::c_void);
            npf_name = xstrdup(token2);
        } else if strcmp(
            token,
            b"OutputFirstLine:\0" as *const u8 as *const libc::c_char,
        ) == 0 as libc::c_int
        {
            token2 = strtok(
                0 as *mut libc::c_char,
                b"\n\0" as *const u8 as *const libc::c_char,
            );
            if token2.is_null() {
                fprintf(
                    stderr,
                    b"%s:%s:%d: \0" as *const u8 as *const libc::c_char,
                    program,
                    buffer_ptr(&mut fname_0),
                    line,
                );
                fprintf(
                    stderr,
                    dcgettext(
                        0 as *const libc::c_char,
                        b"missing argument: %s\0" as *const u8 as *const libc::c_char,
                        5 as libc::c_int,
                    ),
                    token,
                );
                fprintf(stderr, b"\n\0" as *const u8 as *const libc::c_char);
                fflush(stderr);
                exit(1 as libc::c_int);
            }
            xfree(output_first_line as *mut libc::c_void);
            output_first_line = xstrdup(token2);
        } else if strcmp(
            token,
            b"PageLabelFormat:\0" as *const u8 as *const libc::c_char,
        ) == 0 as libc::c_int
        {
            token2 = strtok(
                0 as *mut libc::c_char,
                b" \t\n\0" as *const u8 as *const libc::c_char,
            );
            if token2.is_null() {
                fprintf(
                    stderr,
                    b"%s:%s:%d: \0" as *const u8 as *const libc::c_char,
                    program,
                    buffer_ptr(&mut fname_0),
                    line,
                );
                fprintf(
                    stderr,
                    dcgettext(
                        0 as *const libc::c_char,
                        b"missing argument: %s\0" as *const u8 as *const libc::c_char,
                        5 as libc::c_int,
                    ),
                    token,
                );
                fprintf(stderr, b"\n\0" as *const u8 as *const libc::c_char);
                fflush(stderr);
                exit(1 as libc::c_int);
            }
            xfree(page_label_format as *mut libc::c_void);
            page_label_format = xstrdup(token2);
        } else if strcmp(token, b"PagePrefeed:\0" as *const u8 as *const libc::c_char)
            == 0 as libc::c_int
        {
            token2 = strtok(
                0 as *mut libc::c_char,
                b" \t\n\0" as *const u8 as *const libc::c_char,
            );
            if token2.is_null() {
                fprintf(
                    stderr,
                    b"%s:%s:%d: \0" as *const u8 as *const libc::c_char,
                    program,
                    buffer_ptr(&mut fname_0),
                    line,
                );
                fprintf(
                    stderr,
                    dcgettext(
                        0 as *const libc::c_char,
                        b"missing argument: %s\0" as *const u8 as *const libc::c_char,
                        5 as libc::c_int,
                    ),
                    token,
                );
                fprintf(stderr, b"\n\0" as *const u8 as *const libc::c_char);
                fflush(stderr);
                exit(1 as libc::c_int);
            }
            page_prefeed = atoi(token2);
        } else if strcmp(
            token,
            b"PostScriptLevel:\0" as *const u8 as *const libc::c_char,
        ) == 0 as libc::c_int
        {
            token2 = strtok(
                0 as *mut libc::c_char,
                b" \t\n\0" as *const u8 as *const libc::c_char,
            );
            if token2.is_null() {
                fprintf(
                    stderr,
                    b"%s:%s:%d: \0" as *const u8 as *const libc::c_char,
                    program,
                    buffer_ptr(&mut fname_0),
                    line,
                );
                fprintf(
                    stderr,
                    dcgettext(
                        0 as *const libc::c_char,
                        b"missing argument: %s\0" as *const u8 as *const libc::c_char,
                        5 as libc::c_int,
                    ),
                    token,
                );
                fprintf(stderr, b"\n\0" as *const u8 as *const libc::c_char);
                fflush(stderr);
                exit(1 as libc::c_int);
            }
            pslevel = atoi(token2) as libc::c_uint;
        } else if strcmp(token, b"Printer:\0" as *const u8 as *const libc::c_char)
            == 0 as libc::c_int
        {
            token2 = strtok(
                0 as *mut libc::c_char,
                b" \t\n\0" as *const u8 as *const libc::c_char,
            );
            if token2.is_null() {
                fprintf(
                    stderr,
                    b"%s:%s:%d: \0" as *const u8 as *const libc::c_char,
                    program,
                    buffer_ptr(&mut fname_0),
                    line,
                );
                fprintf(
                    stderr,
                    dcgettext(
                        0 as *const libc::c_char,
                        b"missing argument: %s\0" as *const u8 as *const libc::c_char,
                        5 as libc::c_int,
                    ),
                    token,
                );
                fprintf(stderr, b"\n\0" as *const u8 as *const libc::c_char);
                fflush(stderr);
                exit(1 as libc::c_int);
            }
            xfree(printer as *mut libc::c_void);
            printer = xstrdup(token2);
        } else if strcmp(token, b"QueueParam:\0" as *const u8 as *const libc::c_char)
            == 0 as libc::c_int
        {
            token2 = strtok(
                0 as *mut libc::c_char,
                b"\n\0" as *const u8 as *const libc::c_char,
            );
            if token2.is_null() {
                fprintf(
                    stderr,
                    b"%s:%s:%d: \0" as *const u8 as *const libc::c_char,
                    program,
                    buffer_ptr(&mut fname_0),
                    line,
                );
                fprintf(
                    stderr,
                    dcgettext(
                        0 as *const libc::c_char,
                        b"missing argument: %s\0" as *const u8 as *const libc::c_char,
                        5 as libc::c_int,
                    ),
                    token,
                );
                fprintf(stderr, b"\n\0" as *const u8 as *const libc::c_char);
                fflush(stderr);
                exit(1 as libc::c_int);
            }
            xfree(queue_param as *mut libc::c_void);
            queue_param = xstrdup(token2);
        } else if strcmp(token, b"SetPageDevice:\0" as *const u8 as *const libc::c_char)
            == 0 as libc::c_int
        {
            token2 = strtok(
                0 as *mut libc::c_char,
                b"\n\0" as *const u8 as *const libc::c_char,
            );
            if token2.is_null() {
                fprintf(
                    stderr,
                    b"%s:%s:%d: \0" as *const u8 as *const libc::c_char,
                    program,
                    buffer_ptr(&mut fname_0),
                    line,
                );
                fprintf(
                    stderr,
                    dcgettext(
                        0 as *const libc::c_char,
                        b"missing argument: %s\0" as *const u8 as *const libc::c_char,
                        5 as libc::c_int,
                    ),
                    token,
                );
                fprintf(stderr, b"\n\0" as *const u8 as *const libc::c_char);
                fflush(stderr);
                exit(1 as libc::c_int);
            }
            parse_key_value_pair(pagedevice, token2);
        } else if strcmp(token, b"Spooler:\0" as *const u8 as *const libc::c_char)
            == 0 as libc::c_int
        {
            token2 = strtok(
                0 as *mut libc::c_char,
                b" \t\n\0" as *const u8 as *const libc::c_char,
            );
            if token2.is_null() {
                fprintf(
                    stderr,
                    b"%s:%s:%d: \0" as *const u8 as *const libc::c_char,
                    program,
                    buffer_ptr(&mut fname_0),
                    line,
                );
                fprintf(
                    stderr,
                    dcgettext(
                        0 as *const libc::c_char,
                        b"missing argument: %s\0" as *const u8 as *const libc::c_char,
                        5 as libc::c_int,
                    ),
                    token,
                );
                fprintf(stderr, b"\n\0" as *const u8 as *const libc::c_char);
                fflush(stderr);
                exit(1 as libc::c_int);
            }
            xfree(spooler_command as *mut libc::c_void);
            spooler_command = xstrdup(token2);
        } else if strcmp(token, b"StatesBinary:\0" as *const u8 as *const libc::c_char)
            == 0 as libc::c_int
        {
            token2 = strtok(
                0 as *mut libc::c_char,
                b" \t\n\0" as *const u8 as *const libc::c_char,
            );
            if token2.is_null() {
                fprintf(
                    stderr,
                    b"%s:%s:%d: \0" as *const u8 as *const libc::c_char,
                    program,
                    buffer_ptr(&mut fname_0),
                    line,
                );
                fprintf(
                    stderr,
                    dcgettext(
                        0 as *const libc::c_char,
                        b"missing argument: %s\0" as *const u8 as *const libc::c_char,
                        5 as libc::c_int,
                    ),
                    token,
                );
                fprintf(stderr, b"\n\0" as *const u8 as *const libc::c_char);
                fflush(stderr);
                exit(1 as libc::c_int);
            }
            xfree(states_binary as *mut libc::c_void);
            states_binary = xstrdup(token2);
        } else if strcmp(token, b"StatesColor:\0" as *const u8 as *const libc::c_char)
            == 0 as libc::c_int
        {
            token2 = strtok(
                0 as *mut libc::c_char,
                b" \t\n\0" as *const u8 as *const libc::c_char,
            );
            if token2.is_null() {
                fprintf(
                    stderr,
                    b"%s:%s:%d: \0" as *const u8 as *const libc::c_char,
                    program,
                    buffer_ptr(&mut fname_0),
                    line,
                );
                fprintf(
                    stderr,
                    dcgettext(
                        0 as *const libc::c_char,
                        b"missing argument: %s\0" as *const u8 as *const libc::c_char,
                        5 as libc::c_int,
                    ),
                    token,
                );
                fprintf(stderr, b"\n\0" as *const u8 as *const libc::c_char);
                fflush(stderr);
                exit(1 as libc::c_int);
            }
            states_color = atoi(token2);
        } else if strcmp(
            token,
            b"StatesConfigFile:\0" as *const u8 as *const libc::c_char,
        ) == 0 as libc::c_int
        {
            token2 = strtok(
                0 as *mut libc::c_char,
                b"\n\0" as *const u8 as *const libc::c_char,
            );
            if token2.is_null() {
                fprintf(
                    stderr,
                    b"%s:%s:%d: \0" as *const u8 as *const libc::c_char,
                    program,
                    buffer_ptr(&mut fname_0),
                    line,
                );
                fprintf(
                    stderr,
                    dcgettext(
                        0 as *const libc::c_char,
                        b"missing argument: %s\0" as *const u8 as *const libc::c_char,
                        5 as libc::c_int,
                    ),
                    token,
                );
                fprintf(stderr, b"\n\0" as *const u8 as *const libc::c_char);
                fflush(stderr);
                exit(1 as libc::c_int);
            }
            xfree(states_config_file as *mut libc::c_void);
            states_config_file = xstrdup(token2);
        } else if strcmp(
            token,
            b"StatesHighlightStyle:\0" as *const u8 as *const libc::c_char,
        ) == 0 as libc::c_int
        {
            token2 = strtok(
                0 as *mut libc::c_char,
                b" \t\n\0" as *const u8 as *const libc::c_char,
            );
            if token2.is_null() {
                fprintf(
                    stderr,
                    b"%s:%s:%d: \0" as *const u8 as *const libc::c_char,
                    program,
                    buffer_ptr(&mut fname_0),
                    line,
                );
                fprintf(
                    stderr,
                    dcgettext(
                        0 as *const libc::c_char,
                        b"missing argument: %s\0" as *const u8 as *const libc::c_char,
                        5 as libc::c_int,
                    ),
                    token,
                );
                fprintf(stderr, b"\n\0" as *const u8 as *const libc::c_char);
                fflush(stderr);
                exit(1 as libc::c_int);
            }
            xfree(states_highlight_style as *mut libc::c_void);
            states_highlight_style = xstrdup(token2);
        } else if strcmp(token, b"StatesPath:\0" as *const u8 as *const libc::c_char)
            == 0 as libc::c_int
        {
            token2 = strtok(
                0 as *mut libc::c_char,
                b"\n\0" as *const u8 as *const libc::c_char,
            );
            if token2.is_null() {
                fprintf(
                    stderr,
                    b"%s:%s:%d: \0" as *const u8 as *const libc::c_char,
                    program,
                    buffer_ptr(&mut fname_0),
                    line,
                );
                fprintf(
                    stderr,
                    dcgettext(
                        0 as *const libc::c_char,
                        b"missing argument: %s\0" as *const u8 as *const libc::c_char,
                        5 as libc::c_int,
                    ),
                    token,
                );
                fprintf(stderr, b"\n\0" as *const u8 as *const libc::c_char);
                fflush(stderr);
                exit(1 as libc::c_int);
            }
            xfree(states_path as *mut libc::c_void);
            states_path = xstrdup(token2);
        } else if strcmp(token, b"StatusDict:\0" as *const u8 as *const libc::c_char)
            == 0 as libc::c_int
        {
            token2 = strtok(
                0 as *mut libc::c_char,
                b" \t\n\0" as *const u8 as *const libc::c_char,
            );
            if token2.is_null() {
                fprintf(
                    stderr,
                    b"%s:%s:%d: \0" as *const u8 as *const libc::c_char,
                    program,
                    buffer_ptr(&mut fname_0),
                    line,
                );
                fprintf(
                    stderr,
                    dcgettext(
                        0 as *const libc::c_char,
                        b"missing argument: %s\0" as *const u8 as *const libc::c_char,
                        5 as libc::c_int,
                    ),
                    token,
                );
                fprintf(stderr, b"\n\0" as *const u8 as *const libc::c_char);
                fflush(stderr);
                exit(1 as libc::c_int);
            }
            parse_key_value_pair(statusdict, token2);
        } else if strcmp(token, b"TOCFormat:\0" as *const u8 as *const libc::c_char)
            == 0 as libc::c_int
        {
            token2 = strtok(
                0 as *mut libc::c_char,
                b"\n\0" as *const u8 as *const libc::c_char,
            );
            if token2.is_null() {
                fprintf(
                    stderr,
                    b"%s:%s:%d: \0" as *const u8 as *const libc::c_char,
                    program,
                    buffer_ptr(&mut fname_0),
                    line,
                );
                fprintf(
                    stderr,
                    dcgettext(
                        0 as *const libc::c_char,
                        b"missing argument: %s\0" as *const u8 as *const libc::c_char,
                        5 as libc::c_int,
                    ),
                    token,
                );
                fprintf(stderr, b"\n\0" as *const u8 as *const libc::c_char);
                fflush(stderr);
                exit(1 as libc::c_int);
            }
            toc_fmt_string = xstrdup(token2);
        } else if strcmp(token, b"Underlay:\0" as *const u8 as *const libc::c_char)
            == 0 as libc::c_int
        {
            token2 = strtok(
                0 as *mut libc::c_char,
                b"\n\0" as *const u8 as *const libc::c_char,
            );
            if token2.is_null() {
                fprintf(
                    stderr,
                    b"%s:%s:%d: \0" as *const u8 as *const libc::c_char,
                    program,
                    buffer_ptr(&mut fname_0),
                    line,
                );
                fprintf(
                    stderr,
                    dcgettext(
                        0 as *const libc::c_char,
                        b"missing argument: %s\0" as *const u8 as *const libc::c_char,
                        5 as libc::c_int,
                    ),
                    token,
                );
                fprintf(stderr, b"\n\0" as *const u8 as *const libc::c_char);
                fflush(stderr);
                exit(1 as libc::c_int);
            }
            underlay = xmalloc(
                (strlen(token2)).wrapping_add(1 as libc::c_int as libc::c_ulong),
            ) as *mut libc::c_char;
            strcpy(underlay, token2);
        } else if strcmp(token, b"UnderlayAngle:\0" as *const u8 as *const libc::c_char)
            == 0 as libc::c_int
        {
            token2 = strtok(
                0 as *mut libc::c_char,
                b" \t\n\0" as *const u8 as *const libc::c_char,
            );
            if token2.is_null() {
                fprintf(
                    stderr,
                    b"%s:%s:%d: \0" as *const u8 as *const libc::c_char,
                    program,
                    buffer_ptr(&mut fname_0),
                    line,
                );
                fprintf(
                    stderr,
                    dcgettext(
                        0 as *const libc::c_char,
                        b"missing argument: %s\0" as *const u8 as *const libc::c_char,
                        5 as libc::c_int,
                    ),
                    token,
                );
                fprintf(stderr, b"\n\0" as *const u8 as *const libc::c_char);
                fflush(stderr);
                exit(1 as libc::c_int);
            }
            ul_angle = atof(token2);
            ul_angle_p = 1 as libc::c_int;
        } else if strcmp(token, b"UnderlayFont:\0" as *const u8 as *const libc::c_char)
            == 0 as libc::c_int
        {
            token2 = strtok(
                0 as *mut libc::c_char,
                b" \t\n\0" as *const u8 as *const libc::c_char,
            );
            if token2.is_null() {
                fprintf(
                    stderr,
                    b"%s:%s:%d: \0" as *const u8 as *const libc::c_char,
                    program,
                    buffer_ptr(&mut fname_0),
                    line,
                );
                fprintf(
                    stderr,
                    dcgettext(
                        0 as *const libc::c_char,
                        b"missing argument: %s\0" as *const u8 as *const libc::c_char,
                        5 as libc::c_int,
                    ),
                    token,
                );
                fprintf(stderr, b"\n\0" as *const u8 as *const libc::c_char);
                fflush(stderr);
                exit(1 as libc::c_int);
            }
            if parse_font_spec(
                token2,
                &mut ul_font,
                &mut ul_ptsize,
                0 as *mut InputEncoding,
            ) == 0
            {
                fprintf(
                    stderr,
                    b"%s:%s:%d: \0" as *const u8 as *const libc::c_char,
                    program,
                    buffer_ptr(&mut fname_0),
                    line,
                );
                fprintf(
                    stderr,
                    dcgettext(
                        0 as *const libc::c_char,
                        b"malformed font spec: %s\0" as *const u8 as *const libc::c_char,
                        5 as libc::c_int,
                    ),
                    token2,
                );
                fprintf(stderr, b"\n\0" as *const u8 as *const libc::c_char);
                fflush(stderr);
                exit(1 as libc::c_int);
            }
        } else if strcmp(token, b"UnderlayGray:\0" as *const u8 as *const libc::c_char)
            == 0 as libc::c_int
        {
            token2 = strtok(
                0 as *mut libc::c_char,
                b" \t\n\0" as *const u8 as *const libc::c_char,
            );
            if token2.is_null() {
                fprintf(
                    stderr,
                    b"%s:%s:%d: \0" as *const u8 as *const libc::c_char,
                    program,
                    buffer_ptr(&mut fname_0),
                    line,
                );
                fprintf(
                    stderr,
                    dcgettext(
                        0 as *const libc::c_char,
                        b"missing argument: %s\0" as *const u8 as *const libc::c_char,
                        5 as libc::c_int,
                    ),
                    token,
                );
                fprintf(stderr, b"\n\0" as *const u8 as *const libc::c_char);
                fflush(stderr);
                exit(1 as libc::c_int);
            }
            ul_gray = atof(token2);
        } else if strcmp(
            token,
            b"UnderlayPosition:\0" as *const u8 as *const libc::c_char,
        ) == 0 as libc::c_int
        {
            token2 = strtok(
                0 as *mut libc::c_char,
                b" \t\n\0" as *const u8 as *const libc::c_char,
            );
            if token2.is_null() {
                fprintf(
                    stderr,
                    b"%s:%s:%d: \0" as *const u8 as *const libc::c_char,
                    program,
                    buffer_ptr(&mut fname_0),
                    line,
                );
                fprintf(
                    stderr,
                    dcgettext(
                        0 as *const libc::c_char,
                        b"missing argument: %s\0" as *const u8 as *const libc::c_char,
                        5 as libc::c_int,
                    ),
                    token,
                );
                fprintf(stderr, b"\n\0" as *const u8 as *const libc::c_char);
                fflush(stderr);
                exit(1 as libc::c_int);
            }
            xfree(ul_position as *mut libc::c_void);
            ul_position = xstrdup(token2);
            ul_position_p = 1 as libc::c_int;
        } else if strcmp(token, b"UnderlayStyle:\0" as *const u8 as *const libc::c_char)
            == 0 as libc::c_int
        {
            token2 = strtok(
                0 as *mut libc::c_char,
                b" \t\n\0" as *const u8 as *const libc::c_char,
            );
            if token2.is_null() {
                fprintf(
                    stderr,
                    b"%s:%s:%d: \0" as *const u8 as *const libc::c_char,
                    program,
                    buffer_ptr(&mut fname_0),
                    line,
                );
                fprintf(
                    stderr,
                    dcgettext(
                        0 as *const libc::c_char,
                        b"missing argument: %s\0" as *const u8 as *const libc::c_char,
                        5 as libc::c_int,
                    ),
                    token,
                );
                fprintf(stderr, b"\n\0" as *const u8 as *const libc::c_char);
                fflush(stderr);
                exit(1 as libc::c_int);
            }
            xfree(ul_style_str as *mut libc::c_void);
            ul_style_str = xstrdup(token2);
        } else {
            fprintf(
                stderr,
                b"%s:%s:%d: \0" as *const u8 as *const libc::c_char,
                program,
                buffer_ptr(&mut fname_0),
                line,
            );
            fprintf(
                stderr,
                dcgettext(
                    0 as *const libc::c_char,
                    b"illegal option: %s\0" as *const u8 as *const libc::c_char,
                    5 as libc::c_int,
                ),
                token,
            );
            fprintf(stderr, b"\n\0" as *const u8 as *const libc::c_char);
            fflush(stderr);
            exit(1 as libc::c_int);
        }
    }
    buffer_uninit(&mut fname_0);
    return 1 as libc::c_int;
}
pub unsafe extern "C" fn add_media(
    mut name: *mut libc::c_char,
    mut w: libc::c_int,
    mut h: libc::c_int,
    mut llx: libc::c_int,
    mut lly: libc::c_int,
    mut urx: libc::c_int,
    mut ury: libc::c_int,
) {
    let mut entry: *mut MediaEntry = 0 as *mut MediaEntry;
    if quiet == 0 && verbose >= 2 as libc::c_int {
        fprintf(
            stderr,
            b"add_media: name=%s, w=%d, h=%d, llx=%d, lly=%d, urx=%d, ury=%d\n\0"
                as *const u8 as *const libc::c_char,
            name,
            w,
            h,
            llx,
            lly,
            urx,
            ury,
        );
    }
    entry = xcalloc(
        1 as libc::c_int as size_t,
        ::std::mem::size_of::<MediaEntry>() as libc::c_ulong,
    ) as *mut MediaEntry;
    (*entry)
        .name = xmalloc((strlen(name)).wrapping_add(1 as libc::c_int as libc::c_ulong))
        as *mut libc::c_char;
    strcpy((*entry).name, name);
    (*entry).w = w;
    (*entry).h = h;
    (*entry).llx = llx;
    (*entry).lly = lly;
    (*entry).urx = urx;
    (*entry).ury = ury;
    (*entry).next = media_names;
    media_names = entry;
}
pub unsafe extern "C" fn do_list_missing_characters(mut array: *mut libc::c_int) {
    let mut i: libc::c_int = 0;
    let mut count: libc::c_int = 0 as libc::c_int;
    i = 0 as libc::c_int;
    while i < 256 as libc::c_int {
        if *array.offset(i as isize) != 0 {
            fprintf(stderr, b"%3d \0" as *const u8 as *const libc::c_char, i);
            count += 1;
            count;
            if count % 15 as libc::c_int == 0 as libc::c_int {
                fprintf(stderr, b"\n\0" as *const u8 as *const libc::c_char);
            }
        }
        i += 1;
        i;
    }
    if count % 15 as libc::c_int != 0 as libc::c_int {
        fprintf(stderr, b"\n\0" as *const u8 as *const libc::c_char);
    }
}
pub unsafe extern "C" fn file_existsp(
    mut name: *mut libc::c_char,
    mut suffix: *mut libc::c_char,
) -> libc::c_int {
    let mut ctx: FileLookupCtx = FileLookupCtx {
        name: 0 as *mut libc::c_char,
        suffix: 0 as *mut libc::c_char,
        fullname: 0 as *mut Buffer,
    };
    let mut result: libc::c_int = 0;
    ctx.name = name;
    ctx
        .suffix = (if !suffix.is_null() {
        suffix as *const libc::c_char
    } else {
        b"\0" as *const u8 as *const libc::c_char
    }) as *mut libc::c_char;
    ctx.fullname = buffer_alloc();
    result = pathwalk(
        libpath,
        Some(
            file_lookup
                as unsafe extern "C" fn(
                    *mut libc::c_char,
                    *mut libc::c_void,
                ) -> libc::c_int,
        ),
        &mut ctx as *mut FileLookupCtx as *mut libc::c_void,
    );
    buffer_free(ctx.fullname);
    return result;
}
pub unsafe extern "C" fn paste_file(
    mut name: *mut libc::c_char,
    mut suffix: *mut libc::c_char,
) -> libc::c_int {
    let mut buf: [libc::c_char; 512] = [0; 512];
    let mut resources: [libc::c_char; 512] = [0; 512];
    let mut fp: *mut FILE = 0 as *mut FILE;
    let mut ctx: FileLookupCtx = FileLookupCtx {
        name: 0 as *mut libc::c_char,
        suffix: 0 as *mut libc::c_char,
        fullname: 0 as *mut Buffer,
    };
    let mut pending_comment: libc::c_int = 0 as libc::c_int;
    let mut line: libc::c_int = 0 as libc::c_int;
    ctx.name = name;
    ctx
        .suffix = (if !suffix.is_null() {
        suffix as *const libc::c_char
    } else {
        b"\0" as *const u8 as *const libc::c_char
    }) as *mut libc::c_char;
    ctx.fullname = buffer_alloc();
    if pathwalk(
        libpath,
        Some(
            file_lookup
                as unsafe extern "C" fn(
                    *mut libc::c_char,
                    *mut libc::c_void,
                ) -> libc::c_int,
        ),
        &mut ctx as *mut FileLookupCtx as *mut libc::c_void,
    ) == 0
    {
        buffer_free(ctx.fullname);
        return 0 as libc::c_int;
    }
    fp = fopen(buffer_ptr(ctx.fullname), b"r\0" as *const u8 as *const libc::c_char);
    if fp.is_null() {
        buffer_free(ctx.fullname);
        return 0 as libc::c_int;
    }
    while !(fgets(
        buf.as_mut_ptr(),
        ::std::mem::size_of::<[libc::c_char; 512]>() as libc::c_ulong as libc::c_int,
        fp,
    ))
        .is_null()
    {
        line += 1;
        line;
        if strncmp(
            buf.as_mut_ptr(),
            b"% -- code follows this line --\0" as *const u8 as *const libc::c_char,
            strlen(
                b"% -- code follows this line --\0" as *const u8 as *const libc::c_char,
            ),
        ) == 0 as libc::c_int
        {
            break;
        }
    }
    let mut cp: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut cp2: *mut libc::c_char = 0 as *mut libc::c_char;
    while !(fgets(
        buf.as_mut_ptr(),
        ::std::mem::size_of::<[libc::c_char; 512]>() as libc::c_ulong as libc::c_int,
        fp,
    ))
        .is_null()
    {
        line += 1;
        line;
        if strncmp(
            buf.as_mut_ptr(),
            b"%%DocumentNeededResources:\0" as *const u8 as *const libc::c_char,
            strlen(b"%%DocumentNeededResources:\0" as *const u8 as *const libc::c_char),
        ) == 0 as libc::c_int
        {
            cp = 0 as *mut libc::c_char;
            cp2 = 0 as *mut libc::c_char;
            strcpy(
                resources.as_mut_ptr(),
                buf
                    .as_mut_ptr()
                    .offset(
                        strlen(
                            b"%%DocumentNeededResources:\0" as *const u8
                                as *const libc::c_char,
                        ) as isize,
                    ),
            );
            pending_comment = 1 as libc::c_int;
        } else if pending_comment != 0
            && strncmp(
                buf.as_mut_ptr(),
                b"%%+\0" as *const u8 as *const libc::c_char,
                strlen(b"%%+\0" as *const u8 as *const libc::c_char),
            ) == 0 as libc::c_int
        {
            strcpy(
                resources.as_mut_ptr(),
                buf
                    .as_mut_ptr()
                    .offset(
                        strlen(b"%%+\0" as *const u8 as *const libc::c_char) as isize,
                    ),
            );
        } else {
            pending_comment = 0 as libc::c_int;
            if strncmp(
                buf.as_mut_ptr(),
                b"%Format:\0" as *const u8 as *const libc::c_char,
                strlen(b"%Format:\0" as *const u8 as *const libc::c_char),
            ) == 0 as libc::c_int
            {
                let mut i: libc::c_int = 0;
                let mut j: libc::c_int = 0;
                let mut name_0: [libc::c_char; 256] = [0; 256];
                let mut cp_0: *mut libc::c_char = 0 as *mut libc::c_char;
                let mut cp2_0: *mut libc::c_char = 0 as *mut libc::c_char;
                *__errno_location() = 0 as libc::c_int;
                i = strlen(b"%Format:\0" as *const u8 as *const libc::c_char)
                    as libc::c_int;
                while buf[i as usize] as libc::c_int != 0
                    && *(*__ctype_b_loc())
                        .offset(buf[i as usize] as libc::c_int as isize) as libc::c_int
                        & _ISspace as libc::c_int as libc::c_ushort as libc::c_int != 0
                {
                    i += 1;
                    i;
                }
                if buf[i as usize] == 0 {
                    fprintf(
                        stderr,
                        b"%s: \0" as *const u8 as *const libc::c_char,
                        program,
                    );
                    fprintf(
                        stderr,
                        dcgettext(
                            0 as *const libc::c_char,
                            b"%s:%d: %%Format: no name\0" as *const u8
                                as *const libc::c_char,
                            5 as libc::c_int,
                        ),
                        buffer_ptr(ctx.fullname),
                        line,
                    );
                    fprintf(stderr, b"\n\0" as *const u8 as *const libc::c_char);
                    fflush(stderr);
                    exit(1 as libc::c_int);
                }
                j = 0 as libc::c_int;
                while (j as libc::c_ulong)
                    < (::std::mem::size_of::<[libc::c_char; 256]>() as libc::c_ulong)
                        .wrapping_sub(1 as libc::c_int as libc::c_ulong)
                    && buf[i as usize] as libc::c_int != 0
                    && *(*__ctype_b_loc())
                        .offset(buf[i as usize] as libc::c_int as isize) as libc::c_int
                        & _ISspace as libc::c_int as libc::c_ushort as libc::c_int == 0
                {
                    let fresh0 = j;
                    j = j + 1;
                    name_0[fresh0 as usize] = buf[i as usize];
                    i += 1;
                    i;
                }
                name_0[j as usize] = '\0' as i32 as libc::c_char;
                if j as libc::c_ulong
                    >= (::std::mem::size_of::<[libc::c_char; 256]>() as libc::c_ulong)
                        .wrapping_sub(1 as libc::c_int as libc::c_ulong)
                {
                    fprintf(
                        stderr,
                        b"%s: \0" as *const u8 as *const libc::c_char,
                        program,
                    );
                    fprintf(
                        stderr,
                        dcgettext(
                            0 as *const libc::c_char,
                            b"%s:%d: %%Format: too long name, maxlen=%d\0" as *const u8
                                as *const libc::c_char,
                            5 as libc::c_int,
                        ),
                        buffer_ptr(ctx.fullname),
                        line,
                        (::std::mem::size_of::<[libc::c_char; 256]>() as libc::c_ulong)
                            .wrapping_sub(1 as libc::c_int as libc::c_ulong),
                    );
                    fprintf(stderr, b"\n\0" as *const u8 as *const libc::c_char);
                    fflush(stderr);
                    exit(1 as libc::c_int);
                }
                while buf[i as usize] as libc::c_int != 0
                    && *(*__ctype_b_loc())
                        .offset(buf[i as usize] as libc::c_int as isize) as libc::c_int
                        & _ISspace as libc::c_int as libc::c_ushort as libc::c_int != 0
                {
                    i += 1;
                    i;
                }
                j = strlen(buf.as_mut_ptr()) as libc::c_int;
                j -= 1;
                j;
                while *(*__ctype_b_loc()).offset(buf[j as usize] as libc::c_int as isize)
                    as libc::c_int
                    & _ISspace as libc::c_int as libc::c_ushort as libc::c_int != 0
                    && j > i
                {
                    j -= 1;
                    j;
                }
                j += 1;
                j;
                if quiet == 0 && verbose >= 2 as libc::c_int {
                    fprintf(
                        stderr,
                        b"%%Format: %s %.*s\n\0" as *const u8 as *const libc::c_char,
                        name_0.as_mut_ptr(),
                        j - i,
                        buf.as_mut_ptr().offset(i as isize),
                    );
                }
                cp_0 = xmalloc((j - i + 1 as libc::c_int) as size_t)
                    as *mut libc::c_char;
                memcpy(
                    cp_0 as *mut libc::c_void,
                    buf.as_mut_ptr().offset(i as isize) as *const libc::c_void,
                    (j - i) as libc::c_ulong,
                );
                *cp_0.offset((j - i) as isize) = '\0' as i32 as libc::c_char;
                strhash_put(
                    user_strings,
                    name_0.as_mut_ptr(),
                    (strlen(name_0.as_mut_ptr()))
                        .wrapping_add(1 as libc::c_int as libc::c_ulong) as libc::c_int,
                    cp_0 as *mut libc::c_void,
                    &mut cp2_0 as *mut *mut libc::c_char as *mut *mut libc::c_void,
                );
                if !cp2_0.is_null() {
                    fprintf(
                        stderr,
                        b"%s: \0" as *const u8 as *const libc::c_char,
                        program,
                    );
                    fprintf(
                        stderr,
                        dcgettext(
                            0 as *const libc::c_char,
                            b"%s:%d: %%Format: name \"%s\" is already defined\0"
                                as *const u8 as *const libc::c_char,
                            5 as libc::c_int,
                        ),
                        buffer_ptr(ctx.fullname),
                        line,
                        name_0.as_mut_ptr(),
                    );
                    fprintf(stderr, b"\n\0" as *const u8 as *const libc::c_char);
                    fflush(stderr);
                    exit(1 as libc::c_int);
                }
                continue;
            } else if strncmp(
                buf.as_mut_ptr(),
                b"%HeaderHeight:\0" as *const u8 as *const libc::c_char,
                strlen(b"%HeaderHeight:\0" as *const u8 as *const libc::c_char),
            ) == 0 as libc::c_int
            {
                let mut i_0: libc::c_int = 0;
                i_0 = strlen(b"%HeaderHeight:\0" as *const u8 as *const libc::c_char)
                    as libc::c_int;
                while buf[i_0 as usize] as libc::c_int != 0
                    && *(*__ctype_b_loc())
                        .offset(buf[i_0 as usize] as libc::c_int as isize) as libc::c_int
                        & _ISspace as libc::c_int as libc::c_ushort as libc::c_int == 0
                {
                    i_0 += 1;
                    i_0;
                }
                if buf[i_0 as usize] == 0 {
                    fprintf(
                        stderr,
                        b"%s: \0" as *const u8 as *const libc::c_char,
                        program,
                    );
                    fprintf(
                        stderr,
                        dcgettext(
                            0 as *const libc::c_char,
                            b"%s:%d: %%HeaderHeight: no argument\0" as *const u8
                                as *const libc::c_char,
                            5 as libc::c_int,
                        ),
                        buffer_ptr(ctx.fullname),
                        line,
                    );
                    fprintf(stderr, b"\n\0" as *const u8 as *const libc::c_char);
                    fflush(stderr);
                    exit(1 as libc::c_int);
                }
                d_header_h = atoi(buf.as_mut_ptr().offset(i_0 as isize));
                if quiet == 0 && verbose >= 2 as libc::c_int {
                    fprintf(
                        stderr,
                        b"%%HeaderHeight: %d\n\0" as *const u8 as *const libc::c_char,
                        d_header_h,
                    );
                }
                continue;
            } else if strncmp(
                buf.as_mut_ptr(),
                b"%FooterHeight:\0" as *const u8 as *const libc::c_char,
                strlen(b"%FooterHeight:\0" as *const u8 as *const libc::c_char),
            ) == 0 as libc::c_int
            {
                let mut i_1: libc::c_int = 0;
                i_1 = strlen(b"%FooterHeight:\0" as *const u8 as *const libc::c_char)
                    as libc::c_int;
                while buf[i_1 as usize] as libc::c_int != 0
                    && *(*__ctype_b_loc())
                        .offset(buf[i_1 as usize] as libc::c_int as isize) as libc::c_int
                        & _ISspace as libc::c_int as libc::c_ushort as libc::c_int == 0
                {
                    i_1 += 1;
                    i_1;
                }
                if buf[i_1 as usize] == 0 {
                    fprintf(
                        stderr,
                        b"%s: \0" as *const u8 as *const libc::c_char,
                        program,
                    );
                    fprintf(
                        stderr,
                        dcgettext(
                            0 as *const libc::c_char,
                            b"%s:%d: %%FooterHeight: no argument\0" as *const u8
                                as *const libc::c_char,
                            5 as libc::c_int,
                        ),
                        buffer_ptr(ctx.fullname),
                        line,
                    );
                    fprintf(stderr, b"\n\0" as *const u8 as *const libc::c_char);
                    fflush(stderr);
                    exit(1 as libc::c_int);
                }
                d_footer_h = atoi(buf.as_mut_ptr().offset(i_1 as isize));
                if quiet == 0 && verbose >= 2 as libc::c_int {
                    fprintf(
                        stderr,
                        b"%%FooterHeight: %d\n\0" as *const u8 as *const libc::c_char,
                        d_footer_h,
                    );
                }
                continue;
            } else {
                fputs(buf.as_mut_ptr(), ofp);
                continue;
            }
        }
        cp = strtok(
            resources.as_mut_ptr(),
            b" \t\n\0" as *const u8 as *const libc::c_char,
        );
        if !cp.is_null() {
            if strcmp(cp, b"font\0" as *const u8 as *const libc::c_char)
                == 0 as libc::c_int
            {
                cp = strtok(
                    0 as *mut libc::c_char,
                    b" \t\n\0" as *const u8 as *const libc::c_char,
                );
                while !cp.is_null() {
                    if strhash_get(
                        res_fonts,
                        cp,
                        (strlen(cp)).wrapping_add(1 as libc::c_int as libc::c_ulong)
                            as libc::c_int,
                        &mut cp2 as *mut *mut libc::c_char as *mut *mut libc::c_void,
                    ) == 0
                    {
                        fprintf(
                            ofp,
                            b"%%%%IncludeResource: font %s\n\0" as *const u8
                                as *const libc::c_char,
                            cp,
                        );
                        strhash_put(
                            res_fonts,
                            cp,
                            (strlen(cp)).wrapping_add(1 as libc::c_int as libc::c_ulong)
                                as libc::c_int,
                            0 as *mut libc::c_void,
                            0 as *mut *mut libc::c_void,
                        );
                    }
                    cp = strtok(
                        0 as *mut libc::c_char,
                        b" \t\n\0" as *const u8 as *const libc::c_char,
                    );
                }
            }
        }
    }
    fclose(fp);
    buffer_free(ctx.fullname);
    return 1 as libc::c_int;
}
pub unsafe extern "C" fn parse_font_spec(
    mut spec_a: *mut libc::c_char,
    mut name_return: *mut *mut libc::c_char,
    mut size_return: *mut FontPoint,
    mut encoding_return: *mut InputEncoding,
) -> libc::c_int {
    let mut i: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    let mut cp: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut cp2: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut spec: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut encp: *mut libc::c_char = 0 as *mut libc::c_char;
    spec = xstrdup(spec_a);
    encp = strrchr(spec, ':' as i32);
    if !encp.is_null() {
        *encp = '\0' as i32 as libc::c_char;
        encp = encp.offset(1);
        encp;
    }
    cp = strchr(spec, '@' as i32);
    if !cp.is_null() {
        i = cp.offset_from(spec) as libc::c_long as libc::c_int;
        if *cp.offset(1 as libc::c_int as isize) as libc::c_int == '\0' as i32 {
            xfree(spec as *mut libc::c_void);
            return 0 as libc::c_int;
        }
        cp = cp.offset(1);
        cp;
    } else {
        i = (strlen(spec)).wrapping_sub(1 as libc::c_int as libc::c_ulong)
            as libc::c_int;
        if i <= 0 as libc::c_int
            || !('0' as i32 <= *spec.offset(i as isize) as libc::c_int
                && *spec.offset(i as isize) as libc::c_int <= '9' as i32
                || *spec.offset(i as isize) as libc::c_int == '.' as i32
                || *spec.offset(i as isize) as libc::c_int == '-' as i32
                || *spec.offset(i as isize) as libc::c_int == '+' as i32)
        {
            xfree(spec as *mut libc::c_void);
            return 0 as libc::c_int;
        }
        i -= 1;
        i;
        while i >= 0 as libc::c_int
            && ('0' as i32 <= *spec.offset(i as isize) as libc::c_int
                && *spec.offset(i as isize) as libc::c_int <= '9' as i32
                || *spec.offset(i as isize) as libc::c_int == '.' as i32
                || *spec.offset(i as isize) as libc::c_int == '-' as i32
                || *spec.offset(i as isize) as libc::c_int == '+' as i32)
        {
            i -= 1;
            i;
        }
        if i < 0 as libc::c_int {
            xfree(spec as *mut libc::c_void);
            return 0 as libc::c_int;
        }
        if *spec.offset(i as isize) as libc::c_int == '/' as i32 {
            i -= 1;
            i;
            while i >= 0 as libc::c_int
                && ('0' as i32 <= *spec.offset(i as isize) as libc::c_int
                    && *spec.offset(i as isize) as libc::c_int <= '9' as i32
                    || *spec.offset(i as isize) as libc::c_int == '.' as i32
                    || *spec.offset(i as isize) as libc::c_int == '-' as i32
                    || *spec.offset(i as isize) as libc::c_int == '+' as i32)
            {
                i -= 1;
                i;
            }
            if i < 0 as libc::c_int {
                xfree(spec as *mut libc::c_void);
                return 0 as libc::c_int;
            }
        }
        i += 1;
        i;
        cp = spec.offset(i as isize);
    }
    cp2 = strchr(cp, '/' as i32);
    if !cp2.is_null() {
        let fresh1 = cp2;
        cp2 = cp2.offset(1);
        *fresh1 = '\0' as i32 as libc::c_char;
        (*size_return).w = atof(cp);
        (*size_return).h = atof(cp2);
    } else {
        (*size_return).h = atof(cp);
        (*size_return).w = (*size_return).h;
    }
    *name_return = xcalloc(1 as libc::c_int as size_t, (i + 1 as libc::c_int) as size_t)
        as *mut libc::c_char;
    strncpy(*name_return, spec, i as libc::c_ulong);
    if !encp.is_null() {
        let mut found: libc::c_int = 0 as libc::c_int;
        if encoding_return.is_null() {
            xfree(spec as *mut libc::c_void);
            return 0 as libc::c_int;
        }
        i = 0 as libc::c_int;
        while found == 0
            && !((*encodings.as_mut_ptr().offset(i as isize))
                .names[0 as libc::c_int as usize])
                .is_null()
        {
            j = 0 as libc::c_int;
            while j < 3 as libc::c_int {
                if !((*encodings.as_mut_ptr().offset(i as isize)).names[j as usize])
                    .is_null()
                    && strcmp(
                        (*encodings.as_mut_ptr().offset(i as isize)).names[j as usize],
                        encp,
                    ) == 0 as libc::c_int
                {
                    *encoding_return = (*encodings.as_mut_ptr().offset(i as isize))
                        .encoding;
                    encp = (*encodings.as_mut_ptr().offset(i as isize))
                        .names[0 as libc::c_int as usize];
                    found = 1 as libc::c_int;
                    break;
                } else {
                    j += 1;
                    j;
                }
            }
            i += 1;
            i;
        }
        if found == 0 {
            xfree(spec as *mut libc::c_void);
            return 0 as libc::c_int;
        }
    } else {
        encp = encoding_name;
        if !encoding_return.is_null() {
            *encoding_return = encoding;
        }
    }
    xfree(spec as *mut libc::c_void);
    if quiet == 0 && verbose >= 2 as libc::c_int {
        fprintf(
            stderr,
            b"parse_font_spec(): name=%.*s, size=%g/%g, encoding=%s\n\0" as *const u8
                as *const libc::c_char,
            i,
            *name_return,
            (*size_return).w,
            (*size_return).h,
            encp,
        );
    }
    if (*size_return).w < 0.0f64 && (*size_return).h < 0.0f64 {
        if quiet == 0 && verbose >= 0 as libc::c_int {
            fprintf(
                stderr,
                dcgettext(
                    0 as *const libc::c_char,
                    b"%s: warning: font size is negative\n\0" as *const u8
                        as *const libc::c_char,
                    5 as libc::c_int,
                ),
                program,
            );
        }
    } else if (*size_return).w < 0.0f64 {
        if quiet == 0 && verbose >= 0 as libc::c_int {
            fprintf(
                stderr,
                dcgettext(
                    0 as *const libc::c_char,
                    b"%s: warning: font width is negative\n\0" as *const u8
                        as *const libc::c_char,
                    5 as libc::c_int,
                ),
                program,
            );
        }
    } else if (*size_return).h < 0.0f64 {
        if quiet == 0 && verbose >= 0 as libc::c_int {
            fprintf(
                stderr,
                dcgettext(
                    0 as *const libc::c_char,
                    b"%s: warning: font height is negative\n\0" as *const u8
                        as *const libc::c_char,
                    5 as libc::c_int,
                ),
                program,
            );
        }
    }
    return 1 as libc::c_int;
}
pub unsafe extern "C" fn read_font_info() {
    let mut font_info: *mut CachedFontInfo = 0 as *mut CachedFontInfo;
    let mut font: AFMFont = 0 as *mut afm_font_st;
    let mut font_info_cached: libc::c_int = 1 as libc::c_int;
    let mut font_cached: libc::c_int = 1 as libc::c_int;
    let mut i: libc::c_int = 0;
    let mut enc_flags: libc::c_uint = 0 as libc::c_int as libc::c_uint;
    let mut buf: [libc::c_char; 256] = [0; 256];
    let mut fkey: Buffer = Buffer {
        data: 0 as *mut libc::c_char,
        allocated: 0,
        len: 0,
    };
    if quiet == 0 && verbose >= 2 as libc::c_int {
        fprintf(
            stderr,
            dcgettext(
                0 as *const libc::c_char,
                b"reading AFM info for font \"%s\"\n\0" as *const u8
                    as *const libc::c_char,
                5 as libc::c_int,
            ),
            Fname,
        );
    }
    if accept_composites != 0 {
        enc_flags = 0x1 as libc::c_int as libc::c_uint;
    }
    buffer_init(&mut fkey);
    buffer_append(&mut fkey, Fname);
    sprintf(
        buf.as_mut_ptr(),
        b"@%f:%d\0" as *const u8 as *const libc::c_char,
        Fpt.w,
        encoding as libc::c_uint,
    );
    buffer_append(&mut fkey, buf.as_mut_ptr());
    if strhash_get(
        afm_info_cache,
        buffer_ptr(&mut fkey),
        strlen(buffer_ptr(&mut fkey)) as libc::c_int,
        &mut font_info as *mut *mut CachedFontInfo as *mut *mut libc::c_void,
    ) == 0
    {
        let mut error: AFMError = 0;
        if strhash_get(
            afm_cache,
            Fname,
            strlen(Fname) as libc::c_int,
            &mut font as *mut AFMFont as *mut *mut libc::c_void,
        ) == 0
        {
            error = afm_open_font(
                afm,
                0x1 as libc::c_int as libc::c_uint,
                Fname,
                &mut font,
            );
            if error != 0 as libc::c_int as libc::c_uint {
                if strncmp(
                    Fname,
                    b"Courier\0" as *const u8 as *const libc::c_char,
                    strlen(b"Courier\0" as *const u8 as *const libc::c_char),
                ) != 0 as libc::c_int
                {
                    if quiet == 0 && verbose >= 0 as libc::c_int {
                        fprintf(
                            stderr,
                            dcgettext(
                                0 as *const libc::c_char,
                                b"couldn't open AFM file for font \"%s\", using default\n\0"
                                    as *const u8 as *const libc::c_char,
                                5 as libc::c_int,
                            ),
                            Fname,
                        );
                    }
                }
                error = afm_open_default_font(afm, &mut font);
                if error != 0 as libc::c_int as libc::c_uint {
                    afm_error_to_string(error, buf.as_mut_ptr());
                    fprintf(
                        stderr,
                        b"%s: \0" as *const u8 as *const libc::c_char,
                        program,
                    );
                    fprintf(
                        stderr,
                        dcgettext(
                            0 as *const libc::c_char,
                            b"couldn't open AFM file for the default font: %s\0"
                                as *const u8 as *const libc::c_char,
                            5 as libc::c_int,
                        ),
                        buf.as_mut_ptr(),
                    );
                    fprintf(stderr, b"\n\0" as *const u8 as *const libc::c_char);
                    fflush(stderr);
                    exit(1 as libc::c_int);
                }
            }
            match encoding as libc::c_uint {
                0 => {
                    afm_font_encoding(font, AFM_ENCODING_ISO_8859_1, enc_flags);
                }
                1 => {
                    afm_font_encoding(font, AFM_ENCODING_ISO_8859_2, enc_flags);
                }
                2 => {
                    afm_font_encoding(font, AFM_ENCODING_ISO_8859_3, enc_flags);
                }
                3 => {
                    afm_font_encoding(font, AFM_ENCODING_ISO_8859_4, enc_flags);
                }
                4 => {
                    afm_font_encoding(font, AFM_ENCODING_ISO_8859_5, enc_flags);
                }
                5 => {
                    afm_font_encoding(font, AFM_ENCODING_ISO_8859_7, enc_flags);
                }
                6 => {
                    afm_font_encoding(font, AFM_ENCODING_ISO_8859_9, enc_flags);
                }
                7 => {
                    afm_font_encoding(font, AFM_ENCODING_ISO_8859_10, enc_flags);
                }
                8 => {
                    afm_font_encoding(font, AFM_ENCODING_ASCII, enc_flags);
                }
                9 => {
                    afm_font_encoding(font, AFM_ENCODING_ASCII, enc_flags);
                    i = 0 as libc::c_int;
                    while !(enc_7bit_ascii_fise[i as usize].name).is_null() {
                        afm_font_encode(
                            font,
                            enc_7bit_ascii_fise[i as usize].code as libc::c_uchar,
                            enc_7bit_ascii_fise[i as usize].name,
                            enc_flags,
                        );
                        i += 1;
                        i;
                    }
                }
                10 => {
                    afm_font_encoding(font, AFM_ENCODING_ASCII, enc_flags);
                    i = 0 as libc::c_int;
                    while !(enc_7bit_ascii_dkno[i as usize].name).is_null() {
                        afm_font_encode(
                            font,
                            enc_7bit_ascii_dkno[i as usize].code as libc::c_uchar,
                            enc_7bit_ascii_dkno[i as usize].name,
                            enc_flags,
                        );
                        i += 1;
                        i;
                    }
                }
                11 => {
                    afm_font_encoding(font, AFM_ENCODING_IBMPC, enc_flags);
                }
                12 => {
                    afm_font_encoding(font, AFM_ENCODING_MAC, enc_flags);
                }
                13 => {
                    afm_font_encoding(font, AFM_ENCODING_VMS, enc_flags);
                }
                14 => {
                    afm_font_encoding(font, AFM_ENCODING_HP8, enc_flags);
                }
                15 => {
                    afm_font_encoding(font, AFM_ENCODING_KOI8, enc_flags);
                }
                16 | _ => {}
            }
            if strhash_put(
                afm_cache,
                Fname,
                strlen(Fname) as libc::c_int,
                font as *mut libc::c_void,
                0 as *mut *mut libc::c_void,
            ) == 0
            {
                font_cached = 0 as libc::c_int;
            }
        }
        font_info = xcalloc(
            1 as libc::c_int as size_t,
            ::std::mem::size_of::<CachedFontInfo>() as libc::c_ulong,
        ) as *mut CachedFontInfo;
        i = 0 as libc::c_int;
        while i < 256 as libc::c_int {
            let mut w0x: AFMNumber = 0.;
            let mut w0y: AFMNumber = 0.;
            afm_font_charwidth(font, Fpt.w, i as libc::c_char, &mut w0x, &mut w0y);
            (*font_info).font_widths[i as usize] = w0x;
            if ((*font).encoding[i as usize]).is_null() {
                (*font_info).font_ctype[i as usize] = ' ' as i32 as libc::c_char;
            } else if (*font).encoding[i as usize]
                == 1 as libc::c_int as *mut libc::c_void
                    as *mut AFMIndividualCharacterMetrics
            {
                (*font_info).font_ctype[i as usize] = '.' as i32 as libc::c_char;
            } else {
                (*font_info).font_ctype[i as usize] = '*' as i32 as libc::c_char;
            }
            i += 1;
            i;
        }
        (*font_info)
            .font_is_fixed = (*font)
            .writing_direction_metrics[0 as libc::c_int as usize]
            .IsFixedPitch;
        (*font_info).font_bbox_lly = (*font).global_info.FontBBox_lly;
        if font_cached == 0 {
            afm_close_font(font);
        }
        if strhash_put(
            afm_info_cache,
            buffer_ptr(&mut fkey),
            strlen(buffer_ptr(&mut fkey)) as libc::c_int,
            font_info as *mut libc::c_void,
            0 as *mut *mut libc::c_void,
        ) == 0
        {
            font_info_cached = 0 as libc::c_int;
        }
    }
    memcpy(
        font_widths.as_mut_ptr() as *mut libc::c_void,
        ((*font_info).font_widths).as_mut_ptr() as *const libc::c_void,
        (256 as libc::c_int as libc::c_ulong)
            .wrapping_mul(::std::mem::size_of::<libc::c_double>() as libc::c_ulong),
    );
    memcpy(
        font_ctype.as_mut_ptr() as *mut libc::c_void,
        ((*font_info).font_ctype).as_mut_ptr() as *const libc::c_void,
        256 as libc::c_int as libc::c_ulong,
    );
    font_is_fixed = (*font_info).font_is_fixed as libc::c_int;
    font_bbox_lly = (*font_info).font_bbox_lly;
    if font_info_cached == 0 {
        xfree(font_info as *mut libc::c_void);
    }
    buffer_uninit(&mut fkey);
}
pub unsafe extern "C" fn download_font(mut name: *mut libc::c_char) {
    let mut error: AFMError = 0;
    let mut prefix: *const libc::c_char = 0 as *const libc::c_char;
    let mut stat_st: stat = stat {
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
    let mut fname_0: Buffer = Buffer {
        data: 0 as *mut libc::c_char,
        allocated: 0,
        len: 0,
    };
    let mut buf: [libc::c_uchar; 4096] = [0; 4096];
    let mut fp: *mut FILE = 0 as *mut FILE;
    let mut i: libc::c_int = 0;
    let mut cp: *mut libc::c_char = 0 as *mut libc::c_char;
    error = afm_font_prefix(afm, name, &mut prefix);
    if error != 0 as libc::c_int as libc::c_uint {
        return;
    }
    buffer_init(&mut fname_0);
    buffer_append(&mut fname_0, prefix);
    buffer_append(&mut fname_0, b".pfa\0" as *const u8 as *const libc::c_char);
    if stat(buffer_ptr(&mut fname_0), &mut stat_st) != 0 as libc::c_int {
        buffer_clear(&mut fname_0);
        buffer_append(&mut fname_0, prefix);
        buffer_append(&mut fname_0, b".pfb\0" as *const u8 as *const libc::c_char);
        if stat(buffer_ptr(&mut fname_0), &mut stat_st) != 0 as libc::c_int {
            buffer_uninit(&mut fname_0);
            return;
        }
    }
    if quiet == 0 && verbose >= 1 as libc::c_int {
        fprintf(
            stderr,
            dcgettext(
                0 as *const libc::c_char,
                b"downloading font \"%s\"\n\0" as *const u8 as *const libc::c_char,
                5 as libc::c_int,
            ),
            name,
        );
    }
    fp = fopen(buffer_ptr(&mut fname_0), b"rb\0" as *const u8 as *const libc::c_char);
    if fp.is_null() {
        if quiet == 0 && verbose >= 0 as libc::c_int {
            fprintf(
                stderr,
                dcgettext(
                    0 as *const libc::c_char,
                    b"couldn't open font description file \"%s\": %s\n\0" as *const u8
                        as *const libc::c_char,
                    5 as libc::c_int,
                ),
                buffer_ptr(&mut fname_0),
                strerror(*__errno_location()),
            );
        }
        buffer_uninit(&mut fname_0);
        return;
    }
    buffer_uninit(&mut fname_0);
    fprintf(
        ofp,
        b"%%%%BeginResource: font %s\n\0" as *const u8 as *const libc::c_char,
        name,
    );
    i = fgetc(fp);
    if !(i == -(1 as libc::c_int)) {
        if i == 128 as libc::c_int {
            let mut done: libc::c_int = 0 as libc::c_int;
            let mut chunk: libc::c_uint = 0;
            let mut to_read: libc::c_uint = 0;
            let mut last_was_cr: libc::c_int = 0;
            let mut j: libc::c_int = 0;
            ungetc(i, fp);
            while done == 0 {
                i = fread(
                    buf.as_mut_ptr() as *mut libc::c_void,
                    1 as libc::c_int as libc::c_ulong,
                    6 as libc::c_int as libc::c_ulong,
                    fp,
                ) as libc::c_int;
                if i != 6 as libc::c_int {
                    break;
                }
                chunk = (buf[2 as libc::c_int as usize] as libc::c_int
                    | (buf[3 as libc::c_int as usize] as libc::c_int) << 8 as libc::c_int
                    | (buf[4 as libc::c_int as usize] as libc::c_int)
                        << 16 as libc::c_int
                    | (buf[5 as libc::c_int as usize] as libc::c_int)
                        << 24 as libc::c_int) as libc::c_uint;
                match buf[1 as libc::c_int as usize] as libc::c_int {
                    1 => {
                        last_was_cr = 0 as libc::c_int;
                        while chunk > 0 as libc::c_int as libc::c_uint {
                            to_read = (if (::std::mem::size_of::<[libc::c_uchar; 4096]>()
                                as libc::c_ulong) < chunk as libc::c_ulong
                            {
                                ::std::mem::size_of::<[libc::c_uchar; 4096]>()
                                    as libc::c_ulong
                            } else {
                                chunk as libc::c_ulong
                            }) as libc::c_uint;
                            i = fread(
                                buf.as_mut_ptr() as *mut libc::c_void,
                                1 as libc::c_int as libc::c_ulong,
                                to_read as libc::c_ulong,
                                fp,
                            ) as libc::c_int;
                            if i == 0 as libc::c_int {
                                done = 1 as libc::c_int;
                                break;
                            } else {
                                j = 0 as libc::c_int;
                                while j < i {
                                    if j == 0 as libc::c_int && last_was_cr != 0
                                        && buf[0 as libc::c_int as usize] as libc::c_int
                                            != '\n' as i32
                                    {
                                        fputc('\n' as i32, ofp);
                                        fputc(buf[0 as libc::c_int as usize] as libc::c_int, ofp);
                                    } else if buf[j as usize] as libc::c_int == '\r' as i32
                                        && (j + 1 as libc::c_int) < i
                                        && buf[(j + 1 as libc::c_int) as usize] as libc::c_int
                                            != '\n' as i32
                                    {
                                        fputc('\n' as i32, ofp);
                                    } else if buf[j as usize] as libc::c_int != '\r' as i32 {
                                        fputc(buf[j as usize] as libc::c_int, ofp);
                                    }
                                    j += 1;
                                    j;
                                }
                                chunk = chunk.wrapping_sub(i as libc::c_uint);
                                last_was_cr = (buf[(i - 1 as libc::c_int) as usize]
                                    as libc::c_int == '\r' as i32) as libc::c_int;
                            }
                        }
                    }
                    2 => {
                        while chunk > 0 as libc::c_int as libc::c_uint {
                            to_read = (if (::std::mem::size_of::<[libc::c_uchar; 4096]>()
                                as libc::c_ulong) < chunk as libc::c_ulong
                            {
                                ::std::mem::size_of::<[libc::c_uchar; 4096]>()
                                    as libc::c_ulong
                            } else {
                                chunk as libc::c_ulong
                            }) as libc::c_uint;
                            i = fread(
                                buf.as_mut_ptr() as *mut libc::c_void,
                                1 as libc::c_int as libc::c_ulong,
                                to_read as libc::c_ulong,
                                fp,
                            ) as libc::c_int;
                            if i == 0 as libc::c_int {
                                done = 1 as libc::c_int;
                                break;
                            } else {
                                j = 0 as libc::c_int;
                                while j < i {
                                    fprintf(
                                        ofp,
                                        b"%02X\0" as *const u8 as *const libc::c_char,
                                        buf[j as usize] as libc::c_int,
                                    );
                                    if (j + 1 as libc::c_int) % 32 as libc::c_int
                                        == 0 as libc::c_int
                                    {
                                        fprintf(ofp, b"\n\0" as *const u8 as *const libc::c_char);
                                    }
                                    j += 1;
                                    j;
                                }
                                chunk = chunk.wrapping_sub(i as libc::c_uint);
                            }
                        }
                    }
                    3 => {
                        done = 1 as libc::c_int;
                    }
                    _ => {}
                }
                fprintf(ofp, b"\n\0" as *const u8 as *const libc::c_char);
            }
        } else {
            ungetc(i, fp);
            loop {
                i = fread(
                    buf.as_mut_ptr() as *mut libc::c_void,
                    1 as libc::c_int as libc::c_ulong,
                    ::std::mem::size_of::<[libc::c_uchar; 4096]>() as libc::c_ulong,
                    fp,
                ) as libc::c_int;
                if !(i != 0 as libc::c_int) {
                    break;
                }
                fwrite(
                    buf.as_mut_ptr() as *const libc::c_void,
                    1 as libc::c_int as libc::c_ulong,
                    i as libc::c_ulong,
                    ofp,
                );
            }
        }
    }
    fprintf(ofp, b"%%%%EndResource\n\0" as *const u8 as *const libc::c_char);
    strhash_delete(
        res_fonts,
        name,
        (strlen(name)).wrapping_add(1 as libc::c_int as libc::c_ulong) as libc::c_int,
        &mut cp as *mut *mut libc::c_char as *mut *mut libc::c_void,
    );
    fclose(fp);
}
pub unsafe extern "C" fn escape_string(
    mut string: *mut libc::c_char,
) -> *mut libc::c_char {
    let mut i: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    let mut len: libc::c_int = 0;
    let mut cp: *mut libc::c_char = 0 as *mut libc::c_char;
    len = 0 as libc::c_int;
    i = 0 as libc::c_int;
    while *string.offset(i as isize) != 0 {
        match *string.offset(i as isize) as libc::c_int {
            40 | 41 | 92 => {
                len += 2 as libc::c_int;
            }
            _ => {
                len += 1;
                len;
            }
        }
        i += 1;
        i;
    }
    cp = xmalloc((len + 1 as libc::c_int) as size_t) as *mut libc::c_char;
    if cp.is_null() {
        return 0 as *mut libc::c_char;
    }
    i = 0 as libc::c_int;
    j = 0 as libc::c_int;
    while *string.offset(i as isize) != 0 {
        match *string.offset(i as isize) as libc::c_int {
            40 | 41 | 92 => {
                let fresh2 = j;
                j = j + 1;
                *cp.offset(fresh2 as isize) = '\\' as i32 as libc::c_char;
            }
            _ => {}
        }
        let fresh3 = j;
        j = j + 1;
        *cp.offset(fresh3 as isize) = *string.offset(i as isize);
        i += 1;
        i;
    }
    let fresh4 = j;
    j = j + 1;
    *cp.offset(fresh4 as isize) = '\0' as i32 as libc::c_char;
    return cp;
}
pub unsafe extern "C" fn format_user_string(
    mut context_name: *mut libc::c_char,
    mut str: *mut libc::c_char,
) -> *mut libc::c_char {
    let mut cp: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut rbuf: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut rbuflen: libc::c_int = 0 as libc::c_int;
    let mut rbufpos: libc::c_int = 0 as libc::c_int;
    let mut i: libc::c_int = 0 as libc::c_int;
    let mut j: libc::c_int = 0;
    let mut buf: [libc::c_char; 512] = [0; 512];
    let mut buf2: [libc::c_char; 512] = [0; 512];
    let mut width: libc::c_int = 0 as libc::c_int;
    let mut justification: libc::c_int = 1 as libc::c_int;
    i = 0 as libc::c_int;
    while *str.offset(i as isize) as libc::c_int != '\0' as i32 {
        let mut type_0: libc::c_int = 0;
        type_0 = *str.offset(i as isize) as libc::c_int;
        if type_0 == '%' as i32 || type_0 == '$' as i32 {
            i += 1;
            i;
            width = 0 as libc::c_int;
            justification = 1 as libc::c_int;
            if *str.offset(i as isize) as libc::c_int == '-' as i32 {
                i += 1;
                i;
                justification = -(1 as libc::c_int);
            }
            while *(*__ctype_b_loc())
                .offset(*str.offset(i as isize) as libc::c_int as isize) as libc::c_int
                & _ISdigit as libc::c_int as libc::c_ushort as libc::c_int != 0
            {
                let fresh5 = i;
                i = i + 1;
                width = width * 10 as libc::c_int
                    + *str.offset(fresh5 as isize) as libc::c_int - '0' as i32;
            }
            if type_0 == '%' as i32 {
                match *str.offset(i as isize) as libc::c_int {
                    37 => {
                        let mut a: libc::c_int = 0;
                        if rbufpos + width >= rbuflen {
                            rbuflen += width + 1024 as libc::c_int;
                            rbuf = xrealloc(rbuf as *mut libc::c_void, rbuflen as size_t)
                                as *mut libc::c_char;
                        }
                        if width != 0 && justification < 0 as libc::c_int {
                            let fresh6 = rbufpos;
                            rbufpos = rbufpos + 1;
                            *rbuf.offset(fresh6 as isize) = '%' as i32 as libc::c_char;
                        }
                        a = 0 as libc::c_int;
                        while a < width - 1 as libc::c_int {
                            let fresh7 = rbufpos;
                            rbufpos = rbufpos + 1;
                            *rbuf.offset(fresh7 as isize) = ' ' as i32 as libc::c_char;
                            a += 1;
                            a;
                        }
                        if width == 0 || justification > 0 as libc::c_int {
                            let fresh8 = rbufpos;
                            rbufpos = rbufpos + 1;
                            *rbuf.offset(fresh8 as isize) = '%' as i32 as libc::c_char;
                        }
                    }
                    99 => {
                        getcwd(
                            buf.as_mut_ptr(),
                            ::std::mem::size_of::<[libc::c_char; 512]>() as libc::c_ulong,
                        );
                        cp = strrchr(buf.as_mut_ptr(), '/' as i32);
                        if !cp.is_null() {
                            cp = cp.offset(1);
                            cp;
                        } else {
                            cp = buf.as_mut_ptr();
                        }
                        let mut len: libc::c_int = strlen(cp) as libc::c_int;
                        let mut nspace: libc::c_int = 0;
                        if len > width {
                            nspace = 0 as libc::c_int;
                        } else {
                            nspace = width - len;
                        }
                        if rbufpos + (nspace + len) >= rbuflen {
                            rbuflen += nspace + len + 1024 as libc::c_int;
                            rbuf = xrealloc(rbuf as *mut libc::c_void, rbuflen as size_t)
                                as *mut libc::c_char;
                        }
                        if width != 0 && justification > 0 as libc::c_int {
                            while nspace != 0 {
                                let fresh9 = rbufpos;
                                rbufpos = rbufpos + 1;
                                *rbuf.offset(fresh9 as isize) = ' ' as i32 as libc::c_char;
                                nspace -= 1;
                                nspace;
                            }
                        }
                        memcpy(
                            rbuf.offset(rbufpos as isize) as *mut libc::c_void,
                            cp as *const libc::c_void,
                            len as libc::c_ulong,
                        );
                        rbufpos += len;
                        if width != 0 && justification < 0 as libc::c_int {
                            while nspace != 0 {
                                let fresh10 = rbufpos;
                                rbufpos = rbufpos + 1;
                                *rbuf.offset(fresh10 as isize) = ' ' as i32 as libc::c_char;
                                nspace -= 1;
                                nspace;
                            }
                        }
                    }
                    67 => {
                        sprintf(
                            buf.as_mut_ptr(),
                            b"%02d:%02d:%02d\0" as *const u8 as *const libc::c_char,
                            run_tm.tm_hour,
                            run_tm.tm_min,
                            run_tm.tm_sec,
                        );
                        let mut len_0: libc::c_int = strlen(buf.as_mut_ptr())
                            as libc::c_int;
                        let mut nspace_0: libc::c_int = 0;
                        if len_0 > width {
                            nspace_0 = 0 as libc::c_int;
                        } else {
                            nspace_0 = width - len_0;
                        }
                        if rbufpos + (nspace_0 + len_0) >= rbuflen {
                            rbuflen += nspace_0 + len_0 + 1024 as libc::c_int;
                            rbuf = xrealloc(rbuf as *mut libc::c_void, rbuflen as size_t)
                                as *mut libc::c_char;
                        }
                        if width != 0 && justification > 0 as libc::c_int {
                            while nspace_0 != 0 {
                                let fresh11 = rbufpos;
                                rbufpos = rbufpos + 1;
                                *rbuf.offset(fresh11 as isize) = ' ' as i32 as libc::c_char;
                                nspace_0 -= 1;
                                nspace_0;
                            }
                        }
                        memcpy(
                            rbuf.offset(rbufpos as isize) as *mut libc::c_void,
                            buf.as_mut_ptr() as *const libc::c_void,
                            len_0 as libc::c_ulong,
                        );
                        rbufpos += len_0;
                        if width != 0 && justification < 0 as libc::c_int {
                            while nspace_0 != 0 {
                                let fresh12 = rbufpos;
                                rbufpos = rbufpos + 1;
                                *rbuf.offset(fresh12 as isize) = ' ' as i32 as libc::c_char;
                                nspace_0 -= 1;
                                nspace_0;
                            }
                        }
                    }
                    100 => {
                        getcwd(
                            buf.as_mut_ptr(),
                            ::std::mem::size_of::<[libc::c_char; 512]>() as libc::c_ulong,
                        );
                        let mut len_1: libc::c_int = strlen(buf.as_mut_ptr())
                            as libc::c_int;
                        let mut nspace_1: libc::c_int = 0;
                        if len_1 > width {
                            nspace_1 = 0 as libc::c_int;
                        } else {
                            nspace_1 = width - len_1;
                        }
                        if rbufpos + (nspace_1 + len_1) >= rbuflen {
                            rbuflen += nspace_1 + len_1 + 1024 as libc::c_int;
                            rbuf = xrealloc(rbuf as *mut libc::c_void, rbuflen as size_t)
                                as *mut libc::c_char;
                        }
                        if width != 0 && justification > 0 as libc::c_int {
                            while nspace_1 != 0 {
                                let fresh13 = rbufpos;
                                rbufpos = rbufpos + 1;
                                *rbuf.offset(fresh13 as isize) = ' ' as i32 as libc::c_char;
                                nspace_1 -= 1;
                                nspace_1;
                            }
                        }
                        memcpy(
                            rbuf.offset(rbufpos as isize) as *mut libc::c_void,
                            buf.as_mut_ptr() as *const libc::c_void,
                            len_1 as libc::c_ulong,
                        );
                        rbufpos += len_1;
                        if width != 0 && justification < 0 as libc::c_int {
                            while nspace_1 != 0 {
                                let fresh14 = rbufpos;
                                rbufpos = rbufpos + 1;
                                *rbuf.offset(fresh14 as isize) = ' ' as i32 as libc::c_char;
                                nspace_1 -= 1;
                                nspace_1;
                            }
                        }
                    }
                    68 => {
                        if *str.offset((i + 1 as libc::c_int) as isize) as libc::c_int
                            == '{' as i32
                        {
                            j = 0 as libc::c_int;
                            i += 2 as libc::c_int;
                            while (j as libc::c_ulong)
                                < ::std::mem::size_of::<[libc::c_char; 512]>()
                                    as libc::c_ulong
                                && *str.offset(i as isize) as libc::c_int != 0
                                && *str.offset(i as isize) as libc::c_int != '}' as i32
                            {
                                buf2[j as usize] = *str.offset(i as isize);
                                i += 1;
                                i;
                                j += 1;
                                j;
                            }
                            if *str.offset(i as isize) as libc::c_int != '}' as i32 {
                                fprintf(
                                    stderr,
                                    b"%s: \0" as *const u8 as *const libc::c_char,
                                    program,
                                );
                                fprintf(
                                    stderr,
                                    dcgettext(
                                        0 as *const libc::c_char,
                                        b"%s: too long format for %%D{} escape\0" as *const u8
                                            as *const libc::c_char,
                                        5 as libc::c_int,
                                    ),
                                    context_name,
                                );
                                fprintf(
                                    stderr,
                                    b"\n\0" as *const u8 as *const libc::c_char,
                                );
                                fflush(stderr);
                                exit(1 as libc::c_int);
                            }
                            buf2[j as usize] = '\0' as i32 as libc::c_char;
                            strftime(
                                buf.as_mut_ptr(),
                                ::std::mem::size_of::<[libc::c_char; 512]>()
                                    as libc::c_ulong,
                                buf2.as_mut_ptr(),
                                &mut run_tm,
                            );
                        } else {
                            sprintf(
                                buf.as_mut_ptr(),
                                b"%02d-%02d-%02d\0" as *const u8 as *const libc::c_char,
                                run_tm.tm_year % 100 as libc::c_int,
                                run_tm.tm_mon + 1 as libc::c_int,
                                run_tm.tm_mday,
                            );
                        }
                        let mut len_2: libc::c_int = strlen(buf.as_mut_ptr())
                            as libc::c_int;
                        let mut nspace_2: libc::c_int = 0;
                        if len_2 > width {
                            nspace_2 = 0 as libc::c_int;
                        } else {
                            nspace_2 = width - len_2;
                        }
                        if rbufpos + (nspace_2 + len_2) >= rbuflen {
                            rbuflen += nspace_2 + len_2 + 1024 as libc::c_int;
                            rbuf = xrealloc(rbuf as *mut libc::c_void, rbuflen as size_t)
                                as *mut libc::c_char;
                        }
                        if width != 0 && justification > 0 as libc::c_int {
                            while nspace_2 != 0 {
                                let fresh15 = rbufpos;
                                rbufpos = rbufpos + 1;
                                *rbuf.offset(fresh15 as isize) = ' ' as i32 as libc::c_char;
                                nspace_2 -= 1;
                                nspace_2;
                            }
                        }
                        memcpy(
                            rbuf.offset(rbufpos as isize) as *mut libc::c_void,
                            buf.as_mut_ptr() as *const libc::c_void,
                            len_2 as libc::c_ulong,
                        );
                        rbufpos += len_2;
                        if width != 0 && justification < 0 as libc::c_int {
                            while nspace_2 != 0 {
                                let fresh16 = rbufpos;
                                rbufpos = rbufpos + 1;
                                *rbuf.offset(fresh16 as isize) = ' ' as i32 as libc::c_char;
                                nspace_2 -= 1;
                                nspace_2;
                            }
                        }
                    }
                    69 => {
                        sprintf(
                            buf.as_mut_ptr(),
                            b"%02d/%02d/%02d\0" as *const u8 as *const libc::c_char,
                            run_tm.tm_year % 100 as libc::c_int,
                            run_tm.tm_mon + 1 as libc::c_int,
                            run_tm.tm_mday,
                        );
                        let mut len_3: libc::c_int = strlen(buf.as_mut_ptr())
                            as libc::c_int;
                        let mut nspace_3: libc::c_int = 0;
                        if len_3 > width {
                            nspace_3 = 0 as libc::c_int;
                        } else {
                            nspace_3 = width - len_3;
                        }
                        if rbufpos + (nspace_3 + len_3) >= rbuflen {
                            rbuflen += nspace_3 + len_3 + 1024 as libc::c_int;
                            rbuf = xrealloc(rbuf as *mut libc::c_void, rbuflen as size_t)
                                as *mut libc::c_char;
                        }
                        if width != 0 && justification > 0 as libc::c_int {
                            while nspace_3 != 0 {
                                let fresh17 = rbufpos;
                                rbufpos = rbufpos + 1;
                                *rbuf.offset(fresh17 as isize) = ' ' as i32 as libc::c_char;
                                nspace_3 -= 1;
                                nspace_3;
                            }
                        }
                        memcpy(
                            rbuf.offset(rbufpos as isize) as *mut libc::c_void,
                            buf.as_mut_ptr() as *const libc::c_void,
                            len_3 as libc::c_ulong,
                        );
                        rbufpos += len_3;
                        if width != 0 && justification < 0 as libc::c_int {
                            while nspace_3 != 0 {
                                let fresh18 = rbufpos;
                                rbufpos = rbufpos + 1;
                                *rbuf.offset(fresh18 as isize) = ' ' as i32 as libc::c_char;
                                nspace_3 -= 1;
                                nspace_3;
                            }
                        }
                    }
                    70 => {
                        sprintf(
                            buf.as_mut_ptr(),
                            b"%d.%d.%d\0" as *const u8 as *const libc::c_char,
                            run_tm.tm_mday,
                            run_tm.tm_mon + 1 as libc::c_int,
                            run_tm.tm_year + 1900 as libc::c_int,
                        );
                        let mut len_4: libc::c_int = strlen(buf.as_mut_ptr())
                            as libc::c_int;
                        let mut nspace_4: libc::c_int = 0;
                        if len_4 > width {
                            nspace_4 = 0 as libc::c_int;
                        } else {
                            nspace_4 = width - len_4;
                        }
                        if rbufpos + (nspace_4 + len_4) >= rbuflen {
                            rbuflen += nspace_4 + len_4 + 1024 as libc::c_int;
                            rbuf = xrealloc(rbuf as *mut libc::c_void, rbuflen as size_t)
                                as *mut libc::c_char;
                        }
                        if width != 0 && justification > 0 as libc::c_int {
                            while nspace_4 != 0 {
                                let fresh19 = rbufpos;
                                rbufpos = rbufpos + 1;
                                *rbuf.offset(fresh19 as isize) = ' ' as i32 as libc::c_char;
                                nspace_4 -= 1;
                                nspace_4;
                            }
                        }
                        memcpy(
                            rbuf.offset(rbufpos as isize) as *mut libc::c_void,
                            buf.as_mut_ptr() as *const libc::c_void,
                            len_4 as libc::c_ulong,
                        );
                        rbufpos += len_4;
                        if width != 0 && justification < 0 as libc::c_int {
                            while nspace_4 != 0 {
                                let fresh20 = rbufpos;
                                rbufpos = rbufpos + 1;
                                *rbuf.offset(fresh20 as isize) = ' ' as i32 as libc::c_char;
                                nspace_4 -= 1;
                                nspace_4;
                            }
                        }
                    }
                    72 => {
                        let mut len_5: libc::c_int = strlen(title) as libc::c_int;
                        let mut nspace_5: libc::c_int = 0;
                        if len_5 > width {
                            nspace_5 = 0 as libc::c_int;
                        } else {
                            nspace_5 = width - len_5;
                        }
                        if rbufpos + (nspace_5 + len_5) >= rbuflen {
                            rbuflen += nspace_5 + len_5 + 1024 as libc::c_int;
                            rbuf = xrealloc(rbuf as *mut libc::c_void, rbuflen as size_t)
                                as *mut libc::c_char;
                        }
                        if width != 0 && justification > 0 as libc::c_int {
                            while nspace_5 != 0 {
                                let fresh21 = rbufpos;
                                rbufpos = rbufpos + 1;
                                *rbuf.offset(fresh21 as isize) = ' ' as i32 as libc::c_char;
                                nspace_5 -= 1;
                                nspace_5;
                            }
                        }
                        memcpy(
                            rbuf.offset(rbufpos as isize) as *mut libc::c_void,
                            title as *const libc::c_void,
                            len_5 as libc::c_ulong,
                        );
                        rbufpos += len_5;
                        if width != 0 && justification < 0 as libc::c_int {
                            while nspace_5 != 0 {
                                let fresh22 = rbufpos;
                                rbufpos = rbufpos + 1;
                                *rbuf.offset(fresh22 as isize) = ' ' as i32 as libc::c_char;
                                nspace_5 -= 1;
                                nspace_5;
                            }
                        }
                    }
                    109 => {
                        gethostname(
                            buf.as_mut_ptr(),
                            ::std::mem::size_of::<[libc::c_char; 512]>() as libc::c_ulong,
                        );
                        cp = strchr(buf.as_mut_ptr(), '.' as i32);
                        if !cp.is_null() {
                            *cp = '\0' as i32 as libc::c_char;
                        }
                        let mut len_6: libc::c_int = strlen(buf.as_mut_ptr())
                            as libc::c_int;
                        let mut nspace_6: libc::c_int = 0;
                        if len_6 > width {
                            nspace_6 = 0 as libc::c_int;
                        } else {
                            nspace_6 = width - len_6;
                        }
                        if rbufpos + (nspace_6 + len_6) >= rbuflen {
                            rbuflen += nspace_6 + len_6 + 1024 as libc::c_int;
                            rbuf = xrealloc(rbuf as *mut libc::c_void, rbuflen as size_t)
                                as *mut libc::c_char;
                        }
                        if width != 0 && justification > 0 as libc::c_int {
                            while nspace_6 != 0 {
                                let fresh23 = rbufpos;
                                rbufpos = rbufpos + 1;
                                *rbuf.offset(fresh23 as isize) = ' ' as i32 as libc::c_char;
                                nspace_6 -= 1;
                                nspace_6;
                            }
                        }
                        memcpy(
                            rbuf.offset(rbufpos as isize) as *mut libc::c_void,
                            buf.as_mut_ptr() as *const libc::c_void,
                            len_6 as libc::c_ulong,
                        );
                        rbufpos += len_6;
                        if width != 0 && justification < 0 as libc::c_int {
                            while nspace_6 != 0 {
                                let fresh24 = rbufpos;
                                rbufpos = rbufpos + 1;
                                *rbuf.offset(fresh24 as isize) = ' ' as i32 as libc::c_char;
                                nspace_6 -= 1;
                                nspace_6;
                            }
                        }
                    }
                    77 => {
                        gethostname(
                            buf.as_mut_ptr(),
                            ::std::mem::size_of::<[libc::c_char; 512]>() as libc::c_ulong,
                        );
                        let mut len_7: libc::c_int = strlen(buf.as_mut_ptr())
                            as libc::c_int;
                        let mut nspace_7: libc::c_int = 0;
                        if len_7 > width {
                            nspace_7 = 0 as libc::c_int;
                        } else {
                            nspace_7 = width - len_7;
                        }
                        if rbufpos + (nspace_7 + len_7) >= rbuflen {
                            rbuflen += nspace_7 + len_7 + 1024 as libc::c_int;
                            rbuf = xrealloc(rbuf as *mut libc::c_void, rbuflen as size_t)
                                as *mut libc::c_char;
                        }
                        if width != 0 && justification > 0 as libc::c_int {
                            while nspace_7 != 0 {
                                let fresh25 = rbufpos;
                                rbufpos = rbufpos + 1;
                                *rbuf.offset(fresh25 as isize) = ' ' as i32 as libc::c_char;
                                nspace_7 -= 1;
                                nspace_7;
                            }
                        }
                        memcpy(
                            rbuf.offset(rbufpos as isize) as *mut libc::c_void,
                            buf.as_mut_ptr() as *const libc::c_void,
                            len_7 as libc::c_ulong,
                        );
                        rbufpos += len_7;
                        if width != 0 && justification < 0 as libc::c_int {
                            while nspace_7 != 0 {
                                let fresh26 = rbufpos;
                                rbufpos = rbufpos + 1;
                                *rbuf.offset(fresh26 as isize) = ' ' as i32 as libc::c_char;
                                nspace_7 -= 1;
                                nspace_7;
                            }
                        }
                    }
                    110 => {
                        let mut len_8: libc::c_int = strlen((*passwd).pw_name)
                            as libc::c_int;
                        let mut nspace_8: libc::c_int = 0;
                        if len_8 > width {
                            nspace_8 = 0 as libc::c_int;
                        } else {
                            nspace_8 = width - len_8;
                        }
                        if rbufpos + (nspace_8 + len_8) >= rbuflen {
                            rbuflen += nspace_8 + len_8 + 1024 as libc::c_int;
                            rbuf = xrealloc(rbuf as *mut libc::c_void, rbuflen as size_t)
                                as *mut libc::c_char;
                        }
                        if width != 0 && justification > 0 as libc::c_int {
                            while nspace_8 != 0 {
                                let fresh27 = rbufpos;
                                rbufpos = rbufpos + 1;
                                *rbuf.offset(fresh27 as isize) = ' ' as i32 as libc::c_char;
                                nspace_8 -= 1;
                                nspace_8;
                            }
                        }
                        memcpy(
                            rbuf.offset(rbufpos as isize) as *mut libc::c_void,
                            (*passwd).pw_name as *const libc::c_void,
                            len_8 as libc::c_ulong,
                        );
                        rbufpos += len_8;
                        if width != 0 && justification < 0 as libc::c_int {
                            while nspace_8 != 0 {
                                let fresh28 = rbufpos;
                                rbufpos = rbufpos + 1;
                                *rbuf.offset(fresh28 as isize) = ' ' as i32 as libc::c_char;
                                nspace_8 -= 1;
                                nspace_8;
                            }
                        }
                    }
                    78 => {
                        strcpy(buf.as_mut_ptr(), (*passwd).pw_gecos);
                        cp = strchr(buf.as_mut_ptr(), ',' as i32);
                        if !cp.is_null() {
                            *cp = '\0' as i32 as libc::c_char;
                        }
                        let mut len_9: libc::c_int = strlen(buf.as_mut_ptr())
                            as libc::c_int;
                        let mut nspace_9: libc::c_int = 0;
                        if len_9 > width {
                            nspace_9 = 0 as libc::c_int;
                        } else {
                            nspace_9 = width - len_9;
                        }
                        if rbufpos + (nspace_9 + len_9) >= rbuflen {
                            rbuflen += nspace_9 + len_9 + 1024 as libc::c_int;
                            rbuf = xrealloc(rbuf as *mut libc::c_void, rbuflen as size_t)
                                as *mut libc::c_char;
                        }
                        if width != 0 && justification > 0 as libc::c_int {
                            while nspace_9 != 0 {
                                let fresh29 = rbufpos;
                                rbufpos = rbufpos + 1;
                                *rbuf.offset(fresh29 as isize) = ' ' as i32 as libc::c_char;
                                nspace_9 -= 1;
                                nspace_9;
                            }
                        }
                        memcpy(
                            rbuf.offset(rbufpos as isize) as *mut libc::c_void,
                            buf.as_mut_ptr() as *const libc::c_void,
                            len_9 as libc::c_ulong,
                        );
                        rbufpos += len_9;
                        if width != 0 && justification < 0 as libc::c_int {
                            while nspace_9 != 0 {
                                let fresh30 = rbufpos;
                                rbufpos = rbufpos + 1;
                                *rbuf.offset(fresh30 as isize) = ' ' as i32 as libc::c_char;
                                nspace_9 -= 1;
                                nspace_9;
                            }
                        }
                    }
                    116 => {
                        sprintf(
                            buf.as_mut_ptr(),
                            b"%d:%d%s\0" as *const u8 as *const libc::c_char,
                            if run_tm.tm_hour > 12 as libc::c_int {
                                run_tm.tm_hour - 12 as libc::c_int
                            } else {
                                run_tm.tm_hour
                            },
                            run_tm.tm_min,
                            if run_tm.tm_hour > 12 as libc::c_int {
                                b"pm\0" as *const u8 as *const libc::c_char
                            } else {
                                b"am\0" as *const u8 as *const libc::c_char
                            },
                        );
                        let mut len_10: libc::c_int = strlen(buf.as_mut_ptr())
                            as libc::c_int;
                        let mut nspace_10: libc::c_int = 0;
                        if len_10 > width {
                            nspace_10 = 0 as libc::c_int;
                        } else {
                            nspace_10 = width - len_10;
                        }
                        if rbufpos + (nspace_10 + len_10) >= rbuflen {
                            rbuflen += nspace_10 + len_10 + 1024 as libc::c_int;
                            rbuf = xrealloc(rbuf as *mut libc::c_void, rbuflen as size_t)
                                as *mut libc::c_char;
                        }
                        if width != 0 && justification > 0 as libc::c_int {
                            while nspace_10 != 0 {
                                let fresh31 = rbufpos;
                                rbufpos = rbufpos + 1;
                                *rbuf.offset(fresh31 as isize) = ' ' as i32 as libc::c_char;
                                nspace_10 -= 1;
                                nspace_10;
                            }
                        }
                        memcpy(
                            rbuf.offset(rbufpos as isize) as *mut libc::c_void,
                            buf.as_mut_ptr() as *const libc::c_void,
                            len_10 as libc::c_ulong,
                        );
                        rbufpos += len_10;
                        if width != 0 && justification < 0 as libc::c_int {
                            while nspace_10 != 0 {
                                let fresh32 = rbufpos;
                                rbufpos = rbufpos + 1;
                                *rbuf.offset(fresh32 as isize) = ' ' as i32 as libc::c_char;
                                nspace_10 -= 1;
                                nspace_10;
                            }
                        }
                    }
                    84 => {
                        sprintf(
                            buf.as_mut_ptr(),
                            b"%d:%d\0" as *const u8 as *const libc::c_char,
                            run_tm.tm_hour,
                            run_tm.tm_min,
                        );
                        let mut len_11: libc::c_int = strlen(buf.as_mut_ptr())
                            as libc::c_int;
                        let mut nspace_11: libc::c_int = 0;
                        if len_11 > width {
                            nspace_11 = 0 as libc::c_int;
                        } else {
                            nspace_11 = width - len_11;
                        }
                        if rbufpos + (nspace_11 + len_11) >= rbuflen {
                            rbuflen += nspace_11 + len_11 + 1024 as libc::c_int;
                            rbuf = xrealloc(rbuf as *mut libc::c_void, rbuflen as size_t)
                                as *mut libc::c_char;
                        }
                        if width != 0 && justification > 0 as libc::c_int {
                            while nspace_11 != 0 {
                                let fresh33 = rbufpos;
                                rbufpos = rbufpos + 1;
                                *rbuf.offset(fresh33 as isize) = ' ' as i32 as libc::c_char;
                                nspace_11 -= 1;
                                nspace_11;
                            }
                        }
                        memcpy(
                            rbuf.offset(rbufpos as isize) as *mut libc::c_void,
                            buf.as_mut_ptr() as *const libc::c_void,
                            len_11 as libc::c_ulong,
                        );
                        rbufpos += len_11;
                        if width != 0 && justification < 0 as libc::c_int {
                            while nspace_11 != 0 {
                                let fresh34 = rbufpos;
                                rbufpos = rbufpos + 1;
                                *rbuf.offset(fresh34 as isize) = ' ' as i32 as libc::c_char;
                                nspace_11 -= 1;
                                nspace_11;
                            }
                        }
                    }
                    42 => {
                        sprintf(
                            buf.as_mut_ptr(),
                            b"%d:%d:%d\0" as *const u8 as *const libc::c_char,
                            run_tm.tm_hour,
                            run_tm.tm_min,
                            run_tm.tm_sec,
                        );
                        let mut len_12: libc::c_int = strlen(buf.as_mut_ptr())
                            as libc::c_int;
                        let mut nspace_12: libc::c_int = 0;
                        if len_12 > width {
                            nspace_12 = 0 as libc::c_int;
                        } else {
                            nspace_12 = width - len_12;
                        }
                        if rbufpos + (nspace_12 + len_12) >= rbuflen {
                            rbuflen += nspace_12 + len_12 + 1024 as libc::c_int;
                            rbuf = xrealloc(rbuf as *mut libc::c_void, rbuflen as size_t)
                                as *mut libc::c_char;
                        }
                        if width != 0 && justification > 0 as libc::c_int {
                            while nspace_12 != 0 {
                                let fresh35 = rbufpos;
                                rbufpos = rbufpos + 1;
                                *rbuf.offset(fresh35 as isize) = ' ' as i32 as libc::c_char;
                                nspace_12 -= 1;
                                nspace_12;
                            }
                        }
                        memcpy(
                            rbuf.offset(rbufpos as isize) as *mut libc::c_void,
                            buf.as_mut_ptr() as *const libc::c_void,
                            len_12 as libc::c_ulong,
                        );
                        rbufpos += len_12;
                        if width != 0 && justification < 0 as libc::c_int {
                            while nspace_12 != 0 {
                                let fresh36 = rbufpos;
                                rbufpos = rbufpos + 1;
                                *rbuf.offset(fresh36 as isize) = ' ' as i32 as libc::c_char;
                                nspace_12 -= 1;
                                nspace_12;
                            }
                        }
                    }
                    87 => {
                        sprintf(
                            buf.as_mut_ptr(),
                            b"%02d/%02d/%02d\0" as *const u8 as *const libc::c_char,
                            run_tm.tm_mon + 1 as libc::c_int,
                            run_tm.tm_mday,
                            run_tm.tm_year % 100 as libc::c_int,
                        );
                        let mut len_13: libc::c_int = strlen(buf.as_mut_ptr())
                            as libc::c_int;
                        let mut nspace_13: libc::c_int = 0;
                        if len_13 > width {
                            nspace_13 = 0 as libc::c_int;
                        } else {
                            nspace_13 = width - len_13;
                        }
                        if rbufpos + (nspace_13 + len_13) >= rbuflen {
                            rbuflen += nspace_13 + len_13 + 1024 as libc::c_int;
                            rbuf = xrealloc(rbuf as *mut libc::c_void, rbuflen as size_t)
                                as *mut libc::c_char;
                        }
                        if width != 0 && justification > 0 as libc::c_int {
                            while nspace_13 != 0 {
                                let fresh37 = rbufpos;
                                rbufpos = rbufpos + 1;
                                *rbuf.offset(fresh37 as isize) = ' ' as i32 as libc::c_char;
                                nspace_13 -= 1;
                                nspace_13;
                            }
                        }
                        memcpy(
                            rbuf.offset(rbufpos as isize) as *mut libc::c_void,
                            buf.as_mut_ptr() as *const libc::c_void,
                            len_13 as libc::c_ulong,
                        );
                        rbufpos += len_13;
                        if width != 0 && justification < 0 as libc::c_int {
                            while nspace_13 != 0 {
                                let fresh38 = rbufpos;
                                rbufpos = rbufpos + 1;
                                *rbuf.offset(fresh38 as isize) = ' ' as i32 as libc::c_char;
                                nspace_13 -= 1;
                                nspace_13;
                            }
                        }
                    }
                    _ => {
                        fprintf(
                            stderr,
                            b"%s: \0" as *const u8 as *const libc::c_char,
                            program,
                        );
                        fprintf(
                            stderr,
                            dcgettext(
                                0 as *const libc::c_char,
                                b"%s: unknown `%%' escape `%c' (%d)\0" as *const u8
                                    as *const libc::c_char,
                                5 as libc::c_int,
                            ),
                            context_name,
                            *str.offset(i as isize) as libc::c_int,
                            *str.offset(i as isize) as libc::c_int,
                        );
                        fprintf(stderr, b"\n\0" as *const u8 as *const libc::c_char);
                        fflush(stderr);
                        exit(1 as libc::c_int);
                    }
                }
            } else {
                match *str.offset(i as isize) as libc::c_int {
                    36 => {
                        let mut a_0: libc::c_int = 0;
                        if rbufpos + width >= rbuflen {
                            rbuflen += width + 1024 as libc::c_int;
                            rbuf = xrealloc(rbuf as *mut libc::c_void, rbuflen as size_t)
                                as *mut libc::c_char;
                        }
                        if width != 0 && justification < 0 as libc::c_int {
                            let fresh39 = rbufpos;
                            rbufpos = rbufpos + 1;
                            *rbuf.offset(fresh39 as isize) = '$' as i32 as libc::c_char;
                        }
                        a_0 = 0 as libc::c_int;
                        while a_0 < width - 1 as libc::c_int {
                            let fresh40 = rbufpos;
                            rbufpos = rbufpos + 1;
                            *rbuf.offset(fresh40 as isize) = ' ' as i32 as libc::c_char;
                            a_0 += 1;
                            a_0;
                        }
                        if width == 0 || justification > 0 as libc::c_int {
                            let fresh41 = rbufpos;
                            rbufpos = rbufpos + 1;
                            *rbuf.offset(fresh41 as isize) = '$' as i32 as libc::c_char;
                        }
                    }
                    37 => {
                        if slicing != 0 {
                            sprintf(
                                buf.as_mut_ptr(),
                                b"%d%c\0" as *const u8 as *const libc::c_char,
                                current_pagenum,
                                slice
                                    .wrapping_sub(1 as libc::c_int as libc::c_uint)
                                    .wrapping_add('A' as i32 as libc::c_uint),
                            );
                        } else {
                            sprintf(
                                buf.as_mut_ptr(),
                                b"%d\0" as *const u8 as *const libc::c_char,
                                current_pagenum,
                            );
                        }
                        let mut len_14: libc::c_int = strlen(buf.as_mut_ptr())
                            as libc::c_int;
                        let mut nspace_14: libc::c_int = 0;
                        if len_14 > width {
                            nspace_14 = 0 as libc::c_int;
                        } else {
                            nspace_14 = width - len_14;
                        }
                        if rbufpos + (nspace_14 + len_14) >= rbuflen {
                            rbuflen += nspace_14 + len_14 + 1024 as libc::c_int;
                            rbuf = xrealloc(rbuf as *mut libc::c_void, rbuflen as size_t)
                                as *mut libc::c_char;
                        }
                        if width != 0 && justification > 0 as libc::c_int {
                            while nspace_14 != 0 {
                                let fresh42 = rbufpos;
                                rbufpos = rbufpos + 1;
                                *rbuf.offset(fresh42 as isize) = ' ' as i32 as libc::c_char;
                                nspace_14 -= 1;
                                nspace_14;
                            }
                        }
                        memcpy(
                            rbuf.offset(rbufpos as isize) as *mut libc::c_void,
                            buf.as_mut_ptr() as *const libc::c_void,
                            len_14 as libc::c_ulong,
                        );
                        rbufpos += len_14;
                        if width != 0 && justification < 0 as libc::c_int {
                            while nspace_14 != 0 {
                                let fresh43 = rbufpos;
                                rbufpos = rbufpos + 1;
                                *rbuf.offset(fresh43 as isize) = ' ' as i32 as libc::c_char;
                                nspace_14 -= 1;
                                nspace_14;
                            }
                        }
                    }
                    61 => {
                        let mut a_1: libc::c_int = 0;
                        if rbufpos + width >= rbuflen {
                            rbuflen += width + 1024 as libc::c_int;
                            rbuf = xrealloc(rbuf as *mut libc::c_void, rbuflen as size_t)
                                as *mut libc::c_char;
                        }
                        if width != 0 && justification < 0 as libc::c_int {
                            let fresh44 = rbufpos;
                            rbufpos = rbufpos + 1;
                            *rbuf
                                .offset(fresh44 as isize) = '\u{1}' as i32 as libc::c_char;
                        }
                        a_1 = 0 as libc::c_int;
                        while a_1 < width - 1 as libc::c_int {
                            let fresh45 = rbufpos;
                            rbufpos = rbufpos + 1;
                            *rbuf.offset(fresh45 as isize) = ' ' as i32 as libc::c_char;
                            a_1 += 1;
                            a_1;
                        }
                        if width == 0 || justification > 0 as libc::c_int {
                            let fresh46 = rbufpos;
                            rbufpos = rbufpos + 1;
                            *rbuf
                                .offset(fresh46 as isize) = '\u{1}' as i32 as libc::c_char;
                        }
                    }
                    112 => {
                        sprintf(
                            buf.as_mut_ptr(),
                            b"%d\0" as *const u8 as *const libc::c_char,
                            total_pages,
                        );
                        let mut len_15: libc::c_int = strlen(buf.as_mut_ptr())
                            as libc::c_int;
                        let mut nspace_15: libc::c_int = 0;
                        if len_15 > width {
                            nspace_15 = 0 as libc::c_int;
                        } else {
                            nspace_15 = width - len_15;
                        }
                        if rbufpos + (nspace_15 + len_15) >= rbuflen {
                            rbuflen += nspace_15 + len_15 + 1024 as libc::c_int;
                            rbuf = xrealloc(rbuf as *mut libc::c_void, rbuflen as size_t)
                                as *mut libc::c_char;
                        }
                        if width != 0 && justification > 0 as libc::c_int {
                            while nspace_15 != 0 {
                                let fresh47 = rbufpos;
                                rbufpos = rbufpos + 1;
                                *rbuf.offset(fresh47 as isize) = ' ' as i32 as libc::c_char;
                                nspace_15 -= 1;
                                nspace_15;
                            }
                        }
                        memcpy(
                            rbuf.offset(rbufpos as isize) as *mut libc::c_void,
                            buf.as_mut_ptr() as *const libc::c_void,
                            len_15 as libc::c_ulong,
                        );
                        rbufpos += len_15;
                        if width != 0 && justification < 0 as libc::c_int {
                            while nspace_15 != 0 {
                                let fresh48 = rbufpos;
                                rbufpos = rbufpos + 1;
                                *rbuf.offset(fresh48 as isize) = ' ' as i32 as libc::c_char;
                                nspace_15 -= 1;
                                nspace_15;
                            }
                        }
                    }
                    40 => {
                        j = 0 as libc::c_int;
                        i += 1;
                        i;
                        while *str.offset(i as isize) as libc::c_int != 0
                            && *str.offset(i as isize) as libc::c_int != ')' as i32
                            && (j as libc::c_ulong)
                                < (::std::mem::size_of::<[libc::c_char; 512]>()
                                    as libc::c_ulong)
                                    .wrapping_sub(1 as libc::c_int as libc::c_ulong)
                        {
                            let fresh49 = j;
                            j = j + 1;
                            buf[fresh49 as usize] = *str.offset(i as isize);
                            i += 1;
                            i;
                        }
                        if *str.offset(i as isize) as libc::c_int == '\0' as i32 {
                            fprintf(
                                stderr,
                                b"%s: \0" as *const u8 as *const libc::c_char,
                                program,
                            );
                            fprintf(
                                stderr,
                                dcgettext(
                                    0 as *const libc::c_char,
                                    b"%s: no closing ')' for $() escape\0" as *const u8
                                        as *const libc::c_char,
                                    5 as libc::c_int,
                                ),
                                context_name,
                            );
                            fprintf(stderr, b"\n\0" as *const u8 as *const libc::c_char);
                            fflush(stderr);
                            exit(1 as libc::c_int);
                        }
                        if *str.offset(i as isize) as libc::c_int != ')' as i32 {
                            fprintf(
                                stderr,
                                b"%s: \0" as *const u8 as *const libc::c_char,
                                program,
                            );
                            fprintf(
                                stderr,
                                dcgettext(
                                    0 as *const libc::c_char,
                                    b"%s: too long variable name for $() escape\0" as *const u8
                                        as *const libc::c_char,
                                    5 as libc::c_int,
                                ),
                                context_name,
                            );
                            fprintf(stderr, b"\n\0" as *const u8 as *const libc::c_char);
                            fflush(stderr);
                            exit(1 as libc::c_int);
                        }
                        buf[j as usize] = '\0' as i32 as libc::c_char;
                        cp = getenv(buf.as_mut_ptr());
                        if cp.is_null() {
                            cp = b"\0" as *const u8 as *const libc::c_char
                                as *mut libc::c_char;
                        }
                        let mut len_16: libc::c_int = strlen(cp) as libc::c_int;
                        let mut nspace_16: libc::c_int = 0;
                        if len_16 > width {
                            nspace_16 = 0 as libc::c_int;
                        } else {
                            nspace_16 = width - len_16;
                        }
                        if rbufpos + (nspace_16 + len_16) >= rbuflen {
                            rbuflen += nspace_16 + len_16 + 1024 as libc::c_int;
                            rbuf = xrealloc(rbuf as *mut libc::c_void, rbuflen as size_t)
                                as *mut libc::c_char;
                        }
                        if width != 0 && justification > 0 as libc::c_int {
                            while nspace_16 != 0 {
                                let fresh50 = rbufpos;
                                rbufpos = rbufpos + 1;
                                *rbuf.offset(fresh50 as isize) = ' ' as i32 as libc::c_char;
                                nspace_16 -= 1;
                                nspace_16;
                            }
                        }
                        memcpy(
                            rbuf.offset(rbufpos as isize) as *mut libc::c_void,
                            cp as *const libc::c_void,
                            len_16 as libc::c_ulong,
                        );
                        rbufpos += len_16;
                        if width != 0 && justification < 0 as libc::c_int {
                            while nspace_16 != 0 {
                                let fresh51 = rbufpos;
                                rbufpos = rbufpos + 1;
                                *rbuf.offset(fresh51 as isize) = ' ' as i32 as libc::c_char;
                                nspace_16 -= 1;
                                nspace_16;
                            }
                        }
                    }
                    67 => {
                        sprintf(
                            buf.as_mut_ptr(),
                            b"%02d:%02d:%02d\0" as *const u8 as *const libc::c_char,
                            mod_tm.tm_hour,
                            mod_tm.tm_min,
                            mod_tm.tm_sec,
                        );
                        let mut len_17: libc::c_int = strlen(buf.as_mut_ptr())
                            as libc::c_int;
                        let mut nspace_17: libc::c_int = 0;
                        if len_17 > width {
                            nspace_17 = 0 as libc::c_int;
                        } else {
                            nspace_17 = width - len_17;
                        }
                        if rbufpos + (nspace_17 + len_17) >= rbuflen {
                            rbuflen += nspace_17 + len_17 + 1024 as libc::c_int;
                            rbuf = xrealloc(rbuf as *mut libc::c_void, rbuflen as size_t)
                                as *mut libc::c_char;
                        }
                        if width != 0 && justification > 0 as libc::c_int {
                            while nspace_17 != 0 {
                                let fresh52 = rbufpos;
                                rbufpos = rbufpos + 1;
                                *rbuf.offset(fresh52 as isize) = ' ' as i32 as libc::c_char;
                                nspace_17 -= 1;
                                nspace_17;
                            }
                        }
                        memcpy(
                            rbuf.offset(rbufpos as isize) as *mut libc::c_void,
                            buf.as_mut_ptr() as *const libc::c_void,
                            len_17 as libc::c_ulong,
                        );
                        rbufpos += len_17;
                        if width != 0 && justification < 0 as libc::c_int {
                            while nspace_17 != 0 {
                                let fresh53 = rbufpos;
                                rbufpos = rbufpos + 1;
                                *rbuf.offset(fresh53 as isize) = ' ' as i32 as libc::c_char;
                                nspace_17 -= 1;
                                nspace_17;
                            }
                        }
                    }
                    68 => {
                        if *str.offset((i + 1 as libc::c_int) as isize) as libc::c_int
                            == '{' as i32
                        {
                            j = 0 as libc::c_int;
                            i += 2 as libc::c_int;
                            while (j as libc::c_ulong)
                                < ::std::mem::size_of::<[libc::c_char; 512]>()
                                    as libc::c_ulong
                                && *str.offset(i as isize) as libc::c_int != 0
                                && *str.offset(i as isize) as libc::c_int != '}' as i32
                            {
                                buf2[j as usize] = *str.offset(i as isize);
                                i += 1;
                                i;
                                j += 1;
                                j;
                            }
                            if *str.offset(i as isize) as libc::c_int != '}' as i32 {
                                fprintf(
                                    stderr,
                                    b"%s: \0" as *const u8 as *const libc::c_char,
                                    program,
                                );
                                fprintf(
                                    stderr,
                                    dcgettext(
                                        0 as *const libc::c_char,
                                        b"%s: too long format for $D{} escape\0" as *const u8
                                            as *const libc::c_char,
                                        5 as libc::c_int,
                                    ),
                                    context_name,
                                );
                                fprintf(
                                    stderr,
                                    b"\n\0" as *const u8 as *const libc::c_char,
                                );
                                fflush(stderr);
                                exit(1 as libc::c_int);
                            }
                            buf2[j as usize] = '\0' as i32 as libc::c_char;
                            strftime(
                                buf.as_mut_ptr(),
                                ::std::mem::size_of::<[libc::c_char; 512]>()
                                    as libc::c_ulong,
                                buf2.as_mut_ptr(),
                                &mut mod_tm,
                            );
                        } else {
                            sprintf(
                                buf.as_mut_ptr(),
                                b"%02d-%02d-%02d\0" as *const u8 as *const libc::c_char,
                                mod_tm.tm_year % 100 as libc::c_int,
                                mod_tm.tm_mon + 1 as libc::c_int,
                                mod_tm.tm_mday,
                            );
                        }
                        let mut len_18: libc::c_int = strlen(buf.as_mut_ptr())
                            as libc::c_int;
                        let mut nspace_18: libc::c_int = 0;
                        if len_18 > width {
                            nspace_18 = 0 as libc::c_int;
                        } else {
                            nspace_18 = width - len_18;
                        }
                        if rbufpos + (nspace_18 + len_18) >= rbuflen {
                            rbuflen += nspace_18 + len_18 + 1024 as libc::c_int;
                            rbuf = xrealloc(rbuf as *mut libc::c_void, rbuflen as size_t)
                                as *mut libc::c_char;
                        }
                        if width != 0 && justification > 0 as libc::c_int {
                            while nspace_18 != 0 {
                                let fresh54 = rbufpos;
                                rbufpos = rbufpos + 1;
                                *rbuf.offset(fresh54 as isize) = ' ' as i32 as libc::c_char;
                                nspace_18 -= 1;
                                nspace_18;
                            }
                        }
                        memcpy(
                            rbuf.offset(rbufpos as isize) as *mut libc::c_void,
                            buf.as_mut_ptr() as *const libc::c_void,
                            len_18 as libc::c_ulong,
                        );
                        rbufpos += len_18;
                        if width != 0 && justification < 0 as libc::c_int {
                            while nspace_18 != 0 {
                                let fresh55 = rbufpos;
                                rbufpos = rbufpos + 1;
                                *rbuf.offset(fresh55 as isize) = ' ' as i32 as libc::c_char;
                                nspace_18 -= 1;
                                nspace_18;
                            }
                        }
                    }
                    69 => {
                        sprintf(
                            buf.as_mut_ptr(),
                            b"%02d/%02d/%02d\0" as *const u8 as *const libc::c_char,
                            mod_tm.tm_year % 100 as libc::c_int,
                            mod_tm.tm_mon + 1 as libc::c_int,
                            mod_tm.tm_mday,
                        );
                        let mut len_19: libc::c_int = strlen(buf.as_mut_ptr())
                            as libc::c_int;
                        let mut nspace_19: libc::c_int = 0;
                        if len_19 > width {
                            nspace_19 = 0 as libc::c_int;
                        } else {
                            nspace_19 = width - len_19;
                        }
                        if rbufpos + (nspace_19 + len_19) >= rbuflen {
                            rbuflen += nspace_19 + len_19 + 1024 as libc::c_int;
                            rbuf = xrealloc(rbuf as *mut libc::c_void, rbuflen as size_t)
                                as *mut libc::c_char;
                        }
                        if width != 0 && justification > 0 as libc::c_int {
                            while nspace_19 != 0 {
                                let fresh56 = rbufpos;
                                rbufpos = rbufpos + 1;
                                *rbuf.offset(fresh56 as isize) = ' ' as i32 as libc::c_char;
                                nspace_19 -= 1;
                                nspace_19;
                            }
                        }
                        memcpy(
                            rbuf.offset(rbufpos as isize) as *mut libc::c_void,
                            buf.as_mut_ptr() as *const libc::c_void,
                            len_19 as libc::c_ulong,
                        );
                        rbufpos += len_19;
                        if width != 0 && justification < 0 as libc::c_int {
                            while nspace_19 != 0 {
                                let fresh57 = rbufpos;
                                rbufpos = rbufpos + 1;
                                *rbuf.offset(fresh57 as isize) = ' ' as i32 as libc::c_char;
                                nspace_19 -= 1;
                                nspace_19;
                            }
                        }
                    }
                    70 => {
                        sprintf(
                            buf.as_mut_ptr(),
                            b"%d.%d.%d\0" as *const u8 as *const libc::c_char,
                            mod_tm.tm_mday,
                            mod_tm.tm_mon + 1 as libc::c_int,
                            mod_tm.tm_year + 1900 as libc::c_int,
                        );
                        let mut len_20: libc::c_int = strlen(buf.as_mut_ptr())
                            as libc::c_int;
                        let mut nspace_20: libc::c_int = 0;
                        if len_20 > width {
                            nspace_20 = 0 as libc::c_int;
                        } else {
                            nspace_20 = width - len_20;
                        }
                        if rbufpos + (nspace_20 + len_20) >= rbuflen {
                            rbuflen += nspace_20 + len_20 + 1024 as libc::c_int;
                            rbuf = xrealloc(rbuf as *mut libc::c_void, rbuflen as size_t)
                                as *mut libc::c_char;
                        }
                        if width != 0 && justification > 0 as libc::c_int {
                            while nspace_20 != 0 {
                                let fresh58 = rbufpos;
                                rbufpos = rbufpos + 1;
                                *rbuf.offset(fresh58 as isize) = ' ' as i32 as libc::c_char;
                                nspace_20 -= 1;
                                nspace_20;
                            }
                        }
                        memcpy(
                            rbuf.offset(rbufpos as isize) as *mut libc::c_void,
                            buf.as_mut_ptr() as *const libc::c_void,
                            len_20 as libc::c_ulong,
                        );
                        rbufpos += len_20;
                        if width != 0 && justification < 0 as libc::c_int {
                            while nspace_20 != 0 {
                                let fresh59 = rbufpos;
                                rbufpos = rbufpos + 1;
                                *rbuf.offset(fresh59 as isize) = ' ' as i32 as libc::c_char;
                                nspace_20 -= 1;
                                nspace_20;
                            }
                        }
                    }
                    116 => {
                        sprintf(
                            buf.as_mut_ptr(),
                            b"%d:%d%s\0" as *const u8 as *const libc::c_char,
                            if mod_tm.tm_hour > 12 as libc::c_int {
                                mod_tm.tm_hour - 12 as libc::c_int
                            } else {
                                mod_tm.tm_hour
                            },
                            mod_tm.tm_min,
                            if mod_tm.tm_hour > 12 as libc::c_int {
                                b"pm\0" as *const u8 as *const libc::c_char
                            } else {
                                b"am\0" as *const u8 as *const libc::c_char
                            },
                        );
                        let mut len_21: libc::c_int = strlen(buf.as_mut_ptr())
                            as libc::c_int;
                        let mut nspace_21: libc::c_int = 0;
                        if len_21 > width {
                            nspace_21 = 0 as libc::c_int;
                        } else {
                            nspace_21 = width - len_21;
                        }
                        if rbufpos + (nspace_21 + len_21) >= rbuflen {
                            rbuflen += nspace_21 + len_21 + 1024 as libc::c_int;
                            rbuf = xrealloc(rbuf as *mut libc::c_void, rbuflen as size_t)
                                as *mut libc::c_char;
                        }
                        if width != 0 && justification > 0 as libc::c_int {
                            while nspace_21 != 0 {
                                let fresh60 = rbufpos;
                                rbufpos = rbufpos + 1;
                                *rbuf.offset(fresh60 as isize) = ' ' as i32 as libc::c_char;
                                nspace_21 -= 1;
                                nspace_21;
                            }
                        }
                        memcpy(
                            rbuf.offset(rbufpos as isize) as *mut libc::c_void,
                            buf.as_mut_ptr() as *const libc::c_void,
                            len_21 as libc::c_ulong,
                        );
                        rbufpos += len_21;
                        if width != 0 && justification < 0 as libc::c_int {
                            while nspace_21 != 0 {
                                let fresh61 = rbufpos;
                                rbufpos = rbufpos + 1;
                                *rbuf.offset(fresh61 as isize) = ' ' as i32 as libc::c_char;
                                nspace_21 -= 1;
                                nspace_21;
                            }
                        }
                    }
                    84 => {
                        sprintf(
                            buf.as_mut_ptr(),
                            b"%d:%d\0" as *const u8 as *const libc::c_char,
                            mod_tm.tm_hour,
                            mod_tm.tm_min,
                        );
                        let mut len_22: libc::c_int = strlen(buf.as_mut_ptr())
                            as libc::c_int;
                        let mut nspace_22: libc::c_int = 0;
                        if len_22 > width {
                            nspace_22 = 0 as libc::c_int;
                        } else {
                            nspace_22 = width - len_22;
                        }
                        if rbufpos + (nspace_22 + len_22) >= rbuflen {
                            rbuflen += nspace_22 + len_22 + 1024 as libc::c_int;
                            rbuf = xrealloc(rbuf as *mut libc::c_void, rbuflen as size_t)
                                as *mut libc::c_char;
                        }
                        if width != 0 && justification > 0 as libc::c_int {
                            while nspace_22 != 0 {
                                let fresh62 = rbufpos;
                                rbufpos = rbufpos + 1;
                                *rbuf.offset(fresh62 as isize) = ' ' as i32 as libc::c_char;
                                nspace_22 -= 1;
                                nspace_22;
                            }
                        }
                        memcpy(
                            rbuf.offset(rbufpos as isize) as *mut libc::c_void,
                            buf.as_mut_ptr() as *const libc::c_void,
                            len_22 as libc::c_ulong,
                        );
                        rbufpos += len_22;
                        if width != 0 && justification < 0 as libc::c_int {
                            while nspace_22 != 0 {
                                let fresh63 = rbufpos;
                                rbufpos = rbufpos + 1;
                                *rbuf.offset(fresh63 as isize) = ' ' as i32 as libc::c_char;
                                nspace_22 -= 1;
                                nspace_22;
                            }
                        }
                    }
                    42 => {
                        sprintf(
                            buf.as_mut_ptr(),
                            b"%d:%d:%d\0" as *const u8 as *const libc::c_char,
                            mod_tm.tm_hour,
                            mod_tm.tm_min,
                            mod_tm.tm_sec,
                        );
                        let mut len_23: libc::c_int = strlen(buf.as_mut_ptr())
                            as libc::c_int;
                        let mut nspace_23: libc::c_int = 0;
                        if len_23 > width {
                            nspace_23 = 0 as libc::c_int;
                        } else {
                            nspace_23 = width - len_23;
                        }
                        if rbufpos + (nspace_23 + len_23) >= rbuflen {
                            rbuflen += nspace_23 + len_23 + 1024 as libc::c_int;
                            rbuf = xrealloc(rbuf as *mut libc::c_void, rbuflen as size_t)
                                as *mut libc::c_char;
                        }
                        if width != 0 && justification > 0 as libc::c_int {
                            while nspace_23 != 0 {
                                let fresh64 = rbufpos;
                                rbufpos = rbufpos + 1;
                                *rbuf.offset(fresh64 as isize) = ' ' as i32 as libc::c_char;
                                nspace_23 -= 1;
                                nspace_23;
                            }
                        }
                        memcpy(
                            rbuf.offset(rbufpos as isize) as *mut libc::c_void,
                            buf.as_mut_ptr() as *const libc::c_void,
                            len_23 as libc::c_ulong,
                        );
                        rbufpos += len_23;
                        if width != 0 && justification < 0 as libc::c_int {
                            while nspace_23 != 0 {
                                let fresh65 = rbufpos;
                                rbufpos = rbufpos + 1;
                                *rbuf.offset(fresh65 as isize) = ' ' as i32 as libc::c_char;
                                nspace_23 -= 1;
                                nspace_23;
                            }
                        }
                    }
                    118 => {
                        sprintf(
                            buf.as_mut_ptr(),
                            b"%d\0" as *const u8 as *const libc::c_char,
                            input_filenum,
                        );
                        let mut len_24: libc::c_int = strlen(buf.as_mut_ptr())
                            as libc::c_int;
                        let mut nspace_24: libc::c_int = 0;
                        if len_24 > width {
                            nspace_24 = 0 as libc::c_int;
                        } else {
                            nspace_24 = width - len_24;
                        }
                        if rbufpos + (nspace_24 + len_24) >= rbuflen {
                            rbuflen += nspace_24 + len_24 + 1024 as libc::c_int;
                            rbuf = xrealloc(rbuf as *mut libc::c_void, rbuflen as size_t)
                                as *mut libc::c_char;
                        }
                        if width != 0 && justification > 0 as libc::c_int {
                            while nspace_24 != 0 {
                                let fresh66 = rbufpos;
                                rbufpos = rbufpos + 1;
                                *rbuf.offset(fresh66 as isize) = ' ' as i32 as libc::c_char;
                                nspace_24 -= 1;
                                nspace_24;
                            }
                        }
                        memcpy(
                            rbuf.offset(rbufpos as isize) as *mut libc::c_void,
                            buf.as_mut_ptr() as *const libc::c_void,
                            len_24 as libc::c_ulong,
                        );
                        rbufpos += len_24;
                        if width != 0 && justification < 0 as libc::c_int {
                            while nspace_24 != 0 {
                                let fresh67 = rbufpos;
                                rbufpos = rbufpos + 1;
                                *rbuf.offset(fresh67 as isize) = ' ' as i32 as libc::c_char;
                                nspace_24 -= 1;
                                nspace_24;
                            }
                        }
                    }
                    86 => {
                        if toc != 0 {
                            sprintf(
                                buf.as_mut_ptr(),
                                b"%d-\0" as *const u8 as *const libc::c_char,
                                input_filenum,
                            );
                            let mut len_25: libc::c_int = strlen(buf.as_mut_ptr())
                                as libc::c_int;
                            let mut nspace_25: libc::c_int = 0;
                            if len_25 > width {
                                nspace_25 = 0 as libc::c_int;
                            } else {
                                nspace_25 = width - len_25;
                            }
                            if rbufpos + (nspace_25 + len_25) >= rbuflen {
                                rbuflen += nspace_25 + len_25 + 1024 as libc::c_int;
                                rbuf = xrealloc(
                                    rbuf as *mut libc::c_void,
                                    rbuflen as size_t,
                                ) as *mut libc::c_char;
                            }
                            if width != 0 && justification > 0 as libc::c_int {
                                while nspace_25 != 0 {
                                    let fresh68 = rbufpos;
                                    rbufpos = rbufpos + 1;
                                    *rbuf.offset(fresh68 as isize) = ' ' as i32 as libc::c_char;
                                    nspace_25 -= 1;
                                    nspace_25;
                                }
                            }
                            memcpy(
                                rbuf.offset(rbufpos as isize) as *mut libc::c_void,
                                buf.as_mut_ptr() as *const libc::c_void,
                                len_25 as libc::c_ulong,
                            );
                            rbufpos += len_25;
                            if width != 0 && justification < 0 as libc::c_int {
                                while nspace_25 != 0 {
                                    let fresh69 = rbufpos;
                                    rbufpos = rbufpos + 1;
                                    *rbuf.offset(fresh69 as isize) = ' ' as i32 as libc::c_char;
                                    nspace_25 -= 1;
                                    nspace_25;
                                }
                            }
                        }
                    }
                    87 => {
                        sprintf(
                            buf.as_mut_ptr(),
                            b"%02d/%02d/%02d\0" as *const u8 as *const libc::c_char,
                            mod_tm.tm_mon + 1 as libc::c_int,
                            mod_tm.tm_mday,
                            mod_tm.tm_year % 100 as libc::c_int,
                        );
                        let mut len_26: libc::c_int = strlen(buf.as_mut_ptr())
                            as libc::c_int;
                        let mut nspace_26: libc::c_int = 0;
                        if len_26 > width {
                            nspace_26 = 0 as libc::c_int;
                        } else {
                            nspace_26 = width - len_26;
                        }
                        if rbufpos + (nspace_26 + len_26) >= rbuflen {
                            rbuflen += nspace_26 + len_26 + 1024 as libc::c_int;
                            rbuf = xrealloc(rbuf as *mut libc::c_void, rbuflen as size_t)
                                as *mut libc::c_char;
                        }
                        if width != 0 && justification > 0 as libc::c_int {
                            while nspace_26 != 0 {
                                let fresh70 = rbufpos;
                                rbufpos = rbufpos + 1;
                                *rbuf.offset(fresh70 as isize) = ' ' as i32 as libc::c_char;
                                nspace_26 -= 1;
                                nspace_26;
                            }
                        }
                        memcpy(
                            rbuf.offset(rbufpos as isize) as *mut libc::c_void,
                            buf.as_mut_ptr() as *const libc::c_void,
                            len_26 as libc::c_ulong,
                        );
                        rbufpos += len_26;
                        if width != 0 && justification < 0 as libc::c_int {
                            while nspace_26 != 0 {
                                let fresh71 = rbufpos;
                                rbufpos = rbufpos + 1;
                                *rbuf.offset(fresh71 as isize) = ' ' as i32 as libc::c_char;
                                nspace_26 -= 1;
                                nspace_26;
                            }
                        }
                    }
                    78 => {
                        let mut len_27: libc::c_int = strlen(fname) as libc::c_int;
                        let mut nspace_27: libc::c_int = 0;
                        if len_27 > width {
                            nspace_27 = 0 as libc::c_int;
                        } else {
                            nspace_27 = width - len_27;
                        }
                        if rbufpos + (nspace_27 + len_27) >= rbuflen {
                            rbuflen += nspace_27 + len_27 + 1024 as libc::c_int;
                            rbuf = xrealloc(rbuf as *mut libc::c_void, rbuflen as size_t)
                                as *mut libc::c_char;
                        }
                        if width != 0 && justification > 0 as libc::c_int {
                            while nspace_27 != 0 {
                                let fresh72 = rbufpos;
                                rbufpos = rbufpos + 1;
                                *rbuf.offset(fresh72 as isize) = ' ' as i32 as libc::c_char;
                                nspace_27 -= 1;
                                nspace_27;
                            }
                        }
                        memcpy(
                            rbuf.offset(rbufpos as isize) as *mut libc::c_void,
                            fname as *const libc::c_void,
                            len_27 as libc::c_ulong,
                        );
                        rbufpos += len_27;
                        if width != 0 && justification < 0 as libc::c_int {
                            while nspace_27 != 0 {
                                let fresh73 = rbufpos;
                                rbufpos = rbufpos + 1;
                                *rbuf.offset(fresh73 as isize) = ' ' as i32 as libc::c_char;
                                nspace_27 -= 1;
                                nspace_27;
                            }
                        }
                    }
                    110 => {
                        cp = strrchr(fname, '/' as i32);
                        if !cp.is_null() {
                            cp = cp.offset(1);
                            cp;
                        } else {
                            cp = fname;
                        }
                        let mut len_28: libc::c_int = strlen(cp) as libc::c_int;
                        let mut nspace_28: libc::c_int = 0;
                        if len_28 > width {
                            nspace_28 = 0 as libc::c_int;
                        } else {
                            nspace_28 = width - len_28;
                        }
                        if rbufpos + (nspace_28 + len_28) >= rbuflen {
                            rbuflen += nspace_28 + len_28 + 1024 as libc::c_int;
                            rbuf = xrealloc(rbuf as *mut libc::c_void, rbuflen as size_t)
                                as *mut libc::c_char;
                        }
                        if width != 0 && justification > 0 as libc::c_int {
                            while nspace_28 != 0 {
                                let fresh74 = rbufpos;
                                rbufpos = rbufpos + 1;
                                *rbuf.offset(fresh74 as isize) = ' ' as i32 as libc::c_char;
                                nspace_28 -= 1;
                                nspace_28;
                            }
                        }
                        memcpy(
                            rbuf.offset(rbufpos as isize) as *mut libc::c_void,
                            cp as *const libc::c_void,
                            len_28 as libc::c_ulong,
                        );
                        rbufpos += len_28;
                        if width != 0 && justification < 0 as libc::c_int {
                            while nspace_28 != 0 {
                                let fresh75 = rbufpos;
                                rbufpos = rbufpos + 1;
                                *rbuf.offset(fresh75 as isize) = ' ' as i32 as libc::c_char;
                                nspace_28 -= 1;
                                nspace_28;
                            }
                        }
                    }
                    76 => {
                        sprintf(
                            buf.as_mut_ptr(),
                            b"%d\0" as *const u8 as *const libc::c_char,
                            current_file_linenum
                                .wrapping_sub(1 as libc::c_int as libc::c_uint),
                        );
                        let mut len_29: libc::c_int = strlen(buf.as_mut_ptr())
                            as libc::c_int;
                        let mut nspace_29: libc::c_int = 0;
                        if len_29 > width {
                            nspace_29 = 0 as libc::c_int;
                        } else {
                            nspace_29 = width - len_29;
                        }
                        if rbufpos + (nspace_29 + len_29) >= rbuflen {
                            rbuflen += nspace_29 + len_29 + 1024 as libc::c_int;
                            rbuf = xrealloc(rbuf as *mut libc::c_void, rbuflen as size_t)
                                as *mut libc::c_char;
                        }
                        if width != 0 && justification > 0 as libc::c_int {
                            while nspace_29 != 0 {
                                let fresh76 = rbufpos;
                                rbufpos = rbufpos + 1;
                                *rbuf.offset(fresh76 as isize) = ' ' as i32 as libc::c_char;
                                nspace_29 -= 1;
                                nspace_29;
                            }
                        }
                        memcpy(
                            rbuf.offset(rbufpos as isize) as *mut libc::c_void,
                            buf.as_mut_ptr() as *const libc::c_void,
                            len_29 as libc::c_ulong,
                        );
                        rbufpos += len_29;
                        if width != 0 && justification < 0 as libc::c_int {
                            while nspace_29 != 0 {
                                let fresh77 = rbufpos;
                                rbufpos = rbufpos + 1;
                                *rbuf.offset(fresh77 as isize) = ' ' as i32 as libc::c_char;
                                nspace_29 -= 1;
                                nspace_29;
                            }
                        }
                    }
                    _ => {
                        fprintf(
                            stderr,
                            b"%s: \0" as *const u8 as *const libc::c_char,
                            program,
                        );
                        fprintf(
                            stderr,
                            dcgettext(
                                0 as *const libc::c_char,
                                b"%s: unknown `$' escape `%c' (%d)\0" as *const u8
                                    as *const libc::c_char,
                                5 as libc::c_int,
                            ),
                            context_name,
                            *str.offset(i as isize) as libc::c_int,
                            *str.offset(i as isize) as libc::c_int,
                        );
                        fprintf(stderr, b"\n\0" as *const u8 as *const libc::c_char);
                        fflush(stderr);
                        exit(1 as libc::c_int);
                    }
                }
            }
            width = 0 as libc::c_int;
            justification = 1 as libc::c_int;
        } else {
            let mut a_2: libc::c_int = 0;
            if rbufpos + width >= rbuflen {
                rbuflen += width + 1024 as libc::c_int;
                rbuf = xrealloc(rbuf as *mut libc::c_void, rbuflen as size_t)
                    as *mut libc::c_char;
            }
            if width != 0 && justification < 0 as libc::c_int {
                let fresh78 = rbufpos;
                rbufpos = rbufpos + 1;
                *rbuf.offset(fresh78 as isize) = *str.offset(i as isize);
            }
            a_2 = 0 as libc::c_int;
            while a_2 < width - 1 as libc::c_int {
                let fresh79 = rbufpos;
                rbufpos = rbufpos + 1;
                *rbuf.offset(fresh79 as isize) = ' ' as i32 as libc::c_char;
                a_2 += 1;
                a_2;
            }
            if width == 0 || justification > 0 as libc::c_int {
                let fresh80 = rbufpos;
                rbufpos = rbufpos + 1;
                *rbuf.offset(fresh80 as isize) = *str.offset(i as isize);
            }
        }
        i += 1;
        i;
    }
    let mut a_3: libc::c_int = 0;
    if rbufpos + width >= rbuflen {
        rbuflen += width + 1024 as libc::c_int;
        rbuf = xrealloc(rbuf as *mut libc::c_void, rbuflen as size_t)
            as *mut libc::c_char;
    }
    if width != 0 && justification < 0 as libc::c_int {
        let fresh81 = rbufpos;
        rbufpos = rbufpos + 1;
        *rbuf.offset(fresh81 as isize) = '\0' as i32 as libc::c_char;
    }
    a_3 = 0 as libc::c_int;
    while a_3 < width - 1 as libc::c_int {
        let fresh82 = rbufpos;
        rbufpos = rbufpos + 1;
        *rbuf.offset(fresh82 as isize) = ' ' as i32 as libc::c_char;
        a_3 += 1;
        a_3;
    }
    if width == 0 || justification > 0 as libc::c_int {
        let fresh83 = rbufpos;
        rbufpos = rbufpos + 1;
        *rbuf.offset(fresh83 as isize) = '\0' as i32 as libc::c_char;
    }
    cp = escape_string(rbuf);
    xfree(rbuf as *mut libc::c_void);
    return cp;
}
pub unsafe extern "C" fn parse_key_value_pair(
    mut set: StringHashPtr,
    mut kv: *mut libc::c_char,
) {
    let mut cp: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut key: Buffer = Buffer {
        data: 0 as *mut libc::c_char,
        allocated: 0,
        len: 0,
    };
    cp = strchr(kv, ':' as i32);
    if cp.is_null() {
        if strhash_delete(
            set,
            kv,
            (strlen(kv)).wrapping_add(1 as libc::c_int as libc::c_ulong) as libc::c_int,
            &mut cp as *mut *mut libc::c_char as *mut *mut libc::c_void,
        ) != 0
        {
            xfree(cp as *mut libc::c_void);
        }
    } else {
        buffer_init(&mut key);
        buffer_append_len(&mut key, kv, cp.offset_from(kv) as libc::c_long as size_t);
        strhash_put(
            set,
            buffer_ptr(&mut key),
            (strlen(buffer_ptr(&mut key)))
                .wrapping_add(1 as libc::c_int as libc::c_ulong) as libc::c_int,
            xstrdup(cp.offset(1 as libc::c_int as isize)) as *mut libc::c_void,
            &mut cp as *mut *mut libc::c_char as *mut *mut libc::c_void,
        );
        if !cp.is_null() {
            xfree(cp as *mut libc::c_void);
        }
        buffer_uninit(&mut key);
    };
}
pub unsafe extern "C" fn count_key_value_set(mut set: StringHashPtr) -> libc::c_int {
    let mut i: libc::c_int = 0 as libc::c_int;
    let mut got: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    let mut cp: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut value: *mut libc::c_void = 0 as *mut libc::c_void;
    got = strhash_get_first(set, &mut cp, &mut j, &mut value);
    while got != 0 {
        i += 1;
        i;
        got = strhash_get_next(set, &mut cp, &mut j, &mut value);
    }
    return i;
}
pub unsafe extern "C" fn pathwalk(
    mut path: *mut libc::c_char,
    mut proc_0: PathWalkProc,
    mut context: *mut libc::c_void,
) -> libc::c_int {
    let mut buf: [libc::c_char; 512] = [0; 512];
    let mut cp: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut cp2: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut len: libc::c_int = 0;
    let mut i: libc::c_int = 0;
    cp = path;
    while !cp.is_null() {
        if cp != path {
            cp = cp.offset(1);
            cp;
        }
        cp2 = strchr(cp, ':' as i32);
        if !cp2.is_null() {
            len = cp2.offset_from(cp) as libc::c_long as libc::c_int;
        } else {
            len = strlen(cp) as libc::c_int;
        }
        memcpy(
            buf.as_mut_ptr() as *mut libc::c_void,
            cp as *const libc::c_void,
            len as libc::c_ulong,
        );
        buf[len as usize] = '\0' as i32 as libc::c_char;
        i = (Some(proc_0.unwrap())).unwrap()(buf.as_mut_ptr(), context);
        if i != 0 as libc::c_int {
            return i;
        }
        cp = strchr(cp, ':' as i32);
    }
    return 0 as libc::c_int;
}
pub unsafe extern "C" fn file_lookup(
    mut path: *mut libc::c_char,
    mut context: *mut libc::c_void,
) -> libc::c_int {
    let mut len: libc::c_int = 0;
    let mut ctx: *mut FileLookupCtx = context as *mut FileLookupCtx;
    let mut stat_st: stat = stat {
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
    let mut i: libc::c_int = 0;
    if quiet == 0 && verbose >= 2 as libc::c_int {
        fprintf(
            stderr,
            b"file_lookup(): %s/%s%s\t\0" as *const u8 as *const libc::c_char,
            path,
            (*ctx).name,
            (*ctx).suffix,
        );
    }
    len = strlen(path) as libc::c_int;
    if len != 0
        && *path.offset((len - 1 as libc::c_int) as isize) as libc::c_int == '/' as i32
    {
        len -= 1;
        len;
    }
    buffer_clear((*ctx).fullname);
    buffer_append_len((*ctx).fullname, path, len as size_t);
    buffer_append((*ctx).fullname, b"/\0" as *const u8 as *const libc::c_char);
    buffer_append((*ctx).fullname, (*ctx).name);
    buffer_append((*ctx).fullname, (*ctx).suffix);
    i = (stat(buffer_ptr((*ctx).fullname), &mut stat_st) == 0 as libc::c_int)
        as libc::c_int;
    if quiet == 0 && verbose >= 2 as libc::c_int {
        fprintf(
            stderr,
            b"#%c\n\0" as *const u8 as *const libc::c_char,
            if i != 0 { 't' as i32 } else { 'f' as i32 },
        );
    }
    return i;
}
pub unsafe extern "C" fn tilde_subst(
    mut fname_0: *mut libc::c_char,
) -> *mut libc::c_char {
    let mut cp: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut i: libc::c_int = 0;
    let mut pswd: *mut passwd = 0 as *mut passwd;
    let mut buffer: Buffer = Buffer {
        data: 0 as *mut libc::c_char,
        allocated: 0,
        len: 0,
    };
    let mut result: *mut libc::c_char = 0 as *mut libc::c_char;
    if *fname_0.offset(0 as libc::c_int as isize) as libc::c_int != '~' as i32 {
        return xstrdup(fname_0);
    }
    if *fname_0.offset(1 as libc::c_int as isize) as libc::c_int == '/' as i32
        || *fname_0.offset(1 as libc::c_int as isize) as libc::c_int == '\0' as i32
    {
        cp = getenv(b"HOME\0" as *const u8 as *const libc::c_char);
        if cp.is_null() {
            return xstrdup(fname_0);
        }
        buffer_init(&mut buffer);
        buffer_append(&mut buffer, cp);
        buffer_append(&mut buffer, fname_0.offset(1 as libc::c_int as isize));
        result = buffer_copy(&mut buffer);
        buffer_uninit(&mut buffer);
        return result;
    }
    i = 1 as libc::c_int;
    while *fname_0.offset(i as isize) as libc::c_int != 0
        && *fname_0.offset(i as isize) as libc::c_int != '/' as i32
    {
        i += 1;
        i;
    }
    buffer_init(&mut buffer);
    buffer_append_len(
        &mut buffer,
        fname_0.offset(1 as libc::c_int as isize),
        (i - 1 as libc::c_int) as size_t,
    );
    pswd = getpwnam(buffer_ptr(&mut buffer));
    buffer_uninit(&mut buffer);
    if !pswd.is_null() {
        buffer_init(&mut buffer);
        buffer_append(&mut buffer, (*pswd).pw_dir);
        buffer_append(&mut buffer, fname_0.offset(i as isize));
        result = buffer_copy(&mut buffer);
        buffer_uninit(&mut buffer);
        return result;
    }
    return xstrdup(fname_0);
}
pub unsafe extern "C" fn parse_float(
    mut string: *mut libc::c_char,
    mut units: libc::c_int,
    mut horizontal: libc::c_int,
) -> libc::c_double {
    let mut current_block: u64;
    let mut val: libc::c_double = 0.;
    let mut end: *mut libc::c_char = 0 as *mut libc::c_char;
    val = strtod(string, &mut end);
    if end == string {
        current_block = 1411063431811260509;
    } else {
        current_block = 4906268039856690917;
    }
    loop {
        match current_block {
            1411063431811260509 => {
                fprintf(stderr, b"%s: \0" as *const u8 as *const libc::c_char, program);
                fprintf(
                    stderr,
                    dcgettext(
                        0 as *const libc::c_char,
                        b"malformed float dimension: \"%s\"\0" as *const u8
                            as *const libc::c_char,
                        5 as libc::c_int,
                    ),
                    string,
                );
                fprintf(stderr, b"\n\0" as *const u8 as *const libc::c_char);
                fflush(stderr);
                current_block = 4906268039856690917;
            }
            _ => {
                if units != 0 {
                    match *end as libc::c_int {
                        99 => {
                            val *= 72 as libc::c_int as libc::c_double / 2.54f64;
                            break;
                        }
                        112 => {
                            break;
                        }
                        105 => {
                            val *= 72 as libc::c_int as libc::c_double;
                            break;
                        }
                        0 | 108 => {
                            if horizontal != 0 {
                                val
                                    *= *font_widths
                                        .as_mut_ptr()
                                        .offset('m' as i32 as libc::c_uchar as isize);
                            } else {
                                val *= Fpt.h + baselineskip;
                            }
                            break;
                        }
                        _ => {
                            current_block = 1411063431811260509;
                        }
                    }
                } else if *end as libc::c_int != '\0' as i32 {
                    current_block = 1411063431811260509;
                } else {
                    break;
                }
            }
        }
    }
    return val;
}
pub unsafe extern "C" fn is_open(
    mut is: *mut InputStream,
    mut fp: *mut FILE,
    mut fname_0: *mut libc::c_char,
    mut input_filter: *mut libc::c_char,
) -> libc::c_int {
    (*is).data_in_buf = 0 as libc::c_int as libc::c_uint;
    (*is).bufpos = 0 as libc::c_int as libc::c_uint;
    (*is).nreads = 0 as libc::c_int as libc::c_uint;
    (*is).unget_ch = 0 as *mut libc::c_uchar;
    (*is).unget_pos = 0 as libc::c_int as libc::c_uint;
    (*is).unget_alloc = 0 as libc::c_int as libc::c_uint;
    if !input_filter.is_null() {
        let mut cmd: *mut libc::c_char = 0 as *mut libc::c_char;
        let mut cmdlen: libc::c_int = 0;
        let mut i: libc::c_int = 0;
        let mut pos: libc::c_int = 0;
        let mut cp: *mut libc::c_char = 0 as *mut libc::c_char;
        (*is).is_pipe = 1 as libc::c_int;
        if fname_0.is_null() {
            fname_0 = input_filter_stdin;
        }
        cmdlen = (strlen(input_filter)).wrapping_add(1 as libc::c_int as libc::c_ulong)
            as libc::c_int;
        cmd = xmalloc(cmdlen as size_t) as *mut libc::c_char;
        pos = 0 as libc::c_int;
        i = 0 as libc::c_int;
        while *input_filter.offset(i as isize) != 0 {
            if *input_filter.offset(i as isize) as libc::c_int == '%' as i32 {
                match *input_filter.offset((i + 1 as libc::c_int) as isize)
                    as libc::c_int
                {
                    115 => {
                        cp = shell_escape(fname_0);
                        if !cp.is_null() {
                            cmdlen = (cmdlen as libc::c_ulong).wrapping_add(strlen(cp))
                                as libc::c_int as libc::c_int;
                            cmd = xrealloc(cmd as *mut libc::c_void, cmdlen as size_t)
                                as *mut libc::c_char;
                            strcpy(cmd.offset(pos as isize), cp);
                            pos = (pos as libc::c_ulong).wrapping_add(strlen(cp))
                                as libc::c_int as libc::c_int;
                            free(cp as *mut libc::c_void);
                        }
                        i += 1;
                        i;
                    }
                    37 => {
                        let fresh84 = pos;
                        pos = pos + 1;
                        *cmd.offset(fresh84 as isize) = '%' as i32 as libc::c_char;
                        i += 1;
                        i;
                    }
                    _ => {
                        let fresh85 = pos;
                        pos = pos + 1;
                        *cmd.offset(fresh85 as isize) = *input_filter.offset(i as isize);
                    }
                }
            } else {
                let fresh86 = pos;
                pos = pos + 1;
                *cmd.offset(fresh86 as isize) = *input_filter.offset(i as isize);
            }
            i += 1;
            i;
        }
        let fresh87 = pos;
        pos = pos + 1;
        *cmd.offset(fresh87 as isize) = '\0' as i32 as libc::c_char;
        (*is).fp = popen(cmd, b"r\0" as *const u8 as *const libc::c_char);
        xfree(cmd as *mut libc::c_void);
        if ((*is).fp).is_null() {
            fprintf(stderr, b"%s: \0" as *const u8 as *const libc::c_char, program);
            fprintf(
                stderr,
                dcgettext(
                    0 as *const libc::c_char,
                    b"couldn't open input filter \"%s\" for file \"%s\": %s\0"
                        as *const u8 as *const libc::c_char,
                    5 as libc::c_int,
                ),
                input_filter,
                if !fname_0.is_null() {
                    fname_0 as *const libc::c_char
                } else {
                    b"(stdin)\0" as *const u8 as *const libc::c_char
                },
                strerror(*__errno_location()),
            );
            fprintf(stderr, b"\n\0" as *const u8 as *const libc::c_char);
            fflush(stderr);
            return 0 as libc::c_int;
        }
    } else {
        (*is).is_pipe = 0 as libc::c_int;
        if !fp.is_null() {
            (*is).fp = fp;
        } else {
            (*is).fp = fopen(fname_0, b"rb\0" as *const u8 as *const libc::c_char);
            if ((*is).fp).is_null() {
                fprintf(stderr, b"%s: \0" as *const u8 as *const libc::c_char, program);
                fprintf(
                    stderr,
                    dcgettext(
                        0 as *const libc::c_char,
                        b"couldn't open input file \"%s\": %s\0" as *const u8
                            as *const libc::c_char,
                        5 as libc::c_int,
                    ),
                    fname_0,
                    strerror(*__errno_location()),
                );
                fprintf(stderr, b"\n\0" as *const u8 as *const libc::c_char);
                fflush(stderr);
                return 0 as libc::c_int;
            }
        }
    }
    return 1 as libc::c_int;
}
pub unsafe extern "C" fn is_close(mut is: *mut InputStream) {
    if (*is).is_pipe != 0 {
        pclose((*is).fp);
    } else {
        fclose((*is).fp);
    }
    if !((*is).unget_ch).is_null() {
        xfree((*is).unget_ch as *mut libc::c_void);
    }
}
pub unsafe extern "C" fn is_getc(mut is: *mut InputStream) -> libc::c_int {
    let mut ch: libc::c_int = 0;
    if (*is).unget_pos > 0 as libc::c_int as libc::c_uint {
        (*is).unget_pos = ((*is).unget_pos).wrapping_sub(1);
        ch = *((*is).unget_ch).offset((*is).unget_pos as isize) as libc::c_int;
        return ch;
    }
    while (*is).bufpos >= (*is).data_in_buf {
        if (*is).nreads > 0 as libc::c_int as libc::c_uint
            && ((*is).data_in_buf as libc::c_ulong)
                < ::std::mem::size_of::<[libc::c_uchar; 4096]>() as libc::c_ulong
        {
            return -(1 as libc::c_int);
        }
        (*is)
            .data_in_buf = fread(
            ((*is).buf).as_mut_ptr() as *mut libc::c_void,
            1 as libc::c_int as libc::c_ulong,
            ::std::mem::size_of::<[libc::c_uchar; 4096]>() as libc::c_ulong,
            (*is).fp,
        ) as libc::c_uint;
        (*is).bufpos = 0 as libc::c_int as libc::c_uint;
        (*is).nreads = ((*is).nreads).wrapping_add(1);
        (*is).nreads;
    }
    let fresh88 = (*is).bufpos;
    (*is).bufpos = ((*is).bufpos).wrapping_add(1);
    return (*is).buf[fresh88 as usize] as libc::c_int;
}
pub unsafe extern "C" fn is_ungetc(
    mut ch: libc::c_int,
    mut is: *mut InputStream,
) -> libc::c_int {
    if (*is).unget_pos >= (*is).unget_alloc {
        (*is)
            .unget_alloc = ((*is).unget_alloc)
            .wrapping_add(1024 as libc::c_int as libc::c_uint);
        (*is)
            .unget_ch = xrealloc(
            (*is).unget_ch as *mut libc::c_void,
            (*is).unget_alloc as size_t,
        ) as *mut libc::c_uchar;
    }
    let fresh89 = (*is).unget_pos;
    (*is).unget_pos = ((*is).unget_pos).wrapping_add(1);
    *((*is).unget_ch).offset(fresh89 as isize) = ch as libc::c_uchar;
    return 1 as libc::c_int;
}
pub unsafe extern "C" fn buffer_init(mut buffer: *mut Buffer) {
    (*buffer).allocated = 128 as libc::c_int as size_t;
    (*buffer).data = xmalloc((*buffer).allocated) as *mut libc::c_char;
    *((*buffer).data).offset(0 as libc::c_int as isize) = '\0' as i32 as libc::c_char;
    (*buffer).len = 0 as libc::c_int as size_t;
}
pub unsafe extern "C" fn buffer_uninit(mut buffer: *mut Buffer) {
    xfree((*buffer).data as *mut libc::c_void);
}
pub unsafe extern "C" fn buffer_alloc() -> *mut Buffer {
    let mut buffer: *mut Buffer = xcalloc(
        1 as libc::c_int as size_t,
        ::std::mem::size_of::<Buffer>() as libc::c_ulong,
    ) as *mut Buffer;
    buffer_init(buffer);
    return buffer;
}
pub unsafe extern "C" fn buffer_free(mut buffer: *mut Buffer) {
    buffer_uninit(buffer);
    xfree(buffer as *mut libc::c_void);
}
pub unsafe extern "C" fn buffer_append(
    mut buffer: *mut Buffer,
    mut data: *const libc::c_char,
) {
    buffer_append_len(buffer, data, strlen(data));
}
pub unsafe extern "C" fn buffer_append_len(
    mut buffer: *mut Buffer,
    mut data: *const libc::c_char,
    mut len: size_t,
) {
    if ((*buffer).len).wrapping_add(len).wrapping_add(1 as libc::c_int as libc::c_ulong)
        >= (*buffer).allocated
    {
        (*buffer)
            .allocated = ((*buffer).len)
            .wrapping_add(len)
            .wrapping_add(1024 as libc::c_int as libc::c_ulong);
        (*buffer)
            .data = xrealloc((*buffer).data as *mut libc::c_void, (*buffer).allocated)
            as *mut libc::c_char;
    }
    memcpy(
        ((*buffer).data).offset((*buffer).len as isize) as *mut libc::c_void,
        data as *const libc::c_void,
        len,
    );
    (*buffer)
        .len = ((*buffer).len as libc::c_ulong).wrapping_add(len) as size_t as size_t;
    *((*buffer).data).offset((*buffer).len as isize) = '\0' as i32 as libc::c_char;
}
pub unsafe extern "C" fn buffer_copy(mut buffer: *mut Buffer) -> *mut libc::c_char {
    let mut copy: *mut libc::c_char = xmalloc(
        ((*buffer).len).wrapping_add(1 as libc::c_int as libc::c_ulong),
    ) as *mut libc::c_char;
    memcpy(
        copy as *mut libc::c_void,
        (*buffer).data as *const libc::c_void,
        ((*buffer).len).wrapping_add(1 as libc::c_int as libc::c_ulong),
    );
    return copy;
}
pub unsafe extern "C" fn buffer_clear(mut buffer: *mut Buffer) {
    (*buffer).len = 0 as libc::c_int as size_t;
    *((*buffer).data).offset(0 as libc::c_int as isize) = '\0' as i32 as libc::c_char;
}
pub unsafe extern "C" fn buffer_ptr(mut buffer: *mut Buffer) -> *mut libc::c_char {
    return (*buffer).data;
}
pub unsafe extern "C" fn buffer_len(mut buffer: *mut Buffer) -> size_t {
    return (*buffer).len;
}
pub unsafe extern "C" fn shell_escape(
    mut fn_0: *const libc::c_char,
) -> *mut libc::c_char {
    let mut len: size_t = 0 as libc::c_int as size_t;
    let mut inp: *const libc::c_char = 0 as *const libc::c_char;
    let mut retval: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut outp: *mut libc::c_char = 0 as *mut libc::c_char;
    inp = fn_0;
    while *inp != 0 {
        match *inp as libc::c_int {
            39 => {
                len = (len as libc::c_ulong)
                    .wrapping_add(4 as libc::c_int as libc::c_ulong) as size_t as size_t;
            }
            _ => {
                len = (len as libc::c_ulong)
                    .wrapping_add(1 as libc::c_int as libc::c_ulong) as size_t as size_t;
            }
        }
        inp = inp.offset(1);
        inp;
    }
    retval = malloc(len.wrapping_add(1 as libc::c_int as libc::c_ulong))
        as *mut libc::c_char;
    outp = retval;
    if outp.is_null() {
        return 0 as *mut libc::c_char;
    }
    inp = fn_0;
    while *inp != 0 {
        match *inp as libc::c_int {
            39 => {
                let fresh90 = outp;
                outp = outp.offset(1);
                *fresh90 = '\'' as i32 as libc::c_char;
                let fresh91 = outp;
                outp = outp.offset(1);
                *fresh91 = '\\' as i32 as libc::c_char;
                let fresh92 = outp;
                outp = outp.offset(1);
                *fresh92 = '\'' as i32 as libc::c_char;
                let fresh93 = outp;
                outp = outp.offset(1);
                *fresh93 = '\'' as i32 as libc::c_char;
            }
            _ => {
                let fresh94 = outp;
                outp = outp.offset(1);
                *fresh94 = *inp;
            }
        }
        inp = inp.offset(1);
        inp;
    }
    *outp = 0 as libc::c_int as libc::c_char;
    return retval;
}
