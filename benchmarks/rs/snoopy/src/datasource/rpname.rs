use ::libc;
extern "C" {
    pub type _IO_wide_data;
    pub type _IO_codecvt;
    pub type _IO_marker;
    fn getpid() -> __pid_t;
    fn fclose(__stream: *mut FILE) -> libc::c_int;
    fn fopen(_: *const libc::c_char, _: *const libc::c_char) -> *mut FILE;
    fn snprintf(
        _: *mut libc::c_char,
        _: libc::c_ulong,
        _: *const libc::c_char,
        _: ...
    ) -> libc::c_int;
    fn __getdelim(
        __lineptr: *mut *mut libc::c_char,
        __n: *mut size_t,
        __delimiter: libc::c_int,
        __stream: *mut FILE,
    ) -> __ssize_t;
    fn strncpy(
        _: *mut libc::c_char,
        _: *const libc::c_char,
        _: libc::c_ulong,
    ) -> *mut libc::c_char;
    fn strcmp(_: *const libc::c_char, _: *const libc::c_char) -> libc::c_int;
    fn strdup(__s: *const libc::c_char) -> *mut libc::c_char;
    fn strchr(_: *const libc::c_char, _: libc::c_int) -> *mut libc::c_char;
    fn strstr(_: *const libc::c_char, _: *const libc::c_char) -> *mut libc::c_char;
    fn strlen(_: *const libc::c_char) -> libc::c_ulong;
    fn strtol(
        _: *const libc::c_char,
        _: *mut *mut libc::c_char,
        _: libc::c_int,
    ) -> libc::c_long;
    fn free(__ptr: *mut libc::c_void);
}
pub type __pid_t = libc::c_int;
pub type size_t = libc::c_ulong;
pub type FILE = _IO_FILE;
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
pub type __off64_t = libc::c_long;
pub type _IO_lock_t = ();
pub type __off_t = libc::c_long;
pub type __ssize_t = libc::c_long;
#[inline]
unsafe extern "C" fn getline(
    mut __lineptr: *mut *mut libc::c_char,
    mut __n: *mut size_t,
    mut __stream: *mut FILE,
) -> __ssize_t {
    return __getdelim(__lineptr, __n, '\n' as i32, __stream);
}
#[inline]
unsafe extern "C" fn atoi(mut __nptr: *const libc::c_char) -> libc::c_int {
    return strtol(
        __nptr,
        0 as *mut libc::c_void as *mut *mut libc::c_char,
        10 as libc::c_int,
    ) as libc::c_int;
}
pub unsafe extern "C" fn snoopy_datasource_rpname(
    result: *mut libc::c_char,
    arg: *const libc::c_char,
) -> libc::c_int {
    return get_rpname(getpid(), result);
}
unsafe extern "C" fn read_proc_property(
    mut pid: libc::c_int,
    mut prop_name: *const libc::c_char,
) -> *mut libc::c_char {
    let mut pid_file: [libc::c_char; 32] = [0; 32];
    let mut fp: *mut FILE = 0 as *mut FILE;
    let mut line: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut lineLen: size_t = 0 as libc::c_int as size_t;
    let mut k: *const libc::c_char = 0 as *const libc::c_char;
    let mut v: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut vLen: size_t = 0 as libc::c_int as size_t;
    let mut returnValue: [libc::c_char; 256] = *::std::mem::transmute::<
        &[u8; 256],
        &mut [libc::c_char; 256],
    >(
        b"\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0",
    );
    snprintf(
        pid_file.as_mut_ptr(),
        32 as libc::c_int as libc::c_ulong,
        b"/proc/%d/status\0" as *const u8 as *const libc::c_char,
        pid,
    );
    fp = fopen(pid_file.as_mut_ptr(), b"r\0" as *const u8 as *const libc::c_char);
    if fp.is_null() {
        return 0 as *mut libc::c_char;
    }
    while getline(&mut line, &mut lineLen, fp) != -(1 as libc::c_int) as libc::c_long {
        if 0 as libc::c_int as libc::c_ulong == lineLen
            || (strstr(line, b":\0" as *const u8 as *const libc::c_char)).is_null()
        {
            break;
        }
        k = line;
        v = strchr(line, ':' as i32);
        if v.is_null() {
            continue;
        }
        *v = '\0' as i32 as libc::c_char;
        v = v.offset(1);
        v;
        if strcmp(prop_name, k) == 0 as libc::c_int {
            v = v.offset(1);
            v;
            vLen = strlen(v);
            *v
                .offset(
                    vLen.wrapping_sub(1 as libc::c_int as libc::c_ulong) as isize,
                ) = 0 as libc::c_int as libc::c_char;
            vLen = vLen.wrapping_sub(1);
            vLen;
            if vLen > 255 as libc::c_int as libc::c_ulong {
                strncpy(
                    returnValue.as_mut_ptr(),
                    v,
                    255 as libc::c_int as libc::c_ulong,
                );
                returnValue[(255 as libc::c_int + 1 as libc::c_int - 1 as libc::c_int)
                    as usize] = 0 as libc::c_int as libc::c_char;
            } else {
                strncpy(
                    returnValue.as_mut_ptr(),
                    v,
                    (255 as libc::c_int + 1 as libc::c_int - 1 as libc::c_int)
                        as libc::c_ulong,
                );
            }
            free(line as *mut libc::c_void);
            fclose(fp);
            return strdup(returnValue.as_mut_ptr());
        }
    }
    if !line.is_null() {
        free(line as *mut libc::c_void);
    }
    fclose(fp);
    return 0 as *mut libc::c_char;
}
unsafe extern "C" fn get_parent_pid(mut pid: libc::c_int) -> libc::c_int {
    let mut ppid_str: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut ppid_int: libc::c_int = 0;
    ppid_str = read_proc_property(pid, b"PPid\0" as *const u8 as *const libc::c_char);
    if !ppid_str.is_null() {
        ppid_int = atoi(ppid_str);
        free(ppid_str as *mut libc::c_void);
        return ppid_int;
    }
    return -(1 as libc::c_int);
}
unsafe extern "C" fn get_rpname(
    mut pid: libc::c_int,
    mut result: *mut libc::c_char,
) -> libc::c_int {
    let mut parentPid: libc::c_int = 0;
    let mut name: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut nameLen: size_t = 0;
    parentPid = get_parent_pid(pid);
    if 1 as libc::c_int == parentPid || 0 as libc::c_int == parentPid {
        name = read_proc_property(pid, b"Name\0" as *const u8 as *const libc::c_char);
        if !name.is_null() {
            nameLen = snprintf(
                result,
                2048 as libc::c_int as libc::c_ulong,
                b"%s\0" as *const u8 as *const libc::c_char,
                name,
            ) as size_t;
            free(name as *mut libc::c_void);
        } else {
            nameLen = snprintf(
                result,
                2048 as libc::c_int as libc::c_ulong,
                b"%s\0" as *const u8 as *const libc::c_char,
                b"(unknown)\0" as *const u8 as *const libc::c_char,
            ) as size_t;
        }
        return nameLen as libc::c_int;
    } else if -(1 as libc::c_int) == parentPid {
        return snprintf(
            result,
            2048 as libc::c_int as libc::c_ulong,
            b"%s\0" as *const u8 as *const libc::c_char,
            b"(unknown)\0" as *const u8 as *const libc::c_char,
        )
    } else {
        return get_rpname(parentPid, result)
    };
}
