use ::libc;
extern "C" {
    pub type _IO_wide_data;
    pub type _IO_codecvt;
    pub type _IO_marker;
    pub type osip_sem;
    pub type osip_mutex;
    fn memset(
        _: *mut libc::c_void,
        _: libc::c_int,
        _: libc::c_ulong,
    ) -> *mut libc::c_void;
    fn strcmp(_: *const libc::c_char, _: *const libc::c_char) -> libc::c_int;
    fn malloc(_: libc::c_ulong) -> *mut libc::c_void;
    fn free(__ptr: *mut libc::c_void);
    fn osip_list_init(li: *mut osip_list_t) -> libc::c_int;
    fn osip_gettimeofday(tp: *mut timeval, tz: *mut libc::c_void) -> libc::c_int;
    fn osip_getsystemtime(t: *mut time_t) -> time_t;
    fn osip_list_size(li: *const osip_list_t) -> libc::c_int;
    fn osip_list_eol(li: *const osip_list_t, pos: libc::c_int) -> libc::c_int;
    fn osip_list_add(
        li: *mut osip_list_t,
        element: *mut libc::c_void,
        pos: libc::c_int,
    ) -> libc::c_int;
    fn osip_list_get(li: *const osip_list_t, pos: libc::c_int) -> *mut libc::c_void;
    fn osip_list_remove(li: *mut osip_list_t, pos: libc::c_int) -> libc::c_int;
    fn osip_list_get_first(
        li: *const osip_list_t,
        it: *mut osip_list_iterator_t,
    ) -> *mut libc::c_void;
    fn osip_list_get_next(it: *mut osip_list_iterator_t) -> *mut libc::c_void;
    fn osip_list_iterator_remove(it: *mut osip_list_iterator_t) -> *mut libc::c_void;
    static mut osip_malloc_func: Option::<osip_malloc_func_t>;
    static mut osip_free_func: Option::<osip_free_func_t>;
    fn osip_atoi(number: *const libc::c_char) -> libc::c_int;
    fn osip_strdup(ch: *const libc::c_char) -> *mut libc::c_char;
    fn osip_trace(
        fi: *const libc::c_char,
        li: libc::c_int,
        level: osip_trace_level_t,
        f: *mut FILE,
        chfr: *const libc::c_char,
        _: ...
    ) -> libc::c_int;
    fn osip_uri_param_get_byname(
        url_params: *mut osip_list_t,
        name: *mut libc::c_char,
        dest: *mut *mut osip_uri_param_t,
    ) -> libc::c_int;
    fn osip_message_free(sip: *mut osip_message_t);
    fn osip_message_clone(
        sip: *const osip_message_t,
        dest: *mut *mut osip_message_t,
    ) -> libc::c_int;
    fn parser_init() -> libc::c_int;
    fn min_timercmp(tv1: *mut timeval, tv2: *mut timeval);
    fn add_gettimeofday(atv: *mut timeval, ms: libc::c_int);
    fn osip_mutex_init() -> *mut osip_mutex;
    fn osip_mutex_destroy(mut_0: *mut osip_mutex);
    fn osip_mutex_lock(mut_0: *mut osip_mutex) -> libc::c_int;
    fn osip_mutex_unlock(mut_0: *mut osip_mutex) -> libc::c_int;
    fn osip_fifo_add(ff: *mut osip_fifo_t, element: *mut libc::c_void) -> libc::c_int;
    fn osip_fifo_size(ff: *mut osip_fifo_t) -> libc::c_int;
    fn osip_fifo_tryget(ff: *mut osip_fifo_t) -> *mut libc::c_void;
    fn osip_transaction_init(
        transaction: *mut *mut osip_transaction_t,
        ctx_type: osip_fsm_type_t,
        osip: *mut osip_t,
        request: *mut osip_message_t,
    ) -> libc::c_int;
    fn osip_transaction_add_event(
        transaction: *mut osip_transaction_t,
        evt: *mut osip_event_t,
    ) -> libc::c_int;
    fn osip_transaction_execute(
        transaction: *mut osip_transaction_t,
        evt: *mut osip_event_t,
    ) -> libc::c_int;
    fn __osip_ict_need_timer_a_event(
        ict: *mut osip_ict_t,
        state: state_t,
        transactionid: libc::c_int,
    ) -> *mut osip_event_t;
    fn __osip_ict_need_timer_b_event(
        ict: *mut osip_ict_t,
        state: state_t,
        transactionid: libc::c_int,
    ) -> *mut osip_event_t;
    fn __osip_ict_need_timer_d_event(
        ict: *mut osip_ict_t,
        state: state_t,
        transactionid: libc::c_int,
    ) -> *mut osip_event_t;
    fn __osip_nict_need_timer_e_event(
        nict: *mut osip_nict_t,
        state: state_t,
        transactionid: libc::c_int,
    ) -> *mut osip_event_t;
    fn __osip_nict_need_timer_f_event(
        nict: *mut osip_nict_t,
        state: state_t,
        transactionid: libc::c_int,
    ) -> *mut osip_event_t;
    fn __osip_nict_need_timer_k_event(
        nict: *mut osip_nict_t,
        state: state_t,
        transactionid: libc::c_int,
    ) -> *mut osip_event_t;
    fn __osip_ist_need_timer_g_event(
        ist: *mut osip_ist_t,
        state: state_t,
        transactionid: libc::c_int,
    ) -> *mut osip_event_t;
    fn __osip_ist_need_timer_h_event(
        ist: *mut osip_ist_t,
        state: state_t,
        transactionid: libc::c_int,
    ) -> *mut osip_event_t;
    fn __osip_ist_need_timer_i_event(
        ist: *mut osip_ist_t,
        state: state_t,
        transactionid: libc::c_int,
    ) -> *mut osip_event_t;
    fn __osip_nist_need_timer_j_event(
        nist: *mut osip_nist_t,
        state: state_t,
        transactionid: libc::c_int,
    ) -> *mut osip_event_t;
    fn __osip_transaction_matching_response_osip_to_xict_17_1_3(
        tr: *mut osip_transaction_t,
        resp: *mut osip_message_t,
    ) -> libc::c_int;
    fn __osip_transaction_matching_request_osip_to_xist_17_2_3(
        tr: *mut osip_transaction_t,
        request: *mut osip_message_t,
    ) -> libc::c_int;
    fn osip_dialog_match_as_uas(
        dialog: *mut osip_dialog_t,
        request: *mut osip_message_t,
    ) -> libc::c_int;
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct osip_list_iterator {
    pub actual: *mut __node_t,
    pub prev: *mut *mut __node_t,
    pub li: *mut osip_list_t,
    pub pos: libc::c_int,
}
pub type osip_list_iterator_t = osip_list_iterator;
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
pub type osip_contact_t = osip_from_t;
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
pub struct osip_dialog {
    pub call_id: *mut libc::c_char,
    pub local_tag: *mut libc::c_char,
    pub remote_tag: *mut libc::c_char,
    pub line_param: *mut libc::c_char,
    pub route_set: osip_list_t,
    pub local_cseq: libc::c_int,
    pub remote_cseq: libc::c_int,
    pub remote_uri: *mut osip_to_t,
    pub local_uri: *mut osip_from_t,
    pub remote_contact_uri: *mut osip_contact_t,
    pub secure: libc::c_int,
    pub type_0: osip_dialog_type_t,
    pub state: state_t,
    pub your_instance: *mut libc::c_void,
}
pub type osip_dialog_type_t = _osip_dialog_type_t;
pub type _osip_dialog_type_t = libc::c_uint;
pub const CALLEE: _osip_dialog_type_t = 1;
pub const CALLER: _osip_dialog_type_t = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ixt {
    pub dialog: *mut osip_dialog,
    pub msg2xx: *mut osip_message_t,
    pub ack: *mut osip_message_t,
    pub start: timeval,
    pub interval: libc::c_int,
    pub dest: *mut libc::c_char,
    pub port: libc::c_int,
    pub sock: libc::c_int,
    pub counter: libc::c_int,
}
pub type ixt_t = ixt;
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
pub type osip_dialog_t = osip_dialog;
pub unsafe extern "C" fn osip_response_get_destination(
    mut response: *mut osip_message_t,
    mut address: *mut *mut libc::c_char,
    mut portnum: *mut libc::c_int,
) {
    let mut via: *mut osip_via_t = 0 as *mut osip_via_t;
    let mut host: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut port: libc::c_int = 0 as libc::c_int;
    via = osip_list_get(&mut (*response).vias, 0 as libc::c_int) as *mut osip_via_t;
    if !via.is_null() {
        let mut maddr: *mut osip_generic_param_t = 0 as *mut osip_generic_param_t;
        let mut received: *mut osip_generic_param_t = 0 as *mut osip_generic_param_t;
        let mut rport: *mut osip_generic_param_t = 0 as *mut osip_generic_param_t;
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
    }
    *portnum = port;
    if !host.is_null() {
        *address = osip_strdup(host);
    } else {
        *address = 0 as *mut libc::c_char;
    };
}
unsafe extern "C" fn osip_ixt_lock(mut osip: *mut osip_t) -> libc::c_int {
    return osip_mutex_lock((*osip).ixt_fastmutex as *mut osip_mutex);
}
unsafe extern "C" fn osip_ixt_unlock(mut osip: *mut osip_t) -> libc::c_int {
    return osip_mutex_unlock((*osip).ixt_fastmutex as *mut osip_mutex);
}
pub unsafe extern "C" fn osip_id_mutex_lock(mut osip: *mut osip_t) -> libc::c_int {
    return osip_mutex_lock((*osip).id_mutex as *mut osip_mutex);
}
pub unsafe extern "C" fn osip_id_mutex_unlock(mut osip: *mut osip_t) -> libc::c_int {
    return osip_mutex_unlock((*osip).id_mutex as *mut osip_mutex);
}
unsafe extern "C" fn osip_add_ixt(mut osip: *mut osip_t, mut ixt: *mut ixt_t) {
    osip_ixt_lock(osip);
    osip_list_add(
        &mut (*osip).ixt_retransmissions,
        ixt as *mut libc::c_void,
        0 as libc::c_int,
    );
    osip_ixt_unlock(osip);
}
unsafe extern "C" fn ixt_init(mut ixt: *mut *mut ixt_t) -> libc::c_int {
    let mut pixt: *mut ixt_t = 0 as *mut ixt_t;
    pixt = (if osip_malloc_func.is_some() {
        osip_malloc_func.unwrap()(::std::mem::size_of::<ixt_t>() as libc::c_ulong)
    } else {
        malloc(::std::mem::size_of::<ixt_t>() as libc::c_ulong)
    }) as *mut ixt_t;
    *ixt = pixt;
    if pixt.is_null() {
        return -(4 as libc::c_int);
    }
    (*pixt).dialog = 0 as *mut osip_dialog;
    (*pixt).msg2xx = 0 as *mut osip_message_t;
    (*pixt).ack = 0 as *mut osip_message_t;
    (*pixt).interval = 500 as libc::c_int;
    osip_gettimeofday(&mut (*pixt).start, 0 as *mut libc::c_void);
    add_gettimeofday(&mut (*pixt).start, (*pixt).interval + 10 as libc::c_int);
    (*pixt).counter = 10 as libc::c_int;
    (*pixt).dest = 0 as *mut libc::c_char;
    (*pixt).port = 5060 as libc::c_int;
    (*pixt).sock = -(1 as libc::c_int);
    return 0 as libc::c_int;
}
unsafe extern "C" fn ixt_free(mut ixt: *mut ixt_t) {
    osip_message_free((*ixt).ack);
    osip_message_free((*ixt).msg2xx);
    if !((*ixt).dest).is_null() {
        if osip_free_func.is_some() {
            osip_free_func.unwrap()((*ixt).dest as *mut libc::c_void);
        } else {
            free((*ixt).dest as *mut libc::c_void);
        }
    }
    if !ixt.is_null() {
        if osip_free_func.is_some() {
            osip_free_func.unwrap()(ixt as *mut libc::c_void);
        } else {
            free(ixt as *mut libc::c_void);
        }
    }
}
pub unsafe extern "C" fn osip_start_200ok_retransmissions(
    mut osip: *mut osip_t,
    mut dialog: *mut osip_dialog_t,
    mut msg200ok: *mut osip_message_t,
    mut sock: libc::c_int,
) {
    let mut i: libc::c_int = 0;
    let mut ixt: *mut ixt_t = 0 as *mut ixt_t;
    i = ixt_init(&mut ixt);
    if i != 0 as libc::c_int {
        return;
    }
    (*ixt).dialog = dialog;
    osip_message_clone(msg200ok, &mut (*ixt).msg2xx);
    (*ixt).sock = sock;
    osip_response_get_destination(msg200ok, &mut (*ixt).dest, &mut (*ixt).port);
    osip_add_ixt(osip, ixt);
}
pub unsafe extern "C" fn osip_start_ack_retransmissions(
    mut osip: *mut osip_t,
    mut dialog: *mut osip_dialog_t,
    mut ack: *mut osip_message_t,
    mut dest: *mut libc::c_char,
    mut port: libc::c_int,
    mut sock: libc::c_int,
) {
    let mut i: libc::c_int = 0;
    let mut ixt: *mut ixt_t = 0 as *mut ixt_t;
    i = ixt_init(&mut ixt);
    if i != 0 as libc::c_int {
        return;
    }
    (*ixt).dialog = dialog;
    osip_message_clone(ack, &mut (*ixt).ack);
    (*ixt).dest = osip_strdup(dest);
    (*ixt).port = port;
    (*ixt).sock = sock;
    osip_add_ixt(osip, ixt);
}
pub unsafe extern "C" fn osip_stop_200ok_retransmissions(
    mut osip: *mut osip_t,
    mut ack: *mut osip_message_t,
) -> *mut osip_dialog {
    let mut dialog: *mut osip_dialog_t = 0 as *mut osip_dialog_t;
    let mut i: libc::c_int = 0;
    let mut ixt: *mut ixt_t = 0 as *mut ixt_t;
    if ack.is_null() || ((*ack).cseq).is_null() || ((*(*ack).cseq).number).is_null() {
        return 0 as *mut osip_dialog;
    }
    osip_ixt_lock(osip);
    i = 0 as libc::c_int;
    while osip_list_eol(&mut (*osip).ixt_retransmissions, i) == 0 {
        ixt = osip_list_get(&mut (*osip).ixt_retransmissions, i) as *mut ixt_t;
        if !(((*ixt).msg2xx).is_null() || ((*(*ixt).msg2xx).cseq).is_null()
            || ((*(*(*ixt).msg2xx).cseq).number).is_null())
        {
            if osip_dialog_match_as_uas((*ixt).dialog, ack) == 0 as libc::c_int
                && strcmp((*(*(*ixt).msg2xx).cseq).number, (*(*ack).cseq).number)
                    == 0 as libc::c_int
            {
                osip_list_remove(&mut (*osip).ixt_retransmissions, i);
                dialog = (*ixt).dialog;
                ixt_free(ixt);
                break;
            }
        }
        i += 1;
        i;
    }
    osip_ixt_unlock(osip);
    return dialog;
}
pub unsafe extern "C" fn osip_stop_retransmissions_from_dialog(
    mut osip: *mut osip_t,
    mut dialog: *mut osip_dialog_t,
) {
    let mut i: libc::c_int = 0;
    let mut ixt: *mut ixt_t = 0 as *mut ixt_t;
    osip_ixt_lock(osip);
    i = 0 as libc::c_int;
    while osip_list_eol(&mut (*osip).ixt_retransmissions, i) == 0 {
        ixt = osip_list_get(&mut (*osip).ixt_retransmissions, i) as *mut ixt_t;
        if (*ixt).dialog == dialog {
            osip_list_remove(&mut (*osip).ixt_retransmissions, i);
            ixt_free(ixt);
            i -= 1;
            i;
        }
        i += 1;
        i;
    }
    osip_ixt_unlock(osip);
}
unsafe extern "C" fn ixt_retransmit(
    mut osip: *mut osip_t,
    mut ixt: *mut ixt_t,
    mut current: *mut timeval,
) {
    if if (*current).tv_sec == (*ixt).start.tv_sec {
        ((*current).tv_usec >= (*ixt).start.tv_usec) as libc::c_int
    } else {
        ((*current).tv_sec >= (*ixt).start.tv_sec) as libc::c_int
    } != 0
    {
        (*ixt).interval = (*ixt).interval * 2 as libc::c_int;
        if (*ixt).interval > 4000 as libc::c_int {
            (*ixt).interval = 4000 as libc::c_int;
        }
        add_gettimeofday(&mut (*ixt).start, (*ixt).interval);
        if !((*ixt).ack).is_null() {
            ((*osip).cb_send_message)
                .unwrap()(
                0 as *mut osip_transaction_t,
                (*ixt).ack,
                (*ixt).dest,
                (*ixt).port,
                (*ixt).sock,
            );
        } else if !((*ixt).msg2xx).is_null() {
            ((*osip).cb_send_message)
                .unwrap()(
                0 as *mut osip_transaction_t,
                (*ixt).msg2xx,
                (*ixt).dest,
                (*ixt).port,
                (*ixt).sock,
            );
        }
        (*ixt).counter -= 1;
        (*ixt).counter;
    }
}
pub unsafe extern "C" fn osip_retransmissions_execute(mut osip: *mut osip_t) {
    let mut i: libc::c_int = 0;
    let mut ixt: *mut ixt_t = 0 as *mut ixt_t;
    let mut current: timeval = timeval { tv_sec: 0, tv_usec: 0 };
    osip_gettimeofday(&mut current, 0 as *mut libc::c_void);
    osip_ixt_lock(osip);
    i = 0 as libc::c_int;
    while osip_list_eol(&mut (*osip).ixt_retransmissions, i) == 0 {
        ixt = osip_list_get(&mut (*osip).ixt_retransmissions, i) as *mut ixt_t;
        ixt_retransmit(osip, ixt, &mut current);
        if (*ixt).counter == 0 as libc::c_int {
            osip_list_remove(&mut (*osip).ixt_retransmissions, i);
            ixt_free(ixt);
            i -= 1;
            i;
        }
        i += 1;
        i;
    }
    osip_ixt_unlock(osip);
}
pub unsafe extern "C" fn osip_ict_lock(mut osip: *mut osip_t) -> libc::c_int {
    return osip_mutex_lock((*osip).ict_fastmutex as *mut osip_mutex);
}
pub unsafe extern "C" fn osip_ict_unlock(mut osip: *mut osip_t) -> libc::c_int {
    return osip_mutex_unlock((*osip).ict_fastmutex as *mut osip_mutex);
}
pub unsafe extern "C" fn osip_ist_lock(mut osip: *mut osip_t) -> libc::c_int {
    return osip_mutex_lock((*osip).ist_fastmutex as *mut osip_mutex);
}
pub unsafe extern "C" fn osip_ist_unlock(mut osip: *mut osip_t) -> libc::c_int {
    return osip_mutex_unlock((*osip).ist_fastmutex as *mut osip_mutex);
}
pub unsafe extern "C" fn osip_nict_lock(mut osip: *mut osip_t) -> libc::c_int {
    return osip_mutex_lock((*osip).nict_fastmutex as *mut osip_mutex);
}
pub unsafe extern "C" fn osip_nict_unlock(mut osip: *mut osip_t) -> libc::c_int {
    return osip_mutex_unlock((*osip).nict_fastmutex as *mut osip_mutex);
}
pub unsafe extern "C" fn osip_nist_lock(mut osip: *mut osip_t) -> libc::c_int {
    return osip_mutex_lock((*osip).nist_fastmutex as *mut osip_mutex);
}
pub unsafe extern "C" fn osip_nist_unlock(mut osip: *mut osip_t) -> libc::c_int {
    return osip_mutex_unlock((*osip).nist_fastmutex as *mut osip_mutex);
}
pub unsafe extern "C" fn __osip_add_ict(
    mut osip: *mut osip_t,
    mut ict: *mut osip_transaction_t,
) -> libc::c_int {
    osip_mutex_lock((*osip).ict_fastmutex as *mut osip_mutex);
    osip_list_add(
        &mut (*osip).osip_ict_transactions,
        ict as *mut libc::c_void,
        -(1 as libc::c_int),
    );
    osip_mutex_unlock((*osip).ict_fastmutex as *mut osip_mutex);
    return 0 as libc::c_int;
}
pub unsafe extern "C" fn __osip_add_ist(
    mut osip: *mut osip_t,
    mut ist: *mut osip_transaction_t,
) -> libc::c_int {
    osip_mutex_lock((*osip).ist_fastmutex as *mut osip_mutex);
    osip_list_add(
        &mut (*osip).osip_ist_transactions,
        ist as *mut libc::c_void,
        -(1 as libc::c_int),
    );
    osip_mutex_unlock((*osip).ist_fastmutex as *mut osip_mutex);
    return 0 as libc::c_int;
}
pub unsafe extern "C" fn __osip_add_nict(
    mut osip: *mut osip_t,
    mut nict: *mut osip_transaction_t,
) -> libc::c_int {
    osip_mutex_lock((*osip).nict_fastmutex as *mut osip_mutex);
    osip_list_add(
        &mut (*osip).osip_nict_transactions,
        nict as *mut libc::c_void,
        -(1 as libc::c_int),
    );
    osip_mutex_unlock((*osip).nict_fastmutex as *mut osip_mutex);
    return 0 as libc::c_int;
}
pub unsafe extern "C" fn __osip_add_nist(
    mut osip: *mut osip_t,
    mut nist: *mut osip_transaction_t,
) -> libc::c_int {
    osip_mutex_lock((*osip).nist_fastmutex as *mut osip_mutex);
    osip_list_add(
        &mut (*osip).osip_nist_transactions,
        nist as *mut libc::c_void,
        -(1 as libc::c_int),
    );
    osip_mutex_unlock((*osip).nist_fastmutex as *mut osip_mutex);
    return 0 as libc::c_int;
}
pub unsafe extern "C" fn osip_remove_transaction(
    mut osip: *mut osip_t,
    mut tr: *mut osip_transaction_t,
) -> libc::c_int {
    let mut i: libc::c_int = -(1 as libc::c_int);
    if tr.is_null() {
        return -(2 as libc::c_int);
    }
    if (*tr).ctx_type as libc::c_uint == ICT as libc::c_int as libc::c_uint {
        i = __osip_remove_ict_transaction(osip, tr);
    } else if (*tr).ctx_type as libc::c_uint == IST as libc::c_int as libc::c_uint {
        i = __osip_remove_ist_transaction(osip, tr);
    } else if (*tr).ctx_type as libc::c_uint == NICT as libc::c_int as libc::c_uint {
        i = __osip_remove_nict_transaction(osip, tr);
    } else if (*tr).ctx_type as libc::c_uint == NIST as libc::c_int as libc::c_uint {
        i = __osip_remove_nist_transaction(osip, tr);
    } else {
        return -(2 as libc::c_int)
    }
    return i;
}
pub unsafe extern "C" fn __osip_remove_ict_transaction(
    mut osip: *mut osip_t,
    mut ict: *mut osip_transaction_t,
) -> libc::c_int {
    let mut iterator: osip_list_iterator_t = osip_list_iterator_t {
        actual: 0 as *mut __node_t,
        prev: 0 as *mut *mut __node_t,
        li: 0 as *mut osip_list_t,
        pos: 0,
    };
    let mut tmp: *mut osip_transaction_t = 0 as *mut osip_transaction_t;
    osip_mutex_lock((*osip).ict_fastmutex as *mut osip_mutex);
    tmp = osip_list_get_first(&mut (*osip).osip_ict_transactions, &mut iterator)
        as *mut osip_transaction_t;
    while !(iterator.actual).is_null() && iterator.pos < (*iterator.li).nb_elt {
        if (*tmp).transactionid == (*ict).transactionid {
            osip_list_iterator_remove(&mut iterator);
            osip_mutex_unlock((*osip).ict_fastmutex as *mut osip_mutex);
            return 0 as libc::c_int;
        }
        tmp = osip_list_get_next(&mut iterator) as *mut osip_transaction_t;
    }
    osip_mutex_unlock((*osip).ict_fastmutex as *mut osip_mutex);
    return -(1 as libc::c_int);
}
pub unsafe extern "C" fn __osip_remove_ist_transaction(
    mut osip: *mut osip_t,
    mut ist: *mut osip_transaction_t,
) -> libc::c_int {
    let mut iterator: osip_list_iterator_t = osip_list_iterator_t {
        actual: 0 as *mut __node_t,
        prev: 0 as *mut *mut __node_t,
        li: 0 as *mut osip_list_t,
        pos: 0,
    };
    let mut tmp: *mut osip_transaction_t = 0 as *mut osip_transaction_t;
    osip_mutex_lock((*osip).ist_fastmutex as *mut osip_mutex);
    tmp = osip_list_get_first(&mut (*osip).osip_ist_transactions, &mut iterator)
        as *mut osip_transaction_t;
    while !(iterator.actual).is_null() && iterator.pos < (*iterator.li).nb_elt {
        if (*tmp).transactionid == (*ist).transactionid {
            osip_list_iterator_remove(&mut iterator);
            osip_mutex_unlock((*osip).ist_fastmutex as *mut osip_mutex);
            return 0 as libc::c_int;
        }
        tmp = osip_list_get_next(&mut iterator) as *mut osip_transaction_t;
    }
    osip_mutex_unlock((*osip).ist_fastmutex as *mut osip_mutex);
    return -(1 as libc::c_int);
}
pub unsafe extern "C" fn __osip_remove_nict_transaction(
    mut osip: *mut osip_t,
    mut nict: *mut osip_transaction_t,
) -> libc::c_int {
    let mut iterator: osip_list_iterator_t = osip_list_iterator_t {
        actual: 0 as *mut __node_t,
        prev: 0 as *mut *mut __node_t,
        li: 0 as *mut osip_list_t,
        pos: 0,
    };
    let mut tmp: *mut osip_transaction_t = 0 as *mut osip_transaction_t;
    osip_mutex_lock((*osip).nict_fastmutex as *mut osip_mutex);
    tmp = osip_list_get_first(&mut (*osip).osip_nict_transactions, &mut iterator)
        as *mut osip_transaction_t;
    while !(iterator.actual).is_null() && iterator.pos < (*iterator.li).nb_elt {
        if (*tmp).transactionid == (*nict).transactionid {
            osip_list_iterator_remove(&mut iterator);
            osip_mutex_unlock((*osip).nict_fastmutex as *mut osip_mutex);
            return 0 as libc::c_int;
        }
        tmp = osip_list_get_next(&mut iterator) as *mut osip_transaction_t;
    }
    osip_mutex_unlock((*osip).nict_fastmutex as *mut osip_mutex);
    return -(1 as libc::c_int);
}
pub unsafe extern "C" fn __osip_remove_nist_transaction(
    mut osip: *mut osip_t,
    mut nist: *mut osip_transaction_t,
) -> libc::c_int {
    let mut iterator: osip_list_iterator_t = osip_list_iterator_t {
        actual: 0 as *mut __node_t,
        prev: 0 as *mut *mut __node_t,
        li: 0 as *mut osip_list_t,
        pos: 0,
    };
    let mut tmp: *mut osip_transaction_t = 0 as *mut osip_transaction_t;
    osip_mutex_lock((*osip).nist_fastmutex as *mut osip_mutex);
    tmp = osip_list_get_first(&mut (*osip).osip_nist_transactions, &mut iterator)
        as *mut osip_transaction_t;
    while !(iterator.actual).is_null() && iterator.pos < (*iterator.li).nb_elt {
        if (*tmp).transactionid == (*nist).transactionid {
            osip_list_iterator_remove(&mut iterator);
            osip_mutex_unlock((*osip).nist_fastmutex as *mut osip_mutex);
            return 0 as libc::c_int;
        }
        tmp = osip_list_get_next(&mut iterator) as *mut osip_transaction_t;
    }
    osip_mutex_unlock((*osip).nist_fastmutex as *mut osip_mutex);
    return -(1 as libc::c_int);
}
pub unsafe extern "C" fn osip_find_transaction_and_add_event(
    mut osip: *mut osip_t,
    mut evt: *mut osip_event_t,
) -> libc::c_int {
    let mut transaction: *mut osip_transaction_t = __osip_find_transaction(
        osip,
        evt,
        1 as libc::c_int,
    );
    if transaction.is_null() {
        return -(1 as libc::c_int);
    }
    return 0 as libc::c_int;
}
pub unsafe extern "C" fn __osip_find_transaction(
    mut osip: *mut osip_t,
    mut evt: *mut osip_event_t,
    mut consume: libc::c_int,
) -> *mut osip_transaction_t {
    let mut transaction: *mut osip_transaction_t = 0 as *mut osip_transaction_t;
    let mut transactions: *mut osip_list_t = 0 as *mut osip_list_t;
    let mut mut_0: *mut osip_mutex = 0 as *mut osip_mutex;
    if evt.is_null() || ((*evt).sip).is_null() || ((*(*evt).sip).cseq).is_null() {
        return 0 as *mut osip_transaction_t;
    }
    if (*evt).type_0 as libc::c_uint >= RCV_REQINVITE as libc::c_int as libc::c_uint
        && (*evt).type_0 as libc::c_uint
            <= RCV_STATUS_3456XX as libc::c_int as libc::c_uint
    {
        if (*(*evt).sip).status_code == 0 as libc::c_int {
            if 0 as libc::c_int
                == strcmp(
                    (*(*(*evt).sip).cseq).method,
                    b"INVITE\0" as *const u8 as *const libc::c_char,
                )
                || 0 as libc::c_int
                    == strcmp(
                        (*(*(*evt).sip).cseq).method,
                        b"ACK\0" as *const u8 as *const libc::c_char,
                    )
            {
                transactions = &mut (*osip).osip_ist_transactions;
                mut_0 = (*osip).ist_fastmutex as *mut osip_mutex;
            } else {
                transactions = &mut (*osip).osip_nist_transactions;
                mut_0 = (*osip).nist_fastmutex as *mut osip_mutex;
            }
        } else if 0 as libc::c_int
            == strcmp(
                (*(*(*evt).sip).cseq).method,
                b"INVITE\0" as *const u8 as *const libc::c_char,
            )
        {
            transactions = &mut (*osip).osip_ict_transactions;
            mut_0 = (*osip).ict_fastmutex as *mut osip_mutex;
        } else {
            transactions = &mut (*osip).osip_nict_transactions;
            mut_0 = (*osip).nict_fastmutex as *mut osip_mutex;
        }
    } else if (*evt).type_0 as libc::c_uint
        >= SND_REQINVITE as libc::c_int as libc::c_uint
        && (*evt).type_0 as libc::c_uint
            <= SND_STATUS_3456XX as libc::c_int as libc::c_uint
    {
        if (*(*evt).sip).status_code != 0 as libc::c_int {
            if 0 as libc::c_int
                == strcmp(
                    (*(*(*evt).sip).cseq).method,
                    b"INVITE\0" as *const u8 as *const libc::c_char,
                )
            {
                transactions = &mut (*osip).osip_ist_transactions;
                mut_0 = (*osip).ist_fastmutex as *mut osip_mutex;
            } else {
                transactions = &mut (*osip).osip_nist_transactions;
                mut_0 = (*osip).nist_fastmutex as *mut osip_mutex;
            }
        } else if 0 as libc::c_int
            == strcmp(
                (*(*(*evt).sip).cseq).method,
                b"INVITE\0" as *const u8 as *const libc::c_char,
            )
            || 0 as libc::c_int
                == strcmp(
                    (*(*(*evt).sip).cseq).method,
                    b"ACK\0" as *const u8 as *const libc::c_char,
                )
        {
            transactions = &mut (*osip).osip_ict_transactions;
            mut_0 = (*osip).ict_fastmutex as *mut osip_mutex;
        } else {
            transactions = &mut (*osip).osip_nict_transactions;
            mut_0 = (*osip).nict_fastmutex as *mut osip_mutex;
        }
    }
    if transactions.is_null() {
        return 0 as *mut osip_transaction_t;
    }
    osip_mutex_lock(mut_0);
    transaction = osip_transaction_find(transactions, evt);
    if consume == 1 as libc::c_int {
        if !transaction.is_null() {
            osip_transaction_add_event(transaction, evt);
            osip_mutex_unlock(mut_0);
            return transaction;
        }
    }
    osip_mutex_unlock(mut_0);
    return transaction;
}
pub unsafe extern "C" fn osip_create_transaction(
    mut osip: *mut osip_t,
    mut evt: *mut osip_event_t,
) -> *mut osip_transaction_t {
    let mut transaction: *mut osip_transaction_t = 0 as *mut osip_transaction_t;
    let mut i: libc::c_int = 0;
    let mut ctx_type: osip_fsm_type_t = ICT;
    if evt.is_null() {
        return 0 as *mut osip_transaction_t;
    }
    if ((*evt).sip).is_null() {
        return 0 as *mut osip_transaction_t;
    }
    if (*(*evt).sip).status_code == 0 as libc::c_int {
        if ((*(*evt).sip).cseq).is_null() || ((*(*(*evt).sip).cseq).method).is_null()
            || ((*(*evt).sip).sip_method).is_null()
        {
            return 0 as *mut osip_transaction_t;
        }
        if 0 as libc::c_int
            != strcmp((*(*(*evt).sip).cseq).method, (*(*evt).sip).sip_method)
        {
            osip_trace(
                b"osip.c\0" as *const u8 as *const libc::c_char,
                828 as libc::c_int,
                TRACE_LEVEL3,
                0 as *mut FILE,
                b"core module: Discard invalid message with method!=cseq!\n\0"
                    as *const u8 as *const libc::c_char,
            );
            return 0 as *mut osip_transaction_t;
        }
    }
    if (*(*evt).sip).status_code == 0 as libc::c_int
        && 0 as libc::c_int
            == strcmp(
                (*(*evt).sip).sip_method,
                b"ACK\0" as *const u8 as *const libc::c_char,
            )
    {
        return 0 as *mut osip_transaction_t;
    }
    if (*evt).type_0 as libc::c_uint == RCV_REQINVITE as libc::c_int as libc::c_uint
        || (*evt).type_0 as libc::c_uint == RCV_REQACK as libc::c_int as libc::c_uint
        || (*evt).type_0 as libc::c_uint == RCV_REQUEST as libc::c_int as libc::c_uint
    {
        if 0 as libc::c_int
            == strcmp(
                (*(*(*evt).sip).cseq).method,
                b"INVITE\0" as *const u8 as *const libc::c_char,
            )
        {
            ctx_type = IST;
        } else {
            ctx_type = NIST;
        }
    } else if (*evt).type_0 as libc::c_uint
        == SND_REQINVITE as libc::c_int as libc::c_uint
        || (*evt).type_0 as libc::c_uint == SND_REQACK as libc::c_int as libc::c_uint
        || (*evt).type_0 as libc::c_uint == SND_REQUEST as libc::c_int as libc::c_uint
    {
        if 0 as libc::c_int
            == strcmp(
                (*(*(*evt).sip).cseq).method,
                b"INVITE\0" as *const u8 as *const libc::c_char,
            )
        {
            ctx_type = ICT;
        } else {
            ctx_type = NICT;
        }
    } else {
        osip_trace(
            b"osip.c\0" as *const u8 as *const libc::c_char,
            852 as libc::c_int,
            TRACE_LEVEL2,
            0 as *mut FILE,
            b"Cannot build a transaction for this message!\n\0" as *const u8
                as *const libc::c_char,
        );
        return 0 as *mut osip_transaction_t;
    }
    i = osip_transaction_init(&mut transaction, ctx_type, osip, (*evt).sip);
    if i != 0 as libc::c_int {
        return 0 as *mut osip_transaction_t;
    }
    (*evt).transactionid = (*transaction).transactionid;
    return transaction;
}
pub unsafe extern "C" fn osip_transaction_find(
    mut transactions: *mut osip_list_t,
    mut evt: *mut osip_event_t,
) -> *mut osip_transaction_t {
    let mut iterator: osip_list_iterator_t = osip_list_iterator_t {
        actual: 0 as *mut __node_t,
        prev: 0 as *mut *mut __node_t,
        li: 0 as *mut osip_list_t,
        pos: 0,
    };
    let mut transaction: *mut osip_transaction_t = 0 as *mut osip_transaction_t;
    let mut osip: *mut osip_t = 0 as *mut osip_t;
    transaction = osip_list_get_first(transactions, &mut iterator)
        as *mut osip_transaction_t;
    if !transaction.is_null() {
        osip = (*transaction).config as *mut osip_t;
    }
    if osip.is_null() {
        return 0 as *mut osip_transaction_t;
    }
    if (*evt).type_0 as libc::c_uint == RCV_REQINVITE as libc::c_int as libc::c_uint
        || (*evt).type_0 as libc::c_uint == RCV_REQACK as libc::c_int as libc::c_uint
        || (*evt).type_0 as libc::c_uint == RCV_REQUEST as libc::c_int as libc::c_uint
    {
        transaction = osip_list_get_first(transactions, &mut iterator)
            as *mut osip_transaction_t;
        while !(iterator.actual).is_null() && iterator.pos < (*iterator.li).nb_elt {
            if 0 as libc::c_int
                == __osip_transaction_matching_request_osip_to_xist_17_2_3(
                    transaction,
                    (*evt).sip,
                )
            {
                return transaction;
            }
            transaction = osip_list_get_next(&mut iterator) as *mut osip_transaction_t;
        }
    } else if (*evt).type_0 as libc::c_uint
        == RCV_STATUS_1XX as libc::c_int as libc::c_uint
        || (*evt).type_0 as libc::c_uint == RCV_STATUS_2XX as libc::c_int as libc::c_uint
        || (*evt).type_0 as libc::c_uint
            == RCV_STATUS_3456XX as libc::c_int as libc::c_uint
    {
        transaction = osip_list_get_first(transactions, &mut iterator)
            as *mut osip_transaction_t;
        while !(iterator.actual).is_null() && iterator.pos < (*iterator.li).nb_elt {
            if 0 as libc::c_int
                == __osip_transaction_matching_response_osip_to_xict_17_1_3(
                    transaction,
                    (*evt).sip,
                )
            {
                return transaction;
            }
            transaction = osip_list_get_next(&mut iterator) as *mut osip_transaction_t;
        }
    } else {
        transaction = osip_list_get_first(transactions, &mut iterator)
            as *mut osip_transaction_t;
        while !(iterator.actual).is_null() && iterator.pos < (*iterator.li).nb_elt {
            if (*transaction).transactionid == (*evt).transactionid {
                return transaction;
            }
            transaction = osip_list_get_next(&mut iterator) as *mut osip_transaction_t;
        }
    }
    return 0 as *mut osip_transaction_t;
}
pub unsafe extern "C" fn osip_init(mut osip: *mut *mut osip_t) -> libc::c_int {
    static mut ref_count: libc::c_int = 0 as libc::c_int;
    if ref_count == 0 as libc::c_int {
        ref_count += 1;
        ref_count;
        parser_init();
    }
    *osip = (if osip_malloc_func.is_some() {
        osip_malloc_func.unwrap()(::std::mem::size_of::<osip_t>() as libc::c_ulong)
    } else {
        malloc(::std::mem::size_of::<osip_t>() as libc::c_ulong)
    }) as *mut osip_t;
    if (*osip).is_null() {
        return -(4 as libc::c_int);
    }
    memset(
        *osip as *mut libc::c_void,
        0 as libc::c_int,
        ::std::mem::size_of::<osip_t>() as libc::c_ulong,
    );
    (**osip).ict_fastmutex = osip_mutex_init() as *mut libc::c_void;
    (**osip).ist_fastmutex = osip_mutex_init() as *mut libc::c_void;
    (**osip).nict_fastmutex = osip_mutex_init() as *mut libc::c_void;
    (**osip).nist_fastmutex = osip_mutex_init() as *mut libc::c_void;
    (**osip).ixt_fastmutex = osip_mutex_init() as *mut libc::c_void;
    (**osip).id_mutex = osip_mutex_init() as *mut libc::c_void;
    osip_list_init(&mut (**osip).osip_ict_transactions);
    osip_list_init(&mut (**osip).osip_ist_transactions);
    osip_list_init(&mut (**osip).osip_nict_transactions);
    osip_list_init(&mut (**osip).osip_nist_transactions);
    osip_list_init(&mut (**osip).ixt_retransmissions);
    (**osip).transactionid = 1 as libc::c_int;
    return 0 as libc::c_int;
}
pub unsafe extern "C" fn osip_release(mut osip: *mut osip_t) {
    osip_mutex_destroy((*osip).ict_fastmutex as *mut osip_mutex);
    osip_mutex_destroy((*osip).ist_fastmutex as *mut osip_mutex);
    osip_mutex_destroy((*osip).nict_fastmutex as *mut osip_mutex);
    osip_mutex_destroy((*osip).nist_fastmutex as *mut osip_mutex);
    osip_mutex_destroy((*osip).ixt_fastmutex as *mut osip_mutex);
    osip_mutex_destroy((*osip).id_mutex as *mut osip_mutex);
    if !osip.is_null() {
        if osip_free_func.is_some() {
            osip_free_func.unwrap()(osip as *mut libc::c_void);
        } else {
            free(osip as *mut libc::c_void);
        }
    }
}
pub unsafe extern "C" fn osip_set_application_context(
    mut osip: *mut osip_t,
    mut pointer: *mut libc::c_void,
) {
    (*osip).application_context = pointer;
}
pub unsafe extern "C" fn osip_get_application_context(
    mut osip: *mut osip_t,
) -> *mut libc::c_void {
    if osip.is_null() {
        return 0 as *mut libc::c_void;
    }
    return (*osip).application_context;
}
pub unsafe extern "C" fn osip_ict_execute(mut osip: *mut osip_t) -> libc::c_int {
    let mut transaction: *mut osip_transaction_t = 0 as *mut osip_transaction_t;
    let mut se: *mut osip_event_t = 0 as *mut osip_event_t;
    let mut more_event: libc::c_int = 0;
    let mut iterator: osip_list_iterator_t = osip_list_iterator_t {
        actual: 0 as *mut __node_t,
        prev: 0 as *mut *mut __node_t,
        li: 0 as *mut osip_list_t,
        pos: 0,
    };
    let mut array: *mut *mut libc::c_void = 0 as *mut *mut libc::c_void;
    let mut len: libc::c_int = 0;
    let mut index: libc::c_int = 0 as libc::c_int;
    osip_mutex_lock((*osip).ict_fastmutex as *mut osip_mutex);
    len = osip_list_size(&mut (*osip).osip_ict_transactions);
    if 0 as libc::c_int >= len {
        osip_mutex_unlock((*osip).ict_fastmutex as *mut osip_mutex);
        return 0 as libc::c_int;
    }
    array = (if osip_malloc_func.is_some() {
        osip_malloc_func
            .unwrap()(
            (::std::mem::size_of::<*mut libc::c_void>() as libc::c_ulong)
                .wrapping_mul(len as libc::c_ulong),
        )
    } else {
        malloc(
            (::std::mem::size_of::<*mut libc::c_void>() as libc::c_ulong)
                .wrapping_mul(len as libc::c_ulong),
        )
    }) as *mut *mut libc::c_void;
    if array.is_null() {
        osip_mutex_unlock((*osip).ict_fastmutex as *mut osip_mutex);
        return -(4 as libc::c_int);
    }
    transaction = osip_list_get_first(&mut (*osip).osip_ict_transactions, &mut iterator)
        as *mut osip_transaction_t;
    while !(iterator.actual).is_null() && iterator.pos < (*iterator.li).nb_elt {
        let fresh0 = index;
        index = index + 1;
        let ref mut fresh1 = *array.offset(fresh0 as isize);
        *fresh1 = transaction as *mut libc::c_void;
        transaction = osip_list_get_next(&mut iterator) as *mut osip_transaction_t;
    }
    osip_mutex_unlock((*osip).ict_fastmutex as *mut osip_mutex);
    index = 0 as libc::c_int;
    while index < len {
        transaction = *array.offset(index as isize) as *mut osip_transaction_t;
        more_event = 1 as libc::c_int;
        loop {
            se = osip_fifo_tryget((*transaction).transactionff) as *mut osip_event_t;
            if se.is_null() {
                more_event = 0 as libc::c_int;
            } else {
                osip_transaction_execute(transaction, se);
            }
            if !(more_event == 1 as libc::c_int) {
                break;
            }
        }
        index += 1;
        index;
    }
    if !array.is_null() {
        if osip_free_func.is_some() {
            osip_free_func.unwrap()(array as *mut libc::c_void);
        } else {
            free(array as *mut libc::c_void);
        }
    }
    return 0 as libc::c_int;
}
pub unsafe extern "C" fn osip_ist_execute(mut osip: *mut osip_t) -> libc::c_int {
    let mut transaction: *mut osip_transaction_t = 0 as *mut osip_transaction_t;
    let mut se: *mut osip_event_t = 0 as *mut osip_event_t;
    let mut more_event: libc::c_int = 0;
    let mut iterator: osip_list_iterator_t = osip_list_iterator_t {
        actual: 0 as *mut __node_t,
        prev: 0 as *mut *mut __node_t,
        li: 0 as *mut osip_list_t,
        pos: 0,
    };
    let mut array: *mut *mut libc::c_void = 0 as *mut *mut libc::c_void;
    let mut len: libc::c_int = 0;
    let mut index: libc::c_int = 0 as libc::c_int;
    osip_mutex_lock((*osip).ist_fastmutex as *mut osip_mutex);
    len = osip_list_size(&mut (*osip).osip_ist_transactions);
    if 0 as libc::c_int >= len {
        osip_mutex_unlock((*osip).ist_fastmutex as *mut osip_mutex);
        return 0 as libc::c_int;
    }
    array = (if osip_malloc_func.is_some() {
        osip_malloc_func
            .unwrap()(
            (::std::mem::size_of::<*mut libc::c_void>() as libc::c_ulong)
                .wrapping_mul(len as libc::c_ulong),
        )
    } else {
        malloc(
            (::std::mem::size_of::<*mut libc::c_void>() as libc::c_ulong)
                .wrapping_mul(len as libc::c_ulong),
        )
    }) as *mut *mut libc::c_void;
    if array.is_null() {
        osip_mutex_unlock((*osip).ist_fastmutex as *mut osip_mutex);
        return -(4 as libc::c_int);
    }
    transaction = osip_list_get_first(&mut (*osip).osip_ist_transactions, &mut iterator)
        as *mut osip_transaction_t;
    while !(iterator.actual).is_null() && iterator.pos < (*iterator.li).nb_elt {
        let fresh2 = index;
        index = index + 1;
        let ref mut fresh3 = *array.offset(fresh2 as isize);
        *fresh3 = transaction as *mut libc::c_void;
        transaction = osip_list_get_next(&mut iterator) as *mut osip_transaction_t;
    }
    osip_mutex_unlock((*osip).ist_fastmutex as *mut osip_mutex);
    index = 0 as libc::c_int;
    while index < len {
        transaction = *array.offset(index as isize) as *mut osip_transaction_t;
        more_event = 1 as libc::c_int;
        loop {
            se = osip_fifo_tryget((*transaction).transactionff) as *mut osip_event_t;
            if se.is_null() {
                more_event = 0 as libc::c_int;
            } else {
                osip_transaction_execute(transaction, se);
            }
            if !(more_event == 1 as libc::c_int) {
                break;
            }
        }
        index += 1;
        index;
    }
    if !array.is_null() {
        if osip_free_func.is_some() {
            osip_free_func.unwrap()(array as *mut libc::c_void);
        } else {
            free(array as *mut libc::c_void);
        }
    }
    return 0 as libc::c_int;
}
pub unsafe extern "C" fn osip_nict_execute(mut osip: *mut osip_t) -> libc::c_int {
    let mut transaction: *mut osip_transaction_t = 0 as *mut osip_transaction_t;
    let mut se: *mut osip_event_t = 0 as *mut osip_event_t;
    let mut more_event: libc::c_int = 0;
    let mut iterator: osip_list_iterator_t = osip_list_iterator_t {
        actual: 0 as *mut __node_t,
        prev: 0 as *mut *mut __node_t,
        li: 0 as *mut osip_list_t,
        pos: 0,
    };
    let mut array: *mut *mut libc::c_void = 0 as *mut *mut libc::c_void;
    let mut len: libc::c_int = 0;
    let mut index: libc::c_int = 0 as libc::c_int;
    osip_mutex_lock((*osip).nict_fastmutex as *mut osip_mutex);
    len = osip_list_size(&mut (*osip).osip_nict_transactions);
    if 0 as libc::c_int >= len {
        osip_mutex_unlock((*osip).nict_fastmutex as *mut osip_mutex);
        return 0 as libc::c_int;
    }
    array = (if osip_malloc_func.is_some() {
        osip_malloc_func
            .unwrap()(
            (::std::mem::size_of::<*mut libc::c_void>() as libc::c_ulong)
                .wrapping_mul(len as libc::c_ulong),
        )
    } else {
        malloc(
            (::std::mem::size_of::<*mut libc::c_void>() as libc::c_ulong)
                .wrapping_mul(len as libc::c_ulong),
        )
    }) as *mut *mut libc::c_void;
    if array.is_null() {
        osip_mutex_unlock((*osip).nict_fastmutex as *mut osip_mutex);
        return -(4 as libc::c_int);
    }
    transaction = osip_list_get_first(&mut (*osip).osip_nict_transactions, &mut iterator)
        as *mut osip_transaction_t;
    while !(iterator.actual).is_null() && iterator.pos < (*iterator.li).nb_elt {
        let fresh4 = index;
        index = index + 1;
        let ref mut fresh5 = *array.offset(fresh4 as isize);
        *fresh5 = transaction as *mut libc::c_void;
        transaction = osip_list_get_next(&mut iterator) as *mut osip_transaction_t;
    }
    osip_mutex_unlock((*osip).nict_fastmutex as *mut osip_mutex);
    index = 0 as libc::c_int;
    while index < len {
        transaction = *array.offset(index as isize) as *mut osip_transaction_t;
        more_event = 1 as libc::c_int;
        loop {
            se = osip_fifo_tryget((*transaction).transactionff) as *mut osip_event_t;
            if se.is_null() {
                more_event = 0 as libc::c_int;
            } else {
                osip_transaction_execute(transaction, se);
            }
            if !(more_event == 1 as libc::c_int) {
                break;
            }
        }
        index += 1;
        index;
    }
    if !array.is_null() {
        if osip_free_func.is_some() {
            osip_free_func.unwrap()(array as *mut libc::c_void);
        } else {
            free(array as *mut libc::c_void);
        }
    }
    return 0 as libc::c_int;
}
pub unsafe extern "C" fn osip_nist_execute(mut osip: *mut osip_t) -> libc::c_int {
    let mut transaction: *mut osip_transaction_t = 0 as *mut osip_transaction_t;
    let mut se: *mut osip_event_t = 0 as *mut osip_event_t;
    let mut more_event: libc::c_int = 0;
    let mut iterator: osip_list_iterator_t = osip_list_iterator_t {
        actual: 0 as *mut __node_t,
        prev: 0 as *mut *mut __node_t,
        li: 0 as *mut osip_list_t,
        pos: 0,
    };
    let mut array: *mut *mut libc::c_void = 0 as *mut *mut libc::c_void;
    let mut len: libc::c_int = 0;
    let mut index: libc::c_int = 0 as libc::c_int;
    osip_mutex_lock((*osip).nist_fastmutex as *mut osip_mutex);
    len = osip_list_size(&mut (*osip).osip_nist_transactions);
    if 0 as libc::c_int >= len {
        osip_mutex_unlock((*osip).nist_fastmutex as *mut osip_mutex);
        return 0 as libc::c_int;
    }
    array = (if osip_malloc_func.is_some() {
        osip_malloc_func
            .unwrap()(
            (::std::mem::size_of::<*mut libc::c_void>() as libc::c_ulong)
                .wrapping_mul(len as libc::c_ulong),
        )
    } else {
        malloc(
            (::std::mem::size_of::<*mut libc::c_void>() as libc::c_ulong)
                .wrapping_mul(len as libc::c_ulong),
        )
    }) as *mut *mut libc::c_void;
    if array.is_null() {
        osip_mutex_unlock((*osip).nist_fastmutex as *mut osip_mutex);
        return -(4 as libc::c_int);
    }
    transaction = osip_list_get_first(&mut (*osip).osip_nist_transactions, &mut iterator)
        as *mut osip_transaction_t;
    while !(iterator.actual).is_null() && iterator.pos < (*iterator.li).nb_elt {
        let fresh6 = index;
        index = index + 1;
        let ref mut fresh7 = *array.offset(fresh6 as isize);
        *fresh7 = transaction as *mut libc::c_void;
        transaction = osip_list_get_next(&mut iterator) as *mut osip_transaction_t;
    }
    osip_mutex_unlock((*osip).nist_fastmutex as *mut osip_mutex);
    index = 0 as libc::c_int;
    while index < len {
        transaction = *array.offset(index as isize) as *mut osip_transaction_t;
        more_event = 1 as libc::c_int;
        loop {
            se = osip_fifo_tryget((*transaction).transactionff) as *mut osip_event_t;
            if se.is_null() {
                more_event = 0 as libc::c_int;
            } else {
                osip_transaction_execute(transaction, se);
            }
            if !(more_event == 1 as libc::c_int) {
                break;
            }
        }
        index += 1;
        index;
    }
    if !array.is_null() {
        if osip_free_func.is_some() {
            osip_free_func.unwrap()(array as *mut libc::c_void);
        } else {
            free(array as *mut libc::c_void);
        }
    }
    return 0 as libc::c_int;
}
pub unsafe extern "C" fn osip_timers_gettimeout(
    mut osip: *mut osip_t,
    mut lower_tv: *mut timeval,
) {
    let mut now: timeval = timeval { tv_sec: 0, tv_usec: 0 };
    let mut tr: *mut osip_transaction_t = 0 as *mut osip_transaction_t;
    let mut iterator: osip_list_iterator_t = osip_list_iterator_t {
        actual: 0 as *mut __node_t,
        prev: 0 as *mut *mut __node_t,
        li: 0 as *mut osip_list_t,
        pos: 0,
    };
    osip_gettimeofday(&mut now, 0 as *mut libc::c_void);
    (*lower_tv)
        .tv_sec = now.tv_sec
        + (3600 as libc::c_int * 24 as libc::c_int * 365 as libc::c_int) as libc::c_long;
    (*lower_tv).tv_usec = now.tv_usec;
    osip_mutex_lock((*osip).ict_fastmutex as *mut osip_mutex);
    tr = osip_list_get_first(&mut (*osip).osip_ict_transactions, &mut iterator)
        as *mut osip_transaction_t;
    while !(iterator.actual).is_null() && iterator.pos < (*iterator.li).nb_elt {
        if 1 as libc::c_int <= osip_fifo_size((*tr).transactionff) {
            osip_trace(
                b"osip.c\0" as *const u8 as *const libc::c_char,
                1314 as libc::c_int,
                TRACE_LEVEL7,
                0 as *mut FILE,
                b"1 Pending event already in transaction !\n\0" as *const u8
                    as *const libc::c_char,
            );
            (*lower_tv).tv_sec = 0 as libc::c_int as __time_t;
            (*lower_tv).tv_usec = 0 as libc::c_int as __suseconds_t;
            osip_mutex_unlock((*osip).ict_fastmutex as *mut osip_mutex);
            return;
        } else {
            if (*tr).state as libc::c_uint == ICT_CALLING as libc::c_int as libc::c_uint
            {
                min_timercmp(lower_tv, &mut (*(*tr).ict_context).timer_b_start);
            }
            if (*tr).state as libc::c_uint == ICT_CALLING as libc::c_int as libc::c_uint
            {
                min_timercmp(lower_tv, &mut (*(*tr).ict_context).timer_a_start);
            }
            if (*tr).state as libc::c_uint
                == ICT_COMPLETED as libc::c_int as libc::c_uint
            {
                min_timercmp(lower_tv, &mut (*(*tr).ict_context).timer_d_start);
            }
            if if now.tv_sec == (*lower_tv).tv_sec {
                (now.tv_usec >= (*lower_tv).tv_usec) as libc::c_int
            } else {
                (now.tv_sec >= (*lower_tv).tv_sec) as libc::c_int
            } != 0
            {
                (*lower_tv).tv_sec = 0 as libc::c_int as __time_t;
                (*lower_tv).tv_usec = 0 as libc::c_int as __suseconds_t;
                osip_mutex_unlock((*osip).ict_fastmutex as *mut osip_mutex);
                return;
            }
        }
        tr = osip_list_get_next(&mut iterator) as *mut osip_transaction_t;
    }
    osip_mutex_unlock((*osip).ict_fastmutex as *mut osip_mutex);
    osip_mutex_lock((*osip).ist_fastmutex as *mut osip_mutex);
    tr = osip_list_get_first(&mut (*osip).osip_ist_transactions, &mut iterator)
        as *mut osip_transaction_t;
    while !(iterator.actual).is_null() && iterator.pos < (*iterator.li).nb_elt {
        if (*tr).state as libc::c_uint == IST_CONFIRMED as libc::c_int as libc::c_uint {
            min_timercmp(lower_tv, &mut (*(*tr).ist_context).timer_i_start);
        }
        if (*tr).state as libc::c_uint == IST_COMPLETED as libc::c_int as libc::c_uint {
            min_timercmp(lower_tv, &mut (*(*tr).ist_context).timer_h_start);
        }
        if (*tr).state as libc::c_uint == IST_COMPLETED as libc::c_int as libc::c_uint {
            min_timercmp(lower_tv, &mut (*(*tr).ist_context).timer_g_start);
        }
        if if now.tv_sec == (*lower_tv).tv_sec {
            (now.tv_usec >= (*lower_tv).tv_usec) as libc::c_int
        } else {
            (now.tv_sec >= (*lower_tv).tv_sec) as libc::c_int
        } != 0
        {
            (*lower_tv).tv_sec = 0 as libc::c_int as __time_t;
            (*lower_tv).tv_usec = 0 as libc::c_int as __suseconds_t;
            osip_mutex_unlock((*osip).ist_fastmutex as *mut osip_mutex);
            return;
        }
        tr = osip_list_get_next(&mut iterator) as *mut osip_transaction_t;
    }
    osip_mutex_unlock((*osip).ist_fastmutex as *mut osip_mutex);
    osip_mutex_lock((*osip).nict_fastmutex as *mut osip_mutex);
    tr = osip_list_get_first(&mut (*osip).osip_nict_transactions, &mut iterator)
        as *mut osip_transaction_t;
    while !(iterator.actual).is_null() && iterator.pos < (*iterator.li).nb_elt {
        if (*tr).state as libc::c_uint == NICT_COMPLETED as libc::c_int as libc::c_uint {
            min_timercmp(lower_tv, &mut (*(*tr).nict_context).timer_k_start);
        }
        if (*tr).state as libc::c_uint == NICT_PROCEEDING as libc::c_int as libc::c_uint
            || (*tr).state as libc::c_uint == NICT_TRYING as libc::c_int as libc::c_uint
        {
            min_timercmp(lower_tv, &mut (*(*tr).nict_context).timer_f_start);
        }
        if (*tr).state as libc::c_uint == NICT_PROCEEDING as libc::c_int as libc::c_uint
            || (*tr).state as libc::c_uint == NICT_TRYING as libc::c_int as libc::c_uint
        {
            min_timercmp(lower_tv, &mut (*(*tr).nict_context).timer_e_start);
        }
        if if now.tv_sec == (*lower_tv).tv_sec {
            (now.tv_usec >= (*lower_tv).tv_usec) as libc::c_int
        } else {
            (now.tv_sec >= (*lower_tv).tv_sec) as libc::c_int
        } != 0
        {
            (*lower_tv).tv_sec = 0 as libc::c_int as __time_t;
            (*lower_tv).tv_usec = 0 as libc::c_int as __suseconds_t;
            osip_mutex_unlock((*osip).nict_fastmutex as *mut osip_mutex);
            return;
        }
        tr = osip_list_get_next(&mut iterator) as *mut osip_transaction_t;
    }
    osip_mutex_unlock((*osip).nict_fastmutex as *mut osip_mutex);
    osip_mutex_lock((*osip).nist_fastmutex as *mut osip_mutex);
    tr = osip_list_get_first(&mut (*osip).osip_nist_transactions, &mut iterator)
        as *mut osip_transaction_t;
    while !(iterator.actual).is_null() && iterator.pos < (*iterator.li).nb_elt {
        if (*tr).state as libc::c_uint == NIST_COMPLETED as libc::c_int as libc::c_uint {
            min_timercmp(lower_tv, &mut (*(*tr).nist_context).timer_j_start);
        }
        if if now.tv_sec == (*lower_tv).tv_sec {
            (now.tv_usec >= (*lower_tv).tv_usec) as libc::c_int
        } else {
            (now.tv_sec >= (*lower_tv).tv_sec) as libc::c_int
        } != 0
        {
            (*lower_tv).tv_sec = 0 as libc::c_int as __time_t;
            (*lower_tv).tv_usec = 0 as libc::c_int as __suseconds_t;
            osip_mutex_unlock((*osip).nist_fastmutex as *mut osip_mutex);
            return;
        }
        tr = osip_list_get_next(&mut iterator) as *mut osip_transaction_t;
    }
    osip_mutex_unlock((*osip).nist_fastmutex as *mut osip_mutex);
    osip_mutex_lock((*osip).ixt_fastmutex as *mut osip_mutex);
    let mut ixt: *mut ixt_t = 0 as *mut ixt_t;
    ixt = osip_list_get_first(&mut (*osip).ixt_retransmissions, &mut iterator)
        as *mut ixt_t;
    while !(iterator.actual).is_null() && iterator.pos < (*iterator.li).nb_elt {
        min_timercmp(lower_tv, &mut (*ixt).start);
        if if now.tv_sec == (*lower_tv).tv_sec {
            (now.tv_usec >= (*lower_tv).tv_usec) as libc::c_int
        } else {
            (now.tv_sec >= (*lower_tv).tv_sec) as libc::c_int
        } != 0
        {
            (*lower_tv).tv_sec = 0 as libc::c_int as __time_t;
            (*lower_tv).tv_usec = 0 as libc::c_int as __suseconds_t;
            osip_mutex_unlock((*osip).ixt_fastmutex as *mut osip_mutex);
            return;
        }
        ixt = osip_list_get_next(&mut iterator) as *mut ixt_t;
    }
    osip_mutex_unlock((*osip).ixt_fastmutex as *mut osip_mutex);
    (*lower_tv).tv_sec = (*lower_tv).tv_sec - now.tv_sec;
    (*lower_tv).tv_usec = (*lower_tv).tv_usec - now.tv_usec;
    if (*lower_tv).tv_usec < 0 as libc::c_int as libc::c_long {
        (*lower_tv)
            .tv_usec = (*lower_tv).tv_usec + 1000000 as libc::c_int as libc::c_long;
        (*lower_tv).tv_sec -= 1;
        (*lower_tv).tv_sec;
    }
    if (*lower_tv).tv_sec < 0 as libc::c_int as libc::c_long {
        (*lower_tv).tv_sec = 0 as libc::c_int as __time_t;
        (*lower_tv).tv_usec = 0 as libc::c_int as __suseconds_t;
    }
    if (*lower_tv).tv_usec > 1000000 as libc::c_int as libc::c_long {
        (*lower_tv)
            .tv_usec = (*lower_tv).tv_usec - 1000000 as libc::c_int as libc::c_long;
        (*lower_tv).tv_sec += 1;
        (*lower_tv).tv_sec;
    }
}
pub unsafe extern "C" fn osip_timers_ict_execute(mut osip: *mut osip_t) {
    let mut tr: *mut osip_transaction_t = 0 as *mut osip_transaction_t;
    let mut iterator: osip_list_iterator_t = osip_list_iterator_t {
        actual: 0 as *mut __node_t,
        prev: 0 as *mut *mut __node_t,
        li: 0 as *mut osip_list_t,
        pos: 0,
    };
    osip_mutex_lock((*osip).ict_fastmutex as *mut osip_mutex);
    tr = osip_list_get_first(&mut (*osip).osip_ict_transactions, &mut iterator)
        as *mut osip_transaction_t;
    while !(iterator.actual).is_null() && iterator.pos < (*iterator.li).nb_elt {
        let mut evt: *mut osip_event_t = 0 as *mut osip_event_t;
        if 1 as libc::c_int <= osip_fifo_size((*tr).transactionff) {
            osip_trace(
                b"osip.c\0" as *const u8 as *const libc::c_char,
                1502 as libc::c_int,
                TRACE_LEVEL7,
                0 as *mut FILE,
                b"1 Pending event already in transaction !\n\0" as *const u8
                    as *const libc::c_char,
            );
        } else {
            evt = __osip_ict_need_timer_b_event(
                (*tr).ict_context,
                (*tr).state,
                (*tr).transactionid,
            );
            if !evt.is_null() {
                osip_fifo_add((*tr).transactionff, evt as *mut libc::c_void);
            } else {
                evt = __osip_ict_need_timer_a_event(
                    (*tr).ict_context,
                    (*tr).state,
                    (*tr).transactionid,
                );
                if !evt.is_null() {
                    osip_fifo_add((*tr).transactionff, evt as *mut libc::c_void);
                } else {
                    evt = __osip_ict_need_timer_d_event(
                        (*tr).ict_context,
                        (*tr).state,
                        (*tr).transactionid,
                    );
                    if !evt.is_null() {
                        osip_fifo_add((*tr).transactionff, evt as *mut libc::c_void);
                    }
                }
            }
        }
        tr = osip_list_get_next(&mut iterator) as *mut osip_transaction_t;
    }
    osip_mutex_unlock((*osip).ict_fastmutex as *mut osip_mutex);
}
pub unsafe extern "C" fn osip_timers_ist_execute(mut osip: *mut osip_t) {
    let mut tr: *mut osip_transaction_t = 0 as *mut osip_transaction_t;
    let mut iterator: osip_list_iterator_t = osip_list_iterator_t {
        actual: 0 as *mut __node_t,
        prev: 0 as *mut *mut __node_t,
        li: 0 as *mut osip_list_t,
        pos: 0,
    };
    osip_mutex_lock((*osip).ist_fastmutex as *mut osip_mutex);
    tr = osip_list_get_first(&mut (*osip).osip_ist_transactions, &mut iterator)
        as *mut osip_transaction_t;
    while !(iterator.actual).is_null() && iterator.pos < (*iterator.li).nb_elt {
        let mut evt: *mut osip_event_t = 0 as *mut osip_event_t;
        evt = __osip_ist_need_timer_i_event(
            (*tr).ist_context,
            (*tr).state,
            (*tr).transactionid,
        );
        if !evt.is_null() {
            osip_fifo_add((*tr).transactionff, evt as *mut libc::c_void);
        } else {
            evt = __osip_ist_need_timer_h_event(
                (*tr).ist_context,
                (*tr).state,
                (*tr).transactionid,
            );
            if !evt.is_null() {
                osip_fifo_add((*tr).transactionff, evt as *mut libc::c_void);
            } else {
                evt = __osip_ist_need_timer_g_event(
                    (*tr).ist_context,
                    (*tr).state,
                    (*tr).transactionid,
                );
                if !evt.is_null() {
                    osip_fifo_add((*tr).transactionff, evt as *mut libc::c_void);
                }
            }
        }
        tr = osip_list_get_next(&mut iterator) as *mut osip_transaction_t;
    }
    osip_mutex_unlock((*osip).ist_fastmutex as *mut osip_mutex);
}
pub unsafe extern "C" fn osip_timers_nict_execute(mut osip: *mut osip_t) {
    let mut tr: *mut osip_transaction_t = 0 as *mut osip_transaction_t;
    let mut iterator: osip_list_iterator_t = osip_list_iterator_t {
        actual: 0 as *mut __node_t,
        prev: 0 as *mut *mut __node_t,
        li: 0 as *mut osip_list_t,
        pos: 0,
    };
    osip_mutex_lock((*osip).nict_fastmutex as *mut osip_mutex);
    tr = osip_list_get_first(&mut (*osip).osip_nict_transactions, &mut iterator)
        as *mut osip_transaction_t;
    while !(iterator.actual).is_null() && iterator.pos < (*iterator.li).nb_elt {
        let mut evt: *mut osip_event_t = 0 as *mut osip_event_t;
        evt = __osip_nict_need_timer_k_event(
            (*tr).nict_context,
            (*tr).state,
            (*tr).transactionid,
        );
        if !evt.is_null() {
            osip_fifo_add((*tr).transactionff, evt as *mut libc::c_void);
        } else {
            evt = __osip_nict_need_timer_f_event(
                (*tr).nict_context,
                (*tr).state,
                (*tr).transactionid,
            );
            if !evt.is_null() {
                osip_fifo_add((*tr).transactionff, evt as *mut libc::c_void);
            } else {
                evt = __osip_nict_need_timer_e_event(
                    (*tr).nict_context,
                    (*tr).state,
                    (*tr).transactionid,
                );
                if !evt.is_null() {
                    osip_fifo_add((*tr).transactionff, evt as *mut libc::c_void);
                }
            }
        }
        tr = osip_list_get_next(&mut iterator) as *mut osip_transaction_t;
    }
    osip_mutex_unlock((*osip).nict_fastmutex as *mut osip_mutex);
}
pub unsafe extern "C" fn osip_timers_nist_execute(mut osip: *mut osip_t) {
    let mut tr: *mut osip_transaction_t = 0 as *mut osip_transaction_t;
    let mut iterator: osip_list_iterator_t = osip_list_iterator_t {
        actual: 0 as *mut __node_t,
        prev: 0 as *mut *mut __node_t,
        li: 0 as *mut osip_list_t,
        pos: 0,
    };
    osip_mutex_lock((*osip).nist_fastmutex as *mut osip_mutex);
    tr = osip_list_get_first(&mut (*osip).osip_nist_transactions, &mut iterator)
        as *mut osip_transaction_t;
    while !(iterator.actual).is_null() && iterator.pos < (*iterator.li).nb_elt {
        let mut evt: *mut osip_event_t = 0 as *mut osip_event_t;
        evt = __osip_nist_need_timer_j_event(
            (*tr).nist_context,
            (*tr).state,
            (*tr).transactionid,
        );
        if !evt.is_null() {
            osip_fifo_add((*tr).transactionff, evt as *mut libc::c_void);
        }
        tr = osip_list_get_next(&mut iterator) as *mut osip_transaction_t;
    }
    osip_mutex_unlock((*osip).nist_fastmutex as *mut osip_mutex);
}
pub unsafe extern "C" fn osip_set_cb_send_message(
    mut cf: *mut osip_t,
    mut cb: Option::<
        unsafe extern "C" fn(
            *mut osip_transaction_t,
            *mut osip_message_t,
            *mut libc::c_char,
            libc::c_int,
            libc::c_int,
        ) -> libc::c_int,
    >,
) {
    (*cf).cb_send_message = cb;
}
pub unsafe extern "C" fn __osip_message_callback(
    mut type_0: libc::c_int,
    mut tr: *mut osip_transaction_t,
    mut msg: *mut osip_message_t,
) {
    let mut config: *mut osip_t = (*tr).config as *mut osip_t;
    if type_0 >= OSIP_MESSAGE_CALLBACK_COUNT as libc::c_int {
        osip_trace(
            b"osip.c\0" as *const u8 as *const libc::c_char,
            1647 as libc::c_int,
            TRACE_LEVEL1,
            0 as *mut FILE,
            b"invalid callback type %d\n\0" as *const u8 as *const libc::c_char,
            type_0,
        );
        return;
    }
    if ((*config).msg_callbacks[type_0 as usize]).is_none() {
        return;
    }
    ((*config).msg_callbacks[type_0 as usize]).unwrap()(type_0, tr, msg);
}
pub unsafe extern "C" fn __osip_kill_transaction_callback(
    mut type_0: libc::c_int,
    mut tr: *mut osip_transaction_t,
) {
    let mut config: *mut osip_t = (*tr).config as *mut osip_t;
    if type_0 >= OSIP_KILL_CALLBACK_COUNT as libc::c_int {
        osip_trace(
            b"osip.c\0" as *const u8 as *const libc::c_char,
            1661 as libc::c_int,
            TRACE_LEVEL1,
            0 as *mut FILE,
            b"invalid callback type %d\n\0" as *const u8 as *const libc::c_char,
            type_0,
        );
        return;
    }
    (*tr).completed_time = osip_getsystemtime(0 as *mut time_t);
    osip_gettimeofday(&mut (*tr).destroyed_time, 0 as *mut libc::c_void);
    if ((*config).kill_callbacks[type_0 as usize]).is_none() {
        return;
    }
    ((*config).kill_callbacks[type_0 as usize]).unwrap()(type_0, tr);
}
pub unsafe extern "C" fn __osip_transport_error_callback(
    mut type_0: libc::c_int,
    mut tr: *mut osip_transaction_t,
    mut error: libc::c_int,
) {
    let mut config: *mut osip_t = (*tr).config as *mut osip_t;
    if type_0 >= OSIP_TRANSPORT_ERROR_CALLBACK_COUNT as libc::c_int {
        osip_trace(
            b"osip.c\0" as *const u8 as *const libc::c_char,
            1678 as libc::c_int,
            TRACE_LEVEL1,
            0 as *mut FILE,
            b"invalid callback type %d\n\0" as *const u8 as *const libc::c_char,
            type_0,
        );
        return;
    }
    if ((*config).tp_error_callbacks[type_0 as usize]).is_none() {
        return;
    }
    ((*config).tp_error_callbacks[type_0 as usize]).unwrap()(type_0, tr, error);
}
pub unsafe extern "C" fn osip_set_message_callback(
    mut config: *mut osip_t,
    mut type_0: libc::c_int,
    mut cb: osip_message_cb_t,
) -> libc::c_int {
    if type_0 >= OSIP_MESSAGE_CALLBACK_COUNT as libc::c_int {
        osip_trace(
            b"osip.c\0" as *const u8 as *const libc::c_char,
            1690 as libc::c_int,
            TRACE_LEVEL2,
            0 as *mut FILE,
            b"invalid callback type %d\n\0" as *const u8 as *const libc::c_char,
            type_0,
        );
        return -(2 as libc::c_int);
    }
    (*config).msg_callbacks[type_0 as usize] = cb;
    return 0 as libc::c_int;
}
pub unsafe extern "C" fn osip_set_kill_transaction_callback(
    mut config: *mut osip_t,
    mut type_0: libc::c_int,
    mut cb: osip_kill_transaction_cb_t,
) -> libc::c_int {
    if type_0 >= OSIP_KILL_CALLBACK_COUNT as libc::c_int {
        osip_trace(
            b"osip.c\0" as *const u8 as *const libc::c_char,
            1701 as libc::c_int,
            TRACE_LEVEL2,
            0 as *mut FILE,
            b"invalid callback type %d\n\0" as *const u8 as *const libc::c_char,
            type_0,
        );
        return -(2 as libc::c_int);
    }
    (*config).kill_callbacks[type_0 as usize] = cb;
    return 0 as libc::c_int;
}
pub unsafe extern "C" fn osip_set_transport_error_callback(
    mut config: *mut osip_t,
    mut type_0: libc::c_int,
    mut cb: osip_transport_error_cb_t,
) -> libc::c_int {
    if type_0 >= OSIP_TRANSPORT_ERROR_CALLBACK_COUNT as libc::c_int {
        osip_trace(
            b"osip.c\0" as *const u8 as *const libc::c_char,
            1711 as libc::c_int,
            TRACE_LEVEL2,
            0 as *mut FILE,
            b"invalid callback type %d\n\0" as *const u8 as *const libc::c_char,
            type_0,
        );
        return -(2 as libc::c_int);
    }
    (*config).tp_error_callbacks[type_0 as usize] = cb;
    return 0 as libc::c_int;
}
