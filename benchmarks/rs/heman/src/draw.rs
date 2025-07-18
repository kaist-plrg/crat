use ::libc;
extern "C" {
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
    fn heman_image_clear(_: *mut heman_image, value: libc::c_float);
    fn heman_points_destroy(_: *mut heman_points);
    fn generate_gaussian_splat(target: *mut libc::c_float, fwidth: libc::c_int);
    fn __assert_fail(
        __assertion: *const libc::c_char,
        __file: *const libc::c_char,
        __line: libc::c_uint,
        __function: *const libc::c_char,
    ) -> !;
    fn free(__ptr: *mut libc::c_void);
    fn malloc(_: libc::c_ulong) -> *mut libc::c_void;
    fn heman_internal_draw_seeds(
        target: *mut heman_image,
        pts: *mut heman_points,
        filterd: libc::c_int,
    );
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
pub unsafe extern "C" fn heman_draw_points(
    mut target: *mut heman_image,
    mut pts: *mut heman_points,
    mut val: libc::c_float,
) {
    let mut src: *mut libc::c_float = (*pts).data;
    let mut k: libc::c_int = 0 as libc::c_int;
    while k < (*pts).width {
        let mut x: libc::c_float = *src.offset(0 as libc::c_int as isize);
        let mut y: libc::c_float = *src.offset(1 as libc::c_int as isize);
        src = src.offset((*pts).nbands as isize);
        let mut i: libc::c_int = (x * (*target).width as libc::c_float) as libc::c_int;
        let mut j: libc::c_int = (y * (*target).height as libc::c_float) as libc::c_int;
        if !(i < 0 as libc::c_int || i >= (*target).width || j < 0 as libc::c_int
            || j >= (*target).height)
        {
            let mut texel: *mut libc::c_float = heman_image_texel(target, i, j);
            let mut c: libc::c_int = 0 as libc::c_int;
            while c < (*target).nbands {
                let fresh0 = texel;
                texel = texel.offset(1);
                *fresh0 = val;
                c += 1;
                c;
            }
        }
        k += 1;
        k;
    }
}
pub unsafe extern "C" fn heman_draw_colored_points(
    mut target: *mut heman_image,
    mut pts: *mut heman_points,
    mut colors: *const heman_color,
) {
    if (*target).nbands == 3 as libc::c_int || (*target).nbands == 4 as libc::c_int
    {} else {
        __assert_fail(
            b"target->nbands == 3 || target->nbands == 4\0" as *const u8
                as *const libc::c_char,
            b"src/draw.c\0" as *const u8 as *const libc::c_char,
            27 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 83],
                &[libc::c_char; 83],
            >(
                b"void heman_draw_colored_points(heman_image *, heman_points *, const heman_color *)\0",
            ))
                .as_ptr(),
        );
    }
    'c_642: {
        if (*target).nbands == 3 as libc::c_int || (*target).nbands == 4 as libc::c_int
        {} else {
            __assert_fail(
                b"target->nbands == 3 || target->nbands == 4\0" as *const u8
                    as *const libc::c_char,
                b"src/draw.c\0" as *const u8 as *const libc::c_char,
                27 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 83],
                    &[libc::c_char; 83],
                >(
                    b"void heman_draw_colored_points(heman_image *, heman_points *, const heman_color *)\0",
                ))
                    .as_ptr(),
            );
        }
    };
    let mut src: *mut libc::c_float = (*pts).data;
    let mut inv: libc::c_float = 1.0f32 / 255.0f32;
    let mut k: libc::c_int = 0 as libc::c_int;
    while k < (*pts).width {
        let mut x: libc::c_float = *src.offset(0 as libc::c_int as isize);
        let mut y: libc::c_float = *src.offset(1 as libc::c_int as isize);
        src = src.offset((*pts).nbands as isize);
        let mut i: libc::c_int = (x * (*target).width as libc::c_float) as libc::c_int;
        let mut j: libc::c_int = (y * (*target).height as libc::c_float) as libc::c_int;
        if !(i < 0 as libc::c_int || i >= (*target).width || j < 0 as libc::c_int
            || j >= (*target).height)
        {
            let mut texel: *mut libc::c_float = heman_image_texel(target, i, j);
            let mut rgb: heman_color = *colors.offset(k as isize);
            let fresh1 = texel;
            texel = texel.offset(1);
            *fresh1 = (rgb >> 16 as libc::c_int & 0xff as libc::c_int as libc::c_uint)
                as libc::c_float * inv;
            let fresh2 = texel;
            texel = texel.offset(1);
            *fresh2 = (rgb >> 8 as libc::c_int & 0xff as libc::c_int as libc::c_uint)
                as libc::c_float * inv;
            let fresh3 = texel;
            texel = texel.offset(1);
            *fresh3 = (rgb & 0xff as libc::c_int as libc::c_uint) as libc::c_float * inv;
            if (*target).nbands == 4 as libc::c_int {
                *texel = (rgb >> 24 as libc::c_int) as libc::c_float * inv;
            }
        }
        k += 1;
        k;
    }
}
pub unsafe extern "C" fn heman_draw_colored_circles(
    mut target: *mut heman_image,
    mut pts: *mut heman_points,
    mut radius: libc::c_int,
    mut colors: *const heman_color,
) {
    let mut fwidth: libc::c_int = radius * 2 as libc::c_int + 1 as libc::c_int;
    let mut radius2: libc::c_int = radius * radius;
    let mut src: *mut libc::c_float = (*pts).data;
    let mut inv: libc::c_float = 1.0f32 / 255.0f32;
    let mut w: libc::c_int = (*target).width;
    let mut h: libc::c_int = (*target).height;
    let mut k: libc::c_int = 0 as libc::c_int;
    while k < (*pts).width {
        let mut x: libc::c_float = *src.offset(0 as libc::c_int as isize);
        let mut y: libc::c_float = *src.offset(1 as libc::c_int as isize);
        src = src.offset((*pts).nbands as isize);
        let mut ii: libc::c_int = (x * w as libc::c_float - radius as libc::c_float)
            as libc::c_int;
        let mut jj: libc::c_int = (y * h as libc::c_float - radius as libc::c_float)
            as libc::c_int;
        let mut kj: libc::c_int = 0 as libc::c_int;
        while kj < fwidth {
            let mut ki: libc::c_int = 0 as libc::c_int;
            while ki < fwidth {
                let mut i: libc::c_int = ii + ki;
                let mut j: libc::c_int = jj + kj;
                let mut r2: libc::c_int = ((i as libc::c_float - x * w as libc::c_float)
                    * (i as libc::c_float - x * w as libc::c_float)
                    + (j as libc::c_float - y * h as libc::c_float)
                        * (j as libc::c_float - y * h as libc::c_float)) as libc::c_int;
                if !(r2 > radius2) {
                    let mut texel: *mut libc::c_float = heman_image_texel(target, i, j);
                    let mut rgb: heman_color = *colors.offset(k as isize);
                    let fresh4 = texel;
                    texel = texel.offset(1);
                    *fresh4 = (rgb >> 16 as libc::c_int) as libc::c_float * inv;
                    let fresh5 = texel;
                    texel = texel.offset(1);
                    *fresh5 = (rgb >> 8 as libc::c_int
                        & 0xff as libc::c_int as libc::c_uint) as libc::c_float * inv;
                    *texel = (rgb & 0xff as libc::c_int as libc::c_uint) as libc::c_float
                        * inv;
                }
                ki += 1;
                ki;
            }
            kj += 1;
            kj;
        }
        k += 1;
        k;
    }
}
pub unsafe extern "C" fn heman_draw_splats(
    mut target: *mut heman_image,
    mut pts: *mut heman_points,
    mut radius: libc::c_int,
    mut blend_mode: libc::c_int,
) {
    let mut fwidth: libc::c_int = radius * 2 as libc::c_int + 1 as libc::c_int;
    let mut gaussian_splat: *mut libc::c_float = malloc(
        ((fwidth * fwidth) as libc::c_ulong)
            .wrapping_mul(::std::mem::size_of::<libc::c_float>() as libc::c_ulong),
    ) as *mut libc::c_float;
    generate_gaussian_splat(gaussian_splat, fwidth);
    let mut src: *mut libc::c_float = (*pts).data;
    let mut w: libc::c_int = (*target).width;
    let mut h: libc::c_int = (*target).height;
    let mut i: libc::c_int = 0 as libc::c_int;
    while i < (*pts).width {
        let fresh6 = src;
        src = src.offset(1);
        let mut x: libc::c_float = *fresh6;
        let fresh7 = src;
        src = src.offset(1);
        let mut y: libc::c_float = *fresh7;
        let mut ii: libc::c_int = (x * w as libc::c_float - radius as libc::c_float)
            as libc::c_int;
        let mut jj: libc::c_int = (y * h as libc::c_float - radius as libc::c_float)
            as libc::c_int;
        let mut kj: libc::c_int = 0 as libc::c_int;
        while kj < fwidth {
            let mut ki: libc::c_int = 0 as libc::c_int;
            while ki < fwidth {
                let mut i_0: libc::c_int = ii + ki;
                let mut j: libc::c_int = jj + kj;
                if !(i_0 < 0 as libc::c_int || i_0 >= w || j < 0 as libc::c_int
                    || j >= h)
                {
                    let mut texel: *mut libc::c_float = heman_image_texel(
                        target,
                        i_0,
                        j,
                    );
                    let mut c: libc::c_int = 0 as libc::c_int;
                    while c < (*target).nbands {
                        let fresh8 = texel;
                        texel = texel.offset(1);
                        *fresh8 += *gaussian_splat.offset((kj * fwidth + ki) as isize);
                        c += 1;
                        c;
                    }
                }
                ki += 1;
                ki;
            }
            kj += 1;
            kj;
        }
        i += 1;
        i;
    }
    free(gaussian_splat as *mut libc::c_void);
}
pub unsafe extern "C" fn heman_draw_contour_from_points(
    mut target: *mut heman_image,
    mut coords: *mut heman_points,
    mut rgb: heman_color,
    mut mind: libc::c_float,
    mut maxd: libc::c_float,
    mut filterd: libc::c_int,
) {
    if (*target).nbands == 3 as libc::c_int || (*target).nbands == 4 as libc::c_int
    {} else {
        __assert_fail(
            b"target->nbands == 3 || target->nbands == 4\0" as *const u8
                as *const libc::c_char,
            b"src/draw.c\0" as *const u8 as *const libc::c_char,
            119 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 99],
                &[libc::c_char; 99],
            >(
                b"void heman_draw_contour_from_points(heman_image *, heman_points *, heman_color, float, float, int)\0",
            ))
                .as_ptr(),
        );
    }
    'c_1463: {
        if (*target).nbands == 3 as libc::c_int || (*target).nbands == 4 as libc::c_int
        {} else {
            __assert_fail(
                b"target->nbands == 3 || target->nbands == 4\0" as *const u8
                    as *const libc::c_char,
                b"src/draw.c\0" as *const u8 as *const libc::c_char,
                119 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 99],
                    &[libc::c_char; 99],
                >(
                    b"void heman_draw_contour_from_points(heman_image *, heman_points *, heman_color, float, float, int)\0",
                ))
                    .as_ptr(),
            );
        }
    };
    let mut width: libc::c_int = (*target).width;
    let mut height: libc::c_int = (*target).height;
    let mut seed: *mut heman_image = heman_image_create(width, height, 1 as libc::c_int);
    heman_image_clear(seed, 0 as libc::c_int as libc::c_float);
    heman_internal_draw_seeds(seed, coords, filterd);
    let mut inv: libc::c_float = 1.0f32 / 255.0f32;
    let mut r: libc::c_float = (rgb >> 16 as libc::c_int
        & 0xff as libc::c_int as libc::c_uint) as libc::c_float * inv;
    let mut g: libc::c_float = (rgb >> 8 as libc::c_int
        & 0xff as libc::c_int as libc::c_uint) as libc::c_float * inv;
    let mut b: libc::c_float = (rgb & 0xff as libc::c_int as libc::c_uint)
        as libc::c_float * inv;
    let mut a: libc::c_float = 1 as libc::c_int as libc::c_float;
    if (*target).nbands == 4 as libc::c_int {
        a = (rgb >> 24 as libc::c_int) as libc::c_float * inv;
    }
    let mut y: libc::c_int = 0;
    y = 0 as libc::c_int;
    while y < height {
        let mut dst: *mut libc::c_float = ((*target).data)
            .offset((y * width * (*target).nbands) as isize);
        let mut x: libc::c_int = 0 as libc::c_int;
        while x < width {
            let mut dist: libc::c_float = *heman_image_texel(seed, x, y);
            if dist > mind && dist < maxd {
                *dst.offset(0 as libc::c_int as isize) = r;
                *dst.offset(1 as libc::c_int as isize) = g;
                *dst.offset(2 as libc::c_int as isize) = b;
                if (*target).nbands == 4 as libc::c_int {
                    *dst.offset(3 as libc::c_int as isize) = a;
                }
            }
            dst = dst.offset((*target).nbands as isize);
            x += 1;
            x;
        }
        y += 1;
        y;
    }
    heman_points_destroy(seed);
}
