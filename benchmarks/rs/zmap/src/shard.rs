use ::libc;
extern "C" {
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
    fn blocklist_lookup_index(index: uint64_t) -> uint32_t;
    static mut zsend: state_send;
}
pub type __uint8_t = libc::c_uchar;
pub type __uint16_t = libc::c_ushort;
pub type __uint32_t = libc::c_uint;
pub type __uint64_t = libc::c_ulong;
pub type uint8_t = __uint8_t;
pub type uint16_t = __uint16_t;
pub type uint32_t = __uint32_t;
pub type uint64_t = __uint64_t;
pub type size_t = libc::c_ulong;
pub type mp_limb_t = libc::c_ulong;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct __mpz_struct {
    pub _mp_alloc: libc::c_int,
    pub _mp_size: libc::c_int,
    pub _mp_d: *mut mp_limb_t,
}
pub type mpz_t = [__mpz_struct; 1];
pub type mp_ptr = *mut mp_limb_t;
pub type mp_size_t = libc::c_long;
pub type mpz_srcptr = *const __mpz_struct;
pub type mpz_ptr = *mut __mpz_struct;
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
pub type shard_complete_cb = Option::<
    unsafe extern "C" fn(uint8_t, *mut libc::c_void) -> (),
>;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct shard {
    pub state: shard_state,
    pub params: shard_params,
    pub current: uint64_t,
    pub iterations: uint64_t,
    pub thread_id: uint8_t,
    pub cb: shard_complete_cb,
    pub arg: *mut libc::c_void,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct shard_params {
    pub first: uint64_t,
    pub last: uint64_t,
    pub factor: uint64_t,
    pub modulus: uint64_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct shard_state {
    pub packets_sent: uint64_t,
    pub hosts_scanned: uint32_t,
    pub max_hosts: uint32_t,
    pub max_packets: uint32_t,
    pub hosts_blocklisted: uint32_t,
    pub hosts_allowlisted: uint32_t,
    pub packets_failed: uint32_t,
    pub first_scanned: uint32_t,
}
pub type shard_t = shard;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct state_send {
    pub start: libc::c_double,
    pub finish: libc::c_double,
    pub packets_sent: uint64_t,
    pub hosts_scanned: uint64_t,
    pub blocklisted: uint64_t,
    pub allowlisted: uint64_t,
    pub warmup: libc::c_int,
    pub complete: libc::c_int,
    pub first_scanned: uint32_t,
    pub max_targets: uint32_t,
    pub sendto_failures: uint32_t,
    pub max_index: uint32_t,
    pub list_of_ips_pbm: *mut *mut uint8_t,
}
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
unsafe extern "C" fn shard_roll_to_valid(mut s: *mut shard_t) -> uint32_t {
    if ((*s).current).wrapping_sub(1 as libc::c_int as libc::c_ulong)
        < zsend.max_index as libc::c_ulong
    {
        return (*s).current as uint32_t;
    }
    return shard_get_next_ip(s);
}
pub unsafe extern "C" fn shard_init(
    mut shard: *mut shard_t,
    mut shard_idx: uint16_t,
    mut num_shards: uint16_t,
    mut thread_idx: uint8_t,
    mut num_threads: uint8_t,
    mut max_total_targets: uint32_t,
    mut cycle: *const cycle_t,
    mut cb: shard_complete_cb,
    mut arg: *mut libc::c_void,
) {
    if num_shards as libc::c_int > 0 as libc::c_int {} else {
        __assert_fail(
            b"num_shards > 0\0" as *const u8 as *const libc::c_char,
            b"shard.c\0" as *const u8 as *const libc::c_char,
            38 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 119],
                &[libc::c_char; 119],
            >(
                b"void shard_init(shard_t *, uint16_t, uint16_t, uint8_t, uint8_t, uint32_t, const cycle_t *, shard_complete_cb, void *)\0",
            ))
                .as_ptr(),
        );
    }
    'c_6497: {
        if num_shards as libc::c_int > 0 as libc::c_int {} else {
            __assert_fail(
                b"num_shards > 0\0" as *const u8 as *const libc::c_char,
                b"shard.c\0" as *const u8 as *const libc::c_char,
                38 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 119],
                    &[libc::c_char; 119],
                >(
                    b"void shard_init(shard_t *, uint16_t, uint16_t, uint8_t, uint8_t, uint32_t, const cycle_t *, shard_complete_cb, void *)\0",
                ))
                    .as_ptr(),
            );
        }
    };
    if num_threads as libc::c_int > 0 as libc::c_int {} else {
        __assert_fail(
            b"num_threads > 0\0" as *const u8 as *const libc::c_char,
            b"shard.c\0" as *const u8 as *const libc::c_char,
            39 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 119],
                &[libc::c_char; 119],
            >(
                b"void shard_init(shard_t *, uint16_t, uint16_t, uint8_t, uint8_t, uint32_t, const cycle_t *, shard_complete_cb, void *)\0",
            ))
                .as_ptr(),
        );
    }
    'c_6458: {
        if num_threads as libc::c_int > 0 as libc::c_int {} else {
            __assert_fail(
                b"num_threads > 0\0" as *const u8 as *const libc::c_char,
                b"shard.c\0" as *const u8 as *const libc::c_char,
                39 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 119],
                    &[libc::c_char; 119],
                >(
                    b"void shard_init(shard_t *, uint16_t, uint16_t, uint8_t, uint8_t, uint32_t, const cycle_t *, shard_complete_cb, void *)\0",
                ))
                    .as_ptr(),
            );
        }
    };
    if (shard_idx as libc::c_int) < num_shards as libc::c_int {} else {
        __assert_fail(
            b"shard_idx < num_shards\0" as *const u8 as *const libc::c_char,
            b"shard.c\0" as *const u8 as *const libc::c_char,
            40 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 119],
                &[libc::c_char; 119],
            >(
                b"void shard_init(shard_t *, uint16_t, uint16_t, uint8_t, uint8_t, uint32_t, const cycle_t *, shard_complete_cb, void *)\0",
            ))
                .as_ptr(),
        );
    }
    'c_6415: {
        if (shard_idx as libc::c_int) < num_shards as libc::c_int {} else {
            __assert_fail(
                b"shard_idx < num_shards\0" as *const u8 as *const libc::c_char,
                b"shard.c\0" as *const u8 as *const libc::c_char,
                40 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 119],
                    &[libc::c_char; 119],
                >(
                    b"void shard_init(shard_t *, uint16_t, uint16_t, uint8_t, uint8_t, uint32_t, const cycle_t *, shard_complete_cb, void *)\0",
                ))
                    .as_ptr(),
            );
        }
    };
    if (thread_idx as libc::c_int) < num_threads as libc::c_int {} else {
        __assert_fail(
            b"thread_idx < num_threads\0" as *const u8 as *const libc::c_char,
            b"shard.c\0" as *const u8 as *const libc::c_char,
            41 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 119],
                &[libc::c_char; 119],
            >(
                b"void shard_init(shard_t *, uint16_t, uint16_t, uint8_t, uint8_t, uint32_t, const cycle_t *, shard_complete_cb, void *)\0",
            ))
                .as_ptr(),
        );
    }
    'c_6373: {
        if (thread_idx as libc::c_int) < num_threads as libc::c_int {} else {
            __assert_fail(
                b"thread_idx < num_threads\0" as *const u8 as *const libc::c_char,
                b"shard.c\0" as *const u8 as *const libc::c_char,
                41 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 119],
                    &[libc::c_char; 119],
                >(
                    b"void shard_init(shard_t *, uint16_t, uint16_t, uint8_t, uint8_t, uint32_t, const cycle_t *, shard_complete_cb, void *)\0",
                ))
                    .as_ptr(),
            );
        }
    };
    let mut num_subshards: uint32_t = (num_shards as uint32_t)
        .wrapping_mul(num_threads as uint32_t);
    let mut num_elts: uint64_t = (*cycle).order;
    if (num_subshards as libc::c_ulong) < num_elts {} else {
        __assert_fail(
            b"num_subshards < num_elts\0" as *const u8 as *const libc::c_char,
            b"shard.c\0" as *const u8 as *const libc::c_char,
            44 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 119],
                &[libc::c_char; 119],
            >(
                b"void shard_init(shard_t *, uint16_t, uint16_t, uint8_t, uint8_t, uint32_t, const cycle_t *, shard_complete_cb, void *)\0",
            ))
                .as_ptr(),
        );
    }
    'c_6332: {
        if (num_subshards as libc::c_ulong) < num_elts {} else {
            __assert_fail(
                b"num_subshards < num_elts\0" as *const u8 as *const libc::c_char,
                b"shard.c\0" as *const u8 as *const libc::c_char,
                44 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 119],
                    &[libc::c_char; 119],
                >(
                    b"void shard_init(shard_t *, uint16_t, uint16_t, uint8_t, uint8_t, uint32_t, const cycle_t *, shard_complete_cb, void *)\0",
                ))
                    .as_ptr(),
            );
        }
    };
    if max_total_targets == 0 || num_subshards <= max_total_targets {} else {
        __assert_fail(
            b"!max_total_targets || (num_subshards <= max_total_targets)\0" as *const u8
                as *const libc::c_char,
            b"shard.c\0" as *const u8 as *const libc::c_char,
            45 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 119],
                &[libc::c_char; 119],
            >(
                b"void shard_init(shard_t *, uint16_t, uint16_t, uint8_t, uint8_t, uint32_t, const cycle_t *, shard_complete_cb, void *)\0",
            ))
                .as_ptr(),
        );
    }
    'c_6279: {
        if max_total_targets == 0 || num_subshards <= max_total_targets {} else {
            __assert_fail(
                b"!max_total_targets || (num_subshards <= max_total_targets)\0"
                    as *const u8 as *const libc::c_char,
                b"shard.c\0" as *const u8 as *const libc::c_char,
                45 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 119],
                    &[libc::c_char; 119],
                >(
                    b"void shard_init(shard_t *, uint16_t, uint16_t, uint8_t, uint8_t, uint32_t, const cycle_t *, shard_complete_cb, void *)\0",
                ))
                    .as_ptr(),
            );
        }
    };
    let mut sub_idx: uint32_t = (shard_idx as libc::c_int * num_threads as libc::c_int
        + thread_idx as libc::c_int) as uint32_t;
    let mut exponent_begin: uint64_t = num_elts
        .wrapping_div(num_subshards as libc::c_ulong)
        .wrapping_mul(sub_idx as libc::c_ulong);
    let mut exponent_end: uint64_t = num_elts
        .wrapping_div(num_subshards as libc::c_ulong)
        .wrapping_mul(
            sub_idx
                .wrapping_add(1 as libc::c_int as libc::c_uint)
                .wrapping_rem(num_subshards) as libc::c_ulong,
        );
    exponent_begin = exponent_begin
        .wrapping_add((*cycle).offset as libc::c_ulong)
        .wrapping_rem(num_elts);
    exponent_end = exponent_end
        .wrapping_add((*cycle).offset as libc::c_ulong)
        .wrapping_rem(num_elts);
    let mut generator_m: mpz_t = [__mpz_struct {
        _mp_alloc: 0,
        _mp_size: 0,
        _mp_d: 0 as *mut mp_limb_t,
    }; 1];
    let mut exponent_begin_m: mpz_t = [__mpz_struct {
        _mp_alloc: 0,
        _mp_size: 0,
        _mp_d: 0 as *mut mp_limb_t,
    }; 1];
    let mut exponent_end_m: mpz_t = [__mpz_struct {
        _mp_alloc: 0,
        _mp_size: 0,
        _mp_d: 0 as *mut mp_limb_t,
    }; 1];
    let mut prime_m: mpz_t = [__mpz_struct {
        _mp_alloc: 0,
        _mp_size: 0,
        _mp_d: 0 as *mut mp_limb_t,
    }; 1];
    __gmpz_init_set_ui(generator_m.as_mut_ptr(), (*cycle).generator);
    __gmpz_init_set_ui(exponent_begin_m.as_mut_ptr(), exponent_begin);
    __gmpz_init_set_ui(exponent_end_m.as_mut_ptr(), exponent_end);
    __gmpz_init_set_ui(prime_m.as_mut_ptr(), (*(*cycle).group).prime);
    let mut start_m: mpz_t = [__mpz_struct {
        _mp_alloc: 0,
        _mp_size: 0,
        _mp_d: 0 as *mut mp_limb_t,
    }; 1];
    let mut stop_m: mpz_t = [__mpz_struct {
        _mp_alloc: 0,
        _mp_size: 0,
        _mp_d: 0 as *mut mp_limb_t,
    }; 1];
    __gmpz_init(start_m.as_mut_ptr());
    __gmpz_init(stop_m.as_mut_ptr());
    __gmpz_powm(
        start_m.as_mut_ptr(),
        generator_m.as_mut_ptr() as mpz_srcptr,
        exponent_begin_m.as_mut_ptr() as mpz_srcptr,
        prime_m.as_mut_ptr() as mpz_srcptr,
    );
    __gmpz_powm(
        stop_m.as_mut_ptr(),
        generator_m.as_mut_ptr() as mpz_srcptr,
        exponent_end_m.as_mut_ptr() as mpz_srcptr,
        prime_m.as_mut_ptr() as mpz_srcptr,
    );
    (*shard).params.first = __gmpz_get_ui(start_m.as_mut_ptr() as mpz_srcptr);
    (*shard).params.last = __gmpz_get_ui(stop_m.as_mut_ptr() as mpz_srcptr);
    (*shard).params.factor = (*cycle).generator;
    (*shard).params.modulus = (*(*cycle).group).prime;
    (*shard).current = (*shard).params.first;
    (*shard).thread_id = thread_idx;
    if max_total_targets > 0 as libc::c_int as libc::c_uint {
        let mut max_targets_this_shard: uint32_t = max_total_targets
            .wrapping_div(num_subshards);
        if sub_idx < max_total_targets.wrapping_rem(num_subshards) {
            max_targets_this_shard = max_targets_this_shard.wrapping_add(1);
            max_targets_this_shard;
        }
        (*shard).state.max_hosts = max_targets_this_shard;
    }
    (*shard).cb = cb;
    (*shard).arg = arg;
    shard_roll_to_valid(shard);
    __gmpz_clear(generator_m.as_mut_ptr());
    __gmpz_clear(exponent_begin_m.as_mut_ptr());
    __gmpz_clear(exponent_end_m.as_mut_ptr());
    __gmpz_clear(prime_m.as_mut_ptr());
    __gmpz_clear(start_m.as_mut_ptr());
    __gmpz_clear(stop_m.as_mut_ptr());
}
pub unsafe extern "C" fn shard_get_cur_ip(mut shard: *mut shard_t) -> uint32_t {
    return blocklist_lookup_index(
        ((*shard).current).wrapping_sub(1 as libc::c_int as libc::c_ulong),
    );
}
#[inline]
unsafe extern "C" fn shard_get_next_elem(mut shard: *mut shard_t) -> uint32_t {
    loop {
        (*shard)
            .current = ((*shard).current as libc::c_ulong)
            .wrapping_mul((*shard).params.factor) as uint64_t as uint64_t;
        (*shard)
            .current = ((*shard).current as libc::c_ulong)
            .wrapping_rem((*shard).params.modulus) as uint64_t as uint64_t;
        if !((*shard).current as libc::c_ulonglong
            >= ((1 as libc::c_longlong) << 32 as libc::c_int) as libc::c_ulonglong)
        {
            break;
        }
    }
    return (*shard).current as uint32_t;
}
pub unsafe extern "C" fn shard_get_next_ip(mut shard: *mut shard_t) -> uint32_t {
    if (*shard).current == 0 as libc::c_int as libc::c_ulong {
        return 0 as libc::c_int as uint32_t;
    }
    loop {
        let mut candidate: uint32_t = shard_get_next_elem(shard);
        if candidate as libc::c_ulong == (*shard).params.last {
            (*shard).current = 0 as libc::c_int as uint64_t;
            (*shard).iterations = ((*shard).iterations).wrapping_add(1);
            (*shard).iterations;
            return 0 as libc::c_int as uint32_t;
        }
        if candidate.wrapping_sub(1 as libc::c_int as libc::c_uint) < zsend.max_index {
            (*shard)
                .state
                .hosts_allowlisted = ((*shard).state.hosts_allowlisted).wrapping_add(1);
            (*shard).state.hosts_allowlisted;
            (*shard).iterations = ((*shard).iterations).wrapping_add(1);
            (*shard).iterations;
            return blocklist_lookup_index(
                candidate.wrapping_sub(1 as libc::c_int as libc::c_uint) as uint64_t,
            );
        }
        (*shard)
            .state
            .hosts_blocklisted = ((*shard).state.hosts_blocklisted).wrapping_add(1);
        (*shard).state.hosts_blocklisted;
    };
}
