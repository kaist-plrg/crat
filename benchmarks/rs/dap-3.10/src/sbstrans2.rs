use ::libc;
extern "C" {
    pub type _IO_wide_data;
    pub type _IO_codecvt;
    pub type _IO_marker;
    static mut stderr: *mut FILE;
    fn fprintf(_: *mut FILE, _: *const libc::c_char, _: ...) -> libc::c_int;
    fn fputs(__s: *const libc::c_char, __stream: *mut FILE) -> libc::c_int;
    fn linecpy(l1: *mut libc::c_char, l2: *mut libc::c_char) -> libc::c_int;
    fn linecmp(l1: *mut libc::c_char, l2: *mut libc::c_char) -> libc::c_int;
    fn isby(step: *mut libc::c_char) -> libc::c_int;
    fn getoption(
        step: *mut libc::c_char,
        key: *mut libc::c_char,
        setname: *mut libc::c_char,
        equals: libc::c_int,
    ) -> libc::c_int;
    fn copylist(
        step: *mut libc::c_char,
        key: *mut libc::c_char,
        dapfile: *mut FILE,
    ) -> libc::c_int;
    fn findstatement(step: *mut libc::c_char, key: *mut libc::c_char) -> libc::c_int;
    fn countparts(
        step: *mut libc::c_char,
        setname: *mut libc::c_char,
        dapfile: *mut FILE,
    );
    fn alpha(c: libc::c_int) -> libc::c_int;
    fn exit(_: libc::c_int) -> !;
    fn strcpy(_: *mut libc::c_char, _: *const libc::c_char) -> *mut libc::c_char;
    static mut sbstmp: [libc::c_char; 128];
    static mut sbslineno: libc::c_int;
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
pub static mut sbshaspicts: libc::c_int = 0;
pub unsafe extern "C" fn charttrans(
    mut step: *mut libc::c_char,
    mut dapfile: *mut FILE,
) {
    let mut s: libc::c_int = 0;
    let mut sincr: libc::c_int = 0;
    let mut setname: [libc::c_char; 128] = [0; 128];
    let mut varname: [libc::c_char; 129] = [0; 129];
    s = getoption(
        step,
        b"data\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        setname.as_mut_ptr(),
        1 as libc::c_int,
    );
    if s == 0 {
        strcpy(setname.as_mut_ptr(), sbstmp.as_mut_ptr());
    }
    if isby(step) >= 0 as libc::c_int {
        countparts(step, setname.as_mut_ptr(), dapfile);
    } else {
        fputs(
            b"_sbspictcnt_[_sbsnpicts_] = 1;\n\0" as *const u8 as *const libc::c_char,
            dapfile,
        );
    }
    fprintf(
        dapfile,
        b"_sbspict_[_sbsnpicts_] = histogram(\"%s\", \"\0" as *const u8
            as *const libc::c_char,
        setname.as_mut_ptr(),
    );
    s = findstatement(
        step,
        b"vbar\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    );
    if s != 0 {
        s += linecpy(varname.as_mut_ptr(), step.offset(s as isize)) + 1 as libc::c_int;
        fprintf(
            dapfile,
            b"%s \0" as *const u8 as *const libc::c_char,
            varname.as_mut_ptr(),
        );
        s = getoption(
            step.offset(s as isize),
            b"freq\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            varname.as_mut_ptr(),
            0 as libc::c_int,
        );
        if s != 0 {
            fputs(varname.as_mut_ptr(), dapfile);
        }
        fputs(b"\", \"\0" as *const u8 as *const libc::c_char, dapfile);
    } else {
        fprintf(
            stderr,
            b"sbstrans: before %d: missing vbar statement in proc chart.\n\0"
                as *const u8 as *const libc::c_char,
            sbslineno,
        );
        exit(1 as libc::c_int);
    }
    copylist(
        step,
        b"by\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        dapfile,
    );
    fputs(b"\", \0" as *const u8 as *const libc::c_char, dapfile);
    s = findstatement(
        step,
        b"vbar\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    );
    if getoption(
        step.offset(s as isize),
        b"levels\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        varname.as_mut_ptr(),
        1 as libc::c_int,
    ) != 0
    {
        fputs(varname.as_mut_ptr(), dapfile);
    } else {
        fputs(b"10\0" as *const u8 as *const libc::c_char, dapfile);
    }
    fputs(b", \"\0" as *const u8 as *const libc::c_char, dapfile);
    fputs(b"== \0" as *const u8 as *const libc::c_char, dapfile);
    if getoption(
        step.offset(s as isize),
        b"type\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        varname.as_mut_ptr(),
        1 as libc::c_int,
    ) != 0
    {
        if linecmp(
            varname.as_mut_ptr(),
            b"freq\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        ) == 0
        {
            strcpy(varname.as_mut_ptr(), b"COUNT\0" as *const u8 as *const libc::c_char);
        } else if linecmp(
            varname.as_mut_ptr(),
            b"percent\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        ) == 0
            || linecmp(
                varname.as_mut_ptr(),
                b"pct\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            ) == 0
        {
            strcpy(
                varname.as_mut_ptr(),
                b"PERCENT\0" as *const u8 as *const libc::c_char,
            );
        } else {
            fprintf(
                stderr,
                b"sbstrans: before %d: bad option %s in vbar statement in proc chart.\n\0"
                    as *const u8 as *const libc::c_char,
                sbslineno,
                varname.as_mut_ptr(),
            );
            exit(1 as libc::c_int);
        }
        fputs(varname.as_mut_ptr(), dapfile);
    }
    sincr = getoption(
        step.offset(s as isize),
        b"axis\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        varname.as_mut_ptr(),
        1 as libc::c_int,
    );
    if sincr != 0 {
        if linecmp(
            varname.as_mut_ptr(),
            b"+\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        ) == 0
            || linecmp(
                varname.as_mut_ptr(),
                b"-\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            ) == 0
        {
            s += sincr;
            s
                += linecpy(
                    varname.as_mut_ptr().offset(1 as libc::c_int as isize),
                    step.offset(s as isize),
                ) + 1 as libc::c_int;
        } else {
            s += sincr;
        }
        if '0' as i32 <= *step.offset(s as isize) as libc::c_int
            && *step.offset(s as isize) as libc::c_int <= '9' as i32
            || *step.offset(s as isize) as libc::c_int == '.' as i32
            || *step.offset(s as isize) as libc::c_int == '-' as i32
            || *step.offset(s as isize) as libc::c_int == '+' as i32
        {
            fprintf(
                dapfile,
                b" MINX%s\0" as *const u8 as *const libc::c_char,
                varname.as_mut_ptr(),
            );
            if *step.offset(s as isize) as libc::c_int == '-' as i32
                || *step.offset(s as isize) as libc::c_int == '-' as i32
            {
                s
                    += linecpy(varname.as_mut_ptr(), step.offset(s as isize))
                        + 1 as libc::c_int;
                linecpy(
                    varname.as_mut_ptr().offset(1 as libc::c_int as isize),
                    step.offset(s as isize),
                );
            } else {
                linecpy(varname.as_mut_ptr(), step.offset(s as isize));
            }
        }
        fprintf(
            dapfile,
            b" MAXX%s\0" as *const u8 as *const libc::c_char,
            varname.as_mut_ptr(),
        );
    }
    fprintf(
        dapfile,
        b"\", NULL, %d);\n\0" as *const u8 as *const libc::c_char,
        256 as libc::c_int,
    );
    fputs(
        b"_sbspictpage_[_sbsnpicts_++] = 1;\n\0" as *const u8 as *const libc::c_char,
        dapfile,
    );
    sbshaspicts = 1 as libc::c_int;
}
pub unsafe extern "C" fn plottrans(mut step: *mut libc::c_char, mut dapfile: *mut FILE) {
    let mut s: libc::c_int = 0;
    let mut sincr: libc::c_int = 0;
    let mut by: libc::c_int = 0;
    let mut setname: [libc::c_char; 128] = [0; 128];
    let mut xname: [libc::c_char; 128] = [0; 128];
    let mut yname: [libc::c_char; 128] = [0; 128];
    s = getoption(
        step,
        b"data\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        setname.as_mut_ptr(),
        1 as libc::c_int,
    );
    if s == 0 {
        strcpy(setname.as_mut_ptr(), sbstmp.as_mut_ptr());
    }
    by = isby(step);
    s = 0 as libc::c_int;
    loop {
        sincr = findstatement(
            step.offset(s as isize),
            b"plot\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        );
        if !(sincr != 0) {
            break;
        }
        if by >= 0 as libc::c_int {
            countparts(step, setname.as_mut_ptr(), dapfile);
        } else {
            fputs(
                b"_sbspictcnt_[_sbsnpicts_] = 1;\n\0" as *const u8
                    as *const libc::c_char,
                dapfile,
            );
        }
        s += sincr;
        fprintf(
            dapfile,
            b"_sbspict_[_sbsnpicts_] = plot(\"%s\", \"\0" as *const u8
                as *const libc::c_char,
            setname.as_mut_ptr(),
        );
        s += linecpy(yname.as_mut_ptr(), step.offset(s as isize)) + 1 as libc::c_int;
        if alpha(yname[0 as libc::c_int as usize] as libc::c_int) == 0 {
            fprintf(
                stderr,
                b"sbstrans: before %d: bad vertical variable name in plot statement in proc plot.\n\0"
                    as *const u8 as *const libc::c_char,
                sbslineno,
                yname.as_mut_ptr(),
            );
            exit(1 as libc::c_int);
        }
        if linecmp(
            step.offset(s as isize),
            b"*\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        ) != 0
        {
            fprintf(
                stderr,
                b"sbstrans: before %d: missing * after vertical variable name in plot statement in proc plot.\n\0"
                    as *const u8 as *const libc::c_char,
                sbslineno,
                yname.as_mut_ptr(),
            );
            exit(1 as libc::c_int);
        }
        s += 2 as libc::c_int;
        s += linecpy(xname.as_mut_ptr(), step.offset(s as isize)) + 1 as libc::c_int;
        if alpha(xname[0 as libc::c_int as usize] as libc::c_int) == 0 {
            fprintf(
                stderr,
                b"sbstrans: before %d: bad horizontal variable name in plot statement in proc plot.\n\0"
                    as *const u8 as *const libc::c_char,
                sbslineno,
                xname.as_mut_ptr(),
            );
            exit(1 as libc::c_int);
        }
        fprintf(
            dapfile,
            b"%s %s\", \"\0" as *const u8 as *const libc::c_char,
            xname.as_mut_ptr(),
            yname.as_mut_ptr(),
        );
        copylist(
            step,
            b"by\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            dapfile,
        );
        fputs(b"\", \"\0" as *const u8 as *const libc::c_char, dapfile);
        if *step.offset(s as isize) as libc::c_int == '/' as i32 {
            s += 2 as libc::c_int;
            while *step.offset(s as isize) as libc::c_int != 0
                && *step.offset(s as isize) as libc::c_int != ';' as i32
            {
                s
                    += linecpy(xname.as_mut_ptr(), step.offset(s as isize))
                        + 1 as libc::c_int;
                if linecmp(
                    xname.as_mut_ptr(),
                    b"box\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
                ) == 0
                {
                    fputs(b"== \0" as *const u8 as *const libc::c_char, dapfile);
                } else {
                    fprintf(
                        stderr,
                        b"sbstrans: before %d: bad option %s in plot statement in proc plot.\n\0"
                            as *const u8 as *const libc::c_char,
                        sbslineno,
                        xname.as_mut_ptr(),
                    );
                    exit(1 as libc::c_int);
                }
            }
        }
        fprintf(
            dapfile,
            b"\", NULL, NULL, %d);\n\0" as *const u8 as *const libc::c_char,
            256 as libc::c_int,
        );
        fputs(
            b"_sbspictpage_[_sbsnpicts_++] = 1;\n\0" as *const u8 as *const libc::c_char,
            dapfile,
        );
        sbshaspicts = 1 as libc::c_int;
        s += 2 as libc::c_int;
    }
    if s == 0 {
        fprintf(
            stderr,
            b"sbstrans: before %d: missing plot statement in proc plot.\n\0" as *const u8
                as *const libc::c_char,
            sbslineno,
        );
        exit(1 as libc::c_int);
    }
}
