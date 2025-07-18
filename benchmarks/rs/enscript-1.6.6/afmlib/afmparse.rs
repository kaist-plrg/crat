use ::libc;
extern "C" {
    pub type _IO_wide_data;
    pub type _IO_codecvt;
    pub type _IO_marker;
    pub type stringhash_st;
    fn strtod(_: *const libc::c_char, _: *mut *mut libc::c_char) -> libc::c_double;
    fn strtol(
        _: *const libc::c_char,
        _: *mut *mut libc::c_char,
        _: libc::c_int,
    ) -> libc::c_long;
    fn calloc(_: libc::c_ulong, _: libc::c_ulong) -> *mut libc::c_void;
    fn abort() -> !;
    fn getc(__stream: *mut FILE) -> libc::c_int;
    fn ungetc(__c: libc::c_int, __stream: *mut FILE) -> libc::c_int;
    fn sprintf(_: *mut libc::c_char, _: *const libc::c_char, _: ...) -> libc::c_int;
    fn printf(_: *const libc::c_char, _: ...) -> libc::c_int;
    fn fprintf(_: *mut FILE, _: *const libc::c_char, _: ...) -> libc::c_int;
    fn fopen(_: *const libc::c_char, _: *const libc::c_char) -> *mut FILE;
    fn fclose(__stream: *mut FILE) -> libc::c_int;
    static mut stderr: *mut FILE;
    fn memcpy(
        _: *mut libc::c_void,
        _: *const libc::c_void,
        _: libc::c_ulong,
    ) -> *mut libc::c_void;
    fn strcmp(_: *const libc::c_char, _: *const libc::c_char) -> libc::c_int;
    fn afm_error(handle: AFMHandle, message: *mut libc::c_char);
    fn longjmp(_: *mut __jmp_buf_tag, _: libc::c_int) -> !;
    fn __assert_fail(
        __assertion: *const libc::c_char,
        __file: *const libc::c_char,
        __line: libc::c_uint,
        __function: *const libc::c_char,
    ) -> !;
    fn __errno_location() -> *mut libc::c_int;
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
    fn strlen(_: *const libc::c_char) -> libc::c_ulong;
}
pub type size_t = libc::c_ulong;
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct __sigset_t {
    pub __val: [libc::c_ulong; 16],
}
pub type __jmp_buf = [libc::c_long; 8];
#[derive(Copy, Clone)]
#[repr(C)]
pub struct __jmp_buf_tag {
    pub __jmpbuf: __jmp_buf,
    pub __mask_was_saved: libc::c_int,
    pub __saved_mask: __sigset_t,
}
pub type jmp_buf = [__jmp_buf_tag; 1];
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
    pub u: C2RustUnnamed,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed {
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct afm_handle_st {
    pub verbose: libc::c_uint,
    pub font_map: StringHashPtr,
    pub jmpbuf: jmp_buf,
    pub parse_error: AFMError,
}
pub type StringHashPtr = *mut stringhash_st;
pub type AFMHandle = *mut afm_handle_st;
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct afm_font_private_data_st {
    pub undef: *mut AFMIndividualCharacterMetrics,
    pub fontnames: StringHashPtr,
    pub compositenames: StringHashPtr,
}
pub type AFMFont = *mut afm_font_st;
pub type AFMKey = libc::c_uint;
pub const kEndMaster: AFMKey = 80;
pub const kStartMaster: AFMKey = 79;
pub const kAxisLabel: AFMKey = 78;
pub const kAxisType: AFMKey = 77;
pub const kEndAxis: AFMKey = 76;
pub const kStartAxis: AFMKey = 75;
pub const kPCC: AFMKey = 74;
pub const kCC: AFMKey = 73;
pub const kEndComposites: AFMKey = 72;
pub const kStartComposites: AFMKey = 71;
pub const kKPY: AFMKey = 70;
pub const kKPX: AFMKey = 69;
pub const kKPH: AFMKey = 68;
pub const kKP: AFMKey = 67;
pub const kEndKernPairs: AFMKey = 66;
pub const kStartKernPairs: AFMKey = 65;
pub const kTrackKern: AFMKey = 64;
pub const kEndTrackKern: AFMKey = 63;
pub const kStartTrackKern: AFMKey = 62;
pub const kEndKernData: AFMKey = 61;
pub const kStartKernData: AFMKey = 60;
pub const kL: AFMKey = 59;
pub const kB: AFMKey = 58;
pub const kN: AFMKey = 57;
pub const kVV: AFMKey = 56;
pub const kW1: AFMKey = 55;
pub const kW0: AFMKey = 54;
pub const kW: AFMKey = 53;
pub const kW1Y: AFMKey = 52;
pub const kW0Y: AFMKey = 51;
pub const kWY: AFMKey = 50;
pub const kW1X: AFMKey = 49;
pub const kW0X: AFMKey = 48;
pub const kWX: AFMKey = 47;
pub const kCH: AFMKey = 46;
pub const kC: AFMKey = 45;
pub const kEndCharMetrics: AFMKey = 44;
pub const kStartCharMetrics: AFMKey = 43;
pub const kIsFixedPitch: AFMKey = 42;
pub const kCharWidth: AFMKey = 41;
pub const kItalicAngle: AFMKey = 40;
pub const kUnderlineThickness: AFMKey = 39;
pub const kUnderlinePosition: AFMKey = 38;
pub const kEndDirection: AFMKey = 37;
pub const kStartDirection: AFMKey = 36;
pub const kBlendAxisTypes: AFMKey = 35;
pub const kBlendDesignMap: AFMKey = 34;
pub const kBlendDesignPositions: AFMKey = 33;
pub const kWeightVector: AFMKey = 32;
pub const kDescender: AFMKey = 31;
pub const kAscender: AFMKey = 30;
pub const kXHeight: AFMKey = 29;
pub const kCapHeight: AFMKey = 28;
pub const kIsFixedV: AFMKey = 27;
pub const kVVector: AFMKey = 26;
pub const kIsBaseFont: AFMKey = 25;
pub const kCharacters: AFMKey = 24;
pub const kCharacterSet: AFMKey = 23;
pub const kEscChar: AFMKey = 22;
pub const kMappingScheme: AFMKey = 21;
pub const kEncodingScheme: AFMKey = 20;
pub const kNotice: AFMKey = 19;
pub const kVersion: AFMKey = 18;
pub const kFontBBox: AFMKey = 17;
pub const kWeight: AFMKey = 16;
pub const kFamilyName: AFMKey = 15;
pub const kFullName: AFMKey = 14;
pub const kFontName: AFMKey = 13;
pub const kAxes: AFMKey = 12;
pub const kMasters: AFMKey = 11;
pub const kDescendents: AFMKey = 10;
pub const kMetricsSets: AFMKey = 9;
pub const kEndMasterFontMetrics: AFMKey = 8;
pub const kStartMasterFontMetrics: AFMKey = 7;
pub const kEndDescendent: AFMKey = 6;
pub const kStartDescendent: AFMKey = 5;
pub const kEndCompFontMetrics: AFMKey = 4;
pub const kStartCompFontMetrics: AFMKey = 3;
pub const kEndFontMetrics: AFMKey = 2;
pub const kStartFontMetrics: AFMKey = 1;
pub const kComment: AFMKey = 0;
pub type ParseCtx = parse_ctx_st;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct parse_ctx_st {
    pub fp: *mut FILE,
    pub token: [libc::c_char; 1024],
    pub tokenlen: libc::c_uint,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct keyname_st {
    pub name: *mut libc::c_char,
    pub key: AFMKey,
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
unsafe extern "C" fn atof(mut __nptr: *const libc::c_char) -> libc::c_double {
    return strtod(__nptr, 0 as *mut libc::c_void as *mut *mut libc::c_char);
}
static mut keynames: [keyname_st; 82] = [
    {
        let mut init = keyname_st {
            name: b"Ascender\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            key: kAscender,
        };
        init
    },
    {
        let mut init = keyname_st {
            name: b"Axes\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            key: kAxes,
        };
        init
    },
    {
        let mut init = keyname_st {
            name: b"AxisLabel\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
            key: kAxisLabel,
        };
        init
    },
    {
        let mut init = keyname_st {
            name: b"AxisType\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            key: kAxisType,
        };
        init
    },
    {
        let mut init = keyname_st {
            name: b"B\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            key: kB,
        };
        init
    },
    {
        let mut init = keyname_st {
            name: b"BlendAxisTypes\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
            key: kBlendAxisTypes,
        };
        init
    },
    {
        let mut init = keyname_st {
            name: b"BlendDesignMap\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
            key: kBlendDesignMap,
        };
        init
    },
    {
        let mut init = keyname_st {
            name: b"BlendDesignPositions\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
            key: kBlendDesignPositions,
        };
        init
    },
    {
        let mut init = keyname_st {
            name: b"C\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            key: kC,
        };
        init
    },
    {
        let mut init = keyname_st {
            name: b"CC\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            key: kCC,
        };
        init
    },
    {
        let mut init = keyname_st {
            name: b"CH\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            key: kCH,
        };
        init
    },
    {
        let mut init = keyname_st {
            name: b"CapHeight\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
            key: kCapHeight,
        };
        init
    },
    {
        let mut init = keyname_st {
            name: b"CharWidth\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
            key: kCharWidth,
        };
        init
    },
    {
        let mut init = keyname_st {
            name: b"CharacterSet\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
            key: kCharacterSet,
        };
        init
    },
    {
        let mut init = keyname_st {
            name: b"Characters\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
            key: kCharacters,
        };
        init
    },
    {
        let mut init = keyname_st {
            name: b"Comment\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            key: kComment,
        };
        init
    },
    {
        let mut init = keyname_st {
            name: b"Descendents\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
            key: kDescendents,
        };
        init
    },
    {
        let mut init = keyname_st {
            name: b"Descender\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
            key: kDescender,
        };
        init
    },
    {
        let mut init = keyname_st {
            name: b"EncodingScheme\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
            key: kEncodingScheme,
        };
        init
    },
    {
        let mut init = keyname_st {
            name: b"EndAxis\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            key: kEndAxis,
        };
        init
    },
    {
        let mut init = keyname_st {
            name: b"EndCharMetrics\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
            key: kEndCharMetrics,
        };
        init
    },
    {
        let mut init = keyname_st {
            name: b"EndCompFontMetrics\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
            key: kEndCompFontMetrics,
        };
        init
    },
    {
        let mut init = keyname_st {
            name: b"EndComposites\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
            key: kEndComposites,
        };
        init
    },
    {
        let mut init = keyname_st {
            name: b"EndDescendent\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
            key: kEndDescendent,
        };
        init
    },
    {
        let mut init = keyname_st {
            name: b"EndDirection\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
            key: kEndDirection,
        };
        init
    },
    {
        let mut init = keyname_st {
            name: b"EndFontMetrics\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
            key: kEndFontMetrics,
        };
        init
    },
    {
        let mut init = keyname_st {
            name: b"EndKernData\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
            key: kEndKernData,
        };
        init
    },
    {
        let mut init = keyname_st {
            name: b"EndKernPairs\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
            key: kEndKernPairs,
        };
        init
    },
    {
        let mut init = keyname_st {
            name: b"EndMaster\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
            key: kEndMaster,
        };
        init
    },
    {
        let mut init = keyname_st {
            name: b"EndMasterFontMetrics\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
            key: kEndMasterFontMetrics,
        };
        init
    },
    {
        let mut init = keyname_st {
            name: b"EndTrackKern\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
            key: kEndTrackKern,
        };
        init
    },
    {
        let mut init = keyname_st {
            name: b"EscChar\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            key: kEscChar,
        };
        init
    },
    {
        let mut init = keyname_st {
            name: b"FamilyName\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
            key: kFamilyName,
        };
        init
    },
    {
        let mut init = keyname_st {
            name: b"FontBBox\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            key: kFontBBox,
        };
        init
    },
    {
        let mut init = keyname_st {
            name: b"FontName\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            key: kFontName,
        };
        init
    },
    {
        let mut init = keyname_st {
            name: b"FullName\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            key: kFullName,
        };
        init
    },
    {
        let mut init = keyname_st {
            name: b"IsBaseFont\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
            key: kIsBaseFont,
        };
        init
    },
    {
        let mut init = keyname_st {
            name: b"IsFixedPitch\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
            key: kIsFixedPitch,
        };
        init
    },
    {
        let mut init = keyname_st {
            name: b"IsFixedV\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            key: kIsFixedV,
        };
        init
    },
    {
        let mut init = keyname_st {
            name: b"ItalicAngle\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
            key: kItalicAngle,
        };
        init
    },
    {
        let mut init = keyname_st {
            name: b"KP\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            key: kKP,
        };
        init
    },
    {
        let mut init = keyname_st {
            name: b"KPH\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            key: kKPH,
        };
        init
    },
    {
        let mut init = keyname_st {
            name: b"KPX\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            key: kKPX,
        };
        init
    },
    {
        let mut init = keyname_st {
            name: b"KPY\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            key: kKPY,
        };
        init
    },
    {
        let mut init = keyname_st {
            name: b"L\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            key: kL,
        };
        init
    },
    {
        let mut init = keyname_st {
            name: b"MappingScheme\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
            key: kMappingScheme,
        };
        init
    },
    {
        let mut init = keyname_st {
            name: b"Masters\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            key: kMasters,
        };
        init
    },
    {
        let mut init = keyname_st {
            name: b"MetricsSets\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
            key: kMetricsSets,
        };
        init
    },
    {
        let mut init = keyname_st {
            name: b"N\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            key: kN,
        };
        init
    },
    {
        let mut init = keyname_st {
            name: b"Notice\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            key: kNotice,
        };
        init
    },
    {
        let mut init = keyname_st {
            name: b"PCC\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            key: kPCC,
        };
        init
    },
    {
        let mut init = keyname_st {
            name: b"StartAxis\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
            key: kStartAxis,
        };
        init
    },
    {
        let mut init = keyname_st {
            name: b"StartCharMetrics\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
            key: kStartCharMetrics,
        };
        init
    },
    {
        let mut init = keyname_st {
            name: b"StartCompFontMetrics\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
            key: kStartCompFontMetrics,
        };
        init
    },
    {
        let mut init = keyname_st {
            name: b"StartComposites\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
            key: kStartComposites,
        };
        init
    },
    {
        let mut init = keyname_st {
            name: b"StartDescendent\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
            key: kStartDescendent,
        };
        init
    },
    {
        let mut init = keyname_st {
            name: b"StartDirection\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
            key: kStartDirection,
        };
        init
    },
    {
        let mut init = keyname_st {
            name: b"StartFontMetrics\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
            key: kStartFontMetrics,
        };
        init
    },
    {
        let mut init = keyname_st {
            name: b"StartKernData\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
            key: kStartKernData,
        };
        init
    },
    {
        let mut init = keyname_st {
            name: b"StartKernPairs\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
            key: kStartKernPairs,
        };
        init
    },
    {
        let mut init = keyname_st {
            name: b"StartMaster\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
            key: kStartMaster,
        };
        init
    },
    {
        let mut init = keyname_st {
            name: b"StartMasterFontMetrics\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
            key: kStartMasterFontMetrics,
        };
        init
    },
    {
        let mut init = keyname_st {
            name: b"StartTrackKern\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
            key: kStartTrackKern,
        };
        init
    },
    {
        let mut init = keyname_st {
            name: b"TrackKern\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
            key: kTrackKern,
        };
        init
    },
    {
        let mut init = keyname_st {
            name: b"UnderlinePosition\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
            key: kUnderlinePosition,
        };
        init
    },
    {
        let mut init = keyname_st {
            name: b"UnderlineThickness\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
            key: kUnderlineThickness,
        };
        init
    },
    {
        let mut init = keyname_st {
            name: b"VV\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            key: kVV,
        };
        init
    },
    {
        let mut init = keyname_st {
            name: b"VVector\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            key: kVVector,
        };
        init
    },
    {
        let mut init = keyname_st {
            name: b"Version\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            key: kVersion,
        };
        init
    },
    {
        let mut init = keyname_st {
            name: b"W\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            key: kW,
        };
        init
    },
    {
        let mut init = keyname_st {
            name: b"W0\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            key: kW0,
        };
        init
    },
    {
        let mut init = keyname_st {
            name: b"W0X\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            key: kW0X,
        };
        init
    },
    {
        let mut init = keyname_st {
            name: b"W0Y\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            key: kW0Y,
        };
        init
    },
    {
        let mut init = keyname_st {
            name: b"W1\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            key: kW1,
        };
        init
    },
    {
        let mut init = keyname_st {
            name: b"W1X\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            key: kW1X,
        };
        init
    },
    {
        let mut init = keyname_st {
            name: b"W1Y\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            key: kW1Y,
        };
        init
    },
    {
        let mut init = keyname_st {
            name: b"WX\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            key: kWX,
        };
        init
    },
    {
        let mut init = keyname_st {
            name: b"WY\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            key: kWY,
        };
        init
    },
    {
        let mut init = keyname_st {
            name: b"Weight\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            key: kWeight,
        };
        init
    },
    {
        let mut init = keyname_st {
            name: b"WeightVector\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
            key: kWeightVector,
        };
        init
    },
    {
        let mut init = keyname_st {
            name: b"XHeight\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            key: kXHeight,
        };
        init
    },
    {
        let mut init = keyname_st {
            name: 0 as *const libc::c_char as *mut libc::c_char,
            key: kComment,
        };
        init
    },
];
pub unsafe extern "C" fn afm_parse_file(
    mut handle: AFMHandle,
    mut filename: *const libc::c_char,
    mut font: AFMFont,
) {
    let mut key: AFMKey = kComment;
    let mut node: AFMNode = AFMNode {
        type_0: 0,
        u: C2RustUnnamed {
            string: 0 as *mut libc::c_char,
        },
    };
    let mut context: ParseCtx = ParseCtx {
        fp: 0 as *mut FILE,
        token: [0; 1024],
        tokenlen: 0,
    };
    let mut ctx: *mut ParseCtx = &mut context;
    let mut wd: libc::c_int = 0 as libc::c_int;
    let mut done: libc::c_int = 0 as libc::c_int;
    (*ctx).fp = fopen(filename, b"r\0" as *const u8 as *const libc::c_char);
    if ((*ctx).fp).is_null() {
        parse_error(
            handle,
            (*__errno_location() << 16 as libc::c_int | 7 as libc::c_int) as AFMError,
        );
    }
    get_key(handle, ctx, &mut key);
    if key as libc::c_uint != kStartFontMetrics as libc::c_int as libc::c_uint {
        parse_error(handle, 8 as libc::c_int as AFMError);
    }
    get_type(handle, ctx, 3 as libc::c_int, &mut node);
    (*font).version = node.u.number;
    while done == 0 {
        get_key(handle, ctx, &mut key);
        match key as libc::c_uint {
            0 => {
                get_line_token(handle, ctx);
            }
            1 => {
                get_type(handle, ctx, 3 as libc::c_int, &mut node);
                (*font).version = node.u.number;
            }
            2 => {
                done = 1 as libc::c_int;
            }
            3 | 4 | 7 | 8 => {
                parse_error(handle, 6 as libc::c_int as AFMError);
            }
            13 => {
                get_type(handle, ctx, 1 as libc::c_int, &mut node);
                (*font).global_info.FontName = node.u.string;
            }
            14 => {
                get_type(handle, ctx, 1 as libc::c_int, &mut node);
                (*font).global_info.FullName = node.u.string;
            }
            15 => {
                get_type(handle, ctx, 1 as libc::c_int, &mut node);
                (*font).global_info.FamilyName = node.u.string;
            }
            16 => {
                get_type(handle, ctx, 1 as libc::c_int, &mut node);
                (*font).global_info.Weight = node.u.string;
            }
            17 => {
                get_type(handle, ctx, 3 as libc::c_int, &mut node);
                (*font).global_info.FontBBox_llx = node.u.number;
                get_type(handle, ctx, 3 as libc::c_int, &mut node);
                (*font).global_info.FontBBox_lly = node.u.number;
                get_type(handle, ctx, 3 as libc::c_int, &mut node);
                (*font).global_info.FontBBox_urx = node.u.number;
                get_type(handle, ctx, 3 as libc::c_int, &mut node);
                (*font).global_info.FontBBox_ury = node.u.number;
            }
            18 => {
                get_type(handle, ctx, 1 as libc::c_int, &mut node);
                (*font).global_info.Version = node.u.string;
            }
            19 => {
                get_type(handle, ctx, 1 as libc::c_int, &mut node);
                (*font).global_info.Notice = node.u.string;
            }
            20 => {
                get_type(handle, ctx, 1 as libc::c_int, &mut node);
                (*font).global_info.EncodingScheme = node.u.string;
            }
            21 => {
                get_type(handle, ctx, 4 as libc::c_int, &mut node);
                (*font).global_info.MappingScheme = node.u.integer;
            }
            22 => {
                get_type(handle, ctx, 4 as libc::c_int, &mut node);
                (*font).global_info.EscChar = node.u.integer;
            }
            23 => {
                get_type(handle, ctx, 1 as libc::c_int, &mut node);
                (*font).global_info.CharacterSet = node.u.string;
            }
            24 => {
                get_type(handle, ctx, 4 as libc::c_int, &mut node);
                (*font).global_info.Characters = node.u.integer;
            }
            25 => {
                get_type(handle, ctx, 6 as libc::c_int, &mut node);
                (*font).global_info.IsBaseFont = node.u.boolean;
            }
            26 => {
                get_type(handle, ctx, 3 as libc::c_int, &mut node);
                (*font).global_info.VVector_0 = node.u.number;
                get_type(handle, ctx, 3 as libc::c_int, &mut node);
                (*font).global_info.VVector_1 = node.u.number;
            }
            27 => {
                get_type(handle, ctx, 6 as libc::c_int, &mut node);
                (*font).global_info.IsFixedV = node.u.boolean;
            }
            28 => {
                get_type(handle, ctx, 3 as libc::c_int, &mut node);
                (*font).global_info.CapHeight = node.u.number;
            }
            29 => {
                get_type(handle, ctx, 3 as libc::c_int, &mut node);
                (*font).global_info.XHeight = node.u.number;
            }
            30 => {
                get_type(handle, ctx, 3 as libc::c_int, &mut node);
                (*font).global_info.Ascender = node.u.number;
            }
            31 => {
                get_type(handle, ctx, 3 as libc::c_int, &mut node);
                (*font).global_info.Descender = node.u.number;
            }
            36 => {
                get_type(handle, ctx, 4 as libc::c_int, &mut node);
                wd = node.u.integer as libc::c_int;
                (*font).writing_direction_metrics[wd as usize].is_valid = AFMTrue;
            }
            38 => {
                get_type(handle, ctx, 3 as libc::c_int, &mut node);
                (*font)
                    .writing_direction_metrics[wd as usize]
                    .UnderlinePosition = node.u.number;
            }
            39 => {
                get_type(handle, ctx, 3 as libc::c_int, &mut node);
                (*font)
                    .writing_direction_metrics[wd as usize]
                    .UnderlineThickness = node.u.number;
            }
            40 => {
                get_type(handle, ctx, 3 as libc::c_int, &mut node);
                (*font)
                    .writing_direction_metrics[wd as usize]
                    .ItalicAngle = node.u.number;
            }
            41 => {
                get_type(handle, ctx, 3 as libc::c_int, &mut node);
                (*font)
                    .writing_direction_metrics[wd as usize]
                    .CharWidth_x = node.u.number;
                get_type(handle, ctx, 3 as libc::c_int, &mut node);
                (*font)
                    .writing_direction_metrics[wd as usize]
                    .CharWidth_y = node.u.number;
            }
            42 => {
                get_type(handle, ctx, 6 as libc::c_int, &mut node);
                (*font)
                    .writing_direction_metrics[wd as usize]
                    .IsFixedPitch = node.u.boolean;
            }
            43 => {
                get_type(handle, ctx, 4 as libc::c_int, &mut node);
                (*font).num_character_metrics = node.u.integer;
                (*font)
                    .character_metrics = calloc(
                    ((*font).num_character_metrics + 1 as libc::c_int as libc::c_long)
                        as libc::c_ulong,
                    ::std::mem::size_of::<AFMIndividualCharacterMetrics>()
                        as libc::c_ulong,
                ) as *mut AFMIndividualCharacterMetrics;
                if ((*font).character_metrics).is_null() {
                    parse_error(handle, 2 as libc::c_int as AFMError);
                }
                read_character_metrics(handle, ctx, font);
            }
            65 => {
                if (*font).info_level & 0x2 as libc::c_int as libc::c_uint != 0 {
                    get_type(handle, ctx, 4 as libc::c_int, &mut node);
                    (*font).num_kern_pairs = node.u.integer;
                    (*font)
                        .kern_pairs = calloc(
                        ((*font).num_kern_pairs + 1 as libc::c_int as libc::c_long)
                            as libc::c_ulong,
                        ::std::mem::size_of::<AFMPairWiseKerning>() as libc::c_ulong,
                    ) as *mut AFMPairWiseKerning;
                    if ((*font).kern_pairs).is_null() {
                        parse_error(handle, 2 as libc::c_int as AFMError);
                    }
                    read_kern_pairs(handle, ctx, font);
                } else {
                    loop {
                        get_line_token(handle, ctx);
                        get_key(handle, ctx, &mut key);
                        if !(key as libc::c_uint
                            != kEndKernPairs as libc::c_int as libc::c_uint)
                        {
                            break;
                        }
                    }
                }
            }
            62 => {
                if (*font).info_level & 0x4 as libc::c_int as libc::c_uint != 0 {
                    get_type(handle, ctx, 4 as libc::c_int, &mut node);
                    (*font).num_track_kerns = node.u.integer;
                    (*font)
                        .track_kerns = calloc(
                        ((*font).num_track_kerns + 1 as libc::c_int as libc::c_long)
                            as libc::c_ulong,
                        ::std::mem::size_of::<AFMTrackKern>() as libc::c_ulong,
                    ) as *mut AFMTrackKern;
                    if ((*font).track_kerns).is_null() {
                        parse_error(handle, 2 as libc::c_int as AFMError);
                    }
                    read_track_kerns(handle, ctx, font);
                } else {
                    loop {
                        get_line_token(handle, ctx);
                        get_key(handle, ctx, &mut key);
                        if !(key as libc::c_uint
                            != kEndTrackKern as libc::c_int as libc::c_uint)
                        {
                            break;
                        }
                    }
                }
            }
            37 | 60 | 61 => {}
            71 => {
                if (*font).info_level & 0x1 as libc::c_int as libc::c_uint != 0 {
                    get_type(handle, ctx, 4 as libc::c_int, &mut node);
                    (*font).num_composites = node.u.integer;
                    (*font)
                        .composites = calloc(
                        ((*font).num_composites + 1 as libc::c_int as libc::c_long)
                            as libc::c_ulong,
                        ::std::mem::size_of::<AFMComposite>() as libc::c_ulong,
                    ) as *mut AFMComposite;
                    if ((*font).composites).is_null() {
                        parse_error(handle, 2 as libc::c_int as AFMError);
                    }
                    read_composites(handle, ctx, font);
                } else {
                    loop {
                        get_line_token(handle, ctx);
                        get_key(handle, ctx, &mut key);
                        if !(key as libc::c_uint
                            != kEndComposites as libc::c_int as libc::c_uint)
                        {
                            break;
                        }
                    }
                }
            }
            _ => {}
        }
    }
    fclose((*ctx).fp);
    if (*font).writing_direction_metrics[0 as libc::c_int as usize].is_valid as u64 == 0
        && (*font).writing_direction_metrics[1 as libc::c_int as usize].is_valid as u64
            == 0
    {
        (*font).writing_direction_metrics[0 as libc::c_int as usize].is_valid = AFMTrue;
    }
    if strhash_get(
        (*(*font).private).fontnames,
        b"space\0" as *const u8 as *const libc::c_char,
        5 as libc::c_int,
        (*(*font).private).undef as *mut libc::c_void as *mut *mut libc::c_void,
    ) == 0
    {
        if (*font).num_character_metrics > 0 as libc::c_int as libc::c_long {} else {
            __assert_fail(
                b"font->num_character_metrics > 0\0" as *const u8 as *const libc::c_char,
                b"afmparse.c\0" as *const u8 as *const libc::c_char,
                479 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 54],
                    &[libc::c_char; 54],
                >(b"void afm_parse_file(AFMHandle, const char *, AFMFont)\0"))
                    .as_ptr(),
            );
        }
        'c_4356: {
            if (*font).num_character_metrics > 0 as libc::c_int as libc::c_long {} else {
                __assert_fail(
                    b"font->num_character_metrics > 0\0" as *const u8
                        as *const libc::c_char,
                    b"afmparse.c\0" as *const u8 as *const libc::c_char,
                    479 as libc::c_int as libc::c_uint,
                    (*::std::mem::transmute::<
                        &[u8; 54],
                        &[libc::c_char; 54],
                    >(b"void afm_parse_file(AFMHandle, const char *, AFMFont)\0"))
                        .as_ptr(),
                );
            }
        };
        (*(*font).private)
            .undef = &mut *((*font).character_metrics).offset(0 as libc::c_int as isize)
            as *mut AFMIndividualCharacterMetrics;
    }
    if (*font).writing_direction_metrics[0 as libc::c_int as usize].is_valid
        as libc::c_uint != 0
        && (*font).writing_direction_metrics[0 as libc::c_int as usize].IsFixedPitch
            as libc::c_uint != 0
    {
        (*font)
            .writing_direction_metrics[0 as libc::c_int as usize]
            .CharWidth_x = (*((*font).character_metrics)
            .offset(0 as libc::c_int as isize))
            .w0x;
        (*font)
            .writing_direction_metrics[0 as libc::c_int as usize]
            .CharWidth_y = (*((*font).character_metrics)
            .offset(0 as libc::c_int as isize))
            .w0y;
    }
    if (*font).writing_direction_metrics[1 as libc::c_int as usize].is_valid
        as libc::c_uint != 0
        && (*font).writing_direction_metrics[1 as libc::c_int as usize].IsFixedPitch
            as libc::c_uint != 0
    {
        (*font)
            .writing_direction_metrics[1 as libc::c_int as usize]
            .CharWidth_x = (*((*font).character_metrics)
            .offset(1 as libc::c_int as isize))
            .w1x;
        (*font)
            .writing_direction_metrics[1 as libc::c_int as usize]
            .CharWidth_y = (*((*font).character_metrics)
            .offset(1 as libc::c_int as isize))
            .w1y;
    }
}
unsafe extern "C" fn parse_error(mut handle: AFMHandle, mut error: AFMError) {
    (*handle).parse_error = error;
    longjmp(((*handle).jmpbuf).as_mut_ptr(), 1 as libc::c_int);
}
unsafe extern "C" fn get_token(
    mut handle: AFMHandle,
    mut ctx: *mut ParseCtx,
) -> libc::c_int {
    let mut ch: libc::c_int = 0;
    let mut i: libc::c_int = 0;
    loop {
        ch = getc((*ctx).fp);
        if !(ch != -(1 as libc::c_int)) {
            break;
        }
        if !(ch == ' ' as i32 || ch == '\n' as i32 || ch == '\r' as i32
            || ch == '\t' as i32 || ch == ';' as i32)
        {
            break;
        }
    }
    if ch == -(1 as libc::c_int) {
        return 0 as libc::c_int;
    }
    ungetc(ch, (*ctx).fp);
    i = 0 as libc::c_int;
    ch = getc((*ctx).fp);
    while (i as libc::c_ulong)
        < ::std::mem::size_of::<[libc::c_char; 1024]>() as libc::c_ulong
        && ch != -(1 as libc::c_int)
        && !(ch == ' ' as i32 || ch == '\n' as i32 || ch == '\r' as i32
            || ch == '\t' as i32 || ch == ';' as i32)
    {
        (*ctx).token[i as usize] = ch as libc::c_char;
        i += 1;
        i;
        ch = getc((*ctx).fp);
    }
    if i as libc::c_ulong
        >= ::std::mem::size_of::<[libc::c_char; 1024]>() as libc::c_ulong
    {
        parse_error(handle, 5 as libc::c_int as AFMError);
    }
    (*ctx).token[i as usize] = '\0' as i32 as libc::c_char;
    (*ctx).tokenlen = i as libc::c_uint;
    return 1 as libc::c_int;
}
unsafe extern "C" fn get_line_token(
    mut handle: AFMHandle,
    mut ctx: *mut ParseCtx,
) -> libc::c_int {
    let mut i: libc::c_int = 0;
    let mut ch: libc::c_int = 0;
    loop {
        ch = getc((*ctx).fp);
        if !(ch != -(1 as libc::c_int)) {
            break;
        }
        if !(ch == ' ' as i32 || ch == '\n' as i32 || ch == '\r' as i32
            || ch == '\t' as i32 || ch == ';' as i32)
        {
            break;
        }
    }
    if ch == -(1 as libc::c_int) {
        return 0 as libc::c_int;
    }
    ungetc(ch, (*ctx).fp);
    i = 0 as libc::c_int;
    ch = getc((*ctx).fp);
    while (i as libc::c_ulong)
        < ::std::mem::size_of::<[libc::c_char; 1024]>() as libc::c_ulong
        && ch != -(1 as libc::c_int) && ch != '\n' as i32
    {
        (*ctx).token[i as usize] = ch as libc::c_char;
        i += 1;
        i;
        ch = getc((*ctx).fp);
    }
    if i as libc::c_ulong
        >= ::std::mem::size_of::<[libc::c_char; 1024]>() as libc::c_ulong
    {
        parse_error(handle, 5 as libc::c_int as AFMError);
    }
    i -= 1;
    i;
    while i >= 0 as libc::c_int
        && ((*ctx).token[i as usize] as libc::c_int == ' ' as i32
            || (*ctx).token[i as usize] as libc::c_int == '\n' as i32
            || (*ctx).token[i as usize] as libc::c_int == '\r' as i32
            || (*ctx).token[i as usize] as libc::c_int == '\t' as i32
            || (*ctx).token[i as usize] as libc::c_int == ';' as i32)
    {
        i -= 1;
        i;
    }
    i += 1;
    i;
    (*ctx).token[i as usize] = '\0' as i32 as libc::c_char;
    (*ctx).tokenlen = i as libc::c_uint;
    return 1 as libc::c_int;
}
unsafe extern "C" fn match_key(mut key: *mut libc::c_char) -> libc::c_int {
    let mut lower: libc::c_int = 0 as libc::c_int;
    let mut upper: libc::c_int = (::std::mem::size_of::<[keyname_st; 82]>()
        as libc::c_ulong)
        .wrapping_div(::std::mem::size_of::<keyname_st>() as libc::c_ulong)
        .wrapping_sub(1 as libc::c_int as libc::c_ulong) as libc::c_int;
    let mut midpoint: libc::c_int = 0;
    let mut cmpvalue: libc::c_int = 0;
    let mut found: AFMBoolean = AFMFalse;
    while upper >= lower && found as u64 == 0 {
        midpoint = (lower + upper) / 2 as libc::c_int;
        if (keynames[midpoint as usize].name).is_null() {
            break;
        }
        cmpvalue = strcmp(key, keynames[midpoint as usize].name);
        if cmpvalue == 0 as libc::c_int {
            found = AFMTrue;
        } else if cmpvalue < 0 as libc::c_int {
            upper = midpoint - 1 as libc::c_int;
        } else {
            lower = midpoint + 1 as libc::c_int;
        }
    }
    if found as u64 != 0 {
        return keynames[midpoint as usize].key as libc::c_int;
    }
    return -(1 as libc::c_int);
}
unsafe extern "C" fn get_key(
    mut handle: AFMHandle,
    mut ctx: *mut ParseCtx,
    mut key_return: *mut AFMKey,
) {
    let mut key: libc::c_int = 0;
    let mut msg: [libc::c_char; 256] = [0; 256];
    loop {
        if get_token(handle, ctx) == 0 {
            parse_error(handle, 5 as libc::c_int as AFMError);
        }
        key = match_key(((*ctx).token).as_mut_ptr());
        if key >= 0 as libc::c_int {
            *key_return = key as AFMKey;
            return;
        }
        sprintf(
            msg.as_mut_ptr(),
            b"skipping key \"%s\"\0" as *const u8 as *const libc::c_char,
            ((*ctx).token).as_mut_ptr(),
        );
        afm_error(handle, msg.as_mut_ptr());
        get_line_token(handle, ctx);
    };
}
unsafe extern "C" fn get_type(
    mut handle: AFMHandle,
    mut ctx: *mut ParseCtx,
    mut type_0: libc::c_int,
    mut type_return: *mut AFMNode,
) {
    let mut buf: [libc::c_char; 256] = [0; 256];
    match type_0 {
        1 => {
            if get_line_token(handle, ctx) == 0 {
                parse_error(handle, 5 as libc::c_int as AFMError);
            }
            (*type_return)
                .u
                .string = calloc(
                1 as libc::c_int as libc::c_ulong,
                ((*ctx).tokenlen).wrapping_add(1 as libc::c_int as libc::c_uint)
                    as libc::c_ulong,
            ) as AFMString;
            if ((*type_return).u.string).is_null() {
                parse_error(handle, 2 as libc::c_int as AFMError);
            }
            memcpy(
                (*type_return).u.string as *mut libc::c_void,
                ((*ctx).token).as_mut_ptr() as *const libc::c_void,
                (*ctx).tokenlen as libc::c_ulong,
            );
        }
        2 => {
            if get_token(handle, ctx) == 0 {
                parse_error(handle, 5 as libc::c_int as AFMError);
            }
            (*type_return)
                .u
                .name = calloc(
                1 as libc::c_int as libc::c_ulong,
                ((*ctx).tokenlen).wrapping_add(1 as libc::c_int as libc::c_uint)
                    as libc::c_ulong,
            ) as AFMName;
            if ((*type_return).u.string).is_null() {
                parse_error(handle, 2 as libc::c_int as AFMError);
            }
            memcpy(
                (*type_return).u.name as *mut libc::c_void,
                ((*ctx).token).as_mut_ptr() as *const libc::c_void,
                (*ctx).tokenlen as libc::c_ulong,
            );
        }
        3 => {
            if get_token(handle, ctx) == 0 {
                parse_error(handle, 5 as libc::c_int as AFMError);
            }
            memcpy(
                buf.as_mut_ptr() as *mut libc::c_void,
                ((*ctx).token).as_mut_ptr() as *const libc::c_void,
                (*ctx).tokenlen as libc::c_ulong,
            );
            buf[(*ctx).tokenlen as usize] = '\0' as i32 as libc::c_char;
            (*type_return).u.number = atof(buf.as_mut_ptr());
        }
        4 => {
            if get_token(handle, ctx) == 0 {
                parse_error(handle, 5 as libc::c_int as AFMError);
            }
            memcpy(
                buf.as_mut_ptr() as *mut libc::c_void,
                ((*ctx).token).as_mut_ptr() as *const libc::c_void,
                (*ctx).tokenlen as libc::c_ulong,
            );
            buf[(*ctx).tokenlen as usize] = '\0' as i32 as libc::c_char;
            (*type_return).u.integer = atoi(buf.as_mut_ptr()) as AFMInteger;
        }
        5 => {
            fprintf(
                stderr,
                b"Array types not implemented yet.\n\0" as *const u8
                    as *const libc::c_char,
            );
            abort();
        }
        6 => {
            if get_token(handle, ctx) == 0 {
                parse_error(handle, 5 as libc::c_int as AFMError);
            }
            memcpy(
                buf.as_mut_ptr() as *mut libc::c_void,
                ((*ctx).token).as_mut_ptr() as *const libc::c_void,
                (*ctx).tokenlen as libc::c_ulong,
            );
            buf[(*ctx).tokenlen as usize] = '\0' as i32 as libc::c_char;
            if strcmp(buf.as_mut_ptr(), b"true\0" as *const u8 as *const libc::c_char)
                == 0 as libc::c_int
            {
                (*type_return).u.boolean = AFMTrue;
            } else if strcmp(
                buf.as_mut_ptr(),
                b"false\0" as *const u8 as *const libc::c_char,
            ) == 0 as libc::c_int
            {
                (*type_return).u.boolean = AFMFalse;
            } else {
                parse_error(handle, 5 as libc::c_int as AFMError);
            }
        }
        _ => {
            fprintf(
                stderr,
                b"get_type(): illegal type %d\n\0" as *const u8 as *const libc::c_char,
                (*type_return).type_0,
            );
            abort();
        }
    };
}
unsafe extern "C" fn read_character_metrics(
    mut handle: AFMHandle,
    mut ctx: *mut ParseCtx,
    mut font: AFMFont,
) {
    let mut i: libc::c_int = 0 as libc::c_int;
    let mut node: AFMNode = AFMNode {
        type_0: 0,
        u: C2RustUnnamed {
            string: 0 as *mut libc::c_char,
        },
    };
    let mut cm: *mut AFMIndividualCharacterMetrics = 0
        as *mut AFMIndividualCharacterMetrics;
    let mut key: AFMKey = kComment;
    let mut done: libc::c_int = 0 as libc::c_int;
    let mut first: libc::c_int = 1 as libc::c_int;
    while done == 0 {
        get_key(handle, ctx, &mut key);
        match key as libc::c_uint {
            45 => {
                if first != 0 {
                    first = 0 as libc::c_int;
                } else {
                    i += 1;
                    i;
                }
                if i as libc::c_long >= (*font).num_character_metrics {
                    parse_error(handle, 5 as libc::c_int as AFMError);
                }
                cm = &mut *((*font).character_metrics).offset(i as isize)
                    as *mut AFMIndividualCharacterMetrics;
                get_type(handle, ctx, 4 as libc::c_int, &mut node);
                (*cm).character_code = node.u.integer;
                if (*cm).character_code >= 0 as libc::c_int as libc::c_long
                    && (*cm).character_code <= 255 as libc::c_int as libc::c_long
                {
                    (*font).encoding[(*cm).character_code as usize] = cm;
                }
            }
            46 => {
                printf(b"* CH\n\0" as *const u8 as *const libc::c_char);
            }
            47 | 48 => {
                get_type(handle, ctx, 3 as libc::c_int, &mut node);
                (*cm).w0x = node.u.number;
                (*cm).w0y = 0.0f64;
            }
            49 => {
                get_type(handle, ctx, 3 as libc::c_int, &mut node);
                (*cm).w1x = node.u.number;
                (*cm).w1y = 0.0f64;
            }
            50 | 51 => {
                get_type(handle, ctx, 3 as libc::c_int, &mut node);
                (*cm).w0y = node.u.number;
                (*cm).w0x = 0.0f64;
            }
            52 => {
                get_type(handle, ctx, 3 as libc::c_int, &mut node);
                (*cm).w1y = node.u.number;
                (*cm).w1x = 0.0f64;
            }
            53 | 54 => {
                get_type(handle, ctx, 3 as libc::c_int, &mut node);
                (*cm).w0x = node.u.number;
                get_type(handle, ctx, 3 as libc::c_int, &mut node);
                (*cm).w0y = node.u.number;
            }
            55 => {
                get_type(handle, ctx, 3 as libc::c_int, &mut node);
                (*cm).w1x = node.u.number;
                get_type(handle, ctx, 3 as libc::c_int, &mut node);
                (*cm).w1y = node.u.number;
            }
            56 => {
                get_type(handle, ctx, 3 as libc::c_int, &mut node);
                (*cm).vv_x = node.u.number;
                get_type(handle, ctx, 3 as libc::c_int, &mut node);
                (*cm).vv_y = node.u.number;
            }
            57 => {
                get_type(handle, ctx, 2 as libc::c_int, &mut node);
                (*cm).name = node.u.name;
                if strhash_put(
                    (*(*font).private).fontnames,
                    (*cm).name,
                    strlen((*cm).name as *const libc::c_char) as libc::c_int,
                    cm as *mut libc::c_void,
                    0 as *mut *mut libc::c_void,
                ) == 0
                {
                    parse_error(handle, 2 as libc::c_int as AFMError);
                }
            }
            58 => {
                get_type(handle, ctx, 3 as libc::c_int, &mut node);
                (*cm).llx = node.u.number;
                get_type(handle, ctx, 3 as libc::c_int, &mut node);
                (*cm).lly = node.u.number;
                get_type(handle, ctx, 3 as libc::c_int, &mut node);
                (*cm).urx = node.u.number;
                get_type(handle, ctx, 3 as libc::c_int, &mut node);
                (*cm).ury = node.u.number;
            }
            59 => {
                get_line_token(handle, ctx);
            }
            44 => {
                if i as libc::c_long
                    != (*font).num_character_metrics - 1 as libc::c_int as libc::c_long
                {
                    (*font).num_character_metrics = (i + 1 as libc::c_int) as AFMInteger;
                }
                done = 1 as libc::c_int;
            }
            _ => {
                parse_error(handle, 5 as libc::c_int as AFMError);
            }
        }
    }
}
unsafe extern "C" fn read_kern_pairs(
    mut handle: AFMHandle,
    mut ctx: *mut ParseCtx,
    mut font: AFMFont,
) {
    let mut i: libc::c_int = 0;
    let mut node: AFMNode = AFMNode {
        type_0: 0,
        u: C2RustUnnamed {
            string: 0 as *mut libc::c_char,
        },
    };
    let mut kp: *mut AFMPairWiseKerning = 0 as *mut AFMPairWiseKerning;
    let mut key: AFMKey = kComment;
    i = 0 as libc::c_int;
    while (i as libc::c_long) < (*font).num_kern_pairs {
        kp = &mut *((*font).kern_pairs).offset(i as isize) as *mut AFMPairWiseKerning;
        get_key(handle, ctx, &mut key);
        match key as libc::c_uint {
            67 | 69 | 70 => {
                get_type(handle, ctx, 2 as libc::c_int, &mut node);
                (*kp).name1 = node.u.name;
                get_type(handle, ctx, 2 as libc::c_int, &mut node);
                (*kp).name2 = node.u.name;
                get_type(handle, ctx, 3 as libc::c_int, &mut node);
                match key as libc::c_uint {
                    67 => {
                        (*kp).kx = node.u.number;
                        get_type(handle, ctx, 3 as libc::c_int, &mut node);
                        (*kp).ky = node.u.number;
                    }
                    69 => {
                        (*kp).kx = node.u.number;
                        (*kp).ky = 0.0f64;
                    }
                    70 => {
                        (*kp).ky = node.u.number;
                        (*kp).kx = 0.0f64;
                    }
                    _ => {
                        fprintf(
                            stderr,
                            b"AFM: fatal corruption\n\0" as *const u8
                                as *const libc::c_char,
                        );
                        abort();
                    }
                }
            }
            68 => {}
            _ => {
                parse_error(handle, 5 as libc::c_int as AFMError);
            }
        }
        i += 1;
        i;
    }
    get_key(handle, ctx, &mut key);
    if key as libc::c_uint != kEndKernPairs as libc::c_int as libc::c_uint {
        parse_error(handle, 5 as libc::c_int as AFMError);
    }
}
unsafe extern "C" fn read_track_kerns(
    mut handle: AFMHandle,
    mut ctx: *mut ParseCtx,
    mut font: AFMFont,
) {
    let mut i: libc::c_int = 0;
    let mut node: AFMNode = AFMNode {
        type_0: 0,
        u: C2RustUnnamed {
            string: 0 as *mut libc::c_char,
        },
    };
    let mut tk: *mut AFMTrackKern = 0 as *mut AFMTrackKern;
    let mut key: AFMKey = kComment;
    i = 0 as libc::c_int;
    while (i as libc::c_long) < (*font).num_kern_pairs {
        tk = &mut *((*font).track_kerns).offset(i as isize) as *mut AFMTrackKern;
        get_key(handle, ctx, &mut key);
        if key as libc::c_uint != kTrackKern as libc::c_int as libc::c_uint {
            parse_error(handle, 5 as libc::c_int as AFMError);
        }
        get_type(handle, ctx, 4 as libc::c_int, &mut node);
        (*tk).degree = node.u.integer;
        get_type(handle, ctx, 3 as libc::c_int, &mut node);
        (*tk).min_ptsize = node.u.number;
        get_type(handle, ctx, 3 as libc::c_int, &mut node);
        (*tk).min_kern = node.u.number;
        get_type(handle, ctx, 3 as libc::c_int, &mut node);
        (*tk).max_ptsize = node.u.number;
        get_type(handle, ctx, 3 as libc::c_int, &mut node);
        (*tk).max_kern = node.u.number;
        i += 1;
        i;
    }
    get_key(handle, ctx, &mut key);
    if key as libc::c_uint != kEndTrackKern as libc::c_int as libc::c_uint {
        parse_error(handle, 5 as libc::c_int as AFMError);
    }
}
unsafe extern "C" fn read_composites(
    mut handle: AFMHandle,
    mut ctx: *mut ParseCtx,
    mut font: AFMFont,
) {
    let mut i: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    let mut node: AFMNode = AFMNode {
        type_0: 0,
        u: C2RustUnnamed {
            string: 0 as *mut libc::c_char,
        },
    };
    let mut cm: *mut AFMComposite = 0 as *mut AFMComposite;
    let mut key: AFMKey = kComment;
    i = 0 as libc::c_int;
    while (i as libc::c_long) < (*font).num_composites {
        cm = &mut *((*font).composites).offset(i as isize) as *mut AFMComposite;
        get_key(handle, ctx, &mut key);
        if key as libc::c_uint != kCC as libc::c_int as libc::c_uint {
            parse_error(handle, 5 as libc::c_int as AFMError);
        }
        get_type(handle, ctx, 2 as libc::c_int, &mut node);
        (*cm).name = node.u.name;
        if strhash_put(
            (*(*font).private).compositenames,
            (*cm).name,
            strlen((*cm).name as *const libc::c_char) as libc::c_int,
            cm as *mut libc::c_void,
            0 as *mut *mut libc::c_void,
        ) == 0
        {
            parse_error(handle, 2 as libc::c_int as AFMError);
        }
        get_type(handle, ctx, 4 as libc::c_int, &mut node);
        (*cm).num_components = node.u.integer;
        (*cm)
            .components = calloc(
            ((*cm).num_components + 1 as libc::c_int as libc::c_long) as libc::c_ulong,
            ::std::mem::size_of::<AFMCompositeComponent>() as libc::c_ulong,
        ) as *mut AFMCompositeComponent;
        j = 0 as libc::c_int;
        while (j as libc::c_long) < (*cm).num_components {
            get_key(handle, ctx, &mut key);
            if key as libc::c_uint != kPCC as libc::c_int as libc::c_uint {
                parse_error(handle, 5 as libc::c_int as AFMError);
            }
            get_type(handle, ctx, 2 as libc::c_int, &mut node);
            let ref mut fresh0 = (*((*cm).components).offset(j as isize)).name;
            *fresh0 = node.u.name;
            get_type(handle, ctx, 3 as libc::c_int, &mut node);
            (*((*cm).components).offset(j as isize)).deltax = node.u.number;
            get_type(handle, ctx, 3 as libc::c_int, &mut node);
            (*((*cm).components).offset(j as isize)).deltay = node.u.number;
            j += 1;
            j;
        }
        i += 1;
        i;
    }
    get_key(handle, ctx, &mut key);
    if key as libc::c_uint != kEndComposites as libc::c_int as libc::c_uint {
        parse_error(handle, 5 as libc::c_int as AFMError);
    }
}
