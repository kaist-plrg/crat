use ::libc;
extern "C" {
    fn free(__ptr: *mut libc::c_void);
    fn strncpy(
        _: *mut libc::c_char,
        _: *const libc::c_char,
        _: libc::c_ulong,
    ) -> *mut libc::c_char;
    fn strcmp(_: *const libc::c_char, _: *const libc::c_char) -> libc::c_int;
    fn strlen(_: *const libc::c_char) -> libc::c_ulong;
    fn save(s: *const libc::c_char) -> *mut libc::c_char;
    fn ch_length() -> POSITION;
    fn ch_getflags() -> libc::c_int;
    fn linenumtoa(_: LINENUM, _: *mut libc::c_char, _: libc::c_int);
    fn inttoa(_: libc::c_int, _: *mut libc::c_char, _: libc::c_int);
    fn postoa(_: POSITION, _: *mut libc::c_char, _: libc::c_int);
    fn shell_quote(s: *mut libc::c_char) -> *mut libc::c_char;
    fn last_component(name: *mut libc::c_char) -> *mut libc::c_char;
    fn eof_displayed() -> libc::c_int;
    fn next_ifile(h: *mut libc::c_void) -> *mut libc::c_void;
    fn nifile() -> libc::c_int;
    fn get_filename(ifile: *mut libc::c_void) -> *mut libc::c_char;
    fn get_index(ifile: *mut libc::c_void) -> libc::c_int;
    fn find_linenum(pos: POSITION) -> LINENUM;
    fn currline(where_0: libc::c_int) -> LINENUM;
    fn vlinenum(linenum: LINENUM) -> LINENUM;
    fn percentage(num: POSITION, den: POSITION) -> libc::c_int;
    fn position(sindex: libc::c_int) -> POSITION;
    fn sindex_from_sline(sline: libc::c_int) -> libc::c_int;
    fn ntags() -> libc::c_int;
    fn curr_tag() -> libc::c_int;
    static mut pr_type: libc::c_int;
    static mut new_file: libc::c_int;
    static mut linenums: libc::c_int;
    static mut hshift: libc::c_int;
    static mut sc_height: libc::c_int;
    static mut jump_sline: libc::c_int;
    static mut less_is_more: libc::c_int;
    static mut header_lines: libc::c_int;
    static mut curr_ifile: *mut libc::c_void;
    static mut editor: *mut libc::c_char;
    static mut editproto: *mut libc::c_char;
}
pub type __off_t = libc::c_long;
pub type off_t = __off_t;
pub type POSITION = off_t;
pub type LINENUM = off_t;
static mut s_proto: [libc::c_char; 52] = unsafe {
    *::std::mem::transmute::<
        &[u8; 52],
        &[libc::c_char; 52],
    >(b"?n?f%f .?m(%T %i of %m) ..?e(END) ?x- Next\\: %x..%t\0")
};
static mut m_proto: [libc::c_char; 77] = unsafe {
    *::std::mem::transmute::<
        &[u8; 77],
        &[libc::c_char; 77],
    >(
        b"?n?f%f .?m(%T %i of %m) ..?e(END) ?x- Next\\: %x.:?pB%pB\\%:byte %bB?s/%s...%t\0",
    )
};
static mut M_proto: [libc::c_char; 102] = unsafe {
    *::std::mem::transmute::<
        &[u8; 102],
        &[libc::c_char; 102],
    >(
        b"?f%f .?n?m(%T %i of %m) ..?ltlines %lt-%lb?L/%L. :byte %bB?s/%s. .?e(END) ?x- Next\\: %x.:?pB%pB\\%..%t\0",
    )
};
static mut e_proto: [libc::c_char; 84] = unsafe {
    *::std::mem::transmute::<
        &[u8; 84],
        &[libc::c_char; 84],
    >(
        b"?f%f .?m(%T %i of %m) .?ltlines %lt-%lb?L/%L. .byte %bB?s/%s. ?e(END) :?pB%pB\\%..%t\0",
    )
};
static mut h_proto: [libc::c_char; 80] = unsafe {
    *::std::mem::transmute::<
        &[u8; 80],
        &[libc::c_char; 80],
    >(
        b"HELP -- ?eEND -- Press g to see it again:Press RETURN for more., or q when done\0",
    )
};
static mut w_proto: [libc::c_char; 17] = unsafe {
    *::std::mem::transmute::<&[u8; 17], &[libc::c_char; 17]>(b"Waiting for data\0")
};
static mut more_proto: [libc::c_char; 59] = unsafe {
    *::std::mem::transmute::<
        &[u8; 59],
        &[libc::c_char; 59],
    >(b"--More--(?eEND ?x- Next\\: %x.:?pB%pB\\%:byte %bB?s/%s...%t)\0")
};
pub static mut prproto: [*mut libc::c_char; 3] = [0 as *const libc::c_char
    as *mut libc::c_char; 3];
pub static mut eqproto: *mut libc::c_char = unsafe {
    e_proto.as_ptr() as *mut libc::c_char
};
pub static mut hproto: *mut libc::c_char = unsafe {
    h_proto.as_ptr() as *mut libc::c_char
};
pub static mut wproto: *mut libc::c_char = unsafe {
    w_proto.as_ptr() as *mut libc::c_char
};
static mut message: [libc::c_char; 2048] = [0; 2048];
static mut mp: *mut libc::c_char = 0 as *const libc::c_char as *mut libc::c_char;
pub unsafe extern "C" fn init_prompt() {
    prproto[0 as libc::c_int as usize] = save(s_proto.as_ptr());
    prproto[1 as libc::c_int
        as usize] = save(
        if less_is_more != 0 { more_proto.as_ptr() } else { m_proto.as_ptr() },
    );
    prproto[2 as libc::c_int as usize] = save(M_proto.as_ptr());
    eqproto = save(e_proto.as_ptr());
    hproto = save(h_proto.as_ptr());
    wproto = save(w_proto.as_ptr());
}
unsafe extern "C" fn ap_str(mut s: *mut libc::c_char) {
    let mut len: libc::c_int = 0;
    len = strlen(s) as libc::c_int;
    if mp.offset(len as isize)
        >= message.as_mut_ptr().offset(2048 as libc::c_int as isize)
    {
        len = (message.as_mut_ptr().offset(2048 as libc::c_int as isize).offset_from(mp)
            as libc::c_long - 1 as libc::c_int as libc::c_long) as libc::c_int;
    }
    strncpy(mp, s, len as libc::c_ulong);
    mp = mp.offset(len as isize);
    *mp = '\0' as i32 as libc::c_char;
}
unsafe extern "C" fn ap_char(mut c: libc::c_char) {
    let mut buf: [libc::c_char; 2] = [0; 2];
    buf[0 as libc::c_int as usize] = c;
    buf[1 as libc::c_int as usize] = '\0' as i32 as libc::c_char;
    ap_str(buf.as_mut_ptr());
}
unsafe extern "C" fn ap_pos(mut pos: POSITION) {
    let mut buf: [libc::c_char; 23] = [0; 23];
    postoa(pos, buf.as_mut_ptr(), 10 as libc::c_int);
    ap_str(buf.as_mut_ptr());
}
unsafe extern "C" fn ap_linenum(mut linenum: LINENUM) {
    let mut buf: [libc::c_char; 23] = [0; 23];
    linenumtoa(linenum, buf.as_mut_ptr(), 10 as libc::c_int);
    ap_str(buf.as_mut_ptr());
}
unsafe extern "C" fn ap_int(mut num: libc::c_int) {
    let mut buf: [libc::c_char; 13] = [0; 13];
    inttoa(num, buf.as_mut_ptr(), 10 as libc::c_int);
    ap_str(buf.as_mut_ptr());
}
unsafe extern "C" fn ap_quest() {
    ap_str(b"?\0" as *const u8 as *const libc::c_char as *mut libc::c_char);
}
unsafe extern "C" fn curr_byte(mut where_0: libc::c_int) -> POSITION {
    let mut pos: POSITION = 0;
    pos = position(where_0);
    while pos == -(1 as libc::c_int) as POSITION && where_0 >= 0 as libc::c_int
        && where_0 < sc_height - 1 as libc::c_int
    {
        where_0 += 1;
        pos = position(where_0);
    }
    if pos == -(1 as libc::c_int) as POSITION {
        pos = ch_length();
    }
    return pos;
}
unsafe extern "C" fn cond(mut c: libc::c_char, mut where_0: libc::c_int) -> libc::c_int {
    let mut len: POSITION = 0;
    match c as libc::c_int {
        97 => return (mp > message.as_mut_ptr()) as libc::c_int,
        98 => {
            return (curr_byte(where_0) != -(1 as libc::c_int) as POSITION) as libc::c_int;
        }
        99 => return (hshift != 0 as libc::c_int) as libc::c_int,
        101 => return eof_displayed(),
        102 | 103 => {
            return (strcmp(
                get_filename(curr_ifile),
                b"-\0" as *const u8 as *const libc::c_char,
            ) != 0 as libc::c_int) as libc::c_int;
        }
        108 | 100 => {
            if linenums == 0 {
                return 0 as libc::c_int;
            }
            return (currline(where_0) != 0 as libc::c_int as libc::c_long)
                as libc::c_int;
        }
        76 | 68 => {
            return (linenums != 0 && ch_length() != -(1 as libc::c_int) as POSITION)
                as libc::c_int;
        }
        109 => {
            return if ntags() != 0 {
                (ntags() > 1 as libc::c_int) as libc::c_int
            } else {
                (nifile() > 1 as libc::c_int) as libc::c_int
            };
        }
        110 => return if ntags() != 0 { 1 as libc::c_int } else { new_file },
        112 => {
            return (curr_byte(where_0) != -(1 as libc::c_int) as POSITION
                && ch_length() > 0 as libc::c_int as libc::c_long) as libc::c_int;
        }
        80 => {
            return (currline(where_0) != 0 as libc::c_int as libc::c_long
                && {
                    len = ch_length();
                    len > 0 as libc::c_int as libc::c_long
                } && find_linenum(len) != 0 as libc::c_int as libc::c_long)
                as libc::c_int;
        }
        115 | 66 => {
            return (ch_length() != -(1 as libc::c_int) as POSITION) as libc::c_int;
        }
        120 => {
            if ntags() != 0 {
                return 0 as libc::c_int;
            }
            return (next_ifile(curr_ifile) != 0 as *mut libc::c_void) as libc::c_int;
        }
        _ => {}
    }
    return 0 as libc::c_int;
}
unsafe extern "C" fn protochar(
    mut c: libc::c_int,
    mut where_0: libc::c_int,
    mut iseditproto: libc::c_int,
) {
    let mut pos: POSITION = 0;
    let mut len: POSITION = 0;
    let mut n: libc::c_int = 0;
    let mut linenum: LINENUM = 0;
    let mut last_linenum: LINENUM = 0;
    let mut h: *mut libc::c_void = 0 as *mut libc::c_void;
    let mut s: *mut libc::c_char = 0 as *mut libc::c_char;
    match c {
        98 => {
            pos = curr_byte(where_0);
            if pos != -(1 as libc::c_int) as POSITION {
                ap_pos(pos);
            } else {
                ap_quest();
            }
        }
        99 => {
            ap_int(hshift);
        }
        100 => {
            linenum = currline(where_0);
            if linenum > 0 as libc::c_int as libc::c_long
                && sc_height > header_lines + 1 as libc::c_int
            {
                ap_linenum(
                    (linenum - 1 as libc::c_int as libc::c_long)
                        / (sc_height - header_lines - 1 as libc::c_int) as libc::c_long
                        + 1 as libc::c_int as libc::c_long,
                );
            } else {
                ap_quest();
            }
        }
        68 => {
            len = ch_length();
            if len == -(1 as libc::c_int) as POSITION {
                ap_quest();
            } else if len == 0 as libc::c_int as libc::c_long {
                ap_linenum(0 as libc::c_int as LINENUM);
            } else {
                linenum = find_linenum(len - 1 as libc::c_int as libc::c_long);
                if linenum <= 0 as libc::c_int as libc::c_long {
                    ap_quest();
                } else {
                    ap_linenum(
                        (linenum - 1 as libc::c_int as libc::c_long)
                            / (sc_height - header_lines - 1 as libc::c_int)
                                as libc::c_long + 1 as libc::c_int as libc::c_long,
                    );
                }
            }
        }
        69 => {
            ap_str(editor);
        }
        102 => {
            ap_str(get_filename(curr_ifile));
        }
        70 => {
            ap_str(last_component(get_filename(curr_ifile)));
        }
        103 => {
            s = shell_quote(get_filename(curr_ifile));
            ap_str(s);
            free(s as *mut libc::c_void);
        }
        105 => {
            if ntags() != 0 {
                ap_int(curr_tag());
            } else {
                ap_int(get_index(curr_ifile));
            }
        }
        108 => {
            linenum = currline(where_0);
            if linenum != 0 as libc::c_int as libc::c_long {
                ap_linenum(vlinenum(linenum));
            } else {
                ap_quest();
            }
        }
        76 => {
            len = ch_length();
            if len == -(1 as libc::c_int) as POSITION
                || len == 0 as libc::c_int as POSITION
                || {
                    linenum = find_linenum(len);
                    linenum <= 0 as libc::c_int as libc::c_long
                }
            {
                ap_quest();
            } else {
                ap_linenum(vlinenum(linenum - 1 as libc::c_int as libc::c_long));
            }
        }
        109 => {
            n = ntags();
            if n != 0 {
                ap_int(n);
            } else {
                ap_int(nifile());
            }
        }
        112 => {
            pos = curr_byte(where_0);
            len = ch_length();
            if pos != -(1 as libc::c_int) as POSITION
                && len > 0 as libc::c_int as libc::c_long
            {
                ap_int(percentage(pos, len));
            } else {
                ap_quest();
            }
        }
        80 => {
            linenum = currline(where_0);
            if linenum == 0 as libc::c_int as libc::c_long
                || {
                    len = ch_length();
                    len == -(1 as libc::c_int) as POSITION
                } || len == 0 as libc::c_int as POSITION
                || {
                    last_linenum = find_linenum(len);
                    last_linenum <= 0 as libc::c_int as libc::c_long
                }
            {
                ap_quest();
            } else {
                ap_int(percentage(linenum, last_linenum));
            }
        }
        115 | 66 => {
            len = ch_length();
            if len != -(1 as libc::c_int) as POSITION {
                ap_pos(len);
            } else {
                ap_quest();
            }
        }
        116 => {
            while mp > message.as_mut_ptr()
                && *mp.offset(-(1 as libc::c_int) as isize) as libc::c_int == ' ' as i32
            {
                mp = mp.offset(-1);
                mp;
            }
            *mp = '\0' as i32 as libc::c_char;
        }
        84 => {
            if ntags() != 0 {
                ap_str(
                    b"tag\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
                );
            } else {
                ap_str(
                    b"file\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
                );
            }
        }
        120 => {
            h = next_ifile(curr_ifile);
            if h != 0 as *mut libc::c_void {
                ap_str(get_filename(h));
            } else {
                ap_quest();
            }
        }
        _ => {}
    };
}
unsafe extern "C" fn skipcond(mut p: *const libc::c_char) -> *const libc::c_char {
    let mut iflevel: libc::c_int = 0;
    iflevel = 1 as libc::c_int;
    loop {
        p = p.offset(1);
        match *p as libc::c_int {
            63 => {
                iflevel += 1;
                iflevel;
            }
            58 => {
                if iflevel == 1 as libc::c_int {
                    return p;
                }
            }
            46 => {
                iflevel -= 1;
                if iflevel == 0 as libc::c_int {
                    return p;
                }
            }
            92 => {
                if *p.offset(1 as libc::c_int as isize) as libc::c_int != '\0' as i32 {
                    p = p.offset(1);
                    p;
                }
            }
            0 => return p.offset(-(1 as libc::c_int as isize)),
            _ => {}
        }
    };
}
unsafe extern "C" fn wherechar(
    mut p: *const libc::c_char,
    mut wp: *mut libc::c_int,
) -> *const libc::c_char {
    match *p as libc::c_int {
        98 | 100 | 108 | 112 | 80 => {
            p = p.offset(1);
            match *p as libc::c_int {
                116 => {
                    *wp = 0 as libc::c_int;
                }
                109 => {
                    *wp = -(3 as libc::c_int);
                }
                98 => {
                    *wp = -(1 as libc::c_int);
                }
                66 => {
                    *wp = -(2 as libc::c_int);
                }
                106 => {
                    *wp = sindex_from_sline(jump_sline);
                }
                _ => {
                    *wp = 0 as libc::c_int;
                    p = p.offset(-1);
                    p;
                }
            }
        }
        _ => {}
    }
    return p;
}
pub unsafe extern "C" fn pr_expand(mut proto: *const libc::c_char) -> *mut libc::c_char {
    let mut p: *const libc::c_char = 0 as *const libc::c_char;
    let mut c: libc::c_int = 0;
    let mut where_0: libc::c_int = 0;
    mp = message.as_mut_ptr();
    if *proto as libc::c_int == '\0' as i32 {
        return b"\0" as *const u8 as *const libc::c_char as *mut libc::c_char;
    }
    p = proto;
    while *p as libc::c_int != '\0' as i32 {
        match *p as libc::c_int {
            92 => {
                if *p.offset(1 as libc::c_int as isize) as libc::c_int != '\0' as i32 {
                    p = p.offset(1);
                    ap_char(*p);
                }
            }
            63 => {
                p = p.offset(1);
                c = *p as libc::c_int;
                if c == '\0' as i32 {
                    p = p.offset(-1);
                    p;
                } else {
                    where_0 = 0 as libc::c_int;
                    p = wherechar(p, &mut where_0);
                    if cond(c as libc::c_char, where_0) == 0 {
                        p = skipcond(p);
                    }
                }
            }
            58 => {
                p = skipcond(p);
            }
            46 => {}
            37 => {
                p = p.offset(1);
                c = *p as libc::c_int;
                if c == '\0' as i32 {
                    p = p.offset(-1);
                    p;
                } else {
                    where_0 = 0 as libc::c_int;
                    p = wherechar(p, &mut where_0);
                    protochar(
                        c,
                        where_0,
                        (proto == editproto as *const libc::c_char) as libc::c_int,
                    );
                }
            }
            _ => {
                ap_char(*p);
            }
        }
        p = p.offset(1);
        p;
    }
    if mp == message.as_mut_ptr() {
        return b"\0" as *const u8 as *const libc::c_char as *mut libc::c_char;
    }
    return message.as_mut_ptr();
}
pub unsafe extern "C" fn eq_message() -> *mut libc::c_char {
    return pr_expand(eqproto);
}
pub unsafe extern "C" fn pr_string() -> *mut libc::c_char {
    let mut prompt: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut type_0: libc::c_int = 0;
    type_0 = if less_is_more == 0 {
        pr_type
    } else if pr_type != 0 {
        0 as libc::c_int
    } else {
        1 as libc::c_int
    };
    prompt = pr_expand(
        if ch_getflags() & 0o10 as libc::c_int != 0 {
            hproto
        } else {
            prproto[type_0 as usize]
        },
    );
    new_file = 0 as libc::c_int;
    return prompt;
}
pub unsafe extern "C" fn wait_message() -> *mut libc::c_char {
    return pr_expand(wproto);
}
