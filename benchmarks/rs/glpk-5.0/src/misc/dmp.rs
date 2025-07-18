use ::libc;
extern "C" {
    fn glp_free(ptr: *mut libc::c_void);
    fn glp_alloc(n: libc::c_int, size: libc::c_int) -> *mut libc::c_void;
    fn glp_assert_(
        expr: *const libc::c_char,
        file: *const libc::c_char,
        line: libc::c_int,
    );
    fn glp_printf(fmt: *const libc::c_char, _: ...);
}
pub type size_t = libc::c_ulong;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct DMP {
    pub avail: [*mut libc::c_void; 32],
    pub block: *mut libc::c_void,
    pub used: libc::c_int,
    pub count: size_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct prefix {
    pub pool: *mut DMP,
    pub size: libc::c_int,
}
pub static mut _glp_dmp_debug: libc::c_int = 0;
pub unsafe extern "C" fn _glp_dmp_create_pool() -> *mut DMP {
    let mut pool: *mut DMP = 0 as *mut DMP;
    let mut k: libc::c_int = 0;
    (::std::mem::size_of::<*mut libc::c_void>() as libc::c_ulong
        <= 8 as libc::c_int as libc::c_ulong
        || {
            glp_assert_(
                b"sizeof(void *) <= 8\0" as *const u8 as *const libc::c_char,
                b"misc/dmp.c\0" as *const u8 as *const libc::c_char,
                79 as libc::c_int,
            );
            1 as libc::c_int != 0
        }) as libc::c_int;
    if _glp_dmp_debug != 0 {
        glp_printf(
            b"dmp_create_pool: warning: debug mode is on\n\0" as *const u8
                as *const libc::c_char,
        );
    }
    pool = glp_alloc(
        1 as libc::c_int,
        ::std::mem::size_of::<DMP>() as libc::c_ulong as libc::c_int,
    ) as *mut DMP;
    k = 0 as libc::c_int;
    while k <= 31 as libc::c_int {
        (*pool).avail[k as usize] = 0 as *mut libc::c_void;
        k += 1;
        k;
    }
    (*pool).block = 0 as *mut libc::c_void;
    (*pool).used = 8000 as libc::c_int;
    (*pool).count = 0 as libc::c_int as size_t;
    return pool;
}
pub unsafe extern "C" fn _glp_dmp_get_atom(
    mut pool: *mut DMP,
    mut size: libc::c_int,
) -> *mut libc::c_void {
    let mut atom: *mut libc::c_void = 0 as *mut libc::c_void;
    let mut k: libc::c_int = 0;
    let mut need: libc::c_int = 0;
    (1 as libc::c_int <= size && size <= 256 as libc::c_int
        || {
            glp_assert_(
                b"1 <= size && size <= 256\0" as *const u8 as *const libc::c_char,
                b"misc/dmp.c\0" as *const u8 as *const libc::c_char,
                117 as libc::c_int,
            );
            1 as libc::c_int != 0
        }) as libc::c_int;
    need = size + 7 as libc::c_int & !(7 as libc::c_int);
    k = (need >> 3 as libc::c_int) - 1 as libc::c_int;
    if ((*pool).avail[k as usize]).is_null() {
        if _glp_dmp_debug != 0 {
            need = (need as libc::c_ulong)
                .wrapping_add(
                    (::std::mem::size_of::<prefix>() as libc::c_ulong)
                        .wrapping_add(7 as libc::c_int as libc::c_ulong)
                        & !(7 as libc::c_int) as libc::c_ulong,
                ) as libc::c_int as libc::c_int;
        }
        if (*pool).used + need > 8000 as libc::c_int {
            let mut block: *mut libc::c_void = glp_alloc(
                8000 as libc::c_int,
                ::std::mem::size_of::<libc::c_char>() as libc::c_ulong as libc::c_int,
            ) as *mut libc::c_char as *mut libc::c_void;
            let ref mut fresh0 = *(block as *mut *mut libc::c_void);
            *fresh0 = (*pool).block;
            (*pool).block = block;
            (*pool).used = 8 as libc::c_int;
        }
        atom = ((*pool).block as *mut libc::c_char).offset((*pool).used as isize)
            as *mut libc::c_void;
        (*pool).used += need;
    } else {
        atom = (*pool).avail[k as usize];
        (*pool).avail[k as usize] = *(atom as *mut *mut libc::c_void);
    }
    if _glp_dmp_debug != 0 {
        let ref mut fresh1 = (*(atom as *mut prefix)).pool;
        *fresh1 = pool;
        (*(atom as *mut prefix)).size = size;
        atom = (atom as *mut libc::c_char)
            .offset(
                ((::std::mem::size_of::<prefix>() as libc::c_ulong)
                    .wrapping_add(7 as libc::c_int as libc::c_ulong)
                    & !(7 as libc::c_int) as libc::c_ulong) as isize,
            ) as *mut libc::c_void;
    }
    (*pool).count = ((*pool).count).wrapping_add(1);
    (*pool).count;
    return atom;
}
pub unsafe extern "C" fn _glp_dmp_free_atom(
    mut pool: *mut DMP,
    mut atom: *mut libc::c_void,
    mut size: libc::c_int,
) {
    let mut k: libc::c_int = 0;
    (1 as libc::c_int <= size && size <= 256 as libc::c_int
        || {
            glp_assert_(
                b"1 <= size && size <= 256\0" as *const u8 as *const libc::c_char,
                b"misc/dmp.c\0" as *const u8 as *const libc::c_char,
                178 as libc::c_int,
            );
            1 as libc::c_int != 0
        }) as libc::c_int;
    k = (size + 7 as libc::c_int >> 3 as libc::c_int) - 1 as libc::c_int;
    if _glp_dmp_debug != 0 {
        atom = (atom as *mut libc::c_char)
            .offset(
                -(((::std::mem::size_of::<prefix>() as libc::c_ulong)
                    .wrapping_add(7 as libc::c_int as libc::c_ulong)
                    & !(7 as libc::c_int) as libc::c_ulong) as isize),
            ) as *mut libc::c_void;
        ((*(atom as *mut prefix)).pool == pool
            || {
                glp_assert_(
                    b"((struct prefix *)atom)->pool == pool\0" as *const u8
                        as *const libc::c_char,
                    b"misc/dmp.c\0" as *const u8 as *const libc::c_char,
                    184 as libc::c_int,
                );
                1 as libc::c_int != 0
            }) as libc::c_int;
        ((*(atom as *mut prefix)).size == size
            || {
                glp_assert_(
                    b"((struct prefix *)atom)->size == size\0" as *const u8
                        as *const libc::c_char,
                    b"misc/dmp.c\0" as *const u8 as *const libc::c_char,
                    185 as libc::c_int,
                );
                1 as libc::c_int != 0
            }) as libc::c_int;
    }
    let ref mut fresh2 = *(atom as *mut *mut libc::c_void);
    *fresh2 = (*pool).avail[k as usize];
    (*pool).avail[k as usize] = atom;
    ((*pool).count > 0 as libc::c_int as libc::c_ulong
        || {
            glp_assert_(
                b"pool->count > 0\0" as *const u8 as *const libc::c_char,
                b"misc/dmp.c\0" as *const u8 as *const libc::c_char,
                191 as libc::c_int,
            );
            1 as libc::c_int != 0
        }) as libc::c_int;
    (*pool).count = ((*pool).count).wrapping_sub(1);
    (*pool).count;
}
pub unsafe extern "C" fn _glp_dmp_in_use(mut pool: *mut DMP) -> size_t {
    return (*pool).count;
}
pub unsafe extern "C" fn _glp_dmp_delete_pool(mut pool: *mut DMP) {
    while !((*pool).block).is_null() {
        let mut block: *mut libc::c_void = (*pool).block;
        (*pool).block = *(block as *mut *mut libc::c_void);
        glp_free(block);
    }
    glp_free(pool as *mut libc::c_void);
}
