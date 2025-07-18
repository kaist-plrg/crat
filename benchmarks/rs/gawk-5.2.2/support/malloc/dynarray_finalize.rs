use ::libc;
extern "C" {
    fn memcpy(
        _: *mut libc::c_void,
        _: *const libc::c_void,
        _: libc::c_ulong,
    ) -> *mut libc::c_void;
    fn free(__ptr: *mut libc::c_void);
    fn malloc(_: libc::c_ulong) -> *mut libc::c_void;
}
pub type size_t = libc::c_ulong;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct dynarray_header {
    pub used: size_t,
    pub allocated: size_t,
    pub array: *mut libc::c_void,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct dynarray_finalize_result {
    pub array: *mut libc::c_void,
    pub length: size_t,
}
#[inline]
unsafe extern "C" fn __dynarray_error_marker() -> size_t {
    return -(1 as libc::c_int) as size_t;
}
#[inline]
unsafe extern "C" fn __dynarray_error(mut list: *mut dynarray_header) -> bool {
    return (*list).allocated == __dynarray_error_marker();
}
pub unsafe extern "C" fn __libc_dynarray_finalize(
    mut list: *mut dynarray_header,
    mut scratch: *mut libc::c_void,
    mut element_size: size_t,
    mut result: *mut dynarray_finalize_result,
) -> bool {
    if __dynarray_error(list) {
        return 0 as libc::c_int != 0;
    }
    let mut used: size_t = (*list).used;
    if used == 0 as libc::c_int as libc::c_ulong {
        if (*list).array != scratch {
            free((*list).array);
        }
        *result = {
            let mut init = dynarray_finalize_result {
                array: 0 as *mut libc::c_void,
                length: 0 as libc::c_int as size_t,
            };
            init
        };
        return 1 as libc::c_int != 0;
    }
    let mut allocation_size: size_t = used.wrapping_mul(element_size);
    let mut heap_array: *mut libc::c_void = malloc(allocation_size);
    if !heap_array.is_null() {
        if !((*list).array).is_null() {
            memcpy(heap_array, (*list).array, allocation_size);
        }
        if (*list).array != scratch {
            free((*list).array);
        }
        *result = {
            let mut init = dynarray_finalize_result {
                array: heap_array,
                length: used,
            };
            init
        };
        return 1 as libc::c_int != 0;
    } else {
        return 0 as libc::c_int != 0
    };
}
