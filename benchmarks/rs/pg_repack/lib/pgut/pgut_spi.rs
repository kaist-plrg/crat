use ::libc;
extern "C" {
    pub type _SPI_plan;
    fn elog_start(
        filename: *const libc::c_char,
        lineno: libc::c_int,
        funcname: *const libc::c_char,
    );
    fn pfree(pointer: *mut libc::c_void);
    fn elog_finish(elevel: libc::c_int, fmt: *const libc::c_char, _: ...);
    fn initStringInfo(str: StringInfo);
    fn appendStringInfoVA(
        str: StringInfo,
        fmt: *const libc::c_char,
        args: ::std::ffi::VaList,
    ) -> libc::c_int;
    fn enlargeStringInfo(str: StringInfo, needed: libc::c_int);
    fn SPI_execute(
        src: *const libc::c_char,
        read_only: bool,
        tcount: libc::c_long,
    ) -> libc::c_int;
    fn SPI_execute_plan(
        plan: SPIPlanPtr,
        Values: *mut Datum,
        Nulls: *const libc::c_char,
        read_only: bool,
        tcount: libc::c_long,
    ) -> libc::c_int;
    fn SPI_exec(src: *const libc::c_char, tcount: libc::c_long) -> libc::c_int;
    fn SPI_execute_with_args(
        src: *const libc::c_char,
        nargs: libc::c_int,
        argtypes: *mut Oid,
        Values: *mut Datum,
        Nulls: *const libc::c_char,
        read_only: bool,
        tcount: libc::c_long,
    ) -> libc::c_int;
}
pub type __builtin_va_list = [__va_list_tag; 1];
#[derive(Copy, Clone)]
#[repr(C)]
pub struct __va_list_tag {
    pub gp_offset: libc::c_uint,
    pub fp_offset: libc::c_uint,
    pub overflow_arg_area: *mut libc::c_void,
    pub reg_save_area: *mut libc::c_void,
}
pub type Oid = libc::c_uint;
pub type va_list = __builtin_va_list;
pub type uintptr_t = libc::c_ulong;
pub type Datum = uintptr_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct StringInfoData {
    pub data: *mut libc::c_char,
    pub len: libc::c_int,
    pub maxlen: libc::c_int,
    pub cursor: libc::c_int,
}
pub type StringInfo = *mut StringInfoData;
pub type SPIPlanPtr = *mut _SPI_plan;
unsafe extern "C" fn termStringInfo(mut str: StringInfo) {
    if !str.is_null() && !((*str).data).is_null() {
        pfree((*str).data as *mut libc::c_void);
    }
}
unsafe extern "C" fn appendStringInfoVA_s(
    mut str: StringInfo,
    mut fmt: *const libc::c_char,
    mut args: ::std::ffi::VaList,
) {
    let mut needed: libc::c_int = 0;
    loop {
        needed = appendStringInfoVA(str, fmt, args.as_va_list());
        if !(needed > 0 as libc::c_int) {
            break;
        }
        enlargeStringInfo(str, needed);
    };
}
pub unsafe extern "C" fn execute(
    mut expected: libc::c_int,
    mut sql: *const libc::c_char,
) {
    let mut ret: libc::c_int = SPI_execute(
        sql,
        0 as libc::c_int != 0,
        0 as libc::c_int as libc::c_long,
    );
    if expected > 0 as libc::c_int && ret != expected || ret < 0 as libc::c_int {
        let mut __errno_location: libc::c_int = 0;
        elog_start(
            b"pgut/pgut-spi.c\0" as *const u8 as *const libc::c_char,
            46 as libc::c_int,
            (*::std::mem::transmute::<&[u8; 8], &[libc::c_char; 8]>(b"execute\0"))
                .as_ptr(),
        );
        elog_finish(
            20 as libc::c_int,
            b"query failed: (sql=%s, code=%d, expected=%d)\0" as *const u8
                as *const libc::c_char,
            sql,
            ret,
            expected,
        );
        if 0 != 0 && 20 as libc::c_int >= 20 as libc::c_int {
            unreachable!();
        }
    }
}
pub unsafe extern "C" fn execute_plan(
    mut expected: libc::c_int,
    mut plan: SPIPlanPtr,
    mut values: *mut Datum,
    mut nulls: *const libc::c_char,
) {
    let mut ret: libc::c_int = SPI_execute_plan(
        plan,
        values,
        nulls,
        0 as libc::c_int != 0,
        0 as libc::c_int as libc::c_long,
    );
    if expected > 0 as libc::c_int && ret != expected || ret < 0 as libc::c_int {
        let mut __errno_location: libc::c_int = 0;
        elog_start(
            b"pgut/pgut-spi.c\0" as *const u8 as *const libc::c_char,
            55 as libc::c_int,
            (*::std::mem::transmute::<&[u8; 13], &[libc::c_char; 13]>(b"execute_plan\0"))
                .as_ptr(),
        );
        elog_finish(
            20 as libc::c_int,
            b"query failed: (code=%d, expected=%d)\0" as *const u8
                as *const libc::c_char,
            ret,
            expected,
        );
        if 0 != 0 && 20 as libc::c_int >= 20 as libc::c_int {
            unreachable!();
        }
    }
}
pub unsafe extern "C" fn execute_with_format(
    mut expected: libc::c_int,
    mut format: *const libc::c_char,
    mut args: ...
) {
    let mut ap: ::std::ffi::VaListImpl;
    let mut sql: StringInfoData = StringInfoData {
        data: 0 as *mut libc::c_char,
        len: 0,
        maxlen: 0,
        cursor: 0,
    };
    let mut ret: libc::c_int = 0;
    initStringInfo(&mut sql);
    ap = args.clone();
    appendStringInfoVA_s(&mut sql, format, ap.as_va_list());
    if sql.len == 0 as libc::c_int {
        let mut __errno_location: libc::c_int = 0;
        elog_start(
            b"pgut/pgut-spi.c\0" as *const u8 as *const libc::c_char,
            72 as libc::c_int,
            (*::std::mem::transmute::<
                &[u8; 20],
                &[libc::c_char; 20],
            >(b"execute_with_format\0"))
                .as_ptr(),
        );
        elog_finish(
            19 as libc::c_int,
            b"execute_with_format(%s)\0" as *const u8 as *const libc::c_char,
            format,
        );
        if 0 != 0 && 19 as libc::c_int >= 20 as libc::c_int {
            unreachable!();
        }
    }
    ret = SPI_exec(sql.data, 0 as libc::c_int as libc::c_long);
    if expected > 0 as libc::c_int && ret != expected || ret < 0 as libc::c_int {
        let mut __errno_location_0: libc::c_int = 0;
        elog_start(
            b"pgut/pgut-spi.c\0" as *const u8 as *const libc::c_char,
            75 as libc::c_int,
            (*::std::mem::transmute::<
                &[u8; 20],
                &[libc::c_char; 20],
            >(b"execute_with_format\0"))
                .as_ptr(),
        );
        elog_finish(
            20 as libc::c_int,
            b"query failed: (sql=%s, code=%d, expected=%d)\0" as *const u8
                as *const libc::c_char,
            sql.data,
            ret,
            expected,
        );
        if 0 != 0 && 20 as libc::c_int >= 20 as libc::c_int {
            unreachable!();
        }
    }
    termStringInfo(&mut sql);
}
pub unsafe extern "C" fn execute_with_args(
    mut expected: libc::c_int,
    mut src: *const libc::c_char,
    mut nargs: libc::c_int,
    mut argtypes: *mut Oid,
    mut values: *mut Datum,
    mut nulls: *const bool,
) {
    let mut ret: libc::c_int = 0;
    let mut i: libc::c_int = 0;
    let mut c_nulls: [libc::c_char; 100] = [0; 100];
    i = 0 as libc::c_int;
    while i < nargs {
        c_nulls[i
            as usize] = (if *nulls.offset(i as isize) as libc::c_int != 0 {
            'n' as i32
        } else {
            ' ' as i32
        }) as libc::c_char;
        i += 1;
        i;
    }
    ret = SPI_execute_with_args(
        src,
        nargs,
        argtypes,
        values,
        c_nulls.as_mut_ptr(),
        0 as libc::c_int != 0,
        0 as libc::c_int as libc::c_long,
    );
    if expected > 0 as libc::c_int && ret != expected || ret < 0 as libc::c_int {
        let mut __errno_location: libc::c_int = 0;
        elog_start(
            b"pgut/pgut-spi.c\0" as *const u8 as *const libc::c_char,
            92 as libc::c_int,
            (*::std::mem::transmute::<
                &[u8; 18],
                &[libc::c_char; 18],
            >(b"execute_with_args\0"))
                .as_ptr(),
        );
        elog_finish(
            20 as libc::c_int,
            b"query failed: (sql=%s, code=%d, expected=%d)\0" as *const u8
                as *const libc::c_char,
            src,
            ret,
            expected,
        );
        if 0 != 0 && 20 as libc::c_int >= 20 as libc::c_int {
            unreachable!();
        }
    }
}
pub unsafe extern "C" fn execute_with_format_args(
    mut expected: libc::c_int,
    mut format: *const libc::c_char,
    mut nargs: libc::c_int,
    mut argtypes: *mut Oid,
    mut values: *mut Datum,
    mut nulls: *const bool,
    mut args: ...
) {
    let mut ap: ::std::ffi::VaListImpl;
    let mut sql: StringInfoData = StringInfoData {
        data: 0 as *mut libc::c_char,
        len: 0,
        maxlen: 0,
        cursor: 0,
    };
    initStringInfo(&mut sql);
    ap = args.clone();
    appendStringInfoVA_s(&mut sql, format, ap.as_va_list());
    execute_with_args(expected, sql.data, nargs, argtypes, values, nulls);
    termStringInfo(&mut sql);
}
