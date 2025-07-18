use ::libc;
extern "C" {
    fn __errno_location() -> *mut libc::c_int;
    fn malloc(_: libc::c_ulong) -> *mut libc::c_void;
    fn realloc(_: *mut libc::c_void, _: libc::c_ulong) -> *mut libc::c_void;
    fn free(__ptr: *mut libc::c_void);
    fn show_strerror(filename: *const libc::c_char, errcode: libc::c_int);
    fn set_error_msg(msg: *const libc::c_char);
    fn disable_interrupts();
    fn enable_interrupts();
}
pub type Bool = libc::c_uint;
pub const true_0: Bool = 1;
pub const false_0: Bool = 0;
pub type bool_0 = Bool;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct line {
    pub q_forw: *mut line,
    pub q_back: *mut line,
    pub pos: libc::c_long,
    pub len: libc::c_int,
}
pub type line_t = line;
static mut mem_msg: *const libc::c_char = b"Memory exhausted\0" as *const u8
    as *const libc::c_char;
static mut active_list: *mut *const line_t = 0 as *const *const line_t
    as *mut *const line_t;
static mut active_size: libc::c_int = 0 as libc::c_int;
static mut active_len: libc::c_int = 0 as libc::c_int;
static mut active_idx: libc::c_int = 0 as libc::c_int;
static mut active_idxm: libc::c_int = 0 as libc::c_int;
pub unsafe extern "C" fn clear_active_list() {
    disable_interrupts();
    if !active_list.is_null() {
        free(active_list as *mut libc::c_void);
    }
    active_list = 0 as *mut *const line_t;
    active_idxm = 0 as libc::c_int;
    active_idx = active_idxm;
    active_len = active_idx;
    active_size = active_len;
    enable_interrupts();
}
pub unsafe extern "C" fn next_active_node() -> *const line_t {
    while active_idx < active_len && (*active_list.offset(active_idx as isize)).is_null()
    {
        active_idx += 1;
        active_idx;
    }
    return if active_idx < active_len {
        let fresh0 = active_idx;
        active_idx = active_idx + 1;
        *active_list.offset(fresh0 as isize)
    } else {
        0 as *const line_t
    };
}
pub unsafe extern "C" fn set_active_node(lp: *const line_t) -> bool_0 {
    let min_size: libc::c_uint = ((active_len + 1 as libc::c_int) as libc::c_ulong)
        .wrapping_mul(::std::mem::size_of::<*mut *mut line_t>() as libc::c_ulong)
        as libc::c_uint;
    if (active_size as libc::c_uint) < min_size {
        if min_size >= 2147483647 as libc::c_int as libc::c_uint {
            set_error_msg(
                b"Too many matching lines\0" as *const u8 as *const libc::c_char,
            );
            return false_0;
        }
        let new_size: libc::c_int = (if min_size < 512 as libc::c_int as libc::c_uint {
            512 as libc::c_int as libc::c_uint
        } else if min_size
            >= (2147483647 as libc::c_int / 2 as libc::c_int) as libc::c_uint
        {
            (2147483647 as libc::c_int - 1 as libc::c_int) as libc::c_uint
        } else {
            min_size
                .wrapping_div(512 as libc::c_int as libc::c_uint)
                .wrapping_mul(1024 as libc::c_int as libc::c_uint)
        }) as libc::c_int;
        let mut new_buf: *mut libc::c_void = 0 as *mut libc::c_void;
        disable_interrupts();
        if !active_list.is_null() {
            new_buf = realloc(
                active_list as *mut libc::c_void,
                new_size as libc::c_ulong,
            );
        } else {
            new_buf = malloc(new_size as libc::c_ulong);
        }
        if new_buf.is_null() {
            show_strerror(0 as *const libc::c_char, *__errno_location());
            set_error_msg(mem_msg);
            enable_interrupts();
            return false_0;
        }
        active_size = new_size;
        active_list = new_buf as *mut *const line_t;
        enable_interrupts();
    }
    let fresh1 = active_len;
    active_len = active_len + 1;
    let ref mut fresh2 = *active_list.offset(fresh1 as isize);
    *fresh2 = lp;
    return true_0;
}
pub unsafe extern "C" fn unset_active_nodes(mut bp: *const line_t, ep: *const line_t) {
    while bp != ep {
        let mut i: libc::c_int = 0;
        i = 0 as libc::c_int;
        while i < active_len {
            active_idxm += 1;
            if active_idxm >= active_len {
                active_idxm = 0 as libc::c_int;
            }
            if *active_list.offset(active_idxm as isize) == bp {
                let ref mut fresh3 = *active_list.offset(active_idxm as isize);
                *fresh3 = 0 as *const line_t;
                break;
            } else {
                i += 1;
                i;
            }
        }
        bp = (*bp).q_forw;
    }
}
