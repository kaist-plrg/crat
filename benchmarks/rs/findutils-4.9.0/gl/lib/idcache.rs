use ::libc;
extern "C" {
    fn strcpy(_: *mut libc::c_char, _: *const libc::c_char) -> *mut libc::c_char;
    fn strcmp(_: *const libc::c_char, _: *const libc::c_char) -> libc::c_int;
    fn strlen(_: *const libc::c_char) -> libc::c_ulong;
    fn getpwuid(__uid: __uid_t) -> *mut passwd;
    fn getpwnam(__name: *const libc::c_char) -> *mut passwd;
    fn getgrgid(__gid: __gid_t) -> *mut group;
    fn getgrnam(__name: *const libc::c_char) -> *mut group;
    fn xmalloc(s: size_t) -> *mut libc::c_void;
}
pub type __uid_t = libc::c_uint;
pub type __gid_t = libc::c_uint;
pub type gid_t = __gid_t;
pub type uid_t = __uid_t;
pub type size_t = libc::c_ulong;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct userid {
    pub id: C2RustUnnamed,
    pub next: *mut userid,
    pub name: [libc::c_char; 0],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed {
    pub u: uid_t,
    pub g: gid_t,
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct group {
    pub gr_name: *mut libc::c_char,
    pub gr_passwd: *mut libc::c_char,
    pub gr_gid: __gid_t,
    pub gr_mem: *mut *mut libc::c_char,
}
static mut user_alist: *mut userid = 0 as *const userid as *mut userid;
static mut nouser_alist: *mut userid = 0 as *const userid as *mut userid;
static mut group_alist: *mut userid = 0 as *const userid as *mut userid;
static mut nogroup_alist: *mut userid = 0 as *const userid as *mut userid;
pub unsafe extern "C" fn getuser(mut uid: uid_t) -> *mut libc::c_char {
    let mut tail: *mut userid = 0 as *mut userid;
    let mut match_0: *mut userid = 0 as *mut userid;
    tail = user_alist;
    while !tail.is_null() {
        if (*tail).id.u == uid {
            match_0 = tail;
            break;
        } else {
            tail = (*tail).next;
        }
    }
    if match_0.is_null() {
        let mut pwent: *mut passwd = getpwuid(uid);
        let mut name: *const libc::c_char = if !pwent.is_null() {
            (*pwent).pw_name as *const libc::c_char
        } else {
            b"\0" as *const u8 as *const libc::c_char
        };
        match_0 = xmalloc(
            (16 as libc::c_ulong)
                .wrapping_add(::std::mem::align_of::<userid>() as libc::c_ulong)
                .wrapping_sub(1 as libc::c_int as libc::c_ulong)
                .wrapping_add(
                    (strlen(name)).wrapping_add(1 as libc::c_int as libc::c_ulong),
                )
                & !(::std::mem::align_of::<userid>() as libc::c_ulong)
                    .wrapping_sub(1 as libc::c_int as libc::c_ulong),
        ) as *mut userid;
        (*match_0).id.u = uid;
        strcpy(((*match_0).name).as_mut_ptr(), name);
        (*match_0).next = user_alist;
        user_alist = match_0;
    }
    return if *((*match_0).name).as_mut_ptr().offset(0 as libc::c_int as isize)
        as libc::c_int != 0
    {
        ((*match_0).name).as_mut_ptr()
    } else {
        0 as *mut libc::c_char
    };
}
pub unsafe extern "C" fn getuidbyname(mut user: *const libc::c_char) -> *mut uid_t {
    let mut tail: *mut userid = 0 as *mut userid;
    let mut pwent: *mut passwd = 0 as *mut passwd;
    tail = user_alist;
    while !tail.is_null() {
        if *((*tail).name).as_mut_ptr() as libc::c_int == *user as libc::c_int
            && strcmp(((*tail).name).as_mut_ptr(), user) == 0
        {
            return &mut (*tail).id.u;
        }
        tail = (*tail).next;
    }
    tail = nouser_alist;
    while !tail.is_null() {
        if *((*tail).name).as_mut_ptr() as libc::c_int == *user as libc::c_int
            && strcmp(((*tail).name).as_mut_ptr(), user) == 0
        {
            return 0 as *mut uid_t;
        }
        tail = (*tail).next;
    }
    pwent = getpwnam(user);
    tail = xmalloc(
        (16 as libc::c_ulong)
            .wrapping_add(::std::mem::align_of::<userid>() as libc::c_ulong)
            .wrapping_sub(1 as libc::c_int as libc::c_ulong)
            .wrapping_add((strlen(user)).wrapping_add(1 as libc::c_int as libc::c_ulong))
            & !(::std::mem::align_of::<userid>() as libc::c_ulong)
                .wrapping_sub(1 as libc::c_int as libc::c_ulong),
    ) as *mut userid;
    strcpy(((*tail).name).as_mut_ptr(), user);
    if !pwent.is_null() {
        (*tail).id.u = (*pwent).pw_uid;
        (*tail).next = user_alist;
        user_alist = tail;
        return &mut (*tail).id.u;
    }
    (*tail).next = nouser_alist;
    nouser_alist = tail;
    return 0 as *mut uid_t;
}
pub unsafe extern "C" fn getgroup(mut gid: gid_t) -> *mut libc::c_char {
    let mut tail: *mut userid = 0 as *mut userid;
    let mut match_0: *mut userid = 0 as *mut userid;
    tail = group_alist;
    while !tail.is_null() {
        if (*tail).id.g == gid {
            match_0 = tail;
            break;
        } else {
            tail = (*tail).next;
        }
    }
    if match_0.is_null() {
        let mut grent: *mut group = getgrgid(gid);
        let mut name: *const libc::c_char = if !grent.is_null() {
            (*grent).gr_name as *const libc::c_char
        } else {
            b"\0" as *const u8 as *const libc::c_char
        };
        match_0 = xmalloc(
            (16 as libc::c_ulong)
                .wrapping_add(::std::mem::align_of::<userid>() as libc::c_ulong)
                .wrapping_sub(1 as libc::c_int as libc::c_ulong)
                .wrapping_add(
                    (strlen(name)).wrapping_add(1 as libc::c_int as libc::c_ulong),
                )
                & !(::std::mem::align_of::<userid>() as libc::c_ulong)
                    .wrapping_sub(1 as libc::c_int as libc::c_ulong),
        ) as *mut userid;
        (*match_0).id.g = gid;
        strcpy(((*match_0).name).as_mut_ptr(), name);
        (*match_0).next = group_alist;
        group_alist = match_0;
    }
    return if *((*match_0).name).as_mut_ptr().offset(0 as libc::c_int as isize)
        as libc::c_int != 0
    {
        ((*match_0).name).as_mut_ptr()
    } else {
        0 as *mut libc::c_char
    };
}
pub unsafe extern "C" fn getgidbyname(mut group: *const libc::c_char) -> *mut gid_t {
    let mut tail: *mut userid = 0 as *mut userid;
    let mut grent: *mut group = 0 as *mut group;
    tail = group_alist;
    while !tail.is_null() {
        if *((*tail).name).as_mut_ptr() as libc::c_int == *group as libc::c_int
            && strcmp(((*tail).name).as_mut_ptr(), group) == 0
        {
            return &mut (*tail).id.g;
        }
        tail = (*tail).next;
    }
    tail = nogroup_alist;
    while !tail.is_null() {
        if *((*tail).name).as_mut_ptr() as libc::c_int == *group as libc::c_int
            && strcmp(((*tail).name).as_mut_ptr(), group) == 0
        {
            return 0 as *mut gid_t;
        }
        tail = (*tail).next;
    }
    grent = getgrnam(group);
    tail = xmalloc(
        (16 as libc::c_ulong)
            .wrapping_add(::std::mem::align_of::<userid>() as libc::c_ulong)
            .wrapping_sub(1 as libc::c_int as libc::c_ulong)
            .wrapping_add(
                (strlen(group)).wrapping_add(1 as libc::c_int as libc::c_ulong),
            )
            & !(::std::mem::align_of::<userid>() as libc::c_ulong)
                .wrapping_sub(1 as libc::c_int as libc::c_ulong),
    ) as *mut userid;
    strcpy(((*tail).name).as_mut_ptr(), group);
    if !grent.is_null() {
        (*tail).id.g = (*grent).gr_gid;
        (*tail).next = group_alist;
        group_alist = tail;
        return &mut (*tail).id.g;
    }
    (*tail).next = nogroup_alist;
    nogroup_alist = tail;
    return 0 as *mut gid_t;
}
