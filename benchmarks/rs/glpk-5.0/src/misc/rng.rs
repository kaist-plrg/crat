use ::libc;
extern "C" {
    fn glp_free(ptr: *mut libc::c_void);
    fn glp_alloc(n: libc::c_int, size: libc::c_int) -> *mut libc::c_void;
    fn glp_assert_(
        expr: *const libc::c_char,
        file: *const libc::c_char,
        line: libc::c_int,
    );
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct RNG {
    pub A: [libc::c_int; 56],
    pub fptr: *mut libc::c_int,
}
unsafe extern "C" fn flip_cycle(mut rand: *mut RNG) -> libc::c_int {
    let mut ii: *mut libc::c_int = 0 as *mut libc::c_int;
    let mut jj: *mut libc::c_int = 0 as *mut libc::c_int;
    ii = &mut *((*rand).A).as_mut_ptr().offset(1 as libc::c_int as isize)
        as *mut libc::c_int;
    jj = &mut *((*rand).A).as_mut_ptr().offset(32 as libc::c_int as isize)
        as *mut libc::c_int;
    while jj
        <= &mut *((*rand).A).as_mut_ptr().offset(55 as libc::c_int as isize)
            as *mut libc::c_int
    {
        *ii = *ii - *jj & 0x7fffffff as libc::c_int;
        ii = ii.offset(1);
        ii;
        jj = jj.offset(1);
        jj;
    }
    jj = &mut *((*rand).A).as_mut_ptr().offset(1 as libc::c_int as isize)
        as *mut libc::c_int;
    while ii
        <= &mut *((*rand).A).as_mut_ptr().offset(55 as libc::c_int as isize)
            as *mut libc::c_int
    {
        *ii = *ii - *jj & 0x7fffffff as libc::c_int;
        ii = ii.offset(1);
        ii;
        jj = jj.offset(1);
        jj;
    }
    (*rand)
        .fptr = &mut *((*rand).A).as_mut_ptr().offset(54 as libc::c_int as isize)
        as *mut libc::c_int;
    return (*rand).A[55 as libc::c_int as usize];
}
pub unsafe extern "C" fn _glp_rng_create_rand() -> *mut RNG {
    let mut rand: *mut RNG = 0 as *mut RNG;
    let mut i: libc::c_int = 0;
    rand = glp_alloc(
        1 as libc::c_int,
        ::std::mem::size_of::<RNG>() as libc::c_ulong as libc::c_int,
    ) as *mut RNG;
    (*rand).A[0 as libc::c_int as usize] = -(1 as libc::c_int);
    i = 1 as libc::c_int;
    while i <= 55 as libc::c_int {
        (*rand).A[i as usize] = 0 as libc::c_int;
        i += 1;
        i;
    }
    (*rand).fptr = ((*rand).A).as_mut_ptr();
    _glp_rng_init_rand(rand, 1 as libc::c_int);
    return rand;
}
pub unsafe extern "C" fn _glp_rng_init_rand(mut rand: *mut RNG, mut seed: libc::c_int) {
    let mut i: libc::c_int = 0;
    let mut prev: libc::c_int = seed;
    let mut next: libc::c_int = 1 as libc::c_int;
    prev = prev - 0 as libc::c_int & 0x7fffffff as libc::c_int;
    seed = prev;
    (*rand).A[55 as libc::c_int as usize] = prev;
    i = 21 as libc::c_int;
    while i != 0 {
        (*rand).A[i as usize] = next;
        next = prev - next & 0x7fffffff as libc::c_int;
        if seed & 1 as libc::c_int != 0 {
            seed = 0x40000000 as libc::c_int + (seed >> 1 as libc::c_int);
        } else {
            seed >>= 1 as libc::c_int;
        }
        next = next - seed & 0x7fffffff as libc::c_int;
        prev = (*rand).A[i as usize];
        i = (i + 21 as libc::c_int) % 55 as libc::c_int;
    }
    flip_cycle(rand);
    flip_cycle(rand);
    flip_cycle(rand);
    flip_cycle(rand);
    flip_cycle(rand);
}
pub unsafe extern "C" fn _glp_rng_next_rand(mut rand: *mut RNG) -> libc::c_int {
    return if *(*rand).fptr >= 0 as libc::c_int {
        let fresh0 = (*rand).fptr;
        (*rand).fptr = ((*rand).fptr).offset(-1);
        *fresh0
    } else {
        flip_cycle(rand)
    };
}
pub unsafe extern "C" fn _glp_rng_unif_rand(
    mut rand: *mut RNG,
    mut m: libc::c_int,
) -> libc::c_int {
    let mut t: libc::c_uint = (0x80000000 as libc::c_uint)
        .wrapping_sub((0x80000000 as libc::c_uint).wrapping_rem(m as libc::c_uint));
    let mut r: libc::c_int = 0;
    (m > 0 as libc::c_int
        || {
            glp_assert_(
                b"m > 0\0" as *const u8 as *const libc::c_char,
                b"misc/rng.c\0" as *const u8 as *const libc::c_char,
                175 as libc::c_int,
            );
            1 as libc::c_int != 0
        }) as libc::c_int;
    loop {
        r = _glp_rng_next_rand(rand);
        if !(t <= r as libc::c_uint) {
            break;
        }
    }
    return r % m;
}
pub unsafe extern "C" fn _glp_rng_delete_rand(mut rand: *mut RNG) {
    glp_free(rand as *mut libc::c_void);
}
