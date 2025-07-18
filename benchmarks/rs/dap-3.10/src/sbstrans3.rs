use ::libc;
extern "C" {
    pub type _IO_wide_data;
    pub type _IO_codecvt;
    pub type _IO_marker;
    static mut stderr: *mut FILE;
    fn fprintf(_: *mut FILE, _: *const libc::c_char, _: ...) -> libc::c_int;
    fn sscanf(_: *const libc::c_char, _: *const libc::c_char, _: ...) -> libc::c_int;
    fn putc(__c: libc::c_int, __stream: *mut FILE) -> libc::c_int;
    fn fputs(__s: *const libc::c_char, __stream: *mut FILE) -> libc::c_int;
    static mut sbshaspicts: libc::c_int;
    fn linecpy(l1: *mut libc::c_char, l2: *mut libc::c_char) -> libc::c_int;
    fn linecmp(l1: *mut libc::c_char, l2: *mut libc::c_char) -> libc::c_int;
    fn upper(str: *mut libc::c_char);
    fn putlines(
        line: *mut libc::c_char,
        file: *mut FILE,
        term: libc::c_int,
    ) -> libc::c_int;
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
    fn num(c: libc::c_int) -> libc::c_int;
    fn malloc(_: libc::c_ulong) -> *mut libc::c_void;
    fn exit(_: libc::c_int) -> !;
    fn free(__ptr: *mut libc::c_void);
    fn strcpy(_: *mut libc::c_char, _: *const libc::c_char) -> *mut libc::c_char;
    fn strcmp(_: *const libc::c_char, _: *const libc::c_char) -> libc::c_int;
    fn strlen(_: *const libc::c_char) -> libc::c_ulong;
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
pub unsafe extern "C" fn glmtrans(mut step: *mut libc::c_char, mut dapfile: *mut FILE) {
    let mut s: libc::c_int = 0;
    let mut sincr: libc::c_int = 0;
    static mut setname: [libc::c_char; 128] = [0; 128];
    let mut modelstart: libc::c_int = 0;
    static mut response: [libc::c_char; 128] = [0; 128];
    let mut resplen: libc::c_int = 0;
    let mut e: libc::c_int = 0;
    let mut lsmeans: libc::c_int = 0;
    static mut test: [libc::c_char; 128] = [0; 128];
    static mut level: [libc::c_char; 128] = [0; 128];
    let mut term: libc::c_int = 0;
    let mut nomatch: libc::c_int = 0;
    let mut classstart: libc::c_int = 0;
    let mut nbyvars: libc::c_int = 0;
    let mut classvar: libc::c_int = 0;
    let mut c: libc::c_int = 0;
    static mut classname: [libc::c_char; 128] = [0; 128];
    let mut contrastterm: libc::c_uint = 0;
    let mut coeff: *mut libc::c_int = 0 as *mut libc::c_int;
    let mut ncoeff: libc::c_int = 0;
    let mut minus: libc::c_int = 0;
    let mut coeffsum: libc::c_int = 0;
    if getoption(
        step,
        b"data\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        setname.as_mut_ptr(),
        1 as libc::c_int,
    ) == 0
    {
        strcpy(setname.as_mut_ptr(), sbstmp.as_mut_ptr());
    }
    modelstart = findstatement(
        step,
        b"model\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    );
    if modelstart != 0 {
        resplen = linecpy(response.as_mut_ptr(), step.offset(modelstart as isize));
    } else {
        fprintf(
            stderr,
            b"sbstrans: before %d: missing model statement in proc glm\n\0" as *const u8
                as *const libc::c_char,
            sbslineno,
        );
        exit(1 as libc::c_int);
    }
    classstart = findstatement(
        step,
        b"class\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    );
    if classstart == 0 {
        fprintf(
            stderr,
            b"sbstrans: before %d: missing class statement in proc glm\n\0" as *const u8
                as *const libc::c_char,
            sbslineno,
        );
        exit(1 as libc::c_int);
    }
    s = findstatement(
        step,
        b"by\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    );
    if s != 0 {
        nbyvars = 0 as libc::c_int;
        while *step.offset(s as isize) as libc::c_int != 0
            && *step.offset(s as isize) as libc::c_int != ';' as i32
        {
            s
                += linecpy(
                    0 as *mut libc::c_void as *mut libc::c_char,
                    step.offset(s as isize),
                ) + 1 as libc::c_int;
            nbyvars += 1;
            nbyvars;
        }
    } else {
        nbyvars = 0 as libc::c_int;
    }
    fprintf(
        dapfile,
        b"sort(\"%s\", \"\0" as *const u8 as *const libc::c_char,
        setname.as_mut_ptr(),
    );
    copylist(
        step,
        b"by\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        dapfile,
    );
    copylist(
        step,
        b"class\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        dapfile,
    );
    fputs(b"\", \"\");\n\0" as *const u8 as *const libc::c_char, dapfile);
    fprintf(
        dapfile,
        b"means(\"%s.srt\", \"%s\", \"N MEAN VAR\", \"\0" as *const u8
            as *const libc::c_char,
        setname.as_mut_ptr(),
        response.as_mut_ptr(),
    );
    copylist(
        step,
        b"by\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        dapfile,
    );
    copylist(
        step,
        b"class\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        dapfile,
    );
    fputs(b"\");\n\0" as *const u8 as *const libc::c_char, dapfile);
    fprintf(
        dapfile,
        b"effects(\"%s.srt.mns\", \"%s \0" as *const u8 as *const libc::c_char,
        setname.as_mut_ptr(),
        response.as_mut_ptr(),
    );
    copylist(
        step,
        b"class\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        dapfile,
    );
    fputs(b"\", \"\0" as *const u8 as *const libc::c_char, dapfile);
    s = modelstart + resplen + 1 as libc::c_int;
    if *step.offset(s as isize) as libc::c_int != '=' as i32 {
        fprintf(
            stderr,
            b"sbstrans: before %d: missing = in model statement in proc glm\n\0"
                as *const u8 as *const libc::c_char,
            sbslineno,
        );
        exit(1 as libc::c_int);
    }
    s += 2 as libc::c_int;
    while *step.offset(s as isize) as libc::c_int != 0
        && *step.offset(s as isize) as libc::c_int != '/' as i32
        && *step.offset(s as isize) as libc::c_int != ';' as i32
    {
        if *step.offset(s as isize) as libc::c_int == '\n' as i32 {
            putc(' ' as i32, dapfile);
        } else {
            putc(*step.offset(s as isize) as libc::c_int, dapfile);
        }
        s += 1;
        s;
    }
    fputs(b"\", \"\0" as *const u8 as *const libc::c_char, dapfile);
    copylist(
        step,
        b"by\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        dapfile,
    );
    fputs(b"\");\n\0" as *const u8 as *const libc::c_char, dapfile);
    lsmeans = findstatement(
        step,
        b"lsmeans\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    );
    if lsmeans != 0 {
        if getoption(
            step.offset(lsmeans as isize),
            b"e\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            0 as *mut libc::c_void as *mut libc::c_char,
            1 as libc::c_int,
        ) != 0
        {
            lsmeans = 0 as libc::c_int;
        } else {
            s = lsmeans;
            while *step.offset(s as isize) as libc::c_int != 0
                && *step.offset(s as isize) as libc::c_int != '/' as i32
                && *step.offset(s as isize) as libc::c_int != ';' as i32
            {
                s += 1;
                s;
            }
            test[0 as libc::c_int as usize] = '\0' as i32 as libc::c_char;
            if *step.offset(s as isize) as libc::c_int == '/' as i32 {
                s += 2 as libc::c_int;
                while *step.offset(s as isize) as libc::c_int != 0
                    && *step.offset(s as isize) as libc::c_int != ';' as i32
                {
                    s
                        += linecpy(test.as_mut_ptr(), step.offset(s as isize))
                            + 1 as libc::c_int;
                    upper(test.as_mut_ptr());
                    if strcmp(
                        test.as_mut_ptr(),
                        b"DUNNETT\0" as *const u8 as *const libc::c_char,
                    ) != 0
                        && strcmp(
                            test.as_mut_ptr(),
                            b"TUKEY\0" as *const u8 as *const libc::c_char,
                        ) != 0
                        && strcmp(
                            test.as_mut_ptr(),
                            b"LSD\0" as *const u8 as *const libc::c_char,
                        ) != 0
                    {
                        test[0 as libc::c_int as usize] = '\0' as i32 as libc::c_char;
                    }
                }
            }
            if test[0 as libc::c_int as usize] == 0 {
                fprintf(
                    stderr,
                    b"sbstrans: before %d: no test specified in lsmeans statement in proc glm\n\0"
                        as *const u8 as *const libc::c_char,
                    sbslineno,
                );
                exit(1 as libc::c_int);
            }
            if getoption(
                step.offset(lsmeans as isize),
                b"alpha\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
                level.as_mut_ptr(),
                1 as libc::c_int,
            ) == 0
            {
                strcpy(
                    level.as_mut_ptr(),
                    b"0.05\0" as *const u8 as *const libc::c_char,
                );
            }
        }
    }
    e = modelstart + resplen + 3 as libc::c_int;
    while *step.offset(e as isize) as libc::c_int != 0
        && *step.offset(e as isize) as libc::c_int != '/' as i32
        && *step.offset(e as isize) as libc::c_int != ';' as i32
    {
        fprintf(
            dapfile,
            b"ftest(\"%s.srt.mns.con\", \"%s \0" as *const u8 as *const libc::c_char,
            setname.as_mut_ptr(),
            response.as_mut_ptr(),
        );
        copylist(
            step,
            b"class\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            dapfile,
        );
        fputs(b"\", \"\0" as *const u8 as *const libc::c_char, dapfile);
        term = e;
        while *step.offset(e as isize) as libc::c_int != 0
            && *step.offset(e as isize) as libc::c_int != '\n' as i32
        {
            putc(*step.offset(e as isize) as libc::c_int, dapfile);
            e += 1;
            e;
            if *step.offset(e as isize) as libc::c_int == '\n' as i32
                && *step.offset((e + 1 as libc::c_int) as isize) as libc::c_int
                    == '*' as i32
            {
                putc('*' as i32, dapfile);
                e += 3 as libc::c_int;
                term = 0 as libc::c_int;
            }
        }
        fputs(b"\", \"\", \"\0" as *const u8 as *const libc::c_char, dapfile);
        copylist(
            step,
            b"by\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            dapfile,
        );
        fputs(b"\");\n\0" as *const u8 as *const libc::c_char, dapfile);
        e += 1;
        e;
        if lsmeans != 0 && term != 0 {
            s = lsmeans;
            while *step.offset(s as isize) as libc::c_int != 0
                && *step.offset(s as isize) as libc::c_int != '/' as i32
                && *step.offset(s as isize) as libc::c_int != ';' as i32
                && {
                    nomatch = linecmp(
                        step.offset(s as isize),
                        step.offset(term as isize),
                    );
                    nomatch != 0
                }
            {
                s
                    += linecpy(
                        0 as *mut libc::c_void as *mut libc::c_char,
                        step.offset(s as isize),
                    ) + 1 as libc::c_int;
            }
            if nomatch == 0 {
                fprintf(
                    dapfile,
                    b"lsmeans(\"%s.srt.mns.tst\", \"%s\", %s, \"%s \0" as *const u8
                        as *const libc::c_char,
                    setname.as_mut_ptr(),
                    test.as_mut_ptr(),
                    level.as_mut_ptr(),
                    response.as_mut_ptr(),
                );
                copylist(
                    step,
                    b"class\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
                    dapfile,
                );
                fputs(b"\", \"\0" as *const u8 as *const libc::c_char, dapfile);
                s = term;
                while *step.offset(s as isize) as libc::c_int != 0
                    && *step.offset(s as isize) as libc::c_int != '\n' as i32
                {
                    putc(*step.offset(s as isize) as libc::c_int, dapfile);
                    s += 1;
                    s;
                }
                fputs(b"\", \"\0" as *const u8 as *const libc::c_char, dapfile);
                copylist(
                    step,
                    b"by\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
                    dapfile,
                );
                fputs(b"\", \"s12\");\n\0" as *const u8 as *const libc::c_char, dapfile);
            }
        }
    }
    s = 0 as libc::c_int;
    loop {
        sincr = findstatement(
            step.offset(s as isize),
            b"test\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        );
        if !(sincr != 0) {
            break;
        }
        s += sincr;
        fprintf(
            dapfile,
            b"ftest(\"%s.srt.mns.con\", \"%s \0" as *const u8 as *const libc::c_char,
            setname.as_mut_ptr(),
            response.as_mut_ptr(),
        );
        copylist(
            step,
            b"class\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            dapfile,
        );
        fputs(b"\", \"\0" as *const u8 as *const libc::c_char, dapfile);
        if *step.offset(s as isize) == 0
            || linecmp(
                step.offset(s as isize),
                b"h\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            ) != 0
            || linecmp(
                step.offset(s as isize).offset(2 as libc::c_int as isize),
                b"=\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            ) != 0
        {
            fprintf(
                stderr,
                b"sbstrans: before %d: missing h= in test statement in proc glm\n\0"
                    as *const u8 as *const libc::c_char,
                sbslineno,
            );
            exit(1 as libc::c_int);
        }
        s += 4 as libc::c_int;
        while *step.offset(s as isize) as libc::c_int != 0
            && (linecmp(
                step.offset(s as isize),
                b"e\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            ) != 0
                || *step.offset((s + 2 as libc::c_int) as isize) as libc::c_int != 0
                    && linecmp(
                        step.offset(s as isize).offset(2 as libc::c_int as isize),
                        b"=\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
                    ) != 0)
        {
            s += putlines(step.offset(s as isize), dapfile, '\n' as i32);
            s += 1;
            s;
        }
        if *step.offset(s as isize) == 0
            || linecmp(
                step.offset(s as isize),
                b"e\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            ) != 0
            || linecmp(
                step.offset(s as isize).offset(2 as libc::c_int as isize),
                b"=\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            ) != 0
        {
            fprintf(
                stderr,
                b"sbstrans: before %d: missing e= in test statement in proc glm\n\0"
                    as *const u8 as *const libc::c_char,
                sbslineno,
            );
            exit(1 as libc::c_int);
        }
        fputs(b"\", \"\0" as *const u8 as *const libc::c_char, dapfile);
        s += 4 as libc::c_int;
        s += putlines(step.offset(s as isize), dapfile, ';' as i32);
        if *step.offset(s as isize) as libc::c_int != ';' as i32 {
            fprintf(
                stderr,
                b"sbstrans: before %d: extra characters after e=<effect> in test statement in proc glm\n\0"
                    as *const u8 as *const libc::c_char,
                sbslineno,
            );
            exit(1 as libc::c_int);
        }
        fputs(b"\", \"\0" as *const u8 as *const libc::c_char, dapfile);
        copylist(
            step,
            b"by\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            dapfile,
        );
        fputs(b"\");\n\0" as *const u8 as *const libc::c_char, dapfile);
    }
    coeff = malloc(
        (::std::mem::size_of::<libc::c_int>() as libc::c_ulong)
            .wrapping_mul(strlen(step))
            .wrapping_div(2 as libc::c_int as libc::c_ulong),
    ) as *mut libc::c_int;
    s = 0 as libc::c_int;
    loop {
        sincr = findstatement(
            step.offset(s as isize),
            b"contrast\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        );
        if !(sincr != 0) {
            break;
        }
        s += sincr;
        if *step.offset(s as isize) as libc::c_int != '"' as i32 {
            fprintf(
                stderr,
                b"sbstrans: before %d: missing \"LABEL\" in contrast statement in proc glm\n\0"
                    as *const u8 as *const libc::c_char,
                sbslineno,
            );
            exit(1 as libc::c_int);
        }
        fputs(b"title(\"\0" as *const u8 as *const libc::c_char, dapfile);
        s += 1;
        s;
        while *step.offset(s as isize) as libc::c_int != 0
            && *step.offset(s as isize) as libc::c_int != '"' as i32
        {
            putc(*step.offset(s as isize) as libc::c_int, dapfile);
            s += 1;
            s;
        }
        if *step.offset(s as isize) as libc::c_int != '"' as i32 {
            fprintf(
                stderr,
                b"sbstrans: before %d: no terminating \" in contrast statement label in proc glm\n\0"
                    as *const u8 as *const libc::c_char,
                sbslineno,
            );
            exit(1 as libc::c_int);
        }
        fputs(b"\");\n\0" as *const u8 as *const libc::c_char, dapfile);
        s += 2 as libc::c_int;
        contrastterm = 0x1 as libc::c_int as libc::c_uint;
        e = classstart;
        classvar = 0 as libc::c_int;
        while *step.offset(e as isize) as libc::c_int != 0
            && *step.offset(e as isize) as libc::c_int != ';' as i32
        {
            if linecmp(step.offset(s as isize), step.offset(e as isize)) == 0 {
                break;
            }
            e
                += linecpy(
                    0 as *mut libc::c_void as *mut libc::c_char,
                    step.offset(e as isize),
                ) + 1 as libc::c_int;
            contrastterm = contrastterm << 1 as libc::c_int;
            classvar += 1;
            classvar;
        }
        classvar += 1;
        classvar;
        s += linecpy(classname.as_mut_ptr(), step.offset(s as isize)) + 1 as libc::c_int;
        ncoeff = 0 as libc::c_int;
        coeffsum = 0 as libc::c_int;
        while num(*step.offset(s as isize) as libc::c_int) != 0
            || *step.offset(s as isize) as libc::c_int == '+' as i32
            || *step.offset(s as isize) as libc::c_int == '-' as i32
        {
            if *step.offset(s as isize) as libc::c_int == '+' as i32
                || *step.offset(s as isize) as libc::c_int == '-' as i32
            {
                minus = (*step.offset(s as isize) as libc::c_int == '-' as i32)
                    as libc::c_int;
                s += 2 as libc::c_int;
            } else {
                minus = 0 as libc::c_int;
            }
            if sscanf(
                step.offset(s as isize),
                b"%d\0" as *const u8 as *const libc::c_char,
                coeff.offset(ncoeff as isize),
            ) != 1 as libc::c_int
            {
                fprintf(
                    stderr,
                    b"sbstrans: before %d: invalid coefficient in contrast statement in proc glm\n\0"
                        as *const u8 as *const libc::c_char,
                    sbslineno,
                );
                exit(1 as libc::c_int);
            }
            if minus != 0 {
                *coeff.offset(ncoeff as isize) = -*coeff.offset(ncoeff as isize);
            }
            coeffsum += *coeff.offset(ncoeff as isize);
            s
                += linecpy(
                    0 as *mut libc::c_void as *mut libc::c_char,
                    step.offset(s as isize),
                ) + 1 as libc::c_int;
            ncoeff += 1;
            ncoeff;
        }
        if coeffsum != 0 {
            fprintf(
                stderr,
                b"sbstrans: before %d: coefficients sum to nonzero in contrast statement in proc glm\n\0"
                    as *const u8 as *const libc::c_char,
                sbslineno,
            );
            exit(1 as libc::c_int);
        }
        if *step.offset(s as isize) as libc::c_int == '/' as i32 {
            s += 2 as libc::c_int;
        }
        fprintf(
            dapfile,
            b"inset(\"%s.srt.mns.con\")\n{\n\0" as *const u8 as *const libc::c_char,
            setname.as_mut_ptr(),
        );
        fprintf(
            dapfile,
            b"char _type_[9];\ndouble %s;\nint _n_, _term_;\n\0" as *const u8
                as *const libc::c_char,
            response.as_mut_ptr(),
        );
        fprintf(
            dapfile,
            b"int _partv_[%d];\n\0" as *const u8 as *const libc::c_char,
            nbyvars + classvar,
        );
        fprintf(
            dapfile,
            b"int _c_, _more_, _contr1_;\ndouble _coeff_[%d];\n\0" as *const u8
                as *const libc::c_char,
            ncoeff,
        );
        fprintf(
            dapfile,
            b"outset(\"%s.srt.mns.con.con\", \"\");\n\0" as *const u8
                as *const libc::c_char,
            setname.as_mut_ptr(),
        );
        fputs(b"dap_list(\"\0" as *const u8 as *const libc::c_char, dapfile);
        copylist(
            step,
            b"by\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            dapfile,
        );
        e = classstart;
        c = 0 as libc::c_int;
        while c < classvar {
            e
                += putlines(step.offset(e as isize), dapfile, '\n' as i32)
                    + 1 as libc::c_int;
            c += 1;
            c;
        }
        fprintf(
            dapfile,
            b"\", _partv_, %d);\n\0" as *const u8 as *const libc::c_char,
            nbyvars + classvar,
        );
        c = 0 as libc::c_int;
        while c < ncoeff {
            fprintf(
                dapfile,
                b"_coeff_[%d] = %d.0;\n\0" as *const u8 as *const libc::c_char,
                c,
                *coeff.offset(c as isize),
            );
            c += 1;
            c;
        }
        fputs(
            b"for (_c_ = 0, _contr1_ = 1, _more_ = step(); _more_; )\n{\n\0" as *const u8
                as *const libc::c_char,
            dapfile,
        );
        fprintf(
            dapfile,
            b"if (dap_newpart(_partv_, %d))\n_c_ = 0;\n\0" as *const u8
                as *const libc::c_char,
            nbyvars + classvar - 1 as libc::c_int,
        );
        fprintf(
            dapfile,
            b"else if (dap_newpart(_partv_, %d))\n_c_++;\n\0" as *const u8
                as *const libc::c_char,
            nbyvars + classvar,
        );
        fputs(
            b"output();\nstep();\noutput();\nstep();\noutput();\n\0" as *const u8
                as *const libc::c_char,
            dapfile,
        );
        fputs(
            b"while ((_more_ = step()))\n{\n\0" as *const u8 as *const libc::c_char,
            dapfile,
        );
        fputs(
            b"if (!strcmp(_type_, \"ERROR\"))\noutput();\n\0" as *const u8
                as *const libc::c_char,
            dapfile,
        );
        fputs(
            b"else if (!strcmp(_type_, \"CONTR\"))\n{\n\0" as *const u8
                as *const libc::c_char,
            dapfile,
        );
        fprintf(
            dapfile,
            b"if (_term_ == %d)\n{\n\0" as *const u8 as *const libc::c_char,
            contrastterm,
        );
        fprintf(
            dapfile,
            b"if (_contr1_)\n{\n_contr1_ = 0;\n_term_ = %d;\n\0" as *const u8
                as *const libc::c_char,
            contrastterm,
        );
        fprintf(
            dapfile,
            b"if (_c_ < %d)\n\0" as *const u8 as *const libc::c_char,
            ncoeff,
        );
        fprintf(
            dapfile,
            b"%s = _coeff_[_c_];\nelse\n%s = 0.0;\n\0" as *const u8
                as *const libc::c_char,
            response.as_mut_ptr(),
            response.as_mut_ptr(),
        );
        fputs(b"output();\n}\n}\n\0" as *const u8 as *const libc::c_char, dapfile);
        fputs(b"else\noutput();\n}\n\0" as *const u8 as *const libc::c_char, dapfile);
        fputs(
            b"else if (!strcmp(_type_, \"LSMEAN\"))\noutput();\n\0" as *const u8
                as *const libc::c_char,
            dapfile,
        );
        fputs(
            b"else\n{\n_contr1_ = 1;\nbreak;\n}\n}\n}\n}\n\0" as *const u8
                as *const libc::c_char,
            dapfile,
        );
        fprintf(
            dapfile,
            b"ftest(\"%s.srt.mns.con.con\", \"%s \0" as *const u8 as *const libc::c_char,
            setname.as_mut_ptr(),
            response.as_mut_ptr(),
        );
        copylist(
            step,
            b"class\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            dapfile,
        );
        fprintf(
            dapfile,
            b"\", \"%s\", \"\0" as *const u8 as *const libc::c_char,
            classname.as_mut_ptr(),
        );
        sincr = getoption(
            step.offset(s as isize),
            b"e\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            0 as *mut libc::c_void as *mut libc::c_char,
            1 as libc::c_int,
        );
        if sincr != 0 {
            s += sincr;
            s += putlines(step.offset(s as isize), dapfile, ';' as i32);
        }
        if linecmp(
            step.offset(s as isize),
            b";\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        ) == 0
        {
            s += 2 as libc::c_int;
        } else {
            fprintf(
                stderr,
                b"sbstrans: before %d: missing ; at end of contrast statement in proc glm\n\0"
                    as *const u8 as *const libc::c_char,
                sbslineno,
            );
            exit(1 as libc::c_int);
        }
        fputs(b"\", \"\0" as *const u8 as *const libc::c_char, dapfile);
        copylist(
            step,
            b"by\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            dapfile,
        );
        fputs(b"\");\ntitle(NULL);\n\0" as *const u8 as *const libc::c_char, dapfile);
    }
    free(coeff as *mut libc::c_void);
    loop {
        s = findstatement(
            step.offset(lsmeans as isize),
            b"lsmeans\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        );
        if !(s != 0) {
            break;
        }
        lsmeans += s;
        e = lsmeans
            + getoption(
                step.offset(lsmeans as isize),
                b"e\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
                0 as *mut libc::c_void as *mut libc::c_char,
                1 as libc::c_int,
            );
        if getoption(
            step.offset(lsmeans as isize),
            b"alpha\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            level.as_mut_ptr(),
            1 as libc::c_int,
        ) == 0
        {
            strcpy(level.as_mut_ptr(), b"0.05\0" as *const u8 as *const libc::c_char);
        }
        test[0 as libc::c_int as usize] = '\0' as i32 as libc::c_char;
        s = lsmeans;
        while *step.offset(s as isize) as libc::c_int != 0
            && *step.offset(s as isize) as libc::c_int != '/' as i32
            && *step.offset(s as isize) as libc::c_int != ';' as i32
        {
            s += 1;
            s;
        }
        if *step.offset(s as isize) as libc::c_int == '/' as i32 {
            s += 2 as libc::c_int;
            while *step.offset(s as isize) as libc::c_int != 0
                && *step.offset(s as isize) as libc::c_int != ';' as i32
            {
                s
                    += linecpy(test.as_mut_ptr(), step.offset(s as isize))
                        + 1 as libc::c_int;
                upper(test.as_mut_ptr());
                if strcmp(
                    test.as_mut_ptr(),
                    b"DUNNETT\0" as *const u8 as *const libc::c_char,
                ) != 0
                    && strcmp(
                        test.as_mut_ptr(),
                        b"TUKEY\0" as *const u8 as *const libc::c_char,
                    ) != 0
                    && strcmp(
                        test.as_mut_ptr(),
                        b"LSD\0" as *const u8 as *const libc::c_char,
                    ) != 0
                {
                    test[0 as libc::c_int as usize] = '\0' as i32 as libc::c_char;
                }
            }
        }
        if test[0 as libc::c_int as usize] == 0 {
            fprintf(
                stderr,
                b"sbstrans: before %d: no test specified in lsmeans statement in proc glm\n\0"
                    as *const u8 as *const libc::c_char,
                sbslineno,
            );
            exit(1 as libc::c_int);
        }
        upper(test.as_mut_ptr());
        s = lsmeans;
        while *step.offset(s as isize) as libc::c_int != 0
            && *step.offset(s as isize) as libc::c_int != '/' as i32
            && *step.offset(s as isize) as libc::c_int != ';' as i32
        {
            fprintf(
                dapfile,
                b"ftest(\"%s.srt.mns.con\", \"%s \0" as *const u8 as *const libc::c_char,
                setname.as_mut_ptr(),
                response.as_mut_ptr(),
            );
            copylist(
                step,
                b"class\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
                dapfile,
            );
            fputs(b"\", \"\0" as *const u8 as *const libc::c_char, dapfile);
            term = s;
            s += putlines(step.offset(s as isize), dapfile, '\n' as i32);
            fputs(b"\", \"\0" as *const u8 as *const libc::c_char, dapfile);
            if e > lsmeans {
                s = e;
                while *step.offset(s as isize) as libc::c_int != 0
                    && *step.offset(s as isize) as libc::c_int != ';' as i32
                    && linecmp(
                        step.offset(s as isize),
                        b"alpha\0" as *const u8 as *const libc::c_char
                            as *mut libc::c_char,
                    ) != 0
                    && linecmp(
                        step.offset(s as isize),
                        b"dunnett\0" as *const u8 as *const libc::c_char
                            as *mut libc::c_char,
                    ) != 0
                    && linecmp(
                        step.offset(s as isize),
                        b"tukey\0" as *const u8 as *const libc::c_char
                            as *mut libc::c_char,
                    ) != 0
                    && linecmp(
                        step.offset(s as isize),
                        b"lsd\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
                    ) != 0
                {
                    s
                        += putlines(step.offset(s as isize), dapfile, '\n' as i32)
                            + 1 as libc::c_int;
                }
            }
            fputs(b"\", \"\0" as *const u8 as *const libc::c_char, dapfile);
            copylist(
                step,
                b"by\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
                dapfile,
            );
            fputs(b"\");\n\0" as *const u8 as *const libc::c_char, dapfile);
            fprintf(
                dapfile,
                b"lsmeans(\"%s.srt.mns.tst\", \"%s\", %s, \"%s \0" as *const u8
                    as *const libc::c_char,
                setname.as_mut_ptr(),
                test.as_mut_ptr(),
                level.as_mut_ptr(),
                response.as_mut_ptr(),
            );
            copylist(
                step,
                b"class\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
                dapfile,
            );
            fputs(b"\", \"\0" as *const u8 as *const libc::c_char, dapfile);
            s = term + putlines(step.offset(term as isize), dapfile, '\n' as i32);
            fputs(b"\", \"\0" as *const u8 as *const libc::c_char, dapfile);
            copylist(
                step,
                b"by\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
                dapfile,
            );
            fputs(b"\", \"s12\");\n\0" as *const u8 as *const libc::c_char, dapfile);
            s += 1;
            s;
        }
    };
}
pub unsafe extern "C" fn logistictrans(
    mut step: *mut libc::c_char,
    mut dapfile: *mut FILE,
) {
    let mut s: libc::c_int = 0;
    let mut setname: [libc::c_char; 128] = [0; 128];
    let mut outname: [libc::c_char; 128] = [0; 128];
    if getoption(
        step,
        b"data\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        setname.as_mut_ptr(),
        1 as libc::c_int,
    ) == 0
    {
        strcpy(setname.as_mut_ptr(), sbstmp.as_mut_ptr());
    }
    fprintf(
        dapfile,
        b"logreg(\"%s\", \"\0" as *const u8 as *const libc::c_char,
        setname.as_mut_ptr(),
    );
    s = findstatement(
        step,
        b"model\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    );
    if s != 0 {
        s += putlines(step.offset(s as isize), dapfile, '\n' as i32) + 1 as libc::c_int;
        putc('/' as i32, dapfile);
        if *step.offset(s as isize) as libc::c_int == '/' as i32 {
            s += 2 as libc::c_int;
            s
                += putlines(step.offset(s as isize), dapfile, '\n' as i32)
                    + 1 as libc::c_int;
        } else {
            putc('1' as i32, dapfile);
        }
        if *step.offset(s as isize) as libc::c_int == '=' as i32 {
            fputs(b"\", \"\", \"\0" as *const u8 as *const libc::c_char, dapfile);
            s += 2 as libc::c_int;
            s += putlines(step.offset(s as isize), dapfile, ';' as i32);
            fputs(b"\", \"\0" as *const u8 as *const libc::c_char, dapfile);
            copylist(
                step,
                b"by\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
                dapfile,
            );
            fputs(b"\", NULL, 0.95);\n\0" as *const u8 as *const libc::c_char, dapfile);
        } else {
            fprintf(
                stderr,
                b"sbstrans: before %d: missing = in model statement in proc logistic.\n\0"
                    as *const u8 as *const libc::c_char,
                sbslineno,
            );
            exit(1 as libc::c_int);
        }
    } else {
        fprintf(
            stderr,
            b"sbstrans: before %d: missing model statement in proc logistic.\n\0"
                as *const u8 as *const libc::c_char,
            sbslineno,
        );
        exit(1 as libc::c_int);
    }
    if getoption(
        step,
        b"outest\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        outname.as_mut_ptr(),
        1 as libc::c_int,
    ) != 0
    {
        fprintf(
            dapfile,
            b"dataset(\"%s.cov\", \"%s\", \"RENAME\");\n\0" as *const u8
                as *const libc::c_char,
            setname.as_mut_ptr(),
            outname.as_mut_ptr(),
        );
        strcpy(sbstmp.as_mut_ptr(), outname.as_mut_ptr());
    }
}
pub unsafe extern "C" fn npar1waytrans(
    mut step: *mut libc::c_char,
    mut dapfile: *mut FILE,
) {
    let mut s: libc::c_int = 0;
    let mut setname: [libc::c_char; 128] = [0; 128];
    let mut classname: [libc::c_char; 128] = [0; 128];
    if getoption(
        step,
        b"data\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        setname.as_mut_ptr(),
        1 as libc::c_int,
    ) == 0
    {
        strcpy(setname.as_mut_ptr(), sbstmp.as_mut_ptr());
    }
    s = findstatement(
        step,
        b"class\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    );
    if s != 0 {
        linecpy(classname.as_mut_ptr(), step.offset(s as isize));
    } else {
        fprintf(
            stderr,
            b"sbstrans: before %d: missing class statement in proc npar1way.\n\0"
                as *const u8 as *const libc::c_char,
            sbslineno,
        );
        exit(1 as libc::c_int);
    }
    s = findstatement(
        step,
        b"var\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    );
    if s != 0 {
        while *step.offset(s as isize) as libc::c_int != 0
            && *step.offset(s as isize) as libc::c_int != ';' as i32
        {
            fprintf(
                dapfile,
                b"nonparam(\"%s\", \"\0" as *const u8 as *const libc::c_char,
                setname.as_mut_ptr(),
            );
            while *step.offset(s as isize) as libc::c_int != 0
                && *step.offset(s as isize) as libc::c_int != '\n' as i32
            {
                putc(*step.offset(s as isize) as libc::c_int, dapfile);
                s += 1;
                s;
            }
            fprintf(
                dapfile,
                b" %s\", \"\0" as *const u8 as *const libc::c_char,
                classname.as_mut_ptr(),
            );
            copylist(
                step,
                b"by\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
                dapfile,
            );
            fputs(b"\");\n\0" as *const u8 as *const libc::c_char, dapfile);
            s += 1;
            s;
        }
        if *step.offset(s as isize) as libc::c_int != ';' as i32 {
            fprintf(
                stderr,
                b"sbstrans: before %d: missing ; at end of proc npar1way.\n\0"
                    as *const u8 as *const libc::c_char,
                sbslineno,
            );
            exit(1 as libc::c_int);
        }
    } else {
        fprintf(
            stderr,
            b"sbstrans: before %d: missing var statement in proc npar1way.\n\0"
                as *const u8 as *const libc::c_char,
            sbslineno,
        );
        exit(1 as libc::c_int);
    };
}
pub unsafe extern "C" fn regtrans(mut step: *mut libc::c_char, mut dapfile: *mut FILE) {
    let mut s: libc::c_int = 0;
    let mut setname: [libc::c_char; 128] = [0; 128];
    let mut outname: [libc::c_char; 128] = [0; 128];
    let mut isadd: libc::c_int = 0;
    if getoption(
        step,
        b"data\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        setname.as_mut_ptr(),
        1 as libc::c_int,
    ) == 0
    {
        strcpy(setname.as_mut_ptr(), sbstmp.as_mut_ptr());
    }
    if findstatement(
        step,
        b"plot\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    ) != 0
    {
        if isby(step) >= 0 as libc::c_int {
            countparts(step, setname.as_mut_ptr(), dapfile);
        } else {
            fputs(
                b"_sbspictcnt_[_sbsnpicts_] = 1;\n\0" as *const u8
                    as *const libc::c_char,
                dapfile,
            );
        }
        fprintf(
            dapfile,
            b"_sbspict_[_sbsnpicts_] = plotlinreg(\"%s\", \"\0" as *const u8
                as *const libc::c_char,
            setname.as_mut_ptr(),
        );
        s = findstatement(
            step,
            b"model\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        );
        if s != 0 {
            while *step.offset(s as isize) as libc::c_int != 0
                && *step.offset(s as isize) as libc::c_int != '\n' as i32
            {
                putc(*step.offset(s as isize) as libc::c_int, dapfile);
                s += 1;
                s;
            }
            s += 1;
            s;
            if *step.offset(s as isize) as libc::c_int == '=' as i32 {
                fputs(b"\", \"\0" as *const u8 as *const libc::c_char, dapfile);
                s += 2 as libc::c_int;
                while *step.offset(s as isize) as libc::c_int != 0
                    && *step.offset(s as isize) as libc::c_int != '\n' as i32
                {
                    putc(*step.offset(s as isize) as libc::c_int, dapfile);
                    s += 1;
                    s;
                }
                if *step.offset((s + 1 as libc::c_int) as isize) as libc::c_int
                    != ';' as i32
                {
                    fprintf(
                        stderr,
                        b"sbstrans: before %d: only one explanatory variable allowed in model statement in proc reg with plotting\n\0"
                            as *const u8 as *const libc::c_char,
                        sbslineno,
                    );
                    exit(1 as libc::c_int);
                }
                fputs(b"\", \"==\", \"\0" as *const u8 as *const libc::c_char, dapfile);
                copylist(
                    step,
                    b"by\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
                    dapfile,
                );
                fputs(
                    b"\", _sbspictcnt_[_sbsnpicts_], 0.95);\n\0" as *const u8
                        as *const libc::c_char,
                    dapfile,
                );
                fputs(
                    b"_sbspictpage_[_sbsnpicts_++] = 4;\n\0" as *const u8
                        as *const libc::c_char,
                    dapfile,
                );
                sbshaspicts = 1 as libc::c_int;
            } else {
                fprintf(
                    stderr,
                    b"sbstrans: before %d: only one response variable allowed in model statement in proc reg with plotting\n\0"
                        as *const u8 as *const libc::c_char,
                    sbslineno,
                );
                exit(1 as libc::c_int);
            }
        } else {
            fprintf(
                stderr,
                b"sbstrans: before %d: missing model statement in proc reg.\n\0"
                    as *const u8 as *const libc::c_char,
                sbslineno,
            );
            exit(1 as libc::c_int);
        }
    } else {
        fprintf(
            dapfile,
            b"linreg(\"%s\", \"\0" as *const u8 as *const libc::c_char,
            setname.as_mut_ptr(),
        );
        s = findstatement(
            step,
            b"model\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        );
        if s != 0 {
            s += putlines(step.offset(s as isize), dapfile, '=' as i32);
            if *step.offset(s as isize) as libc::c_int == '=' as i32 {
                fputs(b"\", \"\0" as *const u8 as *const libc::c_char, dapfile);
                isadd = findstatement(
                    step,
                    b"add\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
                );
                if isadd == 0 {
                    fputs(b"\", \"\0" as *const u8 as *const libc::c_char, dapfile);
                }
                s += 2 as libc::c_int;
                s += putlines(step.offset(s as isize), dapfile, ';' as i32);
                fputs(b"\", \"\0" as *const u8 as *const libc::c_char, dapfile);
                if isadd != 0 {
                    copylist(
                        step,
                        b"add\0" as *const u8 as *const libc::c_char
                            as *mut libc::c_char,
                        dapfile,
                    );
                    fputs(b"\", \"\0" as *const u8 as *const libc::c_char, dapfile);
                }
                copylist(
                    step,
                    b"by\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
                    dapfile,
                );
                fputs(
                    b"\", NULL, 0.95);\n\0" as *const u8 as *const libc::c_char,
                    dapfile,
                );
            } else {
                fprintf(
                    stderr,
                    b"sbstrans: before %d: missing = in model statement in proc reg.\n\0"
                        as *const u8 as *const libc::c_char,
                    sbslineno,
                );
                exit(1 as libc::c_int);
            }
        } else {
            fprintf(
                stderr,
                b"sbstrans: before %d: missing model statement in proc reg.\n\0"
                    as *const u8 as *const libc::c_char,
                sbslineno,
            );
            exit(1 as libc::c_int);
        }
    }
    if getoption(
        step,
        b"outest\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        outname.as_mut_ptr(),
        1 as libc::c_int,
    ) != 0
    {
        fprintf(
            dapfile,
            b"dataset(\"%s.cov\", \"%s\", \"RENAME\");\n\0" as *const u8
                as *const libc::c_char,
            setname.as_mut_ptr(),
            outname.as_mut_ptr(),
        );
        strcpy(sbstmp.as_mut_ptr(), outname.as_mut_ptr());
    }
}
pub unsafe extern "C" fn daptrans(mut step: *mut libc::c_char, mut dapfile: *mut FILE) {
    let mut s: libc::c_int = 0;
    let mut brace: libc::c_int = 0;
    s = 0 as libc::c_int;
    if *step.offset(s as isize) as libc::c_int != ';' as i32 {
        fprintf(
            stderr,
            b"sbstrans: before %d: no options allowed for proc dap\n\0" as *const u8
                as *const libc::c_char,
            sbslineno,
        );
        exit(1 as libc::c_int);
    }
    s += 2 as libc::c_int;
    if *step.offset(s as isize) as libc::c_int == '{' as i32 {
        fputs(b"{\n\0" as *const u8 as *const libc::c_char, dapfile);
        s += 2 as libc::c_int;
        brace = 1 as libc::c_int;
        while brace != 0 && *step.offset(s as isize) as libc::c_int != 0 {
            if *step.offset(s as isize) as libc::c_int == '\n' as i32 {
                putc(' ' as i32, dapfile);
            } else {
                putc(*step.offset(s as isize) as libc::c_int, dapfile);
            }
            if *step.offset(s as isize) as libc::c_int == ';' as i32
                || *step.offset(s as isize) as libc::c_int == '{' as i32
                || *step.offset(s as isize) as libc::c_int == '}' as i32
            {
                putc('\n' as i32, dapfile);
            }
            if *step.offset(s as isize) as libc::c_int == '{' as i32 {
                brace += 1;
                brace;
            } else if *step.offset(s as isize) as libc::c_int == '}' as i32 {
                brace -= 1;
                brace;
            }
            s += 1;
            s;
        }
        if brace != 0 {
            fprintf(
                stderr,
                b"sbstrans: before %d: missing } in proc dap\n\0" as *const u8
                    as *const libc::c_char,
                sbslineno,
            );
            exit(1 as libc::c_int);
        }
        putc('\n' as i32, dapfile);
    } else {
        s += putlines(step.offset(s as isize), dapfile, ';' as i32);
        if *step.offset(s as isize) as libc::c_int != ';' as i32 {
            fprintf(
                stderr,
                b"sbstrans: before %d: missing ; in proc dap\n\0" as *const u8
                    as *const libc::c_char,
                sbslineno,
            );
            exit(1 as libc::c_int);
        }
        fputs(b";\n\0" as *const u8 as *const libc::c_char, dapfile);
    };
}
pub unsafe extern "C" fn importtrans(
    mut step: *mut libc::c_char,
    mut dapfile: *mut FILE,
) {
    let mut s: libc::c_int = 0;
    let mut brace: libc::c_int = 0;
    let mut start: libc::c_int = 0;
    let mut resplen: libc::c_int = 0;
    let mut replace: libc::c_int = 0 as libc::c_int;
    static mut delimiter: [libc::c_char; 128] = [0; 128];
    static mut setname: [libc::c_char; 128] = [0; 128];
    static mut datafile: [libc::c_char; 128] = [0; 128];
    static mut dbms: [libc::c_char; 128] = [0; 128];
    static mut strreplace: [libc::c_char; 128] = [0; 128];
    static mut getnames: [libc::c_char; 4] = [0; 4];
    if getoption(
        step,
        b"out\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        setname.as_mut_ptr(),
        1 as libc::c_int,
    ) == 0
    {
        strcpy(setname.as_mut_ptr(), sbstmp.as_mut_ptr());
    }
    if getoption(
        step,
        b"datafile\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        datafile.as_mut_ptr(),
        1 as libc::c_int,
    ) == 0
    {
        strcpy(datafile.as_mut_ptr(), sbstmp.as_mut_ptr());
    }
    if getoption(
        step,
        b"dbms\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        dbms.as_mut_ptr(),
        1 as libc::c_int,
    ) == 0
    {
        strcpy(dbms.as_mut_ptr(), sbstmp.as_mut_ptr());
    }
    if getoption(
        step,
        b"delimiter\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        delimiter.as_mut_ptr(),
        1 as libc::c_int,
    ) == 0
    {
        strcpy(delimiter.as_mut_ptr(), b"\0" as *const u8 as *const libc::c_char);
    } else {
        if delimiter[0 as libc::c_int as usize] as libc::c_int == '\'' as i32 {
            delimiter[0 as libc::c_int as usize] = '"' as i32 as libc::c_char;
        }
        if delimiter[(strlen(delimiter.as_mut_ptr()))
            .wrapping_sub(1 as libc::c_int as libc::c_ulong) as usize] as libc::c_int
            == '\'' as i32
        {
            delimiter[(strlen(delimiter.as_mut_ptr()))
                .wrapping_sub(1 as libc::c_int as libc::c_ulong)
                as usize] = '"' as i32 as libc::c_char;
        }
    }
    s = findstatement(
        step,
        b"getnames\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    );
    if s != 0 {
        let mut i: libc::c_int = 8 as libc::c_int;
        while *step.offset((s + i) as isize) as libc::c_int != 'n' as i32
            && *step.offset((s + i) as isize) as libc::c_int != 'y' as i32
            && *step.offset((s + i) as isize) as libc::c_int != ';' as i32
            && *step.offset((s + i) as isize) as libc::c_int != '\0' as i32
        {
            i += 1;
            i;
        }
        if *step.offset((s + i) as isize) as libc::c_int == 'n' as i32
            || *step.offset((s + i) as isize) as libc::c_int == 'N' as i32
        {
            strcpy(getnames.as_mut_ptr(), b"no\0" as *const u8 as *const libc::c_char);
        } else {
            strcpy(getnames.as_mut_ptr(), b"yes\0" as *const u8 as *const libc::c_char);
        }
    }
    let mut getnam: libc::c_int = 1 as libc::c_int;
    if strcmp(getnames.as_mut_ptr(), b"no\0" as *const u8 as *const libc::c_char)
        == 0 as libc::c_int
    {
        getnam = 0 as libc::c_int;
    }
    if getoption(
        step,
        b"replace\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        strreplace.as_mut_ptr(),
        0 as libc::c_int,
    ) == 0
    {
        replace = 1 as libc::c_int;
    }
    fprintf(
        dapfile,
        b"import(\"%s\", %s , \"%s\",%s, %i,%i);\n\0" as *const u8
            as *const libc::c_char,
        setname.as_mut_ptr(),
        datafile.as_mut_ptr(),
        dbms.as_mut_ptr(),
        delimiter.as_mut_ptr(),
        replace,
        getnam,
    );
}
pub unsafe extern "C" fn surveyselecttrans(
    mut step: *mut libc::c_char,
    mut dapfile: *mut FILE,
) {
    let mut s: libc::c_int = 0;
    let mut brace: libc::c_int = 0;
    let mut start: libc::c_int = 0;
    let mut resplen: libc::c_int = 0;
    let mut replace: libc::c_int = 0 as libc::c_int;
    static mut nbtirage: [libc::c_char; 128] = [0; 128];
    static mut setname: [libc::c_char; 128] = [0; 128];
    static mut datafile: [libc::c_char; 128] = [0; 128];
    static mut method: [libc::c_char; 128] = [0; 128];
    static mut strreplace: [libc::c_char; 128] = [0; 128];
    static mut getnames: [libc::c_char; 4] = [0; 4];
    if getoption(
        step,
        b"out\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        setname.as_mut_ptr(),
        1 as libc::c_int,
    ) == 0
    {
        strcpy(setname.as_mut_ptr(), sbstmp.as_mut_ptr());
    }
    if getoption(
        step,
        b"data\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        datafile.as_mut_ptr(),
        1 as libc::c_int,
    ) == 0
    {
        strcpy(datafile.as_mut_ptr(), sbstmp.as_mut_ptr());
    }
    if getoption(
        step,
        b"method\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        method.as_mut_ptr(),
        1 as libc::c_int,
    ) == 0
    {
        strcpy(method.as_mut_ptr(), sbstmp.as_mut_ptr());
    }
    if getoption(
        step,
        b"n\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        nbtirage.as_mut_ptr(),
        1 as libc::c_int,
    ) == 0
    {
        strcpy(nbtirage.as_mut_ptr(), b"1\0" as *const u8 as *const libc::c_char);
    }
    fprintf(
        dapfile,
        b" surveyselect(\"%s\", \"%s\" , \"%s\",%s);\n\0" as *const u8
            as *const libc::c_char,
        datafile.as_mut_ptr(),
        setname.as_mut_ptr(),
        method.as_mut_ptr(),
        nbtirage.as_mut_ptr(),
    );
}
