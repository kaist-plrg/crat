use ::libc;
extern "C" {
    pub type _IO_wide_data;
    pub type _IO_codecvt;
    pub type _IO_marker;
    static mut stderr: *mut FILE;
    fn fprintf(_: *mut FILE, _: *const libc::c_char, _: ...) -> libc::c_int;
    fn exit(_: libc::c_int) -> !;
    fn __errno_location() -> *mut libc::c_int;
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
    fn strerror(_: libc::c_int) -> *mut libc::c_char;
    fn mmap(
        __addr: *mut libc::c_void,
        __len: size_t,
        __prot: libc::c_int,
        __flags: libc::c_int,
        __fd: libc::c_int,
        __offset: __off_t,
    ) -> *mut libc::c_void;
    fn munmap(__addr: *mut libc::c_void, __len: size_t) -> libc::c_int;
    fn shm_open(
        __name: *const libc::c_char,
        __oflag: libc::c_int,
        __mode: mode_t,
    ) -> libc::c_int;
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct vis_t {
    pub rwlock: pthread_rwlock_t,
    pub buf_size: u32_t,
    pub buf_index: u32_t,
    pub running: bool,
    pub rate: u32_t,
    pub updated: time_t,
    pub buffer: [s16_t; 16384],
}
pub type s16_t = libc::c_short;
pub type time_t = __time_t;
pub type __time_t = libc::c_long;
pub type u32_t = libc::c_uint;
#[derive(Copy, Clone)]
#[repr(C)]
pub union pthread_rwlock_t {
    pub __data: __pthread_rwlock_arch_t,
    pub __size: [libc::c_char; 56],
    pub __align: libc::c_long,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct __pthread_rwlock_arch_t {
    pub __readers: libc::c_uint,
    pub __writers: libc::c_uint,
    pub __wrphase_futex: libc::c_uint,
    pub __writers_futex: libc::c_uint,
    pub __pad3: libc::c_uint,
    pub __pad4: libc::c_uint,
    pub __cur_writer: libc::c_int,
    pub __shared: libc::c_int,
    pub __rwelision: libc::c_schar,
    pub __pad1: [libc::c_uchar; 7],
    pub __pad2: libc::c_ulong,
    pub __flags: libc::c_uint,
}
pub type FILE = _IO_FILE;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _IO_FILE {
    pub _flags: libc::c_int,
    pub _IO_read_ptr: *mut libc::c_char,
    pub _IO_read_end: *mut libc::c_char,
    pub _IO_read_base: *mut libc::c_char,
    pub _IO_write_base: *mut libc::c_char,
    pub _IO_write_ptr: *mut libc::c_char,
    pub _IO_write_end: *mut libc::c_char,
    pub _IO_buf_base: *mut libc::c_char,
    pub _IO_buf_end: *mut libc::c_char,
    pub _IO_save_base: *mut libc::c_char,
    pub _IO_backup_base: *mut libc::c_char,
    pub _IO_save_end: *mut libc::c_char,
    pub _markers: *mut _IO_marker,
    pub _chain: *mut _IO_FILE,
    pub _fileno: libc::c_int,
    pub _flags2: libc::c_int,
    pub _old_offset: __off_t,
    pub _cur_column: libc::c_ushort,
    pub _vtable_offset: libc::c_schar,
    pub _shortbuf: [libc::c_char; 1],
    pub _lock: *mut libc::c_void,
    pub _offset: __off64_t,
    pub _codecvt: *mut _IO_codecvt,
    pub _wide_data: *mut _IO_wide_data,
    pub _freeres_list: *mut _IO_FILE,
    pub _freeres_buf: *mut libc::c_void,
    pub __pad5: size_t,
    pub _mode: libc::c_int,
    pub _unused2: [libc::c_char; 20],
}
pub type size_t = libc::c_ulong;
pub type __off64_t = libc::c_long;
pub type _IO_lock_t = ();
pub type __off_t = libc::c_long;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct timespec {
    pub tv_sec: __time_t,
    pub tv_nsec: __syscall_slong_t,
}
pub type __syscall_slong_t = libc::c_long;
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
pub type int16_t = __int16_t;
pub type __int16_t = libc::c_short;
pub type intptr_t = libc::c_long;
pub type mode_t = __mode_t;
pub type __mode_t = libc::c_uint;
pub static mut rc: libc::c_int = 0;
pub unsafe extern "C" fn input_shmem(mut data: *mut libc::c_void) -> *mut libc::c_void {
    let mut audio: *mut audio_data = data as *mut audio_data;
    let mut mmap_area: *mut vis_t = 0 as *mut vis_t;
    let mut fd: libc::c_int = 0;
    let mut mmap_count: libc::c_int = ::std::mem::size_of::<vis_t>() as libc::c_ulong
        as libc::c_int;
    let mut buf_frames: libc::c_int = 0;
    let mut fftw_frames: libc::c_int = (*audio).input_buffer_size / 2 as libc::c_int;
    let mut req: timespec = {
        let mut init = timespec {
            tv_sec: 0 as libc::c_int as __time_t,
            tv_nsec: 0 as libc::c_int as __syscall_slong_t,
        };
        init
    };
    let vla = (fftw_frames * 2 as libc::c_int) as usize;
    let mut buf: Vec::<int16_t> = ::std::vec::from_elem(0, vla);
    let vla_0 = (fftw_frames * 2 as libc::c_int) as usize;
    let mut silence_buffer: Vec::<s16_t> = ::std::vec::from_elem(0, vla_0);
    let mut i: libc::c_int = 0 as libc::c_int;
    while i < fftw_frames * 2 as libc::c_int {
        *silence_buffer.as_mut_ptr().offset(i as isize) = 0 as libc::c_int as s16_t;
        i += 1;
        i;
    }
    fd = shm_open((*audio).source, 0o2 as libc::c_int, 0o666 as libc::c_int as mode_t);
    if fd < 0 as libc::c_int {
        fprintf(
            stderr,
            b"Could not open source '%s': %s\n\0" as *const u8 as *const libc::c_char,
            (*audio).source,
            strerror(*__errno_location()),
        );
        exit(1 as libc::c_int);
    } else {
        mmap_area = mmap(
            0 as *mut libc::c_void,
            ::std::mem::size_of::<vis_t>() as libc::c_ulong,
            0x1 as libc::c_int | 0x2 as libc::c_int,
            0x1 as libc::c_int,
            fd,
            0 as libc::c_int as __off_t,
        ) as *mut vis_t;
        if mmap_area as intptr_t == -(1 as libc::c_int) as libc::c_long {
            fprintf(
                stderr,
                b"mmap failed - check if squeezelite is running with visualization enabled\n\0"
                    as *const u8 as *const libc::c_char,
            );
            exit(1 as libc::c_int);
        }
    }
    while (*audio).terminate == 0 {
        (*audio).rate = (*mmap_area).rate;
        buf_frames = ((*mmap_area).buf_size)
            .wrapping_div(2 as libc::c_int as libc::c_uint) as libc::c_int;
        req
            .tv_nsec = (1000000 as libc::c_int as libc::c_uint)
            .wrapping_div((*mmap_area).rate)
            .wrapping_mul(buf_frames as libc::c_uint) as __syscall_slong_t;
        if (*mmap_area).running {
            let mut i_0: libc::c_int = 0 as libc::c_int;
            while i_0 < buf_frames / (fftw_frames * 2 as libc::c_int) {
                let mut n: libc::c_int = 0 as libc::c_int;
                while n < fftw_frames * 2 as libc::c_int {
                    *buf
                        .as_mut_ptr()
                        .offset(n as isize) = (*mmap_area).buffer[(n + i_0) as usize];
                    n += 1;
                    n;
                }
                write_to_cava_input_buffers(
                    (fftw_frames * 2 as libc::c_int) as int16_t,
                    buf.as_mut_ptr(),
                    audio as *mut libc::c_void,
                );
                i_0 += fftw_frames * 2 as libc::c_int;
            }
            nanosleep(&mut req, 0 as *mut timespec);
        } else {
            write_to_cava_input_buffers(
                (fftw_frames * 2 as libc::c_int) as int16_t,
                silence_buffer.as_mut_ptr(),
                audio as *mut libc::c_void,
            );
            nanosleep(&mut req, 0 as *mut timespec);
        }
    }
    if fd > 0 as libc::c_int {
        if close(fd) != 0 as libc::c_int {
            fprintf(
                stderr,
                b"Could not close file descriptor %d: %s\0" as *const u8
                    as *const libc::c_char,
                fd,
                strerror(*__errno_location()),
            );
        }
    } else {
        fprintf(
            stderr,
            b"Wrong file descriptor %d\0" as *const u8 as *const libc::c_char,
            fd,
        );
    }
    if munmap(mmap_area as *mut libc::c_void, mmap_count as size_t) != 0 as libc::c_int {
        fprintf(
            stderr,
            b"Could not munmap() area %p+%d. %s\0" as *const u8 as *const libc::c_char,
            mmap_area,
            mmap_count,
            strerror(*__errno_location()),
        );
    }
    return 0 as *mut libc::c_void;
}
