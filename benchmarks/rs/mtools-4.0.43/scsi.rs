use ::libc;
extern "C" {
    fn perror(__s: *const libc::c_char);
    fn open(__file: *const libc::c_char, __oflag: libc::c_int, _: ...) -> libc::c_int;
    fn ioctl(__fd: libc::c_int, __request: libc::c_ulong, _: ...) -> libc::c_int;
    fn memset(
        _: *mut libc::c_void,
        _: libc::c_int,
        _: libc::c_ulong,
    ) -> *mut libc::c_void;
}
pub type __uint8_t = libc::c_uchar;
pub type __uint32_t = libc::c_uint;
pub type uint8_t = __uint8_t;
pub type uint32_t = __uint32_t;
pub type scsi_io_mode_t = libc::c_uint;
pub const SCSI_IO_WRITE: scsi_io_mode_t = 1;
pub const SCSI_IO_READ: scsi_io_mode_t = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct sg_io_hdr {
    pub interface_id: libc::c_int,
    pub dxfer_direction: libc::c_int,
    pub cmd_len: libc::c_uchar,
    pub mx_sb_len: libc::c_uchar,
    pub iovec_count: libc::c_ushort,
    pub dxfer_len: libc::c_uint,
    pub dxferp: *mut libc::c_void,
    pub cmdp: *mut libc::c_uchar,
    pub sbp: *mut libc::c_uchar,
    pub timeout: libc::c_uint,
    pub flags: libc::c_uint,
    pub pack_id: libc::c_int,
    pub usr_ptr: *mut libc::c_void,
    pub status: libc::c_uchar,
    pub masked_status: libc::c_uchar,
    pub msg_status: libc::c_uchar,
    pub sb_len_wr: libc::c_uchar,
    pub host_status: libc::c_ushort,
    pub driver_status: libc::c_ushort,
    pub resid: libc::c_int,
    pub duration: libc::c_uint,
    pub info: libc::c_uint,
}
pub unsafe extern "C" fn scsi_max_length() -> libc::c_uint {
    return 8 as libc::c_int as libc::c_uint;
}
pub unsafe extern "C" fn scsi_open(
    mut name: *const libc::c_char,
    mut flag: libc::c_int,
    mut mode: libc::c_int,
    mut extra_data: *mut *mut libc::c_void,
) -> libc::c_int {
    return open(
        name,
        0 as libc::c_int | 0 as libc::c_int | 0 as libc::c_int | 0o4000 as libc::c_int,
    );
}
pub unsafe extern "C" fn scsi_cmd(
    mut fd: libc::c_int,
    mut cdb: *mut libc::c_uchar,
    mut cmdlen: uint8_t,
    mut mode: scsi_io_mode_t,
    mut data: *mut libc::c_void,
    mut len: uint32_t,
    mut extra_data: *mut libc::c_void,
) -> libc::c_int {
    let mut my_scsi_cmd: sg_io_hdr = sg_io_hdr {
        interface_id: 0,
        dxfer_direction: 0,
        cmd_len: 0,
        mx_sb_len: 0,
        iovec_count: 0,
        dxfer_len: 0,
        dxferp: 0 as *mut libc::c_void,
        cmdp: 0 as *mut libc::c_uchar,
        sbp: 0 as *mut libc::c_uchar,
        timeout: 0,
        flags: 0,
        pack_id: 0,
        usr_ptr: 0 as *mut libc::c_void,
        status: 0,
        masked_status: 0,
        msg_status: 0,
        sb_len_wr: 0,
        host_status: 0,
        driver_status: 0,
        resid: 0,
        duration: 0,
        info: 0,
    };
    memset(
        &mut my_scsi_cmd as *mut sg_io_hdr as *mut libc::c_void,
        0 as libc::c_int,
        ::std::mem::size_of::<sg_io_hdr>() as libc::c_ulong,
    );
    my_scsi_cmd.interface_id = 'S' as i32;
    my_scsi_cmd
        .dxfer_direction = if mode as libc::c_uint
        == SCSI_IO_READ as libc::c_int as libc::c_uint
    {
        -(3 as libc::c_int)
    } else {
        -(2 as libc::c_int)
    };
    my_scsi_cmd.cmd_len = cmdlen;
    my_scsi_cmd.mx_sb_len = 0 as libc::c_int as libc::c_uchar;
    my_scsi_cmd.dxfer_len = len;
    my_scsi_cmd.dxferp = data;
    my_scsi_cmd.cmdp = cdb;
    my_scsi_cmd.timeout = !(0 as libc::c_uint);
    if ioctl(
        fd,
        0x2285 as libc::c_int as libc::c_ulong,
        &mut my_scsi_cmd as *mut sg_io_hdr,
    ) < 0 as libc::c_int
    {
        perror(b"scsi_io\0" as *const u8 as *const libc::c_char);
        return -(1 as libc::c_int);
    }
    return my_scsi_cmd.status as libc::c_int & 0x3e as libc::c_int;
}
