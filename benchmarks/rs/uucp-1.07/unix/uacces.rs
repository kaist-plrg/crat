use ::libc;
extern "C" {
    fn strcmp(_: *const libc::c_char, _: *const libc::c_char) -> libc::c_int;
    fn zbufcpy(z: *const libc::c_char) -> *mut libc::c_char;
    fn ubuffree(z: *mut libc::c_char);
    fn xrealloc(_: pointer, _: size_t) -> pointer;
    fn xfree(_: pointer);
    fn geteuid() -> __uid_t;
    fn getpwnam(__name: *const libc::c_char) -> *mut passwd;
    fn setgrent();
    fn endgrent();
    fn getgrent() -> *mut group;
}
pub type size_t = libc::c_ulong;
pub type __dev_t = libc::c_ulong;
pub type __uid_t = libc::c_uint;
pub type __gid_t = libc::c_uint;
pub type __ino_t = libc::c_ulong;
pub type __mode_t = libc::c_uint;
pub type __nlink_t = libc::c_ulong;
pub type __off_t = libc::c_long;
pub type __time_t = libc::c_long;
pub type __blksize_t = libc::c_long;
pub type __blkcnt_t = libc::c_long;
pub type __syscall_slong_t = libc::c_long;
pub type uid_t = __uid_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct timespec {
    pub tv_sec: __time_t,
    pub tv_nsec: __syscall_slong_t,
}
pub type pointer = *mut libc::c_void;
pub type gid_t = __gid_t;
pub type boolean = libc::c_int;
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
pub struct group {
    pub gr_name: *mut libc::c_char,
    pub gr_passwd: *mut libc::c_char,
    pub gr_gid: __gid_t,
    pub gr_mem: *mut *mut libc::c_char,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct passwd {
    pub pw_name: *mut libc::c_char,
    pub pw_passwd: *mut libc::c_char,
    pub pw_uid: __uid_t,
    pub pw_gid: __gid_t,
    pub pw_gecos: *mut libc::c_char,
    pub pw_dir: *mut libc::c_char,
    pub pw_shell: *mut libc::c_char,
}
pub unsafe extern "C" fn fsuser_access(
    mut q: *const stat,
    mut imode: libc::c_int,
    mut zuser: *const libc::c_char,
) -> boolean {
    static mut zuser_hold: *mut libc::c_char = 0 as *const libc::c_char
        as *mut libc::c_char;
    static mut iuid_hold: uid_t = 0;
    static mut igid_hold: gid_t = 0;
    static mut cgroups_hold: libc::c_int = 0;
    static mut paigroups_hold: *mut gid_t = 0 as *const gid_t as *mut gid_t;
    let mut ir: libc::c_uint = 0;
    let mut iw: libc::c_uint = 0;
    let mut ix: libc::c_uint = 0;
    let mut iand: libc::c_uint = 0;
    if imode == 0 as libc::c_int {
        return 1 as libc::c_int;
    }
    if !zuser.is_null() {
        if zuser_hold.is_null() || strcmp(zuser_hold, zuser) != 0 as libc::c_int {
            let mut qpwd: *mut passwd = 0 as *mut passwd;
            if !zuser_hold.is_null() {
                ubuffree(zuser_hold);
                zuser_hold = 0 as *mut libc::c_char;
                cgroups_hold = 0 as libc::c_int;
                xfree(paigroups_hold as pointer);
                paigroups_hold = 0 as *mut gid_t;
            }
            qpwd = getpwnam(zuser as *mut libc::c_char);
            if qpwd.is_null() {
                zuser = 0 as *const libc::c_char;
            } else {
                let mut qg: *mut group = 0 as *mut group;
                zuser_hold = zbufcpy(zuser);
                iuid_hold = (*qpwd).pw_uid;
                igid_hold = (*qpwd).pw_gid;
                setgrent();
                loop {
                    qg = getgrent();
                    if qg.is_null() {
                        break;
                    }
                    let mut pz: *mut *const libc::c_char = 0 as *mut *const libc::c_char;
                    if (*qg).gr_gid == igid_hold {
                        continue;
                    }
                    pz = (*qg).gr_mem as *mut *const libc::c_char;
                    while !(*pz).is_null() {
                        if *(*pz).offset(0 as libc::c_int as isize) as libc::c_int
                            == *zuser as libc::c_int
                            && strcmp(*pz, zuser) == 0 as libc::c_int
                        {
                            paigroups_hold = xrealloc(
                                paigroups_hold as pointer,
                                ((cgroups_hold + 1 as libc::c_int) as libc::c_ulong)
                                    .wrapping_mul(
                                        ::std::mem::size_of::<gid_t>() as libc::c_ulong,
                                    ),
                            ) as *mut gid_t;
                            *paigroups_hold.offset(cgroups_hold as isize) = (*qg).gr_gid;
                            cgroups_hold += 1;
                            cgroups_hold;
                            break;
                        } else {
                            pz = pz.offset(1);
                            pz;
                        }
                    }
                }
                endgrent();
            }
        }
    }
    if !zuser.is_null() {
        if iuid_hold == 0 as libc::c_int as libc::c_uint {
            return 1 as libc::c_int;
        }
        if iuid_hold == geteuid() {
            return 1 as libc::c_int;
        }
    }
    ir = (0o400 as libc::c_int >> 3 as libc::c_int >> 3 as libc::c_int) as libc::c_uint;
    iw = (0o200 as libc::c_int >> 3 as libc::c_int >> 3 as libc::c_int) as libc::c_uint;
    ix = (0o100 as libc::c_int >> 3 as libc::c_int >> 3 as libc::c_int) as libc::c_uint;
    if !zuser.is_null() {
        if iuid_hold == (*q).st_uid {
            ir = 0o400 as libc::c_int as libc::c_uint;
            iw = 0o200 as libc::c_int as libc::c_uint;
            ix = 0o100 as libc::c_int as libc::c_uint;
        } else {
            let mut fgroup: boolean = 0;
            fgroup = 0 as libc::c_int;
            if igid_hold == (*q).st_gid {
                fgroup = 1 as libc::c_int;
            } else {
                let mut i: libc::c_int = 0;
                i = 0 as libc::c_int;
                while i < cgroups_hold {
                    if *paigroups_hold.offset(i as isize) == (*q).st_gid {
                        fgroup = 1 as libc::c_int;
                        break;
                    } else {
                        i += 1;
                        i;
                    }
                }
            }
            if fgroup != 0 {
                ir = (0o400 as libc::c_int >> 3 as libc::c_int) as libc::c_uint;
                iw = (0o200 as libc::c_int >> 3 as libc::c_int) as libc::c_uint;
                ix = (0o100 as libc::c_int >> 3 as libc::c_int) as libc::c_uint;
            }
        }
    }
    iand = 0 as libc::c_int as libc::c_uint;
    if imode & 4 as libc::c_int != 0 as libc::c_int {
        iand |= ir;
    }
    if imode & 2 as libc::c_int != 0 as libc::c_int {
        iand |= iw;
    }
    if imode & 1 as libc::c_int != 0 as libc::c_int {
        iand |= ix;
    }
    return ((*q).st_mode & iand == iand) as libc::c_int;
}
