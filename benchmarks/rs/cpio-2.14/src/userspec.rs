use ::libc;
extern "C" {
    fn __ctype_b_loc() -> *mut *const libc::c_ushort;
    fn strcpy(_: *mut libc::c_char, _: *const libc::c_char) -> *mut libc::c_char;
    fn strdup(_: *const libc::c_char) -> *mut libc::c_char;
    fn strchr(_: *const libc::c_char, _: libc::c_int) -> *mut libc::c_char;
    fn strlen(_: *const libc::c_char) -> libc::c_ulong;
    fn rpl_free(_: *mut libc::c_void);
    fn strtol(
        _: *const libc::c_char,
        _: *mut *mut libc::c_char,
        _: libc::c_int,
    ) -> libc::c_long;
    fn dcgettext(
        __domainname: *const libc::c_char,
        __msgid: *const libc::c_char,
        __category: libc::c_int,
    ) -> *mut libc::c_char;
    fn getpwnam(__name: *const libc::c_char) -> *mut passwd;
    fn getgrgid(__gid: __gid_t) -> *mut group;
    fn getgrnam(__name: *const libc::c_char) -> *mut group;
    fn umaxtostr(_: uintmax_t, _: *mut libc::c_char) -> *mut libc::c_char;
}
pub type __uintmax_t = libc::c_ulong;
pub type __uid_t = libc::c_uint;
pub type __gid_t = libc::c_uint;
pub type gid_t = __gid_t;
pub type uid_t = __uid_t;
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
pub type uintmax_t = __uintmax_t;
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct group {
    pub gr_name: *mut libc::c_char,
    pub gr_passwd: *mut libc::c_char,
    pub gr_gid: __gid_t,
    pub gr_mem: *mut *mut libc::c_char,
}
#[inline]
unsafe extern "C" fn atoi(mut __nptr: *const libc::c_char) -> libc::c_int {
    return strtol(
        __nptr,
        0 as *mut libc::c_void as *mut *mut libc::c_char,
        10 as libc::c_int,
    ) as libc::c_int;
}
unsafe extern "C" fn isnumber_p(mut str: *const libc::c_char) -> libc::c_int {
    while *str != 0 {
        if *(*__ctype_b_loc()).offset(*str as libc::c_int as isize) as libc::c_int
            & _ISdigit as libc::c_int as libc::c_ushort as libc::c_int == 0
        {
            return 0 as libc::c_int;
        }
        str = str.offset(1);
        str;
    }
    return 1 as libc::c_int;
}
pub unsafe extern "C" fn parse_user_spec(
    mut spec_arg: *const libc::c_char,
    mut uid: *mut uid_t,
    mut gid: *mut gid_t,
    mut username_arg: *mut *mut libc::c_char,
    mut groupname_arg: *mut *mut libc::c_char,
) -> *const libc::c_char {
    static mut tired: *const libc::c_char = b"virtual memory exhausted\0" as *const u8
        as *const libc::c_char;
    let mut error_msg: *const libc::c_char = 0 as *const libc::c_char;
    let mut spec: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut pwd: *mut passwd = 0 as *mut passwd;
    let mut grp: *mut group = 0 as *mut group;
    let mut g: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut u: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut separator: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut groupname: *mut libc::c_char = 0 as *mut libc::c_char;
    error_msg = 0 as *const libc::c_char;
    *groupname_arg = 0 as *mut libc::c_char;
    *username_arg = *groupname_arg;
    groupname = 0 as *mut libc::c_char;
    let mut _len: libc::c_int = strlen(spec_arg) as libc::c_int;
    let mut fresh0 = ::std::vec::from_elem(
        0,
        (_len + 1 as libc::c_int) as libc::c_ulong as usize,
    );
    spec = fresh0.leak().as_mut_ptr() as *mut libc::c_char;
    strcpy(spec, spec_arg);
    separator = strchr(spec, ':' as i32);
    if separator.is_null() {
        separator = strchr(spec, '.' as i32);
    }
    if !separator.is_null() {
        *separator = '\0' as i32 as libc::c_char;
    }
    u = if *spec as libc::c_int == '\0' as i32 { 0 as *mut libc::c_char } else { spec };
    g = if separator.is_null()
        || *separator.offset(1 as libc::c_int as isize) as libc::c_int == '\0' as i32
    {
        0 as *mut libc::c_char
    } else {
        separator.offset(1 as libc::c_int as isize)
    };
    if u.is_null() && g.is_null() {
        return b"can not omit both user and group\0" as *const u8 as *const libc::c_char;
    }
    if !u.is_null() {
        if *u as libc::c_int == '+' as i32 {
            pwd = 0 as *mut passwd;
            u = u.offset(1);
            u;
        } else {
            pwd = getpwnam(u);
        }
        if pwd.is_null() {
            if isnumber_p(u) == 0 {
                error_msg = dcgettext(
                    0 as *const libc::c_char,
                    b"invalid user\0" as *const u8 as *const libc::c_char,
                    5 as libc::c_int,
                );
            } else {
                let mut use_login_group: libc::c_int = 0;
                use_login_group = (!separator.is_null() && g.is_null()) as libc::c_int;
                if use_login_group != 0 {
                    error_msg = dcgettext(
                        0 as *const libc::c_char,
                        b"cannot get the login group of a numeric UID\0" as *const u8
                            as *const libc::c_char,
                        5 as libc::c_int,
                    );
                } else {
                    *uid = atoi(u) as uid_t;
                }
            }
        } else {
            *uid = (*pwd).pw_uid;
            if g.is_null() && !separator.is_null() {
                *gid = (*pwd).pw_gid;
                grp = getgrgid((*pwd).pw_gid);
                if grp.is_null() {
                    let mut nbuf: [libc::c_char; 21] = [0; 21];
                    let mut _len_0: libc::c_int = strlen(
                        umaxtostr((*pwd).pw_gid as uintmax_t, nbuf.as_mut_ptr()),
                    ) as libc::c_int;
                    let mut fresh1 = ::std::vec::from_elem(
                        0,
                        (_len_0 + 1 as libc::c_int) as libc::c_ulong as usize,
                    );
                    groupname = fresh1.leak().as_mut_ptr() as *mut libc::c_char;
                    strcpy(
                        groupname,
                        umaxtostr((*pwd).pw_gid as uintmax_t, nbuf.as_mut_ptr()),
                    );
                } else {
                    let mut _len_1: libc::c_int = strlen((*grp).gr_name) as libc::c_int;
                    let mut fresh2 = ::std::vec::from_elem(
                        0,
                        (_len_1 + 1 as libc::c_int) as libc::c_ulong as usize,
                    );
                    groupname = fresh2.leak().as_mut_ptr() as *mut libc::c_char;
                    strcpy(groupname, (*grp).gr_name);
                }
            }
        }
    }
    if !g.is_null() && error_msg.is_null() {
        if *g as libc::c_int == '+' as i32 {
            grp = 0 as *mut group;
            g = g.offset(1);
            g;
        } else {
            grp = getgrnam(g);
        }
        if grp.is_null() {
            if isnumber_p(g) == 0 {
                error_msg = dcgettext(
                    0 as *const libc::c_char,
                    b"invalid group\0" as *const u8 as *const libc::c_char,
                    5 as libc::c_int,
                );
            } else {
                *gid = atoi(g) as gid_t;
            }
        } else {
            *gid = (*grp).gr_gid;
        }
        if error_msg.is_null() {
            let mut _len_2: libc::c_int = strlen(g) as libc::c_int;
            let mut fresh3 = ::std::vec::from_elem(
                0,
                (_len_2 + 1 as libc::c_int) as libc::c_ulong as usize,
            );
            groupname = fresh3.leak().as_mut_ptr() as *mut libc::c_char;
            strcpy(groupname, g);
        }
    }
    if error_msg.is_null() {
        if !u.is_null() {
            *username_arg = strdup(u);
            if (*username_arg).is_null() {
                error_msg = tired;
            }
        }
        if !groupname.is_null() && error_msg.is_null() {
            *groupname_arg = strdup(groupname);
            if (*groupname_arg).is_null() {
                if !(*username_arg).is_null() {
                    rpl_free(*username_arg as *mut libc::c_void);
                    *username_arg = 0 as *mut libc::c_char;
                }
                error_msg = tired;
            }
        }
    }
    return error_msg;
}
