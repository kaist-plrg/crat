use ::libc;
extern "C" {
    fn __errno_location() -> *mut libc::c_int;
}
pub type pth_ring_t = pth_ring_st;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct pth_ring_st {
    pub r_hook: *mut pth_ringnode_t,
    pub r_nodes: libc::c_uint,
}
pub type pth_ringnode_t = pth_ringnode_st;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct pth_ringnode_st {
    pub rn_next: *mut pth_ringnode_t,
    pub rn_prev: *mut pth_ringnode_t,
}
pub unsafe extern "C" fn __pth_ring_init(mut r: *mut pth_ring_t) {
    if r.is_null() {
        return;
    }
    (*r).r_hook = 0 as *mut pth_ringnode_t;
    (*r).r_nodes = 0 as libc::c_int as libc::c_uint;
}
pub unsafe extern "C" fn __pth_ring_insert_after(
    mut r: *mut pth_ring_t,
    mut rn1: *mut pth_ringnode_t,
    mut rn2: *mut pth_ringnode_t,
) {
    if r.is_null() || rn1.is_null() || rn2.is_null() {
        return;
    }
    (*rn2).rn_prev = rn1;
    (*rn2).rn_next = (*rn1).rn_next;
    (*(*rn2).rn_prev).rn_next = rn2;
    (*(*rn2).rn_next).rn_prev = rn2;
    (*r).r_nodes = ((*r).r_nodes).wrapping_add(1);
    (*r).r_nodes;
}
pub unsafe extern "C" fn __pth_ring_insert_before(
    mut r: *mut pth_ring_t,
    mut rn1: *mut pth_ringnode_t,
    mut rn2: *mut pth_ringnode_t,
) {
    if r.is_null() || rn1.is_null() || rn2.is_null() {
        return;
    }
    (*rn2).rn_next = rn1;
    (*rn2).rn_prev = (*rn1).rn_prev;
    (*(*rn2).rn_prev).rn_next = rn2;
    (*(*rn2).rn_next).rn_prev = rn2;
    (*r).r_nodes = ((*r).r_nodes).wrapping_add(1);
    (*r).r_nodes;
}
pub unsafe extern "C" fn __pth_ring_delete(
    mut r: *mut pth_ring_t,
    mut rn: *mut pth_ringnode_t,
) {
    if r.is_null() || rn.is_null() {
        return;
    }
    if (*r).r_hook == rn && (*rn).rn_prev == rn && (*rn).rn_next == rn {
        (*r).r_hook = 0 as *mut pth_ringnode_t;
    } else {
        if (*r).r_hook == rn {
            (*r).r_hook = (*rn).rn_next;
        }
        (*(*rn).rn_prev).rn_next = (*rn).rn_next;
        (*(*rn).rn_next).rn_prev = (*rn).rn_prev;
    }
    (*r).r_nodes = ((*r).r_nodes).wrapping_sub(1);
    (*r).r_nodes;
}
pub unsafe extern "C" fn __pth_ring_prepend(
    mut r: *mut pth_ring_t,
    mut rn: *mut pth_ringnode_t,
) {
    if r.is_null() || rn.is_null() {
        return;
    }
    if ((*r).r_hook).is_null() {
        (*r).r_hook = rn;
        (*rn).rn_next = rn;
        (*rn).rn_prev = rn;
    } else {
        (*rn).rn_next = (*r).r_hook;
        (*rn).rn_prev = (*(*r).r_hook).rn_prev;
        (*(*rn).rn_next).rn_prev = rn;
        (*(*rn).rn_prev).rn_next = rn;
        (*r).r_hook = rn;
    }
    (*r).r_nodes = ((*r).r_nodes).wrapping_add(1);
    (*r).r_nodes;
}
pub unsafe extern "C" fn __pth_ring_append(
    mut r: *mut pth_ring_t,
    mut rn: *mut pth_ringnode_t,
) {
    if r.is_null() || rn.is_null() {
        return;
    }
    if ((*r).r_hook).is_null() {
        (*r).r_hook = rn;
        (*rn).rn_next = rn;
        (*rn).rn_prev = rn;
    } else {
        (*rn).rn_next = (*r).r_hook;
        (*rn).rn_prev = (*(*r).r_hook).rn_prev;
        (*(*rn).rn_next).rn_prev = rn;
        (*(*rn).rn_prev).rn_next = rn;
    }
    (*r).r_nodes = ((*r).r_nodes).wrapping_add(1);
    (*r).r_nodes;
}
pub unsafe extern "C" fn __pth_ring_pop(mut r: *mut pth_ring_t) -> *mut pth_ringnode_t {
    let mut rn: *mut pth_ringnode_t = 0 as *mut pth_ringnode_t;
    rn = if r.is_null() { 0 as *mut pth_ringnode_t } else { (*r).r_hook };
    if !rn.is_null() {
        __pth_ring_delete(r, rn);
    }
    return rn;
}
pub unsafe extern "C" fn __pth_ring_favorite(
    mut r: *mut pth_ring_t,
    mut rn: *mut pth_ringnode_t,
) -> libc::c_int {
    if r.is_null() {
        return 0 as libc::c_int;
    }
    if ((*r).r_hook).is_null() {
        return 0 as libc::c_int;
    }
    if (*r).r_hook == rn {
        return (0 as libc::c_int == 0) as libc::c_int;
    }
    __pth_ring_delete(r, rn);
    __pth_ring_prepend(r, rn);
    return (0 as libc::c_int == 0) as libc::c_int;
}
pub unsafe extern "C" fn __pth_ring_dequeue(
    mut r: *mut pth_ring_t,
) -> *mut pth_ringnode_t {
    let mut rn: *mut pth_ringnode_t = 0 as *mut pth_ringnode_t;
    rn = if r.is_null() {
        0 as *mut pth_ringnode_t
    } else if ((*r).r_hook).is_null() {
        0 as *mut pth_ringnode_t
    } else {
        (*(*r).r_hook).rn_prev
    };
    if !rn.is_null() {
        __pth_ring_delete(r, rn);
    }
    return rn;
}
pub unsafe extern "C" fn __pth_ring_contains(
    mut r: *mut pth_ring_t,
    mut rns: *mut pth_ringnode_t,
) -> libc::c_int {
    let mut rn: *mut pth_ringnode_t = 0 as *mut pth_ringnode_t;
    let mut rc: libc::c_int = 0;
    if r.is_null() || rns.is_null() {
        *__errno_location() = 22 as libc::c_int;
        return 0 as libc::c_int;
    }
    rc = 0 as libc::c_int;
    rn = (*r).r_hook;
    if !rn.is_null() {
        loop {
            if rn == rns {
                rc = (0 as libc::c_int == 0) as libc::c_int;
                break;
            } else {
                rn = (*rn).rn_next;
                if !(rn != (*r).r_hook) {
                    break;
                }
            }
        }
    }
    return rc;
}
