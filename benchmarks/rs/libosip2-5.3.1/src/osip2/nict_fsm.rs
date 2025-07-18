use ::libc;
extern "C" {
    pub type osip_sem;
    pub type osip_mutex;
    fn strcmp(_: *const libc::c_char, _: *const libc::c_char) -> libc::c_int;
    fn osip_gettimeofday(tp: *mut timeval, tz: *mut libc::c_void) -> libc::c_int;
    fn osip_strcasecmp(s1: *const libc::c_char, s2: *const libc::c_char) -> libc::c_int;
    fn add_gettimeofday(atv: *mut timeval, ms: libc::c_int);
    fn osip_via_get_protocol(header: *mut osip_via_t) -> *mut libc::c_char;
    fn osip_message_free(sip: *mut osip_message_t);
    fn osip_message_get_via(
        sip: *const osip_message_t,
        pos: libc::c_int,
        dest: *mut *mut osip_via_t,
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
pub static mut nict_transition: [transition_t; 12] = [transition_t {
    state: ICT_PRE_CALLING,
    type_0: TIMEOUT_A,
    method: None,
    next: 0 as *mut _transition_t,
    parent: 0 as *mut _transition_t,
}; 12];
pub static mut nict_fsm: osip_statemachine_t = unsafe {
    {
        let mut init = osip_statemachine {
            transitions: nict_transition.as_ptr() as *mut _,
        };
        init
    }
};
unsafe extern "C" fn nict_handle_transport_error(
    mut nict: *mut osip_transaction_t,
    mut err: libc::c_int,
) {
    __osip_transport_error_callback(OSIP_NICT_TRANSPORT_ERROR as libc::c_int, nict, err);
    __osip_transaction_set_state(nict, NICT_TERMINATED);
    __osip_kill_transaction_callback(OSIP_NICT_KILL_TRANSACTION as libc::c_int, nict);
}
pub unsafe extern "C" fn nict_snd_request(
    mut nict: *mut osip_transaction_t,
    mut evt: *mut osip_event_t,
) {
    let mut i: libc::c_int = 0;
    let mut osip: *mut osip_t = (*nict).config as *mut osip_t;
    (*nict).orig_request = (*evt).sip;
    i = ((*osip).cb_send_message)
        .unwrap()(
        nict,
        (*evt).sip,
        (*(*nict).nict_context).destination,
        (*(*nict).nict_context).port,
        (*nict).out_socket,
    );
    if i >= 0 as libc::c_int {
        if (*(*evt).sip).status_code == 0 as libc::c_int
            && 0 as libc::c_int
                == strcmp(
                    (*(*evt).sip).sip_method,
                    b"REGISTER\0" as *const u8 as *const libc::c_char,
                )
        {
            __osip_message_callback(
                OSIP_NICT_REGISTER_SENT as libc::c_int,
                nict,
                (*nict).orig_request,
            );
        } else if (*(*evt).sip).status_code == 0 as libc::c_int
            && 0 as libc::c_int
                == strcmp(
                    (*(*evt).sip).sip_method,
                    b"BYE\0" as *const u8 as *const libc::c_char,
                )
        {
            __osip_message_callback(
                OSIP_NICT_BYE_SENT as libc::c_int,
                nict,
                (*nict).orig_request,
            );
        } else if (*(*evt).sip).status_code == 0 as libc::c_int
            && 0 as libc::c_int
                == strcmp(
                    (*(*evt).sip).sip_method,
                    b"OPTIONS\0" as *const u8 as *const libc::c_char,
                )
        {
            __osip_message_callback(
                OSIP_NICT_OPTIONS_SENT as libc::c_int,
                nict,
                (*nict).orig_request,
            );
        } else if (*(*evt).sip).status_code == 0 as libc::c_int
            && 0 as libc::c_int
                == strcmp(
                    (*(*evt).sip).sip_method,
                    b"INFO\0" as *const u8 as *const libc::c_char,
                )
        {
            __osip_message_callback(
                OSIP_NICT_INFO_SENT as libc::c_int,
                nict,
                (*nict).orig_request,
            );
        } else if (*(*evt).sip).status_code == 0 as libc::c_int
            && 0 as libc::c_int
                == strcmp(
                    (*(*evt).sip).sip_method,
                    b"CANCEL\0" as *const u8 as *const libc::c_char,
                )
        {
            __osip_message_callback(
                OSIP_NICT_CANCEL_SENT as libc::c_int,
                nict,
                (*nict).orig_request,
            );
        } else if (*(*evt).sip).status_code == 0 as libc::c_int
            && 0 as libc::c_int
                == strcmp(
                    (*(*evt).sip).sip_method,
                    b"NOTIFY\0" as *const u8 as *const libc::c_char,
                )
        {
            __osip_message_callback(
                OSIP_NICT_NOTIFY_SENT as libc::c_int,
                nict,
                (*nict).orig_request,
            );
        } else if (*(*evt).sip).status_code == 0 as libc::c_int
            && 0 as libc::c_int
                == strcmp(
                    (*(*evt).sip).sip_method,
                    b"SUBSCRIBE\0" as *const u8 as *const libc::c_char,
                )
        {
            __osip_message_callback(
                OSIP_NICT_SUBSCRIBE_SENT as libc::c_int,
                nict,
                (*nict).orig_request,
            );
        } else {
            __osip_message_callback(
                OSIP_NICT_UNKNOWN_REQUEST_SENT as libc::c_int,
                nict,
                (*nict).orig_request,
            );
        }
        let mut via: *mut osip_via_t = 0 as *mut osip_via_t;
        let mut proto: *mut libc::c_char = 0 as *mut libc::c_char;
        let mut k: libc::c_int = 0;
        k = osip_message_get_via((*nict).orig_request, 0 as libc::c_int, &mut via);
        if k < 0 as libc::c_int {
            nict_handle_transport_error(nict, -(1 as libc::c_int));
            return;
        }
        proto = osip_via_get_protocol(via);
        if proto.is_null() {
            nict_handle_transport_error(nict, -(1 as libc::c_int));
            return;
        }
        if i == 0 as libc::c_int {
            if !(osip_strcasecmp(proto, b"TCP\0" as *const u8 as *const libc::c_char)
                != 0 as libc::c_int
                && osip_strcasecmp(proto, b"TLS\0" as *const u8 as *const libc::c_char)
                    != 0 as libc::c_int
                && osip_strcasecmp(proto, b"SCTP\0" as *const u8 as *const libc::c_char)
                    != 0 as libc::c_int)
            {
                (*(*nict).nict_context).timer_e_length = -(1 as libc::c_int);
                (*(*nict).nict_context)
                    .timer_e_start
                    .tv_sec = -(1 as libc::c_int) as __time_t;
            }
        } else if !(osip_strcasecmp(proto, b"TCP\0" as *const u8 as *const libc::c_char)
            != 0 as libc::c_int
            && osip_strcasecmp(proto, b"TLS\0" as *const u8 as *const libc::c_char)
                != 0 as libc::c_int
            && osip_strcasecmp(proto, b"SCTP\0" as *const u8 as *const libc::c_char)
                != 0 as libc::c_int)
        {
            (*(*nict).nict_context).timer_e_length = 500 as libc::c_int;
        }
        if (*(*nict).nict_context).timer_e_length > 0 as libc::c_int {
            osip_gettimeofday(
                &mut (*(*nict).nict_context).timer_e_start,
                0 as *mut libc::c_void,
            );
            add_gettimeofday(
                &mut (*(*nict).nict_context).timer_e_start,
                (*(*nict).nict_context).timer_e_length,
            );
        }
        __osip_transaction_set_state(nict, NICT_TRYING);
    } else {
        nict_handle_transport_error(nict, i);
    };
}
pub unsafe extern "C" fn osip_nict_timeout_e_event(
    mut nict: *mut osip_transaction_t,
    mut evt: *mut osip_event_t,
) {
    let mut osip: *mut osip_t = (*nict).config as *mut osip_t;
    let mut i: libc::c_int = 0;
    if (*nict).state as libc::c_uint == NICT_TRYING as libc::c_int as libc::c_uint {
        let mut now: timeval = timeval { tv_sec: 0, tv_usec: 0 };
        let mut duration: timeval = timeval { tv_sec: 0, tv_usec: 0 };
        let mut elapsed: libc::c_long = 0;
        osip_gettimeofday(&mut now, 0 as *mut libc::c_void);
        duration.tv_sec = now.tv_sec - (*nict).created_time.tv_sec;
        duration.tv_usec = now.tv_usec - (*nict).created_time.tv_usec;
        if duration.tv_usec < 0 as libc::c_int as libc::c_long {
            duration.tv_sec -= 1;
            duration.tv_sec;
            duration.tv_usec += 1000000 as libc::c_int as libc::c_long;
        }
        elapsed = duration.tv_sec * 1000 as libc::c_int as libc::c_long
            + duration.tv_usec / 1000 as libc::c_int as libc::c_long;
        if elapsed < 500 as libc::c_int as libc::c_long {
            (*(*nict).nict_context).timer_e_length = 500 as libc::c_int;
        } else if elapsed < (2 as libc::c_int * 500 as libc::c_int) as libc::c_long {
            (*(*nict).nict_context)
                .timer_e_length = 2 as libc::c_int * 500 as libc::c_int;
        } else if elapsed < (4 as libc::c_int * 500 as libc::c_int) as libc::c_long {
            (*(*nict).nict_context)
                .timer_e_length = 4 as libc::c_int * 500 as libc::c_int;
        } else {
            (*(*nict).nict_context)
                .timer_e_length = (*(*nict).nict_context).timer_e_length
                * 2 as libc::c_int;
        }
        if (*(*nict).nict_context).timer_e_length > 4000 as libc::c_int {
            (*(*nict).nict_context).timer_e_length = 4000 as libc::c_int;
        }
    } else {
        (*(*nict).nict_context).timer_e_length = 4000 as libc::c_int;
    }
    osip_gettimeofday(
        &mut (*(*nict).nict_context).timer_e_start,
        0 as *mut libc::c_void,
    );
    add_gettimeofday(
        &mut (*(*nict).nict_context).timer_e_start,
        (*(*nict).nict_context).timer_e_length,
    );
    i = ((*osip).cb_send_message)
        .unwrap()(
        nict,
        (*nict).orig_request,
        (*(*nict).nict_context).destination,
        (*(*nict).nict_context).port,
        (*nict).out_socket,
    );
    if i < 0 as libc::c_int {
        nict_handle_transport_error(nict, i);
        return;
    }
    if i == 0 as libc::c_int {
        let mut via: *mut osip_via_t = 0 as *mut osip_via_t;
        let mut proto: *mut libc::c_char = 0 as *mut libc::c_char;
        i = osip_message_get_via((*nict).orig_request, 0 as libc::c_int, &mut via);
        if i < 0 as libc::c_int {
            nict_handle_transport_error(nict, -(1 as libc::c_int));
            return;
        }
        proto = osip_via_get_protocol(via);
        if proto.is_null() {
            nict_handle_transport_error(nict, -(1 as libc::c_int));
            return;
        }
        if !(osip_strcasecmp(proto, b"TCP\0" as *const u8 as *const libc::c_char)
            != 0 as libc::c_int
            && osip_strcasecmp(proto, b"TLS\0" as *const u8 as *const libc::c_char)
                != 0 as libc::c_int
            && osip_strcasecmp(proto, b"SCTP\0" as *const u8 as *const libc::c_char)
                != 0 as libc::c_int)
        {
            (*(*nict).nict_context).timer_e_length = -(1 as libc::c_int);
            (*(*nict).nict_context)
                .timer_e_start
                .tv_sec = -(1 as libc::c_int) as __time_t;
        }
    }
    if i == 0 as libc::c_int {
        __osip_message_callback(
            OSIP_NICT_REQUEST_SENT_AGAIN as libc::c_int,
            nict,
            (*nict).orig_request,
        );
    }
}
pub unsafe extern "C" fn osip_nict_timeout_f_event(
    mut nict: *mut osip_transaction_t,
    mut evt: *mut osip_event_t,
) {
    (*(*nict).nict_context).timer_f_length = -(1 as libc::c_int);
    (*(*nict).nict_context).timer_f_start.tv_sec = -(1 as libc::c_int) as __time_t;
    if (*nict).out_socket == -(999 as libc::c_int) {
        nict_handle_transport_error(nict, -(1 as libc::c_int));
    } else {
        __osip_message_callback(
            OSIP_NICT_STATUS_TIMEOUT as libc::c_int,
            nict,
            (*evt).sip,
        );
        __osip_transaction_set_state(nict, NICT_TERMINATED);
        __osip_kill_transaction_callback(
            OSIP_NICT_KILL_TRANSACTION as libc::c_int,
            nict,
        );
    };
}
pub unsafe extern "C" fn osip_nict_timeout_k_event(
    mut nict: *mut osip_transaction_t,
    mut evt: *mut osip_event_t,
) {
    (*(*nict).nict_context).timer_k_length = -(1 as libc::c_int);
    (*(*nict).nict_context).timer_k_start.tv_sec = -(1 as libc::c_int) as __time_t;
    __osip_transaction_set_state(nict, NICT_TERMINATED);
    __osip_kill_transaction_callback(OSIP_NICT_KILL_TRANSACTION as libc::c_int, nict);
}
pub unsafe extern "C" fn nict_rcv_1xx(
    mut nict: *mut osip_transaction_t,
    mut evt: *mut osip_event_t,
) {
    if !((*nict).last_response).is_null() {
        osip_message_free((*nict).last_response);
    }
    (*nict).last_response = (*evt).sip;
    if (*(*nict).nict_context).timer_e_length > 0 as libc::c_int {
        (*(*nict).nict_context).timer_e_length = 4000 as libc::c_int;
        osip_gettimeofday(
            &mut (*(*nict).nict_context).timer_e_start,
            0 as *mut libc::c_void,
        );
        add_gettimeofday(
            &mut (*(*nict).nict_context).timer_e_start,
            (*(*nict).nict_context).timer_e_length,
        );
    }
    __osip_message_callback(
        OSIP_NICT_STATUS_1XX_RECEIVED as libc::c_int,
        nict,
        (*evt).sip,
    );
    __osip_transaction_set_state(nict, NICT_PROCEEDING);
}
pub unsafe extern "C" fn nict_rcv_23456xx(
    mut nict: *mut osip_transaction_t,
    mut evt: *mut osip_event_t,
) {
    if !((*nict).last_response).is_null() {
        osip_message_free((*nict).last_response);
    }
    (*nict).last_response = (*evt).sip;
    if (*evt).type_0 as libc::c_uint == RCV_STATUS_2XX as libc::c_int as libc::c_uint {
        __osip_message_callback(
            OSIP_NICT_STATUS_2XX_RECEIVED as libc::c_int,
            nict,
            (*nict).last_response,
        );
    } else if (*(*nict).last_response).status_code >= 300 as libc::c_int
        && (*(*nict).last_response).status_code < 400 as libc::c_int
    {
        __osip_message_callback(
            OSIP_NICT_STATUS_3XX_RECEIVED as libc::c_int,
            nict,
            (*nict).last_response,
        );
    } else if (*(*nict).last_response).status_code >= 400 as libc::c_int
        && (*(*nict).last_response).status_code < 500 as libc::c_int
    {
        __osip_message_callback(
            OSIP_NICT_STATUS_4XX_RECEIVED as libc::c_int,
            nict,
            (*nict).last_response,
        );
    } else if (*(*nict).last_response).status_code >= 500 as libc::c_int
        && (*(*nict).last_response).status_code < 600 as libc::c_int
    {
        __osip_message_callback(
            OSIP_NICT_STATUS_5XX_RECEIVED as libc::c_int,
            nict,
            (*nict).last_response,
        );
    } else {
        __osip_message_callback(
            OSIP_NICT_STATUS_6XX_RECEIVED as libc::c_int,
            nict,
            (*nict).last_response,
        );
    }
    if (*nict).state as libc::c_uint != NICT_COMPLETED as libc::c_int as libc::c_uint {
        osip_gettimeofday(
            &mut (*(*nict).nict_context).timer_k_start,
            0 as *mut libc::c_void,
        );
        add_gettimeofday(
            &mut (*(*nict).nict_context).timer_k_start,
            (*(*nict).nict_context).timer_k_length,
        );
    }
    __osip_transaction_set_state(nict, NICT_COMPLETED);
}
unsafe extern "C" fn run_static_initializers() {
    nict_transition = [
        {
            let mut init = _transition_t {
                state: NICT_PRE_TRYING,
                type_0: SND_REQUEST,
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
                        nict_snd_request
                            as unsafe extern "C" fn(
                                *mut osip_transaction_t,
                                *mut osip_event_t,
                            ) -> (),
                    ),
                ),
                next: &mut *nict_transition
                    .as_mut_ptr()
                    .offset(1 as libc::c_int as isize) as *mut transition_t,
                parent: 0 as *mut _transition_t,
            };
            init
        },
        {
            let mut init = _transition_t {
                state: NICT_TRYING,
                type_0: TIMEOUT_F,
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
                        osip_nict_timeout_f_event
                            as unsafe extern "C" fn(
                                *mut osip_transaction_t,
                                *mut osip_event_t,
                            ) -> (),
                    ),
                ),
                next: &mut *nict_transition
                    .as_mut_ptr()
                    .offset(2 as libc::c_int as isize) as *mut transition_t,
                parent: 0 as *mut _transition_t,
            };
            init
        },
        {
            let mut init = _transition_t {
                state: NICT_TRYING,
                type_0: TIMEOUT_E,
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
                        osip_nict_timeout_e_event
                            as unsafe extern "C" fn(
                                *mut osip_transaction_t,
                                *mut osip_event_t,
                            ) -> (),
                    ),
                ),
                next: &mut *nict_transition
                    .as_mut_ptr()
                    .offset(3 as libc::c_int as isize) as *mut transition_t,
                parent: 0 as *mut _transition_t,
            };
            init
        },
        {
            let mut init = _transition_t {
                state: NICT_TRYING,
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
                        nict_rcv_1xx
                            as unsafe extern "C" fn(
                                *mut osip_transaction_t,
                                *mut osip_event_t,
                            ) -> (),
                    ),
                ),
                next: &mut *nict_transition
                    .as_mut_ptr()
                    .offset(4 as libc::c_int as isize) as *mut transition_t,
                parent: 0 as *mut _transition_t,
            };
            init
        },
        {
            let mut init = _transition_t {
                state: NICT_TRYING,
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
                        nict_rcv_23456xx
                            as unsafe extern "C" fn(
                                *mut osip_transaction_t,
                                *mut osip_event_t,
                            ) -> (),
                    ),
                ),
                next: &mut *nict_transition
                    .as_mut_ptr()
                    .offset(5 as libc::c_int as isize) as *mut transition_t,
                parent: 0 as *mut _transition_t,
            };
            init
        },
        {
            let mut init = _transition_t {
                state: NICT_TRYING,
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
                        nict_rcv_23456xx
                            as unsafe extern "C" fn(
                                *mut osip_transaction_t,
                                *mut osip_event_t,
                            ) -> (),
                    ),
                ),
                next: &mut *nict_transition
                    .as_mut_ptr()
                    .offset(6 as libc::c_int as isize) as *mut transition_t,
                parent: 0 as *mut _transition_t,
            };
            init
        },
        {
            let mut init = _transition_t {
                state: NICT_PROCEEDING,
                type_0: TIMEOUT_F,
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
                        osip_nict_timeout_f_event
                            as unsafe extern "C" fn(
                                *mut osip_transaction_t,
                                *mut osip_event_t,
                            ) -> (),
                    ),
                ),
                next: &mut *nict_transition
                    .as_mut_ptr()
                    .offset(7 as libc::c_int as isize) as *mut transition_t,
                parent: 0 as *mut _transition_t,
            };
            init
        },
        {
            let mut init = _transition_t {
                state: NICT_PROCEEDING,
                type_0: TIMEOUT_E,
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
                        osip_nict_timeout_e_event
                            as unsafe extern "C" fn(
                                *mut osip_transaction_t,
                                *mut osip_event_t,
                            ) -> (),
                    ),
                ),
                next: &mut *nict_transition
                    .as_mut_ptr()
                    .offset(8 as libc::c_int as isize) as *mut transition_t,
                parent: 0 as *mut _transition_t,
            };
            init
        },
        {
            let mut init = _transition_t {
                state: NICT_PROCEEDING,
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
                        nict_rcv_1xx
                            as unsafe extern "C" fn(
                                *mut osip_transaction_t,
                                *mut osip_event_t,
                            ) -> (),
                    ),
                ),
                next: &mut *nict_transition
                    .as_mut_ptr()
                    .offset(9 as libc::c_int as isize) as *mut transition_t,
                parent: 0 as *mut _transition_t,
            };
            init
        },
        {
            let mut init = _transition_t {
                state: NICT_PROCEEDING,
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
                        nict_rcv_23456xx
                            as unsafe extern "C" fn(
                                *mut osip_transaction_t,
                                *mut osip_event_t,
                            ) -> (),
                    ),
                ),
                next: &mut *nict_transition
                    .as_mut_ptr()
                    .offset(10 as libc::c_int as isize) as *mut transition_t,
                parent: 0 as *mut _transition_t,
            };
            init
        },
        {
            let mut init = _transition_t {
                state: NICT_PROCEEDING,
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
                        nict_rcv_23456xx
                            as unsafe extern "C" fn(
                                *mut osip_transaction_t,
                                *mut osip_event_t,
                            ) -> (),
                    ),
                ),
                next: &mut *nict_transition
                    .as_mut_ptr()
                    .offset(11 as libc::c_int as isize) as *mut transition_t,
                parent: 0 as *mut _transition_t,
            };
            init
        },
        {
            let mut init = _transition_t {
                state: NICT_COMPLETED,
                type_0: TIMEOUT_K,
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
                        osip_nict_timeout_k_event
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
