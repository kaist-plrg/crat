use ::libc;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ZopfliOptions {
    pub verbose: libc::c_int,
    pub verbose_more: libc::c_int,
    pub numiterations: libc::c_int,
    pub blocksplitting: libc::c_int,
    pub blocksplittinglast: libc::c_int,
    pub blocksplittingmax: libc::c_int,
}
pub unsafe extern "C" fn ZopfliInitOptions(mut options: *mut ZopfliOptions) {
    (*options).verbose = 0 as libc::c_int;
    (*options).verbose_more = 0 as libc::c_int;
    (*options).numiterations = 15 as libc::c_int;
    (*options).blocksplitting = 1 as libc::c_int;
    (*options).blocksplittinglast = 0 as libc::c_int;
    (*options).blocksplittingmax = 15 as libc::c_int;
}
