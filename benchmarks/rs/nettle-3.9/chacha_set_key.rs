use ::libc;
extern "C" {
    fn memcpy(
        _: *mut libc::c_void,
        _: *const libc::c_void,
        _: libc::c_ulong,
    ) -> *mut libc::c_void;
}
pub type __uint8_t = libc::c_uchar;
pub type __uint32_t = libc::c_uint;
pub type uint8_t = __uint8_t;
pub type uint32_t = __uint32_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct chacha_ctx {
    pub state: [uint32_t; 16],
}
pub unsafe extern "C" fn nettle_chacha_set_key(
    mut ctx: *mut chacha_ctx,
    mut key: *const uint8_t,
) {
    static mut sigma: [uint32_t; 4] = [
        0x61707865 as libc::c_int as uint32_t,
        0x3320646e as libc::c_int as uint32_t,
        0x79622d32 as libc::c_int as uint32_t,
        0x6b206574 as libc::c_int as uint32_t,
    ];
    (*ctx)
        .state[4 as libc::c_int
        as usize] = (*key
        .offset(0 as libc::c_int as isize)
        .offset(3 as libc::c_int as isize) as uint32_t) << 24 as libc::c_int
        | (*key.offset(0 as libc::c_int as isize).offset(2 as libc::c_int as isize)
            as uint32_t) << 16 as libc::c_int
        | (*key.offset(0 as libc::c_int as isize).offset(1 as libc::c_int as isize)
            as uint32_t) << 8 as libc::c_int
        | *key.offset(0 as libc::c_int as isize).offset(0 as libc::c_int as isize)
            as uint32_t;
    (*ctx)
        .state[5 as libc::c_int
        as usize] = (*key
        .offset(4 as libc::c_int as isize)
        .offset(3 as libc::c_int as isize) as uint32_t) << 24 as libc::c_int
        | (*key.offset(4 as libc::c_int as isize).offset(2 as libc::c_int as isize)
            as uint32_t) << 16 as libc::c_int
        | (*key.offset(4 as libc::c_int as isize).offset(1 as libc::c_int as isize)
            as uint32_t) << 8 as libc::c_int
        | *key.offset(4 as libc::c_int as isize).offset(0 as libc::c_int as isize)
            as uint32_t;
    (*ctx)
        .state[6 as libc::c_int
        as usize] = (*key
        .offset(8 as libc::c_int as isize)
        .offset(3 as libc::c_int as isize) as uint32_t) << 24 as libc::c_int
        | (*key.offset(8 as libc::c_int as isize).offset(2 as libc::c_int as isize)
            as uint32_t) << 16 as libc::c_int
        | (*key.offset(8 as libc::c_int as isize).offset(1 as libc::c_int as isize)
            as uint32_t) << 8 as libc::c_int
        | *key.offset(8 as libc::c_int as isize).offset(0 as libc::c_int as isize)
            as uint32_t;
    (*ctx)
        .state[7 as libc::c_int
        as usize] = (*key
        .offset(12 as libc::c_int as isize)
        .offset(3 as libc::c_int as isize) as uint32_t) << 24 as libc::c_int
        | (*key.offset(12 as libc::c_int as isize).offset(2 as libc::c_int as isize)
            as uint32_t) << 16 as libc::c_int
        | (*key.offset(12 as libc::c_int as isize).offset(1 as libc::c_int as isize)
            as uint32_t) << 8 as libc::c_int
        | *key.offset(12 as libc::c_int as isize).offset(0 as libc::c_int as isize)
            as uint32_t;
    (*ctx)
        .state[8 as libc::c_int
        as usize] = (*key
        .offset(16 as libc::c_int as isize)
        .offset(3 as libc::c_int as isize) as uint32_t) << 24 as libc::c_int
        | (*key.offset(16 as libc::c_int as isize).offset(2 as libc::c_int as isize)
            as uint32_t) << 16 as libc::c_int
        | (*key.offset(16 as libc::c_int as isize).offset(1 as libc::c_int as isize)
            as uint32_t) << 8 as libc::c_int
        | *key.offset(16 as libc::c_int as isize).offset(0 as libc::c_int as isize)
            as uint32_t;
    (*ctx)
        .state[9 as libc::c_int
        as usize] = (*key
        .offset(20 as libc::c_int as isize)
        .offset(3 as libc::c_int as isize) as uint32_t) << 24 as libc::c_int
        | (*key.offset(20 as libc::c_int as isize).offset(2 as libc::c_int as isize)
            as uint32_t) << 16 as libc::c_int
        | (*key.offset(20 as libc::c_int as isize).offset(1 as libc::c_int as isize)
            as uint32_t) << 8 as libc::c_int
        | *key.offset(20 as libc::c_int as isize).offset(0 as libc::c_int as isize)
            as uint32_t;
    (*ctx)
        .state[10 as libc::c_int
        as usize] = (*key
        .offset(24 as libc::c_int as isize)
        .offset(3 as libc::c_int as isize) as uint32_t) << 24 as libc::c_int
        | (*key.offset(24 as libc::c_int as isize).offset(2 as libc::c_int as isize)
            as uint32_t) << 16 as libc::c_int
        | (*key.offset(24 as libc::c_int as isize).offset(1 as libc::c_int as isize)
            as uint32_t) << 8 as libc::c_int
        | *key.offset(24 as libc::c_int as isize).offset(0 as libc::c_int as isize)
            as uint32_t;
    (*ctx)
        .state[11 as libc::c_int
        as usize] = (*key
        .offset(28 as libc::c_int as isize)
        .offset(3 as libc::c_int as isize) as uint32_t) << 24 as libc::c_int
        | (*key.offset(28 as libc::c_int as isize).offset(2 as libc::c_int as isize)
            as uint32_t) << 16 as libc::c_int
        | (*key.offset(28 as libc::c_int as isize).offset(1 as libc::c_int as isize)
            as uint32_t) << 8 as libc::c_int
        | *key.offset(28 as libc::c_int as isize).offset(0 as libc::c_int as isize)
            as uint32_t;
    memcpy(
        ((*ctx).state).as_mut_ptr() as *mut libc::c_void,
        sigma.as_ptr() as *const libc::c_void,
        ::std::mem::size_of::<[uint32_t; 4]>() as libc::c_ulong,
    );
}
