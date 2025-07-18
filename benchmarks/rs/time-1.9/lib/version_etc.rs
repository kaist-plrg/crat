use ::libc;
extern "C" {
    pub type _IO_wide_data;
    pub type _IO_codecvt;
    pub type _IO_marker;
    static mut stdout: *mut FILE;
    fn fprintf(_: *mut FILE, _: *const libc::c_char, _: ...) -> libc::c_int;
    fn printf(_: *const libc::c_char, _: ...) -> libc::c_int;
    fn fputs(__s: *const libc::c_char, __stream: *mut FILE) -> libc::c_int;
    static version_etc_copyright: [libc::c_char; 0];
    fn abort() -> !;
}
pub type __builtin_va_list = [__va_list_tag; 1];
#[derive(Copy, Clone)]
#[repr(C)]
pub struct __va_list_tag {
    pub gp_offset: libc::c_uint,
    pub fp_offset: libc::c_uint,
    pub overflow_arg_area: *mut libc::c_void,
    pub reg_save_area: *mut libc::c_void,
}
pub type va_list = __builtin_va_list;
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
pub const COPYRIGHT_YEAR: C2RustUnnamed = 2018;
pub type C2RustUnnamed = libc::c_uint;
pub unsafe extern "C" fn version_etc_arn(
    mut stream: *mut FILE,
    mut command_name: *const libc::c_char,
    mut package: *const libc::c_char,
    mut version: *const libc::c_char,
    mut authors: *const *const libc::c_char,
    mut n_authors: size_t,
) {
    if !command_name.is_null() {
        fprintf(
            stream,
            b"%s (%s) %s\n\0" as *const u8 as *const libc::c_char,
            command_name,
            package,
            version,
        );
    } else {
        fprintf(
            stream,
            b"%s %s\n\0" as *const u8 as *const libc::c_char,
            package,
            version,
        );
    }
    fprintf(
        stream,
        version_etc_copyright.as_ptr(),
        b"(C)\0" as *const u8 as *const libc::c_char,
        COPYRIGHT_YEAR as libc::c_int,
    );
    fputs(
        b"\nLicense GPLv3+: GNU GPL version 3 or later <https://gnu.org/licenses/gpl.html>.\nThis is free software: you are free to change and redistribute it.\nThere is NO WARRANTY, to the extent permitted by law.\n\n\0"
            as *const u8 as *const libc::c_char,
        stream,
    );
    match n_authors {
        0 => {
            abort();
        }
        1 => {
            fprintf(
                stream,
                b"Written by %s.\n\0" as *const u8 as *const libc::c_char,
                *authors.offset(0 as libc::c_int as isize),
            );
        }
        2 => {
            fprintf(
                stream,
                b"Written by %s and %s.\n\0" as *const u8 as *const libc::c_char,
                *authors.offset(0 as libc::c_int as isize),
                *authors.offset(1 as libc::c_int as isize),
            );
        }
        3 => {
            fprintf(
                stream,
                b"Written by %s, %s, and %s.\n\0" as *const u8 as *const libc::c_char,
                *authors.offset(0 as libc::c_int as isize),
                *authors.offset(1 as libc::c_int as isize),
                *authors.offset(2 as libc::c_int as isize),
            );
        }
        4 => {
            fprintf(
                stream,
                b"Written by %s, %s, %s,\nand %s.\n\0" as *const u8
                    as *const libc::c_char,
                *authors.offset(0 as libc::c_int as isize),
                *authors.offset(1 as libc::c_int as isize),
                *authors.offset(2 as libc::c_int as isize),
                *authors.offset(3 as libc::c_int as isize),
            );
        }
        5 => {
            fprintf(
                stream,
                b"Written by %s, %s, %s,\n%s, and %s.\n\0" as *const u8
                    as *const libc::c_char,
                *authors.offset(0 as libc::c_int as isize),
                *authors.offset(1 as libc::c_int as isize),
                *authors.offset(2 as libc::c_int as isize),
                *authors.offset(3 as libc::c_int as isize),
                *authors.offset(4 as libc::c_int as isize),
            );
        }
        6 => {
            fprintf(
                stream,
                b"Written by %s, %s, %s,\n%s, %s, and %s.\n\0" as *const u8
                    as *const libc::c_char,
                *authors.offset(0 as libc::c_int as isize),
                *authors.offset(1 as libc::c_int as isize),
                *authors.offset(2 as libc::c_int as isize),
                *authors.offset(3 as libc::c_int as isize),
                *authors.offset(4 as libc::c_int as isize),
                *authors.offset(5 as libc::c_int as isize),
            );
        }
        7 => {
            fprintf(
                stream,
                b"Written by %s, %s, %s,\n%s, %s, %s, and %s.\n\0" as *const u8
                    as *const libc::c_char,
                *authors.offset(0 as libc::c_int as isize),
                *authors.offset(1 as libc::c_int as isize),
                *authors.offset(2 as libc::c_int as isize),
                *authors.offset(3 as libc::c_int as isize),
                *authors.offset(4 as libc::c_int as isize),
                *authors.offset(5 as libc::c_int as isize),
                *authors.offset(6 as libc::c_int as isize),
            );
        }
        8 => {
            fprintf(
                stream,
                b"Written by %s, %s, %s,\n%s, %s, %s, %s,\nand %s.\n\0" as *const u8
                    as *const libc::c_char,
                *authors.offset(0 as libc::c_int as isize),
                *authors.offset(1 as libc::c_int as isize),
                *authors.offset(2 as libc::c_int as isize),
                *authors.offset(3 as libc::c_int as isize),
                *authors.offset(4 as libc::c_int as isize),
                *authors.offset(5 as libc::c_int as isize),
                *authors.offset(6 as libc::c_int as isize),
                *authors.offset(7 as libc::c_int as isize),
            );
        }
        9 => {
            fprintf(
                stream,
                b"Written by %s, %s, %s,\n%s, %s, %s, %s,\n%s, and %s.\n\0" as *const u8
                    as *const libc::c_char,
                *authors.offset(0 as libc::c_int as isize),
                *authors.offset(1 as libc::c_int as isize),
                *authors.offset(2 as libc::c_int as isize),
                *authors.offset(3 as libc::c_int as isize),
                *authors.offset(4 as libc::c_int as isize),
                *authors.offset(5 as libc::c_int as isize),
                *authors.offset(6 as libc::c_int as isize),
                *authors.offset(7 as libc::c_int as isize),
                *authors.offset(8 as libc::c_int as isize),
            );
        }
        _ => {
            fprintf(
                stream,
                b"Written by %s, %s, %s,\n%s, %s, %s, %s,\n%s, %s, and others.\n\0"
                    as *const u8 as *const libc::c_char,
                *authors.offset(0 as libc::c_int as isize),
                *authors.offset(1 as libc::c_int as isize),
                *authors.offset(2 as libc::c_int as isize),
                *authors.offset(3 as libc::c_int as isize),
                *authors.offset(4 as libc::c_int as isize),
                *authors.offset(5 as libc::c_int as isize),
                *authors.offset(6 as libc::c_int as isize),
                *authors.offset(7 as libc::c_int as isize),
                *authors.offset(8 as libc::c_int as isize),
            );
        }
    };
}
pub unsafe extern "C" fn version_etc_ar(
    mut stream: *mut FILE,
    mut command_name: *const libc::c_char,
    mut package: *const libc::c_char,
    mut version: *const libc::c_char,
    mut authors: *const *const libc::c_char,
) {
    let mut n_authors: size_t = 0;
    n_authors = 0 as libc::c_int as size_t;
    while !(*authors.offset(n_authors as isize)).is_null() {
        n_authors = n_authors.wrapping_add(1);
        n_authors;
    }
    version_etc_arn(stream, command_name, package, version, authors, n_authors);
}
pub unsafe extern "C" fn version_etc_va(
    mut stream: *mut FILE,
    mut command_name: *const libc::c_char,
    mut package: *const libc::c_char,
    mut version: *const libc::c_char,
    mut authors: ::std::ffi::VaList,
) {
    let mut n_authors: size_t = 0;
    let mut authtab: [*const libc::c_char; 10] = [0 as *const libc::c_char; 10];
    n_authors = 0 as libc::c_int as size_t;
    while n_authors < 10 as libc::c_int as libc::c_ulong
        && {
            authtab[n_authors as usize] = authors.arg::<*const libc::c_char>();
            !(authtab[n_authors as usize]).is_null()
        }
    {
        n_authors = n_authors.wrapping_add(1);
        n_authors;
    }
    version_etc_arn(
        stream,
        command_name,
        package,
        version,
        authtab.as_mut_ptr(),
        n_authors,
    );
}
pub unsafe extern "C" fn version_etc(
    mut stream: *mut FILE,
    mut command_name: *const libc::c_char,
    mut package: *const libc::c_char,
    mut version: *const libc::c_char,
    mut args: ...
) {
    let mut authors: ::std::ffi::VaListImpl;
    authors = args.clone();
    version_etc_va(stream, command_name, package, version, authors.as_va_list());
}
pub unsafe extern "C" fn emit_bug_reporting_address() {
    printf(
        b"\nReport bugs to: %s\n\0" as *const u8 as *const libc::c_char,
        b"bug-time@gnu.org\0" as *const u8 as *const libc::c_char,
    );
    printf(
        b"%s home page: <%s>\n\0" as *const u8 as *const libc::c_char,
        b"GNU Time\0" as *const u8 as *const libc::c_char,
        b"https://www.gnu.org/software/time/\0" as *const u8 as *const libc::c_char,
    );
    fputs(
        b"General help using GNU software: <https://www.gnu.org/gethelp/>\n\0"
            as *const u8 as *const libc::c_char,
        stdout,
    );
}
