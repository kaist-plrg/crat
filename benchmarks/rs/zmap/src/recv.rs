use ::libc;
use ::c2rust_bitfields;
extern "C" {
    pub type _IO_wide_data;
    pub type _IO_codecvt;
    pub type _IO_marker;
    pub type aesrand;
    fn pthread_mutex_lock(__mutex: *mut pthread_mutex_t) -> libc::c_int;
    fn pthread_mutex_unlock(__mutex: *mut pthread_mutex_t) -> libc::c_int;
    fn recv_update_stats() -> libc::c_int;
    fn __assert_fail(
        __assertion: *const libc::c_char,
        __file: *const libc::c_char,
        __line: libc::c_uint,
        __function: *const libc::c_char,
    ) -> !;
    fn log_debug(
        loggerName: *const libc::c_char,
        logMessage: *const libc::c_char,
        _: ...
    ) -> libc::c_int;
    fn now() -> libc::c_double;
    fn log_info(
        loggerName: *const libc::c_char,
        logMessage: *const libc::c_char,
        _: ...
    ) -> libc::c_int;
    fn pbm_init() -> *mut *mut uint8_t;
    fn pbm_check(b: *mut *mut uint8_t, v: uint32_t) -> libc::c_int;
    fn pbm_set(b: *mut *mut uint8_t, v: uint32_t);
    fn sleep(__seconds: libc::c_uint) -> libc::c_uint;
    fn recv_init();
    fn recv_packets();
    fn recv_cleanup();
    fn free(__ptr: *mut libc::c_void);
    fn memcpy(
        _: *mut libc::c_void,
        _: *const libc::c_void,
        _: libc::c_ulong,
    ) -> *mut libc::c_void;
    static mut zrecv: state_recv;
    fn memset(
        _: *mut libc::c_void,
        _: libc::c_int,
        _: libc::c_ulong,
    ) -> *mut libc::c_void;
    fn evaluate_expression(root: *mut node_t, fields: *mut fieldset_t) -> libc::c_int;
    fn fs_new_fieldset(_: *mut fielddefset_t) -> *mut fieldset_t;
    static mut zsend: state_send;
    static mut zconf: state_conf;
    fn fs_get_uint64_by_index(fs: *mut fieldset_t, index: libc::c_int) -> uint64_t;
    fn fs_free(fs: *mut fieldset_t);
    fn translate_fieldset(fs: *mut fieldset_t, t: *mut translation_t) -> *mut fieldset_t;
    fn validate_gen(src: uint32_t, dst: uint32_t, output: *mut uint8_t);
    fn fs_add_ip_fields(fs: *mut fieldset_t, ip: *mut ip);
    fn fs_add_system_fields(
        fs: *mut fieldset_t,
        is_repeat: libc::c_int,
        in_cooldown: libc::c_int,
    );
}
pub type __u_char = libc::c_uchar;
pub type __uint8_t = libc::c_uchar;
pub type __uint16_t = libc::c_ushort;
pub type __uint32_t = libc::c_uint;
pub type __uint64_t = libc::c_ulong;
pub type __off_t = libc::c_long;
pub type __off64_t = libc::c_long;
pub type __time_t = libc::c_long;
pub type __syscall_slong_t = libc::c_long;
pub type size_t = libc::c_ulong;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct timespec {
    pub tv_sec: __time_t,
    pub tv_nsec: __syscall_slong_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct __pthread_internal_list {
    pub __prev: *mut __pthread_internal_list,
    pub __next: *mut __pthread_internal_list,
}
pub type __pthread_list_t = __pthread_internal_list;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct __pthread_mutex_s {
    pub __lock: libc::c_int,
    pub __count: libc::c_uint,
    pub __owner: libc::c_int,
    pub __nusers: libc::c_uint,
    pub __kind: libc::c_int,
    pub __spins: libc::c_short,
    pub __elision: libc::c_short,
    pub __list: __pthread_list_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union pthread_mutex_t {
    pub __data: __pthread_mutex_s,
    pub __size: [libc::c_char; 40],
    pub __align: libc::c_long,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct state_recv {
    pub success_total: uint32_t,
    pub success_unique: uint32_t,
    pub app_success_total: uint32_t,
    pub app_success_unique: uint32_t,
    pub cooldown_total: uint32_t,
    pub cooldown_unique: uint32_t,
    pub failure_total: uint32_t,
    pub filter_success: uint64_t,
    pub ip_fragments: uint32_t,
    pub validation_passed: uint32_t,
    pub validation_failed: uint32_t,
    pub complete: libc::c_int,
    pub start: libc::c_double,
    pub finish: libc::c_double,
    pub pcap_recv: uint32_t,
    pub pcap_drop: uint32_t,
    pub pcap_ifdrop: uint32_t,
}
pub type uint32_t = __uint32_t;
pub type uint64_t = __uint64_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct state_conf {
    pub log_level: libc::c_int,
    pub target_port: port_h_t,
    pub source_port_first: port_h_t,
    pub source_port_last: port_h_t,
    pub max_targets: uint32_t,
    pub max_runtime: uint32_t,
    pub max_results: uint32_t,
    pub iface: *mut libc::c_char,
    pub rate: libc::c_int,
    pub bandwidth: uint64_t,
    pub cooldown_secs: libc::c_int,
    pub senders: uint8_t,
    pub batch: uint8_t,
    pub pin_cores_len: uint32_t,
    pub pin_cores: *mut uint32_t,
    pub seed_provided: libc::c_int,
    pub seed: uint64_t,
    pub aes: *mut aesrand_t,
    pub generator: uint32_t,
    pub shard_num: uint16_t,
    pub total_shards: uint16_t,
    pub packet_streams: libc::c_int,
    pub probe_module: *mut probe_module,
    pub output_module_name: *mut libc::c_char,
    pub output_module: *mut output_module,
    pub probe_args: *mut libc::c_char,
    pub probe_ttl: uint8_t,
    pub output_args: *mut libc::c_char,
    pub gw_mac: [macaddr_t; 6],
    pub hw_mac: [macaddr_t; 6],
    pub gw_ip: uint32_t,
    pub gw_mac_set: libc::c_int,
    pub hw_mac_set: libc::c_int,
    pub source_ip_addresses: [in_addr_t; 256],
    pub number_source_ips: uint32_t,
    pub send_ip_pkts: libc::c_int,
    pub output_filename: *mut libc::c_char,
    pub blocklist_filename: *mut libc::c_char,
    pub allowlist_filename: *mut libc::c_char,
    pub list_of_ips_filename: *mut libc::c_char,
    pub list_of_ips_count: uint32_t,
    pub metadata_filename: *mut libc::c_char,
    pub metadata_file: *mut FILE,
    pub notes: *mut libc::c_char,
    pub custom_metadata_str: *mut libc::c_char,
    pub destination_cidrs: *mut *mut libc::c_char,
    pub destination_cidrs_len: libc::c_int,
    pub raw_output_fields: *const libc::c_char,
    pub output_fields: *mut *const libc::c_char,
    pub filter: output_filter,
    pub output_filter_str: *mut libc::c_char,
    pub fsconf: fieldset_conf,
    pub output_fields_len: libc::c_int,
    pub log_file: *mut libc::c_char,
    pub log_directory: *mut libc::c_char,
    pub status_updates_file: *mut libc::c_char,
    pub dryrun: libc::c_int,
    pub quiet: libc::c_int,
    pub ignore_invalid_hosts: libc::c_int,
    pub syslog: libc::c_int,
    pub recv_ready: libc::c_int,
    pub num_retries: libc::c_int,
    pub total_allowed: uint64_t,
    pub total_disallowed: uint64_t,
    pub max_sendto_failures: libc::c_int,
    pub min_hitrate: libc::c_float,
    pub data_link_size: libc::c_int,
    pub default_mode: libc::c_int,
    pub no_header_row: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct fieldset_conf {
    pub defs: fielddefset_t,
    pub outdefs: fielddefset_t,
    pub translation: translation_t,
    pub success_index: libc::c_int,
    pub app_success_index: libc::c_int,
    pub classification_index: libc::c_int,
}
pub type translation_t = translation;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct translation {
    pub len: libc::c_int,
    pub translation: [libc::c_int; 128],
}
pub type fielddefset_t = fielddef_set;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct fielddef_set {
    pub fielddefs: [fielddef_t; 128],
    pub len: libc::c_int,
}
pub type fielddef_t = field_def;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct field_def {
    pub name: *const libc::c_char,
    pub type_0: *const libc::c_char,
    pub desc: *const libc::c_char,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct output_filter {
    pub expression: *mut node_t,
}
pub type node_t = node_st;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct node_st {
    pub left_child: *mut node_st,
    pub right_child: *mut node_st,
    pub type_0: node_type,
    pub value: node_value,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union node_value {
    pub field: field_id,
    pub string_literal: *mut libc::c_char,
    pub int_literal: uint64_t,
    pub op: operation,
}
pub type operation = libc::c_uint;
pub const GT_EQ: operation = 7;
pub const LT_EQ: operation = 6;
pub const OR: operation = 5;
pub const AND: operation = 4;
pub const NEQ: operation = 3;
pub const EQ: operation = 2;
pub const LT: operation = 1;
pub const GT: operation = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct field_id {
    pub index: libc::c_int,
    pub fieldname: *mut libc::c_char,
}
pub type node_type = libc::c_uint;
pub const INT: node_type = 3;
pub const STRING: node_type = 2;
pub const FIELD: node_type = 1;
pub const OP: node_type = 0;
pub type FILE = _IO_FILE;
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
pub type in_addr_t = uint32_t;
pub type macaddr_t = libc::c_uchar;
pub type uint8_t = __uint8_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct output_module {
    pub name: *const libc::c_char,
    pub supports_dynamic_output: libc::c_int,
    pub update_interval: libc::c_uint,
    pub init: output_init_cb,
    pub start: output_update_cb,
    pub update: output_update_cb,
    pub close: output_update_cb,
    pub process_ip: output_packet_cb,
    pub helptext: *const libc::c_char,
}
pub type output_packet_cb = Option::<
    unsafe extern "C" fn(*mut fieldset_t) -> libc::c_int,
>;
pub type fieldset_t = fieldset;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct fieldset {
    pub len: libc::c_int,
    pub fields: [field_t; 128],
    pub fds: *mut fielddefset_t,
    pub inner_type: libc::c_int,
    pub type_0: libc::c_int,
    pub free_: libc::c_int,
}
pub type field_t = field;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct field {
    pub name: *const libc::c_char,
    pub type_0: libc::c_int,
    pub free_: libc::c_int,
    pub len: size_t,
    pub value: field_val_t,
}
pub type field_val_t = field_val;
#[derive(Copy, Clone)]
#[repr(C)]
pub union field_val {
    pub ptr: *mut libc::c_void,
    pub num: uint64_t,
}
pub type output_update_cb = Option::<
    unsafe extern "C" fn(
        *mut state_conf,
        *mut state_send,
        *mut state_recv,
    ) -> libc::c_int,
>;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct state_send {
    pub start: libc::c_double,
    pub finish: libc::c_double,
    pub packets_sent: uint64_t,
    pub hosts_scanned: uint64_t,
    pub blocklisted: uint64_t,
    pub allowlisted: uint64_t,
    pub warmup: libc::c_int,
    pub complete: libc::c_int,
    pub first_scanned: uint32_t,
    pub max_targets: uint32_t,
    pub sendto_failures: uint32_t,
    pub max_index: uint32_t,
    pub list_of_ips_pbm: *mut *mut uint8_t,
}
pub type output_init_cb = Option::<
    unsafe extern "C" fn(
        *mut state_conf,
        *mut *const libc::c_char,
        libc::c_int,
    ) -> libc::c_int,
>;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct probe_module {
    pub name: *const libc::c_char,
    pub max_packet_length: size_t,
    pub pcap_filter: *const libc::c_char,
    pub pcap_snaplen: size_t,
    pub port_args: uint8_t,
    pub global_initialize: probe_global_init_cb,
    pub thread_initialize: probe_thread_init_cb,
    pub make_packet: probe_make_packet_cb,
    pub print_packet: probe_print_packet_cb,
    pub validate_packet: probe_validate_packet_cb,
    pub process_packet: probe_classify_packet_cb,
    pub close: probe_close_cb,
    pub output_type: libc::c_int,
    pub fields: *mut fielddef_t,
    pub numfields: libc::c_int,
    pub helptext: *const libc::c_char,
}
pub type probe_close_cb = Option::<
    unsafe extern "C" fn(
        *mut state_conf,
        *mut state_send,
        *mut state_recv,
    ) -> libc::c_int,
>;
pub type probe_classify_packet_cb = Option::<
    unsafe extern "C" fn(
        *const u_char,
        uint32_t,
        *mut fieldset_t,
        *mut uint32_t,
        timespec,
    ) -> (),
>;
pub type u_char = __u_char;
pub type probe_validate_packet_cb = Option::<
    unsafe extern "C" fn(
        *const ip,
        uint32_t,
        *mut uint32_t,
        *mut uint32_t,
    ) -> libc::c_int,
>;
#[derive(Copy, Clone, BitfieldStruct)]
#[repr(C)]
pub struct ip {
    #[bitfield(name = "ip_hl", ty = "libc::c_uint", bits = "0..=3")]
    #[bitfield(name = "ip_v", ty = "libc::c_uint", bits = "4..=7")]
    pub ip_hl_ip_v: [u8; 1],
    pub ip_tos: uint8_t,
    pub ip_len: libc::c_ushort,
    pub ip_id: libc::c_ushort,
    pub ip_off: libc::c_ushort,
    pub ip_ttl: uint8_t,
    pub ip_p: uint8_t,
    pub ip_sum: libc::c_ushort,
    pub ip_src: in_addr,
    pub ip_dst: in_addr,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct in_addr {
    pub s_addr: in_addr_t,
}
pub type probe_print_packet_cb = Option::<
    unsafe extern "C" fn(*mut FILE, *mut libc::c_void) -> (),
>;
pub type probe_make_packet_cb = Option::<
    unsafe extern "C" fn(
        *mut libc::c_void,
        *mut size_t,
        ipaddr_n_t,
        ipaddr_n_t,
        uint8_t,
        *mut uint32_t,
        libc::c_int,
        *mut libc::c_void,
    ) -> libc::c_int,
>;
pub type ipaddr_n_t = uint32_t;
pub type probe_thread_init_cb = Option::<
    unsafe extern "C" fn(
        *mut libc::c_void,
        *mut macaddr_t,
        *mut macaddr_t,
        port_n_t,
        *mut *mut libc::c_void,
    ) -> libc::c_int,
>;
pub type port_n_t = uint16_t;
pub type uint16_t = __uint16_t;
pub type probe_global_init_cb = Option::<
    unsafe extern "C" fn(*mut state_conf) -> libc::c_int,
>;
pub type aesrand_t = aesrand;
pub type port_h_t = uint16_t;
#[derive(Copy, Clone)]
#[repr(C, packed)]
pub struct ether_header {
    pub ether_dhost: [uint8_t; 6],
    pub ether_shost: [uint8_t; 6],
    pub ether_type: uint16_t,
}
#[inline]
unsafe extern "C" fn __bswap_16(mut __bsx: __uint16_t) -> __uint16_t {
    return (__bsx as libc::c_int >> 8 as libc::c_int & 0xff as libc::c_int
        | (__bsx as libc::c_int & 0xff as libc::c_int) << 8 as libc::c_int)
        as __uint16_t;
}
#[inline]
unsafe extern "C" fn __bswap_32(mut __bsx: __uint32_t) -> __uint32_t {
    return (__bsx & 0xff000000 as libc::c_uint) >> 24 as libc::c_int
        | (__bsx & 0xff0000 as libc::c_uint) >> 8 as libc::c_int
        | (__bsx & 0xff00 as libc::c_uint) << 8 as libc::c_int
        | (__bsx & 0xff as libc::c_uint) << 24 as libc::c_int;
}
static mut fake_eth_hdr: [u_char; 65535] = [0; 65535];
static mut seen: *mut *mut uint8_t = 0 as *const *mut uint8_t as *mut *mut uint8_t;
pub unsafe extern "C" fn handle_packet(
    mut buflen: uint32_t,
    mut bytes: *const u_char,
    ts: timespec,
) {
    if (::std::mem::size_of::<ip>() as libc::c_ulong)
        .wrapping_add(zconf.data_link_size as libc::c_ulong) > buflen as libc::c_ulong
    {
        return;
    }
    let mut ip_hdr: *mut ip = &*bytes.offset(zconf.data_link_size as isize)
        as *const u_char as *mut ip;
    let mut src_ip: uint32_t = (*ip_hdr).ip_src.s_addr;
    let mut validation: [uint32_t; 16] = [0; 16];
    validate_gen(
        (*ip_hdr).ip_dst.s_addr,
        (*ip_hdr).ip_src.s_addr,
        validation.as_mut_ptr() as *mut uint8_t,
    );
    if ((*zconf.probe_module).validate_packet)
        .unwrap()(
        ip_hdr,
        (buflen as libc::c_ulong)
            .wrapping_sub(
                (if zconf.send_ip_pkts != 0 {
                    0 as libc::c_int as libc::c_ulong
                } else {
                    ::std::mem::size_of::<ether_header>() as libc::c_ulong
                }),
            ) as uint32_t,
        &mut src_ip,
        validation.as_mut_ptr(),
    ) == 0
    {
        zrecv.validation_failed = (zrecv.validation_failed).wrapping_add(1);
        zrecv.validation_failed;
        return;
    } else {
        zrecv.validation_passed = (zrecv.validation_passed).wrapping_add(1);
        zrecv.validation_passed;
    }
    let mut is_repeat: libc::c_int = pbm_check(seen, __bswap_32(src_ip));
    if (*ip_hdr).ip_off as libc::c_int & 0x2000 as libc::c_int != 0 {
        zrecv.ip_fragments = (zrecv.ip_fragments).wrapping_add(1);
        zrecv.ip_fragments;
    }
    let mut fs: *mut fieldset_t = fs_new_fieldset(&mut zconf.fsconf.defs);
    fs_add_ip_fields(fs, ip_hdr);
    if zconf.send_ip_pkts != 0 {
        if buflen as libc::c_ulong
            > ::std::mem::size_of::<[u_char; 65535]>() as libc::c_ulong
        {
            buflen = ::std::mem::size_of::<[u_char; 65535]>() as libc::c_ulong
                as uint32_t;
        }
        memcpy(
            &mut *fake_eth_hdr
                .as_mut_ptr()
                .offset(::std::mem::size_of::<ether_header>() as libc::c_ulong as isize)
                as *mut u_char as *mut libc::c_void,
            bytes.offset(zconf.data_link_size as isize) as *const libc::c_void,
            buflen as libc::c_ulong,
        );
        bytes = fake_eth_hdr.as_mut_ptr();
    }
    ((*zconf.probe_module).process_packet)
        .unwrap()(bytes, buflen, fs, validation.as_mut_ptr(), ts);
    fs_add_system_fields(fs, is_repeat, zsend.complete);
    let mut success_index: libc::c_int = zconf.fsconf.success_index;
    if success_index < (*fs).len {} else {
        __assert_fail(
            b"success_index < fs->len\0" as *const u8 as *const libc::c_char,
            b"recv.c\0" as *const u8 as *const libc::c_char,
            84 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 68],
                &[libc::c_char; 68],
            >(b"void handle_packet(uint32_t, const u_char *, const struct timespec)\0"))
                .as_ptr(),
        );
    }
    'c_5049: {
        if success_index < (*fs).len {} else {
            __assert_fail(
                b"success_index < fs->len\0" as *const u8 as *const libc::c_char,
                b"recv.c\0" as *const u8 as *const libc::c_char,
                84 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 68],
                    &[libc::c_char; 68],
                >(
                    b"void handle_packet(uint32_t, const u_char *, const struct timespec)\0",
                ))
                    .as_ptr(),
            );
        }
    };
    let mut is_success: libc::c_int = fs_get_uint64_by_index(fs, success_index)
        as libc::c_int;
    if is_success != 0 {
        zrecv.success_total = (zrecv.success_total).wrapping_add(1);
        zrecv.success_total;
        if is_repeat == 0 {
            zrecv.success_unique = (zrecv.success_unique).wrapping_add(1);
            zrecv.success_unique;
            pbm_set(seen, __bswap_32(src_ip));
        }
        if zsend.complete != 0 {
            zrecv.cooldown_total = (zrecv.cooldown_total).wrapping_add(1);
            zrecv.cooldown_total;
            if is_repeat == 0 {
                zrecv.cooldown_unique = (zrecv.cooldown_unique).wrapping_add(1);
                zrecv.cooldown_unique;
            }
        }
    } else {
        zrecv.failure_total = (zrecv.failure_total).wrapping_add(1);
        zrecv.failure_total;
    }
    if zconf.fsconf.app_success_index >= 0 as libc::c_int {
        let mut is_app_success: libc::c_int = fs_get_uint64_by_index(
            fs,
            zconf.fsconf.app_success_index,
        ) as libc::c_int;
        if is_app_success != 0 {
            zrecv.app_success_total = (zrecv.app_success_total).wrapping_add(1);
            zrecv.app_success_total;
            if is_repeat == 0 {
                zrecv.app_success_unique = (zrecv.app_success_unique).wrapping_add(1);
                zrecv.app_success_unique;
            }
        }
    }
    let mut o: *mut fieldset_t = 0 as *mut fieldset_t;
    if !(is_success == 0 && zconf.default_mode != 0) {
        if !(is_repeat != 0 && zconf.default_mode != 0) {
            if !(evaluate_expression(zconf.filter.expression, fs) == 0) {
                zrecv.filter_success = (zrecv.filter_success).wrapping_add(1);
                zrecv.filter_success;
                o = translate_fieldset(fs, &mut zconf.fsconf.translation);
                if !(zconf.output_module).is_null()
                    && ((*zconf.output_module).process_ip).is_some()
                {
                    ((*zconf.output_module).process_ip).unwrap()(o);
                }
            }
        }
    }
    fs_free(fs);
    free(o as *mut libc::c_void);
    if !(zconf.output_module).is_null() && ((*zconf.output_module).update).is_some()
        && (zrecv.success_unique).wrapping_rem((*zconf.output_module).update_interval)
            == 0
    {
        ((*zconf.output_module).update).unwrap()(&mut zconf, &mut zsend, &mut zrecv);
    }
}
pub unsafe extern "C" fn recv_run(
    mut recv_ready_mutex: *mut pthread_mutex_t,
) -> libc::c_int {
    log_debug(
        b"recv\0" as *const u8 as *const libc::c_char,
        b"capturing responses on %s\0" as *const u8 as *const libc::c_char,
        zconf.iface,
    );
    if zconf.dryrun == 0 {
        recv_init();
    }
    if zconf.send_ip_pkts != 0 {
        let mut eth: *mut ether_header = fake_eth_hdr.as_mut_ptr() as *mut ether_header;
        memset(
            fake_eth_hdr.as_mut_ptr() as *mut libc::c_void,
            0 as libc::c_int,
            ::std::mem::size_of::<[u_char; 65535]>() as libc::c_ulong,
        );
        (*eth).ether_type = __bswap_16(0x800 as libc::c_int as __uint16_t);
    }
    seen = pbm_init();
    if zconf.default_mode != 0 {
        log_info(
            b"recv\0" as *const u8 as *const libc::c_char,
            b"duplicate responses will be excluded from output\0" as *const u8
                as *const libc::c_char,
        );
        log_info(
            b"recv\0" as *const u8 as *const libc::c_char,
            b"unsuccessful responses will be excluded from output\0" as *const u8
                as *const libc::c_char,
        );
    } else {
        log_info(
            b"recv\0" as *const u8 as *const libc::c_char,
            b"duplicate responses will be passed to the output module\0" as *const u8
                as *const libc::c_char,
        );
        log_info(
            b"recv\0" as *const u8 as *const libc::c_char,
            b"unsuccessful responses will be passed to the output module\0" as *const u8
                as *const libc::c_char,
        );
    }
    pthread_mutex_lock(recv_ready_mutex);
    zconf.recv_ready = 1 as libc::c_int;
    pthread_mutex_unlock(recv_ready_mutex);
    zrecv.start = now();
    if zconf.max_results == 0 as libc::c_int as libc::c_uint {
        zconf.max_results = -(1 as libc::c_int) as uint32_t;
    }
    loop {
        if zconf.dryrun != 0 {
            sleep(1 as libc::c_int as libc::c_uint);
        } else {
            recv_packets();
            if zconf.max_results != 0
                && zrecv.filter_success >= zconf.max_results as libc::c_ulong
            {
                break;
            }
        }
        if zsend.complete != 0
            && now() - zsend.finish > zconf.cooldown_secs as libc::c_double
        {
            break;
        }
    }
    zrecv.finish = now();
    recv_update_stats();
    if zconf.dryrun == 0 {
        pthread_mutex_lock(recv_ready_mutex);
        recv_cleanup();
        pthread_mutex_unlock(recv_ready_mutex);
    }
    zrecv.complete = 1 as libc::c_int;
    log_debug(
        b"recv\0" as *const u8 as *const libc::c_char,
        b"thread finished\0" as *const u8 as *const libc::c_char,
    );
    return 0 as libc::c_int;
}
