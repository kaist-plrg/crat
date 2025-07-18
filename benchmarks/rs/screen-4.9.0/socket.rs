use ::libc;
use ::c2rust_bitfields;
extern "C" {
    pub type sockaddr_x25;
    pub type sockaddr_ns;
    pub type sockaddr_iso;
    pub type sockaddr_ipx;
    pub type sockaddr_inarp;
    pub type sockaddr_in6;
    pub type sockaddr_in;
    pub type sockaddr_eon;
    pub type sockaddr_dl;
    pub type sockaddr_ax25;
    pub type sockaddr_at;
    pub type logfile;
    pub type __dirstream;
    fn bcopy(__src: *const libc::c_void, __dest: *mut libc::c_void, __n: size_t);
    fn bzero(_: *mut libc::c_void, _: libc::c_ulong);
    fn memset(
        _: *mut libc::c_void,
        _: libc::c_int,
        _: libc::c_ulong,
    ) -> *mut libc::c_void;
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
    fn close(__fd: libc::c_int) -> libc::c_int;
    fn read(__fd: libc::c_int, __buf: *mut libc::c_void, __nbytes: size_t) -> ssize_t;
    fn write(__fd: libc::c_int, __buf: *const libc::c_void, __n: size_t) -> ssize_t;
    fn getcwd(__buf: *mut libc::c_char, __size: size_t) -> *mut libc::c_char;
    fn getpid() -> __pid_t;
    fn geteuid() -> __uid_t;
    fn readlink(
        __path: *const libc::c_char,
        __buf: *mut libc::c_char,
        __len: size_t,
    ) -> ssize_t;
    fn unlink(__name: *const libc::c_char) -> libc::c_int;
    fn crypt(
        __key: *const libc::c_char,
        __salt: *const libc::c_char,
    ) -> *mut libc::c_char;
    fn chmod(__file: *const libc::c_char, __mode: __mode_t) -> libc::c_int;
    fn mkfifo(__path: *const libc::c_char, __mode: __mode_t) -> libc::c_int;
    fn __xstat(
        __ver: libc::c_int,
        __filename: *const libc::c_char,
        __stat_buf: *mut stat,
    ) -> libc::c_int;
    fn fcntl(__fd: libc::c_int, __cmd: libc::c_int, _: ...) -> libc::c_int;
    fn open(__file: *const libc::c_char, __oflag: libc::c_int, _: ...) -> libc::c_int;
    fn socket(
        __domain: libc::c_int,
        __type: libc::c_int,
        __protocol: libc::c_int,
    ) -> libc::c_int;
    fn bind(
        __fd: libc::c_int,
        __addr: __CONST_SOCKADDR_ARG,
        __len: socklen_t,
    ) -> libc::c_int;
    fn connect(
        __fd: libc::c_int,
        __addr: __CONST_SOCKADDR_ARG,
        __len: socklen_t,
    ) -> libc::c_int;
    fn sendmsg(
        __fd: libc::c_int,
        __message: *const msghdr,
        __flags: libc::c_int,
    ) -> ssize_t;
    fn recvmsg(
        __fd: libc::c_int,
        __message: *mut msghdr,
        __flags: libc::c_int,
    ) -> ssize_t;
    fn accept(
        __fd: libc::c_int,
        __addr: __SOCKADDR_ARG,
        __addr_len: *mut socklen_t,
    ) -> libc::c_int;
    fn listen(__fd: libc::c_int, __n: libc::c_int) -> libc::c_int;
    fn kill(__pid: __pid_t, __sig: libc::c_int) -> libc::c_int;
    fn __errno_location() -> *mut libc::c_int;
    fn printf(_: *const libc::c_char, _: ...) -> libc::c_int;
    fn sprintf(_: *mut libc::c_char, _: *const libc::c_char, _: ...) -> libc::c_int;
    fn strtol(
        _: *const libc::c_char,
        _: *mut *mut libc::c_char,
        _: libc::c_int,
    ) -> libc::c_long;
    fn malloc(_: libc::c_ulong) -> *mut libc::c_void;
    fn free(__ptr: *mut libc::c_void);
    fn abort() -> !;
    fn exit(_: libc::c_int) -> !;
    fn getenv(__name: *const libc::c_char) -> *mut libc::c_char;
    fn utimes(__file: *const libc::c_char, __tvp: *const timeval) -> libc::c_int;
    fn SetCanvasWindow(_: *mut canvas, _: *mut win);
    fn MakeDefaultCanvas() -> libc::c_int;
    fn LoadLayout(_: *mut layout, _: *mut canvas);
    static mut strnomem: [libc::c_char; 0];
    fn opendir(__name: *const libc::c_char) -> *mut DIR;
    fn closedir(__dirp: *mut DIR) -> libc::c_int;
    fn readdir(__dirp: *mut DIR) -> *mut dirent;
    fn SigHup(_: libc::c_int);
    fn eexit(_: libc::c_int) -> !;
    fn Detach(_: libc::c_int);
    fn Hangup();
    fn Kill(_: libc::c_int, _: libc::c_int);
    fn Msg(_: libc::c_int, _: *const libc::c_char, _: ...);
    fn Panic(_: libc::c_int, _: *const libc::c_char, _: ...) -> !;
    fn StartRc(_: *mut libc::c_char, _: libc::c_int) -> libc::c_int;
    fn secopen(_: *mut libc::c_char, _: libc::c_int, _: libc::c_int) -> libc::c_int;
    fn GetTTY(_: libc::c_int, _: *mut mode);
    fn SetTTY(_: libc::c_int, _: *mut mode);
    fn SetMode(_: *mut mode, _: *mut mode, _: libc::c_int, _: libc::c_int);
    fn TtyGrabConsole(
        _: libc::c_int,
        _: libc::c_int,
        _: *mut libc::c_char,
    ) -> libc::c_int;
    fn GetPtsPathOrSymlink(_: libc::c_int) -> *mut libc::c_char;
    fn MakeWindow(_: *mut NewWindow) -> libc::c_int;
    fn RemoveLoginSlot();
    fn SetUtmp(_: *mut win) -> libc::c_int;
    fn ProcessInput(_: *mut libc::c_char, _: libc::c_int);
    fn DoAction(_: *mut action, _: libc::c_int);
    fn DoCommand(_: *mut *mut libc::c_char, _: *mut libc::c_int);
    fn Activate(_: libc::c_int);
    fn SetForeWindow(_: *mut win);
    fn Parse(
        _: *mut libc::c_char,
        _: libc::c_int,
        _: *mut *mut libc::c_char,
        _: *mut libc::c_int,
    ) -> libc::c_int;
    fn IsNumColon(
        _: *mut libc::c_char,
        _: libc::c_int,
        _: *mut libc::c_char,
        _: libc::c_int,
    ) -> libc::c_int;
    fn ShowWindows(_: libc::c_int);
    fn WindowByNoN(_: *mut libc::c_char) -> libc::c_int;
    fn FindNiceWindow(_: *mut win, _: *mut libc::c_char) -> *mut win;
    fn InitTermcap(_: libc::c_int, _: libc::c_int) -> libc::c_int;
    fn MakeDisplay(
        _: *mut libc::c_char,
        _: *mut libc::c_char,
        _: *mut libc::c_char,
        _: libc::c_int,
        _: libc::c_int,
        _: *mut mode,
    ) -> *mut display;
    fn FreeDisplay();
    fn InitTerm(_: libc::c_int);
    fn RemoveStatus();
    fn AddStr(_: *mut libc::c_char);
    fn ResetIdle();
    fn CheckScreenSize(_: libc::c_int);
    fn evenq(_: *mut event);
    fn evdeq(_: *mut event);
    fn SaveStr(_: *const libc::c_char) -> *mut libc::c_char;
    fn Filename(_: *mut libc::c_char) -> *mut libc::c_char;
    fn UserContext() -> libc::c_int;
    fn UserReturn(_: libc::c_int);
    fn UserStatus() -> libc::c_int;
    fn xsignal(
        _: libc::c_int,
        _: Option::<unsafe extern "C" fn(libc::c_int) -> ()>,
    ) -> Option::<unsafe extern "C" fn(libc::c_int) -> ()>;
    fn xseteuid(_: libc::c_int);
    fn xsetegid(_: libc::c_int);
    fn AclCheckPermCmd(_: *mut acluser, _: libc::c_int, _: *mut comm) -> libc::c_int;
    fn FindUserPtr(_: *mut libc::c_char) -> *mut *mut acluser;
    fn EncodingName(_: libc::c_int) -> *mut libc::c_char;
    fn display_windows(onblank: libc::c_int, order: libc::c_int, group: *mut win);
    static mut RcFileName: *mut libc::c_char;
    static mut extra_incap: *mut libc::c_char;
    static mut extra_outcap: *mut libc::c_char;
    static mut ServerSocket: libc::c_int;
    static mut real_uid: libc::c_int;
    static mut real_gid: libc::c_int;
    static mut eff_uid: libc::c_int;
    static mut eff_gid: libc::c_int;
    static mut dflag: libc::c_int;
    static mut iflag: libc::c_int;
    static mut rflag: libc::c_int;
    static mut lsflag: libc::c_int;
    static mut quietflag: libc::c_int;
    static mut wipeflag: libc::c_int;
    static mut xflag: libc::c_int;
    static mut queryflag: libc::c_int;
    static mut attach_tty: *mut libc::c_char;
    static mut LoginName: *mut libc::c_char;
    static mut HostName: [libc::c_char; 0];
    static mut display: *mut display;
    static mut displays: *mut display;
    static mut fore: *mut win;
    static mut wtab: *mut *mut win;
    static mut console_window: *mut win;
    static mut windows: *mut win;
    static mut flayer: *mut layer;
    static mut layout_attach: *mut layout;
    static mut layout_last: *mut layout;
    static mut layout_last_marker: layout;
    static mut nwin_undef: NewWindow;
    static mut multi: *mut libc::c_char;
    static mut maxwin: libc::c_int;
    static mut SockPath: [libc::c_char; 0];
    static mut serv_read: event;
    static mut rc_name: *mut libc::c_char;
    static mut comms: [comm; 0];
}
pub type size_t = libc::c_ulong;
pub type __int32_t = libc::c_int;
pub type __dev_t = libc::c_ulong;
pub type __uid_t = libc::c_uint;
pub type __gid_t = libc::c_uint;
pub type __ino_t = libc::c_ulong;
pub type __mode_t = libc::c_uint;
pub type __nlink_t = libc::c_ulong;
pub type __off_t = libc::c_long;
pub type __pid_t = libc::c_int;
pub type __time_t = libc::c_long;
pub type __suseconds_t = libc::c_long;
pub type __blksize_t = libc::c_long;
pub type __blkcnt_t = libc::c_long;
pub type __ssize_t = libc::c_long;
pub type __syscall_slong_t = libc::c_long;
pub type __socklen_t = libc::c_uint;
pub type ssize_t = __ssize_t;
pub type pid_t = __pid_t;
pub type socklen_t = __socklen_t;
pub type int32_t = __int32_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct timeval {
    pub tv_sec: __time_t,
    pub tv_usec: __suseconds_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct timespec {
    pub tv_sec: __time_t,
    pub tv_nsec: __syscall_slong_t,
}
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct iovec {
    pub iov_base: *mut libc::c_void,
    pub iov_len: size_t,
}
pub type __socket_type = libc::c_uint;
pub const SOCK_NONBLOCK: __socket_type = 2048;
pub const SOCK_CLOEXEC: __socket_type = 524288;
pub const SOCK_PACKET: __socket_type = 10;
pub const SOCK_DCCP: __socket_type = 6;
pub const SOCK_SEQPACKET: __socket_type = 5;
pub const SOCK_RDM: __socket_type = 4;
pub const SOCK_RAW: __socket_type = 3;
pub const SOCK_DGRAM: __socket_type = 2;
pub const SOCK_STREAM: __socket_type = 1;
pub type sa_family_t = libc::c_ushort;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct sockaddr {
    pub sa_family: sa_family_t,
    pub sa_data: [libc::c_char; 14],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct msghdr {
    pub msg_name: *mut libc::c_void,
    pub msg_namelen: socklen_t,
    pub msg_iov: *mut iovec,
    pub msg_iovlen: size_t,
    pub msg_control: *mut libc::c_void,
    pub msg_controllen: size_t,
    pub msg_flags: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct cmsghdr {
    pub cmsg_len: size_t,
    pub cmsg_level: libc::c_int,
    pub cmsg_type: libc::c_int,
    pub __cmsg_data: [libc::c_uchar; 0],
}
pub type C2RustUnnamed = libc::c_uint;
pub const SCM_CREDENTIALS: C2RustUnnamed = 2;
pub const SCM_RIGHTS: C2RustUnnamed = 1;
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct sockaddr_un {
    pub sun_family: sa_family_t,
    pub sun_path: [libc::c_char; 108],
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
pub type cc_t = libc::c_uchar;
pub type speed_t = libc::c_uint;
pub type tcflag_t = libc::c_uint;
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
pub type slot_t = *mut libc::c_char;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct exit_status {
    pub e_termination: libc::c_short,
    pub e_exit: libc::c_short,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct utmp {
    pub ut_type: libc::c_short,
    pub ut_pid: pid_t,
    pub ut_line: [libc::c_char; 32],
    pub ut_id: [libc::c_char; 4],
    pub ut_user: [libc::c_char; 32],
    pub ut_host: [libc::c_char; 256],
    pub ut_exit: exit_status,
    pub ut_session: int32_t,
    pub ut_tv: C2RustUnnamed_0,
    pub ut_addr_v6: [int32_t; 4],
    pub __glibc_reserved: [libc::c_char; 20],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_0 {
    pub tv_sec: int32_t,
    pub tv_usec: int32_t,
}
pub type state_t = libc::c_uint;
pub const PRIN4: state_t = 8;
pub const PRINCSI: state_t = 7;
pub const PRINESC: state_t = 6;
pub const PRIN: state_t = 5;
pub const CSI: state_t = 4;
pub const STRESC: state_t = 3;
pub const ASTR: state_t = 2;
pub const ESC: state_t = 1;
pub const LIT: state_t = 0;
pub type string_t = libc::c_uint;
pub const STATUS: string_t = 7;
pub const GM: string_t = 6;
pub const AKA: string_t = 5;
pub const PM: string_t = 4;
pub const APC: string_t = 3;
pub const OSC: string_t = 2;
pub const DCS: string_t = 1;
pub const NONE: string_t = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct event {
    pub next: *mut event,
    pub handler: Option::<unsafe extern "C" fn(*mut event, *mut libc::c_char) -> ()>,
    pub data: *mut libc::c_char,
    pub fd: libc::c_int,
    pub type_0: libc::c_int,
    pub pri: libc::c_int,
    pub timeout: timeval,
    pub queued: libc::c_int,
    pub active: libc::c_int,
    pub condpos: *mut libc::c_int,
    pub condneg: *mut libc::c_int,
}
pub type AclBits = *mut libc::c_uchar;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct aclusergroup {
    pub u: *mut acluser,
    pub next: *mut aclusergroup,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct acluser {
    pub u_next: *mut acluser,
    pub u_name: [libc::c_char; 257],
    pub u_password: *mut libc::c_char,
    pub u_checkpassword: libc::c_int,
    pub u_detachwin: libc::c_int,
    pub u_detachotherwin: libc::c_int,
    pub u_Esc: libc::c_int,
    pub u_MetaEsc: libc::c_int,
    pub u_plop: plop,
    pub u_id: libc::c_int,
    pub u_umask_w_bits: [AclBits; 3],
    pub u_group: *mut aclusergroup,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct plop {
    pub buf: *mut libc::c_char,
    pub len: libc::c_int,
    pub enc: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct comm {
    pub name: *mut libc::c_char,
    pub flags: libc::c_int,
    pub userbits: [AclBits; 1],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct action {
    pub nr: libc::c_int,
    pub args: *mut *mut libc::c_char,
    pub argl: *mut libc::c_int,
    pub quiet: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct mchar {
    pub image: libc::c_uchar,
    pub attr: libc::c_uchar,
    pub font: libc::c_uchar,
    pub fontx: libc::c_uchar,
    pub color: libc::c_uchar,
    pub mbcs: libc::c_uchar,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct LayFuncs {
    pub lf_LayProcess: Option::<
        unsafe extern "C" fn(*mut *mut libc::c_char, *mut libc::c_int) -> (),
    >,
    pub lf_LayAbort: Option::<unsafe extern "C" fn() -> ()>,
    pub lf_LayRedisplayLine: Option::<
        unsafe extern "C" fn(libc::c_int, libc::c_int, libc::c_int, libc::c_int) -> (),
    >,
    pub lf_LayClearLine: Option::<
        unsafe extern "C" fn(libc::c_int, libc::c_int, libc::c_int, libc::c_int) -> (),
    >,
    pub lf_LayRewrite: Option::<
        unsafe extern "C" fn(
            libc::c_int,
            libc::c_int,
            libc::c_int,
            *mut mchar,
            libc::c_int,
        ) -> libc::c_int,
    >,
    pub lf_LayResize: Option::<
        unsafe extern "C" fn(libc::c_int, libc::c_int) -> libc::c_int,
    >,
    pub lf_LayRestore: Option::<unsafe extern "C" fn() -> ()>,
    pub lf_LayFree: Option::<unsafe extern "C" fn(*mut libc::c_void) -> ()>,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct layer {
    pub l_cvlist: *mut canvas,
    pub l_width: libc::c_int,
    pub l_height: libc::c_int,
    pub l_x: libc::c_int,
    pub l_y: libc::c_int,
    pub l_encoding: libc::c_int,
    pub l_layfn: *mut LayFuncs,
    pub l_data: *mut libc::c_void,
    pub l_next: *mut layer,
    pub l_bottom: *mut layer,
    pub l_blocking: libc::c_int,
    pub l_mode: libc::c_int,
    pub l_mouseevent: C2RustUnnamed_2,
    pub l_pause: C2RustUnnamed_1,
}
#[derive(Copy, Clone, BitfieldStruct)]
#[repr(C)]
pub struct C2RustUnnamed_1 {
    #[bitfield(name = "d", ty = "libc::c_int", bits = "0..=0")]
    pub d: [u8; 1],
    #[bitfield(padding)]
    pub c2rust_padding: [u8; 7],
    pub left: *mut libc::c_int,
    pub right: *mut libc::c_int,
    pub top: libc::c_int,
    pub bottom: libc::c_int,
    pub lines: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_2 {
    pub buffer: [libc::c_uchar; 3],
    pub len: libc::c_int,
    pub start: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct canvas {
    pub c_next: *mut canvas,
    pub c_display: *mut display,
    pub c_slnext: *mut canvas,
    pub c_slprev: *mut canvas,
    pub c_slperp: *mut canvas,
    pub c_slback: *mut canvas,
    pub c_slorient: libc::c_int,
    pub c_slweight: libc::c_int,
    pub c_vplist: *mut viewport,
    pub c_layer: *mut layer,
    pub c_lnext: *mut canvas,
    pub c_blank: layer,
    pub c_xoff: libc::c_int,
    pub c_yoff: libc::c_int,
    pub c_xs: libc::c_int,
    pub c_xe: libc::c_int,
    pub c_ys: libc::c_int,
    pub c_ye: libc::c_int,
    pub c_captev: event,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct viewport {
    pub v_next: *mut viewport,
    pub v_canvas: *mut canvas,
    pub v_xoff: libc::c_int,
    pub v_yoff: libc::c_int,
    pub v_xs: libc::c_int,
    pub v_xe: libc::c_int,
    pub v_ys: libc::c_int,
    pub v_ye: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct display {
    pub d_next: *mut display,
    pub d_user: *mut acluser,
    pub d_canvas: canvas,
    pub d_cvlist: *mut canvas,
    pub d_forecv: *mut canvas,
    pub d_layout: *mut layout,
    pub d_processinput: Option::<
        unsafe extern "C" fn(*mut libc::c_char, libc::c_int) -> (),
    >,
    pub d_processinputdata: *mut libc::c_char,
    pub d_vpxmin: libc::c_int,
    pub d_vpxmax: libc::c_int,
    pub d_fore: *mut win,
    pub d_other: *mut win,
    pub d_nonblock: libc::c_int,
    pub d_termname: [libc::c_char; 33],
    pub d_tentry: *mut libc::c_char,
    pub d_tcinited: libc::c_char,
    pub d_width: libc::c_int,
    pub d_height: libc::c_int,
    pub d_defwidth: libc::c_int,
    pub d_defheight: libc::c_int,
    pub d_top: libc::c_int,
    pub d_bot: libc::c_int,
    pub d_x: libc::c_int,
    pub d_y: libc::c_int,
    pub d_rend: mchar,
    pub d_col16change: libc::c_int,
    pub d_atyp: libc::c_char,
    pub d_mbcs: libc::c_int,
    pub d_encoding: libc::c_int,
    pub d_decodestate: libc::c_int,
    pub d_realfont: libc::c_int,
    pub d_insert: libc::c_int,
    pub d_keypad: libc::c_int,
    pub d_cursorkeys: libc::c_int,
    pub d_revvid: libc::c_int,
    pub d_curvis: libc::c_int,
    pub d_has_hstatus: libc::c_int,
    pub d_hstatus: libc::c_int,
    pub d_lp_missing: libc::c_int,
    pub d_mouse: libc::c_int,
    pub d_extmouse: libc::c_int,
    pub d_mouse_parse: mouse_parse,
    pub d_mousetrack: libc::c_int,
    pub d_lpchar: mchar,
    pub d_status_time: timeval,
    pub d_status: libc::c_int,
    pub d_status_bell: libc::c_char,
    pub d_status_len: libc::c_int,
    pub d_status_lastmsg: *mut libc::c_char,
    pub d_status_buflen: libc::c_int,
    pub d_status_lastx: libc::c_int,
    pub d_status_lasty: libc::c_int,
    pub d_status_obuflen: libc::c_int,
    pub d_status_obuffree: libc::c_int,
    pub d_status_obufpos: libc::c_int,
    pub d_statusev: event,
    pub d_hstatusev: event,
    pub d_kaablamm: libc::c_int,
    pub d_ESCseen: *mut action,
    pub d_userpid: libc::c_int,
    pub d_usertty: [libc::c_char; 4096],
    pub d_userfd: libc::c_int,
    pub d_readev: event,
    pub d_writeev: event,
    pub d_blockedev: event,
    pub d_OldMode: mode,
    pub d_NewMode: mode,
    pub d_flow: libc::c_int,
    pub d_intrc: libc::c_int,
    pub d_obuf: *mut libc::c_char,
    pub d_obuflen: libc::c_int,
    pub d_obufmax: libc::c_int,
    pub d_obuflenmax: libc::c_int,
    pub d_obufp: *mut libc::c_char,
    pub d_obuffree: libc::c_int,
    pub d_auto_nuke: libc::c_int,
    pub d_nseqs: libc::c_int,
    pub d_aseqs: libc::c_int,
    pub d_kmaps: *mut libc::c_uchar,
    pub d_seqp: *mut libc::c_uchar,
    pub d_seql: libc::c_int,
    pub d_seqh: *mut libc::c_uchar,
    pub d_mapev: event,
    pub d_dontmap: libc::c_int,
    pub d_mapdefault: libc::c_int,
    pub d_tcs: [tcu; 201],
    pub d_attrtab: [*mut libc::c_char; 6],
    pub d_attrtyp: [libc::c_char; 6],
    pub d_hascolor: libc::c_int,
    pub d_dospeed: libc::c_short,
    pub d_c0_tab: [libc::c_char; 256],
    pub d_xtable: *mut *mut *mut libc::c_char,
    pub d_UPcost: libc::c_int,
    pub d_DOcost: libc::c_int,
    pub d_LEcost: libc::c_int,
    pub d_NDcost: libc::c_int,
    pub d_CRcost: libc::c_int,
    pub d_IMcost: libc::c_int,
    pub d_EIcost: libc::c_int,
    pub d_NLcost: libc::c_int,
    pub d_printfd: libc::c_int,
    pub d_loginslot: slot_t,
    pub d_utmp_logintty: utmp,
    pub d_loginttymode: libc::c_int,
    pub d_blocked: libc::c_int,
    pub d_blocked_fuzz: libc::c_int,
    pub d_idleev: event,
    pub d_blankerpid: libc::c_int,
    pub d_blankerev: event,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union tcu {
    pub flg: libc::c_int,
    pub num: libc::c_int,
    pub str_0: *mut libc::c_char,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct mode {
    pub tio: termios,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct mouse_parse {
    pub sgrmode: libc::c_char,
    pub state: libc::c_int,
    pub params: [libc::c_int; 3],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct win {
    pub w_next: *mut win,
    pub w_type: libc::c_int,
    pub w_data: *mut libc::c_void,
    pub w_layer: layer,
    pub w_savelayer: *mut layer,
    pub w_blocked: libc::c_int,
    pub w_pwin: *mut pseudowin,
    pub w_pdisplay: *mut display,
    pub w_lastdisp: *mut display,
    pub w_number: libc::c_int,
    pub w_readev: event,
    pub w_writeev: event,
    pub w_silenceev: event,
    pub w_zombieev: event,
    pub w_poll_zombie_timeout: libc::c_int,
    pub w_ptyfd: libc::c_int,
    pub w_inbuf: [libc::c_char; 4096],
    pub w_inlen: libc::c_int,
    pub w_outbuf: [libc::c_char; 4096],
    pub w_outlen: libc::c_int,
    pub w_aflag: libc::c_int,
    pub w_dynamicaka: libc::c_int,
    pub w_title: *mut libc::c_char,
    pub w_akachange: *mut libc::c_char,
    pub w_akabuf: [libc::c_char; 768],
    pub w_autoaka: libc::c_int,
    pub w_group: *mut win,
    pub w_intermediate: libc::c_int,
    pub w_args: [libc::c_int; 64],
    pub w_NumArgs: libc::c_int,
    pub w_wlock: libc::c_int,
    pub w_wlockuser: *mut acluser,
    pub w_userbits: [AclBits; 3],
    pub w_lio_notify: AclBits,
    pub w_mon_notify: AclBits,
    pub w_state: state_t,
    pub w_StringType: string_t,
    pub w_mlines: *mut mline,
    pub w_rend: mchar,
    pub w_FontL: libc::c_char,
    pub w_FontR: libc::c_char,
    pub w_FontE: libc::c_char,
    pub w_Charset: libc::c_int,
    pub w_CharsetR: libc::c_int,
    pub w_charsets: [libc::c_int; 4],
    pub w_ss: libc::c_int,
    pub w_saved: cursor,
    pub w_top: libc::c_int,
    pub w_bot: libc::c_int,
    pub w_wrap: libc::c_int,
    pub w_origin: libc::c_int,
    pub w_insert: libc::c_int,
    pub w_keypad: libc::c_int,
    pub w_cursorkeys: libc::c_int,
    pub w_revvid: libc::c_int,
    pub w_curinv: libc::c_int,
    pub w_curvvis: libc::c_int,
    pub w_autolf: libc::c_int,
    pub w_hstatus: *mut libc::c_char,
    pub w_gr: libc::c_int,
    pub w_c1: libc::c_int,
    pub w_bce: libc::c_int,
    pub w_decodestate: libc::c_int,
    pub w_mbcs: libc::c_int,
    pub w_string: [libc::c_char; 768],
    pub w_stringp: *mut libc::c_char,
    pub w_tabs: *mut libc::c_char,
    pub w_bell: libc::c_int,
    pub w_flow: libc::c_int,
    pub w_log: *mut logfile,
    pub w_logsilence: libc::c_int,
    pub w_monitor: libc::c_int,
    pub w_silencewait: libc::c_int,
    pub w_silence: libc::c_int,
    pub w_vbwait: libc::c_char,
    pub w_norefresh: libc::c_char,
    pub w_mouse: libc::c_int,
    pub w_extmouse: libc::c_int,
    pub w_slowpaste: libc::c_int,
    pub w_histheight: libc::c_int,
    pub w_histidx: libc::c_int,
    pub w_scrollback_height: libc::c_int,
    pub w_hlines: *mut mline,
    pub w_paster: paster,
    pub w_pid: libc::c_int,
    pub w_deadpid: libc::c_int,
    pub w_cmdargs: [*mut libc::c_char; 64],
    pub w_dir: *mut libc::c_char,
    pub w_term: *mut libc::c_char,
    pub w_lflag: libc::c_int,
    pub w_slot: slot_t,
    pub w_savut: utmp,
    pub w_tty: [libc::c_char; 768],
    pub w_zauto: libc::c_int,
    pub w_zdisplay: *mut display,
    pub w_alt: C2RustUnnamed_3,
    pub w_destroyev: event,
    pub w_exitstatus: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_3 {
    pub on: libc::c_int,
    pub mlines: *mut mline,
    pub width: libc::c_int,
    pub height: libc::c_int,
    pub histheight: libc::c_int,
    pub hlines: *mut mline,
    pub histidx: libc::c_int,
    pub cursor: cursor,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct cursor {
    pub on: libc::c_int,
    pub x: libc::c_int,
    pub y: libc::c_int,
    pub Rend: mchar,
    pub Charset: libc::c_int,
    pub CharsetR: libc::c_int,
    pub Charsets: [libc::c_int; 4],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct mline {
    pub image: *mut libc::c_uchar,
    pub attr: *mut libc::c_uchar,
    pub font: *mut libc::c_uchar,
    pub fontx: *mut libc::c_uchar,
    pub color: *mut libc::c_uchar,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct paster {
    pub pa_pastebuf: *mut libc::c_char,
    pub pa_pasteptr: *mut libc::c_char,
    pub pa_pastelen: libc::c_int,
    pub pa_pastelayer: *mut layer,
    pub pa_slowev: event,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct pseudowin {
    pub p_fdpat: libc::c_int,
    pub p_pid: libc::c_int,
    pub p_ptyfd: libc::c_int,
    pub p_readev: event,
    pub p_writeev: event,
    pub p_cmd: [libc::c_char; 768],
    pub p_tty: [libc::c_char; 768],
    pub p_inbuf: [libc::c_char; 4096],
    pub p_inlen: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct layout {
    pub lay_next: *mut layout,
    pub lay_title: *mut libc::c_char,
    pub lay_number: libc::c_int,
    pub lay_canvas: canvas,
    pub lay_forecv: *mut canvas,
    pub lay_cvlist: *mut canvas,
    pub lay_autosave: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct NewWindow {
    pub StartAt: libc::c_int,
    pub aka: *mut libc::c_char,
    pub args: *mut *mut libc::c_char,
    pub dir: *mut libc::c_char,
    pub term: *mut libc::c_char,
    pub aflag: libc::c_int,
    pub dynamicaka: libc::c_int,
    pub flowflag: libc::c_int,
    pub lflag: libc::c_int,
    pub histheight: libc::c_int,
    pub monitor: libc::c_int,
    pub wlock: libc::c_int,
    pub silence: libc::c_int,
    pub wrap: libc::c_int,
    pub Lflag: libc::c_int,
    pub slow: libc::c_int,
    pub gr: libc::c_int,
    pub c1: libc::c_int,
    pub bce: libc::c_int,
    pub encoding: libc::c_int,
    pub hstatus: *mut libc::c_char,
    pub charset: *mut libc::c_char,
    pub poll_zombie_timeout: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct msg {
    pub protocol_revision: libc::c_int,
    pub type_0: libc::c_int,
    pub m_tty: [libc::c_char; 4096],
    pub m: C2RustUnnamed_4,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_4 {
    pub create: C2RustUnnamed_8,
    pub attach: C2RustUnnamed_7,
    pub detach: C2RustUnnamed_6,
    pub command: C2RustUnnamed_5,
    pub message: [libc::c_char; 8192],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_5 {
    pub auser: [libc::c_char; 257],
    pub nargs: libc::c_int,
    pub cmd: [libc::c_char; 4096],
    pub apid: libc::c_int,
    pub preselect: [libc::c_char; 20],
    pub writeback: [libc::c_char; 4096],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_6 {
    pub duser: [libc::c_char; 257],
    pub dpid: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_7 {
    pub auser: [libc::c_char; 257],
    pub apid: libc::c_int,
    pub adaptflag: libc::c_int,
    pub lines: libc::c_int,
    pub columns: libc::c_int,
    pub preselect: [libc::c_char; 20],
    pub esc: libc::c_int,
    pub meta_esc: libc::c_int,
    pub envterm: [libc::c_char; 33],
    pub encoding: libc::c_int,
    pub detachfirst: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_8 {
    pub lflag: libc::c_int,
    pub aflag: libc::c_int,
    pub flowflag: libc::c_int,
    pub hheight: libc::c_int,
    pub nargs: libc::c_int,
    pub line: [libc::c_char; 4096],
    pub dir: [libc::c_char; 4096],
    pub screenterm: [libc::c_char; 33],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct dirent {
    pub d_ino: __ino_t,
    pub d_off: __off_t,
    pub d_reclen: libc::c_ushort,
    pub d_type: libc::c_uchar,
    pub d_name: [libc::c_char; 256],
}
pub type DIR = __dirstream;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct sent {
    pub next: *mut sent,
    pub mode: libc::c_int,
    pub name: *mut libc::c_char,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct pwdata {
    pub l: libc::c_int,
    pub buf: [libc::c_char; 257],
    pub m: msg,
}
#[inline]
unsafe extern "C" fn stat(
    mut __path: *const libc::c_char,
    mut __statbuf: *mut stat,
) -> libc::c_int {
    return __xstat(1 as libc::c_int, __path, __statbuf);
}
#[inline]
unsafe extern "C" fn __cmsg_nxthdr(
    mut __mhdr: *mut msghdr,
    mut __cmsg: *mut cmsghdr,
) -> *mut cmsghdr {
    if (*__cmsg).cmsg_len < ::std::mem::size_of::<cmsghdr>() as libc::c_ulong {
        return 0 as *mut cmsghdr;
    }
    __cmsg = (__cmsg as *mut libc::c_uchar)
        .offset(
            (((*__cmsg).cmsg_len)
                .wrapping_add(::std::mem::size_of::<size_t>() as libc::c_ulong)
                .wrapping_sub(1 as libc::c_int as libc::c_ulong)
                & !(::std::mem::size_of::<size_t>() as libc::c_ulong)
                    .wrapping_sub(1 as libc::c_int as libc::c_ulong)) as isize,
        ) as *mut cmsghdr;
    if __cmsg.offset(1 as libc::c_int as isize) as *mut libc::c_uchar
        > ((*__mhdr).msg_control as *mut libc::c_uchar)
            .offset((*__mhdr).msg_controllen as isize)
        || (__cmsg as *mut libc::c_uchar)
            .offset(
                (((*__cmsg).cmsg_len)
                    .wrapping_add(::std::mem::size_of::<size_t>() as libc::c_ulong)
                    .wrapping_sub(1 as libc::c_int as libc::c_ulong)
                    & !(::std::mem::size_of::<size_t>() as libc::c_ulong)
                        .wrapping_sub(1 as libc::c_int as libc::c_ulong)) as isize,
            )
            > ((*__mhdr).msg_control as *mut libc::c_uchar)
                .offset((*__mhdr).msg_controllen as isize)
    {
        return 0 as *mut cmsghdr;
    }
    return __cmsg;
}
#[inline]
unsafe extern "C" fn atoi(mut __nptr: *const libc::c_char) -> libc::c_int {
    return strtol(
        __nptr,
        0 as *mut libc::c_void as *mut *mut libc::c_char,
        10 as libc::c_int,
    ) as libc::c_int;
}
pub unsafe extern "C" fn FindSocket(
    mut fdp: *mut libc::c_int,
    mut nfoundp: *mut libc::c_int,
    mut notherp: *mut libc::c_int,
    mut match_0: *mut libc::c_char,
    mut is_sock: *mut bool,
) -> libc::c_int {
    let mut dirp: *mut DIR = 0 as *mut DIR;
    let mut dp: *mut dirent = 0 as *mut dirent;
    let mut st: stat = stat {
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
    let mut mode: libc::c_int = 0;
    let mut sdirlen: libc::c_int = 0;
    let mut matchlen: libc::c_int = 0 as libc::c_int;
    let mut name: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut n: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut firsts: libc::c_int = -(1 as libc::c_int);
    let mut sockfd: libc::c_int = 0;
    let mut firstn: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut nfound: libc::c_int = 0 as libc::c_int;
    let mut ngood: libc::c_int = 0 as libc::c_int;
    let mut ndead: libc::c_int = 0 as libc::c_int;
    let mut nwipe: libc::c_int = 0 as libc::c_int;
    let mut npriv: libc::c_int = 0 as libc::c_int;
    let mut nperfect: libc::c_int = 0 as libc::c_int;
    let mut slist: *mut sent = 0 as *mut sent;
    let mut slisttail: *mut *mut sent = 0 as *mut *mut sent;
    let mut sent_0: *mut sent = 0 as *mut sent;
    let mut nsent: *mut sent = 0 as *mut sent;
    if !match_0.is_null() {
        matchlen = strlen(match_0) as libc::c_int;
        if matchlen > 255 as libc::c_int {
            matchlen = 255 as libc::c_int;
        }
    }
    sdirlen = strlen(SockPath.as_mut_ptr()) as libc::c_int;
    xseteuid(real_uid);
    xsetegid(real_gid);
    dirp = opendir(SockPath.as_mut_ptr());
    if dirp.is_null() {
        Panic(
            *__errno_location(),
            b"Cannot opendir %s\0" as *const u8 as *const libc::c_char,
            SockPath.as_mut_ptr(),
        );
    }
    slist = 0 as *mut sent;
    slisttail = &mut slist;
    loop {
        dp = readdir(dirp);
        if dp.is_null() {
            break;
        }
        let mut cmatch: libc::c_int = 0 as libc::c_int;
        name = ((*dp).d_name).as_mut_ptr();
        if *name as libc::c_int == 0 as libc::c_int || *name as libc::c_int == '.' as i32
            || strlen(name) > (2 as libc::c_int * 768 as libc::c_int) as libc::c_ulong
        {
            continue;
        }
        if matchlen != 0 {
            n = name;
            if (*match_0 as libc::c_int <= '0' as i32
                || *match_0 as libc::c_int > '9' as i32)
                && (*n as libc::c_int > '0' as i32 && *n as libc::c_int <= '9' as i32)
            {
                while *n as libc::c_int >= '0' as i32 && *n as libc::c_int <= '9' as i32
                {
                    n = n.offset(1);
                    n;
                }
                if *n as libc::c_int == '.' as i32 {
                    n = n.offset(1);
                    n;
                }
            }
            if strncmp(
                match_0,
                b"tty\0" as *const u8 as *const libc::c_char,
                3 as libc::c_int as libc::c_ulong,
            ) != 0
                && strncmp(
                    n,
                    b"tty\0" as *const u8 as *const libc::c_char,
                    3 as libc::c_int as libc::c_ulong,
                ) == 0 as libc::c_int
            {
                n = n.offset(3 as libc::c_int as isize);
            }
            if strncmp(match_0, n, matchlen as libc::c_ulong) != 0 {
                if !(n == name && *match_0 as libc::c_int > '0' as i32
                    && *match_0 as libc::c_int <= '9' as i32)
                {
                    continue;
                }
                while *n as libc::c_int >= '0' as i32 && *n as libc::c_int <= '9' as i32
                {
                    n = n.offset(1);
                    n;
                }
                if *n as libc::c_int == '.' as i32 {
                    n = n.offset(1);
                    n;
                }
                if strncmp(match_0, n, matchlen as libc::c_ulong) != 0 {
                    continue;
                }
            } else {
                cmatch = (*n.offset(matchlen as isize) as libc::c_int
                    == 0 as libc::c_int) as libc::c_int;
            }
        }
        sprintf(
            SockPath.as_mut_ptr().offset(sdirlen as isize),
            b"/%s\0" as *const u8 as *const libc::c_char,
            name,
        );
        *__errno_location() = 0 as libc::c_int;
        if stat(SockPath.as_mut_ptr(), &mut st) != 0 {
            continue;
        }
        *is_sock = st.st_mode & 0o170000 as libc::c_int as libc::c_uint
            == 0o140000 as libc::c_int as libc::c_uint;
        if !*is_sock
            && !(st.st_mode & 0o170000 as libc::c_int as libc::c_uint
                == 0o10000 as libc::c_int as libc::c_uint)
        {
            continue;
        }
        if st.st_uid as libc::c_int != real_uid {
            continue;
        }
        mode = st.st_mode as libc::c_int & 0o777 as libc::c_int;
        if !multi.is_null() && mode & 0o677 as libc::c_int != 0o601 as libc::c_int {
            if strcmp(multi, LoginName) != 0 {
                mode = -(4 as libc::c_int);
            }
        }
        sent_0 = malloc(::std::mem::size_of::<sent>() as libc::c_ulong) as *mut sent;
        if sent_0.is_null() {
            continue;
        }
        (*sent_0).next = 0 as *mut sent;
        (*sent_0).name = SaveStr(name);
        (*sent_0).mode = mode;
        *slisttail = sent_0;
        slisttail = &mut (*sent_0).next;
        nfound += 1;
        nfound;
        sockfd = MakeClientSocket(0 as libc::c_int, *is_sock);
        xseteuid(real_uid);
        xsetegid(real_gid);
        if sockfd == -(1 as libc::c_int) {
            (*sent_0).mode = -(3 as libc::c_int);
            ndead += 1;
            ndead;
            (*sent_0).mode = -(1 as libc::c_int);
            if wipeflag != 0 {
                if unlink(SockPath.as_mut_ptr()) == 0 as libc::c_int {
                    (*sent_0).mode = -(2 as libc::c_int);
                    nwipe += 1;
                    nwipe;
                }
            }
        } else {
            mode &= 0o776 as libc::c_int;
            if mode != 0o700 as libc::c_int && mode != 0o600 as libc::c_int
                || dflag != 0 && rflag == 0 && xflag == 0 && mode == 0o600 as libc::c_int
                || dflag == 0 && rflag != 0 && mode == 0o700 as libc::c_int && xflag == 0
                || dflag == 0 && rflag == 0 && xflag == 0
            {
                close(sockfd);
                npriv += 1;
                npriv;
            } else {
                ngood += 1;
                ngood;
                if cmatch != 0 {
                    nperfect += 1;
                    nperfect;
                }
                if !fdp.is_null()
                    && (firsts == -(1 as libc::c_int)
                        || cmatch != 0 && nperfect == 1 as libc::c_int)
                {
                    if firsts != -(1 as libc::c_int) {
                        close(firsts);
                    }
                    firsts = sockfd;
                    firstn = (*sent_0).name;
                } else {
                    close(sockfd);
                }
            }
        }
    }
    closedir(dirp);
    if lsflag == 0 && nperfect == 1 as libc::c_int {
        ngood = nperfect;
    }
    if nfound != 0 && (lsflag != 0 || ngood != 1 as libc::c_int) && quietflag == 0 {
        match ngood {
            0 => {
                Msg(
                    0 as libc::c_int,
                    if nfound > 1 as libc::c_int {
                        b"There are screens on:\0" as *const u8 as *const libc::c_char
                    } else {
                        b"There is a screen on:\0" as *const u8 as *const libc::c_char
                    },
                );
            }
            1 => {
                Msg(
                    0 as libc::c_int,
                    if nfound > 1 as libc::c_int {
                        b"There are several screens on:\0" as *const u8
                            as *const libc::c_char
                    } else {
                        b"There is a suitable screen on:\0" as *const u8
                            as *const libc::c_char
                    },
                );
            }
            _ => {
                Msg(
                    0 as libc::c_int,
                    b"There are several suitable screens on:\0" as *const u8
                        as *const libc::c_char,
                );
            }
        }
        sent_0 = slist;
        while !sent_0.is_null() {
            match (*sent_0).mode {
                448 => {
                    printf(
                        b"\t%s\t(Attached)\n\0" as *const u8 as *const libc::c_char,
                        (*sent_0).name,
                    );
                }
                384 => {
                    printf(
                        b"\t%s\t(Detached)\n\0" as *const u8 as *const libc::c_char,
                        (*sent_0).name,
                    );
                }
                449 => {
                    printf(
                        b"\t%s\t(Multi, attached)\n\0" as *const u8
                            as *const libc::c_char,
                        (*sent_0).name,
                    );
                }
                385 => {
                    printf(
                        b"\t%s\t(Multi, detached)\n\0" as *const u8
                            as *const libc::c_char,
                        (*sent_0).name,
                    );
                }
                -1 => {
                    printf(
                        b"\t%s\t(Dead ?%c?)\n\0" as *const u8 as *const libc::c_char,
                        (*sent_0).name,
                        '?' as i32,
                    );
                }
                -2 => {
                    printf(
                        b"\t%s\t(Removed)\n\0" as *const u8 as *const libc::c_char,
                        (*sent_0).name,
                    );
                }
                -3 => {
                    printf(
                        b"\t%s\t(Remote or dead)\n\0" as *const u8
                            as *const libc::c_char,
                        (*sent_0).name,
                    );
                }
                -4 => {
                    printf(
                        b"\t%s\t(Private)\n\0" as *const u8 as *const libc::c_char,
                        (*sent_0).name,
                    );
                }
                _ => {}
            }
            sent_0 = (*sent_0).next;
        }
    }
    if ndead != 0 && quietflag == 0 {
        let mut m: *mut libc::c_char = b"Remove dead screens with 'screen -wipe'.\0"
            as *const u8 as *const libc::c_char as *mut libc::c_char;
        if wipeflag != 0 {
            Msg(
                0 as libc::c_int,
                b"%d socket%s wiped out.\0" as *const u8 as *const libc::c_char,
                nwipe,
                if nwipe > 1 as libc::c_int {
                    b"s\0" as *const u8 as *const libc::c_char
                } else {
                    b"\0" as *const u8 as *const libc::c_char
                },
            );
        } else {
            Msg(
                0 as libc::c_int,
                m,
                if ndead > 1 as libc::c_int {
                    b"s\0" as *const u8 as *const libc::c_char
                } else {
                    b"\0" as *const u8 as *const libc::c_char
                },
                if ndead > 1 as libc::c_int {
                    b"\0" as *const u8 as *const libc::c_char
                } else {
                    b"es\0" as *const u8 as *const libc::c_char
                },
            );
        }
    }
    if firsts != -(1 as libc::c_int) {
        sprintf(
            SockPath.as_mut_ptr().offset(sdirlen as isize),
            b"/%s\0" as *const u8 as *const libc::c_char,
            firstn,
        );
        *fdp = firsts;
    } else {
        *SockPath
            .as_mut_ptr()
            .offset(sdirlen as isize) = 0 as libc::c_int as libc::c_char;
    }
    sent_0 = slist;
    while !sent_0.is_null() {
        nsent = (*sent_0).next;
        free((*sent_0).name as *mut libc::c_void);
        free(sent_0 as *mut libc::c_char as *mut libc::c_void);
        sent_0 = nsent;
    }
    xseteuid(eff_uid);
    xsetegid(eff_gid);
    if !notherp.is_null() {
        *notherp = npriv;
    }
    if !nfoundp.is_null() {
        *nfoundp = nfound - nwipe;
    }
    return ngood;
}
unsafe extern "C" fn MakeServerFifo() -> libc::c_int {
    let mut s: libc::c_int = 0;
    let mut st: stat = stat {
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
    xseteuid(real_uid);
    xsetegid(real_gid);
    s = open(SockPath.as_mut_ptr(), 0o1 as libc::c_int | 0o4000 as libc::c_int);
    if s >= 0 as libc::c_int {
        if quietflag != 0 {
            Kill((*display).d_userpid, 1 as libc::c_int);
            eexit(11 as libc::c_int);
        }
        Msg(
            0 as libc::c_int,
            b"There is already a screen running on %s.\0" as *const u8
                as *const libc::c_char,
            Filename(SockPath.as_mut_ptr()),
        );
        if stat(SockPath.as_mut_ptr(), &mut st) < 0 as libc::c_int {
            Panic(*__errno_location(), b"stat\0" as *const u8 as *const libc::c_char);
        }
        if st.st_uid as libc::c_int != real_uid {
            Panic(
                0 as libc::c_int,
                b"Unfortunately you are not its owner.\0" as *const u8
                    as *const libc::c_char,
            );
        }
        if st.st_mode & 0o700 as libc::c_int as libc::c_uint
            == 0o600 as libc::c_int as libc::c_uint
        {
            Panic(
                0 as libc::c_int,
                b"To resume it, use \"screen -r\"\0" as *const u8 as *const libc::c_char,
            );
        } else {
            Panic(
                0 as libc::c_int,
                b"It is not detached.\0" as *const u8 as *const libc::c_char,
            );
        }
    }
    unlink(SockPath.as_mut_ptr());
    if mkfifo(
        SockPath.as_mut_ptr(),
        (0o200 as libc::c_int | 0o400 as libc::c_int
            | (if !displays.is_null() { 0o100 as libc::c_int } else { 0 as libc::c_int })
            | (if !multi.is_null() { 1 as libc::c_int } else { 0 as libc::c_int }))
            as __mode_t,
    ) < 0 as libc::c_int
    {
        Panic(
            0 as libc::c_int,
            b"mkfifo %s failed\0" as *const u8 as *const libc::c_char,
            SockPath.as_mut_ptr(),
        );
    }
    s = open(
        SockPath.as_mut_ptr(),
        0 as libc::c_int | 0o4000 as libc::c_int,
        0 as libc::c_int,
    );
    if s < 0 as libc::c_int {
        Panic(
            *__errno_location(),
            b"open fifo %s\0" as *const u8 as *const libc::c_char,
            SockPath.as_mut_ptr(),
        );
    }
    xseteuid(eff_uid);
    xsetegid(eff_gid);
    return s;
}
unsafe extern "C" fn MakeClientFifo(mut err: libc::c_int) -> libc::c_int {
    let mut s: libc::c_int = 0 as libc::c_int;
    s = secopen(
        SockPath.as_mut_ptr(),
        0o1 as libc::c_int | 0o4000 as libc::c_int,
        0 as libc::c_int,
    );
    if s >= 0 as libc::c_int {
        fcntl(s, 4 as libc::c_int, 0 as libc::c_int);
        return s;
    }
    if err != 0 {
        Msg(
            *__errno_location(),
            b"%s\0" as *const u8 as *const libc::c_char,
            SockPath.as_mut_ptr(),
        );
    }
    return -(1 as libc::c_int);
}
unsafe extern "C" fn MakeServerUnixSocket() -> libc::c_int {
    let mut s: libc::c_int = 0;
    let mut a: sockaddr_un = sockaddr_un {
        sun_family: 0,
        sun_path: [0; 108],
    };
    let mut st: stat = stat {
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
    s = socket(1 as libc::c_int, SOCK_STREAM as libc::c_int, 0 as libc::c_int);
    if s < 0 as libc::c_int {
        Panic(*__errno_location(), b"socket\0" as *const u8 as *const libc::c_char);
    }
    a.sun_family = 1 as libc::c_int as sa_family_t;
    strncpy(
        (a.sun_path).as_mut_ptr(),
        SockPath.as_mut_ptr(),
        ::std::mem::size_of::<[libc::c_char; 108]>() as libc::c_ulong,
    );
    a
        .sun_path[(::std::mem::size_of::<[libc::c_char; 108]>() as libc::c_ulong)
        .wrapping_sub(1 as libc::c_int as libc::c_ulong)
        as usize] = 0 as libc::c_int as libc::c_char;
    xseteuid(real_uid);
    xsetegid(real_gid);
    if connect(
        s,
        __CONST_SOCKADDR_ARG {
            __sockaddr__: &mut a as *mut sockaddr_un as *mut sockaddr,
        },
        (strlen(SockPath.as_mut_ptr())).wrapping_add(2 as libc::c_int as libc::c_ulong)
            as socklen_t,
    ) != -(1 as libc::c_int)
    {
        if quietflag != 0 {
            Kill((*display).d_userpid, 1 as libc::c_int);
            eexit(11 as libc::c_int);
        }
        Msg(
            0 as libc::c_int,
            b"There is already a screen running on %s.\0" as *const u8
                as *const libc::c_char,
            Filename(SockPath.as_mut_ptr()),
        );
        if stat(SockPath.as_mut_ptr(), &mut st) < 0 as libc::c_int {
            Panic(*__errno_location(), b"stat\0" as *const u8 as *const libc::c_char);
        }
        if st.st_uid != real_uid as libc::c_uint {
            Panic(
                0 as libc::c_int,
                b"Unfortunately you are not its owner.\0" as *const u8
                    as *const libc::c_char,
            );
        }
        if st.st_mode & 0o700 as libc::c_int as libc::c_uint
            == 0o600 as libc::c_int as libc::c_uint
        {
            Panic(
                0 as libc::c_int,
                b"To resume it, use \"screen -r\"\0" as *const u8 as *const libc::c_char,
            );
        } else {
            Panic(
                0 as libc::c_int,
                b"It is not detached.\0" as *const u8 as *const libc::c_char,
            );
        }
    }
    unlink(SockPath.as_mut_ptr());
    if bind(
        s,
        __CONST_SOCKADDR_ARG {
            __sockaddr__: &mut a as *mut sockaddr_un as *mut sockaddr,
        },
        (strlen(SockPath.as_mut_ptr())).wrapping_add(2 as libc::c_int as libc::c_ulong)
            as socklen_t,
    ) == -(1 as libc::c_int)
    {
        Panic(
            *__errno_location(),
            b"bind (%s)\0" as *const u8 as *const libc::c_char,
            SockPath.as_mut_ptr(),
        );
    }
    chmod(
        SockPath.as_mut_ptr(),
        (0o200 as libc::c_int | 0o400 as libc::c_int
            | (if !displays.is_null() { 0o100 as libc::c_int } else { 0 as libc::c_int })
            | (if !multi.is_null() { 1 as libc::c_int } else { 0 as libc::c_int }))
            as __mode_t,
    );
    if listen(s, 5 as libc::c_int) == -(1 as libc::c_int) {
        Panic(*__errno_location(), b"listen\0" as *const u8 as *const libc::c_char);
    }
    fcntl(s, 8 as libc::c_int, getpid());
    xseteuid(eff_uid);
    xsetegid(eff_gid);
    return s;
}
unsafe extern "C" fn MakeClientUnixSocket(mut err: libc::c_int) -> libc::c_int {
    let mut s: libc::c_int = 0;
    let mut a: sockaddr_un = sockaddr_un {
        sun_family: 0,
        sun_path: [0; 108],
    };
    s = socket(1 as libc::c_int, SOCK_STREAM as libc::c_int, 0 as libc::c_int);
    if s < 0 as libc::c_int {
        Panic(*__errno_location(), b"socket\0" as *const u8 as *const libc::c_char);
    }
    a.sun_family = 1 as libc::c_int as sa_family_t;
    strncpy(
        (a.sun_path).as_mut_ptr(),
        SockPath.as_mut_ptr(),
        ::std::mem::size_of::<[libc::c_char; 108]>() as libc::c_ulong,
    );
    a
        .sun_path[(::std::mem::size_of::<[libc::c_char; 108]>() as libc::c_ulong)
        .wrapping_sub(1 as libc::c_int as libc::c_ulong)
        as usize] = 0 as libc::c_int as libc::c_char;
    xseteuid(real_uid);
    xsetegid(real_gid);
    if connect(
        s,
        __CONST_SOCKADDR_ARG {
            __sockaddr__: &mut a as *mut sockaddr_un as *mut sockaddr,
        },
        (strlen(SockPath.as_mut_ptr())).wrapping_add(2 as libc::c_int as libc::c_ulong)
            as socklen_t,
    ) == -(1 as libc::c_int)
    {
        if err != 0 {
            Msg(
                *__errno_location(),
                b"%s: connect\0" as *const u8 as *const libc::c_char,
                SockPath.as_mut_ptr(),
            );
        }
        close(s);
        s = -(1 as libc::c_int);
    }
    xseteuid(eff_uid);
    xsetegid(eff_gid);
    return s;
}
pub unsafe extern "C" fn MakeServerSocket(mut socket_0: bool) -> libc::c_int {
    if socket_0 {
        return MakeServerUnixSocket();
    }
    return MakeServerFifo();
}
pub unsafe extern "C" fn MakeClientSocket(
    mut err: libc::c_int,
    mut socket_0: bool,
) -> libc::c_int {
    if socket_0 {
        return MakeClientUnixSocket(err);
    }
    return MakeClientFifo(err);
}
pub unsafe extern "C" fn SendCreateMsg(
    mut sty: *mut libc::c_char,
    mut nwin: *mut NewWindow,
) {
    let mut s: libc::c_int = 0;
    let mut m: msg = msg {
        protocol_revision: 0,
        type_0: 0,
        m_tty: [0; 4096],
        m: C2RustUnnamed_4 {
            create: C2RustUnnamed_8 {
                lflag: 0,
                aflag: 0,
                flowflag: 0,
                hheight: 0,
                nargs: 0,
                line: [0; 4096],
                dir: [0; 4096],
                screenterm: [0; 33],
            },
        },
    };
    let mut p: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut len: libc::c_int = 0;
    let mut n: libc::c_int = 0;
    let mut av: *mut *mut libc::c_char = 0 as *mut *mut libc::c_char;
    let mut is_socket: bool = false;
    if strlen(sty) > 255 as libc::c_int as libc::c_ulong {
        *sty.offset(255 as libc::c_int as isize) = 0 as libc::c_int as libc::c_char;
    }
    if strlen(sty)
        > (2 as libc::c_int * 768 as libc::c_int - 1 as libc::c_int) as libc::c_ulong
    {
        *sty
            .offset(
                (2 as libc::c_int * 768 as libc::c_int - 1 as libc::c_int) as isize,
            ) = 0 as libc::c_int as libc::c_char;
    }
    sprintf(
        SockPath.as_mut_ptr().offset(strlen(SockPath.as_mut_ptr()) as isize),
        b"/%s\0" as *const u8 as *const libc::c_char,
        sty,
    );
    is_socket = IsSocket(SockPath.as_mut_ptr());
    s = MakeClientSocket(1 as libc::c_int, is_socket);
    if s == -(1 as libc::c_int) {
        exit(1 as libc::c_int);
    }
    bzero(
        &mut m as *mut msg as *mut libc::c_char as *mut libc::c_void,
        ::std::mem::size_of::<msg>() as libc::c_ulong,
    );
    m.type_0 = 0 as libc::c_int;
    strncpy(
        (m.m_tty).as_mut_ptr(),
        attach_tty,
        (::std::mem::size_of::<[libc::c_char; 4096]>() as libc::c_ulong)
            .wrapping_sub(1 as libc::c_int as libc::c_ulong),
    );
    m
        .m_tty[(::std::mem::size_of::<[libc::c_char; 4096]>() as libc::c_ulong)
        .wrapping_sub(1 as libc::c_int as libc::c_ulong)
        as usize] = 0 as libc::c_int as libc::c_char;
    p = (m.m.create.line).as_mut_ptr();
    n = 0 as libc::c_int;
    if (*nwin).args != nwin_undef.args {
        av = (*nwin).args;
        while !(*av).is_null() && n < 64 as libc::c_int - 1 as libc::c_int {
            len = (strlen(*av)).wrapping_add(1 as libc::c_int as libc::c_ulong)
                as libc::c_int;
            if p.offset(len as isize)
                >= (m.m.create.line)
                    .as_mut_ptr()
                    .offset(
                        ::std::mem::size_of::<[libc::c_char; 4096]>() as libc::c_ulong
                            as isize,
                    )
                    .offset(-(1 as libc::c_int as isize))
            {
                break;
            }
            strcpy(p, *av);
            p = p.offset(len as isize);
            av = av.offset(1);
            av;
            n += 1;
            n;
        }
    }
    if (*nwin).aka != nwin_undef.aka
        && p.offset(strlen((*nwin).aka) as isize).offset(1 as libc::c_int as isize)
            < (m.m.create.line)
                .as_mut_ptr()
                .offset(
                    ::std::mem::size_of::<[libc::c_char; 4096]>() as libc::c_ulong
                        as isize,
                )
    {
        strcpy(p, (*nwin).aka);
    } else {
        *p = '\0' as i32 as libc::c_char;
    }
    m.m.create.nargs = n;
    m.m.create.aflag = (*nwin).aflag;
    m.m.create.flowflag = (*nwin).flowflag;
    m.m.create.lflag = (*nwin).lflag;
    m.m.create.hheight = (*nwin).histheight;
    if (getcwd(
        (m.m.create.dir).as_mut_ptr(),
        ::std::mem::size_of::<[libc::c_char; 4096]>() as libc::c_ulong,
    ))
        .is_null()
    {
        Msg(*__errno_location(), b"getcwd\0" as *const u8 as *const libc::c_char);
    } else {
        if (*nwin).term != nwin_undef.term {
            strncpy(
                (m.m.create.screenterm).as_mut_ptr(),
                (*nwin).term,
                32 as libc::c_int as libc::c_ulong,
            );
        }
        m.m.create.screenterm[32 as libc::c_int as usize] = '\0' as i32 as libc::c_char;
        m
            .protocol_revision = ('m' as i32) << 24 as libc::c_int
            | ('s' as i32) << 16 as libc::c_int | ('g' as i32) << 8 as libc::c_int
            | 5 as libc::c_int;
        if write(
            s,
            &mut m as *mut msg as *mut libc::c_char as *const libc::c_void,
            ::std::mem::size_of::<msg>() as libc::c_ulong,
        ) as libc::c_ulong != ::std::mem::size_of::<msg>() as libc::c_ulong
        {
            Msg(*__errno_location(), b"write\0" as *const u8 as *const libc::c_char);
        }
    }
    close(s);
}
pub unsafe extern "C" fn SendErrorMsg(
    mut tty: *mut libc::c_char,
    mut buf: *mut libc::c_char,
) -> libc::c_int {
    let mut s: libc::c_int = 0;
    let mut m: msg = msg {
        protocol_revision: 0,
        type_0: 0,
        m_tty: [0; 4096],
        m: C2RustUnnamed_4 {
            create: C2RustUnnamed_8 {
                lflag: 0,
                aflag: 0,
                flowflag: 0,
                hheight: 0,
                nargs: 0,
                line: [0; 4096],
                dir: [0; 4096],
                screenterm: [0; 33],
            },
        },
    };
    let mut is_socket: bool = false;
    strncpy(
        (m.m.message).as_mut_ptr(),
        buf,
        (::std::mem::size_of::<[libc::c_char; 8192]>() as libc::c_ulong)
            .wrapping_sub(1 as libc::c_int as libc::c_ulong),
    );
    m
        .m
        .message[(::std::mem::size_of::<[libc::c_char; 8192]>() as libc::c_ulong)
        .wrapping_sub(1 as libc::c_int as libc::c_ulong)
        as usize] = 0 as libc::c_int as libc::c_char;
    is_socket = IsSocket(SockPath.as_mut_ptr());
    s = MakeClientSocket(0 as libc::c_int, is_socket);
    if s < 0 as libc::c_int {
        return -(1 as libc::c_int);
    }
    m.type_0 = 1 as libc::c_int;
    strncpy(
        (m.m_tty).as_mut_ptr(),
        tty,
        (::std::mem::size_of::<[libc::c_char; 4096]>() as libc::c_ulong)
            .wrapping_sub(1 as libc::c_int as libc::c_ulong),
    );
    m
        .m_tty[(::std::mem::size_of::<[libc::c_char; 4096]>() as libc::c_ulong)
        .wrapping_sub(1 as libc::c_int as libc::c_ulong)
        as usize] = 0 as libc::c_int as libc::c_char;
    m
        .protocol_revision = ('m' as i32) << 24 as libc::c_int
        | ('s' as i32) << 16 as libc::c_int | ('g' as i32) << 8 as libc::c_int
        | 5 as libc::c_int;
    write(
        s,
        &mut m as *mut msg as *mut libc::c_char as *const libc::c_void,
        ::std::mem::size_of::<msg>() as libc::c_ulong,
    );
    close(s);
    return 0 as libc::c_int;
}
unsafe extern "C" fn ExecCreate(mut mp: *mut msg) {
    let mut nwin: NewWindow = NewWindow {
        StartAt: 0,
        aka: 0 as *const libc::c_char as *mut libc::c_char,
        args: 0 as *const *mut libc::c_char as *mut *mut libc::c_char,
        dir: 0 as *const libc::c_char as *mut libc::c_char,
        term: 0 as *const libc::c_char as *mut libc::c_char,
        aflag: 0,
        dynamicaka: 0,
        flowflag: 0,
        lflag: 0,
        histheight: 0,
        monitor: 0,
        wlock: 0,
        silence: 0,
        wrap: 0,
        Lflag: 0,
        slow: 0,
        gr: 0,
        c1: 0,
        bce: 0,
        encoding: 0,
        hstatus: 0 as *const libc::c_char as *mut libc::c_char,
        charset: 0 as *const libc::c_char as *mut libc::c_char,
        poll_zombie_timeout: 0,
    };
    let mut args: [*mut libc::c_char; 64] = [0 as *mut libc::c_char; 64];
    let mut n: libc::c_int = 0;
    let mut pp: *mut *mut libc::c_char = args.as_mut_ptr();
    let mut p: *mut libc::c_char = ((*mp).m.create.line).as_mut_ptr();
    let mut buf: [libc::c_char; 20] = [0; 20];
    nwin = nwin_undef;
    n = (*mp).m.create.nargs;
    if n > 64 as libc::c_int - 1 as libc::c_int {
        n = 64 as libc::c_int - 1 as libc::c_int;
    }
    if n != 0 {
        let mut l: libc::c_int = 0;
        let mut num: libc::c_int = 0;
        l = strlen(p) as libc::c_int;
        if IsNumColon(
            p,
            10 as libc::c_int,
            buf.as_mut_ptr(),
            ::std::mem::size_of::<[libc::c_char; 20]>() as libc::c_ulong as libc::c_int,
        ) != 0
        {
            if *buf.as_mut_ptr() != 0 {
                nwin.aka = buf.as_mut_ptr();
            }
            num = atoi(p);
            if num < 0 as libc::c_int || num > maxwin - 1 as libc::c_int {
                num = 0 as libc::c_int;
            }
            nwin.StartAt = num;
            p = p.offset((l + 1 as libc::c_int) as isize);
            n -= 1;
            n;
        }
    }
    while n > 0 as libc::c_int {
        let fresh0 = pp;
        pp = pp.offset(1);
        *fresh0 = p;
        p = p
            .offset(
                (strlen(p)).wrapping_add(1 as libc::c_int as libc::c_ulong) as isize,
            );
        n -= 1;
        n;
    }
    *pp = 0 as *mut libc::c_char;
    if *p != 0 {
        nwin.aka = p;
    }
    if !(*args.as_mut_ptr()).is_null() {
        nwin.args = args.as_mut_ptr();
    }
    nwin.aflag = (*mp).m.create.aflag;
    nwin.flowflag = (*mp).m.create.flowflag;
    if *((*mp).m.create.dir).as_mut_ptr() != 0 {
        nwin.dir = ((*mp).m.create.dir).as_mut_ptr();
    }
    nwin.lflag = (*mp).m.create.lflag;
    nwin.histheight = (*mp).m.create.hheight;
    if *((*mp).m.create.screenterm).as_mut_ptr() != 0 {
        nwin.term = ((*mp).m.create.screenterm).as_mut_ptr();
    }
    MakeWindow(&mut nwin);
}
unsafe extern "C" fn CheckPid(mut pid: libc::c_int) -> libc::c_int {
    if pid < 2 as libc::c_int {
        return -(1 as libc::c_int);
    }
    if eff_uid == real_uid {
        return kill(pid, 0 as libc::c_int);
    }
    if UserContext() > 0 as libc::c_int {
        UserReturn(kill(pid, 0 as libc::c_int));
    }
    return UserStatus();
}
unsafe extern "C" fn CreateTempDisplay(
    mut m: *mut msg,
    mut recvfd: libc::c_int,
    mut wi: *mut win,
) -> libc::c_int {
    let mut pid: libc::c_int = 0;
    let mut attach: libc::c_int = 0;
    let mut user: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut i: libc::c_int = 0;
    let mut Mode: mode = mode {
        tio: termios {
            c_iflag: 0,
            c_oflag: 0,
            c_cflag: 0,
            c_lflag: 0,
            c_line: 0,
            c_cc: [0; 32],
            c_ispeed: 0,
            c_ospeed: 0,
        },
    };
    let mut olddisplays: *mut display = displays;
    match (*m).type_0 {
        3 | 2 => {
            pid = (*m).m.attach.apid;
            user = ((*m).m.attach.auser).as_mut_ptr();
            attach = 1 as libc::c_int;
        }
        4 | 5 => {
            pid = (*m).m.detach.dpid;
            user = ((*m).m.detach.duser).as_mut_ptr();
            attach = 0 as libc::c_int;
        }
        _ => return -(1 as libc::c_int),
    }
    if CheckPid(pid) != 0 {
        Msg(
            0 as libc::c_int,
            b"Attach attempt with bad pid(%d)!\0" as *const u8 as *const libc::c_char,
            pid,
        );
        return -(1 as libc::c_int);
    }
    if recvfd != -(1 as libc::c_int) {
        let mut ret: libc::c_int = 0;
        let mut ttyname_in_ns: [libc::c_char; 4096] = [0; 4096];
        let mut myttyname: *mut libc::c_char = 0 as *mut libc::c_char;
        i = recvfd;
        recvfd = -(1 as libc::c_int);
        memset(
            &mut ttyname_in_ns as *mut [libc::c_char; 4096] as *mut libc::c_void,
            0 as libc::c_int,
            ::std::mem::size_of::<[libc::c_char; 4096]>() as libc::c_ulong,
        );
        *__errno_location() = 0 as libc::c_int;
        myttyname = GetPtsPathOrSymlink(i);
        if !myttyname.is_null() && *__errno_location() == 19 as libc::c_int {
            ret = readlink(
                myttyname,
                ttyname_in_ns.as_mut_ptr(),
                ::std::mem::size_of::<[libc::c_char; 4096]>() as libc::c_ulong,
            ) as libc::c_int;
            if ret < 0 as libc::c_int
                || ret as size_t
                    >= ::std::mem::size_of::<[libc::c_char; 4096]>() as libc::c_ulong
            {
                Msg(
                    *__errno_location(),
                    b"Could not perform necessary sanity checks on pts device.\0"
                        as *const u8 as *const libc::c_char,
                );
                close(i);
                Kill(pid, 1 as libc::c_int);
                return -(1 as libc::c_int);
            }
            if strcmp(ttyname_in_ns.as_mut_ptr(), ((*m).m_tty).as_mut_ptr()) != 0 {
                Msg(
                    *__errno_location(),
                    b"Attach: passed fd does not match tty: %s - %s!\0" as *const u8
                        as *const libc::c_char,
                    ttyname_in_ns.as_mut_ptr(),
                    if (*m).m_tty[0 as libc::c_int as usize] as libc::c_int
                        != '\0' as i32
                    {
                        ((*m).m_tty).as_mut_ptr() as *const libc::c_char
                    } else {
                        b"(null)\0" as *const u8 as *const libc::c_char
                    },
                );
                close(i);
                Kill(pid, 1 as libc::c_int);
                return -(1 as libc::c_int);
            }
            strncpy(
                ((*m).m_tty).as_mut_ptr(),
                myttyname,
                (::std::mem::size_of::<[libc::c_char; 4096]>() as libc::c_ulong)
                    .wrapping_sub(1 as libc::c_int as libc::c_ulong),
            );
            (*m)
                .m_tty[(::std::mem::size_of::<[libc::c_char; 4096]>() as libc::c_ulong)
                .wrapping_sub(1 as libc::c_int as libc::c_ulong)
                as usize] = 0 as libc::c_int as libc::c_char;
        } else if myttyname.is_null()
            || strcmp(myttyname, ((*m).m_tty).as_mut_ptr()) != 0
        {
            Msg(
                *__errno_location(),
                b"Attach: passed fd does not match tty: %s - %s!\0" as *const u8
                    as *const libc::c_char,
                ((*m).m_tty).as_mut_ptr(),
                if !myttyname.is_null() {
                    myttyname as *const libc::c_char
                } else {
                    b"NULL\0" as *const u8 as *const libc::c_char
                },
            );
            close(i);
            Kill(pid, 1 as libc::c_int);
            return -(1 as libc::c_int);
        }
    } else {
        i = secopen(
            ((*m).m_tty).as_mut_ptr(),
            0o2 as libc::c_int | 0o4000 as libc::c_int,
            0 as libc::c_int,
        );
        if i < 0 as libc::c_int {
            Msg(
                *__errno_location(),
                b"Attach: Could not open %s!\0" as *const u8 as *const libc::c_char,
                ((*m).m_tty).as_mut_ptr(),
            );
            Kill(pid, 1 as libc::c_int);
            return -(1 as libc::c_int);
        }
    }
    if attach != 0 {
        Kill(pid, 18 as libc::c_int);
    }
    if attach != 0 {
        if !display.is_null() || !wi.is_null() {
            write(
                i,
                b"Attaching from inside of screen?\n\0" as *const u8
                    as *const libc::c_char as *const libc::c_void,
                33 as libc::c_int as size_t,
            );
            close(i);
            Kill(pid, 1 as libc::c_int);
            Msg(
                0 as libc::c_int,
                b"Attach msg ignored: coming from inside.\0" as *const u8
                    as *const libc::c_char,
            );
            return -(1 as libc::c_int);
        }
        if strcmp(user, LoginName) != 0 {
            if (*FindUserPtr(user)).is_null() {
                write(
                    i,
                    b"Access to session denied.\n\0" as *const u8 as *const libc::c_char
                        as *const libc::c_void,
                    26 as libc::c_int as size_t,
                );
                close(i);
                Kill(pid, 1 as libc::c_int);
                Msg(
                    0 as libc::c_int,
                    b"Attach: access denied for user %s.\0" as *const u8
                        as *const libc::c_char,
                    user,
                );
                return -(1 as libc::c_int);
            }
        }
    }
    GetTTY(i, &mut Mode);
    if (MakeDisplay(
        user,
        ((*m).m_tty).as_mut_ptr(),
        (if attach != 0 {
            ((*m).m.attach.envterm).as_mut_ptr() as *const libc::c_char
        } else {
            b"\0" as *const u8 as *const libc::c_char
        }) as *mut libc::c_char,
        i,
        pid,
        &mut Mode,
    ))
        .is_null()
    {
        write(
            i,
            b"Could not make display.\n\0" as *const u8 as *const libc::c_char
                as *const libc::c_void,
            24 as libc::c_int as size_t,
        );
        close(i);
        Msg(
            0 as libc::c_int,
            b"Attach: could not make display for user %s\0" as *const u8
                as *const libc::c_char,
            user,
        );
        Kill(pid, 1 as libc::c_int);
        return -(1 as libc::c_int);
    }
    if attach != 0 {
        (*display)
            .d_encoding = if (*m).m.attach.encoding == 1 as libc::c_int {
            8 as libc::c_int
        } else if (*m).m.attach.encoding != 0 {
            (*m).m.attach.encoding - 1 as libc::c_int
        } else {
            0 as libc::c_int
        };
        if (*display).d_encoding < 0 as libc::c_int
            || (EncodingName((*display).d_encoding)).is_null()
        {
            (*display).d_encoding = 0 as libc::c_int;
        }
    }
    if iflag != 0 && !olddisplays.is_null() {
        iflag = 0 as libc::c_int;
        (*olddisplays)
            .d_NewMode
            .tio
            .c_cc[0 as libc::c_int as usize] = '\0' as i32 as cc_t;
        (*olddisplays).d_NewMode.tio.c_lflag &= !(0o1 as libc::c_int) as libc::c_uint;
        SetTTY((*olddisplays).d_userfd, &mut (*olddisplays).d_NewMode);
    }
    SetMode(
        &mut (*display).d_OldMode,
        &mut (*display).d_NewMode,
        (*display).d_flow,
        iflag,
    );
    SetTTY((*display).d_userfd, &mut (*display).d_NewMode);
    if fcntl((*display).d_userfd, 4 as libc::c_int, 0o4000 as libc::c_int) != 0 {
        Msg(
            *__errno_location(),
            b"Warning: NBLOCK fcntl failed\0" as *const u8 as *const libc::c_char,
        );
    }
    return 0 as libc::c_int;
}
pub unsafe extern "C" fn ReceiveMsg() {
    let mut left: libc::c_int = 0;
    let mut len: libc::c_int = 0;
    static mut m: msg = msg {
        protocol_revision: 0,
        type_0: 0,
        m_tty: [0; 4096],
        m: C2RustUnnamed_4 {
            create: C2RustUnnamed_8 {
                lflag: 0,
                aflag: 0,
                flowflag: 0,
                hheight: 0,
                nargs: 0,
                line: [0; 4096],
                dir: [0; 4096],
                screenterm: [0; 33],
            },
        },
    };
    let mut p: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut ns: libc::c_int = ServerSocket;
    let mut wi: *mut win = 0 as *mut win;
    let mut recvfd: libc::c_int = -(1 as libc::c_int);
    let mut user: *mut acluser = 0 as *mut acluser;
    let mut is_socket: bool = false;
    let mut a: sockaddr_un = sockaddr_un {
        sun_family: 0,
        sun_path: [0; 108],
    };
    let mut msg: msghdr = msghdr {
        msg_name: 0 as *mut libc::c_void,
        msg_namelen: 0,
        msg_iov: 0 as *mut iovec,
        msg_iovlen: 0,
        msg_control: 0 as *mut libc::c_void,
        msg_controllen: 0,
        msg_flags: 0,
    };
    let mut iov: iovec = iovec {
        iov_base: 0 as *mut libc::c_void,
        iov_len: 0,
    };
    let mut control: [libc::c_char; 1024] = [0; 1024];
    is_socket = IsSocket(SockPath.as_mut_ptr());
    if !is_socket {
        if fcntl(ServerSocket, 4 as libc::c_int, 0 as libc::c_int) == -(1 as libc::c_int)
        {
            Panic(
                *__errno_location(),
                b"BLOCK fcntl\0" as *const u8 as *const libc::c_char,
            );
        }
        p = &mut m as *mut msg as *mut libc::c_char;
        left = ::std::mem::size_of::<msg>() as libc::c_ulong as libc::c_int;
    } else {
        len = ::std::mem::size_of::<sockaddr_un>() as libc::c_ulong as libc::c_int;
        ns = accept(
            ns,
            __SOCKADDR_ARG {
                __sockaddr__: &mut a as *mut sockaddr_un as *mut sockaddr,
            },
            &mut len as *mut libc::c_int as *mut libc::c_void as *mut socklen_t,
        );
        if ns < 0 as libc::c_int {
            Msg(*__errno_location(), b"accept\0" as *const u8 as *const libc::c_char);
            return;
        }
        p = &mut m as *mut msg as *mut libc::c_char;
        left = ::std::mem::size_of::<msg>() as libc::c_ulong as libc::c_int;
        bzero(
            &mut msg as *mut msghdr as *mut libc::c_void,
            ::std::mem::size_of::<msghdr>() as libc::c_ulong,
        );
        iov.iov_base = &mut m as *mut msg as *mut libc::c_void;
        iov.iov_len = left as size_t;
        msg.msg_iov = &mut iov;
        msg.msg_iovlen = 1 as libc::c_int as size_t;
        msg
            .msg_controllen = ::std::mem::size_of::<[libc::c_char; 1024]>()
            as libc::c_ulong;
        msg.msg_control = &mut control as *mut [libc::c_char; 1024] as *mut libc::c_void;
        while left > 0 as libc::c_int {
            len = recvmsg(ns, &mut msg, 0 as libc::c_int) as libc::c_int;
            if len < 0 as libc::c_int && *__errno_location() == 4 as libc::c_int {
                continue;
            }
            if len < 0 as libc::c_int {
                close(ns);
                Msg(*__errno_location(), b"read\0" as *const u8 as *const libc::c_char);
                return;
            }
            if msg.msg_controllen != 0 {
                let mut cmsg: *mut cmsghdr = 0 as *mut cmsghdr;
                cmsg = if msg.msg_controllen
                    >= ::std::mem::size_of::<cmsghdr>() as libc::c_ulong
                {
                    msg.msg_control as *mut cmsghdr
                } else {
                    0 as *mut cmsghdr
                };
                while !cmsg.is_null() {
                    let mut cl: libc::c_int = 0;
                    let mut cp: *mut libc::c_char = 0 as *mut libc::c_char;
                    if !((*cmsg).cmsg_level != 1 as libc::c_int
                        || (*cmsg).cmsg_type != SCM_RIGHTS as libc::c_int)
                    {
                        cp = ((*cmsg).__cmsg_data).as_mut_ptr() as *mut libc::c_char;
                        cl = (*cmsg).cmsg_len as libc::c_int;
                        while cl as libc::c_ulong
                            >= ((::std::mem::size_of::<cmsghdr>() as libc::c_ulong)
                                .wrapping_add(
                                    ::std::mem::size_of::<size_t>() as libc::c_ulong,
                                )
                                .wrapping_sub(1 as libc::c_int as libc::c_ulong)
                                & !(::std::mem::size_of::<size_t>() as libc::c_ulong)
                                    .wrapping_sub(1 as libc::c_int as libc::c_ulong))
                                .wrapping_add(
                                    ::std::mem::size_of::<libc::c_int>() as libc::c_ulong,
                                )
                        {
                            let mut passedfd: libc::c_int = 0;
                            bcopy(
                                cp as *const libc::c_void,
                                &mut passedfd as *mut libc::c_int as *mut libc::c_void,
                                ::std::mem::size_of::<libc::c_int>() as libc::c_ulong,
                            );
                            if recvfd >= 0 as libc::c_int && passedfd != recvfd {
                                close(recvfd);
                            }
                            recvfd = passedfd;
                            cl = (cl as libc::c_ulong)
                                .wrapping_sub(
                                    ((::std::mem::size_of::<cmsghdr>() as libc::c_ulong)
                                        .wrapping_add(
                                            ::std::mem::size_of::<size_t>() as libc::c_ulong,
                                        )
                                        .wrapping_sub(1 as libc::c_int as libc::c_ulong)
                                        & !(::std::mem::size_of::<size_t>() as libc::c_ulong)
                                            .wrapping_sub(1 as libc::c_int as libc::c_ulong))
                                        .wrapping_add(
                                            ::std::mem::size_of::<libc::c_int>() as libc::c_ulong,
                                        ),
                                ) as libc::c_int as libc::c_int;
                        }
                    }
                    cmsg = __cmsg_nxthdr(&mut msg, cmsg);
                }
            }
            p = p.offset(len as isize);
            left -= len;
            break;
        }
    }
    while left > 0 as libc::c_int {
        len = read(ns, p as *mut libc::c_void, left as size_t) as libc::c_int;
        if len < 0 as libc::c_int && *__errno_location() == 4 as libc::c_int {
            continue;
        }
        if len <= 0 as libc::c_int {
            break;
        }
        p = p.offset(len as isize);
        left -= len;
    }
    if !is_socket {
        close(ServerSocket);
        ServerSocket = secopen(
            SockPath.as_mut_ptr(),
            0 as libc::c_int | 0o4000 as libc::c_int,
            0 as libc::c_int,
        );
        if ServerSocket < 0 as libc::c_int {
            Panic(
                *__errno_location(),
                b"reopen fifo %s\0" as *const u8 as *const libc::c_char,
                SockPath.as_mut_ptr(),
            );
        }
        evdeq(&mut serv_read);
        serv_read.fd = ServerSocket;
        evenq(&mut serv_read);
    } else {
        close(ns);
    }
    if len < 0 as libc::c_int {
        Msg(*__errno_location(), b"read\0" as *const u8 as *const libc::c_char);
        if recvfd != -(1 as libc::c_int) {
            close(recvfd);
        }
        return;
    }
    if left > 0 as libc::c_int {
        if left as libc::c_ulong != ::std::mem::size_of::<msg>() as libc::c_ulong {
            Msg(
                0 as libc::c_int,
                b"Message %d of %d bytes too small\0" as *const u8
                    as *const libc::c_char,
                left,
                ::std::mem::size_of::<msg>() as libc::c_ulong as libc::c_int,
            );
        }
        return;
    }
    if m.protocol_revision
        != ('m' as i32) << 24 as libc::c_int | ('s' as i32) << 16 as libc::c_int
            | ('g' as i32) << 8 as libc::c_int | 5 as libc::c_int
    {
        if recvfd != -(1 as libc::c_int) {
            close(recvfd);
        }
        Msg(
            0 as libc::c_int,
            b"Invalid message (magic 0x%08x).\0" as *const u8 as *const libc::c_char,
            m.protocol_revision,
        );
        return;
    }
    if m.type_0 != 2 as libc::c_int && recvfd != -(1 as libc::c_int) {
        close(recvfd);
        recvfd = -(1 as libc::c_int);
    }
    display = displays;
    while !display.is_null() {
        if strcmp(((*display).d_usertty).as_mut_ptr(), (m.m_tty).as_mut_ptr())
            == 0 as libc::c_int
        {
            break;
        }
        display = (*display).d_next;
    }
    wi = 0 as *mut win;
    if display.is_null() {
        wi = windows;
        while !wi.is_null() {
            if strcmp((m.m_tty).as_mut_ptr(), ((*wi).w_tty).as_mut_ptr()) == 0 {
                display = if !((*wi).w_layer.l_cvlist).is_null() {
                    (*(*wi).w_layer.l_cvlist).c_display
                } else {
                    0 as *mut display
                };
                break;
            } else {
                wi = (*wi).w_next;
            }
        }
    }
    if !display.is_null() && (*display).d_status != 0 {
        RemoveStatus();
    }
    if !display.is_null() && (*display).d_tcinited == 0 && m.type_0 != 7 as libc::c_int {
        if recvfd != -(1 as libc::c_int) {
            close(recvfd);
        }
        return;
    }
    let mut current_block_142: u64;
    match m.type_0 {
        6 => {
            if !display.is_null() {
                CheckScreenSize(1 as libc::c_int);
            }
            current_block_142 = 7293850626974290116;
        }
        0 => {
            ExecCreate(&mut m);
            current_block_142 = 7293850626974290116;
        }
        3 => {
            if !display.is_null() && (*display).d_userpid != 0 as libc::c_int
                && kill((*display).d_userpid, 0 as libc::c_int) == 0 as libc::c_int
            {
                current_block_142 = 7293850626974290116;
            } else {
                current_block_142 = 8613016784100927070;
            }
        }
        2 => {
            current_block_142 = 8613016784100927070;
        }
        1 => {
            let mut blocked: libc::c_int = (*display).d_blocked;
            if (*display).d_blocked == 4 as libc::c_int {
                (*display).d_blocked = 0 as libc::c_int;
            }
            Msg(
                0 as libc::c_int,
                b"%s\0" as *const u8 as *const libc::c_char,
                (m.m.message).as_mut_ptr(),
            );
            (*display).d_blocked = blocked;
            current_block_142 = 7293850626974290116;
        }
        7 => {
            if wi.is_null() {
                Hangup();
            }
            current_block_142 = 7293850626974290116;
        }
        4 | 5 => {
            user = *FindUserPtr((m.m.detach.duser).as_mut_ptr());
            if !user.is_null() && !((*user).u_password).is_null()
                && *(*user).u_password as libc::c_int != 0
            {
                if !(CreateTempDisplay(&mut m, recvfd, 0 as *mut win) != 0) {
                    AskPassword(&mut m);
                }
            } else {
                FinishDetach(&mut m);
            }
            current_block_142 = 7293850626974290116;
        }
        9 => {
            let mut oldSockPath: *mut libc::c_char = SaveStr(SockPath.as_mut_ptr());
            strcpy(SockPath.as_mut_ptr(), (m.m.command.writeback).as_mut_ptr());
            let mut is_socket_0: bool = IsSocket(SockPath.as_mut_ptr());
            let mut s: libc::c_int = MakeClientSocket(0 as libc::c_int, is_socket_0);
            strcpy(SockPath.as_mut_ptr(), oldSockPath);
            if oldSockPath.is_null() {
                abort();
            } else {
                free(oldSockPath as *mut libc::c_void);
            }
            oldSockPath = 0 as *mut libc::c_char;
            if s >= 0 as libc::c_int {
                queryflag = s;
                DoCommandMsg(&mut m);
                close(s);
            } else {
                queryflag = -(1 as libc::c_int);
            }
            Kill(
                m.m.command.apid,
                if queryflag >= 0 as libc::c_int {
                    18 as libc::c_int
                } else {
                    1 as libc::c_int
                },
            );
            queryflag = -(1 as libc::c_int);
            current_block_142 = 7293850626974290116;
        }
        8 => {
            DoCommandMsg(&mut m);
            current_block_142 = 7293850626974290116;
        }
        _ => {
            Msg(
                0 as libc::c_int,
                b"Invalid message (type %d).\0" as *const u8 as *const libc::c_char,
                m.type_0,
            );
            current_block_142 = 7293850626974290116;
        }
    }
    match current_block_142 {
        8613016784100927070 => {
            if !(CreateTempDisplay(&mut m, recvfd, wi) != 0) {
                if !((*(*display).d_user).u_password).is_null()
                    && *(*(*display).d_user).u_password as libc::c_int != 0
                {
                    AskPassword(&mut m);
                } else {
                    FinishAttach(&mut m);
                }
            }
        }
        _ => {}
    };
}
pub unsafe extern "C" fn ReceiveRaw(mut s: libc::c_int) {
    let mut rd: [libc::c_char; 256] = [0; 256];
    let mut len: libc::c_int = 0 as libc::c_int;
    let mut a: sockaddr_un = sockaddr_un {
        sun_family: 0,
        sun_path: [0; 108],
    };
    let mut is_socket: bool = false;
    is_socket = IsSocket(SockPath.as_mut_ptr());
    if !is_socket {
        if fcntl(s, 4 as libc::c_int, 0 as libc::c_int) < 0 as libc::c_int {
            Panic(
                *__errno_location(),
                b"BLOCK fcntl\0" as *const u8 as *const libc::c_char,
            );
        }
    } else {
        len = ::std::mem::size_of::<sockaddr_un>() as libc::c_ulong as libc::c_int;
        s = accept(
            s,
            __SOCKADDR_ARG {
                __sockaddr__: &mut a as *mut sockaddr_un as *mut sockaddr,
            },
            &mut len as *mut libc::c_int as *mut libc::c_void as *mut socklen_t,
        );
        if s < 0 as libc::c_int {
            Msg(*__errno_location(), b"accept\0" as *const u8 as *const libc::c_char);
            return;
        }
    }
    loop {
        len = read(s, rd.as_mut_ptr() as *mut libc::c_void, 255 as libc::c_int as size_t)
            as libc::c_int;
        if !(len > 0 as libc::c_int) {
            break;
        }
        rd[len as usize] = 0 as libc::c_int as libc::c_char;
        printf(b"%s\0" as *const u8 as *const libc::c_char, rd.as_mut_ptr());
    }
    close(s);
}
pub unsafe extern "C" fn chsock() -> libc::c_int {
    let mut r: libc::c_int = 0;
    let mut euid: libc::c_int = geteuid() as libc::c_int;
    if euid != real_uid {
        if UserContext() <= 0 as libc::c_int {
            return UserStatus();
        }
    }
    r = chmod(
        SockPath.as_mut_ptr(),
        (0o200 as libc::c_int | 0o400 as libc::c_int
            | (if !displays.is_null() { 0o100 as libc::c_int } else { 0 as libc::c_int })
            | (if !multi.is_null() { 1 as libc::c_int } else { 0 as libc::c_int }))
            as __mode_t,
    );
    utimes(SockPath.as_mut_ptr(), 0 as *const timeval);
    if euid != real_uid {
        UserReturn(r);
    }
    return r;
}
pub unsafe extern "C" fn RecoverSocket() -> libc::c_int {
    let mut is_socket: bool = false;
    close(ServerSocket);
    if geteuid() as libc::c_int != real_uid {
        if UserContext() > 0 as libc::c_int {
            UserReturn(unlink(SockPath.as_mut_ptr()));
        }
        UserStatus();
    } else {
        unlink(SockPath.as_mut_ptr());
    }
    is_socket = IsSocket(SockPath.as_mut_ptr());
    ServerSocket = MakeServerSocket(is_socket);
    if ServerSocket < 0 as libc::c_int {
        return 0 as libc::c_int;
    }
    evdeq(&mut serv_read);
    serv_read.fd = ServerSocket;
    evenq(&mut serv_read);
    return 1 as libc::c_int;
}
unsafe extern "C" fn FinishAttach(mut m: *mut msg) {
    let mut p: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut pid: libc::c_int = 0;
    let mut noshowwin: libc::c_int = 0;
    let mut wi: *mut win = 0 as *mut win;
    pid = (*display).d_userpid;
    if (*m).m.attach.detachfirst == 4 as libc::c_int
        || (*m).m.attach.detachfirst == 5 as libc::c_int
    {
        FinishDetach(m);
    }
    if !extra_outcap.is_null() {
        free(extra_outcap as *mut libc::c_void);
    }
    if !extra_incap.is_null() {
        free(extra_incap as *mut libc::c_void);
    }
    extra_outcap = 0 as *mut libc::c_char;
    extra_incap = extra_outcap;
    p = getenv(b"SYSSCREENRC\0" as *const u8 as *const libc::c_char);
    if !p.is_null() {
        StartRc(p, 1 as libc::c_int);
    } else {
        StartRc(
            b"/usr/etc/screenrc\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
            1 as libc::c_int,
        );
    }
    StartRc(RcFileName, 1 as libc::c_int);
    if InitTermcap((*m).m.attach.columns, (*m).m.attach.lines) != 0 {
        FreeDisplay();
        Kill(pid, 1 as libc::c_int);
        return;
    }
    MakeDefaultCanvas();
    InitTerm((*m).m.attach.adaptflag);
    if ((*displays).d_next).is_null() {
        chsock();
    }
    xsignal(1 as libc::c_int, Some(SigHup as unsafe extern "C" fn(libc::c_int) -> ()));
    if (*m).m.attach.esc != -(1 as libc::c_int)
        && (*m).m.attach.meta_esc != -(1 as libc::c_int)
    {
        (*(*display).d_user).u_Esc = (*m).m.attach.esc;
        (*(*display).d_user).u_MetaEsc = (*m).m.attach.meta_esc;
    }
    RemoveLoginSlot();
    if ((*displays).d_next).is_null() {
        wi = windows;
        while !wi.is_null() {
            if (*wi).w_ptyfd >= 0 as libc::c_int
                && (*wi).w_slot != -(1 as libc::c_int) as slot_t
            {
                SetUtmp(wi);
            }
            wi = (*wi).w_next;
        }
    }
    (*display).d_fore = 0 as *mut win;
    if !layout_attach.is_null() {
        let mut lay: *mut layout = layout_attach;
        if lay == &mut layout_last_marker as *mut layout {
            lay = layout_last;
        }
        if !lay.is_null() {
            LoadLayout(lay, &mut (*display).d_canvas);
            SetCanvasWindow((*display).d_forecv, 0 as *mut win);
        }
    }
    if (*(*display).d_user).u_detachwin >= 0 as libc::c_int {
        fore = *wtab.offset((*(*display).d_user).u_detachwin as isize);
    } else {
        fore = 0 as *mut win;
    }
    if (*(*display).d_user).u_detachotherwin >= 0 as libc::c_int {
        (*display)
            .d_other = *wtab.offset((*(*display).d_user).u_detachotherwin as isize);
    }
    noshowwin = 0 as libc::c_int;
    if *((*m).m.attach.preselect).as_mut_ptr() != 0 {
        if strcmp(
            ((*m).m.attach.preselect).as_mut_ptr(),
            b"=\0" as *const u8 as *const libc::c_char,
        ) == 0
        {
            fore = 0 as *mut win;
        } else if strcmp(
            ((*m).m.attach.preselect).as_mut_ptr(),
            b"-\0" as *const u8 as *const libc::c_char,
        ) == 0
        {
            fore = 0 as *mut win;
            noshowwin = 1 as libc::c_int;
        } else if strcmp(
            ((*m).m.attach.preselect).as_mut_ptr(),
            b"+\0" as *const u8 as *const libc::c_char,
        ) == 0
        {
            let mut newscreen: action = action {
                nr: 0,
                args: 0 as *mut *mut libc::c_char,
                argl: 0 as *mut libc::c_int,
                quiet: 0,
            };
            let mut na: *mut libc::c_char = 0 as *mut libc::c_char;
            newscreen.nr = 143 as libc::c_int;
            newscreen.args = &mut na;
            newscreen.quiet = 0 as libc::c_int;
            DoAction(&mut newscreen, -(1 as libc::c_int));
        } else {
            fore = FindNiceWindow(fore, ((*m).m.attach.preselect).as_mut_ptr());
        }
    } else {
        fore = FindNiceWindow(fore, 0 as *mut libc::c_char);
    }
    if !fore.is_null() {
        SetForeWindow(fore);
    } else if noshowwin == 0 {
        if AclCheckPermCmd(
            (*display).d_user,
            0 as libc::c_int,
            &mut *comms.as_mut_ptr().offset(180 as libc::c_int as isize),
        ) == 0
        {
            let mut olddisplay: *mut display = display;
            flayer = (*(*display).d_forecv).c_layer;
            display_windows(1 as libc::c_int, 0 as libc::c_int, 0 as *mut win);
            noshowwin = 1 as libc::c_int;
            display = olddisplay;
        }
    }
    Activate(0 as libc::c_int);
    ResetIdle();
    if ((*display).d_fore).is_null() && noshowwin == 0 {
        ShowWindows(-(1 as libc::c_int));
    }
    if ((*displays).d_next).is_null() && !console_window.is_null() {
        if TtyGrabConsole(
            (*console_window).w_ptyfd,
            1 as libc::c_int,
            b"reattach\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        ) == 0 as libc::c_int
        {
            Msg(
                0 as libc::c_int,
                b"console %s is on window %d\0" as *const u8 as *const libc::c_char,
                HostName.as_mut_ptr(),
                (*console_window).w_number,
            );
        }
    }
}
unsafe extern "C" fn FinishDetach(mut m: *mut msg) {
    let mut next: *mut display = 0 as *mut display;
    let mut d: *mut *mut display = 0 as *mut *mut display;
    let mut det: *mut display = 0 as *mut display;
    let mut pid: libc::c_int = 0;
    if (*m).type_0 == 2 as libc::c_int {
        pid = (*display).d_userpid;
    } else {
        pid = (*m).m.detach.dpid;
    }
    d = &mut displays;
    loop {
        det = *d;
        if det.is_null() {
            break;
        }
        if (*det).d_userpid == pid {
            break;
        }
        d = &mut (*det).d_next;
    }
    if !det.is_null() {
        *d = (*det).d_next;
        (*det).d_next = 0 as *mut display;
    }
    display = displays;
    while !display.is_null() {
        next = (*display).d_next;
        if (*m).type_0 == 5 as libc::c_int {
            Detach(4 as libc::c_int);
        } else if (*m).type_0 == 4 as libc::c_int {
            Detach(2 as libc::c_int);
        } else if (*m).type_0 == 2 as libc::c_int {
            if (*m).m.attach.detachfirst == 5 as libc::c_int {
                Detach(4 as libc::c_int);
            } else if (*m).m.attach.detachfirst == 4 as libc::c_int {
                Detach(2 as libc::c_int);
            }
        }
        display = next;
    }
    displays = det;
    display = displays;
    if (*m).type_0 != 2 as libc::c_int {
        if !display.is_null() {
            FreeDisplay();
        }
        Kill(pid, 18 as libc::c_int);
    }
}
unsafe extern "C" fn AskPassword(mut m: *mut msg) {
    let mut pwdata: *mut pwdata = 0 as *mut pwdata;
    pwdata = malloc(::std::mem::size_of::<pwdata>() as libc::c_ulong) as *mut pwdata;
    if pwdata.is_null() {
        Panic(
            0 as libc::c_int,
            b"%s\0" as *const u8 as *const libc::c_char,
            strnomem.as_mut_ptr(),
        );
    }
    (*pwdata).l = 0 as libc::c_int;
    (*pwdata).m = *m;
    (*display).d_processinputdata = pwdata as *mut libc::c_char;
    (*display)
        .d_processinput = Some(
        PasswordProcessInput
            as unsafe extern "C" fn(*mut libc::c_char, libc::c_int) -> (),
    );
    AddStr(
        b"Screen password: \0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    );
}
unsafe extern "C" fn PasswordProcessInput(
    mut ibuf: *mut libc::c_char,
    mut ilen: libc::c_int,
) {
    let mut pwdata: *mut pwdata = 0 as *mut pwdata;
    let mut c: libc::c_int = 0;
    let mut l: libc::c_int = 0;
    let mut up: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut pid: libc::c_int = (*display).d_userpid;
    pwdata = (*display).d_processinputdata as *mut pwdata;
    l = (*pwdata).l;
    loop {
        let fresh1 = ilen;
        ilen = ilen - 1;
        if !(fresh1 > 0 as libc::c_int) {
            break;
        }
        let fresh2 = ibuf;
        ibuf = ibuf.offset(1);
        c = *(fresh2 as *mut libc::c_uchar) as libc::c_int;
        if c == '\r' as i32 || c == '\n' as i32 {
            let mut buf: *mut libc::c_char = 0 as *mut libc::c_char;
            up = (*(*display).d_user).u_password;
            (*pwdata).buf[l as usize] = 0 as libc::c_int as libc::c_char;
            buf = crypt(((*pwdata).buf).as_mut_ptr(), up);
            if buf.is_null() || strncmp(buf, up, strlen(up)) != 0 {
                bzero(
                    ((*pwdata).buf).as_mut_ptr() as *mut libc::c_void,
                    ::std::mem::size_of::<[libc::c_char; 257]>() as libc::c_ulong,
                );
                if buf.is_null() {
                    AddStr(
                        b"\r\ncrypt() failed.\r\n\0" as *const u8 as *const libc::c_char
                            as *mut libc::c_char,
                    );
                } else {
                    AddStr(
                        b"\r\nPassword incorrect.\r\n\0" as *const u8
                            as *const libc::c_char as *mut libc::c_char,
                    );
                }
                (*display).d_processinputdata = 0 as *mut libc::c_char;
                FreeDisplay();
                Msg(
                    0 as libc::c_int,
                    b"Illegal reattach attempt from terminal %s.\0" as *const u8
                        as *const libc::c_char,
                    ((*pwdata).m.m_tty).as_mut_ptr(),
                );
                free(pwdata as *mut libc::c_void);
                Kill(pid, 1 as libc::c_int);
                return;
            }
            bzero(
                ((*pwdata).buf).as_mut_ptr() as *mut libc::c_void,
                ::std::mem::size_of::<[libc::c_char; 257]>() as libc::c_ulong,
            );
            AddStr(b"\r\n\0" as *const u8 as *const libc::c_char as *mut libc::c_char);
            (*display).d_processinputdata = 0 as *mut libc::c_char;
            (*display)
                .d_processinput = Some(
                ProcessInput
                    as unsafe extern "C" fn(*mut libc::c_char, libc::c_int) -> (),
            );
            if (*pwdata).m.type_0 == 4 as libc::c_int
                || (*pwdata).m.type_0 == 5 as libc::c_int
            {
                FinishDetach(&mut (*pwdata).m);
            } else {
                FinishAttach(&mut (*pwdata).m);
            }
            free(pwdata as *mut libc::c_void);
            return;
        }
        if c == 'c' as i32 & 0o37 as libc::c_int {
            AddStr(b"\r\n\0" as *const u8 as *const libc::c_char as *mut libc::c_char);
            FreeDisplay();
            Kill(pid, 1 as libc::c_int);
            return;
        }
        if c == '\u{8}' as i32 || c == 0o177 as libc::c_int {
            if l > 0 as libc::c_int {
                l -= 1;
                l;
            }
        } else if c == 'u' as i32 & 0o37 as libc::c_int {
            l = 0 as libc::c_int;
        } else if l
            < ::std::mem::size_of::<[libc::c_char; 257]>() as libc::c_ulong
                as libc::c_int - 1 as libc::c_int
        {
            let fresh3 = l;
            l = l + 1;
            (*pwdata).buf[fresh3 as usize] = c as libc::c_char;
        }
    }
    (*pwdata).l = l;
}
unsafe extern "C" fn strncpy_escape_quote(
    mut dst: *mut libc::c_char,
    mut src: *const libc::c_char,
    mut end: *const libc::c_char,
) -> *mut libc::c_char {
    while *src as libc::c_int != 0 && dst < end as *mut libc::c_char {
        if *src as libc::c_int == '"' as i32 {
            if dst.offset(2 as libc::c_int as isize) < end as *mut libc::c_char {
                let fresh4 = dst;
                dst = dst.offset(1);
                *fresh4 = '\\' as i32 as libc::c_char;
            } else {
                return 0 as *mut libc::c_char
            }
        }
        let fresh5 = src;
        src = src.offset(1);
        let fresh6 = dst;
        dst = dst.offset(1);
        *fresh6 = *fresh5;
    }
    if dst >= end as *mut libc::c_char {
        return 0 as *mut libc::c_char;
    }
    *dst = '\0' as i32 as libc::c_char;
    return dst;
}
unsafe extern "C" fn DoCommandMsg(mut mp: *mut msg) {
    let mut args: [*mut libc::c_char; 64] = [0 as *mut libc::c_char; 64];
    let mut argl: [libc::c_int; 64] = [0; 64];
    let mut fullcmd: [libc::c_char; 768] = [0; 768];
    let mut fc: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut n: libc::c_int = 0;
    let mut p: *mut libc::c_char = ((*mp).m.command.cmd).as_mut_ptr();
    let mut user: *mut acluser = 0 as *mut acluser;
    extern "C" {
        static mut EffectiveAclUser: *mut acluser;
    }
    n = (*mp).m.command.nargs;
    if n > 64 as libc::c_int - 1 as libc::c_int {
        n = 64 as libc::c_int - 1 as libc::c_int;
    }
    fc = fullcmd.as_mut_ptr();
    while n > 0 as libc::c_int {
        let mut len: libc::c_int = strlen(p) as libc::c_int;
        let fresh7 = fc;
        fc = fc.offset(1);
        *fresh7 = '"' as i32 as libc::c_char;
        fc = strncpy_escape_quote(
            fc,
            p,
            fullcmd
                .as_mut_ptr()
                .offset(
                    ::std::mem::size_of::<[libc::c_char; 768]>() as libc::c_ulong
                        as isize,
                )
                .offset(-(2 as libc::c_int as isize)),
        );
        if fc.is_null() {
            Msg(
                0 as libc::c_int,
                b"Remote command too long.\0" as *const u8 as *const libc::c_char,
            );
            queryflag = -(1 as libc::c_int);
            return;
        }
        p = p.offset((len + 1 as libc::c_int) as isize);
        let fresh8 = fc;
        fc = fc.offset(1);
        *fresh8 = '"' as i32 as libc::c_char;
        let fresh9 = fc;
        fc = fc.offset(1);
        *fresh9 = ' ' as i32 as libc::c_char;
        n -= 1;
        n;
    }
    if fc != fullcmd.as_mut_ptr() {
        fc = fc.offset(-1);
        *fc = 0 as libc::c_int as libc::c_char;
    }
    if Parse(
        fullcmd.as_mut_ptr(),
        ::std::mem::size_of::<[libc::c_char; 768]>() as libc::c_ulong as libc::c_int,
        args.as_mut_ptr(),
        argl.as_mut_ptr(),
    ) <= 0 as libc::c_int
    {
        queryflag = -(1 as libc::c_int);
        return;
    }
    user = *FindUserPtr(((*mp).m.attach.auser).as_mut_ptr());
    if user.is_null() {
        Msg(
            0 as libc::c_int,
            b"Unknown user %s tried to send a command!\0" as *const u8
                as *const libc::c_char,
            ((*mp).m.attach.auser).as_mut_ptr(),
        );
        queryflag = -(1 as libc::c_int);
        return;
    }
    if !((*user).u_password).is_null() && *(*user).u_password as libc::c_int != 0 {
        Msg(
            0 as libc::c_int,
            b"User %s has a password, cannot use remote commands (using -Q or -X option).\0"
                as *const u8 as *const libc::c_char,
            ((*mp).m.attach.auser).as_mut_ptr(),
        );
        queryflag = -(1 as libc::c_int);
        return;
    }
    if display.is_null() {
        display = displays;
        while !display.is_null() {
            if (*display).d_user == user {
                break;
            }
            display = (*display).d_next;
        }
    }
    fore = windows;
    while !fore.is_null() {
        if strcmp(((*mp).m_tty).as_mut_ptr(), ((*fore).w_tty).as_mut_ptr()) == 0 {
            if display.is_null() {
                display = if !((*fore).w_layer.l_cvlist).is_null() {
                    (*(*fore).w_layer.l_cvlist).c_display
                } else {
                    0 as *mut display
                };
            }
            if ((*fore).w_layer.l_cvlist).is_null()
                || ((*(*fore).w_layer.l_cvlist).c_display).is_null()
            {
                fore = 0 as *mut win;
            }
            break;
        } else {
            fore = (*fore).w_next;
        }
    }
    if display.is_null() {
        display = displays;
    }
    if *((*mp).m.command.preselect).as_mut_ptr() != 0 {
        let mut i: libc::c_int = -(1 as libc::c_int);
        if strcmp(
            ((*mp).m.command.preselect).as_mut_ptr(),
            b"-\0" as *const u8 as *const libc::c_char,
        ) != 0
        {
            i = WindowByNoN(((*mp).m.command.preselect).as_mut_ptr());
            if i < 0 as libc::c_int || (*wtab.offset(i as isize)).is_null() {
                Msg(
                    0 as libc::c_int,
                    b"Could not find pre-select window.\0" as *const u8
                        as *const libc::c_char,
                );
                queryflag = -(1 as libc::c_int);
                return;
            }
        }
        fore = if i >= 0 as libc::c_int {
            *wtab.offset(i as isize)
        } else {
            0 as *mut win
        };
    } else if fore.is_null() {
        if !display.is_null() && (*display).d_user == user {
            fore = (*(*(*(*display).d_forecv).c_layer).l_bottom).l_data as *mut win;
        }
        if fore.is_null() {
            fore = if (*user).u_detachwin >= 0 as libc::c_int {
                *wtab.offset((*user).u_detachwin as isize)
            } else {
                0 as *mut win
            };
            fore = FindNiceWindow(fore, 0 as *mut libc::c_char);
        }
    }
    if fore.is_null() {
        fore = windows;
    }
    EffectiveAclUser = user;
    if !(*args.as_mut_ptr()).is_null() {
        let mut oldrcname: *mut libc::c_char = rc_name;
        rc_name = b"-X\0" as *const u8 as *const libc::c_char as *mut libc::c_char;
        flayer = if !fore.is_null() { &mut (*fore).w_layer } else { 0 as *mut layer };
        if !fore.is_null() && !((*fore).w_savelayer).is_null()
            && ((*fore).w_blocked != 0 || ((*(*fore).w_savelayer).l_cvlist).is_null())
        {
            flayer = (*fore).w_savelayer;
        }
        DoCommand(args.as_mut_ptr(), argl.as_mut_ptr());
        rc_name = oldrcname;
    }
    EffectiveAclUser = 0 as *mut acluser;
}
pub unsafe extern "C" fn SendAttachMsg(
    mut s: libc::c_int,
    mut m: *mut msg,
    mut fd: libc::c_int,
) -> libc::c_int {
    let mut r: libc::c_int = 0;
    let mut msg: msghdr = msghdr {
        msg_name: 0 as *mut libc::c_void,
        msg_namelen: 0,
        msg_iov: 0 as *mut iovec,
        msg_iovlen: 0,
        msg_control: 0 as *mut libc::c_void,
        msg_controllen: 0,
        msg_flags: 0,
    };
    let mut iov: iovec = iovec {
        iov_base: 0 as *mut libc::c_void,
        iov_len: 0,
    };
    let mut buf: [libc::c_char; 24] = [0; 24];
    let mut cmsg: *mut cmsghdr = 0 as *mut cmsghdr;
    iov.iov_base = m as *mut libc::c_char as *mut libc::c_void;
    iov.iov_len = ::std::mem::size_of::<msg>() as libc::c_ulong;
    bzero(
        &mut msg as *mut msghdr as *mut libc::c_void,
        ::std::mem::size_of::<msghdr>() as libc::c_ulong,
    );
    msg.msg_name = 0 as *mut libc::c_void;
    msg.msg_namelen = 0 as libc::c_int as socklen_t;
    msg.msg_iov = &mut iov;
    msg.msg_iovlen = 1 as libc::c_int as size_t;
    msg.msg_control = buf.as_mut_ptr() as *mut libc::c_void;
    msg.msg_controllen = ::std::mem::size_of::<[libc::c_char; 24]>() as libc::c_ulong;
    cmsg = if msg.msg_controllen >= ::std::mem::size_of::<cmsghdr>() as libc::c_ulong {
        msg.msg_control as *mut cmsghdr
    } else {
        0 as *mut cmsghdr
    };
    (*cmsg).cmsg_level = 1 as libc::c_int;
    (*cmsg).cmsg_type = SCM_RIGHTS as libc::c_int;
    (*cmsg)
        .cmsg_len = ((::std::mem::size_of::<cmsghdr>() as libc::c_ulong)
        .wrapping_add(::std::mem::size_of::<size_t>() as libc::c_ulong)
        .wrapping_sub(1 as libc::c_int as libc::c_ulong)
        & !(::std::mem::size_of::<size_t>() as libc::c_ulong)
            .wrapping_sub(1 as libc::c_int as libc::c_ulong))
        .wrapping_add(::std::mem::size_of::<libc::c_int>() as libc::c_ulong);
    bcopy(
        &mut fd as *mut libc::c_int as *const libc::c_void,
        ((*cmsg).__cmsg_data).as_mut_ptr() as *mut libc::c_void,
        ::std::mem::size_of::<libc::c_int>() as libc::c_ulong,
    );
    msg.msg_controllen = (*cmsg).cmsg_len;
    loop {
        r = sendmsg(s, &mut msg, 0 as libc::c_int) as libc::c_int;
        if r == -(1 as libc::c_int) && *__errno_location() == 4 as libc::c_int {
            continue;
        }
        if r == -(1 as libc::c_int) {
            return -(1 as libc::c_int);
        }
        return 0 as libc::c_int;
    };
}
pub unsafe extern "C" fn IsSocket(mut path: *const libc::c_char) -> bool {
    let mut st: stat = stat {
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
    if stat(path, &mut st) < 0 as libc::c_int {
        return 0 as libc::c_int != 0;
    }
    return st.st_mode & 0o170000 as libc::c_int as libc::c_uint
        == 0o140000 as libc::c_int as libc::c_uint;
}
