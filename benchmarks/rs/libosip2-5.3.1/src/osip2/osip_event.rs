use ::libc;
extern "C" {
    pub type _IO_wide_data;
    pub type _IO_codecvt;
    pub type _IO_marker;
    fn malloc(_: libc::c_ulong) -> *mut libc::c_void;
    fn free(__ptr: *mut libc::c_void);
    fn strcmp(_: *const libc::c_char, _: *const libc::c_char) -> libc::c_int;
    static mut osip_free_func: Option::<osip_free_func_t>;
    fn osip_trace(
        fi: *const libc::c_char,
        li: libc::c_int,
        level: osip_trace_level_t,
        f: *mut FILE,
        chfr: *const libc::c_char,
        _: ...
    ) -> libc::c_int;
    fn osip_message_init(sip: *mut *mut osip_message_t) -> libc::c_int;
    fn osip_message_free(sip: *mut osip_message_t);
    fn osip_message_parse(
        sip: *mut osip_message_t,
        buf: *const libc::c_char,
        length: size_t,
    ) -> libc::c_int;
    static mut osip_malloc_func: Option::<osip_malloc_func_t>;
}
pub type size_t = libc::c_ulong;
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct osip_event {
    pub type_0: type_t,
    pub transactionid: libc::c_int,
    pub sip: *mut osip_message_t,
}
pub type osip_event_t = osip_event;
pub unsafe extern "C" fn osip_parse(
    mut buf: *const libc::c_char,
    mut length: size_t,
) -> *mut osip_event_t {
    let mut i: libc::c_int = 0;
    let mut se: *mut osip_event_t = __osip_event_new(UNKNOWN_EVT, 0 as libc::c_int);
    if se.is_null() {
        return 0 as *mut osip_event_t;
    }
    i = osip_message_init(&mut (*se).sip);
    if i != 0 as libc::c_int {
        if !se.is_null() {
            if osip_free_func.is_some() {
                osip_free_func.unwrap()(se as *mut libc::c_void);
            } else {
                free(se as *mut libc::c_void);
            }
        }
        return 0 as *mut osip_event_t;
    }
    if osip_message_parse((*se).sip, buf, length) != 0 as libc::c_int {
        osip_trace(
            b"osip_event.c\0" as *const u8 as *const libc::c_char,
            44 as libc::c_int,
            TRACE_LEVEL2,
            0 as *mut FILE,
            b"could not parse message\n\0" as *const u8 as *const libc::c_char,
        );
        osip_message_free((*se).sip);
        if !se.is_null() {
            if osip_free_func.is_some() {
                osip_free_func.unwrap()(se as *mut libc::c_void);
            } else {
                free(se as *mut libc::c_void);
            }
        }
        return 0 as *mut osip_event_t;
    } else {
        if !((*(*se).sip).call_id).is_null()
            && !((*(*(*se).sip).call_id).number).is_null()
        {
            osip_trace(
                b"osip_event.c\0" as *const u8 as *const libc::c_char,
                51 as libc::c_int,
                TRACE_LEVEL6,
                0 as *mut FILE,
                b"MESSAGE REC. CALLID:%s\n\0" as *const u8 as *const libc::c_char,
                (*(*(*se).sip).call_id).number,
            );
        }
        if (*(*se).sip).status_code == 0 as libc::c_int {
            if ((*(*se).sip).sip_method).is_null() || ((*(*se).sip).req_uri).is_null() {
                osip_message_free((*se).sip);
                if !se.is_null() {
                    if osip_free_func.is_some() {
                        osip_free_func.unwrap()(se as *mut libc::c_void);
                    } else {
                        free(se as *mut libc::c_void);
                    }
                }
                return 0 as *mut osip_event_t;
            }
        }
        (*se).type_0 = evt_set_type_incoming_sipmessage((*se).sip);
        return se;
    };
}
pub unsafe extern "C" fn __osip_event_new(
    mut type_0: type_t,
    mut transactionid: libc::c_int,
) -> *mut osip_event_t {
    let mut sipevent: *mut osip_event_t = 0 as *mut osip_event_t;
    sipevent = (if osip_malloc_func.is_some() {
        osip_malloc_func.unwrap()(::std::mem::size_of::<osip_event_t>() as libc::c_ulong)
    } else {
        malloc(::std::mem::size_of::<osip_event_t>() as libc::c_ulong)
    }) as *mut osip_event_t;
    if sipevent.is_null() {
        return 0 as *mut osip_event_t;
    }
    (*sipevent).type_0 = type_0;
    (*sipevent).sip = 0 as *mut osip_message_t;
    (*sipevent).transactionid = transactionid;
    return sipevent;
}
pub unsafe extern "C" fn osip_new_outgoing_sipmessage(
    mut sip: *mut osip_message_t,
) -> *mut osip_event_t {
    let mut sipevent: *mut osip_event_t = 0 as *mut osip_event_t;
    if sip.is_null() {
        return 0 as *mut osip_event_t;
    }
    if (*sip).status_code == 0 as libc::c_int {
        if ((*sip).sip_method).is_null() {
            return 0 as *mut osip_event_t;
        }
        if ((*sip).req_uri).is_null() {
            return 0 as *mut osip_event_t;
        }
    }
    sipevent = (if osip_malloc_func.is_some() {
        osip_malloc_func.unwrap()(::std::mem::size_of::<osip_event_t>() as libc::c_ulong)
    } else {
        malloc(::std::mem::size_of::<osip_event_t>() as libc::c_ulong)
    }) as *mut osip_event_t;
    if sipevent.is_null() {
        return 0 as *mut osip_event_t;
    }
    (*sipevent).sip = sip;
    (*sipevent).type_0 = evt_set_type_outgoing_sipmessage(sip);
    (*sipevent).transactionid = 0 as libc::c_int;
    return sipevent;
}
pub unsafe extern "C" fn evt_set_type_incoming_sipmessage(
    mut sip: *mut osip_message_t,
) -> type_t {
    if (*sip).status_code == 0 as libc::c_int {
        if (*sip).status_code == 0 as libc::c_int
            && 0 as libc::c_int
                == strcmp(
                    (*sip).sip_method,
                    b"INVITE\0" as *const u8 as *const libc::c_char,
                )
        {
            return RCV_REQINVITE
        } else if (*sip).status_code == 0 as libc::c_int
            && 0 as libc::c_int
                == strcmp(
                    (*sip).sip_method,
                    b"ACK\0" as *const u8 as *const libc::c_char,
                )
        {
            return RCV_REQACK
        }
        return RCV_REQUEST;
    } else {
        if (*sip).status_code >= 100 as libc::c_int
            && (*sip).status_code < 200 as libc::c_int
        {
            return RCV_STATUS_1XX
        } else if (*sip).status_code >= 200 as libc::c_int
            && (*sip).status_code < 300 as libc::c_int
        {
            return RCV_STATUS_2XX
        }
        return RCV_STATUS_3456XX;
    };
}
pub unsafe extern "C" fn evt_set_type_outgoing_sipmessage(
    mut sip: *mut osip_message_t,
) -> type_t {
    if (*sip).status_code == 0 as libc::c_int {
        if (*sip).status_code == 0 as libc::c_int
            && 0 as libc::c_int
                == strcmp(
                    (*sip).sip_method,
                    b"INVITE\0" as *const u8 as *const libc::c_char,
                )
        {
            return SND_REQINVITE;
        }
        if (*sip).status_code == 0 as libc::c_int
            && 0 as libc::c_int
                == strcmp(
                    (*sip).sip_method,
                    b"ACK\0" as *const u8 as *const libc::c_char,
                )
        {
            return SND_REQACK;
        }
        return SND_REQUEST;
    } else {
        if (*sip).status_code >= 100 as libc::c_int
            && (*sip).status_code < 200 as libc::c_int
        {
            return SND_STATUS_1XX
        } else if (*sip).status_code >= 200 as libc::c_int
            && (*sip).status_code < 300 as libc::c_int
        {
            return SND_STATUS_2XX
        }
        return SND_STATUS_3456XX;
    };
}
pub unsafe extern "C" fn osip_event_free(mut event: *mut osip_event_t) {
    if !event.is_null() {
        osip_message_free((*event).sip);
        if !event.is_null() {
            if osip_free_func.is_some() {
                osip_free_func.unwrap()(event as *mut libc::c_void);
            } else {
                free(event as *mut libc::c_void);
            }
        }
    }
}
