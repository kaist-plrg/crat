use ::libc;
extern "C" {
    fn exit(_: libc::c_int) -> !;
    fn perror(__s: *const libc::c_char);
    fn execl(
        __path: *const libc::c_char,
        __arg: *const libc::c_char,
        _: ...
    ) -> libc::c_int;
    fn fork() -> __pid_t;
    fn wait(__stat_loc: *mut libc::c_int) -> __pid_t;
}
pub type __uint8_t = libc::c_uchar;
pub type __uint16_t = libc::c_ushort;
pub type __uint32_t = libc::c_uint;
pub type __off_t = libc::c_long;
pub type __pid_t = libc::c_int;
pub type off_t = __off_t;
pub type pid_t = __pid_t;
pub type uint8_t = __uint8_t;
pub type uint16_t = __uint16_t;
pub type uint32_t = __uint32_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct device {
    pub name: *const libc::c_char,
    pub drive: libc::c_char,
    pub fat_bits: libc::c_int,
    pub mode: libc::c_int,
    pub tracks: libc::c_uint,
    pub heads: uint16_t,
    pub sectors: uint16_t,
    pub hidden: libc::c_uint,
    pub offset: off_t,
    pub partition: libc::c_uint,
    pub misc_flags: libc::c_uint,
    pub ssize: uint8_t,
    pub use_2m: libc::c_uint,
    pub precmd: *mut libc::c_char,
    pub file_nr: libc::c_int,
    pub blocksize: libc::c_uint,
    pub codepage: libc::c_uint,
    pub data_map: *const libc::c_char,
    pub tot_sectors: uint32_t,
    pub sector_size: uint16_t,
    pub postcmd: *mut libc::c_char,
    pub cfg_filename: *const libc::c_char,
}
pub unsafe extern "C" fn precmd(mut dev: *mut device) {
    if dev.is_null() {
        return;
    }
    postcmd((*dev).precmd);
}
pub unsafe extern "C" fn postcmd(mut cmd: *const libc::c_char) {
    let mut status: libc::c_int = 0;
    let mut pid: pid_t = 0;
    if cmd.is_null() {
        return;
    }
    pid = fork();
    match pid {
        -1 => {
            perror(b"Could not fork\0" as *const u8 as *const libc::c_char);
            exit(1 as libc::c_int);
        }
        0 => {
            execl(
                b"/bin/sh\0" as *const u8 as *const libc::c_char,
                b"sh\0" as *const u8 as *const libc::c_char,
                b"-c\0" as *const u8 as *const libc::c_char,
                cmd,
                0 as *mut libc::c_void as *mut libc::c_char,
            );
        }
        _ => {
            wait(&mut status);
        }
    };
}
