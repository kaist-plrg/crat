use ::libc;
extern "C" {
    pub type _IO_wide_data;
    pub type _IO_codecvt;
    pub type _IO_marker;
    static mut stderr: *mut FILE;
    fn fclose(__stream: *mut FILE) -> libc::c_int;
    fn fopen(_: *const libc::c_char, _: *const libc::c_char) -> *mut FILE;
    fn fprintf(_: *mut FILE, _: *const libc::c_char, _: ...) -> libc::c_int;
    fn sprintf(_: *mut libc::c_char, _: *const libc::c_char, _: ...) -> libc::c_int;
    fn fgets(
        __s: *mut libc::c_char,
        __n: libc::c_int,
        __stream: *mut FILE,
    ) -> *mut libc::c_char;
    fn fputs(__s: *const libc::c_char, __stream: *mut FILE) -> libc::c_int;
    fn perror(__s: *const libc::c_char);
    fn exit(_: libc::c_int) -> !;
    fn strcpy(_: *mut libc::c_char, _: *const libc::c_char) -> *mut libc::c_char;
    fn strcmp(_: *const libc::c_char, _: *const libc::c_char) -> libc::c_int;
    fn strncmp(
        _: *const libc::c_char,
        _: *const libc::c_char,
        _: libc::c_ulong,
    ) -> libc::c_int;
    fn strdup(_: *const libc::c_char) -> *mut libc::c_char;
    fn strlen(_: *const libc::c_char) -> libc::c_ulong;
    fn __ctype_b_loc() -> *mut *const libc::c_ushort;
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct UsedFile {
    pub name: *mut libc::c_char,
    pub parent: *mut libc::c_char,
    pub line: libc::c_int,
}
pub static mut dir: *mut *mut libc::c_char = 0 as *const *mut libc::c_char
    as *mut *mut libc::c_char;
pub static mut fout: *mut FILE = 0 as *const FILE as *mut FILE;
pub static mut buff: [libc::c_char; 4096] = [0; 4096];
pub static mut used: [UsedFile; 1024] = [UsedFile {
    name: 0 as *const libc::c_char as *mut libc::c_char,
    parent: 0 as *const libc::c_char as *mut libc::c_char,
    line: 0,
}; 1024];
pub static mut nb_used: libc::c_int = 0 as libc::c_int;
unsafe fn main_0(
    mut argc: libc::c_int,
    mut argv: *mut *mut libc::c_char,
) -> libc::c_int {
    if argc < 4 as libc::c_int {
        fprintf(
            stderr,
            b"Usage cpp_headers in_file.h out_file.h search_dir...\n\0" as *const u8
                as *const libc::c_char,
        );
        return 1 as libc::c_int;
    }
    dir = argv.offset(3 as libc::c_int as isize);
    fout = fopen(
        *argv.offset(2 as libc::c_int as isize),
        b"w\0" as *const u8 as *const libc::c_char,
    );
    if fout.is_null() {
        perror(*argv.offset(2 as libc::c_int as isize));
        return 1 as libc::c_int;
    }
    Cpp_File(*argv.offset(1 as libc::c_int as isize), 0 as libc::c_int);
    fclose(fout);
    return 0 as libc::c_int;
}
pub unsafe extern "C" fn Cpp_File(
    mut name: *mut libc::c_char,
    mut skip_comment: libc::c_int,
) {
    let mut d: *mut *mut libc::c_char = 0 as *mut *mut libc::c_char;
    let mut i: libc::c_int = 0;
    let mut fin: *mut FILE = 0 as *mut FILE;
    let mut p: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut q: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut line: libc::c_int = 0 as libc::c_int;
    let mut name1: [libc::c_char; 1024] = [0; 1024];
    d = dir;
    while !(*d).is_null() {
        sprintf(
            buff.as_mut_ptr(),
            b"%s/%s\0" as *const u8 as *const libc::c_char,
            *d,
            name,
        );
        fin = fopen(buff.as_mut_ptr(), b"r\0" as *const u8 as *const libc::c_char);
        if !fin.is_null() {
            break;
        }
        d = d.offset(1);
        d;
    }
    if (*d).is_null() {
        fprintf(
            stderr,
            b"Cannot find the location of %s\n\0" as *const u8 as *const libc::c_char,
            name,
        );
        exit(1 as libc::c_int);
    }
    while !(fgets(
        buff.as_mut_ptr(),
        ::std::mem::size_of::<[libc::c_char; 4096]>() as libc::c_ulong as libc::c_int,
        fin,
    ))
        .is_null()
    {
        line += 1;
        line;
        if !(*buff.as_mut_ptr() as libc::c_int != '#' as i32) {
            p = buff.as_mut_ptr().offset(1 as libc::c_int as isize);
            while *(*__ctype_b_loc()).offset(*p as libc::c_int as isize) as libc::c_int
                & _ISspace as libc::c_int as libc::c_ushort as libc::c_int != 0
            {
                p = p.offset(1);
                p;
            }
            if !(strncmp(
                p,
                b"include\0" as *const u8 as *const libc::c_char,
                7 as libc::c_int as libc::c_ulong,
            ) != 0)
            {
                p = p.offset(7 as libc::c_int as isize);
                while *(*__ctype_b_loc()).offset(*p as libc::c_int as isize)
                    as libc::c_int
                    & _ISspace as libc::c_int as libc::c_ushort as libc::c_int != 0
                {
                    p = p.offset(1);
                    p;
                }
                if !(*p as libc::c_int != '"' as i32) {
                    p = p.offset(1);
                    p;
                    q = p.offset(strlen(p) as isize);
                    while *q as libc::c_int != '"' as i32 {
                        q = q.offset(-1);
                        q;
                    }
                    *q = '\0' as i32 as libc::c_char;
                    strcpy(name1.as_mut_ptr(), p);
                    i = 0 as libc::c_int;
                    while i < nb_used {
                        if strcmp(used[i as usize].name, name1.as_mut_ptr())
                            == 0 as libc::c_int
                        {
                            break;
                        }
                        i += 1;
                        i;
                    }
                    if i >= nb_used {
                        used[nb_used as usize].name = strdup(name1.as_mut_ptr());
                        used[nb_used as usize].parent = strdup(name);
                        used[nb_used as usize].line = line;
                        nb_used += 1;
                        nb_used;
                        Cpp_File(name1.as_mut_ptr(), 1 as libc::c_int);
                    }
                    continue;
                }
            }
        }
        fputs(buff.as_mut_ptr(), fout);
    }
    fclose(fin);
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
