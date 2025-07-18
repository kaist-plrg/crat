use ::libc;
extern "C" {
    fn abort() -> !;
}
pub type size_t = libc::c_ulong;
pub unsafe extern "C" fn __libc_dynarray_at_failure(
    mut size: size_t,
    mut index: size_t,
) -> ! {
    abort();
}
