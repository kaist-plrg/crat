use ::libc;
extern "C" {
    pub type _IO_wide_data;
    pub type _IO_codecvt;
    pub type _IO_marker;
    pub type pg_conn;
    pub type pg_result;
    fn free(__ptr: *mut libc::c_void);
    fn exit(_: libc::c_int) -> !;
    fn getenv(__name: *const libc::c_char) -> *mut libc::c_char;
    fn memcpy(
        _: *mut libc::c_void,
        _: *const libc::c_void,
        _: libc::c_ulong,
    ) -> *mut libc::c_void;
    fn memset(
        _: *mut libc::c_void,
        _: libc::c_int,
        _: libc::c_ulong,
    ) -> *mut libc::c_void;
    fn strncpy(
        _: *mut libc::c_char,
        _: *const libc::c_char,
        _: libc::c_ulong,
    ) -> *mut libc::c_char;
    fn strcmp(_: *const libc::c_char, _: *const libc::c_char) -> libc::c_int;
    fn strchr(_: *const libc::c_char, _: libc::c_int) -> *mut libc::c_char;
    fn strcspn(_: *const libc::c_char, _: *const libc::c_char) -> libc::c_ulong;
    fn strlen(_: *const libc::c_char) -> libc::c_ulong;
    fn __ctype_b_loc() -> *mut *const libc::c_ushort;
    fn __ctype_tolower_loc() -> *mut *const __int32_t;
    fn __ctype_toupper_loc() -> *mut *const __int32_t;
    fn getpwuid(__uid: __uid_t) -> *mut passwd;
    fn pg_fprintf(stream: *mut FILE, fmt: *const libc::c_char, _: ...) -> libc::c_int;
    fn pg_printf(fmt: *const libc::c_char, _: ...) -> libc::c_int;
    fn fgets(
        __s: *mut libc::c_char,
        __n: libc::c_int,
        __stream: *mut FILE,
    ) -> *mut libc::c_char;
    fn fclose(__stream: *mut FILE) -> libc::c_int;
    static mut stderr: *mut FILE;
    fn geteuid() -> __uid_t;
    static mut optarg: *mut libc::c_char;
    static mut optind: libc::c_int;
    fn PQconnectdb(conninfo: *const libc::c_char) -> *mut PGconn;
    fn PQfinish(conn: *mut PGconn);
    fn PQpass(conn: *const PGconn) -> *mut libc::c_char;
    fn PQstatus(conn: *const PGconn) -> ConnStatusType;
    fn PQerrorMessage(conn: *const PGconn) -> *mut libc::c_char;
    fn PQsetnonblocking(conn: *mut PGconn, arg: libc::c_int) -> libc::c_int;
    fn initPQExpBuffer(str: PQExpBuffer);
    fn termPQExpBuffer(str: PQExpBuffer);
    fn appendPQExpBuffer(str: PQExpBuffer, fmt: *const libc::c_char, _: ...);
    fn errcode(sqlerrcode: libc::c_int) -> libc::c_int;
    fn errmsg(fmt: *const libc::c_char, _: ...) -> libc::c_int;
    static mut PROGRAM_NAME: *const libc::c_char;
    static mut PROGRAM_VERSION: *const libc::c_char;
    static mut PROGRAM_URL: *const libc::c_char;
    static mut PROGRAM_ISSUES: *const libc::c_char;
    static mut pgut_log_level: libc::c_int;
    static mut pgut_echo: bool;
    fn pgut_init(argc: libc::c_int, argv: *mut *mut libc::c_char);
    fn pgut_connect(
        info: *const libc::c_char,
        prompt: YesNo,
        elevel: libc::c_int,
    ) -> *mut PGconn;
    fn pgut_disconnect(conn: *mut PGconn);
    fn pgut_execute(
        conn: *mut PGconn,
        query: *const libc::c_char,
        nParams: libc::c_int,
        params: *mut *const libc::c_char,
    ) -> *mut PGresult;
    fn pgut_execute_elevel(
        conn: *mut PGconn,
        query: *const libc::c_char,
        nParams: libc::c_int,
        params: *mut *const libc::c_char,
        elevel: libc::c_int,
    ) -> *mut PGresult;
    fn pgut_command(
        conn: *mut PGconn,
        query: *const libc::c_char,
        nParams: libc::c_int,
        params: *mut *const libc::c_char,
    ) -> ExecStatusType;
    fn pgut_malloc(size: size_t) -> *mut libc::c_void;
    fn pgut_strdup(str: *const libc::c_char) -> *mut libc::c_char;
    fn pgut_fopen(path: *const libc::c_char, mode: *const libc::c_char) -> *mut FILE;
    fn elog(elevel: libc::c_int, fmt: *const libc::c_char, _: ...);
    fn parse_elevel(value: *const libc::c_char) -> libc::c_int;
    fn errcode_errno() -> libc::c_int;
    fn pgut_errstart(elevel: libc::c_int) -> bool;
    fn pgut_errfinish(dummy: libc::c_int, _: ...);
    fn parse_bool(value: *const libc::c_char, result: *mut bool) -> bool;
    fn parse_int32(value: *const libc::c_char, result: *mut int32) -> bool;
    fn parse_uint32(value: *const libc::c_char, result: *mut uint32) -> bool;
    fn parse_int64(value: *const libc::c_char, result: *mut int64) -> bool;
    fn parse_uint64(value: *const libc::c_char, result: *mut uint64) -> bool;
    fn parse_time(value: *const libc::c_char, time: *mut time_t) -> bool;
    fn simple_string_list_append(list: *mut SimpleStringList, val: *const libc::c_char);
    fn pgut_help(details: bool);
    fn getopt_long(
        ___argc: libc::c_int,
        ___argv: *const *mut libc::c_char,
        __shortopts: *const libc::c_char,
        __longopts: *const option,
        __longind: *mut libc::c_int,
    ) -> libc::c_int;
}
pub type size_t = libc::c_ulong;
pub type __int32_t = libc::c_int;
pub type __uid_t = libc::c_uint;
pub type __gid_t = libc::c_uint;
pub type __off_t = libc::c_long;
pub type __off64_t = libc::c_long;
pub type __time_t = libc::c_long;
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
pub type time_t = __time_t;
pub type int32 = libc::c_int;
pub type uint32 = libc::c_uint;
pub type int64 = libc::c_long;
pub type uint64 = libc::c_ulong;
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct passwd {
    pub pw_name: *mut libc::c_char,
    pub pw_passwd: *mut libc::c_char,
    pub pw_uid: __uid_t,
    pub pw_gid: __gid_t,
    pub pw_gecos: *mut libc::c_char,
    pub pw_dir: *mut libc::c_char,
    pub pw_shell: *mut libc::c_char,
}
pub type ConnStatusType = libc::c_uint;
pub const CONNECTION_GSS_STARTUP: ConnStatusType = 11;
pub const CONNECTION_CONSUME: ConnStatusType = 10;
pub const CONNECTION_CHECK_WRITABLE: ConnStatusType = 9;
pub const CONNECTION_NEEDED: ConnStatusType = 8;
pub const CONNECTION_SSL_STARTUP: ConnStatusType = 7;
pub const CONNECTION_SETENV: ConnStatusType = 6;
pub const CONNECTION_AUTH_OK: ConnStatusType = 5;
pub const CONNECTION_AWAITING_RESPONSE: ConnStatusType = 4;
pub const CONNECTION_MADE: ConnStatusType = 3;
pub const CONNECTION_STARTED: ConnStatusType = 2;
pub const CONNECTION_BAD: ConnStatusType = 1;
pub const CONNECTION_OK: ConnStatusType = 0;
pub type ExecStatusType = libc::c_uint;
pub const PGRES_SINGLE_TUPLE: ExecStatusType = 9;
pub const PGRES_COPY_BOTH: ExecStatusType = 8;
pub const PGRES_FATAL_ERROR: ExecStatusType = 7;
pub const PGRES_NONFATAL_ERROR: ExecStatusType = 6;
pub const PGRES_BAD_RESPONSE: ExecStatusType = 5;
pub const PGRES_COPY_IN: ExecStatusType = 4;
pub const PGRES_COPY_OUT: ExecStatusType = 3;
pub const PGRES_TUPLES_OK: ExecStatusType = 2;
pub const PGRES_COMMAND_OK: ExecStatusType = 1;
pub const PGRES_EMPTY_QUERY: ExecStatusType = 0;
pub type PGconn = pg_conn;
pub type PGresult = pg_result;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct PQExpBufferData {
    pub data: *mut libc::c_char,
    pub len: size_t,
    pub maxlen: size_t,
}
pub type PQExpBuffer = *mut PQExpBufferData;
pub type YesNo = libc::c_uint;
pub const YES: YesNo = 2;
pub const NO: YesNo = 1;
pub const DEFAULT: YesNo = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct SimpleStringListCell {
    pub next: *mut SimpleStringListCell,
    pub val: [libc::c_char; 1],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct SimpleStringList {
    pub head: *mut SimpleStringListCell,
    pub tail: *mut SimpleStringListCell,
}
pub type pgut_optsrc = libc::c_uint;
pub const SOURCE_CONST: pgut_optsrc = 4;
pub const SOURCE_CMDLINE: pgut_optsrc = 3;
pub const SOURCE_FILE: pgut_optsrc = 2;
pub const SOURCE_ENV: pgut_optsrc = 1;
pub const SOURCE_DEFAULT: pgut_optsrc = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct pgut_option {
    pub type_0: libc::c_char,
    pub sname: libc::c_char,
    pub lname: *const libc::c_char,
    pub var: *mut libc::c_void,
    pub allowed: pgut_optsrc,
    pub source: pgut_optsrc,
}
pub type pgut_optfn = Option::<
    unsafe extern "C" fn(*mut pgut_option, *const libc::c_char) -> (),
>;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct worker_conns {
    pub max_num_workers: libc::c_int,
    pub num_workers: libc::c_int,
    pub conns: *mut *mut PGconn,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct option {
    pub name: *const libc::c_char,
    pub has_arg: libc::c_int,
    pub flag: *mut libc::c_int,
    pub val: libc::c_int,
}
#[inline]
unsafe extern "C" fn tolower(mut __c: libc::c_int) -> libc::c_int {
    return if __c >= -(128 as libc::c_int) && __c < 256 as libc::c_int {
        *(*__ctype_tolower_loc()).offset(__c as isize)
    } else {
        __c
    };
}
#[inline]
unsafe extern "C" fn toupper(mut __c: libc::c_int) -> libc::c_int {
    return if __c >= -(128 as libc::c_int) && __c < 256 as libc::c_int {
        *(*__ctype_toupper_loc()).offset(__c as isize)
    } else {
        __c
    };
}
pub static mut dbname: *mut libc::c_char = 0 as *const libc::c_char as *mut libc::c_char;
pub static mut host: *mut libc::c_char = 0 as *const libc::c_char as *mut libc::c_char;
pub static mut port: *mut libc::c_char = 0 as *const libc::c_char as *mut libc::c_char;
pub static mut username: *mut libc::c_char = 0 as *const libc::c_char
    as *mut libc::c_char;
pub static mut password: *mut libc::c_char = 0 as *const libc::c_char
    as *mut libc::c_char;
pub static mut prompt_password: YesNo = DEFAULT;
pub static mut connection: *mut PGconn = 0 as *const PGconn as *mut PGconn;
pub static mut conn2: *mut PGconn = 0 as *const PGconn as *mut PGconn;
pub static mut workers: worker_conns = {
    let mut init = worker_conns {
        max_num_workers: 0,
        num_workers: 0 as libc::c_int,
        conns: 0 as *const *mut PGconn as *mut *mut PGconn,
    };
    init
};
pub unsafe extern "C" fn setup_workers(mut num_workers: libc::c_int) {
    let mut buf: PQExpBufferData = PQExpBufferData {
        data: 0 as *mut libc::c_char,
        len: 0,
        maxlen: 0,
    };
    let mut i: libc::c_int = 0;
    let mut conn: *mut PGconn = 0 as *mut PGconn;
    elog(
        13 as libc::c_int,
        b"In setup_workers(), target num_workers = %d\0" as *const u8
            as *const libc::c_char,
        num_workers,
    );
    if num_workers > 1 as libc::c_int && num_workers > workers.num_workers {
        initPQExpBuffer(&mut buf);
        if !dbname.is_null()
            && *dbname.offset(0 as libc::c_int as isize) as libc::c_int != 0
        {
            appendPQExpBuffer(
                &mut buf as *mut PQExpBufferData,
                b"dbname=%s \0" as *const u8 as *const libc::c_char,
                dbname,
            );
        }
        if !host.is_null() && *host.offset(0 as libc::c_int as isize) as libc::c_int != 0
        {
            appendPQExpBuffer(
                &mut buf as *mut PQExpBufferData,
                b"host=%s \0" as *const u8 as *const libc::c_char,
                host,
            );
        }
        if !port.is_null() && *port.offset(0 as libc::c_int as isize) as libc::c_int != 0
        {
            appendPQExpBuffer(
                &mut buf as *mut PQExpBufferData,
                b"port=%s \0" as *const u8 as *const libc::c_char,
                port,
            );
        }
        if !username.is_null()
            && *username.offset(0 as libc::c_int as isize) as libc::c_int != 0
        {
            appendPQExpBuffer(
                &mut buf as *mut PQExpBufferData,
                b"user=%s \0" as *const u8 as *const libc::c_char,
                username,
            );
        }
        if !password.is_null()
            && *password.offset(0 as libc::c_int as isize) as libc::c_int != 0
        {
            appendPQExpBuffer(
                &mut buf as *mut PQExpBufferData,
                b"password=%s \0" as *const u8 as *const libc::c_char,
                password,
            );
        }
        if (workers.conns).is_null() {
            elog(
                18 as libc::c_int,
                b"Setting up workers.conns\0" as *const u8 as *const libc::c_char,
            );
            workers
                .conns = pgut_malloc(
                (::std::mem::size_of::<*mut PGconn>() as libc::c_ulong)
                    .wrapping_mul(num_workers as libc::c_ulong),
            ) as *mut *mut PGconn;
        } else {
            elog(
                20 as libc::c_int,
                b"TODO: Implement pool resizing.\0" as *const u8 as *const libc::c_char,
            );
        }
        i = 0 as libc::c_int;
        while i < num_workers {
            elog(
                13 as libc::c_int,
                b"Setting up worker conn %d\0" as *const u8 as *const libc::c_char,
                i,
            );
            conn = PQconnectdb(buf.data);
            if PQstatus(conn) as libc::c_uint
                == CONNECTION_OK as libc::c_int as libc::c_uint
            {
                let ref mut fresh0 = *(workers.conns).offset(i as isize);
                *fresh0 = conn;
                pgut_command(
                    conn,
                    b"SET search_path TO pg_catalog, pg_temp, public\0" as *const u8
                        as *const libc::c_char,
                    0 as libc::c_int,
                    0 as *mut *const libc::c_char,
                );
                if PQsetnonblocking(
                    *(workers.conns).offset(i as isize),
                    1 as libc::c_int,
                ) != 0
                {
                    elog(
                        20 as libc::c_int,
                        b"Unable to set worker connection %d non-blocking.\0"
                            as *const u8 as *const libc::c_char,
                        i,
                    );
                }
                i += 1;
                i;
            } else {
                elog(
                    19 as libc::c_int,
                    b"Unable to set up worker conn #%d: %s\0" as *const u8
                        as *const libc::c_char,
                    i,
                    PQerrorMessage(conn),
                );
                break;
            }
        }
        workers.num_workers = i;
        termPQExpBuffer(&mut buf);
    }
}
pub unsafe extern "C" fn disconnect_workers() {
    let mut i: libc::c_int = 0;
    if workers.num_workers == 0 {
        elog(
            13 as libc::c_int,
            b"No workers to disconnect.\0" as *const u8 as *const libc::c_char,
        );
    } else {
        i = 0 as libc::c_int;
        while i < workers.num_workers {
            if !(*(workers.conns).offset(i as isize)).is_null() {
                elog(
                    13 as libc::c_int,
                    b"Disconnecting worker %d.\0" as *const u8 as *const libc::c_char,
                    i,
                );
                PQfinish(*(workers.conns).offset(i as isize));
                let ref mut fresh1 = *(workers.conns).offset(i as isize);
                *fresh1 = 0 as *mut PGconn;
            } else {
                elog(
                    18 as libc::c_int,
                    b"Worker %d already disconnected?\0" as *const u8
                        as *const libc::c_char,
                    i,
                );
            }
            i += 1;
            i;
        }
        workers.num_workers = 0 as libc::c_int;
        free(workers.conns as *mut libc::c_void);
        workers.conns = 0 as *mut *mut PGconn;
    };
}
pub unsafe extern "C" fn reconnect(mut elevel: libc::c_int) {
    let mut buf: PQExpBufferData = PQExpBufferData {
        data: 0 as *mut libc::c_char,
        len: 0,
        maxlen: 0,
    };
    let mut new_password: *mut libc::c_char = 0 as *mut libc::c_char;
    disconnect();
    initPQExpBuffer(&mut buf);
    if !dbname.is_null() && *dbname.offset(0 as libc::c_int as isize) as libc::c_int != 0
    {
        appendPQExpBuffer(
            &mut buf as *mut PQExpBufferData,
            b"dbname=%s \0" as *const u8 as *const libc::c_char,
            dbname,
        );
    }
    if !host.is_null() && *host.offset(0 as libc::c_int as isize) as libc::c_int != 0 {
        appendPQExpBuffer(
            &mut buf as *mut PQExpBufferData,
            b"host=%s \0" as *const u8 as *const libc::c_char,
            host,
        );
    }
    if !port.is_null() && *port.offset(0 as libc::c_int as isize) as libc::c_int != 0 {
        appendPQExpBuffer(
            &mut buf as *mut PQExpBufferData,
            b"port=%s \0" as *const u8 as *const libc::c_char,
            port,
        );
    }
    if !username.is_null()
        && *username.offset(0 as libc::c_int as isize) as libc::c_int != 0
    {
        appendPQExpBuffer(
            &mut buf as *mut PQExpBufferData,
            b"user=%s \0" as *const u8 as *const libc::c_char,
            username,
        );
    }
    if !password.is_null()
        && *password.offset(0 as libc::c_int as isize) as libc::c_int != 0
    {
        appendPQExpBuffer(
            &mut buf as *mut PQExpBufferData,
            b"password=%s \0" as *const u8 as *const libc::c_char,
            password,
        );
    }
    connection = pgut_connect(buf.data, prompt_password, elevel);
    conn2 = pgut_connect(buf.data, prompt_password, elevel);
    if !connection.is_null() {
        new_password = PQpass(connection);
        if !new_password.is_null()
            && *new_password.offset(0 as libc::c_int as isize) as libc::c_int != 0
            && (password.is_null() || strcmp(new_password, password) != 0 as libc::c_int)
        {
            free(password as *mut libc::c_void);
            password = pgut_strdup(new_password);
        }
    }
    termPQExpBuffer(&mut buf);
}
pub unsafe extern "C" fn disconnect() {
    if !connection.is_null() {
        pgut_disconnect(connection);
        connection = 0 as *mut PGconn;
    }
    if !conn2.is_null() {
        pgut_disconnect(conn2);
        conn2 = 0 as *mut PGconn;
    }
    disconnect_workers();
}
unsafe extern "C" fn option_from_env(mut options: *mut pgut_option) {
    let mut i: size_t = 0;
    i = 0 as libc::c_int as size_t;
    while !options.is_null() && (*options.offset(i as isize)).type_0 as libc::c_int != 0
    {
        let mut opt: *mut pgut_option = &mut *options.offset(i as isize)
            as *mut pgut_option;
        let mut name: [libc::c_char; 256] = [0; 256];
        let mut j: size_t = 0;
        let mut s: *const libc::c_char = 0 as *const libc::c_char;
        let mut value: *const libc::c_char = 0 as *const libc::c_char;
        if !((*opt).source as libc::c_uint > SOURCE_ENV as libc::c_int as libc::c_uint
            || (*opt).allowed as libc::c_uint
                == SOURCE_DEFAULT as libc::c_int as libc::c_uint
            || (*opt).allowed as libc::c_uint
                > SOURCE_ENV as libc::c_int as libc::c_uint)
        {
            s = (*opt).lname;
            j = 0 as libc::c_int as size_t;
            while *s as libc::c_int != 0
                && j
                    < (::std::mem::size_of::<[libc::c_char; 256]>() as libc::c_ulong)
                        .wrapping_div(
                            ::std::mem::size_of::<libc::c_char>() as libc::c_ulong,
                        )
                        .wrapping_sub(1 as libc::c_int as libc::c_ulong)
            {
                if !(strchr(
                    b"-_ \0" as *const u8 as *const libc::c_char,
                    *s as libc::c_int,
                ))
                    .is_null()
                {
                    name[j as usize] = '_' as i32 as libc::c_char;
                } else {
                    name[j
                        as usize] = ({
                        let mut __res: libc::c_int = 0;
                        if ::std::mem::size_of::<libc::c_char>() as libc::c_ulong
                            > 1 as libc::c_int as libc::c_ulong
                        {
                            if 0 != 0 {
                                let mut __c: libc::c_int = *s as libc::c_int;
                                __res = if __c < -(128 as libc::c_int)
                                    || __c > 255 as libc::c_int
                                {
                                    __c
                                } else {
                                    *(*__ctype_toupper_loc()).offset(__c as isize)
                                };
                            } else {
                                __res = toupper(*s as libc::c_int);
                            }
                        } else {
                            __res = *(*__ctype_toupper_loc())
                                .offset(*s as libc::c_int as isize);
                        }
                        __res
                    }) as libc::c_char;
                }
                s = s.offset(1);
                s;
                j = j.wrapping_add(1);
                j;
            }
            name[j as usize] = '\0' as i32 as libc::c_char;
            value = getenv(name.as_mut_ptr());
            if !value.is_null() {
                pgut_setopt(opt, value, SOURCE_ENV);
            }
        }
        i = i.wrapping_add(1);
        i;
    }
}
pub unsafe extern "C" fn pgut_keyeq(
    mut lhs: *const libc::c_char,
    mut rhs: *const libc::c_char,
) -> bool {
    while *lhs as libc::c_int != 0 && *rhs as libc::c_int != 0 {
        if !(strchr(b"-_ \0" as *const u8 as *const libc::c_char, *lhs as libc::c_int))
            .is_null()
        {
            if (strchr(
                b"-_ \0" as *const u8 as *const libc::c_char,
                *rhs as libc::c_int,
            ))
                .is_null()
            {
                return 0 as libc::c_int != 0;
            }
        } else if ({
            let mut __res: libc::c_int = 0;
            if ::std::mem::size_of::<libc::c_uchar>() as libc::c_ulong
                > 1 as libc::c_int as libc::c_ulong
            {
                if 0 != 0 {
                    let mut __c: libc::c_int = *lhs as libc::c_uchar as libc::c_int;
                    __res = (if __c < -(128 as libc::c_int) || __c > 255 as libc::c_int {
                        __c
                    } else {
                        *(*__ctype_tolower_loc()).offset(__c as isize)
                    });
                } else {
                    __res = tolower(*lhs as libc::c_uchar as libc::c_int);
                }
            } else {
                __res = *(*__ctype_tolower_loc())
                    .offset(*lhs as libc::c_uchar as libc::c_int as isize);
            }
            __res
        })
            != ({
                let mut __res: libc::c_int = 0;
                if ::std::mem::size_of::<libc::c_uchar>() as libc::c_ulong
                    > 1 as libc::c_int as libc::c_ulong
                {
                    if 0 != 0 {
                        let mut __c: libc::c_int = *rhs as libc::c_uchar as libc::c_int;
                        __res = (if __c < -(128 as libc::c_int)
                            || __c > 255 as libc::c_int
                        {
                            __c
                        } else {
                            *(*__ctype_tolower_loc()).offset(__c as isize)
                        });
                    } else {
                        __res = tolower(*rhs as libc::c_uchar as libc::c_int);
                    }
                } else {
                    __res = *(*__ctype_tolower_loc())
                        .offset(*rhs as libc::c_uchar as libc::c_int as isize);
                }
                __res
            })
        {
            return 0 as libc::c_int != 0
        }
        lhs = lhs.offset(1);
        lhs;
        rhs = rhs.offset(1);
        rhs;
    }
    return *lhs as libc::c_int == '\0' as i32 && *rhs as libc::c_int == '\0' as i32;
}
pub unsafe extern "C" fn pgut_setopt(
    mut opt: *mut pgut_option,
    mut optarg_0: *const libc::c_char,
    mut src: pgut_optsrc,
) {
    let mut message: *const libc::c_char = 0 as *const libc::c_char;
    if opt.is_null() {
        pg_fprintf(
            stderr,
            b"Try \"%s --help\" for more information.\n\0" as *const u8
                as *const libc::c_char,
            PROGRAM_NAME,
        );
        exit(22 as libc::c_int);
    }
    if (*opt).source as libc::c_uint > src as libc::c_uint {
        return
    } else if src as libc::c_uint >= SOURCE_CMDLINE as libc::c_int as libc::c_uint
        && (*opt).source as libc::c_uint >= src as libc::c_uint
        && (*opt).type_0 as libc::c_int != 'l' as i32
    {
        message = b"specified only once\0" as *const u8 as *const libc::c_char;
    } else {
        (*opt).source = src;
        match (*opt).type_0 as libc::c_int {
            98 | 66 => {
                if optarg_0.is_null() {
                    *((*opt).var
                        as *mut bool) = (*opt).type_0 as libc::c_int == 'b' as i32;
                    return;
                } else if parse_bool(optarg_0, (*opt).var as *mut bool) {
                    return
                }
                message = b"a boolean\0" as *const u8 as *const libc::c_char;
            }
            102 => {
                (::std::mem::transmute::<*mut libc::c_void, pgut_optfn>((*opt).var))
                    .unwrap()(opt, optarg_0);
                return;
            }
            105 => {
                if parse_int32(optarg_0, (*opt).var as *mut int32) {
                    return;
                }
                message = b"a 32bit signed integer\0" as *const u8
                    as *const libc::c_char;
            }
            108 => {
                message = b"a List\0" as *const u8 as *const libc::c_char;
                simple_string_list_append((*opt).var as *mut SimpleStringList, optarg_0);
                return;
            }
            117 => {
                if parse_uint32(optarg_0, (*opt).var as *mut uint32) {
                    return;
                }
                message = b"a 32bit unsigned integer\0" as *const u8
                    as *const libc::c_char;
            }
            73 => {
                if parse_int64(optarg_0, (*opt).var as *mut int64) {
                    return;
                }
                message = b"a 64bit signed integer\0" as *const u8
                    as *const libc::c_char;
            }
            85 => {
                if parse_uint64(optarg_0, (*opt).var as *mut uint64) {
                    return;
                }
                message = b"a 64bit unsigned integer\0" as *const u8
                    as *const libc::c_char;
            }
            115 => {
                if (*opt).source as libc::c_uint
                    != SOURCE_DEFAULT as libc::c_int as libc::c_uint
                {
                    free(*((*opt).var as *mut *mut libc::c_char) as *mut libc::c_void);
                }
                let ref mut fresh2 = *((*opt).var as *mut *mut libc::c_char);
                *fresh2 = pgut_strdup(optarg_0);
                return;
            }
            116 => {
                if parse_time(optarg_0, (*opt).var as *mut time_t) {
                    return;
                }
                message = b"a time\0" as *const u8 as *const libc::c_char;
            }
            121 | 89 => {
                if optarg_0.is_null() {
                    *((*opt).var
                        as *mut YesNo) = (if (*opt).type_0 as libc::c_int == 'y' as i32 {
                        YES as libc::c_int
                    } else {
                        NO as libc::c_int
                    }) as YesNo;
                    return;
                } else {
                    let mut value: bool = false;
                    if parse_bool(optarg_0, &mut value) {
                        *((*opt).var
                            as *mut YesNo) = (if value as libc::c_int != 0 {
                            YES as libc::c_int
                        } else {
                            NO as libc::c_int
                        }) as YesNo;
                        return;
                    }
                }
                message = b"a boolean\0" as *const u8 as *const libc::c_char;
            }
            _ => {
                if pgut_errstart(20 as libc::c_int) as libc::c_int != 0 {
                    pgut_errfinish(
                        errcode(22 as libc::c_int),
                        errmsg(
                            b"invalid option type: %c\0" as *const u8
                                as *const libc::c_char,
                            (*opt).type_0 as libc::c_int,
                        ),
                    );
                } else {};
                return;
            }
        }
    }
    if *(*__ctype_b_loc()).offset((*opt).sname as libc::c_int as isize) as libc::c_int
        & _ISprint as libc::c_int as libc::c_ushort as libc::c_int != 0
    {
        if pgut_errstart(20 as libc::c_int) as libc::c_int != 0 {
            pgut_errfinish(
                errcode(22 as libc::c_int),
                errmsg(
                    b"option -%c, --%s should be %s: '%s'\0" as *const u8
                        as *const libc::c_char,
                    (*opt).sname as libc::c_int,
                    (*opt).lname,
                    message,
                    optarg_0,
                ),
            );
        } else {};
    } else {
        if pgut_errstart(20 as libc::c_int) as libc::c_int != 0 {
            pgut_errfinish(
                errcode(22 as libc::c_int),
                errmsg(
                    b"option --%s should be %s: '%s'\0" as *const u8
                        as *const libc::c_char,
                    (*opt).lname,
                    message,
                    optarg_0,
                ),
            );
        } else {};
    };
}
pub unsafe extern "C" fn pgut_readopt(
    mut path: *const libc::c_char,
    mut options: *mut pgut_option,
    mut elevel: libc::c_int,
) {
    let mut fp: *mut FILE = 0 as *mut FILE;
    let mut buf: [libc::c_char; 1024] = [0; 1024];
    let mut key: [libc::c_char; 1024] = [0; 1024];
    let mut value: [libc::c_char; 1024] = [0; 1024];
    if options.is_null() {
        return;
    }
    fp = pgut_fopen(path, b"Rt\0" as *const u8 as *const libc::c_char);
    if fp.is_null() {
        return;
    }
    while !(fgets(
        buf.as_mut_ptr(),
        (::std::mem::size_of::<[libc::c_char; 1024]>() as libc::c_ulong)
            .wrapping_div(::std::mem::size_of::<libc::c_char>() as libc::c_ulong)
            as libc::c_int,
        fp,
    ))
        .is_null()
    {
        let mut i: size_t = 0;
        i = strlen(buf.as_mut_ptr());
        while i > 0 as libc::c_int as libc::c_ulong
            && *(*__ctype_b_loc())
                .offset(
                    buf[i.wrapping_sub(1 as libc::c_int as libc::c_ulong) as usize]
                        as libc::c_uchar as libc::c_int as isize,
                ) as libc::c_int
                & _ISspace as libc::c_int as libc::c_ushort as libc::c_int != 0
        {
            buf[i.wrapping_sub(1 as libc::c_int as libc::c_ulong)
                as usize] = '\0' as i32 as libc::c_char;
            i = i.wrapping_sub(1);
            i;
        }
        if parse_pair(
            buf.as_mut_ptr() as *const libc::c_char,
            key.as_mut_ptr(),
            value.as_mut_ptr(),
        ) {
            i = 0 as libc::c_int as size_t;
            while (*options.offset(i as isize)).type_0 != 0 {
                let mut opt: *mut pgut_option = &mut *options.offset(i as isize)
                    as *mut pgut_option;
                if pgut_keyeq(key.as_mut_ptr(), (*opt).lname) {
                    if (*opt).allowed as libc::c_uint
                        == SOURCE_DEFAULT as libc::c_int as libc::c_uint
                        || (*opt).allowed as libc::c_uint
                            > SOURCE_FILE as libc::c_int as libc::c_uint
                    {
                        elog(
                            elevel,
                            b"option %s cannot specified in file\0" as *const u8
                                as *const libc::c_char,
                            (*opt).lname,
                        );
                    } else if (*opt).source as libc::c_uint
                        <= SOURCE_FILE as libc::c_int as libc::c_uint
                    {
                        pgut_setopt(opt, value.as_mut_ptr(), SOURCE_FILE);
                    }
                    break;
                } else {
                    i = i.wrapping_add(1);
                    i;
                }
            }
            if (*options.offset(i as isize)).type_0 == 0 {
                elog(
                    elevel,
                    b"invalid option \"%s\"\0" as *const u8 as *const libc::c_char,
                    key.as_mut_ptr(),
                );
            }
        }
    }
    fclose(fp);
}
unsafe extern "C" fn skip_space(
    mut str: *const libc::c_char,
    mut line: *const libc::c_char,
) -> *const libc::c_char {
    while *(*__ctype_b_loc()).offset(*str as libc::c_uchar as libc::c_int as isize)
        as libc::c_int & _ISspace as libc::c_int as libc::c_ushort as libc::c_int != 0
    {
        str = str.offset(1);
        str;
    }
    return str;
}
unsafe extern "C" fn get_next_token(
    mut src: *const libc::c_char,
    mut dst: *mut libc::c_char,
    mut line: *const libc::c_char,
) -> *const libc::c_char {
    let mut s: *const libc::c_char = 0 as *const libc::c_char;
    let mut i: size_t = 0;
    let mut j: size_t = 0;
    s = skip_space(src, line);
    if s.is_null() {
        return 0 as *const libc::c_char;
    }
    if *s as libc::c_int == '\'' as i32 {
        s = s.offset(1);
        s;
        i = 0 as libc::c_int as size_t;
        j = 0 as libc::c_int as size_t;
        while *s.offset(i as isize) as libc::c_int != '\0' as i32 {
            if *s.offset(i as isize) as libc::c_int == '\\' as i32 {
                i = i.wrapping_add(1);
                i;
                match *s.offset(i as isize) as libc::c_int {
                    98 => {
                        *dst.offset(j as isize) = '\u{8}' as i32 as libc::c_char;
                    }
                    102 => {
                        *dst.offset(j as isize) = '\u{c}' as i32 as libc::c_char;
                    }
                    110 => {
                        *dst.offset(j as isize) = '\n' as i32 as libc::c_char;
                    }
                    114 => {
                        *dst.offset(j as isize) = '\r' as i32 as libc::c_char;
                    }
                    116 => {
                        *dst.offset(j as isize) = '\t' as i32 as libc::c_char;
                    }
                    48 | 49 | 50 | 51 | 52 | 53 | 54 | 55 => {
                        let mut k: libc::c_int = 0;
                        let mut octVal: libc::c_long = 0 as libc::c_int as libc::c_long;
                        k = 0 as libc::c_int;
                        while *s.offset(i.wrapping_add(k as libc::c_ulong) as isize)
                            as libc::c_int >= '0' as i32
                            && *s.offset(i.wrapping_add(k as libc::c_ulong) as isize)
                                as libc::c_int <= '7' as i32 && k < 3 as libc::c_int
                        {
                            octVal = (octVal << 3 as libc::c_int)
                                + (*s.offset(i.wrapping_add(k as libc::c_ulong) as isize)
                                    as libc::c_int - '0' as i32) as libc::c_long;
                            k += 1;
                            k;
                        }
                        i = (i as libc::c_ulong)
                            .wrapping_add((k - 1 as libc::c_int) as libc::c_ulong)
                            as size_t as size_t;
                        *dst.offset(j as isize) = octVal as libc::c_char;
                    }
                    _ => {
                        *dst.offset(j as isize) = *s.offset(i as isize);
                    }
                }
            } else if *s.offset(i as isize) as libc::c_int == '\'' as i32 {
                i = i.wrapping_add(1);
                i;
                if !(*s.offset(i as isize) as libc::c_int == '\'' as i32) {
                    break;
                }
                *dst.offset(j as isize) = *s.offset(i as isize);
            } else {
                *dst.offset(j as isize) = *s.offset(i as isize);
            }
            j = j.wrapping_add(1);
            j;
            i = i.wrapping_add(1);
            i;
        }
    } else {
        j = strcspn(s, b"# \n\r\t\x0B\0" as *const u8 as *const libc::c_char);
        i = j;
        memcpy(dst as *mut libc::c_void, s as *const libc::c_void, j);
    }
    *dst.offset(j as isize) = '\0' as i32 as libc::c_char;
    return s.offset(i as isize);
}
unsafe extern "C" fn parse_pair(
    mut buffer: *const libc::c_char,
    mut key: *mut libc::c_char,
    mut value: *mut libc::c_char,
) -> bool {
    let mut start: *const libc::c_char = 0 as *const libc::c_char;
    let mut end: *const libc::c_char = 0 as *const libc::c_char;
    let ref mut fresh3 = *value.offset(0 as libc::c_int as isize);
    *fresh3 = '\0' as i32 as libc::c_char;
    *key.offset(0 as libc::c_int as isize) = *fresh3;
    start = buffer;
    start = skip_space(start, buffer);
    if start.is_null() {
        return 0 as libc::c_int != 0;
    }
    end = start
        .offset(
            strcspn(start, b"=# \n\r\t\x0B\0" as *const u8 as *const libc::c_char)
                as isize,
        );
    if end.offset_from(start) as libc::c_long <= 0 as libc::c_int as libc::c_long {
        if *start as libc::c_int == '=' as i32 {
            elog(
                19 as libc::c_int,
                b"syntax error in \"%s\"\0" as *const u8 as *const libc::c_char,
                buffer,
            );
        }
        return 0 as libc::c_int != 0;
    }
    strncpy(key, start, end.offset_from(start) as libc::c_long as libc::c_ulong);
    *key
        .offset(
            end.offset_from(start) as libc::c_long as isize,
        ) = '\0' as i32 as libc::c_char;
    start = skip_space(end, buffer);
    if start.is_null() {
        return 0 as libc::c_int != 0;
    }
    if *start as libc::c_int != '=' as i32 {
        elog(
            19 as libc::c_int,
            b"syntax error in \"%s\"\0" as *const u8 as *const libc::c_char,
            buffer,
        );
        return 0 as libc::c_int != 0;
    }
    start = start.offset(1);
    start;
    end = get_next_token(start, value, buffer);
    if end.is_null() {
        return 0 as libc::c_int != 0;
    }
    start = skip_space(end, buffer);
    if start.is_null() {
        return 0 as libc::c_int != 0;
    }
    if *start as libc::c_int != '\0' as i32 && *start as libc::c_int != '#' as i32 {
        elog(
            19 as libc::c_int,
            b"syntax error in \"%s\"\0" as *const u8 as *const libc::c_char,
            buffer,
        );
        return 0 as libc::c_int != 0;
    }
    return 1 as libc::c_int != 0;
}
pub unsafe extern "C" fn execute(
    mut query: *const libc::c_char,
    mut nParams: libc::c_int,
    mut params: *mut *const libc::c_char,
) -> *mut PGresult {
    return pgut_execute(connection, query, nParams, params);
}
pub unsafe extern "C" fn execute_elevel(
    mut query: *const libc::c_char,
    mut nParams: libc::c_int,
    mut params: *mut *const libc::c_char,
    mut elevel: libc::c_int,
) -> *mut PGresult {
    return pgut_execute_elevel(connection, query, nParams, params, elevel);
}
pub unsafe extern "C" fn command(
    mut query: *const libc::c_char,
    mut nParams: libc::c_int,
    mut params: *mut *const libc::c_char,
) -> ExecStatusType {
    return pgut_command(connection, query, nParams, params);
}
unsafe extern "C" fn set_elevel(
    mut opt: *mut pgut_option,
    mut arg: *const libc::c_char,
) {
    pgut_log_level = parse_elevel(arg);
}
static mut default_options: [pgut_option; 9] = unsafe {
    [
        {
            let mut init = pgut_option {
                type_0: 'b' as i32 as libc::c_char,
                sname: 'e' as i32 as libc::c_char,
                lname: b"echo\0" as *const u8 as *const libc::c_char,
                var: &pgut_echo as *const bool as *mut bool as *mut libc::c_void,
                allowed: SOURCE_DEFAULT,
                source: SOURCE_DEFAULT,
            };
            init
        },
        {
            let mut init = pgut_option {
                type_0: 'f' as i32 as libc::c_char,
                sname: 'E' as i32 as libc::c_char,
                lname: b"elevel\0" as *const u8 as *const libc::c_char,
                var: ::std::mem::transmute::<
                    Option::<
                        unsafe extern "C" fn(*mut pgut_option, *const libc::c_char) -> (),
                    >,
                    *mut libc::c_void,
                >(
                    Some(
                        set_elevel
                            as unsafe extern "C" fn(
                                *mut pgut_option,
                                *const libc::c_char,
                            ) -> (),
                    ),
                ),
                allowed: SOURCE_DEFAULT,
                source: SOURCE_DEFAULT,
            };
            init
        },
        {
            let mut init = pgut_option {
                type_0: 's' as i32 as libc::c_char,
                sname: 'd' as i32 as libc::c_char,
                lname: b"dbname\0" as *const u8 as *const libc::c_char,
                var: &dbname as *const *mut libc::c_char as *mut *mut libc::c_char
                    as *mut libc::c_void,
                allowed: SOURCE_DEFAULT,
                source: SOURCE_DEFAULT,
            };
            init
        },
        {
            let mut init = pgut_option {
                type_0: 's' as i32 as libc::c_char,
                sname: 'h' as i32 as libc::c_char,
                lname: b"host\0" as *const u8 as *const libc::c_char,
                var: &host as *const *mut libc::c_char as *mut *mut libc::c_char
                    as *mut libc::c_void,
                allowed: SOURCE_DEFAULT,
                source: SOURCE_DEFAULT,
            };
            init
        },
        {
            let mut init = pgut_option {
                type_0: 's' as i32 as libc::c_char,
                sname: 'p' as i32 as libc::c_char,
                lname: b"port\0" as *const u8 as *const libc::c_char,
                var: &port as *const *mut libc::c_char as *mut *mut libc::c_char
                    as *mut libc::c_void,
                allowed: SOURCE_DEFAULT,
                source: SOURCE_DEFAULT,
            };
            init
        },
        {
            let mut init = pgut_option {
                type_0: 's' as i32 as libc::c_char,
                sname: 'U' as i32 as libc::c_char,
                lname: b"username\0" as *const u8 as *const libc::c_char,
                var: &username as *const *mut libc::c_char as *mut *mut libc::c_char
                    as *mut libc::c_void,
                allowed: SOURCE_DEFAULT,
                source: SOURCE_DEFAULT,
            };
            init
        },
        {
            let mut init = pgut_option {
                type_0: 'Y' as i32 as libc::c_char,
                sname: 'w' as i32 as libc::c_char,
                lname: b"no-password\0" as *const u8 as *const libc::c_char,
                var: &prompt_password as *const YesNo as *mut YesNo as *mut libc::c_void,
                allowed: SOURCE_DEFAULT,
                source: SOURCE_DEFAULT,
            };
            init
        },
        {
            let mut init = pgut_option {
                type_0: 'y' as i32 as libc::c_char,
                sname: 'W' as i32 as libc::c_char,
                lname: b"password\0" as *const u8 as *const libc::c_char,
                var: &prompt_password as *const YesNo as *mut YesNo as *mut libc::c_void,
                allowed: SOURCE_DEFAULT,
                source: SOURCE_DEFAULT,
            };
            init
        },
        {
            let mut init = pgut_option {
                type_0: 0 as libc::c_int as libc::c_char,
                sname: 0,
                lname: 0 as *const libc::c_char,
                var: 0 as *const libc::c_void as *mut libc::c_void,
                allowed: SOURCE_DEFAULT,
                source: SOURCE_DEFAULT,
            };
            init
        },
    ]
};
unsafe extern "C" fn option_length(mut opts: *const pgut_option) -> size_t {
    let mut len: size_t = 0;
    len = 0 as libc::c_int as size_t;
    while !opts.is_null() && (*opts.offset(len as isize)).type_0 as libc::c_int != 0 {
        len = len.wrapping_add(1);
        len;
    }
    return len;
}
unsafe extern "C" fn option_find(
    mut c: libc::c_int,
    mut opts1: *mut pgut_option,
    mut opts2: *mut pgut_option,
) -> *mut pgut_option {
    let mut i: size_t = 0;
    i = 0 as libc::c_int as size_t;
    while !opts1.is_null() && (*opts1.offset(i as isize)).type_0 as libc::c_int != 0 {
        if (*opts1.offset(i as isize)).sname as libc::c_int == c {
            return &mut *opts1.offset(i as isize) as *mut pgut_option;
        }
        i = i.wrapping_add(1);
        i;
    }
    i = 0 as libc::c_int as size_t;
    while !opts2.is_null() && (*opts2.offset(i as isize)).type_0 as libc::c_int != 0 {
        if (*opts2.offset(i as isize)).sname as libc::c_int == c {
            return &mut *opts2.offset(i as isize) as *mut pgut_option;
        }
        i = i.wrapping_add(1);
        i;
    }
    return 0 as *mut pgut_option;
}
unsafe extern "C" fn get_username() -> *mut libc::c_char {
    let mut ret: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut pw: *mut passwd = 0 as *mut passwd;
    pw = getpwuid(geteuid());
    ret = if !pw.is_null() { (*pw).pw_name } else { 0 as *mut libc::c_char };
    if ret.is_null() {
        if pgut_errstart(20 as libc::c_int) as libc::c_int != 0 {
            pgut_errfinish(
                errcode_errno(),
                errmsg(
                    b"could not get current user name: \0" as *const u8
                        as *const libc::c_char,
                ),
            );
        } else {};
    }
    return ret;
}
unsafe extern "C" fn option_has_arg(mut type_0: libc::c_char) -> libc::c_int {
    match type_0 as libc::c_int {
        98 | 66 | 121 | 89 => return 0 as libc::c_int,
        _ => return 1 as libc::c_int,
    };
}
unsafe extern "C" fn option_copy(
    mut dst: *mut option,
    mut opts: *const pgut_option,
    mut len: size_t,
) {
    let mut i: size_t = 0;
    i = 0 as libc::c_int as size_t;
    while i < len {
        let ref mut fresh4 = (*dst.offset(i as isize)).name;
        *fresh4 = (*opts.offset(i as isize)).lname;
        (*dst.offset(i as isize))
            .has_arg = option_has_arg((*opts.offset(i as isize)).type_0);
        let ref mut fresh5 = (*dst.offset(i as isize)).flag;
        *fresh5 = 0 as *mut libc::c_int;
        (*dst.offset(i as isize)).val = (*opts.offset(i as isize)).sname as libc::c_int;
        i = i.wrapping_add(1);
        i;
    }
}
unsafe extern "C" fn option_merge(
    mut opts1: *const pgut_option,
    mut opts2: *const pgut_option,
) -> *mut option {
    let mut result: *mut option = 0 as *mut option;
    let mut len1: size_t = option_length(opts1);
    let mut len2: size_t = option_length(opts2);
    let mut n: size_t = len1.wrapping_add(len2);
    result = pgut_malloc(
        (::std::mem::size_of::<option>() as libc::c_ulong)
            .wrapping_mul(n.wrapping_add(1 as libc::c_int as libc::c_ulong)),
    ) as *mut option;
    option_copy(result, opts1, len1);
    option_copy(result.offset(len1 as isize), opts2, len2);
    memset(
        &mut *result.offset(n as isize) as *mut option as *mut libc::c_void,
        0 as libc::c_int,
        ::std::mem::size_of::<pgut_option>() as libc::c_ulong,
    );
    return result;
}
unsafe extern "C" fn longopts_to_optstring(
    mut opts: *const option,
) -> *mut libc::c_char {
    let mut len: size_t = 0;
    let mut result: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut s: *mut libc::c_char = 0 as *mut libc::c_char;
    len = 0 as libc::c_int as size_t;
    while !((*opts.offset(len as isize)).name).is_null() {
        len = len.wrapping_add(1);
        len;
    }
    result = pgut_malloc(
        len
            .wrapping_mul(2 as libc::c_int as libc::c_ulong)
            .wrapping_add(1 as libc::c_int as libc::c_ulong),
    ) as *mut libc::c_char;
    s = result;
    len = 0 as libc::c_int as size_t;
    while !((*opts.offset(len as isize)).name).is_null() {
        if !(*(*__ctype_b_loc()).offset((*opts.offset(len as isize)).val as isize)
            as libc::c_int & _ISprint as libc::c_int as libc::c_ushort as libc::c_int
            == 0)
        {
            let fresh6 = s;
            s = s.offset(1);
            *fresh6 = (*opts.offset(len as isize)).val as libc::c_char;
            if (*opts.offset(len as isize)).has_arg != 0 as libc::c_int {
                let fresh7 = s;
                s = s.offset(1);
                *fresh7 = ':' as i32 as libc::c_char;
            }
        }
        len = len.wrapping_add(1);
        len;
    }
    *s = '\0' as i32 as libc::c_char;
    return result;
}
pub unsafe extern "C" fn pgut_getopt(
    mut argc: libc::c_int,
    mut argv: *mut *mut libc::c_char,
    mut options: *mut pgut_option,
) -> libc::c_int {
    let mut c: libc::c_int = 0;
    let mut optindex: libc::c_int = 0 as libc::c_int;
    let mut optstring: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut longopts: *mut option = 0 as *mut option;
    let mut opt: *mut pgut_option = 0 as *mut pgut_option;
    pgut_init(argc, argv);
    if argc > 1 as libc::c_int {
        if strcmp(
            *argv.offset(1 as libc::c_int as isize),
            b"--help\0" as *const u8 as *const libc::c_char,
        ) == 0 as libc::c_int
            || strcmp(
                *argv.offset(1 as libc::c_int as isize),
                b"-?\0" as *const u8 as *const libc::c_char,
            ) == 0 as libc::c_int
        {
            help(1 as libc::c_int != 0);
            exit(0 as libc::c_int);
        }
        if strcmp(
            *argv.offset(1 as libc::c_int as isize),
            b"--version\0" as *const u8 as *const libc::c_char,
        ) == 0 as libc::c_int
            || strcmp(
                *argv.offset(1 as libc::c_int as isize),
                b"-V\0" as *const u8 as *const libc::c_char,
            ) == 0 as libc::c_int
        {
            pg_printf(
                b"%s %s\n\0" as *const u8 as *const libc::c_char,
                PROGRAM_NAME,
                PROGRAM_VERSION,
            );
            exit(0 as libc::c_int);
        }
        if strcmp(
            *argv.offset(1 as libc::c_int as isize),
            b"--configuration\0" as *const u8 as *const libc::c_char,
        ) == 0 as libc::c_int
        {
            pg_printf(
                b"%s\n\0" as *const u8 as *const libc::c_char,
                b"PostgreSQL 12.22 (Ubuntu 12.22-0ubuntu0.20.04.4) on x86_64-pc-linux-gnu, compiled by gcc (Ubuntu 9.4.0-1ubuntu1~20.04.2) 9.4.0, 64-bit\0"
                    as *const u8 as *const libc::c_char,
            );
            exit(0 as libc::c_int);
        }
    }
    longopts = option_merge(
        default_options.as_mut_ptr() as *const pgut_option,
        options as *const pgut_option,
    );
    optstring = longopts_to_optstring(longopts as *const option);
    loop {
        c = getopt_long(argc, argv, optstring, longopts, &mut optindex);
        if !(c != -(1 as libc::c_int)) {
            break;
        }
        opt = option_find(c, default_options.as_mut_ptr(), options);
        pgut_setopt(opt, optarg, SOURCE_CMDLINE);
    }
    option_from_env(options);
    (!dbname.is_null()
        || {
            dbname = getenv(b"PGDATABASE\0" as *const u8 as *const libc::c_char);
            !dbname.is_null()
        }
        || {
            dbname = getenv(b"PGUSER\0" as *const u8 as *const libc::c_char);
            !dbname.is_null()
        }
        || {
            dbname = get_username();
            !dbname.is_null()
        }) as libc::c_int;
    return optind;
}
pub unsafe extern "C" fn help(mut details: bool) {
    pgut_help(details);
    if details {
        pg_printf(b"\nConnection options:\n\0" as *const u8 as *const libc::c_char);
        pg_printf(
            b"  -d, --dbname=DBNAME       database to connect\n\0" as *const u8
                as *const libc::c_char,
        );
        pg_printf(
            b"  -h, --host=HOSTNAME       database server host or socket directory\n\0"
                as *const u8 as *const libc::c_char,
        );
        pg_printf(
            b"  -p, --port=PORT           database server port\n\0" as *const u8
                as *const libc::c_char,
        );
        pg_printf(
            b"  -U, --username=USERNAME   user name to connect as\n\0" as *const u8
                as *const libc::c_char,
        );
        pg_printf(
            b"  -w, --no-password         never prompt for password\n\0" as *const u8
                as *const libc::c_char,
        );
        pg_printf(
            b"  -W, --password            force password prompt\n\0" as *const u8
                as *const libc::c_char,
        );
    }
    pg_printf(b"\nGeneric options:\n\0" as *const u8 as *const libc::c_char);
    if details {
        pg_printf(
            b"  -e, --echo                echo queries\n\0" as *const u8
                as *const libc::c_char,
        );
        pg_printf(
            b"  -E, --elevel=LEVEL        set output message level\n\0" as *const u8
                as *const libc::c_char,
        );
    }
    pg_printf(
        b"  --help                    show this help, then exit\n\0" as *const u8
            as *const libc::c_char,
    );
    pg_printf(
        b"  --version                 output version information, then exit\n\0"
            as *const u8 as *const libc::c_char,
    );
    if details as libc::c_int != 0
        && (!PROGRAM_URL.is_null() || !PROGRAM_ISSUES.is_null())
    {
        pg_printf(b"\n\0" as *const u8 as *const libc::c_char);
        if !PROGRAM_URL.is_null() {
            pg_printf(
                b"Read the website for details: <%s>.\n\0" as *const u8
                    as *const libc::c_char,
                PROGRAM_URL,
            );
        }
        if !PROGRAM_ISSUES.is_null() {
            pg_printf(
                b"Report bugs to <%s>.\n\0" as *const u8 as *const libc::c_char,
                PROGRAM_ISSUES,
            );
        }
    }
}
