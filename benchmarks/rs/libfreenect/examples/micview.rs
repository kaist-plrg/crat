use ::libc;
extern "C" {
    pub type _freenect_context;
    pub type _freenect_device;
    fn freenect_init(
        ctx: *mut *mut freenect_context,
        usb_ctx: *mut libc::c_void,
    ) -> libc::c_int;
    fn freenect_shutdown(ctx: *mut freenect_context) -> libc::c_int;
    fn freenect_set_log_level(ctx: *mut freenect_context, level: freenect_loglevel);
    fn freenect_process_events(ctx: *mut freenect_context) -> libc::c_int;
    fn freenect_num_devices(ctx: *mut freenect_context) -> libc::c_int;
    fn freenect_select_subdevices(
        ctx: *mut freenect_context,
        subdevs: freenect_device_flags,
    );
    fn freenect_open_device(
        ctx: *mut freenect_context,
        dev: *mut *mut freenect_device,
        index: libc::c_int,
    ) -> libc::c_int;
    fn freenect_close_device(dev: *mut freenect_device) -> libc::c_int;
    fn freenect_set_user(dev: *mut freenect_device, user: *mut libc::c_void);
    fn freenect_get_user(dev: *mut freenect_device) -> *mut libc::c_void;
    fn freenect_set_audio_in_callback(
        dev: *mut freenect_device,
        callback: freenect_audio_in_cb,
    );
    fn freenect_start_audio(dev: *mut freenect_device) -> libc::c_int;
    fn freenect_stop_audio(dev: *mut freenect_device) -> libc::c_int;
    fn malloc(_: libc::c_ulong) -> *mut libc::c_void;
    fn printf(_: *const libc::c_char, _: ...) -> libc::c_int;
    fn memset(
        _: *mut libc::c_void,
        _: libc::c_int,
        _: libc::c_ulong,
    ) -> *mut libc::c_void;
    fn memcpy(
        _: *mut libc::c_void,
        _: *const libc::c_void,
        _: libc::c_ulong,
    ) -> *mut libc::c_void;
    fn pthread_create(
        __newthread: *mut pthread_t,
        __attr: *const pthread_attr_t,
        __start_routine: Option::<
            unsafe extern "C" fn(*mut libc::c_void) -> *mut libc::c_void,
        >,
        __arg: *mut libc::c_void,
    ) -> libc::c_int;
    fn pthread_join(
        __th: pthread_t,
        __thread_return: *mut *mut libc::c_void,
    ) -> libc::c_int;
    fn pthread_mutex_lock(__mutex: *mut pthread_mutex_t) -> libc::c_int;
    fn pthread_mutex_unlock(__mutex: *mut pthread_mutex_t) -> libc::c_int;
    fn pthread_cond_signal(__cond: *mut pthread_cond_t) -> libc::c_int;
    fn pthread_cond_wait(
        __cond: *mut pthread_cond_t,
        __mutex: *mut pthread_mutex_t,
    ) -> libc::c_int;
    fn glColor4f(red: GLfloat, green: GLfloat, blue: GLfloat, alpha: GLfloat);
    fn glBegin(mode: GLenum);
    fn glVertex3f(x: GLfloat, y: GLfloat, z: GLfloat);
    fn glEnd();
    fn glLoadIdentity();
    fn glViewport(x: GLint, y: GLint, width: GLsizei, height: GLsizei);
    fn glOrtho(
        left: GLdouble,
        right: GLdouble,
        bottom: GLdouble,
        top: GLdouble,
        near_val: GLdouble,
        far_val: GLdouble,
    );
    fn glMatrixMode(mode: GLenum);
    fn glEnable(cap: GLenum);
    fn glBlendFunc(sfactor: GLenum, dfactor: GLenum);
    fn glClear(mask: GLbitfield);
    fn glClearColor(red: GLclampf, green: GLclampf, blue: GLclampf, alpha: GLclampf);
    fn glutDisplayFunc(callback: Option::<unsafe extern "C" fn() -> ()>);
    fn glutReshapeFunc(
        callback: Option::<unsafe extern "C" fn(libc::c_int, libc::c_int) -> ()>,
    );
    fn glutKeyboardFunc(
        callback: Option::<
            unsafe extern "C" fn(libc::c_uchar, libc::c_int, libc::c_int) -> (),
        >,
    );
    fn glutIdleFunc(callback: Option::<unsafe extern "C" fn() -> ()>);
    fn glutSwapBuffers();
    fn glutDestroyWindow(window_0: libc::c_int);
    fn glutCreateWindow(title: *const libc::c_char) -> libc::c_int;
    fn glutInit(pargc: *mut libc::c_int, argv: *mut *mut libc::c_char);
    fn glutInitWindowPosition(x: libc::c_int, y: libc::c_int);
    fn glutInitWindowSize(width: libc::c_int, height: libc::c_int);
    fn glutInitDisplayMode(displayMode: libc::c_uint);
    fn glutMainLoop();
}
pub type __int16_t = libc::c_short;
pub type __int32_t = libc::c_int;
pub type int16_t = __int16_t;
pub type int32_t = __int32_t;
pub type freenect_device_flags = libc::c_uint;
pub const FREENECT_DEVICE_AUDIO: freenect_device_flags = 4;
pub const FREENECT_DEVICE_CAMERA: freenect_device_flags = 2;
pub const FREENECT_DEVICE_MOTOR: freenect_device_flags = 1;
pub type freenect_context = _freenect_context;
pub type freenect_device = _freenect_device;
pub type freenect_usb_context = ();
pub type freenect_loglevel = libc::c_uint;
pub const FREENECT_LOG_FLOOD: freenect_loglevel = 7;
pub const FREENECT_LOG_SPEW: freenect_loglevel = 6;
pub const FREENECT_LOG_DEBUG: freenect_loglevel = 5;
pub const FREENECT_LOG_INFO: freenect_loglevel = 4;
pub const FREENECT_LOG_NOTICE: freenect_loglevel = 3;
pub const FREENECT_LOG_WARNING: freenect_loglevel = 2;
pub const FREENECT_LOG_ERROR: freenect_loglevel = 1;
pub const FREENECT_LOG_FATAL: freenect_loglevel = 0;
pub type freenect_audio_in_cb = Option::<
    unsafe extern "C" fn(
        *mut freenect_device,
        libc::c_int,
        *mut int32_t,
        *mut int32_t,
        *mut int32_t,
        *mut int32_t,
        *mut int16_t,
        *mut libc::c_void,
    ) -> (),
>;
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
pub struct __pthread_cond_s {
    pub c2rust_unnamed: C2RustUnnamed_1,
    pub c2rust_unnamed_0: C2RustUnnamed,
    pub __g_refs: [libc::c_uint; 2],
    pub __g_size: [libc::c_uint; 2],
    pub __g1_orig_size: libc::c_uint,
    pub __wrefs: libc::c_uint,
    pub __g_signals: [libc::c_uint; 2],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed {
    pub __g1_start: libc::c_ulonglong,
    pub __g1_start32: C2RustUnnamed_0,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_0 {
    pub __low: libc::c_uint,
    pub __high: libc::c_uint,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_1 {
    pub __wseq: libc::c_ulonglong,
    pub __wseq32: C2RustUnnamed_2,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_2 {
    pub __low: libc::c_uint,
    pub __high: libc::c_uint,
}
pub type pthread_t = libc::c_ulong;
#[derive(Copy, Clone)]
#[repr(C)]
pub union pthread_attr_t {
    pub __size: [libc::c_char; 56],
    pub __align: libc::c_long,
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
pub union pthread_cond_t {
    pub __data: __pthread_cond_s,
    pub __size: [libc::c_char; 48],
    pub __align: libc::c_longlong,
}
pub type C2RustUnnamed_3 = libc::c_uint;
pub const PTHREAD_MUTEX_DEFAULT: C2RustUnnamed_3 = 0;
pub const PTHREAD_MUTEX_ERRORCHECK: C2RustUnnamed_3 = 2;
pub const PTHREAD_MUTEX_RECURSIVE: C2RustUnnamed_3 = 1;
pub const PTHREAD_MUTEX_NORMAL: C2RustUnnamed_3 = 0;
pub const PTHREAD_MUTEX_ADAPTIVE_NP: C2RustUnnamed_3 = 3;
pub const PTHREAD_MUTEX_ERRORCHECK_NP: C2RustUnnamed_3 = 2;
pub const PTHREAD_MUTEX_RECURSIVE_NP: C2RustUnnamed_3 = 1;
pub const PTHREAD_MUTEX_TIMED_NP: C2RustUnnamed_3 = 0;
pub type GLenum = libc::c_uint;
pub type GLbitfield = libc::c_uint;
pub type GLint = libc::c_int;
pub type GLsizei = libc::c_int;
pub type GLfloat = libc::c_float;
pub type GLclampf = libc::c_float;
pub type GLdouble = libc::c_double;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct capture {
    pub buffers: [*mut int32_t; 4],
    pub max_samples: libc::c_int,
    pub current_idx: libc::c_int,
    pub new_data: libc::c_int,
}
pub static mut freenect_thread: pthread_t = 0;
pub static mut die: libc::c_int = 0 as libc::c_int;
pub static mut window: libc::c_int = 0;
static mut f_ctx: *mut freenect_context = 0 as *const freenect_context
    as *mut freenect_context;
static mut f_dev: *mut freenect_device = 0 as *const freenect_device
    as *mut freenect_device;
pub static mut state: capture = capture {
    buffers: [0 as *const int32_t as *mut int32_t; 4],
    max_samples: 0,
    current_idx: 0,
    new_data: 0,
};
pub static mut paused: libc::c_int = 0 as libc::c_int;
pub static mut audiobuf_mutex: pthread_mutex_t = pthread_mutex_t {
    __data: {
        let mut init = __pthread_mutex_s {
            __lock: 0 as libc::c_int,
            __count: 0 as libc::c_int as libc::c_uint,
            __owner: 0 as libc::c_int,
            __nusers: 0 as libc::c_int as libc::c_uint,
            __kind: PTHREAD_MUTEX_TIMED_NP as libc::c_int,
            __spins: 0 as libc::c_int as libc::c_short,
            __elision: 0 as libc::c_int as libc::c_short,
            __list: {
                let mut init = __pthread_internal_list {
                    __prev: 0 as *const __pthread_internal_list
                        as *mut __pthread_internal_list,
                    __next: 0 as *const __pthread_internal_list
                        as *mut __pthread_internal_list,
                };
                init
            },
        };
        init
    },
};
pub static mut audiobuf_cond: pthread_cond_t = pthread_cond_t {
    __data: {
        let mut init = __pthread_cond_s {
            c2rust_unnamed: C2RustUnnamed_1 {
                __wseq: 0 as libc::c_int as libc::c_ulonglong,
            },
            c2rust_unnamed_0: C2RustUnnamed {
                __g1_start: 0 as libc::c_int as libc::c_ulonglong,
            },
            __g_refs: [
                0 as libc::c_int as libc::c_uint,
                0 as libc::c_int as libc::c_uint,
            ],
            __g_size: [
                0 as libc::c_int as libc::c_uint,
                0 as libc::c_int as libc::c_uint,
            ],
            __g1_orig_size: 0 as libc::c_int as libc::c_uint,
            __wrefs: 0 as libc::c_int as libc::c_uint,
            __g_signals: [
                0 as libc::c_int as libc::c_uint,
                0 as libc::c_int as libc::c_uint,
            ],
        };
        init
    },
};
pub static mut win_h: libc::c_int = 0;
pub static mut win_w: libc::c_int = 0;
pub unsafe extern "C" fn in_callback(
    mut dev: *mut freenect_device,
    mut num_samples: libc::c_int,
    mut mic1: *mut int32_t,
    mut mic2: *mut int32_t,
    mut mic3: *mut int32_t,
    mut mic4: *mut int32_t,
    mut cancelled: *mut int16_t,
    mut unknown: *mut libc::c_void,
) {
    pthread_mutex_lock(&mut audiobuf_mutex);
    let mut c: *mut capture = freenect_get_user(dev) as *mut capture;
    if num_samples < (*c).max_samples - (*c).current_idx {
        memcpy(
            &mut *(*((*c).buffers).as_mut_ptr().offset(0 as libc::c_int as isize))
                .offset((*c).current_idx as isize) as *mut int32_t as *mut libc::c_void,
            mic1 as *const libc::c_void,
            (num_samples as libc::c_ulong)
                .wrapping_mul(::std::mem::size_of::<int32_t>() as libc::c_ulong),
        );
        memcpy(
            &mut *(*((*c).buffers).as_mut_ptr().offset(1 as libc::c_int as isize))
                .offset((*c).current_idx as isize) as *mut int32_t as *mut libc::c_void,
            mic2 as *const libc::c_void,
            (num_samples as libc::c_ulong)
                .wrapping_mul(::std::mem::size_of::<int32_t>() as libc::c_ulong),
        );
        memcpy(
            &mut *(*((*c).buffers).as_mut_ptr().offset(2 as libc::c_int as isize))
                .offset((*c).current_idx as isize) as *mut int32_t as *mut libc::c_void,
            mic3 as *const libc::c_void,
            (num_samples as libc::c_ulong)
                .wrapping_mul(::std::mem::size_of::<int32_t>() as libc::c_ulong),
        );
        memcpy(
            &mut *(*((*c).buffers).as_mut_ptr().offset(3 as libc::c_int as isize))
                .offset((*c).current_idx as isize) as *mut int32_t as *mut libc::c_void,
            mic4 as *const libc::c_void,
            (num_samples as libc::c_ulong)
                .wrapping_mul(::std::mem::size_of::<int32_t>() as libc::c_ulong),
        );
    } else {
        let mut first: libc::c_int = (*c).max_samples - (*c).current_idx;
        let mut left: libc::c_int = num_samples - first;
        memcpy(
            &mut *(*((*c).buffers).as_mut_ptr().offset(0 as libc::c_int as isize))
                .offset((*c).current_idx as isize) as *mut int32_t as *mut libc::c_void,
            mic1 as *const libc::c_void,
            (first as libc::c_ulong)
                .wrapping_mul(::std::mem::size_of::<int32_t>() as libc::c_ulong),
        );
        memcpy(
            &mut *(*((*c).buffers).as_mut_ptr().offset(1 as libc::c_int as isize))
                .offset((*c).current_idx as isize) as *mut int32_t as *mut libc::c_void,
            mic2 as *const libc::c_void,
            (first as libc::c_ulong)
                .wrapping_mul(::std::mem::size_of::<int32_t>() as libc::c_ulong),
        );
        memcpy(
            &mut *(*((*c).buffers).as_mut_ptr().offset(2 as libc::c_int as isize))
                .offset((*c).current_idx as isize) as *mut int32_t as *mut libc::c_void,
            mic3 as *const libc::c_void,
            (first as libc::c_ulong)
                .wrapping_mul(::std::mem::size_of::<int32_t>() as libc::c_ulong),
        );
        memcpy(
            &mut *(*((*c).buffers).as_mut_ptr().offset(3 as libc::c_int as isize))
                .offset((*c).current_idx as isize) as *mut int32_t as *mut libc::c_void,
            mic4 as *const libc::c_void,
            (first as libc::c_ulong)
                .wrapping_mul(::std::mem::size_of::<int32_t>() as libc::c_ulong),
        );
        memcpy(
            (*c).buffers[0 as libc::c_int as usize] as *mut libc::c_void,
            &mut *mic1.offset(first as isize) as *mut int32_t as *const libc::c_void,
            (left as libc::c_ulong)
                .wrapping_mul(::std::mem::size_of::<int32_t>() as libc::c_ulong),
        );
        memcpy(
            (*c).buffers[1 as libc::c_int as usize] as *mut libc::c_void,
            &mut *mic2.offset(first as isize) as *mut int32_t as *const libc::c_void,
            (left as libc::c_ulong)
                .wrapping_mul(::std::mem::size_of::<int32_t>() as libc::c_ulong),
        );
        memcpy(
            (*c).buffers[2 as libc::c_int as usize] as *mut libc::c_void,
            &mut *mic3.offset(first as isize) as *mut int32_t as *const libc::c_void,
            (left as libc::c_ulong)
                .wrapping_mul(::std::mem::size_of::<int32_t>() as libc::c_ulong),
        );
        memcpy(
            (*c).buffers[3 as libc::c_int as usize] as *mut libc::c_void,
            &mut *mic4.offset(first as isize) as *mut int32_t as *const libc::c_void,
            (left as libc::c_ulong)
                .wrapping_mul(::std::mem::size_of::<int32_t>() as libc::c_ulong),
        );
    }
    (*c).current_idx = ((*c).current_idx + num_samples) % (*c).max_samples;
    (*c).new_data = 1 as libc::c_int;
    pthread_cond_signal(&mut audiobuf_cond);
    pthread_mutex_unlock(&mut audiobuf_mutex);
}
pub unsafe extern "C" fn freenect_threadfunc(
    mut arg: *mut libc::c_void,
) -> *mut libc::c_void {
    while die == 0 && freenect_process_events(f_ctx) >= 0 as libc::c_int {}
    freenect_stop_audio(f_dev);
    freenect_close_device(f_dev);
    freenect_shutdown(f_ctx);
    return 0 as *mut libc::c_void;
}
pub unsafe extern "C" fn DrawMicData() {
    if paused != 0 {
        return;
    }
    pthread_mutex_lock(&mut audiobuf_mutex);
    while state.new_data == 0 {
        pthread_cond_wait(&mut audiobuf_cond, &mut audiobuf_mutex);
    }
    state.new_data = 0 as libc::c_int;
    glClear(0x4000 as libc::c_int as GLbitfield);
    glMatrixMode(0x1700 as libc::c_int as GLenum);
    glLoadIdentity();
    let mut xIncr: libc::c_float = win_w as libc::c_float
        / state.max_samples as libc::c_float;
    let mut x: libc::c_float = 0.0f64 as libc::c_float;
    let mut i: libc::c_int = 0;
    let mut base_idx: libc::c_int = state.current_idx;
    pthread_mutex_unlock(&mut audiobuf_mutex);
    let mut mic: libc::c_int = 0;
    mic = 0 as libc::c_int;
    while mic < 4 as libc::c_int {
        glBegin(0x3 as libc::c_int as GLenum);
        glColor4f(1.0f32, 1.0f32, 1.0f32, 0.7f32);
        x = 0 as libc::c_int as libc::c_float;
        i = 0 as libc::c_int;
        while i < state.max_samples {
            glVertex3f(
                x,
                ((win_h as libc::c_float
                    * (2 as libc::c_int * mic + 1 as libc::c_int) as libc::c_float)
                    as libc::c_double / 8.0f64
                    + (*(state.buffers[mic as usize])
                        .offset(((base_idx + i) % state.max_samples) as isize)
                        as libc::c_float
                        * (win_h as libc::c_float / 4 as libc::c_int as libc::c_float))
                        as libc::c_double / 2147483647.0f64) as GLfloat,
                0 as libc::c_int as GLfloat,
            );
            x += xIncr;
            i += 1;
            i;
        }
        glEnd();
        mic += 1;
        mic;
    }
    glutSwapBuffers();
}
pub unsafe extern "C" fn Reshape(mut w: libc::c_int, mut h: libc::c_int) {
    win_w = w;
    win_h = h;
    glViewport(0 as libc::c_int, 0 as libc::c_int, w, h);
    glMatrixMode(0x1701 as libc::c_int as GLenum);
    glLoadIdentity();
    glOrtho(
        0.0f64,
        w as libc::c_float as GLdouble,
        h as libc::c_float as GLdouble,
        0.0f64,
        -1.0f64,
        1.0f64,
    );
    glMatrixMode(0x1700 as libc::c_int as GLenum);
    glLoadIdentity();
}
pub unsafe extern "C" fn Keyboard(
    mut key: libc::c_uchar,
    mut x: libc::c_int,
    mut y: libc::c_int,
) {
    if key as libc::c_int == 'q' as i32 {
        ::std::ptr::write_volatile(&mut die as *mut libc::c_int, 1 as libc::c_int);
        pthread_join(freenect_thread, 0 as *mut *mut libc::c_void);
        glutDestroyWindow(window);
    }
    if key as libc::c_int == 32 as libc::c_int {
        paused = (paused == 0) as libc::c_int;
    }
}
unsafe fn main_0(
    mut argc: libc::c_int,
    mut argv: *mut *mut libc::c_char,
) -> libc::c_int {
    if freenect_init(&mut f_ctx, 0 as *mut libc::c_void) < 0 as libc::c_int {
        printf(b"freenect_init() failed\n\0" as *const u8 as *const libc::c_char);
        return 1 as libc::c_int;
    }
    freenect_set_log_level(f_ctx, FREENECT_LOG_INFO);
    freenect_select_subdevices(f_ctx, FREENECT_DEVICE_AUDIO);
    let mut nr_devices: libc::c_int = freenect_num_devices(f_ctx);
    printf(
        b"Number of devices found: %d\n\0" as *const u8 as *const libc::c_char,
        nr_devices,
    );
    if nr_devices < 1 as libc::c_int {
        freenect_shutdown(f_ctx);
        return 1 as libc::c_int;
    }
    let mut user_device_number: libc::c_int = 0 as libc::c_int;
    if freenect_open_device(f_ctx, &mut f_dev, user_device_number) < 0 as libc::c_int {
        printf(b"Could not open device\n\0" as *const u8 as *const libc::c_char);
        freenect_shutdown(f_ctx);
        return 1 as libc::c_int;
    }
    state.max_samples = 256 as libc::c_int * 60 as libc::c_int;
    state.current_idx = 0 as libc::c_int;
    state
        .buffers[0 as libc::c_int
        as usize] = malloc(
        (state.max_samples as libc::c_ulong)
            .wrapping_mul(::std::mem::size_of::<int32_t>() as libc::c_ulong),
    ) as *mut int32_t;
    state
        .buffers[1 as libc::c_int
        as usize] = malloc(
        (state.max_samples as libc::c_ulong)
            .wrapping_mul(::std::mem::size_of::<int32_t>() as libc::c_ulong),
    ) as *mut int32_t;
    state
        .buffers[2 as libc::c_int
        as usize] = malloc(
        (state.max_samples as libc::c_ulong)
            .wrapping_mul(::std::mem::size_of::<int32_t>() as libc::c_ulong),
    ) as *mut int32_t;
    state
        .buffers[3 as libc::c_int
        as usize] = malloc(
        (state.max_samples as libc::c_ulong)
            .wrapping_mul(::std::mem::size_of::<int32_t>() as libc::c_ulong),
    ) as *mut int32_t;
    memset(
        state.buffers[0 as libc::c_int as usize] as *mut libc::c_void,
        0 as libc::c_int,
        (state.max_samples as libc::c_ulong)
            .wrapping_mul(::std::mem::size_of::<int32_t>() as libc::c_ulong),
    );
    memset(
        state.buffers[1 as libc::c_int as usize] as *mut libc::c_void,
        0 as libc::c_int,
        (state.max_samples as libc::c_ulong)
            .wrapping_mul(::std::mem::size_of::<int32_t>() as libc::c_ulong),
    );
    memset(
        state.buffers[2 as libc::c_int as usize] as *mut libc::c_void,
        0 as libc::c_int,
        (state.max_samples as libc::c_ulong)
            .wrapping_mul(::std::mem::size_of::<int32_t>() as libc::c_ulong),
    );
    memset(
        state.buffers[3 as libc::c_int as usize] as *mut libc::c_void,
        0 as libc::c_int,
        (state.max_samples as libc::c_ulong)
            .wrapping_mul(::std::mem::size_of::<int32_t>() as libc::c_ulong),
    );
    freenect_set_user(f_dev, &mut state as *mut capture as *mut libc::c_void);
    freenect_set_audio_in_callback(
        f_dev,
        Some(
            in_callback
                as unsafe extern "C" fn(
                    *mut freenect_device,
                    libc::c_int,
                    *mut int32_t,
                    *mut int32_t,
                    *mut int32_t,
                    *mut int32_t,
                    *mut int16_t,
                    *mut libc::c_void,
                ) -> (),
        ),
    );
    freenect_start_audio(f_dev);
    let mut res: libc::c_int = pthread_create(
        &mut freenect_thread,
        0 as *const pthread_attr_t,
        Some(
            freenect_threadfunc
                as unsafe extern "C" fn(*mut libc::c_void) -> *mut libc::c_void,
        ),
        0 as *mut libc::c_void,
    );
    if res != 0 {
        printf(b"pthread_create failed\n\0" as *const u8 as *const libc::c_char);
        freenect_shutdown(f_ctx);
        return 1 as libc::c_int;
    }
    printf(
        b"This is the libfreenect microphone waveform viewer.  Press 'q' to quit or spacebar to pause/unpause the view.\n\0"
            as *const u8 as *const libc::c_char,
    );
    glutInit(&mut argc, argv);
    glutInitDisplayMode(
        (0x2 as libc::c_int | 0 as libc::c_int | 0x8 as libc::c_int) as libc::c_uint,
    );
    glutInitWindowSize(800 as libc::c_int, 600 as libc::c_int);
    glutInitWindowPosition(0 as libc::c_int, 0 as libc::c_int);
    window = glutCreateWindow(b"Microphones\0" as *const u8 as *const libc::c_char);
    glClearColor(
        0.0f64 as GLclampf,
        0.0f64 as GLclampf,
        0.0f64 as GLclampf,
        0.0f64 as GLclampf,
    );
    glEnable(0xbe2 as libc::c_int as GLenum);
    glBlendFunc(0x302 as libc::c_int as GLenum, 0x303 as libc::c_int as GLenum);
    Reshape(800 as libc::c_int, 600 as libc::c_int);
    glutReshapeFunc(
        Some(Reshape as unsafe extern "C" fn(libc::c_int, libc::c_int) -> ()),
    );
    glutDisplayFunc(
        ::std::mem::transmute::<
            Option::<unsafe extern "C" fn() -> ()>,
            Option::<unsafe extern "C" fn() -> ()>,
        >(
            Some(
                ::std::mem::transmute::<
                    unsafe extern "C" fn() -> (),
                    unsafe extern "C" fn() -> (),
                >(DrawMicData),
            ),
        ),
    );
    glutIdleFunc(
        ::std::mem::transmute::<
            Option::<unsafe extern "C" fn() -> ()>,
            Option::<unsafe extern "C" fn() -> ()>,
        >(
            Some(
                ::std::mem::transmute::<
                    unsafe extern "C" fn() -> (),
                    unsafe extern "C" fn() -> (),
                >(DrawMicData),
            ),
        ),
    );
    glutKeyboardFunc(
        Some(
            Keyboard
                as unsafe extern "C" fn(libc::c_uchar, libc::c_int, libc::c_int) -> (),
        ),
    );
    glutMainLoop();
    return 0 as libc::c_int;
}
pub fn main() {
    let mut args: Vec::<*mut libc::c_char> = Vec::new();
    for arg in ::std::env::args_os() {
        args.push(
            (::std::ffi::CString::new(
                ::std::os::unix::ffi::OsStrExt::as_bytes(arg.as_os_str()),
            ))
                .unwrap()
                .into_raw(),
        );
    }
    args.push(::std::ptr::null_mut());
    unsafe {
        ::std::process::exit(
            main_0(
                (args.len() - 1) as libc::c_int,
                args.as_mut_ptr() as *mut *mut libc::c_char,
            ) as i32,
        )
    }
}
