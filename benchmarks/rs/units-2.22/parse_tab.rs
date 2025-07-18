use ::libc;
extern "C" {
    pub type _IO_wide_data;
    pub type _IO_codecvt;
    pub type _IO_marker;
    static mut stderr: *mut FILE;
    fn fprintf(_: *mut FILE, _: *const libc::c_char, _: ...) -> libc::c_int;
    fn acos(_: libc::c_double) -> libc::c_double;
    fn asin(_: libc::c_double) -> libc::c_double;
    fn atan(_: libc::c_double) -> libc::c_double;
    fn cos(_: libc::c_double) -> libc::c_double;
    fn sin(_: libc::c_double) -> libc::c_double;
    fn tan(_: libc::c_double) -> libc::c_double;
    fn cosh(_: libc::c_double) -> libc::c_double;
    fn sinh(_: libc::c_double) -> libc::c_double;
    fn tanh(_: libc::c_double) -> libc::c_double;
    fn acosh(_: libc::c_double) -> libc::c_double;
    fn asinh(_: libc::c_double) -> libc::c_double;
    fn atanh(_: libc::c_double) -> libc::c_double;
    fn exp(_: libc::c_double) -> libc::c_double;
    fn log(_: libc::c_double) -> libc::c_double;
    fn log10(_: libc::c_double) -> libc::c_double;
    fn log2(_: libc::c_double) -> libc::c_double;
    fn ceil(_: libc::c_double) -> libc::c_double;
    fn fabs(_: libc::c_double) -> libc::c_double;
    fn floor(_: libc::c_double) -> libc::c_double;
    fn erf(_: libc::c_double) -> libc::c_double;
    fn erfc(_: libc::c_double) -> libc::c_double;
    fn lgamma(_: libc::c_double) -> libc::c_double;
    fn tgamma(_: libc::c_double) -> libc::c_double;
    fn round(_: libc::c_double) -> libc::c_double;
    fn trunc(_: libc::c_double) -> libc::c_double;
    fn __errno_location() -> *mut libc::c_int;
    fn strcmp(_: *const libc::c_char, _: *const libc::c_char) -> libc::c_int;
    fn strncmp(
        _: *const libc::c_char,
        _: *const libc::c_char,
        _: libc::c_ulong,
    ) -> libc::c_int;
    fn strchr(_: *const libc::c_char, _: libc::c_int) -> *mut libc::c_char;
    fn strcspn(_: *const libc::c_char, _: *const libc::c_char) -> libc::c_ulong;
    fn strspn(_: *const libc::c_char, _: *const libc::c_char) -> libc::c_ulong;
    fn strlen(_: *const libc::c_char) -> libc::c_ulong;
    fn strerror(_: libc::c_int) -> *mut libc::c_char;
    fn strtod(_: *const libc::c_char, _: *mut *mut libc::c_char) -> libc::c_double;
    fn strtol(
        _: *const libc::c_char,
        _: *mut *mut libc::c_char,
        _: libc::c_int,
    ) -> libc::c_long;
    fn evalfunc(
        theunit: *mut unittype,
        infunc: *mut func,
        inverse: libc::c_int,
        allerror: libc::c_int,
    ) -> libc::c_int;
    fn fnlookup(str: *const libc::c_char) -> *mut func;
    fn unit2num(input: *mut unittype) -> libc::c_int;
    fn dupnstr(string: *const libc::c_char, length: libc::c_int) -> *mut libc::c_char;
    fn dupstr(str: *const libc::c_char) -> *mut libc::c_char;
    fn unitpower(base: *mut unittype, exponent: *mut unittype) -> libc::c_int;
    fn rootunit(inunit: *mut unittype, n: libc::c_int) -> libc::c_int;
    fn malloc(_: libc::c_ulong) -> *mut libc::c_void;
    fn free(__ptr: *mut libc::c_void);
    static mut errormsg: [*mut libc::c_char; 0];
    static mut parserflags: parseflag;
    static mut parameter_value: *mut unittype;
    static mut function_parameter: *mut libc::c_char;
    static mut lastunitset: libc::c_int;
    static mut lastunit: unittype;
    fn mymalloc(bytes: libc::c_int, mesg: *const libc::c_char) -> *mut libc::c_void;
    fn hassubscript(str: *const libc::c_char) -> libc::c_int;
    fn initializeunit(theunit: *mut unittype);
    fn freeunit(theunit: *mut unittype);
    fn unitcopy(dest: *mut unittype, src: *mut unittype);
    fn divunit(left: *mut unittype, right: *mut unittype) -> libc::c_int;
    fn invertunit(theunit: *mut unittype);
    fn multunit(left: *mut unittype, right: *mut unittype) -> libc::c_int;
    fn addunit(unita: *mut unittype, unitb: *mut unittype) -> libc::c_int;
}
pub type size_t = libc::c_ulong;
pub type __off_t = libc::c_long;
pub type __off64_t = libc::c_long;
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct unittype {
    pub numerator: [*mut libc::c_char; 100],
    pub denominator: [*mut libc::c_char; 100],
    pub factor: libc::c_double,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct functype {
    pub param: *mut libc::c_char,
    pub def: *mut libc::c_char,
    pub dimen: *mut libc::c_char,
    pub domain_min: *mut libc::c_double,
    pub domain_max: *mut libc::c_double,
    pub domain_min_open: libc::c_int,
    pub domain_max_open: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct pair {
    pub location: libc::c_double,
    pub value: libc::c_double,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct func {
    pub name: *mut libc::c_char,
    pub forward: functype,
    pub inverse: functype,
    pub table: *mut pair,
    pub tablelen: libc::c_int,
    pub tableunit: *mut libc::c_char,
    pub next: *mut func,
    pub skip_error_check: libc::c_int,
    pub linenumber: libc::c_int,
    pub file: *mut libc::c_char,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct parseflag {
    pub oldstar: libc::c_int,
    pub minusminus: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct commtype {
    pub location: libc::c_int,
    pub data: *const libc::c_char,
    pub result: *mut unittype,
    pub errorcode: libc::c_int,
}
pub type yy_state_t = yytype_int8;
pub type yytype_int8 = libc::c_schar;
#[derive(Copy, Clone)]
#[repr(C)]
pub union UNITSSTYPE {
    pub number: libc::c_double,
    pub integer: libc::c_int,
    pub unit: *mut unittype,
    pub realfunc: *mut function,
    pub unitfunc: *mut func,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct function {
    pub name: *mut libc::c_char,
    pub func: Option::<unsafe extern "C" fn(libc::c_double) -> libc::c_double>,
    pub type_0: libc::c_int,
}
pub type yy_state_fast_t = libc::c_int;
pub type yytype_int16 = libc::c_short;
pub const UNIT: unitstokentype = 259;
pub const MEMERROR: unitstokentype = 273;
pub const UNITEND: unitstokentype = 275;
pub const UNITFUNC: unitstokentype = 262;
pub const LOG: unitstokentype = 261;
pub const REALFUNC: unitstokentype = 260;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed {
    pub name: *mut libc::c_char,
    pub value: libc::c_int,
}
pub const DIVIDE: unitstokentype = 266;
pub const CUBEROOT: unitstokentype = 269;
pub const SQRT: unitstokentype = 268;
pub const REAL: unitstokentype = 258;
pub const BADNUMBER: unitstokentype = 274;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_0 {
    pub op: libc::c_char,
    pub value: libc::c_int,
}
pub const FUNCINV: unitstokentype = 272;
pub const EXPONENT: unitstokentype = 263;
pub const ADD: unitstokentype = 277;
pub const NUMDIV: unitstokentype = 267;
pub const MULTIPLY: unitstokentype = 264;
pub const LASTUNSET: unitstokentype = 276;
pub const MULTSTAR: unitstokentype = 265;
pub const MULTMINUS: unitstokentype = 270;
pub const MINUS: unitstokentype = 278;
pub const EOL: unitstokentype = 271;
#[derive(Copy, Clone)]
#[repr(C)]
pub union yyalloc {
    pub yyss_alloc: yy_state_t,
    pub yyvs_alloc: UNITSSTYPE,
}
pub type unitstokentype = libc::c_uint;
pub const UNARY: unitstokentype = 279;
#[inline]
unsafe extern "C" fn atoi(mut __nptr: *const libc::c_char) -> libc::c_int {
    return strtol(
        __nptr,
        0 as *mut libc::c_void as *mut *mut libc::c_char,
        10 as libc::c_int,
    ) as libc::c_int;
}
static mut err: libc::c_int = 0;
pub static mut unitcount: libc::c_int = 0 as libc::c_int;
pub unsafe extern "C" fn getnewunit() -> *mut unittype {
    let mut unit: *mut unittype = 0 as *mut unittype;
    if unitcount >= 100 as libc::c_int {
        return 0 as *mut unittype;
    }
    unit = mymalloc(
        ::std::mem::size_of::<unittype>() as libc::c_ulong as libc::c_int,
        b"(getnewunit)\0" as *const u8 as *const libc::c_char,
    ) as *mut unittype;
    if unit.is_null() {
        return 0 as *mut unittype;
    }
    initializeunit(unit);
    unitcount += 1;
    unitcount;
    return unit;
}
pub unsafe extern "C" fn destroyunit(mut unit: *mut unittype) {
    freeunit(unit);
    free(unit as *mut libc::c_void);
    unitcount -= 1;
    unitcount;
}
pub unsafe extern "C" fn makenumunit(
    mut num: libc::c_double,
    mut myerr: *mut libc::c_int,
) -> *mut unittype {
    let mut ret: *mut unittype = 0 as *mut unittype;
    ret = getnewunit();
    if ret.is_null() {
        *myerr = 14 as libc::c_int;
        return 0 as *mut unittype;
    }
    (*ret).factor = num;
    *myerr = 0 as libc::c_int;
    return ret;
}
pub unsafe extern "C" fn logunit(
    mut theunit: *mut unittype,
    mut base: libc::c_int,
) -> libc::c_int {
    err = unit2num(theunit);
    if err != 0 {
        return err;
    }
    if base == 2 as libc::c_int {
        (*theunit).factor = log2((*theunit).factor);
    } else if base == 10 as libc::c_int {
        (*theunit).factor = log10((*theunit).factor);
    } else {
        (*theunit).factor = log((*theunit).factor) / log(base as libc::c_double);
    }
    if *__errno_location() != 0 {
        return 8 as libc::c_int;
    }
    return 0 as libc::c_int;
}
pub unsafe extern "C" fn funcunit(
    mut theunit: *mut unittype,
    mut fun: *const function,
) -> libc::c_int {
    let mut angleunit: unittype = unittype {
        numerator: [0 as *const libc::c_char as *mut libc::c_char; 100],
        denominator: [0 as *const libc::c_char as *mut libc::c_char; 100],
        factor: 0.,
    };
    if (*fun).type_0 == 1 as libc::c_int {
        err = unit2num(theunit);
        if err == 5 as libc::c_int {
            initializeunit(&mut angleunit);
            angleunit
                .denominator[0 as libc::c_int
                as usize] = dupstr(b"radian\0" as *const u8 as *const libc::c_char);
            angleunit.denominator[1 as libc::c_int as usize] = 0 as *mut libc::c_char;
            err = multunit(theunit, &mut angleunit);
            freeunit(&mut angleunit);
            if err == 0 {
                err = unit2num(theunit);
            }
        }
        if err != 0 {
            return err;
        }
    } else if (*fun).type_0 == 2 as libc::c_int || (*fun).type_0 == 0 as libc::c_int
        || (*fun).type_0 == 3 as libc::c_int
    {
        err = unit2num(theunit);
        if err != 0 {
            return err;
        }
        if (*fun).type_0 == 3 as libc::c_int
            && ((*theunit).factor < 0 as libc::c_int as libc::c_double
                || trunc((*theunit).factor) != (*theunit).factor)
        {
            return 11 as libc::c_int;
        }
    } else {
        return 9 as libc::c_int
    }
    *__errno_location() = 0 as libc::c_int;
    (*theunit).factor = (Some(((*fun).func).unwrap())).unwrap()((*theunit).factor);
    if *__errno_location() != 0 {
        return 8 as libc::c_int;
    }
    if (*fun).type_0 == 2 as libc::c_int {
        (*theunit)
            .numerator[0 as libc::c_int
            as usize] = dupstr(b"radian\0" as *const u8 as *const libc::c_char);
        (*theunit).numerator[1 as libc::c_int as usize] = 0 as *mut libc::c_char;
    }
    return 0 as libc::c_int;
}
pub unsafe extern "C" fn factorial(mut x: libc::c_double) -> libc::c_double {
    return tgamma(x + 1 as libc::c_int as libc::c_double);
}
pub static mut realfunctions: [function; 25] = unsafe {
    [
        {
            let mut init = function {
                name: b"sin\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
                func: Some(
                    sin as unsafe extern "C" fn(libc::c_double) -> libc::c_double,
                ),
                type_0: 1 as libc::c_int,
            };
            init
        },
        {
            let mut init = function {
                name: b"cos\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
                func: Some(
                    cos as unsafe extern "C" fn(libc::c_double) -> libc::c_double,
                ),
                type_0: 1 as libc::c_int,
            };
            init
        },
        {
            let mut init = function {
                name: b"tan\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
                func: Some(
                    tan as unsafe extern "C" fn(libc::c_double) -> libc::c_double,
                ),
                type_0: 1 as libc::c_int,
            };
            init
        },
        {
            let mut init = function {
                name: b"ln\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
                func: Some(
                    log as unsafe extern "C" fn(libc::c_double) -> libc::c_double,
                ),
                type_0: 0 as libc::c_int,
            };
            init
        },
        {
            let mut init = function {
                name: b"log\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
                func: Some(
                    log10 as unsafe extern "C" fn(libc::c_double) -> libc::c_double,
                ),
                type_0: 0 as libc::c_int,
            };
            init
        },
        {
            let mut init = function {
                name: b"exp\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
                func: Some(
                    exp as unsafe extern "C" fn(libc::c_double) -> libc::c_double,
                ),
                type_0: 0 as libc::c_int,
            };
            init
        },
        {
            let mut init = function {
                name: b"acos\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
                func: Some(
                    acos as unsafe extern "C" fn(libc::c_double) -> libc::c_double,
                ),
                type_0: 2 as libc::c_int,
            };
            init
        },
        {
            let mut init = function {
                name: b"atan\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
                func: Some(
                    atan as unsafe extern "C" fn(libc::c_double) -> libc::c_double,
                ),
                type_0: 2 as libc::c_int,
            };
            init
        },
        {
            let mut init = function {
                name: b"asin\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
                func: Some(
                    asin as unsafe extern "C" fn(libc::c_double) -> libc::c_double,
                ),
                type_0: 2 as libc::c_int,
            };
            init
        },
        {
            let mut init = function {
                name: b"sinh\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
                func: Some(
                    sinh as unsafe extern "C" fn(libc::c_double) -> libc::c_double,
                ),
                type_0: 0 as libc::c_int,
            };
            init
        },
        {
            let mut init = function {
                name: b"cosh\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
                func: Some(
                    cosh as unsafe extern "C" fn(libc::c_double) -> libc::c_double,
                ),
                type_0: 0 as libc::c_int,
            };
            init
        },
        {
            let mut init = function {
                name: b"tanh\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
                func: Some(
                    tanh as unsafe extern "C" fn(libc::c_double) -> libc::c_double,
                ),
                type_0: 0 as libc::c_int,
            };
            init
        },
        {
            let mut init = function {
                name: b"asinh\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                func: Some(
                    asinh as unsafe extern "C" fn(libc::c_double) -> libc::c_double,
                ),
                type_0: 0 as libc::c_int,
            };
            init
        },
        {
            let mut init = function {
                name: b"acosh\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                func: Some(
                    acosh as unsafe extern "C" fn(libc::c_double) -> libc::c_double,
                ),
                type_0: 0 as libc::c_int,
            };
            init
        },
        {
            let mut init = function {
                name: b"atanh\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                func: Some(
                    atanh as unsafe extern "C" fn(libc::c_double) -> libc::c_double,
                ),
                type_0: 0 as libc::c_int,
            };
            init
        },
        {
            let mut init = function {
                name: b"round\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                func: Some(
                    round as unsafe extern "C" fn(libc::c_double) -> libc::c_double,
                ),
                type_0: 0 as libc::c_int,
            };
            init
        },
        {
            let mut init = function {
                name: b"floor\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                func: Some(
                    floor as unsafe extern "C" fn(libc::c_double) -> libc::c_double,
                ),
                type_0: 0 as libc::c_int,
            };
            init
        },
        {
            let mut init = function {
                name: b"ceil\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
                func: Some(
                    ceil as unsafe extern "C" fn(libc::c_double) -> libc::c_double,
                ),
                type_0: 0 as libc::c_int,
            };
            init
        },
        {
            let mut init = function {
                name: b"abs\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
                func: Some(
                    fabs as unsafe extern "C" fn(libc::c_double) -> libc::c_double,
                ),
                type_0: 0 as libc::c_int,
            };
            init
        },
        {
            let mut init = function {
                name: b"erf\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
                func: Some(
                    erf as unsafe extern "C" fn(libc::c_double) -> libc::c_double,
                ),
                type_0: 0 as libc::c_int,
            };
            init
        },
        {
            let mut init = function {
                name: b"erfc\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
                func: Some(
                    erfc as unsafe extern "C" fn(libc::c_double) -> libc::c_double,
                ),
                type_0: 0 as libc::c_int,
            };
            init
        },
        {
            let mut init = function {
                name: b"Gamma\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                func: Some(
                    tgamma as unsafe extern "C" fn(libc::c_double) -> libc::c_double,
                ),
                type_0: 0 as libc::c_int,
            };
            init
        },
        {
            let mut init = function {
                name: b"lnGamma\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                func: Some(
                    lgamma as unsafe extern "C" fn(libc::c_double) -> libc::c_double,
                ),
                type_0: 0 as libc::c_int,
            };
            init
        },
        {
            let mut init = function {
                name: b"factorial\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                func: Some(
                    factorial as unsafe extern "C" fn(libc::c_double) -> libc::c_double,
                ),
                type_0: 3 as libc::c_int,
            };
            init
        },
        {
            let mut init = function {
                name: 0 as *const libc::c_char as *mut libc::c_char,
                func: None,
                type_0: 0 as libc::c_int,
            };
            init
        },
    ]
};
pub static mut optable: [C2RustUnnamed_0; 9] = [
    {
        let mut init = C2RustUnnamed_0 {
            op: '*' as i32 as libc::c_char,
            value: MULTIPLY as libc::c_int,
        };
        init
    },
    {
        let mut init = C2RustUnnamed_0 {
            op: '/' as i32 as libc::c_char,
            value: DIVIDE as libc::c_int,
        };
        init
    },
    {
        let mut init = C2RustUnnamed_0 {
            op: '|' as i32 as libc::c_char,
            value: NUMDIV as libc::c_int,
        };
        init
    },
    {
        let mut init = C2RustUnnamed_0 {
            op: '+' as i32 as libc::c_char,
            value: ADD as libc::c_int,
        };
        init
    },
    {
        let mut init = C2RustUnnamed_0 {
            op: '(' as i32 as libc::c_char,
            value: '(' as i32,
        };
        init
    },
    {
        let mut init = C2RustUnnamed_0 {
            op: ')' as i32 as libc::c_char,
            value: ')' as i32,
        };
        init
    },
    {
        let mut init = C2RustUnnamed_0 {
            op: '^' as i32 as libc::c_char,
            value: EXPONENT as libc::c_int,
        };
        init
    },
    {
        let mut init = C2RustUnnamed_0 {
            op: '~' as i32 as libc::c_char,
            value: FUNCINV as libc::c_int,
        };
        init
    },
    {
        let mut init = C2RustUnnamed_0 {
            op: 0 as libc::c_int as libc::c_char,
            value: 0 as libc::c_int,
        };
        init
    },
];
pub static mut strtable: [C2RustUnnamed; 4] = [
    {
        let mut init = C2RustUnnamed {
            name: b"sqrt\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            value: SQRT as libc::c_int,
        };
        init
    },
    {
        let mut init = C2RustUnnamed {
            name: b"cuberoot\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            value: CUBEROOT as libc::c_int,
        };
        init
    },
    {
        let mut init = C2RustUnnamed {
            name: b"per\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            value: DIVIDE as libc::c_int,
        };
        init
    },
    {
        let mut init = C2RustUnnamed {
            name: 0 as *const libc::c_char as *mut libc::c_char,
            value: 0 as libc::c_int,
        };
        init
    },
];
pub unsafe extern "C" fn unitslex(
    mut lvalp: *mut UNITSSTYPE,
    mut comm: *mut commtype,
) -> libc::c_int {
    let mut length: libc::c_int = 0;
    let mut count: libc::c_int = 0;
    let mut output: *mut unittype = 0 as *mut unittype;
    let mut inptr: *const libc::c_char = 0 as *const libc::c_char;
    let mut name: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut nonunitchars: *mut libc::c_char = b"~;+-*/|\t\n^ ()\0" as *const u8
        as *const libc::c_char as *mut libc::c_char;
    let mut nonunitends: *mut libc::c_char = b".,_\0" as *const u8 as *const libc::c_char
        as *mut libc::c_char;
    let mut number_start: *mut libc::c_char = b".,0123456789\0" as *const u8
        as *const libc::c_char as *mut libc::c_char;
    if (*comm).location == -(1 as libc::c_int) {
        return 0 as libc::c_int;
    }
    inptr = ((*comm).data).offset((*comm).location as isize);
    while *inptr as libc::c_int == ' ' as i32 {
        inptr = inptr.offset(1);
        inptr;
        (*comm).location += 1;
        (*comm).location;
    }
    if *inptr as libc::c_int == 0 as libc::c_int {
        (*comm).location = -(1 as libc::c_int);
        return EOL as libc::c_int;
    }
    if 0 as libc::c_int
        == strncmp(
            b"**\0" as *const u8 as *const libc::c_char,
            inptr,
            2 as libc::c_int as libc::c_ulong,
        )
    {
        (*comm).location += 2 as libc::c_int;
        return EXPONENT as libc::c_int;
    }
    if *inptr as libc::c_int == '-' as i32 {
        (*comm).location += 1;
        (*comm).location;
        if parserflags.minusminus != 0 {
            return MINUS as libc::c_int;
        }
        return MULTMINUS as libc::c_int;
    }
    if *inptr as libc::c_int == '*' as i32 {
        (*comm).location += 1;
        (*comm).location;
        if parserflags.oldstar != 0 {
            return MULTIPLY as libc::c_int;
        }
        return MULTSTAR as libc::c_int;
    }
    if *inptr as libc::c_int == '_' as i32 {
        (*comm).location += 1;
        (*comm).location;
        if lastunitset == 0 {
            return LASTUNSET as libc::c_int;
        }
        output = getnewunit();
        if output.is_null() {
            return MEMERROR as libc::c_int;
        }
        unitcopy(output, &mut lastunit);
        (*lvalp).unit = output;
        return UNIT as libc::c_int;
    }
    count = 0 as libc::c_int;
    while optable[count as usize].op != 0 {
        if *inptr as libc::c_int == optable[count as usize].op as libc::c_int {
            (*comm).location += 1;
            (*comm).location;
            return optable[count as usize].value;
        }
        count += 1;
        count;
    }
    if !(strchr(number_start, *inptr as libc::c_int)).is_null() {
        let mut endloc: *mut libc::c_char = 0 as *mut libc::c_char;
        (*lvalp).number = strtod(inptr, &mut endloc);
        if inptr != endloc as *const libc::c_char {
            (*comm)
                .location = ((*comm).location as libc::c_long
                + endloc.offset_from(inptr) as libc::c_long) as libc::c_int;
            if *endloc as libc::c_int != 0
                && !(strchr(number_start, *endloc as libc::c_int)).is_null()
            {
                return BADNUMBER as libc::c_int
            } else {
                return REAL as libc::c_int
            }
        }
    }
    length = strcspn(inptr, nonunitchars) as libc::c_int;
    if length == 0 {
        (*comm).location += 1;
        (*comm).location;
        return 0 as libc::c_int;
    }
    if !(strchr(nonunitends, *inptr as libc::c_int)).is_null() {
        (*comm).location += 1;
        (*comm).location;
        return 0 as libc::c_int;
    }
    if !(strchr(
        nonunitends,
        *inptr.offset((length - 1 as libc::c_int) as isize) as libc::c_int,
    ))
        .is_null()
    {
        (*comm).location += length;
        return 0 as libc::c_int;
    }
    name = dupnstr(inptr, length);
    count = 0 as libc::c_int;
    while !(strtable[count as usize].name).is_null() {
        if strcmp(name, strtable[count as usize].name) == 0 {
            free(name as *mut libc::c_void);
            (*comm).location += length;
            return strtable[count as usize].value;
        }
        count += 1;
        count;
    }
    count = 0 as libc::c_int;
    while !(realfunctions[count as usize].name).is_null() {
        if strcmp(name, realfunctions[count as usize].name) == 0 {
            (*lvalp).realfunc = realfunctions.as_mut_ptr().offset(count as isize);
            (*comm).location += length;
            free(name as *mut libc::c_void);
            return REALFUNC as libc::c_int;
        }
        count += 1;
        count;
    }
    if strncmp(
        name,
        b"log\0" as *const u8 as *const libc::c_char,
        3 as libc::c_int as libc::c_ulong,
    ) == 0
    {
        count = strspn(
            name.offset(3 as libc::c_int as isize),
            b"1234567890\0" as *const u8 as *const libc::c_char,
        ) as libc::c_int;
        if (count + 3 as libc::c_int) as libc::c_ulong == strlen(name) {
            (*lvalp).integer = atoi(name.offset(3 as libc::c_int as isize));
            if (*lvalp).integer > 1 as libc::c_int {
                (*comm).location += length;
                free(name as *mut libc::c_void);
                return LOG as libc::c_int;
            }
        }
    }
    if !function_parameter.is_null() && strcmp(name, function_parameter) == 0 {
        free(name as *mut libc::c_void);
        output = getnewunit();
        if output.is_null() {
            return MEMERROR as libc::c_int;
        }
        unitcopy(output, parameter_value);
        (*lvalp).unit = output;
        (*comm).location += length;
        return UNIT as libc::c_int;
    }
    (*lvalp).unitfunc = fnlookup(name);
    if !((*lvalp).unitfunc).is_null() {
        (*comm).location += length;
        free(name as *mut libc::c_void);
        return UNITFUNC as libc::c_int;
    }
    (*comm).location += length;
    if !(strchr(
        b"23456789\0" as *const u8 as *const libc::c_char,
        *inptr.offset((length - 1 as libc::c_int) as isize) as libc::c_int,
    ))
        .is_null() && hassubscript(name) == 0
    {
        count = *name.offset((length - 1 as libc::c_int) as isize) as libc::c_int
            - '0' as i32;
        length -= 1;
        length;
        if !(strchr(
            number_start,
            *name.offset((length - 1 as libc::c_int) as isize) as libc::c_int,
        ))
            .is_null()
        {
            free(name as *mut libc::c_void);
            return UNITEND as libc::c_int;
        }
    } else {
        count = 1 as libc::c_int;
    }
    free(name as *mut libc::c_void);
    output = getnewunit();
    if output.is_null() {
        return MEMERROR as libc::c_int;
    }
    let fresh0 = count;
    count = count - 1;
    (*output).numerator[fresh0 as usize] = 0 as *mut libc::c_char;
    while count >= 0 as libc::c_int {
        (*output).numerator[count as usize] = dupnstr(inptr, length);
        count -= 1;
        count;
    }
    (*lvalp).unit = output;
    return UNIT as libc::c_int;
}
pub unsafe extern "C" fn unitserror(mut comm: *mut commtype, mut s: *mut libc::c_char) {}
pub unsafe extern "C" fn parseunit(
    mut output: *mut unittype,
    mut input: *const libc::c_char,
    mut errstr: *mut *mut libc::c_char,
    mut errloc: *mut libc::c_int,
) -> libc::c_int {
    let mut comm: commtype = commtype {
        location: 0,
        data: 0 as *const libc::c_char,
        result: 0 as *mut unittype,
        errorcode: 0,
    };
    let mut saveunitcount: libc::c_int = 0;
    saveunitcount = unitcount;
    initializeunit(output);
    comm.result = 0 as *mut unittype;
    comm.location = 0 as libc::c_int;
    comm.data = input;
    comm.errorcode = 1 as libc::c_int;
    *__errno_location() = 0 as libc::c_int;
    if unitsparse(&mut comm) != 0 || *__errno_location() != 0 {
        if comm.location == -(1 as libc::c_int) {
            comm.location = strlen(input) as libc::c_int;
        }
        if !errstr.is_null() {
            if comm.errorcode == 8 as libc::c_int || *__errno_location() != 0 {
                *errstr = strerror(*__errno_location());
            } else {
                *errstr = *errormsg.as_mut_ptr().offset(comm.errorcode as isize);
            }
        }
        if !errloc.is_null() {
            *errloc = comm.location;
        }
        if unitcount != saveunitcount {
            fprintf(
                stderr,
                b"units: Parser leaked memory with error: %d in %d out\n\0" as *const u8
                    as *const libc::c_char,
                saveunitcount,
                unitcount,
            );
        }
        return comm.errorcode;
    } else {
        if !errstr.is_null() {
            *errstr = 0 as *mut libc::c_char;
        }
        multunit(output, comm.result);
        destroyunit(comm.result);
        if unitcount != saveunitcount {
            fprintf(
                stderr,
                b"units: Parser leaked memory without error: %d in %d out\n\0"
                    as *const u8 as *const libc::c_char,
                saveunitcount,
                unitcount,
            );
        }
        return 0 as libc::c_int;
    };
}
static mut yytranslate: [yytype_int8; 280] = [
    0 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    25 as libc::c_int as yytype_int8,
    26 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    1 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    3 as libc::c_int as yytype_int8,
    4 as libc::c_int as yytype_int8,
    5 as libc::c_int as yytype_int8,
    6 as libc::c_int as yytype_int8,
    7 as libc::c_int as yytype_int8,
    8 as libc::c_int as yytype_int8,
    9 as libc::c_int as yytype_int8,
    10 as libc::c_int as yytype_int8,
    11 as libc::c_int as yytype_int8,
    12 as libc::c_int as yytype_int8,
    13 as libc::c_int as yytype_int8,
    14 as libc::c_int as yytype_int8,
    15 as libc::c_int as yytype_int8,
    16 as libc::c_int as yytype_int8,
    17 as libc::c_int as yytype_int8,
    18 as libc::c_int as yytype_int8,
    19 as libc::c_int as yytype_int8,
    20 as libc::c_int as yytype_int8,
    21 as libc::c_int as yytype_int8,
    22 as libc::c_int as yytype_int8,
    23 as libc::c_int as yytype_int8,
    24 as libc::c_int as yytype_int8,
];
static mut yypact: [yytype_int16; 61] = [
    8 as libc::c_int as yytype_int16,
    -(22 as libc::c_int) as yytype_int16,
    -(22 as libc::c_int) as yytype_int16,
    -(22 as libc::c_int) as yytype_int16,
    -(21 as libc::c_int) as yytype_int16,
    -(21 as libc::c_int) as yytype_int16,
    -(21 as libc::c_int) as yytype_int16,
    151 as libc::c_int as yytype_int16,
    -(21 as libc::c_int) as yytype_int16,
    -(21 as libc::c_int) as yytype_int16,
    151 as libc::c_int as yytype_int16,
    -(22 as libc::c_int) as yytype_int16,
    39 as libc::c_int as yytype_int16,
    -(22 as libc::c_int) as yytype_int16,
    -(22 as libc::c_int) as yytype_int16,
    -(22 as libc::c_int) as yytype_int16,
    -(22 as libc::c_int) as yytype_int16,
    151 as libc::c_int as yytype_int16,
    59 as libc::c_int as yytype_int16,
    16 as libc::c_int as yytype_int16,
    -(8 as libc::c_int) as yytype_int16,
    83 as libc::c_int as yytype_int16,
    20 as libc::c_int as yytype_int16,
    -(22 as libc::c_int) as yytype_int16,
    82 as libc::c_int as yytype_int16,
    -(22 as libc::c_int) as yytype_int16,
    -(22 as libc::c_int) as yytype_int16,
    -(22 as libc::c_int) as yytype_int16,
    82 as libc::c_int as yytype_int16,
    -(22 as libc::c_int) as yytype_int16,
    -(22 as libc::c_int) as yytype_int16,
    82 as libc::c_int as yytype_int16,
    -(22 as libc::c_int) as yytype_int16,
    -(21 as libc::c_int) as yytype_int16,
    82 as libc::c_int as yytype_int16,
    25 as libc::c_int as yytype_int16,
    -(22 as libc::c_int) as yytype_int16,
    -(22 as libc::c_int) as yytype_int16,
    59 as libc::c_int as yytype_int16,
    59 as libc::c_int as yytype_int16,
    59 as libc::c_int as yytype_int16,
    59 as libc::c_int as yytype_int16,
    59 as libc::c_int as yytype_int16,
    17 as libc::c_int as yytype_int16,
    105 as libc::c_int as yytype_int16,
    151 as libc::c_int as yytype_int16,
    128 as libc::c_int as yytype_int16,
    -(22 as libc::c_int) as yytype_int16,
    -(22 as libc::c_int) as yytype_int16,
    -(22 as libc::c_int) as yytype_int16,
    40 as libc::c_int as yytype_int16,
    40 as libc::c_int as yytype_int16,
    43 as libc::c_int as yytype_int16,
    43 as libc::c_int as yytype_int16,
    -(22 as libc::c_int) as yytype_int16,
    151 as libc::c_int as yytype_int16,
    151 as libc::c_int as yytype_int16,
    42 as libc::c_int as yytype_int16,
    128 as libc::c_int as yytype_int16,
    42 as libc::c_int as yytype_int16,
    42 as libc::c_int as yytype_int16,
];
static mut yydefact: [yytype_int8; 61] = [
    0 as libc::c_int as yytype_int8,
    4 as libc::c_int as yytype_int8,
    15 as libc::c_int as yytype_int8,
    19 as libc::c_int as yytype_int8,
    0 as libc::c_int as yytype_int8,
    0 as libc::c_int as yytype_int8,
    0 as libc::c_int as yytype_int8,
    0 as libc::c_int as yytype_int8,
    0 as libc::c_int as yytype_int8,
    0 as libc::c_int as yytype_int8,
    0 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    0 as libc::c_int as yytype_int8,
    33 as libc::c_int as yytype_int8,
    32 as libc::c_int as yytype_int8,
    34 as libc::c_int as yytype_int8,
    35 as libc::c_int as yytype_int8,
    0 as libc::c_int as yytype_int8,
    0 as libc::c_int as yytype_int8,
    0 as libc::c_int as yytype_int8,
    0 as libc::c_int as yytype_int8,
    5 as libc::c_int as yytype_int8,
    18 as libc::c_int as yytype_int8,
    23 as libc::c_int as yytype_int8,
    7 as libc::c_int as yytype_int8,
    26 as libc::c_int as yytype_int8,
    27 as libc::c_int as yytype_int8,
    28 as libc::c_int as yytype_int8,
    6 as libc::c_int as yytype_int8,
    24 as libc::c_int as yytype_int8,
    25 as libc::c_int as yytype_int8,
    8 as libc::c_int as yytype_int8,
    36 as libc::c_int as yytype_int8,
    0 as libc::c_int as yytype_int8,
    9 as libc::c_int as yytype_int8,
    0 as libc::c_int as yytype_int8,
    1 as libc::c_int as yytype_int8,
    3 as libc::c_int as yytype_int8,
    0 as libc::c_int as yytype_int8,
    0 as libc::c_int as yytype_int8,
    0 as libc::c_int as yytype_int8,
    0 as libc::c_int as yytype_int8,
    0 as libc::c_int as yytype_int8,
    0 as libc::c_int as yytype_int8,
    0 as libc::c_int as yytype_int8,
    0 as libc::c_int as yytype_int8,
    22 as libc::c_int as yytype_int8,
    29 as libc::c_int as yytype_int8,
    17 as libc::c_int as yytype_int8,
    13 as libc::c_int as yytype_int8,
    14 as libc::c_int as yytype_int8,
    12 as libc::c_int as yytype_int8,
    10 as libc::c_int as yytype_int8,
    11 as libc::c_int as yytype_int8,
    16 as libc::c_int as yytype_int8,
    0 as libc::c_int as yytype_int8,
    0 as libc::c_int as yytype_int8,
    20 as libc::c_int as yytype_int8,
    21 as libc::c_int as yytype_int8,
    30 as libc::c_int as yytype_int8,
    31 as libc::c_int as yytype_int8,
];
static mut yypgoto: [yytype_int8; 7] = [
    -(22 as libc::c_int) as yytype_int8,
    -(22 as libc::c_int) as yytype_int8,
    -(22 as libc::c_int) as yytype_int8,
    19 as libc::c_int as yytype_int8,
    24 as libc::c_int as yytype_int8,
    -(3 as libc::c_int) as yytype_int8,
    0 as libc::c_int as yytype_int8,
];
static mut yydefgoto: [yytype_int8; 7] = [
    -(1 as libc::c_int) as yytype_int8,
    19 as libc::c_int as yytype_int8,
    20 as libc::c_int as yytype_int8,
    21 as libc::c_int as yytype_int8,
    22 as libc::c_int as yytype_int8,
    23 as libc::c_int as yytype_int8,
    46 as libc::c_int as yytype_int8,
];
static mut yytable: [yytype_int8; 177] = [
    24 as libc::c_int as yytype_int8,
    25 as libc::c_int as yytype_int8,
    26 as libc::c_int as yytype_int8,
    27 as libc::c_int as yytype_int8,
    18 as libc::c_int as yytype_int8,
    29 as libc::c_int as yytype_int8,
    30 as libc::c_int as yytype_int8,
    28 as libc::c_int as yytype_int8,
    37 as libc::c_int as yytype_int8,
    1 as libc::c_int as yytype_int8,
    31 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    3 as libc::c_int as yytype_int8,
    4 as libc::c_int as yytype_int8,
    5 as libc::c_int as yytype_int8,
    6 as libc::c_int as yytype_int8,
    36 as libc::c_int as yytype_int8,
    34 as libc::c_int as yytype_int8,
    24 as libc::c_int as yytype_int8,
    7 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    8 as libc::c_int as yytype_int8,
    9 as libc::c_int as yytype_int8,
    10 as libc::c_int as yytype_int8,
    11 as libc::c_int as yytype_int8,
    12 as libc::c_int as yytype_int8,
    13 as libc::c_int as yytype_int8,
    14 as libc::c_int as yytype_int8,
    15 as libc::c_int as yytype_int8,
    16 as libc::c_int as yytype_int8,
    47 as libc::c_int as yytype_int8,
    17 as libc::c_int as yytype_int8,
    43 as libc::c_int as yytype_int8,
    18 as libc::c_int as yytype_int8,
    38 as libc::c_int as yytype_int8,
    39 as libc::c_int as yytype_int8,
    40 as libc::c_int as yytype_int8,
    35 as libc::c_int as yytype_int8,
    24 as libc::c_int as yytype_int8,
    24 as libc::c_int as yytype_int8,
    24 as libc::c_int as yytype_int8,
    24 as libc::c_int as yytype_int8,
    24 as libc::c_int as yytype_int8,
    32 as libc::c_int as yytype_int8,
    57 as libc::c_int as yytype_int8,
    58 as libc::c_int as yytype_int8,
    33 as libc::c_int as yytype_int8,
    41 as libc::c_int as yytype_int8,
    42 as libc::c_int as yytype_int8,
    38 as libc::c_int as yytype_int8,
    44 as libc::c_int as yytype_int8,
    48 as libc::c_int as yytype_int8,
    38 as libc::c_int as yytype_int8,
    39 as libc::c_int as yytype_int8,
    40 as libc::c_int as yytype_int8,
    59 as libc::c_int as yytype_int8,
    60 as libc::c_int as yytype_int8,
    49 as libc::c_int as yytype_int8,
    50 as libc::c_int as yytype_int8,
    51 as libc::c_int as yytype_int8,
    52 as libc::c_int as yytype_int8,
    53 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    3 as libc::c_int as yytype_int8,
    4 as libc::c_int as yytype_int8,
    5 as libc::c_int as yytype_int8,
    6 as libc::c_int as yytype_int8,
    54 as libc::c_int as yytype_int8,
    0 as libc::c_int as yytype_int8,
    0 as libc::c_int as yytype_int8,
    0 as libc::c_int as yytype_int8,
    0 as libc::c_int as yytype_int8,
    8 as libc::c_int as yytype_int8,
    9 as libc::c_int as yytype_int8,
    10 as libc::c_int as yytype_int8,
    0 as libc::c_int as yytype_int8,
    12 as libc::c_int as yytype_int8,
    13 as libc::c_int as yytype_int8,
    14 as libc::c_int as yytype_int8,
    15 as libc::c_int as yytype_int8,
    16 as libc::c_int as yytype_int8,
    0 as libc::c_int as yytype_int8,
    17 as libc::c_int as yytype_int8,
    0 as libc::c_int as yytype_int8,
    18 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    3 as libc::c_int as yytype_int8,
    4 as libc::c_int as yytype_int8,
    5 as libc::c_int as yytype_int8,
    6 as libc::c_int as yytype_int8,
    44 as libc::c_int as yytype_int8,
    0 as libc::c_int as yytype_int8,
    38 as libc::c_int as yytype_int8,
    39 as libc::c_int as yytype_int8,
    40 as libc::c_int as yytype_int8,
    8 as libc::c_int as yytype_int8,
    9 as libc::c_int as yytype_int8,
    45 as libc::c_int as yytype_int8,
    0 as libc::c_int as yytype_int8,
    12 as libc::c_int as yytype_int8,
    13 as libc::c_int as yytype_int8,
    14 as libc::c_int as yytype_int8,
    15 as libc::c_int as yytype_int8,
    16 as libc::c_int as yytype_int8,
    0 as libc::c_int as yytype_int8,
    41 as libc::c_int as yytype_int8,
    42 as libc::c_int as yytype_int8,
    18 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    3 as libc::c_int as yytype_int8,
    4 as libc::c_int as yytype_int8,
    5 as libc::c_int as yytype_int8,
    6 as libc::c_int as yytype_int8,
    0 as libc::c_int as yytype_int8,
    0 as libc::c_int as yytype_int8,
    0 as libc::c_int as yytype_int8,
    0 as libc::c_int as yytype_int8,
    0 as libc::c_int as yytype_int8,
    8 as libc::c_int as yytype_int8,
    9 as libc::c_int as yytype_int8,
    55 as libc::c_int as yytype_int8,
    0 as libc::c_int as yytype_int8,
    12 as libc::c_int as yytype_int8,
    13 as libc::c_int as yytype_int8,
    14 as libc::c_int as yytype_int8,
    15 as libc::c_int as yytype_int8,
    16 as libc::c_int as yytype_int8,
    0 as libc::c_int as yytype_int8,
    56 as libc::c_int as yytype_int8,
    0 as libc::c_int as yytype_int8,
    18 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    3 as libc::c_int as yytype_int8,
    4 as libc::c_int as yytype_int8,
    5 as libc::c_int as yytype_int8,
    6 as libc::c_int as yytype_int8,
    44 as libc::c_int as yytype_int8,
    0 as libc::c_int as yytype_int8,
    0 as libc::c_int as yytype_int8,
    0 as libc::c_int as yytype_int8,
    0 as libc::c_int as yytype_int8,
    8 as libc::c_int as yytype_int8,
    9 as libc::c_int as yytype_int8,
    0 as libc::c_int as yytype_int8,
    0 as libc::c_int as yytype_int8,
    12 as libc::c_int as yytype_int8,
    13 as libc::c_int as yytype_int8,
    14 as libc::c_int as yytype_int8,
    15 as libc::c_int as yytype_int8,
    16 as libc::c_int as yytype_int8,
    0 as libc::c_int as yytype_int8,
    0 as libc::c_int as yytype_int8,
    0 as libc::c_int as yytype_int8,
    18 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    3 as libc::c_int as yytype_int8,
    4 as libc::c_int as yytype_int8,
    5 as libc::c_int as yytype_int8,
    6 as libc::c_int as yytype_int8,
    0 as libc::c_int as yytype_int8,
    0 as libc::c_int as yytype_int8,
    0 as libc::c_int as yytype_int8,
    0 as libc::c_int as yytype_int8,
    0 as libc::c_int as yytype_int8,
    8 as libc::c_int as yytype_int8,
    9 as libc::c_int as yytype_int8,
    0 as libc::c_int as yytype_int8,
    0 as libc::c_int as yytype_int8,
    12 as libc::c_int as yytype_int8,
    13 as libc::c_int as yytype_int8,
    14 as libc::c_int as yytype_int8,
    15 as libc::c_int as yytype_int8,
    16 as libc::c_int as yytype_int8,
    0 as libc::c_int as yytype_int8,
    0 as libc::c_int as yytype_int8,
    0 as libc::c_int as yytype_int8,
    18 as libc::c_int as yytype_int8,
];
static mut yycheck: [yytype_int8; 177] = [
    0 as libc::c_int as yytype_int8,
    4 as libc::c_int as yytype_int8,
    5 as libc::c_int as yytype_int8,
    6 as libc::c_int as yytype_int8,
    25 as libc::c_int as yytype_int8,
    8 as libc::c_int as yytype_int8,
    9 as libc::c_int as yytype_int8,
    7 as libc::c_int as yytype_int8,
    16 as libc::c_int as yytype_int8,
    1 as libc::c_int as yytype_int8,
    10 as libc::c_int as yytype_int8,
    3 as libc::c_int as yytype_int8,
    4 as libc::c_int as yytype_int8,
    5 as libc::c_int as yytype_int8,
    6 as libc::c_int as yytype_int8,
    7 as libc::c_int as yytype_int8,
    0 as libc::c_int as yytype_int8,
    17 as libc::c_int as yytype_int8,
    18 as libc::c_int as yytype_int8,
    11 as libc::c_int as yytype_int8,
    3 as libc::c_int as yytype_int8,
    13 as libc::c_int as yytype_int8,
    14 as libc::c_int as yytype_int8,
    15 as libc::c_int as yytype_int8,
    16 as libc::c_int as yytype_int8,
    17 as libc::c_int as yytype_int8,
    18 as libc::c_int as yytype_int8,
    19 as libc::c_int as yytype_int8,
    20 as libc::c_int as yytype_int8,
    21 as libc::c_int as yytype_int8,
    33 as libc::c_int as yytype_int8,
    23 as libc::c_int as yytype_int8,
    12 as libc::c_int as yytype_int8,
    25 as libc::c_int as yytype_int8,
    9 as libc::c_int as yytype_int8,
    10 as libc::c_int as yytype_int8,
    11 as libc::c_int as yytype_int8,
    18 as libc::c_int as yytype_int8,
    38 as libc::c_int as yytype_int8,
    39 as libc::c_int as yytype_int8,
    40 as libc::c_int as yytype_int8,
    41 as libc::c_int as yytype_int8,
    42 as libc::c_int as yytype_int8,
    4 as libc::c_int as yytype_int8,
    44 as libc::c_int as yytype_int8,
    45 as libc::c_int as yytype_int8,
    7 as libc::c_int as yytype_int8,
    22 as libc::c_int as yytype_int8,
    23 as libc::c_int as yytype_int8,
    9 as libc::c_int as yytype_int8,
    8 as libc::c_int as yytype_int8,
    26 as libc::c_int as yytype_int8,
    9 as libc::c_int as yytype_int8,
    10 as libc::c_int as yytype_int8,
    11 as libc::c_int as yytype_int8,
    55 as libc::c_int as yytype_int8,
    56 as libc::c_int as yytype_int8,
    38 as libc::c_int as yytype_int8,
    39 as libc::c_int as yytype_int8,
    40 as libc::c_int as yytype_int8,
    41 as libc::c_int as yytype_int8,
    42 as libc::c_int as yytype_int8,
    3 as libc::c_int as yytype_int8,
    4 as libc::c_int as yytype_int8,
    5 as libc::c_int as yytype_int8,
    6 as libc::c_int as yytype_int8,
    7 as libc::c_int as yytype_int8,
    43 as libc::c_int as yytype_int8,
    -(1 as libc::c_int) as yytype_int8,
    -(1 as libc::c_int) as yytype_int8,
    -(1 as libc::c_int) as yytype_int8,
    -(1 as libc::c_int) as yytype_int8,
    13 as libc::c_int as yytype_int8,
    14 as libc::c_int as yytype_int8,
    15 as libc::c_int as yytype_int8,
    -(1 as libc::c_int) as yytype_int8,
    17 as libc::c_int as yytype_int8,
    18 as libc::c_int as yytype_int8,
    19 as libc::c_int as yytype_int8,
    20 as libc::c_int as yytype_int8,
    21 as libc::c_int as yytype_int8,
    -(1 as libc::c_int) as yytype_int8,
    23 as libc::c_int as yytype_int8,
    -(1 as libc::c_int) as yytype_int8,
    25 as libc::c_int as yytype_int8,
    3 as libc::c_int as yytype_int8,
    4 as libc::c_int as yytype_int8,
    5 as libc::c_int as yytype_int8,
    6 as libc::c_int as yytype_int8,
    7 as libc::c_int as yytype_int8,
    8 as libc::c_int as yytype_int8,
    -(1 as libc::c_int) as yytype_int8,
    9 as libc::c_int as yytype_int8,
    10 as libc::c_int as yytype_int8,
    11 as libc::c_int as yytype_int8,
    13 as libc::c_int as yytype_int8,
    14 as libc::c_int as yytype_int8,
    15 as libc::c_int as yytype_int8,
    -(1 as libc::c_int) as yytype_int8,
    17 as libc::c_int as yytype_int8,
    18 as libc::c_int as yytype_int8,
    19 as libc::c_int as yytype_int8,
    20 as libc::c_int as yytype_int8,
    21 as libc::c_int as yytype_int8,
    -(1 as libc::c_int) as yytype_int8,
    22 as libc::c_int as yytype_int8,
    23 as libc::c_int as yytype_int8,
    25 as libc::c_int as yytype_int8,
    3 as libc::c_int as yytype_int8,
    4 as libc::c_int as yytype_int8,
    5 as libc::c_int as yytype_int8,
    6 as libc::c_int as yytype_int8,
    7 as libc::c_int as yytype_int8,
    -(1 as libc::c_int) as yytype_int8,
    -(1 as libc::c_int) as yytype_int8,
    -(1 as libc::c_int) as yytype_int8,
    -(1 as libc::c_int) as yytype_int8,
    -(1 as libc::c_int) as yytype_int8,
    13 as libc::c_int as yytype_int8,
    14 as libc::c_int as yytype_int8,
    15 as libc::c_int as yytype_int8,
    -(1 as libc::c_int) as yytype_int8,
    17 as libc::c_int as yytype_int8,
    18 as libc::c_int as yytype_int8,
    19 as libc::c_int as yytype_int8,
    20 as libc::c_int as yytype_int8,
    21 as libc::c_int as yytype_int8,
    -(1 as libc::c_int) as yytype_int8,
    23 as libc::c_int as yytype_int8,
    -(1 as libc::c_int) as yytype_int8,
    25 as libc::c_int as yytype_int8,
    3 as libc::c_int as yytype_int8,
    4 as libc::c_int as yytype_int8,
    5 as libc::c_int as yytype_int8,
    6 as libc::c_int as yytype_int8,
    7 as libc::c_int as yytype_int8,
    8 as libc::c_int as yytype_int8,
    -(1 as libc::c_int) as yytype_int8,
    -(1 as libc::c_int) as yytype_int8,
    -(1 as libc::c_int) as yytype_int8,
    -(1 as libc::c_int) as yytype_int8,
    13 as libc::c_int as yytype_int8,
    14 as libc::c_int as yytype_int8,
    -(1 as libc::c_int) as yytype_int8,
    -(1 as libc::c_int) as yytype_int8,
    17 as libc::c_int as yytype_int8,
    18 as libc::c_int as yytype_int8,
    19 as libc::c_int as yytype_int8,
    20 as libc::c_int as yytype_int8,
    21 as libc::c_int as yytype_int8,
    -(1 as libc::c_int) as yytype_int8,
    -(1 as libc::c_int) as yytype_int8,
    -(1 as libc::c_int) as yytype_int8,
    25 as libc::c_int as yytype_int8,
    3 as libc::c_int as yytype_int8,
    4 as libc::c_int as yytype_int8,
    5 as libc::c_int as yytype_int8,
    6 as libc::c_int as yytype_int8,
    7 as libc::c_int as yytype_int8,
    -(1 as libc::c_int) as yytype_int8,
    -(1 as libc::c_int) as yytype_int8,
    -(1 as libc::c_int) as yytype_int8,
    -(1 as libc::c_int) as yytype_int8,
    -(1 as libc::c_int) as yytype_int8,
    13 as libc::c_int as yytype_int8,
    14 as libc::c_int as yytype_int8,
    -(1 as libc::c_int) as yytype_int8,
    -(1 as libc::c_int) as yytype_int8,
    17 as libc::c_int as yytype_int8,
    18 as libc::c_int as yytype_int8,
    19 as libc::c_int as yytype_int8,
    20 as libc::c_int as yytype_int8,
    21 as libc::c_int as yytype_int8,
    -(1 as libc::c_int) as yytype_int8,
    -(1 as libc::c_int) as yytype_int8,
    -(1 as libc::c_int) as yytype_int8,
    25 as libc::c_int as yytype_int8,
];
static mut yystos: [yytype_int8; 61] = [
    0 as libc::c_int as yytype_int8,
    1 as libc::c_int as yytype_int8,
    3 as libc::c_int as yytype_int8,
    4 as libc::c_int as yytype_int8,
    5 as libc::c_int as yytype_int8,
    6 as libc::c_int as yytype_int8,
    7 as libc::c_int as yytype_int8,
    11 as libc::c_int as yytype_int8,
    13 as libc::c_int as yytype_int8,
    14 as libc::c_int as yytype_int8,
    15 as libc::c_int as yytype_int8,
    16 as libc::c_int as yytype_int8,
    17 as libc::c_int as yytype_int8,
    18 as libc::c_int as yytype_int8,
    19 as libc::c_int as yytype_int8,
    20 as libc::c_int as yytype_int8,
    21 as libc::c_int as yytype_int8,
    23 as libc::c_int as yytype_int8,
    25 as libc::c_int as yytype_int8,
    28 as libc::c_int as yytype_int8,
    29 as libc::c_int as yytype_int8,
    30 as libc::c_int as yytype_int8,
    31 as libc::c_int as yytype_int8,
    32 as libc::c_int as yytype_int8,
    33 as libc::c_int as yytype_int8,
    32 as libc::c_int as yytype_int8,
    32 as libc::c_int as yytype_int8,
    32 as libc::c_int as yytype_int8,
    33 as libc::c_int as yytype_int8,
    32 as libc::c_int as yytype_int8,
    32 as libc::c_int as yytype_int8,
    33 as libc::c_int as yytype_int8,
    4 as libc::c_int as yytype_int8,
    7 as libc::c_int as yytype_int8,
    33 as libc::c_int as yytype_int8,
    30 as libc::c_int as yytype_int8,
    0 as libc::c_int as yytype_int8,
    16 as libc::c_int as yytype_int8,
    9 as libc::c_int as yytype_int8,
    10 as libc::c_int as yytype_int8,
    11 as libc::c_int as yytype_int8,
    22 as libc::c_int as yytype_int8,
    23 as libc::c_int as yytype_int8,
    12 as libc::c_int as yytype_int8,
    8 as libc::c_int as yytype_int8,
    15 as libc::c_int as yytype_int8,
    33 as libc::c_int as yytype_int8,
    32 as libc::c_int as yytype_int8,
    26 as libc::c_int as yytype_int8,
    30 as libc::c_int as yytype_int8,
    30 as libc::c_int as yytype_int8,
    30 as libc::c_int as yytype_int8,
    30 as libc::c_int as yytype_int8,
    30 as libc::c_int as yytype_int8,
    31 as libc::c_int as yytype_int8,
    15 as libc::c_int as yytype_int8,
    23 as libc::c_int as yytype_int8,
    33 as libc::c_int as yytype_int8,
    33 as libc::c_int as yytype_int8,
    33 as libc::c_int as yytype_int8,
    33 as libc::c_int as yytype_int8,
];
static mut yyr1: [yytype_int8; 37] = [
    0 as libc::c_int as yytype_int8,
    27 as libc::c_int as yytype_int8,
    28 as libc::c_int as yytype_int8,
    28 as libc::c_int as yytype_int8,
    28 as libc::c_int as yytype_int8,
    29 as libc::c_int as yytype_int8,
    29 as libc::c_int as yytype_int8,
    30 as libc::c_int as yytype_int8,
    30 as libc::c_int as yytype_int8,
    30 as libc::c_int as yytype_int8,
    30 as libc::c_int as yytype_int8,
    30 as libc::c_int as yytype_int8,
    30 as libc::c_int as yytype_int8,
    30 as libc::c_int as yytype_int8,
    30 as libc::c_int as yytype_int8,
    31 as libc::c_int as yytype_int8,
    31 as libc::c_int as yytype_int8,
    32 as libc::c_int as yytype_int8,
    33 as libc::c_int as yytype_int8,
    33 as libc::c_int as yytype_int8,
    33 as libc::c_int as yytype_int8,
    33 as libc::c_int as yytype_int8,
    33 as libc::c_int as yytype_int8,
    33 as libc::c_int as yytype_int8,
    33 as libc::c_int as yytype_int8,
    33 as libc::c_int as yytype_int8,
    33 as libc::c_int as yytype_int8,
    33 as libc::c_int as yytype_int8,
    33 as libc::c_int as yytype_int8,
    33 as libc::c_int as yytype_int8,
    33 as libc::c_int as yytype_int8,
    33 as libc::c_int as yytype_int8,
    33 as libc::c_int as yytype_int8,
    33 as libc::c_int as yytype_int8,
    33 as libc::c_int as yytype_int8,
    33 as libc::c_int as yytype_int8,
    33 as libc::c_int as yytype_int8,
];
static mut yyr2: [yytype_int8; 37] = [
    0 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    1 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    1 as libc::c_int as yytype_int8,
    1 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    1 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    3 as libc::c_int as yytype_int8,
    3 as libc::c_int as yytype_int8,
    3 as libc::c_int as yytype_int8,
    3 as libc::c_int as yytype_int8,
    3 as libc::c_int as yytype_int8,
    1 as libc::c_int as yytype_int8,
    3 as libc::c_int as yytype_int8,
    3 as libc::c_int as yytype_int8,
    1 as libc::c_int as yytype_int8,
    1 as libc::c_int as yytype_int8,
    3 as libc::c_int as yytype_int8,
    3 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    1 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    3 as libc::c_int as yytype_int8,
    4 as libc::c_int as yytype_int8,
    4 as libc::c_int as yytype_int8,
    1 as libc::c_int as yytype_int8,
    1 as libc::c_int as yytype_int8,
    1 as libc::c_int as yytype_int8,
    1 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
];
unsafe extern "C" fn yydestruct(
    mut yymsg: *const libc::c_char,
    mut yytype: libc::c_int,
    mut yyvaluep: *mut UNITSSTYPE,
    mut comm: *mut commtype,
) {
    if yymsg.is_null() {
        yymsg = b"Deleting\0" as *const u8 as *const libc::c_char;
    }
    match yytype {
        4 => {
            destroyunit((*yyvaluep).unit);
        }
        29 => {
            destroyunit((*yyvaluep).unit);
        }
        30 => {
            destroyunit((*yyvaluep).unit);
        }
        32 => {
            destroyunit((*yyvaluep).unit);
        }
        33 => {
            destroyunit((*yyvaluep).unit);
        }
        _ => {}
    };
}
pub unsafe extern "C" fn unitsparse(mut comm: *mut commtype) -> libc::c_int {
    let mut current_block: u64;
    let mut yychar: libc::c_int = 0;
    static mut yyval_default: UNITSSTYPE = UNITSSTYPE { number: 0. };
    let mut yylval: UNITSSTYPE = yyval_default;
    let mut unitsnerrs: libc::c_int = 0;
    let mut yystate: yy_state_fast_t = 0;
    let mut yyerrstatus: libc::c_int = 0;
    let mut yyssa: [yy_state_t; 200] = [0; 200];
    let mut yyss: *mut yy_state_t = 0 as *mut yy_state_t;
    let mut yyssp: *mut yy_state_t = 0 as *mut yy_state_t;
    let mut yyvsa: [UNITSSTYPE; 200] = [UNITSSTYPE { number: 0. }; 200];
    let mut yyvs: *mut UNITSSTYPE = 0 as *mut UNITSSTYPE;
    let mut yyvsp: *mut UNITSSTYPE = 0 as *mut UNITSSTYPE;
    let mut yystacksize: libc::c_long = 0;
    let mut yyn: libc::c_int = 0;
    let mut yyresult: libc::c_int = 0;
    let mut yytoken: libc::c_int = 0 as libc::c_int;
    let mut yyval: UNITSSTYPE = UNITSSTYPE { number: 0. };
    let mut yylen: libc::c_int = 0 as libc::c_int;
    yyss = yyssa.as_mut_ptr();
    yyssp = yyss;
    yyvs = yyvsa.as_mut_ptr();
    yyvsp = yyvs;
    yystacksize = 200 as libc::c_int as libc::c_long;
    yystate = 0 as libc::c_int;
    yyerrstatus = 0 as libc::c_int;
    unitsnerrs = 0 as libc::c_int;
    yychar = -(2 as libc::c_int);
    's_72: loop {
        (0 as libc::c_int != 0
            && (0 as libc::c_int <= yystate && yystate < 61 as libc::c_int))
            as libc::c_int;
        *yyssp = yystate as yy_state_t;
        if yyss.offset(yystacksize as isize).offset(-(1 as libc::c_int as isize))
            <= yyssp
        {
            let mut yysize: libc::c_long = yyssp.offset_from(yyss) as libc::c_long
                + 1 as libc::c_int as libc::c_long;
            if 10000 as libc::c_int as libc::c_long <= yystacksize {
                current_block = 6019918473935222641;
                break;
            }
            yystacksize *= 2 as libc::c_int as libc::c_long;
            if (10000 as libc::c_int as libc::c_long) < yystacksize {
                yystacksize = 10000 as libc::c_int as libc::c_long;
            }
            let mut yyss1: *mut yy_state_t = yyss;
            let mut yyptr: *mut yyalloc = malloc(
                (yystacksize
                    * (::std::mem::size_of::<yy_state_t>() as libc::c_ulong
                        as libc::c_long
                        + ::std::mem::size_of::<UNITSSTYPE>() as libc::c_ulong
                            as libc::c_long)
                    + (::std::mem::size_of::<yyalloc>() as libc::c_ulong as libc::c_long
                        - 1 as libc::c_int as libc::c_long)) as libc::c_ulong,
            ) as *mut yyalloc;
            if yyptr.is_null() {
                current_block = 6019918473935222641;
                break;
            }
            let mut yynewbytes: libc::c_long = 0;
            libc::memcpy(
                &mut (*yyptr).yyss_alloc as *mut yy_state_t as *mut libc::c_void,
                yyss as *const libc::c_void,
                (yysize as libc::c_ulong)
                    .wrapping_mul(::std::mem::size_of::<yy_state_t>() as libc::c_ulong)
                    as libc::size_t,
            );
            yyss = &mut (*yyptr).yyss_alloc;
            yynewbytes = yystacksize
                * ::std::mem::size_of::<yy_state_t>() as libc::c_ulong as libc::c_long
                + (::std::mem::size_of::<yyalloc>() as libc::c_ulong as libc::c_long
                    - 1 as libc::c_int as libc::c_long);
            yyptr = yyptr
                .offset(
                    (yynewbytes
                        / ::std::mem::size_of::<yyalloc>() as libc::c_ulong
                            as libc::c_long) as isize,
                );
            let mut yynewbytes_0: libc::c_long = 0;
            libc::memcpy(
                &mut (*yyptr).yyvs_alloc as *mut UNITSSTYPE as *mut libc::c_void,
                yyvs as *const libc::c_void,
                (yysize as libc::c_ulong)
                    .wrapping_mul(::std::mem::size_of::<UNITSSTYPE>() as libc::c_ulong)
                    as libc::size_t,
            );
            yyvs = &mut (*yyptr).yyvs_alloc;
            yynewbytes_0 = yystacksize
                * ::std::mem::size_of::<UNITSSTYPE>() as libc::c_ulong as libc::c_long
                + (::std::mem::size_of::<yyalloc>() as libc::c_ulong as libc::c_long
                    - 1 as libc::c_int as libc::c_long);
            yyptr = yyptr
                .offset(
                    (yynewbytes_0
                        / ::std::mem::size_of::<yyalloc>() as libc::c_ulong
                            as libc::c_long) as isize,
                );
            if yyss1 != yyssa.as_mut_ptr() {
                free(yyss1 as *mut libc::c_void);
            }
            yyssp = yyss.offset(yysize as isize).offset(-(1 as libc::c_int as isize));
            yyvsp = yyvs.offset(yysize as isize).offset(-(1 as libc::c_int as isize));
            if yyss.offset(yystacksize as isize).offset(-(1 as libc::c_int as isize))
                <= yyssp
            {
                current_block = 10771168077708633522;
                break;
            }
        }
        if yystate == 36 as libc::c_int {
            current_block = 13558608202253810511;
            break;
        }
        yyn = yypact[yystate as usize] as libc::c_int;
        if yyn == -(22 as libc::c_int) {
            current_block = 1855181141018664338;
        } else {
            if yychar == -(2 as libc::c_int) {
                yychar = unitslex(&mut yylval, comm);
            }
            if yychar <= 0 as libc::c_int {
                yytoken = 0 as libc::c_int;
                yychar = yytoken;
            } else {
                yytoken = if 0 as libc::c_int <= yychar && yychar <= 279 as libc::c_int {
                    yytranslate[yychar as usize] as libc::c_int
                } else {
                    2 as libc::c_int
                };
            }
            yyn += yytoken;
            if yyn < 0 as libc::c_int || (176 as libc::c_int) < yyn
                || yycheck[yyn as usize] as libc::c_int != yytoken
            {
                current_block = 1855181141018664338;
            } else {
                yyn = yytable[yyn as usize] as libc::c_int;
                if yyn <= 0 as libc::c_int {
                    yyn = -yyn;
                    current_block = 7946419982477676839;
                } else {
                    if yyerrstatus != 0 {
                        yyerrstatus -= 1;
                        yyerrstatus;
                    }
                    yystate = yyn;
                    yyvsp = yyvsp.offset(1);
                    *yyvsp = yylval;
                    yychar = -(2 as libc::c_int);
                    current_block = 15663895909861267290;
                }
            }
        }
        match current_block {
            1855181141018664338 => {
                yyn = yydefact[yystate as usize] as libc::c_int;
                if yyn == 0 as libc::c_int {
                    yytoken = if yychar == -(2 as libc::c_int) {
                        -(2 as libc::c_int)
                    } else if 0 as libc::c_int <= yychar && yychar <= 279 as libc::c_int
                    {
                        yytranslate[yychar as usize] as libc::c_int
                    } else {
                        2 as libc::c_int
                    };
                    if yyerrstatus == 0 {
                        unitsnerrs += 1;
                        unitsnerrs;
                        unitserror(
                            comm,
                            b"syntax error\0" as *const u8 as *const libc::c_char
                                as *mut libc::c_char,
                        );
                    }
                    if yyerrstatus == 3 as libc::c_int {
                        if yychar <= 0 as libc::c_int {
                            if yychar == 0 as libc::c_int {
                                current_block = 10771168077708633522;
                                break;
                            }
                        } else {
                            yydestruct(
                                b"Error: discarding\0" as *const u8 as *const libc::c_char,
                                yytoken,
                                &mut yylval,
                                comm,
                            );
                            yychar = -(2 as libc::c_int);
                        }
                    }
                    yyerrstatus = 3 as libc::c_int;
                    loop {
                        yyn = yypact[yystate as usize] as libc::c_int;
                        if !(yyn == -(22 as libc::c_int)) {
                            yyn += 1 as libc::c_int;
                            if 0 as libc::c_int <= yyn && yyn <= 176 as libc::c_int
                                && yycheck[yyn as usize] as libc::c_int == 1 as libc::c_int
                            {
                                yyn = yytable[yyn as usize] as libc::c_int;
                                if (0 as libc::c_int) < yyn {
                                    break;
                                }
                            }
                        }
                        if yyssp == yyss {
                            current_block = 10771168077708633522;
                            break 's_72;
                        }
                        yydestruct(
                            b"Error: popping\0" as *const u8 as *const libc::c_char,
                            yystos[yystate as usize] as libc::c_int,
                            yyvsp,
                            comm,
                        );
                        yyvsp = yyvsp.offset(-(1 as libc::c_int as isize));
                        yyssp = yyssp.offset(-(1 as libc::c_int as isize));
                        yystate = *yyssp as yy_state_fast_t;
                    }
                    yyvsp = yyvsp.offset(1);
                    *yyvsp = yylval;
                    yystate = yyn;
                    current_block = 15663895909861267290;
                } else {
                    current_block = 7946419982477676839;
                }
            }
            _ => {}
        }
        match current_block {
            7946419982477676839 => {
                yylen = yyr2[yyn as usize] as libc::c_int;
                yyval = *yyvsp.offset((1 as libc::c_int - yylen) as isize);
                match yyn {
                    2 => {
                        (*comm)
                            .result = makenumunit(
                            1 as libc::c_int as libc::c_double,
                            &mut err,
                        );
                        if err != 0 {
                            current_block = 15594839951440953787;
                            break;
                        } else {
                            current_block = 10067844863897285902;
                            break;
                        }
                    }
                    3 => {
                        (*comm)
                            .result = (*yyvsp.offset(-(1 as libc::c_int) as isize)).unit;
                        (*comm).errorcode = 0 as libc::c_int;
                        current_block = 13558608202253810511;
                        break;
                    }
                    4 => {
                        current_block = 10771168077708633522;
                        break;
                    }
                    5 => {
                        yyval.unit = (*yyvsp.offset(0 as libc::c_int as isize)).unit;
                    }
                    6 => {
                        invertunit((*yyvsp.offset(0 as libc::c_int as isize)).unit);
                        yyval.unit = (*yyvsp.offset(0 as libc::c_int as isize)).unit;
                    }
                    7 => {
                        yyval.unit = (*yyvsp.offset(0 as libc::c_int as isize)).unit;
                    }
                    8 => {
                        yyval.unit = (*yyvsp.offset(0 as libc::c_int as isize)).unit;
                        (*yyval.unit).factor *= -(1 as libc::c_int) as libc::c_double;
                    }
                    9 => {
                        yyval.unit = (*yyvsp.offset(0 as libc::c_int as isize)).unit;
                        (*yyval.unit).factor *= -(1 as libc::c_int) as libc::c_double;
                    }
                    10 => {
                        err = addunit(
                            (*yyvsp.offset(-(2 as libc::c_int) as isize)).unit,
                            (*yyvsp.offset(0 as libc::c_int as isize)).unit,
                        );
                        destroyunit((*yyvsp.offset(0 as libc::c_int as isize)).unit);
                        if err != 0 {
                            (*comm).errorcode = err;
                            if !((*yyvsp.offset(-(2 as libc::c_int) as isize)).unit)
                                .is_null()
                            {
                                destroyunit(
                                    (*yyvsp.offset(-(2 as libc::c_int) as isize)).unit,
                                );
                            }
                            current_block = 10771168077708633522;
                            break;
                        } else {
                            yyval
                                .unit = (*yyvsp.offset(-(2 as libc::c_int) as isize)).unit;
                        }
                    }
                    11 => {
                        (*(*yyvsp.offset(0 as libc::c_int as isize)).unit).factor
                            *= -(1 as libc::c_int) as libc::c_double;
                        err = addunit(
                            (*yyvsp.offset(-(2 as libc::c_int) as isize)).unit,
                            (*yyvsp.offset(0 as libc::c_int as isize)).unit,
                        );
                        destroyunit((*yyvsp.offset(0 as libc::c_int as isize)).unit);
                        if err != 0 {
                            (*comm).errorcode = err;
                            if !((*yyvsp.offset(-(2 as libc::c_int) as isize)).unit)
                                .is_null()
                            {
                                destroyunit(
                                    (*yyvsp.offset(-(2 as libc::c_int) as isize)).unit,
                                );
                            }
                            current_block = 10771168077708633522;
                            break;
                        } else {
                            yyval
                                .unit = (*yyvsp.offset(-(2 as libc::c_int) as isize)).unit;
                        }
                    }
                    12 => {
                        err = divunit(
                            (*yyvsp.offset(-(2 as libc::c_int) as isize)).unit,
                            (*yyvsp.offset(0 as libc::c_int as isize)).unit,
                        );
                        destroyunit((*yyvsp.offset(0 as libc::c_int as isize)).unit);
                        if err != 0 {
                            (*comm).errorcode = err;
                            if !((*yyvsp.offset(-(2 as libc::c_int) as isize)).unit)
                                .is_null()
                            {
                                destroyunit(
                                    (*yyvsp.offset(-(2 as libc::c_int) as isize)).unit,
                                );
                            }
                            current_block = 10771168077708633522;
                            break;
                        } else {
                            yyval
                                .unit = (*yyvsp.offset(-(2 as libc::c_int) as isize)).unit;
                        }
                    }
                    13 => {
                        err = multunit(
                            (*yyvsp.offset(-(2 as libc::c_int) as isize)).unit,
                            (*yyvsp.offset(0 as libc::c_int as isize)).unit,
                        );
                        destroyunit((*yyvsp.offset(0 as libc::c_int as isize)).unit);
                        if err != 0 {
                            (*comm).errorcode = err;
                            if !((*yyvsp.offset(-(2 as libc::c_int) as isize)).unit)
                                .is_null()
                            {
                                destroyunit(
                                    (*yyvsp.offset(-(2 as libc::c_int) as isize)).unit,
                                );
                            }
                            current_block = 10771168077708633522;
                            break;
                        } else {
                            yyval
                                .unit = (*yyvsp.offset(-(2 as libc::c_int) as isize)).unit;
                        }
                    }
                    14 => {
                        err = multunit(
                            (*yyvsp.offset(-(2 as libc::c_int) as isize)).unit,
                            (*yyvsp.offset(0 as libc::c_int as isize)).unit,
                        );
                        destroyunit((*yyvsp.offset(0 as libc::c_int as isize)).unit);
                        if err != 0 {
                            (*comm).errorcode = err;
                            if !((*yyvsp.offset(-(2 as libc::c_int) as isize)).unit)
                                .is_null()
                            {
                                destroyunit(
                                    (*yyvsp.offset(-(2 as libc::c_int) as isize)).unit,
                                );
                            }
                            current_block = 10771168077708633522;
                            break;
                        } else {
                            yyval
                                .unit = (*yyvsp.offset(-(2 as libc::c_int) as isize)).unit;
                        }
                    }
                    15 => {
                        yyval.number = (*yyvsp.offset(0 as libc::c_int as isize)).number;
                    }
                    16 => {
                        yyval
                            .number = (*yyvsp.offset(-(2 as libc::c_int) as isize))
                            .number / (*yyvsp.offset(0 as libc::c_int as isize)).number;
                    }
                    17 => {
                        yyval.unit = (*yyvsp.offset(-(1 as libc::c_int) as isize)).unit;
                    }
                    18 => {
                        yyval
                            .unit = makenumunit(
                            (*yyvsp.offset(0 as libc::c_int as isize)).number,
                            &mut err,
                        );
                        if err != 0 {
                            (*comm).errorcode = err;
                            current_block = 10771168077708633522;
                            break;
                        }
                    }
                    19 => {
                        yyval.unit = (*yyvsp.offset(0 as libc::c_int as isize)).unit;
                    }
                    20 => {
                        err = unitpower(
                            (*yyvsp.offset(-(2 as libc::c_int) as isize)).unit,
                            (*yyvsp.offset(0 as libc::c_int as isize)).unit,
                        );
                        destroyunit((*yyvsp.offset(0 as libc::c_int as isize)).unit);
                        if err != 0 {
                            (*comm).errorcode = err;
                            if !((*yyvsp.offset(-(2 as libc::c_int) as isize)).unit)
                                .is_null()
                            {
                                destroyunit(
                                    (*yyvsp.offset(-(2 as libc::c_int) as isize)).unit,
                                );
                            }
                            current_block = 10771168077708633522;
                            break;
                        } else {
                            yyval
                                .unit = (*yyvsp.offset(-(2 as libc::c_int) as isize)).unit;
                        }
                    }
                    21 => {
                        err = multunit(
                            (*yyvsp.offset(-(2 as libc::c_int) as isize)).unit,
                            (*yyvsp.offset(0 as libc::c_int as isize)).unit,
                        );
                        destroyunit((*yyvsp.offset(0 as libc::c_int as isize)).unit);
                        if err != 0 {
                            (*comm).errorcode = err;
                            if !((*yyvsp.offset(-(2 as libc::c_int) as isize)).unit)
                                .is_null()
                            {
                                destroyunit(
                                    (*yyvsp.offset(-(2 as libc::c_int) as isize)).unit,
                                );
                            }
                            current_block = 10771168077708633522;
                            break;
                        } else {
                            yyval
                                .unit = (*yyvsp.offset(-(2 as libc::c_int) as isize)).unit;
                        }
                    }
                    22 => {
                        err = multunit(
                            (*yyvsp.offset(-(1 as libc::c_int) as isize)).unit,
                            (*yyvsp.offset(0 as libc::c_int as isize)).unit,
                        );
                        destroyunit((*yyvsp.offset(0 as libc::c_int as isize)).unit);
                        if err != 0 {
                            (*comm).errorcode = err;
                            if !((*yyvsp.offset(-(1 as libc::c_int) as isize)).unit)
                                .is_null()
                            {
                                destroyunit(
                                    (*yyvsp.offset(-(1 as libc::c_int) as isize)).unit,
                                );
                            }
                            current_block = 10771168077708633522;
                            break;
                        } else {
                            yyval
                                .unit = (*yyvsp.offset(-(1 as libc::c_int) as isize)).unit;
                        }
                    }
                    23 => {
                        yyval.unit = (*yyvsp.offset(0 as libc::c_int as isize)).unit;
                    }
                    24 => {
                        err = rootunit(
                            (*yyvsp.offset(0 as libc::c_int as isize)).unit,
                            2 as libc::c_int,
                        );
                        if err != 0 {
                            (*comm).errorcode = err;
                            if !((*yyvsp.offset(0 as libc::c_int as isize)).unit)
                                .is_null()
                            {
                                destroyunit(
                                    (*yyvsp.offset(0 as libc::c_int as isize)).unit,
                                );
                            }
                            current_block = 10771168077708633522;
                            break;
                        } else {
                            yyval.unit = (*yyvsp.offset(0 as libc::c_int as isize)).unit;
                        }
                    }
                    25 => {
                        err = rootunit(
                            (*yyvsp.offset(0 as libc::c_int as isize)).unit,
                            3 as libc::c_int,
                        );
                        if err != 0 {
                            (*comm).errorcode = err;
                            if !((*yyvsp.offset(0 as libc::c_int as isize)).unit)
                                .is_null()
                            {
                                destroyunit(
                                    (*yyvsp.offset(0 as libc::c_int as isize)).unit,
                                );
                            }
                            current_block = 10771168077708633522;
                            break;
                        } else {
                            yyval.unit = (*yyvsp.offset(0 as libc::c_int as isize)).unit;
                        }
                    }
                    26 => {
                        err = funcunit(
                            (*yyvsp.offset(0 as libc::c_int as isize)).unit,
                            (*yyvsp.offset(-(1 as libc::c_int) as isize)).realfunc,
                        );
                        if err != 0 {
                            (*comm).errorcode = err;
                            if !((*yyvsp.offset(0 as libc::c_int as isize)).unit)
                                .is_null()
                            {
                                destroyunit(
                                    (*yyvsp.offset(0 as libc::c_int as isize)).unit,
                                );
                            }
                            current_block = 10771168077708633522;
                            break;
                        } else {
                            yyval.unit = (*yyvsp.offset(0 as libc::c_int as isize)).unit;
                        }
                    }
                    27 => {
                        err = logunit(
                            (*yyvsp.offset(0 as libc::c_int as isize)).unit,
                            (*yyvsp.offset(-(1 as libc::c_int) as isize)).integer,
                        );
                        if err != 0 {
                            (*comm).errorcode = err;
                            if !((*yyvsp.offset(0 as libc::c_int as isize)).unit)
                                .is_null()
                            {
                                destroyunit(
                                    (*yyvsp.offset(0 as libc::c_int as isize)).unit,
                                );
                            }
                            current_block = 10771168077708633522;
                            break;
                        } else {
                            yyval.unit = (*yyvsp.offset(0 as libc::c_int as isize)).unit;
                        }
                    }
                    28 => {
                        err = evalfunc(
                            (*yyvsp.offset(0 as libc::c_int as isize)).unit,
                            (*yyvsp.offset(-(1 as libc::c_int) as isize)).unitfunc,
                            0 as libc::c_int,
                            0 as libc::c_int,
                        );
                        if err != 0 {
                            (*comm).errorcode = err;
                            if !((*yyvsp.offset(0 as libc::c_int as isize)).unit)
                                .is_null()
                            {
                                destroyunit(
                                    (*yyvsp.offset(0 as libc::c_int as isize)).unit,
                                );
                            }
                            current_block = 10771168077708633522;
                            break;
                        } else {
                            yyval.unit = (*yyvsp.offset(0 as libc::c_int as isize)).unit;
                        }
                    }
                    29 => {
                        err = evalfunc(
                            (*yyvsp.offset(0 as libc::c_int as isize)).unit,
                            (*yyvsp.offset(-(1 as libc::c_int) as isize)).unitfunc,
                            1 as libc::c_int,
                            0 as libc::c_int,
                        );
                        if err != 0 {
                            (*comm).errorcode = err;
                            if !((*yyvsp.offset(0 as libc::c_int as isize)).unit)
                                .is_null()
                            {
                                destroyunit(
                                    (*yyvsp.offset(0 as libc::c_int as isize)).unit,
                                );
                            }
                            current_block = 10771168077708633522;
                            break;
                        } else {
                            yyval.unit = (*yyvsp.offset(0 as libc::c_int as isize)).unit;
                        }
                    }
                    30 => {
                        (*(*yyvsp.offset(0 as libc::c_int as isize)).unit).factor
                            *= -(1 as libc::c_int) as libc::c_double;
                        err = unitpower(
                            (*yyvsp.offset(-(3 as libc::c_int) as isize)).unit,
                            (*yyvsp.offset(0 as libc::c_int as isize)).unit,
                        );
                        destroyunit((*yyvsp.offset(0 as libc::c_int as isize)).unit);
                        if err != 0 {
                            (*comm).errorcode = err;
                            if !((*yyvsp.offset(-(3 as libc::c_int) as isize)).unit)
                                .is_null()
                            {
                                destroyunit(
                                    (*yyvsp.offset(-(3 as libc::c_int) as isize)).unit,
                                );
                            }
                            current_block = 10771168077708633522;
                            break;
                        } else {
                            yyval
                                .unit = (*yyvsp.offset(-(3 as libc::c_int) as isize)).unit;
                        }
                    }
                    31 => {
                        (*(*yyvsp.offset(0 as libc::c_int as isize)).unit).factor
                            *= -(1 as libc::c_int) as libc::c_double;
                        err = unitpower(
                            (*yyvsp.offset(-(3 as libc::c_int) as isize)).unit,
                            (*yyvsp.offset(0 as libc::c_int as isize)).unit,
                        );
                        destroyunit((*yyvsp.offset(0 as libc::c_int as isize)).unit);
                        if err != 0 {
                            (*comm).errorcode = err;
                            if !((*yyvsp.offset(-(3 as libc::c_int) as isize)).unit)
                                .is_null()
                            {
                                destroyunit(
                                    (*yyvsp.offset(-(3 as libc::c_int) as isize)).unit,
                                );
                            }
                            current_block = 10771168077708633522;
                            break;
                        } else {
                            yyval
                                .unit = (*yyvsp.offset(-(3 as libc::c_int) as isize)).unit;
                        }
                    }
                    32 => {
                        err = 19 as libc::c_int;
                        if err != 0 {
                            (*comm).errorcode = err;
                            current_block = 10771168077708633522;
                            break;
                        }
                    }
                    33 => {
                        err = 14 as libc::c_int;
                        if err != 0 {
                            (*comm).errorcode = err;
                            current_block = 10771168077708633522;
                            break;
                        }
                    }
                    34 => {
                        err = 20 as libc::c_int;
                        if err != 0 {
                            (*comm).errorcode = err;
                            current_block = 10771168077708633522;
                            break;
                        }
                    }
                    35 => {
                        err = 21 as libc::c_int;
                        if err != 0 {
                            (*comm).errorcode = err;
                            current_block = 10771168077708633522;
                            break;
                        }
                    }
                    36 => {
                        err = 25 as libc::c_int;
                        if err != 0 {
                            (*comm).errorcode = err;
                            if !((*yyvsp.offset(0 as libc::c_int as isize)).unit)
                                .is_null()
                            {
                                destroyunit(
                                    (*yyvsp.offset(0 as libc::c_int as isize)).unit,
                                );
                            }
                            current_block = 10771168077708633522;
                            break;
                        }
                    }
                    _ => {}
                }
                yyvsp = yyvsp.offset(-(yylen as isize));
                yyssp = yyssp.offset(-(yylen as isize));
                yylen = 0 as libc::c_int;
                yyvsp = yyvsp.offset(1);
                *yyvsp = yyval;
                let yylhs: libc::c_int = yyr1[yyn as usize] as libc::c_int
                    - 27 as libc::c_int;
                let yyi: libc::c_int = yypgoto[yylhs as usize] as libc::c_int
                    + *yyssp as libc::c_int;
                yystate = if 0 as libc::c_int <= yyi && yyi <= 176 as libc::c_int
                    && yycheck[yyi as usize] as libc::c_int == *yyssp as libc::c_int
                {
                    yytable[yyi as usize] as libc::c_int
                } else {
                    yydefgoto[yylhs as usize] as libc::c_int
                };
            }
            _ => {}
        }
        yyssp = yyssp.offset(1);
        yyssp;
    }
    match current_block {
        6019918473935222641 => {
            unitserror(
                comm,
                b"memory exhausted\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
            );
            yyresult = 2 as libc::c_int;
            current_block = 12668393079891547146;
        }
        15594839951440953787 => {
            (*comm).errorcode = err;
            current_block = 10771168077708633522;
        }
        10067844863897285902 => {
            (*comm).errorcode = 0 as libc::c_int;
            current_block = 13558608202253810511;
        }
        _ => {}
    }
    match current_block {
        10771168077708633522 => {
            yyresult = 1 as libc::c_int;
        }
        13558608202253810511 => {
            yyresult = 0 as libc::c_int;
        }
        _ => {}
    }
    if yychar != -(2 as libc::c_int) {
        yytoken = if 0 as libc::c_int <= yychar && yychar <= 279 as libc::c_int {
            yytranslate[yychar as usize] as libc::c_int
        } else {
            2 as libc::c_int
        };
        yydestruct(
            b"Cleanup: discarding lookahead\0" as *const u8 as *const libc::c_char,
            yytoken,
            &mut yylval,
            comm,
        );
    }
    yyvsp = yyvsp.offset(-(yylen as isize));
    yyssp = yyssp.offset(-(yylen as isize));
    while yyssp != yyss {
        yydestruct(
            b"Cleanup: popping\0" as *const u8 as *const libc::c_char,
            yystos[*yyssp as libc::c_int as usize] as libc::c_int,
            yyvsp,
            comm,
        );
        yyvsp = yyvsp.offset(-(1 as libc::c_int as isize));
        yyssp = yyssp.offset(-(1 as libc::c_int as isize));
    }
    if yyss != yyssa.as_mut_ptr() {
        free(yyss as *mut libc::c_void);
    }
    return yyresult;
}
