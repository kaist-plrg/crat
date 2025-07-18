use ::libc;
extern "C" {
    pub type osip_sem;
    pub type osip_mutex;
    fn sprintf(_: *mut libc::c_char, _: *const libc::c_char, _: ...) -> libc::c_int;
    fn malloc(_: libc::c_ulong) -> *mut libc::c_void;
    fn free(__ptr: *mut libc::c_void);
    fn osip_list_clone(
        src: *const osip_list_t,
        dst: *mut osip_list_t,
        clone_func: Option::<
            unsafe extern "C" fn(
                *mut libc::c_void,
                *mut *mut libc::c_void,
            ) -> libc::c_int,
        >,
    ) -> libc::c_int;
    fn osip_list_size(li: *const osip_list_t) -> libc::c_int;
    fn osip_list_eol(li: *const osip_list_t, pos: libc::c_int) -> libc::c_int;
    fn osip_list_add(
        li: *mut osip_list_t,
        element: *mut libc::c_void,
        pos: libc::c_int,
    ) -> libc::c_int;
    fn osip_list_get(li: *const osip_list_t, pos: libc::c_int) -> *mut libc::c_void;
    static mut osip_malloc_func: Option::<osip_malloc_func_t>;
    static mut osip_free_func: Option::<osip_free_func_t>;
    fn osip_strcasecmp(s1: *const libc::c_char, s2: *const libc::c_char) -> libc::c_int;
    fn osip_strdup(ch: *const libc::c_char) -> *mut libc::c_char;
    fn osip_uri_param_get_byname(
        url_params: *mut osip_list_t,
        name: *mut libc::c_char,
        dest: *mut *mut osip_uri_param_t,
    ) -> libc::c_int;
    fn osip_uri_clone(url: *const osip_uri_t, dest: *mut *mut osip_uri_t) -> libc::c_int;
    fn osip_authorization_clone(
        header: *const osip_authorization_t,
        dest: *mut *mut osip_authorization_t,
    ) -> libc::c_int;
    fn osip_call_id_clone(
        header: *const osip_call_id_t,
        dest: *mut *mut osip_call_id_t,
    ) -> libc::c_int;
    fn osip_from_clone(
        header: *const osip_from_t,
        dest: *mut *mut osip_from_t,
    ) -> libc::c_int;
    fn osip_cseq_clone(
        header: *const osip_cseq_t,
        dest: *mut *mut osip_cseq_t,
    ) -> libc::c_int;
    fn osip_to_clone(header: *const osip_to_t, dest: *mut *mut osip_to_t) -> libc::c_int;
    fn osip_via_clone(
        header: *const osip_via_t,
        dest: *mut *mut osip_via_t,
    ) -> libc::c_int;
    fn osip_via_get_protocol(header: *mut osip_via_t) -> *mut libc::c_char;
    fn osip_gettimeofday(tp: *mut timeval, tz: *mut libc::c_void) -> libc::c_int;
    fn osip_message_init(sip: *mut *mut osip_message_t) -> libc::c_int;
    fn osip_message_free(sip: *mut osip_message_t);
    fn osip_atoi(number: *const libc::c_char) -> libc::c_int;
    fn add_gettimeofday(atv: *mut timeval, ms: libc::c_int);
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
    fn osip_ict_set_destination(
        ict: *mut osip_ict_t,
        destination: *mut libc::c_char,
        port: libc::c_int,
    ) -> libc::c_int;
    fn __osip_message_callback(
        type_0: libc::c_int,
        _: *mut osip_transaction_t,
        _: *mut osip_message_t,
    );
    fn __osip_kill_transaction_callback(type_0: libc::c_int, _: *mut osip_transaction_t);
    fn __osip_transport_error_callback(
        type_0: libc::c_int,
        _: *mut osip_transaction_t,
        error: libc::c_int,
    );
    fn __osip_transaction_set_state(
        transaction: *mut osip_transaction_t,
        state: state_t,
    ) -> libc::c_int;
}
pub type size_t = libc::c_ulong;
pub type __time_t = libc::c_long;
pub type __suseconds_t = libc::c_long;
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
pub struct osip_authorization {
    pub auth_type: *mut libc::c_char,
    pub username: *mut libc::c_char,
    pub realm: *mut libc::c_char,
    pub nonce: *mut libc::c_char,
    pub uri: *mut libc::c_char,
    pub response: *mut libc::c_char,
    pub digest: *mut libc::c_char,
    pub algorithm: *mut libc::c_char,
    pub cnonce: *mut libc::c_char,
    pub opaque: *mut libc::c_char,
    pub message_qop: *mut libc::c_char,
    pub nonce_count: *mut libc::c_char,
    pub version: *mut libc::c_char,
    pub targetname: *mut libc::c_char,
    pub gssapi_data: *mut libc::c_char,
    pub crand: *mut libc::c_char,
    pub cnum: *mut libc::c_char,
    pub auth_param: *mut libc::c_char,
}
pub type osip_authorization_t = osip_authorization;
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
pub type osip_message_callback_type = libc::c_uint;
pub const OSIP_MESSAGE_CALLBACK_COUNT: osip_message_callback_type = 60;
pub const OSIP_NICT_STATUS_TIMEOUT: osip_message_callback_type = 59;
pub const OSIP_ICT_STATUS_TIMEOUT: osip_message_callback_type = 58;
pub const OSIP_NIST_STATUS_3456XX_SENT_AGAIN: osip_message_callback_type = 57;
pub const OSIP_NIST_STATUS_6XX_SENT: osip_message_callback_type = 56;
pub const OSIP_NIST_STATUS_5XX_SENT: osip_message_callback_type = 55;
pub const OSIP_NIST_STATUS_4XX_SENT: osip_message_callback_type = 54;
pub const OSIP_NIST_STATUS_3XX_SENT: osip_message_callback_type = 53;
pub const OSIP_NIST_STATUS_2XX_SENT_AGAIN: osip_message_callback_type = 52;
pub const OSIP_NIST_STATUS_2XX_SENT: osip_message_callback_type = 51;
pub const OSIP_NIST_STATUS_1XX_SENT: osip_message_callback_type = 50;
pub const OSIP_NIST_REQUEST_RECEIVED_AGAIN: osip_message_callback_type = 49;
pub const OSIP_NIST_UNKNOWN_REQUEST_RECEIVED: osip_message_callback_type = 48;
pub const OSIP_NIST_SUBSCRIBE_RECEIVED: osip_message_callback_type = 47;
pub const OSIP_NIST_NOTIFY_RECEIVED: osip_message_callback_type = 46;
pub const OSIP_NIST_CANCEL_RECEIVED: osip_message_callback_type = 45;
pub const OSIP_NIST_INFO_RECEIVED: osip_message_callback_type = 44;
pub const OSIP_NIST_OPTIONS_RECEIVED: osip_message_callback_type = 43;
pub const OSIP_NIST_BYE_RECEIVED: osip_message_callback_type = 42;
pub const OSIP_NIST_REGISTER_RECEIVED: osip_message_callback_type = 41;
pub const OSIP_NICT_STATUS_3456XX_RECEIVED_AGAIN: osip_message_callback_type = 40;
pub const OSIP_NICT_STATUS_6XX_RECEIVED: osip_message_callback_type = 39;
pub const OSIP_NICT_STATUS_5XX_RECEIVED: osip_message_callback_type = 38;
pub const OSIP_NICT_STATUS_4XX_RECEIVED: osip_message_callback_type = 37;
pub const OSIP_NICT_STATUS_3XX_RECEIVED: osip_message_callback_type = 36;
pub const OSIP_NICT_STATUS_2XX_RECEIVED_AGAIN: osip_message_callback_type = 35;
pub const OSIP_NICT_STATUS_2XX_RECEIVED: osip_message_callback_type = 34;
pub const OSIP_NICT_STATUS_1XX_RECEIVED: osip_message_callback_type = 33;
pub const OSIP_NICT_REQUEST_SENT_AGAIN: osip_message_callback_type = 32;
pub const OSIP_NICT_UNKNOWN_REQUEST_SENT: osip_message_callback_type = 31;
pub const OSIP_NICT_SUBSCRIBE_SENT: osip_message_callback_type = 30;
pub const OSIP_NICT_NOTIFY_SENT: osip_message_callback_type = 29;
pub const OSIP_NICT_CANCEL_SENT: osip_message_callback_type = 28;
pub const OSIP_NICT_INFO_SENT: osip_message_callback_type = 27;
pub const OSIP_NICT_OPTIONS_SENT: osip_message_callback_type = 26;
pub const OSIP_NICT_BYE_SENT: osip_message_callback_type = 25;
pub const OSIP_NICT_REGISTER_SENT: osip_message_callback_type = 24;
pub const OSIP_IST_STATUS_3456XX_SENT_AGAIN: osip_message_callback_type = 23;
pub const OSIP_IST_STATUS_6XX_SENT: osip_message_callback_type = 22;
pub const OSIP_IST_STATUS_5XX_SENT: osip_message_callback_type = 21;
pub const OSIP_IST_STATUS_4XX_SENT: osip_message_callback_type = 20;
pub const OSIP_IST_STATUS_3XX_SENT: osip_message_callback_type = 19;
pub const OSIP_IST_STATUS_2XX_SENT_AGAIN: osip_message_callback_type = 18;
pub const OSIP_IST_STATUS_2XX_SENT: osip_message_callback_type = 17;
pub const OSIP_IST_STATUS_1XX_SENT: osip_message_callback_type = 16;
pub const OSIP_IST_ACK_RECEIVED_AGAIN: osip_message_callback_type = 15;
pub const OSIP_IST_ACK_RECEIVED: osip_message_callback_type = 14;
pub const OSIP_IST_INVITE_RECEIVED_AGAIN: osip_message_callback_type = 13;
pub const OSIP_IST_INVITE_RECEIVED: osip_message_callback_type = 12;
pub const OSIP_ICT_STATUS_3456XX_RECEIVED_AGAIN: osip_message_callback_type = 11;
pub const OSIP_ICT_STATUS_6XX_RECEIVED: osip_message_callback_type = 10;
pub const OSIP_ICT_STATUS_5XX_RECEIVED: osip_message_callback_type = 9;
pub const OSIP_ICT_STATUS_4XX_RECEIVED: osip_message_callback_type = 8;
pub const OSIP_ICT_STATUS_3XX_RECEIVED: osip_message_callback_type = 7;
pub const OSIP_ICT_STATUS_2XX_RECEIVED_AGAIN: osip_message_callback_type = 6;
pub const OSIP_ICT_STATUS_2XX_RECEIVED: osip_message_callback_type = 5;
pub const OSIP_ICT_STATUS_1XX_RECEIVED: osip_message_callback_type = 4;
pub const OSIP_ICT_ACK_SENT_AGAIN: osip_message_callback_type = 3;
pub const OSIP_ICT_ACK_SENT: osip_message_callback_type = 2;
pub const OSIP_ICT_INVITE_SENT_AGAIN: osip_message_callback_type = 1;
pub const OSIP_ICT_INVITE_SENT: osip_message_callback_type = 0;
pub type osip_kill_callback_type = libc::c_uint;
pub const OSIP_KILL_CALLBACK_COUNT: osip_kill_callback_type = 4;
pub const OSIP_NIST_KILL_TRANSACTION: osip_kill_callback_type = 3;
pub const OSIP_NICT_KILL_TRANSACTION: osip_kill_callback_type = 2;
pub const OSIP_IST_KILL_TRANSACTION: osip_kill_callback_type = 1;
pub const OSIP_ICT_KILL_TRANSACTION: osip_kill_callback_type = 0;
pub type osip_transport_error_callback_type = libc::c_uint;
pub const OSIP_TRANSPORT_ERROR_CALLBACK_COUNT: osip_transport_error_callback_type = 4;
pub const OSIP_NIST_TRANSPORT_ERROR: osip_transport_error_callback_type = 3;
pub const OSIP_NICT_TRANSPORT_ERROR: osip_transport_error_callback_type = 2;
pub const OSIP_IST_TRANSPORT_ERROR: osip_transport_error_callback_type = 1;
pub const OSIP_ICT_TRANSPORT_ERROR: osip_transport_error_callback_type = 0;
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
pub type osip_statemachine_t = osip_statemachine;
pub type transition_t = _transition_t;
pub static mut ict_transition: [transition_t; 11] = [transition_t {
    state: ICT_PRE_CALLING,
    type_0: TIMEOUT_A,
    method: None,
    next: 0 as *mut _transition_t,
    parent: 0 as *mut _transition_t,
}; 11];
pub static mut ict_fsm: osip_statemachine_t = unsafe {
    {
        let mut init = osip_statemachine {
            transitions: ict_transition.as_ptr() as *mut _,
        };
        init
    }
};
unsafe extern "C" fn ict_handle_transport_error(
    mut ict: *mut osip_transaction_t,
    mut err: libc::c_int,
) {
    __osip_transport_error_callback(OSIP_ICT_TRANSPORT_ERROR as libc::c_int, ict, err);
    __osip_transaction_set_state(ict, ICT_TERMINATED);
    __osip_kill_transaction_callback(OSIP_ICT_KILL_TRANSACTION as libc::c_int, ict);
}
pub unsafe extern "C" fn ict_snd_invite(
    mut ict: *mut osip_transaction_t,
    mut evt: *mut osip_event_t,
) {
    let mut i: libc::c_int = 0;
    let mut osip: *mut osip_t = (*ict).config as *mut osip_t;
    (*ict).orig_request = (*evt).sip;
    i = ((*osip).cb_send_message)
        .unwrap()(
        ict,
        (*evt).sip,
        (*(*ict).ict_context).destination,
        (*(*ict).ict_context).port,
        (*ict).out_socket,
    );
    if i < 0 as libc::c_int {
        ict_handle_transport_error(ict, i);
        return;
    }
    if i == 0 as libc::c_int {
        let mut via: *mut osip_via_t = 0 as *mut osip_via_t;
        let mut proto: *mut libc::c_char = 0 as *mut libc::c_char;
        i = osip_message_get_via((*ict).orig_request, 0 as libc::c_int, &mut via);
        if i < 0 as libc::c_int {
            ict_handle_transport_error(ict, i);
            return;
        }
        proto = osip_via_get_protocol(via);
        if proto.is_null() {
            ict_handle_transport_error(ict, i);
            return;
        }
        if !(osip_strcasecmp(proto, b"TCP\0" as *const u8 as *const libc::c_char)
            != 0 as libc::c_int
            && osip_strcasecmp(proto, b"TLS\0" as *const u8 as *const libc::c_char)
                != 0 as libc::c_int
            && osip_strcasecmp(proto, b"SCTP\0" as *const u8 as *const libc::c_char)
                != 0 as libc::c_int)
        {
            (*(*ict).ict_context).timer_a_length = -(1 as libc::c_int);
            (*(*ict).ict_context).timer_a_start.tv_sec = -(1 as libc::c_int) as __time_t;
        }
    }
    __osip_message_callback(
        OSIP_ICT_INVITE_SENT as libc::c_int,
        ict,
        (*ict).orig_request,
    );
    __osip_transaction_set_state(ict, ICT_CALLING);
}
pub unsafe extern "C" fn osip_ict_timeout_a_event(
    mut ict: *mut osip_transaction_t,
    mut evt: *mut osip_event_t,
) {
    let mut osip: *mut osip_t = (*ict).config as *mut osip_t;
    let mut i: libc::c_int = 0;
    (*(*ict).ict_context)
        .timer_a_length = (*(*ict).ict_context).timer_a_length * 2 as libc::c_int;
    osip_gettimeofday(&mut (*(*ict).ict_context).timer_a_start, 0 as *mut libc::c_void);
    add_gettimeofday(
        &mut (*(*ict).ict_context).timer_a_start,
        (*(*ict).ict_context).timer_a_length,
    );
    i = ((*osip).cb_send_message)
        .unwrap()(
        ict,
        (*ict).orig_request,
        (*(*ict).ict_context).destination,
        (*(*ict).ict_context).port,
        (*ict).out_socket,
    );
    if i < 0 as libc::c_int {
        ict_handle_transport_error(ict, i);
        return;
    }
    if i == 0 as libc::c_int {
        let mut via: *mut osip_via_t = 0 as *mut osip_via_t;
        let mut proto: *mut libc::c_char = 0 as *mut libc::c_char;
        i = osip_message_get_via((*ict).orig_request, 0 as libc::c_int, &mut via);
        if i < 0 as libc::c_int {
            ict_handle_transport_error(ict, i);
            return;
        }
        proto = osip_via_get_protocol(via);
        if proto.is_null() {
            ict_handle_transport_error(ict, i);
            return;
        }
        if !(osip_strcasecmp(proto, b"TCP\0" as *const u8 as *const libc::c_char)
            != 0 as libc::c_int
            && osip_strcasecmp(proto, b"TLS\0" as *const u8 as *const libc::c_char)
                != 0 as libc::c_int
            && osip_strcasecmp(proto, b"SCTP\0" as *const u8 as *const libc::c_char)
                != 0 as libc::c_int)
        {
            (*(*ict).ict_context).timer_a_length = -(1 as libc::c_int);
            (*(*ict).ict_context).timer_a_start.tv_sec = -(1 as libc::c_int) as __time_t;
        }
    }
    if i == 0 as libc::c_int {
        __osip_message_callback(
            OSIP_ICT_INVITE_SENT_AGAIN as libc::c_int,
            ict,
            (*ict).orig_request,
        );
    }
}
pub unsafe extern "C" fn osip_ict_timeout_b_event(
    mut ict: *mut osip_transaction_t,
    mut evt: *mut osip_event_t,
) {
    (*(*ict).ict_context).timer_b_length = -(1 as libc::c_int);
    (*(*ict).ict_context).timer_b_start.tv_sec = -(1 as libc::c_int) as __time_t;
    if (*ict).out_socket == -(999 as libc::c_int) {
        ict_handle_transport_error(ict, -(1 as libc::c_int));
    } else {
        __osip_message_callback(OSIP_ICT_STATUS_TIMEOUT as libc::c_int, ict, (*evt).sip);
        __osip_transaction_set_state(ict, ICT_TERMINATED);
        __osip_kill_transaction_callback(OSIP_ICT_KILL_TRANSACTION as libc::c_int, ict);
    };
}
pub unsafe extern "C" fn ict_rcv_1xx(
    mut ict: *mut osip_transaction_t,
    mut evt: *mut osip_event_t,
) {
    if !((*ict).last_response).is_null() {
        osip_message_free((*ict).last_response);
    }
    (*ict).last_response = (*evt).sip;
    __osip_message_callback(
        OSIP_ICT_STATUS_1XX_RECEIVED as libc::c_int,
        ict,
        (*evt).sip,
    );
    __osip_transaction_set_state(ict, ICT_PROCEEDING);
}
pub unsafe extern "C" fn ict_rcv_2xx(
    mut ict: *mut osip_transaction_t,
    mut evt: *mut osip_event_t,
) {
    if !((*ict).last_response).is_null() {
        osip_message_free((*ict).last_response);
    }
    (*ict).last_response = (*evt).sip;
    __osip_message_callback(
        OSIP_ICT_STATUS_2XX_RECEIVED as libc::c_int,
        ict,
        (*evt).sip,
    );
    __osip_transaction_set_state(ict, ICT_TERMINATED);
    __osip_kill_transaction_callback(OSIP_ICT_KILL_TRANSACTION as libc::c_int, ict);
}
pub unsafe extern "C" fn ict_create_ack(
    mut ict: *mut osip_transaction_t,
    mut response: *mut osip_message_t,
) -> *mut osip_message_t {
    let mut i: libc::c_int = 0;
    let mut ack: *mut osip_message_t = 0 as *mut osip_message_t;
    i = osip_message_init(&mut ack);
    if i != 0 as libc::c_int {
        return 0 as *mut osip_message_t;
    }
    i = osip_from_clone((*response).from, &mut (*ack).from);
    if i != 0 as libc::c_int {
        osip_message_free(ack);
        return 0 as *mut osip_message_t;
    }
    i = osip_to_clone((*response).to, &mut (*ack).to);
    if i != 0 as libc::c_int {
        osip_message_free(ack);
        return 0 as *mut osip_message_t;
    }
    i = osip_call_id_clone((*response).call_id, &mut (*ack).call_id);
    if i != 0 as libc::c_int {
        osip_message_free(ack);
        return 0 as *mut osip_message_t;
    }
    i = osip_cseq_clone((*response).cseq, &mut (*ack).cseq);
    if i != 0 as libc::c_int {
        osip_message_free(ack);
        return 0 as *mut osip_message_t;
    }
    if !((*(*ack).cseq).method).is_null() {
        if osip_free_func.is_some() {
            osip_free_func.unwrap()((*(*ack).cseq).method as *mut libc::c_void);
        } else {
            free((*(*ack).cseq).method as *mut libc::c_void);
        }
    }
    (*(*ack).cseq).method = osip_strdup(b"ACK\0" as *const u8 as *const libc::c_char);
    if ((*(*ack).cseq).method).is_null() {
        osip_message_free(ack);
        return 0 as *mut osip_message_t;
    }
    (*ack)
        .sip_method = (if osip_malloc_func.is_some() {
        osip_malloc_func.unwrap()(5 as libc::c_int as size_t)
    } else {
        malloc(5 as libc::c_int as libc::c_ulong)
    }) as *mut libc::c_char;
    if ((*ack).sip_method).is_null() {
        osip_message_free(ack);
        return 0 as *mut osip_message_t;
    }
    sprintf((*ack).sip_method, b"ACK\0" as *const u8 as *const libc::c_char);
    (*ack).sip_version = osip_strdup((*(*ict).orig_request).sip_version);
    if ((*ack).sip_version).is_null() {
        osip_message_free(ack);
        return 0 as *mut osip_message_t;
    }
    (*ack).status_code = 0 as libc::c_int;
    (*ack).reason_phrase = 0 as *mut libc::c_char;
    i = osip_uri_clone((*(*ict).orig_request).req_uri, &mut (*ack).req_uri);
    if i != 0 as libc::c_int {
        osip_message_free(ack);
        return 0 as *mut osip_message_t;
    }
    let mut via: *mut osip_via_t = 0 as *mut osip_via_t;
    let mut orig_via: *mut osip_via_t = 0 as *mut osip_via_t;
    osip_message_get_via((*ict).orig_request, 0 as libc::c_int, &mut orig_via);
    if orig_via.is_null() {
        osip_message_free(ack);
        return 0 as *mut osip_message_t;
    }
    i = osip_via_clone(orig_via, &mut via);
    if i != 0 as libc::c_int {
        osip_message_free(ack);
        return 0 as *mut osip_message_t;
    }
    osip_list_add(&mut (*ack).vias, via as *mut libc::c_void, -(1 as libc::c_int));
    let mut pos: libc::c_int = 0 as libc::c_int;
    let mut route: *mut osip_route_t = 0 as *mut osip_route_t;
    let mut orig_route: *mut osip_route_t = 0 as *mut osip_route_t;
    while osip_list_eol(&mut (*(*ict).orig_request).routes, pos) == 0 {
        orig_route = osip_list_get(&mut (*(*ict).orig_request).routes, pos)
            as *mut osip_route_t;
        i = osip_from_clone(orig_route, &mut route);
        if i != 0 as libc::c_int {
            osip_message_free(ack);
            return 0 as *mut osip_message_t;
        }
        osip_list_add(
            &mut (*ack).routes,
            route as *mut libc::c_void,
            -(1 as libc::c_int),
        );
        pos += 1;
        pos;
    }
    if (*response).status_code != 401 as libc::c_int
        && (*response).status_code != 407 as libc::c_int
    {
        if osip_list_size(&mut (*(*ict).orig_request).authorizations) > 0 as libc::c_int
        {
            i = osip_list_clone(
                &mut (*(*ict).orig_request).authorizations,
                &mut (*ack).authorizations,
                ::std::mem::transmute::<
                    Option::<
                        unsafe extern "C" fn(
                            *const osip_authorization_t,
                            *mut *mut osip_authorization_t,
                        ) -> libc::c_int,
                    >,
                    Option::<
                        unsafe extern "C" fn(
                            *mut libc::c_void,
                            *mut *mut libc::c_void,
                        ) -> libc::c_int,
                    >,
                >(
                    Some(
                        osip_authorization_clone
                            as unsafe extern "C" fn(
                                *const osip_authorization_t,
                                *mut *mut osip_authorization_t,
                            ) -> libc::c_int,
                    ),
                ),
            );
            if i != 0 as libc::c_int {
                osip_message_free(ack);
                return 0 as *mut osip_message_t;
            }
        }
        if osip_list_size(&mut (*(*ict).orig_request).proxy_authorizations)
            > 0 as libc::c_int
        {
            i = osip_list_clone(
                &mut (*(*ict).orig_request).proxy_authorizations,
                &mut (*ack).proxy_authorizations,
                ::std::mem::transmute::<
                    Option::<
                        unsafe extern "C" fn(
                            *const osip_authorization_t,
                            *mut *mut osip_authorization_t,
                        ) -> libc::c_int,
                    >,
                    Option::<
                        unsafe extern "C" fn(
                            *mut libc::c_void,
                            *mut *mut libc::c_void,
                        ) -> libc::c_int,
                    >,
                >(
                    Some(
                        osip_authorization_clone
                            as unsafe extern "C" fn(
                                *const osip_authorization_t,
                                *mut *mut osip_authorization_t,
                            ) -> libc::c_int,
                    ),
                ),
            );
            if i != 0 as libc::c_int {
                osip_message_free(ack);
                return 0 as *mut osip_message_t;
            }
        }
    }
    return ack;
}
pub unsafe extern "C" fn ict_rcv_3456xx(
    mut ict: *mut osip_transaction_t,
    mut evt: *mut osip_event_t,
) {
    let mut route: *mut osip_route_t = 0 as *mut osip_route_t;
    let mut i: libc::c_int = 0;
    let mut osip: *mut osip_t = (*ict).config as *mut osip_t;
    if !((*ict).last_response).is_null() {
        osip_message_free((*ict).last_response);
    }
    (*ict).last_response = (*evt).sip;
    if (*ict).state as libc::c_uint != ICT_COMPLETED as libc::c_int as libc::c_uint {
        let mut ack: *mut osip_message_t = ict_create_ack(ict, (*evt).sip);
        (*ict).ack = ack;
        if ((*ict).ack).is_null() {
            __osip_transaction_set_state(ict, ICT_TERMINATED);
            __osip_kill_transaction_callback(
                OSIP_ICT_KILL_TRANSACTION as libc::c_int,
                ict,
            );
            return;
        }
        if ((*(*ict).ict_context).destination).is_null() {
            osip_message_get_route(ack, 0 as libc::c_int, &mut route);
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
                osip_ict_set_destination(
                    (*ict).ict_context,
                    osip_strdup((*(*route).url).host),
                    port,
                );
            } else {
                let mut port_0: libc::c_int = 5060 as libc::c_int;
                let mut maddr_param: *mut osip_uri_param_t = 0 as *mut osip_uri_param_t;
                let mut obr_param: *mut osip_uri_param_t = 0 as *mut osip_uri_param_t;
                let mut obp_param: *mut osip_uri_param_t = 0 as *mut osip_uri_param_t;
                port_0 = 5060 as libc::c_int;
                if !((*(*ack).req_uri).port).is_null() {
                    port_0 = osip_atoi((*(*ack).req_uri).port);
                }
                osip_uri_param_get_byname(
                    &mut (*(*ack).req_uri).url_params,
                    b"x-obr\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
                    &mut obr_param,
                );
                osip_uri_param_get_byname(
                    &mut (*(*ack).req_uri).url_params,
                    b"x-obp\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
                    &mut obp_param,
                );
                osip_uri_param_get_byname(
                    &mut (*(*ack).req_uri).url_params,
                    b"maddr\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
                    &mut maddr_param,
                );
                if !maddr_param.is_null() && !((*maddr_param).gvalue).is_null() {
                    osip_ict_set_destination(
                        (*ict).ict_context,
                        osip_strdup((*maddr_param).gvalue),
                        port_0,
                    );
                } else if !obr_param.is_null() && !((*obr_param).gvalue).is_null()
                    && !obp_param.is_null() && !((*obp_param).gvalue).is_null()
                {
                    osip_ict_set_destination(
                        (*ict).ict_context,
                        osip_strdup((*obr_param).gvalue),
                        osip_atoi((*obp_param).gvalue),
                    );
                } else {
                    osip_ict_set_destination(
                        (*ict).ict_context,
                        osip_strdup((*(*ack).req_uri).host),
                        port_0,
                    );
                }
            }
        }
        i = ((*osip).cb_send_message)
            .unwrap()(
            ict,
            ack,
            (*(*ict).ict_context).destination,
            (*(*ict).ict_context).port,
            (*ict).out_socket,
        );
        if i != 0 as libc::c_int {
            ict_handle_transport_error(ict, i);
            return;
        }
        if (*(*evt).sip).status_code >= 300 as libc::c_int
            && (*(*evt).sip).status_code < 400 as libc::c_int
        {
            __osip_message_callback(
                OSIP_ICT_STATUS_3XX_RECEIVED as libc::c_int,
                ict,
                (*evt).sip,
            );
        } else if (*(*evt).sip).status_code >= 400 as libc::c_int
            && (*(*evt).sip).status_code < 500 as libc::c_int
        {
            __osip_message_callback(
                OSIP_ICT_STATUS_4XX_RECEIVED as libc::c_int,
                ict,
                (*evt).sip,
            );
        } else if (*(*evt).sip).status_code >= 500 as libc::c_int
            && (*(*evt).sip).status_code < 600 as libc::c_int
        {
            __osip_message_callback(
                OSIP_ICT_STATUS_5XX_RECEIVED as libc::c_int,
                ict,
                (*evt).sip,
            );
        } else {
            __osip_message_callback(
                OSIP_ICT_STATUS_6XX_RECEIVED as libc::c_int,
                ict,
                (*evt).sip,
            );
        }
        __osip_message_callback(OSIP_ICT_ACK_SENT as libc::c_int, ict, ack);
    }
    osip_gettimeofday(&mut (*(*ict).ict_context).timer_d_start, 0 as *mut libc::c_void);
    add_gettimeofday(
        &mut (*(*ict).ict_context).timer_d_start,
        (*(*ict).ict_context).timer_d_length,
    );
    __osip_transaction_set_state(ict, ICT_COMPLETED);
}
pub unsafe extern "C" fn osip_ict_timeout_d_event(
    mut ict: *mut osip_transaction_t,
    mut evt: *mut osip_event_t,
) {
    (*(*ict).ict_context).timer_d_length = -(1 as libc::c_int);
    (*(*ict).ict_context).timer_d_start.tv_sec = -(1 as libc::c_int) as __time_t;
    __osip_transaction_set_state(ict, ICT_TERMINATED);
    __osip_kill_transaction_callback(OSIP_ICT_KILL_TRANSACTION as libc::c_int, ict);
}
pub unsafe extern "C" fn ict_retransmit_ack(
    mut ict: *mut osip_transaction_t,
    mut evt: *mut osip_event_t,
) {
    let mut i: libc::c_int = 0;
    let mut osip: *mut osip_t = (*ict).config as *mut osip_t;
    __osip_message_callback(
        OSIP_ICT_STATUS_3456XX_RECEIVED_AGAIN as libc::c_int,
        ict,
        (*evt).sip,
    );
    osip_message_free((*evt).sip);
    i = ((*osip).cb_send_message)
        .unwrap()(
        ict,
        (*ict).ack,
        (*(*ict).ict_context).destination,
        (*(*ict).ict_context).port,
        (*ict).out_socket,
    );
    if i == 0 as libc::c_int {
        __osip_message_callback(OSIP_ICT_ACK_SENT_AGAIN as libc::c_int, ict, (*ict).ack);
        __osip_transaction_set_state(ict, ICT_COMPLETED);
    } else {
        ict_handle_transport_error(ict, i);
    };
}
unsafe extern "C" fn run_static_initializers() {
    ict_transition = [
        {
            let mut init = _transition_t {
                state: ICT_PRE_CALLING,
                type_0: SND_REQINVITE,
                method: ::std::mem::transmute::<
                    Option::<
                        unsafe extern "C" fn(
                            *mut osip_transaction_t,
                            *mut osip_event_t,
                        ) -> (),
                    >,
                    Option::<
                        unsafe extern "C" fn(*mut libc::c_void, *mut libc::c_void) -> (),
                    >,
                >(
                    Some(
                        ict_snd_invite
                            as unsafe extern "C" fn(
                                *mut osip_transaction_t,
                                *mut osip_event_t,
                            ) -> (),
                    ),
                ),
                next: &mut *ict_transition.as_mut_ptr().offset(1 as libc::c_int as isize)
                    as *mut transition_t,
                parent: 0 as *mut _transition_t,
            };
            init
        },
        {
            let mut init = _transition_t {
                state: ICT_CALLING,
                type_0: TIMEOUT_A,
                method: ::std::mem::transmute::<
                    Option::<
                        unsafe extern "C" fn(
                            *mut osip_transaction_t,
                            *mut osip_event_t,
                        ) -> (),
                    >,
                    Option::<
                        unsafe extern "C" fn(*mut libc::c_void, *mut libc::c_void) -> (),
                    >,
                >(
                    Some(
                        osip_ict_timeout_a_event
                            as unsafe extern "C" fn(
                                *mut osip_transaction_t,
                                *mut osip_event_t,
                            ) -> (),
                    ),
                ),
                next: &mut *ict_transition.as_mut_ptr().offset(2 as libc::c_int as isize)
                    as *mut transition_t,
                parent: 0 as *mut _transition_t,
            };
            init
        },
        {
            let mut init = _transition_t {
                state: ICT_CALLING,
                type_0: TIMEOUT_B,
                method: ::std::mem::transmute::<
                    Option::<
                        unsafe extern "C" fn(
                            *mut osip_transaction_t,
                            *mut osip_event_t,
                        ) -> (),
                    >,
                    Option::<
                        unsafe extern "C" fn(*mut libc::c_void, *mut libc::c_void) -> (),
                    >,
                >(
                    Some(
                        osip_ict_timeout_b_event
                            as unsafe extern "C" fn(
                                *mut osip_transaction_t,
                                *mut osip_event_t,
                            ) -> (),
                    ),
                ),
                next: &mut *ict_transition.as_mut_ptr().offset(3 as libc::c_int as isize)
                    as *mut transition_t,
                parent: 0 as *mut _transition_t,
            };
            init
        },
        {
            let mut init = _transition_t {
                state: ICT_CALLING,
                type_0: RCV_STATUS_1XX,
                method: ::std::mem::transmute::<
                    Option::<
                        unsafe extern "C" fn(
                            *mut osip_transaction_t,
                            *mut osip_event_t,
                        ) -> (),
                    >,
                    Option::<
                        unsafe extern "C" fn(*mut libc::c_void, *mut libc::c_void) -> (),
                    >,
                >(
                    Some(
                        ict_rcv_1xx
                            as unsafe extern "C" fn(
                                *mut osip_transaction_t,
                                *mut osip_event_t,
                            ) -> (),
                    ),
                ),
                next: &mut *ict_transition.as_mut_ptr().offset(4 as libc::c_int as isize)
                    as *mut transition_t,
                parent: 0 as *mut _transition_t,
            };
            init
        },
        {
            let mut init = _transition_t {
                state: ICT_CALLING,
                type_0: RCV_STATUS_2XX,
                method: ::std::mem::transmute::<
                    Option::<
                        unsafe extern "C" fn(
                            *mut osip_transaction_t,
                            *mut osip_event_t,
                        ) -> (),
                    >,
                    Option::<
                        unsafe extern "C" fn(*mut libc::c_void, *mut libc::c_void) -> (),
                    >,
                >(
                    Some(
                        ict_rcv_2xx
                            as unsafe extern "C" fn(
                                *mut osip_transaction_t,
                                *mut osip_event_t,
                            ) -> (),
                    ),
                ),
                next: &mut *ict_transition.as_mut_ptr().offset(5 as libc::c_int as isize)
                    as *mut transition_t,
                parent: 0 as *mut _transition_t,
            };
            init
        },
        {
            let mut init = _transition_t {
                state: ICT_CALLING,
                type_0: RCV_STATUS_3456XX,
                method: ::std::mem::transmute::<
                    Option::<
                        unsafe extern "C" fn(
                            *mut osip_transaction_t,
                            *mut osip_event_t,
                        ) -> (),
                    >,
                    Option::<
                        unsafe extern "C" fn(*mut libc::c_void, *mut libc::c_void) -> (),
                    >,
                >(
                    Some(
                        ict_rcv_3456xx
                            as unsafe extern "C" fn(
                                *mut osip_transaction_t,
                                *mut osip_event_t,
                            ) -> (),
                    ),
                ),
                next: &mut *ict_transition.as_mut_ptr().offset(6 as libc::c_int as isize)
                    as *mut transition_t,
                parent: 0 as *mut _transition_t,
            };
            init
        },
        {
            let mut init = _transition_t {
                state: ICT_PROCEEDING,
                type_0: RCV_STATUS_1XX,
                method: ::std::mem::transmute::<
                    Option::<
                        unsafe extern "C" fn(
                            *mut osip_transaction_t,
                            *mut osip_event_t,
                        ) -> (),
                    >,
                    Option::<
                        unsafe extern "C" fn(*mut libc::c_void, *mut libc::c_void) -> (),
                    >,
                >(
                    Some(
                        ict_rcv_1xx
                            as unsafe extern "C" fn(
                                *mut osip_transaction_t,
                                *mut osip_event_t,
                            ) -> (),
                    ),
                ),
                next: &mut *ict_transition.as_mut_ptr().offset(7 as libc::c_int as isize)
                    as *mut transition_t,
                parent: 0 as *mut _transition_t,
            };
            init
        },
        {
            let mut init = _transition_t {
                state: ICT_PROCEEDING,
                type_0: RCV_STATUS_2XX,
                method: ::std::mem::transmute::<
                    Option::<
                        unsafe extern "C" fn(
                            *mut osip_transaction_t,
                            *mut osip_event_t,
                        ) -> (),
                    >,
                    Option::<
                        unsafe extern "C" fn(*mut libc::c_void, *mut libc::c_void) -> (),
                    >,
                >(
                    Some(
                        ict_rcv_2xx
                            as unsafe extern "C" fn(
                                *mut osip_transaction_t,
                                *mut osip_event_t,
                            ) -> (),
                    ),
                ),
                next: &mut *ict_transition.as_mut_ptr().offset(8 as libc::c_int as isize)
                    as *mut transition_t,
                parent: 0 as *mut _transition_t,
            };
            init
        },
        {
            let mut init = _transition_t {
                state: ICT_PROCEEDING,
                type_0: RCV_STATUS_3456XX,
                method: ::std::mem::transmute::<
                    Option::<
                        unsafe extern "C" fn(
                            *mut osip_transaction_t,
                            *mut osip_event_t,
                        ) -> (),
                    >,
                    Option::<
                        unsafe extern "C" fn(*mut libc::c_void, *mut libc::c_void) -> (),
                    >,
                >(
                    Some(
                        ict_rcv_3456xx
                            as unsafe extern "C" fn(
                                *mut osip_transaction_t,
                                *mut osip_event_t,
                            ) -> (),
                    ),
                ),
                next: &mut *ict_transition.as_mut_ptr().offset(9 as libc::c_int as isize)
                    as *mut transition_t,
                parent: 0 as *mut _transition_t,
            };
            init
        },
        {
            let mut init = _transition_t {
                state: ICT_COMPLETED,
                type_0: RCV_STATUS_3456XX,
                method: ::std::mem::transmute::<
                    Option::<
                        unsafe extern "C" fn(
                            *mut osip_transaction_t,
                            *mut osip_event_t,
                        ) -> (),
                    >,
                    Option::<
                        unsafe extern "C" fn(*mut libc::c_void, *mut libc::c_void) -> (),
                    >,
                >(
                    Some(
                        ict_retransmit_ack
                            as unsafe extern "C" fn(
                                *mut osip_transaction_t,
                                *mut osip_event_t,
                            ) -> (),
                    ),
                ),
                next: &mut *ict_transition
                    .as_mut_ptr()
                    .offset(10 as libc::c_int as isize) as *mut transition_t,
                parent: 0 as *mut _transition_t,
            };
            init
        },
        {
            let mut init = _transition_t {
                state: ICT_COMPLETED,
                type_0: TIMEOUT_D,
                method: ::std::mem::transmute::<
                    Option::<
                        unsafe extern "C" fn(
                            *mut osip_transaction_t,
                            *mut osip_event_t,
                        ) -> (),
                    >,
                    Option::<
                        unsafe extern "C" fn(*mut libc::c_void, *mut libc::c_void) -> (),
                    >,
                >(
                    Some(
                        osip_ict_timeout_d_event
                            as unsafe extern "C" fn(
                                *mut osip_transaction_t,
                                *mut osip_event_t,
                            ) -> (),
                    ),
                ),
                next: 0 as *mut _transition_t,
                parent: 0 as *mut _transition_t,
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
