use ::libc;
extern "C" {
    pub type _IO_wide_data;
    pub type _IO_codecvt;
    pub type _IO_marker;
    fn calloc(_: libc::c_ulong, _: libc::c_ulong) -> *mut libc::c_void;
    fn free(__ptr: *mut libc::c_void);
    fn exit(_: libc::c_int) -> !;
    static mut stderr: *mut FILE;
    fn fprintf(_: *mut FILE, _: *const libc::c_char, _: ...) -> libc::c_int;
    fn perror(__s: *const libc::c_char);
    fn close(__fd: libc::c_int) -> libc::c_int;
    fn read(__fd: libc::c_int, __buf: *mut libc::c_void, __nbytes: size_t) -> ssize_t;
    fn write(__fd: libc::c_int, __buf: *const libc::c_void, __n: size_t) -> ssize_t;
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
    fn ioctl(__fd: libc::c_int, __request: libc::c_ulong, _: ...) -> libc::c_int;
    fn open(__file: *const libc::c_char, __oflag: libc::c_int, _: ...) -> libc::c_int;
    fn run_cmd(cmd: *mut libc::c_char, _: ...) -> libc::c_int;
}
pub type size_t = libc::c_ulong;
pub type __off_t = libc::c_long;
pub type __off64_t = libc::c_long;
pub type __ssize_t = libc::c_long;
pub type ssize_t = __ssize_t;
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
pub type sa_family_t = libc::c_ushort;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct sockaddr {
    pub sa_family: sa_family_t,
    pub sa_data: [libc::c_char; 14],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct sync_serial_settings {
    pub clock_rate: libc::c_uint,
    pub clock_type: libc::c_uint,
    pub loopback: libc::c_ushort,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct te1_settings {
    pub clock_rate: libc::c_uint,
    pub clock_type: libc::c_uint,
    pub loopback: libc::c_ushort,
    pub slot_map: libc::c_uint,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct raw_hdlc_proto {
    pub encoding: libc::c_ushort,
    pub parity: libc::c_ushort,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct fr_proto {
    pub t391: libc::c_uint,
    pub t392: libc::c_uint,
    pub n391: libc::c_uint,
    pub n392: libc::c_uint,
    pub n393: libc::c_uint,
    pub lmi: libc::c_ushort,
    pub dce: libc::c_ushort,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct fr_proto_pvc {
    pub dlci: libc::c_uint,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct fr_proto_pvc_info {
    pub dlci: libc::c_uint,
    pub master: [libc::c_char; 16],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct cisco_proto {
    pub interval: libc::c_uint,
    pub timeout: libc::c_uint,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ifmap {
    pub mem_start: libc::c_ulong,
    pub mem_end: libc::c_ulong,
    pub base_addr: libc::c_ushort,
    pub irq: libc::c_uchar,
    pub dma: libc::c_uchar,
    pub port: libc::c_uchar,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct if_settings {
    pub type_0: libc::c_uint,
    pub size: libc::c_uint,
    pub ifs_ifsu: C2RustUnnamed,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed {
    pub raw_hdlc: *mut raw_hdlc_proto,
    pub cisco: *mut cisco_proto,
    pub fr: *mut fr_proto,
    pub fr_pvc: *mut fr_proto_pvc,
    pub fr_pvc_info: *mut fr_proto_pvc_info,
    pub sync: *mut sync_serial_settings,
    pub te1: *mut te1_settings,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ifreq {
    pub ifr_ifrn: C2RustUnnamed_1,
    pub ifr_ifru: C2RustUnnamed_0,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_0 {
    pub ifru_addr: sockaddr,
    pub ifru_dstaddr: sockaddr,
    pub ifru_broadaddr: sockaddr,
    pub ifru_netmask: sockaddr,
    pub ifru_hwaddr: sockaddr,
    pub ifru_flags: libc::c_short,
    pub ifru_ivalue: libc::c_int,
    pub ifru_mtu: libc::c_int,
    pub ifru_map: ifmap,
    pub ifru_slave: [libc::c_char; 16],
    pub ifru_newname: [libc::c_char; 16],
    pub ifru_data: *mut libc::c_void,
    pub ifru_settings: if_settings,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_1 {
    pub ifrn_name: [libc::c_char; 16],
}
static mut tun_fd: libc::c_int = 0;
static mut dev: *mut libc::c_char = 0 as *const libc::c_char as *mut libc::c_char;
pub static mut tapaddr: *mut libc::c_char = b"10.0.0.5\0" as *const u8
    as *const libc::c_char as *mut libc::c_char;
pub static mut taproute: *mut libc::c_char = b"10.0.0.0/24\0" as *const u8
    as *const libc::c_char as *mut libc::c_char;
unsafe extern "C" fn set_if_route(
    mut dev_0: *mut libc::c_char,
    mut cidr: *mut libc::c_char,
) -> libc::c_int {
    return run_cmd(
        b"ip route add dev %s %s\0" as *const u8 as *const libc::c_char
            as *mut libc::c_char,
        dev_0,
        cidr,
    );
}
unsafe extern "C" fn set_if_address(
    mut dev_0: *mut libc::c_char,
    mut cidr: *mut libc::c_char,
) -> libc::c_int {
    return run_cmd(
        b"ip address add dev %s local %s\0" as *const u8 as *const libc::c_char
            as *mut libc::c_char,
        dev_0,
        cidr,
    );
}
unsafe extern "C" fn set_if_up(mut dev_0: *mut libc::c_char) -> libc::c_int {
    return run_cmd(
        b"ip link set dev %s up\0" as *const u8 as *const libc::c_char
            as *mut libc::c_char,
        dev_0,
    );
}
unsafe extern "C" fn tun_alloc(mut dev_0: *mut libc::c_char) -> libc::c_int {
    let mut ifr: ifreq = ifreq {
        ifr_ifrn: C2RustUnnamed_1 {
            ifrn_name: [0; 16],
        },
        ifr_ifru: C2RustUnnamed_0 {
            ifru_addr: sockaddr {
                sa_family: 0,
                sa_data: [0; 14],
            },
        },
    };
    let mut fd: libc::c_int = 0;
    let mut err: libc::c_int = 0;
    fd = open(b"/dev/net/tap\0" as *const u8 as *const libc::c_char, 0o2 as libc::c_int);
    if fd < 0 as libc::c_int {
        perror(
            b"Cannot open TUN/TAP dev\nMake sure one exists with '$ mknod /dev/net/tap c 10 200'\0"
                as *const u8 as *const libc::c_char,
        );
        exit(1 as libc::c_int);
    }
    memset(
        &mut ifr as *mut ifreq as *mut libc::c_void,
        0 as libc::c_int,
        ::std::mem::size_of::<ifreq>() as libc::c_ulong,
    );
    ifr
        .ifr_ifru
        .ifru_flags = (0x2 as libc::c_int | 0x1000 as libc::c_int) as libc::c_short;
    if *dev_0 != 0 {
        strncpy(
            (ifr.ifr_ifrn.ifrn_name).as_mut_ptr(),
            dev_0,
            16 as libc::c_int as libc::c_ulong,
        );
    }
    err = ioctl(
        fd,
        ((1 as libc::c_uint)
            << 0 as libc::c_int + 8 as libc::c_int + 8 as libc::c_int + 14 as libc::c_int
            | (('T' as i32) << 0 as libc::c_int + 8 as libc::c_int) as libc::c_uint
            | ((202 as libc::c_int) << 0 as libc::c_int) as libc::c_uint)
            as libc::c_ulong
            | (::std::mem::size_of::<libc::c_int>() as libc::c_ulong)
                << 0 as libc::c_int + 8 as libc::c_int + 8 as libc::c_int,
        &mut ifr as *mut ifreq as *mut libc::c_void,
    );
    if err < 0 as libc::c_int {
        perror(b"ERR: Could not ioctl tun\0" as *const u8 as *const libc::c_char);
        close(fd);
        return err;
    }
    strcpy(dev_0, (ifr.ifr_ifrn.ifrn_name).as_mut_ptr());
    return fd;
}
pub unsafe extern "C" fn tun_read(
    mut buf: *mut libc::c_char,
    mut len: libc::c_int,
) -> libc::c_int {
    return read(tun_fd, buf as *mut libc::c_void, len as size_t) as libc::c_int;
}
pub unsafe extern "C" fn tun_write(
    mut buf: *mut libc::c_char,
    mut len: libc::c_int,
) -> libc::c_int {
    return write(tun_fd, buf as *const libc::c_void, len as size_t) as libc::c_int;
}
pub unsafe extern "C" fn tun_init() {
    dev = calloc(10 as libc::c_int as libc::c_ulong, 1 as libc::c_int as libc::c_ulong)
        as *mut libc::c_char;
    tun_fd = tun_alloc(dev);
    if set_if_up(dev) != 0 as libc::c_int {
        fprintf(
            stderr,
            b"ERROR when setting up if\n\0" as *const u8 as *const libc::c_char,
        );
    }
    if set_if_route(dev, taproute) != 0 as libc::c_int {
        fprintf(
            stderr,
            b"ERROR when setting route for if\n\0" as *const u8 as *const libc::c_char,
        );
    }
    if set_if_address(dev, tapaddr) != 0 as libc::c_int {
        fprintf(
            stderr,
            b"ERROR when setting addr for if\n\0" as *const u8 as *const libc::c_char,
        );
    }
}
pub unsafe extern "C" fn free_tun() {
    free(dev as *mut libc::c_void);
}
