use ::libc;
extern "C" {
    pub type _IO_wide_data;
    pub type _IO_codecvt;
    pub type _IO_marker;
    pub type afm_handle_st;
    pub type afm_font_private_data_st;
    static mut stdout: *mut FILE;
    static mut stderr: *mut FILE;
    fn fclose(__stream: *mut FILE) -> libc::c_int;
    fn fopen(_: *const libc::c_char, _: *const libc::c_char) -> *mut FILE;
    fn fprintf(_: *mut FILE, _: *const libc::c_char, _: ...) -> libc::c_int;
    fn printf(_: *const libc::c_char, _: ...) -> libc::c_int;
    fn sprintf(_: *mut libc::c_char, _: *const libc::c_char, _: ...) -> libc::c_int;
    fn perror(__s: *const libc::c_char);
    fn exit(_: libc::c_int) -> !;
    fn strcmp(_: *const libc::c_char, _: *const libc::c_char) -> libc::c_int;
    fn strrchr(_: *const libc::c_char, _: libc::c_int) -> *mut libc::c_char;
    fn strlen(_: *const libc::c_char) -> libc::c_ulong;
    fn setlocale(
        __category: libc::c_int,
        __locale: *const libc::c_char,
    ) -> *mut libc::c_char;
    fn dcgettext(
        __domainname: *const libc::c_char,
        __msgid: *const libc::c_char,
        __category: libc::c_int,
    ) -> *mut libc::c_char;
    fn textdomain(__domainname: *const libc::c_char) -> *mut libc::c_char;
    fn bindtextdomain(
        __domainname: *const libc::c_char,
        __dirname: *const libc::c_char,
    ) -> *mut libc::c_char;
    fn afm_error_to_string(error: AFMError, buf: *mut libc::c_char);
    fn afm_create(
        path: *const libc::c_char,
        verbose_level: libc::c_uint,
        handle_return: *mut AFMHandle,
    ) -> AFMError;
    fn afm_open_file(
        handle: AFMHandle,
        info_level: libc::c_uint,
        filename: *const libc::c_char,
        font_return: *mut AFMFont,
    ) -> AFMError;
    fn afm_close_font(font: AFMFont) -> AFMError;
    static mut optarg: *mut libc::c_char;
    static mut optind: libc::c_int;
    fn getopt_long(
        argc: libc::c_int,
        argv: *const *mut libc::c_char,
        shortopts: *const libc::c_char,
        longopts: *const option,
        longind: *mut libc::c_int,
    ) -> libc::c_int;
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
pub type AFMFont = *mut afm_font_st;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct option {
    pub name: *const libc::c_char,
    pub has_arg: libc::c_int,
    pub flag: *mut libc::c_int,
    pub val: libc::c_int,
}
static mut fname: *mut libc::c_char = b"font.map\0" as *const u8 as *const libc::c_char
    as *mut libc::c_char;
static mut program: *mut libc::c_char = 0 as *const libc::c_char as *mut libc::c_char;
static mut long_options: [option; 4] = [
    {
        let mut init = option {
            name: b"output-file\0" as *const u8 as *const libc::c_char,
            has_arg: 1 as libc::c_int,
            flag: 0 as *const libc::c_int as *mut libc::c_int,
            val: 'p' as i32,
        };
        init
    },
    {
        let mut init = option {
            name: b"help\0" as *const u8 as *const libc::c_char,
            has_arg: 0 as libc::c_int,
            flag: 0 as *const libc::c_int as *mut libc::c_int,
            val: 'h' as i32,
        };
        init
    },
    {
        let mut init = option {
            name: b"version\0" as *const u8 as *const libc::c_char,
            has_arg: 0 as libc::c_int,
            flag: 0 as *const libc::c_int as *mut libc::c_int,
            val: 'V' as i32,
        };
        init
    },
    {
        let mut init = option {
            name: 0 as *const libc::c_char,
            has_arg: 0 as libc::c_int,
            flag: 0 as *const libc::c_int as *mut libc::c_int,
            val: 0 as libc::c_int,
        };
        init
    },
];
unsafe fn main_0(
    mut argc: libc::c_int,
    mut argv: *mut *mut libc::c_char,
) -> libc::c_int {
    let mut error: AFMError = 0;
    let mut afm: AFMHandle = 0 as *mut afm_handle_st;
    let mut font: AFMFont = 0 as *mut afm_font_st;
    let mut i: libc::c_int = 0;
    let mut ofp: *mut FILE = 0 as *mut FILE;
    let mut mfp: *mut FILE = 0 as *mut FILE;
    program = strrchr(*argv.offset(0 as libc::c_int as isize), '/' as i32);
    if program.is_null() {
        program = *argv.offset(0 as libc::c_int as isize);
    } else {
        program = program.offset(1);
        program;
    }
    let ref mut fresh0 = *argv.offset(0 as libc::c_int as isize);
    *fresh0 = program;
    setlocale(5 as libc::c_int, b"\0" as *const u8 as *const libc::c_char);
    bindtextdomain(
        b"enscript\0" as *const u8 as *const libc::c_char,
        b"/usr/local/share/locale\0" as *const u8 as *const libc::c_char,
    );
    textdomain(b"enscript\0" as *const u8 as *const libc::c_char);
    loop {
        let mut option_index: libc::c_int = 0 as libc::c_int;
        let mut c: libc::c_int = 0;
        c = getopt_long(
            argc,
            argv as *const *mut libc::c_char,
            b"p:h\0" as *const u8 as *const libc::c_char,
            long_options.as_mut_ptr(),
            &mut option_index,
        );
        if c == -(1 as libc::c_int) {
            break;
        }
        match c {
            104 => {
                usage();
                exit(0 as libc::c_int);
            }
            112 => {
                if strcmp(optarg, b"-\0" as *const u8 as *const libc::c_char)
                    == 0 as libc::c_int
                {
                    fname = 0 as *mut libc::c_char;
                } else {
                    fname = optarg;
                }
            }
            86 => {
                printf(
                    b"%s for GNU %s %s\n\0" as *const u8 as *const libc::c_char,
                    program,
                    b"enscript\0" as *const u8 as *const libc::c_char,
                    b"1.6.6\0" as *const u8 as *const libc::c_char,
                );
                exit(0 as libc::c_int);
            }
            63 => {
                usage();
                exit(1 as libc::c_int);
            }
            _ => {}
        }
    }
    printf(
        dcgettext(
            0 as *const libc::c_char,
            b"file=%s\n\0" as *const u8 as *const libc::c_char,
            5 as libc::c_int,
        ),
        if !fname.is_null() {
            fname
        } else {
            dcgettext(
                0 as *const libc::c_char,
                b"stdout\0" as *const u8 as *const libc::c_char,
                5 as libc::c_int,
            )
        },
    );
    if !fname.is_null() {
        ofp = fopen(fname, b"w\0" as *const u8 as *const libc::c_char);
        if ofp.is_null() {
            let mut buf: [libc::c_char; 256] = [0; 256];
            sprintf(
                buf.as_mut_ptr(),
                dcgettext(
                    0 as *const libc::c_char,
                    b"%s: couldn't open output file \"%s\"\0" as *const u8
                        as *const libc::c_char,
                    5 as libc::c_int,
                ),
                program,
                fname,
            );
            perror(buf.as_mut_ptr());
            exit(1 as libc::c_int);
        }
        mfp = stdout;
    } else {
        ofp = stdout;
        mfp = stderr;
    }
    error = afm_create(
        0 as *const libc::c_char,
        0 as libc::c_int as libc::c_uint,
        &mut afm,
    );
    if error != 0 as libc::c_int as libc::c_uint {
        let mut buf_0: [libc::c_char; 256] = [0; 256];
        afm_error_to_string(error, buf_0.as_mut_ptr());
        fprintf(
            stderr,
            b"%s: %s: %s\n\0" as *const u8 as *const libc::c_char,
            program,
            dcgettext(
                0 as *const libc::c_char,
                b"couldn't create AFM library\0" as *const u8 as *const libc::c_char,
                5 as libc::c_int,
            ),
            buf_0.as_mut_ptr(),
        );
        exit(1 as libc::c_int);
    }
    i = optind;
    while i < argc {
        fprintf(
            mfp,
            b"%s...\n\0" as *const u8 as *const libc::c_char,
            *argv.offset(i as isize),
        );
        error = afm_open_file(
            afm,
            0 as libc::c_int as libc::c_uint,
            *argv.offset(i as isize),
            &mut font,
        );
        if error == 0 as libc::c_int as libc::c_uint {
            let mut cp: *mut libc::c_char = 0 as *mut libc::c_char;
            let mut sf: *mut libc::c_char = 0 as *mut libc::c_char;
            let mut len: libc::c_int = 0;
            cp = strrchr(*argv.offset(i as isize), '/' as i32);
            if cp.is_null() {
                cp = *argv.offset(i as isize);
            } else {
                cp = cp.offset(1);
                cp;
            }
            sf = strrchr(*argv.offset(i as isize), '.' as i32);
            if !sf.is_null() {
                len = sf.offset_from(cp) as libc::c_long as libc::c_int;
            } else {
                len = strlen(cp) as libc::c_int;
            }
            fprintf(
                ofp,
                b"%-30s\t%.*s\n\0" as *const u8 as *const libc::c_char,
                (*font).global_info.FontName,
                len,
                cp,
            );
            afm_close_font(font);
        } else {
            let mut buf_1: [libc::c_char; 256] = [0; 256];
            afm_error_to_string(error, buf_1.as_mut_ptr());
            fprintf(
                mfp,
                b"%s: %s\n\0" as *const u8 as *const libc::c_char,
                program,
                buf_1.as_mut_ptr(),
            );
        }
        i += 1;
        i;
    }
    if !fname.is_null() {
        fclose(ofp);
    }
    return 0 as libc::c_int;
}
unsafe extern "C" fn usage() {
    printf(
        dcgettext(
            0 as *const libc::c_char,
            b"Usage: %s [OPTION]... FILE...\nMandatory arguments to long options are mandatory for short options too.\n  -h, --help              print this help and exit\n  -p, --output-file=NAME  print output to file NAME (default file is\n                          font.map).  If FILE is `-', leavy output to\n                          stdout.\n  -V, --version           print version number\n\0"
                as *const u8 as *const libc::c_char,
            5 as libc::c_int,
        ),
        program,
    );
}
pub fn main() {
    let mut args: Vec::<*mut libc::c_char> = Vec::new();
    for arg in ::std::env::args_os() {
        args.push(
            (::std::ffi::CString::new(
                ::std::os::unix::ffi::OsStrExt::as_bytes(arg.as_os_str()),
            ))
                .unwrap()
                .into_raw(),
        );
    }
    args.push(::std::ptr::null_mut());
    unsafe {
        ::std::process::exit(
            main_0(
                (args.len() - 1) as libc::c_int,
                args.as_mut_ptr() as *mut *mut libc::c_char,
            ) as i32,
        )
    }
}
