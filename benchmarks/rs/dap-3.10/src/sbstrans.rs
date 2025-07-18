use ::libc;
extern "C" {
    pub type _IO_wide_data;
    pub type _IO_codecvt;
    pub type _IO_marker;
    static mut stderr: *mut FILE;
    fn fclose(__stream: *mut FILE) -> libc::c_int;
    fn fflush(__stream: *mut FILE) -> libc::c_int;
    fn fopen(_: *const libc::c_char, _: *const libc::c_char) -> *mut FILE;
    fn fprintf(_: *mut FILE, _: *const libc::c_char, _: ...) -> libc::c_int;
    fn sprintf(_: *mut libc::c_char, _: *const libc::c_char, _: ...) -> libc::c_int;
    fn sscanf(_: *const libc::c_char, _: *const libc::c_char, _: ...) -> libc::c_int;
    fn putc(__c: libc::c_int, __stream: *mut FILE) -> libc::c_int;
    fn fputs(__s: *const libc::c_char, __stream: *mut FILE) -> libc::c_int;
    static mut sbshaspicts: libc::c_int;
    static mut newline: libc::c_int;
    static mut incomment: libc::c_int;
    static mut inquote1: libc::c_int;
    static mut inquote2: libc::c_int;
    static mut escape: libc::c_int;
    fn dgetc(dotc: *mut FILE, dapc: *mut FILE, out: libc::c_int) -> libc::c_int;
    fn alpha(c: libc::c_int) -> libc::c_int;
    fn num(c: libc::c_int) -> libc::c_int;
    fn alphanum(c: libc::c_int) -> libc::c_int;
    fn printtrans(step: *mut libc::c_char, dapfile: *mut FILE);
    fn meanstrans(step: *mut libc::c_char, dapfile: *mut FILE);
fn unget1c(c: libc::c_int, dotc: *mut FILE, dapc: *mut FILE);
fn importtrans(step: *mut libc::c_char, dapfile: *mut FILE);
fn surveyselecttrans(step: *mut libc::c_char, dapfile: *mut FILE);
    fn sorttrans(step: *mut libc::c_char, dapfile: *mut FILE);
    fn charttrans(step: *mut libc::c_char, dapfile: *mut FILE);
    fn datasetstrans(step: *mut libc::c_char, dapfile: *mut FILE);
    fn freqtrans(step: *mut libc::c_char, dapfile: *mut FILE);
    fn tabulatetrans(step: *mut libc::c_char, dapfile: *mut FILE);
    fn corrtrans(step: *mut libc::c_char, dapfile: *mut FILE);
    fn plottrans(step: *mut libc::c_char, dapfile: *mut FILE);
    fn ranktrans(step: *mut libc::c_char, dapfile: *mut FILE);
    fn univariatetrans(step: *mut libc::c_char, dapfile: *mut FILE);
    fn glmtrans(step: *mut libc::c_char, dapfile: *mut FILE);
    fn logistictrans(step: *mut libc::c_char, dapfile: *mut FILE);
    fn npar1waytrans(step: *mut libc::c_char, dapfile: *mut FILE);
    fn regtrans(step: *mut libc::c_char, dapfile: *mut FILE);
    fn daptrans(step: *mut libc::c_char, dapfile: *mut FILE);
    fn exit(_: libc::c_int) -> !;
    fn strncmp(
        _: *const libc::c_char,
        _: *const libc::c_char,
        _: libc::c_ulong,
    ) -> libc::c_int;
    fn strlen(_: *const libc::c_char) -> libc::c_ulong;
    fn strcat(_: *mut libc::c_char, _: *const libc::c_char) -> *mut libc::c_char;
    fn strcpy(_: *mut libc::c_char, _: *const libc::c_char) -> *mut libc::c_char;
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
pub static mut sbstempnum: libc::c_int = 0;
pub static mut sbstmp: [libc::c_char; 128] = [0; 128];
pub static mut sbslineno: libc::c_int = 0;
static mut keyword: [*mut libc::c_char; 16] = [
    b"set\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"infile\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"input\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"length\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"merge\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"by\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"drop\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"keep\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"output\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"do\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"end\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"if\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"then\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"else\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"while\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
];
pub unsafe extern "C" fn iskeyword(mut str: *mut libc::c_char) -> libc::c_int {
    let mut k: libc::c_int = 0;
    k = 0 as libc::c_int;
    while *keyword[k as usize] != 0 {
        if linecmp(str, keyword[k as usize]) == 0 {
            return 1 as libc::c_int;
        }
        k += 1;
        k;
    }
    return 0 as libc::c_int;
}
pub unsafe extern "C" fn namecvt(mut sbsname: *mut libc::c_char) {
    let mut dot: libc::c_int = 0;
    dot = 0 as libc::c_int;
    while *sbsname.offset(dot as isize) != 0 {
        dot += 1;
        dot;
    }
    loop {
        dot -= 1;
        if !(dot >= 0 as libc::c_int
            && *sbsname.offset(dot as isize) as libc::c_int != '.' as i32)
        {
            break;
        }
    }
    if dot <= 0 as libc::c_int {
        fprintf(
            stderr,
            b"sbs: %s not a .sbs file\n\0" as *const u8 as *const libc::c_char,
            sbsname,
        );
        exit(1 as libc::c_int);
    }
    strcpy(sbsname.offset(dot as isize), b".c\0" as *const u8 as *const libc::c_char);
}
pub unsafe extern "C" fn is_space(mut c: libc::c_int) -> libc::c_int {
    return (c == -(1 as libc::c_int) || c == ' ' as i32 || c == '\t' as i32
        || c == '\n' as i32) as libc::c_int;
}
pub unsafe extern "C" fn sbsgetc(mut sbsfile: *mut FILE) -> libc::c_int {
    let mut c: libc::c_int = 0;
    c = dgetc(sbsfile, 0 as *mut FILE, 0 as libc::c_int);
    if c == '\n' as i32 {
        sbslineno += 1;
        sbslineno;
    }
    return c;
}
pub unsafe extern "C" fn sbstoken(
    mut sbsfile: *mut FILE,
    mut token: *mut libc::c_char,
) -> libc::c_int {
    let mut t: libc::c_int = 0;
    let mut c: libc::c_int = 0;
    let mut cprev: libc::c_int = 0;
    let mut quote: libc::c_int = 0;
    t = 0 as libc::c_int;
    loop {
        c = sbsgetc(sbsfile);
        if !(c == ' ' as i32 || c == '\t' as i32 || c == '\n' as i32 || incomment != 0) {
            break;
        }
    }
    if c == -(1 as libc::c_int) {
        *token.offset(0 as libc::c_int as isize) = '\0' as i32 as libc::c_char;
        return 0 as libc::c_int;
    }
    if 'a' as i32 <= c && c <= 'z' as i32 || 'A' as i32 <= c && c <= 'Z' as i32
        || c == '_' as i32
    {
        t = 0 as libc::c_int;
        while 'a' as i32 <= c && c <= 'z' as i32 || 'A' as i32 <= c && c <= 'Z' as i32
            || '0' as i32 <= c && c <= '9' as i32 || c == '_' as i32 || c == '.' as i32
        {
            if t < 127 as libc::c_int {
                let fresh0 = t;
                t = t + 1;
                *token.offset(fresh0 as isize) = c as libc::c_char;
            } else {
                *token.offset(t as isize) = '\0' as i32 as libc::c_char;
                fprintf(
                    stderr,
                    b"sbstrans: before %d: token too long: %s\n\0" as *const u8
                        as *const libc::c_char,
                    sbslineno,
                    token,
                );
                exit(1 as libc::c_int);
            }
            c = sbsgetc(sbsfile);
        }
    } else if c == '.' as i32 || '0' as i32 <= c && c <= '9' as i32 {
        t = 0 as libc::c_int;
        while c == '.' as i32 || '0' as i32 <= c && c <= '9' as i32 {
            if t < 127 as libc::c_int {
                let fresh1 = t;
                t = t + 1;
                *token.offset(fresh1 as isize) = c as libc::c_char;
            } else {
                *token.offset(t as isize) = '\0' as i32 as libc::c_char;
                fprintf(
                    stderr,
                    b"sbstrans: before %d: token too long: %s\n\0" as *const u8
                        as *const libc::c_char,
                    sbslineno,
                    token,
                );
                exit(1 as libc::c_int);
            }
            c = sbsgetc(sbsfile);
        }
    } else if c == '=' as i32 || c == '+' as i32 || c == '-' as i32 || c == '*' as i32
        || c == '/' as i32 || c == '>' as i32 || c == '<' as i32 || c == '^' as i32
        || c == '~' as i32 || c == '!' as i32 || c == '&' as i32 || c == '|' as i32
    {
        t = 0 as libc::c_int;
        let fresh2 = t;
        t = t + 1;
        *token.offset(fresh2 as isize) = c as libc::c_char;
        cprev = c;
        c = sbsgetc(sbsfile);
        if c == '=' as i32 || cprev == c {
            if t < 127 as libc::c_int {
                let fresh3 = t;
                t = t + 1;
                *token.offset(fresh3 as isize) = c as libc::c_char;
            } else {
                *token.offset(t as isize) = '\0' as i32 as libc::c_char;
                fprintf(
                    stderr,
                    b"sbstrans: before %d: token too long: %s\n\0" as *const u8
                        as *const libc::c_char,
                    sbslineno,
                    token,
                );
                exit(1 as libc::c_int);
            }
            c = sbsgetc(sbsfile);
        }
    } else if c == '"' as i32 || c == '\'' as i32 {
        quote = c;
        t = 0 as libc::c_int;
        let fresh4 = t;
        t = t + 1;
        *token.offset(fresh4 as isize) = c as libc::c_char;
        cprev = c;
        c = sbsgetc(sbsfile);
        while c != -(1 as libc::c_int) && c != quote {
            if c == '\n' as i32 {
                if !(cprev == '\\' as i32) {
                    break;
                }
                t -= 1;
                t;
            } else if t < 127 as libc::c_int {
                let fresh5 = t;
                t = t + 1;
                *token.offset(fresh5 as isize) = c as libc::c_char;
            } else {
                *token.offset(t as isize) = '\0' as i32 as libc::c_char;
                fprintf(
                    stderr,
                    b"sbstrans: before %d: token too long: %s\n\0" as *const u8
                        as *const libc::c_char,
                    sbslineno,
                    token,
                );
                exit(1 as libc::c_int);
            }
            cprev = c;
            c = sbsgetc(sbsfile);
        }
        if t < 127 as libc::c_int {
            let fresh6 = t;
            t = t + 1;
            *token.offset(fresh6 as isize) = c as libc::c_char;
        } else {
            *token.offset(t as isize) = '\0' as i32 as libc::c_char;
            fprintf(
                stderr,
                b"sbstrans: before %d: token too long: %s\n\0" as *const u8
                    as *const libc::c_char,
                sbslineno,
                token,
            );
            exit(1 as libc::c_int);
        }
        c = sbsgetc(sbsfile);
    } else {
        t = 0 as libc::c_int;
        let fresh7 = t;
        t = t + 1;
        *token.offset(fresh7 as isize) = c as libc::c_char;
        c = sbsgetc(sbsfile);
    }
    *token.offset(t as isize) = '\0' as i32 as libc::c_char;
    while c != -(1 as libc::c_int) && c == ' ' as i32 || c == '\t' as i32
        || c == '\n' as i32 || incomment != 0
    {
        c = sbsgetc(sbsfile);
    }
    if c != -(1 as libc::c_int) {
        unget1c(c, sbsfile, 0 as *mut FILE);
        if c == '\'' as i32 {
            inquote1 = (inquote1 == 0) as libc::c_int;
        } else if c == '"' as i32 {
            inquote2 = (inquote2 == 0) as libc::c_int;
        }
        if c == '\n' as i32 {
            sbslineno -= 1;
            sbslineno;
        }
    }
    return t;
}
pub unsafe extern "C" fn lineput(
    mut line: *mut libc::c_char,
    mut dapfile: *mut FILE,
) -> libc::c_int {
    let mut l: libc::c_int = 0;
    l = 0 as libc::c_int;
    while *line.offset(l as isize) as libc::c_int != 0
        && *line.offset(l as isize) as libc::c_int != '\n' as i32
    {
        putc(*line.offset(l as isize) as libc::c_int, dapfile);
        l += 1;
        l;
    }
    return l;
}
pub unsafe extern "C" fn getstatement(
    mut sbsfile: *mut FILE,
    mut statement: *mut libc::c_char,
) -> libc::c_int {
    let mut s: libc::c_int = 0;
    static mut token: [libc::c_char; 128] = [0; 128];
    let mut toklen: libc::c_int = 0;
    s = 0 as libc::c_int;
    loop {
        toklen = sbstoken(sbsfile, token.as_mut_ptr());
        if !(toklen != 0
            && linecmp(
                token.as_mut_ptr(),
                b";\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            ) != 0)
        {
            break;
        }
        if (s + toklen + 1 as libc::c_int) < 4095 as libc::c_int {
            strcpy(statement.offset(s as isize), token.as_mut_ptr());
            s += toklen;
            let fresh8 = s;
            s = s + 1;
            *statement.offset(fresh8 as isize) = '\n' as i32 as libc::c_char;
        }
    }
    if linecmp(
        token.as_mut_ptr(),
        b";\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    ) == 0
    {
        let fresh9 = s;
        s = s + 1;
        *statement.offset(fresh9 as isize) = ';' as i32 as libc::c_char;
        let fresh10 = s;
        s = s + 1;
        *statement.offset(fresh10 as isize) = '\n' as i32 as libc::c_char;
    }
    *statement.offset(s as isize) = '\0' as i32 as libc::c_char;
    return s;
}
pub unsafe extern "C" fn linecpy(
    mut l1: *mut libc::c_char,
    mut l2: *mut libc::c_char,
) -> libc::c_int {
    let mut start: *mut libc::c_char = 0 as *mut libc::c_char;
    start = l2;
    while *l2 as libc::c_int != 0 && *l2 as libc::c_int != '\n' as i32 {
        if !l1.is_null() {
            let fresh11 = l2;
            l2 = l2.offset(1);
            let fresh12 = l1;
            l1 = l1.offset(1);
            *fresh12 = *fresh11;
        } else {
            l2 = l2.offset(1);
            l2;
        }
    }
    if !l1.is_null() {
        *l1 = '\0' as i32 as libc::c_char;
    }
    return l2.offset_from(start) as libc::c_long as libc::c_int;
}
pub unsafe extern "C" fn putlines(
    mut line: *mut libc::c_char,
    mut file: *mut FILE,
    mut term: libc::c_int,
) -> libc::c_int {
    let mut start: *mut libc::c_char = 0 as *mut libc::c_char;
    start = line;
    while *line as libc::c_int != 0 && *line as libc::c_int != term {
        if *line as libc::c_int == '\n' as i32 {
            putc(' ' as i32, file);
        } else {
            putc(*line as libc::c_int, file);
        }
        line = line.offset(1);
        line;
    }
    putc(' ' as i32, file);
    return line.offset_from(start) as libc::c_long as libc::c_int;
}
pub unsafe extern "C" fn lower(mut c: libc::c_int) -> libc::c_int {
    if 'A' as i32 <= c && c <= 'Z' as i32 {
        return c + 'a' as i32 - 'A' as i32
    } else {
        return c
    };
}
pub unsafe extern "C" fn linecmp(
    mut l1: *mut libc::c_char,
    mut l2: *mut libc::c_char,
) -> libc::c_int {
    while lower(*l1 as libc::c_int) == lower(*l2 as libc::c_int)
        && *l1 as libc::c_int != 0 && *l2 as libc::c_int != 0
        && *l1 as libc::c_int != '\n' as i32 && *l2 as libc::c_int != '\n' as i32
    {
        l1 = l1.offset(1);
        l1;
        l2 = l2.offset(1);
        l2;
    }
    return (*l1 as libc::c_int != 0 && *l1 as libc::c_int != '\n' as i32
        || *l2 as libc::c_int != 0 && *l2 as libc::c_int != '\n' as i32) as libc::c_int;
}
pub unsafe extern "C" fn getstep(
    mut sbsfile: *mut FILE,
    mut step: *mut libc::c_char,
) -> libc::c_int {
    static mut statement: [libc::c_char; 4096] = [0; 4096];
    let mut steptype: libc::c_int = 0;
    let mut s: libc::c_int = 0;
    static mut statelen: libc::c_int = -(1 as libc::c_int);
    if statelen < 0 as libc::c_int {
        statelen = getstatement(sbsfile, statement.as_mut_ptr());
        if statelen == 0 {
            return 0 as libc::c_int;
        }
    }
    if linecmp(
        statement.as_mut_ptr(),
        b"data\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    ) == 0
    {
        steptype = 1 as libc::c_int;
    } else if linecmp(
        statement.as_mut_ptr(),
        b"proc\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    ) == 0
    {
        steptype = 2 as libc::c_int;
    } else {
        return 0 as libc::c_int
    }
    strcpy(step, statement.as_mut_ptr());
    s = statelen;
    loop {
        statelen = getstatement(sbsfile, statement.as_mut_ptr());
        if !(statelen != 0) {
            break;
        }
        if linecmp(
            statement.as_mut_ptr(),
            b"data\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        ) == 0
            && (alpha(statement[5 as libc::c_int as usize] as libc::c_int) != 0
                || statement[5 as libc::c_int as usize] as libc::c_int == ';' as i32
                || statement[5 as libc::c_int as usize] as libc::c_int == '(' as i32)
        {
            break;
        }
        if linecmp(
            statement.as_mut_ptr(),
            b"proc\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        ) == 0 && alpha(statement[5 as libc::c_int as usize] as libc::c_int) != 0
        {
            break;
        }
        if (s + statelen + 1 as libc::c_int) < 65535 as libc::c_int {
            strcpy(step.offset(s as isize), statement.as_mut_ptr());
            s += statelen;
        } else {
            fprintf(
                stderr,
                b"sbstrans: before %d: %s step too long\n\0" as *const u8
                    as *const libc::c_char,
                sbslineno,
                if steptype == 1 as libc::c_int {
                    b"data\0" as *const u8 as *const libc::c_char
                } else {
                    b"proc\0" as *const u8 as *const libc::c_char
                },
            );
            exit(1 as libc::c_int);
        }
    }
    *step.offset(s as isize) = '\0' as i32 as libc::c_char;
    return steptype;
}
pub unsafe extern "C" fn findvar(
    mut varname: *mut libc::c_char,
    mut var: *mut [libc::c_char; 128],
    mut nvars: libc::c_int,
) -> libc::c_int {
    let mut v: libc::c_int = 0;
    v = 0 as libc::c_int;
    while v < nvars {
        if linecmp(varname, (*var.offset(v as isize)).as_mut_ptr()) == 0 {
            break;
        }
        v += 1;
        v;
    }
    if v < 512 as libc::c_int - 1 as libc::c_int {
        strcpy((*var.offset(v as isize)).as_mut_ptr(), varname);
    }
    return v;
}
pub unsafe extern "C" fn opfix(
    mut token: *mut libc::c_char,
    mut dapfile: *mut FILE,
) -> libc::c_int {
    let mut t: libc::c_int = 0;
    if inquote1 != 0 || inquote2 != 0 {
        return lineput(token, dapfile) + 1 as libc::c_int
    } else if linecmp(
        token,
        b"=\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    ) == 0
    {
        fputs(b"==\0" as *const u8 as *const libc::c_char, dapfile);
        return 2 as libc::c_int;
    } else if linecmp(
        token,
        b"^=\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    ) == 0
        || linecmp(
            token,
            b"~=\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        ) == 0
    {
        fputs(b"!=\0" as *const u8 as *const libc::c_char, dapfile);
        return 3 as libc::c_int;
    } else if linecmp(
        token,
        b"&\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    ) == 0
    {
        fputs(b"&&\0" as *const u8 as *const libc::c_char, dapfile);
        return 2 as libc::c_int;
    } else if linecmp(
        token,
        b"|\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    ) == 0
    {
        fputs(b"||\0" as *const u8 as *const libc::c_char, dapfile);
        return 2 as libc::c_int;
    } else if linecmp(
        token,
        b"^\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    ) == 0
        || linecmp(
            token,
            b"~\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        ) == 0
    {
        fputs(b"!\0" as *const u8 as *const libc::c_char, dapfile);
        return 2 as libc::c_int;
    } else if strncmp(
        token,
        b"first.\0" as *const u8 as *const libc::c_char,
        6 as libc::c_int as libc::c_ulong,
    ) == 0
    {
        putc('_' as i32, dapfile);
        t = 6 as libc::c_int;
        while *token.offset(t as isize) as libc::c_int != 0
            && *token.offset(t as isize) as libc::c_int != '\n' as i32
        {
            putc(*token.offset(t as isize) as libc::c_int, dapfile);
            t += 1;
            t;
        }
        putc('_' as i32, dapfile);
        return t + 1 as libc::c_int;
    } else if linecmp(
        token,
        b"[\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    ) == 0
    {
        fputs(b"[(int)(\0" as *const u8 as *const libc::c_char, dapfile);
        return 2 as libc::c_int;
    } else if linecmp(
        token,
        b"]\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    ) == 0
    {
        fputs(b")-1]\0" as *const u8 as *const libc::c_char, dapfile);
        return 2 as libc::c_int;
    } else {
        return lineput(token, dapfile) + 1 as libc::c_int
    };
}
pub unsafe extern "C" fn nonaction(mut step: *mut libc::c_char) -> *mut libc::c_char {
    static mut type_0: [libc::c_char; 128] = [0; 128];
    if linecmp(step, b"set\0" as *const u8 as *const libc::c_char as *mut libc::c_char)
        == 0
        || linecmp(
            step,
            b"infile\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        ) == 0
        || linecmp(
            step,
            b"input\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        ) == 0
        || linecmp(
            step,
            b"length\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        ) == 0
        || linecmp(
            step,
            b"merge\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        ) == 0
        || linecmp(
            step,
            b"by\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        ) == 0
        || linecmp(
            step,
            b"drop\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        ) == 0
        || linecmp(
            step,
            b"keep\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        ) == 0
        || linecmp(
            step,
            b"title\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        ) == 0
    {
        linecpy(type_0.as_mut_ptr(), step);
        return type_0.as_mut_ptr();
    }
    return 0 as *mut libc::c_char;
}
pub unsafe extern "C" fn statementtrans(
    mut step: *mut libc::c_char,
    mut dapfile: *mut FILE,
    mut isoutput: *mut libc::c_int,
) -> libc::c_int {
    let mut s: libc::c_int = 0;
    let mut statementtype: *mut libc::c_char = 0 as *mut libc::c_char;
    s = 0 as libc::c_int;
    statementtype = nonaction(step);
    if !statementtype.is_null() {
        while *step.offset(s as isize) as libc::c_int != 0
            && *step.offset(s as isize) as libc::c_int != ';' as i32
        {
            s += 1;
            s;
        }
        if *step.offset(s as isize) as libc::c_int == ';' as i32 {
            return s + 2 as libc::c_int
        } else {
            fprintf(
                stderr,
                b"sbstrans: before %d: missing ; after %s statement in data step\n\0"
                    as *const u8 as *const libc::c_char,
                sbslineno,
                statementtype,
            );
            exit(1 as libc::c_int);
        }
    } else if linecmp(
        step,
        b"output\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    ) == 0
    {
        fputs(b"output();\n\0" as *const u8 as *const libc::c_char, dapfile);
        s += 7 as libc::c_int;
        *isoutput = 1 as libc::c_int;
        statementtype = b"output\0" as *const u8 as *const libc::c_char
            as *mut libc::c_char;
    } else if linecmp(
        step,
        b"end\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    ) == 0
    {
        fputs(b"}\n\0" as *const u8 as *const libc::c_char, dapfile);
        s += 4 as libc::c_int;
        statementtype = b"end\0" as *const u8 as *const libc::c_char
            as *mut libc::c_char;
    } else if linecmp(
        step,
        b"if\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    ) == 0
    {
        s += 3 as libc::c_int;
        fputs(b"if (\0" as *const u8 as *const libc::c_char, dapfile);
        while *step.offset(s as isize) as libc::c_int != 0
            && linecmp(
                step.offset(s as isize),
                b"then\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            ) != 0
        {
            s += opfix(step.offset(s as isize), dapfile);
        }
        fputs(b")\n\0" as *const u8 as *const libc::c_char, dapfile);
        s += 5 as libc::c_int;
        s
            += statementtrans(step.offset(s as isize), dapfile, isoutput)
                - 2 as libc::c_int;
        statementtype = b"if\0" as *const u8 as *const libc::c_char as *mut libc::c_char;
    } else if linecmp(
        step,
        b"else\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    ) == 0
    {
        fputs(b"else\n\0" as *const u8 as *const libc::c_char, dapfile);
        s += 5 as libc::c_int;
        return s;
    } else if linecmp(
        step.offset(s as isize),
        b"do\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    ) == 0
    {
        s += 3 as libc::c_int;
        if linecmp(
            step.offset(s as isize),
            b"while\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        ) == 0
        {
            s += 6 as libc::c_int;
            if linecmp(
                step.offset(s as isize),
                b"(\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            ) != 0
            {
                fprintf(
                    stderr,
                    b"sbstrans: before %d: missing ( after do while\n\0" as *const u8
                        as *const libc::c_char,
                    sbslineno,
                );
                exit(1 as libc::c_int);
            }
            s += 2 as libc::c_int;
            fputs(b"while (\0" as *const u8 as *const libc::c_char, dapfile);
            while *step.offset(s as isize) as libc::c_int != 0
                && linecmp(
                    step.offset(s as isize),
                    b")\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
                ) != 0
            {
                s += opfix(step.offset(s as isize), dapfile);
            }
            fputs(b")\n{\n\0" as *const u8 as *const libc::c_char, dapfile);
            s += 2 as libc::c_int;
            statementtype = b"do while\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char;
        } else {
            fputs(b"{\n\0" as *const u8 as *const libc::c_char, dapfile);
            statementtype = b"do\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char;
        }
    } else {
        while *step.offset(s as isize) as libc::c_int != 0
            && *step.offset(s as isize) as libc::c_int != ';' as i32
        {
            if *step.offset(s as isize) as libc::c_int == '\n' as i32 {
                putc(' ' as i32, dapfile);
            } else if *step.offset(s as isize) as libc::c_int == '[' as i32 {
                fputs(b"[(int)(\0" as *const u8 as *const libc::c_char, dapfile);
            } else if *step.offset(s as isize) as libc::c_int == ']' as i32 {
                fputs(b")-1]\0" as *const u8 as *const libc::c_char, dapfile);
            } else {
                putc(*step.offset(s as isize) as libc::c_int, dapfile);
            }
            s += 1;
            s;
        }
        putc(';' as i32, dapfile);
        putc('\n' as i32, dapfile);
        statementtype = b"assignment\0" as *const u8 as *const libc::c_char
            as *mut libc::c_char;
    }
    if linecmp(
        step.offset(s as isize),
        b";\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    ) != 0
    {
        fprintf(
            stderr,
            b"sbstrans: before %d: missing ; after %s statement in data step\n\0"
                as *const u8 as *const libc::c_char,
            sbslineno,
            statementtype,
        );
        exit(1 as libc::c_int);
    }
    s += 2 as libc::c_int;
    return s;
}
pub unsafe extern "C" fn globaltrans(
    mut statement: *mut libc::c_char,
    mut dapfile: *mut FILE,
) {
    let mut s: libc::c_int = 0;
    if linecmp(
        statement,
        b"title\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    ) == 0
    {
        fputs(b"title(\0" as *const u8 as *const libc::c_char, dapfile);
        if *statement.offset(6 as libc::c_int as isize) as libc::c_int == '"' as i32 {
            s = 6 as libc::c_int;
            while *statement.offset(s as isize) as libc::c_int != 0
                && *statement.offset(s as isize) as libc::c_int != '\n' as i32
            {
                putc(*statement.offset(s as isize) as libc::c_int, dapfile);
                s += 1;
                s;
            }
            s += 1;
            s;
            if *statement.offset(s as isize) as libc::c_int != ';' as i32 {
                fprintf(
                    stderr,
                    b"sbstrans: before %d: missing ; at end of title statement\n\0"
                        as *const u8 as *const libc::c_char,
                    sbslineno,
                );
                exit(1 as libc::c_int);
            }
        } else if *statement.offset(6 as libc::c_int as isize) as libc::c_int
            == ';' as i32
        {
            fputs(b"NULL\0" as *const u8 as *const libc::c_char, dapfile);
        } else {
            fprintf(
                stderr,
                b"sbstrans: before %d: title must begin with \"\n\0" as *const u8
                    as *const libc::c_char,
                sbslineno,
            );
            exit(1 as libc::c_int);
        }
        fputs(b");\n\0" as *const u8 as *const libc::c_char, dapfile);
    } else {
        fprintf(
            stderr,
            b"sbstrans: before %d: unknown global statement: %s\n\0" as *const u8
                as *const libc::c_char,
            sbslineno,
            statement,
        );
        exit(1 as libc::c_int);
    };
}
pub unsafe extern "C" fn datatrans(mut step: *mut libc::c_char, mut dapfile: *mut FILE) {
    static mut inputname: [libc::c_char; 132] = [0; 132];
    static mut outputname: [libc::c_char; 128] = [0; 128];
    let mut setnum: libc::c_int = 0;
    static mut delim: [libc::c_char; 5] = [0; 5];
    static mut var: [[libc::c_char; 128]; 512] = [[0; 128]; 512];
    static mut len: [libc::c_int; 512] = [0; 512];
    let mut vn: libc::c_int = 0;
    let mut nvars: libc::c_int = 0;
    static mut varname: [libc::c_char; 128] = [0; 128];
    let mut v: libc::c_int = 0;
    let mut s: libc::c_int = 0;
    let mut sincr: libc::c_int = 0;
    let mut inputisfile: libc::c_int = 0;
    static mut skip: [libc::c_char; 128] = [0; 128];
    let mut inputisnull: libc::c_int = 0;
    let mut inputcolumn: libc::c_int = 0;
    let mut sic: libc::c_int = 0;
    let mut startcol: libc::c_int = 0;
    let mut endcol: libc::c_int = 0;
    let mut isdouble: libc::c_int = 0;
    let mut firstdec: libc::c_int = 0;
    let mut isoutput: libc::c_int = 0;
    let mut dropping: libc::c_int = 0;
    let mut keeping: libc::c_int = 0;
    let mut start: libc::c_int = 0;
    let mut bymark: libc::c_int = 0;
    let mut nby: libc::c_int = 0;
    let mut b: libc::c_int = 0;
    let mut i: libc::c_int = 0;
    nvars = 0 as libc::c_int;
    dropping = 0 as libc::c_int;
    keeping = 0 as libc::c_int;
    fputs(
        b"sbstrans: processing data step...\n\0" as *const u8 as *const libc::c_char,
        stderr,
    );
    fflush(stderr);
    bymark = isby(step);
    s = findstatement(
        step,
        b"title\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    );
    if s != 0 {
        globaltrans(
            step.offset(s as isize).offset(-(6 as libc::c_int as isize)),
            dapfile,
        );
    }
    inputisnull = 0 as libc::c_int;
    inputcolumn = 0 as libc::c_int;
    delim[0 as libc::c_int as usize] = '\0' as i32 as libc::c_char;
    inputisfile = findstatement(
        step,
        b"infile\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    );
    if inputisfile != 0 {
        s = inputisfile
            + linecpy(inputname.as_mut_ptr(), step.offset(inputisfile as isize))
            + 1 as libc::c_int;
        fprintf(
            dapfile,
            b"infile(%s, \0" as *const u8 as *const libc::c_char,
            inputname.as_mut_ptr(),
        );
        sincr = getoption(
            step.offset(s as isize),
            b"delimiter\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            delim.as_mut_ptr(),
            1 as libc::c_int,
        );
        if sincr != 0
            || {
                sincr = getoption(
                    step.offset(s as isize),
                    b"dlm\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
                    delim.as_mut_ptr(),
                    1 as libc::c_int,
                );
                sincr != 0
            }
        {
            s += sincr;
            fputs(delim.as_mut_ptr(), dapfile);
        } else {
            sic = findstatement(
                step,
                b"input\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            );
            if sic != 0 {
                while *step.offset(sic as isize) as libc::c_int != 0
                    && *step.offset(sic as isize) as libc::c_int != ';' as i32
                {
                    while *step.offset(sic as isize) as libc::c_int != 0
                        && *step.offset(sic as isize) as libc::c_int != '\n' as i32
                    {
                        sic += 1;
                        sic;
                    }
                    sic += 1;
                    sic;
                    if !(num(*step.offset(sic as isize) as libc::c_int) != 0) {
                        break;
                    }
                    if inputcolumn == 0 {
                        inputcolumn = 1 as libc::c_int;
                        putc('"' as i32, dapfile);
                        endcol = 0 as libc::c_int;
                    }
                    startcol = 0 as libc::c_int;
                    while *step.offset(sic as isize) as libc::c_int != 0
                        && num(*step.offset(sic as isize) as libc::c_int) != 0
                    {
                        startcol = 10 as libc::c_int * startcol
                            + *step.offset(sic as isize) as libc::c_int - '0' as i32;
                        sic += 1;
                        sic;
                    }
                    if *step.offset(sic as isize) as libc::c_int != '\n' as i32
                        || startcol != endcol + 1 as libc::c_int
                    {
                        fprintf(
                            stderr,
                            b"sbstrans: before %d: bad start column in input statement.\n\0"
                                as *const u8 as *const libc::c_char,
                            sbslineno,
                        );
                        exit(1 as libc::c_int);
                    }
                    sic += 1;
                    sic;
                    if *step.offset(sic as isize) as libc::c_int == '-' as i32 {
                        sic += 2 as libc::c_int;
                        endcol = 0 as libc::c_int;
                        while *step.offset(sic as isize) as libc::c_int != 0
                            && num(*step.offset(sic as isize) as libc::c_int) != 0
                        {
                            endcol = 10 as libc::c_int * endcol
                                + *step.offset(sic as isize) as libc::c_int - '0' as i32;
                            sic += 1;
                            sic;
                        }
                        if *step.offset(sic as isize) as libc::c_int != '\n' as i32 {
                            fprintf(
                                stderr,
                                b"sbstrans: before %d: bad end column in input statement.\n\0"
                                    as *const u8 as *const libc::c_char,
                                sbslineno,
                            );
                            exit(1 as libc::c_int);
                        }
                        sic += 1;
                        sic;
                    } else {
                        endcol = startcol;
                    }
                    fprintf(
                        dapfile,
                        b"x%d\0" as *const u8 as *const libc::c_char,
                        endcol - startcol + 1 as libc::c_int,
                    );
                }
            }
        }
        if inputcolumn != 0 {
            fputs(b"\"\0" as *const u8 as *const libc::c_char, dapfile);
        } else if delim[0 as libc::c_int as usize] == 0 {
            fputs(b"\" \"\0" as *const u8 as *const libc::c_char, dapfile);
        }
        fputs(b")\n{\n\0" as *const u8 as *const libc::c_char, dapfile);
    } else {
        s = findstatement(
            step,
            b"set\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        );
        if s != 0 {
            s
                += linecpy(inputname.as_mut_ptr(), step.offset(s as isize))
                    + 1 as libc::c_int;
            fprintf(
                dapfile,
                b"inset (\"%s\")\n{\n\0" as *const u8 as *const libc::c_char,
                inputname.as_mut_ptr(),
            );
            if *step.offset(s as isize) as libc::c_int != ';' as i32 {
                fprintf(
                    stderr,
                    b"sbstrans: before %d: missing ; or extra characters at end of set statement.\n\0"
                        as *const u8 as *const libc::c_char,
                    sbslineno,
                );
                exit(1 as libc::c_int);
            }
        } else {
            s = findstatement(
                step,
                b"merge\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            );
            if s != 0 {
                fputs(b"merge (\"\0" as *const u8 as *const libc::c_char, dapfile);
                setnum = 0 as libc::c_int;
                while setnum < 2 as libc::c_int {
                    s
                        += linecpy(inputname.as_mut_ptr(), step.offset(s as isize))
                            + 1 as libc::c_int;
                    fprintf(
                        dapfile,
                        b"%s\", \"\0" as *const u8 as *const libc::c_char,
                        inputname.as_mut_ptr(),
                    );
                    if linecmp(
                        step.offset(s as isize),
                        b"(\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
                    ) == 0
                    {
                        s += 2 as libc::c_int;
                        if linecmp(
                            step.offset(s as isize),
                            b"keep\0" as *const u8 as *const libc::c_char
                                as *mut libc::c_char,
                        ) == 0
                            || linecmp(
                                step.offset(s as isize),
                                b"drop\0" as *const u8 as *const libc::c_char
                                    as *mut libc::c_char,
                            ) == 0
                        {
                            if *step.offset(s as isize) as libc::c_int == 'd' as i32 {
                                putc('!' as i32, dapfile);
                            }
                            s += 5 as libc::c_int;
                            if linecmp(
                                step.offset(s as isize),
                                b"=\0" as *const u8 as *const libc::c_char
                                    as *mut libc::c_char,
                            ) != 0
                            {
                                fprintf(
                                    stderr,
                                    b"sbstrans: before %d: missing = after keep or drop option in merge statement.\n\0"
                                        as *const u8 as *const libc::c_char,
                                    sbslineno,
                                );
                                exit(1 as libc::c_int);
                            }
                            s += 2 as libc::c_int;
                            s += putlines(step.offset(s as isize), dapfile, ')' as i32);
                            if *step.offset(s as isize) as libc::c_int != ')' as i32 {
                                fprintf(
                                    stderr,
                                    b"sbstrans: before %d: missing ) after keep or drop option in merge statement.\n\0"
                                        as *const u8 as *const libc::c_char,
                                    sbslineno,
                                );
                                exit(1 as libc::c_int);
                            }
                            s += 2 as libc::c_int;
                        } else {
                            fprintf(
                                stderr,
                                b"sbstrans: before %d: invalid dataset option in merge statement.\n\0"
                                    as *const u8 as *const libc::c_char,
                                sbslineno,
                            );
                            exit(1 as libc::c_int);
                        }
                    }
                    fputs(b"\", \"\0" as *const u8 as *const libc::c_char, dapfile);
                    setnum += 1;
                    setnum;
                }
                if *step.offset(s as isize) as libc::c_int != ';' as i32 {
                    fprintf(
                        stderr,
                        b"sbstrans: before %d: missing ; at end of merge statement.\n\0"
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
                sbstempnum += 1;
                sprintf(
                    sbstmp.as_mut_ptr(),
                    b"sbstmp%02d\0" as *const u8 as *const libc::c_char,
                    sbstempnum,
                );
                strcpy(inputname.as_mut_ptr(), sbstmp.as_mut_ptr());
                fprintf(
                    dapfile,
                    b"\", \"%s\");\n\0" as *const u8 as *const libc::c_char,
                    sbstmp.as_mut_ptr(),
                );
                fprintf(
                    dapfile,
                    b"inset(\"%s\")\n{\n\0" as *const u8 as *const libc::c_char,
                    sbstmp.as_mut_ptr(),
                );
            } else {
                inputisnull = 1 as libc::c_int;
                fputs(
                    b"infile (NULL, NULL)\n{\n\0" as *const u8 as *const libc::c_char,
                    dapfile,
                );
            }
        }
    }
    s = findstatement(
        step,
        b"length\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    );
    if s != 0 {
        fputs(b"char \0" as *const u8 as *const libc::c_char, dapfile);
        firstdec = 1 as libc::c_int;
        while *step.offset(s as isize) as libc::c_int != 0
            && *step.offset(s as isize) as libc::c_int != ';' as i32
        {
            s
                += linecpy(varname.as_mut_ptr(), step.offset(s as isize))
                    + 1 as libc::c_int;
            if findvar(varname.as_mut_ptr(), var.as_mut_ptr(), nvars) == nvars {
                if firstdec != 0 {
                    firstdec = 0 as libc::c_int;
                } else {
                    putc(',' as i32, dapfile);
                }
                fprintf(
                    dapfile,
                    b"%s[\0" as *const u8 as *const libc::c_char,
                    varname.as_mut_ptr(),
                );
            } else {
                fprintf(
                    stderr,
                    b"sbstrans: before %d: redeclaration of %s\n\0" as *const u8
                        as *const libc::c_char,
                    sbslineno,
                    varname.as_mut_ptr(),
                );
                exit(1 as libc::c_int);
            }
            if linecmp(
                step.offset(s as isize),
                b"$\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            ) != 0
            {
                fprintf(
                    stderr,
                    b"sbstrans: before %d: missing $ in length statement for %s\n\0"
                        as *const u8 as *const libc::c_char,
                    sbslineno,
                    varname.as_mut_ptr(),
                );
                exit(1 as libc::c_int);
            }
            s += 2 as libc::c_int;
            s
                += linecpy(varname.as_mut_ptr(), step.offset(s as isize))
                    + 1 as libc::c_int;
            if sscanf(
                varname.as_mut_ptr(),
                b"%d\0" as *const u8 as *const libc::c_char,
                len.as_mut_ptr().offset(nvars as isize),
            ) == 0 || len[nvars as usize] <= 0 as libc::c_int
            {
                fprintf(
                    stderr,
                    b"sbstrans: before %d: bad length in length statement: %s\n\0"
                        as *const u8 as *const libc::c_char,
                    sbslineno,
                    varname.as_mut_ptr(),
                );
                exit(1 as libc::c_int);
            }
            fprintf(
                dapfile,
                b"%d]\0" as *const u8 as *const libc::c_char,
                len[nvars as usize] + 1 as libc::c_int,
            );
            nvars += 1;
            nvars;
        }
        if *step.offset(s as isize) as libc::c_int != ';' as i32 {
            fprintf(
                stderr,
                b"sbstrans: before %d: missing ; or extra characters at end of length statement.\n\0"
                    as *const u8 as *const libc::c_char,
                sbslineno,
            );
            exit(1 as libc::c_int);
        }
        fputs(b";\n\0" as *const u8 as *const libc::c_char, dapfile);
    }
    isdouble = 0 as libc::c_int;
    s = findstatement(
        step,
        b"input\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    );
    if s != 0 {
        while *step.offset(s as isize) as libc::c_int != 0
            && *step.offset(s as isize) as libc::c_int != ';' as i32
        {
            s
                += linecpy(varname.as_mut_ptr(), step.offset(s as isize))
                    + 1 as libc::c_int;
            if findvar(varname.as_mut_ptr(), var.as_mut_ptr(), nvars) == nvars {
                isdouble = 1 as libc::c_int;
                break;
            } else if inputcolumn != 0 {
                while *step.offset(s as isize) as libc::c_int != 0
                    && num(*step.offset(s as isize) as libc::c_int) != 0
                {
                    s += 1;
                    s;
                }
                s += 1;
                s;
                if *step.offset(s as isize) as libc::c_int == '-' as i32 {
                    s += 2 as libc::c_int;
                    while *step.offset(s as isize) as libc::c_int != 0
                        && num(*step.offset(s as isize) as libc::c_int) != 0
                    {
                        s += 1;
                        s;
                    }
                    s += 1;
                    s;
                }
            }
        }
    }
    s = 0 as libc::c_int;
    while *step.offset(s as isize) as libc::c_int != 0
        && *step.offset(s as isize) as libc::c_int != ';' as i32
    {
        s += 1;
        s;
    }
    if *step.offset(s as isize) == 0 {
        fprintf(
            stderr,
            b"sbstrans: before %d: null data step body or missing ; in data statement.\n\0"
                as *const u8 as *const libc::c_char,
            sbslineno,
        );
        exit(1 as libc::c_int);
    }
    s += 2 as libc::c_int;
    while isdouble == 0 && *step.offset(s as isize) as libc::c_int != 0 {
        if !(nonaction(step.offset(s as isize))).is_null() {
            while *step.offset(s as isize) as libc::c_int != 0
                && *step.offset(s as isize) as libc::c_int != ';' as i32
            {
                s += 1;
                s;
            }
        } else {
            while isdouble == 0 && *step.offset(s as isize) as libc::c_int != 0
                && *step.offset(s as isize) as libc::c_int != ';' as i32
            {
                if ('a' as i32 <= *step.offset(s as isize) as libc::c_int
                    && *step.offset(s as isize) as libc::c_int <= 'z' as i32
                    || *step.offset(s as isize) as libc::c_int == '_' as i32)
                    && iskeyword(step.offset(s as isize)) == 0
                {
                    s
                        += linecpy(varname.as_mut_ptr(), step.offset(s as isize))
                            + 1 as libc::c_int;
                    if !(*step.offset(s as isize) as libc::c_int != '(' as i32
                        && strncmp(
                            varname.as_mut_ptr(),
                            b"first.\0" as *const u8 as *const libc::c_char,
                            6 as libc::c_int as libc::c_ulong,
                        ) != 0
                        && findvar(varname.as_mut_ptr(), var.as_mut_ptr(), nvars)
                            == nvars)
                    {
                        continue;
                    }
                    isdouble = 1 as libc::c_int;
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
        }
        s += 2 as libc::c_int;
    }
    if isdouble != 0 {
        fputs(b"double \0" as *const u8 as *const libc::c_char, dapfile);
        firstdec = 1 as libc::c_int;
        s = findstatement(
            step,
            b"input\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        );
        if s != 0 {
            while *step.offset(s as isize) as libc::c_int != 0
                && *step.offset(s as isize) as libc::c_int != ';' as i32
            {
                s
                    += linecpy(varname.as_mut_ptr(), step.offset(s as isize))
                        + 1 as libc::c_int;
                if findvar(varname.as_mut_ptr(), var.as_mut_ptr(), nvars) == nvars {
                    if firstdec != 0 {
                        firstdec = 0 as libc::c_int;
                    } else {
                        putc(',' as i32, dapfile);
                    }
                    len[nvars as usize] = -(1 as libc::c_int);
                    nvars += 1;
                    nvars;
                    fputs(varname.as_mut_ptr(), dapfile);
                }
                if inputcolumn != 0 {
                    while *step.offset(s as isize) as libc::c_int != 0
                        && num(*step.offset(s as isize) as libc::c_int) != 0
                    {
                        s += 1;
                        s;
                    }
                    s += 1;
                    s;
                    if *step.offset(s as isize) as libc::c_int == '-' as i32 {
                        s += 2 as libc::c_int;
                        while *step.offset(s as isize) as libc::c_int != 0
                            && num(*step.offset(s as isize) as libc::c_int) != 0
                        {
                            s += 1;
                            s;
                        }
                        s += 1;
                        s;
                    }
                }
            }
        }
        s = 0 as libc::c_int;
        while *step.offset(s as isize) as libc::c_int != 0
            && *step.offset(s as isize) as libc::c_int != ';' as i32
        {
            s += 1;
            s;
        }
        if *step.offset(s as isize) == 0 {
            fprintf(
                stderr,
                b"sbstrans: before %d: null data step body or missing ; in data statement.\n\0"
                    as *const u8 as *const libc::c_char,
                sbslineno,
            );
            exit(1 as libc::c_int);
        }
        s += 2 as libc::c_int;
        while *step.offset(s as isize) != 0 {
            if !(nonaction(step.offset(s as isize))).is_null() {
                while *step.offset(s as isize) as libc::c_int != 0
                    && *step.offset(s as isize) as libc::c_int != ';' as i32
                {
                    s += 1;
                    s;
                }
            } else {
                while *step.offset(s as isize) as libc::c_int != 0
                    && *step.offset(s as isize) as libc::c_int != ';' as i32
                {
                    if ('a' as i32 <= *step.offset(s as isize) as libc::c_int
                        && *step.offset(s as isize) as libc::c_int <= 'z' as i32
                        || *step.offset(s as isize) as libc::c_int == '_' as i32)
                        && iskeyword(step.offset(s as isize)) == 0
                    {
                        s
                            += linecpy(varname.as_mut_ptr(), step.offset(s as isize))
                                + 1 as libc::c_int;
                        if *step.offset(s as isize) as libc::c_int != '(' as i32
                            && strncmp(
                                varname.as_mut_ptr(),
                                b"first.\0" as *const u8 as *const libc::c_char,
                                6 as libc::c_int as libc::c_ulong,
                            ) != 0
                            && findvar(varname.as_mut_ptr(), var.as_mut_ptr(), nvars)
                                == nvars
                        {
                            if firstdec != 0 {
                                firstdec = 0 as libc::c_int;
                            } else {
                                putc(',' as i32, dapfile);
                            }
                            fputs(varname.as_mut_ptr(), dapfile);
                            len[nvars as usize] = -(1 as libc::c_int);
                            nvars += 1;
                            nvars;
                        }
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
            }
            s += 2 as libc::c_int;
        }
        fputs(b";\n\0" as *const u8 as *const libc::c_char, dapfile);
    }
    if bymark >= 0 as libc::c_int {
        fputs(b"int\0" as *const u8 as *const libc::c_char, dapfile);
        s = bymark;
        nby = 0 as libc::c_int;
        while *step.offset(s as isize) as libc::c_int != 0
            && *step.offset(s as isize) as libc::c_int != ';' as i32
        {
            s
                += linecpy(varname.as_mut_ptr(), step.offset(s as isize))
                    + 1 as libc::c_int;
            if nby != 0 {
                putc(',' as i32, dapfile);
            }
            fprintf(
                dapfile,
                b" _%s_\0" as *const u8 as *const libc::c_char,
                varname.as_mut_ptr(),
            );
            nby += 1;
            nby;
        }
        fprintf(
            dapfile,
            b", _partv_[%d], _firstobs_;\n\0" as *const u8 as *const libc::c_char,
            nby,
        );
    }
    if inputisfile != 0 {
        s = findstatement(
            step,
            b"input\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        );
        if s != 0 {
            fputs(b"input(\"\0" as *const u8 as *const libc::c_char, dapfile);
            while *step.offset(s as isize) as libc::c_int != 0
                && *step.offset(s as isize) as libc::c_int != ';' as i32
            {
                while *step.offset(s as isize) as libc::c_int != 0
                    && *step.offset(s as isize) as libc::c_int != '\n' as i32
                {
                    putc(*step.offset(s as isize) as libc::c_int, dapfile);
                    s += 1;
                    s;
                }
                s += 1;
                s;
                putc(' ' as i32, dapfile);
                if inputcolumn != 0 {
                    while *step.offset(s as isize) as libc::c_int != 0
                        && num(*step.offset(s as isize) as libc::c_int) != 0
                    {
                        s += 1;
                        s;
                    }
                    s += 1;
                    s;
                    if *step.offset(s as isize) as libc::c_int == '-' as i32 {
                        s += 2 as libc::c_int;
                        while *step.offset(s as isize) as libc::c_int != 0
                            && num(*step.offset(s as isize) as libc::c_int) != 0
                        {
                            s += 1;
                            s;
                        }
                        s += 1;
                        s;
                    }
                }
            }
            fputs(b"\");\n\0" as *const u8 as *const libc::c_char, dapfile);
            if *step.offset(s as isize) as libc::c_int != ';' as i32 {
                fprintf(
                    stderr,
                    b"sbstrans: before %d: missing ; in input statement.\n\0"
                        as *const u8 as *const libc::c_char,
                    sbslineno,
                );
                exit(1 as libc::c_int);
            }
        } else {
            fprintf(
                stderr,
                b"sbstrans: before %d: infile statement present but missing input statement.\n\0"
                    as *const u8 as *const libc::c_char,
                sbslineno,
            );
            exit(1 as libc::c_int);
        }
    }
    s = 5 as libc::c_int;
    if 'a' as i32 <= *step.offset(s as isize) as libc::c_int
        && *step.offset(s as isize) as libc::c_int <= 'z' as i32
    {
        s
            += linecpy(outputname.as_mut_ptr(), step.offset(s as isize))
                + 1 as libc::c_int;
        strcpy(sbstmp.as_mut_ptr(), outputname.as_mut_ptr());
        if *step.offset(s as isize) as libc::c_int != '(' as i32
            && *step.offset(s as isize) as libc::c_int != ';' as i32
        {
            fprintf(
                stderr,
                b"sbstrans: before %d: missing ; or extra characters at end of data statement.\n\0"
                    as *const u8 as *const libc::c_char,
                sbslineno,
            );
            exit(1 as libc::c_int);
        }
    } else {
        sbstempnum += 1;
        sprintf(
            sbstmp.as_mut_ptr(),
            b"sbstmp%02d\0" as *const u8 as *const libc::c_char,
            sbstempnum,
        );
        strcpy(outputname.as_mut_ptr(), sbstmp.as_mut_ptr());
    }
    fprintf(
        dapfile,
        b"outset(\"%s\", \"\0" as *const u8 as *const libc::c_char,
        outputname.as_mut_ptr(),
    );
    if *step.offset(s as isize) as libc::c_int == '(' as i32 {
        s += 2 as libc::c_int;
        if linecmp(
            step.offset(s as isize),
            b"drop\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        ) == 0
        {
            dropping = 1 as libc::c_int;
            putc('!' as i32, dapfile);
        } else if linecmp(
            step.offset(s as isize),
            b"keep\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        ) == 0
        {
            keeping = 1 as libc::c_int;
        } else {
            fprintf(
                stderr,
                b"sbstrans: before %d: bad option for data statement.\n\0" as *const u8
                    as *const libc::c_char,
                sbslineno,
            );
            exit(1 as libc::c_int);
        }
        s += 5 as libc::c_int;
        if linecmp(
            step.offset(s as isize),
            b"=\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        ) != 0
        {
            fprintf(
                stderr,
                b"sbstrans: before %d: missing = after option name in data statement.\n\0"
                    as *const u8 as *const libc::c_char,
                sbslineno,
            );
            exit(1 as libc::c_int);
        }
        s += 2 as libc::c_int;
        s += putlines(step.offset(s as isize), dapfile, ')' as i32);
        if *step.offset(s as isize) as libc::c_int != ')' as i32 {
            fprintf(
                stderr,
                b"sbstrans: before %d: missing ) after option for data statement.\n\0"
                    as *const u8 as *const libc::c_char,
                sbslineno,
            );
            exit(1 as libc::c_int);
        }
        s += 2 as libc::c_int;
        if *step.offset(s as isize) as libc::c_int != ';' as i32 {
            fprintf(
                stderr,
                b"sbstrans: before %d: missing ; at end of data statement.\n\0"
                    as *const u8 as *const libc::c_char,
                sbslineno,
            );
            exit(1 as libc::c_int);
        }
    } else {
        s = findstatement(
            step,
            b"drop\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        );
        if s != 0 {
            dropping = 1 as libc::c_int;
            putc('!' as i32, dapfile);
            while *step.offset(s as isize) as libc::c_int != 0
                && *step.offset(s as isize) as libc::c_int != ';' as i32
            {
                if *step.offset(s as isize) as libc::c_int == '\n' as i32 {
                    putc(' ' as i32, dapfile);
                } else if alphanum(*step.offset(s as isize) as libc::c_int) != 0 {
                    putc(*step.offset(s as isize) as libc::c_int, dapfile);
                } else {
                    fprintf(
                        stderr,
                        b"sbstrans: before %d: invalid character %c in variable name in drop statement.\n\0"
                            as *const u8 as *const libc::c_char,
                        sbslineno,
                        *step.offset(s as isize) as libc::c_int,
                    );
                    exit(1 as libc::c_int);
                }
                s += 1;
                s;
            }
        } else {
            s = findstatement(
                step,
                b"keep\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            );
            if s != 0 {
                keeping = 1 as libc::c_int;
                s += putlines(step.offset(s as isize), dapfile, ';' as i32);
            }
        }
    }
    if (dropping != 0 || keeping == 0)
        && {
            s = bymark;
            s >= 0 as libc::c_int
        }
    {
        start = 1 as libc::c_int;
        if dropping == 0 {
            fputs(
                b"!_firstobs_ _partv_ \0" as *const u8 as *const libc::c_char,
                dapfile,
            );
        }
        while *step.offset(s as isize) as libc::c_int != 0
            && *step.offset(s as isize) as libc::c_int != ';' as i32
        {
            if start != 0 {
                putc('_' as i32, dapfile);
            }
            if *step.offset(s as isize) as libc::c_int == '\n' as i32 {
                fputs(b"_ \0" as *const u8 as *const libc::c_char, dapfile);
                start = 1 as libc::c_int;
            } else {
                putc(*step.offset(s as isize) as libc::c_int, dapfile);
                start = 0 as libc::c_int;
            }
            s += 1;
            s;
        }
    }
    fputs(b"\");\n\0" as *const u8 as *const libc::c_char, dapfile);
    s = bymark;
    if s >= 0 as libc::c_int {
        fputs(b"dap_list(\"\0" as *const u8 as *const libc::c_char, dapfile);
        copylist(
            step,
            b"by\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            dapfile,
        );
        fprintf(
            dapfile,
            b"\", _partv_, %d);\n\0" as *const u8 as *const libc::c_char,
            nby,
        );
        fputs(b"_firstobs_ = 1;\n\0" as *const u8 as *const libc::c_char, dapfile);
    }
    v = 0 as libc::c_int;
    while v < nvars {
        if len[v as usize] > 0 as libc::c_int {
            fprintf(
                dapfile,
                b"%s[%d] = '\\0';\n\0" as *const u8 as *const libc::c_char,
                (var[v as usize]).as_mut_ptr(),
                len[v as usize],
            );
        }
        v += 1;
        v;
    }
    if inputisnull == 0 {
        if inputisfile != 0
            && {
                s = getoption(
                    step.offset(inputisfile as isize),
                    b"firstobs\0" as *const u8 as *const libc::c_char
                        as *mut libc::c_char,
                    skip.as_mut_ptr(),
                    1 as libc::c_int,
                );
                s != 0
            }
        {
            fprintf(
                dapfile,
                b"skip(%s - 1);\n\0" as *const u8 as *const libc::c_char,
                skip.as_mut_ptr(),
            );
        }
        fputs(b"while (step())\n{\n\0" as *const u8 as *const libc::c_char, dapfile);
        s = bymark;
        if s >= 0 as libc::c_int {
            b = 1 as libc::c_int;
            while b <= nby {
                fprintf(
                    dapfile,
                    b"if (_firstobs_ || dap_newpart(_partv_, %d))\n\0" as *const u8
                        as *const libc::c_char,
                    b,
                );
                s
                    += linecpy(varname.as_mut_ptr(), step.offset(s as isize))
                        + 1 as libc::c_int;
                fprintf(
                    dapfile,
                    b"_%s_ = 1;\nelse _%s_ = 0;\n\0" as *const u8 as *const libc::c_char,
                    varname.as_mut_ptr(),
                    varname.as_mut_ptr(),
                );
                b += 1;
                b;
            }
        }
    }
    s = 0 as libc::c_int;
    while *step.offset(s as isize) as libc::c_int != 0
        && *step.offset(s as isize) as libc::c_int != ';' as i32
    {
        s += 1;
        s;
    }
    if *step.offset(s as isize) == 0 {
        fprintf(
            stderr,
            b"sbstrans: before %d: null data step body or missing ; in data statement.\n\0"
                as *const u8 as *const libc::c_char,
            sbslineno,
        );
        exit(1 as libc::c_int);
    }
    s += 2 as libc::c_int;
    isoutput = 0 as libc::c_int;
    while *step.offset(s as isize) != 0 {
        s += statementtrans(step.offset(s as isize), dapfile, &mut isoutput);
    }
    if isoutput == 0 {
        fputs(b"output();\n\0" as *const u8 as *const libc::c_char, dapfile);
    }
    if inputisnull == 0 {
        if bymark >= 0 as libc::c_int {
            fputs(b"_firstobs_ = 0;\n\0" as *const u8 as *const libc::c_char, dapfile);
        }
        fputs(b"}\n\0" as *const u8 as *const libc::c_char, dapfile);
    }
    fputs(b"}\n\0" as *const u8 as *const libc::c_char, dapfile);
}
pub unsafe extern "C" fn proctrans(mut step: *mut libc::c_char, mut dapfile: *mut FILE) {
    let mut s: libc::c_int = 0;
    let mut procname: [libc::c_char; 128] = [0; 128];
    linecpy(procname.as_mut_ptr(), step.offset(5 as libc::c_int as isize));
    fprintf(
        stderr,
        b"sbstrans: processing proc %s...\n\0" as *const u8 as *const libc::c_char,
        procname.as_mut_ptr(),
    );
    fflush(stderr);
    if linecmp(
        step.offset(5 as libc::c_int as isize),
        b"dap\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    ) != 0
        && {
            s = findstatement(
                step,
                b"title\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            );
            s != 0
        }
    {
        globaltrans(
            step.offset(s as isize).offset(-(6 as libc::c_int as isize)),
            dapfile,
        );
    }
    if linecmp(
        step.offset(5 as libc::c_int as isize),
        b"print\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    ) == 0
    {
        printtrans(step.offset(11 as libc::c_int as isize), dapfile);
    } else if linecmp(
        step.offset(5 as libc::c_int as isize),
        b"means\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    ) == 0
    {
        meanstrans(step.offset(11 as libc::c_int as isize), dapfile);
    } else if linecmp(
        step.offset(5 as libc::c_int as isize),
        b"sort\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    ) == 0
    {
        sorttrans(step.offset(10 as libc::c_int as isize), dapfile);
    } else if linecmp(
        step.offset(5 as libc::c_int as isize),
        b"chart\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    ) == 0
    {
        charttrans(step.offset(11 as libc::c_int as isize), dapfile);
    } else if linecmp(
        step.offset(5 as libc::c_int as isize),
        b"datasets\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    ) == 0
    {
        datasetstrans(step.offset(14 as libc::c_int as isize), dapfile);
    } else if linecmp(
        step.offset(5 as libc::c_int as isize),
        b"freq\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    ) == 0
    {
        freqtrans(step.offset(10 as libc::c_int as isize), dapfile);
    } else if linecmp(
        step.offset(5 as libc::c_int as isize),
        b"tabulate\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    ) == 0
    {
        tabulatetrans(step.offset(14 as libc::c_int as isize), dapfile);
    } else if linecmp(
        step.offset(5 as libc::c_int as isize),
        b"corr\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    ) == 0
    {
        corrtrans(step.offset(10 as libc::c_int as isize), dapfile);
    } else if linecmp(
        step.offset(5 as libc::c_int as isize),
        b"plot\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    ) == 0
    {
        plottrans(step.offset(10 as libc::c_int as isize), dapfile);
    } else if linecmp(
        step.offset(5 as libc::c_int as isize),
        b"rank\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    ) == 0
    {
        ranktrans(step.offset(10 as libc::c_int as isize), dapfile);
    } else if linecmp(
        step.offset(5 as libc::c_int as isize),
        b"univariate\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    ) == 0
    {
        univariatetrans(step.offset(16 as libc::c_int as isize), dapfile);
    } else if linecmp(
        step.offset(5 as libc::c_int as isize),
        b"glm\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    ) == 0
    {
        glmtrans(step.offset(9 as libc::c_int as isize), dapfile);
    } else if linecmp(
        step.offset(5 as libc::c_int as isize),
        b"logistic\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    ) == 0
    {
        logistictrans(step.offset(14 as libc::c_int as isize), dapfile);
    } else if linecmp(
        step.offset(5 as libc::c_int as isize),
        b"npar1way\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    ) == 0
    {
        npar1waytrans(step.offset(14 as libc::c_int as isize), dapfile);
    } else if linecmp(
        step.offset(5 as libc::c_int as isize),
        b"reg\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    ) == 0
    {
        regtrans(step.offset(9 as libc::c_int as isize), dapfile);
    } else if linecmp(
        step.offset(5 as libc::c_int as isize),
        b"dap\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    ) == 0
    {
        daptrans(step.offset(9 as libc::c_int as isize), dapfile);
    } else if linecmp(
        step.offset(5 as libc::c_int as isize),
        b"import\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    ) == 0
    {
        importtrans(step.offset(9 as libc::c_int as isize), dapfile);
    } else if linecmp(
        step.offset(5 as libc::c_int as isize),
        b"surveyselect\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    ) == 0
    {
        surveyselecttrans(step.offset(9 as libc::c_int as isize), dapfile);
    } else {
        s = 5 as libc::c_int;
        while *step.offset(s as isize) as libc::c_int != 0
            && *step.offset(s as isize) as libc::c_int != '\n' as i32
        {
            s += 1;
            s;
        }
        *step.offset(s as isize) = '\0' as i32 as libc::c_char;
        fprintf(
            stderr,
            b"sbstrans: before %d: unknown proc.\n\0" as *const u8
                as *const libc::c_char,
            sbslineno,
        );
        exit(1 as libc::c_int);
    };
}
pub unsafe extern "C" fn header(mut dap: *mut FILE) {
    fputs(b"#include <dap.h>\n\0" as *const u8 as *const libc::c_char, dap);
    fputs(b"void main()\n\0" as *const u8 as *const libc::c_char, dap);
    fputs(b"{\n\0" as *const u8 as *const libc::c_char, dap);
    fprintf(
        dap,
        b"pict *_sbspict_[%d];\n\0" as *const u8 as *const libc::c_char,
        256 as libc::c_int,
    );
    fprintf(
        dap,
        b"int _sbspictcnt_[%d];\n\0" as *const u8 as *const libc::c_char,
        256 as libc::c_int,
    );
    fprintf(
        dap,
        b"int _sbspictpage_[%d];\n\0" as *const u8 as *const libc::c_char,
        256 as libc::c_int,
    );
    fputs(
        b"int _sbsnpicts_ = 0, _sbspictn_, _sbspictindex_;\n\0" as *const u8
            as *const libc::c_char,
        dap,
    );
}
pub unsafe extern "C" fn trailer(mut dap: *mut FILE) {
    if sbshaspicts != 0 {
        fprintf(
            dap,
            b"pict_port(%d);\n\0" as *const u8 as *const libc::c_char,
            256 as libc::c_int,
        );
        fputs(
            b"for (_sbspictn_ = 0; _sbspictn_ < _sbsnpicts_; _sbspictn_++)\n{\n\0"
                as *const u8 as *const libc::c_char,
            dap,
        );
        fputs(
            b"for (_sbspictindex_ = 0; _sbspictindex_ < _sbspictcnt_[_sbspictn_];\0"
                as *const u8 as *const libc::c_char,
            dap,
        );
        fputs(b"_sbspictindex_++)\n{\n\0" as *const u8 as *const libc::c_char, dap);
        fputs(b"pict_page();\n\0" as *const u8 as *const libc::c_char, dap);
        fputs(
            b"pict_show(_sbspict_[_sbspictn_] + _sbspictindex_ * _sbspictpage_[_sbspictn_]);\n\0"
                as *const u8 as *const libc::c_char,
            dap,
        );
        fputs(b"}\n}\npict_end();\n\0" as *const u8 as *const libc::c_char, dap);
    }
    fputs(b"}\n\0" as *const u8 as *const libc::c_char, dap);
}
pub unsafe extern "C" fn isby(mut step: *mut libc::c_char) -> libc::c_int {
    let mut s: libc::c_int = 0;
    s = 0 as libc::c_int;
    while *step.offset(s as isize) != 0 {
        if linecmp(
            step.offset(s as isize),
            b"by\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        ) == 0
        {
            return s + 3 as libc::c_int
        } else {
            while *step.offset(s as isize) as libc::c_int != 0
                && *step.offset(s as isize) as libc::c_int != ';' as i32
            {
                s += 1;
                s;
            }
        }
        s += 2 as libc::c_int;
    }
    return -(1 as libc::c_int);
}
pub unsafe extern "C" fn getoption(
    mut step: *mut libc::c_char,
    mut key: *mut libc::c_char,
    mut optvalue: *mut libc::c_char,
    mut equals: libc::c_int,
) -> libc::c_int {
    let mut s: libc::c_int = 0;
    let mut keylen: libc::c_int = 0;
    keylen = strlen(key) as libc::c_int;
    s = 0 as libc::c_int;
    while *step.offset(s as isize) as libc::c_int != 0
        && *step.offset(s as isize) as libc::c_int != ';' as i32
    {
        if linecmp(step.offset(s as isize), key) == 0 {
            s += keylen + 1 as libc::c_int;
            if linecmp(
                step.offset(s as isize),
                b"=\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            ) == 0
            {
                s += 2 as libc::c_int;
                if !optvalue.is_null() {
                    s += linecpy(optvalue, step.offset(s as isize)) + 1 as libc::c_int;
                }
                return s;
            } else if equals != 0 {
                fprintf(
                    stderr,
                    b"sbstrans: before %d: missing = in option\n\0" as *const u8
                        as *const libc::c_char,
                    sbslineno,
                );
                exit(1 as libc::c_int);
            }
        }
        while *step.offset(s as isize) as libc::c_int != 0
            && *step.offset(s as isize) as libc::c_int != '\n' as i32
        {
            s += 1;
            s;
        }
        s += 1;
        s;
    }
    return 0 as libc::c_int;
}
pub unsafe extern "C" fn findstatement(
    mut step: *mut libc::c_char,
    mut key: *mut libc::c_char,
) -> libc::c_int {
    let mut s: libc::c_int = 0;
    let mut keylen: libc::c_int = 0;
    keylen = strlen(key) as libc::c_int;
    s = 0 as libc::c_int;
    while *step.offset(s as isize) != 0 {
        if linecmp(step.offset(s as isize), key) == 0 {
            return s + keylen + 1 as libc::c_int;
        }
        while *step.offset(s as isize) as libc::c_int != 0
            && *step.offset(s as isize) as libc::c_int != ';' as i32
        {
            s += 1;
            s;
        }
        s += 2 as libc::c_int;
    }
    return 0 as libc::c_int;
}
pub unsafe extern "C" fn countparts(
    mut step: *mut libc::c_char,
    mut setname: *mut libc::c_char,
    mut dapfile: *mut FILE,
) {
    let mut sortname: [libc::c_char; 132] = [0; 132];
    fprintf(dapfile, b"sort(\"%s\", \"\0" as *const u8 as *const libc::c_char, setname);
    strcpy(sortname.as_mut_ptr(), setname);
    strcat(sortname.as_mut_ptr(), b".srt\0" as *const u8 as *const libc::c_char);
    copylist(
        step,
        b"by\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        dapfile,
    );
    fputs(b"\", \"u\");\n\0" as *const u8 as *const libc::c_char, dapfile);
    fprintf(
        dapfile,
        b"inset(\"%s\")\n{\n\0" as *const u8 as *const libc::c_char,
        sortname.as_mut_ptr(),
    );
    fputs(
        b"for (_sbspictcnt_[_sbsnpicts_] = 0; step(); _sbspictcnt_[_sbsnpicts_]++)\n;\n}\n\0"
            as *const u8 as *const libc::c_char,
        dapfile,
    );
}
pub unsafe extern "C" fn copylist(
    mut step: *mut libc::c_char,
    mut key: *mut libc::c_char,
    mut dapfile: *mut FILE,
) -> libc::c_int {
    let mut s: libc::c_int = 0;
    let mut start: libc::c_int = 0;
    s = 0 as libc::c_int;
    start = -(1 as libc::c_int);
    while *step.offset(s as isize) != 0 {
        if linecmp(step.offset(s as isize), key) == 0 {
            s = (s as libc::c_ulong)
                .wrapping_add(
                    (strlen(key)).wrapping_add(1 as libc::c_int as libc::c_ulong),
                ) as libc::c_int as libc::c_int;
            start = s;
            putc(' ' as i32, dapfile);
            s += putlines(step.offset(s as isize), dapfile, ';' as i32);
        } else {
            while *step.offset(s as isize) as libc::c_int != 0
                && *step.offset(s as isize) as libc::c_int != ';' as i32
            {
                s += 1;
                s;
            }
        }
        s += 2 as libc::c_int;
    }
    return start;
}
pub unsafe extern "C" fn sbstrans(mut name: *mut libc::c_char) {
    let mut sbs: *mut FILE = 0 as *mut FILE;
    let mut dap: *mut FILE = 0 as *mut FILE;
    static mut step: [libc::c_char; 65536] = [0; 65536];
    let mut steptype: libc::c_int = 0;
    sbs = fopen(name, b"r\0" as *const u8 as *const libc::c_char);
    if sbs.is_null() {
        fprintf(
            stderr,
            b"sbstrans: can't read %s\n\0" as *const u8 as *const libc::c_char,
            name,
        );
        exit(1 as libc::c_int);
    }
    namecvt(name);
    dap = fopen(name, b"w\0" as *const u8 as *const libc::c_char);
    if dap.is_null() {
        fprintf(
            stderr,
            b"sbstrans: can't write %s\n\0" as *const u8 as *const libc::c_char,
            name,
        );
        exit(1 as libc::c_int);
    }
    sbstempnum = 0 as libc::c_int;
    sbslineno = 1 as libc::c_int;
    sbshaspicts = 0 as libc::c_int;
    header(dap);
    sbstmp[0 as libc::c_int as usize] = '\0' as i32 as libc::c_char;
    newline = 1 as libc::c_int;
    incomment = 0 as libc::c_int;
    inquote1 = 0 as libc::c_int;
    inquote2 = 0 as libc::c_int;
    escape = 0 as libc::c_int;
    loop {
        steptype = getstep(sbs, step.as_mut_ptr());
        if !(steptype != 0) {
            break;
        }
        match steptype {
            1 => {
                datatrans(step.as_mut_ptr(), dap);
            }
            2 => {
                proctrans(step.as_mut_ptr(), dap);
            }
            _ => {}
        }
    }
    trailer(dap);
    fclose(sbs);
    fclose(dap);
}
