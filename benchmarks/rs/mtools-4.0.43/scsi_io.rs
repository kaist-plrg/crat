use ::libc;
extern "C" {
    pub type _IO_wide_data;
    pub type _IO_codecvt;
    pub type _IO_marker;
    pub type doscp_t;
    fn calloc(_: libc::c_ulong, _: libc::c_ulong) -> *mut libc::c_void;
    fn free(__ptr: *mut libc::c_void);
    fn exit(_: libc::c_int) -> !;
    static mut stderr: *mut FILE;
    fn fprintf(_: *mut FILE, _: *const libc::c_char, _: ...) -> libc::c_int;
    fn snprintf(
        _: *mut libc::c_char,
        _: libc::c_ulong,
        _: *const libc::c_char,
        _: ...
    ) -> libc::c_int;
    fn perror(__s: *const libc::c_char);
    fn close(__fd: libc::c_int) -> libc::c_int;
    fn __errno_location() -> *mut libc::c_int;
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
    fn strerror(_: libc::c_int) -> *mut libc::c_char;
    fn reclaim_privs();
    fn drop_privs();
    fn closeExec(fd: libc::c_int);
    fn printOom();
    fn log_2(_: libc::c_uint) -> libc::c_uint;
    fn truncMtOffTo32u(off: mt_off_t) -> uint32_t;
    fn precmd(dev: *mut device);
    fn postcmd(cmd: *const libc::c_char);
    fn init_head(Stream: *mut Stream_t, Class: *mut Class_t, Next: *mut Stream_t);
    fn set_geom_noop(
        Stream: *mut Stream_t,
        dev: *mut device_t,
        orig_dev: *mut device_t,
    ) -> libc::c_int;
    fn scsi_max_length() -> libc::c_uint;
    fn scsi_cmd(
        fd: libc::c_int,
        cdb: *mut libc::c_uchar,
        clen: uint8_t,
        mode: scsi_io_mode_t,
        data: *mut libc::c_void,
        len: uint32_t,
        extra_data: *mut libc::c_void,
    ) -> libc::c_int;
    fn scsi_open(
        name: *const libc::c_char,
        flags: libc::c_int,
        mode: libc::c_int,
        extra_data: *mut *mut libc::c_void,
    ) -> libc::c_int;
    fn LockDevice(
        fd: libc::c_int,
        dev: *mut device,
        locked: libc::c_int,
        lockMode: libc::c_int,
        errmsg: *mut libc::c_char,
    ) -> libc::c_int;
}
pub type __uint8_t = libc::c_uchar;
pub type __uint16_t = libc::c_ushort;
pub type __uint32_t = libc::c_uint;
pub type __off_t = libc::c_long;
pub type __off64_t = libc::c_long;
pub type __time_t = libc::c_long;
pub type __ssize_t = libc::c_long;
pub type off_t = __off_t;
pub type ssize_t = __ssize_t;
pub type time_t = __time_t;
pub type size_t = libc::c_ulong;
pub type uint8_t = __uint8_t;
pub type uint16_t = __uint16_t;
pub type uint32_t = __uint32_t;
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct Stream_t {
    pub Class: *mut Class_t,
    pub refs: libc::c_int,
    pub Next: *mut Stream_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct Class_t {
    pub read: Option::<
        unsafe extern "C" fn(*mut Stream_t, *mut libc::c_char, size_t) -> ssize_t,
    >,
    pub write: Option::<
        unsafe extern "C" fn(*mut Stream_t, *mut libc::c_char, size_t) -> ssize_t,
    >,
    pub pread: Option::<
        unsafe extern "C" fn(
            *mut Stream_t,
            *mut libc::c_char,
            mt_off_t,
            size_t,
        ) -> ssize_t,
    >,
    pub pwrite: Option::<
        unsafe extern "C" fn(
            *mut Stream_t,
            *mut libc::c_char,
            mt_off_t,
            size_t,
        ) -> ssize_t,
    >,
    pub flush: Option::<unsafe extern "C" fn(*mut Stream_t) -> libc::c_int>,
    pub freeFunc: Option::<unsafe extern "C" fn(*mut Stream_t) -> libc::c_int>,
    pub set_geom: Option::<
        unsafe extern "C" fn(*mut Stream_t, *mut device_t, *mut device_t) -> libc::c_int,
    >,
    pub get_data: Option::<
        unsafe extern "C" fn(
            *mut Stream_t,
            *mut time_t,
            *mut mt_off_t,
            *mut libc::c_int,
            *mut uint32_t,
        ) -> libc::c_int,
    >,
    pub pre_allocate: Option::<
        unsafe extern "C" fn(*mut Stream_t, mt_off_t) -> libc::c_int,
    >,
    pub get_dosConvert: Option::<unsafe extern "C" fn(*mut Stream_t) -> *mut doscp_t>,
    pub discard: Option::<unsafe extern "C" fn(*mut Stream_t) -> libc::c_int>,
}
pub type mt_off_t = off_t;
pub type device_t = device;
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
pub type smt_off_t = mt_off_t;
pub type scsi_io_mode_t = libc::c_uint;
pub const SCSI_IO_WRITE: scsi_io_mode_t = 1;
pub const SCSI_IO_READ: scsi_io_mode_t = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ScsiDevice_t {
    pub head: Stream_t,
    pub fd: libc::c_int,
    pub privileged: libc::c_int,
    pub scsi_sector_size: uint32_t,
    pub device_size: mt_off_t,
    pub tot_sectors: uint32_t,
    pub extra_data: *mut libc::c_void,
    pub postcmd: *const libc::c_char,
}
unsafe extern "C" fn scsi_init(mut This: *mut ScsiDevice_t) -> libc::c_int {
    let mut fd: libc::c_int = (*This).fd;
    let mut cdb: [libc::c_uchar; 10] = [0; 10];
    let mut buf: [libc::c_uchar; 8] = [0; 8];
    memset(
        cdb.as_mut_ptr() as *mut libc::c_void,
        0 as libc::c_int,
        ::std::mem::size_of::<[libc::c_uchar; 10]>() as libc::c_ulong,
    );
    memset(
        buf.as_mut_ptr() as *mut libc::c_void,
        0 as libc::c_int,
        ::std::mem::size_of::<[libc::c_uchar; 8]>() as libc::c_ulong,
    );
    cdb[0 as libc::c_int as usize] = 0x25 as libc::c_int as libc::c_uchar;
    if scsi_cmd(
        fd,
        cdb.as_mut_ptr(),
        ::std::mem::size_of::<[libc::c_uchar; 10]>() as libc::c_ulong as uint8_t,
        SCSI_IO_READ,
        buf.as_mut_ptr() as *mut libc::c_void,
        ::std::mem::size_of::<[libc::c_uchar; 8]>() as libc::c_ulong as uint32_t,
        (*This).extra_data,
    ) == 0 as libc::c_int
    {
        (*This)
            .tot_sectors = (buf[0 as libc::c_int as usize] as libc::c_uint)
            << 24 as libc::c_int
            | (buf[1 as libc::c_int as usize] as libc::c_uint) << 16 as libc::c_int
            | (buf[2 as libc::c_int as usize] as libc::c_uint) << 8 as libc::c_int
            | buf[3 as libc::c_int as usize] as libc::c_uint;
        if (*This).tot_sectors < 4294967295 as libc::c_uint {
            (*This).tot_sectors = ((*This).tot_sectors).wrapping_add(1);
            (*This).tot_sectors;
        }
        (*This)
            .scsi_sector_size = (buf[5 as libc::c_int as usize] as libc::c_uint)
            << 16 as libc::c_int
            | (buf[6 as libc::c_int as usize] as libc::c_uint) << 8 as libc::c_int
            | buf[7 as libc::c_int as usize] as libc::c_uint;
        if (*This).scsi_sector_size != 512 as libc::c_int as libc::c_uint {
            fprintf(
                stderr,
                b"  (scsi_sector_size=%d)\n\0" as *const u8 as *const libc::c_char,
                (*This).scsi_sector_size,
            );
        }
        return 0 as libc::c_int;
    } else {
        return -(1 as libc::c_int)
    };
}
unsafe extern "C" fn bytesToSectors(
    mut bytes: size_t,
    mut sector_size: uint32_t,
) -> uint32_t {
    let mut sectors: size_t = bytes.wrapping_div(sector_size as libc::c_ulong);
    if bytes.wrapping_rem(sector_size as libc::c_ulong) != 0 {
        sectors = sectors.wrapping_add(1);
        sectors;
    }
    if sectors > 4294967295 as libc::c_uint as libc::c_ulong {
        return 4294967295 as libc::c_uint
    } else {
        return sectors as uint32_t
    };
}
unsafe extern "C" fn scsi_io(
    mut Stream: *mut Stream_t,
    mut buf: *mut libc::c_char,
    mut where_0: mt_off_t,
    mut len: size_t,
    mut rwcmd: scsi_io_mode_t,
) -> ssize_t {
    let mut firstblock: libc::c_uint = 0;
    let mut nsect: libc::c_uint = 0;
    let mut clen: uint8_t = 0;
    let mut r: libc::c_int = 0;
    let mut max: libc::c_uint = 0;
    let mut offset: uint32_t = 0;
    let mut cdb: [libc::c_uchar; 10] = [0; 10];
    let mut This: *mut ScsiDevice_t = Stream as *mut ScsiDevice_t;
    firstblock = truncMtOffTo32u(where_0 / (*This).scsi_sector_size as mt_off_t);
    offset = (where_0 % (*This).scsi_sector_size as libc::c_long) as uint32_t;
    nsect = bytesToSectors(
        (offset as libc::c_ulong).wrapping_add(len),
        (*This).scsi_sector_size,
    );
    if len > 512 as libc::c_int as libc::c_ulong {
        while nsect.wrapping_mul((*This).scsi_sector_size) as libc::c_ulong > len {
            nsect = nsect.wrapping_sub(1);
            nsect;
        }
        if nsect == 0 {
            fprintf(
                stderr,
                b"Scsi buffer too small\n\0" as *const u8 as *const libc::c_char,
            );
            exit(1 as libc::c_int);
        }
        if rwcmd as libc::c_uint == SCSI_IO_WRITE as libc::c_int as libc::c_uint
            && offset != 0
        {
            fprintf(stderr, b"Unaligned write\n\0" as *const u8 as *const libc::c_char);
            exit(1 as libc::c_int);
        }
    }
    max = scsi_max_length();
    if nsect > max {
        nsect = max;
    }
    memset(
        cdb.as_mut_ptr() as *mut libc::c_void,
        0 as libc::c_int,
        ::std::mem::size_of::<[libc::c_uchar; 10]>() as libc::c_ulong,
    );
    match rwcmd as libc::c_uint {
        0 => {
            cdb[0 as libc::c_int as usize] = 0x8 as libc::c_int as libc::c_uchar;
        }
        1 => {
            cdb[0 as libc::c_int as usize] = 0xa as libc::c_int as libc::c_uchar;
        }
        _ => {}
    }
    cdb[1 as libc::c_int as usize] = 0 as libc::c_int as libc::c_uchar;
    if firstblock > 0x1fffff as libc::c_int as libc::c_uint
        || nsect > 0xff as libc::c_int as libc::c_uint
    {
        cdb[0 as libc::c_int
            as usize] = (cdb[0 as libc::c_int as usize] as libc::c_int
            | 0x20 as libc::c_int) as libc::c_uchar;
        clen = 10 as libc::c_int as uint8_t;
        cdb[2 as libc::c_int
            as usize] = ((firstblock >> 24 as libc::c_int) as libc::c_uchar
            as libc::c_int & 0xff as libc::c_int) as libc::c_uchar;
        cdb[3 as libc::c_int
            as usize] = ((firstblock >> 16 as libc::c_int) as libc::c_uchar
            as libc::c_int & 0xff as libc::c_int) as libc::c_uchar;
        cdb[4 as libc::c_int
            as usize] = ((firstblock >> 8 as libc::c_int) as libc::c_uchar as libc::c_int
            & 0xff as libc::c_int) as libc::c_uchar;
        cdb[5 as libc::c_int
            as usize] = (firstblock as libc::c_uchar as libc::c_int
            & 0xff as libc::c_int) as libc::c_uchar;
        cdb[6 as libc::c_int as usize] = 0 as libc::c_int as libc::c_uchar;
        cdb[7 as libc::c_int
            as usize] = ((nsect >> 8 as libc::c_int) as libc::c_uchar as libc::c_int
            & 0xff as libc::c_int) as libc::c_uchar;
        cdb[8 as libc::c_int
            as usize] = (nsect as libc::c_uchar as libc::c_int & 0xff as libc::c_int)
            as libc::c_uchar;
        cdb[9 as libc::c_int as usize] = 0 as libc::c_int as libc::c_uchar;
    } else {
        clen = 6 as libc::c_int as uint8_t;
        cdb[1 as libc::c_int
            as usize] = (cdb[1 as libc::c_int as usize] as libc::c_int
            | (firstblock >> 16 as libc::c_int & 0x1f as libc::c_int as libc::c_uint)
                as libc::c_uchar as libc::c_int) as libc::c_uchar;
        cdb[2 as libc::c_int
            as usize] = (firstblock >> 8 as libc::c_int
            & 0xff as libc::c_int as libc::c_uint) as libc::c_uchar;
        cdb[3 as libc::c_int
            as usize] = (firstblock as libc::c_uchar as libc::c_int
            & 0xff as libc::c_int) as libc::c_uchar;
        cdb[4 as libc::c_int as usize] = nsect as libc::c_uchar;
        cdb[5 as libc::c_int as usize] = 0 as libc::c_int as libc::c_uchar;
    }
    if (*This).privileged != 0 {
        reclaim_privs();
    }
    r = scsi_cmd(
        (*This).fd,
        cdb.as_mut_ptr(),
        clen,
        rwcmd,
        buf as *mut libc::c_void,
        nsect.wrapping_mul((*This).scsi_sector_size),
        (*This).extra_data,
    );
    if (*This).privileged != 0 {
        drop_privs();
    }
    if r != 0 {
        perror(
            if rwcmd as libc::c_uint == SCSI_IO_READ as libc::c_int as libc::c_uint {
                b"SCMD_READ\0" as *const u8 as *const libc::c_char
            } else {
                b"SCMD_WRITE\0" as *const u8 as *const libc::c_char
            },
        );
        return -(1 as libc::c_int) as ssize_t;
    }
    if offset > 0 as libc::c_int as libc::c_uint {
        memmove(
            buf as *mut libc::c_void,
            buf.offset(offset as isize) as *const libc::c_void,
            nsect.wrapping_mul((*This).scsi_sector_size).wrapping_sub(offset)
                as libc::c_ulong,
        );
    }
    if len == 256 as libc::c_int as libc::c_ulong {
        return 256 as libc::c_int as ssize_t
    } else if len == 512 as libc::c_int as libc::c_ulong {
        return 512 as libc::c_int as ssize_t
    } else {
        return nsect.wrapping_mul((*This).scsi_sector_size).wrapping_sub(offset)
            as ssize_t
    };
}
unsafe extern "C" fn scsi_pread(
    mut Stream: *mut Stream_t,
    mut buf: *mut libc::c_char,
    mut where_0: mt_off_t,
    mut len: size_t,
) -> ssize_t {
    return scsi_io(Stream, buf, where_0, len, SCSI_IO_READ);
}
unsafe extern "C" fn scsi_pwrite(
    mut Stream: *mut Stream_t,
    mut buf: *mut libc::c_char,
    mut where_0: mt_off_t,
    mut len: size_t,
) -> ssize_t {
    return scsi_io(Stream, buf, where_0, len, SCSI_IO_WRITE);
}
unsafe extern "C" fn scsi_get_data(
    mut Stream: *mut Stream_t,
    mut date: *mut time_t,
    mut size: *mut mt_off_t,
    mut type_0: *mut libc::c_int,
    mut address: *mut uint32_t,
) -> libc::c_int {
    let mut This: *mut ScsiDevice_t = Stream as *mut ScsiDevice_t;
    if !date.is_null() || !type_0.is_null() || !address.is_null() {
        fprintf(
            stderr,
            b"Get_data call not supported\n\0" as *const u8 as *const libc::c_char,
        );
    }
    if !size.is_null() {
        *size = (*This).device_size;
    }
    return 0 as libc::c_int;
}
unsafe extern "C" fn scsi_free(mut Stream: *mut Stream_t) -> libc::c_int {
    let mut This: *mut ScsiDevice_t = Stream as *mut ScsiDevice_t;
    if (*This).fd > 2 as libc::c_int {
        let mut ret: libc::c_int = close((*This).fd);
        postcmd((*This).postcmd);
        return ret;
    } else {
        return 0 as libc::c_int
    };
}
static mut ScsiDeviceClass: Class_t = unsafe {
    {
        let mut init = Class_t {
            read: None,
            write: None,
            pread: Some(
                scsi_pread
                    as unsafe extern "C" fn(
                        *mut Stream_t,
                        *mut libc::c_char,
                        mt_off_t,
                        size_t,
                    ) -> ssize_t,
            ),
            pwrite: Some(
                scsi_pwrite
                    as unsafe extern "C" fn(
                        *mut Stream_t,
                        *mut libc::c_char,
                        mt_off_t,
                        size_t,
                    ) -> ssize_t,
            ),
            flush: None,
            freeFunc: Some(
                scsi_free as unsafe extern "C" fn(*mut Stream_t) -> libc::c_int,
            ),
            set_geom: Some(
                set_geom_noop
                    as unsafe extern "C" fn(
                        *mut Stream_t,
                        *mut device_t,
                        *mut device_t,
                    ) -> libc::c_int,
            ),
            get_data: Some(
                scsi_get_data
                    as unsafe extern "C" fn(
                        *mut Stream_t,
                        *mut time_t,
                        *mut mt_off_t,
                        *mut libc::c_int,
                        *mut uint32_t,
                    ) -> libc::c_int,
            ),
            pre_allocate: None,
            get_dosConvert: None,
            discard: None,
        };
        init
    }
};
pub unsafe extern "C" fn OpenScsi(
    mut dev: *mut device,
    mut name: *const libc::c_char,
    mut mode: libc::c_int,
    mut errmsg: *mut libc::c_char,
    mut mode2: libc::c_int,
    mut locked: libc::c_int,
    mut lockMode: libc::c_int,
    mut maxSize: *mut mt_off_t,
) -> *mut Stream_t {
    let mut ret: libc::c_int = 0;
    let mut This: *mut ScsiDevice_t = 0 as *mut ScsiDevice_t;
    if !(!dev.is_null() && (*dev).misc_flags & 0x1 as libc::c_uint != 0) {
        return 0 as *mut Stream_t;
    }
    This = calloc(
        1 as libc::c_int as libc::c_ulong,
        ::std::mem::size_of::<ScsiDevice_t>() as libc::c_ulong,
    ) as *mut ScsiDevice_t;
    if This.is_null() {
        printOom();
        return 0 as *mut Stream_t;
    }
    memset(
        This as *mut libc::c_void,
        0 as libc::c_int,
        ::std::mem::size_of::<ScsiDevice_t>() as libc::c_ulong,
    );
    init_head(&mut (*This).head, &mut ScsiDeviceClass, 0 as *mut Stream_t);
    (*This).scsi_sector_size = 512 as libc::c_int as uint32_t;
    if !dev.is_null() {
        if mode2 & 1 as libc::c_int == 0 {
            (*This)
                .privileged = (!dev.is_null()
                && (*dev).misc_flags & 0x2 as libc::c_uint != 0) as libc::c_int;
        }
        mode |= (*dev).mode;
    }
    precmd(dev);
    if !dev.is_null() {
        (*This).postcmd = (*dev).postcmd;
    }
    if !dev.is_null() && (*dev).misc_flags & 0x2 as libc::c_uint != 0
        && mode2 & 1 as libc::c_int == 0
    {
        reclaim_privs();
    }
    (*This)
        .fd = scsi_open(
        name,
        mode,
        if !dev.is_null() && (*dev).misc_flags & 0x4 as libc::c_uint != 0 {
            0o444 as libc::c_int
        } else {
            0o666 as libc::c_int
        },
        &mut (*This).extra_data,
    );
    if !dev.is_null() && (*dev).misc_flags & 0x2 as libc::c_uint != 0
        && mode2 & 1 as libc::c_int == 0
    {
        drop_privs();
    }
    if (*This).fd < 0 as libc::c_int {
        if !errmsg.is_null() {
            snprintf(
                errmsg,
                199 as libc::c_int as libc::c_ulong,
                b"Can't open %s: %s\0" as *const u8 as *const libc::c_char,
                name,
                strerror(*__errno_location()),
            );
        }
    } else {
        if !dev.is_null() && (*dev).misc_flags & 0x2 as libc::c_uint != 0
            && mode2 & 1 as libc::c_int == 0
        {
            closeExec((*This).fd);
        }
        if !(LockDevice((*This).fd, dev, locked, lockMode, errmsg) < 0 as libc::c_int) {
            if !maxSize.is_null() {
                *maxSize = (((1 as libc::c_int as mt_off_t)
                    << (if ((31 as libc::c_int as libc::c_uint)
                        .wrapping_add(log_2((*This).scsi_sector_size))
                        .wrapping_sub(1 as libc::c_int as libc::c_uint) as libc::c_ulong)
                        < (::std::mem::size_of::<mt_off_t>() as libc::c_ulong)
                            .wrapping_mul(8 as libc::c_int as libc::c_ulong)
                            .wrapping_sub(2 as libc::c_int as libc::c_ulong)
                    {
                        (31 as libc::c_int as libc::c_uint)
                            .wrapping_add(log_2((*This).scsi_sector_size))
                            .wrapping_sub(1 as libc::c_int as libc::c_uint)
                            as libc::c_ulong
                    } else {
                        (::std::mem::size_of::<mt_off_t>() as libc::c_ulong)
                            .wrapping_mul(8 as libc::c_int as libc::c_ulong)
                            .wrapping_sub(2 as libc::c_int as libc::c_ulong)
                    })) - 1 as libc::c_int as libc::c_long) << 1 as libc::c_int
                    | 1 as libc::c_int as libc::c_long;
            }
            if (*This).privileged != 0 {
                reclaim_privs();
            }
            ret = scsi_init(This);
            if (*This).privileged != 0 {
                drop_privs();
            }
            if !(ret < 0 as libc::c_int) {
                (*dev).tot_sectors = (*This).tot_sectors;
                return &mut (*This).head;
            }
        }
        close((*This).fd);
        postcmd((*This).postcmd);
    }
    free(This as *mut libc::c_char as *mut libc::c_void);
    return 0 as *mut Stream_t;
}
