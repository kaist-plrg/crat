use ::libc;
extern "C" {
    pub type _IO_wide_data;
    pub type _IO_codecvt;
    pub type _IO_marker;
    pub type stringhash_st;
    static mut stderr: *mut FILE;
    fn tmpfile() -> *mut FILE;
    fn fclose(__stream: *mut FILE) -> libc::c_int;
    fn fflush(__stream: *mut FILE) -> libc::c_int;
    fn fopen(_: *const libc::c_char, _: *const libc::c_char) -> *mut FILE;
    fn fprintf(_: *mut FILE, _: *const libc::c_char, _: ...) -> libc::c_int;
    fn sprintf(_: *mut libc::c_char, _: *const libc::c_char, _: ...) -> libc::c_int;
    fn sscanf(_: *const libc::c_char, _: *const libc::c_char, _: ...) -> libc::c_int;
    fn fgets(
        __s: *mut libc::c_char,
        __n: libc::c_int,
        __stream: *mut FILE,
    ) -> *mut libc::c_char;
    fn fputs(__s: *const libc::c_char, __stream: *mut FILE) -> libc::c_int;
    fn fread(
        _: *mut libc::c_void,
        _: libc::c_ulong,
        _: libc::c_ulong,
        _: *mut FILE,
    ) -> libc::c_ulong;
    fn fwrite(
        _: *const libc::c_void,
        _: libc::c_ulong,
        _: libc::c_ulong,
        _: *mut FILE,
    ) -> libc::c_ulong;
    fn fseek(
        __stream: *mut FILE,
        __off: libc::c_long,
        __whence: libc::c_int,
    ) -> libc::c_int;
    fn pclose(__stream: *mut FILE) -> libc::c_int;
    fn strtod(_: *const libc::c_char, _: *mut *mut libc::c_char) -> libc::c_double;
    fn strtol(
        _: *const libc::c_char,
        _: *mut *mut libc::c_char,
        _: libc::c_int,
    ) -> libc::c_long;
    fn abort() -> !;
    fn exit(_: libc::c_int) -> !;
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
    fn strdup(_: *const libc::c_char) -> *mut libc::c_char;
    fn strchr(_: *const libc::c_char, _: libc::c_int) -> *mut libc::c_char;
    fn strrchr(_: *const libc::c_char, _: libc::c_int) -> *mut libc::c_char;
    fn strlen(_: *const libc::c_char) -> libc::c_ulong;
    fn strerror(_: libc::c_int) -> *mut libc::c_char;
    fn tilde_subst(fname_0: *mut libc::c_char) -> *mut libc::c_char;
    fn pathwalk(
        path: *mut libc::c_char,
        proc_0: PathWalkProc,
        context: *mut libc::c_void,
    ) -> libc::c_int;
    fn file_lookup(path: *mut libc::c_char, context: *mut libc::c_void) -> libc::c_int;
    fn parse_font_spec(
        spec: *mut libc::c_char,
        name_return: *mut *mut libc::c_char,
        size_return: *mut FontPoint,
        encoding_return: *mut InputEncoding,
    ) -> libc::c_int;
    fn escape_string(string: *mut libc::c_char) -> *mut libc::c_char;
    fn read_font_info();
    fn format_user_string(
        context_name: *mut libc::c_char,
        string: *mut libc::c_char,
    ) -> *mut libc::c_char;
    fn buffer_ptr(buffer: *mut Buffer) -> *mut libc::c_char;
    fn buffer_free(buffer: *mut Buffer);
    fn buffer_alloc() -> *mut Buffer;
    fn is_ungetc(ch: libc::c_int, is: *mut InputStream) -> libc::c_int;
    fn is_getc(is: *mut InputStream) -> libc::c_int;
    fn download_font(name: *mut libc::c_char);
    fn count_key_value_set(set: StringHashPtr) -> libc::c_int;
    fn paste_file(name: *mut libc::c_char, suffix: *mut libc::c_char) -> libc::c_int;
    static mut continuous_page_numbers: libc::c_int;
    static mut swap_even_page_margins: libc::c_int;
    static mut rotate_even_pages: libc::c_int;
    static mut pslevel: libc::c_uint;
    static mut horizontal_column_height: libc::c_double;
    static mut generate_PageSize: libc::c_int;
    static mut output_language_pass_through: libc::c_int;
    static mut output_language: *mut libc::c_char;
    static mut nup_columnwise: libc::c_int;
    static mut nup_scale: libc::c_double;
    static mut nup_height: libc::c_uint;
    static mut nup_width: libc::c_uint;
    static mut nup_landscape: libc::c_int;
    static mut nup_columns: libc::c_uint;
    static mut nup_rows: libc::c_uint;
    static mut nup: libc::c_uint;
    static mut slice: libc::c_uint;
    static mut slicing: libc::c_int;
    static mut file_align: libc::c_uint;
    static mut toc_fmt_string: *mut libc::c_char;
    static mut toc_fp: *mut FILE;
    static mut toc: libc::c_int;
    static mut formfeed_type: FormFeedType;
    static mut bggray: libc::c_double;
    static mut line_highlight_gray: libc::c_double;
    static mut borders: libc::c_int;
    static mut page_ranges: *mut PageRange;
    static mut page_prefeed: libc::c_int;
    static mut highlight_bar_gray: libc::c_double;
    static mut highlight_bars: libc::c_uint;
    static mut clean_7bit: libc::c_int;
    static mut mark_wrapped_lines_style: MarkWrappedLinesStyle;
    static mut non_printable_format: NonPrintableFormat;
    static mut interpret_formfeed: libc::c_int;
    static mut start_line_number: libc::c_uint;
    static mut line_numbers: libc::c_int;
    static mut pass_through: libc::c_int;
    static mut page_label: PageLabelFormat;
    static mut ul_angle_p: libc::c_int;
    static mut ul_position_p: libc::c_int;
    static mut ul_style: libc::c_uint;
    static mut ul_angle: libc::c_double;
    static mut ul_y: libc::c_double;
    static mut ul_x: libc::c_double;
    static mut underlay: *mut libc::c_char;
    static mut ul_font: *mut libc::c_char;
    static mut ul_gray: libc::c_double;
    static mut ul_ptsize: FontPoint;
    static mut baselineskip: libc::c_double;
    static mut tabsize: libc::c_int;
    static mut default_escape_char: libc::c_int;
    static mut escape_char: libc::c_int;
    static mut special_escapes: libc::c_int;
    static mut encoding_name: *mut libc::c_char;
    static mut encoding: InputEncoding;
    static mut lines_per_page: libc::c_uint;
    static mut page_footer: *mut libc::c_char;
    static mut page_header: *mut libc::c_char;
    static mut line_indent: libc::c_double;
    static mut fancy_header_name: *mut libc::c_char;
    static mut header: HeaderType;
    static mut landscape: libc::c_int;
    static mut quiet: libc::c_int;
    static mut line_end: LineEndType;
    static mut num_columns: libc::c_int;
    static mut title: *mut libc::c_char;
    static mut num_copies: libc::c_int;
    static mut verbose: libc::c_int;
    static mut font_bbox_lly: libc::c_double;
    static mut font_is_fixed: libc::c_int;
    static mut font_ctype: [libc::c_char; 0];
    static mut font_widths: [libc::c_double; 0];
    static mut default_Fencoding: InputEncoding;
    static mut default_Fname: *mut libc::c_char;
    static mut default_Fpt: FontPoint;
    static mut Fpt: FontPoint;
    static mut Fname: *mut libc::c_char;
    static mut HFpt: FontPoint;
    static mut HFname: *mut libc::c_char;
    static mut user_strings: StringHashPtr;
    static mut statusdict: StringHashPtr;
    static mut pagedevice: StringHashPtr;
    static mut download_fonts: StringHashPtr;
    static mut res_fonts: StringHashPtr;
    static mut nup_ypad: libc::c_uint;
    static mut nup_xpad: libc::c_uint;
    static mut d_output_y_margin: libc::c_int;
    static mut d_output_x_margin: libc::c_int;
    static mut d_output_h: libc::c_int;
    static mut d_output_w: libc::c_int;
    static mut d_footer_h: libc::c_int;
    static mut d_header_h: libc::c_int;
    static mut d_header_w: libc::c_int;
    static mut d_page_h: libc::c_int;
    static mut d_page_w: libc::c_int;
    static mut non_printable_chars: [libc::c_int; 0];
    static mut num_non_printable_chars: libc::c_int;
    static mut missing_chars: [libc::c_int; 0];
    static mut num_missing_chars: libc::c_int;
    static mut num_truncated_lines: libc::c_int;
    static mut total_pages: libc::c_int;
    static mut bs: libc::c_int;
    static mut nl: libc::c_int;
    static mut output_first_line: *mut libc::c_char;
    static mut media: *mut MediaEntry;
    static mut libpath: *mut libc::c_char;
    static mut passwd: *mut passwd;
    static mut date_string: *mut libc::c_char;
    static mut ofp: *mut FILE;
    static mut program: *mut libc::c_char;
    fn xstrdup(_: *mut libc::c_char) -> *mut libc::c_char;
    fn xfree(ptr: *mut libc::c_void);
    fn xrealloc(ptr: *mut libc::c_void, size: size_t) -> *mut libc::c_void;
    fn strhash_get_next(
        hash: StringHashPtr,
        key_return: *mut *mut libc::c_char,
        keylen_return: *mut libc::c_int,
        data_return: *mut *mut libc::c_void,
    ) -> libc::c_int;
    fn strhash_get_first(
        hash: StringHashPtr,
        key_return: *mut *mut libc::c_char,
        keylen_return: *mut libc::c_int,
        data_return: *mut *mut libc::c_void,
    ) -> libc::c_int;
    fn strhash_put(
        hash: StringHashPtr,
        key: *mut libc::c_char,
        keylen: libc::c_int,
        data: *mut libc::c_void,
        old_data_return: *mut *mut libc::c_void,
    ) -> libc::c_int;
    fn __errno_location() -> *mut libc::c_int;
    fn __assert_fail(
        __assertion: *const libc::c_char,
        __file: *const libc::c_char,
        __line: libc::c_uint,
        __function: *const libc::c_char,
    ) -> !;
    fn __ctype_b_loc() -> *mut *const libc::c_ushort;
    fn dcgettext(
        __domainname: *const libc::c_char,
        __msgid: *const libc::c_char,
        __category: libc::c_int,
    ) -> *mut libc::c_char;
    fn dirname(__path: *mut libc::c_char) -> *mut libc::c_char;
    fn __xpg_basename(__path: *mut libc::c_char) -> *mut libc::c_char;
}
pub type size_t = libc::c_ulong;
pub type __uid_t = libc::c_uint;
pub type __gid_t = libc::c_uint;
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
pub type HeaderType = libc::c_uint;
pub const HDR_FANCY: HeaderType = 2;
pub const HDR_SIMPLE: HeaderType = 1;
pub const HDR_NONE: HeaderType = 0;
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
pub type PageLabelFormat = libc::c_uint;
pub const LABEL_LONG: PageLabelFormat = 1;
pub const LABEL_SHORT: PageLabelFormat = 0;
pub type MarkWrappedLinesStyle = libc::c_uint;
pub const MWLS_ARROW: MarkWrappedLinesStyle = 3;
pub const MWLS_BOX: MarkWrappedLinesStyle = 2;
pub const MWLS_PLUS: MarkWrappedLinesStyle = 1;
pub const MWLS_NONE: MarkWrappedLinesStyle = 0;
pub type NonPrintableFormat = libc::c_uint;
pub const NPF_OCTAL: NonPrintableFormat = 3;
pub const NPF_CARET: NonPrintableFormat = 2;
pub const NPF_QUESTIONMARK: NonPrintableFormat = 1;
pub const NPF_SPACE: NonPrintableFormat = 0;
pub type FormFeedType = libc::c_uint;
pub const FORMFEED_HCOLUMN: FormFeedType = 2;
pub const FORMFEED_PAGE: FormFeedType = 1;
pub const FORMFEED_COLUMN: FormFeedType = 0;
pub type LineEndType = libc::c_uint;
pub const LE_WORD_WRAP: LineEndType = 2;
pub const LE_CHAR_WRAP: LineEndType = 1;
pub const LE_TRUNCATE: LineEndType = 0;
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
pub struct page_range_st {
    pub next: *mut page_range_st,
    pub odd: libc::c_int,
    pub even: libc::c_int,
    pub start: libc::c_uint,
    pub end: libc::c_uint,
}
pub type PageRange = page_range_st;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct font_point_st {
    pub w: libc::c_double,
    pub h: libc::c_double,
}
pub type FontPoint = font_point_st;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct color_st {
    pub r: libc::c_float,
    pub g: libc::c_float,
    pub b: libc::c_float,
}
pub type Color = color_st;
pub type TokenType = libc::c_uint;
pub const tPS: TokenType = 16;
pub const tLOADX: TokenType = 15;
pub const tSAVEX: TokenType = 14;
pub const tBGCOLOR: TokenType = 13;
pub const tCOLOR: TokenType = 12;
pub const tFONT: TokenType = 11;
pub const tNEWPAGE: TokenType = 10;
pub const tSETPAGENUMBER: TokenType = 9;
pub const tSETFILENAME: TokenType = 8;
pub const tEPSF: TokenType = 7;
pub const tWRAPPED_NEWLINE: TokenType = 6;
pub const tCARRIAGE_RETURN: TokenType = 5;
pub const tNEWLINE: TokenType = 4;
pub const tFORMFEED: TokenType = 3;
pub const tSTRING: TokenType = 2;
pub const tEOF: TokenType = 1;
pub const tNONE: TokenType = 0;
pub type Token = gs_token_st;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct gs_token_st {
    pub type_0: TokenType,
    pub flags: libc::c_uint,
    pub new_x: libc::c_double,
    pub new_y: libc::c_double,
    pub new_col: libc::c_int,
    pub u: C2RustUnnamed_0,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_0 {
    pub i: libc::c_int,
    pub str_0: *mut libc::c_char,
    pub epsf: C2RustUnnamed_2,
    pub color: Color,
    pub bgcolor: Color,
    pub font: C2RustUnnamed_1,
    pub filename: [libc::c_char; 4096],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_1 {
    pub name: [libc::c_char; 4096],
    pub size: FontPoint,
    pub encoding: InputEncoding,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_2 {
    pub x: libc::c_double,
    pub y: libc::c_double,
    pub w: libc::c_double,
    pub h: libc::c_double,
    pub xscale: libc::c_double,
    pub yscale: libc::c_double,
    pub llx: libc::c_int,
    pub lly: libc::c_int,
    pub urx: libc::c_int,
    pub ury: libc::c_int,
    pub filename: [libc::c_char; 4096],
    pub skipbuf: *mut libc::c_char,
    pub skipbuf_len: libc::c_uint,
    pub skipbuf_pos: libc::c_uint,
    pub fp: *mut FILE,
    pub pipe: libc::c_int,
}
pub const ESC_PS: SpecialEscape = 13;
pub const ESC_LOADX: SpecialEscape = 12;
pub const ESC_SAVEX: SpecialEscape = 11;
pub const ESC_NEWPAGE: SpecialEscape = 5;
pub const ESC_SETPAGENUMBER: SpecialEscape = 7;
pub const ESC_SETFILENAME: SpecialEscape = 6;
pub const ESC_ESCAPE: SpecialEscape = 10;
pub const ESC_BGGRAY: SpecialEscape = 9;
pub const ESC_SHADE: SpecialEscape = 8;
pub const ESC_COLOR: SpecialEscape = 3;
pub type SpecialEscape = libc::c_uint;
pub const ESC_BGCOLOR: SpecialEscape = 4;
pub const ESC_FONT: SpecialEscape = 2;
pub const ESC_EPSF: SpecialEscape = 1;
pub const ESC_COMMENT: SpecialEscape = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_3 {
    pub name: *mut libc::c_char,
    pub escape: SpecialEscape,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_4 {
    pub magic: *mut libc::c_char,
    pub magiclen: libc::c_uint,
    pub name: *mut libc::c_char,
    pub revert_delta: libc::c_int,
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
pub static mut current_pagenum: libc::c_uint = 0 as libc::c_int as libc::c_uint;
pub static mut total_pages_in_file: libc::c_uint = 0;
pub static mut input_filenum: libc::c_uint = 0 as libc::c_int as libc::c_uint;
pub static mut current_file_linenum: libc::c_uint = 0;
pub static mut first_pagenum_for_file: libc::c_int = 0;
pub static mut fname: *mut libc::c_char = 0 as *const libc::c_char as *mut libc::c_char;
static mut ps_header_dumped: libc::c_int = 0 as libc::c_int;
static mut divertfp: *mut FILE = 0 as *const FILE as *mut FILE;
static mut cofp: *mut FILE = 0 as *const FILE as *mut FILE;
static mut do_print: libc::c_int = 1 as libc::c_int;
static mut user_fontp: libc::c_int = 0 as libc::c_int;
static mut user_font_name: [libc::c_char; 4096] = [0; 4096];
static mut user_font_pt: FontPoint = FontPoint { w: 0., h: 0. };
static mut user_font_encoding: InputEncoding = ENC_ISO_8859_1;
static mut user_colorp: libc::c_int = 0 as libc::c_int;
static mut user_color: Color = Color { r: 0., g: 0., b: 0. };
static mut user_bgcolorp: libc::c_int = 0 as libc::c_int;
static mut user_bgcolor: Color = Color { r: 0., g: 0., b: 0. };
static mut print_line_number_last: libc::c_uint = 0;
static mut xstore: [libc::c_double; 256] = [0.; 256];
pub unsafe extern "C" fn dump_ps_header() {
    let mut cp: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut cp2: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut i: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    let mut got: libc::c_int = 0;
    let mut ps_version_string: *mut libc::c_char = 0 as *mut libc::c_char;
    if ps_header_dumped != 0 {
        return;
    }
    ps_header_dumped = 1 as libc::c_int;
    ps_version_string = xstrdup(
        b"1.6.6\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    );
    cp = strrchr(ps_version_string, '.' as i32);
    *cp = ' ' as i32 as libc::c_char;
    if cofp.is_null() {
        cofp = ofp;
    }
    if do_print != 0 {
        fprintf(cofp, b"%s\n\0" as *const u8 as *const libc::c_char, output_first_line);
    }
    if cofp.is_null() {
        cofp = ofp;
    }
    if do_print != 0 {
        fprintf(
            cofp,
            b"%%%%BoundingBox: %d %d %d %d\n\0" as *const u8 as *const libc::c_char,
            (*media).llx,
            (*media).lly,
            (*media).urx,
            (*media).ury,
        );
    }
    if cofp.is_null() {
        cofp = ofp;
    }
    if do_print != 0 {
        fprintf(cofp, b"%%%%Title: %s\n\0" as *const u8 as *const libc::c_char, title);
    }
    if cofp.is_null() {
        cofp = ofp;
    }
    if do_print != 0 {
        fprintf(
            cofp,
            b"%%%%For: %s\n\0" as *const u8 as *const libc::c_char,
            (*passwd).pw_gecos,
        );
    }
    if cofp.is_null() {
        cofp = ofp;
    }
    if do_print != 0 {
        fprintf(
            cofp,
            b"%%%%Creator: %s\n\0" as *const u8 as *const libc::c_char,
            b"GNU Enscript 1.6.6\0" as *const u8 as *const libc::c_char,
        );
    }
    if cofp.is_null() {
        cofp = ofp;
    }
    if do_print != 0 {
        fprintf(
            cofp,
            b"%%%%CreationDate: %s\n\0" as *const u8 as *const libc::c_char,
            date_string,
        );
    }
    if cofp.is_null() {
        cofp = ofp;
    }
    if do_print != 0 {
        fprintf(
            cofp,
            b"%%%%Orientation: %s\n\0" as *const u8 as *const libc::c_char,
            if nup > 1 as libc::c_int as libc::c_uint && nup_landscape != 0
                || nup == 1 as libc::c_int as libc::c_uint && landscape != 0
            {
                b"Landscape\0" as *const u8 as *const libc::c_char
            } else {
                b"Portrait\0" as *const u8 as *const libc::c_char
            },
        );
    }
    if cofp.is_null() {
        cofp = ofp;
    }
    if do_print != 0 {
        fprintf(cofp, b"%%%%Pages: (atend)\n\0" as *const u8 as *const libc::c_char);
    }
    if cofp.is_null() {
        cofp = ofp;
    }
    if do_print != 0 {
        fprintf(
            cofp,
            b"%%%%DocumentMedia: %s %d %d 0 () ()\n\0" as *const u8
                as *const libc::c_char,
            (*media).name,
            (*media).w,
            (*media).h,
        );
    }
    if cofp.is_null() {
        cofp = ofp;
    }
    if do_print != 0 {
        fprintf(
            cofp,
            b"%%%%DocumentNeededResources: (atend)\n\0" as *const u8
                as *const libc::c_char,
        );
    }
    if count_key_value_set(pagedevice) > 0 as libc::c_int {
        if cofp.is_null() {
            cofp = ofp;
        }
        if do_print != 0 {
            fprintf(
                cofp,
                b"%%%%LanguageLevel: 2\n\0" as *const u8 as *const libc::c_char,
            );
        }
    }
    if cofp.is_null() {
        cofp = ofp;
    }
    if do_print != 0 {
        fprintf(cofp, b"%%%%EndComments\n\0" as *const u8 as *const libc::c_char);
    }
    if cofp.is_null() {
        cofp = ofp;
    }
    if do_print != 0 {
        fprintf(cofp, b"%%%%BeginProlog\n\0" as *const u8 as *const libc::c_char);
    }
    if cofp.is_null() {
        cofp = ofp;
    }
    if do_print != 0 {
        fprintf(
            cofp,
            b"%%%%BeginResource: procset Enscript-Prolog %s\n\0" as *const u8
                as *const libc::c_char,
            ps_version_string,
        );
    }
    if paste_file(
        b"enscript\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        b".pro\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    ) == 0
    {
        fprintf(stderr, b"%s: \0" as *const u8 as *const libc::c_char, program);
        fprintf(
            stderr,
            dcgettext(
                0 as *const libc::c_char,
                b"couldn't find prolog \"%s\": %s\n\0" as *const u8
                    as *const libc::c_char,
                5 as libc::c_int,
            ),
            b"enscript.pro\0" as *const u8 as *const libc::c_char,
            strerror(*__errno_location()),
        );
        fprintf(stderr, b"\n\0" as *const u8 as *const libc::c_char);
        fflush(stderr);
        exit(1 as libc::c_int);
    }
    if cofp.is_null() {
        cofp = ofp;
    }
    if do_print != 0 {
        fprintf(cofp, b"%%%%EndResource\n\0" as *const u8 as *const libc::c_char);
    }
    if cofp.is_null() {
        cofp = ofp;
    }
    if do_print != 0 {
        fprintf(
            cofp,
            b"%%%%BeginResource: procset Enscript-Encoding-%s %s\n\0" as *const u8
                as *const libc::c_char,
            encoding_name,
            ps_version_string,
        );
    }
    if paste_file(
        encoding_name,
        b".enc\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    ) == 0
    {
        fprintf(stderr, b"%s: \0" as *const u8 as *const libc::c_char, program);
        fprintf(
            stderr,
            dcgettext(
                0 as *const libc::c_char,
                b"couldn't find encoding file \"%s.enc\": %s\n\0" as *const u8
                    as *const libc::c_char,
                5 as libc::c_int,
            ),
            encoding_name,
            strerror(*__errno_location()),
        );
        fprintf(stderr, b"\n\0" as *const u8 as *const libc::c_char);
        fflush(stderr);
        exit(1 as libc::c_int);
    }
    if cofp.is_null() {
        cofp = ofp;
    }
    if do_print != 0 {
        fprintf(cofp, b"%%%%EndResource\n\0" as *const u8 as *const libc::c_char);
    }
    if cofp.is_null() {
        cofp = ofp;
    }
    if do_print != 0 {
        fprintf(cofp, b"%%%%EndProlog\n\0" as *const u8 as *const libc::c_char);
    }
    if cofp.is_null() {
        cofp = ofp;
    }
    if do_print != 0 {
        fprintf(cofp, b"%%%%BeginSetup\n\0" as *const u8 as *const libc::c_char);
    }
    got = strhash_get_first(
        download_fonts,
        &mut cp,
        &mut j,
        &mut cp2 as *mut *mut libc::c_char as *mut *mut libc::c_void,
    );
    while got != 0 {
        download_font(cp);
        got = strhash_get_next(
            download_fonts,
            &mut cp,
            &mut j,
            &mut cp2 as *mut *mut libc::c_char as *mut *mut libc::c_void,
        );
    }
    got = strhash_get_first(
        res_fonts,
        &mut cp,
        &mut j,
        &mut cp2 as *mut *mut libc::c_char as *mut *mut libc::c_void,
    );
    while got != 0 {
        if cofp.is_null() {
            cofp = ofp;
        }
        if do_print != 0 {
            fprintf(
                cofp,
                b"%%%%IncludeResource: font %s\n\0" as *const u8 as *const libc::c_char,
                cp,
            );
        }
        got = strhash_get_next(
            res_fonts,
            &mut cp,
            &mut j,
            &mut cp2 as *mut *mut libc::c_char as *mut *mut libc::c_void,
        );
    }
    if cofp.is_null() {
        cofp = ofp;
    }
    if do_print != 0 {
        fprintf(cofp, b"/HFpt_w %g def\n\0" as *const u8 as *const libc::c_char, HFpt.w);
    }
    if cofp.is_null() {
        cofp = ofp;
    }
    if do_print != 0 {
        fprintf(cofp, b"/HFpt_h %g def\n\0" as *const u8 as *const libc::c_char, HFpt.h);
    }
    if cofp.is_null() {
        cofp = ofp;
    }
    if do_print != 0 {
        fprintf(
            cofp,
            b"/%s /HF-gs-font MF\n\0" as *const u8 as *const libc::c_char,
            HFname,
        );
    }
    if cofp.is_null() {
        cofp = ofp;
    }
    if do_print != 0 {
        fprintf(
            cofp,
            b"/HF /HF-gs-font findfont [HFpt_w 0 0 HFpt_h 0 0] makefont def\n\0"
                as *const u8 as *const libc::c_char,
        );
    }
    if cofp.is_null() {
        cofp = ofp;
    }
    if do_print != 0 {
        fprintf(
            cofp,
            b"/%s /F-gs-font MF\n\0" as *const u8 as *const libc::c_char,
            Fname,
        );
    }
    if cofp.is_null() {
        cofp = ofp;
    }
    if do_print != 0 {
        fprintf(
            cofp,
            b"/F-gs-font %g %g SF\n\0" as *const u8 as *const libc::c_char,
            Fpt.w,
            Fpt.h,
        );
    }
    if !underlay.is_null() {
        if cofp.is_null() {
            cofp = ofp;
        }
        if do_print != 0 {
            fprintf(
                cofp,
                b"/ul_str (%s) def\n\0" as *const u8 as *const libc::c_char,
                underlay,
            );
        }
        if cofp.is_null() {
            cofp = ofp;
        }
        if do_print != 0 {
            fprintf(
                cofp,
                b"/ul_w_ptsize %g def\n\0" as *const u8 as *const libc::c_char,
                ul_ptsize.w,
            );
        }
        if cofp.is_null() {
            cofp = ofp;
        }
        if do_print != 0 {
            fprintf(
                cofp,
                b"/ul_h_ptsize %g def\n\0" as *const u8 as *const libc::c_char,
                ul_ptsize.h,
            );
        }
        if cofp.is_null() {
            cofp = ofp;
        }
        if do_print != 0 {
            fprintf(
                cofp,
                b"/ul_gray %g def\n\0" as *const u8 as *const libc::c_char,
                ul_gray,
            );
        }
        if cofp.is_null() {
            cofp = ofp;
        }
        if do_print != 0 {
            fprintf(cofp, b"/ul_x %g def\n\0" as *const u8 as *const libc::c_char, ul_x);
        }
        if cofp.is_null() {
            cofp = ofp;
        }
        if do_print != 0 {
            fprintf(cofp, b"/ul_y %g def\n\0" as *const u8 as *const libc::c_char, ul_y);
        }
        if cofp.is_null() {
            cofp = ofp;
        }
        if do_print != 0 {
            fprintf(
                cofp,
                b"/ul_angle %g def\n\0" as *const u8 as *const libc::c_char,
                ul_angle,
            );
        }
        if cofp.is_null() {
            cofp = ofp;
        }
        if do_print != 0 {
            fprintf(
                cofp,
                b"/ul_style %d def\n\0" as *const u8 as *const libc::c_char,
                ul_style,
            );
        }
        if cofp.is_null() {
            cofp = ofp;
        }
        if do_print != 0 {
            fprintf(
                cofp,
                b"/%s /F-ul-font MF\n\0" as *const u8 as *const libc::c_char,
                ul_font,
            );
        }
        if cofp.is_null() {
            cofp = ofp;
        }
        if do_print != 0 {
            fprintf(
                cofp,
                b"/ul_font /F-ul-font findfont [ul_w_ptsize 0 0 ul_h_ptsize 0 0] makefont def\n\0"
                    as *const u8 as *const libc::c_char,
            );
        }
    }
    if cofp.is_null() {
        cofp = ofp;
    }
    if do_print != 0 {
        fprintf(
            cofp,
            b"/#copies %d def\n\0" as *const u8 as *const libc::c_char,
            num_copies,
        );
    }
    if page_prefeed != 0 {
        if cofp.is_null() {
            cofp = ofp;
        }
        if do_print != 0 {
            fprintf(cofp, b"true page_prefeed\n\0" as *const u8 as *const libc::c_char);
        }
    }
    if count_key_value_set(statusdict) > 0 as libc::c_int {
        if cofp.is_null() {
            cofp = ofp;
        }
        if do_print != 0 {
            fprintf(
                cofp,
                b"%% Statustdict definitions:\nstatusdict begin\n  \0" as *const u8
                    as *const libc::c_char,
            );
        }
        i = 2 as libc::c_int;
        got = strhash_get_first(
            statusdict,
            &mut cp,
            &mut j,
            &mut cp2 as *mut *mut libc::c_char as *mut *mut libc::c_void,
        );
        while got != 0 {
            j = (strlen(cp))
                .wrapping_add(1 as libc::c_int as libc::c_ulong)
                .wrapping_add(strlen(cp2))
                .wrapping_add(1 as libc::c_int as libc::c_ulong) as libc::c_int;
            if i + j > 75 as libc::c_int {
                if cofp.is_null() {
                    cofp = ofp;
                }
                if do_print != 0 {
                    fprintf(cofp, b"\n  \0" as *const u8 as *const libc::c_char);
                }
                i = 2 as libc::c_int;
            }
            if cofp.is_null() {
                cofp = ofp;
            }
            if do_print != 0 {
                fprintf(cofp, b"%s %s \0" as *const u8 as *const libc::c_char, cp2, cp);
            }
            i += j;
            got = strhash_get_next(
                statusdict,
                &mut cp,
                &mut j,
                &mut cp2 as *mut *mut libc::c_char as *mut *mut libc::c_void,
            );
        }
        if cofp.is_null() {
            cofp = ofp;
        }
        if do_print != 0 {
            fprintf(cofp, b"\nend\n\0" as *const u8 as *const libc::c_char);
        }
    }
    if pslevel >= 2 as libc::c_int as libc::c_uint
        && (count_key_value_set(pagedevice) > 0 as libc::c_int || generate_PageSize != 0)
    {
        if cofp.is_null() {
            cofp = ofp;
        }
        if do_print != 0 {
            fprintf(
                cofp,
                b"%% Pagedevice definitions:\n\0" as *const u8 as *const libc::c_char,
            );
        }
        if cofp.is_null() {
            cofp = ofp;
        }
        if do_print != 0 {
            fprintf(
                cofp,
                b"gs_languagelevel 1 gt {\n  <<\n    \0" as *const u8
                    as *const libc::c_char,
            );
        }
        i = 4 as libc::c_int;
        got = strhash_get_first(
            pagedevice,
            &mut cp,
            &mut j,
            &mut cp2 as *mut *mut libc::c_char as *mut *mut libc::c_void,
        );
        while got != 0 {
            j = (strlen(cp2))
                .wrapping_add(1 as libc::c_int as libc::c_ulong)
                .wrapping_add(strlen(cp))
                .wrapping_add(2 as libc::c_int as libc::c_ulong) as libc::c_int;
            if i + j > 75 as libc::c_int {
                if cofp.is_null() {
                    cofp = ofp;
                }
                if do_print != 0 {
                    fprintf(cofp, b"\n    \0" as *const u8 as *const libc::c_char);
                }
                i = 4 as libc::c_int;
            }
            if cofp.is_null() {
                cofp = ofp;
            }
            if do_print != 0 {
                fprintf(cofp, b"/%s %s \0" as *const u8 as *const libc::c_char, cp, cp2);
            }
            i += j;
            got = strhash_get_next(
                pagedevice,
                &mut cp,
                &mut j,
                &mut cp2 as *mut *mut libc::c_char as *mut *mut libc::c_void,
            );
        }
        if generate_PageSize != 0 {
            if i + 21 as libc::c_int > 75 as libc::c_int {
                if cofp.is_null() {
                    cofp = ofp;
                }
                if do_print != 0 {
                    fprintf(cofp, b"\n    \0" as *const u8 as *const libc::c_char);
                }
                i = 4 as libc::c_int;
            }
            if cofp.is_null() {
                cofp = ofp;
            }
            if do_print != 0 {
                fprintf(
                    cofp,
                    b"/PageSize [%d %d] \0" as *const u8 as *const libc::c_char,
                    (*media).w,
                    (*media).h,
                );
            }
            i += 21 as libc::c_int;
        }
        if cofp.is_null() {
            cofp = ofp;
        }
        if do_print != 0 {
            fprintf(
                cofp,
                b"\n  >> setpagedevice\n} if\n\0" as *const u8 as *const libc::c_char,
            );
        }
    }
    if header as libc::c_uint != HDR_NONE as libc::c_int as libc::c_uint {
        let mut hdr: *mut libc::c_char = 0 as *mut libc::c_char;
        if header as libc::c_uint == HDR_SIMPLE as libc::c_int as libc::c_uint {
            hdr = b"simple\0" as *const u8 as *const libc::c_char as *mut libc::c_char;
        } else {
            hdr = fancy_header_name;
        }
        if cofp.is_null() {
            cofp = ofp;
        }
        if do_print != 0 {
            fprintf(
                cofp,
                b"%%%%BeginResource: procset Enscript-Header-%s %s\n\0" as *const u8
                    as *const libc::c_char,
                hdr,
                ps_version_string,
            );
        }
        if paste_file(
            hdr,
            b".hdr\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        ) == 0
        {
            fprintf(stderr, b"%s: \0" as *const u8 as *const libc::c_char, program);
            fprintf(
                stderr,
                dcgettext(
                    0 as *const libc::c_char,
                    b"couldn't find header definition file \"%s.hdr\": %s\n\0"
                        as *const u8 as *const libc::c_char,
                    5 as libc::c_int,
                ),
                hdr,
                strerror(*__errno_location()),
            );
            fprintf(stderr, b"\n\0" as *const u8 as *const libc::c_char);
            fflush(stderr);
            exit(1 as libc::c_int);
        }
        if cofp.is_null() {
            cofp = ofp;
        }
        if do_print != 0 {
            fprintf(cofp, b"%%%%EndResource\n\0" as *const u8 as *const libc::c_char);
        }
    }
    d_output_w = d_page_w;
    d_output_h = d_page_h - d_header_h - d_footer_h;
    if cofp.is_null() {
        cofp = ofp;
    }
    if do_print != 0 {
        fprintf(
            cofp,
            b"/d_page_w %d def\n\0" as *const u8 as *const libc::c_char,
            d_page_w,
        );
    }
    if cofp.is_null() {
        cofp = ofp;
    }
    if do_print != 0 {
        fprintf(
            cofp,
            b"/d_page_h %d def\n\0" as *const u8 as *const libc::c_char,
            d_page_h,
        );
    }
    if cofp.is_null() {
        cofp = ofp;
    }
    if do_print != 0 {
        fprintf(
            cofp,
            b"/d_header_x %d def\n\0" as *const u8 as *const libc::c_char,
            0 as libc::c_int,
        );
    }
    if cofp.is_null() {
        cofp = ofp;
    }
    if do_print != 0 {
        fprintf(
            cofp,
            b"/d_header_y %d def\n\0" as *const u8 as *const libc::c_char,
            d_output_h + d_footer_h,
        );
    }
    if cofp.is_null() {
        cofp = ofp;
    }
    if do_print != 0 {
        fprintf(
            cofp,
            b"/d_header_w %d def\n\0" as *const u8 as *const libc::c_char,
            d_header_w,
        );
    }
    if cofp.is_null() {
        cofp = ofp;
    }
    if do_print != 0 {
        fprintf(
            cofp,
            b"/d_header_h %d def\n\0" as *const u8 as *const libc::c_char,
            d_header_h,
        );
    }
    if cofp.is_null() {
        cofp = ofp;
    }
    if do_print != 0 {
        fprintf(
            cofp,
            b"/d_footer_x %d def\n\0" as *const u8 as *const libc::c_char,
            0 as libc::c_int,
        );
    }
    if cofp.is_null() {
        cofp = ofp;
    }
    if do_print != 0 {
        fprintf(
            cofp,
            b"/d_footer_y %d def\n\0" as *const u8 as *const libc::c_char,
            0 as libc::c_int,
        );
    }
    if cofp.is_null() {
        cofp = ofp;
    }
    if do_print != 0 {
        fprintf(
            cofp,
            b"/d_footer_w %d def\n\0" as *const u8 as *const libc::c_char,
            d_header_w,
        );
    }
    if cofp.is_null() {
        cofp = ofp;
    }
    if do_print != 0 {
        fprintf(
            cofp,
            b"/d_footer_h %d def\n\0" as *const u8 as *const libc::c_char,
            d_footer_h,
        );
    }
    if cofp.is_null() {
        cofp = ofp;
    }
    if do_print != 0 {
        fprintf(
            cofp,
            b"/d_output_w %d def\n\0" as *const u8 as *const libc::c_char,
            d_output_w,
        );
    }
    if cofp.is_null() {
        cofp = ofp;
    }
    if do_print != 0 {
        fprintf(
            cofp,
            b"/d_output_h %d def\n\0" as *const u8 as *const libc::c_char,
            d_output_h,
        );
    }
    if cofp.is_null() {
        cofp = ofp;
    }
    if do_print != 0 {
        fprintf(
            cofp,
            b"/cols %d def\n\0" as *const u8 as *const libc::c_char,
            num_columns,
        );
    }
    if cofp.is_null() {
        cofp = ofp;
    }
    if do_print != 0 {
        fprintf(cofp, b"%%%%EndSetup\n\0" as *const u8 as *const libc::c_char);
    }
}
pub unsafe extern "C" fn dump_ps_trailer() {
    let mut i: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    let mut got: libc::c_int = 0;
    let mut cp: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut value: *mut libc::c_void = 0 as *mut libc::c_void;
    let mut nup_subpage: libc::c_uint = 0;
    if ps_header_dumped == 0 {
        return;
    }
    nup_subpage = ((total_pages - 1 as libc::c_int) as libc::c_uint).wrapping_rem(nup);
    if nup > 1 as libc::c_int as libc::c_uint
        && nup_subpage.wrapping_add(1 as libc::c_int as libc::c_uint) != nup
    {
        if cofp.is_null() {
            cofp = ofp;
        }
        if do_print != 0 {
            fprintf(cofp, b"_R\nS\n\0" as *const u8 as *const libc::c_char);
        }
    }
    if cofp.is_null() {
        cofp = ofp;
    }
    if do_print != 0 {
        fprintf(cofp, b"%%%%Trailer\n\0" as *const u8 as *const libc::c_char);
    }
    if page_prefeed != 0 {
        if cofp.is_null() {
            cofp = ofp;
        }
        if do_print != 0 {
            fprintf(cofp, b"false page_prefeed\n\0" as *const u8 as *const libc::c_char);
        }
    }
    if cofp.is_null() {
        cofp = ofp;
    }
    if do_print != 0 {
        fprintf(
            cofp,
            b"%%%%Pages: %d\n\0" as *const u8 as *const libc::c_char,
            total_pages,
        );
    }
    if cofp.is_null() {
        cofp = ofp;
    }
    if do_print != 0 {
        fprintf(
            cofp,
            b"%%%%DocumentNeededResources: font \0" as *const u8 as *const libc::c_char,
        );
    }
    i = 32 as libc::c_int;
    got = strhash_get_first(res_fonts, &mut cp, &mut j, &mut value);
    while got != 0 {
        if (i as libc::c_ulong)
            .wrapping_add(strlen(cp))
            .wrapping_add(1 as libc::c_int as libc::c_ulong)
            > 75 as libc::c_int as libc::c_ulong
        {
            if cofp.is_null() {
                cofp = ofp;
            }
            if do_print != 0 {
                fprintf(cofp, b"\n%%%%+ font \0" as *const u8 as *const libc::c_char);
            }
            i = 9 as libc::c_int;
        }
        if cofp.is_null() {
            cofp = ofp;
        }
        if do_print != 0 {
            fprintf(cofp, b"%s \0" as *const u8 as *const libc::c_char, cp);
        }
        i = (i as libc::c_ulong)
            .wrapping_add((strlen(cp)).wrapping_add(1 as libc::c_int as libc::c_ulong))
            as libc::c_int as libc::c_int;
        got = strhash_get_next(res_fonts, &mut cp, &mut j, &mut value);
    }
    if cofp.is_null() {
        cofp = ofp;
    }
    if do_print != 0 {
        fprintf(cofp, b"\n%%%%EOF\n\0" as *const u8 as *const libc::c_char);
    }
}
pub unsafe extern "C" fn process_file(
    mut fname_arg: *mut libc::c_char,
    mut is: *mut InputStream,
    mut is_toc: libc::c_int,
) {
    let mut col: libc::c_int = 0;
    let mut x: libc::c_double = 0.;
    let mut y: libc::c_double = 0.;
    let mut lx: libc::c_double = 0.;
    let mut ly: libc::c_double = 0.;
    let mut linewidth: libc::c_double = 0.;
    let mut lineend: libc::c_double = 0.;
    let mut done: libc::c_int = 0 as libc::c_int;
    let mut page_clear: libc::c_int = 1 as libc::c_int;
    let mut line_column: libc::c_uint = 0;
    let mut current_linenum: libc::c_uint = 0;
    let mut linenumber_space: libc::c_double = 0 as libc::c_int as libc::c_double;
    let mut linenumber_margin: libc::c_double = 0 as libc::c_int as libc::c_double;
    let mut token: Token = Token {
        type_0: tNONE,
        flags: 0,
        new_x: 0.,
        new_y: 0.,
        new_col: 0,
        u: C2RustUnnamed_0 { i: 0 },
    };
    let mut reuse_last_token: libc::c_int = 0 as libc::c_int;
    let mut current_slice: libc::c_uint = 1 as libc::c_int as libc::c_uint;
    let mut last_wrapped_line: libc::c_int = -(1 as libc::c_int);
    let mut last_spaced_file_linenum: libc::c_int = -(1 as libc::c_int);
    let mut save_current_pagenum: libc::c_int = 0;
    let mut toc_pagenum: libc::c_int = 0 as libc::c_int;
    xfree(fname as *mut libc::c_void);
    fname = xstrdup(fname_arg);
    if continuous_page_numbers == 0 {
        current_pagenum = 0 as libc::c_int as libc::c_uint;
    }
    total_pages_in_file = 0 as libc::c_int as libc::c_uint;
    current_file_linenum = start_line_number;
    linenumber_space = *font_widths
        .as_mut_ptr()
        .offset('0' as i32 as libc::c_uchar as isize)
        * 5 as libc::c_int as libc::c_double + 1.0f64;
    linenumber_margin = *font_widths
        .as_mut_ptr()
        .offset(':' as i32 as libc::c_uchar as isize)
        + *font_widths.as_mut_ptr().offset('m' as i32 as libc::c_uchar as isize);
    input_filenum = input_filenum.wrapping_add(1);
    input_filenum;
    print_line_number_last = -(1 as libc::c_int) as libc::c_uint;
    if pass_through != 0 || output_language_pass_through != 0 {
        if do_pass_through(fname, is) != 0 {
            return;
        }
    }
    dump_ps_header();
    while (total_pages as libc::c_uint).wrapping_rem(file_align)
        != 0 as libc::c_int as libc::c_uint
    {
        total_pages += 1;
        total_pages;
        dump_empty_page();
    }
    if quiet == 0 && verbose >= 1 as libc::c_int {
        fprintf(
            stderr,
            dcgettext(
                0 as *const libc::c_char,
                b"processing file \"%s\"...\n\0" as *const u8 as *const libc::c_char,
                5 as libc::c_int,
            ),
            fname,
        );
    }
    linewidth = (d_output_w / num_columns - 2 as libc::c_int * d_output_x_margin)
        as libc::c_double - line_indent;
    first_pagenum_for_file = total_pages + 1 as libc::c_int;
    divert();
    while done == 0 {
        page_clear = 1 as libc::c_int;
        col = 0 as libc::c_int;
        's_135: while done == 0 && col < num_columns {
            x = ((col * d_output_w) as libc::c_float / num_columns as libc::c_float
                + d_output_x_margin as libc::c_float) as libc::c_double + line_indent;
            lx = x;
            lineend = lx + linewidth;
            y = (d_footer_h + d_output_h - d_output_y_margin) as libc::c_double
                - (Fpt.h + baselineskip);
            ly = y;
            current_linenum = 0 as libc::c_int as libc::c_uint;
            line_column = 0 as libc::c_int as libc::c_uint;
            loop {
                if line_numbers != 0 && line_column == 0 as libc::c_int as libc::c_uint
                    && current_file_linenum != last_spaced_file_linenum as libc::c_uint
                {
                    x += linenumber_space + linenumber_margin;
                    last_spaced_file_linenum = current_file_linenum as libc::c_int;
                }
                if reuse_last_token == 0 {
                    get_next_token(is, lx, x, line_column, lineend, &mut token);
                }
                reuse_last_token = 0 as libc::c_int;
                if token.type_0 as libc::c_uint == tEOF as libc::c_int as libc::c_uint {
                    done = 1 as libc::c_int;
                    break 's_135;
                } else {
                    if page_clear != 0 {
                        let mut pr: *mut PageRange = 0 as *mut PageRange;
                        current_pagenum = current_pagenum.wrapping_add(1);
                        current_pagenum;
                        total_pages_in_file = total_pages_in_file.wrapping_add(1);
                        total_pages_in_file;
                        if page_ranges.is_null() {
                            do_print = 1 as libc::c_int;
                        } else {
                            do_print = 0 as libc::c_int;
                            pr = page_ranges;
                            while !pr.is_null() {
                                if (*pr).odd != 0 || (*pr).even != 0 {
                                    if (*pr).odd != 0
                                        && current_pagenum
                                            .wrapping_rem(2 as libc::c_int as libc::c_uint)
                                            == 1 as libc::c_int as libc::c_uint
                                        || (*pr).even != 0
                                            && current_pagenum
                                                .wrapping_rem(2 as libc::c_int as libc::c_uint)
                                                == 0 as libc::c_int as libc::c_uint
                                    {
                                        do_print = 1 as libc::c_int;
                                        break;
                                    }
                                } else if (*pr).start <= current_pagenum
                                    && current_pagenum <= (*pr).end
                                {
                                    do_print = 1 as libc::c_int;
                                    break;
                                }
                                pr = (*pr).next;
                            }
                        }
                        if do_print != 0 {
                            total_pages += 1;
                            total_pages;
                        }
                        if is_toc != 0 {
                            save_current_pagenum = current_pagenum as libc::c_int;
                            toc_pagenum -= 1;
                            toc_pagenum;
                            current_pagenum = toc_pagenum as libc::c_uint;
                        }
                        dump_ps_page_header(fname, 0 as libc::c_int);
                        page_clear = 0 as libc::c_int;
                        if is_toc != 0 {
                            current_pagenum = save_current_pagenum as libc::c_uint;
                        }
                    }
                    if line_column == 0 as libc::c_int as libc::c_uint
                        && line_highlight_gray < 1.0f64
                    {
                        if cofp.is_null() {
                            cofp = ofp;
                        }
                        if do_print != 0 {
                            fprintf(
                                cofp,
                                b"%g %g %g %g %g line_highlight\n\0" as *const u8
                                    as *const libc::c_char,
                                lx,
                                y - baselineskip
                                    + font_bbox_lly * Fpt.h
                                        / 1000 as libc::c_int as libc::c_double,
                                linewidth,
                                Fpt.h + baselineskip,
                                line_highlight_gray,
                            );
                        }
                    }
                    if line_numbers != 0
                        && line_column == 0 as libc::c_int as libc::c_uint
                        && token.type_0 as libc::c_uint
                            != tFORMFEED as libc::c_int as libc::c_uint
                    {
                        print_line_number(
                            lx,
                            y,
                            linenumber_space,
                            linenumber_margin,
                            current_file_linenum,
                        );
                    }
                    match token.type_0 as libc::c_uint {
                        3 => {
                            match formfeed_type as libc::c_uint {
                                0 => {
                                    break;
                                }
                                1 => {
                                    break 's_135;
                                }
                                2 => {}
                                _ => {
                                    continue;
                                }
                            }
                            let mut current_row: libc::c_int = 0;
                            current_row = ((ly - y) / horizontal_column_height)
                                as libc::c_int;
                            y = ly
                                - (current_row + 1 as libc::c_int) as libc::c_double
                                    * horizontal_column_height;
                            if y < (d_footer_h + d_output_y_margin) as libc::c_double {
                                break;
                            }
                        }
                        2 => {
                            if slicing == 0 as libc::c_int || current_slice == slice {
                                if bggray < 1.0f64 {
                                    if cofp.is_null() {
                                        cofp = ofp;
                                    }
                                    if do_print != 0 {
                                        fprintf(
                                            cofp,
                                            b"%g %g %g %g %g (%s) bgs\n\0" as *const u8
                                                as *const libc::c_char,
                                            x,
                                            y,
                                            Fpt.h + baselineskip,
                                            baselineskip
                                                - font_bbox_lly * Fpt.h
                                                    / 1000 as libc::c_int as libc::c_double,
                                            bggray,
                                            token.u.str_0,
                                        );
                                    }
                                } else if user_bgcolorp != 0 {
                                    if cofp.is_null() {
                                        cofp = ofp;
                                    }
                                    if do_print != 0 {
                                        fprintf(
                                            cofp,
                                            b"%g %g %g %g %g %g %g (%s) bgcs\n\0" as *const u8
                                                as *const libc::c_char,
                                            x,
                                            y,
                                            Fpt.h + baselineskip,
                                            baselineskip
                                                - font_bbox_lly * Fpt.h
                                                    / 1000 as libc::c_int as libc::c_double,
                                            user_bgcolor.r as libc::c_double,
                                            user_bgcolor.g as libc::c_double,
                                            user_bgcolor.b as libc::c_double,
                                            token.u.str_0,
                                        );
                                    }
                                } else {
                                    if cofp.is_null() {
                                        cofp = ofp;
                                    }
                                    if do_print != 0 {
                                        fprintf(
                                            cofp,
                                            b"%g %g M\n(%s) s\n\0" as *const u8 as *const libc::c_char,
                                            x,
                                            y,
                                            token.u.str_0,
                                        );
                                    }
                                }
                            }
                            x = token.new_x;
                            line_column = token.new_col as libc::c_uint;
                        }
                        5 => {
                            x = ((col * d_output_w) as libc::c_float
                                / num_columns as libc::c_float
                                + d_output_x_margin as libc::c_float) as libc::c_double
                                + line_indent;
                            line_column = 0 as libc::c_int as libc::c_uint;
                        }
                        4 | 6 => {
                            if token.type_0 as libc::c_uint
                                == tNEWLINE as libc::c_int as libc::c_uint
                            {
                                current_file_linenum = current_file_linenum.wrapping_add(1);
                                current_file_linenum;
                                current_slice = 1 as libc::c_int as libc::c_uint;
                                y -= Fpt.h + baselineskip;
                            } else {
                                current_slice = current_slice.wrapping_add(1);
                                current_slice;
                                if slicing == 0 {
                                    match mark_wrapped_lines_style as libc::c_uint {
                                        0 => {}
                                        1 => {
                                            if cofp.is_null() {
                                                cofp = ofp;
                                            }
                                            if do_print != 0 {
                                                fprintf(
                                                    cofp,
                                                    b"%g %g M (+) s\n\0" as *const u8 as *const libc::c_char,
                                                    x,
                                                    y,
                                                );
                                            }
                                        }
                                        _ => {
                                            if cofp.is_null() {
                                                cofp = ofp;
                                            }
                                            if do_print != 0 {
                                                fprintf(
                                                    cofp,
                                                    b"%g %g %g %g %d wrapped_line_mark\n\0" as *const u8
                                                        as *const libc::c_char,
                                                    x,
                                                    y,
                                                    Fpt.w,
                                                    Fpt.h,
                                                    mark_wrapped_lines_style as libc::c_uint,
                                                );
                                            }
                                        }
                                    }
                                    y -= Fpt.h + baselineskip;
                                }
                                if slicing == 0 || current_slice > slice {
                                    if current_file_linenum != last_wrapped_line as libc::c_uint
                                    {
                                        if do_print != 0 {
                                            num_truncated_lines += 1;
                                            num_truncated_lines;
                                        }
                                        last_wrapped_line = current_file_linenum as libc::c_int;
                                    }
                                }
                            }
                            current_linenum = current_linenum.wrapping_add(1);
                            current_linenum;
                            if current_linenum >= lines_per_page
                                || y < (d_footer_h + d_output_y_margin) as libc::c_double
                            {
                                break;
                            }
                            x = ((col * d_output_w) as libc::c_float
                                / num_columns as libc::c_float
                                + d_output_x_margin as libc::c_float) as libc::c_double
                                + line_indent;
                            line_column = 0 as libc::c_int as libc::c_uint;
                        }
                        7 => {
                            if token.flags & 0x20 as libc::c_int as libc::c_uint != 0 {
                                token.new_y = ly;
                            } else {
                                token.new_y = y;
                            }
                            token.new_y += token.u.epsf.y - token.u.epsf.h;
                            if token.flags & 0x10 as libc::c_int as libc::c_uint != 0 {
                                token.new_x = lx;
                            } else {
                                token.new_x = x;
                            }
                            token.new_x += token.u.epsf.x;
                            if token.flags & 0x1 as libc::c_int as libc::c_uint != 0 {
                                token
                                    .new_x = lx
                                    + (linewidth - token.u.epsf.w)
                                        / 2 as libc::c_int as libc::c_double;
                            }
                            if token.flags & 0x2 as libc::c_int as libc::c_uint != 0 {
                                token.new_x = lx + (linewidth - token.u.epsf.w);
                            }
                            if token.flags & 0x8 as libc::c_int as libc::c_uint
                                == 0 as libc::c_int as libc::c_uint
                                && token.new_y
                                    < (d_footer_h + d_output_y_margin) as libc::c_double
                            {
                                if current_linenum == 0 as libc::c_int as libc::c_uint {
                                    if quiet == 0 && verbose >= 0 as libc::c_int {
                                        fprintf(
                                            stderr,
                                            dcgettext(
                                                0 as *const libc::c_char,
                                                b"EPS file \"%s\" is too large for page\n\0" as *const u8
                                                    as *const libc::c_char,
                                                5 as libc::c_int,
                                            ),
                                            (token.u.epsf.filename).as_mut_ptr(),
                                        );
                                    }
                                } else {
                                    reuse_last_token = 1 as libc::c_int;
                                    break;
                                }
                            }
                            if slicing == 0 as libc::c_int || current_slice == slice {
                                paste_epsf(&mut token);
                            }
                            if token.flags & 0x8 as libc::c_int as libc::c_uint == 0 {
                                y = token.new_y;
                            }
                            if token.flags & 0x4 as libc::c_int as libc::c_uint == 0 {
                                x = token.new_x + token.u.epsf.w;
                            }
                            if y < (d_footer_h + d_output_y_margin) as libc::c_double {
                                break;
                            }
                        }
                        11 => {
                            if line_column == 0 as libc::c_int as libc::c_uint {
                                let mut newh: libc::c_double = 0.;
                                if token.u.font.name[0 as libc::c_int as usize]
                                    as libc::c_int == '\0' as i32
                                {
                                    newh = default_Fpt.h;
                                } else {
                                    newh = token.u.font.size.h;
                                }
                                if newh != Fpt.h {
                                    y -= newh - Fpt.h;
                                }
                            }
                            if quiet == 0 && verbose >= 2 as libc::c_int {
                                fprintf(
                                    stderr,
                                    b"^@font=\0" as *const u8 as *const libc::c_char,
                                );
                            }
                            if token.u.font.name[0 as libc::c_int as usize]
                                as libc::c_int == '\0' as i32
                            {
                                Fpt.w = default_Fpt.w;
                                Fpt.h = default_Fpt.h;
                                Fname = default_Fname;
                                encoding = default_Fencoding;
                                if cofp.is_null() {
                                    cofp = ofp;
                                }
                                if do_print != 0 {
                                    fprintf(
                                        cofp,
                                        b"/F-gs-font %g %g SF\n\0" as *const u8
                                            as *const libc::c_char,
                                        Fpt.w,
                                        Fpt.h,
                                    );
                                }
                                user_fontp = 0 as libc::c_int;
                            } else {
                                strhash_put(
                                    res_fonts,
                                    (token.u.font.name).as_mut_ptr(),
                                    (strlen((token.u.font.name).as_mut_ptr()))
                                        .wrapping_add(1 as libc::c_int as libc::c_ulong)
                                        as libc::c_int,
                                    0 as *mut libc::c_void,
                                    0 as *mut *mut libc::c_void,
                                );
                                if token.u.font.encoding as libc::c_uint
                                    == default_Fencoding as libc::c_uint
                                {
                                    if cofp.is_null() {
                                        cofp = ofp;
                                    }
                                    if do_print != 0 {
                                        fprintf(
                                            cofp,
                                            b"/%s %g %g SUF\n\0" as *const u8 as *const libc::c_char,
                                            (token.u.font.name).as_mut_ptr(),
                                            token.u.font.size.w,
                                            token.u.font.size.h,
                                        );
                                    }
                                } else if token.u.font.encoding as libc::c_uint
                                    == ENC_PS as libc::c_int as libc::c_uint
                                {
                                    if cofp.is_null() {
                                        cofp = ofp;
                                    }
                                    if do_print != 0 {
                                        fprintf(
                                            cofp,
                                            b"/%s %g %g SUF_PS\n\0" as *const u8 as *const libc::c_char,
                                            (token.u.font.name).as_mut_ptr(),
                                            token.u.font.size.w,
                                            token.u.font.size.h,
                                        );
                                    }
                                } else {
                                    fprintf(
                                        stderr,
                                        b"%s: \0" as *const u8 as *const libc::c_char,
                                        program,
                                    );
                                    fprintf(
                                        stderr,
                                        dcgettext(
                                            0 as *const libc::c_char,
                                            b"user font encoding can be only the system's default or `ps'\0"
                                                as *const u8 as *const libc::c_char,
                                            5 as libc::c_int,
                                        ),
                                    );
                                    fprintf(
                                        stderr,
                                        b"\n\0" as *const u8 as *const libc::c_char,
                                    );
                                    fflush(stderr);
                                    exit(1 as libc::c_int);
                                }
                                memset(
                                    user_font_name.as_mut_ptr() as *mut libc::c_void,
                                    0 as libc::c_int,
                                    ::std::mem::size_of::<[libc::c_char; 4096]>()
                                        as libc::c_ulong,
                                );
                                strncpy(
                                    user_font_name.as_mut_ptr(),
                                    (token.u.font.name).as_mut_ptr(),
                                    (::std::mem::size_of::<[libc::c_char; 4096]>()
                                        as libc::c_ulong)
                                        .wrapping_sub(1 as libc::c_int as libc::c_ulong),
                                );
                                user_font_pt.w = token.u.font.size.w;
                                user_font_pt.h = token.u.font.size.h;
                                user_font_encoding = token.u.font.encoding;
                                user_fontp = 1 as libc::c_int;
                                Fpt.w = user_font_pt.w;
                                Fpt.h = user_font_pt.h;
                                Fname = user_font_name.as_mut_ptr();
                                encoding = user_font_encoding;
                            }
                            if quiet == 0 && verbose >= 2 as libc::c_int {
                                fprintf(
                                    stderr,
                                    b"%s %g/%gpt\n\0" as *const u8 as *const libc::c_char,
                                    Fname,
                                    Fpt.w,
                                    Fpt.h,
                                );
                            }
                            read_font_info();
                            if y < (d_footer_h + d_output_y_margin) as libc::c_double {
                                break;
                            }
                        }
                        12 => {
                            if quiet == 0 && verbose >= 2 as libc::c_int {
                                fprintf(
                                    stderr,
                                    b"^@color{%f %f %f}\n\0" as *const u8
                                        as *const libc::c_char,
                                    token.u.color.r as libc::c_double,
                                    token.u.color.g as libc::c_double,
                                    token.u.color.b as libc::c_double,
                                );
                            }
                            if token.u.color.r == token.u.color.g
                                && token.u.color.g == token.u.color.b
                                && token.u.color.b as libc::c_double == 0.0f64
                            {
                                if cofp.is_null() {
                                    cofp = ofp;
                                }
                                if do_print != 0 {
                                    fprintf(
                                        cofp,
                                        b"0 setgray\n\0" as *const u8 as *const libc::c_char,
                                    );
                                }
                                user_colorp = 0 as libc::c_int;
                            } else {
                                if cofp.is_null() {
                                    cofp = ofp;
                                }
                                if do_print != 0 {
                                    fprintf(
                                        cofp,
                                        b"%g %g %g setrgbcolor\n\0" as *const u8
                                            as *const libc::c_char,
                                        token.u.color.r as libc::c_double,
                                        token.u.color.g as libc::c_double,
                                        token.u.color.b as libc::c_double,
                                    );
                                }
                                user_color.r = token.u.color.r;
                                user_color.g = token.u.color.g;
                                user_color.b = token.u.color.b;
                                user_colorp = 1 as libc::c_int;
                            }
                        }
                        13 => {
                            if quiet == 0 && verbose >= 2 as libc::c_int {
                                fprintf(
                                    stderr,
                                    b"^@bgcolor{%f %f %f}\n\0" as *const u8
                                        as *const libc::c_char,
                                    token.u.color.r as libc::c_double,
                                    token.u.color.g as libc::c_double,
                                    token.u.color.b as libc::c_double,
                                );
                            }
                            if token.u.color.r == token.u.color.g
                                && token.u.color.g == token.u.color.b
                                && token.u.color.b as libc::c_double == 1.0f64
                            {
                                user_bgcolorp = 0 as libc::c_int;
                            } else {
                                user_bgcolor.r = token.u.color.r;
                                user_bgcolor.g = token.u.color.g;
                                user_bgcolor.b = token.u.color.b;
                                user_bgcolorp = 1 as libc::c_int;
                            }
                        }
                        8 => {
                            xfree(fname as *mut libc::c_void);
                            fname = xstrdup((token.u.filename).as_mut_ptr());
                        }
                        9 => {
                            current_pagenum = (token.u.i - 1 as libc::c_int)
                                as libc::c_uint;
                        }
                        10 => {
                            if current_linenum >= token.u.i as libc::c_uint {
                                break 's_135;
                            }
                        }
                        14 => {
                            xstore[token.u.i as libc::c_uchar as usize] = x;
                        }
                        15 => {
                            x = xstore[token.u.i as libc::c_uchar as usize];
                        }
                        16 => {
                            if cofp.is_null() {
                                cofp = ofp;
                            }
                            if do_print != 0 {
                                fprintf(
                                    cofp,
                                    b"%g %g M\n%s\n\0" as *const u8 as *const libc::c_char,
                                    x,
                                    y,
                                    token.u.str_0,
                                );
                            }
                            xfree(token.u.str_0 as *mut libc::c_void);
                        }
                        0 | _ => {
                            fprintf(
                                stderr,
                                b"%s: \0" as *const u8 as *const libc::c_char,
                                program,
                            );
                            fprintf(
                                stderr,
                                b"process_file(): got illegal token %d\0" as *const u8
                                    as *const libc::c_char,
                                token.type_0 as libc::c_uint,
                            );
                            fprintf(stderr, b"\n\0" as *const u8 as *const libc::c_char);
                            fflush(stderr);
                            exit(1 as libc::c_int);
                        }
                    }
                }
            }
            col += 1;
            col;
        }
        if page_clear == 0 {
            dump_ps_page_trailer();
        }
    }
    do_print = 1 as libc::c_int;
    undivert();
    if toc != 0 {
        let mut cp: *mut libc::c_char = 0 as *mut libc::c_char;
        let mut save_total_pages: libc::c_int = total_pages;
        total_pages = first_pagenum_for_file;
        cp = format_user_string(
            b"TOC\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            toc_fmt_string,
        );
        fprintf(toc_fp, b"%s\n\0" as *const u8 as *const libc::c_char, cp);
        xfree(cp as *mut libc::c_void);
        total_pages = save_total_pages;
    }
}
static mut escapes: [C2RustUnnamed_3; 15] = [
    {
        let mut init = C2RustUnnamed_3 {
            name: b"comment\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            escape: ESC_COMMENT,
        };
        init
    },
    {
        let mut init = C2RustUnnamed_3 {
            name: b"epsf\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            escape: ESC_EPSF,
        };
        init
    },
    {
        let mut init = C2RustUnnamed_3 {
            name: b"font\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            escape: ESC_FONT,
        };
        init
    },
    {
        let mut init = C2RustUnnamed_3 {
            name: b"color\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            escape: ESC_COLOR,
        };
        init
    },
    {
        let mut init = C2RustUnnamed_3 {
            name: b"bgcolor\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            escape: ESC_BGCOLOR,
        };
        init
    },
    {
        let mut init = C2RustUnnamed_3 {
            name: b"newpage\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            escape: ESC_NEWPAGE,
        };
        init
    },
    {
        let mut init = C2RustUnnamed_3 {
            name: b"ps\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            escape: ESC_PS,
        };
        init
    },
    {
        let mut init = C2RustUnnamed_3 {
            name: b"setfilename\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
            escape: ESC_SETFILENAME,
        };
        init
    },
    {
        let mut init = C2RustUnnamed_3 {
            name: b"setpagenumber\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
            escape: ESC_SETPAGENUMBER,
        };
        init
    },
    {
        let mut init = C2RustUnnamed_3 {
            name: b"shade\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            escape: ESC_SHADE,
        };
        init
    },
    {
        let mut init = C2RustUnnamed_3 {
            name: b"bggray\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            escape: ESC_BGGRAY,
        };
        init
    },
    {
        let mut init = C2RustUnnamed_3 {
            name: b"escape\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            escape: ESC_ESCAPE,
        };
        init
    },
    {
        let mut init = C2RustUnnamed_3 {
            name: b"savex\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            escape: ESC_SAVEX,
        };
        init
    },
    {
        let mut init = C2RustUnnamed_3 {
            name: b"loadx\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            escape: ESC_LOADX,
        };
        init
    },
    {
        let mut init = C2RustUnnamed_3 {
            name: 0 as *const libc::c_char as *mut libc::c_char,
            escape: ESC_COMMENT,
        };
        init
    },
];
unsafe extern "C" fn read_special_escape(
    mut is: *mut InputStream,
    mut token: *mut Token,
) {
    let mut escname: [libc::c_char; 256] = [0; 256];
    let mut buf: [libc::c_char; 4096] = [0; 4096];
    let mut i: libc::c_int = 0;
    let mut e: libc::c_int = 0;
    let mut ch: libc::c_int = 0;
    i = 0 as libc::c_int;
    while (i as libc::c_ulong)
        < (::std::mem::size_of::<[libc::c_char; 256]>() as libc::c_ulong)
            .wrapping_sub(1 as libc::c_int as libc::c_ulong)
        && {
            ch = is_getc(is);
            ch != -(1 as libc::c_int)
        }
    {
        if *(*__ctype_b_loc()).offset(ch as isize) as libc::c_int
            & _ISalnum as libc::c_int as libc::c_ushort as libc::c_int == 0
        {
            is_ungetc(ch, is);
            break;
        } else {
            escname[i as usize] = ch as libc::c_char;
            i += 1;
            i;
        }
    }
    escname[i as usize] = '\0' as i32 as libc::c_char;
    e = 0 as libc::c_int;
    while !(escapes[e as usize].name).is_null() {
        if strcmp(escname.as_mut_ptr(), escapes[e as usize].name) == 0 as libc::c_int {
            break;
        }
        e += 1;
        e;
    }
    if (escapes[e as usize].name).is_null() {
        fprintf(stderr, b"%s: \0" as *const u8 as *const libc::c_char, program);
        fprintf(
            stderr,
            dcgettext(
                0 as *const libc::c_char,
                b"unknown special escape: %s\0" as *const u8 as *const libc::c_char,
                5 as libc::c_int,
            ),
            escname.as_mut_ptr(),
        );
        fprintf(stderr, b"\n\0" as *const u8 as *const libc::c_char);
        fflush(stderr);
        exit(1 as libc::c_int);
    }
    if escapes[e as usize].escape as libc::c_uint
        == ESC_EPSF as libc::c_int as libc::c_uint
    {
        let mut i_0: libc::c_int = 0;
        let mut pw: libc::c_int = 0;
        let mut ph: libc::c_int = 0;
        let mut scale: libc::c_double = 0.;
        (*token).flags = 0 as libc::c_int as libc::c_uint;
        (*token).u.epsf.x = 0.0f64;
        (*token).u.epsf.y = 0.0f64;
        (*token).u.epsf.h = 0.0f64;
        (*token).u.epsf.pipe = 0 as libc::c_int;
        ch = is_getc(is);
        if ch == '[' as i32 {
            loop {
                ch = is_getc(is);
                if !(ch != -(1 as libc::c_int) && ch != ']' as i32) {
                    break;
                }
                match ch {
                    99 => {
                        (*token).flags &= !(0x3 as libc::c_int) as libc::c_uint;
                        (*token).flags |= 0x1 as libc::c_int as libc::c_uint;
                    }
                    110 => {
                        ch = is_getc(is);
                        match ch {
                            120 => {
                                (*token).flags |= 0x4 as libc::c_int as libc::c_uint;
                            }
                            121 => {
                                (*token).flags |= 0x8 as libc::c_int as libc::c_uint;
                            }
                            _ => {
                                is_ungetc(ch, is);
                                (*token).flags |= 0x4 as libc::c_int as libc::c_uint;
                                (*token).flags |= 0x8 as libc::c_int as libc::c_uint;
                            }
                        }
                    }
                    114 => {
                        (*token).flags &= !(0x3 as libc::c_int) as libc::c_uint;
                        (*token).flags |= 0x2 as libc::c_int as libc::c_uint;
                    }
                    115 => {
                        ch = is_getc(is);
                        match ch {
                            120 => {
                                (*token).flags |= 0x40 as libc::c_int as libc::c_uint;
                                (*token)
                                    .u
                                    .epsf
                                    .xscale = read_float(
                                    is,
                                    0 as libc::c_int,
                                    1 as libc::c_int,
                                );
                            }
                            121 => {
                                (*token).flags |= 0x80 as libc::c_int as libc::c_uint;
                                (*token)
                                    .u
                                    .epsf
                                    .yscale = read_float(
                                    is,
                                    0 as libc::c_int,
                                    0 as libc::c_int,
                                );
                            }
                            _ => {
                                is_ungetc(ch, is);
                                (*token).flags |= 0x40 as libc::c_int as libc::c_uint;
                                (*token).flags |= 0x80 as libc::c_int as libc::c_uint;
                                (*token)
                                    .u
                                    .epsf
                                    .yscale = read_float(
                                    is,
                                    0 as libc::c_int,
                                    1 as libc::c_int,
                                );
                                (*token).u.epsf.xscale = (*token).u.epsf.yscale;
                            }
                        }
                    }
                    120 => {
                        (*token)
                            .u
                            .epsf
                            .x = read_float(is, 1 as libc::c_int, 1 as libc::c_int);
                        ch = is_getc(is);
                        match ch {
                            97 => {
                                (*token).flags |= 0x10 as libc::c_int as libc::c_uint;
                            }
                            _ => {
                                is_ungetc(ch, is);
                            }
                        }
                    }
                    121 => {
                        (*token)
                            .u
                            .epsf
                            .y = -read_float(is, 1 as libc::c_int, 0 as libc::c_int);
                        ch = is_getc(is);
                        match ch {
                            97 => {
                                (*token).flags |= 0x20 as libc::c_int as libc::c_uint;
                            }
                            _ => {
                                is_ungetc(ch, is);
                            }
                        }
                    }
                    104 => {
                        (*token)
                            .u
                            .epsf
                            .h = read_float(is, 1 as libc::c_int, 0 as libc::c_int);
                    }
                    32 | 9 => {}
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
                                b"illegal option %c for ^@epsf escape\0" as *const u8
                                    as *const libc::c_char,
                                5 as libc::c_int,
                            ),
                            ch,
                        );
                        fprintf(stderr, b"\n\0" as *const u8 as *const libc::c_char);
                        fflush(stderr);
                        exit(1 as libc::c_int);
                    }
                }
            }
            if ch != ']' as i32 {
                fprintf(stderr, b"%s: \0" as *const u8 as *const libc::c_char, program);
                fprintf(
                    stderr,
                    dcgettext(
                        0 as *const libc::c_char,
                        b"malformed ^@epsf escape: no ']' after options\0" as *const u8
                            as *const libc::c_char,
                        5 as libc::c_int,
                    ),
                );
                fprintf(stderr, b"\n\0" as *const u8 as *const libc::c_char);
                fflush(stderr);
                exit(1 as libc::c_int);
            }
            ch = is_getc(is);
        }
        if ch == '{' as i32 {
            i_0 = 0 as libc::c_int;
            loop {
                ch = is_getc(is);
                if !(ch != -(1 as libc::c_int) && ch != '}' as i32) {
                    break;
                }
                (*token).u.epsf.filename[i_0 as usize] = ch as libc::c_char;
                if (i_0 + 1 as libc::c_int) as libc::c_ulong
                    >= ::std::mem::size_of::<[libc::c_char; 4096]>() as libc::c_ulong
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
                            b"too long file name for ^@epsf escape:\n%.*s\0" as *const u8
                                as *const libc::c_char,
                            5 as libc::c_int,
                        ),
                        i_0,
                        ((*token).u.epsf.filename).as_mut_ptr(),
                    );
                    fprintf(stderr, b"\n\0" as *const u8 as *const libc::c_char);
                    fflush(stderr);
                    exit(1 as libc::c_int);
                }
                i_0 += 1;
                i_0;
            }
            if ch == -(1 as libc::c_int) {
                fprintf(stderr, b"%s: \0" as *const u8 as *const libc::c_char, program);
                fprintf(
                    stderr,
                    dcgettext(
                        0 as *const libc::c_char,
                        b"unexpected EOF while scanning ^@epsf escape\0" as *const u8
                            as *const libc::c_char,
                        5 as libc::c_int,
                    ),
                );
                fprintf(stderr, b"\n\0" as *const u8 as *const libc::c_char);
                fflush(stderr);
                exit(1 as libc::c_int);
            }
            (*token).u.epsf.filename[i_0 as usize] = '\0' as i32 as libc::c_char;
            (*token).type_0 = tEPSF;
        } else {
            fprintf(stderr, b"%s: \0" as *const u8 as *const libc::c_char, program);
            fprintf(
                stderr,
                dcgettext(
                    0 as *const libc::c_char,
                    b"malformed ^@epsf escape: no '{' found\0" as *const u8
                        as *const libc::c_char,
                    5 as libc::c_int,
                ),
            );
            fprintf(stderr, b"\n\0" as *const u8 as *const libc::c_char);
            fflush(stderr);
            exit(1 as libc::c_int);
        }
        if recognize_eps_file(token) == 0 {
            (*token).type_0 = tNONE;
        } else {
            (*token).u.epsf.y
                += Fpt.h + baselineskip - 1 as libc::c_int as libc::c_double;
            if (*token).u.epsf.h != 0.0f64 {
                (*token).u.epsf.h -= 1.0f64;
            }
            pw = (*token).u.epsf.urx - (*token).u.epsf.llx;
            ph = (*token).u.epsf.ury - (*token).u.epsf.lly;
            if (*token).u.epsf.h == 0.0f64 {
                scale = 1.0f64;
            } else {
                scale = (*token).u.epsf.h / ph as libc::c_double;
            }
            if (*token).flags & 0x40 as libc::c_int as libc::c_uint
                == 0 as libc::c_int as libc::c_uint
            {
                (*token).u.epsf.xscale = scale;
            }
            if (*token).flags & 0x80 as libc::c_int as libc::c_uint
                == 0 as libc::c_int as libc::c_uint
            {
                (*token).u.epsf.yscale = scale;
            }
            pw = (pw as libc::c_double * (*token).u.epsf.xscale) as libc::c_int;
            ph = (ph as libc::c_double * (*token).u.epsf.yscale) as libc::c_int;
            (*token).u.epsf.w = pw as libc::c_double;
            (*token).u.epsf.h = ph as libc::c_double;
        }
    } else if escapes[e as usize].escape as libc::c_uint
        == ESC_COMMENT as libc::c_int as libc::c_uint
    {
        loop {
            ch = is_getc(is);
            if !(ch != -(1 as libc::c_int) && ch != nl) {
                break;
            }
        }
        (*token).type_0 = tNONE;
    } else {
        let mut cp: *mut libc::c_char = 0 as *mut libc::c_char;
        let mut parenlevel: libc::c_int = 0;
        ch = is_getc(is);
        if ch != '{' as i32 {
            fprintf(stderr, b"%s: \0" as *const u8 as *const libc::c_char, program);
            fprintf(
                stderr,
                dcgettext(
                    0 as *const libc::c_char,
                    b"malformed %s escape: no '{' found\0" as *const u8
                        as *const libc::c_char,
                    5 as libc::c_int,
                ),
                escapes[e as usize].name,
            );
            fprintf(stderr, b"\n\0" as *const u8 as *const libc::c_char);
            fflush(stderr);
            exit(1 as libc::c_int);
        }
        parenlevel = 0 as libc::c_int;
        i = 0 as libc::c_int;
        loop {
            ch = is_getc(is);
            if !(ch != -(1 as libc::c_int)
                && (parenlevel > 0 as libc::c_int || ch != '}' as i32))
            {
                break;
            }
            if ch == '{' as i32 {
                parenlevel += 1;
                parenlevel;
            } else if ch == '}' as i32 {
                parenlevel -= 1;
                parenlevel;
            }
            buf[i as usize] = ch as libc::c_char;
            if (i + 1 as libc::c_int) as libc::c_ulong
                >= ::std::mem::size_of::<[libc::c_char; 4096]>() as libc::c_ulong
            {
                fprintf(stderr, b"%s: \0" as *const u8 as *const libc::c_char, program);
                fprintf(
                    stderr,
                    dcgettext(
                        0 as *const libc::c_char,
                        b"too long argument for %s escape:\n%.*s\0" as *const u8
                            as *const libc::c_char,
                        5 as libc::c_int,
                    ),
                    escapes[e as usize].name,
                    i,
                    buf.as_mut_ptr(),
                );
                fprintf(stderr, b"\n\0" as *const u8 as *const libc::c_char);
                fflush(stderr);
                exit(1 as libc::c_int);
            }
            i += 1;
            i;
        }
        buf[i as usize] = '\0' as i32 as libc::c_char;
        match escapes[e as usize].escape as libc::c_uint {
            2 => {
                memset(
                    ((*token).u.font.name).as_mut_ptr() as *mut libc::c_void,
                    0 as libc::c_int,
                    ::std::mem::size_of::<[libc::c_char; 4096]>() as libc::c_ulong,
                );
                strncpy(
                    ((*token).u.font.name).as_mut_ptr(),
                    buf.as_mut_ptr(),
                    (::std::mem::size_of::<[libc::c_char; 4096]>() as libc::c_ulong)
                        .wrapping_sub(1 as libc::c_int as libc::c_ulong),
                );
                if strcmp(
                    ((*token).u.font.name).as_mut_ptr(),
                    b"default\0" as *const u8 as *const libc::c_char,
                ) == 0 as libc::c_int
                {
                    (*token)
                        .u
                        .font
                        .name[0 as libc::c_int as usize] = '\0' as i32 as libc::c_char;
                } else {
                    if parse_font_spec(
                        ((*token).u.font.name).as_mut_ptr(),
                        &mut cp,
                        &mut (*token).u.font.size,
                        &mut (*token).u.font.encoding,
                    ) == 0
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
                                b"malformed font spec for ^@font escape: %s\0" as *const u8
                                    as *const libc::c_char,
                                5 as libc::c_int,
                            ),
                            ((*token).u.font.name).as_mut_ptr(),
                        );
                        fprintf(stderr, b"\n\0" as *const u8 as *const libc::c_char);
                        fflush(stderr);
                        exit(1 as libc::c_int);
                    }
                    memset(
                        ((*token).u.font.name).as_mut_ptr() as *mut libc::c_void,
                        0 as libc::c_int,
                        ::std::mem::size_of::<[libc::c_char; 4096]>() as libc::c_ulong,
                    );
                    strncpy(
                        ((*token).u.font.name).as_mut_ptr(),
                        cp,
                        (::std::mem::size_of::<[libc::c_char; 4096]>() as libc::c_ulong)
                            .wrapping_sub(1 as libc::c_int as libc::c_ulong),
                    );
                    xfree(cp as *mut libc::c_void);
                }
                (*token).type_0 = tFONT;
            }
            3 | 4 => {
                if strcmp(
                    buf.as_mut_ptr(),
                    b"default\0" as *const u8 as *const libc::c_char,
                ) == 0 as libc::c_int
                {
                    let mut val: libc::c_double = 0 as libc::c_int as libc::c_double;
                    if escapes[e as usize].escape as libc::c_uint
                        == ESC_BGCOLOR as libc::c_int as libc::c_uint
                    {
                        val = 1 as libc::c_int as libc::c_double;
                    }
                    (*token).u.color.r = val as libc::c_float;
                    (*token).u.color.g = val as libc::c_float;
                    (*token).u.color.b = val as libc::c_float;
                } else {
                    let mut got: libc::c_int = 0;
                    got = sscanf(
                        buf.as_mut_ptr(),
                        b"%g %g %g\0" as *const u8 as *const libc::c_char,
                        &mut (*token).u.color.r as *mut libc::c_float,
                        &mut (*token).u.color.g as *mut libc::c_float,
                        &mut (*token).u.color.b as *mut libc::c_float,
                    );
                    match got {
                        0 | 2 => {
                            fprintf(
                                stderr,
                                b"%s: \0" as *const u8 as *const libc::c_char,
                                program,
                            );
                            fprintf(
                                stderr,
                                dcgettext(
                                    0 as *const libc::c_char,
                                    b"malformed color spec for ^@%s escape: %s\0" as *const u8
                                        as *const libc::c_char,
                                    5 as libc::c_int,
                                ),
                                if escapes[e as usize].escape as libc::c_uint
                                    == ESC_COLOR as libc::c_int as libc::c_uint
                                {
                                    b"color\0" as *const u8 as *const libc::c_char
                                } else {
                                    b"bgcolor\0" as *const u8 as *const libc::c_char
                                },
                                buf.as_mut_ptr(),
                            );
                            fprintf(stderr, b"\n\0" as *const u8 as *const libc::c_char);
                            fflush(stderr);
                            exit(1 as libc::c_int);
                        }
                        1 => {
                            (*token).u.color.b = (*token).u.color.r;
                            (*token).u.color.g = (*token).u.color.b;
                        }
                        _ => {}
                    }
                }
                if escapes[e as usize].escape as libc::c_uint
                    == ESC_COLOR as libc::c_int as libc::c_uint
                {
                    (*token).type_0 = tCOLOR;
                } else {
                    (*token).type_0 = tBGCOLOR;
                }
            }
            8 => {
                line_highlight_gray = atof(buf.as_mut_ptr());
                if line_highlight_gray < 0.0f64 || line_highlight_gray > 1.0f64 {
                    fprintf(
                        stderr,
                        b"%s: \0" as *const u8 as *const libc::c_char,
                        program,
                    );
                    fprintf(
                        stderr,
                        dcgettext(
                            0 as *const libc::c_char,
                            b"invalid value for ^@shade escape: %s\0" as *const u8
                                as *const libc::c_char,
                            5 as libc::c_int,
                        ),
                        buf.as_mut_ptr(),
                    );
                    fprintf(stderr, b"\n\0" as *const u8 as *const libc::c_char);
                    fflush(stderr);
                    exit(1 as libc::c_int);
                }
                (*token).type_0 = tNONE;
            }
            9 => {
                bggray = atof(buf.as_mut_ptr());
                if bggray < 0.0f64 || bggray > 1.0f64 {
                    fprintf(
                        stderr,
                        b"%s: \0" as *const u8 as *const libc::c_char,
                        program,
                    );
                    fprintf(
                        stderr,
                        dcgettext(
                            0 as *const libc::c_char,
                            b"invalid value for ^@bggray escape: %s\0" as *const u8
                                as *const libc::c_char,
                            5 as libc::c_int,
                        ),
                        buf.as_mut_ptr(),
                    );
                    fprintf(stderr, b"\n\0" as *const u8 as *const libc::c_char);
                    fflush(stderr);
                    exit(1 as libc::c_int);
                }
                (*token).type_0 = tNONE;
            }
            10 => {
                if strcmp(
                    buf.as_mut_ptr(),
                    b"default\0" as *const u8 as *const libc::c_char,
                ) == 0 as libc::c_int
                {
                    escape_char = default_escape_char;
                } else {
                    escape_char = atoi(buf.as_mut_ptr());
                }
                (*token).type_0 = tNONE;
            }
            6 => {
                memset(
                    ((*token).u.filename).as_mut_ptr() as *mut libc::c_void,
                    0 as libc::c_int,
                    ::std::mem::size_of::<[libc::c_char; 4096]>() as libc::c_ulong,
                );
                strncpy(
                    ((*token).u.filename).as_mut_ptr(),
                    buf.as_mut_ptr(),
                    (::std::mem::size_of::<[libc::c_char; 4096]>() as libc::c_ulong)
                        .wrapping_sub(1 as libc::c_int as libc::c_ulong),
                );
                (*token).type_0 = tSETFILENAME;
            }
            7 => {
                (*token).u.i = atoi(buf.as_mut_ptr());
                (*token).type_0 = tSETPAGENUMBER;
            }
            5 => {
                if i == 0 as libc::c_int {
                    (*token).u.i = 1 as libc::c_int;
                } else {
                    (*token).u.i = atoi(buf.as_mut_ptr());
                }
                (*token).type_0 = tNEWPAGE;
            }
            11 => {
                (*token).type_0 = tSAVEX;
                (*token).u.i = atoi(buf.as_mut_ptr());
            }
            12 => {
                (*token).type_0 = tLOADX;
                (*token).u.i = atoi(buf.as_mut_ptr());
            }
            13 => {
                (*token).u.str_0 = xstrdup(buf.as_mut_ptr());
                (*token).type_0 = tPS;
            }
            _ => {
                abort();
            }
        }
    };
}
unsafe extern "C" fn get_next_token(
    mut is: *mut InputStream,
    mut linestart: libc::c_double,
    mut linepos: libc::c_double,
    mut col: libc::c_uint,
    mut linew: libc::c_double,
    mut token: *mut Token,
) {
    static mut buffer: *mut libc::c_uchar = 0 as *const libc::c_uchar
        as *mut libc::c_uchar;
    static mut buflen: libc::c_uint = 0 as libc::c_int as libc::c_uint;
    let mut bufpos: libc::c_uint = 0 as libc::c_int as libc::c_uint;
    let mut ch: libc::c_int = 0 as libc::c_int;
    let mut done: libc::c_int = 0 as libc::c_int;
    let mut i: libc::c_int = 0;
    static mut pending_token: libc::c_int = tNONE as libc::c_int;
    let mut original_col: libc::c_uint = col;
    if pending_token != tNONE as libc::c_int {
        (*token).type_0 = pending_token as TokenType;
        pending_token = tNONE as libc::c_int;
        return;
    }
    while done == 0 {
        ch = is_getc(is);
        match ch {
            -1 => {
                if bufpos == 0 as libc::c_int as libc::c_uint {
                    (*token).type_0 = tEOF;
                    return;
                }
                done = 1 as libc::c_int;
            }
            13 | 10 => {
                if ch == nl {
                    if bufpos == 0 as libc::c_int as libc::c_uint {
                        (*token).type_0 = tNEWLINE;
                        return;
                    } else {
                        is_ungetc(ch, is);
                        done = 1 as libc::c_int;
                    }
                } else if bufpos == 0 as libc::c_int as libc::c_uint {
                    (*token).type_0 = tCARRIAGE_RETURN;
                    return;
                } else {
                    is_ungetc(ch, is);
                    done = 1 as libc::c_int;
                }
            }
            9 => {
                if font_is_fixed != 0 {
                    i = (tabsize as libc::c_uint)
                        .wrapping_sub(col.wrapping_rem(tabsize as libc::c_uint))
                        as libc::c_int;
                    while i > 0 as libc::c_int {
                        if linepos
                            + *font_widths
                                .as_mut_ptr()
                                .offset(' ' as i32 as libc::c_uchar as isize) < linew
                            || col == 0 as libc::c_int as libc::c_uint
                        {
                            if bufpos >= buflen {
                                buflen = buflen
                                    .wrapping_add(4096 as libc::c_int as libc::c_uint);
                                buffer = xrealloc(
                                    buffer as *mut libc::c_void,
                                    buflen as size_t,
                                ) as *mut libc::c_uchar;
                            }
                            let fresh0 = bufpos;
                            bufpos = bufpos.wrapping_add(1);
                            *buffer
                                .offset(fresh0 as isize) = ' ' as i32 as libc::c_uchar;
                            linepos
                                += *font_widths
                                    .as_mut_ptr()
                                    .offset(' ' as i32 as libc::c_uchar as isize);
                            col = col.wrapping_add(1);
                            col;
                            i -= 1;
                            i;
                        } else {
                            done = 2 as libc::c_int;
                            break;
                        }
                    }
                } else {
                    let mut grid: libc::c_double = tabsize as libc::c_double
                        * *font_widths
                            .as_mut_ptr()
                            .offset(' ' as i32 as libc::c_uchar as isize);
                    col = col.wrapping_add(1);
                    col;
                    linepos = (((linepos - linestart) / grid) as libc::c_int
                        + 1 as libc::c_int) as libc::c_double * grid + linestart;
                    if linepos >= linew {
                        done = 2 as libc::c_int;
                    } else {
                        done = 1 as libc::c_int;
                    }
                }
            }
            12 => {
                if bufpos == 0 as libc::c_int as libc::c_uint {
                    if interpret_formfeed != 0 {
                        (*token).type_0 = tFORMFEED;
                    } else {
                        (*token).type_0 = tNEWLINE;
                    }
                    return;
                } else {
                    is_ungetc(ch, is);
                    done = 1 as libc::c_int;
                }
            }
            _ => {
                if special_escapes != 0 && ch == escape_char {
                    if bufpos == 0 as libc::c_int as libc::c_uint {
                        read_special_escape(is, token);
                        if (*token).type_0 as libc::c_uint
                            != tNONE as libc::c_int as libc::c_uint
                        {
                            return;
                        }
                    } else {
                        is_ungetc(ch, is);
                        done = 1 as libc::c_int;
                    }
                } else if ch == bs {
                    if bufpos == 0 as libc::c_int as libc::c_uint
                        || !(*font_ctype
                            .as_mut_ptr()
                            .offset(
                                *buffer
                                    .offset(
                                        bufpos.wrapping_sub(1 as libc::c_int as libc::c_uint)
                                            as isize,
                                    ) as isize,
                            ) as libc::c_int == '*' as i32)
                    {
                        linepos
                            -= *font_widths
                                .as_mut_ptr()
                                .offset('m' as i32 as libc::c_uchar as isize);
                    } else {
                        linepos
                            -= *font_widths
                                .as_mut_ptr()
                                .offset(
                                    *buffer
                                        .offset(
                                            bufpos.wrapping_sub(1 as libc::c_int as libc::c_uint)
                                                as isize,
                                        ) as isize,
                                );
                    }
                    done = 1 as libc::c_int;
                } else if *font_ctype.as_mut_ptr().offset(ch as libc::c_uchar as isize)
                    as libc::c_int == '*' as i32
                {
                    if linepos
                        + *font_widths.as_mut_ptr().offset(ch as libc::c_uchar as isize)
                        < linew || col == 0 as libc::c_int as libc::c_uint
                    {
                        if ch < 0o40 as libc::c_int
                            || clean_7bit != 0 && ch >= 0o200 as libc::c_int
                        {
                            let mut buf: [libc::c_char; 10] = [0; 10];
                            sprintf(
                                buf.as_mut_ptr(),
                                b"\\%03o\0" as *const u8 as *const libc::c_char,
                                ch,
                            );
                            i = 0 as libc::c_int;
                            while buf[i as usize] != 0 {
                                if bufpos >= buflen {
                                    buflen = buflen
                                        .wrapping_add(4096 as libc::c_int as libc::c_uint);
                                    buffer = xrealloc(
                                        buffer as *mut libc::c_void,
                                        buflen as size_t,
                                    ) as *mut libc::c_uchar;
                                }
                                let fresh1 = bufpos;
                                bufpos = bufpos.wrapping_add(1);
                                *buffer
                                    .offset(fresh1 as isize) = buf[i as usize] as libc::c_uchar;
                                i += 1;
                                i;
                            }
                            linepos
                                += *font_widths
                                    .as_mut_ptr()
                                    .offset(ch as libc::c_uchar as isize);
                            col = col.wrapping_add(1);
                            col;
                        } else if ch == '(' as i32 || ch == ')' as i32
                            || ch == '\\' as i32
                        {
                            if bufpos >= buflen {
                                buflen = buflen
                                    .wrapping_add(4096 as libc::c_int as libc::c_uint);
                                buffer = xrealloc(
                                    buffer as *mut libc::c_void,
                                    buflen as size_t,
                                ) as *mut libc::c_uchar;
                            }
                            let fresh2 = bufpos;
                            bufpos = bufpos.wrapping_add(1);
                            *buffer
                                .offset(fresh2 as isize) = '\\' as i32 as libc::c_uchar;
                            if bufpos >= buflen {
                                buflen = buflen
                                    .wrapping_add(4096 as libc::c_int as libc::c_uint);
                                buffer = xrealloc(
                                    buffer as *mut libc::c_void,
                                    buflen as size_t,
                                ) as *mut libc::c_uchar;
                            }
                            let fresh3 = bufpos;
                            bufpos = bufpos.wrapping_add(1);
                            *buffer.offset(fresh3 as isize) = ch as libc::c_uchar;
                            linepos
                                += *font_widths
                                    .as_mut_ptr()
                                    .offset(ch as libc::c_uchar as isize);
                            col = col.wrapping_add(1);
                            col;
                        } else {
                            if bufpos >= buflen {
                                buflen = buflen
                                    .wrapping_add(4096 as libc::c_int as libc::c_uint);
                                buffer = xrealloc(
                                    buffer as *mut libc::c_void,
                                    buflen as size_t,
                                ) as *mut libc::c_uchar;
                            }
                            let fresh4 = bufpos;
                            bufpos = bufpos.wrapping_add(1);
                            *buffer.offset(fresh4 as isize) = ch as libc::c_uchar;
                            linepos
                                += *font_widths
                                    .as_mut_ptr()
                                    .offset(ch as libc::c_uchar as isize);
                            col = col.wrapping_add(1);
                            col;
                        }
                    } else {
                        is_ungetc(ch, is);
                        done = 2 as libc::c_int;
                    }
                } else if *font_ctype.as_mut_ptr().offset(ch as libc::c_uchar as isize)
                    as libc::c_int != ' ' as i32
                {
                    if linepos
                        + *font_widths
                            .as_mut_ptr()
                            .offset('?' as i32 as libc::c_uchar as isize) < linew
                        || col == 0 as libc::c_int as libc::c_uint
                    {
                        if bufpos >= buflen {
                            buflen = buflen
                                .wrapping_add(4096 as libc::c_int as libc::c_uint);
                            buffer = xrealloc(
                                buffer as *mut libc::c_void,
                                buflen as size_t,
                            ) as *mut libc::c_uchar;
                        }
                        let fresh5 = bufpos;
                        bufpos = bufpos.wrapping_add(1);
                        *buffer.offset(fresh5 as isize) = '?' as i32 as libc::c_uchar;
                        linepos
                            += *font_widths
                                .as_mut_ptr()
                                .offset('?' as i32 as libc::c_uchar as isize);
                        col = col.wrapping_add(1);
                        col;
                        let ref mut fresh6 = *missing_chars
                            .as_mut_ptr()
                            .offset(ch as isize);
                        let fresh7 = *fresh6;
                        *fresh6 = *fresh6 + 1;
                        if fresh7 == 0 as libc::c_int {
                            num_missing_chars += 1;
                            num_missing_chars;
                        }
                    } else {
                        is_ungetc(ch, is);
                        done = 2 as libc::c_int;
                    }
                } else {
                    let mut buf_0: [libc::c_char; 20] = [0; 20];
                    let mut len: libc::c_double = 0.0f64;
                    let ref mut fresh8 = *non_printable_chars
                        .as_mut_ptr()
                        .offset(ch as isize);
                    let fresh9 = *fresh8;
                    *fresh8 = *fresh8 + 1;
                    if fresh9 == 0 as libc::c_int {
                        num_non_printable_chars += 1;
                        num_non_printable_chars;
                    }
                    let mut current_block_145: u64;
                    match non_printable_format as libc::c_uint {
                        0 => {
                            strcpy(
                                buf_0.as_mut_ptr(),
                                b" \0" as *const u8 as *const libc::c_char,
                            );
                            current_block_145 = 7761909515536616543;
                        }
                        1 => {
                            strcpy(
                                buf_0.as_mut_ptr(),
                                b"?\0" as *const u8 as *const libc::c_char,
                            );
                            current_block_145 = 7761909515536616543;
                        }
                        2 => {
                            if ch < 0x20 as libc::c_int {
                                buf_0[0 as libc::c_int
                                    as usize] = '^' as i32 as libc::c_char;
                                buf_0[1 as libc::c_int
                                    as usize] = ('@' as i32 + ch) as libc::c_char;
                                buf_0[2 as libc::c_int
                                    as usize] = '\0' as i32 as libc::c_char;
                                current_block_145 = 7761909515536616543;
                            } else {
                                current_block_145 = 13655882557890020174;
                            }
                        }
                        3 => {
                            current_block_145 = 13655882557890020174;
                        }
                        _ => {
                            current_block_145 = 7761909515536616543;
                        }
                    }
                    match current_block_145 {
                        13655882557890020174 => {
                            sprintf(
                                buf_0.as_mut_ptr(),
                                b"\\%03o\0" as *const u8 as *const libc::c_char,
                                ch,
                            );
                        }
                        _ => {}
                    }
                    i = 0 as libc::c_int;
                    while buf_0[i as usize] != 0 {
                        len
                            += *font_widths
                                .as_mut_ptr()
                                .offset(buf_0[i as usize] as libc::c_uchar as isize);
                        i += 1;
                        i;
                    }
                    if linepos + len < linew || col == 0 as libc::c_int as libc::c_uint {
                        i = 0 as libc::c_int;
                        while buf_0[i as usize] != 0 {
                            if buf_0[i as usize] as libc::c_int == '\\' as i32 {
                                if bufpos >= buflen {
                                    buflen = buflen
                                        .wrapping_add(4096 as libc::c_int as libc::c_uint);
                                    buffer = xrealloc(
                                        buffer as *mut libc::c_void,
                                        buflen as size_t,
                                    ) as *mut libc::c_uchar;
                                }
                                let fresh10 = bufpos;
                                bufpos = bufpos.wrapping_add(1);
                                *buffer
                                    .offset(fresh10 as isize) = '\\' as i32 as libc::c_uchar;
                            }
                            if bufpos >= buflen {
                                buflen = buflen
                                    .wrapping_add(4096 as libc::c_int as libc::c_uint);
                                buffer = xrealloc(
                                    buffer as *mut libc::c_void,
                                    buflen as size_t,
                                ) as *mut libc::c_uchar;
                            }
                            let fresh11 = bufpos;
                            bufpos = bufpos.wrapping_add(1);
                            *buffer
                                .offset(
                                    fresh11 as isize,
                                ) = buf_0[i as usize] as libc::c_uchar;
                            linepos
                                += *font_widths
                                    .as_mut_ptr()
                                    .offset(buf_0[i as usize] as libc::c_uchar as isize);
                            col = col.wrapping_add(1);
                            col;
                            i += 1;
                            i;
                        }
                    } else {
                        is_ungetc(ch, is);
                        done = 2 as libc::c_int;
                    }
                }
            }
        }
    }
    if done == 2 as libc::c_int {
        ch = nl;
        if line_end as libc::c_uint == LE_TRUNCATE as libc::c_int as libc::c_uint {
            loop {
                ch = is_getc(is);
                if !(ch != -(1 as libc::c_int) && ch != nl) {
                    break;
                }
            }
        } else if !(bufpos == 0 as libc::c_int as libc::c_uint)
            && line_end as libc::c_uint == LE_WORD_WRAP as libc::c_int as libc::c_uint
        {
            let mut w: libc::c_int = 0;
            if *buffer
                .offset(bufpos.wrapping_sub(1 as libc::c_int as libc::c_uint) as isize)
                as libc::c_int == ' ' as i32
                || *buffer
                    .offset(
                        bufpos.wrapping_sub(1 as libc::c_int as libc::c_uint) as isize,
                    ) as libc::c_int == '\t' as i32
            {
                loop {
                    w = is_getc(is);
                    if !(w != -(1 as libc::c_int)
                        && (w == ' ' as i32 || w == '\t' as i32))
                    {
                        break;
                    }
                }
                is_ungetc(w, is);
            } else {
                w = bufpos.wrapping_sub(1 as libc::c_int as libc::c_uint) as libc::c_int;
                while w >= 0 as libc::c_int
                    && !(*buffer.offset(w as isize) as libc::c_int == ' ' as i32
                        || *buffer.offset(w as isize) as libc::c_int == '\t' as i32)
                {
                    w -= 1;
                    w;
                }
                w += 1;
                w;
                if w > 0 as libc::c_int
                    || original_col > 0 as libc::c_int as libc::c_uint
                {
                    loop {
                        bufpos = bufpos.wrapping_sub(1);
                        bufpos;
                        if bufpos > w as libc::c_uint
                            && (*buffer.offset(bufpos as isize) as libc::c_int
                                == '(' as i32
                                || *buffer.offset(bufpos as isize) as libc::c_int
                                    == ')' as i32
                                || *buffer.offset(bufpos as isize) as libc::c_int
                                    == '\\' as i32)
                            && *buffer
                                .offset(
                                    bufpos.wrapping_sub(1 as libc::c_int as libc::c_uint)
                                        as isize,
                                ) as libc::c_int == '\\' as i32
                        {
                            is_ungetc(
                                *buffer.offset(bufpos as isize) as libc::c_int,
                                is,
                            );
                            linepos
                                -= *font_widths
                                    .as_mut_ptr()
                                    .offset(*buffer.offset(bufpos as isize) as isize);
                            col = col.wrapping_sub(1);
                            col;
                            bufpos = bufpos.wrapping_sub(1);
                            bufpos;
                        } else {
                            let mut current_block_226: u64;
                            if bufpos.wrapping_sub(2 as libc::c_int as libc::c_uint)
                                > w as libc::c_uint
                                && ('0' as i32
                                    <= *buffer.offset(bufpos as isize) as libc::c_int
                                    && *buffer.offset(bufpos as isize) as libc::c_int
                                        <= '7' as i32)
                                && ('0' as i32
                                    <= *buffer
                                        .offset(
                                            bufpos.wrapping_sub(1 as libc::c_int as libc::c_uint)
                                                as isize,
                                        ) as libc::c_int
                                    && *buffer
                                        .offset(
                                            bufpos.wrapping_sub(1 as libc::c_int as libc::c_uint)
                                                as isize,
                                        ) as libc::c_int <= '7' as i32)
                                && ('0' as i32
                                    <= *buffer
                                        .offset(
                                            bufpos.wrapping_sub(2 as libc::c_int as libc::c_uint)
                                                as isize,
                                        ) as libc::c_int
                                    && *buffer
                                        .offset(
                                            bufpos.wrapping_sub(2 as libc::c_int as libc::c_uint)
                                                as isize,
                                        ) as libc::c_int <= '7' as i32)
                                && *buffer
                                    .offset(
                                        bufpos.wrapping_sub(3 as libc::c_int as libc::c_uint)
                                            as isize,
                                    ) as libc::c_int == '\\' as i32
                            {
                                let mut ti: libc::c_uint = 0;
                                ti = w as libc::c_uint;
                                while ti
                                    < bufpos.wrapping_sub(3 as libc::c_int as libc::c_uint)
                                {
                                    if *buffer.offset(ti as isize) as libc::c_int == '\\' as i32
                                    {
                                        if '0' as i32
                                            <= *buffer
                                                .offset(
                                                    ti.wrapping_add(1 as libc::c_int as libc::c_uint) as isize,
                                                ) as libc::c_int
                                            && *buffer
                                                .offset(
                                                    ti.wrapping_add(1 as libc::c_int as libc::c_uint) as isize,
                                                ) as libc::c_int <= '7' as i32
                                        {
                                            let mut tti: libc::c_uint = 0;
                                            tti = 0 as libc::c_int as libc::c_uint;
                                            while tti < 3 as libc::c_int as libc::c_uint
                                                && ('0' as i32
                                                    <= *buffer
                                                        .offset(
                                                            ti.wrapping_add(1 as libc::c_int as libc::c_uint) as isize,
                                                        ) as libc::c_int
                                                    && *buffer
                                                        .offset(
                                                            ti.wrapping_add(1 as libc::c_int as libc::c_uint) as isize,
                                                        ) as libc::c_int <= '7' as i32)
                                            {
                                                tti = tti.wrapping_add(1);
                                                tti;
                                                ti = ti.wrapping_add(1);
                                                ti;
                                            }
                                        } else {
                                            ti = ti.wrapping_add(1);
                                            ti;
                                        }
                                    }
                                    ti = ti.wrapping_add(1);
                                    ti;
                                }
                                if ti
                                    == bufpos.wrapping_sub(3 as libc::c_int as libc::c_uint)
                                {
                                    let mut tch: libc::c_int = 0;
                                    tch = ((*buffer
                                        .offset(
                                            bufpos.wrapping_sub(2 as libc::c_int as libc::c_uint)
                                                as isize,
                                        ) as libc::c_int - '0' as i32) << 6 as libc::c_int)
                                        + ((*buffer
                                            .offset(
                                                bufpos.wrapping_sub(1 as libc::c_int as libc::c_uint)
                                                    as isize,
                                            ) as libc::c_int - '0' as i32) << 3 as libc::c_int)
                                        + (*buffer.offset(bufpos as isize) as libc::c_int
                                            - '0' as i32);
                                    is_ungetc(tch, is);
                                    linepos
                                        -= *font_widths
                                            .as_mut_ptr()
                                            .offset(tch as libc::c_uchar as isize);
                                    col = col.wrapping_sub(1);
                                    col;
                                    bufpos = bufpos
                                        .wrapping_sub(3 as libc::c_int as libc::c_uint);
                                    current_block_226 = 13164310931121142693;
                                } else {
                                    current_block_226 = 14810870812227877689;
                                }
                            } else {
                                current_block_226 = 14810870812227877689;
                            }
                            match current_block_226 {
                                14810870812227877689 => {
                                    is_ungetc(
                                        *buffer.offset(bufpos as isize) as libc::c_int,
                                        is,
                                    );
                                    linepos
                                        -= *font_widths
                                            .as_mut_ptr()
                                            .offset(*buffer.offset(bufpos as isize) as isize);
                                    col = col.wrapping_sub(1);
                                    col;
                                }
                                _ => {}
                            }
                        }
                        if !(bufpos > w as libc::c_uint) {
                            break;
                        }
                    }
                }
            }
        }
        if ch == nl {
            if line_end as libc::c_uint == LE_TRUNCATE as libc::c_int as libc::c_uint {
                if do_print != 0 {
                    num_truncated_lines += 1;
                    num_truncated_lines;
                }
                pending_token = tNEWLINE as libc::c_int;
            } else {
                pending_token = tWRAPPED_NEWLINE as libc::c_int;
            }
        } else {
            pending_token = tEOF as libc::c_int;
        }
    }
    if bufpos >= buflen {
        buflen = buflen.wrapping_add(4096 as libc::c_int as libc::c_uint);
        buffer = xrealloc(buffer as *mut libc::c_void, buflen as size_t)
            as *mut libc::c_uchar;
    }
    let fresh12 = bufpos;
    bufpos = bufpos.wrapping_add(1);
    *buffer.offset(fresh12 as isize) = '\0' as i32 as libc::c_uchar;
    (*token).type_0 = tSTRING;
    (*token).u.str_0 = buffer as *mut libc::c_char;
    (*token).new_x = linepos;
    (*token).new_col = col as libc::c_int;
}
unsafe extern "C" fn dump_ps_page_header(
    mut fname_0: *mut libc::c_char,
    mut empty: libc::c_int,
) {
    let mut dirc: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut basec: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut fdir: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut ftail: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut got: libc::c_int = 0;
    let mut i: libc::c_int = 0;
    let mut cp: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut cp2: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut cstr: *mut libc::c_char = b"%%\0" as *const u8 as *const libc::c_char
        as *mut libc::c_char;
    let mut nup_subpage: libc::c_uint = 0;
    nup_subpage = ((total_pages - 1 as libc::c_int) as libc::c_uint).wrapping_rem(nup);
    dirc = strdup(fname_0);
    basec = strdup(fname_0);
    fdir = dirname(dirc);
    ftail = __xpg_basename(basec);
    if nup > 1 as libc::c_int as libc::c_uint {
        cstr = b"%\0" as *const u8 as *const libc::c_char as *mut libc::c_char;
        if nup_subpage == 0 as libc::c_int as libc::c_uint {
            match page_label as libc::c_uint {
                0 => {
                    if cofp.is_null() {
                        cofp = ofp;
                    }
                    if do_print != 0 {
                        fprintf(
                            cofp,
                            b"%%%%Page: (%d-%d) %d\n\0" as *const u8
                                as *const libc::c_char,
                            current_pagenum,
                            current_pagenum
                                .wrapping_add(nup)
                                .wrapping_sub(1 as libc::c_int as libc::c_uint),
                            (total_pages as libc::c_uint)
                                .wrapping_div(nup)
                                .wrapping_add(1 as libc::c_int as libc::c_uint),
                        );
                    }
                }
                1 => {
                    if cofp.is_null() {
                        cofp = ofp;
                    }
                    if do_print != 0 {
                        fprintf(
                            cofp,
                            b"%%%%Page: (%s:%3d-%3d) %d\n\0" as *const u8
                                as *const libc::c_char,
                            ftail,
                            current_pagenum,
                            current_pagenum
                                .wrapping_add(nup)
                                .wrapping_sub(1 as libc::c_int as libc::c_uint),
                            (total_pages as libc::c_uint)
                                .wrapping_div(nup)
                                .wrapping_add(1 as libc::c_int as libc::c_uint),
                        );
                    }
                }
                _ => {}
            }
            if cofp.is_null() {
                cofp = ofp;
            }
            if do_print != 0 {
                fprintf(
                    cofp,
                    b"%%%%BeginPageSetup\n_S\n\0" as *const u8 as *const libc::c_char,
                );
            }
            if (total_pages as libc::c_uint)
                .wrapping_div(nup)
                .wrapping_add(1 as libc::c_int as libc::c_uint)
                .wrapping_rem(2 as libc::c_int as libc::c_uint)
                == 0 as libc::c_int as libc::c_uint
            {
                handle_two_side_options();
            }
            if landscape != 0 {
                if nup_landscape != 0 {
                    if cofp.is_null() {
                        cofp = ofp;
                    }
                    if do_print != 0 {
                        fprintf(
                            cofp,
                            b"90 rotate\n%d %d translate\n\0" as *const u8
                                as *const libc::c_char,
                            (*media).lly,
                            -(*media).urx,
                        );
                    }
                } else {
                    if cofp.is_null() {
                        cofp = ofp;
                    }
                    if do_print != 0 {
                        fprintf(
                            cofp,
                            b"%d %d translate\n\0" as *const u8 as *const libc::c_char,
                            (*media).llx,
                            (*media).lly,
                        );
                    }
                }
            } else if nup_landscape != 0 {
                if cofp.is_null() {
                    cofp = ofp;
                }
                if do_print != 0 {
                    fprintf(
                        cofp,
                        b"90 rotate\n%d %d translate\n\0" as *const u8
                            as *const libc::c_char,
                        (*media).lly,
                        -(*media).llx,
                    );
                }
            } else {
                if cofp.is_null() {
                    cofp = ofp;
                }
                if do_print != 0 {
                    fprintf(
                        cofp,
                        b"%d %d translate\n\0" as *const u8 as *const libc::c_char,
                        (*media).llx,
                        (*media).ury,
                    );
                }
            }
        }
    }
    match page_label as libc::c_uint {
        0 => {
            if cofp.is_null() {
                cofp = ofp;
            }
            if do_print != 0 {
                fprintf(
                    cofp,
                    b"%sPage: (%d) %d\n\0" as *const u8 as *const libc::c_char,
                    cstr,
                    current_pagenum,
                    total_pages,
                );
            }
        }
        1 => {
            if cofp.is_null() {
                cofp = ofp;
            }
            if do_print != 0 {
                fprintf(
                    cofp,
                    b"%sPage: (%s:%3d) %d\n\0" as *const u8 as *const libc::c_char,
                    cstr,
                    ftail,
                    current_pagenum,
                    total_pages,
                );
            }
        }
        _ => {}
    }
    if cofp.is_null() {
        cofp = ofp;
    }
    if do_print != 0 {
        fprintf(
            cofp,
            b"%sBeginPageSetup\n_S\n\0" as *const u8 as *const libc::c_char,
            cstr,
        );
    }
    if nup > 1 as libc::c_int as libc::c_uint {
        let mut xm: libc::c_int = 0;
        let mut ym: libc::c_int = 0;
        if cofp.is_null() {
            cofp = ofp;
        }
        if do_print != 0 {
            fprintf(
                cofp,
                b"%% N-up sub-page %d/%d\n\0" as *const u8 as *const libc::c_char,
                nup_subpage.wrapping_add(1 as libc::c_int as libc::c_uint),
                nup,
            );
        }
        if landscape != 0 {
            if nup_columnwise != 0 {
                xm = nup_subpage.wrapping_rem(nup_columns) as libc::c_int;
                ym = nup_subpage.wrapping_div(nup_columns) as libc::c_int;
            } else {
                xm = nup_subpage.wrapping_div(nup_rows) as libc::c_int;
                ym = nup_subpage.wrapping_rem(nup_rows) as libc::c_int;
            }
            if cofp.is_null() {
                cofp = ofp;
            }
            if do_print != 0 {
                fprintf(
                    cofp,
                    b"%d %d translate\n\0" as *const u8 as *const libc::c_char,
                    (xm as libc::c_uint).wrapping_mul(nup_width.wrapping_add(nup_xpad)),
                    (ym as libc::c_uint).wrapping_mul(nup_height.wrapping_add(nup_ypad)),
                );
            }
        } else {
            if nup_columnwise != 0 {
                xm = nup_subpage.wrapping_div(nup_rows) as libc::c_int;
                ym = nup_subpage.wrapping_rem(nup_rows) as libc::c_int;
            } else {
                xm = nup_subpage.wrapping_rem(nup_columns) as libc::c_int;
                ym = nup_subpage.wrapping_div(nup_columns) as libc::c_int;
            }
            if cofp.is_null() {
                cofp = ofp;
            }
            if do_print != 0 {
                fprintf(
                    cofp,
                    b"%d %d translate\n\0" as *const u8 as *const libc::c_char,
                    (xm as libc::c_uint).wrapping_mul(nup_width.wrapping_add(nup_xpad)),
                    -((ym as libc::c_uint)
                        .wrapping_mul(nup_height.wrapping_add(nup_ypad))
                        .wrapping_add(nup_height) as libc::c_int),
                );
            }
        }
        if cofp.is_null() {
            cofp = ofp;
        }
        if do_print != 0 {
            fprintf(
                cofp,
                b"%g dup scale\n\0" as *const u8 as *const libc::c_char,
                nup_scale,
            );
        }
        if landscape != 0 {
            if cofp.is_null() {
                cofp = ofp;
            }
            if do_print != 0 {
                fprintf(
                    cofp,
                    b"90 rotate\n%d %d translate\n\0" as *const u8
                        as *const libc::c_char,
                    0 as libc::c_int,
                    -d_page_h,
                );
            }
        }
    } else {
        if total_pages % 2 as libc::c_int == 0 as libc::c_int {
            handle_two_side_options();
        }
        if landscape != 0 {
            if cofp.is_null() {
                cofp = ofp;
            }
            if do_print != 0 {
                fprintf(
                    cofp,
                    b"90 rotate\n%d %d translate\n\0" as *const u8
                        as *const libc::c_char,
                    (*media).lly,
                    -(*media).urx,
                );
            }
        } else {
            if cofp.is_null() {
                cofp = ofp;
            }
            if do_print != 0 {
                fprintf(
                    cofp,
                    b"%d %d translate\n\0" as *const u8 as *const libc::c_char,
                    (*media).llx,
                    (*media).lly,
                );
            }
        }
    }
    if cofp.is_null() {
        cofp = ofp;
    }
    if do_print != 0 {
        fprintf(
            cofp,
            b"/pagenum %d def\n\0" as *const u8 as *const libc::c_char,
            current_pagenum,
        );
    }
    cp = escape_string(fname_0);
    if cofp.is_null() {
        cofp = ofp;
    }
    if do_print != 0 {
        fprintf(cofp, b"/fname (%s) def\n\0" as *const u8 as *const libc::c_char, cp);
    }
    xfree(cp as *mut libc::c_void);
    cp = escape_string(fdir);
    if cofp.is_null() {
        cofp = ofp;
    }
    if do_print != 0 {
        fprintf(cofp, b"/fdir (%s) def\n\0" as *const u8 as *const libc::c_char, cp);
    }
    xfree(cp as *mut libc::c_void);
    xfree(dirc as *mut libc::c_void);
    cp = escape_string(ftail);
    if cofp.is_null() {
        cofp = ofp;
    }
    if do_print != 0 {
        fprintf(cofp, b"/ftail (%s) def\n\0" as *const u8 as *const libc::c_char, cp);
    }
    xfree(cp as *mut libc::c_void);
    xfree(basec as *mut libc::c_void);
    if user_fontp != 0 {
        if encoding as libc::c_uint == default_Fencoding as libc::c_uint {
            if cofp.is_null() {
                cofp = ofp;
            }
            if do_print != 0 {
                fprintf(
                    cofp,
                    b"/%s %g %g SUF\n\0" as *const u8 as *const libc::c_char,
                    Fname,
                    Fpt.w,
                    Fpt.h,
                );
            }
        } else {
            if cofp.is_null() {
                cofp = ofp;
            }
            if do_print != 0 {
                fprintf(
                    cofp,
                    b"/%s %g %g SUF_PS\n\0" as *const u8 as *const libc::c_char,
                    Fname,
                    Fpt.w,
                    Fpt.h,
                );
            }
        }
    }
    if count_key_value_set(user_strings) > 0 as libc::c_int {
        if cofp.is_null() {
            cofp = ofp;
        }
        if do_print != 0 {
            fprintf(
                cofp,
                b"%% User defined strings:\n\0" as *const u8 as *const libc::c_char,
            );
        }
        got = strhash_get_first(
            user_strings,
            &mut cp,
            &mut i,
            &mut cp2 as *mut *mut libc::c_char as *mut *mut libc::c_void,
        );
        while got != 0 {
            cp2 = format_user_string(
                b"%Format\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
                cp2,
            );
            if cofp.is_null() {
                cofp = ofp;
            }
            if do_print != 0 {
                fprintf(
                    cofp,
                    b"/%s (%s) def\n\0" as *const u8 as *const libc::c_char,
                    cp,
                    cp2,
                );
            }
            xfree(cp2 as *mut libc::c_void);
            got = strhash_get_next(
                user_strings,
                &mut cp,
                &mut i,
                &mut cp2 as *mut *mut libc::c_char as *mut *mut libc::c_void,
            );
        }
    }
    if !page_header.is_null() {
        let mut h_left: *mut libc::c_char = 0 as *mut libc::c_char;
        let mut h_center: *mut libc::c_char = 0 as *mut libc::c_char;
        let mut h_right: *mut libc::c_char = 0 as *mut libc::c_char;
        h_left = format_user_string(
            b"page header\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            page_header,
        );
        h_center = strchr(h_left, '|' as i32);
        if !h_center.is_null() {
            *h_center = '\0' as i32 as libc::c_char;
            h_center = h_center.offset(1);
            h_center;
            h_right = strchr(h_center, '|' as i32);
            if !h_right.is_null() {
                *h_right = '\0' as i32 as libc::c_char;
                h_right = h_right.offset(1);
                h_right;
            }
        }
        if cofp.is_null() {
            cofp = ofp;
        }
        if do_print != 0 {
            fprintf(
                cofp,
                b"/user_header_p true def\n\0" as *const u8 as *const libc::c_char,
            );
        }
        if cofp.is_null() {
            cofp = ofp;
        }
        if do_print != 0 {
            fprintf(
                cofp,
                b"/user_header_left_str (%s) def\n\0" as *const u8
                    as *const libc::c_char,
                h_left,
            );
        }
        if cofp.is_null() {
            cofp = ofp;
        }
        if do_print != 0 {
            fprintf(
                cofp,
                b"/user_header_center_str (%s) def\n\0" as *const u8
                    as *const libc::c_char,
                if !h_center.is_null() {
                    h_center as *const libc::c_char
                } else {
                    b"\0" as *const u8 as *const libc::c_char
                },
            );
        }
        if cofp.is_null() {
            cofp = ofp;
        }
        if do_print != 0 {
            fprintf(
                cofp,
                b"/user_header_right_str (%s) def\n\0" as *const u8
                    as *const libc::c_char,
                if !h_right.is_null() {
                    h_right as *const libc::c_char
                } else {
                    b"\0" as *const u8 as *const libc::c_char
                },
            );
        }
        xfree(h_left as *mut libc::c_void);
    } else {
        if cofp.is_null() {
            cofp = ofp;
        }
        if do_print != 0 {
            fprintf(
                cofp,
                b"/user_header_p false def\n\0" as *const u8 as *const libc::c_char,
            );
        }
    }
    if !page_footer.is_null() {
        let mut f_left: *mut libc::c_char = 0 as *mut libc::c_char;
        let mut f_center: *mut libc::c_char = 0 as *mut libc::c_char;
        let mut f_right: *mut libc::c_char = 0 as *mut libc::c_char;
        f_left = format_user_string(
            b"page footer\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            page_footer,
        );
        f_center = strchr(f_left, '|' as i32);
        if !f_center.is_null() {
            *f_center = '\0' as i32 as libc::c_char;
            f_center = f_center.offset(1);
            f_center;
            f_right = strchr(f_center, '|' as i32);
            if !f_right.is_null() {
                *f_right = '\0' as i32 as libc::c_char;
                f_right = f_right.offset(1);
                f_right;
            }
        }
        if cofp.is_null() {
            cofp = ofp;
        }
        if do_print != 0 {
            fprintf(
                cofp,
                b"/user_footer_p true def\n\0" as *const u8 as *const libc::c_char,
            );
        }
        if cofp.is_null() {
            cofp = ofp;
        }
        if do_print != 0 {
            fprintf(
                cofp,
                b"/user_footer_left_str (%s) def\n\0" as *const u8
                    as *const libc::c_char,
                f_left,
            );
        }
        if cofp.is_null() {
            cofp = ofp;
        }
        if do_print != 0 {
            fprintf(
                cofp,
                b"/user_footer_center_str (%s) def\n\0" as *const u8
                    as *const libc::c_char,
                if !f_center.is_null() {
                    f_center as *const libc::c_char
                } else {
                    b"\0" as *const u8 as *const libc::c_char
                },
            );
        }
        if cofp.is_null() {
            cofp = ofp;
        }
        if do_print != 0 {
            fprintf(
                cofp,
                b"/user_footer_right_str (%s) def\n\0" as *const u8
                    as *const libc::c_char,
                if !f_right.is_null() {
                    f_right as *const libc::c_char
                } else {
                    b"\0" as *const u8 as *const libc::c_char
                },
            );
        }
        xfree(f_left as *mut libc::c_void);
    } else {
        if cofp.is_null() {
            cofp = ofp;
        }
        if do_print != 0 {
            fprintf(
                cofp,
                b"/user_footer_p false def\n\0" as *const u8 as *const libc::c_char,
            );
        }
    }
    if cofp.is_null() {
        cofp = ofp;
    }
    if do_print != 0 {
        fprintf(cofp, b"%%%%EndPageSetup\n\0" as *const u8 as *const libc::c_char);
    }
    if empty == 0 {
        if highlight_bars != 0 {
            if cofp.is_null() {
                cofp = ofp;
            }
            if do_print != 0 {
                fprintf(
                    cofp,
                    b"%d %f %d %f highlight_bars\n\0" as *const u8
                        as *const libc::c_char,
                    highlight_bars,
                    Fpt.h + baselineskip,
                    d_output_y_margin,
                    highlight_bar_gray,
                );
            }
        }
        if !underlay.is_null() {
            if ul_position_p != 0 || ul_angle_p != 0 {
                if cofp.is_null() {
                    cofp = ofp;
                }
                if do_print != 0 {
                    fprintf(
                        cofp,
                        b"user_underlay\n\0" as *const u8 as *const libc::c_char,
                    );
                }
            } else {
                if cofp.is_null() {
                    cofp = ofp;
                }
                if do_print != 0 {
                    fprintf(cofp, b"underlay\n\0" as *const u8 as *const libc::c_char);
                }
            }
        }
        if num_columns > 1 as libc::c_int
            && (header as libc::c_uint == HDR_FANCY as libc::c_int as libc::c_uint
                || borders != 0)
        {
            if cofp.is_null() {
                cofp = ofp;
            }
            if do_print != 0 {
                fprintf(cofp, b"column_lines\n\0" as *const u8 as *const libc::c_char);
            }
        }
        if borders != 0 {
            if cofp.is_null() {
                cofp = ofp;
            }
            if do_print != 0 {
                fprintf(cofp, b"column_borders\n\0" as *const u8 as *const libc::c_char);
            }
        }
        match header as libc::c_uint {
            1 | 2 => {
                if cofp.is_null() {
                    cofp = ofp;
                }
                if do_print != 0 {
                    fprintf(cofp, b"do_header\n\0" as *const u8 as *const libc::c_char);
                }
            }
            0 | _ => {}
        }
    }
    if user_colorp != 0 {
        if cofp.is_null() {
            cofp = ofp;
        }
        if do_print != 0 {
            fprintf(
                cofp,
                b"%g %g %g setrgbcolor\n\0" as *const u8 as *const libc::c_char,
                user_color.r as libc::c_double,
                user_color.g as libc::c_double,
                user_color.b as libc::c_double,
            );
        }
    }
}
unsafe extern "C" fn dump_ps_page_trailer() {
    let mut nup_subpage: libc::c_uint = ((total_pages - 1 as libc::c_int)
        as libc::c_uint)
        .wrapping_rem(nup);
    if cofp.is_null() {
        cofp = ofp;
    }
    if do_print != 0 {
        fprintf(cofp, b"_R\n\0" as *const u8 as *const libc::c_char);
    }
    if nup > 1 as libc::c_int as libc::c_uint {
        if nup_subpage.wrapping_add(1 as libc::c_int as libc::c_uint) == nup {
            if cofp.is_null() {
                cofp = ofp;
            }
            if do_print != 0 {
                fprintf(cofp, b"_R\nS\n\0" as *const u8 as *const libc::c_char);
            }
        }
    } else {
        if cofp.is_null() {
            cofp = ofp;
        }
        if do_print != 0 {
            fprintf(cofp, b"S\n\0" as *const u8 as *const libc::c_char);
        }
    };
}
unsafe extern "C" fn dump_empty_page() {
    if nup > 1 as libc::c_int as libc::c_uint {
        let mut nup_subpage: libc::c_uint = ((total_pages - 1 as libc::c_int)
            as libc::c_uint)
            .wrapping_rem(nup);
        if nup_subpage == 0 as libc::c_int as libc::c_uint {
            dump_ps_page_header(
                b"\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
                1 as libc::c_int,
            );
            if cofp.is_null() {
                cofp = ofp;
            }
            if do_print != 0 {
                fprintf(cofp, b"_R\n\0" as *const u8 as *const libc::c_char);
            }
        } else {
            if cofp.is_null() {
                cofp = ofp;
            }
            if do_print != 0 {
                fprintf(
                    cofp,
                    b"%%Page: (-) %d\n\0" as *const u8 as *const libc::c_char,
                    total_pages,
                );
            }
        }
        if nup_subpage.wrapping_add(1 as libc::c_int as libc::c_uint) == nup {
            if cofp.is_null() {
                cofp = ofp;
            }
            if do_print != 0 {
                fprintf(cofp, b"_R\nS\n\0" as *const u8 as *const libc::c_char);
            }
        }
    } else {
        if cofp.is_null() {
            cofp = ofp;
        }
        if do_print != 0 {
            fprintf(
                cofp,
                b"%%%%Page: (-) %d\nS\n\0" as *const u8 as *const libc::c_char,
                total_pages,
            );
        }
    };
}
unsafe extern "C" fn recognize_eps_file(mut token: *mut Token) -> libc::c_int {
    let mut i: libc::c_int = 0;
    let mut buf: [libc::c_char; 4096] = [0; 4096];
    let mut filename: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut line: libc::c_int = 0;
    let mut valid_epsf: libc::c_int = 0;
    let mut llx: libc::c_float = 0.;
    let mut lly: libc::c_float = 0.;
    let mut urx: libc::c_float = 0.;
    let mut ury: libc::c_float = 0.;
    if quiet == 0 && verbose >= 2 as libc::c_int {
        fprintf(
            stderr,
            b"^@epsf=\"%s\"\n\0" as *const u8 as *const libc::c_char,
            ((*token).u.epsf.filename).as_mut_ptr(),
        );
    }
    i = strlen(((*token).u.epsf.filename).as_mut_ptr()) as libc::c_int;
    filename = tilde_subst(((*token).u.epsf.filename).as_mut_ptr());
    (*token).u.epsf.fp = fopen(filename, b"rb\0" as *const u8 as *const libc::c_char);
    xfree(filename as *mut libc::c_void);
    if ((*token).u.epsf.fp).is_null() {
        if (*token).u.epsf.filename[0 as libc::c_int as usize] as libc::c_int
            != '/' as i32
        {
            let mut ctx: FileLookupCtx = FileLookupCtx {
                name: 0 as *mut libc::c_char,
                suffix: 0 as *mut libc::c_char,
                fullname: 0 as *mut Buffer,
            };
            ctx.name = ((*token).u.epsf.filename).as_mut_ptr();
            ctx.suffix = b"\0" as *const u8 as *const libc::c_char as *mut libc::c_char;
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
            ) != 0
            {
                (*token)
                    .u
                    .epsf
                    .fp = fopen(
                    buffer_ptr(ctx.fullname),
                    b"rb\0" as *const u8 as *const libc::c_char,
                );
            }
            buffer_free(ctx.fullname);
        }
        if ((*token).u.epsf.fp).is_null() {
            if quiet == 0 && verbose >= 0 as libc::c_int {
                fprintf(
                    stderr,
                    dcgettext(
                        0 as *const libc::c_char,
                        b"couldn't open EPS file \"%s\": %s\n\0" as *const u8
                            as *const libc::c_char,
                        5 as libc::c_int,
                    ),
                    ((*token).u.epsf.filename).as_mut_ptr(),
                    strerror(*__errno_location()),
                );
            }
            return 0 as libc::c_int;
        }
    }
    line = 0 as libc::c_int;
    valid_epsf = 0 as libc::c_int;
    (*token).u.epsf.skipbuf = 0 as *mut libc::c_char;
    (*token).u.epsf.skipbuf_len = 0 as libc::c_int as libc::c_uint;
    (*token).u.epsf.skipbuf_pos = 0 as libc::c_int as libc::c_uint;
    while !(fgets(
        buf.as_mut_ptr(),
        ::std::mem::size_of::<[libc::c_char; 4096]>() as libc::c_ulong as libc::c_int,
        (*token).u.epsf.fp,
    ))
        .is_null()
    {
        line += 1;
        line;
        i = strlen(buf.as_mut_ptr()) as libc::c_int;
        if (i as libc::c_uint).wrapping_add((*token).u.epsf.skipbuf_pos)
            >= (*token).u.epsf.skipbuf_len
        {
            (*token)
                .u
                .epsf
                .skipbuf_len = ((*token).u.epsf.skipbuf_len)
                .wrapping_add(8192 as libc::c_int as libc::c_uint);
            (*token)
                .u
                .epsf
                .skipbuf = xrealloc(
                (*token).u.epsf.skipbuf as *mut libc::c_void,
                (*token).u.epsf.skipbuf_len as size_t,
            ) as *mut libc::c_char;
        }
        memcpy(
            ((*token).u.epsf.skipbuf).offset((*token).u.epsf.skipbuf_pos as isize)
                as *mut libc::c_void,
            buf.as_mut_ptr() as *const libc::c_void,
            i as libc::c_ulong,
        );
        (*token)
            .u
            .epsf
            .skipbuf_pos = ((*token).u.epsf.skipbuf_pos).wrapping_add(i as libc::c_uint);
        if line == 1 as libc::c_int {
            if buf[0 as libc::c_int as usize] as libc::c_int != '%' as i32
                || buf[1 as libc::c_int as usize] as libc::c_int != '!' as i32
            {
                if quiet == 0 && verbose >= 0 as libc::c_int {
                    fprintf(
                        stderr,
                        dcgettext(
                            0 as *const libc::c_char,
                            b"EPS file \"%s\" does not start with \"%%!\" magic\n\0"
                                as *const u8 as *const libc::c_char,
                            5 as libc::c_int,
                        ),
                        ((*token).u.epsf.filename).as_mut_ptr(),
                    );
                }
                break;
            }
        }
        if !(strncmp(
            buf.as_mut_ptr(),
            b"%%BoundingBox:\0" as *const u8 as *const libc::c_char,
            strlen(b"%%BoundingBox:\0" as *const u8 as *const libc::c_char),
        ) == 0 as libc::c_int)
        {
            continue;
        }
        i = sscanf(
            buf
                .as_mut_ptr()
                .offset(
                    strlen(b"%%BoundingBox:\0" as *const u8 as *const libc::c_char)
                        as isize,
                ),
            b"%f %f %f %f\0" as *const u8 as *const libc::c_char,
            &mut llx as *mut libc::c_float,
            &mut lly as *mut libc::c_float,
            &mut urx as *mut libc::c_float,
            &mut ury as *mut libc::c_float,
        );
        if i != 4 as libc::c_int {
            i = strlen(b"%%BoundingBox:\0" as *const u8 as *const libc::c_char)
                as libc::c_int;
            while buf[i as usize] as libc::c_int != 0
                && (buf[i as usize] as libc::c_int == ' ' as i32
                    || buf[i as usize] as libc::c_int == '\t' as i32)
            {
                i += 1;
                i;
            }
            if !(strncmp(
                buf.as_mut_ptr().offset(i as isize),
                b"(atend)\0" as *const u8 as *const libc::c_char,
                strlen(b"(atend)\0" as *const u8 as *const libc::c_char),
            ) != 0 as libc::c_int)
            {
                continue;
            }
            if quiet == 0 && verbose >= 0 as libc::c_int {
                fprintf(
                    stderr,
                    dcgettext(
                        0 as *const libc::c_char,
                        b"EPS file \"%s\" contains malformed %%%%BoundingBox row:\n\"%.*s\"\n\0"
                            as *const u8 as *const libc::c_char,
                        5 as libc::c_int,
                    ),
                    ((*token).u.epsf.filename).as_mut_ptr(),
                    (strlen(buf.as_mut_ptr()))
                        .wrapping_sub(1 as libc::c_int as libc::c_ulong),
                    buf.as_mut_ptr(),
                );
            }
            break;
        } else {
            (*token).u.epsf.llx = llx as libc::c_int;
            (*token).u.epsf.lly = lly as libc::c_int;
            (*token).u.epsf.urx = urx as libc::c_int;
            (*token).u.epsf.ury = ury as libc::c_int;
            valid_epsf = 1 as libc::c_int;
            break;
        }
    }
    if valid_epsf == 0 {
        if quiet == 0 && verbose >= 0 as libc::c_int {
            fprintf(
                stderr,
                dcgettext(
                    0 as *const libc::c_char,
                    b"EPS file \"%s\" is not a valid EPS file\n\0" as *const u8
                        as *const libc::c_char,
                    5 as libc::c_int,
                ),
                ((*token).u.epsf.filename).as_mut_ptr(),
            );
        }
        if (*token).u.epsf.pipe != 0 {
            pclose((*token).u.epsf.fp);
        } else {
            fclose((*token).u.epsf.fp);
        }
        xfree((*token).u.epsf.skipbuf as *mut libc::c_void);
        return 0 as libc::c_int;
    }
    if quiet == 0 && verbose >= 2 as libc::c_int {
        fprintf(
            stderr,
            b"BoundingBox: %d %d %d %d\n\0" as *const u8 as *const libc::c_char,
            (*token).u.epsf.llx,
            (*token).u.epsf.lly,
            (*token).u.epsf.urx,
            (*token).u.epsf.ury,
        );
    }
    return 1 as libc::c_int;
}
unsafe extern "C" fn paste_epsf(mut token: *mut Token) {
    let mut buf: [libc::c_char; 4096] = [0; 4096];
    let mut i: libc::c_int = 0;
    if cofp.is_null() {
        cofp = ofp;
    }
    if do_print != 0 {
        fprintf(cofp, b"BeginEPSF\n\0" as *const u8 as *const libc::c_char);
    }
    if cofp.is_null() {
        cofp = ofp;
    }
    if do_print != 0 {
        fprintf(
            cofp,
            b"%g %g translate\n\0" as *const u8 as *const libc::c_char,
            (*token).new_x,
            (*token).new_y,
        );
    }
    if cofp.is_null() {
        cofp = ofp;
    }
    if do_print != 0 {
        fprintf(
            cofp,
            b"%g %g scale\n\0" as *const u8 as *const libc::c_char,
            (*token).u.epsf.xscale,
            (*token).u.epsf.yscale,
        );
    }
    if cofp.is_null() {
        cofp = ofp;
    }
    if do_print != 0 {
        fprintf(
            cofp,
            b"%d %d translate\n\0" as *const u8 as *const libc::c_char,
            -(*token).u.epsf.llx,
            -(*token).u.epsf.lly,
        );
    }
    if cofp.is_null() {
        cofp = ofp;
    }
    if do_print != 0 {
        fprintf(
            cofp,
            b"%d %d %d %d Box clip newpath\n\0" as *const u8 as *const libc::c_char,
            (*token).u.epsf.llx - 1 as libc::c_int,
            (*token).u.epsf.lly - 1 as libc::c_int,
            (*token).u.epsf.urx - (*token).u.epsf.llx + 2 as libc::c_int,
            (*token).u.epsf.ury - (*token).u.epsf.lly + 2 as libc::c_int,
        );
    }
    if cofp.is_null() {
        cofp = ofp;
    }
    if do_print != 0 {
        fprintf(
            cofp,
            b"%%%%BeginDocument: %s%s\n\0" as *const u8 as *const libc::c_char,
            ((*token).u.epsf.filename).as_mut_ptr(),
            if (*token).u.epsf.pipe != 0 {
                b"|\0" as *const u8 as *const libc::c_char
            } else {
                b"\0" as *const u8 as *const libc::c_char
            },
        );
    }
    if do_print != 0 {
        fwrite(
            (*token).u.epsf.skipbuf as *const libc::c_void,
            1 as libc::c_int as libc::c_ulong,
            (*token).u.epsf.skipbuf_pos as libc::c_ulong,
            cofp,
        );
        loop {
            i = fread(
                buf.as_mut_ptr() as *mut libc::c_void,
                1 as libc::c_int as libc::c_ulong,
                ::std::mem::size_of::<[libc::c_char; 4096]>() as libc::c_ulong,
                (*token).u.epsf.fp,
            ) as libc::c_int;
            if !(i != 0 as libc::c_int) {
                break;
            }
            fwrite(
                buf.as_mut_ptr() as *const libc::c_void,
                1 as libc::c_int as libc::c_ulong,
                i as libc::c_ulong,
                cofp,
            );
        }
    }
    if cofp.is_null() {
        cofp = ofp;
    }
    if do_print != 0 {
        fprintf(cofp, b"\n\0" as *const u8 as *const libc::c_char);
    }
    if cofp.is_null() {
        cofp = ofp;
    }
    if do_print != 0 {
        fprintf(
            cofp,
            b"%%%%EndDocument\nEndEPSF\n\0" as *const u8 as *const libc::c_char,
        );
    }
    if (*token).u.epsf.pipe != 0 {
        pclose((*token).u.epsf.fp);
    } else {
        fclose((*token).u.epsf.fp);
    }
    xfree((*token).u.epsf.skipbuf as *mut libc::c_void);
}
unsafe extern "C" fn read_float(
    mut is: *mut InputStream,
    mut units: libc::c_int,
    mut horizontal: libc::c_int,
) -> libc::c_double {
    let mut buf: [libc::c_char; 256] = [0; 256];
    let mut i: libc::c_int = 0;
    let mut ch: libc::c_int = 0;
    let mut val: libc::c_double = 0.;
    i = 0 as libc::c_int;
    while (i as libc::c_ulong)
        < (::std::mem::size_of::<[libc::c_char; 256]>() as libc::c_ulong)
            .wrapping_sub(1 as libc::c_int as libc::c_ulong)
        && {
            ch = is_getc(is);
            ch != -(1 as libc::c_int)
        }
        && ('0' as i32 <= ch && ch <= '9' as i32 || ch == '.' as i32 || ch == '-' as i32
            || ch == '+' as i32)
    {
        buf[i as usize] = ch as libc::c_char;
        i += 1;
        i;
    }
    buf[i as usize] = '\0' as i32 as libc::c_char;
    if ch != -(1 as libc::c_int) {
        is_ungetc(ch, is);
    }
    val = atof(buf.as_mut_ptr());
    if units != 0 {
        ch = is_getc(is);
        let mut current_block_13: u64;
        match ch {
            99 => {
                val *= 72 as libc::c_int as libc::c_double / 2.54f64;
                current_block_13 = 8831408221741692167;
            }
            112 => {
                current_block_13 = 8831408221741692167;
            }
            105 => {
                val *= 72 as libc::c_int as libc::c_double;
                current_block_13 = 8831408221741692167;
            }
            108 => {
                current_block_13 = 464947610555806071;
            }
            _ => {
                is_ungetc(ch, is);
                current_block_13 = 464947610555806071;
            }
        }
        match current_block_13 {
            464947610555806071 => {
                if horizontal != 0 {
                    val
                        *= *font_widths
                            .as_mut_ptr()
                            .offset('m' as i32 as libc::c_uchar as isize);
                } else {
                    val *= Fpt.h + baselineskip;
                }
            }
            _ => {}
        }
    }
    return val;
}
static mut pass_through_magics: [C2RustUnnamed_4; 5] = [
    {
        let mut init = C2RustUnnamed_4 {
            magic: b"%!\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            magiclen: 2 as libc::c_int as libc::c_uint,
            name: b"PostScript\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
            revert_delta: -(2 as libc::c_int),
        };
        init
    },
    {
        let mut init = C2RustUnnamed_4 {
            magic: b"\x04%!\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            magiclen: 3 as libc::c_int as libc::c_uint,
            name: b"PostScript\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
            revert_delta: -(2 as libc::c_int),
        };
        init
    },
    {
        let mut init = C2RustUnnamed_4 {
            magic: b"\x1BE\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            magiclen: 2 as libc::c_int as libc::c_uint,
            name: b"PCL\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            revert_delta: -(2 as libc::c_int),
        };
        init
    },
    {
        let mut init = C2RustUnnamed_4 {
            magic: b"\x1B%\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            magiclen: 2 as libc::c_int as libc::c_uint,
            name: b"PCL\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            revert_delta: -(2 as libc::c_int),
        };
        init
    },
    {
        let mut init = C2RustUnnamed_4 {
            magic: 0 as *const libc::c_char as *mut libc::c_char,
            magiclen: 0 as libc::c_int as libc::c_uint,
            name: 0 as *const libc::c_char as *mut libc::c_char,
            revert_delta: 0 as libc::c_int,
        };
        init
    },
];
unsafe extern "C" fn do_pass_through(
    mut fname_0: *mut libc::c_char,
    mut is: *mut InputStream,
) -> libc::c_int {
    let mut ch: libc::c_int = 0;
    let mut saved_pos: libc::c_ulong = (*is).bufpos as libc::c_ulong;
    let mut i: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    if output_language_pass_through != 0 {
        if quiet == 0 && verbose >= 1 as libc::c_int {
            fprintf(
                stderr,
                dcgettext(
                    0 as *const libc::c_char,
                    b"passing through all input files for output language `%s'\n\0"
                        as *const u8 as *const libc::c_char,
                    5 as libc::c_int,
                ),
                output_language,
            );
        }
    } else {
        i = 0 as libc::c_int;
        while !(pass_through_magics[i as usize].magic).is_null() {
            j = 0 as libc::c_int;
            while (j as libc::c_uint) < pass_through_magics[i as usize].magiclen {
                ch = is_getc(is);
                if ch == -(1 as libc::c_int)
                    || ch
                        != *(pass_through_magics[i as usize].magic).offset(j as isize)
                            as libc::c_uchar as libc::c_int
                {
                    break;
                }
                j += 1;
                j;
            }
            if j as libc::c_uint >= pass_through_magics[i as usize].magiclen {
                break;
            }
            (*is).bufpos = saved_pos as libc::c_uint;
            i += 1;
            i;
        }
        if (pass_through_magics[i as usize].magic).is_null() {
            return 0 as libc::c_int;
        }
        (*is)
            .bufpos = ((*is).bufpos)
            .wrapping_add(pass_through_magics[i as usize].revert_delta as libc::c_uint);
        if ps_header_dumped != 0 {
            if cofp.is_null() {
                cofp = ofp;
            }
            if do_print != 0 {
                fprintf(
                    cofp,
                    b"%%%%Page: (%s) -1\n_S\n%%%%BeginDocument: %s\n\0" as *const u8
                        as *const libc::c_char,
                    fname_0,
                    fname_0,
                );
            }
        }
        if quiet == 0 && verbose >= 1 as libc::c_int {
            fprintf(
                stderr,
                dcgettext(
                    0 as *const libc::c_char,
                    b"passing through %s file \"%s\"\n\0" as *const u8
                        as *const libc::c_char,
                    5 as libc::c_int,
                ),
                pass_through_magics[i as usize].name,
                fname_0,
            );
        }
    }
    loop {
        fwrite(
            ((*is).buf).as_mut_ptr().offset((*is).bufpos as isize)
                as *const libc::c_void,
            1 as libc::c_int as libc::c_ulong,
            ((*is).data_in_buf).wrapping_sub((*is).bufpos) as libc::c_ulong,
            ofp,
        );
        (*is).bufpos = (*is).data_in_buf;
        ch = is_getc(is);
        (*is).bufpos = 0 as libc::c_int as libc::c_uint;
        if !(ch != -(1 as libc::c_int)) {
            break;
        }
    }
    if output_language_pass_through == 0 {
        if ps_header_dumped != 0 {
            if cofp.is_null() {
                cofp = ofp;
            }
            if do_print != 0 {
                fprintf(
                    cofp,
                    b"%%%%EndDocument\n_R\n\0" as *const u8 as *const libc::c_char,
                );
            }
        }
    }
    return 1 as libc::c_int;
}
unsafe extern "C" fn print_line_number(
    mut x: libc::c_double,
    mut y: libc::c_double,
    mut space: libc::c_double,
    mut margin: libc::c_double,
    mut linenum: libc::c_uint,
) {
    let mut len: libc::c_double = 0.0f64;
    let mut buf: [libc::c_char; 20] = [0; 20];
    let mut i: libc::c_int = 0;
    let mut saved_Fname: *mut libc::c_char = b"\0" as *const u8 as *const libc::c_char
        as *mut libc::c_char;
    let mut saved_Fpt: FontPoint = FontPoint { w: 0., h: 0. };
    let mut saved_Fencoding: InputEncoding = ENC_ISO_8859_1;
    saved_Fpt.w = 0.0f64;
    saved_Fpt.h = 0.0f64;
    if linenum == print_line_number_last {
        return;
    }
    print_line_number_last = linenum;
    if user_fontp != 0 {
        saved_Fname = Fname;
        saved_Fpt.w = Fpt.w;
        saved_Fpt.h = Fpt.h;
        saved_Fencoding = encoding;
        Fname = default_Fname;
        Fpt.w = default_Fpt.w;
        Fpt.h = default_Fpt.h;
        encoding = default_Fencoding;
        if cofp.is_null() {
            cofp = ofp;
        }
        if do_print != 0 {
            fprintf(
                cofp,
                b"/F-gs-font %g %g SF\n\0" as *const u8 as *const libc::c_char,
                Fpt.w,
                Fpt.h,
            );
        }
        read_font_info();
    }
    sprintf(buf.as_mut_ptr(), b"%d\0" as *const u8 as *const libc::c_char, linenum);
    i = 0 as libc::c_int;
    while buf[i as usize] != 0 {
        len
            += *font_widths
                .as_mut_ptr()
                .offset(buf[i as usize] as libc::c_uchar as isize);
        i += 1;
        i;
    }
    if cofp.is_null() {
        cofp = ofp;
    }
    if do_print != 0 {
        fprintf(
            cofp,
            b"%g %g M (%s:) s\n\0" as *const u8 as *const libc::c_char,
            x + space - len,
            y,
            buf.as_mut_ptr(),
        );
    }
    if user_fontp != 0 {
        Fname = saved_Fname;
        Fpt.w = saved_Fpt.w;
        Fpt.h = saved_Fpt.h;
        encoding = saved_Fencoding;
        if cofp.is_null() {
            cofp = ofp;
        }
        if do_print != 0 {
            fprintf(
                cofp,
                b"/%s %g %g SUF\n\0" as *const u8 as *const libc::c_char,
                Fname,
                Fpt.w,
                Fpt.h,
            );
        }
        read_font_info();
    }
}
unsafe extern "C" fn divert() {
    if divertfp.is_null() {} else {
        __assert_fail(
            b"divertfp == NULL\0" as *const u8 as *const libc::c_char,
            b"psgen.c\0" as *const u8 as *const libc::c_char,
            2789 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 14],
                &[libc::c_char; 14],
            >(b"void divert()\0"))
                .as_ptr(),
        );
    }
    'c_19919: {
        if divertfp.is_null() {} else {
            __assert_fail(
                b"divertfp == NULL\0" as *const u8 as *const libc::c_char,
                b"psgen.c\0" as *const u8 as *const libc::c_char,
                2789 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 14],
                    &[libc::c_char; 14],
                >(b"void divert()\0"))
                    .as_ptr(),
            );
        }
    };
    divertfp = tmpfile();
    if divertfp.is_null() {
        fprintf(stderr, b"%s: \0" as *const u8 as *const libc::c_char, program);
        fprintf(
            stderr,
            dcgettext(
                0 as *const libc::c_char,
                b"couldn't create temporary divert file: %s\0" as *const u8
                    as *const libc::c_char,
                5 as libc::c_int,
            ),
            strerror(*__errno_location()),
        );
        fprintf(stderr, b"\n\0" as *const u8 as *const libc::c_char);
        fflush(stderr);
        exit(1 as libc::c_int);
    }
    cofp = divertfp;
}
unsafe extern "C" fn undivert() {
    let mut buf: [libc::c_char; 1024] = [0; 1024];
    let mut doc_level: libc::c_int = 0 as libc::c_int;
    let mut cp: *mut libc::c_char = 0 as *mut libc::c_char;
    if !divertfp.is_null() {} else {
        __assert_fail(
            b"divertfp != NULL\0" as *const u8 as *const libc::c_char,
            b"psgen.c\0" as *const u8 as *const libc::c_char,
            2809 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 16],
                &[libc::c_char; 16],
            >(b"void undivert()\0"))
                .as_ptr(),
        );
    }
    'c_9121: {
        if !divertfp.is_null() {} else {
            __assert_fail(
                b"divertfp != NULL\0" as *const u8 as *const libc::c_char,
                b"psgen.c\0" as *const u8 as *const libc::c_char,
                2809 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 16],
                    &[libc::c_char; 16],
                >(b"void undivert()\0"))
                    .as_ptr(),
            );
        }
    };
    if fseek(divertfp, 0 as libc::c_int as libc::c_long, 0 as libc::c_int)
        != 0 as libc::c_int
    {
        fprintf(stderr, b"%s: \0" as *const u8 as *const libc::c_char, program);
        fprintf(
            stderr,
            dcgettext(
                0 as *const libc::c_char,
                b"couldn't rewind divert file: %s\0" as *const u8 as *const libc::c_char,
                5 as libc::c_int,
            ),
            strerror(*__errno_location()),
        );
        fprintf(stderr, b"\n\0" as *const u8 as *const libc::c_char);
        fflush(stderr);
        exit(1 as libc::c_int);
    }
    while !(fgets(
        buf.as_mut_ptr(),
        ::std::mem::size_of::<[libc::c_char; 1024]>() as libc::c_ulong as libc::c_int,
        divertfp,
    ))
        .is_null()
    {
        if strncmp(
            buf.as_mut_ptr(),
            b"%%BeginDocument\0" as *const u8 as *const libc::c_char,
            15 as libc::c_int as libc::c_ulong,
        ) == 0 as libc::c_int
        {
            doc_level += 1;
            doc_level;
        } else if strncmp(
            buf.as_mut_ptr(),
            b"%%EndDocument\0" as *const u8 as *const libc::c_char,
            13 as libc::c_int as libc::c_ulong,
        ) == 0 as libc::c_int
        {
            doc_level -= 1;
            doc_level;
        }
        if doc_level == 0 as libc::c_int {
            if strncmp(
                buf.as_mut_ptr(),
                b"% User defined strings\0" as *const u8 as *const libc::c_char,
                22 as libc::c_int as libc::c_ulong,
            ) == 0 as libc::c_int
            {
                fputs(buf.as_mut_ptr(), ofp);
                while !(fgets(
                    buf.as_mut_ptr(),
                    ::std::mem::size_of::<[libc::c_char; 1024]>() as libc::c_ulong
                        as libc::c_int,
                    divertfp,
                ))
                    .is_null()
                {
                    if strncmp(
                        buf.as_mut_ptr(),
                        b"%%EndPageSetup\0" as *const u8 as *const libc::c_char,
                        14 as libc::c_int as libc::c_ulong,
                    ) == 0 as libc::c_int
                    {
                        break;
                    }
                    cp = strchr(buf.as_mut_ptr(), '\u{1}' as i32);
                    if !cp.is_null() {
                        *cp = '\0' as i32 as libc::c_char;
                        fputs(buf.as_mut_ptr(), ofp);
                        fprintf(
                            ofp,
                            b"%d\0" as *const u8 as *const libc::c_char,
                            total_pages_in_file,
                        );
                        fputs(cp.offset(1 as libc::c_int as isize), ofp);
                    } else {
                        fputs(buf.as_mut_ptr(), ofp);
                    }
                }
            }
        }
        fputs(buf.as_mut_ptr(), ofp);
    }
    fclose(divertfp);
    divertfp = 0 as *mut FILE;
    cofp = ofp;
}
unsafe extern "C" fn handle_two_side_options() {
    if rotate_even_pages != 0 {
        if cofp.is_null() {
            cofp = ofp;
        }
        if do_print != 0 {
            fprintf(
                cofp,
                b"180 rotate\n%d %d translate\n\0" as *const u8 as *const libc::c_char,
                -(*media).w,
                -(*media).h,
            );
        }
    }
    if swap_even_page_margins != 0 {
        if cofp.is_null() {
            cofp = ofp;
        }
        if do_print != 0 {
            fprintf(
                cofp,
                b"%d 0 translate\n\0" as *const u8 as *const libc::c_char,
                -((*media).llx - ((*media).w - (*media).urx)),
            );
        }
    }
}
