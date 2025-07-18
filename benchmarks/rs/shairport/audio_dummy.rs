use ::libc;
extern "C" {
    fn printf(_: *const libc::c_char, _: ...) -> libc::c_int;
    fn usleep(__useconds: __useconds_t) -> libc::c_int;
    fn gettimeofday(__tv: *mut timeval, __tz: *mut libc::c_void) -> libc::c_int;
}
pub type __time_t = libc::c_long;
pub type __useconds_t = libc::c_uint;
pub type __suseconds_t = libc::c_long;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct timeval {
    pub tv_sec: __time_t,
    pub tv_usec: __suseconds_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct audio_output {
    pub help: Option::<unsafe extern "C" fn() -> ()>,
    pub name: *mut libc::c_char,
    pub init: Option::<
        unsafe extern "C" fn(libc::c_int, *mut *mut libc::c_char) -> libc::c_int,
    >,
    pub deinit: Option::<unsafe extern "C" fn() -> ()>,
    pub start: Option::<unsafe extern "C" fn(libc::c_int) -> ()>,
    pub play: Option::<unsafe extern "C" fn(*mut libc::c_short, libc::c_int) -> ()>,
    pub stop: Option::<unsafe extern "C" fn() -> ()>,
    pub volume: Option::<unsafe extern "C" fn(libc::c_double) -> ()>,
}
pub static mut Fs: libc::c_int = 0;
pub static mut starttime: libc::c_longlong = 0;
pub static mut samples_played: libc::c_longlong = 0;
unsafe extern "C" fn init(
    mut argc: libc::c_int,
    mut argv: *mut *mut libc::c_char,
) -> libc::c_int {
    return 0 as libc::c_int;
}
unsafe extern "C" fn deinit() {}
unsafe extern "C" fn start(mut sample_rate: libc::c_int) {
    Fs = sample_rate;
    starttime = 0 as libc::c_int as libc::c_longlong;
    samples_played = 0 as libc::c_int as libc::c_longlong;
    printf(
        b"dummy audio output started at Fs=%d Hz\n\0" as *const u8
            as *const libc::c_char,
        sample_rate,
    );
}
unsafe extern "C" fn play(mut buf: *mut libc::c_short, mut samples: libc::c_int) {
    let mut tv: timeval = timeval { tv_sec: 0, tv_usec: 0 };
    gettimeofday(&mut tv, 0 as *mut libc::c_void);
    let mut nowtime: libc::c_longlong = (tv.tv_usec as libc::c_double
        + 1e6f64 * tv.tv_sec as libc::c_double) as libc::c_longlong;
    if starttime == 0 {
        starttime = nowtime;
    }
    samples_played += samples as libc::c_longlong;
    let mut finishtime: libc::c_longlong = (starttime as libc::c_double
        + samples_played as libc::c_double * 1e6f64 / Fs as libc::c_double)
        as libc::c_longlong;
    usleep((finishtime - nowtime) as __useconds_t);
}
unsafe extern "C" fn stop() {
    printf(b"dummy audio stopped\n\0" as *const u8 as *const libc::c_char);
}
unsafe extern "C" fn help() {
    printf(
        b"    There are no options for dummy audio.\n\0" as *const u8
            as *const libc::c_char,
    );
}
pub static mut audio_dummy: audio_output = {
    let mut init = audio_output {
        help: Some(help as unsafe extern "C" fn() -> ()),
        name: b"dummy\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        init: Some(
            init
                as unsafe extern "C" fn(
                    libc::c_int,
                    *mut *mut libc::c_char,
                ) -> libc::c_int,
        ),
        deinit: Some(deinit as unsafe extern "C" fn() -> ()),
        start: Some(start as unsafe extern "C" fn(libc::c_int) -> ()),
        play: Some(play as unsafe extern "C" fn(*mut libc::c_short, libc::c_int) -> ()),
        stop: Some(stop as unsafe extern "C" fn() -> ()),
        volume: None,
    };
    init
};
