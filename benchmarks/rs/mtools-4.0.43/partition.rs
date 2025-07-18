use ::libc;
extern "C" {
    pub type _IO_wide_data;
    pub type _IO_codecvt;
    pub type _IO_marker;
    pub type doscp_t;
    fn calloc(_: libc::c_ulong, _: libc::c_ulong) -> *mut libc::c_void;
    fn free(__ptr: *mut libc::c_void);
    static mut stdout: *mut FILE;
    static mut stderr: *mut FILE;
    fn fprintf(_: *mut FILE, _: *const libc::c_char, _: ...) -> libc::c_int;
    fn printf(_: *const libc::c_char, _: ...) -> libc::c_int;
    fn sprintf(_: *mut libc::c_char, _: *const libc::c_char, _: ...) -> libc::c_int;
    fn putc(__c: libc::c_int, __stream: *mut FILE) -> libc::c_int;
    fn memset(
        _: *mut libc::c_void,
        _: libc::c_int,
        _: libc::c_ulong,
    ) -> *mut libc::c_void;
    static mut mtools_skip_check: libc::c_uint;
    fn printOom();
    fn get_dosConvert_pass_through(Stream: *mut Stream_t) -> *mut doscp_t;
    fn limitSizeToOffT(len: *mut size_t, maxLen: mt_off_t);
    fn init_head(Stream: *mut Stream_t, Class: *mut Class_t, Next: *mut Stream_t);
    fn force_pread(
        Stream: *mut Stream_t,
        buf: *mut libc::c_char,
        start: mt_off_t,
        len: size_t,
    ) -> ssize_t;
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
pub type smt_off_t = mt_off_t;
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
pub struct hsc {
    pub byte0: libc::c_uchar,
    pub head: libc::c_uchar,
    pub sector: libc::c_uchar,
    pub cyl: libc::c_uchar,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct partition {
    pub start: hsc,
    pub end: hsc,
    pub start_sect: [libc::c_uchar; 4],
    pub nr_sects: [libc::c_uchar; 4],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct Partition_t {
    pub head: Stream_t,
    pub offset: mt_off_t,
    pub size: mt_off_t,
    pub nbSect: uint32_t,
    pub pos: uint8_t,
    pub sectors: uint8_t,
    pub heads: uint8_t,
    pub cyclinders: uint16_t,
}
#[inline]
unsafe extern "C" fn putchar(mut __c: libc::c_int) -> libc::c_int {
    return putc(__c, stdout);
}
#[inline]
unsafe extern "C" fn print_hsc(mut h: *mut hsc) {
    printf(
        b" h=%d s=%d c=%d\n\0" as *const u8 as *const libc::c_char,
        (*h).head as libc::c_int,
        ((*h).sector as libc::c_int & 0x3f as libc::c_int) as uint8_t as libc::c_int,
        ((*h).cyl as libc::c_int
            | ((*h).sector as libc::c_int & 0xc0 as libc::c_int) << 2 as libc::c_int)
            as uint16_t as libc::c_int,
    );
}
unsafe extern "C" fn overlapCheck(
    mut partTable: *mut partition,
    mut i: libc::c_uint,
    mut start: uint32_t,
    mut end: uint32_t,
) -> libc::c_int {
    let mut partition: *mut partition = &mut *partTable.offset(i as isize)
        as *mut partition;
    if (*partition).end.byte0 == 0 {
        return 0 as libc::c_int;
    }
    if end
        > (((*partition).start_sect[0 as libc::c_int as usize] as libc::c_int
            + (((*partition).start_sect[1 as libc::c_int as usize] as libc::c_int)
                << 8 as libc::c_int)) as uint16_t as libc::c_int
            + (((*((*partition).start_sect)
                .as_mut_ptr()
                .offset(2 as libc::c_int as isize)
                .offset(0 as libc::c_int as isize) as libc::c_int
                + ((*((*partition).start_sect)
                    .as_mut_ptr()
                    .offset(2 as libc::c_int as isize)
                    .offset(1 as libc::c_int as isize) as libc::c_int)
                    << 8 as libc::c_int)) as uint16_t as libc::c_int)
                << 16 as libc::c_int)) as uint32_t
        && (start
            < ((((*partition).start_sect[0 as libc::c_int as usize] as libc::c_int
                + (((*partition).start_sect[1 as libc::c_int as usize] as libc::c_int)
                    << 8 as libc::c_int)) as uint16_t as libc::c_int
                + (((*((*partition).start_sect)
                    .as_mut_ptr()
                    .offset(2 as libc::c_int as isize)
                    .offset(0 as libc::c_int as isize) as libc::c_int
                    + ((*((*partition).start_sect)
                        .as_mut_ptr()
                        .offset(2 as libc::c_int as isize)
                        .offset(1 as libc::c_int as isize) as libc::c_int)
                        << 8 as libc::c_int)) as uint16_t as libc::c_int)
                    << 16 as libc::c_int)) as uint32_t)
                .wrapping_add(
                    (((*partition).nr_sects[0 as libc::c_int as usize] as libc::c_int
                        + (((*partition).nr_sects[1 as libc::c_int as usize]
                            as libc::c_int) << 8 as libc::c_int)) as uint16_t
                        as libc::c_int
                        + (((*((*partition).nr_sects)
                            .as_mut_ptr()
                            .offset(2 as libc::c_int as isize)
                            .offset(0 as libc::c_int as isize) as libc::c_int
                            + ((*((*partition).nr_sects)
                                .as_mut_ptr()
                                .offset(2 as libc::c_int as isize)
                                .offset(1 as libc::c_int as isize) as libc::c_int)
                                << 8 as libc::c_int)) as uint16_t as libc::c_int)
                            << 16 as libc::c_int)) as uint32_t,
                )
            || ((((*partition).start_sect[0 as libc::c_int as usize] as libc::c_int
                + (((*partition).start_sect[1 as libc::c_int as usize] as libc::c_int)
                    << 8 as libc::c_int)) as uint16_t as libc::c_int
                + (((*((*partition).start_sect)
                    .as_mut_ptr()
                    .offset(2 as libc::c_int as isize)
                    .offset(0 as libc::c_int as isize) as libc::c_int
                    + ((*((*partition).start_sect)
                        .as_mut_ptr()
                        .offset(2 as libc::c_int as isize)
                        .offset(1 as libc::c_int as isize) as libc::c_int)
                        << 8 as libc::c_int)) as uint16_t as libc::c_int)
                    << 16 as libc::c_int)) as uint32_t)
                .wrapping_add(
                    (((*partition).nr_sects[0 as libc::c_int as usize] as libc::c_int
                        + (((*partition).nr_sects[1 as libc::c_int as usize]
                            as libc::c_int) << 8 as libc::c_int)) as uint16_t
                        as libc::c_int
                        + (((*((*partition).nr_sects)
                            .as_mut_ptr()
                            .offset(2 as libc::c_int as isize)
                            .offset(0 as libc::c_int as isize) as libc::c_int
                            + ((*((*partition).nr_sects)
                                .as_mut_ptr()
                                .offset(2 as libc::c_int as isize)
                                .offset(1 as libc::c_int as isize) as libc::c_int)
                                << 8 as libc::c_int)) as uint16_t as libc::c_int)
                            << 16 as libc::c_int)) as uint32_t,
                )
                < (((*partition).start_sect[0 as libc::c_int as usize] as libc::c_int
                    + (((*partition).start_sect[1 as libc::c_int as usize]
                        as libc::c_int) << 8 as libc::c_int)) as uint16_t as libc::c_int
                    + (((*((*partition).start_sect)
                        .as_mut_ptr()
                        .offset(2 as libc::c_int as isize)
                        .offset(0 as libc::c_int as isize) as libc::c_int
                        + ((*((*partition).start_sect)
                            .as_mut_ptr()
                            .offset(2 as libc::c_int as isize)
                            .offset(1 as libc::c_int as isize) as libc::c_int)
                            << 8 as libc::c_int)) as uint16_t as libc::c_int)
                        << 16 as libc::c_int)) as uint32_t)
    {
        return -(1 as libc::c_int);
    }
    return 0 as libc::c_int;
}
pub unsafe extern "C" fn findOverlap(
    mut partTable: *mut partition,
    mut until: libc::c_uint,
    mut start: uint32_t,
    mut end: uint32_t,
) -> libc::c_uint {
    let mut i: libc::c_uint = 0;
    i = 1 as libc::c_int as libc::c_uint;
    while i <= until {
        if overlapCheck(partTable, i, start, end) != 0 {
            return i;
        }
        i = i.wrapping_add(1);
        i;
    }
    return 0 as libc::c_int as libc::c_uint;
}
pub unsafe extern "C" fn consistencyCheck(
    mut partTable: *mut partition,
    mut doprint: libc::c_int,
    mut verbose: libc::c_int,
    mut has_activated: *mut libc::c_int,
    mut tot_sectors: uint32_t,
    mut used_dev: *mut device,
    mut target_partition: libc::c_uint,
) -> libc::c_int {
    let mut i: libc::c_uint = 0;
    let mut inconsistency: bool = false;
    inconsistency = 0 as libc::c_int != 0;
    *has_activated = 0 as libc::c_int;
    i = 1 as libc::c_int as libc::c_uint;
    while i <= 4 as libc::c_int as libc::c_uint {
        let mut j: libc::c_uint = 0;
        let mut partition: *mut partition = &mut *partTable.offset(i as isize)
            as *mut partition;
        if !((*partition).end.byte0 == 0) {
            if (*partition).start.byte0 != 0 {
                *has_activated += 1;
                *has_activated;
            }
            if ((((*partition).start_sect[0 as libc::c_int as usize] as libc::c_int
                + (((*partition).start_sect[1 as libc::c_int as usize] as libc::c_int)
                    << 8 as libc::c_int)) as uint16_t as libc::c_int
                + (((*((*partition).start_sect)
                    .as_mut_ptr()
                    .offset(2 as libc::c_int as isize)
                    .offset(0 as libc::c_int as isize) as libc::c_int
                    + ((*((*partition).start_sect)
                        .as_mut_ptr()
                        .offset(2 as libc::c_int as isize)
                        .offset(1 as libc::c_int as isize) as libc::c_int)
                        << 8 as libc::c_int)) as uint16_t as libc::c_int)
                    << 16 as libc::c_int)) as uint32_t)
                .wrapping_add(
                    (((*partition).nr_sects[0 as libc::c_int as usize] as libc::c_int
                        + (((*partition).nr_sects[1 as libc::c_int as usize]
                            as libc::c_int) << 8 as libc::c_int)) as uint16_t
                        as libc::c_int
                        + (((*((*partition).nr_sects)
                            .as_mut_ptr()
                            .offset(2 as libc::c_int as isize)
                            .offset(0 as libc::c_int as isize) as libc::c_int
                            + ((*((*partition).nr_sects)
                                .as_mut_ptr()
                                .offset(2 as libc::c_int as isize)
                                .offset(1 as libc::c_int as isize) as libc::c_int)
                                << 8 as libc::c_int)) as uint16_t as libc::c_int)
                            << 16 as libc::c_int)) as uint32_t,
                )
                < (((*partition).start_sect[0 as libc::c_int as usize] as libc::c_int
                    + (((*partition).start_sect[1 as libc::c_int as usize]
                        as libc::c_int) << 8 as libc::c_int)) as uint16_t as libc::c_int
                    + (((*((*partition).start_sect)
                        .as_mut_ptr()
                        .offset(2 as libc::c_int as isize)
                        .offset(0 as libc::c_int as isize) as libc::c_int
                        + ((*((*partition).start_sect)
                            .as_mut_ptr()
                            .offset(2 as libc::c_int as isize)
                            .offset(1 as libc::c_int as isize) as libc::c_int)
                            << 8 as libc::c_int)) as uint16_t as libc::c_int)
                        << 16 as libc::c_int)) as uint32_t
            {
                fprintf(
                    stderr,
                    b"End of partition %d before its begin\n\0" as *const u8
                        as *const libc::c_char,
                    i,
                );
            }
            j = findOverlap(
                partTable,
                i.wrapping_sub(1 as libc::c_int as libc::c_uint),
                (((*partition).start_sect[0 as libc::c_int as usize] as libc::c_int
                    + (((*partition).start_sect[1 as libc::c_int as usize]
                        as libc::c_int) << 8 as libc::c_int)) as uint16_t as libc::c_int
                    + (((*((*partition).start_sect)
                        .as_mut_ptr()
                        .offset(2 as libc::c_int as isize)
                        .offset(0 as libc::c_int as isize) as libc::c_int
                        + ((*((*partition).start_sect)
                            .as_mut_ptr()
                            .offset(2 as libc::c_int as isize)
                            .offset(1 as libc::c_int as isize) as libc::c_int)
                            << 8 as libc::c_int)) as uint16_t as libc::c_int)
                        << 16 as libc::c_int)) as uint32_t,
                ((((*partition).start_sect[0 as libc::c_int as usize] as libc::c_int
                    + (((*partition).start_sect[1 as libc::c_int as usize]
                        as libc::c_int) << 8 as libc::c_int)) as uint16_t as libc::c_int
                    + (((*((*partition).start_sect)
                        .as_mut_ptr()
                        .offset(2 as libc::c_int as isize)
                        .offset(0 as libc::c_int as isize) as libc::c_int
                        + ((*((*partition).start_sect)
                            .as_mut_ptr()
                            .offset(2 as libc::c_int as isize)
                            .offset(1 as libc::c_int as isize) as libc::c_int)
                            << 8 as libc::c_int)) as uint16_t as libc::c_int)
                        << 16 as libc::c_int)) as uint32_t)
                    .wrapping_add(
                        (((*partition).nr_sects[0 as libc::c_int as usize] as libc::c_int
                            + (((*partition).nr_sects[1 as libc::c_int as usize]
                                as libc::c_int) << 8 as libc::c_int)) as uint16_t
                            as libc::c_int
                            + (((*((*partition).nr_sects)
                                .as_mut_ptr()
                                .offset(2 as libc::c_int as isize)
                                .offset(0 as libc::c_int as isize) as libc::c_int
                                + ((*((*partition).nr_sects)
                                    .as_mut_ptr()
                                    .offset(2 as libc::c_int as isize)
                                    .offset(1 as libc::c_int as isize) as libc::c_int)
                                    << 8 as libc::c_int)) as uint16_t as libc::c_int)
                                << 16 as libc::c_int)) as uint32_t,
                    ),
            );
            if j != 0 {
                fprintf(
                    stderr,
                    b"Partitions %d and %d overlap\n\0" as *const u8
                        as *const libc::c_char,
                    j,
                    i,
                );
                inconsistency = 1 as libc::c_int != 0;
            }
            if tot_sectors != 0
                && ((((*partition).start_sect[0 as libc::c_int as usize] as libc::c_int
                    + (((*partition).start_sect[1 as libc::c_int as usize]
                        as libc::c_int) << 8 as libc::c_int)) as uint16_t as libc::c_int
                    + (((*((*partition).start_sect)
                        .as_mut_ptr()
                        .offset(2 as libc::c_int as isize)
                        .offset(0 as libc::c_int as isize) as libc::c_int
                        + ((*((*partition).start_sect)
                            .as_mut_ptr()
                            .offset(2 as libc::c_int as isize)
                            .offset(1 as libc::c_int as isize) as libc::c_int)
                            << 8 as libc::c_int)) as uint16_t as libc::c_int)
                        << 16 as libc::c_int)) as uint32_t)
                    .wrapping_add(
                        (((*partition).nr_sects[0 as libc::c_int as usize] as libc::c_int
                            + (((*partition).nr_sects[1 as libc::c_int as usize]
                                as libc::c_int) << 8 as libc::c_int)) as uint16_t
                            as libc::c_int
                            + (((*((*partition).nr_sects)
                                .as_mut_ptr()
                                .offset(2 as libc::c_int as isize)
                                .offset(0 as libc::c_int as isize) as libc::c_int
                                + ((*((*partition).nr_sects)
                                    .as_mut_ptr()
                                    .offset(2 as libc::c_int as isize)
                                    .offset(1 as libc::c_int as isize) as libc::c_int)
                                    << 8 as libc::c_int)) as uint16_t as libc::c_int)
                                << 16 as libc::c_int)) as uint32_t,
                    ) > tot_sectors
            {
                fprintf(
                    stderr,
                    b"Partition %d extends beyond end of disk\n\0" as *const u8
                        as *const libc::c_char,
                    i,
                );
            }
            if doprint != 0 && verbose != 0 {
                if i == target_partition {
                    putchar('*' as i32);
                } else {
                    putchar(' ' as i32);
                }
                printf(b"Partition %d\n\0" as *const u8 as *const libc::c_char, i);
                printf(
                    b"  active=%x\n\0" as *const u8 as *const libc::c_char,
                    (*partition).start.byte0 as libc::c_int,
                );
                printf(b"  start:\0" as *const u8 as *const libc::c_char);
                print_hsc(&mut (*partition).start);
                printf(
                    b"  type=0x%x\n\0" as *const u8 as *const libc::c_char,
                    (*partition).end.byte0 as libc::c_int,
                );
                printf(b"  end:\0" as *const u8 as *const libc::c_char);
                print_hsc(&mut (*partition).end);
                printf(
                    b"  start=%d\n\0" as *const u8 as *const libc::c_char,
                    (((*partition).start_sect[0 as libc::c_int as usize] as libc::c_int
                        + (((*partition).start_sect[1 as libc::c_int as usize]
                            as libc::c_int) << 8 as libc::c_int)) as uint16_t
                        as libc::c_int
                        + (((*((*partition).start_sect)
                            .as_mut_ptr()
                            .offset(2 as libc::c_int as isize)
                            .offset(0 as libc::c_int as isize) as libc::c_int
                            + ((*((*partition).start_sect)
                                .as_mut_ptr()
                                .offset(2 as libc::c_int as isize)
                                .offset(1 as libc::c_int as isize) as libc::c_int)
                                << 8 as libc::c_int)) as uint16_t as libc::c_int)
                            << 16 as libc::c_int)) as uint32_t,
                );
                printf(
                    b"  nr=%d\n\0" as *const u8 as *const libc::c_char,
                    (((*partition).nr_sects[0 as libc::c_int as usize] as libc::c_int
                        + (((*partition).nr_sects[1 as libc::c_int as usize]
                            as libc::c_int) << 8 as libc::c_int)) as uint16_t
                        as libc::c_int
                        + (((*((*partition).nr_sects)
                            .as_mut_ptr()
                            .offset(2 as libc::c_int as isize)
                            .offset(0 as libc::c_int as isize) as libc::c_int
                            + ((*((*partition).nr_sects)
                                .as_mut_ptr()
                                .offset(2 as libc::c_int as isize)
                                .offset(1 as libc::c_int as isize) as libc::c_int)
                                << 8 as libc::c_int)) as uint16_t as libc::c_int)
                            << 16 as libc::c_int)) as uint32_t,
                );
                printf(b"\n\0" as *const u8 as *const libc::c_char);
            }
        }
        i = i.wrapping_add(1);
        i;
    }
    return inconsistency as libc::c_int;
}
unsafe extern "C" fn limit_size(
    mut This: *mut Partition_t,
    mut start: mt_off_t,
    mut len: *mut size_t,
) -> libc::c_int {
    if start > (*This).size {
        return -(1 as libc::c_int);
    }
    limitSizeToOffT(len, (*This).size - start);
    return 0 as libc::c_int;
}
unsafe extern "C" fn partition_pread(
    mut Stream: *mut Stream_t,
    mut buf: *mut libc::c_char,
    mut start: mt_off_t,
    mut len: size_t,
) -> ssize_t {
    let mut This: *mut Partition_t = Stream as *mut Partition_t;
    if limit_size(This, start, &mut len) < 0 as libc::c_int {
        return -(1 as libc::c_int) as ssize_t;
    }
    return ((*(*(*This).head.Next).Class).pread)
        .unwrap()((*This).head.Next, buf, start + (*This).offset, len);
}
unsafe extern "C" fn partition_pwrite(
    mut Stream: *mut Stream_t,
    mut buf: *mut libc::c_char,
    mut start: mt_off_t,
    mut len: size_t,
) -> ssize_t {
    let mut This: *mut Partition_t = Stream as *mut Partition_t;
    if limit_size(This, start, &mut len) < 0 as libc::c_int {
        return -(1 as libc::c_int) as ssize_t;
    }
    return ((*(*(*This).head.Next).Class).pwrite)
        .unwrap()((*This).head.Next, buf, start + (*This).offset, len);
}
unsafe extern "C" fn partition_data(
    mut Stream: *mut Stream_t,
    mut date: *mut time_t,
    mut size: *mut mt_off_t,
    mut type_0: *mut libc::c_int,
    mut address: *mut uint32_t,
) -> libc::c_int {
    let mut This: *mut Partition_t = Stream as *mut Partition_t;
    if !date.is_null() || !type_0.is_null() || !address.is_null() {
        let mut ret: libc::c_int = ((*(*(*This).head.Next).Class).get_data)
            .unwrap()((*This).head.Next, date, 0 as *mut mt_off_t, type_0, address);
        if ret < 0 as libc::c_int {
            return ret;
        }
    }
    if !size.is_null() {
        *size = (*This).size * 512 as libc::c_int as libc::c_long;
    }
    return 0 as libc::c_int;
}
unsafe extern "C" fn partition_geom(
    mut Stream: *mut Stream_t,
    mut dev: *mut device,
    mut orig_dev: *mut device,
) -> libc::c_int {
    let mut This: *mut Partition_t = Stream as *mut Partition_t;
    if (*dev).tot_sectors == 0 {
        (*dev).tot_sectors = (*This).nbSect;
    }
    return 0 as libc::c_int;
}
static mut PartitionClass: Class_t = unsafe {
    {
        let mut init = Class_t {
            read: None,
            write: None,
            pread: Some(
                partition_pread
                    as unsafe extern "C" fn(
                        *mut Stream_t,
                        *mut libc::c_char,
                        mt_off_t,
                        size_t,
                    ) -> ssize_t,
            ),
            pwrite: Some(
                partition_pwrite
                    as unsafe extern "C" fn(
                        *mut Stream_t,
                        *mut libc::c_char,
                        mt_off_t,
                        size_t,
                    ) -> ssize_t,
            ),
            flush: None,
            freeFunc: None,
            set_geom: Some(
                partition_geom
                    as unsafe extern "C" fn(
                        *mut Stream_t,
                        *mut device,
                        *mut device,
                    ) -> libc::c_int,
            ),
            get_data: Some(
                partition_data
                    as unsafe extern "C" fn(
                        *mut Stream_t,
                        *mut time_t,
                        *mut mt_off_t,
                        *mut libc::c_int,
                        *mut uint32_t,
                    ) -> libc::c_int,
            ),
            pre_allocate: None,
            get_dosConvert: Some(
                get_dosConvert_pass_through
                    as unsafe extern "C" fn(*mut Stream_t) -> *mut doscp_t,
            ),
            discard: None,
        };
        init
    }
};
pub unsafe extern "C" fn OpenPartition(
    mut Next: *mut Stream_t,
    mut dev: *mut device,
    mut errmsg: *mut libc::c_char,
    mut maxSize: *mut mt_off_t,
) -> *mut Stream_t {
    let mut current_block: u64;
    let mut This: *mut Partition_t = 0 as *mut Partition_t;
    let mut has_activated: libc::c_int = 0;
    let mut buf: [libc::c_uchar; 2048] = [0; 2048];
    let mut partTable: *mut partition = buf
        .as_mut_ptr()
        .offset(0x1ae as libc::c_int as isize) as *mut partition;
    let mut partOff: uint32_t = 0;
    let mut partition: *mut partition = 0 as *mut partition;
    if dev.is_null() || (*dev).partition > 4 as libc::c_int as libc::c_uint
        || (*dev).partition <= 0 as libc::c_int as libc::c_uint
    {
        fprintf(
            stderr,
            b"Invalid partition %d (must be between 1 and 4), ignoring it\n\0"
                as *const u8 as *const libc::c_char,
            (*dev).partition,
        );
        return 0 as *mut Stream_t;
    }
    This = calloc(
        1 as libc::c_int as libc::c_ulong,
        ::std::mem::size_of::<Partition_t>() as libc::c_ulong,
    ) as *mut Partition_t;
    if This.is_null() {
        printOom();
        return 0 as *mut Stream_t;
    }
    memset(
        This as *mut libc::c_void,
        0 as libc::c_int,
        ::std::mem::size_of::<Partition_t>() as libc::c_ulong,
    );
    init_head(&mut (*This).head, &mut PartitionClass, Next);
    if !(force_pread(
        (*This).head.Next,
        buf.as_mut_ptr() as *mut libc::c_char,
        0 as libc::c_int as mt_off_t,
        512 as libc::c_int as size_t,
    ) != 512 as libc::c_int as libc::c_long)
    {
        if (*buf
            .as_mut_ptr()
            .offset(510 as libc::c_int as isize)
            .offset(0 as libc::c_int as isize) as libc::c_int
            + ((*buf
                .as_mut_ptr()
                .offset(510 as libc::c_int as isize)
                .offset(1 as libc::c_int as isize) as libc::c_int) << 8 as libc::c_int))
            as uint16_t as libc::c_int != 0xaa55 as libc::c_int
        {
            if !errmsg.is_null() {
                sprintf(
                    errmsg,
                    b"Device does not have a BIOS partition table\n\0" as *const u8
                        as *const libc::c_char,
                );
            }
        } else {
            partition = &mut *partTable.offset((*dev).partition as isize)
                as *mut partition;
            if (*partition).end.byte0 == 0 {
                if !errmsg.is_null() {
                    sprintf(
                        errmsg,
                        b"Partition %d does not exist\n\0" as *const u8
                            as *const libc::c_char,
                        (*dev).partition,
                    );
                }
            } else {
                partOff = (((*partition).start_sect[0 as libc::c_int as usize]
                    as libc::c_int
                    + (((*partition).start_sect[1 as libc::c_int as usize]
                        as libc::c_int) << 8 as libc::c_int)) as uint16_t as libc::c_int
                    + (((*((*partition).start_sect)
                        .as_mut_ptr()
                        .offset(2 as libc::c_int as isize)
                        .offset(0 as libc::c_int as isize) as libc::c_int
                        + ((*((*partition).start_sect)
                            .as_mut_ptr()
                            .offset(2 as libc::c_int as isize)
                            .offset(1 as libc::c_int as isize) as libc::c_int)
                            << 8 as libc::c_int)) as uint16_t as libc::c_int)
                        << 16 as libc::c_int)) as uint32_t;
                if !maxSize.is_null() {
                    if partOff as libc::c_long > *maxSize >> 9 as libc::c_int {
                        if !errmsg.is_null() {
                            sprintf(
                                errmsg,
                                b"init: Big disks not supported\0" as *const u8
                                    as *const libc::c_char,
                            );
                        }
                        current_block = 3980969283000438973;
                    } else {
                        *maxSize -= (partOff << 9 as libc::c_int) as libc::c_long;
                        if *maxSize
                            > ((((*partition).nr_sects[0 as libc::c_int as usize]
                                as libc::c_int
                                + (((*partition).nr_sects[1 as libc::c_int as usize]
                                    as libc::c_int) << 8 as libc::c_int)) as uint16_t
                                as libc::c_int
                                + (((*((*partition).nr_sects)
                                    .as_mut_ptr()
                                    .offset(2 as libc::c_int as isize)
                                    .offset(0 as libc::c_int as isize) as libc::c_int
                                    + ((*((*partition).nr_sects)
                                        .as_mut_ptr()
                                        .offset(2 as libc::c_int as isize)
                                        .offset(1 as libc::c_int as isize) as libc::c_int)
                                        << 8 as libc::c_int)) as uint16_t as libc::c_int)
                                    << 16 as libc::c_int)) as uint32_t as mt_off_t)
                                << 9 as libc::c_int
                        {
                            *maxSize = ((((*partition)
                                .nr_sects[0 as libc::c_int as usize] as libc::c_int
                                + (((*partition).nr_sects[1 as libc::c_int as usize]
                                    as libc::c_int) << 8 as libc::c_int)) as uint16_t
                                as libc::c_int
                                + (((*((*partition).nr_sects)
                                    .as_mut_ptr()
                                    .offset(2 as libc::c_int as isize)
                                    .offset(0 as libc::c_int as isize) as libc::c_int
                                    + ((*((*partition).nr_sects)
                                        .as_mut_ptr()
                                        .offset(2 as libc::c_int as isize)
                                        .offset(1 as libc::c_int as isize) as libc::c_int)
                                        << 8 as libc::c_int)) as uint16_t as libc::c_int)
                                    << 16 as libc::c_int)) as uint32_t as mt_off_t)
                                << 9 as libc::c_int;
                        }
                        current_block = 11057878835866523405;
                    }
                } else {
                    current_block = 11057878835866523405;
                }
                match current_block {
                    3980969283000438973 => {}
                    _ => {
                        (*This).offset = (partOff as mt_off_t) << 9 as libc::c_int;
                        if mtools_skip_check == 0
                            && consistencyCheck(
                                buf.as_mut_ptr().offset(0x1ae as libc::c_int as isize)
                                    as *mut partition,
                                0 as libc::c_int,
                                0 as libc::c_int,
                                &mut has_activated,
                                (*dev).tot_sectors,
                                dev,
                                0 as libc::c_int as libc::c_uint,
                            ) != 0
                        {
                            fprintf(
                                stderr,
                                b"Warning: inconsistent partition table\n\0" as *const u8
                                    as *const libc::c_char,
                            );
                            fprintf(
                                stderr,
                                b"Possibly unpartitioned device\n\0" as *const u8
                                    as *const libc::c_char,
                            );
                            fprintf(
                                stderr,
                                b"\n*** Maybe try without partition=%d in device definition ***\n\n\0"
                                    as *const u8 as *const libc::c_char,
                                (*dev).partition,
                            );
                            fprintf(
                                stderr,
                                b"If this is a PCMCIA card, or a disk partitioned on another computer, this message may be in error: add mtools_skip_check=1 to your .mtoolsrc file to suppress this warning\n\0"
                                    as *const u8 as *const libc::c_char,
                            );
                        }
                        (*This)
                            .nbSect = (((*partition).nr_sects[0 as libc::c_int as usize]
                            as libc::c_int
                            + (((*partition).nr_sects[1 as libc::c_int as usize]
                                as libc::c_int) << 8 as libc::c_int)) as uint16_t
                            as libc::c_int
                            + (((*((*partition).nr_sects)
                                .as_mut_ptr()
                                .offset(2 as libc::c_int as isize)
                                .offset(0 as libc::c_int as isize) as libc::c_int
                                + ((*((*partition).nr_sects)
                                    .as_mut_ptr()
                                    .offset(2 as libc::c_int as isize)
                                    .offset(1 as libc::c_int as isize) as libc::c_int)
                                    << 8 as libc::c_int)) as uint16_t as libc::c_int)
                                << 16 as libc::c_int)) as uint32_t;
                        (*dev).tot_sectors = (*This).nbSect;
                        (*This).size = ((*This).nbSect as mt_off_t) << 9 as libc::c_int;
                        return &mut (*This).head;
                    }
                }
            }
        }
    }
    free(This as *mut libc::c_char as *mut libc::c_void);
    return 0 as *mut Stream_t;
}
