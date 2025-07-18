use ::libc;
extern "C" {
    pub type _IO_wide_data;
    pub type _IO_codecvt;
    pub type _IO_marker;
    pub type doscp_t;
    fn random() -> libc::c_long;
    fn srandom(__seed: libc::c_uint);
    fn exit(_: libc::c_int) -> !;
    static mut stdin: *mut FILE;
    static mut stderr: *mut FILE;
    fn fprintf(_: *mut FILE, _: *const libc::c_char, _: ...) -> libc::c_int;
    fn printf(_: *const libc::c_char, _: ...) -> libc::c_int;
    fn fgets(
        __s: *mut libc::c_char,
        __n: libc::c_int,
        __stream: *mut FILE,
    ) -> *mut libc::c_char;
    fn __ctype_toupper_loc() -> *mut *const __int32_t;
    static mut optarg: *mut libc::c_char;
    static mut optind: libc::c_int;
    fn getopt(
        ___argc: libc::c_int,
        ___argv: *const *mut libc::c_char,
        __shortopts: *const libc::c_char,
    ) -> libc::c_int;
    fn time(__timer: *mut time_t) -> time_t;
    fn __errno_location() -> *mut libc::c_int;
    fn memset(
        _: *mut libc::c_void,
        _: libc::c_int,
        _: libc::c_ulong,
    ) -> *mut libc::c_void;
    fn strncpy(
        _: *mut libc::c_char,
        _: *const libc::c_char,
        _: libc::c_ulong,
    ) -> *mut libc::c_char;
    fn strlen(_: *const libc::c_char) -> libc::c_ulong;
    fn wcschr(_: *const libc::c_int, _: libc::c_int) -> *mut libc::c_int;
    fn iswlower(__wc: wint_t) -> libc::c_int;
    fn iswupper(__wc: wint_t) -> libc::c_int;
    fn towupper(__wc: wint_t) -> wint_t;
    fn mk_entry(
        filename: *const dos_name_t,
        attr: libc::c_uchar,
        fat: libc::c_uint,
        size: uint32_t,
        date: time_t,
        ndir: *mut directory,
    ) -> *mut directory;
    fn allow_interrupts(ss: *mut saved_sig_state);
    fn GetFs(Fs: *mut Stream_t) -> *mut Stream_t;
    fn get_default_drive() -> libc::c_char;
    fn set_cmd_line_image(img: *mut libc::c_char);
    fn check_number_parse_errno(
        c: libc::c_char,
        optarg_0: *const libc::c_char,
        endptr: *mut libc::c_char,
    );
    fn ask_confirmation(_: *const libc::c_char, _: ...) -> libc::c_int;
    fn strtou32(
        nptr: *const libc::c_char,
        endptr: *mut *mut libc::c_char,
        base: libc::c_int,
    ) -> uint32_t;
    fn free_stream(Stream: *mut *mut Stream_t) -> libc::c_int;
    fn force_pwrite(
        Stream: *mut Stream_t,
        buf: *mut libc::c_char,
        start: mt_off_t,
        len: size_t,
    ) -> ssize_t;
    fn force_pread(
        Stream: *mut Stream_t,
        buf: *mut libc::c_char,
        start: mt_off_t,
        len: size_t,
    ) -> ssize_t;
    fn getTimeNow(now: *mut time_t) -> time_t;
    fn mwrite_one(
        Dir: *mut Stream_t,
        argname: *const libc::c_char,
        shortname: *const libc::c_char,
        cb: Option::<write_data_callback>,
        arg: *mut libc::c_void,
        ch: *mut ClashHandling_t,
    ) -> libc::c_int;
    static mut progname: *const libc::c_char;
    static mut mdate: *const libc::c_char;
    static mut mversion: *const libc::c_char;
    fn helpFlag(_: libc::c_int, _: *mut *mut libc::c_char) -> libc::c_int;
    fn init_clash_handling(ch: *mut ClashHandling_t);
    fn open_root_dir(
        drivename: libc::c_char,
        flags: libc::c_int,
        isRop: *mut libc::c_int,
    ) -> *mut Stream_t;
    fn wchar_to_dos(
        toDos: *mut doscp_t,
        wchar: *mut wchar_t,
        dos: *mut libc::c_char,
        len: size_t,
        mangled: *mut libc::c_int,
    );
    fn native_to_wchar(
        native: *const libc::c_char,
        wchar: *mut wchar_t,
        len: size_t,
        end: *const libc::c_char,
        mangled: *mut libc::c_int,
    ) -> size_t;
    fn vfat_lookup(
        entry: *mut direntry_t,
        filename: *const libc::c_char,
        length: size_t,
        flags: libc::c_int,
        shortname: *mut libc::c_char,
        shortname_len: size_t,
        longname: *mut libc::c_char,
        longname_len: size_t,
    ) -> libc::c_int;
    fn isNotFound(entry: *mut direntry_t) -> libc::c_int;
    fn initializeDirentry(entry: *mut direntry_t, Dir: *mut Stream_t);
    fn wipeEntry(entry: *mut direntry_t);
}
pub type __uint8_t = libc::c_uchar;
pub type __uint16_t = libc::c_ushort;
pub type __int32_t = libc::c_int;
pub type __uint32_t = libc::c_uint;
pub type __uid_t = libc::c_uint;
pub type __off_t = libc::c_long;
pub type __off64_t = libc::c_long;
pub type __pid_t = libc::c_int;
pub type __clock_t = libc::c_long;
pub type __time_t = libc::c_long;
pub type __ssize_t = libc::c_long;
pub type off_t = __off_t;
pub type ssize_t = __ssize_t;
pub type time_t = __time_t;
pub type size_t = libc::c_ulong;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct __sigset_t {
    pub __val: [libc::c_ulong; 16],
}
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
pub type __sigval_t = sigval;
#[derive(Copy, Clone)]
#[repr(C)]
pub union sigval {
    pub sival_int: libc::c_int,
    pub sival_ptr: *mut libc::c_void,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct siginfo_t {
    pub si_signo: libc::c_int,
    pub si_errno: libc::c_int,
    pub si_code: libc::c_int,
    pub __pad0: libc::c_int,
    pub _sifields: C2RustUnnamed,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed {
    pub _pad: [libc::c_int; 28],
    pub _kill: C2RustUnnamed_8,
    pub _timer: C2RustUnnamed_7,
    pub _rt: C2RustUnnamed_6,
    pub _sigchld: C2RustUnnamed_5,
    pub _sigfault: C2RustUnnamed_2,
    pub _sigpoll: C2RustUnnamed_1,
    pub _sigsys: C2RustUnnamed_0,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_0 {
    pub _call_addr: *mut libc::c_void,
    pub _syscall: libc::c_int,
    pub _arch: libc::c_uint,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_1 {
    pub si_band: libc::c_long,
    pub si_fd: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_2 {
    pub si_addr: *mut libc::c_void,
    pub si_addr_lsb: libc::c_short,
    pub _bounds: C2RustUnnamed_3,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_3 {
    pub _addr_bnd: C2RustUnnamed_4,
    pub _pkey: __uint32_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_4 {
    pub _lower: *mut libc::c_void,
    pub _upper: *mut libc::c_void,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_5 {
    pub si_pid: __pid_t,
    pub si_uid: __uid_t,
    pub si_status: libc::c_int,
    pub si_utime: __clock_t,
    pub si_stime: __clock_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_6 {
    pub si_pid: __pid_t,
    pub si_uid: __uid_t,
    pub si_sigval: __sigval_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_7 {
    pub si_tid: libc::c_int,
    pub si_overrun: libc::c_int,
    pub si_sigval: __sigval_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_8 {
    pub si_pid: __pid_t,
    pub si_uid: __uid_t,
}
pub type __sighandler_t = Option::<unsafe extern "C" fn(libc::c_int) -> ()>;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct sigaction {
    pub __sigaction_handler: C2RustUnnamed_9,
    pub sa_mask: __sigset_t,
    pub sa_flags: libc::c_int,
    pub sa_restorer: Option::<unsafe extern "C" fn() -> ()>,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_9 {
    pub sa_handler: __sighandler_t,
    pub sa_sigaction: Option::<
        unsafe extern "C" fn(libc::c_int, *mut siginfo_t, *mut libc::c_void) -> (),
    >,
}
pub type wint_t = libc::c_uint;
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
pub type mt_off_t = off_t;
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
pub struct dos_name_t {
    pub base: [libc::c_char; 8],
    pub ext: [libc::c_char; 3],
    pub sentinel: libc::c_char,
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
pub struct saved_sig_state {
    pub sa: [sigaction; 4],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union bootsector {
    pub bytes: [libc::c_uchar; 4096],
    pub characters: [libc::c_char; 4096],
    pub boot: bootsector_s,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct bootsector_s {
    pub jump: [libc::c_uchar; 3],
    pub banner: [libc::c_char; 8],
    pub secsiz: [libc::c_uchar; 2],
    pub clsiz: libc::c_uchar,
    pub nrsvsect: [libc::c_uchar; 2],
    pub nfat: libc::c_uchar,
    pub dirents: [libc::c_uchar; 2],
    pub psect: [libc::c_uchar; 2],
    pub descr: libc::c_uchar,
    pub fatlen: [libc::c_uchar; 2],
    pub nsect: [libc::c_uchar; 2],
    pub nheads: [libc::c_uchar; 2],
    pub nhs: [libc::c_uchar; 4],
    pub bigsect: [libc::c_uchar; 4],
    pub ext: C2RustUnnamed_10,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_10 {
    pub fat32: fat32_t,
    pub old: oldboot_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct oldboot_t {
    pub labelBlock: label_blk_t,
    pub res_2m: libc::c_uchar,
    pub CheckSum: libc::c_uchar,
    pub fmt_2mf: libc::c_uchar,
    pub wt: libc::c_uchar,
    pub rate_0: libc::c_uchar,
    pub rate_any: libc::c_uchar,
    pub BootP: [libc::c_uchar; 2],
    pub Infp0: [libc::c_uchar; 2],
    pub InfpX: [libc::c_uchar; 2],
    pub InfTm: [libc::c_uchar; 2],
    pub DateF: [libc::c_uchar; 2],
    pub TimeF: [libc::c_uchar; 2],
    pub junk: [libc::c_uchar; 944],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct label_blk_t {
    pub physdrive: libc::c_uchar,
    pub reserved: libc::c_uchar,
    pub dos4: libc::c_uchar,
    pub serial: [libc::c_uchar; 4],
    pub label: [libc::c_char; 11],
    pub fat_type: [libc::c_char; 8],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct fat32_t {
    pub bigFat: [libc::c_uchar; 4],
    pub extFlags: [libc::c_uchar; 2],
    pub fsVersion: [libc::c_uchar; 2],
    pub rootCluster: [libc::c_uchar; 4],
    pub infoSector: [libc::c_uchar; 2],
    pub backupBoot: [libc::c_uchar; 2],
    pub reserved: [libc::c_uchar; 6],
    pub reserved2: [libc::c_uchar; 6],
    pub labelBlock: label_blk_t,
}
pub const SER_NONE: C2RustUnnamed_11 = 0;
pub type C2RustUnnamed_11 = libc::c_uint;
pub const SER_SET: C2RustUnnamed_11 = 2;
pub const SER_RANDOM: C2RustUnnamed_11 = 1;
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
pub struct direntry_t {
    pub Dir: *mut Stream_t,
    pub entry: libc::c_int,
    pub dir: directory,
    pub name: [wchar_t; 256],
    pub beginSlot: libc::c_uint,
    pub endSlot: libc::c_uint,
}
pub type write_data_callback = unsafe extern "C" fn(
    *mut dos_name_t,
    *mut libc::c_char,
    *mut libc::c_void,
    *mut direntry_t,
) -> libc::c_int;
#[inline]
unsafe extern "C" fn toupper(mut __c: libc::c_int) -> libc::c_int {
    return if __c >= -(128 as libc::c_int) && __c < 256 as libc::c_int {
        *(*__ctype_toupper_loc()).offset(__c as isize)
    } else {
        __c
    };
}
#[inline]
unsafe extern "C" fn ch_toupper(mut ch: libc::c_char) -> libc::c_char {
    return ({
        let mut __res: libc::c_int = 0;
        if ::std::mem::size_of::<libc::c_uchar>() as libc::c_ulong
            > 1 as libc::c_int as libc::c_ulong
        {
            if 0 != 0 {
                let mut __c: libc::c_int = ch as libc::c_uchar as libc::c_int;
                __res = if __c < -(128 as libc::c_int) || __c > 255 as libc::c_int {
                    __c
                } else {
                    *(*__ctype_toupper_loc()).offset(__c as isize)
                };
            } else {
                __res = toupper(ch as libc::c_uchar as libc::c_int);
            }
        } else {
            __res = *(*__ctype_toupper_loc())
                .offset(ch as libc::c_uchar as libc::c_int as isize);
        }
        __res
    }) as libc::c_char;
}
#[inline]
unsafe extern "C" fn init_random() {
    srandom(time(0 as *mut time_t) as libc::c_uint);
}
#[inline]
unsafe extern "C" fn ch_towupper(mut ch: wchar_t) -> wchar_t {
    return towupper(ch as wint_t) as wchar_t;
}
#[inline]
unsafe extern "C" fn set_dword(mut data: *mut libc::c_uchar, mut value: uint32_t) {
    *data
        .offset(
            3 as libc::c_int as isize,
        ) = (value >> 24 as libc::c_int & 0xff as libc::c_int as libc::c_uint)
        as libc::c_uchar;
    *data
        .offset(
            2 as libc::c_int as isize,
        ) = (value >> 16 as libc::c_int & 0xff as libc::c_int as libc::c_uint)
        as libc::c_uchar;
    *data
        .offset(
            1 as libc::c_int as isize,
        ) = (value >> 8 as libc::c_int & 0xff as libc::c_int as libc::c_uint)
        as libc::c_uchar;
    *data
        .offset(
            0 as libc::c_int as isize,
        ) = (value >> 0 as libc::c_int & 0xff as libc::c_int as libc::c_uint)
        as libc::c_uchar;
}
unsafe extern "C" fn _label_name(
    mut cp: *mut doscp_t,
    mut filename: *const libc::c_char,
    mut verbose: libc::c_int,
    mut mangled: *mut libc::c_int,
    mut ans: *mut dos_name_t,
    mut preserve_case: libc::c_int,
) {
    let mut len: size_t = 0;
    let mut i: size_t = 0;
    let mut have_lower: libc::c_int = 0;
    let mut have_upper: libc::c_int = 0;
    let mut wbuffer: [wchar_t; 12] = [0; 12];
    memset(
        ans as *mut libc::c_void,
        ' ' as i32,
        (::std::mem::size_of::<dos_name_t>() as libc::c_ulong)
            .wrapping_sub(1 as libc::c_int as libc::c_ulong),
    );
    (*ans).sentinel = '\0' as i32 as libc::c_char;
    len = native_to_wchar(
        filename,
        wbuffer.as_mut_ptr(),
        11 as libc::c_int as size_t,
        0 as *const libc::c_char,
        0 as *mut libc::c_int,
    );
    if len > 11 as libc::c_int as libc::c_ulong {
        *mangled = 1 as libc::c_int;
        len = 11 as libc::c_int as size_t;
    } else {
        *mangled = 0 as libc::c_int;
    }
    have_upper = 0 as libc::c_int;
    have_lower = have_upper;
    i = 0 as libc::c_int as size_t;
    while i < len {
        if iswlower(wbuffer[i as usize] as wint_t) != 0 {
            have_lower = 1 as libc::c_int;
        }
        if iswupper(wbuffer[i as usize] as wint_t) != 0 {
            have_upper = 1 as libc::c_int;
        }
        if preserve_case == 0 {
            wbuffer[i as usize] = ch_towupper(wbuffer[i as usize]);
        }
        if !(wcschr(
            (*::std::mem::transmute::<
                &[u8; 68],
                &[libc::c_int; 17],
            >(
                b"^\0\0\0+\0\0\0=\0\0\0/\0\0\0[\0\0\0]\0\0\0:\0\0\0,\0\0\0?\0\0\0*\0\0\0\\\0\0\0<\0\0\0>\0\0\0|\0\0\0\"\0\0\0.\0\0\0\0\0\0\0",
            ))
                .as_ptr(),
            wbuffer[i as usize],
        ))
            .is_null()
        {
            *mangled = 1 as libc::c_int;
            wbuffer[i as usize] = '~' as i32;
        }
        i = i.wrapping_add(1);
        i;
    }
    if have_lower != 0 && have_upper != 0 {
        *mangled = 1 as libc::c_int;
    }
    wchar_to_dos(cp, wbuffer.as_mut_ptr(), ((*ans).base).as_mut_ptr(), len, mangled);
}
pub unsafe extern "C" fn label_name_uc(
    mut cp: *mut doscp_t,
    mut filename: *const libc::c_char,
    mut verbose: libc::c_int,
    mut mangled: *mut libc::c_int,
    mut ans: *mut dos_name_t,
) {
    _label_name(cp, filename, verbose, mangled, ans, 0 as libc::c_int);
}
pub unsafe extern "C" fn label_name_pc(
    mut cp: *mut doscp_t,
    mut filename: *const libc::c_char,
    mut verbose: libc::c_int,
    mut mangled: *mut libc::c_int,
    mut ans: *mut dos_name_t,
) {
    _label_name(cp, filename, verbose, mangled, ans, 1 as libc::c_int);
}
pub unsafe extern "C" fn labelit(
    mut dosname: *mut dos_name_t,
    mut longname: *mut libc::c_char,
    mut arg0: *mut libc::c_void,
    mut entry: *mut direntry_t,
) -> libc::c_int {
    let mut now: time_t = 0;
    getTimeNow(&mut now);
    mk_entry(
        dosname,
        0x8 as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uint,
        0 as libc::c_int as uint32_t,
        now,
        &mut (*entry).dir,
    );
    return 0 as libc::c_int;
}
unsafe extern "C" fn usage(mut ret: libc::c_int) -> ! {
    fprintf(
        stderr,
        b"Mtools version %s, dated %s\n\0" as *const u8 as *const libc::c_char,
        mversion,
        mdate,
    );
    fprintf(
        stderr,
        b"Usage: %s [-vscVn] [-N serial] drive:\n\0" as *const u8 as *const libc::c_char,
        progname,
    );
    exit(ret);
}
pub unsafe extern "C" fn mlabel(
    mut argc: libc::c_int,
    mut argv: *mut *mut libc::c_char,
    mut type_0: libc::c_int,
) {
    let mut newLabel: *const libc::c_char = b"\0" as *const u8 as *const libc::c_char;
    let mut verbose: libc::c_int = 0;
    let mut clear: libc::c_int = 0;
    let mut interactive: libc::c_int = 0;
    let mut show: libc::c_int = 0;
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
    let mut result: libc::c_int = 0 as libc::c_int;
    let mut longname: [libc::c_char; 261] = [0; 261];
    let mut shortname: [libc::c_char; 45] = [0; 45];
    let mut ch: ClashHandling_t = ClashHandling_t {
        action: [NAMEMATCH_NONE; 2],
        namematch_default: [NAMEMATCH_NONE; 2],
        nowarn: 0,
        got_slots: 0,
        mod_time: 0,
        myname: 0 as *mut libc::c_char,
        dosname: 0 as *mut libc::c_uchar,
        single: 0,
        use_longname: 0,
        ignore_entry: 0,
        source: 0,
        source_entry: 0,
        name_converter: None,
        is_label: 0,
    };
    let mut RootDir: *mut Stream_t = 0 as *mut Stream_t;
    let mut c: libc::c_int = 0;
    let mut mangled: libc::c_int = 0;
    let mut set_serial: C2RustUnnamed_11 = SER_NONE;
    let mut serial: uint32_t = 0 as libc::c_int as uint32_t;
    let mut need_write_boot: libc::c_int = 0 as libc::c_int;
    let mut have_boot: libc::c_int = 0 as libc::c_int;
    let mut eptr: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut boot: bootsector = bootsector { bytes: [0; 4096] };
    let mut Fs: *mut Stream_t = 0 as *mut Stream_t;
    let mut r: libc::c_int = 0;
    let mut labelBlock: *mut label_blk_t = 0 as *mut label_blk_t;
    let mut isRo: libc::c_int = 0 as libc::c_int;
    let mut isRop: *mut libc::c_int = 0 as *mut libc::c_int;
    let mut drive: libc::c_char = 0;
    init_clash_handling(&mut ch);
    ch
        .name_converter = Some(
        label_name_uc
            as unsafe extern "C" fn(
                *mut doscp_t,
                *const libc::c_char,
                libc::c_int,
                *mut libc::c_int,
                *mut dos_name_t,
            ) -> (),
    );
    ch.ignore_entry = -(2 as libc::c_int);
    ch.is_label = 1 as libc::c_int;
    verbose = 0 as libc::c_int;
    clear = 0 as libc::c_int;
    show = 0 as libc::c_int;
    if helpFlag(argc, argv) != 0 {
        usage(0 as libc::c_int);
    }
    loop {
        c = getopt(argc, argv, b"i:vcsnN:h\0" as *const u8 as *const libc::c_char);
        if !(c != -(1 as libc::c_int)) {
            break;
        }
        match c {
            105 => {
                set_cmd_line_image(optarg);
            }
            118 => {
                verbose = 1 as libc::c_int;
            }
            99 => {
                clear = 1 as libc::c_int;
            }
            115 => {
                show = 1 as libc::c_int;
            }
            110 => {
                set_serial = SER_RANDOM;
                init_random();
                serial = random() as uint32_t;
            }
            78 => {
                set_serial = SER_SET;
                *__errno_location() = 0 as libc::c_int;
                serial = strtou32(optarg, &mut eptr, 16 as libc::c_int);
                if *eptr != 0 {
                    fprintf(
                        stderr,
                        b"%s not a valid serial number\n\0" as *const u8
                            as *const libc::c_char,
                        optarg,
                    );
                    exit(1 as libc::c_int);
                }
                check_number_parse_errno(c as libc::c_char, optarg, eptr);
            }
            104 => {
                usage(0 as libc::c_int);
            }
            _ => {
                usage(1 as libc::c_int);
            }
        }
    }
    if argc - optind > 1 as libc::c_int {
        usage(1 as libc::c_int);
    }
    if argc - optind == 1 as libc::c_int {
        if *(*argv.offset(optind as isize)).offset(0 as libc::c_int as isize) == 0
            || *(*argv.offset(optind as isize)).offset(1 as libc::c_int as isize)
                as libc::c_int != ':' as i32
        {
            usage(1 as libc::c_int);
        }
        drive = ch_toupper(
            *(*argv.offset((argc - 1 as libc::c_int) as isize))
                .offset(0 as libc::c_int as isize),
        );
        newLabel = (*argv.offset(optind as isize)).offset(2 as libc::c_int as isize);
    } else {
        drive = get_default_drive();
    }
    if strlen(newLabel)
        > (20 as libc::c_int * 13 as libc::c_int + 1 as libc::c_int) as libc::c_ulong
    {
        fprintf(stderr, b"Label too long\n\0" as *const u8 as *const libc::c_char);
        free_stream(&mut RootDir);
        exit(1 as libc::c_int);
    }
    interactive = (show == 0 && clear == 0
        && *newLabel.offset(0 as libc::c_int as isize) == 0
        && set_serial as libc::c_uint == SER_NONE as libc::c_int as libc::c_uint)
        as libc::c_int;
    if clear == 0 && *newLabel.offset(0 as libc::c_int as isize) == 0 {
        isRop = &mut isRo;
    }
    if clear != 0 && *newLabel.offset(0 as libc::c_int as isize) as libc::c_int != 0 {
        fprintf(
            stderr,
            b"Both clear and new label specified\n\0" as *const u8 as *const libc::c_char,
        );
        free_stream(&mut RootDir);
        exit(1 as libc::c_int);
    }
    RootDir = open_root_dir(
        drive,
        if !isRop.is_null() { 0 as libc::c_int } else { 0o2 as libc::c_int },
        isRop,
    );
    if isRo != 0 {
        show = 1 as libc::c_int;
        interactive = 0 as libc::c_int;
    }
    if RootDir.is_null() {
        fprintf(
            stderr,
            b"%s: Cannot initialize drive\n\0" as *const u8 as *const libc::c_char,
            *argv.offset(0 as libc::c_int as isize),
        );
        exit(1 as libc::c_int);
    }
    initializeDirentry(&mut entry, RootDir);
    r = vfat_lookup(
        &mut entry,
        0 as *const libc::c_char,
        0 as libc::c_int as size_t,
        0x8 as libc::c_int | 0x40 as libc::c_int,
        shortname.as_mut_ptr(),
        ::std::mem::size_of::<[libc::c_char; 45]>() as libc::c_ulong,
        longname.as_mut_ptr(),
        ::std::mem::size_of::<[libc::c_char; 261]>() as libc::c_ulong,
    );
    if r == -(2 as libc::c_int) {
        free_stream(&mut RootDir);
        exit(1 as libc::c_int);
    }
    if show != 0 || interactive != 0 {
        if isNotFound(&mut entry) != 0 {
            printf(b" Volume has no label\n\0" as *const u8 as *const libc::c_char);
        } else if *longname.as_mut_ptr() != 0 {
            printf(
                b" Volume label is %s (abbr=%s)\n\0" as *const u8 as *const libc::c_char,
                longname.as_mut_ptr(),
                shortname.as_mut_ptr(),
            );
        } else {
            printf(
                b" Volume label is %s\n\0" as *const u8 as *const libc::c_char,
                shortname.as_mut_ptr(),
            );
        }
    }
    if interactive != 0 {
        let mut ss: saved_sig_state = saved_sig_state {
            sa: [sigaction {
                __sigaction_handler: C2RustUnnamed_9 {
                    sa_handler: None,
                },
                sa_mask: __sigset_t { __val: [0; 16] },
                sa_flags: 0,
                sa_restorer: None,
            }; 4],
        };
        newLabel = longname.as_mut_ptr();
        allow_interrupts(&mut ss);
        fprintf(
            stderr,
            b"Enter the new volume label : \0" as *const u8 as *const libc::c_char,
        );
        if (fgets(
            longname.as_mut_ptr(),
            20 as libc::c_int * 13 as libc::c_int + 1 as libc::c_int,
            stdin,
        ))
            .is_null()
        {
            fprintf(stderr, b"\n\0" as *const u8 as *const libc::c_char);
            if *__errno_location() == 4 as libc::c_int {
                free_stream(&mut RootDir);
                exit(1 as libc::c_int);
            }
            longname[0 as libc::c_int as usize] = '\0' as i32 as libc::c_char;
        }
        if longname[0 as libc::c_int as usize] != 0 {
            longname[(strlen(newLabel)).wrapping_sub(1 as libc::c_int as libc::c_ulong)
                as usize] = '\0' as i32 as libc::c_char;
        }
    }
    if strlen(newLabel) > 11 as libc::c_int as libc::c_ulong {
        fprintf(stderr, b"New label too long\n\0" as *const u8 as *const libc::c_char);
        free_stream(&mut RootDir);
        exit(1 as libc::c_int);
    }
    if (show == 0 || *newLabel.offset(0 as libc::c_int as isize) as libc::c_int != 0)
        && isNotFound(&mut entry) == 0
    {
        if interactive != 0
            && *newLabel.offset(0 as libc::c_int as isize) as libc::c_int == '\0' as i32
        {
            if ask_confirmation(
                b"Delete volume label (y/n): \0" as *const u8 as *const libc::c_char,
            ) != 0
            {
                free_stream(&mut RootDir);
                exit(0 as libc::c_int);
            }
        }
        entry.dir.attr = 0 as libc::c_int as libc::c_uchar;
        wipeEntry(&mut entry);
    }
    if *newLabel.offset(0 as libc::c_int as isize) as libc::c_int != '\0' as i32 {
        ch.ignore_entry = 1 as libc::c_int;
        result = if mwrite_one(
            RootDir,
            newLabel,
            0 as *const libc::c_char,
            Some(
                labelit
                    as unsafe extern "C" fn(
                        *mut dos_name_t,
                        *mut libc::c_char,
                        *mut libc::c_void,
                        *mut direntry_t,
                    ) -> libc::c_int,
            ),
            0 as *mut libc::c_void,
            &mut ch,
        ) != 0
        {
            0 as libc::c_int
        } else {
            1 as libc::c_int
        };
    }
    have_boot = 0 as libc::c_int;
    if show == 0 || *newLabel.offset(0 as libc::c_int as isize) as libc::c_int != 0
        || set_serial as libc::c_uint != SER_NONE as libc::c_int as libc::c_uint
    {
        Fs = GetFs(RootDir);
        have_boot = (force_pread(
            Fs,
            (boot.characters).as_mut_ptr(),
            0 as libc::c_int as mt_off_t,
            ::std::mem::size_of::<bootsector>() as libc::c_ulong,
        ) as libc::c_ulong == ::std::mem::size_of::<bootsector>() as libc::c_ulong)
            as libc::c_int;
    }
    if (boot.boot.fatlen[0 as libc::c_int as usize] as libc::c_int
        + ((boot.boot.fatlen[1 as libc::c_int as usize] as libc::c_int)
            << 8 as libc::c_int)) as uint16_t != 0
    {
        labelBlock = &mut boot.boot.ext.old.labelBlock;
    } else {
        labelBlock = &mut boot.boot.ext.fat32.labelBlock;
    }
    if show == 0 || *newLabel.offset(0 as libc::c_int as isize) as libc::c_int != 0 {
        let mut dosname: dos_name_t = dos_name_t {
            base: [0; 8],
            ext: [0; 3],
            sentinel: 0,
        };
        let mut shrtLabel: *const libc::c_char = 0 as *const libc::c_char;
        let mut cp: *mut doscp_t = 0 as *mut doscp_t;
        if *newLabel.offset(0 as libc::c_int as isize) == 0 {
            shrtLabel = b"NO NAME    \0" as *const u8 as *const libc::c_char;
        } else {
            shrtLabel = newLabel;
        }
        cp = ((*(*Fs).Class).get_dosConvert).unwrap()(Fs);
        label_name_pc(cp, shrtLabel, verbose, &mut mangled, &mut dosname);
        if have_boot != 0 && boot.boot.descr as libc::c_int >= 0xf0 as libc::c_int
            && ((*labelBlock).dos4 as libc::c_int == 0x28 as libc::c_int
                || (*labelBlock).dos4 as libc::c_int == 0x29 as libc::c_int)
        {
            strncpy(
                ((*labelBlock).label).as_mut_ptr(),
                (dosname.base).as_mut_ptr(),
                8 as libc::c_int as libc::c_ulong,
            );
            strncpy(
                ((*labelBlock).label).as_mut_ptr().offset(8 as libc::c_int as isize),
                (dosname.ext).as_mut_ptr(),
                3 as libc::c_int as libc::c_ulong,
            );
            need_write_boot = 1 as libc::c_int;
        }
    }
    if (set_serial as libc::c_uint != SER_NONE as libc::c_int as libc::c_uint)
        as libc::c_int & have_boot != 0
    {
        if have_boot != 0 && boot.boot.descr as libc::c_int >= 0xf0 as libc::c_int
            && ((*labelBlock).dos4 as libc::c_int == 0x28 as libc::c_int
                || (*labelBlock).dos4 as libc::c_int == 0x29 as libc::c_int)
        {
            set_dword(((*labelBlock).serial).as_mut_ptr(), serial);
            need_write_boot = 1 as libc::c_int;
        }
    }
    if need_write_boot != 0 {
        force_pwrite(
            Fs,
            &mut boot as *mut bootsector as *mut libc::c_char,
            0 as libc::c_int as mt_off_t,
            ::std::mem::size_of::<bootsector>() as libc::c_ulong,
        );
        if (boot.boot.fatlen[0 as libc::c_int as usize] as libc::c_int
            + ((boot.boot.fatlen[1 as libc::c_int as usize] as libc::c_int)
                << 8 as libc::c_int)) as uint16_t == 0
        {
            let mut backupBoot: libc::c_int = (boot
                .boot
                .ext
                .fat32
                .backupBoot[0 as libc::c_int as usize] as libc::c_int
                + ((boot.boot.ext.fat32.backupBoot[1 as libc::c_int as usize]
                    as libc::c_int) << 8 as libc::c_int)) as uint16_t as libc::c_int;
            force_pwrite(
                Fs,
                &mut boot as *mut bootsector as *mut libc::c_char,
                (backupBoot
                    * (boot.boot.secsiz[0 as libc::c_int as usize] as libc::c_int
                        + ((boot.boot.secsiz[1 as libc::c_int as usize] as libc::c_int)
                            << 8 as libc::c_int)) as uint16_t as libc::c_int)
                    as mt_off_t,
                ::std::mem::size_of::<bootsector>() as libc::c_ulong,
            );
        }
    }
    free_stream(&mut RootDir);
    exit(result);
}
