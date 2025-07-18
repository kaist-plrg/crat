use ::libc;
extern "C" {
    fn fabs(_: libc::c_double) -> libc::c_double;
    fn floor(_: libc::c_double) -> libc::c_double;
    fn memcpy(
        _: *mut libc::c_void,
        _: *const libc::c_void,
        _: libc::c_ulong,
    ) -> *mut libc::c_void;
    fn glp_assert_(
        expr: *const libc::c_char,
        file: *const libc::c_char,
        line: libc::c_int,
    );
    fn glp_alloc(n: libc::c_int, size: libc::c_int) -> *mut libc::c_void;
    fn glp_free(ptr: *mut libc::c_void);
    fn glp_error_(file: *const libc::c_char, line: libc::c_int) -> glp_errfunc;
    fn _glp_relax4(csa: *mut relax4_csa) -> libc::c_int;
    fn _glp_relax4_inidat(csa: *mut relax4_csa);
}
pub type glp_errfunc = Option::<unsafe extern "C" fn(*const libc::c_char, ...) -> ()>;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct glp_graph {
    pub pool: *mut libc::c_void,
    pub name: *mut libc::c_char,
    pub nv_max: libc::c_int,
    pub nv: libc::c_int,
    pub na: libc::c_int,
    pub v: *mut *mut glp_vertex,
    pub index: *mut libc::c_void,
    pub v_size: libc::c_int,
    pub a_size: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct glp_vertex {
    pub i: libc::c_int,
    pub name: *mut libc::c_char,
    pub entry: *mut libc::c_void,
    pub data: *mut libc::c_void,
    pub temp: *mut libc::c_void,
    pub in_0: *mut glp_arc,
    pub out: *mut glp_arc,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct glp_arc {
    pub tail: *mut glp_vertex,
    pub head: *mut glp_vertex,
    pub data: *mut libc::c_void,
    pub temp: *mut libc::c_void,
    pub t_prev: *mut glp_arc,
    pub t_next: *mut glp_arc,
    pub h_prev: *mut glp_arc,
    pub h_next: *mut glp_arc,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct relax4_csa {
    pub n: libc::c_int,
    pub na: libc::c_int,
    pub large: libc::c_int,
    pub repeat: libc::c_int,
    pub crash: libc::c_int,
    pub startn: *mut libc::c_int,
    pub endn: *mut libc::c_int,
    pub fou: *mut libc::c_int,
    pub nxtou: *mut libc::c_int,
    pub fin: *mut libc::c_int,
    pub nxtin: *mut libc::c_int,
    pub rc: *mut libc::c_int,
    pub u: *mut libc::c_int,
    pub dfct: *mut libc::c_int,
    pub x: *mut libc::c_int,
    pub nmultinode: libc::c_int,
    pub iter: libc::c_int,
    pub num_augm: libc::c_int,
    pub num_ascnt: libc::c_int,
    pub nsp: libc::c_int,
    pub label: *mut libc::c_int,
    pub prdcsr: *mut libc::c_int,
    pub save: *mut libc::c_int,
    pub tfstou: *mut libc::c_int,
    pub tnxtou: *mut libc::c_int,
    pub tfstin: *mut libc::c_int,
    pub tnxtin: *mut libc::c_int,
    pub nxtqueue: *mut libc::c_int,
    pub scan: *mut libc::c_char,
    pub mark: *mut libc::c_char,
    pub extend_arc: *mut libc::c_int,
    pub sb_level: *mut libc::c_int,
    pub sb_arc: *mut libc::c_int,
}
unsafe extern "C" fn overflow(mut u: libc::c_int, mut v: libc::c_int) -> libc::c_int {
    if u > 0 as libc::c_int && v > 0 as libc::c_int && u + v < 0 as libc::c_int {
        return 1 as libc::c_int;
    }
    if u < 0 as libc::c_int && v < 0 as libc::c_int && u + v > 0 as libc::c_int {
        return 1 as libc::c_int;
    }
    return 0 as libc::c_int;
}
pub unsafe extern "C" fn glp_mincost_relax4(
    mut G: *mut glp_graph,
    mut v_rhs: libc::c_int,
    mut a_low: libc::c_int,
    mut a_cap: libc::c_int,
    mut a_cost: libc::c_int,
    mut crash: libc::c_int,
    mut sol: *mut libc::c_double,
    mut a_x: libc::c_int,
    mut a_rc: libc::c_int,
) -> libc::c_int {
    let mut current_block: u64;
    let mut v: *mut glp_vertex = 0 as *mut glp_vertex;
    let mut a: *mut glp_arc = 0 as *mut glp_arc;
    let mut csa: relax4_csa = relax4_csa {
        n: 0,
        na: 0,
        large: 0,
        repeat: 0,
        crash: 0,
        startn: 0 as *mut libc::c_int,
        endn: 0 as *mut libc::c_int,
        fou: 0 as *mut libc::c_int,
        nxtou: 0 as *mut libc::c_int,
        fin: 0 as *mut libc::c_int,
        nxtin: 0 as *mut libc::c_int,
        rc: 0 as *mut libc::c_int,
        u: 0 as *mut libc::c_int,
        dfct: 0 as *mut libc::c_int,
        x: 0 as *mut libc::c_int,
        nmultinode: 0,
        iter: 0,
        num_augm: 0,
        num_ascnt: 0,
        nsp: 0,
        label: 0 as *mut libc::c_int,
        prdcsr: 0 as *mut libc::c_int,
        save: 0 as *mut libc::c_int,
        tfstou: 0 as *mut libc::c_int,
        tnxtou: 0 as *mut libc::c_int,
        tfstin: 0 as *mut libc::c_int,
        tnxtin: 0 as *mut libc::c_int,
        nxtqueue: 0 as *mut libc::c_int,
        scan: 0 as *mut libc::c_char,
        mark: 0 as *mut libc::c_char,
        extend_arc: 0 as *mut libc::c_int,
        sb_level: 0 as *mut libc::c_int,
        sb_arc: 0 as *mut libc::c_int,
    };
    let mut i: libc::c_int = 0;
    let mut k: libc::c_int = 0;
    let mut large: libc::c_int = 0;
    let mut n: libc::c_int = 0;
    let mut na: libc::c_int = 0;
    let mut ret: libc::c_int = 0;
    let mut cap: libc::c_double = 0.;
    let mut cost: libc::c_double = 0.;
    let mut low: libc::c_double = 0.;
    let mut rc: libc::c_double = 0.;
    let mut rhs: libc::c_double = 0.;
    let mut sum: libc::c_double = 0.;
    let mut x: libc::c_double = 0.;
    if v_rhs >= 0 as libc::c_int
        && v_rhs
            > (*G).v_size
                - ::std::mem::size_of::<libc::c_double>() as libc::c_ulong as libc::c_int
    {
        (glp_error_(
            b"api/mcfrelax.c\0" as *const u8 as *const libc::c_char,
            43 as libc::c_int,
        ))
            .unwrap()(
            b"glp_mincost_relax4: v_rhs = %d; invalid offset\n\0" as *const u8
                as *const libc::c_char,
            v_rhs,
        );
    }
    if a_low >= 0 as libc::c_int
        && a_low
            > (*G).a_size
                - ::std::mem::size_of::<libc::c_double>() as libc::c_ulong as libc::c_int
    {
        (glp_error_(
            b"api/mcfrelax.c\0" as *const u8 as *const libc::c_char,
            46 as libc::c_int,
        ))
            .unwrap()(
            b"glp_mincost_relax4: a_low = %d; invalid offset\n\0" as *const u8
                as *const libc::c_char,
            a_low,
        );
    }
    if a_cap >= 0 as libc::c_int
        && a_cap
            > (*G).a_size
                - ::std::mem::size_of::<libc::c_double>() as libc::c_ulong as libc::c_int
    {
        (glp_error_(
            b"api/mcfrelax.c\0" as *const u8 as *const libc::c_char,
            49 as libc::c_int,
        ))
            .unwrap()(
            b"glp_mincost_relax4: a_cap = %d; invalid offset\n\0" as *const u8
                as *const libc::c_char,
            a_cap,
        );
    }
    if a_cost >= 0 as libc::c_int
        && a_cost
            > (*G).a_size
                - ::std::mem::size_of::<libc::c_double>() as libc::c_ulong as libc::c_int
    {
        (glp_error_(
            b"api/mcfrelax.c\0" as *const u8 as *const libc::c_char,
            52 as libc::c_int,
        ))
            .unwrap()(
            b"glp_mincost_relax4: a_cost = %d; invalid offset\n\0" as *const u8
                as *const libc::c_char,
            a_cost,
        );
    }
    if a_x >= 0 as libc::c_int
        && a_x
            > (*G).a_size
                - ::std::mem::size_of::<libc::c_double>() as libc::c_ulong as libc::c_int
    {
        (glp_error_(
            b"api/mcfrelax.c\0" as *const u8 as *const libc::c_char,
            55 as libc::c_int,
        ))
            .unwrap()(
            b"glp_mincost_relax4: a_x = %d; invalid offset\n\0" as *const u8
                as *const libc::c_char,
            a_x,
        );
    }
    if a_rc >= 0 as libc::c_int
        && a_rc
            > (*G).a_size
                - ::std::mem::size_of::<libc::c_double>() as libc::c_ulong as libc::c_int
    {
        (glp_error_(
            b"api/mcfrelax.c\0" as *const u8 as *const libc::c_char,
            58 as libc::c_int,
        ))
            .unwrap()(
            b"glp_mincost_relax4: a_rc = %d; invalid offset\n\0" as *const u8
                as *const libc::c_char,
            a_rc,
        );
    }
    n = (*G).nv;
    csa.n = n;
    na = (*G).na;
    csa.na = na;
    large = 2147483647 as libc::c_int / 4 as libc::c_int;
    csa.large = large;
    csa.repeat = 0 as libc::c_int;
    csa.crash = crash;
    csa
        .startn = glp_alloc(
        1 as libc::c_int + na,
        ::std::mem::size_of::<libc::c_int>() as libc::c_ulong as libc::c_int,
    ) as *mut libc::c_int;
    csa
        .endn = glp_alloc(
        1 as libc::c_int + na,
        ::std::mem::size_of::<libc::c_int>() as libc::c_ulong as libc::c_int,
    ) as *mut libc::c_int;
    csa
        .fou = glp_alloc(
        1 as libc::c_int + n,
        ::std::mem::size_of::<libc::c_int>() as libc::c_ulong as libc::c_int,
    ) as *mut libc::c_int;
    csa
        .nxtou = glp_alloc(
        1 as libc::c_int + na,
        ::std::mem::size_of::<libc::c_int>() as libc::c_ulong as libc::c_int,
    ) as *mut libc::c_int;
    csa
        .fin = glp_alloc(
        1 as libc::c_int + n,
        ::std::mem::size_of::<libc::c_int>() as libc::c_ulong as libc::c_int,
    ) as *mut libc::c_int;
    csa
        .nxtin = glp_alloc(
        1 as libc::c_int + na,
        ::std::mem::size_of::<libc::c_int>() as libc::c_ulong as libc::c_int,
    ) as *mut libc::c_int;
    csa
        .rc = glp_alloc(
        1 as libc::c_int + na,
        ::std::mem::size_of::<libc::c_int>() as libc::c_ulong as libc::c_int,
    ) as *mut libc::c_int;
    csa
        .u = glp_alloc(
        1 as libc::c_int + na,
        ::std::mem::size_of::<libc::c_int>() as libc::c_ulong as libc::c_int,
    ) as *mut libc::c_int;
    csa
        .dfct = glp_alloc(
        1 as libc::c_int + n,
        ::std::mem::size_of::<libc::c_int>() as libc::c_ulong as libc::c_int,
    ) as *mut libc::c_int;
    csa
        .x = glp_alloc(
        1 as libc::c_int + na,
        ::std::mem::size_of::<libc::c_int>() as libc::c_ulong as libc::c_int,
    ) as *mut libc::c_int;
    csa
        .label = glp_alloc(
        1 as libc::c_int + n,
        ::std::mem::size_of::<libc::c_int>() as libc::c_ulong as libc::c_int,
    ) as *mut libc::c_int;
    csa
        .prdcsr = glp_alloc(
        1 as libc::c_int + n,
        ::std::mem::size_of::<libc::c_int>() as libc::c_ulong as libc::c_int,
    ) as *mut libc::c_int;
    csa
        .save = glp_alloc(
        1 as libc::c_int + na,
        ::std::mem::size_of::<libc::c_int>() as libc::c_ulong as libc::c_int,
    ) as *mut libc::c_int;
    csa
        .tfstou = glp_alloc(
        1 as libc::c_int + n,
        ::std::mem::size_of::<libc::c_int>() as libc::c_ulong as libc::c_int,
    ) as *mut libc::c_int;
    csa
        .tnxtou = glp_alloc(
        1 as libc::c_int + na,
        ::std::mem::size_of::<libc::c_int>() as libc::c_ulong as libc::c_int,
    ) as *mut libc::c_int;
    csa
        .tfstin = glp_alloc(
        1 as libc::c_int + n,
        ::std::mem::size_of::<libc::c_int>() as libc::c_ulong as libc::c_int,
    ) as *mut libc::c_int;
    csa
        .tnxtin = glp_alloc(
        1 as libc::c_int + na,
        ::std::mem::size_of::<libc::c_int>() as libc::c_ulong as libc::c_int,
    ) as *mut libc::c_int;
    csa
        .nxtqueue = glp_alloc(
        1 as libc::c_int + n,
        ::std::mem::size_of::<libc::c_int>() as libc::c_ulong as libc::c_int,
    ) as *mut libc::c_int;
    csa
        .scan = glp_alloc(
        1 as libc::c_int + n,
        ::std::mem::size_of::<libc::c_char>() as libc::c_ulong as libc::c_int,
    ) as *mut libc::c_char;
    csa
        .mark = glp_alloc(
        1 as libc::c_int + n,
        ::std::mem::size_of::<libc::c_char>() as libc::c_ulong as libc::c_int,
    ) as *mut libc::c_char;
    if crash != 0 {
        csa
            .extend_arc = glp_alloc(
            1 as libc::c_int + n,
            ::std::mem::size_of::<libc::c_int>() as libc::c_ulong as libc::c_int,
        ) as *mut libc::c_int;
        csa
            .sb_level = glp_alloc(
            1 as libc::c_int + n,
            ::std::mem::size_of::<libc::c_int>() as libc::c_ulong as libc::c_int,
        ) as *mut libc::c_int;
        csa
            .sb_arc = glp_alloc(
            1 as libc::c_int + n,
            ::std::mem::size_of::<libc::c_int>() as libc::c_ulong as libc::c_int,
        ) as *mut libc::c_int;
    } else {
        csa.extend_arc = 0 as *mut libc::c_int;
        csa.sb_level = 0 as *mut libc::c_int;
        csa.sb_arc = 0 as *mut libc::c_int;
    }
    i = 1 as libc::c_int;
    loop {
        if !(i <= n) {
            current_block = 10891380440665537214;
            break;
        }
        v = *((*G).v).offset(i as isize);
        if v_rhs >= 0 as libc::c_int {
            memcpy(
                &mut rhs as *mut libc::c_double as *mut libc::c_void,
                ((*v).data as *mut libc::c_char).offset(v_rhs as isize)
                    as *const libc::c_void,
                ::std::mem::size_of::<libc::c_double>() as libc::c_ulong,
            );
        } else {
            rhs = 0.0f64;
        }
        if !(fabs(rhs) <= large as libc::c_double && rhs == floor(rhs)) {
            ret = 0x12 as libc::c_int;
            current_block = 13311817818311261487;
            break;
        } else {
            *(csa.dfct).offset(i as isize) = -(rhs as libc::c_int);
            i += 1;
            i;
        }
    }
    match current_block {
        10891380440665537214 => {
            k = 0 as libc::c_int;
            i = 1 as libc::c_int;
            's_211: loop {
                if !(i <= n) {
                    current_block = 10512632378975961025;
                    break;
                }
                v = *((*G).v).offset(i as isize);
                a = (*v).out;
                while !a.is_null() {
                    k += 1;
                    k;
                    if (*(*a).tail).i == (*(*a).head).i {
                        ret = 0x12 as libc::c_int;
                        current_block = 13311817818311261487;
                        break 's_211;
                    } else {
                        *(csa.startn).offset(k as isize) = (*(*a).tail).i;
                        *(csa.endn).offset(k as isize) = (*(*a).head).i;
                        if a_cost >= 0 as libc::c_int {
                            memcpy(
                                &mut cost as *mut libc::c_double as *mut libc::c_void,
                                ((*a).data as *mut libc::c_char).offset(a_cost as isize)
                                    as *const libc::c_void,
                                ::std::mem::size_of::<libc::c_double>() as libc::c_ulong,
                            );
                        } else {
                            cost = 0.0f64;
                        }
                        if !(fabs(cost) <= large as libc::c_double
                            && cost == floor(cost))
                        {
                            ret = 0x12 as libc::c_int;
                            current_block = 13311817818311261487;
                            break 's_211;
                        } else {
                            *(csa.rc).offset(k as isize) = cost as libc::c_int;
                            if a_low >= 0 as libc::c_int {
                                memcpy(
                                    &mut low as *mut libc::c_double as *mut libc::c_void,
                                    ((*a).data as *mut libc::c_char).offset(a_low as isize)
                                        as *const libc::c_void,
                                    ::std::mem::size_of::<libc::c_double>() as libc::c_ulong,
                                );
                            } else {
                                low = 0.0f64;
                            }
                            if !(0.0f64 <= low && low <= large as libc::c_double
                                && low == floor(low))
                            {
                                ret = 0x12 as libc::c_int;
                                current_block = 13311817818311261487;
                                break 's_211;
                            } else {
                                if a_cap >= 0 as libc::c_int {
                                    memcpy(
                                        &mut cap as *mut libc::c_double as *mut libc::c_void,
                                        ((*a).data as *mut libc::c_char).offset(a_cap as isize)
                                            as *const libc::c_void,
                                        ::std::mem::size_of::<libc::c_double>() as libc::c_ulong,
                                    );
                                } else {
                                    cap = 1.0f64;
                                }
                                if !(low <= cap && cap <= large as libc::c_double
                                    && cap == floor(cap))
                                {
                                    ret = 0x12 as libc::c_int;
                                    current_block = 13311817818311261487;
                                    break 's_211;
                                } else {
                                    *(csa.u).offset(k as isize) = (cap - low) as libc::c_int;
                                    if overflow(
                                        *(csa.dfct).offset((*(*a).tail).i as isize),
                                        low as libc::c_int,
                                    ) != 0
                                    {
                                        ret = 0x13 as libc::c_int;
                                        current_block = 13311817818311261487;
                                        break 's_211;
                                    } else {
                                        *(csa.dfct).offset((*(*a).tail).i as isize)
                                            += low as libc::c_int;
                                        if overflow(
                                            *(csa.dfct).offset((*(*a).head).i as isize),
                                            -low as libc::c_int,
                                        ) != 0
                                        {
                                            ret = 0x13 as libc::c_int;
                                            current_block = 13311817818311261487;
                                            break 's_211;
                                        } else {
                                            *(csa.dfct).offset((*(*a).head).i as isize)
                                                -= low as libc::c_int;
                                            a = (*a).t_next;
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
                i += 1;
                i;
            }
            match current_block {
                13311817818311261487 => {}
                _ => {
                    _glp_relax4_inidat(&mut csa);
                    ret = _glp_relax4(&mut csa);
                    if ret != 0 as libc::c_int {
                        (1 as libc::c_int <= ret && ret <= 8 as libc::c_int
                            || {
                                glp_assert_(
                                    b"1 <= ret && ret <= 8\0" as *const u8
                                        as *const libc::c_char,
                                    b"api/mcfrelax.c\0" as *const u8 as *const libc::c_char,
                                    184 as libc::c_int,
                                );
                                1 as libc::c_int != 0
                            }) as libc::c_int;
                        ret = 0xa as libc::c_int;
                    } else {
                        sum = 0.0f64;
                        k = 0 as libc::c_int;
                        i = 1 as libc::c_int;
                        while i <= n {
                            v = *((*G).v).offset(i as isize);
                            a = (*v).out;
                            while !a.is_null() {
                                k += 1;
                                k;
                                if a_low >= 0 as libc::c_int {
                                    memcpy(
                                        &mut low as *mut libc::c_double as *mut libc::c_void,
                                        ((*a).data as *mut libc::c_char).offset(a_low as isize)
                                            as *const libc::c_void,
                                        ::std::mem::size_of::<libc::c_double>() as libc::c_ulong,
                                    );
                                } else {
                                    low = 0.0f64;
                                }
                                x = *(csa.x).offset(k as isize) as libc::c_double + low;
                                if a_x >= 0 as libc::c_int {
                                    memcpy(
                                        ((*a).data as *mut libc::c_char).offset(a_x as isize)
                                            as *mut libc::c_void,
                                        &mut x as *mut libc::c_double as *const libc::c_void,
                                        ::std::mem::size_of::<libc::c_double>() as libc::c_ulong,
                                    );
                                }
                                rc = *(csa.rc).offset(k as isize) as libc::c_double;
                                if a_rc >= 0 as libc::c_int {
                                    memcpy(
                                        ((*a).data as *mut libc::c_char).offset(a_rc as isize)
                                            as *mut libc::c_void,
                                        &mut rc as *mut libc::c_double as *const libc::c_void,
                                        ::std::mem::size_of::<libc::c_double>() as libc::c_ulong,
                                    );
                                }
                                if a_cost >= 0 as libc::c_int {
                                    memcpy(
                                        &mut cost as *mut libc::c_double as *mut libc::c_void,
                                        ((*a).data as *mut libc::c_char).offset(a_cost as isize)
                                            as *const libc::c_void,
                                        ::std::mem::size_of::<libc::c_double>() as libc::c_ulong,
                                    );
                                } else {
                                    cost = 0.0f64;
                                }
                                sum += cost * x;
                                a = (*a).t_next;
                            }
                            i += 1;
                            i;
                        }
                        if !sol.is_null() {
                            *sol = sum;
                        }
                    }
                }
            }
        }
        _ => {}
    }
    glp_free(csa.startn as *mut libc::c_void);
    glp_free(csa.endn as *mut libc::c_void);
    glp_free(csa.fou as *mut libc::c_void);
    glp_free(csa.nxtou as *mut libc::c_void);
    glp_free(csa.fin as *mut libc::c_void);
    glp_free(csa.nxtin as *mut libc::c_void);
    glp_free(csa.rc as *mut libc::c_void);
    glp_free(csa.u as *mut libc::c_void);
    glp_free(csa.dfct as *mut libc::c_void);
    glp_free(csa.x as *mut libc::c_void);
    glp_free(csa.label as *mut libc::c_void);
    glp_free(csa.prdcsr as *mut libc::c_void);
    glp_free(csa.save as *mut libc::c_void);
    glp_free(csa.tfstou as *mut libc::c_void);
    glp_free(csa.tnxtou as *mut libc::c_void);
    glp_free(csa.tfstin as *mut libc::c_void);
    glp_free(csa.tnxtin as *mut libc::c_void);
    glp_free(csa.nxtqueue as *mut libc::c_void);
    glp_free(csa.scan as *mut libc::c_void);
    glp_free(csa.mark as *mut libc::c_void);
    if crash != 0 {
        glp_free(csa.extend_arc as *mut libc::c_void);
        glp_free(csa.sb_level as *mut libc::c_void);
        glp_free(csa.sb_arc as *mut libc::c_void);
    }
    return ret;
}
