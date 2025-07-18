use ::libc;
use ::c2rust_asm_casts;
use c2rust_asm_casts::AsmCastTrait;
use std::arch::asm;
extern "C" {
    fn sprintf(_: *mut libc::c_char, _: *const libc::c_char, _: ...) -> libc::c_int;
    fn vsprintf(
        _: *mut libc::c_char,
        _: *const libc::c_char,
        _: ::std::ffi::VaList,
    ) -> libc::c_int;
    fn malloc(_: libc::c_ulong) -> *mut libc::c_void;
    fn free(__ptr: *mut libc::c_void);
    fn exit(_: libc::c_int) -> !;
    fn select(
        __nfds: libc::c_int,
        __readfds: *mut fd_set,
        __writefds: *mut fd_set,
        __exceptfds: *mut fd_set,
        __timeout: *mut timeval,
    ) -> libc::c_int;
    fn __ctype_b_loc() -> *mut *const libc::c_ushort;
    fn __ctype_tolower_loc() -> *mut *const __int32_t;
    fn __ctype_toupper_loc() -> *mut *const __int32_t;
    fn strcpy(_: *mut libc::c_char, _: *const libc::c_char) -> *mut libc::c_char;
    fn strncpy(
        _: *mut libc::c_char,
        _: *const libc::c_char,
        _: libc::c_ulong,
    ) -> *mut libc::c_char;
    fn strcmp(_: *const libc::c_char, _: *const libc::c_char) -> libc::c_int;
    fn strncmp(
        _: *const libc::c_char,
        _: *const libc::c_char,
        _: libc::c_ulong,
    ) -> libc::c_int;
    fn strlen(_: *const libc::c_char) -> libc::c_ulong;
    fn Pl_LE_Open_Terminal();
    fn Pl_LE_Close_Terminal();
    fn Pl_LE_Is_Interrupt_Key(c: libc::c_int) -> libc::c_int;
    fn Pl_LE_Get_Char() -> libc::c_int;
    fn Pl_Emit_Ctrl_C() -> PlLong;
    fn Pl_LE_Initialize() -> libc::c_int;
}
pub type __builtin_va_list = [__va_list_tag; 1];
#[derive(Copy, Clone)]
#[repr(C)]
pub struct __va_list_tag {
    pub gp_offset: libc::c_uint,
    pub fp_offset: libc::c_uint,
    pub overflow_arg_area: *mut libc::c_void,
    pub reg_save_area: *mut libc::c_void,
}
pub type va_list = __builtin_va_list;
pub type __int32_t = libc::c_int;
pub type __time_t = libc::c_long;
pub type __suseconds_t = libc::c_long;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct timeval {
    pub tv_sec: __time_t,
    pub tv_usec: __suseconds_t,
}
pub type __fd_mask = libc::c_long;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct fd_set {
    pub __fds_bits: [__fd_mask; 16],
}
pub type C2RustUnnamed = libc::c_uint;
pub const _ISalnum: C2RustUnnamed = 8;
pub const _ISpunct: C2RustUnnamed = 4;
pub const _IScntrl: C2RustUnnamed = 2;
pub const _ISblank: C2RustUnnamed = 1;
pub const _ISgraph: C2RustUnnamed = 32768;
pub const _ISprint: C2RustUnnamed = 16384;
pub const _ISspace: C2RustUnnamed = 8192;
pub const _ISxdigit: C2RustUnnamed = 4096;
pub const _ISdigit: C2RustUnnamed = 2048;
pub const _ISalpha: C2RustUnnamed = 1024;
pub const _ISlower: C2RustUnnamed = 512;
pub const _ISupper: C2RustUnnamed = 256;
pub type intptr_t = libc::c_long;
pub type PlLong = intptr_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct HistCell {
    pub buff_length: libc::c_int,
    pub line_length: libc::c_int,
    pub line: *mut libc::c_char,
}
pub type CompNode = comp_node;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct comp_node {
    pub word: *mut libc::c_char,
    pub word_length: libc::c_int,
    pub next: *mut CompNode,
}
#[inline]
unsafe extern "C" fn tolower(mut __c: libc::c_int) -> libc::c_int {
    return if __c >= -(128 as libc::c_int) && __c < 256 as libc::c_int {
        *(*__ctype_tolower_loc()).offset(__c as isize)
    } else {
        __c
    };
}
#[inline]
unsafe extern "C" fn toupper(mut __c: libc::c_int) -> libc::c_int {
    return if __c >= -(128 as libc::c_int) && __c < 256 as libc::c_int {
        *(*__ctype_toupper_loc()).offset(__c as isize)
    } else {
        __c
    };
}
pub static mut pl_le_hook_start: Option::<unsafe extern "C" fn() -> ()> = None;
pub static mut pl_le_hook_put_char: Option::<unsafe extern "C" fn() -> ()> = None;
pub static mut pl_le_hook_get_char0: Option::<unsafe extern "C" fn() -> libc::c_int> = None;
pub static mut pl_le_hook_emit_beep: Option::<unsafe extern "C" fn() -> ()> = None;
pub static mut pl_le_hook_ins_mode: Option::<unsafe extern "C" fn() -> ()> = None;
pub static mut pl_le_hook_screen_size: Option::<unsafe extern "C" fn() -> ()> = None;
pub static mut pl_le_hook_kbd_is_not_empty: Option::<
    unsafe extern "C" fn() -> libc::c_int,
> = None;
pub static mut pl_le_hook_backd: Option::<unsafe extern "C" fn() -> ()> = None;
pub static mut pl_le_hook_forwd: Option::<unsafe extern "C" fn() -> ()> = None;
pub static mut pl_le_hook_displ: Option::<unsafe extern "C" fn() -> ()> = None;
pub static mut pl_le_hook_displ_str: Option::<unsafe extern "C" fn() -> ()> = None;
pub static mut pl_le_hook_erase: Option::<unsafe extern "C" fn() -> ()> = None;
pub static mut pl_le_hook_set_line_buffering: Option::<unsafe extern "C" fn() -> ()> = None;
pub static mut pl_le_hook_get_line_buffering: Option::<
    unsafe extern "C" fn() -> libc::c_int,
> = None;
pub static mut pl_le_hook_flush: Option::<unsafe extern "C" fn() -> ()> = None;
pub static mut pl_le_hook_confirm_box: Option::<unsafe extern "C" fn() -> libc::c_int> = None;
pub static mut pl_le_hook_message_box: Option::<unsafe extern "C" fn() -> ()> = None;
pub static mut pl_le_hook_exit_process: Option::<unsafe extern "C" fn() -> ()> = None;
pub static mut pl_le_initialize: Option::<unsafe extern "C" fn() -> libc::c_int> = None;
static mut separators: [libc::c_char; 256] = unsafe {
    *::std::mem::transmute::<
        &[u8; 256],
        &mut [libc::c_char; 256],
    >(
        b" ,;:-'\"!@$#^&()-+*/\\[]|<=>`~{}\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0",
    )
};
static mut ins_mode: libc::c_int = 1 as libc::c_int;
static mut global_str: *mut libc::c_char = 0 as *const libc::c_char as *mut libc::c_char;
static mut global_pos: *mut libc::c_char = 0 as *const libc::c_char as *mut libc::c_char;
static mut global_end: *mut libc::c_char = 0 as *const libc::c_char as *mut libc::c_char;
static mut prompt_length: libc::c_int = 0;
static mut ctrl_c_ret_val: PlLong = 0;
static mut clipboard: [libc::c_char; 4096] = unsafe {
    *::std::mem::transmute::<
        &[u8; 4096],
        &mut [libc::c_char; 4096],
    >(
        b"\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0",
    )
};
static mut hist_tbl: [HistCell; 64] = [HistCell {
    buff_length: 0,
    line_length: 0,
    line: 0 as *const libc::c_char as *mut libc::c_char,
}; 64];
static mut hist_start: libc::c_int = 0 as libc::c_int;
static mut hist_end: libc::c_int = 0 as libc::c_int;
static mut comp_start: *mut CompNode = 0 as *const CompNode as *mut CompNode;
static mut comp_first_match: *mut CompNode = 0 as *const CompNode as *mut CompNode;
static mut comp_last_match: *mut CompNode = 0 as *const CompNode as *mut CompNode;
static mut comp_nb_match: libc::c_int = 0;
static mut comp_match_max_lg: libc::c_int = 0;
static mut comp_cur_match: *mut CompNode = 0 as *const CompNode as *mut CompNode;
pub unsafe extern "C" fn Pl_LE_Gets(mut str: *mut libc::c_char) -> *mut libc::c_char {
    let mut l: libc::c_int = 0;
    let mut big_size: libc::c_int = (-(1 as libc::c_int) as libc::c_uint
        >> 1 as libc::c_int) as libc::c_int;
    str = Pl_LE_FGets(str, big_size, 0 as *mut libc::c_char, 0 as libc::c_int);
    if !str.is_null() {
        l = (strlen(str)).wrapping_sub(1 as libc::c_int as libc::c_ulong) as libc::c_int;
        if l >= 0 as libc::c_int && *str.offset(l as isize) as libc::c_int == '\n' as i32
        {
            *str.offset(l as isize) = '\0' as i32 as libc::c_char;
        }
    }
    return str;
}
pub unsafe extern "C" fn Pl_LE_FGets(
    mut str: *mut libc::c_char,
    mut size: libc::c_int,
    mut prompt: *mut libc::c_char,
    mut display_prompt: libc::c_int,
) -> *mut libc::c_char {
    let mut save_prompt_length: libc::c_int = 0;
    let mut current_block: u64;
    let mut pos: *mut libc::c_char = str;
    let mut end: *mut libc::c_char = str;
    let mut mark: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut p: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut q: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut start: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut stop: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut w: libc::c_char = 0;
    let mut c: libc::c_int = 0;
    let mut n: libc::c_int = 0;
    let mut n1: libc::c_int = 0;
    let mut last_was_eof: libc::c_int = 0 as libc::c_int;
    let mut h_no: libc::c_int = hist_end;
    let mut rest_length: libc::c_int = 0;
    let mut tab_count: libc::c_int = 0 as libc::c_int;
    let mut count_bracket: [libc::c_int; 3] = [0; 3];
    Pl_LE_Initialize();
    size -= 1;
    size;
    prompt_length = (if !prompt.is_null() && display_prompt != 0 {
        strlen(prompt)
    } else {
        0 as libc::c_int as libc::c_ulong
    }) as libc::c_int;
    Pl_LE_Open_Terminal();
    global_str = str;
    global_pos = pos;
    global_end = end;
    loop {
        if !(::std::mem::transmute::<
            _,
            fn() -> libc::c_int,
        >((Some(pl_le_hook_kbd_is_not_empty.unwrap())).unwrap())() != 0)
        {
            current_block = 12147880666119273379;
            break;
        }
        if end.offset_from(str) as libc::c_long >= size as libc::c_long
            || {
                c = Pl_LE_Get_Char();
                c == '\n' as i32
            } || c == '\r' as i32
        {
            if !prompt.is_null() && display_prompt != 0 {
                ::std::mem::transmute::<
                    _,
                    fn(_),
                >((Some(pl_le_hook_displ_str.unwrap())).unwrap())(prompt);
            }
            ::std::mem::transmute::<
                _,
                fn(_, _),
            >(
                (Some(pl_le_hook_displ.unwrap())).unwrap(),
            )(end.offset_from(str) as libc::c_long, str);
            ::std::mem::transmute::<
                _,
                fn(_),
            >(
                (Some(pl_le_hook_backd.unwrap())).unwrap(),
            )(end.offset_from(pos) as libc::c_long);
            current_block = 291999910655467784;
            break;
        } else if c == '\t' as i32 {
            n = Tab_To_Spaces(end.offset_from(str) as libc::c_long as libc::c_int);
            while n != 0 {
                let fresh0 = end;
                end = end.offset(1);
                *fresh0 = ' ' as i32 as libc::c_char;
                n -= 1;
                n;
            }
        } else {
            let fresh1 = end;
            end = end.offset(1);
            *fresh1 = c as libc::c_char;
        }
    }
    match current_block {
        12147880666119273379 => {
            if end != str {
                pos = end;
                current_block = 8610049092612950667;
            } else {
                if !prompt.is_null() && display_prompt != 0 {
                    ::std::mem::transmute::<
                        _,
                        fn(_),
                    >((Some(pl_le_hook_displ_str.unwrap())).unwrap())(prompt);
                }
                current_block = 7172762164747879670;
            }
            '_re_display_line: loop {
                match current_block {
                    8610049092612950667 => {
                        if !prompt.is_null() && display_prompt != 0 {
                            ::std::mem::transmute::<
                                _,
                                fn(_),
                            >((Some(pl_le_hook_displ_str.unwrap())).unwrap())(prompt);
                        }
                        ::std::mem::transmute::<
                            _,
                            fn(_, _),
                        >(
                            (Some(pl_le_hook_displ.unwrap())).unwrap(),
                        )(end.offset_from(str) as libc::c_long, str);
                        ::std::mem::transmute::<
                            _,
                            fn(_),
                        >(
                            (Some(pl_le_hook_backd.unwrap())).unwrap(),
                        )(end.offset_from(pos) as libc::c_long);
                        current_block = 7172762164747879670;
                    }
                    _ => {
                        global_pos = pos;
                        global_end = end;
                        c = Pl_LE_Get_Char();
                        loop {
                            *end = ' ' as i32 as libc::c_char;
                            if Pl_LE_Is_Interrupt_Key(c) != 0 {
                                save_prompt_length = prompt_length;
                                ::std::mem::transmute::<
                                    _,
                                    fn(_, _),
                                >(
                                    (Some(pl_le_hook_forwd.unwrap())).unwrap(),
                                )(end.offset_from(pos) as libc::c_long, pos);
                                Pl_LE_Close_Terminal();
                                c = *end as libc::c_int;
                                *end = '\0' as i32 as libc::c_char;
                                ctrl_c_ret_val = Pl_Emit_Ctrl_C();
                                if ctrl_c_ret_val != 0 as libc::c_int as libc::c_long {
                                    return -(2 as libc::c_int) as *mut libc::c_char;
                                }
                                Pl_LE_Open_Terminal();
                                *end = c as libc::c_char;
                                global_str = str;
                                prompt_length = save_prompt_length;
                                current_block = 8610049092612950667;
                                continue '_re_display_line;
                            } else {
                                if c == 'D' as i32 & 0x1f as libc::c_int {
                                    if end == str {
                                        if c == 'D' as i32 & 0x1f as libc::c_int
                                            && last_was_eof != 0
                                        {
                                            current_block = 18007440463631195037;
                                            break;
                                        } else {
                                            current_block = 1622411330066726685;
                                            break;
                                        }
                                    } else {
                                        last_was_eof = (c == 'D' as i32 & 0x1f as libc::c_int)
                                            as libc::c_int;
                                    }
                                } else {
                                    last_was_eof = 0 as libc::c_int;
                                }
                                if c != '\t' as i32 {
                                    tab_count = 0 as libc::c_int;
                                }
                                match c {
                                    1 | 328 => {
                                        ::std::mem::transmute::<
                                            _,
                                            fn(_),
                                        >(
                                            (Some(pl_le_hook_backd.unwrap())).unwrap(),
                                        )(pos.offset_from(str) as libc::c_long);
                                        pos = str;
                                        current_block = 7172762164747879670;
                                        continue '_re_display_line;
                                    }
                                    5 | 326 => {
                                        ::std::mem::transmute::<
                                            _,
                                            fn(_, _),
                                        >(
                                            (Some(pl_le_hook_forwd.unwrap())).unwrap(),
                                        )(end.offset_from(pos) as libc::c_long, pos);
                                        pos = end;
                                        current_block = 7172762164747879670;
                                        continue '_re_display_line;
                                    }
                                    2 | 324 => {
                                        if pos == str {
                                            current_block = 18007440463631195037;
                                            break;
                                        } else {
                                            current_block = 2520131295878969859;
                                            break;
                                        }
                                    }
                                    6 | 323 => {
                                        if pos == end {
                                            current_block = 18007440463631195037;
                                            break;
                                        } else {
                                            current_block = 7252614138838059896;
                                            break;
                                        }
                                    }
                                    8 | 127 => {
                                        if pos == str {
                                            current_block = 18007440463631195037;
                                            break;
                                        } else {
                                            current_block = 4801495893854419819;
                                            break;
                                        }
                                    }
                                    4 | 259 => {
                                        if pos == end {
                                            current_block = 18007440463631195037;
                                            break;
                                        } else {
                                            current_block = 2723324002591448311;
                                            break;
                                        }
                                    }
                                    21 | 2376 => {
                                        q = clipboard.as_mut_ptr();
                                        p = str;
                                        while p < pos {
                                            let fresh2 = p;
                                            p = p.offset(1);
                                            let fresh3 = q;
                                            q = q.offset(1);
                                            *fresh3 = *fresh2;
                                        }
                                        *q = '\0' as i32 as libc::c_char;
                                        n = pos.offset_from(str) as libc::c_long as libc::c_int;
                                        p = pos;
                                        while p < end {
                                            *p.offset(-n as isize) = *p;
                                            p = p.offset(1);
                                            p;
                                        }
                                        pos = str;
                                        end = end.offset(-(n as isize));
                                        ::std::mem::transmute::<
                                            _,
                                            fn(_),
                                        >((Some(pl_le_hook_backd.unwrap())).unwrap())(n);
                                        ::std::mem::transmute::<
                                            _,
                                            fn(_, _),
                                        >(
                                            (Some(pl_le_hook_displ.unwrap())).unwrap(),
                                        )(end.offset_from(pos) as libc::c_long, pos);
                                        ::std::mem::transmute::<
                                            _,
                                            fn(_),
                                        >((Some(pl_le_hook_erase.unwrap())).unwrap())(n);
                                        ::std::mem::transmute::<
                                            _,
                                            fn(_),
                                        >(
                                            (Some(pl_le_hook_backd.unwrap())).unwrap(),
                                        )(end.offset_from(pos) as libc::c_long);
                                        current_block = 7172762164747879670;
                                        continue '_re_display_line;
                                    }
                                    11 | 2374 => {
                                        q = clipboard.as_mut_ptr();
                                        p = pos;
                                        while p < end {
                                            let fresh4 = p;
                                            p = p.offset(1);
                                            let fresh5 = q;
                                            q = q.offset(1);
                                            *fresh5 = *fresh4;
                                        }
                                        *q = '\0' as i32 as libc::c_char;
                                        ::std::mem::transmute::<
                                            _,
                                            fn(_),
                                        >(
                                            (Some(pl_le_hook_erase.unwrap())).unwrap(),
                                        )(end.offset_from(pos) as libc::c_long);
                                        end = pos;
                                        current_block = 7172762164747879670;
                                        continue '_re_display_line;
                                    }
                                    25 => {
                                        p = clipboard.as_mut_ptr();
                                        current_block = 7301440000599063274;
                                        break;
                                    }
                                    0 => {
                                        mark = pos;
                                        current_block = 7172762164747879670;
                                        continue '_re_display_line;
                                    }
                                    375 => {
                                        current_block = 3285925561237090922;
                                        break;
                                    }
                                    23 => {
                                        current_block = 3285925561237090922;
                                        break;
                                    }
                                    354 | 2372 => {
                                        p = if pos == str {
                                            pos
                                        } else {
                                            pos.offset(-(1 as libc::c_int as isize))
                                        };
                                        p = Skip(p, str, 1 as libc::c_int, -(1 as libc::c_int));
                                        p = Skip(p, str, 0 as libc::c_int, -(1 as libc::c_int));
                                        p = Skip(p, end, 1 as libc::c_int, 1 as libc::c_int);
                                        ::std::mem::transmute::<
                                            _,
                                            fn(_),
                                        >(
                                            (Some(pl_le_hook_backd.unwrap())).unwrap(),
                                        )(pos.offset_from(p) as libc::c_long);
                                        pos = p;
                                        current_block = 7172762164747879670;
                                        continue '_re_display_line;
                                    }
                                    358 | 2371 => {
                                        p = pos;
                                        p = Skip(p, end, 0 as libc::c_int, 1 as libc::c_int);
                                        p = Skip(p, end, 1 as libc::c_int, 1 as libc::c_int);
                                        ::std::mem::transmute::<
                                            _,
                                            fn(_, _),
                                        >(
                                            (Some(pl_le_hook_forwd.unwrap())).unwrap(),
                                        )(p.offset_from(pos) as libc::c_long, pos);
                                        pos = p;
                                        current_block = 7172762164747879670;
                                        continue '_re_display_line;
                                    }
                                    355 => {
                                        p = pos;
                                        p = Skip(p, end, 1 as libc::c_int, 1 as libc::c_int);
                                        if *(*__ctype_b_loc()).offset(*p as libc::c_int as isize)
                                            as libc::c_int
                                            & _ISlower as libc::c_int as libc::c_ushort as libc::c_int
                                            != 0
                                        {
                                            *p = (*p as libc::c_int - 'a' as i32 + 'A' as i32)
                                                as libc::c_char;
                                        }
                                        p = Skip(p, end, 0 as libc::c_int, 1 as libc::c_int);
                                        ::std::mem::transmute::<
                                            _,
                                            fn(_, _),
                                        >(
                                            (Some(pl_le_hook_displ.unwrap())).unwrap(),
                                        )(p.offset_from(pos) as libc::c_long, pos);
                                        pos = p;
                                        current_block = 7172762164747879670;
                                        continue '_re_display_line;
                                    }
                                    364 => {
                                        p = pos;
                                        p = Skip(p, end, 1 as libc::c_int, 1 as libc::c_int);
                                        while p < end && Is_A_Separator(*p) == 0 {
                                            *p = ({
                                                let mut __res: libc::c_int = 0;
                                                if ::std::mem::size_of::<libc::c_char>() as libc::c_ulong
                                                    > 1 as libc::c_int as libc::c_ulong
                                                {
                                                    if 0 != 0 {
                                                        let mut __c: libc::c_int = *p as libc::c_int;
                                                        __res = if __c < -(128 as libc::c_int)
                                                            || __c > 255 as libc::c_int
                                                        {
                                                            __c
                                                        } else {
                                                            *(*__ctype_tolower_loc()).offset(__c as isize)
                                                        };
                                                    } else {
                                                        __res = tolower(*p as libc::c_int);
                                                    }
                                                } else {
                                                    __res = *(*__ctype_tolower_loc())
                                                        .offset(*p as libc::c_int as isize);
                                                }
                                                __res
                                            }) as libc::c_char;
                                            p = p.offset(1);
                                            p;
                                        }
                                        ::std::mem::transmute::<
                                            _,
                                            fn(_, _),
                                        >(
                                            (Some(pl_le_hook_displ.unwrap())).unwrap(),
                                        )(p.offset_from(pos) as libc::c_long, pos);
                                        pos = p;
                                        current_block = 7172762164747879670;
                                        continue '_re_display_line;
                                    }
                                    373 => {
                                        p = pos;
                                        p = Skip(p, end, 1 as libc::c_int, 1 as libc::c_int);
                                        while p < end && Is_A_Separator(*p) == 0 {
                                            *p = ({
                                                let mut __res: libc::c_int = 0;
                                                if ::std::mem::size_of::<libc::c_char>() as libc::c_ulong
                                                    > 1 as libc::c_int as libc::c_ulong
                                                {
                                                    if 0 != 0 {
                                                        let mut __c: libc::c_int = *p as libc::c_int;
                                                        __res = if __c < -(128 as libc::c_int)
                                                            || __c > 255 as libc::c_int
                                                        {
                                                            __c
                                                        } else {
                                                            *(*__ctype_toupper_loc()).offset(__c as isize)
                                                        };
                                                    } else {
                                                        __res = toupper(*p as libc::c_int);
                                                    }
                                                } else {
                                                    __res = *(*__ctype_toupper_loc())
                                                        .offset(*p as libc::c_int as isize);
                                                }
                                                __res
                                            }) as libc::c_char;
                                            p = p.offset(1);
                                            p;
                                        }
                                        ::std::mem::transmute::<
                                            _,
                                            fn(_, _),
                                        >(
                                            (Some(pl_le_hook_displ.unwrap())).unwrap(),
                                        )(p.offset_from(pos) as libc::c_long, pos);
                                        pos = p;
                                        current_block = 7172762164747879670;
                                        continue '_re_display_line;
                                    }
                                    9 => {
                                        if tab_count != 0 as libc::c_int {
                                            current_block = 11227437541145425351;
                                            break;
                                        } else {
                                            current_block = 4976922244085895320;
                                            break;
                                        }
                                    }
                                    297 => {
                                        n = Tab_To_Spaces(
                                            pos.offset_from(str) as libc::c_long as libc::c_int,
                                        );
                                        current_block = 7173345243791314703;
                                        break;
                                    }
                                    22 | 258 => {
                                        ins_mode = 1 as libc::c_int - ins_mode;
                                        ::std::mem::transmute::<
                                            _,
                                            fn(_),
                                        >((Some(pl_le_hook_ins_mode.unwrap())).unwrap())(ins_mode);
                                        current_block = 7172762164747879670;
                                        continue '_re_display_line;
                                    }
                                    20 => {
                                        if pos == str || pos == end {
                                            current_block = 18007440463631195037;
                                            break;
                                        } else {
                                            current_block = 16576776889580422803;
                                            break;
                                        }
                                    }
                                    10 | 13 => {
                                        current_block = 291999910655467784;
                                        break '_re_display_line;
                                    }
                                    16 | 321 => {
                                        if hist_start == hist_end || h_no == hist_start {
                                            current_block = 18007440463631195037;
                                            break;
                                        } else {
                                            current_block = 3151994457458062110;
                                            break;
                                        }
                                    }
                                    14 | 322 => {
                                        if hist_start == hist_end || h_no == hist_end {
                                            current_block = 18007440463631195037;
                                            break;
                                        } else {
                                            current_block = 1587619384396752891;
                                            break;
                                        }
                                    }
                                    368 => {
                                        if hist_start == hist_end || pos == str {
                                            current_block = 18007440463631195037;
                                            break;
                                        } else {
                                            current_block = 9467764101860050311;
                                            break;
                                        }
                                    }
                                    366 => {
                                        if hist_start == hist_end || pos == str {
                                            current_block = 18007440463631195037;
                                            break;
                                        } else {
                                            current_block = 12343738388509029619;
                                            break;
                                        }
                                    }
                                    316 | 261 => {
                                        if hist_start == hist_end || h_no == hist_start {
                                            current_block = 18007440463631195037;
                                            break;
                                        } else {
                                            current_block = 8288085890650723895;
                                            break;
                                        }
                                    }
                                    318 | 262 => {
                                        if hist_start == hist_end || h_no == hist_end {
                                            current_block = 18007440463631195037;
                                            break;
                                        } else {
                                            current_block = 8038949400865391589;
                                            break;
                                        }
                                    }
                                    319 => {
                                        current_block = 11990075014260372988;
                                        break;
                                    }
                                    _ => {
                                        if c as libc::c_uint > 255 as libc::c_int as libc::c_uint
                                            || *(*__ctype_b_loc()).offset(c as isize) as libc::c_int
                                                & _ISprint as libc::c_int as libc::c_ushort as libc::c_int
                                                == 0
                                        {
                                            n = c;
                                            ::std::mem::transmute::<
                                                _,
                                                fn(),
                                            >((Some(pl_le_hook_emit_beep.unwrap())).unwrap())();
                                            c = Pl_LE_Get_Char();
                                            if !(c != n) {
                                                current_block = 11990075014260372988;
                                                break;
                                            }
                                        } else if New_Char(c, str, size, &mut pos, &mut end) == 0 {
                                            current_block = 18007440463631195037;
                                            break;
                                        } else {
                                            current_block = 11254128328725829733;
                                            break;
                                        }
                                    }
                                }
                            }
                        }
                        match current_block {
                            4976922244085895320 => {
                                p = if pos == str {
                                    pos
                                } else {
                                    pos.offset(-(1 as libc::c_int as isize))
                                };
                                p = Skip(p, str, 0 as libc::c_int, -(1 as libc::c_int));
                                p = Skip(p, end, 1 as libc::c_int, 1 as libc::c_int);
                                w = *pos;
                                *pos = '\0' as i32 as libc::c_char;
                                p = Completion_Do_Match(
                                    p,
                                    pos.offset_from(p) as libc::c_long as libc::c_int,
                                    &mut rest_length,
                                );
                                *pos = w;
                                if p.is_null() {
                                    current_block = 18007440463631195037;
                                } else {
                                    loop {
                                        let fresh8 = rest_length;
                                        rest_length = rest_length - 1;
                                        if !(fresh8 != 0) {
                                            current_block = 6535105651042291885;
                                            break;
                                        }
                                        let fresh9 = p;
                                        p = p.offset(1);
                                        if New_Char(
                                            *fresh9 as libc::c_int,
                                            str,
                                            size,
                                            &mut pos,
                                            &mut end,
                                        ) == 0
                                        {
                                            current_block = 18007440463631195037;
                                            break;
                                        }
                                    }
                                    match current_block {
                                        18007440463631195037 => {}
                                        _ => {
                                            if comp_first_match != comp_last_match {
                                                tab_count = 1 as libc::c_int;
                                            } else {
                                                tab_count = 0 as libc::c_int;
                                                current_block = 7172762164747879670;
                                                continue;
                                            }
                                            current_block = 18007440463631195037;
                                        }
                                    }
                                }
                            }
                            7173345243791314703 => {
                                loop {
                                    if !(n != 0) {
                                        current_block = 7172762164747879670;
                                        continue '_re_display_line;
                                    }
                                    if New_Char(' ' as i32, str, size, &mut pos, &mut end) == 0
                                    {
                                        break;
                                    }
                                    n -= 1;
                                    n;
                                }
                                current_block = 18007440463631195037;
                            }
                            11227437541145425351 => {
                                tab_count += 1;
                                if tab_count > 1 as libc::c_int {
                                    ::std::mem::transmute::<
                                        _,
                                        fn(_),
                                    >(
                                        (Some(pl_le_hook_put_char.unwrap())).unwrap(),
                                    )('\n' as i32);
                                    Completion_Print_All();
                                    current_block = 8610049092612950667;
                                    continue;
                                } else {
                                    current_block = 18007440463631195037;
                                }
                            }
                            16576776889580422803 => {
                                w = *pos.offset(0 as libc::c_int as isize);
                                *pos
                                    .offset(
                                        0 as libc::c_int as isize,
                                    ) = *pos.offset(-(1 as libc::c_int) as isize);
                                *pos.offset(-(1 as libc::c_int) as isize) = w;
                                ::std::mem::transmute::<
                                    _,
                                    fn(_),
                                >(
                                    (Some(pl_le_hook_backd.unwrap())).unwrap(),
                                )(1 as libc::c_int);
                                ::std::mem::transmute::<
                                    _,
                                    fn(_, _),
                                >(
                                    (Some(pl_le_hook_displ.unwrap())).unwrap(),
                                )(
                                    2 as libc::c_int,
                                    pos.offset(-(1 as libc::c_int as isize)),
                                );
                                pos = pos.offset(1);
                                pos;
                                current_block = 7172762164747879670;
                                continue;
                            }
                            3151994457458062110 => {
                                *end = '\0' as i32 as libc::c_char;
                                History_Update_Line(
                                    str,
                                    end.offset_from(str) as libc::c_long as libc::c_int,
                                    h_no,
                                );
                                h_no -= 1;
                                if h_no < 0 as libc::c_int {
                                    h_no = 64 as libc::c_int - 1 as libc::c_int;
                                }
                                current_block = 11089047473966393316;
                            }
                            3285925561237090922 => {
                                if mark.is_null() {
                                    current_block = 18007440463631195037;
                                } else {
                                    if mark < pos {
                                        start = mark;
                                        stop = pos;
                                    } else {
                                        start = pos;
                                        stop = mark;
                                    }
                                    q = clipboard.as_mut_ptr();
                                    p = start;
                                    while p < stop {
                                        let fresh6 = p;
                                        p = p.offset(1);
                                        let fresh7 = q;
                                        q = q.offset(1);
                                        *fresh7 = *fresh6;
                                    }
                                    *q = '\0' as i32 as libc::c_char;
                                    if c
                                        == (0 as libc::c_int) << 9 as libc::c_int
                                            | ((1 as libc::c_int) << 8 as libc::c_int
                                                | ((1 as libc::c_int) << 7 as libc::c_int | 'W' as i32
                                                    | 0x20 as libc::c_int) & 0x7f as libc::c_int)
                                    {
                                        current_block = 7172762164747879670;
                                        continue;
                                    }
                                    n = stop.offset_from(start) as libc::c_long as libc::c_int;
                                    p = stop;
                                    while p < end {
                                        *p.offset(-n as isize) = *p;
                                        p = p.offset(1);
                                        p;
                                    }
                                    if mark < pos {
                                        ::std::mem::transmute::<
                                            _,
                                            fn(_),
                                        >((Some(pl_le_hook_backd.unwrap())).unwrap())(n);
                                    }
                                    pos = start;
                                    end = end.offset(-(n as isize));
                                    ::std::mem::transmute::<
                                        _,
                                        fn(_, _),
                                    >(
                                        (Some(pl_le_hook_displ.unwrap())).unwrap(),
                                    )(end.offset_from(pos) as libc::c_long, pos);
                                    ::std::mem::transmute::<
                                        _,
                                        fn(_),
                                    >((Some(pl_le_hook_erase.unwrap())).unwrap())(n);
                                    ::std::mem::transmute::<
                                        _,
                                        fn(_),
                                    >(
                                        (Some(pl_le_hook_backd.unwrap())).unwrap(),
                                    )(end.offset_from(pos) as libc::c_long);
                                    current_block = 7172762164747879670;
                                    continue;
                                }
                            }
                            7301440000599063274 => {
                                loop {
                                    if !(*p != 0) {
                                        current_block = 7172762164747879670;
                                        continue '_re_display_line;
                                    }
                                    if New_Char(
                                        *p as libc::c_int,
                                        str,
                                        size,
                                        &mut pos,
                                        &mut end,
                                    ) == 0
                                    {
                                        break;
                                    }
                                    p = p.offset(1);
                                    p;
                                }
                                current_block = 18007440463631195037;
                            }
                            1587619384396752891 => {
                                *end = '\0' as i32 as libc::c_char;
                                History_Update_Line(
                                    str,
                                    end.offset_from(str) as libc::c_long as libc::c_int,
                                    h_no,
                                );
                                h_no += 1;
                                if h_no >= 64 as libc::c_int {
                                    h_no = 0 as libc::c_int;
                                }
                                current_block = 11089047473966393316;
                            }
                            9467764101860050311 => {
                                *end = '\0' as i32 as libc::c_char;
                                History_Update_Line(
                                    str,
                                    end.offset_from(str) as libc::c_long as libc::c_int,
                                    h_no,
                                );
                                loop {
                                    if h_no == hist_start {
                                        current_block = 18007440463631195037;
                                        break;
                                    }
                                    h_no -= 1;
                                    if h_no < 0 as libc::c_int {
                                        h_no = 64 as libc::c_int - 1 as libc::c_int;
                                    }
                                    if !((hist_tbl[h_no as usize].line).is_null()
                                        || strncmp(
                                            str,
                                            hist_tbl[h_no as usize].line,
                                            pos.offset_from(str) as libc::c_long as libc::c_ulong,
                                        ) != 0 as libc::c_int)
                                    {
                                        current_block = 15810102963287113283;
                                        break;
                                    }
                                }
                            }
                            2723324002591448311 => {
                                ::std::mem::transmute::<
                                    _,
                                    fn(_, _),
                                >(
                                    (Some(pl_le_hook_forwd.unwrap())).unwrap(),
                                )(1 as libc::c_int, pos);
                                pos = pos.offset(1);
                                pos;
                                current_block = 4801495893854419819;
                            }
                            12343738388509029619 => {
                                *end = '\0' as i32 as libc::c_char;
                                History_Update_Line(
                                    str,
                                    end.offset_from(str) as libc::c_long as libc::c_int,
                                    h_no,
                                );
                                loop {
                                    if hist_start == hist_end || h_no == hist_end {
                                        current_block = 18007440463631195037;
                                        break;
                                    }
                                    h_no += 1;
                                    if h_no >= 64 as libc::c_int {
                                        h_no = 0 as libc::c_int;
                                    }
                                    if !((hist_tbl[h_no as usize].line).is_null()
                                        || strncmp(
                                            str,
                                            hist_tbl[h_no as usize].line,
                                            pos.offset_from(str) as libc::c_long as libc::c_ulong,
                                        ) != 0 as libc::c_int)
                                    {
                                        current_block = 15810102963287113283;
                                        break;
                                    }
                                }
                            }
                            7252614138838059896 => {
                                ::std::mem::transmute::<
                                    _,
                                    fn(_, _),
                                >(
                                    (Some(pl_le_hook_forwd.unwrap())).unwrap(),
                                )(1 as libc::c_int, pos);
                                pos = pos.offset(1);
                                pos;
                                current_block = 7172762164747879670;
                                continue;
                            }
                            2520131295878969859 => {
                                ::std::mem::transmute::<
                                    _,
                                    fn(_),
                                >(
                                    (Some(pl_le_hook_backd.unwrap())).unwrap(),
                                )(1 as libc::c_int);
                                pos = pos.offset(-1);
                                pos;
                                current_block = 7172762164747879670;
                                continue;
                            }
                            8288085890650723895 => {
                                *end = '\0' as i32 as libc::c_char;
                                History_Update_Line(
                                    str,
                                    end.offset_from(str) as libc::c_long as libc::c_int,
                                    h_no,
                                );
                                h_no = hist_start;
                                current_block = 11089047473966393316;
                            }
                            8038949400865391589 => {
                                *end = '\0' as i32 as libc::c_char;
                                History_Update_Line(
                                    str,
                                    end.offset_from(str) as libc::c_long as libc::c_int,
                                    h_no,
                                );
                                h_no = hist_end;
                                current_block = 11089047473966393316;
                            }
                            11990075014260372988 => {
                                ::std::mem::transmute::<
                                    _,
                                    fn(_),
                                >(
                                    (Some(pl_le_hook_put_char.unwrap())).unwrap(),
                                )('\n' as i32);
                                Display_Help();
                                current_block = 8610049092612950667;
                                continue;
                            }
                            11254128328725829733 => {
                                if ::std::mem::transmute::<
                                    _,
                                    fn() -> libc::c_int,
                                >((Some(pl_le_hook_kbd_is_not_empty.unwrap())).unwrap())()
                                    != 0
                                    || {
                                        n = Search_Bracket(
                                            b")]}\0" as *const u8 as *const libc::c_char
                                                as *mut libc::c_char,
                                            c as libc::c_char,
                                        );
                                        n < 0 as libc::c_int
                                    }
                                {
                                    current_block = 7172762164747879670;
                                    continue;
                                }
                                count_bracket[2 as libc::c_int as usize] = 0 as libc::c_int;
                                count_bracket[1 as libc::c_int
                                    as usize] = count_bracket[2 as libc::c_int as usize];
                                count_bracket[0 as libc::c_int
                                    as usize] = count_bracket[1 as libc::c_int as usize];
                                count_bracket[n as usize] -= 1;
                                count_bracket[n as usize];
                                p = pos.offset(-(1 as libc::c_int as isize));
                                while count_bracket[n as usize] != 0 as libc::c_int {
                                    p = p.offset(-1);
                                    if p < str {
                                        current_block = 7172762164747879670;
                                        continue '_re_display_line;
                                    }
                                    c = *p as libc::c_int;
                                    n1 = Search_Bracket(
                                        b")]}\0" as *const u8 as *const libc::c_char
                                            as *mut libc::c_char,
                                        c as libc::c_char,
                                    );
                                    if n1 >= 0 as libc::c_int {
                                        count_bracket[n1 as usize] -= 1;
                                        count_bracket[n1 as usize];
                                    } else {
                                        n1 = Search_Bracket(
                                            b"([{\0" as *const u8 as *const libc::c_char
                                                as *mut libc::c_char,
                                            c as libc::c_char,
                                        );
                                        if n1 >= 0 as libc::c_int {
                                            count_bracket[n1 as usize] += 1;
                                            if count_bracket[n1 as usize] > 0 as libc::c_int {
                                                current_block = 7172762164747879670;
                                                continue '_re_display_line;
                                            }
                                        }
                                        if p > str && (c == '\'' as i32 || c == '"' as i32)
                                            && *p.offset(-(1 as libc::c_int) as isize) as libc::c_int
                                                != '\\' as i32
                                        {
                                            loop {
                                                p = p.offset(-1);
                                                if !(p > str
                                                    && (*p as libc::c_int != c
                                                        || *p.offset(-(1 as libc::c_int) as isize) as libc::c_int
                                                            == '\\' as i32))
                                                {
                                                    break;
                                                }
                                            }
                                        }
                                    }
                                }
                                if ::std::mem::transmute::<
                                    _,
                                    fn() -> libc::c_int,
                                >((Some(pl_le_hook_kbd_is_not_empty.unwrap())).unwrap())()
                                    != 0
                                {
                                    current_block = 7172762164747879670;
                                    continue;
                                }
                                n = pos.offset_from(p) as libc::c_long as libc::c_int;
                                q = pos;
                                ::std::mem::transmute::<
                                    _,
                                    fn(_),
                                >((Some(pl_le_hook_backd.unwrap())).unwrap())(n);
                                let mut set: fd_set = fd_set { __fds_bits: [0; 16] };
                                let mut t: timeval = timeval { tv_sec: 0, tv_usec: 0 };
                                t.tv_sec = 0 as libc::c_int as __time_t;
                                t.tv_usec = 900000 as libc::c_int as __suseconds_t;
                                let mut __d0: libc::c_int = 0;
                                let mut __d1: libc::c_int = 0;
                                let fresh11 = &mut __d0;
                                let fresh12;
                                let fresh13 = (::std::mem::size_of::<fd_set>()
                                    as libc::c_ulong)
                                    .wrapping_div(
                                        ::std::mem::size_of::<__fd_mask>() as libc::c_ulong,
                                    );
                                let fresh14 = &mut __d1;
                                let fresh15;
                                let fresh16 = &mut *(set.__fds_bits)
                                    .as_mut_ptr()
                                    .offset(0 as libc::c_int as isize) as *mut __fd_mask;
                                asm!(
                                    "cld; rep; stosq", inlateout("cx")
                                    c2rust_asm_casts::AsmCast::cast_in(fresh11, fresh13) =>
                                    fresh12, inlateout("di")
                                    c2rust_asm_casts::AsmCast::cast_in(fresh14, fresh16) =>
                                    fresh15, inlateout("ax") 0 as libc::c_int => _,
                                    options(preserves_flags, att_syntax)
                                );
                                c2rust_asm_casts::AsmCast::cast_out(
                                    fresh11,
                                    fresh13,
                                    fresh12,
                                );
                                c2rust_asm_casts::AsmCast::cast_out(
                                    fresh14,
                                    fresh16,
                                    fresh15,
                                );
                                set
                                    .__fds_bits[(0 as libc::c_int
                                    / (8 as libc::c_int
                                        * ::std::mem::size_of::<__fd_mask>() as libc::c_ulong
                                            as libc::c_int)) as usize]
                                    |= ((1 as libc::c_ulong)
                                        << 0 as libc::c_int
                                            % (8 as libc::c_int
                                                * ::std::mem::size_of::<__fd_mask>() as libc::c_ulong
                                                    as libc::c_int)) as __fd_mask;
                                select(
                                    1 as libc::c_int,
                                    &mut set,
                                    0 as *mut fd_set,
                                    0 as *mut fd_set,
                                    &mut t,
                                );
                                pos = p;
                                ::std::mem::transmute::<
                                    _,
                                    fn(_, _),
                                >((Some(pl_le_hook_forwd.unwrap())).unwrap())(n, pos);
                                pos = q;
                                current_block = 7172762164747879670;
                                continue;
                            }
                            1622411330066726685 => {
                                str = 0 as *mut libc::c_char;
                                current_block = 16739572250294397020;
                                break;
                            }
                            _ => {}
                        }
                        match current_block {
                            18007440463631195037 => {
                                ::std::mem::transmute::<
                                    _,
                                    fn(),
                                >((Some(pl_le_hook_emit_beep.unwrap())).unwrap())();
                                current_block = 7172762164747879670;
                            }
                            15810102963287113283 => {
                                p = end;
                                end = str.offset(History_Get_Line(str, h_no) as isize);
                                ::std::mem::transmute::<
                                    _,
                                    fn(_, _),
                                >(
                                    (Some(pl_le_hook_displ.unwrap())).unwrap(),
                                )(end.offset_from(pos) as libc::c_long, pos);
                                if end < p {
                                    ::std::mem::transmute::<
                                        _,
                                        fn(_),
                                    >(
                                        (Some(pl_le_hook_erase.unwrap())).unwrap(),
                                    )(p.offset_from(end) as libc::c_long);
                                }
                                ::std::mem::transmute::<
                                    _,
                                    fn(_),
                                >(
                                    (Some(pl_le_hook_backd.unwrap())).unwrap(),
                                )(end.offset_from(pos) as libc::c_long);
                                current_block = 7172762164747879670;
                            }
                            4801495893854419819 => {
                                p = pos;
                                while p < end {
                                    *p.offset(-(1 as libc::c_int) as isize) = *p;
                                    p = p.offset(1);
                                    p;
                                }
                                ::std::mem::transmute::<
                                    _,
                                    fn(_),
                                >(
                                    (Some(pl_le_hook_backd.unwrap())).unwrap(),
                                )(1 as libc::c_int);
                                pos = pos.offset(-1);
                                pos;
                                end = end.offset(-1);
                                end;
                                ::std::mem::transmute::<
                                    _,
                                    fn(_, _),
                                >(
                                    (Some(pl_le_hook_displ.unwrap())).unwrap(),
                                )(end.offset_from(pos) as libc::c_long, pos);
                                ::std::mem::transmute::<
                                    _,
                                    fn(_),
                                >(
                                    (Some(pl_le_hook_erase.unwrap())).unwrap(),
                                )(1 as libc::c_int);
                                ::std::mem::transmute::<
                                    _,
                                    fn(_),
                                >(
                                    (Some(pl_le_hook_backd.unwrap())).unwrap(),
                                )(end.offset_from(pos) as libc::c_long);
                                current_block = 7172762164747879670;
                            }
                            _ => {
                                p = end;
                                end = str.offset(History_Get_Line(str, h_no) as isize);
                                ::std::mem::transmute::<
                                    _,
                                    fn(_),
                                >(
                                    (Some(pl_le_hook_backd.unwrap())).unwrap(),
                                )(pos.offset_from(str) as libc::c_long);
                                ::std::mem::transmute::<
                                    _,
                                    fn(_, _),
                                >(
                                    (Some(pl_le_hook_displ.unwrap())).unwrap(),
                                )(end.offset_from(str) as libc::c_long, str);
                                if end < p {
                                    ::std::mem::transmute::<
                                        _,
                                        fn(_),
                                    >(
                                        (Some(pl_le_hook_erase.unwrap())).unwrap(),
                                    )(p.offset_from(end) as libc::c_long);
                                }
                                pos = end;
                                current_block = 7172762164747879670;
                            }
                        }
                    }
                }
            }
        }
        _ => {}
    }
    match current_block {
        291999910655467784 => {
            ::std::mem::transmute::<
                _,
                fn(_, _),
            >(
                (Some(pl_le_hook_forwd.unwrap())).unwrap(),
            )(end.offset_from(pos) as libc::c_long, pos);
            *end = '\0' as i32 as libc::c_char;
            History_Add_Line(str, end.offset_from(str) as libc::c_long as libc::c_int);
            if (end.offset_from(str) as libc::c_long) < size as libc::c_long {
                let fresh10 = end;
                end = end.offset(1);
                *fresh10 = '\n' as i32 as libc::c_char;
            }
            *end = '\0' as i32 as libc::c_char;
        }
        _ => {}
    }
    ::std::mem::transmute::<
        _,
        fn(_),
    >((Some(pl_le_hook_put_char.unwrap())).unwrap())('\n' as i32);
    Pl_LE_Close_Terminal();
    return str;
}
unsafe extern "C" fn New_Char(
    mut c: libc::c_int,
    mut str: *mut libc::c_char,
    mut size: libc::c_int,
    mut p_pos: *mut *mut libc::c_char,
    mut p_end: *mut *mut libc::c_char,
) -> libc::c_int {
    let mut pos: *mut libc::c_char = *p_pos;
    let mut end: *mut libc::c_char = *p_end;
    let mut p: *mut libc::c_char = 0 as *mut libc::c_char;
    if (ins_mode != 0 || pos == end)
        && end.offset_from(str) as libc::c_long >= size as libc::c_long
    {
        return 0 as libc::c_int;
    }
    if ins_mode == 0 {
        *pos = c as libc::c_char;
        pos = pos.offset(1);
        if pos > end {
            end = pos;
        }
        ::std::mem::transmute::<
            _,
            fn(_),
        >((Some(pl_le_hook_put_char.unwrap())).unwrap())(c);
    } else {
        p = end;
        while p > pos {
            *p = *p.offset(-(1 as libc::c_int) as isize);
            p = p.offset(-1);
            p;
        }
        *pos = c as libc::c_char;
        end = end.offset(1);
        end;
        ::std::mem::transmute::<
            _,
            fn(_, _),
        >(
            (Some(pl_le_hook_displ.unwrap())).unwrap(),
        )(end.offset_from(pos) as libc::c_long, pos);
        pos = pos.offset(1);
        pos;
        ::std::mem::transmute::<
            _,
            fn(_),
        >(
            (Some(pl_le_hook_backd.unwrap())).unwrap(),
        )(end.offset_from(pos) as libc::c_long);
    }
    *p_pos = pos;
    *p_end = end;
    return 1 as libc::c_int;
}
pub unsafe extern "C" fn Pl_LE_Get_Prompt_Length() -> libc::c_int {
    return prompt_length;
}
pub unsafe extern "C" fn Pl_LE_Get_Current_Position() -> libc::c_int {
    return global_pos.offset_from(global_str) as libc::c_long as libc::c_int;
}
pub unsafe extern "C" fn Pl_LE_Get_Current_Word(mut word: *mut libc::c_char) {
    let mut str: *mut libc::c_char = global_str;
    let mut pos: *mut libc::c_char = global_pos;
    let mut end: *mut libc::c_char = global_end;
    let mut p: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut q: *mut libc::c_char = 0 as *mut libc::c_char;
    p = Skip(pos, str, 0 as libc::c_int, -(1 as libc::c_int));
    if Is_A_Separator(*p) != 0 {
        p = p.offset(1);
        p;
    }
    q = Skip(pos, end, 0 as libc::c_int, 1 as libc::c_int);
    while p < q {
        let fresh17 = p;
        p = p.offset(1);
        let fresh18 = word;
        word = word.offset(1);
        *fresh18 = *fresh17;
    }
    *word = '\0' as i32 as libc::c_char;
}
pub unsafe extern "C" fn Pl_LE_Get_Separators() -> *mut libc::c_char {
    return separators.as_mut_ptr();
}
pub unsafe extern "C" fn Pl_LE_Set_Separators(
    mut sep_str: *mut libc::c_char,
) -> *mut libc::c_char {
    return strcpy(separators.as_mut_ptr(), sep_str);
}
pub unsafe extern "C" fn Pl_LE_Get_Ctrl_C_Return_Value() -> PlLong {
    return ctrl_c_ret_val;
}
unsafe extern "C" fn Skip(
    mut from: *mut libc::c_char,
    mut limit: *mut libc::c_char,
    mut res_sep_cmp: libc::c_int,
    mut direction: libc::c_int,
) -> *mut libc::c_char {
    while from != limit {
        if Is_A_Separator(*from) != res_sep_cmp {
            break;
        }
        from = from.offset(direction as isize);
    }
    return from;
}
unsafe extern "C" fn Is_A_Separator(mut c: libc::c_char) -> libc::c_int {
    let mut p: *mut libc::c_char = 0 as *mut libc::c_char;
    p = separators.as_mut_ptr();
    while *p != 0 {
        if *p as libc::c_int == c as libc::c_int {
            return 1 as libc::c_int;
        }
        p = p.offset(1);
        p;
    }
    return 0 as libc::c_int;
}
unsafe extern "C" fn Search_Bracket(
    mut brackets: *mut libc::c_char,
    mut c: libc::c_char,
) -> libc::c_int {
    let mut n: libc::c_int = 0;
    n = 0 as libc::c_int;
    while *brackets.offset(n as isize) as libc::c_int != '\0' as i32 {
        if *brackets.offset(n as isize) as libc::c_int == c as libc::c_int {
            return n;
        }
        n += 1;
        n;
    }
    return -(1 as libc::c_int);
}
unsafe extern "C" fn Tab_To_Spaces(mut p: libc::c_int) -> libc::c_int {
    p += prompt_length;
    p = 8 as libc::c_int - p % 8 as libc::c_int;
    return p;
}
unsafe extern "C" fn History_Add_Line(
    mut line: *mut libc::c_char,
    mut length: libc::c_int,
) {
    let mut p: *mut libc::c_char = line;
    while *p as libc::c_int == ' ' as i32 {
        p = p.offset(1);
        p;
    }
    if *p as libc::c_int == '\0' as i32 {
        return;
    }
    if hist_end > 0 as libc::c_int
        && strcmp(line, hist_tbl[(hist_end - 1 as libc::c_int) as usize].line)
            == 0 as libc::c_int
    {
        return;
    }
    History_Update_Line(line, length, hist_end);
    hist_end += 1;
    if hist_end >= 64 as libc::c_int {
        hist_end = 0 as libc::c_int;
    }
    if hist_end == hist_start {
        hist_start += 1;
        if hist_start >= 64 as libc::c_int {
            hist_start = 0 as libc::c_int;
        }
    }
}
unsafe extern "C" fn History_Update_Line(
    mut line: *mut libc::c_char,
    mut length: libc::c_int,
    mut hist_no: libc::c_int,
) {
    let mut h: *mut HistCell = 0 as *mut HistCell;
    h = hist_tbl.as_mut_ptr().offset(hist_no as isize);
    if !((*h).line).is_null() && (*h).buff_length < length {
        free((*h).line as *mut libc::c_void);
        (*h).line = 0 as *mut libc::c_char;
    }
    if ((*h).line).is_null() {
        (*h)
            .line = malloc((length + 1 as libc::c_int) as libc::c_ulong)
            as *mut libc::c_char;
        if ((*h).line).is_null() {
            exit(1 as libc::c_int);
        }
        (*h).buff_length = length;
    }
    strcpy((*h).line, line);
    (*h).line_length = length;
}
unsafe extern "C" fn History_Get_Line(
    mut str: *mut libc::c_char,
    mut hist_no: libc::c_int,
) -> libc::c_int {
    let mut h: *mut HistCell = hist_tbl.as_mut_ptr().offset(hist_no as isize);
    strcpy(str, (*h).line);
    return (*h).line_length;
}
pub unsafe extern "C" fn Pl_LE_Compl_Add_Word(
    mut word: *mut libc::c_char,
    mut word_length: libc::c_int,
) -> *mut libc::c_char {
    let mut p: *mut *mut CompNode = 0 as *mut *mut CompNode;
    let mut q: *mut CompNode = 0 as *mut CompNode;
    let mut cmp: libc::c_int = 0;
    p = &mut comp_start;
    while !(*p).is_null() {
        cmp = strcmp((**p).word, word);
        if cmp == 0 as libc::c_int {
            return word;
        }
        if cmp > 0 as libc::c_int {
            break;
        }
        p = &mut (**p).next;
    }
    q = malloc(::std::mem::size_of::<CompNode>() as libc::c_ulong) as *mut CompNode;
    if q.is_null() {
        exit(1 as libc::c_int);
    }
    (*q).word = word;
    (*q).word_length = word_length;
    (*q).next = *p;
    *p = q;
    return word;
}
pub unsafe extern "C" fn Pl_LE_Compl_Del_Word(
    mut word: *mut libc::c_char,
) -> *mut libc::c_char {
    let mut p: *mut *mut CompNode = 0 as *mut *mut CompNode;
    let mut q: *mut CompNode = 0 as *mut CompNode;
    let mut cmp: libc::c_int = 0;
    p = &mut comp_start;
    while !(*p).is_null() {
        cmp = strcmp((**p).word, word);
        if cmp == 0 as libc::c_int {
            break;
        }
        if cmp > 0 as libc::c_int {
            return 0 as *mut libc::c_char;
        }
        p = &mut (**p).next;
    }
    q = *p;
    *p = (*q).next;
    free(q as *mut libc::c_void);
    return word;
}
pub unsafe extern "C" fn Pl_LE_Compl_Init_Match(
    mut prefix: *mut libc::c_char,
    mut nb_match: *mut libc::c_int,
    mut max_lg: *mut libc::c_int,
) -> *mut libc::c_char {
    let mut prefix_length: libc::c_int = 0;
    let mut rest_length: libc::c_int = 0;
    let mut str: *mut libc::c_char = 0 as *mut libc::c_char;
    prefix_length = strlen(prefix) as libc::c_int;
    if (Completion_Do_Match(prefix, prefix_length, &mut rest_length)).is_null() {
        return 0 as *mut libc::c_char;
    }
    str = malloc((prefix_length + rest_length + 1 as libc::c_int) as libc::c_ulong)
        as *mut libc::c_char;
    if str.is_null() {
        exit(1 as libc::c_int);
    }
    *nb_match = comp_nb_match;
    *max_lg = comp_match_max_lg;
    comp_cur_match = comp_first_match;
    strncpy(
        str,
        (*comp_first_match).word,
        (prefix_length + rest_length) as libc::c_ulong,
    );
    *str.offset((prefix_length + rest_length) as isize) = '\0' as i32 as libc::c_char;
    return str;
}
pub unsafe extern "C" fn Pl_LE_Compl_Find_Match(
    mut is_last: *mut libc::c_int,
) -> *mut libc::c_char {
    let mut str: *mut libc::c_char = 0 as *mut libc::c_char;
    if comp_cur_match.is_null() {
        return 0 as *mut libc::c_char;
    }
    str = (*comp_cur_match).word;
    if comp_cur_match != comp_last_match {
        comp_cur_match = (*comp_cur_match).next;
        *is_last = 0 as libc::c_int;
    } else {
        comp_cur_match = 0 as *mut CompNode;
        *is_last = 1 as libc::c_int;
    }
    return str;
}
unsafe extern "C" fn Completion_Do_Match(
    mut prefix: *mut libc::c_char,
    mut prefix_length: libc::c_int,
    mut rest_length: *mut libc::c_int,
) -> *mut libc::c_char {
    let mut p: *mut CompNode = 0 as *mut CompNode;
    let mut cmp: libc::c_int = 0;
    let mut l: libc::c_int = 0;
    let mut w: libc::c_char = 0;
    comp_first_match = 0 as *mut CompNode;
    comp_nb_match = 0 as libc::c_int;
    comp_match_max_lg = 0 as libc::c_int;
    p = comp_start;
    while !p.is_null() {
        cmp = strncmp((*p).word, prefix, prefix_length as libc::c_ulong);
        if cmp == 0 as libc::c_int {
            if comp_first_match.is_null() {
                comp_first_match = p;
            }
            comp_last_match = p;
            comp_nb_match += 1;
            comp_nb_match;
            if (*p).word_length > comp_match_max_lg {
                comp_match_max_lg = (*p).word_length;
            }
        } else if cmp > 0 as libc::c_int {
            break;
        }
        p = (*p).next;
    }
    if comp_first_match.is_null() {
        return 0 as *mut libc::c_char;
    }
    if comp_first_match == comp_last_match {
        *rest_length = (*comp_first_match).word_length - prefix_length;
    } else {
        l = prefix_length;
        's_93: loop {
            w = *((*comp_first_match).word).offset(l as isize);
            p = (*comp_first_match).next;
            loop {
                if *((*p).word).offset(l as isize) as libc::c_int != w as libc::c_int {
                    break 's_93;
                }
                if p == comp_last_match {
                    break;
                }
                p = (*p).next;
            }
            l += 1;
            l;
        }
        *rest_length = l - prefix_length;
    }
    return ((*comp_first_match).word).offset(prefix_length as isize);
}
unsafe extern "C" fn Completion_Print_All() {
    let mut p: *mut CompNode = 0 as *mut CompNode;
    let mut p1: *mut CompNode = 0 as *mut CompNode;
    let mut row: libc::c_int = 0;
    let mut col: libc::c_int = 0;
    let mut nb_in_a_line: libc::c_int = 0;
    let mut nb_lines: libc::c_int = 0;
    let mut nb_in_last_line: libc::c_int = 0;
    let mut nb_miss_in_last_line: libc::c_int = 0;
    let mut spaces: libc::c_int = 0;
    let mut skip: libc::c_int = 0;
    let mut k: libc::c_int = 0;
    let mut buff: [libc::c_char; 512] = [0; 512];
    let mut l: libc::c_int = 0;
    let mut c: libc::c_int = 0;
    ::std::mem::transmute::<
        _,
        fn(_, _),
    >((Some(pl_le_hook_screen_size.unwrap())).unwrap())(&mut row, &mut col);
    nb_in_a_line = col / (comp_match_max_lg + 2 as libc::c_int);
    if nb_in_a_line <= 1 as libc::c_int {
        nb_in_a_line = 1 as libc::c_int;
    }
    nb_lines = (comp_nb_match + nb_in_a_line - 1 as libc::c_int) / nb_in_a_line;
    nb_in_last_line = (comp_nb_match - 1 as libc::c_int) % nb_in_a_line
        + 1 as libc::c_int;
    nb_miss_in_last_line = nb_in_a_line - nb_in_last_line;
    spaces = if nb_in_a_line == 1 as libc::c_int {
        0 as libc::c_int
    } else {
        (col - nb_in_a_line * comp_match_max_lg) / nb_in_a_line
    };
    if nb_lines > 20 as libc::c_int {
        sprintf(
            buff.as_mut_ptr(),
            b"Show all %d possibilities (y/n) ? \0" as *const u8 as *const libc::c_char,
            comp_nb_match,
        );
        ::std::mem::transmute::<
            _,
            fn(_),
        >((Some(pl_le_hook_displ_str.unwrap())).unwrap())(buff.as_mut_ptr());
        c = Pl_LE_Get_Char();
        ::std::mem::transmute::<
            _,
            fn(_),
        >((Some(pl_le_hook_put_char.unwrap())).unwrap())('\n' as i32);
        if c != 'y' as i32 {
            return;
        }
    }
    p = comp_first_match;
    l = 0 as libc::c_int;
    loop {
        p1 = p;
        c = 0 as libc::c_int;
        loop {
            ::std::mem::transmute::<
                _,
                fn(_),
            >((Some(pl_le_hook_displ_str.unwrap())).unwrap())((*p1).word);
            c += 1;
            if c
                == (if l < nb_lines - 1 as libc::c_int {
                    nb_in_a_line
                } else {
                    nb_in_last_line
                })
            {
                break;
            }
            sprintf(
                buff.as_mut_ptr(),
                b"%*s\0" as *const u8 as *const libc::c_char,
                comp_match_max_lg - (*p1).word_length + spaces,
                b"\0" as *const u8 as *const libc::c_char,
            );
            ::std::mem::transmute::<
                _,
                fn(_),
            >((Some(pl_le_hook_displ_str.unwrap())).unwrap())(buff.as_mut_ptr());
            skip = nb_lines;
            if c > nb_in_a_line - nb_miss_in_last_line {
                skip -= 1;
                skip;
            }
            k = 0 as libc::c_int;
            while k < skip {
                p1 = (*p1).next;
                k += 1;
                k;
            }
        }
        ::std::mem::transmute::<
            _,
            fn(_),
        >((Some(pl_le_hook_put_char.unwrap())).unwrap())('\n' as i32);
        l += 1;
        if l == nb_lines {
            break;
        }
        p = (*p).next;
    };
}
unsafe extern "C" fn Display_Help() {
    let mut buff: [libc::c_char; 80] = [0; 80];
    ::std::mem::transmute::<
        _,
        fn(_),
    >(
        (Some(pl_le_hook_displ_str.unwrap())).unwrap(),
    )(b"\0" as *const u8 as *const libc::c_char);
    ::std::mem::transmute::<
        _,
        fn(_),
    >((Some(pl_le_hook_put_char.unwrap())).unwrap())('\n' as i32);
    sprintf(
        buff.as_mut_ptr(),
        b"   linedit %-25s Copyright (C) 1999-2021 Daniel Diaz\0" as *const u8
            as *const libc::c_char,
        b"2.6\0" as *const u8 as *const libc::c_char,
    );
    ::std::mem::transmute::<
        _,
        fn(_),
    >((Some(pl_le_hook_displ_str.unwrap())).unwrap())(buff.as_mut_ptr());
    ::std::mem::transmute::<
        _,
        fn(_),
    >((Some(pl_le_hook_put_char.unwrap())).unwrap())('\n' as i32);
    ::std::mem::transmute::<
        _,
        fn(_),
    >(
        (Some(pl_le_hook_displ_str.unwrap())).unwrap(),
    )(b"\0" as *const u8 as *const libc::c_char);
    ::std::mem::transmute::<
        _,
        fn(_),
    >((Some(pl_le_hook_put_char.unwrap())).unwrap())('\n' as i32);
    ::std::mem::transmute::<
        _,
        fn(_),
    >(
        (Some(pl_le_hook_displ_str.unwrap())).unwrap(),
    )(b"                              Moving\0" as *const u8 as *const libc::c_char);
    ::std::mem::transmute::<
        _,
        fn(_),
    >((Some(pl_le_hook_put_char.unwrap())).unwrap())('\n' as i32);
    ::std::mem::transmute::<
        _,
        fn(_),
    >(
        (Some(pl_le_hook_displ_str.unwrap())).unwrap(),
    )(
        b"   Ctl-B   previous char             Ctl-F   next char\0" as *const u8
            as *const libc::c_char,
    );
    ::std::mem::transmute::<
        _,
        fn(_),
    >((Some(pl_le_hook_put_char.unwrap())).unwrap())('\n' as i32);
    ::std::mem::transmute::<
        _,
        fn(_),
    >(
        (Some(pl_le_hook_displ_str.unwrap())).unwrap(),
    )(
        b"   Esc-B   previous word             Esc-F   next word\0" as *const u8
            as *const libc::c_char,
    );
    ::std::mem::transmute::<
        _,
        fn(_),
    >((Some(pl_le_hook_put_char.unwrap())).unwrap())('\n' as i32);
    ::std::mem::transmute::<
        _,
        fn(_),
    >(
        (Some(pl_le_hook_displ_str.unwrap())).unwrap(),
    )(
        b"   Ctl-A   begin of line             Ctl-E   end of line\0" as *const u8
            as *const libc::c_char,
    );
    ::std::mem::transmute::<
        _,
        fn(_),
    >((Some(pl_le_hook_put_char.unwrap())).unwrap())('\n' as i32);
    ::std::mem::transmute::<
        _,
        fn(_),
    >(
        (Some(pl_le_hook_displ_str.unwrap())).unwrap(),
    )(b"\0" as *const u8 as *const libc::c_char);
    ::std::mem::transmute::<
        _,
        fn(_),
    >((Some(pl_le_hook_put_char.unwrap())).unwrap())('\n' as i32);
    ::std::mem::transmute::<
        _,
        fn(_),
    >(
        (Some(pl_le_hook_displ_str.unwrap())).unwrap(),
    )(b"                             Deleting\0" as *const u8 as *const libc::c_char);
    ::std::mem::transmute::<
        _,
        fn(_),
    >((Some(pl_le_hook_put_char.unwrap())).unwrap())('\n' as i32);
    ::std::mem::transmute::<
        _,
        fn(_),
    >(
        (Some(pl_le_hook_displ_str.unwrap())).unwrap(),
    )(
        b"   Ctl-U   delete begin of line      Ctl-K   delete end of line\0" as *const u8
            as *const libc::c_char,
    );
    ::std::mem::transmute::<
        _,
        fn(_),
    >((Some(pl_le_hook_put_char.unwrap())).unwrap())('\n' as i32);
    ::std::mem::transmute::<
        _,
        fn(_),
    >(
        (Some(pl_le_hook_displ_str.unwrap())).unwrap(),
    )(
        b"   Ctl-H   delete previous char      Ctl-D   delete current char\0"
            as *const u8 as *const libc::c_char,
    );
    ::std::mem::transmute::<
        _,
        fn(_),
    >((Some(pl_le_hook_put_char.unwrap())).unwrap())('\n' as i32);
    ::std::mem::transmute::<
        _,
        fn(_),
    >(
        (Some(pl_le_hook_displ_str.unwrap())).unwrap(),
    )(b"\0" as *const u8 as *const libc::c_char);
    ::std::mem::transmute::<
        _,
        fn(_),
    >((Some(pl_le_hook_put_char.unwrap())).unwrap())('\n' as i32);
    ::std::mem::transmute::<
        _,
        fn(_),
    >(
        (Some(pl_le_hook_displ_str.unwrap())).unwrap(),
    )(b"                             Changing\0" as *const u8 as *const libc::c_char);
    ::std::mem::transmute::<
        _,
        fn(_),
    >((Some(pl_le_hook_put_char.unwrap())).unwrap())('\n' as i32);
    ::std::mem::transmute::<
        _,
        fn(_),
    >(
        (Some(pl_le_hook_displ_str.unwrap())).unwrap(),
    )(
        b"   Esc-L   downcase word             Esc-U   upcase word\0" as *const u8
            as *const libc::c_char,
    );
    ::std::mem::transmute::<
        _,
        fn(_),
    >((Some(pl_le_hook_put_char.unwrap())).unwrap())('\n' as i32);
    ::std::mem::transmute::<
        _,
        fn(_),
    >(
        (Some(pl_le_hook_displ_str.unwrap())).unwrap(),
    )(
        b"   Esc-C   capitalize word           Ctl-T   reverse last two chars\0"
            as *const u8 as *const libc::c_char,
    );
    ::std::mem::transmute::<
        _,
        fn(_),
    >((Some(pl_le_hook_put_char.unwrap())).unwrap())('\n' as i32);
    ::std::mem::transmute::<
        _,
        fn(_),
    >(
        (Some(pl_le_hook_displ_str.unwrap())).unwrap(),
    )(b"\0" as *const u8 as *const libc::c_char);
    ::std::mem::transmute::<
        _,
        fn(_),
    >((Some(pl_le_hook_put_char.unwrap())).unwrap())('\n' as i32);
    ::std::mem::transmute::<
        _,
        fn(_),
    >(
        (Some(pl_le_hook_displ_str.unwrap())).unwrap(),
    )(b"                             History\0" as *const u8 as *const libc::c_char);
    ::std::mem::transmute::<
        _,
        fn(_),
    >((Some(pl_le_hook_put_char.unwrap())).unwrap())('\n' as i32);
    ::std::mem::transmute::<
        _,
        fn(_),
    >(
        (Some(pl_le_hook_displ_str.unwrap())).unwrap(),
    )(
        b"   Esc-<   first line                Esc->   last line\0" as *const u8
            as *const libc::c_char,
    );
    ::std::mem::transmute::<
        _,
        fn(_),
    >((Some(pl_le_hook_put_char.unwrap())).unwrap())('\n' as i32);
    ::std::mem::transmute::<
        _,
        fn(_),
    >(
        (Some(pl_le_hook_displ_str.unwrap())).unwrap(),
    )(
        b"   Ctl-P   previous line             Ctl-N   next line\0" as *const u8
            as *const libc::c_char,
    );
    ::std::mem::transmute::<
        _,
        fn(_),
    >((Some(pl_le_hook_put_char.unwrap())).unwrap())('\n' as i32);
    ::std::mem::transmute::<
        _,
        fn(_),
    >(
        (Some(pl_le_hook_displ_str.unwrap())).unwrap(),
    )(
        b"   Esc-P   previous matching line    Esc-N   next matching line\0" as *const u8
            as *const libc::c_char,
    );
    ::std::mem::transmute::<
        _,
        fn(_),
    >((Some(pl_le_hook_put_char.unwrap())).unwrap())('\n' as i32);
    ::std::mem::transmute::<
        _,
        fn(_),
    >(
        (Some(pl_le_hook_displ_str.unwrap())).unwrap(),
    )(b"\0" as *const u8 as *const libc::c_char);
    ::std::mem::transmute::<
        _,
        fn(_),
    >((Some(pl_le_hook_put_char.unwrap())).unwrap())('\n' as i32);
    ::std::mem::transmute::<
        _,
        fn(_),
    >(
        (Some(pl_le_hook_displ_str.unwrap())).unwrap(),
    )(b"                             Selection\0" as *const u8 as *const libc::c_char);
    ::std::mem::transmute::<
        _,
        fn(_),
    >((Some(pl_le_hook_put_char.unwrap())).unwrap())('\n' as i32);
    ::std::mem::transmute::<
        _,
        fn(_),
    >(
        (Some(pl_le_hook_displ_str.unwrap())).unwrap(),
    )(
        b"   Ctl-spc mark selection            Ctl-W   cut  selection\0" as *const u8
            as *const libc::c_char,
    );
    ::std::mem::transmute::<
        _,
        fn(_),
    >((Some(pl_le_hook_put_char.unwrap())).unwrap())('\n' as i32);
    ::std::mem::transmute::<
        _,
        fn(_),
    >(
        (Some(pl_le_hook_displ_str.unwrap())).unwrap(),
    )(
        b"   Esc-W   copy selection            Ctl-Y   past selection\0" as *const u8
            as *const libc::c_char,
    );
    ::std::mem::transmute::<
        _,
        fn(_),
    >((Some(pl_le_hook_put_char.unwrap())).unwrap())('\n' as i32);
    ::std::mem::transmute::<
        _,
        fn(_),
    >(
        (Some(pl_le_hook_displ_str.unwrap())).unwrap(),
    )(b"\0" as *const u8 as *const libc::c_char);
    ::std::mem::transmute::<
        _,
        fn(_),
    >((Some(pl_le_hook_put_char.unwrap())).unwrap())('\n' as i32);
    ::std::mem::transmute::<
        _,
        fn(_),
    >(
        (Some(pl_le_hook_displ_str.unwrap())).unwrap(),
    )(b"                           Miscellaneous\0" as *const u8 as *const libc::c_char);
    ::std::mem::transmute::<
        _,
        fn(_),
    >((Some(pl_le_hook_put_char.unwrap())).unwrap())('\n' as i32);
    ::std::mem::transmute::<
        _,
        fn(_),
    >(
        (Some(pl_le_hook_displ_str.unwrap())).unwrap(),
    )(
        b"   Ctl-V   insert mode switch        Ctl-I   completion (twice = all)\0"
            as *const u8 as *const libc::c_char,
    );
    ::std::mem::transmute::<
        _,
        fn(_),
    >((Some(pl_le_hook_put_char.unwrap())).unwrap())('\n' as i32);
    ::std::mem::transmute::<
        _,
        fn(_),
    >(
        (Some(pl_le_hook_displ_str.unwrap())).unwrap(),
    )(
        b"   Esc-?   display this help       Esc-Ctl-I insert spaces for tab\0"
            as *const u8 as *const libc::c_char,
    );
    ::std::mem::transmute::<
        _,
        fn(_),
    >((Some(pl_le_hook_put_char.unwrap())).unwrap())('\n' as i32);
    ::std::mem::transmute::<
        _,
        fn(_),
    >(
        (Some(pl_le_hook_displ_str.unwrap())).unwrap(),
    )(b"\0" as *const u8 as *const libc::c_char);
    ::std::mem::transmute::<
        _,
        fn(_),
    >((Some(pl_le_hook_put_char.unwrap())).unwrap())('\n' as i32);
}
pub unsafe extern "C" fn Pl_LE_Get_Key(
    mut echo: libc::c_int,
    mut catch_ctrl_c: libc::c_int,
) -> libc::c_int {
    let mut c: libc::c_int = 0;
    Pl_LE_Initialize();
    prompt_length = 0 as libc::c_int;
    loop {
        Pl_LE_Open_Terminal();
        c = Pl_LE_Get_Char();
        if !(catch_ctrl_c != 0 && Pl_LE_Is_Interrupt_Key(c) != 0) {
            break;
        }
        Pl_LE_Close_Terminal();
        ctrl_c_ret_val = Pl_Emit_Ctrl_C();
        if ctrl_c_ret_val != 0 as libc::c_int as libc::c_long {
            return -(2 as libc::c_int);
        }
    }
    if c == 'D' as i32 & 0x1f as libc::c_int {
        c = -(1 as libc::c_int);
    }
    if echo != 0 && c as libc::c_uint <= 255 as libc::c_int as libc::c_uint
        && *(*__ctype_b_loc()).offset(c as isize) as libc::c_int
            & _ISprint as libc::c_int as libc::c_ushort as libc::c_int != 0
    {
        ::std::mem::transmute::<
            _,
            fn(_),
        >((Some(pl_le_hook_put_char.unwrap())).unwrap())(c);
    }
    Pl_LE_Close_Terminal();
    return c;
}
pub unsafe extern "C" fn Pl_LE_Printf(
    mut format: *mut libc::c_char,
    mut args: ...
) -> libc::c_int {
    let mut arg_ptr: ::std::ffi::VaListImpl;
    static mut buff: [libc::c_char; 65535] = [0; 65535];
    let mut ret: libc::c_int = 0;
    Pl_LE_Initialize();
    arg_ptr = args.clone();
    ret = vsprintf(buff.as_mut_ptr(), format, arg_ptr.as_va_list());
    ::std::mem::transmute::<
        _,
        fn(_),
    >((Some(pl_le_hook_displ_str.unwrap())).unwrap())(buff.as_mut_ptr());
    return ret;
}
