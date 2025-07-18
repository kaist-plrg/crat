use ::libc;
extern "C" {
    fn sprintf(_: *mut libc::c_char, _: *const libc::c_char, _: ...) -> libc::c_int;
    fn rand() -> libc::c_int;
    fn srand(__seed: libc::c_uint);
    fn getenv(__name: *const libc::c_char) -> *mut libc::c_char;
    fn __errno_location() -> *mut libc::c_int;
    fn __ctype_b_loc() -> *mut *const libc::c_ushort;
    fn strcat(_: *mut libc::c_char, _: *const libc::c_char) -> *mut libc::c_char;
    fn strcmp(_: *const libc::c_char, _: *const libc::c_char) -> libc::c_int;
    fn strchr(_: *const libc::c_char, _: libc::c_int) -> *mut libc::c_char;
    fn strrchr(_: *const libc::c_char, _: libc::c_int) -> *mut libc::c_char;
    fn strlen(_: *const libc::c_char) -> libc::c_ulong;
    fn strerror(_: libc::c_int) -> *mut libc::c_char;
    fn strcpy(_: *mut libc::c_char, _: *const libc::c_char) -> *mut libc::c_char;
    fn tzset();
    fn getpwnam(__name: *const libc::c_char) -> *mut passwd;
    fn chdir(__path: *const libc::c_char) -> libc::c_int;
    fn getcwd(__buf: *mut libc::c_char, __size: size_t) -> *mut libc::c_char;
    fn getpid() -> __pid_t;
    fn gethostname(__name: *mut libc::c_char, __len: size_t) -> libc::c_int;
    fn gettimeofday(__tv: *mut timeval, __tz: *mut libc::c_void) -> libc::c_int;
    fn getrusage(__who: __rusage_who_t, __usage: *mut rusage) -> libc::c_int;
    fn Pl_Init_Machine1();
    fn mallopt(__param: libc::c_int, __val: libc::c_int) -> libc::c_int;
    fn gethostbyname(__name: *const libc::c_char) -> *mut hostent;
    fn gethostbyaddr(
        __addr: *const libc::c_void,
        __len: __socklen_t,
        __type: libc::c_int,
    ) -> *mut hostent;
    fn inet_aton(__cp: *const libc::c_char, __inp: *mut in_addr) -> libc::c_int;
}
pub type size_t = libc::c_ulong;
pub type __uint32_t = libc::c_uint;
pub type __uid_t = libc::c_uint;
pub type __gid_t = libc::c_uint;
pub type __pid_t = libc::c_int;
pub type __time_t = libc::c_long;
pub type __suseconds_t = libc::c_long;
pub type __syscall_slong_t = libc::c_long;
pub type __socklen_t = libc::c_uint;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct timeval {
    pub tv_sec: __time_t,
    pub tv_usec: __suseconds_t,
}
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
pub type uint32_t = __uint32_t;
pub type intptr_t = libc::c_long;
pub type PlLong = intptr_t;
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
pub type __rusage_who = libc::c_int;
pub const RUSAGE_CHILDREN: __rusage_who = -1;
pub const RUSAGE_SELF: __rusage_who = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct rusage {
    pub ru_utime: timeval,
    pub ru_stime: timeval,
    pub c2rust_unnamed: C2RustUnnamed_13,
    pub c2rust_unnamed_0: C2RustUnnamed_12,
    pub c2rust_unnamed_1: C2RustUnnamed_11,
    pub c2rust_unnamed_2: C2RustUnnamed_10,
    pub c2rust_unnamed_3: C2RustUnnamed_9,
    pub c2rust_unnamed_4: C2RustUnnamed_8,
    pub c2rust_unnamed_5: C2RustUnnamed_7,
    pub c2rust_unnamed_6: C2RustUnnamed_6,
    pub c2rust_unnamed_7: C2RustUnnamed_5,
    pub c2rust_unnamed_8: C2RustUnnamed_4,
    pub c2rust_unnamed_9: C2RustUnnamed_3,
    pub c2rust_unnamed_10: C2RustUnnamed_2,
    pub c2rust_unnamed_11: C2RustUnnamed_1,
    pub c2rust_unnamed_12: C2RustUnnamed_0,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_0 {
    pub ru_nivcsw: libc::c_long,
    pub __ru_nivcsw_word: __syscall_slong_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_1 {
    pub ru_nvcsw: libc::c_long,
    pub __ru_nvcsw_word: __syscall_slong_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_2 {
    pub ru_nsignals: libc::c_long,
    pub __ru_nsignals_word: __syscall_slong_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_3 {
    pub ru_msgrcv: libc::c_long,
    pub __ru_msgrcv_word: __syscall_slong_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_4 {
    pub ru_msgsnd: libc::c_long,
    pub __ru_msgsnd_word: __syscall_slong_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_5 {
    pub ru_oublock: libc::c_long,
    pub __ru_oublock_word: __syscall_slong_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_6 {
    pub ru_inblock: libc::c_long,
    pub __ru_inblock_word: __syscall_slong_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_7 {
    pub ru_nswap: libc::c_long,
    pub __ru_nswap_word: __syscall_slong_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_8 {
    pub ru_majflt: libc::c_long,
    pub __ru_majflt_word: __syscall_slong_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_9 {
    pub ru_minflt: libc::c_long,
    pub __ru_minflt_word: __syscall_slong_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_10 {
    pub ru_isrss: libc::c_long,
    pub __ru_isrss_word: __syscall_slong_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_11 {
    pub ru_idrss: libc::c_long,
    pub __ru_idrss_word: __syscall_slong_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_12 {
    pub ru_ixrss: libc::c_long,
    pub __ru_ixrss_word: __syscall_slong_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_13 {
    pub ru_maxrss: libc::c_long,
    pub __ru_maxrss_word: __syscall_slong_t,
}
pub type __rusage_who_t = libc::c_int;
pub type Bool = libc::c_int;
pub type WamWord = intptr_t;
pub type WamWordP = *mut WamWord;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct hostent {
    pub h_name: *mut libc::c_char,
    pub h_aliases: *mut *mut libc::c_char,
    pub h_addrtype: libc::c_int,
    pub h_length: libc::c_int,
    pub h_addr_list: *mut *mut libc::c_char,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct in_addr {
    pub s_addr: in_addr_t,
}
pub type in_addr_t = uint32_t;
pub static mut pl_reg_bank: *mut WamWord = 0 as *const WamWord as *mut WamWord;
pub static mut TR: WamWordP = 0 as *const WamWord as *mut WamWord;
pub static mut B: WamWordP = 0 as *const WamWord as *mut WamWord;
pub static mut H: WamWordP = 0 as *const WamWord as *mut WamWord;
pub static mut pl_def_max_atom: PlLong = 0;
static mut start_user_time: PlLong = 0 as libc::c_int as PlLong;
static mut start_system_time: PlLong = 0 as libc::c_int as PlLong;
static mut start_real_time: PlLong = 0 as libc::c_int as PlLong;
static mut cur_seed: libc::c_int = 1 as libc::c_int;
pub unsafe extern "C" fn Pl_Init_Machine() {
    tzset();
    start_user_time = Pl_M_User_Time();
    start_system_time = Pl_M_System_Time();
    start_real_time = Pl_M_Real_Time();
    mallopt(-(4 as libc::c_int), 0 as libc::c_int);
    Pl_Init_Machine1();
}
pub unsafe extern "C" fn Pl_M_Sys_Err_String(
    mut ret_val: libc::c_int,
) -> *mut libc::c_char {
    let mut str: *mut libc::c_char = 0 as *mut libc::c_char;
    static mut buff: [libc::c_char; 64] = [0; 64];
    str = strerror(*__errno_location());
    if !str.is_null() {
        return str;
    }
    sprintf(
        buff.as_mut_ptr(),
        b"Unknown error (%d)\0" as *const u8 as *const libc::c_char,
        *__errno_location(),
    );
    return buff.as_mut_ptr();
}
pub unsafe extern "C" fn Pl_M_User_Time() -> PlLong {
    let mut user_time: PlLong = 0;
    let mut rsr_usage: rusage = rusage {
        ru_utime: timeval { tv_sec: 0, tv_usec: 0 },
        ru_stime: timeval { tv_sec: 0, tv_usec: 0 },
        c2rust_unnamed: C2RustUnnamed_13 { ru_maxrss: 0 },
        c2rust_unnamed_0: C2RustUnnamed_12 { ru_ixrss: 0 },
        c2rust_unnamed_1: C2RustUnnamed_11 { ru_idrss: 0 },
        c2rust_unnamed_2: C2RustUnnamed_10 { ru_isrss: 0 },
        c2rust_unnamed_3: C2RustUnnamed_9 { ru_minflt: 0 },
        c2rust_unnamed_4: C2RustUnnamed_8 { ru_majflt: 0 },
        c2rust_unnamed_5: C2RustUnnamed_7 { ru_nswap: 0 },
        c2rust_unnamed_6: C2RustUnnamed_6 { ru_inblock: 0 },
        c2rust_unnamed_7: C2RustUnnamed_5 { ru_oublock: 0 },
        c2rust_unnamed_8: C2RustUnnamed_4 { ru_msgsnd: 0 },
        c2rust_unnamed_9: C2RustUnnamed_3 { ru_msgrcv: 0 },
        c2rust_unnamed_10: C2RustUnnamed_2 { ru_nsignals: 0 },
        c2rust_unnamed_11: C2RustUnnamed_1 { ru_nvcsw: 0 },
        c2rust_unnamed_12: C2RustUnnamed_0 { ru_nivcsw: 0 },
    };
    getrusage(RUSAGE_SELF as libc::c_int, &mut rsr_usage);
    user_time = rsr_usage.ru_utime.tv_sec * 1000 as libc::c_int as libc::c_long
        + rsr_usage.ru_utime.tv_usec / 1000 as libc::c_int as libc::c_long;
    return user_time - start_user_time;
}
pub unsafe extern "C" fn Pl_M_System_Time() -> PlLong {
    let mut system_time: PlLong = 0;
    let mut rsr_usage: rusage = rusage {
        ru_utime: timeval { tv_sec: 0, tv_usec: 0 },
        ru_stime: timeval { tv_sec: 0, tv_usec: 0 },
        c2rust_unnamed: C2RustUnnamed_13 { ru_maxrss: 0 },
        c2rust_unnamed_0: C2RustUnnamed_12 { ru_ixrss: 0 },
        c2rust_unnamed_1: C2RustUnnamed_11 { ru_idrss: 0 },
        c2rust_unnamed_2: C2RustUnnamed_10 { ru_isrss: 0 },
        c2rust_unnamed_3: C2RustUnnamed_9 { ru_minflt: 0 },
        c2rust_unnamed_4: C2RustUnnamed_8 { ru_majflt: 0 },
        c2rust_unnamed_5: C2RustUnnamed_7 { ru_nswap: 0 },
        c2rust_unnamed_6: C2RustUnnamed_6 { ru_inblock: 0 },
        c2rust_unnamed_7: C2RustUnnamed_5 { ru_oublock: 0 },
        c2rust_unnamed_8: C2RustUnnamed_4 { ru_msgsnd: 0 },
        c2rust_unnamed_9: C2RustUnnamed_3 { ru_msgrcv: 0 },
        c2rust_unnamed_10: C2RustUnnamed_2 { ru_nsignals: 0 },
        c2rust_unnamed_11: C2RustUnnamed_1 { ru_nvcsw: 0 },
        c2rust_unnamed_12: C2RustUnnamed_0 { ru_nivcsw: 0 },
    };
    getrusage(RUSAGE_SELF as libc::c_int, &mut rsr_usage);
    system_time = rsr_usage.ru_stime.tv_sec * 1000 as libc::c_int as libc::c_long
        + rsr_usage.ru_stime.tv_usec / 1000 as libc::c_int as libc::c_long;
    return system_time - start_system_time;
}
pub unsafe extern "C" fn Pl_M_Real_Time() -> PlLong {
    let mut real_time: PlLong = 0;
    let mut tv: timeval = timeval { tv_sec: 0, tv_usec: 0 };
    gettimeofday(&mut tv, 0 as *mut libc::c_void);
    real_time = tv.tv_sec * 1000 as libc::c_int as libc::c_long
        + tv.tv_usec / 1000 as libc::c_int as libc::c_long;
    return real_time - start_real_time;
}
pub unsafe extern "C" fn Pl_M_Randomize() {
    static mut count: libc::c_int = 0 as libc::c_int;
    let mut tv: timeval = timeval { tv_sec: 0, tv_usec: 0 };
    let mut seed: libc::c_int = 0;
    gettimeofday(&mut tv, 0 as *mut libc::c_void);
    seed = (tv.tv_sec * 1000 as libc::c_int as libc::c_long
        + tv.tv_usec / 1000 as libc::c_int as libc::c_long) as libc::c_int;
    count = (count + rand()) % 0xffff as libc::c_int;
    seed = seed ^ getpid() << (seed & 0xff as libc::c_int);
    seed *= count;
    seed = seed & 0xffffff as libc::c_int;
    Pl_M_Set_Seed(seed);
}
pub unsafe extern "C" fn Pl_M_Set_Seed(mut n: libc::c_int) {
    cur_seed = n;
    srand(cur_seed as libc::c_uint);
}
pub unsafe extern "C" fn Pl_M_Get_Seed() -> libc::c_int {
    return cur_seed;
}
pub unsafe extern "C" fn Pl_M_Random_Integer(mut n: libc::c_int) -> libc::c_int {
    return (n as libc::c_double * rand() as libc::c_double
        / (2147483647 as libc::c_int as libc::c_double + 1.0f64)) as libc::c_int;
}
pub unsafe extern "C" fn Pl_M_Random_Float(mut n: libc::c_double) -> libc::c_double {
    return n * rand() as libc::c_double
        / (2147483647 as libc::c_int as libc::c_double + 1.0f64);
}
pub unsafe extern "C" fn Pl_M_Host_Name_From_Name(
    mut host_name: *mut libc::c_char,
) -> *mut libc::c_char {
    let mut current_block: u64;
    static mut buff: [libc::c_char; 4096] = [0; 4096];
    let mut host_entry: *mut hostent = 0 as *mut hostent;
    if host_name.is_null() {
        let mut length: PlLong = ::std::mem::size_of::<[libc::c_char; 4096]>()
            as libc::c_ulong as PlLong;
        host_name = buff.as_mut_ptr();
        if gethostname(buff.as_mut_ptr(), length as size_t) != 0 {
            strcpy(
                buff.as_mut_ptr(),
                b"unknown host name\0" as *const u8 as *const libc::c_char,
            );
            current_block = 1161188952840627982;
        } else {
            current_block = 820271813250567934;
        }
    } else {
        current_block = 820271813250567934;
    }
    match current_block {
        820271813250567934 => {
            if (strchr(host_name, '.' as i32)).is_null() {
                host_entry = gethostbyname(host_name);
                if !host_entry.is_null() {
                    host_name = Host_Name_From_Alias(host_entry);
                }
            }
        }
        _ => {}
    }
    return host_name;
}
pub unsafe extern "C" fn Pl_M_Host_Name_From_Adr(
    mut host_address: *mut libc::c_char,
) -> *mut libc::c_char {
    let mut host_entry: *mut hostent = 0 as *mut hostent;
    let mut iadr: in_addr = in_addr { s_addr: 0 };
    if inet_aton(host_address, &mut iadr) == 0 as libc::c_int {
        return 0 as *mut libc::c_char;
    }
    host_entry = gethostbyaddr(
        &mut iadr as *mut in_addr as *mut libc::c_char as *const libc::c_void,
        ::std::mem::size_of::<in_addr>() as libc::c_ulong as __socklen_t,
        2 as libc::c_int,
    );
    if host_entry.is_null() {
        return 0 as *mut libc::c_char;
    }
    return Host_Name_From_Alias(host_entry);
}
unsafe extern "C" fn Host_Name_From_Alias(
    mut host_entry: *mut hostent,
) -> *mut libc::c_char {
    let mut name: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut alias: *mut *mut libc::c_char = 0 as *mut *mut libc::c_char;
    let mut p: *mut libc::c_char = 0 as *mut libc::c_char;
    name = (*host_entry).h_name;
    alias = (*host_entry).h_aliases;
    loop {
        p = strchr(name, '.' as i32);
        if !(p.is_null() && !(*alias).is_null()) {
            break;
        }
        let fresh0 = alias;
        alias = alias.offset(1);
        name = *fresh0;
    }
    if !p.is_null() {
        return name;
    }
    return (*host_entry).h_name;
}
pub unsafe extern "C" fn Pl_M_Set_Working_Dir(mut path: *mut libc::c_char) -> Bool {
    let mut new_path: *mut libc::c_char = Pl_M_Absolute_Path_Name(path);
    return (!new_path.is_null() && chdir(new_path) == 0 as libc::c_int) as libc::c_int;
}
pub unsafe extern "C" fn Pl_M_Get_Working_Dir() -> *mut libc::c_char {
    static mut cur_work_dir: [libc::c_char; 4096] = [0; 4096];
    if (getcwd(
        cur_work_dir.as_mut_ptr(),
        (::std::mem::size_of::<[libc::c_char; 4096]>() as libc::c_ulong)
            .wrapping_sub(1 as libc::c_int as libc::c_ulong),
    ))
        .is_null()
    {
        strcpy(cur_work_dir.as_mut_ptr(), b".\0" as *const u8 as *const libc::c_char);
    }
    return cur_work_dir.as_mut_ptr();
}
pub unsafe extern "C" fn Pl_M_Absolute_Path_Name(
    mut src: *mut libc::c_char,
) -> *mut libc::c_char {
    static mut buff1: [libc::c_char; 4096] = [0; 4096];
    static mut buff2: [libc::c_char; 4096] = [0; 4096];
    let mut dst: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut base_dst: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut p: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut q: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut c: libc::c_char = 0;
    dst = buff1.as_mut_ptr();
    loop {
        let fresh1 = dst;
        dst = dst.offset(1);
        *fresh1 = *src;
        if !(*fresh1 != 0) {
            break;
        }
        let fresh2 = src;
        src = src.offset(1);
        c = *fresh2;
        if c as libc::c_int == '$' as i32 {
            p = dst;
            while *(*__ctype_b_loc()).offset(*src as libc::c_int as isize) as libc::c_int
                & _ISalnum as libc::c_int as libc::c_ushort as libc::c_int != 0
                || *src as libc::c_int == '_' as i32
            {
                let fresh3 = src;
                src = src.offset(1);
                let fresh4 = dst;
                dst = dst.offset(1);
                *fresh4 = *fresh3;
            }
            *dst = '\0' as i32 as libc::c_char;
            q = getenv(p);
            if !q.is_null() {
                p = p.offset(-1);
                p;
                strcpy(p, q);
                dst = p.offset(strlen(p) as isize);
            }
        }
    }
    *dst = '\0' as i32 as libc::c_char;
    src = buff1.as_mut_ptr();
    dst = buff2.as_mut_ptr();
    if *src.offset(0 as libc::c_int as isize) as libc::c_int == '~' as i32 {
        if *src.offset(1 as libc::c_int as isize) as libc::c_int == '/' as i32
            || *src.offset(1 as libc::c_int as isize) as libc::c_int == '/' as i32
            || *src.offset(1 as libc::c_int as isize) as libc::c_int == '\0' as i32
        {
            q = 0 as *mut libc::c_char;
            p = getenv(b"HOME\0" as *const u8 as *const libc::c_char);
            if p.is_null() {
                return 0 as *mut libc::c_char;
            }
            if q.is_null() {
                q = b"\0" as *const u8 as *const libc::c_char as *mut libc::c_char;
            }
            sprintf(
                dst,
                b"%s%s/%s\0" as *const u8 as *const libc::c_char,
                q,
                p,
                src.offset(1 as libc::c_int as isize),
            );
            let mut tmp: *mut libc::c_char = src;
            src = dst;
            dst = tmp;
        } else {
            let mut pw: *mut passwd = 0 as *mut passwd;
            p = src.offset(1 as libc::c_int as isize);
            while *p as libc::c_int != 0
                && !(*p as libc::c_int == '/' as i32 || *p as libc::c_int == '/' as i32)
            {
                p = p.offset(1);
                p;
            }
            *src.offset(0 as libc::c_int as isize) = *p;
            *p = '\0' as i32 as libc::c_char;
            pw = getpwnam(src.offset(1 as libc::c_int as isize));
            if pw.is_null() {
                return 0 as *mut libc::c_char;
            }
            *p = *src.offset(0 as libc::c_int as isize);
            sprintf(
                dst,
                b"%s/%s\0" as *const u8 as *const libc::c_char,
                (*pw).pw_dir,
                p,
            );
            let mut tmp_0: *mut libc::c_char = src;
            src = dst;
            dst = tmp_0;
        }
    }
    if strcmp(src, b"user\0" as *const u8 as *const libc::c_char) == 0 as libc::c_int {
        return src;
    }
    if *src.offset(0 as libc::c_int as isize) as libc::c_int != '/' as i32 {
        sprintf(
            dst,
            b"%s/%.*s\0" as *const u8 as *const libc::c_char,
            Pl_M_Get_Working_Dir(),
            4096 as libc::c_int,
            src,
        );
        let mut tmp_1: *mut libc::c_char = src;
        src = dst;
        dst = tmp_1;
    }
    base_dst = dst;
    's_231: loop {
        let fresh5 = dst;
        dst = dst.offset(1);
        *fresh5 = *src;
        if !(*fresh5 != 0) {
            break;
        }
        let fresh6 = src;
        src = src.offset(1);
        if *fresh6 as libc::c_int != '/' as i32 {
            continue;
        }
        loop {
            while *src as libc::c_int == '/' as i32 {
                src = src.offset(1);
                src;
            }
            if *src as libc::c_int != '.' as i32 {
                continue 's_231;
            }
            if !(*src.offset(1 as libc::c_int as isize) as libc::c_int == '/' as i32
                || *src.offset(1 as libc::c_int as isize) as libc::c_int == '\0' as i32)
            {
                break;
            }
            src = src.offset(1);
            src;
        }
        if *src.offset(1 as libc::c_int as isize) as libc::c_int != '.' as i32
            || *src.offset(2 as libc::c_int as isize) as libc::c_int != '/' as i32
                && *src.offset(2 as libc::c_int as isize) as libc::c_int != '\0' as i32
        {
            continue;
        }
        src = src.offset(2 as libc::c_int as isize);
        dst = dst.offset(-(2 as libc::c_int as isize));
        while dst >= base_dst && *dst as libc::c_int != '/' as i32 {
            dst = dst.offset(-1);
            dst;
        }
        if dst < base_dst {
            return 0 as *mut libc::c_char;
        }
    }
    dst = dst.offset(-1);
    dst;
    if (dst.offset_from(base_dst) as libc::c_long) < 1 as libc::c_int as libc::c_long {
        strcpy(dst, b"/\0" as *const u8 as *const libc::c_char);
    }
    if dst.offset_from(base_dst) as libc::c_long > 1 as libc::c_int as libc::c_long
        && (*dst.offset(-(1 as libc::c_int) as isize) as libc::c_int == '/' as i32
            || *dst.offset(-(1 as libc::c_int) as isize) as libc::c_int == '/' as i32)
    {
        *dst.offset(-(1 as libc::c_int) as isize) = '\0' as i32 as libc::c_char;
    }
    return base_dst;
}
pub unsafe extern "C" fn Pl_M_Is_Absolute_File_Name(
    mut path: *mut libc::c_char,
) -> Bool {
    if *path as libc::c_int == '/' as i32 || *path as libc::c_int == '/' as i32 {
        return 1 as libc::c_int;
    }
    return 0 as libc::c_int;
}
pub unsafe extern "C" fn Pl_M_Decompose_File_Name(
    mut path: *mut libc::c_char,
    mut del_trail_slashes: Bool,
    mut base: *mut *mut libc::c_char,
    mut suffix: *mut *mut libc::c_char,
) -> *mut libc::c_char {
    static mut buff_dir: [libc::c_char; 4096] = [0; 4096];
    static mut buff_base: [libc::c_char; 4096] = [0; 4096];
    let mut dir_start_pos: libc::c_int = 0 as libc::c_int;
    let mut p: *mut libc::c_char = 0 as *mut libc::c_char;
    strcpy(buff_dir.as_mut_ptr(), path);
    let mut _ptr: *mut libc::c_char = 0 as *mut libc::c_char;
    p = 0 as *mut libc::c_char;
    _ptr = buff_dir.as_mut_ptr();
    while *_ptr != 0 {
        if *_ptr as libc::c_int == '/' as i32 || *_ptr as libc::c_int == '/' as i32 {
            p = _ptr;
        }
        _ptr = _ptr.offset(1);
        _ptr;
    }
    p = if p.is_null() {
        buff_dir.as_mut_ptr().offset(dir_start_pos as isize)
    } else {
        p.offset(1 as libc::c_int as isize)
    };
    strcpy(buff_base.as_mut_ptr(), p);
    *p = '\0' as i32 as libc::c_char;
    p = strrchr(buff_base.as_mut_ptr(), '.' as i32);
    if !p.is_null() {
        *suffix = p;
    } else {
        *suffix = buff_base.as_mut_ptr().offset(strlen(buff_base.as_mut_ptr()) as isize);
    }
    if del_trail_slashes != 0 {
        if buff_dir[dir_start_pos as usize] as libc::c_int == '\0' as i32 {
            strcat(buff_dir.as_mut_ptr(), b".\0" as *const u8 as *const libc::c_char);
        } else {
            let mut len: libc::c_int = strlen(buff_dir.as_mut_ptr()) as libc::c_int;
            loop {
                len -= 1;
                if !(len >= dir_start_pos
                    && (buff_dir[len as usize] as libc::c_int == '/' as i32
                        || buff_dir[len as usize] as libc::c_int == '/' as i32))
                {
                    break;
                }
            }
            if len < dir_start_pos {
                len = dir_start_pos;
            }
            buff_dir[(len + 1 as libc::c_int) as usize] = '\0' as i32 as libc::c_char;
        }
    }
    *base = buff_base.as_mut_ptr();
    return buff_dir.as_mut_ptr();
}
