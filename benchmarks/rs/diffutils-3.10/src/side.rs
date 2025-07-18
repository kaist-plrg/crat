use ::libc;
extern "C" {
    pub type _IO_wide_data;
    pub type _IO_codecvt;
    pub type _IO_marker;
    fn begin_output();
    fn print_script(
        _: *mut change,
        _: Option::<unsafe extern "C" fn(*mut change) -> *mut change>,
        _: Option::<unsafe extern "C" fn(*mut change) -> ()>,
    );
    static mut stdout: *mut FILE;
    fn fprintf(_: *mut FILE, _: *const libc::c_char, _: ...) -> libc::c_int;
    fn __overflow(_: *mut FILE, _: libc::c_int) -> libc::c_int;
    fn fwrite_unlocked(
        __ptr: *const libc::c_void,
        __size: size_t,
        __n: size_t,
        __stream: *mut FILE,
    ) -> size_t;
    static mut expand_tabs: bool;
    static mut tabsize: size_t;
    static mut sdiff_merge_assist: bool;
    static mut left_column: bool;
    static mut suppress_common_lines: bool;
    static mut sdiff_half_width: size_t;
    static mut sdiff_column2_offset: size_t;
    static mut files: [file_data; 2];
    static mut outfile: *mut FILE;
    fn find_change(_: *mut change) -> *mut change;
    fn analyze_hunk(
        _: *mut change,
        _: *mut lin,
        _: *mut lin,
        _: *mut lin,
        _: *mut lin,
    ) -> changes;
    fn set_color_context(color_context: color_context);
    fn wcwidth(__c: wchar_t) -> libc::c_int;
    fn rpl_mbrtowc(
        pwc: *mut wchar_t,
        s: *const libc::c_char,
        n: size_t,
        ps: *mut mbstate_t,
    ) -> size_t;
}
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
pub type wchar_t = libc::c_int;
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
pub type lin = ptrdiff_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct mbstate_t {
    pub __count: libc::c_int,
    pub __value: C2RustUnnamed,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed {
    pub __wch: libc::c_uint,
    pub __wchb: [libc::c_char; 4],
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
pub type __mbstate_t = mbstate_t;
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
static mut next0: lin = 0;
static mut next1: lin = 0;
pub unsafe extern "C" fn print_sdiff_script(mut script: *mut change) {
    begin_output();
    next1 = -files[0 as libc::c_int as usize].prefix_lines;
    next0 = next1;
    print_script(
        script,
        Some(find_change as unsafe extern "C" fn(*mut change) -> *mut change),
        Some(print_sdiff_hunk as unsafe extern "C" fn(*mut change) -> ()),
    );
    print_sdiff_common_lines(
        files[0 as libc::c_int as usize].valid_lines,
        files[1 as libc::c_int as usize].valid_lines,
    );
}
unsafe extern "C" fn tab_from_to(mut from: size_t, mut to: size_t) -> size_t {
    let mut out: *mut FILE = outfile;
    let mut tab: size_t = 0;
    let mut tab_size: size_t = tabsize;
    if !expand_tabs {
        tab = from.wrapping_add(tab_size).wrapping_sub(from.wrapping_rem(tab_size));
        while tab <= to {
            putc_unlocked('\t' as i32, out);
            from = tab;
            tab = (tab as libc::c_ulong).wrapping_add(tab_size) as size_t as size_t;
        }
    }
    loop {
        let fresh1 = from;
        from = from.wrapping_add(1);
        if !(fresh1 < to) {
            break;
        }
        putc_unlocked(' ' as i32, out);
    }
    return to;
}
unsafe extern "C" fn print_half_line(
    mut line: *const *const libc::c_char,
    mut indent: size_t,
    mut out_bound: size_t,
) -> size_t {
    let mut out: *mut FILE = outfile;
    let mut in_position: size_t = 0 as libc::c_int as size_t;
    let mut out_position: size_t = 0 as libc::c_int as size_t;
    let mut text_pointer: *const libc::c_char = *line.offset(0 as libc::c_int as isize);
    let mut text_limit: *const libc::c_char = *line.offset(1 as libc::c_int as isize);
    let mut mbstate: mbstate_t = {
        let mut init = __mbstate_t {
            __count: 0 as libc::c_int,
            __value: C2RustUnnamed { __wch: 0 },
        };
        init
    };
    while text_pointer < text_limit {
        let mut tp0: *const libc::c_char = text_pointer;
        let fresh2 = text_pointer;
        text_pointer = text_pointer.offset(1);
        let mut c: libc::c_char = *fresh2;
        let mut current_block_40: u64;
        match c as libc::c_int {
            9 => {
                let mut spaces: size_t = tabsize
                    .wrapping_sub(in_position.wrapping_rem(tabsize));
                if in_position == out_position {
                    let mut tabstop: size_t = out_position.wrapping_add(spaces);
                    if expand_tabs {
                        if out_bound < tabstop {
                            tabstop = out_bound;
                        }
                        while out_position < tabstop {
                            putc_unlocked(' ' as i32, out);
                            out_position = out_position.wrapping_add(1);
                            out_position;
                        }
                    } else if tabstop < out_bound {
                        out_position = tabstop;
                        putc_unlocked(c as libc::c_int, out);
                    }
                }
                in_position = (in_position as libc::c_ulong).wrapping_add(spaces)
                    as size_t as size_t;
                current_block_40 = 18383263831861166299;
            }
            13 => {
                putc_unlocked(c as libc::c_int, out);
                tab_from_to(0 as libc::c_int as size_t, indent);
                out_position = 0 as libc::c_int as size_t;
                in_position = out_position;
                current_block_40 = 18383263831861166299;
            }
            8 => {
                if in_position != 0 as libc::c_int as libc::c_ulong
                    && {
                        in_position = in_position.wrapping_sub(1);
                        in_position < out_bound
                    }
                {
                    if out_position <= in_position {
                        while out_position < in_position {
                            putc_unlocked(' ' as i32, out);
                            out_position = out_position.wrapping_add(1);
                            out_position;
                        }
                    } else {
                        out_position = in_position;
                        putc_unlocked(c as libc::c_int, out);
                    }
                }
                current_block_40 = 18383263831861166299;
            }
            12 | 11 => {
                current_block_40 = 18415473726231320051;
            }
            32 | 33 | 34 | 35 | 37 | 38 | 39 | 40 | 41 | 42 | 43 | 44 | 45 | 46 | 47 | 48
            | 49 | 50 | 51 | 52 | 53 | 54 | 55 | 56 | 57 | 58 | 59 | 60 | 61 | 62 | 63
            | 65 | 66 | 67 | 68 | 69 | 70 | 71 | 72 | 73 | 74 | 75 | 76 | 77 | 78 | 79
            | 80 | 81 | 82 | 83 | 84 | 85 | 86 | 87 | 88 | 89 | 90 | 91 | 92 | 93 | 94
            | 95 | 97 | 98 | 99 | 100 | 101 | 102 | 103 | 104 | 105 | 106 | 107 | 108
            | 109 | 110 | 111 | 112 | 113 | 114 | 115 | 116 | 117 | 118 | 119 | 120 | 121
            | 122 | 123 | 124 | 125 | 126 => {
                let fresh4 = in_position;
                in_position = in_position.wrapping_add(1);
                if fresh4 < out_bound {
                    out_position = in_position;
                    putc_unlocked(c as libc::c_int, out);
                }
                current_block_40 = 18383263831861166299;
            }
            10 => return out_position,
            _ => {
                let mut wc: wchar_t = 0;
                let mut bytes: size_t = rpl_mbrtowc(
                    &mut wc,
                    tp0,
                    text_limit.offset_from(tp0) as libc::c_long as size_t,
                    &mut mbstate,
                );
                if (0 as libc::c_int as libc::c_ulong) < bytes
                    && bytes < -(2 as libc::c_int) as size_t
                {
                    let mut width: libc::c_int = wcwidth(wc);
                    if (0 as libc::c_int) < width {
                        in_position = (in_position as libc::c_ulong)
                            .wrapping_add(width as libc::c_ulong) as size_t as size_t;
                    }
                    if in_position <= out_bound {
                        out_position = in_position;
                        if 0 != 0 && 0 != 0
                            && (1 as libc::c_int as size_t).wrapping_mul(bytes)
                                <= 8 as libc::c_int as libc::c_ulong
                            && 1 as libc::c_int as size_t
                                != 0 as libc::c_int as libc::c_ulong
                        {
                            ({
                                let mut __ptr: *const libc::c_char = tp0;
                                let mut __stream: *mut FILE = stdout;
                                let mut __cnt: size_t = 0;
                                __cnt = (1 as libc::c_int as size_t).wrapping_mul(bytes);
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
                                && 1 as libc::c_int as size_t
                                    == 0 as libc::c_int as libc::c_ulong
                                || 0 != 0 && bytes == 0 as libc::c_int as libc::c_ulong
                            {} else {
                                fwrite_unlocked(
                                    tp0 as *const libc::c_void,
                                    1 as libc::c_int as size_t,
                                    bytes,
                                    stdout,
                                );
                            };
                        };
                        0u8;
                    }
                    text_pointer = tp0.offset(bytes as isize);
                    current_block_40 = 18383263831861166299;
                } else {
                    current_block_40 = 18415473726231320051;
                }
            }
        }
        match current_block_40 {
            18415473726231320051 => {
                if in_position < out_bound {
                    putc_unlocked(c as libc::c_int, out);
                }
            }
            _ => {}
        }
    }
    return out_position;
}
unsafe extern "C" fn print_1sdiff_line(
    mut left: *const *const libc::c_char,
    mut sep: libc::c_char,
    mut right: *const *const libc::c_char,
) {
    let mut out: *mut FILE = outfile;
    let mut hw: size_t = sdiff_half_width;
    let mut c2o: size_t = sdiff_column2_offset;
    let mut col: size_t = 0 as libc::c_int as size_t;
    let mut put_newline: bool = 0 as libc::c_int != 0;
    let mut color_to_reset: bool = 0 as libc::c_int != 0;
    if sep as libc::c_int == '<' as i32 {
        set_color_context(DELETE_CONTEXT);
        color_to_reset = 1 as libc::c_int != 0;
    } else if sep as libc::c_int == '>' as i32 {
        set_color_context(ADD_CONTEXT);
        color_to_reset = 1 as libc::c_int != 0;
    }
    if !left.is_null() {
        put_newline = (put_newline as libc::c_int
            | (*(*left.offset(1 as libc::c_int as isize))
                .offset(-(1 as libc::c_int) as isize) as libc::c_int == '\n' as i32)
                as libc::c_int) != 0;
        col = print_half_line(left, 0 as libc::c_int as size_t, hw);
    }
    if sep as libc::c_int != ' ' as i32 {
        col = (tab_from_to(
            col,
            hw
                .wrapping_add(c2o)
                .wrapping_sub(1 as libc::c_int as libc::c_ulong)
                .wrapping_div(2 as libc::c_int as libc::c_ulong),
        ))
            .wrapping_add(1 as libc::c_int as libc::c_ulong);
        if sep as libc::c_int == '|' as i32
            && put_newline as libc::c_int
                != (*(*right.offset(1 as libc::c_int as isize))
                    .offset(-(1 as libc::c_int) as isize) as libc::c_int == '\n' as i32)
                    as libc::c_int
        {
            sep = (if put_newline as libc::c_int != 0 {
                '/' as i32
            } else {
                '\\' as i32
            }) as libc::c_char;
        }
        putc_unlocked(sep as libc::c_int, out);
    }
    if !right.is_null() {
        put_newline = (put_newline as libc::c_int
            | (*(*right.offset(1 as libc::c_int as isize))
                .offset(-(1 as libc::c_int) as isize) as libc::c_int == '\n' as i32)
                as libc::c_int) != 0;
        if **right as libc::c_int != '\n' as i32 {
            col = tab_from_to(col, c2o);
            print_half_line(right, col, hw);
        }
    }
    if put_newline {
        putc_unlocked('\n' as i32, out);
    }
    if color_to_reset {
        set_color_context(RESET_CONTEXT);
    }
}
unsafe extern "C" fn print_sdiff_common_lines(mut limit0: lin, mut limit1: lin) {
    let mut i0: lin = next0;
    let mut i1: lin = next1;
    if !suppress_common_lines && (i0 != limit0 || i1 != limit1) {
        if sdiff_merge_assist {
            fprintf(
                outfile,
                b"i%td,%td\n\0" as *const u8 as *const libc::c_char,
                limit0 - i0,
                limit1 - i1,
            );
        }
        if !left_column {
            while i0 != limit0 && i1 != limit1 {
                let fresh5 = i0;
                i0 = i0 + 1;
                let fresh6 = i1;
                i1 = i1 + 1;
                print_1sdiff_line(
                    &mut *((*files.as_mut_ptr().offset(0 as libc::c_int as isize))
                        .linbuf)
                        .offset(fresh5 as isize),
                    ' ' as i32 as libc::c_char,
                    &mut *((*files.as_mut_ptr().offset(1 as libc::c_int as isize))
                        .linbuf)
                        .offset(fresh6 as isize),
                );
            }
            while i1 != limit1 {
                let fresh7 = i1;
                i1 = i1 + 1;
                print_1sdiff_line(
                    0 as *const *const libc::c_char,
                    ')' as i32 as libc::c_char,
                    &mut *((*files.as_mut_ptr().offset(1 as libc::c_int as isize))
                        .linbuf)
                        .offset(fresh7 as isize),
                );
            }
        }
        while i0 != limit0 {
            let fresh8 = i0;
            i0 = i0 + 1;
            print_1sdiff_line(
                &mut *((*files.as_mut_ptr().offset(0 as libc::c_int as isize)).linbuf)
                    .offset(fresh8 as isize),
                '(' as i32 as libc::c_char,
                0 as *const *const libc::c_char,
            );
        }
    }
    next0 = limit0;
    next1 = limit1;
}
unsafe extern "C" fn print_sdiff_hunk(mut hunk: *mut change) {
    let mut first0: lin = 0;
    let mut last0: lin = 0;
    let mut first1: lin = 0;
    let mut last1: lin = 0;
    let mut i: lin = 0;
    let mut j: lin = 0;
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
    print_sdiff_common_lines(first0, first1);
    if sdiff_merge_assist {
        fprintf(
            outfile,
            b"c%td,%td\n\0" as *const u8 as *const libc::c_char,
            last0 - first0 + 1 as libc::c_int as libc::c_long,
            last1 - first1 + 1 as libc::c_int as libc::c_long,
        );
    }
    if changes as libc::c_uint == CHANGED as libc::c_int as libc::c_uint {
        i = first0;
        j = first1;
        while i <= last0 && j <= last1 {
            print_1sdiff_line(
                &mut *((*files.as_mut_ptr().offset(0 as libc::c_int as isize)).linbuf)
                    .offset(i as isize),
                '|' as i32 as libc::c_char,
                &mut *((*files.as_mut_ptr().offset(1 as libc::c_int as isize)).linbuf)
                    .offset(j as isize),
            );
            i += 1;
            i;
            j += 1;
            j;
        }
        changes = ((if i <= last0 { OLD as libc::c_int } else { 0 as libc::c_int })
            + (if j <= last1 { NEW as libc::c_int } else { 0 as libc::c_int }))
            as changes;
        first0 = i;
        next0 = first0;
        first1 = j;
        next1 = first1;
    }
    if changes as libc::c_uint & NEW as libc::c_int as libc::c_uint != 0 {
        j = first1;
        while j <= last1 {
            print_1sdiff_line(
                0 as *const *const libc::c_char,
                '>' as i32 as libc::c_char,
                &mut *((*files.as_mut_ptr().offset(1 as libc::c_int as isize)).linbuf)
                    .offset(j as isize),
            );
            j += 1;
            j;
        }
        next1 = j;
    }
    if changes as libc::c_uint & OLD as libc::c_int as libc::c_uint != 0 {
        i = first0;
        while i <= last0 {
            print_1sdiff_line(
                &mut *((*files.as_mut_ptr().offset(0 as libc::c_int as isize)).linbuf)
                    .offset(i as isize),
                '<' as i32 as libc::c_char,
                0 as *const *const libc::c_char,
            );
            i += 1;
            i;
        }
        next0 = i;
    }
}
