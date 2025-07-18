use ::libc;
extern "C" {
    pub type _IO_wide_data;
    pub type _IO_codecvt;
    pub type _IO_marker;
    static mut stderr: *mut FILE;
    fn fprintf(_: *mut FILE, _: *const libc::c_char, _: ...) -> libc::c_int;
    fn printf(_: *const libc::c_char, _: ...) -> libc::c_int;
    fn malloc(_: libc::c_ulong) -> *mut libc::c_void;
    fn free(__ptr: *mut libc::c_void);
    fn exit(_: libc::c_int) -> !;
    fn strcpy(_: *mut libc::c_char, _: *const libc::c_char) -> *mut libc::c_char;
    fn strcmp(_: *const libc::c_char, _: *const libc::c_char) -> libc::c_int;
    fn strchr(_: *const libc::c_char, _: libc::c_int) -> *mut libc::c_char;
    fn strlen(_: *const libc::c_char) -> libc::c_ulong;
}
pub type __off_t = libc::c_long;
pub type __off64_t = libc::c_long;
pub type size_t = libc::c_ulong;
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
static mut version: *mut libc::c_char = b"$Revision: 1.15 $\0" as *const u8
    as *const libc::c_char as *mut libc::c_char;
static mut quote_all: libc::c_int = 0 as libc::c_int;
static mut openquote: libc::c_char = '"' as i32 as libc::c_char;
static mut closequote: libc::c_char = '"' as i32 as libc::c_char;
static mut meta_escape: *mut libc::c_char = b"\\\0" as *const u8 as *const libc::c_char
    as *mut libc::c_char;
static mut meta_escape_buf: [libc::c_char; 2] = [0; 2];
static mut metachars: *mut libc::c_char = 0 as *const libc::c_char as *mut libc::c_char;
static mut num_metachars: libc::c_int = 0 as libc::c_int;
static mut size_metachars: libc::c_int = 0 as libc::c_int;
unsafe extern "C" fn pr_usage() {
    fprintf(
        stderr,
        b"usage: lessecho [-ox] [-cx] [-pn] [-dn] [-mx] [-nn] [-ex] [-fn] [-a] file ...\n\0"
            as *const u8 as *const libc::c_char,
    );
}
unsafe extern "C" fn pr_version() {
    let mut p: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut buf: [libc::c_char; 10] = [0; 10];
    let mut pbuf: *mut libc::c_char = buf.as_mut_ptr();
    p = version;
    while *p as libc::c_int != ' ' as i32 {
        if *p as libc::c_int == '\0' as i32 {
            return;
        }
        p = p.offset(1);
        p;
    }
    p = p.offset(1);
    p;
    while *p as libc::c_int != '$' as i32 && *p as libc::c_int != ' ' as i32
        && *p as libc::c_int != '\0' as i32
    {
        let fresh0 = pbuf;
        pbuf = pbuf.offset(1);
        *fresh0 = *p;
        p = p.offset(1);
        p;
    }
    *pbuf = '\0' as i32 as libc::c_char;
    printf(b"%s\n\0" as *const u8 as *const libc::c_char, buf.as_mut_ptr());
}
unsafe extern "C" fn pr_error(mut s: *mut libc::c_char) {
    fprintf(stderr, b"%s\n\0" as *const u8 as *const libc::c_char, s);
    exit(1 as libc::c_int);
}
unsafe extern "C" fn lstrtol(
    mut s: *mut libc::c_char,
    mut pend: *mut *mut libc::c_char,
    mut radix: libc::c_int,
) -> libc::c_long {
    let mut v: libc::c_int = 0;
    let mut neg: libc::c_int = 0 as libc::c_int;
    let mut n: libc::c_long = 0 as libc::c_int as libc::c_long;
    while *s as libc::c_int == ' ' as i32 || *s as libc::c_int == '\t' as i32 {
        s = s.offset(1);
        s;
    }
    if *s as libc::c_int == '-' as i32 {
        neg = 1 as libc::c_int;
        s = s.offset(1);
        s;
    } else if *s as libc::c_int == '+' as i32 {
        s = s.offset(1);
        s;
    }
    if radix == 0 as libc::c_int {
        radix = 10 as libc::c_int;
        if *s as libc::c_int == '0' as i32 {
            s = s.offset(1);
            match *s as libc::c_int {
                120 => {
                    radix = 16 as libc::c_int;
                    s = s.offset(1);
                    s;
                }
                _ => {
                    radix = 8 as libc::c_int;
                }
            }
        }
    }
    loop {
        if *s as libc::c_int >= '0' as i32 && *s as libc::c_int <= '9' as i32 {
            v = *s as libc::c_int - '0' as i32;
        } else if *s as libc::c_int >= 'a' as i32 && *s as libc::c_int <= 'f' as i32 {
            v = *s as libc::c_int - 'a' as i32 + 10 as libc::c_int;
        } else {
            if !(*s as libc::c_int >= 'A' as i32 && *s as libc::c_int <= 'F' as i32) {
                break;
            }
            v = *s as libc::c_int - 'A' as i32 + 10 as libc::c_int;
        }
        if v >= radix {
            break;
        }
        n = n * radix as libc::c_long + v as libc::c_long;
        s = s.offset(1);
        s;
    }
    if !pend.is_null() {
        while *s as libc::c_int == ' ' as i32 || *s as libc::c_int == '\t' as i32 {
            s = s.offset(1);
            s;
        }
        *pend = s;
    }
    if neg != 0 {
        return -n;
    }
    return n;
}
unsafe extern "C" fn add_metachar(mut ch: libc::c_int) {
    if num_metachars + 1 as libc::c_int >= size_metachars {
        let mut p: *mut libc::c_char = 0 as *mut libc::c_char;
        size_metachars = if size_metachars > 0 as libc::c_int {
            size_metachars * 2 as libc::c_int
        } else {
            16 as libc::c_int
        };
        p = malloc(size_metachars as libc::c_ulong) as *mut libc::c_char;
        if p.is_null() {
            pr_error(
                b"Cannot allocate memory\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
            );
        }
        if !metachars.is_null() {
            strcpy(p, metachars);
            free(metachars as *mut libc::c_void);
        }
        metachars = p;
    }
    let fresh1 = num_metachars;
    num_metachars = num_metachars + 1;
    *metachars.offset(fresh1 as isize) = ch as libc::c_char;
    *metachars.offset(num_metachars as isize) = '\0' as i32 as libc::c_char;
}
unsafe extern "C" fn is_metachar(mut ch: libc::c_int) -> libc::c_int {
    return (!metachars.is_null() && !(strchr(metachars, ch)).is_null()) as libc::c_int;
}
unsafe fn main_0(
    mut argc: libc::c_int,
    mut argv: *mut *mut libc::c_char,
) -> libc::c_int {
    let mut arg: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut s: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut no_more_options: libc::c_int = 0;
    no_more_options = 0 as libc::c_int;
    loop {
        argc -= 1;
        if !(argc > 0 as libc::c_int) {
            break;
        }
        argv = argv.offset(1);
        arg = *argv;
        if *arg as libc::c_int != '-' as i32 || no_more_options != 0 {
            break;
        }
        let mut current_block_35: u64;
        arg = arg.offset(1);
        match *arg as libc::c_int {
            97 => {
                quote_all = 1 as libc::c_int;
                current_block_35 = 4090602189656566074;
            }
            99 => {
                arg = arg.offset(1);
                closequote = *arg;
                current_block_35 = 4090602189656566074;
            }
            100 => {
                arg = arg.offset(1);
                closequote = lstrtol(arg, &mut s, 0 as libc::c_int) as libc::c_char;
                if s == arg {
                    pr_error(
                        b"Missing number after -d\0" as *const u8 as *const libc::c_char
                            as *mut libc::c_char,
                    );
                }
                current_block_35 = 4090602189656566074;
            }
            101 => {
                arg = arg.offset(1);
                if strcmp(arg, b"-\0" as *const u8 as *const libc::c_char)
                    == 0 as libc::c_int
                {
                    meta_escape = b"\0" as *const u8 as *const libc::c_char
                        as *mut libc::c_char;
                } else {
                    meta_escape = arg;
                }
                current_block_35 = 4090602189656566074;
            }
            102 => {
                arg = arg.offset(1);
                meta_escape_buf[0 as libc::c_int
                    as usize] = lstrtol(arg, &mut s, 0 as libc::c_int) as libc::c_char;
                meta_escape_buf[1 as libc::c_int as usize] = '\0' as i32 as libc::c_char;
                meta_escape = meta_escape_buf.as_mut_ptr();
                if s == arg {
                    pr_error(
                        b"Missing number after -f\0" as *const u8 as *const libc::c_char
                            as *mut libc::c_char,
                    );
                }
                current_block_35 = 4090602189656566074;
            }
            111 => {
                arg = arg.offset(1);
                openquote = *arg;
                current_block_35 = 4090602189656566074;
            }
            112 => {
                arg = arg.offset(1);
                openquote = lstrtol(arg, &mut s, 0 as libc::c_int) as libc::c_char;
                if s == arg {
                    pr_error(
                        b"Missing number after -p\0" as *const u8 as *const libc::c_char
                            as *mut libc::c_char,
                    );
                }
                current_block_35 = 4090602189656566074;
            }
            109 => {
                arg = arg.offset(1);
                add_metachar(*arg as libc::c_int);
                current_block_35 = 4090602189656566074;
            }
            110 => {
                arg = arg.offset(1);
                add_metachar(lstrtol(arg, &mut s, 0 as libc::c_int) as libc::c_int);
                if s == arg {
                    pr_error(
                        b"Missing number after -n\0" as *const u8 as *const libc::c_char
                            as *mut libc::c_char,
                    );
                }
                current_block_35 = 4090602189656566074;
            }
            63 => {
                pr_usage();
                return 0 as libc::c_int;
            }
            45 => {
                arg = arg.offset(1);
                if *arg as libc::c_int == '\0' as i32 {
                    no_more_options = 1 as libc::c_int;
                    current_block_35 = 4090602189656566074;
                } else {
                    if strcmp(arg, b"version\0" as *const u8 as *const libc::c_char)
                        == 0 as libc::c_int
                    {
                        pr_version();
                        return 0 as libc::c_int;
                    }
                    if strcmp(arg, b"help\0" as *const u8 as *const libc::c_char)
                        == 0 as libc::c_int
                    {
                        pr_usage();
                        return 0 as libc::c_int;
                    }
                    pr_error(
                        b"Invalid option after --\0" as *const u8 as *const libc::c_char
                            as *mut libc::c_char,
                    );
                    current_block_35 = 15167332338547839163;
                }
            }
            _ => {
                current_block_35 = 15167332338547839163;
            }
        }
        match current_block_35 {
            15167332338547839163 => {
                pr_error(
                    b"Invalid option letter\0" as *const u8 as *const libc::c_char
                        as *mut libc::c_char,
                );
            }
            _ => {}
        }
    }
    loop {
        let fresh2 = argc;
        argc = argc - 1;
        if !(fresh2 > 0 as libc::c_int) {
            break;
        }
        let mut has_meta: libc::c_int = 0 as libc::c_int;
        let fresh3 = argv;
        argv = argv.offset(1);
        arg = *fresh3;
        s = arg;
        while *s as libc::c_int != '\0' as i32 {
            if is_metachar(*s as libc::c_int) != 0 {
                has_meta = 1 as libc::c_int;
                break;
            } else {
                s = s.offset(1);
                s;
            }
        }
        if quote_all != 0
            || has_meta != 0 && strlen(meta_escape) == 0 as libc::c_int as libc::c_ulong
        {
            printf(
                b"%c%s%c\0" as *const u8 as *const libc::c_char,
                openquote as libc::c_int,
                arg,
                closequote as libc::c_int,
            );
        } else {
            s = arg;
            while *s as libc::c_int != '\0' as i32 {
                if is_metachar(*s as libc::c_int) != 0 {
                    printf(b"%s\0" as *const u8 as *const libc::c_char, meta_escape);
                }
                printf(b"%c\0" as *const u8 as *const libc::c_char, *s as libc::c_int);
                s = s.offset(1);
                s;
            }
        }
        if argc > 0 as libc::c_int {
            printf(b" \0" as *const u8 as *const libc::c_char);
        } else {
            printf(b"\n\0" as *const u8 as *const libc::c_char);
        }
    }
    return 0 as libc::c_int;
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
