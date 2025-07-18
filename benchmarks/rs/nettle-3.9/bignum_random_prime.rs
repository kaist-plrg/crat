use ::libc;
extern "C" {
    fn __assert_fail(
        __assertion: *const libc::c_char,
        __file: *const libc::c_char,
        __line: libc::c_uint,
        __function: *const libc::c_char,
    ) -> !;
    fn __gmpz_add(_: mpz_ptr, _: mpz_srcptr, _: mpz_srcptr);
    fn __gmpz_add_ui(_: mpz_ptr, _: mpz_srcptr, _: libc::c_ulong);
    fn __gmpz_clear(_: mpz_ptr);
    fn __gmpz_cmp(_: mpz_srcptr, _: mpz_srcptr) -> libc::c_int;
    fn __gmpz_cmp_ui(_: mpz_srcptr, _: libc::c_ulong) -> libc::c_int;
    fn __gmpz_fdiv_q(_: mpz_ptr, _: mpz_srcptr, _: mpz_srcptr);
    fn __gmpz_fdiv_q_2exp(_: mpz_ptr, _: mpz_srcptr, _: mp_bitcnt_t);
    fn __gmpz_gcd(_: mpz_ptr, _: mpz_srcptr, _: mpz_srcptr);
    fn __gmpz_init(_: mpz_ptr);
    fn __gmpz_mul(_: mpz_ptr, _: mpz_srcptr, _: mpz_srcptr);
    fn __gmpz_mul_2exp(_: mpz_ptr, _: mpz_srcptr, _: mp_bitcnt_t);
    fn __gmpz_mul_ui(_: mpz_ptr, _: mpz_srcptr, _: libc::c_ulong);
    fn __gmpz_powm(_: mpz_ptr, _: mpz_srcptr, _: mpz_srcptr, _: mpz_srcptr);
    fn __gmpz_powm_ui(_: mpz_ptr, _: mpz_srcptr, _: libc::c_ulong, _: mpz_srcptr);
    fn __gmpz_probab_prime_p(_: mpz_srcptr, _: libc::c_int) -> libc::c_int;
    fn __gmpz_scan1(_: mpz_srcptr, _: mp_bitcnt_t) -> mp_bitcnt_t;
    fn __gmpz_set_ui(_: mpz_ptr, _: libc::c_ulong);
    fn __gmpz_sizeinbase(_: mpz_srcptr, _: libc::c_int) -> size_t;
    fn __gmpz_sub_ui(_: mpz_ptr, _: mpz_srcptr, _: libc::c_ulong);
    fn __gmpz_submul_ui(_: mpz_ptr, _: mpz_srcptr, _: libc::c_ulong);
    fn __gmpz_tdiv_qr(_: mpz_ptr, _: mpz_ptr, _: mpz_srcptr, _: mpz_srcptr);
    fn __gmpn_perfect_square_p(_: mp_srcptr, _: mp_size_t) -> libc::c_int;
    fn nettle_mpz_random(
        x: *mut __mpz_struct,
        ctx: *mut libc::c_void,
        random: Option::<nettle_random_func>,
        n: *const __mpz_struct,
    );
}
pub type size_t = libc::c_ulong;
pub type __uint8_t = libc::c_uchar;
pub type __uint16_t = libc::c_ushort;
pub type __uint32_t = libc::c_uint;
pub type uint8_t = __uint8_t;
pub type uint16_t = __uint16_t;
pub type uint32_t = __uint32_t;
pub type nettle_random_func = unsafe extern "C" fn(
    *mut libc::c_void,
    size_t,
    *mut uint8_t,
) -> ();
pub type nettle_progress_func = unsafe extern "C" fn(
    *mut libc::c_void,
    libc::c_int,
) -> ();
pub type mp_limb_t = libc::c_ulong;
pub type mp_bitcnt_t = libc::c_ulong;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct __mpz_struct {
    pub _mp_alloc: libc::c_int,
    pub _mp_size: libc::c_int,
    pub _mp_d: *mut mp_limb_t,
}
pub type mpz_t = [__mpz_struct; 1];
pub type mp_srcptr = *const mp_limb_t;
pub type mp_size_t = libc::c_long;
pub type mpz_srcptr = *const __mpz_struct;
pub type mpz_ptr = *mut __mpz_struct;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct trial_div_info {
    pub inverse: uint32_t,
    pub limit: uint32_t,
}
#[inline]
unsafe extern "C" fn __gmpz_perfect_square_p(mut __gmp_a: mpz_srcptr) -> libc::c_int {
    let mut __gmp_asize: mp_size_t = 0;
    let mut __gmp_result: libc::c_int = 0;
    __gmp_asize = (*__gmp_a)._mp_size as mp_size_t;
    __gmp_result = (__gmp_asize >= 0 as libc::c_int as libc::c_long) as libc::c_int;
    if ((__gmp_asize > 0 as libc::c_int as libc::c_long) as libc::c_int
        != 0 as libc::c_int) as libc::c_int as libc::c_long != 0
    {
        __gmp_result = __gmpn_perfect_square_p(
            (*__gmp_a)._mp_d as mp_srcptr,
            __gmp_asize,
        );
    }
    return __gmp_result;
}
static mut primes: [uint16_t; 171] = [
    3 as libc::c_int as uint16_t,
    5 as libc::c_int as uint16_t,
    7 as libc::c_int as uint16_t,
    11 as libc::c_int as uint16_t,
    13 as libc::c_int as uint16_t,
    17 as libc::c_int as uint16_t,
    19 as libc::c_int as uint16_t,
    23 as libc::c_int as uint16_t,
    29 as libc::c_int as uint16_t,
    31 as libc::c_int as uint16_t,
    37 as libc::c_int as uint16_t,
    41 as libc::c_int as uint16_t,
    43 as libc::c_int as uint16_t,
    47 as libc::c_int as uint16_t,
    53 as libc::c_int as uint16_t,
    59 as libc::c_int as uint16_t,
    61 as libc::c_int as uint16_t,
    67 as libc::c_int as uint16_t,
    71 as libc::c_int as uint16_t,
    73 as libc::c_int as uint16_t,
    79 as libc::c_int as uint16_t,
    83 as libc::c_int as uint16_t,
    89 as libc::c_int as uint16_t,
    97 as libc::c_int as uint16_t,
    101 as libc::c_int as uint16_t,
    103 as libc::c_int as uint16_t,
    107 as libc::c_int as uint16_t,
    109 as libc::c_int as uint16_t,
    113 as libc::c_int as uint16_t,
    127 as libc::c_int as uint16_t,
    131 as libc::c_int as uint16_t,
    137 as libc::c_int as uint16_t,
    139 as libc::c_int as uint16_t,
    149 as libc::c_int as uint16_t,
    151 as libc::c_int as uint16_t,
    157 as libc::c_int as uint16_t,
    163 as libc::c_int as uint16_t,
    167 as libc::c_int as uint16_t,
    173 as libc::c_int as uint16_t,
    179 as libc::c_int as uint16_t,
    181 as libc::c_int as uint16_t,
    191 as libc::c_int as uint16_t,
    193 as libc::c_int as uint16_t,
    197 as libc::c_int as uint16_t,
    199 as libc::c_int as uint16_t,
    211 as libc::c_int as uint16_t,
    223 as libc::c_int as uint16_t,
    227 as libc::c_int as uint16_t,
    229 as libc::c_int as uint16_t,
    233 as libc::c_int as uint16_t,
    239 as libc::c_int as uint16_t,
    241 as libc::c_int as uint16_t,
    251 as libc::c_int as uint16_t,
    257 as libc::c_int as uint16_t,
    263 as libc::c_int as uint16_t,
    269 as libc::c_int as uint16_t,
    271 as libc::c_int as uint16_t,
    277 as libc::c_int as uint16_t,
    281 as libc::c_int as uint16_t,
    283 as libc::c_int as uint16_t,
    293 as libc::c_int as uint16_t,
    307 as libc::c_int as uint16_t,
    311 as libc::c_int as uint16_t,
    313 as libc::c_int as uint16_t,
    317 as libc::c_int as uint16_t,
    331 as libc::c_int as uint16_t,
    337 as libc::c_int as uint16_t,
    347 as libc::c_int as uint16_t,
    349 as libc::c_int as uint16_t,
    353 as libc::c_int as uint16_t,
    359 as libc::c_int as uint16_t,
    367 as libc::c_int as uint16_t,
    373 as libc::c_int as uint16_t,
    379 as libc::c_int as uint16_t,
    383 as libc::c_int as uint16_t,
    389 as libc::c_int as uint16_t,
    397 as libc::c_int as uint16_t,
    401 as libc::c_int as uint16_t,
    409 as libc::c_int as uint16_t,
    419 as libc::c_int as uint16_t,
    421 as libc::c_int as uint16_t,
    431 as libc::c_int as uint16_t,
    433 as libc::c_int as uint16_t,
    439 as libc::c_int as uint16_t,
    443 as libc::c_int as uint16_t,
    449 as libc::c_int as uint16_t,
    457 as libc::c_int as uint16_t,
    461 as libc::c_int as uint16_t,
    463 as libc::c_int as uint16_t,
    467 as libc::c_int as uint16_t,
    479 as libc::c_int as uint16_t,
    487 as libc::c_int as uint16_t,
    491 as libc::c_int as uint16_t,
    499 as libc::c_int as uint16_t,
    503 as libc::c_int as uint16_t,
    509 as libc::c_int as uint16_t,
    521 as libc::c_int as uint16_t,
    523 as libc::c_int as uint16_t,
    541 as libc::c_int as uint16_t,
    547 as libc::c_int as uint16_t,
    557 as libc::c_int as uint16_t,
    563 as libc::c_int as uint16_t,
    569 as libc::c_int as uint16_t,
    571 as libc::c_int as uint16_t,
    577 as libc::c_int as uint16_t,
    587 as libc::c_int as uint16_t,
    593 as libc::c_int as uint16_t,
    599 as libc::c_int as uint16_t,
    601 as libc::c_int as uint16_t,
    607 as libc::c_int as uint16_t,
    613 as libc::c_int as uint16_t,
    617 as libc::c_int as uint16_t,
    619 as libc::c_int as uint16_t,
    631 as libc::c_int as uint16_t,
    641 as libc::c_int as uint16_t,
    643 as libc::c_int as uint16_t,
    647 as libc::c_int as uint16_t,
    653 as libc::c_int as uint16_t,
    659 as libc::c_int as uint16_t,
    661 as libc::c_int as uint16_t,
    673 as libc::c_int as uint16_t,
    677 as libc::c_int as uint16_t,
    683 as libc::c_int as uint16_t,
    691 as libc::c_int as uint16_t,
    701 as libc::c_int as uint16_t,
    709 as libc::c_int as uint16_t,
    719 as libc::c_int as uint16_t,
    727 as libc::c_int as uint16_t,
    733 as libc::c_int as uint16_t,
    739 as libc::c_int as uint16_t,
    743 as libc::c_int as uint16_t,
    751 as libc::c_int as uint16_t,
    757 as libc::c_int as uint16_t,
    761 as libc::c_int as uint16_t,
    769 as libc::c_int as uint16_t,
    773 as libc::c_int as uint16_t,
    787 as libc::c_int as uint16_t,
    797 as libc::c_int as uint16_t,
    809 as libc::c_int as uint16_t,
    811 as libc::c_int as uint16_t,
    821 as libc::c_int as uint16_t,
    823 as libc::c_int as uint16_t,
    827 as libc::c_int as uint16_t,
    829 as libc::c_int as uint16_t,
    839 as libc::c_int as uint16_t,
    853 as libc::c_int as uint16_t,
    857 as libc::c_int as uint16_t,
    859 as libc::c_int as uint16_t,
    863 as libc::c_int as uint16_t,
    877 as libc::c_int as uint16_t,
    881 as libc::c_int as uint16_t,
    883 as libc::c_int as uint16_t,
    887 as libc::c_int as uint16_t,
    907 as libc::c_int as uint16_t,
    911 as libc::c_int as uint16_t,
    919 as libc::c_int as uint16_t,
    929 as libc::c_int as uint16_t,
    937 as libc::c_int as uint16_t,
    941 as libc::c_int as uint16_t,
    947 as libc::c_int as uint16_t,
    953 as libc::c_int as uint16_t,
    967 as libc::c_int as uint16_t,
    971 as libc::c_int as uint16_t,
    977 as libc::c_int as uint16_t,
    983 as libc::c_int as uint16_t,
    991 as libc::c_int as uint16_t,
    997 as libc::c_int as uint16_t,
    1009 as libc::c_int as uint16_t,
    1013 as libc::c_int as uint16_t,
    1019 as libc::c_int as uint16_t,
    1021 as libc::c_int as uint16_t,
];
static mut prime_square: [uint32_t; 172] = [
    9 as libc::c_int as uint32_t,
    25 as libc::c_int as uint32_t,
    49 as libc::c_int as uint32_t,
    121 as libc::c_int as uint32_t,
    169 as libc::c_int as uint32_t,
    289 as libc::c_int as uint32_t,
    361 as libc::c_int as uint32_t,
    529 as libc::c_int as uint32_t,
    841 as libc::c_int as uint32_t,
    961 as libc::c_int as uint32_t,
    1369 as libc::c_int as uint32_t,
    1681 as libc::c_int as uint32_t,
    1849 as libc::c_int as uint32_t,
    2209 as libc::c_int as uint32_t,
    2809 as libc::c_int as uint32_t,
    3481 as libc::c_int as uint32_t,
    3721 as libc::c_int as uint32_t,
    4489 as libc::c_int as uint32_t,
    5041 as libc::c_int as uint32_t,
    5329 as libc::c_int as uint32_t,
    6241 as libc::c_int as uint32_t,
    6889 as libc::c_int as uint32_t,
    7921 as libc::c_int as uint32_t,
    9409 as libc::c_int as uint32_t,
    10201 as libc::c_int as uint32_t,
    10609 as libc::c_int as uint32_t,
    11449 as libc::c_int as uint32_t,
    11881 as libc::c_int as uint32_t,
    12769 as libc::c_int as uint32_t,
    16129 as libc::c_int as uint32_t,
    17161 as libc::c_int as uint32_t,
    18769 as libc::c_int as uint32_t,
    19321 as libc::c_int as uint32_t,
    22201 as libc::c_int as uint32_t,
    22801 as libc::c_int as uint32_t,
    24649 as libc::c_int as uint32_t,
    26569 as libc::c_int as uint32_t,
    27889 as libc::c_int as uint32_t,
    29929 as libc::c_int as uint32_t,
    32041 as libc::c_int as uint32_t,
    32761 as libc::c_int as uint32_t,
    36481 as libc::c_int as uint32_t,
    37249 as libc::c_int as uint32_t,
    38809 as libc::c_int as uint32_t,
    39601 as libc::c_int as uint32_t,
    44521 as libc::c_int as uint32_t,
    49729 as libc::c_int as uint32_t,
    51529 as libc::c_int as uint32_t,
    52441 as libc::c_int as uint32_t,
    54289 as libc::c_int as uint32_t,
    57121 as libc::c_int as uint32_t,
    58081 as libc::c_int as uint32_t,
    63001 as libc::c_int as uint32_t,
    66049 as libc::c_int as uint32_t,
    69169 as libc::c_int as uint32_t,
    72361 as libc::c_int as uint32_t,
    73441 as libc::c_int as uint32_t,
    76729 as libc::c_int as uint32_t,
    78961 as libc::c_int as uint32_t,
    80089 as libc::c_int as uint32_t,
    85849 as libc::c_int as uint32_t,
    94249 as libc::c_int as uint32_t,
    96721 as libc::c_int as uint32_t,
    97969 as libc::c_int as uint32_t,
    100489 as libc::c_int as uint32_t,
    109561 as libc::c_int as uint32_t,
    113569 as libc::c_int as uint32_t,
    120409 as libc::c_int as uint32_t,
    121801 as libc::c_int as uint32_t,
    124609 as libc::c_int as uint32_t,
    128881 as libc::c_int as uint32_t,
    134689 as libc::c_int as uint32_t,
    139129 as libc::c_int as uint32_t,
    143641 as libc::c_int as uint32_t,
    146689 as libc::c_int as uint32_t,
    151321 as libc::c_int as uint32_t,
    157609 as libc::c_int as uint32_t,
    160801 as libc::c_int as uint32_t,
    167281 as libc::c_int as uint32_t,
    175561 as libc::c_int as uint32_t,
    177241 as libc::c_int as uint32_t,
    185761 as libc::c_int as uint32_t,
    187489 as libc::c_int as uint32_t,
    192721 as libc::c_int as uint32_t,
    196249 as libc::c_int as uint32_t,
    201601 as libc::c_int as uint32_t,
    208849 as libc::c_int as uint32_t,
    212521 as libc::c_int as uint32_t,
    214369 as libc::c_int as uint32_t,
    218089 as libc::c_int as uint32_t,
    229441 as libc::c_int as uint32_t,
    237169 as libc::c_int as uint32_t,
    241081 as libc::c_int as uint32_t,
    249001 as libc::c_int as uint32_t,
    253009 as libc::c_int as uint32_t,
    259081 as libc::c_int as uint32_t,
    271441 as libc::c_int as uint32_t,
    273529 as libc::c_int as uint32_t,
    292681 as libc::c_int as uint32_t,
    299209 as libc::c_int as uint32_t,
    310249 as libc::c_int as uint32_t,
    316969 as libc::c_int as uint32_t,
    323761 as libc::c_int as uint32_t,
    326041 as libc::c_int as uint32_t,
    332929 as libc::c_int as uint32_t,
    344569 as libc::c_int as uint32_t,
    351649 as libc::c_int as uint32_t,
    358801 as libc::c_int as uint32_t,
    361201 as libc::c_int as uint32_t,
    368449 as libc::c_int as uint32_t,
    375769 as libc::c_int as uint32_t,
    380689 as libc::c_int as uint32_t,
    383161 as libc::c_int as uint32_t,
    398161 as libc::c_int as uint32_t,
    410881 as libc::c_int as uint32_t,
    413449 as libc::c_int as uint32_t,
    418609 as libc::c_int as uint32_t,
    426409 as libc::c_int as uint32_t,
    434281 as libc::c_int as uint32_t,
    436921 as libc::c_int as uint32_t,
    452929 as libc::c_int as uint32_t,
    458329 as libc::c_int as uint32_t,
    466489 as libc::c_int as uint32_t,
    477481 as libc::c_int as uint32_t,
    491401 as libc::c_int as uint32_t,
    502681 as libc::c_int as uint32_t,
    516961 as libc::c_int as uint32_t,
    528529 as libc::c_int as uint32_t,
    537289 as libc::c_int as uint32_t,
    546121 as libc::c_int as uint32_t,
    552049 as libc::c_int as uint32_t,
    564001 as libc::c_int as uint32_t,
    573049 as libc::c_int as uint32_t,
    579121 as libc::c_int as uint32_t,
    591361 as libc::c_int as uint32_t,
    597529 as libc::c_int as uint32_t,
    619369 as libc::c_int as uint32_t,
    635209 as libc::c_int as uint32_t,
    654481 as libc::c_int as uint32_t,
    657721 as libc::c_int as uint32_t,
    674041 as libc::c_int as uint32_t,
    677329 as libc::c_int as uint32_t,
    683929 as libc::c_int as uint32_t,
    687241 as libc::c_int as uint32_t,
    703921 as libc::c_int as uint32_t,
    727609 as libc::c_int as uint32_t,
    734449 as libc::c_int as uint32_t,
    737881 as libc::c_int as uint32_t,
    744769 as libc::c_int as uint32_t,
    769129 as libc::c_int as uint32_t,
    776161 as libc::c_int as uint32_t,
    779689 as libc::c_int as uint32_t,
    786769 as libc::c_int as uint32_t,
    822649 as libc::c_int as uint32_t,
    829921 as libc::c_int as uint32_t,
    844561 as libc::c_int as uint32_t,
    863041 as libc::c_int as uint32_t,
    877969 as libc::c_int as uint32_t,
    885481 as libc::c_int as uint32_t,
    896809 as libc::c_int as uint32_t,
    908209 as libc::c_int as uint32_t,
    935089 as libc::c_int as uint32_t,
    942841 as libc::c_int as uint32_t,
    954529 as libc::c_int as uint32_t,
    966289 as libc::c_int as uint32_t,
    982081 as libc::c_int as uint32_t,
    994009 as libc::c_int as uint32_t,
    1018081 as libc::c_int as uint32_t,
    1026169 as libc::c_int as uint32_t,
    1038361 as libc::c_int as uint32_t,
    1042441 as libc::c_int as uint32_t,
    ((1 as libc::c_long) << 20 as libc::c_int) as uint32_t,
];
static mut trial_div_table: [trial_div_info; 171] = [
    {
        let mut init = trial_div_info {
            inverse: 699051 as libc::c_int as uint32_t,
            limit: 349525 as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = trial_div_info {
            inverse: 838861 as libc::c_int as uint32_t,
            limit: 209715 as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = trial_div_info {
            inverse: 748983 as libc::c_int as uint32_t,
            limit: 149796 as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = trial_div_info {
            inverse: 953251 as libc::c_int as uint32_t,
            limit: 95325 as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = trial_div_info {
            inverse: 806597 as libc::c_int as uint32_t,
            limit: 80659 as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = trial_div_info {
            inverse: 61681 as libc::c_int as uint32_t,
            limit: 61680 as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = trial_div_info {
            inverse: 772635 as libc::c_int as uint32_t,
            limit: 55188 as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = trial_div_info {
            inverse: 866215 as libc::c_int as uint32_t,
            limit: 45590 as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = trial_div_info {
            inverse: 180789 as libc::c_int as uint32_t,
            limit: 36157 as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = trial_div_info {
            inverse: 1014751 as libc::c_int as uint32_t,
            limit: 33825 as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = trial_div_info {
            inverse: 793517 as libc::c_int as uint32_t,
            limit: 28339 as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = trial_div_info {
            inverse: 1023001 as libc::c_int as uint32_t,
            limit: 25575 as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = trial_div_info {
            inverse: 48771 as libc::c_int as uint32_t,
            limit: 24385 as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = trial_div_info {
            inverse: 870095 as libc::c_int as uint32_t,
            limit: 22310 as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = trial_div_info {
            inverse: 217629 as libc::c_int as uint32_t,
            limit: 19784 as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = trial_div_info {
            inverse: 710899 as libc::c_int as uint32_t,
            limit: 17772 as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = trial_div_info {
            inverse: 825109 as libc::c_int as uint32_t,
            limit: 17189 as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = trial_div_info {
            inverse: 281707 as libc::c_int as uint32_t,
            limit: 15650 as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = trial_div_info {
            inverse: 502135 as libc::c_int as uint32_t,
            limit: 14768 as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = trial_div_info {
            inverse: 258553 as libc::c_int as uint32_t,
            limit: 14364 as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = trial_div_info {
            inverse: 464559 as libc::c_int as uint32_t,
            limit: 13273 as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = trial_div_info {
            inverse: 934875 as libc::c_int as uint32_t,
            limit: 12633 as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = trial_div_info {
            inverse: 1001449 as libc::c_int as uint32_t,
            limit: 11781 as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = trial_div_info {
            inverse: 172961 as libc::c_int as uint32_t,
            limit: 10810 as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = trial_div_info {
            inverse: 176493 as libc::c_int as uint32_t,
            limit: 10381 as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = trial_div_info {
            inverse: 203607 as libc::c_int as uint32_t,
            limit: 10180 as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = trial_div_info {
            inverse: 568387 as libc::c_int as uint32_t,
            limit: 9799 as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = trial_div_info {
            inverse: 788837 as libc::c_int as uint32_t,
            limit: 9619 as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = trial_div_info {
            inverse: 770193 as libc::c_int as uint32_t,
            limit: 9279 as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = trial_div_info {
            inverse: 1032063 as libc::c_int as uint32_t,
            limit: 8256 as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = trial_div_info {
            inverse: 544299 as libc::c_int as uint32_t,
            limit: 8004 as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = trial_div_info {
            inverse: 619961 as libc::c_int as uint32_t,
            limit: 7653 as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = trial_div_info {
            inverse: 550691 as libc::c_int as uint32_t,
            limit: 7543 as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = trial_div_info {
            inverse: 182973 as libc::c_int as uint32_t,
            limit: 7037 as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = trial_div_info {
            inverse: 229159 as libc::c_int as uint32_t,
            limit: 6944 as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = trial_div_info {
            inverse: 427445 as libc::c_int as uint32_t,
            limit: 6678 as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = trial_div_info {
            inverse: 701195 as libc::c_int as uint32_t,
            limit: 6432 as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = trial_div_info {
            inverse: 370455 as libc::c_int as uint32_t,
            limit: 6278 as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = trial_div_info {
            inverse: 90917 as libc::c_int as uint32_t,
            limit: 6061 as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = trial_div_info {
            inverse: 175739 as libc::c_int as uint32_t,
            limit: 5857 as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = trial_div_info {
            inverse: 585117 as libc::c_int as uint32_t,
            limit: 5793 as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = trial_div_info {
            inverse: 225087 as libc::c_int as uint32_t,
            limit: 5489 as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = trial_div_info {
            inverse: 298817 as libc::c_int as uint32_t,
            limit: 5433 as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = trial_div_info {
            inverse: 228877 as libc::c_int as uint32_t,
            limit: 5322 as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = trial_div_info {
            inverse: 442615 as libc::c_int as uint32_t,
            limit: 5269 as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = trial_div_info {
            inverse: 546651 as libc::c_int as uint32_t,
            limit: 4969 as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = trial_div_info {
            inverse: 244511 as libc::c_int as uint32_t,
            limit: 4702 as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = trial_div_info {
            inverse: 83147 as libc::c_int as uint32_t,
            limit: 4619 as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = trial_div_info {
            inverse: 769261 as libc::c_int as uint32_t,
            limit: 4578 as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = trial_div_info {
            inverse: 841561 as libc::c_int as uint32_t,
            limit: 4500 as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = trial_div_info {
            inverse: 732687 as libc::c_int as uint32_t,
            limit: 4387 as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = trial_div_info {
            inverse: 978961 as libc::c_int as uint32_t,
            limit: 4350 as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = trial_div_info {
            inverse: 133683 as libc::c_int as uint32_t,
            limit: 4177 as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = trial_div_info {
            inverse: 65281 as libc::c_int as uint32_t,
            limit: 4080 as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = trial_div_info {
            inverse: 629943 as libc::c_int as uint32_t,
            limit: 3986 as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = trial_div_info {
            inverse: 374213 as libc::c_int as uint32_t,
            limit: 3898 as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = trial_div_info {
            inverse: 708079 as libc::c_int as uint32_t,
            limit: 3869 as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = trial_div_info {
            inverse: 280125 as libc::c_int as uint32_t,
            limit: 3785 as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = trial_div_info {
            inverse: 641833 as libc::c_int as uint32_t,
            limit: 3731 as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = trial_div_info {
            inverse: 618771 as libc::c_int as uint32_t,
            limit: 3705 as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = trial_div_info {
            inverse: 930477 as libc::c_int as uint32_t,
            limit: 3578 as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = trial_div_info {
            inverse: 778747 as libc::c_int as uint32_t,
            limit: 3415 as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = trial_div_info {
            inverse: 623751 as libc::c_int as uint32_t,
            limit: 3371 as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = trial_div_info {
            inverse: 40201 as libc::c_int as uint32_t,
            limit: 3350 as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = trial_div_info {
            inverse: 122389 as libc::c_int as uint32_t,
            limit: 3307 as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = trial_div_info {
            inverse: 950371 as libc::c_int as uint32_t,
            limit: 3167 as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = trial_div_info {
            inverse: 1042353 as libc::c_int as uint32_t,
            limit: 3111 as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = trial_div_info {
            inverse: 18131 as libc::c_int as uint32_t,
            limit: 3021 as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = trial_div_info {
            inverse: 285429 as libc::c_int as uint32_t,
            limit: 3004 as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = trial_div_info {
            inverse: 549537 as libc::c_int as uint32_t,
            limit: 2970 as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = trial_div_info {
            inverse: 166487 as libc::c_int as uint32_t,
            limit: 2920 as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = trial_div_info {
            inverse: 294287 as libc::c_int as uint32_t,
            limit: 2857 as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = trial_div_info {
            inverse: 919261 as libc::c_int as uint32_t,
            limit: 2811 as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = trial_div_info {
            inverse: 636339 as libc::c_int as uint32_t,
            limit: 2766 as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = trial_div_info {
            inverse: 900735 as libc::c_int as uint32_t,
            limit: 2737 as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = trial_div_info {
            inverse: 118605 as libc::c_int as uint32_t,
            limit: 2695 as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = trial_div_info {
            inverse: 10565 as libc::c_int as uint32_t,
            limit: 2641 as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = trial_div_info {
            inverse: 188273 as libc::c_int as uint32_t,
            limit: 2614 as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = trial_div_info {
            inverse: 115369 as libc::c_int as uint32_t,
            limit: 2563 as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = trial_div_info {
            inverse: 735755 as libc::c_int as uint32_t,
            limit: 2502 as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = trial_div_info {
            inverse: 458285 as libc::c_int as uint32_t,
            limit: 2490 as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = trial_div_info {
            inverse: 914767 as libc::c_int as uint32_t,
            limit: 2432 as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = trial_div_info {
            inverse: 370513 as libc::c_int as uint32_t,
            limit: 2421 as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = trial_div_info {
            inverse: 1027079 as libc::c_int as uint32_t,
            limit: 2388 as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = trial_div_info {
            inverse: 629619 as libc::c_int as uint32_t,
            limit: 2366 as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = trial_div_info {
            inverse: 462401 as libc::c_int as uint32_t,
            limit: 2335 as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = trial_div_info {
            inverse: 649337 as libc::c_int as uint32_t,
            limit: 2294 as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = trial_div_info {
            inverse: 316165 as libc::c_int as uint32_t,
            limit: 2274 as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = trial_div_info {
            inverse: 484655 as libc::c_int as uint32_t,
            limit: 2264 as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = trial_div_info {
            inverse: 65115 as libc::c_int as uint32_t,
            limit: 2245 as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = trial_div_info {
            inverse: 326175 as libc::c_int as uint32_t,
            limit: 2189 as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = trial_div_info {
            inverse: 1016279 as libc::c_int as uint32_t,
            limit: 2153 as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = trial_div_info {
            inverse: 990915 as libc::c_int as uint32_t,
            limit: 2135 as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = trial_div_info {
            inverse: 556859 as libc::c_int as uint32_t,
            limit: 2101 as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = trial_div_info {
            inverse: 462791 as libc::c_int as uint32_t,
            limit: 2084 as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = trial_div_info {
            inverse: 844629 as libc::c_int as uint32_t,
            limit: 2060 as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = trial_div_info {
            inverse: 404537 as libc::c_int as uint32_t,
            limit: 2012 as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = trial_div_info {
            inverse: 457123 as libc::c_int as uint32_t,
            limit: 2004 as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = trial_div_info {
            inverse: 577589 as libc::c_int as uint32_t,
            limit: 1938 as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = trial_div_info {
            inverse: 638347 as libc::c_int as uint32_t,
            limit: 1916 as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = trial_div_info {
            inverse: 892325 as libc::c_int as uint32_t,
            limit: 1882 as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = trial_div_info {
            inverse: 182523 as libc::c_int as uint32_t,
            limit: 1862 as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = trial_div_info {
            inverse: 1002505 as libc::c_int as uint32_t,
            limit: 1842 as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = trial_div_info {
            inverse: 624371 as libc::c_int as uint32_t,
            limit: 1836 as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = trial_div_info {
            inverse: 69057 as libc::c_int as uint32_t,
            limit: 1817 as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = trial_div_info {
            inverse: 210787 as libc::c_int as uint32_t,
            limit: 1786 as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = trial_div_info {
            inverse: 558769 as libc::c_int as uint32_t,
            limit: 1768 as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = trial_div_info {
            inverse: 395623 as libc::c_int as uint32_t,
            limit: 1750 as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = trial_div_info {
            inverse: 992745 as libc::c_int as uint32_t,
            limit: 1744 as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = trial_div_info {
            inverse: 317855 as libc::c_int as uint32_t,
            limit: 1727 as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = trial_div_info {
            inverse: 384877 as libc::c_int as uint32_t,
            limit: 1710 as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = trial_div_info {
            inverse: 372185 as libc::c_int as uint32_t,
            limit: 1699 as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = trial_div_info {
            inverse: 105027 as libc::c_int as uint32_t,
            limit: 1693 as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = trial_div_info {
            inverse: 423751 as libc::c_int as uint32_t,
            limit: 1661 as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = trial_div_info {
            inverse: 408961 as libc::c_int as uint32_t,
            limit: 1635 as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = trial_div_info {
            inverse: 908331 as libc::c_int as uint32_t,
            limit: 1630 as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = trial_div_info {
            inverse: 74551 as libc::c_int as uint32_t,
            limit: 1620 as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = trial_div_info {
            inverse: 36933 as libc::c_int as uint32_t,
            limit: 1605 as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = trial_div_info {
            inverse: 617371 as libc::c_int as uint32_t,
            limit: 1591 as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = trial_div_info {
            inverse: 506045 as libc::c_int as uint32_t,
            limit: 1586 as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = trial_div_info {
            inverse: 24929 as libc::c_int as uint32_t,
            limit: 1558 as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = trial_div_info {
            inverse: 529709 as libc::c_int as uint32_t,
            limit: 1548 as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = trial_div_info {
            inverse: 1042435 as libc::c_int as uint32_t,
            limit: 1535 as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = trial_div_info {
            inverse: 31867 as libc::c_int as uint32_t,
            limit: 1517 as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = trial_div_info {
            inverse: 166037 as libc::c_int as uint32_t,
            limit: 1495 as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = trial_div_info {
            inverse: 928781 as libc::c_int as uint32_t,
            limit: 1478 as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = trial_div_info {
            inverse: 508975 as libc::c_int as uint32_t,
            limit: 1458 as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = trial_div_info {
            inverse: 4327 as libc::c_int as uint32_t,
            limit: 1442 as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = trial_div_info {
            inverse: 779637 as libc::c_int as uint32_t,
            limit: 1430 as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = trial_div_info {
            inverse: 742091 as libc::c_int as uint32_t,
            limit: 1418 as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = trial_div_info {
            inverse: 258263 as libc::c_int as uint32_t,
            limit: 1411 as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = trial_div_info {
            inverse: 879631 as libc::c_int as uint32_t,
            limit: 1396 as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = trial_div_info {
            inverse: 72029 as libc::c_int as uint32_t,
            limit: 1385 as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = trial_div_info {
            inverse: 728905 as libc::c_int as uint32_t,
            limit: 1377 as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = trial_div_info {
            inverse: 589057 as libc::c_int as uint32_t,
            limit: 1363 as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = trial_div_info {
            inverse: 348621 as libc::c_int as uint32_t,
            limit: 1356 as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = trial_div_info {
            inverse: 671515 as libc::c_int as uint32_t,
            limit: 1332 as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = trial_div_info {
            inverse: 710453 as libc::c_int as uint32_t,
            limit: 1315 as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = trial_div_info {
            inverse: 84249 as libc::c_int as uint32_t,
            limit: 1296 as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = trial_div_info {
            inverse: 959363 as libc::c_int as uint32_t,
            limit: 1292 as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = trial_div_info {
            inverse: 685853 as libc::c_int as uint32_t,
            limit: 1277 as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = trial_div_info {
            inverse: 467591 as libc::c_int as uint32_t,
            limit: 1274 as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = trial_div_info {
            inverse: 646643 as libc::c_int as uint32_t,
            limit: 1267 as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = trial_div_info {
            inverse: 683029 as libc::c_int as uint32_t,
            limit: 1264 as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = trial_div_info {
            inverse: 439927 as libc::c_int as uint32_t,
            limit: 1249 as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = trial_div_info {
            inverse: 254461 as libc::c_int as uint32_t,
            limit: 1229 as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = trial_div_info {
            inverse: 660713 as libc::c_int as uint32_t,
            limit: 1223 as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = trial_div_info {
            inverse: 554195 as libc::c_int as uint32_t,
            limit: 1220 as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = trial_div_info {
            inverse: 202911 as libc::c_int as uint32_t,
            limit: 1215 as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = trial_div_info {
            inverse: 753253 as libc::c_int as uint32_t,
            limit: 1195 as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = trial_div_info {
            inverse: 941457 as libc::c_int as uint32_t,
            limit: 1190 as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = trial_div_info {
            inverse: 776635 as libc::c_int as uint32_t,
            limit: 1187 as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = trial_div_info {
            inverse: 509511 as libc::c_int as uint32_t,
            limit: 1182 as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = trial_div_info {
            inverse: 986147 as libc::c_int as uint32_t,
            limit: 1156 as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = trial_div_info {
            inverse: 768879 as libc::c_int as uint32_t,
            limit: 1151 as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = trial_div_info {
            inverse: 699431 as libc::c_int as uint32_t,
            limit: 1140 as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = trial_div_info {
            inverse: 696417 as libc::c_int as uint32_t,
            limit: 1128 as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = trial_div_info {
            inverse: 86169 as libc::c_int as uint32_t,
            limit: 1119 as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = trial_div_info {
            inverse: 808997 as libc::c_int as uint32_t,
            limit: 1114 as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = trial_div_info {
            inverse: 25467 as libc::c_int as uint32_t,
            limit: 1107 as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = trial_div_info {
            inverse: 201353 as libc::c_int as uint32_t,
            limit: 1100 as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = trial_div_info {
            inverse: 708087 as libc::c_int as uint32_t,
            limit: 1084 as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = trial_div_info {
            inverse: 1018339 as libc::c_int as uint32_t,
            limit: 1079 as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = trial_div_info {
            inverse: 341297 as libc::c_int as uint32_t,
            limit: 1073 as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = trial_div_info {
            inverse: 434151 as libc::c_int as uint32_t,
            limit: 1066 as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = trial_div_info {
            inverse: 96287 as libc::c_int as uint32_t,
            limit: 1058 as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = trial_div_info {
            inverse: 950765 as libc::c_int as uint32_t,
            limit: 1051 as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = trial_div_info {
            inverse: 298257 as libc::c_int as uint32_t,
            limit: 1039 as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = trial_div_info {
            inverse: 675933 as libc::c_int as uint32_t,
            limit: 1035 as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = trial_div_info {
            inverse: 167731 as libc::c_int as uint32_t,
            limit: 1029 as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = trial_div_info {
            inverse: 815445 as libc::c_int as uint32_t,
            limit: 1027 as libc::c_int as uint32_t,
        };
        init
    },
];
static mut prime_by_size: [uint8_t; 9] = [
    1 as libc::c_int as uint8_t,
    3 as libc::c_int as uint8_t,
    5 as libc::c_int as uint8_t,
    10 as libc::c_int as uint8_t,
    17 as libc::c_int as uint8_t,
    30 as libc::c_int as uint8_t,
    53 as libc::c_int as uint8_t,
    96 as libc::c_int as uint8_t,
    171 as libc::c_int as uint8_t,
];
unsafe extern "C" fn miller_rabin_pocklington(
    mut n: *mut __mpz_struct,
    mut nm1: *mut __mpz_struct,
    mut nm1dq: *mut __mpz_struct,
    mut a: *mut __mpz_struct,
) -> libc::c_int {
    let mut current_block: u64;
    let mut r: mpz_t = [__mpz_struct {
        _mp_alloc: 0,
        _mp_size: 0,
        _mp_d: 0 as *mut mp_limb_t,
    }; 1];
    let mut y: mpz_t = [__mpz_struct {
        _mp_alloc: 0,
        _mp_size: 0,
        _mp_d: 0 as *mut mp_limb_t,
    }; 1];
    let mut is_prime: libc::c_int = 0 as libc::c_int;
    let mut k: libc::c_uint = 0;
    let mut j: libc::c_uint = 0;
    if ((*n)._mp_size != 0 as libc::c_int) as libc::c_int
        & *((*n)._mp_d).offset(0 as libc::c_int as isize) as libc::c_int == 0
        || (if 0 != 0 && 3 as libc::c_int == 0 as libc::c_int {
            (if (*n)._mp_size < 0 as libc::c_int {
                -(1 as libc::c_int)
            } else {
                ((*n)._mp_size > 0 as libc::c_int) as libc::c_int
            })
        } else {
            __gmpz_cmp_ui(n as mpz_srcptr, 3 as libc::c_int as libc::c_ulong)
        }) < 0 as libc::c_int
    {
        return 0 as libc::c_int;
    }
    __gmpz_init(r.as_mut_ptr());
    __gmpz_init(y.as_mut_ptr());
    k = __gmpz_scan1(nm1 as mpz_srcptr, 0 as libc::c_int as mp_bitcnt_t) as libc::c_uint;
    if k > 0 as libc::c_int as libc::c_uint {} else {
        __assert_fail(
            b"k > 0\0" as *const u8 as *const libc::c_char,
            b"bignum-random-prime.c\0" as *const u8 as *const libc::c_char,
            201 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 93],
                &[libc::c_char; 93],
            >(
                b"int miller_rabin_pocklington(__mpz_struct *, __mpz_struct *, __mpz_struct *, __mpz_struct *)\0",
            ))
                .as_ptr(),
        );
    }
    'c_4924: {
        if k > 0 as libc::c_int as libc::c_uint {} else {
            __assert_fail(
                b"k > 0\0" as *const u8 as *const libc::c_char,
                b"bignum-random-prime.c\0" as *const u8 as *const libc::c_char,
                201 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 93],
                    &[libc::c_char; 93],
                >(
                    b"int miller_rabin_pocklington(__mpz_struct *, __mpz_struct *, __mpz_struct *, __mpz_struct *)\0",
                ))
                    .as_ptr(),
            );
        }
    };
    __gmpz_fdiv_q_2exp(r.as_mut_ptr(), nm1 as mpz_srcptr, k as mp_bitcnt_t);
    __gmpz_powm(
        y.as_mut_ptr(),
        a as mpz_srcptr,
        r.as_mut_ptr() as mpz_srcptr,
        n as mpz_srcptr,
    );
    if (if 0 != 0 && 1 as libc::c_int == 0 as libc::c_int {
        (if (*y.as_mut_ptr())._mp_size < 0 as libc::c_int {
            -(1 as libc::c_int)
        } else {
            ((*y.as_mut_ptr())._mp_size > 0 as libc::c_int) as libc::c_int
        })
    } else {
        __gmpz_cmp_ui(y.as_mut_ptr() as mpz_srcptr, 1 as libc::c_int as libc::c_ulong)
    }) == 0 as libc::c_int
        || __gmpz_cmp(y.as_mut_ptr() as mpz_srcptr, nm1 as mpz_srcptr)
            == 0 as libc::c_int
    {
        current_block = 12800627514080957624;
    } else {
        j = 1 as libc::c_int as libc::c_uint;
        loop {
            if !(j < k) {
                current_block = 6009453772311597924;
                break;
            }
            __gmpz_powm_ui(
                y.as_mut_ptr(),
                y.as_mut_ptr() as mpz_srcptr,
                2 as libc::c_int as libc::c_ulong,
                n as mpz_srcptr,
            );
            if (if 0 != 0 && 1 as libc::c_int == 0 as libc::c_int {
                (if (*y.as_mut_ptr())._mp_size < 0 as libc::c_int {
                    -(1 as libc::c_int)
                } else {
                    ((*y.as_mut_ptr())._mp_size > 0 as libc::c_int) as libc::c_int
                })
            } else {
                __gmpz_cmp_ui(
                    y.as_mut_ptr() as mpz_srcptr,
                    1 as libc::c_int as libc::c_ulong,
                )
            }) == 0 as libc::c_int
            {
                current_block = 6009453772311597924;
                break;
            }
            if __gmpz_cmp(y.as_mut_ptr() as mpz_srcptr, nm1 as mpz_srcptr)
                == 0 as libc::c_int
            {
                current_block = 12800627514080957624;
                break;
            }
            j = j.wrapping_add(1);
            j;
        }
    }
    match current_block {
        12800627514080957624 => {
            __gmpz_powm(
                y.as_mut_ptr(),
                a as mpz_srcptr,
                nm1dq as mpz_srcptr,
                n as mpz_srcptr,
            );
            __gmpz_sub_ui(
                y.as_mut_ptr(),
                y.as_mut_ptr() as mpz_srcptr,
                1 as libc::c_int as libc::c_ulong,
            );
            __gmpz_gcd(y.as_mut_ptr(), y.as_mut_ptr() as mpz_srcptr, n as mpz_srcptr);
            is_prime = ((if 0 != 0 && 1 as libc::c_int == 0 as libc::c_int {
                (if (*y.as_mut_ptr())._mp_size < 0 as libc::c_int {
                    -(1 as libc::c_int)
                } else {
                    ((*y.as_mut_ptr())._mp_size > 0 as libc::c_int) as libc::c_int
                })
            } else {
                __gmpz_cmp_ui(
                    y.as_mut_ptr() as mpz_srcptr,
                    1 as libc::c_int as libc::c_ulong,
                )
            }) == 0 as libc::c_int) as libc::c_int;
        }
        _ => {}
    }
    __gmpz_clear(r.as_mut_ptr());
    __gmpz_clear(y.as_mut_ptr());
    return is_prime;
}
pub unsafe extern "C" fn _nettle_generate_pocklington_prime(
    mut p: *mut __mpz_struct,
    mut r: *mut __mpz_struct,
    mut bits: libc::c_uint,
    mut top_bits_set: libc::c_int,
    mut ctx: *mut libc::c_void,
    mut random: Option::<nettle_random_func>,
    mut p0: *const __mpz_struct,
    mut q: *const __mpz_struct,
    mut p0q: *const __mpz_struct,
) {
    let mut r_min: mpz_t = [__mpz_struct {
        _mp_alloc: 0,
        _mp_size: 0,
        _mp_d: 0 as *mut mp_limb_t,
    }; 1];
    let mut r_range: mpz_t = [__mpz_struct {
        _mp_alloc: 0,
        _mp_size: 0,
        _mp_d: 0 as *mut mp_limb_t,
    }; 1];
    let mut pm1: mpz_t = [__mpz_struct {
        _mp_alloc: 0,
        _mp_size: 0,
        _mp_d: 0 as *mut mp_limb_t,
    }; 1];
    let mut a: mpz_t = [__mpz_struct {
        _mp_alloc: 0,
        _mp_size: 0,
        _mp_d: 0 as *mut mp_limb_t,
    }; 1];
    let mut e: mpz_t = [__mpz_struct {
        _mp_alloc: 0,
        _mp_size: 0,
        _mp_d: 0 as *mut mp_limb_t,
    }; 1];
    let mut need_square_test: libc::c_int = 0;
    let mut p0_bits: libc::c_uint = 0;
    let mut x: mpz_t = [__mpz_struct {
        _mp_alloc: 0,
        _mp_size: 0,
        _mp_d: 0 as *mut mp_limb_t,
    }; 1];
    let mut y: mpz_t = [__mpz_struct {
        _mp_alloc: 0,
        _mp_size: 0,
        _mp_d: 0 as *mut mp_limb_t,
    }; 1];
    let mut p04: mpz_t = [__mpz_struct {
        _mp_alloc: 0,
        _mp_size: 0,
        _mp_d: 0 as *mut mp_limb_t,
    }; 1];
    p0_bits = __gmpz_sizeinbase(p0, 2 as libc::c_int) as libc::c_uint;
    if bits <= (3 as libc::c_int as libc::c_uint).wrapping_mul(p0_bits) {} else {
        __assert_fail(
            b"bits <= 3*p0_bits\0" as *const u8 as *const libc::c_char,
            b"bignum-random-prime.c\0" as *const u8 as *const libc::c_char,
            344 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 187],
                &[libc::c_char; 187],
            >(
                b"void _nettle_generate_pocklington_prime(__mpz_struct *, __mpz_struct *, unsigned int, int, void *, nettle_random_func *, const __mpz_struct *, const __mpz_struct *, const __mpz_struct *)\0",
            ))
                .as_ptr(),
        );
    }
    'c_5480: {
        if bits <= (3 as libc::c_int as libc::c_uint).wrapping_mul(p0_bits) {} else {
            __assert_fail(
                b"bits <= 3*p0_bits\0" as *const u8 as *const libc::c_char,
                b"bignum-random-prime.c\0" as *const u8 as *const libc::c_char,
                344 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 187],
                    &[libc::c_char; 187],
                >(
                    b"void _nettle_generate_pocklington_prime(__mpz_struct *, __mpz_struct *, unsigned int, int, void *, nettle_random_func *, const __mpz_struct *, const __mpz_struct *, const __mpz_struct *)\0",
                ))
                    .as_ptr(),
            );
        }
    };
    if bits > p0_bits {} else {
        __assert_fail(
            b"bits > p0_bits\0" as *const u8 as *const libc::c_char,
            b"bignum-random-prime.c\0" as *const u8 as *const libc::c_char,
            345 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 187],
                &[libc::c_char; 187],
            >(
                b"void _nettle_generate_pocklington_prime(__mpz_struct *, __mpz_struct *, unsigned int, int, void *, nettle_random_func *, const __mpz_struct *, const __mpz_struct *, const __mpz_struct *)\0",
            ))
                .as_ptr(),
        );
    }
    'c_5441: {
        if bits > p0_bits {} else {
            __assert_fail(
                b"bits > p0_bits\0" as *const u8 as *const libc::c_char,
                b"bignum-random-prime.c\0" as *const u8 as *const libc::c_char,
                345 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 187],
                    &[libc::c_char; 187],
                >(
                    b"void _nettle_generate_pocklington_prime(__mpz_struct *, __mpz_struct *, unsigned int, int, void *, nettle_random_func *, const __mpz_struct *, const __mpz_struct *, const __mpz_struct *)\0",
                ))
                    .as_ptr(),
            );
        }
    };
    need_square_test = (bits > (2 as libc::c_int as libc::c_uint).wrapping_mul(p0_bits))
        as libc::c_int;
    __gmpz_init(r_min.as_mut_ptr());
    __gmpz_init(r_range.as_mut_ptr());
    __gmpz_init(pm1.as_mut_ptr());
    __gmpz_init(a.as_mut_ptr());
    if need_square_test != 0 {
        __gmpz_init(x.as_mut_ptr());
        __gmpz_init(y.as_mut_ptr());
        __gmpz_init(p04.as_mut_ptr());
        __gmpz_mul_2exp(p04.as_mut_ptr(), p0, 2 as libc::c_int as mp_bitcnt_t);
    }
    if !q.is_null() {
        __gmpz_init(e.as_mut_ptr());
    }
    if top_bits_set != 0 {
        __gmpz_set_ui(r_min.as_mut_ptr(), 1 as libc::c_int as libc::c_ulong);
        __gmpz_mul_2exp(
            r_min.as_mut_ptr(),
            r_min.as_mut_ptr() as mpz_srcptr,
            bits.wrapping_sub(3 as libc::c_int as libc::c_uint) as mp_bitcnt_t,
        );
        __gmpz_fdiv_q(r_min.as_mut_ptr(), r_min.as_mut_ptr() as mpz_srcptr, p0q);
        __gmpz_sub_ui(
            r_range.as_mut_ptr(),
            r_min.as_mut_ptr() as mpz_srcptr,
            2 as libc::c_int as libc::c_ulong,
        );
        __gmpz_mul_ui(
            r_min.as_mut_ptr(),
            r_min.as_mut_ptr() as mpz_srcptr,
            3 as libc::c_int as libc::c_ulong,
        );
        __gmpz_add_ui(
            r_min.as_mut_ptr(),
            r_min.as_mut_ptr() as mpz_srcptr,
            3 as libc::c_int as libc::c_ulong,
        );
    } else {
        __gmpz_set_ui(r_range.as_mut_ptr(), 1 as libc::c_int as libc::c_ulong);
        __gmpz_mul_2exp(
            r_range.as_mut_ptr(),
            r_range.as_mut_ptr() as mpz_srcptr,
            bits.wrapping_sub(2 as libc::c_int as libc::c_uint) as mp_bitcnt_t,
        );
        __gmpz_fdiv_q(r_range.as_mut_ptr(), r_range.as_mut_ptr() as mpz_srcptr, p0q);
        __gmpz_add_ui(
            r_min.as_mut_ptr(),
            r_range.as_mut_ptr() as mpz_srcptr,
            1 as libc::c_int as libc::c_ulong,
        );
    }
    loop {
        let mut buf: [uint8_t; 1] = [0; 1];
        nettle_mpz_random(r, ctx, random, r_range.as_mut_ptr() as *const __mpz_struct);
        __gmpz_add(r, r as mpz_srcptr, r_min.as_mut_ptr() as mpz_srcptr);
        __gmpz_mul_2exp(r, r as mpz_srcptr, 1 as libc::c_int as mp_bitcnt_t);
        __gmpz_mul(pm1.as_mut_ptr(), r as mpz_srcptr, p0q);
        __gmpz_add_ui(
            p,
            pm1.as_mut_ptr() as mpz_srcptr,
            1 as libc::c_int as libc::c_ulong,
        );
        if __gmpz_sizeinbase(p as mpz_srcptr, 2 as libc::c_int) == bits as libc::c_ulong
        {} else {
            __assert_fail(
                b"mpz_sizeinbase(p, 2) == bits\0" as *const u8 as *const libc::c_char,
                b"bignum-random-prime.c\0" as *const u8 as *const libc::c_char,
                397 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 187],
                    &[libc::c_char; 187],
                >(
                    b"void _nettle_generate_pocklington_prime(__mpz_struct *, __mpz_struct *, unsigned int, int, void *, nettle_random_func *, const __mpz_struct *, const __mpz_struct *, const __mpz_struct *)\0",
                ))
                    .as_ptr(),
            );
        }
        'c_5142: {
            if __gmpz_sizeinbase(p as mpz_srcptr, 2 as libc::c_int)
                == bits as libc::c_ulong
            {} else {
                __assert_fail(
                    b"mpz_sizeinbase(p, 2) == bits\0" as *const u8
                        as *const libc::c_char,
                    b"bignum-random-prime.c\0" as *const u8 as *const libc::c_char,
                    397 as libc::c_int as libc::c_uint,
                    (*::std::mem::transmute::<
                        &[u8; 187],
                        &[libc::c_char; 187],
                    >(
                        b"void _nettle_generate_pocklington_prime(__mpz_struct *, __mpz_struct *, unsigned int, int, void *, nettle_random_func *, const __mpz_struct *, const __mpz_struct *, const __mpz_struct *)\0",
                    ))
                        .as_ptr(),
                );
            }
        };
        if __gmpz_probab_prime_p(p as mpz_srcptr, 1 as libc::c_int) == 0 {
            continue;
        }
        random
            .unwrap()(
            ctx,
            ::std::mem::size_of::<[uint8_t; 1]>() as libc::c_ulong,
            buf.as_mut_ptr(),
        );
        __gmpz_set_ui(
            a.as_mut_ptr(),
            (buf[0 as libc::c_int as usize] as libc::c_int + 2 as libc::c_int)
                as libc::c_ulong,
        );
        if !q.is_null() {
            __gmpz_mul(e.as_mut_ptr(), r as mpz_srcptr, q);
            if miller_rabin_pocklington(
                p,
                pm1.as_mut_ptr(),
                e.as_mut_ptr(),
                a.as_mut_ptr(),
            ) == 0
            {
                continue;
            }
            if !(need_square_test != 0) {
                break;
            }
            __gmpz_tdiv_qr(
                x.as_mut_ptr(),
                y.as_mut_ptr(),
                e.as_mut_ptr() as mpz_srcptr,
                p04.as_mut_ptr() as mpz_srcptr,
            );
        } else {
            if miller_rabin_pocklington(p, pm1.as_mut_ptr(), r, a.as_mut_ptr()) == 0 {
                continue;
            }
            if !(need_square_test != 0) {
                break;
            }
            __gmpz_tdiv_qr(
                x.as_mut_ptr(),
                y.as_mut_ptr(),
                r as mpz_srcptr,
                p04.as_mut_ptr() as mpz_srcptr,
            );
        }
        __gmpz_mul(
            y.as_mut_ptr(),
            y.as_mut_ptr() as mpz_srcptr,
            y.as_mut_ptr() as mpz_srcptr,
        );
        __gmpz_submul_ui(
            y.as_mut_ptr(),
            x.as_mut_ptr() as mpz_srcptr,
            16 as libc::c_int as libc::c_ulong,
        );
        if !(__gmpz_perfect_square_p(y.as_mut_ptr() as mpz_srcptr) != 0) {
            break;
        }
    }
    __gmpz_clear(r_min.as_mut_ptr());
    __gmpz_clear(r_range.as_mut_ptr());
    __gmpz_clear(pm1.as_mut_ptr());
    __gmpz_clear(a.as_mut_ptr());
    if need_square_test != 0 {
        __gmpz_clear(x.as_mut_ptr());
        __gmpz_clear(y.as_mut_ptr());
        __gmpz_clear(p04.as_mut_ptr());
    }
    if !q.is_null() {
        __gmpz_clear(e.as_mut_ptr());
    }
}
pub unsafe extern "C" fn nettle_random_prime(
    mut p: *mut __mpz_struct,
    mut bits: libc::c_uint,
    mut top_bits_set: libc::c_int,
    mut random_ctx: *mut libc::c_void,
    mut random: Option::<nettle_random_func>,
    mut progress_ctx: *mut libc::c_void,
    mut progress: Option::<nettle_progress_func>,
) {
    if bits >= 3 as libc::c_int as libc::c_uint {} else {
        __assert_fail(
            b"bits >= 3\0" as *const u8 as *const libc::c_char,
            b"bignum-random-prime.c\0" as *const u8 as *const libc::c_char,
            469 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 122],
                &[libc::c_char; 122],
            >(
                b"void nettle_random_prime(__mpz_struct *, unsigned int, int, void *, nettle_random_func *, void *, nettle_progress_func *)\0",
            ))
                .as_ptr(),
        );
    }
    'c_7625: {
        if bits >= 3 as libc::c_int as libc::c_uint {} else {
            __assert_fail(
                b"bits >= 3\0" as *const u8 as *const libc::c_char,
                b"bignum-random-prime.c\0" as *const u8 as *const libc::c_char,
                469 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 122],
                    &[libc::c_char; 122],
                >(
                    b"void nettle_random_prime(__mpz_struct *, unsigned int, int, void *, nettle_random_func *, void *, nettle_progress_func *)\0",
                ))
                    .as_ptr(),
            );
        }
    };
    if bits <= 10 as libc::c_int as libc::c_uint {
        let mut first: libc::c_uint = 0;
        let mut choices: libc::c_uint = 0;
        let mut buf: uint8_t = 0;
        if top_bits_set == 0 {} else {
            __assert_fail(
                b"!top_bits_set\0" as *const u8 as *const libc::c_char,
                b"bignum-random-prime.c\0" as *const u8 as *const libc::c_char,
                476 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 122],
                    &[libc::c_char; 122],
                >(
                    b"void nettle_random_prime(__mpz_struct *, unsigned int, int, void *, nettle_random_func *, void *, nettle_progress_func *)\0",
                ))
                    .as_ptr(),
            );
        }
        'c_7587: {
            if top_bits_set == 0 {} else {
                __assert_fail(
                    b"!top_bits_set\0" as *const u8 as *const libc::c_char,
                    b"bignum-random-prime.c\0" as *const u8 as *const libc::c_char,
                    476 as libc::c_int as libc::c_uint,
                    (*::std::mem::transmute::<
                        &[u8; 122],
                        &[libc::c_char; 122],
                    >(
                        b"void nettle_random_prime(__mpz_struct *, unsigned int, int, void *, nettle_random_func *, void *, nettle_progress_func *)\0",
                    ))
                        .as_ptr(),
                );
            }
        };
        random
            .unwrap()(
            random_ctx,
            ::std::mem::size_of::<uint8_t>() as libc::c_ulong,
            &mut buf,
        );
        first = prime_by_size[bits.wrapping_sub(3 as libc::c_int as libc::c_uint)
            as usize] as libc::c_uint;
        choices = (prime_by_size[bits.wrapping_sub(2 as libc::c_int as libc::c_uint)
            as usize] as libc::c_uint)
            .wrapping_sub(first);
        __gmpz_set_ui(
            p,
            primes[first.wrapping_add((buf as libc::c_uint).wrapping_rem(choices))
                as usize] as libc::c_ulong,
        );
    } else if bits <= 20 as libc::c_int as libc::c_uint {
        let mut highbit: libc::c_ulong = 0;
        let mut buf_0: [uint8_t; 3] = [0; 3];
        let mut x: libc::c_ulong = 0;
        let mut j: libc::c_uint = 0;
        if top_bits_set == 0 {} else {
            __assert_fail(
                b"!top_bits_set\0" as *const u8 as *const libc::c_char,
                b"bignum-random-prime.c\0" as *const u8 as *const libc::c_char,
                492 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 122],
                    &[libc::c_char; 122],
                >(
                    b"void nettle_random_prime(__mpz_struct *, unsigned int, int, void *, nettle_random_func *, void *, nettle_progress_func *)\0",
                ))
                    .as_ptr(),
            );
        }
        'c_7110: {
            if top_bits_set == 0 {} else {
                __assert_fail(
                    b"!top_bits_set\0" as *const u8 as *const libc::c_char,
                    b"bignum-random-prime.c\0" as *const u8 as *const libc::c_char,
                    492 as libc::c_int as libc::c_uint,
                    (*::std::mem::transmute::<
                        &[u8; 122],
                        &[libc::c_char; 122],
                    >(
                        b"void nettle_random_prime(__mpz_struct *, unsigned int, int, void *, nettle_random_func *, void *, nettle_progress_func *)\0",
                    ))
                        .as_ptr(),
                );
            }
        };
        highbit = ((1 as libc::c_long)
            << bits.wrapping_sub(1 as libc::c_int as libc::c_uint)) as libc::c_ulong;
        '_again: loop {
            random
                .unwrap()(
                random_ctx,
                ::std::mem::size_of::<[uint8_t; 3]>() as libc::c_ulong,
                buf_0.as_mut_ptr(),
            );
            x = ((buf_0[0 as libc::c_int as usize] as uint32_t) << 16 as libc::c_int
                | (buf_0[1 as libc::c_int as usize] as uint32_t) << 8 as libc::c_int
                | buf_0[2 as libc::c_int as usize] as uint32_t) as libc::c_ulong;
            x &= highbit.wrapping_sub(1 as libc::c_int as libc::c_ulong);
            x |= highbit | 1 as libc::c_int as libc::c_ulong;
            j = 0 as libc::c_int as libc::c_uint;
            loop {
                if !(prime_square[j as usize] as libc::c_ulong <= x) {
                    break '_again;
                }
                let mut q: libc::c_uint = (x
                    .wrapping_mul(trial_div_table[j as usize].inverse as libc::c_ulong)
                    & (((1 as libc::c_int) << 20 as libc::c_int) - 1 as libc::c_int)
                        as libc::c_ulong) as libc::c_uint;
                if q <= trial_div_table[j as usize].limit {
                    break;
                }
                j = j.wrapping_add(1);
                j;
            }
        }
        __gmpz_set_ui(p, x);
    } else {
        let mut q_0: mpz_t = [__mpz_struct {
            _mp_alloc: 0,
            _mp_size: 0,
            _mp_d: 0 as *mut mp_limb_t,
        }; 1];
        let mut r: mpz_t = [__mpz_struct {
            _mp_alloc: 0,
            _mp_size: 0,
            _mp_d: 0 as *mut mp_limb_t,
        }; 1];
        __gmpz_init(q_0.as_mut_ptr());
        __gmpz_init(r.as_mut_ptr());
        nettle_random_prime(
            q_0.as_mut_ptr(),
            bits
                .wrapping_add(3 as libc::c_int as libc::c_uint)
                .wrapping_div(2 as libc::c_int as libc::c_uint),
            0 as libc::c_int,
            random_ctx,
            random,
            progress_ctx,
            progress,
        );
        _nettle_generate_pocklington_prime(
            p,
            r.as_mut_ptr(),
            bits,
            top_bits_set,
            random_ctx,
            random,
            q_0.as_mut_ptr() as *const __mpz_struct,
            0 as *const __mpz_struct,
            q_0.as_mut_ptr() as *const __mpz_struct,
        );
        if progress.is_some() {
            progress.unwrap()(progress_ctx, 'x' as i32);
        }
        __gmpz_clear(q_0.as_mut_ptr());
        __gmpz_clear(r.as_mut_ptr());
    };
}
