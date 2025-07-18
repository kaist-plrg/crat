use ::libc;
extern "C" {
    pub type _IO_wide_data;
    pub type _IO_codecvt;
    pub type _IO_marker;
    fn malloc(_: libc::c_ulong) -> *mut libc::c_void;
    fn realloc(_: *mut libc::c_void, _: libc::c_ulong) -> *mut libc::c_void;
    fn free(__ptr: *mut libc::c_void);
    fn exit(_: libc::c_int) -> !;
    fn fclose(__stream: *mut FILE) -> libc::c_int;
    fn fopen(_: *const libc::c_char, _: *const libc::c_char) -> *mut FILE;
    fn printf(_: *const libc::c_char, _: ...) -> libc::c_int;
    fn vsnprintf(
        _: *mut libc::c_char,
        _: libc::c_ulong,
        _: *const libc::c_char,
        _: ::std::ffi::VaList,
    ) -> libc::c_int;
    fn fgets(
        __s: *mut libc::c_char,
        __n: libc::c_int,
        __stream: *mut FILE,
    ) -> *mut libc::c_char;
    fn perror(__s: *const libc::c_char);
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
    fn strcmp(_: *const libc::c_char, _: *const libc::c_char) -> libc::c_int;
    fn __assert_fail(
        __assertion: *const libc::c_char,
        __file: *const libc::c_char,
        __line: libc::c_uint,
        __function: *const libc::c_char,
    ) -> !;
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
pub type size_t = libc::c_ulong;
pub type __uint8_t = libc::c_uchar;
pub type __uint32_t = libc::c_uint;
pub type __off_t = libc::c_long;
pub type __off64_t = libc::c_long;
pub type va_list = __builtin_va_list;
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
pub type ptrdiff_t = libc::c_long;
pub type uint8_t = __uint8_t;
pub type uint32_t = __uint32_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct UT_hash_bucket {
    pub hh_head: *mut UT_hash_handle,
    pub count: libc::c_uint,
    pub expand_mult: libc::c_uint,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct UT_hash_handle {
    pub tbl: *mut UT_hash_table,
    pub prev: *mut libc::c_void,
    pub next: *mut libc::c_void,
    pub hh_prev: *mut UT_hash_handle,
    pub hh_next: *mut UT_hash_handle,
    pub key: *const libc::c_void,
    pub keylen: libc::c_uint,
    pub hashv: libc::c_uint,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct UT_hash_table {
    pub buckets: *mut UT_hash_bucket,
    pub num_buckets: libc::c_uint,
    pub log2_num_buckets: libc::c_uint,
    pub num_items: libc::c_uint,
    pub tail: *mut UT_hash_handle,
    pub hho: ptrdiff_t,
    pub ideal_chain_maxlen: libc::c_uint,
    pub nonideal_items: libc::c_uint,
    pub ineff_expands: libc::c_uint,
    pub noexpand: libc::c_uint,
    pub signature: uint32_t,
    pub bloom_sig: uint32_t,
    pub bloom_bv: *mut uint8_t,
    pub bloom_nbits: uint8_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct UT_string {
    pub d: *mut libc::c_char,
    pub n: size_t,
    pub i: size_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct example_user_t {
    pub id: libc::c_int,
    pub cookie: libc::c_int,
    pub hh: UT_hash_handle,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct el {
    pub bname: [libc::c_char; 20],
    pub next: *mut el,
    pub prev: *mut el,
}
unsafe extern "C" fn utstring_printf_va(
    mut s: *mut UT_string,
    mut fmt: *const libc::c_char,
    mut ap: ::std::ffi::VaList,
) {
    let mut n: libc::c_int = 0;
    let mut cp: ::std::ffi::VaListImpl;
    loop {
        cp = ap.clone();
        n = vsnprintf(
            &mut *((*s).d).offset((*s).i as isize),
            ((*s).n).wrapping_sub((*s).i),
            fmt,
            cp.as_va_list(),
        );
        if n > -(1 as libc::c_int) && (n as size_t) < ((*s).n).wrapping_sub((*s).i) {
            (*s)
                .i = ((*s).i as libc::c_ulong).wrapping_add(n as libc::c_ulong) as size_t
                as size_t;
            return;
        }
        if n > -(1 as libc::c_int) {
            if ((*s).n).wrapping_sub((*s).i) < (n + 1 as libc::c_int) as size_t {
                let mut utstring_tmp: *mut libc::c_char = realloc(
                    (*s).d as *mut libc::c_void,
                    ((*s).n).wrapping_add((n + 1 as libc::c_int) as libc::c_ulong),
                ) as *mut libc::c_char;
                if utstring_tmp.is_null() {
                    exit(-(1 as libc::c_int));
                }
                (*s).d = utstring_tmp;
                (*s)
                    .n = ((*s).n as libc::c_ulong)
                    .wrapping_add((n + 1 as libc::c_int) as libc::c_ulong) as size_t
                    as size_t;
            }
        } else if ((*s).n).wrapping_sub((*s).i)
            < ((*s).n).wrapping_mul(2 as libc::c_int as libc::c_ulong)
        {
            let mut utstring_tmp_0: *mut libc::c_char = realloc(
                (*s).d as *mut libc::c_void,
                ((*s).n)
                    .wrapping_add(
                        ((*s).n).wrapping_mul(2 as libc::c_int as libc::c_ulong),
                    ),
            ) as *mut libc::c_char;
            if utstring_tmp_0.is_null() {
                exit(-(1 as libc::c_int));
            }
            (*s).d = utstring_tmp_0;
            (*s)
                .n = ((*s).n as libc::c_ulong)
                .wrapping_add(((*s).n).wrapping_mul(2 as libc::c_int as libc::c_ulong))
                as size_t as size_t;
        }
    };
}
unsafe extern "C" fn utstring_printf(
    mut s: *mut UT_string,
    mut fmt: *const libc::c_char,
    mut args: ...
) {
    let mut ap: ::std::ffi::VaListImpl;
    ap = args.clone();
    utstring_printf_va(s, fmt, ap.as_va_list());
}
unsafe extern "C" fn namecmp(
    mut _a: *mut libc::c_void,
    mut _b: *mut libc::c_void,
) -> libc::c_int {
    let mut a: *mut el = _a as *mut el;
    let mut b: *mut el = _b as *mut el;
    return strcmp(((*a).bname).as_mut_ptr(), ((*b).bname).as_mut_ptr());
}
unsafe fn main_0() -> libc::c_int {
    let mut name: *mut el = 0 as *mut el;
    let mut elt: *mut el = 0 as *mut el;
    let mut tmp: *mut el = 0 as *mut el;
    let mut etmp: el = el {
        bname: [0; 20],
        next: 0 as *mut el,
        prev: 0 as *mut el,
    };
    let mut i: libc::c_int = 0;
    let mut user: *mut example_user_t = 0 as *mut example_user_t;
    let mut users: *mut example_user_t = 0 as *mut example_user_t;
    let mut head: *mut el = 0 as *mut el;
    let mut linebuf: [libc::c_char; 20] = [0; 20];
    let mut file: *mut FILE = 0 as *mut FILE;
    let mut s: *mut UT_string = 0 as *mut UT_string;
    let mut binary: [libc::c_char; 3] = *::std::mem::transmute::<
        &[u8; 3],
        &mut [libc::c_char; 3],
    >(b"\xFF\xFF\0");
    file = fopen(
        b"test11.dat\0" as *const u8 as *const libc::c_char,
        b"r\0" as *const u8 as *const libc::c_char,
    );
    if file.is_null() {
        perror(b"can't open: \0" as *const u8 as *const libc::c_char);
        exit(-(1 as libc::c_int));
    }
    while !(fgets(linebuf.as_mut_ptr(), 20 as libc::c_int, file)).is_null() {
        name = malloc(::std::mem::size_of::<el>() as libc::c_ulong) as *mut el;
        if name.is_null() {
            exit(-(1 as libc::c_int));
        }
        strcpy(((*name).bname).as_mut_ptr(), linebuf.as_mut_ptr());
        if !head.is_null() {
            (*name).prev = (*head).prev;
            (*(*head).prev).next = name;
            (*head).prev = name;
            (*name).next = 0 as *mut el;
        } else {
            head = name;
            (*head).prev = head;
            (*head).next = 0 as *mut el;
        }
    }
    let mut _ls_p: *mut el = 0 as *mut el;
    let mut _ls_q: *mut el = 0 as *mut el;
    let mut _ls_e: *mut el = 0 as *mut el;
    let mut _ls_tail: *mut el = 0 as *mut el;
    let mut _ls_insize: libc::c_int = 0;
    let mut _ls_nmerges: libc::c_int = 0;
    let mut _ls_psize: libc::c_int = 0;
    let mut _ls_qsize: libc::c_int = 0;
    let mut _ls_i: libc::c_int = 0;
    let mut _ls_looping: libc::c_int = 0;
    if !head.is_null() {
        _ls_insize = 1 as libc::c_int;
        _ls_looping = 1 as libc::c_int;
        while _ls_looping != 0 {
            _ls_p = head;
            head = 0 as *mut el;
            _ls_tail = 0 as *mut el;
            _ls_nmerges = 0 as libc::c_int;
            while !_ls_p.is_null() {
                _ls_nmerges += 1;
                _ls_nmerges;
                _ls_q = _ls_p;
                _ls_psize = 0 as libc::c_int;
                _ls_i = 0 as libc::c_int;
                while _ls_i < _ls_insize {
                    _ls_psize += 1;
                    _ls_psize;
                    _ls_q = (*_ls_q).next;
                    if _ls_q.is_null() {
                        break;
                    }
                    _ls_i += 1;
                    _ls_i;
                }
                _ls_qsize = _ls_insize;
                while _ls_psize > 0 as libc::c_int
                    || _ls_qsize > 0 as libc::c_int && !_ls_q.is_null()
                {
                    if _ls_psize == 0 as libc::c_int {
                        _ls_e = _ls_q;
                        _ls_q = (*_ls_q).next;
                        _ls_qsize -= 1;
                        _ls_qsize;
                    } else if _ls_qsize == 0 as libc::c_int || _ls_q.is_null() {
                        _ls_e = _ls_p;
                        _ls_p = (*_ls_p).next;
                        _ls_psize -= 1;
                        _ls_psize;
                    } else if namecmp(
                        _ls_p as *mut libc::c_void,
                        _ls_q as *mut libc::c_void,
                    ) <= 0 as libc::c_int
                    {
                        _ls_e = _ls_p;
                        _ls_p = (*_ls_p).next;
                        _ls_psize -= 1;
                        _ls_psize;
                    } else {
                        _ls_e = _ls_q;
                        _ls_q = (*_ls_q).next;
                        _ls_qsize -= 1;
                        _ls_qsize;
                    }
                    if !_ls_tail.is_null() {
                        (*_ls_tail).next = _ls_e;
                    } else {
                        head = _ls_e;
                    }
                    (*_ls_e).prev = _ls_tail;
                    _ls_tail = _ls_e;
                }
                _ls_p = _ls_q;
            }
            (*head).prev = _ls_tail;
            (*_ls_tail).next = 0 as *mut el;
            if _ls_nmerges <= 1 as libc::c_int {
                _ls_looping = 0 as libc::c_int;
            }
            _ls_insize *= 2 as libc::c_int;
        }
    }
    elt = head;
    while !elt.is_null() {
        printf(b"%s\0" as *const u8 as *const libc::c_char, ((*elt).bname).as_mut_ptr());
        elt = (*elt).next;
    }
    memcpy(
        (etmp.bname).as_mut_ptr() as *mut libc::c_void,
        b"WES\n\0" as *const u8 as *const libc::c_char as *const libc::c_void,
        5 as libc::c_ulong,
    );
    elt = head;
    while !elt.is_null() {
        if namecmp(elt as *mut libc::c_void, &mut etmp as *mut el as *mut libc::c_void)
            == 0 as libc::c_int
        {
            break;
        }
        elt = (*elt).next;
    }
    if !elt.is_null() {
        printf(
            b"found %s\n\0" as *const u8 as *const libc::c_char,
            ((*elt).bname).as_mut_ptr(),
        );
    }
    elt = head;
    while !elt.is_null()
        && {
            tmp = (*elt).next;
            1 as libc::c_int != 0
        }
    {
        if !head.is_null() {} else {
            __assert_fail(
                b"(head) != NULL\0" as *const u8 as *const libc::c_char,
                b"test56.c\0" as *const u8 as *const libc::c_char,
                68 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 11],
                    &[libc::c_char; 11],
                >(b"int main()\0"))
                    .as_ptr(),
            );
        }
        'c_5372: {
            if !head.is_null() {} else {
                __assert_fail(
                    b"(head) != NULL\0" as *const u8 as *const libc::c_char,
                    b"test56.c\0" as *const u8 as *const libc::c_char,
                    68 as libc::c_int as libc::c_uint,
                    (*::std::mem::transmute::<
                        &[u8; 11],
                        &[libc::c_char; 11],
                    >(b"int main()\0"))
                        .as_ptr(),
                );
            }
        };
        if !((*elt).prev).is_null() {} else {
            __assert_fail(
                b"(elt)->prev != NULL\0" as *const u8 as *const libc::c_char,
                b"test56.c\0" as *const u8 as *const libc::c_char,
                68 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 11],
                    &[libc::c_char; 11],
                >(b"int main()\0"))
                    .as_ptr(),
            );
        }
        'c_5320: {
            if !((*elt).prev).is_null() {} else {
                __assert_fail(
                    b"(elt)->prev != NULL\0" as *const u8 as *const libc::c_char,
                    b"test56.c\0" as *const u8 as *const libc::c_char,
                    68 as libc::c_int as libc::c_uint,
                    (*::std::mem::transmute::<
                        &[u8; 11],
                        &[libc::c_char; 11],
                    >(b"int main()\0"))
                        .as_ptr(),
                );
            }
        };
        if (*elt).prev == elt {
            head = 0 as *mut el;
        } else if elt == head {
            (*(*elt).next).prev = (*elt).prev;
            head = (*elt).next;
        } else {
            (*(*elt).prev).next = (*elt).next;
            if !((*elt).next).is_null() {
                (*(*elt).next).prev = (*elt).prev;
            } else {
                (*head).prev = (*elt).prev;
            }
        }
        elt = tmp;
    }
    fclose(file);
    i = 0 as libc::c_int;
    while i < 10 as libc::c_int {
        user = malloc(::std::mem::size_of::<example_user_t>() as libc::c_ulong)
            as *mut example_user_t;
        if user.is_null() {
            exit(-(1 as libc::c_int));
        }
        (*user).id = i;
        (*user).cookie = i * i;
        let mut _ha_hashv: libc::c_uint = 0;
        let mut _hj_i: libc::c_uint = 0;
        let mut _hj_j: libc::c_uint = 0;
        let mut _hj_k: libc::c_uint = 0;
        let mut _hj_key: *const libc::c_uchar = &mut (*user).id as *mut libc::c_int
            as *const libc::c_uchar;
        _ha_hashv = 0xfeedbeef as libc::c_uint;
        _hj_j = 0x9e3779b9 as libc::c_uint;
        _hj_i = _hj_j;
        _hj_k = ::std::mem::size_of::<libc::c_int>() as libc::c_ulong as libc::c_uint;
        while _hj_k >= 12 as libc::c_uint {
            _hj_i = _hj_i
                .wrapping_add(
                    (*_hj_key.offset(0 as libc::c_int as isize) as libc::c_uint)
                        .wrapping_add(
                            (*_hj_key.offset(1 as libc::c_int as isize) as libc::c_uint)
                                << 8 as libc::c_int,
                        )
                        .wrapping_add(
                            (*_hj_key.offset(2 as libc::c_int as isize) as libc::c_uint)
                                << 16 as libc::c_int,
                        )
                        .wrapping_add(
                            (*_hj_key.offset(3 as libc::c_int as isize) as libc::c_uint)
                                << 24 as libc::c_int,
                        ),
                );
            _hj_j = _hj_j
                .wrapping_add(
                    (*_hj_key.offset(4 as libc::c_int as isize) as libc::c_uint)
                        .wrapping_add(
                            (*_hj_key.offset(5 as libc::c_int as isize) as libc::c_uint)
                                << 8 as libc::c_int,
                        )
                        .wrapping_add(
                            (*_hj_key.offset(6 as libc::c_int as isize) as libc::c_uint)
                                << 16 as libc::c_int,
                        )
                        .wrapping_add(
                            (*_hj_key.offset(7 as libc::c_int as isize) as libc::c_uint)
                                << 24 as libc::c_int,
                        ),
                );
            _ha_hashv = _ha_hashv
                .wrapping_add(
                    (*_hj_key.offset(8 as libc::c_int as isize) as libc::c_uint)
                        .wrapping_add(
                            (*_hj_key.offset(9 as libc::c_int as isize) as libc::c_uint)
                                << 8 as libc::c_int,
                        )
                        .wrapping_add(
                            (*_hj_key.offset(10 as libc::c_int as isize) as libc::c_uint)
                                << 16 as libc::c_int,
                        )
                        .wrapping_add(
                            (*_hj_key.offset(11 as libc::c_int as isize) as libc::c_uint)
                                << 24 as libc::c_int,
                        ),
                );
            _hj_i = _hj_i.wrapping_sub(_hj_j);
            _hj_i = _hj_i.wrapping_sub(_ha_hashv);
            _hj_i ^= _ha_hashv >> 13 as libc::c_int;
            _hj_j = _hj_j.wrapping_sub(_ha_hashv);
            _hj_j = _hj_j.wrapping_sub(_hj_i);
            _hj_j ^= _hj_i << 8 as libc::c_int;
            _ha_hashv = _ha_hashv.wrapping_sub(_hj_i);
            _ha_hashv = _ha_hashv.wrapping_sub(_hj_j);
            _ha_hashv ^= _hj_j >> 13 as libc::c_int;
            _hj_i = _hj_i.wrapping_sub(_hj_j);
            _hj_i = _hj_i.wrapping_sub(_ha_hashv);
            _hj_i ^= _ha_hashv >> 12 as libc::c_int;
            _hj_j = _hj_j.wrapping_sub(_ha_hashv);
            _hj_j = _hj_j.wrapping_sub(_hj_i);
            _hj_j ^= _hj_i << 16 as libc::c_int;
            _ha_hashv = _ha_hashv.wrapping_sub(_hj_i);
            _ha_hashv = _ha_hashv.wrapping_sub(_hj_j);
            _ha_hashv ^= _hj_j >> 5 as libc::c_int;
            _hj_i = _hj_i.wrapping_sub(_hj_j);
            _hj_i = _hj_i.wrapping_sub(_ha_hashv);
            _hj_i ^= _ha_hashv >> 3 as libc::c_int;
            _hj_j = _hj_j.wrapping_sub(_ha_hashv);
            _hj_j = _hj_j.wrapping_sub(_hj_i);
            _hj_j ^= _hj_i << 10 as libc::c_int;
            _ha_hashv = _ha_hashv.wrapping_sub(_hj_i);
            _ha_hashv = _ha_hashv.wrapping_sub(_hj_j);
            _ha_hashv ^= _hj_j >> 15 as libc::c_int;
            _hj_key = _hj_key.offset(12 as libc::c_int as isize);
            _hj_k = _hj_k.wrapping_sub(12 as libc::c_uint);
        }
        _ha_hashv = _ha_hashv
            .wrapping_add(
                ::std::mem::size_of::<libc::c_int>() as libc::c_ulong as libc::c_uint,
            );
        let mut current_block_189: u64;
        match _hj_k {
            11 => {
                _ha_hashv = _ha_hashv
                    .wrapping_add(
                        (*_hj_key.offset(10 as libc::c_int as isize) as libc::c_uint)
                            << 24 as libc::c_int,
                    );
                current_block_189 = 7946502847372478677;
            }
            10 => {
                current_block_189 = 7946502847372478677;
            }
            9 => {
                current_block_189 = 6841610602099657028;
            }
            8 => {
                current_block_189 = 6165142913562364032;
            }
            7 => {
                current_block_189 = 9781088206068579112;
            }
            6 => {
                current_block_189 = 6994972524166957283;
            }
            5 => {
                current_block_189 = 15043556930347843439;
            }
            4 => {
                current_block_189 = 8533902085134371033;
            }
            3 => {
                current_block_189 = 8982081673687713566;
            }
            2 => {
                current_block_189 = 13099938943924290718;
            }
            1 => {
                current_block_189 = 11768068721683293410;
            }
            _ => {
                current_block_189 = 10784681114964964746;
            }
        }
        match current_block_189 {
            7946502847372478677 => {
                _ha_hashv = _ha_hashv
                    .wrapping_add(
                        (*_hj_key.offset(9 as libc::c_int as isize) as libc::c_uint)
                            << 16 as libc::c_int,
                    );
                current_block_189 = 6841610602099657028;
            }
            _ => {}
        }
        match current_block_189 {
            6841610602099657028 => {
                _ha_hashv = _ha_hashv
                    .wrapping_add(
                        (*_hj_key.offset(8 as libc::c_int as isize) as libc::c_uint)
                            << 8 as libc::c_int,
                    );
                current_block_189 = 6165142913562364032;
            }
            _ => {}
        }
        match current_block_189 {
            6165142913562364032 => {
                _hj_j = _hj_j
                    .wrapping_add(
                        (*_hj_key.offset(7 as libc::c_int as isize) as libc::c_uint)
                            << 24 as libc::c_int,
                    );
                current_block_189 = 9781088206068579112;
            }
            _ => {}
        }
        match current_block_189 {
            9781088206068579112 => {
                _hj_j = _hj_j
                    .wrapping_add(
                        (*_hj_key.offset(6 as libc::c_int as isize) as libc::c_uint)
                            << 16 as libc::c_int,
                    );
                current_block_189 = 6994972524166957283;
            }
            _ => {}
        }
        match current_block_189 {
            6994972524166957283 => {
                _hj_j = _hj_j
                    .wrapping_add(
                        (*_hj_key.offset(5 as libc::c_int as isize) as libc::c_uint)
                            << 8 as libc::c_int,
                    );
                current_block_189 = 15043556930347843439;
            }
            _ => {}
        }
        match current_block_189 {
            15043556930347843439 => {
                _hj_j = _hj_j
                    .wrapping_add(
                        *_hj_key.offset(4 as libc::c_int as isize) as libc::c_uint,
                    );
                current_block_189 = 8533902085134371033;
            }
            _ => {}
        }
        match current_block_189 {
            8533902085134371033 => {
                _hj_i = _hj_i
                    .wrapping_add(
                        (*_hj_key.offset(3 as libc::c_int as isize) as libc::c_uint)
                            << 24 as libc::c_int,
                    );
                current_block_189 = 8982081673687713566;
            }
            _ => {}
        }
        match current_block_189 {
            8982081673687713566 => {
                _hj_i = _hj_i
                    .wrapping_add(
                        (*_hj_key.offset(2 as libc::c_int as isize) as libc::c_uint)
                            << 16 as libc::c_int,
                    );
                current_block_189 = 13099938943924290718;
            }
            _ => {}
        }
        match current_block_189 {
            13099938943924290718 => {
                _hj_i = _hj_i
                    .wrapping_add(
                        (*_hj_key.offset(1 as libc::c_int as isize) as libc::c_uint)
                            << 8 as libc::c_int,
                    );
                current_block_189 = 11768068721683293410;
            }
            _ => {}
        }
        match current_block_189 {
            11768068721683293410 => {
                _hj_i = _hj_i
                    .wrapping_add(
                        *_hj_key.offset(0 as libc::c_int as isize) as libc::c_uint,
                    );
            }
            _ => {}
        }
        _hj_i = _hj_i.wrapping_sub(_hj_j);
        _hj_i = _hj_i.wrapping_sub(_ha_hashv);
        _hj_i ^= _ha_hashv >> 13 as libc::c_int;
        _hj_j = _hj_j.wrapping_sub(_ha_hashv);
        _hj_j = _hj_j.wrapping_sub(_hj_i);
        _hj_j ^= _hj_i << 8 as libc::c_int;
        _ha_hashv = _ha_hashv.wrapping_sub(_hj_i);
        _ha_hashv = _ha_hashv.wrapping_sub(_hj_j);
        _ha_hashv ^= _hj_j >> 13 as libc::c_int;
        _hj_i = _hj_i.wrapping_sub(_hj_j);
        _hj_i = _hj_i.wrapping_sub(_ha_hashv);
        _hj_i ^= _ha_hashv >> 12 as libc::c_int;
        _hj_j = _hj_j.wrapping_sub(_ha_hashv);
        _hj_j = _hj_j.wrapping_sub(_hj_i);
        _hj_j ^= _hj_i << 16 as libc::c_int;
        _ha_hashv = _ha_hashv.wrapping_sub(_hj_i);
        _ha_hashv = _ha_hashv.wrapping_sub(_hj_j);
        _ha_hashv ^= _hj_j >> 5 as libc::c_int;
        _hj_i = _hj_i.wrapping_sub(_hj_j);
        _hj_i = _hj_i.wrapping_sub(_ha_hashv);
        _hj_i ^= _ha_hashv >> 3 as libc::c_int;
        _hj_j = _hj_j.wrapping_sub(_ha_hashv);
        _hj_j = _hj_j.wrapping_sub(_hj_i);
        _hj_j ^= _hj_i << 10 as libc::c_int;
        _ha_hashv = _ha_hashv.wrapping_sub(_hj_i);
        _ha_hashv = _ha_hashv.wrapping_sub(_hj_j);
        _ha_hashv ^= _hj_j >> 15 as libc::c_int;
        (*user).hh.hashv = _ha_hashv;
        (*user).hh.key = &mut (*user).id as *mut libc::c_int as *const libc::c_void;
        (*user)
            .hh
            .keylen = ::std::mem::size_of::<libc::c_int>() as libc::c_ulong
            as libc::c_uint;
        if users.is_null() {
            (*user).hh.next = 0 as *mut libc::c_void;
            (*user).hh.prev = 0 as *mut libc::c_void;
            (*user)
                .hh
                .tbl = malloc(::std::mem::size_of::<UT_hash_table>() as libc::c_ulong)
                as *mut UT_hash_table;
            if ((*user).hh.tbl).is_null() {
                exit(-(1 as libc::c_int));
            } else {
                memset(
                    (*user).hh.tbl as *mut libc::c_void,
                    '\0' as i32,
                    ::std::mem::size_of::<UT_hash_table>() as libc::c_ulong,
                );
                (*(*user).hh.tbl).tail = &mut (*user).hh;
                (*(*user).hh.tbl).num_buckets = 32 as libc::c_uint;
                (*(*user).hh.tbl).log2_num_buckets = 5 as libc::c_uint;
                (*(*user).hh.tbl)
                    .hho = (&mut (*user).hh as *mut UT_hash_handle as *mut libc::c_char)
                    .offset_from(user as *mut libc::c_char) as libc::c_long;
                (*(*user).hh.tbl)
                    .buckets = malloc(
                    (32 as libc::c_uint as libc::c_ulong)
                        .wrapping_mul(
                            ::std::mem::size_of::<UT_hash_bucket>() as libc::c_ulong,
                        ),
                ) as *mut UT_hash_bucket;
                (*(*user).hh.tbl).signature = 0xa0111fe1 as libc::c_uint;
                if ((*(*user).hh.tbl).buckets).is_null() {
                    exit(-(1 as libc::c_int));
                } else {
                    memset(
                        (*(*user).hh.tbl).buckets as *mut libc::c_void,
                        '\0' as i32,
                        (32 as libc::c_uint as libc::c_ulong)
                            .wrapping_mul(
                                ::std::mem::size_of::<UT_hash_bucket>() as libc::c_ulong,
                            ),
                    );
                }
            }
            users = user;
        } else {
            (*user).hh.tbl = (*users).hh.tbl;
            (*user).hh.next = 0 as *mut libc::c_void;
            (*user)
                .hh
                .prev = ((*(*users).hh.tbl).tail as *mut libc::c_char)
                .offset(-((*(*users).hh.tbl).hho as isize)) as *mut libc::c_void;
            (*(*(*users).hh.tbl).tail).next = user as *mut libc::c_void;
            (*(*users).hh.tbl).tail = &mut (*user).hh;
        }
        let mut _ha_bkt: libc::c_uint = 0;
        (*(*users).hh.tbl).num_items = ((*(*users).hh.tbl).num_items).wrapping_add(1);
        (*(*users).hh.tbl).num_items;
        _ha_bkt = _ha_hashv
            & ((*(*users).hh.tbl).num_buckets).wrapping_sub(1 as libc::c_uint);
        let mut _ha_head: *mut UT_hash_bucket = &mut *((*(*users).hh.tbl).buckets)
            .offset(_ha_bkt as isize) as *mut UT_hash_bucket;
        (*_ha_head).count = ((*_ha_head).count).wrapping_add(1);
        (*_ha_head).count;
        (*user).hh.hh_next = (*_ha_head).hh_head;
        (*user).hh.hh_prev = 0 as *mut UT_hash_handle;
        if !((*_ha_head).hh_head).is_null() {
            (*(*_ha_head).hh_head).hh_prev = &mut (*user).hh;
        }
        (*_ha_head).hh_head = &mut (*user).hh;
        if (*_ha_head).count
            >= ((*_ha_head).expand_mult)
                .wrapping_add(1 as libc::c_uint)
                .wrapping_mul(10 as libc::c_uint) && (*(*user).hh.tbl).noexpand == 0
        {
            let mut _he_bkt: libc::c_uint = 0;
            let mut _he_bkt_i: libc::c_uint = 0;
            let mut _he_thh: *mut UT_hash_handle = 0 as *mut UT_hash_handle;
            let mut _he_hh_nxt: *mut UT_hash_handle = 0 as *mut UT_hash_handle;
            let mut _he_new_buckets: *mut UT_hash_bucket = 0 as *mut UT_hash_bucket;
            let mut _he_newbkt: *mut UT_hash_bucket = 0 as *mut UT_hash_bucket;
            _he_new_buckets = malloc(
                (::std::mem::size_of::<UT_hash_bucket>() as libc::c_ulong)
                    .wrapping_mul((*(*user).hh.tbl).num_buckets as libc::c_ulong)
                    .wrapping_mul(2 as libc::c_uint as libc::c_ulong),
            ) as *mut UT_hash_bucket;
            if _he_new_buckets.is_null() {
                exit(-(1 as libc::c_int));
            } else {
                memset(
                    _he_new_buckets as *mut libc::c_void,
                    '\0' as i32,
                    (::std::mem::size_of::<UT_hash_bucket>() as libc::c_ulong)
                        .wrapping_mul((*(*user).hh.tbl).num_buckets as libc::c_ulong)
                        .wrapping_mul(2 as libc::c_uint as libc::c_ulong),
                );
                (*(*user).hh.tbl)
                    .ideal_chain_maxlen = ((*(*user).hh.tbl).num_items
                    >> ((*(*user).hh.tbl).log2_num_buckets)
                        .wrapping_add(1 as libc::c_uint))
                    .wrapping_add(
                        (if (*(*user).hh.tbl).num_items
                            & ((*(*user).hh.tbl).num_buckets)
                                .wrapping_mul(2 as libc::c_uint)
                                .wrapping_sub(1 as libc::c_uint) != 0 as libc::c_uint
                        {
                            1 as libc::c_uint
                        } else {
                            0 as libc::c_uint
                        }),
                    );
                (*(*user).hh.tbl).nonideal_items = 0 as libc::c_int as libc::c_uint;
                _he_bkt_i = 0 as libc::c_int as libc::c_uint;
                while _he_bkt_i < (*(*user).hh.tbl).num_buckets {
                    _he_thh = (*((*(*user).hh.tbl).buckets).offset(_he_bkt_i as isize))
                        .hh_head;
                    while !_he_thh.is_null() {
                        _he_hh_nxt = (*_he_thh).hh_next;
                        _he_bkt = (*_he_thh).hashv
                            & ((*(*user).hh.tbl).num_buckets)
                                .wrapping_mul(2 as libc::c_uint)
                                .wrapping_sub(1 as libc::c_uint);
                        _he_newbkt = &mut *_he_new_buckets.offset(_he_bkt as isize)
                            as *mut UT_hash_bucket;
                        (*_he_newbkt).count = ((*_he_newbkt).count).wrapping_add(1);
                        if (*_he_newbkt).count > (*(*user).hh.tbl).ideal_chain_maxlen {
                            (*(*user).hh.tbl)
                                .nonideal_items = ((*(*user).hh.tbl).nonideal_items)
                                .wrapping_add(1);
                            (*(*user).hh.tbl).nonideal_items;
                            if (*_he_newbkt).count
                                > ((*_he_newbkt).expand_mult)
                                    .wrapping_mul((*(*user).hh.tbl).ideal_chain_maxlen)
                            {
                                (*_he_newbkt)
                                    .expand_mult = ((*_he_newbkt).expand_mult).wrapping_add(1);
                                (*_he_newbkt).expand_mult;
                            }
                        }
                        (*_he_thh).hh_prev = 0 as *mut UT_hash_handle;
                        (*_he_thh).hh_next = (*_he_newbkt).hh_head;
                        if !((*_he_newbkt).hh_head).is_null() {
                            (*(*_he_newbkt).hh_head).hh_prev = _he_thh;
                        }
                        (*_he_newbkt).hh_head = _he_thh;
                        _he_thh = _he_hh_nxt;
                    }
                    _he_bkt_i = _he_bkt_i.wrapping_add(1);
                    _he_bkt_i;
                }
                free((*(*user).hh.tbl).buckets as *mut libc::c_void);
                (*(*user).hh.tbl)
                    .num_buckets = ((*(*user).hh.tbl).num_buckets)
                    .wrapping_mul(2 as libc::c_uint);
                (*(*user).hh.tbl)
                    .log2_num_buckets = ((*(*user).hh.tbl).log2_num_buckets)
                    .wrapping_add(1);
                (*(*user).hh.tbl).log2_num_buckets;
                (*(*user).hh.tbl).buckets = _he_new_buckets;
                (*(*user).hh.tbl)
                    .ineff_expands = if (*(*user).hh.tbl).nonideal_items
                    > (*(*user).hh.tbl).num_items >> 1 as libc::c_int
                {
                    ((*(*user).hh.tbl).ineff_expands).wrapping_add(1 as libc::c_uint)
                } else {
                    0 as libc::c_uint
                };
                if (*(*user).hh.tbl).ineff_expands > 1 as libc::c_uint {
                    (*(*user).hh.tbl).noexpand = 1 as libc::c_int as libc::c_uint;
                }
            }
        }
        i += 1;
        i;
    }
    user = users;
    while !user.is_null() {
        printf(
            b"user %d, cookie %d\n\0" as *const u8 as *const libc::c_char,
            (*user).id,
            (*user).cookie,
        );
        user = (*user).hh.next as *mut example_user_t;
    }
    s = malloc(::std::mem::size_of::<UT_string>() as libc::c_ulong) as *mut UT_string;
    if s.is_null() {
        exit(-(1 as libc::c_int));
    }
    (*s).n = 0 as libc::c_int as size_t;
    (*s).i = 0 as libc::c_int as size_t;
    (*s).d = 0 as *mut libc::c_char;
    if ((*s).n).wrapping_sub((*s).i) < 100 as libc::c_int as size_t {
        let mut utstring_tmp: *mut libc::c_char = realloc(
            (*s).d as *mut libc::c_void,
            ((*s).n).wrapping_add(100 as libc::c_int as libc::c_ulong),
        ) as *mut libc::c_char;
        if utstring_tmp.is_null() {
            exit(-(1 as libc::c_int));
        }
        (*s).d = utstring_tmp;
        (*s)
            .n = ((*s).n as libc::c_ulong)
            .wrapping_add(100 as libc::c_int as libc::c_ulong) as size_t as size_t;
    }
    *((*s).d).offset(0 as libc::c_int as isize) = '\0' as i32 as libc::c_char;
    if ((*s).n).wrapping_sub((*s).i)
        < (::std::mem::size_of::<[libc::c_char; 3]>() as libc::c_ulong)
            .wrapping_add(1 as libc::c_int as libc::c_ulong)
    {
        let mut utstring_tmp_0: *mut libc::c_char = realloc(
            (*s).d as *mut libc::c_void,
            ((*s).n)
                .wrapping_add(
                    (::std::mem::size_of::<[libc::c_char; 3]>() as libc::c_ulong)
                        .wrapping_add(1 as libc::c_int as libc::c_ulong),
                ),
        ) as *mut libc::c_char;
        if utstring_tmp_0.is_null() {
            exit(-(1 as libc::c_int));
        }
        (*s).d = utstring_tmp_0;
        (*s)
            .n = ((*s).n as libc::c_ulong)
            .wrapping_add(
                (::std::mem::size_of::<[libc::c_char; 3]>() as libc::c_ulong)
                    .wrapping_add(1 as libc::c_int as libc::c_ulong),
            ) as size_t as size_t;
    }
    if ::std::mem::size_of::<[libc::c_char; 3]>() as libc::c_ulong != 0 {
        memcpy(
            &mut *((*s).d).offset((*s).i as isize) as *mut libc::c_char
                as *mut libc::c_void,
            binary.as_mut_ptr() as *const libc::c_void,
            ::std::mem::size_of::<[libc::c_char; 3]>() as libc::c_ulong,
        );
    }
    (*s)
        .i = ((*s).i as libc::c_ulong)
        .wrapping_add(::std::mem::size_of::<[libc::c_char; 3]>() as libc::c_ulong)
        as size_t as size_t;
    *((*s).d).offset((*s).i as isize) = '\0' as i32 as libc::c_char;
    printf(
        b"length is %u\n\0" as *const u8 as *const libc::c_char,
        (*s).i as libc::c_uint,
    );
    (*s).i = 0 as libc::c_int as size_t;
    *((*s).d).offset(0 as libc::c_int as isize) = '\0' as i32 as libc::c_char;
    utstring_printf(
        s,
        b"number %d\0" as *const u8 as *const libc::c_char,
        10 as libc::c_int,
    );
    printf(b"%s\n\0" as *const u8 as *const libc::c_char, (*s).d);
    if !((*s).d).is_null() {
        free((*s).d as *mut libc::c_void);
    }
    (*s).n = 0 as libc::c_int as size_t;
    free(s as *mut libc::c_void);
    return 0 as libc::c_int;
}
pub fn main() {
    unsafe { ::std::process::exit(main_0() as i32) }
}
