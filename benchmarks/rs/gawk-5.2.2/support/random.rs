use ::libc;
extern "C" {
    pub type _IO_wide_data;
    pub type _IO_codecvt;
    pub type _IO_marker;
    static mut stderr: *mut FILE;
    fn fprintf(_: *mut FILE, _: *const libc::c_char, _: ...) -> libc::c_int;
}
pub type __off_t = libc::c_long;
pub type __off64_t = libc::c_long;
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
pub type gawk_uint32_t = libc::c_uint;
pub type gawk_int32_t = libc::c_int;
static mut shuffle_init: libc::c_int = 1 as libc::c_int;
static mut shuffle_buffer: [libc::c_long; 512] = [0; 512];
static mut degrees: [libc::c_int; 5] = [
    0 as libc::c_int,
    7 as libc::c_int,
    15 as libc::c_int,
    31 as libc::c_int,
    63 as libc::c_int,
];
static mut seps: [libc::c_int; 5] = [
    0 as libc::c_int,
    3 as libc::c_int,
    1 as libc::c_int,
    3 as libc::c_int,
    1 as libc::c_int,
];
static mut randtbl: [gawk_uint32_t; 32] = [
    3 as libc::c_int as gawk_uint32_t,
    0x991539b1 as libc::c_uint,
    0x16a5bce3 as libc::c_int as gawk_uint32_t,
    0x6774a4cd as libc::c_int as gawk_uint32_t,
    0x3e01511e as libc::c_int as gawk_uint32_t,
    0x4e508aaa as libc::c_int as gawk_uint32_t,
    0x61048c05 as libc::c_int as gawk_uint32_t,
    0xf5500617 as libc::c_uint,
    0x846b7115 as libc::c_uint,
    0x6a19892c as libc::c_int as gawk_uint32_t,
    0x896a97af as libc::c_uint,
    0xdb48f936 as libc::c_uint,
    0x14898454 as libc::c_int as gawk_uint32_t,
    0x37ffd106 as libc::c_int as gawk_uint32_t,
    0xb58bff9c as libc::c_uint,
    0x59e17104 as libc::c_int as gawk_uint32_t,
    0xcf918a49 as libc::c_uint,
    0x9378c83 as libc::c_int as gawk_uint32_t,
    0x52c7a471 as libc::c_int as gawk_uint32_t,
    0x8d293ea9 as libc::c_uint,
    0x1f4fc301 as libc::c_int as gawk_uint32_t,
    0xc3db71be as libc::c_uint,
    0x39b44e1c as libc::c_int as gawk_uint32_t,
    0xf8a44ef9 as libc::c_uint,
    0x4c8b80b1 as libc::c_int as gawk_uint32_t,
    0x19edc328 as libc::c_int as gawk_uint32_t,
    0x87bf4bdd as libc::c_uint,
    0xc9b240e5 as libc::c_uint,
    0xe9ee4b1b as libc::c_uint,
    0x4382aee7 as libc::c_int as gawk_uint32_t,
    0x535b6b41 as libc::c_int as gawk_uint32_t,
    0xf3bec5da as libc::c_uint,
];
static mut fptr: *mut gawk_uint32_t = 0 as *const gawk_uint32_t as *mut gawk_uint32_t;
static mut rptr: *mut gawk_uint32_t = 0 as *const gawk_uint32_t as *mut gawk_uint32_t;
static mut state: *mut gawk_uint32_t = 0 as *const gawk_uint32_t as *mut gawk_uint32_t;
static mut rand_type: libc::c_int = 3 as libc::c_int;
static mut rand_deg: libc::c_int = 31 as libc::c_int;
static mut rand_sep: libc::c_int = 3 as libc::c_int;
static mut end_ptr: *mut gawk_uint32_t = 0 as *const gawk_uint32_t as *mut gawk_uint32_t;
#[inline]
unsafe extern "C" fn good_rand(mut x: gawk_int32_t) -> gawk_uint32_t {
    let mut hi: gawk_int32_t = 0;
    let mut lo: gawk_int32_t = 0;
    if x == 0 as libc::c_int {
        x = 123459876 as libc::c_int;
    }
    hi = x / 127773 as libc::c_int;
    lo = x % 127773 as libc::c_int;
    x = 16807 as libc::c_int * lo - 2836 as libc::c_int * hi;
    if x < 0 as libc::c_int {
        x += 0x7fffffff as libc::c_int;
    }
    return x as gawk_uint32_t;
}
pub unsafe extern "C" fn gawk_srandom(mut x: libc::c_ulong) {
    let mut i: libc::c_int = 0;
    let mut lim: libc::c_int = 0;
    shuffle_init = 1 as libc::c_int;
    *state.offset(0 as libc::c_int as isize) = x as gawk_uint32_t;
    if rand_type == 0 as libc::c_int {
        lim = 50 as libc::c_int;
    } else {
        i = 1 as libc::c_int;
        while i < rand_deg {
            *state
                .offset(
                    i as isize,
                ) = good_rand(
                *state.offset((i - 1 as libc::c_int) as isize) as gawk_int32_t,
            );
            i += 1;
            i;
        }
        fptr = &mut *state.offset(rand_sep as isize) as *mut gawk_uint32_t;
        rptr = &mut *state.offset(0 as libc::c_int as isize) as *mut gawk_uint32_t;
        lim = 10 as libc::c_int * rand_deg;
    }
    i = 0 as libc::c_int;
    while i < lim {
        gawk_random();
        i += 1;
        i;
    }
}
pub unsafe extern "C" fn gawk_initstate(
    mut seed: libc::c_ulong,
    mut arg_state: *mut libc::c_char,
    mut n: libc::c_long,
) -> *mut libc::c_char {
    let mut ostate: *mut libc::c_char = &mut *state.offset(-(1 as libc::c_int) as isize)
        as *mut gawk_uint32_t as *mut libc::c_char;
    let mut int_arg_state: *mut gawk_uint32_t = arg_state as *mut gawk_uint32_t;
    if rand_type == 0 as libc::c_int {
        *state.offset(-(1 as libc::c_int) as isize) = rand_type as gawk_uint32_t;
    } else {
        *state
            .offset(
                -(1 as libc::c_int) as isize,
            ) = (5 as libc::c_int as libc::c_long
            * rptr.offset_from(state) as libc::c_long + rand_type as libc::c_long)
            as gawk_uint32_t;
    }
    if n < 8 as libc::c_int as libc::c_long {
        fprintf(
            stderr,
            b"random: not enough state (%ld bytes); ignored.\n\0" as *const u8
                as *const libc::c_char,
            n,
        );
        return 0 as *mut libc::c_char;
    }
    if n < 32 as libc::c_int as libc::c_long {
        rand_type = 0 as libc::c_int;
        rand_deg = 0 as libc::c_int;
        rand_sep = 0 as libc::c_int;
    } else if n < 64 as libc::c_int as libc::c_long {
        rand_type = 1 as libc::c_int;
        rand_deg = 7 as libc::c_int;
        rand_sep = 3 as libc::c_int;
    } else if n < 128 as libc::c_int as libc::c_long {
        rand_type = 2 as libc::c_int;
        rand_deg = 15 as libc::c_int;
        rand_sep = 1 as libc::c_int;
    } else if n < 256 as libc::c_int as libc::c_long {
        rand_type = 3 as libc::c_int;
        rand_deg = 31 as libc::c_int;
        rand_sep = 3 as libc::c_int;
    } else {
        rand_type = 4 as libc::c_int;
        rand_deg = 63 as libc::c_int;
        rand_sep = 1 as libc::c_int;
    }
    state = int_arg_state.offset(1 as libc::c_int as isize);
    end_ptr = &mut *state.offset(rand_deg as isize) as *mut gawk_uint32_t;
    gawk_srandom(seed);
    if rand_type == 0 as libc::c_int {
        *int_arg_state.offset(0 as libc::c_int as isize) = rand_type as gawk_uint32_t;
    } else {
        *int_arg_state
            .offset(
                0 as libc::c_int as isize,
            ) = (5 as libc::c_int as libc::c_long
            * rptr.offset_from(state) as libc::c_long + rand_type as libc::c_long)
            as gawk_uint32_t;
    }
    return ostate;
}
pub unsafe extern "C" fn gawk_setstate(
    mut arg_state: *mut libc::c_char,
) -> *mut libc::c_char {
    let mut new_state: *mut gawk_uint32_t = arg_state as *mut gawk_uint32_t;
    let mut type_0: gawk_uint32_t = (*new_state.offset(0 as libc::c_int as isize))
        .wrapping_rem(5 as libc::c_int as libc::c_uint);
    let mut rear: gawk_uint32_t = (*new_state.offset(0 as libc::c_int as isize))
        .wrapping_div(5 as libc::c_int as libc::c_uint);
    let mut ostate: *mut libc::c_char = &mut *state.offset(-(1 as libc::c_int) as isize)
        as *mut gawk_uint32_t as *mut libc::c_char;
    if rand_type == 0 as libc::c_int {
        *state.offset(-(1 as libc::c_int) as isize) = rand_type as gawk_uint32_t;
    } else {
        *state
            .offset(
                -(1 as libc::c_int) as isize,
            ) = (5 as libc::c_int as libc::c_long
            * rptr.offset_from(state) as libc::c_long + rand_type as libc::c_long)
            as gawk_uint32_t;
    }
    match type_0 {
        0 | 1 | 2 | 3 | 4 => {
            rand_type = type_0 as libc::c_int;
            rand_deg = degrees[type_0 as usize];
            rand_sep = seps[type_0 as usize];
        }
        _ => {
            fprintf(
                stderr,
                b"random: state info corrupted; not changed.\n\0" as *const u8
                    as *const libc::c_char,
            );
        }
    }
    state = new_state.offset(1 as libc::c_int as isize);
    if rand_type != 0 as libc::c_int {
        rptr = &mut *state.offset(rear as isize) as *mut gawk_uint32_t;
        fptr = &mut *state
            .offset(
                rear
                    .wrapping_add(rand_sep as libc::c_uint)
                    .wrapping_rem(rand_deg as libc::c_uint) as isize,
            ) as *mut gawk_uint32_t;
    }
    end_ptr = &mut *state.offset(rand_deg as isize) as *mut gawk_uint32_t;
    return ostate;
}
unsafe extern "C" fn random_old() -> libc::c_long {
    let mut i: gawk_uint32_t = 0;
    let mut f: *mut gawk_uint32_t = 0 as *mut gawk_uint32_t;
    let mut r: *mut gawk_uint32_t = 0 as *mut gawk_uint32_t;
    if rand_type == 0 as libc::c_int {
        i = *state.offset(0 as libc::c_int as isize);
        i = good_rand(i as gawk_int32_t) & 0x7fffffff as libc::c_int as libc::c_uint;
        *state.offset(0 as libc::c_int as isize) = i;
    } else {
        f = fptr;
        r = rptr;
        *f = (*f as libc::c_uint).wrapping_add(*r) as gawk_uint32_t as gawk_uint32_t;
        i = *f >> 1 as libc::c_int & 0x7fffffff as libc::c_int as libc::c_uint;
        f = f.offset(1);
        if f >= end_ptr {
            f = state;
            r = r.offset(1);
            r;
        } else {
            r = r.offset(1);
            if r >= end_ptr {
                r = state;
            }
        }
        fptr = f;
        rptr = r;
    }
    return i as libc::c_long;
}
pub unsafe extern "C" fn gawk_random() -> libc::c_long {
    let mut k: libc::c_int = 0;
    let mut r: libc::c_long = 0;
    static mut s: libc::c_long = 0xcafefeed as libc::c_long;
    if shuffle_init != 0 {
        k = 0 as libc::c_int;
        while k < (1 as libc::c_int) << 9 as libc::c_int {
            shuffle_buffer[k as usize] = random_old();
            k += 1;
            k;
        }
        s = random_old();
        shuffle_init = 0 as libc::c_int;
    }
    r = random_old();
    k = (s
        & (((1 as libc::c_int) << 9 as libc::c_int) - 1 as libc::c_int) as libc::c_long)
        as libc::c_int;
    s = shuffle_buffer[k as usize];
    shuffle_buffer[k as usize] = r;
    return s;
}
unsafe extern "C" fn run_static_initializers() {
    fptr = &mut *randtbl
        .as_mut_ptr()
        .offset((3 as libc::c_int + 1 as libc::c_int) as isize) as *mut gawk_uint32_t;
    rptr = &mut *randtbl.as_mut_ptr().offset(1 as libc::c_int as isize)
        as *mut gawk_uint32_t;
    state = &mut *randtbl.as_mut_ptr().offset(1 as libc::c_int as isize)
        as *mut gawk_uint32_t;
    end_ptr = &mut *randtbl
        .as_mut_ptr()
        .offset((31 as libc::c_int + 1 as libc::c_int) as isize) as *mut gawk_uint32_t;
}
#[used]
#[cfg_attr(target_os = "linux", link_section = ".init_array")]
#[cfg_attr(target_os = "windows", link_section = ".CRT$XIB")]
#[cfg_attr(target_os = "macos", link_section = "__DATA,__mod_init_func")]
static INIT_ARRAY: [unsafe extern "C" fn(); 1] = [run_static_initializers];
