use ::libc;
extern "C" {
    fn pow(_: libc::c_double, _: libc::c_double) -> libc::c_double;
    fn glp_assert_(
        expr: *const libc::c_char,
        file: *const libc::c_char,
        line: libc::c_int,
    );
    fn glp_error_(file: *const libc::c_char, line: libc::c_int) -> glp_errfunc;
    fn glp_printf(fmt: *const libc::c_char, _: ...);
    fn malloc(_: libc::c_ulong) -> *mut libc::c_void;
    fn free(__ptr: *mut libc::c_void);
    fn realloc(_: *mut libc::c_void, _: libc::c_ulong) -> *mut libc::c_void;
}
pub type size_t = libc::c_ulong;
pub type glp_errfunc = Option::<unsafe extern "C" fn(*const libc::c_char, ...) -> ()>;
pub type bool_0 = libc::c_int;
pub type lit = libc::c_int;
pub type lbool = libc::c_int;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct veci {
    pub size: libc::c_int,
    pub cap: libc::c_int,
    pub ptr: *mut libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct vecp {
    pub size: libc::c_int,
    pub cap: libc::c_int,
    pub ptr: *mut *mut libc::c_void,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct clause {
    pub size_learnt: libc::c_int,
    pub lits: [lit; 1],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct stats {
    pub starts: libc::c_double,
    pub decisions: libc::c_double,
    pub propagations: libc::c_double,
    pub inspects: libc::c_double,
    pub conflicts: libc::c_double,
    pub clauses: libc::c_double,
    pub clauses_literals: libc::c_double,
    pub learnts: libc::c_double,
    pub learnts_literals: libc::c_double,
    pub max_literals: libc::c_double,
    pub tot_literals: libc::c_double,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct solver {
    pub size: libc::c_int,
    pub cap: libc::c_int,
    pub qhead: libc::c_int,
    pub qtail: libc::c_int,
    pub clauses: vecp,
    pub learnts: vecp,
    pub var_inc: libc::c_double,
    pub var_decay: libc::c_double,
    pub cla_inc: libc::c_float,
    pub cla_decay: libc::c_float,
    pub wlists: *mut vecp,
    pub activity: *mut libc::c_double,
    pub assigns: *mut lbool,
    pub orderpos: *mut libc::c_int,
    pub reasons: *mut *mut clause,
    pub levels: *mut libc::c_int,
    pub trail: *mut lit,
    pub binary: *mut clause,
    pub tags: *mut lbool,
    pub tagged: veci,
    pub stack: veci,
    pub order: veci,
    pub trail_lim: veci,
    pub model: veci,
    pub root_level: libc::c_int,
    pub simpdb_assigns: libc::c_int,
    pub simpdb_props: libc::c_int,
    pub random_seed: libc::c_double,
    pub progress_estimate: libc::c_double,
    pub verbosity: libc::c_int,
    pub stats: stats,
}
unsafe extern "C" fn ymalloc(mut size: libc::c_int) -> *mut libc::c_void {
    let mut ptr: *mut libc::c_void = 0 as *mut libc::c_void;
    (size > 0 as libc::c_int
        || {
            glp_assert_(
                b"size > 0\0" as *const u8 as *const libc::c_char,
                b"minisat/minisat.c\0" as *const u8 as *const libc::c_char,
                39 as libc::c_int,
            );
            1 as libc::c_int != 0
        }) as libc::c_int;
    ptr = malloc(size as libc::c_ulong);
    if ptr.is_null() {
        (glp_error_(
            b"minisat/minisat.c\0" as *const u8 as *const libc::c_char,
            42 as libc::c_int,
        ))
            .unwrap()(
            b"MiniSat: no memory available\n\0" as *const u8 as *const libc::c_char,
        );
    }
    return ptr;
}
unsafe extern "C" fn yrealloc(
    mut ptr: *mut libc::c_void,
    mut size: libc::c_int,
) -> *mut libc::c_void {
    (size > 0 as libc::c_int
        || {
            glp_assert_(
                b"size > 0\0" as *const u8 as *const libc::c_char,
                b"minisat/minisat.c\0" as *const u8 as *const libc::c_char,
                47 as libc::c_int,
            );
            1 as libc::c_int != 0
        }) as libc::c_int;
    if ptr.is_null() {
        ptr = malloc(size as libc::c_ulong);
    } else {
        ptr = realloc(ptr, size as libc::c_ulong);
    }
    if ptr.is_null() {
        (glp_error_(
            b"minisat/minisat.c\0" as *const u8 as *const libc::c_char,
            53 as libc::c_int,
        ))
            .unwrap()(
            b"MiniSat: no memory available\n\0" as *const u8 as *const libc::c_char,
        );
    }
    return ptr;
}
unsafe extern "C" fn yfree(mut ptr: *mut libc::c_void) {
    (!ptr.is_null()
        || {
            glp_assert_(
                b"ptr != NULL\0" as *const u8 as *const libc::c_char,
                b"minisat/minisat.c\0" as *const u8 as *const libc::c_char,
                58 as libc::c_int,
            );
            1 as libc::c_int != 0
        }) as libc::c_int;
    free(ptr);
}
unsafe extern "C" fn drand(mut seed: *mut libc::c_double) -> libc::c_double {
    let mut q: libc::c_int = 0;
    *seed *= 1389796 as libc::c_int as libc::c_double;
    q = (*seed / 2147483647 as libc::c_int as libc::c_double) as libc::c_int;
    *seed -= q as libc::c_double * 2147483647 as libc::c_int as libc::c_double;
    return *seed / 2147483647 as libc::c_int as libc::c_double;
}
unsafe extern "C" fn irand(
    mut seed: *mut libc::c_double,
    mut size: libc::c_int,
) -> libc::c_int {
    return (drand(seed) * size as libc::c_double) as libc::c_int;
}
unsafe extern "C" fn vecp_remove(mut v: *mut vecp, mut e: *mut libc::c_void) {
    let mut ws: *mut *mut libc::c_void = (*v).ptr;
    let mut j: libc::c_int = 0 as libc::c_int;
    while *ws.offset(j as isize) != e {
        j += 1;
        j;
    }
    (j < (*v).size
        || {
            glp_assert_(
                b"j < vecp_size(v)\0" as *const u8 as *const libc::c_char,
                b"minisat/minisat.c\0" as *const u8 as *const libc::c_char,
                182 as libc::c_int,
            );
            1 as libc::c_int != 0
        }) as libc::c_int;
    while j < (*v).size - 1 as libc::c_int {
        let ref mut fresh0 = *ws.offset(j as isize);
        *fresh0 = *ws.offset((j + 1 as libc::c_int) as isize);
        j += 1;
        j;
    }
    (*v).size = (*v).size - 1 as libc::c_int;
}
unsafe extern "C" fn order_update(mut s: *mut solver, mut v: libc::c_int) {
    let mut orderpos: *mut libc::c_int = (*s).orderpos;
    let mut activity: *mut libc::c_double = (*s).activity;
    let mut heap: *mut libc::c_int = (*s).order.ptr;
    let mut i: libc::c_int = *orderpos.offset(v as isize);
    let mut x: libc::c_int = *heap.offset(i as isize);
    let mut parent: libc::c_int = (i - 1 as libc::c_int) / 2 as libc::c_int;
    (*((*s).orderpos).offset(v as isize) != -(1 as libc::c_int)
        || {
            glp_assert_(
                b"s->orderpos[v] != -1\0" as *const u8 as *const libc::c_char,
                b"minisat/minisat.c\0" as *const u8 as *const libc::c_char,
                199 as libc::c_int,
            );
            1 as libc::c_int != 0
        }) as libc::c_int;
    while i != 0 as libc::c_int
        && *activity.offset(x as isize)
            > *activity.offset(*heap.offset(parent as isize) as isize)
    {
        *heap.offset(i as isize) = *heap.offset(parent as isize);
        *orderpos.offset(*heap.offset(i as isize) as isize) = i;
        i = parent;
        parent = (i - 1 as libc::c_int) / 2 as libc::c_int;
    }
    *heap.offset(i as isize) = x;
    *orderpos.offset(x as isize) = i;
}
unsafe extern "C" fn order_unassigned(mut s: *mut solver, mut v: libc::c_int) {
    let mut orderpos: *mut libc::c_int = (*s).orderpos;
    if *orderpos.offset(v as isize) == -(1 as libc::c_int) {
        *orderpos.offset(v as isize) = (*s).order.size;
        if (*s).order.size == (*s).order.cap {
            let mut newsize: libc::c_int = (*s).order.cap * 2 as libc::c_int
                + 1 as libc::c_int;
            (*s)
                .order
                .ptr = yrealloc(
                (*s).order.ptr as *mut libc::c_void,
                (::std::mem::size_of::<libc::c_int>() as libc::c_ulong)
                    .wrapping_mul(newsize as libc::c_ulong) as libc::c_int,
            ) as *mut libc::c_int;
            (*s).order.cap = newsize;
        }
        let fresh1 = (*s).order.size;
        (*s).order.size = (*s).order.size + 1;
        *((*s).order.ptr).offset(fresh1 as isize) = v;
        order_update(s, v);
    }
}
unsafe extern "C" fn order_select(
    mut s: *mut solver,
    mut random_var_freq: libc::c_float,
) -> libc::c_int {
    let mut heap: *mut libc::c_int = 0 as *mut libc::c_int;
    let mut activity: *mut libc::c_double = 0 as *mut libc::c_double;
    let mut orderpos: *mut libc::c_int = 0 as *mut libc::c_int;
    let mut values: *mut lbool = (*s).assigns;
    if drand(&mut (*s).random_seed) < random_var_freq as libc::c_double {
        let mut next: libc::c_int = irand(&mut (*s).random_seed, (*s).size);
        (next >= 0 as libc::c_int && next < (*s).size
            || {
                glp_assert_(
                    b"next >= 0 && next < s->size\0" as *const u8 as *const libc::c_char,
                    b"minisat/minisat.c\0" as *const u8 as *const libc::c_char,
                    234 as libc::c_int,
                );
                1 as libc::c_int != 0
            }) as libc::c_int;
        if *values.offset(next as isize) == 0 as libc::c_int {
            return next;
        }
    }
    heap = (*s).order.ptr;
    activity = (*s).activity;
    orderpos = (*s).orderpos;
    while (*s).order.size > 0 as libc::c_int {
        let mut next_0: libc::c_int = *heap.offset(0 as libc::c_int as isize);
        let mut size: libc::c_int = (*s).order.size - 1 as libc::c_int;
        let mut x: libc::c_int = *heap.offset(size as isize);
        (*s).order.size = size;
        *orderpos.offset(next_0 as isize) = -(1 as libc::c_int);
        if size > 0 as libc::c_int {
            let mut act: libc::c_double = *activity.offset(x as isize);
            let mut i: libc::c_int = 0 as libc::c_int;
            let mut child: libc::c_int = 1 as libc::c_int;
            while child < size {
                if (child + 1 as libc::c_int) < size
                    && *activity.offset(*heap.offset(child as isize) as isize)
                        < *activity
                            .offset(
                                *heap.offset((child + 1 as libc::c_int) as isize) as isize,
                            )
                {
                    child += 1;
                    child;
                }
                (child < size
                    || {
                        glp_assert_(
                            b"child < size\0" as *const u8 as *const libc::c_char,
                            b"minisat/minisat.c\0" as *const u8 as *const libc::c_char,
                            265 as libc::c_int,
                        );
                        1 as libc::c_int != 0
                    }) as libc::c_int;
                if act >= *activity.offset(*heap.offset(child as isize) as isize) {
                    break;
                }
                *heap.offset(i as isize) = *heap.offset(child as isize);
                *orderpos.offset(*heap.offset(i as isize) as isize) = i;
                i = child;
                child = 2 as libc::c_int * child + 1 as libc::c_int;
            }
            *heap.offset(i as isize) = x;
            *orderpos.offset(*heap.offset(i as isize) as isize) = i;
        }
        if *values.offset(next_0 as isize) == 0 as libc::c_int {
            return next_0;
        }
    }
    return -(1 as libc::c_int);
}
unsafe extern "C" fn act_var_rescale(mut s: *mut solver) {
    let mut activity: *mut libc::c_double = (*s).activity;
    let mut i: libc::c_int = 0;
    i = 0 as libc::c_int;
    while i < (*s).size {
        *activity.offset(i as isize) *= 1e-100f64;
        i += 1;
        i;
    }
    (*s).var_inc *= 1e-100f64;
}
unsafe extern "C" fn act_var_bump(mut s: *mut solver, mut v: libc::c_int) {
    let mut activity: *mut libc::c_double = (*s).activity;
    let ref mut fresh2 = *activity.offset(v as isize);
    *fresh2 += (*s).var_inc;
    if *fresh2 > 1e100f64 {
        act_var_rescale(s);
    }
    if *((*s).orderpos).offset(v as isize) != -(1 as libc::c_int) {
        order_update(s, v);
    }
}
unsafe extern "C" fn act_var_decay(mut s: *mut solver) {
    (*s).var_inc *= (*s).var_decay;
}
unsafe extern "C" fn act_clause_rescale(mut s: *mut solver) {
    let mut cs: *mut *mut clause = (*s).learnts.ptr as *mut *mut clause;
    let mut i: libc::c_int = 0;
    i = 0 as libc::c_int;
    while i < (*s).learnts.size {
        let mut a: libc::c_float = *(&mut *((**cs.offset(i as isize)).lits)
            .as_mut_ptr()
            .offset(((**cs.offset(i as isize)).size_learnt >> 1 as libc::c_int) as isize)
            as *mut lit as *mut libc::c_float);
        *(&mut *((**cs.offset(i as isize)).lits)
            .as_mut_ptr()
            .offset(((**cs.offset(i as isize)).size_learnt >> 1 as libc::c_int) as isize)
            as *mut lit as *mut libc::c_float) = a * 1e-20f64 as libc::c_float;
        i += 1;
        i;
    }
    (*s).cla_inc *= 1e-20f64 as libc::c_float;
}
unsafe extern "C" fn act_clause_bump(mut s: *mut solver, mut c: *mut clause) {
    let mut a: libc::c_float = *(&mut *((*c).lits)
        .as_mut_ptr()
        .offset(((*c).size_learnt >> 1 as libc::c_int) as isize) as *mut lit
        as *mut libc::c_float) + (*s).cla_inc;
    *(&mut *((*c).lits)
        .as_mut_ptr()
        .offset(((*c).size_learnt >> 1 as libc::c_int) as isize) as *mut lit
        as *mut libc::c_float) = a;
    if a as libc::c_double > 1e20f64 {
        act_clause_rescale(s);
    }
}
unsafe extern "C" fn act_clause_decay(mut s: *mut solver) {
    (*s).cla_inc *= (*s).cla_decay;
}
unsafe extern "C" fn clause_new(
    mut s: *mut solver,
    mut begin: *mut lit,
    mut end: *mut lit,
    mut learnt: libc::c_int,
) -> *mut clause {
    let mut size: libc::c_int = 0;
    let mut c: *mut clause = 0 as *mut clause;
    let mut i: libc::c_int = 0;
    (end.offset_from(begin) as libc::c_long > 1 as libc::c_int as libc::c_long
        || {
            glp_assert_(
                b"end - begin > 1\0" as *const u8 as *const libc::c_char,
                b"minisat/minisat.c\0" as *const u8 as *const libc::c_char,
                342 as libc::c_int,
            );
            1 as libc::c_int != 0
        }) as libc::c_int;
    (learnt >= 0 as libc::c_int && learnt < 2 as libc::c_int
        || {
            glp_assert_(
                b"learnt >= 0 && learnt < 2\0" as *const u8 as *const libc::c_char,
                b"minisat/minisat.c\0" as *const u8 as *const libc::c_char,
                343 as libc::c_int,
            );
            1 as libc::c_int != 0
        }) as libc::c_int;
    size = end.offset_from(begin) as libc::c_long as libc::c_int;
    c = ymalloc(
        (::std::mem::size_of::<clause>() as libc::c_ulong)
            .wrapping_add(
                (::std::mem::size_of::<lit>() as libc::c_ulong)
                    .wrapping_mul(size as libc::c_ulong),
            )
            .wrapping_add(
                (learnt as libc::c_ulong)
                    .wrapping_mul(
                        ::std::mem::size_of::<libc::c_float>() as libc::c_ulong,
                    ),
            ) as libc::c_int,
    ) as *mut clause;
    (*c).size_learnt = size << 1 as libc::c_int | learnt;
    (c as size_t & 1 as libc::c_int as libc::c_ulong == 0 as libc::c_int as libc::c_ulong
        || {
            glp_assert_(
                b"((size_t)c & 1) == 0\0" as *const u8 as *const libc::c_char,
                b"minisat/minisat.c\0" as *const u8 as *const libc::c_char,
                352 as libc::c_int,
            );
            1 as libc::c_int != 0
        }) as libc::c_int;
    i = 0 as libc::c_int;
    while i < size {
        *((*c).lits).as_mut_ptr().offset(i as isize) = *begin.offset(i as isize);
        i += 1;
        i;
    }
    if learnt != 0 {
        *(&mut *((*c).lits).as_mut_ptr().offset(size as isize) as *mut lit
            as *mut libc::c_float) = 0.0f64 as libc::c_float;
    }
    (*begin.offset(0 as libc::c_int as isize) >= 0 as libc::c_int
        || {
            glp_assert_(
                b"begin[0] >= 0\0" as *const u8 as *const libc::c_char,
                b"minisat/minisat.c\0" as *const u8 as *const libc::c_char,
                361 as libc::c_int,
            );
            1 as libc::c_int != 0
        }) as libc::c_int;
    (*begin.offset(0 as libc::c_int as isize) < (*s).size * 2 as libc::c_int
        || {
            glp_assert_(
                b"begin[0] < s->size*2\0" as *const u8 as *const libc::c_char,
                b"minisat/minisat.c\0" as *const u8 as *const libc::c_char,
                362 as libc::c_int,
            );
            1 as libc::c_int != 0
        }) as libc::c_int;
    (*begin.offset(1 as libc::c_int as isize) >= 0 as libc::c_int
        || {
            glp_assert_(
                b"begin[1] >= 0\0" as *const u8 as *const libc::c_char,
                b"minisat/minisat.c\0" as *const u8 as *const libc::c_char,
                363 as libc::c_int,
            );
            1 as libc::c_int != 0
        }) as libc::c_int;
    (*begin.offset(1 as libc::c_int as isize) < (*s).size * 2 as libc::c_int
        || {
            glp_assert_(
                b"begin[1] < s->size*2\0" as *const u8 as *const libc::c_char,
                b"minisat/minisat.c\0" as *const u8 as *const libc::c_char,
                364 as libc::c_int,
            );
            1 as libc::c_int != 0
        }) as libc::c_int;
    ((*begin.offset(0 as libc::c_int as isize) ^ 1 as libc::c_int)
        < (*s).size * 2 as libc::c_int
        || {
            glp_assert_(
                b"lit_neg(begin[0]) < s->size*2\0" as *const u8 as *const libc::c_char,
                b"minisat/minisat.c\0" as *const u8 as *const libc::c_char,
                366 as libc::c_int,
            );
            1 as libc::c_int != 0
        }) as libc::c_int;
    ((*begin.offset(1 as libc::c_int as isize) ^ 1 as libc::c_int)
        < (*s).size * 2 as libc::c_int
        || {
            glp_assert_(
                b"lit_neg(begin[1]) < s->size*2\0" as *const u8 as *const libc::c_char,
                b"minisat/minisat.c\0" as *const u8 as *const libc::c_char,
                367 as libc::c_int,
            );
            1 as libc::c_int != 0
        }) as libc::c_int;
    if (*(&mut *((*s).wlists)
        .offset((*begin.offset(0 as libc::c_int as isize) ^ 1 as libc::c_int) as isize)
        as *mut vecp))
        .size
        == (*(&mut *((*s).wlists)
            .offset(
                (*begin.offset(0 as libc::c_int as isize) ^ 1 as libc::c_int) as isize,
            ) as *mut vecp))
            .cap
    {
        let mut newsize: libc::c_int = (*(&mut *((*s).wlists)
            .offset(
                (*begin.offset(0 as libc::c_int as isize) ^ 1 as libc::c_int) as isize,
            ) as *mut vecp))
            .cap * 2 as libc::c_int + 1 as libc::c_int;
        let ref mut fresh3 = (*(&mut *((*s).wlists)
            .offset(
                (*begin.offset(0 as libc::c_int as isize) ^ 1 as libc::c_int) as isize,
            ) as *mut vecp))
            .ptr;
        *fresh3 = yrealloc(
            (*(&mut *((*s).wlists)
                .offset(
                    (*begin.offset(0 as libc::c_int as isize) ^ 1 as libc::c_int)
                        as isize,
                ) as *mut vecp))
                .ptr as *mut libc::c_void,
            (::std::mem::size_of::<*mut libc::c_void>() as libc::c_ulong)
                .wrapping_mul(newsize as libc::c_ulong) as libc::c_int,
        ) as *mut *mut libc::c_void;
        (*(&mut *((*s).wlists)
            .offset(
                (*begin.offset(0 as libc::c_int as isize) ^ 1 as libc::c_int) as isize,
            ) as *mut vecp))
            .cap = newsize;
    }
    let ref mut fresh4 = (*(&mut *((*s).wlists)
        .offset((*begin.offset(0 as libc::c_int as isize) ^ 1 as libc::c_int) as isize)
        as *mut vecp))
        .size;
    let fresh5 = *fresh4;
    *fresh4 = *fresh4 + 1;
    let ref mut fresh6 = *((*(&mut *((*s).wlists)
        .offset((*begin.offset(0 as libc::c_int as isize) ^ 1 as libc::c_int) as isize)
        as *mut vecp))
        .ptr)
        .offset(fresh5 as isize);
    *fresh6 = (if size > 2 as libc::c_int {
        c
    } else {
        (*begin.offset(1 as libc::c_int as isize) as size_t)
            .wrapping_add(*begin.offset(1 as libc::c_int as isize) as size_t)
            .wrapping_add(1 as libc::c_int as libc::c_ulong) as *mut clause
    }) as *mut libc::c_void;
    if (*(&mut *((*s).wlists)
        .offset((*begin.offset(1 as libc::c_int as isize) ^ 1 as libc::c_int) as isize)
        as *mut vecp))
        .size
        == (*(&mut *((*s).wlists)
            .offset(
                (*begin.offset(1 as libc::c_int as isize) ^ 1 as libc::c_int) as isize,
            ) as *mut vecp))
            .cap
    {
        let mut newsize_0: libc::c_int = (*(&mut *((*s).wlists)
            .offset(
                (*begin.offset(1 as libc::c_int as isize) ^ 1 as libc::c_int) as isize,
            ) as *mut vecp))
            .cap * 2 as libc::c_int + 1 as libc::c_int;
        let ref mut fresh7 = (*(&mut *((*s).wlists)
            .offset(
                (*begin.offset(1 as libc::c_int as isize) ^ 1 as libc::c_int) as isize,
            ) as *mut vecp))
            .ptr;
        *fresh7 = yrealloc(
            (*(&mut *((*s).wlists)
                .offset(
                    (*begin.offset(1 as libc::c_int as isize) ^ 1 as libc::c_int)
                        as isize,
                ) as *mut vecp))
                .ptr as *mut libc::c_void,
            (::std::mem::size_of::<*mut libc::c_void>() as libc::c_ulong)
                .wrapping_mul(newsize_0 as libc::c_ulong) as libc::c_int,
        ) as *mut *mut libc::c_void;
        (*(&mut *((*s).wlists)
            .offset(
                (*begin.offset(1 as libc::c_int as isize) ^ 1 as libc::c_int) as isize,
            ) as *mut vecp))
            .cap = newsize_0;
    }
    let ref mut fresh8 = (*(&mut *((*s).wlists)
        .offset((*begin.offset(1 as libc::c_int as isize) ^ 1 as libc::c_int) as isize)
        as *mut vecp))
        .size;
    let fresh9 = *fresh8;
    *fresh8 = *fresh8 + 1;
    let ref mut fresh10 = *((*(&mut *((*s).wlists)
        .offset((*begin.offset(1 as libc::c_int as isize) ^ 1 as libc::c_int) as isize)
        as *mut vecp))
        .ptr)
        .offset(fresh9 as isize);
    *fresh10 = (if size > 2 as libc::c_int {
        c
    } else {
        (*begin.offset(0 as libc::c_int as isize) as size_t)
            .wrapping_add(*begin.offset(0 as libc::c_int as isize) as size_t)
            .wrapping_add(1 as libc::c_int as libc::c_ulong) as *mut clause
    }) as *mut libc::c_void;
    return c;
}
unsafe extern "C" fn clause_remove(mut s: *mut solver, mut c: *mut clause) {
    let mut lits: *mut lit = ((*c).lits).as_mut_ptr();
    ((*lits.offset(0 as libc::c_int as isize) ^ 1 as libc::c_int)
        < (*s).size * 2 as libc::c_int
        || {
            glp_assert_(
                b"lit_neg(lits[0]) < s->size*2\0" as *const u8 as *const libc::c_char,
                b"minisat/minisat.c\0" as *const u8 as *const libc::c_char,
                383 as libc::c_int,
            );
            1 as libc::c_int != 0
        }) as libc::c_int;
    ((*lits.offset(1 as libc::c_int as isize) ^ 1 as libc::c_int)
        < (*s).size * 2 as libc::c_int
        || {
            glp_assert_(
                b"lit_neg(lits[1]) < s->size*2\0" as *const u8 as *const libc::c_char,
                b"minisat/minisat.c\0" as *const u8 as *const libc::c_char,
                384 as libc::c_int,
            );
            1 as libc::c_int != 0
        }) as libc::c_int;
    (*lits.offset(0 as libc::c_int as isize) < (*s).size * 2 as libc::c_int
        || {
            glp_assert_(
                b"lits[0] < s->size*2\0" as *const u8 as *const libc::c_char,
                b"minisat/minisat.c\0" as *const u8 as *const libc::c_char,
                389 as libc::c_int,
            );
            1 as libc::c_int != 0
        }) as libc::c_int;
    vecp_remove(
        &mut *((*s).wlists)
            .offset(
                (*lits.offset(0 as libc::c_int as isize) ^ 1 as libc::c_int) as isize,
            ) as *mut vecp,
        (if (*c).size_learnt >> 1 as libc::c_int > 2 as libc::c_int {
            c
        } else {
            (*lits.offset(1 as libc::c_int as isize) as size_t)
                .wrapping_add(*lits.offset(1 as libc::c_int as isize) as size_t)
                .wrapping_add(1 as libc::c_int as libc::c_ulong) as *mut clause
        }) as *mut libc::c_void,
    );
    vecp_remove(
        &mut *((*s).wlists)
            .offset(
                (*lits.offset(1 as libc::c_int as isize) ^ 1 as libc::c_int) as isize,
            ) as *mut vecp,
        (if (*c).size_learnt >> 1 as libc::c_int > 2 as libc::c_int {
            c
        } else {
            (*lits.offset(0 as libc::c_int as isize) as size_t)
                .wrapping_add(*lits.offset(0 as libc::c_int as isize) as size_t)
                .wrapping_add(1 as libc::c_int as libc::c_ulong) as *mut clause
        }) as *mut libc::c_void,
    );
    if (*c).size_learnt & 1 as libc::c_int != 0 {
        (*s).stats.learnts -= 1.;
        (*s).stats.learnts;
        (*s).stats.learnts_literals
            -= ((*c).size_learnt >> 1 as libc::c_int) as libc::c_double;
    } else {
        (*s).stats.clauses -= 1.;
        (*s).stats.clauses;
        (*s).stats.clauses_literals
            -= ((*c).size_learnt >> 1 as libc::c_int) as libc::c_double;
    }
    yfree(c as *mut libc::c_void);
}
unsafe extern "C" fn clause_simplify(mut s: *mut solver, mut c: *mut clause) -> lbool {
    let mut lits: *mut lit = ((*c).lits).as_mut_ptr();
    let mut values: *mut lbool = (*s).assigns;
    let mut i: libc::c_int = 0;
    ((*s).trail_lim.size == 0 as libc::c_int
        || {
            glp_assert_(
                b"solver_dlevel(s) == 0\0" as *const u8 as *const libc::c_char,
                b"minisat/minisat.c\0" as *const u8 as *const libc::c_char,
                412 as libc::c_int,
            );
            1 as libc::c_int != 0
        }) as libc::c_int;
    i = 0 as libc::c_int;
    while i < (*c).size_learnt >> 1 as libc::c_int {
        let mut sig: lbool = (*lits.offset(i as isize) & 1 as libc::c_int == 0)
            as libc::c_int;
        sig += sig - 1 as libc::c_int;
        if *values.offset((*lits.offset(i as isize) >> 1 as libc::c_int) as isize) == sig
        {
            return 1 as libc::c_int;
        }
        i += 1;
        i;
    }
    return -(1 as libc::c_int);
}
pub unsafe extern "C" fn _glp_minisat_setnvars(mut s: *mut solver, mut n: libc::c_int) {
    let mut var: libc::c_int = 0;
    if (*s).cap < n {
        while (*s).cap < n {
            (*s).cap = (*s).cap * 2 as libc::c_int + 1 as libc::c_int;
        }
        (*s)
            .wlists = yrealloc(
            (*s).wlists as *mut libc::c_void,
            (::std::mem::size_of::<vecp>() as libc::c_ulong)
                .wrapping_mul((*s).cap as libc::c_ulong)
                .wrapping_mul(2 as libc::c_int as libc::c_ulong) as libc::c_int,
        ) as *mut vecp;
        (*s)
            .activity = yrealloc(
            (*s).activity as *mut libc::c_void,
            (::std::mem::size_of::<libc::c_double>() as libc::c_ulong)
                .wrapping_mul((*s).cap as libc::c_ulong) as libc::c_int,
        ) as *mut libc::c_double;
        (*s)
            .assigns = yrealloc(
            (*s).assigns as *mut libc::c_void,
            (::std::mem::size_of::<lbool>() as libc::c_ulong)
                .wrapping_mul((*s).cap as libc::c_ulong) as libc::c_int,
        ) as *mut lbool;
        (*s)
            .orderpos = yrealloc(
            (*s).orderpos as *mut libc::c_void,
            (::std::mem::size_of::<libc::c_int>() as libc::c_ulong)
                .wrapping_mul((*s).cap as libc::c_ulong) as libc::c_int,
        ) as *mut libc::c_int;
        (*s)
            .reasons = yrealloc(
            (*s).reasons as *mut libc::c_void,
            (::std::mem::size_of::<*mut clause>() as libc::c_ulong)
                .wrapping_mul((*s).cap as libc::c_ulong) as libc::c_int,
        ) as *mut *mut clause;
        (*s)
            .levels = yrealloc(
            (*s).levels as *mut libc::c_void,
            (::std::mem::size_of::<libc::c_int>() as libc::c_ulong)
                .wrapping_mul((*s).cap as libc::c_ulong) as libc::c_int,
        ) as *mut libc::c_int;
        (*s)
            .tags = yrealloc(
            (*s).tags as *mut libc::c_void,
            (::std::mem::size_of::<lbool>() as libc::c_ulong)
                .wrapping_mul((*s).cap as libc::c_ulong) as libc::c_int,
        ) as *mut lbool;
        (*s)
            .trail = yrealloc(
            (*s).trail as *mut libc::c_void,
            (::std::mem::size_of::<lit>() as libc::c_ulong)
                .wrapping_mul((*s).cap as libc::c_ulong) as libc::c_int,
        ) as *mut lit;
    }
    var = (*s).size;
    while var < n {
        (*((*s).wlists).offset((2 as libc::c_int * var) as isize))
            .size = 0 as libc::c_int;
        (*((*s).wlists).offset((2 as libc::c_int * var) as isize))
            .cap = 4 as libc::c_int;
        let ref mut fresh11 = (*((*s).wlists).offset((2 as libc::c_int * var) as isize))
            .ptr;
        *fresh11 = ymalloc(
            (::std::mem::size_of::<*mut libc::c_void>() as libc::c_ulong)
                .wrapping_mul(
                    (*((*s).wlists).offset((2 as libc::c_int * var) as isize)).cap
                        as libc::c_ulong,
                ) as libc::c_int,
        ) as *mut *mut libc::c_void;
        (*((*s).wlists).offset((2 as libc::c_int * var + 1 as libc::c_int) as isize))
            .size = 0 as libc::c_int;
        (*((*s).wlists).offset((2 as libc::c_int * var + 1 as libc::c_int) as isize))
            .cap = 4 as libc::c_int;
        let ref mut fresh12 = (*((*s).wlists)
            .offset((2 as libc::c_int * var + 1 as libc::c_int) as isize))
            .ptr;
        *fresh12 = ymalloc(
            (::std::mem::size_of::<*mut libc::c_void>() as libc::c_ulong)
                .wrapping_mul(
                    (*((*s).wlists)
                        .offset((2 as libc::c_int * var + 1 as libc::c_int) as isize))
                        .cap as libc::c_ulong,
                ) as libc::c_int,
        ) as *mut *mut libc::c_void;
        *((*s).activity).offset(var as isize) = 0 as libc::c_int as libc::c_double;
        *((*s).assigns).offset(var as isize) = 0 as libc::c_int;
        *((*s).orderpos).offset(var as isize) = (*s).order.size;
        let ref mut fresh13 = *((*s).reasons).offset(var as isize);
        *fresh13 = 0 as *mut clause;
        *((*s).levels).offset(var as isize) = 0 as libc::c_int;
        *((*s).tags).offset(var as isize) = 0 as libc::c_int;
        if (*s).order.size == (*s).order.cap {
            let mut newsize: libc::c_int = (*s).order.cap * 2 as libc::c_int
                + 1 as libc::c_int;
            (*s)
                .order
                .ptr = yrealloc(
                (*s).order.ptr as *mut libc::c_void,
                (::std::mem::size_of::<libc::c_int>() as libc::c_ulong)
                    .wrapping_mul(newsize as libc::c_ulong) as libc::c_int,
            ) as *mut libc::c_int;
            (*s).order.cap = newsize;
        }
        let fresh14 = (*s).order.size;
        (*s).order.size = (*s).order.size + 1;
        *((*s).order.ptr).offset(fresh14 as isize) = var;
        order_update(s, var);
        var += 1;
        var;
    }
    (*s).size = if n > (*s).size { n } else { (*s).size };
}
unsafe extern "C" fn enqueue(
    mut s: *mut solver,
    mut l: lit,
    mut from: *mut clause,
) -> bool_0 {
    let mut values: *mut lbool = (*s).assigns;
    let mut v: libc::c_int = l >> 1 as libc::c_int;
    let mut val: lbool = *values.offset(v as isize);
    let mut sig: lbool = 0;
    sig = (l & 1 as libc::c_int == 0) as libc::c_int;
    sig += sig - 1 as libc::c_int;
    if val != 0 as libc::c_int {
        return (val == sig) as libc::c_int
    } else {
        let mut levels: *mut libc::c_int = 0 as *mut libc::c_int;
        let mut reasons: *mut *mut clause = 0 as *mut *mut clause;
        levels = (*s).levels;
        reasons = (*s).reasons;
        *values.offset(v as isize) = sig;
        *levels.offset(v as isize) = (*s).trail_lim.size;
        let ref mut fresh15 = *reasons.offset(v as isize);
        *fresh15 = from;
        let fresh16 = (*s).qtail;
        (*s).qtail = (*s).qtail + 1;
        *((*s).trail).offset(fresh16 as isize) = l;
        return 1 as libc::c_int;
    };
}
unsafe extern "C" fn assume(mut s: *mut solver, mut l: lit) {
    ((*s).qtail == (*s).qhead
        || {
            glp_assert_(
                b"s->qtail == s->qhead\0" as *const u8 as *const libc::c_char,
                b"minisat/minisat.c\0" as *const u8 as *const libc::c_char,
                506 as libc::c_int,
            );
            1 as libc::c_int != 0
        }) as libc::c_int;
    (*((*s).assigns).offset((l >> 1 as libc::c_int) as isize) == 0 as libc::c_int
        || {
            glp_assert_(
                b"s->assigns[lit_var(l)] == l_Undef\0" as *const u8
                    as *const libc::c_char,
                b"minisat/minisat.c\0" as *const u8 as *const libc::c_char,
                507 as libc::c_int,
            );
            1 as libc::c_int != 0
        }) as libc::c_int;
    if (*s).trail_lim.size == (*s).trail_lim.cap {
        let mut newsize: libc::c_int = (*s).trail_lim.cap * 2 as libc::c_int
            + 1 as libc::c_int;
        (*s)
            .trail_lim
            .ptr = yrealloc(
            (*s).trail_lim.ptr as *mut libc::c_void,
            (::std::mem::size_of::<libc::c_int>() as libc::c_ulong)
                .wrapping_mul(newsize as libc::c_ulong) as libc::c_int,
        ) as *mut libc::c_int;
        (*s).trail_lim.cap = newsize;
    }
    let fresh17 = (*s).trail_lim.size;
    (*s).trail_lim.size = (*s).trail_lim.size + 1;
    *((*s).trail_lim.ptr).offset(fresh17 as isize) = (*s).qtail;
    enqueue(s, l, 0 as *mut clause);
}
unsafe extern "C" fn solver_canceluntil(mut s: *mut solver, mut level: libc::c_int) {
    let mut trail: *mut lit = 0 as *mut lit;
    let mut values: *mut lbool = 0 as *mut lbool;
    let mut reasons: *mut *mut clause = 0 as *mut *mut clause;
    let mut bound: libc::c_int = 0;
    let mut c: libc::c_int = 0;
    if (*s).trail_lim.size <= level {
        return;
    }
    trail = (*s).trail;
    values = (*s).assigns;
    reasons = (*s).reasons;
    bound = *((*s).trail_lim.ptr).offset(level as isize);
    c = (*s).qtail - 1 as libc::c_int;
    while c >= bound {
        let mut x: libc::c_int = *trail.offset(c as isize) >> 1 as libc::c_int;
        *values.offset(x as isize) = 0 as libc::c_int;
        let ref mut fresh18 = *reasons.offset(x as isize);
        *fresh18 = 0 as *mut clause;
        c -= 1;
        c;
    }
    c = (*s).qhead - 1 as libc::c_int;
    while c >= bound {
        order_unassigned(s, *trail.offset(c as isize) >> 1 as libc::c_int);
        c -= 1;
        c;
    }
    (*s).qtail = bound;
    (*s).qhead = (*s).qtail;
    (*s).trail_lim.size = level;
}
unsafe extern "C" fn solver_record(mut s: *mut solver, mut cls: *mut veci) {
    let mut begin: *mut lit = (*cls).ptr;
    let mut end: *mut lit = begin.offset((*cls).size as isize);
    let mut c: *mut clause = if (*cls).size > 1 as libc::c_int {
        clause_new(s, begin, end, 1 as libc::c_int)
    } else {
        0 as *mut clause
    };
    enqueue(s, *begin, c);
    ((*cls).size > 0 as libc::c_int
        || {
            glp_assert_(
                b"veci_size(cls) > 0\0" as *const u8 as *const libc::c_char,
                b"minisat/minisat.c\0" as *const u8 as *const libc::c_char,
                551 as libc::c_int,
            );
            1 as libc::c_int != 0
        }) as libc::c_int;
    if !c.is_null() {
        if (*s).learnts.size == (*s).learnts.cap {
            let mut newsize: libc::c_int = (*s).learnts.cap * 2 as libc::c_int
                + 1 as libc::c_int;
            (*s)
                .learnts
                .ptr = yrealloc(
                (*s).learnts.ptr as *mut libc::c_void,
                (::std::mem::size_of::<*mut libc::c_void>() as libc::c_ulong)
                    .wrapping_mul(newsize as libc::c_ulong) as libc::c_int,
            ) as *mut *mut libc::c_void;
            (*s).learnts.cap = newsize;
        }
        let fresh19 = (*s).learnts.size;
        (*s).learnts.size = (*s).learnts.size + 1;
        let ref mut fresh20 = *((*s).learnts.ptr).offset(fresh19 as isize);
        *fresh20 = c as *mut libc::c_void;
        act_clause_bump(s, c);
        (*s).stats.learnts += 1.;
        (*s).stats.learnts;
        (*s).stats.learnts_literals += (*cls).size as libc::c_double;
    }
}
unsafe extern "C" fn solver_progress(mut s: *mut solver) -> libc::c_double {
    let mut values: *mut lbool = (*s).assigns;
    let mut levels: *mut libc::c_int = (*s).levels;
    let mut i: libc::c_int = 0;
    let mut progress: libc::c_double = 0 as libc::c_int as libc::c_double;
    let mut F: libc::c_double = 1.0f64 / (*s).size as libc::c_double;
    i = 0 as libc::c_int;
    while i < (*s).size {
        if *values.offset(i as isize) != 0 as libc::c_int {
            progress += pow(F, *levels.offset(i as isize) as libc::c_double);
        }
        i += 1;
        i;
    }
    return progress / (*s).size as libc::c_double;
}
unsafe extern "C" fn solver_lit_removable(
    mut s: *mut solver,
    mut l: lit,
    mut minl: libc::c_int,
) -> bool_0 {
    let mut tags: *mut lbool = (*s).tags;
    let mut reasons: *mut *mut clause = (*s).reasons;
    let mut levels: *mut libc::c_int = (*s).levels;
    let mut top: libc::c_int = (*s).tagged.size;
    (l >> 1 as libc::c_int >= 0 as libc::c_int && (l >> 1 as libc::c_int) < (*s).size
        || {
            glp_assert_(
                b"lit_var(l) >= 0 && lit_var(l) < s->size\0" as *const u8
                    as *const libc::c_char,
                b"minisat/minisat.c\0" as *const u8 as *const libc::c_char,
                585 as libc::c_int,
            );
            1 as libc::c_int != 0
        }) as libc::c_int;
    (!(*reasons.offset((l >> 1 as libc::c_int) as isize)).is_null()
        || {
            glp_assert_(
                b"reasons[lit_var(l)] != 0\0" as *const u8 as *const libc::c_char,
                b"minisat/minisat.c\0" as *const u8 as *const libc::c_char,
                586 as libc::c_int,
            );
            1 as libc::c_int != 0
        }) as libc::c_int;
    (*s).stack.size = 0 as libc::c_int;
    if (*s).stack.size == (*s).stack.cap {
        let mut newsize: libc::c_int = (*s).stack.cap * 2 as libc::c_int
            + 1 as libc::c_int;
        (*s)
            .stack
            .ptr = yrealloc(
            (*s).stack.ptr as *mut libc::c_void,
            (::std::mem::size_of::<libc::c_int>() as libc::c_ulong)
                .wrapping_mul(newsize as libc::c_ulong) as libc::c_int,
        ) as *mut libc::c_int;
        (*s).stack.cap = newsize;
    }
    let fresh21 = (*s).stack.size;
    (*s).stack.size = (*s).stack.size + 1;
    *((*s).stack.ptr).offset(fresh21 as isize) = l >> 1 as libc::c_int;
    while (*s).stack.size > 0 as libc::c_int {
        let mut c: *mut clause = 0 as *mut clause;
        let mut v: libc::c_int = *((*s).stack.ptr)
            .offset(((*s).stack.size - 1 as libc::c_int) as isize);
        (v >= 0 as libc::c_int && v < (*s).size
            || {
                glp_assert_(
                    b"v >= 0 && v < s->size\0" as *const u8 as *const libc::c_char,
                    b"minisat/minisat.c\0" as *const u8 as *const libc::c_char,
                    593 as libc::c_int,
                );
                1 as libc::c_int != 0
            }) as libc::c_int;
        (*s).stack.size = (*s).stack.size - 1 as libc::c_int;
        (!(*reasons.offset(v as isize)).is_null()
            || {
                glp_assert_(
                    b"reasons[v] != 0\0" as *const u8 as *const libc::c_char,
                    b"minisat/minisat.c\0" as *const u8 as *const libc::c_char,
                    595 as libc::c_int,
                );
                1 as libc::c_int != 0
            }) as libc::c_int;
        c = *reasons.offset(v as isize);
        if c as size_t & 1 as libc::c_int as libc::c_ulong != 0 {
            let mut v_0: libc::c_int = (c as size_t >> 1 as libc::c_int) as lit
                >> 1 as libc::c_int;
            if *tags.offset(v_0 as isize) == 0 as libc::c_int
                && *levels.offset(v_0 as isize) != 0 as libc::c_int
            {
                if !(*reasons.offset(v_0 as isize)).is_null()
                    && (1 as libc::c_int)
                        << (*levels.offset(v_0 as isize) & 31 as libc::c_int) & minl != 0
                {
                    if (*s).stack.size == (*s).stack.cap {
                        let mut newsize_0: libc::c_int = (*s).stack.cap
                            * 2 as libc::c_int + 1 as libc::c_int;
                        (*s)
                            .stack
                            .ptr = yrealloc(
                            (*s).stack.ptr as *mut libc::c_void,
                            (::std::mem::size_of::<libc::c_int>() as libc::c_ulong)
                                .wrapping_mul(newsize_0 as libc::c_ulong) as libc::c_int,
                        ) as *mut libc::c_int;
                        (*s).stack.cap = newsize_0;
                    }
                    let fresh22 = (*s).stack.size;
                    (*s).stack.size = (*s).stack.size + 1;
                    *((*s).stack.ptr).offset(fresh22 as isize) = v_0;
                    *tags.offset(v_0 as isize) = 1 as libc::c_int;
                    if (*s).tagged.size == (*s).tagged.cap {
                        let mut newsize_1: libc::c_int = (*s).tagged.cap
                            * 2 as libc::c_int + 1 as libc::c_int;
                        (*s)
                            .tagged
                            .ptr = yrealloc(
                            (*s).tagged.ptr as *mut libc::c_void,
                            (::std::mem::size_of::<libc::c_int>() as libc::c_ulong)
                                .wrapping_mul(newsize_1 as libc::c_ulong) as libc::c_int,
                        ) as *mut libc::c_int;
                        (*s).tagged.cap = newsize_1;
                    }
                    let fresh23 = (*s).tagged.size;
                    (*s).tagged.size = (*s).tagged.size + 1;
                    *((*s).tagged.ptr).offset(fresh23 as isize) = v_0;
                } else {
                    let mut tagged: *mut libc::c_int = (*s).tagged.ptr;
                    let mut j: libc::c_int = 0;
                    j = top;
                    while j < (*s).tagged.size {
                        *tags
                            .offset(
                                *tagged.offset(j as isize) as isize,
                            ) = 0 as libc::c_int;
                        j += 1;
                        j;
                    }
                    (*s).tagged.size = top;
                    return 0 as libc::c_int;
                }
            }
        } else {
            let mut lits: *mut lit = ((*c).lits).as_mut_ptr();
            let mut i: libc::c_int = 0;
            let mut j_0: libc::c_int = 0;
            i = 1 as libc::c_int;
            while i < (*c).size_learnt >> 1 as libc::c_int {
                let mut v_1: libc::c_int = *lits.offset(i as isize) >> 1 as libc::c_int;
                if *tags.offset(v_1 as isize) == 0 as libc::c_int
                    && *levels.offset(v_1 as isize) != 0 as libc::c_int
                {
                    if !(*reasons.offset(v_1 as isize)).is_null()
                        && (1 as libc::c_int)
                            << (*levels.offset(v_1 as isize) & 31 as libc::c_int) & minl
                            != 0
                    {
                        if (*s).stack.size == (*s).stack.cap {
                            let mut newsize_2: libc::c_int = (*s).stack.cap
                                * 2 as libc::c_int + 1 as libc::c_int;
                            (*s)
                                .stack
                                .ptr = yrealloc(
                                (*s).stack.ptr as *mut libc::c_void,
                                (::std::mem::size_of::<libc::c_int>() as libc::c_ulong)
                                    .wrapping_mul(newsize_2 as libc::c_ulong) as libc::c_int,
                            ) as *mut libc::c_int;
                            (*s).stack.cap = newsize_2;
                        }
                        let fresh24 = (*s).stack.size;
                        (*s).stack.size = (*s).stack.size + 1;
                        *((*s).stack.ptr)
                            .offset(
                                fresh24 as isize,
                            ) = *lits.offset(i as isize) >> 1 as libc::c_int;
                        *tags.offset(v_1 as isize) = 1 as libc::c_int;
                        if (*s).tagged.size == (*s).tagged.cap {
                            let mut newsize_3: libc::c_int = (*s).tagged.cap
                                * 2 as libc::c_int + 1 as libc::c_int;
                            (*s)
                                .tagged
                                .ptr = yrealloc(
                                (*s).tagged.ptr as *mut libc::c_void,
                                (::std::mem::size_of::<libc::c_int>() as libc::c_ulong)
                                    .wrapping_mul(newsize_3 as libc::c_ulong) as libc::c_int,
                            ) as *mut libc::c_int;
                            (*s).tagged.cap = newsize_3;
                        }
                        let fresh25 = (*s).tagged.size;
                        (*s).tagged.size = (*s).tagged.size + 1;
                        *((*s).tagged.ptr).offset(fresh25 as isize) = v_1;
                    } else {
                        let mut tagged_0: *mut libc::c_int = (*s).tagged.ptr;
                        j_0 = top;
                        while j_0 < (*s).tagged.size {
                            *tags
                                .offset(
                                    *tagged_0.offset(j_0 as isize) as isize,
                                ) = 0 as libc::c_int;
                            j_0 += 1;
                            j_0;
                        }
                        (*s).tagged.size = top;
                        return 0 as libc::c_int;
                    }
                }
                i += 1;
                i;
            }
        }
    }
    return 1 as libc::c_int;
}
unsafe extern "C" fn solver_analyze(
    mut s: *mut solver,
    mut c: *mut clause,
    mut learnt: *mut veci,
) {
    let mut trail: *mut lit = (*s).trail;
    let mut tags: *mut lbool = (*s).tags;
    let mut reasons: *mut *mut clause = (*s).reasons;
    let mut levels: *mut libc::c_int = (*s).levels;
    let mut cnt: libc::c_int = 0 as libc::c_int;
    let mut p: lit = -(2 as libc::c_int);
    let mut ind: libc::c_int = (*s).qtail - 1 as libc::c_int;
    let mut lits: *mut lit = 0 as *mut lit;
    let mut i: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    let mut minl: libc::c_int = 0;
    let mut tagged: *mut libc::c_int = 0 as *mut libc::c_int;
    if (*learnt).size == (*learnt).cap {
        let mut newsize: libc::c_int = (*learnt).cap * 2 as libc::c_int
            + 1 as libc::c_int;
        (*learnt)
            .ptr = yrealloc(
            (*learnt).ptr as *mut libc::c_void,
            (::std::mem::size_of::<libc::c_int>() as libc::c_ulong)
                .wrapping_mul(newsize as libc::c_ulong) as libc::c_int,
        ) as *mut libc::c_int;
        (*learnt).cap = newsize;
    }
    let fresh26 = (*learnt).size;
    (*learnt).size = (*learnt).size + 1;
    *((*learnt).ptr).offset(fresh26 as isize) = -(2 as libc::c_int);
    loop {
        (!c.is_null()
            || {
                glp_assert_(
                    b"c != 0\0" as *const u8 as *const libc::c_char,
                    b"minisat/minisat.c\0" as *const u8 as *const libc::c_char,
                    659 as libc::c_int,
                );
                1 as libc::c_int != 0
            }) as libc::c_int;
        if c as size_t & 1 as libc::c_int as libc::c_ulong != 0 {
            let mut q: lit = (c as size_t >> 1 as libc::c_int) as lit;
            (q >> 1 as libc::c_int >= 0 as libc::c_int
                && (q >> 1 as libc::c_int) < (*s).size
                || {
                    glp_assert_(
                        b"lit_var(q) >= 0 && lit_var(q) < s->size\0" as *const u8
                            as *const libc::c_char,
                        b"minisat/minisat.c\0" as *const u8 as *const libc::c_char,
                        663 as libc::c_int,
                    );
                    1 as libc::c_int != 0
                }) as libc::c_int;
            if *tags.offset((q >> 1 as libc::c_int) as isize) == 0 as libc::c_int
                && *levels.offset((q >> 1 as libc::c_int) as isize) > 0 as libc::c_int
            {
                *tags.offset((q >> 1 as libc::c_int) as isize) = 1 as libc::c_int;
                if (*s).tagged.size == (*s).tagged.cap {
                    let mut newsize_0: libc::c_int = (*s).tagged.cap * 2 as libc::c_int
                        + 1 as libc::c_int;
                    (*s)
                        .tagged
                        .ptr = yrealloc(
                        (*s).tagged.ptr as *mut libc::c_void,
                        (::std::mem::size_of::<libc::c_int>() as libc::c_ulong)
                            .wrapping_mul(newsize_0 as libc::c_ulong) as libc::c_int,
                    ) as *mut libc::c_int;
                    (*s).tagged.cap = newsize_0;
                }
                let fresh27 = (*s).tagged.size;
                (*s).tagged.size = (*s).tagged.size + 1;
                *((*s).tagged.ptr).offset(fresh27 as isize) = q >> 1 as libc::c_int;
                act_var_bump(s, q >> 1 as libc::c_int);
                if *levels.offset((q >> 1 as libc::c_int) as isize)
                    == (*s).trail_lim.size
                {
                    cnt += 1;
                    cnt;
                } else {
                    if (*learnt).size == (*learnt).cap {
                        let mut newsize_1: libc::c_int = (*learnt).cap * 2 as libc::c_int
                            + 1 as libc::c_int;
                        (*learnt)
                            .ptr = yrealloc(
                            (*learnt).ptr as *mut libc::c_void,
                            (::std::mem::size_of::<libc::c_int>() as libc::c_ulong)
                                .wrapping_mul(newsize_1 as libc::c_ulong) as libc::c_int,
                        ) as *mut libc::c_int;
                        (*learnt).cap = newsize_1;
                    }
                    let fresh28 = (*learnt).size;
                    (*learnt).size = (*learnt).size + 1;
                    *((*learnt).ptr).offset(fresh28 as isize) = q;
                }
            }
        } else {
            if (*c).size_learnt & 1 as libc::c_int != 0 {
                act_clause_bump(s, c);
            }
            lits = ((*c).lits).as_mut_ptr();
            j = if p == -(2 as libc::c_int) {
                0 as libc::c_int
            } else {
                1 as libc::c_int
            };
            while j < (*c).size_learnt >> 1 as libc::c_int {
                let mut q_0: lit = *lits.offset(j as isize);
                (q_0 >> 1 as libc::c_int >= 0 as libc::c_int
                    && (q_0 >> 1 as libc::c_int) < (*s).size
                    || {
                        glp_assert_(
                            b"lit_var(q) >= 0 && lit_var(q) < s->size\0" as *const u8
                                as *const libc::c_char,
                            b"minisat/minisat.c\0" as *const u8 as *const libc::c_char,
                            682 as libc::c_int,
                        );
                        1 as libc::c_int != 0
                    }) as libc::c_int;
                if *tags.offset((q_0 >> 1 as libc::c_int) as isize) == 0 as libc::c_int
                    && *levels.offset((q_0 >> 1 as libc::c_int) as isize)
                        > 0 as libc::c_int
                {
                    *tags.offset((q_0 >> 1 as libc::c_int) as isize) = 1 as libc::c_int;
                    if (*s).tagged.size == (*s).tagged.cap {
                        let mut newsize_2: libc::c_int = (*s).tagged.cap
                            * 2 as libc::c_int + 1 as libc::c_int;
                        (*s)
                            .tagged
                            .ptr = yrealloc(
                            (*s).tagged.ptr as *mut libc::c_void,
                            (::std::mem::size_of::<libc::c_int>() as libc::c_ulong)
                                .wrapping_mul(newsize_2 as libc::c_ulong) as libc::c_int,
                        ) as *mut libc::c_int;
                        (*s).tagged.cap = newsize_2;
                    }
                    let fresh29 = (*s).tagged.size;
                    (*s).tagged.size = (*s).tagged.size + 1;
                    *((*s).tagged.ptr)
                        .offset(fresh29 as isize) = q_0 >> 1 as libc::c_int;
                    act_var_bump(s, q_0 >> 1 as libc::c_int);
                    if *levels.offset((q_0 >> 1 as libc::c_int) as isize)
                        == (*s).trail_lim.size
                    {
                        cnt += 1;
                        cnt;
                    } else {
                        if (*learnt).size == (*learnt).cap {
                            let mut newsize_3: libc::c_int = (*learnt).cap
                                * 2 as libc::c_int + 1 as libc::c_int;
                            (*learnt)
                                .ptr = yrealloc(
                                (*learnt).ptr as *mut libc::c_void,
                                (::std::mem::size_of::<libc::c_int>() as libc::c_ulong)
                                    .wrapping_mul(newsize_3 as libc::c_ulong) as libc::c_int,
                            ) as *mut libc::c_int;
                            (*learnt).cap = newsize_3;
                        }
                        let fresh30 = (*learnt).size;
                        (*learnt).size = (*learnt).size + 1;
                        *((*learnt).ptr).offset(fresh30 as isize) = q_0;
                    }
                }
                j += 1;
                j;
            }
        }
        loop {
            let fresh31 = ind;
            ind = ind - 1;
            if !(*tags
                .offset((*trail.offset(fresh31 as isize) >> 1 as libc::c_int) as isize)
                == 0 as libc::c_int)
            {
                break;
            }
        }
        p = *trail.offset((ind + 1 as libc::c_int) as isize);
        c = *reasons.offset((p >> 1 as libc::c_int) as isize);
        cnt -= 1;
        cnt;
        if !(cnt > 0 as libc::c_int) {
            break;
        }
    }
    *(*learnt).ptr = p ^ 1 as libc::c_int;
    lits = (*learnt).ptr;
    minl = 0 as libc::c_int;
    i = 1 as libc::c_int;
    while i < (*learnt).size {
        let mut lev: libc::c_int = *levels
            .offset((*lits.offset(i as isize) >> 1 as libc::c_int) as isize);
        minl |= (1 as libc::c_int) << (lev & 31 as libc::c_int);
        i += 1;
        i;
    }
    j = 1 as libc::c_int;
    i = j;
    while i < (*learnt).size {
        if (*reasons.offset((*lits.offset(i as isize) >> 1 as libc::c_int) as isize))
            .is_null() || solver_lit_removable(s, *lits.offset(i as isize), minl) == 0
        {
            let fresh32 = j;
            j = j + 1;
            *lits.offset(fresh32 as isize) = *lits.offset(i as isize);
        }
        i += 1;
        i;
    }
    (*s).stats.max_literals += (*learnt).size as libc::c_double;
    (*learnt).size = j;
    (*s).stats.tot_literals += j as libc::c_double;
    tagged = (*s).tagged.ptr;
    i = 0 as libc::c_int;
    while i < (*s).tagged.size {
        *tags.offset(*tagged.offset(i as isize) as isize) = 0 as libc::c_int;
        i += 1;
        i;
    }
    (*s).tagged.size = 0 as libc::c_int;
    if (*learnt).size > 1 as libc::c_int {
        let mut max_i: libc::c_int = 1 as libc::c_int;
        let mut max: libc::c_int = *levels
            .offset(
                (*lits.offset(1 as libc::c_int as isize) >> 1 as libc::c_int) as isize,
            );
        let mut tmp: lit = 0;
        i = 2 as libc::c_int;
        while i < (*learnt).size {
            if *levels.offset((*lits.offset(i as isize) >> 1 as libc::c_int) as isize)
                > max
            {
                max = *levels
                    .offset((*lits.offset(i as isize) >> 1 as libc::c_int) as isize);
                max_i = i;
            }
            i += 1;
            i;
        }
        tmp = *lits.offset(1 as libc::c_int as isize);
        *lits.offset(1 as libc::c_int as isize) = *lits.offset(max_i as isize);
        *lits.offset(max_i as isize) = tmp;
    }
}
pub unsafe extern "C" fn _glp_minisat_propagate(mut s: *mut solver) -> *mut clause {
    let mut values: *mut lbool = (*s).assigns;
    let mut confl: *mut clause = 0 as *mut clause;
    let mut lits: *mut lit = 0 as *mut lit;
    while confl.is_null() && (*s).qtail - (*s).qhead > 0 as libc::c_int {
        let fresh33 = (*s).qhead;
        (*s).qhead = (*s).qhead + 1;
        let mut p: lit = *((*s).trail).offset(fresh33 as isize);
        let mut ws: *mut vecp = &mut *((*s).wlists).offset(p as isize) as *mut vecp;
        let mut begin: *mut *mut clause = (*ws).ptr as *mut *mut clause;
        let mut end: *mut *mut clause = begin.offset((*ws).size as isize);
        let mut i: *mut *mut clause = 0 as *mut *mut clause;
        let mut j: *mut *mut clause = 0 as *mut *mut clause;
        (*s).stats.propagations += 1.;
        (*s).stats.propagations;
        (*s).simpdb_props -= 1;
        (*s).simpdb_props;
        j = begin;
        i = j;
        while i < end {
            let mut current_block_40: u64;
            if *i as size_t & 1 as libc::c_int as libc::c_ulong != 0 {
                let fresh34 = j;
                j = j.offset(1);
                *fresh34 = *i;
                if enqueue(
                    s,
                    (*i as size_t >> 1 as libc::c_int) as lit,
                    (p as size_t)
                        .wrapping_add(p as size_t)
                        .wrapping_add(1 as libc::c_int as libc::c_ulong) as *mut clause,
                ) == 0
                {
                    confl = (*s).binary;
                    *((*confl).lits)
                        .as_mut_ptr()
                        .offset(1 as libc::c_int as isize) = p ^ 1 as libc::c_int;
                    let fresh35 = i;
                    i = i.offset(1);
                    *((*confl).lits)
                        .as_mut_ptr()
                        .offset(
                            0 as libc::c_int as isize,
                        ) = (*fresh35 as size_t >> 1 as libc::c_int) as lit;
                    while i < end {
                        let fresh36 = i;
                        i = i.offset(1);
                        let fresh37 = j;
                        j = j.offset(1);
                        *fresh37 = *fresh36;
                    }
                }
            } else {
                let mut false_lit: lit = 0;
                let mut sig: lbool = 0;
                lits = ((**i).lits).as_mut_ptr();
                false_lit = p ^ 1 as libc::c_int;
                if *lits.offset(0 as libc::c_int as isize) == false_lit {
                    *lits
                        .offset(
                            0 as libc::c_int as isize,
                        ) = *lits.offset(1 as libc::c_int as isize);
                    *lits.offset(1 as libc::c_int as isize) = false_lit;
                }
                (*lits.offset(1 as libc::c_int as isize) == false_lit
                    || {
                        glp_assert_(
                            b"lits[1] == false_lit\0" as *const u8
                                as *const libc::c_char,
                            b"minisat/minisat.c\0" as *const u8 as *const libc::c_char,
                            807 as libc::c_int,
                        );
                        1 as libc::c_int != 0
                    }) as libc::c_int;
                sig = (*lits.offset(0 as libc::c_int as isize) & 1 as libc::c_int == 0)
                    as libc::c_int;
                sig += sig - 1 as libc::c_int;
                if *values
                    .offset(
                        (*lits.offset(0 as libc::c_int as isize) >> 1 as libc::c_int)
                            as isize,
                    ) == sig
                {
                    let fresh38 = j;
                    j = j.offset(1);
                    *fresh38 = *i;
                } else {
                    let mut stop: *mut lit = lits
                        .offset(((**i).size_learnt >> 1 as libc::c_int) as isize);
                    let mut k: *mut lit = 0 as *mut lit;
                    k = lits.offset(2 as libc::c_int as isize);
                    loop {
                        if !(k < stop) {
                            current_block_40 = 17184638872671510253;
                            break;
                        }
                        let mut sig_0: lbool = *k & 1 as libc::c_int;
                        sig_0 += sig_0 - 1 as libc::c_int;
                        if *values.offset((*k >> 1 as libc::c_int) as isize) != sig_0 {
                            *lits.offset(1 as libc::c_int as isize) = *k;
                            *k = false_lit;
                            if (*(&mut *((*s).wlists)
                                .offset(
                                    (*lits.offset(1 as libc::c_int as isize) ^ 1 as libc::c_int)
                                        as isize,
                                ) as *mut vecp))
                                .size
                                == (*(&mut *((*s).wlists)
                                    .offset(
                                        (*lits.offset(1 as libc::c_int as isize) ^ 1 as libc::c_int)
                                            as isize,
                                    ) as *mut vecp))
                                    .cap
                            {
                                let mut newsize: libc::c_int = (*(&mut *((*s).wlists)
                                    .offset(
                                        (*lits.offset(1 as libc::c_int as isize) ^ 1 as libc::c_int)
                                            as isize,
                                    ) as *mut vecp))
                                    .cap * 2 as libc::c_int + 1 as libc::c_int;
                                let ref mut fresh39 = (*(&mut *((*s).wlists)
                                    .offset(
                                        (*lits.offset(1 as libc::c_int as isize) ^ 1 as libc::c_int)
                                            as isize,
                                    ) as *mut vecp))
                                    .ptr;
                                *fresh39 = yrealloc(
                                    (*(&mut *((*s).wlists)
                                        .offset(
                                            (*lits.offset(1 as libc::c_int as isize) ^ 1 as libc::c_int)
                                                as isize,
                                        ) as *mut vecp))
                                        .ptr as *mut libc::c_void,
                                    (::std::mem::size_of::<*mut libc::c_void>()
                                        as libc::c_ulong)
                                        .wrapping_mul(newsize as libc::c_ulong) as libc::c_int,
                                ) as *mut *mut libc::c_void;
                                (*(&mut *((*s).wlists)
                                    .offset(
                                        (*lits.offset(1 as libc::c_int as isize) ^ 1 as libc::c_int)
                                            as isize,
                                    ) as *mut vecp))
                                    .cap = newsize;
                            }
                            let ref mut fresh40 = (*(&mut *((*s).wlists)
                                .offset(
                                    (*lits.offset(1 as libc::c_int as isize) ^ 1 as libc::c_int)
                                        as isize,
                                ) as *mut vecp))
                                .size;
                            let fresh41 = *fresh40;
                            *fresh40 = *fresh40 + 1;
                            let ref mut fresh42 = *((*(&mut *((*s).wlists)
                                .offset(
                                    (*lits.offset(1 as libc::c_int as isize) ^ 1 as libc::c_int)
                                        as isize,
                                ) as *mut vecp))
                                .ptr)
                                .offset(fresh41 as isize);
                            *fresh42 = *i as *mut libc::c_void;
                            current_block_40 = 18224646452157779169;
                            break;
                        } else {
                            k = k.offset(1);
                            k;
                        }
                    }
                    match current_block_40 {
                        18224646452157779169 => {}
                        _ => {
                            let fresh43 = j;
                            j = j.offset(1);
                            *fresh43 = *i;
                            if enqueue(s, *lits.offset(0 as libc::c_int as isize), *i)
                                == 0
                            {
                                let fresh44 = i;
                                i = i.offset(1);
                                confl = *fresh44;
                                while i < end {
                                    let fresh45 = i;
                                    i = i.offset(1);
                                    let fresh46 = j;
                                    j = j.offset(1);
                                    *fresh46 = *fresh45;
                                }
                            }
                        }
                    }
                }
            }
            i = i.offset(1);
            i;
        }
        (*s).stats.inspects
            += j.offset_from((*ws).ptr as *mut *mut clause) as libc::c_long
                as libc::c_double;
        (*ws)
            .size = j.offset_from((*ws).ptr as *mut *mut clause) as libc::c_long
            as libc::c_int;
    }
    return confl;
}
unsafe extern "C" fn clause_cmp(
    mut x: *const libc::c_void,
    mut y: *const libc::c_void,
) -> libc::c_int {
    return if (*(x as *mut clause)).size_learnt >> 1 as libc::c_int > 2 as libc::c_int
        && ((*(y as *mut clause)).size_learnt >> 1 as libc::c_int == 2 as libc::c_int
            || *(&mut *((*(x as *mut clause)).lits)
                .as_mut_ptr()
                .offset(((*(x as *mut clause)).size_learnt >> 1 as libc::c_int) as isize)
                as *mut lit as *mut libc::c_float)
                < *(&mut *((*(y as *mut clause)).lits)
                    .as_mut_ptr()
                    .offset(
                        ((*(y as *mut clause)).size_learnt >> 1 as libc::c_int) as isize,
                    ) as *mut lit as *mut libc::c_float))
    {
        -(1 as libc::c_int)
    } else {
        1 as libc::c_int
    };
}
pub unsafe extern "C" fn _glp_minisat_reducedb(mut s: *mut solver) {
    let mut i: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    let mut extra_lim: libc::c_double = ((*s).cla_inc
        / (*s).learnts.size as libc::c_float) as libc::c_double;
    let mut learnts: *mut *mut clause = (*s).learnts.ptr as *mut *mut clause;
    let mut reasons: *mut *mut clause = (*s).reasons;
    sort(
        (*s).learnts.ptr,
        (*s).learnts.size,
        Some(
            clause_cmp
                as unsafe extern "C" fn(
                    *const libc::c_void,
                    *const libc::c_void,
                ) -> libc::c_int,
        ),
    );
    j = 0 as libc::c_int;
    i = j;
    while i < (*s).learnts.size / 2 as libc::c_int {
        if (**learnts.offset(i as isize)).size_learnt >> 1 as libc::c_int
            > 2 as libc::c_int
            && *reasons
                .offset(
                    (*((**learnts.offset(i as isize)).lits).as_mut_ptr()
                        >> 1 as libc::c_int) as isize,
                ) != *learnts.offset(i as isize)
        {
            clause_remove(s, *learnts.offset(i as isize));
        } else {
            let fresh47 = j;
            j = j + 1;
            let ref mut fresh48 = *learnts.offset(fresh47 as isize);
            *fresh48 = *learnts.offset(i as isize);
        }
        i += 1;
        i;
    }
    while i < (*s).learnts.size {
        if (**learnts.offset(i as isize)).size_learnt >> 1 as libc::c_int
            > 2 as libc::c_int
            && *reasons
                .offset(
                    (*((**learnts.offset(i as isize)).lits).as_mut_ptr()
                        >> 1 as libc::c_int) as isize,
                ) != *learnts.offset(i as isize)
            && (*(&mut *((**learnts.offset(i as isize)).lits)
                .as_mut_ptr()
                .offset(
                    ((**learnts.offset(i as isize)).size_learnt >> 1 as libc::c_int)
                        as isize,
                ) as *mut lit as *mut libc::c_float) as libc::c_double) < extra_lim
        {
            clause_remove(s, *learnts.offset(i as isize));
        } else {
            let fresh49 = j;
            j = j + 1;
            let ref mut fresh50 = *learnts.offset(fresh49 as isize);
            *fresh50 = *learnts.offset(i as isize);
        }
        i += 1;
        i;
    }
    (*s).learnts.size = j;
}
unsafe extern "C" fn solver_search(
    mut s: *mut solver,
    mut nof_conflicts: libc::c_int,
    mut nof_learnts: libc::c_int,
) -> lbool {
    let mut levels: *mut libc::c_int = (*s).levels;
    let mut var_decay: libc::c_double = 0.95f64;
    let mut clause_decay: libc::c_double = 0.999f64;
    let mut random_var_freq: libc::c_double = 0.02f64;
    let mut conflictC: libc::c_int = 0 as libc::c_int;
    let mut learnt_clause: veci = veci {
        size: 0,
        cap: 0,
        ptr: 0 as *mut libc::c_int,
    };
    ((*s).root_level == (*s).trail_lim.size
        || {
            glp_assert_(
                b"s->root_level == solver_dlevel(s)\0" as *const u8
                    as *const libc::c_char,
                b"minisat/minisat.c\0" as *const u8 as *const libc::c_char,
                902 as libc::c_int,
            );
            1 as libc::c_int != 0
        }) as libc::c_int;
    (*s).stats.starts += 1.;
    (*s).stats.starts;
    (*s)
        .var_decay = (1 as libc::c_int as libc::c_double / var_decay) as libc::c_float
        as libc::c_double;
    (*s)
        .cla_decay = (1 as libc::c_int as libc::c_double / clause_decay)
        as libc::c_float;
    (*s).model.size = 0 as libc::c_int;
    learnt_clause.size = 0 as libc::c_int;
    learnt_clause.cap = 4 as libc::c_int;
    learnt_clause
        .ptr = ymalloc(
        (::std::mem::size_of::<libc::c_int>() as libc::c_ulong)
            .wrapping_mul(learnt_clause.cap as libc::c_ulong) as libc::c_int,
    ) as *mut libc::c_int;
    loop {
        let mut confl: *mut clause = _glp_minisat_propagate(s);
        if !confl.is_null() {
            let mut blevel: libc::c_int = 0;
            (*s).stats.conflicts += 1.;
            (*s).stats.conflicts;
            conflictC += 1;
            conflictC;
            if (*s).trail_lim.size == (*s).root_level {
                yfree(learnt_clause.ptr as *mut libc::c_void);
                return -(1 as libc::c_int);
            }
            learnt_clause.size = 0 as libc::c_int;
            solver_analyze(s, confl, &mut learnt_clause);
            blevel = if learnt_clause.size > 1 as libc::c_int {
                *levels
                    .offset(
                        (*(learnt_clause.ptr).offset(1 as libc::c_int as isize)
                            >> 1 as libc::c_int) as isize,
                    )
            } else {
                (*s).root_level
            };
            blevel = if (*s).root_level > blevel { (*s).root_level } else { blevel };
            solver_canceluntil(s, blevel);
            solver_record(s, &mut learnt_clause);
            act_var_decay(s);
            act_clause_decay(s);
        } else {
            let mut next: libc::c_int = 0;
            if nof_conflicts >= 0 as libc::c_int && conflictC >= nof_conflicts {
                (*s).progress_estimate = solver_progress(s);
                solver_canceluntil(s, (*s).root_level);
                yfree(learnt_clause.ptr as *mut libc::c_void);
                return 0 as libc::c_int;
            }
            if (*s).trail_lim.size == 0 as libc::c_int {
                _glp_minisat_simplify(s);
            }
            if nof_learnts >= 0 as libc::c_int
                && (*s).learnts.size - (*s).qtail >= nof_learnts
            {
                _glp_minisat_reducedb(s);
            }
            (*s).stats.decisions += 1.;
            (*s).stats.decisions;
            next = order_select(s, random_var_freq as libc::c_float);
            if next == -(1 as libc::c_int) {
                let mut values: *mut lbool = (*s).assigns;
                let mut i: libc::c_int = 0;
                i = 0 as libc::c_int;
                while i < (*s).size {
                    if (*s).model.size == (*s).model.cap {
                        let mut newsize: libc::c_int = (*s).model.cap * 2 as libc::c_int
                            + 1 as libc::c_int;
                        (*s)
                            .model
                            .ptr = yrealloc(
                            (*s).model.ptr as *mut libc::c_void,
                            (::std::mem::size_of::<libc::c_int>() as libc::c_ulong)
                                .wrapping_mul(newsize as libc::c_ulong) as libc::c_int,
                        ) as *mut libc::c_int;
                        (*s).model.cap = newsize;
                    }
                    let fresh51 = (*s).model.size;
                    (*s).model.size = (*s).model.size + 1;
                    *((*s).model.ptr)
                        .offset(fresh51 as isize) = *values.offset(i as isize);
                    i += 1;
                    i;
                }
                solver_canceluntil(s, (*s).root_level);
                yfree(learnt_clause.ptr as *mut libc::c_void);
                return 1 as libc::c_int;
            }
            assume(s, next + next ^ 1 as libc::c_int);
        }
    };
}
pub unsafe extern "C" fn _glp_minisat_new() -> *mut solver {
    let mut s: *mut solver = ymalloc(
        ::std::mem::size_of::<solver>() as libc::c_ulong as libc::c_int,
    ) as *mut solver;
    (*s).clauses.size = 0 as libc::c_int;
    (*s).clauses.cap = 4 as libc::c_int;
    (*s)
        .clauses
        .ptr = ymalloc(
        (::std::mem::size_of::<*mut libc::c_void>() as libc::c_ulong)
            .wrapping_mul((*s).clauses.cap as libc::c_ulong) as libc::c_int,
    ) as *mut *mut libc::c_void;
    (*s).learnts.size = 0 as libc::c_int;
    (*s).learnts.cap = 4 as libc::c_int;
    (*s)
        .learnts
        .ptr = ymalloc(
        (::std::mem::size_of::<*mut libc::c_void>() as libc::c_ulong)
            .wrapping_mul((*s).learnts.cap as libc::c_ulong) as libc::c_int,
    ) as *mut *mut libc::c_void;
    (*s).order.size = 0 as libc::c_int;
    (*s).order.cap = 4 as libc::c_int;
    (*s)
        .order
        .ptr = ymalloc(
        (::std::mem::size_of::<libc::c_int>() as libc::c_ulong)
            .wrapping_mul((*s).order.cap as libc::c_ulong) as libc::c_int,
    ) as *mut libc::c_int;
    (*s).trail_lim.size = 0 as libc::c_int;
    (*s).trail_lim.cap = 4 as libc::c_int;
    (*s)
        .trail_lim
        .ptr = ymalloc(
        (::std::mem::size_of::<libc::c_int>() as libc::c_ulong)
            .wrapping_mul((*s).trail_lim.cap as libc::c_ulong) as libc::c_int,
    ) as *mut libc::c_int;
    (*s).tagged.size = 0 as libc::c_int;
    (*s).tagged.cap = 4 as libc::c_int;
    (*s)
        .tagged
        .ptr = ymalloc(
        (::std::mem::size_of::<libc::c_int>() as libc::c_ulong)
            .wrapping_mul((*s).tagged.cap as libc::c_ulong) as libc::c_int,
    ) as *mut libc::c_int;
    (*s).stack.size = 0 as libc::c_int;
    (*s).stack.cap = 4 as libc::c_int;
    (*s)
        .stack
        .ptr = ymalloc(
        (::std::mem::size_of::<libc::c_int>() as libc::c_ulong)
            .wrapping_mul((*s).stack.cap as libc::c_ulong) as libc::c_int,
    ) as *mut libc::c_int;
    (*s).model.size = 0 as libc::c_int;
    (*s).model.cap = 4 as libc::c_int;
    (*s)
        .model
        .ptr = ymalloc(
        (::std::mem::size_of::<libc::c_int>() as libc::c_ulong)
            .wrapping_mul((*s).model.cap as libc::c_ulong) as libc::c_int,
    ) as *mut libc::c_int;
    (*s).wlists = 0 as *mut vecp;
    (*s).activity = 0 as *mut libc::c_double;
    (*s).assigns = 0 as *mut lbool;
    (*s).orderpos = 0 as *mut libc::c_int;
    (*s).reasons = 0 as *mut *mut clause;
    (*s).levels = 0 as *mut libc::c_int;
    (*s).tags = 0 as *mut lbool;
    (*s).trail = 0 as *mut lit;
    (*s).size = 0 as libc::c_int;
    (*s).cap = 0 as libc::c_int;
    (*s).qhead = 0 as libc::c_int;
    (*s).qtail = 0 as libc::c_int;
    (*s).cla_inc = 1 as libc::c_int as libc::c_float;
    (*s).cla_decay = 1 as libc::c_int as libc::c_float;
    (*s).var_inc = 1 as libc::c_int as libc::c_double;
    (*s).var_decay = 1 as libc::c_int as libc::c_double;
    (*s).root_level = 0 as libc::c_int;
    (*s).simpdb_assigns = 0 as libc::c_int;
    (*s).simpdb_props = 0 as libc::c_int;
    (*s).random_seed = 91648253 as libc::c_int as libc::c_double;
    (*s).progress_estimate = 0 as libc::c_int as libc::c_double;
    (*s)
        .binary = ymalloc(
        (::std::mem::size_of::<clause>() as libc::c_ulong)
            .wrapping_add(
                (::std::mem::size_of::<lit>() as libc::c_ulong)
                    .wrapping_mul(2 as libc::c_int as libc::c_ulong),
            ) as libc::c_int,
    ) as *mut clause;
    (*(*s).binary).size_learnt = (2 as libc::c_int) << 1 as libc::c_int;
    (*s).verbosity = 0 as libc::c_int;
    (*s).stats.starts = 0 as libc::c_int as libc::c_double;
    (*s).stats.decisions = 0 as libc::c_int as libc::c_double;
    (*s).stats.propagations = 0 as libc::c_int as libc::c_double;
    (*s).stats.inspects = 0 as libc::c_int as libc::c_double;
    (*s).stats.conflicts = 0 as libc::c_int as libc::c_double;
    (*s).stats.clauses = 0 as libc::c_int as libc::c_double;
    (*s).stats.clauses_literals = 0 as libc::c_int as libc::c_double;
    (*s).stats.learnts = 0 as libc::c_int as libc::c_double;
    (*s).stats.learnts_literals = 0 as libc::c_int as libc::c_double;
    (*s).stats.max_literals = 0 as libc::c_int as libc::c_double;
    (*s).stats.tot_literals = 0 as libc::c_int as libc::c_double;
    return s;
}
pub unsafe extern "C" fn _glp_minisat_delete(mut s: *mut solver) {
    let mut i: libc::c_int = 0;
    i = 0 as libc::c_int;
    while i < (*s).clauses.size {
        yfree(*((*s).clauses.ptr).offset(i as isize));
        i += 1;
        i;
    }
    i = 0 as libc::c_int;
    while i < (*s).learnts.size {
        yfree(*((*s).learnts.ptr).offset(i as isize));
        i += 1;
        i;
    }
    yfree((*s).clauses.ptr as *mut libc::c_void);
    yfree((*s).learnts.ptr as *mut libc::c_void);
    yfree((*s).order.ptr as *mut libc::c_void);
    yfree((*s).trail_lim.ptr as *mut libc::c_void);
    yfree((*s).tagged.ptr as *mut libc::c_void);
    yfree((*s).stack.ptr as *mut libc::c_void);
    yfree((*s).model.ptr as *mut libc::c_void);
    yfree((*s).binary as *mut libc::c_void);
    if !((*s).wlists).is_null() {
        let mut i_0: libc::c_int = 0;
        i_0 = 0 as libc::c_int;
        while i_0 < (*s).size * 2 as libc::c_int {
            yfree((*((*s).wlists).offset(i_0 as isize)).ptr as *mut libc::c_void);
            i_0 += 1;
            i_0;
        }
        yfree((*s).wlists as *mut libc::c_void);
        yfree((*s).activity as *mut libc::c_void);
        yfree((*s).assigns as *mut libc::c_void);
        yfree((*s).orderpos as *mut libc::c_void);
        yfree((*s).reasons as *mut libc::c_void);
        yfree((*s).levels as *mut libc::c_void);
        yfree((*s).trail as *mut libc::c_void);
        yfree((*s).tags as *mut libc::c_void);
    }
    yfree(s as *mut libc::c_void);
}
pub unsafe extern "C" fn _glp_minisat_addclause(
    mut s: *mut solver,
    mut begin: *mut lit,
    mut end: *mut lit,
) -> bool_0 {
    let mut i: *mut lit = 0 as *mut lit;
    let mut j: *mut lit = 0 as *mut lit;
    let mut maxvar: libc::c_int = 0;
    let mut values: *mut lbool = 0 as *mut lbool;
    let mut last: lit = 0;
    if begin == end {
        return 0 as libc::c_int;
    }
    maxvar = *begin >> 1 as libc::c_int;
    i = begin.offset(1 as libc::c_int as isize);
    while i < end {
        let mut l: lit = *i;
        maxvar = if l >> 1 as libc::c_int > maxvar {
            l >> 1 as libc::c_int
        } else {
            maxvar
        };
        j = i;
        while j > begin && *j.offset(-(1 as libc::c_int as isize)) > l {
            *j = *j.offset(-(1 as libc::c_int as isize));
            j = j.offset(-1);
            j;
        }
        *j = l;
        i = i.offset(1);
        i;
    }
    _glp_minisat_setnvars(s, maxvar + 1 as libc::c_int);
    values = (*s).assigns;
    last = -(2 as libc::c_int);
    j = begin;
    i = j;
    while i < end {
        let mut sig: lbool = (*i & 1 as libc::c_int == 0) as libc::c_int;
        sig += sig - 1 as libc::c_int;
        if *i == last ^ 1 as libc::c_int
            || sig == *values.offset((*i >> 1 as libc::c_int) as isize)
        {
            return 1 as libc::c_int
        } else if *i != last
            && *values.offset((*i >> 1 as libc::c_int) as isize) == 0 as libc::c_int
        {
            let fresh52 = j;
            j = j.offset(1);
            *fresh52 = *i;
            last = *fresh52;
        }
        i = i.offset(1);
        i;
    }
    if j == begin {
        return 0 as libc::c_int
    } else if j.offset_from(begin) as libc::c_long == 1 as libc::c_int as libc::c_long {
        return enqueue(s, *begin, 0 as *mut clause)
    }
    if (*s).clauses.size == (*s).clauses.cap {
        let mut newsize: libc::c_int = (*s).clauses.cap * 2 as libc::c_int
            + 1 as libc::c_int;
        (*s)
            .clauses
            .ptr = yrealloc(
            (*s).clauses.ptr as *mut libc::c_void,
            (::std::mem::size_of::<*mut libc::c_void>() as libc::c_ulong)
                .wrapping_mul(newsize as libc::c_ulong) as libc::c_int,
        ) as *mut *mut libc::c_void;
        (*s).clauses.cap = newsize;
    }
    let fresh53 = (*s).clauses.size;
    (*s).clauses.size = (*s).clauses.size + 1;
    let ref mut fresh54 = *((*s).clauses.ptr).offset(fresh53 as isize);
    *fresh54 = clause_new(s, begin, j, 0 as libc::c_int) as *mut libc::c_void;
    (*s).stats.clauses += 1.;
    (*s).stats.clauses;
    (*s).stats.clauses_literals
        += j.offset_from(begin) as libc::c_long as libc::c_double;
    return 1 as libc::c_int;
}
pub unsafe extern "C" fn _glp_minisat_simplify(mut s: *mut solver) -> bool_0 {
    let mut reasons: *mut *mut clause = 0 as *mut *mut clause;
    let mut type_0: libc::c_int = 0;
    ((*s).trail_lim.size == 0 as libc::c_int
        || {
            glp_assert_(
                b"solver_dlevel(s) == 0\0" as *const u8 as *const libc::c_char,
                b"minisat/minisat.c\0" as *const u8 as *const libc::c_char,
                1148 as libc::c_int,
            );
            1 as libc::c_int != 0
        }) as libc::c_int;
    if !(_glp_minisat_propagate(s)).is_null() {
        return 0 as libc::c_int;
    }
    if (*s).qhead == (*s).simpdb_assigns || (*s).simpdb_props > 0 as libc::c_int {
        return 1 as libc::c_int;
    }
    reasons = (*s).reasons;
    type_0 = 0 as libc::c_int;
    while type_0 < 2 as libc::c_int {
        let mut cs: *mut vecp = if type_0 != 0 {
            &mut (*s).learnts
        } else {
            &mut (*s).clauses
        };
        let mut cls: *mut *mut clause = (*cs).ptr as *mut *mut clause;
        let mut i: libc::c_int = 0;
        let mut j: libc::c_int = 0;
        i = 0 as libc::c_int;
        j = i;
        while i < (*cs).size {
            if *reasons
                .offset(
                    (*((**cls.offset(i as isize)).lits).as_mut_ptr() >> 1 as libc::c_int)
                        as isize,
                ) != *cls.offset(i as isize)
                && clause_simplify(s, *cls.offset(i as isize)) == 1 as libc::c_int
            {
                clause_remove(s, *cls.offset(i as isize));
            } else {
                let fresh55 = j;
                j = j + 1;
                let ref mut fresh56 = *cls.offset(fresh55 as isize);
                *fresh56 = *cls.offset(i as isize);
            }
            i += 1;
            i;
        }
        (*cs).size = j;
        type_0 += 1;
        type_0;
    }
    (*s).simpdb_assigns = (*s).qhead;
    (*s)
        .simpdb_props = ((*s).stats.clauses_literals + (*s).stats.learnts_literals)
        as libc::c_int;
    return 1 as libc::c_int;
}
pub unsafe extern "C" fn _glp_minisat_solve(
    mut s: *mut solver,
    mut begin: *mut lit,
    mut end: *mut lit,
) -> bool_0 {
    let mut nof_conflicts: libc::c_double = 100 as libc::c_int as libc::c_double;
    let mut nof_learnts: libc::c_double = (_glp_minisat_nclauses(s) / 3 as libc::c_int)
        as libc::c_double;
    let mut status: lbool = 0 as libc::c_int;
    let mut values: *mut lbool = (*s).assigns;
    let mut i: *mut lit = 0 as *mut lit;
    i = begin;
    while i < end {
        let mut current_block_3: u64;
        match if *i & 1 as libc::c_int != 0 {
            -*values.offset((*i >> 1 as libc::c_int) as isize)
        } else {
            *values.offset((*i >> 1 as libc::c_int) as isize)
        } {
            0 => {
                assume(s, *i);
                if (_glp_minisat_propagate(s)).is_null() {
                    current_block_3 = 8515828400728868193;
                } else {
                    current_block_3 = 17874056243412667812;
                }
            }
            -1 => {
                current_block_3 = 17874056243412667812;
            }
            1 | _ => {
                current_block_3 = 8515828400728868193;
            }
        }
        match current_block_3 {
            8515828400728868193 => {}
            _ => {
                solver_canceluntil(s, 0 as libc::c_int);
                return 0 as libc::c_int;
            }
        }
        i = i.offset(1);
        i;
    }
    (*s).root_level = (*s).trail_lim.size;
    if (*s).verbosity >= 1 as libc::c_int {
        glp_printf(
            b"==================================[MINISAT]===================================\n\0"
                as *const u8 as *const libc::c_char,
        );
        glp_printf(
            b"| Conflicts |     ORIGINAL     |              LEARNT              | Progress |\n\0"
                as *const u8 as *const libc::c_char,
        );
        glp_printf(
            b"|           | Clauses Literals |   Limit Clauses Literals  Lit/Cl |          |\n\0"
                as *const u8 as *const libc::c_char,
        );
        glp_printf(
            b"==============================================================================\n\0"
                as *const u8 as *const libc::c_char,
        );
    }
    while status == 0 as libc::c_int {
        let mut Ratio: libc::c_double = if (*s).stats.learnts
            == 0 as libc::c_int as libc::c_double
        {
            0.0f64
        } else {
            (*s).stats.learnts_literals / (*s).stats.learnts
        };
        if (*s).verbosity >= 1 as libc::c_int {
            glp_printf(
                b"| %9.0f | %7.0f %8.0f | %7.0f %7.0f %8.0f %7.1f | %6.3f %% |\n\0"
                    as *const u8 as *const libc::c_char,
                (*s).stats.conflicts,
                (*s).stats.clauses,
                (*s).stats.clauses_literals,
                nof_learnts,
                (*s).stats.learnts,
                (*s).stats.learnts_literals,
                Ratio,
                (*s).progress_estimate * 100 as libc::c_int as libc::c_double,
            );
        }
        status = solver_search(
            s,
            nof_conflicts as libc::c_int,
            nof_learnts as libc::c_int,
        );
        nof_conflicts *= 1.5f64;
        nof_learnts *= 1.1f64;
    }
    if (*s).verbosity >= 1 as libc::c_int {
        glp_printf(
            b"==============================================================================\n\0"
                as *const u8 as *const libc::c_char,
        );
    }
    solver_canceluntil(s, 0 as libc::c_int);
    return (status != -(1 as libc::c_int)) as libc::c_int;
}
pub unsafe extern "C" fn _glp_minisat_nvars(mut s: *mut solver) -> libc::c_int {
    return (*s).size;
}
pub unsafe extern "C" fn _glp_minisat_nclauses(mut s: *mut solver) -> libc::c_int {
    return (*s).clauses.size;
}
pub unsafe extern "C" fn _glp_minisat_nconflicts(mut s: *mut solver) -> libc::c_int {
    return (*s).stats.conflicts as libc::c_int;
}
unsafe extern "C" fn selectionsort(
    mut array: *mut *mut libc::c_void,
    mut size: libc::c_int,
    mut comp: Option::<
        unsafe extern "C" fn(*const libc::c_void, *const libc::c_void) -> libc::c_int,
    >,
) {
    let mut i: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    let mut best_i: libc::c_int = 0;
    let mut tmp: *mut libc::c_void = 0 as *mut libc::c_void;
    i = 0 as libc::c_int;
    while i < size - 1 as libc::c_int {
        best_i = i;
        j = i + 1 as libc::c_int;
        while j < size {
            if comp.unwrap()(*array.offset(j as isize), *array.offset(best_i as isize))
                < 0 as libc::c_int
            {
                best_i = j;
            }
            j += 1;
            j;
        }
        tmp = *array.offset(i as isize);
        let ref mut fresh57 = *array.offset(i as isize);
        *fresh57 = *array.offset(best_i as isize);
        let ref mut fresh58 = *array.offset(best_i as isize);
        *fresh58 = tmp;
        i += 1;
        i;
    }
}
unsafe extern "C" fn sortrnd(
    mut array: *mut *mut libc::c_void,
    mut size: libc::c_int,
    mut comp: Option::<
        unsafe extern "C" fn(*const libc::c_void, *const libc::c_void) -> libc::c_int,
    >,
    mut seed: *mut libc::c_double,
) {
    if size <= 15 as libc::c_int {
        selectionsort(array, size, comp);
    } else {
        let mut pivot: *mut libc::c_void = *array.offset(irand(seed, size) as isize);
        let mut tmp: *mut libc::c_void = 0 as *mut libc::c_void;
        let mut i: libc::c_int = -(1 as libc::c_int);
        let mut j: libc::c_int = size;
        loop {
            loop {
                i += 1;
                i;
                if !(comp.unwrap()(*array.offset(i as isize), pivot) < 0 as libc::c_int)
                {
                    break;
                }
            }
            loop {
                j -= 1;
                j;
                if !(comp.unwrap()(pivot, *array.offset(j as isize)) < 0 as libc::c_int)
                {
                    break;
                }
            }
            if i >= j {
                break;
            }
            tmp = *array.offset(i as isize);
            let ref mut fresh59 = *array.offset(i as isize);
            *fresh59 = *array.offset(j as isize);
            let ref mut fresh60 = *array.offset(j as isize);
            *fresh60 = tmp;
        }
        sortrnd(array, i, comp, seed);
        sortrnd(&mut *array.offset(i as isize), size - i, comp, seed);
    };
}
unsafe extern "C" fn sort(
    mut array: *mut *mut libc::c_void,
    mut size: libc::c_int,
    mut comp: Option::<
        unsafe extern "C" fn(*const libc::c_void, *const libc::c_void) -> libc::c_int,
    >,
) {
    let mut seed: libc::c_double = 91648253 as libc::c_int as libc::c_double;
    sortrnd(array, size, comp, &mut seed);
}
