use ::libc;
extern "C" {
    pub type osn_context;
    fn heman_image_create(
        width: libc::c_int,
        height: libc::c_int,
        nbands: libc::c_int,
    ) -> *mut heman_image;
    fn heman_image_texel(
        _: *mut heman_image,
        x: libc::c_int,
        y: libc::c_int,
    ) -> *mut libc::c_float;
    fn heman_image_destroy(_: *mut heman_image);
    fn heman_color_to_grayscale(colorimg: *mut heman_image) -> *mut heman_image;
    fn heman_distance_identity_cpcf(
        width: libc::c_int,
        height: libc::c_int,
    ) -> *mut heman_image;
    fn open_simplex_noise_free(ctx: *mut osn_context);
    fn open_simplex_noise2(
        ctx: *mut osn_context,
        x: libc::c_double,
        y: libc::c_double,
    ) -> libc::c_double;
    fn open_simplex_noise(seed: int64_t, ctx: *mut *mut osn_context) -> libc::c_int;
    fn __assert_fail(
        __assertion: *const libc::c_char,
        __file: *const libc::c_char,
        __line: libc::c_uint,
        __function: *const libc::c_char,
    ) -> !;
    fn pow(_: libc::c_double, _: libc::c_double) -> libc::c_double;
    fn fabsf(_: libc::c_float) -> libc::c_float;
    fn floor(_: libc::c_double) -> libc::c_double;
    fn kmVec3Lerp(
        pOut: *mut kmVec3,
        pV1: *const kmVec3,
        pV2: *const kmVec3,
        t: libc::c_float,
    ) -> *mut kmVec3;
    fn memcpy(
        _: *mut libc::c_void,
        _: *const libc::c_void,
        _: libc::c_ulong,
    ) -> *mut libc::c_void;
    fn malloc(_: libc::c_ulong) -> *mut libc::c_void;
    fn free(__ptr: *mut libc::c_void);
    fn omp_get_max_threads() -> libc::c_int;
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
pub type heman_color = libc::c_uint;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct kmVec3 {
    pub x: libc::c_float,
    pub y: libc::c_float,
    pub z: libc::c_float,
}
pub type int64_t = __int64_t;
pub type __int64_t = libc::c_long;
pub unsafe extern "C" fn heman_get_num_threads() -> libc::c_int {
    return omp_get_max_threads();
}
pub unsafe extern "C" fn heman_ops_step(
    mut hmap: *mut heman_image,
    mut threshold: libc::c_float,
) -> *mut heman_image {
    if (*hmap).nbands == 1 as libc::c_int {} else {
        __assert_fail(
            b"hmap->nbands == 1\0" as *const u8 as *const libc::c_char,
            b"src/ops.c\0" as *const u8 as *const libc::c_char,
            17 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 50],
                &[libc::c_char; 50],
            >(b"heman_image *heman_ops_step(heman_image *, float)\0"))
                .as_ptr(),
        );
    }
    'c_1413: {
        if (*hmap).nbands == 1 as libc::c_int {} else {
            __assert_fail(
                b"hmap->nbands == 1\0" as *const u8 as *const libc::c_char,
                b"src/ops.c\0" as *const u8 as *const libc::c_char,
                17 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 50],
                    &[libc::c_char; 50],
                >(b"heman_image *heman_ops_step(heman_image *, float)\0"))
                    .as_ptr(),
            );
        }
    };
    let mut result: *mut heman_image = heman_image_create(
        (*hmap).width,
        (*hmap).height,
        1 as libc::c_int,
    );
    let mut size: libc::c_int = (*hmap).height * (*hmap).width;
    let mut src: *mut libc::c_float = (*hmap).data;
    let mut dst: *mut libc::c_float = (*result).data;
    let mut i: libc::c_int = 0 as libc::c_int;
    while i < size {
        let fresh0 = src;
        src = src.offset(1);
        let fresh1 = dst;
        dst = dst.offset(1);
        *fresh1 = (if *fresh0 >= threshold {
            1 as libc::c_int
        } else {
            0 as libc::c_int
        }) as libc::c_float;
        i += 1;
        i;
    }
    return result;
}
pub unsafe extern "C" fn heman_ops_max(
    mut imga: *mut heman_image,
    mut imgb: *mut heman_image,
) -> *mut heman_image {
    if (*imga).width == (*imgb).width {} else {
        __assert_fail(
            b"imga->width == imgb->width\0" as *const u8 as *const libc::c_char,
            b"src/ops.c\0" as *const u8 as *const libc::c_char,
            30 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 57],
                &[libc::c_char; 57],
            >(b"heman_image *heman_ops_max(heman_image *, heman_image *)\0"))
                .as_ptr(),
        );
    }
    'c_1291: {
        if (*imga).width == (*imgb).width {} else {
            __assert_fail(
                b"imga->width == imgb->width\0" as *const u8 as *const libc::c_char,
                b"src/ops.c\0" as *const u8 as *const libc::c_char,
                30 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 57],
                    &[libc::c_char; 57],
                >(b"heman_image *heman_ops_max(heman_image *, heman_image *)\0"))
                    .as_ptr(),
            );
        }
    };
    if (*imga).height == (*imgb).height {} else {
        __assert_fail(
            b"imga->height == imgb->height\0" as *const u8 as *const libc::c_char,
            b"src/ops.c\0" as *const u8 as *const libc::c_char,
            31 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 57],
                &[libc::c_char; 57],
            >(b"heman_image *heman_ops_max(heman_image *, heman_image *)\0"))
                .as_ptr(),
        );
    }
    'c_1245: {
        if (*imga).height == (*imgb).height {} else {
            __assert_fail(
                b"imga->height == imgb->height\0" as *const u8 as *const libc::c_char,
                b"src/ops.c\0" as *const u8 as *const libc::c_char,
                31 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 57],
                    &[libc::c_char; 57],
                >(b"heman_image *heman_ops_max(heman_image *, heman_image *)\0"))
                    .as_ptr(),
            );
        }
    };
    if (*imga).nbands == (*imgb).nbands {} else {
        __assert_fail(
            b"imga->nbands == imgb->nbands\0" as *const u8 as *const libc::c_char,
            b"src/ops.c\0" as *const u8 as *const libc::c_char,
            32 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 57],
                &[libc::c_char; 57],
            >(b"heman_image *heman_ops_max(heman_image *, heman_image *)\0"))
                .as_ptr(),
        );
    }
    'c_1197: {
        if (*imga).nbands == (*imgb).nbands {} else {
            __assert_fail(
                b"imga->nbands == imgb->nbands\0" as *const u8 as *const libc::c_char,
                b"src/ops.c\0" as *const u8 as *const libc::c_char,
                32 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 57],
                    &[libc::c_char; 57],
                >(b"heman_image *heman_ops_max(heman_image *, heman_image *)\0"))
                    .as_ptr(),
            );
        }
    };
    let mut result: *mut heman_image = heman_image_create(
        (*imga).width,
        (*imga).height,
        (*imga).nbands,
    );
    let mut size: libc::c_int = (*imga).height * (*imga).width * (*imga).nbands;
    let mut srca: *mut libc::c_float = (*imga).data;
    let mut srcb: *mut libc::c_float = (*imgb).data;
    let mut dst: *mut libc::c_float = (*result).data;
    let mut i: libc::c_int = 0 as libc::c_int;
    while i < size {
        *dst = if *srca > *srcb { *srca } else { *srcb };
        i += 1;
        i;
        dst = dst.offset(1);
        dst;
        srca = srca.offset(1);
        srca;
        srcb = srcb.offset(1);
        srcb;
    }
    return result;
}
pub unsafe extern "C" fn heman_ops_sweep(
    mut hmap: *mut heman_image,
) -> *mut heman_image {
    if (*hmap).nbands == 1 as libc::c_int {} else {
        __assert_fail(
            b"hmap->nbands == 1\0" as *const u8 as *const libc::c_char,
            b"src/ops.c\0" as *const u8 as *const libc::c_char,
            47 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 44],
                &[libc::c_char; 44],
            >(b"heman_image *heman_ops_sweep(heman_image *)\0"))
                .as_ptr(),
        );
    }
    'c_2539: {
        if (*hmap).nbands == 1 as libc::c_int {} else {
            __assert_fail(
                b"hmap->nbands == 1\0" as *const u8 as *const libc::c_char,
                b"src/ops.c\0" as *const u8 as *const libc::c_char,
                47 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 44],
                    &[libc::c_char; 44],
                >(b"heman_image *heman_ops_sweep(heman_image *)\0"))
                    .as_ptr(),
            );
        }
    };
    let mut result: *mut heman_image = heman_image_create(
        (*hmap).height,
        1 as libc::c_int,
        1 as libc::c_int,
    );
    let mut dst: *mut libc::c_float = (*result).data;
    let mut src: *const libc::c_float = (*hmap).data;
    let mut invw: libc::c_float = 1.0f32 / (*hmap).width as libc::c_float;
    let mut y: libc::c_int = 0 as libc::c_int;
    while y < (*hmap).height {
        let mut acc: libc::c_float = 0 as libc::c_int as libc::c_float;
        let mut x: libc::c_int = 0 as libc::c_int;
        while x < (*hmap).width {
            let fresh2 = src;
            src = src.offset(1);
            acc += *fresh2;
            x += 1;
            x;
        }
        let fresh3 = dst;
        dst = dst.offset(1);
        *fresh3 = acc * invw;
        y += 1;
        y;
    }
    return result;
}
unsafe extern "C" fn copy_row(
    mut src: *mut heman_image,
    mut dst: *mut heman_image,
    mut dstx: libc::c_int,
    mut y: libc::c_int,
) {
    let mut width: libc::c_int = (*src).width;
    if (*src).nbands == 1 as libc::c_int {
        let mut x: libc::c_int = 0 as libc::c_int;
        while x < width {
            let mut srcp: *mut libc::c_float = heman_image_texel(src, x, y);
            let mut dstp: *mut libc::c_float = heman_image_texel(dst, dstx + x, y);
            *dstp = *srcp;
            x += 1;
            x;
        }
        return;
    }
    let mut x_0: libc::c_int = 0 as libc::c_int;
    while x_0 < width {
        let mut srcp_0: *mut libc::c_float = heman_image_texel(src, x_0, y);
        let mut dstp_0: *mut libc::c_float = heman_image_texel(dst, dstx + x_0, y);
        let mut nbands: libc::c_int = (*src).nbands;
        loop {
            let fresh4 = nbands;
            nbands = nbands - 1;
            if !(fresh4 != 0) {
                break;
            }
            let fresh5 = srcp_0;
            srcp_0 = srcp_0.offset(1);
            let fresh6 = dstp_0;
            dstp_0 = dstp_0.offset(1);
            *fresh6 = *fresh5;
        }
        x_0 += 1;
        x_0;
    }
}
pub unsafe extern "C" fn heman_ops_stitch_horizontal(
    mut images: *mut *mut heman_image,
    mut count: libc::c_int,
) -> *mut heman_image {
    if count > 0 as libc::c_int {} else {
        __assert_fail(
            b"count > 0\0" as *const u8 as *const libc::c_char,
            b"src/ops.c\0" as *const u8 as *const libc::c_char,
            85 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 62],
                &[libc::c_char; 62],
            >(b"heman_image *heman_ops_stitch_horizontal(heman_image **, int)\0"))
                .as_ptr(),
        );
    }
    'c_618: {
        if count > 0 as libc::c_int {} else {
            __assert_fail(
                b"count > 0\0" as *const u8 as *const libc::c_char,
                b"src/ops.c\0" as *const u8 as *const libc::c_char,
                85 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 62],
                    &[libc::c_char; 62],
                >(b"heman_image *heman_ops_stitch_horizontal(heman_image **, int)\0"))
                    .as_ptr(),
            );
        }
    };
    let mut width: libc::c_int = (**images.offset(0 as libc::c_int as isize)).width;
    let mut height: libc::c_int = (**images.offset(0 as libc::c_int as isize)).height;
    let mut nbands: libc::c_int = (**images.offset(0 as libc::c_int as isize)).nbands;
    let mut i: libc::c_int = 1 as libc::c_int;
    while i < count {
        if (**images.offset(i as isize)).width == width {} else {
            __assert_fail(
                b"images[i]->width == width\0" as *const u8 as *const libc::c_char,
                b"src/ops.c\0" as *const u8 as *const libc::c_char,
                90 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 62],
                    &[libc::c_char; 62],
                >(b"heman_image *heman_ops_stitch_horizontal(heman_image **, int)\0"))
                    .as_ptr(),
            );
        }
        'c_562: {
            if (**images.offset(i as isize)).width == width {} else {
                __assert_fail(
                    b"images[i]->width == width\0" as *const u8 as *const libc::c_char,
                    b"src/ops.c\0" as *const u8 as *const libc::c_char,
                    90 as libc::c_int as libc::c_uint,
                    (*::std::mem::transmute::<
                        &[u8; 62],
                        &[libc::c_char; 62],
                    >(
                        b"heman_image *heman_ops_stitch_horizontal(heman_image **, int)\0",
                    ))
                        .as_ptr(),
                );
            }
        };
        if (**images.offset(i as isize)).height == height {} else {
            __assert_fail(
                b"images[i]->height == height\0" as *const u8 as *const libc::c_char,
                b"src/ops.c\0" as *const u8 as *const libc::c_char,
                91 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 62],
                    &[libc::c_char; 62],
                >(b"heman_image *heman_ops_stitch_horizontal(heman_image **, int)\0"))
                    .as_ptr(),
            );
        }
        'c_512: {
            if (**images.offset(i as isize)).height == height {} else {
                __assert_fail(
                    b"images[i]->height == height\0" as *const u8 as *const libc::c_char,
                    b"src/ops.c\0" as *const u8 as *const libc::c_char,
                    91 as libc::c_int as libc::c_uint,
                    (*::std::mem::transmute::<
                        &[u8; 62],
                        &[libc::c_char; 62],
                    >(
                        b"heman_image *heman_ops_stitch_horizontal(heman_image **, int)\0",
                    ))
                        .as_ptr(),
                );
            }
        };
        if (**images.offset(i as isize)).nbands == nbands {} else {
            __assert_fail(
                b"images[i]->nbands == nbands\0" as *const u8 as *const libc::c_char,
                b"src/ops.c\0" as *const u8 as *const libc::c_char,
                92 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 62],
                    &[libc::c_char; 62],
                >(b"heman_image *heman_ops_stitch_horizontal(heman_image **, int)\0"))
                    .as_ptr(),
            );
        }
        'c_448: {
            if (**images.offset(i as isize)).nbands == nbands {} else {
                __assert_fail(
                    b"images[i]->nbands == nbands\0" as *const u8 as *const libc::c_char,
                    b"src/ops.c\0" as *const u8 as *const libc::c_char,
                    92 as libc::c_int as libc::c_uint,
                    (*::std::mem::transmute::<
                        &[u8; 62],
                        &[libc::c_char; 62],
                    >(
                        b"heman_image *heman_ops_stitch_horizontal(heman_image **, int)\0",
                    ))
                        .as_ptr(),
                );
            }
        };
        i += 1;
        i;
    }
    let mut result: *mut heman_image = heman_image_create(width * count, height, nbands);
    let mut y: libc::c_int = 0;
    y = 0 as libc::c_int;
    while y < height {
        let mut tile: libc::c_int = 0 as libc::c_int;
        while tile < count {
            copy_row(*images.offset(tile as isize), result, tile * width, y);
            tile += 1;
            tile;
        }
        y += 1;
        y;
    }
    return result;
}
pub unsafe extern "C" fn heman_ops_stitch_vertical(
    mut images: *mut *mut heman_image,
    mut count: libc::c_int,
) -> *mut heman_image {
    if count > 0 as libc::c_int {} else {
        __assert_fail(
            b"count > 0\0" as *const u8 as *const libc::c_char,
            b"src/ops.c\0" as *const u8 as *const libc::c_char,
            109 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 60],
                &[libc::c_char; 60],
            >(b"heman_image *heman_ops_stitch_vertical(heman_image **, int)\0"))
                .as_ptr(),
        );
    }
    'c_930: {
        if count > 0 as libc::c_int {} else {
            __assert_fail(
                b"count > 0\0" as *const u8 as *const libc::c_char,
                b"src/ops.c\0" as *const u8 as *const libc::c_char,
                109 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 60],
                    &[libc::c_char; 60],
                >(b"heman_image *heman_ops_stitch_vertical(heman_image **, int)\0"))
                    .as_ptr(),
            );
        }
    };
    let mut width: libc::c_int = (**images.offset(0 as libc::c_int as isize)).width;
    let mut height: libc::c_int = (**images.offset(0 as libc::c_int as isize)).height;
    let mut nbands: libc::c_int = (**images.offset(0 as libc::c_int as isize)).nbands;
    let mut i: libc::c_int = 1 as libc::c_int;
    while i < count {
        if (**images.offset(i as isize)).width == width {} else {
            __assert_fail(
                b"images[i]->width == width\0" as *const u8 as *const libc::c_char,
                b"src/ops.c\0" as *const u8 as *const libc::c_char,
                114 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 60],
                    &[libc::c_char; 60],
                >(b"heman_image *heman_ops_stitch_vertical(heman_image **, int)\0"))
                    .as_ptr(),
            );
        }
        'c_875: {
            if (**images.offset(i as isize)).width == width {} else {
                __assert_fail(
                    b"images[i]->width == width\0" as *const u8 as *const libc::c_char,
                    b"src/ops.c\0" as *const u8 as *const libc::c_char,
                    114 as libc::c_int as libc::c_uint,
                    (*::std::mem::transmute::<
                        &[u8; 60],
                        &[libc::c_char; 60],
                    >(b"heman_image *heman_ops_stitch_vertical(heman_image **, int)\0"))
                        .as_ptr(),
                );
            }
        };
        if (**images.offset(i as isize)).height == height {} else {
            __assert_fail(
                b"images[i]->height == height\0" as *const u8 as *const libc::c_char,
                b"src/ops.c\0" as *const u8 as *const libc::c_char,
                115 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 60],
                    &[libc::c_char; 60],
                >(b"heman_image *heman_ops_stitch_vertical(heman_image **, int)\0"))
                    .as_ptr(),
            );
        }
        'c_825: {
            if (**images.offset(i as isize)).height == height {} else {
                __assert_fail(
                    b"images[i]->height == height\0" as *const u8 as *const libc::c_char,
                    b"src/ops.c\0" as *const u8 as *const libc::c_char,
                    115 as libc::c_int as libc::c_uint,
                    (*::std::mem::transmute::<
                        &[u8; 60],
                        &[libc::c_char; 60],
                    >(b"heman_image *heman_ops_stitch_vertical(heman_image **, int)\0"))
                        .as_ptr(),
                );
            }
        };
        if (**images.offset(i as isize)).nbands == nbands {} else {
            __assert_fail(
                b"images[i]->nbands == nbands\0" as *const u8 as *const libc::c_char,
                b"src/ops.c\0" as *const u8 as *const libc::c_char,
                116 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 60],
                    &[libc::c_char; 60],
                >(b"heman_image *heman_ops_stitch_vertical(heman_image **, int)\0"))
                    .as_ptr(),
            );
        }
        'c_772: {
            if (**images.offset(i as isize)).nbands == nbands {} else {
                __assert_fail(
                    b"images[i]->nbands == nbands\0" as *const u8 as *const libc::c_char,
                    b"src/ops.c\0" as *const u8 as *const libc::c_char,
                    116 as libc::c_int as libc::c_uint,
                    (*::std::mem::transmute::<
                        &[u8; 60],
                        &[libc::c_char; 60],
                    >(b"heman_image *heman_ops_stitch_vertical(heman_image **, int)\0"))
                        .as_ptr(),
                );
            }
        };
        i += 1;
        i;
    }
    let mut result: *mut heman_image = heman_image_create(width, height * count, nbands);
    let mut size: libc::c_int = width * height * nbands;
    let mut dst: *mut libc::c_float = (*result).data;
    let mut tile: libc::c_int = 0 as libc::c_int;
    while tile < count {
        memcpy(
            dst as *mut libc::c_void,
            (**images.offset(tile as isize)).data as *const libc::c_void,
            (size as libc::c_ulong)
                .wrapping_mul(::std::mem::size_of::<libc::c_float>() as libc::c_ulong),
        );
        dst = dst.offset(size as isize);
        tile += 1;
        tile;
    }
    return result;
}
pub unsafe extern "C" fn heman_ops_normalize_f32(
    mut source: *mut heman_image,
    mut minv: libc::c_float,
    mut maxv: libc::c_float,
) -> *mut heman_image {
    let mut result: *mut heman_image = heman_image_create(
        (*source).width,
        (*source).height,
        (*source).nbands,
    );
    let mut src: *mut libc::c_float = (*source).data;
    let mut dst: *mut libc::c_float = (*result).data;
    let mut scale: libc::c_float = 1.0f32 / (maxv - minv);
    let mut size: libc::c_int = (*source).height * (*source).width * (*source).nbands;
    let mut i: libc::c_int = 0 as libc::c_int;
    while i < size {
        let fresh7 = src;
        src = src.offset(1);
        let mut v: libc::c_float = (*fresh7 - minv) * scale;
        let fresh8 = dst;
        dst = dst.offset(1);
        *fresh8 = if 0 as libc::c_int as libc::c_float
            > (if 1 as libc::c_int as libc::c_float > v {
                v
            } else {
                1 as libc::c_int as libc::c_float
            })
        {
            0 as libc::c_int as libc::c_float
        } else if 1 as libc::c_int as libc::c_float > v {
            v
        } else {
            1 as libc::c_int as libc::c_float
        };
        i += 1;
        i;
    }
    return result;
}
pub unsafe extern "C" fn heman_ops_laplacian(
    mut heightmap: *mut heman_image,
) -> *mut heman_image {
    if (*heightmap).nbands == 1 as libc::c_int {} else {
        __assert_fail(
            b"heightmap->nbands == 1\0" as *const u8 as *const libc::c_char,
            b"src/ops.c\0" as *const u8 as *const libc::c_char,
            146 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 48],
                &[libc::c_char; 48],
            >(b"heman_image *heman_ops_laplacian(heman_image *)\0"))
                .as_ptr(),
        );
    }
    'c_2765: {
        if (*heightmap).nbands == 1 as libc::c_int {} else {
            __assert_fail(
                b"heightmap->nbands == 1\0" as *const u8 as *const libc::c_char,
                b"src/ops.c\0" as *const u8 as *const libc::c_char,
                146 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 48],
                    &[libc::c_char; 48],
                >(b"heman_image *heman_ops_laplacian(heman_image *)\0"))
                    .as_ptr(),
            );
        }
    };
    let mut width: libc::c_int = (*heightmap).width;
    let mut height: libc::c_int = (*heightmap).height;
    let mut result: *mut heman_image = heman_image_create(
        width,
        height,
        1 as libc::c_int,
    );
    let mut maxx: libc::c_int = width - 1 as libc::c_int;
    let mut maxy: libc::c_int = height - 1 as libc::c_int;
    let mut y: libc::c_int = 0;
    y = 0 as libc::c_int;
    while y < height {
        let mut y1: libc::c_int = if y + 1 as libc::c_int > maxy {
            maxy
        } else {
            y + 1 as libc::c_int
        };
        let mut dst: *mut libc::c_float = ((*result).data).offset((y * width) as isize);
        let mut x: libc::c_int = 0 as libc::c_int;
        while x < width {
            let mut x1: libc::c_int = if x + 1 as libc::c_int > maxx {
                maxx
            } else {
                x + 1 as libc::c_int
            };
            let mut p: libc::c_float = *heman_image_texel(heightmap, x, y);
            let mut px: libc::c_float = *heman_image_texel(heightmap, x1, y);
            let mut py: libc::c_float = *heman_image_texel(heightmap, x, y1);
            let fresh9 = dst;
            dst = dst.offset(1);
            *fresh9 = (p - px) * (p - px) + (p - py) * (p - py);
            x += 1;
            x;
        }
        y += 1;
        y;
    }
    return result;
}
pub unsafe extern "C" fn heman_ops_accumulate(
    mut dst: *mut heman_image,
    mut src: *mut heman_image,
) {
    if (*dst).nbands == (*src).nbands {} else {
        __assert_fail(
            b"dst->nbands == src->nbands\0" as *const u8 as *const libc::c_char,
            b"src/ops.c\0" as *const u8 as *const libc::c_char,
            172 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 56],
                &[libc::c_char; 56],
            >(b"void heman_ops_accumulate(heman_image *, heman_image *)\0"))
                .as_ptr(),
        );
    }
    'c_3427: {
        if (*dst).nbands == (*src).nbands {} else {
            __assert_fail(
                b"dst->nbands == src->nbands\0" as *const u8 as *const libc::c_char,
                b"src/ops.c\0" as *const u8 as *const libc::c_char,
                172 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 56],
                    &[libc::c_char; 56],
                >(b"void heman_ops_accumulate(heman_image *, heman_image *)\0"))
                    .as_ptr(),
            );
        }
    };
    if (*dst).width == (*src).width {} else {
        __assert_fail(
            b"dst->width == src->width\0" as *const u8 as *const libc::c_char,
            b"src/ops.c\0" as *const u8 as *const libc::c_char,
            173 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 56],
                &[libc::c_char; 56],
            >(b"void heman_ops_accumulate(heman_image *, heman_image *)\0"))
                .as_ptr(),
        );
    }
    'c_3380: {
        if (*dst).width == (*src).width {} else {
            __assert_fail(
                b"dst->width == src->width\0" as *const u8 as *const libc::c_char,
                b"src/ops.c\0" as *const u8 as *const libc::c_char,
                173 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 56],
                    &[libc::c_char; 56],
                >(b"void heman_ops_accumulate(heman_image *, heman_image *)\0"))
                    .as_ptr(),
            );
        }
    };
    if (*dst).height == (*src).height {} else {
        __assert_fail(
            b"dst->height == src->height\0" as *const u8 as *const libc::c_char,
            b"src/ops.c\0" as *const u8 as *const libc::c_char,
            174 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 56],
                &[libc::c_char; 56],
            >(b"void heman_ops_accumulate(heman_image *, heman_image *)\0"))
                .as_ptr(),
        );
    }
    'c_3333: {
        if (*dst).height == (*src).height {} else {
            __assert_fail(
                b"dst->height == src->height\0" as *const u8 as *const libc::c_char,
                b"src/ops.c\0" as *const u8 as *const libc::c_char,
                174 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 56],
                    &[libc::c_char; 56],
                >(b"void heman_ops_accumulate(heman_image *, heman_image *)\0"))
                    .as_ptr(),
            );
        }
    };
    let mut size: libc::c_int = (*dst).height * (*dst).width;
    let mut sdata: *mut libc::c_float = (*src).data;
    let mut ddata: *mut libc::c_float = (*dst).data;
    let mut i: libc::c_int = 0 as libc::c_int;
    while i < size {
        let fresh10 = sdata;
        sdata = sdata.offset(1);
        let fresh11 = ddata;
        ddata = ddata.offset(1);
        *fresh11 += *fresh10;
        i += 1;
        i;
    }
}
pub unsafe extern "C" fn heman_ops_sobel(
    mut img: *mut heman_image,
    mut rgb: heman_color,
) -> *mut heman_image {
    let mut width: libc::c_int = (*img).width;
    let mut height: libc::c_int = (*img).height;
    if (*img).nbands == 3 as libc::c_int {} else {
        __assert_fail(
            b"img->nbands == 3\0" as *const u8 as *const libc::c_char,
            b"src/ops.c\0" as *const u8 as *const libc::c_char,
            187 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 57],
                &[libc::c_char; 57],
            >(b"heman_image *heman_ops_sobel(heman_image *, heman_color)\0"))
                .as_ptr(),
        );
    }
    'c_3239: {
        if (*img).nbands == 3 as libc::c_int {} else {
            __assert_fail(
                b"img->nbands == 3\0" as *const u8 as *const libc::c_char,
                b"src/ops.c\0" as *const u8 as *const libc::c_char,
                187 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 57],
                    &[libc::c_char; 57],
                >(b"heman_image *heman_ops_sobel(heman_image *, heman_color)\0"))
                    .as_ptr(),
            );
        }
    };
    let mut result: *mut heman_image = heman_image_create(
        width,
        height,
        3 as libc::c_int,
    );
    let mut gray: *mut heman_image = heman_color_to_grayscale(img);
    let mut inv: libc::c_float = 1.0f32 / 255.0f32;
    let mut edge_rgb: kmVec3 = kmVec3 { x: 0., y: 0., z: 0. };
    edge_rgb.x = (rgb >> 16 as libc::c_int) as libc::c_float * inv;
    edge_rgb
        .y = (rgb >> 8 as libc::c_int & 0xff as libc::c_int as libc::c_uint)
        as libc::c_float * inv;
    edge_rgb.z = (rgb & 0xff as libc::c_int as libc::c_uint) as libc::c_float * inv;
    let mut y: libc::c_int = 0;
    y = 0 as libc::c_int;
    while y < height {
        let mut dst: *mut kmVec3 = ((*result).data as *mut kmVec3)
            .offset((y * width) as isize);
        let mut src: *const kmVec3 = ((*img).data as *mut kmVec3)
            .offset((y * width) as isize);
        let mut x: libc::c_int = 0 as libc::c_int;
        while x < width {
            let mut xm1: libc::c_int = if x - 1 as libc::c_int > 0 as libc::c_int {
                x - 1 as libc::c_int
            } else {
                0 as libc::c_int
            };
            let mut xp1: libc::c_int = if x + 1 as libc::c_int > width - 1 as libc::c_int
            {
                width - 1 as libc::c_int
            } else {
                x + 1 as libc::c_int
            };
            let mut ym1: libc::c_int = if y - 1 as libc::c_int > 0 as libc::c_int {
                y - 1 as libc::c_int
            } else {
                0 as libc::c_int
            };
            let mut yp1: libc::c_int = if y + 1 as libc::c_int
                > height - 1 as libc::c_int
            {
                height - 1 as libc::c_int
            } else {
                y + 1 as libc::c_int
            };
            let mut t00: libc::c_float = *heman_image_texel(gray, xm1, ym1);
            let mut t10: libc::c_float = *heman_image_texel(gray, x, ym1);
            let mut t20: libc::c_float = *heman_image_texel(gray, xp1, ym1);
            let mut t01: libc::c_float = *heman_image_texel(gray, xm1, 0 as libc::c_int);
            let mut t21: libc::c_float = *heman_image_texel(gray, xp1, 0 as libc::c_int);
            let mut t02: libc::c_float = *heman_image_texel(gray, xm1, yp1);
            let mut t12: libc::c_float = *heman_image_texel(gray, x, yp1);
            let mut t22: libc::c_float = *heman_image_texel(gray, xp1, yp1);
            let mut gx: libc::c_float = (t00 as libc::c_double
                + 2.0f64 * t01 as libc::c_double + t02 as libc::c_double
                - t20 as libc::c_double - 2.0f64 * t21 as libc::c_double
                - t22 as libc::c_double) as libc::c_float;
            let mut gy: libc::c_float = (t00 as libc::c_double
                + 2.0f64 * t10 as libc::c_double + t20 as libc::c_double
                - t02 as libc::c_double - 2.0f64 * t12 as libc::c_double
                - t22 as libc::c_double) as libc::c_float;
            let mut is_edge: libc::c_float = ((gx * gx + gy * gy) as libc::c_double
                > 1e-5f64) as libc::c_int as libc::c_float;
            let fresh12 = dst;
            dst = dst.offset(1);
            let fresh13 = src;
            src = src.offset(1);
            kmVec3Lerp(fresh12, fresh13, &mut edge_rgb, is_edge);
            x += 1;
            x;
        }
        y += 1;
        y;
    }
    heman_image_destroy(gray);
    return result;
}
pub unsafe extern "C" fn heman_ops_warp_core(
    mut img: *mut heman_image,
    mut secondary: *mut heman_image,
    mut seed: libc::c_int,
    mut octaves: libc::c_int,
) -> *mut heman_image {
    let mut ctx: *mut osn_context = 0 as *mut osn_context;
    open_simplex_noise(seed as int64_t, &mut ctx);
    let mut width: libc::c_int = (*img).width;
    let mut height: libc::c_int = (*img).height;
    let mut nbands: libc::c_int = (*img).nbands;
    let mut result: *mut heman_image = heman_image_create(width, height, nbands);
    let mut result2: *mut heman_image = if !secondary.is_null() {
        heman_image_create(width, height, (*secondary).nbands)
    } else {
        0 as *mut heman_image
    };
    let mut invw: libc::c_float = (1.0f64 / width as libc::c_double) as libc::c_float;
    let mut invh: libc::c_float = (1.0f64 / height as libc::c_double) as libc::c_float;
    let mut inv: libc::c_float = if invw > invh { invh } else { invw };
    let mut aspect: libc::c_float = width as libc::c_float / height as libc::c_float;
    let mut gain: libc::c_float = 0.6f64 as libc::c_float;
    let mut lacunarity: libc::c_float = 2.0f64 as libc::c_float;
    let mut initial_amplitude: libc::c_float = 0.05f64 as libc::c_float;
    let mut initial_frequency: libc::c_float = 8.0f64 as libc::c_float;
    let mut y: libc::c_int = 0;
    y = 0 as libc::c_int;
    while y < height {
        let mut dst: *mut libc::c_float = ((*result).data)
            .offset((y * width * nbands) as isize);
        let mut x: libc::c_int = 0 as libc::c_int;
        while x < width {
            let mut a: libc::c_float = initial_amplitude;
            let mut f: libc::c_float = initial_frequency;
            let mut src: *mut libc::c_float = 0 as *mut libc::c_float;
            if nbands == 4 as libc::c_int {
                src = heman_image_texel(img, x, y);
                let mut elev: libc::c_float = 1 as libc::c_int as libc::c_float
                    - *src.offset(3 as libc::c_int as isize);
                a = (a as libc::c_double
                    * pow(elev as libc::c_double, 4 as libc::c_int as libc::c_double))
                    as libc::c_float;
            }
            let mut s: libc::c_float = x as libc::c_float * inv;
            let mut t: libc::c_float = y as libc::c_float * inv;
            let mut u: libc::c_float = x as libc::c_float * invw;
            let mut v: libc::c_float = y as libc::c_float * invh;
            let mut i: libc::c_int = 0 as libc::c_int;
            while i < octaves {
                u = (u as libc::c_double
                    + a as libc::c_double
                        * open_simplex_noise2(
                            ctx,
                            (s * f) as libc::c_double,
                            (t * f) as libc::c_double,
                        )) as libc::c_float;
                v = (v as libc::c_double
                    + aspect as libc::c_double
                        * (a as libc::c_double
                            * open_simplex_noise2(
                                ctx,
                                (s * f) as libc::c_double + 0.5f64,
                                (t * f) as libc::c_double,
                            ))) as libc::c_float;
                a *= gain;
                f *= lacunarity;
                i += 1;
                i;
            }
            let mut i_0: libc::c_int = (if 0 as libc::c_int as libc::c_float
                > (if (width - 1 as libc::c_int) as libc::c_float
                    > u * width as libc::c_float
                {
                    u * width as libc::c_float
                } else {
                    (width - 1 as libc::c_int) as libc::c_float
                })
            {
                0 as libc::c_int as libc::c_float
            } else if (width - 1 as libc::c_int) as libc::c_float
                > u * width as libc::c_float
            {
                u * width as libc::c_float
            } else {
                (width - 1 as libc::c_int) as libc::c_float
            }) as libc::c_int;
            let mut j: libc::c_int = (if 0 as libc::c_int as libc::c_float
                > (if (height - 1 as libc::c_int) as libc::c_float
                    > v * height as libc::c_float
                {
                    v * height as libc::c_float
                } else {
                    (height - 1 as libc::c_int) as libc::c_float
                })
            {
                0 as libc::c_int as libc::c_float
            } else if (height - 1 as libc::c_int) as libc::c_float
                > v * height as libc::c_float
            {
                v * height as libc::c_float
            } else {
                (height - 1 as libc::c_int) as libc::c_float
            }) as libc::c_int;
            src = heman_image_texel(img, i_0, j);
            let mut n: libc::c_int = 0 as libc::c_int;
            while n < nbands {
                let fresh14 = src;
                src = src.offset(1);
                let fresh15 = dst;
                dst = dst.offset(1);
                *fresh15 = *fresh14;
                n += 1;
                n;
            }
            if !secondary.is_null() {
                src = heman_image_texel(secondary, x, y);
                let mut dst2: *mut libc::c_float = heman_image_texel(result2, i_0, j);
                let mut n_0: libc::c_int = 0 as libc::c_int;
                while n_0 < (*secondary).nbands {
                    let fresh16 = src;
                    src = src.offset(1);
                    let fresh17 = dst2;
                    dst2 = dst2.offset(1);
                    *fresh17 = *fresh16;
                    n_0 += 1;
                    n_0;
                }
            }
            x += 1;
            x;
        }
        y += 1;
        y;
    }
    open_simplex_noise_free(ctx);
    if !secondary.is_null() {
        free((*secondary).data as *mut libc::c_void);
        (*secondary).data = (*result2).data;
        free(result2 as *mut libc::c_void);
    }
    return result;
}
pub unsafe extern "C" fn heman_ops_warp_points(
    mut img: *mut heman_image,
    mut seed: libc::c_int,
    mut octaves: libc::c_int,
    mut pts: *mut heman_points,
) -> *mut heman_image {
    let mut width: libc::c_int = (*img).width;
    let mut height: libc::c_int = (*img).height;
    let mut mapping: *mut heman_image = heman_distance_identity_cpcf(width, height);
    let mut retval: *mut heman_image = heman_ops_warp_core(img, mapping, seed, octaves);
    let mut src: *mut libc::c_float = (*pts).data;
    let mut k: libc::c_int = 0 as libc::c_int;
    while k < (*pts).width {
        let mut x: libc::c_float = *src.offset(0 as libc::c_int as isize);
        let mut y: libc::c_float = *src.offset(1 as libc::c_int as isize);
        let mut i: libc::c_int = (x * (*mapping).width as libc::c_float) as libc::c_int;
        let mut j: libc::c_int = (y * (*mapping).height as libc::c_float) as libc::c_int;
        if !(i < 0 as libc::c_int || i >= (*mapping).width || j < 0 as libc::c_int
            || j >= (*mapping).height)
        {
            let mut texel: *mut libc::c_float = heman_image_texel(mapping, i, j);
            *src
                .offset(
                    0 as libc::c_int as isize,
                ) = *texel.offset(0 as libc::c_int as isize)
                / (*mapping).width as libc::c_float;
            *src
                .offset(
                    1 as libc::c_int as isize,
                ) = *texel.offset(1 as libc::c_int as isize)
                / (*mapping).height as libc::c_float;
        }
        k += 1;
        k;
        src = src.offset((*pts).nbands as isize);
    }
    heman_image_destroy(mapping);
    return retval;
}
pub unsafe extern "C" fn heman_ops_warp(
    mut img: *mut heman_image,
    mut seed: libc::c_int,
    mut octaves: libc::c_int,
) -> *mut heman_image {
    return heman_ops_warp_core(img, 0 as *mut heman_image, seed, octaves);
}
pub unsafe extern "C" fn heman_ops_extract_mask(
    mut source: *mut heman_image,
    mut color: heman_color,
    mut invert: libc::c_int,
) -> *mut heman_image {
    if (*source).nbands == 3 as libc::c_int {} else {
        __assert_fail(
            b"source->nbands == 3\0" as *const u8 as *const libc::c_char,
            b"src/ops.c\0" as *const u8 as *const libc::c_char,
            330 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 69],
                &[libc::c_char; 69],
            >(b"heman_image *heman_ops_extract_mask(heman_image *, heman_color, int)\0"))
                .as_ptr(),
        );
    }
    'c_4456: {
        if (*source).nbands == 3 as libc::c_int {} else {
            __assert_fail(
                b"source->nbands == 3\0" as *const u8 as *const libc::c_char,
                b"src/ops.c\0" as *const u8 as *const libc::c_char,
                330 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 69],
                    &[libc::c_char; 69],
                >(
                    b"heman_image *heman_ops_extract_mask(heman_image *, heman_color, int)\0",
                ))
                    .as_ptr(),
            );
        }
    };
    let mut inv: libc::c_float = 1.0f32 / 255.0f32;
    let mut r: libc::c_float = (color >> 16 as libc::c_int) as libc::c_float * inv;
    let mut g: libc::c_float = (color >> 8 as libc::c_int
        & 0xff as libc::c_int as libc::c_uint) as libc::c_float * inv;
    let mut b: libc::c_float = (color & 0xff as libc::c_int as libc::c_uint)
        as libc::c_float * inv;
    let mut height: libc::c_int = (*source).height;
    let mut width: libc::c_int = (*source).width;
    let mut result: *mut heman_image = heman_image_create(
        width,
        height,
        1 as libc::c_int,
    );
    let mut y: libc::c_int = 0;
    y = 0 as libc::c_int;
    while y < height {
        let mut dst: *mut libc::c_float = ((*result).data).offset((y * width) as isize);
        let mut src: *mut libc::c_float = ((*source).data)
            .offset((y * width * 3 as libc::c_int) as isize);
        let mut x: libc::c_int = 0 as libc::c_int;
        while x < width {
            let mut val: libc::c_float = (*src.offset(0 as libc::c_int as isize) == r
                && *src.offset(1 as libc::c_int as isize) == g
                && *src.offset(2 as libc::c_int as isize) == b) as libc::c_int
                as libc::c_float;
            if invert == 0 {
                val = 1 as libc::c_int as libc::c_float - val;
            }
            let fresh18 = dst;
            dst = dst.offset(1);
            *fresh18 = val;
            x += 1;
            x;
            src = src.offset(3 as libc::c_int as isize);
        }
        y += 1;
        y;
    }
    return result;
}
pub unsafe extern "C" fn heman_ops_replace_color(
    mut source: *mut heman_image,
    mut color: heman_color,
    mut texture: *mut heman_image,
) -> *mut heman_image {
    if (*source).nbands == 3 as libc::c_int {} else {
        __assert_fail(
            b"source->nbands == 3\0" as *const u8 as *const libc::c_char,
            b"src/ops.c\0" as *const u8 as *const libc::c_char,
            359 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 80],
                &[libc::c_char; 80],
            >(
                b"heman_image *heman_ops_replace_color(heman_image *, heman_color, heman_image *)\0",
            ))
                .as_ptr(),
        );
    }
    'c_4882: {
        if (*source).nbands == 3 as libc::c_int {} else {
            __assert_fail(
                b"source->nbands == 3\0" as *const u8 as *const libc::c_char,
                b"src/ops.c\0" as *const u8 as *const libc::c_char,
                359 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 80],
                    &[libc::c_char; 80],
                >(
                    b"heman_image *heman_ops_replace_color(heman_image *, heman_color, heman_image *)\0",
                ))
                    .as_ptr(),
            );
        }
    };
    if (*texture).nbands == 3 as libc::c_int {} else {
        __assert_fail(
            b"texture->nbands == 3\0" as *const u8 as *const libc::c_char,
            b"src/ops.c\0" as *const u8 as *const libc::c_char,
            360 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 80],
                &[libc::c_char; 80],
            >(
                b"heman_image *heman_ops_replace_color(heman_image *, heman_color, heman_image *)\0",
            ))
                .as_ptr(),
        );
    }
    'c_4841: {
        if (*texture).nbands == 3 as libc::c_int {} else {
            __assert_fail(
                b"texture->nbands == 3\0" as *const u8 as *const libc::c_char,
                b"src/ops.c\0" as *const u8 as *const libc::c_char,
                360 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 80],
                    &[libc::c_char; 80],
                >(
                    b"heman_image *heman_ops_replace_color(heman_image *, heman_color, heman_image *)\0",
                ))
                    .as_ptr(),
            );
        }
    };
    let mut height: libc::c_int = (*source).height;
    let mut width: libc::c_int = (*source).width;
    if (*texture).width == width {} else {
        __assert_fail(
            b"texture->width == width\0" as *const u8 as *const libc::c_char,
            b"src/ops.c\0" as *const u8 as *const libc::c_char,
            363 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 80],
                &[libc::c_char; 80],
            >(
                b"heman_image *heman_ops_replace_color(heman_image *, heman_color, heman_image *)\0",
            ))
                .as_ptr(),
        );
    }
    'c_4798: {
        if (*texture).width == width {} else {
            __assert_fail(
                b"texture->width == width\0" as *const u8 as *const libc::c_char,
                b"src/ops.c\0" as *const u8 as *const libc::c_char,
                363 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 80],
                    &[libc::c_char; 80],
                >(
                    b"heman_image *heman_ops_replace_color(heman_image *, heman_color, heman_image *)\0",
                ))
                    .as_ptr(),
            );
        }
    };
    if (*texture).height == height {} else {
        __assert_fail(
            b"texture->height == height\0" as *const u8 as *const libc::c_char,
            b"src/ops.c\0" as *const u8 as *const libc::c_char,
            364 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 80],
                &[libc::c_char; 80],
            >(
                b"heman_image *heman_ops_replace_color(heman_image *, heman_color, heman_image *)\0",
            ))
                .as_ptr(),
        );
    }
    'c_4755: {
        if (*texture).height == height {} else {
            __assert_fail(
                b"texture->height == height\0" as *const u8 as *const libc::c_char,
                b"src/ops.c\0" as *const u8 as *const libc::c_char,
                364 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 80],
                    &[libc::c_char; 80],
                >(
                    b"heman_image *heman_ops_replace_color(heman_image *, heman_color, heman_image *)\0",
                ))
                    .as_ptr(),
            );
        }
    };
    let mut inv: libc::c_float = 1.0f32 / 255.0f32;
    let mut r: libc::c_float = (color >> 16 as libc::c_int) as libc::c_float * inv;
    let mut g: libc::c_float = (color >> 8 as libc::c_int
        & 0xff as libc::c_int as libc::c_uint) as libc::c_float * inv;
    let mut b: libc::c_float = (color & 0xff as libc::c_int as libc::c_uint)
        as libc::c_float * inv;
    let mut result: *mut heman_image = heman_image_create(
        width,
        height,
        3 as libc::c_int,
    );
    let mut y: libc::c_int = 0;
    y = 0 as libc::c_int;
    while y < height {
        let mut dst: *mut libc::c_float = ((*result).data)
            .offset((y * width * 3 as libc::c_int) as isize);
        let mut src: *mut libc::c_float = ((*source).data)
            .offset((y * width * 3 as libc::c_int) as isize);
        let mut tex: *mut libc::c_float = ((*texture).data)
            .offset((y * width * 3 as libc::c_int) as isize);
        let mut x: libc::c_int = 0 as libc::c_int;
        while x < width {
            if *src.offset(0 as libc::c_int as isize) == r
                && *src.offset(1 as libc::c_int as isize) == g
                && *src.offset(2 as libc::c_int as isize) == b
            {
                *dst
                    .offset(
                        0 as libc::c_int as isize,
                    ) = *tex.offset(0 as libc::c_int as isize);
                *dst
                    .offset(
                        1 as libc::c_int as isize,
                    ) = *tex.offset(1 as libc::c_int as isize);
                *dst
                    .offset(
                        2 as libc::c_int as isize,
                    ) = *tex.offset(2 as libc::c_int as isize);
            } else {
                *dst
                    .offset(
                        0 as libc::c_int as isize,
                    ) = *src.offset(0 as libc::c_int as isize);
                *dst
                    .offset(
                        1 as libc::c_int as isize,
                    ) = *src.offset(1 as libc::c_int as isize);
                *dst
                    .offset(
                        2 as libc::c_int as isize,
                    ) = *src.offset(2 as libc::c_int as isize);
            }
            x += 1;
            x;
            src = src.offset(3 as libc::c_int as isize);
            dst = dst.offset(3 as libc::c_int as isize);
            tex = tex.offset(3 as libc::c_int as isize);
        }
        y += 1;
        y;
    }
    return result;
}
unsafe extern "C" fn _match(
    mut mask: *mut heman_image,
    mut mask_color: heman_color,
    mut invert_mask: libc::c_int,
    mut pixel_index: libc::c_int,
) -> libc::c_int {
    let mut mcolor: *mut libc::c_float = ((*mask).data)
        .offset((pixel_index * 3 as libc::c_int) as isize);
    let mut r1: libc::c_uchar = (*mcolor.offset(0 as libc::c_int as isize)
        * 255 as libc::c_int as libc::c_float) as libc::c_uchar;
    let mut g1: libc::c_uchar = (*mcolor.offset(1 as libc::c_int as isize)
        * 255 as libc::c_int as libc::c_float) as libc::c_uchar;
    let mut b1: libc::c_uchar = (*mcolor.offset(2 as libc::c_int as isize)
        * 255 as libc::c_int as libc::c_float) as libc::c_uchar;
    let mut r2: libc::c_uchar = (mask_color >> 16 as libc::c_int) as libc::c_uchar;
    let mut g2: libc::c_uchar = (mask_color >> 8 as libc::c_int
        & 0xff as libc::c_int as libc::c_uint) as libc::c_uchar;
    let mut b2: libc::c_uchar = (mask_color & 0xff as libc::c_int as libc::c_uint)
        as libc::c_uchar;
    let mut retval: libc::c_int = (r1 as libc::c_int == r2 as libc::c_int
        && g1 as libc::c_int == g2 as libc::c_int
        && b1 as libc::c_int == b2 as libc::c_int) as libc::c_int;
    return if invert_mask != 0 { 1 as libc::c_int - retval } else { retval };
}
unsafe extern "C" fn qselect(
    mut v: *mut libc::c_float,
    mut len: libc::c_int,
    mut k: libc::c_int,
) -> libc::c_float {
    let mut i: libc::c_int = 0;
    let mut st: libc::c_int = 0;
    i = 0 as libc::c_int;
    st = i;
    while i < len - 1 as libc::c_int {
        if !(*v.offset(i as isize) > *v.offset((len - 1 as libc::c_int) as isize)) {
            let mut _t: libc::c_float = *v.offset(i as isize);
            *v.offset(i as isize) = *v.offset(st as isize);
            *v.offset(st as isize) = _t;
            st += 1;
            st;
        }
        i += 1;
        i;
    }
    let mut __0: libc::c_float = *v.offset((len - 1 as libc::c_int) as isize);
    *v.offset((len - 1 as libc::c_int) as isize) = *v.offset(st as isize);
    *v.offset(st as isize) = __0;
    return if k == st {
        *v.offset(st as isize)
    } else if st > k {
        qselect(v, st, k)
    } else {
        qselect(v.offset(st as isize), len - st, k - st)
    };
}
pub unsafe extern "C" fn heman_ops_percentiles(
    mut hmap: *mut heman_image,
    mut nsteps: libc::c_int,
    mut mask: *mut heman_image,
    mut mask_color: heman_color,
    mut invert_mask: libc::c_int,
    mut offset: libc::c_float,
) -> *mut heman_image {
    if (*hmap).nbands == 1 as libc::c_int {} else {
        __assert_fail(
            b"hmap->nbands == 1\0" as *const u8 as *const libc::c_char,
            b"src/ops.c\0" as *const u8 as *const libc::c_char,
            427 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 95],
                &[libc::c_char; 95],
            >(
                b"heman_image *heman_ops_percentiles(heman_image *, int, heman_image *, heman_color, int, float)\0",
            ))
                .as_ptr(),
        );
    }
    'c_2409: {
        if (*hmap).nbands == 1 as libc::c_int {} else {
            __assert_fail(
                b"hmap->nbands == 1\0" as *const u8 as *const libc::c_char,
                b"src/ops.c\0" as *const u8 as *const libc::c_char,
                427 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 95],
                    &[libc::c_char; 95],
                >(
                    b"heman_image *heman_ops_percentiles(heman_image *, int, heman_image *, heman_color, int, float)\0",
                ))
                    .as_ptr(),
            );
        }
    };
    if mask.is_null() || (*mask).nbands == 3 as libc::c_int {} else {
        __assert_fail(
            b"!mask || mask->nbands == 3\0" as *const u8 as *const libc::c_char,
            b"src/ops.c\0" as *const u8 as *const libc::c_char,
            428 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 95],
                &[libc::c_char; 95],
            >(
                b"heman_image *heman_ops_percentiles(heman_image *, int, heman_image *, heman_color, int, float)\0",
            ))
                .as_ptr(),
        );
    }
    'c_2360: {
        if mask.is_null() || (*mask).nbands == 3 as libc::c_int {} else {
            __assert_fail(
                b"!mask || mask->nbands == 3\0" as *const u8 as *const libc::c_char,
                b"src/ops.c\0" as *const u8 as *const libc::c_char,
                428 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 95],
                    &[libc::c_char; 95],
                >(
                    b"heman_image *heman_ops_percentiles(heman_image *, int, heman_image *, heman_color, int, float)\0",
                ))
                    .as_ptr(),
            );
        }
    };
    let mut size: libc::c_int = (*hmap).height * (*hmap).width;
    let mut src: *mut libc::c_float = (*hmap).data;
    let mut minv: libc::c_float = 1000 as libc::c_int as libc::c_float;
    let mut maxv: libc::c_float = -(1000 as libc::c_int) as libc::c_float;
    let mut npixels: libc::c_int = 0 as libc::c_int;
    let mut i: libc::c_int = 0 as libc::c_int;
    while i < size {
        if mask.is_null() || _match(mask, mask_color, invert_mask, i) != 0 {
            minv = if minv > *src.offset(i as isize) {
                *src.offset(i as isize)
            } else {
                minv
            };
            maxv = if maxv > *src.offset(i as isize) {
                maxv
            } else {
                *src.offset(i as isize)
            };
            npixels += 1;
            npixels;
        }
        i += 1;
        i;
    }
    let mut vals: *mut libc::c_float = malloc(
        (::std::mem::size_of::<libc::c_float>() as libc::c_ulong)
            .wrapping_mul(npixels as libc::c_ulong),
    ) as *mut libc::c_float;
    npixels = 0 as libc::c_int;
    let mut i_0: libc::c_int = 0 as libc::c_int;
    while i_0 < size {
        if mask.is_null() || _match(mask, mask_color, invert_mask, i_0) != 0 {
            let fresh19 = npixels;
            npixels = npixels + 1;
            *vals.offset(fresh19 as isize) = *src.offset(i_0 as isize);
        }
        i_0 += 1;
        i_0;
    }
    let mut percentiles: *mut libc::c_float = malloc(
        (::std::mem::size_of::<libc::c_float>() as libc::c_ulong)
            .wrapping_mul(nsteps as libc::c_ulong),
    ) as *mut libc::c_float;
    let mut tier: libc::c_int = 0 as libc::c_int;
    while tier < nsteps {
        let mut height: libc::c_float = qselect(vals, npixels, tier * npixels / nsteps);
        *percentiles.offset(tier as isize) = height;
        tier += 1;
        tier;
    }
    free(vals as *mut libc::c_void);
    let mut i_1: libc::c_int = 0 as libc::c_int;
    while i_1 < size {
        let mut e: libc::c_float = *src;
        if mask.is_null() || _match(mask, mask_color, invert_mask, i_1) != 0 {
            let mut tier_0: libc::c_int = nsteps - 1 as libc::c_int;
            while tier_0 >= 0 as libc::c_int {
                if e > *percentiles.offset(tier_0 as isize) {
                    e = *percentiles.offset(tier_0 as isize);
                    break;
                } else {
                    tier_0 -= 1;
                    tier_0;
                }
            }
        }
        let fresh20 = src;
        src = src.offset(1);
        *fresh20 = e + offset;
        i_1 += 1;
        i_1;
    }
    free(percentiles as *mut libc::c_void);
    return hmap;
}
pub unsafe extern "C" fn heman_ops_stairstep(
    mut hmap: *mut heman_image,
    mut nsteps: libc::c_int,
    mut mask: *mut heman_image,
    mut mask_color: heman_color,
    mut invert_mask: libc::c_int,
    mut offset: libc::c_float,
) -> *mut heman_image {
    if (*hmap).nbands == 1 as libc::c_int {} else {
        __assert_fail(
            b"hmap->nbands == 1\0" as *const u8 as *const libc::c_char,
            b"src/ops.c\0" as *const u8 as *const libc::c_char,
            477 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 93],
                &[libc::c_char; 93],
            >(
                b"heman_image *heman_ops_stairstep(heman_image *, int, heman_image *, heman_color, int, float)\0",
            ))
                .as_ptr(),
        );
    }
    'c_1832: {
        if (*hmap).nbands == 1 as libc::c_int {} else {
            __assert_fail(
                b"hmap->nbands == 1\0" as *const u8 as *const libc::c_char,
                b"src/ops.c\0" as *const u8 as *const libc::c_char,
                477 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 93],
                    &[libc::c_char; 93],
                >(
                    b"heman_image *heman_ops_stairstep(heman_image *, int, heman_image *, heman_color, int, float)\0",
                ))
                    .as_ptr(),
            );
        }
    };
    if mask.is_null() || (*mask).nbands == 3 as libc::c_int {} else {
        __assert_fail(
            b"!mask || mask->nbands == 3\0" as *const u8 as *const libc::c_char,
            b"src/ops.c\0" as *const u8 as *const libc::c_char,
            478 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 93],
                &[libc::c_char; 93],
            >(
                b"heman_image *heman_ops_stairstep(heman_image *, int, heman_image *, heman_color, int, float)\0",
            ))
                .as_ptr(),
        );
    }
    'c_1783: {
        if mask.is_null() || (*mask).nbands == 3 as libc::c_int {} else {
            __assert_fail(
                b"!mask || mask->nbands == 3\0" as *const u8 as *const libc::c_char,
                b"src/ops.c\0" as *const u8 as *const libc::c_char,
                478 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 93],
                    &[libc::c_char; 93],
                >(
                    b"heman_image *heman_ops_stairstep(heman_image *, int, heman_image *, heman_color, int, float)\0",
                ))
                    .as_ptr(),
            );
        }
    };
    let mut size: libc::c_int = (*hmap).height * (*hmap).width;
    let mut src: *mut libc::c_float = (*hmap).data;
    let mut minv: libc::c_float = 1000 as libc::c_int as libc::c_float;
    let mut maxv: libc::c_float = -(1000 as libc::c_int) as libc::c_float;
    let mut i: libc::c_int = 0 as libc::c_int;
    while i < size {
        if mask.is_null() || _match(mask, mask_color, invert_mask, i) != 0 {
            minv = if minv > *src.offset(i as isize) {
                *src.offset(i as isize)
            } else {
                minv
            };
            maxv = if maxv > *src.offset(i as isize) {
                maxv
            } else {
                *src.offset(i as isize)
            };
        }
        i += 1;
        i;
    }
    let mut range: libc::c_float = maxv - minv;
    let mut i_0: libc::c_int = 0 as libc::c_int;
    while i_0 < size {
        let mut e: libc::c_float = *src;
        if mask.is_null() || _match(mask, mask_color, invert_mask, i_0) != 0 {
            e = e - minv;
            e /= range;
            e = (floor((e * nsteps as libc::c_float) as libc::c_double)
                / nsteps as libc::c_double) as libc::c_float;
            e = e * range + minv;
        }
        let fresh21 = src;
        src = src.offset(1);
        *fresh21 = e + offset;
        i_0 += 1;
        i_0;
    }
    return hmap;
}
pub unsafe extern "C" fn heman_ops_merge_political(
    mut hmap: *mut heman_image,
    mut cmap: *mut heman_image,
    mut ocean: heman_color,
) -> *mut heman_image {
    if (*hmap).nbands == 1 as libc::c_int {} else {
        __assert_fail(
            b"hmap->nbands == 1\0" as *const u8 as *const libc::c_char,
            b"src/ops.c\0" as *const u8 as *const libc::c_char,
            506 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 82],
                &[libc::c_char; 82],
            >(
                b"heman_image *heman_ops_merge_political(heman_image *, heman_image *, heman_color)\0",
            ))
                .as_ptr(),
        );
    }
    'c_5221: {
        if (*hmap).nbands == 1 as libc::c_int {} else {
            __assert_fail(
                b"hmap->nbands == 1\0" as *const u8 as *const libc::c_char,
                b"src/ops.c\0" as *const u8 as *const libc::c_char,
                506 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 82],
                    &[libc::c_char; 82],
                >(
                    b"heman_image *heman_ops_merge_political(heman_image *, heman_image *, heman_color)\0",
                ))
                    .as_ptr(),
            );
        }
    };
    if (*cmap).nbands == 3 as libc::c_int {} else {
        __assert_fail(
            b"cmap->nbands == 3\0" as *const u8 as *const libc::c_char,
            b"src/ops.c\0" as *const u8 as *const libc::c_char,
            507 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 82],
                &[libc::c_char; 82],
            >(
                b"heman_image *heman_ops_merge_political(heman_image *, heman_image *, heman_color)\0",
            ))
                .as_ptr(),
        );
    }
    'c_5180: {
        if (*cmap).nbands == 3 as libc::c_int {} else {
            __assert_fail(
                b"cmap->nbands == 3\0" as *const u8 as *const libc::c_char,
                b"src/ops.c\0" as *const u8 as *const libc::c_char,
                507 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 82],
                    &[libc::c_char; 82],
                >(
                    b"heman_image *heman_ops_merge_political(heman_image *, heman_image *, heman_color)\0",
                ))
                    .as_ptr(),
            );
        }
    };
    let mut result: *mut heman_image = heman_image_create(
        (*hmap).width,
        (*hmap).height,
        4 as libc::c_int,
    );
    let mut pheight: *mut libc::c_float = (*hmap).data;
    let mut pcolour: *mut libc::c_float = (*cmap).data;
    let mut pmerged: *mut libc::c_float = (*result).data;
    let mut inv: libc::c_float = 1.0f32 / 255.0f32;
    let mut oceanr: libc::c_float = (ocean >> 16 as libc::c_int) as libc::c_float * inv;
    let mut oceang: libc::c_float = (ocean >> 8 as libc::c_int
        & 0xff as libc::c_int as libc::c_uint) as libc::c_float * inv;
    let mut oceanb: libc::c_float = (ocean & 0xff as libc::c_int as libc::c_uint)
        as libc::c_float * inv;
    let mut size: libc::c_int = (*hmap).height * (*hmap).width;
    let mut minh: libc::c_float = 1000 as libc::c_int as libc::c_float;
    let mut maxh: libc::c_float = -(1000 as libc::c_int) as libc::c_float;
    let mut i: libc::c_int = 0 as libc::c_int;
    while i < size {
        minh = if minh > *pheight.offset(i as isize) {
            *pheight.offset(i as isize)
        } else {
            minh
        };
        maxh = if maxh > *pheight.offset(i as isize) {
            *pheight.offset(i as isize)
        } else {
            maxh
        };
        i += 1;
        i;
    }
    let mut i_0: libc::c_int = 0 as libc::c_int;
    while i_0 < size {
        let fresh22 = pheight;
        pheight = pheight.offset(1);
        let mut h: libc::c_float = *fresh22;
        if h < 0 as libc::c_int as libc::c_float {
            let fresh23 = pmerged;
            pmerged = pmerged.offset(1);
            *fresh23 = oceanr;
            let fresh24 = pmerged;
            pmerged = pmerged.offset(1);
            *fresh24 = oceang;
            let fresh25 = pmerged;
            pmerged = pmerged.offset(1);
            *fresh25 = oceanb;
            pcolour = pcolour.offset(3 as libc::c_int as isize);
        } else {
            let fresh26 = pcolour;
            pcolour = pcolour.offset(1);
            let fresh27 = pmerged;
            pmerged = pmerged.offset(1);
            *fresh27 = *fresh26;
            let fresh28 = pcolour;
            pcolour = pcolour.offset(1);
            let fresh29 = pmerged;
            pmerged = pmerged.offset(1);
            *fresh29 = *fresh28;
            let fresh30 = pcolour;
            pcolour = pcolour.offset(1);
            let fresh31 = pmerged;
            pmerged = pmerged.offset(1);
            *fresh31 = *fresh30;
        }
        let fresh32 = pmerged;
        pmerged = pmerged.offset(1);
        *fresh32 = (h - minh) / (maxh - minh);
        i_0 += 1;
        i_0;
    }
    return result;
}
pub unsafe extern "C" fn heman_ops_emboss(
    mut img: *mut heman_image,
    mut mode: libc::c_int,
) -> *mut heman_image {
    let mut seed: libc::c_int = 1 as libc::c_int;
    let mut octaves: libc::c_int = 4 as libc::c_int;
    let mut ctx: *mut osn_context = 0 as *mut osn_context;
    open_simplex_noise(seed as int64_t, &mut ctx);
    let mut width: libc::c_int = (*img).width;
    let mut height: libc::c_int = (*img).height;
    if (*img).nbands == 1 as libc::c_int {} else {
        __assert_fail(
            b"img->nbands == 1\0" as *const u8 as *const libc::c_char,
            b"src/ops.c\0" as *const u8 as *const libc::c_char,
            549 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 50],
                &[libc::c_char; 50],
            >(b"heman_image *heman_ops_emboss(heman_image *, int)\0"))
                .as_ptr(),
        );
    }
    'c_5616: {
        if (*img).nbands == 1 as libc::c_int {} else {
            __assert_fail(
                b"img->nbands == 1\0" as *const u8 as *const libc::c_char,
                b"src/ops.c\0" as *const u8 as *const libc::c_char,
                549 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 50],
                    &[libc::c_char; 50],
                >(b"heman_image *heman_ops_emboss(heman_image *, int)\0"))
                    .as_ptr(),
            );
        }
    };
    let mut result: *mut heman_image = heman_image_create(
        width,
        height,
        1 as libc::c_int,
    );
    let mut invw: libc::c_float = (1.0f64 / width as libc::c_double) as libc::c_float;
    let mut invh: libc::c_float = (1.0f64 / height as libc::c_double) as libc::c_float;
    let mut inv: libc::c_float = if invw > invh { invh } else { invw };
    let mut gain: libc::c_float = 0.6f64 as libc::c_float;
    let mut lacunarity: libc::c_float = 2.0f64 as libc::c_float;
    let mut land_amplitude: libc::c_float = 0.0005f64 as libc::c_float;
    let mut land_frequency: libc::c_float = 256.0f64 as libc::c_float;
    let mut ocean_amplitude: libc::c_float = 0.5f64 as libc::c_float;
    let mut ocean_frequency: libc::c_float = 1.0f64 as libc::c_float;
    let mut y: libc::c_int = 0;
    y = 0 as libc::c_int;
    while y < height {
        let mut dst: *mut libc::c_float = ((*result).data).offset((y * width) as isize);
        let mut x: libc::c_int = 0 as libc::c_int;
        while x < width {
            let mut z: libc::c_float = *heman_image_texel(img, x, y);
            if z > 0 as libc::c_int as libc::c_float && mode == 1 as libc::c_int {
                let mut s: libc::c_float = x as libc::c_float * inv;
                let mut t: libc::c_float = y as libc::c_float * inv;
                let mut a: libc::c_float = land_amplitude;
                let mut f: libc::c_float = land_frequency;
                let mut i: libc::c_int = 0 as libc::c_int;
                while i < octaves {
                    z = (z as libc::c_double
                        + a as libc::c_double
                            * open_simplex_noise2(
                                ctx,
                                (s * f) as libc::c_double,
                                (t * f) as libc::c_double,
                            )) as libc::c_float;
                    a *= gain;
                    f *= lacunarity;
                    i += 1;
                    i;
                }
            } else if z <= 0 as libc::c_int as libc::c_float
                && mode == -(1 as libc::c_int)
            {
                z = (if z as libc::c_double > -0.1f64 {
                    z as libc::c_double
                } else {
                    -0.1f64
                }) as libc::c_float;
                let mut soften: libc::c_float = fabsf(z);
                let mut s_0: libc::c_float = x as libc::c_float * inv;
                let mut t_0: libc::c_float = y as libc::c_float * inv;
                let mut a_0: libc::c_float = ocean_amplitude;
                let mut f_0: libc::c_float = ocean_frequency;
                let mut i_0: libc::c_int = 0 as libc::c_int;
                while i_0 < octaves {
                    z = (z as libc::c_double
                        + soften as libc::c_double
                            * (a_0 as libc::c_double
                                * open_simplex_noise2(
                                    ctx,
                                    (s_0 * f_0) as libc::c_double,
                                    (t_0 * f_0) as libc::c_double,
                                ))) as libc::c_float;
                    a_0 *= gain;
                    f_0 *= lacunarity;
                    i_0 += 1;
                    i_0;
                }
            }
            let fresh33 = dst;
            dst = dst.offset(1);
            *fresh33 = z;
            x += 1;
            x;
        }
        y += 1;
        y;
    }
    open_simplex_noise_free(ctx);
    return result;
}
