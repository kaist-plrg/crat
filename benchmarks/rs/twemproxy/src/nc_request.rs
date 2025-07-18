use ::libc;
use ::c2rust_bitfields;
extern "C" {
    pub type epoll_event;
    pub type conf;
    fn __errno_location() -> *mut libc::c_int;
    fn array_get(a: *const array, idx: uint32_t) -> *mut libc::c_void;
    fn nc_usec_now() -> int64_t;
    fn nc_unresolve_peer_desc(sd: libc::c_int) -> *const libc::c_char;
    fn log_loggable(level: libc::c_int) -> libc::c_int;
    fn _log(
        file: *const libc::c_char,
        line: libc::c_int,
        panic: libc::c_int,
        fmt: *const libc::c_char,
        _: ...
    );
    fn event_add_out(evb: *mut event_base, c: *mut conn) -> libc::c_int;
    fn event_del_out(evb: *mut event_base, c: *mut conn) -> libc::c_int;
    fn _stats_server_incr(
        ctx: *mut context,
        server: *const server,
        fidx: stats_server_field_t,
    );
    fn _stats_server_decr(
        ctx: *mut context,
        server: *const server,
        fidx: stats_server_field_t,
    );
    fn _stats_server_incr_by(
        ctx: *mut context,
        server: *const server,
        fidx: stats_server_field_t,
        val: int64_t,
    );
    fn _stats_server_decr_by(
        ctx: *mut context,
        server: *const server,
        fidx: stats_server_field_t,
        val: int64_t,
    );
    fn msg_tmo_insert(msg: *mut msg, conn: *mut conn);
    fn msg_tmo_delete(msg: *mut msg);
    fn msg_type_string(type_0: msg_type_t) -> *const string;
    fn msg_get(conn: *mut conn, request: bool, redis: bool) -> *mut msg;
    fn msg_put(msg: *mut msg);
    fn msg_empty(msg: *const msg) -> bool;
    fn rsp_put(msg: *mut msg);
    fn conn_authenticated(conn: *const conn) -> bool;
    fn server_pool_conn(
        ctx: *mut context,
        pool: *mut server_pool,
        key: *const uint8_t,
        keylen: uint32_t,
    ) -> *mut conn;
    fn server_connected(ctx: *mut context, conn: *mut conn);
}
pub type rstatus_t = libc::c_int;
pub type err_t = libc::c_int;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct array {
    pub nelem: uint32_t,
    pub elem: *mut libc::c_void,
    pub size: size_t,
    pub nalloc: uint32_t,
}
pub type uint32_t = __uint32_t;
pub type __uint32_t = libc::c_uint;
pub type size_t = libc::c_ulong;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct string {
    pub len: uint32_t,
    pub data: *mut uint8_t,
}
pub type uint8_t = __uint8_t;
pub type __uint8_t = libc::c_uchar;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct context {
    pub id: uint32_t,
    pub cf: *mut conf,
    pub stats: *mut stats,
    pub pool: array,
    pub evb: *mut event_base,
    pub max_timeout: libc::c_int,
    pub timeout: libc::c_int,
    pub max_nfd: uint32_t,
    pub max_ncconn: uint32_t,
    pub max_nsconn: uint32_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct event_base {
    pub ep: libc::c_int,
    pub event: *mut epoll_event,
    pub nevent: libc::c_int,
    pub cb: event_cb_t,
}
pub type event_cb_t = Option::<
    unsafe extern "C" fn(*mut libc::c_void, uint32_t) -> libc::c_int,
>;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct stats {
    pub port: uint16_t,
    pub interval: libc::c_int,
    pub addr: string,
    pub start_ts: int64_t,
    pub buf: stats_buffer,
    pub current: array,
    pub shadow: array,
    pub sum: array,
    pub tid: pthread_t,
    pub sd: libc::c_int,
    pub service_str: string,
    pub service: string,
    pub source_str: string,
    pub source: string,
    pub version_str: string,
    pub version: string,
    pub uptime_str: string,
    pub timestamp_str: string,
    pub ntotal_conn_str: string,
    pub ncurr_conn_str: string,
    pub aggregate: libc::c_int,
    pub updated: libc::c_int,
}
pub type pthread_t = libc::c_ulong;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct stats_buffer {
    pub len: size_t,
    pub data: *mut uint8_t,
    pub size: size_t,
}
pub type int64_t = __int64_t;
pub type __int64_t = libc::c_long;
pub type uint16_t = __uint16_t;
pub type __uint16_t = libc::c_ushort;
#[derive(Copy, Clone, BitfieldStruct)]
#[repr(C)]
pub struct conn {
    pub conn_tqe: C2RustUnnamed_5,
    pub owner: *mut libc::c_void,
    pub sd: libc::c_int,
    pub family: libc::c_int,
    pub addrlen: socklen_t,
    pub addr: *mut sockaddr,
    pub imsg_q: msg_tqh,
    pub omsg_q: msg_tqh,
    pub rmsg: *mut msg,
    pub smsg: *mut msg,
    pub recv: conn_recv_t,
    pub recv_next: conn_recv_next_t,
    pub recv_done: conn_recv_done_t,
    pub send: conn_send_t,
    pub send_next: conn_send_next_t,
    pub send_done: conn_send_done_t,
    pub close: conn_close_t,
    pub active: conn_active_t,
    pub post_connect: conn_post_connect_t,
    pub swallow_msg: conn_swallow_msg_t,
    pub ref_0: conn_ref_t,
    pub unref: conn_unref_t,
    pub enqueue_inq: conn_msgq_t,
    pub dequeue_inq: conn_msgq_t,
    pub enqueue_outq: conn_msgq_t,
    pub dequeue_outq: conn_msgq_t,
    pub recv_bytes: size_t,
    pub send_bytes: size_t,
    pub events: uint32_t,
    pub err: err_t,
    #[bitfield(name = "recv_active", ty = "libc::c_uint", bits = "0..=0")]
    #[bitfield(name = "recv_ready", ty = "libc::c_uint", bits = "1..=1")]
    #[bitfield(name = "send_active", ty = "libc::c_uint", bits = "2..=2")]
    #[bitfield(name = "send_ready", ty = "libc::c_uint", bits = "3..=3")]
    #[bitfield(name = "client", ty = "libc::c_uint", bits = "4..=4")]
    #[bitfield(name = "proxy", ty = "libc::c_uint", bits = "5..=5")]
    #[bitfield(name = "connecting", ty = "libc::c_uint", bits = "6..=6")]
    #[bitfield(name = "connected", ty = "libc::c_uint", bits = "7..=7")]
    #[bitfield(name = "eof", ty = "libc::c_uint", bits = "8..=8")]
    #[bitfield(name = "done", ty = "libc::c_uint", bits = "9..=9")]
    #[bitfield(name = "redis", ty = "libc::c_uint", bits = "10..=10")]
    #[bitfield(name = "authenticated", ty = "libc::c_uint", bits = "11..=11")]
    pub recv_active_recv_ready_send_active_send_ready_client_proxy_connecting_connected_eof_done_redis_authenticated: [u8; 2],
    #[bitfield(padding)]
    pub c2rust_padding: [u8; 6],
}
pub type conn_msgq_t = Option::<
    unsafe extern "C" fn(*mut context, *mut conn, *mut msg) -> (),
>;
#[derive(Copy, Clone, BitfieldStruct)]
#[repr(C)]
pub struct msg {
    pub c_tqe: C2RustUnnamed_2,
    pub s_tqe: C2RustUnnamed_1,
    pub m_tqe: C2RustUnnamed_0,
    pub id: uint64_t,
    pub peer: *mut msg,
    pub owner: *mut conn,
    pub tmo_rbe: rbnode,
    pub mhdr: mhdr,
    pub mlen: uint32_t,
    pub start_ts: int64_t,
    pub state: libc::c_int,
    pub pos: *mut uint8_t,
    pub token: *mut uint8_t,
    pub parser: msg_parse_t,
    pub result: msg_parse_result_t,
    pub fragment: msg_fragment_t,
    pub reply: msg_reply_t,
    pub add_auth: msg_add_auth_t,
    pub failure: msg_failure_t,
    pub pre_coalesce: msg_coalesce_t,
    pub post_coalesce: msg_coalesce_t,
    pub type_0: msg_type_t,
    pub keys: *mut array,
    pub vlen: uint32_t,
    pub end: *mut uint8_t,
    pub narg_start: *mut uint8_t,
    pub narg_end: *mut uint8_t,
    pub narg: uint32_t,
    pub rnarg: uint32_t,
    pub rlen: uint32_t,
    pub integer: uint32_t,
    pub is_top_level: uint8_t,
    pub frag_owner: *mut msg,
    pub nfrag: uint32_t,
    pub nfrag_done: uint32_t,
    pub frag_id: uint64_t,
    pub frag_seq: *mut *mut msg,
    pub err: err_t,
    #[bitfield(name = "error", ty = "libc::c_uint", bits = "0..=0")]
    #[bitfield(name = "ferror", ty = "libc::c_uint", bits = "1..=1")]
    #[bitfield(name = "request", ty = "libc::c_uint", bits = "2..=2")]
    #[bitfield(name = "quit", ty = "libc::c_uint", bits = "3..=3")]
    #[bitfield(name = "noreply", ty = "libc::c_uint", bits = "4..=4")]
    #[bitfield(name = "noforward", ty = "libc::c_uint", bits = "5..=5")]
    #[bitfield(name = "done", ty = "libc::c_uint", bits = "6..=6")]
    #[bitfield(name = "fdone", ty = "libc::c_uint", bits = "7..=7")]
    #[bitfield(name = "swallow", ty = "libc::c_uint", bits = "8..=8")]
    #[bitfield(name = "redis", ty = "libc::c_uint", bits = "9..=9")]
    pub error_ferror_request_quit_noreply_noforward_done_fdone_swallow_redis: [u8; 2],
    #[bitfield(padding)]
    pub c2rust_padding: [u8; 2],
}
pub type uint64_t = __uint64_t;
pub type __uint64_t = libc::c_ulong;
pub type msg_type_t = msg_type;
pub type msg_type = libc::c_uint;
pub const MSG_SENTINEL: msg_type = 184;
pub const MSG_RSP_REDIS_MULTIBULK: msg_type = 183;
pub const MSG_RSP_REDIS_BULK: msg_type = 182;
pub const MSG_RSP_REDIS_INTEGER: msg_type = 181;
pub const MSG_RSP_REDIS_ERROR_NOREPLICAS: msg_type = 180;
pub const MSG_RSP_REDIS_ERROR_MASTERDOWN: msg_type = 179;
pub const MSG_RSP_REDIS_ERROR_EXECABORT: msg_type = 178;
pub const MSG_RSP_REDIS_ERROR_WRONGTYPE: msg_type = 177;
pub const MSG_RSP_REDIS_ERROR_READONLY: msg_type = 176;
pub const MSG_RSP_REDIS_ERROR_NOSCRIPT: msg_type = 175;
pub const MSG_RSP_REDIS_ERROR_MISCONF: msg_type = 174;
pub const MSG_RSP_REDIS_ERROR_BUSYKEY: msg_type = 173;
pub const MSG_RSP_REDIS_ERROR_LOADING: msg_type = 172;
pub const MSG_RSP_REDIS_ERROR_NOAUTH: msg_type = 171;
pub const MSG_RSP_REDIS_ERROR_BUSY: msg_type = 170;
pub const MSG_RSP_REDIS_ERROR_OOM: msg_type = 169;
pub const MSG_RSP_REDIS_ERROR_ERR: msg_type = 168;
pub const MSG_RSP_REDIS_ERROR: msg_type = 167;
pub const MSG_RSP_REDIS_STATUS: msg_type = 166;
pub const MSG_REQ_REDIS_LOLWUT: msg_type = 165;
pub const MSG_REQ_REDIS_COMMAND: msg_type = 164;
pub const MSG_REQ_REDIS_SELECT: msg_type = 163;
pub const MSG_REQ_REDIS_AUTH: msg_type = 162;
pub const MSG_REQ_REDIS_QUIT: msg_type = 161;
pub const MSG_REQ_REDIS_PING: msg_type = 160;
pub const MSG_REQ_REDIS_EVALSHA: msg_type = 159;
pub const MSG_REQ_REDIS_EVAL: msg_type = 158;
pub const MSG_REQ_REDIS_GEOSEARCHSTORE: msg_type = 157;
pub const MSG_REQ_REDIS_GEOSEARCH: msg_type = 156;
pub const MSG_REQ_REDIS_GEOPOS: msg_type = 155;
pub const MSG_REQ_REDIS_GEORADIUSBYMEMBER: msg_type = 154;
pub const MSG_REQ_REDIS_GEORADIUS: msg_type = 153;
pub const MSG_REQ_REDIS_GEOHASH: msg_type = 152;
pub const MSG_REQ_REDIS_GEODIST: msg_type = 151;
pub const MSG_REQ_REDIS_GEOADD: msg_type = 150;
pub const MSG_REQ_REDIS_ZUNIONSTORE: msg_type = 149;
pub const MSG_REQ_REDIS_ZSCORE: msg_type = 148;
pub const MSG_REQ_REDIS_ZSCAN: msg_type = 147;
pub const MSG_REQ_REDIS_ZUNION: msg_type = 146;
pub const MSG_REQ_REDIS_ZREVRANK: msg_type = 145;
pub const MSG_REQ_REDIS_ZREVRANGEBYSCORE: msg_type = 144;
pub const MSG_REQ_REDIS_ZREVRANGEBYLEX: msg_type = 143;
pub const MSG_REQ_REDIS_ZREVRANGE: msg_type = 142;
pub const MSG_REQ_REDIS_ZREMRANGEBYSCORE: msg_type = 141;
pub const MSG_REQ_REDIS_ZREMRANGEBYLEX: msg_type = 140;
pub const MSG_REQ_REDIS_ZREMRANGEBYRANK: msg_type = 139;
pub const MSG_REQ_REDIS_ZREM: msg_type = 138;
pub const MSG_REQ_REDIS_ZRANK: msg_type = 137;
pub const MSG_REQ_REDIS_ZRANGESTORE: msg_type = 136;
pub const MSG_REQ_REDIS_ZRANGEBYSCORE: msg_type = 135;
pub const MSG_REQ_REDIS_ZRANGEBYLEX: msg_type = 134;
pub const MSG_REQ_REDIS_ZRANGE: msg_type = 133;
pub const MSG_REQ_REDIS_ZRANDMEMBER: msg_type = 132;
pub const MSG_REQ_REDIS_ZPOPMAX: msg_type = 131;
pub const MSG_REQ_REDIS_ZPOPMIN: msg_type = 130;
pub const MSG_REQ_REDIS_ZMSCORE: msg_type = 129;
pub const MSG_REQ_REDIS_ZLEXCOUNT: msg_type = 128;
pub const MSG_REQ_REDIS_ZINTERSTORE: msg_type = 127;
pub const MSG_REQ_REDIS_ZINTER: msg_type = 126;
pub const MSG_REQ_REDIS_ZINCRBY: msg_type = 125;
pub const MSG_REQ_REDIS_ZDIFFSTORE: msg_type = 124;
pub const MSG_REQ_REDIS_ZDIFF: msg_type = 123;
pub const MSG_REQ_REDIS_ZCOUNT: msg_type = 122;
pub const MSG_REQ_REDIS_ZCARD: msg_type = 121;
pub const MSG_REQ_REDIS_ZADD: msg_type = 120;
pub const MSG_REQ_REDIS_SSCAN: msg_type = 119;
pub const MSG_REQ_REDIS_SUNIONSTORE: msg_type = 118;
pub const MSG_REQ_REDIS_SUNION: msg_type = 117;
pub const MSG_REQ_REDIS_SREM: msg_type = 116;
pub const MSG_REQ_REDIS_SRANDMEMBER: msg_type = 115;
pub const MSG_REQ_REDIS_SPOP: msg_type = 114;
pub const MSG_REQ_REDIS_SMOVE: msg_type = 113;
pub const MSG_REQ_REDIS_SMEMBERS: msg_type = 112;
pub const MSG_REQ_REDIS_SMISMEMBER: msg_type = 111;
pub const MSG_REQ_REDIS_SISMEMBER: msg_type = 110;
pub const MSG_REQ_REDIS_SINTERSTORE: msg_type = 109;
pub const MSG_REQ_REDIS_SINTER: msg_type = 108;
pub const MSG_REQ_REDIS_SDIFFSTORE: msg_type = 107;
pub const MSG_REQ_REDIS_SDIFF: msg_type = 106;
pub const MSG_REQ_REDIS_SCARD: msg_type = 105;
pub const MSG_REQ_REDIS_SADD: msg_type = 104;
pub const MSG_REQ_REDIS_RPUSHX: msg_type = 103;
pub const MSG_REQ_REDIS_RPUSH: msg_type = 102;
pub const MSG_REQ_REDIS_RPOPLPUSH: msg_type = 101;
pub const MSG_REQ_REDIS_RPOP: msg_type = 100;
pub const MSG_REQ_REDIS_PFMERGE: msg_type = 99;
pub const MSG_REQ_REDIS_PFCOUNT: msg_type = 98;
pub const MSG_REQ_REDIS_PFADD: msg_type = 97;
pub const MSG_REQ_REDIS_LTRIM: msg_type = 96;
pub const MSG_REQ_REDIS_LSET: msg_type = 95;
pub const MSG_REQ_REDIS_LREM: msg_type = 94;
pub const MSG_REQ_REDIS_LRANGE: msg_type = 93;
pub const MSG_REQ_REDIS_LPUSHX: msg_type = 92;
pub const MSG_REQ_REDIS_LPUSH: msg_type = 91;
pub const MSG_REQ_REDIS_LPOS: msg_type = 90;
pub const MSG_REQ_REDIS_LPOP: msg_type = 89;
pub const MSG_REQ_REDIS_LMOVE: msg_type = 88;
pub const MSG_REQ_REDIS_LLEN: msg_type = 87;
pub const MSG_REQ_REDIS_LINSERT: msg_type = 86;
pub const MSG_REQ_REDIS_LINDEX: msg_type = 85;
pub const MSG_REQ_REDIS_HVALS: msg_type = 84;
pub const MSG_REQ_REDIS_HSTRLEN: msg_type = 83;
pub const MSG_REQ_REDIS_HSCAN: msg_type = 82;
pub const MSG_REQ_REDIS_HSETNX: msg_type = 81;
pub const MSG_REQ_REDIS_HSET: msg_type = 80;
pub const MSG_REQ_REDIS_HRANDFIELD: msg_type = 79;
pub const MSG_REQ_REDIS_HMSET: msg_type = 78;
pub const MSG_REQ_REDIS_HMGET: msg_type = 77;
pub const MSG_REQ_REDIS_HLEN: msg_type = 76;
pub const MSG_REQ_REDIS_HKEYS: msg_type = 75;
pub const MSG_REQ_REDIS_HINCRBYFLOAT: msg_type = 74;
pub const MSG_REQ_REDIS_HINCRBY: msg_type = 73;
pub const MSG_REQ_REDIS_HGETALL: msg_type = 72;
pub const MSG_REQ_REDIS_HGET: msg_type = 71;
pub const MSG_REQ_REDIS_HEXISTS: msg_type = 70;
pub const MSG_REQ_REDIS_HDEL: msg_type = 69;
pub const MSG_REQ_REDIS_STRLEN: msg_type = 68;
pub const MSG_REQ_REDIS_SETRANGE: msg_type = 67;
pub const MSG_REQ_REDIS_SETNX: msg_type = 66;
pub const MSG_REQ_REDIS_SETEX: msg_type = 65;
pub const MSG_REQ_REDIS_SETBIT: msg_type = 64;
pub const MSG_REQ_REDIS_SET: msg_type = 63;
pub const MSG_REQ_REDIS_RESTORE: msg_type = 62;
pub const MSG_REQ_REDIS_PSETEX: msg_type = 61;
pub const MSG_REQ_REDIS_MSET: msg_type = 60;
pub const MSG_REQ_REDIS_MGET: msg_type = 59;
pub const MSG_REQ_REDIS_INCRBYFLOAT: msg_type = 58;
pub const MSG_REQ_REDIS_INCRBY: msg_type = 57;
pub const MSG_REQ_REDIS_INCR: msg_type = 56;
pub const MSG_REQ_REDIS_GETSET: msg_type = 55;
pub const MSG_REQ_REDIS_GETRANGE: msg_type = 54;
pub const MSG_REQ_REDIS_GETEX: msg_type = 53;
pub const MSG_REQ_REDIS_GETDEL: msg_type = 52;
pub const MSG_REQ_REDIS_GETBIT: msg_type = 51;
pub const MSG_REQ_REDIS_GET: msg_type = 50;
pub const MSG_REQ_REDIS_DUMP: msg_type = 49;
pub const MSG_REQ_REDIS_DECRBY: msg_type = 48;
pub const MSG_REQ_REDIS_DECR: msg_type = 47;
pub const MSG_REQ_REDIS_BITPOS: msg_type = 46;
pub const MSG_REQ_REDIS_BITFIELD: msg_type = 45;
pub const MSG_REQ_REDIS_BITCOUNT: msg_type = 44;
pub const MSG_REQ_REDIS_APPEND: msg_type = 43;
pub const MSG_REQ_REDIS_UNLINK: msg_type = 42;
pub const MSG_REQ_REDIS_TYPE: msg_type = 41;
pub const MSG_REQ_REDIS_TTL: msg_type = 40;
pub const MSG_REQ_REDIS_TOUCH: msg_type = 39;
pub const MSG_REQ_REDIS_SORT: msg_type = 38;
pub const MSG_REQ_REDIS_PTTL: msg_type = 37;
pub const MSG_REQ_REDIS_PERSIST: msg_type = 36;
pub const MSG_REQ_REDIS_PEXPIREAT: msg_type = 35;
pub const MSG_REQ_REDIS_PEXPIRE: msg_type = 34;
pub const MSG_REQ_REDIS_MOVE: msg_type = 33;
pub const MSG_REQ_REDIS_EXPIREAT: msg_type = 32;
pub const MSG_REQ_REDIS_EXPIRE: msg_type = 31;
pub const MSG_REQ_REDIS_EXISTS: msg_type = 30;
pub const MSG_REQ_REDIS_DEL: msg_type = 29;
pub const MSG_REQ_REDIS_COPY: msg_type = 28;
pub const MSG_RSP_MC_SERVER_ERROR: msg_type = 27;
pub const MSG_RSP_MC_CLIENT_ERROR: msg_type = 26;
pub const MSG_RSP_MC_ERROR: msg_type = 25;
pub const MSG_RSP_MC_VERSION: msg_type = 24;
pub const MSG_RSP_MC_TOUCHED: msg_type = 23;
pub const MSG_RSP_MC_DELETED: msg_type = 22;
pub const MSG_RSP_MC_VALUE: msg_type = 21;
pub const MSG_RSP_MC_END: msg_type = 20;
pub const MSG_RSP_MC_NOT_FOUND: msg_type = 19;
pub const MSG_RSP_MC_EXISTS: msg_type = 18;
pub const MSG_RSP_MC_NOT_STORED: msg_type = 17;
pub const MSG_RSP_MC_STORED: msg_type = 16;
pub const MSG_RSP_MC_NUM: msg_type = 15;
pub const MSG_REQ_MC_VERSION: msg_type = 14;
pub const MSG_REQ_MC_QUIT: msg_type = 13;
pub const MSG_REQ_MC_TOUCH: msg_type = 12;
pub const MSG_REQ_MC_DECR: msg_type = 11;
pub const MSG_REQ_MC_INCR: msg_type = 10;
pub const MSG_REQ_MC_PREPEND: msg_type = 9;
pub const MSG_REQ_MC_APPEND: msg_type = 8;
pub const MSG_REQ_MC_REPLACE: msg_type = 7;
pub const MSG_REQ_MC_ADD: msg_type = 6;
pub const MSG_REQ_MC_SET: msg_type = 5;
pub const MSG_REQ_MC_CAS: msg_type = 4;
pub const MSG_REQ_MC_DELETE: msg_type = 3;
pub const MSG_REQ_MC_GETS: msg_type = 2;
pub const MSG_REQ_MC_GET: msg_type = 1;
pub const MSG_UNKNOWN: msg_type = 0;
pub type msg_coalesce_t = Option::<unsafe extern "C" fn(*mut msg) -> ()>;
pub type msg_failure_t = Option::<unsafe extern "C" fn(*const msg) -> bool>;
pub type msg_add_auth_t = Option::<
    unsafe extern "C" fn(*mut context, *mut conn, *mut conn) -> rstatus_t,
>;
pub type msg_reply_t = Option::<unsafe extern "C" fn(*mut msg) -> rstatus_t>;
pub type msg_fragment_t = Option::<
    unsafe extern "C" fn(*mut msg, uint32_t, *mut msg_tqh) -> rstatus_t,
>;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct msg_tqh {
    pub tqh_first: *mut msg,
    pub tqh_last: *mut *mut msg,
}
pub type msg_parse_result_t = msg_parse_result;
pub type msg_parse_result = libc::c_uint;
pub const MSG_PARSE_AGAIN: msg_parse_result = 3;
pub const MSG_PARSE_REPAIR: msg_parse_result = 2;
pub const MSG_PARSE_ERROR: msg_parse_result = 1;
pub const MSG_PARSE_OK: msg_parse_result = 0;
pub type msg_parse_t = Option::<unsafe extern "C" fn(*mut msg) -> ()>;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct mhdr {
    pub stqh_first: *mut mbuf,
    pub stqh_last: *mut *mut mbuf,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct mbuf {
    pub magic: uint32_t,
    pub next: C2RustUnnamed,
    pub pos: *mut uint8_t,
    pub last: *mut uint8_t,
    pub start: *mut uint8_t,
    pub end: *mut uint8_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed {
    pub stqe_next: *mut mbuf,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct rbnode {
    pub left: *mut rbnode,
    pub right: *mut rbnode,
    pub parent: *mut rbnode,
    pub key: int64_t,
    pub data: *mut libc::c_void,
    pub color: uint8_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_0 {
    pub tqe_next: *mut msg,
    pub tqe_prev: *mut *mut msg,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_1 {
    pub tqe_next: *mut msg,
    pub tqe_prev: *mut *mut msg,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_2 {
    pub tqe_next: *mut msg,
    pub tqe_prev: *mut *mut msg,
}
pub type conn_unref_t = Option::<unsafe extern "C" fn(*mut conn) -> ()>;
pub type conn_ref_t = Option::<unsafe extern "C" fn(*mut conn, *mut libc::c_void) -> ()>;
pub type conn_swallow_msg_t = Option::<
    unsafe extern "C" fn(*mut conn, *mut msg, *mut msg) -> (),
>;
pub type conn_post_connect_t = Option::<
    unsafe extern "C" fn(*mut context, *mut conn, *mut server) -> (),
>;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct server {
    pub idx: uint32_t,
    pub owner: *mut server_pool,
    pub pname: string,
    pub name: string,
    pub addrstr: string,
    pub port: uint16_t,
    pub weight: uint32_t,
    pub info: sockinfo,
    pub ns_conn_q: uint32_t,
    pub s_conn_q: conn_tqh,
    pub next_retry: int64_t,
    pub failure_count: uint32_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct conn_tqh {
    pub tqh_first: *mut conn,
    pub tqh_last: *mut *mut conn,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct sockinfo {
    pub family: libc::c_int,
    pub addrlen: socklen_t,
    pub addr: C2RustUnnamed_3,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_3 {
    pub in_0: sockaddr_in,
    pub in6: sockaddr_in6,
    pub un: sockaddr_un,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct sockaddr_un {
    pub sun_family: sa_family_t,
    pub sun_path: [libc::c_char; 108],
}
pub type sa_family_t = libc::c_ushort;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct sockaddr_in6 {
    pub sin6_family: sa_family_t,
    pub sin6_port: in_port_t,
    pub sin6_flowinfo: uint32_t,
    pub sin6_addr: in6_addr,
    pub sin6_scope_id: uint32_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct in6_addr {
    pub __in6_u: C2RustUnnamed_4,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_4 {
    pub __u6_addr8: [uint8_t; 16],
    pub __u6_addr16: [uint16_t; 8],
    pub __u6_addr32: [uint32_t; 4],
}
pub type in_port_t = uint16_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct sockaddr_in {
    pub sin_family: sa_family_t,
    pub sin_port: in_port_t,
    pub sin_addr: in_addr,
    pub sin_zero: [libc::c_uchar; 8],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct in_addr {
    pub s_addr: in_addr_t,
}
pub type in_addr_t = uint32_t;
pub type socklen_t = __socklen_t;
pub type __socklen_t = libc::c_uint;
#[derive(Copy, Clone, BitfieldStruct)]
#[repr(C)]
pub struct server_pool {
    pub idx: uint32_t,
    pub ctx: *mut context,
    pub p_conn: *mut conn,
    pub nc_conn_q: uint32_t,
    pub c_conn_q: conn_tqh,
    pub server: array,
    pub ncontinuum: uint32_t,
    pub nserver_continuum: uint32_t,
    pub continuum: *mut continuum,
    pub nlive_server: uint32_t,
    pub next_rebuild: int64_t,
    pub name: string,
    pub addrstr: string,
    pub port: uint16_t,
    pub info: sockinfo,
    pub perm: mode_t,
    pub dist_type: libc::c_int,
    pub key_hash_type: libc::c_int,
    pub key_hash: hash_t,
    pub hash_tag: string,
    pub timeout: libc::c_int,
    pub backlog: libc::c_int,
    pub redis_db: libc::c_int,
    pub client_connections: uint32_t,
    pub server_connections: uint32_t,
    pub server_retry_timeout: int64_t,
    pub server_failure_limit: uint32_t,
    pub redis_auth: string,
    pub require_auth: libc::c_uint,
    #[bitfield(name = "auto_eject_hosts", ty = "libc::c_uint", bits = "0..=0")]
    #[bitfield(name = "preconnect", ty = "libc::c_uint", bits = "1..=1")]
    #[bitfield(name = "redis", ty = "libc::c_uint", bits = "2..=2")]
    #[bitfield(name = "tcpkeepalive", ty = "libc::c_uint", bits = "3..=3")]
    #[bitfield(name = "reuseport", ty = "libc::c_uint", bits = "4..=4")]
    pub auto_eject_hosts_preconnect_redis_tcpkeepalive_reuseport: [u8; 1],
    #[bitfield(padding)]
    pub c2rust_padding: [u8; 3],
}
pub type hash_t = Option::<
    unsafe extern "C" fn(*const libc::c_char, size_t) -> uint32_t,
>;
pub type mode_t = __mode_t;
pub type __mode_t = libc::c_uint;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct continuum {
    pub index: uint32_t,
    pub value: uint32_t,
}
pub type conn_active_t = Option::<unsafe extern "C" fn(*const conn) -> bool>;
pub type conn_close_t = Option::<unsafe extern "C" fn(*mut context, *mut conn) -> ()>;
pub type conn_send_done_t = Option::<
    unsafe extern "C" fn(*mut context, *mut conn, *mut msg) -> (),
>;
pub type conn_send_next_t = Option::<
    unsafe extern "C" fn(*mut context, *mut conn) -> *mut msg,
>;
pub type conn_send_t = Option::<
    unsafe extern "C" fn(*mut context, *mut conn) -> rstatus_t,
>;
pub type conn_recv_done_t = Option::<
    unsafe extern "C" fn(*mut context, *mut conn, *mut msg, *mut msg) -> (),
>;
pub type conn_recv_next_t = Option::<
    unsafe extern "C" fn(*mut context, *mut conn, bool) -> *mut msg,
>;
pub type conn_recv_t = Option::<
    unsafe extern "C" fn(*mut context, *mut conn) -> rstatus_t,
>;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct sockaddr {
    pub sa_family: sa_family_t,
    pub sa_data: [libc::c_char; 14],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_5 {
    pub tqe_next: *mut conn,
    pub tqe_prev: *mut *mut conn,
}
pub type stats_server_field = libc::c_uint;
pub const STATS_SERVER_NFIELD: stats_server_field = 13;
pub const STATS_SERVER_out_queue_bytes: stats_server_field = 12;
pub const STATS_SERVER_out_queue: stats_server_field = 11;
pub const STATS_SERVER_in_queue_bytes: stats_server_field = 10;
pub const STATS_SERVER_in_queue: stats_server_field = 9;
pub const STATS_SERVER_response_bytes: stats_server_field = 8;
pub const STATS_SERVER_responses: stats_server_field = 7;
pub const STATS_SERVER_request_bytes: stats_server_field = 6;
pub const STATS_SERVER_requests: stats_server_field = 5;
pub const STATS_SERVER_server_ejected_at: stats_server_field = 4;
pub const STATS_SERVER_server_connections: stats_server_field = 3;
pub const STATS_SERVER_server_timedout: stats_server_field = 2;
pub const STATS_SERVER_server_err: stats_server_field = 1;
pub const STATS_SERVER_server_eof: stats_server_field = 0;
pub type stats_server_field_t = stats_server_field;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct keypos {
    pub start: *mut uint8_t,
    pub end: *mut uint8_t,
}
#[inline]
unsafe extern "C" fn array_n(mut a: *const array) -> uint32_t {
    return (*a).nelem;
}
pub unsafe extern "C" fn req_get(mut conn: *mut conn) -> *mut msg {
    let mut msg: *mut msg = 0 as *mut msg;
    msg = msg_get(conn, 1 as libc::c_int != 0, (*conn).redis() != 0);
    if msg.is_null() {
        (*conn).err = *__errno_location();
    }
    return msg;
}
unsafe extern "C" fn req_log(mut req: *const msg) {
    let mut rsp: *const msg = 0 as *const msg;
    let mut req_time: int64_t = 0;
    let mut peer_str: *const libc::c_char = 0 as *const libc::c_char;
    let mut req_len: uint32_t = 0;
    let mut rsp_len: uint32_t = 0;
    let mut req_type: *const string = 0 as *const string;
    let mut kpos: *const keypos = 0 as *const keypos;
    if log_loggable(5 as libc::c_int) == 0 as libc::c_int {
        return;
    }
    if (*req).frag_id != 0 as libc::c_int as libc::c_ulong
        && (*req).frag_owner != req as *mut msg
    {
        return;
    }
    if (*req).mlen == 0 as libc::c_int as libc::c_uint {
        return;
    }
    if (*req).start_ts == 0 as libc::c_int as libc::c_long {
        return;
    }
    req_time = nc_usec_now() - (*req).start_ts;
    rsp = (*req).peer;
    req_len = (*req).mlen;
    rsp_len = if !rsp.is_null() {
        (*rsp).mlen
    } else {
        0 as libc::c_int as libc::c_uint
    };
    if array_n((*req).keys) < 1 as libc::c_int as libc::c_uint {
        return;
    }
    kpos = array_get((*req).keys, 0 as libc::c_int as uint32_t) as *const keypos;
    peer_str = nc_unresolve_peer_desc((*(*req).owner).sd);
    req_type = msg_type_string((*req).type_0);
}
pub unsafe extern "C" fn req_put(mut msg: *mut msg) {
    let mut pmsg: *mut msg = 0 as *mut msg;
    req_log(msg);
    pmsg = (*msg).peer;
    if !pmsg.is_null() {
        (*msg).peer = 0 as *mut msg;
        (*pmsg).peer = 0 as *mut msg;
        rsp_put(pmsg);
    }
    msg_tmo_delete(msg);
    msg_put(msg);
}
pub unsafe extern "C" fn req_done(mut conn: *const conn, mut msg: *mut msg) -> bool {
    let mut cmsg: *mut msg = 0 as *mut msg;
    let mut id: uint64_t = 0;
    let mut nfragment: uint32_t = 0;
    if (*msg).done() == 0 {
        return 0 as libc::c_int != 0;
    }
    id = (*msg).frag_id;
    if id == 0 as libc::c_int as libc::c_ulong {
        return 1 as libc::c_int != 0;
    }
    if (*msg).fdone() != 0 {
        return 1 as libc::c_int != 0;
    }
    if (*msg).nfrag_done < (*msg).nfrag {
        return 0 as libc::c_int != 0;
    }
    cmsg = *(*((*msg).c_tqe.tqe_prev as *mut msg_tqh)).tqh_last;
    while !cmsg.is_null() && (*cmsg).frag_id == id {
        if (*cmsg).done() == 0 {
            return 0 as libc::c_int != 0;
        }
        cmsg = *(*((*cmsg).c_tqe.tqe_prev as *mut msg_tqh)).tqh_last;
    }
    cmsg = (*msg).c_tqe.tqe_next;
    while !cmsg.is_null() && (*cmsg).frag_id == id {
        if (*cmsg).done() == 0 {
            return 0 as libc::c_int != 0;
        }
        cmsg = (*cmsg).c_tqe.tqe_next;
    }
    (*msg).set_fdone(1 as libc::c_int as libc::c_uint);
    nfragment = 0 as libc::c_int as uint32_t;
    cmsg = *(*((*msg).c_tqe.tqe_prev as *mut msg_tqh)).tqh_last;
    while !cmsg.is_null() && (*cmsg).frag_id == id {
        (*cmsg).set_fdone(1 as libc::c_int as libc::c_uint);
        nfragment = nfragment.wrapping_add(1);
        nfragment;
        cmsg = *(*((*cmsg).c_tqe.tqe_prev as *mut msg_tqh)).tqh_last;
    }
    cmsg = (*msg).c_tqe.tqe_next;
    while !cmsg.is_null() && (*cmsg).frag_id == id {
        (*cmsg).set_fdone(1 as libc::c_int as libc::c_uint);
        nfragment = nfragment.wrapping_add(1);
        nfragment;
        cmsg = (*cmsg).c_tqe.tqe_next;
    }
    ((*msg).post_coalesce).unwrap()((*msg).frag_owner);
    return 1 as libc::c_int != 0;
}
pub unsafe extern "C" fn req_error(mut conn: *const conn, mut msg: *mut msg) -> bool {
    let mut current_block: u64;
    let mut cmsg: *mut msg = 0 as *mut msg;
    let mut id: uint64_t = 0;
    let mut nfragment: uint32_t = 0;
    if (*msg).error() != 0 {
        return 1 as libc::c_int != 0;
    }
    id = (*msg).frag_id;
    if id == 0 as libc::c_int as libc::c_ulong {
        return 0 as libc::c_int != 0;
    }
    if (*msg).ferror() != 0 {
        return 1 as libc::c_int != 0;
    }
    cmsg = *(*((*msg).c_tqe.tqe_prev as *mut msg_tqh)).tqh_last;
    loop {
        if !(!cmsg.is_null() && (*cmsg).frag_id == id) {
            current_block = 1841672684692190573;
            break;
        }
        if (*cmsg).error() != 0 {
            current_block = 8670815411212808193;
            break;
        }
        cmsg = *(*((*cmsg).c_tqe.tqe_prev as *mut msg_tqh)).tqh_last;
    }
    match current_block {
        1841672684692190573 => {
            cmsg = (*msg).c_tqe.tqe_next;
            loop {
                if !(!cmsg.is_null() && (*cmsg).frag_id == id) {
                    current_block = 8831408221741692167;
                    break;
                }
                if (*cmsg).error() != 0 {
                    current_block = 8670815411212808193;
                    break;
                }
                cmsg = (*cmsg).c_tqe.tqe_next;
            }
            match current_block {
                8670815411212808193 => {}
                _ => return 0 as libc::c_int != 0,
            }
        }
        _ => {}
    }
    (*msg).set_ferror(1 as libc::c_int as libc::c_uint);
    nfragment = 1 as libc::c_int as uint32_t;
    cmsg = *(*((*msg).c_tqe.tqe_prev as *mut msg_tqh)).tqh_last;
    while !cmsg.is_null() && (*cmsg).frag_id == id {
        (*cmsg).set_ferror(1 as libc::c_int as libc::c_uint);
        nfragment = nfragment.wrapping_add(1);
        nfragment;
        cmsg = *(*((*cmsg).c_tqe.tqe_prev as *mut msg_tqh)).tqh_last;
    }
    cmsg = (*msg).c_tqe.tqe_next;
    while !cmsg.is_null() && (*cmsg).frag_id == id {
        (*cmsg).set_ferror(1 as libc::c_int as libc::c_uint);
        nfragment = nfragment.wrapping_add(1);
        nfragment;
        cmsg = (*cmsg).c_tqe.tqe_next;
    }
    return 1 as libc::c_int != 0;
}
pub unsafe extern "C" fn req_server_enqueue_imsgq(
    mut ctx: *mut context,
    mut conn: *mut conn,
    mut msg: *mut msg,
) {
    if (*msg).noreply() == 0 {
        msg_tmo_insert(msg, conn);
    }
    (*msg).s_tqe.tqe_next = 0 as *mut msg;
    (*msg).s_tqe.tqe_prev = (*conn).imsg_q.tqh_last;
    *(*conn).imsg_q.tqh_last = msg;
    (*conn).imsg_q.tqh_last = &mut (*msg).s_tqe.tqe_next;
    _stats_server_incr(ctx, (*conn).owner as *const server, STATS_SERVER_in_queue);
    _stats_server_incr_by(
        ctx,
        (*conn).owner as *const server,
        STATS_SERVER_in_queue_bytes,
        (*msg).mlen as int64_t,
    );
}
pub unsafe extern "C" fn req_server_enqueue_imsgq_head(
    mut ctx: *mut context,
    mut conn: *mut conn,
    mut msg: *mut msg,
) {
    if (*msg).noreply() == 0 {
        msg_tmo_insert(msg, conn);
    }
    (*msg).s_tqe.tqe_next = (*conn).imsg_q.tqh_first;
    if !((*msg).s_tqe.tqe_next).is_null() {
        (*(*conn).imsg_q.tqh_first).s_tqe.tqe_prev = &mut (*msg).s_tqe.tqe_next;
    } else {
        (*conn).imsg_q.tqh_last = &mut (*msg).s_tqe.tqe_next;
    }
    (*conn).imsg_q.tqh_first = msg;
    (*msg).s_tqe.tqe_prev = &mut (*conn).imsg_q.tqh_first;
    _stats_server_incr(ctx, (*conn).owner as *const server, STATS_SERVER_in_queue);
    _stats_server_incr_by(
        ctx,
        (*conn).owner as *const server,
        STATS_SERVER_in_queue_bytes,
        (*msg).mlen as int64_t,
    );
}
pub unsafe extern "C" fn req_server_dequeue_imsgq(
    mut ctx: *mut context,
    mut conn: *mut conn,
    mut msg: *mut msg,
) {
    let mut oldnext: *mut *mut libc::c_void = &mut (*msg).s_tqe.tqe_next as *mut *mut msg
        as *mut libc::c_void as *mut *mut libc::c_void;
    let mut oldprev: *mut *mut libc::c_void = &mut (*msg).s_tqe.tqe_prev
        as *mut *mut *mut msg as *mut libc::c_void as *mut *mut libc::c_void;
    if !((*msg).s_tqe.tqe_next).is_null() {
        (*(*msg).s_tqe.tqe_next).s_tqe.tqe_prev = (*msg).s_tqe.tqe_prev;
    } else {
        (*conn).imsg_q.tqh_last = (*msg).s_tqe.tqe_prev;
    }
    *(*msg).s_tqe.tqe_prev = (*msg).s_tqe.tqe_next;
    *oldnext = 0 as *mut libc::c_void;
    *oldprev = 0 as *mut libc::c_void;
    _stats_server_decr(ctx, (*conn).owner as *const server, STATS_SERVER_in_queue);
    _stats_server_decr_by(
        ctx,
        (*conn).owner as *const server,
        STATS_SERVER_in_queue_bytes,
        (*msg).mlen as int64_t,
    );
}
pub unsafe extern "C" fn req_client_enqueue_omsgq(
    mut ctx: *mut context,
    mut conn: *mut conn,
    mut msg: *mut msg,
) {
    (*msg).c_tqe.tqe_next = 0 as *mut msg;
    (*msg).c_tqe.tqe_prev = (*conn).omsg_q.tqh_last;
    *(*conn).omsg_q.tqh_last = msg;
    (*conn).omsg_q.tqh_last = &mut (*msg).c_tqe.tqe_next;
}
pub unsafe extern "C" fn req_server_enqueue_omsgq(
    mut ctx: *mut context,
    mut conn: *mut conn,
    mut msg: *mut msg,
) {
    (*msg).s_tqe.tqe_next = 0 as *mut msg;
    (*msg).s_tqe.tqe_prev = (*conn).omsg_q.tqh_last;
    *(*conn).omsg_q.tqh_last = msg;
    (*conn).omsg_q.tqh_last = &mut (*msg).s_tqe.tqe_next;
    _stats_server_incr(ctx, (*conn).owner as *const server, STATS_SERVER_out_queue);
    _stats_server_incr_by(
        ctx,
        (*conn).owner as *const server,
        STATS_SERVER_out_queue_bytes,
        (*msg).mlen as int64_t,
    );
}
pub unsafe extern "C" fn req_client_dequeue_omsgq(
    mut ctx: *mut context,
    mut conn: *mut conn,
    mut msg: *mut msg,
) {
    let mut oldnext: *mut *mut libc::c_void = &mut (*msg).c_tqe.tqe_next as *mut *mut msg
        as *mut libc::c_void as *mut *mut libc::c_void;
    let mut oldprev: *mut *mut libc::c_void = &mut (*msg).c_tqe.tqe_prev
        as *mut *mut *mut msg as *mut libc::c_void as *mut *mut libc::c_void;
    if !((*msg).c_tqe.tqe_next).is_null() {
        (*(*msg).c_tqe.tqe_next).c_tqe.tqe_prev = (*msg).c_tqe.tqe_prev;
    } else {
        (*conn).omsg_q.tqh_last = (*msg).c_tqe.tqe_prev;
    }
    *(*msg).c_tqe.tqe_prev = (*msg).c_tqe.tqe_next;
    *oldnext = 0 as *mut libc::c_void;
    *oldprev = 0 as *mut libc::c_void;
}
pub unsafe extern "C" fn req_server_dequeue_omsgq(
    mut ctx: *mut context,
    mut conn: *mut conn,
    mut msg: *mut msg,
) {
    msg_tmo_delete(msg);
    let mut oldnext: *mut *mut libc::c_void = &mut (*msg).s_tqe.tqe_next as *mut *mut msg
        as *mut libc::c_void as *mut *mut libc::c_void;
    let mut oldprev: *mut *mut libc::c_void = &mut (*msg).s_tqe.tqe_prev
        as *mut *mut *mut msg as *mut libc::c_void as *mut *mut libc::c_void;
    if !((*msg).s_tqe.tqe_next).is_null() {
        (*(*msg).s_tqe.tqe_next).s_tqe.tqe_prev = (*msg).s_tqe.tqe_prev;
    } else {
        (*conn).omsg_q.tqh_last = (*msg).s_tqe.tqe_prev;
    }
    *(*msg).s_tqe.tqe_prev = (*msg).s_tqe.tqe_next;
    *oldnext = 0 as *mut libc::c_void;
    *oldprev = 0 as *mut libc::c_void;
    _stats_server_decr(ctx, (*conn).owner as *const server, STATS_SERVER_out_queue);
    _stats_server_decr_by(
        ctx,
        (*conn).owner as *const server,
        STATS_SERVER_out_queue_bytes,
        (*msg).mlen as int64_t,
    );
}
pub unsafe extern "C" fn req_recv_next(
    mut ctx: *mut context,
    mut conn: *mut conn,
    mut alloc: bool,
) -> *mut msg {
    let mut msg: *mut msg = 0 as *mut msg;
    if (*conn).eof() != 0 {
        msg = (*conn).rmsg;
        if !msg.is_null() {
            (*conn).rmsg = 0 as *mut msg;
            if log_loggable(1 as libc::c_int) != 0 as libc::c_int {
                _log(
                    b"nc_request.c\0" as *const u8 as *const libc::c_char,
                    416 as libc::c_int,
                    0 as libc::c_int,
                    b"eof c %d discarding incomplete req %lu len %u\0" as *const u8
                        as *const libc::c_char,
                    (*conn).sd,
                    (*msg).id,
                    (*msg).mlen,
                );
            }
            req_put(msg);
        }
        if !((*conn).active).unwrap()(conn) {
            (*conn).set_done(1 as libc::c_int as libc::c_uint);
        }
        return 0 as *mut msg;
    }
    msg = (*conn).rmsg;
    if !msg.is_null() {
        return msg;
    }
    if !alloc {
        return 0 as *mut msg;
    }
    msg = req_get(conn);
    if !msg.is_null() {
        (*conn).rmsg = msg;
    }
    return msg;
}
unsafe extern "C" fn req_make_reply(
    mut ctx: *mut context,
    mut conn: *mut conn,
    mut req: *mut msg,
) -> rstatus_t {
    let mut rsp: *mut msg = 0 as *mut msg;
    rsp = msg_get(conn, 0 as libc::c_int != 0, (*conn).redis() != 0);
    if rsp.is_null() {
        (*conn).err = *__errno_location();
        return -(3 as libc::c_int);
    }
    (*req).peer = rsp;
    (*rsp).peer = req;
    (*rsp).set_request(0 as libc::c_int as libc::c_uint);
    (*req).set_done(1 as libc::c_int as libc::c_uint);
    ((*conn).enqueue_outq).unwrap()(ctx, conn, req);
    return 0 as libc::c_int;
}
unsafe extern "C" fn req_filter(mut conn: *mut conn, mut msg: *mut msg) -> bool {
    if msg_empty(msg) {
        req_put(msg);
        return 1 as libc::c_int != 0;
    }
    if (*msg).quit() != 0 {
        !((*conn).rmsg).is_null();
        (*conn).set_eof(1 as libc::c_int as libc::c_uint);
        (*conn).set_recv_ready(0 as libc::c_int as libc::c_uint);
        req_put(msg);
        return 1 as libc::c_int != 0;
    }
    if !conn_authenticated(conn) {
        (*msg).set_noforward(1 as libc::c_int as libc::c_uint);
    }
    return 0 as libc::c_int != 0;
}
unsafe extern "C" fn req_forward_error(
    mut ctx: *mut context,
    mut conn: *mut conn,
    mut msg: *mut msg,
) {
    let mut status: rstatus_t = 0;
    (*msg).set_done(1 as libc::c_int as libc::c_uint);
    (*msg).set_error(1 as libc::c_int as libc::c_uint);
    (*msg).err = *__errno_location();
    if (*msg).noreply() != 0 {
        req_put(msg);
        return;
    }
    if req_done(conn, (*conn).omsg_q.tqh_first) {
        status = event_add_out((*ctx).evb, conn);
        if status != 0 as libc::c_int {
            (*conn).err = *__errno_location();
        }
    }
}
unsafe extern "C" fn req_forward_stats(
    mut ctx: *mut context,
    mut server: *mut server,
    mut msg: *mut msg,
) {
    _stats_server_incr(ctx, server, STATS_SERVER_requests);
    _stats_server_incr_by(
        ctx,
        server,
        STATS_SERVER_request_bytes,
        (*msg).mlen as int64_t,
    );
}
unsafe extern "C" fn req_forward(
    mut ctx: *mut context,
    mut c_conn: *mut conn,
    mut msg: *mut msg,
) {
    let mut status: rstatus_t = 0;
    let mut s_conn: *mut conn = 0 as *mut conn;
    let mut key: *mut uint8_t = 0 as *mut uint8_t;
    let mut keylen: uint32_t = 0;
    let mut kpos: *mut keypos = 0 as *mut keypos;
    if (*msg).noreply() == 0 {
        ((*c_conn).enqueue_outq).unwrap()(ctx, c_conn, msg);
    }
    kpos = array_get((*msg).keys, 0 as libc::c_int as uint32_t) as *mut keypos;
    key = (*kpos).start;
    keylen = ((*kpos).end).offset_from((*kpos).start) as libc::c_long as uint32_t;
    s_conn = server_pool_conn(ctx, (*c_conn).owner as *mut server_pool, key, keylen);
    if s_conn.is_null() {
        if !((*msg).frag_owner).is_null() {
            (*(*msg).frag_owner)
                .nfrag_done = ((*(*msg).frag_owner).nfrag_done).wrapping_add(1);
            (*(*msg).frag_owner).nfrag_done;
        }
        req_forward_error(ctx, c_conn, msg);
        return;
    }
    if ((*s_conn).imsg_q.tqh_first).is_null() {
        status = event_add_out((*ctx).evb, s_conn);
        if status != 0 as libc::c_int {
            req_forward_error(ctx, c_conn, msg);
            (*s_conn).err = *__errno_location();
            return;
        }
    }
    if !conn_authenticated(s_conn) {
        status = ((*msg).add_auth).unwrap()(ctx, c_conn, s_conn);
        if status != 0 as libc::c_int {
            req_forward_error(ctx, c_conn, msg);
            (*s_conn).err = *__errno_location();
            return;
        }
    }
    ((*s_conn).enqueue_inq).unwrap()(ctx, s_conn, msg);
    req_forward_stats(ctx, (*s_conn).owner as *mut server, msg);
}
pub unsafe extern "C" fn req_recv_done(
    mut ctx: *mut context,
    mut conn: *mut conn,
    mut msg: *mut msg,
    mut nmsg: *mut msg,
) {
    let mut status: rstatus_t = 0;
    let mut pool: *mut server_pool = 0 as *mut server_pool;
    let mut frag_msgq: msg_tqh = msg_tqh {
        tqh_first: 0 as *mut msg,
        tqh_last: 0 as *mut *mut msg,
    };
    let mut sub_msg: *mut msg = 0 as *mut msg;
    let mut tmsg: *mut msg = 0 as *mut msg;
    (*conn).rmsg = nmsg;
    if req_filter(conn, msg) {
        return;
    }
    if (*msg).noforward() != 0 {
        status = req_make_reply(ctx, conn, msg);
        if status != 0 as libc::c_int {
            (*conn).err = *__errno_location();
            return;
        }
        status = ((*msg).reply).unwrap()(msg);
        if status != 0 as libc::c_int {
            (*conn).err = *__errno_location();
            return;
        }
        status = event_add_out((*ctx).evb, conn);
        if status != 0 as libc::c_int {
            (*conn).err = *__errno_location();
        }
        return;
    }
    pool = (*conn).owner as *mut server_pool;
    frag_msgq.tqh_first = 0 as *mut msg;
    frag_msgq.tqh_last = &mut frag_msgq.tqh_first;
    status = ((*msg).fragment)
        .unwrap()(msg, array_n(&mut (*pool).server), &mut frag_msgq);
    if status != 0 as libc::c_int {
        if (*msg).noreply() == 0 {
            ((*conn).enqueue_outq).unwrap()(ctx, conn, msg);
        }
        req_forward_error(ctx, conn, msg);
    }
    if (frag_msgq.tqh_first).is_null() {
        req_forward(ctx, conn, msg);
        return;
    }
    status = req_make_reply(ctx, conn, msg);
    if status != 0 as libc::c_int {
        if (*msg).noreply() == 0 {
            ((*conn).enqueue_outq).unwrap()(ctx, conn, msg);
        }
        req_forward_error(ctx, conn, msg);
    }
    sub_msg = frag_msgq.tqh_first;
    while !sub_msg.is_null() {
        tmsg = (*sub_msg).m_tqe.tqe_next;
        let mut oldnext: *mut *mut libc::c_void = &mut (*sub_msg).m_tqe.tqe_next
            as *mut *mut msg as *mut libc::c_void as *mut *mut libc::c_void;
        let mut oldprev: *mut *mut libc::c_void = &mut (*sub_msg).m_tqe.tqe_prev
            as *mut *mut *mut msg as *mut libc::c_void as *mut *mut libc::c_void;
        if !((*sub_msg).m_tqe.tqe_next).is_null() {
            (*(*sub_msg).m_tqe.tqe_next).m_tqe.tqe_prev = (*sub_msg).m_tqe.tqe_prev;
        } else {
            frag_msgq.tqh_last = (*sub_msg).m_tqe.tqe_prev;
        }
        *(*sub_msg).m_tqe.tqe_prev = (*sub_msg).m_tqe.tqe_next;
        *oldnext = 0 as *mut libc::c_void;
        *oldprev = 0 as *mut libc::c_void;
        req_forward(ctx, conn, sub_msg);
        sub_msg = tmsg;
    }
}
pub unsafe extern "C" fn req_send_next(
    mut ctx: *mut context,
    mut conn: *mut conn,
) -> *mut msg {
    let mut status: rstatus_t = 0;
    let mut msg: *mut msg = 0 as *mut msg;
    let mut nmsg: *mut msg = 0 as *mut msg;
    if (*conn).connecting() != 0 {
        server_connected(ctx, conn);
    }
    nmsg = (*conn).imsg_q.tqh_first;
    if nmsg.is_null() {
        status = event_del_out((*ctx).evb, conn);
        if status != 0 as libc::c_int {
            (*conn).err = *__errno_location();
        }
        return 0 as *mut msg;
    }
    msg = (*conn).smsg;
    if !msg.is_null() {
        nmsg = (*msg).s_tqe.tqe_next;
    }
    (*conn).smsg = nmsg;
    if nmsg.is_null() {
        return 0 as *mut msg;
    }
    return nmsg;
}
pub unsafe extern "C" fn req_send_done(
    mut ctx: *mut context,
    mut conn: *mut conn,
    mut msg: *mut msg,
) {
    ((*conn).dequeue_inq).unwrap()(ctx, conn, msg);
    if (*msg).noreply() == 0 {
        ((*conn).enqueue_outq).unwrap()(ctx, conn, msg);
    } else {
        req_put(msg);
    };
}
