use ::libc;
extern "C" {
    fn malloc(_: libc::c_ulong) -> *mut libc::c_void;
    fn strlen(_: *const libc::c_char) -> libc::c_ulong;
    fn mempcpy(
        _: *mut libc::c_void,
        _: *const libc::c_void,
        _: libc::c_ulong,
    ) -> *mut libc::c_void;
    fn last_component(filename: *const libc::c_char) -> *mut libc::c_char;
    fn base_len(filename: *const libc::c_char) -> size_t;
}
pub type size_t = libc::c_ulong;
pub unsafe extern "C" fn mfile_name_concat(
    mut dir: *const libc::c_char,
    mut base: *const libc::c_char,
    mut base_in_result: *mut *mut libc::c_char,
) -> *mut libc::c_char {
    let mut dirbase: *const libc::c_char = last_component(dir);
    let mut dirbaselen: size_t = base_len(dirbase);
    let mut dirlen: size_t = (dirbase.offset_from(dir) as libc::c_long as libc::c_ulong)
        .wrapping_add(dirbaselen);
    let mut baselen: size_t = strlen(base);
    let mut sep: libc::c_char = '\0' as i32 as libc::c_char;
    if dirbaselen != 0 {
        if !(*dir.offset(dirlen.wrapping_sub(1 as libc::c_int as libc::c_ulong) as isize)
            as libc::c_int == '/' as i32) && !(*base as libc::c_int == '/' as i32)
        {
            sep = '/' as i32 as libc::c_char;
        }
    } else if *base as libc::c_int == '/' as i32 {
        sep = '.' as i32 as libc::c_char;
    }
    let mut p_concat: *mut libc::c_char = malloc(
        dirlen
            .wrapping_add(
                (sep as libc::c_int != '\0' as i32) as libc::c_int as libc::c_ulong,
            )
            .wrapping_add(baselen)
            .wrapping_add(1 as libc::c_int as libc::c_ulong),
    ) as *mut libc::c_char;
    if p_concat.is_null() {
        return 0 as *mut libc::c_char;
    }
    let mut p: *mut libc::c_char = 0 as *mut libc::c_char;
    p = mempcpy(p_concat as *mut libc::c_void, dir as *const libc::c_void, dirlen)
        as *mut libc::c_char;
    *p = sep;
    p = p.offset((sep as libc::c_int != '\0' as i32) as libc::c_int as isize);
    if !base_in_result.is_null() {
        *base_in_result = p;
    }
    p = mempcpy(p as *mut libc::c_void, base as *const libc::c_void, baselen)
        as *mut libc::c_char;
    *p = '\0' as i32 as libc::c_char;
    return p_concat;
}
