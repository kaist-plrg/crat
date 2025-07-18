use ::libc;
extern "C" {
    fn __errno_location() -> *mut libc::c_int;
    fn rpl_free(ptr: *mut libc::c_void);
    fn malloc(_: libc::c_ulong) -> *mut libc::c_void;
    fn realloc(_: *mut libc::c_void, _: libc::c_ulong) -> *mut libc::c_void;
    fn abort() -> !;
    fn memcpy(
        _: *mut libc::c_void,
        _: *const libc::c_void,
        _: libc::c_ulong,
    ) -> *mut libc::c_void;
    fn u8_mbtouc_unsafe_aux(
        puc: *mut ucs4_t,
        s: *const uint8_t,
        n: size_t,
    ) -> libc::c_int;
    fn u8_uctomb_aux(s: *mut uint8_t, uc: ucs4_t, n: ptrdiff_t) -> libc::c_int;
    fn u8_cpy(dest: *mut uint8_t, src: *const uint8_t, n: size_t) -> *mut uint8_t;
    fn uc_combining_class(uc: ucs4_t) -> libc::c_int;
    fn gl_uninorm_decompose_merge_sort_inplace(
        src: *mut ucs4_with_ccc,
        n: size_t,
        tmp: *mut ucs4_with_ccc,
    );
}
pub type ptrdiff_t = libc::c_long;
pub type size_t = libc::c_ulong;
pub type __uint8_t = libc::c_uchar;
pub type __uint32_t = libc::c_uint;
pub type uint8_t = __uint8_t;
pub type uint32_t = __uint32_t;
pub type ucs4_t = uint32_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct unicode_normalization_form {
    pub description: libc::c_uint,
    pub decomposer: Option::<unsafe extern "C" fn(ucs4_t, *mut ucs4_t) -> libc::c_int>,
    pub composer: Option::<unsafe extern "C" fn(ucs4_t, ucs4_t) -> ucs4_t>,
    pub decomposing_variant: *const unicode_normalization_form,
}
pub type uninorm_t = *const unicode_normalization_form;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ucs4_with_ccc {
    pub code: ucs4_t,
    pub ccc: libc::c_int,
}
#[inline]
unsafe extern "C" fn u8_mbtouc_unsafe(
    mut puc: *mut ucs4_t,
    mut s: *const uint8_t,
    mut n: size_t,
) -> libc::c_int {
    let mut c: uint8_t = *s;
    if (c as libc::c_int) < 0x80 as libc::c_int {
        *puc = c as ucs4_t;
        return 1 as libc::c_int;
    } else {
        return u8_mbtouc_unsafe_aux(puc, s, n)
    };
}
#[inline]
unsafe extern "C" fn u8_uctomb(
    mut s: *mut uint8_t,
    mut uc: ucs4_t,
    mut n: ptrdiff_t,
) -> libc::c_int {
    if uc < 0x80 as libc::c_int as libc::c_uint && n > 0 as libc::c_int as libc::c_long {
        *s.offset(0 as libc::c_int as isize) = uc as uint8_t;
        return 1 as libc::c_int;
    } else {
        return u8_uctomb_aux(s, uc, n)
    };
}
pub unsafe extern "C" fn u8_normalize(
    mut nf: uninorm_t,
    mut s: *const uint8_t,
    mut n: size_t,
    mut resultbuf: *mut uint8_t,
    mut lengthp: *mut size_t,
) -> *mut uint8_t {
    let mut current_block: u64;
    let mut decomposer: Option::<
        unsafe extern "C" fn(ucs4_t, *mut ucs4_t) -> libc::c_int,
    > = (*nf).decomposer;
    let mut composer: Option::<unsafe extern "C" fn(ucs4_t, ucs4_t) -> ucs4_t> = (*nf)
        .composer;
    let mut result: *mut uint8_t = 0 as *mut uint8_t;
    let mut length: size_t = 0;
    let mut allocated: size_t = 0;
    let mut sortbuf_preallocated: [ucs4_with_ccc; 128] = [ucs4_with_ccc {
        code: 0,
        ccc: 0,
    }; 128];
    let mut sortbuf: *mut ucs4_with_ccc = 0 as *mut ucs4_with_ccc;
    let mut sortbuf_allocated: size_t = 0;
    let mut sortbuf_count: size_t = 0;
    if resultbuf.is_null() {
        result = 0 as *mut uint8_t;
        allocated = 0 as libc::c_int as size_t;
    } else {
        result = resultbuf;
        allocated = *lengthp;
    }
    length = 0 as libc::c_int as size_t;
    sortbuf = sortbuf_preallocated.as_mut_ptr();
    sortbuf_allocated = 64 as libc::c_int as size_t;
    sortbuf_count = 0 as libc::c_int as size_t;
    let mut s_end: *const uint8_t = s.offset(n as isize);
    's_65: loop {
        let mut count: libc::c_int = 0;
        let mut decomposed: [ucs4_t; 32] = [0; 32];
        let mut decomposed_count: libc::c_int = 0;
        let mut i: libc::c_int = 0;
        if s < s_end {
            count = u8_mbtouc_unsafe(
                &mut *decomposed.as_mut_ptr().offset(0 as libc::c_int as isize),
                s,
                s_end.offset_from(s) as libc::c_long as size_t,
            );
            decomposed_count = 1 as libc::c_int;
            let mut curr: libc::c_int = 0;
            curr = 0 as libc::c_int;
            while curr < decomposed_count {
                let mut curr_decomposed: [ucs4_t; 32] = [0; 32];
                let mut curr_decomposed_count: libc::c_int = 0;
                curr_decomposed_count = decomposer
                    .unwrap()(decomposed[curr as usize], curr_decomposed.as_mut_ptr());
                if curr_decomposed_count >= 0 as libc::c_int {
                    let mut shift: libc::c_int = curr_decomposed_count
                        - 1 as libc::c_int;
                    if shift < 0 as libc::c_int {
                        abort();
                    }
                    if shift > 0 as libc::c_int {
                        let mut j: libc::c_int = 0;
                        decomposed_count += shift;
                        if decomposed_count > 32 as libc::c_int {
                            abort();
                        }
                        j = decomposed_count - 1 as libc::c_int - shift;
                        while j > curr {
                            decomposed[(j + shift) as usize] = decomposed[j as usize];
                            j -= 1;
                            j;
                        }
                    }
                    while shift >= 0 as libc::c_int {
                        decomposed[(curr + shift)
                            as usize] = curr_decomposed[shift as usize];
                        shift -= 1;
                        shift;
                    }
                } else {
                    curr += 1;
                    curr;
                }
            }
        } else {
            count = 0 as libc::c_int;
            decomposed_count = 0 as libc::c_int;
        }
        i = 0 as libc::c_int;
        loop {
            let mut uc: ucs4_t = 0;
            let mut ccc: libc::c_int = 0;
            if s < s_end {
                if i == decomposed_count {
                    break;
                }
                uc = decomposed[i as usize];
                ccc = uc_combining_class(uc);
            } else {
                uc = 0 as libc::c_int as ucs4_t;
                ccc = 0 as libc::c_int;
            }
            if ccc == 0 as libc::c_int {
                let mut j_0: size_t = 0;
                if sortbuf_count > 1 as libc::c_int as libc::c_ulong {
                    gl_uninorm_decompose_merge_sort_inplace(
                        sortbuf,
                        sortbuf_count,
                        sortbuf.offset(sortbuf_count as isize),
                    );
                }
                if composer.is_some() {
                    if sortbuf_count > 0 as libc::c_int as libc::c_ulong
                        && (*sortbuf.offset(0 as libc::c_int as isize)).ccc
                            == 0 as libc::c_int
                    {
                        j_0 = 1 as libc::c_int as size_t;
                        while j_0 < sortbuf_count {
                            if (*sortbuf.offset(j_0 as isize)).ccc
                                > (*sortbuf
                                    .offset(
                                        j_0.wrapping_sub(1 as libc::c_int as libc::c_ulong) as isize,
                                    ))
                                    .ccc
                            {
                                let mut combined: ucs4_t = composer
                                    .unwrap()(
                                    (*sortbuf.offset(0 as libc::c_int as isize)).code,
                                    (*sortbuf.offset(j_0 as isize)).code,
                                );
                                if combined != 0 {
                                    let mut k: size_t = 0;
                                    (*sortbuf.offset(0 as libc::c_int as isize))
                                        .code = combined;
                                    k = j_0.wrapping_add(1 as libc::c_int as libc::c_ulong);
                                    while k < sortbuf_count {
                                        *sortbuf
                                            .offset(
                                                k.wrapping_sub(1 as libc::c_int as libc::c_ulong) as isize,
                                            ) = *sortbuf.offset(k as isize);
                                        k = k.wrapping_add(1);
                                        k;
                                    }
                                    sortbuf_count = sortbuf_count.wrapping_sub(1);
                                    sortbuf_count;
                                    continue;
                                }
                            }
                            j_0 = j_0.wrapping_add(1);
                            j_0;
                        }
                        if s < s_end
                            && sortbuf_count == 1 as libc::c_int as libc::c_ulong
                        {
                            let mut combined_0: ucs4_t = composer
                                .unwrap()(
                                (*sortbuf.offset(0 as libc::c_int as isize)).code,
                                uc,
                            );
                            if combined_0 != 0 {
                                uc = combined_0;
                                ccc = 0 as libc::c_int;
                                sortbuf_count = 0 as libc::c_int as size_t;
                            }
                        }
                    }
                }
                j_0 = 0 as libc::c_int as size_t;
                while j_0 < sortbuf_count {
                    let mut muc: ucs4_t = (*sortbuf.offset(j_0 as isize)).code;
                    if length < allocated {
                        let mut ret: libc::c_int = u8_uctomb(
                            result.offset(length as isize),
                            muc,
                            allocated.wrapping_sub(length) as ptrdiff_t,
                        );
                        if ret == -(1 as libc::c_int) {
                            *__errno_location() = 22 as libc::c_int;
                            current_block = 11223704093569886599;
                            break 's_65;
                        } else if ret >= 0 as libc::c_int {
                            length = (length as libc::c_ulong)
                                .wrapping_add(ret as libc::c_ulong) as size_t as size_t;
                            current_block = 7419121793134201633;
                        } else {
                            current_block = 726525485109251713;
                        }
                    } else {
                        current_block = 726525485109251713;
                    }
                    match current_block {
                        726525485109251713 => {
                            let mut old_allocated: size_t = allocated;
                            let mut new_allocated: size_t = (2 as libc::c_int
                                as libc::c_ulong)
                                .wrapping_mul(old_allocated);
                            if new_allocated < 64 as libc::c_int as libc::c_ulong {
                                new_allocated = 64 as libc::c_int as size_t;
                            }
                            if new_allocated < old_allocated {
                                abort();
                            }
                            let mut larger_result: *mut uint8_t = 0 as *mut uint8_t;
                            if result.is_null() {
                                larger_result = malloc(
                                    new_allocated
                                        .wrapping_mul(
                                            ::std::mem::size_of::<uint8_t>() as libc::c_ulong,
                                        ),
                                ) as *mut uint8_t;
                                if larger_result.is_null() {
                                    *__errno_location() = 12 as libc::c_int;
                                    current_block = 11223704093569886599;
                                    break 's_65;
                                }
                            } else if result == resultbuf {
                                larger_result = malloc(
                                    new_allocated
                                        .wrapping_mul(
                                            ::std::mem::size_of::<uint8_t>() as libc::c_ulong,
                                        ),
                                ) as *mut uint8_t;
                                if larger_result.is_null() {
                                    *__errno_location() = 12 as libc::c_int;
                                    current_block = 11223704093569886599;
                                    break 's_65;
                                } else {
                                    u8_cpy(larger_result, resultbuf, length);
                                }
                            } else {
                                larger_result = realloc(
                                    result as *mut libc::c_void,
                                    new_allocated
                                        .wrapping_mul(
                                            ::std::mem::size_of::<uint8_t>() as libc::c_ulong,
                                        ),
                                ) as *mut uint8_t;
                                if larger_result.is_null() {
                                    *__errno_location() = 12 as libc::c_int;
                                    current_block = 11223704093569886599;
                                    break 's_65;
                                }
                            }
                            result = larger_result;
                            allocated = new_allocated;
                            let mut ret_0: libc::c_int = u8_uctomb(
                                result.offset(length as isize),
                                muc,
                                allocated.wrapping_sub(length) as ptrdiff_t,
                            );
                            if ret_0 == -(1 as libc::c_int) {
                                *__errno_location() = 22 as libc::c_int;
                                current_block = 11223704093569886599;
                                break 's_65;
                            } else {
                                if ret_0 < 0 as libc::c_int {
                                    abort();
                                }
                                length = (length as libc::c_ulong)
                                    .wrapping_add(ret_0 as libc::c_ulong) as size_t as size_t;
                            }
                        }
                        _ => {}
                    }
                    j_0 = j_0.wrapping_add(1);
                    j_0;
                }
                sortbuf_count = 0 as libc::c_int as size_t;
            }
            if !(s < s_end) {
                break;
            }
            if sortbuf_count == sortbuf_allocated {
                let mut new_sortbuf: *mut ucs4_with_ccc = 0 as *mut ucs4_with_ccc;
                sortbuf_allocated = (2 as libc::c_int as libc::c_ulong)
                    .wrapping_mul(sortbuf_allocated);
                if sortbuf_allocated < sortbuf_count {
                    abort();
                }
                new_sortbuf = malloc(
                    (2 as libc::c_int as libc::c_ulong)
                        .wrapping_mul(sortbuf_allocated)
                        .wrapping_mul(
                            ::std::mem::size_of::<ucs4_with_ccc>() as libc::c_ulong,
                        ),
                ) as *mut ucs4_with_ccc;
                if new_sortbuf.is_null() {
                    *__errno_location() = 12 as libc::c_int;
                    current_block = 11223704093569886599;
                    break 's_65;
                } else {
                    memcpy(
                        new_sortbuf as *mut libc::c_void,
                        sortbuf as *const libc::c_void,
                        sortbuf_count
                            .wrapping_mul(
                                ::std::mem::size_of::<ucs4_with_ccc>() as libc::c_ulong,
                            ),
                    );
                    if sortbuf != sortbuf_preallocated.as_mut_ptr() {
                        rpl_free(sortbuf as *mut libc::c_void);
                    }
                    sortbuf = new_sortbuf;
                }
            }
            (*sortbuf.offset(sortbuf_count as isize)).code = uc;
            (*sortbuf.offset(sortbuf_count as isize)).ccc = ccc;
            sortbuf_count = sortbuf_count.wrapping_add(1);
            sortbuf_count;
            i += 1;
            i;
        }
        if !(s < s_end) {
            current_block = 16314074004867283505;
            break;
        }
        s = s.offset(count as isize);
    }
    match current_block {
        16314074004867283505 => {
            if length == 0 as libc::c_int as libc::c_ulong {
                if result.is_null() {
                    result = malloc(1 as libc::c_int as libc::c_ulong) as *mut uint8_t;
                    if result.is_null() {
                        *__errno_location() = 12 as libc::c_int;
                        current_block = 11223704093569886599;
                    } else {
                        current_block = 18201902862271706575;
                    }
                } else {
                    current_block = 18201902862271706575;
                }
            } else {
                if result != resultbuf && length < allocated {
                    let mut memory: *mut uint8_t = 0 as *mut uint8_t;
                    memory = realloc(
                        result as *mut libc::c_void,
                        length
                            .wrapping_mul(
                                ::std::mem::size_of::<uint8_t>() as libc::c_ulong,
                            ),
                    ) as *mut uint8_t;
                    if !memory.is_null() {
                        result = memory;
                    }
                }
                current_block = 18201902862271706575;
            }
            match current_block {
                11223704093569886599 => {}
                _ => {
                    if sortbuf_count > 0 as libc::c_int as libc::c_ulong {
                        abort();
                    }
                    if sortbuf != sortbuf_preallocated.as_mut_ptr() {
                        rpl_free(sortbuf as *mut libc::c_void);
                    }
                    *lengthp = length;
                    return result;
                }
            }
        }
        _ => {}
    }
    let mut saved_errno: libc::c_int = *__errno_location();
    if sortbuf != sortbuf_preallocated.as_mut_ptr() {
        rpl_free(sortbuf as *mut libc::c_void);
    }
    if result != resultbuf {
        rpl_free(result as *mut libc::c_void);
    }
    *__errno_location() = saved_errno;
    return 0 as *mut uint8_t;
}
