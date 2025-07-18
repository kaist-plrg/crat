use ::libc;
extern "C" {
    pub type _IO_wide_data;
    pub type _IO_codecvt;
    pub type _IO_marker;
    static mut stderr: *mut FILE;
    fn fclose(__stream: *mut FILE) -> libc::c_int;
    fn fopen(_: *const libc::c_char, _: *const libc::c_char) -> *mut FILE;
    fn fprintf(_: *mut FILE, _: *const libc::c_char, _: ...) -> libc::c_int;
    fn getc(__stream: *mut FILE) -> libc::c_int;
    fn putc(__c: libc::c_int, __stream: *mut FILE) -> libc::c_int;
    fn fputs(__s: *const libc::c_char, __stream: *mut FILE) -> libc::c_int;
    fn ungetc(__c: libc::c_int, __stream: *mut FILE) -> libc::c_int;
    fn fseek(
        __stream: *mut FILE,
        __off: libc::c_long,
        __whence: libc::c_int,
    ) -> libc::c_int;
    fn ftell(__stream: *mut FILE) -> libc::c_long;
    fn perror(__s: *const libc::c_char);
    fn malloc(_: libc::c_ulong) -> *mut libc::c_void;
    fn free(__ptr: *mut libc::c_void);
    fn exit(_: libc::c_int) -> !;
    fn strcpy(_: *mut libc::c_char, _: *const libc::c_char) -> *mut libc::c_char;
    fn strcmp(_: *const libc::c_char, _: *const libc::c_char) -> libc::c_int;
    fn strlen(_: *const libc::c_char) -> libc::c_ulong;
    fn sbstrans(name: *mut libc::c_char);
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
pub struct define {
    pub def_str: [libc::c_char; 128],
    pub def_val: libc::c_int,
}
pub static mut dotname: *mut libc::c_char = 0 as *const libc::c_char
    as *mut libc::c_char;
pub static mut lineno: libc::c_int = 0;
pub static mut column: libc::c_int = 0;
pub static mut newline: libc::c_int = 0;
pub static mut pound: libc::c_int = 0;
pub static mut incomment: libc::c_int = 0;
pub static mut inquote1: libc::c_int = 0;
pub static mut inquote2: libc::c_int = 0;
pub static mut escape: libc::c_int = 0;
pub static mut def: [define; 512] = [define {
    def_str: [0; 128],
    def_val: 0,
}; 512];
pub static mut ndef: libc::c_int = 0;
pub unsafe extern "C" fn white(mut c: libc::c_int) -> libc::c_int {
    return (c == ' ' as i32 || c == '\t' as i32 || c == '\n' as i32 || c == '\r' as i32)
        as libc::c_int;
}
pub unsafe extern "C" fn alpha(mut c: libc::c_int) -> libc::c_int {
    return ('a' as i32 <= c && c <= 'z' as i32 || 'A' as i32 <= c && c <= 'Z' as i32
        || c == '_' as i32) as libc::c_int;
}
pub unsafe extern "C" fn num(mut c: libc::c_int) -> libc::c_int {
    return ('0' as i32 <= c && c <= '9' as i32) as libc::c_int;
}
pub unsafe extern "C" fn alphanum(mut c: libc::c_int) -> libc::c_int {
    return (alpha(c) != 0 || num(c) != 0) as libc::c_int;
}
pub unsafe extern "C" fn get1c(mut fp: *mut FILE) -> libc::c_int {
    let mut c: libc::c_int = 0;
    c = getc(fp);
    if newline != 0 {
        lineno += 1;
        lineno;
        column = 0 as libc::c_int;
    } else {
        column += 1;
        column;
    }
    newline = (c == '\n' as i32) as libc::c_int;
    if newline != 0 {
        pound = 0 as libc::c_int;
    }
    return c;
}
pub unsafe extern "C" fn unget1c(
    mut c: libc::c_int,
    mut dotc: *mut FILE,
    mut dapc: *mut FILE,
) {
    if newline != 0 {
        newline = 0 as libc::c_int;
    }
    ungetc(c, dotc);
    if !dapc.is_null() {
        fseek(dapc, ftell(dapc) - 1 as libc::c_int as libc::c_long, 0 as libc::c_int);
    }
}
pub unsafe extern "C" fn dgetc(
    mut dotc: *mut FILE,
    mut dapc: *mut FILE,
    mut out: libc::c_int,
) -> libc::c_int {
    let mut c: libc::c_int = 0;
    c = get1c(dotc);
    match c {
        35 => {
            if incomment == 0 && column == 0 {
                pound = 1 as libc::c_int;
            }
            escape = 0 as libc::c_int;
        }
        34 => {
            if inquote2 != 0 && escape == 0 {
                inquote2 = 0 as libc::c_int;
            } else if inquote1 == 0 && incomment == 0 {
                inquote2 = 1 as libc::c_int;
            }
            escape = 0 as libc::c_int;
        }
        39 => {
            if inquote1 != 0 && escape == 0 {
                inquote1 = 0 as libc::c_int;
            } else if inquote2 == 0 && incomment == 0 {
                inquote1 = 1 as libc::c_int;
            }
            escape = 0 as libc::c_int;
        }
        42 => {
            if incomment != 0 {
                c = get1c(dotc);
                if c == '/' as i32 {
                    if out != 0 {
                        fputs(b"*/\0" as *const u8 as *const libc::c_char, dapc);
                    }
                    c = get1c(dotc);
                    incomment = 0 as libc::c_int;
                } else {
                    unget1c(c, dotc, 0 as *mut FILE);
                    c = '*' as i32;
                }
            }
            escape = 0 as libc::c_int;
        }
        47 => {
            if incomment == 0 && inquote1 == 0 && inquote2 == 0 {
                c = get1c(dotc);
                if c == '*' as i32 {
                    if out != 0 {
                        fputs(b"/*\0" as *const u8 as *const libc::c_char, dapc);
                    }
                    c = get1c(dotc);
                    incomment = 1 as libc::c_int;
                } else {
                    unget1c(c, dotc, 0 as *mut FILE);
                    c = '/' as i32;
                }
            }
            escape = 0 as libc::c_int;
        }
        92 => {
            if incomment == 0 && (inquote1 != 0 || inquote2 != 0) {
                escape = 1 as libc::c_int;
            }
        }
        -1 => {
            if inquote1 != 0 {
                fprintf(
                    stderr,
                    b"dappp:%s:%d: unmatched '\n\0" as *const u8 as *const libc::c_char,
                    dotname,
                    lineno,
                );
                exit(1 as libc::c_int);
            }
            if inquote2 != 0 {
                fprintf(
                    stderr,
                    b"dappp:%s:%d: unmatched \"\n\0" as *const u8 as *const libc::c_char,
                    dotname,
                    lineno,
                );
                exit(1 as libc::c_int);
            }
            if incomment != 0 {
                fprintf(
                    stderr,
                    b"dappp:%s:%d: non-terminated comment\n\0" as *const u8
                        as *const libc::c_char,
                    dotname,
                    lineno,
                );
                exit(1 as libc::c_int);
            }
            if escape != 0 {
                fprintf(
                    stderr,
                    b"dappp:%s:%d: incomplete escape\n\0" as *const u8
                        as *const libc::c_char,
                    dotname,
                    lineno,
                );
                exit(1 as libc::c_int);
            }
            if pound != 0 {
                fprintf(
                    stderr,
                    b"dappp:%s:%d: incomplete #\n\0" as *const u8 as *const libc::c_char,
                    dotname,
                    lineno,
                );
                exit(1 as libc::c_int);
            }
        }
        _ => {
            escape = 0 as libc::c_int;
        }
    }
    if out != 0 && c != -(1 as libc::c_int) {
        putc(c, dapc);
    }
    return c;
}
pub unsafe extern "C" fn gettoken(
    mut token: *mut libc::c_char,
    mut dotc: *mut FILE,
    mut dapc: *mut FILE,
    mut out: libc::c_int,
) -> libc::c_int {
    let mut c: libc::c_int = 0;
    let mut t: libc::c_int = 0;
    *token.offset(0 as libc::c_int as isize) = '\0' as i32 as libc::c_char;
    loop {
        c = dgetc(dotc, dapc, out);
        if !(white(c) != 0 || incomment != 0 || inquote1 != 0 || inquote2 != 0) {
            break;
        }
    }
    t = 0 as libc::c_int;
    if alpha(c) != 0 {
        while alphanum(c) != 0 {
            if t < 127 as libc::c_int {
                let fresh0 = t;
                t = t + 1;
                *token.offset(fresh0 as isize) = c as libc::c_char;
            } else {
                *token.offset(t as isize) = '\0' as i32 as libc::c_char;
                fprintf(
                    stderr,
                    b"dappp:%s:%d: token too long: %s\n\0" as *const u8
                        as *const libc::c_char,
                    dotname,
                    lineno,
                    token,
                );
                exit(1 as libc::c_int);
            }
            c = dgetc(dotc, dapc, out);
        }
        unget1c(c, dotc, if out != 0 { dapc } else { 0 as *mut FILE });
    } else if num(c) != 0 {
        while num(c) != 0 {
            if t < 127 as libc::c_int {
                let fresh1 = t;
                t = t + 1;
                *token.offset(fresh1 as isize) = c as libc::c_char;
            } else {
                *token.offset(t as isize) = '\0' as i32 as libc::c_char;
                fprintf(
                    stderr,
                    b"dappp:%s:%d: token too long: %s\n\0" as *const u8
                        as *const libc::c_char,
                    dotname,
                    lineno,
                    token,
                );
                exit(1 as libc::c_int);
            }
            c = dgetc(dotc, dapc, out);
        }
        unget1c(c, dotc, if out != 0 { dapc } else { 0 as *mut FILE });
    } else if c != -(1 as libc::c_int) {
        let fresh2 = t;
        t = t + 1;
        *token.offset(fresh2 as isize) = c as libc::c_char;
    }
    *token.offset(t as isize) = '\0' as i32 as libc::c_char;
    return (t > 0 as libc::c_int) as libc::c_int;
}
pub unsafe extern "C" fn newdef(mut dotc: *mut FILE, mut dapc: *mut FILE) {
    static mut defstr: [libc::c_char; 128] = [0; 128];
    static mut defval_0: [libc::c_char; 128] = [0; 128];
    let mut d: libc::c_int = 0;
    let mut c: libc::c_int = 0;
    let mut v: libc::c_int = 0;
    if gettoken(defstr.as_mut_ptr(), dotc, dapc, 1 as libc::c_int) != 0 {
        if gettoken(defval_0.as_mut_ptr(), dotc, dapc, 1 as libc::c_int) != 0 {
            v = 0 as libc::c_int;
            d = 0 as libc::c_int;
            loop {
                c = defval_0[d as usize] as libc::c_int;
                if !(num(c) != 0) {
                    break;
                }
                v = 10 as libc::c_int * v + c - '0' as i32;
                d += 1;
                d;
            }
            if v > 0 as libc::c_int && c == 0 {
                if ndef < 512 as libc::c_int {
                    strcpy(
                        (def[ndef as usize].def_str).as_mut_ptr(),
                        defstr.as_mut_ptr(),
                    );
                    def[ndef as usize].def_val = v;
                    ndef += 1;
                    ndef;
                } else {
                    fprintf(
                        stderr,
                        b"dappp:%s:%d: too many #defines\n\0" as *const u8
                            as *const libc::c_char,
                        dotname,
                        lineno,
                    );
                    exit(1 as libc::c_int);
                }
            }
        }
    }
}
pub unsafe extern "C" fn defval(mut str: *mut libc::c_char) -> libc::c_int {
    let mut d: libc::c_int = 0;
    d = 0 as libc::c_int;
    while d < ndef {
        if strcmp((def[d as usize].def_str).as_mut_ptr(), str as *const libc::c_char)
            == 0
        {
            return def[d as usize].def_val;
        }
        d += 1;
        d;
    }
    return 0 as libc::c_int;
}
pub unsafe extern "C" fn declare(
    mut dotc: *mut FILE,
    mut dapc: *mut FILE,
    mut decl: *mut libc::c_char,
) {
    static mut token: [libc::c_char; 128] = [0; 128];
    static mut tokdel: [libc::c_char; 128] = [0; 128];
    static mut tokdim: [libc::c_char; 128] = [0; 128];
    let mut c: libc::c_int = 0;
    let mut dim: libc::c_int = 0;
    let mut dimd: libc::c_int = 0;
    let mut d: libc::c_int = 0;
    let mut delpos: libc::c_long = 0;
    while gettoken(token.as_mut_ptr(), dotc, dapc, 0 as libc::c_int) != 0 {
        delpos = ftell(dotc);
        if delpos < 0 as libc::c_int as libc::c_long {
            perror(b"dappp\0" as *const u8 as *const libc::c_char);
            exit(1 as libc::c_int);
        }
        if gettoken(tokdel.as_mut_ptr(), dotc, dapc, 0 as libc::c_int) != 0 {
            if strcmp(tokdel.as_mut_ptr(), b"[\0" as *const u8 as *const libc::c_char)
                == 0
            {
                if gettoken(tokdim.as_mut_ptr(), dotc, dapc, 0 as libc::c_int) == 0 {
                    fprintf(
                        stderr,
                        b"dappp:%s:%d: missing array dimension\n\0" as *const u8
                            as *const libc::c_char,
                        dotname,
                        lineno,
                    );
                    exit(1 as libc::c_int);
                }
                dim = defval(tokdim.as_mut_ptr());
                if dim == 0 {
                    dimd = 0 as libc::c_int;
                    dim = 0 as libc::c_int;
                    loop {
                        c = tokdim[dimd as usize] as libc::c_int;
                        if !(num(c) != 0) {
                            break;
                        }
                        dim = 10 as libc::c_int * dim + c - '0' as i32;
                        dimd += 1;
                        dimd;
                    }
                }
                if dim == 0 {
                    fprintf(
                        stderr,
                        b"dappp:%s:%d: zero array dimension\n\0" as *const u8
                            as *const libc::c_char,
                        dotname,
                        lineno,
                    );
                    exit(1 as libc::c_int);
                }
                if gettoken(tokdel.as_mut_ptr(), dotc, dapc, 0 as libc::c_int) == 0 {
                    fprintf(
                        stderr,
                        b"dappp:%s:%d: missing ]\n\0" as *const u8
                            as *const libc::c_char,
                        dotname,
                        lineno,
                    );
                    exit(1 as libc::c_int);
                }
                if strcmp(
                    tokdel.as_mut_ptr(),
                    b"]\0" as *const u8 as *const libc::c_char,
                ) != 0
                {
                    fprintf(
                        stderr,
                        b"dappp:%s:%d: expected ], got %s\n\0" as *const u8
                            as *const libc::c_char,
                        dotname,
                        lineno,
                        tokdel.as_mut_ptr(),
                    );
                    exit(1 as libc::c_int);
                }
                if strcmp(
                    decl as *const libc::c_char,
                    b"int\0" as *const u8 as *const libc::c_char,
                ) == 0
                {
                    d = 0 as libc::c_int;
                    while d < dim {
                        fprintf(
                            dapc,
                            b"dap_vd(\"%s[%d] 0\", 0);\0" as *const u8
                                as *const libc::c_char,
                            token.as_mut_ptr(),
                            d,
                        );
                        d += 1;
                        d;
                    }
                    fprintf(
                        dapc,
                        b"dap_il(\"%s\", %s);\0" as *const u8 as *const libc::c_char,
                        token.as_mut_ptr(),
                        token.as_mut_ptr(),
                    );
                } else if strcmp(
                    decl as *const libc::c_char,
                    b"double\0" as *const u8 as *const libc::c_char,
                ) == 0
                {
                    d = 0 as libc::c_int;
                    while d < dim {
                        fprintf(
                            dapc,
                            b"dap_vd(\"%s[%d] -1\", 0);\0" as *const u8
                                as *const libc::c_char,
                            token.as_mut_ptr(),
                            d,
                        );
                        d += 1;
                        d;
                    }
                    fprintf(
                        dapc,
                        b"dap_dl(\"%s\", %s);\0" as *const u8 as *const libc::c_char,
                        token.as_mut_ptr(),
                        token.as_mut_ptr(),
                    );
                } else {
                    if dim <= 1 as libc::c_int {
                        fprintf(
                            stderr,
                            b"dappp:%s:%d: dimension of character array must be at least 2\n\0"
                                as *const u8 as *const libc::c_char,
                            dotname,
                            lineno,
                        );
                        exit(1 as libc::c_int);
                    }
                    fprintf(
                        dapc,
                        b"dap_vd(\"%s %d\", 0);\0" as *const u8 as *const libc::c_char,
                        token.as_mut_ptr(),
                        dim - 1 as libc::c_int,
                    );
                    fprintf(
                        dapc,
                        b"dap_sl(\"%s\", %s);\0" as *const u8 as *const libc::c_char,
                        token.as_mut_ptr(),
                        token.as_mut_ptr(),
                    );
                }
                if gettoken(tokdel.as_mut_ptr(), dotc, dapc, 0 as libc::c_int) == 0 {
                    fprintf(
                        stderr,
                        b"dappp:%s:%d: missing delimiter after string declaration\n\0"
                            as *const u8 as *const libc::c_char,
                        dotname,
                        lineno,
                    );
                    exit(1 as libc::c_int);
                }
            } else if strcmp(
                tokdel.as_mut_ptr(),
                b",\0" as *const u8 as *const libc::c_char,
            ) == 0
                || strcmp(
                    tokdel.as_mut_ptr(),
                    b";\0" as *const u8 as *const libc::c_char,
                ) == 0
            {
                if strcmp(
                    decl as *const libc::c_char,
                    b"int\0" as *const u8 as *const libc::c_char,
                ) == 0
                {
                    fprintf(
                        dapc,
                        b"dap_vd(\"%s 0\", 0);\0" as *const u8 as *const libc::c_char,
                        token.as_mut_ptr(),
                    );
                    fprintf(
                        dapc,
                        b"dap_il(\"%s\", &%s);\0" as *const u8 as *const libc::c_char,
                        token.as_mut_ptr(),
                        token.as_mut_ptr(),
                    );
                } else if strcmp(
                    decl as *const libc::c_char,
                    b"double\0" as *const u8 as *const libc::c_char,
                ) == 0
                {
                    fprintf(
                        dapc,
                        b"dap_vd(\"%s -1\", 0);\0" as *const u8 as *const libc::c_char,
                        token.as_mut_ptr(),
                    );
                    fprintf(
                        dapc,
                        b"dap_dl(\"%s\", &%s);\0" as *const u8 as *const libc::c_char,
                        token.as_mut_ptr(),
                        token.as_mut_ptr(),
                    );
                } else {
                    fprintf(
                        stderr,
                        b"dappp:%s:%d: string variables must be arrays with explicit dimension\n\0"
                            as *const u8 as *const libc::c_char,
                        dotname,
                        lineno,
                    );
                    exit(1 as libc::c_int);
                }
            }
        }
        if strcmp(tokdel.as_mut_ptr(), b",\0" as *const u8 as *const libc::c_char) == 0 {
            loop {
                c = dgetc(dotc, dapc, 0 as libc::c_int);
                if !(white(c) != 0) {
                    break;
                }
            }
            unget1c(c, dotc, 0 as *mut FILE);
        } else {
            if strcmp(tokdel.as_mut_ptr(), b";\0" as *const u8 as *const libc::c_char)
                == 0
            {
                break;
            }
            fprintf(
                stderr,
                b"dappp:%s:%d: expected `;' or `,', got %s\n\0" as *const u8
                    as *const libc::c_char,
                dotname,
                lineno,
                tokdel.as_mut_ptr(),
            );
            exit(1 as libc::c_int);
        }
    }
}
pub unsafe extern "C" fn preproc(mut dotc: *mut FILE, mut dapc: *mut FILE) {
    let mut c: libc::c_int = 0;
    static mut token: [libc::c_char; 128] = [0; 128];
    static mut tokdel: [libc::c_char; 128] = [0; 128];
    let mut isdecl: libc::c_int = 0;
    let mut decpos: libc::c_long = 0;
    let mut decline: libc::c_int = 0;
    let mut inpos: libc::c_long = 0;
    let mut infline: libc::c_int = 0;
    let mut indecl: libc::c_int = 0;
    let mut brace: libc::c_int = 0;
    indecl = 0 as libc::c_int;
    inpos = ftell(dotc);
    if inpos < 0 as libc::c_int as libc::c_long {
        perror(b"dappp\0" as *const u8 as *const libc::c_char);
        exit(1 as libc::c_int);
    }
    infline = lineno;
    loop {
        decpos = ftell(dotc);
        if decpos < 0 as libc::c_int as libc::c_long {
            perror(b"dappp\0" as *const u8 as *const libc::c_char);
            exit(1 as libc::c_int);
        }
        decline = lineno;
        if gettoken(token.as_mut_ptr(), dotc, dapc, 0 as libc::c_int) != 0
            && (strcmp(
                token.as_mut_ptr(),
                b"double\0" as *const u8 as *const libc::c_char,
            ) == 0
                || strcmp(
                    token.as_mut_ptr(),
                    b"int\0" as *const u8 as *const libc::c_char,
                ) == 0
                || strcmp(
                    token.as_mut_ptr(),
                    b"char\0" as *const u8 as *const libc::c_char,
                ) == 0)
        {
            isdecl = 1 as libc::c_int;
            if fseek(dotc, decpos, 0 as libc::c_int) < 0 as libc::c_int {
                perror(b"dappp\0" as *const u8 as *const libc::c_char);
                exit(1 as libc::c_int);
            }
            lineno = decline;
            loop {
                c = dgetc(dotc, dapc, 1 as libc::c_int);
                if !(c != ';' as i32 && c != -(1 as libc::c_int)) {
                    break;
                }
            }
        } else {
            isdecl = 0 as libc::c_int;
        }
        if !(isdecl != 0) {
            break;
        }
    }
    if fseek(dotc, inpos, 0 as libc::c_int) < 0 as libc::c_int {
        perror(b"dappp\0" as *const u8 as *const libc::c_char);
        exit(1 as libc::c_int);
    }
    lineno = infline;
    loop {
        decpos = ftell(dotc);
        if decpos < 0 as libc::c_int as libc::c_long {
            perror(b"dappp\0" as *const u8 as *const libc::c_char);
            exit(1 as libc::c_int);
        }
        decline = lineno;
        if gettoken(token.as_mut_ptr(), dotc, dapc, 0 as libc::c_int) != 0
            && (strcmp(
                token.as_mut_ptr(),
                b"double\0" as *const u8 as *const libc::c_char,
            ) == 0
                || strcmp(
                    token.as_mut_ptr(),
                    b"int\0" as *const u8 as *const libc::c_char,
                ) == 0
                || strcmp(
                    token.as_mut_ptr(),
                    b"char\0" as *const u8 as *const libc::c_char,
                ) == 0)
        {
            isdecl = 1 as libc::c_int;
            declare(dotc, dapc, token.as_mut_ptr());
        } else {
            isdecl = 0 as libc::c_int;
        }
        if !(isdecl != 0) {
            break;
        }
    }
    if fseek(dotc, decpos, 0 as libc::c_int) < 0 as libc::c_int {
        perror(b"dappp\0" as *const u8 as *const libc::c_char);
        exit(1 as libc::c_int);
    }
    lineno = decline;
    brace = 1 as libc::c_int;
    while brace != 0 && gettoken(token.as_mut_ptr(), dotc, dapc, 1 as libc::c_int) != 0 {
        if strcmp(token.as_mut_ptr(), b"infile\0" as *const u8 as *const libc::c_char)
            == 0
            || strcmp(token.as_mut_ptr(), b"inset\0" as *const u8 as *const libc::c_char)
                == 0
            || strcmp(token.as_mut_ptr(), b"main\0" as *const u8 as *const libc::c_char)
                == 0
        {
            if gettoken(tokdel.as_mut_ptr(), dotc, dapc, 1 as libc::c_int) != 0 {
                if strcmp(
                    tokdel.as_mut_ptr(),
                    b"(\0" as *const u8 as *const libc::c_char,
                ) == 0
                {
                    fprintf(
                        stderr,
                        b"dappp:%s:%d: call to %s in infile or inset body\n\0"
                            as *const u8 as *const libc::c_char,
                        dotname,
                        lineno,
                        token.as_mut_ptr(),
                    );
                    exit(1 as libc::c_int);
                }
            }
        } else if strcmp(token.as_mut_ptr(), b"{\0" as *const u8 as *const libc::c_char)
            == 0
        {
            brace += 1;
            brace;
        } else if strcmp(token.as_mut_ptr(), b"}\0" as *const u8 as *const libc::c_char)
            == 0
        {
            brace -= 1;
            brace;
        }
    }
}
pub unsafe extern "C" fn infile(
    mut dotc: *mut FILE,
    mut dapc: *mut FILE,
) -> libc::c_int {
    let mut token: [libc::c_char; 128] = [0; 128];
    let mut tokdel: [libc::c_char; 128] = [0; 128];
    let mut c: libc::c_int = 0;
    let mut dotcpos: libc::c_long = 0;
    let mut dapcpos: libc::c_long = 0;
    let mut paren: libc::c_int = 0;
    while gettoken(token.as_mut_ptr(), dotc, dapc, 1 as libc::c_int) != 0 {
        if pound != 0 {
            if column > 1 as libc::c_int {
                if strcmp(
                    token.as_mut_ptr(),
                    b"define\0" as *const u8 as *const libc::c_char,
                ) == 0
                {
                    newdef(dotc, dapc);
                }
                pound = 0 as libc::c_int;
            }
        } else if strcmp(
            token.as_mut_ptr(),
            b"main\0" as *const u8 as *const libc::c_char,
        ) == 0
        {
            dotcpos = ftell(dotc) - 4 as libc::c_int as libc::c_long;
            dapcpos = ftell(dapc) - 4 as libc::c_int as libc::c_long;
            if gettoken(tokdel.as_mut_ptr(), dotc, dapc, 1 as libc::c_int) != 0 {
                if strcmp(
                    tokdel.as_mut_ptr(),
                    b"(\0" as *const u8 as *const libc::c_char,
                ) == 0
                {
                    if fseek(dotc, dotcpos, 0 as libc::c_int) < 0 as libc::c_int {
                        perror(b"dappp\0" as *const u8 as *const libc::c_char);
                        exit(1 as libc::c_int);
                    }
                    if fseek(dapc, dapcpos, 0 as libc::c_int) < 0 as libc::c_int {
                        perror(b"dappp\0" as *const u8 as *const libc::c_char);
                        exit(1 as libc::c_int);
                    }
                    fputs(b"dap_\0" as *const u8 as *const libc::c_char, dapc);
                    gettoken(token.as_mut_ptr(), dotc, dapc, 1 as libc::c_int);
                    gettoken(tokdel.as_mut_ptr(), dotc, dapc, 1 as libc::c_int);
                }
            }
        } else if strcmp(
            token.as_mut_ptr(),
            b"infile\0" as *const u8 as *const libc::c_char,
        ) == 0
            || strcmp(token.as_mut_ptr(), b"inset\0" as *const u8 as *const libc::c_char)
                == 0
        {
            if gettoken(tokdel.as_mut_ptr(), dotc, dapc, 1 as libc::c_int) != 0
                && strcmp(
                    tokdel.as_mut_ptr(),
                    b"(\0" as *const u8 as *const libc::c_char,
                ) == 0
            {
                paren = 1 as libc::c_int;
                while paren != 0
                    && {
                        c = dgetc(dotc, dapc, 1 as libc::c_int);
                        c != -(1 as libc::c_int)
                    }
                {
                    if incomment == 0 && inquote1 == 0 && inquote2 == 0 {
                        if c == '(' as i32 {
                            paren += 1;
                            paren;
                        } else if c == ')' as i32 {
                            paren -= 1;
                            paren;
                        }
                    }
                }
                putc(';' as i32, dapc);
                loop {
                    c = dgetc(dotc, dapc, 1 as libc::c_int);
                    if !(white(c) != 0 || incomment != 0) {
                        break;
                    }
                }
                if c == '{' as i32 {
                    loop {
                        c = dgetc(dotc, dapc, 1 as libc::c_int);
                        if !(white(c) != 0 || incomment != 0) {
                            break;
                        }
                    }
                    unget1c(c, dotc, dapc);
                } else {
                    fprintf(
                        stderr,
                        b"dappp:%s:%d: expected {, got %c\n\0" as *const u8
                            as *const libc::c_char,
                        dotname,
                        lineno,
                        c,
                    );
                    exit(1 as libc::c_int);
                }
                return 1 as libc::c_int;
            } else {
                fprintf(
                    stderr,
                    b"dappp:%s:%d: expected (, got %s\n\0" as *const u8
                        as *const libc::c_char,
                    dotname,
                    lineno,
                    tokdel.as_mut_ptr(),
                );
                exit(1 as libc::c_int);
            }
        }
    }
    return 0 as libc::c_int;
}
unsafe fn main_0(
    mut argc: libc::c_int,
    mut argv: *mut *mut libc::c_char,
) -> libc::c_int {
    let mut dotc: *mut FILE = 0 as *mut FILE;
    let mut dapc: *mut FILE = 0 as *mut FILE;
    let mut namelen: libc::c_int = 0;
    let mut dapname: *mut libc::c_char = 0 as *mut libc::c_char;
    loop {
        argc -= 1;
        if !(argc > 0 as libc::c_int) {
            break;
        }
        argv = argv.offset(1);
        argv;
        namelen = strlen(*argv) as libc::c_int;
        if namelen > 4 as libc::c_int
            && strcmp(
                (*argv).offset(namelen as isize).offset(-(4 as libc::c_int as isize)),
                b".sbs\0" as *const u8 as *const libc::c_char,
            ) == 0
        {
            sbstrans(*argv);
            namelen -= 2 as libc::c_int;
        }
        if namelen <= 2 as libc::c_int
            || strcmp(
                (*argv).offset(namelen as isize).offset(-(2 as libc::c_int as isize)),
                b".c\0" as *const u8 as *const libc::c_char,
            ) != 0
        {
            fprintf(
                stderr,
                b"dappp: file name does not end in .c: %s\n\0" as *const u8
                    as *const libc::c_char,
                *argv,
            );
            exit(1 as libc::c_int);
        }
        dotname = malloc((namelen + 1 as libc::c_int) as libc::c_ulong)
            as *mut libc::c_char;
        if dotname.is_null() {
            perror(b"dappp\0" as *const u8 as *const libc::c_char);
            exit(1 as libc::c_int);
        }
        dapname = malloc((namelen + 5 as libc::c_int) as libc::c_ulong)
            as *mut libc::c_char;
        if dapname.is_null() {
            perror(b"dappp\0" as *const u8 as *const libc::c_char);
            exit(1 as libc::c_int);
        }
        strcpy(dotname, *argv);
        strcpy(dapname, *argv);
        strcpy(
            dapname.offset(namelen as isize).offset(-(1 as libc::c_int as isize)),
            b"dap.c\0" as *const u8 as *const libc::c_char,
        );
        dotc = fopen(dotname, b"r\0" as *const u8 as *const libc::c_char);
        if dotc.is_null() {
            fputs(b"dappp:\0" as *const u8 as *const libc::c_char, stderr);
            perror(dotname);
            exit(1 as libc::c_int);
        }
        dapc = fopen(dapname, b"w\0" as *const u8 as *const libc::c_char);
        if dapc.is_null() {
            fputs(b"dappp:\0" as *const u8 as *const libc::c_char, stderr);
            perror(dapname);
            exit(1 as libc::c_int);
        }
        lineno = 0 as libc::c_int;
        column = 0 as libc::c_int;
        newline = 1 as libc::c_int;
        incomment = 0 as libc::c_int;
        inquote1 = 0 as libc::c_int;
        inquote2 = 0 as libc::c_int;
        escape = 0 as libc::c_int;
        ndef = 0 as libc::c_int;
        while infile(dotc, dapc) != 0 {
            preproc(dotc, dapc);
        }
        fclose(dotc);
        fclose(dapc);
        free(dotname as *mut libc::c_void);
        free(dapname as *mut libc::c_void);
    }
    return 0 as libc::c_int;
}
pub fn main() {
    let mut args: Vec::<*mut libc::c_char> = Vec::new();
    for arg in ::std::env::args_os() {
        args.push(
            (::std::ffi::CString::new(
                ::std::os::unix::ffi::OsStrExt::as_bytes(arg.as_os_str()),
            ))
                .unwrap()
                .into_raw(),
        );
    }
    args.push(::std::ptr::null_mut());
    unsafe {
        ::std::process::exit(
            main_0(
                (args.len() - 1) as libc::c_int,
                args.as_mut_ptr() as *mut *mut libc::c_char,
            ) as i32,
        )
    }
}
