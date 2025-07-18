use ::libc;
extern "C" {
    pub type _IO_wide_data;
    pub type _IO_codecvt;
    pub type _IO_marker;
    fn acos(_: libc::c_double) -> libc::c_double;
    fn cos(_: libc::c_double) -> libc::c_double;
    fn sin(_: libc::c_double) -> libc::c_double;
    fn memcpy(
        _: *mut libc::c_void,
        _: *const libc::c_void,
        _: libc::c_ulong,
    ) -> *mut libc::c_void;
    static mut stdout: *mut FILE;
    fn fflush(__stream: *mut FILE) -> libc::c_int;
    fn printf(_: *const libc::c_char, _: ...) -> libc::c_int;
    fn gsl_rng_env_setup() -> *const gsl_rng_type;
    fn gsl_rng_alloc(T: *const gsl_rng_type) -> *mut gsl_rng;
    fn gsl_siman_solve(
        r: *const gsl_rng,
        x0_p: *mut libc::c_void,
        Ef: gsl_siman_Efunc_t,
        take_step: gsl_siman_step_t,
        distance: gsl_siman_metric_t,
        print_position: gsl_siman_print_t,
        copyfunc: gsl_siman_copy_t,
        copy_constructor: gsl_siman_copy_construct_t,
        destructor: gsl_siman_destroy_t,
        element_size: size_t,
        params_0: gsl_siman_params_t,
    );
    fn gsl_ieee_env_setup();
}
pub type __off_t = libc::c_long;
pub type __off64_t = libc::c_long;
pub type size_t = libc::c_ulong;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _IO_FILE {
    pub _flags: libc::c_int,
    pub _IO_read_ptr: *mut libc::c_char,
    pub _IO_read_end: *mut libc::c_char,
    pub _IO_read_base: *mut libc::c_char,
    pub _IO_write_base: *mut libc::c_char,
    pub _IO_write_ptr: *mut libc::c_char,
    pub _IO_write_end: *mut libc::c_char,
    pub _IO_buf_base: *mut libc::c_char,
    pub _IO_buf_end: *mut libc::c_char,
    pub _IO_save_base: *mut libc::c_char,
    pub _IO_backup_base: *mut libc::c_char,
    pub _IO_save_end: *mut libc::c_char,
    pub _markers: *mut _IO_marker,
    pub _chain: *mut _IO_FILE,
    pub _fileno: libc::c_int,
    pub _flags2: libc::c_int,
    pub _old_offset: __off_t,
    pub _cur_column: libc::c_ushort,
    pub _vtable_offset: libc::c_schar,
    pub _shortbuf: [libc::c_char; 1],
    pub _lock: *mut libc::c_void,
    pub _offset: __off64_t,
    pub _codecvt: *mut _IO_codecvt,
    pub _wide_data: *mut _IO_wide_data,
    pub _freeres_list: *mut _IO_FILE,
    pub _freeres_buf: *mut libc::c_void,
    pub __pad5: size_t,
    pub _mode: libc::c_int,
    pub _unused2: [libc::c_char; 20],
}
pub type _IO_lock_t = ();
pub type FILE = _IO_FILE;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct gsl_rng_type {
    pub name: *const libc::c_char,
    pub max: libc::c_ulong,
    pub min: libc::c_ulong,
    pub size: size_t,
    pub set: Option::<unsafe extern "C" fn(*mut libc::c_void, libc::c_ulong) -> ()>,
    pub get: Option::<unsafe extern "C" fn(*mut libc::c_void) -> libc::c_ulong>,
    pub get_double: Option::<unsafe extern "C" fn(*mut libc::c_void) -> libc::c_double>,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct gsl_rng {
    pub type_0: *const gsl_rng_type,
    pub state: *mut libc::c_void,
}
pub type gsl_siman_Efunc_t = Option::<
    unsafe extern "C" fn(*mut libc::c_void) -> libc::c_double,
>;
pub type gsl_siman_step_t = Option::<
    unsafe extern "C" fn(*const gsl_rng, *mut libc::c_void, libc::c_double) -> (),
>;
pub type gsl_siman_metric_t = Option::<
    unsafe extern "C" fn(*mut libc::c_void, *mut libc::c_void) -> libc::c_double,
>;
pub type gsl_siman_print_t = Option::<unsafe extern "C" fn(*mut libc::c_void) -> ()>;
pub type gsl_siman_copy_t = Option::<
    unsafe extern "C" fn(*mut libc::c_void, *mut libc::c_void) -> (),
>;
pub type gsl_siman_copy_construct_t = Option::<
    unsafe extern "C" fn(*mut libc::c_void) -> *mut libc::c_void,
>;
pub type gsl_siman_destroy_t = Option::<unsafe extern "C" fn(*mut libc::c_void) -> ()>;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct gsl_siman_params_t {
    pub n_tries: libc::c_int,
    pub iters_fixed_T: libc::c_int,
    pub step_size: libc::c_double,
    pub k: libc::c_double,
    pub t_initial: libc::c_double,
    pub mu_t: libc::c_double,
    pub t_min: libc::c_double,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct s_tsp_city {
    pub name: *const libc::c_char,
    pub lat: libc::c_double,
    pub longitude: libc::c_double,
}
pub type Stsp_city = s_tsp_city;
#[inline]
unsafe extern "C" fn gsl_rng_get(mut r: *const gsl_rng) -> libc::c_ulong {
    return ((*(*r).type_0).get).unwrap()((*r).state);
}
pub static mut params: gsl_siman_params_t = {
    let mut init = gsl_siman_params_t {
        n_tries: 200 as libc::c_int,
        iters_fixed_T: 2000 as libc::c_int,
        step_size: 1.0f64,
        k: 1.0f64,
        t_initial: 5000.0f64,
        mu_t: 1.002f64,
        t_min: 5.0e-1f64,
    };
    init
};
pub static mut cities: [Stsp_city; 12] = [
    {
        let mut init = s_tsp_city {
            name: b"Santa Fe\0" as *const u8 as *const libc::c_char,
            lat: 35.68f64,
            longitude: 105.95f64,
        };
        init
    },
    {
        let mut init = s_tsp_city {
            name: b"Phoenix\0" as *const u8 as *const libc::c_char,
            lat: 33.54f64,
            longitude: 112.07f64,
        };
        init
    },
    {
        let mut init = s_tsp_city {
            name: b"Albuquerque\0" as *const u8 as *const libc::c_char,
            lat: 35.12f64,
            longitude: 106.62f64,
        };
        init
    },
    {
        let mut init = s_tsp_city {
            name: b"Clovis\0" as *const u8 as *const libc::c_char,
            lat: 34.41f64,
            longitude: 103.20f64,
        };
        init
    },
    {
        let mut init = s_tsp_city {
            name: b"Durango\0" as *const u8 as *const libc::c_char,
            lat: 37.29f64,
            longitude: 107.87f64,
        };
        init
    },
    {
        let mut init = s_tsp_city {
            name: b"Dallas\0" as *const u8 as *const libc::c_char,
            lat: 32.79f64,
            longitude: 96.77f64,
        };
        init
    },
    {
        let mut init = s_tsp_city {
            name: b"Tesuque\0" as *const u8 as *const libc::c_char,
            lat: 35.77f64,
            longitude: 105.92f64,
        };
        init
    },
    {
        let mut init = s_tsp_city {
            name: b"Grants\0" as *const u8 as *const libc::c_char,
            lat: 35.15f64,
            longitude: 107.84f64,
        };
        init
    },
    {
        let mut init = s_tsp_city {
            name: b"Los Alamos\0" as *const u8 as *const libc::c_char,
            lat: 35.89f64,
            longitude: 106.28f64,
        };
        init
    },
    {
        let mut init = s_tsp_city {
            name: b"Las Cruces\0" as *const u8 as *const libc::c_char,
            lat: 32.34f64,
            longitude: 106.76f64,
        };
        init
    },
    {
        let mut init = s_tsp_city {
            name: b"Cortez\0" as *const u8 as *const libc::c_char,
            lat: 37.35f64,
            longitude: 108.58f64,
        };
        init
    },
    {
        let mut init = s_tsp_city {
            name: b"Gallup\0" as *const u8 as *const libc::c_char,
            lat: 35.52f64,
            longitude: 108.74f64,
        };
        init
    },
];
pub static mut distance_matrix: [[libc::c_double; 12]; 12] = [[0.; 12]; 12];
pub unsafe extern "C" fn city_distance(
    mut c1: Stsp_city,
    mut c2: Stsp_city,
) -> libc::c_double {
    let earth_radius: libc::c_double = 6375.000f64;
    let mut sla1: libc::c_double = sin(
        c1.lat * 3.14159265358979323846f64 / 180 as libc::c_int as libc::c_double,
    );
    let mut cla1: libc::c_double = cos(
        c1.lat * 3.14159265358979323846f64 / 180 as libc::c_int as libc::c_double,
    );
    let mut slo1: libc::c_double = sin(
        c1.longitude * 3.14159265358979323846f64 / 180 as libc::c_int as libc::c_double,
    );
    let mut clo1: libc::c_double = cos(
        c1.longitude * 3.14159265358979323846f64 / 180 as libc::c_int as libc::c_double,
    );
    let mut sla2: libc::c_double = sin(
        c2.lat * 3.14159265358979323846f64 / 180 as libc::c_int as libc::c_double,
    );
    let mut cla2: libc::c_double = cos(
        c2.lat * 3.14159265358979323846f64 / 180 as libc::c_int as libc::c_double,
    );
    let mut slo2: libc::c_double = sin(
        c2.longitude * 3.14159265358979323846f64 / 180 as libc::c_int as libc::c_double,
    );
    let mut clo2: libc::c_double = cos(
        c2.longitude * 3.14159265358979323846f64 / 180 as libc::c_int as libc::c_double,
    );
    let mut x1: libc::c_double = cla1 * clo1;
    let mut x2: libc::c_double = cla2 * clo2;
    let mut y1: libc::c_double = cla1 * slo1;
    let mut y2: libc::c_double = cla2 * slo2;
    let mut z1: libc::c_double = sla1;
    let mut z2: libc::c_double = sla2;
    let mut dot_product: libc::c_double = x1 * x2 + y1 * y2 + z1 * z2;
    let mut angle: libc::c_double = acos(dot_product);
    return angle * earth_radius;
}
pub unsafe extern "C" fn Etsp(mut xp: *mut libc::c_void) -> libc::c_double {
    let mut route: *mut libc::c_int = xp as *mut libc::c_int;
    let mut E: libc::c_double = 0 as libc::c_int as libc::c_double;
    let mut i: libc::c_uint = 0;
    i = 0 as libc::c_int as libc::c_uint;
    while (i as libc::c_ulong)
        < (::std::mem::size_of::<[Stsp_city; 12]>() as libc::c_ulong)
            .wrapping_div(::std::mem::size_of::<Stsp_city>() as libc::c_ulong)
    {
        E
            += distance_matrix[*route.offset(i as isize)
                as usize][*route
                .offset(
                    (i.wrapping_add(1 as libc::c_int as libc::c_uint) as libc::c_ulong)
                        .wrapping_rem(
                            (::std::mem::size_of::<[Stsp_city; 12]>() as libc::c_ulong)
                                .wrapping_div(
                                    ::std::mem::size_of::<Stsp_city>() as libc::c_ulong,
                                ),
                        ) as isize,
                ) as usize];
        i = i.wrapping_add(1);
        i;
    }
    return E;
}
pub unsafe extern "C" fn Mtsp(
    mut xp: *mut libc::c_void,
    mut yp: *mut libc::c_void,
) -> libc::c_double {
    let mut route1: *mut libc::c_int = xp as *mut libc::c_int;
    let mut route2: *mut libc::c_int = yp as *mut libc::c_int;
    let mut distance: libc::c_double = 0 as libc::c_int as libc::c_double;
    let mut i: libc::c_uint = 0;
    i = 0 as libc::c_int as libc::c_uint;
    while (i as libc::c_ulong)
        < (::std::mem::size_of::<[Stsp_city; 12]>() as libc::c_ulong)
            .wrapping_div(::std::mem::size_of::<Stsp_city>() as libc::c_ulong)
    {
        distance
            += (if *route1.offset(i as isize) == *route2.offset(i as isize) {
                0 as libc::c_int
            } else {
                1 as libc::c_int
            }) as libc::c_double;
        i = i.wrapping_add(1);
        i;
    }
    return distance;
}
pub unsafe extern "C" fn Stsp(
    mut r: *const gsl_rng,
    mut xp: *mut libc::c_void,
    mut step_size: libc::c_double,
) {
    let mut x1: libc::c_int = 0;
    let mut x2: libc::c_int = 0;
    let mut dummy: libc::c_int = 0;
    let mut route: *mut libc::c_int = xp as *mut libc::c_int;
    step_size = 0 as libc::c_int as libc::c_double;
    x1 = (gsl_rng_get(r))
        .wrapping_rem(
            (::std::mem::size_of::<[Stsp_city; 12]>() as libc::c_ulong)
                .wrapping_div(::std::mem::size_of::<Stsp_city>() as libc::c_ulong)
                .wrapping_sub(1 as libc::c_int as libc::c_ulong),
        )
        .wrapping_add(1 as libc::c_int as libc::c_ulong) as libc::c_int;
    loop {
        x2 = (gsl_rng_get(r))
            .wrapping_rem(
                (::std::mem::size_of::<[Stsp_city; 12]>() as libc::c_ulong)
                    .wrapping_div(::std::mem::size_of::<Stsp_city>() as libc::c_ulong)
                    .wrapping_sub(1 as libc::c_int as libc::c_ulong),
            )
            .wrapping_add(1 as libc::c_int as libc::c_ulong) as libc::c_int;
        if !(x2 == x1) {
            break;
        }
    }
    dummy = *route.offset(x1 as isize);
    *route.offset(x1 as isize) = *route.offset(x2 as isize);
    *route.offset(x2 as isize) = dummy;
}
pub unsafe extern "C" fn Ptsp(mut xp: *mut libc::c_void) {
    let mut i: libc::c_uint = 0;
    let mut route: *mut libc::c_int = xp as *mut libc::c_int;
    printf(b"  [\0" as *const u8 as *const libc::c_char);
    i = 0 as libc::c_int as libc::c_uint;
    while (i as libc::c_ulong)
        < (::std::mem::size_of::<[Stsp_city; 12]>() as libc::c_ulong)
            .wrapping_div(::std::mem::size_of::<Stsp_city>() as libc::c_ulong)
    {
        printf(b" %d \0" as *const u8 as *const libc::c_char, *route.offset(i as isize));
        i = i.wrapping_add(1);
        i;
    }
    printf(b"]  \0" as *const u8 as *const libc::c_char);
}
unsafe fn main_0() -> libc::c_int {
    let mut x_initial: [libc::c_int; 12] = [0; 12];
    let mut i: libc::c_uint = 0;
    let mut r: *const gsl_rng = gsl_rng_alloc(gsl_rng_env_setup());
    gsl_ieee_env_setup();
    prepare_distance_matrix();
    printf(b"# initial order of cities:\n\0" as *const u8 as *const libc::c_char);
    i = 0 as libc::c_int as libc::c_uint;
    while (i as libc::c_ulong)
        < (::std::mem::size_of::<[Stsp_city; 12]>() as libc::c_ulong)
            .wrapping_div(::std::mem::size_of::<Stsp_city>() as libc::c_ulong)
    {
        printf(
            b"# \"%s\"\n\0" as *const u8 as *const libc::c_char,
            cities[i as usize].name,
        );
        x_initial[i as usize] = i as libc::c_int;
        i = i.wrapping_add(1);
        i;
    }
    printf(b"# distance matrix is:\n\0" as *const u8 as *const libc::c_char);
    print_distance_matrix();
    printf(
        b"# initial coordinates of cities (longitude and latitude)\n\0" as *const u8
            as *const libc::c_char,
    );
    i = 0 as libc::c_int as libc::c_uint;
    while (i as libc::c_ulong)
        < (::std::mem::size_of::<[Stsp_city; 12]>() as libc::c_ulong)
            .wrapping_div(::std::mem::size_of::<Stsp_city>() as libc::c_ulong)
            .wrapping_add(1 as libc::c_int as libc::c_ulong)
    {
        printf(
            b"###initial_city_coord: %g %g \"%s\"\n\0" as *const u8
                as *const libc::c_char,
            -cities[x_initial[(i as libc::c_ulong)
                    .wrapping_rem(
                        (::std::mem::size_of::<[Stsp_city; 12]>() as libc::c_ulong)
                            .wrapping_div(
                                ::std::mem::size_of::<Stsp_city>() as libc::c_ulong,
                            ),
                    ) as usize] as usize]
                .longitude,
            cities[x_initial[(i as libc::c_ulong)
                    .wrapping_rem(
                        (::std::mem::size_of::<[Stsp_city; 12]>() as libc::c_ulong)
                            .wrapping_div(
                                ::std::mem::size_of::<Stsp_city>() as libc::c_ulong,
                            ),
                    ) as usize] as usize]
                .lat,
            cities[x_initial[(i as libc::c_ulong)
                    .wrapping_rem(
                        (::std::mem::size_of::<[Stsp_city; 12]>() as libc::c_ulong)
                            .wrapping_div(
                                ::std::mem::size_of::<Stsp_city>() as libc::c_ulong,
                            ),
                    ) as usize] as usize]
                .name,
        );
        i = i.wrapping_add(1);
        i;
    }
    gsl_siman_solve(
        r,
        x_initial.as_mut_ptr() as *mut libc::c_void,
        Some(Etsp as unsafe extern "C" fn(*mut libc::c_void) -> libc::c_double),
        Some(
            Stsp
                as unsafe extern "C" fn(
                    *const gsl_rng,
                    *mut libc::c_void,
                    libc::c_double,
                ) -> (),
        ),
        Some(
            Mtsp
                as unsafe extern "C" fn(
                    *mut libc::c_void,
                    *mut libc::c_void,
                ) -> libc::c_double,
        ),
        Some(Ptsp as unsafe extern "C" fn(*mut libc::c_void) -> ()),
        None,
        None,
        None,
        (::std::mem::size_of::<[Stsp_city; 12]>() as libc::c_ulong)
            .wrapping_div(::std::mem::size_of::<Stsp_city>() as libc::c_ulong)
            .wrapping_mul(::std::mem::size_of::<libc::c_int>() as libc::c_ulong),
        params,
    );
    printf(b"# final order of cities:\n\0" as *const u8 as *const libc::c_char);
    i = 0 as libc::c_int as libc::c_uint;
    while (i as libc::c_ulong)
        < (::std::mem::size_of::<[Stsp_city; 12]>() as libc::c_ulong)
            .wrapping_div(::std::mem::size_of::<Stsp_city>() as libc::c_ulong)
    {
        printf(
            b"# \"%s\"\n\0" as *const u8 as *const libc::c_char,
            cities[x_initial[i as usize] as usize].name,
        );
        i = i.wrapping_add(1);
        i;
    }
    printf(
        b"# final coordinates of cities (longitude and latitude)\n\0" as *const u8
            as *const libc::c_char,
    );
    i = 0 as libc::c_int as libc::c_uint;
    while (i as libc::c_ulong)
        < (::std::mem::size_of::<[Stsp_city; 12]>() as libc::c_ulong)
            .wrapping_div(::std::mem::size_of::<Stsp_city>() as libc::c_ulong)
            .wrapping_add(1 as libc::c_int as libc::c_ulong)
    {
        printf(
            b"###final_city_coord: %g %g %s\n\0" as *const u8 as *const libc::c_char,
            -cities[x_initial[(i as libc::c_ulong)
                    .wrapping_rem(
                        (::std::mem::size_of::<[Stsp_city; 12]>() as libc::c_ulong)
                            .wrapping_div(
                                ::std::mem::size_of::<Stsp_city>() as libc::c_ulong,
                            ),
                    ) as usize] as usize]
                .longitude,
            cities[x_initial[(i as libc::c_ulong)
                    .wrapping_rem(
                        (::std::mem::size_of::<[Stsp_city; 12]>() as libc::c_ulong)
                            .wrapping_div(
                                ::std::mem::size_of::<Stsp_city>() as libc::c_ulong,
                            ),
                    ) as usize] as usize]
                .lat,
            cities[x_initial[(i as libc::c_ulong)
                    .wrapping_rem(
                        (::std::mem::size_of::<[Stsp_city; 12]>() as libc::c_ulong)
                            .wrapping_div(
                                ::std::mem::size_of::<Stsp_city>() as libc::c_ulong,
                            ),
                    ) as usize] as usize]
                .name,
        );
        i = i.wrapping_add(1);
        i;
    }
    printf(b"# \0" as *const u8 as *const libc::c_char);
    fflush(stdout);
    fflush(stdout);
    return 0 as libc::c_int;
}
pub unsafe extern "C" fn prepare_distance_matrix() {
    let mut i: libc::c_uint = 0;
    let mut j: libc::c_uint = 0;
    let mut dist: libc::c_double = 0.;
    i = 0 as libc::c_int as libc::c_uint;
    while (i as libc::c_ulong)
        < (::std::mem::size_of::<[Stsp_city; 12]>() as libc::c_ulong)
            .wrapping_div(::std::mem::size_of::<Stsp_city>() as libc::c_ulong)
    {
        j = 0 as libc::c_int as libc::c_uint;
        while (j as libc::c_ulong)
            < (::std::mem::size_of::<[Stsp_city; 12]>() as libc::c_ulong)
                .wrapping_div(::std::mem::size_of::<Stsp_city>() as libc::c_ulong)
        {
            if i == j {
                dist = 0 as libc::c_int as libc::c_double;
            } else {
                dist = city_distance(cities[i as usize], cities[j as usize]);
            }
            distance_matrix[i as usize][j as usize] = dist;
            j = j.wrapping_add(1);
            j;
        }
        i = i.wrapping_add(1);
        i;
    }
}
pub unsafe extern "C" fn print_distance_matrix() {
    let mut i: libc::c_uint = 0;
    let mut j: libc::c_uint = 0;
    i = 0 as libc::c_int as libc::c_uint;
    while (i as libc::c_ulong)
        < (::std::mem::size_of::<[Stsp_city; 12]>() as libc::c_ulong)
            .wrapping_div(::std::mem::size_of::<Stsp_city>() as libc::c_ulong)
    {
        printf(b"# \0" as *const u8 as *const libc::c_char);
        j = 0 as libc::c_int as libc::c_uint;
        while (j as libc::c_ulong)
            < (::std::mem::size_of::<[Stsp_city; 12]>() as libc::c_ulong)
                .wrapping_div(::std::mem::size_of::<Stsp_city>() as libc::c_ulong)
        {
            printf(
                b"%15.8f   \0" as *const u8 as *const libc::c_char,
                distance_matrix[i as usize][j as usize],
            );
            j = j.wrapping_add(1);
            j;
        }
        printf(b"\n\0" as *const u8 as *const libc::c_char);
        i = i.wrapping_add(1);
        i;
    }
}
static mut best_E: libc::c_double = 1.0e100f64;
static mut second_E: libc::c_double = 1.0e100f64;
static mut third_E: libc::c_double = 1.0e100f64;
static mut best_route: [libc::c_int; 12] = [0; 12];
static mut second_route: [libc::c_int; 12] = [0; 12];
static mut third_route: [libc::c_int; 12] = [0; 12];
pub unsafe extern "C" fn exhaustive_search() {
    static mut initial_route: [libc::c_int; 12] = [
        0 as libc::c_int,
        1 as libc::c_int,
        2 as libc::c_int,
        3 as libc::c_int,
        4 as libc::c_int,
        5 as libc::c_int,
        6 as libc::c_int,
        7 as libc::c_int,
        8 as libc::c_int,
        9 as libc::c_int,
        10 as libc::c_int,
        11 as libc::c_int,
    ];
    printf(b"\n# \0" as *const u8 as *const libc::c_char);
    fflush(stdout);
    fflush(stdout);
    do_all_perms(initial_route.as_mut_ptr(), 1 as libc::c_int);
    printf(b"\n# \0" as *const u8 as *const libc::c_char);
    fflush(stdout);
    fflush(stdout);
    printf(b"# exhaustive best route: \0" as *const u8 as *const libc::c_char);
    Ptsp(best_route.as_mut_ptr() as *mut libc::c_void);
    printf(b"\n# its energy is: %g\n\0" as *const u8 as *const libc::c_char, best_E);
    printf(b"# exhaustive second_best route: \0" as *const u8 as *const libc::c_char);
    Ptsp(second_route.as_mut_ptr() as *mut libc::c_void);
    printf(b"\n# its energy is: %g\n\0" as *const u8 as *const libc::c_char, second_E);
    printf(b"# exhaustive third_best route: \0" as *const u8 as *const libc::c_char);
    Ptsp(third_route.as_mut_ptr() as *mut libc::c_void);
    printf(b"\n# its energy is: %g\n\0" as *const u8 as *const libc::c_char, third_E);
}
unsafe extern "C" fn do_all_perms(mut route: *mut libc::c_int, mut n: libc::c_int) {
    if n as libc::c_ulong
        == (::std::mem::size_of::<[Stsp_city; 12]>() as libc::c_ulong)
            .wrapping_div(::std::mem::size_of::<Stsp_city>() as libc::c_ulong)
            .wrapping_sub(1 as libc::c_int as libc::c_ulong)
    {
        let mut E: libc::c_double = 0.;
        E = Etsp(route as *mut libc::c_void);
        if E < best_E {
            third_E = second_E;
            memcpy(
                third_route.as_mut_ptr() as *mut libc::c_void,
                second_route.as_mut_ptr() as *const libc::c_void,
                (::std::mem::size_of::<[Stsp_city; 12]>() as libc::c_ulong)
                    .wrapping_div(::std::mem::size_of::<Stsp_city>() as libc::c_ulong)
                    .wrapping_mul(::std::mem::size_of::<libc::c_int>() as libc::c_ulong),
            );
            second_E = best_E;
            memcpy(
                second_route.as_mut_ptr() as *mut libc::c_void,
                best_route.as_mut_ptr() as *const libc::c_void,
                (::std::mem::size_of::<[Stsp_city; 12]>() as libc::c_ulong)
                    .wrapping_div(::std::mem::size_of::<Stsp_city>() as libc::c_ulong)
                    .wrapping_mul(::std::mem::size_of::<libc::c_int>() as libc::c_ulong),
            );
            best_E = E;
            memcpy(
                best_route.as_mut_ptr() as *mut libc::c_void,
                route as *const libc::c_void,
                (::std::mem::size_of::<[Stsp_city; 12]>() as libc::c_ulong)
                    .wrapping_div(::std::mem::size_of::<Stsp_city>() as libc::c_ulong)
                    .wrapping_mul(::std::mem::size_of::<libc::c_int>() as libc::c_ulong),
            );
        } else if E < second_E {
            third_E = second_E;
            memcpy(
                third_route.as_mut_ptr() as *mut libc::c_void,
                second_route.as_mut_ptr() as *const libc::c_void,
                (::std::mem::size_of::<[Stsp_city; 12]>() as libc::c_ulong)
                    .wrapping_div(::std::mem::size_of::<Stsp_city>() as libc::c_ulong)
                    .wrapping_mul(::std::mem::size_of::<libc::c_int>() as libc::c_ulong),
            );
            second_E = E;
            memcpy(
                second_route.as_mut_ptr() as *mut libc::c_void,
                route as *const libc::c_void,
                (::std::mem::size_of::<[Stsp_city; 12]>() as libc::c_ulong)
                    .wrapping_div(::std::mem::size_of::<Stsp_city>() as libc::c_ulong)
                    .wrapping_mul(::std::mem::size_of::<libc::c_int>() as libc::c_ulong),
            );
        } else if E < third_E {
            third_E = E;
            memcpy(
                route as *mut libc::c_void,
                third_route.as_mut_ptr() as *const libc::c_void,
                (::std::mem::size_of::<[Stsp_city; 12]>() as libc::c_ulong)
                    .wrapping_div(::std::mem::size_of::<Stsp_city>() as libc::c_ulong)
                    .wrapping_mul(::std::mem::size_of::<libc::c_int>() as libc::c_ulong),
            );
        }
    } else {
        let mut new_route: [libc::c_int; 12] = [0; 12];
        let mut j: libc::c_uint = 0;
        let mut swap_tmp: libc::c_int = 0;
        memcpy(
            new_route.as_mut_ptr() as *mut libc::c_void,
            route as *const libc::c_void,
            (::std::mem::size_of::<[Stsp_city; 12]>() as libc::c_ulong)
                .wrapping_div(::std::mem::size_of::<Stsp_city>() as libc::c_ulong)
                .wrapping_mul(::std::mem::size_of::<libc::c_int>() as libc::c_ulong),
        );
        j = n as libc::c_uint;
        while (j as libc::c_ulong)
            < (::std::mem::size_of::<[Stsp_city; 12]>() as libc::c_ulong)
                .wrapping_div(::std::mem::size_of::<Stsp_city>() as libc::c_ulong)
        {
            swap_tmp = new_route[j as usize];
            new_route[j as usize] = new_route[n as usize];
            new_route[n as usize] = swap_tmp;
            do_all_perms(new_route.as_mut_ptr(), n + 1 as libc::c_int);
            j = j.wrapping_add(1);
            j;
        }
    };
}
pub fn main() {
    unsafe { ::std::process::exit(main_0() as i32) }
}
