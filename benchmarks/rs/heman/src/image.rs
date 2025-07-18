use ::libc;
extern "C" {
    fn malloc(_: libc::c_ulong) -> *mut libc::c_void;
    fn free(__ptr: *mut libc::c_void);
    fn __assert_fail(
        __assertion: *const libc::c_char,
        __file: *const libc::c_char,
        __line: libc::c_uint,
        __function: *const libc::c_char,
    ) -> !;
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
pub unsafe extern "C" fn heman_image_data(
    mut img: *mut heman_image,
) -> *mut libc::c_float {
    return (*img).data;
}
pub unsafe extern "C" fn heman_image_array(
    mut img: *mut heman_image,
    mut data: *mut *mut libc::c_float,
    mut nfloats: *mut libc::c_int,
) {
    *data = (*img).data;
    *nfloats = (*img).width * (*img).height * (*img).nbands;
}
pub unsafe extern "C" fn heman_image_info(
    mut img: *mut heman_image,
    mut width: *mut libc::c_int,
    mut height: *mut libc::c_int,
    mut nbands: *mut libc::c_int,
) {
    *width = (*img).width;
    *height = (*img).height;
    *nbands = (*img).nbands;
}
pub unsafe extern "C" fn heman_image_texel(
    mut img: *mut heman_image,
    mut x: libc::c_int,
    mut y: libc::c_int,
) -> *mut libc::c_float {
    return ((*img).data)
        .offset((y * (*img).width * (*img).nbands) as isize)
        .offset((x * (*img).nbands) as isize);
}
pub unsafe extern "C" fn heman_image_create(
    mut width: libc::c_int,
    mut height: libc::c_int,
    mut nbands: libc::c_int,
) -> *mut heman_image {
    let mut img: *mut heman_image = malloc(
        ::std::mem::size_of::<heman_image>() as libc::c_ulong,
    ) as *mut heman_image;
    (*img).width = width;
    (*img).height = height;
    (*img).nbands = nbands;
    (*img)
        .data = malloc(
        (::std::mem::size_of::<libc::c_float>() as libc::c_ulong)
            .wrapping_mul(width as libc::c_ulong)
            .wrapping_mul(height as libc::c_ulong)
            .wrapping_mul(nbands as libc::c_ulong),
    ) as *mut libc::c_float;
    return img;
}
pub unsafe extern "C" fn heman_image_destroy(mut img: *mut heman_image) {
    free((*img).data as *mut libc::c_void);
    free(img as *mut libc::c_void);
}
pub unsafe extern "C" fn heman_image_sample(
    mut img: *mut heman_image,
    mut u: libc::c_float,
    mut v: libc::c_float,
    mut result: *mut libc::c_float,
) {
    let mut x: libc::c_int = (if 0 as libc::c_int as libc::c_float
        > (if ((*img).width - 1 as libc::c_int) as libc::c_float
            > (*img).width as libc::c_float * u
        {
            (*img).width as libc::c_float * u
        } else {
            ((*img).width - 1 as libc::c_int) as libc::c_float
        })
    {
        0 as libc::c_int as libc::c_float
    } else if ((*img).width - 1 as libc::c_int) as libc::c_float
        > (*img).width as libc::c_float * u
    {
        (*img).width as libc::c_float * u
    } else {
        ((*img).width - 1 as libc::c_int) as libc::c_float
    }) as libc::c_int;
    let mut y: libc::c_int = (if 0 as libc::c_int as libc::c_float
        > (if ((*img).height - 1 as libc::c_int) as libc::c_float
            > (*img).height as libc::c_float * v
        {
            (*img).height as libc::c_float * v
        } else {
            ((*img).height - 1 as libc::c_int) as libc::c_float
        })
    {
        0 as libc::c_int as libc::c_float
    } else if ((*img).height - 1 as libc::c_int) as libc::c_float
        > (*img).height as libc::c_float * v
    {
        (*img).height as libc::c_float * v
    } else {
        ((*img).height - 1 as libc::c_int) as libc::c_float
    }) as libc::c_int;
    let mut data: *mut libc::c_float = heman_image_texel(img, x, y);
    let mut b: libc::c_int = 0 as libc::c_int;
    while b < (*img).nbands {
        let fresh0 = data;
        data = data.offset(1);
        let fresh1 = result;
        result = result.offset(1);
        *fresh1 = *fresh0;
        b += 1;
        b;
    }
}
pub unsafe extern "C" fn heman_image_clear(
    mut img: *mut heman_image,
    mut value: libc::c_float,
) {
    let mut size: libc::c_int = (*img).width * (*img).height * (*img).nbands;
    let mut dst: *mut libc::c_float = (*img).data;
    loop {
        let fresh2 = size;
        size = size - 1;
        if !(fresh2 != 0) {
            break;
        }
        let fresh3 = dst;
        dst = dst.offset(1);
        *fresh3 = value;
    };
}
pub unsafe extern "C" fn heman_image_extract_alpha(
    mut img: *mut heman_image,
) -> *mut heman_image {
    if (*img).nbands == 4 as libc::c_int {} else {
        __assert_fail(
            b"img->nbands == 4\0" as *const u8 as *const libc::c_char,
            b"src/image.c\0" as *const u8 as *const libc::c_char,
            63 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 54],
                &[libc::c_char; 54],
            >(b"heman_image *heman_image_extract_alpha(heman_image *)\0"))
                .as_ptr(),
        );
    }
    'c_548: {
        if (*img).nbands == 4 as libc::c_int {} else {
            __assert_fail(
                b"img->nbands == 4\0" as *const u8 as *const libc::c_char,
                b"src/image.c\0" as *const u8 as *const libc::c_char,
                63 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 54],
                    &[libc::c_char; 54],
                >(b"heman_image *heman_image_extract_alpha(heman_image *)\0"))
                    .as_ptr(),
            );
        }
    };
    let mut retval: *mut heman_image = heman_image_create(
        (*img).width,
        (*img).height,
        1 as libc::c_int,
    );
    let mut size: libc::c_int = (*img).width * (*img).height;
    let mut src: *mut libc::c_float = (*img).data;
    let mut dst: *mut libc::c_float = (*retval).data;
    loop {
        let fresh4 = size;
        size = size - 1;
        if !(fresh4 != 0) {
            break;
        }
        src = src.offset(3 as libc::c_int as isize);
        let fresh5 = src;
        src = src.offset(1);
        let fresh6 = dst;
        dst = dst.offset(1);
        *fresh6 = *fresh5;
    }
    return retval;
}
pub unsafe extern "C" fn heman_image_extract_rgb(
    mut img: *mut heman_image,
) -> *mut heman_image {
    if (*img).nbands == 4 as libc::c_int {} else {
        __assert_fail(
            b"img->nbands == 4\0" as *const u8 as *const libc::c_char,
            b"src/image.c\0" as *const u8 as *const libc::c_char,
            77 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 52],
                &[libc::c_char; 52],
            >(b"heman_image *heman_image_extract_rgb(heman_image *)\0"))
                .as_ptr(),
        );
    }
    'c_677: {
        if (*img).nbands == 4 as libc::c_int {} else {
            __assert_fail(
                b"img->nbands == 4\0" as *const u8 as *const libc::c_char,
                b"src/image.c\0" as *const u8 as *const libc::c_char,
                77 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 52],
                    &[libc::c_char; 52],
                >(b"heman_image *heman_image_extract_rgb(heman_image *)\0"))
                    .as_ptr(),
            );
        }
    };
    let mut retval: *mut heman_image = heman_image_create(
        (*img).width,
        (*img).height,
        3 as libc::c_int,
    );
    let mut size: libc::c_int = (*img).width * (*img).height;
    let mut src: *mut libc::c_float = (*img).data;
    let mut dst: *mut libc::c_float = (*retval).data;
    loop {
        let fresh7 = size;
        size = size - 1;
        if !(fresh7 != 0) {
            break;
        }
        let fresh8 = src;
        src = src.offset(1);
        let fresh9 = dst;
        dst = dst.offset(1);
        *fresh9 = *fresh8;
        let fresh10 = src;
        src = src.offset(1);
        let fresh11 = dst;
        dst = dst.offset(1);
        *fresh11 = *fresh10;
        let fresh12 = src;
        src = src.offset(1);
        let fresh13 = dst;
        dst = dst.offset(1);
        *fresh13 = *fresh12;
        src = src.offset(1);
        src;
    }
    return retval;
}
