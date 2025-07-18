use ::libc;
use ::c2rust_bitfields;
extern "C" {
    pub type _IO_wide_data;
    pub type _IO_codecvt;
    pub type _IO_marker;
    fn rpl_fclose(stream: *mut FILE) -> libc::c_int;
    fn sscanf(_: *const libc::c_char, _: *const libc::c_char, _: ...) -> libc::c_int;
    fn __getdelim(
        __lineptr: *mut *mut libc::c_char,
        __n: *mut size_t,
        __delimiter: libc::c_int,
        __stream: *mut FILE,
    ) -> __ssize_t;
    fn fopen(_: *const libc::c_char, _: *const libc::c_char) -> *mut FILE;
    fn rpl_free(ptr: *mut libc::c_void);
    fn strcmp(_: *const libc::c_char, _: *const libc::c_char) -> libc::c_int;
    fn strchr(_: *const libc::c_char, _: libc::c_int) -> *mut libc::c_char;
    fn strstr(_: *const libc::c_char, _: *const libc::c_char) -> *mut libc::c_char;
    fn strlen(_: *const libc::c_char) -> libc::c_ulong;
    fn xmalloc(s: size_t) -> *mut libc::c_void;
    fn xstrdup(str: *const libc::c_char) -> *mut libc::c_char;
    fn __errno_location() -> *mut libc::c_int;
    fn setmntent(__file: *const libc::c_char, __mode: *const libc::c_char) -> *mut FILE;
    fn getmntent(__stream: *mut FILE) -> *mut mntent;
    fn endmntent(__stream: *mut FILE) -> libc::c_int;
    fn hasmntopt(__mnt: *const mntent, __opt: *const libc::c_char) -> *mut libc::c_char;
}
pub type __dev_t = libc::c_ulong;
pub type __off_t = libc::c_long;
pub type __off64_t = libc::c_long;
pub type __ssize_t = libc::c_long;
pub type dev_t = __dev_t;
pub type size_t = libc::c_ulong;
#[derive(Copy, Clone, BitfieldStruct)]
#[repr(C)]
pub struct mount_entry {
    pub me_devname: *mut libc::c_char,
    pub me_mountdir: *mut libc::c_char,
    pub me_mntroot: *mut libc::c_char,
    pub me_type: *mut libc::c_char,
    pub me_dev: dev_t,
    #[bitfield(name = "me_dummy", ty = "libc::c_uint", bits = "0..=0")]
    #[bitfield(name = "me_remote", ty = "libc::c_uint", bits = "1..=1")]
    #[bitfield(name = "me_type_malloced", ty = "libc::c_uint", bits = "2..=2")]
    pub me_dummy_me_remote_me_type_malloced: [u8; 1],
    #[bitfield(padding)]
    pub c2rust_padding: [u8; 7],
    pub me_next: *mut mount_entry,
}
pub type FILE = _IO_FILE;
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct mntent {
    pub mnt_fsname: *mut libc::c_char,
    pub mnt_dir: *mut libc::c_char,
    pub mnt_type: *mut libc::c_char,
    pub mnt_opts: *mut libc::c_char,
    pub mnt_freq: libc::c_int,
    pub mnt_passno: libc::c_int,
}
#[inline]
unsafe extern "C" fn ferror_unlocked(mut __stream: *mut FILE) -> libc::c_int {
    return ((*__stream)._flags & 0x20 as libc::c_int != 0 as libc::c_int) as libc::c_int;
}
#[inline]
unsafe extern "C" fn getline(
    mut __lineptr: *mut *mut libc::c_char,
    mut __n: *mut size_t,
    mut __stream: *mut FILE,
) -> __ssize_t {
    return __getdelim(__lineptr, __n, '\n' as i32, __stream);
}
#[inline]
unsafe extern "C" fn gnu_dev_makedev(
    mut __major: libc::c_uint,
    mut __minor: libc::c_uint,
) -> __dev_t {
    let mut __dev: __dev_t = 0;
    __dev = ((__major & 0xfff as libc::c_uint) as __dev_t) << 8 as libc::c_int;
    __dev |= ((__major & 0xfffff000 as libc::c_uint) as __dev_t) << 32 as libc::c_int;
    __dev |= ((__minor & 0xff as libc::c_uint) as __dev_t) << 0 as libc::c_int;
    __dev |= ((__minor & 0xffffff00 as libc::c_uint) as __dev_t) << 12 as libc::c_int;
    return __dev;
}
unsafe extern "C" fn dev_from_mount_options(
    mut mount_options: *const libc::c_char,
) -> dev_t {
    return -(1 as libc::c_int) as dev_t;
}
unsafe extern "C" fn unescape_tab(mut str: *mut libc::c_char) {
    let mut i: size_t = 0;
    let mut j: size_t = 0 as libc::c_int as size_t;
    let mut len: size_t = (strlen(str)).wrapping_add(1 as libc::c_int as libc::c_ulong);
    i = 0 as libc::c_int as size_t;
    while i < len {
        if *str.offset(i as isize) as libc::c_int == '\\' as i32
            && i.wrapping_add(4 as libc::c_int as libc::c_ulong) < len
            && *str.offset(i.wrapping_add(1 as libc::c_int as libc::c_ulong) as isize)
                as libc::c_int >= '0' as i32
            && *str.offset(i.wrapping_add(1 as libc::c_int as libc::c_ulong) as isize)
                as libc::c_int <= '3' as i32
            && *str.offset(i.wrapping_add(2 as libc::c_int as libc::c_ulong) as isize)
                as libc::c_int >= '0' as i32
            && *str.offset(i.wrapping_add(2 as libc::c_int as libc::c_ulong) as isize)
                as libc::c_int <= '7' as i32
            && *str.offset(i.wrapping_add(3 as libc::c_int as libc::c_ulong) as isize)
                as libc::c_int >= '0' as i32
            && *str.offset(i.wrapping_add(3 as libc::c_int as libc::c_ulong) as isize)
                as libc::c_int <= '7' as i32
        {
            let fresh0 = j;
            j = j.wrapping_add(1);
            *str
                .offset(
                    fresh0 as isize,
                ) = ((*str
                .offset(i.wrapping_add(1 as libc::c_int as libc::c_ulong) as isize)
                as libc::c_int - '0' as i32) * 64 as libc::c_int
                + (*str
                    .offset(i.wrapping_add(2 as libc::c_int as libc::c_ulong) as isize)
                    as libc::c_int - '0' as i32) * 8 as libc::c_int
                + (*str
                    .offset(i.wrapping_add(3 as libc::c_int as libc::c_ulong) as isize)
                    as libc::c_int - '0' as i32)) as libc::c_char;
            i = (i as libc::c_ulong).wrapping_add(3 as libc::c_int as libc::c_ulong)
                as size_t as size_t;
        } else {
            let fresh1 = j;
            j = j.wrapping_add(1);
            *str.offset(fresh1 as isize) = *str.offset(i as isize);
        }
        i = i.wrapping_add(1);
        i;
    }
}
unsafe extern "C" fn terminate_at_blank(
    mut str: *mut libc::c_char,
) -> *mut libc::c_char {
    let mut s: *mut libc::c_char = strchr(str, ' ' as i32);
    if !s.is_null() {
        *s = '\0' as i32 as libc::c_char;
    }
    return s;
}
pub unsafe extern "C" fn read_file_system_list(
    mut need_fs_type: bool,
) -> *mut mount_entry {
    let mut current_block: u64;
    let mut mount_list: *mut mount_entry = 0 as *mut mount_entry;
    let mut me: *mut mount_entry = 0 as *mut mount_entry;
    let mut mtail: *mut *mut mount_entry = &mut mount_list;
    let mut fp: *mut FILE = 0 as *mut FILE;
    let mut mountinfo: *const libc::c_char = b"/proc/self/mountinfo\0" as *const u8
        as *const libc::c_char;
    fp = fopen(mountinfo, b"re\0" as *const u8 as *const libc::c_char);
    if !fp.is_null() {
        let mut line: *mut libc::c_char = 0 as *mut libc::c_char;
        let mut buf_size: size_t = 0 as libc::c_int as size_t;
        while getline(&mut line, &mut buf_size, fp)
            != -(1 as libc::c_int) as libc::c_long
        {
            let mut devmaj: libc::c_uint = 0;
            let mut devmin: libc::c_uint = 0;
            let mut rc: libc::c_int = 0;
            let mut mntroot_s: libc::c_int = 0;
            rc = sscanf(
                line,
                b"%*u %*u %u:%u %n\0" as *const u8 as *const libc::c_char,
                &mut devmaj as *mut libc::c_uint,
                &mut devmin as *mut libc::c_uint,
                &mut mntroot_s as *mut libc::c_int,
            );
            if rc != 2 as libc::c_int && rc != 3 as libc::c_int {
                continue;
            }
            let mut mntroot: *mut libc::c_char = line.offset(mntroot_s as isize);
            let mut blank: *mut libc::c_char = terminate_at_blank(mntroot);
            if blank.is_null() {
                continue;
            }
            let mut target: *mut libc::c_char = blank.offset(1 as libc::c_int as isize);
            blank = terminate_at_blank(target);
            if blank.is_null() {
                continue;
            }
            let mut dash: *mut libc::c_char = strstr(
                blank.offset(1 as libc::c_int as isize),
                b" - \0" as *const u8 as *const libc::c_char,
            );
            if dash.is_null() {
                continue;
            }
            let mut fstype: *mut libc::c_char = dash.offset(3 as libc::c_int as isize);
            blank = terminate_at_blank(fstype);
            if blank.is_null() {
                continue;
            }
            let mut source: *mut libc::c_char = blank.offset(1 as libc::c_int as isize);
            if (terminate_at_blank(source)).is_null() {
                continue;
            }
            unescape_tab(source);
            unescape_tab(target);
            unescape_tab(mntroot);
            unescape_tab(fstype);
            me = xmalloc(::std::mem::size_of::<mount_entry>() as libc::c_ulong)
                as *mut mount_entry;
            (*me).me_devname = xstrdup(source);
            (*me).me_mountdir = xstrdup(target);
            (*me).me_mntroot = xstrdup(mntroot);
            (*me).me_type = xstrdup(fstype);
            (*me).set_me_type_malloced(1 as libc::c_int as libc::c_uint);
            (*me).me_dev = gnu_dev_makedev(devmaj, devmin);
            (*me)
                .set_me_dummy(
                    (strcmp(
                        (*me).me_type,
                        b"autofs\0" as *const u8 as *const libc::c_char,
                    ) == 0 as libc::c_int
                        || strcmp(
                            (*me).me_type,
                            b"proc\0" as *const u8 as *const libc::c_char,
                        ) == 0 as libc::c_int
                        || strcmp(
                            (*me).me_type,
                            b"subfs\0" as *const u8 as *const libc::c_char,
                        ) == 0 as libc::c_int
                        || strcmp(
                            (*me).me_type,
                            b"debugfs\0" as *const u8 as *const libc::c_char,
                        ) == 0 as libc::c_int
                        || strcmp(
                            (*me).me_type,
                            b"devpts\0" as *const u8 as *const libc::c_char,
                        ) == 0 as libc::c_int
                        || strcmp(
                            (*me).me_type,
                            b"fusectl\0" as *const u8 as *const libc::c_char,
                        ) == 0 as libc::c_int
                        || strcmp(
                            (*me).me_type,
                            b"fuse.portal\0" as *const u8 as *const libc::c_char,
                        ) == 0 as libc::c_int
                        || strcmp(
                            (*me).me_type,
                            b"mqueue\0" as *const u8 as *const libc::c_char,
                        ) == 0 as libc::c_int
                        || strcmp(
                            (*me).me_type,
                            b"rpc_pipefs\0" as *const u8 as *const libc::c_char,
                        ) == 0 as libc::c_int
                        || strcmp(
                            (*me).me_type,
                            b"sysfs\0" as *const u8 as *const libc::c_char,
                        ) == 0 as libc::c_int
                        || strcmp(
                            (*me).me_type,
                            b"devfs\0" as *const u8 as *const libc::c_char,
                        ) == 0 as libc::c_int
                        || strcmp(
                            (*me).me_type,
                            b"kernfs\0" as *const u8 as *const libc::c_char,
                        ) == 0 as libc::c_int
                        || strcmp(
                            (*me).me_type,
                            b"ignore\0" as *const u8 as *const libc::c_char,
                        ) == 0 as libc::c_int
                        || strcmp(
                            (*me).me_type,
                            b"none\0" as *const u8 as *const libc::c_char,
                        ) == 0 as libc::c_int && 0 as libc::c_int == 0) as libc::c_int
                        as libc::c_uint,
                );
            (*me)
                .set_me_remote(
                    (!(strchr((*me).me_devname, ':' as i32)).is_null()
                        || *((*me).me_devname).offset(0 as libc::c_int as isize)
                            as libc::c_int == '/' as i32
                            && *((*me).me_devname).offset(1 as libc::c_int as isize)
                                as libc::c_int == '/' as i32
                            && (strcmp(
                                (*me).me_type,
                                b"smbfs\0" as *const u8 as *const libc::c_char,
                            ) == 0 as libc::c_int
                                || strcmp(
                                    (*me).me_type,
                                    b"smb3\0" as *const u8 as *const libc::c_char,
                                ) == 0 as libc::c_int
                                || strcmp(
                                    (*me).me_type,
                                    b"cifs\0" as *const u8 as *const libc::c_char,
                                ) == 0 as libc::c_int)
                        || strcmp(
                            (*me).me_type,
                            b"acfs\0" as *const u8 as *const libc::c_char,
                        ) == 0 as libc::c_int
                        || strcmp(
                            (*me).me_type,
                            b"afs\0" as *const u8 as *const libc::c_char,
                        ) == 0 as libc::c_int
                        || strcmp(
                            (*me).me_type,
                            b"coda\0" as *const u8 as *const libc::c_char,
                        ) == 0 as libc::c_int
                        || strcmp(
                            (*me).me_type,
                            b"auristorfs\0" as *const u8 as *const libc::c_char,
                        ) == 0 as libc::c_int
                        || strcmp(
                            (*me).me_type,
                            b"fhgfs\0" as *const u8 as *const libc::c_char,
                        ) == 0 as libc::c_int
                        || strcmp(
                            (*me).me_type,
                            b"gpfs\0" as *const u8 as *const libc::c_char,
                        ) == 0 as libc::c_int
                        || strcmp(
                            (*me).me_type,
                            b"ibrix\0" as *const u8 as *const libc::c_char,
                        ) == 0 as libc::c_int
                        || strcmp(
                            (*me).me_type,
                            b"ocfs2\0" as *const u8 as *const libc::c_char,
                        ) == 0 as libc::c_int
                        || strcmp(
                            (*me).me_type,
                            b"vxfs\0" as *const u8 as *const libc::c_char,
                        ) == 0 as libc::c_int
                        || strcmp(
                            b"-hosts\0" as *const u8 as *const libc::c_char,
                            (*me).me_devname,
                        ) == 0 as libc::c_int) as libc::c_int as libc::c_uint,
                );
            *mtail = me;
            mtail = &mut (*me).me_next;
        }
        rpl_free(line as *mut libc::c_void);
        if ferror_unlocked(fp) != 0 {
            let mut saved_errno: libc::c_int = *__errno_location();
            rpl_fclose(fp);
            *__errno_location() = saved_errno;
            current_block = 12497913735442871383;
        } else if rpl_fclose(fp) == -(1 as libc::c_int) {
            current_block = 12497913735442871383;
        } else {
            current_block = 11626999923138678822;
        }
    } else {
        let mut mnt: *mut mntent = 0 as *mut mntent;
        let mut table: *const libc::c_char = b"/etc/mtab\0" as *const u8
            as *const libc::c_char;
        fp = setmntent(table, b"r\0" as *const u8 as *const libc::c_char);
        if fp.is_null() {
            return 0 as *mut mount_entry;
        }
        loop {
            mnt = getmntent(fp);
            if mnt.is_null() {
                break;
            }
            let mut bind: bool = !(hasmntopt(
                mnt,
                b"bind\0" as *const u8 as *const libc::c_char,
            ))
                .is_null();
            me = xmalloc(::std::mem::size_of::<mount_entry>() as libc::c_ulong)
                as *mut mount_entry;
            (*me).me_devname = xstrdup((*mnt).mnt_fsname);
            (*me).me_mountdir = xstrdup((*mnt).mnt_dir);
            (*me).me_mntroot = 0 as *mut libc::c_char;
            (*me).me_type = xstrdup((*mnt).mnt_type);
            (*me).set_me_type_malloced(1 as libc::c_int as libc::c_uint);
            (*me)
                .set_me_dummy(
                    (strcmp(
                        (*me).me_type,
                        b"autofs\0" as *const u8 as *const libc::c_char,
                    ) == 0 as libc::c_int
                        || strcmp(
                            (*me).me_type,
                            b"proc\0" as *const u8 as *const libc::c_char,
                        ) == 0 as libc::c_int
                        || strcmp(
                            (*me).me_type,
                            b"subfs\0" as *const u8 as *const libc::c_char,
                        ) == 0 as libc::c_int
                        || strcmp(
                            (*me).me_type,
                            b"debugfs\0" as *const u8 as *const libc::c_char,
                        ) == 0 as libc::c_int
                        || strcmp(
                            (*me).me_type,
                            b"devpts\0" as *const u8 as *const libc::c_char,
                        ) == 0 as libc::c_int
                        || strcmp(
                            (*me).me_type,
                            b"fusectl\0" as *const u8 as *const libc::c_char,
                        ) == 0 as libc::c_int
                        || strcmp(
                            (*me).me_type,
                            b"fuse.portal\0" as *const u8 as *const libc::c_char,
                        ) == 0 as libc::c_int
                        || strcmp(
                            (*me).me_type,
                            b"mqueue\0" as *const u8 as *const libc::c_char,
                        ) == 0 as libc::c_int
                        || strcmp(
                            (*me).me_type,
                            b"rpc_pipefs\0" as *const u8 as *const libc::c_char,
                        ) == 0 as libc::c_int
                        || strcmp(
                            (*me).me_type,
                            b"sysfs\0" as *const u8 as *const libc::c_char,
                        ) == 0 as libc::c_int
                        || strcmp(
                            (*me).me_type,
                            b"devfs\0" as *const u8 as *const libc::c_char,
                        ) == 0 as libc::c_int
                        || strcmp(
                            (*me).me_type,
                            b"kernfs\0" as *const u8 as *const libc::c_char,
                        ) == 0 as libc::c_int
                        || strcmp(
                            (*me).me_type,
                            b"ignore\0" as *const u8 as *const libc::c_char,
                        ) == 0 as libc::c_int
                        || strcmp(
                            (*me).me_type,
                            b"none\0" as *const u8 as *const libc::c_char,
                        ) == 0 as libc::c_int && !bind) as libc::c_int as libc::c_uint,
                );
            (*me)
                .set_me_remote(
                    (!(strchr((*me).me_devname, ':' as i32)).is_null()
                        || *((*me).me_devname).offset(0 as libc::c_int as isize)
                            as libc::c_int == '/' as i32
                            && *((*me).me_devname).offset(1 as libc::c_int as isize)
                                as libc::c_int == '/' as i32
                            && (strcmp(
                                (*me).me_type,
                                b"smbfs\0" as *const u8 as *const libc::c_char,
                            ) == 0 as libc::c_int
                                || strcmp(
                                    (*me).me_type,
                                    b"smb3\0" as *const u8 as *const libc::c_char,
                                ) == 0 as libc::c_int
                                || strcmp(
                                    (*me).me_type,
                                    b"cifs\0" as *const u8 as *const libc::c_char,
                                ) == 0 as libc::c_int)
                        || strcmp(
                            (*me).me_type,
                            b"acfs\0" as *const u8 as *const libc::c_char,
                        ) == 0 as libc::c_int
                        || strcmp(
                            (*me).me_type,
                            b"afs\0" as *const u8 as *const libc::c_char,
                        ) == 0 as libc::c_int
                        || strcmp(
                            (*me).me_type,
                            b"coda\0" as *const u8 as *const libc::c_char,
                        ) == 0 as libc::c_int
                        || strcmp(
                            (*me).me_type,
                            b"auristorfs\0" as *const u8 as *const libc::c_char,
                        ) == 0 as libc::c_int
                        || strcmp(
                            (*me).me_type,
                            b"fhgfs\0" as *const u8 as *const libc::c_char,
                        ) == 0 as libc::c_int
                        || strcmp(
                            (*me).me_type,
                            b"gpfs\0" as *const u8 as *const libc::c_char,
                        ) == 0 as libc::c_int
                        || strcmp(
                            (*me).me_type,
                            b"ibrix\0" as *const u8 as *const libc::c_char,
                        ) == 0 as libc::c_int
                        || strcmp(
                            (*me).me_type,
                            b"ocfs2\0" as *const u8 as *const libc::c_char,
                        ) == 0 as libc::c_int
                        || strcmp(
                            (*me).me_type,
                            b"vxfs\0" as *const u8 as *const libc::c_char,
                        ) == 0 as libc::c_int
                        || strcmp(
                            b"-hosts\0" as *const u8 as *const libc::c_char,
                            (*me).me_devname,
                        ) == 0 as libc::c_int) as libc::c_int as libc::c_uint,
                );
            (*me).me_dev = dev_from_mount_options((*mnt).mnt_opts);
            *mtail = me;
            mtail = &mut (*me).me_next;
        }
        if endmntent(fp) == 0 as libc::c_int {
            current_block = 12497913735442871383;
        } else {
            current_block = 11626999923138678822;
        }
    }
    match current_block {
        12497913735442871383 => {
            let mut saved_errno_0: libc::c_int = *__errno_location();
            *mtail = 0 as *mut mount_entry;
            while !mount_list.is_null() {
                me = (*mount_list).me_next;
                free_mount_entry(mount_list);
                mount_list = me;
            }
            *__errno_location() = saved_errno_0;
            return 0 as *mut mount_entry;
        }
        _ => {
            *mtail = 0 as *mut mount_entry;
            return mount_list;
        }
    };
}
pub unsafe extern "C" fn free_mount_entry(mut me: *mut mount_entry) {
    rpl_free((*me).me_devname as *mut libc::c_void);
    rpl_free((*me).me_mountdir as *mut libc::c_void);
    rpl_free((*me).me_mntroot as *mut libc::c_void);
    if (*me).me_type_malloced() != 0 {
        rpl_free((*me).me_type as *mut libc::c_void);
    }
    rpl_free(me as *mut libc::c_void);
}
