use ::libc;
extern "C" {
    fn _nettle_chacha_core(
        dst: *mut uint32_t,
        src: *const uint32_t,
        rounds: libc::c_uint,
    );
    fn nettle_memxor3(
        dst: *mut libc::c_void,
        a: *const libc::c_void,
        b: *const libc::c_void,
        n: size_t,
    ) -> *mut libc::c_void;
}
pub type size_t = libc::c_ulong;
pub type __uint8_t = libc::c_uchar;
pub type __uint32_t = libc::c_uint;
pub type uint8_t = __uint8_t;
pub type uint32_t = __uint32_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct chacha_ctx {
    pub state: [uint32_t; 16],
}
pub unsafe extern "C" fn nettle_chacha_crypt(
    mut ctx: *mut chacha_ctx,
    mut length: size_t,
    mut dst: *mut uint8_t,
    mut src: *const uint8_t,
) {
    if length == 0 {
        return;
    }
    loop {
        let mut x: [uint32_t; 16] = [0; 16];
        _nettle_chacha_core(
            x.as_mut_ptr(),
            ((*ctx).state).as_mut_ptr(),
            20 as libc::c_int as libc::c_uint,
        );
        (*ctx)
            .state[12 as libc::c_int
            as usize] = ((*ctx).state[12 as libc::c_int as usize]).wrapping_add(1);
        (*ctx)
            .state[13 as libc::c_int
            as usize] = ((*ctx).state[13 as libc::c_int as usize] as libc::c_uint)
            .wrapping_add(
                ((*ctx).state[12 as libc::c_int as usize]
                    == 0 as libc::c_int as libc::c_uint) as libc::c_int as libc::c_uint,
            ) as uint32_t as uint32_t;
        if length <= 64 as libc::c_int as libc::c_ulong {
            nettle_memxor3(
                dst as *mut libc::c_void,
                src as *const libc::c_void,
                x.as_mut_ptr() as *const libc::c_void,
                length,
            );
            return;
        }
        nettle_memxor3(
            dst as *mut libc::c_void,
            src as *const libc::c_void,
            x.as_mut_ptr() as *const libc::c_void,
            64 as libc::c_int as size_t,
        );
        length = (length as libc::c_ulong)
            .wrapping_sub(64 as libc::c_int as libc::c_ulong) as size_t as size_t;
        dst = dst.offset(64 as libc::c_int as isize);
        src = src.offset(64 as libc::c_int as isize);
    };
}
pub unsafe extern "C" fn nettle_chacha_crypt32(
    mut ctx: *mut chacha_ctx,
    mut length: size_t,
    mut dst: *mut uint8_t,
    mut src: *const uint8_t,
) {
    if length == 0 {
        return;
    }
    loop {
        let mut x: [uint32_t; 16] = [0; 16];
        _nettle_chacha_core(
            x.as_mut_ptr(),
            ((*ctx).state).as_mut_ptr(),
            20 as libc::c_int as libc::c_uint,
        );
        (*ctx)
            .state[12 as libc::c_int
            as usize] = ((*ctx).state[12 as libc::c_int as usize]).wrapping_add(1);
        (*ctx).state[12 as libc::c_int as usize];
        if length <= 64 as libc::c_int as libc::c_ulong {
            nettle_memxor3(
                dst as *mut libc::c_void,
                src as *const libc::c_void,
                x.as_mut_ptr() as *const libc::c_void,
                length,
            );
            return;
        }
        nettle_memxor3(
            dst as *mut libc::c_void,
            src as *const libc::c_void,
            x.as_mut_ptr() as *const libc::c_void,
            64 as libc::c_int as size_t,
        );
        length = (length as libc::c_ulong)
            .wrapping_sub(64 as libc::c_int as libc::c_ulong) as size_t as size_t;
        dst = dst.offset(64 as libc::c_int as isize);
        src = src.offset(64 as libc::c_int as isize);
    };
}
