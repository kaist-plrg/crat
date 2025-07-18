use ::libc;
extern "C" {
    pub type strm_queue;
    pub type node_error;
    fn strm_cfunc_value(_: strm_cfunc) -> strm_value;
    fn strm_float_value(_: libc::c_double) -> strm_value;
    fn strm_ptr_value(_: *mut libc::c_void) -> strm_value;
    fn strm_str_new(_: *const libc::c_char, _: strm_int) -> strm_string;
    fn strm_stream_new(
        mode: strm_stream_mode,
        start: strm_callback,
        close_0: strm_callback,
        data: *mut libc::c_void,
    ) -> *mut strm_stream;
    fn strm_emit(strm: *mut strm_stream, data: strm_value, cb: strm_callback);
    fn strm_raise(_: *mut strm_stream, _: *const libc::c_char);
    fn strm_parse_args(
        _: *mut strm_stream,
        _: libc::c_int,
        _: *mut strm_value,
        _: *const libc::c_char,
        _: ...
    ) -> libc::c_int;
    fn malloc(_: libc::c_ulong) -> *mut libc::c_void;
    fn free(__ptr: *mut libc::c_void);
    fn close(__fd: libc::c_int) -> libc::c_int;
    fn read(__fd: libc::c_int, __buf: *mut libc::c_void, __nbytes: size_t) -> ssize_t;
    fn memcpy(
        _: *mut libc::c_void,
        _: *const libc::c_void,
        _: libc::c_ulong,
    ) -> *mut libc::c_void;
    fn strm_var_def(
        _: *mut strm_state,
        _: *const libc::c_char,
        _: strm_value,
    ) -> libc::c_int;
    fn fstat(__fd: libc::c_int, __buf: *mut stat) -> libc::c_int;
    fn gettimeofday(__tv: *mut timeval, __tz: *mut libc::c_void) -> libc::c_int;
    fn open(__file: *const libc::c_char, __oflag: libc::c_int, _: ...) -> libc::c_int;
    fn log(_: libc::c_double) -> libc::c_double;
    fn sqrt(_: libc::c_double) -> libc::c_double;
}
pub type size_t = libc::c_ulong;
pub type __int32_t = libc::c_int;
pub type __uint32_t = libc::c_uint;
pub type __uint64_t = libc::c_ulong;
pub type __dev_t = libc::c_ulong;
pub type __uid_t = libc::c_uint;
pub type __gid_t = libc::c_uint;
pub type __ino_t = libc::c_ulong;
pub type __mode_t = libc::c_uint;
pub type __nlink_t = libc::c_ulong;
pub type __off_t = libc::c_long;
pub type __time_t = libc::c_long;
pub type __suseconds_t = libc::c_long;
pub type __blksize_t = libc::c_long;
pub type __blkcnt_t = libc::c_long;
pub type __ssize_t = libc::c_long;
pub type __syscall_slong_t = libc::c_long;
pub type ssize_t = __ssize_t;
pub type int32_t = __int32_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct timeval {
    pub tv_sec: __time_t,
    pub tv_usec: __suseconds_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct timespec {
    pub tv_sec: __time_t,
    pub tv_nsec: __syscall_slong_t,
}
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct stat {
    pub st_dev: __dev_t,
    pub st_ino: __ino_t,
    pub st_nlink: __nlink_t,
    pub st_mode: __mode_t,
    pub st_uid: __uid_t,
    pub st_gid: __gid_t,
    pub __pad0: libc::c_int,
    pub st_rdev: __dev_t,
    pub st_size: __off_t,
    pub st_blksize: __blksize_t,
    pub st_blocks: __blkcnt_t,
    pub st_atim: timespec,
    pub st_mtim: timespec,
    pub st_ctim: timespec,
    pub __glibc_reserved: [__syscall_slong_t; 3],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct rand_data {
    pub seed: [uint32_t; 4],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct rnorm_data {
    pub seed: [uint32_t; 4],
    pub has_spare: libc::c_int,
    pub spare: libc::c_double,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct sample_data {
    pub seed: [uint32_t; 4],
    pub i: strm_int,
    pub len: strm_int,
    pub samples: [strm_value; 0],
}
unsafe extern "C" fn xorshift128init(mut seed: *mut uint32_t) {
    let mut tv: timeval = timeval { tv_sec: 0, tv_usec: 0 };
    let mut y: uint32_t = 0;
    let mut i: libc::c_int = 0;
    let mut fd: libc::c_int = open(
        b"/dev/urandom\0" as *const u8 as *const libc::c_char,
        0o4000 as libc::c_int | 0o400 as libc::c_int | 0 as libc::c_int,
        0 as libc::c_int,
    );
    if fd > 0 as libc::c_int {
        let mut statbuf: stat = stat {
            st_dev: 0,
            st_ino: 0,
            st_nlink: 0,
            st_mode: 0,
            st_uid: 0,
            st_gid: 0,
            __pad0: 0,
            st_rdev: 0,
            st_size: 0,
            st_blksize: 0,
            st_blocks: 0,
            st_atim: timespec { tv_sec: 0, tv_nsec: 0 },
            st_mtim: timespec { tv_sec: 0, tv_nsec: 0 },
            st_ctim: timespec { tv_sec: 0, tv_nsec: 0 },
            __glibc_reserved: [0; 3],
        };
        let mut ret: ssize_t = 0 as libc::c_int as ssize_t;
        let size: size_t = (::std::mem::size_of::<uint64_t>() as libc::c_ulong)
            .wrapping_mul(2 as libc::c_int as libc::c_ulong);
        if fstat(fd, &mut statbuf) == 0 as libc::c_int
            && statbuf.st_mode & 0o170000 as libc::c_int as libc::c_uint
                == 0o20000 as libc::c_int as libc::c_uint
        {
            ret = read(fd, seed as *mut libc::c_void, size);
        }
        close(fd);
        if ret as size_t == size {
            return;
        }
    }
    y = 2463534242 as libc::c_long as uint32_t;
    gettimeofday(&mut tv, 0 as *mut libc::c_void);
    y ^= tv.tv_usec as uint32_t;
    i = 0 as libc::c_int;
    while i < 4 as libc::c_int {
        y = y ^ y << 13 as libc::c_int;
        y = y ^ y >> 17 as libc::c_int;
        y = y ^ y << 5 as libc::c_int;
        *seed.offset(i as isize) = y;
        i += 1;
        i;
    }
}
unsafe extern "C" fn xorshift128(mut seed: *mut uint32_t) -> uint32_t {
    let mut x: uint32_t = *seed.offset(0 as libc::c_int as isize);
    let mut y: uint32_t = *seed.offset(1 as libc::c_int as isize);
    let mut z: uint32_t = *seed.offset(2 as libc::c_int as isize);
    let mut w: uint32_t = *seed.offset(3 as libc::c_int as isize);
    let mut t: uint32_t = 0;
    t = x ^ x << 11 as libc::c_int;
    x = y;
    y = z;
    z = w;
    w = w ^ w >> 19 as libc::c_int ^ (t ^ t >> 8 as libc::c_int);
    *seed.offset(0 as libc::c_int as isize) = x;
    *seed.offset(1 as libc::c_int as isize) = y;
    *seed.offset(2 as libc::c_int as isize) = z;
    *seed.offset(3 as libc::c_int as isize) = w;
    return w;
}
unsafe extern "C" fn rand_float(mut seed: *mut uint32_t) -> libc::c_double {
    return xorshift128(seed) as libc::c_double * (1.0f64 / 4294967295.0f64);
}
unsafe extern "C" fn gen_rand(
    mut strm: *mut strm_stream,
    mut data: strm_value,
) -> libc::c_int {
    let mut d: *mut rand_data = (*strm).data as *mut rand_data;
    let mut f: libc::c_double = rand_float(((*d).seed).as_mut_ptr());
    strm_emit(
        strm,
        strm_float_value(f),
        Some(
            gen_rand as unsafe extern "C" fn(*mut strm_stream, strm_value) -> libc::c_int,
        ),
    );
    return 0 as libc::c_int;
}
unsafe extern "C" fn exec_rand(
    mut strm: *mut strm_stream,
    mut argc: libc::c_int,
    mut args: *mut strm_value,
    mut ret: *mut strm_value,
) -> libc::c_int {
    let mut d: *mut rand_data = 0 as *mut rand_data;
    let mut s: *const libc::c_char = 0 as *const libc::c_char;
    let mut len: strm_int = 0;
    if strm_parse_args(
        strm,
        argc,
        args,
        b"|s\0" as *const u8 as *const libc::c_char,
        &mut s as *mut *const libc::c_char,
        &mut len as *mut strm_int,
    ) == 1 as libc::c_int
    {
        return 1 as libc::c_int;
    }
    d = malloc(::std::mem::size_of::<rand_data>() as libc::c_ulong) as *mut rand_data;
    if d.is_null() {
        return 1 as libc::c_int;
    }
    if argc == 2 as libc::c_int {
        if len as libc::c_ulong
            != ::std::mem::size_of::<[uint32_t; 4]>() as libc::c_ulong
        {
            strm_raise(strm, b"seed size differ\0" as *const u8 as *const libc::c_char);
            free(d as *mut libc::c_void);
            return 1 as libc::c_int;
        }
        memcpy(
            ((*d).seed).as_mut_ptr() as *mut libc::c_void,
            s as *const libc::c_void,
            len as libc::c_ulong,
        );
    } else {
        xorshift128init(((*d).seed).as_mut_ptr());
    }
    *ret = strm_ptr_value(
        strm_stream_new(
            strm_producer,
            Some(
                gen_rand
                    as unsafe extern "C" fn(*mut strm_stream, strm_value) -> libc::c_int,
            ),
            None,
            d as *mut libc::c_void,
        ) as *mut libc::c_void,
    );
    return 0 as libc::c_int;
}
unsafe extern "C" fn rand_seed(
    mut strm: *mut strm_stream,
    mut argc: libc::c_int,
    mut args: *mut strm_value,
    mut ret: *mut strm_value,
) -> libc::c_int {
    let mut seed: [uint32_t; 4] = [0; 4];
    if strm_parse_args(strm, argc, args, b"\0" as *const u8 as *const libc::c_char)
        == 1 as libc::c_int
    {
        return 1 as libc::c_int;
    }
    xorshift128init(seed.as_mut_ptr());
    *ret = strm_str_new(
        seed.as_mut_ptr() as *const libc::c_char,
        ::std::mem::size_of::<[uint32_t; 4]>() as libc::c_ulong as strm_int,
    );
    return 0 as libc::c_int;
}
unsafe extern "C" fn rand_normal(mut d: *mut rnorm_data) -> libc::c_double {
    let mut u: libc::c_double = 0.;
    let mut v: libc::c_double = 0.;
    let mut s: libc::c_double = 0.;
    if (*d).has_spare != 0 {
        (*d).has_spare = 0 as libc::c_int;
        return (*d).spare;
    }
    (*d).has_spare = 1 as libc::c_int;
    loop {
        u = rand_float(((*d).seed).as_mut_ptr()) * 2.0f64 - 1.0f64;
        v = rand_float(((*d).seed).as_mut_ptr()) * 2.0f64 - 1.0f64;
        s = u * u + v * v;
        if !(s >= 1.0f64 || s == 0.0f64) {
            break;
        }
    }
    s = sqrt(-2.0f64 * log(s) / s);
    (*d).spare = v * s;
    return u * s;
}
unsafe extern "C" fn gen_rnorm(
    mut strm: *mut strm_stream,
    mut data: strm_value,
) -> libc::c_int {
    let mut d: *mut rnorm_data = (*strm).data as *mut rnorm_data;
    let mut f: libc::c_double = rand_normal(d);
    strm_emit(
        strm,
        strm_float_value(f),
        Some(
            gen_rnorm
                as unsafe extern "C" fn(*mut strm_stream, strm_value) -> libc::c_int,
        ),
    );
    return 0 as libc::c_int;
}
unsafe extern "C" fn exec_rnorm(
    mut strm: *mut strm_stream,
    mut argc: libc::c_int,
    mut args: *mut strm_value,
    mut ret: *mut strm_value,
) -> libc::c_int {
    let mut d: *mut rnorm_data = 0 as *mut rnorm_data;
    let mut s: *const libc::c_char = 0 as *const libc::c_char;
    let mut len: strm_int = 0;
    if strm_parse_args(
        strm,
        argc,
        args,
        b"|s\0" as *const u8 as *const libc::c_char,
        &mut s as *mut *const libc::c_char,
        &mut len as *mut strm_int,
    ) == 1 as libc::c_int
    {
        return 1 as libc::c_int;
    }
    d = malloc(::std::mem::size_of::<rnorm_data>() as libc::c_ulong) as *mut rnorm_data;
    if d.is_null() {
        return 1 as libc::c_int;
    }
    if argc == 2 as libc::c_int {
        if len as libc::c_ulong
            != ::std::mem::size_of::<[uint32_t; 4]>() as libc::c_ulong
        {
            strm_raise(strm, b"seed size differ\0" as *const u8 as *const libc::c_char);
            free(d as *mut libc::c_void);
            return 1 as libc::c_int;
        }
        memcpy(
            ((*d).seed).as_mut_ptr() as *mut libc::c_void,
            s as *const libc::c_void,
            len as libc::c_ulong,
        );
    } else {
        xorshift128init(((*d).seed).as_mut_ptr());
    }
    (*d).has_spare = 0 as libc::c_int;
    *ret = strm_ptr_value(
        strm_stream_new(
            strm_producer,
            Some(
                gen_rnorm
                    as unsafe extern "C" fn(*mut strm_stream, strm_value) -> libc::c_int,
            ),
            None,
            d as *mut libc::c_void,
        ) as *mut libc::c_void,
    );
    return 0 as libc::c_int;
}
unsafe extern "C" fn iter_sample(
    mut strm: *mut strm_stream,
    mut data: strm_value,
) -> libc::c_int {
    let mut d: *mut sample_data = (*strm).data as *mut sample_data;
    let mut r: uint32_t = 0;
    if (*d).i < (*d).len {
        let fresh0 = (*d).i;
        (*d).i = (*d).i + 1;
        *((*d).samples).as_mut_ptr().offset(fresh0 as isize) = data;
        return 0 as libc::c_int;
    }
    r = (xorshift128(((*d).seed).as_mut_ptr())).wrapping_rem((*d).i as libc::c_uint);
    if r < (*d).len as libc::c_uint {
        *((*d).samples).as_mut_ptr().offset(r as isize) = data;
    }
    (*d).i += 1;
    (*d).i;
    return 0 as libc::c_int;
}
unsafe extern "C" fn finish_sample(
    mut strm: *mut strm_stream,
    mut data: strm_value,
) -> libc::c_int {
    let mut d: *mut sample_data = (*strm).data as *mut sample_data;
    let mut i: strm_int = 0;
    let mut len: strm_int = (*d).len;
    i = 0 as libc::c_int;
    while i < len {
        strm_emit(strm, *((*d).samples).as_mut_ptr().offset(i as isize), None);
        i += 1;
        i;
    }
    free(d as *mut libc::c_void);
    return 0 as libc::c_int;
}
unsafe extern "C" fn exec_sample(
    mut strm: *mut strm_stream,
    mut argc: libc::c_int,
    mut args: *mut strm_value,
    mut ret: *mut strm_value,
) -> libc::c_int {
    let mut d: *mut sample_data = 0 as *mut sample_data;
    let mut len: strm_int = 0;
    if strm_parse_args(
        strm,
        argc,
        args,
        b"i\0" as *const u8 as *const libc::c_char,
        &mut len as *mut strm_int,
    ) == 1 as libc::c_int
    {
        return 1 as libc::c_int;
    }
    d = malloc(
        (::std::mem::size_of::<sample_data>() as libc::c_ulong)
            .wrapping_add(
                (::std::mem::size_of::<strm_value>() as libc::c_ulong)
                    .wrapping_mul(len as libc::c_ulong),
            ),
    ) as *mut sample_data;
    if d.is_null() {
        return 1 as libc::c_int;
    }
    (*d).len = len;
    (*d).i = 0 as libc::c_int;
    xorshift128init(((*d).seed).as_mut_ptr());
    *ret = strm_ptr_value(
        strm_stream_new(
            strm_filter,
            Some(
                iter_sample
                    as unsafe extern "C" fn(*mut strm_stream, strm_value) -> libc::c_int,
            ),
            Some(
                finish_sample
                    as unsafe extern "C" fn(*mut strm_stream, strm_value) -> libc::c_int,
            ),
            d as *mut libc::c_void,
        ) as *mut libc::c_void,
    );
    return 0 as libc::c_int;
}
pub unsafe extern "C" fn strm_rand_init(mut state: *mut strm_state) {
    strm_var_def(
        state,
        b"rand_seed\0" as *const u8 as *const libc::c_char,
        strm_cfunc_value(
            Some(
                rand_seed
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
        b"rand\0" as *const u8 as *const libc::c_char,
        strm_cfunc_value(
            Some(
                exec_rand
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
        b"rand_norm\0" as *const u8 as *const libc::c_char,
        strm_cfunc_value(
            Some(
                exec_rnorm
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
        b"sample\0" as *const u8 as *const libc::c_char,
        strm_cfunc_value(
            Some(
                exec_sample
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
