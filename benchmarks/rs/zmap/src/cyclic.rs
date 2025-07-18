use ::libc;
extern "C" {
    pub type aesrand;
    fn aesrand_getword(aes: *mut aesrand_t) -> uint64_t;
    fn __assert_fail(
        __assertion: *const libc::c_char,
        __file: *const libc::c_char,
        __line: libc::c_uint,
        __function: *const libc::c_char,
    ) -> !;
    fn __gmpz_clear(_: mpz_ptr);
    fn __gmpz_init(_: mpz_ptr);
    fn __gmpz_init_set_ui(_: mpz_ptr, _: libc::c_ulong);
    fn __gmpz_powm(_: mpz_ptr, _: mpz_srcptr, _: mpz_srcptr, _: mpz_srcptr);
    fn log_debug(
        loggerName: *const libc::c_char,
        logMessage: *const libc::c_char,
        _: ...
    ) -> libc::c_int;
}
pub type __uint32_t = libc::c_uint;
pub type __uint64_t = libc::c_ulong;
pub type uint32_t = __uint32_t;
pub type uint64_t = __uint64_t;
pub type size_t = libc::c_ulong;
pub type aesrand_t = aesrand;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct cyclic_group {
    pub prime: uint64_t,
    pub known_primroot: uint64_t,
    pub num_prime_factors: size_t,
    pub prime_factors: [uint64_t; 10],
}
pub type cyclic_group_t = cyclic_group;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct cycle {
    pub group: *const cyclic_group_t,
    pub generator: uint64_t,
    pub order: uint64_t,
    pub offset: uint32_t,
}
pub type cycle_t = cycle;
pub type mpz_srcptr = *const __mpz_struct;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct __mpz_struct {
    pub _mp_alloc: libc::c_int,
    pub _mp_size: libc::c_int,
    pub _mp_d: *mut mp_limb_t,
}
pub type mp_limb_t = libc::c_ulong;
pub type mpz_t = [__mpz_struct; 1];
pub type mp_ptr = *mut mp_limb_t;
pub type mp_size_t = libc::c_long;
pub type mpz_ptr = *mut __mpz_struct;
#[inline]
unsafe extern "C" fn __gmpz_get_ui(mut __gmp_z: mpz_srcptr) -> libc::c_ulong {
    let mut __gmp_p: mp_ptr = (*__gmp_z)._mp_d;
    let mut __gmp_n: mp_size_t = (*__gmp_z)._mp_size as mp_size_t;
    let mut __gmp_l: mp_limb_t = *__gmp_p.offset(0 as libc::c_int as isize);
    return if __gmp_n != 0 as libc::c_int as libc::c_long {
        __gmp_l
    } else {
        0 as libc::c_int as libc::c_ulong
    };
}
static mut groups: [cyclic_group_t; 5] = [
    {
        let mut init = cyclic_group {
            prime: 257 as libc::c_int as uint64_t,
            known_primroot: 3 as libc::c_int as uint64_t,
            num_prime_factors: 1 as libc::c_int as size_t,
            prime_factors: [2 as libc::c_int as uint64_t, 0, 0, 0, 0, 0, 0, 0, 0, 0],
        };
        init
    },
    {
        let mut init = cyclic_group {
            prime: 65537 as libc::c_int as uint64_t,
            known_primroot: 3 as libc::c_int as uint64_t,
            num_prime_factors: 1 as libc::c_int as size_t,
            prime_factors: [2 as libc::c_int as uint64_t, 0, 0, 0, 0, 0, 0, 0, 0, 0],
        };
        init
    },
    {
        let mut init = cyclic_group {
            prime: 16777259 as libc::c_int as uint64_t,
            known_primroot: 2 as libc::c_int as uint64_t,
            num_prime_factors: 4 as libc::c_int as size_t,
            prime_factors: [
                2 as libc::c_int as uint64_t,
                23 as libc::c_int as uint64_t,
                103 as libc::c_int as uint64_t,
                3541 as libc::c_int as uint64_t,
                0,
                0,
                0,
                0,
                0,
                0,
            ],
        };
        init
    },
    {
        let mut init = cyclic_group {
            prime: 268435459 as libc::c_int as uint64_t,
            known_primroot: 2 as libc::c_int as uint64_t,
            num_prime_factors: 4 as libc::c_int as size_t,
            prime_factors: [
                2 as libc::c_int as uint64_t,
                3 as libc::c_int as uint64_t,
                19 as libc::c_int as uint64_t,
                87211 as libc::c_int as uint64_t,
                0,
                0,
                0,
                0,
                0,
                0,
            ],
        };
        init
    },
    {
        let mut init = cyclic_group {
            prime: 4294967311 as libc::c_long as uint64_t,
            known_primroot: 3 as libc::c_int as uint64_t,
            num_prime_factors: 5 as libc::c_int as size_t,
            prime_factors: [
                2 as libc::c_int as uint64_t,
                3 as libc::c_int as uint64_t,
                5 as libc::c_int as uint64_t,
                131 as libc::c_int as uint64_t,
                364289 as libc::c_int as uint64_t,
                0,
                0,
                0,
                0,
                0,
            ],
        };
        init
    },
];
unsafe extern "C" fn check_coprime(
    mut check: uint64_t,
    mut group: *const cyclic_group_t,
) -> libc::c_int {
    if check == 0 as libc::c_int as libc::c_ulong
        || check == 1 as libc::c_int as libc::c_ulong
    {
        return 0 as libc::c_int;
    }
    let mut i: libc::c_uint = 0 as libc::c_int as libc::c_uint;
    while (i as libc::c_ulong) < (*group).num_prime_factors {
        if (*group).prime_factors[i as usize] > check
            && ((*group).prime_factors[i as usize]).wrapping_rem(check) == 0
        {
            return 0 as libc::c_int
        } else if (*group).prime_factors[i as usize] < check
            && check.wrapping_rem((*group).prime_factors[i as usize]) == 0
        {
            return 0 as libc::c_int
        } else if (*group).prime_factors[i as usize] == check {
            return 0 as libc::c_int
        }
        i = i.wrapping_add(1);
        i;
    }
    return 1 as libc::c_int;
}
unsafe extern "C" fn find_primroot(
    mut group: *const cyclic_group_t,
    mut aes: *mut aesrand_t,
) -> uint32_t {
    let mut candidate: uint32_t = (aesrand_getword(aes)
        & 0xffffffff as libc::c_uint as libc::c_ulong)
        .wrapping_rem((*group).prime) as uint32_t;
    let mut retv: uint64_t = 0 as libc::c_int as uint64_t;
    let max_root: uint64_t = ((1 as libc::c_ulong) << 32 as libc::c_int)
        .wrapping_sub(14 as libc::c_int as libc::c_ulong);
    loop {
        while check_coprime(candidate as uint64_t, group) != 1 as libc::c_int {
            candidate = (candidate as libc::c_uint)
                .wrapping_add(1 as libc::c_int as libc::c_uint) as uint32_t as uint32_t;
            candidate = (candidate as libc::c_ulong).wrapping_rem((*group).prime)
                as uint32_t as uint32_t;
        }
        retv = isomorphism(candidate as uint64_t, group);
        if !(retv > max_root) {
            break;
        }
    }
    return retv as uint32_t;
}
pub unsafe extern "C" fn get_group(mut min_size: uint64_t) -> *const cyclic_group_t {
    let mut i: libc::c_uint = 0 as libc::c_int as libc::c_uint;
    while (i as libc::c_ulong)
        < ::std::mem::size_of::<[cyclic_group_t; 5]>() as libc::c_ulong
    {
        if groups[i as usize].prime > min_size {
            return &mut *groups.as_mut_ptr().offset(i as isize) as *mut cyclic_group_t;
        }
        i = i.wrapping_add(1);
        i;
    }
    __assert_fail(
        b"0\0" as *const u8 as *const libc::c_char,
        b"cyclic.c\0" as *const u8 as *const libc::c_char,
        142 as libc::c_int as libc::c_uint,
        (*::std::mem::transmute::<
            &[u8; 42],
            &[libc::c_char; 42],
        >(b"const cyclic_group_t *get_group(uint64_t)\0"))
            .as_ptr(),
    );
    'c_202: {
        __assert_fail(
            b"0\0" as *const u8 as *const libc::c_char,
            b"cyclic.c\0" as *const u8 as *const libc::c_char,
            142 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 42],
                &[libc::c_char; 42],
            >(b"const cyclic_group_t *get_group(uint64_t)\0"))
                .as_ptr(),
        );
    };
    panic!();
}
pub unsafe extern "C" fn make_cycle(
    mut group: *const cyclic_group_t,
    mut aes: *mut aesrand_t,
) -> cycle_t {
    let mut cycle: cycle_t = cycle_t {
        group: 0 as *const cyclic_group_t,
        generator: 0,
        order: 0,
        offset: 0,
    };
    cycle.group = group;
    cycle.generator = find_primroot(group, aes) as uint64_t;
    cycle
        .offset = (aesrand_getword(aes) & 0xffffffff as libc::c_uint as libc::c_ulong)
        as uint32_t;
    cycle
        .offset = (cycle.offset as libc::c_ulong).wrapping_rem((*group).prime)
        as uint32_t as uint32_t;
    cycle.order = ((*group).prime).wrapping_sub(1 as libc::c_int as libc::c_ulong);
    return cycle;
}
pub unsafe extern "C" fn isomorphism(
    mut additive_elt: uint64_t,
    mut mult_group: *const cyclic_group_t,
) -> uint64_t {
    if additive_elt < (*mult_group).prime {} else {
        __assert_fail(
            b"additive_elt < mult_group->prime\0" as *const u8 as *const libc::c_char,
            b"cyclic.c\0" as *const u8 as *const libc::c_char,
            158 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 55],
                &[libc::c_char; 55],
            >(b"uint64_t isomorphism(uint64_t, const cyclic_group_t *)\0"))
                .as_ptr(),
        );
    }
    'c_697: {
        if additive_elt < (*mult_group).prime {} else {
            __assert_fail(
                b"additive_elt < mult_group->prime\0" as *const u8
                    as *const libc::c_char,
                b"cyclic.c\0" as *const u8 as *const libc::c_char,
                158 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 55],
                    &[libc::c_char; 55],
                >(b"uint64_t isomorphism(uint64_t, const cyclic_group_t *)\0"))
                    .as_ptr(),
            );
        }
    };
    let mut base: mpz_t = [__mpz_struct {
        _mp_alloc: 0,
        _mp_size: 0,
        _mp_d: 0 as *mut mp_limb_t,
    }; 1];
    let mut power: mpz_t = [__mpz_struct {
        _mp_alloc: 0,
        _mp_size: 0,
        _mp_d: 0 as *mut mp_limb_t,
    }; 1];
    let mut prime: mpz_t = [__mpz_struct {
        _mp_alloc: 0,
        _mp_size: 0,
        _mp_d: 0 as *mut mp_limb_t,
    }; 1];
    let mut primroot: mpz_t = [__mpz_struct {
        _mp_alloc: 0,
        _mp_size: 0,
        _mp_d: 0 as *mut mp_limb_t,
    }; 1];
    __gmpz_init_set_ui(base.as_mut_ptr(), (*mult_group).known_primroot);
    __gmpz_init_set_ui(power.as_mut_ptr(), additive_elt);
    __gmpz_init_set_ui(prime.as_mut_ptr(), (*mult_group).prime);
    __gmpz_init(primroot.as_mut_ptr());
    __gmpz_powm(
        primroot.as_mut_ptr(),
        base.as_mut_ptr() as mpz_srcptr,
        power.as_mut_ptr() as mpz_srcptr,
        prime.as_mut_ptr() as mpz_srcptr,
    );
    let mut retv: uint64_t = __gmpz_get_ui(primroot.as_mut_ptr() as mpz_srcptr);
    log_debug(
        b"zmap\0" as *const u8 as *const libc::c_char,
        b"Isomorphism: %llu\0" as *const u8 as *const libc::c_char,
        retv,
    );
    __gmpz_clear(base.as_mut_ptr());
    __gmpz_clear(power.as_mut_ptr());
    __gmpz_clear(prime.as_mut_ptr());
    __gmpz_clear(primroot.as_mut_ptr());
    return retv;
}
