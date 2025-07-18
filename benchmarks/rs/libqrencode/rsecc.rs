use ::libc;
extern "C" {
    fn memmove(
        _: *mut libc::c_void,
        _: *const libc::c_void,
        _: libc::c_ulong,
    ) -> *mut libc::c_void;
    fn memset(
        _: *mut libc::c_void,
        _: libc::c_int,
        _: libc::c_ulong,
    ) -> *mut libc::c_void;
    fn pthread_mutex_lock(__mutex: *mut pthread_mutex_t) -> libc::c_int;
    fn pthread_mutex_unlock(__mutex: *mut pthread_mutex_t) -> libc::c_int;
}
pub type size_t = libc::c_ulong;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct __pthread_internal_list {
    pub __prev: *mut __pthread_internal_list,
    pub __next: *mut __pthread_internal_list,
}
pub type __pthread_list_t = __pthread_internal_list;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct __pthread_mutex_s {
    pub __lock: libc::c_int,
    pub __count: libc::c_uint,
    pub __owner: libc::c_int,
    pub __nusers: libc::c_uint,
    pub __kind: libc::c_int,
    pub __spins: libc::c_short,
    pub __elision: libc::c_short,
    pub __list: __pthread_list_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union pthread_mutex_t {
    pub __data: __pthread_mutex_s,
    pub __size: [libc::c_char; 40],
    pub __align: libc::c_long,
}
pub type C2RustUnnamed = libc::c_uint;
pub const PTHREAD_MUTEX_DEFAULT: C2RustUnnamed = 0;
pub const PTHREAD_MUTEX_ERRORCHECK: C2RustUnnamed = 2;
pub const PTHREAD_MUTEX_RECURSIVE: C2RustUnnamed = 1;
pub const PTHREAD_MUTEX_NORMAL: C2RustUnnamed = 0;
pub const PTHREAD_MUTEX_ADAPTIVE_NP: C2RustUnnamed = 3;
pub const PTHREAD_MUTEX_ERRORCHECK_NP: C2RustUnnamed = 2;
pub const PTHREAD_MUTEX_RECURSIVE_NP: C2RustUnnamed = 1;
pub const PTHREAD_MUTEX_TIMED_NP: C2RustUnnamed = 0;
static mut RSECC_mutex: pthread_mutex_t = pthread_mutex_t {
    __data: {
        let mut init = __pthread_mutex_s {
            __lock: 0 as libc::c_int,
            __count: 0 as libc::c_int as libc::c_uint,
            __owner: 0 as libc::c_int,
            __nusers: 0 as libc::c_int as libc::c_uint,
            __kind: PTHREAD_MUTEX_TIMED_NP as libc::c_int,
            __spins: 0 as libc::c_int as libc::c_short,
            __elision: 0 as libc::c_int as libc::c_short,
            __list: {
                let mut init = __pthread_internal_list {
                    __prev: 0 as *const __pthread_internal_list
                        as *mut __pthread_internal_list,
                    __next: 0 as *const __pthread_internal_list
                        as *mut __pthread_internal_list,
                };
                init
            },
        };
        init
    },
};
static mut initialized: libc::c_int = 0 as libc::c_int;
static mut proot: libc::c_uint = 0x11d as libc::c_int as libc::c_uint;
static mut alpha: [libc::c_uchar; 256] = [0; 256];
static mut aindex: [libc::c_uchar; 256] = [0; 256];
static mut generator: [[libc::c_uchar; 31]; 29] = [[0; 31]; 29];
static mut generatorInitialized: [libc::c_uchar; 29] = [0; 29];
unsafe extern "C" fn RSECC_initLookupTable() {
    let mut i: libc::c_uint = 0;
    let mut b: libc::c_uint = 0;
    alpha[((1 as libc::c_uint) << 8 as libc::c_int)
        .wrapping_sub(1 as libc::c_int as libc::c_uint)
        as usize] = 0 as libc::c_int as libc::c_uchar;
    aindex[0 as libc::c_int
        as usize] = ((1 as libc::c_uint) << 8 as libc::c_int)
        .wrapping_sub(1 as libc::c_int as libc::c_uint) as libc::c_uchar;
    b = 1 as libc::c_int as libc::c_uint;
    i = 0 as libc::c_int as libc::c_uint;
    while i
        < ((1 as libc::c_uint) << 8 as libc::c_int)
            .wrapping_sub(1 as libc::c_int as libc::c_uint)
    {
        alpha[i as usize] = b as libc::c_uchar;
        aindex[b as usize] = i as libc::c_uchar;
        b <<= 1 as libc::c_int;
        if b
            & ((1 as libc::c_uint) << 8 as libc::c_int)
                .wrapping_sub(1 as libc::c_int as libc::c_uint)
                .wrapping_add(1 as libc::c_int as libc::c_uint) != 0
        {
            b ^= proot;
        }
        b
            &= ((1 as libc::c_uint) << 8 as libc::c_int)
                .wrapping_sub(1 as libc::c_int as libc::c_uint);
        i = i.wrapping_add(1);
        i;
    }
}
unsafe extern "C" fn RSECC_init() {
    RSECC_initLookupTable();
    memset(
        generatorInitialized.as_mut_ptr() as *mut libc::c_void,
        0 as libc::c_int,
        (30 as libc::c_int - 2 as libc::c_int + 1 as libc::c_int) as libc::c_ulong,
    );
    initialized = 1 as libc::c_int;
}
unsafe extern "C" fn generator_init(mut length: size_t) {
    let mut i: size_t = 0;
    let mut j: size_t = 0;
    let mut g: [libc::c_int; 31] = [0; 31];
    g[0 as libc::c_int as usize] = 1 as libc::c_int;
    i = 0 as libc::c_int as size_t;
    while i < length {
        g[i.wrapping_add(1 as libc::c_int as libc::c_ulong) as usize] = 1 as libc::c_int;
        j = i;
        while j > 0 as libc::c_int as libc::c_ulong {
            g[j
                as usize] = g[j.wrapping_sub(1 as libc::c_int as libc::c_ulong) as usize]
                ^ alpha[(aindex[g[j as usize] as usize] as libc::c_ulong)
                    .wrapping_add(i)
                    .wrapping_rem(
                        ((1 as libc::c_uint) << 8 as libc::c_int)
                            .wrapping_sub(1 as libc::c_int as libc::c_uint)
                            as libc::c_ulong,
                    ) as usize] as libc::c_int;
            j = j.wrapping_sub(1);
            j;
        }
        g[0 as libc::c_int
            as usize] = alpha[(aindex[g[0 as libc::c_int as usize] as usize]
            as libc::c_ulong)
            .wrapping_add(i)
            .wrapping_rem(
                ((1 as libc::c_uint) << 8 as libc::c_int)
                    .wrapping_sub(1 as libc::c_int as libc::c_uint) as libc::c_ulong,
            ) as usize] as libc::c_int;
        i = i.wrapping_add(1);
        i;
    }
    i = 0 as libc::c_int as size_t;
    while i <= length {
        generator[length.wrapping_sub(2 as libc::c_int as libc::c_ulong)
            as usize][i as usize] = aindex[g[i as usize] as usize];
        i = i.wrapping_add(1);
        i;
    }
    generatorInitialized[length.wrapping_sub(2 as libc::c_int as libc::c_ulong)
        as usize] = 1 as libc::c_int as libc::c_uchar;
}
pub unsafe extern "C" fn RSECC_encode(
    mut data_length: size_t,
    mut ecc_length: size_t,
    mut data: *const libc::c_uchar,
    mut ecc: *mut libc::c_uchar,
) -> libc::c_int {
    let mut i: size_t = 0;
    let mut j: size_t = 0;
    let mut feedback: libc::c_uchar = 0;
    let mut gen: *mut libc::c_uchar = 0 as *mut libc::c_uchar;
    pthread_mutex_lock(&mut RSECC_mutex);
    if initialized == 0 {
        RSECC_init();
    }
    pthread_mutex_unlock(&mut RSECC_mutex);
    if ecc_length > 30 as libc::c_int as libc::c_ulong {
        return -(1 as libc::c_int);
    }
    memset(ecc as *mut libc::c_void, 0 as libc::c_int, ecc_length);
    pthread_mutex_lock(&mut RSECC_mutex);
    if generatorInitialized[ecc_length.wrapping_sub(2 as libc::c_int as libc::c_ulong)
        as usize] == 0
    {
        generator_init(ecc_length);
    }
    pthread_mutex_unlock(&mut RSECC_mutex);
    gen = (generator[ecc_length.wrapping_sub(2 as libc::c_int as libc::c_ulong)
        as usize])
        .as_mut_ptr();
    i = 0 as libc::c_int as size_t;
    while i < data_length {
        feedback = aindex[(*data.offset(i as isize) as libc::c_int
            ^ *ecc.offset(0 as libc::c_int as isize) as libc::c_int) as usize];
        if feedback as libc::c_uint
            != ((1 as libc::c_uint) << 8 as libc::c_int)
                .wrapping_sub(1 as libc::c_int as libc::c_uint)
        {
            j = 1 as libc::c_int as size_t;
            while j < ecc_length {
                let ref mut fresh0 = *ecc.offset(j as isize);
                *fresh0 = (*fresh0 as libc::c_int
                    ^ alpha[((feedback as libc::c_int
                        + *gen.offset(ecc_length.wrapping_sub(j) as isize)
                            as libc::c_int) as libc::c_uint)
                        .wrapping_rem(
                            ((1 as libc::c_uint) << 8 as libc::c_int)
                                .wrapping_sub(1 as libc::c_int as libc::c_uint),
                        ) as usize] as libc::c_int) as libc::c_uchar;
                j = j.wrapping_add(1);
                j;
            }
        }
        memmove(
            &mut *ecc.offset(0 as libc::c_int as isize) as *mut libc::c_uchar
                as *mut libc::c_void,
            &mut *ecc.offset(1 as libc::c_int as isize) as *mut libc::c_uchar
                as *const libc::c_void,
            ecc_length.wrapping_sub(1 as libc::c_int as libc::c_ulong),
        );
        if feedback as libc::c_uint
            != ((1 as libc::c_uint) << 8 as libc::c_int)
                .wrapping_sub(1 as libc::c_int as libc::c_uint)
        {
            *ecc
                .offset(
                    ecc_length.wrapping_sub(1 as libc::c_int as libc::c_ulong) as isize,
                ) = alpha[((feedback as libc::c_int
                + *gen.offset(0 as libc::c_int as isize) as libc::c_int) as libc::c_uint)
                .wrapping_rem(
                    ((1 as libc::c_uint) << 8 as libc::c_int)
                        .wrapping_sub(1 as libc::c_int as libc::c_uint),
                ) as usize];
        } else {
            *ecc
                .offset(
                    ecc_length.wrapping_sub(1 as libc::c_int as libc::c_ulong) as isize,
                ) = 0 as libc::c_int as libc::c_uchar;
        }
        i = i.wrapping_add(1);
        i;
    }
    return 0 as libc::c_int;
}
