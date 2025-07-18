use ::libc;
extern "C" {
    fn heman_image_create(
        width: libc::c_int,
        height: libc::c_int,
        nbands: libc::c_int,
    ) -> *mut heman_image;
    fn heman_image_sample(
        _: *mut heman_image,
        u: libc::c_float,
        v: libc::c_float,
        result: *mut libc::c_float,
    );
    fn rand() -> libc::c_int;
    fn malloc(_: libc::c_ulong) -> *mut libc::c_void;
    fn free(__ptr: *mut libc::c_void);
    fn memcpy(
        _: *mut libc::c_void,
        _: *const libc::c_void,
        _: libc::c_ulong,
    ) -> *mut libc::c_void;
    fn __assert_fail(
        __assertion: *const libc::c_char,
        __file: *const libc::c_char,
        __line: libc::c_uint,
        __function: *const libc::c_char,
    ) -> !;
    fn sqrtf(_: libc::c_float) -> libc::c_float;
    fn sqrt(_: libc::c_double) -> libc::c_double;
    fn ceil(_: libc::c_double) -> libc::c_double;
    fn kmVec2Subtract(
        pOut: *mut kmVec2,
        pV1: *const kmVec2,
        pV2: *const kmVec2,
    ) -> *mut kmVec2;
    fn kmVec2Add(
        pOut: *mut kmVec2,
        pV1: *const kmVec2,
        pV2: *const kmVec2,
    ) -> *mut kmVec2;
    fn kmVec2LengthSq(pIn: *const kmVec2) -> libc::c_float;
    fn kmVec2Scale(
        pOut: *mut kmVec2,
        pIn: *const kmVec2,
        s: libc::c_float,
    ) -> *mut kmVec2;
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct heman_image_s {
    pub width: libc::c_int,
    pub height: libc::c_int,
    pub nbands: libc::c_int,
    pub data: *mut libc::c_float,
}
pub type heman_image = heman_image_s;
pub type heman_points = heman_image_s;
#[derive(Copy, Clone)]
#[repr(C, packed)]
pub struct kmVec2 {
    pub x: libc::c_float,
    pub y: libc::c_float,
}
pub unsafe extern "C" fn randhash(mut seed: libc::c_uint) -> libc::c_uint {
    let mut i: libc::c_uint = (seed ^ 12345391 as libc::c_uint)
        .wrapping_mul(2654435769 as libc::c_uint);
    i ^= i << 6 as libc::c_int ^ i >> 26 as libc::c_int;
    i = i.wrapping_mul(2654435769 as libc::c_uint);
    i = i.wrapping_add(i << 5 as libc::c_int ^ i >> 12 as libc::c_int);
    return i;
}
pub unsafe extern "C" fn randhashf(
    mut seed: libc::c_uint,
    mut a: libc::c_float,
    mut b: libc::c_float,
) -> libc::c_float {
    return (b - a) * randhash(seed) as libc::c_float
        / (2147483647 as libc::c_int as libc::c_uint)
            .wrapping_mul(2 as libc::c_uint)
            .wrapping_add(1 as libc::c_uint) as libc::c_float + a;
}
pub unsafe extern "C" fn heman_points_create(
    mut xy: *mut libc::c_float,
    mut npoints: libc::c_int,
    mut nbands: libc::c_int,
) -> *mut heman_image {
    let mut img: *mut heman_points = malloc(
        ::std::mem::size_of::<heman_image>() as libc::c_ulong,
    ) as *mut heman_points;
    (*img).width = npoints;
    (*img).height = 1 as libc::c_int;
    (*img).nbands = nbands;
    let mut nbytes: libc::c_int = (::std::mem::size_of::<libc::c_float>()
        as libc::c_ulong)
        .wrapping_mul(npoints as libc::c_ulong)
        .wrapping_mul(nbands as libc::c_ulong) as libc::c_int;
    (*img).data = malloc(nbytes as libc::c_ulong) as *mut libc::c_float;
    memcpy(
        (*img).data as *mut libc::c_void,
        xy as *const libc::c_void,
        nbytes as libc::c_ulong,
    );
    return img;
}
pub unsafe extern "C" fn heman_points_destroy(mut img: *mut heman_points) {
    free((*img).data as *mut libc::c_void);
    free(img as *mut libc::c_void);
}
pub unsafe extern "C" fn heman_points_from_grid(
    mut width: libc::c_float,
    mut height: libc::c_float,
    mut cellsize: libc::c_float,
    mut jitter: libc::c_float,
) -> *mut heman_points {
    let mut cols: libc::c_int = (width / cellsize) as libc::c_int;
    let mut rows: libc::c_int = (height / cellsize) as libc::c_int;
    let mut ncells: libc::c_int = cols * rows;
    let mut result: *mut heman_points = heman_image_create(
        ncells,
        1 as libc::c_int,
        2 as libc::c_int,
    );
    let mut rscale: libc::c_float = (2.0f64 * jitter as libc::c_double
        / 2147483647 as libc::c_int as libc::c_float as libc::c_double) as libc::c_float;
    let mut j: libc::c_int = 0;
    j = 0 as libc::c_int;
    while j < rows {
        let mut dst: *mut libc::c_float = ((*result).data)
            .offset((j * cols * 2 as libc::c_int) as isize);
        let mut y: libc::c_float = (cellsize as libc::c_double * 0.5f64
            + (cellsize * j as libc::c_float) as libc::c_double) as libc::c_float;
        let mut x: libc::c_float = (cellsize as libc::c_double * 0.5f64)
            as libc::c_float;
        let mut i: libc::c_int = 0 as libc::c_int;
        while i < cols {
            let mut rx: libc::c_float = rand() as libc::c_float * rscale - jitter;
            let mut ry: libc::c_float = rand() as libc::c_float * rscale - jitter;
            let fresh0 = dst;
            dst = dst.offset(1);
            *fresh0 = x + rx;
            let fresh1 = dst;
            dst = dst.offset(1);
            *fresh1 = y + ry;
            x += cellsize;
            i += 1;
            i;
        }
        j += 1;
        j;
    }
    return result;
}
pub unsafe extern "C" fn sample_annulus(
    mut radius: libc::c_float,
    mut center: kmVec2,
    mut seedptr: *mut libc::c_uint,
) -> kmVec2 {
    let mut seed: libc::c_uint = *seedptr;
    let mut r: kmVec2 = kmVec2 { x: 0., y: 0. };
    let mut rscale: libc::c_float = 1.0f32
        / (2147483647 as libc::c_int as libc::c_uint)
            .wrapping_mul(2 as libc::c_uint)
            .wrapping_add(1 as libc::c_uint) as libc::c_float;
    loop {
        let fresh2 = seed;
        seed = seed.wrapping_add(1);
        r
            .x = 4 as libc::c_int as libc::c_float * rscale
            * randhash(fresh2) as libc::c_float - 2 as libc::c_int as libc::c_float;
        let fresh3 = seed;
        seed = seed.wrapping_add(1);
        r
            .y = 4 as libc::c_int as libc::c_float * rscale
            * randhash(fresh3) as libc::c_float - 2 as libc::c_int as libc::c_float;
        let mut r2: libc::c_float = kmVec2LengthSq(&mut r);
        if r2 > 1 as libc::c_int as libc::c_float
            && r2 <= 4 as libc::c_int as libc::c_float
        {
            break;
        }
    }
    *seedptr = seed;
    kmVec2Scale(&mut r, &mut r, radius);
    kmVec2Add(&mut r, &mut r, &mut center);
    return r;
}
pub unsafe extern "C" fn heman_points_from_poisson(
    mut width: libc::c_float,
    mut height: libc::c_float,
    mut radius: libc::c_float,
) -> *mut heman_points {
    let mut maxattempts: libc::c_int = 30 as libc::c_int;
    let mut rscale: libc::c_float = 1.0f32
        / (2147483647 as libc::c_int as libc::c_uint)
            .wrapping_mul(2 as libc::c_uint)
            .wrapping_add(1 as libc::c_uint) as libc::c_float;
    let mut seed: libc::c_uint = 0 as libc::c_int as libc::c_uint;
    let mut rvec: kmVec2 = kmVec2 { x: 0., y: 0. };
    rvec.y = radius;
    rvec.x = rvec.y;
    let mut r2: libc::c_float = radius * radius;
    let mut cellsize: libc::c_float = radius / sqrtf(2 as libc::c_int as libc::c_float);
    let mut invcell: libc::c_float = 1.0f32 / cellsize;
    let mut ncols: libc::c_int = ceil((width * invcell) as libc::c_double)
        as libc::c_int;
    let mut nrows: libc::c_int = ceil((height * invcell) as libc::c_double)
        as libc::c_int;
    let mut maxcol: libc::c_int = ncols - 1 as libc::c_int;
    let mut maxrow: libc::c_int = nrows - 1 as libc::c_int;
    let mut ncells: libc::c_int = ncols * nrows;
    let mut grid: *mut libc::c_int = malloc(
        (ncells as libc::c_ulong)
            .wrapping_mul(::std::mem::size_of::<libc::c_int>() as libc::c_ulong),
    ) as *mut libc::c_int;
    let mut i: libc::c_int = 0 as libc::c_int;
    while i < ncells {
        *grid.offset(i as isize) = -(1 as libc::c_int);
        i += 1;
        i;
    }
    let mut actives: *mut libc::c_int = malloc(
        (ncells as libc::c_ulong)
            .wrapping_mul(::std::mem::size_of::<libc::c_int>() as libc::c_ulong),
    ) as *mut libc::c_int;
    let mut nactives: libc::c_int = 0 as libc::c_int;
    let mut result: *mut heman_points = heman_image_create(
        ncells,
        1 as libc::c_int,
        2 as libc::c_int,
    );
    let mut samples: *mut kmVec2 = (*result).data as *mut kmVec2;
    let mut nsamples: libc::c_int = 0 as libc::c_int;
    let mut pt: kmVec2 = kmVec2 { x: 0., y: 0. };
    let fresh4 = seed;
    seed = seed.wrapping_add(1);
    pt.x = width * randhash(fresh4) as libc::c_float * rscale;
    let fresh5 = seed;
    seed = seed.wrapping_add(1);
    pt.y = height * randhash(fresh5) as libc::c_float * rscale;
    let fresh6 = nactives;
    nactives = nactives + 1;
    let ref mut fresh7 = *actives.offset(fresh6 as isize);
    *fresh7 = nsamples;
    *grid
        .offset(
            ((pt.x * invcell) as libc::c_int + ncols * (pt.y * invcell) as libc::c_int)
                as isize,
        ) = *fresh7;
    let fresh8 = nsamples;
    nsamples = nsamples + 1;
    *samples.offset(fresh8 as isize) = pt;
    while nsamples < ncells {
        let fresh9 = seed;
        seed = seed.wrapping_add(1);
        let mut aindex: libc::c_int = (if randhashf(
            fresh9,
            0 as libc::c_int as libc::c_float,
            nactives as libc::c_float,
        ) > (nactives - 1 as libc::c_int) as libc::c_float
        {
            (nactives - 1 as libc::c_int) as libc::c_float
        } else {
            let fresh10 = seed;
            seed = seed.wrapping_add(1);
            randhashf(
                fresh10,
                0 as libc::c_int as libc::c_float,
                nactives as libc::c_float,
            )
        }) as libc::c_int;
        let mut sindex: libc::c_int = *actives.offset(aindex as isize);
        let mut found: libc::c_int = 0 as libc::c_int;
        let mut j: kmVec2 = kmVec2 { x: 0., y: 0. };
        let mut minj: kmVec2 = kmVec2 { x: 0., y: 0. };
        let mut maxj: kmVec2 = kmVec2 { x: 0., y: 0. };
        let mut delta: kmVec2 = kmVec2 { x: 0., y: 0. };
        let mut attempt: libc::c_int = 0;
        attempt = 0 as libc::c_int;
        while attempt < maxattempts && found == 0 {
            pt = sample_annulus(radius, *samples.offset(sindex as isize), &mut seed);
            if !(pt.x < 0 as libc::c_int as libc::c_float || pt.x >= width
                || pt.y < 0 as libc::c_int as libc::c_float || pt.y >= height)
            {
                maxj = pt;
                minj = maxj;
                kmVec2Add(&mut maxj, &mut maxj, &mut rvec);
                kmVec2Subtract(&mut minj, &mut minj, &mut rvec);
                kmVec2Scale(&mut minj, &mut minj, invcell);
                kmVec2Scale(&mut maxj, &mut maxj, invcell);
                minj
                    .x = (if 0 as libc::c_int
                    > (if maxcol > minj.x as libc::c_int {
                        minj.x as libc::c_int
                    } else {
                        maxcol
                    })
                {
                    0 as libc::c_int
                } else if maxcol > minj.x as libc::c_int {
                    minj.x as libc::c_int
                } else {
                    maxcol
                }) as libc::c_float;
                maxj
                    .x = (if 0 as libc::c_int
                    > (if maxcol > maxj.x as libc::c_int {
                        maxj.x as libc::c_int
                    } else {
                        maxcol
                    })
                {
                    0 as libc::c_int
                } else if maxcol > maxj.x as libc::c_int {
                    maxj.x as libc::c_int
                } else {
                    maxcol
                }) as libc::c_float;
                minj
                    .y = (if 0 as libc::c_int
                    > (if maxrow > minj.y as libc::c_int {
                        minj.y as libc::c_int
                    } else {
                        maxrow
                    })
                {
                    0 as libc::c_int
                } else if maxrow > minj.y as libc::c_int {
                    minj.y as libc::c_int
                } else {
                    maxrow
                }) as libc::c_float;
                maxj
                    .y = (if 0 as libc::c_int
                    > (if maxrow > maxj.y as libc::c_int {
                        maxj.y as libc::c_int
                    } else {
                        maxrow
                    })
                {
                    0 as libc::c_int
                } else if maxrow > maxj.y as libc::c_int {
                    maxj.y as libc::c_int
                } else {
                    maxrow
                }) as libc::c_float;
                let mut reject: libc::c_int = 0 as libc::c_int;
                j.y = minj.y;
                while j.y <= maxj.y && reject == 0 {
                    j.x = minj.x;
                    while j.x <= maxj.x && reject == 0 {
                        let mut entry: libc::c_int = *grid
                            .offset(
                                (j.y as libc::c_int * ncols + j.x as libc::c_int) as isize,
                            );
                        if entry > -(1 as libc::c_int) && entry != sindex {
                            kmVec2Subtract(
                                &mut delta,
                                &mut *samples.offset(entry as isize),
                                &mut pt,
                            );
                            if kmVec2LengthSq(&mut delta) < r2 {
                                reject = 1 as libc::c_int;
                            }
                        }
                        j.x += 1.;
                        j.x;
                    }
                    j.y += 1.;
                    j.y;
                }
                if !(reject != 0) {
                    found = 1 as libc::c_int;
                }
            }
            attempt += 1;
            attempt;
        }
        if found != 0 {
            let fresh11 = nactives;
            nactives = nactives + 1;
            let ref mut fresh12 = *actives.offset(fresh11 as isize);
            *fresh12 = nsamples;
            *grid
                .offset(
                    ((pt.x * invcell) as libc::c_int
                        + ncols * (pt.y * invcell) as libc::c_int) as isize,
                ) = *fresh12;
            let fresh13 = nsamples;
            nsamples = nsamples + 1;
            *samples.offset(fresh13 as isize) = pt;
        } else {
            nactives -= 1;
            if nactives <= 0 as libc::c_int {
                break;
            }
            *actives.offset(aindex as isize) = *actives.offset(nactives as isize);
        }
    }
    (*result).width = nsamples;
    free(grid as *mut libc::c_void);
    free(actives as *mut libc::c_void);
    return result;
}
pub unsafe extern "C" fn heman_points_from_density(
    mut density: *mut heman_image,
    mut minradius: libc::c_float,
    mut maxradius: libc::c_float,
) -> *mut heman_points {
    if (*density).nbands == 1 as libc::c_int {} else {
        __assert_fail(
            b"density->nbands == 1\0" as *const u8 as *const libc::c_char,
            b"src/points.c\0" as *const u8 as *const libc::c_char,
            215 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 69],
                &[libc::c_char; 69],
            >(b"heman_points *heman_points_from_density(heman_image *, float, float)\0"))
                .as_ptr(),
        );
    }
    'c_2648: {
        if (*density).nbands == 1 as libc::c_int {} else {
            __assert_fail(
                b"density->nbands == 1\0" as *const u8 as *const libc::c_char,
                b"src/points.c\0" as *const u8 as *const libc::c_char,
                215 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 69],
                    &[libc::c_char; 69],
                >(
                    b"heman_points *heman_points_from_density(heman_image *, float, float)\0",
                ))
                    .as_ptr(),
            );
        }
    };
    let mut width: libc::c_float = 1 as libc::c_int as libc::c_float;
    let mut height: libc::c_float = 1 as libc::c_int as libc::c_float;
    let mut maxattempts: libc::c_int = 30 as libc::c_int;
    let mut rscale: libc::c_float = 1.0f32
        / (2147483647 as libc::c_int as libc::c_uint)
            .wrapping_mul(2 as libc::c_uint)
            .wrapping_add(1 as libc::c_uint) as libc::c_float;
    let mut seed: libc::c_uint = 0 as libc::c_int as libc::c_uint;
    let mut rvec: kmVec2 = kmVec2 { x: 0., y: 0. };
    rvec.y = maxradius;
    rvec.x = rvec.y;
    let mut gindex: libc::c_int = 0;
    let mut cellsize: libc::c_float = maxradius
        / sqrtf(2 as libc::c_int as libc::c_float);
    let mut invcell: libc::c_float = 1.0f32 / cellsize;
    let mut ncols: libc::c_int = ceil((width * invcell) as libc::c_double)
        as libc::c_int;
    let mut nrows: libc::c_int = ceil((height * invcell) as libc::c_double)
        as libc::c_int;
    let mut maxcol: libc::c_int = ncols - 1 as libc::c_int;
    let mut maxrow: libc::c_int = nrows - 1 as libc::c_int;
    let mut ncells: libc::c_int = ncols * nrows;
    let mut ntexels: libc::c_int = (cellsize * (*density).width as libc::c_float)
        as libc::c_int;
    let mut gcapacity: libc::c_int = ntexels * ntexels;
    let mut grid: *mut libc::c_int = malloc(
        (ncells as libc::c_ulong)
            .wrapping_mul(::std::mem::size_of::<libc::c_int>() as libc::c_ulong)
            .wrapping_mul(gcapacity as libc::c_ulong),
    ) as *mut libc::c_int;
    let mut ngrid: *mut libc::c_int = malloc(
        (ncells as libc::c_ulong)
            .wrapping_mul(::std::mem::size_of::<libc::c_int>() as libc::c_ulong),
    ) as *mut libc::c_int;
    let mut i: libc::c_int = 0 as libc::c_int;
    while i < ncells {
        *ngrid.offset(i as isize) = 0 as libc::c_int;
        i += 1;
        i;
    }
    let mut actives: *mut libc::c_int = malloc(
        (ncells as libc::c_ulong)
            .wrapping_mul(::std::mem::size_of::<libc::c_int>() as libc::c_ulong),
    ) as *mut libc::c_int;
    let mut nactives: libc::c_int = 0 as libc::c_int;
    let mut maxsamples: libc::c_int = ncells * gcapacity;
    let mut result: *mut heman_points = heman_image_create(
        maxsamples,
        1 as libc::c_int,
        2 as libc::c_int,
    );
    let mut samples: *mut kmVec2 = (*result).data as *mut kmVec2;
    let mut nsamples: libc::c_int = 0 as libc::c_int;
    let mut pt: kmVec2 = kmVec2 { x: 0., y: 0. };
    let fresh14 = seed;
    seed = seed.wrapping_add(1);
    pt.x = width * randhash(fresh14) as libc::c_float * rscale;
    let fresh15 = seed;
    seed = seed.wrapping_add(1);
    pt.y = height * randhash(fresh15) as libc::c_float * rscale;
    let fresh16 = nactives;
    nactives = nactives + 1;
    *actives.offset(fresh16 as isize) = nsamples;
    gindex = (pt.x * invcell) as libc::c_int + ncols * (pt.y * invcell) as libc::c_int;
    *grid
        .offset(
            (gcapacity * gindex + *ngrid.offset(gindex as isize)) as isize,
        ) = nsamples;
    let ref mut fresh17 = *ngrid.offset(gindex as isize);
    *fresh17 += 1;
    *fresh17;
    let fresh18 = nsamples;
    nsamples = nsamples + 1;
    *samples.offset(fresh18 as isize) = pt;
    while nsamples < maxsamples {
        let fresh19 = seed;
        seed = seed.wrapping_add(1);
        let mut aindex: libc::c_int = (if randhashf(
            fresh19,
            0 as libc::c_int as libc::c_float,
            nactives as libc::c_float,
        ) > (nactives - 1 as libc::c_int) as libc::c_float
        {
            (nactives - 1 as libc::c_int) as libc::c_float
        } else {
            let fresh20 = seed;
            seed = seed.wrapping_add(1);
            randhashf(
                fresh20,
                0 as libc::c_int as libc::c_float,
                nactives as libc::c_float,
            )
        }) as libc::c_int;
        let mut sindex: libc::c_int = *actives.offset(aindex as isize);
        let mut found: libc::c_int = 0 as libc::c_int;
        let mut j: kmVec2 = kmVec2 { x: 0., y: 0. };
        let mut minj: kmVec2 = kmVec2 { x: 0., y: 0. };
        let mut maxj: kmVec2 = kmVec2 { x: 0., y: 0. };
        let mut delta: kmVec2 = kmVec2 { x: 0., y: 0. };
        let mut attempt: libc::c_int = 0;
        attempt = 0 as libc::c_int;
        while attempt < maxattempts && found == 0 {
            pt = sample_annulus(maxradius, *samples.offset(sindex as isize), &mut seed);
            if !(pt.x < 0 as libc::c_int as libc::c_float || pt.x >= width
                || pt.y < 0 as libc::c_int as libc::c_float || pt.y >= height)
            {
                maxj = pt;
                minj = maxj;
                kmVec2Add(&mut maxj, &mut maxj, &mut rvec);
                kmVec2Subtract(&mut minj, &mut minj, &mut rvec);
                kmVec2Scale(&mut minj, &mut minj, invcell);
                kmVec2Scale(&mut maxj, &mut maxj, invcell);
                minj
                    .x = (if 0 as libc::c_int
                    > (if maxcol > minj.x as libc::c_int {
                        minj.x as libc::c_int
                    } else {
                        maxcol
                    })
                {
                    0 as libc::c_int
                } else if maxcol > minj.x as libc::c_int {
                    minj.x as libc::c_int
                } else {
                    maxcol
                }) as libc::c_float;
                maxj
                    .x = (if 0 as libc::c_int
                    > (if maxcol > maxj.x as libc::c_int {
                        maxj.x as libc::c_int
                    } else {
                        maxcol
                    })
                {
                    0 as libc::c_int
                } else if maxcol > maxj.x as libc::c_int {
                    maxj.x as libc::c_int
                } else {
                    maxcol
                }) as libc::c_float;
                minj
                    .y = (if 0 as libc::c_int
                    > (if maxrow > minj.y as libc::c_int {
                        minj.y as libc::c_int
                    } else {
                        maxrow
                    })
                {
                    0 as libc::c_int
                } else if maxrow > minj.y as libc::c_int {
                    minj.y as libc::c_int
                } else {
                    maxrow
                }) as libc::c_float;
                maxj
                    .y = (if 0 as libc::c_int
                    > (if maxrow > maxj.y as libc::c_int {
                        maxj.y as libc::c_int
                    } else {
                        maxrow
                    })
                {
                    0 as libc::c_int
                } else if maxrow > maxj.y as libc::c_int {
                    maxj.y as libc::c_int
                } else {
                    maxrow
                }) as libc::c_float;
                let mut reject: libc::c_int = 0 as libc::c_int;
                let mut densityval: libc::c_float = 0.;
                heman_image_sample(density, pt.x, pt.y, &mut densityval);
                densityval = sqrt(densityval as libc::c_double) as libc::c_float;
                let mut mindist: libc::c_float = maxradius
                    - densityval * (maxradius - minradius);
                let mut r2: libc::c_float = mindist * mindist;
                j.y = minj.y;
                while j.y <= maxj.y && reject == 0 {
                    j.x = minj.x;
                    while j.x <= maxj.x && reject == 0 {
                        let mut g: libc::c_int = (j.y as libc::c_int * ncols
                            + j.x as libc::c_int) * gcapacity;
                        while g
                            < (j.y as libc::c_int * ncols + j.x as libc::c_int)
                                * gcapacity
                                + *ngrid
                                    .offset(
                                        (j.y as libc::c_int * ncols + j.x as libc::c_int) as isize,
                                    )
                        {
                            let mut entry: libc::c_int = *grid.offset(g as isize);
                            if entry != sindex {
                                kmVec2Subtract(
                                    &mut delta,
                                    &mut *samples.offset(entry as isize),
                                    &mut pt,
                                );
                                if kmVec2LengthSq(&mut delta) < r2 {
                                    reject = 1 as libc::c_int;
                                }
                            }
                            g += 1;
                            g;
                        }
                        j.x += 1.;
                        j.x;
                    }
                    j.y += 1.;
                    j.y;
                }
                if !(reject != 0) {
                    found = 1 as libc::c_int;
                }
            }
            attempt += 1;
            attempt;
        }
        if found != 0
            && *ngrid
                .offset(
                    ((pt.x * invcell) as libc::c_int
                        + ncols * (pt.y * invcell) as libc::c_int) as isize,
                ) >= gcapacity
        {
            found = 0 as libc::c_int;
        }
        if found != 0 {
            let fresh21 = nactives;
            nactives = nactives + 1;
            *actives.offset(fresh21 as isize) = nsamples;
            gindex = (pt.x * invcell) as libc::c_int
                + ncols * (pt.y * invcell) as libc::c_int;
            *grid
                .offset(
                    (gcapacity * gindex + *ngrid.offset(gindex as isize)) as isize,
                ) = nsamples;
            let ref mut fresh22 = *ngrid.offset(gindex as isize);
            *fresh22 += 1;
            *fresh22;
            let fresh23 = nsamples;
            nsamples = nsamples + 1;
            *samples.offset(fresh23 as isize) = pt;
        } else {
            nactives -= 1;
            if nactives <= 0 as libc::c_int {
                break;
            }
            *actives.offset(aindex as isize) = *actives.offset(nactives as isize);
        }
    }
    (*result).width = nsamples;
    free(grid as *mut libc::c_void);
    free(ngrid as *mut libc::c_void);
    free(actives as *mut libc::c_void);
    return result;
}
