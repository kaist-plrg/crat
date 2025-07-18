use ::libc;
extern "C" {
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
    fn strcpy(_: *mut libc::c_char, _: *const libc::c_char) -> *mut libc::c_char;
    fn strncpy(
        _: *mut libc::c_char,
        _: *const libc::c_char,
        _: libc::c_ulong,
    ) -> *mut libc::c_char;
    fn strcat(_: *mut libc::c_char, _: *const libc::c_char) -> *mut libc::c_char;
    fn strncat(
        _: *mut libc::c_char,
        _: *const libc::c_char,
        _: libc::c_ulong,
    ) -> *mut libc::c_char;
    fn strncmp(
        _: *const libc::c_char,
        _: *const libc::c_char,
        _: libc::c_ulong,
    ) -> libc::c_int;
    fn strlen(_: *const libc::c_char) -> libc::c_ulong;
    fn error(
        __status: libc::c_int,
        __errnum: libc::c_int,
        __format: *const libc::c_char,
        _: ...
    );
    fn dcgettext(
        __domainname: *const libc::c_char,
        __msgid: *const libc::c_char,
        __category: libc::c_int,
    ) -> *mut libc::c_char;
    fn cpio_set_c_name(file_hdr: *mut cpio_file_stat, name: *mut libc::c_char);
    static mut archive_format: archive_format;
    static mut numeric_uid: libc::c_int;
    fn warn_junk_bytes(bytes_skipped: libc::c_long);
    fn to_ascii(
        where_0: *mut libc::c_char,
        v: uintmax_t,
        digits: size_t,
        logbase: libc::c_uint,
        nul: bool,
    ) -> libc::c_int;
    fn field_width_error(
        filename: *const libc::c_char,
        fieldname: *const libc::c_char,
        value: uintmax_t,
        width: size_t,
        nul: bool,
    );
    fn tape_buffered_write(
        in_buf: *mut libc::c_char,
        out_des: libc::c_int,
        num_bytes: off_t,
    );
    fn tape_buffered_read(
        in_buf: *mut libc::c_char,
        in_des: libc::c_int,
        num_bytes: off_t,
    );
    fn getgidbyname(group: *mut libc::c_char) -> *mut gid_t;
    fn getuidbyname(user: *mut libc::c_char) -> *mut uid_t;
    fn getuser(uid: uid_t) -> *mut libc::c_char;
    fn getgroup(gid: gid_t) -> *mut libc::c_char;
    fn from_ascii(
        where_0: *const libc::c_char,
        digs: size_t,
        logbase: libc::c_uint,
    ) -> uintmax_t;
}
pub type __uint32_t = libc::c_uint;
pub type __uintmax_t = libc::c_ulong;
pub type __uid_t = libc::c_uint;
pub type __gid_t = libc::c_uint;
pub type __ino_t = libc::c_ulong;
pub type __mode_t = libc::c_uint;
pub type __off_t = libc::c_long;
pub type __time_t = libc::c_long;
pub type ino_t = __ino_t;
pub type gid_t = __gid_t;
pub type mode_t = __mode_t;
pub type uid_t = __uid_t;
pub type off_t = __off_t;
pub type time_t = __time_t;
pub type size_t = libc::c_ulong;
pub type uint32_t = __uint32_t;
pub type uintmax_t = __uintmax_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct cpio_file_stat {
    pub c_magic: libc::c_ushort,
    pub c_ino: ino_t,
    pub c_mode: mode_t,
    pub c_uid: uid_t,
    pub c_gid: gid_t,
    pub c_nlink: size_t,
    pub c_mtime: time_t,
    pub c_filesize: off_t,
    pub c_dev_maj: libc::c_uint,
    pub c_dev_min: libc::c_uint,
    pub c_rdev_maj: libc::c_uint,
    pub c_rdev_min: libc::c_uint,
    pub c_namesize: size_t,
    pub c_chksum: uint32_t,
    pub c_name: *mut libc::c_char,
    pub c_name_buflen: size_t,
    pub c_tar_linkname: *const libc::c_char,
}
pub type archive_format = libc::c_uint;
pub const arf_hpbinary: archive_format = 8;
pub const arf_hpoldascii: archive_format = 7;
pub const arf_ustar: archive_format = 6;
pub const arf_tar: archive_format = 5;
pub const arf_crcascii: archive_format = 4;
pub const arf_newascii: archive_format = 3;
pub const arf_oldascii: archive_format = 2;
pub const arf_binary: archive_format = 1;
pub const arf_unknown: archive_format = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub union tar_record {
    pub header: tar_header,
    pub buffer: [libc::c_char; 512],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct tar_header {
    pub name: [libc::c_char; 100],
    pub mode: [libc::c_char; 8],
    pub uid: [libc::c_char; 8],
    pub gid: [libc::c_char; 8],
    pub size: [libc::c_char; 12],
    pub mtime: [libc::c_char; 12],
    pub chksum: [libc::c_char; 8],
    pub typeflag: libc::c_char,
    pub linkname: [libc::c_char; 100],
    pub magic: [libc::c_char; 6],
    pub version: [libc::c_char; 2],
    pub uname: [libc::c_char; 32],
    pub gname: [libc::c_char; 32],
    pub devmajor: [libc::c_char; 8],
    pub devminor: [libc::c_char; 8],
    pub prefix: [libc::c_char; 155],
}
unsafe extern "C" fn stash_tar_linkname(
    mut linkname: *mut libc::c_char,
) -> *mut libc::c_char {
    static mut hold_tar_linkname: [libc::c_char; 101] = [0; 101];
    strncpy(
        hold_tar_linkname.as_mut_ptr(),
        linkname,
        100 as libc::c_int as libc::c_ulong,
    );
    hold_tar_linkname[100 as libc::c_int as usize] = '\0' as i32 as libc::c_char;
    return hold_tar_linkname.as_mut_ptr();
}
unsafe extern "C" fn split_long_name(
    mut name: *const libc::c_char,
    mut length: size_t,
) -> size_t {
    let mut i: size_t = 0;
    if length > 155 as libc::c_int as libc::c_ulong {
        length = (155 as libc::c_int + 2 as libc::c_int) as size_t;
    }
    i = length.wrapping_sub(1 as libc::c_int as libc::c_ulong);
    while i > 0 as libc::c_int as libc::c_ulong {
        if *name.offset(i as isize) as libc::c_int == '/' as i32 {
            break;
        }
        i = i.wrapping_sub(1);
        i;
    }
    return i;
}
unsafe extern "C" fn stash_tar_filename(
    mut prefix: *mut libc::c_char,
    mut filename: *mut libc::c_char,
) -> *mut libc::c_char {
    static mut hold_tar_filename: [libc::c_char; 257] = [0; 257];
    if prefix.is_null() || *prefix as libc::c_int == '\0' as i32 {
        strncpy(
            hold_tar_filename.as_mut_ptr(),
            filename,
            100 as libc::c_int as libc::c_ulong,
        );
        hold_tar_filename[100 as libc::c_int as usize] = '\0' as i32 as libc::c_char;
    } else {
        strncpy(
            hold_tar_filename.as_mut_ptr(),
            prefix,
            155 as libc::c_int as libc::c_ulong,
        );
        hold_tar_filename[155 as libc::c_int as usize] = '\0' as i32 as libc::c_char;
        strcat(
            hold_tar_filename.as_mut_ptr(),
            b"/\0" as *const u8 as *const libc::c_char,
        );
        strncat(
            hold_tar_filename.as_mut_ptr(),
            filename,
            100 as libc::c_int as libc::c_ulong,
        );
        hold_tar_filename[(155 as libc::c_int + 100 as libc::c_int)
            as usize] = '\0' as i32 as libc::c_char;
    }
    return hold_tar_filename.as_mut_ptr();
}
unsafe extern "C" fn to_oct_or_error(
    mut value: uintmax_t,
    mut digits: size_t,
    mut where_0: *mut libc::c_char,
    mut field: *const libc::c_char,
    mut file: *const libc::c_char,
) -> libc::c_int {
    if to_ascii(
        where_0,
        value,
        digits,
        3 as libc::c_int as libc::c_uint,
        1 as libc::c_int != 0,
    ) != 0
    {
        field_width_error(file, field, value, digits, 1 as libc::c_int != 0);
        return 1 as libc::c_int;
    }
    return 0 as libc::c_int;
}
pub unsafe extern "C" fn tar_checksum(mut tar_hdr: *mut tar_header) -> libc::c_uint {
    let mut sum: libc::c_uint = 0 as libc::c_int as libc::c_uint;
    let mut p: *mut libc::c_char = tar_hdr as *mut libc::c_char;
    let mut q: *mut libc::c_char = p.offset(512 as libc::c_int as isize);
    let mut i: libc::c_int = 0;
    while p < ((*tar_hdr).chksum).as_mut_ptr() {
        let fresh0 = p;
        p = p.offset(1);
        sum = sum
            .wrapping_add(
                (*fresh0 as libc::c_int & 0xff as libc::c_int) as libc::c_uint,
            );
    }
    i = 0 as libc::c_int;
    while i < 8 as libc::c_int {
        sum = sum.wrapping_add(' ' as i32 as libc::c_uint);
        p = p.offset(1);
        p;
        i += 1;
        i;
    }
    while p < q {
        let fresh1 = p;
        p = p.offset(1);
        sum = sum
            .wrapping_add(
                (*fresh1 as libc::c_int & 0xff as libc::c_int) as libc::c_uint,
            );
    }
    return sum;
}
pub unsafe extern "C" fn write_out_tar_header(
    mut file_hdr: *mut cpio_file_stat,
    mut out_des: libc::c_int,
) -> libc::c_int {
    let mut name_len: libc::c_int = 0;
    let mut tar_rec: tar_record = tar_record {
        header: tar_header {
            name: [0; 100],
            mode: [0; 8],
            uid: [0; 8],
            gid: [0; 8],
            size: [0; 12],
            mtime: [0; 12],
            chksum: [0; 8],
            typeflag: 0,
            linkname: [0; 100],
            magic: [0; 6],
            version: [0; 2],
            uname: [0; 32],
            gname: [0; 32],
            devmajor: [0; 8],
            devminor: [0; 8],
            prefix: [0; 155],
        },
    };
    let mut tar_hdr: *mut tar_header = &mut tar_rec as *mut tar_record
        as *mut tar_header;
    memset(
        &mut tar_rec as *mut tar_record as *mut libc::c_void,
        0 as libc::c_int,
        ::std::mem::size_of::<tar_record>() as libc::c_ulong,
    );
    name_len = strlen((*file_hdr).c_name) as libc::c_int;
    if name_len <= 100 as libc::c_int {
        strncpy(
            ((*tar_hdr).name).as_mut_ptr(),
            (*file_hdr).c_name,
            name_len as libc::c_ulong,
        );
    } else {
        let mut prefix_len: libc::c_int = split_long_name(
            (*file_hdr).c_name,
            name_len as size_t,
        ) as libc::c_int;
        strncpy(
            ((*tar_hdr).prefix).as_mut_ptr(),
            (*file_hdr).c_name,
            prefix_len as libc::c_ulong,
        );
        strncpy(
            ((*tar_hdr).name).as_mut_ptr(),
            ((*file_hdr).c_name)
                .offset(prefix_len as isize)
                .offset(1 as libc::c_int as isize),
            (name_len - prefix_len - 1 as libc::c_int) as libc::c_ulong,
        );
    }
    if to_oct_or_error(
        ((*file_hdr).c_mode
            & (0o4000 as libc::c_int | 0o2000 as libc::c_int | 0o1000 as libc::c_int
                | (0o100 as libc::c_int | 0o100 as libc::c_int >> 3 as libc::c_int
                    | 0o100 as libc::c_int >> 3 as libc::c_int >> 3 as libc::c_int
                    | (0o200 as libc::c_int | 0o200 as libc::c_int >> 3 as libc::c_int
                        | 0o200 as libc::c_int >> 3 as libc::c_int >> 3 as libc::c_int
                        | (0o400 as libc::c_int
                            | 0o400 as libc::c_int >> 3 as libc::c_int
                            | 0o400 as libc::c_int >> 3 as libc::c_int
                                >> 3 as libc::c_int)))) as libc::c_uint) as uintmax_t,
        8 as libc::c_int as size_t,
        ((*tar_hdr).mode).as_mut_ptr(),
        b"mode\0" as *const u8 as *const libc::c_char,
        (*file_hdr).c_name,
    ) != 0
    {
        return 1 as libc::c_int;
    }
    if to_oct_or_error(
        (*file_hdr).c_uid as uintmax_t,
        8 as libc::c_int as size_t,
        ((*tar_hdr).uid).as_mut_ptr(),
        b"uid\0" as *const u8 as *const libc::c_char,
        (*file_hdr).c_name,
    ) != 0
    {
        return 1 as libc::c_int;
    }
    if to_oct_or_error(
        (*file_hdr).c_gid as uintmax_t,
        8 as libc::c_int as size_t,
        ((*tar_hdr).gid).as_mut_ptr(),
        b"gid\0" as *const u8 as *const libc::c_char,
        (*file_hdr).c_name,
    ) != 0
    {
        return 1 as libc::c_int;
    }
    if to_oct_or_error(
        (*file_hdr).c_filesize as uintmax_t,
        12 as libc::c_int as size_t,
        ((*tar_hdr).size).as_mut_ptr(),
        b"size\0" as *const u8 as *const libc::c_char,
        (*file_hdr).c_name,
    ) != 0
    {
        return 1 as libc::c_int;
    }
    if to_oct_or_error(
        (*file_hdr).c_mtime as uintmax_t,
        12 as libc::c_int as size_t,
        ((*tar_hdr).mtime).as_mut_ptr(),
        b"mtime\0" as *const u8 as *const libc::c_char,
        (*file_hdr).c_name,
    ) != 0
    {
        return 1 as libc::c_int;
    }
    match (*file_hdr).c_mode & 0o170000 as libc::c_int as libc::c_uint {
        32768 => {
            if !((*file_hdr).c_tar_linkname).is_null() {
                strncpy(
                    ((*tar_hdr).linkname).as_mut_ptr(),
                    (*file_hdr).c_tar_linkname,
                    100 as libc::c_int as libc::c_ulong,
                );
                (*tar_hdr).typeflag = '1' as i32 as libc::c_char;
                to_ascii(
                    ((*tar_hdr).size).as_mut_ptr(),
                    0 as libc::c_int as uintmax_t,
                    12 as libc::c_int as size_t,
                    3 as libc::c_int as libc::c_uint,
                    1 as libc::c_int != 0,
                );
            } else {
                (*tar_hdr).typeflag = '0' as i32 as libc::c_char;
            }
        }
        16384 => {
            (*tar_hdr).typeflag = '5' as i32 as libc::c_char;
        }
        8192 => {
            (*tar_hdr).typeflag = '3' as i32 as libc::c_char;
        }
        24576 => {
            (*tar_hdr).typeflag = '4' as i32 as libc::c_char;
        }
        4096 => {
            (*tar_hdr).typeflag = '6' as i32 as libc::c_char;
        }
        40960 => {
            (*tar_hdr).typeflag = '2' as i32 as libc::c_char;
            strncpy(
                ((*tar_hdr).linkname).as_mut_ptr(),
                (*file_hdr).c_tar_linkname,
                100 as libc::c_int as libc::c_ulong,
            );
            to_ascii(
                ((*tar_hdr).size).as_mut_ptr(),
                0 as libc::c_int as uintmax_t,
                12 as libc::c_int as size_t,
                3 as libc::c_int as libc::c_uint,
                1 as libc::c_int != 0,
            );
        }
        _ => {}
    }
    if archive_format as libc::c_uint == arf_ustar as libc::c_int as libc::c_uint {
        let mut name: *mut libc::c_char = 0 as *mut libc::c_char;
        strncpy(
            ((*tar_hdr).magic).as_mut_ptr(),
            b"ustar\0" as *const u8 as *const libc::c_char,
            6 as libc::c_int as libc::c_ulong,
        );
        strncpy(
            ((*tar_hdr).version).as_mut_ptr(),
            b"00\0" as *const u8 as *const libc::c_char,
            2 as libc::c_int as libc::c_ulong,
        );
        name = getuser((*file_hdr).c_uid);
        if !name.is_null() {
            strcpy(((*tar_hdr).uname).as_mut_ptr(), name);
        }
        name = getgroup((*file_hdr).c_gid);
        if !name.is_null() {
            strcpy(((*tar_hdr).gname).as_mut_ptr(), name);
        }
        if to_oct_or_error(
            (*file_hdr).c_rdev_maj as uintmax_t,
            8 as libc::c_int as size_t,
            ((*tar_hdr).devmajor).as_mut_ptr(),
            b"devmajor\0" as *const u8 as *const libc::c_char,
            (*file_hdr).c_name,
        ) != 0
        {
            return 1 as libc::c_int;
        }
        if to_oct_or_error(
            (*file_hdr).c_rdev_min as uintmax_t,
            8 as libc::c_int as size_t,
            ((*tar_hdr).devminor).as_mut_ptr(),
            b"devminor\0" as *const u8 as *const libc::c_char,
            (*file_hdr).c_name,
        ) != 0
        {
            return 1 as libc::c_int;
        }
    }
    to_ascii(
        ((*tar_hdr).chksum).as_mut_ptr(),
        tar_checksum(tar_hdr) as uintmax_t,
        8 as libc::c_int as size_t,
        3 as libc::c_int as libc::c_uint,
        1 as libc::c_int != 0,
    );
    tape_buffered_write(
        &mut tar_rec as *mut tar_record as *mut libc::c_char,
        out_des,
        512 as libc::c_int as off_t,
    );
    return 0 as libc::c_int;
}
pub unsafe extern "C" fn null_block(
    mut block: *mut libc::c_long,
    mut size: libc::c_int,
) -> libc::c_int {
    let mut p: *mut libc::c_long = block;
    let mut i: libc::c_int = (size as libc::c_ulong)
        .wrapping_div(::std::mem::size_of::<libc::c_long>() as libc::c_ulong)
        as libc::c_int;
    loop {
        let fresh2 = i;
        i = i - 1;
        if !(fresh2 != 0) {
            break;
        }
        let fresh3 = p;
        p = p.offset(1);
        if *fresh3 != 0 {
            return 0 as libc::c_int;
        }
    }
    return 1 as libc::c_int;
}
pub unsafe extern "C" fn read_in_tar_header(
    mut file_hdr: *mut cpio_file_stat,
    mut in_des: libc::c_int,
) {
    let mut bytes_skipped: libc::c_long = 0 as libc::c_int as libc::c_long;
    let mut warned: libc::c_int = 0 as libc::c_int;
    let mut tar_rec: tar_record = tar_record {
        header: tar_header {
            name: [0; 100],
            mode: [0; 8],
            uid: [0; 8],
            gid: [0; 8],
            size: [0; 12],
            mtime: [0; 12],
            chksum: [0; 8],
            typeflag: 0,
            linkname: [0; 100],
            magic: [0; 6],
            version: [0; 2],
            uname: [0; 32],
            gname: [0; 32],
            devmajor: [0; 8],
            devminor: [0; 8],
            prefix: [0; 155],
        },
    };
    let mut tar_hdr: *mut tar_header = &mut tar_rec as *mut tar_record
        as *mut tar_header;
    let mut uidp: *mut uid_t = 0 as *mut uid_t;
    let mut gidp: *mut gid_t = 0 as *mut gid_t;
    tape_buffered_read(
        &mut tar_rec as *mut tar_record as *mut libc::c_char,
        in_des,
        512 as libc::c_int as off_t,
    );
    if null_block(
        &mut tar_rec as *mut tar_record as *mut libc::c_long,
        512 as libc::c_int,
    ) != 0
    {
        cpio_set_c_name(
            file_hdr,
            b"TRAILER!!!\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        );
        return;
    }
    loop {
        (*file_hdr)
            .c_chksum = from_ascii(
            ((*tar_hdr).chksum).as_mut_ptr(),
            ::std::mem::size_of::<[libc::c_char; 8]>() as libc::c_ulong,
            3 as libc::c_int as libc::c_uint,
        ) as uint32_t;
        if (*file_hdr).c_chksum != tar_checksum(tar_hdr) {
            if warned == 0 {
                error(
                    0 as libc::c_int,
                    0 as libc::c_int,
                    dcgettext(
                        0 as *const libc::c_char,
                        b"invalid header: checksum error\0" as *const u8
                            as *const libc::c_char,
                        5 as libc::c_int,
                    ),
                );
                warned = 1 as libc::c_int;
            }
            memmove(
                &mut tar_rec as *mut tar_record as *mut libc::c_void,
                (&mut tar_rec as *mut tar_record as *mut libc::c_char)
                    .offset(1 as libc::c_int as isize) as *const libc::c_void,
                (512 as libc::c_int - 1 as libc::c_int) as libc::c_ulong,
            );
            tape_buffered_read(
                (&mut tar_rec as *mut tar_record as *mut libc::c_char)
                    .offset((512 as libc::c_int - 1 as libc::c_int) as isize),
                in_des,
                1 as libc::c_int as off_t,
            );
            bytes_skipped += 1;
            bytes_skipped;
        } else {
            if archive_format as libc::c_uint != arf_ustar as libc::c_int as libc::c_uint
            {
                cpio_set_c_name(
                    file_hdr,
                    stash_tar_filename(
                        0 as *mut libc::c_char,
                        ((*tar_hdr).name).as_mut_ptr(),
                    ),
                );
            } else {
                cpio_set_c_name(
                    file_hdr,
                    stash_tar_filename(
                        ((*tar_hdr).prefix).as_mut_ptr(),
                        ((*tar_hdr).name).as_mut_ptr(),
                    ),
                );
            }
            (*file_hdr).c_nlink = 1 as libc::c_int as size_t;
            (*file_hdr)
                .c_mode = from_ascii(
                ((*tar_hdr).mode).as_mut_ptr(),
                ::std::mem::size_of::<[libc::c_char; 8]>() as libc::c_ulong,
                3 as libc::c_int as libc::c_uint,
            ) as mode_t;
            (*file_hdr)
                .c_mode = (*file_hdr).c_mode & 0o7777 as libc::c_int as libc::c_uint;
            if archive_format as libc::c_uint == arf_ustar as libc::c_int as libc::c_uint
                && numeric_uid == 0
                && {
                    uidp = getuidbyname(((*tar_hdr).uname).as_mut_ptr());
                    !uidp.is_null()
                }
            {
                (*file_hdr).c_uid = *uidp;
            } else {
                (*file_hdr)
                    .c_uid = from_ascii(
                    ((*tar_hdr).uid).as_mut_ptr(),
                    ::std::mem::size_of::<[libc::c_char; 8]>() as libc::c_ulong,
                    3 as libc::c_int as libc::c_uint,
                ) as uid_t;
            }
            if archive_format as libc::c_uint == arf_ustar as libc::c_int as libc::c_uint
                && numeric_uid == 0
                && {
                    gidp = getgidbyname(((*tar_hdr).gname).as_mut_ptr());
                    !gidp.is_null()
                }
            {
                (*file_hdr).c_gid = *gidp;
            } else {
                (*file_hdr)
                    .c_gid = from_ascii(
                    ((*tar_hdr).gid).as_mut_ptr(),
                    ::std::mem::size_of::<[libc::c_char; 8]>() as libc::c_ulong,
                    3 as libc::c_int as libc::c_uint,
                ) as gid_t;
            }
            (*file_hdr)
                .c_filesize = from_ascii(
                ((*tar_hdr).size).as_mut_ptr(),
                ::std::mem::size_of::<[libc::c_char; 12]>() as libc::c_ulong,
                3 as libc::c_int as libc::c_uint,
            ) as off_t;
            (*file_hdr)
                .c_mtime = from_ascii(
                ((*tar_hdr).mtime).as_mut_ptr(),
                ::std::mem::size_of::<[libc::c_char; 12]>() as libc::c_ulong,
                3 as libc::c_int as libc::c_uint,
            ) as time_t;
            (*file_hdr)
                .c_rdev_maj = from_ascii(
                ((*tar_hdr).devmajor).as_mut_ptr(),
                ::std::mem::size_of::<[libc::c_char; 8]>() as libc::c_ulong,
                3 as libc::c_int as libc::c_uint,
            ) as libc::c_uint;
            (*file_hdr)
                .c_rdev_min = from_ascii(
                ((*tar_hdr).devminor).as_mut_ptr(),
                ::std::mem::size_of::<[libc::c_char; 8]>() as libc::c_ulong,
                3 as libc::c_int as libc::c_uint,
            ) as libc::c_uint;
            (*file_hdr).c_tar_linkname = 0 as *const libc::c_char;
            match (*tar_hdr).typeflag as libc::c_int {
                53 => {
                    (*file_hdr).c_mode |= 0o40000 as libc::c_int as libc::c_uint;
                }
                51 => {
                    (*file_hdr).c_mode |= 0o20000 as libc::c_int as libc::c_uint;
                    (*file_hdr)
                        .c_tar_linkname = stash_tar_linkname(
                        ((*tar_hdr).linkname).as_mut_ptr(),
                    );
                    (*file_hdr).c_filesize = 0 as libc::c_int as off_t;
                }
                52 => {
                    (*file_hdr).c_mode |= 0o60000 as libc::c_int as libc::c_uint;
                    (*file_hdr)
                        .c_tar_linkname = stash_tar_linkname(
                        ((*tar_hdr).linkname).as_mut_ptr(),
                    );
                    (*file_hdr).c_filesize = 0 as libc::c_int as off_t;
                }
                54 => {
                    (*file_hdr).c_mode |= 0o10000 as libc::c_int as libc::c_uint;
                    (*file_hdr)
                        .c_tar_linkname = stash_tar_linkname(
                        ((*tar_hdr).linkname).as_mut_ptr(),
                    );
                    (*file_hdr).c_filesize = 0 as libc::c_int as off_t;
                }
                50 => {
                    (*file_hdr).c_mode |= 0o120000 as libc::c_int as libc::c_uint;
                    (*file_hdr)
                        .c_tar_linkname = stash_tar_linkname(
                        ((*tar_hdr).linkname).as_mut_ptr(),
                    );
                    (*file_hdr).c_filesize = 0 as libc::c_int as off_t;
                }
                49 => {
                    (*file_hdr).c_mode |= 0o100000 as libc::c_int as libc::c_uint;
                    (*file_hdr)
                        .c_tar_linkname = stash_tar_linkname(
                        ((*tar_hdr).linkname).as_mut_ptr(),
                    );
                    (*file_hdr).c_filesize = 0 as libc::c_int as off_t;
                }
                0 => {
                    if *((*file_hdr).c_name)
                        .offset(
                            ((*file_hdr).c_namesize)
                                .wrapping_sub(1 as libc::c_int as libc::c_ulong) as isize,
                        ) as libc::c_int == '/' as i32
                    {
                        (*file_hdr).c_mode |= 0o40000 as libc::c_int as libc::c_uint;
                    } else {
                        (*file_hdr).c_mode |= 0o100000 as libc::c_int as libc::c_uint;
                    }
                }
                48 | 55 | _ => {
                    (*file_hdr).c_mode |= 0o100000 as libc::c_int as libc::c_uint;
                }
            }
            break;
        }
    }
    if bytes_skipped > 0 as libc::c_int as libc::c_long {
        warn_junk_bytes(bytes_skipped);
    }
}
pub unsafe extern "C" fn is_tar_header(mut buf: *mut libc::c_char) -> libc::c_int {
    let mut tar_hdr: *mut tar_header = buf as *mut tar_header;
    let mut chksum: libc::c_ulong = 0;
    chksum = from_ascii(
        ((*tar_hdr).chksum).as_mut_ptr(),
        ::std::mem::size_of::<[libc::c_char; 8]>() as libc::c_ulong,
        3 as libc::c_int as libc::c_uint,
    );
    if chksum != tar_checksum(tar_hdr) as libc::c_ulong {
        return 0 as libc::c_int;
    }
    if strncmp(
        ((*tar_hdr).magic).as_mut_ptr(),
        b"ustar\0" as *const u8 as *const libc::c_char,
        (6 as libc::c_int - 1 as libc::c_int) as libc::c_ulong,
    ) == 0
    {
        return 2 as libc::c_int;
    }
    return 1 as libc::c_int;
}
pub unsafe extern "C" fn is_tar_filename_too_long(
    mut name: *mut libc::c_char,
) -> libc::c_int {
    let mut whole_name_len: libc::c_int = 0;
    let mut prefix_name_len: libc::c_int = 0;
    whole_name_len = strlen(name) as libc::c_int;
    if whole_name_len <= 100 as libc::c_int {
        return 0 as libc::c_int;
    }
    if archive_format as libc::c_uint != arf_ustar as libc::c_int as libc::c_uint {
        return 1 as libc::c_int;
    }
    if whole_name_len > 100 as libc::c_int + 155 as libc::c_int + 1 as libc::c_int {
        return 1 as libc::c_int;
    }
    prefix_name_len = split_long_name(name, whole_name_len as size_t) as libc::c_int;
    if prefix_name_len == 0 as libc::c_int
        || whole_name_len - prefix_name_len - 1 as libc::c_int > 100 as libc::c_int
    {
        return 1 as libc::c_int;
    }
    return 0 as libc::c_int;
}
