use ::libc;
extern "C" {
    pub type _IO_wide_data;
    pub type _IO_codecvt;
    pub type _IO_marker;
    fn __overflow(_: *mut FILE, _: libc::c_int) -> libc::c_int;
    fn fputs_unlocked(__s: *const libc::c_char, __stream: *mut FILE) -> libc::c_int;
    static mut files: [file_data; 2];
    static mut outfile: *mut FILE;
    fn set_color_context(color_context: color_context);
    fn print_1_line_nl(_: *const libc::c_char, _: *const *const libc::c_char, _: bool);
    fn analyze_hunk(
        _: *mut change,
        _: *mut lin,
        _: *mut lin,
        _: *mut lin,
        _: *mut lin,
    ) -> changes;
    fn print_number_range(_: libc::c_char, _: *mut file_data, _: lin, _: lin);
    static change_letter: [libc::c_char; 4];
    fn begin_output();
    fn find_change(_: *mut change) -> *mut change;
    fn print_script(
        _: *mut change,
        _: Option::<unsafe extern "C" fn(*mut change) -> *mut change>,
        _: Option::<unsafe extern "C" fn(*mut change) -> ()>,
    );
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
unsafe extern "C" fn fputc_unlocked(
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
unsafe extern "C" fn putc_unlocked(
    mut __c: libc::c_int,
    mut __stream: *mut FILE,
) -> libc::c_int {
    return if ((*__stream)._IO_write_ptr >= (*__stream)._IO_write_end) as libc::c_int
        as libc::c_long != 0
    {
        __overflow(__stream, __c as libc::c_uchar as libc::c_int)
    } else {
        let fresh1 = (*__stream)._IO_write_ptr;
        (*__stream)._IO_write_ptr = ((*__stream)._IO_write_ptr).offset(1);
        *fresh1 = __c as libc::c_char;
        *fresh1 as libc::c_uchar as libc::c_int
    };
}
pub unsafe extern "C" fn print_normal_script(mut script: *mut change) {
    print_script(
        script,
        Some(find_change as unsafe extern "C" fn(*mut change) -> *mut change),
        Some(print_normal_hunk as unsafe extern "C" fn(*mut change) -> ()),
    );
}
unsafe extern "C" fn print_normal_hunk(mut hunk: *mut change) {
    let mut first0: lin = 0;
    let mut last0: lin = 0;
    let mut first1: lin = 0;
    let mut last1: lin = 0;
    let mut i: lin = 0;
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
    set_color_context(LINE_NUMBER_CONTEXT);
    print_number_range(
        ',' as i32 as libc::c_char,
        &mut *files.as_mut_ptr().offset(0 as libc::c_int as isize),
        first0,
        last0,
    );
    fputc_unlocked(change_letter[changes as usize] as libc::c_int, outfile);
    print_number_range(
        ',' as i32 as libc::c_char,
        &mut *files.as_mut_ptr().offset(1 as libc::c_int as isize),
        first1,
        last1,
    );
    set_color_context(RESET_CONTEXT);
    fputc_unlocked('\n' as i32, outfile);
    if changes as libc::c_uint & OLD as libc::c_int as libc::c_uint != 0 {
        i = first0;
        while i <= last0 {
            set_color_context(DELETE_CONTEXT);
            print_1_line_nl(
                b"<\0" as *const u8 as *const libc::c_char,
                &mut *((*files.as_mut_ptr().offset(0 as libc::c_int as isize)).linbuf)
                    .offset(i as isize),
                1 as libc::c_int != 0,
            );
            set_color_context(RESET_CONTEXT);
            if *(*(files[0 as libc::c_int as usize].linbuf)
                .offset((i + 1 as libc::c_int as libc::c_long) as isize))
                .offset(-(1 as libc::c_int) as isize) as libc::c_int == '\n' as i32
            {
                putc_unlocked('\n' as i32, outfile);
            }
            i += 1;
            i;
        }
    }
    if changes as libc::c_uint == CHANGED as libc::c_int as libc::c_uint {
        fputs_unlocked(b"---\n\0" as *const u8 as *const libc::c_char, outfile);
    }
    if changes as libc::c_uint & NEW as libc::c_int as libc::c_uint != 0 {
        i = first1;
        while i <= last1 {
            set_color_context(ADD_CONTEXT);
            print_1_line_nl(
                b">\0" as *const u8 as *const libc::c_char,
                &mut *((*files.as_mut_ptr().offset(1 as libc::c_int as isize)).linbuf)
                    .offset(i as isize),
                1 as libc::c_int != 0,
            );
            set_color_context(RESET_CONTEXT);
            if *(*(files[1 as libc::c_int as usize].linbuf)
                .offset((i + 1 as libc::c_int as libc::c_long) as isize))
                .offset(-(1 as libc::c_int) as isize) as libc::c_int == '\n' as i32
            {
                putc_unlocked('\n' as i32, outfile);
            }
            i += 1;
            i;
        }
    }
}
