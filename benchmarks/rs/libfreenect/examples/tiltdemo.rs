use ::libc;
extern "C" {
    fn freenect_get_mks_accel(
        state: *mut freenect_raw_tilt_state,
        x: *mut libc::c_double,
        y: *mut libc::c_double,
        z: *mut libc::c_double,
    );
    fn freenect_sync_set_tilt_degs(
        angle: libc::c_int,
        index: libc::c_int,
    ) -> libc::c_int;
    fn freenect_sync_get_tilt_state(
        state: *mut *mut freenect_raw_tilt_state,
        index: libc::c_int,
    ) -> libc::c_int;
    fn freenect_sync_set_led(
        led: freenect_led_options,
        index: libc::c_int,
    ) -> libc::c_int;
    fn printf(_: *const libc::c_char, _: ...) -> libc::c_int;
    fn rand() -> libc::c_int;
    fn srand(__seed: libc::c_uint);
    fn exit(_: libc::c_int) -> !;
    fn time(__timer: *mut time_t) -> time_t;
    fn sleep(__seconds: libc::c_uint) -> libc::c_uint;
}
pub type __int8_t = libc::c_schar;
pub type __int16_t = libc::c_short;
pub type __time_t = libc::c_long;
pub type int8_t = __int8_t;
pub type int16_t = __int16_t;
pub type time_t = __time_t;
pub type freenect_led_options = libc::c_uint;
pub const LED_BLINK_RED_YELLOW: freenect_led_options = 6;
pub const LED_BLINK_GREEN: freenect_led_options = 4;
pub const LED_YELLOW: freenect_led_options = 3;
pub const LED_RED: freenect_led_options = 2;
pub const LED_GREEN: freenect_led_options = 1;
pub const LED_OFF: freenect_led_options = 0;
pub type freenect_tilt_status_code = libc::c_uint;
pub const TILT_STATUS_MOVING: freenect_tilt_status_code = 4;
pub const TILT_STATUS_LIMIT: freenect_tilt_status_code = 1;
pub const TILT_STATUS_STOPPED: freenect_tilt_status_code = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct freenect_raw_tilt_state {
    pub accelerometer_x: int16_t,
    pub accelerometer_y: int16_t,
    pub accelerometer_z: int16_t,
    pub tilt_angle: int8_t,
    pub tilt_status: freenect_tilt_status_code,
}
pub unsafe extern "C" fn no_kinect_quit() {
    printf(b"Error: Kinect not connected?\n\0" as *const u8 as *const libc::c_char);
    exit(1 as libc::c_int);
}
unsafe fn main_0(
    mut argc: libc::c_int,
    mut argv: *mut *mut libc::c_char,
) -> libc::c_int {
    srand(time(0 as *mut time_t) as libc::c_uint);
    loop {
        let mut led: freenect_led_options = (rand() % 6 as libc::c_int)
            as freenect_led_options;
        let mut tilt: libc::c_int = rand() % 30 as libc::c_int - 15 as libc::c_int;
        let mut state: *mut freenect_raw_tilt_state = 0 as *mut freenect_raw_tilt_state;
        let mut dx: libc::c_double = 0.;
        let mut dy: libc::c_double = 0.;
        let mut dz: libc::c_double = 0.;
        if freenect_sync_set_led(led, 0 as libc::c_int) != 0 {
            no_kinect_quit();
        }
        if freenect_sync_set_tilt_degs(tilt, 0 as libc::c_int) != 0 {
            no_kinect_quit();
        }
        if freenect_sync_get_tilt_state(&mut state, 0 as libc::c_int) != 0 {
            no_kinect_quit();
        }
        freenect_get_mks_accel(state, &mut dx, &mut dy, &mut dz);
        printf(
            b"led[%d] tilt[%d] accel[%lf,%lf,%lf]\n\0" as *const u8
                as *const libc::c_char,
            led as libc::c_uint,
            tilt,
            dx,
            dy,
            dz,
        );
        sleep(3 as libc::c_int as libc::c_uint);
    };
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
