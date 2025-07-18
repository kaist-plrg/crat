use ::libc;
use ::c2rust_bitfields;
extern "C" {
    pub type tm_zone;
    pub type re_dfa_t;
    pub type _IO_wide_data;
    pub type _IO_codecvt;
    pub type _IO_marker;
    fn localtime(__timer: *const time_t) -> *mut tm;
    fn abort() -> !;
    fn rpl_re_search(
        __buffer: *mut re_pattern_buffer,
        __String: *const libc::c_char,
        __length: regoff_t,
        __start: regoff_t,
        __range: regoff_t,
        __regs: *mut re_registers,
    ) -> regoff_t;
    fn fprintf(_: *mut FILE, _: *const libc::c_char, _: ...) -> libc::c_int;
    fn sprintf(_: *mut libc::c_char, _: *const libc::c_char, _: ...) -> libc::c_int;
    fn print_1_line(_: *const libc::c_char, _: *const *const libc::c_char);
    fn print_script(
        _: *mut change,
        _: Option::<unsafe extern "C" fn(*mut change) -> *mut change>,
        _: Option::<unsafe extern "C" fn(*mut change) -> ()>,
    );
    fn begin_output();
    fn translate_range(_: *const file_data, _: lin, _: lin, _: *mut lin, _: *mut lin);
    fn analyze_hunk(
        _: *mut change,
        _: *mut lin,
        _: *mut lin,
        _: *mut lin,
        _: *mut lin,
    ) -> changes;
    fn print_1_line_nl(_: *const libc::c_char, _: *const *const libc::c_char, _: bool);
    fn set_color_context(color_context: color_context);
    static mut outfile: *mut FILE;
    static mut files: [file_data; 2];
    static mut time_format: *const libc::c_char;
    static mut context: lin;
    static mut ignore_blank_lines: bool;
    static mut file_label: [*mut libc::c_char; 2];
    static mut function_regexp: re_pattern_buffer;
    static mut ignore_regexp: re_pattern_buffer;
    static mut initial_tab: bool;
    static mut suppress_blank_empty: bool;
    fn fputs_unlocked(__s: *const libc::c_char, __stream: *mut FILE) -> libc::c_int;
    fn fwrite_unlocked(
        __ptr: *const libc::c_void,
        __size: size_t,
        __n: size_t,
        __stream: *mut FILE,
    ) -> size_t;
    fn __overflow(_: *mut FILE, _: libc::c_int) -> libc::c_int;
    fn nstrftime(
        _: *mut libc::c_char,
        _: size_t,
        _: *const libc::c_char,
        _: *const tm,
        __tz: timezone_t,
        __ns: libc::c_int,
    ) -> size_t;
}
pub type __intmax_t = libc::c_long;
pub type __uintmax_t = libc::c_ulong;
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
pub type __ssize_t = libc::c_long;
pub type __syscall_slong_t = libc::c_long;
pub type ssize_t = __ssize_t;
pub type time_t = __time_t;
pub type size_t = libc::c_ulong;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct timespec {
    pub tv_sec: __time_t,
    pub tv_nsec: __syscall_slong_t,
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
pub type ptrdiff_t = libc::c_long;
pub type timezone_t = *mut tm_zone;
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
pub type intmax_t = __intmax_t;
pub type uintmax_t = __uintmax_t;
pub type lin = ptrdiff_t;
pub type __re_size_t = size_t;
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
pub type regoff_t = ssize_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct re_registers {
    pub num_regs: __re_size_t,
    pub start: *mut regoff_t,
    pub end: *mut regoff_t,
}
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
pub type changes = libc::c_uint;
pub const CHANGED: changes = 3;
pub const NEW: changes = 2;
pub const OLD: changes = 1;
pub const UNCHANGED: changes = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct change {
    pub link: *mut change,
    pub inserted: lin,
    pub deleted: lin,
    pub line0: lin,
    pub line1: lin,
    pub ignore: bool,
}
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
pub type color_context = libc::c_uint;
pub const LINE_NUMBER_CONTEXT: color_context = 4;
pub const RESET_CONTEXT: color_context = 3;
pub const DELETE_CONTEXT: color_context = 2;
pub const ADD_CONTEXT: color_context = 1;
pub const HEADER_CONTEXT: color_context = 0;
#[inline]
unsafe extern "C" fn putc_unlocked(
    mut __c: libc::c_int,
    mut __stream: *mut FILE,
) -> libc::c_int {
    return if ((*__stream)._IO_write_ptr >= (*__stream)._IO_write_end) as libc::c_int
        as libc::c_long != 0
    {
        __overflow(__stream, __c as libc::c_uchar as libc::c_int)
    } else {
        let fresh0 = (*__stream)._IO_write_ptr;
        (*__stream)._IO_write_ptr = ((*__stream)._IO_write_ptr).offset(1);
        *fresh0 = __c as libc::c_char;
        *fresh0 as libc::c_uchar as libc::c_int
    };
}
#[inline]
unsafe extern "C" fn c_isspace(mut c: libc::c_int) -> bool {
    match c {
        32 | 9 | 10 | 11 | 12 | 13 => return 1 as libc::c_int != 0,
        _ => return 0 as libc::c_int != 0,
    };
}
#[inline]
unsafe extern "C" fn get_stat_mtime_ns(mut st: *const stat) -> libc::c_long {
    return (*st).st_mtim.tv_nsec;
}
static mut find_function_last_search: lin = 0;
static mut find_function_last_match: lin = 0;
unsafe extern "C" fn print_context_label(
    mut mark: *const libc::c_char,
    mut inf: *mut file_data,
    mut name: *const libc::c_char,
    mut label: *const libc::c_char,
) {
    set_color_context(HEADER_CONTEXT);
    if !label.is_null() {
        fprintf(outfile, b"%s %s\0" as *const u8 as *const libc::c_char, mark, label);
    } else {
        let mut buf: [libc::c_char; 43] = [0; 43];
        let mut tm: *const tm = localtime(&mut (*inf).stat.st_mtim.tv_sec);
        let mut nsec: libc::c_int = get_stat_mtime_ns(&mut (*inf).stat) as libc::c_int;
        if !(!tm.is_null()
            && nstrftime(
                buf.as_mut_ptr(),
                ::std::mem::size_of::<[libc::c_char; 43]>() as libc::c_ulong,
                time_format,
                tm,
                0 as timezone_t,
                nsec,
            ) != 0)
        {
            if -(9223372036854775807 as libc::c_long) - 1 as libc::c_long
                <= !(if (0 as libc::c_int as time_t) < -(1 as libc::c_int) as time_t {
                    -(1 as libc::c_int) as time_t
                } else {
                    (((1 as libc::c_int as time_t)
                        << (::std::mem::size_of::<time_t>() as libc::c_ulong)
                            .wrapping_mul(8 as libc::c_int as libc::c_ulong)
                            .wrapping_sub(2 as libc::c_int as libc::c_ulong))
                        - 1 as libc::c_int as libc::c_long)
                        * 2 as libc::c_int as libc::c_long
                        + 1 as libc::c_int as libc::c_long
                })
                && (if (0 as libc::c_int as time_t) < -(1 as libc::c_int) as time_t {
                    -(1 as libc::c_int) as time_t
                } else {
                    (((1 as libc::c_int as time_t)
                        << (::std::mem::size_of::<time_t>() as libc::c_ulong)
                            .wrapping_mul(8 as libc::c_int as libc::c_ulong)
                            .wrapping_sub(2 as libc::c_int as libc::c_ulong))
                        - 1 as libc::c_int as libc::c_long)
                        * 2 as libc::c_int as libc::c_long
                        + 1 as libc::c_int as libc::c_long
                }) <= 9223372036854775807 as libc::c_long
            {
                let mut sec: libc::c_long = (*inf).stat.st_mtim.tv_sec;
                sprintf(
                    buf.as_mut_ptr(),
                    b"%ld.%.9d\0" as *const u8 as *const libc::c_char,
                    sec,
                    nsec,
                );
            } else if (if (0 as libc::c_int as time_t) < -(1 as libc::c_int) as time_t {
                -(1 as libc::c_int) as time_t
            } else {
                (((1 as libc::c_int as time_t)
                    << (::std::mem::size_of::<time_t>() as libc::c_ulong)
                        .wrapping_mul(8 as libc::c_int as libc::c_ulong)
                        .wrapping_sub(2 as libc::c_int as libc::c_ulong))
                    - 1 as libc::c_int as libc::c_long)
                    * 2 as libc::c_int as libc::c_long + 1 as libc::c_int as libc::c_long
            }) <= 9223372036854775807 as libc::c_long
            {
                let mut sec_0: intmax_t = (*inf).stat.st_mtim.tv_sec;
                sprintf(
                    buf.as_mut_ptr(),
                    b"%ld.%.9d\0" as *const u8 as *const libc::c_char,
                    sec_0,
                    nsec,
                );
            } else {
                let mut sec_1: uintmax_t = (*inf).stat.st_mtim.tv_sec as uintmax_t;
                sprintf(
                    buf.as_mut_ptr(),
                    b"%lu.%.9d\0" as *const u8 as *const libc::c_char,
                    sec_1,
                    nsec,
                );
            }
        }
        fprintf(
            outfile,
            b"%s %s\t%s\0" as *const u8 as *const libc::c_char,
            mark,
            name,
            buf.as_mut_ptr(),
        );
    }
    set_color_context(RESET_CONTEXT);
    putc_unlocked('\n' as i32, outfile);
}
pub unsafe extern "C" fn print_context_header(
    mut inf: *mut file_data,
    mut names: *const *const libc::c_char,
    mut unidiff: bool,
) {
    if unidiff {
        print_context_label(
            b"---\0" as *const u8 as *const libc::c_char,
            &mut *inf.offset(0 as libc::c_int as isize),
            *names.offset(0 as libc::c_int as isize),
            file_label[0 as libc::c_int as usize],
        );
        print_context_label(
            b"+++\0" as *const u8 as *const libc::c_char,
            &mut *inf.offset(1 as libc::c_int as isize),
            *names.offset(1 as libc::c_int as isize),
            file_label[1 as libc::c_int as usize],
        );
    } else {
        print_context_label(
            b"***\0" as *const u8 as *const libc::c_char,
            &mut *inf.offset(0 as libc::c_int as isize),
            *names.offset(0 as libc::c_int as isize),
            file_label[0 as libc::c_int as usize],
        );
        print_context_label(
            b"---\0" as *const u8 as *const libc::c_char,
            &mut *inf.offset(1 as libc::c_int as isize),
            *names.offset(1 as libc::c_int as isize),
            file_label[1 as libc::c_int as usize],
        );
    };
}
pub unsafe extern "C" fn print_context_script(
    mut script: *mut change,
    mut unidiff: bool,
) {
    if ignore_blank_lines as libc::c_int != 0 || !(ignore_regexp.fastmap).is_null() {
        mark_ignorable(script);
    } else {
        let mut e: *mut change = 0 as *mut change;
        e = script;
        while !e.is_null() {
            (*e).ignore = 0 as libc::c_int != 0;
            e = (*e).link;
        }
    }
    find_function_last_search = -files[0 as libc::c_int as usize].prefix_lines;
    find_function_last_match = 9223372036854775807 as libc::c_long;
    if unidiff {
        print_script(
            script,
            Some(find_hunk as unsafe extern "C" fn(*mut change) -> *mut change),
            Some(pr_unidiff_hunk as unsafe extern "C" fn(*mut change) -> ()),
        );
    } else {
        print_script(
            script,
            Some(find_hunk as unsafe extern "C" fn(*mut change) -> *mut change),
            Some(pr_context_hunk as unsafe extern "C" fn(*mut change) -> ()),
        );
    };
}
unsafe extern "C" fn print_context_number_range(
    mut file: *const file_data,
    mut a: lin,
    mut b: lin,
) {
    let mut trans_a: lin = 0;
    let mut trans_b: lin = 0;
    translate_range(file, a, b, &mut trans_a, &mut trans_b);
    if trans_b <= trans_a {
        fprintf(outfile, b"%td\0" as *const u8 as *const libc::c_char, trans_b);
    } else {
        fprintf(
            outfile,
            b"%td,%td\0" as *const u8 as *const libc::c_char,
            trans_a,
            trans_b,
        );
    };
}
unsafe extern "C" fn print_context_function(
    mut out: *mut FILE,
    mut function: *const libc::c_char,
) {
    let mut i: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    putc_unlocked(' ' as i32, out);
    i = 0 as libc::c_int;
    while c_isspace(*function.offset(i as isize) as libc::c_uchar as libc::c_int)
        as libc::c_int != 0 && *function.offset(i as isize) as libc::c_int != '\n' as i32
    {
        i += 1;
        i;
    }
    j = i;
    while j < i + 40 as libc::c_int
        && *function.offset(j as isize) as libc::c_int != '\n' as i32
    {
        j += 1;
        j;
    }
    while i < j
        && c_isspace(
            *function.offset((j - 1 as libc::c_int) as isize) as libc::c_uchar
                as libc::c_int,
        ) as libc::c_int != 0
    {
        j -= 1;
        j;
    }
    if 0 != 0 && 0 != 0
        && (::std::mem::size_of::<libc::c_char>() as libc::c_ulong)
            .wrapping_mul((j - i) as size_t) <= 8 as libc::c_int as libc::c_ulong
        && ::std::mem::size_of::<libc::c_char>() as libc::c_ulong
            != 0 as libc::c_int as libc::c_ulong
    {
        ({
            let mut __ptr: *const libc::c_char = function.offset(i as isize);
            let mut __stream: *mut FILE = out;
            let mut __cnt: size_t = 0;
            __cnt = (::std::mem::size_of::<libc::c_char>() as libc::c_ulong)
                .wrapping_mul((j - i) as size_t);
            while __cnt > 0 as libc::c_int as libc::c_ulong {
                let fresh1 = __ptr;
                __ptr = __ptr.offset(1);
                if putc_unlocked(*fresh1 as libc::c_int, __stream) == -(1 as libc::c_int)
                {
                    break;
                }
                __cnt = __cnt.wrapping_sub(1);
                __cnt;
            }
            0u8
        });
    } else {
        if 0 != 0
            && ::std::mem::size_of::<libc::c_char>() as libc::c_ulong
                == 0 as libc::c_int as libc::c_ulong
            || 0 != 0 && (j - i) as size_t == 0 as libc::c_int as libc::c_ulong
        {} else {
            fwrite_unlocked(
                function.offset(i as isize) as *const libc::c_void,
                ::std::mem::size_of::<libc::c_char>() as libc::c_ulong,
                (j - i) as size_t,
                out,
            );
        };
    };
    0u8;
}
unsafe extern "C" fn pr_context_hunk(mut hunk: *mut change) {
    let mut first0: lin = 0;
    let mut last0: lin = 0;
    let mut first1: lin = 0;
    let mut last1: lin = 0;
    let mut i: lin = 0;
    let mut prefix: *const libc::c_char = 0 as *const libc::c_char;
    let mut function: *const libc::c_char = 0 as *const libc::c_char;
    let mut out: *mut FILE = 0 as *mut FILE;
    let mut changes: changes = analyze_hunk(
        hunk,
        &mut first0,
        &mut last0,
        &mut first1,
        &mut last1,
    );
    if changes as u64 == 0 {
        return;
    }
    i = -files[0 as libc::c_int as usize].prefix_lines;
    first0 = if first0 - context >= i { first0 - context } else { i };
    first1 = if first1 - context >= i { first1 - context } else { i };
    if last0 < files[0 as libc::c_int as usize].valid_lines - context {
        last0 += context;
    } else {
        last0 = files[0 as libc::c_int as usize].valid_lines
            - 1 as libc::c_int as libc::c_long;
    }
    if last1 < files[1 as libc::c_int as usize].valid_lines - context {
        last1 += context;
    } else {
        last1 = files[1 as libc::c_int as usize].valid_lines
            - 1 as libc::c_int as libc::c_long;
    }
    function = 0 as *const libc::c_char;
    if !(function_regexp.fastmap).is_null() {
        function = find_function(files[0 as libc::c_int as usize].linbuf, first0);
    }
    begin_output();
    out = outfile;
    fputs_unlocked(b"***************\0" as *const u8 as *const libc::c_char, out);
    if !function.is_null() {
        print_context_function(out, function);
    }
    putc_unlocked('\n' as i32, out);
    set_color_context(LINE_NUMBER_CONTEXT);
    fputs_unlocked(b"*** \0" as *const u8 as *const libc::c_char, out);
    print_context_number_range(
        &mut *files.as_mut_ptr().offset(0 as libc::c_int as isize),
        first0,
        last0,
    );
    fputs_unlocked(b" ****\0" as *const u8 as *const libc::c_char, out);
    set_color_context(RESET_CONTEXT);
    putc_unlocked('\n' as i32, out);
    if changes as libc::c_uint & OLD as libc::c_int as libc::c_uint != 0 {
        let mut next: *mut change = hunk;
        i = first0;
        while i <= last0 {
            set_color_context(DELETE_CONTEXT);
            while !next.is_null() && (*next).line0 + (*next).deleted <= i {
                next = (*next).link;
            }
            prefix = b" \0" as *const u8 as *const libc::c_char;
            if !next.is_null() && (*next).line0 <= i {
                prefix = if (*next).inserted > 0 as libc::c_int as libc::c_long {
                    b"!\0" as *const u8 as *const libc::c_char
                } else {
                    b"-\0" as *const u8 as *const libc::c_char
                };
            }
            print_1_line_nl(
                prefix,
                &mut *((*files.as_mut_ptr().offset(0 as libc::c_int as isize)).linbuf)
                    .offset(i as isize),
                1 as libc::c_int != 0,
            );
            set_color_context(RESET_CONTEXT);
            if *(*(files[0 as libc::c_int as usize].linbuf)
                .offset((i + 1 as libc::c_int as libc::c_long) as isize))
                .offset(-(1 as libc::c_int) as isize) as libc::c_int == '\n' as i32
            {
                putc_unlocked('\n' as i32, out);
            }
            i += 1;
            i;
        }
    }
    set_color_context(LINE_NUMBER_CONTEXT);
    fputs_unlocked(b"--- \0" as *const u8 as *const libc::c_char, out);
    print_context_number_range(
        &mut *files.as_mut_ptr().offset(1 as libc::c_int as isize),
        first1,
        last1,
    );
    fputs_unlocked(b" ----\0" as *const u8 as *const libc::c_char, out);
    set_color_context(RESET_CONTEXT);
    putc_unlocked('\n' as i32, out);
    if changes as libc::c_uint & NEW as libc::c_int as libc::c_uint != 0 {
        let mut next_0: *mut change = hunk;
        i = first1;
        while i <= last1 {
            set_color_context(ADD_CONTEXT);
            while !next_0.is_null() && (*next_0).line1 + (*next_0).inserted <= i {
                next_0 = (*next_0).link;
            }
            prefix = b" \0" as *const u8 as *const libc::c_char;
            if !next_0.is_null() && (*next_0).line1 <= i {
                prefix = if (*next_0).deleted > 0 as libc::c_int as libc::c_long {
                    b"!\0" as *const u8 as *const libc::c_char
                } else {
                    b"+\0" as *const u8 as *const libc::c_char
                };
            }
            print_1_line_nl(
                prefix,
                &mut *((*files.as_mut_ptr().offset(1 as libc::c_int as isize)).linbuf)
                    .offset(i as isize),
                1 as libc::c_int != 0,
            );
            set_color_context(RESET_CONTEXT);
            if *(*(files[1 as libc::c_int as usize].linbuf)
                .offset((i + 1 as libc::c_int as libc::c_long) as isize))
                .offset(-(1 as libc::c_int) as isize) as libc::c_int == '\n' as i32
            {
                putc_unlocked('\n' as i32, out);
            }
            i += 1;
            i;
        }
    }
}
unsafe extern "C" fn print_unidiff_number_range(
    mut file: *const file_data,
    mut a: lin,
    mut b: lin,
) {
    let mut trans_a: lin = 0;
    let mut trans_b: lin = 0;
    translate_range(file, a, b, &mut trans_a, &mut trans_b);
    if trans_b <= trans_a {
        fprintf(
            outfile,
            if trans_b < trans_a {
                b"%td,0\0" as *const u8 as *const libc::c_char
            } else {
                b"%td\0" as *const u8 as *const libc::c_char
            },
            trans_b,
        );
    } else {
        fprintf(
            outfile,
            b"%td,%td\0" as *const u8 as *const libc::c_char,
            trans_a,
            trans_b - trans_a + 1 as libc::c_int as libc::c_long,
        );
    };
}
unsafe extern "C" fn pr_unidiff_hunk(mut hunk: *mut change) {
    let mut first0: lin = 0;
    let mut last0: lin = 0;
    let mut first1: lin = 0;
    let mut last1: lin = 0;
    let mut i: lin = 0;
    let mut j: lin = 0;
    let mut k: lin = 0;
    let mut next: *mut change = 0 as *mut change;
    let mut function: *const libc::c_char = 0 as *const libc::c_char;
    let mut out: *mut FILE = 0 as *mut FILE;
    if analyze_hunk(hunk, &mut first0, &mut last0, &mut first1, &mut last1) as u64 == 0 {
        return;
    }
    i = -files[0 as libc::c_int as usize].prefix_lines;
    first0 = if first0 - context >= i { first0 - context } else { i };
    first1 = if first1 - context >= i { first1 - context } else { i };
    if last0 < files[0 as libc::c_int as usize].valid_lines - context {
        last0 += context;
    } else {
        last0 = files[0 as libc::c_int as usize].valid_lines
            - 1 as libc::c_int as libc::c_long;
    }
    if last1 < files[1 as libc::c_int as usize].valid_lines - context {
        last1 += context;
    } else {
        last1 = files[1 as libc::c_int as usize].valid_lines
            - 1 as libc::c_int as libc::c_long;
    }
    function = 0 as *const libc::c_char;
    if !(function_regexp.fastmap).is_null() {
        function = find_function(files[0 as libc::c_int as usize].linbuf, first0);
    }
    begin_output();
    out = outfile;
    set_color_context(LINE_NUMBER_CONTEXT);
    fputs_unlocked(b"@@ -\0" as *const u8 as *const libc::c_char, out);
    print_unidiff_number_range(
        &mut *files.as_mut_ptr().offset(0 as libc::c_int as isize),
        first0,
        last0,
    );
    fputs_unlocked(b" +\0" as *const u8 as *const libc::c_char, out);
    print_unidiff_number_range(
        &mut *files.as_mut_ptr().offset(1 as libc::c_int as isize),
        first1,
        last1,
    );
    fputs_unlocked(b" @@\0" as *const u8 as *const libc::c_char, out);
    set_color_context(RESET_CONTEXT);
    if !function.is_null() {
        print_context_function(out, function);
    }
    putc_unlocked('\n' as i32, out);
    next = hunk;
    i = first0;
    j = first1;
    while i <= last0 || j <= last1 {
        if next.is_null() || i < (*next).line0 {
            let fresh2 = i;
            i = i + 1;
            let mut line: *const *const libc::c_char = &mut *((*files
                .as_mut_ptr()
                .offset(0 as libc::c_int as isize))
                .linbuf)
                .offset(fresh2 as isize) as *mut *const libc::c_char;
            if !(suppress_blank_empty as libc::c_int != 0
                && **line as libc::c_int == '\n' as i32)
            {
                putc_unlocked(
                    if initial_tab as libc::c_int != 0 {
                        '\t' as i32
                    } else {
                        ' ' as i32
                    },
                    out,
                );
            }
            print_1_line(0 as *const libc::c_char, line);
            j += 1;
            j;
        } else {
            k = (*next).deleted;
            loop {
                let fresh3 = k;
                k = k - 1;
                if !(fresh3 != 0) {
                    break;
                }
                let fresh4 = i;
                i = i + 1;
                let mut line_0: *const *const libc::c_char = &mut *((*files
                    .as_mut_ptr()
                    .offset(0 as libc::c_int as isize))
                    .linbuf)
                    .offset(fresh4 as isize) as *mut *const libc::c_char;
                set_color_context(DELETE_CONTEXT);
                putc_unlocked('-' as i32, out);
                if initial_tab as libc::c_int != 0
                    && !(suppress_blank_empty as libc::c_int != 0
                        && **line_0 as libc::c_int == '\n' as i32)
                {
                    putc_unlocked('\t' as i32, out);
                }
                print_1_line_nl(0 as *const libc::c_char, line_0, 1 as libc::c_int != 0);
                set_color_context(RESET_CONTEXT);
                if *(*line_0.offset(1 as libc::c_int as isize))
                    .offset(-(1 as libc::c_int) as isize) as libc::c_int == '\n' as i32
                {
                    putc_unlocked('\n' as i32, out);
                }
            }
            k = (*next).inserted;
            loop {
                let fresh5 = k;
                k = k - 1;
                if !(fresh5 != 0) {
                    break;
                }
                let fresh6 = j;
                j = j + 1;
                let mut line_1: *const *const libc::c_char = &mut *((*files
                    .as_mut_ptr()
                    .offset(1 as libc::c_int as isize))
                    .linbuf)
                    .offset(fresh6 as isize) as *mut *const libc::c_char;
                set_color_context(ADD_CONTEXT);
                putc_unlocked('+' as i32, out);
                if initial_tab as libc::c_int != 0
                    && !(suppress_blank_empty as libc::c_int != 0
                        && **line_1 as libc::c_int == '\n' as i32)
                {
                    putc_unlocked('\t' as i32, out);
                }
                print_1_line_nl(0 as *const libc::c_char, line_1, 1 as libc::c_int != 0);
                set_color_context(RESET_CONTEXT);
                if *(*line_1.offset(1 as libc::c_int as isize))
                    .offset(-(1 as libc::c_int) as isize) as libc::c_int == '\n' as i32
                {
                    putc_unlocked('\n' as i32, out);
                }
            }
            next = (*next).link;
        }
    }
}
unsafe extern "C" fn find_hunk(mut start: *mut change) -> *mut change {
    let mut prev: *mut change = 0 as *mut change;
    let mut top0: lin = 0;
    let mut top1: lin = 0;
    let mut thresh: lin = 0;
    let mut ignorable_threshold: lin = context;
    let mut non_ignorable_threshold: lin = 2 as libc::c_int as libc::c_long * context
        + 1 as libc::c_int as libc::c_long;
    loop {
        top0 = (*start).line0 + (*start).deleted;
        top1 = (*start).line1 + (*start).inserted;
        prev = start;
        start = (*start).link;
        thresh = if !start.is_null() && (*start).ignore as libc::c_int != 0 {
            ignorable_threshold
        } else {
            non_ignorable_threshold
        };
        if !start.is_null() && (*start).line0 - top0 != (*start).line1 - top1 {
            abort();
        }
        if !(!start.is_null() && (*start).line0 - top0 < thresh) {
            break;
        }
    }
    return prev;
}
unsafe extern "C" fn mark_ignorable(mut script: *mut change) {
    while !script.is_null() {
        let mut next: *mut change = (*script).link;
        let mut first0: lin = 0;
        let mut last0: lin = 0;
        let mut first1: lin = 0;
        let mut last1: lin = 0;
        (*script).link = 0 as *mut change;
        (*script)
            .ignore = analyze_hunk(
            script,
            &mut first0,
            &mut last0,
            &mut first1,
            &mut last1,
        ) as u64 == 0;
        (*script).link = next;
        script = next;
    }
}
unsafe extern "C" fn find_function(
    mut linbuf: *const *const libc::c_char,
    mut linenum: lin,
) -> *const libc::c_char {
    let mut i: lin = linenum;
    let mut last: lin = find_function_last_search;
    find_function_last_search = i;
    loop {
        i -= 1;
        if !(last <= i) {
            break;
        }
        let mut line: *const libc::c_char = *linbuf.offset(i as isize);
        let mut linelen: size_t = ((*linbuf
            .offset((i + 1 as libc::c_int as libc::c_long) as isize))
            .offset_from(line) as libc::c_long - 1 as libc::c_int as libc::c_long)
            as size_t;
        let mut len: regoff_t = (if linelen
            <= (if (0 as libc::c_int as regoff_t) < -(1 as libc::c_int) as regoff_t {
                -(1 as libc::c_int) as regoff_t
            } else {
                (((1 as libc::c_int as regoff_t)
                    << (::std::mem::size_of::<regoff_t>() as libc::c_ulong)
                        .wrapping_mul(8 as libc::c_int as libc::c_ulong)
                        .wrapping_sub(2 as libc::c_int as libc::c_ulong))
                    - 1 as libc::c_int as libc::c_long)
                    * 2 as libc::c_int as libc::c_long + 1 as libc::c_int as libc::c_long
            }) as libc::c_ulong
        {
            linelen
        } else {
            (if (0 as libc::c_int as regoff_t) < -(1 as libc::c_int) as regoff_t {
                -(1 as libc::c_int) as regoff_t
            } else {
                (((1 as libc::c_int as regoff_t)
                    << (::std::mem::size_of::<regoff_t>() as libc::c_ulong)
                        .wrapping_mul(8 as libc::c_int as libc::c_ulong)
                        .wrapping_sub(2 as libc::c_int as libc::c_ulong))
                    - 1 as libc::c_int as libc::c_long)
                    * 2 as libc::c_int as libc::c_long + 1 as libc::c_int as libc::c_long
            }) as libc::c_ulong
        }) as regoff_t;
        if 0 as libc::c_int as libc::c_long
            <= rpl_re_search(
                &mut function_regexp,
                line,
                len,
                0 as libc::c_int as regoff_t,
                len,
                0 as *mut re_registers,
            )
        {
            find_function_last_match = i;
            return line;
        }
    }
    if find_function_last_match != 9223372036854775807 as libc::c_long {
        return *linbuf.offset(find_function_last_match as isize);
    }
    return 0 as *const libc::c_char;
}
