use ::libc;
extern "C" {
    fn printf(_: *const libc::c_char, _: ...) -> libc::c_int;
    fn strcasecmp(_: *const libc::c_char, _: *const libc::c_char) -> libc::c_int;
    static mut audio_ao: audio_output;
    static mut audio_pulse: audio_output;
    static mut audio_alsa: audio_output;
    static mut audio_dummy: audio_output;
    static mut audio_pipe: audio_output;
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
static mut outputs: [*mut audio_output; 6] = unsafe {
    [
        &audio_alsa as *const audio_output as *mut audio_output,
        &audio_pulse as *const audio_output as *mut audio_output,
        &audio_ao as *const audio_output as *mut audio_output,
        &audio_dummy as *const audio_output as *mut audio_output,
        &audio_pipe as *const audio_output as *mut audio_output,
        0 as *const audio_output as *mut audio_output,
    ]
};
pub unsafe extern "C" fn audio_get_output(
    mut name: *mut libc::c_char,
) -> *mut audio_output {
    let mut out: *mut *mut audio_output = 0 as *mut *mut audio_output;
    if name.is_null() {
        return outputs[0 as libc::c_int as usize];
    }
    out = outputs.as_mut_ptr();
    while !(*out).is_null() {
        if strcasecmp(name, (**out).name) == 0 {
            return *out;
        }
        out = out.offset(1);
        out;
    }
    return 0 as *mut audio_output;
}
pub unsafe extern "C" fn audio_ls_outputs() {
    let mut out: *mut *mut audio_output = 0 as *mut *mut audio_output;
    printf(b"Available audio outputs:\n\0" as *const u8 as *const libc::c_char);
    out = outputs.as_mut_ptr();
    while !(*out).is_null() {
        printf(
            b"    %s%s\n\0" as *const u8 as *const libc::c_char,
            (**out).name,
            if out == outputs.as_mut_ptr() {
                b" (default)\0" as *const u8 as *const libc::c_char
            } else {
                b"\0" as *const u8 as *const libc::c_char
            },
        );
        out = out.offset(1);
        out;
    }
    out = outputs.as_mut_ptr();
    while !(*out).is_null() {
        printf(b"\n\0" as *const u8 as *const libc::c_char);
        printf(
            b"Options for output %s:\n\0" as *const u8 as *const libc::c_char,
            (**out).name,
        );
        ((**out).help).unwrap()();
        out = out.offset(1);
        out;
    }
}
