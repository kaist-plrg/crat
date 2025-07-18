use ::libc;
extern "C" {
    pub type _IO_wide_data;
    pub type _IO_codecvt;
    pub type _IO_marker;
    pub type wlink;
    pub type atat;
    pub type fro;
    pub type ephemstuff;
    pub type isr_scratch;
    pub type hash;
    pub type lockdef;
    pub type link;
    fn rpl_sprintf(
        str: *mut libc::c_char,
        format: *const libc::c_char,
        _: ...
    ) -> libc::c_int;
    static mut top: *mut top;
    fn strtol(
        _: *const libc::c_char,
        _: *mut *mut libc::c_char,
        _: libc::c_int,
    ) -> libc::c_long;
    fn generic_error(who: *const libc::c_char, fmt: *const libc::c_char, _: ...);
    fn generic_fatal(who: *const libc::c_char, fmt: *const libc::c_char, _: ...);
    fn parzone(s: *const libc::c_char, zone: *mut libc::c_long) -> *const libc::c_char;
    fn local_tm(timep: *const time_t, result: *mut tm) -> *mut tm;
    fn time2tm(unixtime: time_t, localzone: bool) -> *mut tm;
    fn difftm(a: *const tm, b: *const tm) -> time_t;
    fn str2time(
        source: *const libc::c_char,
        default_time: time_t,
        default_zone: libc::c_long,
    ) -> time_t;
    fn tm2time(tm: *mut tm, localzone: bool, yweek: libc::c_int) -> time_t;
    fn adjzone(t: *mut tm, seconds: libc::c_long);
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
pub type off_t = __off_t;
pub type time_t = __time_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct timespec {
    pub tv_sec: __time_t,
    pub tv_nsec: __syscall_slong_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct tm {
    pub tm_sec: libc::c_int,
    pub tm_min: libc::c_int,
    pub tm_hour: libc::c_int,
    pub tm_mday: libc::c_int,
    pub tm_mon: libc::c_int,
    pub tm_year: libc::c_int,
    pub tm_wday: libc::c_int,
    pub tm_yday: libc::c_int,
    pub tm_isdst: libc::c_int,
    pub tm_gmtoff: libc::c_long,
    pub tm_zone: *const libc::c_char,
}
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct cbuf {
    pub string: *const libc::c_char,
    pub size: size_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct delta {
    pub num: *const libc::c_char,
    pub date: *const libc::c_char,
    pub author: *const libc::c_char,
    pub lockedby: *const libc::c_char,
    pub state: *const libc::c_char,
    pub log: *mut atat,
    pub text: *mut atat,
    pub name: *const libc::c_char,
    pub pretty_log: cbuf,
    pub branches: *mut wlink,
    pub commitid: *const libc::c_char,
    pub ilk: *mut delta,
    pub selector: bool,
    pub neck: off_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct program {
    pub invoke: *const libc::c_char,
    pub name: *const libc::c_char,
    pub desc: *const libc::c_char,
    pub help: *const libc::c_char,
    pub tyag: libc::c_int,
}
pub type maker = libc::c_uint;
pub const effective: maker = 2;
pub const real: maker = 1;
pub const notmade: maker = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct sff {
    pub filename: *const libc::c_char,
    pub disposition: maker,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct behavior {
    pub invdir: *const libc::c_char,
    pub unbuffered: bool,
    pub quiet: bool,
    pub interactive_valid: bool,
    pub interactive: bool,
    pub inclusive_of_Locker_in_Id_val: bool,
    pub strictly_locking: bool,
    pub version_set: bool,
    pub version: libc::c_int,
    pub stick_with_euid: bool,
    pub ruid: libc::c_int,
    pub euid: libc::c_int,
    pub ruid_cached: bool,
    pub euid_cached: bool,
    pub already_setuid: bool,
    pub kws: libc::c_int,
    pub pe: *const libc::c_char,
    pub zone_offset: zone_offset,
    pub username: *mut libc::c_char,
    pub now: timespec,
    pub fixed_SIGCHLD: bool,
    pub Oerrloop: bool,
    pub cwd: *mut libc::c_char,
    pub mem_limit: off_t,
    pub sff: *mut sff,
    pub isr: *mut isr_scratch,
    pub ephemstuff: *mut ephemstuff,
    pub maketimestuff: *mut maketimestuff,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct maketimestuff {
    pub tzset_already_called: bool,
    pub TZ: *const libc::c_char,
    pub time2tm_stash: tm,
    pub t_cache: [time_t; 2],
    pub tm_cache: [tm; 2],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct zone_offset {
    pub valid: bool,
    pub seconds: libc::c_long,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct manifestation {
    pub filename: *mut libc::c_char,
    pub standard_output: *mut FILE,
    pub prev: C2RustUnnamed,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed {
    pub valid: bool,
    pub author: *mut libc::c_char,
    pub date: *mut libc::c_char,
    pub name: *mut libc::c_char,
    pub rev: *mut libc::c_char,
    pub state: *mut libc::c_char,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct repo {
    pub head: *const libc::c_char,
    pub branch: *const libc::c_char,
    pub access_count: size_t,
    pub access: *mut link,
    pub symbols_count: size_t,
    pub symbols: *mut link,
    pub locks_count: size_t,
    pub locks: *mut link,
    pub strict: bool,
    pub integrity: *mut atat,
    pub comment: *mut atat,
    pub expand: libc::c_int,
    pub deltas_count: size_t,
    pub deltas: *mut wlink,
    pub desc: *mut atat,
    pub neck: off_t,
    pub lockdefs: *mut lockdef,
    pub ht: *mut hash,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct repository {
    pub filename: *const libc::c_char,
    pub fd_lock: libc::c_int,
    pub stat: stat,
    pub r: *mut repo,
    pub tip: *mut delta,
    pub log_lead: cbuf,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct flow {
    pub from: *mut fro,
    pub rewr: *mut FILE,
    pub to: *mut FILE,
    pub res: *mut FILE,
    pub result: *const libc::c_char,
    pub erroneous: bool,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct top {
    pub program: *const program,
    pub behavior: behavior,
    pub manifestation: manifestation,
    pub repository: repository,
    pub flow: flow,
}
pub unsafe extern "C" fn time2date(mut unixtime: time_t, mut date: *mut libc::c_char) {
    let mut tm: *const tm = time2tm(
        unixtime,
        (*top).behavior.version < 5 as libc::c_int - 5 as libc::c_int,
    );
    rpl_sprintf(
        date,
        b"%.2d.%.2d.%.2d.%.2d.%.2d.%.2d\0" as *const u8 as *const libc::c_char,
        (*tm).tm_year
            + (if ((*tm).tm_year as libc::c_uint) < 100 as libc::c_int as libc::c_uint {
                0 as libc::c_int
            } else {
                1900 as libc::c_int
            }),
        (*tm).tm_mon + 1 as libc::c_int,
        (*tm).tm_mday,
        (*tm).tm_hour,
        (*tm).tm_min,
        (*tm).tm_sec,
    );
}
unsafe extern "C" fn str2time_checked(
    mut source: *const libc::c_char,
    mut default_time: time_t,
    mut default_zone: libc::c_long,
) -> time_t {
    let mut t: time_t = str2time(source, default_time, default_zone);
    if t == -(1 as libc::c_int) as libc::c_long {
        generic_fatal(
            0 as *const libc::c_char,
            b"unknown date/time: %s\0" as *const u8 as *const libc::c_char,
            source,
        );
    }
    return t;
}
pub unsafe extern "C" fn str2date(
    mut source: *const libc::c_char,
    mut target: *mut libc::c_char,
) {
    time2date(
        str2time_checked(
            source,
            (*top).behavior.now.tv_sec,
            if (*top).behavior.zone_offset.valid as libc::c_int != 0 {
                (*top).behavior.zone_offset.seconds
            } else if (*top).behavior.version < 5 as libc::c_int - 5 as libc::c_int {
                -(24 as libc::c_int) as libc::c_long * 60 as libc::c_int as libc::c_long
                    * 60 as libc::c_int as libc::c_long
                    - 1 as libc::c_int as libc::c_long
            } else {
                0 as libc::c_int as libc::c_long
            },
        ),
        target,
    );
}
pub unsafe extern "C" fn date2time(mut source: *const libc::c_char) -> time_t {
    let mut s: [libc::c_char; 31] = [0; 31];
    return str2time_checked(
        date2str(source, s.as_mut_ptr()),
        0 as libc::c_int as time_t,
        0 as libc::c_int as libc::c_long,
    );
}
pub unsafe extern "C" fn zone_set(mut s: *const libc::c_char) {
    (*top).behavior.zone_offset.valid = *s != 0;
    if (*top).behavior.zone_offset.valid {
        let mut zone: libc::c_long = 0;
        let mut zonetail: *const libc::c_char = parzone(s, &mut zone);
        if zonetail.is_null() || *zonetail as libc::c_int != 0 {
            generic_error(
                0 as *const libc::c_char,
                b"%s: not a known time zone\0" as *const u8 as *const libc::c_char,
                s,
            );
        } else {
            (*top).behavior.zone_offset.seconds = zone;
        }
    }
}
pub unsafe extern "C" fn date2str(
    mut date: *const libc::c_char,
    mut datebuf: *mut libc::c_char,
) -> *const libc::c_char {
    let mut p: *const libc::c_char = date;
    loop {
        let fresh0 = p;
        p = p.offset(1);
        if !(*fresh0 as libc::c_int != '.' as i32) {
            break;
        }
    }
    if !(*top).behavior.zone_offset.valid {
        rpl_sprintf(
            datebuf,
            (b"19%.*s/%.2s/%.2s %.2s:%.2s:%s\0" as *const u8 as *const libc::c_char)
                .offset(
                    (if *date.offset(2 as libc::c_int as isize) as libc::c_int
                        == '.' as i32
                        && 5 as libc::c_int - 5 as libc::c_int <= (*top).behavior.version
                    {
                        0 as libc::c_int
                    } else {
                        2 as libc::c_int
                    }) as isize,
                ),
            (p.offset_from(date) as libc::c_long - 1 as libc::c_int as libc::c_long)
                as libc::c_int,
            date,
            p,
            p.offset(3 as libc::c_int as isize),
            p.offset(6 as libc::c_int as isize),
            p.offset(9 as libc::c_int as isize),
            p.offset(12 as libc::c_int as isize),
        );
    } else {
        let mut q: *mut libc::c_char = 0 as *mut libc::c_char;
        let mut t: tm = tm {
            tm_sec: 0,
            tm_min: 0,
            tm_hour: 0,
            tm_mday: 0,
            tm_mon: 0,
            tm_year: 0,
            tm_wday: 0,
            tm_yday: 0,
            tm_isdst: 0,
            tm_gmtoff: 0,
            tm_zone: 0 as *const libc::c_char,
        };
        let mut z: *const tm = 0 as *const tm;
        let mut z_stash: tm = tm {
            tm_sec: 0,
            tm_min: 0,
            tm_hour: 0,
            tm_mday: 0,
            tm_mon: 0,
            tm_year: 0,
            tm_wday: 0,
            tm_yday: 0,
            tm_isdst: 0,
            tm_gmtoff: 0,
            tm_zone: 0 as *const libc::c_char,
        };
        let mut non_hour: libc::c_int = 0;
        let mut w: libc::c_int = 0;
        let mut zone: libc::c_long = 0;
        let mut c: libc::c_char = 0;
        p = date;
        t.tm_year = strtol(p, &mut q, 10 as libc::c_int) as libc::c_int;
        p = q.offset(1 as libc::c_int as isize);
        if '.' as i32 != *date.offset(2 as libc::c_int as isize) as libc::c_int {
            t.tm_year -= 1900 as libc::c_int;
        }
        t.tm_mon = strtol(p, &mut q, 10 as libc::c_int) as libc::c_int;
        p = q.offset(1 as libc::c_int as isize);
        t.tm_mon -= 1;
        t.tm_mon;
        t.tm_mday = strtol(p, &mut q, 10 as libc::c_int) as libc::c_int;
        p = q.offset(1 as libc::c_int as isize);
        t.tm_hour = strtol(p, &mut q, 10 as libc::c_int) as libc::c_int;
        p = q.offset(1 as libc::c_int as isize);
        t.tm_min = strtol(p, &mut q, 10 as libc::c_int) as libc::c_int;
        p = q.offset(1 as libc::c_int as isize);
        t.tm_sec = strtol(p, &mut q, 10 as libc::c_int) as libc::c_int;
        p = q.offset(1 as libc::c_int as isize);
        t.tm_wday = -(1 as libc::c_int);
        t.tm_yday = -(1 as libc::c_int);
        zone = (*top).behavior.zone_offset.seconds;
        if zone
            == -(24 as libc::c_int) as libc::c_long * 60 as libc::c_int as libc::c_long
                * 60 as libc::c_int as libc::c_long - 1 as libc::c_int as libc::c_long
        {
            let mut u: time_t = tm2time(
                &mut t,
                0 as libc::c_int != 0,
                -(1 as libc::c_int),
            );
            let mut d: time_t = 0;
            z = local_tm(&mut u, &mut z_stash);
            d = difftm(z, &mut t);
            zone = if (-(1 as libc::c_int) as time_t) < 0 as libc::c_int as libc::c_long
                || d < -d
            {
                d
            } else {
                --d
            };
        } else {
            adjzone(&mut t, zone);
            z = &mut t;
        }
        c = '+' as i32 as libc::c_char;
        if zone < 0 as libc::c_int as libc::c_long {
            zone = -zone;
            c = '-' as i32 as libc::c_char;
        }
        w = rpl_sprintf(
            datebuf,
            b"%.2d-%.2d-%.2d %.2d:%.2d:%.2d%c%.2d\0" as *const u8 as *const libc::c_char,
            (*z).tm_year + 1900 as libc::c_int,
            (*z).tm_mon + 1 as libc::c_int,
            (*z).tm_mday,
            (*z).tm_hour,
            (*z).tm_min,
            (*z).tm_sec,
            c as libc::c_int,
            (zone / (60 as libc::c_int * 60 as libc::c_int) as libc::c_long)
                as libc::c_int,
        );
        non_hour = (zone % (60 as libc::c_int * 60 as libc::c_int) as libc::c_long)
            as libc::c_int;
        if non_hour != 0 {
            let mut fmt: *const libc::c_char = b":%.2d\0" as *const u8
                as *const libc::c_char;
            w
                += rpl_sprintf(
                    datebuf.offset(w as isize),
                    fmt,
                    non_hour / 60 as libc::c_int,
                );
            non_hour %= 60 as libc::c_int;
            if non_hour != 0 {
                w += rpl_sprintf(datebuf.offset(w as isize), fmt, non_hour);
            }
        }
    }
    return datebuf as *const libc::c_char;
}
