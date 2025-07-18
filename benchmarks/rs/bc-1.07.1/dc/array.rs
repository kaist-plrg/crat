use ::libc;
extern "C" {
    pub type bc_struct;
    pub type dc_string;
    fn free(__ptr: *mut libc::c_void);
    fn dc_malloc(_: size_t) -> *mut libc::c_void;
    fn dc_get_stacked_array(_: libc::c_int) -> *mut dc_array;
    fn dc_free_num(_: *mut dc_num);
    fn dc_free_str(_: *mut dc_str);
    fn dc_garbage(_: *const libc::c_char, _: libc::c_int);
    fn dc_set_stacked_array(_: libc::c_int, _: *mut dc_array);
    fn dc_dup(_: dc_data) -> dc_data;
    fn dc_int2data(_: libc::c_int) -> dc_data;
}
pub type size_t = libc::c_ulong;
pub type dc_value_type = libc::c_uint;
pub const DC_STRING: dc_value_type = 2;
pub const DC_NUMBER: dc_value_type = 1;
pub const DC_UNINITIALIZED: dc_value_type = 0;
pub type dc_num = *mut bc_struct;
pub type dc_str = *mut dc_string;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct dc_data {
    pub dc_type: dc_value_type,
    pub v: C2RustUnnamed,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed {
    pub number: dc_num,
    pub string: dc_str,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct dc_array {
    pub Index: libc::c_int,
    pub value: dc_data,
    pub next: *mut dc_array,
}
pub unsafe extern "C" fn dc_array_init() {}
pub unsafe extern "C" fn dc_array_set(
    mut array_id: libc::c_int,
    mut Index: libc::c_int,
    mut value: dc_data,
) {
    let mut cur: *mut dc_array = 0 as *mut dc_array;
    let mut prev: *mut dc_array = 0 as *mut dc_array;
    cur = dc_get_stacked_array(array_id);
    while !cur.is_null() && (*cur).Index < Index {
        prev = cur;
        cur = (*cur).next;
    }
    if !cur.is_null() && (*cur).Index == Index {
        if (*cur).value.dc_type as libc::c_uint
            == DC_NUMBER as libc::c_int as libc::c_uint
        {
            dc_free_num(&mut (*cur).value.v.number);
        } else if (*cur).value.dc_type as libc::c_uint
            == DC_STRING as libc::c_int as libc::c_uint
        {
            dc_free_str(&mut (*cur).value.v.string);
        } else {
            dc_garbage(b" in array\0" as *const u8 as *const libc::c_char, array_id);
        }
        (*cur).value = value;
    } else {
        let mut newentry: *mut dc_array = dc_malloc(
            ::std::mem::size_of::<dc_array>() as libc::c_ulong,
        ) as *mut dc_array;
        (*newentry).Index = Index;
        (*newentry).value = value;
        (*newentry).next = cur;
        if !prev.is_null() {
            (*prev).next = newentry;
        } else {
            dc_set_stacked_array(array_id, newentry);
        }
    };
}
pub unsafe extern "C" fn dc_array_get(
    mut array_id: libc::c_int,
    mut Index: libc::c_int,
) -> dc_data {
    let mut cur: *mut dc_array = dc_get_stacked_array(array_id);
    while !cur.is_null() && (*cur).Index < Index {
        cur = (*cur).next;
    }
    if !cur.is_null() && (*cur).Index == Index {
        return dc_dup((*cur).value);
    }
    return dc_int2data(0 as libc::c_int);
}
pub unsafe extern "C" fn dc_array_free(mut a_head: *mut dc_array) {
    let mut cur: *mut dc_array = 0 as *mut dc_array;
    let mut next: *mut dc_array = 0 as *mut dc_array;
    cur = a_head;
    while !cur.is_null() {
        next = (*cur).next;
        if (*cur).value.dc_type as libc::c_uint
            == DC_NUMBER as libc::c_int as libc::c_uint
        {
            dc_free_num(&mut (*cur).value.v.number);
        } else if (*cur).value.dc_type as libc::c_uint
            == DC_STRING as libc::c_int as libc::c_uint
        {
            dc_free_str(&mut (*cur).value.v.string);
        } else {
            dc_garbage(
                b"in stack\0" as *const u8 as *const libc::c_char,
                -(1 as libc::c_int),
            );
        }
        free(cur as *mut libc::c_void);
        cur = next;
    }
}
