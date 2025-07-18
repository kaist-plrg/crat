use ::libc;
use ::c2rust_bitfields;
extern "C" {
    pub type epoll_event;
    pub type conf;
    fn vsnprintf(
        _: *mut libc::c_char,
        _: libc::c_ulong,
        _: *const libc::c_char,
        _: ::std::ffi::VaList,
    ) -> libc::c_int;
    fn strerror(_: libc::c_int) -> *mut libc::c_char;
    fn __errno_location() -> *mut libc::c_int;
    fn array_create(n: uint32_t, size: size_t) -> *mut array;
    fn array_destroy(a: *mut array);
    fn array_push(a: *mut array) -> *mut libc::c_void;
    fn _nc_alloc(
        size: size_t,
        name: *const libc::c_char,
        line: libc::c_int,
    ) -> *mut libc::c_void;
    fn _nc_free(ptr: *mut libc::c_void, name: *const libc::c_char, line: libc::c_int);
    fn _scnprintf(
        buf: *mut libc::c_char,
        size: size_t,
        fmt: *const libc::c_char,
        _: ...
    ) -> libc::c_int;
    fn nc_usec_now() -> int64_t;
    fn nc_msec_now() -> int64_t;
    fn log_loggable(level: libc::c_int) -> libc::c_int;
    fn _log(
        file: *const libc::c_char,
        line: libc::c_int,
        panic: libc::c_int,
        fmt: *const libc::c_char,
        _: ...
    );
    fn _log_hexdump(
        file: *const libc::c_char,
        line: libc::c_int,
        data: *const libc::c_char,
        datalen: libc::c_int,
        fmt: *const libc::c_char,
        _: ...
    );
    fn rbtree_node_init(node: *mut rbnode);
    fn rbtree_init(tree: *mut rbtree, node: *mut rbnode);
    fn rbtree_min(tree: *const rbtree) -> *mut rbnode;
    fn rbtree_insert(tree: *mut rbtree, node: *mut rbnode);
    fn rbtree_delete(tree: *mut rbtree, node: *mut rbnode);
    fn mbuf_get() -> *mut mbuf;
    fn mbuf_put(mbuf: *mut mbuf);
    fn mbuf_length(mbuf: *const mbuf) -> uint32_t;
    fn mbuf_size(mbuf: *const mbuf) -> uint32_t;
    fn mbuf_insert(mhdr: *mut mhdr, mbuf: *mut mbuf);
    fn mbuf_remove(mhdr: *mut mhdr, mbuf: *mut mbuf);
    fn mbuf_copy(mbuf: *mut mbuf, pos: *const uint8_t, n: size_t);
    fn mbuf_split(
        h: *mut mhdr,
        pos: *mut uint8_t,
        cb: mbuf_copy_t,
        cbarg: *mut libc::c_void,
    ) -> *mut mbuf;
    fn memcache_parse_req(r: *mut msg);
    fn memcache_parse_rsp(r: *mut msg);
    fn memcache_failure(r: *const msg) -> bool;
    fn memcache_pre_coalesce(r: *mut msg);
    fn memcache_post_coalesce(r: *mut msg);
    fn memcache_add_auth(
        ctx: *mut context,
        c_conn: *mut conn,
        s_conn: *mut conn,
    ) -> rstatus_t;
    fn memcache_fragment(
        r: *mut msg,
        nserver: uint32_t,
        frag_msgq: *mut msg_tqh,
    ) -> rstatus_t;
    fn redis_parse_req(r: *mut msg);
    fn redis_parse_rsp(r: *mut msg);
    fn redis_failure(r: *const msg) -> bool;
    fn redis_pre_coalesce(r: *mut msg);
    fn redis_post_coalesce(r: *mut msg);
    fn redis_add_auth(
        ctx: *mut context,
        c_conn: *mut conn,
        s_conn: *mut conn,
    ) -> rstatus_t;
    fn redis_fragment(
        r: *mut msg,
        nserver: uint32_t,
        frag_msgq: *mut msg_tqh,
    ) -> rstatus_t;
    fn redis_reply(r: *mut msg) -> rstatus_t;
    fn server_timeout(conn: *mut conn) -> libc::c_int;
    fn server_pool_idx(
        pool: *const server_pool,
        key: *const uint8_t,
        keylen: uint32_t,
    ) -> uint32_t;
    fn conn_sendv(conn: *mut conn, sendv: *const array, nsend: size_t) -> ssize_t;
    fn conn_recv(conn: *mut conn, buf: *mut libc::c_void, size: size_t) -> ssize_t;
}
pub type __builtin_va_list = [__va_list_tag; 1];
#[derive(Copy, Clone)]
#[repr(C)]
pub struct __va_list_tag {
    pub gp_offset: libc::c_uint,
    pub fp_offset: libc::c_uint,
    pub overflow_arg_area: *mut libc::c_void,
    pub reg_save_area: *mut libc::c_void,
}
pub type size_t = libc::c_ulong;
pub type va_list = __builtin_va_list;
pub type __uint8_t = libc::c_uchar;
pub type __uint16_t = libc::c_ushort;
pub type __uint32_t = libc::c_uint;
pub type __int64_t = libc::c_long;
pub type __uint64_t = libc::c_ulong;
pub type __mode_t = libc::c_uint;
pub type __ssize_t = libc::c_long;
pub type __socklen_t = libc::c_uint;
pub type ssize_t = __ssize_t;
pub type mode_t = __mode_t;
pub type int64_t = __int64_t;
pub type pthread_t = libc::c_ulong;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct iovec {
    pub iov_base: *mut libc::c_void,
    pub iov_len: size_t,
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct string {
    pub len: uint32_t,
    pub data: *mut uint8_t,
}
pub type uint8_t = __uint8_t;
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct stats_buffer {
    pub len: size_t,
    pub data: *mut uint8_t,
    pub size: size_t,
}
pub type uint16_t = __uint16_t;
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct rbtree {
    pub root: *mut rbnode,
    pub sentinel: *mut rbnode,
}
pub type mbuf_copy_t = Option::<
    unsafe extern "C" fn(*mut mbuf, *mut libc::c_void) -> (),
>;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct keypos {
    pub start: *mut uint8_t,
    pub end: *mut uint8_t,
}
#[inline]
unsafe extern "C" fn array_set(
    mut a: *mut array,
    mut elem: *mut libc::c_void,
    mut size: size_t,
    mut nalloc: uint32_t,
) {
    (*a).nelem = 0 as libc::c_int as uint32_t;
    (*a).elem = elem;
    (*a).size = size;
    (*a).nalloc = nalloc;
}
#[inline]
unsafe extern "C" fn array_n(mut a: *const array) -> uint32_t {
    return (*a).nelem;
}
#[inline]
unsafe extern "C" fn mbuf_empty(mut mbuf: *const mbuf) -> bool {
    return (*mbuf).pos == (*mbuf).last;
}
#[inline]
unsafe extern "C" fn mbuf_full(mut mbuf: *const mbuf) -> bool {
    return (*mbuf).last == (*mbuf).end;
}
static mut msg_id: uint64_t = 0;
static mut frag_id: uint64_t = 0;
static mut nfree_msgq: uint32_t = 0;
static mut free_msgq: msg_tqh = msg_tqh {
    tqh_first: 0 as *const msg as *mut msg,
    tqh_last: 0 as *const *mut msg as *mut *mut msg,
};
static mut tmo_rbt: rbtree = rbtree {
    root: 0 as *const rbnode as *mut rbnode,
    sentinel: 0 as *const rbnode as *mut rbnode,
};
static mut tmo_rbs: rbnode = rbnode {
    left: 0 as *const rbnode as *mut rbnode,
    right: 0 as *const rbnode as *mut rbnode,
    parent: 0 as *const rbnode as *mut rbnode,
    key: 0,
    data: 0 as *const libc::c_void as *mut libc::c_void,
    color: 0,
};
static mut msg_type_strings: [string; 186] = [string {
    len: 0,
    data: 0 as *mut uint8_t,
}; 186];
unsafe extern "C" fn msg_from_rbe(mut node: *mut rbnode) -> *mut msg {
    let mut msg: *mut msg = 0 as *mut msg;
    let mut offset: libc::c_int = 0;
    offset = 72 as libc::c_ulong as libc::c_int;
    msg = (node as *mut libc::c_char).offset(-(offset as isize)) as *mut msg;
    return msg;
}
pub unsafe extern "C" fn msg_tmo_min() -> *mut msg {
    let mut node: *mut rbnode = 0 as *mut rbnode;
    node = rbtree_min(&mut tmo_rbt);
    if node.is_null() {
        return 0 as *mut msg;
    }
    return msg_from_rbe(node);
}
pub unsafe extern "C" fn msg_tmo_insert(mut msg: *mut msg, mut conn: *mut conn) {
    let mut node: *mut rbnode = 0 as *mut rbnode;
    let mut timeout: libc::c_int = 0;
    timeout = server_timeout(conn);
    if timeout <= 0 as libc::c_int {
        return;
    }
    node = &mut (*msg).tmo_rbe;
    (*node).key = nc_msec_now() + timeout as libc::c_long;
    (*node).data = conn as *mut libc::c_void;
    rbtree_insert(&mut tmo_rbt, node);
}
pub unsafe extern "C" fn msg_tmo_delete(mut msg: *mut msg) {
    let mut node: *mut rbnode = 0 as *mut rbnode;
    node = &mut (*msg).tmo_rbe;
    if ((*node).data).is_null() {
        return;
    }
    rbtree_delete(&mut tmo_rbt, node);
}
unsafe extern "C" fn _msg_get() -> *mut msg {
    let mut msg: *mut msg = 0 as *mut msg;
    if !(free_msgq.tqh_first).is_null() {
        msg = free_msgq.tqh_first;
        nfree_msgq = nfree_msgq.wrapping_sub(1);
        nfree_msgq;
        let mut oldnext: *mut *mut libc::c_void = &mut (*msg).m_tqe.tqe_next
            as *mut *mut msg as *mut libc::c_void as *mut *mut libc::c_void;
        let mut oldprev: *mut *mut libc::c_void = &mut (*msg).m_tqe.tqe_prev
            as *mut *mut *mut msg as *mut libc::c_void as *mut *mut libc::c_void;
        if !((*msg).m_tqe.tqe_next).is_null() {
            (*(*msg).m_tqe.tqe_next).m_tqe.tqe_prev = (*msg).m_tqe.tqe_prev;
        } else {
            free_msgq.tqh_last = (*msg).m_tqe.tqe_prev;
        }
        *(*msg).m_tqe.tqe_prev = (*msg).m_tqe.tqe_next;
        *oldnext = 0 as *mut libc::c_void;
        *oldprev = 0 as *mut libc::c_void;
    } else {
        msg = _nc_alloc(
            ::std::mem::size_of::<msg>() as libc::c_ulong,
            b"nc_message.c\0" as *const u8 as *const libc::c_char,
            207 as libc::c_int,
        ) as *mut msg;
        if msg.is_null() {
            return 0 as *mut msg;
        }
    }
    msg_id = msg_id.wrapping_add(1);
    (*msg).id = msg_id;
    (*msg).peer = 0 as *mut msg;
    (*msg).owner = 0 as *mut conn;
    rbtree_node_init(&mut (*msg).tmo_rbe);
    (*msg).mhdr.stqh_first = 0 as *mut mbuf;
    (*msg).mhdr.stqh_last = &mut (*msg).mhdr.stqh_first;
    (*msg).mlen = 0 as libc::c_int as uint32_t;
    (*msg).start_ts = 0 as libc::c_int as int64_t;
    (*msg).state = 0 as libc::c_int;
    (*msg).pos = 0 as *mut uint8_t;
    (*msg).token = 0 as *mut uint8_t;
    (*msg).parser = None;
    (*msg).add_auth = None;
    (*msg).result = MSG_PARSE_OK;
    (*msg).fragment = None;
    (*msg).reply = None;
    (*msg).pre_coalesce = None;
    (*msg).post_coalesce = None;
    (*msg).type_0 = MSG_UNKNOWN;
    (*msg)
        .keys = array_create(
        1 as libc::c_int as uint32_t,
        ::std::mem::size_of::<keypos>() as libc::c_ulong,
    );
    if ((*msg).keys).is_null() {
        _nc_free(
            msg as *mut libc::c_void,
            b"nc_message.c\0" as *const u8 as *const libc::c_char,
            241 as libc::c_int,
        );
        msg = 0 as *mut msg;
        return 0 as *mut msg;
    }
    (*msg).vlen = 0 as libc::c_int as uint32_t;
    (*msg).end = 0 as *mut uint8_t;
    (*msg).frag_owner = 0 as *mut msg;
    (*msg).frag_seq = 0 as *mut *mut msg;
    (*msg).nfrag = 0 as libc::c_int as uint32_t;
    (*msg).nfrag_done = 0 as libc::c_int as uint32_t;
    (*msg).frag_id = 0 as libc::c_int as uint64_t;
    (*msg).narg_start = 0 as *mut uint8_t;
    (*msg).narg_end = 0 as *mut uint8_t;
    (*msg).narg = 0 as libc::c_int as uint32_t;
    (*msg).rnarg = 0 as libc::c_int as uint32_t;
    (*msg).rlen = 0 as libc::c_int as uint32_t;
    (*msg).integer = 0 as libc::c_int as uint32_t;
    (*msg).err = 0 as libc::c_int;
    (*msg).set_error(0 as libc::c_int as libc::c_uint);
    (*msg).set_ferror(0 as libc::c_int as libc::c_uint);
    (*msg).set_request(0 as libc::c_int as libc::c_uint);
    (*msg).set_quit(0 as libc::c_int as libc::c_uint);
    (*msg).set_noreply(0 as libc::c_int as libc::c_uint);
    (*msg).set_noforward(0 as libc::c_int as libc::c_uint);
    (*msg).set_done(0 as libc::c_int as libc::c_uint);
    (*msg).set_fdone(0 as libc::c_int as libc::c_uint);
    (*msg).set_swallow(0 as libc::c_int as libc::c_uint);
    (*msg).set_redis(0 as libc::c_int as libc::c_uint);
    return msg;
}
pub unsafe extern "C" fn msg_get(
    mut conn: *mut conn,
    mut request: bool,
    mut redis: bool,
) -> *mut msg {
    let mut msg: *mut msg = 0 as *mut msg;
    msg = _msg_get();
    if msg.is_null() {
        return 0 as *mut msg;
    }
    (*msg).owner = conn;
    (*msg)
        .set_request(
            (if request as libc::c_int != 0 {
                1 as libc::c_int
            } else {
                0 as libc::c_int
            }) as libc::c_uint,
        );
    (*msg)
        .set_redis(
            (if redis as libc::c_int != 0 { 1 as libc::c_int } else { 0 as libc::c_int })
                as libc::c_uint,
        );
    if redis {
        if request {
            (*msg)
                .parser = Some(redis_parse_req as unsafe extern "C" fn(*mut msg) -> ());
        } else {
            (*msg)
                .parser = Some(redis_parse_rsp as unsafe extern "C" fn(*mut msg) -> ());
        }
        (*msg)
            .add_auth = Some(
            redis_add_auth
                as unsafe extern "C" fn(*mut context, *mut conn, *mut conn) -> rstatus_t,
        );
        (*msg)
            .fragment = Some(
            redis_fragment
                as unsafe extern "C" fn(*mut msg, uint32_t, *mut msg_tqh) -> rstatus_t,
        );
        (*msg).reply = Some(redis_reply as unsafe extern "C" fn(*mut msg) -> rstatus_t);
        (*msg).failure = Some(redis_failure as unsafe extern "C" fn(*const msg) -> bool);
        (*msg)
            .pre_coalesce = Some(
            redis_pre_coalesce as unsafe extern "C" fn(*mut msg) -> (),
        );
        (*msg)
            .post_coalesce = Some(
            redis_post_coalesce as unsafe extern "C" fn(*mut msg) -> (),
        );
    } else {
        if request {
            (*msg)
                .parser = Some(
                memcache_parse_req as unsafe extern "C" fn(*mut msg) -> (),
            );
        } else {
            (*msg)
                .parser = Some(
                memcache_parse_rsp as unsafe extern "C" fn(*mut msg) -> (),
            );
        }
        (*msg)
            .add_auth = Some(
            memcache_add_auth
                as unsafe extern "C" fn(*mut context, *mut conn, *mut conn) -> rstatus_t,
        );
        (*msg)
            .fragment = Some(
            memcache_fragment
                as unsafe extern "C" fn(*mut msg, uint32_t, *mut msg_tqh) -> rstatus_t,
        );
        (*msg)
            .failure = Some(
            memcache_failure as unsafe extern "C" fn(*const msg) -> bool,
        );
        (*msg)
            .pre_coalesce = Some(
            memcache_pre_coalesce as unsafe extern "C" fn(*mut msg) -> (),
        );
        (*msg)
            .post_coalesce = Some(
            memcache_post_coalesce as unsafe extern "C" fn(*mut msg) -> (),
        );
    }
    if log_loggable(5 as libc::c_int) != 0 as libc::c_int {
        (*msg).start_ts = nc_usec_now();
    }
    return msg;
}
pub unsafe extern "C" fn msg_get_error(mut redis: bool, mut err: err_t) -> *mut msg {
    let mut msg: *mut msg = 0 as *mut msg;
    let mut mbuf: *mut mbuf = 0 as *mut mbuf;
    let mut n: libc::c_int = 0;
    let mut errstr: *const libc::c_char = if err != 0 {
        strerror(err) as *const libc::c_char
    } else {
        b"unknown\0" as *const u8 as *const libc::c_char
    };
    let mut protstr: *const libc::c_char = if redis as libc::c_int != 0 {
        b"-ERR\0" as *const u8 as *const libc::c_char
    } else {
        b"SERVER_ERROR\0" as *const u8 as *const libc::c_char
    };
    msg = _msg_get();
    if msg.is_null() {
        return 0 as *mut msg;
    }
    (*msg).state = 0 as libc::c_int;
    (*msg).type_0 = MSG_RSP_MC_SERVER_ERROR;
    mbuf = mbuf_get();
    if mbuf.is_null() {
        msg_put(msg);
        return 0 as *mut msg;
    }
    mbuf_insert(&mut (*msg).mhdr, mbuf);
    n = _scnprintf(
        (*mbuf).last as *mut libc::c_char,
        mbuf_size(mbuf) as size_t,
        b"%s %s\r\n\0" as *const u8 as *const libc::c_char,
        protstr,
        errstr,
    );
    (*mbuf).last = ((*mbuf).last).offset(n as isize);
    (*msg).mlen = n as uint32_t;
    return msg;
}
unsafe extern "C" fn msg_free(mut msg: *mut msg) {
    _nc_free(
        msg as *mut libc::c_void,
        b"nc_message.c\0" as *const u8 as *const libc::c_char,
        369 as libc::c_int,
    );
    msg = 0 as *mut msg;
}
pub unsafe extern "C" fn msg_put(mut msg: *mut msg) {
    while !((*msg).mhdr.stqh_first).is_null() {
        let mut mbuf: *mut mbuf = (*msg).mhdr.stqh_first;
        mbuf_remove(&mut (*msg).mhdr, mbuf);
        mbuf_put(mbuf);
    }
    if !((*msg).frag_seq).is_null() {
        _nc_free(
            (*msg).frag_seq as *mut libc::c_void,
            b"nc_message.c\0" as *const u8 as *const libc::c_char,
            384 as libc::c_int,
        );
        (*msg).frag_seq = 0 as *mut *mut msg;
        (*msg).frag_seq = 0 as *mut *mut msg;
    }
    if !((*msg).keys).is_null() {
        (*(*msg).keys).nelem = 0 as libc::c_int as uint32_t;
        array_destroy((*msg).keys);
        (*msg).keys = 0 as *mut array;
    }
    nfree_msgq = nfree_msgq.wrapping_add(1);
    nfree_msgq;
    (*msg).m_tqe.tqe_next = free_msgq.tqh_first;
    if !((*msg).m_tqe.tqe_next).is_null() {
        (*free_msgq.tqh_first).m_tqe.tqe_prev = &mut (*msg).m_tqe.tqe_next;
    } else {
        free_msgq.tqh_last = &mut (*msg).m_tqe.tqe_next;
    }
    free_msgq.tqh_first = msg;
    (*msg).m_tqe.tqe_prev = &mut free_msgq.tqh_first;
}
pub unsafe extern "C" fn msg_dump(mut msg: *const msg, mut level: libc::c_int) {
    let mut mbuf: *const mbuf = 0 as *const mbuf;
    if log_loggable(level) == 0 as libc::c_int {
        return;
    }
    _log(
        b"nc_message.c\0" as *const u8 as *const libc::c_char,
        409 as libc::c_int,
        0 as libc::c_int,
        b"msg dump id %lu request %d len %u type %d done %d error %d (err %d)\0"
            as *const u8 as *const libc::c_char,
        (*msg).id,
        (*msg).request() as libc::c_int,
        (*msg).mlen,
        (*msg).type_0 as libc::c_uint,
        (*msg).done() as libc::c_int,
        (*msg).error() as libc::c_int,
        (*msg).err,
    );
    mbuf = (*msg).mhdr.stqh_first;
    while !mbuf.is_null() {
        let mut p: *mut uint8_t = 0 as *mut uint8_t;
        let mut q: *mut uint8_t = 0 as *mut uint8_t;
        let mut len: libc::c_long = 0;
        p = (*mbuf).start;
        q = (*mbuf).last;
        len = q.offset_from(p) as libc::c_long;
        _log(
            b"nc_message.c\0" as *const u8 as *const libc::c_char,
            419 as libc::c_int,
            0 as libc::c_int,
            b"mbuf [%p] with %ld bytes of data\0" as *const u8 as *const libc::c_char,
            p,
            len,
        );
        _log_hexdump(
            b"nc_message.c\0" as *const u8 as *const libc::c_char,
            419 as libc::c_int,
            p as *mut libc::c_char,
            len as libc::c_int,
            b"mbuf [%p] with %ld bytes of data\0" as *const u8 as *const libc::c_char,
            p,
            len,
        );
        mbuf = (*mbuf).next.stqe_next;
    }
}
pub unsafe extern "C" fn msg_init() {
    msg_id = 0 as libc::c_int as uint64_t;
    frag_id = 0 as libc::c_int as uint64_t;
    nfree_msgq = 0 as libc::c_int as uint32_t;
    free_msgq.tqh_first = 0 as *mut msg;
    free_msgq.tqh_last = &mut free_msgq.tqh_first;
    rbtree_init(&mut tmo_rbt, &mut tmo_rbs);
}
pub unsafe extern "C" fn msg_deinit() {
    let mut msg: *mut msg = 0 as *mut msg;
    let mut nmsg: *mut msg = 0 as *mut msg;
    msg = free_msgq.tqh_first;
    while !msg.is_null() {
        nmsg = (*msg).m_tqe.tqe_next;
        msg_free(msg);
        msg = nmsg;
        nfree_msgq = nfree_msgq.wrapping_sub(1);
        nfree_msgq;
    }
}
pub unsafe extern "C" fn msg_type_string(mut type_0: msg_type_t) -> *const string {
    return &*msg_type_strings.as_ptr().offset(type_0 as isize) as *const string;
}
pub unsafe extern "C" fn msg_empty(mut msg: *const msg) -> bool {
    return (*msg).mlen == 0 as libc::c_int as libc::c_uint;
}
pub unsafe extern "C" fn msg_backend_idx(
    mut msg: *const msg,
    mut key: *const uint8_t,
    mut keylen: uint32_t,
) -> uint32_t {
    let mut conn: *mut conn = (*msg).owner;
    let mut pool: *mut server_pool = (*conn).owner as *mut server_pool;
    return server_pool_idx(pool, key, keylen);
}
pub unsafe extern "C" fn msg_ensure_mbuf(
    mut msg: *mut msg,
    mut len: size_t,
) -> *mut mbuf {
    let mut mbuf: *mut mbuf = 0 as *mut mbuf;
    if ((*msg).mhdr.stqh_first).is_null()
        || (mbuf_size(
            (if ((*msg).mhdr.stqh_first).is_null() {
                0 as *mut mbuf
            } else {
                ((*msg).mhdr.stqh_last as *mut libc::c_char)
                    .offset(
                        -(&mut (*(0 as *mut libc::c_void as *mut mbuf)).next
                            as *mut C2RustUnnamed as size_t as isize),
                    ) as *mut libc::c_void as *mut mbuf
            }),
        ) as libc::c_ulong) < len
    {
        mbuf = mbuf_get();
        if mbuf.is_null() {
            return 0 as *mut mbuf;
        }
        mbuf_insert(&mut (*msg).mhdr, mbuf);
    } else {
        mbuf = if ((*msg).mhdr.stqh_first).is_null() {
            0 as *mut mbuf
        } else {
            ((*msg).mhdr.stqh_last as *mut libc::c_char)
                .offset(
                    -(&mut (*(0 as *mut libc::c_void as *mut mbuf)).next
                        as *mut C2RustUnnamed as size_t as isize),
                ) as *mut libc::c_void as *mut mbuf
        };
    }
    return mbuf;
}
pub unsafe extern "C" fn msg_append(
    mut msg: *mut msg,
    mut pos: *const uint8_t,
    mut n: size_t,
) -> rstatus_t {
    let mut mbuf: *mut mbuf = 0 as *mut mbuf;
    mbuf = msg_ensure_mbuf(msg, n);
    if mbuf.is_null() {
        return -(3 as libc::c_int);
    }
    mbuf_copy(mbuf, pos, n);
    (*msg)
        .mlen = ((*msg).mlen as libc::c_uint).wrapping_add(n as uint32_t) as uint32_t
        as uint32_t;
    return 0 as libc::c_int;
}
pub unsafe extern "C" fn msg_prepend(
    mut msg: *mut msg,
    mut pos: *const uint8_t,
    mut n: size_t,
) -> rstatus_t {
    let mut mbuf: *mut mbuf = 0 as *mut mbuf;
    mbuf = mbuf_get();
    if mbuf.is_null() {
        return -(3 as libc::c_int);
    }
    mbuf_copy(mbuf, pos, n);
    (*msg)
        .mlen = ((*msg).mlen as libc::c_uint).wrapping_add(n as uint32_t) as uint32_t
        as uint32_t;
    (*mbuf).next.stqe_next = (*msg).mhdr.stqh_first;
    if ((*mbuf).next.stqe_next).is_null() {
        (*msg).mhdr.stqh_last = &mut (*mbuf).next.stqe_next;
    }
    (*msg).mhdr.stqh_first = mbuf;
    return 0 as libc::c_int;
}
pub unsafe extern "C" fn msg_prepend_format(
    mut msg: *mut msg,
    mut fmt: *const libc::c_char,
    mut args: ...
) -> rstatus_t {
    let mut mbuf: *mut mbuf = 0 as *mut mbuf;
    let mut n: libc::c_int = 0;
    let mut size: uint32_t = 0;
    let mut args_0: ::std::ffi::VaListImpl;
    mbuf = mbuf_get();
    if mbuf.is_null() {
        return -(3 as libc::c_int);
    }
    size = mbuf_size(mbuf);
    args_0 = args.clone();
    n = vsnprintf(
        (*mbuf).last as *mut libc::c_char,
        size as size_t,
        fmt,
        args_0.as_va_list(),
    );
    if n <= 0 as libc::c_int || n >= size as libc::c_int {
        return -(1 as libc::c_int);
    }
    (*mbuf).last = ((*mbuf).last).offset(n as isize);
    (*msg)
        .mlen = ((*msg).mlen as libc::c_uint).wrapping_add(n as uint32_t) as uint32_t
        as uint32_t;
    (*mbuf).next.stqe_next = (*msg).mhdr.stqh_first;
    if ((*mbuf).next.stqe_next).is_null() {
        (*msg).mhdr.stqh_last = &mut (*mbuf).next.stqe_next;
    }
    (*msg).mhdr.stqh_first = mbuf;
    return 0 as libc::c_int;
}
pub unsafe extern "C" fn msg_gen_frag_id() -> uint64_t {
    frag_id = frag_id.wrapping_add(1);
    return frag_id;
}
unsafe extern "C" fn msg_parsed(
    mut ctx: *mut context,
    mut conn: *mut conn,
    mut msg: *mut msg,
) -> rstatus_t {
    let mut nmsg: *mut msg = 0 as *mut msg;
    let mut mbuf: *mut mbuf = 0 as *mut mbuf;
    let mut nbuf: *mut mbuf = 0 as *mut mbuf;
    mbuf = if ((*msg).mhdr.stqh_first).is_null() {
        0 as *mut mbuf
    } else {
        ((*msg).mhdr.stqh_last as *mut libc::c_char)
            .offset(
                -(&mut (*(0 as *mut libc::c_void as *mut mbuf)).next
                    as *mut C2RustUnnamed as size_t as isize),
            ) as *mut libc::c_void as *mut mbuf
    };
    if (*msg).pos == (*mbuf).last {
        ((*conn).recv_done).unwrap()(ctx, conn, msg, 0 as *mut msg);
        return 0 as libc::c_int;
    }
    nbuf = mbuf_split(&mut (*msg).mhdr, (*msg).pos, None, 0 as *mut libc::c_void);
    if nbuf.is_null() {
        return -(3 as libc::c_int);
    }
    nmsg = msg_get((*msg).owner, (*msg).request() != 0, (*conn).redis() != 0);
    if nmsg.is_null() {
        mbuf_put(nbuf);
        return -(3 as libc::c_int);
    }
    mbuf_insert(&mut (*nmsg).mhdr, nbuf);
    (*nmsg).pos = (*nbuf).pos;
    (*nmsg).mlen = mbuf_length(nbuf);
    (*msg)
        .mlen = ((*msg).mlen as libc::c_uint).wrapping_sub((*nmsg).mlen) as uint32_t
        as uint32_t;
    ((*conn).recv_done).unwrap()(ctx, conn, msg, nmsg);
    return 0 as libc::c_int;
}
unsafe extern "C" fn msg_repair(
    mut ctx: *mut context,
    mut conn: *mut conn,
    mut msg: *mut msg,
) -> rstatus_t {
    let mut nbuf: *mut mbuf = 0 as *mut mbuf;
    nbuf = mbuf_split(&mut (*msg).mhdr, (*msg).pos, None, 0 as *mut libc::c_void);
    if nbuf.is_null() {
        return -(3 as libc::c_int);
    }
    mbuf_insert(&mut (*msg).mhdr, nbuf);
    (*msg).pos = (*nbuf).pos;
    return 0 as libc::c_int;
}
unsafe extern "C" fn msg_parse(
    mut ctx: *mut context,
    mut conn: *mut conn,
    mut msg: *mut msg,
) -> rstatus_t {
    let mut status: rstatus_t = 0;
    if msg_empty(msg) {
        ((*conn).recv_done).unwrap()(ctx, conn, msg, 0 as *mut msg);
        return 0 as libc::c_int;
    }
    ((*msg).parser).unwrap()(msg);
    match (*msg).result as libc::c_uint {
        0 => {
            status = msg_parsed(ctx, conn, msg);
        }
        2 => {
            status = msg_repair(ctx, conn, msg);
        }
        3 => {
            status = 0 as libc::c_int;
        }
        _ => {
            status = -(1 as libc::c_int);
            (*conn).err = *__errno_location();
        }
    }
    return if (*conn).err != 0 as libc::c_int { -(1 as libc::c_int) } else { status };
}
unsafe extern "C" fn msg_recv_chain(
    mut ctx: *mut context,
    mut conn: *mut conn,
    mut msg: *mut msg,
) -> rstatus_t {
    let mut status: rstatus_t = 0;
    let mut nmsg: *mut msg = 0 as *mut msg;
    let mut mbuf: *mut mbuf = 0 as *mut mbuf;
    let mut msize: size_t = 0;
    let mut n: ssize_t = 0;
    mbuf = if ((*msg).mhdr.stqh_first).is_null() {
        0 as *mut mbuf
    } else {
        ((*msg).mhdr.stqh_last as *mut libc::c_char)
            .offset(
                -(&mut (*(0 as *mut libc::c_void as *mut mbuf)).next
                    as *mut C2RustUnnamed as size_t as isize),
            ) as *mut libc::c_void as *mut mbuf
    };
    if mbuf.is_null() || mbuf_full(mbuf) as libc::c_int != 0 {
        mbuf = mbuf_get();
        if mbuf.is_null() {
            return -(3 as libc::c_int);
        }
        mbuf_insert(&mut (*msg).mhdr, mbuf);
        (*msg).pos = (*mbuf).pos;
    }
    msize = mbuf_size(mbuf) as size_t;
    n = conn_recv(conn, (*mbuf).last as *mut libc::c_void, msize);
    if n < 0 as libc::c_int as libc::c_long {
        if n == -(2 as libc::c_int) as libc::c_long {
            return 0 as libc::c_int;
        }
        return -(1 as libc::c_int);
    }
    (*mbuf).last = ((*mbuf).last).offset(n as isize);
    (*msg)
        .mlen = ((*msg).mlen as libc::c_uint).wrapping_add(n as uint32_t) as uint32_t
        as uint32_t;
    loop {
        status = msg_parse(ctx, conn, msg);
        if status != 0 as libc::c_int {
            return status;
        }
        nmsg = ((*conn).recv_next).unwrap()(ctx, conn, 0 as libc::c_int != 0);
        if nmsg.is_null() || nmsg == msg {
            break;
        }
        msg = nmsg;
    }
    return 0 as libc::c_int;
}
pub unsafe extern "C" fn msg_recv(
    mut ctx: *mut context,
    mut conn: *mut conn,
) -> rstatus_t {
    let mut status: rstatus_t = 0;
    let mut msg: *mut msg = 0 as *mut msg;
    (*conn).set_recv_ready(1 as libc::c_int as libc::c_uint);
    loop {
        msg = ((*conn).recv_next).unwrap()(ctx, conn, 1 as libc::c_int != 0);
        if msg.is_null() {
            return 0 as libc::c_int;
        }
        status = msg_recv_chain(ctx, conn, msg);
        if status != 0 as libc::c_int {
            return status;
        }
        if !((*conn).recv_ready() != 0) {
            break;
        }
    }
    return 0 as libc::c_int;
}
unsafe extern "C" fn msg_send_chain(
    mut ctx: *mut context,
    mut conn: *mut conn,
    mut msg: *mut msg,
) -> rstatus_t {
    let mut send_msgq: msg_tqh = msg_tqh {
        tqh_first: 0 as *const msg as *mut msg,
        tqh_last: 0 as *const *mut msg as *mut *mut msg,
    };
    let mut nmsg: *mut msg = 0 as *mut msg;
    let mut mbuf: *mut mbuf = 0 as *mut mbuf;
    let mut nbuf: *mut mbuf = 0 as *mut mbuf;
    let mut mlen: size_t = 0;
    let mut ciov: *mut iovec = 0 as *mut iovec;
    let mut iov: [iovec; 128] = [iovec {
        iov_base: 0 as *mut libc::c_void,
        iov_len: 0,
    }; 128];
    let mut sendv: array = array {
        nelem: 0,
        elem: 0 as *mut libc::c_void,
        size: 0,
        nalloc: 0,
    };
    let mut nsend: size_t = 0;
    let mut nsent: size_t = 0;
    let mut limit: size_t = 0;
    let mut n: ssize_t = 0;
    send_msgq.tqh_first = 0 as *mut msg;
    send_msgq.tqh_last = &mut send_msgq.tqh_first;
    array_set(
        &mut sendv,
        iov.as_mut_ptr() as *mut libc::c_void,
        ::std::mem::size_of::<iovec>() as libc::c_ulong,
        128 as libc::c_int as uint32_t,
    );
    nsend = 0 as libc::c_int as size_t;
    limit = 9223372036854775807 as libc::c_long as size_t;
    loop {
        (*msg).m_tqe.tqe_next = 0 as *mut msg;
        (*msg).m_tqe.tqe_prev = send_msgq.tqh_last;
        *send_msgq.tqh_last = msg;
        send_msgq.tqh_last = &mut (*msg).m_tqe.tqe_next;
        mbuf = (*msg).mhdr.stqh_first;
        while !mbuf.is_null() && array_n(&mut sendv) < 128 as libc::c_int as libc::c_uint
            && nsend < limit
        {
            nbuf = (*mbuf).next.stqe_next;
            if !mbuf_empty(mbuf) {
                mlen = mbuf_length(mbuf) as size_t;
                if nsend.wrapping_add(mlen) > limit {
                    mlen = limit.wrapping_sub(nsend);
                }
                ciov = array_push(&mut sendv) as *mut iovec;
                (*ciov).iov_base = (*mbuf).pos as *mut libc::c_void;
                (*ciov).iov_len = mlen;
                nsend = (nsend as libc::c_ulong).wrapping_add(mlen) as size_t as size_t;
            }
            mbuf = nbuf;
        }
        if array_n(&mut sendv) >= 128 as libc::c_int as libc::c_uint || nsend >= limit {
            break;
        }
        msg = ((*conn).send_next).unwrap()(ctx, conn);
        if msg.is_null() {
            break;
        }
    }
    (*conn).smsg = 0 as *mut msg;
    if !(send_msgq.tqh_first).is_null() && nsend != 0 as libc::c_int as libc::c_ulong {
        n = conn_sendv(conn, &mut sendv, nsend);
    } else {
        n = 0 as libc::c_int as ssize_t;
    }
    nsent = if n > 0 as libc::c_int as libc::c_long {
        n as size_t
    } else {
        0 as libc::c_int as libc::c_ulong
    };
    msg = send_msgq.tqh_first;
    while !msg.is_null() {
        nmsg = (*msg).m_tqe.tqe_next;
        let mut oldnext: *mut *mut libc::c_void = &mut (*msg).m_tqe.tqe_next
            as *mut *mut msg as *mut libc::c_void as *mut *mut libc::c_void;
        let mut oldprev: *mut *mut libc::c_void = &mut (*msg).m_tqe.tqe_prev
            as *mut *mut *mut msg as *mut libc::c_void as *mut *mut libc::c_void;
        if !((*msg).m_tqe.tqe_next).is_null() {
            (*(*msg).m_tqe.tqe_next).m_tqe.tqe_prev = (*msg).m_tqe.tqe_prev;
        } else {
            send_msgq.tqh_last = (*msg).m_tqe.tqe_prev;
        }
        *(*msg).m_tqe.tqe_prev = (*msg).m_tqe.tqe_next;
        *oldnext = 0 as *mut libc::c_void;
        *oldprev = 0 as *mut libc::c_void;
        if nsent == 0 as libc::c_int as libc::c_ulong {
            if (*msg).mlen == 0 as libc::c_int as libc::c_uint {
                ((*conn).send_done).unwrap()(ctx, conn, msg);
            }
        } else {
            mbuf = (*msg).mhdr.stqh_first;
            while !mbuf.is_null() {
                nbuf = (*mbuf).next.stqe_next;
                if !mbuf_empty(mbuf) {
                    mlen = mbuf_length(mbuf) as size_t;
                    if nsent < mlen {
                        (*mbuf).pos = ((*mbuf).pos).offset(nsent as isize);
                        nsent = 0 as libc::c_int as size_t;
                        break;
                    } else {
                        (*mbuf).pos = (*mbuf).last;
                        nsent = (nsent as libc::c_ulong).wrapping_sub(mlen) as size_t
                            as size_t;
                    }
                }
                mbuf = nbuf;
            }
            if mbuf.is_null() {
                ((*conn).send_done).unwrap()(ctx, conn, msg);
            }
        }
        msg = nmsg;
    }
    if n >= 0 as libc::c_int as libc::c_long {
        return 0 as libc::c_int;
    }
    return if n == -(2 as libc::c_int) as libc::c_long {
        0 as libc::c_int
    } else {
        -(1 as libc::c_int)
    };
}
pub unsafe extern "C" fn msg_send(
    mut ctx: *mut context,
    mut conn: *mut conn,
) -> rstatus_t {
    let mut status: rstatus_t = 0;
    let mut msg: *mut msg = 0 as *mut msg;
    (*conn).set_send_ready(1 as libc::c_int as libc::c_uint);
    loop {
        msg = ((*conn).send_next).unwrap()(ctx, conn);
        if msg.is_null() {
            return 0 as libc::c_int;
        }
        status = msg_send_chain(ctx, conn, msg);
        if status != 0 as libc::c_int {
            return status;
        }
        if !((*conn).send_ready() != 0) {
            break;
        }
    }
    return 0 as libc::c_int;
}
pub unsafe extern "C" fn msg_set_placeholder_key(mut r: *mut msg) -> bool {
    let mut kpos: *mut keypos = 0 as *mut keypos;
    kpos = array_push((*r).keys) as *mut keypos;
    if kpos.is_null() {
        return 0 as libc::c_int != 0;
    }
    (*kpos).start = b"placeholder\0" as *const u8 as *const libc::c_char as *mut uint8_t;
    (*kpos)
        .end = ((*kpos).start)
        .offset(::std::mem::size_of::<[libc::c_char; 12]>() as libc::c_ulong as isize)
        .offset(-(1 as libc::c_int as isize));
    return 1 as libc::c_int != 0;
}
unsafe extern "C" fn run_static_initializers() {
    msg_type_strings = [
        {
            let mut init = string {
                len: (::std::mem::size_of::<[libc::c_char; 8]>() as libc::c_ulong)
                    .wrapping_sub(1 as libc::c_int as libc::c_ulong) as uint32_t,
                data: b"UNKNOWN\0" as *const u8 as *const libc::c_char as *mut uint8_t,
            };
            init
        },
        {
            let mut init = string {
                len: (::std::mem::size_of::<[libc::c_char; 11]>() as libc::c_ulong)
                    .wrapping_sub(1 as libc::c_int as libc::c_ulong) as uint32_t,
                data: b"REQ_MC_GET\0" as *const u8 as *const libc::c_char as *mut uint8_t,
            };
            init
        },
        {
            let mut init = string {
                len: (::std::mem::size_of::<[libc::c_char; 12]>() as libc::c_ulong)
                    .wrapping_sub(1 as libc::c_int as libc::c_ulong) as uint32_t,
                data: b"REQ_MC_GETS\0" as *const u8 as *const libc::c_char
                    as *mut uint8_t,
            };
            init
        },
        {
            let mut init = string {
                len: (::std::mem::size_of::<[libc::c_char; 14]>() as libc::c_ulong)
                    .wrapping_sub(1 as libc::c_int as libc::c_ulong) as uint32_t,
                data: b"REQ_MC_DELETE\0" as *const u8 as *const libc::c_char
                    as *mut uint8_t,
            };
            init
        },
        {
            let mut init = string {
                len: (::std::mem::size_of::<[libc::c_char; 11]>() as libc::c_ulong)
                    .wrapping_sub(1 as libc::c_int as libc::c_ulong) as uint32_t,
                data: b"REQ_MC_CAS\0" as *const u8 as *const libc::c_char as *mut uint8_t,
            };
            init
        },
        {
            let mut init = string {
                len: (::std::mem::size_of::<[libc::c_char; 11]>() as libc::c_ulong)
                    .wrapping_sub(1 as libc::c_int as libc::c_ulong) as uint32_t,
                data: b"REQ_MC_SET\0" as *const u8 as *const libc::c_char as *mut uint8_t,
            };
            init
        },
        {
            let mut init = string {
                len: (::std::mem::size_of::<[libc::c_char; 11]>() as libc::c_ulong)
                    .wrapping_sub(1 as libc::c_int as libc::c_ulong) as uint32_t,
                data: b"REQ_MC_ADD\0" as *const u8 as *const libc::c_char as *mut uint8_t,
            };
            init
        },
        {
            let mut init = string {
                len: (::std::mem::size_of::<[libc::c_char; 15]>() as libc::c_ulong)
                    .wrapping_sub(1 as libc::c_int as libc::c_ulong) as uint32_t,
                data: b"REQ_MC_REPLACE\0" as *const u8 as *const libc::c_char
                    as *mut uint8_t,
            };
            init
        },
        {
            let mut init = string {
                len: (::std::mem::size_of::<[libc::c_char; 14]>() as libc::c_ulong)
                    .wrapping_sub(1 as libc::c_int as libc::c_ulong) as uint32_t,
                data: b"REQ_MC_APPEND\0" as *const u8 as *const libc::c_char
                    as *mut uint8_t,
            };
            init
        },
        {
            let mut init = string {
                len: (::std::mem::size_of::<[libc::c_char; 15]>() as libc::c_ulong)
                    .wrapping_sub(1 as libc::c_int as libc::c_ulong) as uint32_t,
                data: b"REQ_MC_PREPEND\0" as *const u8 as *const libc::c_char
                    as *mut uint8_t,
            };
            init
        },
        {
            let mut init = string {
                len: (::std::mem::size_of::<[libc::c_char; 12]>() as libc::c_ulong)
                    .wrapping_sub(1 as libc::c_int as libc::c_ulong) as uint32_t,
                data: b"REQ_MC_INCR\0" as *const u8 as *const libc::c_char
                    as *mut uint8_t,
            };
            init
        },
        {
            let mut init = string {
                len: (::std::mem::size_of::<[libc::c_char; 12]>() as libc::c_ulong)
                    .wrapping_sub(1 as libc::c_int as libc::c_ulong) as uint32_t,
                data: b"REQ_MC_DECR\0" as *const u8 as *const libc::c_char
                    as *mut uint8_t,
            };
            init
        },
        {
            let mut init = string {
                len: (::std::mem::size_of::<[libc::c_char; 13]>() as libc::c_ulong)
                    .wrapping_sub(1 as libc::c_int as libc::c_ulong) as uint32_t,
                data: b"REQ_MC_TOUCH\0" as *const u8 as *const libc::c_char
                    as *mut uint8_t,
            };
            init
        },
        {
            let mut init = string {
                len: (::std::mem::size_of::<[libc::c_char; 12]>() as libc::c_ulong)
                    .wrapping_sub(1 as libc::c_int as libc::c_ulong) as uint32_t,
                data: b"REQ_MC_QUIT\0" as *const u8 as *const libc::c_char
                    as *mut uint8_t,
            };
            init
        },
        {
            let mut init = string {
                len: (::std::mem::size_of::<[libc::c_char; 15]>() as libc::c_ulong)
                    .wrapping_sub(1 as libc::c_int as libc::c_ulong) as uint32_t,
                data: b"REQ_MC_VERSION\0" as *const u8 as *const libc::c_char
                    as *mut uint8_t,
            };
            init
        },
        {
            let mut init = string {
                len: (::std::mem::size_of::<[libc::c_char; 11]>() as libc::c_ulong)
                    .wrapping_sub(1 as libc::c_int as libc::c_ulong) as uint32_t,
                data: b"RSP_MC_NUM\0" as *const u8 as *const libc::c_char as *mut uint8_t,
            };
            init
        },
        {
            let mut init = string {
                len: (::std::mem::size_of::<[libc::c_char; 14]>() as libc::c_ulong)
                    .wrapping_sub(1 as libc::c_int as libc::c_ulong) as uint32_t,
                data: b"RSP_MC_STORED\0" as *const u8 as *const libc::c_char
                    as *mut uint8_t,
            };
            init
        },
        {
            let mut init = string {
                len: (::std::mem::size_of::<[libc::c_char; 18]>() as libc::c_ulong)
                    .wrapping_sub(1 as libc::c_int as libc::c_ulong) as uint32_t,
                data: b"RSP_MC_NOT_STORED\0" as *const u8 as *const libc::c_char
                    as *mut uint8_t,
            };
            init
        },
        {
            let mut init = string {
                len: (::std::mem::size_of::<[libc::c_char; 14]>() as libc::c_ulong)
                    .wrapping_sub(1 as libc::c_int as libc::c_ulong) as uint32_t,
                data: b"RSP_MC_EXISTS\0" as *const u8 as *const libc::c_char
                    as *mut uint8_t,
            };
            init
        },
        {
            let mut init = string {
                len: (::std::mem::size_of::<[libc::c_char; 17]>() as libc::c_ulong)
                    .wrapping_sub(1 as libc::c_int as libc::c_ulong) as uint32_t,
                data: b"RSP_MC_NOT_FOUND\0" as *const u8 as *const libc::c_char
                    as *mut uint8_t,
            };
            init
        },
        {
            let mut init = string {
                len: (::std::mem::size_of::<[libc::c_char; 11]>() as libc::c_ulong)
                    .wrapping_sub(1 as libc::c_int as libc::c_ulong) as uint32_t,
                data: b"RSP_MC_END\0" as *const u8 as *const libc::c_char as *mut uint8_t,
            };
            init
        },
        {
            let mut init = string {
                len: (::std::mem::size_of::<[libc::c_char; 13]>() as libc::c_ulong)
                    .wrapping_sub(1 as libc::c_int as libc::c_ulong) as uint32_t,
                data: b"RSP_MC_VALUE\0" as *const u8 as *const libc::c_char
                    as *mut uint8_t,
            };
            init
        },
        {
            let mut init = string {
                len: (::std::mem::size_of::<[libc::c_char; 15]>() as libc::c_ulong)
                    .wrapping_sub(1 as libc::c_int as libc::c_ulong) as uint32_t,
                data: b"RSP_MC_DELETED\0" as *const u8 as *const libc::c_char
                    as *mut uint8_t,
            };
            init
        },
        {
            let mut init = string {
                len: (::std::mem::size_of::<[libc::c_char; 15]>() as libc::c_ulong)
                    .wrapping_sub(1 as libc::c_int as libc::c_ulong) as uint32_t,
                data: b"RSP_MC_TOUCHED\0" as *const u8 as *const libc::c_char
                    as *mut uint8_t,
            };
            init
        },
        {
            let mut init = string {
                len: (::std::mem::size_of::<[libc::c_char; 15]>() as libc::c_ulong)
                    .wrapping_sub(1 as libc::c_int as libc::c_ulong) as uint32_t,
                data: b"RSP_MC_VERSION\0" as *const u8 as *const libc::c_char
                    as *mut uint8_t,
            };
            init
        },
        {
            let mut init = string {
                len: (::std::mem::size_of::<[libc::c_char; 13]>() as libc::c_ulong)
                    .wrapping_sub(1 as libc::c_int as libc::c_ulong) as uint32_t,
                data: b"RSP_MC_ERROR\0" as *const u8 as *const libc::c_char
                    as *mut uint8_t,
            };
            init
        },
        {
            let mut init = string {
                len: (::std::mem::size_of::<[libc::c_char; 20]>() as libc::c_ulong)
                    .wrapping_sub(1 as libc::c_int as libc::c_ulong) as uint32_t,
                data: b"RSP_MC_CLIENT_ERROR\0" as *const u8 as *const libc::c_char
                    as *mut uint8_t,
            };
            init
        },
        {
            let mut init = string {
                len: (::std::mem::size_of::<[libc::c_char; 20]>() as libc::c_ulong)
                    .wrapping_sub(1 as libc::c_int as libc::c_ulong) as uint32_t,
                data: b"RSP_MC_SERVER_ERROR\0" as *const u8 as *const libc::c_char
                    as *mut uint8_t,
            };
            init
        },
        {
            let mut init = string {
                len: (::std::mem::size_of::<[libc::c_char; 15]>() as libc::c_ulong)
                    .wrapping_sub(1 as libc::c_int as libc::c_ulong) as uint32_t,
                data: b"REQ_REDIS_COPY\0" as *const u8 as *const libc::c_char
                    as *mut uint8_t,
            };
            init
        },
        {
            let mut init = string {
                len: (::std::mem::size_of::<[libc::c_char; 14]>() as libc::c_ulong)
                    .wrapping_sub(1 as libc::c_int as libc::c_ulong) as uint32_t,
                data: b"REQ_REDIS_DEL\0" as *const u8 as *const libc::c_char
                    as *mut uint8_t,
            };
            init
        },
        {
            let mut init = string {
                len: (::std::mem::size_of::<[libc::c_char; 17]>() as libc::c_ulong)
                    .wrapping_sub(1 as libc::c_int as libc::c_ulong) as uint32_t,
                data: b"REQ_REDIS_EXISTS\0" as *const u8 as *const libc::c_char
                    as *mut uint8_t,
            };
            init
        },
        {
            let mut init = string {
                len: (::std::mem::size_of::<[libc::c_char; 17]>() as libc::c_ulong)
                    .wrapping_sub(1 as libc::c_int as libc::c_ulong) as uint32_t,
                data: b"REQ_REDIS_EXPIRE\0" as *const u8 as *const libc::c_char
                    as *mut uint8_t,
            };
            init
        },
        {
            let mut init = string {
                len: (::std::mem::size_of::<[libc::c_char; 19]>() as libc::c_ulong)
                    .wrapping_sub(1 as libc::c_int as libc::c_ulong) as uint32_t,
                data: b"REQ_REDIS_EXPIREAT\0" as *const u8 as *const libc::c_char
                    as *mut uint8_t,
            };
            init
        },
        {
            let mut init = string {
                len: (::std::mem::size_of::<[libc::c_char; 15]>() as libc::c_ulong)
                    .wrapping_sub(1 as libc::c_int as libc::c_ulong) as uint32_t,
                data: b"REQ_REDIS_MOVE\0" as *const u8 as *const libc::c_char
                    as *mut uint8_t,
            };
            init
        },
        {
            let mut init = string {
                len: (::std::mem::size_of::<[libc::c_char; 18]>() as libc::c_ulong)
                    .wrapping_sub(1 as libc::c_int as libc::c_ulong) as uint32_t,
                data: b"REQ_REDIS_PEXPIRE\0" as *const u8 as *const libc::c_char
                    as *mut uint8_t,
            };
            init
        },
        {
            let mut init = string {
                len: (::std::mem::size_of::<[libc::c_char; 20]>() as libc::c_ulong)
                    .wrapping_sub(1 as libc::c_int as libc::c_ulong) as uint32_t,
                data: b"REQ_REDIS_PEXPIREAT\0" as *const u8 as *const libc::c_char
                    as *mut uint8_t,
            };
            init
        },
        {
            let mut init = string {
                len: (::std::mem::size_of::<[libc::c_char; 18]>() as libc::c_ulong)
                    .wrapping_sub(1 as libc::c_int as libc::c_ulong) as uint32_t,
                data: b"REQ_REDIS_PERSIST\0" as *const u8 as *const libc::c_char
                    as *mut uint8_t,
            };
            init
        },
        {
            let mut init = string {
                len: (::std::mem::size_of::<[libc::c_char; 15]>() as libc::c_ulong)
                    .wrapping_sub(1 as libc::c_int as libc::c_ulong) as uint32_t,
                data: b"REQ_REDIS_PTTL\0" as *const u8 as *const libc::c_char
                    as *mut uint8_t,
            };
            init
        },
        {
            let mut init = string {
                len: (::std::mem::size_of::<[libc::c_char; 15]>() as libc::c_ulong)
                    .wrapping_sub(1 as libc::c_int as libc::c_ulong) as uint32_t,
                data: b"REQ_REDIS_SORT\0" as *const u8 as *const libc::c_char
                    as *mut uint8_t,
            };
            init
        },
        {
            let mut init = string {
                len: (::std::mem::size_of::<[libc::c_char; 16]>() as libc::c_ulong)
                    .wrapping_sub(1 as libc::c_int as libc::c_ulong) as uint32_t,
                data: b"REQ_REDIS_TOUCH\0" as *const u8 as *const libc::c_char
                    as *mut uint8_t,
            };
            init
        },
        {
            let mut init = string {
                len: (::std::mem::size_of::<[libc::c_char; 14]>() as libc::c_ulong)
                    .wrapping_sub(1 as libc::c_int as libc::c_ulong) as uint32_t,
                data: b"REQ_REDIS_TTL\0" as *const u8 as *const libc::c_char
                    as *mut uint8_t,
            };
            init
        },
        {
            let mut init = string {
                len: (::std::mem::size_of::<[libc::c_char; 15]>() as libc::c_ulong)
                    .wrapping_sub(1 as libc::c_int as libc::c_ulong) as uint32_t,
                data: b"REQ_REDIS_TYPE\0" as *const u8 as *const libc::c_char
                    as *mut uint8_t,
            };
            init
        },
        {
            let mut init = string {
                len: (::std::mem::size_of::<[libc::c_char; 17]>() as libc::c_ulong)
                    .wrapping_sub(1 as libc::c_int as libc::c_ulong) as uint32_t,
                data: b"REQ_REDIS_UNLINK\0" as *const u8 as *const libc::c_char
                    as *mut uint8_t,
            };
            init
        },
        {
            let mut init = string {
                len: (::std::mem::size_of::<[libc::c_char; 17]>() as libc::c_ulong)
                    .wrapping_sub(1 as libc::c_int as libc::c_ulong) as uint32_t,
                data: b"REQ_REDIS_APPEND\0" as *const u8 as *const libc::c_char
                    as *mut uint8_t,
            };
            init
        },
        {
            let mut init = string {
                len: (::std::mem::size_of::<[libc::c_char; 19]>() as libc::c_ulong)
                    .wrapping_sub(1 as libc::c_int as libc::c_ulong) as uint32_t,
                data: b"REQ_REDIS_BITCOUNT\0" as *const u8 as *const libc::c_char
                    as *mut uint8_t,
            };
            init
        },
        {
            let mut init = string {
                len: (::std::mem::size_of::<[libc::c_char; 19]>() as libc::c_ulong)
                    .wrapping_sub(1 as libc::c_int as libc::c_ulong) as uint32_t,
                data: b"REQ_REDIS_BITFIELD\0" as *const u8 as *const libc::c_char
                    as *mut uint8_t,
            };
            init
        },
        {
            let mut init = string {
                len: (::std::mem::size_of::<[libc::c_char; 17]>() as libc::c_ulong)
                    .wrapping_sub(1 as libc::c_int as libc::c_ulong) as uint32_t,
                data: b"REQ_REDIS_BITPOS\0" as *const u8 as *const libc::c_char
                    as *mut uint8_t,
            };
            init
        },
        {
            let mut init = string {
                len: (::std::mem::size_of::<[libc::c_char; 15]>() as libc::c_ulong)
                    .wrapping_sub(1 as libc::c_int as libc::c_ulong) as uint32_t,
                data: b"REQ_REDIS_DECR\0" as *const u8 as *const libc::c_char
                    as *mut uint8_t,
            };
            init
        },
        {
            let mut init = string {
                len: (::std::mem::size_of::<[libc::c_char; 17]>() as libc::c_ulong)
                    .wrapping_sub(1 as libc::c_int as libc::c_ulong) as uint32_t,
                data: b"REQ_REDIS_DECRBY\0" as *const u8 as *const libc::c_char
                    as *mut uint8_t,
            };
            init
        },
        {
            let mut init = string {
                len: (::std::mem::size_of::<[libc::c_char; 15]>() as libc::c_ulong)
                    .wrapping_sub(1 as libc::c_int as libc::c_ulong) as uint32_t,
                data: b"REQ_REDIS_DUMP\0" as *const u8 as *const libc::c_char
                    as *mut uint8_t,
            };
            init
        },
        {
            let mut init = string {
                len: (::std::mem::size_of::<[libc::c_char; 14]>() as libc::c_ulong)
                    .wrapping_sub(1 as libc::c_int as libc::c_ulong) as uint32_t,
                data: b"REQ_REDIS_GET\0" as *const u8 as *const libc::c_char
                    as *mut uint8_t,
            };
            init
        },
        {
            let mut init = string {
                len: (::std::mem::size_of::<[libc::c_char; 17]>() as libc::c_ulong)
                    .wrapping_sub(1 as libc::c_int as libc::c_ulong) as uint32_t,
                data: b"REQ_REDIS_GETBIT\0" as *const u8 as *const libc::c_char
                    as *mut uint8_t,
            };
            init
        },
        {
            let mut init = string {
                len: (::std::mem::size_of::<[libc::c_char; 17]>() as libc::c_ulong)
                    .wrapping_sub(1 as libc::c_int as libc::c_ulong) as uint32_t,
                data: b"REQ_REDIS_GETDEL\0" as *const u8 as *const libc::c_char
                    as *mut uint8_t,
            };
            init
        },
        {
            let mut init = string {
                len: (::std::mem::size_of::<[libc::c_char; 16]>() as libc::c_ulong)
                    .wrapping_sub(1 as libc::c_int as libc::c_ulong) as uint32_t,
                data: b"REQ_REDIS_GETEX\0" as *const u8 as *const libc::c_char
                    as *mut uint8_t,
            };
            init
        },
        {
            let mut init = string {
                len: (::std::mem::size_of::<[libc::c_char; 19]>() as libc::c_ulong)
                    .wrapping_sub(1 as libc::c_int as libc::c_ulong) as uint32_t,
                data: b"REQ_REDIS_GETRANGE\0" as *const u8 as *const libc::c_char
                    as *mut uint8_t,
            };
            init
        },
        {
            let mut init = string {
                len: (::std::mem::size_of::<[libc::c_char; 17]>() as libc::c_ulong)
                    .wrapping_sub(1 as libc::c_int as libc::c_ulong) as uint32_t,
                data: b"REQ_REDIS_GETSET\0" as *const u8 as *const libc::c_char
                    as *mut uint8_t,
            };
            init
        },
        {
            let mut init = string {
                len: (::std::mem::size_of::<[libc::c_char; 15]>() as libc::c_ulong)
                    .wrapping_sub(1 as libc::c_int as libc::c_ulong) as uint32_t,
                data: b"REQ_REDIS_INCR\0" as *const u8 as *const libc::c_char
                    as *mut uint8_t,
            };
            init
        },
        {
            let mut init = string {
                len: (::std::mem::size_of::<[libc::c_char; 17]>() as libc::c_ulong)
                    .wrapping_sub(1 as libc::c_int as libc::c_ulong) as uint32_t,
                data: b"REQ_REDIS_INCRBY\0" as *const u8 as *const libc::c_char
                    as *mut uint8_t,
            };
            init
        },
        {
            let mut init = string {
                len: (::std::mem::size_of::<[libc::c_char; 22]>() as libc::c_ulong)
                    .wrapping_sub(1 as libc::c_int as libc::c_ulong) as uint32_t,
                data: b"REQ_REDIS_INCRBYFLOAT\0" as *const u8 as *const libc::c_char
                    as *mut uint8_t,
            };
            init
        },
        {
            let mut init = string {
                len: (::std::mem::size_of::<[libc::c_char; 15]>() as libc::c_ulong)
                    .wrapping_sub(1 as libc::c_int as libc::c_ulong) as uint32_t,
                data: b"REQ_REDIS_MGET\0" as *const u8 as *const libc::c_char
                    as *mut uint8_t,
            };
            init
        },
        {
            let mut init = string {
                len: (::std::mem::size_of::<[libc::c_char; 15]>() as libc::c_ulong)
                    .wrapping_sub(1 as libc::c_int as libc::c_ulong) as uint32_t,
                data: b"REQ_REDIS_MSET\0" as *const u8 as *const libc::c_char
                    as *mut uint8_t,
            };
            init
        },
        {
            let mut init = string {
                len: (::std::mem::size_of::<[libc::c_char; 17]>() as libc::c_ulong)
                    .wrapping_sub(1 as libc::c_int as libc::c_ulong) as uint32_t,
                data: b"REQ_REDIS_PSETEX\0" as *const u8 as *const libc::c_char
                    as *mut uint8_t,
            };
            init
        },
        {
            let mut init = string {
                len: (::std::mem::size_of::<[libc::c_char; 18]>() as libc::c_ulong)
                    .wrapping_sub(1 as libc::c_int as libc::c_ulong) as uint32_t,
                data: b"REQ_REDIS_RESTORE\0" as *const u8 as *const libc::c_char
                    as *mut uint8_t,
            };
            init
        },
        {
            let mut init = string {
                len: (::std::mem::size_of::<[libc::c_char; 14]>() as libc::c_ulong)
                    .wrapping_sub(1 as libc::c_int as libc::c_ulong) as uint32_t,
                data: b"REQ_REDIS_SET\0" as *const u8 as *const libc::c_char
                    as *mut uint8_t,
            };
            init
        },
        {
            let mut init = string {
                len: (::std::mem::size_of::<[libc::c_char; 17]>() as libc::c_ulong)
                    .wrapping_sub(1 as libc::c_int as libc::c_ulong) as uint32_t,
                data: b"REQ_REDIS_SETBIT\0" as *const u8 as *const libc::c_char
                    as *mut uint8_t,
            };
            init
        },
        {
            let mut init = string {
                len: (::std::mem::size_of::<[libc::c_char; 16]>() as libc::c_ulong)
                    .wrapping_sub(1 as libc::c_int as libc::c_ulong) as uint32_t,
                data: b"REQ_REDIS_SETEX\0" as *const u8 as *const libc::c_char
                    as *mut uint8_t,
            };
            init
        },
        {
            let mut init = string {
                len: (::std::mem::size_of::<[libc::c_char; 16]>() as libc::c_ulong)
                    .wrapping_sub(1 as libc::c_int as libc::c_ulong) as uint32_t,
                data: b"REQ_REDIS_SETNX\0" as *const u8 as *const libc::c_char
                    as *mut uint8_t,
            };
            init
        },
        {
            let mut init = string {
                len: (::std::mem::size_of::<[libc::c_char; 19]>() as libc::c_ulong)
                    .wrapping_sub(1 as libc::c_int as libc::c_ulong) as uint32_t,
                data: b"REQ_REDIS_SETRANGE\0" as *const u8 as *const libc::c_char
                    as *mut uint8_t,
            };
            init
        },
        {
            let mut init = string {
                len: (::std::mem::size_of::<[libc::c_char; 17]>() as libc::c_ulong)
                    .wrapping_sub(1 as libc::c_int as libc::c_ulong) as uint32_t,
                data: b"REQ_REDIS_STRLEN\0" as *const u8 as *const libc::c_char
                    as *mut uint8_t,
            };
            init
        },
        {
            let mut init = string {
                len: (::std::mem::size_of::<[libc::c_char; 15]>() as libc::c_ulong)
                    .wrapping_sub(1 as libc::c_int as libc::c_ulong) as uint32_t,
                data: b"REQ_REDIS_HDEL\0" as *const u8 as *const libc::c_char
                    as *mut uint8_t,
            };
            init
        },
        {
            let mut init = string {
                len: (::std::mem::size_of::<[libc::c_char; 18]>() as libc::c_ulong)
                    .wrapping_sub(1 as libc::c_int as libc::c_ulong) as uint32_t,
                data: b"REQ_REDIS_HEXISTS\0" as *const u8 as *const libc::c_char
                    as *mut uint8_t,
            };
            init
        },
        {
            let mut init = string {
                len: (::std::mem::size_of::<[libc::c_char; 15]>() as libc::c_ulong)
                    .wrapping_sub(1 as libc::c_int as libc::c_ulong) as uint32_t,
                data: b"REQ_REDIS_HGET\0" as *const u8 as *const libc::c_char
                    as *mut uint8_t,
            };
            init
        },
        {
            let mut init = string {
                len: (::std::mem::size_of::<[libc::c_char; 18]>() as libc::c_ulong)
                    .wrapping_sub(1 as libc::c_int as libc::c_ulong) as uint32_t,
                data: b"REQ_REDIS_HGETALL\0" as *const u8 as *const libc::c_char
                    as *mut uint8_t,
            };
            init
        },
        {
            let mut init = string {
                len: (::std::mem::size_of::<[libc::c_char; 18]>() as libc::c_ulong)
                    .wrapping_sub(1 as libc::c_int as libc::c_ulong) as uint32_t,
                data: b"REQ_REDIS_HINCRBY\0" as *const u8 as *const libc::c_char
                    as *mut uint8_t,
            };
            init
        },
        {
            let mut init = string {
                len: (::std::mem::size_of::<[libc::c_char; 23]>() as libc::c_ulong)
                    .wrapping_sub(1 as libc::c_int as libc::c_ulong) as uint32_t,
                data: b"REQ_REDIS_HINCRBYFLOAT\0" as *const u8 as *const libc::c_char
                    as *mut uint8_t,
            };
            init
        },
        {
            let mut init = string {
                len: (::std::mem::size_of::<[libc::c_char; 16]>() as libc::c_ulong)
                    .wrapping_sub(1 as libc::c_int as libc::c_ulong) as uint32_t,
                data: b"REQ_REDIS_HKEYS\0" as *const u8 as *const libc::c_char
                    as *mut uint8_t,
            };
            init
        },
        {
            let mut init = string {
                len: (::std::mem::size_of::<[libc::c_char; 15]>() as libc::c_ulong)
                    .wrapping_sub(1 as libc::c_int as libc::c_ulong) as uint32_t,
                data: b"REQ_REDIS_HLEN\0" as *const u8 as *const libc::c_char
                    as *mut uint8_t,
            };
            init
        },
        {
            let mut init = string {
                len: (::std::mem::size_of::<[libc::c_char; 16]>() as libc::c_ulong)
                    .wrapping_sub(1 as libc::c_int as libc::c_ulong) as uint32_t,
                data: b"REQ_REDIS_HMGET\0" as *const u8 as *const libc::c_char
                    as *mut uint8_t,
            };
            init
        },
        {
            let mut init = string {
                len: (::std::mem::size_of::<[libc::c_char; 16]>() as libc::c_ulong)
                    .wrapping_sub(1 as libc::c_int as libc::c_ulong) as uint32_t,
                data: b"REQ_REDIS_HMSET\0" as *const u8 as *const libc::c_char
                    as *mut uint8_t,
            };
            init
        },
        {
            let mut init = string {
                len: (::std::mem::size_of::<[libc::c_char; 21]>() as libc::c_ulong)
                    .wrapping_sub(1 as libc::c_int as libc::c_ulong) as uint32_t,
                data: b"REQ_REDIS_HRANDFIELD\0" as *const u8 as *const libc::c_char
                    as *mut uint8_t,
            };
            init
        },
        {
            let mut init = string {
                len: (::std::mem::size_of::<[libc::c_char; 15]>() as libc::c_ulong)
                    .wrapping_sub(1 as libc::c_int as libc::c_ulong) as uint32_t,
                data: b"REQ_REDIS_HSET\0" as *const u8 as *const libc::c_char
                    as *mut uint8_t,
            };
            init
        },
        {
            let mut init = string {
                len: (::std::mem::size_of::<[libc::c_char; 17]>() as libc::c_ulong)
                    .wrapping_sub(1 as libc::c_int as libc::c_ulong) as uint32_t,
                data: b"REQ_REDIS_HSETNX\0" as *const u8 as *const libc::c_char
                    as *mut uint8_t,
            };
            init
        },
        {
            let mut init = string {
                len: (::std::mem::size_of::<[libc::c_char; 16]>() as libc::c_ulong)
                    .wrapping_sub(1 as libc::c_int as libc::c_ulong) as uint32_t,
                data: b"REQ_REDIS_HSCAN\0" as *const u8 as *const libc::c_char
                    as *mut uint8_t,
            };
            init
        },
        {
            let mut init = string {
                len: (::std::mem::size_of::<[libc::c_char; 18]>() as libc::c_ulong)
                    .wrapping_sub(1 as libc::c_int as libc::c_ulong) as uint32_t,
                data: b"REQ_REDIS_HSTRLEN\0" as *const u8 as *const libc::c_char
                    as *mut uint8_t,
            };
            init
        },
        {
            let mut init = string {
                len: (::std::mem::size_of::<[libc::c_char; 16]>() as libc::c_ulong)
                    .wrapping_sub(1 as libc::c_int as libc::c_ulong) as uint32_t,
                data: b"REQ_REDIS_HVALS\0" as *const u8 as *const libc::c_char
                    as *mut uint8_t,
            };
            init
        },
        {
            let mut init = string {
                len: (::std::mem::size_of::<[libc::c_char; 17]>() as libc::c_ulong)
                    .wrapping_sub(1 as libc::c_int as libc::c_ulong) as uint32_t,
                data: b"REQ_REDIS_LINDEX\0" as *const u8 as *const libc::c_char
                    as *mut uint8_t,
            };
            init
        },
        {
            let mut init = string {
                len: (::std::mem::size_of::<[libc::c_char; 18]>() as libc::c_ulong)
                    .wrapping_sub(1 as libc::c_int as libc::c_ulong) as uint32_t,
                data: b"REQ_REDIS_LINSERT\0" as *const u8 as *const libc::c_char
                    as *mut uint8_t,
            };
            init
        },
        {
            let mut init = string {
                len: (::std::mem::size_of::<[libc::c_char; 15]>() as libc::c_ulong)
                    .wrapping_sub(1 as libc::c_int as libc::c_ulong) as uint32_t,
                data: b"REQ_REDIS_LLEN\0" as *const u8 as *const libc::c_char
                    as *mut uint8_t,
            };
            init
        },
        {
            let mut init = string {
                len: (::std::mem::size_of::<[libc::c_char; 16]>() as libc::c_ulong)
                    .wrapping_sub(1 as libc::c_int as libc::c_ulong) as uint32_t,
                data: b"REQ_REDIS_LMOVE\0" as *const u8 as *const libc::c_char
                    as *mut uint8_t,
            };
            init
        },
        {
            let mut init = string {
                len: (::std::mem::size_of::<[libc::c_char; 15]>() as libc::c_ulong)
                    .wrapping_sub(1 as libc::c_int as libc::c_ulong) as uint32_t,
                data: b"REQ_REDIS_LPOP\0" as *const u8 as *const libc::c_char
                    as *mut uint8_t,
            };
            init
        },
        {
            let mut init = string {
                len: (::std::mem::size_of::<[libc::c_char; 15]>() as libc::c_ulong)
                    .wrapping_sub(1 as libc::c_int as libc::c_ulong) as uint32_t,
                data: b"REQ_REDIS_LPOS\0" as *const u8 as *const libc::c_char
                    as *mut uint8_t,
            };
            init
        },
        {
            let mut init = string {
                len: (::std::mem::size_of::<[libc::c_char; 16]>() as libc::c_ulong)
                    .wrapping_sub(1 as libc::c_int as libc::c_ulong) as uint32_t,
                data: b"REQ_REDIS_LPUSH\0" as *const u8 as *const libc::c_char
                    as *mut uint8_t,
            };
            init
        },
        {
            let mut init = string {
                len: (::std::mem::size_of::<[libc::c_char; 17]>() as libc::c_ulong)
                    .wrapping_sub(1 as libc::c_int as libc::c_ulong) as uint32_t,
                data: b"REQ_REDIS_LPUSHX\0" as *const u8 as *const libc::c_char
                    as *mut uint8_t,
            };
            init
        },
        {
            let mut init = string {
                len: (::std::mem::size_of::<[libc::c_char; 17]>() as libc::c_ulong)
                    .wrapping_sub(1 as libc::c_int as libc::c_ulong) as uint32_t,
                data: b"REQ_REDIS_LRANGE\0" as *const u8 as *const libc::c_char
                    as *mut uint8_t,
            };
            init
        },
        {
            let mut init = string {
                len: (::std::mem::size_of::<[libc::c_char; 15]>() as libc::c_ulong)
                    .wrapping_sub(1 as libc::c_int as libc::c_ulong) as uint32_t,
                data: b"REQ_REDIS_LREM\0" as *const u8 as *const libc::c_char
                    as *mut uint8_t,
            };
            init
        },
        {
            let mut init = string {
                len: (::std::mem::size_of::<[libc::c_char; 15]>() as libc::c_ulong)
                    .wrapping_sub(1 as libc::c_int as libc::c_ulong) as uint32_t,
                data: b"REQ_REDIS_LSET\0" as *const u8 as *const libc::c_char
                    as *mut uint8_t,
            };
            init
        },
        {
            let mut init = string {
                len: (::std::mem::size_of::<[libc::c_char; 16]>() as libc::c_ulong)
                    .wrapping_sub(1 as libc::c_int as libc::c_ulong) as uint32_t,
                data: b"REQ_REDIS_LTRIM\0" as *const u8 as *const libc::c_char
                    as *mut uint8_t,
            };
            init
        },
        {
            let mut init = string {
                len: (::std::mem::size_of::<[libc::c_char; 16]>() as libc::c_ulong)
                    .wrapping_sub(1 as libc::c_int as libc::c_ulong) as uint32_t,
                data: b"REQ_REDIS_PFADD\0" as *const u8 as *const libc::c_char
                    as *mut uint8_t,
            };
            init
        },
        {
            let mut init = string {
                len: (::std::mem::size_of::<[libc::c_char; 18]>() as libc::c_ulong)
                    .wrapping_sub(1 as libc::c_int as libc::c_ulong) as uint32_t,
                data: b"REQ_REDIS_PFCOUNT\0" as *const u8 as *const libc::c_char
                    as *mut uint8_t,
            };
            init
        },
        {
            let mut init = string {
                len: (::std::mem::size_of::<[libc::c_char; 18]>() as libc::c_ulong)
                    .wrapping_sub(1 as libc::c_int as libc::c_ulong) as uint32_t,
                data: b"REQ_REDIS_PFMERGE\0" as *const u8 as *const libc::c_char
                    as *mut uint8_t,
            };
            init
        },
        {
            let mut init = string {
                len: (::std::mem::size_of::<[libc::c_char; 15]>() as libc::c_ulong)
                    .wrapping_sub(1 as libc::c_int as libc::c_ulong) as uint32_t,
                data: b"REQ_REDIS_RPOP\0" as *const u8 as *const libc::c_char
                    as *mut uint8_t,
            };
            init
        },
        {
            let mut init = string {
                len: (::std::mem::size_of::<[libc::c_char; 20]>() as libc::c_ulong)
                    .wrapping_sub(1 as libc::c_int as libc::c_ulong) as uint32_t,
                data: b"REQ_REDIS_RPOPLPUSH\0" as *const u8 as *const libc::c_char
                    as *mut uint8_t,
            };
            init
        },
        {
            let mut init = string {
                len: (::std::mem::size_of::<[libc::c_char; 16]>() as libc::c_ulong)
                    .wrapping_sub(1 as libc::c_int as libc::c_ulong) as uint32_t,
                data: b"REQ_REDIS_RPUSH\0" as *const u8 as *const libc::c_char
                    as *mut uint8_t,
            };
            init
        },
        {
            let mut init = string {
                len: (::std::mem::size_of::<[libc::c_char; 17]>() as libc::c_ulong)
                    .wrapping_sub(1 as libc::c_int as libc::c_ulong) as uint32_t,
                data: b"REQ_REDIS_RPUSHX\0" as *const u8 as *const libc::c_char
                    as *mut uint8_t,
            };
            init
        },
        {
            let mut init = string {
                len: (::std::mem::size_of::<[libc::c_char; 15]>() as libc::c_ulong)
                    .wrapping_sub(1 as libc::c_int as libc::c_ulong) as uint32_t,
                data: b"REQ_REDIS_SADD\0" as *const u8 as *const libc::c_char
                    as *mut uint8_t,
            };
            init
        },
        {
            let mut init = string {
                len: (::std::mem::size_of::<[libc::c_char; 16]>() as libc::c_ulong)
                    .wrapping_sub(1 as libc::c_int as libc::c_ulong) as uint32_t,
                data: b"REQ_REDIS_SCARD\0" as *const u8 as *const libc::c_char
                    as *mut uint8_t,
            };
            init
        },
        {
            let mut init = string {
                len: (::std::mem::size_of::<[libc::c_char; 16]>() as libc::c_ulong)
                    .wrapping_sub(1 as libc::c_int as libc::c_ulong) as uint32_t,
                data: b"REQ_REDIS_SDIFF\0" as *const u8 as *const libc::c_char
                    as *mut uint8_t,
            };
            init
        },
        {
            let mut init = string {
                len: (::std::mem::size_of::<[libc::c_char; 21]>() as libc::c_ulong)
                    .wrapping_sub(1 as libc::c_int as libc::c_ulong) as uint32_t,
                data: b"REQ_REDIS_SDIFFSTORE\0" as *const u8 as *const libc::c_char
                    as *mut uint8_t,
            };
            init
        },
        {
            let mut init = string {
                len: (::std::mem::size_of::<[libc::c_char; 17]>() as libc::c_ulong)
                    .wrapping_sub(1 as libc::c_int as libc::c_ulong) as uint32_t,
                data: b"REQ_REDIS_SINTER\0" as *const u8 as *const libc::c_char
                    as *mut uint8_t,
            };
            init
        },
        {
            let mut init = string {
                len: (::std::mem::size_of::<[libc::c_char; 22]>() as libc::c_ulong)
                    .wrapping_sub(1 as libc::c_int as libc::c_ulong) as uint32_t,
                data: b"REQ_REDIS_SINTERSTORE\0" as *const u8 as *const libc::c_char
                    as *mut uint8_t,
            };
            init
        },
        {
            let mut init = string {
                len: (::std::mem::size_of::<[libc::c_char; 20]>() as libc::c_ulong)
                    .wrapping_sub(1 as libc::c_int as libc::c_ulong) as uint32_t,
                data: b"REQ_REDIS_SISMEMBER\0" as *const u8 as *const libc::c_char
                    as *mut uint8_t,
            };
            init
        },
        {
            let mut init = string {
                len: (::std::mem::size_of::<[libc::c_char; 21]>() as libc::c_ulong)
                    .wrapping_sub(1 as libc::c_int as libc::c_ulong) as uint32_t,
                data: b"REQ_REDIS_SMISMEMBER\0" as *const u8 as *const libc::c_char
                    as *mut uint8_t,
            };
            init
        },
        {
            let mut init = string {
                len: (::std::mem::size_of::<[libc::c_char; 19]>() as libc::c_ulong)
                    .wrapping_sub(1 as libc::c_int as libc::c_ulong) as uint32_t,
                data: b"REQ_REDIS_SMEMBERS\0" as *const u8 as *const libc::c_char
                    as *mut uint8_t,
            };
            init
        },
        {
            let mut init = string {
                len: (::std::mem::size_of::<[libc::c_char; 16]>() as libc::c_ulong)
                    .wrapping_sub(1 as libc::c_int as libc::c_ulong) as uint32_t,
                data: b"REQ_REDIS_SMOVE\0" as *const u8 as *const libc::c_char
                    as *mut uint8_t,
            };
            init
        },
        {
            let mut init = string {
                len: (::std::mem::size_of::<[libc::c_char; 15]>() as libc::c_ulong)
                    .wrapping_sub(1 as libc::c_int as libc::c_ulong) as uint32_t,
                data: b"REQ_REDIS_SPOP\0" as *const u8 as *const libc::c_char
                    as *mut uint8_t,
            };
            init
        },
        {
            let mut init = string {
                len: (::std::mem::size_of::<[libc::c_char; 22]>() as libc::c_ulong)
                    .wrapping_sub(1 as libc::c_int as libc::c_ulong) as uint32_t,
                data: b"REQ_REDIS_SRANDMEMBER\0" as *const u8 as *const libc::c_char
                    as *mut uint8_t,
            };
            init
        },
        {
            let mut init = string {
                len: (::std::mem::size_of::<[libc::c_char; 15]>() as libc::c_ulong)
                    .wrapping_sub(1 as libc::c_int as libc::c_ulong) as uint32_t,
                data: b"REQ_REDIS_SREM\0" as *const u8 as *const libc::c_char
                    as *mut uint8_t,
            };
            init
        },
        {
            let mut init = string {
                len: (::std::mem::size_of::<[libc::c_char; 17]>() as libc::c_ulong)
                    .wrapping_sub(1 as libc::c_int as libc::c_ulong) as uint32_t,
                data: b"REQ_REDIS_SUNION\0" as *const u8 as *const libc::c_char
                    as *mut uint8_t,
            };
            init
        },
        {
            let mut init = string {
                len: (::std::mem::size_of::<[libc::c_char; 22]>() as libc::c_ulong)
                    .wrapping_sub(1 as libc::c_int as libc::c_ulong) as uint32_t,
                data: b"REQ_REDIS_SUNIONSTORE\0" as *const u8 as *const libc::c_char
                    as *mut uint8_t,
            };
            init
        },
        {
            let mut init = string {
                len: (::std::mem::size_of::<[libc::c_char; 16]>() as libc::c_ulong)
                    .wrapping_sub(1 as libc::c_int as libc::c_ulong) as uint32_t,
                data: b"REQ_REDIS_SSCAN\0" as *const u8 as *const libc::c_char
                    as *mut uint8_t,
            };
            init
        },
        {
            let mut init = string {
                len: (::std::mem::size_of::<[libc::c_char; 15]>() as libc::c_ulong)
                    .wrapping_sub(1 as libc::c_int as libc::c_ulong) as uint32_t,
                data: b"REQ_REDIS_ZADD\0" as *const u8 as *const libc::c_char
                    as *mut uint8_t,
            };
            init
        },
        {
            let mut init = string {
                len: (::std::mem::size_of::<[libc::c_char; 16]>() as libc::c_ulong)
                    .wrapping_sub(1 as libc::c_int as libc::c_ulong) as uint32_t,
                data: b"REQ_REDIS_ZCARD\0" as *const u8 as *const libc::c_char
                    as *mut uint8_t,
            };
            init
        },
        {
            let mut init = string {
                len: (::std::mem::size_of::<[libc::c_char; 17]>() as libc::c_ulong)
                    .wrapping_sub(1 as libc::c_int as libc::c_ulong) as uint32_t,
                data: b"REQ_REDIS_ZCOUNT\0" as *const u8 as *const libc::c_char
                    as *mut uint8_t,
            };
            init
        },
        {
            let mut init = string {
                len: (::std::mem::size_of::<[libc::c_char; 16]>() as libc::c_ulong)
                    .wrapping_sub(1 as libc::c_int as libc::c_ulong) as uint32_t,
                data: b"REQ_REDIS_ZDIFF\0" as *const u8 as *const libc::c_char
                    as *mut uint8_t,
            };
            init
        },
        {
            let mut init = string {
                len: (::std::mem::size_of::<[libc::c_char; 21]>() as libc::c_ulong)
                    .wrapping_sub(1 as libc::c_int as libc::c_ulong) as uint32_t,
                data: b"REQ_REDIS_ZDIFFSTORE\0" as *const u8 as *const libc::c_char
                    as *mut uint8_t,
            };
            init
        },
        {
            let mut init = string {
                len: (::std::mem::size_of::<[libc::c_char; 18]>() as libc::c_ulong)
                    .wrapping_sub(1 as libc::c_int as libc::c_ulong) as uint32_t,
                data: b"REQ_REDIS_ZINCRBY\0" as *const u8 as *const libc::c_char
                    as *mut uint8_t,
            };
            init
        },
        {
            let mut init = string {
                len: (::std::mem::size_of::<[libc::c_char; 17]>() as libc::c_ulong)
                    .wrapping_sub(1 as libc::c_int as libc::c_ulong) as uint32_t,
                data: b"REQ_REDIS_ZINTER\0" as *const u8 as *const libc::c_char
                    as *mut uint8_t,
            };
            init
        },
        {
            let mut init = string {
                len: (::std::mem::size_of::<[libc::c_char; 22]>() as libc::c_ulong)
                    .wrapping_sub(1 as libc::c_int as libc::c_ulong) as uint32_t,
                data: b"REQ_REDIS_ZINTERSTORE\0" as *const u8 as *const libc::c_char
                    as *mut uint8_t,
            };
            init
        },
        {
            let mut init = string {
                len: (::std::mem::size_of::<[libc::c_char; 20]>() as libc::c_ulong)
                    .wrapping_sub(1 as libc::c_int as libc::c_ulong) as uint32_t,
                data: b"REQ_REDIS_ZLEXCOUNT\0" as *const u8 as *const libc::c_char
                    as *mut uint8_t,
            };
            init
        },
        {
            let mut init = string {
                len: (::std::mem::size_of::<[libc::c_char; 18]>() as libc::c_ulong)
                    .wrapping_sub(1 as libc::c_int as libc::c_ulong) as uint32_t,
                data: b"REQ_REDIS_ZMSCORE\0" as *const u8 as *const libc::c_char
                    as *mut uint8_t,
            };
            init
        },
        {
            let mut init = string {
                len: (::std::mem::size_of::<[libc::c_char; 18]>() as libc::c_ulong)
                    .wrapping_sub(1 as libc::c_int as libc::c_ulong) as uint32_t,
                data: b"REQ_REDIS_ZPOPMIN\0" as *const u8 as *const libc::c_char
                    as *mut uint8_t,
            };
            init
        },
        {
            let mut init = string {
                len: (::std::mem::size_of::<[libc::c_char; 18]>() as libc::c_ulong)
                    .wrapping_sub(1 as libc::c_int as libc::c_ulong) as uint32_t,
                data: b"REQ_REDIS_ZPOPMAX\0" as *const u8 as *const libc::c_char
                    as *mut uint8_t,
            };
            init
        },
        {
            let mut init = string {
                len: (::std::mem::size_of::<[libc::c_char; 22]>() as libc::c_ulong)
                    .wrapping_sub(1 as libc::c_int as libc::c_ulong) as uint32_t,
                data: b"REQ_REDIS_ZRANDMEMBER\0" as *const u8 as *const libc::c_char
                    as *mut uint8_t,
            };
            init
        },
        {
            let mut init = string {
                len: (::std::mem::size_of::<[libc::c_char; 17]>() as libc::c_ulong)
                    .wrapping_sub(1 as libc::c_int as libc::c_ulong) as uint32_t,
                data: b"REQ_REDIS_ZRANGE\0" as *const u8 as *const libc::c_char
                    as *mut uint8_t,
            };
            init
        },
        {
            let mut init = string {
                len: (::std::mem::size_of::<[libc::c_char; 22]>() as libc::c_ulong)
                    .wrapping_sub(1 as libc::c_int as libc::c_ulong) as uint32_t,
                data: b"REQ_REDIS_ZRANGEBYLEX\0" as *const u8 as *const libc::c_char
                    as *mut uint8_t,
            };
            init
        },
        {
            let mut init = string {
                len: (::std::mem::size_of::<[libc::c_char; 24]>() as libc::c_ulong)
                    .wrapping_sub(1 as libc::c_int as libc::c_ulong) as uint32_t,
                data: b"REQ_REDIS_ZRANGEBYSCORE\0" as *const u8 as *const libc::c_char
                    as *mut uint8_t,
            };
            init
        },
        {
            let mut init = string {
                len: (::std::mem::size_of::<[libc::c_char; 22]>() as libc::c_ulong)
                    .wrapping_sub(1 as libc::c_int as libc::c_ulong) as uint32_t,
                data: b"REQ_REDIS_ZRANGESTORE\0" as *const u8 as *const libc::c_char
                    as *mut uint8_t,
            };
            init
        },
        {
            let mut init = string {
                len: (::std::mem::size_of::<[libc::c_char; 16]>() as libc::c_ulong)
                    .wrapping_sub(1 as libc::c_int as libc::c_ulong) as uint32_t,
                data: b"REQ_REDIS_ZRANK\0" as *const u8 as *const libc::c_char
                    as *mut uint8_t,
            };
            init
        },
        {
            let mut init = string {
                len: (::std::mem::size_of::<[libc::c_char; 15]>() as libc::c_ulong)
                    .wrapping_sub(1 as libc::c_int as libc::c_ulong) as uint32_t,
                data: b"REQ_REDIS_ZREM\0" as *const u8 as *const libc::c_char
                    as *mut uint8_t,
            };
            init
        },
        {
            let mut init = string {
                len: (::std::mem::size_of::<[libc::c_char; 26]>() as libc::c_ulong)
                    .wrapping_sub(1 as libc::c_int as libc::c_ulong) as uint32_t,
                data: b"REQ_REDIS_ZREMRANGEBYRANK\0" as *const u8 as *const libc::c_char
                    as *mut uint8_t,
            };
            init
        },
        {
            let mut init = string {
                len: (::std::mem::size_of::<[libc::c_char; 25]>() as libc::c_ulong)
                    .wrapping_sub(1 as libc::c_int as libc::c_ulong) as uint32_t,
                data: b"REQ_REDIS_ZREMRANGEBYLEX\0" as *const u8 as *const libc::c_char
                    as *mut uint8_t,
            };
            init
        },
        {
            let mut init = string {
                len: (::std::mem::size_of::<[libc::c_char; 27]>() as libc::c_ulong)
                    .wrapping_sub(1 as libc::c_int as libc::c_ulong) as uint32_t,
                data: b"REQ_REDIS_ZREMRANGEBYSCORE\0" as *const u8 as *const libc::c_char
                    as *mut uint8_t,
            };
            init
        },
        {
            let mut init = string {
                len: (::std::mem::size_of::<[libc::c_char; 20]>() as libc::c_ulong)
                    .wrapping_sub(1 as libc::c_int as libc::c_ulong) as uint32_t,
                data: b"REQ_REDIS_ZREVRANGE\0" as *const u8 as *const libc::c_char
                    as *mut uint8_t,
            };
            init
        },
        {
            let mut init = string {
                len: (::std::mem::size_of::<[libc::c_char; 25]>() as libc::c_ulong)
                    .wrapping_sub(1 as libc::c_int as libc::c_ulong) as uint32_t,
                data: b"REQ_REDIS_ZREVRANGEBYLEX\0" as *const u8 as *const libc::c_char
                    as *mut uint8_t,
            };
            init
        },
        {
            let mut init = string {
                len: (::std::mem::size_of::<[libc::c_char; 27]>() as libc::c_ulong)
                    .wrapping_sub(1 as libc::c_int as libc::c_ulong) as uint32_t,
                data: b"REQ_REDIS_ZREVRANGEBYSCORE\0" as *const u8 as *const libc::c_char
                    as *mut uint8_t,
            };
            init
        },
        {
            let mut init = string {
                len: (::std::mem::size_of::<[libc::c_char; 19]>() as libc::c_ulong)
                    .wrapping_sub(1 as libc::c_int as libc::c_ulong) as uint32_t,
                data: b"REQ_REDIS_ZREVRANK\0" as *const u8 as *const libc::c_char
                    as *mut uint8_t,
            };
            init
        },
        {
            let mut init = string {
                len: (::std::mem::size_of::<[libc::c_char; 17]>() as libc::c_ulong)
                    .wrapping_sub(1 as libc::c_int as libc::c_ulong) as uint32_t,
                data: b"REQ_REDIS_ZUNION\0" as *const u8 as *const libc::c_char
                    as *mut uint8_t,
            };
            init
        },
        {
            let mut init = string {
                len: (::std::mem::size_of::<[libc::c_char; 16]>() as libc::c_ulong)
                    .wrapping_sub(1 as libc::c_int as libc::c_ulong) as uint32_t,
                data: b"REQ_REDIS_ZSCAN\0" as *const u8 as *const libc::c_char
                    as *mut uint8_t,
            };
            init
        },
        {
            let mut init = string {
                len: (::std::mem::size_of::<[libc::c_char; 17]>() as libc::c_ulong)
                    .wrapping_sub(1 as libc::c_int as libc::c_ulong) as uint32_t,
                data: b"REQ_REDIS_ZSCORE\0" as *const u8 as *const libc::c_char
                    as *mut uint8_t,
            };
            init
        },
        {
            let mut init = string {
                len: (::std::mem::size_of::<[libc::c_char; 22]>() as libc::c_ulong)
                    .wrapping_sub(1 as libc::c_int as libc::c_ulong) as uint32_t,
                data: b"REQ_REDIS_ZUNIONSTORE\0" as *const u8 as *const libc::c_char
                    as *mut uint8_t,
            };
            init
        },
        {
            let mut init = string {
                len: (::std::mem::size_of::<[libc::c_char; 17]>() as libc::c_ulong)
                    .wrapping_sub(1 as libc::c_int as libc::c_ulong) as uint32_t,
                data: b"REQ_REDIS_GEOADD\0" as *const u8 as *const libc::c_char
                    as *mut uint8_t,
            };
            init
        },
        {
            let mut init = string {
                len: (::std::mem::size_of::<[libc::c_char; 18]>() as libc::c_ulong)
                    .wrapping_sub(1 as libc::c_int as libc::c_ulong) as uint32_t,
                data: b"REQ_REDIS_GEODIST\0" as *const u8 as *const libc::c_char
                    as *mut uint8_t,
            };
            init
        },
        {
            let mut init = string {
                len: (::std::mem::size_of::<[libc::c_char; 18]>() as libc::c_ulong)
                    .wrapping_sub(1 as libc::c_int as libc::c_ulong) as uint32_t,
                data: b"REQ_REDIS_GEOHASH\0" as *const u8 as *const libc::c_char
                    as *mut uint8_t,
            };
            init
        },
        {
            let mut init = string {
                len: (::std::mem::size_of::<[libc::c_char; 20]>() as libc::c_ulong)
                    .wrapping_sub(1 as libc::c_int as libc::c_ulong) as uint32_t,
                data: b"REQ_REDIS_GEORADIUS\0" as *const u8 as *const libc::c_char
                    as *mut uint8_t,
            };
            init
        },
        {
            let mut init = string {
                len: (::std::mem::size_of::<[libc::c_char; 28]>() as libc::c_ulong)
                    .wrapping_sub(1 as libc::c_int as libc::c_ulong) as uint32_t,
                data: b"REQ_REDIS_GEORADIUSBYMEMBER\0" as *const u8
                    as *const libc::c_char as *mut uint8_t,
            };
            init
        },
        {
            let mut init = string {
                len: (::std::mem::size_of::<[libc::c_char; 17]>() as libc::c_ulong)
                    .wrapping_sub(1 as libc::c_int as libc::c_ulong) as uint32_t,
                data: b"REQ_REDIS_GEOPOS\0" as *const u8 as *const libc::c_char
                    as *mut uint8_t,
            };
            init
        },
        {
            let mut init = string {
                len: (::std::mem::size_of::<[libc::c_char; 20]>() as libc::c_ulong)
                    .wrapping_sub(1 as libc::c_int as libc::c_ulong) as uint32_t,
                data: b"REQ_REDIS_GEOSEARCH\0" as *const u8 as *const libc::c_char
                    as *mut uint8_t,
            };
            init
        },
        {
            let mut init = string {
                len: (::std::mem::size_of::<[libc::c_char; 25]>() as libc::c_ulong)
                    .wrapping_sub(1 as libc::c_int as libc::c_ulong) as uint32_t,
                data: b"REQ_REDIS_GEOSEARCHSTORE\0" as *const u8 as *const libc::c_char
                    as *mut uint8_t,
            };
            init
        },
        {
            let mut init = string {
                len: (::std::mem::size_of::<[libc::c_char; 15]>() as libc::c_ulong)
                    .wrapping_sub(1 as libc::c_int as libc::c_ulong) as uint32_t,
                data: b"REQ_REDIS_EVAL\0" as *const u8 as *const libc::c_char
                    as *mut uint8_t,
            };
            init
        },
        {
            let mut init = string {
                len: (::std::mem::size_of::<[libc::c_char; 18]>() as libc::c_ulong)
                    .wrapping_sub(1 as libc::c_int as libc::c_ulong) as uint32_t,
                data: b"REQ_REDIS_EVALSHA\0" as *const u8 as *const libc::c_char
                    as *mut uint8_t,
            };
            init
        },
        {
            let mut init = string {
                len: (::std::mem::size_of::<[libc::c_char; 15]>() as libc::c_ulong)
                    .wrapping_sub(1 as libc::c_int as libc::c_ulong) as uint32_t,
                data: b"REQ_REDIS_PING\0" as *const u8 as *const libc::c_char
                    as *mut uint8_t,
            };
            init
        },
        {
            let mut init = string {
                len: (::std::mem::size_of::<[libc::c_char; 15]>() as libc::c_ulong)
                    .wrapping_sub(1 as libc::c_int as libc::c_ulong) as uint32_t,
                data: b"REQ_REDIS_QUIT\0" as *const u8 as *const libc::c_char
                    as *mut uint8_t,
            };
            init
        },
        {
            let mut init = string {
                len: (::std::mem::size_of::<[libc::c_char; 15]>() as libc::c_ulong)
                    .wrapping_sub(1 as libc::c_int as libc::c_ulong) as uint32_t,
                data: b"REQ_REDIS_AUTH\0" as *const u8 as *const libc::c_char
                    as *mut uint8_t,
            };
            init
        },
        {
            let mut init = string {
                len: (::std::mem::size_of::<[libc::c_char; 17]>() as libc::c_ulong)
                    .wrapping_sub(1 as libc::c_int as libc::c_ulong) as uint32_t,
                data: b"REQ_REDIS_SELECT\0" as *const u8 as *const libc::c_char
                    as *mut uint8_t,
            };
            init
        },
        {
            let mut init = string {
                len: (::std::mem::size_of::<[libc::c_char; 18]>() as libc::c_ulong)
                    .wrapping_sub(1 as libc::c_int as libc::c_ulong) as uint32_t,
                data: b"REQ_REDIS_COMMAND\0" as *const u8 as *const libc::c_char
                    as *mut uint8_t,
            };
            init
        },
        {
            let mut init = string {
                len: (::std::mem::size_of::<[libc::c_char; 17]>() as libc::c_ulong)
                    .wrapping_sub(1 as libc::c_int as libc::c_ulong) as uint32_t,
                data: b"REQ_REDIS_LOLWUT\0" as *const u8 as *const libc::c_char
                    as *mut uint8_t,
            };
            init
        },
        {
            let mut init = string {
                len: (::std::mem::size_of::<[libc::c_char; 17]>() as libc::c_ulong)
                    .wrapping_sub(1 as libc::c_int as libc::c_ulong) as uint32_t,
                data: b"RSP_REDIS_STATUS\0" as *const u8 as *const libc::c_char
                    as *mut uint8_t,
            };
            init
        },
        {
            let mut init = string {
                len: (::std::mem::size_of::<[libc::c_char; 16]>() as libc::c_ulong)
                    .wrapping_sub(1 as libc::c_int as libc::c_ulong) as uint32_t,
                data: b"RSP_REDIS_ERROR\0" as *const u8 as *const libc::c_char
                    as *mut uint8_t,
            };
            init
        },
        {
            let mut init = string {
                len: (::std::mem::size_of::<[libc::c_char; 20]>() as libc::c_ulong)
                    .wrapping_sub(1 as libc::c_int as libc::c_ulong) as uint32_t,
                data: b"RSP_REDIS_ERROR_ERR\0" as *const u8 as *const libc::c_char
                    as *mut uint8_t,
            };
            init
        },
        {
            let mut init = string {
                len: (::std::mem::size_of::<[libc::c_char; 20]>() as libc::c_ulong)
                    .wrapping_sub(1 as libc::c_int as libc::c_ulong) as uint32_t,
                data: b"RSP_REDIS_ERROR_OOM\0" as *const u8 as *const libc::c_char
                    as *mut uint8_t,
            };
            init
        },
        {
            let mut init = string {
                len: (::std::mem::size_of::<[libc::c_char; 21]>() as libc::c_ulong)
                    .wrapping_sub(1 as libc::c_int as libc::c_ulong) as uint32_t,
                data: b"RSP_REDIS_ERROR_BUSY\0" as *const u8 as *const libc::c_char
                    as *mut uint8_t,
            };
            init
        },
        {
            let mut init = string {
                len: (::std::mem::size_of::<[libc::c_char; 23]>() as libc::c_ulong)
                    .wrapping_sub(1 as libc::c_int as libc::c_ulong) as uint32_t,
                data: b"RSP_REDIS_ERROR_NOAUTH\0" as *const u8 as *const libc::c_char
                    as *mut uint8_t,
            };
            init
        },
        {
            let mut init = string {
                len: (::std::mem::size_of::<[libc::c_char; 24]>() as libc::c_ulong)
                    .wrapping_sub(1 as libc::c_int as libc::c_ulong) as uint32_t,
                data: b"RSP_REDIS_ERROR_LOADING\0" as *const u8 as *const libc::c_char
                    as *mut uint8_t,
            };
            init
        },
        {
            let mut init = string {
                len: (::std::mem::size_of::<[libc::c_char; 24]>() as libc::c_ulong)
                    .wrapping_sub(1 as libc::c_int as libc::c_ulong) as uint32_t,
                data: b"RSP_REDIS_ERROR_BUSYKEY\0" as *const u8 as *const libc::c_char
                    as *mut uint8_t,
            };
            init
        },
        {
            let mut init = string {
                len: (::std::mem::size_of::<[libc::c_char; 24]>() as libc::c_ulong)
                    .wrapping_sub(1 as libc::c_int as libc::c_ulong) as uint32_t,
                data: b"RSP_REDIS_ERROR_MISCONF\0" as *const u8 as *const libc::c_char
                    as *mut uint8_t,
            };
            init
        },
        {
            let mut init = string {
                len: (::std::mem::size_of::<[libc::c_char; 25]>() as libc::c_ulong)
                    .wrapping_sub(1 as libc::c_int as libc::c_ulong) as uint32_t,
                data: b"RSP_REDIS_ERROR_NOSCRIPT\0" as *const u8 as *const libc::c_char
                    as *mut uint8_t,
            };
            init
        },
        {
            let mut init = string {
                len: (::std::mem::size_of::<[libc::c_char; 25]>() as libc::c_ulong)
                    .wrapping_sub(1 as libc::c_int as libc::c_ulong) as uint32_t,
                data: b"RSP_REDIS_ERROR_READONLY\0" as *const u8 as *const libc::c_char
                    as *mut uint8_t,
            };
            init
        },
        {
            let mut init = string {
                len: (::std::mem::size_of::<[libc::c_char; 26]>() as libc::c_ulong)
                    .wrapping_sub(1 as libc::c_int as libc::c_ulong) as uint32_t,
                data: b"RSP_REDIS_ERROR_WRONGTYPE\0" as *const u8 as *const libc::c_char
                    as *mut uint8_t,
            };
            init
        },
        {
            let mut init = string {
                len: (::std::mem::size_of::<[libc::c_char; 26]>() as libc::c_ulong)
                    .wrapping_sub(1 as libc::c_int as libc::c_ulong) as uint32_t,
                data: b"RSP_REDIS_ERROR_EXECABORT\0" as *const u8 as *const libc::c_char
                    as *mut uint8_t,
            };
            init
        },
        {
            let mut init = string {
                len: (::std::mem::size_of::<[libc::c_char; 27]>() as libc::c_ulong)
                    .wrapping_sub(1 as libc::c_int as libc::c_ulong) as uint32_t,
                data: b"RSP_REDIS_ERROR_MASTERDOWN\0" as *const u8 as *const libc::c_char
                    as *mut uint8_t,
            };
            init
        },
        {
            let mut init = string {
                len: (::std::mem::size_of::<[libc::c_char; 27]>() as libc::c_ulong)
                    .wrapping_sub(1 as libc::c_int as libc::c_ulong) as uint32_t,
                data: b"RSP_REDIS_ERROR_NOREPLICAS\0" as *const u8 as *const libc::c_char
                    as *mut uint8_t,
            };
            init
        },
        {
            let mut init = string {
                len: (::std::mem::size_of::<[libc::c_char; 18]>() as libc::c_ulong)
                    .wrapping_sub(1 as libc::c_int as libc::c_ulong) as uint32_t,
                data: b"RSP_REDIS_INTEGER\0" as *const u8 as *const libc::c_char
                    as *mut uint8_t,
            };
            init
        },
        {
            let mut init = string {
                len: (::std::mem::size_of::<[libc::c_char; 15]>() as libc::c_ulong)
                    .wrapping_sub(1 as libc::c_int as libc::c_ulong) as uint32_t,
                data: b"RSP_REDIS_BULK\0" as *const u8 as *const libc::c_char
                    as *mut uint8_t,
            };
            init
        },
        {
            let mut init = string {
                len: (::std::mem::size_of::<[libc::c_char; 20]>() as libc::c_ulong)
                    .wrapping_sub(1 as libc::c_int as libc::c_ulong) as uint32_t,
                data: b"RSP_REDIS_MULTIBULK\0" as *const u8 as *const libc::c_char
                    as *mut uint8_t,
            };
            init
        },
        {
            let mut init = string {
                len: (::std::mem::size_of::<[libc::c_char; 9]>() as libc::c_ulong)
                    .wrapping_sub(1 as libc::c_int as libc::c_ulong) as uint32_t,
                data: b"SENTINEL\0" as *const u8 as *const libc::c_char as *mut uint8_t,
            };
            init
        },
        {
            let mut init = string {
                len: 0 as libc::c_int as uint32_t,
                data: 0 as *mut uint8_t,
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
