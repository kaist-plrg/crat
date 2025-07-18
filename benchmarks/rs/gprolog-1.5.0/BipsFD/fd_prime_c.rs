use ::libc;
extern "C" {
    fn free(__ptr: *mut libc::c_void);
    fn Pl_Malloc_Check(
        size: libc::c_uint,
        src_file: *mut libc::c_char,
        src_line: libc::c_int,
    ) -> *mut libc::c_char;
    static mut pl_vec_size: WamWord;
    static mut pl_vec_max_integer: WamWord;
    fn Pl_Vector_Next_After(vec: Vector, n: libc::c_int) -> libc::c_int;
    fn Pl_Vector_Full(vec: Vector);
    fn Pl_Range_Copy(range: *mut Range, range1: *mut Range);
}
pub type intptr_t = libc::c_long;
pub type uintptr_t = libc::c_ulong;
pub type PlLong = intptr_t;
pub type PlULong = uintptr_t;
pub type Bool = libc::c_int;
pub type WamWord = intptr_t;
pub type WamWordP = *mut WamWord;
pub type VecWord = PlULong;
pub type Vector = *mut VecWord;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct Range {
    pub extra_cstr: Bool,
    pub min: libc::c_int,
    pub max: libc::c_int,
    pub vec: Vector,
}
pub static mut pl_reg_bank: *mut WamWord = 0 as *const WamWord as *mut WamWord;
pub static mut TR: WamWordP = 0 as *const WamWord as *mut WamWord;
pub static mut B: WamWordP = 0 as *const WamWord as *mut WamWord;
pub static mut H: WamWordP = 0 as *const WamWord as *mut WamWord;
pub static mut pl_def_max_atom: PlLong = 0;
pub static mut pl_byte_code: *mut libc::c_uint = 0 as *const libc::c_uint
    as *mut libc::c_uint;
pub static mut pl_last_read_col: libc::c_int = 0;
pub static mut pl_last_read_line: libc::c_int = 0;
static mut prime_vec_size: libc::c_int = 0;
static mut prime_range: Range = Range {
    extra_cstr: 0,
    min: 0,
    max: 0,
    vec: 0 as *const VecWord as *mut VecWord,
};
static mut not_prime_range: Range = Range {
    extra_cstr: 0,
    min: 0,
    max: 0,
    vec: 0 as *const VecWord as *mut VecWord,
};
pub unsafe extern "C" fn Pl_Prime_Range(mut r: *mut Range) {
    if prime_vec_size as libc::c_long != pl_vec_size {
        Compute_Prime_Range();
    }
    Pl_Range_Copy(r, &mut prime_range);
}
pub unsafe extern "C" fn Pl_Not_Prime_Range(mut r: *mut Range) {
    if prime_vec_size as libc::c_long != pl_vec_size {
        Compute_Prime_Range();
    }
    Pl_Range_Copy(r, &mut not_prime_range);
}
unsafe extern "C" fn Compute_Prime_Range() {
    let mut i: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    let mut vec: Vector = 0 as *mut VecWord;
    let mut nvec: Vector = 0 as *mut VecWord;
    let mut end: Vector = 0 as *mut VecWord;
    if !(prime_range.vec).is_null() {
        free(prime_range.vec as *mut libc::c_void);
        free(not_prime_range.vec as *mut libc::c_void);
    }
    vec = Pl_Malloc_Check(
        (pl_vec_size as libc::c_ulong)
            .wrapping_mul(::std::mem::size_of::<VecWord>() as libc::c_ulong)
            as libc::c_uint,
        b"fd_prime_c.c\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        126 as libc::c_int,
    ) as Vector;
    prime_range.vec = vec;
    nvec = Pl_Malloc_Check(
        (pl_vec_size as libc::c_ulong)
            .wrapping_mul(::std::mem::size_of::<VecWord>() as libc::c_ulong)
            as libc::c_uint,
        b"fd_prime_c.c\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        127 as libc::c_int,
    ) as Vector;
    not_prime_range.vec = nvec;
    Pl_Vector_Full(vec);
    let ref mut fresh0 = *vec
        .offset((0 as libc::c_int as VecWord >> 6 as libc::c_int) as isize);
    *fresh0
        &= !((1 as libc::c_int as VecWord)
            << (0 as libc::c_int as libc::c_ulong
                & ((1 as libc::c_int as VecWord) << 6 as libc::c_int)
                    .wrapping_sub(1 as libc::c_int as libc::c_ulong)));
    let ref mut fresh1 = *vec
        .offset((1 as libc::c_int as VecWord >> 6 as libc::c_int) as isize);
    *fresh1
        &= !((1 as libc::c_int as VecWord)
            << (1 as libc::c_int as libc::c_ulong
                & ((1 as libc::c_int as VecWord) << 6 as libc::c_int)
                    .wrapping_sub(1 as libc::c_int as libc::c_ulong)));
    i = 2 as libc::c_int;
    loop {
        j = i;
        loop {
            j += i;
            if !(j as libc::c_long <= pl_vec_max_integer) {
                break;
            }
            let ref mut fresh2 = *vec
                .offset((j as VecWord >> 6 as libc::c_int) as isize);
            *fresh2
                &= !((1 as libc::c_int as VecWord)
                    << (j as libc::c_ulong
                        & ((1 as libc::c_int as VecWord) << 6 as libc::c_int)
                            .wrapping_sub(1 as libc::c_int as libc::c_ulong)));
        }
        j = i;
        i = Pl_Vector_Next_After(vec, i);
        if !(i > 0 as libc::c_int) {
            break;
        }
    }
    prime_range.extra_cstr = 1 as libc::c_int;
    prime_range.min = 2 as libc::c_int;
    prime_range.max = j;
    not_prime_range.extra_cstr = 1 as libc::c_int;
    not_prime_range.min = 0 as libc::c_int;
    not_prime_range
        .max = (if (j as libc::c_long) < pl_vec_max_integer {
        pl_vec_max_integer
    } else {
        pl_vec_max_integer - 1 as libc::c_int as libc::c_long
    }) as libc::c_int;
    end = vec.offset(pl_vec_size as isize);
    loop {
        *nvec = !*vec;
        vec = vec.offset(1);
        vec;
        nvec = nvec.offset(1);
        nvec;
        if !(vec < end) {
            break;
        }
    }
    prime_vec_size = pl_vec_size as libc::c_int;
}
