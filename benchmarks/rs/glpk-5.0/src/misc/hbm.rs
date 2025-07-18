use ::libc;
extern "C" {
    pub type _IO_wide_data;
    pub type _IO_codecvt;
    pub type _IO_marker;
    fn __errno_location() -> *mut libc::c_int;
    fn __ctype_toupper_loc() -> *mut *const __int32_t;
    fn __ctype_b_loc() -> *mut *const libc::c_ushort;
    fn fclose(__stream: *mut FILE) -> libc::c_int;
    fn fopen(_: *const libc::c_char, _: *const libc::c_char) -> *mut FILE;
    fn fgetc(__stream: *mut FILE) -> libc::c_int;
    fn ferror(__stream: *mut FILE) -> libc::c_int;
    fn memcpy(
        _: *mut libc::c_void,
        _: *const libc::c_void,
        _: libc::c_ulong,
    ) -> *mut libc::c_void;
    fn memmove(
        _: *mut libc::c_void,
        _: *const libc::c_void,
        _: libc::c_ulong,
    ) -> *mut libc::c_void;
    fn memset(
        _: *mut libc::c_void,
        _: libc::c_int,
        _: libc::c_ulong,
    ) -> *mut libc::c_void;
    fn strcpy(_: *mut libc::c_char, _: *const libc::c_char) -> *mut libc::c_char;
    fn strcmp(_: *const libc::c_char, _: *const libc::c_char) -> libc::c_int;
    fn strchr(_: *const libc::c_char, _: libc::c_int) -> *mut libc::c_char;
    fn strlen(_: *const libc::c_char) -> libc::c_ulong;
    fn _glp_xstrerr(_: libc::c_int) -> *mut libc::c_char;
    fn glp_printf(fmt: *const libc::c_char, _: ...);
    fn glp_error_(file: *const libc::c_char, line: libc::c_int) -> glp_errfunc;
    fn glp_assert_(
        expr: *const libc::c_char,
        file: *const libc::c_char,
        line: libc::c_int,
    );
    fn glp_alloc(n: libc::c_int, size: libc::c_int) -> *mut libc::c_void;
    fn glp_free(ptr: *mut libc::c_void);
    fn _glp_str2int(str: *const libc::c_char, val: *mut libc::c_int) -> libc::c_int;
    fn _glp_str2num(str: *const libc::c_char, val: *mut libc::c_double) -> libc::c_int;
    fn _glp_strspx(str: *mut libc::c_char) -> *mut libc::c_char;
    fn _glp_strtrim(str: *mut libc::c_char) -> *mut libc::c_char;
}
pub type __int32_t = libc::c_int;
pub type __off_t = libc::c_long;
pub type __off64_t = libc::c_long;
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
pub type glp_errfunc = Option::<unsafe extern "C" fn(*const libc::c_char, ...) -> ()>;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct HBM {
    pub title: [libc::c_char; 73],
    pub key: [libc::c_char; 9],
    pub mxtype: [libc::c_char; 4],
    pub rhstyp: [libc::c_char; 4],
    pub ptrfmt: [libc::c_char; 17],
    pub indfmt: [libc::c_char; 17],
    pub valfmt: [libc::c_char; 21],
    pub rhsfmt: [libc::c_char; 21],
    pub totcrd: libc::c_int,
    pub ptrcrd: libc::c_int,
    pub indcrd: libc::c_int,
    pub valcrd: libc::c_int,
    pub rhscrd: libc::c_int,
    pub nrow: libc::c_int,
    pub ncol: libc::c_int,
    pub nnzero: libc::c_int,
    pub neltvl: libc::c_int,
    pub nrhs: libc::c_int,
    pub nrhsix: libc::c_int,
    pub nrhsvl: libc::c_int,
    pub nguess: libc::c_int,
    pub nexact: libc::c_int,
    pub colptr: *mut libc::c_int,
    pub rowind: *mut libc::c_int,
    pub rhsptr: *mut libc::c_int,
    pub rhsind: *mut libc::c_int,
    pub values: *mut libc::c_double,
    pub rhsval: *mut libc::c_double,
    pub sguess: *mut libc::c_double,
    pub xexact: *mut libc::c_double,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct dsa {
    pub fname: *const libc::c_char,
    pub fp: *mut FILE,
    pub seqn: libc::c_int,
    pub card: [libc::c_char; 81],
    pub fmt_p: libc::c_int,
    pub fmt_k: libc::c_int,
    pub fmt_f: libc::c_int,
    pub fmt_w: libc::c_int,
    pub fmt_d: libc::c_int,
}
#[inline]
unsafe extern "C" fn toupper(mut __c: libc::c_int) -> libc::c_int {
    return if __c >= -(128 as libc::c_int) && __c < 256 as libc::c_int {
        *(*__ctype_toupper_loc()).offset(__c as isize)
    } else {
        __c
    };
}
unsafe extern "C" fn read_card(mut dsa: *mut dsa) -> libc::c_int {
    let mut current_block: u64;
    let mut c: libc::c_int = 0;
    let mut len: libc::c_int = 0 as libc::c_int;
    let mut buf: [libc::c_char; 256] = [0; 256];
    (*dsa).seqn += 1;
    (*dsa).seqn;
    loop {
        c = fgetc((*dsa).fp);
        if c == -(1 as libc::c_int) {
            if ferror((*dsa).fp) != 0 {
                glp_printf(
                    b"%s:%d: read error\n\0" as *const u8 as *const libc::c_char,
                    (*dsa).fname,
                    (*dsa).seqn,
                );
            } else {
                glp_printf(
                    b"%s:%d: unexpected end-of-file\n\0" as *const u8
                        as *const libc::c_char,
                    (*dsa).fname,
                    (*dsa).seqn,
                );
            }
            return 1 as libc::c_int;
        } else {
            if c == '\r' as i32 {
                continue;
            }
            if c == '\n' as i32 {
                current_block = 13056961889198038528;
                break;
            }
            if *(*__ctype_b_loc()).offset(c as isize) as libc::c_int
                & _IScntrl as libc::c_int as libc::c_ushort as libc::c_int != 0
            {
                glp_printf(
                    b"%s:%d: invalid control character\n\0" as *const u8
                        as *const libc::c_char,
                    (*dsa).fname,
                    (*dsa).seqn,
                    c,
                );
                return 1 as libc::c_int;
            } else {
                if len as libc::c_ulong
                    == (::std::mem::size_of::<[libc::c_char; 256]>() as libc::c_ulong)
                        .wrapping_sub(1 as libc::c_int as libc::c_ulong)
                {
                    current_block = 2972889306200436927;
                    break;
                }
                let fresh0 = len;
                len = len + 1;
                buf[fresh0 as usize] = c as libc::c_char;
            }
        }
    }
    match current_block {
        13056961889198038528 => {
            while len > 80 as libc::c_int
                && buf[(len - 1 as libc::c_int) as usize] as libc::c_int == ' ' as i32
            {
                len -= 1;
                len;
            }
            buf[len as usize] = '\0' as i32 as libc::c_char;
            if !(len > 80 as libc::c_int) {
                strcpy(((*dsa).card).as_mut_ptr(), buf.as_mut_ptr());
                memset(
                    &mut *((*dsa).card).as_mut_ptr().offset(len as isize)
                        as *mut libc::c_char as *mut libc::c_void,
                    ' ' as i32,
                    (80 as libc::c_int - len) as libc::c_ulong,
                );
                (*dsa).card[80 as libc::c_int as usize] = '\0' as i32 as libc::c_char;
                return 0 as libc::c_int;
            }
        }
        _ => {}
    }
    (glp_error_(b"misc/hbm.c\0" as *const u8 as *const libc::c_char, 119 as libc::c_int))
        .unwrap()(
        b"%s:%d: card image too long\n\0" as *const u8 as *const libc::c_char,
        (*dsa).fname,
        (*dsa).seqn,
    );
    return 1 as libc::c_int;
}
unsafe extern "C" fn scan_int(
    mut dsa: *mut dsa,
    mut fld: *mut libc::c_char,
    mut pos: libc::c_int,
    mut width: libc::c_int,
    mut val: *mut libc::c_int,
) -> libc::c_int {
    let mut str: [libc::c_char; 81] = [0; 81];
    (1 as libc::c_int <= width && width <= 80 as libc::c_int
        || {
            glp_assert_(
                b"1 <= width && width <= 80\0" as *const u8 as *const libc::c_char,
                b"misc/hbm.c\0" as *const u8 as *const libc::c_char,
                143 as libc::c_int,
            );
            1 as libc::c_int != 0
        }) as libc::c_int;
    memcpy(
        str.as_mut_ptr() as *mut libc::c_void,
        ((*dsa).card).as_mut_ptr().offset(pos as isize) as *const libc::c_void,
        width as libc::c_ulong,
    );
    str[width as usize] = '\0' as i32 as libc::c_char;
    if _glp_str2int(_glp_strspx(str.as_mut_ptr()), val) != 0 {
        glp_printf(
            b"%s:%d: field '%s' contains invalid value '%s'\n\0" as *const u8
                as *const libc::c_char,
            (*dsa).fname,
            (*dsa).seqn,
            fld,
            str.as_mut_ptr(),
        );
        return 1 as libc::c_int;
    }
    return 0 as libc::c_int;
}
unsafe extern "C" fn parse_fmt(
    mut dsa: *mut dsa,
    mut fmt: *mut libc::c_char,
) -> libc::c_int {
    let mut current_block: u64;
    let mut k: libc::c_int = 0;
    let mut s: libc::c_int = 0;
    let mut val: libc::c_int = 0;
    let mut str: [libc::c_char; 81] = [0; 81];
    if !(*fmt.offset(0 as libc::c_int as isize) as libc::c_int != '(' as i32) {
        k = 1 as libc::c_int;
        (*dsa).fmt_p = 0 as libc::c_int;
        if *(*__ctype_b_loc())
            .offset(*fmt.offset(k as isize) as libc::c_uchar as libc::c_int as isize)
            as libc::c_int & _ISdigit as libc::c_int as libc::c_ushort as libc::c_int
            != 0
        {
            s = 0 as libc::c_int;
            loop {
                if !(*(*__ctype_b_loc())
                    .offset(
                        *fmt.offset(k as isize) as libc::c_uchar as libc::c_int as isize,
                    ) as libc::c_int
                    & _ISdigit as libc::c_int as libc::c_ushort as libc::c_int != 0)
                {
                    current_block = 13513818773234778473;
                    break;
                }
                if s == 80 as libc::c_int {
                    current_block = 4701464773222888860;
                    break;
                }
                let fresh1 = k;
                k = k + 1;
                let fresh2 = s;
                s = s + 1;
                str[fresh2 as usize] = *fmt.offset(fresh1 as isize);
            }
            match current_block {
                4701464773222888860 => {}
                _ => {
                    str[s as usize] = '\0' as i32 as libc::c_char;
                    if _glp_str2int(str.as_mut_ptr(), &mut val) != 0 {
                        current_block = 4701464773222888860;
                    } else if ({
                        let mut __res: libc::c_int = 0;
                        if ::std::mem::size_of::<libc::c_uchar>() as libc::c_ulong
                            > 1 as libc::c_int as libc::c_ulong
                        {
                            if 0 != 0 {
                                let mut __c: libc::c_int = *fmt.offset(k as isize)
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
                                    *fmt.offset(k as isize) as libc::c_uchar as libc::c_int,
                                );
                            }
                        } else {
                            __res = *(*__ctype_toupper_loc())
                                .offset(
                                    *fmt.offset(k as isize) as libc::c_uchar as libc::c_int
                                        as isize,
                                );
                        }
                        __res
                    }) != 'P' as i32
                    {
                        current_block = 10904538013685467426;
                    } else {
                        (*dsa).fmt_p = val;
                        k += 1;
                        k;
                        if !(0 as libc::c_int <= (*dsa).fmt_p
                            && (*dsa).fmt_p <= 255 as libc::c_int)
                        {
                            current_block = 4701464773222888860;
                        } else {
                            if *fmt.offset(k as isize) as libc::c_int == ',' as i32 {
                                k += 1;
                                k;
                            }
                            current_block = 9606288038608642794;
                        }
                    }
                }
            }
        } else {
            current_block = 9606288038608642794;
        }
        match current_block {
            4701464773222888860 => {}
            _ => {
                match current_block {
                    9606288038608642794 => {
                        (*dsa).fmt_k = 1 as libc::c_int;
                        if *(*__ctype_b_loc())
                            .offset(
                                *fmt.offset(k as isize) as libc::c_uchar as libc::c_int
                                    as isize,
                            ) as libc::c_int
                            & _ISdigit as libc::c_int as libc::c_ushort as libc::c_int
                            != 0
                        {
                            s = 0 as libc::c_int;
                            loop {
                                if !(*(*__ctype_b_loc())
                                    .offset(
                                        *fmt.offset(k as isize) as libc::c_uchar as libc::c_int
                                            as isize,
                                    ) as libc::c_int
                                    & _ISdigit as libc::c_int as libc::c_ushort as libc::c_int
                                    != 0)
                                {
                                    current_block = 26972500619410423;
                                    break;
                                }
                                if s == 80 as libc::c_int {
                                    current_block = 4701464773222888860;
                                    break;
                                }
                                let fresh3 = k;
                                k = k + 1;
                                let fresh4 = s;
                                s = s + 1;
                                str[fresh4 as usize] = *fmt.offset(fresh3 as isize);
                            }
                            match current_block {
                                4701464773222888860 => {}
                                _ => {
                                    str[s as usize] = '\0' as i32 as libc::c_char;
                                    if _glp_str2int(str.as_mut_ptr(), &mut val) != 0 {
                                        current_block = 4701464773222888860;
                                    } else {
                                        current_block = 10904538013685467426;
                                    }
                                }
                            }
                        } else {
                            current_block = 2719512138335094285;
                        }
                    }
                    _ => {}
                }
                match current_block {
                    4701464773222888860 => {}
                    _ => {
                        match current_block {
                            10904538013685467426 => {
                                (*dsa).fmt_k = val;
                                if !(1 as libc::c_int <= (*dsa).fmt_k
                                    && (*dsa).fmt_k <= 255 as libc::c_int)
                                {
                                    current_block = 4701464773222888860;
                                } else {
                                    current_block = 2719512138335094285;
                                }
                            }
                            _ => {}
                        }
                        match current_block {
                            4701464773222888860 => {}
                            _ => {
                                (*dsa)
                                    .fmt_f = ({
                                    let mut __res: libc::c_int = 0;
                                    if ::std::mem::size_of::<libc::c_uchar>() as libc::c_ulong
                                        > 1 as libc::c_int as libc::c_ulong
                                    {
                                        if 0 != 0 {
                                            let fresh5 = k;
                                            k = k + 1;
                                            let mut __c: libc::c_int = *fmt.offset(fresh5 as isize)
                                                as libc::c_uchar as libc::c_int;
                                            __res = if __c < -(128 as libc::c_int)
                                                || __c > 255 as libc::c_int
                                            {
                                                __c
                                            } else {
                                                *(*__ctype_toupper_loc()).offset(__c as isize)
                                            };
                                        } else {
                                            let fresh6 = k;
                                            k = k + 1;
                                            __res = toupper(
                                                *fmt.offset(fresh6 as isize) as libc::c_uchar as libc::c_int,
                                            );
                                        }
                                    } else {
                                        let fresh7 = k;
                                        k = k + 1;
                                        __res = *(*__ctype_toupper_loc())
                                            .offset(
                                                *fmt.offset(fresh7 as isize) as libc::c_uchar as libc::c_int
                                                    as isize,
                                            );
                                    }
                                    __res
                                });
                                if (*dsa).fmt_f == 'D' as i32 || (*dsa).fmt_f == 'E' as i32
                                    || (*dsa).fmt_f == 'F' as i32 || (*dsa).fmt_f == 'G' as i32
                                    || (*dsa).fmt_f == 'I' as i32
                                {
                                    if !(*(*__ctype_b_loc())
                                        .offset(
                                            *fmt.offset(k as isize) as libc::c_uchar as libc::c_int
                                                as isize,
                                        ) as libc::c_int
                                        & _ISdigit as libc::c_int as libc::c_ushort as libc::c_int
                                        == 0)
                                    {
                                        s = 0 as libc::c_int;
                                        loop {
                                            if !(*(*__ctype_b_loc())
                                                .offset(
                                                    *fmt.offset(k as isize) as libc::c_uchar as libc::c_int
                                                        as isize,
                                                ) as libc::c_int
                                                & _ISdigit as libc::c_int as libc::c_ushort as libc::c_int
                                                != 0)
                                            {
                                                current_block = 2569451025026770673;
                                                break;
                                            }
                                            if s == 80 as libc::c_int {
                                                current_block = 4701464773222888860;
                                                break;
                                            }
                                            let fresh8 = k;
                                            k = k + 1;
                                            let fresh9 = s;
                                            s = s + 1;
                                            str[fresh9 as usize] = *fmt.offset(fresh8 as isize);
                                        }
                                        match current_block {
                                            4701464773222888860 => {}
                                            _ => {
                                                str[s as usize] = '\0' as i32 as libc::c_char;
                                                if !(_glp_str2int(str.as_mut_ptr(), &mut (*dsa).fmt_w) != 0)
                                                {
                                                    if 1 as libc::c_int <= (*dsa).fmt_w
                                                        && (*dsa).fmt_w <= 255 as libc::c_int
                                                    {
                                                        (*dsa).fmt_d = 0 as libc::c_int;
                                                        if *fmt.offset(k as isize) as libc::c_int == '.' as i32 {
                                                            k += 1;
                                                            k;
                                                            if *(*__ctype_b_loc())
                                                                .offset(
                                                                    *fmt.offset(k as isize) as libc::c_uchar as libc::c_int
                                                                        as isize,
                                                                ) as libc::c_int
                                                                & _ISdigit as libc::c_int as libc::c_ushort as libc::c_int
                                                                == 0
                                                            {
                                                                current_block = 4701464773222888860;
                                                            } else {
                                                                s = 0 as libc::c_int;
                                                                loop {
                                                                    if !(*(*__ctype_b_loc())
                                                                        .offset(
                                                                            *fmt.offset(k as isize) as libc::c_uchar as libc::c_int
                                                                                as isize,
                                                                        ) as libc::c_int
                                                                        & _ISdigit as libc::c_int as libc::c_ushort as libc::c_int
                                                                        != 0)
                                                                    {
                                                                        current_block = 7226443171521532240;
                                                                        break;
                                                                    }
                                                                    if s == 80 as libc::c_int {
                                                                        current_block = 4701464773222888860;
                                                                        break;
                                                                    }
                                                                    let fresh10 = k;
                                                                    k = k + 1;
                                                                    let fresh11 = s;
                                                                    s = s + 1;
                                                                    str[fresh11 as usize] = *fmt.offset(fresh10 as isize);
                                                                }
                                                                match current_block {
                                                                    4701464773222888860 => {}
                                                                    _ => {
                                                                        str[s as usize] = '\0' as i32 as libc::c_char;
                                                                        if _glp_str2int(str.as_mut_ptr(), &mut (*dsa).fmt_d) != 0 {
                                                                            current_block = 4701464773222888860;
                                                                        } else if !(0 as libc::c_int <= (*dsa).fmt_d
                                                                            && (*dsa).fmt_d <= 255 as libc::c_int)
                                                                        {
                                                                            current_block = 4701464773222888860;
                                                                        } else {
                                                                            current_block = 721385680381463314;
                                                                        }
                                                                    }
                                                                }
                                                            }
                                                        } else {
                                                            current_block = 721385680381463314;
                                                        }
                                                        match current_block {
                                                            4701464773222888860 => {}
                                                            _ => {
                                                                if *fmt.offset(k as isize) as libc::c_int == ')' as i32
                                                                    && *fmt.offset((k + 1 as libc::c_int) as isize)
                                                                        as libc::c_int == '\0' as i32
                                                                {
                                                                    return 0 as libc::c_int;
                                                                }
                                                            }
                                                        }
                                                    }
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
    glp_printf(
        b"hbm_read_mat: format '%s' not recognised\n\0" as *const u8
            as *const libc::c_char,
        fmt,
    );
    return 1 as libc::c_int;
}
unsafe extern "C" fn read_int_array(
    mut dsa: *mut dsa,
    mut name: *mut libc::c_char,
    mut fmt: *mut libc::c_char,
    mut n: libc::c_int,
    mut val: *mut libc::c_int,
) -> libc::c_int {
    let mut k: libc::c_int = 0;
    let mut pos: libc::c_int = 0;
    let mut str: [libc::c_char; 81] = [0; 81];
    if parse_fmt(dsa, fmt) != 0 {
        return 1 as libc::c_int;
    }
    if !((*dsa).fmt_f == 'I' as i32 && (*dsa).fmt_w <= 80 as libc::c_int
        && (*dsa).fmt_k * (*dsa).fmt_w <= 80 as libc::c_int)
    {
        glp_printf(
            b"%s:%d: can't read array '%s' - invalid format '%s'\n\0" as *const u8
                as *const libc::c_char,
            (*dsa).fname,
            (*dsa).seqn,
            name,
            fmt,
        );
        return 1 as libc::c_int;
    }
    k = 1 as libc::c_int;
    pos = 2147483647 as libc::c_int;
    while k <= n {
        if pos >= (*dsa).fmt_k {
            if read_card(dsa) != 0 {
                return 1 as libc::c_int;
            }
            pos = 0 as libc::c_int;
        }
        memcpy(
            str.as_mut_ptr() as *mut libc::c_void,
            ((*dsa).card).as_mut_ptr().offset(((*dsa).fmt_w * pos) as isize)
                as *const libc::c_void,
            (*dsa).fmt_w as libc::c_ulong,
        );
        str[(*dsa).fmt_w as usize] = '\0' as i32 as libc::c_char;
        _glp_strspx(str.as_mut_ptr());
        if _glp_str2int(str.as_mut_ptr(), &mut *val.offset(k as isize)) != 0 {
            glp_printf(
                b"%s:%d: can't read array '%s' - invalid value '%s'\n\0" as *const u8
                    as *const libc::c_char,
                (*dsa).fname,
                (*dsa).seqn,
                name,
                str.as_mut_ptr(),
            );
            return 1 as libc::c_int;
        }
        k += 1;
        k;
        pos += 1;
        pos;
    }
    return 0 as libc::c_int;
}
unsafe extern "C" fn read_real_array(
    mut dsa: *mut dsa,
    mut name: *mut libc::c_char,
    mut fmt: *mut libc::c_char,
    mut n: libc::c_int,
    mut val: *mut libc::c_double,
) -> libc::c_int {
    let mut k: libc::c_int = 0;
    let mut pos: libc::c_int = 0;
    let mut str: [libc::c_char; 81] = [0; 81];
    let mut ptr: *mut libc::c_char = 0 as *mut libc::c_char;
    if parse_fmt(dsa, fmt) != 0 {
        return 1 as libc::c_int;
    }
    if !((*dsa).fmt_f != 'I' as i32 && (*dsa).fmt_w <= 80 as libc::c_int
        && (*dsa).fmt_k * (*dsa).fmt_w <= 80 as libc::c_int)
    {
        glp_printf(
            b"%s:%d: can't read array '%s' - invalid format '%s'\n\0" as *const u8
                as *const libc::c_char,
            (*dsa).fname,
            (*dsa).seqn,
            name,
            fmt,
        );
        return 1 as libc::c_int;
    }
    k = 1 as libc::c_int;
    pos = 2147483647 as libc::c_int;
    while k <= n {
        if pos >= (*dsa).fmt_k {
            if read_card(dsa) != 0 {
                return 1 as libc::c_int;
            }
            pos = 0 as libc::c_int;
        }
        memcpy(
            str.as_mut_ptr() as *mut libc::c_void,
            ((*dsa).card).as_mut_ptr().offset(((*dsa).fmt_w * pos) as isize)
                as *const libc::c_void,
            (*dsa).fmt_w as libc::c_ulong,
        );
        str[(*dsa).fmt_w as usize] = '\0' as i32 as libc::c_char;
        _glp_strspx(str.as_mut_ptr());
        if (strchr(str.as_mut_ptr(), '.' as i32)).is_null()
            && strcmp(str.as_mut_ptr(), b"0\0" as *const u8 as *const libc::c_char) != 0
        {
            glp_printf(
                b"%s(%d): can't read array '%s' - value '%s' has no decimal point\n\0"
                    as *const u8 as *const libc::c_char,
                (*dsa).fname,
                (*dsa).seqn,
                name,
                str.as_mut_ptr(),
            );
            return 1 as libc::c_int;
        }
        ptr = str.as_mut_ptr();
        while *ptr != 0 {
            *ptr = ({
                let mut __res: libc::c_int = 0;
                if ::std::mem::size_of::<libc::c_uchar>() as libc::c_ulong
                    > 1 as libc::c_int as libc::c_ulong
                {
                    if 0 != 0 {
                        let mut __c: libc::c_int = *ptr as libc::c_uchar as libc::c_int;
                        __res = if __c < -(128 as libc::c_int)
                            || __c > 255 as libc::c_int
                        {
                            __c
                        } else {
                            *(*__ctype_toupper_loc()).offset(__c as isize)
                        };
                    } else {
                        __res = toupper(*ptr as libc::c_uchar as libc::c_int);
                    }
                } else {
                    __res = *(*__ctype_toupper_loc())
                        .offset(*ptr as libc::c_uchar as libc::c_int as isize);
                }
                __res
            }) as libc::c_char;
            ptr = ptr.offset(1);
            ptr;
        }
        ptr = strchr(str.as_mut_ptr(), 'D' as i32);
        if !ptr.is_null() {
            *ptr = 'E' as i32 as libc::c_char;
        }
        ptr = strchr(str.as_mut_ptr().offset(1 as libc::c_int as isize), '+' as i32);
        if ptr.is_null() {
            ptr = strchr(str.as_mut_ptr().offset(1 as libc::c_int as isize), '-' as i32);
        }
        if !ptr.is_null()
            && *ptr.offset(-(1 as libc::c_int as isize)) as libc::c_int != 'E' as i32
        {
            (strlen(str.as_mut_ptr()) < 80 as libc::c_int as libc::c_ulong
                || {
                    glp_assert_(
                        b"strlen(str) < 80\0" as *const u8 as *const libc::c_char,
                        b"misc/hbm.c\0" as *const u8 as *const libc::c_char,
                        318 as libc::c_int,
                    );
                    1 as libc::c_int != 0
                }) as libc::c_int;
            memmove(
                ptr.offset(1 as libc::c_int as isize) as *mut libc::c_void,
                ptr as *const libc::c_void,
                (strlen(ptr)).wrapping_add(1 as libc::c_int as libc::c_ulong),
            );
            *ptr = 'E' as i32 as libc::c_char;
        }
        if _glp_str2num(str.as_mut_ptr(), &mut *val.offset(k as isize)) != 0 {
            glp_printf(
                b"%s:%d: can't read array '%s' - invalid value '%s'\n\0" as *const u8
                    as *const libc::c_char,
                (*dsa).fname,
                (*dsa).seqn,
                name,
                str.as_mut_ptr(),
            );
            return 1 as libc::c_int;
        }
        k += 1;
        k;
        pos += 1;
        pos;
    }
    return 0 as libc::c_int;
}
pub unsafe extern "C" fn _glp_hbm_read_mat(mut fname: *const libc::c_char) -> *mut HBM {
    let mut current_block: u64;
    let mut _dsa: dsa = dsa {
        fname: 0 as *const libc::c_char,
        fp: 0 as *mut FILE,
        seqn: 0,
        card: [0; 81],
        fmt_p: 0,
        fmt_k: 0,
        fmt_f: 0,
        fmt_w: 0,
        fmt_d: 0,
    };
    let mut dsa: *mut dsa = &mut _dsa;
    let mut hbm: *mut HBM = 0 as *mut HBM;
    (*dsa).fname = fname;
    glp_printf(
        b"hbm_read_mat: reading matrix from '%s'...\n\0" as *const u8
            as *const libc::c_char,
        (*dsa).fname,
    );
    (*dsa).fp = fopen((*dsa).fname, b"r\0" as *const u8 as *const libc::c_char);
    if ((*dsa).fp).is_null() {
        glp_printf(
            b"hbm_read_mat: unable to open '%s' - %s\n\0" as *const u8
                as *const libc::c_char,
            (*dsa).fname,
            _glp_xstrerr(*__errno_location()),
        );
    } else {
        (*dsa).seqn = 0 as libc::c_int;
        hbm = glp_alloc(
            1 as libc::c_int,
            ::std::mem::size_of::<HBM>() as libc::c_ulong as libc::c_int,
        ) as *mut HBM;
        memset(
            hbm as *mut libc::c_void,
            0 as libc::c_int,
            ::std::mem::size_of::<HBM>() as libc::c_ulong,
        );
        if !(read_card(dsa) != 0) {
            memcpy(
                ((*hbm).title).as_mut_ptr() as *mut libc::c_void,
                ((*dsa).card).as_mut_ptr() as *const libc::c_void,
                72 as libc::c_int as libc::c_ulong,
            );
            (*hbm).title[72 as libc::c_int as usize] = '\0' as i32 as libc::c_char;
            _glp_strtrim(((*hbm).title).as_mut_ptr());
            glp_printf(
                b"%s\n\0" as *const u8 as *const libc::c_char,
                ((*hbm).title).as_mut_ptr(),
            );
            memcpy(
                ((*hbm).key).as_mut_ptr() as *mut libc::c_void,
                ((*dsa).card).as_mut_ptr().offset(72 as libc::c_int as isize)
                    as *const libc::c_void,
                8 as libc::c_int as libc::c_ulong,
            );
            (*hbm).key[8 as libc::c_int as usize] = '\0' as i32 as libc::c_char;
            _glp_strspx(((*hbm).key).as_mut_ptr());
            glp_printf(
                b"key = %s\n\0" as *const u8 as *const libc::c_char,
                ((*hbm).key).as_mut_ptr(),
            );
            if !(read_card(dsa) != 0) {
                if !(scan_int(
                    dsa,
                    b"totcrd\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
                    0 as libc::c_int,
                    14 as libc::c_int,
                    &mut (*hbm).totcrd,
                ) != 0)
                {
                    if !(scan_int(
                        dsa,
                        b"ptrcrd\0" as *const u8 as *const libc::c_char
                            as *mut libc::c_char,
                        14 as libc::c_int,
                        14 as libc::c_int,
                        &mut (*hbm).ptrcrd,
                    ) != 0)
                    {
                        if !(scan_int(
                            dsa,
                            b"indcrd\0" as *const u8 as *const libc::c_char
                                as *mut libc::c_char,
                            28 as libc::c_int,
                            14 as libc::c_int,
                            &mut (*hbm).indcrd,
                        ) != 0)
                        {
                            if !(scan_int(
                                dsa,
                                b"valcrd\0" as *const u8 as *const libc::c_char
                                    as *mut libc::c_char,
                                42 as libc::c_int,
                                14 as libc::c_int,
                                &mut (*hbm).valcrd,
                            ) != 0)
                            {
                                if !(scan_int(
                                    dsa,
                                    b"rhscrd\0" as *const u8 as *const libc::c_char
                                        as *mut libc::c_char,
                                    56 as libc::c_int,
                                    14 as libc::c_int,
                                    &mut (*hbm).rhscrd,
                                ) != 0)
                                {
                                    glp_printf(
                                        b"totcrd = %d; ptrcrd = %d; indcrd = %d; valcrd = %d; rhscrd = %d\n\0"
                                            as *const u8 as *const libc::c_char,
                                        (*hbm).totcrd,
                                        (*hbm).ptrcrd,
                                        (*hbm).indcrd,
                                        (*hbm).valcrd,
                                        (*hbm).rhscrd,
                                    );
                                    if !(read_card(dsa) != 0) {
                                        memcpy(
                                            ((*hbm).mxtype).as_mut_ptr() as *mut libc::c_void,
                                            ((*dsa).card).as_mut_ptr() as *const libc::c_void,
                                            3 as libc::c_int as libc::c_ulong,
                                        );
                                        (*hbm)
                                            .mxtype[3 as libc::c_int
                                            as usize] = '\0' as i32 as libc::c_char;
                                        if (strchr(
                                            b"RCP\0" as *const u8 as *const libc::c_char,
                                            (*hbm).mxtype[0 as libc::c_int as usize] as libc::c_int,
                                        ))
                                            .is_null()
                                            || (strchr(
                                                b"SUHZR\0" as *const u8 as *const libc::c_char,
                                                (*hbm).mxtype[1 as libc::c_int as usize] as libc::c_int,
                                            ))
                                                .is_null()
                                            || (strchr(
                                                b"AE\0" as *const u8 as *const libc::c_char,
                                                (*hbm).mxtype[2 as libc::c_int as usize] as libc::c_int,
                                            ))
                                                .is_null()
                                        {
                                            glp_printf(
                                                b"%s:%d: matrix type '%s' not recognised\n\0" as *const u8
                                                    as *const libc::c_char,
                                                (*dsa).fname,
                                                (*dsa).seqn,
                                                ((*hbm).mxtype).as_mut_ptr(),
                                            );
                                        } else if !(scan_int(
                                            dsa,
                                            b"nrow\0" as *const u8 as *const libc::c_char
                                                as *mut libc::c_char,
                                            14 as libc::c_int,
                                            14 as libc::c_int,
                                            &mut (*hbm).nrow,
                                        ) != 0)
                                        {
                                            if !(scan_int(
                                                dsa,
                                                b"ncol\0" as *const u8 as *const libc::c_char
                                                    as *mut libc::c_char,
                                                28 as libc::c_int,
                                                14 as libc::c_int,
                                                &mut (*hbm).ncol,
                                            ) != 0)
                                            {
                                                if !(scan_int(
                                                    dsa,
                                                    b"nnzero\0" as *const u8 as *const libc::c_char
                                                        as *mut libc::c_char,
                                                    42 as libc::c_int,
                                                    14 as libc::c_int,
                                                    &mut (*hbm).nnzero,
                                                ) != 0)
                                                {
                                                    if !(scan_int(
                                                        dsa,
                                                        b"neltvl\0" as *const u8 as *const libc::c_char
                                                            as *mut libc::c_char,
                                                        56 as libc::c_int,
                                                        14 as libc::c_int,
                                                        &mut (*hbm).neltvl,
                                                    ) != 0)
                                                    {
                                                        glp_printf(
                                                            b"mxtype = %s; nrow = %d; ncol = %d; nnzero = %d; neltvl = %d\n\0"
                                                                as *const u8 as *const libc::c_char,
                                                            ((*hbm).mxtype).as_mut_ptr(),
                                                            (*hbm).nrow,
                                                            (*hbm).ncol,
                                                            (*hbm).nnzero,
                                                            (*hbm).neltvl,
                                                        );
                                                        if !(read_card(dsa) != 0) {
                                                            memcpy(
                                                                ((*hbm).ptrfmt).as_mut_ptr() as *mut libc::c_void,
                                                                ((*dsa).card).as_mut_ptr() as *const libc::c_void,
                                                                16 as libc::c_int as libc::c_ulong,
                                                            );
                                                            (*hbm)
                                                                .ptrfmt[16 as libc::c_int
                                                                as usize] = '\0' as i32 as libc::c_char;
                                                            _glp_strspx(((*hbm).ptrfmt).as_mut_ptr());
                                                            memcpy(
                                                                ((*hbm).indfmt).as_mut_ptr() as *mut libc::c_void,
                                                                ((*dsa).card)
                                                                    .as_mut_ptr()
                                                                    .offset(16 as libc::c_int as isize) as *const libc::c_void,
                                                                16 as libc::c_int as libc::c_ulong,
                                                            );
                                                            (*hbm)
                                                                .indfmt[16 as libc::c_int
                                                                as usize] = '\0' as i32 as libc::c_char;
                                                            _glp_strspx(((*hbm).indfmt).as_mut_ptr());
                                                            memcpy(
                                                                ((*hbm).valfmt).as_mut_ptr() as *mut libc::c_void,
                                                                ((*dsa).card)
                                                                    .as_mut_ptr()
                                                                    .offset(32 as libc::c_int as isize) as *const libc::c_void,
                                                                20 as libc::c_int as libc::c_ulong,
                                                            );
                                                            (*hbm)
                                                                .valfmt[20 as libc::c_int
                                                                as usize] = '\0' as i32 as libc::c_char;
                                                            _glp_strspx(((*hbm).valfmt).as_mut_ptr());
                                                            memcpy(
                                                                ((*hbm).rhsfmt).as_mut_ptr() as *mut libc::c_void,
                                                                ((*dsa).card)
                                                                    .as_mut_ptr()
                                                                    .offset(52 as libc::c_int as isize) as *const libc::c_void,
                                                                20 as libc::c_int as libc::c_ulong,
                                                            );
                                                            (*hbm)
                                                                .rhsfmt[20 as libc::c_int
                                                                as usize] = '\0' as i32 as libc::c_char;
                                                            _glp_strspx(((*hbm).rhsfmt).as_mut_ptr());
                                                            glp_printf(
                                                                b"ptrfmt = %s; indfmt = %s; valfmt = %s; rhsfmt = %s\n\0"
                                                                    as *const u8 as *const libc::c_char,
                                                                ((*hbm).ptrfmt).as_mut_ptr(),
                                                                ((*hbm).indfmt).as_mut_ptr(),
                                                                ((*hbm).valfmt).as_mut_ptr(),
                                                                ((*hbm).rhsfmt).as_mut_ptr(),
                                                            );
                                                            if (*hbm).rhscrd <= 0 as libc::c_int {
                                                                strcpy(
                                                                    ((*hbm).rhstyp).as_mut_ptr(),
                                                                    b"???\0" as *const u8 as *const libc::c_char,
                                                                );
                                                                (*hbm).nrhs = 0 as libc::c_int;
                                                                (*hbm).nrhsix = 0 as libc::c_int;
                                                                current_block = 7226443171521532240;
                                                            } else if read_card(dsa) != 0 {
                                                                current_block = 12074316055261152827;
                                                            } else {
                                                                memcpy(
                                                                    ((*hbm).rhstyp).as_mut_ptr() as *mut libc::c_void,
                                                                    ((*dsa).card).as_mut_ptr() as *const libc::c_void,
                                                                    3 as libc::c_int as libc::c_ulong,
                                                                );
                                                                (*hbm)
                                                                    .rhstyp[3 as libc::c_int
                                                                    as usize] = '\0' as i32 as libc::c_char;
                                                                if scan_int(
                                                                    dsa,
                                                                    b"nrhs\0" as *const u8 as *const libc::c_char
                                                                        as *mut libc::c_char,
                                                                    14 as libc::c_int,
                                                                    14 as libc::c_int,
                                                                    &mut (*hbm).nrhs,
                                                                ) != 0
                                                                {
                                                                    current_block = 12074316055261152827;
                                                                } else if scan_int(
                                                                    dsa,
                                                                    b"nrhsix\0" as *const u8 as *const libc::c_char
                                                                        as *mut libc::c_char,
                                                                    28 as libc::c_int,
                                                                    14 as libc::c_int,
                                                                    &mut (*hbm).nrhsix,
                                                                ) != 0
                                                                {
                                                                    current_block = 12074316055261152827;
                                                                } else {
                                                                    glp_printf(
                                                                        b"rhstyp = '%s'; nrhs = %d; nrhsix = %d\n\0" as *const u8
                                                                            as *const libc::c_char,
                                                                        ((*hbm).rhstyp).as_mut_ptr(),
                                                                        (*hbm).nrhs,
                                                                        (*hbm).nrhsix,
                                                                    );
                                                                    current_block = 7226443171521532240;
                                                                }
                                                            }
                                                            match current_block {
                                                                12074316055261152827 => {}
                                                                _ => {
                                                                    (*hbm)
                                                                        .colptr = glp_alloc(
                                                                        1 as libc::c_int + (*hbm).ncol + 1 as libc::c_int,
                                                                        ::std::mem::size_of::<libc::c_int>() as libc::c_ulong
                                                                            as libc::c_int,
                                                                    ) as *mut libc::c_int;
                                                                    if !(read_int_array(
                                                                        dsa,
                                                                        b"colptr\0" as *const u8 as *const libc::c_char
                                                                            as *mut libc::c_char,
                                                                        ((*hbm).ptrfmt).as_mut_ptr(),
                                                                        (*hbm).ncol + 1 as libc::c_int,
                                                                        (*hbm).colptr,
                                                                    ) != 0)
                                                                    {
                                                                        (*hbm)
                                                                            .rowind = glp_alloc(
                                                                            1 as libc::c_int + (*hbm).nnzero,
                                                                            ::std::mem::size_of::<libc::c_int>() as libc::c_ulong
                                                                                as libc::c_int,
                                                                        ) as *mut libc::c_int;
                                                                        if !(read_int_array(
                                                                            dsa,
                                                                            b"rowind\0" as *const u8 as *const libc::c_char
                                                                                as *mut libc::c_char,
                                                                            ((*hbm).indfmt).as_mut_ptr(),
                                                                            (*hbm).nnzero,
                                                                            (*hbm).rowind,
                                                                        ) != 0)
                                                                        {
                                                                            if (*hbm).valcrd <= 0 as libc::c_int {
                                                                                current_block = 4284134429355460312;
                                                                            } else {
                                                                                if (*hbm).mxtype[2 as libc::c_int as usize] as libc::c_int
                                                                                    == 'A' as i32
                                                                                {
                                                                                    (*hbm)
                                                                                        .values = glp_alloc(
                                                                                        1 as libc::c_int + (*hbm).nnzero,
                                                                                        ::std::mem::size_of::<libc::c_double>() as libc::c_ulong
                                                                                            as libc::c_int,
                                                                                    ) as *mut libc::c_double;
                                                                                    if read_real_array(
                                                                                        dsa,
                                                                                        b"values\0" as *const u8 as *const libc::c_char
                                                                                            as *mut libc::c_char,
                                                                                        ((*hbm).valfmt).as_mut_ptr(),
                                                                                        (*hbm).nnzero,
                                                                                        (*hbm).values,
                                                                                    ) != 0
                                                                                    {
                                                                                        current_block = 12074316055261152827;
                                                                                    } else {
                                                                                        current_block = 13678349939556791712;
                                                                                    }
                                                                                } else {
                                                                                    (*hbm)
                                                                                        .values = glp_alloc(
                                                                                        1 as libc::c_int + (*hbm).neltvl,
                                                                                        ::std::mem::size_of::<libc::c_double>() as libc::c_ulong
                                                                                            as libc::c_int,
                                                                                    ) as *mut libc::c_double;
                                                                                    if read_real_array(
                                                                                        dsa,
                                                                                        b"values\0" as *const u8 as *const libc::c_char
                                                                                            as *mut libc::c_char,
                                                                                        ((*hbm).valfmt).as_mut_ptr(),
                                                                                        (*hbm).neltvl,
                                                                                        (*hbm).values,
                                                                                    ) != 0
                                                                                    {
                                                                                        current_block = 12074316055261152827;
                                                                                    } else {
                                                                                        current_block = 13678349939556791712;
                                                                                    }
                                                                                }
                                                                                match current_block {
                                                                                    12074316055261152827 => {}
                                                                                    _ => {
                                                                                        if (*hbm).nrhs <= 0 as libc::c_int {
                                                                                            current_block = 4284134429355460312;
                                                                                        } else {
                                                                                            if (*hbm).rhstyp[0 as libc::c_int as usize] as libc::c_int
                                                                                                == 'F' as i32
                                                                                            {
                                                                                                (*hbm).nrhsvl = (*hbm).nrow * (*hbm).nrhs;
                                                                                                (*hbm)
                                                                                                    .rhsval = glp_alloc(
                                                                                                    1 as libc::c_int + (*hbm).nrhsvl,
                                                                                                    ::std::mem::size_of::<libc::c_double>() as libc::c_ulong
                                                                                                        as libc::c_int,
                                                                                                ) as *mut libc::c_double;
                                                                                                if read_real_array(
                                                                                                    dsa,
                                                                                                    b"rhsval\0" as *const u8 as *const libc::c_char
                                                                                                        as *mut libc::c_char,
                                                                                                    ((*hbm).rhsfmt).as_mut_ptr(),
                                                                                                    (*hbm).nrhsvl,
                                                                                                    (*hbm).rhsval,
                                                                                                ) != 0
                                                                                                {
                                                                                                    current_block = 12074316055261152827;
                                                                                                } else {
                                                                                                    current_block = 981995395831942902;
                                                                                                }
                                                                                            } else if (*hbm).rhstyp[0 as libc::c_int as usize]
                                                                                                as libc::c_int == 'M' as i32
                                                                                                && (*hbm).mxtype[2 as libc::c_int as usize] as libc::c_int
                                                                                                    == 'A' as i32
                                                                                            {
                                                                                                (*hbm)
                                                                                                    .rhsptr = glp_alloc(
                                                                                                    1 as libc::c_int + (*hbm).nrhs + 1 as libc::c_int,
                                                                                                    ::std::mem::size_of::<libc::c_int>() as libc::c_ulong
                                                                                                        as libc::c_int,
                                                                                                ) as *mut libc::c_int;
                                                                                                if read_int_array(
                                                                                                    dsa,
                                                                                                    b"rhsptr\0" as *const u8 as *const libc::c_char
                                                                                                        as *mut libc::c_char,
                                                                                                    ((*hbm).ptrfmt).as_mut_ptr(),
                                                                                                    (*hbm).nrhs + 1 as libc::c_int,
                                                                                                    (*hbm).rhsptr,
                                                                                                ) != 0
                                                                                                {
                                                                                                    current_block = 12074316055261152827;
                                                                                                } else {
                                                                                                    (*hbm)
                                                                                                        .rhsind = glp_alloc(
                                                                                                        1 as libc::c_int + (*hbm).nrhsix,
                                                                                                        ::std::mem::size_of::<libc::c_int>() as libc::c_ulong
                                                                                                            as libc::c_int,
                                                                                                    ) as *mut libc::c_int;
                                                                                                    if read_int_array(
                                                                                                        dsa,
                                                                                                        b"rhsind\0" as *const u8 as *const libc::c_char
                                                                                                            as *mut libc::c_char,
                                                                                                        ((*hbm).indfmt).as_mut_ptr(),
                                                                                                        (*hbm).nrhsix,
                                                                                                        (*hbm).rhsind,
                                                                                                    ) != 0
                                                                                                    {
                                                                                                        current_block = 12074316055261152827;
                                                                                                    } else {
                                                                                                        (*hbm)
                                                                                                            .rhsval = glp_alloc(
                                                                                                            1 as libc::c_int + (*hbm).nrhsix,
                                                                                                            ::std::mem::size_of::<libc::c_double>() as libc::c_ulong
                                                                                                                as libc::c_int,
                                                                                                        ) as *mut libc::c_double;
                                                                                                        if read_real_array(
                                                                                                            dsa,
                                                                                                            b"rhsval\0" as *const u8 as *const libc::c_char
                                                                                                                as *mut libc::c_char,
                                                                                                            ((*hbm).rhsfmt).as_mut_ptr(),
                                                                                                            (*hbm).nrhsix,
                                                                                                            (*hbm).rhsval,
                                                                                                        ) != 0
                                                                                                        {
                                                                                                            current_block = 12074316055261152827;
                                                                                                        } else {
                                                                                                            current_block = 981995395831942902;
                                                                                                        }
                                                                                                    }
                                                                                                }
                                                                                            } else if (*hbm).rhstyp[0 as libc::c_int as usize]
                                                                                                as libc::c_int == 'M' as i32
                                                                                                && (*hbm).mxtype[2 as libc::c_int as usize] as libc::c_int
                                                                                                    == 'E' as i32
                                                                                            {
                                                                                                (*hbm)
                                                                                                    .rhsval = glp_alloc(
                                                                                                    1 as libc::c_int + (*hbm).nrhsvl,
                                                                                                    ::std::mem::size_of::<libc::c_double>() as libc::c_ulong
                                                                                                        as libc::c_int,
                                                                                                ) as *mut libc::c_double;
                                                                                                if read_real_array(
                                                                                                    dsa,
                                                                                                    b"rhsval\0" as *const u8 as *const libc::c_char
                                                                                                        as *mut libc::c_char,
                                                                                                    ((*hbm).rhsfmt).as_mut_ptr(),
                                                                                                    (*hbm).nrhsvl,
                                                                                                    (*hbm).rhsval,
                                                                                                ) != 0
                                                                                                {
                                                                                                    current_block = 12074316055261152827;
                                                                                                } else {
                                                                                                    current_block = 981995395831942902;
                                                                                                }
                                                                                            } else {
                                                                                                glp_printf(
                                                                                                    b"%s:%d: right-hand side type '%c' not recognised\n\0"
                                                                                                        as *const u8 as *const libc::c_char,
                                                                                                    (*dsa).fname,
                                                                                                    (*dsa).seqn,
                                                                                                    (*hbm).rhstyp[0 as libc::c_int as usize] as libc::c_int,
                                                                                                );
                                                                                                current_block = 12074316055261152827;
                                                                                            }
                                                                                            match current_block {
                                                                                                12074316055261152827 => {}
                                                                                                _ => {
                                                                                                    if (*hbm).rhstyp[1 as libc::c_int as usize] as libc::c_int
                                                                                                        == 'G' as i32
                                                                                                    {
                                                                                                        (*hbm).nguess = (*hbm).nrow * (*hbm).nrhs;
                                                                                                        (*hbm)
                                                                                                            .sguess = glp_alloc(
                                                                                                            1 as libc::c_int + (*hbm).nguess,
                                                                                                            ::std::mem::size_of::<libc::c_double>() as libc::c_ulong
                                                                                                                as libc::c_int,
                                                                                                        ) as *mut libc::c_double;
                                                                                                        if read_real_array(
                                                                                                            dsa,
                                                                                                            b"sguess\0" as *const u8 as *const libc::c_char
                                                                                                                as *mut libc::c_char,
                                                                                                            ((*hbm).rhsfmt).as_mut_ptr(),
                                                                                                            (*hbm).nguess,
                                                                                                            (*hbm).sguess,
                                                                                                        ) != 0
                                                                                                        {
                                                                                                            current_block = 12074316055261152827;
                                                                                                        } else {
                                                                                                            current_block = 1868291631715963762;
                                                                                                        }
                                                                                                    } else {
                                                                                                        current_block = 1868291631715963762;
                                                                                                    }
                                                                                                    match current_block {
                                                                                                        12074316055261152827 => {}
                                                                                                        _ => {
                                                                                                            if (*hbm).rhstyp[2 as libc::c_int as usize] as libc::c_int
                                                                                                                == 'X' as i32
                                                                                                            {
                                                                                                                (*hbm).nexact = (*hbm).nrow * (*hbm).nrhs;
                                                                                                                (*hbm)
                                                                                                                    .xexact = glp_alloc(
                                                                                                                    1 as libc::c_int + (*hbm).nexact,
                                                                                                                    ::std::mem::size_of::<libc::c_double>() as libc::c_ulong
                                                                                                                        as libc::c_int,
                                                                                                                ) as *mut libc::c_double;
                                                                                                                if read_real_array(
                                                                                                                    dsa,
                                                                                                                    b"xexact\0" as *const u8 as *const libc::c_char
                                                                                                                        as *mut libc::c_char,
                                                                                                                    ((*hbm).rhsfmt).as_mut_ptr(),
                                                                                                                    (*hbm).nexact,
                                                                                                                    (*hbm).xexact,
                                                                                                                ) != 0
                                                                                                                {
                                                                                                                    current_block = 12074316055261152827;
                                                                                                                } else {
                                                                                                                    current_block = 4284134429355460312;
                                                                                                                }
                                                                                                            } else {
                                                                                                                current_block = 4284134429355460312;
                                                                                                            }
                                                                                                        }
                                                                                                    }
                                                                                                }
                                                                                            }
                                                                                        }
                                                                                    }
                                                                                }
                                                                            }
                                                                            match current_block {
                                                                                12074316055261152827 => {}
                                                                                _ => {
                                                                                    glp_printf(
                                                                                        b"hbm_read_mat: %d cards were read\n\0" as *const u8
                                                                                            as *const libc::c_char,
                                                                                        (*dsa).seqn,
                                                                                    );
                                                                                    fclose((*dsa).fp);
                                                                                    return hbm;
                                                                                }
                                                                            }
                                                                        }
                                                                    }
                                                                }
                                                            }
                                                        }
                                                    }
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
    if !hbm.is_null() {
        if !((*hbm).colptr).is_null() {
            glp_free((*hbm).colptr as *mut libc::c_void);
        }
        if !((*hbm).rowind).is_null() {
            glp_free((*hbm).rowind as *mut libc::c_void);
        }
        if !((*hbm).rhsptr).is_null() {
            glp_free((*hbm).rhsptr as *mut libc::c_void);
        }
        if !((*hbm).rhsind).is_null() {
            glp_free((*hbm).rhsind as *mut libc::c_void);
        }
        if !((*hbm).values).is_null() {
            glp_free((*hbm).values as *mut libc::c_void);
        }
        if !((*hbm).rhsval).is_null() {
            glp_free((*hbm).rhsval as *mut libc::c_void);
        }
        if !((*hbm).sguess).is_null() {
            glp_free((*hbm).sguess as *mut libc::c_void);
        }
        if !((*hbm).xexact).is_null() {
            glp_free((*hbm).xexact as *mut libc::c_void);
        }
        glp_free(hbm as *mut libc::c_void);
    }
    if !((*dsa).fp).is_null() {
        fclose((*dsa).fp);
    }
    return 0 as *mut HBM;
}
pub unsafe extern "C" fn _glp_hbm_free_mat(mut hbm: *mut HBM) {
    if !((*hbm).colptr).is_null() {
        glp_free((*hbm).colptr as *mut libc::c_void);
    }
    if !((*hbm).rowind).is_null() {
        glp_free((*hbm).rowind as *mut libc::c_void);
    }
    if !((*hbm).rhsptr).is_null() {
        glp_free((*hbm).rhsptr as *mut libc::c_void);
    }
    if !((*hbm).rhsind).is_null() {
        glp_free((*hbm).rhsind as *mut libc::c_void);
    }
    if !((*hbm).values).is_null() {
        glp_free((*hbm).values as *mut libc::c_void);
    }
    if !((*hbm).rhsval).is_null() {
        glp_free((*hbm).rhsval as *mut libc::c_void);
    }
    if !((*hbm).sguess).is_null() {
        glp_free((*hbm).sguess as *mut libc::c_void);
    }
    if !((*hbm).xexact).is_null() {
        glp_free((*hbm).xexact as *mut libc::c_void);
    }
    glp_free(hbm as *mut libc::c_void);
}
