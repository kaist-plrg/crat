use ::libc;
extern "C" {
    pub type _IO_wide_data;
    pub type _IO_codecvt;
    pub type _IO_marker;
    fn strchr(_: *const libc::c_char, _: libc::c_int) -> *mut libc::c_char;
    fn strlen(_: *const libc::c_char) -> libc::c_ulong;
    fn malloc(_: libc::c_ulong) -> *mut libc::c_void;
    fn realloc(_: *mut libc::c_void, _: libc::c_ulong) -> *mut libc::c_void;
    fn free(__ptr: *mut libc::c_void);
    fn osip_list_special_free(
        li: *mut osip_list_t,
        free_func: Option::<unsafe extern "C" fn(*mut libc::c_void) -> ()>,
    );
    fn osip_list_init(li: *mut osip_list_t) -> libc::c_int;
    fn osip_list_get(li: *const osip_list_t, pos: libc::c_int) -> *mut libc::c_void;
    fn osip_list_ofchar_free(li: *mut osip_list_t);
    fn osip_list_size(li: *const osip_list_t) -> libc::c_int;
    fn osip_list_eol(li: *const osip_list_t, pos: libc::c_int) -> libc::c_int;
    fn osip_list_add(
        li: *mut osip_list_t,
        element: *mut libc::c_void,
        pos: libc::c_int,
    ) -> libc::c_int;
    static mut osip_malloc_func: Option::<osip_malloc_func_t>;
    static mut osip_realloc_func: Option::<osip_realloc_func_t>;
    static mut osip_free_func: Option::<osip_free_func_t>;
    fn osip_trace(
        fi: *const libc::c_char,
        li: libc::c_int,
        level: osip_trace_level_t,
        f: *mut FILE,
        chfr: *const libc::c_char,
        _: ...
    ) -> libc::c_int;
    fn osip_strncpy(
        dest: *mut libc::c_char,
        src: *const libc::c_char,
        length: size_t,
    ) -> *mut libc::c_char;
    fn __osip_set_next_token(
        dest: *mut *mut libc::c_char,
        buf: *mut libc::c_char,
        end_separator: libc::c_int,
        next: *mut *mut libc::c_char,
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct sdp_bandwidth {
    pub b_bwtype: *mut libc::c_char,
    pub b_bandwidth: *mut libc::c_char,
}
pub type sdp_bandwidth_t = sdp_bandwidth;
pub type osip_malloc_func_t = unsafe extern "C" fn(size_t) -> *mut libc::c_void;
pub type osip_free_func_t = unsafe extern "C" fn(*mut libc::c_void) -> ();
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
pub type osip_trace_level_t = _trace_level;
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
pub type osip_realloc_func_t = unsafe extern "C" fn(
    *mut libc::c_void,
    size_t,
) -> *mut libc::c_void;
pub unsafe extern "C" fn sdp_bandwidth_init(
    mut b: *mut *mut sdp_bandwidth_t,
) -> libc::c_int {
    *b = (if osip_malloc_func.is_some() {
        osip_malloc_func
            .unwrap()(::std::mem::size_of::<sdp_bandwidth_t>() as libc::c_ulong)
    } else {
        malloc(::std::mem::size_of::<sdp_bandwidth_t>() as libc::c_ulong)
    }) as *mut sdp_bandwidth_t;
    if (*b).is_null() {
        return -(4 as libc::c_int);
    }
    (**b).b_bwtype = 0 as *mut libc::c_char;
    (**b).b_bandwidth = 0 as *mut libc::c_char;
    return 0 as libc::c_int;
}
pub unsafe extern "C" fn sdp_bandwidth_free(mut b: *mut sdp_bandwidth_t) {
    if b.is_null() {
        return;
    }
    if !((*b).b_bwtype).is_null() {
        if osip_free_func.is_some() {
            osip_free_func.unwrap()((*b).b_bwtype as *mut libc::c_void);
        } else {
            free((*b).b_bwtype as *mut libc::c_void);
        }
    }
    if !((*b).b_bandwidth).is_null() {
        if osip_free_func.is_some() {
            osip_free_func.unwrap()((*b).b_bandwidth as *mut libc::c_void);
        } else {
            free((*b).b_bandwidth as *mut libc::c_void);
        }
    }
    if !b.is_null() {
        if osip_free_func.is_some() {
            osip_free_func.unwrap()(b as *mut libc::c_void);
        } else {
            free(b as *mut libc::c_void);
        }
    }
}
pub unsafe extern "C" fn sdp_time_descr_init(
    mut td: *mut *mut sdp_time_descr_t,
) -> libc::c_int {
    *td = (if osip_malloc_func.is_some() {
        osip_malloc_func
            .unwrap()(::std::mem::size_of::<sdp_time_descr_t>() as libc::c_ulong)
    } else {
        malloc(::std::mem::size_of::<sdp_time_descr_t>() as libc::c_ulong)
    }) as *mut sdp_time_descr_t;
    if (*td).is_null() {
        return -(4 as libc::c_int);
    }
    (**td).t_start_time = 0 as *mut libc::c_char;
    (**td).t_stop_time = 0 as *mut libc::c_char;
    osip_list_init(&mut (**td).r_repeats);
    return 0 as libc::c_int;
}
pub unsafe extern "C" fn sdp_time_descr_free(mut td: *mut sdp_time_descr_t) {
    if td.is_null() {
        return;
    }
    if !((*td).t_start_time).is_null() {
        if osip_free_func.is_some() {
            osip_free_func.unwrap()((*td).t_start_time as *mut libc::c_void);
        } else {
            free((*td).t_start_time as *mut libc::c_void);
        }
    }
    if !((*td).t_stop_time).is_null() {
        if osip_free_func.is_some() {
            osip_free_func.unwrap()((*td).t_stop_time as *mut libc::c_void);
        } else {
            free((*td).t_stop_time as *mut libc::c_void);
        }
    }
    osip_list_ofchar_free(&mut (*td).r_repeats);
    if !td.is_null() {
        if osip_free_func.is_some() {
            osip_free_func.unwrap()(td as *mut libc::c_void);
        } else {
            free(td as *mut libc::c_void);
        }
    }
}
pub unsafe extern "C" fn sdp_key_init(mut key: *mut *mut sdp_key_t) -> libc::c_int {
    *key = (if osip_malloc_func.is_some() {
        osip_malloc_func.unwrap()(::std::mem::size_of::<sdp_key_t>() as libc::c_ulong)
    } else {
        malloc(::std::mem::size_of::<sdp_key_t>() as libc::c_ulong)
    }) as *mut sdp_key_t;
    if (*key).is_null() {
        return -(4 as libc::c_int);
    }
    (**key).k_keytype = 0 as *mut libc::c_char;
    (**key).k_keydata = 0 as *mut libc::c_char;
    return 0 as libc::c_int;
}
pub unsafe extern "C" fn sdp_key_free(mut key: *mut sdp_key_t) {
    if key.is_null() {
        return;
    }
    if !((*key).k_keytype).is_null() {
        if osip_free_func.is_some() {
            osip_free_func.unwrap()((*key).k_keytype as *mut libc::c_void);
        } else {
            free((*key).k_keytype as *mut libc::c_void);
        }
    }
    if !((*key).k_keydata).is_null() {
        if osip_free_func.is_some() {
            osip_free_func.unwrap()((*key).k_keydata as *mut libc::c_void);
        } else {
            free((*key).k_keydata as *mut libc::c_void);
        }
    }
    if !key.is_null() {
        if osip_free_func.is_some() {
            osip_free_func.unwrap()(key as *mut libc::c_void);
        } else {
            free(key as *mut libc::c_void);
        }
    }
}
pub unsafe extern "C" fn sdp_attribute_init(
    mut attribute: *mut *mut sdp_attribute_t,
) -> libc::c_int {
    *attribute = (if osip_malloc_func.is_some() {
        osip_malloc_func
            .unwrap()(::std::mem::size_of::<sdp_attribute_t>() as libc::c_ulong)
    } else {
        malloc(::std::mem::size_of::<sdp_attribute_t>() as libc::c_ulong)
    }) as *mut sdp_attribute_t;
    if (*attribute).is_null() {
        return -(4 as libc::c_int);
    }
    (**attribute).a_att_field = 0 as *mut libc::c_char;
    (**attribute).a_att_value = 0 as *mut libc::c_char;
    return 0 as libc::c_int;
}
pub unsafe extern "C" fn sdp_attribute_free(mut attribute: *mut sdp_attribute_t) {
    if attribute.is_null() {
        return;
    }
    if !((*attribute).a_att_field).is_null() {
        if osip_free_func.is_some() {
            osip_free_func.unwrap()((*attribute).a_att_field as *mut libc::c_void);
        } else {
            free((*attribute).a_att_field as *mut libc::c_void);
        }
    }
    if !((*attribute).a_att_value).is_null() {
        if osip_free_func.is_some() {
            osip_free_func.unwrap()((*attribute).a_att_value as *mut libc::c_void);
        } else {
            free((*attribute).a_att_value as *mut libc::c_void);
        }
    }
    if !attribute.is_null() {
        if osip_free_func.is_some() {
            osip_free_func.unwrap()(attribute as *mut libc::c_void);
        } else {
            free(attribute as *mut libc::c_void);
        }
    }
}
pub unsafe extern "C" fn sdp_connection_init(
    mut connection: *mut *mut sdp_connection_t,
) -> libc::c_int {
    *connection = (if osip_malloc_func.is_some() {
        osip_malloc_func
            .unwrap()(::std::mem::size_of::<sdp_connection_t>() as libc::c_ulong)
    } else {
        malloc(::std::mem::size_of::<sdp_connection_t>() as libc::c_ulong)
    }) as *mut sdp_connection_t;
    if (*connection).is_null() {
        return -(4 as libc::c_int);
    }
    (**connection).c_nettype = 0 as *mut libc::c_char;
    (**connection).c_addrtype = 0 as *mut libc::c_char;
    (**connection).c_addr = 0 as *mut libc::c_char;
    (**connection).c_addr_multicast_ttl = 0 as *mut libc::c_char;
    (**connection).c_addr_multicast_int = 0 as *mut libc::c_char;
    return 0 as libc::c_int;
}
pub unsafe extern "C" fn sdp_connection_free(mut connection: *mut sdp_connection_t) {
    if connection.is_null() {
        return;
    }
    if !((*connection).c_nettype).is_null() {
        if osip_free_func.is_some() {
            osip_free_func.unwrap()((*connection).c_nettype as *mut libc::c_void);
        } else {
            free((*connection).c_nettype as *mut libc::c_void);
        }
    }
    if !((*connection).c_addrtype).is_null() {
        if osip_free_func.is_some() {
            osip_free_func.unwrap()((*connection).c_addrtype as *mut libc::c_void);
        } else {
            free((*connection).c_addrtype as *mut libc::c_void);
        }
    }
    if !((*connection).c_addr).is_null() {
        if osip_free_func.is_some() {
            osip_free_func.unwrap()((*connection).c_addr as *mut libc::c_void);
        } else {
            free((*connection).c_addr as *mut libc::c_void);
        }
    }
    if !((*connection).c_addr_multicast_ttl).is_null() {
        if osip_free_func.is_some() {
            osip_free_func
                .unwrap()((*connection).c_addr_multicast_ttl as *mut libc::c_void);
        } else {
            free((*connection).c_addr_multicast_ttl as *mut libc::c_void);
        }
    }
    if !((*connection).c_addr_multicast_int).is_null() {
        if osip_free_func.is_some() {
            osip_free_func
                .unwrap()((*connection).c_addr_multicast_int as *mut libc::c_void);
        } else {
            free((*connection).c_addr_multicast_int as *mut libc::c_void);
        }
    }
    if !connection.is_null() {
        if osip_free_func.is_some() {
            osip_free_func.unwrap()(connection as *mut libc::c_void);
        } else {
            free(connection as *mut libc::c_void);
        }
    }
}
pub unsafe extern "C" fn sdp_media_init(
    mut media: *mut *mut sdp_media_t,
) -> libc::c_int {
    let mut i: libc::c_int = 0;
    *media = (if osip_malloc_func.is_some() {
        osip_malloc_func.unwrap()(::std::mem::size_of::<sdp_media_t>() as libc::c_ulong)
    } else {
        malloc(::std::mem::size_of::<sdp_media_t>() as libc::c_ulong)
    }) as *mut sdp_media_t;
    if (*media).is_null() {
        return -(4 as libc::c_int);
    }
    (**media).m_media = 0 as *mut libc::c_char;
    (**media).m_port = 0 as *mut libc::c_char;
    (**media).m_number_of_port = 0 as *mut libc::c_char;
    (**media).m_proto = 0 as *mut libc::c_char;
    i = osip_list_init(&mut (**media).m_payloads);
    if i != 0 as libc::c_int {
        if !(*media).is_null() {
            if osip_free_func.is_some() {
                osip_free_func.unwrap()(*media as *mut libc::c_void);
            } else {
                free(*media as *mut libc::c_void);
            }
        }
        *media = 0 as *mut sdp_media_t;
        return -(4 as libc::c_int);
    }
    (**media).i_info = 0 as *mut libc::c_char;
    i = osip_list_init(&mut (**media).c_connections);
    if i != 0 as libc::c_int {
        osip_list_ofchar_free(&mut (**media).m_payloads);
        if !(*media).is_null() {
            if osip_free_func.is_some() {
                osip_free_func.unwrap()(*media as *mut libc::c_void);
            } else {
                free(*media as *mut libc::c_void);
            }
        }
        *media = 0 as *mut sdp_media_t;
        return -(4 as libc::c_int);
    }
    i = osip_list_init(&mut (**media).b_bandwidths);
    if i != 0 as libc::c_int {
        osip_list_ofchar_free(&mut (**media).m_payloads);
        osip_list_special_free(
            &mut (**media).c_connections,
            ::std::mem::transmute::<
                Option::<unsafe extern "C" fn(*mut sdp_connection_t) -> ()>,
                Option::<unsafe extern "C" fn(*mut libc::c_void) -> ()>,
            >(
                Some(
                    sdp_connection_free
                        as unsafe extern "C" fn(*mut sdp_connection_t) -> (),
                ),
            ),
        );
        if !(*media).is_null() {
            if osip_free_func.is_some() {
                osip_free_func.unwrap()(*media as *mut libc::c_void);
            } else {
                free(*media as *mut libc::c_void);
            }
        }
        *media = 0 as *mut sdp_media_t;
        return -(4 as libc::c_int);
    }
    i = osip_list_init(&mut (**media).a_attributes);
    if i != 0 as libc::c_int {
        osip_list_ofchar_free(&mut (**media).m_payloads);
        osip_list_special_free(
            &mut (**media).c_connections,
            ::std::mem::transmute::<
                Option::<unsafe extern "C" fn(*mut sdp_connection_t) -> ()>,
                Option::<unsafe extern "C" fn(*mut libc::c_void) -> ()>,
            >(
                Some(
                    sdp_connection_free
                        as unsafe extern "C" fn(*mut sdp_connection_t) -> (),
                ),
            ),
        );
        osip_list_special_free(
            &mut (**media).b_bandwidths,
            ::std::mem::transmute::<
                Option::<unsafe extern "C" fn(*mut sdp_bandwidth_t) -> ()>,
                Option::<unsafe extern "C" fn(*mut libc::c_void) -> ()>,
            >(
                Some(
                    sdp_bandwidth_free
                        as unsafe extern "C" fn(*mut sdp_bandwidth_t) -> (),
                ),
            ),
        );
        if !(*media).is_null() {
            if osip_free_func.is_some() {
                osip_free_func.unwrap()(*media as *mut libc::c_void);
            } else {
                free(*media as *mut libc::c_void);
            }
        }
        *media = 0 as *mut sdp_media_t;
        return -(4 as libc::c_int);
    }
    (**media).k_key = 0 as *mut sdp_key_t;
    return 0 as libc::c_int;
}
pub unsafe extern "C" fn sdp_media_free(mut media: *mut sdp_media_t) {
    if media.is_null() {
        return;
    }
    if !((*media).m_media).is_null() {
        if osip_free_func.is_some() {
            osip_free_func.unwrap()((*media).m_media as *mut libc::c_void);
        } else {
            free((*media).m_media as *mut libc::c_void);
        }
    }
    if !((*media).m_port).is_null() {
        if osip_free_func.is_some() {
            osip_free_func.unwrap()((*media).m_port as *mut libc::c_void);
        } else {
            free((*media).m_port as *mut libc::c_void);
        }
    }
    if !((*media).m_number_of_port).is_null() {
        if osip_free_func.is_some() {
            osip_free_func.unwrap()((*media).m_number_of_port as *mut libc::c_void);
        } else {
            free((*media).m_number_of_port as *mut libc::c_void);
        }
    }
    if !((*media).m_proto).is_null() {
        if osip_free_func.is_some() {
            osip_free_func.unwrap()((*media).m_proto as *mut libc::c_void);
        } else {
            free((*media).m_proto as *mut libc::c_void);
        }
    }
    osip_list_ofchar_free(&mut (*media).m_payloads);
    if !((*media).i_info).is_null() {
        if osip_free_func.is_some() {
            osip_free_func.unwrap()((*media).i_info as *mut libc::c_void);
        } else {
            free((*media).i_info as *mut libc::c_void);
        }
    }
    osip_list_special_free(
        &mut (*media).c_connections,
        ::std::mem::transmute::<
            Option::<unsafe extern "C" fn(*mut sdp_connection_t) -> ()>,
            Option::<unsafe extern "C" fn(*mut libc::c_void) -> ()>,
        >(Some(sdp_connection_free as unsafe extern "C" fn(*mut sdp_connection_t) -> ())),
    );
    osip_list_special_free(
        &mut (*media).b_bandwidths,
        ::std::mem::transmute::<
            Option::<unsafe extern "C" fn(*mut sdp_bandwidth_t) -> ()>,
            Option::<unsafe extern "C" fn(*mut libc::c_void) -> ()>,
        >(Some(sdp_bandwidth_free as unsafe extern "C" fn(*mut sdp_bandwidth_t) -> ())),
    );
    osip_list_special_free(
        &mut (*media).a_attributes,
        ::std::mem::transmute::<
            Option::<unsafe extern "C" fn(*mut sdp_attribute_t) -> ()>,
            Option::<unsafe extern "C" fn(*mut libc::c_void) -> ()>,
        >(Some(sdp_attribute_free as unsafe extern "C" fn(*mut sdp_attribute_t) -> ())),
    );
    sdp_key_free((*media).k_key);
    if !media.is_null() {
        if osip_free_func.is_some() {
            osip_free_func.unwrap()(media as *mut libc::c_void);
        } else {
            free(media as *mut libc::c_void);
        }
    }
}
pub unsafe extern "C" fn sdp_message_init(
    mut sdp: *mut *mut sdp_message_t,
) -> libc::c_int {
    let mut i: libc::c_int = 0;
    *sdp = (if osip_malloc_func.is_some() {
        osip_malloc_func
            .unwrap()(::std::mem::size_of::<sdp_message_t>() as libc::c_ulong)
    } else {
        malloc(::std::mem::size_of::<sdp_message_t>() as libc::c_ulong)
    }) as *mut sdp_message_t;
    if (*sdp).is_null() {
        return -(4 as libc::c_int);
    }
    (**sdp).v_version = 0 as *mut libc::c_char;
    (**sdp).o_username = 0 as *mut libc::c_char;
    (**sdp).o_sess_id = 0 as *mut libc::c_char;
    (**sdp).o_sess_version = 0 as *mut libc::c_char;
    (**sdp).o_nettype = 0 as *mut libc::c_char;
    (**sdp).o_addrtype = 0 as *mut libc::c_char;
    (**sdp).o_addr = 0 as *mut libc::c_char;
    (**sdp).s_name = 0 as *mut libc::c_char;
    (**sdp).i_info = 0 as *mut libc::c_char;
    (**sdp).u_uri = 0 as *mut libc::c_char;
    i = osip_list_init(&mut (**sdp).e_emails);
    if i != 0 as libc::c_int {
        osip_list_ofchar_free(&mut (**sdp).e_emails);
        if !(*sdp).is_null() {
            if osip_free_func.is_some() {
                osip_free_func.unwrap()(*sdp as *mut libc::c_void);
            } else {
                free(*sdp as *mut libc::c_void);
            }
        }
        *sdp = 0 as *mut sdp_message_t;
        return -(4 as libc::c_int);
    }
    i = osip_list_init(&mut (**sdp).p_phones);
    if i != 0 as libc::c_int {
        osip_list_ofchar_free(&mut (**sdp).e_emails);
        if !(*sdp).is_null() {
            if osip_free_func.is_some() {
                osip_free_func.unwrap()(*sdp as *mut libc::c_void);
            } else {
                free(*sdp as *mut libc::c_void);
            }
        }
        *sdp = 0 as *mut sdp_message_t;
        return -(4 as libc::c_int);
    }
    (**sdp).c_connection = 0 as *mut sdp_connection_t;
    i = osip_list_init(&mut (**sdp).b_bandwidths);
    if i != 0 as libc::c_int {
        osip_list_ofchar_free(&mut (**sdp).e_emails);
        osip_list_ofchar_free(&mut (**sdp).p_phones);
        if !(*sdp).is_null() {
            if osip_free_func.is_some() {
                osip_free_func.unwrap()(*sdp as *mut libc::c_void);
            } else {
                free(*sdp as *mut libc::c_void);
            }
        }
        *sdp = 0 as *mut sdp_message_t;
        return -(4 as libc::c_int);
    }
    i = osip_list_init(&mut (**sdp).t_descrs);
    if i != 0 as libc::c_int {
        osip_list_ofchar_free(&mut (**sdp).e_emails);
        osip_list_ofchar_free(&mut (**sdp).p_phones);
        osip_list_special_free(
            &mut (**sdp).b_bandwidths,
            ::std::mem::transmute::<
                Option::<unsafe extern "C" fn(*mut sdp_bandwidth_t) -> ()>,
                Option::<unsafe extern "C" fn(*mut libc::c_void) -> ()>,
            >(
                Some(
                    sdp_bandwidth_free
                        as unsafe extern "C" fn(*mut sdp_bandwidth_t) -> (),
                ),
            ),
        );
        if !(*sdp).is_null() {
            if osip_free_func.is_some() {
                osip_free_func.unwrap()(*sdp as *mut libc::c_void);
            } else {
                free(*sdp as *mut libc::c_void);
            }
        }
        *sdp = 0 as *mut sdp_message_t;
        return -(4 as libc::c_int);
    }
    (**sdp).z_adjustments = 0 as *mut libc::c_char;
    (**sdp).k_key = 0 as *mut sdp_key_t;
    i = osip_list_init(&mut (**sdp).a_attributes);
    if i != 0 as libc::c_int {
        osip_list_ofchar_free(&mut (**sdp).e_emails);
        osip_list_ofchar_free(&mut (**sdp).p_phones);
        osip_list_special_free(
            &mut (**sdp).b_bandwidths,
            ::std::mem::transmute::<
                Option::<unsafe extern "C" fn(*mut sdp_bandwidth_t) -> ()>,
                Option::<unsafe extern "C" fn(*mut libc::c_void) -> ()>,
            >(
                Some(
                    sdp_bandwidth_free
                        as unsafe extern "C" fn(*mut sdp_bandwidth_t) -> (),
                ),
            ),
        );
        osip_list_special_free(
            &mut (**sdp).t_descrs,
            ::std::mem::transmute::<
                Option::<unsafe extern "C" fn(*mut sdp_time_descr_t) -> ()>,
                Option::<unsafe extern "C" fn(*mut libc::c_void) -> ()>,
            >(
                Some(
                    sdp_time_descr_free
                        as unsafe extern "C" fn(*mut sdp_time_descr_t) -> (),
                ),
            ),
        );
        if !(*sdp).is_null() {
            if osip_free_func.is_some() {
                osip_free_func.unwrap()(*sdp as *mut libc::c_void);
            } else {
                free(*sdp as *mut libc::c_void);
            }
        }
        *sdp = 0 as *mut sdp_message_t;
        return -(4 as libc::c_int);
    }
    i = osip_list_init(&mut (**sdp).m_medias);
    if i != 0 as libc::c_int {
        osip_list_ofchar_free(&mut (**sdp).e_emails);
        osip_list_ofchar_free(&mut (**sdp).p_phones);
        osip_list_special_free(
            &mut (**sdp).b_bandwidths,
            ::std::mem::transmute::<
                Option::<unsafe extern "C" fn(*mut sdp_bandwidth_t) -> ()>,
                Option::<unsafe extern "C" fn(*mut libc::c_void) -> ()>,
            >(
                Some(
                    sdp_bandwidth_free
                        as unsafe extern "C" fn(*mut sdp_bandwidth_t) -> (),
                ),
            ),
        );
        osip_list_special_free(
            &mut (**sdp).t_descrs,
            ::std::mem::transmute::<
                Option::<unsafe extern "C" fn(*mut sdp_time_descr_t) -> ()>,
                Option::<unsafe extern "C" fn(*mut libc::c_void) -> ()>,
            >(
                Some(
                    sdp_time_descr_free
                        as unsafe extern "C" fn(*mut sdp_time_descr_t) -> (),
                ),
            ),
        );
        osip_list_special_free(
            &mut (**sdp).a_attributes,
            ::std::mem::transmute::<
                Option::<unsafe extern "C" fn(*mut sdp_attribute_t) -> ()>,
                Option::<unsafe extern "C" fn(*mut libc::c_void) -> ()>,
            >(
                Some(
                    sdp_attribute_free
                        as unsafe extern "C" fn(*mut sdp_attribute_t) -> (),
                ),
            ),
        );
        if !(*sdp).is_null() {
            if osip_free_func.is_some() {
                osip_free_func.unwrap()(*sdp as *mut libc::c_void);
            } else {
                free(*sdp as *mut libc::c_void);
            }
        }
        *sdp = 0 as *mut sdp_message_t;
        return -(4 as libc::c_int);
    }
    return 0 as libc::c_int;
}
unsafe extern "C" fn __osip_sdp_append_string(
    mut string: *mut *mut libc::c_char,
    mut size: *mut libc::c_int,
    mut cur: *mut libc::c_char,
    mut string_osip_to_append: *mut libc::c_char,
) -> *mut libc::c_char {
    let mut length: libc::c_int = strlen(string_osip_to_append) as libc::c_int;
    if cur.offset_from(*string) as libc::c_long + length as libc::c_long
        + 1 as libc::c_int as libc::c_long > *size as libc::c_long
    {
        let mut length2: libc::c_int = 0;
        length2 = cur.offset_from(*string) as libc::c_long as libc::c_int;
        *string = (if osip_realloc_func.is_some() {
            osip_realloc_func
                .unwrap()(
                *string as *mut libc::c_void,
                (*size + length + 500 as libc::c_int) as size_t,
            )
        } else {
            realloc(
                *string as *mut libc::c_void,
                (*size + length + 500 as libc::c_int) as libc::c_ulong,
            )
        }) as *mut libc::c_char;
        *size = *size + length + 500 as libc::c_int;
        cur = (*string).offset(length2 as isize);
    }
    osip_strncpy(cur, string_osip_to_append, length as size_t);
    return cur.offset(strlen(cur) as isize);
}
unsafe extern "C" fn sdp_message_parse_v(
    mut sdp: *mut sdp_message_t,
    mut buf: *mut libc::c_char,
    mut next: *mut *mut libc::c_char,
) -> libc::c_int {
    let mut equal: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut crlf: *mut libc::c_char = 0 as *mut libc::c_char;
    *next = buf;
    equal = buf;
    while *equal as libc::c_int != '=' as i32 && *equal as libc::c_int != '\0' as i32 {
        equal = equal.offset(1);
        equal;
    }
    if *equal as libc::c_int == '\0' as i32 {
        return -(1 as libc::c_int);
    }
    if equal == buf {
        return 0 as libc::c_int;
    }
    if *equal.offset(-(1 as libc::c_int) as isize) as libc::c_int != 'v' as i32 {
        return 0 as libc::c_int;
    }
    crlf = equal.offset(1 as libc::c_int as isize);
    while *crlf as libc::c_int != '\r' as i32 && *crlf as libc::c_int != '\n' as i32
        && *crlf as libc::c_int != '\0' as i32
    {
        crlf = crlf.offset(1);
        crlf;
    }
    if *crlf as libc::c_int == '\0' as i32 {
        return -(1 as libc::c_int);
    }
    if crlf == equal.offset(1 as libc::c_int as isize) {
        return -(1 as libc::c_int);
    }
    (*sdp)
        .v_version = (if osip_malloc_func.is_some() {
        osip_malloc_func
            .unwrap()(
            (crlf.offset_from(equal.offset(1 as libc::c_int as isize)) as libc::c_long
                + 1 as libc::c_int as libc::c_long) as size_t,
        )
    } else {
        malloc(
            (crlf.offset_from(equal.offset(1 as libc::c_int as isize)) as libc::c_long
                + 1 as libc::c_int as libc::c_long) as libc::c_ulong,
        )
    }) as *mut libc::c_char;
    if ((*sdp).v_version).is_null() {
        return -(4 as libc::c_int);
    }
    osip_strncpy(
        (*sdp).v_version,
        equal.offset(1 as libc::c_int as isize),
        crlf.offset_from(equal.offset(1 as libc::c_int as isize)) as libc::c_long
            as size_t,
    );
    if *crlf.offset(1 as libc::c_int as isize) as libc::c_int == '\n' as i32 {
        *next = crlf.offset(2 as libc::c_int as isize);
    } else {
        *next = crlf.offset(1 as libc::c_int as isize);
    }
    return 1 as libc::c_int;
}
unsafe extern "C" fn sdp_message_parse_o(
    mut sdp: *mut sdp_message_t,
    mut buf: *mut libc::c_char,
    mut next: *mut *mut libc::c_char,
) -> libc::c_int {
    let mut equal: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut crlf: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut tmp: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut tmp_next: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut i: libc::c_int = 0;
    *next = buf;
    equal = buf;
    while *equal as libc::c_int != '=' as i32 && *equal as libc::c_int != '\0' as i32 {
        equal = equal.offset(1);
        equal;
    }
    if *equal as libc::c_int == '\0' as i32 {
        return -(1 as libc::c_int);
    }
    if *equal.offset(-(1 as libc::c_int) as isize) as libc::c_int != 'o' as i32 {
        return 0 as libc::c_int;
    }
    crlf = equal.offset(1 as libc::c_int as isize);
    while *crlf as libc::c_int != '\r' as i32 && *crlf as libc::c_int != '\n' as i32
        && *crlf as libc::c_int != '\0' as i32
    {
        crlf = crlf.offset(1);
        crlf;
    }
    if *crlf as libc::c_int == '\0' as i32 {
        return -(1 as libc::c_int);
    }
    if crlf == equal.offset(1 as libc::c_int as isize) {
        return -(1 as libc::c_int);
    }
    tmp = equal.offset(1 as libc::c_int as isize);
    i = __osip_set_next_token(&mut (*sdp).o_username, tmp, ' ' as i32, &mut tmp_next);
    if i != 0 as libc::c_int {
        return -(1 as libc::c_int);
    }
    tmp = tmp_next;
    i = __osip_set_next_token(&mut (*sdp).o_sess_id, tmp, ' ' as i32, &mut tmp_next);
    if i != 0 as libc::c_int {
        return -(1 as libc::c_int);
    }
    tmp = tmp_next;
    i = __osip_set_next_token(
        &mut (*sdp).o_sess_version,
        tmp,
        ' ' as i32,
        &mut tmp_next,
    );
    if i != 0 as libc::c_int {
        return -(1 as libc::c_int);
    }
    tmp = tmp_next;
    i = __osip_set_next_token(&mut (*sdp).o_nettype, tmp, ' ' as i32, &mut tmp_next);
    if i != 0 as libc::c_int {
        return -(1 as libc::c_int);
    }
    tmp = tmp_next;
    i = __osip_set_next_token(&mut (*sdp).o_addrtype, tmp, ' ' as i32, &mut tmp_next);
    if i != 0 as libc::c_int {
        return -(1 as libc::c_int);
    }
    tmp = tmp_next;
    i = __osip_set_next_token(&mut (*sdp).o_addr, tmp, '\r' as i32, &mut tmp_next);
    if i != 0 as libc::c_int {
        i = __osip_set_next_token(&mut (*sdp).o_addr, tmp, '\n' as i32, &mut tmp_next);
        if i != 0 as libc::c_int {
            return -(1 as libc::c_int);
        }
    }
    if *crlf.offset(1 as libc::c_int as isize) as libc::c_int == '\n' as i32 {
        *next = crlf.offset(2 as libc::c_int as isize);
    } else {
        *next = crlf.offset(1 as libc::c_int as isize);
    }
    return 1 as libc::c_int;
}
unsafe extern "C" fn sdp_message_parse_s(
    mut sdp: *mut sdp_message_t,
    mut buf: *mut libc::c_char,
    mut next: *mut *mut libc::c_char,
) -> libc::c_int {
    let mut equal: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut crlf: *mut libc::c_char = 0 as *mut libc::c_char;
    *next = buf;
    equal = buf;
    while *equal as libc::c_int != '=' as i32 && *equal as libc::c_int != '\0' as i32 {
        equal = equal.offset(1);
        equal;
    }
    if *equal as libc::c_int == '\0' as i32 {
        return -(1 as libc::c_int);
    }
    if *equal.offset(-(1 as libc::c_int) as isize) as libc::c_int != 's' as i32 {
        return 0 as libc::c_int;
    }
    crlf = equal.offset(1 as libc::c_int as isize);
    while *crlf as libc::c_int != '\r' as i32 && *crlf as libc::c_int != '\n' as i32
        && *crlf as libc::c_int != '\0' as i32
    {
        crlf = crlf.offset(1);
        crlf;
    }
    if *crlf as libc::c_int == '\0' as i32 {
        return -(1 as libc::c_int);
    }
    if crlf == equal.offset(1 as libc::c_int as isize) {
        return -(1 as libc::c_int);
    }
    (*sdp)
        .s_name = (if osip_malloc_func.is_some() {
        osip_malloc_func
            .unwrap()(
            (crlf.offset_from(equal.offset(1 as libc::c_int as isize)) as libc::c_long
                + 1 as libc::c_int as libc::c_long) as size_t,
        )
    } else {
        malloc(
            (crlf.offset_from(equal.offset(1 as libc::c_int as isize)) as libc::c_long
                + 1 as libc::c_int as libc::c_long) as libc::c_ulong,
        )
    }) as *mut libc::c_char;
    if ((*sdp).s_name).is_null() {
        return -(4 as libc::c_int);
    }
    osip_strncpy(
        (*sdp).s_name,
        equal.offset(1 as libc::c_int as isize),
        crlf.offset_from(equal.offset(1 as libc::c_int as isize)) as libc::c_long
            as size_t,
    );
    if *crlf.offset(1 as libc::c_int as isize) as libc::c_int == '\n' as i32 {
        *next = crlf.offset(2 as libc::c_int as isize);
    } else {
        *next = crlf.offset(1 as libc::c_int as isize);
    }
    return 1 as libc::c_int;
}
unsafe extern "C" fn sdp_message_parse_i(
    mut sdp: *mut sdp_message_t,
    mut buf: *mut libc::c_char,
    mut next: *mut *mut libc::c_char,
) -> libc::c_int {
    let mut equal: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut crlf: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut i: libc::c_int = 0;
    let mut i_info: *mut libc::c_char = 0 as *mut libc::c_char;
    *next = buf;
    equal = buf;
    while *equal as libc::c_int != '=' as i32 && *equal as libc::c_int != '\0' as i32 {
        equal = equal.offset(1);
        equal;
    }
    if *equal as libc::c_int == '\0' as i32 {
        return -(1 as libc::c_int);
    }
    if *equal.offset(-(1 as libc::c_int) as isize) as libc::c_int != 'i' as i32 {
        return 0 as libc::c_int;
    }
    crlf = equal.offset(1 as libc::c_int as isize);
    while *crlf as libc::c_int != '\r' as i32 && *crlf as libc::c_int != '\n' as i32
        && *crlf as libc::c_int != '\0' as i32
    {
        crlf = crlf.offset(1);
        crlf;
    }
    if *crlf as libc::c_int == '\0' as i32 {
        return -(1 as libc::c_int);
    }
    if crlf == equal.offset(1 as libc::c_int as isize) {
        return -(1 as libc::c_int);
    }
    i_info = (if osip_malloc_func.is_some() {
        osip_malloc_func
            .unwrap()(
            (crlf.offset_from(equal.offset(1 as libc::c_int as isize)) as libc::c_long
                + 1 as libc::c_int as libc::c_long) as size_t,
        )
    } else {
        malloc(
            (crlf.offset_from(equal.offset(1 as libc::c_int as isize)) as libc::c_long
                + 1 as libc::c_int as libc::c_long) as libc::c_ulong,
        )
    }) as *mut libc::c_char;
    if i_info.is_null() {
        return -(4 as libc::c_int);
    }
    osip_strncpy(
        i_info,
        equal.offset(1 as libc::c_int as isize),
        crlf.offset_from(equal.offset(1 as libc::c_int as isize)) as libc::c_long
            as size_t,
    );
    i = osip_list_size(&mut (*sdp).m_medias);
    if i == 0 as libc::c_int {
        (*sdp).i_info = i_info;
    } else {
        let mut last_sdp_media: *mut sdp_media_t = osip_list_get(
            &mut (*sdp).m_medias,
            i - 1 as libc::c_int,
        ) as *mut sdp_media_t;
        (*last_sdp_media).i_info = i_info;
    }
    if *crlf.offset(1 as libc::c_int as isize) as libc::c_int == '\n' as i32 {
        *next = crlf.offset(2 as libc::c_int as isize);
    } else {
        *next = crlf.offset(1 as libc::c_int as isize);
    }
    return 1 as libc::c_int;
}
unsafe extern "C" fn sdp_message_parse_u(
    mut sdp: *mut sdp_message_t,
    mut buf: *mut libc::c_char,
    mut next: *mut *mut libc::c_char,
) -> libc::c_int {
    let mut equal: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut crlf: *mut libc::c_char = 0 as *mut libc::c_char;
    *next = buf;
    equal = buf;
    while *equal as libc::c_int != '=' as i32 && *equal as libc::c_int != '\0' as i32 {
        equal = equal.offset(1);
        equal;
    }
    if *equal as libc::c_int == '\0' as i32 {
        return -(1 as libc::c_int);
    }
    if *equal.offset(-(1 as libc::c_int) as isize) as libc::c_int != 'u' as i32 {
        return 0 as libc::c_int;
    }
    crlf = equal.offset(1 as libc::c_int as isize);
    while *crlf as libc::c_int != '\r' as i32 && *crlf as libc::c_int != '\n' as i32
        && *crlf as libc::c_int != '\0' as i32
    {
        crlf = crlf.offset(1);
        crlf;
    }
    if *crlf as libc::c_int == '\0' as i32 {
        return -(1 as libc::c_int);
    }
    if crlf == equal.offset(1 as libc::c_int as isize) {
        return -(1 as libc::c_int);
    }
    (*sdp)
        .u_uri = (if osip_malloc_func.is_some() {
        osip_malloc_func
            .unwrap()(
            (crlf.offset_from(equal.offset(1 as libc::c_int as isize)) as libc::c_long
                + 1 as libc::c_int as libc::c_long) as size_t,
        )
    } else {
        malloc(
            (crlf.offset_from(equal.offset(1 as libc::c_int as isize)) as libc::c_long
                + 1 as libc::c_int as libc::c_long) as libc::c_ulong,
        )
    }) as *mut libc::c_char;
    if ((*sdp).u_uri).is_null() {
        return -(4 as libc::c_int);
    }
    osip_strncpy(
        (*sdp).u_uri,
        equal.offset(1 as libc::c_int as isize),
        crlf.offset_from(equal.offset(1 as libc::c_int as isize)) as libc::c_long
            as size_t,
    );
    if *crlf.offset(1 as libc::c_int as isize) as libc::c_int == '\n' as i32 {
        *next = crlf.offset(2 as libc::c_int as isize);
    } else {
        *next = crlf.offset(1 as libc::c_int as isize);
    }
    return 1 as libc::c_int;
}
unsafe extern "C" fn sdp_message_parse_e(
    mut sdp: *mut sdp_message_t,
    mut buf: *mut libc::c_char,
    mut next: *mut *mut libc::c_char,
) -> libc::c_int {
    let mut equal: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut crlf: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut e_email: *mut libc::c_char = 0 as *mut libc::c_char;
    *next = buf;
    equal = buf;
    while *equal as libc::c_int != '=' as i32 && *equal as libc::c_int != '\0' as i32 {
        equal = equal.offset(1);
        equal;
    }
    if *equal as libc::c_int == '\0' as i32 {
        return -(1 as libc::c_int);
    }
    if *equal.offset(-(1 as libc::c_int) as isize) as libc::c_int != 'e' as i32 {
        return 0 as libc::c_int;
    }
    crlf = equal.offset(1 as libc::c_int as isize);
    while *crlf as libc::c_int != '\r' as i32 && *crlf as libc::c_int != '\n' as i32
        && *crlf as libc::c_int != '\0' as i32
    {
        crlf = crlf.offset(1);
        crlf;
    }
    if *crlf as libc::c_int == '\0' as i32 {
        return -(1 as libc::c_int);
    }
    if crlf == equal.offset(1 as libc::c_int as isize) {
        return -(1 as libc::c_int);
    }
    e_email = (if osip_malloc_func.is_some() {
        osip_malloc_func
            .unwrap()(
            (crlf.offset_from(equal.offset(1 as libc::c_int as isize)) as libc::c_long
                + 1 as libc::c_int as libc::c_long) as size_t,
        )
    } else {
        malloc(
            (crlf.offset_from(equal.offset(1 as libc::c_int as isize)) as libc::c_long
                + 1 as libc::c_int as libc::c_long) as libc::c_ulong,
        )
    }) as *mut libc::c_char;
    if e_email.is_null() {
        return -(4 as libc::c_int);
    }
    osip_strncpy(
        e_email,
        equal.offset(1 as libc::c_int as isize),
        crlf.offset_from(equal.offset(1 as libc::c_int as isize)) as libc::c_long
            as size_t,
    );
    osip_list_add(
        &mut (*sdp).e_emails,
        e_email as *mut libc::c_void,
        -(1 as libc::c_int),
    );
    if *crlf.offset(1 as libc::c_int as isize) as libc::c_int == '\n' as i32 {
        *next = crlf.offset(2 as libc::c_int as isize);
    } else {
        *next = crlf.offset(1 as libc::c_int as isize);
    }
    return 1 as libc::c_int;
}
unsafe extern "C" fn sdp_message_parse_p(
    mut sdp: *mut sdp_message_t,
    mut buf: *mut libc::c_char,
    mut next: *mut *mut libc::c_char,
) -> libc::c_int {
    let mut equal: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut crlf: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut p_phone: *mut libc::c_char = 0 as *mut libc::c_char;
    *next = buf;
    equal = buf;
    while *equal as libc::c_int != '=' as i32 && *equal as libc::c_int != '\0' as i32 {
        equal = equal.offset(1);
        equal;
    }
    if *equal as libc::c_int == '\0' as i32 {
        return -(1 as libc::c_int);
    }
    if *equal.offset(-(1 as libc::c_int) as isize) as libc::c_int != 'p' as i32 {
        return 0 as libc::c_int;
    }
    crlf = equal.offset(1 as libc::c_int as isize);
    while *crlf as libc::c_int != '\r' as i32 && *crlf as libc::c_int != '\n' as i32
        && *crlf as libc::c_int != '\0' as i32
    {
        crlf = crlf.offset(1);
        crlf;
    }
    if *crlf as libc::c_int == '\0' as i32 {
        return -(1 as libc::c_int);
    }
    if crlf == equal.offset(1 as libc::c_int as isize) {
        return -(1 as libc::c_int);
    }
    p_phone = (if osip_malloc_func.is_some() {
        osip_malloc_func
            .unwrap()(
            (crlf.offset_from(equal.offset(1 as libc::c_int as isize)) as libc::c_long
                + 1 as libc::c_int as libc::c_long) as size_t,
        )
    } else {
        malloc(
            (crlf.offset_from(equal.offset(1 as libc::c_int as isize)) as libc::c_long
                + 1 as libc::c_int as libc::c_long) as libc::c_ulong,
        )
    }) as *mut libc::c_char;
    if p_phone.is_null() {
        return -(4 as libc::c_int);
    }
    osip_strncpy(
        p_phone,
        equal.offset(1 as libc::c_int as isize),
        crlf.offset_from(equal.offset(1 as libc::c_int as isize)) as libc::c_long
            as size_t,
    );
    osip_list_add(
        &mut (*sdp).p_phones,
        p_phone as *mut libc::c_void,
        -(1 as libc::c_int),
    );
    if *crlf.offset(1 as libc::c_int as isize) as libc::c_int == '\n' as i32 {
        *next = crlf.offset(2 as libc::c_int as isize);
    } else {
        *next = crlf.offset(1 as libc::c_int as isize);
    }
    return 1 as libc::c_int;
}
unsafe extern "C" fn sdp_message_parse_c(
    mut sdp: *mut sdp_message_t,
    mut buf: *mut libc::c_char,
    mut next: *mut *mut libc::c_char,
) -> libc::c_int {
    let mut equal: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut crlf: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut tmp: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut tmp_next: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut c_header: *mut sdp_connection_t = 0 as *mut sdp_connection_t;
    let mut i: libc::c_int = 0;
    *next = buf;
    equal = buf;
    while *equal as libc::c_int != '=' as i32 && *equal as libc::c_int != '\0' as i32 {
        equal = equal.offset(1);
        equal;
    }
    if *equal as libc::c_int == '\0' as i32 {
        return -(1 as libc::c_int);
    }
    if *equal.offset(-(1 as libc::c_int) as isize) as libc::c_int != 'c' as i32 {
        return 0 as libc::c_int;
    }
    crlf = equal.offset(1 as libc::c_int as isize);
    while *crlf as libc::c_int != '\r' as i32 && *crlf as libc::c_int != '\n' as i32
        && *crlf as libc::c_int != '\0' as i32
    {
        crlf = crlf.offset(1);
        crlf;
    }
    if *crlf as libc::c_int == '\0' as i32 {
        return -(1 as libc::c_int);
    }
    if crlf == equal.offset(1 as libc::c_int as isize) {
        return -(1 as libc::c_int);
    }
    tmp = equal.offset(1 as libc::c_int as isize);
    i = sdp_connection_init(&mut c_header);
    if i != 0 as libc::c_int {
        return -(1 as libc::c_int);
    }
    i = __osip_set_next_token(
        &mut (*c_header).c_nettype,
        tmp,
        ' ' as i32,
        &mut tmp_next,
    );
    if i != 0 as libc::c_int {
        sdp_connection_free(c_header);
        return -(1 as libc::c_int);
    }
    tmp = tmp_next;
    i = __osip_set_next_token(
        &mut (*c_header).c_addrtype,
        tmp,
        ' ' as i32,
        &mut tmp_next,
    );
    if i != 0 as libc::c_int {
        sdp_connection_free(c_header);
        return -(1 as libc::c_int);
    }
    tmp = tmp_next;
    let mut slash: *mut libc::c_char = strchr(tmp, '/' as i32);
    if !slash.is_null() && slash < crlf {
        i = __osip_set_next_token(
            &mut (*c_header).c_addr,
            tmp,
            '/' as i32,
            &mut tmp_next,
        );
        if i != 0 as libc::c_int {
            sdp_connection_free(c_header);
            return -(1 as libc::c_int);
        }
        tmp = tmp_next;
        slash = strchr(slash.offset(1 as libc::c_int as isize), '/' as i32);
        if !slash.is_null() && slash < crlf {
            i = __osip_set_next_token(
                &mut (*c_header).c_addr_multicast_ttl,
                tmp,
                '/' as i32,
                &mut tmp_next,
            );
            if i != 0 as libc::c_int {
                sdp_connection_free(c_header);
                return -(1 as libc::c_int);
            }
            tmp = tmp_next;
            i = __osip_set_next_token(
                &mut (*c_header).c_addr_multicast_int,
                tmp,
                '\r' as i32,
                &mut tmp_next,
            );
            if i != 0 as libc::c_int {
                i = __osip_set_next_token(
                    &mut (*c_header).c_addr_multicast_int,
                    tmp,
                    '\n' as i32,
                    &mut tmp_next,
                );
                if i != 0 as libc::c_int {
                    sdp_connection_free(c_header);
                    return -(1 as libc::c_int);
                }
            }
        } else {
            i = __osip_set_next_token(
                &mut (*c_header).c_addr_multicast_ttl,
                tmp,
                '\r' as i32,
                &mut tmp_next,
            );
            if i != 0 as libc::c_int {
                i = __osip_set_next_token(
                    &mut (*c_header).c_addr_multicast_ttl,
                    tmp,
                    '\n' as i32,
                    &mut tmp_next,
                );
                if i != 0 as libc::c_int {
                    sdp_connection_free(c_header);
                    return -(1 as libc::c_int);
                }
            }
        }
    } else {
        i = __osip_set_next_token(
            &mut (*c_header).c_addr,
            tmp,
            '\r' as i32,
            &mut tmp_next,
        );
        if i != 0 as libc::c_int {
            i = __osip_set_next_token(
                &mut (*c_header).c_addr,
                tmp,
                '\n' as i32,
                &mut tmp_next,
            );
            if i != 0 as libc::c_int {
                sdp_connection_free(c_header);
                return -(1 as libc::c_int);
            }
        }
    }
    i = osip_list_size(&mut (*sdp).m_medias);
    if i == 0 as libc::c_int {
        (*sdp).c_connection = c_header;
    } else {
        let mut last_sdp_media: *mut sdp_media_t = osip_list_get(
            &mut (*sdp).m_medias,
            i - 1 as libc::c_int,
        ) as *mut sdp_media_t;
        osip_list_add(
            &mut (*last_sdp_media).c_connections,
            c_header as *mut libc::c_void,
            -(1 as libc::c_int),
        );
    }
    if *crlf.offset(1 as libc::c_int as isize) as libc::c_int == '\n' as i32 {
        *next = crlf.offset(2 as libc::c_int as isize);
    } else {
        *next = crlf.offset(1 as libc::c_int as isize);
    }
    return 1 as libc::c_int;
}
unsafe extern "C" fn sdp_message_parse_b(
    mut sdp: *mut sdp_message_t,
    mut buf: *mut libc::c_char,
    mut next: *mut *mut libc::c_char,
) -> libc::c_int {
    let mut equal: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut crlf: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut tmp: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut tmp_next: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut i: libc::c_int = 0;
    let mut b_header: *mut sdp_bandwidth_t = 0 as *mut sdp_bandwidth_t;
    *next = buf;
    equal = buf;
    while *equal as libc::c_int != '=' as i32 && *equal as libc::c_int != '\0' as i32 {
        equal = equal.offset(1);
        equal;
    }
    if *equal as libc::c_int == '\0' as i32 {
        return -(1 as libc::c_int);
    }
    if *equal.offset(-(1 as libc::c_int) as isize) as libc::c_int != 'b' as i32 {
        return 0 as libc::c_int;
    }
    crlf = equal.offset(1 as libc::c_int as isize);
    while *crlf as libc::c_int != '\r' as i32 && *crlf as libc::c_int != '\n' as i32
        && *crlf as libc::c_int != '\0' as i32
    {
        crlf = crlf.offset(1);
        crlf;
    }
    if *crlf as libc::c_int == '\0' as i32 {
        return -(1 as libc::c_int);
    }
    if crlf == equal.offset(1 as libc::c_int as isize) {
        return -(1 as libc::c_int);
    }
    tmp = equal.offset(1 as libc::c_int as isize);
    i = sdp_bandwidth_init(&mut b_header);
    if i != 0 as libc::c_int {
        return -(1 as libc::c_int);
    }
    i = __osip_set_next_token(&mut (*b_header).b_bwtype, tmp, ':' as i32, &mut tmp_next);
    if i != 0 as libc::c_int {
        sdp_bandwidth_free(b_header);
        return -(1 as libc::c_int);
    }
    tmp = tmp_next;
    i = __osip_set_next_token(
        &mut (*b_header).b_bandwidth,
        tmp,
        '\r' as i32,
        &mut tmp_next,
    );
    if i != 0 as libc::c_int {
        i = __osip_set_next_token(
            &mut (*b_header).b_bandwidth,
            tmp,
            '\n' as i32,
            &mut tmp_next,
        );
        if i != 0 as libc::c_int {
            sdp_bandwidth_free(b_header);
            return -(1 as libc::c_int);
        }
    }
    i = osip_list_size(&mut (*sdp).m_medias);
    if i == 0 as libc::c_int {
        osip_list_add(
            &mut (*sdp).b_bandwidths,
            b_header as *mut libc::c_void,
            -(1 as libc::c_int),
        );
    } else {
        let mut last_sdp_media: *mut sdp_media_t = osip_list_get(
            &mut (*sdp).m_medias,
            i - 1 as libc::c_int,
        ) as *mut sdp_media_t;
        osip_list_add(
            &mut (*last_sdp_media).b_bandwidths,
            b_header as *mut libc::c_void,
            -(1 as libc::c_int),
        );
    }
    if *crlf.offset(1 as libc::c_int as isize) as libc::c_int == '\n' as i32 {
        *next = crlf.offset(2 as libc::c_int as isize);
    } else {
        *next = crlf.offset(1 as libc::c_int as isize);
    }
    return 1 as libc::c_int;
}
unsafe extern "C" fn sdp_message_parse_t(
    mut sdp: *mut sdp_message_t,
    mut buf: *mut libc::c_char,
    mut next: *mut *mut libc::c_char,
) -> libc::c_int {
    let mut equal: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut crlf: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut tmp: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut tmp_next: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut i: libc::c_int = 0;
    let mut t_header: *mut sdp_time_descr_t = 0 as *mut sdp_time_descr_t;
    *next = buf;
    equal = buf;
    while *equal as libc::c_int != '=' as i32 && *equal as libc::c_int != '\0' as i32 {
        equal = equal.offset(1);
        equal;
    }
    if *equal as libc::c_int == '\0' as i32 {
        return -(1 as libc::c_int);
    }
    if *equal.offset(-(1 as libc::c_int) as isize) as libc::c_int != 't' as i32 {
        return 0 as libc::c_int;
    }
    crlf = equal.offset(1 as libc::c_int as isize);
    while *crlf as libc::c_int != '\r' as i32 && *crlf as libc::c_int != '\n' as i32
        && *crlf as libc::c_int != '\0' as i32
    {
        crlf = crlf.offset(1);
        crlf;
    }
    if *crlf as libc::c_int == '\0' as i32 {
        return -(1 as libc::c_int);
    }
    if crlf == equal.offset(1 as libc::c_int as isize) {
        return -(1 as libc::c_int);
    }
    tmp = equal.offset(1 as libc::c_int as isize);
    i = sdp_time_descr_init(&mut t_header);
    if i != 0 as libc::c_int {
        return -(1 as libc::c_int);
    }
    i = __osip_set_next_token(
        &mut (*t_header).t_start_time,
        tmp,
        ' ' as i32,
        &mut tmp_next,
    );
    if i != 0 as libc::c_int {
        sdp_time_descr_free(t_header);
        return -(1 as libc::c_int);
    }
    tmp = tmp_next;
    i = __osip_set_next_token(
        &mut (*t_header).t_stop_time,
        tmp,
        '\r' as i32,
        &mut tmp_next,
    );
    if i != 0 as libc::c_int {
        i = __osip_set_next_token(
            &mut (*t_header).t_stop_time,
            tmp,
            '\n' as i32,
            &mut tmp_next,
        );
        if i != 0 as libc::c_int {
            sdp_time_descr_free(t_header);
            return -(1 as libc::c_int);
        }
    }
    osip_list_add(
        &mut (*sdp).t_descrs,
        t_header as *mut libc::c_void,
        -(1 as libc::c_int),
    );
    if *crlf.offset(1 as libc::c_int as isize) as libc::c_int == '\n' as i32 {
        *next = crlf.offset(2 as libc::c_int as isize);
    } else {
        *next = crlf.offset(1 as libc::c_int as isize);
    }
    return 1 as libc::c_int;
}
unsafe extern "C" fn sdp_message_parse_r(
    mut sdp: *mut sdp_message_t,
    mut buf: *mut libc::c_char,
    mut next: *mut *mut libc::c_char,
) -> libc::c_int {
    let mut equal: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut crlf: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut index: libc::c_int = 0;
    let mut r_header: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut t_descr: *mut sdp_time_descr_t = 0 as *mut sdp_time_descr_t;
    *next = buf;
    equal = buf;
    while *equal as libc::c_int != '=' as i32 && *equal as libc::c_int != '\0' as i32 {
        equal = equal.offset(1);
        equal;
    }
    if *equal as libc::c_int == '\0' as i32 {
        return -(1 as libc::c_int);
    }
    if *equal.offset(-(1 as libc::c_int) as isize) as libc::c_int != 'r' as i32 {
        return 0 as libc::c_int;
    }
    index = osip_list_size(&mut (*sdp).t_descrs);
    if index == 0 as libc::c_int {
        return -(1 as libc::c_int);
    }
    crlf = equal.offset(1 as libc::c_int as isize);
    while *crlf as libc::c_int != '\r' as i32 && *crlf as libc::c_int != '\n' as i32
        && *crlf as libc::c_int != '\0' as i32
    {
        crlf = crlf.offset(1);
        crlf;
    }
    if *crlf as libc::c_int == '\0' as i32 {
        return -(1 as libc::c_int);
    }
    if crlf == equal.offset(1 as libc::c_int as isize) {
        return -(1 as libc::c_int);
    }
    r_header = (if osip_malloc_func.is_some() {
        osip_malloc_func
            .unwrap()(
            (crlf.offset_from(equal.offset(1 as libc::c_int as isize)) as libc::c_long
                + 1 as libc::c_int as libc::c_long) as size_t,
        )
    } else {
        malloc(
            (crlf.offset_from(equal.offset(1 as libc::c_int as isize)) as libc::c_long
                + 1 as libc::c_int as libc::c_long) as libc::c_ulong,
        )
    }) as *mut libc::c_char;
    if r_header.is_null() {
        return -(4 as libc::c_int);
    }
    osip_strncpy(
        r_header,
        equal.offset(1 as libc::c_int as isize),
        crlf.offset_from(equal.offset(1 as libc::c_int as isize)) as libc::c_long
            as size_t,
    );
    t_descr = osip_list_get(&mut (*sdp).t_descrs, index - 1 as libc::c_int)
        as *mut sdp_time_descr_t;
    osip_list_add(
        &mut (*t_descr).r_repeats,
        r_header as *mut libc::c_void,
        -(1 as libc::c_int),
    );
    if *crlf.offset(1 as libc::c_int as isize) as libc::c_int == '\n' as i32 {
        *next = crlf.offset(2 as libc::c_int as isize);
    } else {
        *next = crlf.offset(1 as libc::c_int as isize);
    }
    return 1 as libc::c_int;
}
unsafe extern "C" fn sdp_message_parse_z(
    mut sdp: *mut sdp_message_t,
    mut buf: *mut libc::c_char,
    mut next: *mut *mut libc::c_char,
) -> libc::c_int {
    let mut equal: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut crlf: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut z_header: *mut libc::c_char = 0 as *mut libc::c_char;
    *next = buf;
    equal = buf;
    while *equal as libc::c_int != '=' as i32 && *equal as libc::c_int != '\0' as i32 {
        equal = equal.offset(1);
        equal;
    }
    if *equal as libc::c_int == '\0' as i32 {
        return -(1 as libc::c_int);
    }
    if *equal.offset(-(1 as libc::c_int) as isize) as libc::c_int != 'z' as i32 {
        return 0 as libc::c_int;
    }
    crlf = equal.offset(1 as libc::c_int as isize);
    while *crlf as libc::c_int != '\r' as i32 && *crlf as libc::c_int != '\n' as i32
        && *crlf as libc::c_int != '\0' as i32
    {
        crlf = crlf.offset(1);
        crlf;
    }
    if *crlf as libc::c_int == '\0' as i32 {
        return -(1 as libc::c_int);
    }
    if crlf == equal.offset(1 as libc::c_int as isize) {
        return -(1 as libc::c_int);
    }
    z_header = (if osip_malloc_func.is_some() {
        osip_malloc_func
            .unwrap()(
            (crlf.offset_from(equal.offset(1 as libc::c_int as isize)) as libc::c_long
                + 1 as libc::c_int as libc::c_long) as size_t,
        )
    } else {
        malloc(
            (crlf.offset_from(equal.offset(1 as libc::c_int as isize)) as libc::c_long
                + 1 as libc::c_int as libc::c_long) as libc::c_ulong,
        )
    }) as *mut libc::c_char;
    if z_header.is_null() {
        return -(4 as libc::c_int);
    }
    osip_strncpy(
        z_header,
        equal.offset(1 as libc::c_int as isize),
        crlf.offset_from(equal.offset(1 as libc::c_int as isize)) as libc::c_long
            as size_t,
    );
    (*sdp).z_adjustments = z_header;
    if *crlf.offset(1 as libc::c_int as isize) as libc::c_int == '\n' as i32 {
        *next = crlf.offset(2 as libc::c_int as isize);
    } else {
        *next = crlf.offset(1 as libc::c_int as isize);
    }
    return 1 as libc::c_int;
}
unsafe extern "C" fn sdp_message_parse_k(
    mut sdp: *mut sdp_message_t,
    mut buf: *mut libc::c_char,
    mut next: *mut *mut libc::c_char,
) -> libc::c_int {
    let mut equal: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut crlf: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut i: libc::c_int = 0;
    let mut colon: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut k_header: *mut sdp_key_t = 0 as *mut sdp_key_t;
    let mut tmp: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut tmp_next: *mut libc::c_char = 0 as *mut libc::c_char;
    *next = buf;
    equal = buf;
    while *equal as libc::c_int != '=' as i32 && *equal as libc::c_int != '\0' as i32 {
        equal = equal.offset(1);
        equal;
    }
    if *equal as libc::c_int == '\0' as i32 {
        return -(1 as libc::c_int);
    }
    if *equal.offset(-(1 as libc::c_int) as isize) as libc::c_int != 'k' as i32 {
        return 0 as libc::c_int;
    }
    crlf = equal.offset(1 as libc::c_int as isize);
    while *crlf as libc::c_int != '\r' as i32 && *crlf as libc::c_int != '\n' as i32
        && *crlf as libc::c_int != '\0' as i32
    {
        crlf = crlf.offset(1);
        crlf;
    }
    if *crlf as libc::c_int == '\0' as i32 {
        return -(1 as libc::c_int);
    }
    if crlf == equal.offset(1 as libc::c_int as isize) {
        return -(1 as libc::c_int);
    }
    tmp = equal.offset(1 as libc::c_int as isize);
    i = sdp_key_init(&mut k_header);
    if i != 0 as libc::c_int {
        return -(1 as libc::c_int);
    }
    colon = strchr(equal.offset(1 as libc::c_int as isize), ':' as i32);
    if !colon.is_null() && colon < crlf {
        i = __osip_set_next_token(
            &mut (*k_header).k_keytype,
            tmp,
            ':' as i32,
            &mut tmp_next,
        );
        if i != 0 as libc::c_int {
            sdp_key_free(k_header);
            return -(1 as libc::c_int);
        }
        tmp = tmp_next;
        i = __osip_set_next_token(
            &mut (*k_header).k_keydata,
            tmp,
            '\r' as i32,
            &mut tmp_next,
        );
        if i != 0 as libc::c_int {
            i = __osip_set_next_token(
                &mut (*k_header).k_keydata,
                tmp,
                '\n' as i32,
                &mut tmp_next,
            );
            if i != 0 as libc::c_int {
                sdp_key_free(k_header);
                return -(1 as libc::c_int);
            }
        }
    } else {
        i = __osip_set_next_token(
            &mut (*k_header).k_keytype,
            tmp,
            '\r' as i32,
            &mut tmp_next,
        );
        if i != 0 as libc::c_int {
            i = __osip_set_next_token(
                &mut (*k_header).k_keytype,
                tmp,
                '\n' as i32,
                &mut tmp_next,
            );
            if i != 0 as libc::c_int {
                sdp_key_free(k_header);
                return -(1 as libc::c_int);
            }
        }
    }
    i = osip_list_size(&mut (*sdp).m_medias);
    if i == 0 as libc::c_int {
        (*sdp).k_key = k_header;
    } else {
        let mut last_sdp_media: *mut sdp_media_t = osip_list_get(
            &mut (*sdp).m_medias,
            i - 1 as libc::c_int,
        ) as *mut sdp_media_t;
        if !((*last_sdp_media).k_key).is_null() {
            sdp_key_free(k_header);
            return -(1 as libc::c_int);
        }
        (*last_sdp_media).k_key = k_header;
    }
    if *crlf.offset(1 as libc::c_int as isize) as libc::c_int == '\n' as i32 {
        *next = crlf.offset(2 as libc::c_int as isize);
    } else {
        *next = crlf.offset(1 as libc::c_int as isize);
    }
    return 1 as libc::c_int;
}
unsafe extern "C" fn sdp_message_parse_a(
    mut sdp: *mut sdp_message_t,
    mut buf: *mut libc::c_char,
    mut next: *mut *mut libc::c_char,
) -> libc::c_int {
    let mut equal: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut crlf: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut tmp: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut tmp_next: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut i: libc::c_int = 0;
    let mut a_attribute: *mut sdp_attribute_t = 0 as *mut sdp_attribute_t;
    let mut colon: *mut libc::c_char = 0 as *mut libc::c_char;
    *next = buf;
    equal = buf;
    while *equal as libc::c_int != '=' as i32 && *equal as libc::c_int != '\0' as i32 {
        equal = equal.offset(1);
        equal;
    }
    if *equal as libc::c_int == '\0' as i32 {
        return -(1 as libc::c_int);
    }
    if *equal.offset(-(1 as libc::c_int) as isize) as libc::c_int != 'a' as i32 {
        return 0 as libc::c_int;
    }
    crlf = equal.offset(1 as libc::c_int as isize);
    while *crlf as libc::c_int != '\r' as i32 && *crlf as libc::c_int != '\n' as i32
        && *crlf as libc::c_int != '\0' as i32
    {
        crlf = crlf.offset(1);
        crlf;
    }
    if *crlf as libc::c_int == '\0' as i32 {
        return -(1 as libc::c_int);
    }
    if crlf == equal.offset(1 as libc::c_int as isize) {
        return -(1 as libc::c_int);
    }
    tmp = equal.offset(1 as libc::c_int as isize);
    i = sdp_attribute_init(&mut a_attribute);
    if i != 0 as libc::c_int {
        return -(1 as libc::c_int);
    }
    colon = strchr(equal.offset(1 as libc::c_int as isize), ':' as i32);
    if !colon.is_null() && colon < crlf {
        i = __osip_set_next_token(
            &mut (*a_attribute).a_att_field,
            tmp,
            ':' as i32,
            &mut tmp_next,
        );
        if i != 0 as libc::c_int {
            sdp_attribute_free(a_attribute);
            return -(1 as libc::c_int);
        }
        tmp = tmp_next;
        i = __osip_set_next_token(
            &mut (*a_attribute).a_att_value,
            tmp,
            '\r' as i32,
            &mut tmp_next,
        );
        if i != 0 as libc::c_int {
            i = __osip_set_next_token(
                &mut (*a_attribute).a_att_value,
                tmp,
                '\n' as i32,
                &mut tmp_next,
            );
            if i != 0 as libc::c_int {
                sdp_attribute_free(a_attribute);
                return -(1 as libc::c_int);
            }
        }
    } else {
        i = __osip_set_next_token(
            &mut (*a_attribute).a_att_field,
            tmp,
            '\r' as i32,
            &mut tmp_next,
        );
        if i != 0 as libc::c_int {
            i = __osip_set_next_token(
                &mut (*a_attribute).a_att_field,
                tmp,
                '\n' as i32,
                &mut tmp_next,
            );
            if i != 0 as libc::c_int {
                sdp_attribute_free(a_attribute);
                return -(1 as libc::c_int);
            }
        }
    }
    i = osip_list_size(&mut (*sdp).m_medias);
    if i == 0 as libc::c_int {
        osip_list_add(
            &mut (*sdp).a_attributes,
            a_attribute as *mut libc::c_void,
            -(1 as libc::c_int),
        );
    } else {
        let mut last_sdp_media: *mut sdp_media_t = osip_list_get(
            &mut (*sdp).m_medias,
            i - 1 as libc::c_int,
        ) as *mut sdp_media_t;
        osip_list_add(
            &mut (*last_sdp_media).a_attributes,
            a_attribute as *mut libc::c_void,
            -(1 as libc::c_int),
        );
    }
    if *crlf.offset(1 as libc::c_int as isize) as libc::c_int == '\n' as i32 {
        *next = crlf.offset(2 as libc::c_int as isize);
    } else {
        *next = crlf.offset(1 as libc::c_int as isize);
    }
    return 1 as libc::c_int;
}
unsafe extern "C" fn sdp_message_parse_m(
    mut sdp: *mut sdp_message_t,
    mut buf: *mut libc::c_char,
    mut next: *mut *mut libc::c_char,
) -> libc::c_int {
    let mut equal: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut crlf: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut tmp: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut tmp_next: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut i: libc::c_int = 0;
    let mut m_header: *mut sdp_media_t = 0 as *mut sdp_media_t;
    let mut slash: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut space: *mut libc::c_char = 0 as *mut libc::c_char;
    *next = buf;
    equal = buf;
    while *equal as libc::c_int != '=' as i32 && *equal as libc::c_int != '\0' as i32 {
        equal = equal.offset(1);
        equal;
    }
    if *equal as libc::c_int == '\0' as i32 {
        return -(1 as libc::c_int);
    }
    if *equal.offset(-(1 as libc::c_int) as isize) as libc::c_int != 'm' as i32 {
        return -(1 as libc::c_int);
    }
    crlf = equal.offset(1 as libc::c_int as isize);
    while *crlf as libc::c_int != '\r' as i32 && *crlf as libc::c_int != '\n' as i32
        && *crlf as libc::c_int != '\0' as i32
    {
        crlf = crlf.offset(1);
        crlf;
    }
    if *crlf as libc::c_int == '\0' as i32 {
        return -(1 as libc::c_int);
    }
    if crlf == equal.offset(1 as libc::c_int as isize) {
        return -(1 as libc::c_int);
    }
    tmp = equal.offset(1 as libc::c_int as isize);
    i = sdp_media_init(&mut m_header);
    if i != 0 as libc::c_int {
        return -(1 as libc::c_int);
    }
    i = __osip_set_next_token(&mut (*m_header).m_media, tmp, ' ' as i32, &mut tmp_next);
    if i != 0 as libc::c_int {
        sdp_media_free(m_header);
        return -(1 as libc::c_int);
    }
    tmp = tmp_next;
    slash = strchr(tmp, '/' as i32);
    space = strchr(tmp, ' ' as i32);
    if space.is_null() {
        sdp_media_free(m_header);
        return -(1 as libc::c_int);
    }
    if !slash.is_null() && slash < space {
        i = __osip_set_next_token(
            &mut (*m_header).m_port,
            tmp,
            '/' as i32,
            &mut tmp_next,
        );
        if i != 0 as libc::c_int {
            sdp_media_free(m_header);
            return -(1 as libc::c_int);
        }
        tmp = tmp_next;
        i = __osip_set_next_token(
            &mut (*m_header).m_number_of_port,
            tmp,
            ' ' as i32,
            &mut tmp_next,
        );
        if i != 0 as libc::c_int {
            sdp_media_free(m_header);
            return -(1 as libc::c_int);
        }
        tmp = tmp_next;
    } else {
        i = __osip_set_next_token(
            &mut (*m_header).m_port,
            tmp,
            ' ' as i32,
            &mut tmp_next,
        );
        if i != 0 as libc::c_int {
            sdp_media_free(m_header);
            return -(1 as libc::c_int);
        }
        tmp = tmp_next;
    }
    i = __osip_set_next_token(&mut (*m_header).m_proto, tmp, ' ' as i32, &mut tmp_next);
    if i != 0 as libc::c_int {
        i = __osip_set_next_token(
            &mut (*m_header).m_proto,
            tmp,
            '\r' as i32,
            &mut tmp_next,
        );
        if i != 0 as libc::c_int {
            i = __osip_set_next_token(
                &mut (*m_header).m_proto,
                tmp,
                '\n' as i32,
                &mut tmp_next,
            );
            if i != 0 as libc::c_int {
                sdp_media_free(m_header);
                return -(1 as libc::c_int);
            }
        }
    }
    tmp = tmp_next;
    let mut str: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut more_space_before_crlf: libc::c_int = 0;
    space = strchr(tmp.offset(1 as libc::c_int as isize), ' ' as i32);
    if space.is_null() {
        more_space_before_crlf = 1 as libc::c_int;
    } else if !space.is_null() && space > crlf {
        more_space_before_crlf = 1 as libc::c_int;
    } else {
        more_space_before_crlf = 0 as libc::c_int;
    }
    while more_space_before_crlf == 0 as libc::c_int {
        i = __osip_set_next_token(&mut str, tmp, ' ' as i32, &mut tmp_next);
        if i != 0 as libc::c_int {
            sdp_media_free(m_header);
            return -(1 as libc::c_int);
        }
        tmp = tmp_next;
        osip_list_add(
            &mut (*m_header).m_payloads,
            str as *mut libc::c_void,
            -(1 as libc::c_int),
        );
        space = strchr(tmp.offset(1 as libc::c_int as isize), ' ' as i32);
        if space.is_null() {
            more_space_before_crlf = 1 as libc::c_int;
        } else if !space.is_null() && space > crlf {
            more_space_before_crlf = 1 as libc::c_int;
        } else {
            more_space_before_crlf = 0 as libc::c_int;
        }
    }
    if tmp_next < crlf {
        i = __osip_set_next_token(&mut str, tmp, '\r' as i32, &mut tmp_next);
        if i != 0 as libc::c_int {
            i = __osip_set_next_token(&mut str, tmp, '\n' as i32, &mut tmp_next);
            if i != 0 as libc::c_int {
                sdp_media_free(m_header);
                return -(1 as libc::c_int);
            }
        }
        osip_list_add(
            &mut (*m_header).m_payloads,
            str as *mut libc::c_void,
            -(1 as libc::c_int),
        );
    }
    osip_list_add(
        &mut (*sdp).m_medias,
        m_header as *mut libc::c_void,
        -(1 as libc::c_int),
    );
    if *crlf.offset(1 as libc::c_int as isize) as libc::c_int == '\n' as i32 {
        *next = crlf.offset(2 as libc::c_int as isize);
    } else {
        *next = crlf.offset(1 as libc::c_int as isize);
    }
    return 1 as libc::c_int;
}
pub unsafe extern "C" fn sdp_message_parse(
    mut sdp: *mut sdp_message_t,
    mut buf: *const libc::c_char,
) -> libc::c_int {
    let mut next_buf: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut ptr: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut i: libc::c_int = 0;
    ptr = buf as *mut libc::c_char;
    i = sdp_message_parse_v(sdp, ptr, &mut next_buf);
    if i == -(1 as libc::c_int) {
        return -(1 as libc::c_int)
    } else if 0 as libc::c_int == i {
        return -(1 as libc::c_int)
    }
    ptr = next_buf;
    i = sdp_message_parse_s(sdp, ptr, &mut next_buf);
    if i == -(1 as libc::c_int) {
        return -(1 as libc::c_int);
    }
    ptr = next_buf;
    i = sdp_message_parse_o(sdp, ptr, &mut next_buf);
    if i == -(1 as libc::c_int) {
        return -(1 as libc::c_int)
    } else if 0 as libc::c_int == i {
        return -(1 as libc::c_int)
    }
    ptr = next_buf;
    if ((*sdp).s_name).is_null() {
        i = sdp_message_parse_s(sdp, ptr, &mut next_buf);
        if i == -(1 as libc::c_int) {
            return -(1 as libc::c_int)
        } else if 0 as libc::c_int == i {
            osip_trace(
                b"sdp_message.c\0" as *const u8 as *const libc::c_char,
                1654 as libc::c_int,
                TRACE_LEVEL7,
                0 as *mut FILE,
                b"The \"s\" parameter is mandatory, but this packet does not contain any! - anyway, we don't mind about it.\n\0"
                    as *const u8 as *const libc::c_char,
            );
        }
        ptr = next_buf;
    }
    i = sdp_message_parse_i(sdp, ptr, &mut next_buf);
    if i == -(1 as libc::c_int) {
        return -(1 as libc::c_int);
    }
    ptr = next_buf;
    i = sdp_message_parse_u(sdp, ptr, &mut next_buf);
    if i == -(1 as libc::c_int) {
        return -(1 as libc::c_int);
    }
    ptr = next_buf;
    i = 1 as libc::c_int;
    while i == 1 as libc::c_int {
        i = sdp_message_parse_e(sdp, ptr, &mut next_buf);
        if i == -(1 as libc::c_int) {
            return -(1 as libc::c_int);
        }
        ptr = next_buf;
    }
    i = 1 as libc::c_int;
    while i == 1 as libc::c_int {
        i = sdp_message_parse_p(sdp, ptr, &mut next_buf);
        if i == -(1 as libc::c_int) {
            return -(1 as libc::c_int);
        }
        ptr = next_buf;
    }
    if osip_list_size(&mut (*sdp).e_emails) == 0 as libc::c_int
        && osip_list_size(&mut (*sdp).p_phones) == 0 as libc::c_int
    {
        osip_trace(
            b"sdp_message.c\0" as *const u8 as *const libc::c_char,
            1698 as libc::c_int,
            TRACE_LEVEL7,
            0 as *mut FILE,
            b"The rfc2327 says there should be at least an email or a phone header!- anyway, we don't mind about it.\n\0"
                as *const u8 as *const libc::c_char,
        );
    }
    i = sdp_message_parse_c(sdp, ptr, &mut next_buf);
    if i == -(1 as libc::c_int) {
        return -(1 as libc::c_int);
    }
    ptr = next_buf;
    i = 1 as libc::c_int;
    while i == 1 as libc::c_int {
        i = sdp_message_parse_b(sdp, ptr, &mut next_buf);
        if i == -(1 as libc::c_int) {
            return -(1 as libc::c_int);
        }
        ptr = next_buf;
    }
    i = sdp_message_parse_t(sdp, ptr, &mut next_buf);
    if i == -(1 as libc::c_int) {
        return -(1 as libc::c_int)
    } else if i == 0 as libc::c_int {
        return -(1 as libc::c_int)
    }
    ptr = next_buf;
    if *ptr as libc::c_int == '\0' as i32 || *ptr as libc::c_int == '\r' as i32
        || *ptr as libc::c_int == '\n' as i32
    {
        return 0 as libc::c_int;
    }
    i = 1 as libc::c_int;
    while i == 1 as libc::c_int {
        i = sdp_message_parse_r(sdp, ptr, &mut next_buf);
        if i == -(1 as libc::c_int) {
            return -(1 as libc::c_int);
        }
        ptr = next_buf;
        if *ptr as libc::c_int == '\0' as i32 || *ptr as libc::c_int == '\r' as i32
            || *ptr as libc::c_int == '\n' as i32
        {
            return 0 as libc::c_int;
        }
    }
    let mut more_t_header: libc::c_int = 1 as libc::c_int;
    i = sdp_message_parse_t(sdp, ptr, &mut next_buf);
    if i == -(1 as libc::c_int) {
        return -(1 as libc::c_int);
    }
    ptr = next_buf;
    if *ptr as libc::c_int == '\0' as i32 || *ptr as libc::c_int == '\r' as i32
        || *ptr as libc::c_int == '\n' as i32
    {
        return 0 as libc::c_int;
    }
    while more_t_header == 1 as libc::c_int {
        i = 1 as libc::c_int;
        while i == 1 as libc::c_int {
            i = sdp_message_parse_r(sdp, ptr, &mut next_buf);
            if i == -(1 as libc::c_int) {
                return -(1 as libc::c_int);
            }
            ptr = next_buf;
            if *ptr as libc::c_int == '\0' as i32 || *ptr as libc::c_int == '\r' as i32
                || *ptr as libc::c_int == '\n' as i32
            {
                return 0 as libc::c_int;
            }
        }
        i = sdp_message_parse_t(sdp, ptr, &mut next_buf);
        if i == -(1 as libc::c_int) {
            return -(1 as libc::c_int)
        } else if i == 0 as libc::c_int {
            more_t_header = 0 as libc::c_int;
        } else {
            more_t_header = 1 as libc::c_int;
        }
        ptr = next_buf;
        if *ptr as libc::c_int == '\0' as i32 || *ptr as libc::c_int == '\r' as i32
            || *ptr as libc::c_int == '\n' as i32
        {
            return 0 as libc::c_int;
        }
    }
    i = sdp_message_parse_z(sdp, ptr, &mut next_buf);
    if i == -(1 as libc::c_int) {
        return -(1 as libc::c_int);
    }
    ptr = next_buf;
    if *ptr as libc::c_int == '\0' as i32 || *ptr as libc::c_int == '\r' as i32
        || *ptr as libc::c_int == '\n' as i32
    {
        return 0 as libc::c_int;
    }
    i = sdp_message_parse_k(sdp, ptr, &mut next_buf);
    if i == -(1 as libc::c_int) {
        return -(1 as libc::c_int);
    }
    ptr = next_buf;
    if *ptr as libc::c_int == '\0' as i32 || *ptr as libc::c_int == '\r' as i32
        || *ptr as libc::c_int == '\n' as i32
    {
        return 0 as libc::c_int;
    }
    i = 1 as libc::c_int;
    while i == 1 as libc::c_int {
        i = sdp_message_parse_a(sdp, ptr, &mut next_buf);
        if i == -(1 as libc::c_int) {
            return -(1 as libc::c_int);
        }
        ptr = next_buf;
        if *ptr as libc::c_int == '\0' as i32 || *ptr as libc::c_int == '\r' as i32
            || *ptr as libc::c_int == '\n' as i32
        {
            return 0 as libc::c_int;
        }
    }
    let mut more_m_header: libc::c_int = 1 as libc::c_int;
    while more_m_header == 1 as libc::c_int {
        more_m_header = sdp_message_parse_m(sdp, ptr, &mut next_buf);
        if more_m_header == -(1 as libc::c_int) {
            return -(1 as libc::c_int);
        }
        ptr = next_buf;
        if *ptr as libc::c_int == '\0' as i32 || *ptr as libc::c_int == '\r' as i32
            || *ptr as libc::c_int == '\n' as i32
        {
            return 0 as libc::c_int;
        }
        i = sdp_message_parse_i(sdp, ptr, &mut next_buf);
        if i == -(1 as libc::c_int) {
            return -(1 as libc::c_int);
        }
        ptr = next_buf;
        if *ptr as libc::c_int == '\0' as i32 || *ptr as libc::c_int == '\r' as i32
            || *ptr as libc::c_int == '\n' as i32
        {
            return 0 as libc::c_int;
        }
        i = 1 as libc::c_int;
        while i == 1 as libc::c_int {
            i = sdp_message_parse_c(sdp, ptr, &mut next_buf);
            if i == -(1 as libc::c_int) {
                return -(1 as libc::c_int);
            }
            ptr = next_buf;
            if *ptr as libc::c_int == '\0' as i32 || *ptr as libc::c_int == '\r' as i32
                || *ptr as libc::c_int == '\n' as i32
            {
                return 0 as libc::c_int;
            }
        }
        i = 1 as libc::c_int;
        while i == 1 as libc::c_int {
            i = sdp_message_parse_b(sdp, ptr, &mut next_buf);
            if i == -(1 as libc::c_int) {
                return -(1 as libc::c_int);
            }
            ptr = next_buf;
            if *ptr as libc::c_int == '\0' as i32 || *ptr as libc::c_int == '\r' as i32
                || *ptr as libc::c_int == '\n' as i32
            {
                return 0 as libc::c_int;
            }
        }
        i = sdp_message_parse_k(sdp, ptr, &mut next_buf);
        if i == -(1 as libc::c_int) {
            return -(1 as libc::c_int);
        }
        ptr = next_buf;
        if *ptr as libc::c_int == '\0' as i32 || *ptr as libc::c_int == '\r' as i32
            || *ptr as libc::c_int == '\n' as i32
        {
            return 0 as libc::c_int;
        }
        i = 1 as libc::c_int;
        while i == 1 as libc::c_int {
            i = sdp_message_parse_a(sdp, ptr, &mut next_buf);
            if i == -(1 as libc::c_int) {
                return -(1 as libc::c_int);
            }
            ptr = next_buf;
            if *ptr as libc::c_int == '\0' as i32 || *ptr as libc::c_int == '\r' as i32
                || *ptr as libc::c_int == '\n' as i32
            {
                return 0 as libc::c_int;
            }
        }
    }
    return 0 as libc::c_int;
}
unsafe extern "C" fn sdp_append_connection(
    mut string: *mut *mut libc::c_char,
    mut size: *mut libc::c_int,
    mut tmp: *mut libc::c_char,
    mut conn: *mut sdp_connection_t,
    mut next_tmp: *mut *mut libc::c_char,
) -> libc::c_int {
    if ((*conn).c_nettype).is_null() {
        return -(1 as libc::c_int);
    }
    if ((*conn).c_addrtype).is_null() {
        return -(1 as libc::c_int);
    }
    if ((*conn).c_addr).is_null() {
        return -(1 as libc::c_int);
    }
    tmp = __osip_sdp_append_string(
        string,
        size,
        tmp,
        b"c=\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    );
    tmp = __osip_sdp_append_string(string, size, tmp, (*conn).c_nettype);
    tmp = __osip_sdp_append_string(
        string,
        size,
        tmp,
        b" \0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    );
    tmp = __osip_sdp_append_string(string, size, tmp, (*conn).c_addrtype);
    tmp = __osip_sdp_append_string(
        string,
        size,
        tmp,
        b" \0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    );
    tmp = __osip_sdp_append_string(string, size, tmp, (*conn).c_addr);
    if !((*conn).c_addr_multicast_ttl).is_null() {
        tmp = __osip_sdp_append_string(
            string,
            size,
            tmp,
            b"/\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        );
        tmp = __osip_sdp_append_string(string, size, tmp, (*conn).c_addr_multicast_ttl);
    }
    if !((*conn).c_addr_multicast_int).is_null() {
        tmp = __osip_sdp_append_string(
            string,
            size,
            tmp,
            b"/\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        );
        tmp = __osip_sdp_append_string(string, size, tmp, (*conn).c_addr_multicast_int);
    }
    tmp = __osip_sdp_append_string(
        string,
        size,
        tmp,
        b"\r\n\0\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    );
    *next_tmp = tmp;
    return 0 as libc::c_int;
}
unsafe extern "C" fn sdp_append_bandwidth(
    mut string: *mut *mut libc::c_char,
    mut size: *mut libc::c_int,
    mut tmp: *mut libc::c_char,
    mut bandwidth: *mut sdp_bandwidth_t,
    mut next_tmp: *mut *mut libc::c_char,
) -> libc::c_int {
    if ((*bandwidth).b_bwtype).is_null() {
        return -(1 as libc::c_int);
    }
    if ((*bandwidth).b_bandwidth).is_null() {
        return -(1 as libc::c_int);
    }
    tmp = __osip_sdp_append_string(
        string,
        size,
        tmp,
        b"b=\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    );
    tmp = __osip_sdp_append_string(string, size, tmp, (*bandwidth).b_bwtype);
    tmp = __osip_sdp_append_string(
        string,
        size,
        tmp,
        b":\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    );
    tmp = __osip_sdp_append_string(string, size, tmp, (*bandwidth).b_bandwidth);
    tmp = __osip_sdp_append_string(
        string,
        size,
        tmp,
        b"\r\n\0\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    );
    *next_tmp = tmp;
    return 0 as libc::c_int;
}
unsafe extern "C" fn sdp_append_time_descr(
    mut string: *mut *mut libc::c_char,
    mut size: *mut libc::c_int,
    mut tmp: *mut libc::c_char,
    mut time_descr: *mut sdp_time_descr_t,
    mut next_tmp: *mut *mut libc::c_char,
) -> libc::c_int {
    let mut pos: libc::c_int = 0;
    if ((*time_descr).t_start_time).is_null() {
        return -(1 as libc::c_int);
    }
    if ((*time_descr).t_stop_time).is_null() {
        return -(1 as libc::c_int);
    }
    tmp = __osip_sdp_append_string(
        string,
        size,
        tmp,
        b"t=\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    );
    tmp = __osip_sdp_append_string(string, size, tmp, (*time_descr).t_start_time);
    tmp = __osip_sdp_append_string(
        string,
        size,
        tmp,
        b" \0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    );
    tmp = __osip_sdp_append_string(string, size, tmp, (*time_descr).t_stop_time);
    tmp = __osip_sdp_append_string(
        string,
        size,
        tmp,
        b"\r\n\0\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    );
    pos = 0 as libc::c_int;
    while osip_list_eol(&mut (*time_descr).r_repeats, pos) == 0 {
        let mut str: *mut libc::c_char = osip_list_get(&mut (*time_descr).r_repeats, pos)
            as *mut libc::c_char;
        tmp = __osip_sdp_append_string(
            string,
            size,
            tmp,
            b"r=\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        );
        tmp = __osip_sdp_append_string(string, size, tmp, str);
        tmp = __osip_sdp_append_string(
            string,
            size,
            tmp,
            b"\r\n\0\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        );
        pos += 1;
        pos;
    }
    *next_tmp = tmp;
    return 0 as libc::c_int;
}
unsafe extern "C" fn sdp_append_key(
    mut string: *mut *mut libc::c_char,
    mut size: *mut libc::c_int,
    mut tmp: *mut libc::c_char,
    mut key: *mut sdp_key_t,
    mut next_tmp: *mut *mut libc::c_char,
) -> libc::c_int {
    if ((*key).k_keytype).is_null() {
        return -(1 as libc::c_int);
    }
    tmp = __osip_sdp_append_string(
        string,
        size,
        tmp,
        b"k=\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    );
    tmp = __osip_sdp_append_string(string, size, tmp, (*key).k_keytype);
    if !((*key).k_keydata).is_null() {
        tmp = __osip_sdp_append_string(
            string,
            size,
            tmp,
            b":\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        );
        tmp = __osip_sdp_append_string(string, size, tmp, (*key).k_keydata);
    }
    tmp = __osip_sdp_append_string(
        string,
        size,
        tmp,
        b"\r\n\0\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    );
    *next_tmp = tmp;
    return 0 as libc::c_int;
}
unsafe extern "C" fn sdp_append_attribute(
    mut string: *mut *mut libc::c_char,
    mut size: *mut libc::c_int,
    mut tmp: *mut libc::c_char,
    mut attribute: *mut sdp_attribute_t,
    mut next_tmp: *mut *mut libc::c_char,
) -> libc::c_int {
    if ((*attribute).a_att_field).is_null() {
        return -(1 as libc::c_int);
    }
    tmp = __osip_sdp_append_string(
        string,
        size,
        tmp,
        b"a=\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    );
    tmp = __osip_sdp_append_string(string, size, tmp, (*attribute).a_att_field);
    if !((*attribute).a_att_value).is_null() {
        tmp = __osip_sdp_append_string(
            string,
            size,
            tmp,
            b":\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        );
        tmp = __osip_sdp_append_string(string, size, tmp, (*attribute).a_att_value);
    }
    tmp = __osip_sdp_append_string(
        string,
        size,
        tmp,
        b"\r\n\0\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    );
    *next_tmp = tmp;
    return 0 as libc::c_int;
}
unsafe extern "C" fn sdp_append_media(
    mut string: *mut *mut libc::c_char,
    mut size: *mut libc::c_int,
    mut tmp: *mut libc::c_char,
    mut media: *mut sdp_media_t,
    mut next_tmp: *mut *mut libc::c_char,
) -> libc::c_int {
    let mut pos: libc::c_int = 0;
    if ((*media).m_media).is_null() {
        return -(1 as libc::c_int);
    }
    if ((*media).m_port).is_null() {
        return -(1 as libc::c_int);
    }
    if ((*media).m_proto).is_null() {
        return -(1 as libc::c_int);
    }
    tmp = __osip_sdp_append_string(
        string,
        size,
        tmp,
        b"m=\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    );
    tmp = __osip_sdp_append_string(string, size, tmp, (*media).m_media);
    tmp = __osip_sdp_append_string(
        string,
        size,
        tmp,
        b" \0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    );
    if *((*media).m_port).offset(0 as libc::c_int as isize) as libc::c_int == '0' as i32
    {
        tmp = __osip_sdp_append_string(
            string,
            size,
            tmp,
            b"0\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        );
    } else {
        tmp = __osip_sdp_append_string(string, size, tmp, (*media).m_port);
    }
    if !((*media).m_number_of_port).is_null() {
        tmp = __osip_sdp_append_string(
            string,
            size,
            tmp,
            b"/\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        );
        tmp = __osip_sdp_append_string(string, size, tmp, (*media).m_number_of_port);
    }
    tmp = __osip_sdp_append_string(
        string,
        size,
        tmp,
        b" \0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    );
    tmp = __osip_sdp_append_string(string, size, tmp, (*media).m_proto);
    pos = 0 as libc::c_int;
    while osip_list_eol(&mut (*media).m_payloads, pos) == 0 {
        let mut str: *mut libc::c_char = osip_list_get(&mut (*media).m_payloads, pos)
            as *mut libc::c_char;
        tmp = __osip_sdp_append_string(
            string,
            size,
            tmp,
            b" \0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        );
        tmp = __osip_sdp_append_string(string, size, tmp, str);
        pos += 1;
        pos;
    }
    tmp = __osip_sdp_append_string(
        string,
        size,
        tmp,
        b"\r\n\0\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    );
    if !((*media).i_info).is_null() {
        tmp = __osip_sdp_append_string(
            string,
            size,
            tmp,
            b"i=\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        );
        tmp = __osip_sdp_append_string(string, size, tmp, (*media).i_info);
        tmp = __osip_sdp_append_string(
            string,
            size,
            tmp,
            b"\r\n\0\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        );
    }
    pos = 0 as libc::c_int;
    while osip_list_eol(&mut (*media).c_connections, pos) == 0 {
        let mut conn: *mut sdp_connection_t = osip_list_get(
            &mut (*media).c_connections,
            pos,
        ) as *mut sdp_connection_t;
        let mut next_tmp2: *mut libc::c_char = 0 as *mut libc::c_char;
        let mut i: libc::c_int = 0;
        i = sdp_append_connection(string, size, tmp, conn, &mut next_tmp2);
        if i != 0 as libc::c_int {
            return -(1 as libc::c_int);
        }
        tmp = next_tmp2;
        pos += 1;
        pos;
    }
    pos = 0 as libc::c_int;
    while osip_list_eol(&mut (*media).b_bandwidths, pos) == 0 {
        let mut band: *mut sdp_bandwidth_t = osip_list_get(
            &mut (*media).b_bandwidths,
            pos,
        ) as *mut sdp_bandwidth_t;
        let mut next_tmp2_0: *mut libc::c_char = 0 as *mut libc::c_char;
        let mut i_0: libc::c_int = 0;
        i_0 = sdp_append_bandwidth(string, size, tmp, band, &mut next_tmp2_0);
        if i_0 != 0 as libc::c_int {
            return -(1 as libc::c_int);
        }
        tmp = next_tmp2_0;
        pos += 1;
        pos;
    }
    if !((*media).k_key).is_null() {
        let mut next_tmp2_1: *mut libc::c_char = 0 as *mut libc::c_char;
        let mut i_1: libc::c_int = 0;
        i_1 = sdp_append_key(string, size, tmp, (*media).k_key, &mut next_tmp2_1);
        if i_1 != 0 as libc::c_int {
            return -(1 as libc::c_int);
        }
        tmp = next_tmp2_1;
    }
    pos = 0 as libc::c_int;
    while osip_list_eol(&mut (*media).a_attributes, pos) == 0 {
        let mut attr: *mut sdp_attribute_t = osip_list_get(
            &mut (*media).a_attributes,
            pos,
        ) as *mut sdp_attribute_t;
        let mut next_tmp2_2: *mut libc::c_char = 0 as *mut libc::c_char;
        let mut i_2: libc::c_int = 0;
        i_2 = sdp_append_attribute(string, size, tmp, attr, &mut next_tmp2_2);
        if i_2 != 0 as libc::c_int {
            return -(1 as libc::c_int);
        }
        tmp = next_tmp2_2;
        pos += 1;
        pos;
    }
    *next_tmp = tmp;
    return 0 as libc::c_int;
}
pub unsafe extern "C" fn sdp_message_to_str(
    mut sdp: *mut sdp_message_t,
    mut dest: *mut *mut libc::c_char,
) -> libc::c_int {
    let mut size: libc::c_int = 0;
    let mut pos: libc::c_int = 0;
    let mut tmp: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut string: *mut libc::c_char = 0 as *mut libc::c_char;
    *dest = 0 as *mut libc::c_char;
    if sdp.is_null() || ((*sdp).v_version).is_null() {
        return -(1 as libc::c_int);
    }
    if ((*sdp).o_username).is_null() || ((*sdp).o_sess_id).is_null()
        || ((*sdp).o_sess_version).is_null() || ((*sdp).o_nettype).is_null()
        || ((*sdp).o_addrtype).is_null() || ((*sdp).o_addr).is_null()
    {
        return -(1 as libc::c_int);
    }
    size = 4000 as libc::c_int;
    tmp = (if osip_malloc_func.is_some() {
        osip_malloc_func.unwrap()(size as size_t)
    } else {
        malloc(size as libc::c_ulong)
    }) as *mut libc::c_char;
    if tmp.is_null() {
        return -(4 as libc::c_int);
    }
    string = tmp;
    tmp = __osip_sdp_append_string(
        &mut string,
        &mut size,
        tmp,
        b"v=\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    );
    tmp = __osip_sdp_append_string(&mut string, &mut size, tmp, (*sdp).v_version);
    tmp = __osip_sdp_append_string(
        &mut string,
        &mut size,
        tmp,
        b"\r\n\0\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    );
    tmp = __osip_sdp_append_string(
        &mut string,
        &mut size,
        tmp,
        b"o=\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    );
    tmp = __osip_sdp_append_string(&mut string, &mut size, tmp, (*sdp).o_username);
    tmp = __osip_sdp_append_string(
        &mut string,
        &mut size,
        tmp,
        b" \0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    );
    tmp = __osip_sdp_append_string(&mut string, &mut size, tmp, (*sdp).o_sess_id);
    tmp = __osip_sdp_append_string(
        &mut string,
        &mut size,
        tmp,
        b" \0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    );
    tmp = __osip_sdp_append_string(&mut string, &mut size, tmp, (*sdp).o_sess_version);
    tmp = __osip_sdp_append_string(
        &mut string,
        &mut size,
        tmp,
        b" \0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    );
    tmp = __osip_sdp_append_string(&mut string, &mut size, tmp, (*sdp).o_nettype);
    tmp = __osip_sdp_append_string(
        &mut string,
        &mut size,
        tmp,
        b" \0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    );
    tmp = __osip_sdp_append_string(&mut string, &mut size, tmp, (*sdp).o_addrtype);
    tmp = __osip_sdp_append_string(
        &mut string,
        &mut size,
        tmp,
        b" \0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    );
    tmp = __osip_sdp_append_string(&mut string, &mut size, tmp, (*sdp).o_addr);
    tmp = __osip_sdp_append_string(
        &mut string,
        &mut size,
        tmp,
        b"\r\n\0\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    );
    if !((*sdp).s_name).is_null() {
        tmp = __osip_sdp_append_string(
            &mut string,
            &mut size,
            tmp,
            b"s=\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        );
        tmp = __osip_sdp_append_string(&mut string, &mut size, tmp, (*sdp).s_name);
        tmp = __osip_sdp_append_string(
            &mut string,
            &mut size,
            tmp,
            b"\r\n\0\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        );
    }
    if !((*sdp).i_info).is_null() {
        tmp = __osip_sdp_append_string(
            &mut string,
            &mut size,
            tmp,
            b"i=\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        );
        tmp = __osip_sdp_append_string(&mut string, &mut size, tmp, (*sdp).i_info);
        tmp = __osip_sdp_append_string(
            &mut string,
            &mut size,
            tmp,
            b"\r\n\0\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        );
    }
    if !((*sdp).u_uri).is_null() {
        tmp = __osip_sdp_append_string(
            &mut string,
            &mut size,
            tmp,
            b"u=\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        );
        tmp = __osip_sdp_append_string(&mut string, &mut size, tmp, (*sdp).u_uri);
        tmp = __osip_sdp_append_string(
            &mut string,
            &mut size,
            tmp,
            b"\r\n\0\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        );
    }
    pos = 0 as libc::c_int;
    while osip_list_eol(&mut (*sdp).e_emails, pos) == 0 {
        let mut email: *mut libc::c_char = osip_list_get(&mut (*sdp).e_emails, pos)
            as *mut libc::c_char;
        tmp = __osip_sdp_append_string(
            &mut string,
            &mut size,
            tmp,
            b"e=\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        );
        tmp = __osip_sdp_append_string(&mut string, &mut size, tmp, email);
        tmp = __osip_sdp_append_string(
            &mut string,
            &mut size,
            tmp,
            b"\r\n\0\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        );
        pos += 1;
        pos;
    }
    pos = 0 as libc::c_int;
    while osip_list_eol(&mut (*sdp).p_phones, pos) == 0 {
        let mut phone: *mut libc::c_char = osip_list_get(&mut (*sdp).p_phones, pos)
            as *mut libc::c_char;
        tmp = __osip_sdp_append_string(
            &mut string,
            &mut size,
            tmp,
            b"p=\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        );
        tmp = __osip_sdp_append_string(&mut string, &mut size, tmp, phone);
        tmp = __osip_sdp_append_string(
            &mut string,
            &mut size,
            tmp,
            b"\r\n\0\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        );
        pos += 1;
        pos;
    }
    if !((*sdp).c_connection).is_null() {
        let mut next_tmp: *mut libc::c_char = 0 as *mut libc::c_char;
        let mut i: libc::c_int = 0;
        i = sdp_append_connection(
            &mut string,
            &mut size,
            tmp,
            (*sdp).c_connection,
            &mut next_tmp,
        );
        if i != 0 as libc::c_int {
            if !string.is_null() {
                if osip_free_func.is_some() {
                    osip_free_func.unwrap()(string as *mut libc::c_void);
                } else {
                    free(string as *mut libc::c_void);
                }
            }
            return -(1 as libc::c_int);
        }
        tmp = next_tmp;
    }
    pos = 0 as libc::c_int;
    while osip_list_eol(&mut (*sdp).b_bandwidths, pos) == 0 {
        let mut header: *mut sdp_bandwidth_t = osip_list_get(
            &mut (*sdp).b_bandwidths,
            pos,
        ) as *mut sdp_bandwidth_t;
        let mut next_tmp_0: *mut libc::c_char = 0 as *mut libc::c_char;
        let mut i_0: libc::c_int = 0;
        i_0 = sdp_append_bandwidth(&mut string, &mut size, tmp, header, &mut next_tmp_0);
        if i_0 != 0 as libc::c_int {
            if !string.is_null() {
                if osip_free_func.is_some() {
                    osip_free_func.unwrap()(string as *mut libc::c_void);
                } else {
                    free(string as *mut libc::c_void);
                }
            }
            return -(1 as libc::c_int);
        }
        tmp = next_tmp_0;
        pos += 1;
        pos;
    }
    pos = 0 as libc::c_int;
    while osip_list_eol(&mut (*sdp).t_descrs, pos) == 0 {
        let mut header_0: *mut sdp_time_descr_t = osip_list_get(
            &mut (*sdp).t_descrs,
            pos,
        ) as *mut sdp_time_descr_t;
        let mut next_tmp_1: *mut libc::c_char = 0 as *mut libc::c_char;
        let mut i_1: libc::c_int = 0;
        i_1 = sdp_append_time_descr(
            &mut string,
            &mut size,
            tmp,
            header_0,
            &mut next_tmp_1,
        );
        if i_1 != 0 as libc::c_int {
            if !string.is_null() {
                if osip_free_func.is_some() {
                    osip_free_func.unwrap()(string as *mut libc::c_void);
                } else {
                    free(string as *mut libc::c_void);
                }
            }
            return -(1 as libc::c_int);
        }
        tmp = next_tmp_1;
        pos += 1;
        pos;
    }
    if !((*sdp).z_adjustments).is_null() {
        tmp = __osip_sdp_append_string(
            &mut string,
            &mut size,
            tmp,
            b"z=\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        );
        tmp = __osip_sdp_append_string(
            &mut string,
            &mut size,
            tmp,
            (*sdp).z_adjustments,
        );
        tmp = __osip_sdp_append_string(
            &mut string,
            &mut size,
            tmp,
            b"\r\n\0\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        );
    }
    if !((*sdp).k_key).is_null() {
        let mut next_tmp_2: *mut libc::c_char = 0 as *mut libc::c_char;
        let mut i_2: libc::c_int = 0;
        i_2 = sdp_append_key(&mut string, &mut size, tmp, (*sdp).k_key, &mut next_tmp_2);
        if i_2 != 0 as libc::c_int {
            if !string.is_null() {
                if osip_free_func.is_some() {
                    osip_free_func.unwrap()(string as *mut libc::c_void);
                } else {
                    free(string as *mut libc::c_void);
                }
            }
            return -(1 as libc::c_int);
        }
        tmp = next_tmp_2;
    }
    pos = 0 as libc::c_int;
    while osip_list_eol(&mut (*sdp).a_attributes, pos) == 0 {
        let mut header_1: *mut sdp_attribute_t = osip_list_get(
            &mut (*sdp).a_attributes,
            pos,
        ) as *mut sdp_attribute_t;
        let mut next_tmp_3: *mut libc::c_char = 0 as *mut libc::c_char;
        let mut i_3: libc::c_int = 0;
        i_3 = sdp_append_attribute(
            &mut string,
            &mut size,
            tmp,
            header_1,
            &mut next_tmp_3,
        );
        if i_3 != 0 as libc::c_int {
            if !string.is_null() {
                if osip_free_func.is_some() {
                    osip_free_func.unwrap()(string as *mut libc::c_void);
                } else {
                    free(string as *mut libc::c_void);
                }
            }
            return -(1 as libc::c_int);
        }
        tmp = next_tmp_3;
        pos += 1;
        pos;
    }
    pos = 0 as libc::c_int;
    while osip_list_eol(&mut (*sdp).m_medias, pos) == 0 {
        let mut header_2: *mut sdp_media_t = osip_list_get(&mut (*sdp).m_medias, pos)
            as *mut sdp_media_t;
        let mut next_tmp_4: *mut libc::c_char = 0 as *mut libc::c_char;
        let mut i_4: libc::c_int = 0;
        i_4 = sdp_append_media(&mut string, &mut size, tmp, header_2, &mut next_tmp_4);
        if i_4 != 0 as libc::c_int {
            if !string.is_null() {
                if osip_free_func.is_some() {
                    osip_free_func.unwrap()(string as *mut libc::c_void);
                } else {
                    free(string as *mut libc::c_void);
                }
            }
            return -(1 as libc::c_int);
        }
        tmp = next_tmp_4;
        pos += 1;
        pos;
    }
    *dest = string;
    return 0 as libc::c_int;
}
pub unsafe extern "C" fn sdp_message_free(mut sdp: *mut sdp_message_t) {
    if sdp.is_null() {
        return;
    }
    if !((*sdp).v_version).is_null() {
        if osip_free_func.is_some() {
            osip_free_func.unwrap()((*sdp).v_version as *mut libc::c_void);
        } else {
            free((*sdp).v_version as *mut libc::c_void);
        }
    }
    if !((*sdp).o_username).is_null() {
        if osip_free_func.is_some() {
            osip_free_func.unwrap()((*sdp).o_username as *mut libc::c_void);
        } else {
            free((*sdp).o_username as *mut libc::c_void);
        }
    }
    if !((*sdp).o_sess_id).is_null() {
        if osip_free_func.is_some() {
            osip_free_func.unwrap()((*sdp).o_sess_id as *mut libc::c_void);
        } else {
            free((*sdp).o_sess_id as *mut libc::c_void);
        }
    }
    if !((*sdp).o_sess_version).is_null() {
        if osip_free_func.is_some() {
            osip_free_func.unwrap()((*sdp).o_sess_version as *mut libc::c_void);
        } else {
            free((*sdp).o_sess_version as *mut libc::c_void);
        }
    }
    if !((*sdp).o_nettype).is_null() {
        if osip_free_func.is_some() {
            osip_free_func.unwrap()((*sdp).o_nettype as *mut libc::c_void);
        } else {
            free((*sdp).o_nettype as *mut libc::c_void);
        }
    }
    if !((*sdp).o_addrtype).is_null() {
        if osip_free_func.is_some() {
            osip_free_func.unwrap()((*sdp).o_addrtype as *mut libc::c_void);
        } else {
            free((*sdp).o_addrtype as *mut libc::c_void);
        }
    }
    if !((*sdp).o_addr).is_null() {
        if osip_free_func.is_some() {
            osip_free_func.unwrap()((*sdp).o_addr as *mut libc::c_void);
        } else {
            free((*sdp).o_addr as *mut libc::c_void);
        }
    }
    if !((*sdp).s_name).is_null() {
        if osip_free_func.is_some() {
            osip_free_func.unwrap()((*sdp).s_name as *mut libc::c_void);
        } else {
            free((*sdp).s_name as *mut libc::c_void);
        }
    }
    if !((*sdp).i_info).is_null() {
        if osip_free_func.is_some() {
            osip_free_func.unwrap()((*sdp).i_info as *mut libc::c_void);
        } else {
            free((*sdp).i_info as *mut libc::c_void);
        }
    }
    if !((*sdp).u_uri).is_null() {
        if osip_free_func.is_some() {
            osip_free_func.unwrap()((*sdp).u_uri as *mut libc::c_void);
        } else {
            free((*sdp).u_uri as *mut libc::c_void);
        }
    }
    osip_list_ofchar_free(&mut (*sdp).e_emails);
    osip_list_ofchar_free(&mut (*sdp).p_phones);
    sdp_connection_free((*sdp).c_connection);
    osip_list_special_free(
        &mut (*sdp).b_bandwidths,
        ::std::mem::transmute::<
            Option::<unsafe extern "C" fn(*mut sdp_bandwidth_t) -> ()>,
            Option::<unsafe extern "C" fn(*mut libc::c_void) -> ()>,
        >(Some(sdp_bandwidth_free as unsafe extern "C" fn(*mut sdp_bandwidth_t) -> ())),
    );
    osip_list_special_free(
        &mut (*sdp).t_descrs,
        ::std::mem::transmute::<
            Option::<unsafe extern "C" fn(*mut sdp_time_descr_t) -> ()>,
            Option::<unsafe extern "C" fn(*mut libc::c_void) -> ()>,
        >(Some(sdp_time_descr_free as unsafe extern "C" fn(*mut sdp_time_descr_t) -> ())),
    );
    if !((*sdp).z_adjustments).is_null() {
        if osip_free_func.is_some() {
            osip_free_func.unwrap()((*sdp).z_adjustments as *mut libc::c_void);
        } else {
            free((*sdp).z_adjustments as *mut libc::c_void);
        }
    }
    sdp_key_free((*sdp).k_key);
    osip_list_special_free(
        &mut (*sdp).a_attributes,
        ::std::mem::transmute::<
            Option::<unsafe extern "C" fn(*mut sdp_attribute_t) -> ()>,
            Option::<unsafe extern "C" fn(*mut libc::c_void) -> ()>,
        >(Some(sdp_attribute_free as unsafe extern "C" fn(*mut sdp_attribute_t) -> ())),
    );
    osip_list_special_free(
        &mut (*sdp).m_medias,
        ::std::mem::transmute::<
            Option::<unsafe extern "C" fn(*mut sdp_media_t) -> ()>,
            Option::<unsafe extern "C" fn(*mut libc::c_void) -> ()>,
        >(Some(sdp_media_free as unsafe extern "C" fn(*mut sdp_media_t) -> ())),
    );
    if !sdp.is_null() {
        if osip_free_func.is_some() {
            osip_free_func.unwrap()(sdp as *mut libc::c_void);
        } else {
            free(sdp as *mut libc::c_void);
        }
    }
}
pub unsafe extern "C" fn sdp_message_clone(
    mut sdp: *mut sdp_message_t,
    mut dest: *mut *mut sdp_message_t,
) -> libc::c_int {
    let mut i: libc::c_int = 0;
    let mut body: *mut libc::c_char = 0 as *mut libc::c_char;
    i = sdp_message_init(dest);
    if i != 0 as libc::c_int {
        return -(1 as libc::c_int);
    }
    i = sdp_message_to_str(sdp, &mut body);
    if !(i != 0 as libc::c_int) {
        i = sdp_message_parse(*dest, body);
        if !body.is_null() {
            if osip_free_func.is_some() {
                osip_free_func.unwrap()(body as *mut libc::c_void);
            } else {
                free(body as *mut libc::c_void);
            }
        }
        if !(i != 0 as libc::c_int) {
            return 0 as libc::c_int;
        }
    }
    sdp_message_free(*dest);
    return -(1 as libc::c_int);
}
