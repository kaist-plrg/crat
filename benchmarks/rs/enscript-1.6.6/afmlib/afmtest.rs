use ::libc;
extern "C" {
    pub type _IO_wide_data;
    pub type _IO_codecvt;
    pub type _IO_marker;
    pub type afm_handle_st;
    pub type afm_font_private_data_st;
    static mut stdout: *mut FILE;
    static mut stderr: *mut FILE;
    fn fprintf(_: *mut FILE, _: *const libc::c_char, _: ...) -> libc::c_int;
    fn printf(_: *const libc::c_char, _: ...) -> libc::c_int;
    fn strtod(_: *const libc::c_char, _: *mut *mut libc::c_char) -> libc::c_double;
    fn exit(_: libc::c_int) -> !;
    fn strcmp(_: *const libc::c_char, _: *const libc::c_char) -> libc::c_int;
    fn strrchr(_: *const libc::c_char, _: libc::c_int) -> *mut libc::c_char;
    fn strlen(_: *const libc::c_char) -> libc::c_ulong;
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
    fn afm_open_default_font(handle: AFMHandle, font_return: *mut AFMFont) -> AFMError;
    fn afm_close_font(font: AFMFont) -> AFMError;
    fn afm_font_dump(fp: *mut FILE, font: AFMFont);
    fn afm_font_stringwidth(
        font: AFMFont,
        ptsize: AFMNumber,
        string: *mut libc::c_char,
        stringlen: libc::c_uint,
        w0x_return: *mut AFMNumber,
        w0y_return: *mut AFMNumber,
    ) -> AFMError;
    fn afm_font_encoding(
        font: AFMFont,
        enc: AFMEncoding,
        flags: libc::c_uint,
    ) -> AFMError;
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
#[inline]
unsafe extern "C" fn atof(mut __nptr: *const libc::c_char) -> libc::c_double {
    return strtod(__nptr, 0 as *mut libc::c_void as *mut *mut libc::c_char);
}
static mut program: *mut libc::c_char = 0 as *const libc::c_char as *mut libc::c_char;
unsafe fn main_0(
    mut argc: libc::c_int,
    mut argv: *mut *mut libc::c_char,
) -> libc::c_int {
    let mut afm: AFMHandle = 0 as *mut afm_handle_st;
    let mut font: AFMFont = 0 as *mut afm_font_st;
    let mut error: AFMError = 0;
    let mut width: AFMNumber = 0.;
    let mut height: AFMNumber = 0.;
    let mut buf: [libc::c_char; 256] = [0; 256];
    program = strrchr(*argv.offset(0 as libc::c_int as isize), '/' as i32);
    if !program.is_null() {
        program = program.offset(1);
        program;
    } else {
        program = *argv.offset(0 as libc::c_int as isize);
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
            b"afmtest: %s: %s\n\0" as *const u8 as *const libc::c_char,
            b"couldn't create library\0" as *const u8 as *const libc::c_char,
            buf_0.as_mut_ptr(),
        );
        exit(1 as libc::c_int);
    }
    if argc < 2 as libc::c_int {
        usage();
        exit(1 as libc::c_int);
    }
    if strcmp(
        *argv.offset(1 as libc::c_int as isize),
        b"dump\0" as *const u8 as *const libc::c_char,
    ) == 0 as libc::c_int && argc == 3 as libc::c_int
    {
        error = afm_open_file(
            afm,
            0xffffffff as libc::c_uint,
            *argv.offset(2 as libc::c_int as isize),
            &mut font,
        );
        if error != 0 as libc::c_int as libc::c_uint {
            fprintf(
                stderr,
                b"%s: couldn't open font \"%s\", using default\n\0" as *const u8
                    as *const libc::c_char,
                program,
                *argv.offset(2 as libc::c_int as isize),
            );
            error = afm_open_default_font(afm, &mut font);
            if error != 0 as libc::c_int as libc::c_uint {
                let mut buf_1: [libc::c_char; 256] = [0; 256];
                afm_error_to_string(error, buf_1.as_mut_ptr());
                fprintf(
                    stderr,
                    b"afmtest: %s: %s\n\0" as *const u8 as *const libc::c_char,
                    b"couldn't open default font\0" as *const u8 as *const libc::c_char,
                    buf_1.as_mut_ptr(),
                );
                exit(1 as libc::c_int);
            }
        }
        afm_font_dump(stdout, font);
        error = afm_close_font(font);
        if error != 0 as libc::c_int as libc::c_uint {
            let mut buf_2: [libc::c_char; 256] = [0; 256];
            afm_error_to_string(error, buf_2.as_mut_ptr());
            fprintf(
                stderr,
                b"afmtest: %s: %s\n\0" as *const u8 as *const libc::c_char,
                b"couldn't close font\0" as *const u8 as *const libc::c_char,
                buf_2.as_mut_ptr(),
            );
            exit(1 as libc::c_int);
        }
    } else if strcmp(
        *argv.offset(1 as libc::c_int as isize),
        b"stringwidth\0" as *const u8 as *const libc::c_char,
    ) == 0 as libc::c_int && argc == 5 as libc::c_int
    {
        error = afm_open_file(
            afm,
            0xffffffff as libc::c_uint,
            *argv.offset(2 as libc::c_int as isize),
            &mut font,
        );
        if error != 0 as libc::c_int as libc::c_uint {
            let mut buf_3: [libc::c_char; 256] = [0; 256];
            afm_error_to_string(error, buf_3.as_mut_ptr());
            fprintf(
                stderr,
                b"afmtest: %s: %s\n\0" as *const u8 as *const libc::c_char,
                b"couldn't open font\0" as *const u8 as *const libc::c_char,
                buf_3.as_mut_ptr(),
            );
            exit(1 as libc::c_int);
        }
        error = afm_font_encoding(
            font,
            AFM_ENCODING_ISO_8859_1,
            0 as libc::c_int as libc::c_uint,
        );
        if error != 0 as libc::c_int as libc::c_uint {
            let mut buf_4: [libc::c_char; 256] = [0; 256];
            afm_error_to_string(error, buf_4.as_mut_ptr());
            fprintf(
                stderr,
                b"afmtest: %s: %s\n\0" as *const u8 as *const libc::c_char,
                b"couldn't encode font\0" as *const u8 as *const libc::c_char,
                buf_4.as_mut_ptr(),
            );
            exit(1 as libc::c_int);
        }
        error = afm_font_stringwidth(
            font,
            atof(*argv.offset(3 as libc::c_int as isize)),
            *argv.offset(4 as libc::c_int as isize),
            strlen(*argv.offset(4 as libc::c_int as isize)) as libc::c_uint,
            &mut width,
            &mut height,
        );
        printf(
            b"stringwidth is [%g %g]\n\0" as *const u8 as *const libc::c_char,
            width,
            height,
        );
        error = afm_close_font(font);
        if error != 0 as libc::c_int as libc::c_uint {
            let mut buf_5: [libc::c_char; 256] = [0; 256];
            afm_error_to_string(error, buf_5.as_mut_ptr());
            fprintf(
                stderr,
                b"afmtest: %s: %s\n\0" as *const u8 as *const libc::c_char,
                b"couldn't close font\0" as *const u8 as *const libc::c_char,
                buf_5.as_mut_ptr(),
            );
            exit(1 as libc::c_int);
        }
    } else if strcmp(
        *argv.offset(1 as libc::c_int as isize),
        b"chardump\0" as *const u8 as *const libc::c_char,
    ) == 0 as libc::c_int && argc > 2 as libc::c_int
    {
        let mut i: libc::c_int = 0;
        let mut j: libc::c_int = 0;
        i = 2 as libc::c_int;
        while i < argc {
            error = afm_open_file(
                afm,
                0x1 as libc::c_int as libc::c_uint,
                *argv.offset(i as isize),
                &mut font,
            );
            if error != 0 as libc::c_int as libc::c_uint {
                afm_error_to_string(error, buf.as_mut_ptr());
                fprintf(
                    stderr,
                    b"%s: couldn't open AFM file \"%s\": %s\n\0" as *const u8
                        as *const libc::c_char,
                    program,
                    *argv.offset(i as isize),
                    buf.as_mut_ptr(),
                );
            } else {
                j = 0 as libc::c_int;
                while (j as libc::c_long) < (*font).num_character_metrics {
                    let mut cm: *mut AFMIndividualCharacterMetrics = 0
                        as *mut AFMIndividualCharacterMetrics;
                    cm = &mut *((*font).character_metrics).offset(j as isize)
                        as *mut AFMIndividualCharacterMetrics;
                    printf(
                        b"/%-30s %3ld glyph %s\n\0" as *const u8 as *const libc::c_char,
                        (*cm).name,
                        (*cm).character_code,
                        (*font).global_info.FontName,
                    );
                    j += 1;
                    j;
                }
                j = 0 as libc::c_int;
                while (j as libc::c_long) < (*font).num_composites {
                    let mut cc: *mut AFMComposite = 0 as *mut AFMComposite;
                    cc = &mut *((*font).composites).offset(j as isize)
                        as *mut AFMComposite;
                    printf(
                        b"/%-30s -1 composite %s\n\0" as *const u8
                            as *const libc::c_char,
                        (*cc).name,
                        (*font).global_info.FontName,
                    );
                    j += 1;
                    j;
                }
                afm_close_font(font);
            }
            i += 1;
            i;
        }
    } else {
        usage();
        exit(1 as libc::c_int);
    }
    return 0 as libc::c_int;
}
unsafe extern "C" fn usage() {
    fprintf(
        stderr,
        b"Usage: %s dump file\n       %s stringwidth file ptsize string\n       %s chardump file [file ...]\n\0"
            as *const u8 as *const libc::c_char,
        program,
        program,
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
