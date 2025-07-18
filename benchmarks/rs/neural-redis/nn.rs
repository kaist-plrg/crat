use ::libc;
extern "C" {
    fn printf(_: *const libc::c_char, _: ...) -> libc::c_int;
    fn rand() -> libc::c_int;
    fn malloc(_: libc::c_ulong) -> *mut libc::c_void;
    fn free(__ptr: *mut libc::c_void);
    fn exp(_: libc::c_double) -> libc::c_double;
    fn memcpy(
        _: *mut libc::c_void,
        _: *const libc::c_void,
        _: libc::c_ulong,
    ) -> *mut libc::c_void;
    fn memset(
        _: *mut libc::c_void,
        _: libc::c_int,
        _: libc::c_ulong,
    ) -> *mut libc::c_void;
}
pub type size_t = libc::c_ulong;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct AnnLayer {
    pub output: *mut libc::c_float,
    pub error: *mut libc::c_float,
    pub weight: *mut libc::c_float,
    pub gradient: *mut libc::c_float,
    pub sgradient: *mut libc::c_float,
    pub pgradient: *mut libc::c_float,
    pub delta: *mut libc::c_float,
    pub units: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct Ann {
    pub flags: libc::c_int,
    pub layers: libc::c_int,
    pub rprop_nminus: libc::c_float,
    pub rprop_nplus: libc::c_float,
    pub rprop_maxupdate: libc::c_float,
    pub rprop_minupdate: libc::c_float,
    pub learn_rate: libc::c_float,
    pub _filler_: libc::c_float,
    pub layer: *mut AnnLayer,
}
pub type AnnTrainAlgoFunc = Option::<
    unsafe extern "C" fn(
        *mut Ann,
        *mut libc::c_float,
        *mut libc::c_float,
        libc::c_int,
    ) -> libc::c_float,
>;
pub unsafe extern "C" fn sigmoid(mut x: libc::c_float) -> libc::c_float {
    return (1 as libc::c_int as libc::c_float as libc::c_double
        / (1 as libc::c_int as libc::c_double + exp(-x as libc::c_double)))
        as libc::c_float;
}
pub unsafe extern "C" fn relu(mut x: libc::c_float) -> libc::c_float {
    return if x > 0 as libc::c_int as libc::c_float {
        x
    } else {
        0 as libc::c_int as libc::c_float
    };
}
pub unsafe extern "C" fn AnnResetLayer(mut layer: *mut AnnLayer) {
    (*layer).units = 0 as libc::c_int;
    (*layer).output = 0 as *mut libc::c_float;
    (*layer).error = 0 as *mut libc::c_float;
    (*layer).weight = 0 as *mut libc::c_float;
    (*layer).gradient = 0 as *mut libc::c_float;
    (*layer).pgradient = 0 as *mut libc::c_float;
    (*layer).delta = 0 as *mut libc::c_float;
    (*layer).sgradient = 0 as *mut libc::c_float;
}
pub unsafe extern "C" fn AnnAlloc(mut layers: libc::c_int) -> *mut Ann {
    let mut net: *mut Ann = 0 as *mut Ann;
    let mut i: libc::c_int = 0;
    net = malloc(::std::mem::size_of::<Ann>() as libc::c_ulong) as *mut Ann;
    if net.is_null() {
        return 0 as *mut Ann;
    }
    (*net)
        .layer = malloc(
        (::std::mem::size_of::<AnnLayer>() as libc::c_ulong)
            .wrapping_mul(layers as libc::c_ulong),
    ) as *mut AnnLayer;
    if ((*net).layer).is_null() {
        free(net as *mut libc::c_void);
        return 0 as *mut Ann;
    }
    (*net).layers = layers;
    (*net).flags = 0 as libc::c_int;
    (*net).rprop_nminus = 0.5f64 as libc::c_float;
    (*net).rprop_nplus = 1.2f64 as libc::c_float;
    (*net).rprop_maxupdate = 50 as libc::c_int as libc::c_float;
    (*net).rprop_minupdate = 0.000001f64 as libc::c_float;
    i = 0 as libc::c_int;
    while i < layers {
        AnnResetLayer(&mut *((*net).layer).offset(i as isize));
        i += 1;
        i;
    }
    return net;
}
pub unsafe extern "C" fn AnnFreeLayer(mut layer: *mut AnnLayer) {
    free((*layer).output as *mut libc::c_void);
    free((*layer).error as *mut libc::c_void);
    free((*layer).weight as *mut libc::c_void);
    free((*layer).gradient as *mut libc::c_void);
    free((*layer).pgradient as *mut libc::c_void);
    free((*layer).delta as *mut libc::c_void);
    free((*layer).sgradient as *mut libc::c_void);
    AnnResetLayer(layer);
}
pub unsafe extern "C" fn AnnFree(mut net: *mut Ann) {
    let mut i: libc::c_int = 0;
    i = 0 as libc::c_int;
    while i < (*net).layers {
        AnnFreeLayer(&mut *((*net).layer).offset(i as isize));
        i += 1;
        i;
    }
    free((*net).layer as *mut libc::c_void);
    free(net as *mut libc::c_void);
}
pub unsafe extern "C" fn AnnInitLayer(
    mut net: *mut Ann,
    mut i: libc::c_int,
    mut units: libc::c_int,
    mut bias: libc::c_int,
) -> libc::c_int {
    if bias != 0 {
        units += 1;
        units;
    }
    let ref mut fresh0 = (*((*net).layer).offset(i as isize)).output;
    *fresh0 = malloc(
        (::std::mem::size_of::<libc::c_float>() as libc::c_ulong)
            .wrapping_mul(units as libc::c_ulong),
    ) as *mut libc::c_float;
    let ref mut fresh1 = (*((*net).layer).offset(i as isize)).error;
    *fresh1 = malloc(
        (::std::mem::size_of::<libc::c_float>() as libc::c_ulong)
            .wrapping_mul(units as libc::c_ulong),
    ) as *mut libc::c_float;
    if i != 0 {
        let ref mut fresh2 = (*((*net).layer).offset(i as isize)).weight;
        *fresh2 = malloc(
            (::std::mem::size_of::<libc::c_float>() as libc::c_ulong)
                .wrapping_mul(units as libc::c_ulong)
                .wrapping_mul(
                    (*((*net).layer).offset((i - 1 as libc::c_int) as isize)).units
                        as libc::c_ulong,
                ),
        ) as *mut libc::c_float;
        let ref mut fresh3 = (*((*net).layer).offset(i as isize)).gradient;
        *fresh3 = malloc(
            (::std::mem::size_of::<libc::c_float>() as libc::c_ulong)
                .wrapping_mul(units as libc::c_ulong)
                .wrapping_mul(
                    (*((*net).layer).offset((i - 1 as libc::c_int) as isize)).units
                        as libc::c_ulong,
                ),
        ) as *mut libc::c_float;
        let ref mut fresh4 = (*((*net).layer).offset(i as isize)).pgradient;
        *fresh4 = malloc(
            (::std::mem::size_of::<libc::c_float>() as libc::c_ulong)
                .wrapping_mul(units as libc::c_ulong)
                .wrapping_mul(
                    (*((*net).layer).offset((i - 1 as libc::c_int) as isize)).units
                        as libc::c_ulong,
                ),
        ) as *mut libc::c_float;
        let ref mut fresh5 = (*((*net).layer).offset(i as isize)).delta;
        *fresh5 = malloc(
            (::std::mem::size_of::<libc::c_float>() as libc::c_ulong)
                .wrapping_mul(units as libc::c_ulong)
                .wrapping_mul(
                    (*((*net).layer).offset((i - 1 as libc::c_int) as isize)).units
                        as libc::c_ulong,
                ),
        ) as *mut libc::c_float;
        let ref mut fresh6 = (*((*net).layer).offset(i as isize)).sgradient;
        *fresh6 = malloc(
            (::std::mem::size_of::<libc::c_float>() as libc::c_ulong)
                .wrapping_mul(units as libc::c_ulong)
                .wrapping_mul(
                    (*((*net).layer).offset((i - 1 as libc::c_int) as isize)).units
                        as libc::c_ulong,
                ),
        ) as *mut libc::c_float;
    }
    (*((*net).layer).offset(i as isize)).units = units;
    if ((*((*net).layer).offset(i as isize)).output).is_null()
        || ((*((*net).layer).offset(i as isize)).error).is_null()
        || i != 0 && ((*((*net).layer).offset(i as isize)).weight).is_null()
        || i != 0 && ((*((*net).layer).offset(i as isize)).gradient).is_null()
        || i != 0 && ((*((*net).layer).offset(i as isize)).pgradient).is_null()
        || i != 0 && ((*((*net).layer).offset(i as isize)).sgradient).is_null()
        || i != 0 && ((*((*net).layer).offset(i as isize)).delta).is_null()
    {
        AnnFreeLayer(&mut *((*net).layer).offset(i as isize));
        AnnResetLayer(&mut *((*net).layer).offset(i as isize));
        return 1 as libc::c_int;
    }
    memset(
        (*((*net).layer).offset(i as isize)).output as *mut libc::c_void,
        0 as libc::c_int,
        (::std::mem::size_of::<libc::c_float>() as libc::c_ulong)
            .wrapping_mul(units as libc::c_ulong),
    );
    memset(
        (*((*net).layer).offset(i as isize)).error as *mut libc::c_void,
        0 as libc::c_int,
        (::std::mem::size_of::<libc::c_float>() as libc::c_ulong)
            .wrapping_mul(units as libc::c_ulong),
    );
    if i != 0 {
        memset(
            (*((*net).layer).offset(i as isize)).weight as *mut libc::c_void,
            0 as libc::c_int,
            (::std::mem::size_of::<libc::c_float>() as libc::c_ulong)
                .wrapping_mul(units as libc::c_ulong)
                .wrapping_mul(
                    (*((*net).layer).offset((i - 1 as libc::c_int) as isize)).units
                        as libc::c_ulong,
                ),
        );
        memset(
            (*((*net).layer).offset(i as isize)).gradient as *mut libc::c_void,
            0 as libc::c_int,
            (::std::mem::size_of::<libc::c_float>() as libc::c_ulong)
                .wrapping_mul(units as libc::c_ulong)
                .wrapping_mul(
                    (*((*net).layer).offset((i - 1 as libc::c_int) as isize)).units
                        as libc::c_ulong,
                ),
        );
        memset(
            (*((*net).layer).offset(i as isize)).pgradient as *mut libc::c_void,
            0 as libc::c_int,
            (::std::mem::size_of::<libc::c_float>() as libc::c_ulong)
                .wrapping_mul(units as libc::c_ulong)
                .wrapping_mul(
                    (*((*net).layer).offset((i - 1 as libc::c_int) as isize)).units
                        as libc::c_ulong,
                ),
        );
        memset(
            (*((*net).layer).offset(i as isize)).delta as *mut libc::c_void,
            0 as libc::c_int,
            (::std::mem::size_of::<libc::c_float>() as libc::c_ulong)
                .wrapping_mul(units as libc::c_ulong)
                .wrapping_mul(
                    (*((*net).layer).offset((i - 1 as libc::c_int) as isize)).units
                        as libc::c_ulong,
                ),
        );
        memset(
            (*((*net).layer).offset(i as isize)).sgradient as *mut libc::c_void,
            0 as libc::c_int,
            (::std::mem::size_of::<libc::c_float>() as libc::c_ulong)
                .wrapping_mul(units as libc::c_ulong)
                .wrapping_mul(
                    (*((*net).layer).offset((i - 1 as libc::c_int) as isize)).units
                        as libc::c_ulong,
                ),
        );
    }
    if bias != 0 {
        *((*((*net).layer).offset(i as isize)).output)
            .offset(
                (units - 1 as libc::c_int) as isize,
            ) = 1 as libc::c_int as libc::c_float;
    }
    return 0 as libc::c_int;
}
pub unsafe extern "C" fn AnnClone(mut net: *mut Ann) -> *mut Ann {
    let mut copy: *mut Ann = 0 as *mut Ann;
    let mut j: libc::c_int = 0;
    copy = AnnAlloc((*net).layers);
    if copy.is_null() {
        return 0 as *mut Ann;
    }
    j = 0 as libc::c_int;
    while j < (*net).layers {
        let mut ldst: *mut AnnLayer = 0 as *mut AnnLayer;
        let mut lsrc: *mut AnnLayer = 0 as *mut AnnLayer;
        let mut units: libc::c_int = (*((*net).layer).offset(j as isize)).units;
        let mut bias: libc::c_int = (j > 0 as libc::c_int) as libc::c_int;
        if AnnInitLayer(copy, j, units - bias, bias) != 0 {
            AnnFree(copy);
            return 0 as *mut Ann;
        }
        lsrc = &mut *((*net).layer).offset(j as isize) as *mut AnnLayer;
        ldst = &mut *((*copy).layer).offset(j as isize) as *mut AnnLayer;
        if !((*lsrc).output).is_null() {
            memcpy(
                (*ldst).output as *mut libc::c_void,
                (*lsrc).output as *const libc::c_void,
                (::std::mem::size_of::<libc::c_float>() as libc::c_ulong)
                    .wrapping_mul(units as libc::c_ulong),
            );
        }
        if !((*lsrc).error).is_null() {
            memcpy(
                (*ldst).error as *mut libc::c_void,
                (*lsrc).error as *const libc::c_void,
                (::std::mem::size_of::<libc::c_float>() as libc::c_ulong)
                    .wrapping_mul(units as libc::c_ulong),
            );
        }
        if j != 0 {
            let mut weights: libc::c_int = (*((*net).layer).offset(j as isize)).units
                * (*((*net).layer).offset((j - 1 as libc::c_int) as isize)).units;
            if !((*lsrc).weight).is_null() {
                memcpy(
                    (*ldst).weight as *mut libc::c_void,
                    (*lsrc).weight as *const libc::c_void,
                    (::std::mem::size_of::<libc::c_float>() as libc::c_ulong)
                        .wrapping_mul(weights as libc::c_ulong),
                );
            }
            if !((*lsrc).gradient).is_null() {
                memcpy(
                    (*ldst).gradient as *mut libc::c_void,
                    (*lsrc).gradient as *const libc::c_void,
                    (::std::mem::size_of::<libc::c_float>() as libc::c_ulong)
                        .wrapping_mul(weights as libc::c_ulong),
                );
            }
            if !((*lsrc).pgradient).is_null() {
                memcpy(
                    (*ldst).pgradient as *mut libc::c_void,
                    (*lsrc).pgradient as *const libc::c_void,
                    (::std::mem::size_of::<libc::c_float>() as libc::c_ulong)
                        .wrapping_mul(weights as libc::c_ulong),
                );
            }
            if !((*lsrc).delta).is_null() {
                memcpy(
                    (*ldst).delta as *mut libc::c_void,
                    (*lsrc).delta as *const libc::c_void,
                    (::std::mem::size_of::<libc::c_float>() as libc::c_ulong)
                        .wrapping_mul(weights as libc::c_ulong),
                );
            }
            if !((*lsrc).sgradient).is_null() {
                memcpy(
                    (*ldst).sgradient as *mut libc::c_void,
                    (*lsrc).sgradient as *const libc::c_void,
                    (::std::mem::size_of::<libc::c_float>() as libc::c_ulong)
                        .wrapping_mul(weights as libc::c_ulong),
                );
            }
        }
        j += 1;
        j;
    }
    (*copy).rprop_nminus = (*net).rprop_nminus;
    (*copy).rprop_nplus = (*net).rprop_nplus;
    (*copy).rprop_maxupdate = (*net).rprop_maxupdate;
    (*copy).rprop_minupdate = (*net).rprop_minupdate;
    (*copy).flags = (*net).flags;
    return copy;
}
pub unsafe extern "C" fn AnnCreateNet(
    mut layers: libc::c_int,
    mut units: *mut libc::c_int,
) -> *mut Ann {
    let mut net: *mut Ann = 0 as *mut Ann;
    let mut i: libc::c_int = 0;
    net = AnnAlloc(layers);
    if net.is_null() {
        return 0 as *mut Ann;
    }
    i = 0 as libc::c_int;
    while i < layers {
        if AnnInitLayer(
            net,
            i,
            *units.offset(i as isize),
            (i > 0 as libc::c_int) as libc::c_int,
        ) != 0
        {
            AnnFree(net);
            return 0 as *mut Ann;
        }
        i += 1;
        i;
    }
    AnnSetRandomWeights(net);
    AnnSetDeltas(net, 0.1f64 as libc::c_float);
    (*net).learn_rate = 0.1f64 as libc::c_float;
    return net;
}
pub unsafe extern "C" fn AnnCountWeights(mut net: *mut Ann) -> size_t {
    let mut weights: size_t = 0 as libc::c_int as size_t;
    let mut i: libc::c_int = (*net).layers - 1 as libc::c_int;
    while i > 0 as libc::c_int {
        let mut nextunits: libc::c_int = (*((*net).layer)
            .offset((i - 1 as libc::c_int) as isize))
            .units;
        let mut units: libc::c_int = (*((*net).layer).offset(i as isize)).units;
        if i > 1 as libc::c_int {
            nextunits -= 1;
            nextunits;
        }
        weights = (weights as libc::c_ulong)
            .wrapping_add((units * nextunits) as libc::c_ulong) as size_t as size_t;
        i -= 1;
        i;
    }
    return weights;
}
pub unsafe extern "C" fn AnnCreateNet4(
    mut iunits: libc::c_int,
    mut hunits: libc::c_int,
    mut hunits2: libc::c_int,
    mut ounits: libc::c_int,
) -> *mut Ann {
    let mut units: [libc::c_int; 4] = [0; 4];
    units[0 as libc::c_int as usize] = ounits;
    units[1 as libc::c_int as usize] = hunits2;
    units[2 as libc::c_int as usize] = hunits;
    units[3 as libc::c_int as usize] = iunits;
    return AnnCreateNet(4 as libc::c_int, units.as_mut_ptr());
}
pub unsafe extern "C" fn AnnCreateNet3(
    mut iunits: libc::c_int,
    mut hunits: libc::c_int,
    mut ounits: libc::c_int,
) -> *mut Ann {
    let mut units: [libc::c_int; 3] = [0; 3];
    units[0 as libc::c_int as usize] = ounits;
    units[1 as libc::c_int as usize] = hunits;
    units[2 as libc::c_int as usize] = iunits;
    return AnnCreateNet(3 as libc::c_int, units.as_mut_ptr());
}
pub unsafe extern "C" fn AnnCreateNet2(
    mut iunits: libc::c_int,
    mut ounits: libc::c_int,
) -> *mut Ann {
    let mut units: [libc::c_int; 2] = [0; 2];
    units[0 as libc::c_int as usize] = ounits;
    units[1 as libc::c_int as usize] = iunits;
    return AnnCreateNet(2 as libc::c_int, units.as_mut_ptr());
}
pub unsafe extern "C" fn AnnSimulate(mut net: *mut Ann) {
    let mut i: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    let mut k: libc::c_int = 0;
    i = (*net).layers - 1 as libc::c_int;
    while i > 0 as libc::c_int {
        let mut nextunits: libc::c_int = (*((*net).layer)
            .offset((i - 1 as libc::c_int) as isize))
            .units;
        let mut units: libc::c_int = (*((*net).layer).offset(i as isize)).units;
        if i > 1 as libc::c_int {
            nextunits -= 1;
            nextunits;
        }
        j = 0 as libc::c_int;
        while j < nextunits {
            let mut A: libc::c_float = 0 as libc::c_int as libc::c_float;
            let mut w: *mut libc::c_float = ((*((*net).layer).offset(i as isize)).weight)
                .offset((j * units) as isize);
            let mut o: *mut libc::c_float = (*((*net).layer).offset(i as isize)).output;
            k = 0 as libc::c_int;
            while k < units {
                let fresh7 = w;
                w = w.offset(1);
                let fresh8 = o;
                o = o.offset(1);
                A += *fresh7 * *fresh8;
                k += 1;
                k;
            }
            *((*((*net).layer).offset((i - 1 as libc::c_int) as isize)).output)
                .offset(j as isize) = sigmoid(A);
            j += 1;
            j;
        }
        i -= 1;
        i;
    }
}
pub unsafe extern "C" fn Ann2Tcl(mut net: *mut Ann) {
    let mut i: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    let mut k: libc::c_int = 0;
    printf(b"proc ann input {\n\0" as *const u8 as *const libc::c_char);
    printf(b"    set output {\0" as *const u8 as *const libc::c_char);
    i = 0 as libc::c_int;
    while i < (*((*net).layer).offset(0 as libc::c_int as isize)).units {
        printf(b"0 \0" as *const u8 as *const libc::c_char);
        i += 1;
        i;
    }
    printf(b"}\n\0" as *const u8 as *const libc::c_char);
    i = (*net).layers - 1 as libc::c_int;
    while i > 0 as libc::c_int {
        let mut nextunits: libc::c_int = (*((*net).layer)
            .offset((i - 1 as libc::c_int) as isize))
            .units;
        let mut units: libc::c_int = (*((*net).layer).offset(i as isize)).units;
        if i > 1 as libc::c_int {
            nextunits -= 1;
            nextunits;
        }
        j = 0 as libc::c_int;
        while j < nextunits {
            let mut W: libc::c_float = 0.;
            if i == 1 as libc::c_int {
                printf(b"    lset output %d \0" as *const u8 as *const libc::c_char, j);
            } else {
                printf(
                    b"    set O_%d_%d\0" as *const u8 as *const libc::c_char,
                    i - 1 as libc::c_int,
                    j,
                );
            }
            printf(b" [expr { \\\n\0" as *const u8 as *const libc::c_char);
            k = 0 as libc::c_int;
            while k < units {
                W = *((*((*net).layer).offset(i as isize)).weight)
                    .offset(
                        (j * (*((*net).layer).offset(i as isize)).units + k) as isize,
                    );
                if i > 1 as libc::c_int && k == units - 1 as libc::c_int {
                    printf(
                        b"        (%.9f)\0" as *const u8 as *const libc::c_char,
                        W as libc::c_double,
                    );
                } else if i == (*net).layers - 1 as libc::c_int {
                    printf(
                        b"        (%.9f*[lindex $input %d])\0" as *const u8
                            as *const libc::c_char,
                        W as libc::c_double,
                        k,
                    );
                } else {
                    printf(
                        b"        (%.9f*$O_%d_%d)\0" as *const u8 as *const libc::c_char,
                        W as libc::c_double,
                        i,
                        k,
                    );
                }
                if (k + 1 as libc::c_int) < units {
                    printf(b"+ \\\n\0" as *const u8 as *const libc::c_char);
                }
                k += 1;
                k;
            }
            printf(b"}]\n\0" as *const u8 as *const libc::c_char);
            if i == 1 as libc::c_int {
                printf(
                    b"    lset output %d [expr {1/(1+exp(-[lindex $output %d]))}]\n\0"
                        as *const u8 as *const libc::c_char,
                    j,
                    j,
                );
            } else {
                printf(
                    b"    lset O_%d_%d [expr {1/(1+exp(-$O_%d_%d))}]\n\0" as *const u8
                        as *const libc::c_char,
                    i - 1 as libc::c_int,
                    j,
                    i - 1 as libc::c_int,
                    j,
                );
            }
            j += 1;
            j;
        }
        i -= 1;
        i;
    }
    printf(b"    return $output\n\0" as *const u8 as *const libc::c_char);
    printf(b"}\n\0" as *const u8 as *const libc::c_char);
}
pub unsafe extern "C" fn AnnPrint(mut net: *mut Ann) {
    let mut i: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    let mut k: libc::c_int = 0;
    i = 0 as libc::c_int;
    while i < (*net).layers {
        let mut layertype: *mut libc::c_char = b"Hidden\0" as *const u8
            as *const libc::c_char as *mut libc::c_char;
        if i == 0 as libc::c_int {
            layertype = b"Output\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char;
        }
        if i == (*net).layers - 1 as libc::c_int {
            layertype = b"Input\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char;
        }
        printf(
            b"%s layer %d, units %d\n\0" as *const u8 as *const libc::c_char,
            layertype,
            i,
            (*((*net).layer).offset(i as isize)).units,
        );
        if i != 0 {
            let mut targets: libc::c_int = (*((*net).layer)
                .offset((i - 1 as libc::c_int) as isize))
                .units - (i - 1 as libc::c_int > 0 as libc::c_int) as libc::c_int;
            printf(b"\tW\0" as *const u8 as *const libc::c_char);
            j = 0 as libc::c_int;
            while j < (*((*net).layer).offset(i as isize)).units {
                printf(b"(\0" as *const u8 as *const libc::c_char);
                k = 0 as libc::c_int;
                while k < targets {
                    printf(
                        b"%f\0" as *const u8 as *const libc::c_char,
                        *((*((*net).layer).offset(i as isize)).weight)
                            .offset(
                                (k * (*((*net).layer).offset(i as isize)).units + j)
                                    as isize,
                            ) as libc::c_double,
                    );
                    if k != targets - 1 as libc::c_int {
                        printf(b" \0" as *const u8 as *const libc::c_char);
                    }
                    k += 1;
                    k;
                }
                printf(b") \0" as *const u8 as *const libc::c_char);
                j += 1;
                j;
            }
            printf(b"\n\0" as *const u8 as *const libc::c_char);
            printf(b"\tg\0" as *const u8 as *const libc::c_char);
            j = 0 as libc::c_int;
            while j < (*((*net).layer).offset(i as isize)).units {
                printf(b"[\0" as *const u8 as *const libc::c_char);
                k = 0 as libc::c_int;
                while k < targets {
                    printf(
                        b"%f\0" as *const u8 as *const libc::c_char,
                        *((*((*net).layer).offset(i as isize)).gradient)
                            .offset(
                                (k * (*((*net).layer).offset(i as isize)).units + j)
                                    as isize,
                            ) as libc::c_double,
                    );
                    if k != targets - 1 as libc::c_int {
                        printf(b" \0" as *const u8 as *const libc::c_char);
                    }
                    k += 1;
                    k;
                }
                printf(b"] \0" as *const u8 as *const libc::c_char);
                j += 1;
                j;
            }
            printf(b"\n\0" as *const u8 as *const libc::c_char);
            printf(b"\tG\0" as *const u8 as *const libc::c_char);
            j = 0 as libc::c_int;
            while j < (*((*net).layer).offset(i as isize)).units {
                printf(b"[\0" as *const u8 as *const libc::c_char);
                k = 0 as libc::c_int;
                while k < targets {
                    printf(
                        b"%f\0" as *const u8 as *const libc::c_char,
                        *((*((*net).layer).offset(i as isize)).sgradient)
                            .offset(
                                (k * (*((*net).layer).offset(i as isize)).units + j)
                                    as isize,
                            ) as libc::c_double,
                    );
                    if k != targets - 1 as libc::c_int {
                        printf(b" \0" as *const u8 as *const libc::c_char);
                    }
                    k += 1;
                    k;
                }
                printf(b"] \0" as *const u8 as *const libc::c_char);
                j += 1;
                j;
            }
            printf(b"\n\0" as *const u8 as *const libc::c_char);
            printf(b"\tP\0" as *const u8 as *const libc::c_char);
            j = 0 as libc::c_int;
            while j < (*((*net).layer).offset(i as isize)).units {
                printf(b"[\0" as *const u8 as *const libc::c_char);
                k = 0 as libc::c_int;
                while k < targets {
                    printf(
                        b"%f\0" as *const u8 as *const libc::c_char,
                        *((*((*net).layer).offset(i as isize)).pgradient)
                            .offset(
                                (k * (*((*net).layer).offset(i as isize)).units + j)
                                    as isize,
                            ) as libc::c_double,
                    );
                    if k != targets - 1 as libc::c_int {
                        printf(b" \0" as *const u8 as *const libc::c_char);
                    }
                    k += 1;
                    k;
                }
                printf(b"] \0" as *const u8 as *const libc::c_char);
                j += 1;
                j;
            }
            printf(b"\n\0" as *const u8 as *const libc::c_char);
            printf(b"\tD\0" as *const u8 as *const libc::c_char);
            j = 0 as libc::c_int;
            while j < (*((*net).layer).offset(i as isize)).units {
                printf(b"|\0" as *const u8 as *const libc::c_char);
                k = 0 as libc::c_int;
                while k < targets {
                    printf(
                        b"%f\0" as *const u8 as *const libc::c_char,
                        *((*((*net).layer).offset(i as isize)).delta)
                            .offset(
                                (k * (*((*net).layer).offset(i as isize)).units + j)
                                    as isize,
                            ) as libc::c_double,
                    );
                    if k != targets - 1 as libc::c_int {
                        printf(b" \0" as *const u8 as *const libc::c_char);
                    }
                    k += 1;
                    k;
                }
                printf(b"| \0" as *const u8 as *const libc::c_char);
                j += 1;
                j;
            }
            printf(b"\n\0" as *const u8 as *const libc::c_char);
        }
        j = 0 as libc::c_int;
        while j < (*((*net).layer).offset(i as isize)).units {
            printf(
                b"\tO: %f \0" as *const u8 as *const libc::c_char,
                *((*((*net).layer).offset(i as isize)).output).offset(j as isize)
                    as libc::c_double,
            );
            j += 1;
            j;
        }
        printf(b"\n\0" as *const u8 as *const libc::c_char);
        printf(b"\tE /\0" as *const u8 as *const libc::c_char);
        j = 0 as libc::c_int;
        while j < (*((*net).layer).offset(i as isize)).units {
            printf(
                b"%f \0" as *const u8 as *const libc::c_char,
                *((*((*net).layer).offset(i as isize)).error).offset(j as isize)
                    as libc::c_double,
            );
            j += 1;
            j;
        }
        printf(b"/\n\0" as *const u8 as *const libc::c_char);
        i += 1;
        i;
    }
}
pub unsafe extern "C" fn AnnGlobalError(
    mut net: *mut Ann,
    mut desired: *mut libc::c_float,
) -> libc::c_float {
    let mut e: libc::c_float = 0.;
    let mut t: libc::c_float = 0.;
    let mut i: libc::c_int = 0;
    let mut outputs: libc::c_int = (*((*net).layer).offset(0 as libc::c_int as isize))
        .units;
    e = 0 as libc::c_int as libc::c_float;
    i = 0 as libc::c_int;
    while i < outputs {
        t = *desired.offset(i as isize)
            - *((*((*net).layer).offset(0 as libc::c_int as isize)).output)
                .offset(i as isize);
        e += t * t;
        i += 1;
        i;
    }
    return (0.5f64 * e as libc::c_double) as libc::c_float;
}
pub unsafe extern "C" fn AnnSetInput(mut net: *mut Ann, mut input: *mut libc::c_float) {
    let mut i: libc::c_int = 0;
    let mut inputs: libc::c_int = (*((*net).layer)
        .offset(((*net).layers - 1 as libc::c_int) as isize))
        .units - 1 as libc::c_int;
    i = 0 as libc::c_int;
    while i < inputs {
        *((*((*net).layer).offset(((*net).layers - 1 as libc::c_int) as isize)).output)
            .offset(i as isize) = *input.offset(i as isize);
        i += 1;
        i;
    }
}
pub unsafe extern "C" fn AnnSimulateError(
    mut net: *mut Ann,
    mut input: *mut libc::c_float,
    mut desired: *mut libc::c_float,
) -> libc::c_float {
    AnnSetInput(net, input);
    AnnSimulate(net);
    return AnnGlobalError(net, desired);
}
pub unsafe extern "C" fn AnnCalculateOutputError(
    mut net: *mut Ann,
    mut desired: *mut libc::c_float,
) {
    let mut units: libc::c_int = (*((*net).layer).offset(0 as libc::c_int as isize))
        .units;
    let mut factor: libc::c_float = 2 as libc::c_int as libc::c_float
        / units as libc::c_float;
    let mut j: libc::c_int = 0 as libc::c_int;
    while j < units {
        *((*((*net).layer).offset(0 as libc::c_int as isize)).error)
            .offset(
                j as isize,
            ) = factor
            * (*((*((*net).layer).offset(0 as libc::c_int as isize)).output)
                .offset(j as isize) - *desired.offset(j as isize));
        j += 1;
        j;
    }
}
pub unsafe extern "C" fn AnnCalculateGradientsTrivial(
    mut net: *mut Ann,
    mut desired: *mut libc::c_float,
) {
    let mut j: libc::c_int = 0;
    let mut i: libc::c_int = 0;
    let mut layers: libc::c_int = (*net).layers;
    j = 1 as libc::c_int;
    while j < layers {
        let mut units: libc::c_int = (*((*net).layer).offset(j as isize)).units;
        let mut weights: libc::c_int = units
            * (*((*net).layer).offset((j - 1 as libc::c_int) as isize)).units;
        i = 0 as libc::c_int;
        while i < weights {
            let mut t: libc::c_float = 0.;
            let mut e1: libc::c_float = 0.;
            let mut e2: libc::c_float = 0.;
            AnnSimulate(net);
            e1 = AnnGlobalError(net, desired);
            t = *((*((*net).layer).offset(j as isize)).weight).offset(i as isize);
            let ref mut fresh9 = *((*((*net).layer).offset(j as isize)).weight)
                .offset(i as isize);
            *fresh9 = (*fresh9 as libc::c_double + 0.001f64) as libc::c_float;
            AnnSimulate(net);
            e2 = AnnGlobalError(net, desired);
            *((*((*net).layer).offset(j as isize)).weight).offset(i as isize) = t;
            *((*((*net).layer).offset(j as isize)).gradient)
                .offset(
                    i as isize,
                ) = ((e2 - e1) as libc::c_double / 0.001f64) as libc::c_float;
            i += 1;
            i;
        }
        j += 1;
        j;
    }
}
pub unsafe extern "C" fn AnnCalculateGradients(
    mut net: *mut Ann,
    mut desired: *mut libc::c_float,
) {
    let mut j: libc::c_int = 0;
    let mut layers: libc::c_int = (*net).layers - 1 as libc::c_int;
    AnnCalculateOutputError(net, desired);
    j = 0 as libc::c_int;
    while j < layers {
        let mut layer: *mut AnnLayer = &mut *((*net).layer).offset(j as isize)
            as *mut AnnLayer;
        let mut prev_layer: *mut AnnLayer = &mut *((*net).layer)
            .offset((j + 1 as libc::c_int) as isize) as *mut AnnLayer;
        let mut i: libc::c_int = 0;
        let mut units: libc::c_int = (*layer).units;
        let mut prevunits: libc::c_int = (*prev_layer).units;
        if j > 1 as libc::c_int {
            units -= 1;
            units;
        }
        i = 0 as libc::c_int;
        while i < prevunits {
            *((*prev_layer).error)
                .offset(i as isize) = 0 as libc::c_int as libc::c_float;
            i += 1;
            i;
        }
        i = 0 as libc::c_int;
        while i < units {
            let mut error_signal: libc::c_float = 0.;
            let mut ei: libc::c_float = 0.;
            let mut oi: libc::c_float = 0.;
            let mut derivative: libc::c_float = 0.;
            let mut k: libc::c_int = 0;
            ei = *((*layer).error).offset(i as isize);
            oi = *((*layer).output).offset(i as isize);
            derivative = oi * (1 as libc::c_int as libc::c_float - oi);
            error_signal = ei * derivative;
            let mut g: *mut libc::c_float = ((*prev_layer).gradient)
                .offset((i * prevunits) as isize);
            let mut w: *mut libc::c_float = ((*prev_layer).weight)
                .offset((i * prevunits) as isize);
            let mut o: *mut libc::c_float = (*prev_layer).output;
            let mut e: *mut libc::c_float = (*prev_layer).error;
            k = 0 as libc::c_int;
            while k < prevunits {
                let fresh10 = o;
                o = o.offset(1);
                let fresh11 = g;
                g = g.offset(1);
                *fresh11 = error_signal * *fresh10;
                k += 1;
                k;
            }
            k = 0 as libc::c_int;
            while k < prevunits {
                let fresh12 = w;
                w = w.offset(1);
                let fresh13 = e;
                e = e.offset(1);
                *fresh13 += error_signal * *fresh12;
                k += 1;
                k;
            }
            i += 1;
            i;
        }
        j += 1;
        j;
    }
}
pub unsafe extern "C" fn AnnSetDeltas(mut net: *mut Ann, mut val: libc::c_float) {
    let mut j: libc::c_int = 0;
    let mut layers: libc::c_int = (*net).layers;
    j = 1 as libc::c_int;
    while j < layers {
        let mut units: libc::c_int = (*((*net).layer).offset(j as isize)).units;
        let mut weights: libc::c_int = units
            * (*((*net).layer).offset((j - 1 as libc::c_int) as isize)).units;
        let mut i: libc::c_int = 0;
        i = 0 as libc::c_int;
        while i < weights {
            *((*((*net).layer).offset(j as isize)).delta).offset(i as isize) = val;
            i += 1;
            i;
        }
        j += 1;
        j;
    }
}
pub unsafe extern "C" fn AnnResetSgradient(mut net: *mut Ann) {
    let mut j: libc::c_int = 0;
    let mut layers: libc::c_int = (*net).layers;
    j = 1 as libc::c_int;
    while j < layers {
        let mut units: libc::c_int = (*((*net).layer).offset(j as isize)).units;
        let mut weights: libc::c_int = units
            * (*((*net).layer).offset((j - 1 as libc::c_int) as isize)).units;
        memset(
            (*((*net).layer).offset(j as isize)).sgradient as *mut libc::c_void,
            0 as libc::c_int,
            (::std::mem::size_of::<libc::c_float>() as libc::c_ulong)
                .wrapping_mul(weights as libc::c_ulong),
        );
        j += 1;
        j;
    }
}
pub unsafe extern "C" fn AnnSetRandomWeights(mut net: *mut Ann) {
    let mut i: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    let mut k: libc::c_int = 0;
    i = 1 as libc::c_int;
    while i < (*net).layers {
        k = 0 as libc::c_int;
        while k < (*((*net).layer).offset((i - 1 as libc::c_int) as isize)).units {
            j = 0 as libc::c_int;
            while j < (*((*net).layer).offset(i as isize)).units {
                *((*((*net).layer).offset(i as isize)).weight)
                    .offset(
                        (k * (*((*net).layer).offset(i as isize)).units + j) as isize,
                    ) = (-0.05f64
                    + 0.1f64
                        * (rand() as libc::c_double
                            / (2147483647 as libc::c_int as libc::c_double + 1.0f64)))
                    as libc::c_float;
                j += 1;
                j;
            }
            k += 1;
            k;
        }
        i += 1;
        i;
    }
}
pub unsafe extern "C" fn AnnScaleWeights(mut net: *mut Ann, mut factor: libc::c_float) {
    let mut j: libc::c_int = 0;
    let mut layers: libc::c_int = (*net).layers;
    j = 1 as libc::c_int;
    while j < layers {
        let mut units: libc::c_int = (*((*net).layer).offset(j as isize)).units;
        let mut weights: libc::c_int = units
            * (*((*net).layer).offset((j - 1 as libc::c_int) as isize)).units;
        let mut i: libc::c_int = 0;
        i = 0 as libc::c_int;
        while i < weights {
            *((*((*net).layer).offset(j as isize)).weight).offset(i as isize) *= factor;
            i += 1;
            i;
        }
        j += 1;
        j;
    }
}
pub unsafe extern "C" fn AnnUpdateSgradient(mut net: *mut Ann) {
    let mut j: libc::c_int = 0;
    let mut i: libc::c_int = 0;
    let mut layers: libc::c_int = (*net).layers;
    j = 1 as libc::c_int;
    while j < layers {
        let mut units: libc::c_int = (*((*net).layer).offset(j as isize)).units;
        let mut weights: libc::c_int = units
            * (*((*net).layer).offset((j - 1 as libc::c_int) as isize)).units;
        i = 0 as libc::c_int;
        while i < weights {
            *((*((*net).layer).offset(j as isize)).sgradient).offset(i as isize)
                += *((*((*net).layer).offset(j as isize)).gradient).offset(i as isize);
            i += 1;
            i;
        }
        j += 1;
        j;
    }
}
pub unsafe extern "C" fn sign(mut n: libc::c_float) -> libc::c_float {
    if n > 0 as libc::c_int as libc::c_float {
        return 1 as libc::c_int as libc::c_float;
    }
    if n < 0 as libc::c_int as libc::c_float {
        return -(1 as libc::c_int) as libc::c_float;
    }
    return 0 as libc::c_int as libc::c_float;
}
pub unsafe extern "C" fn AnnAdjustWeightsResilientBP(mut net: *mut Ann) {
    let mut j: libc::c_int = 0;
    let mut i: libc::c_int = 0;
    let mut layers: libc::c_int = (*net).layers;
    j = 1 as libc::c_int;
    while j < layers {
        let mut units: libc::c_int = (*((*net).layer).offset(j as isize)).units;
        let mut weights: libc::c_int = units
            * (*((*net).layer).offset((j - 1 as libc::c_int) as isize)).units
            - (j - 1 as libc::c_int > 0 as libc::c_int) as libc::c_int;
        i = 0 as libc::c_int;
        while i < weights {
            let mut t: libc::c_float = *((*((*net).layer).offset(j as isize)).pgradient)
                .offset(i as isize)
                * *((*((*net).layer).offset(j as isize)).sgradient).offset(i as isize);
            let mut delta: libc::c_float = *((*((*net).layer).offset(j as isize)).delta)
                .offset(i as isize);
            if t > 0 as libc::c_int as libc::c_float {
                delta = if delta * (*net).rprop_nplus < (*net).rprop_maxupdate {
                    delta * (*net).rprop_nplus
                } else {
                    (*net).rprop_maxupdate
                };
                let mut wdelta: libc::c_float = -sign(
                    *((*((*net).layer).offset(j as isize)).sgradient).offset(i as isize),
                ) * delta;
                *((*((*net).layer).offset(j as isize)).weight).offset(i as isize)
                    += wdelta;
                *((*((*net).layer).offset(j as isize)).delta).offset(i as isize) = delta;
                *((*((*net).layer).offset(j as isize)).pgradient)
                    .offset(
                        i as isize,
                    ) = *((*((*net).layer).offset(j as isize)).sgradient)
                    .offset(i as isize);
            } else if t < 0 as libc::c_int as libc::c_float {
                let mut past_wdelta: libc::c_float = -sign(
                    *((*((*net).layer).offset(j as isize)).pgradient).offset(i as isize),
                ) * delta;
                delta = if delta * (*net).rprop_nminus > (*net).rprop_minupdate {
                    delta * (*net).rprop_nminus
                } else {
                    (*net).rprop_minupdate
                };
                *((*((*net).layer).offset(j as isize)).weight).offset(i as isize)
                    -= past_wdelta;
                *((*((*net).layer).offset(j as isize)).delta).offset(i as isize) = delta;
                *((*((*net).layer).offset(j as isize)).pgradient)
                    .offset(i as isize) = 0 as libc::c_int as libc::c_float;
            } else {
                let mut wdelta_0: libc::c_float = -sign(
                    *((*((*net).layer).offset(j as isize)).sgradient).offset(i as isize),
                ) * delta;
                *((*((*net).layer).offset(j as isize)).weight).offset(i as isize)
                    += wdelta_0;
                *((*((*net).layer).offset(j as isize)).pgradient)
                    .offset(
                        i as isize,
                    ) = *((*((*net).layer).offset(j as isize)).sgradient)
                    .offset(i as isize);
            }
            i += 1;
            i;
        }
        j += 1;
        j;
    }
}
pub unsafe extern "C" fn AnnResilientBPEpoch(
    mut net: *mut Ann,
    mut input: *mut libc::c_float,
    mut desired: *mut libc::c_float,
    mut setlen: libc::c_int,
) -> libc::c_float {
    let mut error: libc::c_float = 0 as libc::c_int as libc::c_float;
    let mut j: libc::c_int = 0;
    let mut inputs: libc::c_int = (*((*net).layer)
        .offset(((*net).layers - 1 as libc::c_int) as isize))
        .units - 1 as libc::c_int;
    let mut outputs: libc::c_int = (*((*net).layer).offset(0 as libc::c_int as isize))
        .units;
    AnnResetSgradient(net);
    j = 0 as libc::c_int;
    while j < setlen {
        error += AnnSimulateError(net, input, desired);
        AnnCalculateGradients(net, desired);
        AnnUpdateSgradient(net);
        input = input.offset(inputs as isize);
        desired = desired.offset(outputs as isize);
        j += 1;
        j;
    }
    AnnAdjustWeightsResilientBP(net);
    return error / setlen as libc::c_float;
}
pub unsafe extern "C" fn AnnUpdateDeltasGD(mut net: *mut Ann) {
    let mut j: libc::c_int = 0;
    let mut i: libc::c_int = 0;
    let mut layers: libc::c_int = (*net).layers;
    j = 1 as libc::c_int;
    while j < layers {
        let mut units: libc::c_int = (*((*net).layer).offset(j as isize)).units;
        let mut weights: libc::c_int = units
            * (*((*net).layer).offset((j - 1 as libc::c_int) as isize)).units;
        i = 0 as libc::c_int;
        while i < weights {
            *((*((*net).layer).offset(j as isize)).delta).offset(i as isize)
                += *((*((*net).layer).offset(j as isize)).gradient).offset(i as isize);
            i += 1;
            i;
        }
        j += 1;
        j;
    }
}
pub unsafe extern "C" fn AnnAdjustWeights(mut net: *mut Ann, mut setlen: libc::c_int) {
    let mut j: libc::c_int = 0;
    let mut i: libc::c_int = 0;
    let mut layers: libc::c_int = (*net).layers;
    j = 1 as libc::c_int;
    while j < layers {
        let mut units: libc::c_int = (*((*net).layer).offset(j as isize)).units;
        let mut weights: libc::c_int = units
            * (*((*net).layer).offset((j - 1 as libc::c_int) as isize)).units;
        i = 0 as libc::c_int;
        while i < weights {
            *((*((*net).layer).offset(j as isize)).weight).offset(i as isize)
                -= (*net).learn_rate / setlen as libc::c_float
                    * *((*((*net).layer).offset(j as isize)).delta).offset(i as isize);
            i += 1;
            i;
        }
        j += 1;
        j;
    }
}
pub unsafe extern "C" fn AnnGDEpoch(
    mut net: *mut Ann,
    mut input: *mut libc::c_float,
    mut desidered: *mut libc::c_float,
    mut setlen: libc::c_int,
) -> libc::c_float {
    let mut error: libc::c_float = 0 as libc::c_int as libc::c_float;
    let mut j: libc::c_int = 0;
    let mut inputs: libc::c_int = (*((*net).layer)
        .offset(((*net).layers - 1 as libc::c_int) as isize))
        .units - 1 as libc::c_int;
    let mut outputs: libc::c_int = (*((*net).layer).offset(0 as libc::c_int as isize))
        .units;
    j = 0 as libc::c_int;
    while j < setlen {
        AnnSetDeltas(net, 0 as libc::c_int as libc::c_float);
        error += AnnSimulateError(net, input, desidered);
        AnnCalculateGradients(net, desidered);
        AnnUpdateDeltasGD(net);
        input = input.offset(inputs as isize);
        desidered = desidered.offset(outputs as isize);
        AnnAdjustWeights(net, setlen);
        j += 1;
        j;
    }
    return error / setlen as libc::c_float;
}
pub unsafe extern "C" fn AnnTestClassError(
    mut net: *mut Ann,
    mut desired: *mut libc::c_float,
) -> libc::c_int {
    let mut i: libc::c_int = 0;
    let mut outputs: libc::c_int = (*((*net).layer).offset(0 as libc::c_int as isize))
        .units;
    let mut classid: libc::c_int = 0;
    let mut outid: libc::c_int = 0;
    let mut max: libc::c_float = 0 as libc::c_int as libc::c_float;
    classid = 0 as libc::c_int;
    i = 0 as libc::c_int;
    while i < outputs {
        if *desired.offset(i as isize) == 1 as libc::c_int as libc::c_float {
            break;
        }
        i += 1;
        i;
    }
    classid = i;
    max = *((*((*net).layer).offset(0 as libc::c_int as isize)).output)
        .offset(0 as libc::c_int as isize);
    outid = 0 as libc::c_int;
    i = 1 as libc::c_int;
    while i < outputs {
        let mut o: libc::c_float = *((*((*net).layer).offset(0 as libc::c_int as isize))
            .output)
            .offset(i as isize);
        if o > max {
            outid = i;
            max = o;
        }
        i += 1;
        i;
    }
    return (outid != classid) as libc::c_int;
}
pub unsafe extern "C" fn AnnTestError(
    mut net: *mut Ann,
    mut input: *mut libc::c_float,
    mut desired: *mut libc::c_float,
    mut setlen: libc::c_int,
    mut avgerr: *mut libc::c_float,
    mut classerr: *mut libc::c_float,
) {
    let mut error: libc::c_float = 0 as libc::c_int as libc::c_float;
    let mut j: libc::c_int = 0;
    let mut inputs: libc::c_int = (*((*net).layer)
        .offset(((*net).layers - 1 as libc::c_int) as isize))
        .units - 1 as libc::c_int;
    let mut outputs: libc::c_int = (*((*net).layer).offset(0 as libc::c_int as isize))
        .units;
    let mut class_errors: libc::c_int = 0 as libc::c_int;
    j = 0 as libc::c_int;
    while j < setlen {
        error += AnnSimulateError(net, input, desired);
        if !classerr.is_null() {
            class_errors += AnnTestClassError(net, desired);
        }
        input = input.offset(inputs as isize);
        desired = desired.offset(outputs as isize);
        j += 1;
        j;
    }
    if !avgerr.is_null() {
        *avgerr = error / setlen as libc::c_float;
    }
    if !classerr.is_null() {
        *classerr = class_errors as libc::c_float * 100 as libc::c_int as libc::c_float
            / setlen as libc::c_float;
    }
}
pub unsafe extern "C" fn AnnTrainWithAlgoFunc(
    mut net: *mut Ann,
    mut input: *mut libc::c_float,
    mut desired: *mut libc::c_float,
    mut maxerr: libc::c_float,
    mut maxepochs: libc::c_int,
    mut setlen: libc::c_int,
    mut algo_func: AnnTrainAlgoFunc,
) -> libc::c_float {
    let mut i: libc::c_int = 0 as libc::c_int;
    let mut e: libc::c_float = maxerr + 1 as libc::c_int as libc::c_float;
    loop {
        let fresh14 = i;
        i = i + 1;
        if !(fresh14 < maxepochs && e >= maxerr) {
            break;
        }
        e = (Some(algo_func.unwrap())).unwrap()(net, input, desired, setlen);
    }
    return e;
}
pub unsafe extern "C" fn AnnTrain(
    mut net: *mut Ann,
    mut input: *mut libc::c_float,
    mut desired: *mut libc::c_float,
    mut maxerr: libc::c_float,
    mut maxepochs: libc::c_int,
    mut setlen: libc::c_int,
    mut algo: libc::c_int,
) -> libc::c_float {
    let mut algo_func: AnnTrainAlgoFunc = None;
    if algo == 0 as libc::c_int {
        algo_func = Some(
            AnnResilientBPEpoch
                as unsafe extern "C" fn(
                    *mut Ann,
                    *mut libc::c_float,
                    *mut libc::c_float,
                    libc::c_int,
                ) -> libc::c_float,
        );
    } else if algo == 1 as libc::c_int {
        algo_func = Some(
            AnnGDEpoch
                as unsafe extern "C" fn(
                    *mut Ann,
                    *mut libc::c_float,
                    *mut libc::c_float,
                    libc::c_int,
                ) -> libc::c_float,
        );
    } else {
        return -(1 as libc::c_int) as libc::c_float
    }
    return AnnTrainWithAlgoFunc(
        net,
        input,
        desired,
        maxerr,
        maxepochs,
        setlen,
        algo_func,
    );
}
