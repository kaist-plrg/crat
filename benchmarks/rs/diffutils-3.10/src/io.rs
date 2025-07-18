use ::libc;
use ::c2rust_bitfields;
extern "C" {
    pub type re_dfa_t;
    fn lseek(__fd: libc::c_int, __offset: __off_t, __whence: libc::c_int) -> __off_t;
    fn rpl_free(ptr: *mut libc::c_void);
    fn memset(
        _: *mut libc::c_void,
        _: libc::c_int,
        _: libc::c_ulong,
    ) -> *mut libc::c_void;
    fn memcmp(
        _: *const libc::c_void,
        _: *const libc::c_void,
        _: libc::c_ulong,
    ) -> libc::c_int;
    fn memchr(
        _: *const libc::c_void,
        _: libc::c_int,
        _: libc::c_ulong,
    ) -> *mut libc::c_void;
    fn rawmemchr(__s: *const libc::c_void, __c: libc::c_int) -> *mut libc::c_void;
    fn __ctype_b_loc() -> *mut *const libc::c_ushort;
    fn __ctype_tolower_loc() -> *mut *const __int32_t;
    static mut output_style: output_style;
    static mut no_diff_means_no_output: bool;
    static mut context: lin;
    static mut text: bool;
    static mut horizon_lines: lin;
    static mut ignore_white_space: DIFF_white_space;
    static mut ignore_case: bool;
    static mut function_regexp: re_pattern_buffer;
    static mut tabsize: size_t;
    static mut strip_trailing_cr: bool;
    fn pfatal_with_name(_: *const libc::c_char);
    fn lines_differ(_: *const libc::c_char, _: *const libc::c_char) -> bool;
    fn block_read(_: libc::c_int, _: *mut libc::c_char, _: size_t) -> size_t;
    fn buffer_lcm(_: size_t, _: size_t, _: size_t) -> size_t;
    fn xalloc_die();
    fn xmalloc(s: size_t) -> *mut libc::c_void;
    fn xcalloc(n: size_t, s: size_t) -> *mut libc::c_void;
    fn xrealloc(p: *mut libc::c_void, s: size_t) -> *mut libc::c_void;
    fn xnmalloc(n: size_t, s: size_t) -> *mut libc::c_void;
}
pub type __int32_t = libc::c_int;
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
pub type off_t = __off_t;
pub type size_t = libc::c_ulong;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct timespec {
    pub tv_sec: __time_t,
    pub tv_nsec: __syscall_slong_t,
}
pub type ptrdiff_t = libc::c_long;
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
pub type lin = ptrdiff_t;
pub type __re_long_size_t = size_t;
pub type reg_syntax_t = libc::c_ulong;
#[derive(Copy, Clone, BitfieldStruct)]
#[repr(C)]
pub struct re_pattern_buffer {
    pub buffer: *mut re_dfa_t,
    pub allocated: __re_long_size_t,
    pub used: __re_long_size_t,
    pub syntax: reg_syntax_t,
    pub fastmap: *mut libc::c_char,
    pub translate: *mut libc::c_uchar,
    pub re_nsub: size_t,
    #[bitfield(name = "can_be_null", ty = "libc::c_uint", bits = "0..=0")]
    #[bitfield(name = "regs_allocated", ty = "libc::c_uint", bits = "1..=2")]
    #[bitfield(name = "fastmap_accurate", ty = "libc::c_uint", bits = "3..=3")]
    #[bitfield(name = "no_sub", ty = "libc::c_uint", bits = "4..=4")]
    #[bitfield(name = "not_bol", ty = "libc::c_uint", bits = "5..=5")]
    #[bitfield(name = "not_eol", ty = "libc::c_uint", bits = "6..=6")]
    #[bitfield(name = "newline_anchor", ty = "libc::c_uint", bits = "7..=7")]
    pub can_be_null_regs_allocated_fastmap_accurate_no_sub_not_bol_not_eol_newline_anchor: [u8; 1],
    #[bitfield(padding)]
    pub c2rust_padding: [u8; 7],
}
pub type output_style = libc::c_uint;
pub const OUTPUT_SDIFF: output_style = 8;
pub const OUTPUT_IFDEF: output_style = 7;
pub const OUTPUT_RCS: output_style = 6;
pub const OUTPUT_FORWARD_ED: output_style = 5;
pub const OUTPUT_ED: output_style = 4;
pub const OUTPUT_UNIFIED: output_style = 3;
pub const OUTPUT_CONTEXT: output_style = 2;
pub const OUTPUT_NORMAL: output_style = 1;
pub const OUTPUT_UNSPECIFIED: output_style = 0;
pub type DIFF_white_space = libc::c_uint;
pub const IGNORE_ALL_SPACE: DIFF_white_space = 5;
pub const IGNORE_SPACE_CHANGE: DIFF_white_space = 4;
pub const IGNORE_TAB_EXPANSION_AND_TRAILING_SPACE: DIFF_white_space = 3;
pub const IGNORE_TRAILING_SPACE: DIFF_white_space = 2;
pub const IGNORE_TAB_EXPANSION: DIFF_white_space = 1;
pub const IGNORE_NO_WHITE_SPACE: DIFF_white_space = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct file_data {
    pub desc: libc::c_int,
    pub name: *const libc::c_char,
    pub stat: stat,
    pub buffer: *mut size_t,
    pub bufsize: size_t,
    pub buffered: size_t,
    pub linbuf: *mut *const libc::c_char,
    pub linbuf_base: lin,
    pub buffered_lines: lin,
    pub valid_lines: lin,
    pub alloc_lines: lin,
    pub prefix_end: *const libc::c_char,
    pub prefix_lines: lin,
    pub suffix_begin: *const libc::c_char,
    pub equivs: *mut lin,
    pub undiscarded: *mut lin,
    pub realindexes: *mut lin,
    pub nondiscarded_lines: lin,
    pub changed: *mut libc::c_char,
    pub missing_newline: bool,
    pub eof: bool,
    pub equiv_max: lin,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct equivclass {
    pub next: lin,
    pub hash: hash_value,
    pub line: *const libc::c_char,
    pub length: size_t,
}
pub type hash_value = size_t;
#[inline]
unsafe extern "C" fn tolower(mut __c: libc::c_int) -> libc::c_int {
    return if __c >= -(128 as libc::c_int) && __c < 256 as libc::c_int {
        *(*__ctype_tolower_loc()).offset(__c as isize)
    } else {
        __c
    };
}
#[inline]
unsafe extern "C" fn robust_output_style(mut s: output_style) -> bool {
    return s as libc::c_uint != OUTPUT_ED as libc::c_int as libc::c_uint
        && s as libc::c_uint != OUTPUT_FORWARD_ED as libc::c_int as libc::c_uint;
}
#[inline]
unsafe extern "C" fn __gl_setmode(
    mut fd: libc::c_int,
    mut mode: libc::c_int,
) -> libc::c_int {
    return 0 as libc::c_int;
}
#[inline]
unsafe extern "C" fn set_binary_mode(
    mut fd: libc::c_int,
    mut mode: libc::c_int,
) -> libc::c_int {
    return __gl_setmode(fd, mode);
}
static mut buckets: *mut lin = 0 as *const lin as *mut lin;
static mut nbuckets: size_t = 0;
static mut equivs: *mut equivclass = 0 as *const equivclass as *mut equivclass;
static mut equivs_index: lin = 0;
static mut equivs_alloc: lin = 0;
unsafe extern "C" fn file_buffer(mut f: *const file_data) -> *mut libc::c_char {
    return (*f).buffer as *mut libc::c_char;
}
pub unsafe extern "C" fn file_block_read(mut current: *mut file_data, mut size: size_t) {
    if size != 0 && !(*current).eof {
        let mut s: size_t = block_read(
            (*current).desc,
            (file_buffer(current)).offset((*current).buffered as isize),
            size,
        );
        if s == 18446744073709551615 as libc::c_ulong {
            pfatal_with_name((*current).name);
        }
        (*current)
            .buffered = ((*current).buffered as libc::c_ulong).wrapping_add(s) as size_t
            as size_t;
        (*current).eof = s < size;
    }
}
unsafe extern "C" fn sip(mut current: *mut file_data, mut skip_test: bool) -> bool {
    if (*current).desc < 0 as libc::c_int {
        (*current).bufsize = ::std::mem::size_of::<size_t>() as libc::c_ulong;
        (*current).buffer = xmalloc((*current).bufsize) as *mut size_t;
    } else {
        (*current)
            .bufsize = buffer_lcm(
            ::std::mem::size_of::<size_t>() as libc::c_ulong,
            (*current).stat.st_blksize as size_t,
            (9223372036854775807 as libc::c_long as libc::c_ulong)
                .wrapping_sub(
                    (2 as libc::c_int as libc::c_ulong)
                        .wrapping_mul(::std::mem::size_of::<size_t>() as libc::c_ulong),
                ),
        );
        (*current).buffer = xmalloc((*current).bufsize) as *mut size_t;
        if !skip_test {
            let mut prev_mode: libc::c_int = set_binary_mode(
                (*current).desc,
                0 as libc::c_int,
            );
            let mut buffered: off_t = 0;
            file_block_read(current, (*current).bufsize);
            buffered = (*current).buffered as off_t;
            if prev_mode != 0 as libc::c_int {
                if lseek((*current).desc, -buffered, 1 as libc::c_int)
                    < 0 as libc::c_int as libc::c_long
                {
                    pfatal_with_name((*current).name);
                }
                set_binary_mode((*current).desc, prev_mode);
                (*current).buffered = 0 as libc::c_int as size_t;
                (*current).eof = 0 as libc::c_int != 0;
            }
            return !(memchr(
                (*current).buffer as *const libc::c_void,
                0 as libc::c_int,
                buffered as libc::c_ulong,
            ))
                .is_null();
        }
    }
    (*current).buffered = 0 as libc::c_int as size_t;
    (*current).eof = 0 as libc::c_int != 0;
    return 0 as libc::c_int != 0;
}
unsafe extern "C" fn slurp(mut current: *mut file_data) {
    let mut cc: size_t = 0;
    if (*current).desc < 0 as libc::c_int {
        return;
    }
    if (*current).stat.st_mode & 0o170000 as libc::c_int as libc::c_uint
        == 0o100000 as libc::c_int as libc::c_uint
    {
        let mut file_size: off_t = (*current).stat.st_size;
        let (fresh0, fresh1) = (2 as libc::c_int as libc::c_ulong)
            .wrapping_mul(::std::mem::size_of::<size_t>() as libc::c_ulong)
            .wrapping_sub(
                (file_size as libc::c_ulong)
                    .wrapping_rem(::std::mem::size_of::<size_t>() as libc::c_ulong),
            )
            .overflowing_add(file_size.try_into().unwrap());
        *(&mut cc as *mut size_t) = fresh0;
        if fresh1 {
            xalloc_die();
        }
        if (*current).bufsize < cc {
            (*current).bufsize = cc;
            (*current)
                .buffer = xrealloc((*current).buffer as *mut libc::c_void, cc)
                as *mut size_t;
        }
        if (*current).buffered <= file_size as libc::c_ulong {
            file_block_read(
                current,
                ((file_size + 1 as libc::c_int as libc::c_long) as libc::c_ulong)
                    .wrapping_sub((*current).buffered),
            );
            if (*current).buffered <= file_size as libc::c_ulong {
                return;
            }
        }
    }
    file_block_read(current, ((*current).bufsize).wrapping_sub((*current).buffered));
    if (*current).buffered != 0 {
        while (*current).buffered == (*current).bufsize {
            if ((9223372036854775807 as libc::c_long / 2 as libc::c_int as libc::c_long)
                as libc::c_ulong)
                .wrapping_sub(::std::mem::size_of::<size_t>() as libc::c_ulong)
                < (*current).bufsize
            {
                xalloc_die();
            }
            (*current)
                .bufsize = ((*current).bufsize as libc::c_ulong)
                .wrapping_mul(2 as libc::c_int as libc::c_ulong) as size_t as size_t;
            (*current)
                .buffer = xrealloc(
                (*current).buffer as *mut libc::c_void,
                (*current).bufsize,
            ) as *mut size_t;
            file_block_read(
                current,
                ((*current).bufsize).wrapping_sub((*current).buffered),
            );
        }
        cc = ((*current).buffered)
            .wrapping_add(
                (2 as libc::c_int as libc::c_ulong)
                    .wrapping_mul(::std::mem::size_of::<size_t>() as libc::c_ulong),
            );
        (*current)
            .bufsize = cc
            .wrapping_sub(
                cc.wrapping_rem(::std::mem::size_of::<size_t>() as libc::c_ulong),
            );
        (*current)
            .buffer = xrealloc(
            (*current).buffer as *mut libc::c_void,
            (*current).bufsize,
        ) as *mut size_t;
    }
}
unsafe extern "C" fn find_and_hash_each_line(mut current: *mut file_data) {
    let mut p: *const libc::c_char = (*current).prefix_end;
    let mut i: lin = 0;
    let mut bucket: *mut lin = 0 as *mut lin;
    let mut length: size_t = 0;
    let mut linbuf: *mut *const libc::c_char = (*current).linbuf;
    let mut alloc_lines: lin = (*current).alloc_lines;
    let mut line: lin = 0 as libc::c_int as lin;
    let mut linbuf_base: lin = (*current).linbuf_base;
    let mut cureqs: *mut lin = xmalloc(
        (alloc_lines as libc::c_ulong)
            .wrapping_mul(::std::mem::size_of::<lin>() as libc::c_ulong),
    ) as *mut lin;
    let mut eqs: *mut equivclass = equivs;
    let mut eqs_index: lin = equivs_index;
    let mut eqs_alloc: lin = equivs_alloc;
    let mut suffix_begin: *const libc::c_char = (*current).suffix_begin;
    let mut bufend: *const libc::c_char = (file_buffer(current))
        .offset((*current).buffered as isize);
    let mut ig_case: bool = ignore_case;
    let mut ig_white_space: DIFF_white_space = ignore_white_space;
    let mut diff_length_compare_anyway: bool = ig_white_space as libc::c_uint
        != IGNORE_NO_WHITE_SPACE as libc::c_int as libc::c_uint;
    let mut same_length_diff_contents_compare_anyway: bool = diff_length_compare_anyway
        as libc::c_int | ig_case as libc::c_int != 0;
    while p < suffix_begin {
        let mut ip: *const libc::c_char = p;
        let mut h: hash_value = 0 as libc::c_int as hash_value;
        let mut c: libc::c_uchar = 0;
        match ig_white_space as libc::c_uint {
            5 => {
                loop {
                    let fresh2 = p;
                    p = p.offset(1);
                    c = *fresh2 as libc::c_uchar;
                    if !(c as libc::c_int != '\n' as i32) {
                        break;
                    }
                    if *(*__ctype_b_loc()).offset(c as libc::c_int as isize)
                        as libc::c_int
                        & _ISspace as libc::c_int as libc::c_ushort as libc::c_int == 0
                    {
                        h = ((if ig_case as libc::c_int != 0 {
                            {
                                let mut __res: libc::c_int = 0;
                                if ::std::mem::size_of::<libc::c_uchar>() as libc::c_ulong
                                    > 1 as libc::c_int as libc::c_ulong
                                {
                                    if 0 != 0 {
                                        let mut __c: libc::c_int = c as libc::c_int;
                                        __res = if __c < -(128 as libc::c_int)
                                            || __c > 255 as libc::c_int
                                        {
                                            __c
                                        } else {
                                            *(*__ctype_tolower_loc()).offset(__c as isize)
                                        };
                                    } else {
                                        __res = tolower(c as libc::c_int);
                                    }
                                } else {
                                    __res = *(*__ctype_tolower_loc())
                                        .offset(c as libc::c_int as isize);
                                }
                                __res
                            }
                        } else {
                            c as libc::c_int
                        }) as libc::c_ulong)
                            .wrapping_add(
                                h << 7 as libc::c_int
                                    | h
                                        >> (::std::mem::size_of::<hash_value>() as libc::c_ulong)
                                            .wrapping_mul(8 as libc::c_int as libc::c_ulong)
                                            .wrapping_sub(7 as libc::c_int as libc::c_ulong),
                            );
                    }
                }
            }
            4 => {
                's_66: loop {
                    let fresh3 = p;
                    p = p.offset(1);
                    c = *fresh3 as libc::c_uchar;
                    if !(c as libc::c_int != '\n' as i32) {
                        break;
                    }
                    if *(*__ctype_b_loc()).offset(c as libc::c_int as isize)
                        as libc::c_int
                        & _ISspace as libc::c_int as libc::c_ushort as libc::c_int != 0
                    {
                        loop {
                            let fresh4 = p;
                            p = p.offset(1);
                            c = *fresh4 as libc::c_uchar;
                            if c as libc::c_int == '\n' as i32 {
                                break 's_66;
                            }
                            if !(*(*__ctype_b_loc()).offset(c as libc::c_int as isize)
                                as libc::c_int
                                & _ISspace as libc::c_int as libc::c_ushort as libc::c_int
                                != 0)
                            {
                                break;
                            }
                        }
                        h = (' ' as i32 as libc::c_ulong)
                            .wrapping_add(
                                h << 7 as libc::c_int
                                    | h
                                        >> (::std::mem::size_of::<hash_value>() as libc::c_ulong)
                                            .wrapping_mul(8 as libc::c_int as libc::c_ulong)
                                            .wrapping_sub(7 as libc::c_int as libc::c_ulong),
                            );
                    }
                    h = ((if ig_case as libc::c_int != 0 {
                        {
                            let mut __res: libc::c_int = 0;
                            if ::std::mem::size_of::<libc::c_uchar>() as libc::c_ulong
                                > 1 as libc::c_int as libc::c_ulong
                            {
                                if 0 != 0 {
                                    let mut __c: libc::c_int = c as libc::c_int;
                                    __res = if __c < -(128 as libc::c_int)
                                        || __c > 255 as libc::c_int
                                    {
                                        __c
                                    } else {
                                        *(*__ctype_tolower_loc()).offset(__c as isize)
                                    };
                                } else {
                                    __res = tolower(c as libc::c_int);
                                }
                            } else {
                                __res = *(*__ctype_tolower_loc())
                                    .offset(c as libc::c_int as isize);
                            }
                            __res
                        }
                    } else {
                        c as libc::c_int
                    }) as libc::c_ulong)
                        .wrapping_add(
                            h << 7 as libc::c_int
                                | h
                                    >> (::std::mem::size_of::<hash_value>() as libc::c_ulong)
                                        .wrapping_mul(8 as libc::c_int as libc::c_ulong)
                                        .wrapping_sub(7 as libc::c_int as libc::c_ulong),
                        );
                }
            }
            1 | 3 | 2 => {
                let mut column: size_t = 0 as libc::c_int as size_t;
                's_104: loop {
                    let fresh5 = p;
                    p = p.offset(1);
                    c = *fresh5 as libc::c_uchar;
                    if !(c as libc::c_int != '\n' as i32) {
                        break;
                    }
                    if ig_white_space as libc::c_uint
                        & IGNORE_TRAILING_SPACE as libc::c_int as libc::c_uint != 0
                        && *(*__ctype_b_loc()).offset(c as libc::c_int as isize)
                            as libc::c_int
                            & _ISspace as libc::c_int as libc::c_ushort as libc::c_int
                            != 0
                    {
                        let mut p1: *const libc::c_char = p;
                        let mut c1: libc::c_uchar = 0;
                        loop {
                            let fresh6 = p1;
                            p1 = p1.offset(1);
                            c1 = *fresh6 as libc::c_uchar;
                            if c1 as libc::c_int == '\n' as i32 {
                                p = p1;
                                break 's_104;
                            } else if !(*(*__ctype_b_loc())
                                .offset(c1 as libc::c_int as isize) as libc::c_int
                                & _ISspace as libc::c_int as libc::c_ushort as libc::c_int
                                != 0)
                            {
                                break;
                            }
                        }
                    }
                    let mut repetitions: size_t = 1 as libc::c_int as size_t;
                    if ig_white_space as libc::c_uint
                        & IGNORE_TAB_EXPANSION as libc::c_int as libc::c_uint != 0
                    {
                        match c as libc::c_int {
                            8 => {
                                column = (column as libc::c_ulong)
                                    .wrapping_sub(
                                        ((0 as libc::c_int as libc::c_ulong) < column)
                                            as libc::c_int as libc::c_ulong,
                                    ) as size_t as size_t;
                            }
                            9 => {
                                c = ' ' as i32 as libc::c_uchar;
                                repetitions = tabsize
                                    .wrapping_sub(column.wrapping_rem(tabsize));
                                column = if column.wrapping_add(repetitions) < column {
                                    0 as libc::c_int as libc::c_ulong
                                } else {
                                    column.wrapping_add(repetitions)
                                };
                            }
                            13 => {
                                column = 0 as libc::c_int as size_t;
                            }
                            _ => {
                                column = column.wrapping_add(1);
                                column;
                            }
                        }
                    }
                    if ig_case {
                        c = ({
                            let mut __res: libc::c_int = 0;
                            if ::std::mem::size_of::<libc::c_uchar>() as libc::c_ulong
                                > 1 as libc::c_int as libc::c_ulong
                            {
                                if 0 != 0 {
                                    let mut __c: libc::c_int = c as libc::c_int;
                                    __res = if __c < -(128 as libc::c_int)
                                        || __c > 255 as libc::c_int
                                    {
                                        __c
                                    } else {
                                        *(*__ctype_tolower_loc()).offset(__c as isize)
                                    };
                                } else {
                                    __res = tolower(c as libc::c_int);
                                }
                            } else {
                                __res = *(*__ctype_tolower_loc())
                                    .offset(c as libc::c_int as isize);
                            }
                            __res
                        }) as libc::c_uchar;
                    }
                    loop {
                        h = (c as libc::c_ulong)
                            .wrapping_add(
                                h << 7 as libc::c_int
                                    | h
                                        >> (::std::mem::size_of::<hash_value>() as libc::c_ulong)
                                            .wrapping_mul(8 as libc::c_int as libc::c_ulong)
                                            .wrapping_sub(7 as libc::c_int as libc::c_ulong),
                            );
                        repetitions = repetitions.wrapping_sub(1);
                        if !(repetitions != 0 as libc::c_int as libc::c_ulong) {
                            break;
                        }
                    }
                }
            }
            _ => {
                if ig_case {
                    loop {
                        let fresh7 = p;
                        p = p.offset(1);
                        c = *fresh7 as libc::c_uchar;
                        if !(c as libc::c_int != '\n' as i32) {
                            break;
                        }
                        h = (({
                            let mut __res: libc::c_int = 0;
                            if ::std::mem::size_of::<libc::c_uchar>() as libc::c_ulong
                                > 1 as libc::c_int as libc::c_ulong
                            {
                                if 0 != 0 {
                                    let mut __c: libc::c_int = c as libc::c_int;
                                    __res = if __c < -(128 as libc::c_int)
                                        || __c > 255 as libc::c_int
                                    {
                                        __c
                                    } else {
                                        *(*__ctype_tolower_loc()).offset(__c as isize)
                                    };
                                } else {
                                    __res = tolower(c as libc::c_int);
                                }
                            } else {
                                __res = *(*__ctype_tolower_loc())
                                    .offset(c as libc::c_int as isize);
                            }
                            __res
                        }) as libc::c_ulong)
                            .wrapping_add(
                                h << 7 as libc::c_int
                                    | h
                                        >> (::std::mem::size_of::<hash_value>() as libc::c_ulong)
                                            .wrapping_mul(8 as libc::c_int as libc::c_ulong)
                                            .wrapping_sub(7 as libc::c_int as libc::c_ulong),
                            );
                    }
                } else {
                    loop {
                        let fresh8 = p;
                        p = p.offset(1);
                        c = *fresh8 as libc::c_uchar;
                        if !(c as libc::c_int != '\n' as i32) {
                            break;
                        }
                        h = (c as libc::c_ulong)
                            .wrapping_add(
                                h << 7 as libc::c_int
                                    | h
                                        >> (::std::mem::size_of::<hash_value>() as libc::c_ulong)
                                            .wrapping_mul(8 as libc::c_int as libc::c_ulong)
                                            .wrapping_sub(7 as libc::c_int as libc::c_ulong),
                            );
                    }
                }
            }
        }
        bucket = &mut *buckets.offset(h.wrapping_rem(nbuckets) as isize) as *mut lin;
        length = (p.offset_from(ip) as libc::c_long - 1 as libc::c_int as libc::c_long)
            as size_t;
        if p == bufend && (*current).missing_newline as libc::c_int != 0
            && robust_output_style(output_style) as libc::c_int != 0
        {
            if (ig_white_space as libc::c_uint)
                < IGNORE_TRAILING_SPACE as libc::c_int as libc::c_uint
            {
                bucket = &mut *buckets.offset(-(1 as libc::c_int) as isize) as *mut lin;
            }
        }
        let mut current_block_42: u64;
        i = *bucket;
        loop {
            if i == 0 {
                let fresh9 = eqs_index;
                eqs_index = eqs_index + 1;
                i = fresh9;
                if i == eqs_alloc {
                    if (9223372036854775807 as libc::c_long as libc::c_ulong)
                        .wrapping_div(
                            (2 as libc::c_int as libc::c_ulong)
                                .wrapping_mul(
                                    ::std::mem::size_of::<equivclass>() as libc::c_ulong,
                                ),
                        ) <= eqs_alloc as libc::c_ulong
                    {
                        xalloc_die();
                    }
                    eqs_alloc *= 2 as libc::c_int as libc::c_long;
                    eqs = xrealloc(
                        eqs as *mut libc::c_void,
                        (eqs_alloc as libc::c_ulong)
                            .wrapping_mul(
                                ::std::mem::size_of::<equivclass>() as libc::c_ulong,
                            ),
                    ) as *mut equivclass;
                }
                (*eqs.offset(i as isize)).next = *bucket;
                (*eqs.offset(i as isize)).hash = h;
                let ref mut fresh10 = (*eqs.offset(i as isize)).line;
                *fresh10 = ip;
                (*eqs.offset(i as isize)).length = length;
                *bucket = i;
                break;
            } else {
                if (*eqs.offset(i as isize)).hash == h {
                    let mut eqline: *const libc::c_char = (*eqs.offset(i as isize)).line;
                    if (*eqs.offset(i as isize)).length == length {
                        if memcmp(
                            eqline as *const libc::c_void,
                            ip as *const libc::c_void,
                            length,
                        ) == 0 as libc::c_int
                        {
                            break;
                        }
                        if !same_length_diff_contents_compare_anyway {
                            current_block_42 = 13763002826403452995;
                        } else {
                            current_block_42 = 7330218953828964527;
                        }
                    } else if !diff_length_compare_anyway {
                        current_block_42 = 13763002826403452995;
                    } else {
                        current_block_42 = 7330218953828964527;
                    }
                    match current_block_42 {
                        13763002826403452995 => {}
                        _ => {
                            if !lines_differ(eqline, ip) {
                                break;
                            }
                        }
                    }
                }
                i = (*eqs.offset(i as isize)).next;
            }
        }
        if line == alloc_lines {
            if 9223372036854775807 as libc::c_long / 3 as libc::c_int as libc::c_long
                <= alloc_lines
                || (9223372036854775807 as libc::c_long as libc::c_ulong)
                    .wrapping_div(::std::mem::size_of::<lin>() as libc::c_ulong)
                    <= (2 as libc::c_int as libc::c_long * alloc_lines - linbuf_base)
                        as libc::c_ulong
                || (9223372036854775807 as libc::c_long as libc::c_ulong)
                    .wrapping_div(
                        ::std::mem::size_of::<*const libc::c_char>() as libc::c_ulong,
                    ) <= (alloc_lines - linbuf_base) as libc::c_ulong
            {
                xalloc_die();
            }
            alloc_lines = 2 as libc::c_int as libc::c_long * alloc_lines - linbuf_base;
            cureqs = xrealloc(
                cureqs as *mut libc::c_void,
                (alloc_lines as libc::c_ulong)
                    .wrapping_mul(::std::mem::size_of::<lin>() as libc::c_ulong),
            ) as *mut lin;
            linbuf = linbuf.offset(linbuf_base as isize);
            linbuf = xrealloc(
                linbuf as *mut libc::c_void,
                ((alloc_lines - linbuf_base) as libc::c_ulong)
                    .wrapping_mul(
                        ::std::mem::size_of::<*const libc::c_char>() as libc::c_ulong,
                    ),
            ) as *mut *const libc::c_char;
            linbuf = linbuf.offset(-(linbuf_base as isize));
        }
        let ref mut fresh11 = *linbuf.offset(line as isize);
        *fresh11 = ip;
        *cureqs.offset(line as isize) = i;
        line += 1;
        line;
    }
    (*current).buffered_lines = line;
    i = 0 as libc::c_int as lin;
    loop {
        if line == alloc_lines {
            if 9223372036854775807 as libc::c_long / 3 as libc::c_int as libc::c_long
                <= alloc_lines
                || (9223372036854775807 as libc::c_long as libc::c_ulong)
                    .wrapping_div(::std::mem::size_of::<lin>() as libc::c_ulong)
                    <= (2 as libc::c_int as libc::c_long * alloc_lines - linbuf_base)
                        as libc::c_ulong
                || (9223372036854775807 as libc::c_long as libc::c_ulong)
                    .wrapping_div(
                        ::std::mem::size_of::<*const libc::c_char>() as libc::c_ulong,
                    ) <= (alloc_lines - linbuf_base) as libc::c_ulong
            {
                xalloc_die();
            }
            alloc_lines = 2 as libc::c_int as libc::c_long * alloc_lines - linbuf_base;
            linbuf = linbuf.offset(linbuf_base as isize);
            linbuf = xrealloc(
                linbuf as *mut libc::c_void,
                ((alloc_lines - linbuf_base) as libc::c_ulong)
                    .wrapping_mul(
                        ::std::mem::size_of::<*const libc::c_char>() as libc::c_ulong,
                    ),
            ) as *mut *const libc::c_char;
            linbuf = linbuf.offset(-(linbuf_base as isize));
        }
        let ref mut fresh12 = *linbuf.offset(line as isize);
        *fresh12 = p;
        if p == bufend {
            if (*current).missing_newline as libc::c_int != 0
                && robust_output_style(output_style) as libc::c_int != 0
            {
                let ref mut fresh13 = *linbuf.offset(line as isize);
                *fresh13 = (*fresh13).offset(-1);
                *fresh13;
            }
            break;
        } else {
            if context <= i && no_diff_means_no_output as libc::c_int != 0 {
                break;
            }
            line += 1;
            line;
            loop {
                let fresh14 = p;
                p = p.offset(1);
                if !(*fresh14 as libc::c_int != '\n' as i32) {
                    break;
                }
            }
            i += 1;
            i;
        }
    }
    (*current).linbuf = linbuf;
    (*current).valid_lines = line;
    (*current).alloc_lines = alloc_lines;
    (*current).equivs = cureqs;
    equivs = eqs;
    equivs_alloc = eqs_alloc;
    equivs_index = eqs_index;
}
unsafe extern "C" fn prepare_text(mut current: *mut file_data) {
    let mut buffered: size_t = (*current).buffered;
    let mut p: *mut libc::c_char = file_buffer(current);
    if p.is_null() {
        return;
    }
    if strip_trailing_cr {
        let mut srclim: *mut libc::c_char = p.offset(buffered as isize);
        *srclim = '\r' as i32 as libc::c_char;
        let mut dst: *mut libc::c_char = rawmemchr(p as *const libc::c_void, '\r' as i32)
            as *mut libc::c_char;
        let mut src: *const libc::c_char = dst;
        while src != srclim as *const libc::c_char {
            src = src
                .offset(
                    (*src as libc::c_int == '\r' as i32
                        && *src.offset(1 as libc::c_int as isize) as libc::c_int
                            == '\n' as i32) as libc::c_int as isize,
                );
            let fresh15 = dst;
            dst = dst.offset(1);
            *fresh15 = *src;
            src = src.offset(1);
            src;
        }
        buffered = (buffered as libc::c_ulong)
            .wrapping_sub(srclim.offset_from(dst) as libc::c_long as libc::c_ulong)
            as size_t as size_t;
    }
    if buffered != 0 as libc::c_int as libc::c_ulong
        && *p.offset(buffered.wrapping_sub(1 as libc::c_int as libc::c_ulong) as isize)
            as libc::c_int != '\n' as i32
    {
        let fresh16 = buffered;
        buffered = buffered.wrapping_add(1);
        *p.offset(fresh16 as isize) = '\n' as i32 as libc::c_char;
        (*current).missing_newline = 1 as libc::c_int != 0;
    }
    memset(
        p.offset(buffered as isize) as *mut libc::c_void,
        0 as libc::c_int,
        ::std::mem::size_of::<size_t>() as libc::c_ulong,
    );
    (*current).buffered = buffered;
}
unsafe extern "C" fn guess_lines(mut n: lin, mut s: size_t, mut t: size_t) -> lin {
    let mut guessed_bytes_per_line: size_t = if n < 10 as libc::c_int as libc::c_long {
        32 as libc::c_int as libc::c_ulong
    } else {
        s.wrapping_div((n - 1 as libc::c_int as libc::c_long) as libc::c_ulong)
    };
    let mut guessed_lines: lin = (if 1 as libc::c_int as libc::c_ulong
        >= t.wrapping_div(guessed_bytes_per_line)
    {
        1 as libc::c_int as libc::c_ulong
    } else {
        t.wrapping_div(guessed_bytes_per_line)
    }) as lin;
    return (if guessed_lines as libc::c_ulong
        <= (9223372036854775807 as libc::c_long as libc::c_ulong)
            .wrapping_div(
                (2 as libc::c_int as libc::c_ulong)
                    .wrapping_mul(
                        ::std::mem::size_of::<*mut libc::c_char>() as libc::c_ulong,
                    )
                    .wrapping_add(1 as libc::c_int as libc::c_ulong),
            )
            .wrapping_sub(5 as libc::c_int as libc::c_ulong)
    {
        guessed_lines as libc::c_ulong
    } else {
        (9223372036854775807 as libc::c_long as libc::c_ulong)
            .wrapping_div(
                (2 as libc::c_int as libc::c_ulong)
                    .wrapping_mul(
                        ::std::mem::size_of::<*mut libc::c_char>() as libc::c_ulong,
                    )
                    .wrapping_add(1 as libc::c_int as libc::c_ulong),
            )
            .wrapping_sub(5 as libc::c_int as libc::c_ulong)
    })
        .wrapping_add(5 as libc::c_int as libc::c_ulong) as lin;
}
unsafe extern "C" fn find_identical_ends(mut filevec: *mut file_data) {
    let mut w0: *mut size_t = 0 as *mut size_t;
    let mut w1: *mut size_t = 0 as *mut size_t;
    let mut p0: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut p1: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut buffer0: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut buffer1: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut end0: *const libc::c_char = 0 as *const libc::c_char;
    let mut beg0: *const libc::c_char = 0 as *const libc::c_char;
    let mut linbuf0: *mut *const libc::c_char = 0 as *mut *const libc::c_char;
    let mut linbuf1: *mut *const libc::c_char = 0 as *mut *const libc::c_char;
    let mut i: lin = 0;
    let mut lines: lin = 0;
    let mut n0: size_t = 0;
    let mut n1: size_t = 0;
    let mut alloc_lines0: lin = 0;
    let mut alloc_lines1: lin = 0;
    let mut prefix_needed: bool = false;
    let mut buffered_prefix: lin = 0;
    let mut prefix_count: lin = 0;
    let mut prefix_mask: lin = 0;
    let mut middle_guess: lin = 0;
    let mut suffix_guess: lin = 0;
    slurp(&mut *filevec.offset(0 as libc::c_int as isize));
    prepare_text(&mut *filevec.offset(0 as libc::c_int as isize));
    if (*filevec.offset(0 as libc::c_int as isize)).desc
        != (*filevec.offset(1 as libc::c_int as isize)).desc
    {
        slurp(&mut *filevec.offset(1 as libc::c_int as isize));
        prepare_text(&mut *filevec.offset(1 as libc::c_int as isize));
    } else {
        let ref mut fresh17 = (*filevec.offset(1 as libc::c_int as isize)).buffer;
        *fresh17 = (*filevec.offset(0 as libc::c_int as isize)).buffer;
        (*filevec.offset(1 as libc::c_int as isize))
            .bufsize = (*filevec.offset(0 as libc::c_int as isize)).bufsize;
        (*filevec.offset(1 as libc::c_int as isize))
            .buffered = (*filevec.offset(0 as libc::c_int as isize)).buffered;
        (*filevec.offset(1 as libc::c_int as isize))
            .missing_newline = (*filevec.offset(0 as libc::c_int as isize))
            .missing_newline;
    }
    w0 = (*filevec.offset(0 as libc::c_int as isize)).buffer;
    w1 = (*filevec.offset(1 as libc::c_int as isize)).buffer;
    buffer0 = w0 as *mut libc::c_char;
    p0 = buffer0;
    buffer1 = w1 as *mut libc::c_char;
    p1 = buffer1;
    n0 = (*filevec.offset(0 as libc::c_int as isize)).buffered;
    n1 = (*filevec.offset(1 as libc::c_int as isize)).buffered;
    if p0 == p1 {
        p1 = p1.offset(n1 as isize);
        p0 = p1;
    } else {
        if n0 < n1 {
            *p0
                .offset(
                    n0 as isize,
                ) = !(*p1.offset(n0 as isize) as libc::c_int) as libc::c_char;
        } else {
            *p1
                .offset(
                    n1 as isize,
                ) = !(*p0.offset(n1 as isize) as libc::c_int) as libc::c_char;
        }
        while *w0 == *w1 {
            w0 = w0.offset(1);
            w0;
            w1 = w1.offset(1);
            w1;
        }
        p0 = w0 as *mut libc::c_char;
        p1 = w1 as *mut libc::c_char;
        while *p0 as libc::c_int == *p1 as libc::c_int {
            p0 = p0.offset(1);
            p0;
            p1 = p1.offset(1);
            p1;
        }
        if robust_output_style(output_style) as libc::c_int != 0
            && (buffer0
                .offset(n0 as isize)
                .offset(
                    -((*filevec.offset(0 as libc::c_int as isize)).missing_newline
                        as libc::c_int as isize),
                ) < p0) as libc::c_int
                != (buffer1
                    .offset(n1 as isize)
                    .offset(
                        -((*filevec.offset(1 as libc::c_int as isize)).missing_newline
                            as libc::c_int as isize),
                    ) < p1) as libc::c_int
        {
            p0 = p0.offset(-1);
            p0;
            p1 = p1.offset(-1);
            p1;
        }
    }
    i = horizon_lines;
    while p0 != buffer0
        && (*p0.offset(-(1 as libc::c_int) as isize) as libc::c_int != '\n' as i32
            || {
                let fresh18 = i;
                i = i - 1;
                fresh18 != 0
            })
    {
        p0 = p0.offset(-1);
        p0;
        p1 = p1.offset(-1);
        p1;
    }
    let ref mut fresh19 = (*filevec.offset(0 as libc::c_int as isize)).prefix_end;
    *fresh19 = p0;
    let ref mut fresh20 = (*filevec.offset(1 as libc::c_int as isize)).prefix_end;
    *fresh20 = p1;
    p0 = buffer0.offset(n0 as isize);
    p1 = buffer1.offset(n1 as isize);
    if !robust_output_style(output_style)
        || (*filevec.offset(0 as libc::c_int as isize)).missing_newline as libc::c_int
            == (*filevec.offset(1 as libc::c_int as isize)).missing_newline
                as libc::c_int
    {
        end0 = p0;
        beg0 = ((*filevec.offset(0 as libc::c_int as isize)).prefix_end)
            .offset(
                (if n0 < n1 {
                    0 as libc::c_int as libc::c_ulong
                } else {
                    n0.wrapping_sub(n1)
                }) as isize,
            );
        while p0 != beg0 as *mut libc::c_char {
            p0 = p0.offset(-1);
            p1 = p1.offset(-1);
            if !(*p0 as libc::c_int != *p1 as libc::c_int) {
                continue;
            }
            p0 = p0.offset(1);
            p0;
            p1 = p1.offset(1);
            p1;
            beg0 = p0;
            break;
        }
        i = horizon_lines
            + !((buffer0 == p0
                || *p0.offset(-(1 as libc::c_int) as isize) as libc::c_int
                    == '\n' as i32)
                && (buffer1 == p1
                    || *p1.offset(-(1 as libc::c_int) as isize) as libc::c_int
                        == '\n' as i32)) as libc::c_int as libc::c_long;
        loop {
            let fresh21 = i;
            i = i - 1;
            if !(fresh21 != 0 && p0 != end0 as *mut libc::c_char) {
                break;
            }
            loop {
                let fresh22 = p0;
                p0 = p0.offset(1);
                if !(*fresh22 as libc::c_int != '\n' as i32) {
                    break;
                }
            }
        }
        p1 = p1.offset(p0.offset_from(beg0) as libc::c_long as isize);
    }
    let ref mut fresh23 = (*filevec.offset(0 as libc::c_int as isize)).suffix_begin;
    *fresh23 = p0;
    let ref mut fresh24 = (*filevec.offset(1 as libc::c_int as isize)).suffix_begin;
    *fresh24 = p1;
    if no_diff_means_no_output as libc::c_int != 0 && (function_regexp.fastmap).is_null()
        && context
            < 9223372036854775807 as libc::c_long / 4 as libc::c_int as libc::c_long
        && (context as libc::c_ulong) < n0
    {
        middle_guess = guess_lines(
            0 as libc::c_int as lin,
            0 as libc::c_int as size_t,
            p0.offset_from((*filevec.offset(0 as libc::c_int as isize)).prefix_end)
                as libc::c_long as size_t,
        );
        suffix_guess = guess_lines(
            0 as libc::c_int as lin,
            0 as libc::c_int as size_t,
            buffer0.offset(n0 as isize).offset_from(p0) as libc::c_long as size_t,
        );
        prefix_count = 1 as libc::c_int as lin;
        while prefix_count <= context {
            prefix_count *= 2 as libc::c_int as libc::c_long;
        }
        alloc_lines0 = prefix_count + middle_guess
            + (if context <= suffix_guess { context } else { suffix_guess });
    } else {
        prefix_count = 0 as libc::c_int as lin;
        alloc_lines0 = guess_lines(
            0 as libc::c_int as lin,
            0 as libc::c_int as size_t,
            n0,
        );
    }
    prefix_mask = prefix_count - 1 as libc::c_int as libc::c_long;
    lines = 0 as libc::c_int as lin;
    linbuf0 = xmalloc(
        (alloc_lines0 as libc::c_ulong)
            .wrapping_mul(::std::mem::size_of::<*const libc::c_char>() as libc::c_ulong),
    ) as *mut *const libc::c_char;
    prefix_needed = !(no_diff_means_no_output as libc::c_int != 0
        && (*filevec.offset(0 as libc::c_int as isize)).prefix_end
            == p0 as *const libc::c_char
        && (*filevec.offset(1 as libc::c_int as isize)).prefix_end
            == p1 as *const libc::c_char);
    p0 = buffer0;
    if prefix_needed {
        end0 = (*filevec.offset(0 as libc::c_int as isize)).prefix_end;
        while p0 != end0 as *mut libc::c_char {
            let fresh25 = lines;
            lines = lines + 1;
            let mut l: lin = fresh25 & prefix_mask;
            if l == alloc_lines0 {
                if (9223372036854775807 as libc::c_long as libc::c_ulong)
                    .wrapping_div(
                        (2 as libc::c_int as libc::c_ulong)
                            .wrapping_mul(
                                ::std::mem::size_of::<*const libc::c_char>()
                                    as libc::c_ulong,
                            ),
                    ) <= alloc_lines0 as libc::c_ulong
                {
                    xalloc_die();
                }
                alloc_lines0 *= 2 as libc::c_int as libc::c_long;
                linbuf0 = xrealloc(
                    linbuf0 as *mut libc::c_void,
                    (alloc_lines0 as libc::c_ulong)
                        .wrapping_mul(
                            ::std::mem::size_of::<*const libc::c_char>() as libc::c_ulong,
                        ),
                ) as *mut *const libc::c_char;
            }
            let ref mut fresh26 = *linbuf0.offset(l as isize);
            *fresh26 = p0;
            loop {
                let fresh27 = p0;
                p0 = p0.offset(1);
                if !(*fresh27 as libc::c_int != '\n' as i32) {
                    break;
                }
            }
        }
    }
    buffered_prefix = if prefix_count != 0 && context < lines { context } else { lines };
    middle_guess = guess_lines(
        lines,
        p0.offset_from(buffer0) as libc::c_long as size_t,
        p1.offset_from((*filevec.offset(1 as libc::c_int as isize)).prefix_end)
            as libc::c_long as size_t,
    );
    suffix_guess = guess_lines(
        lines,
        p0.offset_from(buffer0) as libc::c_long as size_t,
        buffer1.offset(n1 as isize).offset_from(p1) as libc::c_long as size_t,
    );
    let (fresh28, fresh29) = buffered_prefix
        .overflowing_add(
            middle_guess + (if context <= suffix_guess { context } else { suffix_guess }),
        );
    *(&mut alloc_lines1 as *mut lin) = fresh28;
    if fresh29 {
        xalloc_die();
    }
    linbuf1 = xnmalloc(
        alloc_lines1 as size_t,
        ::std::mem::size_of::<*const libc::c_char>() as libc::c_ulong,
    ) as *mut *const libc::c_char;
    if buffered_prefix != lines {
        i = 0 as libc::c_int as lin;
        while i < buffered_prefix {
            let ref mut fresh30 = *linbuf1.offset(i as isize);
            *fresh30 = *linbuf0.offset((lines - context + i & prefix_mask) as isize);
            i += 1;
            i;
        }
        i = 0 as libc::c_int as lin;
        while i < buffered_prefix {
            let ref mut fresh31 = *linbuf0.offset(i as isize);
            *fresh31 = *linbuf1.offset(i as isize);
            i += 1;
            i;
        }
    }
    i = 0 as libc::c_int as lin;
    while i < buffered_prefix {
        let ref mut fresh32 = *linbuf1.offset(i as isize);
        *fresh32 = buffer1
            .offset(
                (*linbuf0.offset(i as isize)).offset_from(buffer0) as libc::c_long
                    as isize,
            );
        i += 1;
        i;
    }
    let ref mut fresh33 = (*filevec.offset(0 as libc::c_int as isize)).linbuf;
    *fresh33 = linbuf0.offset(buffered_prefix as isize);
    let ref mut fresh34 = (*filevec.offset(1 as libc::c_int as isize)).linbuf;
    *fresh34 = linbuf1.offset(buffered_prefix as isize);
    let ref mut fresh35 = (*filevec.offset(1 as libc::c_int as isize)).linbuf_base;
    *fresh35 = -buffered_prefix;
    (*filevec.offset(0 as libc::c_int as isize)).linbuf_base = *fresh35;
    (*filevec.offset(0 as libc::c_int as isize))
        .alloc_lines = alloc_lines0 - buffered_prefix;
    (*filevec.offset(1 as libc::c_int as isize))
        .alloc_lines = alloc_lines1 - buffered_prefix;
    let ref mut fresh36 = (*filevec.offset(1 as libc::c_int as isize)).prefix_lines;
    *fresh36 = lines;
    (*filevec.offset(0 as libc::c_int as isize)).prefix_lines = *fresh36;
}
static mut prime_offset: [libc::c_uchar; 64] = [
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    1 as libc::c_int as libc::c_uchar,
    1 as libc::c_int as libc::c_uchar,
    3 as libc::c_int as libc::c_uchar,
    1 as libc::c_int as libc::c_uchar,
    3 as libc::c_int as libc::c_uchar,
    1 as libc::c_int as libc::c_uchar,
    5 as libc::c_int as libc::c_uchar,
    3 as libc::c_int as libc::c_uchar,
    3 as libc::c_int as libc::c_uchar,
    9 as libc::c_int as libc::c_uchar,
    3 as libc::c_int as libc::c_uchar,
    1 as libc::c_int as libc::c_uchar,
    3 as libc::c_int as libc::c_uchar,
    19 as libc::c_int as libc::c_uchar,
    15 as libc::c_int as libc::c_uchar,
    1 as libc::c_int as libc::c_uchar,
    5 as libc::c_int as libc::c_uchar,
    1 as libc::c_int as libc::c_uchar,
    3 as libc::c_int as libc::c_uchar,
    9 as libc::c_int as libc::c_uchar,
    3 as libc::c_int as libc::c_uchar,
    15 as libc::c_int as libc::c_uchar,
    3 as libc::c_int as libc::c_uchar,
    39 as libc::c_int as libc::c_uchar,
    5 as libc::c_int as libc::c_uchar,
    39 as libc::c_int as libc::c_uchar,
    57 as libc::c_int as libc::c_uchar,
    3 as libc::c_int as libc::c_uchar,
    35 as libc::c_int as libc::c_uchar,
    1 as libc::c_int as libc::c_uchar,
    5 as libc::c_int as libc::c_uchar,
    9 as libc::c_int as libc::c_uchar,
    41 as libc::c_int as libc::c_uchar,
    31 as libc::c_int as libc::c_uchar,
    5 as libc::c_int as libc::c_uchar,
    25 as libc::c_int as libc::c_uchar,
    45 as libc::c_int as libc::c_uchar,
    7 as libc::c_int as libc::c_uchar,
    87 as libc::c_int as libc::c_uchar,
    21 as libc::c_int as libc::c_uchar,
    11 as libc::c_int as libc::c_uchar,
    57 as libc::c_int as libc::c_uchar,
    17 as libc::c_int as libc::c_uchar,
    55 as libc::c_int as libc::c_uchar,
    21 as libc::c_int as libc::c_uchar,
    115 as libc::c_int as libc::c_uchar,
    59 as libc::c_int as libc::c_uchar,
    81 as libc::c_int as libc::c_uchar,
    27 as libc::c_int as libc::c_uchar,
    129 as libc::c_int as libc::c_uchar,
    47 as libc::c_int as libc::c_uchar,
    111 as libc::c_int as libc::c_uchar,
    33 as libc::c_int as libc::c_uchar,
    55 as libc::c_int as libc::c_uchar,
    5 as libc::c_int as libc::c_uchar,
    13 as libc::c_int as libc::c_uchar,
    27 as libc::c_int as libc::c_uchar,
    55 as libc::c_int as libc::c_uchar,
    93 as libc::c_int as libc::c_uchar,
    1 as libc::c_int as libc::c_uchar,
    57 as libc::c_int as libc::c_uchar,
    25 as libc::c_int as libc::c_uchar,
];
pub unsafe extern "C" fn read_files(
    mut filevec: *mut file_data,
    mut pretend_binary: bool,
) -> bool {
    let mut i: libc::c_int = 0;
    let mut skip_test: bool = text as libc::c_int | pretend_binary as libc::c_int != 0;
    let mut appears_binary: bool = pretend_binary as libc::c_int
        | sip(&mut *filevec.offset(0 as libc::c_int as isize), skip_test) as libc::c_int
        != 0;
    if (*filevec.offset(0 as libc::c_int as isize)).desc
        != (*filevec.offset(1 as libc::c_int as isize)).desc
    {
        appears_binary = (appears_binary as libc::c_int
            | sip(
                &mut *filevec.offset(1 as libc::c_int as isize),
                skip_test as libc::c_int | appears_binary as libc::c_int != 0,
            ) as libc::c_int) != 0;
    } else {
        let ref mut fresh37 = (*filevec.offset(1 as libc::c_int as isize)).buffer;
        *fresh37 = (*filevec.offset(0 as libc::c_int as isize)).buffer;
        (*filevec.offset(1 as libc::c_int as isize))
            .bufsize = (*filevec.offset(0 as libc::c_int as isize)).bufsize;
        (*filevec.offset(1 as libc::c_int as isize))
            .buffered = (*filevec.offset(0 as libc::c_int as isize)).buffered;
    }
    if appears_binary {
        set_binary_mode(
            (*filevec.offset(0 as libc::c_int as isize)).desc,
            0 as libc::c_int,
        );
        set_binary_mode(
            (*filevec.offset(1 as libc::c_int as isize)).desc,
            0 as libc::c_int,
        );
        return 1 as libc::c_int != 0;
    }
    find_identical_ends(filevec);
    equivs_alloc = (*filevec.offset(0 as libc::c_int as isize)).alloc_lines
        + (*filevec.offset(1 as libc::c_int as isize)).alloc_lines
        + 1 as libc::c_int as libc::c_long;
    equivs = xnmalloc(
        equivs_alloc as size_t,
        ::std::mem::size_of::<equivclass>() as libc::c_ulong,
    ) as *mut equivclass;
    equivs_index = 1 as libc::c_int as lin;
    i = 9 as libc::c_int;
    while (1 as libc::c_int as size_t) << i
        < (equivs_alloc / 3 as libc::c_int as libc::c_long) as libc::c_ulong
    {
        i += 1;
        i;
    }
    nbuckets = ((1 as libc::c_int as size_t) << i)
        .wrapping_sub(prime_offset[i as usize] as libc::c_ulong);
    buckets = xcalloc(
        nbuckets.wrapping_add(1 as libc::c_int as libc::c_ulong),
        ::std::mem::size_of::<lin>() as libc::c_ulong,
    ) as *mut lin;
    buckets = buckets.offset(1);
    buckets;
    i = 0 as libc::c_int;
    while i < 2 as libc::c_int {
        find_and_hash_each_line(&mut *filevec.offset(i as isize));
        i += 1;
        i;
    }
    let ref mut fresh38 = (*filevec.offset(1 as libc::c_int as isize)).equiv_max;
    *fresh38 = equivs_index;
    (*filevec.offset(0 as libc::c_int as isize)).equiv_max = *fresh38;
    rpl_free(equivs as *mut libc::c_void);
    rpl_free(buckets.offset(-(1 as libc::c_int as isize)) as *mut libc::c_void);
    return 0 as libc::c_int != 0;
}
