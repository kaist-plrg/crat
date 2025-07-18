use ::libc;
extern "C" {
    pub type _IO_wide_data;
    pub type _IO_codecvt;
    pub type _IO_marker;
    pub type doscp_t;
    pub type FatMap_t;
    fn random() -> libc::c_long;
    fn srandom(__seed: libc::c_uint);
    fn malloc(_: libc::c_ulong) -> *mut libc::c_void;
    fn exit(_: libc::c_int) -> !;
    static mut stderr: *mut FILE;
    fn fopen(_: *const libc::c_char, _: *const libc::c_char) -> *mut FILE;
    fn fprintf(_: *mut FILE, _: *const libc::c_char, _: ...) -> libc::c_int;
    fn printf(_: *const libc::c_char, _: ...) -> libc::c_int;
    fn fgets(
        __s: *mut libc::c_char,
        __n: libc::c_int,
        __stream: *mut FILE,
    ) -> *mut libc::c_char;
    fn perror(__s: *const libc::c_char);
    static mut optarg: *mut libc::c_char;
    static mut optind: libc::c_int;
    fn getopt(
        ___argc: libc::c_int,
        ___argv: *const *mut libc::c_char,
        __shortopts: *const libc::c_char,
    ) -> libc::c_int;
    fn time(__timer: *mut time_t) -> time_t;
    fn __errno_location() -> *mut libc::c_int;
    fn strdup(_: *const libc::c_char) -> *mut libc::c_char;
    fn strspn(_: *const libc::c_char, _: *const libc::c_char) -> libc::c_ulong;
    fn strerror(_: libc::c_int) -> *mut libc::c_char;
    fn free_stream(Stream: *mut *mut Stream_t) -> libc::c_int;
    fn force_pwrite(
        Stream: *mut Stream_t,
        buf: *mut libc::c_char,
        start: mt_off_t,
        len: size_t,
    ) -> ssize_t;
    fn force_pread(
        Stream: *mut Stream_t,
        buf: *mut libc::c_char,
        start: mt_off_t,
        len: size_t,
    ) -> ssize_t;
    fn open_root_dir(
        drivename: libc::c_char,
        flags: libc::c_int,
        isRop: *mut libc::c_int,
    ) -> *mut Stream_t;
    static mut got_signal: libc::c_int;
    fn GetFs(Fs: *mut Stream_t) -> *mut Stream_t;
    fn set_cmd_line_image(img: *mut libc::c_char);
    fn atoui(nptr: *const libc::c_char) -> libc::c_uint;
    fn strtou32(
        nptr: *const libc::c_char,
        endptr: *mut *mut libc::c_char,
        base: libc::c_int,
    ) -> uint32_t;
    static mut mversion: *const libc::c_char;
    fn printOom();
    static mut progname: *const libc::c_char;
    static mut mdate: *const libc::c_char;
    fn fatEncode(This: *mut Fs_t, pos: libc::c_uint, value: libc::c_uint);
    fn sectorsToBytes(This: *mut Fs_t, off: uint32_t) -> mt_off_t;
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
pub type mt_off_t = off_t;
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct Fs_t {
    pub head: Stream_t,
    pub serialized: libc::c_int,
    pub serial_number: libc::c_ulong,
    pub cluster_size: uint8_t,
    pub sector_size: uint16_t,
    pub fat_error: libc::c_int,
    pub fat_decode: Option::<
        unsafe extern "C" fn(*mut Fs_t, libc::c_uint) -> libc::c_uint,
    >,
    pub fat_encode: Option::<
        unsafe extern "C" fn(*mut Fs_t, libc::c_uint, libc::c_uint) -> (),
    >,
    pub fat_dirty: libc::c_int,
    pub fat_start: uint16_t,
    pub fat_len: uint32_t,
    pub num_fat: uint8_t,
    pub end_fat: uint32_t,
    pub last_fat: uint32_t,
    pub fat_bits: libc::c_uint,
    pub FatMap: *mut FatMap_t,
    pub dir_start: uint32_t,
    pub dir_len: uint16_t,
    pub clus_start: uint32_t,
    pub num_clus: uint32_t,
    pub drive: libc::c_char,
    pub primaryFat: uint32_t,
    pub writeAllFats: uint32_t,
    pub rootCluster: uint32_t,
    pub infoSectorLoc: uint32_t,
    pub backupBoot: uint16_t,
    pub last: uint32_t,
    pub freeSpace: uint32_t,
    pub preallocatedClusters: libc::c_uint,
    pub lastFatSectorNr: uint32_t,
    pub lastFatSectorData: *mut libc::c_uchar,
    pub lastFatAccessMode: fatAccessMode_t,
    pub sectorMask: libc::c_uint,
    pub sectorShift: libc::c_uint,
    pub cp: *mut doscp_t,
}
pub type fatAccessMode_t = libc::c_uint;
pub const FAT_ACCESS_WRITE: fatAccessMode_t = 1;
pub const FAT_ACCESS_READ: fatAccessMode_t = 0;
#[inline]
unsafe extern "C" fn init_random() {
    srandom(time(0 as *mut time_t) as libc::c_uint);
}
unsafe extern "C" fn usage(mut ret: libc::c_int) -> ! {
    fprintf(
        stderr,
        b"Mtools version %s, dated %s\n\0" as *const u8 as *const libc::c_char,
        mversion,
        mdate,
    );
    fprintf(
        stderr,
        b"Usage: %s: [-c clusterList] [-s sectorList] [-c] [-V] device\n\0" as *const u8
            as *const libc::c_char,
        progname,
    );
    exit(ret);
}
unsafe extern "C" fn checkListTwice(mut filename: *mut libc::c_char) {
    if !filename.is_null() {
        fprintf(
            stderr,
            b"Only one of the -c or -s options may be given\n\0" as *const u8
                as *const libc::c_char,
        );
        exit(1 as libc::c_int);
    }
}
unsafe extern "C" fn mark(
    mut Fs: *mut Fs_t,
    mut offset: uint32_t,
    mut badClus: uint32_t,
) {
    let mut old: uint32_t = ((*Fs).fat_decode).unwrap()(Fs, offset);
    if old == 0 as libc::c_int as libc::c_uint {
        fatEncode(Fs, offset, badClus);
        return;
    }
    if old == badClus {
        fprintf(
            stderr,
            b"Cluster %d already marked\n\0" as *const u8 as *const libc::c_char,
            offset,
        );
    } else {
        fprintf(
            stderr,
            b"Cluster %d is busy\n\0" as *const u8 as *const libc::c_char,
            offset,
        );
    };
}
static mut in_buf: *mut libc::c_char = 0 as *const libc::c_char as *mut libc::c_char;
static mut pat_buf: *mut libc::c_char = 0 as *const libc::c_char as *mut libc::c_char;
static mut in_len: size_t = 0;
unsafe extern "C" fn progress(mut i: libc::c_uint, mut total: libc::c_uint) {
    if i.wrapping_rem(10 as libc::c_int as libc::c_uint)
        == 0 as libc::c_int as libc::c_uint
    {
        fprintf(
            stderr,
            b"                     \r%d/%d\r\0" as *const u8 as *const libc::c_char,
            i,
            total,
        );
    }
}
unsafe extern "C" fn scan(
    mut Fs: *mut Fs_t,
    mut dev: *mut Stream_t,
    mut cluster: uint32_t,
    mut badClus: uint32_t,
    mut buffer: *mut libc::c_char,
    mut doWrite: libc::c_int,
) -> libc::c_int {
    let mut start: uint32_t = 0;
    let mut ret: off_t = 0;
    let mut pos: mt_off_t = 0;
    let mut bad: libc::c_int = 0 as libc::c_int;
    if ((*Fs).fat_decode).unwrap()(Fs, cluster) != 0 {
        return 0 as libc::c_int;
    }
    start = cluster
        .wrapping_sub(2 as libc::c_int as libc::c_uint)
        .wrapping_mul((*Fs).cluster_size as libc::c_uint)
        .wrapping_add((*Fs).clus_start);
    pos = sectorsToBytes(Fs, start);
    if doWrite != 0 {
        ret = force_pwrite(dev, buffer, pos, in_len);
        if ret < 0 as libc::c_int as libc::c_long || (ret as size_t) < in_len {
            bad = 1 as libc::c_int;
        }
    } else {
        ret = force_pread(dev, in_buf, pos, in_len);
        if ret < in_len as off_t {
            bad = 1 as libc::c_int;
        } else if !buffer.is_null() {
            let mut i: size_t = 0;
            i = 0 as libc::c_int as size_t;
            while i < in_len {
                if *in_buf.offset(i as isize) as libc::c_int
                    != *buffer.offset(i as isize) as libc::c_int
                {
                    bad = 1 as libc::c_int;
                    break;
                } else {
                    i = i.wrapping_add(1);
                    i;
                }
            }
        }
    }
    if bad != 0 {
        printf(b"Bad cluster %d found\n\0" as *const u8 as *const libc::c_char, cluster);
        fatEncode(Fs, cluster, badClus);
        return 1 as libc::c_int;
    }
    return 0 as libc::c_int;
}
pub unsafe extern "C" fn mbadblocks(
    mut argc: libc::c_int,
    mut argv: *mut *mut libc::c_char,
    mut type_0: libc::c_int,
) {
    let mut current_block: u64;
    let mut i: libc::c_uint = 0;
    let mut startSector: libc::c_uint = 2 as libc::c_int as libc::c_uint;
    let mut endSector: libc::c_uint = 0 as libc::c_int as libc::c_uint;
    let mut Fs: *mut Fs_t = 0 as *mut Fs_t;
    let mut Dir: *mut Stream_t = 0 as *mut Stream_t;
    let mut ret: libc::c_int = 0;
    let mut filename: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut c: libc::c_int = 0;
    let mut badClus: libc::c_uint = 0;
    let mut sectorMode: libc::c_int = 0 as libc::c_int;
    let mut writeMode: libc::c_int = 0 as libc::c_int;
    loop {
        c = getopt(argc, argv, b"i:s:cwS:E:\0" as *const u8 as *const libc::c_char);
        if !(c != -(1 as libc::c_int)) {
            break;
        }
        match c {
            105 => {
                set_cmd_line_image(optarg);
            }
            99 => {
                checkListTwice(filename);
                filename = strdup(optarg);
            }
            115 => {
                checkListTwice(filename);
                filename = strdup(optarg);
                sectorMode = 1 as libc::c_int;
            }
            83 => {
                startSector = atoui(optarg);
            }
            69 => {
                endSector = atoui(optarg);
            }
            119 => {
                writeMode = 1 as libc::c_int;
            }
            104 => {
                usage(0 as libc::c_int);
            }
            _ => {
                usage(1 as libc::c_int);
            }
        }
    }
    if argc != optind + 1 as libc::c_int
        || *(*argv.offset(optind as isize)).offset(0 as libc::c_int as isize) == 0
        || *(*argv.offset(optind as isize)).offset(1 as libc::c_int as isize)
            as libc::c_int != ':' as i32
        || *(*argv.offset(optind as isize)).offset(2 as libc::c_int as isize)
            as libc::c_int != 0
    {
        usage(1 as libc::c_int);
    }
    Dir = open_root_dir(
        *(*argv.offset(optind as isize)).offset(0 as libc::c_int as isize),
        0o2 as libc::c_int,
        0 as *mut libc::c_int,
    );
    if Dir.is_null() {
        fprintf(
            stderr,
            b"%s: Cannot initialize drive\n\0" as *const u8 as *const libc::c_char,
            *argv.offset(0 as libc::c_int as isize),
        );
        exit(1 as libc::c_int);
    }
    Fs = GetFs(Dir) as *mut Fs_t;
    in_len = ((*Fs).cluster_size as libc::c_int * (*Fs).sector_size as libc::c_int)
        as size_t;
    in_buf = malloc(in_len) as *mut libc::c_char;
    if in_buf.is_null() {
        printOom();
        ret = 1 as libc::c_int;
    } else {
        if writeMode != 0 {
            pat_buf = malloc(in_len.wrapping_mul(311 as libc::c_int as libc::c_ulong))
                as *mut libc::c_char;
            if pat_buf.is_null() {
                printOom();
                ret = 1 as libc::c_int;
                current_block = 9620083545441291948;
            } else {
                init_random();
                i = 0 as libc::c_int as libc::c_uint;
                while (i as libc::c_ulong)
                    < in_len.wrapping_mul(311 as libc::c_int as libc::c_ulong)
                {
                    *pat_buf.offset(i as isize) = random() as libc::c_char;
                    i = i.wrapping_add(1);
                    i;
                }
                current_block = 14832935472441733737;
            }
        } else {
            current_block = 14832935472441733737;
        }
        match current_block {
            9620083545441291948 => {}
            _ => {
                i = 0 as libc::c_int as libc::c_uint;
                loop {
                    if !(i < (*Fs).clus_start) {
                        current_block = 1434579379687443766;
                        break;
                    }
                    let mut r: ssize_t = 0;
                    r = ((*(*(*Fs).head.Next).Class).pread)
                        .unwrap()(
                        (*Fs).head.Next,
                        in_buf,
                        sectorsToBytes(Fs, i),
                        (*Fs).sector_size as size_t,
                    );
                    if r < 0 as libc::c_int as libc::c_long {
                        perror(b"early error\0" as *const u8 as *const libc::c_char);
                        ret = -(1 as libc::c_int);
                        current_block = 9620083545441291948;
                        break;
                    } else if (r as size_t) < (*Fs).sector_size as libc::c_ulong {
                        fprintf(
                            stderr,
                            b"end of file in file_read\n\0" as *const u8
                                as *const libc::c_char,
                        );
                        ret = 1 as libc::c_int;
                        current_block = 9620083545441291948;
                        break;
                    } else {
                        i = i.wrapping_add(1);
                        i;
                    }
                }
                match current_block {
                    9620083545441291948 => {}
                    _ => {
                        ret = 0 as libc::c_int;
                        badClus = ((*Fs).last_fat)
                            .wrapping_add(1 as libc::c_int as libc::c_uint);
                        if startSector < 2 as libc::c_int as libc::c_uint {
                            startSector = 2 as libc::c_int as libc::c_uint;
                        }
                        if endSector
                            > ((*Fs).num_clus)
                                .wrapping_add(2 as libc::c_int as libc::c_uint)
                            || endSector <= 0 as libc::c_int as libc::c_uint
                        {
                            endSector = ((*Fs).num_clus)
                                .wrapping_add(2 as libc::c_int as libc::c_uint);
                        }
                        if !filename.is_null() {
                            let mut line: [libc::c_char; 80] = [0; 80];
                            let mut f: *mut FILE = fopen(
                                filename,
                                b"r\0" as *const u8 as *const libc::c_char,
                            );
                            if f.is_null() {
                                fprintf(
                                    stderr,
                                    b"Could not open %s (%s)\n\0" as *const u8
                                        as *const libc::c_char,
                                    filename,
                                    strerror(*__errno_location()),
                                );
                                ret = 1 as libc::c_int;
                            } else {
                                while !(fgets(
                                    line.as_mut_ptr(),
                                    ::std::mem::size_of::<[libc::c_char; 80]>() as libc::c_ulong
                                        as libc::c_int,
                                    f,
                                ))
                                    .is_null()
                                {
                                    let mut ptr: *mut libc::c_char = line
                                        .as_mut_ptr()
                                        .offset(
                                            strspn(
                                                line.as_mut_ptr(),
                                                b" \t\0" as *const u8 as *const libc::c_char,
                                            ) as isize,
                                        );
                                    let mut offset: uint32_t = strtou32(
                                        ptr,
                                        0 as *mut *mut libc::c_char,
                                        0 as libc::c_int,
                                    );
                                    if sectorMode != 0 {
                                        offset = offset
                                            .wrapping_sub((*Fs).clus_start)
                                            .wrapping_div((*Fs).cluster_size as libc::c_uint)
                                            .wrapping_add(2 as libc::c_int as libc::c_uint);
                                    }
                                    if offset < 2 as libc::c_int as libc::c_uint {
                                        fprintf(
                                            stderr,
                                            b"Sector before start\n\0" as *const u8
                                                as *const libc::c_char,
                                        );
                                    } else if offset >= (*Fs).num_clus {
                                        fprintf(
                                            stderr,
                                            b"Sector beyond end\n\0" as *const u8 as *const libc::c_char,
                                        );
                                    } else {
                                        mark(Fs, offset, badClus);
                                        ret = 1 as libc::c_int;
                                    }
                                }
                            }
                        } else {
                            let mut dev: *mut Stream_t = 0 as *mut Stream_t;
                            dev = (*Fs).head.Next;
                            if !((*dev).Next).is_null() {
                                dev = (*dev).Next;
                            }
                            in_len = ((*Fs).cluster_size as libc::c_int
                                * (*Fs).sector_size as libc::c_int) as size_t;
                            if writeMode != 0 {
                                i = startSector;
                                while i < endSector {
                                    if got_signal != 0 {
                                        break;
                                    }
                                    progress(i, (*Fs).num_clus);
                                    ret
                                        |= scan(
                                            Fs,
                                            dev,
                                            i,
                                            badClus,
                                            pat_buf
                                                .offset(
                                                    in_len
                                                        .wrapping_mul(
                                                            i.wrapping_rem(311 as libc::c_int as libc::c_uint)
                                                                as libc::c_ulong,
                                                        ) as isize,
                                                ),
                                            1 as libc::c_int,
                                        );
                                    i = i.wrapping_add(1);
                                    i;
                                }
                                if got_signal == 0 {
                                    ((*(*dev).Class).discard).unwrap()(dev);
                                }
                                i = startSector;
                                while i < endSector {
                                    if got_signal != 0 {
                                        break;
                                    }
                                    progress(i, (*Fs).num_clus);
                                    ret
                                        |= scan(
                                            Fs,
                                            dev,
                                            i,
                                            badClus,
                                            pat_buf
                                                .offset(
                                                    in_len
                                                        .wrapping_mul(
                                                            i.wrapping_rem(311 as libc::c_int as libc::c_uint)
                                                                as libc::c_ulong,
                                                        ) as isize,
                                                ),
                                            0 as libc::c_int,
                                        );
                                    i = i.wrapping_add(1);
                                    i;
                                }
                            } else {
                                i = startSector;
                                while i < endSector {
                                    if got_signal != 0 {
                                        break;
                                    }
                                    progress(i, (*Fs).num_clus);
                                    ret
                                        |= scan(
                                            Fs,
                                            dev,
                                            i,
                                            badClus,
                                            0 as *mut libc::c_char,
                                            0 as libc::c_int,
                                        );
                                    i = i.wrapping_add(1);
                                    i;
                                }
                            }
                        }
                    }
                }
            }
        }
    }
    free_stream(&mut Dir);
    exit(ret);
}
