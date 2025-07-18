use ::libc;
extern "C" {
    pub type _IO_wide_data;
    pub type _IO_codecvt;
    pub type _IO_marker;
    pub type osip_sem;
    pub type osip_mutex;
    fn malloc(_: libc::c_ulong) -> *mut libc::c_void;
    fn free(__ptr: *mut libc::c_void);
    fn memset(
        _: *mut libc::c_void,
        _: libc::c_int,
        _: libc::c_ulong,
    ) -> *mut libc::c_void;
    fn osip_uri_param_get_byname(
        url_params: *mut osip_list_t,
        name: *mut libc::c_char,
        dest: *mut *mut osip_uri_param_t,
    ) -> libc::c_int;
    static mut osip_malloc_func: Option::<osip_malloc_func_t>;
    fn osip_strcasecmp(s1: *const libc::c_char, s2: *const libc::c_char) -> libc::c_int;
    fn osip_strdup(ch: *const libc::c_char) -> *mut libc::c_char;
    fn osip_trace(
        fi: *const libc::c_char,
        li: libc::c_int,
        level: osip_trace_level_t,
        f: *mut FILE,
        chfr: *const libc::c_char,
        _: ...
    ) -> libc::c_int;
    fn osip_gettimeofday(tp: *mut timeval, tz: *mut libc::c_void) -> libc::c_int;
    fn add_gettimeofday(atv: *mut timeval, ms: libc::c_int);
    fn osip_via_get_protocol(header: *mut osip_via_t) -> *mut libc::c_char;
    fn osip_atoi(number: *const libc::c_char) -> libc::c_int;
    fn osip_message_get_route(
        sip: *const osip_message_t,
        pos: libc::c_int,
        dest: *mut *mut osip_route_t,
    ) -> libc::c_int;
    fn osip_message_get_via(
        sip: *const osip_message_t,
        pos: libc::c_int,
        dest: *mut *mut osip_via_t,
    ) -> libc::c_int;
    static mut osip_free_func: Option::<osip_free_func_t>;
    fn __osip_transaction_need_timer_x_event(
        xixt: *mut libc::c_void,
        timer: *mut timeval,
        cond_state: libc::c_int,
        transactionid: libc::c_int,
        TIMER_VAL: libc::c_int,
    ) -> *mut osip_event_t;
}
pub type size_t = libc::c_ulong;
pub type __off_t = libc::c_long;
pub type __off64_t = libc::c_long;
pub type __time_t = libc::c_long;
pub type __suseconds_t = libc::c_long;
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct timeval {
    pub tv_sec: __time_t,
    pub tv_usec: __suseconds_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct __node {
    pub next: *mut __node_t,
    pub element: *mut libc::c_void,
}
pub type __node_t = __node;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct osip_list {
    pub nb_elt: libc::c_int,
    pub node: *mut __node_t,
}
pub type osip_list_t = osip_list;
pub type osip_malloc_func_t = unsafe extern "C" fn(size_t) -> *mut libc::c_void;
pub type osip_free_func_t = unsafe extern "C" fn(*mut libc::c_void) -> ();
pub type _trace_level = libc::c_uint;
pub const END_TRACE_LEVEL: _trace_level = 8;
pub const TRACE_LEVEL7: _trace_level = 7;
pub const TRACE_LEVEL6: _trace_level = 6;
pub const TRACE_LEVEL5: _trace_level = 5;
pub const TRACE_LEVEL4: _trace_level = 4;
pub const TRACE_LEVEL3: _trace_level = 3;
pub const TRACE_LEVEL2: _trace_level = 2;
pub const TRACE_LEVEL1: _trace_level = 1;
pub const TRACE_LEVEL0: _trace_level = 0;
pub type osip_trace_level_t = _trace_level;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct osip_uri_param {
    pub gname: *mut libc::c_char,
    pub gvalue: *mut libc::c_char,
}
pub type osip_uri_param_t = osip_uri_param;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct osip_uri {
    pub scheme: *mut libc::c_char,
    pub username: *mut libc::c_char,
    pub password: *mut libc::c_char,
    pub host: *mut libc::c_char,
    pub port: *mut libc::c_char,
    pub url_params: osip_list_t,
    pub url_headers: osip_list_t,
    pub string: *mut libc::c_char,
}
pub type osip_uri_t = osip_uri;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct osip_content_type {
    pub type_0: *mut libc::c_char,
    pub subtype: *mut libc::c_char,
    pub gen_params: osip_list_t,
}
pub type osip_content_type_t = osip_content_type;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct osip_content_length {
    pub value: *mut libc::c_char,
}
pub type osip_content_length_t = osip_content_length;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct osip_call_id {
    pub number: *mut libc::c_char,
    pub host: *mut libc::c_char,
}
pub type osip_call_id_t = osip_call_id;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct osip_from {
    pub displayname: *mut libc::c_char,
    pub url: *mut osip_uri_t,
    pub gen_params: osip_list_t,
}
pub type osip_from_t = osip_from;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct osip_cseq {
    pub method: *mut libc::c_char,
    pub number: *mut libc::c_char,
}
pub type osip_cseq_t = osip_cseq;
pub type osip_mime_version_t = osip_content_length_t;
pub type osip_route_t = osip_from_t;
pub type osip_to_t = osip_from_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct osip_via {
    pub version: *mut libc::c_char,
    pub protocol: *mut libc::c_char,
    pub host: *mut libc::c_char,
    pub port: *mut libc::c_char,
    pub comment: *mut libc::c_char,
    pub via_params: osip_list_t,
}
pub type osip_via_t = osip_via;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct osip_message {
    pub sip_version: *mut libc::c_char,
    pub req_uri: *mut osip_uri_t,
    pub sip_method: *mut libc::c_char,
    pub status_code: libc::c_int,
    pub reason_phrase: *mut libc::c_char,
    pub accepts: osip_list_t,
    pub accept_encodings: osip_list_t,
    pub accept_languages: osip_list_t,
    pub alert_infos: osip_list_t,
    pub allows: osip_list_t,
    pub authentication_infos: osip_list_t,
    pub authorizations: osip_list_t,
    pub call_id: *mut osip_call_id_t,
    pub call_infos: osip_list_t,
    pub contacts: osip_list_t,
    pub content_encodings: osip_list_t,
    pub content_length: *mut osip_content_length_t,
    pub content_type: *mut osip_content_type_t,
    pub cseq: *mut osip_cseq_t,
    pub error_infos: osip_list_t,
    pub from: *mut osip_from_t,
    pub mime_version: *mut osip_mime_version_t,
    pub proxy_authenticates: osip_list_t,
    pub proxy_authentication_infos: osip_list_t,
    pub proxy_authorizations: osip_list_t,
    pub record_routes: osip_list_t,
    pub routes: osip_list_t,
    pub to: *mut osip_to_t,
    pub vias: osip_list_t,
    pub www_authenticates: osip_list_t,
    pub headers: osip_list_t,
    pub bodies: osip_list_t,
    pub message_property: libc::c_int,
    pub message: *mut libc::c_char,
    pub message_length: size_t,
    pub application_data: *mut libc::c_void,
}
pub type osip_message_t = osip_message;
pub type osip_fifo_state = libc::c_uint;
pub const osip_empty: osip_fifo_state = 1;
pub const osip_ok: osip_fifo_state = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct osip_fifo {
    pub qislocked: *mut osip_mutex,
    pub qisempty: *mut osip_sem,
    pub queue: osip_list_t,
    pub nb_elt: libc::c_int,
    pub state: osip_fifo_state,
}
pub type osip_fifo_t = osip_fifo;
pub type _state_t = libc::c_uint;
pub const DIALOG_CLOSE: _state_t = 22;
pub const DIALOG_CONFIRMED: _state_t = 21;
pub const DIALOG_EARLY: _state_t = 20;
pub const NIST_TERMINATED: _state_t = 19;
pub const NIST_COMPLETED: _state_t = 18;
pub const NIST_PROCEEDING: _state_t = 17;
pub const NIST_TRYING: _state_t = 16;
pub const NIST_PRE_TRYING: _state_t = 15;
pub const NICT_TERMINATED: _state_t = 14;
pub const NICT_COMPLETED: _state_t = 13;
pub const NICT_PROCEEDING: _state_t = 12;
pub const NICT_TRYING: _state_t = 11;
pub const NICT_PRE_TRYING: _state_t = 10;
pub const IST_TERMINATED: _state_t = 9;
pub const IST_CONFIRMED: _state_t = 8;
pub const IST_COMPLETED: _state_t = 7;
pub const IST_PROCEEDING: _state_t = 6;
pub const IST_PRE_PROCEEDING: _state_t = 5;
pub const ICT_TERMINATED: _state_t = 4;
pub const ICT_COMPLETED: _state_t = 3;
pub const ICT_PROCEEDING: _state_t = 2;
pub const ICT_CALLING: _state_t = 1;
pub const ICT_PRE_CALLING: _state_t = 0;
pub type state_t = _state_t;
pub type type_t = libc::c_uint;
pub const UNKNOWN_EVT: type_t = 23;
pub const KILL_TRANSACTION: type_t = 22;
pub const SND_STATUS_3456XX: type_t = 21;
pub const SND_STATUS_2XX: type_t = 20;
pub const SND_STATUS_1XX: type_t = 19;
pub const SND_REQUEST: type_t = 18;
pub const SND_REQACK: type_t = 17;
pub const SND_REQINVITE: type_t = 16;
pub const RCV_STATUS_3456XX: type_t = 15;
pub const RCV_STATUS_2XX: type_t = 14;
pub const RCV_STATUS_1XX: type_t = 13;
pub const RCV_REQUEST: type_t = 12;
pub const RCV_REQACK: type_t = 11;
pub const RCV_REQINVITE: type_t = 10;
pub const TIMEOUT_J: type_t = 9;
pub const TIMEOUT_I: type_t = 8;
pub const TIMEOUT_H: type_t = 7;
pub const TIMEOUT_G: type_t = 6;
pub const TIMEOUT_K: type_t = 5;
pub const TIMEOUT_F: type_t = 4;
pub const TIMEOUT_E: type_t = 3;
pub const TIMEOUT_D: type_t = 2;
pub const TIMEOUT_B: type_t = 1;
pub const TIMEOUT_A: type_t = 0;
pub type osip_fsm_type_t = libc::c_uint;
pub const NIST: osip_fsm_type_t = 3;
pub const NICT: osip_fsm_type_t = 2;
pub const IST: osip_fsm_type_t = 1;
pub const ICT: osip_fsm_type_t = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct osip_ict {
    pub timer_a_length: libc::c_int,
    pub timer_a_start: timeval,
    pub timer_b_length: libc::c_int,
    pub timer_b_start: timeval,
    pub timer_d_length: libc::c_int,
    pub timer_d_start: timeval,
    pub destination: *mut libc::c_char,
    pub port: libc::c_int,
}
pub type osip_ict_t = osip_ict;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct osip_nict {
    pub timer_e_length: libc::c_int,
    pub timer_e_start: timeval,
    pub timer_f_length: libc::c_int,
    pub timer_f_start: timeval,
    pub timer_k_length: libc::c_int,
    pub timer_k_start: timeval,
    pub destination: *mut libc::c_char,
    pub port: libc::c_int,
}
pub type osip_nict_t = osip_nict;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct osip_ist {
    pub timer_g_length: libc::c_int,
    pub timer_g_start: timeval,
    pub timer_h_length: libc::c_int,
    pub timer_h_start: timeval,
    pub timer_i_length: libc::c_int,
    pub timer_i_start: timeval,
}
pub type osip_ist_t = osip_ist;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct osip_nist {
    pub timer_j_length: libc::c_int,
    pub timer_j_start: timeval,
}
pub type osip_nist_t = osip_nist;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct osip_srv_entry {
    pub srv: [libc::c_char; 512],
    pub priority: libc::c_int,
    pub weight: libc::c_int,
    pub rweight: libc::c_int,
    pub port: libc::c_int,
    pub ipaddress: [libc::c_char; 512],
    pub srv_is_broken: timeval,
}
pub type osip_srv_entry_t = osip_srv_entry;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct osip_srv_record {
    pub name: [libc::c_char; 1024],
    pub srv_state: libc::c_int,
    pub flag: [libc::c_char; 256],
    pub protocol: [libc::c_char; 1024],
    pub regexp: [libc::c_char; 1024],
    pub replacement: [libc::c_char; 1024],
    pub order: libc::c_int,
    pub preference: libc::c_int,
    pub index: libc::c_int,
    pub srventry: [osip_srv_entry_t; 10],
}
pub type osip_srv_record_t = osip_srv_record;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct osip_naptr {
    pub domain: [libc::c_char; 512],
    pub AUS: [libc::c_char; 64],
    pub naptr_state: libc::c_int,
    pub arg: *mut libc::c_void,
    pub keep_in_cache: libc::c_int,
    pub sipudp_record: osip_srv_record,
    pub siptcp_record: osip_srv_record,
    pub siptls_record: osip_srv_record,
    pub sipdtls_record: osip_srv_record,
    pub sipsctp_record: osip_srv_record,
    pub sipenum_record: osip_srv_record,
}
pub type osip_naptr_t = osip_naptr;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct osip_transaction {
    pub your_instance: *mut libc::c_void,
    pub transactionid: libc::c_int,
    pub transactionff: *mut osip_fifo_t,
    pub topvia: *mut osip_via_t,
    pub from: *mut osip_from_t,
    pub to: *mut osip_to_t,
    pub callid: *mut osip_call_id_t,
    pub cseq: *mut osip_cseq_t,
    pub orig_request: *mut osip_message_t,
    pub last_response: *mut osip_message_t,
    pub ack: *mut osip_message_t,
    pub state: state_t,
    pub birth_time: time_t,
    pub completed_time: time_t,
    pub created_time: timeval,
    pub destroyed_time: timeval,
    pub in_socket: libc::c_int,
    pub out_socket: libc::c_int,
    pub config: *mut libc::c_void,
    pub ctx_type: osip_fsm_type_t,
    pub ict_context: *mut osip_ict_t,
    pub ist_context: *mut osip_ist_t,
    pub nict_context: *mut osip_nict_t,
    pub nist_context: *mut osip_nist_t,
    pub record: osip_srv_record_t,
    pub naptr_record: *mut osip_naptr_t,
    pub reserved1: *mut libc::c_void,
    pub reserved2: *mut libc::c_void,
    pub reserved3: *mut libc::c_void,
    pub reserved4: *mut libc::c_void,
    pub reserved5: *mut libc::c_void,
    pub reserved6: *mut libc::c_void,
}
pub type osip_transaction_t = osip_transaction;
pub type osip_message_cb_t = Option::<
    unsafe extern "C" fn(libc::c_int, *mut osip_transaction_t, *mut osip_message_t) -> (),
>;
pub type osip_kill_transaction_cb_t = Option::<
    unsafe extern "C" fn(libc::c_int, *mut osip_transaction_t) -> (),
>;
pub type osip_transport_error_cb_t = Option::<
    unsafe extern "C" fn(libc::c_int, *mut osip_transaction_t, libc::c_int) -> (),
>;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct osip {
    pub application_context: *mut libc::c_void,
    pub ict_fastmutex: *mut libc::c_void,
    pub ist_fastmutex: *mut libc::c_void,
    pub nict_fastmutex: *mut libc::c_void,
    pub nist_fastmutex: *mut libc::c_void,
    pub ixt_fastmutex: *mut libc::c_void,
    pub id_mutex: *mut libc::c_void,
    pub transactionid: libc::c_int,
    pub osip_ict_transactions: osip_list_t,
    pub osip_ist_transactions: osip_list_t,
    pub osip_nict_transactions: osip_list_t,
    pub osip_nist_transactions: osip_list_t,
    pub ixt_retransmissions: osip_list_t,
    pub msg_callbacks: [osip_message_cb_t; 60],
    pub kill_callbacks: [osip_kill_transaction_cb_t; 4],
    pub tp_error_callbacks: [osip_transport_error_cb_t; 4],
    pub cb_send_message: Option::<
        unsafe extern "C" fn(
            *mut osip_transaction_t,
            *mut osip_message_t,
            *mut libc::c_char,
            libc::c_int,
            libc::c_int,
        ) -> libc::c_int,
    >,
    pub osip_ict_hastable: *mut libc::c_void,
    pub osip_ist_hastable: *mut libc::c_void,
    pub osip_nict_hastable: *mut libc::c_void,
    pub osip_nist_hastable: *mut libc::c_void,
}
pub type osip_t = osip;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct osip_event {
    pub type_0: type_t,
    pub transactionid: libc::c_int,
    pub sip: *mut osip_message_t,
}
pub type osip_event_t = osip_event;
pub unsafe extern "C" fn __osip_nict_init(
    mut nict: *mut *mut osip_nict_t,
    mut osip: *mut osip_t,
    mut request: *mut osip_message_t,
) -> libc::c_int {
    let mut route: *mut osip_route_t = 0 as *mut osip_route_t;
    let mut i: libc::c_int = 0;
    osip_trace(
        b"nict.c\0" as *const u8 as *const libc::c_char,
        30 as libc::c_int,
        TRACE_LEVEL5,
        0 as *mut FILE,
        b"allocating NICT context\n\0" as *const u8 as *const libc::c_char,
    );
    *nict = (if osip_malloc_func.is_some() {
        osip_malloc_func.unwrap()(::std::mem::size_of::<osip_nict_t>() as libc::c_ulong)
    } else {
        malloc(::std::mem::size_of::<osip_nict_t>() as libc::c_ulong)
    }) as *mut osip_nict_t;
    if (*nict).is_null() {
        return -(4 as libc::c_int);
    }
    memset(
        *nict as *mut libc::c_void,
        0 as libc::c_int,
        ::std::mem::size_of::<osip_nict_t>() as libc::c_ulong,
    );
    let mut via: *mut osip_via_t = 0 as *mut osip_via_t;
    let mut proto: *mut libc::c_char = 0 as *mut libc::c_char;
    i = osip_message_get_via(request, 0 as libc::c_int, &mut via);
    if i < 0 as libc::c_int {
        if !(*nict).is_null() {
            if osip_free_func.is_some() {
                osip_free_func.unwrap()(*nict as *mut libc::c_void);
            } else {
                free(*nict as *mut libc::c_void);
            }
        }
        *nict = 0 as *mut osip_nict_t;
        return i;
    }
    proto = osip_via_get_protocol(via);
    if proto.is_null() {
        if !(*nict).is_null() {
            if osip_free_func.is_some() {
                osip_free_func.unwrap()(*nict as *mut libc::c_void);
            } else {
                free(*nict as *mut libc::c_void);
            }
        }
        *nict = 0 as *mut osip_nict_t;
        return -(1 as libc::c_int);
    }
    if osip_strcasecmp(proto, b"TCP\0" as *const u8 as *const libc::c_char)
        != 0 as libc::c_int
        && osip_strcasecmp(proto, b"TLS\0" as *const u8 as *const libc::c_char)
            != 0 as libc::c_int
        && osip_strcasecmp(proto, b"SCTP\0" as *const u8 as *const libc::c_char)
            != 0 as libc::c_int
    {
        (**nict).timer_e_length = 500 as libc::c_int;
        (**nict).timer_k_length = 5000 as libc::c_int;
        (**nict).timer_e_start.tv_sec = -(1 as libc::c_int) as __time_t;
        (**nict).timer_k_start.tv_sec = -(1 as libc::c_int) as __time_t;
    } else {
        (**nict).timer_e_length = 500 as libc::c_int;
        (**nict).timer_k_length = 0 as libc::c_int;
        (**nict).timer_e_start.tv_sec = -(1 as libc::c_int) as __time_t;
        (**nict).timer_k_start.tv_sec = -(1 as libc::c_int) as __time_t;
    }
    osip_message_get_route(request, 0 as libc::c_int, &mut route);
    if !route.is_null() && !((*route).url).is_null() {
        let mut lr_param: *mut osip_uri_param_t = 0 as *mut osip_uri_param_t;
        osip_uri_param_get_byname(
            &mut (*(*route).url).url_params,
            b"lr\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            &mut lr_param,
        );
        if lr_param.is_null() {
            route = 0 as *mut osip_route_t;
        }
    }
    if !route.is_null() && !((*route).url).is_null() {
        let mut port: libc::c_int = 5060 as libc::c_int;
        if !((*(*route).url).port).is_null() {
            port = osip_atoi((*(*route).url).port);
        }
        osip_nict_set_destination(*nict, osip_strdup((*(*route).url).host), port);
    } else {
        let mut port_0: libc::c_int = 5060 as libc::c_int;
        let mut maddr_param: *mut osip_uri_param_t = 0 as *mut osip_uri_param_t;
        let mut obr_param: *mut osip_uri_param_t = 0 as *mut osip_uri_param_t;
        let mut obp_param: *mut osip_uri_param_t = 0 as *mut osip_uri_param_t;
        port_0 = 5060 as libc::c_int;
        if !((*(*request).req_uri).port).is_null() {
            port_0 = osip_atoi((*(*request).req_uri).port);
        }
        osip_uri_param_get_byname(
            &mut (*(*request).req_uri).url_params,
            b"x-obr\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            &mut obr_param,
        );
        osip_uri_param_get_byname(
            &mut (*(*request).req_uri).url_params,
            b"x-obp\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            &mut obp_param,
        );
        osip_uri_param_get_byname(
            &mut (*(*request).req_uri).url_params,
            b"maddr\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            &mut maddr_param,
        );
        if !maddr_param.is_null() && !((*maddr_param).gvalue).is_null() {
            osip_nict_set_destination(*nict, osip_strdup((*maddr_param).gvalue), port_0);
        } else if !obr_param.is_null() && !((*obr_param).gvalue).is_null()
            && !obp_param.is_null() && !((*obp_param).gvalue).is_null()
        {
            osip_nict_set_destination(
                *nict,
                osip_strdup((*obr_param).gvalue),
                osip_atoi((*obp_param).gvalue),
            );
        } else {
            osip_nict_set_destination(
                *nict,
                osip_strdup((*(*request).req_uri).host),
                port_0,
            );
        }
    }
    (**nict).timer_f_length = 64 as libc::c_int * 500 as libc::c_int;
    osip_gettimeofday(&mut (**nict).timer_f_start, 0 as *mut libc::c_void);
    add_gettimeofday(&mut (**nict).timer_f_start, (**nict).timer_f_length);
    return 0 as libc::c_int;
}
pub unsafe extern "C" fn __osip_nict_free(mut nict: *mut osip_nict_t) -> libc::c_int {
    if nict.is_null() {
        return 0 as libc::c_int;
    }
    osip_trace(
        b"nict.c\0" as *const u8 as *const libc::c_char,
        157 as libc::c_int,
        TRACE_LEVEL5,
        0 as *mut FILE,
        b"free nict resource\n\0" as *const u8 as *const libc::c_char,
    );
    if !((*nict).destination).is_null() {
        if osip_free_func.is_some() {
            osip_free_func.unwrap()((*nict).destination as *mut libc::c_void);
        } else {
            free((*nict).destination as *mut libc::c_void);
        }
    }
    if !nict.is_null() {
        if osip_free_func.is_some() {
            osip_free_func.unwrap()(nict as *mut libc::c_void);
        } else {
            free(nict as *mut libc::c_void);
        }
    }
    return 0 as libc::c_int;
}
pub unsafe extern "C" fn osip_nict_set_destination(
    mut nict: *mut osip_nict_t,
    mut destination: *mut libc::c_char,
    mut port: libc::c_int,
) -> libc::c_int {
    if nict.is_null() {
        return -(2 as libc::c_int);
    }
    if !((*nict).destination).is_null() {
        if !((*nict).destination).is_null() {
            if osip_free_func.is_some() {
                osip_free_func.unwrap()((*nict).destination as *mut libc::c_void);
            } else {
                free((*nict).destination as *mut libc::c_void);
            }
        }
    }
    (*nict).destination = destination;
    (*nict).port = port;
    return 0 as libc::c_int;
}
pub unsafe extern "C" fn __osip_nict_need_timer_e_event(
    mut nict: *mut osip_nict_t,
    mut state: state_t,
    mut transactionid: libc::c_int,
) -> *mut osip_event_t {
    return __osip_transaction_need_timer_x_event(
        nict as *mut libc::c_void,
        &mut (*nict).timer_e_start,
        (state as libc::c_uint == NICT_PROCEEDING as libc::c_int as libc::c_uint
            || state as libc::c_uint == NICT_TRYING as libc::c_int as libc::c_uint)
            as libc::c_int,
        transactionid,
        TIMEOUT_E as libc::c_int,
    );
}
pub unsafe extern "C" fn __osip_nict_need_timer_f_event(
    mut nict: *mut osip_nict_t,
    mut state: state_t,
    mut transactionid: libc::c_int,
) -> *mut osip_event_t {
    return __osip_transaction_need_timer_x_event(
        nict as *mut libc::c_void,
        &mut (*nict).timer_f_start,
        (state as libc::c_uint == NICT_PROCEEDING as libc::c_int as libc::c_uint
            || state as libc::c_uint == NICT_TRYING as libc::c_int as libc::c_uint)
            as libc::c_int,
        transactionid,
        TIMEOUT_F as libc::c_int,
    );
}
pub unsafe extern "C" fn __osip_nict_need_timer_k_event(
    mut nict: *mut osip_nict_t,
    mut state: state_t,
    mut transactionid: libc::c_int,
) -> *mut osip_event_t {
    return __osip_transaction_need_timer_x_event(
        nict as *mut libc::c_void,
        &mut (*nict).timer_k_start,
        (state as libc::c_uint == NICT_COMPLETED as libc::c_int as libc::c_uint)
            as libc::c_int,
        transactionid,
        TIMEOUT_K as libc::c_int,
    );
}
