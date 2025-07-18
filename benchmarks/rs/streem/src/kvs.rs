use ::libc;
extern "C" {
    pub type strm_queue;
    pub type node_error;
    fn sprintf(_: *mut libc::c_char, _: *const libc::c_char, _: ...) -> libc::c_int;
    fn malloc(_: libc::c_ulong) -> *mut libc::c_void;
    fn calloc(_: libc::c_ulong, _: libc::c_ulong) -> *mut libc::c_void;
    fn realloc(_: *mut libc::c_void, _: libc::c_ulong) -> *mut libc::c_void;
    fn free(__ptr: *mut libc::c_void);
    fn memset(
        _: *mut libc::c_void,
        _: libc::c_int,
        _: libc::c_ulong,
    ) -> *mut libc::c_void;
    fn strm_cfunc_value(_: strm_cfunc) -> strm_value;
    fn strm_nil_value() -> strm_value;
    fn strm_value_ptr(_: strm_value, _: strm_ptr_type) -> *mut libc::c_void;
    fn strm_ptr_value(_: *mut libc::c_void) -> strm_value;
    fn strm_str_new(_: *const libc::c_char, _: strm_int) -> strm_string;
    fn strm_str_intern_str(s: strm_string) -> strm_string;
    fn strm_to_str(v: strm_value) -> strm_string;
    fn strm_raise(_: *mut strm_stream, _: *const libc::c_char);
    fn strm_funcall(
        _: *mut strm_stream,
        _: strm_value,
        _: libc::c_int,
        _: *mut strm_value,
        _: *mut strm_value,
    ) -> libc::c_int;
    fn strm_ns_new(_: *mut strm_state, _: *const libc::c_char) -> *mut strm_state;
    fn strm_var_def(
        _: *mut strm_state,
        _: *const libc::c_char,
        _: strm_value,
    ) -> libc::c_int;
    fn pthread_mutex_init(
        __mutex: *mut pthread_mutex_t,
        __mutexattr: *const pthread_mutexattr_t,
    ) -> libc::c_int;
    fn pthread_mutex_lock(__mutex: *mut pthread_mutex_t) -> libc::c_int;
    fn pthread_mutex_unlock(__mutex: *mut pthread_mutex_t) -> libc::c_int;
}
pub type size_t = libc::c_ulong;
pub type __int32_t = libc::c_int;
pub type __uint32_t = libc::c_uint;
pub type __uint64_t = libc::c_ulong;
pub type __intptr_t = libc::c_long;
pub type int32_t = __int32_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct __pthread_internal_list {
    pub __prev: *mut __pthread_internal_list,
    pub __next: *mut __pthread_internal_list,
}
pub type __pthread_list_t = __pthread_internal_list;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct __pthread_mutex_s {
    pub __lock: libc::c_int,
    pub __count: libc::c_uint,
    pub __owner: libc::c_int,
    pub __nusers: libc::c_uint,
    pub __kind: libc::c_int,
    pub __spins: libc::c_short,
    pub __elision: libc::c_short,
    pub __list: __pthread_list_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union pthread_mutexattr_t {
    pub __size: [libc::c_char; 4],
    pub __align: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union pthread_mutex_t {
    pub __data: __pthread_mutex_s,
    pub __size: [libc::c_char; 40],
    pub __align: libc::c_long,
}
pub type intptr_t = __intptr_t;
pub type uint32_t = __uint32_t;
pub type uint64_t = __uint64_t;
pub type strm_value = uint64_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct strm_state {
    pub env: *mut libc::c_void,
    pub prev: *mut strm_state,
    pub flags: uint32_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct strm_stream {
    pub type_0: strm_ptr_type,
    pub flags: libc::c_uint,
    pub mode: strm_stream_mode,
    pub start_func: strm_callback,
    pub close_func: strm_callback,
    pub data: *mut libc::c_void,
    pub dst: *mut strm_stream,
    pub rest: *mut *mut strm_stream,
    pub rsize: size_t,
    pub rcapa: size_t,
    pub exc: *mut node_error,
    pub refcnt: strm_int,
    pub queue: *mut strm_queue,
    pub excl: strm_int,
}
pub type strm_int = int32_t;
pub type strm_callback = Option::<
    unsafe extern "C" fn(*mut strm_stream, strm_value) -> libc::c_int,
>;
pub type strm_stream_mode = libc::c_uint;
pub const strm_killed: strm_stream_mode = 4;
pub const strm_dying: strm_stream_mode = 3;
pub const strm_consumer: strm_stream_mode = 2;
pub const strm_filter: strm_stream_mode = 1;
pub const strm_producer: strm_stream_mode = 0;
pub type strm_ptr_type = libc::c_uint;
pub const STRM_PTR_AUX: strm_ptr_type = 4;
pub const STRM_PTR_IO: strm_ptr_type = 3;
pub const STRM_PTR_GENFUNC: strm_ptr_type = 2;
pub const STRM_PTR_LAMBDA: strm_ptr_type = 1;
pub const STRM_PTR_STREAM: strm_ptr_type = 0;
pub type strm_cfunc = Option::<
    unsafe extern "C" fn(
        *mut strm_stream,
        libc::c_int,
        *mut strm_value,
        *mut strm_value,
    ) -> libc::c_int,
>;
pub type strm_string = uint64_t;
pub type khint32_t = libc::c_uint;
pub type khint_t = khint32_t;
pub type khiter_t = khint_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct kh_kvs_s {
    pub n_buckets: khint_t,
    pub size: khint_t,
    pub n_occupied: khint_t,
    pub upper_bound: khint_t,
    pub flags: *mut khint32_t,
    pub keys: *mut strm_string,
    pub vals: *mut strm_value,
}
pub type kh_kvs_t = kh_kvs_s;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct kh_txn_s {
    pub n_buckets: khint_t,
    pub size: khint_t,
    pub n_occupied: khint_t,
    pub upper_bound: khint_t,
    pub flags: *mut khint32_t,
    pub keys: *mut strm_string,
    pub vals: *mut strm_value,
}
pub type kh_txn_t = kh_txn_s;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct strm_kvs {
    pub type_0: strm_ptr_type,
    pub ns: *mut strm_state,
    pub kv: *mut kh_kvs_t,
    pub serial: uint64_t,
    pub lock: pthread_mutex_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct strm_txn {
    pub type_0: strm_ptr_type,
    pub ns: *mut strm_state,
    pub tv: *mut kh_txn_t,
    pub kvs: *mut strm_kvs,
    pub serial: uint64_t,
}
static mut khash_ac_HASH_UPPER: libc::c_double = 0.77f64;
#[inline]
unsafe extern "C" fn kh_put_kvs(
    mut h: *mut kh_kvs_t,
    mut key: strm_string,
    mut ret: *mut libc::c_int,
) -> khint_t {
    let mut x: khint_t = 0;
    if (*h).n_occupied >= (*h).upper_bound {
        if (*h).n_buckets > (*h).size << 1 as libc::c_int {
            if kh_resize_kvs(
                h,
                ((*h).n_buckets).wrapping_sub(1 as libc::c_int as libc::c_uint),
            ) < 0 as libc::c_int
            {
                *ret = -(1 as libc::c_int);
                return (*h).n_buckets;
            }
        } else if kh_resize_kvs(
            h,
            ((*h).n_buckets).wrapping_add(1 as libc::c_int as libc::c_uint),
        ) < 0 as libc::c_int
        {
            *ret = -(1 as libc::c_int);
            return (*h).n_buckets;
        }
    }
    let mut k: khint_t = 0;
    let mut i: khint_t = 0;
    let mut site: khint_t = 0;
    let mut last: khint_t = 0;
    let mut mask: khint_t = ((*h).n_buckets)
        .wrapping_sub(1 as libc::c_int as libc::c_uint);
    let mut step: khint_t = 0 as libc::c_int as khint_t;
    site = (*h).n_buckets;
    x = site;
    k = (key >> 33 as libc::c_int ^ key ^ key << 11 as libc::c_int) as khint32_t;
    i = k & mask;
    if *((*h).flags).offset((i >> 4 as libc::c_int) as isize)
        >> ((i & 0xf as libc::c_uint) << 1 as libc::c_int)
        & 2 as libc::c_int as libc::c_uint != 0
    {
        x = i;
    } else {
        last = i;
        while *((*h).flags).offset((i >> 4 as libc::c_int) as isize)
            >> ((i & 0xf as libc::c_uint) << 1 as libc::c_int)
            & 2 as libc::c_int as libc::c_uint == 0
            && (*((*h).flags).offset((i >> 4 as libc::c_int) as isize)
                >> ((i & 0xf as libc::c_uint) << 1 as libc::c_int)
                & 1 as libc::c_int as libc::c_uint != 0
                || !(*((*h).keys).offset(i as isize) == key))
        {
            if *((*h).flags).offset((i >> 4 as libc::c_int) as isize)
                >> ((i & 0xf as libc::c_uint) << 1 as libc::c_int)
                & 1 as libc::c_int as libc::c_uint != 0
            {
                site = i;
            }
            step = step.wrapping_add(1);
            i = i.wrapping_add(step) & mask;
            if !(i == last) {
                continue;
            }
            x = site;
            break;
        }
        if x == (*h).n_buckets {
            if *((*h).flags).offset((i >> 4 as libc::c_int) as isize)
                >> ((i & 0xf as libc::c_uint) << 1 as libc::c_int)
                & 2 as libc::c_int as libc::c_uint != 0 && site != (*h).n_buckets
            {
                x = site;
            } else {
                x = i;
            }
        }
    }
    if *((*h).flags).offset((x >> 4 as libc::c_int) as isize)
        >> ((x & 0xf as libc::c_uint) << 1 as libc::c_int)
        & 2 as libc::c_int as libc::c_uint != 0
    {
        *((*h).keys).offset(x as isize) = key;
        let ref mut fresh0 = *((*h).flags).offset((x >> 4 as libc::c_int) as isize);
        *fresh0 = (*fresh0 as libc::c_ulong
            & !((3 as libc::c_ulong) << ((x & 0xf as libc::c_uint) << 1 as libc::c_int)))
            as khint32_t;
        (*h).size = ((*h).size).wrapping_add(1);
        (*h).size;
        (*h).n_occupied = ((*h).n_occupied).wrapping_add(1);
        (*h).n_occupied;
        *ret = 1 as libc::c_int;
    } else if *((*h).flags).offset((x >> 4 as libc::c_int) as isize)
        >> ((x & 0xf as libc::c_uint) << 1 as libc::c_int)
        & 1 as libc::c_int as libc::c_uint != 0
    {
        *((*h).keys).offset(x as isize) = key;
        let ref mut fresh1 = *((*h).flags).offset((x >> 4 as libc::c_int) as isize);
        *fresh1 = (*fresh1 as libc::c_ulong
            & !((3 as libc::c_ulong) << ((x & 0xf as libc::c_uint) << 1 as libc::c_int)))
            as khint32_t;
        (*h).size = ((*h).size).wrapping_add(1);
        (*h).size;
        *ret = 2 as libc::c_int;
    } else {
        *ret = 0 as libc::c_int;
    }
    return x;
}
#[inline]
unsafe extern "C" fn kh_get_kvs(
    mut h: *const kh_kvs_t,
    mut key: strm_string,
) -> khint_t {
    if (*h).n_buckets != 0 {
        let mut k: khint_t = 0;
        let mut i: khint_t = 0;
        let mut last: khint_t = 0;
        let mut mask: khint_t = 0;
        let mut step: khint_t = 0 as libc::c_int as khint_t;
        mask = ((*h).n_buckets).wrapping_sub(1 as libc::c_int as libc::c_uint);
        k = (key >> 33 as libc::c_int ^ key ^ key << 11 as libc::c_int) as khint32_t;
        i = k & mask;
        last = i;
        while *((*h).flags).offset((i >> 4 as libc::c_int) as isize)
            >> ((i & 0xf as libc::c_uint) << 1 as libc::c_int)
            & 2 as libc::c_int as libc::c_uint == 0
            && (*((*h).flags).offset((i >> 4 as libc::c_int) as isize)
                >> ((i & 0xf as libc::c_uint) << 1 as libc::c_int)
                & 1 as libc::c_int as libc::c_uint != 0
                || !(*((*h).keys).offset(i as isize) == key))
        {
            step = step.wrapping_add(1);
            i = i.wrapping_add(step) & mask;
            if i == last {
                return (*h).n_buckets;
            }
        }
        return if *((*h).flags).offset((i >> 4 as libc::c_int) as isize)
            >> ((i & 0xf as libc::c_uint) << 1 as libc::c_int)
            & 3 as libc::c_int as libc::c_uint != 0
        {
            (*h).n_buckets
        } else {
            i
        };
    } else {
        return 0 as libc::c_int as khint_t
    };
}
#[inline]
unsafe extern "C" fn kh_init_kvs() -> *mut kh_kvs_t {
    return calloc(
        1 as libc::c_int as libc::c_ulong,
        ::std::mem::size_of::<kh_kvs_t>() as libc::c_ulong,
    ) as *mut kh_kvs_t;
}
#[inline]
unsafe extern "C" fn kh_resize_kvs(
    mut h: *mut kh_kvs_t,
    mut new_n_buckets: khint_t,
) -> libc::c_int {
    let mut new_flags: *mut khint32_t = 0 as *mut khint32_t;
    let mut j: khint_t = 1 as libc::c_int as khint_t;
    new_n_buckets = new_n_buckets.wrapping_sub(1);
    new_n_buckets;
    new_n_buckets |= new_n_buckets >> 1 as libc::c_int;
    new_n_buckets |= new_n_buckets >> 2 as libc::c_int;
    new_n_buckets |= new_n_buckets >> 4 as libc::c_int;
    new_n_buckets |= new_n_buckets >> 8 as libc::c_int;
    new_n_buckets |= new_n_buckets >> 16 as libc::c_int;
    new_n_buckets = new_n_buckets.wrapping_add(1);
    new_n_buckets;
    if new_n_buckets < 4 as libc::c_int as libc::c_uint {
        new_n_buckets = 4 as libc::c_int as khint_t;
    }
    if (*h).size
        >= (new_n_buckets as libc::c_double * khash_ac_HASH_UPPER + 0.5f64) as khint_t
    {
        j = 0 as libc::c_int as khint_t;
    } else {
        new_flags = malloc(
            ((if new_n_buckets < 16 as libc::c_int as libc::c_uint {
                1 as libc::c_int as libc::c_uint
            } else {
                new_n_buckets >> 4 as libc::c_int
            }) as libc::c_ulong)
                .wrapping_mul(::std::mem::size_of::<khint32_t>() as libc::c_ulong),
        ) as *mut khint32_t;
        if new_flags.is_null() {
            return -(1 as libc::c_int);
        }
        memset(
            new_flags as *mut libc::c_void,
            0xaa as libc::c_int,
            ((if new_n_buckets < 16 as libc::c_int as libc::c_uint {
                1 as libc::c_int as libc::c_uint
            } else {
                new_n_buckets >> 4 as libc::c_int
            }) as libc::c_ulong)
                .wrapping_mul(::std::mem::size_of::<khint32_t>() as libc::c_ulong),
        );
        if (*h).n_buckets < new_n_buckets {
            let mut new_keys: *mut strm_string = realloc(
                (*h).keys as *mut libc::c_void,
                (new_n_buckets as libc::c_ulong)
                    .wrapping_mul(::std::mem::size_of::<strm_string>() as libc::c_ulong),
            ) as *mut strm_string;
            if new_keys.is_null() {
                free(new_flags as *mut libc::c_void);
                return -(1 as libc::c_int);
            }
            (*h).keys = new_keys;
            let mut new_vals: *mut strm_value = realloc(
                (*h).vals as *mut libc::c_void,
                (new_n_buckets as libc::c_ulong)
                    .wrapping_mul(::std::mem::size_of::<strm_value>() as libc::c_ulong),
            ) as *mut strm_value;
            if new_vals.is_null() {
                free(new_flags as *mut libc::c_void);
                return -(1 as libc::c_int);
            }
            (*h).vals = new_vals;
        }
    }
    if j != 0 {
        j = 0 as libc::c_int as khint_t;
        while j != (*h).n_buckets {
            if *((*h).flags).offset((j >> 4 as libc::c_int) as isize)
                >> ((j & 0xf as libc::c_uint) << 1 as libc::c_int)
                & 3 as libc::c_int as libc::c_uint == 0 as libc::c_int as libc::c_uint
            {
                let mut key: strm_string = *((*h).keys).offset(j as isize);
                let mut val: strm_value = 0;
                let mut new_mask: khint_t = 0;
                new_mask = new_n_buckets.wrapping_sub(1 as libc::c_int as libc::c_uint);
                val = *((*h).vals).offset(j as isize);
                let ref mut fresh2 = *((*h).flags)
                    .offset((j >> 4 as libc::c_int) as isize);
                *fresh2 = (*fresh2 as libc::c_ulong
                    | (1 as libc::c_ulong)
                        << ((j & 0xf as libc::c_uint) << 1 as libc::c_int)) as khint32_t;
                loop {
                    let mut k: khint_t = 0;
                    let mut i: khint_t = 0;
                    let mut step: khint_t = 0 as libc::c_int as khint_t;
                    k = (key >> 33 as libc::c_int ^ key ^ key << 11 as libc::c_int)
                        as khint32_t;
                    i = k & new_mask;
                    while *new_flags.offset((i >> 4 as libc::c_int) as isize)
                        >> ((i & 0xf as libc::c_uint) << 1 as libc::c_int)
                        & 2 as libc::c_int as libc::c_uint == 0
                    {
                        step = step.wrapping_add(1);
                        i = i.wrapping_add(step) & new_mask;
                    }
                    let ref mut fresh3 = *new_flags
                        .offset((i >> 4 as libc::c_int) as isize);
                    *fresh3 = (*fresh3 as libc::c_ulong
                        & !((2 as libc::c_ulong)
                            << ((i & 0xf as libc::c_uint) << 1 as libc::c_int)))
                        as khint32_t;
                    if i < (*h).n_buckets
                        && *((*h).flags).offset((i >> 4 as libc::c_int) as isize)
                            >> ((i & 0xf as libc::c_uint) << 1 as libc::c_int)
                            & 3 as libc::c_int as libc::c_uint
                            == 0 as libc::c_int as libc::c_uint
                    {
                        let mut tmp: strm_string = *((*h).keys).offset(i as isize);
                        *((*h).keys).offset(i as isize) = key;
                        key = tmp;
                        let mut tmp_0: strm_value = *((*h).vals).offset(i as isize);
                        *((*h).vals).offset(i as isize) = val;
                        val = tmp_0;
                        let ref mut fresh4 = *((*h).flags)
                            .offset((i >> 4 as libc::c_int) as isize);
                        *fresh4 = (*fresh4 as libc::c_ulong
                            | (1 as libc::c_ulong)
                                << ((i & 0xf as libc::c_uint) << 1 as libc::c_int))
                            as khint32_t;
                    } else {
                        *((*h).keys).offset(i as isize) = key;
                        *((*h).vals).offset(i as isize) = val;
                        break;
                    }
                }
            }
            j = j.wrapping_add(1);
            j;
        }
        if (*h).n_buckets > new_n_buckets {
            (*h)
                .keys = realloc(
                (*h).keys as *mut libc::c_void,
                (new_n_buckets as libc::c_ulong)
                    .wrapping_mul(::std::mem::size_of::<strm_string>() as libc::c_ulong),
            ) as *mut strm_string;
            (*h)
                .vals = realloc(
                (*h).vals as *mut libc::c_void,
                (new_n_buckets as libc::c_ulong)
                    .wrapping_mul(::std::mem::size_of::<strm_value>() as libc::c_ulong),
            ) as *mut strm_value;
        }
        free((*h).flags as *mut libc::c_void);
        (*h).flags = new_flags;
        (*h).n_buckets = new_n_buckets;
        (*h).n_occupied = (*h).size;
        (*h)
            .upper_bound = ((*h).n_buckets as libc::c_double * khash_ac_HASH_UPPER
            + 0.5f64) as khint_t;
    }
    return 0 as libc::c_int;
}
#[inline]
unsafe extern "C" fn kh_destroy_kvs(mut h: *mut kh_kvs_t) {
    if !h.is_null() {
        free((*h).keys as *mut libc::c_void);
        free((*h).flags as *mut libc::c_void);
        free((*h).vals as *mut libc::c_void);
        free(h as *mut libc::c_void);
    }
}
#[inline]
unsafe extern "C" fn kh_get_txn(
    mut h: *const kh_txn_t,
    mut key: strm_string,
) -> khint_t {
    if (*h).n_buckets != 0 {
        let mut k: khint_t = 0;
        let mut i: khint_t = 0;
        let mut last: khint_t = 0;
        let mut mask: khint_t = 0;
        let mut step: khint_t = 0 as libc::c_int as khint_t;
        mask = ((*h).n_buckets).wrapping_sub(1 as libc::c_int as libc::c_uint);
        k = (key >> 33 as libc::c_int ^ key ^ key << 11 as libc::c_int) as khint32_t;
        i = k & mask;
        last = i;
        while *((*h).flags).offset((i >> 4 as libc::c_int) as isize)
            >> ((i & 0xf as libc::c_uint) << 1 as libc::c_int)
            & 2 as libc::c_int as libc::c_uint == 0
            && (*((*h).flags).offset((i >> 4 as libc::c_int) as isize)
                >> ((i & 0xf as libc::c_uint) << 1 as libc::c_int)
                & 1 as libc::c_int as libc::c_uint != 0
                || !(*((*h).keys).offset(i as isize) == key))
        {
            step = step.wrapping_add(1);
            i = i.wrapping_add(step) & mask;
            if i == last {
                return (*h).n_buckets;
            }
        }
        return if *((*h).flags).offset((i >> 4 as libc::c_int) as isize)
            >> ((i & 0xf as libc::c_uint) << 1 as libc::c_int)
            & 3 as libc::c_int as libc::c_uint != 0
        {
            (*h).n_buckets
        } else {
            i
        };
    } else {
        return 0 as libc::c_int as khint_t
    };
}
#[inline]
unsafe extern "C" fn kh_resize_txn(
    mut h: *mut kh_txn_t,
    mut new_n_buckets: khint_t,
) -> libc::c_int {
    let mut new_flags: *mut khint32_t = 0 as *mut khint32_t;
    let mut j: khint_t = 1 as libc::c_int as khint_t;
    new_n_buckets = new_n_buckets.wrapping_sub(1);
    new_n_buckets;
    new_n_buckets |= new_n_buckets >> 1 as libc::c_int;
    new_n_buckets |= new_n_buckets >> 2 as libc::c_int;
    new_n_buckets |= new_n_buckets >> 4 as libc::c_int;
    new_n_buckets |= new_n_buckets >> 8 as libc::c_int;
    new_n_buckets |= new_n_buckets >> 16 as libc::c_int;
    new_n_buckets = new_n_buckets.wrapping_add(1);
    new_n_buckets;
    if new_n_buckets < 4 as libc::c_int as libc::c_uint {
        new_n_buckets = 4 as libc::c_int as khint_t;
    }
    if (*h).size
        >= (new_n_buckets as libc::c_double * khash_ac_HASH_UPPER + 0.5f64) as khint_t
    {
        j = 0 as libc::c_int as khint_t;
    } else {
        new_flags = malloc(
            ((if new_n_buckets < 16 as libc::c_int as libc::c_uint {
                1 as libc::c_int as libc::c_uint
            } else {
                new_n_buckets >> 4 as libc::c_int
            }) as libc::c_ulong)
                .wrapping_mul(::std::mem::size_of::<khint32_t>() as libc::c_ulong),
        ) as *mut khint32_t;
        if new_flags.is_null() {
            return -(1 as libc::c_int);
        }
        memset(
            new_flags as *mut libc::c_void,
            0xaa as libc::c_int,
            ((if new_n_buckets < 16 as libc::c_int as libc::c_uint {
                1 as libc::c_int as libc::c_uint
            } else {
                new_n_buckets >> 4 as libc::c_int
            }) as libc::c_ulong)
                .wrapping_mul(::std::mem::size_of::<khint32_t>() as libc::c_ulong),
        );
        if (*h).n_buckets < new_n_buckets {
            let mut new_keys: *mut strm_string = realloc(
                (*h).keys as *mut libc::c_void,
                (new_n_buckets as libc::c_ulong)
                    .wrapping_mul(::std::mem::size_of::<strm_string>() as libc::c_ulong),
            ) as *mut strm_string;
            if new_keys.is_null() {
                free(new_flags as *mut libc::c_void);
                return -(1 as libc::c_int);
            }
            (*h).keys = new_keys;
            let mut new_vals: *mut strm_value = realloc(
                (*h).vals as *mut libc::c_void,
                (new_n_buckets as libc::c_ulong)
                    .wrapping_mul(::std::mem::size_of::<strm_value>() as libc::c_ulong),
            ) as *mut strm_value;
            if new_vals.is_null() {
                free(new_flags as *mut libc::c_void);
                return -(1 as libc::c_int);
            }
            (*h).vals = new_vals;
        }
    }
    if j != 0 {
        j = 0 as libc::c_int as khint_t;
        while j != (*h).n_buckets {
            if *((*h).flags).offset((j >> 4 as libc::c_int) as isize)
                >> ((j & 0xf as libc::c_uint) << 1 as libc::c_int)
                & 3 as libc::c_int as libc::c_uint == 0 as libc::c_int as libc::c_uint
            {
                let mut key: strm_string = *((*h).keys).offset(j as isize);
                let mut val: strm_value = 0;
                let mut new_mask: khint_t = 0;
                new_mask = new_n_buckets.wrapping_sub(1 as libc::c_int as libc::c_uint);
                val = *((*h).vals).offset(j as isize);
                let ref mut fresh5 = *((*h).flags)
                    .offset((j >> 4 as libc::c_int) as isize);
                *fresh5 = (*fresh5 as libc::c_ulong
                    | (1 as libc::c_ulong)
                        << ((j & 0xf as libc::c_uint) << 1 as libc::c_int)) as khint32_t;
                loop {
                    let mut k: khint_t = 0;
                    let mut i: khint_t = 0;
                    let mut step: khint_t = 0 as libc::c_int as khint_t;
                    k = (key >> 33 as libc::c_int ^ key ^ key << 11 as libc::c_int)
                        as khint32_t;
                    i = k & new_mask;
                    while *new_flags.offset((i >> 4 as libc::c_int) as isize)
                        >> ((i & 0xf as libc::c_uint) << 1 as libc::c_int)
                        & 2 as libc::c_int as libc::c_uint == 0
                    {
                        step = step.wrapping_add(1);
                        i = i.wrapping_add(step) & new_mask;
                    }
                    let ref mut fresh6 = *new_flags
                        .offset((i >> 4 as libc::c_int) as isize);
                    *fresh6 = (*fresh6 as libc::c_ulong
                        & !((2 as libc::c_ulong)
                            << ((i & 0xf as libc::c_uint) << 1 as libc::c_int)))
                        as khint32_t;
                    if i < (*h).n_buckets
                        && *((*h).flags).offset((i >> 4 as libc::c_int) as isize)
                            >> ((i & 0xf as libc::c_uint) << 1 as libc::c_int)
                            & 3 as libc::c_int as libc::c_uint
                            == 0 as libc::c_int as libc::c_uint
                    {
                        let mut tmp: strm_string = *((*h).keys).offset(i as isize);
                        *((*h).keys).offset(i as isize) = key;
                        key = tmp;
                        let mut tmp_0: strm_value = *((*h).vals).offset(i as isize);
                        *((*h).vals).offset(i as isize) = val;
                        val = tmp_0;
                        let ref mut fresh7 = *((*h).flags)
                            .offset((i >> 4 as libc::c_int) as isize);
                        *fresh7 = (*fresh7 as libc::c_ulong
                            | (1 as libc::c_ulong)
                                << ((i & 0xf as libc::c_uint) << 1 as libc::c_int))
                            as khint32_t;
                    } else {
                        *((*h).keys).offset(i as isize) = key;
                        *((*h).vals).offset(i as isize) = val;
                        break;
                    }
                }
            }
            j = j.wrapping_add(1);
            j;
        }
        if (*h).n_buckets > new_n_buckets {
            (*h)
                .keys = realloc(
                (*h).keys as *mut libc::c_void,
                (new_n_buckets as libc::c_ulong)
                    .wrapping_mul(::std::mem::size_of::<strm_string>() as libc::c_ulong),
            ) as *mut strm_string;
            (*h)
                .vals = realloc(
                (*h).vals as *mut libc::c_void,
                (new_n_buckets as libc::c_ulong)
                    .wrapping_mul(::std::mem::size_of::<strm_value>() as libc::c_ulong),
            ) as *mut strm_value;
        }
        free((*h).flags as *mut libc::c_void);
        (*h).flags = new_flags;
        (*h).n_buckets = new_n_buckets;
        (*h).n_occupied = (*h).size;
        (*h)
            .upper_bound = ((*h).n_buckets as libc::c_double * khash_ac_HASH_UPPER
            + 0.5f64) as khint_t;
    }
    return 0 as libc::c_int;
}
#[inline]
unsafe extern "C" fn kh_put_txn(
    mut h: *mut kh_txn_t,
    mut key: strm_string,
    mut ret: *mut libc::c_int,
) -> khint_t {
    let mut x: khint_t = 0;
    if (*h).n_occupied >= (*h).upper_bound {
        if (*h).n_buckets > (*h).size << 1 as libc::c_int {
            if kh_resize_txn(
                h,
                ((*h).n_buckets).wrapping_sub(1 as libc::c_int as libc::c_uint),
            ) < 0 as libc::c_int
            {
                *ret = -(1 as libc::c_int);
                return (*h).n_buckets;
            }
        } else if kh_resize_txn(
            h,
            ((*h).n_buckets).wrapping_add(1 as libc::c_int as libc::c_uint),
        ) < 0 as libc::c_int
        {
            *ret = -(1 as libc::c_int);
            return (*h).n_buckets;
        }
    }
    let mut k: khint_t = 0;
    let mut i: khint_t = 0;
    let mut site: khint_t = 0;
    let mut last: khint_t = 0;
    let mut mask: khint_t = ((*h).n_buckets)
        .wrapping_sub(1 as libc::c_int as libc::c_uint);
    let mut step: khint_t = 0 as libc::c_int as khint_t;
    site = (*h).n_buckets;
    x = site;
    k = (key >> 33 as libc::c_int ^ key ^ key << 11 as libc::c_int) as khint32_t;
    i = k & mask;
    if *((*h).flags).offset((i >> 4 as libc::c_int) as isize)
        >> ((i & 0xf as libc::c_uint) << 1 as libc::c_int)
        & 2 as libc::c_int as libc::c_uint != 0
    {
        x = i;
    } else {
        last = i;
        while *((*h).flags).offset((i >> 4 as libc::c_int) as isize)
            >> ((i & 0xf as libc::c_uint) << 1 as libc::c_int)
            & 2 as libc::c_int as libc::c_uint == 0
            && (*((*h).flags).offset((i >> 4 as libc::c_int) as isize)
                >> ((i & 0xf as libc::c_uint) << 1 as libc::c_int)
                & 1 as libc::c_int as libc::c_uint != 0
                || !(*((*h).keys).offset(i as isize) == key))
        {
            if *((*h).flags).offset((i >> 4 as libc::c_int) as isize)
                >> ((i & 0xf as libc::c_uint) << 1 as libc::c_int)
                & 1 as libc::c_int as libc::c_uint != 0
            {
                site = i;
            }
            step = step.wrapping_add(1);
            i = i.wrapping_add(step) & mask;
            if !(i == last) {
                continue;
            }
            x = site;
            break;
        }
        if x == (*h).n_buckets {
            if *((*h).flags).offset((i >> 4 as libc::c_int) as isize)
                >> ((i & 0xf as libc::c_uint) << 1 as libc::c_int)
                & 2 as libc::c_int as libc::c_uint != 0 && site != (*h).n_buckets
            {
                x = site;
            } else {
                x = i;
            }
        }
    }
    if *((*h).flags).offset((x >> 4 as libc::c_int) as isize)
        >> ((x & 0xf as libc::c_uint) << 1 as libc::c_int)
        & 2 as libc::c_int as libc::c_uint != 0
    {
        *((*h).keys).offset(x as isize) = key;
        let ref mut fresh8 = *((*h).flags).offset((x >> 4 as libc::c_int) as isize);
        *fresh8 = (*fresh8 as libc::c_ulong
            & !((3 as libc::c_ulong) << ((x & 0xf as libc::c_uint) << 1 as libc::c_int)))
            as khint32_t;
        (*h).size = ((*h).size).wrapping_add(1);
        (*h).size;
        (*h).n_occupied = ((*h).n_occupied).wrapping_add(1);
        (*h).n_occupied;
        *ret = 1 as libc::c_int;
    } else if *((*h).flags).offset((x >> 4 as libc::c_int) as isize)
        >> ((x & 0xf as libc::c_uint) << 1 as libc::c_int)
        & 1 as libc::c_int as libc::c_uint != 0
    {
        *((*h).keys).offset(x as isize) = key;
        let ref mut fresh9 = *((*h).flags).offset((x >> 4 as libc::c_int) as isize);
        *fresh9 = (*fresh9 as libc::c_ulong
            & !((3 as libc::c_ulong) << ((x & 0xf as libc::c_uint) << 1 as libc::c_int)))
            as khint32_t;
        (*h).size = ((*h).size).wrapping_add(1);
        (*h).size;
        *ret = 2 as libc::c_int;
    } else {
        *ret = 0 as libc::c_int;
    }
    return x;
}
#[inline]
unsafe extern "C" fn kh_destroy_txn(mut h: *mut kh_txn_t) {
    if !h.is_null() {
        free((*h).keys as *mut libc::c_void);
        free((*h).flags as *mut libc::c_void);
        free((*h).vals as *mut libc::c_void);
        free(h as *mut libc::c_void);
    }
}
#[inline]
unsafe extern "C" fn kh_init_txn() -> *mut kh_txn_t {
    return calloc(
        1 as libc::c_int as libc::c_ulong,
        ::std::mem::size_of::<kh_txn_t>() as libc::c_ulong,
    ) as *mut kh_txn_t;
}
unsafe extern "C" fn get_kvs(
    mut argc: libc::c_int,
    mut args: *mut strm_value,
) -> *mut strm_kvs {
    if argc == 0 as libc::c_int {
        return 0 as *mut strm_kvs;
    }
    return strm_value_ptr(*args.offset(0 as libc::c_int as isize), STRM_PTR_AUX)
        as *mut strm_kvs;
}
unsafe extern "C" fn kvs_get(
    mut strm: *mut strm_stream,
    mut argc: libc::c_int,
    mut args: *mut strm_value,
    mut ret: *mut strm_value,
) -> libc::c_int {
    let mut k: *mut strm_kvs = get_kvs(argc, args);
    let mut key: strm_string = strm_str_intern_str(
        strm_to_str(*args.offset(1 as libc::c_int as isize)),
    );
    let mut i: khiter_t = 0;
    if k.is_null() {
        strm_raise(strm, b"no kvs given\0" as *const u8 as *const libc::c_char);
        return 1 as libc::c_int;
    }
    pthread_mutex_lock(&mut (*k).lock);
    i = kh_get_kvs((*k).kv, key);
    if i == (*(*k).kv).n_buckets {
        *ret = strm_nil_value();
    } else {
        *ret = *((*(*k).kv).vals).offset(i as isize);
    }
    pthread_mutex_unlock(&mut (*k).lock);
    return 0 as libc::c_int;
}
unsafe extern "C" fn kvs_serial(mut kvs: *mut strm_kvs) -> uint64_t {
    let mut serial: uint64_t = 0;
    pthread_mutex_lock(&mut (*kvs).lock);
    serial = (*kvs).serial;
    pthread_mutex_unlock(&mut (*kvs).lock);
    return serial;
}
unsafe extern "C" fn kvs_put(
    mut strm: *mut strm_stream,
    mut argc: libc::c_int,
    mut args: *mut strm_value,
    mut ret: *mut strm_value,
) -> libc::c_int {
    let mut k: *mut strm_kvs = get_kvs(argc, args);
    let mut key: strm_string = strm_str_intern_str(
        strm_to_str(*args.offset(1 as libc::c_int as isize)),
    );
    let mut i: khiter_t = 0;
    let mut st: libc::c_int = 0;
    if k.is_null() {
        strm_raise(strm, b"no kvs given\0" as *const u8 as *const libc::c_char);
        return 1 as libc::c_int;
    }
    pthread_mutex_lock(&mut (*k).lock);
    i = kh_put_kvs((*k).kv, key, &mut st);
    if st < 0 as libc::c_int {
        pthread_mutex_unlock(&mut (*k).lock);
        return 1 as libc::c_int;
    }
    (*k).serial = ((*k).serial).wrapping_add(1);
    (*k).serial;
    *((*(*k).kv).vals).offset(i as isize) = *args.offset(2 as libc::c_int as isize);
    pthread_mutex_unlock(&mut (*k).lock);
    return 0 as libc::c_int;
}
unsafe extern "C" fn kvs_update(
    mut strm: *mut strm_stream,
    mut argc: libc::c_int,
    mut args: *mut strm_value,
    mut ret: *mut strm_value,
) -> libc::c_int {
    let mut k: *mut strm_kvs = get_kvs(argc, args);
    let mut key: strm_string = strm_str_intern_str(
        strm_to_str(*args.offset(1 as libc::c_int as isize)),
    );
    let mut old: strm_value = 0;
    let mut val: strm_value = 0;
    let mut i: khiter_t = 0;
    let mut st: libc::c_int = 0;
    if k.is_null() {
        strm_raise(strm, b"no kvs given\0" as *const u8 as *const libc::c_char);
        return 1 as libc::c_int;
    }
    pthread_mutex_lock(&mut (*k).lock);
    i = kh_get_kvs((*k).kv, key);
    if i == (*(*k).kv).n_buckets {
        pthread_mutex_unlock(&mut (*k).lock);
        return 1 as libc::c_int;
    }
    old = *((*(*k).kv).vals).offset(i as isize);
    pthread_mutex_unlock(&mut (*k).lock);
    if strm_funcall(
        strm,
        *args.offset(2 as libc::c_int as isize),
        1 as libc::c_int,
        &mut old,
        &mut val,
    ) == 1 as libc::c_int
    {
        return 1 as libc::c_int;
    }
    pthread_mutex_lock(&mut (*k).lock);
    i = kh_put_kvs((*k).kv, key, &mut st);
    if st != 0 as libc::c_int || *((*(*k).kv).vals).offset(i as isize) != old {
        pthread_mutex_unlock(&mut (*k).lock);
        return 1 as libc::c_int;
    }
    (*k).serial = ((*k).serial).wrapping_add(1);
    (*k).serial;
    *((*(*k).kv).vals).offset(i as isize) = val;
    pthread_mutex_unlock(&mut (*k).lock);
    *ret = val;
    return 0 as libc::c_int;
}
unsafe extern "C" fn kvs_close(
    mut strm: *mut strm_stream,
    mut argc: libc::c_int,
    mut args: *mut strm_value,
    mut ret: *mut strm_value,
) -> libc::c_int {
    let mut k: *mut strm_kvs = get_kvs(argc, args);
    if k.is_null() {
        strm_raise(strm, b"no kvs given\0" as *const u8 as *const libc::c_char);
        return 1 as libc::c_int;
    }
    kh_destroy_kvs((*k).kv);
    return 0 as libc::c_int;
}
static mut ns_kvs: *mut strm_state = 0 as *const strm_state as *mut strm_state;
static mut ns_txn: *mut strm_state = 0 as *const strm_state as *mut strm_state;
unsafe extern "C" fn kvs_new(
    mut strm: *mut strm_stream,
    mut argc: libc::c_int,
    mut args: *mut strm_value,
    mut ret: *mut strm_value,
) -> libc::c_int {
    let mut k: *mut strm_kvs = malloc(::std::mem::size_of::<strm_kvs>() as libc::c_ulong)
        as *mut strm_kvs;
    if k.is_null() {
        return 1 as libc::c_int;
    }
    (*k).ns = ns_kvs;
    (*k).type_0 = STRM_PTR_AUX;
    (*k).kv = kh_init_kvs();
    (*k).serial = 1 as libc::c_int as uint64_t;
    pthread_mutex_init(&mut (*k).lock, 0 as *const pthread_mutexattr_t);
    *ret = strm_ptr_value(k as *mut libc::c_void);
    return 0 as libc::c_int;
}
unsafe extern "C" fn txn_new(mut kvs: *mut strm_kvs) -> *mut strm_txn {
    let mut t: *mut strm_txn = malloc(::std::mem::size_of::<strm_txn>() as libc::c_ulong)
        as *mut strm_txn;
    if t.is_null() {
        return 0 as *mut strm_txn;
    }
    (*t).ns = ns_txn;
    (*t).type_0 = STRM_PTR_AUX;
    (*t).tv = kh_init_txn();
    (*t).kvs = kvs;
    (*t).serial = kvs_serial(kvs);
    return t;
}
unsafe extern "C" fn txn_free(mut txn: *mut strm_txn) {
    kh_destroy_txn((*txn).tv);
    (*txn).tv = 0 as *mut kh_txn_t;
}
unsafe extern "C" fn kvs_txn(
    mut strm: *mut strm_stream,
    mut argc: libc::c_int,
    mut args: *mut strm_value,
    mut ret: *mut strm_value,
) -> libc::c_int {
    let mut kvs: *mut strm_kvs = get_kvs(argc, args);
    let mut txn: *mut strm_txn = 0 as *mut strm_txn;
    let mut val: strm_value = 0;
    let mut i: khiter_t = 0;
    let mut j: khiter_t = 0;
    let mut tv: *mut kh_txn_t = 0 as *mut kh_txn_t;
    let mut kv: *mut kh_kvs_t = 0 as *mut kh_kvs_t;
    let mut st: libc::c_int = 0 as libc::c_int;
    let mut result: libc::c_int = 0 as libc::c_int;
    let mut tries: libc::c_int = 0 as libc::c_int;
    if kvs.is_null() {
        strm_raise(strm, b"no kvs given\0" as *const u8 as *const libc::c_char);
        return 1 as libc::c_int;
    }
    txn = txn_new(kvs);
    val = strm_ptr_value(txn as *mut libc::c_void);
    '_retry: loop {
        if strm_funcall(
            strm,
            *args.offset(1 as libc::c_int as isize),
            1 as libc::c_int,
            &mut val,
            ret,
        ) == 1 as libc::c_int
        {
            if (*txn).serial == 0 as libc::c_int as libc::c_ulong {
                tries += 1;
                tries;
                if tries > 10 as libc::c_int {
                    strm_raise(
                        strm,
                        b"too many transaction retries\0" as *const u8
                            as *const libc::c_char,
                    );
                    break;
                } else {
                    (*txn).serial = kvs_serial(kvs);
                }
            } else {
                txn_free(txn);
                return 1 as libc::c_int;
            }
        } else {
            pthread_mutex_lock(&mut (*kvs).lock);
            if (*kvs).serial != (*txn).serial {
                pthread_mutex_unlock(&mut (*kvs).lock);
            } else {
                kv = (*kvs).kv;
                tv = (*txn).tv;
                i = 0 as libc::c_int as khint_t;
                while i != (*tv).n_buckets {
                    if *((*tv).flags).offset((i >> 4 as libc::c_int) as isize)
                        >> ((i & 0xf as libc::c_uint) << 1 as libc::c_int)
                        & 3 as libc::c_int as libc::c_uint == 0
                    {
                        let mut key: strm_value = *((*tv).keys).offset(i as isize);
                        let mut v: strm_value = *((*tv).vals).offset(i as isize);
                        j = kh_put_kvs(kv, key, &mut st);
                        if st < 0 as libc::c_int {
                            pthread_mutex_unlock(&mut (*kvs).lock);
                            break '_retry;
                        } else {
                            *((*kv).vals).offset(j as isize) = v;
                        }
                    }
                    i = i.wrapping_add(1);
                    i;
                }
                if result == 0 as libc::c_int {
                    (*kvs).serial = ((*kvs).serial).wrapping_add(1);
                    (*kvs).serial;
                }
                pthread_mutex_unlock(&mut (*kvs).lock);
                match result {
                    1 => {
                        continue;
                    }
                    2 | 0 | _ => {}
                }
                txn_free(txn);
                return 0 as libc::c_int;
            }
        }
    }
    txn_free(txn);
    return 1 as libc::c_int;
}
unsafe extern "C" fn get_txn(
    mut argc: libc::c_int,
    mut args: *mut strm_value,
) -> *mut strm_txn {
    let mut txn: *mut strm_txn = 0 as *mut strm_txn;
    if argc == 0 as libc::c_int {
        return 0 as *mut strm_txn;
    }
    txn = strm_value_ptr(*args.offset(0 as libc::c_int as isize), STRM_PTR_AUX)
        as *mut strm_txn;
    if ((*txn).tv).is_null() {
        return 0 as *mut strm_txn;
    }
    return txn;
}
unsafe extern "C" fn void_txn(mut strm: *mut strm_stream) -> libc::c_int {
    strm_raise(strm, b"invalid transaction\0" as *const u8 as *const libc::c_char);
    return 1 as libc::c_int;
}
unsafe extern "C" fn txn_retry(mut txn: *mut strm_txn) -> libc::c_int {
    (*txn).serial = 0 as libc::c_int as uint64_t;
    return 1 as libc::c_int;
}
unsafe extern "C" fn txn_get(
    mut strm: *mut strm_stream,
    mut argc: libc::c_int,
    mut args: *mut strm_value,
    mut ret: *mut strm_value,
) -> libc::c_int {
    let mut t: *mut strm_txn = get_txn(argc, args);
    let mut k: *mut strm_kvs = 0 as *mut strm_kvs;
    let mut key: strm_string = strm_str_intern_str(
        strm_to_str(*args.offset(1 as libc::c_int as isize)),
    );
    let mut i: khiter_t = 0;
    if t.is_null() {
        return void_txn(strm);
    }
    k = (*t).kvs;
    if (*t).serial != kvs_serial(k) {
        return txn_retry(t);
    }
    i = kh_get_txn((*t).tv, key);
    if i == (*(*t).tv).n_buckets {
        pthread_mutex_lock(&mut (*k).lock);
        i = kh_get_kvs((*k).kv, key);
        if i == (*(*k).kv).n_buckets {
            *ret = strm_nil_value();
        } else {
            *ret = *((*(*k).kv).vals).offset(i as isize);
        }
        pthread_mutex_unlock(&mut (*k).lock);
    } else {
        *ret = *((*(*t).tv).vals).offset(i as isize);
    }
    return 0 as libc::c_int;
}
unsafe extern "C" fn txn_put(
    mut strm: *mut strm_stream,
    mut argc: libc::c_int,
    mut args: *mut strm_value,
    mut ret: *mut strm_value,
) -> libc::c_int {
    let mut t: *mut strm_txn = get_txn(argc, args);
    let mut key: strm_string = strm_str_intern_str(
        strm_to_str(*args.offset(1 as libc::c_int as isize)),
    );
    let mut i: khiter_t = 0;
    let mut st: libc::c_int = 0;
    if t.is_null() {
        return void_txn(strm);
    }
    i = kh_put_txn((*t).tv, key, &mut st);
    if st < 0 as libc::c_int {
        return 1 as libc::c_int;
    }
    *((*(*t).tv).vals).offset(i as isize) = *args.offset(2 as libc::c_int as isize);
    return 0 as libc::c_int;
}
unsafe extern "C" fn txn_update(
    mut strm: *mut strm_stream,
    mut argc: libc::c_int,
    mut args: *mut strm_value,
    mut ret: *mut strm_value,
) -> libc::c_int {
    let mut t: *mut strm_txn = get_txn(argc, args);
    let mut k: *mut strm_kvs = 0 as *mut strm_kvs;
    let mut key: strm_string = strm_str_intern_str(
        strm_to_str(*args.offset(1 as libc::c_int as isize)),
    );
    let mut val: strm_value = 0;
    let mut i: khiter_t = 0;
    let mut st: libc::c_int = 0;
    if t.is_null() {
        return void_txn(strm);
    }
    k = (*t).kvs;
    if (*t).serial != kvs_serial(k) {
        return txn_retry(t);
    }
    i = kh_put_txn((*t).tv, key, &mut st);
    if st < 0 as libc::c_int {
        return 1 as libc::c_int;
    }
    if st == 0 as libc::c_int {
        val = *((*(*t).tv).vals).offset(i as isize);
    } else {
        pthread_mutex_lock(&mut (*k).lock);
        i = kh_get_kvs((*k).kv, key);
        if i == (*(*k).kv).n_buckets {
            pthread_mutex_unlock(&mut (*k).lock);
            return 1 as libc::c_int;
        } else {
            val = *((*(*k).kv).vals).offset(i as isize);
        }
        pthread_mutex_unlock(&mut (*k).lock);
    }
    if strm_funcall(
        strm,
        *args.offset(2 as libc::c_int as isize),
        1 as libc::c_int,
        &mut val,
        &mut val,
    ) == 1 as libc::c_int
    {
        return 1 as libc::c_int;
    }
    *((*(*t).tv).vals).offset(i as isize) = val;
    *ret = val;
    return 0 as libc::c_int;
}
unsafe extern "C" fn to_str(
    mut strm: *mut strm_stream,
    mut val: strm_value,
    mut type_0: *mut libc::c_char,
) -> strm_value {
    let mut buf: [libc::c_char; 256] = [0; 256];
    let mut n: libc::c_int = 0;
    n = sprintf(
        buf.as_mut_ptr(),
        b"<%s:%p>\0" as *const u8 as *const libc::c_char,
        type_0,
        (val
            & !(((0xfff0 as libc::c_int | 0xf as libc::c_int) as uint64_t)
                << 48 as libc::c_int)) as intptr_t as *mut libc::c_void,
    );
    return strm_str_new(buf.as_mut_ptr(), n);
}
unsafe extern "C" fn kvs_str(
    mut strm: *mut strm_stream,
    mut argc: libc::c_int,
    mut args: *mut strm_value,
    mut ret: *mut strm_value,
) -> libc::c_int {
    if argc != 1 as libc::c_int {
        return 1 as libc::c_int;
    }
    *ret = to_str(
        strm,
        *args.offset(0 as libc::c_int as isize),
        b"kvs\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    );
    return 0 as libc::c_int;
}
unsafe extern "C" fn txn_str(
    mut strm: *mut strm_stream,
    mut argc: libc::c_int,
    mut args: *mut strm_value,
    mut ret: *mut strm_value,
) -> libc::c_int {
    if argc != 1 as libc::c_int {
        return 1 as libc::c_int;
    }
    *ret = to_str(
        strm,
        *args.offset(0 as libc::c_int as isize),
        b"txn\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    );
    return 0 as libc::c_int;
}
pub unsafe extern "C" fn strm_kvs_init(mut state: *mut strm_state) {
    ns_kvs = strm_ns_new(
        0 as *mut strm_state,
        b"kvs\0" as *const u8 as *const libc::c_char,
    );
    strm_var_def(
        ns_kvs,
        b"get\0" as *const u8 as *const libc::c_char,
        strm_cfunc_value(
            Some(
                kvs_get
                    as unsafe extern "C" fn(
                        *mut strm_stream,
                        libc::c_int,
                        *mut strm_value,
                        *mut strm_value,
                    ) -> libc::c_int,
            ),
        ),
    );
    strm_var_def(
        ns_kvs,
        b"put\0" as *const u8 as *const libc::c_char,
        strm_cfunc_value(
            Some(
                kvs_put
                    as unsafe extern "C" fn(
                        *mut strm_stream,
                        libc::c_int,
                        *mut strm_value,
                        *mut strm_value,
                    ) -> libc::c_int,
            ),
        ),
    );
    strm_var_def(
        ns_kvs,
        b"update\0" as *const u8 as *const libc::c_char,
        strm_cfunc_value(
            Some(
                kvs_update
                    as unsafe extern "C" fn(
                        *mut strm_stream,
                        libc::c_int,
                        *mut strm_value,
                        *mut strm_value,
                    ) -> libc::c_int,
            ),
        ),
    );
    strm_var_def(
        ns_kvs,
        b"txn\0" as *const u8 as *const libc::c_char,
        strm_cfunc_value(
            Some(
                kvs_txn
                    as unsafe extern "C" fn(
                        *mut strm_stream,
                        libc::c_int,
                        *mut strm_value,
                        *mut strm_value,
                    ) -> libc::c_int,
            ),
        ),
    );
    strm_var_def(
        ns_kvs,
        b"close\0" as *const u8 as *const libc::c_char,
        strm_cfunc_value(
            Some(
                kvs_close
                    as unsafe extern "C" fn(
                        *mut strm_stream,
                        libc::c_int,
                        *mut strm_value,
                        *mut strm_value,
                    ) -> libc::c_int,
            ),
        ),
    );
    strm_var_def(
        ns_kvs,
        b"string\0" as *const u8 as *const libc::c_char,
        strm_cfunc_value(
            Some(
                kvs_str
                    as unsafe extern "C" fn(
                        *mut strm_stream,
                        libc::c_int,
                        *mut strm_value,
                        *mut strm_value,
                    ) -> libc::c_int,
            ),
        ),
    );
    ns_txn = strm_ns_new(
        0 as *mut strm_state,
        b"kvs_txn\0" as *const u8 as *const libc::c_char,
    );
    strm_var_def(
        ns_txn,
        b"get\0" as *const u8 as *const libc::c_char,
        strm_cfunc_value(
            Some(
                txn_get
                    as unsafe extern "C" fn(
                        *mut strm_stream,
                        libc::c_int,
                        *mut strm_value,
                        *mut strm_value,
                    ) -> libc::c_int,
            ),
        ),
    );
    strm_var_def(
        ns_txn,
        b"put\0" as *const u8 as *const libc::c_char,
        strm_cfunc_value(
            Some(
                txn_put
                    as unsafe extern "C" fn(
                        *mut strm_stream,
                        libc::c_int,
                        *mut strm_value,
                        *mut strm_value,
                    ) -> libc::c_int,
            ),
        ),
    );
    strm_var_def(
        ns_txn,
        b"update\0" as *const u8 as *const libc::c_char,
        strm_cfunc_value(
            Some(
                txn_update
                    as unsafe extern "C" fn(
                        *mut strm_stream,
                        libc::c_int,
                        *mut strm_value,
                        *mut strm_value,
                    ) -> libc::c_int,
            ),
        ),
    );
    strm_var_def(
        ns_kvs,
        b"string\0" as *const u8 as *const libc::c_char,
        strm_cfunc_value(
            Some(
                txn_str
                    as unsafe extern "C" fn(
                        *mut strm_stream,
                        libc::c_int,
                        *mut strm_value,
                        *mut strm_value,
                    ) -> libc::c_int,
            ),
        ),
    );
    strm_var_def(
        state,
        b"kvs\0" as *const u8 as *const libc::c_char,
        strm_cfunc_value(
            Some(
                kvs_new
                    as unsafe extern "C" fn(
                        *mut strm_stream,
                        libc::c_int,
                        *mut strm_value,
                        *mut strm_value,
                    ) -> libc::c_int,
            ),
        ),
    );
}
