use ::libc;
extern "C" {
    pub type _IO_wide_data;
    pub type _IO_codecvt;
    pub type _IO_marker;
    pub type doscp_t;
    fn free(__ptr: *mut libc::c_void);
    static mut stderr: *mut FILE;
    fn fflush(__stream: *mut FILE) -> libc::c_int;
    fn fprintf(_: *mut FILE, _: *const libc::c_char, _: ...) -> libc::c_int;
    fn fgetc(__stream: *mut FILE) -> libc::c_int;
    fn fgets(
        __s: *mut libc::c_char,
        __n: libc::c_int,
        __stream: *mut FILE,
    ) -> *mut libc::c_char;
    fn fputs(__s: *const libc::c_char, __stream: *mut FILE) -> libc::c_int;
    fn __ctype_b_loc() -> *mut *const libc::c_ushort;
    fn __ctype_tolower_loc() -> *mut *const __int32_t;
    fn strcpy(_: *mut libc::c_char, _: *const libc::c_char) -> *mut libc::c_char;
    fn strncpy(
        _: *mut libc::c_char,
        _: *const libc::c_char,
        _: libc::c_ulong,
    ) -> *mut libc::c_char;
    fn strcmp(_: *const libc::c_char, _: *const libc::c_char) -> libc::c_int;
    fn strncmp(
        _: *const libc::c_char,
        _: *const libc::c_char,
        _: libc::c_ulong,
    ) -> libc::c_int;
    fn strdup(_: *const libc::c_char) -> *mut libc::c_char;
    fn strchr(_: *const libc::c_char, _: libc::c_int) -> *mut libc::c_char;
    fn strspn(_: *const libc::c_char, _: *const libc::c_char) -> libc::c_ulong;
    fn strlen(_: *const libc::c_char) -> libc::c_ulong;
    fn strncasecmp(
        _: *const libc::c_char,
        _: *const libc::c_char,
        _: libc::c_ulong,
    ) -> libc::c_int;
    fn __assert_fail(
        __assertion: *const libc::c_char,
        __file: *const libc::c_char,
        __line: libc::c_uint,
        __function: *const libc::c_char,
    ) -> !;
    static mut short_illegals: *const libc::c_char;
    static mut long_illegals: *const libc::c_char;
    fn unix_normalize(
        cp: *mut doscp_t,
        ans: *mut libc::c_char,
        dn: *mut dos_name_t,
        ans_size: size_t,
    ) -> *mut libc::c_char;
    fn dos_name(
        cp: *mut doscp_t,
        filename: *const libc::c_char,
        verbose: libc::c_int,
        mangled: *mut libc::c_int,
        _: *mut dos_name_t,
    );
    fn opentty(mode: libc::c_int) -> *mut FILE;
    fn dir_grow(Dir: *mut Stream_t, size: libc::c_uint) -> libc::c_int;
    static mut got_signal: libc::c_int;
    static mut mtools_raw_tty: libc::c_int;
    static mut progname: *const libc::c_char;
    fn ask_confirmation(_: *const libc::c_char, _: ...) -> libc::c_int;
    fn dir_read(entry: *mut direntry_t, error: *mut libc::c_int) -> *mut directory;
    fn setEntryToPos(entry: *mut direntry_t, pos: libc::c_uint);
    fn fatFreeWithDirentry(entry: *mut direntry_t) -> libc::c_int;
    fn write_vfat(
        _: *mut Stream_t,
        _: *mut dos_name_t,
        _: *mut libc::c_char,
        _: libc::c_uint,
        _: *mut direntry_t,
    ) -> libc::c_uint;
    fn wipeEntry(entry: *mut direntry_t);
    fn lookupForInsert(
        Dir: *mut Stream_t,
        direntry: *mut direntry_t,
        dosname: *mut dos_name_t,
        longname: *mut libc::c_char,
        ssp: *mut scan_state,
        ignore_entry: libc::c_int,
        source_entry: libc::c_int,
        pessimisticShortRename: libc::c_int,
        use_longname: libc::c_int,
    ) -> libc::c_int;
    fn autorename_short(_: *mut dos_name_t, _: libc::c_int);
    fn autorename_long(_: *mut libc::c_char, _: libc::c_int);
    fn fat_error(Dir: *mut Stream_t) -> libc::c_int;
    fn native_to_wchar(
        native: *const libc::c_char,
        wchar: *mut wchar_t,
        len: size_t,
        end: *const libc::c_char,
        mangled: *mut libc::c_int,
    ) -> size_t;
    fn isSpecial(name: *const libc::c_char) -> libc::c_int;
}
pub type __uint8_t = libc::c_uchar;
pub type __uint16_t = libc::c_ushort;
pub type __int32_t = libc::c_int;
pub type __uint32_t = libc::c_uint;
pub type __off_t = libc::c_long;
pub type __off64_t = libc::c_long;
pub type __time_t = libc::c_long;
pub type __ssize_t = libc::c_long;
pub type off_t = __off_t;
pub type ssize_t = __ssize_t;
pub type time_t = __time_t;
pub type size_t = libc::c_ulong;
pub type uint8_t = __uint8_t;
pub type uint16_t = __uint16_t;
pub type uint32_t = __uint32_t;
pub type wchar_t = libc::c_int;
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
pub type mt_off_t = off_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct dos_name_t {
    pub base: [libc::c_char; 8],
    pub ext: [libc::c_char; 3],
    pub sentinel: libc::c_char,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct Stream_t {
    pub Class: *mut Class_t,
    pub refs: libc::c_int,
    pub Next: *mut Stream_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct Class_t {
    pub read: Option::<
        unsafe extern "C" fn(*mut Stream_t, *mut libc::c_char, size_t) -> ssize_t,
    >,
    pub write: Option::<
        unsafe extern "C" fn(*mut Stream_t, *mut libc::c_char, size_t) -> ssize_t,
    >,
    pub pread: Option::<
        unsafe extern "C" fn(
            *mut Stream_t,
            *mut libc::c_char,
            mt_off_t,
            size_t,
        ) -> ssize_t,
    >,
    pub pwrite: Option::<
        unsafe extern "C" fn(
            *mut Stream_t,
            *mut libc::c_char,
            mt_off_t,
            size_t,
        ) -> ssize_t,
    >,
    pub flush: Option::<unsafe extern "C" fn(*mut Stream_t) -> libc::c_int>,
    pub freeFunc: Option::<unsafe extern "C" fn(*mut Stream_t) -> libc::c_int>,
    pub set_geom: Option::<
        unsafe extern "C" fn(*mut Stream_t, *mut device_t, *mut device_t) -> libc::c_int,
    >,
    pub get_data: Option::<
        unsafe extern "C" fn(
            *mut Stream_t,
            *mut time_t,
            *mut mt_off_t,
            *mut libc::c_int,
            *mut uint32_t,
        ) -> libc::c_int,
    >,
    pub pre_allocate: Option::<
        unsafe extern "C" fn(*mut Stream_t, mt_off_t) -> libc::c_int,
    >,
    pub get_dosConvert: Option::<unsafe extern "C" fn(*mut Stream_t) -> *mut doscp_t>,
    pub discard: Option::<unsafe extern "C" fn(*mut Stream_t) -> libc::c_int>,
}
pub type device_t = device;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct device {
    pub name: *const libc::c_char,
    pub drive: libc::c_char,
    pub fat_bits: libc::c_int,
    pub mode: libc::c_int,
    pub tracks: libc::c_uint,
    pub heads: uint16_t,
    pub sectors: uint16_t,
    pub hidden: libc::c_uint,
    pub offset: off_t,
    pub partition: libc::c_uint,
    pub misc_flags: libc::c_uint,
    pub ssize: uint8_t,
    pub use_2m: libc::c_uint,
    pub precmd: *mut libc::c_char,
    pub file_nr: libc::c_int,
    pub blocksize: libc::c_uint,
    pub codepage: libc::c_uint,
    pub data_map: *const libc::c_char,
    pub tot_sectors: uint32_t,
    pub sector_size: uint16_t,
    pub postcmd: *mut libc::c_char,
    pub cfg_filename: *const libc::c_char,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct directory {
    pub name: [libc::c_char; 8],
    pub ext: [libc::c_char; 3],
    pub attr: libc::c_uchar,
    pub Case: libc::c_uchar,
    pub ctime_ms: libc::c_uchar,
    pub ctime: [libc::c_uchar; 2],
    pub cdate: [libc::c_uchar; 2],
    pub adate: [libc::c_uchar; 2],
    pub startHi: [libc::c_uchar; 2],
    pub time: [libc::c_uchar; 2],
    pub date: [libc::c_uchar; 2],
    pub start: [libc::c_uchar; 2],
    pub size: [libc::c_uchar; 4],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct scan_state {
    pub match_free: libc::c_int,
    pub shortmatch: libc::c_int,
    pub longmatch: libc::c_int,
    pub free_start: libc::c_uint,
    pub free_end: libc::c_uint,
    pub slot: libc::c_uint,
    pub got_slots: libc::c_int,
    pub size_needed: libc::c_uint,
    pub max_entry: libc::c_uint,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct direntry_t {
    pub Dir: *mut Stream_t,
    pub entry: libc::c_int,
    pub dir: directory,
    pub name: [wchar_t; 256],
    pub beginSlot: libc::c_uint,
    pub endSlot: libc::c_uint,
}
pub type clash_action = libc::c_uint;
pub const NAMEMATCH_GREW: clash_action = 9;
pub const NAMEMATCH_SUCCESS: clash_action = 8;
pub const NAMEMATCH_ERROR: clash_action = 7;
pub const NAMEMATCH_OVERWRITE: clash_action = 6;
pub const NAMEMATCH_PRENAME: clash_action = 5;
pub const NAMEMATCH_RENAME: clash_action = 4;
pub const NAMEMATCH_SKIP: clash_action = 3;
pub const NAMEMATCH_QUIT: clash_action = 2;
pub const NAMEMATCH_AUTORENAME: clash_action = 1;
pub const NAMEMATCH_NONE: clash_action = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ClashHandling_t {
    pub action: [clash_action; 2],
    pub namematch_default: [clash_action; 2],
    pub nowarn: libc::c_int,
    pub got_slots: libc::c_int,
    pub mod_time: libc::c_int,
    pub myname: *mut libc::c_char,
    pub dosname: *mut libc::c_uchar,
    pub single: libc::c_int,
    pub use_longname: libc::c_int,
    pub ignore_entry: libc::c_int,
    pub source: libc::c_int,
    pub source_entry: libc::c_int,
    pub name_converter: Option::<
        unsafe extern "C" fn(
            *mut doscp_t,
            *const libc::c_char,
            libc::c_int,
            *mut libc::c_int,
            *mut dos_name_t,
        ) -> (),
    >,
    pub is_label: libc::c_int,
}
pub type write_data_callback = unsafe extern "C" fn(
    *mut dos_name_t,
    *mut libc::c_char,
    *mut libc::c_void,
    *mut direntry_t,
) -> libc::c_int;
#[inline]
unsafe extern "C" fn tolower(mut __c: libc::c_int) -> libc::c_int {
    return if __c >= -(128 as libc::c_int) && __c < 256 as libc::c_int {
        *(*__ctype_tolower_loc()).offset(__c as isize)
    } else {
        __c
    };
}
#[inline]
unsafe extern "C" fn convert_to_shortname(
    mut cp: *mut doscp_t,
    mut ch: *mut ClashHandling_t,
    mut un: *const libc::c_char,
    mut dn: *mut dos_name_t,
) -> libc::c_int {
    let mut mangled: libc::c_int = 0;
    ((*ch).name_converter).unwrap()(cp, un, 0 as libc::c_int, &mut mangled, dn);
    (*dn).sentinel = '\0' as i32 as libc::c_char;
    if (*dn).base[0 as libc::c_int as usize] as libc::c_int == -27i32 {
        (*dn).base[0 as libc::c_int as usize] = '\u{5}' as i32 as libc::c_char;
    }
    return mangled;
}
#[inline]
unsafe extern "C" fn chomp(mut line: *mut libc::c_char) {
    let mut l: size_t = strlen(line);
    while l > 0 as libc::c_int as libc::c_ulong
        && (*line.offset(l.wrapping_sub(1 as libc::c_int as libc::c_ulong) as isize)
            as libc::c_int == '\n' as i32
            || *line.offset(l.wrapping_sub(1 as libc::c_int as libc::c_ulong) as isize)
                as libc::c_int == '\r' as i32)
    {
        l = l.wrapping_sub(1);
        *line.offset(l as isize) = '\0' as i32 as libc::c_char;
    }
}
#[inline]
unsafe extern "C" fn ask_rename(
    mut cp: *mut doscp_t,
    mut ch: *mut ClashHandling_t,
    mut shortname: *mut dos_name_t,
    mut longname: *mut libc::c_char,
    mut isprimary: libc::c_int,
) -> libc::c_int {
    let mut mangled: libc::c_int = 0;
    if (opentty(0 as libc::c_int)).is_null() {
        return 0 as libc::c_int;
    }
    mangled = 0 as libc::c_int;
    loop {
        let mut tname: [libc::c_char; 1021] = [0; 1021];
        fprintf(
            stderr,
            b"New %s name for \"%s\": \0" as *const u8 as *const libc::c_char,
            if isprimary != 0 {
                b"primary\0" as *const u8 as *const libc::c_char
            } else {
                b"secondary\0" as *const u8 as *const libc::c_char
            },
            longname,
        );
        fflush(stderr);
        if (fgets(
            tname.as_mut_ptr(),
            4 as libc::c_int * 255 as libc::c_int + 1 as libc::c_int,
            opentty(0 as libc::c_int),
        ))
            .is_null()
        {
            return 0 as libc::c_int;
        }
        chomp(tname.as_mut_ptr());
        if isprimary != 0 {
            strcpy(longname, tname.as_mut_ptr());
        } else {
            mangled = convert_to_shortname(cp, ch, tname.as_mut_ptr(), shortname);
        }
        if !(mangled & 1 as libc::c_int != 0) {
            break;
        }
    }
    return 1 as libc::c_int;
}
#[inline]
unsafe extern "C" fn ask_namematch(
    mut cp: *mut doscp_t,
    mut dosname: *mut dos_name_t,
    mut longname: *mut libc::c_char,
    mut isprimary: libc::c_int,
    mut ch: *mut ClashHandling_t,
    mut no_overwrite: libc::c_int,
    mut reason: libc::c_int,
) -> clash_action {
    let mut ans: [libc::c_char; 10] = [0; 10];
    let mut a: clash_action = NAMEMATCH_NONE;
    let mut perm: libc::c_int = 0;
    let mut name_buffer: [libc::c_char; 52] = [0; 52];
    let mut name: *mut libc::c_char = 0 as *mut libc::c_char;
    static mut reasons: [*const libc::c_char; 3] = [
        b"already exists\0" as *const u8 as *const libc::c_char,
        b"is reserved\0" as *const u8 as *const libc::c_char,
        b"contains illegal character(s)\0" as *const u8 as *const libc::c_char,
    ];
    a = (*ch).action[isprimary as usize];
    if a as libc::c_uint == NAMEMATCH_NONE as libc::c_int as libc::c_uint
        && (opentty(1 as libc::c_int)).is_null()
    {
        return NAMEMATCH_SKIP;
    }
    if isprimary == 0 {
        name = unix_normalize(
            cp,
            name_buffer.as_mut_ptr(),
            dosname,
            ::std::mem::size_of::<dos_name_t>() as libc::c_ulong,
        );
    } else {
        name = longname;
    }
    perm = 0 as libc::c_int;
    while a as libc::c_uint == NAMEMATCH_NONE as libc::c_int as libc::c_uint {
        fprintf(
            stderr,
            b"%s file name \"%s\" %s.\n\0" as *const u8 as *const libc::c_char,
            if isprimary != 0 {
                b"Long\0" as *const u8 as *const libc::c_char
            } else {
                b"Short\0" as *const u8 as *const libc::c_char
            },
            name,
            reasons[reason as usize],
        );
        fprintf(
            stderr,
            b"a)utorename A)utorename-all r)ename R)ename-all \0" as *const u8
                as *const libc::c_char,
        );
        if no_overwrite == 0 {
            fprintf(
                stderr,
                b"o)verwrite O)verwrite-all\0" as *const u8 as *const libc::c_char,
            );
        }
        fprintf(
            stderr,
            b"\ns)kip S)kip-all q)uit (aArR\0" as *const u8 as *const libc::c_char,
        );
        if no_overwrite == 0 {
            fprintf(stderr, b"oO\0" as *const u8 as *const libc::c_char);
        }
        fprintf(stderr, b"sSq): \0" as *const u8 as *const libc::c_char);
        fflush(stderr);
        fflush(opentty(1 as libc::c_int));
        if mtools_raw_tty != 0 {
            let mut rep: libc::c_int = 0;
            rep = fgetc(opentty(1 as libc::c_int));
            fputs(b"\n\0" as *const u8 as *const libc::c_char, stderr);
            if rep == -(1 as libc::c_int) {
                ans[0 as libc::c_int as usize] = 'q' as i32 as libc::c_char;
            } else {
                ans[0 as libc::c_int as usize] = rep as libc::c_char;
            }
        } else if (fgets(ans.as_mut_ptr(), 9 as libc::c_int, opentty(0 as libc::c_int)))
            .is_null()
        {
            ans[0 as libc::c_int as usize] = 'q' as i32 as libc::c_char;
        }
        perm = *(*__ctype_b_loc())
            .offset(
                ans[0 as libc::c_int as usize] as libc::c_uchar as libc::c_int as isize,
            ) as libc::c_int & _ISupper as libc::c_int as libc::c_ushort as libc::c_int;
        match ({
            let mut __res: libc::c_int = 0;
            if ::std::mem::size_of::<libc::c_uchar>() as libc::c_ulong
                > 1 as libc::c_int as libc::c_ulong
            {
                if 0 != 0 {
                    let mut __c: libc::c_int = ans[0 as libc::c_int as usize]
                        as libc::c_uchar as libc::c_int;
                    __res = if __c < -(128 as libc::c_int) || __c > 255 as libc::c_int {
                        __c
                    } else {
                        *(*__ctype_tolower_loc()).offset(__c as isize)
                    };
                } else {
                    __res = tolower(
                        ans[0 as libc::c_int as usize] as libc::c_uchar as libc::c_int,
                    );
                }
            } else {
                __res = *(*__ctype_tolower_loc())
                    .offset(
                        ans[0 as libc::c_int as usize] as libc::c_uchar as libc::c_int
                            as isize,
                    );
            }
            __res
        }) {
            97 => {
                a = NAMEMATCH_AUTORENAME;
            }
            114 => {
                if isprimary != 0 {
                    a = NAMEMATCH_PRENAME;
                } else {
                    a = NAMEMATCH_RENAME;
                }
            }
            111 => {
                if no_overwrite != 0 {
                    continue;
                }
                a = NAMEMATCH_OVERWRITE;
            }
            115 => {
                a = NAMEMATCH_SKIP;
            }
            113 => {
                perm = 0 as libc::c_int;
                a = NAMEMATCH_QUIT;
            }
            _ => {
                perm = 0 as libc::c_int;
            }
        }
    }
    (*ch).action[isprimary as usize] = a;
    if perm != 0 {
        (*ch).namematch_default[isprimary as usize] = a;
    }
    if a as libc::c_uint == NAMEMATCH_OVERWRITE as libc::c_int as libc::c_uint {
        (*ch).action[isprimary as usize] = NAMEMATCH_NONE;
    }
    return a;
}
#[inline]
unsafe extern "C" fn process_namematch(
    mut cp: *mut doscp_t,
    mut dosname: *mut dos_name_t,
    mut longname: *mut libc::c_char,
    mut isprimary: libc::c_int,
    mut ch: *mut ClashHandling_t,
    mut no_overwrite: libc::c_int,
    mut reason: libc::c_int,
) -> clash_action {
    let mut action: clash_action = NAMEMATCH_NONE;
    action = ask_namematch(cp, dosname, longname, isprimary, ch, no_overwrite, reason);
    match action as libc::c_uint {
        2 => {
            got_signal = 1 as libc::c_int;
            return NAMEMATCH_SKIP;
        }
        3 => return NAMEMATCH_SKIP,
        4 | 5 => {
            ask_rename(cp, ch, dosname, longname, isprimary);
            return action;
        }
        1 => {
            if isprimary != 0 {
                autorename_long(longname, 1 as libc::c_int);
                return NAMEMATCH_PRENAME;
            } else {
                autorename_short(dosname, 1 as libc::c_int);
                return NAMEMATCH_RENAME;
            }
        }
        6 => {
            if no_overwrite != 0 {
                return NAMEMATCH_SKIP
            } else {
                return NAMEMATCH_OVERWRITE
            }
        }
        0 | 7 | 8 | 9 => return NAMEMATCH_NONE,
        _ => {}
    }
    return action;
}
unsafe extern "C" fn contains_illegals(
    mut string: *const libc::c_char,
    mut illegals: *const libc::c_char,
    mut len: libc::c_int,
) -> libc::c_int {
    while *string as libc::c_int != 0
        && {
            let fresh0 = len;
            len = len - 1;
            fresh0 != 0
        }
    {
        if (*string as libc::c_int) < ' ' as i32
            && *string as libc::c_int != '\u{5}' as i32
            && *string as libc::c_int & 0x80 as libc::c_int == 0
            || !(strchr(illegals, *string as libc::c_int)).is_null()
        {
            return 1 as libc::c_int;
        }
        string = string.offset(1);
        string;
    }
    return 0 as libc::c_int;
}
unsafe extern "C" fn is_reserved(
    mut ans: *mut libc::c_char,
    mut islong: libc::c_int,
) -> libc::c_int {
    let mut i: libc::c_uint = 0;
    static mut dev3: [*const libc::c_char; 5] = [
        b"CON\0" as *const u8 as *const libc::c_char,
        b"AUX\0" as *const u8 as *const libc::c_char,
        b"PRN\0" as *const u8 as *const libc::c_char,
        b"NUL\0" as *const u8 as *const libc::c_char,
        b"   \0" as *const u8 as *const libc::c_char,
    ];
    static mut dev4: [*const libc::c_char; 2] = [
        b"COM\0" as *const u8 as *const libc::c_char,
        b"LPT\0" as *const u8 as *const libc::c_char,
    ];
    i = 0 as libc::c_int as libc::c_uint;
    while (i as libc::c_ulong)
        < (::std::mem::size_of::<[*const libc::c_char; 5]>() as libc::c_ulong)
            .wrapping_div(::std::mem::size_of::<*const libc::c_char>() as libc::c_ulong)
    {
        if strncasecmp(ans, dev3[i as usize], 3 as libc::c_int as libc::c_ulong) == 0
            && (islong != 0 && *ans.offset(3 as libc::c_int as isize) == 0
                || islong == 0
                    && strncmp(
                        ans.offset(3 as libc::c_int as isize),
                        b"     \0" as *const u8 as *const libc::c_char,
                        5 as libc::c_int as libc::c_ulong,
                    ) == 0)
        {
            return 1 as libc::c_int;
        }
        i = i.wrapping_add(1);
        i;
    }
    i = 0 as libc::c_int as libc::c_uint;
    while (i as libc::c_ulong)
        < (::std::mem::size_of::<[*const libc::c_char; 2]>() as libc::c_ulong)
            .wrapping_div(::std::mem::size_of::<*const libc::c_char>() as libc::c_ulong)
    {
        if strncasecmp(ans, dev4[i as usize], 3 as libc::c_int as libc::c_ulong) == 0
            && (*ans.offset(3 as libc::c_int as isize) as libc::c_int >= '1' as i32
                && *ans.offset(3 as libc::c_int as isize) as libc::c_int <= '4' as i32)
            && (islong != 0 && *ans.offset(4 as libc::c_int as isize) == 0
                || islong == 0
                    && strncmp(
                        ans.offset(4 as libc::c_int as isize),
                        b"    \0" as *const u8 as *const libc::c_char,
                        4 as libc::c_int as libc::c_ulong,
                    ) == 0)
        {
            return 1 as libc::c_int;
        }
        i = i.wrapping_add(1);
        i;
    }
    return 0 as libc::c_int;
}
#[inline]
unsafe extern "C" fn get_slots(
    mut Dir: *mut Stream_t,
    mut dosname: *mut dos_name_t,
    mut longname: *mut libc::c_char,
    mut ssp: *mut scan_state,
    mut ch: *mut ClashHandling_t,
) -> clash_action {
    let mut error: libc::c_int = 0;
    let mut ret: clash_action = NAMEMATCH_NONE;
    let mut match_pos: libc::c_int = 0 as libc::c_int;
    let mut entry: direntry_t = direntry_t {
        Dir: 0 as *mut Stream_t,
        entry: 0,
        dir: directory {
            name: [0; 8],
            ext: [0; 3],
            attr: 0,
            Case: 0,
            ctime_ms: 0,
            ctime: [0; 2],
            cdate: [0; 2],
            adate: [0; 2],
            startHi: [0; 2],
            time: [0; 2],
            date: [0; 2],
            start: [0; 2],
            size: [0; 4],
        },
        name: [0; 256],
        beginSlot: 0,
        endSlot: 0,
    };
    let mut isprimary: libc::c_int = 0;
    let mut no_overwrite: libc::c_int = 0;
    let mut reason: libc::c_int = 0;
    let mut pessimisticShortRename: libc::c_int = 0;
    let mut cp: *mut doscp_t = ((*(*Dir).Class).get_dosConvert).unwrap()(Dir);
    pessimisticShortRename = ((*ch).action[0 as libc::c_int as usize] as libc::c_uint
        == NAMEMATCH_AUTORENAME as libc::c_int as libc::c_uint) as libc::c_int;
    entry.Dir = Dir;
    no_overwrite = 1 as libc::c_int;
    if is_reserved(longname, 1 as libc::c_int) != 0
        || *longname
            .offset(
                strspn(longname, b". \0" as *const u8 as *const libc::c_char) as isize,
            ) as libc::c_int == '\0' as i32
    {
        reason = 1 as libc::c_int;
        isprimary = 1 as libc::c_int;
    } else if contains_illegals(longname, long_illegals, 1024 as libc::c_int) != 0 {
        reason = 2 as libc::c_int;
        isprimary = 1 as libc::c_int;
    } else if is_reserved(((*dosname).base).as_mut_ptr(), 0 as libc::c_int) != 0 {
        reason = 1 as libc::c_int;
        (*ch).use_longname = 1 as libc::c_int;
        isprimary = 0 as libc::c_int;
    } else if (*ch).is_label == 0
        && contains_illegals(
            ((*dosname).base).as_mut_ptr(),
            short_illegals,
            11 as libc::c_int,
        ) != 0
    {
        reason = 2 as libc::c_int;
        (*ch).use_longname = 1 as libc::c_int;
        isprimary = 0 as libc::c_int;
    } else {
        reason = 0 as libc::c_int;
        match lookupForInsert(
            Dir,
            &mut entry,
            dosname,
            longname,
            ssp,
            (*ch).ignore_entry,
            (*ch).source_entry,
            (pessimisticShortRename != 0 && (*ch).use_longname != 0) as libc::c_int,
            (*ch).use_longname,
        ) {
            -1 => return NAMEMATCH_ERROR,
            0 => return NAMEMATCH_SKIP,
            5 => return NAMEMATCH_GREW,
            6 => return NAMEMATCH_SUCCESS,
            _ => {}
        }
        if (*ssp).longmatch >= 0 as libc::c_int {
            match_pos = (*ssp).longmatch;
            isprimary = 1 as libc::c_int;
        } else if (*ch).use_longname & 1 as libc::c_int != 0
            && (*ssp).shortmatch != -(1 as libc::c_int)
        {
            match_pos = (*ssp).shortmatch;
            isprimary = 0 as libc::c_int;
        } else if (*ssp).shortmatch >= 0 as libc::c_int {
            match_pos = (*ssp).shortmatch;
            isprimary = 1 as libc::c_int;
        } else {
            return NAMEMATCH_RENAME
        }
        if match_pos > -(1 as libc::c_int) {
            entry.entry = match_pos;
            dir_read(&mut entry, &mut error);
            if error != 0 {
                return NAMEMATCH_ERROR;
            }
            no_overwrite = (match_pos == (*ch).source
                || entry.dir.attr as libc::c_int & 0x10 as libc::c_int != 0)
                as libc::c_int;
        }
    }
    ret = process_namematch(cp, dosname, longname, isprimary, ch, no_overwrite, reason);
    if ret as libc::c_uint == NAMEMATCH_OVERWRITE as libc::c_int as libc::c_uint
        && match_pos > -(1 as libc::c_int)
    {
        if entry.dir.attr as libc::c_int & 0x5 as libc::c_int != 0
            && ask_confirmation(
                b"file is read only, overwrite anyway (y/n) ? \0" as *const u8
                    as *const libc::c_char,
            ) != 0
        {
            return NAMEMATCH_RENAME;
        }
        if fatFreeWithDirentry(&mut entry) != 0 {
            return NAMEMATCH_ERROR;
        }
        wipeEntry(&mut entry);
        return NAMEMATCH_RENAME;
    }
    return ret;
}
#[inline]
unsafe extern "C" fn write_slots(
    mut Dir: *mut Stream_t,
    mut dosname: *mut dos_name_t,
    mut longname: *mut libc::c_char,
    mut ssp: *mut scan_state,
    mut cb: Option::<write_data_callback>,
    mut arg: *mut libc::c_void,
    mut Case: libc::c_int,
) -> libc::c_int {
    let mut entry: direntry_t = direntry_t {
        Dir: 0 as *mut Stream_t,
        entry: 0,
        dir: directory {
            name: [0; 8],
            ext: [0; 3],
            attr: 0,
            Case: 0,
            ctime_ms: 0,
            ctime: [0; 2],
            cdate: [0; 2],
            adate: [0; 2],
            startHi: [0; 2],
            time: [0; 2],
            date: [0; 2],
            start: [0; 2],
            size: [0; 4],
        },
        name: [0; 256],
        beginSlot: 0,
        endSlot: 0,
    };
    if fat_error(Dir) != 0 {
        return 0 as libc::c_int;
    }
    entry.Dir = Dir;
    if (*ssp).got_slots != 0 {} else {
        __assert_fail(
            b"ssp->got_slots\0" as *const u8 as *const libc::c_char,
            b"mk_direntry.c\0" as *const u8 as *const libc::c_char,
            499 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 107],
                &[libc::c_char; 107],
            >(
                b"int write_slots(Stream_t *, dos_name_t *, char *, struct scan_state *, write_data_callback *, void *, int)\0",
            ))
                .as_ptr(),
        );
    }
    'c_7782: {
        if (*ssp).got_slots != 0 {} else {
            __assert_fail(
                b"ssp->got_slots\0" as *const u8 as *const libc::c_char,
                b"mk_direntry.c\0" as *const u8 as *const libc::c_char,
                499 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 107],
                    &[libc::c_char; 107],
                >(
                    b"int write_slots(Stream_t *, dos_name_t *, char *, struct scan_state *, write_data_callback *, void *, int)\0",
                ))
                    .as_ptr(),
            );
        }
    };
    setEntryToPos(&mut entry, (*ssp).slot);
    native_to_wchar(
        longname,
        (entry.name).as_mut_ptr(),
        255 as libc::c_int as size_t,
        0 as *const libc::c_char,
        0 as *mut libc::c_int,
    );
    entry.name[255 as libc::c_int as usize] = '\0' as i32;
    entry
        .dir
        .Case = (Case & (0x10 as libc::c_int | 0x8 as libc::c_int)) as libc::c_uchar;
    if cb.unwrap()(dosname, longname, arg, &mut entry) >= 0 as libc::c_int {
        if (*ssp).size_needed > 1 as libc::c_int as libc::c_uint
            && ((*ssp).free_end).wrapping_sub((*ssp).free_start) >= (*ssp).size_needed
        {
            (*ssp)
                .slot = write_vfat(
                Dir,
                dosname,
                longname,
                (*ssp).free_start,
                &mut entry,
            );
        } else {
            (*ssp).size_needed = 1 as libc::c_int as libc::c_uint;
            write_vfat(
                Dir,
                dosname,
                0 as *mut libc::c_char,
                (*ssp).free_start,
                &mut entry,
            );
        }
    } else {
        return 0 as libc::c_int
    }
    return 1 as libc::c_int;
}
unsafe extern "C" fn stripspaces(mut name: *mut libc::c_char) {
    let mut p: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut non_space: *mut libc::c_char = 0 as *mut libc::c_char;
    non_space = name;
    p = name;
    while *p != 0 {
        if *p as libc::c_int != ' ' as i32 {
            non_space = p;
        }
        p = p.offset(1);
        p;
    }
    if *name.offset(0 as libc::c_int as isize) != 0 {
        *non_space.offset(1 as libc::c_int as isize) = '\0' as i32 as libc::c_char;
    }
}
unsafe extern "C" fn _mwrite_one(
    mut Dir: *mut Stream_t,
    mut argname: *mut libc::c_char,
    mut shortname: *mut libc::c_char,
    mut cb: Option::<write_data_callback>,
    mut arg: *mut libc::c_void,
    mut ch: *mut ClashHandling_t,
) -> libc::c_int {
    let mut longname: [libc::c_char; 261] = [0; 261];
    let mut dstname: *const libc::c_char = 0 as *const libc::c_char;
    let mut dosname: dos_name_t = dos_name_t {
        base: [0; 8],
        ext: [0; 3],
        sentinel: 0,
    };
    let mut expanded: libc::c_int = 0;
    let mut scan: scan_state = scan_state {
        match_free: 0,
        shortmatch: 0,
        longmatch: 0,
        free_start: 0,
        free_end: 0,
        slot: 0,
        got_slots: 0,
        size_needed: 0,
        max_entry: 0,
    };
    let mut ret: clash_action = NAMEMATCH_NONE;
    let mut cp: *mut doscp_t = ((*(*Dir).Class).get_dosConvert).unwrap()(Dir);
    expanded = 0 as libc::c_int;
    if isSpecial(argname) != 0 {
        fprintf(
            stderr,
            b"Cannot create entry named . or ..\n\0" as *const u8 as *const libc::c_char,
        );
        return -(1 as libc::c_int);
    }
    if (*ch).name_converter
        == Some(
            dos_name
                as unsafe extern "C" fn(
                    *mut doscp_t,
                    *const libc::c_char,
                    libc::c_int,
                    *mut libc::c_int,
                    *mut dos_name_t,
                ) -> (),
        )
    {
        if !shortname.is_null() {
            stripspaces(shortname);
        }
        if !argname.is_null() {
            stripspaces(argname);
        }
    }
    if !shortname.is_null() {
        convert_to_shortname(cp, ch, shortname, &mut dosname);
        if (*ch).use_longname & 1 as libc::c_int != 0 {
            argname = shortname;
            shortname = 0 as *mut libc::c_char;
        }
    }
    if *argname.offset(0 as libc::c_int as isize) as libc::c_int != 0
        && *argname.offset(1 as libc::c_int as isize) as libc::c_int == ':' as i32
    {
        dstname = argname.offset(2 as libc::c_int as isize);
    } else {
        dstname = argname;
    }
    strncpy(
        longname.as_mut_ptr(),
        dstname,
        (20 as libc::c_int * 13 as libc::c_int + 1 as libc::c_int - 1 as libc::c_int)
            as libc::c_ulong,
    );
    if !shortname.is_null() {
        (*ch).use_longname = convert_to_shortname(cp, ch, shortname, &mut dosname);
        if strcmp(shortname, longname.as_mut_ptr()) != 0 {
            (*ch).use_longname |= 1 as libc::c_int;
        }
    } else {
        (*ch)
            .use_longname = convert_to_shortname(
            cp,
            ch,
            longname.as_mut_ptr(),
            &mut dosname,
        );
    }
    (*ch)
        .action[0 as libc::c_int
        as usize] = (*ch).namematch_default[0 as libc::c_int as usize];
    (*ch)
        .action[1 as libc::c_int
        as usize] = (*ch).namematch_default[1 as libc::c_int as usize];
    loop {
        ret = get_slots(Dir, &mut dosname, longname.as_mut_ptr(), &mut scan, ch);
        match ret as libc::c_uint {
            7 => return -(1 as libc::c_int),
            3 => return -(1 as libc::c_int),
            5 => {
                (*ch)
                    .use_longname = convert_to_shortname(
                    cp,
                    ch,
                    longname.as_mut_ptr(),
                    &mut dosname,
                );
            }
            9 => {
                if expanded != 0 {
                    fprintf(
                        stderr,
                        b"%s: No directory slots\n\0" as *const u8
                            as *const libc::c_char,
                        progname,
                    );
                    return -(1 as libc::c_int);
                }
                expanded = 1 as libc::c_int;
                if dir_grow(Dir, scan.max_entry) != 0 {
                    return -(1 as libc::c_int);
                }
            }
            6 | 8 => {
                return write_slots(
                    Dir,
                    &mut dosname,
                    longname.as_mut_ptr(),
                    &mut scan,
                    cb,
                    arg,
                    (*ch).use_longname,
                );
            }
            0 | 1 | 2 => {
                fprintf(
                    stderr,
                    b"Internal error: clash_action=%d\n\0" as *const u8
                        as *const libc::c_char,
                    ret as libc::c_uint,
                );
                return -(1 as libc::c_int);
            }
            4 | _ => {}
        }
    };
}
pub unsafe extern "C" fn mwrite_one(
    mut Dir: *mut Stream_t,
    mut _argname: *const libc::c_char,
    mut _shortname: *const libc::c_char,
    mut cb: Option::<write_data_callback>,
    mut arg: *mut libc::c_void,
    mut ch: *mut ClashHandling_t,
) -> libc::c_int {
    let mut argname: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut shortname: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut ret: libc::c_int = 0;
    if !_argname.is_null() {
        argname = strdup(_argname);
    } else {
        argname = 0 as *mut libc::c_char;
    }
    if !_shortname.is_null() {
        shortname = strdup(_shortname);
    } else {
        shortname = 0 as *mut libc::c_char;
    }
    ret = _mwrite_one(Dir, argname, shortname, cb, arg, ch);
    if !argname.is_null() {
        free(argname as *mut libc::c_void);
    }
    if !shortname.is_null() {
        free(shortname as *mut libc::c_void);
    }
    return ret;
}
pub unsafe extern "C" fn init_clash_handling(mut ch: *mut ClashHandling_t) {
    (*ch).ignore_entry = -(1 as libc::c_int);
    (*ch).source_entry = -(2 as libc::c_int);
    (*ch).nowarn = 0 as libc::c_int;
    (*ch).namematch_default[0 as libc::c_int as usize] = NAMEMATCH_AUTORENAME;
    (*ch).namematch_default[1 as libc::c_int as usize] = NAMEMATCH_NONE;
    (*ch)
        .name_converter = Some(
        dos_name
            as unsafe extern "C" fn(
                *mut doscp_t,
                *const libc::c_char,
                libc::c_int,
                *mut libc::c_int,
                *mut dos_name_t,
            ) -> (),
    );
    (*ch).source = -(2 as libc::c_int);
    (*ch).is_label = 0 as libc::c_int;
}
pub unsafe extern "C" fn handle_clash_options(
    mut ch: *mut ClashHandling_t,
    mut c: libc::c_int,
) -> libc::c_int {
    let mut isprimary: libc::c_int = 0;
    if *(*__ctype_b_loc()).offset(c as isize) as libc::c_int
        & _ISupper as libc::c_int as libc::c_ushort as libc::c_int != 0
    {
        isprimary = 0 as libc::c_int;
    } else {
        isprimary = 1 as libc::c_int;
    }
    c = ({
        let mut __res: libc::c_int = 0;
        if ::std::mem::size_of::<libc::c_int>() as libc::c_ulong
            > 1 as libc::c_int as libc::c_ulong
        {
            if 0 != 0 {
                let mut __c: libc::c_int = c;
                __res = if __c < -(128 as libc::c_int) || __c > 255 as libc::c_int {
                    __c
                } else {
                    *(*__ctype_tolower_loc()).offset(__c as isize)
                };
            } else {
                __res = tolower(c);
            }
        } else {
            __res = *(*__ctype_tolower_loc()).offset(c as isize);
        }
        __res
    });
    match c {
        111 => {
            (*ch).namematch_default[isprimary as usize] = NAMEMATCH_OVERWRITE;
            return 0 as libc::c_int;
        }
        114 => {
            (*ch).namematch_default[isprimary as usize] = NAMEMATCH_RENAME;
            return 0 as libc::c_int;
        }
        115 => {
            (*ch).namematch_default[isprimary as usize] = NAMEMATCH_SKIP;
            return 0 as libc::c_int;
        }
        109 => {
            (*ch).namematch_default[isprimary as usize] = NAMEMATCH_NONE;
            return 0 as libc::c_int;
        }
        97 => {
            (*ch).namematch_default[isprimary as usize] = NAMEMATCH_AUTORENAME;
            return 0 as libc::c_int;
        }
        _ => return -(1 as libc::c_int),
    };
}
pub unsafe extern "C" fn dosnameToDirentry(
    mut dn: *const dos_name_t,
    mut dir: *mut directory,
) {
    strncpy(
        ((*dir).name).as_mut_ptr(),
        ((*dn).base).as_ptr(),
        8 as libc::c_int as libc::c_ulong,
    );
    strncpy(
        ((*dir).ext).as_mut_ptr(),
        ((*dn).ext).as_ptr(),
        3 as libc::c_int as libc::c_ulong,
    );
}
