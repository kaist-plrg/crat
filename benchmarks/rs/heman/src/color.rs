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
    fn heman_image_sample(
        _: *mut heman_image,
        u: libc::c_float,
        v: libc::c_float,
        result: *mut libc::c_float,
    );
    fn __assert_fail(
        __assertion: *const libc::c_char,
        __file: *const libc::c_char,
        __line: libc::c_uint,
        __function: *const libc::c_char,
    ) -> !;
    fn malloc(_: libc::c_ulong) -> *mut libc::c_void;
    fn free(__ptr: *mut libc::c_void);
    fn pow(_: libc::c_double, _: libc::c_double) -> libc::c_double;
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
pub type heman_color = libc::c_uint;
pub static mut _gamma: libc::c_float = 2.2f32;
pub unsafe extern "C" fn heman_color_set_gamma(mut g: libc::c_float) {
    _gamma = g;
}
pub unsafe extern "C" fn heman_color_create_gradient(
    mut width: libc::c_int,
    mut num_colors: libc::c_int,
    mut cp_locations: *const libc::c_int,
    mut cp_values: *const heman_color,
) -> *mut heman_image {
    if width > 0 as libc::c_int && num_colors >= 2 as libc::c_int {} else {
        __assert_fail(
            b"width > 0 && num_colors >= 2\0" as *const u8 as *const libc::c_char,
            b"src/color.c\0" as *const u8 as *const libc::c_char,
            13 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 85],
                &[libc::c_char; 85],
            >(
                b"heman_image *heman_color_create_gradient(int, int, const int *, const heman_color *)\0",
            ))
                .as_ptr(),
        );
    }
    'c_660: {
        if width > 0 as libc::c_int && num_colors >= 2 as libc::c_int {} else {
            __assert_fail(
                b"width > 0 && num_colors >= 2\0" as *const u8 as *const libc::c_char,
                b"src/color.c\0" as *const u8 as *const libc::c_char,
                13 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 85],
                    &[libc::c_char; 85],
                >(
                    b"heman_image *heman_color_create_gradient(int, int, const int *, const heman_color *)\0",
                ))
                    .as_ptr(),
            );
        }
    };
    if *cp_locations.offset(0 as libc::c_int as isize) == 0 as libc::c_int {} else {
        __assert_fail(
            b"cp_locations[0] == 0\0" as *const u8 as *const libc::c_char,
            b"src/color.c\0" as *const u8 as *const libc::c_char,
            14 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 85],
                &[libc::c_char; 85],
            >(
                b"heman_image *heman_color_create_gradient(int, int, const int *, const heman_color *)\0",
            ))
                .as_ptr(),
        );
    }
    'c_617: {
        if *cp_locations.offset(0 as libc::c_int as isize) == 0 as libc::c_int {} else {
            __assert_fail(
                b"cp_locations[0] == 0\0" as *const u8 as *const libc::c_char,
                b"src/color.c\0" as *const u8 as *const libc::c_char,
                14 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 85],
                    &[libc::c_char; 85],
                >(
                    b"heman_image *heman_color_create_gradient(int, int, const int *, const heman_color *)\0",
                ))
                    .as_ptr(),
            );
        }
    };
    if *cp_locations.offset((num_colors - 1 as libc::c_int) as isize)
        == width - 1 as libc::c_int
    {} else {
        __assert_fail(
            b"cp_locations[num_colors - 1] == width - 1\0" as *const u8
                as *const libc::c_char,
            b"src/color.c\0" as *const u8 as *const libc::c_char,
            15 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 85],
                &[libc::c_char; 85],
            >(
                b"heman_image *heman_color_create_gradient(int, int, const int *, const heman_color *)\0",
            ))
                .as_ptr(),
        );
    }
    'c_552: {
        if *cp_locations.offset((num_colors - 1 as libc::c_int) as isize)
            == width - 1 as libc::c_int
        {} else {
            __assert_fail(
                b"cp_locations[num_colors - 1] == width - 1\0" as *const u8
                    as *const libc::c_char,
                b"src/color.c\0" as *const u8 as *const libc::c_char,
                15 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 85],
                    &[libc::c_char; 85],
                >(
                    b"heman_image *heman_color_create_gradient(int, int, const int *, const heman_color *)\0",
                ))
                    .as_ptr(),
            );
        }
    };
    let mut f32colors: *mut libc::c_float = malloc(
        (::std::mem::size_of::<libc::c_float>() as libc::c_ulong)
            .wrapping_mul(3 as libc::c_int as libc::c_ulong)
            .wrapping_mul(num_colors as libc::c_ulong),
    ) as *mut libc::c_float;
    let mut inv: libc::c_float = 1.0f32 / 255.0f32;
    let mut f32color: *mut libc::c_float = f32colors;
    let mut u32color: *const heman_color = cp_values;
    let mut index: libc::c_int = 0 as libc::c_int;
    while index < num_colors {
        let fresh0 = u32color;
        u32color = u32color.offset(1);
        let mut rgb: heman_color = *fresh0;
        let mut r: libc::c_float = (rgb >> 16 as libc::c_int) as libc::c_float * inv;
        let mut g: libc::c_float = (rgb >> 8 as libc::c_int
            & 0xff as libc::c_int as libc::c_uint) as libc::c_float * inv;
        let mut b: libc::c_float = (rgb & 0xff as libc::c_int as libc::c_uint)
            as libc::c_float * inv;
        let fresh1 = f32color;
        f32color = f32color.offset(1);
        *fresh1 = pow(r as libc::c_double, _gamma as libc::c_double) as libc::c_float;
        let fresh2 = f32color;
        f32color = f32color.offset(1);
        *fresh2 = pow(g as libc::c_double, _gamma as libc::c_double) as libc::c_float;
        let fresh3 = f32color;
        f32color = f32color.offset(1);
        *fresh3 = pow(b as libc::c_double, _gamma as libc::c_double) as libc::c_float;
        index += 1;
        index;
    }
    let mut result: *mut heman_image = heman_image_create(
        width,
        1 as libc::c_int,
        3 as libc::c_int,
    );
    let mut index0: libc::c_int = 0 as libc::c_int;
    let mut index1: libc::c_int = 1 as libc::c_int;
    let mut dst: *mut libc::c_float = (*result).data;
    let mut t: libc::c_float = 0.;
    let mut invgamma: libc::c_float = 1.0f32 / _gamma;
    let mut current_block_16: u64;
    let mut x: libc::c_int = 0 as libc::c_int;
    while x < width {
        let mut loc0: libc::c_int = *cp_locations.offset(index0 as isize);
        let mut loc1: libc::c_int = *cp_locations.offset(index1 as isize);
        if loc0 == loc1 {
            t = 0 as libc::c_int as libc::c_float;
            current_block_16 = 11584701595673473500;
        } else {
            t = (x - loc0) as libc::c_float / (loc1 - loc0) as libc::c_float;
            if t >= 1 as libc::c_int as libc::c_float {
                x -= 1;
                x;
                index0 += 1;
                index0;
                index1 = if index1 + 1 as libc::c_int > num_colors - 1 as libc::c_int {
                    num_colors - 1 as libc::c_int
                } else {
                    index1 + 1 as libc::c_int
                };
                current_block_16 = 5399440093318478209;
            } else {
                current_block_16 = 11584701595673473500;
            }
        }
        match current_block_16 {
            11584701595673473500 => {
                let mut r0: libc::c_float = *f32colors
                    .offset((index0 * 3 as libc::c_int) as isize);
                let mut g0: libc::c_float = *f32colors
                    .offset((index0 * 3 as libc::c_int + 1 as libc::c_int) as isize);
                let mut b0: libc::c_float = *f32colors
                    .offset((index0 * 3 as libc::c_int + 2 as libc::c_int) as isize);
                let mut r1: libc::c_float = *f32colors
                    .offset((index1 * 3 as libc::c_int) as isize);
                let mut g1: libc::c_float = *f32colors
                    .offset((index1 * 3 as libc::c_int + 1 as libc::c_int) as isize);
                let mut b1: libc::c_float = *f32colors
                    .offset((index1 * 3 as libc::c_int + 2 as libc::c_int) as isize);
                let mut invt: libc::c_float = 1.0f32 - t;
                let mut r_0: libc::c_float = r0 * invt + r1 * t;
                let mut g_0: libc::c_float = g0 * invt + g1 * t;
                let mut b_0: libc::c_float = b0 * invt + b1 * t;
                let fresh4 = dst;
                dst = dst.offset(1);
                *fresh4 = pow(r_0 as libc::c_double, invgamma as libc::c_double)
                    as libc::c_float;
                let fresh5 = dst;
                dst = dst.offset(1);
                *fresh5 = pow(g_0 as libc::c_double, invgamma as libc::c_double)
                    as libc::c_float;
                let fresh6 = dst;
                dst = dst.offset(1);
                *fresh6 = pow(b_0 as libc::c_double, invgamma as libc::c_double)
                    as libc::c_float;
            }
            _ => {}
        }
        x += 1;
        x;
    }
    free(f32colors as *mut libc::c_void);
    return result;
}
pub unsafe extern "C" fn heman_color_apply_gradient(
    mut heightmap: *mut heman_image,
    mut minheight: libc::c_float,
    mut maxheight: libc::c_float,
    mut gradient: *mut heman_image,
) -> *mut heman_image {
    if (*heightmap).nbands == 1 as libc::c_int {} else {
        __assert_fail(
            b"heightmap->nbands == 1\0" as *const u8 as *const libc::c_char,
            b"src/color.c\0" as *const u8 as *const libc::c_char,
            74 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 84],
                &[libc::c_char; 84],
            >(
                b"heman_image *heman_color_apply_gradient(heman_image *, float, float, heman_image *)\0",
            ))
                .as_ptr(),
        );
    }
    'c_963: {
        if (*heightmap).nbands == 1 as libc::c_int {} else {
            __assert_fail(
                b"heightmap->nbands == 1\0" as *const u8 as *const libc::c_char,
                b"src/color.c\0" as *const u8 as *const libc::c_char,
                74 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 84],
                    &[libc::c_char; 84],
                >(
                    b"heman_image *heman_color_apply_gradient(heman_image *, float, float, heman_image *)\0",
                ))
                    .as_ptr(),
            );
        }
    };
    if (*gradient).height == 1 as libc::c_int {} else {
        __assert_fail(
            b"gradient->height == 1\0" as *const u8 as *const libc::c_char,
            b"src/color.c\0" as *const u8 as *const libc::c_char,
            75 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 84],
                &[libc::c_char; 84],
            >(
                b"heman_image *heman_color_apply_gradient(heman_image *, float, float, heman_image *)\0",
            ))
                .as_ptr(),
        );
    }
    'c_923: {
        if (*gradient).height == 1 as libc::c_int {} else {
            __assert_fail(
                b"gradient->height == 1\0" as *const u8 as *const libc::c_char,
                b"src/color.c\0" as *const u8 as *const libc::c_char,
                75 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 84],
                    &[libc::c_char; 84],
                >(
                    b"heman_image *heman_color_apply_gradient(heman_image *, float, float, heman_image *)\0",
                ))
                    .as_ptr(),
            );
        }
    };
    if (*gradient).nbands == 3 as libc::c_int {} else {
        __assert_fail(
            b"gradient->nbands == 3\0" as *const u8 as *const libc::c_char,
            b"src/color.c\0" as *const u8 as *const libc::c_char,
            76 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 84],
                &[libc::c_char; 84],
            >(
                b"heman_image *heman_color_apply_gradient(heman_image *, float, float, heman_image *)\0",
            ))
                .as_ptr(),
        );
    }
    'c_881: {
        if (*gradient).nbands == 3 as libc::c_int {} else {
            __assert_fail(
                b"gradient->nbands == 3\0" as *const u8 as *const libc::c_char,
                b"src/color.c\0" as *const u8 as *const libc::c_char,
                76 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 84],
                    &[libc::c_char; 84],
                >(
                    b"heman_image *heman_color_apply_gradient(heman_image *, float, float, heman_image *)\0",
                ))
                    .as_ptr(),
            );
        }
    };
    let mut w: libc::c_int = (*heightmap).width;
    let mut h: libc::c_int = (*heightmap).height;
    let mut result: *mut heman_image = heman_image_create(w, h, 3 as libc::c_int);
    let mut size: libc::c_int = (*result).height * (*result).width;
    let mut dst: *mut libc::c_float = (*result).data;
    let mut src: *const libc::c_float = (*heightmap).data;
    let mut scale: libc::c_float = 1.0f32 / (maxheight - minheight);
    let mut i: libc::c_int = 0 as libc::c_int;
    while i < size {
        let mut u: libc::c_float = if 0.0f32
            > (if 1.0f32 > (*src - minheight) * scale {
                (*src - minheight) * scale
            } else {
                1.0f32
            })
        {
            0.0f32
        } else if 1.0f32 > (*src - minheight) * scale {
            (*src - minheight) * scale
        } else {
            1.0f32
        };
        heman_image_sample(gradient, u, 0.5f32, dst);
        i += 1;
        i;
        dst = dst.offset(3 as libc::c_int as isize);
        src = src.offset(1);
        src;
    }
    return result;
}
pub unsafe extern "C" fn heman_color_from_grayscale(
    mut grayscale: *mut heman_image,
) -> *mut heman_image {
    if (*grayscale).nbands == 1 as libc::c_int {} else {
        __assert_fail(
            b"grayscale->nbands == 1\0" as *const u8 as *const libc::c_char,
            b"src/color.c\0" as *const u8 as *const libc::c_char,
            93 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 55],
                &[libc::c_char; 55],
            >(b"heman_image *heman_color_from_grayscale(heman_image *)\0"))
                .as_ptr(),
        );
    }
    'c_1092: {
        if (*grayscale).nbands == 1 as libc::c_int {} else {
            __assert_fail(
                b"grayscale->nbands == 1\0" as *const u8 as *const libc::c_char,
                b"src/color.c\0" as *const u8 as *const libc::c_char,
                93 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 55],
                    &[libc::c_char; 55],
                >(b"heman_image *heman_color_from_grayscale(heman_image *)\0"))
                    .as_ptr(),
            );
        }
    };
    let mut w: libc::c_int = (*grayscale).width;
    let mut h: libc::c_int = (*grayscale).height;
    let mut result: *mut heman_image = heman_image_create(w, h, 3 as libc::c_int);
    let mut size: libc::c_int = w * h;
    let mut dst: *mut libc::c_float = (*result).data;
    let mut src: *const libc::c_float = (*grayscale).data;
    let mut i: libc::c_int = 0 as libc::c_int;
    while i < size {
        let fresh7 = src;
        src = src.offset(1);
        let mut v: libc::c_float = *fresh7;
        let fresh8 = dst;
        dst = dst.offset(1);
        *fresh8 = v;
        let fresh9 = dst;
        dst = dst.offset(1);
        *fresh9 = v;
        let fresh10 = dst;
        dst = dst.offset(1);
        *fresh10 = v;
        i += 1;
        i;
    }
    return result;
}
pub unsafe extern "C" fn heman_color_to_grayscale(
    mut colorimg: *mut heman_image,
) -> *mut heman_image {
    if (*colorimg).nbands == 3 as libc::c_int {} else {
        __assert_fail(
            b"colorimg->nbands == 3\0" as *const u8 as *const libc::c_char,
            b"src/color.c\0" as *const u8 as *const libc::c_char,
            111 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 53],
                &[libc::c_char; 53],
            >(b"heman_image *heman_color_to_grayscale(heman_image *)\0"))
                .as_ptr(),
        );
    }
    'c_1235: {
        if (*colorimg).nbands == 3 as libc::c_int {} else {
            __assert_fail(
                b"colorimg->nbands == 3\0" as *const u8 as *const libc::c_char,
                b"src/color.c\0" as *const u8 as *const libc::c_char,
                111 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 53],
                    &[libc::c_char; 53],
                >(b"heman_image *heman_color_to_grayscale(heman_image *)\0"))
                    .as_ptr(),
            );
        }
    };
    let mut w: libc::c_int = (*colorimg).width;
    let mut h: libc::c_int = (*colorimg).height;
    let mut result: *mut heman_image = heman_image_create(w, h, 1 as libc::c_int);
    let mut size: libc::c_int = w * h;
    let mut dst: *mut libc::c_float = (*result).data;
    let mut src: *const libc::c_float = (*colorimg).data;
    let mut i: libc::c_int = 0 as libc::c_int;
    while i < size {
        let fresh11 = src;
        src = src.offset(1);
        let mut r: libc::c_float = *fresh11;
        let fresh12 = src;
        src = src.offset(1);
        let mut g: libc::c_float = *fresh12;
        let fresh13 = src;
        src = src.offset(1);
        let mut b: libc::c_float = *fresh13;
        let fresh14 = dst;
        dst = dst.offset(1);
        *fresh14 = (0.299f64 * r as libc::c_double + 0.587f64 * g as libc::c_double
            + 0.114f64 * b as libc::c_double) as libc::c_float;
        i += 1;
        i;
    }
    return result;
}
pub unsafe extern "C" fn heman_internal_rg(
    mut cfield: *mut heman_image,
) -> *mut heman_image {
    if (*cfield).nbands == 2 as libc::c_int {} else {
        __assert_fail(
            b"cfield->nbands == 2\0" as *const u8 as *const libc::c_char,
            b"src/color.c\0" as *const u8 as *const libc::c_char,
            129 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 46],
                &[libc::c_char; 46],
            >(b"heman_image *heman_internal_rg(heman_image *)\0"))
                .as_ptr(),
        );
    }
    'c_1695: {
        if (*cfield).nbands == 2 as libc::c_int {} else {
            __assert_fail(
                b"cfield->nbands == 2\0" as *const u8 as *const libc::c_char,
                b"src/color.c\0" as *const u8 as *const libc::c_char,
                129 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 46],
                    &[libc::c_char; 46],
                >(b"heman_image *heman_internal_rg(heman_image *)\0"))
                    .as_ptr(),
            );
        }
    };
    let mut w: libc::c_int = (*cfield).width;
    let mut h: libc::c_int = (*cfield).height;
    let mut target: *mut heman_image = heman_image_create(w, h, 3 as libc::c_int);
    let mut dst: *mut libc::c_float = (*target).data;
    let mut src: *mut libc::c_float = (*cfield).data;
    let mut size: libc::c_int = w * h;
    let mut i: libc::c_int = 0 as libc::c_int;
    while i < size {
        let fresh15 = src;
        src = src.offset(1);
        let mut u: libc::c_float = *fresh15 / w as libc::c_float;
        let fresh16 = src;
        src = src.offset(1);
        let mut v: libc::c_float = *fresh16 / h as libc::c_float;
        let fresh17 = dst;
        dst = dst.offset(1);
        *fresh17 = u;
        let fresh18 = dst;
        dst = dst.offset(1);
        *fresh18 = v;
        let fresh19 = dst;
        dst = dst.offset(1);
        *fresh19 = 0 as libc::c_int as libc::c_float;
        i += 1;
        i;
    }
    return target;
}
pub unsafe extern "C" fn heman_color_from_cpcf(
    mut cfield: *mut heman_image,
    mut texture: *mut heman_image,
) -> *mut heman_image {
    if texture.is_null() {
        return heman_internal_rg(cfield);
    }
    if (*cfield).nbands == 2 as libc::c_int {} else {
        __assert_fail(
            b"cfield->nbands == 2\0" as *const u8 as *const libc::c_char,
            b"src/color.c\0" as *const u8 as *const libc::c_char,
            151 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 65],
                &[libc::c_char; 65],
            >(b"heman_image *heman_color_from_cpcf(heman_image *, heman_image *)\0"))
                .as_ptr(),
        );
    }
    'c_1544: {
        if (*cfield).nbands == 2 as libc::c_int {} else {
            __assert_fail(
                b"cfield->nbands == 2\0" as *const u8 as *const libc::c_char,
                b"src/color.c\0" as *const u8 as *const libc::c_char,
                151 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 65],
                    &[libc::c_char; 65],
                >(b"heman_image *heman_color_from_cpcf(heman_image *, heman_image *)\0"))
                    .as_ptr(),
            );
        }
    };
    if (*texture).nbands == 3 as libc::c_int || (*texture).nbands == 4 as libc::c_int
    {} else {
        __assert_fail(
            b"texture->nbands == 3 || texture->nbands == 4\0" as *const u8
                as *const libc::c_char,
            b"src/color.c\0" as *const u8 as *const libc::c_char,
            152 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 65],
                &[libc::c_char; 65],
            >(b"heman_image *heman_color_from_cpcf(heman_image *, heman_image *)\0"))
                .as_ptr(),
        );
    }
    'c_1489: {
        if (*texture).nbands == 3 as libc::c_int || (*texture).nbands == 4 as libc::c_int
        {} else {
            __assert_fail(
                b"texture->nbands == 3 || texture->nbands == 4\0" as *const u8
                    as *const libc::c_char,
                b"src/color.c\0" as *const u8 as *const libc::c_char,
                152 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 65],
                    &[libc::c_char; 65],
                >(b"heman_image *heman_color_from_cpcf(heman_image *, heman_image *)\0"))
                    .as_ptr(),
            );
        }
    };
    if (*cfield).width == (*texture).width {} else {
        __assert_fail(
            b"cfield->width == texture->width\0" as *const u8 as *const libc::c_char,
            b"src/color.c\0" as *const u8 as *const libc::c_char,
            153 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 65],
                &[libc::c_char; 65],
            >(b"heman_image *heman_color_from_cpcf(heman_image *, heman_image *)\0"))
                .as_ptr(),
        );
    }
    'c_1442: {
        if (*cfield).width == (*texture).width {} else {
            __assert_fail(
                b"cfield->width == texture->width\0" as *const u8 as *const libc::c_char,
                b"src/color.c\0" as *const u8 as *const libc::c_char,
                153 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 65],
                    &[libc::c_char; 65],
                >(b"heman_image *heman_color_from_cpcf(heman_image *, heman_image *)\0"))
                    .as_ptr(),
            );
        }
    };
    if (*cfield).height == (*texture).height {} else {
        __assert_fail(
            b"cfield->height == texture->height\0" as *const u8 as *const libc::c_char,
            b"src/color.c\0" as *const u8 as *const libc::c_char,
            154 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 65],
                &[libc::c_char; 65],
            >(b"heman_image *heman_color_from_cpcf(heman_image *, heman_image *)\0"))
                .as_ptr(),
        );
    }
    'c_1394: {
        if (*cfield).height == (*texture).height {} else {
            __assert_fail(
                b"cfield->height == texture->height\0" as *const u8
                    as *const libc::c_char,
                b"src/color.c\0" as *const u8 as *const libc::c_char,
                154 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 65],
                    &[libc::c_char; 65],
                >(b"heman_image *heman_color_from_cpcf(heman_image *, heman_image *)\0"))
                    .as_ptr(),
            );
        }
    };
    let mut w: libc::c_int = (*cfield).width;
    let mut h: libc::c_int = (*cfield).height;
    let mut target: *mut heman_image = heman_image_create(w, h, (*texture).nbands);
    let mut dst: *mut libc::c_float = (*target).data;
    let mut src: *mut libc::c_float = (*cfield).data;
    let mut size: libc::c_int = w * h;
    let mut i: libc::c_int = 0 as libc::c_int;
    while i < size {
        let fresh20 = src;
        src = src.offset(1);
        let mut u: libc::c_float = *fresh20;
        let fresh21 = src;
        src = src.offset(1);
        let mut v: libc::c_float = *fresh21;
        let mut texel: *mut libc::c_float = heman_image_texel(
            texture,
            u as libc::c_int,
            v as libc::c_int,
        );
        let mut c: libc::c_int = 0 as libc::c_int;
        while c < (*texture).nbands {
            let fresh22 = texel;
            texel = texel.offset(1);
            let fresh23 = dst;
            dst = dst.offset(1);
            *fresh23 = *fresh22;
            c += 1;
            c;
        }
        i += 1;
        i;
    }
    return target;
}
