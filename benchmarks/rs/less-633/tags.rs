use ::libc;
extern "C" {
    pub type _IO_wide_data;
    pub type _IO_codecvt;
    pub type _IO_marker;
    fn strtol(
        _: *const libc::c_char,
        _: *mut *mut libc::c_char,
        _: libc::c_int,
    ) -> libc::c_long;
    static mut stdin: *mut FILE;
    fn fclose(__stream: *mut FILE) -> libc::c_int;
    fn fopen(_: *const libc::c_char, _: *const libc::c_char) -> *mut FILE;
    fn sprintf(_: *mut libc::c_char, _: *const libc::c_char, _: ...) -> libc::c_int;
    fn fgetc(__stream: *mut FILE) -> libc::c_int;
    fn fgets(
        __s: *mut libc::c_char,
        __n: libc::c_int,
        __stream: *mut FILE,
    ) -> *mut libc::c_char;
    fn popen(__command: *const libc::c_char, __modes: *const libc::c_char) -> *mut FILE;
    fn pclose(__stream: *mut FILE) -> libc::c_int;
    fn open(__file: *const libc::c_char, __oflag: libc::c_int, _: ...) -> libc::c_int;
    fn close(__fd: libc::c_int) -> libc::c_int;
    fn __ctype_b_loc() -> *mut *const libc::c_ushort;
    fn free(__ptr: *mut libc::c_void);
    fn strcpy(_: *mut libc::c_char, _: *const libc::c_char) -> *mut libc::c_char;
    fn strcmp(_: *const libc::c_char, _: *const libc::c_char) -> libc::c_int;
    fn strncmp(
        _: *const libc::c_char,
        _: *const libc::c_char,
        _: libc::c_ulong,
    ) -> libc::c_int;
    fn strlen(_: *const libc::c_char) -> libc::c_ulong;
    fn ecalloc(count: libc::c_int, size: libc::c_uint) -> *mut libc::c_void;
    fn skipsp(s: *mut libc::c_char) -> *mut libc::c_char;
    fn cvt_length(len: libc::c_int, ops: libc::c_int) -> libc::c_int;
    fn cvt_alloc_chpos(len: libc::c_int) -> *mut libc::c_int;
    fn cvt_text(
        odst: *mut libc::c_char,
        osrc: *mut libc::c_char,
        chpos: *mut libc::c_int,
        lenp: *mut libc::c_int,
        ops: libc::c_int,
    );
    fn lgetenv(var: *mut libc::c_char) -> *mut libc::c_char;
    fn isnullenv(s: *mut libc::c_char) -> libc::c_int;
    fn edit(filename: *mut libc::c_char) -> libc::c_int;
    fn shell_unquote(str: *mut libc::c_char) -> *mut libc::c_char;
    fn shell_quote(s: *mut libc::c_char) -> *mut libc::c_char;
    fn forw_raw_line(
        curr_pos: POSITION,
        linep: *mut *mut libc::c_char,
        line_lenp: *mut libc::c_int,
    ) -> POSITION;
    fn add_lnum(linenum: LINENUM, pos: POSITION);
    fn find_linenum(pos: POSITION) -> LINENUM;
    fn find_pos(linenum: LINENUM) -> POSITION;
    fn getnum(
        sp: *mut *mut libc::c_char,
        printopt: *mut libc::c_char,
        errp: *mut libc::c_int,
    ) -> libc::c_int;
    fn error(fmt: *mut libc::c_char, parg: *mut PARG);
    static mut linenums: libc::c_int;
    static mut sigs: libc::c_int;
    static mut ctldisp: libc::c_int;
}
pub type __off_t = libc::c_long;
pub type __off64_t = libc::c_long;
pub type off_t = __off_t;
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
pub type POSITION = off_t;
pub type LINENUM = off_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub union parg {
    pub p_string: *mut libc::c_char,
    pub p_int: libc::c_int,
    pub p_linenum: LINENUM,
    pub p_char: libc::c_char,
}
pub type PARG = parg;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct tag {
    pub next: *mut tag,
    pub prev: *mut tag,
    pub tag_file: *mut libc::c_char,
    pub tag_linenum: LINENUM,
    pub tag_pattern: *mut libc::c_char,
    pub tag_endline: libc::c_char,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct taglist {
    pub tl_first: *mut tag,
    pub tl_last: *mut tag,
}
pub const T_GTAGS: C2RustUnnamed_0 = 2;
pub const T_CTAGS: C2RustUnnamed_0 = 0;
pub const T_CTAGS_X: C2RustUnnamed_0 = 1;
pub const T_GPATH: C2RustUnnamed_0 = 5;
pub const T_GSYMS: C2RustUnnamed_0 = 4;
pub const T_GRTAGS: C2RustUnnamed_0 = 3;
pub const TAG_NOTYPE: tag_result = 3;
pub const TAG_NOTAG: tag_result = 2;
pub const TAG_NOFILE: tag_result = 1;
pub const TAG_INTR: tag_result = 4;
pub const TAG_FOUND: tag_result = 0;
pub type tag_result = libc::c_uint;
pub type C2RustUnnamed_0 = libc::c_uint;
#[inline]
unsafe extern "C" fn atoi(mut __nptr: *const libc::c_char) -> libc::c_int {
    return strtol(
        __nptr,
        0 as *mut libc::c_void as *mut *mut libc::c_char,
        10 as libc::c_int,
    ) as libc::c_int;
}
pub static mut ztags: [libc::c_char; 5] = unsafe {
    *::std::mem::transmute::<&[u8; 5], &mut [libc::c_char; 5]>(b"tags\0")
};
pub static mut tags: *mut libc::c_char = unsafe { ztags.as_ptr() as *mut _ };
static mut total: libc::c_int = 0;
static mut curseq: libc::c_int = 0;
static mut taglist: taglist = unsafe {
    {
        let mut init = taglist {
            tl_first: &taglist as *const taglist as *mut taglist as *mut tag,
            tl_last: &taglist as *const taglist as *mut taglist as *mut tag,
        };
        init
    }
};
static mut curtag: *mut tag = 0 as *const tag as *mut tag;
pub unsafe extern "C" fn cleantags() {
    let mut tp: *mut tag = 0 as *mut tag;
    loop {
        tp = taglist.tl_first;
        if !(tp != &mut taglist as *mut taglist as *mut tag) {
            break;
        }
        (*(*tp).next).prev = (*tp).prev;
        (*(*tp).prev).next = (*tp).next;
        free((*tp).tag_file as *mut libc::c_void);
        free((*tp).tag_pattern as *mut libc::c_void);
        free(tp as *mut libc::c_void);
    }
    curtag = 0 as *mut tag;
    curseq = 0 as libc::c_int;
    total = curseq;
}
unsafe extern "C" fn maketagent(
    mut name: *mut libc::c_char,
    mut file: *mut libc::c_char,
    mut linenum: LINENUM,
    mut pattern: *mut libc::c_char,
    mut endline: libc::c_int,
) -> *mut tag {
    let mut tp: *mut tag = 0 as *mut tag;
    tp = ecalloc(
        ::std::mem::size_of::<tag>() as libc::c_ulong as libc::c_int,
        1 as libc::c_int as libc::c_uint,
    ) as *mut tag;
    (*tp)
        .tag_file = ecalloc(
        (strlen(file)).wrapping_add(1 as libc::c_int as libc::c_ulong) as libc::c_int,
        ::std::mem::size_of::<libc::c_char>() as libc::c_ulong as libc::c_uint,
    ) as *mut libc::c_char;
    strcpy((*tp).tag_file, file);
    (*tp).tag_linenum = linenum;
    (*tp).tag_endline = endline as libc::c_char;
    if pattern.is_null() {
        (*tp).tag_pattern = 0 as *mut libc::c_char;
    } else {
        (*tp)
            .tag_pattern = ecalloc(
            (strlen(pattern)).wrapping_add(1 as libc::c_int as libc::c_ulong)
                as libc::c_int,
            ::std::mem::size_of::<libc::c_char>() as libc::c_ulong as libc::c_uint,
        ) as *mut libc::c_char;
        strcpy((*tp).tag_pattern, pattern);
    }
    return tp;
}
pub unsafe extern "C" fn gettagtype() -> libc::c_int {
    let mut f: libc::c_int = 0;
    if strcmp(tags, b"GTAGS\0" as *const u8 as *const libc::c_char) == 0 as libc::c_int {
        return T_GTAGS as libc::c_int;
    }
    if strcmp(tags, b"GRTAGS\0" as *const u8 as *const libc::c_char) == 0 as libc::c_int
    {
        return T_GRTAGS as libc::c_int;
    }
    if strcmp(tags, b"GSYMS\0" as *const u8 as *const libc::c_char) == 0 as libc::c_int {
        return T_GSYMS as libc::c_int;
    }
    if strcmp(tags, b"GPATH\0" as *const u8 as *const libc::c_char) == 0 as libc::c_int {
        return T_GPATH as libc::c_int;
    }
    if strcmp(tags, b"-\0" as *const u8 as *const libc::c_char) == 0 as libc::c_int {
        return T_CTAGS_X as libc::c_int;
    }
    f = open(tags, 0 as libc::c_int);
    if f >= 0 as libc::c_int {
        close(f);
        return T_CTAGS as libc::c_int;
    }
    return T_GTAGS as libc::c_int;
}
pub unsafe extern "C" fn findtag(mut tag: *mut libc::c_char) {
    let mut type_0: libc::c_int = gettagtype();
    let mut result: tag_result = TAG_FOUND;
    if type_0 == T_CTAGS as libc::c_int {
        result = findctag(tag);
    } else {
        result = findgtag(tag, type_0);
    }
    match result as libc::c_uint {
        1 => {
            error(
                b"No tags file\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                0 as *mut libc::c_void as *mut PARG,
            );
        }
        2 => {
            error(
                b"No such tag in tags file\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                0 as *mut libc::c_void as *mut PARG,
            );
        }
        3 => {
            error(
                b"unknown tag type\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                0 as *mut libc::c_void as *mut PARG,
            );
        }
        0 | 4 | _ => {}
    };
}
pub unsafe extern "C" fn tagsearch() -> POSITION {
    if curtag.is_null() {
        return -(1 as libc::c_int) as POSITION;
    }
    if (*curtag).tag_linenum != 0 as libc::c_int as libc::c_long {
        return gtagsearch()
    } else {
        return ctagsearch()
    };
}
pub unsafe extern "C" fn nexttag(mut n: libc::c_int) -> *mut libc::c_char {
    let mut tagfile: *mut libc::c_char = 0 as *mut libc::c_void as *mut libc::c_char;
    loop {
        let fresh0 = n;
        n = n - 1;
        if !(fresh0 > 0 as libc::c_int) {
            break;
        }
        tagfile = nextgtag();
    }
    return tagfile;
}
pub unsafe extern "C" fn prevtag(mut n: libc::c_int) -> *mut libc::c_char {
    let mut tagfile: *mut libc::c_char = 0 as *mut libc::c_void as *mut libc::c_char;
    loop {
        let fresh1 = n;
        n = n - 1;
        if !(fresh1 > 0 as libc::c_int) {
            break;
        }
        tagfile = prevgtag();
    }
    return tagfile;
}
pub unsafe extern "C" fn ntags() -> libc::c_int {
    return total;
}
pub unsafe extern "C" fn curr_tag() -> libc::c_int {
    return curseq;
}
unsafe extern "C" fn findctag(mut tag: *mut libc::c_char) -> tag_result {
    let mut p: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut q: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut f: *mut FILE = 0 as *mut FILE;
    let mut taglen: libc::c_int = 0;
    let mut taglinenum: LINENUM = 0;
    let mut tagfile: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut tagpattern: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut tagendline: libc::c_int = 0;
    let mut search_char: libc::c_int = 0;
    let mut err: libc::c_int = 0;
    let mut tline: [libc::c_char; 1024] = [0; 1024];
    let mut tp: *mut tag = 0 as *mut tag;
    p = shell_unquote(tags);
    f = fopen(p, b"r\0" as *const u8 as *const libc::c_char);
    free(p as *mut libc::c_void);
    if f.is_null() {
        return TAG_NOFILE;
    }
    cleantags();
    total = 0 as libc::c_int;
    taglen = strlen(tag) as libc::c_int;
    while !(fgets(
        tline.as_mut_ptr(),
        ::std::mem::size_of::<[libc::c_char; 1024]>() as libc::c_ulong as libc::c_int,
        f,
    ))
        .is_null()
    {
        if !(tline[0 as libc::c_int as usize] as libc::c_int == '!' as i32) {
            if strncmp(tag, tline.as_mut_ptr(), taglen as libc::c_ulong)
                != 0 as libc::c_int
                || !(tline[taglen as usize] as libc::c_int == ' ' as i32
                    || tline[taglen as usize] as libc::c_int == '\t' as i32)
            {
                continue;
            }
            tagpattern = 0 as *mut libc::c_char;
            p = skipsp(tline.as_mut_ptr().offset(taglen as isize));
            if !(*p as libc::c_int == '\0' as i32) {
                tagfile = p;
                while !(*p as libc::c_int == ' ' as i32
                    || *p as libc::c_int == '\t' as i32)
                    && *p as libc::c_int != '\0' as i32
                {
                    p = p.offset(1);
                    p;
                }
                let fresh2 = p;
                p = p.offset(1);
                *fresh2 = '\0' as i32 as libc::c_char;
                p = skipsp(p);
                if !(*p as libc::c_int == '\0' as i32) {
                    tagendline = 0 as libc::c_int;
                    taglinenum = getnum(&mut p, 0 as *mut libc::c_char, &mut err)
                        as LINENUM;
                    if err != 0 {
                        taglinenum = 0 as libc::c_int as LINENUM;
                        let fresh3 = p;
                        p = p.offset(1);
                        search_char = *fresh3 as libc::c_int;
                        if *p as libc::c_int == '^' as i32 {
                            p = p.offset(1);
                            p;
                        }
                        q = p;
                        tagpattern = q;
                        while *p as libc::c_int != search_char
                            && *p as libc::c_int != '\0' as i32
                        {
                            if *p as libc::c_int == '\\' as i32 {
                                p = p.offset(1);
                                p;
                            }
                            if q != p {
                                let fresh4 = p;
                                p = p.offset(1);
                                let fresh5 = q;
                                q = q.offset(1);
                                *fresh5 = *fresh4;
                            } else {
                                q = q.offset(1);
                                q;
                                p = p.offset(1);
                                p;
                            }
                        }
                        tagendline = (*q.offset(-(1 as libc::c_int) as isize)
                            as libc::c_int == '$' as i32) as libc::c_int;
                        if tagendline != 0 {
                            q = q.offset(-1);
                            q;
                        }
                        *q = '\0' as i32 as libc::c_char;
                    }
                    tp = maketagent(tag, tagfile, taglinenum, tagpattern, tagendline);
                    (*tp).next = &mut taglist as *mut taglist as *mut tag;
                    (*tp).prev = taglist.tl_last;
                    (*taglist.tl_last).next = tp;
                    taglist.tl_last = tp;
                    total += 1;
                    total;
                }
            }
        }
    }
    fclose(f);
    if total == 0 as libc::c_int {
        return TAG_NOTAG;
    }
    curtag = taglist.tl_first;
    curseq = 1 as libc::c_int;
    return TAG_FOUND;
}
pub unsafe extern "C" fn edit_tagfile() -> libc::c_int {
    if curtag.is_null() {
        return 1 as libc::c_int;
    }
    return edit((*curtag).tag_file);
}
unsafe extern "C" fn curtag_match(
    mut line: *const libc::c_char,
    mut linepos: POSITION,
) -> libc::c_int {
    let mut len: libc::c_int = strlen((*curtag).tag_pattern) as libc::c_int;
    if strncmp((*curtag).tag_pattern, line, len as libc::c_ulong) == 0 as libc::c_int
        && ((*curtag).tag_endline == 0
            || *line.offset(len as isize) as libc::c_int == '\0' as i32
            || *line.offset(len as isize) as libc::c_int == '\r' as i32)
    {
        (*curtag).tag_linenum = find_linenum(linepos);
        return 1 as libc::c_int;
    }
    return 0 as libc::c_int;
}
unsafe extern "C" fn ctagsearch() -> POSITION {
    let mut pos: POSITION = 0;
    let mut linepos: POSITION = 0;
    let mut linenum: LINENUM = 0;
    let mut line_len: libc::c_int = 0;
    let mut line: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut found: libc::c_int = 0;
    pos = 0 as libc::c_int as POSITION;
    linenum = find_linenum(pos);
    found = 0 as libc::c_int;
    while found == 0 {
        if sigs & (0o1 as libc::c_int | 0o2 as libc::c_int) != 0 {
            return -(1 as libc::c_int) as POSITION;
        }
        linepos = pos;
        pos = forw_raw_line(pos, &mut line, &mut line_len);
        if linenum != 0 as libc::c_int as libc::c_long {
            linenum += 1;
            linenum;
        }
        if pos == -(1 as libc::c_int) as POSITION {
            error(
                b"Tag not found\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                0 as *mut libc::c_void as *mut PARG,
            );
            return -(1 as libc::c_int) as POSITION;
        }
        if linenums != 0 {
            add_lnum(linenum, pos);
        }
        if ctldisp != 2 as libc::c_int {
            if curtag_match(line, linepos) != 0 {
                found = 1 as libc::c_int;
            }
        } else {
            let mut cvt_ops: libc::c_int = 0o10 as libc::c_int;
            let mut cvt_len: libc::c_int = cvt_length(line_len, cvt_ops);
            let mut chpos: *mut libc::c_int = cvt_alloc_chpos(cvt_len);
            let mut cline: *mut libc::c_char = ecalloc(
                1 as libc::c_int,
                cvt_len as libc::c_uint,
            ) as *mut libc::c_char;
            cvt_text(cline, line, chpos, &mut line_len, cvt_ops);
            if curtag_match(cline, linepos) != 0 {
                found = 1 as libc::c_int;
            }
            free(chpos as *mut libc::c_void);
            free(cline as *mut libc::c_void);
        }
    }
    return linepos;
}
unsafe extern "C" fn findgtag(
    mut tag: *mut libc::c_char,
    mut type_0: libc::c_int,
) -> tag_result {
    let mut buf: [libc::c_char; 1024] = [0; 1024];
    let mut fp: *mut FILE = 0 as *mut FILE;
    let mut tp: *mut tag = 0 as *mut tag;
    if type_0 != T_CTAGS_X as libc::c_int && tag.is_null() {
        return TAG_NOFILE;
    }
    cleantags();
    total = 0 as libc::c_int;
    if type_0 == T_CTAGS_X as libc::c_int {
        fp = stdin;
        tags = ztags.as_mut_ptr();
    } else {
        let mut command: *mut libc::c_char = 0 as *mut libc::c_char;
        let mut flag: *mut libc::c_char = 0 as *mut libc::c_char;
        let mut qtag: *mut libc::c_char = 0 as *mut libc::c_char;
        let mut cmd: *mut libc::c_char = lgetenv(
            b"LESSGLOBALTAGS\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        );
        if isnullenv(cmd) != 0 {
            return TAG_NOFILE;
        }
        match type_0 {
            2 => {
                flag = b"\0" as *const u8 as *const libc::c_char as *mut libc::c_char;
            }
            3 => {
                flag = b"r\0" as *const u8 as *const libc::c_char as *mut libc::c_char;
            }
            4 => {
                flag = b"s\0" as *const u8 as *const libc::c_char as *mut libc::c_char;
            }
            5 => {
                flag = b"P\0" as *const u8 as *const libc::c_char as *mut libc::c_char;
            }
            _ => return TAG_NOTYPE,
        }
        qtag = shell_quote(tag);
        if qtag.is_null() {
            qtag = tag;
        }
        command = ecalloc(
            (strlen(cmd))
                .wrapping_add(strlen(flag))
                .wrapping_add(strlen(qtag))
                .wrapping_add(5 as libc::c_int as libc::c_ulong) as libc::c_int,
            ::std::mem::size_of::<libc::c_char>() as libc::c_ulong as libc::c_uint,
        ) as *mut libc::c_char;
        sprintf(
            command,
            b"%s -x%s %s\0" as *const u8 as *const libc::c_char,
            cmd,
            flag,
            qtag,
        );
        if qtag != tag {
            free(qtag as *mut libc::c_void);
        }
        fp = popen(command, b"r\0" as *const u8 as *const libc::c_char);
        free(command as *mut libc::c_void);
    }
    if !fp.is_null() {
        while !(fgets(
            buf.as_mut_ptr(),
            ::std::mem::size_of::<[libc::c_char; 1024]>() as libc::c_ulong
                as libc::c_int,
            fp,
        ))
            .is_null()
        {
            let mut name: *mut libc::c_char = 0 as *mut libc::c_char;
            let mut file: *mut libc::c_char = 0 as *mut libc::c_char;
            let mut line: *mut libc::c_char = 0 as *mut libc::c_char;
            let mut len: libc::c_int = 0;
            if sigs != 0 {
                if fp != stdin {
                    pclose(fp);
                }
                return TAG_INTR;
            }
            len = strlen(buf.as_mut_ptr()) as libc::c_int;
            if len > 0 as libc::c_int
                && buf[(len - 1 as libc::c_int) as usize] as libc::c_int == '\n' as i32
            {
                buf[(len - 1 as libc::c_int) as usize] = '\0' as i32 as libc::c_char;
            } else {
                let mut c: libc::c_int = 0;
                loop {
                    c = fgetc(fp);
                    if !(c != '\n' as i32 && c != -(1 as libc::c_int)) {
                        break;
                    }
                }
            }
            if getentry(buf.as_mut_ptr(), &mut name, &mut file, &mut line) != 0 {
                break;
            }
            tp = maketagent(
                name,
                file,
                atoi(line) as LINENUM,
                0 as *mut libc::c_char,
                0 as libc::c_int,
            );
            (*tp).next = &mut taglist as *mut taglist as *mut tag;
            (*tp).prev = taglist.tl_last;
            (*taglist.tl_last).next = tp;
            taglist.tl_last = tp;
            total += 1;
            total;
        }
        if fp != stdin {
            if pclose(fp) != 0 {
                curtag = 0 as *mut tag;
                curseq = 0 as libc::c_int;
                total = curseq;
                return TAG_NOFILE;
            }
        }
    }
    tp = taglist.tl_first;
    if tp == &mut taglist as *mut taglist as *mut tag {
        return TAG_NOTAG;
    }
    curtag = tp;
    curseq = 1 as libc::c_int;
    return TAG_FOUND;
}
static mut circular: libc::c_int = 0 as libc::c_int;
unsafe extern "C" fn nextgtag() -> *mut libc::c_char {
    let mut tp: *mut tag = 0 as *mut tag;
    if curtag.is_null() {
        return 0 as *mut libc::c_char;
    }
    tp = (*curtag).next;
    if tp == &mut taglist as *mut taglist as *mut tag {
        if circular == 0 {
            return 0 as *mut libc::c_char;
        }
        curtag = taglist.tl_first;
        curseq = 1 as libc::c_int;
    } else {
        curtag = tp;
        curseq += 1;
        curseq;
    }
    return (*curtag).tag_file;
}
unsafe extern "C" fn prevgtag() -> *mut libc::c_char {
    let mut tp: *mut tag = 0 as *mut tag;
    if curtag.is_null() {
        return 0 as *mut libc::c_char;
    }
    tp = (*curtag).prev;
    if tp == &mut taglist as *mut taglist as *mut tag {
        if circular == 0 {
            return 0 as *mut libc::c_char;
        }
        curtag = taglist.tl_last;
        curseq = total;
    } else {
        curtag = tp;
        curseq -= 1;
        curseq;
    }
    return (*curtag).tag_file;
}
unsafe extern "C" fn gtagsearch() -> POSITION {
    if curtag.is_null() {
        return -(1 as libc::c_int) as POSITION;
    }
    return find_pos((*curtag).tag_linenum);
}
unsafe extern "C" fn getentry(
    mut buf: *mut libc::c_char,
    mut tag: *mut *mut libc::c_char,
    mut file: *mut *mut libc::c_char,
    mut line: *mut *mut libc::c_char,
) -> libc::c_int {
    let mut p: *mut libc::c_char = buf;
    *tag = p;
    while *p as libc::c_int != 0
        && *(*__ctype_b_loc()).offset(*p as libc::c_uchar as libc::c_int as isize)
            as libc::c_int & _ISspace as libc::c_int as libc::c_ushort as libc::c_int
            == 0
    {
        p = p.offset(1);
        p;
    }
    if *p as libc::c_int == 0 as libc::c_int {
        return -(1 as libc::c_int);
    }
    let fresh6 = p;
    p = p.offset(1);
    *fresh6 = 0 as libc::c_int as libc::c_char;
    while *p as libc::c_int != 0
        && *(*__ctype_b_loc()).offset(*p as libc::c_uchar as libc::c_int as isize)
            as libc::c_int & _ISspace as libc::c_int as libc::c_ushort as libc::c_int
            != 0
    {
        p = p.offset(1);
        p;
    }
    if *p as libc::c_int == 0 as libc::c_int {
        return -(1 as libc::c_int);
    }
    if *(*__ctype_b_loc()).offset(*p as libc::c_uchar as libc::c_int as isize)
        as libc::c_int & _ISdigit as libc::c_int as libc::c_ushort as libc::c_int == 0
    {
        while *p as libc::c_int != 0
            && *(*__ctype_b_loc()).offset(*p as libc::c_uchar as libc::c_int as isize)
                as libc::c_int & _ISspace as libc::c_int as libc::c_ushort as libc::c_int
                == 0
        {
            p = p.offset(1);
            p;
        }
        while *p as libc::c_int != 0
            && *(*__ctype_b_loc()).offset(*p as libc::c_uchar as libc::c_int as isize)
                as libc::c_int & _ISspace as libc::c_int as libc::c_ushort as libc::c_int
                != 0
        {
            p = p.offset(1);
            p;
        }
    }
    if *(*__ctype_b_loc()).offset(*p as libc::c_uchar as libc::c_int as isize)
        as libc::c_int & _ISdigit as libc::c_int as libc::c_ushort as libc::c_int == 0
    {
        return -(1 as libc::c_int);
    }
    *line = p;
    *line = p;
    while *p as libc::c_int != 0
        && *(*__ctype_b_loc()).offset(*p as libc::c_uchar as libc::c_int as isize)
            as libc::c_int & _ISspace as libc::c_int as libc::c_ushort as libc::c_int
            == 0
    {
        p = p.offset(1);
        p;
    }
    if *p as libc::c_int == 0 as libc::c_int {
        return -(1 as libc::c_int);
    }
    let fresh7 = p;
    p = p.offset(1);
    *fresh7 = 0 as libc::c_int as libc::c_char;
    while *p as libc::c_int != 0
        && *(*__ctype_b_loc()).offset(*p as libc::c_uchar as libc::c_int as isize)
            as libc::c_int & _ISspace as libc::c_int as libc::c_ushort as libc::c_int
            != 0
    {
        p = p.offset(1);
        p;
    }
    if *p as libc::c_int == 0 as libc::c_int {
        return -(1 as libc::c_int);
    }
    *file = p;
    *file = p;
    while *p as libc::c_int != 0
        && *(*__ctype_b_loc()).offset(*p as libc::c_uchar as libc::c_int as isize)
            as libc::c_int & _ISspace as libc::c_int as libc::c_ushort as libc::c_int
            == 0
    {
        p = p.offset(1);
        p;
    }
    if *p as libc::c_int == 0 as libc::c_int {
        return -(1 as libc::c_int);
    }
    *p = 0 as libc::c_int as libc::c_char;
    if strlen(*tag) != 0 && strlen(*line) != 0 && strlen(*file) != 0
        && atoi(*line) > 0 as libc::c_int
    {
        return 0 as libc::c_int;
    }
    return -(1 as libc::c_int);
}
