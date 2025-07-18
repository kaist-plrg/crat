use ::libc;
extern "C" {
    fn free(__ptr: *mut libc::c_void);
    fn strcmp(_: *const libc::c_char, _: *const libc::c_char) -> libc::c_int;
    fn reset_output_buffers(data: *mut audio_data);
    fn read(__fd: libc::c_int, __buf: *mut libc::c_void, __nbytes: size_t) -> ssize_t;
    fn write_to_cava_input_buffers(
        size: int16_t,
        buf: *mut int16_t,
        data: *mut libc::c_void,
    ) -> libc::c_int;
    fn nanosleep(
        __requested_time: *const timespec,
        __remaining: *mut timespec,
    ) -> libc::c_int;
    fn close(__fd: libc::c_int) -> libc::c_int;
    fn fcntl(__fd: libc::c_int, __cmd: libc::c_int, _: ...) -> libc::c_int;
    fn open(__file: *const libc::c_char, __oflag: libc::c_int, _: ...) -> libc::c_int;
    fn calloc(_: libc::c_ulong, _: libc::c_ulong) -> *mut libc::c_void;
}
pub type uint16_t = __uint16_t;
pub type __uint16_t = libc::c_ushort;
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
#[derive(Copy, Clone)]
#[repr(C)]
pub union pthread_mutex_t {
    pub __data: __pthread_mutex_s,
    pub __size: [libc::c_char; 40],
    pub __align: libc::c_long,
}
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
pub type __pthread_list_t = __pthread_internal_list;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct __pthread_internal_list {
    pub __prev: *mut __pthread_internal_list,
    pub __next: *mut __pthread_internal_list,
}
pub type uint8_t = __uint8_t;
pub type __uint8_t = libc::c_uchar;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct timespec {
    pub tv_sec: __time_t,
    pub tv_nsec: __syscall_slong_t,
}
pub type __syscall_slong_t = libc::c_long;
pub type __time_t = libc::c_long;
pub type int16_t = __int16_t;
pub type __int16_t = libc::c_short;
pub type ssize_t = __ssize_t;
pub type __ssize_t = libc::c_long;
pub type size_t = libc::c_ulong;
pub unsafe extern "C" fn open_fifo(mut path: *const libc::c_char) -> libc::c_int {
    let mut fd: libc::c_int = open(path, 0 as libc::c_int);
    let mut flags: libc::c_int = fcntl(fd, 3 as libc::c_int, 0 as libc::c_int);
    fcntl(fd, 4 as libc::c_int, flags | 0o4000 as libc::c_int);
    return fd;
}
pub unsafe extern "C" fn input_fifo(mut data: *mut libc::c_void) -> *mut libc::c_void {
    let mut audio: *mut audio_data = data as *mut audio_data;
    let mut SAMPLES_PER_BUFFER: libc::c_int = (*audio).input_buffer_size;
    let mut bytes_per_sample: libc::c_int = (*audio).format / 8 as libc::c_int;
    let vla = (SAMPLES_PER_BUFFER * bytes_per_sample) as usize;
    let mut buf: Vec::<uint8_t> = ::std::vec::from_elem(0, vla);
    let mut samples: *mut uint16_t = (if bytes_per_sample == 2 as libc::c_int {
        buf.as_mut_ptr() as *mut uint8_t as *mut uint16_t as *mut libc::c_void
    } else {
        calloc(
            SAMPLES_PER_BUFFER as libc::c_ulong,
            ::std::mem::size_of::<uint16_t>() as libc::c_ulong,
        )
    }) as *mut uint16_t;
    let mut fd: libc::c_int = open_fifo((*audio).source);
    let mut test_mode: libc::c_int = 0 as libc::c_int;
    if strcmp((*audio).source, b"/dev/zero\0" as *const u8 as *const libc::c_char)
        == 0 as libc::c_int
    {
        test_mode = 1 as libc::c_int;
    }
    while (*audio).terminate == 0 {
        let mut time_since_last_input: libc::c_int = 0 as libc::c_int;
        let mut offset: libc::c_uint = 0 as libc::c_int as libc::c_uint;
        loop {
            let mut num_read: libc::c_int = read(
                fd,
                buf.as_mut_ptr().offset(offset as isize) as *mut libc::c_void,
                ((vla * ::std::mem::size_of::<uint8_t>()) as libc::c_ulong)
                    .wrapping_sub(offset as libc::c_ulong),
            ) as libc::c_int;
            if num_read < 1 as libc::c_int {
                nanosleep(
                    &mut {
                        let mut init = timespec {
                            tv_sec: 0 as libc::c_int as __time_t,
                            tv_nsec: 10000000 as libc::c_int as __syscall_slong_t,
                        };
                        init
                    },
                    0 as *mut timespec,
                );
                time_since_last_input += 1;
                time_since_last_input;
                if time_since_last_input > 10 as libc::c_int {
                    reset_output_buffers(audio);
                    close(fd);
                    fd = open_fifo((*audio).source);
                    time_since_last_input = 0 as libc::c_int;
                    offset = 0 as libc::c_int as libc::c_uint;
                }
            } else {
                offset = offset.wrapping_add(num_read as libc::c_uint);
                time_since_last_input = 0 as libc::c_int;
            }
            if !((offset as libc::c_ulong)
                < (vla * ::std::mem::size_of::<uint8_t>()) as libc::c_ulong)
            {
                break;
            }
        }
        match bytes_per_sample {
            3 => {
                let mut i: libc::c_int = 0 as libc::c_int;
                while i < SAMPLES_PER_BUFFER {
                    *samples
                        .offset(
                            i as isize,
                        ) = ((*buf
                        .as_mut_ptr()
                        .offset((3 as libc::c_int * i + 2 as libc::c_int) as isize)
                        as libc::c_int) << 8 as libc::c_int
                        | *buf
                            .as_mut_ptr()
                            .offset((3 as libc::c_int * i + 1 as libc::c_int) as isize)
                            as libc::c_int) as uint16_t;
                    i += 1;
                    i;
                }
            }
            4 => {
                let mut i_0: libc::c_int = 0 as libc::c_int;
                while i_0 < SAMPLES_PER_BUFFER {
                    *samples
                        .offset(
                            i_0 as isize,
                        ) = ((*buf
                        .as_mut_ptr()
                        .offset((4 as libc::c_int * i_0 + 3 as libc::c_int) as isize)
                        as libc::c_int) << 8 as libc::c_int
                        | *buf
                            .as_mut_ptr()
                            .offset((4 as libc::c_int * i_0 + 2 as libc::c_int) as isize)
                            as libc::c_int) as uint16_t;
                    i_0 += 1;
                    i_0;
                }
            }
            2 | _ => {}
        }
        write_to_cava_input_buffers(
            SAMPLES_PER_BUFFER as int16_t,
            samples as *mut int16_t,
            audio as *mut libc::c_void,
        );
        if test_mode != 0 {
            nanosleep(
                &mut {
                    let mut init = timespec {
                        tv_sec: 0 as libc::c_int as __time_t,
                        tv_nsec: 1000000 as libc::c_int as __syscall_slong_t,
                    };
                    init
                },
                0 as *mut timespec,
            );
        }
    }
    close(fd);
    if bytes_per_sample != 2 as libc::c_int {
        free(samples as *mut libc::c_void);
    }
    return 0 as *mut libc::c_void;
}
