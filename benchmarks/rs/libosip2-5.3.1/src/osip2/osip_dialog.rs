use ::libc;
extern "C" {
    pub type _IO_wide_data;
    pub type _IO_codecvt;
    pub type _IO_marker;
    fn memset(
        _: *mut libc::c_void,
        _: libc::c_int,
        _: libc::c_ulong,
    ) -> *mut libc::c_void;
    fn strcmp(_: *const libc::c_char, _: *const libc::c_char) -> libc::c_int;
    fn malloc(_: libc::c_ulong) -> *mut libc::c_void;
    fn free(__ptr: *mut libc::c_void);
    fn osip_list_add(
        li: *mut osip_list_t,
        element: *mut libc::c_void,
        pos: libc::c_int,
    ) -> libc::c_int;
    fn osip_list_get(li: *const osip_list_t, pos: libc::c_int) -> *mut libc::c_void;
    static mut osip_malloc_func: Option::<osip_malloc_func_t>;
    static mut osip_free_func: Option::<osip_free_func_t>;
    fn osip_atoi(number: *const libc::c_char) -> libc::c_int;
    fn osip_strdup(ch: *const libc::c_char) -> *mut libc::c_char;
    fn osip_uri_param_get_byname(
        url_params: *mut osip_list_t,
        name: *mut libc::c_char,
        dest: *mut *mut osip_uri_param_t,
    ) -> libc::c_int;
    fn osip_list_size(li: *const osip_list_t) -> libc::c_int;
    fn osip_list_special_free(
        li: *mut osip_list_t,
        free_func: Option::<unsafe extern "C" fn(*mut libc::c_void) -> ()>,
    );
    fn osip_list_init(li: *mut osip_list_t) -> libc::c_int;
    fn osip_call_id_to_str(
        header: *const osip_call_id_t,
        dest: *mut *mut libc::c_char,
    ) -> libc::c_int;
    fn osip_from_free(header: *mut osip_from_t);
    fn osip_from_clone(
        header: *const osip_from_t,
        dest: *mut *mut osip_from_t,
    ) -> libc::c_int;
    fn osip_from_compare(
        header1: *mut osip_from_t,
        header2: *mut osip_from_t,
    ) -> libc::c_int;
    fn osip_contact_free(header: *mut osip_contact_t);
    fn osip_contact_clone(
        header: *const osip_contact_t,
        dest: *mut *mut osip_contact_t,
    ) -> libc::c_int;
    fn osip_record_route_free(header: *mut osip_record_route_t);
    fn osip_to_free(header: *mut osip_to_t);
    fn osip_to_clone(header: *const osip_to_t, dest: *mut *mut osip_to_t) -> libc::c_int;
    fn osip_list_eol(li: *const osip_list_t, pos: libc::c_int) -> libc::c_int;
    fn osip_trace(
        fi: *const libc::c_char,
        li: libc::c_int,
        level: osip_trace_level_t,
        f: *mut FILE,
        chfr: *const libc::c_char,
        _: ...
    ) -> libc::c_int;
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
pub type osip_record_route_t = osip_from_t;
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
pub type osip_dialog_t = osip_dialog;
pub unsafe extern "C" fn osip_dialog_set_state(
    mut dialog: *mut osip_dialog_t,
    mut state: state_t,
) {
    if dialog.is_null() {
        return;
    }
    (*dialog).state = state;
}
pub unsafe extern "C" fn osip_dialog_update_route_set_as_uas(
    mut dialog: *mut osip_dialog_t,
    mut invite: *mut osip_message_t,
) -> libc::c_int {
    let mut contact: *mut osip_contact_t = 0 as *mut osip_contact_t;
    let mut i: libc::c_int = 0;
    if dialog.is_null() {
        return -(2 as libc::c_int);
    }
    if invite.is_null() {
        return -(2 as libc::c_int);
    }
    if osip_list_eol(&mut (*invite).contacts, 0 as libc::c_int) != 0 {
        osip_trace(
            b"osip_dialog.c\0" as *const u8 as *const libc::c_char,
            44 as libc::c_int,
            TRACE_LEVEL3,
            0 as *mut FILE,
            b"missing a contact in invite!\n\0" as *const u8 as *const libc::c_char,
        );
    } else {
        if !((*dialog).remote_contact_uri).is_null() {
            osip_contact_free((*dialog).remote_contact_uri);
        }
        (*dialog).remote_contact_uri = 0 as *mut osip_contact_t;
        contact = osip_list_get(&mut (*invite).contacts, 0 as libc::c_int)
            as *mut osip_contact_t;
        i = osip_contact_clone(contact, &mut (*dialog).remote_contact_uri);
        if i != 0 as libc::c_int {
            return i;
        }
    }
    return 0 as libc::c_int;
}
pub unsafe extern "C" fn osip_dialog_update_osip_cseq_as_uas(
    mut dialog: *mut osip_dialog_t,
    mut invite: *mut osip_message_t,
) -> libc::c_int {
    if dialog.is_null() {
        return -(2 as libc::c_int);
    }
    if invite.is_null() || ((*invite).cseq).is_null()
        || ((*(*invite).cseq).number).is_null()
    {
        return -(2 as libc::c_int);
    }
    (*dialog).remote_cseq = osip_atoi((*(*invite).cseq).number);
    return 0 as libc::c_int;
}
pub unsafe extern "C" fn osip_dialog_update_route_set_as_uac(
    mut dialog: *mut osip_dialog_t,
    mut response: *mut osip_message_t,
) -> libc::c_int {
    let mut contact: *mut osip_contact_t = 0 as *mut osip_contact_t;
    let mut i: libc::c_int = 0;
    if dialog.is_null() {
        return -(2 as libc::c_int);
    }
    if response.is_null() {
        return -(2 as libc::c_int);
    }
    if osip_list_eol(&mut (*response).contacts, 0 as libc::c_int) != 0 {
        osip_trace(
            b"osip_dialog.c\0" as *const u8 as *const libc::c_char,
            85 as libc::c_int,
            TRACE_LEVEL3,
            0 as *mut FILE,
            b"missing a contact in response!\n\0" as *const u8 as *const libc::c_char,
        );
    } else {
        if !((*dialog).remote_contact_uri).is_null() {
            osip_contact_free((*dialog).remote_contact_uri);
        }
        (*dialog).remote_contact_uri = 0 as *mut osip_contact_t;
        contact = osip_list_get(&mut (*response).contacts, 0 as libc::c_int)
            as *mut osip_contact_t;
        i = osip_contact_clone(contact, &mut (*dialog).remote_contact_uri);
        if i != 0 as libc::c_int {
            return i;
        }
    }
    if (*dialog).state as libc::c_uint == DIALOG_EARLY as libc::c_int as libc::c_uint
        && osip_list_size(&mut (*dialog).route_set) > 0 as libc::c_int
    {
        osip_list_special_free(
            &mut (*dialog).route_set,
            ::std::mem::transmute::<
                Option::<unsafe extern "C" fn(*mut osip_record_route_t) -> ()>,
                Option::<unsafe extern "C" fn(*mut libc::c_void) -> ()>,
            >(
                Some(
                    osip_record_route_free
                        as unsafe extern "C" fn(*mut osip_record_route_t) -> (),
                ),
            ),
        );
        osip_list_init(&mut (*dialog).route_set);
    }
    if (*dialog).state as libc::c_uint == DIALOG_EARLY as libc::c_int as libc::c_uint
        && osip_list_size(&mut (*dialog).route_set) == 0 as libc::c_int
    {
        let mut pos: libc::c_int = 0 as libc::c_int;
        while osip_list_eol(&mut (*response).record_routes, pos) == 0 {
            let mut rr: *mut osip_record_route_t = 0 as *mut osip_record_route_t;
            let mut rr2: *mut osip_record_route_t = 0 as *mut osip_record_route_t;
            rr = osip_list_get(&mut (*response).record_routes, pos)
                as *mut osip_record_route_t;
            i = osip_from_clone(rr, &mut rr2);
            if i != 0 as libc::c_int {
                return i;
            }
            osip_list_add(
                &mut (*dialog).route_set,
                rr2 as *mut libc::c_void,
                0 as libc::c_int,
            );
            pos += 1;
            pos;
        }
    }
    if (*response).status_code >= 200 as libc::c_int
        && (*response).status_code < 300 as libc::c_int
    {
        (*dialog).state = DIALOG_CONFIRMED;
    }
    return 0 as libc::c_int;
}
pub unsafe extern "C" fn osip_dialog_update_tag_as_uac(
    mut dialog: *mut osip_dialog_t,
    mut response: *mut osip_message_t,
) -> libc::c_int {
    let mut tag: *mut osip_generic_param_t = 0 as *mut osip_generic_param_t;
    let mut i: libc::c_int = 0;
    if dialog.is_null() {
        return -(2 as libc::c_int);
    }
    if response.is_null() || ((*response).to).is_null() {
        return -(2 as libc::c_int);
    }
    if !((*dialog).remote_tag).is_null() {
        osip_trace(
            b"osip_dialog.c\0" as *const u8 as *const libc::c_char,
            142 as libc::c_int,
            TRACE_LEVEL3,
            0 as *mut FILE,
            b"This dialog already have a remote tag: it can't be changed!\n\0"
                as *const u8 as *const libc::c_char,
        );
        return -(3 as libc::c_int);
    }
    i = osip_uri_param_get_byname(
        &mut (*(*response).to).gen_params,
        b"tag\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        &mut tag,
    );
    if i != 0 as libc::c_int || tag.is_null() || ((*tag).gvalue).is_null() {
        osip_trace(
            b"osip_dialog.c\0" as *const u8 as *const libc::c_char,
            149 as libc::c_int,
            TRACE_LEVEL3,
            0 as *mut FILE,
            b"Remote UA is not compliant: missing a tag in response!\n\0" as *const u8
                as *const libc::c_char,
        );
        (*dialog).remote_tag = 0 as *mut libc::c_char;
    } else {
        (*dialog).remote_tag = osip_strdup((*tag).gvalue);
    }
    return 0 as libc::c_int;
}
pub unsafe extern "C" fn osip_dialog_match_as_uac(
    mut dlg: *mut osip_dialog_t,
    mut answer: *mut osip_message_t,
) -> libc::c_int {
    let mut tag_param_local: *mut osip_generic_param_t = 0 as *mut osip_generic_param_t;
    let mut tag_param_remote: *mut osip_generic_param_t = 0 as *mut osip_generic_param_t;
    let mut tmp: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut i: libc::c_int = 0;
    if dlg.is_null() || ((*dlg).call_id).is_null() {
        return -(2 as libc::c_int);
    }
    if answer.is_null() || ((*answer).call_id).is_null() || ((*answer).from).is_null()
        || ((*answer).to).is_null()
    {
        return -(2 as libc::c_int);
    }
    osip_trace(
        b"osip_dialog.c\0" as *const u8 as *const libc::c_char,
        170 as libc::c_int,
        TRACE_LEVEL3,
        0 as *mut FILE,
        b"Using this method is discouraged. See source code explanations!\n\0"
            as *const u8 as *const libc::c_char,
    );
    i = osip_call_id_to_str((*answer).call_id, &mut tmp);
    if i != 0 as libc::c_int {
        return i;
    }
    if 0 as libc::c_int != strcmp((*dlg).call_id, tmp) {
        if !tmp.is_null() {
            if osip_free_func.is_some() {
                osip_free_func.unwrap()(tmp as *mut libc::c_void);
            } else {
                free(tmp as *mut libc::c_void);
            }
        }
        return -(1 as libc::c_int);
    }
    if !tmp.is_null() {
        if osip_free_func.is_some() {
            osip_free_func.unwrap()(tmp as *mut libc::c_void);
        } else {
            free(tmp as *mut libc::c_void);
        }
    }
    i = osip_uri_param_get_byname(
        &mut (*(*answer).from).gen_params,
        b"tag\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        &mut tag_param_local,
    );
    if i != 0 as libc::c_int {
        return -(5 as libc::c_int);
    }
    if ((*dlg).local_tag).is_null() {
        return -(5 as libc::c_int);
    }
    if 0 as libc::c_int != strcmp((*tag_param_local).gvalue, (*dlg).local_tag) {
        return -(1 as libc::c_int);
    }
    i = osip_uri_param_get_byname(
        &mut (*(*answer).to).gen_params,
        b"tag\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        &mut tag_param_remote,
    );
    if i != 0 as libc::c_int && !((*dlg).remote_tag).is_null() {
        return -(5 as libc::c_int);
    }
    if i != 0 as libc::c_int && ((*dlg).remote_tag).is_null() {
        if 0 as libc::c_int == osip_from_compare((*dlg).local_uri, (*answer).from)
            && 0 as libc::c_int == osip_from_compare((*dlg).remote_uri, (*answer).to)
        {
            return 0 as libc::c_int;
        }
        return -(1 as libc::c_int);
    }
    if ((*dlg).remote_tag).is_null() {
        osip_trace(
            b"osip_dialog.c\0" as *const u8 as *const libc::c_char,
            224 as libc::c_int,
            TRACE_LEVEL3,
            0 as *mut FILE,
            b"Remote UA is not compliant: missing a tag in To fields!\n\0" as *const u8
                as *const libc::c_char,
        );
        if 0 as libc::c_int == osip_from_compare((*dlg).local_uri, (*answer).from)
            && 0 as libc::c_int == osip_from_compare((*dlg).remote_uri, (*answer).to)
        {
            return 0 as libc::c_int;
        }
        return -(1 as libc::c_int);
    }
    if 0 as libc::c_int == strcmp((*tag_param_remote).gvalue, (*dlg).remote_tag) {
        return 0 as libc::c_int;
    }
    return -(1 as libc::c_int);
}
pub unsafe extern "C" fn osip_dialog_match_as_uas(
    mut dlg: *mut osip_dialog_t,
    mut request: *mut osip_message_t,
) -> libc::c_int {
    let mut tag_param_remote: *mut osip_generic_param_t = 0 as *mut osip_generic_param_t;
    let mut i: libc::c_int = 0;
    let mut tmp: *mut libc::c_char = 0 as *mut libc::c_char;
    if dlg.is_null() || ((*dlg).call_id).is_null() {
        return -(2 as libc::c_int);
    }
    if request.is_null() || ((*request).call_id).is_null() || ((*request).from).is_null()
        || ((*request).to).is_null()
    {
        return -(2 as libc::c_int);
    }
    i = osip_call_id_to_str((*request).call_id, &mut tmp);
    if i != 0 as libc::c_int {
        return i;
    }
    if 0 as libc::c_int != strcmp((*dlg).call_id, tmp) {
        if !tmp.is_null() {
            if osip_free_func.is_some() {
                osip_free_func.unwrap()(tmp as *mut libc::c_void);
            } else {
                free(tmp as *mut libc::c_void);
            }
        }
        return -(1 as libc::c_int);
    }
    if !tmp.is_null() {
        if osip_free_func.is_some() {
            osip_free_func.unwrap()(tmp as *mut libc::c_void);
        } else {
            free(tmp as *mut libc::c_void);
        }
    }
    if ((*dlg).local_tag).is_null() {
        return -(5 as libc::c_int);
    }
    i = osip_uri_param_get_byname(
        &mut (*(*request).from).gen_params,
        b"tag\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        &mut tag_param_remote,
    );
    if i != 0 as libc::c_int && !((*dlg).remote_tag).is_null() {
        return -(5 as libc::c_int);
    }
    if i != 0 as libc::c_int && ((*dlg).remote_tag).is_null() {
        if 0 as libc::c_int
            == osip_from_compare((*dlg).remote_uri as *mut osip_from_t, (*request).from)
            && 0 as libc::c_int == osip_from_compare((*dlg).local_uri, (*request).to)
        {
            return 0 as libc::c_int;
        }
        return -(1 as libc::c_int);
    }
    if ((*dlg).remote_tag).is_null() {
        osip_trace(
            b"osip_dialog.c\0" as *const u8 as *const libc::c_char,
            301 as libc::c_int,
            TRACE_LEVEL3,
            0 as *mut FILE,
            b"Remote UA is not compliant: missing a tag in To feilds!\n\0" as *const u8
                as *const libc::c_char,
        );
        if 0 as libc::c_int
            == osip_from_compare((*dlg).remote_uri as *mut osip_from_t, (*request).from)
            && 0 as libc::c_int == osip_from_compare((*dlg).local_uri, (*request).to)
        {
            return 0 as libc::c_int;
        }
        return -(1 as libc::c_int);
    }
    if 0 as libc::c_int == strcmp((*tag_param_remote).gvalue, (*dlg).remote_tag) {
        return 0 as libc::c_int;
    }
    return -(1 as libc::c_int);
}
unsafe extern "C" fn __osip_dialog_init(
    mut dialog: *mut *mut osip_dialog_t,
    mut invite: *mut osip_message_t,
    mut response: *mut osip_message_t,
    mut local: *mut osip_from_t,
    mut remote: *mut osip_to_t,
    mut remote_msg: *mut osip_message_t,
) -> libc::c_int {
    let mut i: libc::c_int = 0;
    let mut pos: libc::c_int = 0;
    let mut tag: *mut osip_generic_param_t = 0 as *mut osip_generic_param_t;
    *dialog = 0 as *mut osip_dialog_t;
    if response.is_null() {
        return -(2 as libc::c_int);
    }
    if ((*response).cseq).is_null() || local.is_null() || remote.is_null() {
        return -(5 as libc::c_int);
    }
    *dialog = (if osip_malloc_func.is_some() {
        osip_malloc_func
            .unwrap()(::std::mem::size_of::<osip_dialog_t>() as libc::c_ulong)
    } else {
        malloc(::std::mem::size_of::<osip_dialog_t>() as libc::c_ulong)
    }) as *mut osip_dialog_t;
    if (*dialog).is_null() {
        return -(4 as libc::c_int);
    }
    memset(
        *dialog as *mut libc::c_void,
        0 as libc::c_int,
        ::std::mem::size_of::<osip_dialog_t>() as libc::c_ulong,
    );
    (**dialog).your_instance = 0 as *mut libc::c_void;
    if (*response).status_code >= 200 as libc::c_int
        && (*response).status_code < 300 as libc::c_int
    {
        (**dialog).state = DIALOG_CONFIRMED;
    } else {
        (**dialog).state = DIALOG_EARLY;
    }
    i = osip_call_id_to_str((*response).call_id, &mut (**dialog).call_id);
    if i != 0 as libc::c_int {
        osip_trace(
            b"osip_dialog.c\0" as *const u8 as *const libc::c_char,
            349 as libc::c_int,
            TRACE_LEVEL2,
            0 as *mut FILE,
            b"Could not establish dialog!\n\0" as *const u8 as *const libc::c_char,
        );
        osip_dialog_free(*dialog);
        *dialog = 0 as *mut osip_dialog_t;
        return i;
    }
    i = osip_uri_param_get_byname(
        &mut (*local).gen_params,
        b"tag\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        &mut tag,
    );
    if i != 0 as libc::c_int {
        osip_trace(
            b"osip_dialog.c\0" as *const u8 as *const libc::c_char,
            358 as libc::c_int,
            TRACE_LEVEL2,
            0 as *mut FILE,
            b"Could not establish dialog!\n\0" as *const u8 as *const libc::c_char,
        );
        osip_dialog_free(*dialog);
        *dialog = 0 as *mut osip_dialog_t;
        return i;
    }
    (**dialog).local_tag = osip_strdup((*tag).gvalue);
    i = osip_uri_param_get_byname(
        &mut (*remote).gen_params,
        b"tag\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        &mut tag,
    );
    if i == 0 as libc::c_int {
        (**dialog).remote_tag = osip_strdup((*tag).gvalue);
    }
    if !invite.is_null() {
        let mut line_param: *mut osip_uri_param_t = 0 as *mut osip_uri_param_t;
        i = osip_uri_param_get_byname(
            &mut (*(*invite).req_uri).url_params,
            b"line\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            &mut line_param,
        );
        if i == 0 as libc::c_int && !line_param.is_null()
            && !((*line_param).gvalue).is_null()
        {
            (**dialog).line_param = osip_strdup((*line_param).gvalue);
        }
    }
    osip_list_init(&mut (**dialog).route_set);
    pos = 0 as libc::c_int;
    while osip_list_eol(&mut (*response).record_routes, pos) == 0 {
        let mut rr: *mut osip_record_route_t = 0 as *mut osip_record_route_t;
        let mut rr2: *mut osip_record_route_t = 0 as *mut osip_record_route_t;
        rr = osip_list_get(&mut (*response).record_routes, pos)
            as *mut osip_record_route_t;
        i = osip_from_clone(rr, &mut rr2);
        if i != 0 as libc::c_int {
            osip_trace(
                b"osip_dialog.c\0" as *const u8 as *const libc::c_char,
                393 as libc::c_int,
                TRACE_LEVEL2,
                0 as *mut FILE,
                b"Could not establish dialog!\n\0" as *const u8 as *const libc::c_char,
            );
            osip_dialog_free(*dialog);
            *dialog = 0 as *mut osip_dialog_t;
            return i;
        }
        if invite.is_null() {
            osip_list_add(
                &mut (**dialog).route_set,
                rr2 as *mut libc::c_void,
                0 as libc::c_int,
            );
        } else {
            osip_list_add(
                &mut (**dialog).route_set,
                rr2 as *mut libc::c_void,
                -(1 as libc::c_int),
            );
        }
        pos += 1;
        pos;
    }
    (**dialog).local_cseq = osip_atoi((*(*response).cseq).number);
    i = osip_from_clone(remote, &mut (**dialog).remote_uri);
    if i != 0 as libc::c_int {
        osip_trace(
            b"osip_dialog.c\0" as *const u8 as *const libc::c_char,
            415 as libc::c_int,
            TRACE_LEVEL2,
            0 as *mut FILE,
            b"Could not establish dialog!\n\0" as *const u8 as *const libc::c_char,
        );
        osip_dialog_free(*dialog);
        *dialog = 0 as *mut osip_dialog_t;
        return i;
    }
    i = osip_to_clone(local, &mut (**dialog).local_uri);
    if i != 0 as libc::c_int {
        osip_trace(
            b"osip_dialog.c\0" as *const u8 as *const libc::c_char,
            424 as libc::c_int,
            TRACE_LEVEL2,
            0 as *mut FILE,
            b"Could not establish dialog!\n\0" as *const u8 as *const libc::c_char,
        );
        osip_dialog_free(*dialog);
        *dialog = 0 as *mut osip_dialog_t;
        return i;
    }
    let mut contact: *mut osip_contact_t = 0 as *mut osip_contact_t;
    if osip_list_eol(&mut (*remote_msg).contacts, 0 as libc::c_int) == 0 {
        contact = osip_list_get(&mut (*remote_msg).contacts, 0 as libc::c_int)
            as *mut osip_contact_t;
        i = osip_contact_clone(contact, &mut (**dialog).remote_contact_uri);
        if i != 0 as libc::c_int {
            osip_trace(
                b"osip_dialog.c\0" as *const u8 as *const libc::c_char,
                438 as libc::c_int,
                TRACE_LEVEL2,
                0 as *mut FILE,
                b"Could not establish dialog!\n\0" as *const u8 as *const libc::c_char,
            );
            osip_dialog_free(*dialog);
            *dialog = 0 as *mut osip_dialog_t;
            return i;
        }
    } else {
        (**dialog).remote_contact_uri = 0 as *mut osip_contact_t;
        osip_trace(
            b"osip_dialog.c\0" as *const u8 as *const libc::c_char,
            446 as libc::c_int,
            TRACE_LEVEL3,
            0 as *mut FILE,
            b"Remote UA is not compliant: missing a contact in remote message!\n\0"
                as *const u8 as *const libc::c_char,
        );
    }
    (**dialog).secure = -(1 as libc::c_int);
    return 0 as libc::c_int;
}
pub unsafe extern "C" fn osip_dialog_init_as_uac(
    mut dialog: *mut *mut osip_dialog_t,
    mut response: *mut osip_message_t,
) -> libc::c_int {
    let mut i: libc::c_int = 0;
    i = __osip_dialog_init(
        dialog,
        0 as *mut osip_message_t,
        response,
        (*response).from,
        (*response).to,
        response,
    );
    if i != 0 as libc::c_int {
        *dialog = 0 as *mut osip_dialog_t;
        return i;
    }
    (**dialog).type_0 = CALLER;
    (**dialog).remote_cseq = -(1 as libc::c_int);
    return 0 as libc::c_int;
}
pub unsafe extern "C" fn osip_dialog_init_as_uac_with_remote_request(
    mut dialog: *mut *mut osip_dialog_t,
    mut next_request: *mut osip_message_t,
    mut local_cseq: libc::c_int,
) -> libc::c_int {
    let mut i: libc::c_int = 0;
    *dialog = 0 as *mut osip_dialog_t;
    if ((*next_request).cseq).is_null() {
        return -(2 as libc::c_int);
    }
    i = __osip_dialog_init(
        dialog,
        next_request,
        next_request,
        (*next_request).to,
        (*next_request).from,
        next_request,
    );
    if i != 0 as libc::c_int {
        *dialog = 0 as *mut osip_dialog_t;
        return i;
    }
    (**dialog).type_0 = CALLER;
    (**dialog).state = DIALOG_CONFIRMED;
    (**dialog).local_cseq = local_cseq;
    (**dialog).remote_cseq = osip_atoi((*(*next_request).cseq).number);
    return 0 as libc::c_int;
}
pub unsafe extern "C" fn osip_dialog_init_as_uas(
    mut dialog: *mut *mut osip_dialog_t,
    mut invite: *mut osip_message_t,
    mut response: *mut osip_message_t,
) -> libc::c_int {
    let mut i: libc::c_int = 0;
    *dialog = 0 as *mut osip_dialog_t;
    if ((*response).cseq).is_null() {
        return -(5 as libc::c_int);
    }
    i = __osip_dialog_init(
        dialog,
        invite,
        response,
        (*response).to,
        (*response).from,
        invite,
    );
    if i != 0 as libc::c_int {
        *dialog = 0 as *mut osip_dialog_t;
        return i;
    }
    (**dialog).type_0 = CALLEE;
    (**dialog).remote_cseq = osip_atoi((*(*response).cseq).number);
    return 0 as libc::c_int;
}
pub unsafe extern "C" fn osip_dialog_free(mut dialog: *mut osip_dialog_t) {
    if dialog.is_null() {
        return;
    }
    osip_contact_free((*dialog).remote_contact_uri);
    osip_from_free((*dialog).local_uri);
    osip_to_free((*dialog).remote_uri);
    osip_list_special_free(
        &mut (*dialog).route_set,
        ::std::mem::transmute::<
            Option::<unsafe extern "C" fn(*mut osip_record_route_t) -> ()>,
            Option::<unsafe extern "C" fn(*mut libc::c_void) -> ()>,
        >(
            Some(
                osip_record_route_free
                    as unsafe extern "C" fn(*mut osip_record_route_t) -> (),
            ),
        ),
    );
    if !((*dialog).line_param).is_null() {
        if osip_free_func.is_some() {
            osip_free_func.unwrap()((*dialog).line_param as *mut libc::c_void);
        } else {
            free((*dialog).line_param as *mut libc::c_void);
        }
    }
    if !((*dialog).remote_tag).is_null() {
        if osip_free_func.is_some() {
            osip_free_func.unwrap()((*dialog).remote_tag as *mut libc::c_void);
        } else {
            free((*dialog).remote_tag as *mut libc::c_void);
        }
    }
    if !((*dialog).local_tag).is_null() {
        if osip_free_func.is_some() {
            osip_free_func.unwrap()((*dialog).local_tag as *mut libc::c_void);
        } else {
            free((*dialog).local_tag as *mut libc::c_void);
        }
    }
    if !((*dialog).call_id).is_null() {
        if osip_free_func.is_some() {
            osip_free_func.unwrap()((*dialog).call_id as *mut libc::c_void);
        } else {
            free((*dialog).call_id as *mut libc::c_void);
        }
    }
    if !dialog.is_null() {
        if osip_free_func.is_some() {
            osip_free_func.unwrap()(dialog as *mut libc::c_void);
        } else {
            free(dialog as *mut libc::c_void);
        }
    }
}
