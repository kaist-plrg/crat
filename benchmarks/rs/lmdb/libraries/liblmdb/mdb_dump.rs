use ::libc;
extern "C" {
    pub type _IO_wide_data;
    pub type _IO_codecvt;
    pub type _IO_marker;
    pub type MDB_env;
    pub type MDB_txn;
    pub type MDB_cursor;
    static mut stdout: *mut FILE;
    static mut stderr: *mut FILE;
    fn freopen(
        __filename: *const libc::c_char,
        __modes: *const libc::c_char,
        __stream: *mut FILE,
    ) -> *mut FILE;
    fn fprintf(_: *mut FILE, _: *const libc::c_char, _: ...) -> libc::c_int;
    fn printf(_: *const libc::c_char, _: ...) -> libc::c_int;
    fn putc(__c: libc::c_int, __stream: *mut FILE) -> libc::c_int;
    fn __errno_location() -> *mut libc::c_int;
    fn malloc(_: libc::c_ulong) -> *mut libc::c_void;
    fn free(__ptr: *mut libc::c_void);
    fn exit(_: libc::c_int) -> !;
    fn memcpy(
        _: *mut libc::c_void,
        _: *const libc::c_void,
        _: libc::c_ulong,
    ) -> *mut libc::c_void;
    fn memchr(
        _: *const libc::c_void,
        _: libc::c_int,
        _: libc::c_ulong,
    ) -> *mut libc::c_void;
    fn strerror(_: libc::c_int) -> *mut libc::c_char;
    fn __ctype_b_loc() -> *mut *const libc::c_ushort;
    fn getopt(
        ___argc: libc::c_int,
        ___argv: *const *mut libc::c_char,
        __shortopts: *const libc::c_char,
    ) -> libc::c_int;
    static mut optarg: *mut libc::c_char;
    static mut optind: libc::c_int;
    fn signal(__sig: libc::c_int, __handler: __sighandler_t) -> __sighandler_t;
    fn mdb_strerror(err: libc::c_int) -> *mut libc::c_char;
    fn mdb_env_create(env: *mut *mut MDB_env) -> libc::c_int;
    fn mdb_env_open(
        env: *mut MDB_env,
        path: *const libc::c_char,
        flags: libc::c_uint,
        mode_0: mdb_mode_t,
    ) -> libc::c_int;
    fn mdb_env_info(env: *mut MDB_env, stat: *mut MDB_envinfo) -> libc::c_int;
    fn mdb_env_close(env: *mut MDB_env);
    fn mdb_env_set_maxdbs(env: *mut MDB_env, dbs: MDB_dbi) -> libc::c_int;
    fn mdb_txn_begin(
        env: *mut MDB_env,
        parent: *mut MDB_txn,
        flags: libc::c_uint,
        txn: *mut *mut MDB_txn,
    ) -> libc::c_int;
    fn mdb_txn_env(txn: *mut MDB_txn) -> *mut MDB_env;
    fn mdb_txn_abort(txn: *mut MDB_txn);
    fn mdb_dbi_open(
        txn: *mut MDB_txn,
        name: *const libc::c_char,
        flags: libc::c_uint,
        dbi: *mut MDB_dbi,
    ) -> libc::c_int;
    fn mdb_stat(txn: *mut MDB_txn, dbi: MDB_dbi, stat: *mut MDB_stat) -> libc::c_int;
    fn mdb_dbi_flags(
        txn: *mut MDB_txn,
        dbi: MDB_dbi,
        flags: *mut libc::c_uint,
    ) -> libc::c_int;
    fn mdb_dbi_close(env: *mut MDB_env, dbi: MDB_dbi);
    fn mdb_cursor_open(
        txn: *mut MDB_txn,
        dbi: MDB_dbi,
        cursor: *mut *mut MDB_cursor,
    ) -> libc::c_int;
    fn mdb_cursor_close(cursor: *mut MDB_cursor);
    fn mdb_cursor_get(
        cursor: *mut MDB_cursor,
        key: *mut MDB_val,
        data: *mut MDB_val,
        op: MDB_cursor_op,
    ) -> libc::c_int;
}
pub type size_t = libc::c_ulong;
pub type __mode_t = libc::c_uint;
pub type __off_t = libc::c_long;
pub type __off64_t = libc::c_long;
pub type __sig_atomic_t = libc::c_int;
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
pub type mode_t = __mode_t;
pub type C2RustUnnamed = libc::c_uint;
pub const _ISalnum: C2RustUnnamed = 8;
pub const _ISpunct: C2RustUnnamed = 4;
pub const _IScntrl: C2RustUnnamed = 2;
pub const _ISblank: C2RustUnnamed = 1;
pub const _ISgraph: C2RustUnnamed = 32768;
pub const _ISprint: C2RustUnnamed = 16384;
pub const _ISspace: C2RustUnnamed = 8192;
pub const _ISxdigit: C2RustUnnamed = 4096;
pub const _ISdigit: C2RustUnnamed = 2048;
pub const _ISalpha: C2RustUnnamed = 1024;
pub const _ISlower: C2RustUnnamed = 512;
pub const _ISupper: C2RustUnnamed = 256;
pub type sig_atomic_t = __sig_atomic_t;
pub type __sighandler_t = Option::<unsafe extern "C" fn(libc::c_int) -> ()>;
pub type mdb_mode_t = mode_t;
pub type mdb_size_t = size_t;
pub type MDB_dbi = libc::c_uint;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct MDB_val {
    pub mv_size: size_t,
    pub mv_data: *mut libc::c_void,
}
pub type MDB_cursor_op = libc::c_uint;
pub const MDB_PREV_MULTIPLE: MDB_cursor_op = 18;
pub const MDB_SET_RANGE: MDB_cursor_op = 17;
pub const MDB_SET_KEY: MDB_cursor_op = 16;
pub const MDB_SET: MDB_cursor_op = 15;
pub const MDB_PREV_NODUP: MDB_cursor_op = 14;
pub const MDB_PREV_DUP: MDB_cursor_op = 13;
pub const MDB_PREV: MDB_cursor_op = 12;
pub const MDB_NEXT_NODUP: MDB_cursor_op = 11;
pub const MDB_NEXT_MULTIPLE: MDB_cursor_op = 10;
pub const MDB_NEXT_DUP: MDB_cursor_op = 9;
pub const MDB_NEXT: MDB_cursor_op = 8;
pub const MDB_LAST_DUP: MDB_cursor_op = 7;
pub const MDB_LAST: MDB_cursor_op = 6;
pub const MDB_GET_MULTIPLE: MDB_cursor_op = 5;
pub const MDB_GET_CURRENT: MDB_cursor_op = 4;
pub const MDB_GET_BOTH_RANGE: MDB_cursor_op = 3;
pub const MDB_GET_BOTH: MDB_cursor_op = 2;
pub const MDB_FIRST_DUP: MDB_cursor_op = 1;
pub const MDB_FIRST: MDB_cursor_op = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct MDB_stat {
    pub ms_psize: libc::c_uint,
    pub ms_depth: libc::c_uint,
    pub ms_branch_pages: mdb_size_t,
    pub ms_leaf_pages: mdb_size_t,
    pub ms_overflow_pages: mdb_size_t,
    pub ms_entries: mdb_size_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct MDB_envinfo {
    pub me_mapaddr: *mut libc::c_void,
    pub me_mapsize: mdb_size_t,
    pub me_last_pgno: mdb_size_t,
    pub me_last_txnid: mdb_size_t,
    pub me_maxreaders: libc::c_uint,
    pub me_numreaders: libc::c_uint,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct flagbit {
    pub bit: libc::c_int,
    pub name: *mut libc::c_char,
}
#[inline]
unsafe extern "C" fn putchar(mut __c: libc::c_int) -> libc::c_int {
    return putc(__c, stdout);
}
static mut mode: libc::c_int = 0;
pub static mut dbflags: [flagbit; 7] = [
    {
        let mut init = flagbit {
            bit: 0x2 as libc::c_int,
            name: b"reversekey\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = flagbit {
            bit: 0x4 as libc::c_int,
            name: b"dupsort\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = flagbit {
            bit: 0x8 as libc::c_int,
            name: b"integerkey\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = flagbit {
            bit: 0x10 as libc::c_int,
            name: b"dupfixed\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = flagbit {
            bit: 0x20 as libc::c_int,
            name: b"integerdup\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = flagbit {
            bit: 0x40 as libc::c_int,
            name: b"reversedup\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = flagbit {
            bit: 0 as libc::c_int,
            name: 0 as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
];
static mut gotsig: sig_atomic_t = 0;
unsafe extern "C" fn dumpsig(mut sig: libc::c_int) {
    ::std::ptr::write_volatile(&mut gotsig as *mut sig_atomic_t, 1 as libc::c_int);
}
static mut hexc: [libc::c_char; 17] = unsafe {
    *::std::mem::transmute::<&[u8; 17], &[libc::c_char; 17]>(b"0123456789abcdef\0")
};
unsafe extern "C" fn hex(mut c: libc::c_uchar) {
    putchar(hexc[(c as libc::c_int >> 4 as libc::c_int) as usize] as libc::c_int);
    putchar(hexc[(c as libc::c_int & 0xf as libc::c_int) as usize] as libc::c_int);
}
unsafe extern "C" fn text(mut v: *mut MDB_val) {
    let mut c: *mut libc::c_uchar = 0 as *mut libc::c_uchar;
    let mut end: *mut libc::c_uchar = 0 as *mut libc::c_uchar;
    putchar(' ' as i32);
    c = (*v).mv_data as *mut libc::c_uchar;
    end = c.offset((*v).mv_size as isize);
    while c < end {
        if *(*__ctype_b_loc()).offset(*c as libc::c_int as isize) as libc::c_int
            & _ISprint as libc::c_int as libc::c_ushort as libc::c_int != 0
        {
            if *c as libc::c_int == '\\' as i32 {
                putchar('\\' as i32);
            }
            putchar(*c as libc::c_int);
        } else {
            putchar('\\' as i32);
            hex(*c);
        }
        c = c.offset(1);
        c;
    }
    putchar('\n' as i32);
}
unsafe extern "C" fn byte(mut v: *mut MDB_val) {
    let mut c: *mut libc::c_uchar = 0 as *mut libc::c_uchar;
    let mut end: *mut libc::c_uchar = 0 as *mut libc::c_uchar;
    putchar(' ' as i32);
    c = (*v).mv_data as *mut libc::c_uchar;
    end = c.offset((*v).mv_size as isize);
    while c < end {
        let fresh0 = c;
        c = c.offset(1);
        hex(*fresh0);
    }
    putchar('\n' as i32);
}
unsafe extern "C" fn dumpit(
    mut txn: *mut MDB_txn,
    mut dbi: MDB_dbi,
    mut name: *mut libc::c_char,
) -> libc::c_int {
    let mut mc: *mut MDB_cursor = 0 as *mut MDB_cursor;
    let mut ms: MDB_stat = MDB_stat {
        ms_psize: 0,
        ms_depth: 0,
        ms_branch_pages: 0,
        ms_leaf_pages: 0,
        ms_overflow_pages: 0,
        ms_entries: 0,
    };
    let mut key: MDB_val = MDB_val {
        mv_size: 0,
        mv_data: 0 as *mut libc::c_void,
    };
    let mut data: MDB_val = MDB_val {
        mv_size: 0,
        mv_data: 0 as *mut libc::c_void,
    };
    let mut info: MDB_envinfo = MDB_envinfo {
        me_mapaddr: 0 as *mut libc::c_void,
        me_mapsize: 0,
        me_last_pgno: 0,
        me_last_txnid: 0,
        me_maxreaders: 0,
        me_numreaders: 0,
    };
    let mut flags: libc::c_uint = 0;
    let mut rc: libc::c_int = 0;
    let mut i: libc::c_int = 0;
    rc = mdb_dbi_flags(txn, dbi, &mut flags);
    if rc != 0 {
        return rc;
    }
    rc = mdb_stat(txn, dbi, &mut ms);
    if rc != 0 {
        return rc;
    }
    rc = mdb_env_info(mdb_txn_env(txn), &mut info);
    if rc != 0 {
        return rc;
    }
    printf(b"VERSION=3\n\0" as *const u8 as *const libc::c_char);
    printf(
        b"format=%s\n\0" as *const u8 as *const libc::c_char,
        if mode & 1 as libc::c_int != 0 {
            b"print\0" as *const u8 as *const libc::c_char
        } else {
            b"bytevalue\0" as *const u8 as *const libc::c_char
        },
    );
    if !name.is_null() {
        printf(b"database=%s\n\0" as *const u8 as *const libc::c_char, name);
    }
    printf(b"type=btree\n\0" as *const u8 as *const libc::c_char);
    printf(b"mapsize=%zu\n\0" as *const u8 as *const libc::c_char, info.me_mapsize);
    if !(info.me_mapaddr).is_null() {
        printf(b"mapaddr=%p\n\0" as *const u8 as *const libc::c_char, info.me_mapaddr);
    }
    printf(b"maxreaders=%u\n\0" as *const u8 as *const libc::c_char, info.me_maxreaders);
    if flags & 0x4 as libc::c_int as libc::c_uint != 0 {
        printf(b"duplicates=1\n\0" as *const u8 as *const libc::c_char);
    }
    i = 0 as libc::c_int;
    while dbflags[i as usize].bit != 0 {
        if flags & dbflags[i as usize].bit as libc::c_uint != 0 {
            printf(
                b"%s=1\n\0" as *const u8 as *const libc::c_char,
                dbflags[i as usize].name,
            );
        }
        i += 1;
        i;
    }
    printf(b"db_pagesize=%d\n\0" as *const u8 as *const libc::c_char, ms.ms_psize);
    printf(b"HEADER=END\n\0" as *const u8 as *const libc::c_char);
    rc = mdb_cursor_open(txn, dbi, &mut mc);
    if rc != 0 {
        return rc;
    }
    loop {
        rc = (mdb_cursor_get(mc, &mut key, &mut data, MDB_NEXT) == 0 as libc::c_int)
            as libc::c_int;
        if !(rc != 0) {
            break;
        }
        if gotsig != 0 {
            rc = 4 as libc::c_int;
            break;
        } else if mode & 1 as libc::c_int != 0 {
            text(&mut key);
            text(&mut data);
        } else {
            byte(&mut key);
            byte(&mut data);
        }
    }
    printf(b"DATA=END\n\0" as *const u8 as *const libc::c_char);
    if rc == -(30798 as libc::c_int) {
        rc = 0 as libc::c_int;
    }
    return rc;
}
unsafe extern "C" fn usage(mut prog: *mut libc::c_char) {
    fprintf(
        stderr,
        b"usage: %s [-V] [-f output] [-l] [-n] [-p] [-v] [-a|-s subdb] dbpath\n\0"
            as *const u8 as *const libc::c_char,
        prog,
    );
    exit(1 as libc::c_int);
}
unsafe fn main_0(
    mut argc: libc::c_int,
    mut argv: *mut *mut libc::c_char,
) -> libc::c_int {
    let mut current_block: u64;
    let mut i: libc::c_int = 0;
    let mut rc: libc::c_int = 0;
    let mut env: *mut MDB_env = 0 as *mut MDB_env;
    let mut txn: *mut MDB_txn = 0 as *mut MDB_txn;
    let mut dbi: MDB_dbi = 0;
    let mut prog: *mut libc::c_char = *argv.offset(0 as libc::c_int as isize);
    let mut envname: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut subname: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut alldbs: libc::c_int = 0 as libc::c_int;
    let mut envflags: libc::c_int = 0 as libc::c_int;
    let mut list: libc::c_int = 0 as libc::c_int;
    if argc < 2 as libc::c_int {
        usage(prog);
    }
    loop {
        i = getopt(
            argc,
            argv as *const *mut libc::c_char,
            b"af:lnps:vV\0" as *const u8 as *const libc::c_char,
        );
        if !(i != -(1 as libc::c_int)) {
            break;
        }
        let mut current_block_20: u64;
        match i {
            86 => {
                printf(
                    b"%s\n\0" as *const u8 as *const libc::c_char,
                    b"LMDB 0.9.70: (December 19, 2015)\0" as *const u8
                        as *const libc::c_char,
                );
                exit(0 as libc::c_int);
            }
            108 => {
                list = 1 as libc::c_int;
                current_block_20 = 5684663562884776962;
            }
            97 => {
                current_block_20 = 5684663562884776962;
            }
            102 => {
                if (freopen(optarg, b"w\0" as *const u8 as *const libc::c_char, stdout))
                    .is_null()
                {
                    fprintf(
                        stderr,
                        b"%s: %s: reopen: %s\n\0" as *const u8 as *const libc::c_char,
                        prog,
                        optarg,
                        strerror(*__errno_location()),
                    );
                    exit(1 as libc::c_int);
                }
                current_block_20 = 7172762164747879670;
            }
            110 => {
                envflags |= 0x4000 as libc::c_int;
                current_block_20 = 7172762164747879670;
            }
            118 => {
                envflags |= 0x2000000 as libc::c_int;
                current_block_20 = 7172762164747879670;
            }
            112 => {
                mode |= 1 as libc::c_int;
                current_block_20 = 7172762164747879670;
            }
            115 => {
                if alldbs != 0 {
                    usage(prog);
                }
                subname = optarg;
                current_block_20 = 7172762164747879670;
            }
            _ => {
                usage(prog);
                current_block_20 = 7172762164747879670;
            }
        }
        match current_block_20 {
            5684663562884776962 => {
                if !subname.is_null() {
                    usage(prog);
                }
                alldbs += 1;
                alldbs;
            }
            _ => {}
        }
    }
    if optind != argc - 1 as libc::c_int {
        usage(prog);
    }
    signal(13 as libc::c_int, Some(dumpsig as unsafe extern "C" fn(libc::c_int) -> ()));
    signal(1 as libc::c_int, Some(dumpsig as unsafe extern "C" fn(libc::c_int) -> ()));
    signal(2 as libc::c_int, Some(dumpsig as unsafe extern "C" fn(libc::c_int) -> ()));
    signal(15 as libc::c_int, Some(dumpsig as unsafe extern "C" fn(libc::c_int) -> ()));
    envname = *argv.offset(optind as isize);
    rc = mdb_env_create(&mut env);
    if rc != 0 {
        fprintf(
            stderr,
            b"mdb_env_create failed, error %d %s\n\0" as *const u8
                as *const libc::c_char,
            rc,
            mdb_strerror(rc),
        );
        return 1 as libc::c_int;
    }
    if alldbs != 0 || !subname.is_null() {
        mdb_env_set_maxdbs(env, 2 as libc::c_int as MDB_dbi);
    }
    rc = mdb_env_open(
        env,
        envname,
        (envflags | 0x20000 as libc::c_int) as libc::c_uint,
        0o664 as libc::c_int as mdb_mode_t,
    );
    if rc != 0 {
        fprintf(
            stderr,
            b"mdb_env_open failed, error %d %s\n\0" as *const u8 as *const libc::c_char,
            rc,
            mdb_strerror(rc),
        );
    } else {
        rc = mdb_txn_begin(
            env,
            0 as *mut MDB_txn,
            0x20000 as libc::c_int as libc::c_uint,
            &mut txn,
        );
        if rc != 0 {
            fprintf(
                stderr,
                b"mdb_txn_begin failed, error %d %s\n\0" as *const u8
                    as *const libc::c_char,
                rc,
                mdb_strerror(rc),
            );
        } else {
            rc = mdb_dbi_open(txn, subname, 0 as libc::c_int as libc::c_uint, &mut dbi);
            if rc != 0 {
                fprintf(
                    stderr,
                    b"mdb_open failed, error %d %s\n\0" as *const u8
                        as *const libc::c_char,
                    rc,
                    mdb_strerror(rc),
                );
            } else {
                if alldbs != 0 {
                    let mut cursor: *mut MDB_cursor = 0 as *mut MDB_cursor;
                    let mut key: MDB_val = MDB_val {
                        mv_size: 0,
                        mv_data: 0 as *mut libc::c_void,
                    };
                    let mut count: libc::c_int = 0 as libc::c_int;
                    rc = mdb_cursor_open(txn, dbi, &mut cursor);
                    if rc != 0 {
                        fprintf(
                            stderr,
                            b"mdb_cursor_open failed, error %d %s\n\0" as *const u8
                                as *const libc::c_char,
                            rc,
                            mdb_strerror(rc),
                        );
                        current_block = 17248785892041839259;
                    } else {
                        loop {
                            rc = mdb_cursor_get(
                                cursor,
                                &mut key,
                                0 as *mut MDB_val,
                                MDB_NEXT_NODUP,
                            );
                            if !(rc == 0 as libc::c_int) {
                                break;
                            }
                            let mut str: *mut libc::c_char = 0 as *mut libc::c_char;
                            let mut db2: MDB_dbi = 0;
                            if !(memchr(key.mv_data, '\0' as i32, key.mv_size)).is_null()
                            {
                                continue;
                            }
                            count += 1;
                            count;
                            str = malloc(
                                (key.mv_size)
                                    .wrapping_add(1 as libc::c_int as libc::c_ulong),
                            ) as *mut libc::c_char;
                            memcpy(str as *mut libc::c_void, key.mv_data, key.mv_size);
                            *str
                                .offset(key.mv_size as isize) = '\0' as i32 as libc::c_char;
                            rc = mdb_dbi_open(
                                txn,
                                str,
                                0 as libc::c_int as libc::c_uint,
                                &mut db2,
                            );
                            if rc == 0 as libc::c_int {
                                if list != 0 {
                                    printf(b"%s\n\0" as *const u8 as *const libc::c_char, str);
                                    list += 1;
                                    list;
                                } else {
                                    rc = dumpit(txn, db2, str);
                                    if rc != 0 {
                                        break;
                                    }
                                }
                                mdb_dbi_close(env, db2);
                            }
                            free(str as *mut libc::c_void);
                            rc != 0;
                        }
                        mdb_cursor_close(cursor);
                        if count == 0 {
                            fprintf(
                                stderr,
                                b"%s: %s does not contain multiple databases\n\0"
                                    as *const u8 as *const libc::c_char,
                                prog,
                                envname,
                            );
                            rc = -(30798 as libc::c_int);
                        } else if rc == -(30798 as libc::c_int) {
                            rc = 0 as libc::c_int;
                        }
                        current_block = 7385833325316299293;
                    }
                } else {
                    rc = dumpit(txn, dbi, subname);
                    current_block = 7385833325316299293;
                }
                match current_block {
                    17248785892041839259 => {}
                    _ => {
                        if rc != 0 && rc != -(30798 as libc::c_int) {
                            fprintf(
                                stderr,
                                b"%s: %s: %s\n\0" as *const u8 as *const libc::c_char,
                                prog,
                                envname,
                                mdb_strerror(rc),
                            );
                        }
                        mdb_dbi_close(env, dbi);
                    }
                }
            }
            mdb_txn_abort(txn);
        }
    }
    mdb_env_close(env);
    return if rc != 0 { 1 as libc::c_int } else { 0 as libc::c_int };
}
pub fn main() {
    let mut args: Vec::<*mut libc::c_char> = Vec::new();
    for arg in ::std::env::args_os() {
        args.push(
            (::std::ffi::CString::new(
                ::std::os::unix::ffi::OsStrExt::as_bytes(arg.as_os_str()),
            ))
                .unwrap()
                .into_raw(),
        );
    }
    args.push(::std::ptr::null_mut());
    unsafe {
        ::std::process::exit(
            main_0(
                (args.len() - 1) as libc::c_int,
                args.as_mut_ptr() as *mut *mut libc::c_char,
            ) as i32,
        )
    }
}
