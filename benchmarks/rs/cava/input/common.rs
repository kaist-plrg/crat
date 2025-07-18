use ::libc;
extern "C" {
    fn pthread_mutex_lock(__mutex: *mut pthread_mutex_t) -> libc::c_int;
    fn pthread_mutex_unlock(__mutex: *mut pthread_mutex_t) -> libc::c_int;
}
pub type __int16_t = libc::c_short;
pub type __uint16_t = libc::c_ushort;
pub type int16_t = __int16_t;
pub type uint16_t = __uint16_t;
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
pub union pthread_mutex_t {
    pub __data: __pthread_mutex_s,
    pub __size: [libc::c_char; 40],
    pub __align: libc::c_long,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct audio_data {
    pub cava_in: *mut libc::c_double,
    pub input_buffer_size: libc::c_int,
    pub cava_buffer_size: libc::c_int,
    pub format: libc::c_int,
    pub rate: libc::c_uint,
    pub channels: libc::c_uint,
    pub source: *mut libc::c_char,
    pub im: libc::c_int,
    pub terminate: libc::c_int,
    pub error_message: [libc::c_char; 1024],
    pub samples_counter: libc::c_int,
    pub lock: pthread_mutex_t,
}
pub unsafe extern "C" fn write_to_cava_input_buffers(
    mut size: int16_t,
    mut buf: *mut int16_t,
    mut data: *mut libc::c_void,
) -> libc::c_int {
    if size as libc::c_int == 0 as libc::c_int {
        return 0 as libc::c_int;
    }
    let mut audio: *mut audio_data = data as *mut audio_data;
    pthread_mutex_lock(&mut (*audio).lock);
    if (*audio).samples_counter + size as libc::c_int
        > (*audio).input_buffer_size * 4 as libc::c_int
    {
        (*audio).samples_counter = 0 as libc::c_int;
    }
    let mut i: uint16_t = 0 as libc::c_int as uint16_t;
    while (i as libc::c_int) < size as libc::c_int {
        *((*audio).cava_in)
            .offset(
                (i as libc::c_int + (*audio).samples_counter) as isize,
            ) = *buf.offset(i as isize) as libc::c_double;
        i = i.wrapping_add(1);
        i;
    }
    (*audio).samples_counter += size as libc::c_int;
    pthread_mutex_unlock(&mut (*audio).lock);
    return 0 as libc::c_int;
}
pub unsafe extern "C" fn reset_output_buffers(mut data: *mut audio_data) {
    let mut audio: *mut audio_data = data;
    pthread_mutex_lock(&mut (*audio).lock);
    let mut n: uint16_t = 0 as libc::c_int as uint16_t;
    while (n as libc::c_int)
        < (*audio).input_buffer_size * 4 as libc::c_int * 2 as libc::c_int
    {
        *((*audio).cava_in).offset(n as isize) = 0 as libc::c_int as libc::c_double;
        n = n.wrapping_add(1);
        n;
    }
    pthread_mutex_unlock(&mut (*audio).lock);
}
