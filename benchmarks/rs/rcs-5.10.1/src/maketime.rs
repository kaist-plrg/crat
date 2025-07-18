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
    fn gmtime_r(__timer: *const time_t, __tp: *mut tm) -> *mut tm;
    fn localtime_r(__timer: *const time_t, __tp: *mut tm) -> *mut tm;
    fn tzset();
    static mut top: *mut top;
    fn partime(s: *const libc::c_char, t: *mut partime) -> *const libc::c_char;
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct partime {
    pub tm: tm,
    pub ymodulus: libc::c_int,
    pub yweek: libc::c_int,
    pub zone: libc::c_long,
}
pub unsafe extern "C" fn local_tm(
    mut timep: *const time_t,
    mut result: *mut tm,
) -> *mut tm {
    if !(*(*top).behavior.maketimestuff).tzset_already_called {
        tzset();
        (*(*top).behavior.maketimestuff).tzset_already_called = 1 as libc::c_int != 0;
    }
    return localtime_r(timep, result);
}
unsafe extern "C" fn isleap(mut y: libc::c_int) -> bool {
    return y & 3 as libc::c_int == 0 as libc::c_int
        && (y % 100 as libc::c_int != 0 as libc::c_int
            || y % 400 as libc::c_int == 0 as libc::c_int);
}
static mut month_yday: [libc::c_int; 13] = [
    0 as libc::c_int,
    31 as libc::c_int,
    59 as libc::c_int,
    90 as libc::c_int,
    120 as libc::c_int,
    151 as libc::c_int,
    181 as libc::c_int,
    212 as libc::c_int,
    243 as libc::c_int,
    273 as libc::c_int,
    304 as libc::c_int,
    334 as libc::c_int,
    365 as libc::c_int,
];
unsafe extern "C" fn month_days(mut tm: *const tm) -> libc::c_int {
    let mut m: libc::c_int = (*tm).tm_mon;
    return month_yday[(m + 1 as libc::c_int) as usize] - month_yday[m as usize]
        + (m == 1 as libc::c_int
            && isleap((*tm).tm_year + 1900 as libc::c_int) as libc::c_int != 0)
            as libc::c_int;
}
pub unsafe extern "C" fn time2tm(mut unixtime: time_t, mut localzone: bool) -> *mut tm {
    let mut tm: *mut tm = 0 as *mut tm;
    if localzone as libc::c_int != 0
        || {
            tm = gmtime_r(
                &mut unixtime,
                &mut (*(*top).behavior.maketimestuff).time2tm_stash,
            );
            tm.is_null()
        }
    {
        tm = local_tm(
            &mut unixtime,
            &mut (*(*top).behavior.maketimestuff).time2tm_stash,
        );
    }
    return tm;
}
pub unsafe extern "C" fn difftm(mut a: *const tm, mut b: *const tm) -> time_t {
    let mut ay: libc::c_int = (*a).tm_year + (1900 as libc::c_int - 1 as libc::c_int);
    let mut by: libc::c_int = (*b).tm_year + (1900 as libc::c_int - 1 as libc::c_int);
    let mut difference_in_day_of_year: libc::c_int = (*a).tm_yday - (*b).tm_yday;
    let mut intervening_leap_days: libc::c_int = (ay >> 2 as libc::c_int)
        - (by >> 2 as libc::c_int) - (ay / 100 as libc::c_int - by / 100 as libc::c_int)
        + ((ay / 100 as libc::c_int >> 2 as libc::c_int)
            - (by / 100 as libc::c_int >> 2 as libc::c_int));
    let mut difference_in_years: time_t = (ay - by) as time_t;
    let mut difference_in_days: time_t = difference_in_years
        * 365 as libc::c_int as libc::c_long
        + (intervening_leap_days + difference_in_day_of_year) as libc::c_long;
    return ((24 as libc::c_int as libc::c_long * difference_in_days
        + ((*a).tm_hour - (*b).tm_hour) as libc::c_long)
        * 60 as libc::c_int as libc::c_long
        + ((*a).tm_min - (*b).tm_min) as libc::c_long)
        * 60 as libc::c_int as libc::c_long
        + ((*a).tm_sec - (*b).tm_sec) as libc::c_long;
}
pub unsafe extern "C" fn adjzone(mut t: *mut tm, mut seconds: libc::c_long) {
    let mut leap_second: libc::c_int = ((*t).tm_sec == 60 as libc::c_int) as libc::c_int;
    let mut sec: libc::c_long = seconds + ((*t).tm_sec - leap_second) as libc::c_long;
    if sec < 0 as libc::c_int as libc::c_long {
        (*t)
            .tm_min = ((*t).tm_min as libc::c_long
            - (59 as libc::c_int as libc::c_long - sec)
                / 60 as libc::c_int as libc::c_long) as libc::c_int;
        if (*t).tm_min < 0 as libc::c_int {
            (*t).tm_hour -= (59 as libc::c_int - (*t).tm_min) / 60 as libc::c_int;
            if (*t).tm_hour < 0 as libc::c_int {
                (*t).tm_hour += 24 as libc::c_int;
                if 0 as libc::c_int <= (*t).tm_wday
                    && {
                        (*t).tm_wday -= 1;
                        (*t).tm_wday < 0 as libc::c_int
                    }
                {
                    (*t).tm_wday = 6 as libc::c_int;
                }
                (*t).tm_mday -= 1;
                if (*t).tm_mday <= 0 as libc::c_int {
                    (*t).tm_mon -= 1;
                    if (*t).tm_mon < 0 as libc::c_int {
                        (*t).tm_year -= 1;
                        (*t).tm_year;
                        (*t).tm_mon = 11 as libc::c_int;
                    }
                    (*t).tm_mday = month_days(t);
                }
            }
            (*t).tm_min += 24 as libc::c_int * 60 as libc::c_int;
        }
        sec
            += 24 as libc::c_long * 60 as libc::c_int as libc::c_long
                * 60 as libc::c_int as libc::c_long;
    } else {
        (*t)
            .tm_min = ((*t).tm_min as libc::c_long
            + sec / 60 as libc::c_int as libc::c_long) as libc::c_int;
        if 60 as libc::c_int <= (*t).tm_min {
            (*t).tm_hour += (*t).tm_min / 60 as libc::c_int;
            if 24 as libc::c_int <= (*t).tm_hour {
                (*t).tm_hour -= 24 as libc::c_int;
                if 0 as libc::c_int <= (*t).tm_wday
                    && {
                        (*t).tm_wday += 1;
                        (*t).tm_wday == 7 as libc::c_int
                    }
                {
                    (*t).tm_wday = 0 as libc::c_int;
                }
                (*t).tm_mday += 1;
                if month_days(t) < (*t).tm_mday {
                    (*t).tm_mon += 1;
                    if (11 as libc::c_int) < (*t).tm_mon {
                        (*t).tm_year += 1;
                        (*t).tm_year;
                        (*t).tm_mon = 0 as libc::c_int;
                    }
                    (*t).tm_mday = 1 as libc::c_int;
                }
            }
        }
    }
    (*t).tm_min %= 60 as libc::c_int;
    (*t).tm_sec = (sec % 60 as libc::c_int as libc::c_long) as libc::c_int + leap_second;
}
unsafe extern "C" fn ISO_day_of_week(
    mut zy: libc::c_int,
    mut mij: libc::c_int,
) -> libc::c_int {
    let mut zd: libc::c_int = (mij + 365 as libc::c_int * zy + zy / 4 as libc::c_int
        - zy / 100 as libc::c_int + zy / 400 as libc::c_int) % 7 as libc::c_int;
    return if zd != 0 { zd } else { 7 as libc::c_int };
}
pub unsafe extern "C" fn tm2time(
    mut tm: *mut tm,
    mut localzone: bool,
    mut yweek: libc::c_int,
) -> time_t {
    let mut leap: bool = false;
    let mut d: time_t = 0;
    let mut gt: time_t = 0;
    let mut gtm: *const tm = 0 as *const tm;
    let mut remaining_tries: libc::c_int = 8 as libc::c_int;
    if 12 as libc::c_int as libc::c_uint <= (*tm).tm_mon as libc::c_uint {
        return -(1 as libc::c_int) as time_t;
    }
    leap = isleap((*tm).tm_year + 1900 as libc::c_int);
    if -(1 as libc::c_int) != yweek {
        let mut nyd: libc::c_int = 0;
        let mut doy: libc::c_int = 0;
        let wday: libc::c_int = if (*tm).tm_wday != 0 {
            (*tm).tm_wday
        } else {
            7 as libc::c_int
        };
        let mut zy: libc::c_int = (*tm).tm_year + 1900 as libc::c_int - 1 as libc::c_int;
        if yweek == 0 {
            zy -= 1;
            zy;
            leap = isleap(1 as libc::c_int + zy);
        }
        nyd = ISO_day_of_week(zy, 1 as libc::c_int);
        if yweek == 0 {
            yweek = 52 as libc::c_int
                + (4 as libc::c_int == nyd
                    || leap as libc::c_int != 0 && 3 as libc::c_int == nyd)
                    as libc::c_int;
        }
        doy = yweek * 7 as libc::c_int + wday - 3 as libc::c_int
            - ISO_day_of_week(zy, 4 as libc::c_int);
        if (365 as libc::c_int + leap as libc::c_int) < doy {
            doy -= 365 as libc::c_int + leap as libc::c_int;
            zy += 1;
            zy;
            leap = isleap(1 as libc::c_int + zy);
        }
        if 1 as libc::c_int > doy {
            zy -= 1;
            zy;
            leap = isleap(1 as libc::c_int + zy);
            doy += 365 as libc::c_int + leap as libc::c_int;
        }
        (*tm).tm_year = zy + 1 as libc::c_int - 1900 as libc::c_int;
        (*tm).tm_yday = doy - 1 as libc::c_int;
    }
    if 0 as libc::c_int > (*tm).tm_yday || (365 as libc::c_int) < (*tm).tm_yday {
        (*tm)
            .tm_yday = month_yday[(*tm).tm_mon as usize] + (*tm).tm_mday
            - !(leap as libc::c_int != 0 && (1 as libc::c_int) < (*tm).tm_mon)
                as libc::c_int;
    } else {
        let mut mon: libc::c_int = 1 as libc::c_int;
        let mut day: libc::c_int = 1 as libc::c_int + (*tm).tm_yday;
        while day
            > month_yday[mon as usize]
                + (leap as libc::c_int != 0 && (1 as libc::c_int) < mon) as libc::c_int
        {
            mon += 1;
            mon;
        }
        mon -= 1;
        mon;
        day
            -= month_yday[mon as usize]
                + (leap as libc::c_int != 0 && (1 as libc::c_int) < mon) as libc::c_int;
        (*tm).tm_mon = mon;
        (*tm).tm_mday = day;
    }
    gt = (*(*top).behavior.maketimestuff).t_cache[localzone as usize];
    gtm = if gt != 0 {
        &mut *((*(*top).behavior.maketimestuff).tm_cache)
            .as_mut_ptr()
            .offset(localzone as isize) as *mut tm
    } else {
        time2tm(gt, localzone)
    };
    loop {
        d = difftm(tm, gtm);
        if !(d != 0 as libc::c_int as libc::c_long) {
            break;
        }
        remaining_tries -= 1;
        if remaining_tries == 0 as libc::c_int {
            return -(1 as libc::c_int) as time_t;
        }
        gt += d;
        gtm = time2tm(gt, localzone);
    }
    (*(*top).behavior.maketimestuff).t_cache[localzone as usize] = gt;
    (*(*top).behavior.maketimestuff).tm_cache[localzone as usize] = *gtm;
    if (*tm).tm_year ^ (*gtm).tm_year | (*tm).tm_mon ^ (*gtm).tm_mon
        | (*tm).tm_mday ^ (*gtm).tm_mday | (*tm).tm_hour ^ (*gtm).tm_hour
        | (*tm).tm_min ^ (*gtm).tm_min | (*tm).tm_sec ^ (*gtm).tm_sec != 0
    {
        return -(1 as libc::c_int) as time_t;
    }
    (*tm).tm_wday = (*gtm).tm_wday;
    return gt;
}
unsafe extern "C" fn maketime(
    mut pt: *const partime,
    mut default_time: time_t,
) -> time_t {
    let mut localzone: bool = (*pt).zone
        == -(24 as libc::c_int) as libc::c_long * 60 as libc::c_int as libc::c_long
            * 60 as libc::c_int as libc::c_long - 1 as libc::c_int as libc::c_long;
    let mut wday: libc::c_int = 0;
    let mut tm: tm = tm {
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
    let mut tm0: *mut tm = 0 as *mut tm;
    let mut r: time_t = 0;
    tm0 = 0 as *mut tm;
    tm = (*pt).tm;
    if 0 as libc::c_int <= (*pt).ymodulus || !(0 as libc::c_int <= tm.tm_year) {
        tm0 = time2tm(default_time, localzone);
        if !localzone {
            adjzone(tm0, (*pt).zone);
        }
    }
    if 0 as libc::c_int <= (*pt).ymodulus {
        tm.tm_year
            += ((*tm0).tm_year + 1900 as libc::c_int) / (*pt).ymodulus * (*pt).ymodulus;
    } else if !(0 as libc::c_int <= tm.tm_year) {
        tm.tm_year = (*tm0).tm_year + 1900 as libc::c_int;
        if !(0 as libc::c_int <= tm.tm_mon) {
            tm.tm_mon = (*tm0).tm_mon;
            if !(0 as libc::c_int <= tm.tm_mday) {
                tm.tm_mday = (*tm0).tm_mday;
            }
        }
    }
    tm.tm_year -= 1900 as libc::c_int;
    if !(0 as libc::c_int <= tm.tm_mon) {
        tm.tm_mon = 0 as libc::c_int;
    }
    if !(0 as libc::c_int <= tm.tm_mday) {
        tm.tm_mday = 1 as libc::c_int;
    }
    if !(0 as libc::c_int <= tm.tm_hour) {
        tm.tm_hour = 0 as libc::c_int;
    }
    if !(0 as libc::c_int <= tm.tm_min) {
        tm.tm_min = 0 as libc::c_int;
    }
    if !(0 as libc::c_int <= tm.tm_sec) {
        tm.tm_sec = 0 as libc::c_int;
    }
    if !localzone {
        adjzone(&mut tm, -(*pt).zone);
    }
    wday = tm.tm_wday;
    r = tm2time(&mut tm, localzone, (*pt).yweek);
    if r != -(1 as libc::c_int) as libc::c_long && 0 as libc::c_int <= wday
        && wday != tm.tm_wday
    {
        return -(1 as libc::c_int) as time_t;
    }
    return r;
}
pub unsafe extern "C" fn str2time(
    mut source: *const libc::c_char,
    mut default_time: time_t,
    mut default_zone: libc::c_long,
) -> time_t {
    let mut pt: partime = partime {
        tm: tm {
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
        },
        ymodulus: 0,
        yweek: 0,
        zone: 0,
    };
    if *partime(source, &mut pt) != 0 {
        return -(1 as libc::c_int) as time_t;
    }
    if pt.zone
        == -(24 as libc::c_int) as libc::c_long * 60 as libc::c_int as libc::c_long
            * 60 as libc::c_int as libc::c_long
    {
        pt.zone = default_zone;
    }
    return maketime(&mut pt, default_time);
}
