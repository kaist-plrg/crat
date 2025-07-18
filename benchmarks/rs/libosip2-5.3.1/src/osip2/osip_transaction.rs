use ::libc;
extern "C" {
    pub type _IO_wide_data;
    pub type _IO_codecvt;
    pub type _IO_marker;
    pub type osip_sem;
    pub type osip_mutex;
    fn strcmp(_: *const libc::c_char, _: *const libc::c_char) -> libc::c_int;
    fn strncmp(
        _: *const libc::c_char,
        _: *const libc::c_char,
        _: libc::c_ulong,
    ) -> libc::c_int;
    fn strlen(_: *const libc::c_char) -> libc::c_ulong;
    fn malloc(_: libc::c_ulong) -> *mut libc::c_void;
    fn free(__ptr: *mut libc::c_void);
    fn memset(
        _: *mut libc::c_void,
        _: libc::c_int,
        _: libc::c_ulong,
    ) -> *mut libc::c_void;
    fn memcpy(
        _: *mut libc::c_void,
        _: *const libc::c_void,
        _: libc::c_ulong,
    ) -> *mut libc::c_void;
    fn osip_uri_param_get_byname(
        url_params: *mut osip_list_t,
        name: *mut libc::c_char,
        dest: *mut *mut osip_uri_param_t,
    ) -> libc::c_int;
    fn osip_list_get(li: *const osip_list_t, pos: libc::c_int) -> *mut libc::c_void;
    static mut osip_malloc_func: Option::<osip_malloc_func_t>;
    static mut osip_free_func: Option::<osip_free_func_t>;
    fn osip_atoi(number: *const libc::c_char) -> libc::c_int;
    fn osip_trace(
        fi: *const libc::c_char,
        li: libc::c_int,
        level: osip_trace_level_t,
        f: *mut FILE,
        chfr: *const libc::c_char,
        _: ...
    ) -> libc::c_int;
    fn osip_getsystemtime(t: *mut time_t) -> time_t;
    fn osip_call_id_free(header: *mut osip_call_id_t);
    fn osip_call_id_clone(
        header: *const osip_call_id_t,
        dest: *mut *mut osip_call_id_t,
    ) -> libc::c_int;
    fn osip_call_id_match(
        callid1: *mut osip_call_id_t,
        callid2: *mut osip_call_id_t,
    ) -> libc::c_int;
    fn osip_from_free(header: *mut osip_from_t);
    fn osip_from_clone(
        header: *const osip_from_t,
        dest: *mut *mut osip_from_t,
    ) -> libc::c_int;
    fn osip_from_tag_match(
        from1: *mut osip_from_t,
        from2: *mut osip_from_t,
    ) -> libc::c_int;
    fn osip_cseq_free(header: *mut osip_cseq_t);
    fn osip_cseq_clone(
        header: *const osip_cseq_t,
        dest: *mut *mut osip_cseq_t,
    ) -> libc::c_int;
    fn osip_cseq_match(cseq1: *mut osip_cseq_t, cseq2: *mut osip_cseq_t) -> libc::c_int;
    fn osip_to_free(header: *mut osip_to_t);
    fn osip_to_clone(header: *const osip_to_t, dest: *mut *mut osip_to_t) -> libc::c_int;
    fn osip_to_tag_match(to1: *mut osip_to_t, to2: *mut osip_to_t) -> libc::c_int;
    fn osip_via_free(header: *mut osip_via_t);
    fn osip_via_clone(
        header: *const osip_via_t,
        dest: *mut *mut osip_via_t,
    ) -> libc::c_int;
    fn osip_via_get_host(header: *mut osip_via_t) -> *mut libc::c_char;
    fn osip_via_get_port(header: *mut osip_via_t) -> *mut libc::c_char;
    fn osip_via_match(via1: *mut osip_via_t, via2: *mut osip_via_t) -> libc::c_int;
    fn osip_message_free(sip: *mut osip_message_t);
    fn osip_fifo_init(ff: *mut osip_fifo_t);
    fn osip_fifo_free(ff: *mut osip_fifo_t);
    fn osip_fifo_add(ff: *mut osip_fifo_t, element: *mut libc::c_void) -> libc::c_int;
    fn osip_fifo_tryget(ff: *mut osip_fifo_t) -> *mut libc::c_void;
    fn osip_remove_transaction(
        osip: *mut osip_t,
        ict: *mut osip_transaction_t,
    ) -> libc::c_int;
    fn osip_gettimeofday(tp: *mut timeval, tz: *mut libc::c_void) -> libc::c_int;
    fn fsm_callmethod(
        type_0: type_t,
        state: state_t,
        statemachine: *mut osip_statemachine_t,
        sipevent: *mut libc::c_void,
        transaction: *mut libc::c_void,
    ) -> libc::c_int;
    fn __osip_add_nist(osip: *mut osip_t, nist: *mut osip_transaction_t) -> libc::c_int;
    fn __osip_nist_free(nist: *mut osip_nist_t) -> libc::c_int;
    fn __osip_nict_free(nict: *mut osip_nict_t) -> libc::c_int;
    fn __osip_ist_free(ist: *mut osip_ist_t) -> libc::c_int;
    fn __osip_ict_free(ict: *mut osip_ict_t) -> libc::c_int;
    fn __osip_nist_init(
        nist: *mut *mut osip_nist_t,
        osip: *mut osip_t,
        request: *mut osip_message_t,
    ) -> libc::c_int;
    fn __osip_add_nict(osip: *mut osip_t, nict: *mut osip_transaction_t) -> libc::c_int;
    fn __osip_nict_init(
        nict: *mut *mut osip_nict_t,
        osip: *mut osip_t,
        request: *mut osip_message_t,
    ) -> libc::c_int;
    fn __osip_add_ist(osip: *mut osip_t, ist: *mut osip_transaction_t) -> libc::c_int;
    fn __osip_ist_init(
        ist: *mut *mut osip_ist_t,
        osip: *mut osip_t,
        invite: *mut osip_message_t,
    ) -> libc::c_int;
    fn __osip_add_ict(osip: *mut osip_t, ict: *mut osip_transaction_t) -> libc::c_int;
    fn __osip_ict_init(
        ict: *mut *mut osip_ict_t,
        osip: *mut osip_t,
        invite: *mut osip_message_t,
    ) -> libc::c_int;
    fn __osip_event_new(type_0: type_t, transactionid: libc::c_int) -> *mut osip_event_t;
    static mut ict_fsm: osip_statemachine_t;
    static mut ist_fsm: osip_statemachine_t;
    static mut nict_fsm: osip_statemachine_t;
    static mut nist_fsm: osip_statemachine_t;
    fn osip_id_mutex_lock(osip: *mut osip_t) -> libc::c_int;
    fn osip_id_mutex_unlock(osip: *mut osip_t) -> libc::c_int;
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
pub type osip_generic_param_t = osip_uri_param_t;
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
pub type osip_statemachine_t = osip_statemachine;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct osip_statemachine {
    pub transitions: *mut _transition_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _transition_t {
    pub state: state_t,
    pub type_0: type_t,
    pub method: Option::<
        unsafe extern "C" fn(*mut libc::c_void, *mut libc::c_void) -> (),
    >,
    pub next: *mut _transition_t,
    pub parent: *mut _transition_t,
}
unsafe extern "C" fn __osip_transaction_set_topvia(
    mut transaction: *mut osip_transaction_t,
    mut topvia: *mut osip_via_t,
) -> libc::c_int {
    let mut i: libc::c_int = 0;
    if transaction.is_null() {
        return -(2 as libc::c_int);
    }
    i = osip_via_clone(topvia, &mut (*transaction).topvia);
    if i == 0 as libc::c_int {
        return 0 as libc::c_int;
    }
    (*transaction).topvia = 0 as *mut osip_via_t;
    return i;
}
unsafe extern "C" fn __osip_transaction_set_from(
    mut transaction: *mut osip_transaction_t,
    mut from: *mut osip_from_t,
) -> libc::c_int {
    let mut i: libc::c_int = 0;
    if transaction.is_null() {
        return -(2 as libc::c_int);
    }
    i = osip_from_clone(from, &mut (*transaction).from);
    if i == 0 as libc::c_int {
        return 0 as libc::c_int;
    }
    (*transaction).from = 0 as *mut osip_from_t;
    return i;
}
unsafe extern "C" fn __osip_transaction_set_to(
    mut transaction: *mut osip_transaction_t,
    mut to: *mut osip_to_t,
) -> libc::c_int {
    let mut i: libc::c_int = 0;
    if transaction.is_null() {
        return -(2 as libc::c_int);
    }
    i = osip_to_clone(to, &mut (*transaction).to);
    if i == 0 as libc::c_int {
        return 0 as libc::c_int;
    }
    (*transaction).to = 0 as *mut osip_to_t;
    return i;
}
unsafe extern "C" fn __osip_transaction_set_call_id(
    mut transaction: *mut osip_transaction_t,
    mut call_id: *mut osip_call_id_t,
) -> libc::c_int {
    let mut i: libc::c_int = 0;
    if transaction.is_null() {
        return -(2 as libc::c_int);
    }
    i = osip_call_id_clone(call_id, &mut (*transaction).callid);
    if i == 0 as libc::c_int {
        return 0 as libc::c_int;
    }
    (*transaction).callid = 0 as *mut osip_call_id_t;
    return i;
}
unsafe extern "C" fn __osip_transaction_set_cseq(
    mut transaction: *mut osip_transaction_t,
    mut cseq: *mut osip_cseq_t,
) -> libc::c_int {
    let mut i: libc::c_int = 0;
    if transaction.is_null() {
        return -(2 as libc::c_int);
    }
    i = osip_cseq_clone(cseq, &mut (*transaction).cseq);
    if i == 0 as libc::c_int {
        return 0 as libc::c_int;
    }
    (*transaction).cseq = 0 as *mut osip_cseq_t;
    return i;
}
pub unsafe extern "C" fn osip_transaction_init(
    mut transaction: *mut *mut osip_transaction_t,
    mut ctx_type: osip_fsm_type_t,
    mut osip: *mut osip_t,
    mut request: *mut osip_message_t,
) -> libc::c_int {
    let mut topvia: *mut osip_via_t = 0 as *mut osip_via_t;
    let mut i: libc::c_int = 0;
    *transaction = 0 as *mut osip_transaction_t;
    if request.is_null() {
        return -(2 as libc::c_int);
    }
    if ((*request).call_id).is_null() {
        return -(2 as libc::c_int);
    }
    if ((*(*request).call_id).number).is_null() {
        return -(2 as libc::c_int);
    }
    *transaction = (if osip_malloc_func.is_some() {
        osip_malloc_func
            .unwrap()(::std::mem::size_of::<osip_transaction_t>() as libc::c_ulong)
    } else {
        malloc(::std::mem::size_of::<osip_transaction_t>() as libc::c_ulong)
    }) as *mut osip_transaction_t;
    if (*transaction).is_null() {
        return -(4 as libc::c_int);
    }
    memset(
        *transaction as *mut libc::c_void,
        0 as libc::c_int,
        ::std::mem::size_of::<osip_transaction_t>() as libc::c_ulong,
    );
    (**transaction).birth_time = osip_getsystemtime(0 as *mut time_t);
    osip_gettimeofday(&mut (**transaction).created_time, 0 as *mut libc::c_void);
    osip_id_mutex_lock(osip);
    let fresh0 = (*osip).transactionid;
    (*osip).transactionid = (*osip).transactionid + 1;
    (**transaction).transactionid = fresh0;
    osip_id_mutex_unlock(osip);
    osip_trace(
        b"osip_transaction.c\0" as *const u8 as *const libc::c_char,
        143 as libc::c_int,
        TRACE_LEVEL5,
        0 as *mut FILE,
        b"allocating transaction resource %i %s\n\0" as *const u8 as *const libc::c_char,
        (**transaction).transactionid,
        (*(*request).call_id).number,
    );
    (**transaction).ctx_type = ctx_type;
    (**transaction).ict_context = 0 as *mut osip_ict_t;
    (**transaction).ist_context = 0 as *mut osip_ist_t;
    (**transaction).nict_context = 0 as *mut osip_nict_t;
    (**transaction).nist_context = 0 as *mut osip_nist_t;
    (**transaction).config = osip as *mut libc::c_void;
    topvia = osip_list_get(&mut (*request).vias, 0 as libc::c_int) as *mut osip_via_t;
    if topvia.is_null() {
        osip_transaction_free(*transaction);
        *transaction = 0 as *mut osip_transaction_t;
        return -(5 as libc::c_int);
    }
    i = __osip_transaction_set_topvia(*transaction, topvia);
    if i != 0 as libc::c_int {
        osip_transaction_free(*transaction);
        *transaction = 0 as *mut osip_transaction_t;
        return i;
    }
    i = __osip_transaction_set_from(*transaction, (*request).from);
    if i != 0 as libc::c_int {
        osip_transaction_free(*transaction);
        *transaction = 0 as *mut osip_transaction_t;
        return i;
    }
    i = __osip_transaction_set_to(*transaction, (*request).to);
    if i != 0 as libc::c_int {
        osip_transaction_free(*transaction);
        *transaction = 0 as *mut osip_transaction_t;
        return i;
    }
    i = __osip_transaction_set_call_id(*transaction, (*request).call_id);
    if i != 0 as libc::c_int {
        osip_transaction_free(*transaction);
        *transaction = 0 as *mut osip_transaction_t;
        return i;
    }
    i = __osip_transaction_set_cseq(*transaction, (*request).cseq);
    if i != 0 as libc::c_int {
        osip_transaction_free(*transaction);
        *transaction = 0 as *mut osip_transaction_t;
        return i;
    }
    (**transaction).orig_request = 0 as *mut osip_message_t;
    (**transaction)
        .transactionff = (if osip_malloc_func.is_some() {
        osip_malloc_func.unwrap()(::std::mem::size_of::<osip_fifo_t>() as libc::c_ulong)
    } else {
        malloc(::std::mem::size_of::<osip_fifo_t>() as libc::c_ulong)
    }) as *mut osip_fifo_t;
    if ((**transaction).transactionff).is_null() {
        osip_transaction_free(*transaction);
        *transaction = 0 as *mut osip_transaction_t;
        return -(4 as libc::c_int);
    }
    osip_fifo_init((**transaction).transactionff);
    if ctx_type as libc::c_uint == ICT as libc::c_int as libc::c_uint {
        (**transaction).state = ICT_PRE_CALLING;
        i = __osip_ict_init(&mut (**transaction).ict_context, osip, request);
        if i != 0 as libc::c_int {
            osip_transaction_free(*transaction);
            *transaction = 0 as *mut osip_transaction_t;
            return i;
        }
        __osip_add_ict(osip, *transaction);
    } else if ctx_type as libc::c_uint == IST as libc::c_int as libc::c_uint {
        (**transaction).state = IST_PRE_PROCEEDING;
        i = __osip_ist_init(&mut (**transaction).ist_context, osip, request);
        if i != 0 as libc::c_int {
            osip_transaction_free(*transaction);
            *transaction = 0 as *mut osip_transaction_t;
            return i;
        }
        __osip_add_ist(osip, *transaction);
    } else if ctx_type as libc::c_uint == NICT as libc::c_int as libc::c_uint {
        (**transaction).state = NICT_PRE_TRYING;
        i = __osip_nict_init(&mut (**transaction).nict_context, osip, request);
        if i != 0 as libc::c_int {
            osip_transaction_free(*transaction);
            *transaction = 0 as *mut osip_transaction_t;
            return i;
        }
        __osip_add_nict(osip, *transaction);
    } else {
        (**transaction).state = NIST_PRE_TRYING;
        i = __osip_nist_init(&mut (**transaction).nist_context, osip, request);
        if i != 0 as libc::c_int {
            osip_transaction_free(*transaction);
            *transaction = 0 as *mut osip_transaction_t;
            return i;
        }
        __osip_add_nist(osip, *transaction);
    }
    return 0 as libc::c_int;
}
pub unsafe extern "C" fn osip_transaction_free(
    mut transaction: *mut osip_transaction_t,
) -> libc::c_int {
    let mut i: libc::c_int = 0;
    if transaction.is_null() {
        return -(2 as libc::c_int);
    }
    i = osip_remove_transaction((*transaction).config as *mut osip_t, transaction);
    if i != 0 as libc::c_int {
        osip_trace(
            b"osip_transaction.c\0" as *const u8 as *const libc::c_char,
            293 as libc::c_int,
            TRACE_LEVEL7,
            0 as *mut FILE,
            b"transaction already removed from list %i!\n\0" as *const u8
                as *const libc::c_char,
            (*transaction).transactionid,
        );
    }
    return osip_transaction_free2(transaction);
}
pub unsafe extern "C" fn osip_transaction_free2(
    mut transaction: *mut osip_transaction_t,
) -> libc::c_int {
    let mut evt: *mut osip_event_t = 0 as *mut osip_event_t;
    if transaction.is_null() {
        return -(2 as libc::c_int);
    }
    if !((*transaction).orig_request).is_null()
        && !((*(*transaction).orig_request).call_id).is_null()
        && !((*(*(*transaction).orig_request).call_id).number).is_null()
    {
        osip_trace(
            b"osip_transaction.c\0" as *const u8 as *const libc::c_char,
            308 as libc::c_int,
            TRACE_LEVEL5,
            0 as *mut FILE,
            b"free transaction resource %i %s\n\0" as *const u8 as *const libc::c_char,
            (*transaction).transactionid,
            (*(*(*transaction).orig_request).call_id).number,
        );
    }
    if (*transaction).ctx_type as libc::c_uint == ICT as libc::c_int as libc::c_uint {
        __osip_ict_free((*transaction).ict_context);
    } else if (*transaction).ctx_type as libc::c_uint
        == IST as libc::c_int as libc::c_uint
    {
        __osip_ist_free((*transaction).ist_context);
    } else if (*transaction).ctx_type as libc::c_uint
        == NICT as libc::c_int as libc::c_uint
    {
        __osip_nict_free((*transaction).nict_context);
    } else {
        __osip_nist_free((*transaction).nist_context);
    }
    if !((*transaction).transactionff).is_null() {
        evt = osip_fifo_tryget((*transaction).transactionff) as *mut osip_event_t;
        while !evt.is_null() {
            osip_message_free((*evt).sip);
            if !evt.is_null() {
                if osip_free_func.is_some() {
                    osip_free_func.unwrap()(evt as *mut libc::c_void);
                } else {
                    free(evt as *mut libc::c_void);
                }
            }
            evt = osip_fifo_tryget((*transaction).transactionff) as *mut osip_event_t;
        }
        osip_fifo_free((*transaction).transactionff);
    }
    osip_message_free((*transaction).orig_request);
    osip_message_free((*transaction).last_response);
    osip_message_free((*transaction).ack);
    osip_via_free((*transaction).topvia);
    osip_from_free((*transaction).from);
    osip_to_free((*transaction).to);
    osip_call_id_free((*transaction).callid);
    osip_cseq_free((*transaction).cseq);
    if !transaction.is_null() {
        if osip_free_func.is_some() {
            osip_free_func.unwrap()(transaction as *mut libc::c_void);
        } else {
            free(transaction as *mut libc::c_void);
        }
    }
    return 0 as libc::c_int;
}
pub unsafe extern "C" fn osip_transaction_add_event(
    mut transaction: *mut osip_transaction_t,
    mut evt: *mut osip_event_t,
) -> libc::c_int {
    if evt.is_null() {
        return -(2 as libc::c_int);
    }
    if transaction.is_null() {
        return -(2 as libc::c_int);
    }
    (*evt).transactionid = (*transaction).transactionid;
    osip_fifo_add((*transaction).transactionff, evt as *mut libc::c_void);
    return 0 as libc::c_int;
}
pub unsafe extern "C" fn osip_transaction_execute(
    mut transaction: *mut osip_transaction_t,
    mut evt: *mut osip_event_t,
) -> libc::c_int {
    let mut statemachine: *mut osip_statemachine_t = 0 as *mut osip_statemachine_t;
    if (*evt).type_0 as libc::c_uint == KILL_TRANSACTION as libc::c_int as libc::c_uint {
        if !evt.is_null() {
            if osip_free_func.is_some() {
                osip_free_func.unwrap()(evt as *mut libc::c_void);
            } else {
                free(evt as *mut libc::c_void);
            }
        }
        return 0 as libc::c_int;
    }
    osip_trace(
        b"osip_transaction.c\0" as *const u8 as *const libc::c_char,
        377 as libc::c_int,
        TRACE_LEVEL7,
        0 as *mut FILE,
        b"sipevent tr->transactionid: %i\n\0" as *const u8 as *const libc::c_char,
        (*transaction).transactionid,
    );
    osip_trace(
        b"osip_transaction.c\0" as *const u8 as *const libc::c_char,
        378 as libc::c_int,
        TRACE_LEVEL7,
        0 as *mut FILE,
        b"sipevent tr->state: %i\n\0" as *const u8 as *const libc::c_char,
        (*transaction).state as libc::c_uint,
    );
    osip_trace(
        b"osip_transaction.c\0" as *const u8 as *const libc::c_char,
        379 as libc::c_int,
        TRACE_LEVEL7,
        0 as *mut FILE,
        b"sipevent evt->type: %i\n\0" as *const u8 as *const libc::c_char,
        (*evt).type_0 as libc::c_uint,
    );
    osip_trace(
        b"osip_transaction.c\0" as *const u8 as *const libc::c_char,
        380 as libc::c_int,
        TRACE_LEVEL7,
        0 as *mut FILE,
        b"sipevent evt->sip: %x\n\0" as *const u8 as *const libc::c_char,
        (*evt).sip,
    );
    if (*transaction).ctx_type as libc::c_uint == ICT as libc::c_int as libc::c_uint {
        statemachine = &mut ict_fsm;
    } else if (*transaction).ctx_type as libc::c_uint
        == IST as libc::c_int as libc::c_uint
    {
        statemachine = &mut ist_fsm;
    } else if (*transaction).ctx_type as libc::c_uint
        == NICT as libc::c_int as libc::c_uint
    {
        statemachine = &mut nict_fsm;
    } else {
        statemachine = &mut nist_fsm;
    }
    if 0 as libc::c_int
        != fsm_callmethod(
            (*evt).type_0,
            (*transaction).state,
            statemachine,
            evt as *mut libc::c_void,
            transaction as *mut libc::c_void,
        )
    {
        osip_trace(
            b"osip_transaction.c\0" as *const u8 as *const libc::c_char,
            395 as libc::c_int,
            TRACE_LEVEL6,
            0 as *mut FILE,
            b"USELESS event!\n\0" as *const u8 as *const libc::c_char,
        );
        if (*evt).type_0 as libc::c_uint >= RCV_REQINVITE as libc::c_int as libc::c_uint
            && (*evt).type_0 as libc::c_uint
                <= SND_STATUS_3456XX as libc::c_int as libc::c_uint
        {
            if !((*evt).sip).is_null() {
                osip_message_free((*evt).sip);
            }
        }
    } else {
        osip_trace(
            b"osip_transaction.c\0" as *const u8 as *const libc::c_char,
            405 as libc::c_int,
            TRACE_LEVEL7,
            0 as *mut FILE,
            b"sipevent evt: method called!\n\0" as *const u8 as *const libc::c_char,
        );
    }
    if !evt.is_null() {
        if osip_free_func.is_some() {
            osip_free_func.unwrap()(evt as *mut libc::c_void);
        } else {
            free(evt as *mut libc::c_void);
        }
    }
    return 1 as libc::c_int;
}
pub unsafe extern "C" fn osip_transaction_get_destination(
    mut transaction: *mut osip_transaction_t,
    mut ip: *mut *mut libc::c_char,
    mut port: *mut libc::c_int,
) -> libc::c_int {
    *ip = 0 as *mut libc::c_char;
    *port = 0 as libc::c_int;
    if transaction.is_null() {
        return -(2 as libc::c_int);
    }
    if !((*transaction).ict_context).is_null() {
        *ip = (*(*transaction).ict_context).destination;
        *port = (*(*transaction).ict_context).port;
        return 0 as libc::c_int;
    } else if !((*transaction).nict_context).is_null() {
        *ip = (*(*transaction).nict_context).destination;
        *port = (*(*transaction).nict_context).port;
        return 0 as libc::c_int;
    }
    return -(1 as libc::c_int);
}
pub unsafe extern "C" fn osip_transaction_set_srv_record(
    mut transaction: *mut osip_transaction_t,
    mut record: *mut osip_srv_record_t,
) -> libc::c_int {
    if transaction.is_null() {
        return -(2 as libc::c_int);
    }
    memcpy(
        &mut (*transaction).record as *mut osip_srv_record_t as *mut libc::c_void,
        record as *const libc::c_void,
        ::std::mem::size_of::<osip_srv_record_t>() as libc::c_ulong,
    );
    return 0 as libc::c_int;
}
pub unsafe extern "C" fn osip_transaction_set_naptr_record(
    mut transaction: *mut osip_transaction_t,
    mut record: *mut osip_naptr_t,
) -> libc::c_int {
    if transaction.is_null() {
        return -(2 as libc::c_int);
    }
    (*transaction).naptr_record = record;
    return 0 as libc::c_int;
}
pub unsafe extern "C" fn osip_transaction_set_your_instance(
    mut transaction: *mut osip_transaction_t,
    mut ptr: *mut libc::c_void,
) -> libc::c_int {
    if transaction.is_null() {
        return -(2 as libc::c_int);
    }
    (*transaction).reserved1 = ptr;
    return 0 as libc::c_int;
}
pub unsafe extern "C" fn osip_transaction_set_reserved1(
    mut transaction: *mut osip_transaction_t,
    mut ptr: *mut libc::c_void,
) -> libc::c_int {
    if transaction.is_null() {
        return -(2 as libc::c_int);
    }
    (*transaction).reserved1 = ptr;
    return 0 as libc::c_int;
}
pub unsafe extern "C" fn osip_transaction_set_reserved2(
    mut transaction: *mut osip_transaction_t,
    mut ptr: *mut libc::c_void,
) -> libc::c_int {
    if transaction.is_null() {
        return -(2 as libc::c_int);
    }
    (*transaction).reserved2 = ptr;
    return 0 as libc::c_int;
}
pub unsafe extern "C" fn osip_transaction_set_reserved3(
    mut transaction: *mut osip_transaction_t,
    mut ptr: *mut libc::c_void,
) -> libc::c_int {
    if transaction.is_null() {
        return -(2 as libc::c_int);
    }
    (*transaction).reserved3 = ptr;
    return 0 as libc::c_int;
}
pub unsafe extern "C" fn osip_transaction_set_reserved4(
    mut transaction: *mut osip_transaction_t,
    mut ptr: *mut libc::c_void,
) -> libc::c_int {
    if transaction.is_null() {
        return -(2 as libc::c_int);
    }
    (*transaction).reserved4 = ptr;
    return 0 as libc::c_int;
}
pub unsafe extern "C" fn osip_transaction_set_reserved5(
    mut transaction: *mut osip_transaction_t,
    mut ptr: *mut libc::c_void,
) -> libc::c_int {
    if transaction.is_null() {
        return -(2 as libc::c_int);
    }
    (*transaction).reserved5 = ptr;
    return 0 as libc::c_int;
}
pub unsafe extern "C" fn osip_transaction_set_reserved6(
    mut transaction: *mut osip_transaction_t,
    mut ptr: *mut libc::c_void,
) -> libc::c_int {
    if transaction.is_null() {
        return -(2 as libc::c_int);
    }
    (*transaction).reserved6 = ptr;
    return 0 as libc::c_int;
}
pub unsafe extern "C" fn osip_transaction_get_your_instance(
    mut transaction: *mut osip_transaction_t,
) -> *mut libc::c_void {
    if transaction.is_null() {
        return 0 as *mut libc::c_void;
    }
    return (*transaction).reserved1;
}
pub unsafe extern "C" fn osip_transaction_get_reserved1(
    mut transaction: *mut osip_transaction_t,
) -> *mut libc::c_void {
    if transaction.is_null() {
        return 0 as *mut libc::c_void;
    }
    return (*transaction).reserved1;
}
pub unsafe extern "C" fn osip_transaction_get_reserved2(
    mut transaction: *mut osip_transaction_t,
) -> *mut libc::c_void {
    if transaction.is_null() {
        return 0 as *mut libc::c_void;
    }
    return (*transaction).reserved2;
}
pub unsafe extern "C" fn osip_transaction_get_reserved3(
    mut transaction: *mut osip_transaction_t,
) -> *mut libc::c_void {
    if transaction.is_null() {
        return 0 as *mut libc::c_void;
    }
    return (*transaction).reserved3;
}
pub unsafe extern "C" fn osip_transaction_get_reserved4(
    mut transaction: *mut osip_transaction_t,
) -> *mut libc::c_void {
    if transaction.is_null() {
        return 0 as *mut libc::c_void;
    }
    return (*transaction).reserved4;
}
pub unsafe extern "C" fn osip_transaction_get_reserved5(
    mut transaction: *mut osip_transaction_t,
) -> *mut libc::c_void {
    if transaction.is_null() {
        return 0 as *mut libc::c_void;
    }
    return (*transaction).reserved5;
}
pub unsafe extern "C" fn osip_transaction_get_reserved6(
    mut transaction: *mut osip_transaction_t,
) -> *mut libc::c_void {
    if transaction.is_null() {
        return 0 as *mut libc::c_void;
    }
    return (*transaction).reserved6;
}
pub unsafe extern "C" fn __osip_transaction_set_state(
    mut transaction: *mut osip_transaction_t,
    mut state: state_t,
) -> libc::c_int {
    if transaction.is_null() {
        return -(2 as libc::c_int);
    }
    (*transaction).state = state;
    return 0 as libc::c_int;
}
pub unsafe extern "C" fn osip_transaction_set_in_socket(
    mut transaction: *mut osip_transaction_t,
    mut sock: libc::c_int,
) -> libc::c_int {
    if transaction.is_null() {
        return -(2 as libc::c_int);
    }
    (*transaction).in_socket = sock;
    return 0 as libc::c_int;
}
pub unsafe extern "C" fn osip_transaction_set_out_socket(
    mut transaction: *mut osip_transaction_t,
    mut sock: libc::c_int,
) -> libc::c_int {
    if transaction.is_null() {
        return -(2 as libc::c_int);
    }
    (*transaction).out_socket = sock;
    return 0 as libc::c_int;
}
pub unsafe extern "C" fn __osip_transaction_matching_response_osip_to_xict_17_1_3(
    mut tr: *mut osip_transaction_t,
    mut response: *mut osip_message_t,
) -> libc::c_int {
    let mut b_request: *mut osip_generic_param_t = 0 as *mut osip_generic_param_t;
    let mut b_response: *mut osip_generic_param_t = 0 as *mut osip_generic_param_t;
    let mut topvia_response: *mut osip_via_t = 0 as *mut osip_via_t;
    if tr.is_null() || ((*tr).ict_context).is_null() && ((*tr).nict_context).is_null()
        || response.is_null() || ((*response).cseq).is_null()
        || ((*(*response).cseq).method).is_null()
    {
        return -(2 as libc::c_int);
    }
    topvia_response = osip_list_get(&mut (*response).vias, 0 as libc::c_int)
        as *mut osip_via_t;
    if topvia_response.is_null() {
        osip_trace(
            b"osip_transaction.c\0" as *const u8 as *const libc::c_char,
            592 as libc::c_int,
            TRACE_LEVEL2,
            0 as *mut FILE,
            b"Remote UA is not compliant: missing a Via header!\n\0" as *const u8
                as *const libc::c_char,
        );
        return -(5 as libc::c_int);
    }
    osip_uri_param_get_byname(
        &mut (*(*tr).topvia).via_params,
        b"branch\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        &mut b_request,
    );
    if b_request.is_null() {
        osip_trace(
            b"osip_transaction.c\0" as *const u8 as *const libc::c_char,
            599 as libc::c_int,
            TRACE_LEVEL1,
            0 as *mut FILE,
            b"You created a transaction without any branch! THIS IS NOT ALLOWED\n\0"
                as *const u8 as *const libc::c_char,
        );
        return -(5 as libc::c_int);
    }
    osip_uri_param_get_byname(
        &mut (*topvia_response).via_params,
        b"branch\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        &mut b_response,
    );
    if b_response.is_null() {
        osip_trace(
            b"osip_transaction.c\0" as *const u8 as *const libc::c_char,
            638 as libc::c_int,
            TRACE_LEVEL1,
            0 as *mut FILE,
            b"Remote UA is not compliant: missing a branch parameter in  Via header!\n\0"
                as *const u8 as *const libc::c_char,
        );
        return -(5 as libc::c_int);
    }
    if ((*b_request).gvalue).is_null() || ((*b_response).gvalue).is_null() {
        osip_trace(
            b"osip_transaction.c\0" as *const u8 as *const libc::c_char,
            644 as libc::c_int,
            TRACE_LEVEL1,
            0 as *mut FILE,
            b"Remote UA is not compliant: missing a branch parameter in  Via header!\n\0"
                as *const u8 as *const libc::c_char,
        );
        return -(5 as libc::c_int);
    }
    if 0 as libc::c_int != strcmp((*b_request).gvalue, (*b_response).gvalue) {
        return -(1 as libc::c_int);
    }
    if 0 as libc::c_int == strcmp((*(*response).cseq).method, (*(*tr).cseq).method) {
        return 0 as libc::c_int;
    }
    return -(1 as libc::c_int);
}
pub unsafe extern "C" fn __osip_transaction_matching_request_osip_to_xist_17_2_3(
    mut tr: *mut osip_transaction_t,
    mut request: *mut osip_message_t,
) -> libc::c_int {
    let mut b_origrequest: *mut osip_generic_param_t = 0 as *mut osip_generic_param_t;
    let mut b_request: *mut osip_generic_param_t = 0 as *mut osip_generic_param_t;
    let mut topvia_request: *mut osip_via_t = 0 as *mut osip_via_t;
    let mut length_br: size_t = 0;
    let mut length_br2: size_t = 0;
    if tr.is_null() || ((*tr).ist_context).is_null() && ((*tr).nist_context).is_null()
        || request.is_null() || ((*request).cseq).is_null()
        || ((*(*request).cseq).method).is_null()
    {
        return -(2 as libc::c_int);
    }
    topvia_request = osip_list_get(&mut (*request).vias, 0 as libc::c_int)
        as *mut osip_via_t;
    if topvia_request.is_null() {
        osip_trace(
            b"osip_transaction.c\0" as *const u8 as *const libc::c_char,
            690 as libc::c_int,
            TRACE_LEVEL2,
            0 as *mut FILE,
            b"Remote UA is not compliant: missing a Via header!\n\0" as *const u8
                as *const libc::c_char,
        );
        return -(5 as libc::c_int);
    }
    osip_uri_param_get_byname(
        &mut (*topvia_request).via_params,
        b"branch\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        &mut b_request,
    );
    osip_uri_param_get_byname(
        &mut (*(*tr).topvia).via_params,
        b"branch\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        &mut b_origrequest,
    );
    if b_origrequest.is_null() && !b_request.is_null()
        || !b_origrequest.is_null() && b_request.is_null()
    {
        return -(5 as libc::c_int);
    }
    if !b_origrequest.is_null() && !b_request.is_null() {
        if ((*b_origrequest).gvalue).is_null() {
            return -(1 as libc::c_int);
        }
        if ((*b_request).gvalue).is_null() {
            return -(1 as libc::c_int);
        }
        length_br = strlen((*b_origrequest).gvalue);
        length_br2 = strlen((*b_request).gvalue);
        if length_br != length_br2 {
            return -(1 as libc::c_int);
        }
        if 0 as libc::c_int
            == strncmp(
                (*b_origrequest).gvalue,
                b"z9hG4bK\0" as *const u8 as *const libc::c_char,
                7 as libc::c_int as libc::c_ulong,
            )
            && 0 as libc::c_int
                == strncmp(
                    (*b_request).gvalue,
                    b"z9hG4bK\0" as *const u8 as *const libc::c_char,
                    7 as libc::c_int as libc::c_ulong,
                )
        {
            if 0 as libc::c_int != strcmp((*b_origrequest).gvalue, (*b_request).gvalue) {
                return -(1 as libc::c_int);
            }
            let mut b_port: *mut libc::c_char = osip_via_get_port(topvia_request);
            let mut b_origport: *mut libc::c_char = osip_via_get_port((*tr).topvia);
            let mut b_host: *mut libc::c_char = osip_via_get_host(topvia_request);
            let mut b_orighost: *mut libc::c_char = osip_via_get_host((*tr).topvia);
            if b_host.is_null() || b_orighost.is_null() {
                return -(1 as libc::c_int);
            }
            if 0 as libc::c_int != strcmp(b_orighost, b_host) {
                return -(1 as libc::c_int);
            }
            if !b_port.is_null() && b_origport.is_null()
                && 0 as libc::c_int
                    != strcmp(b_port, b"5060\0" as *const u8 as *const libc::c_char)
            {
                return -(1 as libc::c_int)
            } else if !b_origport.is_null() && b_port.is_null()
                && 0 as libc::c_int
                    != strcmp(b_origport, b"5060\0" as *const u8 as *const libc::c_char)
            {
                return -(1 as libc::c_int)
            } else if !b_origport.is_null() && !b_port.is_null()
                && 0 as libc::c_int != strcmp(b_origport, b_port)
            {
                return -(1 as libc::c_int)
            }
            if !(0 as libc::c_int
                == strcmp(
                    (*(*tr).cseq).method,
                    b"INVITE\0" as *const u8 as *const libc::c_char,
                )
                && 0 as libc::c_int
                    == strcmp(
                        (*(*request).cseq).method,
                        b"ACK\0" as *const u8 as *const libc::c_char,
                    ))
                && 0 as libc::c_int
                    != strcmp((*(*tr).cseq).method, (*(*request).cseq).method)
            {
                return -(1 as libc::c_int);
            }
            return 0 as libc::c_int;
        }
    }
    if 0 as libc::c_int != osip_call_id_match((*tr).callid, (*request).call_id) {
        return -(1 as libc::c_int);
    }
    if (*request).status_code == 0 as libc::c_int
        && 0 as libc::c_int
            == strcmp(
                (*request).sip_method,
                b"ACK\0" as *const u8 as *const libc::c_char,
            )
    {
        let mut tag_from1: *mut osip_generic_param_t = 0 as *mut osip_generic_param_t;
        let mut tag_from2: *mut osip_generic_param_t = 0 as *mut osip_generic_param_t;
        osip_uri_param_get_byname(
            &mut (*(*tr).to).gen_params,
            b"tag\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            &mut tag_from1,
        );
        osip_uri_param_get_byname(
            &mut (*(*request).to).gen_params,
            b"tag\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            &mut tag_from2,
        );
        if !(tag_from1.is_null() && !tag_from2.is_null()) {
            if !tag_from1.is_null() && tag_from2.is_null() {
                return -(1 as libc::c_int)
            } else if 0 as libc::c_int != osip_to_tag_match((*tr).to, (*request).to) {
                return -(1 as libc::c_int)
            }
        }
    } else {
        if ((*tr).orig_request).is_null() || ((*(*tr).orig_request).to).is_null() {
            return -(1 as libc::c_int);
        }
        if 0 as libc::c_int != osip_to_tag_match((*(*tr).orig_request).to, (*request).to)
        {
            return -(1 as libc::c_int);
        }
    }
    if 0 as libc::c_int != osip_from_tag_match((*tr).from, (*request).from) {
        return -(1 as libc::c_int);
    }
    if 0 as libc::c_int != osip_cseq_match((*tr).cseq, (*request).cseq) {
        return -(1 as libc::c_int);
    }
    if 0 as libc::c_int != osip_via_match((*tr).topvia, topvia_request) {
        return -(1 as libc::c_int);
    }
    return 0 as libc::c_int;
}
pub unsafe extern "C" fn __osip_transaction_need_timer_x_event(
    mut xixt: *mut libc::c_void,
    mut timer: *mut timeval,
    mut cond_state: libc::c_int,
    mut transactionid: libc::c_int,
    mut TIMER_VAL: libc::c_int,
) -> *mut osip_event_t {
    let mut now: timeval = timeval { tv_sec: 0, tv_usec: 0 };
    osip_gettimeofday(&mut now, 0 as *mut libc::c_void);
    if xixt.is_null() {
        return 0 as *mut osip_event_t;
    }
    if cond_state != 0 {
        if (*timer).tv_sec == -(1 as libc::c_int) as libc::c_long {
            return 0 as *mut osip_event_t;
        }
        if if now.tv_sec == (*timer).tv_sec {
            (now.tv_usec >= (*timer).tv_usec) as libc::c_int
        } else {
            (now.tv_sec >= (*timer).tv_sec) as libc::c_int
        } != 0
        {
            return __osip_event_new(TIMER_VAL as type_t, transactionid);
        }
    }
    return 0 as *mut osip_event_t;
}
pub unsafe extern "C" fn __osip_transaction_snd_xxx(
    mut ist: *mut osip_transaction_t,
    mut msg: *mut osip_message_t,
) -> libc::c_int {
    let mut osip: *mut osip_t = (*ist).config as *mut osip_t;
    let mut via: *mut osip_via_t = 0 as *mut osip_via_t;
    let mut host: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut port: libc::c_int = 0;
    let mut maddr: *mut osip_generic_param_t = 0 as *mut osip_generic_param_t;
    let mut received: *mut osip_generic_param_t = 0 as *mut osip_generic_param_t;
    let mut rport: *mut osip_generic_param_t = 0 as *mut osip_generic_param_t;
    via = osip_list_get(&mut (*msg).vias, 0 as libc::c_int) as *mut osip_via_t;
    if via.is_null() {
        return -(5 as libc::c_int);
    }
    osip_uri_param_get_byname(
        &mut (*via).via_params,
        b"maddr\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        &mut maddr,
    );
    osip_uri_param_get_byname(
        &mut (*via).via_params,
        b"received\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        &mut received,
    );
    osip_uri_param_get_byname(
        &mut (*via).via_params,
        b"rport\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        &mut rport,
    );
    if !maddr.is_null() {
        host = (*maddr).gvalue;
    } else if !received.is_null() {
        host = (*received).gvalue;
    } else {
        host = (*via).host;
    }
    if rport.is_null() || ((*rport).gvalue).is_null() {
        if !((*via).port).is_null() {
            port = osip_atoi((*via).port);
        } else {
            port = 5060 as libc::c_int;
        }
    } else {
        port = osip_atoi((*rport).gvalue);
    }
    return ((*osip).cb_send_message).unwrap()(ist, msg, host, port, (*ist).out_socket);
}
