use ::libc;
extern "C" {
    fn memcpy(
        _: *mut libc::c_void,
        _: *const libc::c_void,
        _: libc::c_ulong,
    ) -> *mut libc::c_void;
    fn memcmp(
        _: *const libc::c_void,
        _: *const libc::c_void,
        _: libc::c_ulong,
    ) -> libc::c_int;
    fn strncmp(
        _: *const libc::c_char,
        _: *const libc::c_char,
        _: libc::c_ulong,
    ) -> libc::c_int;
    fn strrchr(_: *const libc::c_char, _: libc::c_int) -> *mut libc::c_char;
    fn strlen(_: *const libc::c_char) -> libc::c_ulong;
    fn xmalloc(s: size_t) -> *mut libc::c_void;
    static mut use_compress_program_option: *const libc::c_char;
}
pub type size_t = libc::c_ulong;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct compression_suffix {
    pub suffix: *const libc::c_char,
    pub length: size_t,
    pub program: *const libc::c_char,
}
static mut compression_suffixes: [compression_suffix; 19] = [compression_suffix {
    suffix: 0 as *const libc::c_char,
    length: 0,
    program: 0 as *const libc::c_char,
}; 19];
unsafe extern "C" fn find_compression_suffix(
    mut name: *const libc::c_char,
    mut ret_len: *mut size_t,
) -> *const compression_suffix {
    let mut suf: *mut libc::c_char = strrchr(name, '.' as i32);
    if !suf.is_null() {
        let mut len: size_t = 0;
        let mut p: *mut compression_suffix = 0 as *mut compression_suffix;
        suf = suf.offset(1);
        suf;
        len = strlen(suf);
        p = compression_suffixes.as_mut_ptr();
        while !((*p).suffix).is_null() {
            if (*p).length == len
                && memcmp(
                    (*p).suffix as *const libc::c_void,
                    suf as *const libc::c_void,
                    len,
                ) == 0 as libc::c_int
            {
                if !ret_len.is_null() {
                    *ret_len = (strlen(name))
                        .wrapping_sub(len)
                        .wrapping_sub(1 as libc::c_int as libc::c_ulong);
                }
                return p;
            }
            p = p.offset(1);
            p;
        }
    }
    return 0 as *const compression_suffix;
}
unsafe extern "C" fn find_compression_program(
    mut name: *const libc::c_char,
    mut defprog: *const libc::c_char,
) -> *const libc::c_char {
    let mut p: *const compression_suffix = find_compression_suffix(
        name,
        0 as *mut size_t,
    );
    if !p.is_null() {
        return (*p).program;
    }
    return defprog;
}
pub unsafe extern "C" fn set_compression_program_by_suffix(
    mut name: *const libc::c_char,
    mut defprog: *const libc::c_char,
) {
    let mut program: *const libc::c_char = find_compression_program(name, defprog);
    if !program.is_null() {
        use_compress_program_option = program;
    }
}
pub unsafe extern "C" fn strip_compression_suffix(
    mut name: *const libc::c_char,
) -> *mut libc::c_char {
    let mut s: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut len: size_t = 0;
    let mut p: *const compression_suffix = find_compression_suffix(name, &mut len);
    if !p.is_null() {
        if len > 4 as libc::c_int as libc::c_ulong
            && strncmp(
                name.offset(len as isize).offset(-(4 as libc::c_int as isize)),
                b".tar\0" as *const u8 as *const libc::c_char,
                4 as libc::c_int as libc::c_ulong,
            ) == 0 as libc::c_int
            && *((*p).suffix).offset(0 as libc::c_int as isize) as libc::c_int
                != 't' as i32
        {
            len = (len as libc::c_ulong).wrapping_sub(4 as libc::c_int as libc::c_ulong)
                as size_t as size_t;
        }
        if len == 0 as libc::c_int as libc::c_ulong {
            return 0 as *mut libc::c_char;
        }
        s = xmalloc(len.wrapping_add(1 as libc::c_int as libc::c_ulong))
            as *mut libc::c_char;
        memcpy(s as *mut libc::c_void, name as *const libc::c_void, len);
        *s.offset(len as isize) = 0 as libc::c_int as libc::c_char;
    }
    return s;
}
unsafe extern "C" fn run_static_initializers() {
    compression_suffixes = [
        {
            let mut init = compression_suffix {
                suffix: b"tar\0" as *const u8 as *const libc::c_char,
                length: 3 as libc::c_int as size_t,
                program: 0 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = compression_suffix {
                suffix: b"gz\0" as *const u8 as *const libc::c_char,
                length: (::std::mem::size_of::<[libc::c_char; 3]>() as libc::c_ulong)
                    .wrapping_sub(1 as libc::c_int as libc::c_ulong),
                program: b"gzip\0" as *const u8 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = compression_suffix {
                suffix: b"tgz\0" as *const u8 as *const libc::c_char,
                length: (::std::mem::size_of::<[libc::c_char; 4]>() as libc::c_ulong)
                    .wrapping_sub(1 as libc::c_int as libc::c_ulong),
                program: b"gzip\0" as *const u8 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = compression_suffix {
                suffix: b"taz\0" as *const u8 as *const libc::c_char,
                length: (::std::mem::size_of::<[libc::c_char; 4]>() as libc::c_ulong)
                    .wrapping_sub(1 as libc::c_int as libc::c_ulong),
                program: b"gzip\0" as *const u8 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = compression_suffix {
                suffix: b"Z\0" as *const u8 as *const libc::c_char,
                length: (::std::mem::size_of::<[libc::c_char; 2]>() as libc::c_ulong)
                    .wrapping_sub(1 as libc::c_int as libc::c_ulong),
                program: b"compress\0" as *const u8 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = compression_suffix {
                suffix: b"taZ\0" as *const u8 as *const libc::c_char,
                length: (::std::mem::size_of::<[libc::c_char; 4]>() as libc::c_ulong)
                    .wrapping_sub(1 as libc::c_int as libc::c_ulong),
                program: b"compress\0" as *const u8 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = compression_suffix {
                suffix: b"bz2\0" as *const u8 as *const libc::c_char,
                length: (::std::mem::size_of::<[libc::c_char; 4]>() as libc::c_ulong)
                    .wrapping_sub(1 as libc::c_int as libc::c_ulong),
                program: b"bzip2\0" as *const u8 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = compression_suffix {
                suffix: b"tbz\0" as *const u8 as *const libc::c_char,
                length: (::std::mem::size_of::<[libc::c_char; 4]>() as libc::c_ulong)
                    .wrapping_sub(1 as libc::c_int as libc::c_ulong),
                program: b"bzip2\0" as *const u8 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = compression_suffix {
                suffix: b"tbz2\0" as *const u8 as *const libc::c_char,
                length: (::std::mem::size_of::<[libc::c_char; 5]>() as libc::c_ulong)
                    .wrapping_sub(1 as libc::c_int as libc::c_ulong),
                program: b"bzip2\0" as *const u8 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = compression_suffix {
                suffix: b"tz2\0" as *const u8 as *const libc::c_char,
                length: (::std::mem::size_of::<[libc::c_char; 4]>() as libc::c_ulong)
                    .wrapping_sub(1 as libc::c_int as libc::c_ulong),
                program: b"bzip2\0" as *const u8 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = compression_suffix {
                suffix: b"lz\0" as *const u8 as *const libc::c_char,
                length: (::std::mem::size_of::<[libc::c_char; 3]>() as libc::c_ulong)
                    .wrapping_sub(1 as libc::c_int as libc::c_ulong),
                program: b"lzip\0" as *const u8 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = compression_suffix {
                suffix: b"lzma\0" as *const u8 as *const libc::c_char,
                length: (::std::mem::size_of::<[libc::c_char; 5]>() as libc::c_ulong)
                    .wrapping_sub(1 as libc::c_int as libc::c_ulong),
                program: b"lzma\0" as *const u8 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = compression_suffix {
                suffix: b"tlz\0" as *const u8 as *const libc::c_char,
                length: (::std::mem::size_of::<[libc::c_char; 4]>() as libc::c_ulong)
                    .wrapping_sub(1 as libc::c_int as libc::c_ulong),
                program: b"lzma\0" as *const u8 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = compression_suffix {
                suffix: b"lzo\0" as *const u8 as *const libc::c_char,
                length: (::std::mem::size_of::<[libc::c_char; 4]>() as libc::c_ulong)
                    .wrapping_sub(1 as libc::c_int as libc::c_ulong),
                program: b"lzop\0" as *const u8 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = compression_suffix {
                suffix: b"xz\0" as *const u8 as *const libc::c_char,
                length: (::std::mem::size_of::<[libc::c_char; 3]>() as libc::c_ulong)
                    .wrapping_sub(1 as libc::c_int as libc::c_ulong),
                program: b"xz\0" as *const u8 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = compression_suffix {
                suffix: b"txz\0" as *const u8 as *const libc::c_char,
                length: (::std::mem::size_of::<[libc::c_char; 4]>() as libc::c_ulong)
                    .wrapping_sub(1 as libc::c_int as libc::c_ulong),
                program: b"xz\0" as *const u8 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = compression_suffix {
                suffix: b"zst\0" as *const u8 as *const libc::c_char,
                length: (::std::mem::size_of::<[libc::c_char; 4]>() as libc::c_ulong)
                    .wrapping_sub(1 as libc::c_int as libc::c_ulong),
                program: b"zstd\0" as *const u8 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = compression_suffix {
                suffix: b"tzst\0" as *const u8 as *const libc::c_char,
                length: (::std::mem::size_of::<[libc::c_char; 5]>() as libc::c_ulong)
                    .wrapping_sub(1 as libc::c_int as libc::c_ulong),
                program: b"zstd\0" as *const u8 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = compression_suffix {
                suffix: 0 as *const libc::c_char,
                length: 0,
                program: 0 as *const libc::c_char,
            };
            init
        },
    ];
}
#[used]
#[cfg_attr(target_os = "linux", link_section = ".init_array")]
#[cfg_attr(target_os = "windows", link_section = ".CRT$XIB")]
#[cfg_attr(target_os = "macos", link_section = "__DATA,__mod_init_func")]
static INIT_ARRAY: [unsafe extern "C" fn(); 1] = [run_static_initializers];
