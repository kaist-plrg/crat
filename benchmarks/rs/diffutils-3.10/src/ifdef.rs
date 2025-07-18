use ::libc;
extern "C" {
    pub type _IO_wide_data;
    pub type _IO_codecvt;
    pub type _IO_marker;
    fn __strtol_internal(
        __nptr: *const libc::c_char,
        __endptr: *mut *mut libc::c_char,
        __base: libc::c_int,
        __group: libc::c_int,
    ) -> libc::c_long;
    fn mempcpy(
        _: *mut libc::c_void,
        _: *const libc::c_void,
        _: libc::c_ulong,
    ) -> *mut libc::c_void;
    fn stpcpy(_: *mut libc::c_char, _: *const libc::c_char) -> *mut libc::c_char;
    fn __errno_location() -> *mut libc::c_int;
    fn fprintf(_: *mut FILE, _: *const libc::c_char, _: ...) -> libc::c_int;
    fn __overflow(_: *mut FILE, _: libc::c_int) -> libc::c_int;
    fn fwrite_unlocked(
        __ptr: *const libc::c_void,
        __size: size_t,
        __n: size_t,
        __stream: *mut FILE,
    ) -> size_t;
    static mut expand_tabs: bool;
    static mut group_format: [*const libc::c_char; 4];
    static mut line_format: [*const libc::c_char; 3];
    static mut files: [file_data; 2];
    static mut outfile: *mut FILE;
    fn translate_line_number(_: *const file_data, _: lin) -> lin;
    fn output_1_line(
        _: *const libc::c_char,
        _: *const libc::c_char,
        _: *const libc::c_char,
        _: *const libc::c_char,
    );
    fn begin_output();
    fn analyze_hunk(
        _: *mut change,
        _: *mut lin,
        _: *mut lin,
        _: *mut lin,
        _: *mut lin,
    ) -> changes;
    fn find_change(_: *mut change) -> *mut change;
    fn print_script(
        _: *mut change,
        _: Option::<unsafe extern "C" fn(*mut change) -> *mut change>,
        _: Option::<unsafe extern "C" fn(*mut change) -> ()>,
    );
    fn xmmalloca(n: size_t) -> *mut libc::c_void;
    fn freea(p: *mut libc::c_void);
}
pub type __intmax_t = libc::c_long;
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
pub type uintptr_t = libc::c_ulong;
pub type intmax_t = __intmax_t;
pub type lin = ptrdiff_t;
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct group {
    pub file: *const file_data,
    pub from: lin,
    pub upto: lin,
}
pub const sa_alignment_max: C2RustUnnamed = 16;
pub type C2RustUnnamed = libc::c_uint;
pub const sa_alignment_longdouble: C2RustUnnamed = 16;
pub const sa_alignment_longlong: C2RustUnnamed = 8;
pub const sa_alignment_double: C2RustUnnamed = 8;
pub const sa_alignment_long: C2RustUnnamed = 8;
#[inline]
unsafe extern "C" fn strtoimax(
    mut nptr: *const libc::c_char,
    mut endptr: *mut *mut libc::c_char,
    mut base: libc::c_int,
) -> intmax_t {
    return __strtol_internal(nptr, endptr, base, 0 as libc::c_int);
}
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
static mut next_line0: lin = 0;
static mut next_line1: lin = 0;
pub unsafe extern "C" fn print_ifdef_script(mut script: *mut change) {
    next_line1 = -files[0 as libc::c_int as usize].prefix_lines;
    next_line0 = next_line1;
    print_script(
        script,
        Some(find_change as unsafe extern "C" fn(*mut change) -> *mut change),
        Some(print_ifdef_hunk as unsafe extern "C" fn(*mut change) -> ()),
    );
    if next_line0 < files[0 as libc::c_int as usize].valid_lines
        || next_line1 < files[1 as libc::c_int as usize].valid_lines
    {
        begin_output();
        format_ifdef(
            group_format[UNCHANGED as libc::c_int as usize],
            next_line0,
            files[0 as libc::c_int as usize].valid_lines,
            next_line1,
            files[1 as libc::c_int as usize].valid_lines,
        );
    }
}
unsafe extern "C" fn print_ifdef_hunk(mut hunk: *mut change) {
    let mut first0: lin = 0;
    let mut last0: lin = 0;
    let mut first1: lin = 0;
    let mut last1: lin = 0;
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
    begin_output();
    if next_line0 < first0 || next_line1 < first1 {
        format_ifdef(
            group_format[UNCHANGED as libc::c_int as usize],
            next_line0,
            first0,
            next_line1,
            first1,
        );
    }
    next_line0 = last0 + 1 as libc::c_int as libc::c_long;
    next_line1 = last1 + 1 as libc::c_int as libc::c_long;
    format_ifdef(group_format[changes as usize], first0, next_line0, first1, next_line1);
}
unsafe extern "C" fn format_ifdef(
    mut format: *const libc::c_char,
    mut beg0: lin,
    mut end0: lin,
    mut beg1: lin,
    mut end1: lin,
) {
    let mut groups: [group; 2] = [group {
        file: 0 as *const file_data,
        from: 0,
        upto: 0,
    }; 2];
    groups[0 as libc::c_int as usize]
        .file = &mut *files.as_mut_ptr().offset(0 as libc::c_int as isize)
        as *mut file_data;
    groups[0 as libc::c_int as usize].from = beg0;
    groups[0 as libc::c_int as usize].upto = end0;
    groups[1 as libc::c_int as usize]
        .file = &mut *files.as_mut_ptr().offset(1 as libc::c_int as isize)
        as *mut file_data;
    groups[1 as libc::c_int as usize].from = beg1;
    groups[1 as libc::c_int as usize].upto = end1;
    format_group(outfile, format, 0 as libc::c_int as libc::c_char, groups.as_mut_ptr());
}
unsafe extern "C" fn format_group(
    mut out: *mut FILE,
    mut format: *const libc::c_char,
    mut endchar: libc::c_char,
    mut groups: *const group,
) -> *const libc::c_char {
    let mut current_block: u64;
    let mut c: libc::c_char = 0;
    let mut f: *const libc::c_char = format;
    loop {
        c = *f;
        if !(c as libc::c_int != endchar as libc::c_int
            && c as libc::c_int != 0 as libc::c_int)
        {
            break;
        }
        f = f.offset(1);
        let mut f1: *const libc::c_char = f;
        if c as libc::c_int == '%' as i32 {
            let fresh1 = f;
            f = f.offset(1);
            c = *fresh1;
            match c as libc::c_int {
                37 => {}
                40 => {
                    current_block = 10680521327981672866;
                    match current_block {
                        11058672248517667920 => {
                            print_ifdef_lines(
                                out,
                                line_format[NEW as libc::c_int as usize],
                                &*groups.offset(1 as libc::c_int as isize),
                            );
                            continue;
                        }
                        8053580538223045129 => {
                            f = do_printf_spec(
                                out,
                                f.offset(-(2 as libc::c_int as isize)),
                                0 as *const file_data,
                                0 as libc::c_int as lin,
                                groups,
                            );
                            if !f.is_null() {
                                continue;
                            }
                        }
                        15738653310423343346 => {
                            print_ifdef_lines(
                                out,
                                line_format[OLD as libc::c_int as usize],
                                &*groups.offset(0 as libc::c_int as isize),
                            );
                            continue;
                        }
                        17620377729944291641 => {
                            print_ifdef_lines(
                                out,
                                line_format[UNCHANGED as libc::c_int as usize],
                                &*groups.offset(0 as libc::c_int as isize),
                            );
                            continue;
                        }
                        _ => {
                            let mut i: libc::c_int = 0;
                            let mut value: [intmax_t; 2] = [0; 2];
                            let mut thenout: *mut FILE = 0 as *mut FILE;
                            let mut elseout: *mut FILE = 0 as *mut FILE;
                            i = 0 as libc::c_int;
                            loop {
                                if !(i < 2 as libc::c_int) {
                                    current_block = 1054647088692577877;
                                    break;
                                }
                                if (*f as libc::c_uint)
                                    .wrapping_sub('0' as i32 as libc::c_uint)
                                    <= 9 as libc::c_int as libc::c_uint
                                {
                                    let mut fend: *mut libc::c_char = 0 as *mut libc::c_char;
                                    *__errno_location() = 0 as libc::c_int;
                                    value[i
                                        as usize] = strtoimax(f, &mut fend, 10 as libc::c_int);
                                    if *__errno_location() != 0 {
                                        current_block = 11735863289274782722;
                                        break;
                                    }
                                    f = fend;
                                } else {
                                    value[i as usize] = groups_letter_value(groups, *f);
                                    if value[i as usize] < 0 as libc::c_int as libc::c_long {
                                        current_block = 11735863289274782722;
                                        break;
                                    }
                                    f = f.offset(1);
                                    f;
                                }
                                let fresh2 = f;
                                f = f.offset(1);
                                if *fresh2 as libc::c_int
                                    != (*::std::mem::transmute::<
                                        &[u8; 3],
                                        &[libc::c_char; 3],
                                    >(b"=?\0"))[i as usize] as libc::c_int
                                {
                                    current_block = 11735863289274782722;
                                    break;
                                }
                                i += 1;
                                i;
                            }
                            match current_block {
                                11735863289274782722 => {}
                                _ => {
                                    if value[0 as libc::c_int as usize]
                                        == value[1 as libc::c_int as usize]
                                    {
                                        thenout = out;
                                        elseout = 0 as *mut FILE;
                                    } else {
                                        thenout = 0 as *mut FILE;
                                        elseout = out;
                                    }
                                    f = format_group(
                                        thenout,
                                        f,
                                        ':' as i32 as libc::c_char,
                                        groups,
                                    );
                                    if *f != 0 {
                                        f = format_group(
                                            elseout,
                                            f.offset(1 as libc::c_int as isize),
                                            ')' as i32 as libc::c_char,
                                            groups,
                                        );
                                        if *f != 0 {
                                            f = f.offset(1);
                                            f;
                                        }
                                    }
                                    continue;
                                }
                            }
                        }
                    }
                    c = '%' as i32 as libc::c_char;
                    f = f1;
                }
                60 => {
                    current_block = 15738653310423343346;
                    match current_block {
                        11058672248517667920 => {
                            print_ifdef_lines(
                                out,
                                line_format[NEW as libc::c_int as usize],
                                &*groups.offset(1 as libc::c_int as isize),
                            );
                            continue;
                        }
                        8053580538223045129 => {
                            f = do_printf_spec(
                                out,
                                f.offset(-(2 as libc::c_int as isize)),
                                0 as *const file_data,
                                0 as libc::c_int as lin,
                                groups,
                            );
                            if !f.is_null() {
                                continue;
                            }
                        }
                        15738653310423343346 => {
                            print_ifdef_lines(
                                out,
                                line_format[OLD as libc::c_int as usize],
                                &*groups.offset(0 as libc::c_int as isize),
                            );
                            continue;
                        }
                        17620377729944291641 => {
                            print_ifdef_lines(
                                out,
                                line_format[UNCHANGED as libc::c_int as usize],
                                &*groups.offset(0 as libc::c_int as isize),
                            );
                            continue;
                        }
                        _ => {
                            let mut i: libc::c_int = 0;
                            let mut value: [intmax_t; 2] = [0; 2];
                            let mut thenout: *mut FILE = 0 as *mut FILE;
                            let mut elseout: *mut FILE = 0 as *mut FILE;
                            i = 0 as libc::c_int;
                            loop {
                                if !(i < 2 as libc::c_int) {
                                    current_block = 1054647088692577877;
                                    break;
                                }
                                if (*f as libc::c_uint)
                                    .wrapping_sub('0' as i32 as libc::c_uint)
                                    <= 9 as libc::c_int as libc::c_uint
                                {
                                    let mut fend: *mut libc::c_char = 0 as *mut libc::c_char;
                                    *__errno_location() = 0 as libc::c_int;
                                    value[i
                                        as usize] = strtoimax(f, &mut fend, 10 as libc::c_int);
                                    if *__errno_location() != 0 {
                                        current_block = 11735863289274782722;
                                        break;
                                    }
                                    f = fend;
                                } else {
                                    value[i as usize] = groups_letter_value(groups, *f);
                                    if value[i as usize] < 0 as libc::c_int as libc::c_long {
                                        current_block = 11735863289274782722;
                                        break;
                                    }
                                    f = f.offset(1);
                                    f;
                                }
                                let fresh2 = f;
                                f = f.offset(1);
                                if *fresh2 as libc::c_int
                                    != (*::std::mem::transmute::<
                                        &[u8; 3],
                                        &[libc::c_char; 3],
                                    >(b"=?\0"))[i as usize] as libc::c_int
                                {
                                    current_block = 11735863289274782722;
                                    break;
                                }
                                i += 1;
                                i;
                            }
                            match current_block {
                                11735863289274782722 => {}
                                _ => {
                                    if value[0 as libc::c_int as usize]
                                        == value[1 as libc::c_int as usize]
                                    {
                                        thenout = out;
                                        elseout = 0 as *mut FILE;
                                    } else {
                                        thenout = 0 as *mut FILE;
                                        elseout = out;
                                    }
                                    f = format_group(
                                        thenout,
                                        f,
                                        ':' as i32 as libc::c_char,
                                        groups,
                                    );
                                    if *f != 0 {
                                        f = format_group(
                                            elseout,
                                            f.offset(1 as libc::c_int as isize),
                                            ')' as i32 as libc::c_char,
                                            groups,
                                        );
                                        if *f != 0 {
                                            f = f.offset(1);
                                            f;
                                        }
                                    }
                                    continue;
                                }
                            }
                        }
                    }
                    c = '%' as i32 as libc::c_char;
                    f = f1;
                }
                61 => {
                    current_block = 17620377729944291641;
                    match current_block {
                        11058672248517667920 => {
                            print_ifdef_lines(
                                out,
                                line_format[NEW as libc::c_int as usize],
                                &*groups.offset(1 as libc::c_int as isize),
                            );
                            continue;
                        }
                        8053580538223045129 => {
                            f = do_printf_spec(
                                out,
                                f.offset(-(2 as libc::c_int as isize)),
                                0 as *const file_data,
                                0 as libc::c_int as lin,
                                groups,
                            );
                            if !f.is_null() {
                                continue;
                            }
                        }
                        15738653310423343346 => {
                            print_ifdef_lines(
                                out,
                                line_format[OLD as libc::c_int as usize],
                                &*groups.offset(0 as libc::c_int as isize),
                            );
                            continue;
                        }
                        17620377729944291641 => {
                            print_ifdef_lines(
                                out,
                                line_format[UNCHANGED as libc::c_int as usize],
                                &*groups.offset(0 as libc::c_int as isize),
                            );
                            continue;
                        }
                        _ => {
                            let mut i: libc::c_int = 0;
                            let mut value: [intmax_t; 2] = [0; 2];
                            let mut thenout: *mut FILE = 0 as *mut FILE;
                            let mut elseout: *mut FILE = 0 as *mut FILE;
                            i = 0 as libc::c_int;
                            loop {
                                if !(i < 2 as libc::c_int) {
                                    current_block = 1054647088692577877;
                                    break;
                                }
                                if (*f as libc::c_uint)
                                    .wrapping_sub('0' as i32 as libc::c_uint)
                                    <= 9 as libc::c_int as libc::c_uint
                                {
                                    let mut fend: *mut libc::c_char = 0 as *mut libc::c_char;
                                    *__errno_location() = 0 as libc::c_int;
                                    value[i
                                        as usize] = strtoimax(f, &mut fend, 10 as libc::c_int);
                                    if *__errno_location() != 0 {
                                        current_block = 11735863289274782722;
                                        break;
                                    }
                                    f = fend;
                                } else {
                                    value[i as usize] = groups_letter_value(groups, *f);
                                    if value[i as usize] < 0 as libc::c_int as libc::c_long {
                                        current_block = 11735863289274782722;
                                        break;
                                    }
                                    f = f.offset(1);
                                    f;
                                }
                                let fresh2 = f;
                                f = f.offset(1);
                                if *fresh2 as libc::c_int
                                    != (*::std::mem::transmute::<
                                        &[u8; 3],
                                        &[libc::c_char; 3],
                                    >(b"=?\0"))[i as usize] as libc::c_int
                                {
                                    current_block = 11735863289274782722;
                                    break;
                                }
                                i += 1;
                                i;
                            }
                            match current_block {
                                11735863289274782722 => {}
                                _ => {
                                    if value[0 as libc::c_int as usize]
                                        == value[1 as libc::c_int as usize]
                                    {
                                        thenout = out;
                                        elseout = 0 as *mut FILE;
                                    } else {
                                        thenout = 0 as *mut FILE;
                                        elseout = out;
                                    }
                                    f = format_group(
                                        thenout,
                                        f,
                                        ':' as i32 as libc::c_char,
                                        groups,
                                    );
                                    if *f != 0 {
                                        f = format_group(
                                            elseout,
                                            f.offset(1 as libc::c_int as isize),
                                            ')' as i32 as libc::c_char,
                                            groups,
                                        );
                                        if *f != 0 {
                                            f = f.offset(1);
                                            f;
                                        }
                                    }
                                    continue;
                                }
                            }
                        }
                    }
                    c = '%' as i32 as libc::c_char;
                    f = f1;
                }
                62 => {
                    current_block = 11058672248517667920;
                    match current_block {
                        11058672248517667920 => {
                            print_ifdef_lines(
                                out,
                                line_format[NEW as libc::c_int as usize],
                                &*groups.offset(1 as libc::c_int as isize),
                            );
                            continue;
                        }
                        8053580538223045129 => {
                            f = do_printf_spec(
                                out,
                                f.offset(-(2 as libc::c_int as isize)),
                                0 as *const file_data,
                                0 as libc::c_int as lin,
                                groups,
                            );
                            if !f.is_null() {
                                continue;
                            }
                        }
                        15738653310423343346 => {
                            print_ifdef_lines(
                                out,
                                line_format[OLD as libc::c_int as usize],
                                &*groups.offset(0 as libc::c_int as isize),
                            );
                            continue;
                        }
                        17620377729944291641 => {
                            print_ifdef_lines(
                                out,
                                line_format[UNCHANGED as libc::c_int as usize],
                                &*groups.offset(0 as libc::c_int as isize),
                            );
                            continue;
                        }
                        _ => {
                            let mut i: libc::c_int = 0;
                            let mut value: [intmax_t; 2] = [0; 2];
                            let mut thenout: *mut FILE = 0 as *mut FILE;
                            let mut elseout: *mut FILE = 0 as *mut FILE;
                            i = 0 as libc::c_int;
                            loop {
                                if !(i < 2 as libc::c_int) {
                                    current_block = 1054647088692577877;
                                    break;
                                }
                                if (*f as libc::c_uint)
                                    .wrapping_sub('0' as i32 as libc::c_uint)
                                    <= 9 as libc::c_int as libc::c_uint
                                {
                                    let mut fend: *mut libc::c_char = 0 as *mut libc::c_char;
                                    *__errno_location() = 0 as libc::c_int;
                                    value[i
                                        as usize] = strtoimax(f, &mut fend, 10 as libc::c_int);
                                    if *__errno_location() != 0 {
                                        current_block = 11735863289274782722;
                                        break;
                                    }
                                    f = fend;
                                } else {
                                    value[i as usize] = groups_letter_value(groups, *f);
                                    if value[i as usize] < 0 as libc::c_int as libc::c_long {
                                        current_block = 11735863289274782722;
                                        break;
                                    }
                                    f = f.offset(1);
                                    f;
                                }
                                let fresh2 = f;
                                f = f.offset(1);
                                if *fresh2 as libc::c_int
                                    != (*::std::mem::transmute::<
                                        &[u8; 3],
                                        &[libc::c_char; 3],
                                    >(b"=?\0"))[i as usize] as libc::c_int
                                {
                                    current_block = 11735863289274782722;
                                    break;
                                }
                                i += 1;
                                i;
                            }
                            match current_block {
                                11735863289274782722 => {}
                                _ => {
                                    if value[0 as libc::c_int as usize]
                                        == value[1 as libc::c_int as usize]
                                    {
                                        thenout = out;
                                        elseout = 0 as *mut FILE;
                                    } else {
                                        thenout = 0 as *mut FILE;
                                        elseout = out;
                                    }
                                    f = format_group(
                                        thenout,
                                        f,
                                        ':' as i32 as libc::c_char,
                                        groups,
                                    );
                                    if *f != 0 {
                                        f = format_group(
                                            elseout,
                                            f.offset(1 as libc::c_int as isize),
                                            ')' as i32 as libc::c_char,
                                            groups,
                                        );
                                        if *f != 0 {
                                            f = f.offset(1);
                                            f;
                                        }
                                    }
                                    continue;
                                }
                            }
                        }
                    }
                    c = '%' as i32 as libc::c_char;
                    f = f1;
                }
                _ => {
                    current_block = 8053580538223045129;
                    match current_block {
                        11058672248517667920 => {
                            print_ifdef_lines(
                                out,
                                line_format[NEW as libc::c_int as usize],
                                &*groups.offset(1 as libc::c_int as isize),
                            );
                            continue;
                        }
                        8053580538223045129 => {
                            f = do_printf_spec(
                                out,
                                f.offset(-(2 as libc::c_int as isize)),
                                0 as *const file_data,
                                0 as libc::c_int as lin,
                                groups,
                            );
                            if !f.is_null() {
                                continue;
                            }
                        }
                        15738653310423343346 => {
                            print_ifdef_lines(
                                out,
                                line_format[OLD as libc::c_int as usize],
                                &*groups.offset(0 as libc::c_int as isize),
                            );
                            continue;
                        }
                        17620377729944291641 => {
                            print_ifdef_lines(
                                out,
                                line_format[UNCHANGED as libc::c_int as usize],
                                &*groups.offset(0 as libc::c_int as isize),
                            );
                            continue;
                        }
                        _ => {
                            let mut i: libc::c_int = 0;
                            let mut value: [intmax_t; 2] = [0; 2];
                            let mut thenout: *mut FILE = 0 as *mut FILE;
                            let mut elseout: *mut FILE = 0 as *mut FILE;
                            i = 0 as libc::c_int;
                            loop {
                                if !(i < 2 as libc::c_int) {
                                    current_block = 1054647088692577877;
                                    break;
                                }
                                if (*f as libc::c_uint)
                                    .wrapping_sub('0' as i32 as libc::c_uint)
                                    <= 9 as libc::c_int as libc::c_uint
                                {
                                    let mut fend: *mut libc::c_char = 0 as *mut libc::c_char;
                                    *__errno_location() = 0 as libc::c_int;
                                    value[i
                                        as usize] = strtoimax(f, &mut fend, 10 as libc::c_int);
                                    if *__errno_location() != 0 {
                                        current_block = 11735863289274782722;
                                        break;
                                    }
                                    f = fend;
                                } else {
                                    value[i as usize] = groups_letter_value(groups, *f);
                                    if value[i as usize] < 0 as libc::c_int as libc::c_long {
                                        current_block = 11735863289274782722;
                                        break;
                                    }
                                    f = f.offset(1);
                                    f;
                                }
                                let fresh2 = f;
                                f = f.offset(1);
                                if *fresh2 as libc::c_int
                                    != (*::std::mem::transmute::<
                                        &[u8; 3],
                                        &[libc::c_char; 3],
                                    >(b"=?\0"))[i as usize] as libc::c_int
                                {
                                    current_block = 11735863289274782722;
                                    break;
                                }
                                i += 1;
                                i;
                            }
                            match current_block {
                                11735863289274782722 => {}
                                _ => {
                                    if value[0 as libc::c_int as usize]
                                        == value[1 as libc::c_int as usize]
                                    {
                                        thenout = out;
                                        elseout = 0 as *mut FILE;
                                    } else {
                                        thenout = 0 as *mut FILE;
                                        elseout = out;
                                    }
                                    f = format_group(
                                        thenout,
                                        f,
                                        ':' as i32 as libc::c_char,
                                        groups,
                                    );
                                    if *f != 0 {
                                        f = format_group(
                                            elseout,
                                            f.offset(1 as libc::c_int as isize),
                                            ')' as i32 as libc::c_char,
                                            groups,
                                        );
                                        if *f != 0 {
                                            f = f.offset(1);
                                            f;
                                        }
                                    }
                                    continue;
                                }
                            }
                        }
                    }
                    c = '%' as i32 as libc::c_char;
                    f = f1;
                }
            }
        }
        if !out.is_null() {
            putc_unlocked(c as libc::c_int, out);
        }
    }
    return f;
}
unsafe extern "C" fn groups_letter_value(
    mut g: *const group,
    mut letter: libc::c_char,
) -> lin {
    match letter as libc::c_int {
        69 => {
            letter = 'e' as i32 as libc::c_char;
            g = g.offset(1);
            g;
        }
        70 => {
            letter = 'f' as i32 as libc::c_char;
            g = g.offset(1);
            g;
        }
        76 => {
            letter = 'l' as i32 as libc::c_char;
            g = g.offset(1);
            g;
        }
        77 => {
            letter = 'm' as i32 as libc::c_char;
            g = g.offset(1);
            g;
        }
        78 => {
            letter = 'n' as i32 as libc::c_char;
            g = g.offset(1);
            g;
        }
        _ => {}
    }
    match letter as libc::c_int {
        101 => {
            return translate_line_number((*g).file, (*g).from)
                - 1 as libc::c_int as libc::c_long;
        }
        102 => return translate_line_number((*g).file, (*g).from),
        108 => {
            return translate_line_number((*g).file, (*g).upto)
                - 1 as libc::c_int as libc::c_long;
        }
        109 => return translate_line_number((*g).file, (*g).upto),
        110 => return (*g).upto - (*g).from,
        _ => return -(1 as libc::c_int) as lin,
    };
}
unsafe extern "C" fn print_ifdef_lines(
    mut out: *mut FILE,
    mut format: *const libc::c_char,
    mut group: *const group,
) {
    let mut file: *const file_data = (*group).file;
    let mut linbuf: *const *const libc::c_char = (*file).linbuf;
    let mut from: lin = (*group).from;
    let mut upto: lin = (*group).upto;
    if out.is_null() {
        return;
    }
    if !expand_tabs
        && *format.offset(0 as libc::c_int as isize) as libc::c_int == '%' as i32
    {
        if *format.offset(1 as libc::c_int as isize) as libc::c_int == 'l' as i32
            && *format.offset(2 as libc::c_int as isize) as libc::c_int == '\n' as i32
            && *format.offset(3 as libc::c_int as isize) == 0 && from < upto
        {
            if 0 != 0 && 0 != 0
                && (::std::mem::size_of::<libc::c_char>() as libc::c_ulong)
                    .wrapping_mul(
                        (*linbuf.offset(upto as isize))
                            .offset(
                                (*(*linbuf.offset(upto as isize))
                                    .offset(-(1 as libc::c_int) as isize) as libc::c_int
                                    != '\n' as i32) as libc::c_int as isize,
                            )
                            .offset_from(*linbuf.offset(from as isize)) as libc::c_long
                            as size_t,
                    ) <= 8 as libc::c_int as libc::c_ulong
                && ::std::mem::size_of::<libc::c_char>() as libc::c_ulong
                    != 0 as libc::c_int as libc::c_ulong
            {
                ({
                    let mut __ptr: *const libc::c_char = *linbuf.offset(from as isize);
                    let mut __stream: *mut FILE = out;
                    let mut __cnt: size_t = 0;
                    __cnt = (::std::mem::size_of::<libc::c_char>() as libc::c_ulong)
                        .wrapping_mul(
                            (*linbuf.offset(upto as isize))
                                .offset(
                                    (*(*linbuf.offset(upto as isize))
                                        .offset(-(1 as libc::c_int) as isize) as libc::c_int
                                        != '\n' as i32) as libc::c_int as isize,
                                )
                                .offset_from(*linbuf.offset(from as isize)) as libc::c_long
                                as size_t,
                        );
                    while __cnt > 0 as libc::c_int as libc::c_ulong {
                        let fresh3 = __ptr;
                        __ptr = __ptr.offset(1);
                        if putc_unlocked(*fresh3 as libc::c_int, __stream)
                            == -(1 as libc::c_int)
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
                    || 0 != 0
                        && (*linbuf.offset(upto as isize))
                            .offset(
                                (*(*linbuf.offset(upto as isize))
                                    .offset(-(1 as libc::c_int) as isize) as libc::c_int
                                    != '\n' as i32) as libc::c_int as isize,
                            )
                            .offset_from(*linbuf.offset(from as isize)) as libc::c_long
                            as size_t == 0 as libc::c_int as libc::c_ulong
                {} else {
                    fwrite_unlocked(
                        *linbuf.offset(from as isize) as *const libc::c_void,
                        ::std::mem::size_of::<libc::c_char>() as libc::c_ulong,
                        (*linbuf.offset(upto as isize))
                            .offset(
                                (*(*linbuf.offset(upto as isize))
                                    .offset(-(1 as libc::c_int) as isize) as libc::c_int
                                    != '\n' as i32) as libc::c_int as isize,
                            )
                            .offset_from(*linbuf.offset(from as isize)) as libc::c_long
                            as size_t,
                        out,
                    );
                };
            };
            0u8;
            return;
        }
        if *format.offset(1 as libc::c_int as isize) as libc::c_int == 'L' as i32
            && *format.offset(2 as libc::c_int as isize) == 0
        {
            if 0 != 0 && 0 != 0
                && (::std::mem::size_of::<libc::c_char>() as libc::c_ulong)
                    .wrapping_mul(
                        (*linbuf.offset(upto as isize))
                            .offset_from(*linbuf.offset(from as isize)) as libc::c_long
                            as size_t,
                    ) <= 8 as libc::c_int as libc::c_ulong
                && ::std::mem::size_of::<libc::c_char>() as libc::c_ulong
                    != 0 as libc::c_int as libc::c_ulong
            {
                ({
                    let mut __ptr: *const libc::c_char = *linbuf.offset(from as isize);
                    let mut __stream: *mut FILE = out;
                    let mut __cnt: size_t = 0;
                    __cnt = (::std::mem::size_of::<libc::c_char>() as libc::c_ulong)
                        .wrapping_mul(
                            (*linbuf.offset(upto as isize))
                                .offset_from(*linbuf.offset(from as isize)) as libc::c_long
                                as size_t,
                        );
                    while __cnt > 0 as libc::c_int as libc::c_ulong {
                        let fresh4 = __ptr;
                        __ptr = __ptr.offset(1);
                        if putc_unlocked(*fresh4 as libc::c_int, __stream)
                            == -(1 as libc::c_int)
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
                    || 0 != 0
                        && (*linbuf.offset(upto as isize))
                            .offset_from(*linbuf.offset(from as isize)) as libc::c_long
                            as size_t == 0 as libc::c_int as libc::c_ulong
                {} else {
                    fwrite_unlocked(
                        *linbuf.offset(from as isize) as *const libc::c_void,
                        ::std::mem::size_of::<libc::c_char>() as libc::c_ulong,
                        (*linbuf.offset(upto as isize))
                            .offset_from(*linbuf.offset(from as isize)) as libc::c_long
                            as size_t,
                        out,
                    );
                };
            };
            0u8;
            return;
        }
    }
    while from < upto {
        let mut c: libc::c_char = 0;
        let mut f: *const libc::c_char = format;
        let mut current_block_17: u64;
        loop {
            let fresh5 = f;
            f = f.offset(1);
            c = *fresh5;
            if !(c as libc::c_int != 0 as libc::c_int) {
                break;
            }
            let mut f1: *const libc::c_char = f;
            if c as libc::c_int == '%' as i32 {
                let fresh6 = f;
                f = f.offset(1);
                c = *fresh6;
                match c as libc::c_int {
                    37 => {}
                    108 => {
                        current_block_17 = 11898206195258247614;
                        match current_block_17 {
                            2151901123398569495 => {
                                output_1_line(
                                    *linbuf.offset(from as isize),
                                    *linbuf
                                        .offset((from + 1 as libc::c_int as libc::c_long) as isize),
                                    0 as *const libc::c_char,
                                    0 as *const libc::c_char,
                                );
                                continue;
                            }
                            11898206195258247614 => {
                                output_1_line(
                                    *linbuf.offset(from as isize),
                                    (*linbuf
                                        .offset((from + 1 as libc::c_int as libc::c_long) as isize))
                                        .offset(
                                            -((*(*linbuf
                                                .offset((from + 1 as libc::c_int as libc::c_long) as isize))
                                                .offset(-(1 as libc::c_int) as isize) as libc::c_int
                                                == '\n' as i32) as libc::c_int as isize),
                                        ),
                                    0 as *const libc::c_char,
                                    0 as *const libc::c_char,
                                );
                                continue;
                            }
                            _ => {
                                f = do_printf_spec(
                                    out,
                                    f.offset(-(2 as libc::c_int as isize)),
                                    file,
                                    from,
                                    0 as *const group,
                                );
                                if !f.is_null() {
                                    continue;
                                }
                                c = '%' as i32 as libc::c_char;
                                f = f1;
                            }
                        }
                    }
                    76 => {
                        current_block_17 = 2151901123398569495;
                        match current_block_17 {
                            2151901123398569495 => {
                                output_1_line(
                                    *linbuf.offset(from as isize),
                                    *linbuf
                                        .offset((from + 1 as libc::c_int as libc::c_long) as isize),
                                    0 as *const libc::c_char,
                                    0 as *const libc::c_char,
                                );
                                continue;
                            }
                            11898206195258247614 => {
                                output_1_line(
                                    *linbuf.offset(from as isize),
                                    (*linbuf
                                        .offset((from + 1 as libc::c_int as libc::c_long) as isize))
                                        .offset(
                                            -((*(*linbuf
                                                .offset((from + 1 as libc::c_int as libc::c_long) as isize))
                                                .offset(-(1 as libc::c_int) as isize) as libc::c_int
                                                == '\n' as i32) as libc::c_int as isize),
                                        ),
                                    0 as *const libc::c_char,
                                    0 as *const libc::c_char,
                                );
                                continue;
                            }
                            _ => {
                                f = do_printf_spec(
                                    out,
                                    f.offset(-(2 as libc::c_int as isize)),
                                    file,
                                    from,
                                    0 as *const group,
                                );
                                if !f.is_null() {
                                    continue;
                                }
                                c = '%' as i32 as libc::c_char;
                                f = f1;
                            }
                        }
                    }
                    _ => {
                        current_block_17 = 6504071537074756326;
                        match current_block_17 {
                            2151901123398569495 => {
                                output_1_line(
                                    *linbuf.offset(from as isize),
                                    *linbuf
                                        .offset((from + 1 as libc::c_int as libc::c_long) as isize),
                                    0 as *const libc::c_char,
                                    0 as *const libc::c_char,
                                );
                                continue;
                            }
                            11898206195258247614 => {
                                output_1_line(
                                    *linbuf.offset(from as isize),
                                    (*linbuf
                                        .offset((from + 1 as libc::c_int as libc::c_long) as isize))
                                        .offset(
                                            -((*(*linbuf
                                                .offset((from + 1 as libc::c_int as libc::c_long) as isize))
                                                .offset(-(1 as libc::c_int) as isize) as libc::c_int
                                                == '\n' as i32) as libc::c_int as isize),
                                        ),
                                    0 as *const libc::c_char,
                                    0 as *const libc::c_char,
                                );
                                continue;
                            }
                            _ => {
                                f = do_printf_spec(
                                    out,
                                    f.offset(-(2 as libc::c_int as isize)),
                                    file,
                                    from,
                                    0 as *const group,
                                );
                                if !f.is_null() {
                                    continue;
                                }
                                c = '%' as i32 as libc::c_char;
                                f = f1;
                            }
                        }
                    }
                }
            }
            putc_unlocked(c as libc::c_int, out);
        }
        from += 1;
        from;
    }
}
unsafe extern "C" fn do_printf_spec(
    mut out: *mut FILE,
    mut spec: *const libc::c_char,
    mut file: *const file_data,
    mut n: lin,
    mut groups: *const group,
) -> *const libc::c_char {
    let mut f: *const libc::c_char = spec;
    let mut c: libc::c_char = 0;
    let mut c1: libc::c_char = 0;
    f = f.offset(1);
    f;
    loop {
        let fresh7 = f;
        f = f.offset(1);
        c = *fresh7;
        if !(c as libc::c_int == '-' as i32 || c as libc::c_int == '\'' as i32
            || c as libc::c_int == '0' as i32)
        {
            break;
        }
    }
    while (c as libc::c_uint).wrapping_sub('0' as i32 as libc::c_uint)
        <= 9 as libc::c_int as libc::c_uint
    {
        let fresh8 = f;
        f = f.offset(1);
        c = *fresh8;
    }
    if c as libc::c_int == '.' as i32 {
        loop {
            let fresh9 = f;
            f = f.offset(1);
            c = *fresh9;
            if !((c as libc::c_uint).wrapping_sub('0' as i32 as libc::c_uint)
                <= 9 as libc::c_int as libc::c_uint)
            {
                break;
            }
        }
    }
    let fresh10 = f;
    f = f.offset(1);
    c1 = *fresh10;
    match c as libc::c_int {
        99 => {
            if c1 as libc::c_int != '\'' as i32 {
                return 0 as *const libc::c_char
            } else {
                let mut value: libc::c_char = 0;
                f = scan_char_literal(f, &mut value);
                if f.is_null() {
                    return 0 as *const libc::c_char;
                }
                if !out.is_null() {
                    putc_unlocked(value as libc::c_int, out);
                }
            }
        }
        100 | 111 | 120 | 88 => {
            let mut value_0: lin = 0;
            if !file.is_null() {
                if c1 as libc::c_int != 'n' as i32 {
                    return 0 as *const libc::c_char;
                }
                value_0 = translate_line_number(file, n);
            } else {
                value_0 = groups_letter_value(groups, c1);
                if value_0 < 0 as libc::c_int as libc::c_long {
                    return 0 as *const libc::c_char;
                }
            }
            if !out.is_null() {
                let mut spec_prefix_len: size_t = (f.offset_from(spec) as libc::c_long
                    - 2 as libc::c_int as libc::c_long) as size_t;
                let mut pI_len: size_t = (::std::mem::size_of::<[libc::c_char; 2]>()
                    as libc::c_ulong)
                    .wrapping_sub(1 as libc::c_int as libc::c_ulong);
                let mut format: *mut libc::c_char = (if spec_prefix_len
                    .wrapping_add(pI_len)
                    .wrapping_add(2 as libc::c_int as libc::c_ulong)
                    < (4032 as libc::c_int
                        - (2 as libc::c_int * sa_alignment_max as libc::c_int
                            - 1 as libc::c_int)) as libc::c_ulong
                {
                    let mut fresh11 = ::std::vec::from_elem(
                        0,
                        spec_prefix_len
                            .wrapping_add(pI_len)
                            .wrapping_add(2 as libc::c_int as libc::c_ulong)
                            .wrapping_add(
                                (2 as libc::c_int * sa_alignment_max as libc::c_int)
                                    as libc::c_ulong,
                            )
                            .wrapping_sub(1 as libc::c_int as libc::c_ulong) as usize,
                    );
                    ((fresh11.leak().as_mut_ptr() as *mut libc::c_char as uintptr_t)
                        .wrapping_add(
                            (2 as libc::c_int * sa_alignment_max as libc::c_int
                                - 1 as libc::c_int) as libc::c_ulong,
                        )
                        & !((2 as libc::c_int * sa_alignment_max as libc::c_int
                            - 1 as libc::c_int) as uintptr_t)) as *mut libc::c_void
                } else {
                    xmmalloca(
                        spec_prefix_len
                            .wrapping_add(pI_len)
                            .wrapping_add(2 as libc::c_int as libc::c_ulong),
                    )
                }) as *mut libc::c_char;
                let mut p: *mut libc::c_char = mempcpy(
                    format as *mut libc::c_void,
                    spec as *const libc::c_void,
                    spec_prefix_len,
                ) as *mut libc::c_char;
                p = stpcpy(p, b"t\0" as *const u8 as *const libc::c_char);
                let fresh12 = p;
                p = p.offset(1);
                *fresh12 = c;
                *p = '\0' as i32 as libc::c_char;
                fprintf(out, format, value_0);
                freea(format as *mut libc::c_void);
            }
        }
        _ => return 0 as *const libc::c_char,
    }
    return f;
}
unsafe extern "C" fn scan_char_literal(
    mut lit: *const libc::c_char,
    mut valptr: *mut libc::c_char,
) -> *const libc::c_char {
    let mut p: *const libc::c_char = lit;
    let mut value: libc::c_char = 0;
    let mut digits: ptrdiff_t = 0;
    let fresh13 = p;
    p = p.offset(1);
    let mut c: libc::c_char = *fresh13;
    match c as libc::c_int {
        0 | 39 => return 0 as *const libc::c_char,
        92 => {
            value = 0 as libc::c_int as libc::c_char;
            loop {
                let fresh14 = p;
                p = p.offset(1);
                c = *fresh14;
                if !(c as libc::c_int != '\'' as i32) {
                    break;
                }
                let mut digit: libc::c_uint = (c as libc::c_int - '0' as i32)
                    as libc::c_uint;
                if 8 as libc::c_int as libc::c_uint <= digit {
                    return 0 as *const libc::c_char;
                }
                value = ((8 as libc::c_int * value as libc::c_int) as libc::c_uint)
                    .wrapping_add(digit) as libc::c_char;
            }
            digits = p.offset_from(lit) as libc::c_long
                - 2 as libc::c_int as libc::c_long;
            if !(1 as libc::c_int as libc::c_long <= digits
                && digits <= 3 as libc::c_int as libc::c_long)
            {
                return 0 as *const libc::c_char;
            }
        }
        _ => {
            value = c;
            let fresh15 = p;
            p = p.offset(1);
            if *fresh15 as libc::c_int != '\'' as i32 {
                return 0 as *const libc::c_char;
            }
        }
    }
    *valptr = value;
    return p;
}
