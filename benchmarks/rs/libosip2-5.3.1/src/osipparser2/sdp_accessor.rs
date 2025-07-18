use ::libc;
extern "C" {
    fn strcmp(_: *const libc::c_char, _: *const libc::c_char) -> libc::c_int;
    fn free(__ptr: *mut libc::c_void);
    static mut osip_free_func: Option::<osip_free_func_t>;
    fn osip_list_size(li: *const osip_list_t) -> libc::c_int;
    fn osip_list_eol(li: *const osip_list_t, pos: libc::c_int) -> libc::c_int;
    fn osip_list_add(
        li: *mut osip_list_t,
        element: *mut libc::c_void,
        pos: libc::c_int,
    ) -> libc::c_int;
    fn osip_list_remove(li: *mut osip_list_t, pos: libc::c_int) -> libc::c_int;
    fn osip_list_get(li: *const osip_list_t, pos: libc::c_int) -> *mut libc::c_void;
    fn sdp_bandwidth_init(elem: *mut *mut sdp_bandwidth_t) -> libc::c_int;
    fn sdp_time_descr_init(elem: *mut *mut sdp_time_descr_t) -> libc::c_int;
    fn sdp_key_init(elem: *mut *mut sdp_key_t) -> libc::c_int;
    fn sdp_attribute_init(elem: *mut *mut sdp_attribute_t) -> libc::c_int;
    fn sdp_attribute_free(elem: *mut sdp_attribute_t);
    fn sdp_connection_init(elem: *mut *mut sdp_connection_t) -> libc::c_int;
    fn sdp_media_init(elem: *mut *mut sdp_media_t) -> libc::c_int;
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
pub type osip_free_func_t = unsafe extern "C" fn(*mut libc::c_void) -> ();
#[derive(Copy, Clone)]
#[repr(C)]
pub struct sdp_bandwidth {
    pub b_bwtype: *mut libc::c_char,
    pub b_bandwidth: *mut libc::c_char,
}
pub type sdp_bandwidth_t = sdp_bandwidth;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct sdp_time_descr {
    pub t_start_time: *mut libc::c_char,
    pub t_stop_time: *mut libc::c_char,
    pub r_repeats: osip_list_t,
}
pub type sdp_time_descr_t = sdp_time_descr;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct sdp_key {
    pub k_keytype: *mut libc::c_char,
    pub k_keydata: *mut libc::c_char,
}
pub type sdp_key_t = sdp_key;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct sdp_attribute {
    pub a_att_field: *mut libc::c_char,
    pub a_att_value: *mut libc::c_char,
}
pub type sdp_attribute_t = sdp_attribute;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct sdp_connection {
    pub c_nettype: *mut libc::c_char,
    pub c_addrtype: *mut libc::c_char,
    pub c_addr: *mut libc::c_char,
    pub c_addr_multicast_ttl: *mut libc::c_char,
    pub c_addr_multicast_int: *mut libc::c_char,
}
pub type sdp_connection_t = sdp_connection;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct sdp_media {
    pub m_media: *mut libc::c_char,
    pub m_port: *mut libc::c_char,
    pub m_number_of_port: *mut libc::c_char,
    pub m_proto: *mut libc::c_char,
    pub m_payloads: osip_list_t,
    pub i_info: *mut libc::c_char,
    pub c_connections: osip_list_t,
    pub b_bandwidths: osip_list_t,
    pub a_attributes: osip_list_t,
    pub k_key: *mut sdp_key_t,
}
pub type sdp_media_t = sdp_media;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct sdp_message {
    pub v_version: *mut libc::c_char,
    pub o_username: *mut libc::c_char,
    pub o_sess_id: *mut libc::c_char,
    pub o_sess_version: *mut libc::c_char,
    pub o_nettype: *mut libc::c_char,
    pub o_addrtype: *mut libc::c_char,
    pub o_addr: *mut libc::c_char,
    pub s_name: *mut libc::c_char,
    pub i_info: *mut libc::c_char,
    pub u_uri: *mut libc::c_char,
    pub e_emails: osip_list_t,
    pub p_phones: osip_list_t,
    pub c_connection: *mut sdp_connection_t,
    pub b_bandwidths: osip_list_t,
    pub t_descrs: osip_list_t,
    pub z_adjustments: *mut libc::c_char,
    pub k_key: *mut sdp_key_t,
    pub a_attributes: osip_list_t,
    pub m_medias: osip_list_t,
}
pub type sdp_message_t = sdp_message;
pub unsafe extern "C" fn sdp_message_v_version_set(
    mut sdp: *mut sdp_message_t,
    mut v_version: *mut libc::c_char,
) -> libc::c_int {
    if sdp.is_null() {
        return -(2 as libc::c_int);
    }
    (*sdp).v_version = v_version;
    return 0 as libc::c_int;
}
pub unsafe extern "C" fn sdp_message_v_version_get(
    mut sdp: *mut sdp_message_t,
) -> *mut libc::c_char {
    if sdp.is_null() {
        return 0 as *mut libc::c_char;
    }
    return (*sdp).v_version;
}
pub unsafe extern "C" fn sdp_message_o_origin_set(
    mut sdp: *mut sdp_message_t,
    mut username: *mut libc::c_char,
    mut sess_id: *mut libc::c_char,
    mut sess_version: *mut libc::c_char,
    mut nettype: *mut libc::c_char,
    mut addrtype: *mut libc::c_char,
    mut addr: *mut libc::c_char,
) -> libc::c_int {
    if sdp.is_null() {
        return -(2 as libc::c_int);
    }
    (*sdp).o_username = username;
    (*sdp).o_sess_id = sess_id;
    (*sdp).o_sess_version = sess_version;
    (*sdp).o_nettype = nettype;
    (*sdp).o_addrtype = addrtype;
    (*sdp).o_addr = addr;
    return 0 as libc::c_int;
}
pub unsafe extern "C" fn sdp_message_o_username_get(
    mut sdp: *mut sdp_message_t,
) -> *mut libc::c_char {
    if sdp.is_null() {
        return 0 as *mut libc::c_char;
    }
    return (*sdp).o_username;
}
pub unsafe extern "C" fn sdp_message_o_sess_id_get(
    mut sdp: *mut sdp_message_t,
) -> *mut libc::c_char {
    if sdp.is_null() {
        return 0 as *mut libc::c_char;
    }
    return (*sdp).o_sess_id;
}
pub unsafe extern "C" fn sdp_message_o_sess_version_get(
    mut sdp: *mut sdp_message_t,
) -> *mut libc::c_char {
    if sdp.is_null() {
        return 0 as *mut libc::c_char;
    }
    return (*sdp).o_sess_version;
}
pub unsafe extern "C" fn sdp_message_o_nettype_get(
    mut sdp: *mut sdp_message_t,
) -> *mut libc::c_char {
    if sdp.is_null() {
        return 0 as *mut libc::c_char;
    }
    return (*sdp).o_nettype;
}
pub unsafe extern "C" fn sdp_message_o_addrtype_get(
    mut sdp: *mut sdp_message_t,
) -> *mut libc::c_char {
    if sdp.is_null() {
        return 0 as *mut libc::c_char;
    }
    return (*sdp).o_addrtype;
}
pub unsafe extern "C" fn sdp_message_o_addr_get(
    mut sdp: *mut sdp_message_t,
) -> *mut libc::c_char {
    if sdp.is_null() {
        return 0 as *mut libc::c_char;
    }
    return (*sdp).o_addr;
}
pub unsafe extern "C" fn sdp_message_s_name_set(
    mut sdp: *mut sdp_message_t,
    mut name: *mut libc::c_char,
) -> libc::c_int {
    if sdp.is_null() {
        return -(1 as libc::c_int);
    }
    (*sdp).s_name = name;
    return 0 as libc::c_int;
}
pub unsafe extern "C" fn sdp_message_s_name_get(
    mut sdp: *mut sdp_message_t,
) -> *mut libc::c_char {
    if sdp.is_null() {
        return 0 as *mut libc::c_char;
    }
    return (*sdp).s_name;
}
pub unsafe extern "C" fn sdp_message_i_info_set(
    mut sdp: *mut sdp_message_t,
    mut pos_media: libc::c_int,
    mut info: *mut libc::c_char,
) -> libc::c_int {
    let mut med: *mut sdp_media_t = 0 as *mut sdp_media_t;
    if sdp.is_null() {
        return -(2 as libc::c_int);
    }
    if pos_media == -(1 as libc::c_int) {
        (*sdp).i_info = info;
        return 0 as libc::c_int;
    }
    med = osip_list_get(&mut (*sdp).m_medias, pos_media) as *mut sdp_media_t;
    if med.is_null() {
        return -(1 as libc::c_int);
    }
    (*med).i_info = info;
    return 0 as libc::c_int;
}
pub unsafe extern "C" fn sdp_message_i_info_get(
    mut sdp: *mut sdp_message_t,
    mut pos_media: libc::c_int,
) -> *mut libc::c_char {
    let mut med: *mut sdp_media_t = 0 as *mut sdp_media_t;
    if sdp.is_null() {
        return 0 as *mut libc::c_char;
    }
    if pos_media == -(1 as libc::c_int) {
        return (*sdp).i_info;
    }
    med = osip_list_get(&mut (*sdp).m_medias, pos_media) as *mut sdp_media_t;
    if med.is_null() {
        return 0 as *mut libc::c_char;
    }
    return (*med).i_info;
}
pub unsafe extern "C" fn sdp_message_u_uri_set(
    mut sdp: *mut sdp_message_t,
    mut uri: *mut libc::c_char,
) -> libc::c_int {
    if sdp.is_null() {
        return -(2 as libc::c_int);
    }
    (*sdp).u_uri = uri;
    return 0 as libc::c_int;
}
pub unsafe extern "C" fn sdp_message_u_uri_get(
    mut sdp: *mut sdp_message_t,
) -> *mut libc::c_char {
    if sdp.is_null() {
        return 0 as *mut libc::c_char;
    }
    return (*sdp).u_uri;
}
pub unsafe extern "C" fn sdp_message_e_email_add(
    mut sdp: *mut sdp_message_t,
    mut email: *mut libc::c_char,
) -> libc::c_int {
    if sdp.is_null() {
        return -(2 as libc::c_int);
    }
    osip_list_add(&mut (*sdp).e_emails, email as *mut libc::c_void, -(1 as libc::c_int));
    return 0 as libc::c_int;
}
pub unsafe extern "C" fn sdp_message_e_email_get(
    mut sdp: *mut sdp_message_t,
    mut pos: libc::c_int,
) -> *mut libc::c_char {
    if sdp.is_null() {
        return 0 as *mut libc::c_char;
    }
    if osip_list_size(&mut (*sdp).e_emails) > pos {
        return osip_list_get(&mut (*sdp).e_emails, pos) as *mut libc::c_char;
    }
    return 0 as *mut libc::c_char;
}
pub unsafe extern "C" fn sdp_message_p_phone_add(
    mut sdp: *mut sdp_message_t,
    mut phone: *mut libc::c_char,
) -> libc::c_int {
    if sdp.is_null() {
        return -(2 as libc::c_int);
    }
    osip_list_add(&mut (*sdp).p_phones, phone as *mut libc::c_void, -(1 as libc::c_int));
    return 0 as libc::c_int;
}
pub unsafe extern "C" fn sdp_message_p_phone_get(
    mut sdp: *mut sdp_message_t,
    mut pos: libc::c_int,
) -> *mut libc::c_char {
    if sdp.is_null() {
        return 0 as *mut libc::c_char;
    }
    if osip_list_size(&mut (*sdp).p_phones) > pos {
        return osip_list_get(&mut (*sdp).p_phones, pos) as *mut libc::c_char;
    }
    return 0 as *mut libc::c_char;
}
pub unsafe extern "C" fn sdp_message_c_connection_add(
    mut sdp: *mut sdp_message_t,
    mut pos_media: libc::c_int,
    mut nettype: *mut libc::c_char,
    mut addrtype: *mut libc::c_char,
    mut addr: *mut libc::c_char,
    mut addr_multicast_ttl: *mut libc::c_char,
    mut addr_multicast_int: *mut libc::c_char,
) -> libc::c_int {
    let mut i: libc::c_int = 0;
    let mut med: *mut sdp_media_t = 0 as *mut sdp_media_t;
    let mut conn: *mut sdp_connection_t = 0 as *mut sdp_connection_t;
    if sdp.is_null() {
        return -(2 as libc::c_int);
    }
    if pos_media != -(1 as libc::c_int)
        && osip_list_size(&mut (*sdp).m_medias) < pos_media + 1 as libc::c_int
    {
        return -(1 as libc::c_int);
    }
    i = sdp_connection_init(&mut conn);
    if i != 0 as libc::c_int {
        return i;
    }
    (*conn).c_nettype = nettype;
    (*conn).c_addrtype = addrtype;
    (*conn).c_addr = addr;
    (*conn).c_addr_multicast_ttl = addr_multicast_ttl;
    (*conn).c_addr_multicast_int = addr_multicast_int;
    if pos_media == -(1 as libc::c_int) {
        (*sdp).c_connection = conn;
        return 0 as libc::c_int;
    }
    med = osip_list_get(&mut (*sdp).m_medias, pos_media) as *mut sdp_media_t;
    osip_list_add(
        &mut (*med).c_connections,
        conn as *mut libc::c_void,
        -(1 as libc::c_int),
    );
    return 0 as libc::c_int;
}
pub unsafe extern "C" fn sdp_message_connection_get(
    mut sdp: *mut sdp_message_t,
    mut pos_media: libc::c_int,
    mut pos: libc::c_int,
) -> *mut sdp_connection_t {
    let mut med: *mut sdp_media_t = 0 as *mut sdp_media_t;
    if sdp.is_null() {
        return 0 as *mut sdp_connection_t;
    }
    if pos_media == -(1 as libc::c_int) {
        return (*sdp).c_connection;
    }
    med = osip_list_get(&mut (*sdp).m_medias, pos_media) as *mut sdp_media_t;
    if med.is_null() {
        return 0 as *mut sdp_connection_t;
    }
    return osip_list_get(&mut (*med).c_connections, pos) as *mut sdp_connection_t;
}
pub unsafe extern "C" fn sdp_message_c_nettype_get(
    mut sdp: *mut sdp_message_t,
    mut pos_media: libc::c_int,
    mut pos: libc::c_int,
) -> *mut libc::c_char {
    let mut conn: *mut sdp_connection_t = sdp_message_connection_get(
        sdp,
        pos_media,
        pos,
    );
    if conn.is_null() {
        return 0 as *mut libc::c_char;
    }
    return (*conn).c_nettype;
}
pub unsafe extern "C" fn sdp_message_c_addrtype_get(
    mut sdp: *mut sdp_message_t,
    mut pos_media: libc::c_int,
    mut pos: libc::c_int,
) -> *mut libc::c_char {
    let mut conn: *mut sdp_connection_t = sdp_message_connection_get(
        sdp,
        pos_media,
        pos,
    );
    if conn.is_null() {
        return 0 as *mut libc::c_char;
    }
    return (*conn).c_addrtype;
}
pub unsafe extern "C" fn sdp_message_c_addr_get(
    mut sdp: *mut sdp_message_t,
    mut pos_media: libc::c_int,
    mut pos: libc::c_int,
) -> *mut libc::c_char {
    let mut conn: *mut sdp_connection_t = sdp_message_connection_get(
        sdp,
        pos_media,
        pos,
    );
    if conn.is_null() {
        return 0 as *mut libc::c_char;
    }
    return (*conn).c_addr;
}
pub unsafe extern "C" fn sdp_message_c_addr_multicast_ttl_get(
    mut sdp: *mut sdp_message_t,
    mut pos_media: libc::c_int,
    mut pos: libc::c_int,
) -> *mut libc::c_char {
    let mut conn: *mut sdp_connection_t = sdp_message_connection_get(
        sdp,
        pos_media,
        pos,
    );
    if conn.is_null() {
        return 0 as *mut libc::c_char;
    }
    return (*conn).c_addr_multicast_ttl;
}
pub unsafe extern "C" fn sdp_message_c_addr_multicast_int_get(
    mut sdp: *mut sdp_message_t,
    mut pos_media: libc::c_int,
    mut pos: libc::c_int,
) -> *mut libc::c_char {
    let mut conn: *mut sdp_connection_t = sdp_message_connection_get(
        sdp,
        pos_media,
        pos,
    );
    if conn.is_null() {
        return 0 as *mut libc::c_char;
    }
    return (*conn).c_addr_multicast_int;
}
pub unsafe extern "C" fn sdp_message_b_bandwidth_add(
    mut sdp: *mut sdp_message_t,
    mut pos_media: libc::c_int,
    mut bwtype: *mut libc::c_char,
    mut bandwidth: *mut libc::c_char,
) -> libc::c_int {
    let mut i: libc::c_int = 0;
    let mut med: *mut sdp_media_t = 0 as *mut sdp_media_t;
    let mut band: *mut sdp_bandwidth_t = 0 as *mut sdp_bandwidth_t;
    if sdp.is_null() {
        return -(2 as libc::c_int);
    }
    if pos_media != -(1 as libc::c_int)
        && osip_list_size(&mut (*sdp).m_medias) < pos_media + 1 as libc::c_int
    {
        return -(1 as libc::c_int);
    }
    i = sdp_bandwidth_init(&mut band);
    if i != 0 as libc::c_int {
        return i;
    }
    (*band).b_bwtype = bwtype;
    (*band).b_bandwidth = bandwidth;
    if pos_media == -(1 as libc::c_int) {
        osip_list_add(
            &mut (*sdp).b_bandwidths,
            band as *mut libc::c_void,
            -(1 as libc::c_int),
        );
        return 0 as libc::c_int;
    }
    med = osip_list_get(&mut (*sdp).m_medias, pos_media) as *mut sdp_media_t;
    osip_list_add(
        &mut (*med).b_bandwidths,
        band as *mut libc::c_void,
        -(1 as libc::c_int),
    );
    return 0 as libc::c_int;
}
pub unsafe extern "C" fn sdp_message_bandwidth_get(
    mut sdp: *mut sdp_message_t,
    mut pos_media: libc::c_int,
    mut pos: libc::c_int,
) -> *mut sdp_bandwidth_t {
    let mut med: *mut sdp_media_t = 0 as *mut sdp_media_t;
    if sdp.is_null() {
        return 0 as *mut sdp_bandwidth_t;
    }
    if pos_media == -(1 as libc::c_int) {
        return osip_list_get(&mut (*sdp).b_bandwidths, pos) as *mut sdp_bandwidth_t;
    }
    med = osip_list_get(&mut (*sdp).m_medias, pos_media) as *mut sdp_media_t;
    if med.is_null() {
        return 0 as *mut sdp_bandwidth_t;
    }
    return osip_list_get(&mut (*med).b_bandwidths, pos) as *mut sdp_bandwidth_t;
}
pub unsafe extern "C" fn sdp_message_b_bwtype_get(
    mut sdp: *mut sdp_message_t,
    mut pos_media: libc::c_int,
    mut pos: libc::c_int,
) -> *mut libc::c_char {
    let mut band: *mut sdp_bandwidth_t = sdp_message_bandwidth_get(sdp, pos_media, pos);
    if band.is_null() {
        return 0 as *mut libc::c_char;
    }
    return (*band).b_bwtype;
}
pub unsafe extern "C" fn sdp_message_b_bandwidth_get(
    mut sdp: *mut sdp_message_t,
    mut pos_media: libc::c_int,
    mut pos: libc::c_int,
) -> *mut libc::c_char {
    let mut band: *mut sdp_bandwidth_t = sdp_message_bandwidth_get(sdp, pos_media, pos);
    if band.is_null() {
        return 0 as *mut libc::c_char;
    }
    return (*band).b_bandwidth;
}
pub unsafe extern "C" fn sdp_message_t_time_descr_add(
    mut sdp: *mut sdp_message_t,
    mut start: *mut libc::c_char,
    mut stop: *mut libc::c_char,
) -> libc::c_int {
    let mut i: libc::c_int = 0;
    let mut td: *mut sdp_time_descr_t = 0 as *mut sdp_time_descr_t;
    if sdp.is_null() {
        return -(2 as libc::c_int);
    }
    i = sdp_time_descr_init(&mut td);
    if i != 0 as libc::c_int {
        return i;
    }
    (*td).t_start_time = start;
    (*td).t_stop_time = stop;
    osip_list_add(&mut (*sdp).t_descrs, td as *mut libc::c_void, -(1 as libc::c_int));
    return 0 as libc::c_int;
}
pub unsafe extern "C" fn sdp_message_t_start_time_get(
    mut sdp: *mut sdp_message_t,
    mut pos_td: libc::c_int,
) -> *mut libc::c_char {
    let mut td: *mut sdp_time_descr_t = 0 as *mut sdp_time_descr_t;
    if sdp.is_null() {
        return 0 as *mut libc::c_char;
    }
    td = osip_list_get(&mut (*sdp).t_descrs, pos_td) as *mut sdp_time_descr_t;
    if td.is_null() {
        return 0 as *mut libc::c_char;
    }
    return (*td).t_start_time;
}
pub unsafe extern "C" fn sdp_message_t_stop_time_get(
    mut sdp: *mut sdp_message_t,
    mut pos_td: libc::c_int,
) -> *mut libc::c_char {
    let mut td: *mut sdp_time_descr_t = 0 as *mut sdp_time_descr_t;
    if sdp.is_null() {
        return 0 as *mut libc::c_char;
    }
    td = osip_list_get(&mut (*sdp).t_descrs, pos_td) as *mut sdp_time_descr_t;
    if td.is_null() {
        return 0 as *mut libc::c_char;
    }
    return (*td).t_stop_time;
}
pub unsafe extern "C" fn sdp_message_r_repeat_add(
    mut sdp: *mut sdp_message_t,
    mut pos_time_descr: libc::c_int,
    mut field: *mut libc::c_char,
) -> libc::c_int {
    let mut td: *mut sdp_time_descr_t = 0 as *mut sdp_time_descr_t;
    if sdp.is_null() {
        return -(2 as libc::c_int);
    }
    td = osip_list_get(&mut (*sdp).t_descrs, pos_time_descr) as *mut sdp_time_descr_t;
    if td.is_null() {
        return -(1 as libc::c_int);
    }
    osip_list_add(&mut (*td).r_repeats, field as *mut libc::c_void, -(1 as libc::c_int));
    return 0 as libc::c_int;
}
pub unsafe extern "C" fn sdp_message_r_repeat_get(
    mut sdp: *mut sdp_message_t,
    mut pos_time_descr: libc::c_int,
    mut pos_repeat: libc::c_int,
) -> *mut libc::c_char {
    let mut td: *mut sdp_time_descr_t = 0 as *mut sdp_time_descr_t;
    if sdp.is_null() {
        return 0 as *mut libc::c_char;
    }
    td = osip_list_get(&mut (*sdp).t_descrs, pos_time_descr) as *mut sdp_time_descr_t;
    if td.is_null() {
        return 0 as *mut libc::c_char;
    }
    return osip_list_get(&mut (*td).r_repeats, pos_repeat) as *mut libc::c_char;
}
pub unsafe extern "C" fn sdp_message_z_adjustments_set(
    mut sdp: *mut sdp_message_t,
    mut field: *mut libc::c_char,
) -> libc::c_int {
    if sdp.is_null() {
        return -(2 as libc::c_int);
    }
    (*sdp).z_adjustments = field;
    return 0 as libc::c_int;
}
pub unsafe extern "C" fn sdp_message_z_adjustments_get(
    mut sdp: *mut sdp_message_t,
) -> *mut libc::c_char {
    if sdp.is_null() {
        return 0 as *mut libc::c_char;
    }
    return (*sdp).z_adjustments;
}
pub unsafe extern "C" fn sdp_message_k_key_set(
    mut sdp: *mut sdp_message_t,
    mut pos_media: libc::c_int,
    mut keytype: *mut libc::c_char,
    mut keydata: *mut libc::c_char,
) -> libc::c_int {
    let mut key: *mut sdp_key_t = 0 as *mut sdp_key_t;
    let mut med: *mut sdp_media_t = 0 as *mut sdp_media_t;
    let mut i: libc::c_int = 0;
    if sdp.is_null() {
        return -(2 as libc::c_int);
    }
    if pos_media != -(1 as libc::c_int)
        && osip_list_size(&mut (*sdp).m_medias) < pos_media + 1 as libc::c_int
    {
        return -(1 as libc::c_int);
    }
    i = sdp_key_init(&mut key);
    if i != 0 as libc::c_int {
        return i;
    }
    (*key).k_keytype = keytype;
    (*key).k_keydata = keydata;
    if pos_media == -(1 as libc::c_int) {
        (*sdp).k_key = key;
        return 0 as libc::c_int;
    }
    med = osip_list_get(&mut (*sdp).m_medias, pos_media) as *mut sdp_media_t;
    (*med).k_key = key;
    return 0 as libc::c_int;
}
pub unsafe extern "C" fn sdp_message_k_keytype_get(
    mut sdp: *mut sdp_message_t,
    mut pos_media: libc::c_int,
) -> *mut libc::c_char {
    let mut med: *mut sdp_media_t = 0 as *mut sdp_media_t;
    if sdp.is_null() {
        return 0 as *mut libc::c_char;
    }
    if pos_media == -(1 as libc::c_int) {
        if ((*sdp).k_key).is_null() {
            return 0 as *mut libc::c_char;
        }
        return (*(*sdp).k_key).k_keytype;
    }
    if pos_media != -(1 as libc::c_int)
        && osip_list_size(&mut (*sdp).m_medias) < pos_media + 1 as libc::c_int
    {
        return 0 as *mut libc::c_char;
    }
    med = osip_list_get(&mut (*sdp).m_medias, pos_media) as *mut sdp_media_t;
    if ((*med).k_key).is_null() {
        return 0 as *mut libc::c_char;
    }
    return (*(*med).k_key).k_keytype;
}
pub unsafe extern "C" fn sdp_message_k_keydata_get(
    mut sdp: *mut sdp_message_t,
    mut pos_media: libc::c_int,
) -> *mut libc::c_char {
    let mut med: *mut sdp_media_t = 0 as *mut sdp_media_t;
    if sdp.is_null() {
        return 0 as *mut libc::c_char;
    }
    if pos_media == -(1 as libc::c_int) {
        if ((*sdp).k_key).is_null() {
            return 0 as *mut libc::c_char;
        }
        return (*(*sdp).k_key).k_keydata;
    }
    if pos_media != -(1 as libc::c_int)
        && osip_list_size(&mut (*sdp).m_medias) < pos_media + 1 as libc::c_int
    {
        return 0 as *mut libc::c_char;
    }
    med = osip_list_get(&mut (*sdp).m_medias, pos_media) as *mut sdp_media_t;
    if ((*med).k_key).is_null() {
        return 0 as *mut libc::c_char;
    }
    return (*(*med).k_key).k_keydata;
}
pub unsafe extern "C" fn sdp_message_a_attribute_add(
    mut sdp: *mut sdp_message_t,
    mut pos_media: libc::c_int,
    mut att_field: *mut libc::c_char,
    mut att_value: *mut libc::c_char,
) -> libc::c_int {
    let mut i: libc::c_int = 0;
    let mut med: *mut sdp_media_t = 0 as *mut sdp_media_t;
    let mut attr: *mut sdp_attribute_t = 0 as *mut sdp_attribute_t;
    if sdp.is_null() {
        return -(2 as libc::c_int);
    }
    if pos_media != -(1 as libc::c_int)
        && osip_list_size(&mut (*sdp).m_medias) < pos_media + 1 as libc::c_int
    {
        return -(1 as libc::c_int);
    }
    i = sdp_attribute_init(&mut attr);
    if i != 0 as libc::c_int {
        return i;
    }
    (*attr).a_att_field = att_field;
    (*attr).a_att_value = att_value;
    if pos_media == -(1 as libc::c_int) {
        osip_list_add(
            &mut (*sdp).a_attributes,
            attr as *mut libc::c_void,
            -(1 as libc::c_int),
        );
        return 0 as libc::c_int;
    }
    med = osip_list_get(&mut (*sdp).m_medias, pos_media) as *mut sdp_media_t;
    osip_list_add(
        &mut (*med).a_attributes,
        attr as *mut libc::c_void,
        -(1 as libc::c_int),
    );
    return 0 as libc::c_int;
}
pub unsafe extern "C" fn sdp_message_a_attribute_del(
    mut sdp: *mut sdp_message_t,
    mut pos_media: libc::c_int,
    mut att_field: *mut libc::c_char,
) -> libc::c_int {
    let mut i: libc::c_int = 0;
    let mut med: *mut sdp_media_t = 0 as *mut sdp_media_t;
    let mut attr: *mut sdp_attribute_t = 0 as *mut sdp_attribute_t;
    if sdp.is_null() {
        return -(2 as libc::c_int);
    }
    if pos_media != -(1 as libc::c_int)
        && osip_list_size(&mut (*sdp).m_medias) < pos_media + 1 as libc::c_int
    {
        return -(1 as libc::c_int);
    }
    if pos_media == -(1 as libc::c_int) {
        i = 0 as libc::c_int;
        while i < osip_list_size(&mut (*sdp).a_attributes) {
            attr = osip_list_get(&mut (*sdp).a_attributes, i) as *mut sdp_attribute_t;
            if strcmp((*attr).a_att_field, att_field) == 0 as libc::c_int {
                osip_list_remove(&mut (*sdp).a_attributes, i);
                sdp_attribute_free(attr);
            } else {
                i += 1;
                i;
            }
        }
        return 0 as libc::c_int;
    }
    med = osip_list_get(&mut (*sdp).m_medias, pos_media) as *mut sdp_media_t;
    if med.is_null() {
        return -(1 as libc::c_int);
    }
    i = 0 as libc::c_int;
    while i < osip_list_size(&mut (*med).a_attributes) {
        attr = osip_list_get(&mut (*med).a_attributes, i) as *mut sdp_attribute_t;
        if strcmp((*attr).a_att_field, att_field) == 0 as libc::c_int {
            osip_list_remove(&mut (*med).a_attributes, i);
            sdp_attribute_free(attr);
        } else {
            i += 1;
            i;
        }
    }
    return 0 as libc::c_int;
}
pub unsafe extern "C" fn sdp_message_a_attribute_del_at_index(
    mut sdp: *mut sdp_message_t,
    mut pos_media: libc::c_int,
    mut att_field: *mut libc::c_char,
    mut pos_attr: libc::c_int,
) -> libc::c_int {
    let mut med: *mut sdp_media_t = 0 as *mut sdp_media_t;
    let mut attr: *mut sdp_attribute_t = 0 as *mut sdp_attribute_t;
    if sdp.is_null() {
        return -(2 as libc::c_int);
    }
    if pos_media != -(1 as libc::c_int)
        && osip_list_size(&mut (*sdp).m_medias) < pos_media + 1 as libc::c_int
    {
        return -(2 as libc::c_int);
    }
    if pos_attr == -(1 as libc::c_int) {
        return sdp_message_a_attribute_del(sdp, pos_media, att_field);
    }
    if pos_media == -(1 as libc::c_int) {
        attr = osip_list_get(&mut (*sdp).a_attributes, pos_attr) as *mut sdp_attribute_t;
        if !attr.is_null() {
            if strcmp((*attr).a_att_field, att_field) == 0 as libc::c_int {
                osip_list_remove(&mut (*sdp).a_attributes, pos_attr);
                sdp_attribute_free(attr);
                return 0 as libc::c_int;
            }
        }
        return -(2 as libc::c_int);
    }
    med = osip_list_get(&mut (*sdp).m_medias, pos_media) as *mut sdp_media_t;
    if med.is_null() {
        return -(1 as libc::c_int);
    }
    attr = osip_list_get(&mut (*med).a_attributes, pos_attr) as *mut sdp_attribute_t;
    if !attr.is_null() {
        if strcmp((*attr).a_att_field, att_field) == 0 as libc::c_int {
            osip_list_remove(&mut (*med).a_attributes, pos_attr);
            sdp_attribute_free(attr);
            return 0 as libc::c_int;
        }
    }
    return -(2 as libc::c_int);
}
pub unsafe extern "C" fn sdp_message_attribute_get(
    mut sdp: *mut sdp_message_t,
    mut pos_media: libc::c_int,
    mut pos: libc::c_int,
) -> *mut sdp_attribute_t {
    let mut med: *mut sdp_media_t = 0 as *mut sdp_media_t;
    if sdp.is_null() {
        return 0 as *mut sdp_attribute_t;
    }
    if pos_media == -(1 as libc::c_int) {
        return osip_list_get(&mut (*sdp).a_attributes, pos) as *mut sdp_attribute_t;
    }
    med = osip_list_get(&mut (*sdp).m_medias, pos_media) as *mut sdp_media_t;
    if med.is_null() {
        return 0 as *mut sdp_attribute_t;
    }
    return osip_list_get(&mut (*med).a_attributes, pos) as *mut sdp_attribute_t;
}
pub unsafe extern "C" fn sdp_message_a_att_field_get(
    mut sdp: *mut sdp_message_t,
    mut pos_media: libc::c_int,
    mut pos: libc::c_int,
) -> *mut libc::c_char {
    let mut attr: *mut sdp_attribute_t = sdp_message_attribute_get(sdp, pos_media, pos);
    if attr.is_null() {
        return 0 as *mut libc::c_char;
    }
    return (*attr).a_att_field;
}
pub unsafe extern "C" fn sdp_message_a_att_value_get(
    mut sdp: *mut sdp_message_t,
    mut pos_media: libc::c_int,
    mut pos: libc::c_int,
) -> *mut libc::c_char {
    let mut attr: *mut sdp_attribute_t = sdp_message_attribute_get(sdp, pos_media, pos);
    if attr.is_null() {
        return 0 as *mut libc::c_char;
    }
    return (*attr).a_att_value;
}
pub unsafe extern "C" fn sdp_message_endof_media(
    mut sdp: *mut sdp_message_t,
    mut i: libc::c_int,
) -> libc::c_int {
    if sdp.is_null() {
        return -(2 as libc::c_int);
    }
    if i == -(1 as libc::c_int) {
        return 0 as libc::c_int;
    }
    if osip_list_eol(&mut (*sdp).m_medias, i) == 0 {
        return 0 as libc::c_int;
    }
    return -(1 as libc::c_int);
}
pub unsafe extern "C" fn sdp_message_m_media_add(
    mut sdp: *mut sdp_message_t,
    mut media: *mut libc::c_char,
    mut port: *mut libc::c_char,
    mut number_of_port: *mut libc::c_char,
    mut proto: *mut libc::c_char,
) -> libc::c_int {
    let mut i: libc::c_int = 0;
    let mut med: *mut sdp_media_t = 0 as *mut sdp_media_t;
    i = sdp_media_init(&mut med);
    if i != 0 as libc::c_int {
        return i;
    }
    (*med).m_media = media;
    (*med).m_port = port;
    (*med).m_number_of_port = number_of_port;
    (*med).m_proto = proto;
    osip_list_add(&mut (*sdp).m_medias, med as *mut libc::c_void, -(1 as libc::c_int));
    return 0 as libc::c_int;
}
pub unsafe extern "C" fn sdp_message_m_media_get(
    mut sdp: *mut sdp_message_t,
    mut pos_media: libc::c_int,
) -> *mut libc::c_char {
    let mut med: *mut sdp_media_t = osip_list_get(&mut (*sdp).m_medias, pos_media)
        as *mut sdp_media_t;
    if med.is_null() {
        return 0 as *mut libc::c_char;
    }
    return (*med).m_media;
}
pub unsafe extern "C" fn sdp_message_m_port_get(
    mut sdp: *mut sdp_message_t,
    mut pos_media: libc::c_int,
) -> *mut libc::c_char {
    let mut med: *mut sdp_media_t = osip_list_get(&mut (*sdp).m_medias, pos_media)
        as *mut sdp_media_t;
    if med.is_null() {
        return 0 as *mut libc::c_char;
    }
    return (*med).m_port;
}
pub unsafe extern "C" fn sdp_message_m_number_of_port_get(
    mut sdp: *mut sdp_message_t,
    mut pos_media: libc::c_int,
) -> *mut libc::c_char {
    let mut med: *mut sdp_media_t = osip_list_get(&mut (*sdp).m_medias, pos_media)
        as *mut sdp_media_t;
    if med.is_null() {
        return 0 as *mut libc::c_char;
    }
    return (*med).m_number_of_port;
}
pub unsafe extern "C" fn sdp_message_m_port_set(
    mut sdp: *mut sdp_message_t,
    mut pos_media: libc::c_int,
    mut port: *mut libc::c_char,
) -> libc::c_int {
    let mut med: *mut sdp_media_t = osip_list_get(&mut (*sdp).m_medias, pos_media)
        as *mut sdp_media_t;
    if med.is_null() {
        return -(2 as libc::c_int);
    }
    if !((*med).m_port).is_null() {
        if !((*med).m_port).is_null() {
            if osip_free_func.is_some() {
                osip_free_func.unwrap()((*med).m_port as *mut libc::c_void);
            } else {
                free((*med).m_port as *mut libc::c_void);
            }
        }
    }
    (*med).m_port = port;
    return 0 as libc::c_int;
}
pub unsafe extern "C" fn sdp_message_m_proto_get(
    mut sdp: *mut sdp_message_t,
    mut pos_media: libc::c_int,
) -> *mut libc::c_char {
    let mut med: *mut sdp_media_t = osip_list_get(&mut (*sdp).m_medias, pos_media)
        as *mut sdp_media_t;
    if med.is_null() {
        return 0 as *mut libc::c_char;
    }
    return (*med).m_proto;
}
pub unsafe extern "C" fn sdp_message_m_payload_add(
    mut sdp: *mut sdp_message_t,
    mut pos_media: libc::c_int,
    mut payload: *mut libc::c_char,
) -> libc::c_int {
    let mut med: *mut sdp_media_t = osip_list_get(&mut (*sdp).m_medias, pos_media)
        as *mut sdp_media_t;
    if med.is_null() {
        return -(2 as libc::c_int);
    }
    osip_list_add(
        &mut (*med).m_payloads,
        payload as *mut libc::c_void,
        -(1 as libc::c_int),
    );
    return 0 as libc::c_int;
}
pub unsafe extern "C" fn sdp_message_m_payload_get(
    mut sdp: *mut sdp_message_t,
    mut pos_media: libc::c_int,
    mut pos: libc::c_int,
) -> *mut libc::c_char {
    let mut med: *mut sdp_media_t = osip_list_get(&mut (*sdp).m_medias, pos_media)
        as *mut sdp_media_t;
    if med.is_null() {
        return 0 as *mut libc::c_char;
    }
    return osip_list_get(&mut (*med).m_payloads, pos) as *mut libc::c_char;
}
pub unsafe extern "C" fn sdp_message_m_payload_del(
    mut sdp: *mut sdp_message_t,
    mut pos_media: libc::c_int,
    mut pos: libc::c_int,
) -> libc::c_int {
    let mut med: *mut sdp_media_t = osip_list_get(&mut (*sdp).m_medias, pos_media)
        as *mut sdp_media_t;
    let mut payload: *mut libc::c_char = 0 as *mut libc::c_char;
    if med.is_null() {
        return -(2 as libc::c_int);
    }
    payload = osip_list_get(&mut (*med).m_payloads, pos) as *mut libc::c_char;
    if payload.is_null() {
        return -(1 as libc::c_int);
    }
    osip_list_remove(&mut (*med).m_payloads, pos);
    if !payload.is_null() {
        if osip_free_func.is_some() {
            osip_free_func.unwrap()(payload as *mut libc::c_void);
        } else {
            free(payload as *mut libc::c_void);
        }
    }
    return 0 as libc::c_int;
}
