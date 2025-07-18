use ::libc;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct yarrow_key_event_ctx {
    pub index: libc::c_uint,
    pub chars: [libc::c_uint; 16],
    pub previous: libc::c_uint,
}
pub unsafe extern "C" fn nettle_yarrow_key_event_init(
    mut ctx: *mut yarrow_key_event_ctx,
) {
    let mut i: libc::c_uint = 0;
    (*ctx).index = 0 as libc::c_int as libc::c_uint;
    (*ctx).previous = 0 as libc::c_int as libc::c_uint;
    i = 0 as libc::c_int as libc::c_uint;
    while i < 16 as libc::c_int as libc::c_uint {
        (*ctx).chars[i as usize] = 0 as libc::c_int as libc::c_uint;
        i = i.wrapping_add(1);
        i;
    }
}
pub unsafe extern "C" fn nettle_yarrow_key_event_estimate(
    mut ctx: *mut yarrow_key_event_ctx,
    mut key: libc::c_uint,
    mut time: libc::c_uint,
) -> libc::c_uint {
    let mut entropy: libc::c_uint = 0 as libc::c_int as libc::c_uint;
    let mut i: libc::c_uint = 0;
    if (*ctx).previous != 0 && time > (*ctx).previous {
        if time.wrapping_sub((*ctx).previous) >= 256 as libc::c_int as libc::c_uint {
            entropy = entropy.wrapping_add(1);
            entropy;
        }
    }
    (*ctx).previous = time;
    if key == 0 {
        return entropy;
    }
    i = 0 as libc::c_int as libc::c_uint;
    while i < 16 as libc::c_int as libc::c_uint {
        if key == (*ctx).chars[i as usize] {
            return entropy;
        }
        i = i.wrapping_add(1);
        i;
    }
    if (*ctx).chars[(*ctx).index as usize] != 0 {
        entropy = entropy.wrapping_add(1);
        entropy;
    }
    (*ctx).chars[(*ctx).index as usize] = key;
    (*ctx)
        .index = ((*ctx).index)
        .wrapping_add(1 as libc::c_int as libc::c_uint)
        .wrapping_rem(16 as libc::c_int as libc::c_uint);
    return entropy;
}
