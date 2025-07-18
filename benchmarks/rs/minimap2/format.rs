use ::libc;
use ::c2rust_bitfields;
extern "C" {
    pub type _IO_wide_data;
    pub type _IO_codecvt;
    pub type _IO_marker;
    pub type mm_idx_intv_s;
    pub type mm_idx_bucket_s;
    fn malloc(_: libc::c_ulong) -> *mut libc::c_void;
    fn realloc(_: *mut libc::c_void, _: libc::c_ulong) -> *mut libc::c_void;
    fn free(__ptr: *mut libc::c_void);
    fn abort() -> !;
    fn memcpy(
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
    fn strstr(_: *const libc::c_char, _: *const libc::c_char) -> *mut libc::c_char;
    fn strlen(_: *const libc::c_char) -> libc::c_ulong;
    fn __assert_fail(
        __assertion: *const libc::c_char,
        __file: *const libc::c_char,
        __line: libc::c_uint,
        __function: *const libc::c_char,
    ) -> !;
    static mut stderr: *mut FILE;
    fn fprintf(_: *mut FILE, _: *const libc::c_char, _: ...) -> libc::c_int;
    fn snprintf(
        _: *mut libc::c_char,
        _: libc::c_ulong,
        _: *const libc::c_char,
        _: ...
    ) -> libc::c_int;
    fn kmalloc(km: *mut libc::c_void, size: size_t) -> *mut libc::c_void;
    fn kfree(km: *mut libc::c_void, ptr: *mut libc::c_void);
    static mut mm_verbose: libc::c_int;
    static mut seq_nt4_table: [libc::c_uchar; 256];
    fn mm_idx_getseq(
        mi: *const mm_idx_t,
        rid: uint32_t,
        st: uint32_t,
        en: uint32_t,
        seq: *mut uint8_t,
    ) -> libc::c_int;
    fn mm_idx_getseq2(
        mi: *const mm_idx_t,
        is_rev: libc::c_int,
        rid: uint32_t,
        st: uint32_t,
        en: uint32_t,
        seq: *mut uint8_t,
    ) -> libc::c_int;
    static mut seq_comp_table: [libc::c_uchar; 256];
    fn mm_event_identity(r: *const mm_reg1_t) -> libc::c_double;
    fn mm_err_puts(str: *const libc::c_char);
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
pub type va_list = __builtin_va_list;
pub type size_t = libc::c_ulong;
pub type __uint8_t = libc::c_uchar;
pub type __int32_t = libc::c_int;
pub type __uint32_t = libc::c_uint;
pub type __int64_t = libc::c_long;
pub type __uint64_t = libc::c_ulong;
pub type __off_t = libc::c_long;
pub type __off64_t = libc::c_long;
pub type int32_t = __int32_t;
pub type int64_t = __int64_t;
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
pub type uint8_t = __uint8_t;
pub type uint32_t = __uint32_t;
pub type uint64_t = __uint64_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct mm_idx_seq_t {
    pub name: *mut libc::c_char,
    pub offset: uint64_t,
    pub len: uint32_t,
    pub is_alt: uint32_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct mm_idx_t {
    pub b: int32_t,
    pub w: int32_t,
    pub k: int32_t,
    pub flag: int32_t,
    pub n_seq: uint32_t,
    pub index: int32_t,
    pub n_alt: int32_t,
    pub seq: *mut mm_idx_seq_t,
    pub S: *mut uint32_t,
    pub B: *mut mm_idx_bucket_s,
    pub I: *mut mm_idx_intv_s,
    pub km: *mut libc::c_void,
    pub h: *mut libc::c_void,
}
#[derive(Copy, Clone, BitfieldStruct)]
#[repr(C)]
pub struct mm_extra_t {
    pub capacity: uint32_t,
    pub dp_score: int32_t,
    pub dp_max: int32_t,
    pub dp_max2: int32_t,
    #[bitfield(name = "n_ambi", ty = "uint32_t", bits = "0..=29")]
    #[bitfield(name = "trans_strand", ty = "uint32_t", bits = "30..=31")]
    pub n_ambi_trans_strand: [u8; 4],
    pub n_cigar: uint32_t,
    pub cigar: [uint32_t; 0],
}
#[derive(Copy, Clone, BitfieldStruct)]
#[repr(C)]
pub struct mm_reg1_t {
    pub id: int32_t,
    pub cnt: int32_t,
    pub rid: int32_t,
    pub score: int32_t,
    pub qs: int32_t,
    pub qe: int32_t,
    pub rs: int32_t,
    pub re: int32_t,
    pub parent: int32_t,
    pub subsc: int32_t,
    pub as_0: int32_t,
    pub mlen: int32_t,
    pub blen: int32_t,
    pub n_sub: int32_t,
    pub score0: int32_t,
    #[bitfield(name = "mapq", ty = "uint32_t", bits = "0..=7")]
    #[bitfield(name = "split", ty = "uint32_t", bits = "8..=9")]
    #[bitfield(name = "rev", ty = "uint32_t", bits = "10..=10")]
    #[bitfield(name = "inv", ty = "uint32_t", bits = "11..=11")]
    #[bitfield(name = "sam_pri", ty = "uint32_t", bits = "12..=12")]
    #[bitfield(name = "proper_frag", ty = "uint32_t", bits = "13..=13")]
    #[bitfield(name = "pe_thru", ty = "uint32_t", bits = "14..=14")]
    #[bitfield(name = "seg_split", ty = "uint32_t", bits = "15..=15")]
    #[bitfield(name = "seg_id", ty = "uint32_t", bits = "16..=23")]
    #[bitfield(name = "split_inv", ty = "uint32_t", bits = "24..=24")]
    #[bitfield(name = "is_alt", ty = "uint32_t", bits = "25..=25")]
    #[bitfield(name = "strand_retained", ty = "uint32_t", bits = "26..=26")]
    #[bitfield(name = "dummy", ty = "uint32_t", bits = "27..=31")]
    pub mapq_split_rev_inv_sam_pri_proper_frag_pe_thru_seg_split_seg_id_split_inv_is_alt_strand_retained_dummy: [u8; 4],
    pub hash: uint32_t,
    pub div: libc::c_float,
    pub p: *mut mm_extra_t,
}
pub type kstring_t = __kstring_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct __kstring_t {
    pub l: size_t,
    pub m: size_t,
    pub s: *mut libc::c_char,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct mm_bseq1_t {
    pub l_seq: libc::c_int,
    pub rid: libc::c_int,
    pub name: *mut libc::c_char,
    pub seq: *mut libc::c_char,
    pub qual: *mut libc::c_char,
    pub comment: *mut libc::c_char,
}
#[inline]
unsafe extern "C" fn mm_qname_len(mut s: *const libc::c_char) -> libc::c_int {
    let mut l: libc::c_int = 0;
    l = strlen(s) as libc::c_int;
    return if l >= 3 as libc::c_int
        && *s.offset((l - 1 as libc::c_int) as isize) as libc::c_int >= '0' as i32
        && *s.offset((l - 1 as libc::c_int) as isize) as libc::c_int <= '9' as i32
        && *s.offset((l - 2 as libc::c_int) as isize) as libc::c_int == '/' as i32
    {
        l - 2 as libc::c_int
    } else {
        l
    };
}
static mut mm_rg_id: [libc::c_char; 256] = [0; 256];
#[inline]
unsafe extern "C" fn str_enlarge(mut s: *mut kstring_t, mut l: libc::c_int) {
    if ((*s).l)
        .wrapping_add(l as libc::c_ulong)
        .wrapping_add(1 as libc::c_int as libc::c_ulong) > (*s).m
    {
        (*s)
            .m = ((*s).l)
            .wrapping_add(l as libc::c_ulong)
            .wrapping_add(1 as libc::c_int as libc::c_ulong);
        (*s).m = ((*s).m).wrapping_sub(1);
        (*s).m;
        (*s).m |= (*s).m >> 1 as libc::c_int;
        (*s).m |= (*s).m >> 2 as libc::c_int;
        (*s).m |= (*s).m >> 4 as libc::c_int;
        (*s).m |= (*s).m >> 8 as libc::c_int;
        (*s).m |= (*s).m >> 16 as libc::c_int;
        (*s).m = ((*s).m).wrapping_add(1);
        (*s).m;
        (*s).s = realloc((*s).s as *mut libc::c_void, (*s).m) as *mut libc::c_char;
    }
}
#[inline]
unsafe extern "C" fn str_copy(
    mut s: *mut kstring_t,
    mut st: *const libc::c_char,
    mut en: *const libc::c_char,
) {
    str_enlarge(s, en.offset_from(st) as libc::c_long as libc::c_int);
    memcpy(
        &mut *((*s).s).offset((*s).l as isize) as *mut libc::c_char as *mut libc::c_void,
        st as *const libc::c_void,
        en.offset_from(st) as libc::c_long as libc::c_ulong,
    );
    (*s)
        .l = ((*s).l as libc::c_ulong)
        .wrapping_add(en.offset_from(st) as libc::c_long as libc::c_ulong) as size_t
        as size_t;
}
unsafe extern "C" fn mm_sprintf_lite(
    mut s: *mut kstring_t,
    mut fmt: *const libc::c_char,
    mut args: ...
) {
    let mut buf: [libc::c_char; 16] = [0; 16];
    let mut p: *const libc::c_char = 0 as *const libc::c_char;
    let mut q: *const libc::c_char = 0 as *const libc::c_char;
    let mut ap: ::std::ffi::VaListImpl;
    ap = args.clone();
    p = fmt;
    q = p;
    while *p != 0 {
        if *p as libc::c_int == '%' as i32 {
            if p > q {
                str_copy(s, q, p);
            }
            p = p.offset(1);
            p;
            if *p as libc::c_int == 'd' as i32 {
                let mut c: libc::c_int = 0;
                let mut i: libc::c_int = 0;
                let mut l: libc::c_int = 0 as libc::c_int;
                let mut x: libc::c_uint = 0;
                c = ap.arg::<libc::c_int>();
                x = (if c >= 0 as libc::c_int { c } else { -c }) as libc::c_uint;
                loop {
                    let fresh0 = l;
                    l = l + 1;
                    buf[fresh0
                        as usize] = x
                        .wrapping_rem(10 as libc::c_int as libc::c_uint)
                        .wrapping_add('0' as i32 as libc::c_uint) as libc::c_char;
                    x = x.wrapping_div(10 as libc::c_int as libc::c_uint);
                    if !(x > 0 as libc::c_int as libc::c_uint) {
                        break;
                    }
                }
                if c < 0 as libc::c_int {
                    let fresh1 = l;
                    l = l + 1;
                    buf[fresh1 as usize] = '-' as i32 as libc::c_char;
                }
                str_enlarge(s, l);
                i = l - 1 as libc::c_int;
                while i >= 0 as libc::c_int {
                    let fresh2 = (*s).l;
                    (*s).l = ((*s).l).wrapping_add(1);
                    *((*s).s).offset(fresh2 as isize) = buf[i as usize];
                    i -= 1;
                    i;
                }
            } else if *p as libc::c_int == 'u' as i32 {
                let mut i_0: libc::c_int = 0;
                let mut l_0: libc::c_int = 0 as libc::c_int;
                let mut x_0: uint32_t = 0;
                x_0 = ap.arg::<uint32_t>();
                loop {
                    let fresh3 = l_0;
                    l_0 = l_0 + 1;
                    buf[fresh3
                        as usize] = x_0
                        .wrapping_rem(10 as libc::c_int as libc::c_uint)
                        .wrapping_add('0' as i32 as libc::c_uint) as libc::c_char;
                    x_0 = (x_0 as libc::c_uint)
                        .wrapping_div(10 as libc::c_int as libc::c_uint) as uint32_t
                        as uint32_t;
                    if !(x_0 > 0 as libc::c_int as libc::c_uint) {
                        break;
                    }
                }
                str_enlarge(s, l_0);
                i_0 = l_0 - 1 as libc::c_int;
                while i_0 >= 0 as libc::c_int {
                    let fresh4 = (*s).l;
                    (*s).l = ((*s).l).wrapping_add(1);
                    *((*s).s).offset(fresh4 as isize) = buf[i_0 as usize];
                    i_0 -= 1;
                    i_0;
                }
            } else if *p as libc::c_int == 's' as i32 {
                let mut r: *mut libc::c_char = ap.arg::<*mut libc::c_char>();
                str_copy(s, r, r.offset(strlen(r) as isize));
            } else if *p as libc::c_int == 'c' as i32 {
                str_enlarge(s, 1 as libc::c_int);
                let fresh5 = (*s).l;
                (*s).l = ((*s).l).wrapping_add(1);
                *((*s).s)
                    .offset(fresh5 as isize) = ap.arg::<libc::c_int>() as libc::c_char;
            } else {
                abort();
            }
            q = p.offset(1 as libc::c_int as isize);
        }
        p = p.offset(1);
        p;
    }
    if p > q {
        str_copy(s, q, p);
    }
    *((*s).s).offset((*s).l as isize) = 0 as libc::c_int as libc::c_char;
}
unsafe extern "C" fn mm_escape(mut s: *mut libc::c_char) -> *mut libc::c_char {
    let mut p: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut q: *mut libc::c_char = 0 as *mut libc::c_char;
    q = s;
    p = q;
    while *p != 0 {
        if *p as libc::c_int == '\\' as i32 {
            p = p.offset(1);
            p;
            if *p as libc::c_int == 't' as i32 {
                let fresh6 = q;
                q = q.offset(1);
                *fresh6 = '\t' as i32 as libc::c_char;
            } else if *p as libc::c_int == '\\' as i32 {
                let fresh7 = q;
                q = q.offset(1);
                *fresh7 = '\\' as i32 as libc::c_char;
            }
        } else {
            let fresh8 = q;
            q = q.offset(1);
            *fresh8 = *p;
        }
        p = p.offset(1);
        p;
    }
    *q = '\0' as i32 as libc::c_char;
    return s;
}
unsafe extern "C" fn sam_write_rg_line(
    mut str: *mut kstring_t,
    mut s: *const libc::c_char,
) -> libc::c_int {
    let mut p: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut q: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut r: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut rg_line: *mut libc::c_char = 0 as *mut libc::c_char;
    memset(
        mm_rg_id.as_mut_ptr() as *mut libc::c_void,
        0 as libc::c_int,
        256 as libc::c_int as libc::c_ulong,
    );
    if s.is_null() {
        return 0 as libc::c_int;
    }
    if strstr(s, b"@RG\0" as *const u8 as *const libc::c_char) != s as *mut libc::c_char
    {
        if mm_verbose >= 1 as libc::c_int {
            fprintf(
                stderr,
                b"[ERROR] the read group line is not started with @RG\n\0" as *const u8
                    as *const libc::c_char,
            );
        }
    } else if !(strstr(s, b"\t\0" as *const u8 as *const libc::c_char)).is_null() {
        if mm_verbose >= 1 as libc::c_int {
            fprintf(
                stderr,
                b"[ERROR] the read group line contained literal <tab> characters -- replace with escaped tabs: \\t\n\0"
                    as *const u8 as *const libc::c_char,
            );
        }
    } else {
        rg_line = malloc((strlen(s)).wrapping_add(1 as libc::c_int as libc::c_ulong))
            as *mut libc::c_char;
        strcpy(rg_line, s);
        mm_escape(rg_line);
        p = strstr(rg_line, b"\tID:\0" as *const u8 as *const libc::c_char);
        if p.is_null() {
            if mm_verbose >= 1 as libc::c_int {
                fprintf(
                    stderr,
                    b"[ERROR] no ID within the read group line\n\0" as *const u8
                        as *const libc::c_char,
                );
            }
        } else {
            p = p.offset(4 as libc::c_int as isize);
            q = p;
            while *q as libc::c_int != 0 && *q as libc::c_int != '\t' as i32
                && *q as libc::c_int != '\n' as i32
            {
                q = q.offset(1);
                q;
            }
            if q.offset_from(p) as libc::c_long + 1 as libc::c_int as libc::c_long
                > 256 as libc::c_int as libc::c_long
            {
                if mm_verbose >= 1 as libc::c_int {
                    fprintf(
                        stderr,
                        b"[ERROR] @RG:ID is longer than 255 characters\n\0" as *const u8
                            as *const libc::c_char,
                    );
                }
            } else {
                q = p;
                r = mm_rg_id.as_mut_ptr();
                while *q as libc::c_int != 0 && *q as libc::c_int != '\t' as i32
                    && *q as libc::c_int != '\n' as i32
                {
                    let fresh9 = r;
                    r = r.offset(1);
                    *fresh9 = *q;
                    q = q.offset(1);
                    q;
                }
                mm_sprintf_lite(
                    str,
                    b"%s\n\0" as *const u8 as *const libc::c_char,
                    rg_line,
                );
                return 0 as libc::c_int;
            }
        }
    }
    free(rg_line as *mut libc::c_void);
    return -(1 as libc::c_int);
}
pub unsafe extern "C" fn mm_write_sam_hdr(
    mut idx: *const mm_idx_t,
    mut rg: *const libc::c_char,
    mut ver: *const libc::c_char,
    mut argc: libc::c_int,
    mut argv: *mut *mut libc::c_char,
) -> libc::c_int {
    let mut str: kstring_t = {
        let mut init = __kstring_t {
            l: 0 as libc::c_int as size_t,
            m: 0 as libc::c_int as size_t,
            s: 0 as *mut libc::c_char,
        };
        init
    };
    let mut ret: libc::c_int = 0 as libc::c_int;
    if !idx.is_null() {
        let mut i: uint32_t = 0;
        i = 0 as libc::c_int as uint32_t;
        while i < (*idx).n_seq {
            mm_sprintf_lite(
                &mut str as *mut kstring_t,
                b"@SQ\tSN:%s\tLN:%d\n\0" as *const u8 as *const libc::c_char,
                (*((*idx).seq).offset(i as isize)).name,
                (*((*idx).seq).offset(i as isize)).len,
            );
            i = i.wrapping_add(1);
            i;
        }
    }
    if !rg.is_null() {
        ret = sam_write_rg_line(&mut str, rg);
    }
    mm_sprintf_lite(
        &mut str as *mut kstring_t,
        b"@PG\tID:minimap2\tPN:minimap2\0" as *const u8 as *const libc::c_char,
    );
    if !ver.is_null() {
        mm_sprintf_lite(
            &mut str as *mut kstring_t,
            b"\tVN:%s\0" as *const u8 as *const libc::c_char,
            ver,
        );
    }
    if argc > 1 as libc::c_int {
        let mut i_0: libc::c_int = 0;
        mm_sprintf_lite(
            &mut str as *mut kstring_t,
            b"\tCL:minimap2\0" as *const u8 as *const libc::c_char,
        );
        i_0 = 1 as libc::c_int;
        while i_0 < argc {
            mm_sprintf_lite(
                &mut str as *mut kstring_t,
                b" %s\0" as *const u8 as *const libc::c_char,
                *argv.offset(i_0 as isize),
            );
            i_0 += 1;
            i_0;
        }
    }
    mm_err_puts(str.s);
    free(str.s as *mut libc::c_void);
    return ret;
}
unsafe extern "C" fn write_cs_core(
    mut s: *mut kstring_t,
    mut tseq: *const uint8_t,
    mut qseq: *const uint8_t,
    mut r: *const mm_reg1_t,
    mut tmp: *mut libc::c_char,
    mut no_iden: libc::c_int,
    mut write_tag: libc::c_int,
) {
    let mut i: libc::c_int = 0;
    let mut q_off: libc::c_int = 0;
    let mut t_off: libc::c_int = 0;
    if write_tag != 0 {
        mm_sprintf_lite(s, b"\tcs:Z:\0" as *const u8 as *const libc::c_char);
    }
    t_off = 0 as libc::c_int;
    q_off = t_off;
    i = q_off;
    while i < (*(*r).p).n_cigar as libc::c_int {
        let mut j: libc::c_int = 0;
        let mut op: libc::c_int = (*((*(*r).p).cigar).as_mut_ptr().offset(i as isize)
            & 0xf as libc::c_int as libc::c_uint) as libc::c_int;
        let mut len: libc::c_int = (*((*(*r).p).cigar).as_mut_ptr().offset(i as isize)
            >> 4 as libc::c_int) as libc::c_int;
        if op >= 0 as libc::c_int && op <= 3 as libc::c_int || op == 7 as libc::c_int
            || op == 8 as libc::c_int
        {} else {
            __assert_fail(
                b"(op >= MM_CIGAR_MATCH && op <= MM_CIGAR_N_SKIP) || op == MM_CIGAR_EQ_MATCH || op == MM_CIGAR_X_MISMATCH\0"
                    as *const u8 as *const libc::c_char,
                b"format.c\0" as *const u8 as *const libc::c_char,
                147 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 103],
                    &[libc::c_char; 103],
                >(
                    b"void write_cs_core(kstring_t *, const uint8_t *, const uint8_t *, const mm_reg1_t *, char *, int, int)\0",
                ))
                    .as_ptr(),
            );
        }
        'c_3963: {
            if op >= 0 as libc::c_int && op <= 3 as libc::c_int || op == 7 as libc::c_int
                || op == 8 as libc::c_int
            {} else {
                __assert_fail(
                    b"(op >= MM_CIGAR_MATCH && op <= MM_CIGAR_N_SKIP) || op == MM_CIGAR_EQ_MATCH || op == MM_CIGAR_X_MISMATCH\0"
                        as *const u8 as *const libc::c_char,
                    b"format.c\0" as *const u8 as *const libc::c_char,
                    147 as libc::c_int as libc::c_uint,
                    (*::std::mem::transmute::<
                        &[u8; 103],
                        &[libc::c_char; 103],
                    >(
                        b"void write_cs_core(kstring_t *, const uint8_t *, const uint8_t *, const mm_reg1_t *, char *, int, int)\0",
                    ))
                        .as_ptr(),
                );
            }
        };
        if op == 0 as libc::c_int || op == 7 as libc::c_int || op == 8 as libc::c_int {
            let mut l_tmp: libc::c_int = 0 as libc::c_int;
            j = 0 as libc::c_int;
            while j < len {
                if *qseq.offset((q_off + j) as isize) as libc::c_int
                    != *tseq.offset((t_off + j) as isize) as libc::c_int
                {
                    if l_tmp > 0 as libc::c_int {
                        if no_iden == 0 {
                            *tmp
                                .offset(l_tmp as isize) = 0 as libc::c_int as libc::c_char;
                            mm_sprintf_lite(
                                s,
                                b"=%s\0" as *const u8 as *const libc::c_char,
                                tmp,
                            );
                        } else {
                            mm_sprintf_lite(
                                s,
                                b":%d\0" as *const u8 as *const libc::c_char,
                                l_tmp,
                            );
                        }
                        l_tmp = 0 as libc::c_int;
                    }
                    mm_sprintf_lite(
                        s,
                        b"*%c%c\0" as *const u8 as *const libc::c_char,
                        (*::std::mem::transmute::<
                            &[u8; 6],
                            &[libc::c_char; 6],
                        >(b"acgtn\0"))[*tseq.offset((t_off + j) as isize) as usize]
                            as libc::c_int,
                        (*::std::mem::transmute::<
                            &[u8; 6],
                            &[libc::c_char; 6],
                        >(b"acgtn\0"))[*qseq.offset((q_off + j) as isize) as usize]
                            as libc::c_int,
                    );
                } else {
                    let fresh10 = l_tmp;
                    l_tmp = l_tmp + 1;
                    *tmp
                        .offset(
                            fresh10 as isize,
                        ) = (*::std::mem::transmute::<
                        &[u8; 6],
                        &[libc::c_char; 6],
                    >(b"ACGTN\0"))[*qseq.offset((q_off + j) as isize) as usize];
                }
                j += 1;
                j;
            }
            if l_tmp > 0 as libc::c_int {
                if no_iden == 0 {
                    *tmp.offset(l_tmp as isize) = 0 as libc::c_int as libc::c_char;
                    mm_sprintf_lite(
                        s,
                        b"=%s\0" as *const u8 as *const libc::c_char,
                        tmp,
                    );
                } else {
                    mm_sprintf_lite(
                        s,
                        b":%d\0" as *const u8 as *const libc::c_char,
                        l_tmp,
                    );
                }
            }
            q_off += len;
            t_off += len;
        } else if op == 1 as libc::c_int {
            j = 0 as libc::c_int;
            *tmp.offset(len as isize) = 0 as libc::c_int as libc::c_char;
            while j < len {
                *tmp
                    .offset(
                        j as isize,
                    ) = (*::std::mem::transmute::<
                    &[u8; 6],
                    &[libc::c_char; 6],
                >(b"acgtn\0"))[*qseq.offset((q_off + j) as isize) as usize];
                j += 1;
                j;
            }
            mm_sprintf_lite(s, b"+%s\0" as *const u8 as *const libc::c_char, tmp);
            q_off += len;
        } else if op == 2 as libc::c_int {
            j = 0 as libc::c_int;
            *tmp.offset(len as isize) = 0 as libc::c_int as libc::c_char;
            while j < len {
                *tmp
                    .offset(
                        j as isize,
                    ) = (*::std::mem::transmute::<
                    &[u8; 6],
                    &[libc::c_char; 6],
                >(b"acgtn\0"))[*tseq.offset((t_off + j) as isize) as usize];
                j += 1;
                j;
            }
            mm_sprintf_lite(s, b"-%s\0" as *const u8 as *const libc::c_char, tmp);
            t_off += len;
        } else {
            if len >= 2 as libc::c_int {} else {
                __assert_fail(
                    b"len >= 2\0" as *const u8 as *const libc::c_char,
                    b"format.c\0" as *const u8 as *const libc::c_char,
                    180 as libc::c_int as libc::c_uint,
                    (*::std::mem::transmute::<
                        &[u8; 103],
                        &[libc::c_char; 103],
                    >(
                        b"void write_cs_core(kstring_t *, const uint8_t *, const uint8_t *, const mm_reg1_t *, char *, int, int)\0",
                    ))
                        .as_ptr(),
                );
            }
            'c_3585: {
                if len >= 2 as libc::c_int {} else {
                    __assert_fail(
                        b"len >= 2\0" as *const u8 as *const libc::c_char,
                        b"format.c\0" as *const u8 as *const libc::c_char,
                        180 as libc::c_int as libc::c_uint,
                        (*::std::mem::transmute::<
                            &[u8; 103],
                            &[libc::c_char; 103],
                        >(
                            b"void write_cs_core(kstring_t *, const uint8_t *, const uint8_t *, const mm_reg1_t *, char *, int, int)\0",
                        ))
                            .as_ptr(),
                    );
                }
            };
            mm_sprintf_lite(
                s,
                b"~%c%c%d%c%c\0" as *const u8 as *const libc::c_char,
                (*::std::mem::transmute::<
                    &[u8; 6],
                    &[libc::c_char; 6],
                >(b"acgtn\0"))[*tseq.offset(t_off as isize) as usize] as libc::c_int,
                (*::std::mem::transmute::<
                    &[u8; 6],
                    &[libc::c_char; 6],
                >(
                    b"acgtn\0",
                ))[*tseq.offset((t_off + 1 as libc::c_int) as isize) as usize]
                    as libc::c_int,
                len,
                (*::std::mem::transmute::<
                    &[u8; 6],
                    &[libc::c_char; 6],
                >(
                    b"acgtn\0",
                ))[*tseq.offset((t_off + len - 2 as libc::c_int) as isize) as usize]
                    as libc::c_int,
                (*::std::mem::transmute::<
                    &[u8; 6],
                    &[libc::c_char; 6],
                >(
                    b"acgtn\0",
                ))[*tseq.offset((t_off + len - 1 as libc::c_int) as isize) as usize]
                    as libc::c_int,
            );
            t_off += len;
        }
        i += 1;
        i;
    }
    if t_off == (*r).re - (*r).rs && q_off == (*r).qe - (*r).qs {} else {
        __assert_fail(
            b"t_off == r->re - r->rs && q_off == r->qe - r->qs\0" as *const u8
                as *const libc::c_char,
            b"format.c\0" as *const u8 as *const libc::c_char,
            186 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 103],
                &[libc::c_char; 103],
            >(
                b"void write_cs_core(kstring_t *, const uint8_t *, const uint8_t *, const mm_reg1_t *, char *, int, int)\0",
            ))
                .as_ptr(),
        );
    }
    'c_2812: {
        if t_off == (*r).re - (*r).rs && q_off == (*r).qe - (*r).qs {} else {
            __assert_fail(
                b"t_off == r->re - r->rs && q_off == r->qe - r->qs\0" as *const u8
                    as *const libc::c_char,
                b"format.c\0" as *const u8 as *const libc::c_char,
                186 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 103],
                    &[libc::c_char; 103],
                >(
                    b"void write_cs_core(kstring_t *, const uint8_t *, const uint8_t *, const mm_reg1_t *, char *, int, int)\0",
                ))
                    .as_ptr(),
            );
        }
    };
}
unsafe extern "C" fn write_MD_core(
    mut s: *mut kstring_t,
    mut tseq: *const uint8_t,
    mut qseq: *const uint8_t,
    mut r: *const mm_reg1_t,
    mut tmp: *mut libc::c_char,
    mut write_tag: libc::c_int,
) {
    let mut i: libc::c_int = 0;
    let mut q_off: libc::c_int = 0;
    let mut t_off: libc::c_int = 0;
    let mut l_MD: libc::c_int = 0 as libc::c_int;
    if write_tag != 0 {
        mm_sprintf_lite(s, b"\tMD:Z:\0" as *const u8 as *const libc::c_char);
    }
    t_off = 0 as libc::c_int;
    q_off = t_off;
    i = q_off;
    while i < (*(*r).p).n_cigar as libc::c_int {
        let mut j: libc::c_int = 0;
        let mut op: libc::c_int = (*((*(*r).p).cigar).as_mut_ptr().offset(i as isize)
            & 0xf as libc::c_int as libc::c_uint) as libc::c_int;
        let mut len: libc::c_int = (*((*(*r).p).cigar).as_mut_ptr().offset(i as isize)
            >> 4 as libc::c_int) as libc::c_int;
        if op >= 0 as libc::c_int && op <= 3 as libc::c_int || op == 7 as libc::c_int
            || op == 8 as libc::c_int
        {} else {
            __assert_fail(
                b"(op >= MM_CIGAR_MATCH && op <= MM_CIGAR_N_SKIP) || op == MM_CIGAR_EQ_MATCH || op == MM_CIGAR_X_MISMATCH\0"
                    as *const u8 as *const libc::c_char,
                b"format.c\0" as *const u8 as *const libc::c_char,
                195 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 98],
                    &[libc::c_char; 98],
                >(
                    b"void write_MD_core(kstring_t *, const uint8_t *, const uint8_t *, const mm_reg1_t *, char *, int)\0",
                ))
                    .as_ptr(),
            );
        }
        'c_4412: {
            if op >= 0 as libc::c_int && op <= 3 as libc::c_int || op == 7 as libc::c_int
                || op == 8 as libc::c_int
            {} else {
                __assert_fail(
                    b"(op >= MM_CIGAR_MATCH && op <= MM_CIGAR_N_SKIP) || op == MM_CIGAR_EQ_MATCH || op == MM_CIGAR_X_MISMATCH\0"
                        as *const u8 as *const libc::c_char,
                    b"format.c\0" as *const u8 as *const libc::c_char,
                    195 as libc::c_int as libc::c_uint,
                    (*::std::mem::transmute::<
                        &[u8; 98],
                        &[libc::c_char; 98],
                    >(
                        b"void write_MD_core(kstring_t *, const uint8_t *, const uint8_t *, const mm_reg1_t *, char *, int)\0",
                    ))
                        .as_ptr(),
                );
            }
        };
        if op == 0 as libc::c_int || op == 7 as libc::c_int || op == 8 as libc::c_int {
            j = 0 as libc::c_int;
            while j < len {
                if *qseq.offset((q_off + j) as isize) as libc::c_int
                    != *tseq.offset((t_off + j) as isize) as libc::c_int
                {
                    mm_sprintf_lite(
                        s,
                        b"%d%c\0" as *const u8 as *const libc::c_char,
                        l_MD,
                        (*::std::mem::transmute::<
                            &[u8; 6],
                            &[libc::c_char; 6],
                        >(b"ACGTN\0"))[*tseq.offset((t_off + j) as isize) as usize]
                            as libc::c_int,
                    );
                    l_MD = 0 as libc::c_int;
                } else {
                    l_MD += 1;
                    l_MD;
                }
                j += 1;
                j;
            }
            q_off += len;
            t_off += len;
        } else if op == 1 as libc::c_int {
            q_off += len;
        } else if op == 2 as libc::c_int {
            j = 0 as libc::c_int;
            *tmp.offset(len as isize) = 0 as libc::c_int as libc::c_char;
            while j < len {
                *tmp
                    .offset(
                        j as isize,
                    ) = (*::std::mem::transmute::<
                    &[u8; 6],
                    &[libc::c_char; 6],
                >(b"ACGTN\0"))[*tseq.offset((t_off + j) as isize) as usize];
                j += 1;
                j;
            }
            mm_sprintf_lite(
                s,
                b"%d^%s\0" as *const u8 as *const libc::c_char,
                l_MD,
                tmp,
            );
            l_MD = 0 as libc::c_int;
            t_off += len;
        } else if op == 3 as libc::c_int {
            t_off += len;
        }
        i += 1;
        i;
    }
    if l_MD > 0 as libc::c_int {
        mm_sprintf_lite(s, b"%d\0" as *const u8 as *const libc::c_char, l_MD);
    }
    if t_off == (*r).re - (*r).rs && q_off == (*r).qe - (*r).qs {} else {
        __assert_fail(
            b"t_off == r->re - r->rs && q_off == r->qe - r->qs\0" as *const u8
                as *const libc::c_char,
            b"format.c\0" as *const u8 as *const libc::c_char,
            217 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 98],
                &[libc::c_char; 98],
            >(
                b"void write_MD_core(kstring_t *, const uint8_t *, const uint8_t *, const mm_reg1_t *, char *, int)\0",
            ))
                .as_ptr(),
        );
    }
    'c_4092: {
        if t_off == (*r).re - (*r).rs && q_off == (*r).qe - (*r).qs {} else {
            __assert_fail(
                b"t_off == r->re - r->rs && q_off == r->qe - r->qs\0" as *const u8
                    as *const libc::c_char,
                b"format.c\0" as *const u8 as *const libc::c_char,
                217 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 98],
                    &[libc::c_char; 98],
                >(
                    b"void write_MD_core(kstring_t *, const uint8_t *, const uint8_t *, const mm_reg1_t *, char *, int)\0",
                ))
                    .as_ptr(),
            );
        }
    };
}
unsafe extern "C" fn write_cs_or_MD(
    mut km: *mut libc::c_void,
    mut s: *mut kstring_t,
    mut mi: *const mm_idx_t,
    mut t: *const mm_bseq1_t,
    mut r: *const mm_reg1_t,
    mut no_iden: libc::c_int,
    mut is_MD: libc::c_int,
    mut write_tag: libc::c_int,
    mut is_qstrand: libc::c_int,
) {
    extern "C" {
        #[link_name = "seq_nt4_table"]
        static mut seq_nt4_table_0: [libc::c_uchar; 256];
    }
    let mut i: libc::c_int = 0;
    let mut qseq: *mut uint8_t = 0 as *mut uint8_t;
    let mut tseq: *mut uint8_t = 0 as *mut uint8_t;
    let mut tmp: *mut libc::c_char = 0 as *mut libc::c_char;
    if ((*r).p).is_null() {
        return;
    }
    qseq = kmalloc(km, ((*r).qe - (*r).qs) as size_t) as *mut uint8_t;
    tseq = kmalloc(km, ((*r).re - (*r).rs) as size_t) as *mut uint8_t;
    tmp = kmalloc(
        km,
        (if (*r).re - (*r).rs > (*r).qe - (*r).qs {
            (*r).re - (*r).rs + 1 as libc::c_int
        } else {
            (*r).qe - (*r).qs + 1 as libc::c_int
        }) as size_t,
    ) as *mut libc::c_char;
    if is_qstrand != 0 {
        mm_idx_getseq2(
            mi,
            (*r).rev() as libc::c_int,
            (*r).rid as uint32_t,
            (*r).rs as uint32_t,
            (*r).re as uint32_t,
            tseq,
        );
        i = (*r).qs;
        while i < (*r).qe {
            *qseq
                .offset(
                    (i - (*r).qs) as isize,
                ) = seq_nt4_table[*((*t).seq).offset(i as isize) as uint8_t as usize];
            i += 1;
            i;
        }
    } else {
        mm_idx_getseq(
            mi,
            (*r).rid as uint32_t,
            (*r).rs as uint32_t,
            (*r).re as uint32_t,
            tseq,
        );
        if (*r).rev() == 0 {
            i = (*r).qs;
            while i < (*r).qe {
                *qseq
                    .offset(
                        (i - (*r).qs) as isize,
                    ) = seq_nt4_table[*((*t).seq).offset(i as isize) as uint8_t
                    as usize];
                i += 1;
                i;
            }
        } else {
            i = (*r).qs;
            while i < (*r).qe {
                let mut c: uint8_t = seq_nt4_table[*((*t).seq).offset(i as isize)
                    as uint8_t as usize];
                *qseq
                    .offset(
                        ((*r).qe - i - 1 as libc::c_int) as isize,
                    ) = (if c as libc::c_int >= 4 as libc::c_int {
                    4 as libc::c_int
                } else {
                    3 as libc::c_int - c as libc::c_int
                }) as uint8_t;
                i += 1;
                i;
            }
        }
    }
    if is_MD != 0 {
        write_MD_core(s, tseq, qseq, r, tmp, write_tag);
    } else {
        write_cs_core(s, tseq, qseq, r, tmp, no_iden, write_tag);
    }
    kfree(km, qseq as *mut libc::c_void);
    kfree(km, tseq as *mut libc::c_void);
    kfree(km, tmp as *mut libc::c_void);
}
pub unsafe extern "C" fn mm_gen_cs_or_MD(
    mut km: *mut libc::c_void,
    mut buf: *mut *mut libc::c_char,
    mut max_len: *mut libc::c_int,
    mut mi: *const mm_idx_t,
    mut r: *const mm_reg1_t,
    mut seq: *const libc::c_char,
    mut is_MD: libc::c_int,
    mut no_iden: libc::c_int,
    mut is_qstrand: libc::c_int,
) -> libc::c_int {
    let mut t: mm_bseq1_t = mm_bseq1_t {
        l_seq: 0,
        rid: 0,
        name: 0 as *mut libc::c_char,
        seq: 0 as *mut libc::c_char,
        qual: 0 as *mut libc::c_char,
        comment: 0 as *mut libc::c_char,
    };
    let mut str: kstring_t = kstring_t {
        l: 0,
        m: 0,
        s: 0 as *mut libc::c_char,
    };
    str.s = *buf;
    str.l = 0 as libc::c_int as size_t;
    str.m = *max_len as size_t;
    t.l_seq = strlen(seq) as libc::c_int;
    t.seq = seq as *mut libc::c_char;
    write_cs_or_MD(
        km,
        &mut str,
        mi,
        &mut t,
        r,
        no_iden,
        is_MD,
        0 as libc::c_int,
        is_qstrand,
    );
    *max_len = str.m as libc::c_int;
    *buf = str.s;
    return str.l as libc::c_int;
}
pub unsafe extern "C" fn mm_gen_cs(
    mut km: *mut libc::c_void,
    mut buf: *mut *mut libc::c_char,
    mut max_len: *mut libc::c_int,
    mut mi: *const mm_idx_t,
    mut r: *const mm_reg1_t,
    mut seq: *const libc::c_char,
    mut no_iden: libc::c_int,
) -> libc::c_int {
    return mm_gen_cs_or_MD(
        km,
        buf,
        max_len,
        mi,
        r,
        seq,
        0 as libc::c_int,
        no_iden,
        0 as libc::c_int,
    );
}
pub unsafe extern "C" fn mm_gen_MD(
    mut km: *mut libc::c_void,
    mut buf: *mut *mut libc::c_char,
    mut max_len: *mut libc::c_int,
    mut mi: *const mm_idx_t,
    mut r: *const mm_reg1_t,
    mut seq: *const libc::c_char,
) -> libc::c_int {
    return mm_gen_cs_or_MD(
        km,
        buf,
        max_len,
        mi,
        r,
        seq,
        1 as libc::c_int,
        0 as libc::c_int,
        0 as libc::c_int,
    );
}
#[inline]
unsafe extern "C" fn write_tags(mut s: *mut kstring_t, mut r: *const mm_reg1_t) {
    let mut type_0: libc::c_int = 0;
    if (*r).id == (*r).parent {
        type_0 = if (*r).inv() as libc::c_int != 0 { 'I' as i32 } else { 'P' as i32 };
    } else {
        type_0 = if (*r).inv() as libc::c_int != 0 { 'i' as i32 } else { 'S' as i32 };
    }
    if !((*r).p).is_null() {
        mm_sprintf_lite(
            s,
            b"\tNM:i:%d\tms:i:%d\tAS:i:%d\tnn:i:%d\0" as *const u8
                as *const libc::c_char,
            (*r).blen - (*r).mlen + (*(*r).p).n_ambi() as libc::c_int,
            (*(*r).p).dp_max,
            (*(*r).p).dp_score,
            (*(*r).p).n_ambi() as libc::c_int,
        );
        if (*(*r).p).trans_strand() as libc::c_int == 1 as libc::c_int
            || (*(*r).p).trans_strand() as libc::c_int == 2 as libc::c_int
        {
            mm_sprintf_lite(
                s,
                b"\tts:A:%c\0" as *const u8 as *const libc::c_char,
                (*::std::mem::transmute::<
                    &[u8; 5],
                    &[libc::c_char; 5],
                >(b"?+-?\0"))[(*(*r).p).trans_strand() as usize] as libc::c_int,
            );
        }
    }
    mm_sprintf_lite(
        s,
        b"\ttp:A:%c\tcm:i:%d\ts1:i:%d\0" as *const u8 as *const libc::c_char,
        type_0,
        (*r).cnt,
        (*r).score,
    );
    if (*r).parent == (*r).id {
        mm_sprintf_lite(
            s,
            b"\ts2:i:%d\0" as *const u8 as *const libc::c_char,
            (*r).subsc,
        );
    }
    if !((*r).p).is_null() {
        let mut buf: [libc::c_char; 16] = [0; 16];
        let mut div: libc::c_double = 0.;
        div = 1.0f64 - mm_event_identity(r);
        if div == 0.0f64 {
            buf[0 as libc::c_int as usize] = '0' as i32 as libc::c_char;
            buf[1 as libc::c_int as usize] = 0 as libc::c_int as libc::c_char;
        } else {
            snprintf(
                buf.as_mut_ptr(),
                16 as libc::c_int as libc::c_ulong,
                b"%.4f\0" as *const u8 as *const libc::c_char,
                1.0f64 - mm_event_identity(r),
            );
        }
        mm_sprintf_lite(
            s,
            b"\tde:f:%s\0" as *const u8 as *const libc::c_char,
            buf.as_mut_ptr(),
        );
    } else if (*r).div >= 0.0f32 && (*r).div <= 1.0f32 {
        let mut buf_0: [libc::c_char; 16] = [0; 16];
        if (*r).div == 0.0f32 {
            buf_0[0 as libc::c_int as usize] = '0' as i32 as libc::c_char;
            buf_0[1 as libc::c_int as usize] = 0 as libc::c_int as libc::c_char;
        } else {
            snprintf(
                buf_0.as_mut_ptr(),
                16 as libc::c_int as libc::c_ulong,
                b"%.4f\0" as *const u8 as *const libc::c_char,
                (*r).div as libc::c_double,
            );
        }
        mm_sprintf_lite(
            s,
            b"\tdv:f:%s\0" as *const u8 as *const libc::c_char,
            buf_0.as_mut_ptr(),
        );
    }
    if (*r).split() != 0 {
        mm_sprintf_lite(
            s,
            b"\tzd:i:%d\0" as *const u8 as *const libc::c_char,
            (*r).split() as libc::c_int,
        );
    }
}
pub unsafe extern "C" fn mm_write_paf3(
    mut s: *mut kstring_t,
    mut mi: *const mm_idx_t,
    mut t: *const mm_bseq1_t,
    mut r: *const mm_reg1_t,
    mut km: *mut libc::c_void,
    mut opt_flag: int64_t,
    mut rep_len: libc::c_int,
) {
    (*s).l = 0 as libc::c_int as size_t;
    if r.is_null() {
        mm_sprintf_lite(
            s,
            b"%s\t%d\t0\t0\t*\t*\t0\t0\t0\t0\t0\t0\0" as *const u8
                as *const libc::c_char,
            (*t).name,
            (*t).l_seq,
        );
        if rep_len >= 0 as libc::c_int {
            mm_sprintf_lite(
                s,
                b"\trl:i:%d\0" as *const u8 as *const libc::c_char,
                rep_len,
            );
        }
        return;
    }
    mm_sprintf_lite(
        s,
        b"%s\t%d\t%d\t%d\t%c\t\0" as *const u8 as *const libc::c_char,
        (*t).name,
        (*t).l_seq,
        (*r).qs,
        (*r).qe,
        (*::std::mem::transmute::<
            &[u8; 3],
            &[libc::c_char; 3],
        >(b"+-\0"))[(*r).rev() as usize] as libc::c_int,
    );
    if !((*((*mi).seq).offset((*r).rid as isize)).name).is_null() {
        mm_sprintf_lite(
            s,
            b"%s\0" as *const u8 as *const libc::c_char,
            (*((*mi).seq).offset((*r).rid as isize)).name,
        );
    } else {
        mm_sprintf_lite(s, b"%d\0" as *const u8 as *const libc::c_char, (*r).rid);
    }
    mm_sprintf_lite(
        s,
        b"\t%d\0" as *const u8 as *const libc::c_char,
        (*((*mi).seq).offset((*r).rid as isize)).len,
    );
    if opt_flag as libc::c_longlong & 0x100000000 as libc::c_longlong != 0
        && (*r).rev() as libc::c_int != 0
    {
        mm_sprintf_lite(
            s,
            b"\t%d\t%d\0" as *const u8 as *const libc::c_char,
            ((*((*mi).seq).offset((*r).rid as isize)).len)
                .wrapping_sub((*r).re as libc::c_uint),
            ((*((*mi).seq).offset((*r).rid as isize)).len)
                .wrapping_sub((*r).rs as libc::c_uint),
        );
    } else {
        mm_sprintf_lite(
            s,
            b"\t%d\t%d\0" as *const u8 as *const libc::c_char,
            (*r).rs,
            (*r).re,
        );
    }
    mm_sprintf_lite(
        s,
        b"\t%d\t%d\0" as *const u8 as *const libc::c_char,
        (*r).mlen,
        (*r).blen,
    );
    mm_sprintf_lite(
        s,
        b"\t%d\0" as *const u8 as *const libc::c_char,
        (*r).mapq() as libc::c_int,
    );
    write_tags(s, r);
    if rep_len >= 0 as libc::c_int {
        mm_sprintf_lite(s, b"\trl:i:%d\0" as *const u8 as *const libc::c_char, rep_len);
    }
    if !((*r).p).is_null() && opt_flag & 0x20 as libc::c_int as libc::c_long != 0 {
        let mut k: uint32_t = 0;
        mm_sprintf_lite(s, b"\tcg:Z:\0" as *const u8 as *const libc::c_char);
        k = 0 as libc::c_int as uint32_t;
        while k < (*(*r).p).n_cigar {
            mm_sprintf_lite(
                s,
                b"%d%c\0" as *const u8 as *const libc::c_char,
                *((*(*r).p).cigar).as_mut_ptr().offset(k as isize) >> 4 as libc::c_int,
                (*::std::mem::transmute::<
                    &[u8; 11],
                    &[libc::c_char; 11],
                >(
                    b"MIDNSHP=XB\0",
                ))[(*((*(*r).p).cigar).as_mut_ptr().offset(k as isize)
                    & 0xf as libc::c_int as libc::c_uint) as usize] as libc::c_int,
            );
            k = k.wrapping_add(1);
            k;
        }
    }
    if !((*r).p).is_null()
        && opt_flag & (0x40 as libc::c_int | 0x1000000 as libc::c_int) as libc::c_long
            != 0
    {
        write_cs_or_MD(
            km,
            s,
            mi,
            t,
            r,
            (opt_flag & 0x800 as libc::c_int as libc::c_long == 0) as libc::c_int,
            (opt_flag & 0x1000000 as libc::c_int as libc::c_long) as libc::c_int,
            1 as libc::c_int,
            (opt_flag as libc::c_longlong & 0x100000000 as libc::c_longlong != 0)
                as libc::c_int,
        );
    }
    if opt_flag & 0x2000000 as libc::c_int as libc::c_long != 0
        && !((*t).comment).is_null()
    {
        mm_sprintf_lite(s, b"\t%s\0" as *const u8 as *const libc::c_char, (*t).comment);
    }
}
pub unsafe extern "C" fn mm_write_paf(
    mut s: *mut kstring_t,
    mut mi: *const mm_idx_t,
    mut t: *const mm_bseq1_t,
    mut r: *const mm_reg1_t,
    mut km: *mut libc::c_void,
    mut opt_flag: int64_t,
) {
    mm_write_paf3(s, mi, t, r, km, opt_flag, -(1 as libc::c_int));
}
unsafe extern "C" fn sam_write_sq(
    mut s: *mut kstring_t,
    mut seq: *mut libc::c_char,
    mut l: libc::c_int,
    mut rev: libc::c_int,
    mut comp: libc::c_int,
) {
    extern "C" {
        #[link_name = "seq_comp_table"]
        static mut seq_comp_table_0: [libc::c_uchar; 256];
    }
    if rev != 0 {
        let mut i: libc::c_int = 0;
        str_enlarge(s, l);
        i = 0 as libc::c_int;
        while i < l {
            let mut c: libc::c_int = *seq.offset((l - 1 as libc::c_int - i) as isize)
                as libc::c_int;
            *((*s).s)
                .offset(
                    ((*s).l).wrapping_add(i as libc::c_ulong) as isize,
                ) = (if c < 128 as libc::c_int && comp != 0 {
                seq_comp_table[c as usize] as libc::c_int
            } else {
                c
            }) as libc::c_char;
            i += 1;
            i;
        }
        (*s)
            .l = ((*s).l as libc::c_ulong).wrapping_add(l as libc::c_ulong) as size_t
            as size_t;
    } else {
        str_copy(s, seq, seq.offset(l as isize));
    };
}
#[inline]
unsafe extern "C" fn get_sam_pri(
    mut n_regs: libc::c_int,
    mut regs: *const mm_reg1_t,
) -> *const mm_reg1_t {
    let mut i: libc::c_int = 0;
    i = 0 as libc::c_int;
    while i < n_regs {
        if (*regs.offset(i as isize)).sam_pri() != 0 {
            return &*regs.offset(i as isize) as *const mm_reg1_t;
        }
        i += 1;
        i;
    }
    if n_regs == 0 as libc::c_int {} else {
        __assert_fail(
            b"n_regs == 0\0" as *const u8 as *const libc::c_char,
            b"format.c\0" as *const u8 as *const libc::c_char,
            359 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 53],
                &[libc::c_char; 53],
            >(b"const mm_reg1_t *get_sam_pri(int, const mm_reg1_t *)\0"))
                .as_ptr(),
        );
    }
    'c_9020: {
        if n_regs == 0 as libc::c_int {} else {
            __assert_fail(
                b"n_regs == 0\0" as *const u8 as *const libc::c_char,
                b"format.c\0" as *const u8 as *const libc::c_char,
                359 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 53],
                    &[libc::c_char; 53],
                >(b"const mm_reg1_t *get_sam_pri(int, const mm_reg1_t *)\0"))
                    .as_ptr(),
            );
        }
    };
    return 0 as *const mm_reg1_t;
}
unsafe extern "C" fn write_sam_cigar(
    mut s: *mut kstring_t,
    mut sam_flag: libc::c_int,
    mut in_tag: libc::c_int,
    mut qlen: libc::c_int,
    mut r: *const mm_reg1_t,
    mut opt_flag: int64_t,
) {
    if ((*r).p).is_null() {
        mm_sprintf_lite(s, b"*\0" as *const u8 as *const libc::c_char);
    } else {
        let mut k: uint32_t = 0;
        let mut clip_len: [uint32_t; 2] = [0; 2];
        clip_len[0 as libc::c_int
            as usize] = (if (*r).rev() as libc::c_int != 0 {
            qlen - (*r).qe
        } else {
            (*r).qs
        }) as uint32_t;
        clip_len[1 as libc::c_int
            as usize] = (if (*r).rev() as libc::c_int != 0 {
            (*r).qs
        } else {
            qlen - (*r).qe
        }) as uint32_t;
        if in_tag != 0 {
            let mut clip_char: libc::c_int = if sam_flag & 0x800 as libc::c_int != 0
                && opt_flag & 0x80000 as libc::c_int as libc::c_long == 0
            {
                5 as libc::c_int
            } else {
                4 as libc::c_int
            };
            mm_sprintf_lite(s, b"\tCG:B:I\0" as *const u8 as *const libc::c_char);
            if clip_len[0 as libc::c_int as usize] != 0 {
                mm_sprintf_lite(
                    s,
                    b",%u\0" as *const u8 as *const libc::c_char,
                    clip_len[0 as libc::c_int as usize] << 4 as libc::c_int
                        | clip_char as libc::c_uint,
                );
            }
            k = 0 as libc::c_int as uint32_t;
            while k < (*(*r).p).n_cigar {
                mm_sprintf_lite(
                    s,
                    b",%u\0" as *const u8 as *const libc::c_char,
                    *((*(*r).p).cigar).as_mut_ptr().offset(k as isize),
                );
                k = k.wrapping_add(1);
                k;
            }
            if clip_len[1 as libc::c_int as usize] != 0 {
                mm_sprintf_lite(
                    s,
                    b",%u\0" as *const u8 as *const libc::c_char,
                    clip_len[1 as libc::c_int as usize] << 4 as libc::c_int
                        | clip_char as libc::c_uint,
                );
            }
        } else {
            let mut clip_char_0: libc::c_int = if sam_flag & 0x800 as libc::c_int != 0
                && opt_flag & 0x80000 as libc::c_int as libc::c_long == 0
            {
                'H' as i32
            } else {
                'S' as i32
            };
            if clip_len[0 as libc::c_int as usize] < qlen as libc::c_uint
                && clip_len[1 as libc::c_int as usize] < qlen as libc::c_uint
            {} else {
                __assert_fail(
                    b"clip_len[0] < qlen && clip_len[1] < qlen\0" as *const u8
                        as *const libc::c_char,
                    b"format.c\0" as *const u8 as *const libc::c_char,
                    380 as libc::c_int as libc::c_uint,
                    (*::std::mem::transmute::<
                        &[u8; 77],
                        &[libc::c_char; 77],
                    >(
                        b"void write_sam_cigar(kstring_t *, int, int, int, const mm_reg1_t *, int64_t)\0",
                    ))
                        .as_ptr(),
                );
            }
            'c_7233: {
                if clip_len[0 as libc::c_int as usize] < qlen as libc::c_uint
                    && clip_len[1 as libc::c_int as usize] < qlen as libc::c_uint
                {} else {
                    __assert_fail(
                        b"clip_len[0] < qlen && clip_len[1] < qlen\0" as *const u8
                            as *const libc::c_char,
                        b"format.c\0" as *const u8 as *const libc::c_char,
                        380 as libc::c_int as libc::c_uint,
                        (*::std::mem::transmute::<
                            &[u8; 77],
                            &[libc::c_char; 77],
                        >(
                            b"void write_sam_cigar(kstring_t *, int, int, int, const mm_reg1_t *, int64_t)\0",
                        ))
                            .as_ptr(),
                    );
                }
            };
            if clip_len[0 as libc::c_int as usize] != 0 {
                mm_sprintf_lite(
                    s,
                    b"%d%c\0" as *const u8 as *const libc::c_char,
                    clip_len[0 as libc::c_int as usize],
                    clip_char_0,
                );
            }
            k = 0 as libc::c_int as uint32_t;
            while k < (*(*r).p).n_cigar {
                mm_sprintf_lite(
                    s,
                    b"%d%c\0" as *const u8 as *const libc::c_char,
                    *((*(*r).p).cigar).as_mut_ptr().offset(k as isize)
                        >> 4 as libc::c_int,
                    (*::std::mem::transmute::<
                        &[u8; 11],
                        &[libc::c_char; 11],
                    >(
                        b"MIDNSHP=XB\0",
                    ))[(*((*(*r).p).cigar).as_mut_ptr().offset(k as isize)
                        & 0xf as libc::c_int as libc::c_uint) as usize] as libc::c_int,
                );
                k = k.wrapping_add(1);
                k;
            }
            if clip_len[1 as libc::c_int as usize] != 0 {
                mm_sprintf_lite(
                    s,
                    b"%d%c\0" as *const u8 as *const libc::c_char,
                    clip_len[1 as libc::c_int as usize],
                    clip_char_0,
                );
            }
        }
    };
}
pub unsafe extern "C" fn mm_write_sam3(
    mut s: *mut kstring_t,
    mut mi: *const mm_idx_t,
    mut t: *const mm_bseq1_t,
    mut seg_idx: libc::c_int,
    mut reg_idx: libc::c_int,
    mut n_seg: libc::c_int,
    mut n_regss: *const libc::c_int,
    mut regss: *const *const mm_reg1_t,
    mut km: *mut libc::c_void,
    mut opt_flag: int64_t,
    mut rep_len: libc::c_int,
) {
    let max_bam_cigar_op: libc::c_int = 65535 as libc::c_int;
    let mut flag: libc::c_int = 0;
    let mut n_regs: libc::c_int = *n_regss.offset(seg_idx as isize);
    let mut cigar_in_tag: libc::c_int = 0 as libc::c_int;
    let mut this_rid: libc::c_int = -(1 as libc::c_int);
    let mut this_pos: libc::c_int = -(1 as libc::c_int);
    let mut regs: *const mm_reg1_t = *regss.offset(seg_idx as isize);
    let mut r_prev: *const mm_reg1_t = 0 as *const mm_reg1_t;
    let mut r_next: *const mm_reg1_t = 0 as *const mm_reg1_t;
    let mut r: *const mm_reg1_t = if n_regs > 0 as libc::c_int && reg_idx < n_regs
        && reg_idx >= 0 as libc::c_int
    {
        &*regs.offset(reg_idx as isize) as *const mm_reg1_t
    } else {
        0 as *const mm_reg1_t
    };
    if n_seg > 1 as libc::c_int {
        let mut i: libc::c_int = 0;
        let mut next_sid: libc::c_int = (seg_idx + 1 as libc::c_int) % n_seg;
        r_next = get_sam_pri(
            *n_regss.offset(next_sid as isize),
            *regss.offset(next_sid as isize),
        );
        if n_seg > 2 as libc::c_int {
            i = 1 as libc::c_int;
            while i <= n_seg - 1 as libc::c_int {
                let mut prev_sid: libc::c_int = (seg_idx + n_seg - i) % n_seg;
                if *n_regss.offset(prev_sid as isize) > 0 as libc::c_int {
                    r_prev = get_sam_pri(
                        *n_regss.offset(prev_sid as isize),
                        *regss.offset(prev_sid as isize),
                    );
                    break;
                } else {
                    i += 1;
                    i;
                }
            }
        } else {
            r_prev = r_next;
        }
    } else {
        r_next = 0 as *const mm_reg1_t;
        r_prev = r_next;
    }
    (*s).l = 0 as libc::c_int as size_t;
    mm_sprintf_lite(s, b"%s\0" as *const u8 as *const libc::c_char, (*t).name);
    if n_seg > 1 as libc::c_int {
        (*s).l = mm_qname_len((*t).name) as size_t;
    }
    flag = if n_seg > 1 as libc::c_int { 0x1 as libc::c_int } else { 0 as libc::c_int };
    if r.is_null() {
        flag |= 0x4 as libc::c_int;
    } else {
        if (*r).rev() != 0 {
            flag |= 0x10 as libc::c_int;
        }
        if (*r).parent != (*r).id {
            flag |= 0x100 as libc::c_int;
        } else if (*r).sam_pri() == 0 {
            flag |= 0x800 as libc::c_int;
        }
    }
    if n_seg > 1 as libc::c_int {
        if !r.is_null() && (*r).proper_frag() as libc::c_int != 0 {
            flag |= 0x2 as libc::c_int;
        }
        if seg_idx == 0 as libc::c_int {
            flag |= 0x40 as libc::c_int;
        } else if seg_idx == n_seg - 1 as libc::c_int {
            flag |= 0x80 as libc::c_int;
        }
        if r_next.is_null() {
            flag |= 0x8 as libc::c_int;
        } else if (*r_next).rev() != 0 {
            flag |= 0x20 as libc::c_int;
        }
    }
    mm_sprintf_lite(s, b"\t%d\0" as *const u8 as *const libc::c_char, flag);
    if r.is_null() {
        if !r_prev.is_null() {
            this_rid = (*r_prev).rid;
            this_pos = (*r_prev).rs;
            mm_sprintf_lite(
                s,
                b"\t%s\t%d\t0\t*\0" as *const u8 as *const libc::c_char,
                (*((*mi).seq).offset(this_rid as isize)).name,
                this_pos + 1 as libc::c_int,
            );
        } else {
            mm_sprintf_lite(s, b"\t*\t0\t0\t*\0" as *const u8 as *const libc::c_char);
        }
    } else {
        this_rid = (*r).rid;
        this_pos = (*r).rs;
        mm_sprintf_lite(
            s,
            b"\t%s\t%d\t%d\t\0" as *const u8 as *const libc::c_char,
            (*((*mi).seq).offset((*r).rid as isize)).name,
            (*r).rs + 1 as libc::c_int,
            (*r).mapq() as libc::c_int,
        );
        if opt_flag & 0x10000 as libc::c_int as libc::c_long != 0 && !((*r).p).is_null()
            && (*(*r).p).n_cigar > (max_bam_cigar_op - 2 as libc::c_int) as libc::c_uint
        {
            let mut n_cigar: libc::c_int = (*(*r).p).n_cigar as libc::c_int;
            if (*r).qs != 0 as libc::c_int {
                n_cigar += 1;
                n_cigar;
            }
            if (*r).qe != (*t).l_seq {
                n_cigar += 1;
                n_cigar;
            }
            if n_cigar > max_bam_cigar_op {
                cigar_in_tag = 1 as libc::c_int;
            }
        }
        if cigar_in_tag != 0 {
            let mut slen: libc::c_int = 0;
            if flag & 0x900 as libc::c_int == 0 as libc::c_int
                || opt_flag & 0x80000 as libc::c_int as libc::c_long != 0
            {
                slen = (*t).l_seq;
            } else if flag & 0x100 as libc::c_int != 0 {
                slen = 0 as libc::c_int;
            } else {
                slen = (*r).qe - (*r).qs;
            }
            mm_sprintf_lite(
                s,
                b"%dS%dN\0" as *const u8 as *const libc::c_char,
                slen,
                (*r).re - (*r).rs,
            );
        } else {
            write_sam_cigar(s, flag, 0 as libc::c_int, (*t).l_seq, r, opt_flag);
        }
    }
    if n_seg > 1 as libc::c_int {
        let mut tlen: libc::c_int = 0 as libc::c_int;
        if this_rid >= 0 as libc::c_int && !r_next.is_null() {
            if this_rid == (*r_next).rid {
                if !r.is_null() {
                    let mut this_pos5: libc::c_int = if (*r).rev() as libc::c_int != 0 {
                        (*r).re - 1 as libc::c_int
                    } else {
                        this_pos
                    };
                    let mut next_pos5: libc::c_int = if (*r_next).rev() as libc::c_int
                        != 0
                    {
                        (*r_next).re - 1 as libc::c_int
                    } else {
                        (*r_next).rs
                    };
                    tlen = next_pos5 - this_pos5;
                }
                mm_sprintf_lite(s, b"\t=\t\0" as *const u8 as *const libc::c_char);
            } else {
                mm_sprintf_lite(
                    s,
                    b"\t%s\t\0" as *const u8 as *const libc::c_char,
                    (*((*mi).seq).offset((*r_next).rid as isize)).name,
                );
            }
            mm_sprintf_lite(
                s,
                b"%d\t\0" as *const u8 as *const libc::c_char,
                (*r_next).rs + 1 as libc::c_int,
            );
        } else if !r_next.is_null() {
            mm_sprintf_lite(
                s,
                b"\t%s\t%d\t\0" as *const u8 as *const libc::c_char,
                (*((*mi).seq).offset((*r_next).rid as isize)).name,
                (*r_next).rs + 1 as libc::c_int,
            );
        } else if this_rid >= 0 as libc::c_int {
            mm_sprintf_lite(
                s,
                b"\t=\t%d\t\0" as *const u8 as *const libc::c_char,
                this_pos + 1 as libc::c_int,
            );
        } else {
            mm_sprintf_lite(s, b"\t*\t0\t\0" as *const u8 as *const libc::c_char);
        }
        if tlen > 0 as libc::c_int {
            tlen += 1;
            tlen;
        } else if tlen < 0 as libc::c_int {
            tlen -= 1;
            tlen;
        }
        mm_sprintf_lite(s, b"%d\t\0" as *const u8 as *const libc::c_char, tlen);
    } else {
        mm_sprintf_lite(s, b"\t*\t0\t0\t\0" as *const u8 as *const libc::c_char);
    }
    if r.is_null() {
        sam_write_sq(s, (*t).seq, (*t).l_seq, 0 as libc::c_int, 0 as libc::c_int);
        mm_sprintf_lite(s, b"\t\0" as *const u8 as *const libc::c_char);
        if !((*t).qual).is_null() {
            sam_write_sq(s, (*t).qual, (*t).l_seq, 0 as libc::c_int, 0 as libc::c_int);
        } else {
            mm_sprintf_lite(s, b"*\0" as *const u8 as *const libc::c_char);
        }
    } else if flag & 0x900 as libc::c_int == 0 as libc::c_int
        || opt_flag & 0x80000 as libc::c_int as libc::c_long != 0
    {
        sam_write_sq(
            s,
            (*t).seq,
            (*t).l_seq,
            (*r).rev() as libc::c_int,
            (*r).rev() as libc::c_int,
        );
        mm_sprintf_lite(s, b"\t\0" as *const u8 as *const libc::c_char);
        if !((*t).qual).is_null() {
            sam_write_sq(
                s,
                (*t).qual,
                (*t).l_seq,
                (*r).rev() as libc::c_int,
                0 as libc::c_int,
            );
        } else {
            mm_sprintf_lite(s, b"*\0" as *const u8 as *const libc::c_char);
        }
    } else if flag & 0x100 as libc::c_int != 0 {
        mm_sprintf_lite(s, b"*\t*\0" as *const u8 as *const libc::c_char);
    } else {
        sam_write_sq(
            s,
            ((*t).seq).offset((*r).qs as isize),
            (*r).qe - (*r).qs,
            (*r).rev() as libc::c_int,
            (*r).rev() as libc::c_int,
        );
        mm_sprintf_lite(s, b"\t\0" as *const u8 as *const libc::c_char);
        if !((*t).qual).is_null() {
            sam_write_sq(
                s,
                ((*t).qual).offset((*r).qs as isize),
                (*r).qe - (*r).qs,
                (*r).rev() as libc::c_int,
                0 as libc::c_int,
            );
        } else {
            mm_sprintf_lite(s, b"*\0" as *const u8 as *const libc::c_char);
        }
    }
    if mm_rg_id[0 as libc::c_int as usize] != 0 {
        mm_sprintf_lite(
            s,
            b"\tRG:Z:%s\0" as *const u8 as *const libc::c_char,
            mm_rg_id.as_mut_ptr(),
        );
    }
    if n_seg > 2 as libc::c_int {
        mm_sprintf_lite(s, b"\tFI:i:%d\0" as *const u8 as *const libc::c_char, seg_idx);
    }
    if !r.is_null() {
        write_tags(s, r);
        if (*r).parent == (*r).id && !((*r).p).is_null() && n_regs > 1 as libc::c_int
            && !regs.is_null() && r >= regs
            && (r.offset_from(regs) as libc::c_long) < n_regs as libc::c_long
        {
            let mut i_0: libc::c_int = 0;
            let mut n_sa: libc::c_int = 0 as libc::c_int;
            i_0 = 0 as libc::c_int;
            while i_0 < n_regs {
                if i_0 as libc::c_long != r.offset_from(regs) as libc::c_long
                    && (*regs.offset(i_0 as isize)).parent
                        == (*regs.offset(i_0 as isize)).id
                    && !((*regs.offset(i_0 as isize)).p).is_null()
                {
                    n_sa += 1;
                    n_sa;
                }
                i_0 += 1;
                i_0;
            }
            if n_sa > 0 as libc::c_int {
                mm_sprintf_lite(s, b"\tSA:Z:\0" as *const u8 as *const libc::c_char);
                i_0 = 0 as libc::c_int;
                while i_0 < n_regs {
                    let mut q: *const mm_reg1_t = &*regs.offset(i_0 as isize)
                        as *const mm_reg1_t;
                    let mut l_M: libc::c_int = 0;
                    let mut l_I: libc::c_int = 0 as libc::c_int;
                    let mut l_D: libc::c_int = 0 as libc::c_int;
                    let mut clip5: libc::c_int = 0 as libc::c_int;
                    let mut clip3: libc::c_int = 0 as libc::c_int;
                    if !(r == q || (*q).parent != (*q).id || ((*q).p).is_null()) {
                        if (*q).qe - (*q).qs < (*q).re - (*q).rs {
                            l_M = (*q).qe - (*q).qs;
                            l_D = (*q).re - (*q).rs - l_M;
                        } else {
                            l_M = (*q).re - (*q).rs;
                            l_I = (*q).qe - (*q).qs - l_M;
                        }
                        clip5 = if (*q).rev() as libc::c_int != 0 {
                            (*t).l_seq - (*q).qe
                        } else {
                            (*q).qs
                        };
                        clip3 = if (*q).rev() as libc::c_int != 0 {
                            (*q).qs
                        } else {
                            (*t).l_seq - (*q).qe
                        };
                        mm_sprintf_lite(
                            s,
                            b"%s,%d,%c,\0" as *const u8 as *const libc::c_char,
                            (*((*mi).seq).offset((*q).rid as isize)).name,
                            (*q).rs + 1 as libc::c_int,
                            (*::std::mem::transmute::<
                                &[u8; 3],
                                &[libc::c_char; 3],
                            >(b"+-\0"))[(*q).rev() as usize] as libc::c_int,
                        );
                        if clip5 != 0 {
                            mm_sprintf_lite(
                                s,
                                b"%dS\0" as *const u8 as *const libc::c_char,
                                clip5,
                            );
                        }
                        if l_M != 0 {
                            mm_sprintf_lite(
                                s,
                                b"%dM\0" as *const u8 as *const libc::c_char,
                                l_M,
                            );
                        }
                        if l_I != 0 {
                            mm_sprintf_lite(
                                s,
                                b"%dI\0" as *const u8 as *const libc::c_char,
                                l_I,
                            );
                        }
                        if l_D != 0 {
                            mm_sprintf_lite(
                                s,
                                b"%dD\0" as *const u8 as *const libc::c_char,
                                l_D,
                            );
                        }
                        if clip3 != 0 {
                            mm_sprintf_lite(
                                s,
                                b"%dS\0" as *const u8 as *const libc::c_char,
                                clip3,
                            );
                        }
                        mm_sprintf_lite(
                            s,
                            b",%d,%d;\0" as *const u8 as *const libc::c_char,
                            (*q).mapq() as libc::c_int,
                            (*q).blen - (*q).mlen + (*(*q).p).n_ambi() as libc::c_int,
                        );
                    }
                    i_0 += 1;
                    i_0;
                }
            }
        }
        if !((*r).p).is_null()
            && opt_flag
                & (0x40 as libc::c_int | 0x1000000 as libc::c_int) as libc::c_long != 0
        {
            write_cs_or_MD(
                km,
                s,
                mi,
                t,
                r,
                (opt_flag & 0x800 as libc::c_int as libc::c_long == 0) as libc::c_int,
                (opt_flag & 0x1000000 as libc::c_int as libc::c_long) as libc::c_int,
                1 as libc::c_int,
                0 as libc::c_int,
            );
        }
        if cigar_in_tag != 0 {
            write_sam_cigar(s, flag, 1 as libc::c_int, (*t).l_seq, r, opt_flag);
        }
    }
    if rep_len >= 0 as libc::c_int {
        mm_sprintf_lite(s, b"\trl:i:%d\0" as *const u8 as *const libc::c_char, rep_len);
    }
    if opt_flag & 0x2000000 as libc::c_int as libc::c_long != 0
        && !((*t).comment).is_null()
    {
        mm_sprintf_lite(s, b"\t%s\0" as *const u8 as *const libc::c_char, (*t).comment);
    }
    *((*s).s).offset((*s).l as isize) = 0 as libc::c_int as libc::c_char;
}
pub unsafe extern "C" fn mm_write_sam2(
    mut s: *mut kstring_t,
    mut mi: *const mm_idx_t,
    mut t: *const mm_bseq1_t,
    mut seg_idx: libc::c_int,
    mut reg_idx: libc::c_int,
    mut n_seg: libc::c_int,
    mut n_regss: *const libc::c_int,
    mut regss: *const *const mm_reg1_t,
    mut km: *mut libc::c_void,
    mut opt_flag: int64_t,
) {
    mm_write_sam3(
        s,
        mi,
        t,
        seg_idx,
        reg_idx,
        n_seg,
        n_regss,
        regss,
        km,
        opt_flag,
        -(1 as libc::c_int),
    );
}
pub unsafe extern "C" fn mm_write_sam(
    mut s: *mut kstring_t,
    mut mi: *const mm_idx_t,
    mut t: *const mm_bseq1_t,
    mut r: *const mm_reg1_t,
    mut n_regs: libc::c_int,
    mut regs: *const mm_reg1_t,
) {
    let mut i: libc::c_int = 0;
    i = 0 as libc::c_int;
    while i < n_regs {
        if r == &*regs.offset(i as isize) as *const mm_reg1_t {
            break;
        }
        i += 1;
        i;
    }
    mm_write_sam2(
        s,
        mi,
        t,
        0 as libc::c_int,
        i,
        1 as libc::c_int,
        &mut n_regs,
        &mut regs,
        0 as *mut libc::c_void,
        0 as libc::c_int as int64_t,
    );
}
