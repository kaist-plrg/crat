use ::libc;
extern "C" {
    fn _nettle_salsa20_core(
        dst: *mut uint32_t,
        src: *const uint32_t,
        rounds: libc::c_uint,
    );
    fn _nettle_salsa20_2core(
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
pub struct salsa20_ctx {
    pub input: [uint32_t; 16],
}
pub unsafe extern "C" fn _nettle_salsa20_crypt(
    mut ctx: *mut salsa20_ctx,
    mut rounds: libc::c_uint,
    mut length: size_t,
    mut dst: *mut uint8_t,
    mut src: *const uint8_t,
) {
    let mut x: [uint32_t; 32] = [0; 32];
    while length > 64 as libc::c_int as libc::c_ulong {
        _nettle_salsa20_2core(x.as_mut_ptr(), ((*ctx).input).as_mut_ptr(), rounds);
        (*ctx)
            .input[8 as libc::c_int
            as usize] = ((*ctx).input[8 as libc::c_int as usize] as libc::c_uint)
            .wrapping_add(2 as libc::c_int as libc::c_uint) as uint32_t as uint32_t;
        (*ctx)
            .input[9 as libc::c_int
            as usize] = ((*ctx).input[9 as libc::c_int as usize] as libc::c_uint)
            .wrapping_add(
                ((*ctx).input[8 as libc::c_int as usize]
                    < 2 as libc::c_int as libc::c_uint) as libc::c_int as libc::c_uint,
            ) as uint32_t as uint32_t;
        if length <= (2 as libc::c_int * 64 as libc::c_int) as libc::c_ulong {
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
            (2 as libc::c_int * 64 as libc::c_int) as size_t,
        );
        length = (length as libc::c_ulong)
            .wrapping_sub((2 as libc::c_int * 64 as libc::c_int) as libc::c_ulong)
            as size_t as size_t;
        dst = dst.offset((2 as libc::c_int * 64 as libc::c_int) as isize);
        src = src.offset((2 as libc::c_int * 64 as libc::c_int) as isize);
    }
    _nettle_salsa20_core(x.as_mut_ptr(), ((*ctx).input).as_mut_ptr(), rounds);
    (*ctx)
        .input[8 as libc::c_int
        as usize] = ((*ctx).input[8 as libc::c_int as usize]).wrapping_add(1);
    (*ctx)
        .input[9 as libc::c_int
        as usize] = ((*ctx).input[9 as libc::c_int as usize] as libc::c_uint)
        .wrapping_add(
            ((*ctx).input[8 as libc::c_int as usize] == 0 as libc::c_int as libc::c_uint)
                as libc::c_int as libc::c_uint,
        ) as uint32_t as uint32_t;
    nettle_memxor3(
        dst as *mut libc::c_void,
        src as *const libc::c_void,
        x.as_mut_ptr() as *const libc::c_void,
        length,
    );
}
