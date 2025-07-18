use ::libc;
extern "C" {
    fn memcpy(
        _: *mut libc::c_void,
        _: *const libc::c_void,
        _: libc::c_ulong,
    ) -> *mut libc::c_void;
}
pub type size_t = libc::c_ulong;
pub type __uint8_t = libc::c_uchar;
pub type uint8_t = __uint8_t;
pub type nettle_hash_init_func = unsafe extern "C" fn(*mut libc::c_void) -> ();
pub type nettle_hash_update_func = unsafe extern "C" fn(
    *mut libc::c_void,
    size_t,
    *const uint8_t,
) -> ();
pub type nettle_hash_digest_func = unsafe extern "C" fn(
    *mut libc::c_void,
    size_t,
    *mut uint8_t,
) -> ();
#[derive(Copy, Clone)]
#[repr(C)]
pub struct nettle_hash {
    pub name: *const libc::c_char,
    pub context_size: libc::c_uint,
    pub digest_size: libc::c_uint,
    pub block_size: libc::c_uint,
    pub init: Option::<nettle_hash_init_func>,
    pub update: Option::<nettle_hash_update_func>,
    pub digest: Option::<nettle_hash_digest_func>,
}
pub unsafe extern "C" fn nettle_pss_mgf1(
    mut seed: *const libc::c_void,
    mut hash: *const nettle_hash,
    mut length: size_t,
    mut mask: *mut uint8_t,
) {
    let mut h: *mut uint8_t = 0 as *mut uint8_t;
    let mut state: *mut libc::c_void = 0 as *mut libc::c_void;
    let mut i: size_t = 0;
    let mut c: [uint8_t; 4] = [0; 4];
    let mut fresh0 = ::std::vec::from_elem(
        0,
        (::std::mem::size_of::<uint8_t>() as libc::c_ulong)
            .wrapping_mul((*hash).digest_size as libc::c_ulong) as usize,
    );
    h = fresh0.leak().as_mut_ptr() as *mut uint8_t;
    let mut fresh1 = ::std::vec::from_elem(
        0,
        (*hash).context_size as libc::c_ulong as usize,
    );
    state = fresh1.leak().as_mut_ptr() as *mut _;
    i = 0 as libc::c_int as size_t;
    loop {
        c[0 as libc::c_int
            as usize] = (i >> 24 as libc::c_int & 0xff as libc::c_int as libc::c_ulong)
            as uint8_t;
        c[1 as libc::c_int
            as usize] = (i >> 16 as libc::c_int & 0xff as libc::c_int as libc::c_ulong)
            as uint8_t;
        c[2 as libc::c_int
            as usize] = (i >> 8 as libc::c_int & 0xff as libc::c_int as libc::c_ulong)
            as uint8_t;
        c[3 as libc::c_int
            as usize] = (i & 0xff as libc::c_int as libc::c_ulong) as uint8_t;
        memcpy(state, seed, (*hash).context_size as libc::c_ulong);
        ((*hash).update).unwrap()(state, 4 as libc::c_int as size_t, c.as_mut_ptr());
        if length <= (*hash).digest_size as libc::c_ulong {
            ((*hash).digest).unwrap()(state, length, mask);
            return;
        }
        ((*hash).digest).unwrap()(state, (*hash).digest_size as size_t, mask);
        i = i.wrapping_add(1);
        i;
        mask = mask.offset((*hash).digest_size as isize);
        length = (length as libc::c_ulong)
            .wrapping_sub((*hash).digest_size as libc::c_ulong) as size_t as size_t;
    };
}
