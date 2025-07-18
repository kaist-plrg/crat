use ::libc;
extern "C" {
    fn malloc(_: libc::c_ulong) -> *mut libc::c_void;
    fn strlen(_: *const libc::c_char) -> libc::c_ulong;
    fn strchr(_: *const libc::c_char, _: libc::c_int) -> *mut libc::c_char;
    static mut osip_malloc_func: Option::<osip_malloc_func_t>;
    fn osip_clrncpy(
        dst: *mut libc::c_char,
        src: *const libc::c_char,
        len: size_t,
    ) -> *mut libc::c_char;
    fn __osip_generic_param_parseall(
        gen_params: *mut osip_list_t,
        params: *const libc::c_char,
    ) -> libc::c_int;
}
pub type size_t = libc::c_ulong;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct __node {
    pub next: *mut __node_t,
    pub element: *mut libc::c_void,
}
pub type __node_t = __node;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct osip_list {
    pub nb_elt: libc::c_int,
    pub node: *mut __node_t,
}
pub type osip_list_t = osip_list;
pub type osip_malloc_func_t = unsafe extern "C" fn(size_t) -> *mut libc::c_void;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct osip_call_info {
    pub element: *mut libc::c_char,
    pub gen_params: osip_list_t,
}
pub type osip_call_info_t = osip_call_info;
pub type osip_content_disposition_t = osip_call_info_t;
pub unsafe extern "C" fn osip_content_disposition_parse(
    mut cd: *mut osip_content_disposition_t,
    mut hvalue: *const libc::c_char,
) -> libc::c_int {
    let mut cd_params: *const libc::c_char = 0 as *const libc::c_char;
    let mut i: libc::c_int = 0;
    cd_params = strchr(hvalue, ';' as i32);
    if !cd_params.is_null() {
        i = __osip_generic_param_parseall(&mut (*cd).gen_params, cd_params);
        if i != 0 as libc::c_int {
            return i;
        }
    } else {
        cd_params = hvalue.offset(strlen(hvalue) as isize);
    }
    if (cd_params.offset_from(hvalue) as libc::c_long + 1 as libc::c_int as libc::c_long)
        < 2 as libc::c_int as libc::c_long
    {
        return -(5 as libc::c_int);
    }
    (*cd)
        .element = (if osip_malloc_func.is_some() {
        osip_malloc_func
            .unwrap()(
            (cd_params.offset_from(hvalue) as libc::c_long
                + 1 as libc::c_int as libc::c_long) as size_t,
        )
    } else {
        malloc(
            (cd_params.offset_from(hvalue) as libc::c_long
                + 1 as libc::c_int as libc::c_long) as libc::c_ulong,
        )
    }) as *mut libc::c_char;
    if ((*cd).element).is_null() {
        return -(4 as libc::c_int);
    }
    osip_clrncpy(
        (*cd).element,
        hvalue,
        cd_params.offset_from(hvalue) as libc::c_long as size_t,
    );
    return 0 as libc::c_int;
}
