use ::libc;
extern "C" {
    pub type doscp_t;
    fn calloc(_: libc::c_ulong, _: libc::c_ulong) -> *mut libc::c_void;
    fn free(__ptr: *mut libc::c_void);
    fn sprintf(_: *mut libc::c_char, _: *const libc::c_char, _: ...) -> libc::c_int;
    fn snprintf(
        _: *mut libc::c_char,
        _: libc::c_ulong,
        _: *const libc::c_char,
        _: ...
    ) -> libc::c_int;
    fn perror(__s: *const libc::c_char);
    fn close(__fd: libc::c_int) -> libc::c_int;
    fn read(__fd: libc::c_int, __buf: *mut libc::c_void, __nbytes: size_t) -> ssize_t;
    fn write(__fd: libc::c_int, __buf: *const libc::c_void, __n: size_t) -> ssize_t;
    fn open(__file: *const libc::c_char, __oflag: libc::c_int, _: ...) -> libc::c_int;
    fn __fxstat(
        __ver: libc::c_int,
        __fildes: libc::c_int,
        __stat_buf: *mut stat,
    ) -> libc::c_int;
    fn __errno_location() -> *mut libc::c_int;
    fn memset(
        _: *mut libc::c_void,
        _: libc::c_int,
        _: libc::c_ulong,
    ) -> *mut libc::c_void;
    fn strcmp(_: *const libc::c_char, _: *const libc::c_char) -> libc::c_int;
    fn strerror(_: libc::c_int) -> *mut libc::c_char;
    fn init_head(Stream: *mut Stream_t, Class: *mut Class_t, Next: *mut Stream_t);
    fn reclaim_privs();
    fn drop_privs();
    fn closeExec(fd: libc::c_int);
    fn printOom();
    fn lock_dev(fd: libc::c_int, mode: libc::c_int, dev: *mut device) -> libc::c_int;
    fn precmd(dev: *mut device);
    fn postcmd(cmd: *const libc::c_char);
    static max_off_t_seek: mt_off_t;
    fn mt_lseek(fd: libc::c_int, where_0: mt_off_t, whence: libc::c_int) -> libc::c_int;
    fn init_geom(
        fd: libc::c_int,
        dev: *mut device,
        orig_dev: *mut device,
        statbuf: *mut stat,
    ) -> libc::c_int;
}
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
pub type __time_t = libc::c_long;
pub type __blksize_t = libc::c_long;
pub type __blkcnt_t = libc::c_long;
pub type __ssize_t = libc::c_long;
pub type __syscall_slong_t = libc::c_long;
pub type off_t = __off_t;
pub type ssize_t = __ssize_t;
pub type time_t = __time_t;
pub type size_t = libc::c_ulong;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct timespec {
    pub tv_sec: __time_t,
    pub tv_nsec: __syscall_slong_t,
}
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct SimpleFile_t {
    pub head: Stream_t,
    pub statbuf: stat,
    pub fd: libc::c_int,
    pub lastwhere: mt_off_t,
    pub seekable: libc::c_int,
    pub privileged: libc::c_int,
    pub postcmd: *const libc::c_char,
}
pub type iofn = Option::<
    unsafe extern "C" fn(libc::c_int, *mut libc::c_void, size_t) -> ssize_t,
>;
#[inline]
unsafe extern "C" fn fstat(
    mut __fd: libc::c_int,
    mut __statbuf: *mut stat,
) -> libc::c_int {
    return __fxstat(1 as libc::c_int, __fd, __statbuf);
}
unsafe extern "C" fn file_io(
    mut This: *mut SimpleFile_t,
    mut buf: *mut libc::c_char,
    mut where_0: mt_off_t,
    mut len: size_t,
    mut io: iofn,
) -> ssize_t {
    let mut ret: ssize_t = 0;
    if (*This).seekable != 0 && where_0 != (*This).lastwhere {
        if mt_lseek((*This).fd, where_0, 0 as libc::c_int) < 0 as libc::c_int {
            perror(b"seek\0" as *const u8 as *const libc::c_char);
            return -(1 as libc::c_int) as ssize_t;
        }
        (*This).lastwhere = where_0;
    }
    ret = io.unwrap()((*This).fd, buf as *mut libc::c_void, len);
    if ret == -(1 as libc::c_int) as libc::c_long {
        perror(b"plain_io read/write\0" as *const u8 as *const libc::c_char);
        return -(1 as libc::c_int) as ssize_t;
    }
    (*This).lastwhere = where_0 + ret;
    return ret;
}
unsafe extern "C" fn file_read(
    mut Stream: *mut Stream_t,
    mut buf: *mut libc::c_char,
    mut len: size_t,
) -> ssize_t {
    let mut This: *mut SimpleFile_t = Stream as *mut SimpleFile_t;
    return file_io(
        This,
        buf,
        (*This).lastwhere,
        len,
        Some(
            read
                as unsafe extern "C" fn(
                    libc::c_int,
                    *mut libc::c_void,
                    size_t,
                ) -> ssize_t,
        ),
    );
}
unsafe extern "C" fn file_write(
    mut Stream: *mut Stream_t,
    mut buf: *mut libc::c_char,
    mut len: size_t,
) -> ssize_t {
    let mut This: *mut SimpleFile_t = Stream as *mut SimpleFile_t;
    return file_io(
        This,
        buf,
        (*This).lastwhere,
        len,
        ::std::mem::transmute::<
            Option::<
                unsafe extern "C" fn(libc::c_int, *const libc::c_void, size_t) -> ssize_t,
            >,
            iofn,
        >(
            Some(
                write
                    as unsafe extern "C" fn(
                        libc::c_int,
                        *const libc::c_void,
                        size_t,
                    ) -> ssize_t,
            ),
        ),
    );
}
unsafe extern "C" fn file_pread(
    mut Stream: *mut Stream_t,
    mut buf: *mut libc::c_char,
    mut where_0: mt_off_t,
    mut len: size_t,
) -> ssize_t {
    let mut This: *mut SimpleFile_t = Stream as *mut SimpleFile_t;
    return file_io(
        This,
        buf,
        where_0,
        len,
        Some(
            read
                as unsafe extern "C" fn(
                    libc::c_int,
                    *mut libc::c_void,
                    size_t,
                ) -> ssize_t,
        ),
    );
}
unsafe extern "C" fn file_pwrite(
    mut Stream: *mut Stream_t,
    mut buf: *mut libc::c_char,
    mut where_0: mt_off_t,
    mut len: size_t,
) -> ssize_t {
    let mut This: *mut SimpleFile_t = Stream as *mut SimpleFile_t;
    return file_io(
        This,
        buf,
        where_0,
        len,
        ::std::mem::transmute::<
            Option::<
                unsafe extern "C" fn(libc::c_int, *const libc::c_void, size_t) -> ssize_t,
            >,
            iofn,
        >(
            Some(
                write
                    as unsafe extern "C" fn(
                        libc::c_int,
                        *const libc::c_void,
                        size_t,
                    ) -> ssize_t,
            ),
        ),
    );
}
unsafe extern "C" fn file_flush(mut Stream: *mut Stream_t) -> libc::c_int {
    return 0 as libc::c_int;
}
unsafe extern "C" fn file_free(mut Stream: *mut Stream_t) -> libc::c_int {
    let mut This: *mut SimpleFile_t = Stream as *mut SimpleFile_t;
    if (*This).fd > 2 as libc::c_int {
        let mut ret: libc::c_int = close((*This).fd);
        postcmd((*This).postcmd);
        return ret;
    } else {
        return 0 as libc::c_int
    };
}
unsafe extern "C" fn init_geom_with_reg(
    mut fd: libc::c_int,
    mut dev: *mut device,
    mut orig_dev: *mut device,
    mut statbuf: *mut stat,
) -> libc::c_int {
    if (*statbuf).st_mode & 0o170000 as libc::c_int as libc::c_uint
        == 0o100000 as libc::c_int as libc::c_uint
    {
        let mut sectors: mt_off_t = 0;
        if (*statbuf).st_size == 0 as libc::c_int as libc::c_long {
            return 0 as libc::c_int;
        }
        sectors = (*statbuf).st_size
            / (if (*dev).sector_size as libc::c_int != 0 {
                (*dev).sector_size as libc::c_int
            } else {
                512 as libc::c_int
            }) as mt_off_t;
        (*dev)
            .tot_sectors = if sectors > 4294967295 as libc::c_uint as libc::c_long {
            4294967295 as libc::c_uint
        } else {
            sectors as uint32_t
        };
        return 0 as libc::c_int;
    } else {
        return init_geom(fd, dev, orig_dev, statbuf)
    };
}
unsafe extern "C" fn file_geom(
    mut Stream: *mut Stream_t,
    mut dev: *mut device,
    mut orig_dev: *mut device,
) -> libc::c_int {
    let mut ret: libc::c_int = 0;
    let mut This: *mut SimpleFile_t = Stream as *mut SimpleFile_t;
    if (*dev).sector_size as libc::c_int != 0
        && (*dev).sector_size as libc::c_int != 512 as libc::c_int
    {
        (*dev)
            .sectors = ((*dev).sectors as libc::c_int * (*dev).sector_size as libc::c_int
            / 512 as libc::c_int) as uint16_t;
    }
    ret = init_geom_with_reg((*This).fd, dev, orig_dev, &mut (*This).statbuf);
    if (*dev).sector_size as libc::c_int != 0
        && (*dev).sector_size as libc::c_int != 512 as libc::c_int
    {
        (*dev)
            .sectors = ((*dev).sectors as libc::c_int * 512 as libc::c_int
            / (*dev).sector_size as libc::c_int) as uint16_t;
    }
    return ret;
}
unsafe extern "C" fn file_data(
    mut Stream: *mut Stream_t,
    mut date: *mut time_t,
    mut size: *mut mt_off_t,
    mut type_0: *mut libc::c_int,
    mut address: *mut uint32_t,
) -> libc::c_int {
    let mut This: *mut SimpleFile_t = Stream as *mut SimpleFile_t;
    if !date.is_null() {
        *date = (*This).statbuf.st_mtim.tv_sec;
    }
    if !size.is_null() {
        *size = (*This).statbuf.st_size;
    }
    if !type_0.is_null() {
        *type_0 = ((*This).statbuf.st_mode & 0o170000 as libc::c_int as libc::c_uint
            == 0o40000 as libc::c_int as libc::c_uint) as libc::c_int;
    }
    if !address.is_null() {
        *address = 0 as libc::c_int as uint32_t;
    }
    return 0 as libc::c_int;
}
unsafe extern "C" fn file_discard(mut Stream: *mut Stream_t) -> libc::c_int {
    return 0 as libc::c_int;
}
static mut SimpleFileClass: Class_t = unsafe {
    {
        let mut init = Class_t {
            read: Some(
                file_read
                    as unsafe extern "C" fn(
                        *mut Stream_t,
                        *mut libc::c_char,
                        size_t,
                    ) -> ssize_t,
            ),
            write: Some(
                file_write
                    as unsafe extern "C" fn(
                        *mut Stream_t,
                        *mut libc::c_char,
                        size_t,
                    ) -> ssize_t,
            ),
            pread: Some(
                file_pread
                    as unsafe extern "C" fn(
                        *mut Stream_t,
                        *mut libc::c_char,
                        mt_off_t,
                        size_t,
                    ) -> ssize_t,
            ),
            pwrite: Some(
                file_pwrite
                    as unsafe extern "C" fn(
                        *mut Stream_t,
                        *mut libc::c_char,
                        mt_off_t,
                        size_t,
                    ) -> ssize_t,
            ),
            flush: Some(
                file_flush as unsafe extern "C" fn(*mut Stream_t) -> libc::c_int,
            ),
            freeFunc: Some(
                file_free as unsafe extern "C" fn(*mut Stream_t) -> libc::c_int,
            ),
            set_geom: Some(
                file_geom
                    as unsafe extern "C" fn(
                        *mut Stream_t,
                        *mut device,
                        *mut device,
                    ) -> libc::c_int,
            ),
            get_data: Some(
                file_data
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
            discard: Some(
                file_discard as unsafe extern "C" fn(*mut Stream_t) -> libc::c_int,
            ),
        };
        init
    }
};
pub unsafe extern "C" fn LockDevice(
    mut fd: libc::c_int,
    mut dev: *mut device,
    mut locked: libc::c_int,
    mut lockMode: libc::c_int,
    mut errmsg: *mut libc::c_char,
) -> libc::c_int {
    if locked != 0
        && lock_dev(
            fd,
            (lockMode & 0o3 as libc::c_int == 0o2 as libc::c_int) as libc::c_int,
            dev,
        ) != 0
    {
        if !errmsg.is_null() {
            snprintf(
                errmsg,
                199 as libc::c_int as libc::c_ulong,
                b"plain floppy: device \"%s\" busy (%s):\0" as *const u8
                    as *const libc::c_char,
                if !dev.is_null() {
                    (*dev).name
                } else {
                    b"unknown\0" as *const u8 as *const libc::c_char
                },
                strerror(*__errno_location()),
            );
        }
        if *__errno_location() != 95 as libc::c_int
            || lockMode & 0o3 as libc::c_int == 0o2 as libc::c_int
        {
            return -(1 as libc::c_int);
        }
    }
    return 0 as libc::c_int;
}
pub unsafe extern "C" fn SimpleFileOpen(
    mut dev: *mut device,
    mut orig_dev: *mut device,
    mut name: *const libc::c_char,
    mut mode: libc::c_int,
    mut errmsg: *mut libc::c_char,
    mut mode2: libc::c_int,
    mut locked: libc::c_int,
    mut maxSize: *mut mt_off_t,
) -> *mut Stream_t {
    return SimpleFileOpenWithLm(
        dev,
        orig_dev,
        name,
        mode,
        errmsg,
        mode2,
        locked,
        mode,
        maxSize,
        0 as *mut libc::c_int,
    );
}
pub unsafe extern "C" fn SimpleFileOpenWithLm(
    mut dev: *mut device,
    mut orig_dev: *mut device,
    mut name: *const libc::c_char,
    mut mode: libc::c_int,
    mut errmsg: *mut libc::c_char,
    mut mode2: libc::c_int,
    mut locked: libc::c_int,
    mut lockMode: libc::c_int,
    mut maxSize: *mut mt_off_t,
    mut geomFailure: *mut libc::c_int,
) -> *mut Stream_t {
    let mut current_block: u64;
    let mut This: *mut SimpleFile_t = 0 as *mut SimpleFile_t;
    if !dev.is_null() && (*dev).misc_flags & 0x1 as libc::c_uint != 0 {
        return 0 as *mut Stream_t;
    }
    This = calloc(
        1 as libc::c_int as libc::c_ulong,
        ::std::mem::size_of::<SimpleFile_t>() as libc::c_ulong,
    ) as *mut SimpleFile_t;
    if This.is_null() {
        printOom();
        return 0 as *mut Stream_t;
    }
    memset(
        This as *mut libc::c_void,
        0 as libc::c_int,
        ::std::mem::size_of::<SimpleFile_t>() as libc::c_ulong,
    );
    (*This).seekable = 1 as libc::c_int;
    init_head(&mut (*This).head, &mut SimpleFileClass, 0 as *mut Stream_t);
    if name.is_null()
        || strcmp(name, b"-\0" as *const u8 as *const libc::c_char) == 0 as libc::c_int
    {
        if mode == 0 as libc::c_int {
            (*This).fd = 0 as libc::c_int;
        } else {
            (*This).fd = 1 as libc::c_int;
        }
        (*This).seekable = 0 as libc::c_int;
        if fstat((*This).fd, &mut (*This).statbuf) < 0 as libc::c_int {
            free(This as *mut libc::c_char as *mut libc::c_void);
            if !errmsg.is_null() {
                snprintf(
                    errmsg,
                    199 as libc::c_int as libc::c_ulong,
                    b"Can't stat -: %s\0" as *const u8 as *const libc::c_char,
                    strerror(*__errno_location()),
                );
            }
            return 0 as *mut Stream_t;
        }
        return &mut (*This).head;
    }
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
        .fd = open(
        name,
        mode | 0 as libc::c_int | 0 as libc::c_int,
        if !dev.is_null() && (*dev).misc_flags & 0x4 as libc::c_uint != 0 {
            0o444 as libc::c_int
        } else {
            0o666 as libc::c_int
        },
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
        if fstat((*This).fd, &mut (*This).statbuf) < 0 as libc::c_int {
            if !errmsg.is_null() {
                snprintf(
                    errmsg,
                    199 as libc::c_int as libc::c_ulong,
                    b"Can't stat %s: %s\0" as *const u8 as *const libc::c_char,
                    name,
                    strerror(*__errno_location()),
                );
            }
        } else if !(LockDevice((*This).fd, dev, locked, lockMode, errmsg)
            < 0 as libc::c_int)
        {
            if !dev.is_null() {
                *__errno_location() = 0 as libc::c_int;
                if (!(!dev.is_null() && (*dev).misc_flags & 0x10 as libc::c_uint != 0)
                    && (*dev).tracks != 0 || mode2 & 4 as libc::c_int != 0)
                    && init_geom_with_reg(
                        (*This).fd,
                        dev,
                        orig_dev,
                        &mut (*This).statbuf,
                    ) != 0
                {
                    if !geomFailure.is_null()
                        && (*__errno_location() == 9 as libc::c_int
                            || *__errno_location() == 1 as libc::c_int)
                    {
                        *geomFailure = 1 as libc::c_int;
                        return 0 as *mut Stream_t;
                    } else if !errmsg.is_null() {
                        sprintf(
                            errmsg,
                            b"init: set default params\0" as *const u8
                                as *const libc::c_char,
                        );
                    }
                    current_block = 15451230448610817873;
                } else {
                    current_block = 1622411330066726685;
                }
            } else {
                current_block = 1622411330066726685;
            }
            match current_block {
                15451230448610817873 => {}
                _ => {
                    if !maxSize.is_null() {
                        *maxSize = max_off_t_seek;
                    }
                    (*This).lastwhere = 0 as libc::c_int as mt_off_t;
                    return &mut (*This).head;
                }
            }
        }
        close((*This).fd);
        postcmd((*This).postcmd);
    }
    free(This as *mut libc::c_char as *mut libc::c_void);
    return 0 as *mut Stream_t;
}
pub unsafe extern "C" fn get_fd(mut Stream: *mut Stream_t) -> libc::c_int {
    let mut clazz: *mut Class_t = 0 as *mut Class_t;
    let mut This: *mut SimpleFile_t = Stream as *mut SimpleFile_t;
    clazz = (*This).head.Class;
    if clazz != &mut SimpleFileClass as *mut Class_t {
        return -(1 as libc::c_int)
    } else {
        return (*This).fd
    };
}
