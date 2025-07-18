use ::libc;
extern "C" {
    pub type _IO_wide_data;
    pub type _IO_codecvt;
    pub type _IO_marker;
    pub type HASH_T;
    fn fclose(__stream: *mut FILE) -> libc::c_int;
    fn fopen(_: *const libc::c_char, _: *const libc::c_char) -> *mut FILE;
    fn printf(_: *const libc::c_char, _: ...) -> libc::c_int;
    fn snprintf(
        _: *mut libc::c_char,
        _: libc::c_ulong,
        _: *const libc::c_char,
        _: ...
    ) -> libc::c_int;
    fn fgetc(__stream: *mut FILE) -> libc::c_int;
    fn fgets(
        __s: *mut libc::c_char,
        __n: libc::c_int,
        __stream: *mut FILE,
    ) -> *mut libc::c_char;
    fn display_help();
    fn realloc(_: *mut libc::c_void, _: libc::c_ulong) -> *mut libc::c_void;
    fn free(__ptr: *mut libc::c_void);
    fn exit(_: libc::c_int) -> !;
    fn __ctype_b_loc() -> *mut *const libc::c_ushort;
    fn memset(
        _: *mut libc::c_void,
        _: libc::c_int,
        _: libc::c_ulong,
    ) -> *mut libc::c_void;
    fn strdup(_: *const libc::c_char) -> *mut libc::c_char;
    fn strchr(_: *const libc::c_char, _: libc::c_int) -> *mut libc::c_char;
    fn strstr(_: *const libc::c_char, _: *const libc::c_char) -> *mut libc::c_char;
    fn strlen(_: *const libc::c_char) -> libc::c_ulong;
    fn new_hash() -> HASH;
    fn hash_add(this: HASH, key: *mut libc::c_char, value: *mut libc::c_void);
    fn hash_destroy(this: HASH) -> HASH;
    fn escape(buf: *mut libc::c_char) -> *mut libc::c_char;
    fn evaluate(hash: HASH, buf: *mut libc::c_char) -> *mut libc::c_char;
    fn chomp(str: *mut libc::c_char) -> *mut libc::c_char;
    fn trim(str: *mut libc::c_char) -> *mut libc::c_char;
    fn xstrdup(str: *const libc::c_char) -> *mut libc::c_char;
    fn xmalloc(_: size_t) -> *mut libc::c_void;
    fn xfree(ptr: *mut libc::c_void);
    fn NOTIFY(L: LEVEL, fmt: *const libc::c_char, _: ...);
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
pub type BOOLEAN = libc::c_uint;
pub const boolean_true: BOOLEAN = 1;
pub const boolean_false: BOOLEAN = 0;
pub type HASH = *mut HASH_T;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct LINES {
    pub index: libc::c_int,
    pub line: *mut *mut libc::c_char,
}
pub type LEVEL = libc::c_uint;
pub const FATAL: LEVEL = 3;
pub const ERROR: LEVEL = 2;
pub const WARNING: LEVEL = 1;
pub const DEBUG: LEVEL = 0;
pub unsafe extern "C" fn parse(mut str: *mut libc::c_char) {
    let mut ch: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut sp: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut sl: *mut libc::c_char = 0 as *mut libc::c_char;
    str = trim(str);
    if *str.offset(0 as libc::c_int as isize) as libc::c_int == '#' as i32 {
        *str.offset(0 as libc::c_int as isize) = '\0' as i32 as libc::c_char;
    }
    sp = strchr(str, ' ' as i32);
    sl = strchr(str, '/' as i32);
    if sl.is_null() && !sp.is_null() {
        ch = strstr(str, b"#\0" as *const u8 as *const libc::c_char);
        if !ch.is_null() {
            *ch = '\0' as i32 as libc::c_char;
        }
    }
    ch = strstr(str, b"\n\0" as *const u8 as *const libc::c_char);
    if !ch.is_null() {
        *ch = '\0' as i32 as libc::c_char;
    }
    trim(str);
}
pub unsafe extern "C" fn count(
    mut s: *mut libc::c_char,
    mut c: libc::c_char,
) -> libc::c_int {
    return if *s as libc::c_int == '\0' as i32 {
        0 as libc::c_int
    } else {
        count(s.offset(1 as libc::c_int as isize), c)
            + (*s as libc::c_int == c as libc::c_int) as libc::c_int
    };
}
pub unsafe extern "C" fn read_cfg_file(
    mut l: *mut LINES,
    mut filename: *mut libc::c_char,
) -> libc::c_int {
    let mut file: *mut FILE = 0 as *mut FILE;
    let mut H: HASH = 0 as *mut HASH_T;
    let mut line: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut option: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut value: *mut libc::c_char = 0 as *mut libc::c_char;
    if l.is_null() {
        printf(b"Structure not initialized!\n\0" as *const u8 as *const libc::c_char);
        return -(1 as libc::c_int);
    }
    file = fopen(filename, b"r\0" as *const u8 as *const libc::c_char);
    if file.is_null() {
        NOTIFY(
            WARNING,
            b"unable to open file: %s\0" as *const u8 as *const libc::c_char,
            filename,
        );
        display_help();
        exit(1 as libc::c_int);
    }
    line = xmalloc(40000 as libc::c_int as size_t) as *mut libc::c_char;
    memset(
        line as *mut libc::c_void,
        '\0' as i32,
        40000 as libc::c_int as libc::c_ulong,
    );
    H = new_hash();
    (*l).index = 0 as libc::c_int;
    while !(fgets(line, 40000 as libc::c_int, file)).is_null() {
        let mut num: libc::c_int = 0;
        let mut p: *mut libc::c_char = strchr(line, '\n' as i32);
        if !p.is_null() {
            *p = '\0' as i32 as libc::c_char;
        } else {
            num = fgetc(file);
            if num != -(1 as libc::c_int) {
                loop {
                    num = fgetc(file);
                    if !(num != -(1 as libc::c_int) && num != '\n' as i32) {
                        break;
                    }
                }
                *line.offset(0 as libc::c_int as isize) = '\0' as i32 as libc::c_char;
            }
        }
        parse(line);
        chomp(line);
        if !(strlen(line) == 0 as libc::c_int as libc::c_ulong) {
            if is_variable_line(line) as u64 != 0 {
                let mut tmp: *mut libc::c_char = line;
                option = tmp;
                while *tmp as libc::c_int != 0
                    && *(*__ctype_b_loc())
                        .offset(
                            *tmp as libc::c_int as libc::c_uchar as libc::c_int as isize,
                        ) as libc::c_int
                        & _ISspace as libc::c_int as libc::c_ushort as libc::c_int == 0
                    && !('=' as i32 == *tmp as libc::c_int
                        || ':' as i32 == *tmp as libc::c_int)
                {
                    tmp = tmp.offset(1);
                    tmp;
                }
                let fresh0 = tmp;
                tmp = tmp.offset(1);
                *fresh0 = 0 as libc::c_int as libc::c_char;
                while *(*__ctype_b_loc())
                    .offset(*tmp as libc::c_int as libc::c_uchar as libc::c_int as isize)
                    as libc::c_int
                    & _ISspace as libc::c_int as libc::c_ushort as libc::c_int != 0
                    || ('=' as i32 == *tmp as libc::c_int
                        || ':' as i32 == *tmp as libc::c_int)
                {
                    tmp = tmp.offset(1);
                    tmp;
                }
                value = tmp;
                while *tmp != 0 {
                    tmp = tmp.offset(1);
                    tmp;
                }
                let fresh1 = tmp;
                tmp = tmp.offset(1);
                *fresh1 = 0 as libc::c_int as libc::c_char;
                hash_add(H, option, value as *mut libc::c_void);
            } else {
                let mut tmp_0: *mut libc::c_char = xstrdup(line);
                let mut r: libc::c_int = 0 as libc::c_int;
                let mut cnt: libc::c_int = 0 as libc::c_int;
                cnt += count(tmp_0, '$' as i32 as libc::c_char);
                while !(strstr(tmp_0, b"$\0" as *const u8 as *const libc::c_char))
                    .is_null()
                {
                    if !(strstr(tmp_0, b"\\$\0" as *const u8 as *const libc::c_char))
                        .is_null()
                    {
                        tmp_0 = escape(tmp_0);
                    } else {
                        tmp_0 = evaluate(H, tmp_0);
                    }
                    r += 1;
                    r;
                    if r == cnt {
                        break;
                    }
                }
                (*l)
                    .line = realloc(
                    (*l).line as *mut libc::c_void,
                    (::std::mem::size_of::<*mut libc::c_char>() as libc::c_ulong)
                        .wrapping_mul(((*l).index + 1 as libc::c_int) as libc::c_ulong),
                ) as *mut *mut libc::c_char;
                let ref mut fresh2 = *((*l).line).offset((*l).index as isize);
                *fresh2 = strdup(tmp_0);
                (*l).index += 1;
                (*l).index;
                free(tmp_0 as *mut libc::c_void);
            }
        }
        memset(
            line as *mut libc::c_void,
            0 as libc::c_int,
            40000 as libc::c_int as libc::c_ulong,
        );
    }
    fclose(file);
    xfree(line as *mut libc::c_void);
    hash_destroy(H);
    return (*l).index;
}
pub unsafe extern "C" fn read_cmd_line(
    mut l: *mut LINES,
    mut url: *mut libc::c_char,
) -> libc::c_int {
    let mut x: libc::c_int = 0 as libc::c_int;
    let mut head: [libc::c_char; 40000] = [0; 40000];
    if l.is_null() {
        printf(b"Structure not initialized!\n\0" as *const u8 as *const libc::c_char);
        return -(1 as libc::c_int);
    }
    (*l).index = 0 as libc::c_int;
    while x < 4 as libc::c_int {
        snprintf(
            head.as_mut_ptr(),
            ::std::mem::size_of::<[libc::c_char; 40000]>() as libc::c_ulong,
            b"%s\0" as *const u8 as *const libc::c_char,
            url,
        );
        parse(head.as_mut_ptr());
        chomp(head.as_mut_ptr());
        if !(strlen(head.as_mut_ptr()) == 0 as libc::c_int as libc::c_ulong) {
            (*l)
                .line = realloc(
                (*l).line as *mut libc::c_void,
                (::std::mem::size_of::<*mut libc::c_char>() as libc::c_ulong)
                    .wrapping_mul(((*l).index + 1 as libc::c_int) as libc::c_ulong),
            ) as *mut *mut libc::c_char;
            let ref mut fresh3 = *((*l).line).offset((*l).index as isize);
            *fresh3 = strdup(head.as_mut_ptr());
            (*l).index += 1;
            (*l).index;
        }
        x += 1;
        x;
    }
    return (*l).index;
}
pub unsafe extern "C" fn is_variable_line(mut line: *mut libc::c_char) -> BOOLEAN {
    let mut pos: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut x: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut c: libc::c_char = 0;
    pos = strstr(line, b"=\0" as *const u8 as *const libc::c_char);
    if !pos.is_null() {
        x = line;
        while x < pos {
            c = *x;
            if ((c as libc::c_int) < 'a' as i32 || c as libc::c_int > 'z' as i32)
                && ((c as libc::c_int) < 'A' as i32 || c as libc::c_int > 'Z' as i32)
                && ((c as libc::c_int) < '0' as i32 || c as libc::c_int > '9' as i32)
                && c as libc::c_int != '_' as i32
            {
                return boolean_false;
            }
            x = x.offset(1);
            x;
        }
    } else {
        return boolean_false
    }
    return boolean_true;
}
