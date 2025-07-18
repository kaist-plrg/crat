use ::libc;
extern "C" {
    fn rpl_free(ptr: *mut libc::c_void);
    fn strcmp(_: *const libc::c_char, _: *const libc::c_char) -> libc::c_int;
    fn hash_pjw(x: *const libc::c_void, tablesize: size_t) -> size_t;
}
pub type __dev_t = libc::c_ulong;
pub type __ino_t = libc::c_ulong;
pub type ino_t = __ino_t;
pub type dev_t = __dev_t;
pub type size_t = libc::c_ulong;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct F_triple {
    pub name: *mut libc::c_char,
    pub st_ino: ino_t,
    pub st_dev: dev_t,
}
pub unsafe extern "C" fn triple_hash(
    mut x: *const libc::c_void,
    mut table_size: size_t,
) -> size_t {
    let mut p: *const F_triple = x as *const F_triple;
    let mut tmp: size_t = hash_pjw((*p).name as *const libc::c_void, table_size);
    return (tmp ^ (*p).st_ino).wrapping_rem(table_size);
}
pub unsafe extern "C" fn triple_compare_ino_str(
    mut x: *const libc::c_void,
    mut y: *const libc::c_void,
) -> bool {
    let mut a: *const F_triple = x as *const F_triple;
    let mut b: *const F_triple = y as *const F_triple;
    return if (*a).st_ino == (*b).st_ino && (*a).st_dev == (*b).st_dev
        && strcmp((*a).name, (*b).name) == 0 as libc::c_int
    {
        1 as libc::c_int
    } else {
        0 as libc::c_int
    } != 0;
}
pub unsafe extern "C" fn triple_free(mut x: *mut libc::c_void) {
    let mut a: *mut F_triple = x as *mut F_triple;
    rpl_free((*a).name as *mut libc::c_void);
    rpl_free(a as *mut libc::c_void);
}
