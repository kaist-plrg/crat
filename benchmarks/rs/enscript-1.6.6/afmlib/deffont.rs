use ::libc;
extern "C" {
    pub type stringhash_st;
    fn malloc(_: libc::c_ulong) -> *mut libc::c_void;
    fn calloc(_: libc::c_ulong, _: libc::c_ulong) -> *mut libc::c_void;
    fn strcpy(_: *mut libc::c_char, _: *const libc::c_char) -> *mut libc::c_char;
    fn strhash_init() -> StringHashPtr;
    fn strhash_put(
        hash: StringHashPtr,
        key: *mut libc::c_char,
        keylen: libc::c_int,
        data: *mut libc::c_void,
        old_data_return: *mut *mut libc::c_void,
    ) -> libc::c_int;
    fn afm_close_font(font: AFMFont) -> AFMError;
    fn strlen(_: *const libc::c_char) -> libc::c_ulong;
}
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
pub type AFMEncodingTable = encoding_table_st;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct encoding_table_st {
    pub code: libc::c_int,
    pub character: *mut libc::c_char,
}
static mut builtin_courier: [AFMEncodingTable; 261] = [
    {
        let mut init = encoding_table_st {
            code: 32 as libc::c_int,
            character: b"space\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = encoding_table_st {
            code: 33 as libc::c_int,
            character: b"exclam\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = encoding_table_st {
            code: 34 as libc::c_int,
            character: b"quotedbl\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = encoding_table_st {
            code: 35 as libc::c_int,
            character: b"numbersign\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = encoding_table_st {
            code: 36 as libc::c_int,
            character: b"dollar\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = encoding_table_st {
            code: 37 as libc::c_int,
            character: b"percent\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = encoding_table_st {
            code: 38 as libc::c_int,
            character: b"ampersand\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = encoding_table_st {
            code: 39 as libc::c_int,
            character: b"quoteright\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = encoding_table_st {
            code: 40 as libc::c_int,
            character: b"parenleft\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = encoding_table_st {
            code: 41 as libc::c_int,
            character: b"parenright\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = encoding_table_st {
            code: 42 as libc::c_int,
            character: b"asterisk\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = encoding_table_st {
            code: 43 as libc::c_int,
            character: b"plus\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = encoding_table_st {
            code: 44 as libc::c_int,
            character: b"comma\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = encoding_table_st {
            code: 45 as libc::c_int,
            character: b"hyphen\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = encoding_table_st {
            code: 46 as libc::c_int,
            character: b"period\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = encoding_table_st {
            code: 47 as libc::c_int,
            character: b"slash\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = encoding_table_st {
            code: 48 as libc::c_int,
            character: b"zero\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = encoding_table_st {
            code: 49 as libc::c_int,
            character: b"one\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = encoding_table_st {
            code: 50 as libc::c_int,
            character: b"two\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = encoding_table_st {
            code: 51 as libc::c_int,
            character: b"three\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = encoding_table_st {
            code: 52 as libc::c_int,
            character: b"four\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = encoding_table_st {
            code: 53 as libc::c_int,
            character: b"five\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = encoding_table_st {
            code: 54 as libc::c_int,
            character: b"six\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = encoding_table_st {
            code: 55 as libc::c_int,
            character: b"seven\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = encoding_table_st {
            code: 56 as libc::c_int,
            character: b"eight\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = encoding_table_st {
            code: 57 as libc::c_int,
            character: b"nine\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = encoding_table_st {
            code: 58 as libc::c_int,
            character: b"colon\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = encoding_table_st {
            code: 59 as libc::c_int,
            character: b"semicolon\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = encoding_table_st {
            code: 60 as libc::c_int,
            character: b"less\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = encoding_table_st {
            code: 61 as libc::c_int,
            character: b"equal\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = encoding_table_st {
            code: 62 as libc::c_int,
            character: b"greater\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = encoding_table_st {
            code: 63 as libc::c_int,
            character: b"question\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = encoding_table_st {
            code: 64 as libc::c_int,
            character: b"at\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = encoding_table_st {
            code: 65 as libc::c_int,
            character: b"A\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = encoding_table_st {
            code: 66 as libc::c_int,
            character: b"B\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = encoding_table_st {
            code: 67 as libc::c_int,
            character: b"C\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = encoding_table_st {
            code: 68 as libc::c_int,
            character: b"D\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = encoding_table_st {
            code: 69 as libc::c_int,
            character: b"E\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = encoding_table_st {
            code: 70 as libc::c_int,
            character: b"F\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = encoding_table_st {
            code: 71 as libc::c_int,
            character: b"G\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = encoding_table_st {
            code: 72 as libc::c_int,
            character: b"H\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = encoding_table_st {
            code: 73 as libc::c_int,
            character: b"I\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = encoding_table_st {
            code: 74 as libc::c_int,
            character: b"J\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = encoding_table_st {
            code: 75 as libc::c_int,
            character: b"K\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = encoding_table_st {
            code: 76 as libc::c_int,
            character: b"L\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = encoding_table_st {
            code: 77 as libc::c_int,
            character: b"M\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = encoding_table_st {
            code: 78 as libc::c_int,
            character: b"N\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = encoding_table_st {
            code: 79 as libc::c_int,
            character: b"O\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = encoding_table_st {
            code: 80 as libc::c_int,
            character: b"P\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = encoding_table_st {
            code: 81 as libc::c_int,
            character: b"Q\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = encoding_table_st {
            code: 82 as libc::c_int,
            character: b"R\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = encoding_table_st {
            code: 83 as libc::c_int,
            character: b"S\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = encoding_table_st {
            code: 84 as libc::c_int,
            character: b"T\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = encoding_table_st {
            code: 85 as libc::c_int,
            character: b"U\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = encoding_table_st {
            code: 86 as libc::c_int,
            character: b"V\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = encoding_table_st {
            code: 87 as libc::c_int,
            character: b"W\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = encoding_table_st {
            code: 88 as libc::c_int,
            character: b"X\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = encoding_table_st {
            code: 89 as libc::c_int,
            character: b"Y\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = encoding_table_st {
            code: 90 as libc::c_int,
            character: b"Z\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = encoding_table_st {
            code: 91 as libc::c_int,
            character: b"bracketleft\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = encoding_table_st {
            code: 92 as libc::c_int,
            character: b"backslash\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = encoding_table_st {
            code: 93 as libc::c_int,
            character: b"bracketright\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = encoding_table_st {
            code: 94 as libc::c_int,
            character: b"asciicircum\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = encoding_table_st {
            code: 95 as libc::c_int,
            character: b"underscore\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = encoding_table_st {
            code: 96 as libc::c_int,
            character: b"quoteleft\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = encoding_table_st {
            code: 97 as libc::c_int,
            character: b"a\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = encoding_table_st {
            code: 98 as libc::c_int,
            character: b"b\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = encoding_table_st {
            code: 99 as libc::c_int,
            character: b"c\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = encoding_table_st {
            code: 100 as libc::c_int,
            character: b"d\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = encoding_table_st {
            code: 101 as libc::c_int,
            character: b"e\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = encoding_table_st {
            code: 102 as libc::c_int,
            character: b"f\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = encoding_table_st {
            code: 103 as libc::c_int,
            character: b"g\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = encoding_table_st {
            code: 104 as libc::c_int,
            character: b"h\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = encoding_table_st {
            code: 105 as libc::c_int,
            character: b"i\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = encoding_table_st {
            code: 106 as libc::c_int,
            character: b"j\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = encoding_table_st {
            code: 107 as libc::c_int,
            character: b"k\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = encoding_table_st {
            code: 108 as libc::c_int,
            character: b"l\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = encoding_table_st {
            code: 109 as libc::c_int,
            character: b"m\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = encoding_table_st {
            code: 110 as libc::c_int,
            character: b"n\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = encoding_table_st {
            code: 111 as libc::c_int,
            character: b"o\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = encoding_table_st {
            code: 112 as libc::c_int,
            character: b"p\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = encoding_table_st {
            code: 113 as libc::c_int,
            character: b"q\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = encoding_table_st {
            code: 114 as libc::c_int,
            character: b"r\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = encoding_table_st {
            code: 115 as libc::c_int,
            character: b"s\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = encoding_table_st {
            code: 116 as libc::c_int,
            character: b"t\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = encoding_table_st {
            code: 117 as libc::c_int,
            character: b"u\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = encoding_table_st {
            code: 118 as libc::c_int,
            character: b"v\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = encoding_table_st {
            code: 119 as libc::c_int,
            character: b"w\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = encoding_table_st {
            code: 120 as libc::c_int,
            character: b"x\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = encoding_table_st {
            code: 121 as libc::c_int,
            character: b"y\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = encoding_table_st {
            code: 122 as libc::c_int,
            character: b"z\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = encoding_table_st {
            code: 123 as libc::c_int,
            character: b"braceleft\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = encoding_table_st {
            code: 124 as libc::c_int,
            character: b"bar\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = encoding_table_st {
            code: 125 as libc::c_int,
            character: b"braceright\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = encoding_table_st {
            code: 126 as libc::c_int,
            character: b"asciitilde\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = encoding_table_st {
            code: 161 as libc::c_int,
            character: b"exclamdown\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = encoding_table_st {
            code: 162 as libc::c_int,
            character: b"cent\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = encoding_table_st {
            code: 163 as libc::c_int,
            character: b"sterling\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = encoding_table_st {
            code: 164 as libc::c_int,
            character: b"fraction\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = encoding_table_st {
            code: 165 as libc::c_int,
            character: b"yen\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = encoding_table_st {
            code: 166 as libc::c_int,
            character: b"florin\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = encoding_table_st {
            code: 167 as libc::c_int,
            character: b"section\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = encoding_table_st {
            code: 168 as libc::c_int,
            character: b"currency\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = encoding_table_st {
            code: 169 as libc::c_int,
            character: b"quotesingle\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = encoding_table_st {
            code: 170 as libc::c_int,
            character: b"quotedblleft\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = encoding_table_st {
            code: 171 as libc::c_int,
            character: b"guillemotleft\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = encoding_table_st {
            code: 172 as libc::c_int,
            character: b"guilsinglleft\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = encoding_table_st {
            code: 173 as libc::c_int,
            character: b"guilsinglright\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = encoding_table_st {
            code: 174 as libc::c_int,
            character: b"fi\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = encoding_table_st {
            code: 175 as libc::c_int,
            character: b"fl\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = encoding_table_st {
            code: 177 as libc::c_int,
            character: b"endash\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = encoding_table_st {
            code: 178 as libc::c_int,
            character: b"dagger\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = encoding_table_st {
            code: 179 as libc::c_int,
            character: b"daggerdbl\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = encoding_table_st {
            code: 180 as libc::c_int,
            character: b"periodcentered\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = encoding_table_st {
            code: 182 as libc::c_int,
            character: b"paragraph\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = encoding_table_st {
            code: 183 as libc::c_int,
            character: b"bullet\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = encoding_table_st {
            code: 184 as libc::c_int,
            character: b"quotesinglbase\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = encoding_table_st {
            code: 185 as libc::c_int,
            character: b"quotedblbase\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = encoding_table_st {
            code: 186 as libc::c_int,
            character: b"quotedblright\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = encoding_table_st {
            code: 187 as libc::c_int,
            character: b"guillemotright\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = encoding_table_st {
            code: 188 as libc::c_int,
            character: b"ellipsis\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = encoding_table_st {
            code: 189 as libc::c_int,
            character: b"perthousand\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = encoding_table_st {
            code: 191 as libc::c_int,
            character: b"questiondown\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = encoding_table_st {
            code: 193 as libc::c_int,
            character: b"grave\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = encoding_table_st {
            code: 194 as libc::c_int,
            character: b"acute\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = encoding_table_st {
            code: 195 as libc::c_int,
            character: b"circumflex\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = encoding_table_st {
            code: 196 as libc::c_int,
            character: b"tilde\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = encoding_table_st {
            code: 197 as libc::c_int,
            character: b"macron\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = encoding_table_st {
            code: 198 as libc::c_int,
            character: b"breve\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = encoding_table_st {
            code: 199 as libc::c_int,
            character: b"dotaccent\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = encoding_table_st {
            code: 200 as libc::c_int,
            character: b"dieresis\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = encoding_table_st {
            code: 202 as libc::c_int,
            character: b"ring\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = encoding_table_st {
            code: 203 as libc::c_int,
            character: b"cedilla\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = encoding_table_st {
            code: 205 as libc::c_int,
            character: b"hungarumlaut\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = encoding_table_st {
            code: 206 as libc::c_int,
            character: b"ogonek\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = encoding_table_st {
            code: 207 as libc::c_int,
            character: b"caron\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = encoding_table_st {
            code: 208 as libc::c_int,
            character: b"emdash\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = encoding_table_st {
            code: 225 as libc::c_int,
            character: b"AE\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = encoding_table_st {
            code: 227 as libc::c_int,
            character: b"ordfeminine\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = encoding_table_st {
            code: 232 as libc::c_int,
            character: b"Lslash\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = encoding_table_st {
            code: 233 as libc::c_int,
            character: b"Oslash\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = encoding_table_st {
            code: 234 as libc::c_int,
            character: b"OE\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = encoding_table_st {
            code: 235 as libc::c_int,
            character: b"ordmasculine\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = encoding_table_st {
            code: 241 as libc::c_int,
            character: b"ae\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = encoding_table_st {
            code: 245 as libc::c_int,
            character: b"dotlessi\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = encoding_table_st {
            code: 248 as libc::c_int,
            character: b"lslash\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = encoding_table_st {
            code: 249 as libc::c_int,
            character: b"oslash\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = encoding_table_st {
            code: 250 as libc::c_int,
            character: b"oe\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = encoding_table_st {
            code: 251 as libc::c_int,
            character: b"germandbls\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = encoding_table_st {
            code: -(1 as libc::c_int),
            character: b"Aacute\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = encoding_table_st {
            code: -(1 as libc::c_int),
            character: b"Acircumflex\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = encoding_table_st {
            code: -(1 as libc::c_int),
            character: b"Adieresis\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = encoding_table_st {
            code: -(1 as libc::c_int),
            character: b"Agrave\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = encoding_table_st {
            code: -(1 as libc::c_int),
            character: b"Aring\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = encoding_table_st {
            code: -(1 as libc::c_int),
            character: b"Atilde\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = encoding_table_st {
            code: -(1 as libc::c_int),
            character: b"Ccedilla\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = encoding_table_st {
            code: -(1 as libc::c_int),
            character: b"Eacute\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = encoding_table_st {
            code: -(1 as libc::c_int),
            character: b"Ecircumflex\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = encoding_table_st {
            code: -(1 as libc::c_int),
            character: b"Edieresis\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = encoding_table_st {
            code: -(1 as libc::c_int),
            character: b"Egrave\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = encoding_table_st {
            code: -(1 as libc::c_int),
            character: b"Eth\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = encoding_table_st {
            code: -(1 as libc::c_int),
            character: b"Gcaron\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = encoding_table_st {
            code: -(1 as libc::c_int),
            character: b"IJ\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = encoding_table_st {
            code: -(1 as libc::c_int),
            character: b"Iacute\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = encoding_table_st {
            code: -(1 as libc::c_int),
            character: b"Icircumflex\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = encoding_table_st {
            code: -(1 as libc::c_int),
            character: b"Idieresis\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = encoding_table_st {
            code: -(1 as libc::c_int),
            character: b"Idot\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = encoding_table_st {
            code: -(1 as libc::c_int),
            character: b"Igrave\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = encoding_table_st {
            code: -(1 as libc::c_int),
            character: b"LL\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = encoding_table_st {
            code: -(1 as libc::c_int),
            character: b"Ntilde\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = encoding_table_st {
            code: -(1 as libc::c_int),
            character: b"Oacute\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = encoding_table_st {
            code: -(1 as libc::c_int),
            character: b"Ocircumflex\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = encoding_table_st {
            code: -(1 as libc::c_int),
            character: b"Odieresis\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = encoding_table_st {
            code: -(1 as libc::c_int),
            character: b"Ograve\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = encoding_table_st {
            code: -(1 as libc::c_int),
            character: b"Otilde\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = encoding_table_st {
            code: -(1 as libc::c_int),
            character: b"Scaron\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = encoding_table_st {
            code: -(1 as libc::c_int),
            character: b"Scedilla\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = encoding_table_st {
            code: -(1 as libc::c_int),
            character: b"Thorn\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = encoding_table_st {
            code: -(1 as libc::c_int),
            character: b"Uacute\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = encoding_table_st {
            code: -(1 as libc::c_int),
            character: b"Ucircumflex\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = encoding_table_st {
            code: -(1 as libc::c_int),
            character: b"Udieresis\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = encoding_table_st {
            code: -(1 as libc::c_int),
            character: b"Ugrave\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = encoding_table_st {
            code: -(1 as libc::c_int),
            character: b"Yacute\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = encoding_table_st {
            code: -(1 as libc::c_int),
            character: b"Ydieresis\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = encoding_table_st {
            code: -(1 as libc::c_int),
            character: b"Zcaron\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = encoding_table_st {
            code: -(1 as libc::c_int),
            character: b"aacute\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = encoding_table_st {
            code: -(1 as libc::c_int),
            character: b"acircumflex\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = encoding_table_st {
            code: -(1 as libc::c_int),
            character: b"adieresis\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = encoding_table_st {
            code: -(1 as libc::c_int),
            character: b"agrave\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = encoding_table_st {
            code: -(1 as libc::c_int),
            character: b"aring\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = encoding_table_st {
            code: -(1 as libc::c_int),
            character: b"arrowboth\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = encoding_table_st {
            code: -(1 as libc::c_int),
            character: b"arrowdown\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = encoding_table_st {
            code: -(1 as libc::c_int),
            character: b"arrowleft\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = encoding_table_st {
            code: -(1 as libc::c_int),
            character: b"arrowright\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = encoding_table_st {
            code: -(1 as libc::c_int),
            character: b"arrowup\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = encoding_table_st {
            code: -(1 as libc::c_int),
            character: b"atilde\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = encoding_table_st {
            code: -(1 as libc::c_int),
            character: b"brokenbar\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = encoding_table_st {
            code: -(1 as libc::c_int),
            character: b"ccedilla\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = encoding_table_st {
            code: -(1 as libc::c_int),
            character: b"center\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = encoding_table_st {
            code: -(1 as libc::c_int),
            character: b"copyright\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = encoding_table_st {
            code: -(1 as libc::c_int),
            character: b"dectab\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = encoding_table_st {
            code: -(1 as libc::c_int),
            character: b"degree\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = encoding_table_st {
            code: -(1 as libc::c_int),
            character: b"divide\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = encoding_table_st {
            code: -(1 as libc::c_int),
            character: b"down\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = encoding_table_st {
            code: -(1 as libc::c_int),
            character: b"eacute\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = encoding_table_st {
            code: -(1 as libc::c_int),
            character: b"ecircumflex\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = encoding_table_st {
            code: -(1 as libc::c_int),
            character: b"edieresis\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = encoding_table_st {
            code: -(1 as libc::c_int),
            character: b"egrave\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = encoding_table_st {
            code: -(1 as libc::c_int),
            character: b"eth\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = encoding_table_st {
            code: -(1 as libc::c_int),
            character: b"format\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = encoding_table_st {
            code: -(1 as libc::c_int),
            character: b"gcaron\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = encoding_table_st {
            code: -(1 as libc::c_int),
            character: b"graybox\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = encoding_table_st {
            code: -(1 as libc::c_int),
            character: b"iacute\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = encoding_table_st {
            code: -(1 as libc::c_int),
            character: b"icircumflex\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = encoding_table_st {
            code: -(1 as libc::c_int),
            character: b"idieresis\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = encoding_table_st {
            code: -(1 as libc::c_int),
            character: b"igrave\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = encoding_table_st {
            code: -(1 as libc::c_int),
            character: b"ij\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = encoding_table_st {
            code: -(1 as libc::c_int),
            character: b"indent\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = encoding_table_st {
            code: -(1 as libc::c_int),
            character: b"largebullet\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = encoding_table_st {
            code: -(1 as libc::c_int),
            character: b"left\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = encoding_table_st {
            code: -(1 as libc::c_int),
            character: b"lira\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = encoding_table_st {
            code: -(1 as libc::c_int),
            character: b"ll\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = encoding_table_st {
            code: -(1 as libc::c_int),
            character: b"logicalnot\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = encoding_table_st {
            code: -(1 as libc::c_int),
            character: b"merge\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = encoding_table_st {
            code: -(1 as libc::c_int),
            character: b"minus\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = encoding_table_st {
            code: -(1 as libc::c_int),
            character: b"mu\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = encoding_table_st {
            code: -(1 as libc::c_int),
            character: b"multiply\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = encoding_table_st {
            code: -(1 as libc::c_int),
            character: b"notegraphic\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = encoding_table_st {
            code: -(1 as libc::c_int),
            character: b"ntilde\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = encoding_table_st {
            code: -(1 as libc::c_int),
            character: b"oacute\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = encoding_table_st {
            code: -(1 as libc::c_int),
            character: b"ocircumflex\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = encoding_table_st {
            code: -(1 as libc::c_int),
            character: b"odieresis\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = encoding_table_st {
            code: -(1 as libc::c_int),
            character: b"ograve\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = encoding_table_st {
            code: -(1 as libc::c_int),
            character: b"onehalf\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = encoding_table_st {
            code: -(1 as libc::c_int),
            character: b"onequarter\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = encoding_table_st {
            code: -(1 as libc::c_int),
            character: b"onesuperior\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = encoding_table_st {
            code: -(1 as libc::c_int),
            character: b"otilde\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = encoding_table_st {
            code: -(1 as libc::c_int),
            character: b"overscore\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = encoding_table_st {
            code: -(1 as libc::c_int),
            character: b"plusminus\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = encoding_table_st {
            code: -(1 as libc::c_int),
            character: b"prescription\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = encoding_table_st {
            code: -(1 as libc::c_int),
            character: b"registered\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = encoding_table_st {
            code: -(1 as libc::c_int),
            character: b"return\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = encoding_table_st {
            code: -(1 as libc::c_int),
            character: b"scaron\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = encoding_table_st {
            code: -(1 as libc::c_int),
            character: b"scedilla\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = encoding_table_st {
            code: -(1 as libc::c_int),
            character: b"square\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = encoding_table_st {
            code: -(1 as libc::c_int),
            character: b"stop\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = encoding_table_st {
            code: -(1 as libc::c_int),
            character: b"tab\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = encoding_table_st {
            code: -(1 as libc::c_int),
            character: b"thorn\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = encoding_table_st {
            code: -(1 as libc::c_int),
            character: b"threequarters\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = encoding_table_st {
            code: -(1 as libc::c_int),
            character: b"threesuperior\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = encoding_table_st {
            code: -(1 as libc::c_int),
            character: b"trademark\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = encoding_table_st {
            code: -(1 as libc::c_int),
            character: b"twosuperior\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = encoding_table_st {
            code: -(1 as libc::c_int),
            character: b"uacute\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = encoding_table_st {
            code: -(1 as libc::c_int),
            character: b"ucircumflex\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = encoding_table_st {
            code: -(1 as libc::c_int),
            character: b"udieresis\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = encoding_table_st {
            code: -(1 as libc::c_int),
            character: b"ugrave\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = encoding_table_st {
            code: -(1 as libc::c_int),
            character: b"up\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = encoding_table_st {
            code: -(1 as libc::c_int),
            character: b"yacute\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = encoding_table_st {
            code: -(1 as libc::c_int),
            character: b"ydieresis\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = encoding_table_st {
            code: -(1 as libc::c_int),
            character: b"zcaron\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = encoding_table_st {
            code: 0 as libc::c_int,
            character: 0 as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
];
pub unsafe extern "C" fn afm_open_default_font(
    mut handle: AFMHandle,
    mut font_return: *mut AFMFont,
) -> AFMError {
    let mut current_block: u64;
    let mut font: AFMFont = 0 as *mut afm_font_st;
    let mut cm: *mut AFMIndividualCharacterMetrics = 0
        as *mut AFMIndividualCharacterMetrics;
    let mut i: libc::c_int = 0;
    font = calloc(
        1 as libc::c_int as libc::c_ulong,
        ::std::mem::size_of::<afm_font_st>() as libc::c_ulong,
    ) as AFMFont;
    if !font.is_null() {
        (*font)
            .private = calloc(
            1 as libc::c_int as libc::c_ulong,
            ::std::mem::size_of::<afm_font_private_data_st>() as libc::c_ulong,
        ) as *mut afm_font_private_data_st;
        if !((*font).private).is_null() {
            (*(*font).private).fontnames = strhash_init();
            if !((*(*font).private).fontnames).is_null() {
                (*font).version = 4.0f64;
                (*font)
                    .global_info
                    .FontName = malloc(
                    (strlen(b"Courier\0" as *const u8 as *const libc::c_char))
                        .wrapping_add(1 as libc::c_int as libc::c_ulong),
                ) as *mut libc::c_char;
                if !((*font).global_info.FontName).is_null() {
                    strcpy(
                        (*font).global_info.FontName,
                        b"Courier\0" as *const u8 as *const libc::c_char,
                    );
                    (*font).global_info.FontBBox_llx = -40.0f64;
                    (*font).global_info.FontBBox_lly = -290.0f64;
                    (*font).global_info.FontBBox_urx = 640.0f64;
                    (*font).global_info.FontBBox_ury = 795.0f64;
                    (*font)
                        .writing_direction_metrics[0 as libc::c_int as usize]
                        .is_valid = AFMTrue;
                    (*font)
                        .writing_direction_metrics[0 as libc::c_int as usize]
                        .IsFixedPitch = AFMTrue;
                    (*font)
                        .writing_direction_metrics[0 as libc::c_int as usize]
                        .CharWidth_x = 600.0f64;
                    (*font)
                        .writing_direction_metrics[0 as libc::c_int as usize]
                        .CharWidth_y = 0.0f64;
                    (*font)
                        .num_character_metrics = (::std::mem::size_of::<
                        [AFMEncodingTable; 261],
                    >() as libc::c_ulong)
                        .wrapping_div(
                            ::std::mem::size_of::<AFMEncodingTable>() as libc::c_ulong,
                        )
                        .wrapping_sub(1 as libc::c_int as libc::c_ulong) as AFMInteger;
                    (*font)
                        .character_metrics = calloc(
                        (::std::mem::size_of::<[AFMEncodingTable; 261]>()
                            as libc::c_ulong)
                            .wrapping_div(
                                ::std::mem::size_of::<AFMEncodingTable>() as libc::c_ulong,
                            )
                            .wrapping_sub(1 as libc::c_int as libc::c_ulong),
                        ::std::mem::size_of::<AFMIndividualCharacterMetrics>()
                            as libc::c_ulong,
                    ) as *mut AFMIndividualCharacterMetrics;
                    if !((*font).character_metrics).is_null() {
                        i = 0 as libc::c_int;
                        loop {
                            if (builtin_courier[i as usize].character).is_null() {
                                current_block = 10652014663920648156;
                                break;
                            }
                            cm = &mut *((*font).character_metrics).offset(i as isize)
                                as *mut AFMIndividualCharacterMetrics;
                            (*cm)
                                .name = malloc(
                                (strlen(builtin_courier[i as usize].character))
                                    .wrapping_add(1 as libc::c_int as libc::c_ulong),
                            ) as *mut libc::c_char;
                            if ((*cm).name).is_null() {
                                current_block = 14994023731348187808;
                                break;
                            }
                            strcpy((*cm).name, builtin_courier[i as usize].character);
                            if strhash_put(
                                (*(*font).private).fontnames,
                                (*cm).name,
                                strlen((*cm).name as *const libc::c_char) as libc::c_int,
                                cm as *mut libc::c_void,
                                0 as *mut *mut libc::c_void,
                            ) == 0
                            {
                                current_block = 14994023731348187808;
                                break;
                            }
                            (*cm)
                                .character_code = builtin_courier[i as usize].code
                                as AFMInteger;
                            (*cm).w0x = 600.0f64;
                            (*cm).w0y = 0.0f64;
                            i += 1;
                            i;
                        }
                        match current_block {
                            14994023731348187808 => {}
                            _ => {
                                *font_return = font;
                                return 0 as libc::c_int as AFMError;
                            }
                        }
                    }
                }
            }
        }
    }
    afm_close_font(font);
    return 2 as libc::c_int as AFMError;
}
