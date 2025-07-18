use ::libc;
extern "C" {
    fn malloc(_: libc::c_ulong) -> *mut libc::c_void;
    fn free(__ptr: *mut libc::c_void);
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ti_buffer {
    pub size: libc::c_int,
    pub pushes: libc::c_int,
    pub index: libc::c_int,
    pub sum: libc::c_double,
    pub vals: [libc::c_double; 1],
}
pub unsafe extern "C" fn ti_buffer_new(mut size: libc::c_int) -> *mut ti_buffer {
    let s: libc::c_int = ::std::mem::size_of::<ti_buffer>() as libc::c_ulong
        as libc::c_int
        + (size - 1 as libc::c_int)
            * ::std::mem::size_of::<libc::c_double>() as libc::c_ulong as libc::c_int;
    let mut ret: *mut ti_buffer = malloc(s as libc::c_uint as libc::c_ulong)
        as *mut ti_buffer;
    (*ret).size = size;
    (*ret).pushes = 0 as libc::c_int;
    (*ret).index = 0 as libc::c_int;
    (*ret).sum = 0 as libc::c_int as libc::c_double;
    return ret;
}
pub unsafe extern "C" fn ti_buffer_free(mut buffer: *mut ti_buffer) {
    free(buffer as *mut libc::c_void);
}
