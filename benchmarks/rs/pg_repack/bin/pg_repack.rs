use ::libc;
extern "C" {
    pub type _IO_wide_data;
    pub type _IO_codecvt;
    pub type _IO_marker;
    pub type pg_conn;
    pub type pg_result;
    static mut stderr: *mut FILE;
    fn getenv(__name: *const libc::c_char) -> *mut libc::c_char;
    fn free(__ptr: *mut libc::c_void);
    fn calloc(_: libc::c_ulong, _: libc::c_ulong) -> *mut libc::c_void;
    fn strcmp(_: *const libc::c_char, _: *const libc::c_char) -> libc::c_int;
    fn strncat(
        _: *mut libc::c_char,
        _: *const libc::c_char,
        _: libc::c_ulong,
    ) -> *mut libc::c_char;
    fn strtol(
        _: *const libc::c_char,
        _: *mut *mut libc::c_char,
        _: libc::c_int,
    ) -> libc::c_long;
    fn strtoul(
        _: *const libc::c_char,
        _: *mut *mut libc::c_char,
        _: libc::c_int,
    ) -> libc::c_ulong;
    fn __errno_location() -> *mut libc::c_int;
    fn pg_printf(fmt: *const libc::c_char, _: ...) -> libc::c_int;
    fn pg_fprintf(stream: *mut FILE, fmt: *const libc::c_char, _: ...) -> libc::c_int;
    fn pg_sprintf(
        str: *mut libc::c_char,
        fmt: *const libc::c_char,
        _: ...
    ) -> libc::c_int;
    fn pg_snprintf(
        str: *mut libc::c_char,
        count: size_t,
        fmt: *const libc::c_char,
        _: ...
    ) -> libc::c_int;
    fn __assert_fail(
        __assertion: *const libc::c_char,
        __file: *const libc::c_char,
        __line: libc::c_uint,
        __function: *const libc::c_char,
    ) -> !;
    fn sleep(__seconds: libc::c_uint) -> libc::c_uint;
    fn PQstatus(conn: *const PGconn) -> ConnStatusType;
    fn PQparameterStatus(
        conn: *const PGconn,
        paramName: *const libc::c_char,
    ) -> *const libc::c_char;
    fn PQserverVersion(conn: *const PGconn) -> libc::c_int;
    fn PQerrorMessage(conn: *const PGconn) -> *mut libc::c_char;
    fn PQsocket(conn: *const PGconn) -> libc::c_int;
    fn PQsendQuery(conn: *mut PGconn, query: *const libc::c_char) -> libc::c_int;
    fn PQgetResult(conn: *mut PGconn) -> *mut PGresult;
    fn PQisBusy(conn: *mut PGconn) -> libc::c_int;
    fn PQconsumeInput(conn: *mut PGconn) -> libc::c_int;
    fn PQsetnonblocking(conn: *mut PGconn, arg: libc::c_int) -> libc::c_int;
    fn PQresultStatus(res: *const PGresult) -> ExecStatusType;
    fn PQresultErrorField(
        res: *const PGresult,
        fieldcode: libc::c_int,
    ) -> *mut libc::c_char;
    fn PQntuples(res: *const PGresult) -> libc::c_int;
    fn PQgetvalue(
        res: *const PGresult,
        tup_num: libc::c_int,
        field_num: libc::c_int,
    ) -> *mut libc::c_char;
    fn PQgetisnull(
        res: *const PGresult,
        tup_num: libc::c_int,
        field_num: libc::c_int,
    ) -> libc::c_int;
    fn PQclear(res: *mut PGresult);
    fn errcode(sqlerrcode: libc::c_int) -> libc::c_int;
    fn errmsg(fmt: *const libc::c_char, _: ...) -> libc::c_int;
    fn errdetail(fmt: *const libc::c_char, _: ...) -> libc::c_int;
    static mut PROGRAM_NAME: *const libc::c_char;
    fn pgut_atexit_push(callback: pgut_atexit_callback, userdata: *mut libc::c_void);
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
    fn pgut_rollback(conn: *mut PGconn);
    fn pgut_malloc(size: size_t) -> *mut libc::c_void;
    fn pgut_strdup(str: *const libc::c_char) -> *mut libc::c_char;
    fn elog(elevel: libc::c_int, fmt: *const libc::c_char, _: ...);
    fn pgut_errstart(elevel: libc::c_int) -> bool;
    fn pgut_errfinish(dummy: libc::c_int, _: ...);
    fn simple_string_list_append(list: *mut SimpleStringList, val: *const libc::c_char);
    fn simple_string_list_size(list: SimpleStringList) -> size_t;
    static mut dbname: *mut libc::c_char;
    static mut connection: *mut PGconn;
    static mut conn2: *mut PGconn;
    static mut workers: worker_conns;
    fn disconnect();
    fn reconnect(elevel: libc::c_int);
    fn setup_workers(num_workers: libc::c_int);
    fn execute(
        query: *const libc::c_char,
        nParams: libc::c_int,
        params: *mut *const libc::c_char,
    ) -> *mut PGresult;
    fn execute_elevel(
        query: *const libc::c_char,
        nParams: libc::c_int,
        params: *mut *const libc::c_char,
        elevel: libc::c_int,
    ) -> *mut PGresult;
    fn command(
        query: *const libc::c_char,
        nParams: libc::c_int,
        params: *mut *const libc::c_char,
    ) -> ExecStatusType;
    fn pgut_getopt(
        argc: libc::c_int,
        argv: *mut *mut libc::c_char,
        options_0: *mut pgut_option,
    ) -> libc::c_int;
    fn initPQExpBuffer(str: PQExpBuffer);
    fn termPQExpBuffer(str: PQExpBuffer);
    fn resetPQExpBuffer(str: PQExpBuffer);
    fn printfPQExpBuffer(str: PQExpBuffer, fmt: *const libc::c_char, _: ...);
    fn appendPQExpBuffer(str: PQExpBuffer, fmt: *const libc::c_char, _: ...);
    fn appendPQExpBufferStr(str: PQExpBuffer, data: *const libc::c_char);
    fn appendPQExpBufferChar(str: PQExpBuffer, ch: libc::c_char);
    fn time(__timer: *mut time_t) -> time_t;
    fn poll(__fds: *mut pollfd, __nfds: nfds_t, __timeout: libc::c_int) -> libc::c_int;
}
pub type Oid = libc::c_uint;
pub type size_t = libc::c_ulong;
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
pub type pgut_atexit_callback = Option::<
    unsafe extern "C" fn(bool, *mut libc::c_void) -> (),
>;
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct worker_conns {
    pub max_num_workers: libc::c_int,
    pub num_workers: libc::c_int,
    pub conns: *mut *mut PGconn,
}
pub type nfds_t = libc::c_ulong;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct pollfd {
    pub fd: libc::c_int,
    pub events: libc::c_short,
    pub revents: libc::c_short,
}
pub type index_status_t = libc::c_uint;
pub const FINISHED: index_status_t = 2;
pub const INPROGRESS: index_status_t = 1;
pub const UNPROCESSED: index_status_t = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct repack_index {
    pub target_oid: Oid,
    pub create_index: *const libc::c_char,
    pub status: index_status_t,
    pub worker_idx: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct repack_table {
    pub target_name: *const libc::c_char,
    pub target_oid: Oid,
    pub target_toast: Oid,
    pub target_tidx: Oid,
    pub pkid: Oid,
    pub ckid: Oid,
    pub create_pktype: *const libc::c_char,
    pub create_log: *const libc::c_char,
    pub create_trigger: *const libc::c_char,
    pub enable_trigger: *const libc::c_char,
    pub create_table: *const libc::c_char,
    pub copy_data: *const libc::c_char,
    pub alter_col_storage: *const libc::c_char,
    pub drop_columns: *const libc::c_char,
    pub delete_log: *const libc::c_char,
    pub lock_table: *const libc::c_char,
    pub sql_peek: *const libc::c_char,
    pub sql_insert: *const libc::c_char,
    pub sql_delete: *const libc::c_char,
    pub sql_update: *const libc::c_char,
    pub sql_pop: *const libc::c_char,
    pub n_indexes: libc::c_int,
    pub indexes: *mut repack_index,
}
pub static mut PROGRAM_URL: *const libc::c_char = b"https://reorg.github.io/pg_repack/\0"
    as *const u8 as *const libc::c_char;
pub static mut PROGRAM_ISSUES: *const libc::c_char = b"https://github.com/reorg/pg_repack/issues\0"
    as *const u8 as *const libc::c_char;
pub static mut PROGRAM_VERSION: *const libc::c_char = b"1.4.7\0" as *const u8
    as *const libc::c_char;
#[inline]
unsafe extern "C" fn atoi(mut __nptr: *const libc::c_char) -> libc::c_int {
    return strtol(
        __nptr,
        0 as *mut libc::c_void as *mut *mut libc::c_char,
        10 as libc::c_int,
    ) as libc::c_int;
}
unsafe extern "C" fn sqlstate_equals(
    mut res: *mut PGresult,
    mut state: *const libc::c_char,
) -> bool {
    return strcmp(PQresultErrorField(res, 'C' as i32), state) == 0 as libc::c_int;
}
static mut analyze: bool = 1 as libc::c_int != 0;
static mut alldb: bool = 0 as libc::c_int != 0;
static mut noorder: bool = 0 as libc::c_int != 0;
static mut parent_table_list: SimpleStringList = {
    let mut init = SimpleStringList {
        head: 0 as *const SimpleStringListCell as *mut SimpleStringListCell,
        tail: 0 as *const SimpleStringListCell as *mut SimpleStringListCell,
    };
    init
};
static mut table_list: SimpleStringList = {
    let mut init = SimpleStringList {
        head: 0 as *const SimpleStringListCell as *mut SimpleStringListCell,
        tail: 0 as *const SimpleStringListCell as *mut SimpleStringListCell,
    };
    init
};
static mut schema_list: SimpleStringList = {
    let mut init = SimpleStringList {
        head: 0 as *const SimpleStringListCell as *mut SimpleStringListCell,
        tail: 0 as *const SimpleStringListCell as *mut SimpleStringListCell,
    };
    init
};
static mut orderby: *mut libc::c_char = 0 as *const libc::c_char as *mut libc::c_char;
static mut tablespace: *mut libc::c_char = 0 as *const libc::c_char as *mut libc::c_char;
static mut moveidx: bool = 0 as libc::c_int != 0;
static mut r_index: SimpleStringList = {
    let mut init = SimpleStringList {
        head: 0 as *const SimpleStringListCell as *mut SimpleStringListCell,
        tail: 0 as *const SimpleStringListCell as *mut SimpleStringListCell,
    };
    init
};
static mut only_indexes: bool = 0 as libc::c_int != 0;
static mut wait_timeout: libc::c_int = 60 as libc::c_int;
static mut jobs: libc::c_int = 0 as libc::c_int;
static mut dryrun: bool = 0 as libc::c_int != 0;
static mut temp_obj_num: libc::c_uint = 0 as libc::c_int as libc::c_uint;
static mut no_kill_backend: bool = 0 as libc::c_int != 0;
static mut no_superuser_check: bool = 0 as libc::c_int != 0;
static mut exclude_extension_list: SimpleStringList = {
    let mut init = SimpleStringList {
        head: 0 as *const SimpleStringListCell as *mut SimpleStringListCell,
        tail: 0 as *const SimpleStringListCell as *mut SimpleStringListCell,
    };
    init
};
unsafe extern "C" fn utoa(
    mut value: libc::c_uint,
    mut buffer: *mut libc::c_char,
) -> *mut libc::c_char {
    pg_sprintf(buffer, b"%u\0" as *const u8 as *const libc::c_char, value);
    return buffer;
}
static mut options: [pgut_option; 18] = unsafe {
    [
        {
            let mut init = pgut_option {
                type_0: 'b' as i32 as libc::c_char,
                sname: 'a' as i32 as libc::c_char,
                lname: b"all\0" as *const u8 as *const libc::c_char,
                var: &alldb as *const bool as *mut bool as *mut libc::c_void,
                allowed: SOURCE_DEFAULT,
                source: SOURCE_DEFAULT,
            };
            init
        },
        {
            let mut init = pgut_option {
                type_0: 'l' as i32 as libc::c_char,
                sname: 't' as i32 as libc::c_char,
                lname: b"table\0" as *const u8 as *const libc::c_char,
                var: &table_list as *const SimpleStringList as *mut SimpleStringList
                    as *mut libc::c_void,
                allowed: SOURCE_DEFAULT,
                source: SOURCE_DEFAULT,
            };
            init
        },
        {
            let mut init = pgut_option {
                type_0: 'l' as i32 as libc::c_char,
                sname: 'I' as i32 as libc::c_char,
                lname: b"parent-table\0" as *const u8 as *const libc::c_char,
                var: &parent_table_list as *const SimpleStringList
                    as *mut SimpleStringList as *mut libc::c_void,
                allowed: SOURCE_DEFAULT,
                source: SOURCE_DEFAULT,
            };
            init
        },
        {
            let mut init = pgut_option {
                type_0: 'l' as i32 as libc::c_char,
                sname: 'c' as i32 as libc::c_char,
                lname: b"schema\0" as *const u8 as *const libc::c_char,
                var: &schema_list as *const SimpleStringList as *mut SimpleStringList
                    as *mut libc::c_void,
                allowed: SOURCE_DEFAULT,
                source: SOURCE_DEFAULT,
            };
            init
        },
        {
            let mut init = pgut_option {
                type_0: 'b' as i32 as libc::c_char,
                sname: 'n' as i32 as libc::c_char,
                lname: b"no-order\0" as *const u8 as *const libc::c_char,
                var: &noorder as *const bool as *mut bool as *mut libc::c_void,
                allowed: SOURCE_DEFAULT,
                source: SOURCE_DEFAULT,
            };
            init
        },
        {
            let mut init = pgut_option {
                type_0: 'b' as i32 as libc::c_char,
                sname: 'N' as i32 as libc::c_char,
                lname: b"dry-run\0" as *const u8 as *const libc::c_char,
                var: &dryrun as *const bool as *mut bool as *mut libc::c_void,
                allowed: SOURCE_DEFAULT,
                source: SOURCE_DEFAULT,
            };
            init
        },
        {
            let mut init = pgut_option {
                type_0: 's' as i32 as libc::c_char,
                sname: 'o' as i32 as libc::c_char,
                lname: b"order-by\0" as *const u8 as *const libc::c_char,
                var: &orderby as *const *mut libc::c_char as *mut *mut libc::c_char
                    as *mut libc::c_void,
                allowed: SOURCE_DEFAULT,
                source: SOURCE_DEFAULT,
            };
            init
        },
        {
            let mut init = pgut_option {
                type_0: 's' as i32 as libc::c_char,
                sname: 's' as i32 as libc::c_char,
                lname: b"tablespace\0" as *const u8 as *const libc::c_char,
                var: &tablespace as *const *mut libc::c_char as *mut *mut libc::c_char
                    as *mut libc::c_void,
                allowed: SOURCE_DEFAULT,
                source: SOURCE_DEFAULT,
            };
            init
        },
        {
            let mut init = pgut_option {
                type_0: 'b' as i32 as libc::c_char,
                sname: 'S' as i32 as libc::c_char,
                lname: b"moveidx\0" as *const u8 as *const libc::c_char,
                var: &moveidx as *const bool as *mut bool as *mut libc::c_void,
                allowed: SOURCE_DEFAULT,
                source: SOURCE_DEFAULT,
            };
            init
        },
        {
            let mut init = pgut_option {
                type_0: 'l' as i32 as libc::c_char,
                sname: 'i' as i32 as libc::c_char,
                lname: b"index\0" as *const u8 as *const libc::c_char,
                var: &r_index as *const SimpleStringList as *mut SimpleStringList
                    as *mut libc::c_void,
                allowed: SOURCE_DEFAULT,
                source: SOURCE_DEFAULT,
            };
            init
        },
        {
            let mut init = pgut_option {
                type_0: 'b' as i32 as libc::c_char,
                sname: 'x' as i32 as libc::c_char,
                lname: b"only-indexes\0" as *const u8 as *const libc::c_char,
                var: &only_indexes as *const bool as *mut bool as *mut libc::c_void,
                allowed: SOURCE_DEFAULT,
                source: SOURCE_DEFAULT,
            };
            init
        },
        {
            let mut init = pgut_option {
                type_0: 'i' as i32 as libc::c_char,
                sname: 'T' as i32 as libc::c_char,
                lname: b"wait-timeout\0" as *const u8 as *const libc::c_char,
                var: &wait_timeout as *const libc::c_int as *mut libc::c_int
                    as *mut libc::c_void,
                allowed: SOURCE_DEFAULT,
                source: SOURCE_DEFAULT,
            };
            init
        },
        {
            let mut init = pgut_option {
                type_0: 'B' as i32 as libc::c_char,
                sname: 'Z' as i32 as libc::c_char,
                lname: b"no-analyze\0" as *const u8 as *const libc::c_char,
                var: &analyze as *const bool as *mut bool as *mut libc::c_void,
                allowed: SOURCE_DEFAULT,
                source: SOURCE_DEFAULT,
            };
            init
        },
        {
            let mut init = pgut_option {
                type_0: 'i' as i32 as libc::c_char,
                sname: 'j' as i32 as libc::c_char,
                lname: b"jobs\0" as *const u8 as *const libc::c_char,
                var: &jobs as *const libc::c_int as *mut libc::c_int
                    as *mut libc::c_void,
                allowed: SOURCE_DEFAULT,
                source: SOURCE_DEFAULT,
            };
            init
        },
        {
            let mut init = pgut_option {
                type_0: 'b' as i32 as libc::c_char,
                sname: 'D' as i32 as libc::c_char,
                lname: b"no-kill-backend\0" as *const u8 as *const libc::c_char,
                var: &no_kill_backend as *const bool as *mut bool as *mut libc::c_void,
                allowed: SOURCE_DEFAULT,
                source: SOURCE_DEFAULT,
            };
            init
        },
        {
            let mut init = pgut_option {
                type_0: 'b' as i32 as libc::c_char,
                sname: 'k' as i32 as libc::c_char,
                lname: b"no-superuser-check\0" as *const u8 as *const libc::c_char,
                var: &no_superuser_check as *const bool as *mut bool
                    as *mut libc::c_void,
                allowed: SOURCE_DEFAULT,
                source: SOURCE_DEFAULT,
            };
            init
        },
        {
            let mut init = pgut_option {
                type_0: 'l' as i32 as libc::c_char,
                sname: 'C' as i32 as libc::c_char,
                lname: b"exclude-extension\0" as *const u8 as *const libc::c_char,
                var: &exclude_extension_list as *const SimpleStringList
                    as *mut SimpleStringList as *mut libc::c_void,
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
unsafe fn main_0(
    mut argc: libc::c_int,
    mut argv: *mut *mut libc::c_char,
) -> libc::c_int {
    let mut i: libc::c_int = 0;
    let mut errbuf: [libc::c_char; 256] = [0; 256];
    i = pgut_getopt(argc, argv, options.as_mut_ptr());
    if i == argc - 1 as libc::c_int {
        dbname = *argv.offset(i as isize);
    } else if i < argc {
        if pgut_errstart(20 as libc::c_int) as libc::c_int != 0 {
            pgut_errfinish(
                errcode(22 as libc::c_int),
                errmsg(b"too many arguments\0" as *const u8 as *const libc::c_char),
            );
        } else {};
    }
    check_tablespace();
    if dryrun {
        elog(
            17 as libc::c_int,
            b"Dry run enabled, not executing repack\0" as *const u8
                as *const libc::c_char,
        );
    }
    if !(r_index.head).is_null() || only_indexes as libc::c_int != 0 {
        if !(r_index.head).is_null() && !(table_list.head).is_null() {
            if pgut_errstart(20 as libc::c_int) as libc::c_int != 0 {
                pgut_errfinish(
                    errcode(22 as libc::c_int),
                    errmsg(
                        b"cannot specify --index (-i) and --table (-t)\0" as *const u8
                            as *const libc::c_char,
                    ),
                );
            } else {};
        }
        if !(r_index.head).is_null() && !(parent_table_list.head).is_null() {
            if pgut_errstart(20 as libc::c_int) as libc::c_int != 0 {
                pgut_errfinish(
                    errcode(22 as libc::c_int),
                    errmsg(
                        b"cannot specify --index (-i) and --parent-table (-I)\0"
                            as *const u8 as *const libc::c_char,
                    ),
                );
            } else {};
        } else if !(r_index.head).is_null() && only_indexes as libc::c_int != 0 {
            if pgut_errstart(20 as libc::c_int) as libc::c_int != 0 {
                pgut_errfinish(
                    errcode(22 as libc::c_int),
                    errmsg(
                        b"cannot specify --index (-i) and --only-indexes (-x)\0"
                            as *const u8 as *const libc::c_char,
                    ),
                );
            } else {};
        } else if !(r_index.head).is_null() && !(exclude_extension_list.head).is_null() {
            if pgut_errstart(20 as libc::c_int) as libc::c_int != 0 {
                pgut_errfinish(
                    errcode(22 as libc::c_int),
                    errmsg(
                        b"cannot specify --index (-i) and --exclude-extension (-C)\0"
                            as *const u8 as *const libc::c_char,
                    ),
                );
            } else {};
        } else if only_indexes as libc::c_int != 0
            && !(!(table_list.head).is_null() || !(parent_table_list.head).is_null())
        {
            if pgut_errstart(20 as libc::c_int) as libc::c_int != 0 {
                pgut_errfinish(
                    errcode(22 as libc::c_int),
                    errmsg(
                        b"cannot repack all indexes of database, specify the table(s)via --table (-t) or --parent-table (-I)\0"
                            as *const u8 as *const libc::c_char,
                    ),
                );
            } else {};
        } else if only_indexes as libc::c_int != 0
            && !(exclude_extension_list.head).is_null()
        {
            if pgut_errstart(20 as libc::c_int) as libc::c_int != 0 {
                pgut_errfinish(
                    errcode(22 as libc::c_int),
                    errmsg(
                        b"cannot specify --only-indexes (-x) and --exclude-extension (-C)\0"
                            as *const u8 as *const libc::c_char,
                    ),
                );
            } else {};
        } else if alldb {
            if pgut_errstart(20 as libc::c_int) as libc::c_int != 0 {
                pgut_errfinish(
                    errcode(22 as libc::c_int),
                    errmsg(
                        b"cannot repack specific index(es) in all databases\0"
                            as *const u8 as *const libc::c_char,
                    ),
                );
            } else {};
        } else {
            if !orderby.is_null() {
                if pgut_errstart(19 as libc::c_int) as libc::c_int != 0 {
                    pgut_errfinish(
                        errcode(22 as libc::c_int),
                        errmsg(
                            b"option -o (--order-by) has no effect while repacking indexes\0"
                                as *const u8 as *const libc::c_char,
                        ),
                    );
                } else {};
            } else if noorder {
                if pgut_errstart(19 as libc::c_int) as libc::c_int != 0 {
                    pgut_errfinish(
                        errcode(22 as libc::c_int),
                        errmsg(
                            b"option -n (--no-order) has no effect while repacking indexes\0"
                                as *const u8 as *const libc::c_char,
                        ),
                    );
                } else {};
            } else if !analyze {
                if pgut_errstart(19 as libc::c_int) as libc::c_int != 0 {
                    pgut_errfinish(
                        errcode(22 as libc::c_int),
                        errmsg(
                            b"ANALYZE is not performed after repacking indexes, -z (--no-analyze) has no effect\0"
                                as *const u8 as *const libc::c_char,
                        ),
                    );
                } else {};
            } else if jobs != 0 {
                if pgut_errstart(19 as libc::c_int) as libc::c_int != 0 {
                    pgut_errfinish(
                        errcode(22 as libc::c_int),
                        errmsg(
                            b"option -j (--jobs) has no effect, repacking indexes does not use parallel jobs\0"
                                as *const u8 as *const libc::c_char,
                        ),
                    );
                } else {};
            }
            if !repack_all_indexes(
                errbuf.as_mut_ptr(),
                ::std::mem::size_of::<[libc::c_char; 256]>() as libc::c_ulong,
            ) {
                if pgut_errstart(20 as libc::c_int) as libc::c_int != 0 {
                    pgut_errfinish(
                        errcode(20 as libc::c_int),
                        errmsg(
                            b"%s\0" as *const u8 as *const libc::c_char,
                            errbuf.as_mut_ptr(),
                        ),
                    );
                } else {};
            }
        }
    } else {
        if !(schema_list.head).is_null()
            && (!(table_list.head).is_null() || !(parent_table_list.head).is_null())
        {
            if pgut_errstart(20 as libc::c_int) as libc::c_int != 0 {
                pgut_errfinish(
                    errcode(22 as libc::c_int),
                    errmsg(
                        b"cannot repack specific table(s) in schema, use schema.table notation instead\0"
                            as *const u8 as *const libc::c_char,
                    ),
                );
            } else {};
        }
        if !(exclude_extension_list.head).is_null() && !(table_list.head).is_null() {
            if pgut_errstart(20 as libc::c_int) as libc::c_int != 0 {
                pgut_errfinish(
                    errcode(22 as libc::c_int),
                    errmsg(
                        b"cannot specify --table (-t) and --exclude-extension (-C)\0"
                            as *const u8 as *const libc::c_char,
                    ),
                );
            } else {};
        }
        if !(exclude_extension_list.head).is_null()
            && !(parent_table_list.head).is_null()
        {
            if pgut_errstart(20 as libc::c_int) as libc::c_int != 0 {
                pgut_errfinish(
                    errcode(22 as libc::c_int),
                    errmsg(
                        b"cannot specify --parent-table (-I) and --exclude-extension (-C)\0"
                            as *const u8 as *const libc::c_char,
                    ),
                );
            } else {};
        }
        if noorder {
            orderby = b"\0" as *const u8 as *const libc::c_char as *mut libc::c_char;
        }
        if alldb {
            if !(table_list.head).is_null() || !(parent_table_list.head).is_null() {
                if pgut_errstart(20 as libc::c_int) as libc::c_int != 0 {
                    pgut_errfinish(
                        errcode(22 as libc::c_int),
                        errmsg(
                            b"cannot repack specific table(s) in all databases\0"
                                as *const u8 as *const libc::c_char,
                        ),
                    );
                } else {};
            }
            if !(schema_list.head).is_null() {
                if pgut_errstart(20 as libc::c_int) as libc::c_int != 0 {
                    pgut_errfinish(
                        errcode(22 as libc::c_int),
                        errmsg(
                            b"cannot repack specific schema(s) in all databases\0"
                                as *const u8 as *const libc::c_char,
                        ),
                    );
                } else {};
            }
            repack_all_databases(orderby);
        } else if !repack_one_database(
            orderby,
            errbuf.as_mut_ptr(),
            ::std::mem::size_of::<[libc::c_char; 256]>() as libc::c_ulong,
        ) {
            if pgut_errstart(20 as libc::c_int) as libc::c_int != 0 {
                pgut_errfinish(
                    errcode(20 as libc::c_int),
                    errmsg(
                        b"%s failed with error: %s\0" as *const u8
                            as *const libc::c_char,
                        PROGRAM_NAME,
                        errbuf.as_mut_ptr(),
                    ),
                );
            } else {};
        }
    }
    return 0 as libc::c_int;
}
unsafe extern "C" fn is_superuser() -> bool {
    let mut val: *const libc::c_char = 0 as *const libc::c_char;
    if no_superuser_check {
        return 1 as libc::c_int != 0;
    }
    if connection.is_null() {
        return 0 as libc::c_int != 0;
    }
    val = PQparameterStatus(
        connection,
        b"is_superuser\0" as *const u8 as *const libc::c_char,
    );
    if !val.is_null()
        && strcmp(val, b"on\0" as *const u8 as *const libc::c_char) == 0 as libc::c_int
    {
        return 1 as libc::c_int != 0;
    }
    return 0 as libc::c_int != 0;
}
unsafe extern "C" fn check_tablespace() {
    let mut res: *mut PGresult = 0 as *mut PGresult;
    let mut params: [*const libc::c_char; 1] = [0 as *const libc::c_char; 1];
    if tablespace.is_null() {
        if moveidx {
            if pgut_errstart(20 as libc::c_int) as libc::c_int != 0 {
                pgut_errfinish(
                    errcode(22 as libc::c_int),
                    errmsg(
                        b"cannot specify --moveidx (-S) without --tablespace (-s)\0"
                            as *const u8 as *const libc::c_char,
                    ),
                );
            } else {};
        }
        return;
    }
    reconnect(20 as libc::c_int);
    params[0 as libc::c_int as usize] = tablespace;
    res = execute_elevel(
        b"select spcname from pg_tablespace where spcname = $1\0" as *const u8
            as *const libc::c_char,
        1 as libc::c_int,
        params.as_mut_ptr(),
        13 as libc::c_int,
    );
    if PQresultStatus(res) as libc::c_uint
        == PGRES_TUPLES_OK as libc::c_int as libc::c_uint
    {
        if PQntuples(res) == 0 as libc::c_int {
            if pgut_errstart(20 as libc::c_int) as libc::c_int != 0 {
                pgut_errfinish(
                    errcode(22 as libc::c_int),
                    errmsg(
                        b"the tablespace \"%s\" doesn't exist\0" as *const u8
                            as *const libc::c_char,
                        tablespace,
                    ),
                );
            } else {};
        }
    } else {
        if pgut_errstart(20 as libc::c_int) as libc::c_int != 0 {
            pgut_errfinish(
                errcode(22 as libc::c_int),
                errmsg(
                    b"error checking the namespace: %s\0" as *const u8
                        as *const libc::c_char,
                    PQerrorMessage(connection),
                ),
            );
        } else {};
    }
    PQclear(res);
    res = 0 as *mut PGresult;
}
unsafe extern "C" fn preliminary_checks(
    mut errbuf: *mut libc::c_char,
    mut errsize: size_t,
) -> bool {
    let mut ret: bool = 0 as libc::c_int != 0;
    let mut res: *mut PGresult = 0 as *mut PGresult;
    if !is_superuser() {
        if !errbuf.is_null() {
            pg_snprintf(
                errbuf,
                errsize,
                b"You must be a superuser to use %s\0" as *const u8
                    as *const libc::c_char,
                PROGRAM_NAME,
            );
        }
    } else {
        res = execute_elevel(
            b"select repack.version(), repack.version_sql()\0" as *const u8
                as *const libc::c_char,
            0 as libc::c_int,
            0 as *mut *const libc::c_char,
            13 as libc::c_int,
        );
        if PQresultStatus(res) as libc::c_uint
            == PGRES_TUPLES_OK as libc::c_int as libc::c_uint
        {
            let mut libver: *const libc::c_char = 0 as *const libc::c_char;
            let mut buf: [libc::c_char; 64] = [0; 64];
            pg_snprintf(
                buf.as_mut_ptr(),
                ::std::mem::size_of::<[libc::c_char; 64]>() as libc::c_ulong,
                b"%s %s\0" as *const u8 as *const libc::c_char,
                PROGRAM_NAME,
                PROGRAM_VERSION,
            );
            libver = getstr(res, 0 as libc::c_int, 0 as libc::c_int);
            if 0 as libc::c_int != strcmp(buf.as_mut_ptr(), libver) {
                if !errbuf.is_null() {
                    pg_snprintf(
                        errbuf,
                        errsize,
                        b"program '%s' does not match database library '%s'\0"
                            as *const u8 as *const libc::c_char,
                        buf.as_mut_ptr(),
                        libver,
                    );
                }
            } else {
                libver = getstr(res, 0 as libc::c_int, 1 as libc::c_int);
                if 0 as libc::c_int != strcmp(buf.as_mut_ptr(), libver) {
                    if !errbuf.is_null() {
                        pg_snprintf(
                            errbuf,
                            errsize,
                            b"extension '%s' required, found '%s'; please drop and re-create the extension\0"
                                as *const u8 as *const libc::c_char,
                            buf.as_mut_ptr(),
                            libver,
                        );
                    }
                } else {
                    PQclear(res);
                    res = 0 as *mut PGresult;
                    command(
                        b"SET statement_timeout = 0\0" as *const u8
                            as *const libc::c_char,
                        0 as libc::c_int,
                        0 as *mut *const libc::c_char,
                    );
                    command(
                        b"SET search_path = pg_catalog, pg_temp, public\0" as *const u8
                            as *const libc::c_char,
                        0 as libc::c_int,
                        0 as *mut *const libc::c_char,
                    );
                    command(
                        b"SET client_min_messages = warning\0" as *const u8
                            as *const libc::c_char,
                        0 as libc::c_int,
                        0 as *mut *const libc::c_char,
                    );
                    ret = 1 as libc::c_int != 0;
                }
            }
        } else if sqlstate_equals(res, b"3F000\0" as *const u8 as *const libc::c_char)
            as libc::c_int != 0
            || sqlstate_equals(res, b"42883\0" as *const u8 as *const libc::c_char)
                as libc::c_int != 0
        {
            if !errbuf.is_null() {
                pg_snprintf(
                    errbuf,
                    errsize,
                    b"%s %s is not installed in the database\0" as *const u8
                        as *const libc::c_char,
                    PROGRAM_NAME,
                    PROGRAM_VERSION,
                );
            }
        } else if !errbuf.is_null() {
            pg_snprintf(
                errbuf,
                errsize,
                b"%s\0" as *const u8 as *const libc::c_char,
                PQerrorMessage(connection),
            );
        }
    }
    PQclear(res);
    res = 0 as *mut PGresult;
    return ret;
}
unsafe extern "C" fn is_requested_relation_exists(
    mut errbuf: *mut libc::c_char,
    mut errsize: size_t,
) -> bool {
    let mut ret: bool = 0 as libc::c_int != 0;
    let mut res: *mut PGresult = 0 as *mut PGresult;
    let mut params: *mut *const libc::c_char = 0 as *mut *const libc::c_char;
    let mut iparam: libc::c_int = 0 as libc::c_int;
    let mut sql: PQExpBufferData = PQExpBufferData {
        data: 0 as *mut libc::c_char,
        len: 0,
        maxlen: 0,
    };
    let mut num_relations: libc::c_int = 0;
    let mut cell: *mut SimpleStringListCell = 0 as *mut SimpleStringListCell;
    num_relations = (simple_string_list_size(parent_table_list))
        .wrapping_add(simple_string_list_size(table_list)) as libc::c_int;
    if num_relations == 0 as libc::c_int {
        return 1 as libc::c_int != 0;
    }
    if PQserverVersion(connection) < 90600 as libc::c_int {
        return 1 as libc::c_int != 0;
    }
    params = pgut_malloc(
        (num_relations as libc::c_ulong)
            .wrapping_mul(::std::mem::size_of::<*mut libc::c_char>() as libc::c_ulong),
    ) as *mut *const libc::c_char;
    initPQExpBuffer(&mut sql);
    appendPQExpBufferStr(
        &mut sql,
        b"SELECT r FROM (VALUES \0" as *const u8 as *const libc::c_char,
    );
    cell = table_list.head;
    while !cell.is_null() {
        appendPQExpBuffer(
            &mut sql as *mut PQExpBufferData,
            b"($%d)\0" as *const u8 as *const libc::c_char,
            iparam + 1 as libc::c_int,
        );
        let fresh0 = iparam;
        iparam = iparam + 1;
        let ref mut fresh1 = *params.offset(fresh0 as isize);
        *fresh1 = ((*cell).val).as_mut_ptr();
        if iparam < num_relations {
            appendPQExpBufferChar(&mut sql, ',' as i32 as libc::c_char);
        }
        cell = (*cell).next;
    }
    cell = parent_table_list.head;
    while !cell.is_null() {
        appendPQExpBuffer(
            &mut sql as *mut PQExpBufferData,
            b"($%d)\0" as *const u8 as *const libc::c_char,
            iparam + 1 as libc::c_int,
        );
        let fresh2 = iparam;
        iparam = iparam + 1;
        let ref mut fresh3 = *params.offset(fresh2 as isize);
        *fresh3 = ((*cell).val).as_mut_ptr();
        if iparam < num_relations {
            appendPQExpBufferChar(&mut sql, ',' as i32 as libc::c_char);
        }
        cell = (*cell).next;
    }
    appendPQExpBufferStr(
        &mut sql,
        b") AS given_t(r) WHERE NOT EXISTS(  SELECT FROM repack.tables WHERE relid=to_regclass(given_t.r) )\0"
            as *const u8 as *const libc::c_char,
    );
    if iparam != num_relations {
        if !errbuf.is_null() {
            pg_snprintf(
                errbuf,
                errsize,
                b"internal error: bad parameters count: %i instead of %i\0" as *const u8
                    as *const libc::c_char,
                iparam,
                num_relations,
            );
        }
    } else {
        res = execute_elevel(sql.data, iparam, params, 13 as libc::c_int);
        if PQresultStatus(res) as libc::c_uint
            == PGRES_TUPLES_OK as libc::c_int as libc::c_uint
        {
            let mut num: libc::c_int = 0;
            num = PQntuples(res);
            if num != 0 as libc::c_int {
                let mut i: libc::c_int = 0;
                let mut rel_names: PQExpBufferData = PQExpBufferData {
                    data: 0 as *mut libc::c_char,
                    len: 0,
                    maxlen: 0,
                };
                initPQExpBuffer(&mut rel_names);
                i = 0 as libc::c_int;
                while i < num {
                    appendPQExpBuffer(
                        &mut rel_names as *mut PQExpBufferData,
                        b"\"%s\"\0" as *const u8 as *const libc::c_char,
                        getstr(res, i, 0 as libc::c_int),
                    );
                    if i + 1 as libc::c_int != num {
                        appendPQExpBufferStr(
                            &mut rel_names,
                            b", \0" as *const u8 as *const libc::c_char,
                        );
                    }
                    i += 1;
                    i;
                }
                if !errbuf.is_null() {
                    if num > 1 as libc::c_int {
                        pg_snprintf(
                            errbuf,
                            errsize,
                            b"relations do not exist: %s\0" as *const u8
                                as *const libc::c_char,
                            rel_names.data,
                        );
                    } else {
                        pg_snprintf(
                            errbuf,
                            errsize,
                            b"ERROR:  relation %s does not exist\0" as *const u8
                                as *const libc::c_char,
                            rel_names.data,
                        );
                    }
                }
                termPQExpBuffer(&mut rel_names);
            } else {
                ret = 1 as libc::c_int != 0;
            }
        } else if !errbuf.is_null() {
            pg_snprintf(
                errbuf,
                errsize,
                b"%s\0" as *const u8 as *const libc::c_char,
                PQerrorMessage(connection),
            );
        }
        PQclear(res);
        res = 0 as *mut PGresult;
    }
    PQclear(res);
    res = 0 as *mut PGresult;
    termPQExpBuffer(&mut sql);
    free(params as *mut libc::c_void);
    return ret;
}
unsafe extern "C" fn repack_all_databases(mut orderby_0: *const libc::c_char) {
    let mut result: *mut PGresult = 0 as *mut PGresult;
    let mut i: libc::c_int = 0;
    dbname = b"postgres\0" as *const u8 as *const libc::c_char as *mut libc::c_char;
    reconnect(20 as libc::c_int);
    if !is_superuser() {
        elog(
            20 as libc::c_int,
            b"You must be a superuser to use %s\0" as *const u8 as *const libc::c_char,
            PROGRAM_NAME,
        );
    }
    result = execute(
        b"SELECT datname FROM pg_database WHERE datallowconn ORDER BY 1;\0" as *const u8
            as *const libc::c_char,
        0 as libc::c_int,
        0 as *mut *const libc::c_char,
    );
    disconnect();
    i = 0 as libc::c_int;
    while i < PQntuples(result) {
        let mut ret: bool = false;
        let mut errbuf: [libc::c_char; 256] = [0; 256];
        dbname = PQgetvalue(result, i, 0 as libc::c_int);
        elog(
            17 as libc::c_int,
            b"repacking database \"%s\"\0" as *const u8 as *const libc::c_char,
            dbname,
        );
        if !dryrun {
            ret = repack_one_database(
                orderby_0,
                errbuf.as_mut_ptr(),
                ::std::mem::size_of::<[libc::c_char; 256]>() as libc::c_ulong,
            );
            if !ret {
                elog(
                    17 as libc::c_int,
                    b"database \"%s\" skipped: %s\0" as *const u8 as *const libc::c_char,
                    dbname,
                    errbuf.as_mut_ptr(),
                );
            }
        }
        i += 1;
        i;
    }
    PQclear(result);
    result = 0 as *mut PGresult;
}
unsafe extern "C" fn getstr(
    mut res: *mut PGresult,
    mut row: libc::c_int,
    mut col: libc::c_int,
) -> *mut libc::c_char {
    if PQgetisnull(res, row, col) != 0 {
        return 0 as *mut libc::c_char
    } else {
        return PQgetvalue(res, row, col)
    };
}
unsafe extern "C" fn getoid(
    mut res: *mut PGresult,
    mut row: libc::c_int,
    mut col: libc::c_int,
) -> Oid {
    if PQgetisnull(res, row, col) != 0 {
        return 0 as libc::c_int as Oid
    } else {
        return strtoul(
            PQgetvalue(res, row, col),
            0 as *mut *mut libc::c_char,
            10 as libc::c_int,
        ) as Oid
    };
}
unsafe extern "C" fn repack_one_database(
    mut orderby_0: *const libc::c_char,
    mut errbuf: *mut libc::c_char,
    mut errsize: size_t,
) -> bool {
    let mut ret: bool = 0 as libc::c_int != 0;
    let mut res: *mut PGresult = 0 as *mut PGresult;
    let mut i: libc::c_int = 0;
    let mut num: libc::c_int = 0;
    let mut sql: PQExpBufferData = PQExpBufferData {
        data: 0 as *mut libc::c_char,
        len: 0,
        maxlen: 0,
    };
    let mut cell: *mut SimpleStringListCell = 0 as *mut SimpleStringListCell;
    let mut params: *mut *const libc::c_char = 0 as *mut *const libc::c_char;
    let mut iparam: libc::c_int = 0 as libc::c_int;
    let mut num_parent_tables: size_t = 0;
    let mut num_tables: size_t = 0;
    let mut num_schemas: size_t = 0;
    let mut num_params: size_t = 0;
    let mut num_excluded_extensions: size_t = 0;
    num_parent_tables = simple_string_list_size(parent_table_list);
    num_tables = simple_string_list_size(table_list);
    num_schemas = simple_string_list_size(schema_list);
    num_excluded_extensions = simple_string_list_size(exclude_extension_list);
    num_params = num_excluded_extensions
        .wrapping_add(num_parent_tables)
        .wrapping_add(num_tables)
        .wrapping_add(num_schemas)
        .wrapping_add(1 as libc::c_int as libc::c_ulong);
    params = pgut_malloc(
        num_params
            .wrapping_mul(::std::mem::size_of::<*mut libc::c_char>() as libc::c_ulong),
    ) as *mut *const libc::c_char;
    initPQExpBuffer(&mut sql);
    reconnect(20 as libc::c_int);
    if jobs > 1 as libc::c_int {
        setup_workers(jobs);
    }
    if preliminary_checks(errbuf, errsize) {
        if is_requested_relation_exists(errbuf, errsize) {
            appendPQExpBufferStr(
                &mut sql,
                b"SELECT t.*, coalesce(v.tablespace, t.tablespace_orig) as tablespace_dest FROM repack.tables t,  (VALUES (quote_ident($1::text))) as v (tablespace) WHERE \0"
                    as *const u8 as *const libc::c_char,
            );
            let fresh4 = iparam;
            iparam = iparam + 1;
            let ref mut fresh5 = *params.offset(fresh4 as isize);
            *fresh5 = tablespace;
            if num_tables != 0 || num_parent_tables != 0 {
                if num_tables != 0 {
                    appendPQExpBufferStr(
                        &mut sql,
                        b"(\0" as *const u8 as *const libc::c_char,
                    );
                    cell = table_list.head;
                    while !cell.is_null() {
                        appendPQExpBuffer(
                            &mut sql as *mut PQExpBufferData,
                            b"relid = $%d::regclass\0" as *const u8
                                as *const libc::c_char,
                            iparam + 1 as libc::c_int,
                        );
                        let fresh6 = iparam;
                        iparam = iparam + 1;
                        let ref mut fresh7 = *params.offset(fresh6 as isize);
                        *fresh7 = ((*cell).val).as_mut_ptr();
                        if !((*cell).next).is_null() {
                            appendPQExpBufferStr(
                                &mut sql,
                                b" OR \0" as *const u8 as *const libc::c_char,
                            );
                        }
                        cell = (*cell).next;
                    }
                    appendPQExpBufferStr(
                        &mut sql,
                        b")\0" as *const u8 as *const libc::c_char,
                    );
                }
                if num_tables != 0 && num_parent_tables != 0 {
                    appendPQExpBufferStr(
                        &mut sql,
                        b" OR \0" as *const u8 as *const libc::c_char,
                    );
                }
                if num_parent_tables != 0 {
                    appendPQExpBufferStr(
                        &mut sql,
                        b"(\0" as *const u8 as *const libc::c_char,
                    );
                    cell = parent_table_list.head;
                    while !cell.is_null() {
                        appendPQExpBuffer(
                            &mut sql as *mut PQExpBufferData,
                            b"relid = ANY(repack.get_table_and_inheritors($%d::regclass))\0"
                                as *const u8 as *const libc::c_char,
                            iparam + 1 as libc::c_int,
                        );
                        let fresh8 = iparam;
                        iparam = iparam + 1;
                        let ref mut fresh9 = *params.offset(fresh8 as isize);
                        *fresh9 = ((*cell).val).as_mut_ptr();
                        if !((*cell).next).is_null() {
                            appendPQExpBufferStr(
                                &mut sql,
                                b" OR \0" as *const u8 as *const libc::c_char,
                            );
                        }
                        cell = (*cell).next;
                    }
                    appendPQExpBufferStr(
                        &mut sql,
                        b")\0" as *const u8 as *const libc::c_char,
                    );
                }
            } else if num_schemas != 0 {
                appendPQExpBufferStr(
                    &mut sql,
                    b"schemaname IN (\0" as *const u8 as *const libc::c_char,
                );
                cell = schema_list.head;
                while !cell.is_null() {
                    appendPQExpBuffer(
                        &mut sql as *mut PQExpBufferData,
                        b"$%d\0" as *const u8 as *const libc::c_char,
                        iparam + 1 as libc::c_int,
                    );
                    let fresh10 = iparam;
                    iparam = iparam + 1;
                    let ref mut fresh11 = *params.offset(fresh10 as isize);
                    *fresh11 = ((*cell).val).as_mut_ptr();
                    if !((*cell).next).is_null() {
                        appendPQExpBufferStr(
                            &mut sql,
                            b", \0" as *const u8 as *const libc::c_char,
                        );
                    }
                    cell = (*cell).next;
                }
                appendPQExpBufferStr(
                    &mut sql,
                    b")\0" as *const u8 as *const libc::c_char,
                );
            } else {
                appendPQExpBufferStr(
                    &mut sql,
                    b"pkid IS NOT NULL\0" as *const u8 as *const libc::c_char,
                );
            }
            if !(exclude_extension_list.head).is_null() {
                appendPQExpBufferStr(
                    &mut sql,
                    b" AND t.relid NOT IN  (SELECT d.objid::regclass   FROM pg_depend d JOIN pg_extension e   ON d.refobjid = e.oid   WHERE d.classid = 'pg_class'::regclass AND (\0"
                        as *const u8 as *const libc::c_char,
                );
                cell = exclude_extension_list.head;
                while !cell.is_null() {
                    appendPQExpBuffer(
                        &mut sql as *mut PQExpBufferData,
                        b"e.extname = $%d\0" as *const u8 as *const libc::c_char,
                        iparam + 1 as libc::c_int,
                    );
                    let fresh12 = iparam;
                    iparam = iparam + 1;
                    let ref mut fresh13 = *params.offset(fresh12 as isize);
                    *fresh13 = ((*cell).val).as_mut_ptr();
                    appendPQExpBufferStr(
                        &mut sql,
                        if !((*cell).next).is_null() {
                            b" OR \0" as *const u8 as *const libc::c_char
                        } else {
                            b")\0" as *const u8 as *const libc::c_char
                        },
                    );
                    cell = (*cell).next;
                }
                appendPQExpBufferStr(
                    &mut sql,
                    b")\0" as *const u8 as *const libc::c_char,
                );
            }
            appendPQExpBufferStr(
                &mut sql,
                b" ORDER BY t.relname, t.schemaname\0" as *const u8
                    as *const libc::c_char,
            );
            if iparam as libc::c_ulong != num_params {
                if !errbuf.is_null() {
                    pg_snprintf(
                        errbuf,
                        errsize,
                        b"internal error: bad parameters count: %i instead of %zi\0"
                            as *const u8 as *const libc::c_char,
                        iparam,
                        num_params,
                    );
                }
            } else {
                res = execute_elevel(
                    sql.data,
                    num_params as libc::c_int,
                    params,
                    13 as libc::c_int,
                );
                if PQresultStatus(res) as libc::c_uint
                    != PGRES_TUPLES_OK as libc::c_int as libc::c_uint
                {
                    if !errbuf.is_null() {
                        pg_snprintf(
                            errbuf,
                            errsize,
                            b"%s\0" as *const u8 as *const libc::c_char,
                            PQerrorMessage(connection),
                        );
                    }
                } else {
                    num = PQntuples(res);
                    i = 0 as libc::c_int;
                    while i < num {
                        let mut table: repack_table = repack_table {
                            target_name: 0 as *const libc::c_char,
                            target_oid: 0,
                            target_toast: 0,
                            target_tidx: 0,
                            pkid: 0,
                            ckid: 0,
                            create_pktype: 0 as *const libc::c_char,
                            create_log: 0 as *const libc::c_char,
                            create_trigger: 0 as *const libc::c_char,
                            enable_trigger: 0 as *const libc::c_char,
                            create_table: 0 as *const libc::c_char,
                            copy_data: 0 as *const libc::c_char,
                            alter_col_storage: 0 as *const libc::c_char,
                            drop_columns: 0 as *const libc::c_char,
                            delete_log: 0 as *const libc::c_char,
                            lock_table: 0 as *const libc::c_char,
                            sql_peek: 0 as *const libc::c_char,
                            sql_insert: 0 as *const libc::c_char,
                            sql_delete: 0 as *const libc::c_char,
                            sql_update: 0 as *const libc::c_char,
                            sql_pop: 0 as *const libc::c_char,
                            n_indexes: 0,
                            indexes: 0 as *mut repack_index,
                        };
                        let mut copy_sql: PQExpBufferData = PQExpBufferData {
                            data: 0 as *mut libc::c_char,
                            len: 0,
                            maxlen: 0,
                        };
                        let mut create_table_1: *const libc::c_char = 0
                            as *const libc::c_char;
                        let mut create_table_2: *const libc::c_char = 0
                            as *const libc::c_char;
                        let mut tablespace_0: *const libc::c_char = 0
                            as *const libc::c_char;
                        let mut ckey: *const libc::c_char = 0 as *const libc::c_char;
                        let mut c: libc::c_int = 0 as libc::c_int;
                        let fresh14 = c;
                        c = c + 1;
                        table.target_name = getstr(res, i, fresh14);
                        let fresh15 = c;
                        c = c + 1;
                        table.target_oid = getoid(res, i, fresh15);
                        let fresh16 = c;
                        c = c + 1;
                        table.target_toast = getoid(res, i, fresh16);
                        let fresh17 = c;
                        c = c + 1;
                        table.target_tidx = getoid(res, i, fresh17);
                        c += 1;
                        c;
                        let fresh18 = c;
                        c = c + 1;
                        table.pkid = getoid(res, i, fresh18);
                        let fresh19 = c;
                        c = c + 1;
                        table.ckid = getoid(res, i, fresh19);
                        if table.pkid == 0 as libc::c_int as libc::c_uint {
                            if pgut_errstart(19 as libc::c_int) as libc::c_int != 0 {
                                pgut_errfinish(
                                    errcode(-(2 as libc::c_int)),
                                    errmsg(
                                        b"relation \"%s\" must have a primary key or not-null unique keys\0"
                                            as *const u8 as *const libc::c_char,
                                        table.target_name,
                                    ),
                                );
                            } else {};
                        } else {
                            let fresh20 = c;
                            c = c + 1;
                            table.create_pktype = getstr(res, i, fresh20);
                            let fresh21 = c;
                            c = c + 1;
                            table.create_log = getstr(res, i, fresh21);
                            let fresh22 = c;
                            c = c + 1;
                            table.create_trigger = getstr(res, i, fresh22);
                            let fresh23 = c;
                            c = c + 1;
                            table.enable_trigger = getstr(res, i, fresh23);
                            let fresh24 = c;
                            c = c + 1;
                            create_table_1 = getstr(res, i, fresh24);
                            let fresh25 = c;
                            c = c + 1;
                            tablespace_0 = getstr(res, i, fresh25);
                            let fresh26 = c;
                            c = c + 1;
                            create_table_2 = getstr(res, i, fresh26);
                            let fresh27 = c;
                            c = c + 1;
                            table.copy_data = getstr(res, i, fresh27);
                            let fresh28 = c;
                            c = c + 1;
                            table.alter_col_storage = getstr(res, i, fresh28);
                            let fresh29 = c;
                            c = c + 1;
                            table.drop_columns = getstr(res, i, fresh29);
                            let fresh30 = c;
                            c = c + 1;
                            table.delete_log = getstr(res, i, fresh30);
                            let fresh31 = c;
                            c = c + 1;
                            table.lock_table = getstr(res, i, fresh31);
                            let fresh32 = c;
                            c = c + 1;
                            ckey = getstr(res, i, fresh32);
                            let fresh33 = c;
                            c = c + 1;
                            table.sql_peek = getstr(res, i, fresh33);
                            let fresh34 = c;
                            c = c + 1;
                            table.sql_insert = getstr(res, i, fresh34);
                            let fresh35 = c;
                            c = c + 1;
                            table.sql_delete = getstr(res, i, fresh35);
                            let fresh36 = c;
                            c = c + 1;
                            table.sql_update = getstr(res, i, fresh36);
                            let fresh37 = c;
                            c = c + 1;
                            table.sql_pop = getstr(res, i, fresh37);
                            let fresh38 = c;
                            c = c + 1;
                            tablespace_0 = getstr(res, i, fresh38);
                            resetPQExpBuffer(&mut sql);
                            appendPQExpBufferStr(&mut sql, create_table_1);
                            appendPQExpBufferStr(&mut sql, tablespace_0);
                            appendPQExpBufferStr(&mut sql, create_table_2);
                            appendPQExpBufferStr(
                                &mut sql,
                                b" WITH NO DATA\0" as *const u8 as *const libc::c_char,
                            );
                            table.create_table = sql.data;
                            initPQExpBuffer(&mut copy_sql);
                            appendPQExpBufferStr(&mut copy_sql, table.copy_data);
                            if orderby_0.is_null() {
                                if !ckey.is_null() {
                                    appendPQExpBufferStr(
                                        &mut copy_sql,
                                        b" ORDER BY \0" as *const u8 as *const libc::c_char,
                                    );
                                    appendPQExpBufferStr(&mut copy_sql, ckey);
                                }
                            } else if !(*orderby_0.offset(0 as libc::c_int as isize)
                                == 0)
                            {
                                appendPQExpBufferStr(
                                    &mut copy_sql,
                                    b" ORDER BY \0" as *const u8 as *const libc::c_char,
                                );
                                appendPQExpBufferStr(&mut copy_sql, orderby_0);
                            }
                            table.copy_data = copy_sql.data;
                            repack_one_table(&mut table, orderby_0);
                        }
                        i += 1;
                        i;
                    }
                    ret = 1 as libc::c_int != 0;
                }
            }
        }
    }
    PQclear(res);
    res = 0 as *mut PGresult;
    disconnect();
    termPQExpBuffer(&mut sql);
    free(params as *mut libc::c_void);
    return ret;
}
unsafe extern "C" fn apply_log(
    mut conn: *mut PGconn,
    mut table: *const repack_table,
    mut count: libc::c_int,
) -> libc::c_int {
    let mut result: libc::c_int = 0;
    let mut res: *mut PGresult = 0 as *mut PGresult;
    let mut params: [*const libc::c_char; 6] = [0 as *const libc::c_char; 6];
    let mut buffer: [libc::c_char; 12] = [0; 12];
    params[0 as libc::c_int as usize] = (*table).sql_peek;
    params[1 as libc::c_int as usize] = (*table).sql_insert;
    params[2 as libc::c_int as usize] = (*table).sql_delete;
    params[3 as libc::c_int as usize] = (*table).sql_update;
    params[4 as libc::c_int as usize] = (*table).sql_pop;
    params[5 as libc::c_int as usize] = utoa(count as libc::c_uint, buffer.as_mut_ptr());
    res = pgut_execute(
        conn,
        b"SELECT repack.repack_apply($1, $2, $3, $4, $5, $6)\0" as *const u8
            as *const libc::c_char,
        6 as libc::c_int,
        params.as_mut_ptr(),
    );
    result = atoi(PQgetvalue(res, 0 as libc::c_int, 0 as libc::c_int));
    PQclear(res);
    res = 0 as *mut PGresult;
    return result;
}
unsafe extern "C" fn rebuild_indexes(mut table: *const repack_table) -> bool {
    let mut current_block: u64;
    let mut res: *mut PGresult = 0 as *mut PGresult;
    let mut num_indexes: libc::c_int = 0;
    let mut i: libc::c_int = 0;
    let mut num_active_workers: libc::c_int = 0;
    let mut num_workers: libc::c_int = 0;
    let mut index_jobs: *mut repack_index = 0 as *mut repack_index;
    let mut have_error: bool = 0 as libc::c_int != 0;
    elog(
        13 as libc::c_int,
        b"---- create indexes ----\0" as *const u8 as *const libc::c_char,
    );
    num_indexes = (*table).n_indexes;
    num_workers = if num_indexes > workers.num_workers {
        workers.num_workers
    } else {
        num_indexes
    };
    num_active_workers = num_workers;
    elog(
        13 as libc::c_int,
        b"Have %d indexes and num_workers=%d\0" as *const u8 as *const libc::c_char,
        num_indexes,
        num_workers,
    );
    index_jobs = (*table).indexes;
    i = 0 as libc::c_int;
    loop {
        if !(i < num_indexes) {
            current_block = 7175849428784450219;
            break;
        }
        elog(
            13 as libc::c_int,
            b"set up index_jobs [%d]\0" as *const u8 as *const libc::c_char,
            i,
        );
        elog(
            13 as libc::c_int,
            b"target_oid   : %u\0" as *const u8 as *const libc::c_char,
            (*index_jobs.offset(i as isize)).target_oid,
        );
        elog(
            13 as libc::c_int,
            b"create_index : %s\0" as *const u8 as *const libc::c_char,
            (*index_jobs.offset(i as isize)).create_index,
        );
        if num_workers <= 1 as libc::c_int {
            command(
                (*index_jobs.offset(i as isize)).create_index,
                0 as libc::c_int,
                0 as *mut *const libc::c_char,
            );
            (*index_jobs.offset(i as isize)).status = FINISHED;
        } else if i < num_workers {
            (*index_jobs.offset(i as isize)).status = INPROGRESS;
            (*index_jobs.offset(i as isize)).worker_idx = i;
            elog(
                15 as libc::c_int,
                b"Initial worker %d to build index: %s\0" as *const u8
                    as *const libc::c_char,
                i,
                (*index_jobs.offset(i as isize)).create_index,
            );
            if PQsendQuery(
                *(workers.conns).offset(i as isize),
                (*index_jobs.offset(i as isize)).create_index,
            ) == 0
            {
                elog(
                    19 as libc::c_int,
                    b"Error sending async query: %s\n%s\0" as *const u8
                        as *const libc::c_char,
                    (*index_jobs.offset(i as isize)).create_index,
                    PQerrorMessage(*(workers.conns).offset(i as isize)),
                );
                have_error = 1 as libc::c_int != 0;
                current_block = 3021958063889393925;
                break;
            }
        }
        i += 1;
        i;
    }
    match current_block {
        7175849428784450219 => {
            if num_workers > 1 as libc::c_int {
                let mut freed_worker: libc::c_int = -(1 as libc::c_int);
                let mut ret: libc::c_int = 0;
                let mut input_fds: *mut pollfd = 0 as *mut pollfd;
                input_fds = pgut_malloc(
                    (::std::mem::size_of::<pollfd>() as libc::c_ulong)
                        .wrapping_mul(num_workers as libc::c_ulong),
                ) as *mut pollfd;
                i = 0 as libc::c_int;
                while i < num_workers {
                    (*input_fds.offset(i as isize))
                        .fd = PQsocket(*(workers.conns).offset(i as isize));
                    (*input_fds.offset(i as isize))
                        .events = (0x1 as libc::c_int | 0x8 as libc::c_int)
                        as libc::c_short;
                    (*input_fds.offset(i as isize))
                        .revents = 0 as libc::c_int as libc::c_short;
                    i += 1;
                    i;
                }
                's_135: while num_active_workers > 0 as libc::c_int {
                    elog(
                        13 as libc::c_int,
                        b"polling %d active workers\0" as *const u8
                            as *const libc::c_char,
                        num_active_workers,
                    );
                    ret = poll(
                        input_fds,
                        num_workers as nfds_t,
                        3 as libc::c_int * 1000 as libc::c_int,
                    );
                    if ret < 0 as libc::c_int && *__errno_location() != 4 as libc::c_int
                    {
                        elog(
                            20 as libc::c_int,
                            b"poll() failed: %d, %d\0" as *const u8
                                as *const libc::c_char,
                            ret,
                            *__errno_location(),
                        );
                    }
                    elog(
                        13 as libc::c_int,
                        b"Poll returned: %d\0" as *const u8 as *const libc::c_char,
                        ret,
                    );
                    i = 0 as libc::c_int;
                    while i < num_indexes {
                        if (*index_jobs.offset(i as isize)).status as libc::c_uint
                            == INPROGRESS as libc::c_int as libc::c_uint
                        {
                            if PQconsumeInput(
                                *(workers.conns)
                                    .offset(
                                        (*index_jobs.offset(i as isize)).worker_idx as isize,
                                    ),
                            ) != 1 as libc::c_int
                            {
                                elog(
                                    19 as libc::c_int,
                                    b"Error fetching async query status: %s\0" as *const u8
                                        as *const libc::c_char,
                                    PQerrorMessage(
                                        *(workers.conns)
                                            .offset(
                                                (*index_jobs.offset(i as isize)).worker_idx as isize,
                                            ),
                                    ),
                                );
                                have_error = 1 as libc::c_int != 0;
                                break 's_135;
                            } else if PQisBusy(
                                *(workers.conns)
                                    .offset(
                                        (*index_jobs.offset(i as isize)).worker_idx as isize,
                                    ),
                            ) == 0
                            {
                                elog(
                                    15 as libc::c_int,
                                    b"Command finished in worker %d: %s\0" as *const u8
                                        as *const libc::c_char,
                                    (*index_jobs.offset(i as isize)).worker_idx,
                                    (*index_jobs.offset(i as isize)).create_index,
                                );
                                loop {
                                    res = PQgetResult(
                                        *(workers.conns)
                                            .offset(
                                                (*index_jobs.offset(i as isize)).worker_idx as isize,
                                            ),
                                    );
                                    if res.is_null() {
                                        break;
                                    }
                                    if PQresultStatus(res) as libc::c_uint
                                        != PGRES_COMMAND_OK as libc::c_int as libc::c_uint
                                    {
                                        elog(
                                            19 as libc::c_int,
                                            b"Error with create index: %s\0" as *const u8
                                                as *const libc::c_char,
                                            PQerrorMessage(
                                                *(workers.conns)
                                                    .offset(
                                                        (*index_jobs.offset(i as isize)).worker_idx as isize,
                                                    ),
                                            ),
                                        );
                                        have_error = 1 as libc::c_int != 0;
                                        break 's_135;
                                    } else {
                                        PQclear(res);
                                        res = 0 as *mut PGresult;
                                    }
                                }
                                freed_worker = (*index_jobs.offset(i as isize)).worker_idx;
                                (*index_jobs.offset(i as isize)).status = FINISHED;
                                num_active_workers -= 1;
                                num_active_workers;
                                break;
                            }
                        }
                        i += 1;
                        i;
                    }
                    if !(freed_worker > -(1 as libc::c_int)) {
                        continue;
                    }
                    i = 0 as libc::c_int;
                    while i < num_indexes {
                        if (*index_jobs.offset(i as isize)).status as libc::c_uint
                            == UNPROCESSED as libc::c_int as libc::c_uint
                        {
                            (*index_jobs.offset(i as isize)).status = INPROGRESS;
                            (*index_jobs.offset(i as isize)).worker_idx = freed_worker;
                            elog(
                                15 as libc::c_int,
                                b"Assigning worker %d to build index #%d: %s\0" as *const u8
                                    as *const libc::c_char,
                                freed_worker,
                                i,
                                (*index_jobs.offset(i as isize)).create_index,
                            );
                            if PQsendQuery(
                                *(workers.conns).offset(freed_worker as isize),
                                (*index_jobs.offset(i as isize)).create_index,
                            ) == 0
                            {
                                elog(
                                    19 as libc::c_int,
                                    b"Error sending async query: %s\n%s\0" as *const u8
                                        as *const libc::c_char,
                                    (*index_jobs.offset(i as isize)).create_index,
                                    PQerrorMessage(
                                        *(workers.conns).offset(freed_worker as isize),
                                    ),
                                );
                                have_error = 1 as libc::c_int != 0;
                                break 's_135;
                            } else {
                                num_active_workers += 1;
                                num_active_workers;
                                break;
                            }
                        } else {
                            i += 1;
                            i;
                        }
                    }
                    freed_worker = -(1 as libc::c_int);
                }
            }
        }
        _ => {}
    }
    PQclear(res);
    res = 0 as *mut PGresult;
    return !have_error;
}
unsafe extern "C" fn repack_one_table(
    mut table: *mut repack_table,
    mut orderby_0: *const libc::c_char,
) {
    let mut current_block: u64;
    let mut res: *mut PGresult = 0 as *mut PGresult;
    let mut params: [*const libc::c_char; 3] = [0 as *const libc::c_char; 3];
    let mut num: libc::c_int = 0;
    let mut vxid: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut buffer: [libc::c_char; 12] = [0; 12];
    let mut sql: PQExpBufferData = PQExpBufferData {
        data: 0 as *mut libc::c_char,
        len: 0,
        maxlen: 0,
    };
    let mut ret: bool = 0 as libc::c_int != 0;
    let mut indexres: *mut PGresult = 0 as *mut PGresult;
    let mut indexparams: [*const libc::c_char; 2] = [0 as *const libc::c_char; 2];
    let mut indexbuffer: [libc::c_char; 12] = [0; 12];
    let mut j: libc::c_int = 0;
    let mut appname: *const libc::c_char = getenv(
        b"PGAPPNAME\0" as *const u8 as *const libc::c_char,
    );
    let mut table_init: bool = 0 as libc::c_int != 0;
    initPQExpBuffer(&mut sql);
    elog(
        17 as libc::c_int,
        b"repacking table \"%s\"\0" as *const u8 as *const libc::c_char,
        (*table).target_name,
    );
    elog(
        13 as libc::c_int,
        b"---- repack_one_table ----\0" as *const u8 as *const libc::c_char,
    );
    elog(
        13 as libc::c_int,
        b"target_name       : %s\0" as *const u8 as *const libc::c_char,
        (*table).target_name,
    );
    elog(
        13 as libc::c_int,
        b"target_oid        : %u\0" as *const u8 as *const libc::c_char,
        (*table).target_oid,
    );
    elog(
        13 as libc::c_int,
        b"target_toast      : %u\0" as *const u8 as *const libc::c_char,
        (*table).target_toast,
    );
    elog(
        13 as libc::c_int,
        b"target_tidx       : %u\0" as *const u8 as *const libc::c_char,
        (*table).target_tidx,
    );
    elog(
        13 as libc::c_int,
        b"pkid              : %u\0" as *const u8 as *const libc::c_char,
        (*table).pkid,
    );
    elog(
        13 as libc::c_int,
        b"ckid              : %u\0" as *const u8 as *const libc::c_char,
        (*table).ckid,
    );
    elog(
        13 as libc::c_int,
        b"create_pktype     : %s\0" as *const u8 as *const libc::c_char,
        (*table).create_pktype,
    );
    elog(
        13 as libc::c_int,
        b"create_log        : %s\0" as *const u8 as *const libc::c_char,
        (*table).create_log,
    );
    elog(
        13 as libc::c_int,
        b"create_trigger    : %s\0" as *const u8 as *const libc::c_char,
        (*table).create_trigger,
    );
    elog(
        13 as libc::c_int,
        b"enable_trigger    : %s\0" as *const u8 as *const libc::c_char,
        (*table).enable_trigger,
    );
    elog(
        13 as libc::c_int,
        b"create_table      : %s\0" as *const u8 as *const libc::c_char,
        (*table).create_table,
    );
    elog(
        13 as libc::c_int,
        b"copy_data         : %s\0" as *const u8 as *const libc::c_char,
        (*table).copy_data,
    );
    elog(
        13 as libc::c_int,
        b"alter_col_storage : %s\0" as *const u8 as *const libc::c_char,
        if !((*table).alter_col_storage).is_null() {
            (*table).alter_col_storage
        } else {
            b"(skipped)\0" as *const u8 as *const libc::c_char
        },
    );
    elog(
        13 as libc::c_int,
        b"drop_columns      : %s\0" as *const u8 as *const libc::c_char,
        if !((*table).drop_columns).is_null() {
            (*table).drop_columns
        } else {
            b"(skipped)\0" as *const u8 as *const libc::c_char
        },
    );
    elog(
        13 as libc::c_int,
        b"delete_log        : %s\0" as *const u8 as *const libc::c_char,
        (*table).delete_log,
    );
    elog(
        13 as libc::c_int,
        b"lock_table        : %s\0" as *const u8 as *const libc::c_char,
        (*table).lock_table,
    );
    elog(
        13 as libc::c_int,
        b"sql_peek          : %s\0" as *const u8 as *const libc::c_char,
        (*table).sql_peek,
    );
    elog(
        13 as libc::c_int,
        b"sql_insert        : %s\0" as *const u8 as *const libc::c_char,
        (*table).sql_insert,
    );
    elog(
        13 as libc::c_int,
        b"sql_delete        : %s\0" as *const u8 as *const libc::c_char,
        (*table).sql_delete,
    );
    elog(
        13 as libc::c_int,
        b"sql_update        : %s\0" as *const u8 as *const libc::c_char,
        (*table).sql_update,
    );
    elog(
        13 as libc::c_int,
        b"sql_pop           : %s\0" as *const u8 as *const libc::c_char,
        (*table).sql_pop,
    );
    if dryrun {
        return;
    }
    pgut_atexit_push(
        Some(
            repack_cleanup_callback
                as unsafe extern "C" fn(bool, *mut libc::c_void) -> (),
        ),
        &mut (*table).target_oid as *mut Oid as *mut libc::c_void,
    );
    elog(13 as libc::c_int, b"---- setup ----\0" as *const u8 as *const libc::c_char);
    params[0 as libc::c_int as usize] = utoa((*table).target_oid, buffer.as_mut_ptr());
    if advisory_lock(connection, buffer.as_mut_ptr()) {
        if !lock_exclusive(
            connection,
            buffer.as_mut_ptr(),
            (*table).lock_table,
            1 as libc::c_int != 0,
        ) {
            if no_kill_backend {
                elog(
                    17 as libc::c_int,
                    b"Skipping repack %s due to timeout\0" as *const u8
                        as *const libc::c_char,
                    (*table).target_name,
                );
            } else {
                elog(
                    19 as libc::c_int,
                    b"lock_exclusive() failed for %s\0" as *const u8
                        as *const libc::c_char,
                    (*table).target_name,
                );
            }
        } else {
            indexparams[0 as libc::c_int
                as usize] = utoa((*table).target_oid, indexbuffer.as_mut_ptr());
            indexparams[1 as libc::c_int
                as usize] = if moveidx as libc::c_int != 0 {
                tablespace
            } else {
                0 as *mut libc::c_char
            };
            indexres = execute(
                b"SELECT pg_get_indexdef(indexrelid) FROM pg_index WHERE indrelid = $1 AND NOT indisvalid\0"
                    as *const u8 as *const libc::c_char,
                1 as libc::c_int,
                indexparams.as_mut_ptr(),
            );
            j = 0 as libc::c_int;
            while j < PQntuples(indexres) {
                let mut indexdef: *const libc::c_char = 0 as *const libc::c_char;
                indexdef = getstr(indexres, j, 0 as libc::c_int);
                elog(
                    19 as libc::c_int,
                    b"skipping invalid index: %s\0" as *const u8 as *const libc::c_char,
                    indexdef,
                );
                j += 1;
                j;
            }
            indexres = execute(
                b"SELECT indexrelid, repack.repack_indexdef(indexrelid, indrelid, $2, FALSE)  FROM pg_index WHERE indrelid = $1 AND indisvalid\0"
                    as *const u8 as *const libc::c_char,
                2 as libc::c_int,
                indexparams.as_mut_ptr(),
            );
            (*table).n_indexes = PQntuples(indexres);
            (*table)
                .indexes = pgut_malloc(
                ((*table).n_indexes as libc::c_ulong)
                    .wrapping_mul(::std::mem::size_of::<repack_index>() as libc::c_ulong),
            ) as *mut repack_index;
            j = 0 as libc::c_int;
            while j < (*table).n_indexes {
                (*((*table).indexes).offset(j as isize))
                    .target_oid = getoid(indexres, j, 0 as libc::c_int);
                let ref mut fresh39 = (*((*table).indexes).offset(j as isize))
                    .create_index;
                *fresh39 = getstr(indexres, j, 1 as libc::c_int);
                (*((*table).indexes).offset(j as isize)).status = UNPROCESSED;
                (*((*table).indexes).offset(j as isize))
                    .worker_idx = -(1 as libc::c_int);
                j += 1;
                j;
            }
            j = 0 as libc::c_int;
            while j < (*table).n_indexes {
                elog(
                    13 as libc::c_int,
                    b"index[%d].target_oid      : %u\0" as *const u8
                        as *const libc::c_char,
                    j,
                    (*((*table).indexes).offset(j as isize)).target_oid,
                );
                elog(
                    13 as libc::c_int,
                    b"index[%d].create_index    : %s\0" as *const u8
                        as *const libc::c_char,
                    j,
                    (*((*table).indexes).offset(j as isize)).create_index,
                );
                j += 1;
                j;
            }
            res = execute(
                b"SELECT repack.conflicted_triggers($1)\0" as *const u8
                    as *const libc::c_char,
                1 as libc::c_int,
                params.as_mut_ptr(),
            );
            if PQntuples(res) > 0 as libc::c_int {
                if pgut_errstart(19 as libc::c_int) as libc::c_int != 0 {
                    pgut_errfinish(
                        errcode(-(2 as libc::c_int)),
                        errmsg(
                            b"the table \"%s\" already has a trigger called \"%s\"\0"
                                as *const u8 as *const libc::c_char,
                            (*table).target_name,
                            b"repack_trigger\0" as *const u8 as *const libc::c_char,
                        ),
                        errdetail(
                            b"The trigger was probably installed during a previous attempt to run pg_repack on the table which was interrupted and for some reason failed to clean up the temporary objects.  Please drop the trigger or drop and recreate the pg_repack extension altogether to remove all the temporary objects left over.\0"
                                as *const u8 as *const libc::c_char,
                        ),
                    );
                } else {};
            } else {
                PQclear(res);
                res = 0 as *mut PGresult;
                command(
                    (*table).create_pktype,
                    0 as libc::c_int,
                    0 as *mut *const libc::c_char,
                );
                temp_obj_num = temp_obj_num.wrapping_add(1);
                temp_obj_num;
                command(
                    (*table).create_log,
                    0 as libc::c_int,
                    0 as *mut *const libc::c_char,
                );
                temp_obj_num = temp_obj_num.wrapping_add(1);
                temp_obj_num;
                command(
                    (*table).create_trigger,
                    0 as libc::c_int,
                    0 as *mut *const libc::c_char,
                );
                temp_obj_num = temp_obj_num.wrapping_add(1);
                temp_obj_num;
                command(
                    (*table).enable_trigger,
                    0 as libc::c_int,
                    0 as *mut *const libc::c_char,
                );
                printfPQExpBuffer(
                    &mut sql as *mut PQExpBufferData,
                    b"SELECT repack.disable_autovacuum('repack.log_%u')\0" as *const u8
                        as *const libc::c_char,
                    (*table).target_oid,
                );
                command(sql.data, 0 as libc::c_int, 0 as *mut *const libc::c_char);
                pgut_command(
                    conn2,
                    b"BEGIN ISOLATION LEVEL READ COMMITTED\0" as *const u8
                        as *const libc::c_char,
                    0 as libc::c_int,
                    0 as *mut *const libc::c_char,
                );
                res = pgut_execute(
                    conn2,
                    b"SELECT pg_backend_pid()\0" as *const u8 as *const libc::c_char,
                    0 as libc::c_int,
                    0 as *mut *const libc::c_char,
                );
                buffer[0 as libc::c_int as usize] = '\0' as i32 as libc::c_char;
                strncat(
                    buffer.as_mut_ptr(),
                    PQgetvalue(res, 0 as libc::c_int, 0 as libc::c_int),
                    (::std::mem::size_of::<[libc::c_char; 12]>() as libc::c_ulong)
                        .wrapping_sub(1 as libc::c_int as libc::c_ulong),
                );
                PQclear(res);
                res = 0 as *mut PGresult;
                printfPQExpBuffer(
                    &mut sql as *mut PQExpBufferData,
                    b"LOCK TABLE %s IN ACCESS SHARE MODE\0" as *const u8
                        as *const libc::c_char,
                    (*table).target_name,
                );
                elog(
                    13 as libc::c_int,
                    b"LOCK TABLE %s IN ACCESS SHARE MODE\0" as *const u8
                        as *const libc::c_char,
                    (*table).target_name,
                );
                if PQsetnonblocking(conn2, 1 as libc::c_int) != 0 {
                    elog(
                        19 as libc::c_int,
                        b"Unable to set conn2 nonblocking.\0" as *const u8
                            as *const libc::c_char,
                    );
                } else if PQsendQuery(conn2, sql.data) == 0 {
                    elog(
                        19 as libc::c_int,
                        b"Error sending async query: %s\n%s\0" as *const u8
                            as *const libc::c_char,
                        sql.data,
                        PQerrorMessage(conn2),
                    );
                } else if !kill_ddl(
                    connection,
                    (*table).target_oid,
                    1 as libc::c_int != 0,
                ) {
                    if no_kill_backend {
                        elog(
                            17 as libc::c_int,
                            b"Skipping repack %s due to timeout.\0" as *const u8
                                as *const libc::c_char,
                            (*table).target_name,
                        );
                    } else {
                        elog(
                            19 as libc::c_int,
                            b"kill_ddl() failed.\0" as *const u8 as *const libc::c_char,
                        );
                    }
                } else {
                    command(
                        b"COMMIT\0" as *const u8 as *const libc::c_char,
                        0 as libc::c_int,
                        0 as *mut *const libc::c_char,
                    );
                    table_init = 1 as libc::c_int != 0;
                    loop {
                        res = PQgetResult(conn2);
                        if res.is_null() {
                            current_block = 16108440464692313034;
                            break;
                        }
                        elog(
                            13 as libc::c_int,
                            b"Waiting on ACCESS SHARE lock...\0" as *const u8
                                as *const libc::c_char,
                        );
                        if PQresultStatus(res) as libc::c_uint
                            != PGRES_COMMAND_OK as libc::c_int as libc::c_uint
                        {
                            elog(
                                19 as libc::c_int,
                                b"Error with LOCK TABLE: %s\0" as *const u8
                                    as *const libc::c_char,
                                PQerrorMessage(conn2),
                            );
                            current_block = 8952145992953240062;
                            break;
                        } else {
                            PQclear(res);
                            res = 0 as *mut PGresult;
                        }
                    }
                    match current_block {
                        8952145992953240062 => {}
                        _ => {
                            if PQsetnonblocking(conn2, 0 as libc::c_int) != 0 {
                                elog(
                                    19 as libc::c_int,
                                    b"Unable to set conn2 blocking.\0" as *const u8
                                        as *const libc::c_char,
                                );
                            } else {
                                elog(
                                    13 as libc::c_int,
                                    b"---- copy tuples ----\0" as *const u8
                                        as *const libc::c_char,
                                );
                                command(
                                    b"BEGIN ISOLATION LEVEL SERIALIZABLE\0" as *const u8
                                        as *const libc::c_char,
                                    0 as libc::c_int,
                                    0 as *mut *const libc::c_char,
                                );
                                command(
                                    b"SELECT set_config('work_mem', current_setting('maintenance_work_mem'), true)\0"
                                        as *const u8 as *const libc::c_char,
                                    0 as libc::c_int,
                                    0 as *mut *const libc::c_char,
                                );
                                if !orderby_0.is_null()
                                    && *orderby_0.offset(0 as libc::c_int as isize) == 0
                                {
                                    command(
                                        b"SET LOCAL synchronize_seqscans = off\0" as *const u8
                                            as *const libc::c_char,
                                        0 as libc::c_int,
                                        0 as *mut *const libc::c_char,
                                    );
                                }
                                params[0 as libc::c_int as usize] = buffer.as_mut_ptr();
                                params[1 as libc::c_int as usize] = PROGRAM_NAME;
                                res = execute(
                                    if PQserverVersion(connection) >= 90200 as libc::c_int {
                                        b"SELECT coalesce(array_agg(l.virtualtransaction), '{}')   FROM pg_locks AS l   LEFT JOIN pg_stat_activity AS a     ON l.pid = a.pid   LEFT JOIN pg_database AS d     ON a.datid = d.oid   WHERE l.locktype = 'virtualxid'   AND l.pid NOT IN (pg_backend_pid(), $1)   AND (l.virtualxid, l.virtualtransaction) <> ('1/1', '-1/0')   AND (a.application_name IS NULL OR a.application_name <> $2)  AND a.query !~* E'^\\\\s*vacuum\\\\s+'   AND a.query !~ E'^autovacuum: '   AND ((d.datname IS NULL OR d.datname = current_database()) OR l.database = 0)\0"
                                            as *const u8 as *const libc::c_char
                                    } else if PQserverVersion(connection)
                                        >= 90000 as libc::c_int
                                    {
                                        b"SELECT coalesce(array_agg(l.virtualtransaction), '{}')   FROM pg_locks AS l   LEFT JOIN pg_stat_activity AS a     ON l.pid = a.procpid   LEFT JOIN pg_database AS d     ON a.datid = d.oid   WHERE l.locktype = 'virtualxid'   AND l.pid NOT IN (pg_backend_pid(), $1)   AND (l.virtualxid, l.virtualtransaction) <> ('1/1', '-1/0')   AND (a.application_name IS NULL OR a.application_name <> $2)  AND a.current_query !~* E'^\\\\s*vacuum\\\\s+'   AND a.current_query !~ E'^autovacuum: '   AND ((d.datname IS NULL OR d.datname = current_database()) OR l.database = 0)\0"
                                            as *const u8 as *const libc::c_char
                                    } else {
                                        b"SELECT coalesce(array_agg(l.virtualtransaction), '{}')   FROM pg_locks AS l  LEFT JOIN pg_stat_activity AS a     ON l.pid = a.procpid   LEFT JOIN pg_database AS d     ON a.datid = d.oid  WHERE l.locktype = 'virtualxid' AND l.pid NOT IN (pg_backend_pid(), $1) AND (l.virtualxid, l.virtualtransaction) <> ('1/1', '-1/0')  AND a.current_query !~* E'^\\\\s*vacuum\\\\s+'  AND a.current_query !~ E'^autovacuum: '  AND ((d.datname IS NULL OR d.datname = current_database()) OR l.database = 0) AND ($2::text IS NOT NULL)\0"
                                            as *const u8 as *const libc::c_char
                                    },
                                    2 as libc::c_int,
                                    params.as_mut_ptr(),
                                );
                                vxid = pgut_strdup(
                                    PQgetvalue(res, 0 as libc::c_int, 0 as libc::c_int),
                                );
                                PQclear(res);
                                res = 0 as *mut PGresult;
                                command(
                                    (*table).delete_log,
                                    0 as libc::c_int,
                                    0 as *mut *const libc::c_char,
                                );
                                if lock_access_share(
                                    connection,
                                    (*table).target_oid,
                                    (*table).target_name,
                                ) {
                                    command(
                                        (*table).create_table,
                                        0 as libc::c_int,
                                        0 as *mut *const libc::c_char,
                                    );
                                    if !((*table).alter_col_storage).is_null() {
                                        command(
                                            (*table).alter_col_storage,
                                            0 as libc::c_int,
                                            0 as *mut *const libc::c_char,
                                        );
                                    }
                                    command(
                                        (*table).copy_data,
                                        0 as libc::c_int,
                                        0 as *mut *const libc::c_char,
                                    );
                                    temp_obj_num = temp_obj_num.wrapping_add(1);
                                    temp_obj_num;
                                    printfPQExpBuffer(
                                        &mut sql as *mut PQExpBufferData,
                                        b"SELECT repack.disable_autovacuum('repack.table_%u')\0"
                                            as *const u8 as *const libc::c_char,
                                        (*table).target_oid,
                                    );
                                    if !((*table).drop_columns).is_null() {
                                        command(
                                            (*table).drop_columns,
                                            0 as libc::c_int,
                                            0 as *mut *const libc::c_char,
                                        );
                                    }
                                    command(
                                        sql.data,
                                        0 as libc::c_int,
                                        0 as *mut *const libc::c_char,
                                    );
                                    command(
                                        b"COMMIT\0" as *const u8 as *const libc::c_char,
                                        0 as libc::c_int,
                                        0 as *mut *const libc::c_char,
                                    );
                                    if rebuild_indexes(table) {
                                        PQclear(indexres);
                                        indexres = 0 as *mut PGresult;
                                        PQclear(res);
                                        res = 0 as *mut PGresult;
                                        loop {
                                            num = apply_log(connection, table, 1000 as libc::c_int);
                                            if num > 20 as libc::c_int {
                                                continue;
                                            }
                                            params[0 as libc::c_int as usize] = vxid;
                                            res = execute(
                                                b"SELECT pid FROM pg_locks WHERE locktype = 'virtualxid' AND pid <> pg_backend_pid() AND virtualtransaction = ANY($1)\0"
                                                    as *const u8 as *const libc::c_char,
                                                1 as libc::c_int,
                                                params.as_mut_ptr(),
                                            );
                                            num = PQntuples(res);
                                            if num > 0 as libc::c_int {
                                                if appname.is_null()
                                                    || strcmp(
                                                        appname,
                                                        b"pg_regress\0" as *const u8 as *const libc::c_char,
                                                    ) != 0 as libc::c_int
                                                {
                                                    elog(
                                                        18 as libc::c_int,
                                                        b"Waiting for %d transactions to finish. First PID: %s\0"
                                                            as *const u8 as *const libc::c_char,
                                                        num,
                                                        PQgetvalue(res, 0 as libc::c_int, 0 as libc::c_int),
                                                    );
                                                }
                                                PQclear(res);
                                                res = 0 as *mut PGresult;
                                                sleep(1 as libc::c_int as libc::c_uint);
                                            } else {
                                                PQclear(res);
                                                res = 0 as *mut PGresult;
                                                break;
                                            }
                                        }
                                        elog(
                                            13 as libc::c_int,
                                            b"---- swap ----\0" as *const u8 as *const libc::c_char,
                                        );
                                        if !lock_exclusive(
                                            conn2,
                                            utoa((*table).target_oid, buffer.as_mut_ptr()),
                                            (*table).lock_table,
                                            0 as libc::c_int != 0,
                                        ) {
                                            elog(
                                                19 as libc::c_int,
                                                b"lock_exclusive() failed in conn2 for %s\0" as *const u8
                                                    as *const libc::c_char,
                                                (*table).target_name,
                                            );
                                        } else {
                                            apply_log(conn2, table, 0 as libc::c_int);
                                            params[0 as libc::c_int
                                                as usize] = utoa((*table).target_oid, buffer.as_mut_ptr());
                                            pgut_command(
                                                conn2,
                                                b"SELECT repack.repack_swap($1)\0" as *const u8
                                                    as *const libc::c_char,
                                                1 as libc::c_int,
                                                params.as_mut_ptr(),
                                            );
                                            pgut_command(
                                                conn2,
                                                b"COMMIT\0" as *const u8 as *const libc::c_char,
                                                0 as libc::c_int,
                                                0 as *mut *const libc::c_char,
                                            );
                                            elog(
                                                13 as libc::c_int,
                                                b"---- drop ----\0" as *const u8 as *const libc::c_char,
                                            );
                                            command(
                                                b"BEGIN ISOLATION LEVEL READ COMMITTED\0" as *const u8
                                                    as *const libc::c_char,
                                                0 as libc::c_int,
                                                0 as *mut *const libc::c_char,
                                            );
                                            if !lock_exclusive(
                                                connection,
                                                utoa((*table).target_oid, buffer.as_mut_ptr()),
                                                (*table).lock_table,
                                                0 as libc::c_int != 0,
                                            ) {
                                                elog(
                                                    19 as libc::c_int,
                                                    b"lock_exclusive() failed in connection for %s\0"
                                                        as *const u8 as *const libc::c_char,
                                                    (*table).target_name,
                                                );
                                            } else {
                                                params[1 as libc::c_int
                                                    as usize] = utoa(temp_obj_num, indexbuffer.as_mut_ptr());
                                                command(
                                                    b"SELECT repack.repack_drop($1, $2)\0" as *const u8
                                                        as *const libc::c_char,
                                                    2 as libc::c_int,
                                                    params.as_mut_ptr(),
                                                );
                                                command(
                                                    b"COMMIT\0" as *const u8 as *const libc::c_char,
                                                    0 as libc::c_int,
                                                    0 as *mut *const libc::c_char,
                                                );
                                                temp_obj_num = 0 as libc::c_int as libc::c_uint;
                                                if analyze {
                                                    elog(
                                                        13 as libc::c_int,
                                                        b"---- analyze ----\0" as *const u8 as *const libc::c_char,
                                                    );
                                                    command(
                                                        b"BEGIN ISOLATION LEVEL READ COMMITTED\0" as *const u8
                                                            as *const libc::c_char,
                                                        0 as libc::c_int,
                                                        0 as *mut *const libc::c_char,
                                                    );
                                                    printfPQExpBuffer(
                                                        &mut sql as *mut PQExpBufferData,
                                                        b"ANALYZE %s\0" as *const u8 as *const libc::c_char,
                                                        (*table).target_name,
                                                    );
                                                    command(
                                                        sql.data,
                                                        0 as libc::c_int,
                                                        0 as *mut *const libc::c_char,
                                                    );
                                                    command(
                                                        b"COMMIT\0" as *const u8 as *const libc::c_char,
                                                        0 as libc::c_int,
                                                        0 as *mut *const libc::c_char,
                                                    );
                                                }
                                                params[0 as libc::c_int
                                                    as usize] = b"16185446\0" as *const u8
                                                    as *const libc::c_char;
                                                params[1 as libc::c_int
                                                    as usize] = utoa((*table).target_oid, buffer.as_mut_ptr());
                                                res = pgut_execute(
                                                    connection,
                                                    b"SELECT pg_advisory_unlock($1, CAST(-2147483648 + $2::bigint AS integer))\0"
                                                        as *const u8 as *const libc::c_char,
                                                    2 as libc::c_int,
                                                    params.as_mut_ptr(),
                                                );
                                                ret = 1 as libc::c_int != 0;
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
    PQclear(res);
    res = 0 as *mut PGresult;
    termPQExpBuffer(&mut sql);
    if !vxid.is_null() {
        free(vxid as *mut libc::c_void);
    }
    pgut_rollback(connection);
    pgut_rollback(conn2);
    if !ret && table_init as libc::c_int != 0 {
        repack_cleanup(0 as libc::c_int != 0, table);
    }
}
unsafe extern "C" fn kill_ddl(
    mut conn: *mut PGconn,
    mut relid: Oid,
    mut terminate: bool,
) -> bool {
    let mut ret: bool = 1 as libc::c_int != 0;
    let mut res: *mut PGresult = 0 as *mut PGresult;
    let mut sql: PQExpBufferData = PQExpBufferData {
        data: 0 as *mut libc::c_char,
        len: 0,
        maxlen: 0,
    };
    let mut n_tuples: libc::c_int = 0;
    initPQExpBuffer(&mut sql);
    printfPQExpBuffer(
        &mut sql as *mut PQExpBufferData,
        b"SELECT pid FROM pg_locks WHERE locktype = 'relation' AND granted = false AND relation = %u AND mode = 'AccessExclusiveLock' AND pid <> pg_backend_pid()\0"
            as *const u8 as *const libc::c_char,
        relid,
    );
    res = pgut_execute(conn, sql.data, 0 as libc::c_int, 0 as *mut *const libc::c_char);
    n_tuples = PQntuples(res);
    if n_tuples != 0 as libc::c_int {
        if no_kill_backend {
            elog(
                19 as libc::c_int,
                b"%d unsafe queries remain but do not cancel them and skip to repack it\0"
                    as *const u8 as *const libc::c_char,
                n_tuples,
            );
            ret = 0 as libc::c_int != 0;
        } else {
            resetPQExpBuffer(&mut sql);
            printfPQExpBuffer(
                &mut sql as *mut PQExpBufferData,
                b"SELECT pg_cancel_backend(pid) FROM pg_locks WHERE locktype = 'relation' AND granted = false AND relation = %u AND mode = 'AccessExclusiveLock' AND pid <> pg_backend_pid()\0"
                    as *const u8 as *const libc::c_char,
                relid,
            );
            res = pgut_execute(
                conn,
                sql.data,
                0 as libc::c_int,
                0 as *mut *const libc::c_char,
            );
            if PQresultStatus(res) as libc::c_uint
                != PGRES_TUPLES_OK as libc::c_int as libc::c_uint
            {
                elog(
                    19 as libc::c_int,
                    b"Error canceling unsafe queries: %s\0" as *const u8
                        as *const libc::c_char,
                    PQerrorMessage(conn),
                );
                ret = 0 as libc::c_int != 0;
            } else if PQntuples(res) > 0 as libc::c_int && terminate as libc::c_int != 0
                && PQserverVersion(conn) >= 80400 as libc::c_int
            {
                elog(
                    19 as libc::c_int,
                    b"Canceled %d unsafe queries. Terminating any remaining PIDs.\0"
                        as *const u8 as *const libc::c_char,
                    PQntuples(res),
                );
                PQclear(res);
                res = 0 as *mut PGresult;
                printfPQExpBuffer(
                    &mut sql as *mut PQExpBufferData,
                    b"SELECT pg_terminate_backend(pid) FROM pg_locks WHERE locktype = 'relation' AND granted = false AND relation = %u AND mode = 'AccessExclusiveLock' AND pid <> pg_backend_pid()\0"
                        as *const u8 as *const libc::c_char,
                    relid,
                );
                res = pgut_execute(
                    conn,
                    sql.data,
                    0 as libc::c_int,
                    0 as *mut *const libc::c_char,
                );
                if PQresultStatus(res) as libc::c_uint
                    != PGRES_TUPLES_OK as libc::c_int as libc::c_uint
                {
                    elog(
                        19 as libc::c_int,
                        b"Error killing unsafe queries: %s\0" as *const u8
                            as *const libc::c_char,
                        PQerrorMessage(conn),
                    );
                    ret = 0 as libc::c_int != 0;
                }
            } else if PQntuples(res) > 0 as libc::c_int {
                elog(
                    18 as libc::c_int,
                    b"Canceled %d unsafe queries\0" as *const u8 as *const libc::c_char,
                    PQntuples(res),
                );
            }
        }
    } else {
        elog(
            13 as libc::c_int,
            b"No competing DDL to cancel.\0" as *const u8 as *const libc::c_char,
        );
    }
    PQclear(res);
    res = 0 as *mut PGresult;
    termPQExpBuffer(&mut sql);
    return ret;
}
unsafe extern "C" fn lock_access_share(
    mut conn: *mut PGconn,
    mut relid: Oid,
    mut target_name: *const libc::c_char,
) -> bool {
    let mut sql: PQExpBufferData = PQExpBufferData {
        data: 0 as *mut libc::c_char,
        len: 0,
        maxlen: 0,
    };
    let mut start: time_t = time(0 as *mut time_t);
    let mut i: libc::c_int = 0;
    let mut ret: bool = 1 as libc::c_int != 0;
    initPQExpBuffer(&mut sql);
    i = 1 as libc::c_int;
    loop {
        let mut duration: time_t = 0;
        let mut res: *mut PGresult = 0 as *mut PGresult;
        let mut wait_msec: libc::c_int = 0;
        duration = time(0 as *mut time_t) - start;
        if duration > (wait_timeout * 2 as libc::c_int) as libc::c_long {
            ret = kill_ddl(conn, relid, 1 as libc::c_int != 0);
        } else {
            ret = kill_ddl(conn, relid, 0 as libc::c_int != 0);
        }
        if !ret {
            break;
        }
        wait_msec = if (1000 as libc::c_int) < i * 100 as libc::c_int {
            1000 as libc::c_int
        } else {
            i * 100 as libc::c_int
        };
        printfPQExpBuffer(
            &mut sql as *mut PQExpBufferData,
            b"SET LOCAL statement_timeout = %d\0" as *const u8 as *const libc::c_char,
            wait_msec,
        );
        pgut_command(conn, sql.data, 0 as libc::c_int, 0 as *mut *const libc::c_char);
        printfPQExpBuffer(
            &mut sql as *mut PQExpBufferData,
            b"LOCK TABLE %s IN ACCESS SHARE MODE\0" as *const u8 as *const libc::c_char,
            target_name,
        );
        res = pgut_execute_elevel(
            conn,
            sql.data,
            0 as libc::c_int,
            0 as *mut *const libc::c_char,
            13 as libc::c_int,
        );
        if PQresultStatus(res) as libc::c_uint
            == PGRES_COMMAND_OK as libc::c_int as libc::c_uint
        {
            PQclear(res);
            res = 0 as *mut PGresult;
            break;
        } else if sqlstate_equals(res, b"57014\0" as *const u8 as *const libc::c_char) {
            PQclear(res);
            res = 0 as *mut PGresult;
            pgut_rollback(conn);
            i += 1;
            i;
        } else {
            elog(
                19 as libc::c_int,
                b"%s\0" as *const u8 as *const libc::c_char,
                PQerrorMessage(connection),
            );
            PQclear(res);
            res = 0 as *mut PGresult;
            ret = 0 as libc::c_int != 0;
            break;
        }
    }
    termPQExpBuffer(&mut sql);
    pgut_command(
        conn,
        b"RESET statement_timeout\0" as *const u8 as *const libc::c_char,
        0 as libc::c_int,
        0 as *mut *const libc::c_char,
    );
    return ret;
}
unsafe extern "C" fn advisory_lock(
    mut conn: *mut PGconn,
    mut relid: *const libc::c_char,
) -> bool {
    let mut res: *mut PGresult = 0 as *mut PGresult;
    let mut ret: bool = 0 as libc::c_int != 0;
    let mut params: [*const libc::c_char; 2] = [0 as *const libc::c_char; 2];
    params[0 as libc::c_int
        as usize] = b"16185446\0" as *const u8 as *const libc::c_char;
    params[1 as libc::c_int as usize] = relid;
    res = pgut_execute(
        conn,
        b"SELECT pg_try_advisory_lock($1, CAST(-2147483648 + $2::bigint AS integer))\0"
            as *const u8 as *const libc::c_char,
        2 as libc::c_int,
        params.as_mut_ptr(),
    );
    if PQresultStatus(res) as libc::c_uint
        != PGRES_TUPLES_OK as libc::c_int as libc::c_uint
    {
        elog(
            20 as libc::c_int,
            b"%s\0" as *const u8 as *const libc::c_char,
            PQerrorMessage(connection),
        );
    } else if strcmp(
        getstr(res, 0 as libc::c_int, 0 as libc::c_int),
        b"t\0" as *const u8 as *const libc::c_char,
    ) != 0 as libc::c_int
    {
        elog(
            20 as libc::c_int,
            b"Another pg_repack command may be running on the table. Please try again later.\0"
                as *const u8 as *const libc::c_char,
        );
    } else {
        ret = 1 as libc::c_int != 0;
    }
    PQclear(res);
    res = 0 as *mut PGresult;
    return ret;
}
unsafe extern "C" fn lock_exclusive(
    mut conn: *mut PGconn,
    mut relid: *const libc::c_char,
    mut lock_query: *const libc::c_char,
    mut start_xact: bool,
) -> bool {
    let mut start: time_t = time(0 as *mut time_t);
    let mut i: libc::c_int = 0;
    let mut ret: bool = 1 as libc::c_int != 0;
    i = 1 as libc::c_int;
    loop {
        let mut duration: time_t = 0;
        let mut sql: [libc::c_char; 1024] = [0; 1024];
        let mut res: *mut PGresult = 0 as *mut PGresult;
        let mut wait_msec: libc::c_int = 0;
        if start_xact {
            pgut_command(
                conn,
                b"BEGIN ISOLATION LEVEL READ COMMITTED\0" as *const u8
                    as *const libc::c_char,
                0 as libc::c_int,
                0 as *mut *const libc::c_char,
            );
        } else {
            pgut_command(
                conn,
                b"SAVEPOINT repack_sp1\0" as *const u8 as *const libc::c_char,
                0 as libc::c_int,
                0 as *mut *const libc::c_char,
            );
        }
        duration = time(0 as *mut time_t) - start;
        if duration > wait_timeout as libc::c_long {
            if no_kill_backend {
                elog(
                    19 as libc::c_int,
                    b"timed out, do not cancel conflicting backends\0" as *const u8
                        as *const libc::c_char,
                );
                ret = 0 as libc::c_int != 0;
                if start_xact {
                    pgut_rollback(conn);
                } else {
                    pgut_command(
                        conn,
                        b"ROLLBACK TO SAVEPOINT repack_sp1\0" as *const u8
                            as *const libc::c_char,
                        0 as libc::c_int,
                        0 as *mut *const libc::c_char,
                    );
                }
                break;
            } else {
                let mut cancel_query: *const libc::c_char = 0 as *const libc::c_char;
                if PQserverVersion(conn) >= 80400 as libc::c_int
                    && duration > (wait_timeout * 2 as libc::c_int) as libc::c_long
                {
                    elog(
                        19 as libc::c_int,
                        b"terminating conflicted backends\0" as *const u8
                            as *const libc::c_char,
                    );
                    cancel_query = b"SELECT pg_terminate_backend(pid) FROM pg_locks WHERE locktype = 'relation'   AND relation = $1 AND pid <> pg_backend_pid()\0"
                        as *const u8 as *const libc::c_char;
                } else {
                    elog(
                        19 as libc::c_int,
                        b"canceling conflicted backends\0" as *const u8
                            as *const libc::c_char,
                    );
                    cancel_query = b"SELECT pg_cancel_backend(pid) FROM pg_locks WHERE locktype = 'relation'   AND relation = $1 AND pid <> pg_backend_pid()\0"
                        as *const u8 as *const libc::c_char;
                }
                pgut_command(conn, cancel_query, 1 as libc::c_int, &mut relid);
            }
        }
        wait_msec = if (1000 as libc::c_int) < i * 100 as libc::c_int {
            1000 as libc::c_int
        } else {
            i * 100 as libc::c_int
        };
        pg_snprintf(
            sql.as_mut_ptr(),
            (::std::mem::size_of::<[libc::c_char; 1024]>() as libc::c_ulong)
                .wrapping_div(::std::mem::size_of::<libc::c_char>() as libc::c_ulong),
            b"SET LOCAL statement_timeout = %d\0" as *const u8 as *const libc::c_char,
            wait_msec,
        );
        pgut_command(
            conn,
            sql.as_mut_ptr(),
            0 as libc::c_int,
            0 as *mut *const libc::c_char,
        );
        res = pgut_execute_elevel(
            conn,
            lock_query,
            0 as libc::c_int,
            0 as *mut *const libc::c_char,
            13 as libc::c_int,
        );
        if PQresultStatus(res) as libc::c_uint
            == PGRES_COMMAND_OK as libc::c_int as libc::c_uint
        {
            PQclear(res);
            res = 0 as *mut PGresult;
            break;
        } else if sqlstate_equals(res, b"57014\0" as *const u8 as *const libc::c_char) {
            PQclear(res);
            res = 0 as *mut PGresult;
            if start_xact {
                pgut_rollback(conn);
            } else {
                pgut_command(
                    conn,
                    b"ROLLBACK TO SAVEPOINT repack_sp1\0" as *const u8
                        as *const libc::c_char,
                    0 as libc::c_int,
                    0 as *mut *const libc::c_char,
                );
            }
            i += 1;
            i;
        } else {
            pg_printf(
                b"%s\0" as *const u8 as *const libc::c_char,
                PQerrorMessage(connection),
            );
            PQclear(res);
            res = 0 as *mut PGresult;
            ret = 0 as libc::c_int != 0;
            break;
        }
    }
    pgut_command(
        conn,
        b"RESET statement_timeout\0" as *const u8 as *const libc::c_char,
        0 as libc::c_int,
        0 as *mut *const libc::c_char,
    );
    return ret;
}
unsafe extern "C" fn repack_cleanup_callback(
    mut fatal: bool,
    mut userdata: *mut libc::c_void,
) {
    let mut target_table: Oid = *(userdata as *mut Oid);
    let mut params: [*const libc::c_char; 2] = [0 as *const libc::c_char; 2];
    let mut buffer: [libc::c_char; 12] = [0; 12];
    let mut num_buff: [libc::c_char; 12] = [0; 12];
    if fatal {
        params[0 as libc::c_int as usize] = utoa(target_table, buffer.as_mut_ptr());
        params[1 as libc::c_int as usize] = utoa(temp_obj_num, num_buff.as_mut_ptr());
        reconnect(20 as libc::c_int);
        command(
            b"SELECT repack.repack_drop($1, $2)\0" as *const u8 as *const libc::c_char,
            2 as libc::c_int,
            params.as_mut_ptr(),
        );
        temp_obj_num = 0 as libc::c_int as libc::c_uint;
    }
}
unsafe extern "C" fn repack_cleanup(mut fatal: bool, mut table: *const repack_table) {
    if fatal {
        pg_fprintf(
            stderr,
            b"!!!FATAL ERROR!!! Please refer to the manual.\n\n\0" as *const u8
                as *const libc::c_char,
        );
    } else {
        let mut buffer: [libc::c_char; 12] = [0; 12];
        let mut num_buff: [libc::c_char; 12] = [0; 12];
        let mut params: [*const libc::c_char; 2] = [0 as *const libc::c_char; 2];
        if PQstatus(connection) as libc::c_uint
            != CONNECTION_OK as libc::c_int as libc::c_uint
            || PQstatus(conn2) as libc::c_uint
                != CONNECTION_OK as libc::c_int as libc::c_uint
        {
            reconnect(20 as libc::c_int);
        }
        params[0 as libc::c_int
            as usize] = utoa((*table).target_oid, buffer.as_mut_ptr());
        params[1 as libc::c_int as usize] = utoa(temp_obj_num, num_buff.as_mut_ptr());
        command(
            b"SELECT repack.repack_drop($1, $2)\0" as *const u8 as *const libc::c_char,
            2 as libc::c_int,
            params.as_mut_ptr(),
        );
        temp_obj_num = 0 as libc::c_int as libc::c_uint;
    };
}
unsafe extern "C" fn repack_table_indexes(mut index_details: *mut PGresult) -> bool {
    let mut ret: bool = 0 as libc::c_int != 0;
    let mut res: *mut PGresult = 0 as *mut PGresult;
    let mut res2: *mut PGresult = 0 as *mut PGresult;
    let mut sql: PQExpBufferData = PQExpBufferData {
        data: 0 as *mut libc::c_char,
        len: 0,
        maxlen: 0,
    };
    let mut sql_drop: PQExpBufferData = PQExpBufferData {
        data: 0 as *mut libc::c_char,
        len: 0,
        maxlen: 0,
    };
    let mut buffer: [[libc::c_char; 12]; 2] = [[0; 12]; 2];
    let mut create_idx: *const libc::c_char = 0 as *const libc::c_char;
    let mut schema_name: *const libc::c_char = 0 as *const libc::c_char;
    let mut table_name: *const libc::c_char = 0 as *const libc::c_char;
    let mut params: [*const libc::c_char; 3] = [0 as *const libc::c_char; 3];
    let mut table: Oid = 0;
    let mut index: Oid = 0;
    let mut i: libc::c_int = 0;
    let mut num: libc::c_int = 0;
    let mut num_repacked: libc::c_int = 0 as libc::c_int;
    let mut repacked_indexes: *mut bool = 0 as *mut bool;
    initPQExpBuffer(&mut sql);
    num = PQntuples(index_details);
    table = getoid(index_details, 0 as libc::c_int, 3 as libc::c_int);
    params[1 as libc::c_int
        as usize] = utoa(table, (buffer[1 as libc::c_int as usize]).as_mut_ptr());
    params[2 as libc::c_int as usize] = tablespace;
    schema_name = getstr(index_details, 0 as libc::c_int, 5 as libc::c_int);
    table_name = getstr(index_details, 0 as libc::c_int, 4 as libc::c_int);
    repacked_indexes = calloc(
        num as libc::c_ulong,
        ::std::mem::size_of::<bool>() as libc::c_ulong,
    ) as *mut bool;
    if repacked_indexes.is_null() {
        if pgut_errstart(20 as libc::c_int) as libc::c_int != 0 {
            pgut_errfinish(
                errcode(12 as libc::c_int),
                errmsg(
                    b"Unable to calloc repacked_indexes\0" as *const u8
                        as *const libc::c_char,
                ),
            );
        } else {};
    }
    if !advisory_lock(connection, params[1 as libc::c_int as usize]) {
        if pgut_errstart(20 as libc::c_int) as libc::c_int != 0 {
            pgut_errfinish(
                errcode(22 as libc::c_int),
                errmsg(
                    b"Unable to obtain advisory lock on \"%s\"\0" as *const u8
                        as *const libc::c_char,
                    table_name,
                ),
            );
        } else {};
    }
    i = 0 as libc::c_int;
    while i < num {
        let mut isvalid: *mut libc::c_char = getstr(index_details, i, 2 as libc::c_int);
        let mut idx_name: *mut libc::c_char = getstr(index_details, i, 0 as libc::c_int);
        if *isvalid.offset(0 as libc::c_int as isize) as libc::c_int == 't' as i32 {
            index = getoid(index_details, i, 1 as libc::c_int);
            resetPQExpBuffer(&mut sql);
            appendPQExpBuffer(
                &mut sql as *mut PQExpBufferData,
                b"SELECT pgc.relname, nsp.nspname FROM pg_class pgc INNER JOIN pg_namespace nsp ON nsp.oid = pgc.relnamespace WHERE pgc.relname = 'index_%u' AND nsp.nspname = $1\0"
                    as *const u8 as *const libc::c_char,
                index,
            );
            params[0 as libc::c_int as usize] = schema_name;
            elog(
                17 as libc::c_int,
                b"repacking index \"%s\"\0" as *const u8 as *const libc::c_char,
                idx_name,
            );
            res = execute(sql.data, 1 as libc::c_int, params.as_mut_ptr());
            if PQresultStatus(res) as libc::c_uint
                != PGRES_TUPLES_OK as libc::c_int as libc::c_uint
            {
                elog(
                    19 as libc::c_int,
                    b"%s\0" as *const u8 as *const libc::c_char,
                    PQerrorMessage(connection),
                );
            } else if PQntuples(res) > 0 as libc::c_int {
                if pgut_errstart(19 as libc::c_int) as libc::c_int != 0 {
                    pgut_errfinish(
                        errcode(-(2 as libc::c_int)),
                        errmsg(
                            b"Cannot create index \"%s\".\"index_%u\", already exists\0"
                                as *const u8 as *const libc::c_char,
                            schema_name,
                            index,
                        ),
                        errdetail(
                            b"An invalid index may have been left behind by a previous pg_repack on the table which was interrupted. Please use DROP INDEX \"%s\".\"index_%u\" to remove this index and try again.\0"
                                as *const u8 as *const libc::c_char,
                            schema_name,
                            index,
                        ),
                    );
                } else {};
            } else if !dryrun {
                params[0 as libc::c_int
                    as usize] = utoa(
                    index,
                    (buffer[0 as libc::c_int as usize]).as_mut_ptr(),
                );
                res = execute(
                    b"SELECT repack.repack_indexdef($1, $2, $3, true)\0" as *const u8
                        as *const libc::c_char,
                    3 as libc::c_int,
                    params.as_mut_ptr(),
                );
                if PQntuples(res) < 1 as libc::c_int {
                    elog(
                        19 as libc::c_int,
                        b"unable to generate SQL to CREATE work index for %s\0"
                            as *const u8 as *const libc::c_char,
                        getstr(index_details, i, 0 as libc::c_int),
                    );
                } else {
                    create_idx = getstr(res, 0 as libc::c_int, 0 as libc::c_int);
                    res2 = execute_elevel(
                        create_idx,
                        0 as libc::c_int,
                        0 as *mut *const libc::c_char,
                        13 as libc::c_int,
                    );
                    if PQresultStatus(res2) as libc::c_uint
                        != PGRES_COMMAND_OK as libc::c_int as libc::c_uint
                    {
                        if pgut_errstart(19 as libc::c_int) as libc::c_int != 0 {
                            pgut_errfinish(
                                errcode(-(2 as libc::c_int)),
                                errmsg(
                                    b"Error creating index \"%s\".\"index_%u\": %s\0"
                                        as *const u8 as *const libc::c_char,
                                    schema_name,
                                    index,
                                    PQerrorMessage(connection),
                                ),
                            );
                        } else {};
                    } else {
                        *repacked_indexes.offset(i as isize) = 1 as libc::c_int != 0;
                        num_repacked += 1;
                        num_repacked;
                    }
                    PQclear(res);
                    res = 0 as *mut PGresult;
                    PQclear(res2);
                    res2 = 0 as *mut PGresult;
                }
            }
        } else {
            elog(
                19 as libc::c_int,
                b"skipping invalid index: %s.%s\0" as *const u8 as *const libc::c_char,
                schema_name,
                getstr(index_details, i, 0 as libc::c_int),
            );
        }
        i += 1;
        i;
    }
    if dryrun {
        ret = 1 as libc::c_int != 0;
    } else {
        if num_repacked == 0 {
            elog(
                19 as libc::c_int,
                b"Skipping index swapping for \"%s\", since no new indexes built\0"
                    as *const u8 as *const libc::c_char,
                table_name,
            );
        } else {
            resetPQExpBuffer(&mut sql);
            appendPQExpBuffer(
                &mut sql as *mut PQExpBufferData,
                b"LOCK TABLE %s IN ACCESS EXCLUSIVE MODE\0" as *const u8
                    as *const libc::c_char,
                table_name,
            );
            if !lock_exclusive(
                connection,
                params[1 as libc::c_int as usize],
                sql.data,
                1 as libc::c_int != 0,
            ) {
                elog(
                    19 as libc::c_int,
                    b"lock_exclusive() failed in connection for %s\0" as *const u8
                        as *const libc::c_char,
                    table_name,
                );
            } else {
                i = 0 as libc::c_int;
                while i < num {
                    index = getoid(index_details, i, 1 as libc::c_int);
                    if *repacked_indexes.offset(i as isize) {
                        params[0 as libc::c_int
                            as usize] = utoa(
                            index,
                            (buffer[0 as libc::c_int as usize]).as_mut_ptr(),
                        );
                        pgut_command(
                            connection,
                            b"SELECT repack.repack_index_swap($1)\0" as *const u8
                                as *const libc::c_char,
                            1 as libc::c_int,
                            params.as_mut_ptr(),
                        );
                    } else {
                        elog(
                            17 as libc::c_int,
                            b"Skipping index swap for index_%u\0" as *const u8
                                as *const libc::c_char,
                            index,
                        );
                    }
                    i += 1;
                    i;
                }
                pgut_command(
                    connection,
                    b"COMMIT\0" as *const u8 as *const libc::c_char,
                    0 as libc::c_int,
                    0 as *mut *const libc::c_char,
                );
                ret = 1 as libc::c_int != 0;
            }
        }
        resetPQExpBuffer(&mut sql);
        initPQExpBuffer(&mut sql_drop);
        appendPQExpBufferStr(
            &mut sql,
            b"DROP INDEX CONCURRENTLY \0" as *const u8 as *const libc::c_char,
        );
        appendPQExpBuffer(
            &mut sql as *mut PQExpBufferData,
            b"\"%s\".\0" as *const u8 as *const libc::c_char,
            schema_name,
        );
        i = 0 as libc::c_int;
        while i < num {
            index = getoid(index_details, i, 1 as libc::c_int);
            if *repacked_indexes.offset(i as isize) {
                initPQExpBuffer(&mut sql_drop);
                appendPQExpBuffer(
                    &mut sql_drop as *mut PQExpBufferData,
                    b"%s\"index_%u\"\0" as *const u8 as *const libc::c_char,
                    sql.data,
                    index,
                );
                command(sql_drop.data, 0 as libc::c_int, 0 as *mut *const libc::c_char);
            } else {
                elog(
                    17 as libc::c_int,
                    b"Skipping drop of index_%u\0" as *const u8 as *const libc::c_char,
                    index,
                );
            }
            i += 1;
            i;
        }
        termPQExpBuffer(&mut sql_drop);
        termPQExpBuffer(&mut sql);
    }
    PQclear(res);
    res = 0 as *mut PGresult;
    free(repacked_indexes as *mut libc::c_void);
    return ret;
}
unsafe extern "C" fn repack_all_indexes(
    mut errbuf: *mut libc::c_char,
    mut errsize: size_t,
) -> bool {
    let mut ret: bool = 0 as libc::c_int != 0;
    let mut res: *mut PGresult = 0 as *mut PGresult;
    let mut sql: PQExpBufferData = PQExpBufferData {
        data: 0 as *mut libc::c_char,
        len: 0,
        maxlen: 0,
    };
    let mut cell: *mut SimpleStringListCell = 0 as *mut SimpleStringListCell;
    let mut params: [*const libc::c_char; 1] = [0 as *const libc::c_char; 1];
    initPQExpBuffer(&mut sql);
    reconnect(20 as libc::c_int);
    if !(r_index.head).is_null() || !(table_list.head).is_null()
        || !(parent_table_list.head).is_null()
    {} else {
        __assert_fail(
            b"r_index.head || table_list.head || parent_table_list.head\0" as *const u8
                as *const libc::c_char,
            b"pg_repack.c\0" as *const u8 as *const libc::c_char,
            2225 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 41],
                &[libc::c_char; 41],
            >(b"_Bool repack_all_indexes(char *, size_t)\0"))
                .as_ptr(),
        );
    }
    'c_18309: {
        if !(r_index.head).is_null() || !(table_list.head).is_null()
            || !(parent_table_list.head).is_null()
        {} else {
            __assert_fail(
                b"r_index.head || table_list.head || parent_table_list.head\0"
                    as *const u8 as *const libc::c_char,
                b"pg_repack.c\0" as *const u8 as *const libc::c_char,
                2225 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 41],
                    &[libc::c_char; 41],
                >(b"_Bool repack_all_indexes(char *, size_t)\0"))
                    .as_ptr(),
            );
        }
    };
    if preliminary_checks(errbuf, errsize) {
        if is_requested_relation_exists(errbuf, errsize) {
            if !(r_index.head).is_null() {
                appendPQExpBufferStr(
                    &mut sql,
                    b"SELECT repack.oid2text(i.oid), idx.indexrelid, idx.indisvalid, idx.indrelid, repack.oid2text(idx.indrelid), n.nspname FROM pg_index idx JOIN pg_class i ON i.oid = idx.indexrelid JOIN pg_namespace n ON n.oid = i.relnamespace WHERE idx.indexrelid = $1::regclass ORDER BY indisvalid DESC, i.relname, n.nspname\0"
                        as *const u8 as *const libc::c_char,
                );
                cell = r_index.head;
            } else if !(table_list.head).is_null() || !(parent_table_list.head).is_null()
            {
                appendPQExpBufferStr(
                    &mut sql,
                    b"SELECT repack.oid2text(i.oid), idx.indexrelid, idx.indisvalid, idx.indrelid, $1::text, n.nspname FROM pg_index idx JOIN pg_class i ON i.oid = idx.indexrelid JOIN pg_namespace n ON n.oid = i.relnamespace WHERE idx.indrelid = $1::regclass ORDER BY indisvalid DESC, i.relname, n.nspname\0"
                        as *const u8 as *const libc::c_char,
                );
                cell = parent_table_list.head;
                while !cell.is_null() {
                    let mut nchildren: libc::c_int = 0;
                    let mut i: libc::c_int = 0;
                    params[0 as libc::c_int as usize] = ((*cell).val).as_mut_ptr();
                    res = execute_elevel(
                        b"SELECT quote_ident(n.nspname) || '.' || quote_ident(c.relname) FROM pg_class c JOIN pg_namespace n on n.oid = c.relnamespace WHERE c.oid = ANY (repack.get_table_and_inheritors($1::regclass)) ORDER BY n.nspname, c.relname\0"
                            as *const u8 as *const libc::c_char,
                        1 as libc::c_int,
                        params.as_mut_ptr(),
                        13 as libc::c_int,
                    );
                    if PQresultStatus(res) as libc::c_uint
                        != PGRES_TUPLES_OK as libc::c_int as libc::c_uint
                    {
                        elog(
                            19 as libc::c_int,
                            b"%s\0" as *const u8 as *const libc::c_char,
                            PQerrorMessage(connection),
                        );
                    } else {
                        nchildren = PQntuples(res);
                        if nchildren == 0 as libc::c_int {
                            elog(
                                19 as libc::c_int,
                                b"relation \"%s\" does not exist\0" as *const u8
                                    as *const libc::c_char,
                                ((*cell).val).as_mut_ptr(),
                            );
                        } else {
                            i = 0 as libc::c_int;
                            while i < nchildren {
                                simple_string_list_append(
                                    &mut table_list,
                                    getstr(res, i, 0 as libc::c_int),
                                );
                                i += 1;
                                i;
                            }
                        }
                    }
                    cell = (*cell).next;
                }
                PQclear(res);
                res = 0 as *mut PGresult;
                cell = table_list.head;
            }
            while !cell.is_null() {
                params[0 as libc::c_int as usize] = ((*cell).val).as_mut_ptr();
                res = execute_elevel(
                    sql.data,
                    1 as libc::c_int,
                    params.as_mut_ptr(),
                    13 as libc::c_int,
                );
                if PQresultStatus(res) as libc::c_uint
                    != PGRES_TUPLES_OK as libc::c_int as libc::c_uint
                {
                    elog(
                        19 as libc::c_int,
                        b"%s\0" as *const u8 as *const libc::c_char,
                        PQerrorMessage(connection),
                    );
                } else if PQntuples(res) == 0 as libc::c_int {
                    if !(table_list.head).is_null() {
                        elog(
                            19 as libc::c_int,
                            b"\"%s\" does not have any indexes\0" as *const u8
                                as *const libc::c_char,
                            ((*cell).val).as_mut_ptr(),
                        );
                    } else if !(r_index.head).is_null() {
                        elog(
                            19 as libc::c_int,
                            b"\"%s\" is not a valid index\0" as *const u8
                                as *const libc::c_char,
                            ((*cell).val).as_mut_ptr(),
                        );
                    }
                } else {
                    if !(table_list.head).is_null() {
                        elog(
                            17 as libc::c_int,
                            b"repacking indexes of \"%s\"\0" as *const u8
                                as *const libc::c_char,
                            ((*cell).val).as_mut_ptr(),
                        );
                    }
                    if !repack_table_indexes(res) {
                        elog(
                            19 as libc::c_int,
                            b"repack failed for \"%s\"\0" as *const u8
                                as *const libc::c_char,
                            ((*cell).val).as_mut_ptr(),
                        );
                    }
                    PQclear(res);
                    res = 0 as *mut PGresult;
                }
                cell = (*cell).next;
            }
            ret = 1 as libc::c_int != 0;
        }
    }
    disconnect();
    termPQExpBuffer(&mut sql);
    return ret;
}
pub unsafe extern "C" fn pgut_help(mut details: bool) {
    pg_printf(
        b"%s re-organizes a PostgreSQL database.\n\n\0" as *const u8
            as *const libc::c_char,
        PROGRAM_NAME,
    );
    pg_printf(b"Usage:\n\0" as *const u8 as *const libc::c_char);
    pg_printf(
        b"  %s [OPTION]... [DBNAME]\n\0" as *const u8 as *const libc::c_char,
        PROGRAM_NAME,
    );
    if !details {
        return;
    }
    pg_printf(b"Options:\n\0" as *const u8 as *const libc::c_char);
    pg_printf(
        b"  -a, --all                 repack all databases\n\0" as *const u8
            as *const libc::c_char,
    );
    pg_printf(
        b"  -t, --table=TABLE         repack specific table only\n\0" as *const u8
            as *const libc::c_char,
    );
    pg_printf(
        b"  -I, --parent-table=TABLE  repack specific parent table and its inheritors\n\0"
            as *const u8 as *const libc::c_char,
    );
    pg_printf(
        b"  -c, --schema=SCHEMA       repack tables in specific schema only\n\0"
            as *const u8 as *const libc::c_char,
    );
    pg_printf(
        b"  -s, --tablespace=TBLSPC   move repacked tables to a new tablespace\n\0"
            as *const u8 as *const libc::c_char,
    );
    pg_printf(
        b"  -S, --moveidx             move repacked indexes to TBLSPC too\n\0"
            as *const u8 as *const libc::c_char,
    );
    pg_printf(
        b"  -o, --order-by=COLUMNS    order by columns instead of cluster keys\n\0"
            as *const u8 as *const libc::c_char,
    );
    pg_printf(
        b"  -n, --no-order            do vacuum full instead of cluster\n\0" as *const u8
            as *const libc::c_char,
    );
    pg_printf(
        b"  -N, --dry-run             print what would have been repacked\n\0"
            as *const u8 as *const libc::c_char,
    );
    pg_printf(
        b"  -j, --jobs=NUM            Use this many parallel jobs for each table\n\0"
            as *const u8 as *const libc::c_char,
    );
    pg_printf(
        b"  -i, --index=INDEX         move only the specified index\n\0" as *const u8
            as *const libc::c_char,
    );
    pg_printf(
        b"  -x, --only-indexes        move only indexes of the specified table\n\0"
            as *const u8 as *const libc::c_char,
    );
    pg_printf(
        b"  -T, --wait-timeout=SECS   timeout to cancel other backends on conflict\n\0"
            as *const u8 as *const libc::c_char,
    );
    pg_printf(
        b"  -D, --no-kill-backend     don't kill other backends when timed out\n\0"
            as *const u8 as *const libc::c_char,
    );
    pg_printf(
        b"  -Z, --no-analyze          don't analyze at end\n\0" as *const u8
            as *const libc::c_char,
    );
    pg_printf(
        b"  -k, --no-superuser-check  skip superuser checks in client\n\0" as *const u8
            as *const libc::c_char,
    );
    pg_printf(
        b"  -C, --exclude-extension   don't repack tables which belong to specific extension\n\0"
            as *const u8 as *const libc::c_char,
    );
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
