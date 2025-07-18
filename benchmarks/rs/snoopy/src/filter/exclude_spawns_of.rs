use ::libc;
extern "C" {
    pub type _IO_wide_data;
    pub type _IO_codecvt;
    pub type _IO_marker;
    fn getppid() -> __pid_t;
    fn fclose(__stream: *mut FILE) -> libc::c_int;
    fn fopen(_: *const libc::c_char, _: *const libc::c_char) -> *mut FILE;
    fn snprintf(
        _: *mut libc::c_char,
        _: libc::c_ulong,
        _: *const libc::c_char,
        _: ...
    ) -> libc::c_int;
    fn sscanf(_: *const libc::c_char, _: *const libc::c_char, _: ...) -> libc::c_int;
    fn fread(
        _: *mut libc::c_void,
        _: libc::c_ulong,
        _: libc::c_ulong,
        _: *mut FILE,
    ) -> libc::c_ulong;
    fn free(__ptr: *mut libc::c_void);
    fn calloc(_: libc::c_ulong, _: libc::c_ulong) -> *mut libc::c_void;
    fn memcpy(
        _: *mut libc::c_void,
        _: *const libc::c_void,
        _: libc::c_ulong,
    ) -> *mut libc::c_void;
    fn strcmp(_: *const libc::c_char, _: *const libc::c_char) -> libc::c_int;
    fn strdup(__s: *const libc::c_char) -> *mut libc::c_char;
    fn strchr(_: *const libc::c_char, _: libc::c_int) -> *mut libc::c_char;
    fn strrchr(_: *const libc::c_char, _: libc::c_int) -> *mut libc::c_char;
    fn strtok_r(
        __s: *mut libc::c_char,
        __delim: *const libc::c_char,
        __save_ptr: *mut *mut libc::c_char,
    ) -> *mut libc::c_char;
}
pub type pid_t = __pid_t;
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
pub unsafe extern "C" fn snoopy_filter_exclude_spawns_of(
    arg: *const libc::c_char,
) -> libc::c_int {
    let mut argDup: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut losp: *mut *mut libc::c_char = 0 as *mut *mut libc::c_char;
    let mut is_ancestor_in_list: libc::c_int = 0 as libc::c_int;
    argDup = strdup(arg);
    losp = string_to_token_array(argDup);
    if losp.is_null() {
        return 1 as libc::c_int;
    }
    is_ancestor_in_list = find_ancestor_in_list(losp);
    free(losp as *mut libc::c_void);
    free(argDup as *mut libc::c_void);
    return if is_ancestor_in_list == 1 as libc::c_int {
        0 as libc::c_int
    } else {
        1 as libc::c_int
    };
}
unsafe extern "C" fn find_ancestor_in_list(
    mut name_list: *mut *mut libc::c_char,
) -> libc::c_int {
    let mut ppid: pid_t = 0;
    let mut stat_path: [libc::c_char; 32] = [0; 32];
    let mut statf: *mut FILE = 0 as *mut FILE;
    let mut rc: libc::c_int = 0;
    let mut found: libc::c_int = 0;
    let mut left: *const libc::c_char = 0 as *const libc::c_char;
    let mut right: *const libc::c_char = 0 as *const libc::c_char;
    let mut len: size_t = 0;
    let mut st_buf: [libc::c_char; 78] = [0; 78];
    let mut st_comm_buf: [libc::c_char; 32] = [0; 32];
    let mut st_state: libc::c_char = 0;
    if name_list.is_null() {
        return -(1 as libc::c_int);
    }
    ppid = getppid();
    while ppid != 0 as libc::c_int {
        snprintf(
            stat_path.as_mut_ptr(),
            32 as libc::c_int as libc::c_ulong,
            b"/proc/%d/stat\0" as *const u8 as *const libc::c_char,
            ppid,
        );
        statf = fopen(
            stat_path.as_mut_ptr(),
            b"r\0" as *const u8 as *const libc::c_char,
        );
        if statf.is_null() {
            return -(1 as libc::c_int);
        }
        rc = fread(
            st_buf.as_mut_ptr() as *mut libc::c_void,
            1 as libc::c_int as libc::c_ulong,
            (46 as libc::c_int + 32 as libc::c_int - 1 as libc::c_int) as libc::c_ulong,
            statf,
        ) as libc::c_int;
        st_buf[rc as usize] = '\0' as i32 as libc::c_char;
        fclose(statf);
        if rc < 8 as libc::c_int {
            return -(1 as libc::c_int);
        }
        left = strchr(st_buf.as_mut_ptr(), '(' as i32);
        right = strrchr(st_buf.as_mut_ptr(), ')' as i32);
        if left.is_null() || right.is_null() {
            return -(1 as libc::c_int);
        }
        len = (right.offset_from(left) as libc::c_long
            - 1 as libc::c_int as libc::c_long) as size_t;
        if len <= 0 as libc::c_int as libc::c_ulong
            || len >= 32 as libc::c_int as libc::c_ulong
        {
            return -(1 as libc::c_int);
        }
        memcpy(
            st_comm_buf.as_mut_ptr() as *mut libc::c_void,
            left.offset(1 as libc::c_int as isize) as *const libc::c_void,
            len,
        );
        st_comm_buf[len as usize] = '\0' as i32 as libc::c_char;
        rc = sscanf(
            right.offset(1 as libc::c_int as isize),
            b" %c %d\0" as *const u8 as *const libc::c_char,
            &mut st_state as *mut libc::c_char,
            &mut ppid as *mut pid_t,
        );
        if rc != 2 as libc::c_int {
            return -(1 as libc::c_int);
        }
        found = find_string_in_array(st_comm_buf.as_mut_ptr(), name_list);
        if found != 0 {
            return 1 as libc::c_int;
        }
    }
    return 0 as libc::c_int;
}
unsafe extern "C" fn find_string_in_array(
    mut str: *const libc::c_char,
    mut str_array: *mut *mut libc::c_char,
) -> libc::c_int {
    if str.is_null() || str_array.is_null() {
        return 0 as libc::c_int;
    }
    let mut p: *mut *mut libc::c_char = str_array;
    while !(*p).is_null() {
        if strcmp(str, *p) == 0 as libc::c_int {
            return 1 as libc::c_int;
        }
        p = p.offset(1);
        p;
    }
    return 0 as libc::c_int;
}
unsafe extern "C" fn string_to_token_array(
    mut str: *mut libc::c_char,
) -> *mut *mut libc::c_char {
    let mut p: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut sepcount: libc::c_int = 0 as libc::c_int;
    let mut token_count: libc::c_int = 0;
    let mut token_array: *mut *mut libc::c_char = 0 as *mut *mut libc::c_char;
    let mut saveptr: *mut libc::c_char = 0 as *mut libc::c_char;
    if str.is_null() || *str as libc::c_int == '\0' as i32 {
        return 0 as *mut *mut libc::c_char;
    }
    p = strchr(str, ',' as i32);
    while !p.is_null() {
        sepcount += 1;
        sepcount;
        p = strchr(p.offset(1 as libc::c_int as isize), ',' as i32);
    }
    token_count = sepcount + 1 as libc::c_int;
    token_array = calloc(
        (token_count + 1 as libc::c_int) as libc::c_ulong,
        ::std::mem::size_of::<*mut libc::c_char>() as libc::c_ulong,
    ) as *mut *mut libc::c_char;
    if token_array.is_null() {
        return 0 as *mut *mut libc::c_char;
    }
    let mut delim: [libc::c_char; 2] = [
        ',' as i32 as libc::c_char,
        '\0' as i32 as libc::c_char,
    ];
    p = str;
    let mut i: libc::c_int = 0 as libc::c_int;
    while i < token_count {
        let ref mut fresh0 = *token_array.offset(i as isize);
        *fresh0 = strtok_r(p, delim.as_mut_ptr(), &mut saveptr);
        p = 0 as *mut libc::c_char;
        i += 1;
        i;
    }
    let ref mut fresh1 = *token_array.offset(token_count as isize);
    *fresh1 = 0 as *mut libc::c_char;
    return token_array;
}
