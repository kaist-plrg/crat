use ::libc;
extern "C" {
    pub type AVL;
    pub type DMP;
    pub type glp_file;
    fn _glp_jday(d: libc::c_int, m: libc::c_int, y: libc::c_int) -> libc::c_int;
    fn _glp_jdate(
        j: libc::c_int,
        d: *mut libc::c_int,
        m: *mut libc::c_int,
        y: *mut libc::c_int,
    ) -> libc::c_int;
    fn __ctype_toupper_loc() -> *mut *const __int32_t;
    fn fabs(_: libc::c_double) -> libc::c_double;
    fn floor(_: libc::c_double) -> libc::c_double;
    fn sprintf(_: *mut libc::c_char, _: *const libc::c_char, _: ...) -> libc::c_int;
    fn memcpy(
        _: *mut libc::c_void,
        _: *const libc::c_void,
        _: libc::c_ulong,
    ) -> *mut libc::c_void;
    fn strcpy(_: *mut libc::c_char, _: *const libc::c_char) -> *mut libc::c_char;
    fn strlen(_: *const libc::c_char) -> libc::c_ulong;
    fn time(__timer: *mut time_t) -> time_t;
    fn _glp_xgmtime(_: *const time_t) -> *mut tm;
    fn glp_printf(fmt: *const libc::c_char, _: ...);
    fn glp_assert_(
        expr: *const libc::c_char,
        file: *const libc::c_char,
        line: libc::c_int,
    );
    fn _glp_mpl_error(mpl: *mut MPL, fmt: *mut libc::c_char, _: ...);
}
pub type __int32_t = libc::c_int;
pub type __time_t = libc::c_long;
pub type __jmp_buf = [libc::c_long; 8];
#[derive(Copy, Clone)]
#[repr(C)]
pub struct __sigset_t {
    pub __val: [libc::c_ulong; 16],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct __jmp_buf_tag {
    pub __jmpbuf: __jmp_buf,
    pub __mask_was_saved: libc::c_int,
    pub __saved_mask: __sigset_t,
}
pub type jmp_buf = [__jmp_buf_tag; 1];
pub type time_t = __time_t;
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
pub struct RNG {
    pub A: [libc::c_int; 56],
    pub fptr: *mut libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct glp_tran {
    pub line: libc::c_int,
    pub c: libc::c_int,
    pub token: libc::c_int,
    pub imlen: libc::c_int,
    pub image: *mut libc::c_char,
    pub value: libc::c_double,
    pub b_token: libc::c_int,
    pub b_imlen: libc::c_int,
    pub b_image: *mut libc::c_char,
    pub b_value: libc::c_double,
    pub f_dots: libc::c_int,
    pub f_scan: libc::c_int,
    pub f_token: libc::c_int,
    pub f_imlen: libc::c_int,
    pub f_image: *mut libc::c_char,
    pub f_value: libc::c_double,
    pub context: *mut libc::c_char,
    pub c_ptr: libc::c_int,
    pub flag_d: libc::c_int,
    pub pool: *mut DMP,
    pub tree: *mut AVL,
    pub model: *mut STATEMENT,
    pub flag_x: libc::c_int,
    pub as_within: libc::c_int,
    pub as_in: libc::c_int,
    pub as_binary: libc::c_int,
    pub flag_s: libc::c_int,
    pub strings: *mut DMP,
    pub symbols: *mut DMP,
    pub tuples: *mut DMP,
    pub arrays: *mut DMP,
    pub members: *mut DMP,
    pub elemvars: *mut DMP,
    pub formulae: *mut DMP,
    pub elemcons: *mut DMP,
    pub a_list: *mut ARRAY,
    pub sym_buf: *mut libc::c_char,
    pub tup_buf: *mut libc::c_char,
    pub rand: *mut RNG,
    pub flag_p: libc::c_int,
    pub stmt: *mut STATEMENT,
    pub dca: *mut TABDCA,
    pub m: libc::c_int,
    pub n: libc::c_int,
    pub row: *mut *mut ELEMCON,
    pub col: *mut *mut ELEMVAR,
    pub in_fp: *mut glp_file,
    pub in_file: *mut libc::c_char,
    pub out_fp: *mut glp_file,
    pub out_file: *mut libc::c_char,
    pub prt_fp: *mut glp_file,
    pub prt_file: *mut libc::c_char,
    pub jump: jmp_buf,
    pub phase: libc::c_int,
    pub mod_file: *mut libc::c_char,
    pub mpl_buf: *mut libc::c_char,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ELEMVAR {
    pub j: libc::c_int,
    pub var: *mut VARIABLE,
    pub memb: *mut MEMBER,
    pub lbnd: libc::c_double,
    pub ubnd: libc::c_double,
    pub temp: libc::c_double,
    pub stat: libc::c_int,
    pub prim: libc::c_double,
    pub dual: libc::c_double,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct MEMBER {
    pub tuple: *mut TUPLE,
    pub next: *mut MEMBER,
    pub value: VALUE,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union VALUE {
    pub none: *mut libc::c_void,
    pub num: libc::c_double,
    pub sym: *mut SYMBOL,
    pub bit: libc::c_int,
    pub tuple: *mut TUPLE,
    pub set: *mut ELEMSET,
    pub var: *mut ELEMVAR,
    pub form: *mut FORMULA,
    pub con: *mut ELEMCON,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ELEMCON {
    pub i: libc::c_int,
    pub con: *mut CONSTRAINT,
    pub memb: *mut MEMBER,
    pub form: *mut FORMULA,
    pub lbnd: libc::c_double,
    pub ubnd: libc::c_double,
    pub stat: libc::c_int,
    pub prim: libc::c_double,
    pub dual: libc::c_double,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct FORMULA {
    pub coef: libc::c_double,
    pub var: *mut ELEMVAR,
    pub next: *mut FORMULA,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct CONSTRAINT {
    pub name: *mut libc::c_char,
    pub alias: *mut libc::c_char,
    pub dim: libc::c_int,
    pub domain: *mut DOMAIN1,
    pub type_0: libc::c_int,
    pub code: *mut CODE,
    pub lbnd: *mut CODE,
    pub ubnd: *mut CODE,
    pub array: *mut ARRAY,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ARRAY {
    pub type_0: libc::c_int,
    pub dim: libc::c_int,
    pub size: libc::c_int,
    pub head: *mut MEMBER,
    pub tail: *mut MEMBER,
    pub tree: *mut AVL,
    pub prev: *mut ARRAY,
    pub next: *mut ARRAY,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct CODE {
    pub op: libc::c_int,
    pub arg: OPERANDS,
    pub type_0: libc::c_int,
    pub dim: libc::c_int,
    pub up: *mut CODE,
    pub vflag: libc::c_int,
    pub valid: libc::c_int,
    pub value: VALUE,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union OPERANDS {
    pub num: libc::c_double,
    pub str_0: *mut libc::c_char,
    pub index: C2RustUnnamed_5,
    pub par: C2RustUnnamed_4,
    pub set: C2RustUnnamed_3,
    pub var: C2RustUnnamed_2,
    pub con: C2RustUnnamed_1,
    pub list: *mut ARG_LIST,
    pub slice: *mut DOMAIN_BLOCK,
    pub arg: C2RustUnnamed_0,
    pub loop_0: C2RustUnnamed,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed {
    pub domain: *mut DOMAIN1,
    pub x: *mut CODE,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct DOMAIN1 {
    pub list: *mut DOMAIN_BLOCK,
    pub code: *mut CODE,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct DOMAIN_BLOCK {
    pub list: *mut DOMAIN_SLOT,
    pub code: *mut CODE,
    pub backup: *mut TUPLE,
    pub next: *mut DOMAIN_BLOCK,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct TUPLE {
    pub sym: *mut SYMBOL,
    pub next: *mut TUPLE,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct SYMBOL {
    pub num: libc::c_double,
    pub str_0: *mut STRING,
}
pub type STRING = libc::c_char;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct DOMAIN_SLOT {
    pub name: *mut libc::c_char,
    pub code: *mut CODE,
    pub value: *mut SYMBOL,
    pub list: *mut CODE,
    pub next: *mut DOMAIN_SLOT,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_0 {
    pub x: *mut CODE,
    pub y: *mut CODE,
    pub z: *mut CODE,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ARG_LIST {
    pub x: *mut CODE,
    pub next: *mut ARG_LIST,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_1 {
    pub con: *mut CONSTRAINT,
    pub list: *mut ARG_LIST,
    pub suff: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_2 {
    pub var: *mut VARIABLE,
    pub list: *mut ARG_LIST,
    pub suff: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct VARIABLE {
    pub name: *mut libc::c_char,
    pub alias: *mut libc::c_char,
    pub dim: libc::c_int,
    pub domain: *mut DOMAIN1,
    pub type_0: libc::c_int,
    pub lbnd: *mut CODE,
    pub ubnd: *mut CODE,
    pub array: *mut ARRAY,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_3 {
    pub set: *mut SET,
    pub list: *mut ARG_LIST,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct SET {
    pub name: *mut libc::c_char,
    pub alias: *mut libc::c_char,
    pub dim: libc::c_int,
    pub domain: *mut DOMAIN1,
    pub dimen: libc::c_int,
    pub within: *mut WITHIN,
    pub assign: *mut CODE,
    pub option: *mut CODE,
    pub gadget: *mut GADGET,
    pub data: libc::c_int,
    pub array: *mut ARRAY,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct GADGET {
    pub set: *mut SET,
    pub ind: [libc::c_int; 20],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct WITHIN {
    pub code: *mut CODE,
    pub next: *mut WITHIN,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_4 {
    pub par: *mut PARAMETER,
    pub list: *mut ARG_LIST,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct PARAMETER {
    pub name: *mut libc::c_char,
    pub alias: *mut libc::c_char,
    pub dim: libc::c_int,
    pub domain: *mut DOMAIN1,
    pub type_0: libc::c_int,
    pub cond: *mut CONDITION,
    pub in_0: *mut WITHIN,
    pub assign: *mut CODE,
    pub option: *mut CODE,
    pub data: libc::c_int,
    pub defval: *mut SYMBOL,
    pub array: *mut ARRAY,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct CONDITION {
    pub rho: libc::c_int,
    pub code: *mut CODE,
    pub next: *mut CONDITION,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_5 {
    pub slot: *mut DOMAIN_SLOT,
    pub next: *mut CODE,
}
pub type ELEMSET = ARRAY;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct TABDCA {
    pub id: libc::c_int,
    pub link: *mut libc::c_void,
    pub na: libc::c_int,
    pub arg: *mut *mut libc::c_char,
    pub nf: libc::c_int,
    pub name: *mut *mut libc::c_char,
    pub type_0: *mut libc::c_int,
    pub num: *mut libc::c_double,
    pub str_0: *mut *mut libc::c_char,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct STATEMENT {
    pub line: libc::c_int,
    pub type_0: libc::c_int,
    pub u: C2RustUnnamed_6,
    pub next: *mut STATEMENT,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_6 {
    pub set: *mut SET,
    pub par: *mut PARAMETER,
    pub var: *mut VARIABLE,
    pub con: *mut CONSTRAINT,
    pub tab: *mut TABLE,
    pub slv: *mut libc::c_void,
    pub chk: *mut CHECK,
    pub dpy: *mut DISPLAY,
    pub prt: *mut PRINTF,
    pub fur: *mut FOR,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct FOR {
    pub domain: *mut DOMAIN1,
    pub list: *mut STATEMENT,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct PRINTF {
    pub domain: *mut DOMAIN1,
    pub fmt: *mut CODE,
    pub list: *mut PRINTF1,
    pub fname: *mut CODE,
    pub app: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct PRINTF1 {
    pub code: *mut CODE,
    pub next: *mut PRINTF1,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct DISPLAY {
    pub domain: *mut DOMAIN1,
    pub list: *mut DISPLAY1,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct DISPLAY1 {
    pub type_0: libc::c_int,
    pub u: C2RustUnnamed_7,
    pub next: *mut DISPLAY1,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_7 {
    pub slot: *mut DOMAIN_SLOT,
    pub set: *mut SET,
    pub par: *mut PARAMETER,
    pub var: *mut VARIABLE,
    pub con: *mut CONSTRAINT,
    pub code: *mut CODE,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct CHECK {
    pub domain: *mut DOMAIN1,
    pub code: *mut CODE,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct TABLE {
    pub name: *mut libc::c_char,
    pub alias: *mut libc::c_char,
    pub type_0: libc::c_int,
    pub arg: *mut TABARG,
    pub u: C2RustUnnamed_8,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_8 {
    pub in_0: C2RustUnnamed_10,
    pub out: C2RustUnnamed_9,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_9 {
    pub domain: *mut DOMAIN1,
    pub list: *mut TABOUT,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct TABOUT {
    pub code: *mut CODE,
    pub name: *mut libc::c_char,
    pub next: *mut TABOUT,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_10 {
    pub set: *mut SET,
    pub fld: *mut TABFLD,
    pub list: *mut TABIN,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct TABIN {
    pub par: *mut PARAMETER,
    pub name: *mut libc::c_char,
    pub next: *mut TABIN,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct TABFLD {
    pub name: *mut libc::c_char,
    pub next: *mut TABFLD,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct TABARG {
    pub code: *mut CODE,
    pub next: *mut TABARG,
}
pub type MPL = glp_tran;
#[inline]
unsafe extern "C" fn toupper(mut __c: libc::c_int) -> libc::c_int {
    return if __c >= -(128 as libc::c_int) && __c < 256 as libc::c_int {
        *(*__ctype_toupper_loc()).offset(__c as isize)
    } else {
        __c
    };
}
pub unsafe extern "C" fn _glp_mpl_fn_gmtime(mut mpl: *mut MPL) -> libc::c_double {
    let mut current_block: u64;
    let mut timer: time_t = 0;
    let mut tm: *mut tm = 0 as *mut tm;
    let mut j: libc::c_int = 0;
    time(&mut timer);
    if timer == -(1 as libc::c_int) as time_t {
        current_block = 12620318504059535818;
    } else {
        current_block = 16658872821858055392;
    }
    loop {
        match current_block {
            12620318504059535818 => {
                _glp_mpl_error(
                    mpl,
                    b"gmtime(); unable to obtain current calendar time\0" as *const u8
                        as *const libc::c_char as *mut libc::c_char,
                );
                current_block = 16658872821858055392;
            }
            _ => {
                tm = _glp_xgmtime(&mut timer);
                if tm.is_null() {
                    current_block = 12620318504059535818;
                    continue;
                }
                j = _glp_jday(
                    (*tm).tm_mday,
                    (*tm).tm_mon + 1 as libc::c_int,
                    1900 as libc::c_int + (*tm).tm_year,
                );
                if j < 0 as libc::c_int {
                    current_block = 12620318504059535818;
                } else {
                    break;
                }
            }
        }
    }
    return (((j - _glp_jday(1 as libc::c_int, 1 as libc::c_int, 1970 as libc::c_int))
        as libc::c_double * 24.0f64 + (*tm).tm_hour as libc::c_double) * 60.0f64
        + (*tm).tm_min as libc::c_double) * 60.0f64 + (*tm).tm_sec as libc::c_double;
}
static mut week: [*mut libc::c_char; 7] = [
    b"Monday\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"Tuesday\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"Wednesday\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"Thursday\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"Friday\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"Saturday\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"Sunday\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
];
static mut moon: [*mut libc::c_char; 12] = [
    b"January\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"February\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"March\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"April\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"May\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"June\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"July\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"August\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"September\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"October\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"November\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"December\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
];
unsafe extern "C" fn error1(
    mut mpl: *mut MPL,
    mut str: *const libc::c_char,
    mut s: *const libc::c_char,
    mut fmt: *const libc::c_char,
    mut f: *const libc::c_char,
    mut msg: *const libc::c_char,
) {
    glp_printf(
        b"Input string passed to str2time:\n\0" as *const u8 as *const libc::c_char,
    );
    glp_printf(b"%s\n\0" as *const u8 as *const libc::c_char, str);
    glp_printf(
        b"%*s\n\0" as *const u8 as *const libc::c_char,
        s.offset_from(str) as libc::c_long + 1 as libc::c_int as libc::c_long,
        b"^\0" as *const u8 as *const libc::c_char,
    );
    glp_printf(
        b"Format string passed to str2time:\n\0" as *const u8 as *const libc::c_char,
    );
    glp_printf(b"%s\n\0" as *const u8 as *const libc::c_char, fmt);
    glp_printf(
        b"%*s\n\0" as *const u8 as *const libc::c_char,
        f.offset_from(fmt) as libc::c_long + 1 as libc::c_int as libc::c_long,
        b"^\0" as *const u8 as *const libc::c_char,
    );
    _glp_mpl_error(
        mpl,
        b"%s\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        msg,
    );
}
pub unsafe extern "C" fn _glp_mpl_fn_str2time(
    mut mpl: *mut MPL,
    mut str: *const libc::c_char,
    mut fmt: *const libc::c_char,
) -> libc::c_double {
    let mut current_block: u64;
    let mut j: libc::c_int = 0;
    let mut year: libc::c_int = 0;
    let mut month: libc::c_int = 0;
    let mut day: libc::c_int = 0;
    let mut hh: libc::c_int = 0;
    let mut mm: libc::c_int = 0;
    let mut ss: libc::c_int = 0;
    let mut zone: libc::c_int = 0;
    let mut s: *const libc::c_char = 0 as *const libc::c_char;
    let mut f: *const libc::c_char = 0 as *const libc::c_char;
    ss = -(1 as libc::c_int);
    mm = ss;
    hh = mm;
    day = hh;
    month = day;
    year = month;
    zone = 2147483647 as libc::c_int;
    s = str;
    f = fmt;
    while *f as libc::c_int != '\0' as i32 {
        if *f as libc::c_int == '%' as i32 {
            f = f.offset(1);
            f;
            if *f as libc::c_int == 'b' as i32 || *f as libc::c_int == 'h' as i32 {
                let mut k: libc::c_int = 0;
                let mut name: *mut libc::c_char = 0 as *mut libc::c_char;
                if month >= 0 as libc::c_int {
                    error1(
                        mpl,
                        str,
                        s,
                        fmt,
                        f,
                        b"month multiply specified\0" as *const u8 as *const libc::c_char,
                    );
                }
                while *s as libc::c_int == ' ' as i32 {
                    s = s.offset(1);
                    s;
                }
                let mut current_block_15: u64;
                month = 1 as libc::c_int;
                while month <= 12 as libc::c_int {
                    name = moon[(month - 1 as libc::c_int) as usize];
                    k = 0 as libc::c_int;
                    loop {
                        if !(k <= 2 as libc::c_int) {
                            current_block_15 = 1054647088692577877;
                            break;
                        }
                        if ({
                            let mut __res: libc::c_int = 0;
                            if ::std::mem::size_of::<libc::c_uchar>() as libc::c_ulong
                                > 1 as libc::c_int as libc::c_ulong
                            {
                                if 0 != 0 {
                                    let mut __c: libc::c_int = *s.offset(k as isize)
                                        as libc::c_uchar as libc::c_int;
                                    __res = (if __c < -(128 as libc::c_int)
                                        || __c > 255 as libc::c_int
                                    {
                                        __c
                                    } else {
                                        *(*__ctype_toupper_loc()).offset(__c as isize)
                                    });
                                } else {
                                    __res = toupper(
                                        *s.offset(k as isize) as libc::c_uchar as libc::c_int,
                                    );
                                }
                            } else {
                                __res = *(*__ctype_toupper_loc())
                                    .offset(
                                        *s.offset(k as isize) as libc::c_uchar as libc::c_int
                                            as isize,
                                    );
                            }
                            __res
                        })
                            != ({
                                let mut __res: libc::c_int = 0;
                                if ::std::mem::size_of::<libc::c_uchar>() as libc::c_ulong
                                    > 1 as libc::c_int as libc::c_ulong
                                {
                                    if 0 != 0 {
                                        let mut __c: libc::c_int = *name.offset(k as isize)
                                            as libc::c_uchar as libc::c_int;
                                        __res = (if __c < -(128 as libc::c_int)
                                            || __c > 255 as libc::c_int
                                        {
                                            __c
                                        } else {
                                            *(*__ctype_toupper_loc()).offset(__c as isize)
                                        });
                                    } else {
                                        __res = toupper(
                                            *name.offset(k as isize) as libc::c_uchar as libc::c_int,
                                        );
                                    }
                                } else {
                                    __res = *(*__ctype_toupper_loc())
                                        .offset(
                                            *name.offset(k as isize) as libc::c_uchar as libc::c_int
                                                as isize,
                                        );
                                }
                                __res
                            })
                        {
                            current_block_15 = 11650488183268122163;
                            break;
                        }
                        k += 1;
                        k;
                    }
                    match current_block_15 {
                        11650488183268122163 => {
                            month += 1;
                            month;
                        }
                        _ => {
                            s = s.offset(3 as libc::c_int as isize);
                            k = 3 as libc::c_int;
                            while *name.offset(k as isize) as libc::c_int != '\0' as i32
                            {
                                if ({
                                    let mut __res: libc::c_int = 0;
                                    if ::std::mem::size_of::<libc::c_uchar>() as libc::c_ulong
                                        > 1 as libc::c_int as libc::c_ulong
                                    {
                                        if 0 != 0 {
                                            let mut __c: libc::c_int = *s as libc::c_uchar
                                                as libc::c_int;
                                            __res = (if __c < -(128 as libc::c_int)
                                                || __c > 255 as libc::c_int
                                            {
                                                __c
                                            } else {
                                                *(*__ctype_toupper_loc()).offset(__c as isize)
                                            });
                                        } else {
                                            __res = toupper(*s as libc::c_uchar as libc::c_int);
                                        }
                                    } else {
                                        __res = *(*__ctype_toupper_loc())
                                            .offset(*s as libc::c_uchar as libc::c_int as isize);
                                    }
                                    __res
                                })
                                    != ({
                                        let mut __res: libc::c_int = 0;
                                        if ::std::mem::size_of::<libc::c_uchar>() as libc::c_ulong
                                            > 1 as libc::c_int as libc::c_ulong
                                        {
                                            if 0 != 0 {
                                                let mut __c: libc::c_int = *name.offset(k as isize)
                                                    as libc::c_uchar as libc::c_int;
                                                __res = (if __c < -(128 as libc::c_int)
                                                    || __c > 255 as libc::c_int
                                                {
                                                    __c
                                                } else {
                                                    *(*__ctype_toupper_loc()).offset(__c as isize)
                                                });
                                            } else {
                                                __res = toupper(
                                                    *name.offset(k as isize) as libc::c_uchar as libc::c_int,
                                                );
                                            }
                                        } else {
                                            __res = *(*__ctype_toupper_loc())
                                                .offset(
                                                    *name.offset(k as isize) as libc::c_uchar as libc::c_int
                                                        as isize,
                                                );
                                        }
                                        __res
                                    })
                                {
                                    break;
                                }
                                s = s.offset(1);
                                s;
                                k += 1;
                                k;
                            }
                            break;
                        }
                    }
                }
                if month > 12 as libc::c_int {
                    error1(
                        mpl,
                        str,
                        s,
                        fmt,
                        f,
                        b"abbreviated month name missing or invalid\0" as *const u8
                            as *const libc::c_char,
                    );
                }
                current_block = 14916268686031723178;
            } else if *f as libc::c_int == 'd' as i32 {
                if day >= 0 as libc::c_int {
                    error1(
                        mpl,
                        str,
                        s,
                        fmt,
                        f,
                        b"day multiply specified\0" as *const u8 as *const libc::c_char,
                    );
                }
                while *s as libc::c_int == ' ' as i32 {
                    s = s.offset(1);
                    s;
                }
                if !('0' as i32 <= *s as libc::c_int && *s as libc::c_int <= '9' as i32)
                {
                    error1(
                        mpl,
                        str,
                        s,
                        fmt,
                        f,
                        b"day missing or invalid\0" as *const u8 as *const libc::c_char,
                    );
                }
                let fresh0 = s;
                s = s.offset(1);
                day = *fresh0 as libc::c_int - '0' as i32;
                if '0' as i32 <= *s as libc::c_int && *s as libc::c_int <= '9' as i32 {
                    let fresh1 = s;
                    s = s.offset(1);
                    day = 10 as libc::c_int * day
                        + (*fresh1 as libc::c_int - '0' as i32);
                }
                if !(1 as libc::c_int <= day && day <= 31 as libc::c_int) {
                    error1(
                        mpl,
                        str,
                        s,
                        fmt,
                        f,
                        b"day out of range\0" as *const u8 as *const libc::c_char,
                    );
                }
                current_block = 14916268686031723178;
            } else if *f as libc::c_int == 'H' as i32 {
                if hh >= 0 as libc::c_int {
                    error1(
                        mpl,
                        str,
                        s,
                        fmt,
                        f,
                        b"hour multiply specified\0" as *const u8 as *const libc::c_char,
                    );
                }
                while *s as libc::c_int == ' ' as i32 {
                    s = s.offset(1);
                    s;
                }
                if !('0' as i32 <= *s as libc::c_int && *s as libc::c_int <= '9' as i32)
                {
                    error1(
                        mpl,
                        str,
                        s,
                        fmt,
                        f,
                        b"hour missing or invalid\0" as *const u8 as *const libc::c_char,
                    );
                }
                let fresh2 = s;
                s = s.offset(1);
                hh = *fresh2 as libc::c_int - '0' as i32;
                if '0' as i32 <= *s as libc::c_int && *s as libc::c_int <= '9' as i32 {
                    let fresh3 = s;
                    s = s.offset(1);
                    hh = 10 as libc::c_int * hh + (*fresh3 as libc::c_int - '0' as i32);
                }
                if !(0 as libc::c_int <= hh && hh <= 23 as libc::c_int) {
                    error1(
                        mpl,
                        str,
                        s,
                        fmt,
                        f,
                        b"hour out of range\0" as *const u8 as *const libc::c_char,
                    );
                }
                current_block = 14916268686031723178;
            } else if *f as libc::c_int == 'm' as i32 {
                if month >= 0 as libc::c_int {
                    error1(
                        mpl,
                        str,
                        s,
                        fmt,
                        f,
                        b"month multiply specified\0" as *const u8 as *const libc::c_char,
                    );
                }
                while *s as libc::c_int == ' ' as i32 {
                    s = s.offset(1);
                    s;
                }
                if !('0' as i32 <= *s as libc::c_int && *s as libc::c_int <= '9' as i32)
                {
                    error1(
                        mpl,
                        str,
                        s,
                        fmt,
                        f,
                        b"month missing or invalid\0" as *const u8 as *const libc::c_char,
                    );
                }
                let fresh4 = s;
                s = s.offset(1);
                month = *fresh4 as libc::c_int - '0' as i32;
                if '0' as i32 <= *s as libc::c_int && *s as libc::c_int <= '9' as i32 {
                    let fresh5 = s;
                    s = s.offset(1);
                    month = 10 as libc::c_int * month
                        + (*fresh5 as libc::c_int - '0' as i32);
                }
                if !(1 as libc::c_int <= month && month <= 12 as libc::c_int) {
                    error1(
                        mpl,
                        str,
                        s,
                        fmt,
                        f,
                        b"month out of range\0" as *const u8 as *const libc::c_char,
                    );
                }
                current_block = 14916268686031723178;
            } else if *f as libc::c_int == 'M' as i32 {
                if mm >= 0 as libc::c_int {
                    error1(
                        mpl,
                        str,
                        s,
                        fmt,
                        f,
                        b"minute multiply specified\0" as *const u8
                            as *const libc::c_char,
                    );
                }
                while *s as libc::c_int == ' ' as i32 {
                    s = s.offset(1);
                    s;
                }
                if !('0' as i32 <= *s as libc::c_int && *s as libc::c_int <= '9' as i32)
                {
                    error1(
                        mpl,
                        str,
                        s,
                        fmt,
                        f,
                        b"minute missing or invalid\0" as *const u8
                            as *const libc::c_char,
                    );
                }
                let fresh6 = s;
                s = s.offset(1);
                mm = *fresh6 as libc::c_int - '0' as i32;
                if '0' as i32 <= *s as libc::c_int && *s as libc::c_int <= '9' as i32 {
                    let fresh7 = s;
                    s = s.offset(1);
                    mm = 10 as libc::c_int * mm + (*fresh7 as libc::c_int - '0' as i32);
                }
                if !(0 as libc::c_int <= mm && mm <= 59 as libc::c_int) {
                    error1(
                        mpl,
                        str,
                        s,
                        fmt,
                        f,
                        b"minute out of range\0" as *const u8 as *const libc::c_char,
                    );
                }
                current_block = 14916268686031723178;
            } else if *f as libc::c_int == 'S' as i32 {
                if ss >= 0 as libc::c_int {
                    error1(
                        mpl,
                        str,
                        s,
                        fmt,
                        f,
                        b"second multiply specified\0" as *const u8
                            as *const libc::c_char,
                    );
                }
                while *s as libc::c_int == ' ' as i32 {
                    s = s.offset(1);
                    s;
                }
                if !('0' as i32 <= *s as libc::c_int && *s as libc::c_int <= '9' as i32)
                {
                    error1(
                        mpl,
                        str,
                        s,
                        fmt,
                        f,
                        b"second missing or invalid\0" as *const u8
                            as *const libc::c_char,
                    );
                }
                let fresh8 = s;
                s = s.offset(1);
                ss = *fresh8 as libc::c_int - '0' as i32;
                if '0' as i32 <= *s as libc::c_int && *s as libc::c_int <= '9' as i32 {
                    let fresh9 = s;
                    s = s.offset(1);
                    ss = 10 as libc::c_int * ss + (*fresh9 as libc::c_int - '0' as i32);
                }
                if !(0 as libc::c_int <= ss && ss <= 60 as libc::c_int) {
                    error1(
                        mpl,
                        str,
                        s,
                        fmt,
                        f,
                        b"second out of range\0" as *const u8 as *const libc::c_char,
                    );
                }
                current_block = 14916268686031723178;
            } else if *f as libc::c_int == 'y' as i32 {
                if year >= 0 as libc::c_int {
                    error1(
                        mpl,
                        str,
                        s,
                        fmt,
                        f,
                        b"year multiply specified\0" as *const u8 as *const libc::c_char,
                    );
                }
                while *s as libc::c_int == ' ' as i32 {
                    s = s.offset(1);
                    s;
                }
                if !('0' as i32 <= *s as libc::c_int && *s as libc::c_int <= '9' as i32)
                {
                    error1(
                        mpl,
                        str,
                        s,
                        fmt,
                        f,
                        b"year missing or invalid\0" as *const u8 as *const libc::c_char,
                    );
                }
                let fresh10 = s;
                s = s.offset(1);
                year = *fresh10 as libc::c_int - '0' as i32;
                if '0' as i32 <= *s as libc::c_int && *s as libc::c_int <= '9' as i32 {
                    let fresh11 = s;
                    s = s.offset(1);
                    year = 10 as libc::c_int * year
                        + (*fresh11 as libc::c_int - '0' as i32);
                }
                year
                    += if year >= 69 as libc::c_int {
                        1900 as libc::c_int
                    } else {
                        2000 as libc::c_int
                    };
                current_block = 14916268686031723178;
            } else if *f as libc::c_int == 'Y' as i32 {
                if year >= 0 as libc::c_int {
                    error1(
                        mpl,
                        str,
                        s,
                        fmt,
                        f,
                        b"year multiply specified\0" as *const u8 as *const libc::c_char,
                    );
                }
                while *s as libc::c_int == ' ' as i32 {
                    s = s.offset(1);
                    s;
                }
                if !('0' as i32 <= *s as libc::c_int && *s as libc::c_int <= '9' as i32)
                {
                    error1(
                        mpl,
                        str,
                        s,
                        fmt,
                        f,
                        b"year missing or invalid\0" as *const u8 as *const libc::c_char,
                    );
                }
                year = 0 as libc::c_int;
                j = 1 as libc::c_int;
                while j <= 4 as libc::c_int {
                    if !('0' as i32 <= *s as libc::c_int
                        && *s as libc::c_int <= '9' as i32)
                    {
                        break;
                    }
                    let fresh12 = s;
                    s = s.offset(1);
                    year = 10 as libc::c_int * year
                        + (*fresh12 as libc::c_int - '0' as i32);
                    j += 1;
                    j;
                }
                if !(1 as libc::c_int <= year && year <= 4000 as libc::c_int) {
                    error1(
                        mpl,
                        str,
                        s,
                        fmt,
                        f,
                        b"year out of range\0" as *const u8 as *const libc::c_char,
                    );
                }
                current_block = 14916268686031723178;
            } else if *f as libc::c_int == 'z' as i32 {
                let mut z: libc::c_int = 0;
                let mut hh_0: libc::c_int = 0;
                let mut mm_0: libc::c_int = 0;
                if zone != 2147483647 as libc::c_int {
                    error1(
                        mpl,
                        str,
                        s,
                        fmt,
                        f,
                        b"time zone offset multiply specified\0" as *const u8
                            as *const libc::c_char,
                    );
                }
                while *s as libc::c_int == ' ' as i32 {
                    s = s.offset(1);
                    s;
                }
                if *s as libc::c_int == 'Z' as i32 {
                    mm_0 = 0 as libc::c_int;
                    hh_0 = mm_0;
                    z = hh_0;
                    s = s.offset(1);
                    s;
                } else {
                    if *s as libc::c_int == '+' as i32 {
                        z = 1 as libc::c_int;
                        s = s.offset(1);
                        s;
                    } else if *s as libc::c_int == '-' as i32 {
                        z = -(1 as libc::c_int);
                        s = s.offset(1);
                        s;
                    } else {
                        error1(
                            mpl,
                            str,
                            s,
                            fmt,
                            f,
                            b"time zone offset sign missing\0" as *const u8
                                as *const libc::c_char,
                        );
                    }
                    hh_0 = 0 as libc::c_int;
                    j = 1 as libc::c_int;
                    's_554: loop {
                        if j <= 2 as libc::c_int {
                            if !('0' as i32 <= *s as libc::c_int
                                && *s as libc::c_int <= '9' as i32)
                            {
                                current_block = 10060746838730062494;
                            } else {
                                current_block = 12079920068676227593;
                            }
                        } else {
                            if hh_0 > 23 as libc::c_int {
                                current_block = 3522540602934368994;
                            } else {
                                current_block = 11718254377427810743;
                            }
                            '_err2: loop {
                                match current_block {
                                    3522540602934368994 => {
                                        error1(
                                            mpl,
                                            str,
                                            s,
                                            fmt,
                                            f,
                                            b"time zone offset value out of range\0" as *const u8
                                                as *const libc::c_char,
                                        );
                                        current_block = 11718254377427810743;
                                    }
                                    _ => {
                                        if *s as libc::c_int == ':' as i32 {
                                            s = s.offset(1);
                                            s;
                                            if !('0' as i32 <= *s as libc::c_int
                                                && *s as libc::c_int <= '9' as i32)
                                            {
                                                break;
                                            }
                                        }
                                        mm_0 = 0 as libc::c_int;
                                        if !('0' as i32 <= *s as libc::c_int
                                            && *s as libc::c_int <= '9' as i32)
                                        {
                                            break 's_554;
                                        }
                                        j = 1 as libc::c_int;
                                        while j <= 2 as libc::c_int {
                                            if !('0' as i32 <= *s as libc::c_int
                                                && *s as libc::c_int <= '9' as i32)
                                            {
                                                break '_err2;
                                            }
                                            let fresh14 = s;
                                            s = s.offset(1);
                                            mm_0 = 10 as libc::c_int * mm_0
                                                + (*fresh14 as libc::c_int - '0' as i32);
                                            j += 1;
                                            j;
                                        }
                                        if mm_0 > 59 as libc::c_int {
                                            current_block = 3522540602934368994;
                                        } else {
                                            break 's_554;
                                        }
                                    }
                                }
                            }
                            current_block = 10060746838730062494;
                        }
                        match current_block {
                            10060746838730062494 => {
                                error1(
                                    mpl,
                                    str,
                                    s,
                                    fmt,
                                    f,
                                    b"time zone offset value incomplete or invalid\0"
                                        as *const u8 as *const libc::c_char,
                                );
                            }
                            _ => {}
                        }
                        let fresh13 = s;
                        s = s.offset(1);
                        hh_0 = 10 as libc::c_int * hh_0
                            + (*fresh13 as libc::c_int - '0' as i32);
                        j += 1;
                        j;
                    }
                }
                zone = z * (60 as libc::c_int * hh_0 + mm_0);
                current_block = 14916268686031723178;
            } else if *f as libc::c_int == '%' as i32 {
                current_block = 15316964327808624751;
            } else {
                error1(
                    mpl,
                    str,
                    s,
                    fmt,
                    f,
                    b"invalid conversion specifier\0" as *const u8 as *const libc::c_char,
                );
                current_block = 14916268686031723178;
            }
        } else if *f as libc::c_int == ' ' as i32 {
            current_block = 14916268686031723178;
        } else {
            current_block = 15316964327808624751;
        }
        match current_block {
            15316964327808624751 => {
                if *s as libc::c_int != *f as libc::c_int {
                    error1(
                        mpl,
                        str,
                        s,
                        fmt,
                        f,
                        b"character mismatch\0" as *const u8 as *const libc::c_char,
                    );
                }
                s = s.offset(1);
                s;
            }
            _ => {}
        }
        f = f.offset(1);
        f;
    }
    if year < 0 as libc::c_int {
        year = 1970 as libc::c_int;
    }
    if month < 0 as libc::c_int {
        month = 1 as libc::c_int;
    }
    if day < 0 as libc::c_int {
        day = 1 as libc::c_int;
    }
    if hh < 0 as libc::c_int {
        hh = 0 as libc::c_int;
    }
    if mm < 0 as libc::c_int {
        mm = 0 as libc::c_int;
    }
    if ss < 0 as libc::c_int {
        ss = 0 as libc::c_int;
    }
    if zone == 2147483647 as libc::c_int {
        zone = 0 as libc::c_int;
    }
    j = _glp_jday(day, month, year);
    (j >= 0 as libc::c_int
        || {
            glp_assert_(
                b"j >= 0\0" as *const u8 as *const libc::c_char,
                b"mpl/mpl5.c\0" as *const u8 as *const libc::c_char,
                279 as libc::c_int,
            );
            1 as libc::c_int != 0
        }) as libc::c_int;
    return (((j - _glp_jday(1 as libc::c_int, 1 as libc::c_int, 1970 as libc::c_int))
        as libc::c_double * 24.0f64 + hh as libc::c_double) * 60.0f64
        + mm as libc::c_double) * 60.0f64 + ss as libc::c_double
        - 60.0f64 * zone as libc::c_double;
}
unsafe extern "C" fn error2(
    mut mpl: *mut MPL,
    mut fmt: *const libc::c_char,
    mut f: *const libc::c_char,
    mut msg: *const libc::c_char,
) {
    glp_printf(
        b"Format string passed to time2str:\n\0" as *const u8 as *const libc::c_char,
    );
    glp_printf(b"%s\n\0" as *const u8 as *const libc::c_char, fmt);
    glp_printf(
        b"%*s\n\0" as *const u8 as *const libc::c_char,
        f.offset_from(fmt) as libc::c_long + 1 as libc::c_int as libc::c_long,
        b"^\0" as *const u8 as *const libc::c_char,
    );
    _glp_mpl_error(
        mpl,
        b"%s\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        msg,
    );
}
unsafe extern "C" fn weekday(mut j: libc::c_int) -> libc::c_int {
    return (j + _glp_jday(1 as libc::c_int, 1 as libc::c_int, 1970 as libc::c_int))
        % 7 as libc::c_int + 1 as libc::c_int;
}
unsafe extern "C" fn firstday(mut year: libc::c_int) -> libc::c_int {
    let mut j: libc::c_int = 0;
    j = _glp_jday(1 as libc::c_int, 1 as libc::c_int, year)
        - _glp_jday(1 as libc::c_int, 1 as libc::c_int, 1970 as libc::c_int);
    match weekday(j) {
        1 => {
            j += 0 as libc::c_int;
        }
        2 => {
            j -= 1 as libc::c_int;
        }
        3 => {
            j -= 2 as libc::c_int;
        }
        4 => {
            j -= 3 as libc::c_int;
        }
        5 => {
            j += 3 as libc::c_int;
        }
        6 => {
            j += 2 as libc::c_int;
        }
        7 => {
            j += 1 as libc::c_int;
        }
        _ => {
            (j != j
                || {
                    glp_assert_(
                        b"j != j\0" as *const u8 as *const libc::c_char,
                        b"mpl/mpl5.c\0" as *const u8 as *const libc::c_char,
                        314 as libc::c_int,
                    );
                    1 as libc::c_int != 0
                }) as libc::c_int;
        }
    }
    (weekday(j) == 1 as libc::c_int
        || {
            glp_assert_(
                b"weekday(j) == 1\0" as *const u8 as *const libc::c_char,
                b"mpl/mpl5.c\0" as *const u8 as *const libc::c_char,
                317 as libc::c_int,
            );
            1 as libc::c_int != 0
        }) as libc::c_int;
    return j;
}
pub unsafe extern "C" fn _glp_mpl_fn_time2str(
    mut mpl: *mut MPL,
    mut str: *mut libc::c_char,
    mut t: libc::c_double,
    mut fmt: *const libc::c_char,
) {
    let mut j: libc::c_int = 0;
    let mut year: libc::c_int = 0;
    let mut month: libc::c_int = 0;
    let mut day: libc::c_int = 0;
    let mut hh: libc::c_int = 0;
    let mut mm: libc::c_int = 0;
    let mut ss: libc::c_int = 0;
    let mut len: libc::c_int = 0;
    let mut temp: libc::c_double = 0.;
    let mut f: *const libc::c_char = 0 as *const libc::c_char;
    let mut buf: [libc::c_char; 101] = [0; 101];
    if !(-62135596800.0f64 <= t && t <= 64092211199.0f64) {
        _glp_mpl_error(
            mpl,
            b"time2str(%.*g,...); argument out of range\0" as *const u8
                as *const libc::c_char as *mut libc::c_char,
            15 as libc::c_int,
            t,
        );
    }
    t = floor(t + 0.5f64);
    temp = fabs(t) / 86400.0f64;
    j = floor(temp) as libc::c_int;
    if t < 0.0f64 {
        if temp == floor(temp) {
            j = -j;
        } else {
            j = -(j + 1 as libc::c_int);
        }
    }
    (_glp_jdate(
        j + _glp_jday(1 as libc::c_int, 1 as libc::c_int, 1970 as libc::c_int),
        &mut day,
        &mut month,
        &mut year,
    ) == 0 as libc::c_int
        || {
            glp_assert_(
                b"jdate(j + jday(1, 1, 1970), &day, &month, &year) == 0\0" as *const u8
                    as *const libc::c_char,
                b"mpl/mpl5.c\0" as *const u8 as *const libc::c_char,
                339 as libc::c_int,
            );
            1 as libc::c_int != 0
        }) as libc::c_int;
    ss = (t - 86400.0f64 * j as libc::c_double) as libc::c_int;
    (0 as libc::c_int <= ss && ss < 86400 as libc::c_int
        || {
            glp_assert_(
                b"0 <= ss && ss < 86400\0" as *const u8 as *const libc::c_char,
                b"mpl/mpl5.c\0" as *const u8 as *const libc::c_char,
                341 as libc::c_int,
            );
            1 as libc::c_int != 0
        }) as libc::c_int;
    mm = ss / 60 as libc::c_int;
    ss %= 60 as libc::c_int;
    hh = mm / 60 as libc::c_int;
    mm %= 60 as libc::c_int;
    len = 0 as libc::c_int;
    f = fmt;
    while *f as libc::c_int != '\0' as i32 {
        if *f as libc::c_int == '%' as i32 {
            f = f.offset(1);
            f;
            if *f as libc::c_int == 'a' as i32 {
                memcpy(
                    buf.as_mut_ptr() as *mut libc::c_void,
                    week[(weekday(j) - 1 as libc::c_int) as usize]
                        as *const libc::c_void,
                    3 as libc::c_int as libc::c_ulong,
                );
                buf[3 as libc::c_int as usize] = '\0' as i32 as libc::c_char;
            } else if *f as libc::c_int == 'A' as i32 {
                strcpy(buf.as_mut_ptr(), week[(weekday(j) - 1 as libc::c_int) as usize]);
            } else if *f as libc::c_int == 'b' as i32 || *f as libc::c_int == 'h' as i32
            {
                memcpy(
                    buf.as_mut_ptr() as *mut libc::c_void,
                    moon[(month - 1 as libc::c_int) as usize] as *const libc::c_void,
                    3 as libc::c_int as libc::c_ulong,
                );
                buf[3 as libc::c_int as usize] = '\0' as i32 as libc::c_char;
            } else if *f as libc::c_int == 'B' as i32 {
                strcpy(buf.as_mut_ptr(), moon[(month - 1 as libc::c_int) as usize]);
            } else if *f as libc::c_int == 'C' as i32 {
                sprintf(
                    buf.as_mut_ptr(),
                    b"%02d\0" as *const u8 as *const libc::c_char,
                    year / 100 as libc::c_int,
                );
            } else if *f as libc::c_int == 'd' as i32 {
                sprintf(
                    buf.as_mut_ptr(),
                    b"%02d\0" as *const u8 as *const libc::c_char,
                    day,
                );
            } else if *f as libc::c_int == 'D' as i32 {
                sprintf(
                    buf.as_mut_ptr(),
                    b"%02d/%02d/%02d\0" as *const u8 as *const libc::c_char,
                    month,
                    day,
                    year % 100 as libc::c_int,
                );
            } else if *f as libc::c_int == 'e' as i32 {
                sprintf(
                    buf.as_mut_ptr(),
                    b"%2d\0" as *const u8 as *const libc::c_char,
                    day,
                );
            } else if *f as libc::c_int == 'F' as i32 {
                sprintf(
                    buf.as_mut_ptr(),
                    b"%04d-%02d-%02d\0" as *const u8 as *const libc::c_char,
                    year,
                    month,
                    day,
                );
            } else if *f as libc::c_int == 'g' as i32 {
                let mut iso: libc::c_int = 0;
                if j < firstday(year) {
                    iso = year - 1 as libc::c_int;
                } else if j < firstday(year + 1 as libc::c_int) {
                    iso = year;
                } else {
                    iso = year + 1 as libc::c_int;
                }
                sprintf(
                    buf.as_mut_ptr(),
                    b"%02d\0" as *const u8 as *const libc::c_char,
                    iso % 100 as libc::c_int,
                );
            } else if *f as libc::c_int == 'G' as i32 {
                let mut iso_0: libc::c_int = 0;
                if j < firstday(year) {
                    iso_0 = year - 1 as libc::c_int;
                } else if j < firstday(year + 1 as libc::c_int) {
                    iso_0 = year;
                } else {
                    iso_0 = year + 1 as libc::c_int;
                }
                sprintf(
                    buf.as_mut_ptr(),
                    b"%04d\0" as *const u8 as *const libc::c_char,
                    iso_0,
                );
            } else if *f as libc::c_int == 'H' as i32 {
                sprintf(
                    buf.as_mut_ptr(),
                    b"%02d\0" as *const u8 as *const libc::c_char,
                    hh,
                );
            } else if *f as libc::c_int == 'I' as i32 {
                sprintf(
                    buf.as_mut_ptr(),
                    b"%02d\0" as *const u8 as *const libc::c_char,
                    if hh == 0 as libc::c_int {
                        12 as libc::c_int
                    } else if hh <= 12 as libc::c_int {
                        hh
                    } else {
                        hh - 12 as libc::c_int
                    },
                );
            } else if *f as libc::c_int == 'j' as i32 {
                sprintf(
                    buf.as_mut_ptr(),
                    b"%03d\0" as *const u8 as *const libc::c_char,
                    _glp_jday(day, month, year)
                        - _glp_jday(1 as libc::c_int, 1 as libc::c_int, year)
                        + 1 as libc::c_int,
                );
            } else if *f as libc::c_int == 'k' as i32 {
                sprintf(
                    buf.as_mut_ptr(),
                    b"%2d\0" as *const u8 as *const libc::c_char,
                    hh,
                );
            } else if *f as libc::c_int == 'l' as i32 {
                sprintf(
                    buf.as_mut_ptr(),
                    b"%2d\0" as *const u8 as *const libc::c_char,
                    if hh == 0 as libc::c_int {
                        12 as libc::c_int
                    } else if hh <= 12 as libc::c_int {
                        hh
                    } else {
                        hh - 12 as libc::c_int
                    },
                );
            } else if *f as libc::c_int == 'm' as i32 {
                sprintf(
                    buf.as_mut_ptr(),
                    b"%02d\0" as *const u8 as *const libc::c_char,
                    month,
                );
            } else if *f as libc::c_int == 'M' as i32 {
                sprintf(
                    buf.as_mut_ptr(),
                    b"%02d\0" as *const u8 as *const libc::c_char,
                    mm,
                );
            } else if *f as libc::c_int == 'p' as i32 {
                strcpy(
                    buf.as_mut_ptr(),
                    if hh <= 11 as libc::c_int {
                        b"AM\0" as *const u8 as *const libc::c_char
                    } else {
                        b"PM\0" as *const u8 as *const libc::c_char
                    },
                );
            } else if *f as libc::c_int == 'P' as i32 {
                strcpy(
                    buf.as_mut_ptr(),
                    if hh <= 11 as libc::c_int {
                        b"am\0" as *const u8 as *const libc::c_char
                    } else {
                        b"pm\0" as *const u8 as *const libc::c_char
                    },
                );
            } else if *f as libc::c_int == 'r' as i32 {
                sprintf(
                    buf.as_mut_ptr(),
                    b"%02d:%02d:%02d %s\0" as *const u8 as *const libc::c_char,
                    if hh == 0 as libc::c_int {
                        12 as libc::c_int
                    } else if hh <= 12 as libc::c_int {
                        hh
                    } else {
                        hh - 12 as libc::c_int
                    },
                    mm,
                    ss,
                    if hh <= 11 as libc::c_int {
                        b"AM\0" as *const u8 as *const libc::c_char
                    } else {
                        b"PM\0" as *const u8 as *const libc::c_char
                    },
                );
            } else if *f as libc::c_int == 'R' as i32 {
                sprintf(
                    buf.as_mut_ptr(),
                    b"%02d:%02d\0" as *const u8 as *const libc::c_char,
                    hh,
                    mm,
                );
            } else if *f as libc::c_int == 'S' as i32 {
                sprintf(
                    buf.as_mut_ptr(),
                    b"%02d\0" as *const u8 as *const libc::c_char,
                    ss,
                );
            } else if *f as libc::c_int == 'T' as i32 {
                sprintf(
                    buf.as_mut_ptr(),
                    b"%02d:%02d:%02d\0" as *const u8 as *const libc::c_char,
                    hh,
                    mm,
                    ss,
                );
            } else if *f as libc::c_int == 'u' as i32 {
                sprintf(
                    buf.as_mut_ptr(),
                    b"%d\0" as *const u8 as *const libc::c_char,
                    weekday(j),
                );
            } else if *f as libc::c_int == 'U' as i32 {
                let mut sun: libc::c_int = 0;
                sun = _glp_jday(1 as libc::c_int, 1 as libc::c_int, year)
                    - _glp_jday(1 as libc::c_int, 1 as libc::c_int, 1970 as libc::c_int);
                sun += 7 as libc::c_int - weekday(sun);
                sprintf(
                    buf.as_mut_ptr(),
                    b"%02d\0" as *const u8 as *const libc::c_char,
                    (j + 7 as libc::c_int - sun) / 7 as libc::c_int,
                );
            } else if *f as libc::c_int == 'V' as i32 {
                let mut iso_1: libc::c_int = 0;
                if j < firstday(year) {
                    iso_1 = j - firstday(year - 1 as libc::c_int);
                } else if j < firstday(year + 1 as libc::c_int) {
                    iso_1 = j - firstday(year);
                } else {
                    iso_1 = j - firstday(year + 1 as libc::c_int);
                }
                sprintf(
                    buf.as_mut_ptr(),
                    b"%02d\0" as *const u8 as *const libc::c_char,
                    iso_1 / 7 as libc::c_int + 1 as libc::c_int,
                );
            } else if *f as libc::c_int == 'w' as i32 {
                sprintf(
                    buf.as_mut_ptr(),
                    b"%d\0" as *const u8 as *const libc::c_char,
                    weekday(j) % 7 as libc::c_int,
                );
            } else if *f as libc::c_int == 'W' as i32 {
                let mut mon: libc::c_int = 0;
                mon = _glp_jday(1 as libc::c_int, 1 as libc::c_int, year)
                    - _glp_jday(1 as libc::c_int, 1 as libc::c_int, 1970 as libc::c_int);
                mon += (8 as libc::c_int - weekday(mon)) % 7 as libc::c_int;
                sprintf(
                    buf.as_mut_ptr(),
                    b"%02d\0" as *const u8 as *const libc::c_char,
                    (j + 7 as libc::c_int - mon) / 7 as libc::c_int,
                );
            } else if *f as libc::c_int == 'y' as i32 {
                sprintf(
                    buf.as_mut_ptr(),
                    b"%02d\0" as *const u8 as *const libc::c_char,
                    year % 100 as libc::c_int,
                );
            } else if *f as libc::c_int == 'Y' as i32 {
                sprintf(
                    buf.as_mut_ptr(),
                    b"%04d\0" as *const u8 as *const libc::c_char,
                    year,
                );
            } else if *f as libc::c_int == '%' as i32 {
                buf[0 as libc::c_int as usize] = '%' as i32 as libc::c_char;
                buf[1 as libc::c_int as usize] = '\0' as i32 as libc::c_char;
            } else {
                error2(
                    mpl,
                    fmt,
                    f,
                    b"invalid conversion specifier\0" as *const u8 as *const libc::c_char,
                );
            }
        } else {
            buf[0 as libc::c_int as usize] = *f;
            buf[1 as libc::c_int as usize] = '\0' as i32 as libc::c_char;
        }
        if (len as libc::c_ulong).wrapping_add(strlen(buf.as_mut_ptr()))
            > 100 as libc::c_int as libc::c_ulong
        {
            _glp_mpl_error(
                mpl,
                b"time2str; output string length exceeds %d characters\0" as *const u8
                    as *const libc::c_char as *mut libc::c_char,
                100 as libc::c_int,
            );
        }
        memcpy(
            str.offset(len as isize) as *mut libc::c_void,
            buf.as_mut_ptr() as *const libc::c_void,
            strlen(buf.as_mut_ptr()),
        );
        len = (len as libc::c_ulong).wrapping_add(strlen(buf.as_mut_ptr()))
            as libc::c_int as libc::c_int;
        f = f.offset(1);
        f;
    }
    *str.offset(len as isize) = '\0' as i32 as libc::c_char;
}
