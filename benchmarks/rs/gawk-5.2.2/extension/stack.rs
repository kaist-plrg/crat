use ::libc;
extern "C" {
    fn malloc(_: libc::c_ulong) -> *mut libc::c_void;
    fn realloc(_: *mut libc::c_void, _: libc::c_ulong) -> *mut libc::c_void;
}
pub type size_t = libc::c_ulong;
static mut size: size_t = 0;
static mut stack: *mut *mut libc::c_void = 0 as *const *mut libc::c_void
    as *mut *mut libc::c_void;
static mut index: libc::c_int = -(1 as libc::c_int);
pub unsafe extern "C" fn stack_empty() -> libc::c_int {
    return (index < 0 as libc::c_int) as libc::c_int;
}
pub unsafe extern "C" fn stack_top() -> *mut libc::c_void {
    if stack_empty() != 0 || stack.is_null() {
        return 0 as *mut libc::c_void;
    }
    return *stack.offset(index as isize);
}
pub unsafe extern "C" fn stack_pop() -> *mut libc::c_void {
    if stack_empty() != 0 || stack.is_null() {
        return 0 as *mut libc::c_void;
    }
    let fresh0 = index;
    index = index - 1;
    return *stack.offset(fresh0 as isize);
}
pub unsafe extern "C" fn stack_push(mut object: *mut libc::c_void) -> libc::c_int {
    let mut new_stack: *mut *mut libc::c_void = 0 as *mut *mut libc::c_void;
    let mut new_size: size_t = (2 as libc::c_int as libc::c_ulong).wrapping_mul(size);
    if stack.is_null() {
        stack = malloc(
            (20 as libc::c_int as libc::c_ulong)
                .wrapping_mul(
                    ::std::mem::size_of::<*mut libc::c_void>() as libc::c_ulong,
                ),
        ) as *mut *mut libc::c_void;
        if stack.is_null() {
            return 0 as libc::c_int;
        }
        size = 20 as libc::c_int as size_t;
    } else if (index + 1 as libc::c_int) as libc::c_ulong >= size {
        if new_size < size {
            return 0 as libc::c_int;
        }
        new_stack = realloc(
            stack as *mut libc::c_void,
            new_size
                .wrapping_mul(
                    ::std::mem::size_of::<*mut libc::c_void>() as libc::c_ulong,
                ),
        ) as *mut *mut libc::c_void;
        if new_stack.is_null() {
            return 0 as libc::c_int;
        }
        size = new_size;
        stack = new_stack;
    }
    index += 1;
    let ref mut fresh1 = *stack.offset(index as isize);
    *fresh1 = object;
    return 1 as libc::c_int;
}
