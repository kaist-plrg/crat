use ::libc;
use ::c2rust_bitfields;
use ::c2rust_asm_casts;
use c2rust_asm_casts::AsmCastTrait;
use std::arch::asm;
extern "C" {
    pub type _IO_wide_data;
    pub type _IO_codecvt;
    pub type _IO_marker;
    pub type re_dfa_t;
    pub type dfa;
    pub type break_point;
    pub type sockaddr_x25;
    pub type sockaddr_un;
    pub type sockaddr_ns;
    pub type sockaddr_iso;
    pub type sockaddr_ipx;
    pub type sockaddr_inarp;
    pub type sockaddr_eon;
    pub type sockaddr_dl;
    pub type sockaddr_ax25;
    pub type sockaddr_at;
    static mut stdin: *mut FILE;
    static mut stdout: *mut FILE;
    static mut stderr: *mut FILE;
    fn fclose(__stream: *mut FILE) -> libc::c_int;
    fn fflush(__stream: *mut FILE) -> libc::c_int;
    fn fdopen(__fd: libc::c_int, __modes: *const libc::c_char) -> *mut FILE;
    fn sprintf(_: *mut libc::c_char, _: *const libc::c_char, _: ...) -> libc::c_int;
    fn __overflow(_: *mut FILE, _: libc::c_int) -> libc::c_int;
    fn fwrite_unlocked(
        __ptr: *const libc::c_void,
        __size: size_t,
        __n: size_t,
        __stream: *mut FILE,
    ) -> size_t;
    fn ferror(__stream: *mut FILE) -> libc::c_int;
    fn fileno(__stream: *mut FILE) -> libc::c_int;
    fn popen(__command: *const libc::c_char, __modes: *const libc::c_char) -> *mut FILE;
    fn pclose(__stream: *mut FILE) -> libc::c_int;
    fn dcgettext(
        __domainname: *const libc::c_char,
        __msgid: *const libc::c_char,
        __category: libc::c_int,
    ) -> *mut libc::c_char;
    fn memcpy(
        _: *mut libc::c_void,
        _: *const libc::c_void,
        _: libc::c_ulong,
    ) -> *mut libc::c_void;
    fn memmove(
        _: *mut libc::c_void,
        _: *const libc::c_void,
        _: libc::c_ulong,
    ) -> *mut libc::c_void;
    fn memset(
        _: *mut libc::c_void,
        _: libc::c_int,
        _: libc::c_ulong,
    ) -> *mut libc::c_void;
    fn memcmp(
        _: *const libc::c_void,
        _: *const libc::c_void,
        _: libc::c_ulong,
    ) -> libc::c_int;
    fn strcpy(_: *mut libc::c_char, _: *const libc::c_char) -> *mut libc::c_char;
    fn strcat(_: *mut libc::c_char, _: *const libc::c_char) -> *mut libc::c_char;
    fn strcmp(_: *const libc::c_char, _: *const libc::c_char) -> libc::c_int;
    fn strncmp(
        _: *const libc::c_char,
        _: *const libc::c_char,
        _: libc::c_ulong,
    ) -> libc::c_int;
    fn strchr(_: *const libc::c_char, _: libc::c_int) -> *mut libc::c_char;
    fn strlen(_: *const libc::c_char) -> libc::c_ulong;
    fn strerror(_: libc::c_int) -> *mut libc::c_char;
    fn strcasecmp(_: *const libc::c_char, _: *const libc::c_char) -> libc::c_int;
    fn signal(__sig: libc::c_int, __handler: __sighandler_t) -> __sighandler_t;
    fn kill(__pid: __pid_t, __sig: libc::c_int) -> libc::c_int;
    fn sigemptyset(__set: *mut sigset_t) -> libc::c_int;
    fn sigaddset(__set: *mut sigset_t, __signo: libc::c_int) -> libc::c_int;
    fn sigprocmask(
        __how: libc::c_int,
        __set: *const sigset_t,
        __oset: *mut sigset_t,
    ) -> libc::c_int;
    fn __errno_location() -> *mut libc::c_int;
    fn strtol(
        _: *const libc::c_char,
        _: *mut *mut libc::c_char,
        _: libc::c_int,
    ) -> libc::c_long;
    fn strtoul(
        _: *const libc::c_char,
        _: *mut *mut libc::c_char,
        _: libc::c_int,
    ) -> libc::c_ulong;
    fn select(
        __nfds: libc::c_int,
        __readfds: *mut fd_set,
        __writefds: *mut fd_set,
        __exceptfds: *mut fd_set,
        __timeout: *mut timeval,
    ) -> libc::c_int;
    fn malloc(_: libc::c_ulong) -> *mut libc::c_void;
    fn calloc(_: libc::c_ulong, _: libc::c_ulong) -> *mut libc::c_void;
    fn realloc(_: *mut libc::c_void, _: libc::c_ulong) -> *mut libc::c_void;
    fn free(__ptr: *mut libc::c_void);
    fn getenv(__name: *const libc::c_char) -> *mut libc::c_char;
    fn posix_openpt(__oflag: libc::c_int) -> libc::c_int;
    fn grantpt(__fd: libc::c_int) -> libc::c_int;
    fn unlockpt(__fd: libc::c_int) -> libc::c_int;
    fn ptsname(__fd: libc::c_int) -> *mut libc::c_char;
    fn mbrtowc(
        __pwc: *mut wchar_t,
        __s: *const libc::c_char,
        __n: size_t,
        __p: *mut mbstate_t,
    ) -> size_t;
    fn __mbrlen(__s: *const libc::c_char, __n: size_t, __ps: *mut mbstate_t) -> size_t;
    fn fcntl(__fd: libc::c_int, __cmd: libc::c_int, _: ...) -> libc::c_int;
    fn open(__file: *const libc::c_char, __oflag: libc::c_int, _: ...) -> libc::c_int;
    fn __xstat(
        __ver: libc::c_int,
        __filename: *const libc::c_char,
        __stat_buf: *mut stat,
    ) -> libc::c_int;
    fn __fxstat(
        __ver: libc::c_int,
        __fildes: libc::c_int,
        __stat_buf: *mut stat,
    ) -> libc::c_int;
    fn __lxstat(
        __ver: libc::c_int,
        __filename: *const libc::c_char,
        __stat_buf: *mut stat,
    ) -> libc::c_int;
    fn access(__name: *const libc::c_char, __type: libc::c_int) -> libc::c_int;
    fn close(__fd: libc::c_int) -> libc::c_int;
    fn read(__fd: libc::c_int, __buf: *mut libc::c_void, __nbytes: size_t) -> ssize_t;
    fn pipe(__pipedes: *mut libc::c_int) -> libc::c_int;
    fn usleep(__useconds: __useconds_t) -> libc::c_int;
    fn dup(__fd: libc::c_int) -> libc::c_int;
    fn dup2(__fd: libc::c_int, __fd2: libc::c_int) -> libc::c_int;
    fn execl(
        __path: *const libc::c_char,
        __arg: *const libc::c_char,
        _: ...
    ) -> libc::c_int;
    fn _exit(_: libc::c_int) -> !;
    fn getpid() -> __pid_t;
    fn setsid() -> __pid_t;
    fn fork() -> __pid_t;
    static mut NR: libc::c_long;
    static mut FNR: libc::c_long;
    static mut BINMODE: libc::c_int;
    static mut IGNORECASE: bool;
    static mut CONVFMT: *const libc::c_char;
    static mut CONVFMTidx: libc::c_int;
    static mut FILENAME_node: *mut NODE;
    static mut FNR_node: *mut NODE;
    static mut NR_node: *mut NODE;
    static mut RS_node: *mut NODE;
    static mut RT_node: *mut NODE;
    static mut SUBSEP_node: *mut NODE;
    static mut PROCINFO_node: *mut NODE;
    static mut Nnull_string: *mut NODE;
    static mut fields_arr: *mut *mut NODE;
    static mut make_number: Option::<unsafe extern "C" fn(libc::c_double) -> *mut NODE>;
    static mut str2number: Option::<unsafe extern "C" fn(*mut NODE) -> *mut NODE>;
    static mut format_val: Option::<
        unsafe extern "C" fn(*const libc::c_char, libc::c_int, *mut NODE) -> *mut NODE,
    >;
    static mut do_flags: do_flag_values;
    static mut gawk_mb_cur_max: libc::c_int;
    static mut defpath: *const libc::c_char;
    static mut deflibpath: *const libc::c_char;
    static envsep: libc::c_char;
    static mut stack_ptr: *mut STACK_ITEM;
    fn r_unref(tmp: *mut NODE);
    fn array_vname(symbol: *const NODE) -> *const libc::c_char;
    fn efflush(fp: *mut FILE, from: *const libc::c_char, rp: *mut redirect);
    fn sanitize_exit_status(status: libc::c_int) -> libc::c_int;
    fn update_ERRNO_int(_: libc::c_int);
    fn update_ERRNO_string(string: *const libc::c_char);
    fn genflags2str(flagval: libc::c_int, tab: *const flagtab) -> *const libc::c_char;
    fn elem_new_to_scalar(n: *mut NODE) -> *mut NODE;
    fn set_record(
        buf: *const libc::c_char,
        cnt: size_t,
        _: *const awk_fieldwidth_info_t,
    );
    fn set_FS();
    fn current_field_sep() -> field_sep_type;
    static mut btowc_cache: [wint_t; 0];
    static mut lintfunc: Option::<unsafe extern "C" fn(*const libc::c_char, ...) -> ()>;
    fn set_loc(file: *const libc::c_char, line: libc::c_int);
    fn reisstring(
        text: *const libc::c_char,
        len: size_t,
        re: *mut Regexp,
        buf: *const libc::c_char,
    ) -> libc::c_int;
    fn research(
        rp: *mut Regexp,
        str: *mut libc::c_char,
        start: libc::c_int,
        len: size_t,
        flags: libc::c_int,
    ) -> libc::c_int;
    fn make_regexp(
        s: *const libc::c_char,
        len: size_t,
        ignorecase: bool,
        dfa: bool,
        canfatal: bool,
    ) -> *mut Regexp;
    fn refree(rp: *mut Regexp);
    fn r_dupnode(n: *mut NODE) -> *mut NODE;
    fn os_devopen(name: *const libc::c_char, flag: libc::c_int) -> libc::c_int;
    fn os_close_on_exec(
        fd: libc::c_int,
        name: *const libc::c_char,
        what: *const libc::c_char,
        dir: *const libc::c_char,
    );
    fn os_isatty(fd: libc::c_int) -> libc::c_int;
    fn os_isreadable(iobuf: *const awk_input_buf_t, isdir: *mut bool) -> libc::c_int;
    fn os_setbinmode(fd: libc::c_int, mode: libc::c_int) -> libc::c_int;
    fn os_restore_mode(fd: libc::c_int);
    fn os_maybe_set_errno();
    fn optimal_bufsize(fd: libc::c_int, sbuf: *mut stat) -> size_t;
    fn ispath(file: *const libc::c_char) -> libc::c_int;
    fn isdirpunct(c: libc::c_int) -> libc::c_int;
    fn init_sockets();
    fn getenv_long(name: *const libc::c_char) -> libc::c_long;
    fn r_fatal(mesg: *const libc::c_char, _: ...);
    fn make_str_node(
        s: *const libc::c_char,
        len: size_t,
        flags: libc::c_int,
    ) -> *mut NODE;
    fn r_warning(mesg: *const libc::c_char, _: ...);
    fn arg_assign(arg: *mut libc::c_char, initing: bool) -> libc::c_int;
    fn estrdup(str: *const libc::c_char, len: size_t) -> *mut libc::c_char;
    fn r_free_wstr(n: *mut NODE);
    fn waitpid(
        __pid: __pid_t,
        __stat_loc: *mut libc::c_int,
        __options: libc::c_int,
    ) -> __pid_t;
    fn ioctl(__fd: libc::c_int, __request: libc::c_ulong, _: ...) -> libc::c_int;
    fn tcsetattr(
        __fd: libc::c_int,
        __optional_actions: libc::c_int,
        __termios_p: *const termios,
    ) -> libc::c_int;
    fn tcgetattr(__fd: libc::c_int, __termios_p: *mut termios) -> libc::c_int;
    fn connect(
        __fd: libc::c_int,
        __addr: __CONST_SOCKADDR_ARG,
        __len: socklen_t,
    ) -> libc::c_int;
    fn recvfrom(
        __fd: libc::c_int,
        __buf: *mut libc::c_void,
        __n: size_t,
        __flags: libc::c_int,
        __addr: __SOCKADDR_ARG,
        __addr_len: *mut socklen_t,
    ) -> ssize_t;
    fn accept(
        __fd: libc::c_int,
        __addr: __SOCKADDR_ARG,
        __addr_len: *mut socklen_t,
    ) -> libc::c_int;
    fn listen(__fd: libc::c_int, __n: libc::c_int) -> libc::c_int;
    fn bind(
        __fd: libc::c_int,
        __addr: __CONST_SOCKADDR_ARG,
        __len: socklen_t,
    ) -> libc::c_int;
    fn setsockopt(
        __fd: libc::c_int,
        __level: libc::c_int,
        __optname: libc::c_int,
        __optval: *const libc::c_void,
        __optlen: socklen_t,
    ) -> libc::c_int;
    fn socket(
        __domain: libc::c_int,
        __type: libc::c_int,
        __protocol: libc::c_int,
    ) -> libc::c_int;
    fn shutdown(__fd: libc::c_int, __how: libc::c_int) -> libc::c_int;
    fn freeaddrinfo(__ai: *mut addrinfo);
    fn gai_strerror(__ecode: libc::c_int) -> *const libc::c_char;
    fn getaddrinfo(
        __name: *const libc::c_char,
        __service: *const libc::c_char,
        __req: *const addrinfo,
        __pai: *mut *mut addrinfo,
    ) -> libc::c_int;
    static mut ARGC_node: *mut NODE;
    static mut ARGV_node: *mut NODE;
    static mut ARGIND_node: *mut NODE;
}
pub type size_t = libc::c_ulong;
pub type __uint8_t = libc::c_uchar;
pub type __uint16_t = libc::c_ushort;
pub type __uint32_t = libc::c_uint;
pub type __dev_t = libc::c_ulong;
pub type __uid_t = libc::c_uint;
pub type __gid_t = libc::c_uint;
pub type __ino_t = libc::c_ulong;
pub type __mode_t = libc::c_uint;
pub type __nlink_t = libc::c_ulong;
pub type __off_t = libc::c_long;
pub type __off64_t = libc::c_long;
pub type __pid_t = libc::c_int;
pub type __time_t = libc::c_long;
pub type __useconds_t = libc::c_uint;
pub type __suseconds_t = libc::c_long;
pub type __blksize_t = libc::c_long;
pub type __blkcnt_t = libc::c_long;
pub type __ssize_t = libc::c_long;
pub type __syscall_slong_t = libc::c_long;
pub type __socklen_t = libc::c_uint;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct __mbstate_t {
    pub __count: libc::c_int,
    pub __value: C2RustUnnamed,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed {
    pub __wch: libc::c_uint,
    pub __wchb: [libc::c_char; 4],
}
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
pub type _IO_lock_t = ();
pub type FILE = _IO_FILE;
pub type ssize_t = __ssize_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct __sigset_t {
    pub __val: [libc::c_ulong; 16],
}
pub type sigset_t = __sigset_t;
pub type pid_t = __pid_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct timespec {
    pub tv_sec: __time_t,
    pub tv_nsec: __syscall_slong_t,
}
pub type __sighandler_t = Option::<unsafe extern "C" fn(libc::c_int) -> ()>;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct timeval {
    pub tv_sec: __time_t,
    pub tv_usec: __suseconds_t,
}
pub type time_t = __time_t;
pub type wchar_t = libc::c_int;
pub type __fd_mask = libc::c_long;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct fd_set {
    pub fds_bits: [__fd_mask; 16],
}
pub type wint_t = libc::c_uint;
pub type mbstate_t = __mbstate_t;
pub type uint8_t = __uint8_t;
pub type uint16_t = __uint16_t;
pub type uint32_t = __uint32_t;
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
pub type socklen_t = __socklen_t;
pub type __re_size_t = libc::c_uint;
pub type __re_long_size_t = libc::c_ulong;
pub type reg_syntax_t = libc::c_ulong;
#[derive(Copy, Clone, BitfieldStruct)]
#[repr(C)]
pub struct re_pattern_buffer {
    pub buffer: *mut re_dfa_t,
    pub allocated: __re_long_size_t,
    pub used: __re_long_size_t,
    pub syntax: reg_syntax_t,
    pub fastmap: *mut libc::c_char,
    pub translate: *mut libc::c_uchar,
    pub re_nsub: size_t,
    #[bitfield(name = "can_be_null", ty = "libc::c_uint", bits = "0..=0")]
    #[bitfield(name = "regs_allocated", ty = "libc::c_uint", bits = "1..=2")]
    #[bitfield(name = "fastmap_accurate", ty = "libc::c_uint", bits = "3..=3")]
    #[bitfield(name = "no_sub", ty = "libc::c_uint", bits = "4..=4")]
    #[bitfield(name = "not_bol", ty = "libc::c_uint", bits = "5..=5")]
    #[bitfield(name = "not_eol", ty = "libc::c_uint", bits = "6..=6")]
    #[bitfield(name = "newline_anchor", ty = "libc::c_uint", bits = "7..=7")]
    pub can_be_null_regs_allocated_fastmap_accurate_no_sub_not_bol_not_eol_newline_anchor: [u8; 1],
    #[bitfield(padding)]
    pub c2rust_padding: [u8; 7],
}
pub type regoff_t = libc::c_int;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct re_registers {
    pub num_regs: __re_size_t,
    pub start: *mut regoff_t,
    pub end: *mut regoff_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct Regexp {
    pub pat: re_pattern_buffer,
    pub regs: re_registers,
    pub dfareg: *mut dfa,
    pub has_meta: bool,
    pub maybe_long: bool,
}
pub type awk_bool = libc::c_uint;
pub const awk_true: awk_bool = 1;
pub const awk_false: awk_bool = 0;
pub type awk_bool_t = awk_bool;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct awk_fieldwidth_info_t {
    pub use_chars: awk_bool_t,
    pub nf: size_t,
    pub fields: [awk_field_info; 1],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct awk_field_info {
    pub skip: size_t,
    pub len: size_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct awk_input {
    pub name: *const libc::c_char,
    pub fd: libc::c_int,
    pub opaque: *mut libc::c_void,
    pub get_record: Option::<
        unsafe extern "C" fn(
            *mut *mut libc::c_char,
            *mut awk_input,
            *mut libc::c_int,
            *mut *mut libc::c_char,
            *mut size_t,
            *mut *const awk_fieldwidth_info_t,
        ) -> libc::c_int,
    >,
    pub read_func: Option::<
        unsafe extern "C" fn(libc::c_int, *mut libc::c_void, size_t) -> ssize_t,
    >,
    pub close_func: Option::<unsafe extern "C" fn(*mut awk_input) -> ()>,
    pub sbuf: stat,
}
pub type awk_input_buf_t = awk_input;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct awk_input_parser {
    pub name: *const libc::c_char,
    pub can_take_file: Option::<
        unsafe extern "C" fn(*const awk_input_buf_t) -> awk_bool_t,
    >,
    pub take_control_of: Option::<
        unsafe extern "C" fn(*mut awk_input_buf_t) -> awk_bool_t,
    >,
    pub next: *mut awk_input_parser,
}
pub type awk_input_parser_t = awk_input_parser;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct awk_output_buf {
    pub name: *const libc::c_char,
    pub mode: *const libc::c_char,
    pub fp: *mut FILE,
    pub redirected: awk_bool_t,
    pub opaque: *mut libc::c_void,
    pub gawk_fwrite: Option::<
        unsafe extern "C" fn(
            *const libc::c_void,
            size_t,
            size_t,
            *mut FILE,
            *mut libc::c_void,
        ) -> size_t,
    >,
    pub gawk_fflush: Option::<
        unsafe extern "C" fn(*mut FILE, *mut libc::c_void) -> libc::c_int,
    >,
    pub gawk_ferror: Option::<
        unsafe extern "C" fn(*mut FILE, *mut libc::c_void) -> libc::c_int,
    >,
    pub gawk_fclose: Option::<
        unsafe extern "C" fn(*mut FILE, *mut libc::c_void) -> libc::c_int,
    >,
}
pub type awk_output_buf_t = awk_output_buf;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct awk_output_wrapper {
    pub name: *const libc::c_char,
    pub can_take_file: Option::<
        unsafe extern "C" fn(*const awk_output_buf_t) -> awk_bool_t,
    >,
    pub take_control_of: Option::<
        unsafe extern "C" fn(*mut awk_output_buf_t) -> awk_bool_t,
    >,
    pub next: *mut awk_output_wrapper,
}
pub type awk_output_wrapper_t = awk_output_wrapper;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct awk_two_way_processor {
    pub name: *const libc::c_char,
    pub can_take_two_way: Option::<
        unsafe extern "C" fn(*const libc::c_char) -> awk_bool_t,
    >,
    pub take_control_of: Option::<
        unsafe extern "C" fn(
            *const libc::c_char,
            *mut awk_input_buf_t,
            *mut awk_output_buf_t,
        ) -> awk_bool_t,
    >,
    pub next: *mut awk_two_way_processor,
}
pub type awk_two_way_processor_t = awk_two_way_processor;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct awk_string {
    pub str_0: *mut libc::c_char,
    pub len: size_t,
}
pub type awk_string_t = awk_string;
pub type AWK_NUMBER_TYPE = libc::c_uint;
pub const AWK_NUMBER_TYPE_MPZ: AWK_NUMBER_TYPE = 2;
pub const AWK_NUMBER_TYPE_MPFR: AWK_NUMBER_TYPE = 1;
pub const AWK_NUMBER_TYPE_DOUBLE: AWK_NUMBER_TYPE = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct awk_number {
    pub d: libc::c_double,
    pub type_0: AWK_NUMBER_TYPE,
    pub ptr: *mut libc::c_void,
}
pub type awk_number_t = awk_number;
pub type awk_array_t = *mut libc::c_void;
pub type awk_scalar_t = *mut libc::c_void;
pub type awk_value_cookie_t = *mut libc::c_void;
pub type awk_valtype_t = libc::c_uint;
pub const AWK_BOOL: awk_valtype_t = 8;
pub const AWK_VALUE_COOKIE: awk_valtype_t = 7;
pub const AWK_SCALAR: awk_valtype_t = 6;
pub const AWK_ARRAY: awk_valtype_t = 5;
pub const AWK_STRNUM: awk_valtype_t = 4;
pub const AWK_REGEX: awk_valtype_t = 3;
pub const AWK_STRING: awk_valtype_t = 2;
pub const AWK_NUMBER: awk_valtype_t = 1;
pub const AWK_UNDEFINED: awk_valtype_t = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct awk_value {
    pub val_type: awk_valtype_t,
    pub u: C2RustUnnamed_0,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_0 {
    pub s: awk_string_t,
    pub n: awk_number_t,
    pub a: awk_array_t,
    pub scl: awk_scalar_t,
    pub vc: awk_value_cookie_t,
    pub b: awk_bool_t,
}
pub type awk_value_t = awk_value;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct awk_ext_func {
    pub name: *const libc::c_char,
    pub function: Option::<
        unsafe extern "C" fn(
            libc::c_int,
            *mut awk_value_t,
            *mut awk_ext_func,
        ) -> *mut awk_value_t,
    >,
    pub max_expected_args: size_t,
    pub min_required_args: size_t,
    pub suppress_lint: awk_bool_t,
    pub data: *mut libc::c_void,
}
pub type awk_ext_func_t = awk_ext_func;
pub type nodevals = libc::c_uint;
pub const Node_final: nodevals = 19;
pub const Node_instruction: nodevals = 18;
pub const Node_frame: nodevals = 17;
pub const Node_arrayfor: nodevals = 16;
pub const Node_dump_array: nodevals = 15;
pub const Node_array_leaf: nodevals = 14;
pub const Node_array_tree: nodevals = 13;
pub const Node_array_ref: nodevals = 12;
pub const Node_builtin_func: nodevals = 11;
pub const Node_ext_func: nodevals = 10;
pub const Node_func: nodevals = 9;
pub const Node_param_list: nodevals = 8;
pub const Node_elem_new: nodevals = 7;
pub const Node_var_new: nodevals = 6;
pub const Node_var_array: nodevals = 5;
pub const Node_var: nodevals = 4;
pub const Node_dynregex: nodevals = 3;
pub const Node_regex: nodevals = 2;
pub const Node_val: nodevals = 1;
pub const Node_illegal: nodevals = 0;
pub type NODETYPE = nodevals;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct exp_node {
    pub sub: C2RustUnnamed_1,
    pub type_0: NODETYPE,
    pub flags: flagvals,
    pub valref: libc::c_long,
}
pub type flagvals = libc::c_uint;
pub const REGEX: flagvals = 524288;
pub const NUMCONSTSTR: flagvals = 262144;
pub const XARRAY: flagvals = 131072;
pub const HALFHAT: flagvals = 65536;
pub const ARRAYMAXED: flagvals = 32768;
pub const NULL_FIELD: flagvals = 16384;
pub const NO_EXT_SET: flagvals = 8192;
pub const MPZN: flagvals = 4096;
pub const MPFN: flagvals = 2048;
pub const WSTRCUR: flagvals = 1024;
pub const INTIND: flagvals = 512;
pub const NUMINT: flagvals = 256;
pub const INTLSTR: flagvals = 128;
pub const BOOLVAL: flagvals = 64;
pub const USER_INPUT: flagvals = 32;
pub const NUMBER: flagvals = 16;
pub const NUMCUR: flagvals = 8;
pub const STRCUR: flagvals = 4;
pub const STRING: flagvals = 2;
pub const MALLOC: flagvals = 1;
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_1 {
    pub nodep: C2RustUnnamed_3,
    pub val: C2RustUnnamed_2,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_2 {
    pub fltnum: libc::c_double,
    pub sp: *mut libc::c_char,
    pub slen: size_t,
    pub idx: libc::c_int,
    pub wsp: *mut wchar_t,
    pub wslen: size_t,
    pub typre: *mut exp_node,
    pub comtype: commenttype,
}
pub type commenttype = libc::c_uint;
pub const FOR_COMMENT: commenttype = 3;
pub const BLOCK_COMMENT: commenttype = 2;
pub const EOL_COMMENT: commenttype = 1;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_3 {
    pub l: C2RustUnnamed_10,
    pub r: C2RustUnnamed_5,
    pub x: C2RustUnnamed_4,
    pub name: *mut libc::c_char,
    pub reserved: size_t,
    pub rn: *mut exp_node,
    pub cnt: libc::c_ulong,
    pub reflags: reflagvals,
}
pub type reflagvals = libc::c_uint;
pub const FS_DFLT: reflagvals = 2;
pub const CONSTANT: reflagvals = 1;
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_4 {
    pub extra: *mut exp_node,
    pub aptr: Option::<unsafe extern "C" fn() -> ()>,
    pub xl: libc::c_long,
    pub cmnt: *mut libc::c_void,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_5 {
    pub rptr: *mut exp_node,
    pub preg: [*mut Regexp; 2],
    pub av: *mut *mut exp_node,
    pub bv: *mut *mut BUCKET,
    pub uptr: Option::<unsafe extern "C" fn() -> ()>,
    pub iptr: *mut exp_instruction,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct exp_instruction {
    pub nexti: *mut exp_instruction,
    pub d: C2RustUnnamed_7,
    pub x: C2RustUnnamed_6,
    pub comment: *mut exp_instruction,
    pub source_line: libc::c_short,
    pub pool_size: libc::c_short,
    pub opcode: OPCODE,
}
pub type OPCODE = opcodeval;
pub type opcodeval = libc::c_uint;
pub const Op_final: opcodeval = 122;
pub const Op_parens: opcodeval = 121;
pub const Op_cond_exp: opcodeval = 120;
pub const Op_K_function: opcodeval = 119;
pub const Op_K_else: opcodeval = 118;
pub const Op_K_if: opcodeval = 117;
pub const Op_K_switch: opcodeval = 116;
pub const Op_K_while: opcodeval = 115;
pub const Op_K_arrayfor: opcodeval = 114;
pub const Op_K_for: opcodeval = 113;
pub const Op_K_do: opcodeval = 112;
pub const Op_list: opcodeval = 111;
pub const Op_symbol: opcodeval = 110;
pub const Op_token: opcodeval = 109;
pub const Op_stop: opcodeval = 108;
pub const Op_atexit: opcodeval = 107;
pub const Op_lint_plus: opcodeval = 106;
pub const Op_lint: opcodeval = 105;
pub const Op_breakpoint: opcodeval = 104;
pub const Op_exec_count: opcodeval = 103;
pub const Op_comment: opcodeval = 102;
pub const Op_func: opcodeval = 101;
pub const Op_after_endfile: opcodeval = 100;
pub const Op_after_beginfile: opcodeval = 99;
pub const Op_subscript_assign: opcodeval = 98;
pub const Op_field_assign: opcodeval = 97;
pub const Op_var_assign: opcodeval = 96;
pub const Op_var_update: opcodeval = 95;
pub const Op_arrayfor_final: opcodeval = 94;
pub const Op_arrayfor_incr: opcodeval = 93;
pub const Op_arrayfor_init: opcodeval = 92;
pub const Op_newfile: opcodeval = 91;
pub const Op_get_record: opcodeval = 90;
pub const Op_jmp_false: opcodeval = 89;
pub const Op_jmp_true: opcodeval = 88;
pub const Op_jmp: opcodeval = 87;
pub const Op_pop: opcodeval = 86;
pub const Op_no_op: opcodeval = 85;
pub const Op_field_spec_lhs: opcodeval = 84;
pub const Op_subscript_lhs: opcodeval = 83;
pub const Op_push_lhs: opcodeval = 82;
pub const Op_push_param: opcodeval = 81;
pub const Op_push_array: opcodeval = 80;
pub const Op_push_re: opcodeval = 79;
pub const Op_push_i: opcodeval = 78;
pub const Op_push_arg_untyped: opcodeval = 77;
pub const Op_push_arg: opcodeval = 76;
pub const Op_push: opcodeval = 75;
pub const Op_indirect_func_call: opcodeval = 74;
pub const Op_func_call: opcodeval = 73;
pub const Op_in_array: opcodeval = 72;
pub const Op_ext_builtin: opcodeval = 71;
pub const Op_sub_builtin: opcodeval = 70;
pub const Op_builtin: opcodeval = 69;
pub const Op_K_namespace: opcodeval = 68;
pub const Op_K_nextfile: opcodeval = 67;
pub const Op_K_getline: opcodeval = 66;
pub const Op_K_getline_redir: opcodeval = 65;
pub const Op_K_delete_loop: opcodeval = 64;
pub const Op_K_delete: opcodeval = 63;
pub const Op_K_return_from_eval: opcodeval = 62;
pub const Op_K_return: opcodeval = 61;
pub const Op_K_exit: opcodeval = 60;
pub const Op_K_next: opcodeval = 59;
pub const Op_K_printf: opcodeval = 58;
pub const Op_K_print_rec: opcodeval = 57;
pub const Op_K_print: opcodeval = 56;
pub const Op_K_continue: opcodeval = 55;
pub const Op_K_break: opcodeval = 54;
pub const Op_K_default: opcodeval = 53;
pub const Op_K_case: opcodeval = 52;
pub const Op_rule: opcodeval = 51;
pub const Op_nomatch: opcodeval = 50;
pub const Op_match_rec: opcodeval = 49;
pub const Op_match: opcodeval = 48;
pub const Op_geq: opcodeval = 47;
pub const Op_leq: opcodeval = 46;
pub const Op_greater: opcodeval = 45;
pub const Op_less: opcodeval = 44;
pub const Op_notequal: opcodeval = 43;
pub const Op_equal: opcodeval = 42;
pub const Op_or_final: opcodeval = 41;
pub const Op_or: opcodeval = 40;
pub const Op_and_final: opcodeval = 39;
pub const Op_and: opcodeval = 38;
pub const Op_assign_concat: opcodeval = 37;
pub const Op_assign_exp: opcodeval = 36;
pub const Op_assign_minus: opcodeval = 35;
pub const Op_assign_plus: opcodeval = 34;
pub const Op_assign_mod: opcodeval = 33;
pub const Op_assign_quotient: opcodeval = 32;
pub const Op_assign_times: opcodeval = 31;
pub const Op_store_field_exp: opcodeval = 30;
pub const Op_store_field: opcodeval = 29;
pub const Op_store_sub: opcodeval = 28;
pub const Op_store_var: opcodeval = 27;
pub const Op_assign: opcodeval = 26;
pub const Op_not: opcodeval = 25;
pub const Op_field_spec: opcodeval = 24;
pub const Op_unary_plus: opcodeval = 23;
pub const Op_unary_minus: opcodeval = 22;
pub const Op_postdecrement: opcodeval = 21;
pub const Op_postincrement: opcodeval = 20;
pub const Op_predecrement: opcodeval = 19;
pub const Op_preincrement: opcodeval = 18;
pub const Op_sub_array: opcodeval = 17;
pub const Op_subscript: opcodeval = 16;
pub const Op_cond_pair: opcodeval = 15;
pub const Op_line_range: opcodeval = 14;
pub const Op_concat: opcodeval = 13;
pub const Op_exp_i: opcodeval = 12;
pub const Op_exp: opcodeval = 11;
pub const Op_minus_i: opcodeval = 10;
pub const Op_minus: opcodeval = 9;
pub const Op_plus_i: opcodeval = 8;
pub const Op_plus: opcodeval = 7;
pub const Op_mod_i: opcodeval = 6;
pub const Op_mod: opcodeval = 5;
pub const Op_quotient_i: opcodeval = 4;
pub const Op_quotient: opcodeval = 3;
pub const Op_times_i: opcodeval = 2;
pub const Op_times: opcodeval = 1;
pub const Op_illegal: opcodeval = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_6 {
    pub xl: libc::c_long,
    pub xn: *mut NODE,
    pub aptr: Option::<unsafe extern "C" fn() -> ()>,
    pub xi: *mut exp_instruction,
    pub bpt: *mut break_point,
    pub exf: *mut awk_ext_func_t,
}
pub type NODE = exp_node;
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_7 {
    pub dn: *mut NODE,
    pub di: *mut exp_instruction,
    pub fptr: Option::<unsafe extern "C" fn(libc::c_int) -> *mut NODE>,
    pub efptr: Option::<
        unsafe extern "C" fn(
            libc::c_int,
            *mut awk_value_t,
            *mut awk_ext_func,
        ) -> *mut awk_value_t,
    >,
    pub dl: libc::c_long,
    pub ldl: exec_count_t,
    pub name: *mut libc::c_char,
}
pub type exec_count_t = libc::c_ulonglong;
pub type BUCKET = bucket_item;
#[derive(Copy, Clone)]
#[repr(C)]
pub union bucket_item {
    pub hs: C2RustUnnamed_9,
    pub hi: C2RustUnnamed_8,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_8 {
    pub next: *mut bucket_item,
    pub li: [libc::c_long; 2],
    pub val: [*mut exp_node; 2],
    pub cnt: size_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_9 {
    pub next: *mut bucket_item,
    pub str_0: *mut libc::c_char,
    pub len: size_t,
    pub code: size_t,
    pub name: *mut exp_node,
    pub val: *mut exp_node,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_10 {
    pub lptr: *mut exp_node,
    pub li: *mut exp_instruction,
    pub ll: libc::c_long,
    pub lp: *const array_funcs_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct array_funcs_t {
    pub name: *const libc::c_char,
    pub init: afunc_t,
    pub type_of: afunc_t,
    pub lookup: afunc_t,
    pub exists: afunc_t,
    pub clear: afunc_t,
    pub remove: afunc_t,
    pub list: afunc_t,
    pub copy: afunc_t,
    pub dump: afunc_t,
    pub store: afunc_t,
}
pub type afunc_t = Option::<
    unsafe extern "C" fn(*mut exp_node, *mut exp_node) -> *mut *mut exp_node,
>;
pub type redirval = libc::c_uint;
pub const redirect_twoway: redirval = 6;
pub const redirect_input: redirval = 5;
pub const redirect_pipein: redirval = 4;
pub const redirect_pipe: redirval = 3;
pub const redirect_append: redirval = 2;
pub const redirect_output: redirval = 1;
pub const redirect_none: redirval = 0;
pub type INSTRUCTION = exp_instruction;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct iobuf {
    pub public: awk_input_buf_t,
    pub buf: *mut libc::c_char,
    pub off: *mut libc::c_char,
    pub dataend: *mut libc::c_char,
    pub end: *mut libc::c_char,
    pub readsize: size_t,
    pub size: size_t,
    pub count: ssize_t,
    pub scanoff: size_t,
    pub valid: bool,
    pub errcode: libc::c_int,
    pub flag: iobuf_flags,
}
pub type iobuf_flags = libc::c_uint;
pub const IOP_AT_START: iobuf_flags = 8;
pub const IOP_CLOSED: iobuf_flags = 4;
pub const IOP_AT_EOF: iobuf_flags = 2;
pub const IOP_IS_TTY: iobuf_flags = 1;
pub type IOBUF = iobuf;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct redirect {
    pub flag: redirect_flags,
    pub value: *mut libc::c_char,
    pub ifp: *mut FILE,
    pub iop: *mut IOBUF,
    pub pid: libc::c_int,
    pub status: libc::c_int,
    pub prev: *mut redirect,
    pub next: *mut redirect,
    pub mode: *const libc::c_char,
    pub output: awk_output_buf_t,
}
pub type redirect_flags = libc::c_uint;
pub const RED_TCP: redirect_flags = 2048;
pub const RED_SOCKET: redirect_flags = 1024;
pub const RED_PTY: redirect_flags = 512;
pub const RED_TWOWAY: redirect_flags = 256;
pub const RED_EOF: redirect_flags = 128;
pub const RED_USED: redirect_flags = 64;
pub const RED_FLUSH: redirect_flags = 32;
pub const RED_APPEND: redirect_flags = 16;
pub const RED_WRITE: redirect_flags = 8;
pub const RED_READ: redirect_flags = 4;
pub const RED_PIPE: redirect_flags = 2;
pub const RED_FILE: redirect_flags = 1;
pub const RED_NONE: redirect_flags = 0;
pub type redirect_flags_t = redirect_flags;
pub type binmode_values = libc::c_uint;
pub const BINMODE_BOTH: binmode_values = 3;
pub const BINMODE_OUTPUT: binmode_values = 2;
pub const BINMODE_INPUT: binmode_values = 1;
pub const TEXT_TRANSLATE: binmode_values = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct srcfile {
    pub next: *mut srcfile,
    pub prev: *mut srcfile,
    pub stype: srctype,
    pub src: *mut libc::c_char,
    pub fullpath: *mut libc::c_char,
    pub mtime: time_t,
    pub sbuf: stat,
    pub srclines: libc::c_int,
    pub bufsize: size_t,
    pub buf: *mut libc::c_char,
    pub line_offset: *mut libc::c_int,
    pub fd: libc::c_int,
    pub maxlen: libc::c_int,
    pub fini_func: Option::<unsafe extern "C" fn() -> ()>,
    pub lexptr: *mut libc::c_char,
    pub lexend: *mut libc::c_char,
    pub lexeme: *mut libc::c_char,
    pub lexptr_begin: *mut libc::c_char,
    pub lasttok: libc::c_int,
    pub comment: *mut INSTRUCTION,
    pub namespace: *const libc::c_char,
}
pub type srctype = libc::c_uint;
pub const SRC_EXTLIB: srctype = 5;
pub const SRC_INC: srctype = 4;
pub const SRC_FILE: srctype = 3;
pub const SRC_STDIN: srctype = 2;
pub const SRC_CMDLINE: srctype = 1;
pub type SRCFILE = srcfile;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct flagtab {
    pub val: libc::c_int,
    pub name: *const libc::c_char,
}
pub type do_flag_values = libc::c_uint;
pub const DO_MPFR: do_flag_values = 32768;
pub const DO_DEBUG: do_flag_values = 16384;
pub const DO_PROFILE: do_flag_values = 8192;
pub const DO_SANDBOX: do_flag_values = 4096;
pub const DO_TIDY_MEM: do_flag_values = 2048;
pub const DO_DUMP_VARS: do_flag_values = 1024;
pub const DO_PRETTY_PRINT: do_flag_values = 512;
pub const DO_INTERVALS: do_flag_values = 256;
pub const DO_NON_DEC_DATA: do_flag_values = 128;
pub const DO_INTL: do_flag_values = 64;
pub const DO_POSIX: do_flag_values = 32;
pub const DO_TRADITIONAL: do_flag_values = 16;
pub const DO_LINT_OLD: do_flag_values = 8;
pub const DO_LINT_ALL: do_flag_values = 4;
pub const DO_LINT_EXTENSIONS: do_flag_values = 2;
pub const DO_LINT_INVALID: do_flag_values = 1;
pub const DO_FLAG_NONE: do_flag_values = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub union stack_item {
    pub rptr: *mut NODE,
    pub lptr: *mut *mut NODE,
}
pub type STACK_ITEM = stack_item;
pub const Using_FS: field_sep_type = 0;
pub type field_sep_type = libc::c_uint;
pub const Using_API: field_sep_type = 3;
pub const Using_FPAT: field_sep_type = 2;
pub const Using_FIELDWIDTHS: field_sep_type = 1;
pub type SCANSTATE = scanstate;
pub type scanstate = libc::c_uint;
pub const INTERM: scanstate = 3;
pub const INDATA: scanstate = 2;
pub const INLEADER: scanstate = 1;
pub const NOSTATE: scanstate = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct recmatch {
    pub start: *mut libc::c_char,
    pub len: size_t,
    pub rt_start: *mut libc::c_char,
    pub rt_len: size_t,
}
pub type RECVALUE = recvalues;
pub type recvalues = libc::c_uint;
pub const TERMNEAREND: recvalues = 3;
pub const TERMATEND: recvalues = 2;
pub const NOTERM: recvalues = 1;
pub const REC_OK: recvalues = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_11 {
    pub offset: libc::c_int,
    pub len: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct inet_socket_info {
    pub family: libc::c_int,
    pub protocol: libc::c_int,
    pub localport: C2RustUnnamed_11,
    pub remotehost: C2RustUnnamed_11,
    pub remoteport: C2RustUnnamed_11,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct addrinfo {
    pub ai_flags: libc::c_int,
    pub ai_family: libc::c_int,
    pub ai_socktype: libc::c_int,
    pub ai_protocol: libc::c_int,
    pub ai_addrlen: socklen_t,
    pub ai_addr: *mut sockaddr,
    pub ai_canonname: *mut libc::c_char,
    pub ai_next: *mut addrinfo,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct sockaddr {
    pub sa_family: sa_family_t,
    pub sa_data: [libc::c_char; 14],
}
pub type sa_family_t = libc::c_ushort;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct sockaddr_storage {
    pub ss_family: sa_family_t,
    pub __ss_padding: [libc::c_char; 118],
    pub __ss_align: libc::c_ulong,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union __CONST_SOCKADDR_ARG {
    pub __sockaddr__: *const sockaddr,
    pub __sockaddr_at__: *const sockaddr_at,
    pub __sockaddr_ax25__: *const sockaddr_ax25,
    pub __sockaddr_dl__: *const sockaddr_dl,
    pub __sockaddr_eon__: *const sockaddr_eon,
    pub __sockaddr_in__: *const sockaddr_in,
    pub __sockaddr_in6__: *const sockaddr_in6,
    pub __sockaddr_inarp__: *const sockaddr_inarp,
    pub __sockaddr_ipx__: *const sockaddr_ipx,
    pub __sockaddr_iso__: *const sockaddr_iso,
    pub __sockaddr_ns__: *const sockaddr_ns,
    pub __sockaddr_un__: *const sockaddr_un,
    pub __sockaddr_x25__: *const sockaddr_x25,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct sockaddr_in6 {
    pub sin6_family: sa_family_t,
    pub sin6_port: in_port_t,
    pub sin6_flowinfo: uint32_t,
    pub sin6_addr: in6_addr,
    pub sin6_scope_id: uint32_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct in6_addr {
    pub __in6_u: C2RustUnnamed_12,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_12 {
    pub __u6_addr8: [uint8_t; 16],
    pub __u6_addr16: [uint16_t; 8],
    pub __u6_addr32: [uint32_t; 4],
}
pub type in_port_t = uint16_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct sockaddr_in {
    pub sin_family: sa_family_t,
    pub sin_port: in_port_t,
    pub sin_addr: in_addr,
    pub sin_zero: [libc::c_uchar; 8],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct in_addr {
    pub s_addr: in_addr_t,
}
pub type in_addr_t = uint32_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub union __SOCKADDR_ARG {
    pub __sockaddr__: *mut sockaddr,
    pub __sockaddr_at__: *mut sockaddr_at,
    pub __sockaddr_ax25__: *mut sockaddr_ax25,
    pub __sockaddr_dl__: *mut sockaddr_dl,
    pub __sockaddr_eon__: *mut sockaddr_eon,
    pub __sockaddr_in__: *mut sockaddr_in,
    pub __sockaddr_in6__: *mut sockaddr_in6,
    pub __sockaddr_inarp__: *mut sockaddr_inarp,
    pub __sockaddr_ipx__: *mut sockaddr_ipx,
    pub __sockaddr_iso__: *mut sockaddr_iso,
    pub __sockaddr_ns__: *mut sockaddr_ns,
    pub __sockaddr_un__: *mut sockaddr_un,
    pub __sockaddr_x25__: *mut sockaddr_x25,
}
pub const MSG_PEEK: C2RustUnnamed_13 = 2;
pub const SOCK_DGRAM: __socket_type = 2;
pub const SOCK_STREAM: __socket_type = 1;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct linger {
    pub l_onoff: libc::c_int,
    pub l_linger: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct termios {
    pub c_iflag: tcflag_t,
    pub c_oflag: tcflag_t,
    pub c_cflag: tcflag_t,
    pub c_lflag: tcflag_t,
    pub c_line: cc_t,
    pub c_cc: [cc_t; 32],
    pub c_ispeed: speed_t,
    pub c_ospeed: speed_t,
}
pub type speed_t = libc::c_uint;
pub type cc_t = libc::c_uchar;
pub type tcflag_t = libc::c_uint;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct mixture {
    pub common: redirect_flags_t,
    pub mode: redirect_flags_t,
    pub other_mode: redirect_flags_t,
    pub message: *const libc::c_char,
}
pub type two_way_close_type = libc::c_uint;
pub const CLOSE_FROM: two_way_close_type = 2;
pub const CLOSE_TO: two_way_close_type = 1;
pub const CLOSE_ALL: two_way_close_type = 0;
pub const SHUT_RD: C2RustUnnamed_14 = 0;
pub const SHUT_WR: C2RustUnnamed_14 = 1;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct path_info {
    pub envname: *const libc::c_char,
    pub dfltp: *mut *const libc::c_char,
    pub awkpath: *mut *const libc::c_char,
    pub max_pathlen: libc::c_int,
}
pub type __socket_type = libc::c_uint;
pub const SOCK_NONBLOCK: __socket_type = 2048;
pub const SOCK_CLOEXEC: __socket_type = 524288;
pub const SOCK_PACKET: __socket_type = 10;
pub const SOCK_DCCP: __socket_type = 6;
pub const SOCK_SEQPACKET: __socket_type = 5;
pub const SOCK_RDM: __socket_type = 4;
pub const SOCK_RAW: __socket_type = 3;
pub type C2RustUnnamed_13 = libc::c_uint;
pub const MSG_CMSG_CLOEXEC: C2RustUnnamed_13 = 1073741824;
pub const MSG_FASTOPEN: C2RustUnnamed_13 = 536870912;
pub const MSG_ZEROCOPY: C2RustUnnamed_13 = 67108864;
pub const MSG_BATCH: C2RustUnnamed_13 = 262144;
pub const MSG_WAITFORONE: C2RustUnnamed_13 = 65536;
pub const MSG_MORE: C2RustUnnamed_13 = 32768;
pub const MSG_NOSIGNAL: C2RustUnnamed_13 = 16384;
pub const MSG_ERRQUEUE: C2RustUnnamed_13 = 8192;
pub const MSG_RST: C2RustUnnamed_13 = 4096;
pub const MSG_CONFIRM: C2RustUnnamed_13 = 2048;
pub const MSG_SYN: C2RustUnnamed_13 = 1024;
pub const MSG_FIN: C2RustUnnamed_13 = 512;
pub const MSG_WAITALL: C2RustUnnamed_13 = 256;
pub const MSG_EOR: C2RustUnnamed_13 = 128;
pub const MSG_DONTWAIT: C2RustUnnamed_13 = 64;
pub const MSG_TRUNC: C2RustUnnamed_13 = 32;
pub const MSG_PROXY: C2RustUnnamed_13 = 16;
pub const MSG_CTRUNC: C2RustUnnamed_13 = 8;
pub const MSG_TRYHARD: C2RustUnnamed_13 = 4;
pub const MSG_DONTROUTE: C2RustUnnamed_13 = 4;
pub const MSG_OOB: C2RustUnnamed_13 = 1;
pub type C2RustUnnamed_14 = libc::c_uint;
pub const SHUT_RDWR: C2RustUnnamed_14 = 2;
#[inline]
unsafe extern "C" fn putc_unlocked(
    mut __c: libc::c_int,
    mut __stream: *mut FILE,
) -> libc::c_int {
    return if ((*__stream)._IO_write_ptr >= (*__stream)._IO_write_end) as libc::c_int
        as libc::c_long != 0
    {
        __overflow(__stream, __c as libc::c_uchar as libc::c_int)
    } else {
        let fresh0 = (*__stream)._IO_write_ptr;
        (*__stream)._IO_write_ptr = ((*__stream)._IO_write_ptr).offset(1);
        *fresh0 = __c as libc::c_char;
        *fresh0 as libc::c_uchar as libc::c_int
    };
}
#[inline]
unsafe extern "C" fn mbrlen(
    mut __s: *const libc::c_char,
    mut __n: size_t,
    mut __ps: *mut mbstate_t,
) -> size_t {
    return if !__ps.is_null() {
        mbrtowc(0 as *mut wchar_t, __s, __n, __ps)
    } else {
        __mbrlen(__s, __n, 0 as *mut mbstate_t)
    };
}
#[inline]
unsafe extern "C" fn stat(
    mut __path: *const libc::c_char,
    mut __statbuf: *mut stat,
) -> libc::c_int {
    return __xstat(1 as libc::c_int, __path, __statbuf);
}
#[inline]
unsafe extern "C" fn fstat(
    mut __fd: libc::c_int,
    mut __statbuf: *mut stat,
) -> libc::c_int {
    return __fxstat(1 as libc::c_int, __fd, __statbuf);
}
#[inline]
unsafe extern "C" fn lstat(
    mut __path: *const libc::c_char,
    mut __statbuf: *mut stat,
) -> libc::c_int {
    return __lxstat(1 as libc::c_int, __path, __statbuf);
}
#[inline]
unsafe extern "C" fn DEREF(mut r: *mut NODE) {
    (*r).valref -= 1;
    if (*r).valref > 0 as libc::c_int as libc::c_long {
        return;
    }
    r_unref(r);
}
#[inline]
unsafe extern "C" fn force_string_fmt(
    mut s: *mut NODE,
    mut fmtstr: *const libc::c_char,
    mut fmtidx: libc::c_int,
) -> *mut NODE {
    if (*s).type_0 as libc::c_uint == Node_elem_new as libc::c_int as libc::c_uint {
        (*s).type_0 = Node_val;
        (*s)
            .flags = ::std::mem::transmute::<
            libc::c_uint,
            flagvals,
        >((*s).flags as libc::c_uint & !(NUMBER as libc::c_int) as libc::c_uint);
        return s;
    }
    if (*s).flags as libc::c_uint & STRCUR as libc::c_int as libc::c_uint
        != 0 as libc::c_int as libc::c_uint
        && ((*s).sub.val.idx == -(1 as libc::c_int) || (*s).sub.val.idx == fmtidx)
    {
        return s;
    }
    return format_val.unwrap()(fmtstr, fmtidx, s);
}
#[inline]
unsafe extern "C" fn dupnode(mut n: *mut NODE) -> *mut NODE {
    if (*n).flags as libc::c_uint & MALLOC as libc::c_int as libc::c_uint
        != 0 as libc::c_int as libc::c_uint
    {
        (*n).valref += 1;
        (*n).valref;
        return n;
    }
    return r_dupnode(n);
}
#[inline]
unsafe extern "C" fn unref(mut r: *mut NODE) {
    if !r.is_null()
        && {
            (*r).valref -= 1;
            (*r).valref <= 0 as libc::c_int as libc::c_long
        }
    {
        r_unref(r);
    }
}
#[inline]
unsafe extern "C" fn force_number(mut n: *mut NODE) -> *mut NODE {
    return if (*n).flags as libc::c_uint & NUMCUR as libc::c_int as libc::c_uint
        != 0 as libc::c_int as libc::c_uint
    {
        n
    } else {
        str2number.unwrap()(n)
    };
}
#[inline]
unsafe extern "C" fn fixtype(mut n: *mut NODE) -> *mut NODE {
    if (*n).flags as libc::c_uint
        & (NUMCUR as libc::c_int | USER_INPUT as libc::c_int) as libc::c_uint
        == USER_INPUT as libc::c_int as libc::c_uint
    {
        return force_number(n);
    }
    if (*n).flags as libc::c_uint & INTIND as libc::c_int as libc::c_uint
        != 0 as libc::c_int as libc::c_uint
    {
        return force_string_fmt(n, CONVFMT, CONVFMTidx);
    }
    return n;
}
#[inline]
unsafe extern "C" fn in_array(mut a: *mut NODE, mut s: *mut NODE) -> *mut NODE {
    let mut ret: *mut *mut NODE = 0 as *mut *mut NODE;
    ret = ((*(*a).sub.nodep.l.lp).exists).unwrap()(a, s);
    return if !ret.is_null() { *ret } else { 0 as *mut NODE };
}
#[inline]
unsafe extern "C" fn erealloc_real(
    mut ptr: *mut libc::c_void,
    mut count: size_t,
    mut where_0: *const libc::c_char,
    mut var: *const libc::c_char,
    mut file: *const libc::c_char,
    mut line: libc::c_int,
) -> *mut libc::c_void {
    let mut ret: *mut libc::c_void = 0 as *mut libc::c_void;
    if count == 0 as libc::c_int as libc::c_ulong {
        (set_loc
            as unsafe extern "C" fn(
                *const libc::c_char,
                libc::c_int,
            ) -> ())(
            b"./awk.h\0" as *const u8 as *const libc::c_char,
            2088 as libc::c_int,
        );
        (Some(
            (Some(r_fatal as unsafe extern "C" fn(*const libc::c_char, ...) -> ()))
                .unwrap(),
        ))
            .unwrap()(
            b"%s:%d: erealloc called with zero bytes\0" as *const u8
                as *const libc::c_char,
            file,
            line,
        );
    }
    ret = realloc(ptr, count);
    if ret.is_null() {
        (set_loc
            as unsafe extern "C" fn(
                *const libc::c_char,
                libc::c_int,
            ) -> ())(
            b"./awk.h\0" as *const u8 as *const libc::c_char,
            2092 as libc::c_int,
        );
        (Some(
            (Some(r_fatal as unsafe extern "C" fn(*const libc::c_char, ...) -> ()))
                .unwrap(),
        ))
            .unwrap()(
            dcgettext(
                0 as *const libc::c_char,
                b"%s:%d:%s: %s: cannot reallocate %ld bytes of memory: %s\0" as *const u8
                    as *const libc::c_char,
                5 as libc::c_int,
            ),
            file,
            line,
            where_0,
            var,
            count as libc::c_long,
            strerror(*__errno_location()),
        );
    }
    return ret;
}
#[inline]
unsafe extern "C" fn emalloc_real(
    mut count: size_t,
    mut where_0: *const libc::c_char,
    mut var: *const libc::c_char,
    mut file: *const libc::c_char,
    mut line: libc::c_int,
) -> *mut libc::c_void {
    let mut ret: *mut libc::c_void = 0 as *mut libc::c_void;
    if count == 0 as libc::c_int as libc::c_ulong {
        (set_loc
            as unsafe extern "C" fn(
                *const libc::c_char,
                libc::c_int,
            ) -> ())(
            b"./awk.h\0" as *const u8 as *const libc::c_char,
            2052 as libc::c_int,
        );
        (Some(
            (Some(r_fatal as unsafe extern "C" fn(*const libc::c_char, ...) -> ()))
                .unwrap(),
        ))
            .unwrap()(
            b"%s:%d: emalloc called with zero bytes\0" as *const u8
                as *const libc::c_char,
            file,
            line,
        );
    }
    ret = malloc(count);
    if ret.is_null() {
        (set_loc
            as unsafe extern "C" fn(
                *const libc::c_char,
                libc::c_int,
            ) -> ())(
            b"./awk.h\0" as *const u8 as *const libc::c_char,
            2056 as libc::c_int,
        );
        (Some(
            (Some(r_fatal as unsafe extern "C" fn(*const libc::c_char, ...) -> ()))
                .unwrap(),
        ))
            .unwrap()(
            dcgettext(
                0 as *const libc::c_char,
                b"%s:%d:%s: %s: cannot allocate %ld bytes of memory: %s\0" as *const u8
                    as *const libc::c_char,
                5 as libc::c_int,
            ),
            file,
            line,
            where_0,
            var,
            count as libc::c_long,
            strerror(*__errno_location()),
        );
    }
    return ret;
}
#[inline]
unsafe extern "C" fn ezalloc_real(
    mut count: size_t,
    mut where_0: *const libc::c_char,
    mut var: *const libc::c_char,
    mut file: *const libc::c_char,
    mut line: libc::c_int,
) -> *mut libc::c_void {
    let mut ret: *mut libc::c_void = 0 as *mut libc::c_void;
    if count == 0 as libc::c_int as libc::c_ulong {
        (set_loc
            as unsafe extern "C" fn(
                *const libc::c_char,
                libc::c_int,
            ) -> ())(
            b"./awk.h\0" as *const u8 as *const libc::c_char,
            2070 as libc::c_int,
        );
        (Some(
            (Some(r_fatal as unsafe extern "C" fn(*const libc::c_char, ...) -> ()))
                .unwrap(),
        ))
            .unwrap()(
            b"%s:%d: ezalloc called with zero bytes\0" as *const u8
                as *const libc::c_char,
            file,
            line,
        );
    }
    ret = calloc(1 as libc::c_int as libc::c_ulong, count);
    if ret.is_null() {
        (set_loc
            as unsafe extern "C" fn(
                *const libc::c_char,
                libc::c_int,
            ) -> ())(
            b"./awk.h\0" as *const u8 as *const libc::c_char,
            2074 as libc::c_int,
        );
        (Some(
            (Some(r_fatal as unsafe extern "C" fn(*const libc::c_char, ...) -> ()))
                .unwrap(),
        ))
            .unwrap()(
            dcgettext(
                0 as *const libc::c_char,
                b"%s:%d:%s: %s: cannot allocate %ld bytes of memory: %s\0" as *const u8
                    as *const libc::c_char,
                5 as libc::c_int,
            ),
            file,
            line,
            where_0,
            var,
            count as libc::c_long,
            strerror(*__errno_location()),
        );
    }
    return ret;
}
#[inline]
unsafe extern "C" fn boolval(mut t: *mut NODE) -> bool {
    if (*t).type_0 as libc::c_uint == Node_var as libc::c_int as libc::c_uint {
        t = (*t).sub.nodep.l.lptr;
    }
    fixtype(t);
    if (*t).flags as libc::c_uint & NUMBER as libc::c_int as libc::c_uint
        != 0 as libc::c_int as libc::c_uint
    {
        return !((*t).sub.val.fltnum == 0.0f64);
    }
    return (*t).sub.val.slen > 0 as libc::c_int as libc::c_ulong;
}
#[inline]
unsafe extern "C" fn POP_SCALAR() -> *mut NODE {
    let fresh1 = stack_ptr;
    stack_ptr = stack_ptr.offset(-1);
    let mut t: *mut NODE = (*fresh1).rptr;
    if (*t).type_0 as libc::c_uint == Node_var_array as libc::c_int as libc::c_uint {
        (set_loc
            as unsafe extern "C" fn(
                *const libc::c_char,
                libc::c_int,
            ) -> ())(
            b"./awk.h\0" as *const u8 as *const libc::c_char,
            1881 as libc::c_int,
        );
        (Some(
            (Some(r_fatal as unsafe extern "C" fn(*const libc::c_char, ...) -> ()))
                .unwrap(),
        ))
            .unwrap()(
            dcgettext(
                0 as *const libc::c_char,
                b"attempt to use array `%s' in a scalar context\0" as *const u8
                    as *const libc::c_char,
                5 as libc::c_int,
            ),
            array_vname(t),
        );
    } else if (*t).type_0 as libc::c_uint == Node_elem_new as libc::c_int as libc::c_uint
    {
        t = elem_new_to_scalar(t);
    }
    return t;
}
static mut matchrec: Option::<
    unsafe extern "C" fn(*mut IOBUF, *mut recmatch, *mut SCANSTATE) -> RECVALUE,
> = unsafe {
    Some(
        rs1scan
            as unsafe extern "C" fn(
                *mut IOBUF,
                *mut recmatch,
                *mut SCANSTATE,
            ) -> RECVALUE,
    )
};
static mut read_can_timeout: bool = 0 as libc::c_int != 0;
static mut read_timeout: libc::c_long = 0;
static mut read_default_timeout: libc::c_long = 0;
static mut red_head: *mut redirect = 0 as *const redirect as *mut redirect;
static mut RS: *mut NODE = 0 as *const NODE as *mut NODE;
static mut RS_re: [*mut Regexp; 2] = [0 as *const Regexp as *mut Regexp; 2];
static mut RS_regexp: *mut Regexp = 0 as *const Regexp as *mut Regexp;
static mut nonfatal: [libc::c_char; 9] = unsafe {
    *::std::mem::transmute::<&[u8; 9], &[libc::c_char; 9]>(b"NONFATAL\0")
};
pub static mut RS_is_null: bool = false;
pub unsafe extern "C" fn init_io() {
    let mut tmout: libc::c_long = 0;
    init_sockets();
    tmout = getenv_long(b"GAWK_READ_TIMEOUT\0" as *const u8 as *const libc::c_char);
    if tmout > 0 as libc::c_int as libc::c_long {
        read_default_timeout = tmout;
        read_can_timeout = 1 as libc::c_int != 0;
    }
    if !PROCINFO_node.is_null() {
        read_can_timeout = 1 as libc::c_int != 0;
    }
}
pub unsafe extern "C" fn after_beginfile(mut curfile: *mut *mut IOBUF) {
    let mut iop: *mut IOBUF = 0 as *mut IOBUF;
    iop = *curfile;
    find_input_parser(iop);
    if !(*iop).valid {
        let mut fname: *const libc::c_char = 0 as *const libc::c_char;
        let mut errcode: libc::c_int = 0;
        let mut valid: bool = false;
        fname = (*iop).public.name;
        errcode = (*iop).errcode;
        valid = (*iop).valid;
        *__errno_location() = 0 as libc::c_int;
        update_ERRNO_int(errcode);
        iop_close(iop);
        *curfile = 0 as *mut IOBUF;
        if !valid && errcode == 21 as libc::c_int
            && do_flags as libc::c_uint & DO_TRADITIONAL as libc::c_int as libc::c_uint
                == 0
        {
            (set_loc
                as unsafe extern "C" fn(
                    *const libc::c_char,
                    libc::c_int,
                ) -> ())(
                b"io.c\0" as *const u8 as *const libc::c_char,
                406 as libc::c_int,
            );
            (Some(
                (Some(r_warning as unsafe extern "C" fn(*const libc::c_char, ...) -> ()))
                    .unwrap(),
            ))
                .unwrap()(
                dcgettext(
                    0 as *const libc::c_char,
                    b"command line argument `%s' is a directory: skipped\0" as *const u8
                        as *const libc::c_char,
                    5 as libc::c_int,
                ),
                fname,
            );
            return;
        }
        (set_loc
            as unsafe extern "C" fn(
                *const libc::c_char,
                libc::c_int,
            ) -> ())(b"io.c\0" as *const u8 as *const libc::c_char, 409 as libc::c_int);
        (Some(
            (Some(r_fatal as unsafe extern "C" fn(*const libc::c_char, ...) -> ()))
                .unwrap(),
        ))
            .unwrap()(
            dcgettext(
                0 as *const libc::c_char,
                b"cannot open file `%s' for reading: %s\0" as *const u8
                    as *const libc::c_char,
                5 as libc::c_int,
            ),
            fname,
            strerror(errcode),
        );
    }
}
pub unsafe extern "C" fn nextfile(
    mut curfile: *mut *mut IOBUF,
    mut skipping: bool,
) -> libc::c_int {
    static mut i: libc::c_long = 1 as libc::c_int as libc::c_long;
    static mut files: bool = 0 as libc::c_int != 0;
    let mut arg: *mut NODE = 0 as *mut NODE;
    let mut tmp: *mut NODE = 0 as *mut NODE;
    let mut fname: *const libc::c_char = 0 as *const libc::c_char;
    let mut fd: libc::c_int = -(1 as libc::c_int);
    let mut errcode: libc::c_int = 0 as libc::c_int;
    let mut iop: *mut IOBUF = *curfile;
    if skipping {
        errcode = 0 as libc::c_int;
        if !iop.is_null() {
            errcode = (*iop).errcode;
            iop_close(iop);
        }
        *curfile = 0 as *mut IOBUF;
        return (errcode == 0 as libc::c_int) as libc::c_int;
    }
    if !iop.is_null() {
        if (*iop).flag as libc::c_uint & IOP_AT_EOF as libc::c_int as libc::c_uint
            != 0 as libc::c_int as libc::c_uint
        {
            iop_close(iop);
            *curfile = 0 as *mut IOBUF;
            return 1 as libc::c_int;
        } else {
            return 0 as libc::c_int
        }
    }
    while i < (*(*ARGC_node).sub.nodep.l.lptr).sub.val.fltnum as libc::c_long {
        tmp = make_number.unwrap()(i as libc::c_double);
        force_string_fmt(tmp, CONVFMT, CONVFMTidx);
        arg = in_array(ARGV_node, tmp);
        unref(tmp);
        if !(arg.is_null() || (*arg).sub.val.slen == 0 as libc::c_int as libc::c_ulong) {
            arg = force_string_fmt(arg, CONVFMT, CONVFMTidx);
            if do_flags as libc::c_uint & DO_TRADITIONAL as libc::c_int as libc::c_uint
                == 0
            {
                unref((*ARGIND_node).sub.nodep.l.lptr);
                (*ARGIND_node)
                    .sub
                    .nodep
                    .l
                    .lptr = make_number.unwrap()(i as libc::c_double);
            }
            if arg_assign((*arg).sub.val.sp, 0 as libc::c_int != 0) == 0 {
                files = 1 as libc::c_int != 0;
                fname = (*arg).sub.val.sp;
                unref((*FILENAME_node).sub.nodep.l.lptr);
                (*FILENAME_node).sub.nodep.l.lptr = dupnode(arg);
                FNR = 0 as libc::c_int as libc::c_long;
                *__errno_location() = 0 as libc::c_int;
                fd = devopen(fname, b"r\0" as *const u8 as *const libc::c_char);
                if fd == -(1 as libc::c_int) && *__errno_location() == 24 as libc::c_int
                {
                    close_one();
                    close_one();
                    fd = devopen(fname, b"r\0" as *const u8 as *const libc::c_char);
                }
                errcode = *__errno_location();
                if do_flags as libc::c_uint
                    & DO_TRADITIONAL as libc::c_int as libc::c_uint == 0
                {
                    update_ERRNO_int(*__errno_location());
                }
                iop = iop_alloc(fd, fname, errcode);
                *curfile = iop_finish(iop);
                if (*iop).public.fd == -(1 as libc::c_int) {
                    (*iop).errcode = errcode;
                } else if (*iop).valid {
                    (*iop).errcode = 0 as libc::c_int;
                }
                if do_flags as libc::c_uint
                    & DO_TRADITIONAL as libc::c_int as libc::c_uint == 0
                    && (*iop).errcode != 0 as libc::c_int
                {
                    update_ERRNO_int((*iop).errcode);
                }
                i += 1;
                return i as libc::c_int;
            }
        }
        i += 1;
        i;
    }
    if files as libc::c_int == 0 as libc::c_int {
        files = 1 as libc::c_int != 0;
        *__errno_location() = 0 as libc::c_int;
        if do_flags as libc::c_uint & DO_TRADITIONAL as libc::c_int as libc::c_uint == 0
        {
            update_ERRNO_int(*__errno_location());
        }
        unref((*FILENAME_node).sub.nodep.l.lptr);
        (*FILENAME_node)
            .sub
            .nodep
            .l
            .lptr = make_str_node(
            b"-\0" as *const u8 as *const libc::c_char,
            1 as libc::c_int as size_t,
            0 as libc::c_int,
        );
        (*(*FILENAME_node).sub.nodep.l.lptr)
            .flags = ::std::mem::transmute::<
            libc::c_uint,
            flagvals,
        >(
            (*(*FILENAME_node).sub.nodep.l.lptr).flags as libc::c_uint
                | USER_INPUT as libc::c_int as libc::c_uint,
        );
        fname = b"-\0" as *const u8 as *const libc::c_char;
        iop = iop_alloc(fileno(stdin), fname, 0 as libc::c_int);
        *curfile = iop_finish(iop);
        if (*iop).public.fd == -(1 as libc::c_int) {
            errcode = *__errno_location();
            *__errno_location() = 0 as libc::c_int;
            update_ERRNO_int(*__errno_location());
            iop_close(iop);
            *curfile = 0 as *mut IOBUF;
            (set_loc
                as unsafe extern "C" fn(
                    *const libc::c_char,
                    libc::c_int,
                ) -> ())(
                b"io.c\0" as *const u8 as *const libc::c_char,
                523 as libc::c_int,
            );
            (Some(
                (Some(r_fatal as unsafe extern "C" fn(*const libc::c_char, ...) -> ()))
                    .unwrap(),
            ))
                .unwrap()(
                dcgettext(
                    0 as *const libc::c_char,
                    b"cannot open file `%s' for reading: %s\0" as *const u8
                        as *const libc::c_char,
                    5 as libc::c_int,
                ),
                fname,
                strerror(errcode),
            );
        }
        i += 1;
        return i as libc::c_int;
    }
    return -(1 as libc::c_int);
}
pub unsafe extern "C" fn set_FNR() {
    let mut n: *mut NODE = (*FNR_node).sub.nodep.l.lptr;
    force_number(n);
    FNR = (*n).sub.val.fltnum as libc::c_long;
}
pub unsafe extern "C" fn set_NR() {
    let mut n: *mut NODE = (*NR_node).sub.nodep.l.lptr;
    force_number(n);
    NR = (*n).sub.val.fltnum as libc::c_long;
}
pub unsafe extern "C" fn inrec(
    mut iop: *mut IOBUF,
    mut errcode: *mut libc::c_int,
) -> bool {
    let mut begin: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut cnt: size_t = 0;
    let mut retval: bool = false;
    let mut field_width: *const awk_fieldwidth_info_t = 0
        as *const awk_fieldwidth_info_t;
    if (*iop).flag as libc::c_uint & IOP_AT_EOF as libc::c_int as libc::c_uint
        != 0 as libc::c_int as libc::c_uint && (*iop).off >= (*iop).dataend
    {
        retval = 0 as libc::c_int != 0;
    } else if (*iop).flag as libc::c_uint & IOP_CLOSED as libc::c_int as libc::c_uint
        != 0 as libc::c_int as libc::c_uint
    {
        retval = 0 as libc::c_int != 0;
    } else {
        retval = get_a_record(&mut begin, &mut cnt, iop, errcode, &mut field_width)
            == 0 as libc::c_int;
    }
    if retval {
        NR += 1;
        NR;
        FNR += 1;
        FNR;
        set_record(begin, cnt, field_width);
        if *errcode > 0 as libc::c_int {
            retval = 0 as libc::c_int != 0;
        }
    }
    return retval;
}
unsafe extern "C" fn remap_std_file(mut oldfd: libc::c_int) -> libc::c_int {
    let mut newfd: libc::c_int = 0;
    let mut ret: libc::c_int = -(1 as libc::c_int);
    newfd = os_devopen(
        b"/dev/null\0" as *const u8 as *const libc::c_char,
        0o2 as libc::c_int,
    );
    if newfd == -(1 as libc::c_int) {
        newfd = open(
            b"/dev/null\0" as *const u8 as *const libc::c_char,
            0o2 as libc::c_int,
        );
    }
    if newfd >= 0 as libc::c_int {
        ret = dup2(newfd, oldfd);
        close(newfd);
    } else {
        ret = 0 as libc::c_int;
    }
    return ret;
}
unsafe extern "C" fn iop_close(mut iop: *mut IOBUF) -> libc::c_int {
    let mut ret: libc::c_int = 0 as libc::c_int;
    if iop.is_null() {
        return 0 as libc::c_int;
    }
    *__errno_location() = 0 as libc::c_int;
    (*iop)
        .flag = ::std::mem::transmute::<
        libc::c_uint,
        iobuf_flags,
    >((*iop).flag as libc::c_uint & !(IOP_AT_EOF as libc::c_int) as libc::c_uint);
    (*iop)
        .flag = ::std::mem::transmute::<
        libc::c_uint,
        iobuf_flags,
    >((*iop).flag as libc::c_uint | IOP_CLOSED as libc::c_int as libc::c_uint);
    (*iop).dataend = 0 as *mut libc::c_char;
    if ((*iop).public.close_func).is_some() {
        ((*iop).public.close_func).unwrap()(&mut (*iop).public);
    }
    if (*iop).public.fd != -(1 as libc::c_int) {
        if (*iop).public.fd == fileno(stdin) || (*iop).public.fd == fileno(stdout)
            || (*iop).public.fd == fileno(stderr)
        {
            ret = remap_std_file((*iop).public.fd);
        } else {
            ret = close((*iop).public.fd);
        }
    }
    if ret == -(1 as libc::c_int) {
        (set_loc
            as unsafe extern "C" fn(
                *const libc::c_char,
                libc::c_int,
            ) -> ())(b"io.c\0" as *const u8 as *const libc::c_char, 650 as libc::c_int);
        (Some(
            (Some(r_warning as unsafe extern "C" fn(*const libc::c_char, ...) -> ()))
                .unwrap(),
        ))
            .unwrap()(
            dcgettext(
                0 as *const libc::c_char,
                b"close of fd %d (`%s') failed: %s\0" as *const u8
                    as *const libc::c_char,
                5 as libc::c_int,
            ),
            (*iop).public.fd,
            (*iop).public.name,
            strerror(*__errno_location()),
        );
    }
    if !((*iop).buf).is_null() {
        if (**fields_arr.offset(0 as libc::c_int as isize)).sub.val.sp >= (*iop).buf
            && (**fields_arr.offset(0 as libc::c_int as isize)).sub.val.sp
                < ((*iop).buf).offset((*iop).size as isize)
        {
            let mut t: *mut NODE = 0 as *mut NODE;
            t = make_str_node(
                (**fields_arr.offset(0 as libc::c_int as isize)).sub.val.sp,
                (**fields_arr.offset(0 as libc::c_int as isize)).sub.val.slen,
                0 as libc::c_int,
            );
            unref(*fields_arr.offset(0 as libc::c_int as isize));
            let ref mut fresh2 = *fields_arr.offset(0 as libc::c_int as isize);
            *fresh2 = t;
        }
        free((*iop).buf as *mut libc::c_void);
        (*iop).buf = 0 as *mut libc::c_char;
    }
    free(iop as *mut libc::c_void);
    return if ret == -(1 as libc::c_int) { 1 as libc::c_int } else { 0 as libc::c_int };
}
pub unsafe extern "C" fn redflags2str(mut flags: libc::c_int) -> *const libc::c_char {
    static mut redtab: [flagtab; 12] = [
        {
            let mut init = flagtab {
                val: RED_FILE as libc::c_int,
                name: b"RED_FILE\0" as *const u8 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = flagtab {
                val: RED_PIPE as libc::c_int,
                name: b"RED_PIPE\0" as *const u8 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = flagtab {
                val: RED_READ as libc::c_int,
                name: b"RED_READ\0" as *const u8 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = flagtab {
                val: RED_WRITE as libc::c_int,
                name: b"RED_WRITE\0" as *const u8 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = flagtab {
                val: RED_APPEND as libc::c_int,
                name: b"RED_APPEND\0" as *const u8 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = flagtab {
                val: RED_FLUSH as libc::c_int,
                name: b"RED_FLUSH\0" as *const u8 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = flagtab {
                val: RED_EOF as libc::c_int,
                name: b"RED_EOF\0" as *const u8 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = flagtab {
                val: RED_TWOWAY as libc::c_int,
                name: b"RED_TWOWAY\0" as *const u8 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = flagtab {
                val: RED_PTY as libc::c_int,
                name: b"RED_PTY\0" as *const u8 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = flagtab {
                val: RED_SOCKET as libc::c_int,
                name: b"RED_SOCKET\0" as *const u8 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = flagtab {
                val: RED_TCP as libc::c_int,
                name: b"RED_TCP\0" as *const u8 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = flagtab {
                val: 0 as libc::c_int,
                name: 0 as *const libc::c_char,
            };
            init
        },
    ];
    if flags == RED_NONE as libc::c_int {
        return b"RED_NONE\0" as *const u8 as *const libc::c_char;
    }
    return genflags2str(flags, redtab.as_ptr());
}
unsafe extern "C" fn check_duplicated_redirections(
    mut name: *const libc::c_char,
    mut len: size_t,
    mut oldflags: redirect_flags_t,
    mut newflags: redirect_flags_t,
) {
    static mut mixtures: [mixture; 11] = [
        {
            let mut init = mixture {
                common: RED_FILE,
                mode: RED_READ,
                other_mode: RED_WRITE,
                message: b"`%.*s' used for input file and for output file\0" as *const u8
                    as *const libc::c_char,
            };
            init
        },
        {
            let mut init = mixture {
                common: RED_READ,
                mode: RED_FILE,
                other_mode: RED_PIPE,
                message: b"`%.*s' used for input file and input pipe\0" as *const u8
                    as *const libc::c_char,
            };
            init
        },
        {
            let mut init = mixture {
                common: RED_READ,
                mode: RED_FILE,
                other_mode: RED_TWOWAY,
                message: b"`%.*s' used for input file and two-way pipe\0" as *const u8
                    as *const libc::c_char,
            };
            init
        },
        {
            let mut init = mixture {
                common: RED_NONE,
                mode: (RED_FILE as libc::c_int | RED_READ as libc::c_int)
                    as redirect_flags_t,
                other_mode: (RED_PIPE as libc::c_int | RED_WRITE as libc::c_int)
                    as redirect_flags_t,
                message: b"`%.*s' used for input file and output pipe\0" as *const u8
                    as *const libc::c_char,
            };
            init
        },
        {
            let mut init = mixture {
                common: (RED_FILE as libc::c_int | RED_WRITE as libc::c_int)
                    as redirect_flags_t,
                mode: (RED_FILE as libc::c_int | RED_WRITE as libc::c_int)
                    as redirect_flags_t,
                other_mode: RED_APPEND,
                message: b"unnecessary mixing of `>' and `>>' for file `%.*s'\0"
                    as *const u8 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = mixture {
                common: RED_NONE,
                mode: (RED_FILE as libc::c_int | RED_WRITE as libc::c_int)
                    as redirect_flags_t,
                other_mode: (RED_PIPE as libc::c_int | RED_READ as libc::c_int)
                    as redirect_flags_t,
                message: b"`%.*s' used for input pipe and output file\0" as *const u8
                    as *const libc::c_char,
            };
            init
        },
        {
            let mut init = mixture {
                common: RED_WRITE,
                mode: RED_FILE,
                other_mode: RED_PIPE,
                message: b"`%.*s' used for output file and output pipe\0" as *const u8
                    as *const libc::c_char,
            };
            init
        },
        {
            let mut init = mixture {
                common: RED_WRITE,
                mode: RED_FILE,
                other_mode: RED_TWOWAY,
                message: b"`%.*s' used for output file and two-way pipe\0" as *const u8
                    as *const libc::c_char,
            };
            init
        },
        {
            let mut init = mixture {
                common: RED_PIPE,
                mode: RED_READ,
                other_mode: RED_WRITE,
                message: b"`%.*s' used for input pipe and output pipe\0" as *const u8
                    as *const libc::c_char,
            };
            init
        },
        {
            let mut init = mixture {
                common: RED_READ,
                mode: RED_PIPE,
                other_mode: RED_TWOWAY,
                message: b"`%.*s' used for input pipe and two-way pipe\0" as *const u8
                    as *const libc::c_char,
            };
            init
        },
        {
            let mut init = mixture {
                common: RED_WRITE,
                mode: RED_PIPE,
                other_mode: RED_TWOWAY,
                message: b"`%.*s' used for output pipe and two-way pipe\0" as *const u8
                    as *const libc::c_char,
            };
            init
        },
    ];
    let mut i: libc::c_int = 0 as libc::c_int;
    let mut j: libc::c_int = (::std::mem::size_of::<[mixture; 11]>() as libc::c_ulong)
        .wrapping_div(::std::mem::size_of::<mixture>() as libc::c_ulong) as libc::c_int;
    oldflags = ::std::mem::transmute::<
        libc::c_uint,
        redirect_flags_t,
    >(
        oldflags as libc::c_uint
            & !(RED_FLUSH as libc::c_int | RED_EOF as libc::c_int
                | RED_PTY as libc::c_int) as libc::c_uint,
    );
    newflags = ::std::mem::transmute::<
        libc::c_uint,
        redirect_flags_t,
    >(
        newflags as libc::c_uint
            & !(RED_FLUSH as libc::c_int | RED_EOF as libc::c_int
                | RED_PTY as libc::c_int) as libc::c_uint,
    );
    i = 0 as libc::c_int;
    while i < j {
        let mut both_have_common: bool = oldflags as libc::c_uint
            & mixtures[i as usize].common as libc::c_uint
            == mixtures[i as usize].common as libc::c_uint
            && newflags as libc::c_uint & mixtures[i as usize].common as libc::c_uint
                == mixtures[i as usize].common as libc::c_uint;
        let mut old_has_mode: bool = oldflags as libc::c_uint
            & mixtures[i as usize].mode as libc::c_uint
            == mixtures[i as usize].mode as libc::c_uint;
        let mut new_has_mode: bool = newflags as libc::c_uint
            & mixtures[i as usize].mode as libc::c_uint
            == mixtures[i as usize].mode as libc::c_uint;
        let mut old_has_other_mode: bool = oldflags as libc::c_uint
            & mixtures[i as usize].other_mode as libc::c_uint
            == mixtures[i as usize].other_mode as libc::c_uint;
        let mut new_has_other_mode: bool = newflags as libc::c_uint
            & mixtures[i as usize].other_mode as libc::c_uint
            == mixtures[i as usize].other_mode as libc::c_uint;
        if both_have_common as libc::c_int != 0
            && oldflags as libc::c_uint != newflags as libc::c_uint
            && ((old_has_mode as libc::c_int != 0 || new_has_mode as libc::c_int != 0)
                && (old_has_other_mode as libc::c_int != 0
                    || new_has_other_mode as libc::c_int != 0))
        {
            (set_loc
                as unsafe extern "C" fn(
                    *const libc::c_char,
                    libc::c_int,
                ) -> ())(
                b"io.c\0" as *const u8 as *const libc::c_char,
                763 as libc::c_int,
            );
            (Some(lintfunc.unwrap()))
                .unwrap()(
                dcgettext(
                    0 as *const libc::c_char,
                    mixtures[i as usize].message,
                    5 as libc::c_int,
                ),
                len,
                name,
            );
            return;
        }
        i += 1;
        i;
    }
}
pub unsafe extern "C" fn redirect_string(
    mut str: *const libc::c_char,
    mut explen: size_t,
    mut not_string: bool,
    mut redirtype: libc::c_int,
    mut errflg: *mut libc::c_int,
    mut extfd: libc::c_int,
    mut failure_fatal: bool,
) -> *mut redirect {
    let mut rp: *mut redirect = 0 as *mut redirect;
    let mut tflag: redirect_flags_t = RED_NONE;
    let mut outflag: redirect_flags_t = RED_NONE;
    let mut direction: *const libc::c_char = b"to\0" as *const u8 as *const libc::c_char;
    let mut mode: *const libc::c_char = 0 as *const libc::c_char;
    let mut fd: libc::c_int = 0;
    let mut what: *const libc::c_char = 0 as *const libc::c_char;
    let mut new_rp: bool = 0 as libc::c_int != 0;
    let mut isi: inet_socket_info = inet_socket_info {
        family: 0,
        protocol: 0,
        localport: C2RustUnnamed_11 {
            offset: 0,
            len: 0,
        },
        remotehost: C2RustUnnamed_11 {
            offset: 0,
            len: 0,
        },
        remoteport: C2RustUnnamed_11 {
            offset: 0,
            len: 0,
        },
    };
    static mut save_rp: *mut redirect = 0 as *const redirect as *mut redirect;
    if do_flags as libc::c_uint & DO_SANDBOX as libc::c_int as libc::c_uint != 0 {
        (set_loc
            as unsafe extern "C" fn(
                *const libc::c_char,
                libc::c_int,
            ) -> ())(b"io.c\0" as *const u8 as *const libc::c_char, 791 as libc::c_int);
        (Some(
            (Some(r_fatal as unsafe extern "C" fn(*const libc::c_char, ...) -> ()))
                .unwrap(),
        ))
            .unwrap()(
            dcgettext(
                0 as *const libc::c_char,
                b"redirection not allowed in sandbox mode\0" as *const u8
                    as *const libc::c_char,
                5 as libc::c_int,
            ),
        );
    }
    let mut current_block_16: u64;
    match redirtype {
        2 => {
            tflag = RED_APPEND;
            current_block_16 = 14213223266661779562;
        }
        1 => {
            current_block_16 = 14213223266661779562;
        }
        3 => {
            tflag = (RED_PIPE as libc::c_int | RED_WRITE as libc::c_int)
                as redirect_flags_t;
            what = b"|\0" as *const u8 as *const libc::c_char;
            current_block_16 = 224731115979188411;
        }
        4 => {
            tflag = (RED_PIPE as libc::c_int | RED_READ as libc::c_int)
                as redirect_flags_t;
            what = b"|\0" as *const u8 as *const libc::c_char;
            current_block_16 = 224731115979188411;
        }
        5 => {
            tflag = (RED_FILE as libc::c_int | RED_READ as libc::c_int)
                as redirect_flags_t;
            what = b"<\0" as *const u8 as *const libc::c_char;
            current_block_16 = 224731115979188411;
        }
        6 => {
            tflag = (RED_READ as libc::c_int | RED_WRITE as libc::c_int
                | RED_TWOWAY as libc::c_int) as redirect_flags_t;
            what = b"|&\0" as *const u8 as *const libc::c_char;
            current_block_16 = 224731115979188411;
        }
        _ => {
            r_fatal(
                b"internal error: file %s, line %d: invalid redirection type %d\0"
                    as *const u8 as *const libc::c_char,
                b"io.c\0" as *const u8 as *const libc::c_char,
                822 as libc::c_int,
                redirtype,
            );
            current_block_16 = 224731115979188411;
        }
    }
    match current_block_16 {
        14213223266661779562 => {
            outflag = (RED_FILE as libc::c_int | RED_WRITE as libc::c_int)
                as redirect_flags_t;
            tflag = ::std::mem::transmute::<
                libc::c_uint,
                redirect_flags_t,
            >(tflag as libc::c_uint | outflag as libc::c_uint);
            if redirtype == redirect_output as libc::c_int {
                what = b">\0" as *const u8 as *const libc::c_char;
            } else {
                what = b">>\0" as *const u8 as *const libc::c_char;
            }
        }
        _ => {}
    }
    if do_flags as libc::c_uint
        & (DO_LINT_INVALID as libc::c_int | DO_LINT_ALL as libc::c_int) as libc::c_uint
        != 0 && not_string as libc::c_int != 0
    {
        (set_loc
            as unsafe extern "C" fn(
                *const libc::c_char,
                libc::c_int,
            ) -> ())(b"io.c\0" as *const u8 as *const libc::c_char, 825 as libc::c_int);
        (Some(lintfunc.unwrap()))
            .unwrap()(
            dcgettext(
                0 as *const libc::c_char,
                b"expression in `%s' redirection is a number\0" as *const u8
                    as *const libc::c_char,
                5 as libc::c_int,
            ),
            what,
        );
    }
    if explen < 1 as libc::c_int as libc::c_ulong || str.is_null()
        || *str as libc::c_int == '\0' as i32
    {
        (set_loc
            as unsafe extern "C" fn(
                *const libc::c_char,
                libc::c_int,
            ) -> ())(b"io.c\0" as *const u8 as *const libc::c_char, 829 as libc::c_int);
        (Some(
            (Some(r_fatal as unsafe extern "C" fn(*const libc::c_char, ...) -> ()))
                .unwrap(),
        ))
            .unwrap()(
            dcgettext(
                0 as *const libc::c_char,
                b"expression for `%s' redirection has null string value\0" as *const u8
                    as *const libc::c_char,
                5 as libc::c_int,
            ),
            what,
        );
    }
    if do_flags as libc::c_uint
        & (DO_LINT_INVALID as libc::c_int | DO_LINT_ALL as libc::c_int) as libc::c_uint
        != 0
        && (strncmp(str, b"0\0" as *const u8 as *const libc::c_char, explen)
            == 0 as libc::c_int
            || strncmp(str, b"1\0" as *const u8 as *const libc::c_char, explen)
                == 0 as libc::c_int)
    {
        (set_loc
            as unsafe extern "C" fn(
                *const libc::c_char,
                libc::c_int,
            ) -> ())(b"io.c\0" as *const u8 as *const libc::c_char, 834 as libc::c_int);
        (Some(lintfunc.unwrap()))
            .unwrap()(
            dcgettext(
                0 as *const libc::c_char,
                b"filename `%.*s' for `%s' redirection may be result of logical expression\0"
                    as *const u8 as *const libc::c_char,
                5 as libc::c_int,
            ),
            explen as libc::c_int,
            str,
            what,
        );
    }
    if inetfile(str, explen, &mut isi) {
        tflag = ::std::mem::transmute::<
            libc::c_uint,
            redirect_flags_t,
        >(tflag as libc::c_uint | RED_SOCKET as libc::c_int as libc::c_uint);
        if isi.protocol == SOCK_STREAM as libc::c_int {
            tflag = ::std::mem::transmute::<
                libc::c_uint,
                redirect_flags_t,
            >(tflag as libc::c_uint | RED_TCP as libc::c_int as libc::c_uint);
        }
    }
    rp = red_head;
    while !rp.is_null() {
        if (*rp).flag as libc::c_uint & RED_EOF as libc::c_int as libc::c_uint
            != 0 as libc::c_int as libc::c_uint
            && redirtype == redirect_pipein as libc::c_int
        {
            if (*rp).pid != -(1 as libc::c_int) {
                wait_any(0 as libc::c_int);
            }
        }
        if strlen((*rp).value) == explen
            && memcmp(
                (*rp).value as *const libc::c_void,
                str as *const libc::c_void,
                explen,
            ) == 0 as libc::c_int
        {
            if do_flags as libc::c_uint
                & (DO_LINT_INVALID as libc::c_int | DO_LINT_ALL as libc::c_int)
                    as libc::c_uint != 0
            {
                check_duplicated_redirections((*rp).value, explen, (*rp).flag, tflag);
            }
            if (*rp).flag as libc::c_uint
                & !(RED_FLUSH as libc::c_int | RED_EOF as libc::c_int
                    | RED_PTY as libc::c_int) as libc::c_uint == tflag as libc::c_uint
                || outflag as libc::c_uint != 0 as libc::c_int as libc::c_uint
                    && (*rp).flag as libc::c_uint
                        & (RED_FILE as libc::c_int | RED_WRITE as libc::c_int)
                            as libc::c_uint == outflag as libc::c_uint
            {
                break;
            }
        }
        rp = (*rp).next;
    }
    if rp.is_null() {
        let mut newstr: *mut libc::c_char = 0 as *mut libc::c_char;
        new_rp = 1 as libc::c_int != 0;
        if !save_rp.is_null() {
            rp = save_rp;
            free((*rp).value as *mut libc::c_void);
        } else {
            rp = emalloc_real(
                ::std::mem::size_of::<redirect>() as libc::c_ulong,
                b"redirect\0" as *const u8 as *const libc::c_char,
                b"rp\0" as *const u8 as *const libc::c_char,
                b"io.c\0" as *const u8 as *const libc::c_char,
                893 as libc::c_int,
            ) as *mut redirect;
        }
        newstr = emalloc_real(
            explen.wrapping_add(1 as libc::c_int as libc::c_ulong),
            b"redirect\0" as *const u8 as *const libc::c_char,
            b"newstr\0" as *const u8 as *const libc::c_char,
            b"io.c\0" as *const u8 as *const libc::c_char,
            894 as libc::c_int,
        ) as *mut libc::c_char;
        memcpy(newstr as *mut libc::c_void, str as *const libc::c_void, explen);
        *newstr.offset(explen as isize) = '\0' as i32 as libc::c_char;
        str = newstr;
        (*rp).value = newstr;
        (*rp).flag = tflag;
        init_output_wrapper(&mut (*rp).output);
        (*rp).output.name = str;
        (*rp).iop = 0 as *mut IOBUF;
        (*rp).pid = -(1 as libc::c_int);
        (*rp).status = 0 as libc::c_int;
    } else {
        str = (*rp).value;
    }
    save_rp = rp;
    while ((*rp).output.fp).is_null() && ((*rp).iop).is_null() {
        if !new_rp
            && (*rp).flag as libc::c_uint & RED_EOF as libc::c_int as libc::c_uint
                != 0 as libc::c_int as libc::c_uint
        {
            save_rp = 0 as *mut redirect;
            return rp;
        }
        mode = 0 as *const libc::c_char;
        *__errno_location() = 0 as libc::c_int;
        match redirtype {
            1 => {
                mode = b"w\0" as *const u8 as *const libc::c_char;
                if (*rp).flag as libc::c_uint & RED_USED as libc::c_int as libc::c_uint
                    != 0 as libc::c_int as libc::c_uint
                {
                    mode = if *((*rp).mode).offset(1 as libc::c_int as isize)
                        as libc::c_int == 'b' as i32
                    {
                        b"ab\0" as *const u8 as *const libc::c_char
                    } else {
                        b"a\0" as *const u8 as *const libc::c_char
                    };
                }
            }
            2 => {
                mode = b"a\0" as *const u8 as *const libc::c_char;
            }
            3 => {
                if extfd >= 0 as libc::c_int {
                    (set_loc
                        as unsafe extern "C" fn(
                            *const libc::c_char,
                            libc::c_int,
                        ) -> ())(
                        b"io.c\0" as *const u8 as *const libc::c_char,
                        931 as libc::c_int,
                    );
                    (Some(
                        (Some(
                            r_warning
                                as unsafe extern "C" fn(*const libc::c_char, ...) -> (),
                        ))
                            .unwrap(),
                    ))
                        .unwrap()(
                        dcgettext(
                            0 as *const libc::c_char,
                            b"get_file cannot create pipe `%s' with fd %d\0" as *const u8
                                as *const libc::c_char,
                            5 as libc::c_int,
                        ),
                        str,
                        extfd,
                    );
                    return 0 as *mut redirect;
                }
                flush_io();
                os_restore_mode(fileno(stdin));
                signal(13 as libc::c_int, None);
                (*rp).output.fp = popen(str, b"w\0" as *const u8 as *const libc::c_char);
                if ((*rp).output.fp).is_null() {
                    (set_loc
                        as unsafe extern "C" fn(
                            *const libc::c_char,
                            libc::c_int,
                        ) -> ())(
                        b"io.c\0" as *const u8 as *const libc::c_char,
                        946 as libc::c_int,
                    );
                    (Some(
                        (Some(
                            r_fatal
                                as unsafe extern "C" fn(*const libc::c_char, ...) -> (),
                        ))
                            .unwrap(),
                    ))
                        .unwrap()(
                        dcgettext(
                            0 as *const libc::c_char,
                            b"cannot open pipe `%s' for output: %s\0" as *const u8
                                as *const libc::c_char,
                            5 as libc::c_int,
                        ),
                        str,
                        strerror(*__errno_location()),
                    );
                }
                signal(
                    13 as libc::c_int,
                    ::std::mem::transmute::<
                        libc::intptr_t,
                        __sighandler_t,
                    >(1 as libc::c_int as libc::intptr_t),
                );
                os_close_on_exec(
                    fileno((*rp).output.fp),
                    str,
                    b"pipe\0" as *const u8 as *const libc::c_char,
                    b"to\0" as *const u8 as *const libc::c_char,
                );
                (*rp)
                    .flag = ::std::mem::transmute::<
                    libc::c_uint,
                    redirect_flags,
                >((*rp).flag as libc::c_uint | RED_FLUSH as libc::c_int as libc::c_uint);
            }
            4 => {
                if extfd >= 0 as libc::c_int {
                    (set_loc
                        as unsafe extern "C" fn(
                            *const libc::c_char,
                            libc::c_int,
                        ) -> ())(
                        b"io.c\0" as *const u8 as *const libc::c_char,
                        956 as libc::c_int,
                    );
                    (Some(
                        (Some(
                            r_warning
                                as unsafe extern "C" fn(*const libc::c_char, ...) -> (),
                        ))
                            .unwrap(),
                    ))
                        .unwrap()(
                        dcgettext(
                            0 as *const libc::c_char,
                            b"get_file cannot create pipe `%s' with fd %d\0" as *const u8
                                as *const libc::c_char,
                            5 as libc::c_int,
                        ),
                        str,
                        extfd,
                    );
                    return 0 as *mut redirect;
                }
                direction = b"from\0" as *const u8 as *const libc::c_char;
                if (gawk_popen(str, rp)).is_null() {
                    (set_loc
                        as unsafe extern "C" fn(
                            *const libc::c_char,
                            libc::c_int,
                        ) -> ())(
                        b"io.c\0" as *const u8 as *const libc::c_char,
                        961 as libc::c_int,
                    );
                    (Some(
                        (Some(
                            r_fatal
                                as unsafe extern "C" fn(*const libc::c_char, ...) -> (),
                        ))
                            .unwrap(),
                    ))
                        .unwrap()(
                        dcgettext(
                            0 as *const libc::c_char,
                            b"cannot open pipe `%s' for input: %s\0" as *const u8
                                as *const libc::c_char,
                            5 as libc::c_int,
                        ),
                        str,
                        strerror(*__errno_location()),
                    );
                }
            }
            5 => {
                direction = b"from\0" as *const u8 as *const libc::c_char;
                fd = if extfd >= 0 as libc::c_int {
                    extfd
                } else {
                    devopen(str, b"r\0" as *const u8 as *const libc::c_char)
                };
                if fd == -(1 as libc::c_int) && *__errno_location() == 21 as libc::c_int
                {
                    *errflg = 21 as libc::c_int;
                    return 0 as *mut redirect;
                }
                (*rp).iop = iop_alloc(fd, str, *__errno_location());
                find_input_parser((*rp).iop);
                iop_finish((*rp).iop);
                if !(*(*rp).iop).valid {
                    if do_flags as libc::c_uint
                        & DO_TRADITIONAL as libc::c_int as libc::c_uint == 0
                        && (*(*rp).iop).errcode != 0 as libc::c_int
                    {
                        update_ERRNO_int((*(*rp).iop).errcode);
                    }
                    iop_close((*rp).iop);
                    (*rp).iop = 0 as *mut IOBUF;
                }
            }
            6 => {
                direction = b"to/from\0" as *const u8 as *const libc::c_char;
                if two_way_open(str, rp, extfd) == 0 {
                    if !failure_fatal
                        || is_non_fatal_redirect(str, explen) as libc::c_int != 0
                    {
                        *errflg = *__errno_location();
                        return 0 as *mut redirect;
                    } else {
                        (set_loc
                            as unsafe extern "C" fn(
                                *const libc::c_char,
                                libc::c_int,
                            ) -> ())(
                            b"io.c\0" as *const u8 as *const libc::c_char,
                            996 as libc::c_int,
                        );
                        (Some(
                            (Some(
                                r_fatal
                                    as unsafe extern "C" fn(*const libc::c_char, ...) -> (),
                            ))
                                .unwrap(),
                        ))
                            .unwrap()(
                            dcgettext(
                                0 as *const libc::c_char,
                                b"cannot open two way pipe `%s' for input/output: %s\0"
                                    as *const u8 as *const libc::c_char,
                                5 as libc::c_int,
                            ),
                            str,
                            strerror(*__errno_location()),
                        );
                    }
                }
            }
            _ => {
                r_fatal(
                    b"internal error: file %s, line %d: invalid redirection type %d\0"
                        as *const u8 as *const libc::c_char,
                    b"io.c\0" as *const u8 as *const libc::c_char,
                    1001 as libc::c_int,
                    redirtype,
                );
            }
        }
        if !mode.is_null() {
            *__errno_location() = 0 as libc::c_int;
            (*rp).output.mode = mode;
            fd = if extfd >= 0 as libc::c_int { extfd } else { devopen(str, mode) };
            if fd > -(1 as libc::c_int) {
                if fd == fileno(stdin) {
                    (*rp).output.fp = stdin;
                } else if fd == fileno(stdout) {
                    (*rp).output.fp = stdout;
                } else if fd == fileno(stderr) {
                    (*rp).output.fp = stderr;
                } else {
                    let mut omode: *const libc::c_char = mode;
                    let mut fd_flags: libc::c_int = 0;
                    fd_flags = fcntl(fd, 3 as libc::c_int);
                    if fd_flags != -(1 as libc::c_int)
                        && fd_flags & 0o2000 as libc::c_int == 0o2000 as libc::c_int
                    {
                        omode = b"a\0" as *const u8 as *const libc::c_char;
                    }
                    os_close_on_exec(
                        fd,
                        str,
                        b"file\0" as *const u8 as *const libc::c_char,
                        b"\0" as *const u8 as *const libc::c_char,
                    );
                    (*rp).output.fp = fdopen(fd, omode);
                    (*rp).mode = mode;
                    if ((*rp).output.fp).is_null() {
                        close(fd);
                    }
                }
                if !((*rp).output.fp).is_null() && os_isatty(fd) != 0 {
                    (*rp)
                        .flag = ::std::mem::transmute::<
                        libc::c_uint,
                        redirect_flags,
                    >(
                        (*rp).flag as libc::c_uint
                            | RED_FLUSH as libc::c_int as libc::c_uint,
                    );
                }
                if !new_rp && red_head != rp {
                    (*(*rp).prev).next = (*rp).next;
                    if !((*(*rp).prev).next).is_null() {
                        (*(*rp).next).prev = (*rp).prev;
                    }
                    (*red_head).prev = rp;
                    (*rp).prev = 0 as *mut redirect;
                    (*rp).next = red_head;
                    red_head = rp;
                }
            }
            find_output_wrapper(&mut (*rp).output);
        }
        if ((*rp).output.fp).is_null() && ((*rp).iop).is_null() {
            if *__errno_location() == 24 as libc::c_int
                || *__errno_location() == 23 as libc::c_int
            {
                close_one();
            } else {
                if !errflg.is_null() {
                    *errflg = *__errno_location();
                }
                if failure_fatal as libc::c_int != 0
                    && !is_non_fatal_redirect(str, explen)
                    && (redirtype == redirect_output as libc::c_int
                        || redirtype == redirect_append as libc::c_int)
                {
                    if *direction as libc::c_int == 'f' as i32 {
                        (set_loc
                            as unsafe extern "C" fn(
                                *const libc::c_char,
                                libc::c_int,
                            ) -> ())(
                            b"io.c\0" as *const u8 as *const libc::c_char,
                            1083 as libc::c_int,
                        );
                        (Some(
                            (Some(
                                r_fatal
                                    as unsafe extern "C" fn(*const libc::c_char, ...) -> (),
                            ))
                                .unwrap(),
                        ))
                            .unwrap()(
                            dcgettext(
                                0 as *const libc::c_char,
                                b"cannot redirect from `%s': %s\0" as *const u8
                                    as *const libc::c_char,
                                5 as libc::c_int,
                            ),
                            str,
                            strerror(*__errno_location()),
                        );
                    } else {
                        (set_loc
                            as unsafe extern "C" fn(
                                *const libc::c_char,
                                libc::c_int,
                            ) -> ())(
                            b"io.c\0" as *const u8 as *const libc::c_char,
                            1086 as libc::c_int,
                        );
                        (Some(
                            (Some(
                                r_fatal
                                    as unsafe extern "C" fn(*const libc::c_char, ...) -> (),
                            ))
                                .unwrap(),
                        ))
                            .unwrap()(
                            dcgettext(
                                0 as *const libc::c_char,
                                b"cannot redirect to `%s': %s\0" as *const u8
                                    as *const libc::c_char,
                                5 as libc::c_int,
                            ),
                            str,
                            strerror(*__errno_location()),
                        );
                    }
                } else {
                    return 0 as *mut redirect
                }
            }
        }
    }
    if new_rp {
        if !red_head.is_null() {
            (*red_head).prev = rp;
        }
        (*rp).prev = 0 as *mut redirect;
        (*rp).next = red_head;
        red_head = rp;
    }
    save_rp = 0 as *mut redirect;
    return rp;
}
pub unsafe extern "C" fn redirect(
    mut redir_exp: *mut NODE,
    mut redirtype: libc::c_int,
    mut errflg: *mut libc::c_int,
    mut failure_fatal: bool,
) -> *mut redirect {
    let mut not_string: bool = (*fixtype(redir_exp)).flags as libc::c_uint
        & STRING as libc::c_int as libc::c_uint == 0 as libc::c_int as libc::c_uint;
    redir_exp = force_string_fmt(redir_exp, CONVFMT, CONVFMTidx);
    return redirect_string(
        (*redir_exp).sub.val.sp,
        (*redir_exp).sub.val.slen,
        not_string,
        redirtype,
        errflg,
        -(1 as libc::c_int),
        failure_fatal,
    );
}
pub unsafe extern "C" fn getredirect(
    mut str: *const libc::c_char,
    mut len: libc::c_int,
) -> *mut redirect {
    let mut rp: *mut redirect = 0 as *mut redirect;
    rp = red_head;
    while !rp.is_null() {
        if strlen((*rp).value) == len as libc::c_ulong
            && memcmp(
                (*rp).value as *const libc::c_void,
                str as *const libc::c_void,
                len as libc::c_ulong,
            ) == 0 as libc::c_int
        {
            return rp;
        }
        rp = (*rp).next;
    }
    return 0 as *mut redirect;
}
pub unsafe extern "C" fn is_non_fatal_std(mut fp: *mut FILE) -> bool {
    if !(in_PROCINFO(nonfatal.as_ptr(), 0 as *const libc::c_char, 0 as *mut *mut NODE))
        .is_null()
    {
        return 1 as libc::c_int != 0;
    }
    if fp == stdout {
        return !(in_PROCINFO(
            b"-\0" as *const u8 as *const libc::c_char,
            nonfatal.as_ptr(),
            0 as *mut *mut NODE,
        ))
            .is_null()
            || !(in_PROCINFO(
                b"/dev/stdout\0" as *const u8 as *const libc::c_char,
                nonfatal.as_ptr(),
                0 as *mut *mut NODE,
            ))
                .is_null()
    } else if fp == stderr {
        return !(in_PROCINFO(
            b"/dev/stderr\0" as *const u8 as *const libc::c_char,
            nonfatal.as_ptr(),
            0 as *mut *mut NODE,
        ))
            .is_null()
    }
    return 0 as libc::c_int != 0;
}
pub unsafe extern "C" fn is_non_fatal_redirect(
    mut str: *const libc::c_char,
    mut len: size_t,
) -> bool {
    let mut ret: bool = false;
    let mut save: libc::c_char = 0;
    let mut s: *mut libc::c_char = str as *mut libc::c_char;
    save = *s.offset(len as isize);
    *s.offset(len as isize) = '\0' as i32 as libc::c_char;
    ret = !(in_PROCINFO(
        nonfatal.as_ptr(),
        0 as *const libc::c_char,
        0 as *mut *mut NODE,
    ))
        .is_null()
        || !(in_PROCINFO(s, nonfatal.as_ptr(), 0 as *mut *mut NODE)).is_null();
    *s.offset(len as isize) = save;
    return ret;
}
unsafe extern "C" fn close_one() {
    let mut rp: *mut redirect = 0 as *mut redirect;
    let mut rplast: *mut redirect = 0 as *mut redirect;
    static mut warned: bool = 0 as libc::c_int != 0;
    if do_flags as libc::c_uint
        & (DO_LINT_INVALID as libc::c_int | DO_LINT_ALL as libc::c_int) as libc::c_uint
        != 0 && !warned
    {
        warned = 1 as libc::c_int != 0;
        (set_loc
            as unsafe extern "C" fn(
                *const libc::c_char,
                libc::c_int,
            ) -> ())(b"io.c\0" as *const u8 as *const libc::c_char, 1188 as libc::c_int);
        (Some(lintfunc.unwrap()))
            .unwrap()(
            dcgettext(
                0 as *const libc::c_char,
                b"reached system limit for open files: starting to multiplex file descriptors\0"
                    as *const u8 as *const libc::c_char,
                5 as libc::c_int,
            ),
        );
    }
    rp = red_head;
    while !rp.is_null() {
        rplast = rp;
        rp = (*rp).next;
    }
    rp = rplast;
    while !rp.is_null() {
        if !(((*rp).output.fp).is_null() || (*rp).output.fp == stderr
            || (*rp).output.fp == stdout)
        {
            if (*rp).flag as libc::c_uint
                & (RED_FILE as libc::c_int | RED_WRITE as libc::c_int) as libc::c_uint
                == (RED_FILE as libc::c_int | RED_WRITE as libc::c_int) as libc::c_uint
            {
                (*rp)
                    .flag = ::std::mem::transmute::<
                    libc::c_uint,
                    redirect_flags,
                >((*rp).flag as libc::c_uint | RED_USED as libc::c_int as libc::c_uint);
                *__errno_location() = 0 as libc::c_int;
                if ((*rp).output.gawk_fclose)
                    .unwrap()((*rp).output.fp, (*rp).output.opaque) != 0 as libc::c_int
                {
                    (set_loc
                        as unsafe extern "C" fn(
                            *const libc::c_char,
                            libc::c_int,
                        ) -> ())(
                        b"io.c\0" as *const u8 as *const libc::c_char,
                        1204 as libc::c_int,
                    );
                    (Some(
                        (Some(
                            r_warning
                                as unsafe extern "C" fn(*const libc::c_char, ...) -> (),
                        ))
                            .unwrap(),
                    ))
                        .unwrap()(
                        dcgettext(
                            0 as *const libc::c_char,
                            b"close of `%s' failed: %s\0" as *const u8
                                as *const libc::c_char,
                            5 as libc::c_int,
                        ),
                        (*rp).value,
                        strerror(*__errno_location()),
                    );
                }
                (*rp).output.fp = 0 as *mut FILE;
                break;
            }
        }
        rp = (*rp).prev;
    }
    if rp.is_null() {
        (set_loc
            as unsafe extern "C" fn(
                *const libc::c_char,
                libc::c_int,
            ) -> ())(b"io.c\0" as *const u8 as *const libc::c_char, 1212 as libc::c_int);
        (Some(
            (Some(r_fatal as unsafe extern "C" fn(*const libc::c_char, ...) -> ()))
                .unwrap(),
        ))
            .unwrap()(
            dcgettext(
                0 as *const libc::c_char,
                b"too many pipes or input files open\0" as *const u8
                    as *const libc::c_char,
                5 as libc::c_int,
            ),
        );
    }
}
pub unsafe extern "C" fn do_close(mut nargs: libc::c_int) -> *mut NODE {
    let mut tmp: *mut NODE = 0 as *mut NODE;
    let mut tmp2: *mut NODE = 0 as *mut NODE;
    let mut rp: *mut redirect = 0 as *mut redirect;
    let mut how: two_way_close_type = CLOSE_ALL;
    if nargs == 2 as libc::c_int {
        let mut save: libc::c_char = 0;
        tmp2 = force_string_fmt(POP_SCALAR(), CONVFMT, CONVFMTidx);
        save = *((*tmp2).sub.val.sp).offset((*tmp2).sub.val.slen as isize);
        *((*tmp2).sub.val.sp)
            .offset((*tmp2).sub.val.slen as isize) = '\0' as i32 as libc::c_char;
        if strcasecmp((*tmp2).sub.val.sp, b"to\0" as *const u8 as *const libc::c_char)
            == 0 as libc::c_int
        {
            how = CLOSE_TO;
        } else if strcasecmp(
            (*tmp2).sub.val.sp,
            b"from\0" as *const u8 as *const libc::c_char,
        ) == 0 as libc::c_int
        {
            how = CLOSE_FROM;
        } else {
            DEREF(tmp2);
            (set_loc
                as unsafe extern "C" fn(
                    *const libc::c_char,
                    libc::c_int,
                ) -> ())(
                b"io.c\0" as *const u8 as *const libc::c_char,
                1238 as libc::c_int,
            );
            (Some(
                (Some(r_fatal as unsafe extern "C" fn(*const libc::c_char, ...) -> ()))
                    .unwrap(),
            ))
                .unwrap()(
                dcgettext(
                    0 as *const libc::c_char,
                    b"close: second argument must be `to' or `from'\0" as *const u8
                        as *const libc::c_char,
                    5 as libc::c_int,
                ),
            );
        }
        *((*tmp2).sub.val.sp).offset((*tmp2).sub.val.slen as isize) = save;
        DEREF(tmp2);
    }
    tmp = force_string_fmt(POP_SCALAR(), CONVFMT, CONVFMTidx);
    rp = red_head;
    while !rp.is_null() {
        if strlen((*rp).value) == (*tmp).sub.val.slen
            && memcmp(
                (*rp).value as *const libc::c_void,
                (*tmp).sub.val.sp as *const libc::c_void,
                (*tmp).sub.val.slen,
            ) == 0 as libc::c_int
        {
            break;
        }
        rp = (*rp).next;
    }
    if rp.is_null() {
        let mut cp: *mut libc::c_char = 0 as *mut libc::c_char;
        if do_flags as libc::c_uint
            & (DO_LINT_INVALID as libc::c_int | DO_LINT_ALL as libc::c_int)
                as libc::c_uint != 0
        {
            (set_loc
                as unsafe extern "C" fn(
                    *const libc::c_char,
                    libc::c_int,
                ) -> ())(
                b"io.c\0" as *const u8 as *const libc::c_char,
                1256 as libc::c_int,
            );
            (Some(lintfunc.unwrap()))
                .unwrap()(
                dcgettext(
                    0 as *const libc::c_char,
                    b"close: `%.*s' is not an open file, pipe or co-process\0"
                        as *const u8 as *const libc::c_char,
                    5 as libc::c_int,
                ),
                (*tmp).sub.val.slen as libc::c_int,
                (*tmp).sub.val.sp,
            );
        }
        if do_flags as libc::c_uint & DO_TRADITIONAL as libc::c_int as libc::c_uint == 0
        {
            cp = dcgettext(
                0 as *const libc::c_char,
                b"close of redirection that was never opened\0" as *const u8
                    as *const libc::c_char,
                5 as libc::c_int,
            );
            update_ERRNO_string(cp);
        }
        DEREF(tmp);
        return make_number.unwrap()(-1.0f64);
    }
    DEREF(tmp);
    fflush(stdout);
    tmp = make_number
        .unwrap()(close_redir(rp, 0 as libc::c_int != 0, how) as libc::c_double);
    rp = 0 as *mut redirect;
    if do_flags as libc::c_uint & DO_POSIX as libc::c_int as libc::c_uint != 0 {
        unref(tmp);
        tmp = make_number.unwrap()(0 as libc::c_int as libc::c_double);
    }
    return tmp;
}
pub unsafe extern "C" fn close_rp(
    mut rp: *mut redirect,
    mut how: two_way_close_type,
) -> libc::c_int {
    let mut status: libc::c_int = 0 as libc::c_int;
    *__errno_location() = 0 as libc::c_int;
    if (*rp).flag as libc::c_uint & RED_TWOWAY as libc::c_int as libc::c_uint
        != 0 as libc::c_int as libc::c_uint
    {
        if (how as libc::c_uint == CLOSE_ALL as libc::c_int as libc::c_uint
            || how as libc::c_uint == CLOSE_TO as libc::c_int as libc::c_uint)
            && !((*rp).output.fp).is_null()
        {
            if (*rp).flag as libc::c_uint & RED_TCP as libc::c_int as libc::c_uint
                != 0 as libc::c_int as libc::c_uint
            {
                shutdown(fileno((*rp).output.fp), SHUT_WR as libc::c_int);
            }
            if (*rp).flag as libc::c_uint & RED_PTY as libc::c_int as libc::c_uint
                != 0 as libc::c_int as libc::c_uint
            {
                ((*rp).output.gawk_fwrite)
                    .unwrap()(
                    b"\x04\n\0" as *const u8 as *const libc::c_char
                        as *const libc::c_void,
                    (::std::mem::size_of::<[libc::c_char; 3]>() as libc::c_ulong)
                        .wrapping_sub(1 as libc::c_int as libc::c_ulong),
                    1 as libc::c_int as size_t,
                    (*rp).output.fp,
                    (*rp).output.opaque,
                );
                ((*rp).output.gawk_fflush)
                    .unwrap()((*rp).output.fp, (*rp).output.opaque);
            }
            status = ((*rp).output.gawk_fclose)
                .unwrap()((*rp).output.fp, (*rp).output.opaque);
            (*rp).output.fp = 0 as *mut FILE;
        }
        if how as libc::c_uint == CLOSE_ALL as libc::c_int as libc::c_uint
            || how as libc::c_uint == CLOSE_FROM as libc::c_int as libc::c_uint
        {
            if (*rp).flag as libc::c_uint & RED_SOCKET as libc::c_int as libc::c_uint
                != 0 as libc::c_int as libc::c_uint && !((*rp).iop).is_null()
            {
                if (*rp).flag as libc::c_uint & RED_TCP as libc::c_int as libc::c_uint
                    != 0 as libc::c_int as libc::c_uint
                {
                    shutdown((*(*rp).iop).public.fd, SHUT_RD as libc::c_int);
                }
                iop_close((*rp).iop);
            } else {
                status = gawk_pclose(rp);
            }
            (*rp).iop = 0 as *mut IOBUF;
        }
    } else if (*rp).flag as libc::c_uint
        & (RED_PIPE as libc::c_int | RED_WRITE as libc::c_int) as libc::c_uint
        == (RED_PIPE as libc::c_int | RED_WRITE as libc::c_int) as libc::c_uint
    {
        status = sanitize_exit_status(pclose((*rp).output.fp));
        if BINMODE & BINMODE_INPUT as libc::c_int != 0 as libc::c_int {
            os_setbinmode(fileno(stdin), 0 as libc::c_int);
        }
        (*rp).output.fp = 0 as *mut FILE;
    } else if !((*rp).output.fp).is_null() {
        status = ((*rp).output.gawk_fclose)
            .unwrap()((*rp).output.fp, (*rp).output.opaque);
        (*rp).output.fp = 0 as *mut FILE;
    } else if !((*rp).iop).is_null() {
        if (*rp).flag as libc::c_uint & RED_PIPE as libc::c_int as libc::c_uint
            != 0 as libc::c_int as libc::c_uint
        {
            status = gawk_pclose(rp);
        } else {
            status = iop_close((*rp).iop);
            (*rp).iop = 0 as *mut IOBUF;
        }
    }
    return status;
}
unsafe extern "C" fn close_redir(
    mut rp: *mut redirect,
    mut exitwarn: bool,
    mut how: two_way_close_type,
) -> libc::c_int {
    let mut status: libc::c_int = 0 as libc::c_int;
    if rp.is_null() {
        return 0 as libc::c_int;
    }
    if (*rp).flag as libc::c_uint & RED_WRITE as libc::c_int as libc::c_uint != 0
        && !((*rp).output.fp).is_null()
    {
        efflush((*rp).output.fp, b"flush\0" as *const u8 as *const libc::c_char, rp);
    }
    if !((*rp).output.fp == stdout || (*rp).output.fp == stderr) {
        if do_flags as libc::c_uint
            & (DO_LINT_INVALID as libc::c_int | DO_LINT_ALL as libc::c_int)
                as libc::c_uint != 0
            && (*rp).flag as libc::c_uint & RED_TWOWAY as libc::c_int as libc::c_uint
                == 0 as libc::c_int as libc::c_uint
            && how as libc::c_uint != CLOSE_ALL as libc::c_int as libc::c_uint
        {
            (set_loc
                as unsafe extern "C" fn(
                    *const libc::c_char,
                    libc::c_int,
                ) -> ())(
                b"io.c\0" as *const u8 as *const libc::c_char,
                1363 as libc::c_int,
            );
            (Some(lintfunc.unwrap()))
                .unwrap()(
                dcgettext(
                    0 as *const libc::c_char,
                    b"close: redirection `%s' not opened with `|&', second argument ignored\0"
                        as *const u8 as *const libc::c_char,
                    5 as libc::c_int,
                ),
                (*rp).value,
            );
        }
        status = close_rp(rp, how);
        if status != 0 as libc::c_int {
            let mut save_errno: libc::c_int = *__errno_location();
            let mut s: *mut libc::c_char = strerror(save_errno);
            if do_flags as libc::c_uint
                & (DO_LINT_INVALID as libc::c_int | DO_LINT_ALL as libc::c_int)
                    as libc::c_uint != 0
            {
                if (*rp).flag as libc::c_uint & RED_PIPE as libc::c_int as libc::c_uint
                    != 0 as libc::c_int as libc::c_uint
                {
                    (set_loc
                        as unsafe extern "C" fn(
                            *const libc::c_char,
                            libc::c_int,
                        ) -> ())(
                        b"io.c\0" as *const u8 as *const libc::c_char,
                        1380 as libc::c_int,
                    );
                    (Some(lintfunc.unwrap()))
                        .unwrap()(
                        dcgettext(
                            0 as *const libc::c_char,
                            b"failure status (%d) on pipe close of `%s': %s\0"
                                as *const u8 as *const libc::c_char,
                            5 as libc::c_int,
                        ),
                        status,
                        (*rp).value,
                        s,
                    );
                } else if (*rp).flag as libc::c_uint
                    & RED_TWOWAY as libc::c_int as libc::c_uint
                    != 0 as libc::c_int as libc::c_uint
                {
                    (set_loc
                        as unsafe extern "C" fn(
                            *const libc::c_char,
                            libc::c_int,
                        ) -> ())(
                        b"io.c\0" as *const u8 as *const libc::c_char,
                        1383 as libc::c_int,
                    );
                    (Some(lintfunc.unwrap()))
                        .unwrap()(
                        dcgettext(
                            0 as *const libc::c_char,
                            b"failure status (%d) on two-way pipe close of `%s': %s\0"
                                as *const u8 as *const libc::c_char,
                            5 as libc::c_int,
                        ),
                        status,
                        (*rp).value,
                        s,
                    );
                } else {
                    (set_loc
                        as unsafe extern "C" fn(
                            *const libc::c_char,
                            libc::c_int,
                        ) -> ())(
                        b"io.c\0" as *const u8 as *const libc::c_char,
                        1386 as libc::c_int,
                    );
                    (Some(lintfunc.unwrap()))
                        .unwrap()(
                        dcgettext(
                            0 as *const libc::c_char,
                            b"failure status (%d) on file close of `%s': %s\0"
                                as *const u8 as *const libc::c_char,
                            5 as libc::c_int,
                        ),
                        status,
                        (*rp).value,
                        s,
                    );
                }
            }
            if do_flags as libc::c_uint & DO_TRADITIONAL as libc::c_int as libc::c_uint
                == 0
            {
                update_ERRNO_int(save_errno);
            }
        }
    }
    if exitwarn {
        if (*rp).flag as libc::c_uint & RED_SOCKET as libc::c_int as libc::c_uint
            != 0 as libc::c_int as libc::c_uint
        {
            (set_loc
                as unsafe extern "C" fn(
                    *const libc::c_char,
                    libc::c_int,
                ) -> ())(
                b"io.c\0" as *const u8 as *const libc::c_char,
                1406 as libc::c_int,
            );
            (Some(
                (Some(r_warning as unsafe extern "C" fn(*const libc::c_char, ...) -> ()))
                    .unwrap(),
            ))
                .unwrap()(
                dcgettext(
                    0 as *const libc::c_char,
                    b"no explicit close of socket `%s' provided\0" as *const u8
                        as *const libc::c_char,
                    5 as libc::c_int,
                ),
                (*rp).value,
            );
        } else if (*rp).flag as libc::c_uint & RED_TWOWAY as libc::c_int as libc::c_uint
            != 0 as libc::c_int as libc::c_uint
        {
            (set_loc
                as unsafe extern "C" fn(
                    *const libc::c_char,
                    libc::c_int,
                ) -> ())(
                b"io.c\0" as *const u8 as *const libc::c_char,
                1409 as libc::c_int,
            );
            (Some(
                (Some(r_warning as unsafe extern "C" fn(*const libc::c_char, ...) -> ()))
                    .unwrap(),
            ))
                .unwrap()(
                dcgettext(
                    0 as *const libc::c_char,
                    b"no explicit close of co-process `%s' provided\0" as *const u8
                        as *const libc::c_char,
                    5 as libc::c_int,
                ),
                (*rp).value,
            );
        } else if (*rp).flag as libc::c_uint & RED_PIPE as libc::c_int as libc::c_uint
            != 0 as libc::c_int as libc::c_uint
        {
            (set_loc
                as unsafe extern "C" fn(
                    *const libc::c_char,
                    libc::c_int,
                ) -> ())(
                b"io.c\0" as *const u8 as *const libc::c_char,
                1412 as libc::c_int,
            );
            (Some(
                (Some(r_warning as unsafe extern "C" fn(*const libc::c_char, ...) -> ()))
                    .unwrap(),
            ))
                .unwrap()(
                dcgettext(
                    0 as *const libc::c_char,
                    b"no explicit close of pipe `%s' provided\0" as *const u8
                        as *const libc::c_char,
                    5 as libc::c_int,
                ),
                (*rp).value,
            );
        } else {
            (set_loc
                as unsafe extern "C" fn(
                    *const libc::c_char,
                    libc::c_int,
                ) -> ())(
                b"io.c\0" as *const u8 as *const libc::c_char,
                1415 as libc::c_int,
            );
            (Some(
                (Some(r_warning as unsafe extern "C" fn(*const libc::c_char, ...) -> ()))
                    .unwrap(),
            ))
                .unwrap()(
                dcgettext(
                    0 as *const libc::c_char,
                    b"no explicit close of file `%s' provided\0" as *const u8
                        as *const libc::c_char,
                    5 as libc::c_int,
                ),
                (*rp).value,
            );
        }
    }
    if how as libc::c_uint == CLOSE_ALL as libc::c_int as libc::c_uint
        || ((*rp).iop).is_null() && ((*rp).output.fp).is_null()
    {
        if !((*rp).next).is_null() {
            (*(*rp).next).prev = (*rp).prev;
        }
        if !((*rp).prev).is_null() {
            (*(*rp).prev).next = (*rp).next;
        } else {
            red_head = (*rp).next;
        }
        free_rp(rp);
    }
    return status;
}
pub unsafe extern "C" fn non_fatal_flush_std_file(mut fp: *mut FILE) -> bool {
    let mut status: libc::c_int = fflush(fp);
    if status != 0 as libc::c_int {
        let mut is_fatal: bool = !is_non_fatal_std(fp);
        if is_fatal {
            os_maybe_set_errno();
            if *__errno_location() == 32 as libc::c_int {
                signal(13 as libc::c_int, None);
                kill(getpid(), 13 as libc::c_int);
            } else {
                (set_loc
                    as unsafe extern "C" fn(
                        *const libc::c_char,
                        libc::c_int,
                    ) -> ())(
                    b"io.c\0" as *const u8 as *const libc::c_char,
                    1449 as libc::c_int,
                );
                (Some(
                    (Some(
                        r_fatal as unsafe extern "C" fn(*const libc::c_char, ...) -> (),
                    ))
                        .unwrap(),
                ))
                    .unwrap()(
                    if fp == stdout {
                        dcgettext(
                            0 as *const libc::c_char,
                            b"fflush: cannot flush standard output: %s\0" as *const u8
                                as *const libc::c_char,
                            5 as libc::c_int,
                        )
                    } else {
                        dcgettext(
                            0 as *const libc::c_char,
                            b"fflush: cannot flush standard error: %s\0" as *const u8
                                as *const libc::c_char,
                            5 as libc::c_int,
                        )
                    },
                    strerror(*__errno_location()),
                );
            }
        } else {
            update_ERRNO_int(*__errno_location());
            (set_loc
                as unsafe extern "C" fn(
                    *const libc::c_char,
                    libc::c_int,
                ) -> ())(
                b"io.c\0" as *const u8 as *const libc::c_char,
                1455 as libc::c_int,
            );
            (Some(
                (Some(r_warning as unsafe extern "C" fn(*const libc::c_char, ...) -> ()))
                    .unwrap(),
            ))
                .unwrap()(
                if fp == stdout {
                    dcgettext(
                        0 as *const libc::c_char,
                        b"error writing standard output: %s\0" as *const u8
                            as *const libc::c_char,
                        5 as libc::c_int,
                    )
                } else {
                    dcgettext(
                        0 as *const libc::c_char,
                        b"error writing standard error: %s\0" as *const u8
                            as *const libc::c_char,
                        5 as libc::c_int,
                    )
                },
                strerror(*__errno_location()),
            );
        }
        return 0 as libc::c_int != 0;
    }
    return 1 as libc::c_int != 0;
}
pub unsafe extern "C" fn flush_io() -> libc::c_int {
    let mut rp: *mut redirect = 0 as *mut redirect;
    let mut status: libc::c_int = 0 as libc::c_int;
    *__errno_location() = 0 as libc::c_int;
    if !non_fatal_flush_std_file(stdout) {
        status += 1;
        status;
    }
    *__errno_location() = 0 as libc::c_int;
    if !non_fatal_flush_std_file(stderr) {
        status += 1;
        status;
    }
    rp = red_head;
    while !rp.is_null() {
        let mut messagefunc: Option::<
            unsafe extern "C" fn(*const libc::c_char, ...) -> (),
        > = Some(r_fatal as unsafe extern "C" fn(*const libc::c_char, ...) -> ());
        if (*rp).flag as libc::c_uint & RED_WRITE as libc::c_int as libc::c_uint
            != 0 as libc::c_int as libc::c_uint && !((*rp).output.fp).is_null()
        {
            if ((*rp).output.gawk_fflush).unwrap()((*rp).output.fp, (*rp).output.opaque)
                != 0 as libc::c_int
            {
                update_ERRNO_int(*__errno_location());
                if is_non_fatal_redirect((*rp).value, strlen((*rp).value)) {
                    messagefunc = Some(
                        r_warning as unsafe extern "C" fn(*const libc::c_char, ...) -> (),
                    );
                }
                if (*rp).flag as libc::c_uint & RED_PIPE as libc::c_int as libc::c_uint
                    != 0 as libc::c_int as libc::c_uint
                {
                    messagefunc
                        .unwrap()(
                        dcgettext(
                            0 as *const libc::c_char,
                            b"pipe flush of `%s' failed: %s\0" as *const u8
                                as *const libc::c_char,
                            5 as libc::c_int,
                        ),
                        (*rp).value,
                        strerror(*__errno_location()),
                    );
                } else if (*rp).flag as libc::c_uint
                    & RED_TWOWAY as libc::c_int as libc::c_uint
                    != 0 as libc::c_int as libc::c_uint
                {
                    messagefunc
                        .unwrap()(
                        dcgettext(
                            0 as *const libc::c_char,
                            b"co-process flush of pipe to `%s' failed: %s\0" as *const u8
                                as *const libc::c_char,
                            5 as libc::c_int,
                        ),
                        (*rp).value,
                        strerror(*__errno_location()),
                    );
                } else {
                    messagefunc
                        .unwrap()(
                        dcgettext(
                            0 as *const libc::c_char,
                            b"file flush of `%s' failed: %s\0" as *const u8
                                as *const libc::c_char,
                            5 as libc::c_int,
                        ),
                        (*rp).value,
                        strerror(*__errno_location()),
                    );
                }
                status += 1;
                status;
            }
        }
        rp = (*rp).next;
    }
    if status != 0 as libc::c_int {
        status = -(1 as libc::c_int);
    }
    return status;
}
pub unsafe extern "C" fn close_io(
    mut stdio_problem: *mut bool,
    mut got_EPIPE: *mut bool,
) -> libc::c_int {
    let mut rp: *mut redirect = 0 as *mut redirect;
    let mut next: *mut redirect = 0 as *mut redirect;
    let mut status: libc::c_int = 0 as libc::c_int;
    *got_EPIPE = 0 as libc::c_int != 0;
    *stdio_problem = *got_EPIPE;
    *__errno_location() = 0 as libc::c_int;
    rp = red_head;
    while !rp.is_null() {
        next = (*rp).next;
        if close_redir(
            rp,
            do_flags as libc::c_uint
                & (DO_LINT_INVALID as libc::c_int | DO_LINT_ALL as libc::c_int)
                    as libc::c_uint != 0,
            CLOSE_ALL,
        ) != 0
        {
            status += 1;
            status;
        }
        rp = 0 as *mut redirect;
        rp = next;
    }
    *stdio_problem = 0 as libc::c_int != 0;
    if fflush(stdout) != 0 as libc::c_int {
        os_maybe_set_errno();
        if *__errno_location() != 32 as libc::c_int {
            (set_loc
                as unsafe extern "C" fn(
                    *const libc::c_char,
                    libc::c_int,
                ) -> ())(
                b"io.c\0" as *const u8 as *const libc::c_char,
                1545 as libc::c_int,
            );
            (Some(
                (Some(r_warning as unsafe extern "C" fn(*const libc::c_char, ...) -> ()))
                    .unwrap(),
            ))
                .unwrap()(
                dcgettext(
                    0 as *const libc::c_char,
                    b"error writing standard output: %s\0" as *const u8
                        as *const libc::c_char,
                    5 as libc::c_int,
                ),
                strerror(*__errno_location()),
            );
        } else {
            *got_EPIPE = 1 as libc::c_int != 0;
        }
        status += 1;
        status;
        *stdio_problem = 1 as libc::c_int != 0;
    }
    if fflush(stderr) != 0 as libc::c_int {
        os_maybe_set_errno();
        if *__errno_location() != 32 as libc::c_int {
            (set_loc
                as unsafe extern "C" fn(
                    *const libc::c_char,
                    libc::c_int,
                ) -> ())(
                b"io.c\0" as *const u8 as *const libc::c_char,
                1556 as libc::c_int,
            );
            (Some(
                (Some(r_warning as unsafe extern "C" fn(*const libc::c_char, ...) -> ()))
                    .unwrap(),
            ))
                .unwrap()(
                dcgettext(
                    0 as *const libc::c_char,
                    b"error writing standard error: %s\0" as *const u8
                        as *const libc::c_char,
                    5 as libc::c_int,
                ),
                strerror(*__errno_location()),
            );
        } else {
            *got_EPIPE = 1 as libc::c_int != 0;
        }
        status += 1;
        status;
        *stdio_problem = 1 as libc::c_int != 0;
    }
    return status;
}
unsafe extern "C" fn str2mode(mut mode: *const libc::c_char) -> libc::c_int {
    let mut ret: libc::c_int = 0;
    let mut second: *const libc::c_char = &*mode.offset(1 as libc::c_int as isize)
        as *const libc::c_char;
    if *second as libc::c_int == 'b' as i32 {
        second = second.offset(1);
        second;
    }
    match *mode.offset(0 as libc::c_int as isize) as libc::c_int {
        114 => {
            ret = 0 as libc::c_int;
            if *second as libc::c_int == '+' as i32
                || *second as libc::c_int == 'w' as i32
            {
                ret = 0o2 as libc::c_int;
            }
        }
        119 => {
            ret = 0o1 as libc::c_int | 0o100 as libc::c_int | 0o1000 as libc::c_int;
            if *second as libc::c_int == '+' as i32
                || *second as libc::c_int == 'r' as i32
            {
                ret = 0o2 as libc::c_int | 0o100 as libc::c_int | 0o1000 as libc::c_int;
            }
        }
        97 => {
            ret = 0o1 as libc::c_int | 0o2000 as libc::c_int | 0o100 as libc::c_int;
            if *second as libc::c_int == '+' as i32 {
                ret = 0o2 as libc::c_int | 0o2000 as libc::c_int | 0o100 as libc::c_int;
            }
        }
        _ => {
            ret = 0 as libc::c_int;
            r_fatal(
                b"internal error: file %s, line %d: invalid open mode \"%s\"\0"
                    as *const u8 as *const libc::c_char,
                b"io.c\0" as *const u8 as *const libc::c_char,
                1598 as libc::c_int,
                mode,
            );
        }
    }
    if !(strchr(mode, 'b' as i32)).is_null() {
        ret |= 0 as libc::c_int;
    }
    return ret;
}
unsafe extern "C" fn socketopen(
    mut family: libc::c_int,
    mut type_0: libc::c_int,
    mut localpname: *const libc::c_char,
    mut remotepname: *const libc::c_char,
    mut remotehostname: *const libc::c_char,
    mut hard_error: *mut bool,
) -> libc::c_int {
    let mut lres: *mut addrinfo = 0 as *mut addrinfo;
    let mut lres0: *mut addrinfo = 0 as *mut addrinfo;
    let mut lhints: addrinfo = addrinfo {
        ai_flags: 0,
        ai_family: 0,
        ai_socktype: 0,
        ai_protocol: 0,
        ai_addrlen: 0,
        ai_addr: 0 as *mut sockaddr,
        ai_canonname: 0 as *mut libc::c_char,
        ai_next: 0 as *mut addrinfo,
    };
    let mut rres: *mut addrinfo = 0 as *mut addrinfo;
    let mut rres0: *mut addrinfo = 0 as *mut addrinfo;
    let mut rhints: addrinfo = addrinfo {
        ai_flags: 0,
        ai_family: 0,
        ai_socktype: 0,
        ai_protocol: 0,
        ai_addrlen: 0,
        ai_addr: 0 as *mut sockaddr,
        ai_canonname: 0 as *mut libc::c_char,
        ai_next: 0 as *mut addrinfo,
    };
    let mut lerror: libc::c_int = 0;
    let mut rerror: libc::c_int = 0;
    let mut socket_fd: libc::c_int = -(1 as libc::c_int);
    let mut any_remote_host: libc::c_int = (strcmp(
        remotehostname,
        b"0\0" as *const u8 as *const libc::c_char,
    ) == 0 as libc::c_int) as libc::c_int;
    memset(
        &mut lhints as *mut addrinfo as *mut libc::c_void,
        '\0' as i32,
        ::std::mem::size_of::<addrinfo>() as libc::c_ulong,
    );
    lhints.ai_socktype = type_0;
    lhints.ai_family = family;
    lhints.ai_flags = 0x1 as libc::c_int;
    if lhints.ai_family == 0 as libc::c_int {
        lhints.ai_flags |= 0x20 as libc::c_int;
    }
    lerror = getaddrinfo(0 as *const libc::c_char, localpname, &mut lhints, &mut lres);
    if lerror != 0 {
        if strcmp(localpname, b"0\0" as *const u8 as *const libc::c_char)
            != 0 as libc::c_int
        {
            (set_loc
                as unsafe extern "C" fn(
                    *const libc::c_char,
                    libc::c_int,
                ) -> ())(
                b"io.c\0" as *const u8 as *const libc::c_char,
                1645 as libc::c_int,
            );
            (Some(
                (Some(r_warning as unsafe extern "C" fn(*const libc::c_char, ...) -> ()))
                    .unwrap(),
            ))
                .unwrap()(
                dcgettext(
                    0 as *const libc::c_char,
                    b"local port %s invalid in `/inet': %s\0" as *const u8
                        as *const libc::c_char,
                    5 as libc::c_int,
                ),
                localpname,
                gai_strerror(lerror),
            );
            *hard_error = 1 as libc::c_int != 0;
            return -(1 as libc::c_int);
        }
        lres0 = 0 as *mut addrinfo;
        lres = &mut lhints;
    } else {
        lres0 = lres;
    }
    while !lres.is_null() {
        memset(
            &mut rhints as *mut addrinfo as *mut libc::c_void,
            '\0' as i32,
            ::std::mem::size_of::<addrinfo>() as libc::c_ulong,
        );
        rhints.ai_flags = lhints.ai_flags;
        rhints.ai_socktype = lhints.ai_socktype;
        rhints.ai_family = lhints.ai_family;
        rhints.ai_protocol = lhints.ai_protocol;
        rerror = getaddrinfo(
            if any_remote_host != 0 { 0 as *const libc::c_char } else { remotehostname },
            remotepname,
            &mut rhints,
            &mut rres,
        );
        if rerror != 0 {
            if !lres0.is_null() {
                freeaddrinfo(lres0);
            }
            (set_loc
                as unsafe extern "C" fn(
                    *const libc::c_char,
                    libc::c_int,
                ) -> ())(
                b"io.c\0" as *const u8 as *const libc::c_char,
                1671 as libc::c_int,
            );
            (Some(
                (Some(r_warning as unsafe extern "C" fn(*const libc::c_char, ...) -> ()))
                    .unwrap(),
            ))
                .unwrap()(
                dcgettext(
                    0 as *const libc::c_char,
                    b"remote host and port information (%s, %s) invalid: %s\0"
                        as *const u8 as *const libc::c_char,
                    5 as libc::c_int,
                ),
                remotehostname,
                remotepname,
                gai_strerror(rerror),
            );
            *hard_error = 1 as libc::c_int != 0;
            return -(1 as libc::c_int);
        }
        rres0 = rres;
        socket_fd = -(1 as libc::c_int);
        while !rres.is_null() {
            socket_fd = socket(
                (*rres).ai_family,
                (*rres).ai_socktype,
                (*rres).ai_protocol,
            );
            if !(socket_fd < 0 as libc::c_int || socket_fd == -(1 as libc::c_int)) {
                if type_0 == SOCK_STREAM as libc::c_int {
                    let mut on: libc::c_int = 1 as libc::c_int;
                    let mut linger: linger = linger { l_onoff: 0, l_linger: 0 };
                    memset(
                        &mut linger as *mut linger as *mut libc::c_void,
                        '\0' as i32,
                        ::std::mem::size_of::<linger>() as libc::c_ulong,
                    );
                    setsockopt(
                        socket_fd,
                        1 as libc::c_int,
                        2 as libc::c_int,
                        &mut on as *mut libc::c_int as *mut libc::c_char
                            as *const libc::c_void,
                        ::std::mem::size_of::<libc::c_int>() as libc::c_ulong
                            as socklen_t,
                    );
                    linger.l_onoff = 1 as libc::c_int;
                    linger.l_linger = 30 as libc::c_int;
                    setsockopt(
                        socket_fd,
                        1 as libc::c_int,
                        13 as libc::c_int,
                        &mut linger as *mut linger as *mut libc::c_char
                            as *const libc::c_void,
                        ::std::mem::size_of::<linger>() as libc::c_ulong as socklen_t,
                    );
                }
                if !(bind(
                    socket_fd,
                    __CONST_SOCKADDR_ARG {
                        __sockaddr__: (*lres).ai_addr,
                    },
                    (*lres).ai_addrlen,
                ) != 0 as libc::c_int)
                {
                    if any_remote_host == 0 {
                        if connect(
                            socket_fd,
                            __CONST_SOCKADDR_ARG {
                                __sockaddr__: (*rres).ai_addr,
                            },
                            (*rres).ai_addrlen,
                        ) == 0 as libc::c_int
                        {
                            break;
                        }
                    } else if type_0 == SOCK_STREAM as libc::c_int {
                        let mut clientsocket_fd: libc::c_int = -(1 as libc::c_int);
                        let mut remote_addr: sockaddr_storage = sockaddr_storage {
                            ss_family: 0,
                            __ss_padding: [0; 118],
                            __ss_align: 0,
                        };
                        let mut namelen: socklen_t = ::std::mem::size_of::<
                            sockaddr_storage,
                        >() as libc::c_ulong as socklen_t;
                        if listen(socket_fd, 1 as libc::c_int) >= 0 as libc::c_int
                            && {
                                clientsocket_fd = accept(
                                    socket_fd,
                                    __SOCKADDR_ARG {
                                        __sockaddr__: &mut remote_addr as *mut sockaddr_storage
                                            as *mut sockaddr,
                                    },
                                    &mut namelen,
                                );
                                clientsocket_fd >= 0 as libc::c_int
                            }
                        {
                            close(socket_fd);
                            socket_fd = clientsocket_fd;
                            break;
                        }
                    } else if type_0 == SOCK_DGRAM as libc::c_int {
                        let mut buf: [libc::c_char; 10] = [0; 10];
                        let mut remote_addr_0: sockaddr_storage = sockaddr_storage {
                            ss_family: 0,
                            __ss_padding: [0; 118],
                            __ss_align: 0,
                        };
                        let mut read_len: socklen_t = ::std::mem::size_of::<
                            sockaddr_storage,
                        >() as libc::c_ulong as socklen_t;
                        if recvfrom(
                            socket_fd,
                            buf.as_mut_ptr() as *mut libc::c_void,
                            1 as libc::c_int as size_t,
                            MSG_PEEK as libc::c_int,
                            __SOCKADDR_ARG {
                                __sockaddr__: &mut remote_addr_0 as *mut sockaddr_storage
                                    as *mut sockaddr,
                            },
                            &mut read_len,
                        ) >= 0 as libc::c_int as libc::c_long && read_len != 0
                            && connect(
                                socket_fd,
                                __CONST_SOCKADDR_ARG {
                                    __sockaddr__: &mut remote_addr_0 as *mut sockaddr_storage
                                        as *mut sockaddr,
                                },
                                read_len,
                            ) == 0 as libc::c_int
                        {
                            break;
                        }
                    }
                }
            }
            if socket_fd != -(1 as libc::c_int) {
                close(socket_fd);
            }
            socket_fd = -(1 as libc::c_int);
            rres = (*rres).ai_next;
        }
        freeaddrinfo(rres0);
        if socket_fd != -(1 as libc::c_int) {
            break;
        }
        lres = (*lres).ai_next;
    }
    if !lres0.is_null() {
        freeaddrinfo(lres0);
    }
    return socket_fd;
}
pub unsafe extern "C" fn devopen_simple(
    mut name: *const libc::c_char,
    mut mode: *const libc::c_char,
    mut try_real_open: bool,
) -> libc::c_int {
    let mut openfd: libc::c_int = 0;
    let mut cp: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut ptr: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut flag: libc::c_int = 0 as libc::c_int;
    if strcmp(name, b"-\0" as *const u8 as *const libc::c_char) == 0 as libc::c_int {
        if *mode.offset(0 as libc::c_int as isize) as libc::c_int == 'r' as i32 {
            return fileno(stdin)
        } else {
            return fileno(stdout)
        }
    }
    flag = str2mode(mode);
    openfd = -(1 as libc::c_int);
    if !(do_flags as libc::c_uint & DO_POSIX as libc::c_int as libc::c_uint != 0) {
        openfd = os_devopen(name, flag);
        if openfd != -(1 as libc::c_int) {
            os_close_on_exec(
                openfd,
                name,
                b"file\0" as *const u8 as *const libc::c_char,
                b"\0" as *const u8 as *const libc::c_char,
            );
            return openfd;
        }
        if strncmp(
            name,
            b"/dev/\0" as *const u8 as *const libc::c_char,
            5 as libc::c_int as libc::c_ulong,
        ) == 0 as libc::c_int
        {
            cp = (name as *mut libc::c_char).offset(5 as libc::c_int as isize);
            if strcmp(cp, b"stdin\0" as *const u8 as *const libc::c_char)
                == 0 as libc::c_int && flag & 0o3 as libc::c_int == 0 as libc::c_int
            {
                openfd = fileno(stdin);
            } else if strcmp(cp, b"stdout\0" as *const u8 as *const libc::c_char)
                == 0 as libc::c_int && flag & 0o3 as libc::c_int == 0o1 as libc::c_int
            {
                openfd = fileno(stdout);
            } else if strcmp(cp, b"stderr\0" as *const u8 as *const libc::c_char)
                == 0 as libc::c_int && flag & 0o3 as libc::c_int == 0o1 as libc::c_int
            {
                openfd = fileno(stderr);
            } else if !(do_flags as libc::c_uint
                & DO_TRADITIONAL as libc::c_int as libc::c_uint != 0)
            {
                if strncmp(
                    cp,
                    b"fd/\0" as *const u8 as *const libc::c_char,
                    3 as libc::c_int as libc::c_ulong,
                ) == 0 as libc::c_int
                {
                    let mut sbuf: stat = stat {
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
                    cp = cp.offset(3 as libc::c_int as isize);
                    openfd = strtoul(cp, &mut ptr, 10 as libc::c_int) as libc::c_int;
                    if openfd <= -(1 as libc::c_int) || ptr == cp
                        || fstat(openfd, &mut sbuf) < 0 as libc::c_int
                    {
                        openfd = -(1 as libc::c_int);
                    }
                }
            }
        }
    }
    if try_real_open {
        openfd = open(name, flag, 0o666 as libc::c_int);
    }
    return openfd;
}
pub unsafe extern "C" fn devopen(
    mut name: *const libc::c_char,
    mut mode: *const libc::c_char,
) -> libc::c_int {
    let mut openfd: libc::c_int = 0;
    let mut flag: libc::c_int = 0;
    let mut isi: inet_socket_info = inet_socket_info {
        family: 0,
        protocol: 0,
        localport: C2RustUnnamed_11 {
            offset: 0,
            len: 0,
        },
        remotehost: C2RustUnnamed_11 {
            offset: 0,
            len: 0,
        },
        remoteport: C2RustUnnamed_11 {
            offset: 0,
            len: 0,
        },
    };
    let mut save_errno: libc::c_int = 0 as libc::c_int;
    openfd = devopen_simple(name, mode, 0 as libc::c_int != 0);
    if openfd != -(1 as libc::c_int) {
        return openfd;
    }
    flag = str2mode(mode);
    if !(do_flags as libc::c_uint & DO_TRADITIONAL as libc::c_int as libc::c_uint != 0) {
        if inetfile(name, strlen(name), &mut isi) {
            static mut def_retries: libc::c_ulong = 20 as libc::c_int as libc::c_ulong;
            static mut first_time: bool = 1 as libc::c_int != 0;
            let mut retries: libc::c_ulong = 0 as libc::c_int as libc::c_ulong;
            static mut msleep: libc::c_long = 1000 as libc::c_int as libc::c_long;
            let mut hard_error: bool = 0 as libc::c_int != 0;
            let mut non_fatal: bool = is_non_fatal_redirect(name, strlen(name));
            let mut save: libc::c_char = 0;
            let mut cp: *mut libc::c_char = name as *mut libc::c_char;
            *cp
                .offset(
                    (isi.localport.offset + isi.localport.len) as isize,
                ) = '\0' as i32 as libc::c_char;
            *cp
                .offset(
                    (isi.remotehost.offset + isi.remotehost.len) as isize,
                ) = '\0' as i32 as libc::c_char;
            save = *cp.offset((isi.remoteport.offset + isi.remoteport.len) as isize);
            *cp
                .offset(
                    (isi.remoteport.offset + isi.remoteport.len) as isize,
                ) = '\0' as i32 as libc::c_char;
            if first_time {
                let mut cp_0: *mut libc::c_char = 0 as *mut libc::c_char;
                let mut end: *mut libc::c_char = 0 as *mut libc::c_char;
                let mut count: libc::c_ulong = 0 as libc::c_int as libc::c_ulong;
                let mut ms2: *mut libc::c_char = 0 as *mut libc::c_char;
                first_time = 0 as libc::c_int != 0;
                cp_0 = getenv(
                    b"GAWK_SOCK_RETRIES\0" as *const u8 as *const libc::c_char,
                );
                if !cp_0.is_null() {
                    count = strtoul(cp_0, &mut end, 10 as libc::c_int);
                    if end != cp_0 && count > 0 as libc::c_int as libc::c_ulong {
                        def_retries = count;
                    }
                }
                ms2 = getenv(b"GAWK_MSEC_SLEEP\0" as *const u8 as *const libc::c_char);
                if !ms2.is_null() {
                    msleep = strtol(ms2, &mut end, 10 as libc::c_int);
                    if end == ms2 || msleep < 0 as libc::c_int as libc::c_long {
                        msleep = 1000 as libc::c_int as libc::c_long;
                    } else {
                        msleep *= 1000 as libc::c_int as libc::c_long;
                    }
                }
            }
            retries = if non_fatal as libc::c_int != 0 {
                1 as libc::c_int as libc::c_ulong
            } else {
                def_retries
            };
            *__errno_location() = 0 as libc::c_int;
            loop {
                openfd = socketopen(
                    isi.family,
                    isi.protocol,
                    name.offset(isi.localport.offset as isize),
                    name.offset(isi.remoteport.offset as isize),
                    name.offset(isi.remotehost.offset as isize),
                    &mut hard_error,
                );
                retries = retries.wrapping_sub(1);
                retries;
                if !(openfd == -(1 as libc::c_int) && !hard_error
                    && retries > 0 as libc::c_int as libc::c_ulong
                    && usleep(msleep as __useconds_t) == 0 as libc::c_int)
                {
                    break;
                }
            }
            save_errno = *__errno_location();
            *cp
                .offset(
                    (isi.localport.offset + isi.localport.len) as isize,
                ) = '/' as i32 as libc::c_char;
            *cp
                .offset(
                    (isi.remotehost.offset + isi.remotehost.len) as isize,
                ) = '/' as i32 as libc::c_char;
            *cp.offset((isi.remoteport.offset + isi.remoteport.len) as isize) = save;
        }
    }
    if openfd == -(1 as libc::c_int) {
        openfd = open(name, flag, 0o666 as libc::c_int);
        if openfd == -(1 as libc::c_int) && *__errno_location() == 2 as libc::c_int
            && save_errno != 0
        {
            *__errno_location() = save_errno;
        }
    }
    if openfd != -(1 as libc::c_int) {
        if openfd > fileno(stderr) {
            os_close_on_exec(
                openfd,
                name,
                b"file\0" as *const u8 as *const libc::c_char,
                b"\0" as *const u8 as *const libc::c_char,
            );
        }
    }
    return openfd;
}
unsafe extern "C" fn push_pty_line_disciplines(mut slave: libc::c_int) {}
unsafe extern "C" fn set_slave_pty_attributes(mut slave: libc::c_int) {
    let mut st: termios = termios {
        c_iflag: 0,
        c_oflag: 0,
        c_cflag: 0,
        c_lflag: 0,
        c_line: 0,
        c_cc: [0; 32],
        c_ispeed: 0,
        c_ospeed: 0,
    };
    tcgetattr(slave, &mut st);
    st.c_iflag
        &= !(0o40 as libc::c_int | 0o200 as libc::c_int | 0o100 as libc::c_int
            | 0o10000 as libc::c_int) as libc::c_uint;
    st.c_iflag
        |= (0o400 as libc::c_int | 0o4 as libc::c_int | 0o2 as libc::c_int
            | 0o2000 as libc::c_int) as libc::c_uint;
    st.c_oflag &= !(0o1 as libc::c_int) as libc::c_uint;
    st.c_cflag &= !(0o60 as libc::c_int) as libc::c_uint;
    st.c_cflag
        |= (0o200 as libc::c_int | 0o60 as libc::c_int | 0o4000 as libc::c_int)
            as libc::c_uint;
    st.c_lflag
        &= !(0o10 as libc::c_int | 0o20 as libc::c_int | 0o40 as libc::c_int
            | 0o200 as libc::c_int | 0o400 as libc::c_int) as libc::c_uint;
    st.c_lflag |= 0o1 as libc::c_int as libc::c_uint;
    st.c_cc[0 as libc::c_int as usize] = '\u{3}' as i32 as cc_t;
    st.c_cc[1 as libc::c_int as usize] = '\u{1c}' as i32 as cc_t;
    st.c_cc[2 as libc::c_int as usize] = '\u{7f}' as i32 as cc_t;
    st.c_cc[3 as libc::c_int as usize] = '\u{15}' as i32 as cc_t;
    st.c_cc[4 as libc::c_int as usize] = '\u{4}' as i32 as cc_t;
    tcsetattr(slave, 0 as libc::c_int, &mut st);
}
unsafe extern "C" fn fork_and_open_slave_pty(
    mut slavenam: *const libc::c_char,
    mut master: libc::c_int,
    mut command: *const libc::c_char,
    mut pid: *mut pid_t,
) -> bool {
    let mut slave: libc::c_int = 0;
    let mut save_errno: libc::c_int = 0;
    slave = open(slavenam, 0o2 as libc::c_int);
    if slave < 0 as libc::c_int {
        close(master);
        (set_loc
            as unsafe extern "C" fn(
                *const libc::c_char,
                libc::c_int,
            ) -> ())(b"io.c\0" as *const u8 as *const libc::c_char, 2087 as libc::c_int);
        (Some(
            (Some(r_fatal as unsafe extern "C" fn(*const libc::c_char, ...) -> ()))
                .unwrap(),
        ))
            .unwrap()(
            dcgettext(
                0 as *const libc::c_char,
                b"could not open `%s', mode `%s'\0" as *const u8 as *const libc::c_char,
                5 as libc::c_int,
            ),
            slavenam,
            b"r+\0" as *const u8 as *const libc::c_char,
        );
    }
    push_pty_line_disciplines(slave);
    set_slave_pty_attributes(slave);
    *pid = fork();
    match *pid {
        0 => {
            setsid();
            ioctl(slave, 0x540e as libc::c_int as libc::c_ulong, 0 as libc::c_int);
            if close(master) == -(1 as libc::c_int) {
                (set_loc
                    as unsafe extern "C" fn(
                        *const libc::c_char,
                        libc::c_int,
                    ) -> ())(
                    b"io.c\0" as *const u8 as *const libc::c_char,
                    2104 as libc::c_int,
                );
                (Some(
                    (Some(
                        r_fatal as unsafe extern "C" fn(*const libc::c_char, ...) -> (),
                    ))
                        .unwrap(),
                ))
                    .unwrap()(
                    dcgettext(
                        0 as *const libc::c_char,
                        b"close of master pty failed: %s\0" as *const u8
                            as *const libc::c_char,
                        5 as libc::c_int,
                    ),
                    strerror(*__errno_location()),
                );
            }
            if close(1 as libc::c_int) == -(1 as libc::c_int) {
                (set_loc
                    as unsafe extern "C" fn(
                        *const libc::c_char,
                        libc::c_int,
                    ) -> ())(
                    b"io.c\0" as *const u8 as *const libc::c_char,
                    2106 as libc::c_int,
                );
                (Some(
                    (Some(
                        r_fatal as unsafe extern "C" fn(*const libc::c_char, ...) -> (),
                    ))
                        .unwrap(),
                ))
                    .unwrap()(
                    dcgettext(
                        0 as *const libc::c_char,
                        b"close of stdout in child failed: %s\0" as *const u8
                            as *const libc::c_char,
                        5 as libc::c_int,
                    ),
                    strerror(*__errno_location()),
                );
            }
            if dup(slave) != 1 as libc::c_int {
                (set_loc
                    as unsafe extern "C" fn(
                        *const libc::c_char,
                        libc::c_int,
                    ) -> ())(
                    b"io.c\0" as *const u8 as *const libc::c_char,
                    2109 as libc::c_int,
                );
                (Some(
                    (Some(
                        r_fatal as unsafe extern "C" fn(*const libc::c_char, ...) -> (),
                    ))
                        .unwrap(),
                ))
                    .unwrap()(
                    dcgettext(
                        0 as *const libc::c_char,
                        b"moving slave pty to stdout in child failed (dup: %s)\0"
                            as *const u8 as *const libc::c_char,
                        5 as libc::c_int,
                    ),
                    strerror(*__errno_location()),
                );
            }
            if close(0 as libc::c_int) == -(1 as libc::c_int) {
                (set_loc
                    as unsafe extern "C" fn(
                        *const libc::c_char,
                        libc::c_int,
                    ) -> ())(
                    b"io.c\0" as *const u8 as *const libc::c_char,
                    2111 as libc::c_int,
                );
                (Some(
                    (Some(
                        r_fatal as unsafe extern "C" fn(*const libc::c_char, ...) -> (),
                    ))
                        .unwrap(),
                ))
                    .unwrap()(
                    dcgettext(
                        0 as *const libc::c_char,
                        b"close of stdin in child failed: %s\0" as *const u8
                            as *const libc::c_char,
                        5 as libc::c_int,
                    ),
                    strerror(*__errno_location()),
                );
            }
            if dup(slave) != 0 as libc::c_int {
                (set_loc
                    as unsafe extern "C" fn(
                        *const libc::c_char,
                        libc::c_int,
                    ) -> ())(
                    b"io.c\0" as *const u8 as *const libc::c_char,
                    2114 as libc::c_int,
                );
                (Some(
                    (Some(
                        r_fatal as unsafe extern "C" fn(*const libc::c_char, ...) -> (),
                    ))
                        .unwrap(),
                ))
                    .unwrap()(
                    dcgettext(
                        0 as *const libc::c_char,
                        b"moving slave pty to stdin in child failed (dup: %s)\0"
                            as *const u8 as *const libc::c_char,
                        5 as libc::c_int,
                    ),
                    strerror(*__errno_location()),
                );
            }
            if close(slave) != 0 {
                (set_loc
                    as unsafe extern "C" fn(
                        *const libc::c_char,
                        libc::c_int,
                    ) -> ())(
                    b"io.c\0" as *const u8 as *const libc::c_char,
                    2116 as libc::c_int,
                );
                (Some(
                    (Some(
                        r_fatal as unsafe extern "C" fn(*const libc::c_char, ...) -> (),
                    ))
                        .unwrap(),
                ))
                    .unwrap()(
                    dcgettext(
                        0 as *const libc::c_char,
                        b"close of slave pty failed: %s\0" as *const u8
                            as *const libc::c_char,
                        5 as libc::c_int,
                    ),
                    strerror(*__errno_location()),
                );
            }
            signal(13 as libc::c_int, None);
            execl(
                b"/bin/sh\0" as *const u8 as *const libc::c_char,
                b"sh\0" as *const u8 as *const libc::c_char,
                b"-c\0" as *const u8 as *const libc::c_char,
                command,
                0 as *mut libc::c_void,
            );
            _exit(
                if *__errno_location() == 2 as libc::c_int {
                    127 as libc::c_int
                } else {
                    126 as libc::c_int
                },
            );
        }
        -1 => {
            save_errno = *__errno_location();
            close(master);
            close(slave);
            *__errno_location() = save_errno;
            return 0 as libc::c_int != 0;
        }
        _ => {}
    }
    if close(slave) != 0 as libc::c_int {
        close(master);
        kill(*pid, 9 as libc::c_int);
        (set_loc
            as unsafe extern "C" fn(
                *const libc::c_char,
                libc::c_int,
            ) -> ())(b"io.c\0" as *const u8 as *const libc::c_char, 2138 as libc::c_int);
        (Some(
            (Some(r_fatal as unsafe extern "C" fn(*const libc::c_char, ...) -> ()))
                .unwrap(),
        ))
            .unwrap()(
            dcgettext(
                0 as *const libc::c_char,
                b"close of slave pty failed: %s\0" as *const u8 as *const libc::c_char,
                5 as libc::c_int,
            ),
            strerror(*__errno_location()),
        );
    }
    return 1 as libc::c_int != 0;
}
unsafe extern "C" fn two_way_open(
    mut str: *const libc::c_char,
    mut rp: *mut redirect,
    mut extfd: libc::c_int,
) -> libc::c_int {
    let mut current_block: u64;
    static mut no_ptys: bool = 0 as libc::c_int != 0;
    if extfd >= 0 as libc::c_int
        || inetfile(str, strlen(str), 0 as *mut inet_socket_info) as libc::c_int != 0
    {
        let mut fd: libc::c_int = 0;
        let mut newfd: libc::c_int = 0;
        fd = if extfd >= 0 as libc::c_int {
            extfd
        } else {
            devopen(str, b"rw\0" as *const u8 as *const libc::c_char)
        };
        if fd == -(1 as libc::c_int) {
            return 0 as libc::c_int;
        }
        if BINMODE & BINMODE_OUTPUT as libc::c_int != 0 as libc::c_int {
            os_setbinmode(fd, 0 as libc::c_int);
        }
        (*rp).output.fp = fdopen(fd, b"wb\0" as *const u8 as *const libc::c_char);
        if ((*rp).output.fp).is_null() {
            close(fd);
            return 0 as libc::c_int;
        }
        newfd = dup(fd);
        if newfd < 0 as libc::c_int {
            ((*rp).output.gawk_fclose).unwrap()((*rp).output.fp, (*rp).output.opaque);
            return 0 as libc::c_int;
        }
        if BINMODE & BINMODE_INPUT as libc::c_int != 0 as libc::c_int {
            os_setbinmode(newfd, 0 as libc::c_int);
        }
        os_close_on_exec(
            fd,
            str,
            b"socket\0" as *const u8 as *const libc::c_char,
            b"to/from\0" as *const u8 as *const libc::c_char,
        );
        os_close_on_exec(
            newfd,
            str,
            b"socket\0" as *const u8 as *const libc::c_char,
            b"to/from\0" as *const u8 as *const libc::c_char,
        );
        (*rp).iop = iop_alloc(newfd, str, 0 as libc::c_int);
        (*rp).output.name = str;
        find_input_parser((*rp).iop);
        iop_finish((*rp).iop);
        if !(*(*rp).iop).valid {
            if do_flags as libc::c_uint & DO_TRADITIONAL as libc::c_int as libc::c_uint
                == 0 && (*(*rp).iop).errcode != 0 as libc::c_int
            {
                update_ERRNO_int((*(*rp).iop).errcode);
            }
            iop_close((*rp).iop);
            (*rp).iop = 0 as *mut IOBUF;
            ((*rp).output.gawk_fclose).unwrap()((*rp).output.fp, (*rp).output.opaque);
            return 0 as libc::c_int;
        }
        (*rp)
            .flag = ::std::mem::transmute::<
            libc::c_uint,
            redirect_flags,
        >((*rp).flag as libc::c_uint | RED_SOCKET as libc::c_int as libc::c_uint);
        return 1 as libc::c_int;
    }
    if find_two_way_processor(str, rp) {
        return 1 as libc::c_int;
    }
    if !no_ptys && pty_vs_pipe(str) as libc::c_int != 0 {
        static mut initialized: bool = 0 as libc::c_int != 0;
        static mut first_pty_letter: libc::c_char = 0;
        let mut slavenam: [libc::c_char; 32] = [0; 32];
        let mut c: libc::c_char = 0;
        let mut master: libc::c_int = 0;
        let mut dup_master: libc::c_int = 0;
        let mut pid: pid_t = 0;
        let mut statb: stat = stat {
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
        static mut pty_chars: [libc::c_char; 27] = unsafe {
            *::std::mem::transmute::<
                &[u8; 27],
                &mut [libc::c_char; 27],
            >(b"pqrstuvwxyzabcdefghijklmno\0")
        };
        let mut i: libc::c_int = 0;
        if !initialized {
            initialized = 1 as libc::c_int != 0;
            i = 0 as libc::c_int;
            loop {
                let fresh3 = i;
                i = i + 1;
                c = pty_chars[fresh3 as usize];
                sprintf(
                    slavenam.as_mut_ptr(),
                    b"/dev/pty%c0\0" as *const u8 as *const libc::c_char,
                    c as libc::c_int,
                );
                if stat(slavenam.as_mut_ptr(), &mut statb) >= 0 as libc::c_int {
                    first_pty_letter = c;
                    break;
                } else if !(pty_chars[i as usize] as libc::c_int != '\0' as i32) {
                    break;
                }
            }
        }
        master = posix_openpt(0o2 as libc::c_int | 0o400 as libc::c_int);
        if master >= 0 as libc::c_int {
            let mut tem: *mut libc::c_char = 0 as *mut libc::c_char;
            grantpt(master);
            unlockpt(master);
            tem = ptsname(master);
            if !tem.is_null() {
                strcpy(slavenam.as_mut_ptr(), tem);
                current_block = 2952062933943373302;
            } else {
                close(master);
                current_block = 15004371738079956865;
            }
        } else {
            current_block = 15004371738079956865;
        }
        match current_block {
            15004371738079956865 => {
                if first_pty_letter != 0 {
                    c = first_pty_letter;
                    's_236: loop {
                        let mut i_0: libc::c_int = 0;
                        let mut cp: *mut libc::c_char = 0 as *mut libc::c_char;
                        i_0 = 0 as libc::c_int;
                        while i_0 < 16 as libc::c_int {
                            sprintf(
                                slavenam.as_mut_ptr(),
                                b"/dev/pty%c%x\0" as *const u8 as *const libc::c_char,
                                c as libc::c_int,
                                i_0,
                            );
                            if stat(slavenam.as_mut_ptr(), &mut statb) < 0 as libc::c_int
                            {
                                no_ptys = 1 as libc::c_int != 0;
                                current_block = 5038789069032708078;
                                break 's_236;
                            } else {
                                master = open(slavenam.as_mut_ptr(), 0o2 as libc::c_int);
                                if master >= 0 as libc::c_int {
                                    slavenam[(::std::mem::size_of::<[libc::c_char; 6]>()
                                        as libc::c_ulong)
                                        .wrapping_sub(1 as libc::c_int as libc::c_ulong)
                                        as usize] = 't' as i32 as libc::c_char;
                                    if access(
                                        slavenam.as_mut_ptr(),
                                        4 as libc::c_int | 2 as libc::c_int,
                                    ) == 0 as libc::c_int
                                    {
                                        current_block = 2952062933943373302;
                                        break 's_236;
                                    }
                                    close(master);
                                }
                                i_0 += 1;
                                i_0;
                            }
                        }
                        cp = strchr(pty_chars.as_mut_ptr(), c as libc::c_int);
                        if *cp.offset(1 as libc::c_int as isize) as libc::c_int
                            != '\0' as i32
                        {
                            cp = cp.offset(1);
                            cp;
                        } else {
                            cp = pty_chars.as_mut_ptr();
                        }
                        c = *cp;
                        if !(c as libc::c_int != first_pty_letter as libc::c_int) {
                            current_block = 17075014677070940716;
                            break;
                        }
                    }
                } else {
                    no_ptys = 1 as libc::c_int != 0;
                    current_block = 17075014677070940716;
                }
                match current_block {
                    5038789069032708078 => {}
                    2952062933943373302 => {}
                    _ => {
                        current_block = 5038789069032708078;
                    }
                }
            }
            _ => {}
        }
        match current_block {
            5038789069032708078 => {}
            _ => {
                if !fork_and_open_slave_pty(
                    slavenam.as_mut_ptr(),
                    master,
                    str,
                    &mut pid,
                ) {
                    (set_loc
                        as unsafe extern "C" fn(
                            *const libc::c_char,
                            libc::c_int,
                        ) -> ())(
                        b"io.c\0" as *const u8 as *const libc::c_char,
                        2300 as libc::c_int,
                    );
                    (Some(
                        (Some(
                            r_fatal
                                as unsafe extern "C" fn(*const libc::c_char, ...) -> (),
                        ))
                            .unwrap(),
                    ))
                        .unwrap()(
                        dcgettext(
                            0 as *const libc::c_char,
                            b"could not create child process or open pty\0" as *const u8
                                as *const libc::c_char,
                            5 as libc::c_int,
                        ),
                    );
                }
                (*rp).pid = pid;
                (*rp).iop = iop_alloc(master, str, 0 as libc::c_int);
                find_input_parser((*rp).iop);
                iop_finish((*rp).iop);
                if !(*(*rp).iop).valid {
                    if do_flags as libc::c_uint
                        & DO_TRADITIONAL as libc::c_int as libc::c_uint == 0
                        && (*(*rp).iop).errcode != 0 as libc::c_int
                    {
                        update_ERRNO_int((*(*rp).iop).errcode);
                    }
                    iop_close((*rp).iop);
                    (*rp).iop = 0 as *mut IOBUF;
                    kill(pid, 9 as libc::c_int);
                    return 0 as libc::c_int;
                }
                (*rp).output.name = str;
                (*rp).output.mode = b"w\0" as *const u8 as *const libc::c_char;
                dup_master = dup(master);
                if dup_master < 0 as libc::c_int
                    || {
                        (*rp)
                            .output
                            .fp = fdopen(
                            dup_master,
                            b"w\0" as *const u8 as *const libc::c_char,
                        );
                        ((*rp).output.fp).is_null()
                    }
                {
                    iop_close((*rp).iop);
                    (*rp).iop = 0 as *mut IOBUF;
                    close(master);
                    kill(pid, 9 as libc::c_int);
                    if dup_master > 0 as libc::c_int {
                        close(dup_master);
                    }
                    return 0 as libc::c_int;
                } else {
                    find_output_wrapper(&mut (*rp).output);
                }
                (*rp)
                    .flag = ::std::mem::transmute::<
                    libc::c_uint,
                    redirect_flags,
                >((*rp).flag as libc::c_uint | RED_PTY as libc::c_int as libc::c_uint);
                os_close_on_exec(
                    master,
                    str,
                    b"pipe\0" as *const u8 as *const libc::c_char,
                    b"from\0" as *const u8 as *const libc::c_char,
                );
                os_close_on_exec(
                    dup_master,
                    str,
                    b"pipe\0" as *const u8 as *const libc::c_char,
                    b"to\0" as *const u8 as *const libc::c_char,
                );
                first_pty_letter = '\0' as i32 as libc::c_char;
                return 1 as libc::c_int;
            }
        }
    }
    let mut ptoc: [libc::c_int; 2] = [0; 2];
    let mut ctop: [libc::c_int; 2] = [0; 2];
    let mut pid_0: libc::c_int = 0;
    let mut save_errno: libc::c_int = 0;
    if pipe(ptoc.as_mut_ptr()) < 0 as libc::c_int {
        return 0 as libc::c_int;
    }
    if pipe(ctop.as_mut_ptr()) < 0 as libc::c_int {
        save_errno = *__errno_location();
        close(ptoc[0 as libc::c_int as usize]);
        close(ptoc[1 as libc::c_int as usize]);
        *__errno_location() = save_errno;
        return 0 as libc::c_int;
    }
    pid_0 = fork();
    if pid_0 < 0 as libc::c_int {
        save_errno = *__errno_location();
        close(ptoc[0 as libc::c_int as usize]);
        close(ptoc[1 as libc::c_int as usize]);
        close(ctop[0 as libc::c_int as usize]);
        close(ctop[1 as libc::c_int as usize]);
        *__errno_location() = save_errno;
        return 0 as libc::c_int;
    }
    if pid_0 == 0 as libc::c_int {
        if close(1 as libc::c_int) == -(1 as libc::c_int) {
            (set_loc
                as unsafe extern "C" fn(
                    *const libc::c_char,
                    libc::c_int,
                ) -> ())(
                b"io.c\0" as *const u8 as *const libc::c_char,
                2447 as libc::c_int,
            );
            (Some(
                (Some(r_fatal as unsafe extern "C" fn(*const libc::c_char, ...) -> ()))
                    .unwrap(),
            ))
                .unwrap()(
                dcgettext(
                    0 as *const libc::c_char,
                    b"close of stdout in child failed: %s\0" as *const u8
                        as *const libc::c_char,
                    5 as libc::c_int,
                ),
                strerror(*__errno_location()),
            );
        }
        if dup(ctop[1 as libc::c_int as usize]) != 1 as libc::c_int {
            (set_loc
                as unsafe extern "C" fn(
                    *const libc::c_char,
                    libc::c_int,
                ) -> ())(
                b"io.c\0" as *const u8 as *const libc::c_char,
                2450 as libc::c_int,
            );
            (Some(
                (Some(r_fatal as unsafe extern "C" fn(*const libc::c_char, ...) -> ()))
                    .unwrap(),
            ))
                .unwrap()(
                dcgettext(
                    0 as *const libc::c_char,
                    b"moving pipe to stdout in child failed (dup: %s)\0" as *const u8
                        as *const libc::c_char,
                    5 as libc::c_int,
                ),
                strerror(*__errno_location()),
            );
        }
        if close(0 as libc::c_int) == -(1 as libc::c_int) {
            (set_loc
                as unsafe extern "C" fn(
                    *const libc::c_char,
                    libc::c_int,
                ) -> ())(
                b"io.c\0" as *const u8 as *const libc::c_char,
                2452 as libc::c_int,
            );
            (Some(
                (Some(r_fatal as unsafe extern "C" fn(*const libc::c_char, ...) -> ()))
                    .unwrap(),
            ))
                .unwrap()(
                dcgettext(
                    0 as *const libc::c_char,
                    b"close of stdin in child failed: %s\0" as *const u8
                        as *const libc::c_char,
                    5 as libc::c_int,
                ),
                strerror(*__errno_location()),
            );
        }
        if dup(ptoc[0 as libc::c_int as usize]) != 0 as libc::c_int {
            (set_loc
                as unsafe extern "C" fn(
                    *const libc::c_char,
                    libc::c_int,
                ) -> ())(
                b"io.c\0" as *const u8 as *const libc::c_char,
                2455 as libc::c_int,
            );
            (Some(
                (Some(r_fatal as unsafe extern "C" fn(*const libc::c_char, ...) -> ()))
                    .unwrap(),
            ))
                .unwrap()(
                dcgettext(
                    0 as *const libc::c_char,
                    b"moving pipe to stdin in child failed (dup: %s)\0" as *const u8
                        as *const libc::c_char,
                    5 as libc::c_int,
                ),
                strerror(*__errno_location()),
            );
        }
        if close(ptoc[0 as libc::c_int as usize]) == -(1 as libc::c_int)
            || close(ptoc[1 as libc::c_int as usize]) == -(1 as libc::c_int)
            || close(ctop[0 as libc::c_int as usize]) == -(1 as libc::c_int)
            || close(ctop[1 as libc::c_int as usize]) == -(1 as libc::c_int)
        {
            (set_loc
                as unsafe extern "C" fn(
                    *const libc::c_char,
                    libc::c_int,
                ) -> ())(
                b"io.c\0" as *const u8 as *const libc::c_char,
                2458 as libc::c_int,
            );
            (Some(
                (Some(r_fatal as unsafe extern "C" fn(*const libc::c_char, ...) -> ()))
                    .unwrap(),
            ))
                .unwrap()(
                dcgettext(
                    0 as *const libc::c_char,
                    b"close of pipe failed: %s\0" as *const u8 as *const libc::c_char,
                    5 as libc::c_int,
                ),
                strerror(*__errno_location()),
            );
        }
        signal(13 as libc::c_int, None);
        execl(
            b"/bin/sh\0" as *const u8 as *const libc::c_char,
            b"sh\0" as *const u8 as *const libc::c_char,
            b"-c\0" as *const u8 as *const libc::c_char,
            str,
            0 as *mut libc::c_void,
        );
        _exit(
            if *__errno_location() == 2 as libc::c_int {
                127 as libc::c_int
            } else {
                126 as libc::c_int
            },
        );
    }
    if BINMODE & BINMODE_INPUT as libc::c_int != 0 as libc::c_int {
        os_setbinmode(ctop[0 as libc::c_int as usize], 0 as libc::c_int);
    }
    if BINMODE & BINMODE_OUTPUT as libc::c_int != 0 as libc::c_int {
        os_setbinmode(ptoc[1 as libc::c_int as usize], 0 as libc::c_int);
    }
    (*rp).pid = pid_0;
    (*rp).iop = iop_alloc(ctop[0 as libc::c_int as usize], str, 0 as libc::c_int);
    find_input_parser((*rp).iop);
    iop_finish((*rp).iop);
    if !(*(*rp).iop).valid {
        if do_flags as libc::c_uint & DO_TRADITIONAL as libc::c_int as libc::c_uint == 0
            && (*(*rp).iop).errcode != 0 as libc::c_int
        {
            update_ERRNO_int((*(*rp).iop).errcode);
        }
        iop_close((*rp).iop);
        (*rp).iop = 0 as *mut IOBUF;
        close(ctop[1 as libc::c_int as usize]);
        close(ptoc[0 as libc::c_int as usize]);
        close(ptoc[1 as libc::c_int as usize]);
        kill(pid_0, 9 as libc::c_int);
        return 0 as libc::c_int;
    }
    (*rp)
        .output
        .fp = fdopen(
        ptoc[1 as libc::c_int as usize],
        b"w\0" as *const u8 as *const libc::c_char,
    );
    (*rp).output.mode = b"w\0" as *const u8 as *const libc::c_char;
    (*rp).output.name = str;
    if ((*rp).output.fp).is_null() {
        iop_close((*rp).iop);
        (*rp).iop = 0 as *mut IOBUF;
        close(ctop[0 as libc::c_int as usize]);
        close(ctop[1 as libc::c_int as usize]);
        close(ptoc[0 as libc::c_int as usize]);
        close(ptoc[1 as libc::c_int as usize]);
        kill(pid_0, 9 as libc::c_int);
        return 0 as libc::c_int;
    } else {
        find_output_wrapper(&mut (*rp).output);
    }
    os_close_on_exec(
        ctop[0 as libc::c_int as usize],
        str,
        b"pipe\0" as *const u8 as *const libc::c_char,
        b"from\0" as *const u8 as *const libc::c_char,
    );
    os_close_on_exec(
        ptoc[1 as libc::c_int as usize],
        str,
        b"pipe\0" as *const u8 as *const libc::c_char,
        b"from\0" as *const u8 as *const libc::c_char,
    );
    close(ptoc[0 as libc::c_int as usize]);
    close(ctop[1 as libc::c_int as usize]);
    return 1 as libc::c_int;
}
unsafe extern "C" fn wait_any(mut interesting: libc::c_int) -> libc::c_int {
    let mut pid: libc::c_int = 0;
    let mut status: libc::c_int = 0 as libc::c_int;
    let mut redp: *mut redirect = 0 as *mut redirect;
    let mut set: sigset_t = sigset_t { __val: [0; 16] };
    let mut oldset: sigset_t = sigset_t { __val: [0; 16] };
    sigemptyset(&mut set);
    sigaddset(&mut set, 2 as libc::c_int);
    sigaddset(&mut set, 1 as libc::c_int);
    sigaddset(&mut set, 3 as libc::c_int);
    sigprocmask(0 as libc::c_int, &mut set, &mut oldset);
    loop {
        pid = waitpid(
            -(1 as libc::c_int),
            &mut status,
            if interesting != 0 { 0 as libc::c_int } else { 1 as libc::c_int },
        );
        if pid == 0 as libc::c_int {
            break;
        }
        if interesting != 0 && pid == interesting {
            break;
        }
        if pid != -(1 as libc::c_int) {
            redp = red_head;
            while !redp.is_null() {
                if pid == (*redp).pid {
                    (*redp).pid = -(1 as libc::c_int);
                    (*redp).status = sanitize_exit_status(status);
                    break;
                } else {
                    redp = (*redp).next;
                }
            }
        }
        if pid == -(1 as libc::c_int) && *__errno_location() == 10 as libc::c_int {
            break;
        }
    }
    sigprocmask(2 as libc::c_int, &mut oldset, 0 as *mut sigset_t);
    return status;
}
unsafe extern "C" fn gawk_popen(
    mut cmd: *const libc::c_char,
    mut rp: *mut redirect,
) -> *mut IOBUF {
    let mut p: [libc::c_int; 2] = [0; 2];
    let mut pid: libc::c_int = 0;
    if pipe(p.as_mut_ptr()) < 0 as libc::c_int {
        (set_loc
            as unsafe extern "C" fn(
                *const libc::c_char,
                libc::c_int,
            ) -> ())(b"io.c\0" as *const u8 as *const libc::c_char, 2645 as libc::c_int);
        (Some(
            (Some(r_fatal as unsafe extern "C" fn(*const libc::c_char, ...) -> ()))
                .unwrap(),
        ))
            .unwrap()(
            dcgettext(
                0 as *const libc::c_char,
                b"cannot open pipe `%s': %s\0" as *const u8 as *const libc::c_char,
                5 as libc::c_int,
            ),
            cmd,
            strerror(*__errno_location()),
        );
    }
    pid = fork();
    if pid == 0 as libc::c_int {
        if close(1 as libc::c_int) == -(1 as libc::c_int) {
            (set_loc
                as unsafe extern "C" fn(
                    *const libc::c_char,
                    libc::c_int,
                ) -> ())(
                b"io.c\0" as *const u8 as *const libc::c_char,
                2685 as libc::c_int,
            );
            (Some(
                (Some(r_fatal as unsafe extern "C" fn(*const libc::c_char, ...) -> ()))
                    .unwrap(),
            ))
                .unwrap()(
                dcgettext(
                    0 as *const libc::c_char,
                    b"close of stdout in child failed: %s\0" as *const u8
                        as *const libc::c_char,
                    5 as libc::c_int,
                ),
                strerror(*__errno_location()),
            );
        }
        if dup(p[1 as libc::c_int as usize]) != 1 as libc::c_int {
            (set_loc
                as unsafe extern "C" fn(
                    *const libc::c_char,
                    libc::c_int,
                ) -> ())(
                b"io.c\0" as *const u8 as *const libc::c_char,
                2688 as libc::c_int,
            );
            (Some(
                (Some(r_fatal as unsafe extern "C" fn(*const libc::c_char, ...) -> ()))
                    .unwrap(),
            ))
                .unwrap()(
                dcgettext(
                    0 as *const libc::c_char,
                    b"moving pipe to stdout in child failed (dup: %s)\0" as *const u8
                        as *const libc::c_char,
                    5 as libc::c_int,
                ),
                strerror(*__errno_location()),
            );
        }
        if close(p[0 as libc::c_int as usize]) == -(1 as libc::c_int)
            || close(p[1 as libc::c_int as usize]) == -(1 as libc::c_int)
        {
            (set_loc
                as unsafe extern "C" fn(
                    *const libc::c_char,
                    libc::c_int,
                ) -> ())(
                b"io.c\0" as *const u8 as *const libc::c_char,
                2690 as libc::c_int,
            );
            (Some(
                (Some(r_fatal as unsafe extern "C" fn(*const libc::c_char, ...) -> ()))
                    .unwrap(),
            ))
                .unwrap()(
                dcgettext(
                    0 as *const libc::c_char,
                    b"close of pipe failed: %s\0" as *const u8 as *const libc::c_char,
                    5 as libc::c_int,
                ),
                strerror(*__errno_location()),
            );
        }
        signal(13 as libc::c_int, None);
        execl(
            b"/bin/sh\0" as *const u8 as *const libc::c_char,
            b"sh\0" as *const u8 as *const libc::c_char,
            b"-c\0" as *const u8 as *const libc::c_char,
            cmd,
            0 as *mut libc::c_void,
        );
        _exit(
            if *__errno_location() == 2 as libc::c_int {
                127 as libc::c_int
            } else {
                126 as libc::c_int
            },
        );
    }
    if pid == -(1 as libc::c_int) {
        close(p[0 as libc::c_int as usize]);
        close(p[1 as libc::c_int as usize]);
        (set_loc
            as unsafe extern "C" fn(
                *const libc::c_char,
                libc::c_int,
            ) -> ())(b"io.c\0" as *const u8 as *const libc::c_char, 2699 as libc::c_int);
        (Some(
            (Some(r_fatal as unsafe extern "C" fn(*const libc::c_char, ...) -> ()))
                .unwrap(),
        ))
            .unwrap()(
            dcgettext(
                0 as *const libc::c_char,
                b"cannot create child process for `%s' (fork: %s)\0" as *const u8
                    as *const libc::c_char,
                5 as libc::c_int,
            ),
            cmd,
            strerror(*__errno_location()),
        );
    }
    (*rp).pid = pid;
    if close(p[1 as libc::c_int as usize]) == -(1 as libc::c_int) {
        close(p[0 as libc::c_int as usize]);
        (set_loc
            as unsafe extern "C" fn(
                *const libc::c_char,
                libc::c_int,
            ) -> ())(b"io.c\0" as *const u8 as *const libc::c_char, 2705 as libc::c_int);
        (Some(
            (Some(r_fatal as unsafe extern "C" fn(*const libc::c_char, ...) -> ()))
                .unwrap(),
        ))
            .unwrap()(
            dcgettext(
                0 as *const libc::c_char,
                b"close of pipe failed: %s\0" as *const u8 as *const libc::c_char,
                5 as libc::c_int,
            ),
            strerror(*__errno_location()),
        );
    }
    os_close_on_exec(
        p[0 as libc::c_int as usize],
        cmd,
        b"pipe\0" as *const u8 as *const libc::c_char,
        b"from\0" as *const u8 as *const libc::c_char,
    );
    if BINMODE & BINMODE_INPUT as libc::c_int != 0 as libc::c_int {
        os_setbinmode(p[0 as libc::c_int as usize], 0 as libc::c_int);
    }
    (*rp).iop = iop_alloc(p[0 as libc::c_int as usize], cmd, 0 as libc::c_int);
    find_input_parser((*rp).iop);
    iop_finish((*rp).iop);
    if !(*(*rp).iop).valid {
        if do_flags as libc::c_uint & DO_TRADITIONAL as libc::c_int as libc::c_uint == 0
            && (*(*rp).iop).errcode != 0 as libc::c_int
        {
            update_ERRNO_int((*(*rp).iop).errcode);
        }
        iop_close((*rp).iop);
        (*rp).iop = 0 as *mut IOBUF;
    }
    return (*rp).iop;
}
unsafe extern "C" fn gawk_pclose(mut rp: *mut redirect) -> libc::c_int {
    if !((*rp).iop).is_null() {
        iop_close((*rp).iop);
    }
    (*rp).iop = 0 as *mut IOBUF;
    if (*rp).pid == -(1 as libc::c_int) {
        return (*rp).status;
    }
    (*rp).status = sanitize_exit_status(wait_any((*rp).pid));
    (*rp).pid = -(1 as libc::c_int);
    return (*rp).status;
}
pub unsafe extern "C" fn do_getline_redir(
    mut into_variable: libc::c_int,
    mut redirtype: redirval,
) -> *mut NODE {
    let mut rp: *mut redirect = 0 as *mut redirect;
    let mut iop: *mut IOBUF = 0 as *mut IOBUF;
    let mut cnt: size_t = 0;
    let mut retval: libc::c_int = -(1 as libc::c_int);
    let mut s: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut errcode: libc::c_int = 0;
    let mut redir_exp: *mut NODE = 0 as *mut NODE;
    let mut lhs: *mut *mut NODE = 0 as *mut *mut NODE;
    let mut redir_error: libc::c_int = 0 as libc::c_int;
    let mut field_width: *const awk_fieldwidth_info_t = 0
        as *const awk_fieldwidth_info_t;
    if into_variable != 0 {
        let fresh4 = stack_ptr;
        stack_ptr = stack_ptr.offset(-1);
        lhs = (*fresh4).lptr;
    }
    redir_exp = (*stack_ptr).rptr;
    rp = redirect(
        redir_exp,
        redirtype as libc::c_int,
        &mut redir_error,
        0 as libc::c_int != 0,
    );
    DEREF(redir_exp);
    stack_ptr = stack_ptr.offset(-1);
    stack_ptr;
    if rp.is_null() {
        if redir_error != 0 {
            if do_flags as libc::c_uint & DO_TRADITIONAL as libc::c_int as libc::c_uint
                == 0
            {
                update_ERRNO_int(redir_error);
            }
        }
        return make_number.unwrap()(-1.0f64);
    } else if (*rp).flag as libc::c_uint & RED_TWOWAY as libc::c_int as libc::c_uint
        != 0 as libc::c_int as libc::c_uint && ((*rp).iop).is_null()
    {
        if is_non_fatal_redirect((*redir_exp).sub.val.sp, (*redir_exp).sub.val.slen) {
            update_ERRNO_int(9 as libc::c_int);
            return make_number.unwrap()(-1.0f64);
        }
        close_rp(rp, CLOSE_ALL);
        (set_loc
            as unsafe extern "C" fn(
                *const libc::c_char,
                libc::c_int,
            ) -> ())(b"io.c\0" as *const u8 as *const libc::c_char, 2838 as libc::c_int);
        (Some(
            (Some(r_fatal as unsafe extern "C" fn(*const libc::c_char, ...) -> ()))
                .unwrap(),
        ))
            .unwrap()(
            dcgettext(
                0 as *const libc::c_char,
                b"getline: attempt to read from closed read end of two-way pipe\0"
                    as *const u8 as *const libc::c_char,
                5 as libc::c_int,
            ),
        );
    }
    iop = (*rp).iop;
    if iop.is_null() {
        return make_number.unwrap()(0.0f64);
    }
    errcode = 0 as libc::c_int;
    retval = get_a_record(
        &mut s,
        &mut cnt,
        iop,
        &mut errcode,
        if !lhs.is_null() {
            0 as *mut *const awk_fieldwidth_info_t
        } else {
            &mut field_width
        },
    );
    if errcode != 0 as libc::c_int {
        if do_flags as libc::c_uint & DO_TRADITIONAL as libc::c_int as libc::c_uint == 0
            && errcode != -(1 as libc::c_int)
        {
            update_ERRNO_int(errcode);
        }
        return make_number.unwrap()(retval as libc::c_double);
    }
    if retval == -(1 as libc::c_int) {
        if (*rp).flag as libc::c_uint
            & (RED_PIPE as libc::c_int | RED_TWOWAY as libc::c_int) as libc::c_uint
            == 0 as libc::c_int as libc::c_uint
        {
            iop_close(iop);
            (*rp).iop = 0 as *mut IOBUF;
        }
        (*rp)
            .flag = ::std::mem::transmute::<
            libc::c_uint,
            redirect_flags,
        >((*rp).flag as libc::c_uint | RED_EOF as libc::c_int as libc::c_uint);
        return make_number.unwrap()(0.0f64);
    }
    if lhs.is_null() {
        set_record(s, cnt, field_width);
    } else {
        unref(*lhs);
        *lhs = make_str_node(
            if !s.is_null() {
                s as *const libc::c_char
            } else {
                b"\0" as *const u8 as *const libc::c_char
            },
            cnt,
            0 as libc::c_int,
        );
        (**lhs)
            .flags = ::std::mem::transmute::<
            libc::c_uint,
            flagvals,
        >((**lhs).flags as libc::c_uint | USER_INPUT as libc::c_int as libc::c_uint);
    }
    return make_number.unwrap()(1.0f64);
}
pub unsafe extern "C" fn do_getline(
    mut into_variable: libc::c_int,
    mut iop: *mut IOBUF,
) -> *mut NODE {
    let mut cnt: size_t = 0;
    let mut retval: libc::c_int = -(1 as libc::c_int);
    let mut s: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut errcode: libc::c_int = 0;
    let mut field_width: *const awk_fieldwidth_info_t = 0
        as *const awk_fieldwidth_info_t;
    if iop.is_null() {
        if into_variable != 0 {
            stack_ptr = stack_ptr.offset(-1);
            stack_ptr;
        }
        return make_number.unwrap()(0.0f64);
    }
    errcode = 0 as libc::c_int;
    retval = get_a_record(
        &mut s,
        &mut cnt,
        iop,
        &mut errcode,
        if into_variable != 0 {
            0 as *mut *const awk_fieldwidth_info_t
        } else {
            &mut field_width
        },
    );
    if errcode != 0 as libc::c_int {
        if do_flags as libc::c_uint & DO_TRADITIONAL as libc::c_int as libc::c_uint == 0
            && errcode != -(1 as libc::c_int)
        {
            update_ERRNO_int(errcode);
        }
        if into_variable != 0 {
            stack_ptr = stack_ptr.offset(-1);
            stack_ptr;
        }
        return make_number.unwrap()(retval as libc::c_double);
    }
    if retval == -(1 as libc::c_int) {
        return 0 as *mut NODE;
    }
    NR += 1;
    NR;
    FNR += 1;
    FNR;
    if into_variable == 0 {
        set_record(s, cnt, field_width);
    } else {
        let mut lhs: *mut *mut NODE = 0 as *mut *mut NODE;
        let fresh5 = stack_ptr;
        stack_ptr = stack_ptr.offset(-1);
        lhs = (*fresh5).lptr;
        unref(*lhs);
        *lhs = make_str_node(
            if !s.is_null() {
                s as *const libc::c_char
            } else {
                b"\0" as *const u8 as *const libc::c_char
            },
            cnt,
            0 as libc::c_int,
        );
        (**lhs)
            .flags = ::std::mem::transmute::<
            libc::c_uint,
            flagvals,
        >((**lhs).flags as libc::c_uint | USER_INPUT as libc::c_int as libc::c_uint);
    }
    return make_number.unwrap()(1.0f64);
}
static mut pi_awkpath: path_info = unsafe {
    {
        let mut init = path_info {
            envname: b"AWKPATH\0" as *const u8 as *const libc::c_char,
            dfltp: &defpath as *const *const libc::c_char as *mut *const libc::c_char,
            awkpath: 0 as *const *const libc::c_char as *mut *const libc::c_char,
            max_pathlen: 0,
        };
        init
    }
};
static mut pi_awklibpath: path_info = unsafe {
    {
        let mut init = path_info {
            envname: b"AWKLIBPATH\0" as *const u8 as *const libc::c_char,
            dfltp: &deflibpath as *const *const libc::c_char as *mut *const libc::c_char,
            awkpath: 0 as *const *const libc::c_char as *mut *const libc::c_char,
            max_pathlen: 0,
        };
        init
    }
};
unsafe extern "C" fn init_awkpath(mut pi: *mut path_info) {
    let mut path: *const libc::c_char = 0 as *const libc::c_char;
    let mut start: *const libc::c_char = 0 as *const libc::c_char;
    let mut end: *const libc::c_char = 0 as *const libc::c_char;
    let mut p: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut len: libc::c_int = 0;
    let mut i: libc::c_int = 0;
    let mut max_path: libc::c_int = 0;
    (*pi).max_pathlen = 0 as libc::c_int;
    path = getenv((*pi).envname);
    if path.is_null() || *path as libc::c_int == '\0' as i32 {
        path = *((*pi).dfltp).offset(0 as libc::c_int as isize);
    }
    max_path = 0 as libc::c_int;
    p = path as *mut libc::c_char;
    while *p != 0 {
        if *p as libc::c_int == envsep as libc::c_int {
            max_path += 1;
            max_path;
        }
        p = p.offset(1);
        p;
    }
    (*pi)
        .awkpath = ezalloc_real(
        ((max_path + 3 as libc::c_int) as libc::c_ulong)
            .wrapping_mul(::std::mem::size_of::<*mut libc::c_char>() as libc::c_ulong),
        b"init_awkpath\0" as *const u8 as *const libc::c_char,
        b"pi->awkpath\0" as *const u8 as *const libc::c_char,
        b"io.c\0" as *const u8 as *const libc::c_char,
        2963 as libc::c_int,
    ) as *mut *const libc::c_char;
    start = path;
    i = 0 as libc::c_int;
    if *path as libc::c_int == envsep as libc::c_int {
        let fresh6 = i;
        i = i + 1;
        let ref mut fresh7 = *((*pi).awkpath).offset(fresh6 as isize);
        *fresh7 = b".\0" as *const u8 as *const libc::c_char;
        (*pi).max_pathlen = 1 as libc::c_int;
    }
    while *start != 0 {
        if *start as libc::c_int == envsep as libc::c_int {
            if *start.offset(1 as libc::c_int as isize) as libc::c_int
                == envsep as libc::c_int
            {
                let fresh8 = i;
                i = i + 1;
                let ref mut fresh9 = *((*pi).awkpath).offset(fresh8 as isize);
                *fresh9 = b".\0" as *const u8 as *const libc::c_char;
                if (*pi).max_pathlen == 0 as libc::c_int {
                    (*pi).max_pathlen = 1 as libc::c_int;
                }
                start = start.offset(1);
                start;
            } else if *start.offset(1 as libc::c_int as isize) as libc::c_int
                == '\0' as i32
            {
                let fresh10 = i;
                i = i + 1;
                let ref mut fresh11 = *((*pi).awkpath).offset(fresh10 as isize);
                *fresh11 = b".\0" as *const u8 as *const libc::c_char;
                if (*pi).max_pathlen == 0 as libc::c_int {
                    (*pi).max_pathlen = 1 as libc::c_int;
                }
                break;
            } else {
                start = start.offset(1);
                start;
            }
        } else {
            end = start;
            while *end as libc::c_int != 0
                && *end as libc::c_int != envsep as libc::c_int
            {
                end = end.offset(1);
                end;
            }
            len = end.offset_from(start) as libc::c_long as libc::c_int;
            if len > 0 as libc::c_int {
                p = emalloc_real(
                    (len + 2 as libc::c_int) as size_t,
                    b"init_awkpath\0" as *const u8 as *const libc::c_char,
                    b"p\0" as *const u8 as *const libc::c_char,
                    b"io.c\0" as *const u8 as *const libc::c_char,
                    2993 as libc::c_int,
                ) as *mut libc::c_char;
                memcpy(
                    p as *mut libc::c_void,
                    start as *const libc::c_void,
                    len as libc::c_ulong,
                );
                if isdirpunct(*end.offset(-(1 as libc::c_int) as isize) as libc::c_int)
                    == 0
                {
                    let fresh12 = len;
                    len = len + 1;
                    *p.offset(fresh12 as isize) = '/' as i32 as libc::c_char;
                }
                *p.offset(len as isize) = '\0' as i32 as libc::c_char;
                let fresh13 = i;
                i = i + 1;
                let ref mut fresh14 = *((*pi).awkpath).offset(fresh13 as isize);
                *fresh14 = p;
                if len > (*pi).max_pathlen {
                    (*pi).max_pathlen = len;
                }
                start = end;
            } else {
                start = start.offset(1);
                start;
            }
        }
    }
    let ref mut fresh15 = *((*pi).awkpath).offset(i as isize);
    *fresh15 = 0 as *const libc::c_char;
}
unsafe extern "C" fn do_find_source(
    mut src: *const libc::c_char,
    mut stb: *mut stat,
    mut errcode: *mut libc::c_int,
    mut pi: *mut path_info,
) -> *mut libc::c_char {
    let mut path: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut i: libc::c_int = 0;
    if ispath(src) != 0 {
        path = emalloc_real(
            (strlen(src)).wrapping_add(1 as libc::c_int as libc::c_ulong),
            b"do_find_source\0" as *const u8 as *const libc::c_char,
            b"path\0" as *const u8 as *const libc::c_char,
            b"io.c\0" as *const u8 as *const libc::c_char,
            3025 as libc::c_int,
        ) as *mut libc::c_char;
        strcpy(path, src);
        if stat(path, stb) == 0 as libc::c_int {
            return path;
        }
        *errcode = *__errno_location();
        free(path as *mut libc::c_void);
        return 0 as *mut libc::c_char;
    }
    if ((*pi).awkpath).is_null() {
        init_awkpath(pi);
    }
    path = emalloc_real(
        ((*pi).max_pathlen as libc::c_ulong)
            .wrapping_add(strlen(src))
            .wrapping_add(1 as libc::c_int as libc::c_ulong),
        b"do_find_source\0" as *const u8 as *const libc::c_char,
        b"path\0" as *const u8 as *const libc::c_char,
        b"io.c\0" as *const u8 as *const libc::c_char,
        3037 as libc::c_int,
    ) as *mut libc::c_char;
    i = 0 as libc::c_int;
    while !(*((*pi).awkpath).offset(i as isize)).is_null() {
        if strcmp(
            *((*pi).awkpath).offset(i as isize),
            b"./\0" as *const u8 as *const libc::c_char,
        ) == 0 as libc::c_int
            || strcmp(
                *((*pi).awkpath).offset(i as isize),
                b".\0" as *const u8 as *const libc::c_char,
            ) == 0 as libc::c_int
        {
            *path = '\0' as i32 as libc::c_char;
        } else {
            strcpy(path, *((*pi).awkpath).offset(i as isize));
        }
        strcat(path, src);
        if stat(path, stb) == 0 as libc::c_int {
            return path;
        }
        i += 1;
        i;
    }
    *errcode = *__errno_location();
    free(path as *mut libc::c_void);
    return 0 as *mut libc::c_char;
}
pub unsafe extern "C" fn find_source(
    mut src: *const libc::c_char,
    mut stb: *mut stat,
    mut errcode: *mut libc::c_int,
    mut is_extlib: libc::c_int,
) -> *mut libc::c_char {
    let mut path: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut pi: *mut path_info = if is_extlib != 0 {
        &mut pi_awklibpath
    } else {
        &mut pi_awkpath
    };
    *errcode = 0 as libc::c_int;
    if src.is_null() || *src as libc::c_int == '\0' as i32 {
        return 0 as *mut libc::c_char;
    }
    path = do_find_source(src, stb, errcode, pi);
    if path.is_null() && is_extlib != 0 {
        let mut file_ext: *mut libc::c_char = 0 as *mut libc::c_char;
        let mut save_errno: libc::c_int = 0;
        let mut src_len: size_t = 0;
        let mut suffix_len: size_t = 0;
        src_len = strlen(src);
        suffix_len = strlen(b".so\0" as *const u8 as *const libc::c_char);
        if src_len >= suffix_len
            && strcmp(
                &*src.offset(src_len.wrapping_sub(suffix_len) as isize),
                b".so\0" as *const u8 as *const libc::c_char,
            ) == 0 as libc::c_int
        {
            return 0 as *mut libc::c_char;
        }
        save_errno = *__errno_location();
        file_ext = emalloc_real(
            src_len
                .wrapping_add(suffix_len)
                .wrapping_add(1 as libc::c_int as libc::c_ulong),
            b"find_source\0" as *const u8 as *const libc::c_char,
            b"file_ext\0" as *const u8 as *const libc::c_char,
            b"io.c\0" as *const u8 as *const libc::c_char,
            3083 as libc::c_int,
        ) as *mut libc::c_char;
        sprintf(
            file_ext,
            b"%s%s\0" as *const u8 as *const libc::c_char,
            src,
            b".so\0" as *const u8 as *const libc::c_char,
        );
        path = do_find_source(file_ext, stb, errcode, pi);
        free(file_ext as *mut libc::c_void);
        if path.is_null() {
            *__errno_location() = save_errno;
        }
        return path;
    }
    if do_flags as libc::c_uint & DO_TRADITIONAL as libc::c_int as libc::c_uint == 0
        && path.is_null()
    {
        let mut file_awk: *mut libc::c_char = 0 as *mut libc::c_char;
        let mut save_errno_0: libc::c_int = *__errno_location();
        file_awk = emalloc_real(
            (strlen(src))
                .wrapping_add(
                    ::std::mem::size_of::<[libc::c_char; 5]>() as libc::c_ulong,
                )
                .wrapping_add(1 as libc::c_int as libc::c_ulong),
            b"find_source\0" as *const u8 as *const libc::c_char,
            b"file_awk\0" as *const u8 as *const libc::c_char,
            b"io.c\0" as *const u8 as *const libc::c_char,
            3111 as libc::c_int,
        ) as *mut libc::c_char;
        sprintf(
            file_awk,
            b"%s%s\0" as *const u8 as *const libc::c_char,
            src,
            b".awk\0" as *const u8 as *const libc::c_char,
        );
        path = do_find_source(file_awk, stb, errcode, pi);
        free(file_awk as *mut libc::c_void);
        if path.is_null() {
            *__errno_location() = save_errno_0;
        }
    }
    return path;
}
pub unsafe extern "C" fn srcopen(mut s: *mut SRCFILE) -> libc::c_int {
    let mut fd: libc::c_int = -(1 as libc::c_int);
    if (*s).stype as libc::c_uint == SRC_STDIN as libc::c_int as libc::c_uint {
        fd = fileno(stdin);
    } else if (*s).stype as libc::c_uint == SRC_FILE as libc::c_int as libc::c_uint
        || (*s).stype as libc::c_uint == SRC_INC as libc::c_int as libc::c_uint
    {
        fd = devopen((*s).fullpath, b"r\0" as *const u8 as *const libc::c_char);
    }
    if fd != -(1 as libc::c_int) {
        os_setbinmode(fd, 0 as libc::c_int);
    }
    return fd;
}
static mut ip_head: *mut awk_input_parser_t = 0 as *const awk_input_parser_t
    as *mut awk_input_parser_t;
static mut ip_tail: *mut awk_input_parser_t = 0 as *const awk_input_parser_t
    as *mut awk_input_parser_t;
pub unsafe extern "C" fn register_input_parser(
    mut input_parser: *mut awk_input_parser_t,
) {
    if input_parser.is_null() {
        (set_loc
            as unsafe extern "C" fn(
                *const libc::c_char,
                libc::c_int,
            ) -> ())(b"io.c\0" as *const u8 as *const libc::c_char, 3162 as libc::c_int);
        (Some(
            (Some(r_fatal as unsafe extern "C" fn(*const libc::c_char, ...) -> ()))
                .unwrap(),
        ))
            .unwrap()(
            dcgettext(
                0 as *const libc::c_char,
                b"register_input_parser: received NULL pointer\0" as *const u8
                    as *const libc::c_char,
                5 as libc::c_int,
            ),
        );
    }
    (*input_parser).next = 0 as *mut awk_input_parser;
    if ip_head.is_null() {
        ip_tail = input_parser;
        ip_head = ip_tail;
    } else {
        (*ip_tail).next = input_parser;
        ip_tail = (*ip_tail).next;
    };
}
unsafe extern "C" fn find_input_parser(mut iop: *mut IOBUF) {
    let mut ip: *mut awk_input_parser_t = 0 as *mut awk_input_parser_t;
    let mut ip2: *mut awk_input_parser_t = 0 as *mut awk_input_parser_t;
    if ((*iop).public.get_record).is_some()
        || (*iop).public.read_func
            != Some(
                read
                    as unsafe extern "C" fn(
                        libc::c_int,
                        *mut libc::c_void,
                        size_t,
                    ) -> ssize_t,
            )
    {
        return;
    }
    ip2 = 0 as *mut awk_input_parser_t;
    ip = ip2;
    ip2 = ip_head;
    while !ip2.is_null() {
        if ((*ip2).can_take_file).unwrap()(&mut (*iop).public) as u64 != 0 {
            if ip.is_null() {
                ip = ip2;
            } else {
                (set_loc
                    as unsafe extern "C" fn(
                        *const libc::c_char,
                        libc::c_int,
                    ) -> ())(
                    b"io.c\0" as *const u8 as *const libc::c_char,
                    3190 as libc::c_int,
                );
                (Some(
                    (Some(
                        r_fatal as unsafe extern "C" fn(*const libc::c_char, ...) -> (),
                    ))
                        .unwrap(),
                ))
                    .unwrap()(
                    dcgettext(
                        0 as *const libc::c_char,
                        b"input parser `%s' conflicts with previously installed input parser `%s'\0"
                            as *const u8 as *const libc::c_char,
                        5 as libc::c_int,
                    ),
                    (*ip2).name,
                    (*ip).name,
                );
            }
        }
        ip2 = (*ip2).next;
    }
    if !ip.is_null() {
        if ((*ip).take_control_of).unwrap()(&mut (*iop).public) as u64 == 0 {
            (set_loc
                as unsafe extern "C" fn(
                    *const libc::c_char,
                    libc::c_int,
                ) -> ())(
                b"io.c\0" as *const u8 as *const libc::c_char,
                3197 as libc::c_int,
            );
            (Some(
                (Some(r_warning as unsafe extern "C" fn(*const libc::c_char, ...) -> ()))
                    .unwrap(),
            ))
                .unwrap()(
                dcgettext(
                    0 as *const libc::c_char,
                    b"input parser `%s' failed to open `%s'\0" as *const u8
                        as *const libc::c_char,
                    5 as libc::c_int,
                ),
                (*ip).name,
                (*iop).public.name,
            );
        } else {
            (*iop).valid = 1 as libc::c_int != 0;
        }
    }
}
static mut op_head: *mut awk_output_wrapper_t = 0 as *const awk_output_wrapper_t
    as *mut awk_output_wrapper_t;
static mut op_tail: *mut awk_output_wrapper_t = 0 as *const awk_output_wrapper_t
    as *mut awk_output_wrapper_t;
pub unsafe extern "C" fn register_output_wrapper(
    mut wrapper: *mut awk_output_wrapper_t,
) {
    if wrapper.is_null() {
        (set_loc
            as unsafe extern "C" fn(
                *const libc::c_char,
                libc::c_int,
            ) -> ())(b"io.c\0" as *const u8 as *const libc::c_char, 3217 as libc::c_int);
        (Some(
            (Some(r_fatal as unsafe extern "C" fn(*const libc::c_char, ...) -> ()))
                .unwrap(),
        ))
            .unwrap()(
            dcgettext(
                0 as *const libc::c_char,
                b"register_output_wrapper: received NULL pointer\0" as *const u8
                    as *const libc::c_char,
                5 as libc::c_int,
            ),
        );
    }
    (*wrapper).next = 0 as *mut awk_output_wrapper;
    if op_head.is_null() {
        op_tail = wrapper;
        op_head = op_tail;
    } else {
        (*op_tail).next = wrapper;
        op_tail = (*op_tail).next;
    };
}
unsafe extern "C" fn find_output_wrapper(mut outbuf: *mut awk_output_buf_t) -> bool {
    let mut op: *mut awk_output_wrapper_t = 0 as *mut awk_output_wrapper_t;
    let mut op2: *mut awk_output_wrapper_t = 0 as *mut awk_output_wrapper_t;
    if (*outbuf).redirected as u64 != 0 {
        return 0 as libc::c_int != 0;
    }
    op2 = 0 as *mut awk_output_wrapper_t;
    op = op2;
    op2 = op_head;
    while !op2.is_null() {
        if ((*op2).can_take_file).unwrap()(outbuf) as u64 != 0 {
            if op.is_null() {
                op = op2;
            } else {
                (set_loc
                    as unsafe extern "C" fn(
                        *const libc::c_char,
                        libc::c_int,
                    ) -> ())(
                    b"io.c\0" as *const u8 as *const libc::c_char,
                    3245 as libc::c_int,
                );
                (Some(
                    (Some(
                        r_fatal as unsafe extern "C" fn(*const libc::c_char, ...) -> (),
                    ))
                        .unwrap(),
                ))
                    .unwrap()(
                    dcgettext(
                        0 as *const libc::c_char,
                        b"output wrapper `%s' conflicts with previously installed output wrapper `%s'\0"
                            as *const u8 as *const libc::c_char,
                        5 as libc::c_int,
                    ),
                    (*op2).name,
                    (*op).name,
                );
            }
        }
        op2 = (*op2).next;
    }
    if !op.is_null() {
        if ((*op).take_control_of).unwrap()(outbuf) as u64 == 0 {
            (set_loc
                as unsafe extern "C" fn(
                    *const libc::c_char,
                    libc::c_int,
                ) -> ())(
                b"io.c\0" as *const u8 as *const libc::c_char,
                3252 as libc::c_int,
            );
            (Some(
                (Some(r_warning as unsafe extern "C" fn(*const libc::c_char, ...) -> ()))
                    .unwrap(),
            ))
                .unwrap()(
                dcgettext(
                    0 as *const libc::c_char,
                    b"output wrapper `%s' failed to open `%s'\0" as *const u8
                        as *const libc::c_char,
                    5 as libc::c_int,
                ),
                (*op).name,
                (*outbuf).name,
            );
            return 0 as libc::c_int != 0;
        }
        return 1 as libc::c_int != 0;
    }
    return 0 as libc::c_int != 0;
}
static mut tw_head: *mut awk_two_way_processor_t = 0 as *const awk_two_way_processor_t
    as *mut awk_two_way_processor_t;
static mut tw_tail: *mut awk_two_way_processor_t = 0 as *const awk_two_way_processor_t
    as *mut awk_two_way_processor_t;
pub unsafe extern "C" fn register_two_way_processor(
    mut processor: *mut awk_two_way_processor_t,
) {
    if processor.is_null() {
        (set_loc
            as unsafe extern "C" fn(
                *const libc::c_char,
                libc::c_int,
            ) -> ())(b"io.c\0" as *const u8 as *const libc::c_char, 3273 as libc::c_int);
        (Some(
            (Some(r_fatal as unsafe extern "C" fn(*const libc::c_char, ...) -> ()))
                .unwrap(),
        ))
            .unwrap()(
            dcgettext(
                0 as *const libc::c_char,
                b"register_output_processor: received NULL pointer\0" as *const u8
                    as *const libc::c_char,
                5 as libc::c_int,
            ),
        );
    }
    (*processor).next = 0 as *mut awk_two_way_processor;
    if tw_head.is_null() {
        tw_tail = processor;
        tw_head = tw_tail;
    } else {
        (*tw_tail).next = processor;
        tw_tail = (*tw_tail).next;
    };
}
unsafe extern "C" fn find_two_way_processor(
    mut name: *const libc::c_char,
    mut rp: *mut redirect,
) -> bool {
    let mut tw: *mut awk_two_way_processor_t = 0 as *mut awk_two_way_processor_t;
    let mut tw2: *mut awk_two_way_processor_t = 0 as *mut awk_two_way_processor_t;
    if !((*rp).iop).is_null() && (*(*rp).iop).public.fd != -(1 as libc::c_int)
        || !((*rp).output.fp).is_null()
    {
        return 0 as libc::c_int != 0;
    }
    tw2 = 0 as *mut awk_two_way_processor_t;
    tw = tw2;
    tw2 = tw_head;
    while !tw2.is_null() {
        if ((*tw2).can_take_two_way).unwrap()(name) as u64 != 0 {
            if tw.is_null() {
                tw = tw2;
            } else {
                (set_loc
                    as unsafe extern "C" fn(
                        *const libc::c_char,
                        libc::c_int,
                    ) -> ())(
                    b"io.c\0" as *const u8 as *const libc::c_char,
                    3302 as libc::c_int,
                );
                (Some(
                    (Some(
                        r_fatal as unsafe extern "C" fn(*const libc::c_char, ...) -> (),
                    ))
                        .unwrap(),
                ))
                    .unwrap()(
                    dcgettext(
                        0 as *const libc::c_char,
                        b"two-way processor `%s' conflicts with previously installed two-way processor `%s'\0"
                            as *const u8 as *const libc::c_char,
                        5 as libc::c_int,
                    ),
                    (*tw2).name,
                    (*tw).name,
                );
            }
        }
        tw2 = (*tw2).next;
    }
    if !tw.is_null() {
        if ((*rp).iop).is_null() {
            (*rp).iop = iop_alloc(-(1 as libc::c_int), name, 0 as libc::c_int);
        }
        if ((*tw).take_control_of)
            .unwrap()(name, &mut (*(*rp).iop).public, &mut (*rp).output) as u64 == 0
        {
            (set_loc
                as unsafe extern "C" fn(
                    *const libc::c_char,
                    libc::c_int,
                ) -> ())(
                b"io.c\0" as *const u8 as *const libc::c_char,
                3311 as libc::c_int,
            );
            (Some(
                (Some(r_warning as unsafe extern "C" fn(*const libc::c_char, ...) -> ()))
                    .unwrap(),
            ))
                .unwrap()(
                dcgettext(
                    0 as *const libc::c_char,
                    b"two way processor `%s' failed to open `%s'\0" as *const u8
                        as *const libc::c_char,
                    5 as libc::c_int,
                ),
                (*tw).name,
                name,
            );
            return 0 as libc::c_int != 0;
        }
        iop_finish((*rp).iop);
        return 1 as libc::c_int != 0;
    }
    return 0 as libc::c_int != 0;
}
unsafe extern "C" fn iop_alloc(
    mut fd: libc::c_int,
    mut name: *const libc::c_char,
    mut errno_val: libc::c_int,
) -> *mut IOBUF {
    let mut iop: *mut IOBUF = 0 as *mut IOBUF;
    iop = ezalloc_real(
        ::std::mem::size_of::<IOBUF>() as libc::c_ulong,
        b"iop_alloc\0" as *const u8 as *const libc::c_char,
        b"iop\0" as *const u8 as *const libc::c_char,
        b"io.c\0" as *const u8 as *const libc::c_char,
        3370 as libc::c_int,
    ) as *mut IOBUF;
    (*iop).public.fd = fd;
    (*iop).public.name = name;
    (*iop)
        .public
        .read_func = ::std::mem::transmute::<
        Option::<unsafe extern "C" fn() -> ssize_t>,
        Option::<unsafe extern "C" fn(libc::c_int, *mut libc::c_void, size_t) -> ssize_t>,
    >(
        ::std::mem::transmute::<
            Option::<
                unsafe extern "C" fn(libc::c_int, *mut libc::c_void, size_t) -> ssize_t,
            >,
            Option::<unsafe extern "C" fn() -> ssize_t>,
        >(
            Some(
                read
                    as unsafe extern "C" fn(
                        libc::c_int,
                        *mut libc::c_void,
                        size_t,
                    ) -> ssize_t,
            ),
        ),
    );
    (*iop).valid = 0 as libc::c_int != 0;
    (*iop).errcode = errno_val;
    if fd != -(1 as libc::c_int) {
        fstat(fd, &mut (*iop).public.sbuf);
    } else {
        let mut statf: Option::<
            unsafe extern "C" fn(*const libc::c_char, *mut stat) -> libc::c_int,
        > = Some(
            lstat as unsafe extern "C" fn(*const libc::c_char, *mut stat) -> libc::c_int,
        );
        if statf.unwrap()(name, &mut (*iop).public.sbuf) < 0 as libc::c_int {
            memset(
                &mut (*iop).public.sbuf as *mut stat as *mut libc::c_void,
                0 as libc::c_int,
                ::std::mem::size_of::<stat>() as libc::c_ulong,
            );
        }
    }
    return iop;
}
unsafe extern "C" fn iop_finish(mut iop: *mut IOBUF) -> *mut IOBUF {
    let mut isdir: bool = 0 as libc::c_int != 0;
    if (*iop).public.fd != -(1 as libc::c_int) {
        if os_isreadable(&mut (*iop).public, &mut isdir) != 0 {
            (*iop).valid = 1 as libc::c_int != 0;
        } else if isdir {
            (*iop).errcode = 21 as libc::c_int;
        } else {
            (*iop).errcode = 5 as libc::c_int;
            if fcntl((*iop).public.fd, 3 as libc::c_int) >= 0 as libc::c_int {
                close((*iop).public.fd);
            }
            (*iop).public.fd = -(1 as libc::c_int);
        }
    }
    if !(*iop).valid || (*iop).public.fd == -(1 as libc::c_int) {
        return iop;
    }
    if os_isatty((*iop).public.fd) != 0 {
        (*iop)
            .flag = ::std::mem::transmute::<
            libc::c_uint,
            iobuf_flags,
        >((*iop).flag as libc::c_uint | IOP_IS_TTY as libc::c_int as libc::c_uint);
    }
    (*iop).size = optimal_bufsize((*iop).public.fd, &mut (*iop).public.sbuf);
    (*iop).readsize = (*iop).size;
    if do_flags as libc::c_uint
        & (DO_LINT_INVALID as libc::c_int | DO_LINT_ALL as libc::c_int) as libc::c_uint
        != 0
        && (*iop).public.sbuf.st_mode & 0o170000 as libc::c_int as libc::c_uint
            == 0o100000 as libc::c_int as libc::c_uint
        && (*iop).public.sbuf.st_size == 0 as libc::c_int as libc::c_long
    {
        (set_loc
            as unsafe extern "C" fn(
                *const libc::c_char,
                libc::c_int,
            ) -> ())(b"io.c\0" as *const u8 as *const libc::c_char, 3442 as libc::c_int);
        (Some(lintfunc.unwrap()))
            .unwrap()(
            dcgettext(
                0 as *const libc::c_char,
                b"data file `%s' is empty\0" as *const u8 as *const libc::c_char,
                5 as libc::c_int,
            ),
            (*iop).public.name,
        );
    }
    let ref mut fresh16 = *__errno_location();
    *fresh16 = 0 as libc::c_int;
    (*iop).errcode = *fresh16;
    (*iop).scanoff = 0 as libc::c_int as size_t;
    (*iop).count = (*iop).scanoff as ssize_t;
    (*iop)
        .size = ((*iop).size as libc::c_ulong)
        .wrapping_add(1 as libc::c_int as libc::c_ulong) as size_t as size_t;
    (*iop)
        .buf = emalloc_real(
        (*iop).size,
        b"iop_finish\0" as *const u8 as *const libc::c_char,
        b"iop->buf\0" as *const u8 as *const libc::c_char,
        b"io.c\0" as *const u8 as *const libc::c_char,
        3445 as libc::c_int,
    ) as *mut libc::c_char;
    (*iop).off = (*iop).buf;
    (*iop).dataend = 0 as *mut libc::c_char;
    (*iop).end = ((*iop).buf).offset((*iop).size as isize);
    (*iop)
        .flag = ::std::mem::transmute::<
        libc::c_uint,
        iobuf_flags,
    >((*iop).flag as libc::c_uint | IOP_AT_START as libc::c_int as libc::c_uint);
    return iop;
}
unsafe extern "C" fn grow_iop_buffer(mut iop: *mut IOBUF) {
    let mut valid: size_t = ((*iop).dataend).offset_from((*iop).off) as libc::c_long
        as size_t;
    let mut off: size_t = ((*iop).off).offset_from((*iop).buf) as libc::c_long as size_t;
    let mut newsize: size_t = 0;
    newsize = ((*iop).size)
        .wrapping_sub(1 as libc::c_int as libc::c_ulong)
        .wrapping_mul(2 as libc::c_int as libc::c_ulong)
        .wrapping_add(1 as libc::c_int as libc::c_ulong);
    if newsize <= (*iop).size {
        (set_loc
            as unsafe extern "C" fn(
                *const libc::c_char,
                libc::c_int,
            ) -> ())(b"io.c\0" as *const u8 as *const libc::c_char, 3484 as libc::c_int);
        (Some(
            (Some(r_fatal as unsafe extern "C" fn(*const libc::c_char, ...) -> ()))
                .unwrap(),
        ))
            .unwrap()(
            dcgettext(
                0 as *const libc::c_char,
                b"could not allocate more input memory\0" as *const u8
                    as *const libc::c_char,
                5 as libc::c_int,
            ),
        );
    }
    if newsize.wrapping_sub(valid) < (*iop).readsize {
        newsize = (newsize as libc::c_ulong)
            .wrapping_add(
                ((*iop).readsize).wrapping_add(1 as libc::c_int as libc::c_ulong),
            ) as size_t as size_t;
    }
    if newsize <= (*iop).size {
        (set_loc
            as unsafe extern "C" fn(
                *const libc::c_char,
                libc::c_int,
            ) -> ())(b"io.c\0" as *const u8 as *const libc::c_char, 3492 as libc::c_int);
        (Some(
            (Some(r_fatal as unsafe extern "C" fn(*const libc::c_char, ...) -> ()))
                .unwrap(),
        ))
            .unwrap()(
            dcgettext(
                0 as *const libc::c_char,
                b"could not allocate more input memory\0" as *const u8
                    as *const libc::c_char,
                5 as libc::c_int,
            ),
        );
    }
    (*iop).size = newsize;
    (*iop)
        .buf = erealloc_real(
        (*iop).buf as *mut libc::c_void,
        (*iop).size,
        b"grow_iop_buffer\0" as *const u8 as *const libc::c_char,
        b"iop->buf\0" as *const u8 as *const libc::c_char,
        b"io.c\0" as *const u8 as *const libc::c_char,
        3495 as libc::c_int,
    ) as *mut libc::c_char;
    (*iop).off = ((*iop).buf).offset(off as isize);
    (*iop).dataend = ((*iop).off).offset(valid as isize);
    (*iop).end = ((*iop).buf).offset((*iop).size as isize);
}
unsafe extern "C" fn rs1scan(
    mut iop: *mut IOBUF,
    mut recm: *mut recmatch,
    mut state: *mut SCANSTATE,
) -> RECVALUE {
    let mut bp: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut rs: libc::c_char = 0;
    let mut mbclen: size_t = 0 as libc::c_int as size_t;
    let mut mbs: mbstate_t = mbstate_t {
        __count: 0,
        __value: C2RustUnnamed { __wch: 0 },
    };
    memset(
        recm as *mut libc::c_void,
        '\0' as i32,
        ::std::mem::size_of::<recmatch>() as libc::c_ulong,
    );
    rs = *((*RS).sub.val.sp).offset(0 as libc::c_int as isize);
    *(*iop).dataend = rs;
    (*recm).start = (*iop).off;
    bp = (*iop).off;
    if *state as libc::c_uint == INDATA as libc::c_int as libc::c_uint {
        bp = bp.offset((*iop).scanoff as isize);
    }
    if rs as libc::c_int != '\n' as i32 && gawk_mb_cur_max > 1 as libc::c_int {
        let mut len: libc::c_int = ((*iop).dataend).offset_from(bp) as libc::c_long
            as libc::c_int;
        let mut found: bool = 0 as libc::c_int != 0;
        memset(
            &mut mbs as *mut mbstate_t as *mut libc::c_void,
            0 as libc::c_int,
            ::std::mem::size_of::<mbstate_t>() as libc::c_ulong,
        );
        loop {
            if *bp as libc::c_int == rs as libc::c_int {
                found = 1 as libc::c_int != 0;
            }
            if *btowc_cache
                .as_mut_ptr()
                .offset((*bp as libc::c_int & 0xff as libc::c_int) as isize)
                != 0xffffffff as libc::c_uint
            {
                mbclen = 1 as libc::c_int as size_t;
            } else {
                mbclen = mbrlen(bp, len as size_t, &mut mbs);
            }
            if mbclen == 1 as libc::c_int as libc::c_ulong
                || mbclen == -(1 as libc::c_int) as size_t
                || mbclen == -(2 as libc::c_int) as size_t
                || mbclen == 0 as libc::c_int as libc::c_ulong
            {
                mbclen = 1 as libc::c_int as size_t;
            }
            len = (len as libc::c_ulong).wrapping_sub(mbclen) as libc::c_int
                as libc::c_int;
            bp = bp.offset(mbclen as isize);
            if !(len > 0 as libc::c_int && !found) {
                break;
            }
        }
        if found as libc::c_int != 0 && bp.offset(-(mbclen as isize)) < (*iop).dataend {
            (*recm)
                .len = (bp.offset_from((*recm).start) as libc::c_long as libc::c_ulong)
                .wrapping_sub(mbclen);
            (*recm).rt_start = bp.offset(-(mbclen as isize));
            (*recm).rt_len = mbclen;
            *state = NOSTATE;
            return REC_OK;
        } else {
            (*recm).len = bp.offset_from((*recm).start) as libc::c_long as size_t;
            *state = INDATA;
            (*iop).scanoff = bp.offset_from((*iop).off) as libc::c_long as size_t;
            return NOTERM;
        }
    }
    while *bp as libc::c_int != rs as libc::c_int {
        bp = bp.offset(1);
        bp;
    }
    (*recm).len = bp.offset_from((*recm).start) as libc::c_long as size_t;
    if bp < (*iop).dataend {
        (*recm).rt_start = bp;
        (*recm).rt_len = 1 as libc::c_int as size_t;
        *state = NOSTATE;
        return REC_OK;
    } else {
        *state = INDATA;
        (*iop).scanoff = bp.offset_from((*iop).off) as libc::c_long as size_t;
        return NOTERM;
    };
}
unsafe extern "C" fn rsrescan(
    mut iop: *mut IOBUF,
    mut recm: *mut recmatch,
    mut state: *mut SCANSTATE,
) -> RECVALUE {
    let mut bp: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut restart: size_t = 0 as libc::c_int as size_t;
    let mut reend: size_t = 0 as libc::c_int as size_t;
    let mut RSre: *mut Regexp = RS_regexp;
    let mut regex_flags: libc::c_int = 1 as libc::c_int;
    memset(
        recm as *mut libc::c_void,
        '\0' as i32,
        ::std::mem::size_of::<recmatch>() as libc::c_ulong,
    );
    (*recm).start = (*iop).off;
    bp = (*iop).off;
    if *state as libc::c_uint == INDATA as libc::c_int as libc::c_uint {
        bp = bp.offset((*iop).scanoff as isize);
    }
    if (*iop).flag as libc::c_uint & IOP_AT_START as libc::c_int as libc::c_uint
        == 0 as libc::c_int as libc::c_uint
    {
        regex_flags |= 2 as libc::c_int;
    }
    loop {
        if research(
            RSre,
            bp,
            0 as libc::c_int,
            ((*iop).dataend).offset_from(bp) as libc::c_long as size_t,
            regex_flags,
        ) == -(1 as libc::c_int)
        {
            (*recm)
                .len = ((*iop).dataend).offset_from((*iop).off) as libc::c_long
                as size_t;
            return NOTERM;
        }
        restart = *((*RSre).regs.start).offset(0 as libc::c_int as isize) as size_t;
        reend = *((*RSre).regs.end).offset(0 as libc::c_int as isize) as size_t;
        if restart == reend {
            *state = INDATA;
            (*iop).scanoff = reend.wrapping_add(1 as libc::c_int as libc::c_ulong);
            if bp.offset((*iop).scanoff as isize) <= (*iop).dataend {
                bp = bp.offset((*iop).scanoff as isize);
            } else {
                (*recm)
                    .len = (bp.offset_from((*iop).off) as libc::c_long as libc::c_ulong)
                    .wrapping_add(restart);
                return NOTERM;
            }
        } else {
            (*recm).len = restart;
            (*recm).rt_start = bp.offset(restart as isize);
            (*recm).rt_len = reend.wrapping_sub(restart);
            *state = NOSTATE;
            if ((*iop).off).offset(reend as isize) >= (*iop).dataend {
                if reisstring((*RS).sub.val.sp, (*RS).sub.val.slen, RSre, (*iop).off)
                    != 0
                {
                    return REC_OK
                } else {
                    return TERMATEND
                }
            }
            if (*RSre).maybe_long {
                let mut matchend: *mut libc::c_char = ((*iop).off)
                    .offset(reend as isize);
                if (((*iop).dataend).offset_from(matchend) as libc::c_long
                    as libc::c_ulong) < (*RS).sub.val.slen
                {
                    return TERMNEAREND;
                }
            }
            return REC_OK;
        }
    };
}
unsafe extern "C" fn rsnullscan(
    mut iop: *mut IOBUF,
    mut recm: *mut recmatch,
    mut state: *mut SCANSTATE,
) -> RECVALUE {
    let mut bp: *mut libc::c_char = 0 as *mut libc::c_char;
    if *state as libc::c_uint == NOSTATE as libc::c_int as libc::c_uint
        || *state as libc::c_uint == INLEADER as libc::c_int as libc::c_uint
    {
        memset(
            recm as *mut libc::c_void,
            '\0' as i32,
            ::std::mem::size_of::<recmatch>() as libc::c_ulong,
        );
    }
    (*recm).start = (*iop).off;
    bp = (*iop).off;
    if *state as libc::c_uint != NOSTATE as libc::c_int as libc::c_uint {
        bp = bp.offset((*iop).scanoff as isize);
    }
    *(*iop).dataend = '\n' as i32 as libc::c_char;
    if !(*state as libc::c_uint == INTERM as libc::c_int as libc::c_uint) {
        if !(*state as libc::c_uint == INDATA as libc::c_int as libc::c_uint) {
            while *bp as libc::c_int == '\n' as i32 && bp < (*iop).dataend {
                bp = bp.offset(1);
                bp;
            }
            if bp >= (*iop).dataend {
                *state = INLEADER;
                (*iop).scanoff = bp.offset_from((*iop).off) as libc::c_long as size_t;
                return NOTERM;
            }
            (*recm).start = bp;
            (*iop).off = (*recm).start;
        }
        loop {
            loop {
                let fresh17 = bp;
                bp = bp.offset(1);
                if !(*fresh17 as libc::c_int != '\n' as i32) {
                    break;
                }
            }
            if bp >= (*iop).dataend {
                (*recm)
                    .len = (bp.offset_from((*iop).off) as libc::c_long
                    - 1 as libc::c_int as libc::c_long) as size_t;
                (*iop).scanoff = (*recm).len;
                if bp == (*iop).dataend {
                    (*recm).rt_start = bp.offset(-(1 as libc::c_int as isize));
                    (*recm).rt_len = 1 as libc::c_int as size_t;
                }
                *state = INDATA;
                return NOTERM;
            }
            if !(*bp as libc::c_int != '\n' as i32) {
                break;
            }
        }
        *state = INTERM;
        (*recm)
            .len = (bp.offset_from((*iop).off) as libc::c_long
            - 1 as libc::c_int as libc::c_long) as size_t;
        (*recm).rt_start = bp.offset(-(1 as libc::c_int as isize));
    }
    while *bp as libc::c_int == '\n' as i32 && bp < (*iop).dataend {
        bp = bp.offset(1);
        bp;
    }
    (*recm).rt_len = bp.offset_from((*recm).rt_start) as libc::c_long as size_t;
    (*iop).scanoff = bp.offset_from((*iop).off) as libc::c_long as size_t;
    if bp >= (*iop).dataend {
        return TERMATEND;
    }
    return REC_OK;
}
#[inline]
unsafe extern "C" fn retryable(mut iop: *mut IOBUF) -> libc::c_int {
    return (!PROCINFO_node.is_null()
        && !(in_PROCINFO(
            (*iop).public.name,
            b"RETRY\0" as *const u8 as *const libc::c_char,
            0 as *mut *mut NODE,
        ))
            .is_null()) as libc::c_int;
}
#[inline]
unsafe extern "C" fn errno_io_retry() -> libc::c_int {
    match *__errno_location() {
        11 | 4 | 110 => return 1 as libc::c_int,
        _ => return 0 as libc::c_int,
    };
}
unsafe extern "C" fn get_a_record(
    mut out: *mut *mut libc::c_char,
    mut len: *mut size_t,
    mut iop: *mut IOBUF,
    mut errcode: *mut libc::c_int,
    mut field_width: *mut *const awk_fieldwidth_info_t,
) -> libc::c_int {
    let mut recm: recmatch = recmatch {
        start: 0 as *mut libc::c_char,
        len: 0,
        rt_start: 0 as *mut libc::c_char,
        rt_len: 0,
    };
    let mut state: SCANSTATE = NOSTATE;
    let mut ret: RECVALUE = REC_OK;
    let mut rtval: *mut NODE = 0 as *mut NODE;
    static mut lastmatchrec: Option::<
        unsafe extern "C" fn(*mut IOBUF, *mut recmatch, *mut SCANSTATE) -> RECVALUE,
    > = None;
    if (*iop).flag as libc::c_uint & IOP_AT_EOF as libc::c_int as libc::c_uint
        != 0 as libc::c_int as libc::c_uint && (*iop).off >= (*iop).dataend
    {
        return -(1 as libc::c_int);
    }
    if read_can_timeout {
        read_timeout = get_read_timeout(iop);
    }
    if ((*iop).public.get_record).is_some() {
        let mut rt_start: *mut libc::c_char = 0 as *mut libc::c_char;
        let mut rt_len: size_t = 0;
        let mut rc: libc::c_int = ((*iop).public.get_record)
            .unwrap()(
            out,
            &mut (*iop).public,
            errcode,
            &mut rt_start,
            &mut rt_len,
            field_width,
        );
        if rc == -(1 as libc::c_int) {
            (*iop)
                .flag = ::std::mem::transmute::<
                libc::c_uint,
                iobuf_flags,
            >((*iop).flag as libc::c_uint | IOP_AT_EOF as libc::c_int as libc::c_uint);
        } else {
            *len = rc as size_t;
            rc = 0 as libc::c_int;
            if rt_len != 0 as libc::c_int as libc::c_ulong {
                (do_flags as libc::c_uint & DO_TRADITIONAL as libc::c_int as libc::c_uint
                    == 0
                    && {
                        unref((*RT_node).sub.nodep.l.lptr);
                        (*RT_node)
                            .sub
                            .nodep
                            .l
                            .lptr = make_str_node(rt_start, rt_len, 0 as libc::c_int);
                        !((*RT_node).sub.nodep.l.lptr).is_null()
                    }) as libc::c_int;
            } else {
                (do_flags as libc::c_uint & DO_TRADITIONAL as libc::c_int as libc::c_uint
                    == 0
                    && {
                        unref((*RT_node).sub.nodep.l.lptr);
                        (*RT_node).sub.nodep.l.lptr = dupnode(Nnull_string);
                        !((*RT_node).sub.nodep.l.lptr).is_null()
                    }) as libc::c_int;
            }
        }
        return rc;
    }
    if ((*iop).dataend).is_null() || (*iop).off >= (*iop).dataend {
        (*iop)
            .count = ((*iop).public.read_func)
            .unwrap()(
            (*iop).public.fd,
            (*iop).buf as *mut libc::c_void,
            (*iop).readsize,
        );
        if (*iop).count == 0 as libc::c_int as libc::c_long {
            (*iop)
                .flag = ::std::mem::transmute::<
                libc::c_uint,
                iobuf_flags,
            >((*iop).flag as libc::c_uint | IOP_AT_EOF as libc::c_int as libc::c_uint);
            return -(1 as libc::c_int);
        } else if (*iop).count == -(1 as libc::c_int) as libc::c_long {
            *errcode = *__errno_location();
            if errno_io_retry() != 0 && retryable(iop) != 0 {
                return -(2 as libc::c_int);
            }
            (*iop)
                .flag = ::std::mem::transmute::<
                libc::c_uint,
                iobuf_flags,
            >((*iop).flag as libc::c_uint | IOP_AT_EOF as libc::c_int as libc::c_uint);
            return -(1 as libc::c_int);
        } else {
            (*iop).dataend = ((*iop).buf).offset((*iop).count as isize);
            (*iop).off = (*iop).buf;
        }
    }
    state = NOSTATE;
    loop {
        let mut dataend_off: size_t = 0;
        let mut room_left: size_t = 0;
        let mut amt_to_read: size_t = 0;
        ret = (Some(matchrec.unwrap())).unwrap()(iop, &mut recm, &mut state);
        (*iop)
            .flag = ::std::mem::transmute::<
            libc::c_uint,
            iobuf_flags,
        >((*iop).flag as libc::c_uint & !(IOP_AT_START as libc::c_int) as libc::c_uint);
        if ret as libc::c_uint == REC_OK as libc::c_int as libc::c_uint {
            break;
        }
        if ret as libc::c_uint == TERMNEAREND as libc::c_int as libc::c_uint
            && ((*iop).dataend).offset_from((*iop).off) as libc::c_long
                == (*iop).public.sbuf.st_size
        {
            break;
        }
        dataend_off = ((*iop).dataend).offset_from((*iop).off) as libc::c_long as size_t;
        memmove(
            (*iop).buf as *mut libc::c_void,
            (*iop).off as *const libc::c_void,
            dataend_off,
        );
        (*iop).off = (*iop).buf;
        (*iop).dataend = ((*iop).buf).offset(dataend_off as isize);
        recm.start = (*iop).off;
        if !(recm.rt_start).is_null() {
            recm.rt_start = ((*iop).off).offset(recm.len as isize);
        }
        room_left = (((*iop).end).offset_from((*iop).dataend) as libc::c_long
            - 1 as libc::c_int as libc::c_long) as size_t;
        amt_to_read = if (*iop).readsize < room_left {
            (*iop).readsize
        } else {
            room_left
        };
        if amt_to_read < (*iop).readsize {
            grow_iop_buffer(iop);
            recm.start = (*iop).off;
            if !(recm.rt_start).is_null() {
                recm.rt_start = ((*iop).off).offset(recm.len as isize);
            }
            room_left = (((*iop).end).offset_from((*iop).dataend) as libc::c_long
                - 1 as libc::c_int as libc::c_long) as size_t;
            amt_to_read = if (*iop).readsize < room_left {
                (*iop).readsize
            } else {
                room_left
            };
        }
        while amt_to_read.wrapping_add((*iop).readsize) < room_left {
            amt_to_read = (amt_to_read as libc::c_ulong).wrapping_add((*iop).readsize)
                as size_t as size_t;
        }
        amt_to_read = if amt_to_read
            < 9223372036854775807 as libc::c_long as libc::c_ulong
        {
            amt_to_read
        } else {
            9223372036854775807 as libc::c_long as libc::c_ulong
        };
        (*iop)
            .count = ((*iop).public.read_func)
            .unwrap()(
            (*iop).public.fd,
            (*iop).dataend as *mut libc::c_void,
            amt_to_read,
        );
        if (*iop).count == -(1 as libc::c_int) as libc::c_long {
            *errcode = *__errno_location();
            if errno_io_retry() != 0 && retryable(iop) != 0 {
                return -(2 as libc::c_int);
            }
            (*iop)
                .flag = ::std::mem::transmute::<
                libc::c_uint,
                iobuf_flags,
            >((*iop).flag as libc::c_uint | IOP_AT_EOF as libc::c_int as libc::c_uint);
            break;
        } else if (*iop).count == 0 as libc::c_int as libc::c_long {
            if ret as libc::c_uint != TERMNEAREND as libc::c_int as libc::c_uint {
                (*iop)
                    .flag = ::std::mem::transmute::<
                    libc::c_uint,
                    iobuf_flags,
                >(
                    (*iop).flag as libc::c_uint
                        | IOP_AT_EOF as libc::c_int as libc::c_uint,
                );
            }
            break;
        } else {
            (*iop).dataend = ((*iop).dataend).offset((*iop).count as isize);
        }
    }
    rtval = (*RT_node).sub.nodep.l.lptr;
    if recm.rt_len == 0 as libc::c_int as libc::c_ulong {
        (do_flags as libc::c_uint & DO_TRADITIONAL as libc::c_int as libc::c_uint == 0
            && {
                unref((*RT_node).sub.nodep.l.lptr);
                (*RT_node).sub.nodep.l.lptr = dupnode(Nnull_string);
                !((*RT_node).sub.nodep.l.lptr).is_null()
            }) as libc::c_int;
        lastmatchrec = None;
    } else if lastmatchrec.is_none() || lastmatchrec != matchrec {
        lastmatchrec = matchrec;
        (do_flags as libc::c_uint & DO_TRADITIONAL as libc::c_int as libc::c_uint == 0
            && {
                unref((*RT_node).sub.nodep.l.lptr);
                (*RT_node)
                    .sub
                    .nodep
                    .l
                    .lptr = make_str_node(recm.rt_start, recm.rt_len, 0 as libc::c_int);
                !((*RT_node).sub.nodep.l.lptr).is_null()
            }) as libc::c_int;
    } else if matchrec
        == Some(
            rs1scan
                as unsafe extern "C" fn(
                    *mut IOBUF,
                    *mut recmatch,
                    *mut SCANSTATE,
                ) -> RECVALUE,
        )
    {
        if (*rtval).sub.val.slen != 1 as libc::c_int as libc::c_ulong
            || *((*rtval).sub.val.sp).offset(0 as libc::c_int as isize) as libc::c_int
                != *(recm.rt_start).offset(0 as libc::c_int as isize) as libc::c_int
        {
            (do_flags as libc::c_uint & DO_TRADITIONAL as libc::c_int as libc::c_uint
                == 0
                && {
                    unref((*RT_node).sub.nodep.l.lptr);
                    (*RT_node)
                        .sub
                        .nodep
                        .l
                        .lptr = make_str_node(
                        recm.rt_start,
                        recm.rt_len,
                        0 as libc::c_int,
                    );
                    !((*RT_node).sub.nodep.l.lptr).is_null()
                }) as libc::c_int;
        }
    } else if matchrec
        == Some(
            rsnullscan
                as unsafe extern "C" fn(
                    *mut IOBUF,
                    *mut recmatch,
                    *mut SCANSTATE,
                ) -> RECVALUE,
        )
    {
        if (*rtval).sub.val.slen >= recm.rt_len {
            (*rtval).sub.val.slen = recm.rt_len;
            if (*rtval).flags as libc::c_uint & WSTRCUR as libc::c_int as libc::c_uint
                != 0
            {
                r_free_wstr(rtval);
            }
        } else {
            (do_flags as libc::c_uint & DO_TRADITIONAL as libc::c_int as libc::c_uint
                == 0
                && {
                    unref((*RT_node).sub.nodep.l.lptr);
                    (*RT_node)
                        .sub
                        .nodep
                        .l
                        .lptr = make_str_node(
                        recm.rt_start,
                        recm.rt_len,
                        0 as libc::c_int,
                    );
                    !((*RT_node).sub.nodep.l.lptr).is_null()
                }) as libc::c_int;
        }
    } else {
        (do_flags as libc::c_uint & DO_TRADITIONAL as libc::c_int as libc::c_uint == 0
            && {
                unref((*RT_node).sub.nodep.l.lptr);
                (*RT_node)
                    .sub
                    .nodep
                    .l
                    .lptr = make_str_node(recm.rt_start, recm.rt_len, 0 as libc::c_int);
                !((*RT_node).sub.nodep.l.lptr).is_null()
            }) as libc::c_int;
    }
    if recm.len == 0 as libc::c_int as libc::c_ulong {
        *out = 0 as *mut libc::c_char;
        *len = 0 as libc::c_int as size_t;
    } else {
        *out = recm.start;
        *len = recm.len;
    }
    (*iop).off = ((*iop).off).offset((recm.len).wrapping_add(recm.rt_len) as isize);
    if recm.len == 0 as libc::c_int as libc::c_ulong
        && recm.rt_len == 0 as libc::c_int as libc::c_ulong
        && (*iop).flag as libc::c_uint & IOP_AT_EOF as libc::c_int as libc::c_uint
            != 0 as libc::c_int as libc::c_uint
    {
        return -(1 as libc::c_int)
    } else {
        return 0 as libc::c_int
    };
}
pub unsafe extern "C" fn set_RS() {
    static mut save_rs: *mut NODE = 0 as *const NODE as *mut NODE;
    if !save_rs.is_null()
        && (*(*RS_node).sub.nodep.l.lptr).sub.val.slen == (*save_rs).sub.val.slen
        && memcmp(
            (*(*RS_node).sub.nodep.l.lptr).sub.val.sp as *const libc::c_void,
            (*save_rs).sub.val.sp as *const libc::c_void,
            (*save_rs).sub.val.slen,
        ) == 0 as libc::c_int
    {
        RS_regexp = RS_re[IGNORECASE as usize];
    } else {
        unref(save_rs);
        save_rs = dupnode((*RS_node).sub.nodep.l.lptr);
        RS_is_null = 0 as libc::c_int != 0;
        RS = force_string_fmt((*RS_node).sub.nodep.l.lptr, CONVFMT, CONVFMTidx);
        refree(RS_re[0 as libc::c_int as usize]);
        refree(RS_re[1 as libc::c_int as usize]);
        RS_regexp = 0 as *mut Regexp;
        RS_re[1 as libc::c_int as usize] = RS_regexp;
        RS_re[0 as libc::c_int as usize] = RS_re[1 as libc::c_int as usize];
        if (*RS).sub.val.slen == 0 as libc::c_int as libc::c_ulong {
            RS_is_null = 1 as libc::c_int != 0;
            matchrec = Some(
                rsnullscan
                    as unsafe extern "C" fn(
                        *mut IOBUF,
                        *mut recmatch,
                        *mut SCANSTATE,
                    ) -> RECVALUE,
            );
        } else if ((*RS).sub.val.slen > 1 as libc::c_int as libc::c_ulong
            || (*RS).flags as libc::c_uint & REGEX as libc::c_int as libc::c_uint
                != 0 as libc::c_int as libc::c_uint)
            && do_flags as libc::c_uint & DO_TRADITIONAL as libc::c_int as libc::c_uint
                == 0
        {
            static mut warned: bool = 0 as libc::c_int != 0;
            RS_re[0 as libc::c_int
                as usize] = make_regexp(
                (*RS).sub.val.sp,
                (*RS).sub.val.slen,
                0 as libc::c_int != 0,
                1 as libc::c_int != 0,
                1 as libc::c_int != 0,
            );
            RS_re[1 as libc::c_int
                as usize] = make_regexp(
                (*RS).sub.val.sp,
                (*RS).sub.val.slen,
                1 as libc::c_int != 0,
                1 as libc::c_int != 0,
                1 as libc::c_int != 0,
            );
            RS_regexp = RS_re[IGNORECASE as usize];
            matchrec = Some(
                rsrescan
                    as unsafe extern "C" fn(
                        *mut IOBUF,
                        *mut recmatch,
                        *mut SCANSTATE,
                    ) -> RECVALUE,
            );
            if do_flags as libc::c_uint
                & DO_LINT_EXTENSIONS as libc::c_int as libc::c_uint != 0 && !warned
            {
                (set_loc
                    as unsafe extern "C" fn(
                        *const libc::c_char,
                        libc::c_int,
                    ) -> ())(
                    b"io.c\0" as *const u8 as *const libc::c_char,
                    4113 as libc::c_int,
                );
                (Some(lintfunc.unwrap()))
                    .unwrap()(
                    dcgettext(
                        0 as *const libc::c_char,
                        b"multicharacter value of `RS' is a gawk extension\0"
                            as *const u8 as *const libc::c_char,
                        5 as libc::c_int,
                    ),
                );
                warned = 1 as libc::c_int != 0;
            }
        } else {
            matchrec = Some(
                rs1scan
                    as unsafe extern "C" fn(
                        *mut IOBUF,
                        *mut recmatch,
                        *mut SCANSTATE,
                    ) -> RECVALUE,
            );
        }
    }
    if current_field_sep() as libc::c_uint == Using_FS as libc::c_int as libc::c_uint {
        set_FS();
    }
}
unsafe extern "C" fn pty_vs_pipe(mut command: *const libc::c_char) -> bool {
    let mut val: *mut NODE = 0 as *mut NODE;
    val = in_PROCINFO(
        command,
        b"pty\0" as *const u8 as *const libc::c_char,
        0 as *mut *mut NODE,
    );
    if !val.is_null() {
        return boolval(val);
    }
    return 0 as libc::c_int != 0;
}
pub unsafe extern "C" fn iopflags2str(mut flag: libc::c_int) -> *const libc::c_char {
    static mut values: [flagtab; 5] = [
        {
            let mut init = flagtab {
                val: IOP_IS_TTY as libc::c_int,
                name: b"IOP_IS_TTY\0" as *const u8 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = flagtab {
                val: IOP_AT_EOF as libc::c_int,
                name: b"IOP_AT_EOF\0" as *const u8 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = flagtab {
                val: IOP_CLOSED as libc::c_int,
                name: b"IOP_CLOSED\0" as *const u8 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = flagtab {
                val: IOP_AT_START as libc::c_int,
                name: b"IOP_AT_START\0" as *const u8 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = flagtab {
                val: 0 as libc::c_int,
                name: 0 as *const libc::c_char,
            };
            init
        },
    ];
    return genflags2str(flag, values.as_ptr());
}
unsafe extern "C" fn free_rp(mut rp: *mut redirect) {
    free((*rp).value as *mut libc::c_void);
    free(rp as *mut libc::c_void);
}
unsafe extern "C" fn inetfile(
    mut str: *const libc::c_char,
    mut len: size_t,
    mut isi: *mut inet_socket_info,
) -> bool {
    let mut cp: *const libc::c_char = str;
    let mut cpend: *const libc::c_char = str.offset(len as isize);
    let mut buf: inet_socket_info = inet_socket_info {
        family: 0,
        protocol: 0,
        localport: C2RustUnnamed_11 {
            offset: 0,
            len: 0,
        },
        remotehost: C2RustUnnamed_11 {
            offset: 0,
            len: 0,
        },
        remoteport: C2RustUnnamed_11 {
            offset: 0,
            len: 0,
        },
    };
    if len < 5 as libc::c_int as libc::c_ulong
        || memcmp(
            cp as *const libc::c_void,
            b"/inet\0" as *const u8 as *const libc::c_char as *const libc::c_void,
            5 as libc::c_int as libc::c_ulong,
        ) != 0 as libc::c_int
    {
        return 0 as libc::c_int != 0;
    }
    if isi.is_null() {
        isi = &mut buf;
    }
    cp = cp.offset(5 as libc::c_int as isize);
    if (cpend.offset_from(cp) as libc::c_long) < 2 as libc::c_int as libc::c_long {
        return 0 as libc::c_int != 0;
    }
    match *cp as libc::c_int {
        47 => {
            (*isi).family = 0 as libc::c_int;
        }
        52 => {
            cp = cp.offset(1);
            if *cp as libc::c_int != '/' as i32 {
                return 0 as libc::c_int != 0;
            }
            (*isi).family = 2 as libc::c_int;
        }
        54 => {
            cp = cp.offset(1);
            if *cp as libc::c_int != '/' as i32 {
                return 0 as libc::c_int != 0;
            }
            (*isi).family = 10 as libc::c_int;
        }
        _ => return 0 as libc::c_int != 0,
    }
    cp = cp.offset(1);
    cp;
    if (cpend.offset_from(cp) as libc::c_long) < 5 as libc::c_int as libc::c_long {
        return 0 as libc::c_int != 0;
    }
    if memcmp(
        cp as *const libc::c_void,
        b"tcp/\0" as *const u8 as *const libc::c_char as *const libc::c_void,
        4 as libc::c_int as libc::c_ulong,
    ) == 0 as libc::c_int
    {
        (*isi).protocol = SOCK_STREAM as libc::c_int;
    } else if memcmp(
        cp as *const libc::c_void,
        b"udp/\0" as *const u8 as *const libc::c_char as *const libc::c_void,
        4 as libc::c_int as libc::c_ulong,
    ) == 0 as libc::c_int
    {
        (*isi).protocol = SOCK_DGRAM as libc::c_int;
    } else {
        return 0 as libc::c_int != 0
    }
    cp = cp.offset(4 as libc::c_int as isize);
    (*isi).localport.offset = cp.offset_from(str) as libc::c_long as libc::c_int;
    while *cp as libc::c_int != '/' as i32 {
        cp = cp.offset(1);
        if cp >= cpend {
            return 0 as libc::c_int != 0;
        }
    }
    (*isi)
        .localport
        .len = (cp.offset_from(str) as libc::c_long
        - (*isi).localport.offset as libc::c_long) as libc::c_int;
    if (*isi).localport.len == 0 as libc::c_int {
        return 0 as libc::c_int != 0;
    }
    if (cpend.offset_from(cp) as libc::c_long) < 2 as libc::c_int as libc::c_long {
        return 0 as libc::c_int != 0;
    }
    cp = cp.offset(1);
    cp;
    (*isi).remotehost.offset = cp.offset_from(str) as libc::c_long as libc::c_int;
    while *cp as libc::c_int != '/' as i32 {
        cp = cp.offset(1);
        if cp >= cpend {
            return 0 as libc::c_int != 0;
        }
    }
    (*isi)
        .remotehost
        .len = (cp.offset_from(str) as libc::c_long
        - (*isi).remotehost.offset as libc::c_long) as libc::c_int;
    if (*isi).remotehost.len == 0 as libc::c_int {
        return 0 as libc::c_int != 0;
    }
    if (cpend.offset_from(cp) as libc::c_long) < 2 as libc::c_int as libc::c_long {
        return 0 as libc::c_int != 0;
    }
    cp = cp.offset(1);
    cp;
    (*isi).remoteport.offset = cp.offset_from(str) as libc::c_long as libc::c_int;
    while *cp as libc::c_int != '/' as i32 && cp < cpend {
        cp = cp.offset(1);
        cp;
    }
    if cp != cpend
        || {
            (*isi)
                .remoteport
                .len = (cp.offset_from(str) as libc::c_long
                - (*isi).remoteport.offset as libc::c_long) as libc::c_int;
            (*isi).remoteport.len == 0 as libc::c_int
        }
    {
        return 0 as libc::c_int != 0;
    }
    return 1 as libc::c_int != 0;
}
unsafe extern "C" fn in_PROCINFO(
    mut pidx1: *const libc::c_char,
    mut pidx2: *const libc::c_char,
    mut full_idx: *mut *mut NODE,
) -> *mut NODE {
    let mut str: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut str_len: size_t = 0;
    let mut r: *mut NODE = 0 as *mut NODE;
    let mut sub: *mut NODE = 0 as *mut NODE;
    let mut subsep: *mut NODE = (*SUBSEP_node).sub.nodep.l.lptr;
    if PROCINFO_node.is_null() || pidx1.is_null() && pidx2.is_null() {
        return 0 as *mut NODE;
    }
    if !full_idx.is_null() {
        sub = *full_idx;
    }
    if !pidx1.is_null() && pidx2.is_null() {
        str_len = strlen(pidx1);
    } else if pidx1.is_null() && !pidx2.is_null() {
        str_len = strlen(pidx2);
    } else {
        str_len = (strlen(pidx1))
            .wrapping_add((*subsep).sub.val.slen)
            .wrapping_add(strlen(pidx2));
    }
    if sub.is_null() {
        str = emalloc_real(
            str_len.wrapping_add(1 as libc::c_int as libc::c_ulong),
            b"in_PROCINFO\0" as *const u8 as *const libc::c_char,
            b"str\0" as *const u8 as *const libc::c_char,
            b"io.c\0" as *const u8 as *const libc::c_char,
            4302 as libc::c_int,
        ) as *mut libc::c_char;
        sub = make_str_node(str, str_len, 2 as libc::c_int);
        if !full_idx.is_null() {
            *full_idx = sub;
        }
    } else if str_len != (*sub).sub.val.slen {
        (*sub)
            .sub
            .val
            .sp = erealloc_real(
            (*sub).sub.val.sp as *mut libc::c_void,
            str_len.wrapping_add(1 as libc::c_int as libc::c_ulong),
            b"in_PROCINFO\0" as *const u8 as *const libc::c_char,
            b"sub->stptr\0" as *const u8 as *const libc::c_char,
            b"io.c\0" as *const u8 as *const libc::c_char,
            4310 as libc::c_int,
        ) as *mut libc::c_char;
        (*sub).sub.val.slen = str_len;
    }
    if !pidx1.is_null() && pidx2.is_null() {
        strcpy((*sub).sub.val.sp, pidx1);
    } else if pidx1.is_null() && !pidx2.is_null() {
        strcpy((*sub).sub.val.sp, pidx2);
    } else {
        sprintf(
            (*sub).sub.val.sp,
            b"%s%.*s%s\0" as *const u8 as *const libc::c_char,
            pidx1,
            (*subsep).sub.val.slen as libc::c_int,
            (*subsep).sub.val.sp,
            pidx2,
        );
    }
    r = in_array(PROCINFO_node, sub);
    if full_idx.is_null() {
        unref(sub);
    }
    return r;
}
unsafe extern "C" fn get_read_timeout(mut iop: *mut IOBUF) -> libc::c_long {
    let mut tmout: libc::c_long = 0 as libc::c_int as libc::c_long;
    if !PROCINFO_node.is_null() {
        let mut name: *const libc::c_char = (*iop).public.name;
        let mut val: *mut NODE = 0 as *mut NODE;
        static mut full_idx: *mut NODE = 0 as *const NODE as *mut NODE;
        static mut last_name: *const libc::c_char = 0 as *const libc::c_char;
        if full_idx.is_null() || strcmp(name, last_name) != 0 as libc::c_int {
            val = in_PROCINFO(
                name,
                b"READ_TIMEOUT\0" as *const u8 as *const libc::c_char,
                &mut full_idx,
            );
            if !last_name.is_null() {
                free(last_name as *mut libc::c_void);
            }
            last_name = estrdup(name, strlen(name));
        } else {
            val = in_array(PROCINFO_node, full_idx);
        }
        if !val.is_null() {
            force_number(val);
            tmout = (*val).sub.val.fltnum as libc::c_long;
        }
    } else {
        tmout = read_default_timeout;
    }
    if (*iop).public.read_func
        == ::std::mem::transmute::<
            Option::<unsafe extern "C" fn() -> ssize_t>,
            Option::<
                unsafe extern "C" fn(libc::c_int, *mut libc::c_void, size_t) -> ssize_t,
            >,
        >(
            ::std::mem::transmute::<
                Option::<
                    unsafe extern "C" fn(
                        libc::c_int,
                        *mut libc::c_void,
                        size_t,
                    ) -> ssize_t,
                >,
                Option::<unsafe extern "C" fn() -> ssize_t>,
            >(
                Some(
                    read
                        as unsafe extern "C" fn(
                            libc::c_int,
                            *mut libc::c_void,
                            size_t,
                        ) -> ssize_t,
                ),
            ),
        ) && tmout > 0 as libc::c_int as libc::c_long
    {
        (*iop)
            .public
            .read_func = Some(
            read_with_timeout
                as unsafe extern "C" fn(
                    libc::c_int,
                    *mut libc::c_void,
                    size_t,
                ) -> ssize_t,
        );
    }
    return tmout;
}
unsafe extern "C" fn read_with_timeout(
    mut fd: libc::c_int,
    mut buf: *mut libc::c_void,
    mut size: size_t,
) -> ssize_t {
    let mut readfds: fd_set = fd_set { fds_bits: [0; 16] };
    let mut tv: timeval = timeval { tv_sec: 0, tv_usec: 0 };
    let mut s: libc::c_int = fd;
    tv.tv_sec = read_timeout / 1000 as libc::c_int as libc::c_long;
    tv
        .tv_usec = 1000 as libc::c_int as libc::c_long
        * (read_timeout - 1000 as libc::c_int as libc::c_long * tv.tv_sec);
    let mut __d0: libc::c_int = 0;
    let mut __d1: libc::c_int = 0;
    let fresh18 = &mut __d0;
    let fresh19;
    let fresh20 = (::std::mem::size_of::<fd_set>() as libc::c_ulong)
        .wrapping_div(::std::mem::size_of::<__fd_mask>() as libc::c_ulong);
    let fresh21 = &mut __d1;
    let fresh22;
    let fresh23 = &mut *(readfds.fds_bits).as_mut_ptr().offset(0 as libc::c_int as isize)
        as *mut __fd_mask;
    asm!(
        "cld; rep; stosq", inlateout("cx") c2rust_asm_casts::AsmCast::cast_in(fresh18,
        fresh20) => fresh19, inlateout("di") c2rust_asm_casts::AsmCast::cast_in(fresh21,
        fresh23) => fresh22, inlateout("ax") 0 as libc::c_int => _,
        options(preserves_flags, att_syntax)
    );
    c2rust_asm_casts::AsmCast::cast_out(fresh18, fresh20, fresh19);
    c2rust_asm_casts::AsmCast::cast_out(fresh21, fresh23, fresh22);
    readfds
        .fds_bits[(s
        / (8 as libc::c_int
            * ::std::mem::size_of::<__fd_mask>() as libc::c_ulong as libc::c_int))
        as usize]
        |= ((1 as libc::c_ulong)
            << s
                % (8 as libc::c_int
                    * ::std::mem::size_of::<__fd_mask>() as libc::c_ulong
                        as libc::c_int)) as __fd_mask;
    *__errno_location() = 0 as libc::c_int;
    if select(
        fd + 1 as libc::c_int,
        &mut readfds,
        0 as *mut fd_set,
        0 as *mut fd_set,
        &mut tv,
    ) < 0 as libc::c_int
    {
        return -(1 as libc::c_int) as ssize_t;
    }
    if readfds
        .fds_bits[(s
        / (8 as libc::c_int
            * ::std::mem::size_of::<__fd_mask>() as libc::c_ulong as libc::c_int))
        as usize]
        & ((1 as libc::c_ulong)
            << s
                % (8 as libc::c_int
                    * ::std::mem::size_of::<__fd_mask>() as libc::c_ulong
                        as libc::c_int)) as __fd_mask != 0 as libc::c_int as libc::c_long
    {
        return read(fd, buf, size);
    }
    *__errno_location() = 110 as libc::c_int;
    return -(1 as libc::c_int) as ssize_t;
}
pub unsafe extern "C" fn gawk_fwrite(
    mut buf: *const libc::c_void,
    mut size: size_t,
    mut count: size_t,
    mut fp: *mut FILE,
    mut opaque: *mut libc::c_void,
) -> size_t {
    return if 0 != 0 && 0 != 0
        && size.wrapping_mul(count) <= 8 as libc::c_int as libc::c_ulong
        && size != 0 as libc::c_int as libc::c_ulong
    {
        {
            let mut __ptr: *const libc::c_char = buf as *const libc::c_char;
            let mut __stream: *mut FILE = fp;
            let mut __cnt: size_t = 0;
            __cnt = size.wrapping_mul(count);
            while __cnt > 0 as libc::c_int as libc::c_ulong {
                let fresh24 = __ptr;
                __ptr = __ptr.offset(1);
                if putc_unlocked(*fresh24 as libc::c_int, __stream)
                    == -(1 as libc::c_int)
                {
                    break;
                }
                __cnt = __cnt.wrapping_sub(1);
                __cnt;
            }
            size.wrapping_mul(count).wrapping_sub(__cnt).wrapping_div(size)
        }
    } else if 0 != 0 && size == 0 as libc::c_int as libc::c_ulong
        || 0 != 0 && count == 0 as libc::c_int as libc::c_ulong
    {
        0 as libc::c_int as size_t
    } else {
        fwrite_unlocked(buf, size, count, fp)
    };
}
unsafe extern "C" fn gawk_fflush(
    mut fp: *mut FILE,
    mut opaque: *mut libc::c_void,
) -> libc::c_int {
    return fflush(fp);
}
unsafe extern "C" fn gawk_ferror(
    mut fp: *mut FILE,
    mut opaque: *mut libc::c_void,
) -> libc::c_int {
    return ferror(fp);
}
unsafe extern "C" fn gawk_fclose(
    mut fp: *mut FILE,
    mut opaque: *mut libc::c_void,
) -> libc::c_int {
    let mut result: libc::c_int = 0;
    result = fclose(fp);
    return result;
}
unsafe extern "C" fn init_output_wrapper(mut outbuf: *mut awk_output_buf_t) {
    (*outbuf).name = 0 as *const libc::c_char;
    (*outbuf).mode = 0 as *const libc::c_char;
    (*outbuf).fp = 0 as *mut FILE;
    (*outbuf).opaque = 0 as *mut libc::c_void;
    (*outbuf).redirected = awk_false;
    (*outbuf)
        .gawk_fwrite = Some(
        gawk_fwrite
            as unsafe extern "C" fn(
                *const libc::c_void,
                size_t,
                size_t,
                *mut FILE,
                *mut libc::c_void,
            ) -> size_t,
    );
    (*outbuf)
        .gawk_fflush = Some(
        gawk_fflush as unsafe extern "C" fn(*mut FILE, *mut libc::c_void) -> libc::c_int,
    );
    (*outbuf)
        .gawk_ferror = Some(
        gawk_ferror as unsafe extern "C" fn(*mut FILE, *mut libc::c_void) -> libc::c_int,
    );
    (*outbuf)
        .gawk_fclose = Some(
        gawk_fclose as unsafe extern "C" fn(*mut FILE, *mut libc::c_void) -> libc::c_int,
    );
}
