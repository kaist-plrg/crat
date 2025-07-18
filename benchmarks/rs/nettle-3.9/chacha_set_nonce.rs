use ::libc;
pub type __uint8_t = libc::c_uchar;
pub type __uint32_t = libc::c_uint;
pub type uint8_t = __uint8_t;
pub type uint32_t = __uint32_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct chacha_ctx {
    pub state: [uint32_t; 16],
}
pub unsafe extern "C" fn nettle_chacha_set_nonce(
    mut ctx: *mut chacha_ctx,
    mut nonce: *const uint8_t,
) {
    (*ctx).state[12 as libc::c_int as usize] = 0 as libc::c_int as uint32_t;
    (*ctx).state[13 as libc::c_int as usize] = 0 as libc::c_int as uint32_t;
    (*ctx)
        .state[14 as libc::c_int
        as usize] = (*nonce
        .offset(0 as libc::c_int as isize)
        .offset(3 as libc::c_int as isize) as uint32_t) << 24 as libc::c_int
        | (*nonce.offset(0 as libc::c_int as isize).offset(2 as libc::c_int as isize)
            as uint32_t) << 16 as libc::c_int
        | (*nonce.offset(0 as libc::c_int as isize).offset(1 as libc::c_int as isize)
            as uint32_t) << 8 as libc::c_int
        | *nonce.offset(0 as libc::c_int as isize).offset(0 as libc::c_int as isize)
            as uint32_t;
    (*ctx)
        .state[15 as libc::c_int
        as usize] = (*nonce
        .offset(4 as libc::c_int as isize)
        .offset(3 as libc::c_int as isize) as uint32_t) << 24 as libc::c_int
        | (*nonce.offset(4 as libc::c_int as isize).offset(2 as libc::c_int as isize)
            as uint32_t) << 16 as libc::c_int
        | (*nonce.offset(4 as libc::c_int as isize).offset(1 as libc::c_int as isize)
            as uint32_t) << 8 as libc::c_int
        | *nonce.offset(4 as libc::c_int as isize).offset(0 as libc::c_int as isize)
            as uint32_t;
}
pub unsafe extern "C" fn nettle_chacha_set_nonce96(
    mut ctx: *mut chacha_ctx,
    mut nonce: *const uint8_t,
) {
    (*ctx).state[12 as libc::c_int as usize] = 0 as libc::c_int as uint32_t;
    (*ctx)
        .state[13 as libc::c_int
        as usize] = (*nonce
        .offset(0 as libc::c_int as isize)
        .offset(3 as libc::c_int as isize) as uint32_t) << 24 as libc::c_int
        | (*nonce.offset(0 as libc::c_int as isize).offset(2 as libc::c_int as isize)
            as uint32_t) << 16 as libc::c_int
        | (*nonce.offset(0 as libc::c_int as isize).offset(1 as libc::c_int as isize)
            as uint32_t) << 8 as libc::c_int
        | *nonce.offset(0 as libc::c_int as isize).offset(0 as libc::c_int as isize)
            as uint32_t;
    (*ctx)
        .state[14 as libc::c_int
        as usize] = (*nonce
        .offset(4 as libc::c_int as isize)
        .offset(3 as libc::c_int as isize) as uint32_t) << 24 as libc::c_int
        | (*nonce.offset(4 as libc::c_int as isize).offset(2 as libc::c_int as isize)
            as uint32_t) << 16 as libc::c_int
        | (*nonce.offset(4 as libc::c_int as isize).offset(1 as libc::c_int as isize)
            as uint32_t) << 8 as libc::c_int
        | *nonce.offset(4 as libc::c_int as isize).offset(0 as libc::c_int as isize)
            as uint32_t;
    (*ctx)
        .state[15 as libc::c_int
        as usize] = (*nonce
        .offset(8 as libc::c_int as isize)
        .offset(3 as libc::c_int as isize) as uint32_t) << 24 as libc::c_int
        | (*nonce.offset(8 as libc::c_int as isize).offset(2 as libc::c_int as isize)
            as uint32_t) << 16 as libc::c_int
        | (*nonce.offset(8 as libc::c_int as isize).offset(1 as libc::c_int as isize)
            as uint32_t) << 8 as libc::c_int
        | *nonce.offset(8 as libc::c_int as isize).offset(0 as libc::c_int as isize)
            as uint32_t;
}
pub unsafe extern "C" fn nettle_chacha_set_counter(
    mut ctx: *mut chacha_ctx,
    mut counter: *const uint8_t,
) {
    (*ctx)
        .state[12 as libc::c_int
        as usize] = (*counter
        .offset(0 as libc::c_int as isize)
        .offset(3 as libc::c_int as isize) as uint32_t) << 24 as libc::c_int
        | (*counter.offset(0 as libc::c_int as isize).offset(2 as libc::c_int as isize)
            as uint32_t) << 16 as libc::c_int
        | (*counter.offset(0 as libc::c_int as isize).offset(1 as libc::c_int as isize)
            as uint32_t) << 8 as libc::c_int
        | *counter.offset(0 as libc::c_int as isize).offset(0 as libc::c_int as isize)
            as uint32_t;
    (*ctx)
        .state[13 as libc::c_int
        as usize] = (*counter
        .offset(4 as libc::c_int as isize)
        .offset(3 as libc::c_int as isize) as uint32_t) << 24 as libc::c_int
        | (*counter.offset(4 as libc::c_int as isize).offset(2 as libc::c_int as isize)
            as uint32_t) << 16 as libc::c_int
        | (*counter.offset(4 as libc::c_int as isize).offset(1 as libc::c_int as isize)
            as uint32_t) << 8 as libc::c_int
        | *counter.offset(4 as libc::c_int as isize).offset(0 as libc::c_int as isize)
            as uint32_t;
}
pub unsafe extern "C" fn nettle_chacha_set_counter32(
    mut ctx: *mut chacha_ctx,
    mut counter: *const uint8_t,
) {
    (*ctx)
        .state[12 as libc::c_int
        as usize] = (*counter
        .offset(0 as libc::c_int as isize)
        .offset(3 as libc::c_int as isize) as uint32_t) << 24 as libc::c_int
        | (*counter.offset(0 as libc::c_int as isize).offset(2 as libc::c_int as isize)
            as uint32_t) << 16 as libc::c_int
        | (*counter.offset(0 as libc::c_int as isize).offset(1 as libc::c_int as isize)
            as uint32_t) << 8 as libc::c_int
        | *counter.offset(0 as libc::c_int as isize).offset(0 as libc::c_int as isize)
            as uint32_t;
}
