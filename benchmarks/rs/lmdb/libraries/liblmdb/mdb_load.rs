use ::libc;
extern "C" {
    pub type _IO_wide_data;
    pub type _IO_codecvt;
    pub type _IO_marker;
    pub type MDB_env;
    pub type MDB_txn;
    pub type MDB_cursor;
    static mut stdin: *mut FILE;
    static mut stderr: *mut FILE;
    fn freopen(
        __filename: *const libc::c_char,
        __modes: *const libc::c_char,
        __stream: *mut FILE,
    ) -> *mut FILE;
    fn fprintf(_: *mut FILE, _: *const libc::c_char, _: ...) -> libc::c_int;
    fn printf(_: *const libc::c_char, _: ...) -> libc::c_int;
    fn sscanf(_: *const libc::c_char, _: *const libc::c_char, _: ...) -> libc::c_int;
    fn fgetc(__stream: *mut FILE) -> libc::c_int;
    fn fgets(
        __s: *mut libc::c_char,
        __n: libc::c_int,
        __stream: *mut FILE,
    ) -> *mut libc::c_char;
    fn strtol(
        _: *const libc::c_char,
        _: *mut *mut libc::c_char,
        _: libc::c_int,
    ) -> libc::c_long;
    fn malloc(_: libc::c_ulong) -> *mut libc::c_void;
    fn realloc(_: *mut libc::c_void, _: libc::c_ulong) -> *mut libc::c_void;
    fn free(__ptr: *mut libc::c_void);
    fn exit(_: libc::c_int) -> !;
    fn __errno_location() -> *mut libc::c_int;
    fn memcpy(
        _: *mut libc::c_void,
        _: *const libc::c_void,
        _: libc::c_ulong,
    ) -> *mut libc::c_void;
    fn memcmp(
        _: *const libc::c_void,
        _: *const libc::c_void,
        _: libc::c_ulong,
    ) -> libc::c_int;
    fn memchr(
        _: *const libc::c_void,
        _: libc::c_int,
        _: libc::c_ulong,
    ) -> *mut libc::c_void;
    fn strncmp(
        _: *const libc::c_char,
        _: *const libc::c_char,
        _: libc::c_ulong,
    ) -> libc::c_int;
    fn strdup(_: *const libc::c_char) -> *mut libc::c_char;
    fn strlen(_: *const libc::c_char) -> libc::c_ulong;
    fn strerror(_: libc::c_int) -> *mut libc::c_char;
    fn __ctype_b_loc() -> *mut *const libc::c_ushort;
    static mut optarg: *mut libc::c_char;
    static mut optind: libc::c_int;
    fn getopt(
        ___argc: libc::c_int,
        ___argv: *const *mut libc::c_char,
        __shortopts: *const libc::c_char,
    ) -> libc::c_int;
    fn mdb_strerror(err: libc::c_int) -> *mut libc::c_char;
    fn mdb_env_create(env: *mut *mut MDB_env) -> libc::c_int;
    fn mdb_env_open(
        env: *mut MDB_env,
        path: *const libc::c_char,
        flags_0: libc::c_uint,
        mode_0: mdb_mode_t,
    ) -> libc::c_int;
    fn mdb_env_close(env: *mut MDB_env);
    fn mdb_env_set_mapsize(env: *mut MDB_env, size: mdb_size_t) -> libc::c_int;
    fn mdb_env_set_maxreaders(env: *mut MDB_env, readers: libc::c_uint) -> libc::c_int;
    fn mdb_env_set_maxdbs(env: *mut MDB_env, dbs: MDB_dbi) -> libc::c_int;
    fn mdb_env_get_maxkeysize(env: *mut MDB_env) -> libc::c_int;
    fn mdb_txn_begin(
        env: *mut MDB_env,
        parent: *mut MDB_txn,
        flags_0: libc::c_uint,
        txn: *mut *mut MDB_txn,
    ) -> libc::c_int;
    fn mdb_txn_commit(txn: *mut MDB_txn) -> libc::c_int;
    fn mdb_txn_abort(txn: *mut MDB_txn);
    fn mdb_dbi_open(
        txn: *mut MDB_txn,
        name: *const libc::c_char,
        flags_0: libc::c_uint,
        dbi: *mut MDB_dbi,
    ) -> libc::c_int;
    fn mdb_dbi_close(env: *mut MDB_env, dbi: MDB_dbi);
    fn mdb_set_compare(
        txn: *mut MDB_txn,
        dbi: MDB_dbi,
        cmp: Option::<MDB_cmp_func>,
    ) -> libc::c_int;
    fn mdb_set_dupsort(
        txn: *mut MDB_txn,
        dbi: MDB_dbi,
        cmp: Option::<MDB_cmp_func>,
    ) -> libc::c_int;
    fn mdb_cursor_open(
        txn: *mut MDB_txn,
        dbi: MDB_dbi,
        cursor: *mut *mut MDB_cursor,
    ) -> libc::c_int;
    fn mdb_cursor_get(
        cursor: *mut MDB_cursor,
        key: *mut MDB_val,
        data: *mut MDB_val,
        op: MDB_cursor_op,
    ) -> libc::c_int;
    fn mdb_cursor_put(
        cursor: *mut MDB_cursor,
        key: *mut MDB_val,
        data: *mut MDB_val,
        flags_0: libc::c_uint,
    ) -> libc::c_int;
}
pub type size_t = libc::c_ulong;
pub type __mode_t = libc::c_uint;
pub type __off_t = libc::c_long;
pub type __off64_t = libc::c_long;
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
pub type mdb_mode_t = mode_t;
pub type mdb_size_t = size_t;
pub type MDB_dbi = libc::c_uint;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct MDB_val {
    pub mv_size: size_t,
    pub mv_data: *mut libc::c_void,
}
pub type MDB_cmp_func = unsafe extern "C" fn(
    *const MDB_val,
    *const MDB_val,
) -> libc::c_int;
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
    pub len: libc::c_int,
}
#[inline]
unsafe extern "C" fn atoi(mut __nptr: *const libc::c_char) -> libc::c_int {
    return strtol(
        __nptr,
        0 as *mut libc::c_void as *mut *mut libc::c_char,
        10 as libc::c_int,
    ) as libc::c_int;
}
static mut mode: libc::c_int = 0;
static mut subname: *mut libc::c_char = 0 as *const libc::c_char as *mut libc::c_char;
static mut lineno: mdb_size_t = 0;
static mut version: libc::c_int = 0;
static mut flags: libc::c_int = 0;
static mut prog: *mut libc::c_char = 0 as *const libc::c_char as *mut libc::c_char;
static mut Eof: libc::c_int = 0;
static mut info: MDB_envinfo = MDB_envinfo {
    me_mapaddr: 0 as *const libc::c_void as *mut libc::c_void,
    me_mapsize: 0,
    me_last_pgno: 0,
    me_last_txnid: 0,
    me_maxreaders: 0,
    me_numreaders: 0,
};
static mut kbuf: MDB_val = MDB_val {
    mv_size: 0,
    mv_data: 0 as *const libc::c_void as *mut libc::c_void,
};
static mut dbuf: MDB_val = MDB_val {
    mv_size: 0,
    mv_data: 0 as *const libc::c_void as *mut libc::c_void,
};
static mut k0buf: MDB_val = MDB_val {
    mv_size: 0,
    mv_data: 0 as *const libc::c_void as *mut libc::c_void,
};
pub static mut dbflags: [flagbit; 7] = [flagbit {
    bit: 0,
    name: 0 as *mut libc::c_char,
    len: 0,
}; 7];
unsafe extern "C" fn readhdr() {
    let mut ptr: *mut libc::c_char = 0 as *mut libc::c_char;
    flags = 0 as libc::c_int;
    while !(fgets(dbuf.mv_data as *mut libc::c_char, dbuf.mv_size as libc::c_int, stdin))
        .is_null()
    {
        lineno = lineno.wrapping_add(1);
        lineno;
        if strncmp(
            dbuf.mv_data as *const libc::c_char,
            b"VERSION=\0" as *const u8 as *const libc::c_char,
            (::std::mem::size_of::<[libc::c_char; 9]>() as libc::c_ulong)
                .wrapping_sub(1 as libc::c_int as libc::c_ulong),
        ) == 0
        {
            version = atoi(
                (dbuf.mv_data as *mut libc::c_char)
                    .offset(
                        (::std::mem::size_of::<[libc::c_char; 9]>() as libc::c_ulong)
                            .wrapping_sub(1 as libc::c_int as libc::c_ulong) as isize,
                    ),
            );
            if version > 3 as libc::c_int {
                fprintf(
                    stderr,
                    b"%s: line %zu: unsupported VERSION %d\n\0" as *const u8
                        as *const libc::c_char,
                    prog,
                    lineno,
                    version,
                );
                exit(1 as libc::c_int);
            }
        } else {
            if strncmp(
                dbuf.mv_data as *const libc::c_char,
                b"HEADER=END\0" as *const u8 as *const libc::c_char,
                (::std::mem::size_of::<[libc::c_char; 11]>() as libc::c_ulong)
                    .wrapping_sub(1 as libc::c_int as libc::c_ulong),
            ) == 0
            {
                break;
            }
            if strncmp(
                dbuf.mv_data as *const libc::c_char,
                b"format=\0" as *const u8 as *const libc::c_char,
                (::std::mem::size_of::<[libc::c_char; 8]>() as libc::c_ulong)
                    .wrapping_sub(1 as libc::c_int as libc::c_ulong),
            ) == 0
            {
                if strncmp(
                    (dbuf.mv_data as *mut libc::c_char)
                        .offset(
                            (::std::mem::size_of::<[libc::c_char; 8]>() as libc::c_ulong)
                                .wrapping_sub(1 as libc::c_int as libc::c_ulong) as isize,
                        ),
                    b"print\0" as *const u8 as *const libc::c_char,
                    (::std::mem::size_of::<[libc::c_char; 6]>() as libc::c_ulong)
                        .wrapping_sub(1 as libc::c_int as libc::c_ulong),
                ) == 0
                {
                    mode |= 1 as libc::c_int;
                } else if strncmp(
                    (dbuf.mv_data as *mut libc::c_char)
                        .offset(
                            (::std::mem::size_of::<[libc::c_char; 8]>() as libc::c_ulong)
                                .wrapping_sub(1 as libc::c_int as libc::c_ulong) as isize,
                        ),
                    b"bytevalue\0" as *const u8 as *const libc::c_char,
                    (::std::mem::size_of::<[libc::c_char; 10]>() as libc::c_ulong)
                        .wrapping_sub(1 as libc::c_int as libc::c_ulong),
                ) != 0
                {
                    fprintf(
                        stderr,
                        b"%s: line %zu: unsupported FORMAT %s\n\0" as *const u8
                            as *const libc::c_char,
                        prog,
                        lineno,
                        (dbuf.mv_data as *mut libc::c_char)
                            .offset(
                                (::std::mem::size_of::<[libc::c_char; 8]>()
                                    as libc::c_ulong)
                                    .wrapping_sub(1 as libc::c_int as libc::c_ulong) as isize,
                            ),
                    );
                    exit(1 as libc::c_int);
                }
            } else if strncmp(
                dbuf.mv_data as *const libc::c_char,
                b"database=\0" as *const u8 as *const libc::c_char,
                (::std::mem::size_of::<[libc::c_char; 10]>() as libc::c_ulong)
                    .wrapping_sub(1 as libc::c_int as libc::c_ulong),
            ) == 0
            {
                ptr = memchr(dbuf.mv_data, '\n' as i32, dbuf.mv_size)
                    as *mut libc::c_char;
                if !ptr.is_null() {
                    *ptr = '\0' as i32 as libc::c_char;
                }
                if !subname.is_null() {
                    free(subname as *mut libc::c_void);
                }
                subname = strdup(
                    (dbuf.mv_data as *mut libc::c_char)
                        .offset(
                            (::std::mem::size_of::<[libc::c_char; 10]>()
                                as libc::c_ulong)
                                .wrapping_sub(1 as libc::c_int as libc::c_ulong) as isize,
                        ),
                );
            } else if strncmp(
                dbuf.mv_data as *const libc::c_char,
                b"type=\0" as *const u8 as *const libc::c_char,
                (::std::mem::size_of::<[libc::c_char; 6]>() as libc::c_ulong)
                    .wrapping_sub(1 as libc::c_int as libc::c_ulong),
            ) == 0
            {
                if strncmp(
                    (dbuf.mv_data as *mut libc::c_char)
                        .offset(
                            (::std::mem::size_of::<[libc::c_char; 6]>() as libc::c_ulong)
                                .wrapping_sub(1 as libc::c_int as libc::c_ulong) as isize,
                        ),
                    b"btree\0" as *const u8 as *const libc::c_char,
                    (::std::mem::size_of::<[libc::c_char; 6]>() as libc::c_ulong)
                        .wrapping_sub(1 as libc::c_int as libc::c_ulong),
                ) != 0
                {
                    fprintf(
                        stderr,
                        b"%s: line %zu: unsupported type %s\n\0" as *const u8
                            as *const libc::c_char,
                        prog,
                        lineno,
                        (dbuf.mv_data as *mut libc::c_char)
                            .offset(
                                (::std::mem::size_of::<[libc::c_char; 6]>()
                                    as libc::c_ulong)
                                    .wrapping_sub(1 as libc::c_int as libc::c_ulong) as isize,
                            ),
                    );
                    exit(1 as libc::c_int);
                }
            } else if strncmp(
                dbuf.mv_data as *const libc::c_char,
                b"mapaddr=\0" as *const u8 as *const libc::c_char,
                (::std::mem::size_of::<[libc::c_char; 9]>() as libc::c_ulong)
                    .wrapping_sub(1 as libc::c_int as libc::c_ulong),
            ) == 0
            {
                let mut i: libc::c_int = 0;
                ptr = memchr(dbuf.mv_data, '\n' as i32, dbuf.mv_size)
                    as *mut libc::c_char;
                if !ptr.is_null() {
                    *ptr = '\0' as i32 as libc::c_char;
                }
                i = sscanf(
                    (dbuf.mv_data as *mut libc::c_char)
                        .offset(
                            (::std::mem::size_of::<[libc::c_char; 9]>() as libc::c_ulong)
                                .wrapping_sub(1 as libc::c_int as libc::c_ulong) as isize,
                        ),
                    b"%p\0" as *const u8 as *const libc::c_char,
                    &mut info.me_mapaddr as *mut *mut libc::c_void,
                );
                if i != 1 as libc::c_int {
                    fprintf(
                        stderr,
                        b"%s: line %zu: invalid mapaddr %s\n\0" as *const u8
                            as *const libc::c_char,
                        prog,
                        lineno,
                        (dbuf.mv_data as *mut libc::c_char)
                            .offset(
                                (::std::mem::size_of::<[libc::c_char; 9]>()
                                    as libc::c_ulong)
                                    .wrapping_sub(1 as libc::c_int as libc::c_ulong) as isize,
                            ),
                    );
                    exit(1 as libc::c_int);
                }
            } else if strncmp(
                dbuf.mv_data as *const libc::c_char,
                b"mapsize=\0" as *const u8 as *const libc::c_char,
                (::std::mem::size_of::<[libc::c_char; 9]>() as libc::c_ulong)
                    .wrapping_sub(1 as libc::c_int as libc::c_ulong),
            ) == 0
            {
                let mut i_0: libc::c_int = 0;
                ptr = memchr(dbuf.mv_data, '\n' as i32, dbuf.mv_size)
                    as *mut libc::c_char;
                if !ptr.is_null() {
                    *ptr = '\0' as i32 as libc::c_char;
                }
                i_0 = sscanf(
                    (dbuf.mv_data as *mut libc::c_char)
                        .offset(
                            (::std::mem::size_of::<[libc::c_char; 9]>() as libc::c_ulong)
                                .wrapping_sub(1 as libc::c_int as libc::c_ulong) as isize,
                        ),
                    b"%zu\0" as *const u8 as *const libc::c_char,
                    &mut info.me_mapsize as *mut mdb_size_t,
                );
                if i_0 != 1 as libc::c_int {
                    fprintf(
                        stderr,
                        b"%s: line %zu: invalid mapsize %s\n\0" as *const u8
                            as *const libc::c_char,
                        prog,
                        lineno,
                        (dbuf.mv_data as *mut libc::c_char)
                            .offset(
                                (::std::mem::size_of::<[libc::c_char; 9]>()
                                    as libc::c_ulong)
                                    .wrapping_sub(1 as libc::c_int as libc::c_ulong) as isize,
                            ),
                    );
                    exit(1 as libc::c_int);
                }
            } else if strncmp(
                dbuf.mv_data as *const libc::c_char,
                b"maxreaders=\0" as *const u8 as *const libc::c_char,
                (::std::mem::size_of::<[libc::c_char; 12]>() as libc::c_ulong)
                    .wrapping_sub(1 as libc::c_int as libc::c_ulong),
            ) == 0
            {
                let mut i_1: libc::c_int = 0;
                ptr = memchr(dbuf.mv_data, '\n' as i32, dbuf.mv_size)
                    as *mut libc::c_char;
                if !ptr.is_null() {
                    *ptr = '\0' as i32 as libc::c_char;
                }
                i_1 = sscanf(
                    (dbuf.mv_data as *mut libc::c_char)
                        .offset(
                            (::std::mem::size_of::<[libc::c_char; 12]>()
                                as libc::c_ulong)
                                .wrapping_sub(1 as libc::c_int as libc::c_ulong) as isize,
                        ),
                    b"%u\0" as *const u8 as *const libc::c_char,
                    &mut info.me_maxreaders as *mut libc::c_uint,
                );
                if i_1 != 1 as libc::c_int {
                    fprintf(
                        stderr,
                        b"%s: line %zu: invalid maxreaders %s\n\0" as *const u8
                            as *const libc::c_char,
                        prog,
                        lineno,
                        (dbuf.mv_data as *mut libc::c_char)
                            .offset(
                                (::std::mem::size_of::<[libc::c_char; 12]>()
                                    as libc::c_ulong)
                                    .wrapping_sub(1 as libc::c_int as libc::c_ulong) as isize,
                            ),
                    );
                    exit(1 as libc::c_int);
                }
            } else {
                let mut i_2: libc::c_int = 0;
                i_2 = 0 as libc::c_int;
                while dbflags[i_2 as usize].bit != 0 {
                    if strncmp(
                        dbuf.mv_data as *const libc::c_char,
                        dbflags[i_2 as usize].name,
                        dbflags[i_2 as usize].len as libc::c_ulong,
                    ) == 0
                        && *(dbuf.mv_data as *mut libc::c_char)
                            .offset(dbflags[i_2 as usize].len as isize) as libc::c_int
                            == '=' as i32
                    {
                        flags |= dbflags[i_2 as usize].bit;
                        break;
                    } else {
                        i_2 += 1;
                        i_2;
                    }
                }
                if dbflags[i_2 as usize].bit == 0 {
                    ptr = memchr(dbuf.mv_data, '=' as i32, dbuf.mv_size)
                        as *mut libc::c_char;
                    if ptr.is_null() {
                        fprintf(
                            stderr,
                            b"%s: line %zu: unexpected format\n\0" as *const u8
                                as *const libc::c_char,
                            prog,
                            lineno,
                        );
                        exit(1 as libc::c_int);
                    } else {
                        *ptr = '\0' as i32 as libc::c_char;
                        fprintf(
                            stderr,
                            b"%s: line %zu: unrecognized keyword ignored: %s\n\0"
                                as *const u8 as *const libc::c_char,
                            prog,
                            lineno,
                            dbuf.mv_data as *mut libc::c_char,
                        );
                    }
                }
            }
        }
    }
}
unsafe extern "C" fn badend() {
    fprintf(
        stderr,
        b"%s: line %zu: unexpected end of input\n\0" as *const u8 as *const libc::c_char,
        prog,
        lineno,
    );
}
unsafe extern "C" fn unhex(mut c2: *mut libc::c_uchar) -> libc::c_int {
    let mut x: libc::c_int = 0;
    let mut c: libc::c_int = 0;
    let fresh0 = c2;
    c2 = c2.offset(1);
    x = *fresh0 as libc::c_int & 0x4f as libc::c_int;
    if x & 0x40 as libc::c_int != 0 {
        x -= 55 as libc::c_int;
    }
    c = x << 4 as libc::c_int;
    x = *c2 as libc::c_int & 0x4f as libc::c_int;
    if x & 0x40 as libc::c_int != 0 {
        x -= 55 as libc::c_int;
    }
    c |= x;
    return c;
}
unsafe extern "C" fn readline(
    mut out: *mut MDB_val,
    mut buf: *mut MDB_val,
) -> libc::c_int {
    let mut c1: *mut libc::c_uchar = 0 as *mut libc::c_uchar;
    let mut c2: *mut libc::c_uchar = 0 as *mut libc::c_uchar;
    let mut end: *mut libc::c_uchar = 0 as *mut libc::c_uchar;
    let mut len: size_t = 0;
    let mut l2: size_t = 0;
    let mut c: libc::c_int = 0;
    if mode & 2 as libc::c_int == 0 {
        c = fgetc(stdin);
        if c == -(1 as libc::c_int) {
            Eof = 1 as libc::c_int;
            return -(1 as libc::c_int);
        }
        if c != ' ' as i32 {
            lineno = lineno.wrapping_add(1);
            lineno;
            if !(fgets(
                (*buf).mv_data as *mut libc::c_char,
                (*buf).mv_size as libc::c_int,
                stdin,
            ))
                .is_null()
            {
                if c == 'D' as i32
                    && strncmp(
                        (*buf).mv_data as *const libc::c_char,
                        b"ATA=END\0" as *const u8 as *const libc::c_char,
                        (::std::mem::size_of::<[libc::c_char; 8]>() as libc::c_ulong)
                            .wrapping_sub(1 as libc::c_int as libc::c_ulong),
                    ) == 0
                {
                    return -(1 as libc::c_int);
                }
            }
            Eof = 1 as libc::c_int;
            badend();
            return -(1 as libc::c_int);
        }
    }
    if (fgets((*buf).mv_data as *mut libc::c_char, (*buf).mv_size as libc::c_int, stdin))
        .is_null()
    {
        Eof = 1 as libc::c_int;
        return -(1 as libc::c_int);
    }
    lineno = lineno.wrapping_add(1);
    lineno;
    c1 = (*buf).mv_data as *mut libc::c_uchar;
    len = strlen(c1 as *mut libc::c_char);
    l2 = len;
    while *c1.offset(len.wrapping_sub(1 as libc::c_int as libc::c_ulong) as isize)
        as libc::c_int != '\n' as i32
    {
        (*buf)
            .mv_data = realloc(
            (*buf).mv_data,
            ((*buf).mv_size).wrapping_mul(2 as libc::c_int as libc::c_ulong),
        );
        if ((*buf).mv_data).is_null() {
            Eof = 1 as libc::c_int;
            fprintf(
                stderr,
                b"%s: line %zu: out of memory, line too long\n\0" as *const u8
                    as *const libc::c_char,
                prog,
                lineno,
            );
            return -(1 as libc::c_int);
        }
        c1 = (*buf).mv_data as *mut libc::c_uchar;
        c1 = c1.offset(l2 as isize);
        if (fgets(
            c1 as *mut libc::c_char,
            ((*buf).mv_size).wrapping_add(1 as libc::c_int as libc::c_ulong)
                as libc::c_int,
            stdin,
        ))
            .is_null()
        {
            Eof = 1 as libc::c_int;
            badend();
            return -(1 as libc::c_int);
        }
        (*buf)
            .mv_size = ((*buf).mv_size as libc::c_ulong)
            .wrapping_mul(2 as libc::c_int as libc::c_ulong) as size_t as size_t;
        len = strlen(c1 as *mut libc::c_char);
        l2 = (l2 as libc::c_ulong).wrapping_add(len) as size_t as size_t;
    }
    c2 = (*buf).mv_data as *mut libc::c_uchar;
    c1 = c2;
    len = l2;
    len = len.wrapping_sub(1);
    *c1.offset(len as isize) = '\0' as i32 as libc::c_uchar;
    end = c1.offset(len as isize);
    if mode & 1 as libc::c_int != 0 {
        while c2 < end {
            if *c2 as libc::c_int == '\\' as i32 {
                if *c2.offset(1 as libc::c_int as isize) as libc::c_int == '\\' as i32 {
                    let fresh1 = c1;
                    c1 = c1.offset(1);
                    *fresh1 = *c2;
                } else {
                    if c2.offset(3 as libc::c_int as isize) > end
                        || *(*__ctype_b_loc())
                            .offset(
                                *c2.offset(1 as libc::c_int as isize) as libc::c_int
                                    as isize,
                            ) as libc::c_int
                            & _ISxdigit as libc::c_int as libc::c_ushort as libc::c_int
                            == 0
                        || *(*__ctype_b_loc())
                            .offset(
                                *c2.offset(2 as libc::c_int as isize) as libc::c_int
                                    as isize,
                            ) as libc::c_int
                            & _ISxdigit as libc::c_int as libc::c_ushort as libc::c_int
                            == 0
                    {
                        Eof = 1 as libc::c_int;
                        badend();
                        return -(1 as libc::c_int);
                    }
                    c2 = c2.offset(1);
                    let fresh2 = c1;
                    c1 = c1.offset(1);
                    *fresh2 = unhex(c2) as libc::c_uchar;
                }
                c2 = c2.offset(2 as libc::c_int as isize);
            } else {
                let fresh3 = c2;
                c2 = c2.offset(1);
                let fresh4 = c1;
                c1 = c1.offset(1);
                *fresh4 = *fresh3;
            }
        }
    } else {
        if len & 1 as libc::c_int as libc::c_ulong != 0 {
            Eof = 1 as libc::c_int;
            badend();
            return -(1 as libc::c_int);
        }
        while c2 < end {
            if *(*__ctype_b_loc()).offset(*c2 as libc::c_int as isize) as libc::c_int
                & _ISxdigit as libc::c_int as libc::c_ushort as libc::c_int == 0
                || *(*__ctype_b_loc())
                    .offset(
                        *c2.offset(1 as libc::c_int as isize) as libc::c_int as isize,
                    ) as libc::c_int
                    & _ISxdigit as libc::c_int as libc::c_ushort as libc::c_int == 0
            {
                Eof = 1 as libc::c_int;
                badend();
                return -(1 as libc::c_int);
            }
            let fresh5 = c1;
            c1 = c1.offset(1);
            *fresh5 = unhex(c2) as libc::c_uchar;
            c2 = c2.offset(2 as libc::c_int as isize);
        }
    }
    (*out).mv_data = (*buf).mv_data;
    c2 = (*out).mv_data as *mut libc::c_uchar;
    (*out).mv_size = c1.offset_from(c2) as libc::c_long as size_t;
    return 0 as libc::c_int;
}
unsafe extern "C" fn usage() {
    fprintf(
        stderr,
        b"usage: %s [-V] [-a] [-f input] [-n] [-s name] [-N] [-T] dbpath\n\0"
            as *const u8 as *const libc::c_char,
        prog,
    );
    exit(1 as libc::c_int);
}
unsafe extern "C" fn greater(
    mut a: *const MDB_val,
    mut b: *const MDB_val,
) -> libc::c_int {
    return 1 as libc::c_int;
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
    let mut mc: *mut MDB_cursor = 0 as *mut MDB_cursor;
    let mut dbi: MDB_dbi = 0;
    let mut envname: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut envflags: libc::c_int = 0x10000 as libc::c_int;
    let mut putflags: libc::c_int = 0 as libc::c_int;
    let mut dohdr: libc::c_int = 0 as libc::c_int;
    let mut append: libc::c_int = 0 as libc::c_int;
    let mut prevk: MDB_val = MDB_val {
        mv_size: 0,
        mv_data: 0 as *const libc::c_void as *mut libc::c_void,
    };
    prog = *argv.offset(0 as libc::c_int as isize);
    if argc < 2 as libc::c_int {
        usage();
    }
    loop {
        i = getopt(
            argc,
            argv as *const *mut libc::c_char,
            b"af:ns:NTV\0" as *const u8 as *const libc::c_char,
        );
        if !(i != -(1 as libc::c_int)) {
            break;
        }
        match i {
            86 => {
                printf(
                    b"%s\n\0" as *const u8 as *const libc::c_char,
                    b"LMDB 0.9.70: (December 19, 2015)\0" as *const u8
                        as *const libc::c_char,
                );
                exit(0 as libc::c_int);
            }
            97 => {
                append = 1 as libc::c_int;
            }
            102 => {
                if (freopen(optarg, b"r\0" as *const u8 as *const libc::c_char, stdin))
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
            }
            110 => {
                envflags |= 0x4000 as libc::c_int;
            }
            115 => {
                subname = strdup(optarg);
            }
            78 => {
                putflags = 0x10 as libc::c_int | 0x20 as libc::c_int;
            }
            84 => {
                mode |= 2 as libc::c_int | 1 as libc::c_int;
            }
            _ => {
                usage();
            }
        }
    }
    if optind != argc - 1 as libc::c_int {
        usage();
    }
    dbuf.mv_size = 4096 as libc::c_int as size_t;
    dbuf.mv_data = malloc(dbuf.mv_size);
    if mode & 2 as libc::c_int == 0 {
        readhdr();
    }
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
    mdb_env_set_maxdbs(env, 2 as libc::c_int as MDB_dbi);
    if info.me_maxreaders != 0 {
        mdb_env_set_maxreaders(env, info.me_maxreaders);
    }
    if info.me_mapsize != 0 {
        mdb_env_set_mapsize(env, info.me_mapsize);
    }
    if !(info.me_mapaddr).is_null() {
        envflags |= 0x1 as libc::c_int;
    }
    rc = mdb_env_open(
        env,
        envname,
        envflags as libc::c_uint,
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
        kbuf
            .mv_size = (mdb_env_get_maxkeysize(env) * 2 as libc::c_int
            + 2 as libc::c_int) as size_t;
        kbuf
            .mv_data = malloc(
            (kbuf.mv_size).wrapping_mul(2 as libc::c_int as libc::c_ulong),
        );
        k0buf.mv_size = kbuf.mv_size;
        k0buf
            .mv_data = (kbuf.mv_data as *mut libc::c_char).offset(kbuf.mv_size as isize)
            as *mut libc::c_void;
        prevk.mv_data = k0buf.mv_data;
        's_215: loop {
            if !(Eof == 0) {
                current_block = 2279025794374861465;
                break;
            }
            let mut key: MDB_val = MDB_val {
                mv_size: 0,
                mv_data: 0 as *const libc::c_void as *mut libc::c_void,
            };
            let mut data: MDB_val = MDB_val {
                mv_size: 0,
                mv_data: 0 as *const libc::c_void as *mut libc::c_void,
            };
            let mut batch: libc::c_int = 0 as libc::c_int;
            let mut appflag: libc::c_int = 0;
            if dohdr == 0 {
                dohdr = 1 as libc::c_int;
            } else if mode & 2 as libc::c_int == 0 {
                readhdr();
            }
            rc = mdb_txn_begin(
                env,
                0 as *mut MDB_txn,
                0 as libc::c_int as libc::c_uint,
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
                current_block = 5854648343751297114;
                break;
            } else {
                rc = mdb_dbi_open(
                    txn,
                    subname,
                    (flags | 0x40000 as libc::c_int) as libc::c_uint,
                    &mut dbi,
                );
                if rc != 0 {
                    fprintf(
                        stderr,
                        b"mdb_open failed, error %d %s\n\0" as *const u8
                            as *const libc::c_char,
                        rc,
                        mdb_strerror(rc),
                    );
                    current_block = 2279025794374861465;
                    break;
                } else {
                    prevk.mv_size = 0 as libc::c_int as size_t;
                    if append != 0 {
                        mdb_set_compare(
                            txn,
                            dbi,
                            Some(
                                greater
                                    as unsafe extern "C" fn(
                                        *const MDB_val,
                                        *const MDB_val,
                                    ) -> libc::c_int,
                            ),
                        );
                        if flags & 0x4 as libc::c_int != 0 {
                            mdb_set_dupsort(
                                txn,
                                dbi,
                                Some(
                                    greater
                                        as unsafe extern "C" fn(
                                            *const MDB_val,
                                            *const MDB_val,
                                        ) -> libc::c_int,
                                ),
                            );
                        }
                    }
                    rc = mdb_cursor_open(txn, dbi, &mut mc);
                    if rc != 0 {
                        fprintf(
                            stderr,
                            b"mdb_cursor_open failed, error %d %s\n\0" as *const u8
                                as *const libc::c_char,
                            rc,
                            mdb_strerror(rc),
                        );
                        current_block = 2279025794374861465;
                        break;
                    } else {
                        loop {
                            rc = readline(&mut key, &mut kbuf);
                            if rc != 0 {
                                break;
                            }
                            rc = readline(&mut data, &mut dbuf);
                            if rc != 0 {
                                fprintf(
                                    stderr,
                                    b"%s: line %zu: failed to read key value\n\0" as *const u8
                                        as *const libc::c_char,
                                    prog,
                                    lineno,
                                );
                                current_block = 2279025794374861465;
                                break 's_215;
                            } else {
                                if append != 0 {
                                    appflag = 0x20000 as libc::c_int;
                                    if flags & 0x4 as libc::c_int != 0 {
                                        if prevk.mv_size == key.mv_size
                                            && memcmp(prevk.mv_data, key.mv_data, key.mv_size) == 0
                                        {
                                            appflag = 0x40 as libc::c_int | 0x40000 as libc::c_int;
                                        } else {
                                            memcpy(prevk.mv_data, key.mv_data, key.mv_size);
                                            prevk.mv_size = key.mv_size;
                                        }
                                    }
                                } else {
                                    appflag = 0 as libc::c_int;
                                }
                                rc = mdb_cursor_put(
                                    mc,
                                    &mut key,
                                    &mut data,
                                    (putflags | appflag) as libc::c_uint,
                                );
                                if rc == -(30799 as libc::c_int) && putflags != 0 {
                                    continue;
                                }
                                if rc != 0 {
                                    fprintf(
                                        stderr,
                                        b"mdb_cursor_put failed, error %d %s\n\0" as *const u8
                                            as *const libc::c_char,
                                        rc,
                                        mdb_strerror(rc),
                                    );
                                    current_block = 2279025794374861465;
                                    break 's_215;
                                } else {
                                    batch += 1;
                                    batch;
                                    if !(batch == 100 as libc::c_int) {
                                        continue;
                                    }
                                    rc = mdb_txn_commit(txn);
                                    if rc != 0 {
                                        fprintf(
                                            stderr,
                                            b"%s: line %zu: txn_commit: %s\n\0" as *const u8
                                                as *const libc::c_char,
                                            prog,
                                            lineno,
                                            mdb_strerror(rc),
                                        );
                                        current_block = 5854648343751297114;
                                        break 's_215;
                                    } else {
                                        rc = mdb_txn_begin(
                                            env,
                                            0 as *mut MDB_txn,
                                            0 as libc::c_int as libc::c_uint,
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
                                            current_block = 5854648343751297114;
                                            break 's_215;
                                        } else {
                                            rc = mdb_cursor_open(txn, dbi, &mut mc);
                                            if rc != 0 {
                                                fprintf(
                                                    stderr,
                                                    b"mdb_cursor_open failed, error %d %s\n\0" as *const u8
                                                        as *const libc::c_char,
                                                    rc,
                                                    mdb_strerror(rc),
                                                );
                                                current_block = 2279025794374861465;
                                                break 's_215;
                                            } else {
                                                if appflag & 0x40000 as libc::c_int != 0 {
                                                    let mut k: MDB_val = MDB_val {
                                                        mv_size: 0,
                                                        mv_data: 0 as *const libc::c_void as *mut libc::c_void,
                                                    };
                                                    let mut d: MDB_val = MDB_val {
                                                        mv_size: 0,
                                                        mv_data: 0 as *const libc::c_void as *mut libc::c_void,
                                                    };
                                                    mdb_cursor_get(mc, &mut k, &mut d, MDB_LAST);
                                                }
                                                batch = 0 as libc::c_int;
                                            }
                                        }
                                    }
                                }
                            }
                        }
                        rc = mdb_txn_commit(txn);
                        txn = 0 as *mut MDB_txn;
                        if rc != 0 {
                            fprintf(
                                stderr,
                                b"%s: line %zu: txn_commit: %s\n\0" as *const u8
                                    as *const libc::c_char,
                                prog,
                                lineno,
                                mdb_strerror(rc),
                            );
                            current_block = 5854648343751297114;
                            break;
                        } else {
                            mdb_dbi_close(env, dbi);
                        }
                    }
                }
            }
        }
        match current_block {
            5854648343751297114 => {}
            _ => {
                mdb_txn_abort(txn);
            }
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
unsafe extern "C" fn run_static_initializers() {
    dbflags = [
        {
            let mut init = flagbit {
                bit: 0x2 as libc::c_int,
                name: b"reversekey\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                len: (::std::mem::size_of::<[libc::c_char; 11]>() as libc::c_ulong)
                    .wrapping_sub(1 as libc::c_int as libc::c_ulong) as libc::c_int,
            };
            init
        },
        {
            let mut init = flagbit {
                bit: 0x4 as libc::c_int,
                name: b"dupsort\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                len: (::std::mem::size_of::<[libc::c_char; 8]>() as libc::c_ulong)
                    .wrapping_sub(1 as libc::c_int as libc::c_ulong) as libc::c_int,
            };
            init
        },
        {
            let mut init = flagbit {
                bit: 0x8 as libc::c_int,
                name: b"integerkey\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                len: (::std::mem::size_of::<[libc::c_char; 11]>() as libc::c_ulong)
                    .wrapping_sub(1 as libc::c_int as libc::c_ulong) as libc::c_int,
            };
            init
        },
        {
            let mut init = flagbit {
                bit: 0x10 as libc::c_int,
                name: b"dupfixed\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                len: (::std::mem::size_of::<[libc::c_char; 9]>() as libc::c_ulong)
                    .wrapping_sub(1 as libc::c_int as libc::c_ulong) as libc::c_int,
            };
            init
        },
        {
            let mut init = flagbit {
                bit: 0x20 as libc::c_int,
                name: b"integerdup\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                len: (::std::mem::size_of::<[libc::c_char; 11]>() as libc::c_ulong)
                    .wrapping_sub(1 as libc::c_int as libc::c_ulong) as libc::c_int,
            };
            init
        },
        {
            let mut init = flagbit {
                bit: 0x40 as libc::c_int,
                name: b"reversedup\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                len: (::std::mem::size_of::<[libc::c_char; 11]>() as libc::c_ulong)
                    .wrapping_sub(1 as libc::c_int as libc::c_ulong) as libc::c_int,
            };
            init
        },
        {
            let mut init = flagbit {
                bit: 0 as libc::c_int,
                name: 0 as *mut libc::c_char,
                len: 0 as libc::c_int,
            };
            init
        },
    ];
}
#[used]
#[cfg_attr(target_os = "linux", link_section = ".init_array")]
#[cfg_attr(target_os = "windows", link_section = ".CRT$XIB")]
#[cfg_attr(target_os = "macos", link_section = "__DATA,__mod_init_func")]
static INIT_ARRAY: [unsafe extern "C" fn(); 1] = [run_static_initializers];
