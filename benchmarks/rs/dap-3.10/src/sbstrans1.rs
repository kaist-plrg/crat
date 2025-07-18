use ::libc;
extern "C" {
    pub type _IO_wide_data;
    pub type _IO_codecvt;
    pub type _IO_marker;
    static mut stderr: *mut FILE;
    fn fprintf(_: *mut FILE, _: *const libc::c_char, _: ...) -> libc::c_int;
    fn sprintf(_: *mut libc::c_char, _: *const libc::c_char, _: ...) -> libc::c_int;
    fn sscanf(_: *const libc::c_char, _: *const libc::c_char, _: ...) -> libc::c_int;
    fn putc(__c: libc::c_int, __stream: *mut FILE) -> libc::c_int;
    fn fputs(__s: *const libc::c_char, __stream: *mut FILE) -> libc::c_int;
    static mut sbstempnum: libc::c_int;
    static mut sbshaspicts: libc::c_int;
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
    fn num(c: libc::c_int) -> libc::c_int;
    fn alphanum(c: libc::c_int) -> libc::c_int;
    fn exit(_: libc::c_int) -> !;
    fn strcpy(_: *mut libc::c_char, _: *const libc::c_char) -> *mut libc::c_char;
    fn strcat(_: *mut libc::c_char, _: *const libc::c_char) -> *mut libc::c_char;
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
pub unsafe extern "C" fn upper(mut str: *mut libc::c_char) {
    while *str != 0 {
        if 'a' as i32 <= *str as libc::c_int && *str as libc::c_int <= 'z' as i32 {
            *str = (*str as libc::c_int + ('A' as i32 - 'a' as i32)) as libc::c_char;
        }
        str = str.offset(1);
        str;
    }
}
pub unsafe extern "C" fn printtrans(
    mut step: *mut libc::c_char,
    mut dapfile: *mut FILE,
) {
    let mut filename: [libc::c_char; 128] = [0; 128];
    if getoption(
        step,
        b"data\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        filename.as_mut_ptr(),
        1 as libc::c_int,
    ) == 0
    {
        strcpy(filename.as_mut_ptr(), sbstmp.as_mut_ptr());
    }
    fprintf(
        dapfile,
        b"print(\"%s\", \"\0" as *const u8 as *const libc::c_char,
        filename.as_mut_ptr(),
    );
    copylist(
        step,
        b"var\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        dapfile,
    );
    fputs(b"\");\n\0" as *const u8 as *const libc::c_char, dapfile);
}
pub unsafe extern "C" fn meanstrans(
    mut step: *mut libc::c_char,
    mut dapfile: *mut FILE,
) {
    let mut s: libc::c_int = 0;
    let mut setname: [libc::c_char; 128] = [0; 128];
    let mut outname: [libc::c_char; 128] = [0; 128];
    let mut noprint: libc::c_int = 0;
    let mut statsreq: libc::c_int = 0;
    let mut vardf: libc::c_int = 0;
    let mut stat: [libc::c_char; 128] = [0; 128];
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
        b"means(\"%s\", \"\0" as *const u8 as *const libc::c_char,
        setname.as_mut_ptr(),
    );
    copylist(
        step,
        b"var\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        dapfile,
    );
    if findstatement(
        step,
        b"weight\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    ) != 0
    {
        putc('*' as i32, dapfile);
        copylist(
            step,
            b"weight\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            dapfile,
        );
    }
    fputs(b"\", \"\0" as *const u8 as *const libc::c_char, dapfile);
    vardf = 0 as libc::c_int;
    if getoption(
        step,
        b"vardf\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        stat.as_mut_ptr(),
        1 as libc::c_int,
    ) != 0
    {
        if linecmp(
            stat.as_mut_ptr(),
            b"wdf\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        ) == 0
        {
            vardf = 1 as libc::c_int;
        } else if linecmp(
            stat.as_mut_ptr(),
            b"df\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        ) != 0
        {
            fprintf(
                stderr,
                b"sbstrans: before %d: invalid option for vardf in proc means\n\0"
                    as *const u8 as *const libc::c_char,
                sbslineno,
            );
            exit(1 as libc::c_int);
        }
    }
    s = 0 as libc::c_int;
    noprint = 0 as libc::c_int;
    statsreq = 0 as libc::c_int;
    while *step.offset(s as isize) as libc::c_int != 0
        && *step.offset(s as isize) as libc::c_int != ';' as i32
    {
        if linecmp(
            step.offset(s as isize),
            b"noprint\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        ) == 0
        {
            noprint = 1 as libc::c_int;
            s += 7 as libc::c_int;
        } else if linecmp(
            step.offset(s as isize),
            b"data\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        ) == 0
        {
            s += 7 as libc::c_int;
            while *step.offset(s as isize) as libc::c_int != 0
                && *step.offset(s as isize) as libc::c_int != '\n' as i32
            {
                s += 1;
                s;
            }
        } else if linecmp(
            step.offset(s as isize),
            b"vardf\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        ) == 0
        {
            s += 8 as libc::c_int;
            if linecmp(
                step.offset(s as isize),
                b"wdf\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            ) == 0
            {
                s += 3 as libc::c_int;
            } else {
                s += 2 as libc::c_int;
            }
        } else {
            statsreq = 1 as libc::c_int;
            s += linecpy(stat.as_mut_ptr(), step.offset(s as isize));
            upper(stat.as_mut_ptr());
            if linecmp(
                stat.as_mut_ptr(),
                b"STD\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            ) == 0
            {
                strcpy(stat.as_mut_ptr(), b"SD\0" as *const u8 as *const libc::c_char);
            } else if linecmp(
                stat.as_mut_ptr(),
                b"STDERR\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            ) == 0
            {
                strcpy(stat.as_mut_ptr(), b"SEM\0" as *const u8 as *const libc::c_char);
            }
            if vardf != 0
                && (linecmp(
                    stat.as_mut_ptr(),
                    b"SD\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
                ) == 0
                    || linecmp(
                        stat.as_mut_ptr(),
                        b"SEM\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
                    ) == 0
                    || linecmp(
                        stat.as_mut_ptr(),
                        b"VAR\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
                    ) == 0)
            {
                strcat(stat.as_mut_ptr(), b"FREQ\0" as *const u8 as *const libc::c_char);
            }
            fputs(stat.as_mut_ptr(), dapfile);
            putc(' ' as i32, dapfile);
        }
        s += 1;
        s;
    }
    if statsreq == 0 {
        if vardf != 0 {
            fputs(
                b"N MEAN SDFREQ MIN MAX\0" as *const u8 as *const libc::c_char,
                dapfile,
            );
        } else {
            fputs(b"N MEAN SD MIN MAX\0" as *const u8 as *const libc::c_char, dapfile);
        }
    }
    fputs(b"\", \"\0" as *const u8 as *const libc::c_char, dapfile);
    copylist(
        step,
        b"by\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        dapfile,
    );
    fputs(b"\");\n\0" as *const u8 as *const libc::c_char, dapfile);
    s = findstatement(
        step,
        b"output\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    );
    if s != 0 {
        if getoption(
            step.offset(s as isize),
            b"out\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            outname.as_mut_ptr(),
            1 as libc::c_int,
        ) == 0
        {
            sbstempnum += 1;
            sprintf(
                sbstmp.as_mut_ptr(),
                b"sbstmp%02d\0" as *const u8 as *const libc::c_char,
                sbstempnum,
            );
            strcpy(outname.as_mut_ptr(), sbstmp.as_mut_ptr());
        }
    } else {
        outname[0 as libc::c_int as usize] = '\0' as i32 as libc::c_char;
    }
    if noprint == 0 {
        fprintf(
            dapfile,
            b"print(\"%s.mns\", \"\");\n\0" as *const u8 as *const libc::c_char,
            setname.as_mut_ptr(),
        );
    }
    if outname[0 as libc::c_int as usize] != 0 {
        fprintf(
            dapfile,
            b"dataset(\"%s.mns\", \"%s\", \"RENAME\");\n\0" as *const u8
                as *const libc::c_char,
            setname.as_mut_ptr(),
            outname.as_mut_ptr(),
        );
        strcpy(sbstmp.as_mut_ptr(), outname.as_mut_ptr());
    }
}
pub unsafe extern "C" fn sorttrans(mut step: *mut libc::c_char, mut dapfile: *mut FILE) {
    let mut s: libc::c_int = 0;
    let mut sin: libc::c_int = 0;
    let mut sout: libc::c_int = 0;
    let mut inname: [libc::c_char; 132] = [0; 132];
    let mut outname: [libc::c_char; 128] = [0; 128];
    let mut optstart: libc::c_int = 0;
    let mut descend: libc::c_int = 0;
    fputs(b"sort(\"\0" as *const u8 as *const libc::c_char, dapfile);
    inname[0 as libc::c_int as usize] = '\0' as i32 as libc::c_char;
    outname[0 as libc::c_int as usize] = '\0' as i32 as libc::c_char;
    sin = getoption(
        step,
        b"data\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        inname.as_mut_ptr(),
        1 as libc::c_int,
    );
    if sin == 0 {
        strcpy(inname.as_mut_ptr(), sbstmp.as_mut_ptr());
    }
    sout = getoption(
        step,
        b"out\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        outname.as_mut_ptr(),
        1 as libc::c_int,
    );
    if sout == 0 {
        strcpy(outname.as_mut_ptr(), inname.as_mut_ptr());
    }
    fprintf(
        dapfile,
        b"%s\", \"\0" as *const u8 as *const libc::c_char,
        inname.as_mut_ptr(),
    );
    optstart = sin;
    if sout > optstart {
        optstart = sout;
    }
    descend = 0 as libc::c_int;
    s = findstatement(
        step,
        b"by\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    );
    if s != 0 {
        while *step.offset(s as isize) as libc::c_int != 0
            && *step.offset(s as isize) as libc::c_int != ';' as i32
        {
            if linecmp(
                step.offset(s as isize),
                b"descending\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            ) == 0
            {
                s += 11 as libc::c_int;
                descend = 1 as libc::c_int;
            }
            while *step.offset(s as isize) as libc::c_int != 0
                && *step.offset(s as isize) as libc::c_int != '\n' as i32
            {
                putc(*step.offset(s as isize) as libc::c_int, dapfile);
                s += 1;
                s;
            }
            putc(' ' as i32, dapfile);
            s += 1;
            s;
        }
    } else {
        fprintf(
            stderr,
            b"sbstrans: before %d: missing by statement in proc sort.\n\0" as *const u8
                as *const libc::c_char,
            sbslineno,
        );
        exit(1 as libc::c_int);
    }
    fputs(b"\", \"\0" as *const u8 as *const libc::c_char, dapfile);
    if linecmp(
        step.offset(optstart as isize),
        b"nodupkey\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    ) == 0
    {
        putc('u' as i32, dapfile);
    }
    if descend != 0 {
        s = findstatement(
            step,
            b"by\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        );
        while *step.offset(s as isize) as libc::c_int != 0
            && *step.offset(s as isize) as libc::c_int != ';' as i32
        {
            if linecmp(
                step.offset(s as isize),
                b"descending\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            ) == 0
            {
                s += 11 as libc::c_int;
                while *step.offset(s as isize) as libc::c_int != 0
                    && *step.offset(s as isize) as libc::c_int != '\n' as i32
                {
                    s += 1;
                    s;
                }
                putc('d' as i32, dapfile);
            } else {
                while *step.offset(s as isize) as libc::c_int != 0
                    && *step.offset(s as isize) as libc::c_int != '\n' as i32
                {
                    s += 1;
                    s;
                }
                putc('i' as i32, dapfile);
            }
            s += 1;
            s;
        }
    }
    fputs(b"\");\n\0" as *const u8 as *const libc::c_char, dapfile);
    if outname[0 as libc::c_int as usize] == 0 {
        strcpy(outname.as_mut_ptr(), inname.as_mut_ptr());
    }
    strcat(inname.as_mut_ptr(), b".srt\0" as *const u8 as *const libc::c_char);
    fprintf(
        dapfile,
        b"dataset(\"%s\", \"%s\", \"RENAME\");\n\0" as *const u8 as *const libc::c_char,
        inname.as_mut_ptr(),
        outname.as_mut_ptr(),
    );
    strcpy(sbstmp.as_mut_ptr(), outname.as_mut_ptr());
}
pub unsafe extern "C" fn datasetstrans(
    mut step: *mut libc::c_char,
    mut dapfile: *mut FILE,
) {
    let mut oldname: [libc::c_char; 128] = [0; 128];
    let mut newname: [libc::c_char; 128] = [0; 128];
    let mut s: libc::c_int = 0;
    let mut t: libc::c_int = 0;
    s = findstatement(
        step,
        b"append\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    );
    if s != 0 {
        if getoption(
            step.offset(s as isize),
            b"base\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            newname.as_mut_ptr(),
            1 as libc::c_int,
        ) != 0
            || getoption(
                step.offset(s as isize),
                b"out\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
                newname.as_mut_ptr(),
                1 as libc::c_int,
            ) != 0
        {
            if !(getoption(
                step.offset(s as isize),
                b"data\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
                oldname.as_mut_ptr(),
                1 as libc::c_int,
            ) != 0
                || getoption(
                    step.offset(s as isize),
                    b"new\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
                    oldname.as_mut_ptr(),
                    1 as libc::c_int,
                ) != 0)
            {
                strcpy(oldname.as_mut_ptr(), sbstmp.as_mut_ptr());
            }
            fprintf(
                dapfile,
                b"dataset(\"%s\", \"%s\", \"APPEND\");\n\0" as *const u8
                    as *const libc::c_char,
                oldname.as_mut_ptr(),
                newname.as_mut_ptr(),
            );
        } else {
            fprintf(
                stderr,
                b"sbstrans: before %d: missing base or out statement in proc datasets.\n\0"
                    as *const u8 as *const libc::c_char,
                sbslineno,
            );
            exit(1 as libc::c_int);
        }
    }
    s = findstatement(
        step,
        b"change\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    );
    if s != 0 {
        while *step.offset(s as isize) as libc::c_int != 0
            && *step.offset(s as isize) as libc::c_int != ';' as i32
        {
            s
                += linecpy(oldname.as_mut_ptr(), step.offset(s as isize))
                    + 1 as libc::c_int;
            if linecmp(
                step.offset(s as isize),
                b"=\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            ) == 0
            {
                s += 2 as libc::c_int;
                t = linecpy(newname.as_mut_ptr(), step.offset(s as isize));
                if t != 0
                    && linecmp(
                        newname.as_mut_ptr(),
                        b";\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
                    ) != 0
                {
                    fprintf(
                        dapfile,
                        b"dataset(\"%s\", \"%s\", \"RENAME\");\n\0" as *const u8
                            as *const libc::c_char,
                        oldname.as_mut_ptr(),
                        newname.as_mut_ptr(),
                    );
                    s += t + 1 as libc::c_int;
                } else {
                    fprintf(
                        stderr,
                        b"sbstrans: before %d: missing filename after = in change statement in proc datasets.\n\0"
                            as *const u8 as *const libc::c_char,
                        sbslineno,
                    );
                    exit(1 as libc::c_int);
                }
            } else {
                fprintf(
                    stderr,
                    b"sbstrans: before %d: missing = after filename in change statement in proc datasets.\n\0"
                        as *const u8 as *const libc::c_char,
                    sbslineno,
                );
                exit(1 as libc::c_int);
            }
        }
    }
    s = findstatement(
        step,
        b"delete\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    );
    if s != 0 {
        while *step.offset(s as isize) as libc::c_int != 0
            && *step.offset(s as isize) as libc::c_int != ';' as i32
        {
            s
                += linecpy(oldname.as_mut_ptr(), step.offset(s as isize))
                    + 1 as libc::c_int;
            fprintf(
                dapfile,
                b"dataset(\"%s\", \"\", \"REMOVE\");\n\0" as *const u8
                    as *const libc::c_char,
                oldname.as_mut_ptr(),
            );
        }
    }
}
pub unsafe extern "C" fn freqtrans(mut step: *mut libc::c_char, mut dapfile: *mut FILE) {
    let mut s: libc::c_int = 0;
    let mut setname: [libc::c_char; 128] = [0; 128];
    let mut outname: [libc::c_char; 128] = [0; 128];
    let mut tablesstart: libc::c_int = 0;
    let mut optionsstart: libc::c_int = 0;
    let mut nstats: libc::c_int = 0;
    let mut stat: [libc::c_char; 128] = [0; 128];
    let mut nofreq: libc::c_int = 0;
    let mut nopercent: libc::c_int = 0;
    let mut norow: libc::c_int = 0;
    let mut nocol: libc::c_int = 0;
    let mut noprint: libc::c_int = 0;
    let mut nvars: libc::c_int = 0;
    let mut newvar: libc::c_int = 0;
    let mut varn: libc::c_int = 0;
    noprint = 0 as libc::c_int;
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
        b"sort(\"%s\", \"\0" as *const u8 as *const libc::c_char,
        setname.as_mut_ptr(),
    );
    s = findstatement(
        step,
        b"tables\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    );
    if s != 0 {
        tablesstart = s;
        copylist(
            step,
            b"by\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            dapfile,
        );
        putc(' ' as i32, dapfile);
        while *step.offset(s as isize) as libc::c_int != 0
            && *step.offset(s as isize) as libc::c_int != '/' as i32
            && *step.offset(s as isize) as libc::c_int != ';' as i32
        {
            if *step.offset(s as isize) as libc::c_int == '*' as i32 {
                putc(' ' as i32, dapfile);
                s += 1;
                s;
            } else if *step.offset(s as isize) as libc::c_int != '\n' as i32 {
                putc(*step.offset(s as isize) as libc::c_int, dapfile);
            }
            s += 1;
            s;
        }
    } else {
        fprintf(
            stderr,
            b"sbstrans: before %d: missing tables statement in proc freq.\n\0"
                as *const u8 as *const libc::c_char,
            sbslineno,
        );
        exit(1 as libc::c_int);
    }
    fputs(b"\", \"\");\n\0" as *const u8 as *const libc::c_char, dapfile);
    if *step.offset(s as isize) as libc::c_int == '/' as i32 {
        optionsstart = s + 2 as libc::c_int;
    } else {
        optionsstart = 0 as libc::c_int;
    }
    fprintf(
        dapfile,
        b"freq(\"%s.srt\", \"\0" as *const u8 as *const libc::c_char,
        setname.as_mut_ptr(),
    );
    s = tablesstart;
    newvar = 1 as libc::c_int;
    nvars = 0 as libc::c_int;
    while *step.offset(s as isize) as libc::c_int != 0
        && *step.offset(s as isize) as libc::c_int != '/' as i32
        && *step.offset(s as isize) as libc::c_int != ';' as i32
    {
        if *step.offset(s as isize) as libc::c_int == '*' as i32 {
            putc(' ' as i32, dapfile);
            s += 1;
            s;
            newvar = 1 as libc::c_int;
        } else if *step.offset(s as isize) as libc::c_int != '\n' as i32 {
            if newvar != 0 {
                newvar = 0 as libc::c_int;
                nvars += 1;
                nvars;
            }
            putc(*step.offset(s as isize) as libc::c_int, dapfile);
        }
        s += 1;
        s;
    }
    if nvars == 0 {
        fprintf(
            stderr,
            b"sbstrans: before %d: no variables in tables statement in proc freq.\n\0"
                as *const u8 as *const libc::c_char,
            sbslineno,
        );
        exit(1 as libc::c_int);
    }
    s = findstatement(
        step,
        b"weight\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    );
    if s != 0 {
        putc('*' as i32, dapfile);
        while *step.offset(s as isize) as libc::c_int != 0
            && *step.offset(s as isize) as libc::c_int != '\n' as i32
        {
            putc(*step.offset(s as isize) as libc::c_int, dapfile);
            s += 1;
            s;
        }
        s += 1;
        s;
        if *step.offset(s as isize) as libc::c_int != ';' as i32 {
            fprintf(
                stderr,
                b"sbstrans: before %d: only one weight variable allowed in proc freq.\n\0"
                    as *const u8 as *const libc::c_char,
                sbslineno,
            );
            exit(1 as libc::c_int);
        }
    }
    fputs(b"\", \"\0" as *const u8 as *const libc::c_char, dapfile);
    outname[0 as libc::c_int as usize] = '\0' as i32 as libc::c_char;
    nofreq = 0 as libc::c_int;
    nopercent = 0 as libc::c_int;
    norow = 0 as libc::c_int;
    nocol = 0 as libc::c_int;
    nstats = 4 as libc::c_int;
    s = optionsstart;
    while *step.offset(s as isize) as libc::c_int != 0
        && *step.offset(s as isize) as libc::c_int != ';' as i32
    {
        if linecmp(
            step.offset(s as isize),
            b"noprint\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        ) == 0
        {
            noprint = 1 as libc::c_int;
            s += 7 as libc::c_int;
        } else if linecmp(
            step.offset(s as isize),
            b"out\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        ) == 0
        {
            s += 4 as libc::c_int;
            if linecmp(
                step.offset(s as isize),
                b"=\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            ) != 0
            {
                fprintf(
                    stderr,
                    b"sbstrans: before %d: missing = after out option in tables statement in proc freq.\n\0"
                        as *const u8 as *const libc::c_char,
                    sbslineno,
                );
                exit(1 as libc::c_int);
            }
            s += 2 as libc::c_int;
            s += linecpy(outname.as_mut_ptr(), step.offset(s as isize));
        } else {
            s += linecpy(stat.as_mut_ptr(), step.offset(s as isize));
            upper(stat.as_mut_ptr());
            if linecmp(
                stat.as_mut_ptr(),
                b"NOFREQ\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            ) == 0
            {
                nofreq = 1 as libc::c_int;
                nstats -= 1;
                nstats;
            } else if linecmp(
                stat.as_mut_ptr(),
                b"NOPERCENT\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            ) == 0
            {
                nopercent = 1 as libc::c_int;
                nstats -= 1;
                nstats;
            } else if linecmp(
                stat.as_mut_ptr(),
                b"NOROW\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            ) == 0
            {
                norow = 1 as libc::c_int;
                nstats -= 1;
                nstats;
            } else if linecmp(
                stat.as_mut_ptr(),
                b"NOCOL\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            ) == 0
            {
                nocol = 1 as libc::c_int;
                nstats -= 1;
                nstats;
            } else {
                if linecmp(
                    stat.as_mut_ptr(),
                    b"EXPECTED\0" as *const u8 as *const libc::c_char
                        as *mut libc::c_char,
                ) == 0
                {
                    nstats += 1;
                    nstats;
                } else if linecmp(
                    stat.as_mut_ptr(),
                    b"CHISQ\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
                ) == 0
                {
                    fputs(b" FISHER \0" as *const u8 as *const libc::c_char, dapfile);
                } else if linecmp(
                    stat.as_mut_ptr(),
                    b"MEASURES\0" as *const u8 as *const libc::c_char
                        as *mut libc::c_char,
                ) == 0
                {
                    fputs(b" ODDSRAT \0" as *const u8 as *const libc::c_char, dapfile);
                    strcpy(
                        stat.as_mut_ptr(),
                        b"ORDINAL\0" as *const u8 as *const libc::c_char,
                    );
                }
                fputs(stat.as_mut_ptr(), dapfile);
            }
            putc(' ' as i32, dapfile);
        }
        s += 1;
        s;
    }
    if *step.offset(s as isize) as libc::c_int != ';' as i32 {
        fprintf(
            stderr,
            b"sbstrans: before %d: missing ; at end of tables statement in proc freq.\n\0"
                as *const u8 as *const libc::c_char,
            sbslineno,
        );
        exit(1 as libc::c_int);
    }
    if noprint == 0 {
        if nofreq == 0 {
            fputs(b" COUNT\0" as *const u8 as *const libc::c_char, dapfile);
        }
        if nopercent == 0 {
            fputs(b" PERCENT\0" as *const u8 as *const libc::c_char, dapfile);
        }
        if norow == 0 {
            fputs(b" ROWPERC\0" as *const u8 as *const libc::c_char, dapfile);
        }
        if nocol == 0 {
            fputs(b" COLPERC\0" as *const u8 as *const libc::c_char, dapfile);
        }
    }
    fputs(b"\", \"\0" as *const u8 as *const libc::c_char, dapfile);
    copylist(
        step,
        b"by\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        dapfile,
    );
    fputs(b"\");\n\0" as *const u8 as *const libc::c_char, dapfile);
    if noprint == 0 && nstats > 0 as libc::c_int {
        if nvars == 1 as libc::c_int {
            fprintf(
                dapfile,
                b"print(\"%s.srt.frq\", \"\");\n\0" as *const u8 as *const libc::c_char,
                setname.as_mut_ptr(),
            );
        } else {
            fprintf(
                dapfile,
                b"sort(\"%s.srt.frq\", \"\0" as *const u8 as *const libc::c_char,
                setname.as_mut_ptr(),
            );
            copylist(
                step,
                b"by\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
                dapfile,
            );
            s = tablesstart;
            varn = 0 as libc::c_int;
            while varn < nvars - 1 as libc::c_int {
                while *step.offset(s as isize) as libc::c_int != 0
                    && *step.offset(s as isize) as libc::c_int != '\n' as i32
                {
                    putc(*step.offset(s as isize) as libc::c_int, dapfile);
                    s += 1;
                    s;
                }
                putc(' ' as i32, dapfile);
                s += 3 as libc::c_int;
                varn += 1;
                varn;
            }
            fputs(b" _type_ \0" as *const u8 as *const libc::c_char, dapfile);
            while *step.offset(s as isize) as libc::c_int != 0
                && *step.offset(s as isize) as libc::c_int != '\n' as i32
            {
                putc(*step.offset(s as isize) as libc::c_int, dapfile);
                s += 1;
                s;
            }
            fputs(b"\", \"\");\n\0" as *const u8 as *const libc::c_char, dapfile);
            fprintf(
                dapfile,
                b"table(\"%s.srt.frq.srt\", \"\0" as *const u8 as *const libc::c_char,
                setname.as_mut_ptr(),
            );
            s = tablesstart;
            varn = 0 as libc::c_int;
            while varn < nvars - 2 as libc::c_int {
                while *step.offset(s as isize) as libc::c_int != 0
                    && *step.offset(s as isize) as libc::c_int != '\n' as i32
                {
                    s += 1;
                    s;
                }
                s += 3 as libc::c_int;
                varn += 1;
                varn;
            }
            while *step.offset(s as isize) as libc::c_int != 0
                && *step.offset(s as isize) as libc::c_int != '\n' as i32
            {
                putc(*step.offset(s as isize) as libc::c_int, dapfile);
                s += 1;
                s;
            }
            if nstats > 1 as libc::c_int {
                fputs(b" _type_\0" as *const u8 as *const libc::c_char, dapfile);
            }
            fputs(b"\", \"\0" as *const u8 as *const libc::c_char, dapfile);
            s += 3 as libc::c_int;
            while *step.offset(s as isize) as libc::c_int != 0
                && *step.offset(s as isize) as libc::c_int != '\n' as i32
            {
                putc(*step.offset(s as isize) as libc::c_int, dapfile);
                s += 1;
                s;
            }
            fputs(
                b" _cell_\", \"s12\", \"\0" as *const u8 as *const libc::c_char,
                dapfile,
            );
            copylist(
                step,
                b"by\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
                dapfile,
            );
            s = tablesstart;
            varn = 0 as libc::c_int;
            while varn < nvars - 2 as libc::c_int {
                while *step.offset(s as isize) as libc::c_int != 0
                    && *step.offset(s as isize) as libc::c_int != '\n' as i32
                {
                    putc(*step.offset(s as isize) as libc::c_int, dapfile);
                    s += 1;
                    s;
                }
                putc(' ' as i32, dapfile);
                s += 3 as libc::c_int;
                varn += 1;
                varn;
            }
            fputs(b"\");\n\0" as *const u8 as *const libc::c_char, dapfile);
        }
    }
    if outname[0 as libc::c_int as usize] != 0 {
        fprintf(
            dapfile,
            b"dataset(\"%s.srt.frq\", \"%s\", \"RENAME\");\n\0" as *const u8
                as *const libc::c_char,
            setname.as_mut_ptr(),
            outname.as_mut_ptr(),
        );
        strcpy(sbstmp.as_mut_ptr(), outname.as_mut_ptr());
    }
}
pub unsafe extern "C" fn tabulatetrans(
    mut step: *mut libc::c_char,
    mut dapfile: *mut FILE,
) {
    let mut s: libc::c_int = 0;
    let mut setname: [libc::c_char; 128] = [0; 128];
    let mut sortname: [libc::c_char; 132] = [0; 132];
    let mut format: [libc::c_char; 128] = [0; 128];
    let mut tablestart: libc::c_int = 0;
    if getoption(
        step,
        b"data\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        setname.as_mut_ptr(),
        1 as libc::c_int,
    ) == 0
    {
        strcpy(setname.as_mut_ptr(), sbstmp.as_mut_ptr());
    }
    strcpy(sortname.as_mut_ptr(), setname.as_mut_ptr());
    strcat(sortname.as_mut_ptr(), b".srt\0" as *const u8 as *const libc::c_char);
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
    tablestart = findstatement(
        step,
        b"table\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    );
    if tablestart != 0 {
        s = tablestart;
        while *step.offset(s as isize) as libc::c_int != 0
            && *step.offset(s as isize) as libc::c_int != '*' as i32
            && *step.offset(s as isize) as libc::c_int != ';' as i32
        {
            if *step.offset(s as isize) as libc::c_int == '\n' as i32 {
                putc(' ' as i32, dapfile);
            } else if *step.offset(s as isize) as libc::c_int == ',' as i32 {
                s += 1;
                s;
            } else {
                putc(*step.offset(s as isize) as libc::c_int, dapfile);
            }
            s += 1;
            s;
        }
        if *step.offset(s as isize) as libc::c_int != '*' as i32 {
            fprintf(
                stderr,
                b"sbstrans: before %d: no analysis variable in table statement in proc tabulate.\n\0"
                    as *const u8 as *const libc::c_char,
                sbslineno,
            );
            exit(1 as libc::c_int);
        }
    } else {
        fprintf(
            stderr,
            b"sbstrans: before %d: no table statement in proc tabulate.\n\0" as *const u8
                as *const libc::c_char,
            sbslineno,
        );
        exit(1 as libc::c_int);
    }
    fputs(b"\", \"\");\n\0" as *const u8 as *const libc::c_char, dapfile);
    fprintf(
        dapfile,
        b"table(\"%s\", \"\0" as *const u8 as *const libc::c_char,
        sortname.as_mut_ptr(),
    );
    if getoption(
        step,
        b"format\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        format.as_mut_ptr(),
        1 as libc::c_int,
    ) == 0
    {
        strcpy(format.as_mut_ptr(), b"12\0" as *const u8 as *const libc::c_char);
    }
    s = tablestart;
    while *step.offset(s as isize) as libc::c_int != 0
        && *step.offset(s as isize) as libc::c_int != ',' as i32
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
    if *step.offset(s as isize) as libc::c_int != ',' as i32 {
        fprintf(
            stderr,
            b"sbstrans: before %d: no column variables in table statement in proc tabulate.\n\0"
                as *const u8 as *const libc::c_char,
            sbslineno,
        );
        exit(1 as libc::c_int);
    }
    s += 2 as libc::c_int;
    fputs(b"\", \"\0" as *const u8 as *const libc::c_char, dapfile);
    while *step.offset(s as isize) as libc::c_int != 0
        && *step.offset(s as isize) as libc::c_int != '*' as i32
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
    if *step.offset(s as isize) as libc::c_int != '*' as i32 {
        fprintf(
            stderr,
            b"sbstrans: before %d: no analysis variable in table statement in proc tabulate.\n\0"
                as *const u8 as *const libc::c_char,
            sbslineno,
        );
        exit(1 as libc::c_int);
    }
    s += 2 as libc::c_int;
    while *step.offset(s as isize) as libc::c_int != 0
        && *step.offset(s as isize) as libc::c_int != '\n' as i32
    {
        putc(*step.offset(s as isize) as libc::c_int, dapfile);
        s += 1;
        s;
    }
    fprintf(
        dapfile,
        b"\", \"%s \0" as *const u8 as *const libc::c_char,
        format.as_mut_ptr(),
    );
    s += 1;
    s;
    if *step.offset(s as isize) as libc::c_int == '/' as i32 {
        s += 2 as libc::c_int;
        if linecmp(
            step.offset(s as isize),
            b"rtspace\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        ) == 0
        {
            s += 8 as libc::c_int;
        } else if linecmp(
            step.offset(s as isize),
            b"rts\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        ) == 0
        {
            s += 4 as libc::c_int;
        } else {
            fprintf(
                stderr,
                b"sbstrans: before %d: bad option in table statement in proc tabulate.\n\0"
                    as *const u8 as *const libc::c_char,
                sbslineno,
            );
            exit(1 as libc::c_int);
        }
        if linecmp(
            step.offset(s as isize),
            b"=\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        ) == 0
        {
            s += 2 as libc::c_int;
            while *step.offset(s as isize) as libc::c_int != 0
                && *step.offset(s as isize) as libc::c_int != '\n' as i32
            {
                putc(*step.offset(s as isize) as libc::c_int, dapfile);
                s += 1;
                s;
            }
        } else {
            fprintf(
                stderr,
                b"sbstrans: before %d: bad format for rtspace in table statement in proc tabulate.\n\0"
                    as *const u8 as *const libc::c_char,
                sbslineno,
            );
            exit(1 as libc::c_int);
        }
    } else if *step.offset(s as isize) as libc::c_int != ';' as i32 {
        fprintf(
            stderr,
            b"sbstrans: before %d: extra characters at end of table statement in proc tabulate.\n\0"
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
pub unsafe extern "C" fn corrtrans(mut step: *mut libc::c_char, mut dapfile: *mut FILE) {
    let mut s: libc::c_int = 0;
    let mut setname: [libc::c_char; 128] = [0; 128];
    let mut outname: [libc::c_char; 128] = [0; 128];
    let mut noprint: libc::c_int = 0;
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
        b"corr(\"%s\", \"\0" as *const u8 as *const libc::c_char,
        setname.as_mut_ptr(),
    );
    copylist(
        step,
        b"var\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        dapfile,
    );
    fputs(b"\", \"\0" as *const u8 as *const libc::c_char, dapfile);
    copylist(
        step,
        b"by\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        dapfile,
    );
    fputs(b"\");\n\0" as *const u8 as *const libc::c_char, dapfile);
    noprint = 0 as libc::c_int;
    s = 0 as libc::c_int;
    while *step.offset(s as isize) as libc::c_int != 0
        && *step.offset(s as isize) as libc::c_int != ';' as i32
    {
        if linecmp(
            step.offset(s as isize),
            b"noprint\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        ) == 0
        {
            noprint += 1;
            noprint;
            break;
        } else {
            while *step.offset(s as isize) as libc::c_int != 0
                && *step.offset(s as isize) as libc::c_int != '\n' as i32
            {
                s += 1;
                s;
            }
            s += 1;
            s;
        }
    }
    if noprint == 0 {
        fprintf(
            dapfile,
            b"sort(\"%s.cor\", \"\0" as *const u8 as *const libc::c_char,
            setname.as_mut_ptr(),
        );
        copylist(
            step,
            b"by\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            dapfile,
        );
        fputs(
            b" _var1_ _type_ _var2_\", \"\");\n\0" as *const u8 as *const libc::c_char,
            dapfile,
        );
        fprintf(
            dapfile,
            b"table(\"%s.cor.srt\", \"_var1_ _type_\", \"_var2_ _corr_\", \"s12\", \"\0"
                as *const u8 as *const libc::c_char,
            setname.as_mut_ptr(),
        );
        copylist(
            step,
            b"by\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            dapfile,
        );
        fputs(b"\");\n\0" as *const u8 as *const libc::c_char, dapfile);
    }
    if getoption(
        step,
        b"outp\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        outname.as_mut_ptr(),
        1 as libc::c_int,
    ) != 0
    {
        fprintf(
            dapfile,
            b"dataset(\"%s.cor\", \"%s\", \"RENAME\");\n\0" as *const u8
                as *const libc::c_char,
            setname.as_mut_ptr(),
            outname.as_mut_ptr(),
        );
        strcpy(sbstmp.as_mut_ptr(), outname.as_mut_ptr());
    }
}
pub unsafe extern "C" fn ranktrans(mut step: *mut libc::c_char, mut dapfile: *mut FILE) {
    let mut s: libc::c_int = 0;
    let mut setname: [libc::c_char; 132] = [0; 132];
    let mut outname: [libc::c_char; 128] = [0; 128];
    let mut option: [libc::c_char; 128] = [0; 128];
    let mut ngroups: libc::c_int = 0;
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
        b"group(\"%s\", \"\0" as *const u8 as *const libc::c_char,
        setname.as_mut_ptr(),
    );
    ngroups = 0 as libc::c_int;
    if getoption(
        step,
        b"groups\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        option.as_mut_ptr(),
        1 as libc::c_int,
    ) != 0
    {
        if sscanf(
            option.as_mut_ptr(),
            b"%d\0" as *const u8 as *const libc::c_char,
            &mut ngroups as *mut libc::c_int,
        ) != 1 as libc::c_int || ngroups <= 0 as libc::c_int
        {
            fprintf(
                stderr,
                b"sbstrans: before %d: bad number %s of groups in proc rank\n\0"
                    as *const u8 as *const libc::c_char,
                sbslineno,
                option.as_mut_ptr(),
            );
            exit(1 as libc::c_int);
        }
    }
    s = 0 as libc::c_int;
    while *step.offset(s as isize) as libc::c_int != 0
        && *step.offset(s as isize) as libc::c_int != ';' as i32
    {
        s += linecpy(option.as_mut_ptr(), step.offset(s as isize)) + 1 as libc::c_int;
        if linecmp(
            option.as_mut_ptr(),
            b"groups\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        ) == 0
            || linecmp(
                option.as_mut_ptr(),
                b"data\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            ) == 0
            || linecmp(
                option.as_mut_ptr(),
                b"out\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            ) == 0
        {
            s += 2 as libc::c_int;
            while alphanum(*step.offset(s as isize) as libc::c_int) != 0 {
                s += 1;
                s;
            }
            s += 1;
            s;
        } else if ngroups != 0 {
            fprintf(
                stderr,
                b"sbstrans: before %d: can't combine %s with groups= in proc rank\n\0"
                    as *const u8 as *const libc::c_char,
                sbslineno,
                option.as_mut_ptr(),
            );
            exit(1 as libc::c_int);
        } else if linecmp(
            option.as_mut_ptr(),
            b"fraction\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        ) == 0
            || linecmp(
                option.as_mut_ptr(),
                b"f\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            ) == 0
        {
            fputs(b"/ \0" as *const u8 as *const libc::c_char, dapfile);
        } else if linecmp(
            option.as_mut_ptr(),
            b"percent\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        ) == 0
            || linecmp(
                option.as_mut_ptr(),
                b"p\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            ) == 0
        {
            fputs(b"% \0" as *const u8 as *const libc::c_char, dapfile);
        } else {
            fprintf(
                stderr,
                b"sbstrans: before %d: invalid option %s for proc rank\n\0" as *const u8
                    as *const libc::c_char,
                sbslineno,
                option.as_mut_ptr(),
            );
            exit(1 as libc::c_int);
        }
    }
    s = findstatement(
        step,
        b"var\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    );
    if s != 0 {
        while *step.offset(s as isize) as libc::c_int != 0
            && *step.offset(s as isize) as libc::c_int != ';' as i32
        {
            if *step.offset(s as isize) as libc::c_int == '\n' as i32 {
                if ngroups != 0 {
                    fprintf(
                        dapfile,
                        b" %d# \0" as *const u8 as *const libc::c_char,
                        ngroups,
                    );
                } else {
                    putc(' ' as i32, dapfile);
                }
            } else {
                putc(*step.offset(s as isize) as libc::c_int, dapfile);
            }
            s += 1;
            s;
        }
    } else {
        fprintf(
            stderr,
            b"sbstrans: before %d: missing var statement in proc rank\n\0" as *const u8
                as *const libc::c_char,
            sbslineno,
            option.as_mut_ptr(),
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
    if getoption(
        step,
        b"out\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        outname.as_mut_ptr(),
        1 as libc::c_int,
    ) == 0
    {
        sbstempnum += 1;
        sprintf(
            sbstmp.as_mut_ptr(),
            b"sbstmp%02d\0" as *const u8 as *const libc::c_char,
            sbstempnum,
        );
        strcpy(outname.as_mut_ptr(), sbstmp.as_mut_ptr());
    }
    fprintf(
        dapfile,
        b"dataset(\"%s.grp\", \"%s\", \"RENAME\");\n\0" as *const u8
            as *const libc::c_char,
        setname.as_mut_ptr(),
        outname.as_mut_ptr(),
    );
    strcpy(sbstmp.as_mut_ptr(), outname.as_mut_ptr());
}
pub unsafe extern "C" fn univariatetrans(
    mut step: *mut libc::c_char,
    mut dapfile: *mut FILE,
) {
    let mut s: libc::c_int = 0;
    let mut setname: [libc::c_char; 128] = [0; 128];
    let mut outname: [libc::c_char; 128] = [0; 128];
    let mut varname: [libc::c_char; 128] = [0; 128];
    let mut noprint: libc::c_int = 0;
    let mut normal: libc::c_int = 0;
    let mut plot: libc::c_int = 0;
    let mut statsreq: libc::c_int = 0;
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
        b"pctiles(\"%s\", \"\0" as *const u8 as *const libc::c_char,
        setname.as_mut_ptr(),
    );
    copylist(
        step,
        b"var\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        dapfile,
    );
    if findstatement(
        step,
        b"weight\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    ) != 0
    {
        putc('*' as i32, dapfile);
        copylist(
            step,
            b"weight\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            dapfile,
        );
    }
    fputs(b"\", \"\0" as *const u8 as *const libc::c_char, dapfile);
    statsreq = 0 as libc::c_int;
    outname[0 as libc::c_int as usize] = '\0' as i32 as libc::c_char;
    s = findstatement(
        step,
        b"output\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    );
    if s != 0 {
        if getoption(
            step.offset(s as isize),
            b"out\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            outname.as_mut_ptr(),
            1 as libc::c_int,
        ) == 0
        {
            fprintf(
                stderr,
                b"sbstrans: before %d: bad option for output in proc univariate.\n\0"
                    as *const u8 as *const libc::c_char,
                sbslineno,
            );
            exit(1 as libc::c_int);
        }
        while *step.offset(s as isize) as libc::c_int != 0
            && *step.offset(s as isize) as libc::c_int != ';' as i32
        {
            if linecmp(
                step.offset(s as isize),
                b"out\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            ) == 0
            {
                s += 6 as libc::c_int;
                while *step.offset(s as isize) as libc::c_int != 0
                    && *step.offset(s as isize) as libc::c_int != '\n' as i32
                {
                    s += 1;
                    s;
                }
            } else if linecmp(
                step.offset(s as isize),
                b"pctlpts\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            ) == 0
            {
                statsreq = 1 as libc::c_int;
                s += 8 as libc::c_int;
                if linecmp(
                    step.offset(s as isize),
                    b"=\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
                ) != 0
                {
                    fprintf(
                        stderr,
                        b"sbstrans: before %d: missing = after pctlpts in proc univariate.\n\0"
                            as *const u8 as *const libc::c_char,
                        sbslineno,
                    );
                    exit(1 as libc::c_int);
                }
                s += 2 as libc::c_int;
                while num(*step.offset(s as isize) as libc::c_int) != 0 {
                    putc('P' as i32, dapfile);
                    while *step.offset(s as isize) as libc::c_int != 0
                        && *step.offset(s as isize) as libc::c_int != '\n' as i32
                    {
                        putc(*step.offset(s as isize) as libc::c_int, dapfile);
                        s += 1;
                        s;
                    }
                    putc(' ' as i32, dapfile);
                    s += 1;
                    s;
                }
                s -= 1;
                s;
            } else {
                statsreq = 1 as libc::c_int;
                while *step.offset(s as isize) as libc::c_int != 0
                    && *step.offset(s as isize) as libc::c_int != '\n' as i32
                {
                    if 'a' as i32 <= *step.offset(s as isize) as libc::c_int
                        && *step.offset(s as isize) as libc::c_int <= 'z' as i32
                    {
                        putc(
                            *step.offset(s as isize) as libc::c_int + 'A' as i32
                                - 'a' as i32,
                            dapfile,
                        );
                    } else {
                        putc(*step.offset(s as isize) as libc::c_int, dapfile);
                    }
                    s += 1;
                    s;
                }
                putc(' ' as i32, dapfile);
            }
            s += 1;
            s;
        }
    }
    if statsreq == 0 {
        fputs(
            b"MAX MED MIN N P1 P5 P10 P90 P95 P99 Q1 Q3 QRANGE RANGE\0" as *const u8
                as *const libc::c_char,
            dapfile,
        );
    }
    fputs(b"\", \"\0" as *const u8 as *const libc::c_char, dapfile);
    copylist(
        step,
        b"by\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        dapfile,
    );
    fputs(b"\");\n\0" as *const u8 as *const libc::c_char, dapfile);
    s = 0 as libc::c_int;
    noprint = 0 as libc::c_int;
    normal = 0 as libc::c_int;
    plot = 0 as libc::c_int;
    while *step.offset(s as isize) as libc::c_int != 0
        && *step.offset(s as isize) as libc::c_int != ';' as i32
    {
        if linecmp(
            step.offset(s as isize),
            b"noprint\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        ) == 0
        {
            noprint = 1 as libc::c_int;
            s += 7 as libc::c_int;
        } else if linecmp(
            step.offset(s as isize),
            b"data\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        ) == 0
        {
            s += 7 as libc::c_int;
            while *step.offset(s as isize) as libc::c_int != 0
                && *step.offset(s as isize) as libc::c_int != '\n' as i32
            {
                s += 1;
                s;
            }
        } else if linecmp(
            step.offset(s as isize),
            b"normal\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        ) == 0
        {
            normal = 1 as libc::c_int;
            s += 6 as libc::c_int;
        } else if linecmp(
            step.offset(s as isize),
            b"plot\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        ) == 0
        {
            plot = 1 as libc::c_int;
            s += 4 as libc::c_int;
        } else {
            fprintf(
                stderr,
                b"sbstrans: before %d: invalid option for proc univariate.\n\0"
                    as *const u8 as *const libc::c_char,
                sbslineno,
            );
            exit(1 as libc::c_int);
        }
        s += 1;
        s;
    }
    if noprint == 0 {
        fprintf(
            dapfile,
            b"print(\"%s.pct\", \"\");\n\0" as *const u8 as *const libc::c_char,
            setname.as_mut_ptr(),
        );
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
                s += 1;
                s;
                fputs(b"\", \"\0" as *const u8 as *const libc::c_char, dapfile);
                copylist(
                    step,
                    b"by\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
                    dapfile,
                );
                fputs(b"\");\n\0" as *const u8 as *const libc::c_char, dapfile);
            }
        }
    }
    if outname[0 as libc::c_int as usize] != 0 {
        fprintf(
            dapfile,
            b"dataset(\"%s.pct\", \"%s\", \"RENAME\");\n\0" as *const u8
                as *const libc::c_char,
            setname.as_mut_ptr(),
            outname.as_mut_ptr(),
        );
        strcpy(sbstmp.as_mut_ptr(), outname.as_mut_ptr());
    }
    if normal != 0 {
        s = findstatement(
            step,
            b"var\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        );
        if s != 0 {
            while *step.offset(s as isize) as libc::c_int != 0
                && *step.offset(s as isize) as libc::c_int != ';' as i32
            {
                s
                    += linecpy(varname.as_mut_ptr(), step.offset(s as isize))
                        + 1 as libc::c_int;
                if plot != 0 {
                    if isby(step) >= 0 as libc::c_int {
                        countparts(step, setname.as_mut_ptr(), dapfile);
                    } else {
                        fputs(
                            b"_sbspictcnt_[_sbsnpicts_] = 1;\n\0" as *const u8
                                as *const libc::c_char,
                            dapfile,
                        );
                    }
                    fputs(
                        b"_sbspict_[_sbsnpicts_] = \0" as *const u8
                            as *const libc::c_char,
                        dapfile,
                    );
                }
                fprintf(
                    dapfile,
                    b"normal(\"%s\", \"%s\", \"\0" as *const u8 as *const libc::c_char,
                    setname.as_mut_ptr(),
                    varname.as_mut_ptr(),
                );
                copylist(
                    step,
                    b"by\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
                    dapfile,
                );
                fprintf(
                    dapfile,
                    b"\", %d);\n\0" as *const u8 as *const libc::c_char,
                    if plot != 0 { 256 as libc::c_int } else { 0 as libc::c_int },
                );
                if plot != 0 {
                    fputs(
                        b"_sbspictpage_[_sbsnpicts_++] = 1;\n\0" as *const u8
                            as *const libc::c_char,
                        dapfile,
                    );
                }
            }
        } else {
            fprintf(
                stderr,
                b"sbstrans: before %d: var statement required for normality testing proc univariate.\n\0"
                    as *const u8 as *const libc::c_char,
                sbslineno,
            );
            exit(1 as libc::c_int);
        }
        if plot != 0 {
            sbshaspicts = 1 as libc::c_int;
        }
    }
}
