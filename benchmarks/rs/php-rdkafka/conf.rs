use ::libc;
extern "C" {
    pub type _zend_unserialize_data;
    pub type _zend_serialize_data;
    pub type rd_kafka_s;
    pub type rd_kafka_topic_s;
    pub type rd_kafka_conf_s;
    pub type rd_kafka_topic_conf_s;
    fn _emalloc(size: size_t) -> *mut libc::c_void;
    fn free(__ptr: *mut libc::c_void);
    fn memcpy(
        _: *mut libc::c_void,
        _: *const libc::c_void,
        _: libc::c_ulong,
    ) -> *mut libc::c_void;
    fn memset(
        _: *mut libc::c_void,
        _: libc::c_int,
        _: libc::c_ulong,
    ) -> *mut libc::c_void;
    fn strlen(_: *const libc::c_char) -> libc::c_ulong;
    fn _efree(ptr: *mut libc::c_void);
    fn _ecalloc(nmemb: size_t, size: size_t) -> *mut libc::c_void;
    fn _emalloc_8() -> *mut libc::c_void;
    fn _emalloc_16() -> *mut libc::c_void;
    fn _emalloc_24() -> *mut libc::c_void;
    fn _emalloc_32() -> *mut libc::c_void;
    fn _emalloc_40() -> *mut libc::c_void;
    fn _emalloc_48() -> *mut libc::c_void;
    fn _emalloc_56() -> *mut libc::c_void;
    fn _emalloc_64() -> *mut libc::c_void;
    fn _emalloc_80() -> *mut libc::c_void;
    fn _emalloc_96() -> *mut libc::c_void;
    fn _emalloc_112() -> *mut libc::c_void;
    fn _emalloc_128() -> *mut libc::c_void;
    fn _emalloc_160() -> *mut libc::c_void;
    fn _emalloc_192() -> *mut libc::c_void;
    fn _emalloc_224() -> *mut libc::c_void;
    fn _emalloc_256() -> *mut libc::c_void;
    fn _emalloc_320() -> *mut libc::c_void;
    fn _emalloc_384() -> *mut libc::c_void;
    fn _emalloc_448() -> *mut libc::c_void;
    fn _emalloc_512() -> *mut libc::c_void;
    fn _emalloc_640() -> *mut libc::c_void;
    fn _emalloc_768() -> *mut libc::c_void;
    fn _emalloc_896() -> *mut libc::c_void;
    fn _emalloc_1024() -> *mut libc::c_void;
    fn _emalloc_1280() -> *mut libc::c_void;
    fn _emalloc_1536() -> *mut libc::c_void;
    fn _emalloc_1792() -> *mut libc::c_void;
    fn _emalloc_2048() -> *mut libc::c_void;
    fn _emalloc_2560() -> *mut libc::c_void;
    fn _emalloc_3072() -> *mut libc::c_void;
    fn _emalloc_large(size: size_t) -> *mut libc::c_void;
    fn _emalloc_huge(size: size_t) -> *mut libc::c_void;
    fn __zend_malloc(len: size_t) -> *mut libc::c_void;
    static mut zend_string_init_interned: zend_string_init_interned_func_t;
    fn _zend_new_array_0() -> *mut HashTable;
    fn _zend_new_array(size: uint32_t) -> *mut HashTable;
    fn zend_array_dup(source: *mut HashTable) -> *mut HashTable;
    fn zval_ptr_dtor(zval_ptr: *mut zval);
    fn zend_replace_error_handling(
        error_handling: zend_error_handling_t,
        exception_class: *mut zend_class_entry,
        current: *mut zend_error_handling,
    );
    fn zend_restore_error_handling(saved: *mut zend_error_handling);
    fn zend_object_std_init(object: *mut zend_object, ce: *mut zend_class_entry);
    fn zend_object_std_dtor(object: *mut zend_object);
    fn zend_parse_parameters(
        num_args: libc::c_int,
        type_spec: *const libc::c_char,
        _: ...
    ) -> libc::c_int;
    fn zend_register_internal_class_ex(
        class_entry: *mut zend_class_entry,
        parent_ce: *mut zend_class_entry,
    ) -> *mut zend_class_entry;
    fn object_properties_init(
        object: *mut zend_object,
        class_type: *mut zend_class_entry,
    );
    fn add_assoc_string_ex(
        arg: *mut zval,
        key: *const libc::c_char,
        key_len: size_t,
        str: *const libc::c_char,
    ) -> libc::c_int;
    fn zend_call_function(
        fci: *mut zend_fcall_info,
        fci_cache: *mut zend_fcall_info_cache,
    ) -> libc::c_int;
    fn rd_kafka_err2str(err: rd_kafka_resp_err_t) -> *const libc::c_char;
    fn rd_kafka_conf_new() -> *mut rd_kafka_conf_t;
    fn rd_kafka_conf_destroy(conf: *mut rd_kafka_conf_t);
    fn rd_kafka_conf_set(
        conf: *mut rd_kafka_conf_t,
        name: *const libc::c_char,
        value: *const libc::c_char,
        errstr: *mut libc::c_char,
        errstr_size: size_t,
    ) -> rd_kafka_conf_res_t;
    fn rd_kafka_conf_set_dr_msg_cb(
        conf: *mut rd_kafka_conf_t,
        dr_msg_cb: Option::<
            unsafe extern "C" fn(
                *mut rd_kafka_t,
                *const rd_kafka_message_t,
                *mut libc::c_void,
            ) -> (),
        >,
    );
    fn rd_kafka_conf_set_consume_cb(
        conf: *mut rd_kafka_conf_t,
        consume_cb: Option::<
            unsafe extern "C" fn(*mut rd_kafka_message_t, *mut libc::c_void) -> (),
        >,
    );
    fn rd_kafka_conf_set_rebalance_cb(
        conf: *mut rd_kafka_conf_t,
        rebalance_cb: Option::<
            unsafe extern "C" fn(
                *mut rd_kafka_t,
                rd_kafka_resp_err_t,
                *mut rd_kafka_topic_partition_list_t,
                *mut libc::c_void,
            ) -> (),
        >,
    );
    fn rd_kafka_conf_set_offset_commit_cb(
        conf: *mut rd_kafka_conf_t,
        offset_commit_cb: Option::<
            unsafe extern "C" fn(
                *mut rd_kafka_t,
                rd_kafka_resp_err_t,
                *mut rd_kafka_topic_partition_list_t,
                *mut libc::c_void,
            ) -> (),
        >,
    );
    fn rd_kafka_conf_set_error_cb(
        conf: *mut rd_kafka_conf_t,
        error_cb: Option::<
            unsafe extern "C" fn(
                *mut rd_kafka_t,
                libc::c_int,
                *const libc::c_char,
                *mut libc::c_void,
            ) -> (),
        >,
    );
    fn rd_kafka_conf_set_log_cb(
        conf: *mut rd_kafka_conf_t,
        log_cb: Option::<
            unsafe extern "C" fn(
                *const rd_kafka_t,
                libc::c_int,
                *const libc::c_char,
                *const libc::c_char,
            ) -> (),
        >,
    );
    fn rd_kafka_conf_set_stats_cb(
        conf: *mut rd_kafka_conf_t,
        stats_cb: Option::<
            unsafe extern "C" fn(
                *mut rd_kafka_t,
                *mut libc::c_char,
                size_t,
                *mut libc::c_void,
            ) -> libc::c_int,
        >,
    );
    fn rd_kafka_conf_set_oauthbearer_token_refresh_cb(
        conf: *mut rd_kafka_conf_t,
        oauthbearer_token_refresh_cb: Option::<
            unsafe extern "C" fn(
                *mut rd_kafka_t,
                *const libc::c_char,
                *mut libc::c_void,
            ) -> (),
        >,
    );
    fn rd_kafka_opaque(rk: *const rd_kafka_t) -> *mut libc::c_void;
    fn rd_kafka_conf_set_default_topic_conf(
        conf: *mut rd_kafka_conf_t,
        tconf: *mut rd_kafka_topic_conf_t,
    );
    fn rd_kafka_conf_dump(
        conf: *mut rd_kafka_conf_t,
        cntp: *mut size_t,
    ) -> *mut *const libc::c_char;
    fn rd_kafka_topic_conf_dump(
        conf: *mut rd_kafka_topic_conf_t,
        cntp: *mut size_t,
    ) -> *mut *const libc::c_char;
    fn rd_kafka_conf_dump_free(arr: *mut *const libc::c_char, cnt: size_t);
    fn rd_kafka_topic_conf_new() -> *mut rd_kafka_topic_conf_t;
    fn rd_kafka_topic_conf_dup(
        conf: *const rd_kafka_topic_conf_t,
    ) -> *mut rd_kafka_topic_conf_t;
    fn rd_kafka_topic_conf_destroy(topic_conf: *mut rd_kafka_topic_conf_t);
    fn rd_kafka_topic_conf_set(
        conf: *mut rd_kafka_topic_conf_t,
        name: *const libc::c_char,
        value: *const libc::c_char,
        errstr: *mut libc::c_char,
        errstr_size: size_t,
    ) -> rd_kafka_conf_res_t;
    fn rd_kafka_topic_conf_set_partitioner_cb(
        topic_conf: *mut rd_kafka_topic_conf_t,
        partitioner: Option::<
            unsafe extern "C" fn(
                *const rd_kafka_topic_t,
                *const libc::c_void,
                size_t,
                int32_t,
                *mut libc::c_void,
                *mut libc::c_void,
            ) -> int32_t,
        >,
    );
    fn rd_kafka_msg_partitioner_random(
        rkt: *const rd_kafka_topic_t,
        key: *const libc::c_void,
        keylen: size_t,
        partition_cnt: int32_t,
        rkt_opaque: *mut libc::c_void,
        msg_opaque: *mut libc::c_void,
    ) -> int32_t;
    fn rd_kafka_msg_partitioner_consistent(
        rkt: *const rd_kafka_topic_t,
        key: *const libc::c_void,
        keylen: size_t,
        partition_cnt: int32_t,
        rkt_opaque: *mut libc::c_void,
        msg_opaque: *mut libc::c_void,
    ) -> int32_t;
    fn rd_kafka_msg_partitioner_consistent_random(
        rkt: *const rd_kafka_topic_t,
        key: *const libc::c_void,
        keylen: size_t,
        partition_cnt: int32_t,
        rkt_opaque: *mut libc::c_void,
        msg_opaque: *mut libc::c_void,
    ) -> int32_t;
    fn rd_kafka_msg_partitioner_murmur2(
        rkt: *const rd_kafka_topic_t,
        key: *const libc::c_void,
        keylen: size_t,
        partition_cnt: int32_t,
        rkt_opaque: *mut libc::c_void,
        msg_opaque: *mut libc::c_void,
    ) -> int32_t;
    fn rd_kafka_msg_partitioner_murmur2_random(
        rkt: *const rd_kafka_topic_t,
        key: *const libc::c_void,
        keylen: size_t,
        partition_cnt: int32_t,
        rkt_opaque: *mut libc::c_void,
        msg_opaque: *mut libc::c_void,
    ) -> int32_t;
    fn rd_kafka_assign(
        rk: *mut rd_kafka_t,
        partitions: *const rd_kafka_topic_partition_list_t,
    ) -> rd_kafka_resp_err_t;
    static mut kafka_default_object_handlers: zend_object_handlers;
    static mut ce_kafka_exception: *mut zend_class_entry;
    fn zend_throw_exception(
        exception_ce: *mut zend_class_entry,
        message: *const libc::c_char,
        code: zend_long,
    ) -> *mut zend_object;
    fn zend_throw_exception_ex(
        exception_ce: *mut zend_class_entry,
        code: zend_long,
        format: *const libc::c_char,
        _: ...
    ) -> *mut zend_object;
    static mut spl_ce_InvalidArgumentException: *mut zend_class_entry;
    fn kafka_topic_partition_list_to_array(
        return_value: *mut zval,
        list: *mut rd_kafka_topic_partition_list_t,
    );
    fn kafka_message_new(
        return_value: *mut zval,
        message: *const rd_kafka_message_t,
        msg_opaque: *mut zend_string,
    );
}
pub type size_t = libc::c_ulong;
pub type __uint8_t = libc::c_uchar;
pub type __uint16_t = libc::c_ushort;
pub type __int32_t = libc::c_int;
pub type __uint32_t = libc::c_uint;
pub type __int64_t = libc::c_long;
pub type __uint64_t = libc::c_ulong;
pub type int32_t = __int32_t;
pub type int64_t = __int64_t;
pub type uint8_t = __uint8_t;
pub type uint16_t = __uint16_t;
pub type uint32_t = __uint32_t;
pub type uint64_t = __uint64_t;
pub type uintptr_t = libc::c_ulong;
pub type zend_long = int64_t;
pub type zend_ulong = uint64_t;
pub type zend_bool = libc::c_uchar;
pub type zend_uchar = libc::c_uchar;
pub type C2RustUnnamed = libc::c_int;
pub const FAILURE: C2RustUnnamed = -1;
pub const SUCCESS: C2RustUnnamed = 0;
pub type zend_uintptr_t = uintptr_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _zend_object_handlers {
    pub offset: libc::c_int,
    pub free_obj: zend_object_free_obj_t,
    pub dtor_obj: zend_object_dtor_obj_t,
    pub clone_obj: zend_object_clone_obj_t,
    pub read_property: zend_object_read_property_t,
    pub write_property: zend_object_write_property_t,
    pub read_dimension: zend_object_read_dimension_t,
    pub write_dimension: zend_object_write_dimension_t,
    pub get_property_ptr_ptr: zend_object_get_property_ptr_ptr_t,
    pub get: zend_object_get_t,
    pub set: zend_object_set_t,
    pub has_property: zend_object_has_property_t,
    pub unset_property: zend_object_unset_property_t,
    pub has_dimension: zend_object_has_dimension_t,
    pub unset_dimension: zend_object_unset_dimension_t,
    pub get_properties: zend_object_get_properties_t,
    pub get_method: zend_object_get_method_t,
    pub call_method: zend_object_call_method_t,
    pub get_constructor: zend_object_get_constructor_t,
    pub get_class_name: zend_object_get_class_name_t,
    pub compare_objects: zend_object_compare_t,
    pub cast_object: zend_object_cast_t,
    pub count_elements: zend_object_count_elements_t,
    pub get_debug_info: zend_object_get_debug_info_t,
    pub get_closure: zend_object_get_closure_t,
    pub get_gc: zend_object_get_gc_t,
    pub do_operation: zend_object_do_operation_t,
    pub compare: zend_object_compare_zvals_t,
    pub get_properties_for: zend_object_get_properties_for_t,
}
pub type zend_object_get_properties_for_t = Option::<
    unsafe extern "C" fn(*mut zval, zend_prop_purpose) -> *mut zend_array,
>;
pub type zend_prop_purpose = _zend_prop_purpose;
pub type _zend_prop_purpose = libc::c_uint;
pub const _ZEND_PROP_PURPOSE_NON_EXHAUSTIVE_ENUM: _zend_prop_purpose = 6;
pub const _ZEND_PROP_PURPOSE_ARRAY_KEY_EXISTS: _zend_prop_purpose = 5;
pub const ZEND_PROP_PURPOSE_JSON: _zend_prop_purpose = 4;
pub const ZEND_PROP_PURPOSE_VAR_EXPORT: _zend_prop_purpose = 3;
pub const ZEND_PROP_PURPOSE_SERIALIZE: _zend_prop_purpose = 2;
pub const ZEND_PROP_PURPOSE_ARRAY_CAST: _zend_prop_purpose = 1;
pub const ZEND_PROP_PURPOSE_DEBUG: _zend_prop_purpose = 0;
pub type zval = _zval_struct;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _zval_struct {
    pub value: zend_value,
    pub u1: C2RustUnnamed_1,
    pub u2: C2RustUnnamed_0,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_0 {
    pub next: uint32_t,
    pub cache_slot: uint32_t,
    pub opline_num: uint32_t,
    pub lineno: uint32_t,
    pub num_args: uint32_t,
    pub fe_pos: uint32_t,
    pub fe_iter_idx: uint32_t,
    pub access_flags: uint32_t,
    pub property_guard: uint32_t,
    pub constant_flags: uint32_t,
    pub extra: uint32_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_1 {
    pub v: C2RustUnnamed_2,
    pub type_info: uint32_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_2 {
    pub type_0: zend_uchar,
    pub type_flags: zend_uchar,
    pub u: C2RustUnnamed_3,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_3 {
    pub extra: uint16_t,
}
pub type zend_value = _zend_value;
#[derive(Copy, Clone)]
#[repr(C)]
pub union _zend_value {
    pub lval: zend_long,
    pub dval: libc::c_double,
    pub counted: *mut zend_refcounted,
    pub str_0: *mut zend_string,
    pub arr: *mut zend_array,
    pub obj: *mut zend_object,
    pub res: *mut zend_resource,
    pub ref_0: *mut zend_reference,
    pub ast: *mut zend_ast_ref,
    pub zv: *mut zval,
    pub ptr: *mut libc::c_void,
    pub ce: *mut zend_class_entry,
    pub func: *mut zend_function,
    pub ww: C2RustUnnamed_4,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_4 {
    pub w1: uint32_t,
    pub w2: uint32_t,
}
pub type zend_function = _zend_function;
#[derive(Copy, Clone)]
#[repr(C)]
pub union _zend_function {
    pub type_0: zend_uchar,
    pub quick_arg_flags: uint32_t,
    pub common: C2RustUnnamed_14,
    pub op_array: zend_op_array,
    pub internal_function: zend_internal_function,
}
pub type zend_internal_function = _zend_internal_function;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _zend_internal_function {
    pub type_0: zend_uchar,
    pub arg_flags: [zend_uchar; 3],
    pub fn_flags: uint32_t,
    pub function_name: *mut zend_string,
    pub scope: *mut zend_class_entry,
    pub prototype: *mut zend_function,
    pub num_args: uint32_t,
    pub required_num_args: uint32_t,
    pub arg_info: *mut zend_internal_arg_info,
    pub handler: zif_handler,
    pub module: *mut _zend_module_entry,
    pub reserved: [*mut libc::c_void; 6],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _zend_module_entry {
    pub size: libc::c_ushort,
    pub zend_api: libc::c_uint,
    pub zend_debug: libc::c_uchar,
    pub zts: libc::c_uchar,
    pub ini_entry: *const _zend_ini_entry,
    pub deps: *const _zend_module_dep,
    pub name: *const libc::c_char,
    pub functions: *const _zend_function_entry,
    pub module_startup_func: Option::<
        unsafe extern "C" fn(libc::c_int, libc::c_int) -> libc::c_int,
    >,
    pub module_shutdown_func: Option::<
        unsafe extern "C" fn(libc::c_int, libc::c_int) -> libc::c_int,
    >,
    pub request_startup_func: Option::<
        unsafe extern "C" fn(libc::c_int, libc::c_int) -> libc::c_int,
    >,
    pub request_shutdown_func: Option::<
        unsafe extern "C" fn(libc::c_int, libc::c_int) -> libc::c_int,
    >,
    pub info_func: Option::<unsafe extern "C" fn(*mut zend_module_entry) -> ()>,
    pub version: *const libc::c_char,
    pub globals_size: size_t,
    pub globals_ptr: *mut libc::c_void,
    pub globals_ctor: Option::<unsafe extern "C" fn(*mut libc::c_void) -> ()>,
    pub globals_dtor: Option::<unsafe extern "C" fn(*mut libc::c_void) -> ()>,
    pub post_deactivate_func: Option::<unsafe extern "C" fn() -> libc::c_int>,
    pub module_started: libc::c_int,
    pub type_0: libc::c_uchar,
    pub handle: *mut libc::c_void,
    pub module_number: libc::c_int,
    pub build_id: *const libc::c_char,
}
pub type zend_module_entry = _zend_module_entry;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _zend_function_entry {
    pub fname: *const libc::c_char,
    pub handler: zif_handler,
    pub arg_info: *const _zend_internal_arg_info,
    pub num_args: uint32_t,
    pub flags: uint32_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _zend_internal_arg_info {
    pub name: *const libc::c_char,
    pub type_0: zend_type,
    pub pass_by_reference: zend_uchar,
    pub is_variadic: zend_bool,
}
pub type zend_type = uintptr_t;
pub type zif_handler = Option::<
    unsafe extern "C" fn(*mut zend_execute_data, *mut zval) -> (),
>;
pub type zend_execute_data = _zend_execute_data;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _zend_execute_data {
    pub opline: *const zend_op,
    pub call: *mut zend_execute_data,
    pub return_value: *mut zval,
    pub func: *mut zend_function,
    pub This: zval,
    pub prev_execute_data: *mut zend_execute_data,
    pub symbol_table: *mut zend_array,
    pub run_time_cache: *mut *mut libc::c_void,
}
pub type zend_array = _zend_array;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _zend_array {
    pub gc: zend_refcounted_h,
    pub u: C2RustUnnamed_6,
    pub nTableMask: uint32_t,
    pub arData: *mut Bucket,
    pub nNumUsed: uint32_t,
    pub nNumOfElements: uint32_t,
    pub nTableSize: uint32_t,
    pub nInternalPointer: uint32_t,
    pub nNextFreeElement: zend_long,
    pub pDestructor: dtor_func_t,
}
pub type dtor_func_t = Option::<unsafe extern "C" fn(*mut zval) -> ()>;
pub type Bucket = _Bucket;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _Bucket {
    pub val: zval,
    pub h: zend_ulong,
    pub key: *mut zend_string,
}
pub type zend_string = _zend_string;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _zend_string {
    pub gc: zend_refcounted_h,
    pub h: zend_ulong,
    pub len: size_t,
    pub val: [libc::c_char; 1],
}
pub type zend_refcounted_h = _zend_refcounted_h;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _zend_refcounted_h {
    pub refcount: uint32_t,
    pub u: C2RustUnnamed_5,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_5 {
    pub type_info: uint32_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_6 {
    pub v: C2RustUnnamed_7,
    pub flags: uint32_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_7 {
    pub flags: zend_uchar,
    pub _unused: zend_uchar,
    pub nIteratorsCount: zend_uchar,
    pub _unused2: zend_uchar,
}
pub type zend_op = _zend_op;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _zend_op {
    pub handler: *const libc::c_void,
    pub op1: znode_op,
    pub op2: znode_op,
    pub result: znode_op,
    pub extended_value: uint32_t,
    pub lineno: uint32_t,
    pub opcode: zend_uchar,
    pub op1_type: zend_uchar,
    pub op2_type: zend_uchar,
    pub result_type: zend_uchar,
}
pub type znode_op = _znode_op;
#[derive(Copy, Clone)]
#[repr(C)]
pub union _znode_op {
    pub constant: uint32_t,
    pub var: uint32_t,
    pub num: uint32_t,
    pub opline_num: uint32_t,
    pub jmp_offset: uint32_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _zend_module_dep {
    pub name: *const libc::c_char,
    pub rel: *const libc::c_char,
    pub version: *const libc::c_char,
    pub type_0: libc::c_uchar,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _zend_ini_entry {
    pub name: *mut zend_string,
    pub on_modify: Option::<
        unsafe extern "C" fn(
            *mut zend_ini_entry,
            *mut zend_string,
            *mut libc::c_void,
            *mut libc::c_void,
            *mut libc::c_void,
            libc::c_int,
        ) -> libc::c_int,
    >,
    pub mh_arg1: *mut libc::c_void,
    pub mh_arg2: *mut libc::c_void,
    pub mh_arg3: *mut libc::c_void,
    pub value: *mut zend_string,
    pub orig_value: *mut zend_string,
    pub displayer: Option::<
        unsafe extern "C" fn(*mut zend_ini_entry, libc::c_int) -> (),
    >,
    pub module_number: libc::c_int,
    pub modifiable: uint8_t,
    pub orig_modifiable: uint8_t,
    pub modified: uint8_t,
}
pub type zend_ini_entry = _zend_ini_entry;
pub type zend_internal_arg_info = _zend_internal_arg_info;
pub type zend_class_entry = _zend_class_entry;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _zend_class_entry {
    pub type_0: libc::c_char,
    pub name: *mut zend_string,
    pub c2rust_unnamed: C2RustUnnamed_13,
    pub refcount: libc::c_int,
    pub ce_flags: uint32_t,
    pub default_properties_count: libc::c_int,
    pub default_static_members_count: libc::c_int,
    pub default_properties_table: *mut zval,
    pub default_static_members_table: *mut zval,
    pub static_members_table__ptr: *mut *mut zval,
    pub function_table: HashTable,
    pub properties_info: HashTable,
    pub constants_table: HashTable,
    pub properties_info_table: *mut *mut _zend_property_info,
    pub constructor: *mut zend_function,
    pub destructor: *mut zend_function,
    pub clone: *mut zend_function,
    pub __get: *mut zend_function,
    pub __set: *mut zend_function,
    pub __unset: *mut zend_function,
    pub __isset: *mut zend_function,
    pub __call: *mut zend_function,
    pub __callstatic: *mut zend_function,
    pub __tostring: *mut zend_function,
    pub __debugInfo: *mut zend_function,
    pub serialize_func: *mut zend_function,
    pub unserialize_func: *mut zend_function,
    pub iterator_funcs_ptr: *mut zend_class_iterator_funcs,
    pub c2rust_unnamed_0: C2RustUnnamed_12,
    pub get_iterator: Option::<
        unsafe extern "C" fn(
            *mut zend_class_entry,
            *mut zval,
            libc::c_int,
        ) -> *mut zend_object_iterator,
    >,
    pub get_static_method: Option::<
        unsafe extern "C" fn(
            *mut zend_class_entry,
            *mut zend_string,
        ) -> *mut zend_function,
    >,
    pub serialize: Option::<
        unsafe extern "C" fn(
            *mut zval,
            *mut *mut libc::c_uchar,
            *mut size_t,
            *mut zend_serialize_data,
        ) -> libc::c_int,
    >,
    pub unserialize: Option::<
        unsafe extern "C" fn(
            *mut zval,
            *mut zend_class_entry,
            *const libc::c_uchar,
            size_t,
            *mut zend_unserialize_data,
        ) -> libc::c_int,
    >,
    pub num_interfaces: uint32_t,
    pub num_traits: uint32_t,
    pub c2rust_unnamed_1: C2RustUnnamed_11,
    pub trait_names: *mut zend_class_name,
    pub trait_aliases: *mut *mut zend_trait_alias,
    pub trait_precedences: *mut *mut zend_trait_precedence,
    pub info: C2RustUnnamed_8,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_8 {
    pub user: C2RustUnnamed_10,
    pub internal: C2RustUnnamed_9,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_9 {
    pub builtin_functions: *const _zend_function_entry,
    pub module: *mut _zend_module_entry,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_10 {
    pub filename: *mut zend_string,
    pub line_start: uint32_t,
    pub line_end: uint32_t,
    pub doc_comment: *mut zend_string,
}
pub type zend_trait_precedence = _zend_trait_precedence;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _zend_trait_precedence {
    pub trait_method: zend_trait_method_reference,
    pub num_excludes: uint32_t,
    pub exclude_class_names: [*mut zend_string; 1],
}
pub type zend_trait_method_reference = _zend_trait_method_reference;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _zend_trait_method_reference {
    pub method_name: *mut zend_string,
    pub class_name: *mut zend_string,
}
pub type zend_trait_alias = _zend_trait_alias;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _zend_trait_alias {
    pub trait_method: zend_trait_method_reference,
    pub alias: *mut zend_string,
    pub modifiers: uint32_t,
}
pub type zend_class_name = _zend_class_name;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _zend_class_name {
    pub name: *mut zend_string,
    pub lc_name: *mut zend_string,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_11 {
    pub interfaces: *mut *mut zend_class_entry,
    pub interface_names: *mut zend_class_name,
}
pub type zend_unserialize_data = _zend_unserialize_data;
pub type zend_serialize_data = _zend_serialize_data;
pub type zend_object_iterator = _zend_object_iterator;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _zend_object_iterator {
    pub std: zend_object,
    pub data: zval,
    pub funcs: *const zend_object_iterator_funcs,
    pub index: zend_ulong,
}
pub type zend_object_iterator_funcs = _zend_object_iterator_funcs;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _zend_object_iterator_funcs {
    pub dtor: Option::<unsafe extern "C" fn(*mut zend_object_iterator) -> ()>,
    pub valid: Option::<unsafe extern "C" fn(*mut zend_object_iterator) -> libc::c_int>,
    pub get_current_data: Option::<
        unsafe extern "C" fn(*mut zend_object_iterator) -> *mut zval,
    >,
    pub get_current_key: Option::<
        unsafe extern "C" fn(*mut zend_object_iterator, *mut zval) -> (),
    >,
    pub move_forward: Option::<unsafe extern "C" fn(*mut zend_object_iterator) -> ()>,
    pub rewind: Option::<unsafe extern "C" fn(*mut zend_object_iterator) -> ()>,
    pub invalidate_current: Option::<
        unsafe extern "C" fn(*mut zend_object_iterator) -> (),
    >,
}
pub type zend_object = _zend_object;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _zend_object {
    pub gc: zend_refcounted_h,
    pub handle: uint32_t,
    pub ce: *mut zend_class_entry,
    pub handlers: *const zend_object_handlers,
    pub properties: *mut HashTable,
    pub properties_table: [zval; 1],
}
pub type HashTable = _zend_array;
pub type zend_object_handlers = _zend_object_handlers;
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_12 {
    pub create_object: Option::<
        unsafe extern "C" fn(*mut zend_class_entry) -> *mut zend_object,
    >,
    pub interface_gets_implemented: Option::<
        unsafe extern "C" fn(*mut zend_class_entry, *mut zend_class_entry) -> libc::c_int,
    >,
}
pub type zend_class_iterator_funcs = _zend_class_iterator_funcs;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _zend_class_iterator_funcs {
    pub zf_new_iterator: *mut zend_function,
    pub zf_valid: *mut zend_function,
    pub zf_current: *mut zend_function,
    pub zf_key: *mut zend_function,
    pub zf_next: *mut zend_function,
    pub zf_rewind: *mut zend_function,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _zend_property_info {
    pub offset: uint32_t,
    pub flags: uint32_t,
    pub name: *mut zend_string,
    pub doc_comment: *mut zend_string,
    pub ce: *mut zend_class_entry,
    pub type_0: zend_type,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_13 {
    pub parent: *mut zend_class_entry,
    pub parent_name: *mut zend_string,
}
pub type zend_op_array = _zend_op_array;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _zend_op_array {
    pub type_0: zend_uchar,
    pub arg_flags: [zend_uchar; 3],
    pub fn_flags: uint32_t,
    pub function_name: *mut zend_string,
    pub scope: *mut zend_class_entry,
    pub prototype: *mut zend_function,
    pub num_args: uint32_t,
    pub required_num_args: uint32_t,
    pub arg_info: *mut zend_arg_info,
    pub cache_size: libc::c_int,
    pub last_var: libc::c_int,
    pub T: uint32_t,
    pub last: uint32_t,
    pub opcodes: *mut zend_op,
    pub run_time_cache__ptr: *mut *mut *mut libc::c_void,
    pub static_variables_ptr__ptr: *mut *mut HashTable,
    pub static_variables: *mut HashTable,
    pub vars: *mut *mut zend_string,
    pub refcount: *mut uint32_t,
    pub last_live_range: libc::c_int,
    pub last_try_catch: libc::c_int,
    pub live_range: *mut zend_live_range,
    pub try_catch_array: *mut zend_try_catch_element,
    pub filename: *mut zend_string,
    pub line_start: uint32_t,
    pub line_end: uint32_t,
    pub doc_comment: *mut zend_string,
    pub last_literal: libc::c_int,
    pub literals: *mut zval,
    pub reserved: [*mut libc::c_void; 6],
}
pub type zend_try_catch_element = _zend_try_catch_element;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _zend_try_catch_element {
    pub try_op: uint32_t,
    pub catch_op: uint32_t,
    pub finally_op: uint32_t,
    pub finally_end: uint32_t,
}
pub type zend_live_range = _zend_live_range;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _zend_live_range {
    pub var: uint32_t,
    pub start: uint32_t,
    pub end: uint32_t,
}
pub type zend_arg_info = _zend_arg_info;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _zend_arg_info {
    pub name: *mut zend_string,
    pub type_0: zend_type,
    pub pass_by_reference: zend_uchar,
    pub is_variadic: zend_bool,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_14 {
    pub type_0: zend_uchar,
    pub arg_flags: [zend_uchar; 3],
    pub fn_flags: uint32_t,
    pub function_name: *mut zend_string,
    pub scope: *mut zend_class_entry,
    pub prototype: *mut zend_function,
    pub num_args: uint32_t,
    pub required_num_args: uint32_t,
    pub arg_info: *mut zend_arg_info,
}
pub type zend_ast_ref = _zend_ast_ref;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _zend_ast_ref {
    pub gc: zend_refcounted_h,
}
pub type zend_reference = _zend_reference;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _zend_reference {
    pub gc: zend_refcounted_h,
    pub val: zval,
    pub sources: zend_property_info_source_list,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union zend_property_info_source_list {
    pub ptr: *mut _zend_property_info,
    pub list: uintptr_t,
}
pub type zend_resource = _zend_resource;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _zend_resource {
    pub gc: zend_refcounted_h,
    pub handle: libc::c_int,
    pub type_0: libc::c_int,
    pub ptr: *mut libc::c_void,
}
pub type zend_refcounted = _zend_refcounted;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _zend_refcounted {
    pub gc: zend_refcounted_h,
}
pub type zend_object_compare_zvals_t = Option::<
    unsafe extern "C" fn(*mut zval, *mut zval, *mut zval) -> libc::c_int,
>;
pub type zend_object_do_operation_t = Option::<
    unsafe extern "C" fn(zend_uchar, *mut zval, *mut zval, *mut zval) -> libc::c_int,
>;
pub type zend_object_get_gc_t = Option::<
    unsafe extern "C" fn(*mut zval, *mut *mut zval, *mut libc::c_int) -> *mut HashTable,
>;
pub type zend_object_get_closure_t = Option::<
    unsafe extern "C" fn(
        *mut zval,
        *mut *mut zend_class_entry,
        *mut *mut zend_function,
        *mut *mut zend_object,
    ) -> libc::c_int,
>;
pub type zend_object_get_debug_info_t = Option::<
    unsafe extern "C" fn(*mut zval, *mut libc::c_int) -> *mut HashTable,
>;
pub type zend_object_count_elements_t = Option::<
    unsafe extern "C" fn(*mut zval, *mut zend_long) -> libc::c_int,
>;
pub type zend_object_cast_t = Option::<
    unsafe extern "C" fn(*mut zval, *mut zval, libc::c_int) -> libc::c_int,
>;
pub type zend_object_compare_t = Option::<
    unsafe extern "C" fn(*mut zval, *mut zval) -> libc::c_int,
>;
pub type zend_object_get_class_name_t = Option::<
    unsafe extern "C" fn(*const zend_object) -> *mut zend_string,
>;
pub type zend_object_get_constructor_t = Option::<
    unsafe extern "C" fn(*mut zend_object) -> *mut zend_function,
>;
pub type zend_object_call_method_t = Option::<
    unsafe extern "C" fn(
        *mut zend_string,
        *mut zend_object,
        *mut zend_execute_data,
        *mut zval,
    ) -> libc::c_int,
>;
pub type zend_object_get_method_t = Option::<
    unsafe extern "C" fn(
        *mut *mut zend_object,
        *mut zend_string,
        *const zval,
    ) -> *mut zend_function,
>;
pub type zend_object_get_properties_t = Option::<
    unsafe extern "C" fn(*mut zval) -> *mut HashTable,
>;
pub type zend_object_unset_dimension_t = Option::<
    unsafe extern "C" fn(*mut zval, *mut zval) -> (),
>;
pub type zend_object_has_dimension_t = Option::<
    unsafe extern "C" fn(*mut zval, *mut zval, libc::c_int) -> libc::c_int,
>;
pub type zend_object_unset_property_t = Option::<
    unsafe extern "C" fn(*mut zval, *mut zval, *mut *mut libc::c_void) -> (),
>;
pub type zend_object_has_property_t = Option::<
    unsafe extern "C" fn(
        *mut zval,
        *mut zval,
        libc::c_int,
        *mut *mut libc::c_void,
    ) -> libc::c_int,
>;
pub type zend_object_set_t = Option::<unsafe extern "C" fn(*mut zval, *mut zval) -> ()>;
pub type zend_object_get_t = Option::<
    unsafe extern "C" fn(*mut zval, *mut zval) -> *mut zval,
>;
pub type zend_object_get_property_ptr_ptr_t = Option::<
    unsafe extern "C" fn(
        *mut zval,
        *mut zval,
        libc::c_int,
        *mut *mut libc::c_void,
    ) -> *mut zval,
>;
pub type zend_object_write_dimension_t = Option::<
    unsafe extern "C" fn(*mut zval, *mut zval, *mut zval) -> (),
>;
pub type zend_object_read_dimension_t = Option::<
    unsafe extern "C" fn(*mut zval, *mut zval, libc::c_int, *mut zval) -> *mut zval,
>;
pub type zend_object_write_property_t = Option::<
    unsafe extern "C" fn(
        *mut zval,
        *mut zval,
        *mut zval,
        *mut *mut libc::c_void,
    ) -> *mut zval,
>;
pub type zend_object_read_property_t = Option::<
    unsafe extern "C" fn(
        *mut zval,
        *mut zval,
        libc::c_int,
        *mut *mut libc::c_void,
        *mut zval,
    ) -> *mut zval,
>;
pub type zend_object_clone_obj_t = Option::<
    unsafe extern "C" fn(*mut zval) -> *mut zend_object,
>;
pub type zend_object_dtor_obj_t = Option::<unsafe extern "C" fn(*mut zend_object) -> ()>;
pub type zend_object_free_obj_t = Option::<unsafe extern "C" fn(*mut zend_object) -> ()>;
pub type zend_string_init_interned_func_t = Option::<
    unsafe extern "C" fn(*const libc::c_char, size_t, libc::c_int) -> *mut zend_string,
>;
pub type zend_error_handling_t = libc::c_uint;
pub const EH_THROW: zend_error_handling_t = 1;
pub const EH_NORMAL: zend_error_handling_t = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct zend_error_handling {
    pub handling: zend_error_handling_t,
    pub exception: *mut zend_class_entry,
    pub user_handler: zval,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _zend_fcall_info {
    pub size: size_t,
    pub function_name: zval,
    pub retval: *mut zval,
    pub params: *mut zval,
    pub object: *mut zend_object,
    pub no_separation: zend_bool,
    pub param_count: uint32_t,
}
pub type zend_function_entry = _zend_function_entry;
pub type zend_fcall_info = _zend_fcall_info;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _zend_fcall_info_cache {
    pub function_handler: *mut zend_function,
    pub calling_scope: *mut zend_class_entry,
    pub called_scope: *mut zend_class_entry,
    pub object: *mut zend_object,
}
pub type zend_fcall_info_cache = _zend_fcall_info_cache;
pub type rd_kafka_t = rd_kafka_s;
pub type rd_kafka_topic_t = rd_kafka_topic_s;
pub type rd_kafka_conf_t = rd_kafka_conf_s;
pub type rd_kafka_topic_conf_t = rd_kafka_topic_conf_s;
pub type rd_kafka_resp_err_t = libc::c_int;
pub const RD_KAFKA_RESP_ERR_END_ALL: rd_kafka_resp_err_t = 82;
pub const RD_KAFKA_RESP_ERR_GROUP_MAX_SIZE_REACHED: rd_kafka_resp_err_t = 81;
pub const RD_KAFKA_RESP_ERR_PREFERRED_LEADER_NOT_AVAILABLE: rd_kafka_resp_err_t = 80;
pub const RD_KAFKA_RESP_ERR_MEMBER_ID_REQUIRED: rd_kafka_resp_err_t = 79;
pub const RD_KAFKA_RESP_ERR_OFFSET_NOT_AVAILABLE: rd_kafka_resp_err_t = 78;
pub const RD_KAFKA_RESP_ERR_STALE_BROKER_EPOCH: rd_kafka_resp_err_t = 77;
pub const RD_KAFKA_RESP_ERR_UNSUPPORTED_COMPRESSION_TYPE: rd_kafka_resp_err_t = 76;
pub const RD_KAFKA_RESP_ERR_UNKNOWN_LEADER_EPOCH: rd_kafka_resp_err_t = 75;
pub const RD_KAFKA_RESP_ERR_FENCED_LEADER_EPOCH: rd_kafka_resp_err_t = 74;
pub const RD_KAFKA_RESP_ERR_TOPIC_DELETION_DISABLED: rd_kafka_resp_err_t = 73;
pub const RD_KAFKA_RESP_ERR_LISTENER_NOT_FOUND: rd_kafka_resp_err_t = 72;
pub const RD_KAFKA_RESP_ERR_INVALID_FETCH_SESSION_EPOCH: rd_kafka_resp_err_t = 71;
pub const RD_KAFKA_RESP_ERR_FETCH_SESSION_ID_NOT_FOUND: rd_kafka_resp_err_t = 70;
pub const RD_KAFKA_RESP_ERR_GROUP_ID_NOT_FOUND: rd_kafka_resp_err_t = 69;
pub const RD_KAFKA_RESP_ERR_NON_EMPTY_GROUP: rd_kafka_resp_err_t = 68;
pub const RD_KAFKA_RESP_ERR_INVALID_PRINCIPAL_TYPE: rd_kafka_resp_err_t = 67;
pub const RD_KAFKA_RESP_ERR_DELEGATION_TOKEN_EXPIRED: rd_kafka_resp_err_t = 66;
pub const RD_KAFKA_RESP_ERR_DELEGATION_TOKEN_AUTHORIZATION_FAILED: rd_kafka_resp_err_t = 65;
pub const RD_KAFKA_RESP_ERR_DELEGATION_TOKEN_REQUEST_NOT_ALLOWED: rd_kafka_resp_err_t = 64;
pub const RD_KAFKA_RESP_ERR_DELEGATION_TOKEN_OWNER_MISMATCH: rd_kafka_resp_err_t = 63;
pub const RD_KAFKA_RESP_ERR_DELEGATION_TOKEN_NOT_FOUND: rd_kafka_resp_err_t = 62;
pub const RD_KAFKA_RESP_ERR_DELEGATION_TOKEN_AUTH_DISABLED: rd_kafka_resp_err_t = 61;
pub const RD_KAFKA_RESP_ERR_REASSIGNMENT_IN_PROGRESS: rd_kafka_resp_err_t = 60;
pub const RD_KAFKA_RESP_ERR_UNKNOWN_PRODUCER_ID: rd_kafka_resp_err_t = 59;
pub const RD_KAFKA_RESP_ERR_SASL_AUTHENTICATION_FAILED: rd_kafka_resp_err_t = 58;
pub const RD_KAFKA_RESP_ERR_LOG_DIR_NOT_FOUND: rd_kafka_resp_err_t = 57;
pub const RD_KAFKA_RESP_ERR_KAFKA_STORAGE_ERROR: rd_kafka_resp_err_t = 56;
pub const RD_KAFKA_RESP_ERR_OPERATION_NOT_ATTEMPTED: rd_kafka_resp_err_t = 55;
pub const RD_KAFKA_RESP_ERR_SECURITY_DISABLED: rd_kafka_resp_err_t = 54;
pub const RD_KAFKA_RESP_ERR_TRANSACTIONAL_ID_AUTHORIZATION_FAILED: rd_kafka_resp_err_t = 53;
pub const RD_KAFKA_RESP_ERR_TRANSACTION_COORDINATOR_FENCED: rd_kafka_resp_err_t = 52;
pub const RD_KAFKA_RESP_ERR_CONCURRENT_TRANSACTIONS: rd_kafka_resp_err_t = 51;
pub const RD_KAFKA_RESP_ERR_INVALID_TRANSACTION_TIMEOUT: rd_kafka_resp_err_t = 50;
pub const RD_KAFKA_RESP_ERR_INVALID_PRODUCER_ID_MAPPING: rd_kafka_resp_err_t = 49;
pub const RD_KAFKA_RESP_ERR_INVALID_TXN_STATE: rd_kafka_resp_err_t = 48;
pub const RD_KAFKA_RESP_ERR_INVALID_PRODUCER_EPOCH: rd_kafka_resp_err_t = 47;
pub const RD_KAFKA_RESP_ERR_DUPLICATE_SEQUENCE_NUMBER: rd_kafka_resp_err_t = 46;
pub const RD_KAFKA_RESP_ERR_OUT_OF_ORDER_SEQUENCE_NUMBER: rd_kafka_resp_err_t = 45;
pub const RD_KAFKA_RESP_ERR_POLICY_VIOLATION: rd_kafka_resp_err_t = 44;
pub const RD_KAFKA_RESP_ERR_UNSUPPORTED_FOR_MESSAGE_FORMAT: rd_kafka_resp_err_t = 43;
pub const RD_KAFKA_RESP_ERR_INVALID_REQUEST: rd_kafka_resp_err_t = 42;
pub const RD_KAFKA_RESP_ERR_NOT_CONTROLLER: rd_kafka_resp_err_t = 41;
pub const RD_KAFKA_RESP_ERR_INVALID_CONFIG: rd_kafka_resp_err_t = 40;
pub const RD_KAFKA_RESP_ERR_INVALID_REPLICA_ASSIGNMENT: rd_kafka_resp_err_t = 39;
pub const RD_KAFKA_RESP_ERR_INVALID_REPLICATION_FACTOR: rd_kafka_resp_err_t = 38;
pub const RD_KAFKA_RESP_ERR_INVALID_PARTITIONS: rd_kafka_resp_err_t = 37;
pub const RD_KAFKA_RESP_ERR_TOPIC_ALREADY_EXISTS: rd_kafka_resp_err_t = 36;
pub const RD_KAFKA_RESP_ERR_UNSUPPORTED_VERSION: rd_kafka_resp_err_t = 35;
pub const RD_KAFKA_RESP_ERR_ILLEGAL_SASL_STATE: rd_kafka_resp_err_t = 34;
pub const RD_KAFKA_RESP_ERR_UNSUPPORTED_SASL_MECHANISM: rd_kafka_resp_err_t = 33;
pub const RD_KAFKA_RESP_ERR_INVALID_TIMESTAMP: rd_kafka_resp_err_t = 32;
pub const RD_KAFKA_RESP_ERR_CLUSTER_AUTHORIZATION_FAILED: rd_kafka_resp_err_t = 31;
pub const RD_KAFKA_RESP_ERR_GROUP_AUTHORIZATION_FAILED: rd_kafka_resp_err_t = 30;
pub const RD_KAFKA_RESP_ERR_TOPIC_AUTHORIZATION_FAILED: rd_kafka_resp_err_t = 29;
pub const RD_KAFKA_RESP_ERR_INVALID_COMMIT_OFFSET_SIZE: rd_kafka_resp_err_t = 28;
pub const RD_KAFKA_RESP_ERR_REBALANCE_IN_PROGRESS: rd_kafka_resp_err_t = 27;
pub const RD_KAFKA_RESP_ERR_INVALID_SESSION_TIMEOUT: rd_kafka_resp_err_t = 26;
pub const RD_KAFKA_RESP_ERR_UNKNOWN_MEMBER_ID: rd_kafka_resp_err_t = 25;
pub const RD_KAFKA_RESP_ERR_INVALID_GROUP_ID: rd_kafka_resp_err_t = 24;
pub const RD_KAFKA_RESP_ERR_INCONSISTENT_GROUP_PROTOCOL: rd_kafka_resp_err_t = 23;
pub const RD_KAFKA_RESP_ERR_ILLEGAL_GENERATION: rd_kafka_resp_err_t = 22;
pub const RD_KAFKA_RESP_ERR_INVALID_REQUIRED_ACKS: rd_kafka_resp_err_t = 21;
pub const RD_KAFKA_RESP_ERR_NOT_ENOUGH_REPLICAS_AFTER_APPEND: rd_kafka_resp_err_t = 20;
pub const RD_KAFKA_RESP_ERR_NOT_ENOUGH_REPLICAS: rd_kafka_resp_err_t = 19;
pub const RD_KAFKA_RESP_ERR_RECORD_LIST_TOO_LARGE: rd_kafka_resp_err_t = 18;
pub const RD_KAFKA_RESP_ERR_TOPIC_EXCEPTION: rd_kafka_resp_err_t = 17;
pub const RD_KAFKA_RESP_ERR_NOT_COORDINATOR_FOR_GROUP: rd_kafka_resp_err_t = 16;
pub const RD_KAFKA_RESP_ERR_GROUP_COORDINATOR_NOT_AVAILABLE: rd_kafka_resp_err_t = 15;
pub const RD_KAFKA_RESP_ERR_GROUP_LOAD_IN_PROGRESS: rd_kafka_resp_err_t = 14;
pub const RD_KAFKA_RESP_ERR_NETWORK_EXCEPTION: rd_kafka_resp_err_t = 13;
pub const RD_KAFKA_RESP_ERR_OFFSET_METADATA_TOO_LARGE: rd_kafka_resp_err_t = 12;
pub const RD_KAFKA_RESP_ERR_STALE_CTRL_EPOCH: rd_kafka_resp_err_t = 11;
pub const RD_KAFKA_RESP_ERR_MSG_SIZE_TOO_LARGE: rd_kafka_resp_err_t = 10;
pub const RD_KAFKA_RESP_ERR_REPLICA_NOT_AVAILABLE: rd_kafka_resp_err_t = 9;
pub const RD_KAFKA_RESP_ERR_BROKER_NOT_AVAILABLE: rd_kafka_resp_err_t = 8;
pub const RD_KAFKA_RESP_ERR_REQUEST_TIMED_OUT: rd_kafka_resp_err_t = 7;
pub const RD_KAFKA_RESP_ERR_NOT_LEADER_FOR_PARTITION: rd_kafka_resp_err_t = 6;
pub const RD_KAFKA_RESP_ERR_LEADER_NOT_AVAILABLE: rd_kafka_resp_err_t = 5;
pub const RD_KAFKA_RESP_ERR_INVALID_MSG_SIZE: rd_kafka_resp_err_t = 4;
pub const RD_KAFKA_RESP_ERR_UNKNOWN_TOPIC_OR_PART: rd_kafka_resp_err_t = 3;
pub const RD_KAFKA_RESP_ERR_INVALID_MSG: rd_kafka_resp_err_t = 2;
pub const RD_KAFKA_RESP_ERR_OFFSET_OUT_OF_RANGE: rd_kafka_resp_err_t = 1;
pub const RD_KAFKA_RESP_ERR_NO_ERROR: rd_kafka_resp_err_t = 0;
pub const RD_KAFKA_RESP_ERR_UNKNOWN: rd_kafka_resp_err_t = -1;
pub const RD_KAFKA_RESP_ERR__END: rd_kafka_resp_err_t = -100;
pub const RD_KAFKA_RESP_ERR__MAX_POLL_EXCEEDED: rd_kafka_resp_err_t = -147;
pub const RD_KAFKA_RESP_ERR__GAPLESS_GUARANTEE: rd_kafka_resp_err_t = -148;
pub const RD_KAFKA_RESP_ERR__INCONSISTENT: rd_kafka_resp_err_t = -149;
pub const RD_KAFKA_RESP_ERR__FATAL: rd_kafka_resp_err_t = -150;
pub const RD_KAFKA_RESP_ERR__PURGE_INFLIGHT: rd_kafka_resp_err_t = -151;
pub const RD_KAFKA_RESP_ERR__PURGE_QUEUE: rd_kafka_resp_err_t = -152;
pub const RD_KAFKA_RESP_ERR__RETRY: rd_kafka_resp_err_t = -153;
pub const RD_KAFKA_RESP_ERR__INVALID_TYPE: rd_kafka_resp_err_t = -154;
pub const RD_KAFKA_RESP_ERR__UNDERFLOW: rd_kafka_resp_err_t = -155;
pub const RD_KAFKA_RESP_ERR__NOENT: rd_kafka_resp_err_t = -156;
pub const RD_KAFKA_RESP_ERR__READ_ONLY: rd_kafka_resp_err_t = -157;
pub const RD_KAFKA_RESP_ERR__PARTIAL: rd_kafka_resp_err_t = -158;
pub const RD_KAFKA_RESP_ERR__VALUE_DESERIALIZATION: rd_kafka_resp_err_t = -159;
pub const RD_KAFKA_RESP_ERR__KEY_DESERIALIZATION: rd_kafka_resp_err_t = -160;
pub const RD_KAFKA_RESP_ERR__VALUE_SERIALIZATION: rd_kafka_resp_err_t = -161;
pub const RD_KAFKA_RESP_ERR__KEY_SERIALIZATION: rd_kafka_resp_err_t = -162;
pub const RD_KAFKA_RESP_ERR__INTR: rd_kafka_resp_err_t = -163;
pub const RD_KAFKA_RESP_ERR__WAIT_CACHE: rd_kafka_resp_err_t = -164;
pub const RD_KAFKA_RESP_ERR__UNSUPPORTED_FEATURE: rd_kafka_resp_err_t = -165;
pub const RD_KAFKA_RESP_ERR__TIMED_OUT_QUEUE: rd_kafka_resp_err_t = -166;
pub const RD_KAFKA_RESP_ERR__OUTDATED: rd_kafka_resp_err_t = -167;
pub const RD_KAFKA_RESP_ERR__NO_OFFSET: rd_kafka_resp_err_t = -168;
pub const RD_KAFKA_RESP_ERR__AUTHENTICATION: rd_kafka_resp_err_t = -169;
pub const RD_KAFKA_RESP_ERR__NOT_IMPLEMENTED: rd_kafka_resp_err_t = -170;
pub const RD_KAFKA_RESP_ERR__UNKNOWN_PROTOCOL: rd_kafka_resp_err_t = -171;
pub const RD_KAFKA_RESP_ERR__STATE: rd_kafka_resp_err_t = -172;
pub const RD_KAFKA_RESP_ERR__CONFLICT: rd_kafka_resp_err_t = -173;
pub const RD_KAFKA_RESP_ERR__REVOKE_PARTITIONS: rd_kafka_resp_err_t = -174;
pub const RD_KAFKA_RESP_ERR__ASSIGN_PARTITIONS: rd_kafka_resp_err_t = -175;
pub const RD_KAFKA_RESP_ERR__EXISTING_SUBSCRIPTION: rd_kafka_resp_err_t = -176;
pub const RD_KAFKA_RESP_ERR__PREV_IN_PROGRESS: rd_kafka_resp_err_t = -177;
pub const RD_KAFKA_RESP_ERR__IN_PROGRESS: rd_kafka_resp_err_t = -178;
pub const RD_KAFKA_RESP_ERR__UNKNOWN_GROUP: rd_kafka_resp_err_t = -179;
pub const RD_KAFKA_RESP_ERR__WAIT_COORD: rd_kafka_resp_err_t = -180;
pub const RD_KAFKA_RESP_ERR__SSL: rd_kafka_resp_err_t = -181;
pub const RD_KAFKA_RESP_ERR__NODE_UPDATE: rd_kafka_resp_err_t = -182;
pub const RD_KAFKA_RESP_ERR__ISR_INSUFF: rd_kafka_resp_err_t = -183;
pub const RD_KAFKA_RESP_ERR__QUEUE_FULL: rd_kafka_resp_err_t = -184;
pub const RD_KAFKA_RESP_ERR__TIMED_OUT: rd_kafka_resp_err_t = -185;
pub const RD_KAFKA_RESP_ERR__INVALID_ARG: rd_kafka_resp_err_t = -186;
pub const RD_KAFKA_RESP_ERR__ALL_BROKERS_DOWN: rd_kafka_resp_err_t = -187;
pub const RD_KAFKA_RESP_ERR__UNKNOWN_TOPIC: rd_kafka_resp_err_t = -188;
pub const RD_KAFKA_RESP_ERR__FS: rd_kafka_resp_err_t = -189;
pub const RD_KAFKA_RESP_ERR__UNKNOWN_PARTITION: rd_kafka_resp_err_t = -190;
pub const RD_KAFKA_RESP_ERR__PARTITION_EOF: rd_kafka_resp_err_t = -191;
pub const RD_KAFKA_RESP_ERR__MSG_TIMED_OUT: rd_kafka_resp_err_t = -192;
pub const RD_KAFKA_RESP_ERR__RESOLVE: rd_kafka_resp_err_t = -193;
pub const RD_KAFKA_RESP_ERR__CRIT_SYS_RESOURCE: rd_kafka_resp_err_t = -194;
pub const RD_KAFKA_RESP_ERR__TRANSPORT: rd_kafka_resp_err_t = -195;
pub const RD_KAFKA_RESP_ERR__FAIL: rd_kafka_resp_err_t = -196;
pub const RD_KAFKA_RESP_ERR__DESTROY: rd_kafka_resp_err_t = -197;
pub const RD_KAFKA_RESP_ERR__BAD_COMPRESSION: rd_kafka_resp_err_t = -198;
pub const RD_KAFKA_RESP_ERR__BAD_MSG: rd_kafka_resp_err_t = -199;
pub const RD_KAFKA_RESP_ERR__BEGIN: rd_kafka_resp_err_t = -200;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct rd_kafka_topic_partition_s {
    pub topic: *mut libc::c_char,
    pub partition: int32_t,
    pub offset: int64_t,
    pub metadata: *mut libc::c_void,
    pub metadata_size: size_t,
    pub opaque: *mut libc::c_void,
    pub err: rd_kafka_resp_err_t,
    pub _private: *mut libc::c_void,
}
pub type rd_kafka_topic_partition_t = rd_kafka_topic_partition_s;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct rd_kafka_topic_partition_list_s {
    pub cnt: libc::c_int,
    pub size: libc::c_int,
    pub elems: *mut rd_kafka_topic_partition_t,
}
pub type rd_kafka_topic_partition_list_t = rd_kafka_topic_partition_list_s;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct rd_kafka_message_s {
    pub err: rd_kafka_resp_err_t,
    pub rkt: *mut rd_kafka_topic_t,
    pub partition: int32_t,
    pub payload: *mut libc::c_void,
    pub len: size_t,
    pub key: *mut libc::c_void,
    pub key_len: size_t,
    pub offset: int64_t,
    pub _private: *mut libc::c_void,
}
pub type rd_kafka_message_t = rd_kafka_message_s;
pub type rd_kafka_conf_res_t = libc::c_int;
pub const RD_KAFKA_CONF_OK: rd_kafka_conf_res_t = 0;
pub const RD_KAFKA_CONF_INVALID: rd_kafka_conf_res_t = -1;
pub const RD_KAFKA_CONF_UNKNOWN: rd_kafka_conf_res_t = -2;
pub type C2RustUnnamed_15 = libc::c_uint;
pub const MSG_PARTITIONER_MURMUR2_RANDOM: C2RustUnnamed_15 = 6;
pub const MSG_PARTITIONER_MURMUR2: C2RustUnnamed_15 = 5;
pub const MSG_PARTITIONER_CONSISTENT_RANDOM: C2RustUnnamed_15 = 4;
pub const MSG_PARTITIONER_CONSISTENT: C2RustUnnamed_15 = 3;
pub const MSG_PARTITIONER_RANDOM: C2RustUnnamed_15 = 2;
pub type kafka_conf_type = libc::c_uint;
pub const KAFKA_TOPIC_CONF: kafka_conf_type = 2;
pub const KAFKA_CONF: kafka_conf_type = 1;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _kafka_conf_callback {
    pub fci: zend_fcall_info,
    pub fcc: zend_fcall_info_cache,
}
pub type kafka_conf_callback = _kafka_conf_callback;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _kafka_conf_callbacks {
    pub zrk: zval,
    pub error: *mut kafka_conf_callback,
    pub rebalance: *mut kafka_conf_callback,
    pub dr_msg: *mut kafka_conf_callback,
    pub stats: *mut kafka_conf_callback,
    pub consume: *mut kafka_conf_callback,
    pub offset_commit: *mut kafka_conf_callback,
    pub log: *mut kafka_conf_callback,
    pub oauthbearer_token_refresh: *mut kafka_conf_callback,
}
pub type kafka_conf_callbacks = _kafka_conf_callbacks;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _kafka_conf_object {
    pub type_0: kafka_conf_type,
    pub u: C2RustUnnamed_16,
    pub cbs: kafka_conf_callbacks,
    pub std: zend_object,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_16 {
    pub conf: *mut rd_kafka_conf_t,
    pub topic_conf: *mut rd_kafka_topic_conf_t,
}
pub type kafka_conf_object = _kafka_conf_object;
#[inline(always)]
unsafe extern "C" fn zval_get_type(mut pz: *const zval) -> zend_uchar {
    return (*pz).u1.v.type_0;
}
#[inline(always)]
unsafe extern "C" fn zval_gc_flags(mut gc_type_info: uint32_t) -> uint32_t {
    return gc_type_info >> 0 as libc::c_int
        & (0x3f0 as libc::c_int >> 0 as libc::c_int) as libc::c_uint;
}
#[inline(always)]
unsafe extern "C" fn zend_gc_set_refcount(
    mut p: *mut zend_refcounted_h,
    mut rc: uint32_t,
) -> uint32_t {
    (*p).refcount = rc;
    return (*p).refcount;
}
#[inline(always)]
unsafe extern "C" fn zend_gc_addref(mut p: *mut zend_refcounted_h) -> uint32_t {
    (*p).refcount = ((*p).refcount).wrapping_add(1);
    return (*p).refcount;
}
#[inline(always)]
unsafe extern "C" fn zend_gc_delref(mut p: *mut zend_refcounted_h) -> uint32_t {
    if !((*p).refcount > 0 as libc::c_int as libc::c_uint) as libc::c_int as libc::c_long
        != 0
    {
        unreachable!();
    }
    (*p).refcount = ((*p).refcount).wrapping_sub(1);
    return (*p).refcount;
}
#[inline(always)]
unsafe extern "C" fn zval_addref_p(mut pz: *mut zval) -> uint32_t {
    if !((*pz).u1.v.type_flags as libc::c_int != 0 as libc::c_int) as libc::c_int
        as libc::c_long != 0
    {
        unreachable!();
    }
    return zend_gc_addref(&mut (*(*pz).value.counted).gc);
}
#[inline(always)]
unsafe extern "C" fn zend_string_alloc(
    mut len: size_t,
    mut persistent: libc::c_int,
) -> *mut zend_string {
    let mut ret: *mut zend_string = (if persistent != 0 {
        __zend_malloc(
            (24 as libc::c_ulong)
                .wrapping_add(len)
                .wrapping_add(1 as libc::c_int as libc::c_ulong)
                .wrapping_add(8 as libc::c_int as libc::c_ulong)
                .wrapping_sub(1 as libc::c_int as libc::c_ulong)
                & !(8 as libc::c_int - 1 as libc::c_int) as libc::c_ulong,
        )
    } else if 0 != 0 {
        if (24 as libc::c_ulong)
            .wrapping_add(len)
            .wrapping_add(1 as libc::c_int as libc::c_ulong)
            .wrapping_add(8 as libc::c_int as libc::c_ulong)
            .wrapping_sub(1 as libc::c_int as libc::c_ulong)
            & !(8 as libc::c_int - 1 as libc::c_int) as libc::c_ulong
            <= 8 as libc::c_int as libc::c_ulong
        {
            _emalloc_8()
        } else if (24 as libc::c_ulong)
            .wrapping_add(len)
            .wrapping_add(1 as libc::c_int as libc::c_ulong)
            .wrapping_add(8 as libc::c_int as libc::c_ulong)
            .wrapping_sub(1 as libc::c_int as libc::c_ulong)
            & !(8 as libc::c_int - 1 as libc::c_int) as libc::c_ulong
            <= 16 as libc::c_int as libc::c_ulong
        {
            _emalloc_16()
        } else if (24 as libc::c_ulong)
            .wrapping_add(len)
            .wrapping_add(1 as libc::c_int as libc::c_ulong)
            .wrapping_add(8 as libc::c_int as libc::c_ulong)
            .wrapping_sub(1 as libc::c_int as libc::c_ulong)
            & !(8 as libc::c_int - 1 as libc::c_int) as libc::c_ulong
            <= 24 as libc::c_int as libc::c_ulong
        {
            _emalloc_24()
        } else if (24 as libc::c_ulong)
            .wrapping_add(len)
            .wrapping_add(1 as libc::c_int as libc::c_ulong)
            .wrapping_add(8 as libc::c_int as libc::c_ulong)
            .wrapping_sub(1 as libc::c_int as libc::c_ulong)
            & !(8 as libc::c_int - 1 as libc::c_int) as libc::c_ulong
            <= 32 as libc::c_int as libc::c_ulong
        {
            _emalloc_32()
        } else if (24 as libc::c_ulong)
            .wrapping_add(len)
            .wrapping_add(1 as libc::c_int as libc::c_ulong)
            .wrapping_add(8 as libc::c_int as libc::c_ulong)
            .wrapping_sub(1 as libc::c_int as libc::c_ulong)
            & !(8 as libc::c_int - 1 as libc::c_int) as libc::c_ulong
            <= 40 as libc::c_int as libc::c_ulong
        {
            _emalloc_40()
        } else if (24 as libc::c_ulong)
            .wrapping_add(len)
            .wrapping_add(1 as libc::c_int as libc::c_ulong)
            .wrapping_add(8 as libc::c_int as libc::c_ulong)
            .wrapping_sub(1 as libc::c_int as libc::c_ulong)
            & !(8 as libc::c_int - 1 as libc::c_int) as libc::c_ulong
            <= 48 as libc::c_int as libc::c_ulong
        {
            _emalloc_48()
        } else if (24 as libc::c_ulong)
            .wrapping_add(len)
            .wrapping_add(1 as libc::c_int as libc::c_ulong)
            .wrapping_add(8 as libc::c_int as libc::c_ulong)
            .wrapping_sub(1 as libc::c_int as libc::c_ulong)
            & !(8 as libc::c_int - 1 as libc::c_int) as libc::c_ulong
            <= 56 as libc::c_int as libc::c_ulong
        {
            _emalloc_56()
        } else if (24 as libc::c_ulong)
            .wrapping_add(len)
            .wrapping_add(1 as libc::c_int as libc::c_ulong)
            .wrapping_add(8 as libc::c_int as libc::c_ulong)
            .wrapping_sub(1 as libc::c_int as libc::c_ulong)
            & !(8 as libc::c_int - 1 as libc::c_int) as libc::c_ulong
            <= 64 as libc::c_int as libc::c_ulong
        {
            _emalloc_64()
        } else if (24 as libc::c_ulong)
            .wrapping_add(len)
            .wrapping_add(1 as libc::c_int as libc::c_ulong)
            .wrapping_add(8 as libc::c_int as libc::c_ulong)
            .wrapping_sub(1 as libc::c_int as libc::c_ulong)
            & !(8 as libc::c_int - 1 as libc::c_int) as libc::c_ulong
            <= 80 as libc::c_int as libc::c_ulong
        {
            _emalloc_80()
        } else if (24 as libc::c_ulong)
            .wrapping_add(len)
            .wrapping_add(1 as libc::c_int as libc::c_ulong)
            .wrapping_add(8 as libc::c_int as libc::c_ulong)
            .wrapping_sub(1 as libc::c_int as libc::c_ulong)
            & !(8 as libc::c_int - 1 as libc::c_int) as libc::c_ulong
            <= 96 as libc::c_int as libc::c_ulong
        {
            _emalloc_96()
        } else if (24 as libc::c_ulong)
            .wrapping_add(len)
            .wrapping_add(1 as libc::c_int as libc::c_ulong)
            .wrapping_add(8 as libc::c_int as libc::c_ulong)
            .wrapping_sub(1 as libc::c_int as libc::c_ulong)
            & !(8 as libc::c_int - 1 as libc::c_int) as libc::c_ulong
            <= 112 as libc::c_int as libc::c_ulong
        {
            _emalloc_112()
        } else if (24 as libc::c_ulong)
            .wrapping_add(len)
            .wrapping_add(1 as libc::c_int as libc::c_ulong)
            .wrapping_add(8 as libc::c_int as libc::c_ulong)
            .wrapping_sub(1 as libc::c_int as libc::c_ulong)
            & !(8 as libc::c_int - 1 as libc::c_int) as libc::c_ulong
            <= 128 as libc::c_int as libc::c_ulong
        {
            _emalloc_128()
        } else if (24 as libc::c_ulong)
            .wrapping_add(len)
            .wrapping_add(1 as libc::c_int as libc::c_ulong)
            .wrapping_add(8 as libc::c_int as libc::c_ulong)
            .wrapping_sub(1 as libc::c_int as libc::c_ulong)
            & !(8 as libc::c_int - 1 as libc::c_int) as libc::c_ulong
            <= 160 as libc::c_int as libc::c_ulong
        {
            _emalloc_160()
        } else if (24 as libc::c_ulong)
            .wrapping_add(len)
            .wrapping_add(1 as libc::c_int as libc::c_ulong)
            .wrapping_add(8 as libc::c_int as libc::c_ulong)
            .wrapping_sub(1 as libc::c_int as libc::c_ulong)
            & !(8 as libc::c_int - 1 as libc::c_int) as libc::c_ulong
            <= 192 as libc::c_int as libc::c_ulong
        {
            _emalloc_192()
        } else if (24 as libc::c_ulong)
            .wrapping_add(len)
            .wrapping_add(1 as libc::c_int as libc::c_ulong)
            .wrapping_add(8 as libc::c_int as libc::c_ulong)
            .wrapping_sub(1 as libc::c_int as libc::c_ulong)
            & !(8 as libc::c_int - 1 as libc::c_int) as libc::c_ulong
            <= 224 as libc::c_int as libc::c_ulong
        {
            _emalloc_224()
        } else if (24 as libc::c_ulong)
            .wrapping_add(len)
            .wrapping_add(1 as libc::c_int as libc::c_ulong)
            .wrapping_add(8 as libc::c_int as libc::c_ulong)
            .wrapping_sub(1 as libc::c_int as libc::c_ulong)
            & !(8 as libc::c_int - 1 as libc::c_int) as libc::c_ulong
            <= 256 as libc::c_int as libc::c_ulong
        {
            _emalloc_256()
        } else if (24 as libc::c_ulong)
            .wrapping_add(len)
            .wrapping_add(1 as libc::c_int as libc::c_ulong)
            .wrapping_add(8 as libc::c_int as libc::c_ulong)
            .wrapping_sub(1 as libc::c_int as libc::c_ulong)
            & !(8 as libc::c_int - 1 as libc::c_int) as libc::c_ulong
            <= 320 as libc::c_int as libc::c_ulong
        {
            _emalloc_320()
        } else if (24 as libc::c_ulong)
            .wrapping_add(len)
            .wrapping_add(1 as libc::c_int as libc::c_ulong)
            .wrapping_add(8 as libc::c_int as libc::c_ulong)
            .wrapping_sub(1 as libc::c_int as libc::c_ulong)
            & !(8 as libc::c_int - 1 as libc::c_int) as libc::c_ulong
            <= 384 as libc::c_int as libc::c_ulong
        {
            _emalloc_384()
        } else if (24 as libc::c_ulong)
            .wrapping_add(len)
            .wrapping_add(1 as libc::c_int as libc::c_ulong)
            .wrapping_add(8 as libc::c_int as libc::c_ulong)
            .wrapping_sub(1 as libc::c_int as libc::c_ulong)
            & !(8 as libc::c_int - 1 as libc::c_int) as libc::c_ulong
            <= 448 as libc::c_int as libc::c_ulong
        {
            _emalloc_448()
        } else if (24 as libc::c_ulong)
            .wrapping_add(len)
            .wrapping_add(1 as libc::c_int as libc::c_ulong)
            .wrapping_add(8 as libc::c_int as libc::c_ulong)
            .wrapping_sub(1 as libc::c_int as libc::c_ulong)
            & !(8 as libc::c_int - 1 as libc::c_int) as libc::c_ulong
            <= 512 as libc::c_int as libc::c_ulong
        {
            _emalloc_512()
        } else if (24 as libc::c_ulong)
            .wrapping_add(len)
            .wrapping_add(1 as libc::c_int as libc::c_ulong)
            .wrapping_add(8 as libc::c_int as libc::c_ulong)
            .wrapping_sub(1 as libc::c_int as libc::c_ulong)
            & !(8 as libc::c_int - 1 as libc::c_int) as libc::c_ulong
            <= 640 as libc::c_int as libc::c_ulong
        {
            _emalloc_640()
        } else if (24 as libc::c_ulong)
            .wrapping_add(len)
            .wrapping_add(1 as libc::c_int as libc::c_ulong)
            .wrapping_add(8 as libc::c_int as libc::c_ulong)
            .wrapping_sub(1 as libc::c_int as libc::c_ulong)
            & !(8 as libc::c_int - 1 as libc::c_int) as libc::c_ulong
            <= 768 as libc::c_int as libc::c_ulong
        {
            _emalloc_768()
        } else if (24 as libc::c_ulong)
            .wrapping_add(len)
            .wrapping_add(1 as libc::c_int as libc::c_ulong)
            .wrapping_add(8 as libc::c_int as libc::c_ulong)
            .wrapping_sub(1 as libc::c_int as libc::c_ulong)
            & !(8 as libc::c_int - 1 as libc::c_int) as libc::c_ulong
            <= 896 as libc::c_int as libc::c_ulong
        {
            _emalloc_896()
        } else if (24 as libc::c_ulong)
            .wrapping_add(len)
            .wrapping_add(1 as libc::c_int as libc::c_ulong)
            .wrapping_add(8 as libc::c_int as libc::c_ulong)
            .wrapping_sub(1 as libc::c_int as libc::c_ulong)
            & !(8 as libc::c_int - 1 as libc::c_int) as libc::c_ulong
            <= 1024 as libc::c_int as libc::c_ulong
        {
            _emalloc_1024()
        } else if (24 as libc::c_ulong)
            .wrapping_add(len)
            .wrapping_add(1 as libc::c_int as libc::c_ulong)
            .wrapping_add(8 as libc::c_int as libc::c_ulong)
            .wrapping_sub(1 as libc::c_int as libc::c_ulong)
            & !(8 as libc::c_int - 1 as libc::c_int) as libc::c_ulong
            <= 1280 as libc::c_int as libc::c_ulong
        {
            _emalloc_1280()
        } else if (24 as libc::c_ulong)
            .wrapping_add(len)
            .wrapping_add(1 as libc::c_int as libc::c_ulong)
            .wrapping_add(8 as libc::c_int as libc::c_ulong)
            .wrapping_sub(1 as libc::c_int as libc::c_ulong)
            & !(8 as libc::c_int - 1 as libc::c_int) as libc::c_ulong
            <= 1536 as libc::c_int as libc::c_ulong
        {
            _emalloc_1536()
        } else if (24 as libc::c_ulong)
            .wrapping_add(len)
            .wrapping_add(1 as libc::c_int as libc::c_ulong)
            .wrapping_add(8 as libc::c_int as libc::c_ulong)
            .wrapping_sub(1 as libc::c_int as libc::c_ulong)
            & !(8 as libc::c_int - 1 as libc::c_int) as libc::c_ulong
            <= 1792 as libc::c_int as libc::c_ulong
        {
            _emalloc_1792()
        } else if (24 as libc::c_ulong)
            .wrapping_add(len)
            .wrapping_add(1 as libc::c_int as libc::c_ulong)
            .wrapping_add(8 as libc::c_int as libc::c_ulong)
            .wrapping_sub(1 as libc::c_int as libc::c_ulong)
            & !(8 as libc::c_int - 1 as libc::c_int) as libc::c_ulong
            <= 2048 as libc::c_int as libc::c_ulong
        {
            _emalloc_2048()
        } else if (24 as libc::c_ulong)
            .wrapping_add(len)
            .wrapping_add(1 as libc::c_int as libc::c_ulong)
            .wrapping_add(8 as libc::c_int as libc::c_ulong)
            .wrapping_sub(1 as libc::c_int as libc::c_ulong)
            & !(8 as libc::c_int - 1 as libc::c_int) as libc::c_ulong
            <= 2560 as libc::c_int as libc::c_ulong
        {
            _emalloc_2560()
        } else if (24 as libc::c_ulong)
            .wrapping_add(len)
            .wrapping_add(1 as libc::c_int as libc::c_ulong)
            .wrapping_add(8 as libc::c_int as libc::c_ulong)
            .wrapping_sub(1 as libc::c_int as libc::c_ulong)
            & !(8 as libc::c_int - 1 as libc::c_int) as libc::c_ulong
            <= 3072 as libc::c_int as libc::c_ulong
        {
            _emalloc_3072()
        } else if (24 as libc::c_ulong)
            .wrapping_add(len)
            .wrapping_add(1 as libc::c_int as libc::c_ulong)
            .wrapping_add(8 as libc::c_int as libc::c_ulong)
            .wrapping_sub(1 as libc::c_int as libc::c_ulong)
            & !(8 as libc::c_int - 1 as libc::c_int) as libc::c_ulong
            <= (2 as libc::c_int * 1024 as libc::c_int * 1024 as libc::c_int
                - 4 as libc::c_int * 1024 as libc::c_int * 1 as libc::c_int)
                as libc::c_ulong
        {
            _emalloc_large(
                (24 as libc::c_ulong)
                    .wrapping_add(len)
                    .wrapping_add(1 as libc::c_int as libc::c_ulong)
                    .wrapping_add(8 as libc::c_int as libc::c_ulong)
                    .wrapping_sub(1 as libc::c_int as libc::c_ulong)
                    & !(8 as libc::c_int - 1 as libc::c_int) as libc::c_ulong,
            )
        } else {
            _emalloc_huge(
                (24 as libc::c_ulong)
                    .wrapping_add(len)
                    .wrapping_add(1 as libc::c_int as libc::c_ulong)
                    .wrapping_add(8 as libc::c_int as libc::c_ulong)
                    .wrapping_sub(1 as libc::c_int as libc::c_ulong)
                    & !(8 as libc::c_int - 1 as libc::c_int) as libc::c_ulong,
            )
        }
    } else {
        _emalloc(
            (24 as libc::c_ulong)
                .wrapping_add(len)
                .wrapping_add(1 as libc::c_int as libc::c_ulong)
                .wrapping_add(8 as libc::c_int as libc::c_ulong)
                .wrapping_sub(1 as libc::c_int as libc::c_ulong)
                & !(8 as libc::c_int - 1 as libc::c_int) as libc::c_ulong,
        )
    }) as *mut zend_string;
    zend_gc_set_refcount(&mut (*ret).gc, 1 as libc::c_int as uint32_t);
    (*ret)
        .gc
        .u
        .type_info = (6 as libc::c_int
        | (if persistent != 0 {
            (1 as libc::c_int) << 7 as libc::c_int
        } else {
            0 as libc::c_int
        }) << 0 as libc::c_int) as uint32_t;
    (*ret).h = 0 as libc::c_int as zend_ulong;
    (*ret).len = len;
    return ret;
}
#[inline(always)]
unsafe extern "C" fn zend_string_init(
    mut str: *const libc::c_char,
    mut len: size_t,
    mut persistent: libc::c_int,
) -> *mut zend_string {
    let mut ret: *mut zend_string = zend_string_alloc(len, persistent);
    memcpy(
        ((*ret).val).as_mut_ptr() as *mut libc::c_void,
        str as *const libc::c_void,
        len,
    );
    *((*ret).val).as_mut_ptr().offset(len as isize) = '\0' as i32 as libc::c_char;
    return ret;
}
#[inline(always)]
unsafe extern "C" fn zend_string_release(mut s: *mut zend_string) {
    if zval_gc_flags((*s).gc.u.type_info)
        & ((1 as libc::c_int) << 6 as libc::c_int) as libc::c_uint == 0
    {
        if zend_gc_delref(&mut (*s).gc) == 0 as libc::c_int as libc::c_uint {
            if zval_gc_flags((*s).gc.u.type_info)
                & ((1 as libc::c_int) << 7 as libc::c_int) as libc::c_uint != 0
            {
                free(s as *mut libc::c_void);
            } else {
                _efree(s as *mut libc::c_void);
            };
        }
    }
}
#[inline(always)]
unsafe extern "C" fn zval_copy_ctor(mut zvalue: *mut zval) {
    if zval_get_type(zvalue) as libc::c_int == 7 as libc::c_int {
        let mut __arr: *mut zend_array = zend_array_dup((*zvalue).value.arr);
        let mut __z: *mut zval = zvalue;
        (*__z).value.arr = __arr;
        (*__z)
            .u1
            .type_info = (7 as libc::c_int
            | ((1 as libc::c_int) << 0 as libc::c_int) << 8 as libc::c_int
            | ((1 as libc::c_int) << 1 as libc::c_int) << 8 as libc::c_int) as uint32_t;
    } else if (*zvalue).u1.v.type_flags as libc::c_int != 0 as libc::c_int {
        zval_addref_p(zvalue);
    }
}
#[inline(always)]
unsafe extern "C" fn zend_object_properties_size(
    mut ce: *mut zend_class_entry,
) -> size_t {
    return (::std::mem::size_of::<zval>() as libc::c_ulong)
        .wrapping_mul(
            ((*ce).default_properties_count
                - (if (*ce).ce_flags
                    & ((1 as libc::c_int) << 11 as libc::c_int) as libc::c_uint != 0
                {
                    0 as libc::c_int
                } else {
                    1 as libc::c_int
                })) as libc::c_ulong,
        );
}
#[inline(always)]
unsafe extern "C" fn zend_object_alloc(
    mut obj_size: size_t,
    mut ce: *mut zend_class_entry,
) -> *mut libc::c_void {
    let mut obj: *mut libc::c_void = if 0 != 0 {
        if obj_size.wrapping_add(zend_object_properties_size(ce))
            <= 8 as libc::c_int as libc::c_ulong
        {
            _emalloc_8()
        } else if obj_size.wrapping_add(zend_object_properties_size(ce))
            <= 16 as libc::c_int as libc::c_ulong
        {
            _emalloc_16()
        } else if obj_size.wrapping_add(zend_object_properties_size(ce))
            <= 24 as libc::c_int as libc::c_ulong
        {
            _emalloc_24()
        } else if obj_size.wrapping_add(zend_object_properties_size(ce))
            <= 32 as libc::c_int as libc::c_ulong
        {
            _emalloc_32()
        } else if obj_size.wrapping_add(zend_object_properties_size(ce))
            <= 40 as libc::c_int as libc::c_ulong
        {
            _emalloc_40()
        } else if obj_size.wrapping_add(zend_object_properties_size(ce))
            <= 48 as libc::c_int as libc::c_ulong
        {
            _emalloc_48()
        } else if obj_size.wrapping_add(zend_object_properties_size(ce))
            <= 56 as libc::c_int as libc::c_ulong
        {
            _emalloc_56()
        } else if obj_size.wrapping_add(zend_object_properties_size(ce))
            <= 64 as libc::c_int as libc::c_ulong
        {
            _emalloc_64()
        } else if obj_size.wrapping_add(zend_object_properties_size(ce))
            <= 80 as libc::c_int as libc::c_ulong
        {
            _emalloc_80()
        } else if obj_size.wrapping_add(zend_object_properties_size(ce))
            <= 96 as libc::c_int as libc::c_ulong
        {
            _emalloc_96()
        } else if obj_size.wrapping_add(zend_object_properties_size(ce))
            <= 112 as libc::c_int as libc::c_ulong
        {
            _emalloc_112()
        } else if obj_size.wrapping_add(zend_object_properties_size(ce))
            <= 128 as libc::c_int as libc::c_ulong
        {
            _emalloc_128()
        } else if obj_size.wrapping_add(zend_object_properties_size(ce))
            <= 160 as libc::c_int as libc::c_ulong
        {
            _emalloc_160()
        } else if obj_size.wrapping_add(zend_object_properties_size(ce))
            <= 192 as libc::c_int as libc::c_ulong
        {
            _emalloc_192()
        } else if obj_size.wrapping_add(zend_object_properties_size(ce))
            <= 224 as libc::c_int as libc::c_ulong
        {
            _emalloc_224()
        } else if obj_size.wrapping_add(zend_object_properties_size(ce))
            <= 256 as libc::c_int as libc::c_ulong
        {
            _emalloc_256()
        } else if obj_size.wrapping_add(zend_object_properties_size(ce))
            <= 320 as libc::c_int as libc::c_ulong
        {
            _emalloc_320()
        } else if obj_size.wrapping_add(zend_object_properties_size(ce))
            <= 384 as libc::c_int as libc::c_ulong
        {
            _emalloc_384()
        } else if obj_size.wrapping_add(zend_object_properties_size(ce))
            <= 448 as libc::c_int as libc::c_ulong
        {
            _emalloc_448()
        } else if obj_size.wrapping_add(zend_object_properties_size(ce))
            <= 512 as libc::c_int as libc::c_ulong
        {
            _emalloc_512()
        } else if obj_size.wrapping_add(zend_object_properties_size(ce))
            <= 640 as libc::c_int as libc::c_ulong
        {
            _emalloc_640()
        } else if obj_size.wrapping_add(zend_object_properties_size(ce))
            <= 768 as libc::c_int as libc::c_ulong
        {
            _emalloc_768()
        } else if obj_size.wrapping_add(zend_object_properties_size(ce))
            <= 896 as libc::c_int as libc::c_ulong
        {
            _emalloc_896()
        } else if obj_size.wrapping_add(zend_object_properties_size(ce))
            <= 1024 as libc::c_int as libc::c_ulong
        {
            _emalloc_1024()
        } else if obj_size.wrapping_add(zend_object_properties_size(ce))
            <= 1280 as libc::c_int as libc::c_ulong
        {
            _emalloc_1280()
        } else if obj_size.wrapping_add(zend_object_properties_size(ce))
            <= 1536 as libc::c_int as libc::c_ulong
        {
            _emalloc_1536()
        } else if obj_size.wrapping_add(zend_object_properties_size(ce))
            <= 1792 as libc::c_int as libc::c_ulong
        {
            _emalloc_1792()
        } else if obj_size.wrapping_add(zend_object_properties_size(ce))
            <= 2048 as libc::c_int as libc::c_ulong
        {
            _emalloc_2048()
        } else if obj_size.wrapping_add(zend_object_properties_size(ce))
            <= 2560 as libc::c_int as libc::c_ulong
        {
            _emalloc_2560()
        } else if obj_size.wrapping_add(zend_object_properties_size(ce))
            <= 3072 as libc::c_int as libc::c_ulong
        {
            _emalloc_3072()
        } else if obj_size.wrapping_add(zend_object_properties_size(ce))
            <= (2 as libc::c_int * 1024 as libc::c_int * 1024 as libc::c_int
                - 4 as libc::c_int * 1024 as libc::c_int * 1 as libc::c_int)
                as libc::c_ulong
        {
            _emalloc_large(obj_size.wrapping_add(zend_object_properties_size(ce)))
        } else {
            _emalloc_huge(obj_size.wrapping_add(zend_object_properties_size(ce)))
        }
    } else {
        _emalloc(obj_size.wrapping_add(zend_object_properties_size(ce)))
    };
    memset(
        obj,
        0 as libc::c_int,
        obj_size.wrapping_sub(::std::mem::size_of::<zval>() as libc::c_ulong),
    );
    return obj;
}
#[inline]
unsafe extern "C" fn rdkafka_call_function(
    mut fci: *mut zend_fcall_info,
    mut fci_cache: *mut zend_fcall_info_cache,
    mut retval: *mut zval,
    mut param_count: uint32_t,
    mut params: *mut zval,
) {
    let mut local_retval: libc::c_int = 0;
    let mut local_retval_zv: zval = zval {
        value: _zend_value { lval: 0 },
        u1: C2RustUnnamed_1 {
            v: C2RustUnnamed_2 {
                type_0: 0,
                type_flags: 0,
                u: C2RustUnnamed_3 { extra: 0 },
            },
        },
        u2: C2RustUnnamed_0 { next: 0 },
    };
    if !retval.is_null() {
        local_retval = 0 as libc::c_int;
    } else {
        local_retval = 1 as libc::c_int;
        retval = &mut local_retval_zv;
    }
    (*fci).retval = retval;
    (*fci).params = params;
    (*fci).param_count = param_count;
    zend_call_function(fci, fci_cache);
    if local_retval != 0 {
        zval_ptr_dtor(retval);
    }
}
static mut arginfo_class_RdKafka_Conf___construct: [zend_internal_arg_info; 1] = [
    {
        let mut init = _zend_internal_arg_info {
            name: 0 as *const libc::c_char,
            type_0: 0 as libc::c_int as zend_type,
            pass_by_reference: 0 as libc::c_int as zend_uchar,
            is_variadic: 0 as libc::c_int as zend_bool,
        };
        init
    },
];
static mut arginfo_class_RdKafka_Conf_dump: [zend_internal_arg_info; 1] = [
    {
        let mut init = _zend_internal_arg_info {
            name: 0 as *const libc::c_char,
            type_0: 0 as libc::c_int as zend_type,
            pass_by_reference: 0 as libc::c_int as zend_uchar,
            is_variadic: 0 as libc::c_int as zend_bool,
        };
        init
    },
];
static mut arginfo_class_RdKafka_Conf_set: [zend_internal_arg_info; 3] = unsafe {
    [
        {
            let mut init = _zend_internal_arg_info {
                name: 2 as libc::c_int as zend_uintptr_t as *const libc::c_char,
                type_0: 0 as libc::c_int as zend_type,
                pass_by_reference: 0 as libc::c_int as zend_uchar,
                is_variadic: 0 as libc::c_int as zend_bool,
            };
            init
        },
        {
            let mut init = _zend_internal_arg_info {
                name: b"name\0" as *const u8 as *const libc::c_char,
                type_0: 0 as libc::c_int as zend_type,
                pass_by_reference: 0 as libc::c_int as zend_uchar,
                is_variadic: 0 as libc::c_int as zend_bool,
            };
            init
        },
        {
            let mut init = _zend_internal_arg_info {
                name: b"value\0" as *const u8 as *const libc::c_char,
                type_0: 0 as libc::c_int as zend_type,
                pass_by_reference: 0 as libc::c_int as zend_uchar,
                is_variadic: 0 as libc::c_int as zend_bool,
            };
            init
        },
    ]
};
static mut arginfo_class_RdKafka_Conf_setDefaultTopicConf: [zend_internal_arg_info; 2] = unsafe {
    [
        {
            let mut init = _zend_internal_arg_info {
                name: 1 as libc::c_int as zend_uintptr_t as *const libc::c_char,
                type_0: 0 as libc::c_int as zend_type,
                pass_by_reference: 0 as libc::c_int as zend_uchar,
                is_variadic: 0 as libc::c_int as zend_bool,
            };
            init
        },
        {
            let mut init = _zend_internal_arg_info {
                name: b"topic_conf\0" as *const u8 as *const libc::c_char,
                type_0: 0 as libc::c_int as zend_type,
                pass_by_reference: 0 as libc::c_int as zend_uchar,
                is_variadic: 0 as libc::c_int as zend_bool,
            };
            init
        },
    ]
};
static mut arginfo_class_RdKafka_Conf_setErrorCb: [zend_internal_arg_info; 2] = unsafe {
    [
        {
            let mut init = _zend_internal_arg_info {
                name: 1 as libc::c_int as zend_uintptr_t as *const libc::c_char,
                type_0: 0 as libc::c_int as zend_type,
                pass_by_reference: 0 as libc::c_int as zend_uchar,
                is_variadic: 0 as libc::c_int as zend_bool,
            };
            init
        },
        {
            let mut init = _zend_internal_arg_info {
                name: b"callback\0" as *const u8 as *const libc::c_char,
                type_0: 0 as libc::c_int as zend_type,
                pass_by_reference: 0 as libc::c_int as zend_uchar,
                is_variadic: 0 as libc::c_int as zend_bool,
            };
            init
        },
    ]
};
static mut arginfo_class_RdKafka_TopicConf_setPartitioner: [zend_internal_arg_info; 2] = unsafe {
    [
        {
            let mut init = _zend_internal_arg_info {
                name: 1 as libc::c_int as zend_uintptr_t as *const libc::c_char,
                type_0: 0 as libc::c_int as zend_type,
                pass_by_reference: 0 as libc::c_int as zend_uchar,
                is_variadic: 0 as libc::c_int as zend_bool,
            };
            init
        },
        {
            let mut init = _zend_internal_arg_info {
                name: b"partitioner\0" as *const u8 as *const libc::c_char,
                type_0: 0 as libc::c_int as zend_type,
                pass_by_reference: 0 as libc::c_int as zend_uchar,
                is_variadic: 0 as libc::c_int as zend_bool,
            };
            init
        },
    ]
};
static mut class_RdKafka_Conf_methods: [zend_function_entry; 13] = [zend_function_entry {
    fname: 0 as *const libc::c_char,
    handler: None,
    arg_info: 0 as *const _zend_internal_arg_info,
    num_args: 0,
    flags: 0,
}; 13];
static mut class_RdKafka_TopicConf_methods: [zend_function_entry; 5] = [zend_function_entry {
    fname: 0 as *const libc::c_char,
    handler: None,
    arg_info: 0 as *const _zend_internal_arg_info,
    num_args: 0,
    flags: 0,
}; 5];
unsafe extern "C" fn register_class_RdKafka_Conf() -> *mut zend_class_entry {
    let mut ce: zend_class_entry = zend_class_entry {
        type_0: 0,
        name: 0 as *mut zend_string,
        c2rust_unnamed: C2RustUnnamed_13 {
            parent: 0 as *mut zend_class_entry,
        },
        refcount: 0,
        ce_flags: 0,
        default_properties_count: 0,
        default_static_members_count: 0,
        default_properties_table: 0 as *mut zval,
        default_static_members_table: 0 as *mut zval,
        static_members_table__ptr: 0 as *mut *mut zval,
        function_table: HashTable {
            gc: zend_refcounted_h {
                refcount: 0,
                u: C2RustUnnamed_5 { type_info: 0 },
            },
            u: C2RustUnnamed_6 {
                v: C2RustUnnamed_7 {
                    flags: 0,
                    _unused: 0,
                    nIteratorsCount: 0,
                    _unused2: 0,
                },
            },
            nTableMask: 0,
            arData: 0 as *mut Bucket,
            nNumUsed: 0,
            nNumOfElements: 0,
            nTableSize: 0,
            nInternalPointer: 0,
            nNextFreeElement: 0,
            pDestructor: None,
        },
        properties_info: HashTable {
            gc: zend_refcounted_h {
                refcount: 0,
                u: C2RustUnnamed_5 { type_info: 0 },
            },
            u: C2RustUnnamed_6 {
                v: C2RustUnnamed_7 {
                    flags: 0,
                    _unused: 0,
                    nIteratorsCount: 0,
                    _unused2: 0,
                },
            },
            nTableMask: 0,
            arData: 0 as *mut Bucket,
            nNumUsed: 0,
            nNumOfElements: 0,
            nTableSize: 0,
            nInternalPointer: 0,
            nNextFreeElement: 0,
            pDestructor: None,
        },
        constants_table: HashTable {
            gc: zend_refcounted_h {
                refcount: 0,
                u: C2RustUnnamed_5 { type_info: 0 },
            },
            u: C2RustUnnamed_6 {
                v: C2RustUnnamed_7 {
                    flags: 0,
                    _unused: 0,
                    nIteratorsCount: 0,
                    _unused2: 0,
                },
            },
            nTableMask: 0,
            arData: 0 as *mut Bucket,
            nNumUsed: 0,
            nNumOfElements: 0,
            nTableSize: 0,
            nInternalPointer: 0,
            nNextFreeElement: 0,
            pDestructor: None,
        },
        properties_info_table: 0 as *mut *mut _zend_property_info,
        constructor: 0 as *mut zend_function,
        destructor: 0 as *mut zend_function,
        clone: 0 as *mut zend_function,
        __get: 0 as *mut zend_function,
        __set: 0 as *mut zend_function,
        __unset: 0 as *mut zend_function,
        __isset: 0 as *mut zend_function,
        __call: 0 as *mut zend_function,
        __callstatic: 0 as *mut zend_function,
        __tostring: 0 as *mut zend_function,
        __debugInfo: 0 as *mut zend_function,
        serialize_func: 0 as *mut zend_function,
        unserialize_func: 0 as *mut zend_function,
        iterator_funcs_ptr: 0 as *mut zend_class_iterator_funcs,
        c2rust_unnamed_0: C2RustUnnamed_12 {
            create_object: None,
        },
        get_iterator: None,
        get_static_method: None,
        serialize: None,
        unserialize: None,
        num_interfaces: 0,
        num_traits: 0,
        c2rust_unnamed_1: C2RustUnnamed_11 {
            interfaces: 0 as *mut *mut zend_class_entry,
        },
        trait_names: 0 as *mut zend_class_name,
        trait_aliases: 0 as *mut *mut zend_trait_alias,
        trait_precedences: 0 as *mut *mut zend_trait_precedence,
        info: C2RustUnnamed_8 {
            user: C2RustUnnamed_10 {
                filename: 0 as *mut zend_string,
                line_start: 0,
                line_end: 0,
                doc_comment: 0 as *mut zend_string,
            },
        },
    };
    let mut class_entry: *mut zend_class_entry = 0 as *mut zend_class_entry;
    memset(
        &mut ce as *mut zend_class_entry as *mut libc::c_void,
        0 as libc::c_int,
        ::std::mem::size_of::<zend_class_entry>() as libc::c_ulong,
    );
    ce
        .name = zend_string_init_interned
        .unwrap()(
        b"RdKafka\\Conf\0" as *const u8 as *const libc::c_char,
        (::std::mem::size_of::<[libc::c_char; 13]>() as libc::c_ulong)
            .wrapping_sub(1 as libc::c_int as libc::c_ulong),
        1 as libc::c_int,
    );
    ce.info.internal.builtin_functions = class_RdKafka_Conf_methods.as_ptr();
    class_entry = zend_register_internal_class_ex(&mut ce, 0 as *mut zend_class_entry);
    return class_entry;
}
unsafe extern "C" fn register_class_RdKafka_TopicConf() -> *mut zend_class_entry {
    let mut ce: zend_class_entry = zend_class_entry {
        type_0: 0,
        name: 0 as *mut zend_string,
        c2rust_unnamed: C2RustUnnamed_13 {
            parent: 0 as *mut zend_class_entry,
        },
        refcount: 0,
        ce_flags: 0,
        default_properties_count: 0,
        default_static_members_count: 0,
        default_properties_table: 0 as *mut zval,
        default_static_members_table: 0 as *mut zval,
        static_members_table__ptr: 0 as *mut *mut zval,
        function_table: HashTable {
            gc: zend_refcounted_h {
                refcount: 0,
                u: C2RustUnnamed_5 { type_info: 0 },
            },
            u: C2RustUnnamed_6 {
                v: C2RustUnnamed_7 {
                    flags: 0,
                    _unused: 0,
                    nIteratorsCount: 0,
                    _unused2: 0,
                },
            },
            nTableMask: 0,
            arData: 0 as *mut Bucket,
            nNumUsed: 0,
            nNumOfElements: 0,
            nTableSize: 0,
            nInternalPointer: 0,
            nNextFreeElement: 0,
            pDestructor: None,
        },
        properties_info: HashTable {
            gc: zend_refcounted_h {
                refcount: 0,
                u: C2RustUnnamed_5 { type_info: 0 },
            },
            u: C2RustUnnamed_6 {
                v: C2RustUnnamed_7 {
                    flags: 0,
                    _unused: 0,
                    nIteratorsCount: 0,
                    _unused2: 0,
                },
            },
            nTableMask: 0,
            arData: 0 as *mut Bucket,
            nNumUsed: 0,
            nNumOfElements: 0,
            nTableSize: 0,
            nInternalPointer: 0,
            nNextFreeElement: 0,
            pDestructor: None,
        },
        constants_table: HashTable {
            gc: zend_refcounted_h {
                refcount: 0,
                u: C2RustUnnamed_5 { type_info: 0 },
            },
            u: C2RustUnnamed_6 {
                v: C2RustUnnamed_7 {
                    flags: 0,
                    _unused: 0,
                    nIteratorsCount: 0,
                    _unused2: 0,
                },
            },
            nTableMask: 0,
            arData: 0 as *mut Bucket,
            nNumUsed: 0,
            nNumOfElements: 0,
            nTableSize: 0,
            nInternalPointer: 0,
            nNextFreeElement: 0,
            pDestructor: None,
        },
        properties_info_table: 0 as *mut *mut _zend_property_info,
        constructor: 0 as *mut zend_function,
        destructor: 0 as *mut zend_function,
        clone: 0 as *mut zend_function,
        __get: 0 as *mut zend_function,
        __set: 0 as *mut zend_function,
        __unset: 0 as *mut zend_function,
        __isset: 0 as *mut zend_function,
        __call: 0 as *mut zend_function,
        __callstatic: 0 as *mut zend_function,
        __tostring: 0 as *mut zend_function,
        __debugInfo: 0 as *mut zend_function,
        serialize_func: 0 as *mut zend_function,
        unserialize_func: 0 as *mut zend_function,
        iterator_funcs_ptr: 0 as *mut zend_class_iterator_funcs,
        c2rust_unnamed_0: C2RustUnnamed_12 {
            create_object: None,
        },
        get_iterator: None,
        get_static_method: None,
        serialize: None,
        unserialize: None,
        num_interfaces: 0,
        num_traits: 0,
        c2rust_unnamed_1: C2RustUnnamed_11 {
            interfaces: 0 as *mut *mut zend_class_entry,
        },
        trait_names: 0 as *mut zend_class_name,
        trait_aliases: 0 as *mut *mut zend_trait_alias,
        trait_precedences: 0 as *mut *mut zend_trait_precedence,
        info: C2RustUnnamed_8 {
            user: C2RustUnnamed_10 {
                filename: 0 as *mut zend_string,
                line_start: 0,
                line_end: 0,
                doc_comment: 0 as *mut zend_string,
            },
        },
    };
    let mut class_entry: *mut zend_class_entry = 0 as *mut zend_class_entry;
    memset(
        &mut ce as *mut zend_class_entry as *mut libc::c_void,
        0 as libc::c_int,
        ::std::mem::size_of::<zend_class_entry>() as libc::c_ulong,
    );
    ce
        .name = zend_string_init_interned
        .unwrap()(
        b"RdKafka\\TopicConf\0" as *const u8 as *const libc::c_char,
        (::std::mem::size_of::<[libc::c_char; 18]>() as libc::c_ulong)
            .wrapping_sub(1 as libc::c_int as libc::c_ulong),
        1 as libc::c_int,
    );
    ce.info.internal.builtin_functions = class_RdKafka_TopicConf_methods.as_ptr();
    class_entry = zend_register_internal_class_ex(&mut ce, 0 as *mut zend_class_entry);
    return class_entry;
}
pub static mut ce_kafka_conf: *mut zend_class_entry = 0 as *const zend_class_entry
    as *mut zend_class_entry;
pub static mut ce_kafka_topic_conf: *mut zend_class_entry = 0 as *const zend_class_entry
    as *mut zend_class_entry;
static mut handlers: zend_object_handlers = zend_object_handlers {
    offset: 0,
    free_obj: None,
    dtor_obj: None,
    clone_obj: None,
    read_property: None,
    write_property: None,
    read_dimension: None,
    write_dimension: None,
    get_property_ptr_ptr: None,
    get: None,
    set: None,
    has_property: None,
    unset_property: None,
    has_dimension: None,
    unset_dimension: None,
    get_properties: None,
    get_method: None,
    call_method: None,
    get_constructor: None,
    get_class_name: None,
    compare_objects: None,
    cast_object: None,
    count_elements: None,
    get_debug_info: None,
    get_closure: None,
    get_gc: None,
    do_operation: None,
    compare: None,
    get_properties_for: None,
};
unsafe extern "C" fn kafka_conf_callback_dtor(mut cb: *mut kafka_conf_callback) {
    if !cb.is_null() {
        zval_ptr_dtor(&mut (*cb).fci.function_name);
        _efree(cb as *mut libc::c_void);
    }
}
pub unsafe extern "C" fn kafka_conf_callbacks_dtor(mut cbs: *mut kafka_conf_callbacks) {
    kafka_conf_callback_dtor((*cbs).error);
    (*cbs).error = 0 as *mut kafka_conf_callback;
    kafka_conf_callback_dtor((*cbs).rebalance);
    (*cbs).rebalance = 0 as *mut kafka_conf_callback;
    kafka_conf_callback_dtor((*cbs).dr_msg);
    (*cbs).dr_msg = 0 as *mut kafka_conf_callback;
    kafka_conf_callback_dtor((*cbs).stats);
    (*cbs).stats = 0 as *mut kafka_conf_callback;
    kafka_conf_callback_dtor((*cbs).consume);
    (*cbs).consume = 0 as *mut kafka_conf_callback;
    kafka_conf_callback_dtor((*cbs).offset_commit);
    (*cbs).offset_commit = 0 as *mut kafka_conf_callback;
    kafka_conf_callback_dtor((*cbs).log);
    (*cbs).log = 0 as *mut kafka_conf_callback;
    kafka_conf_callback_dtor((*cbs).oauthbearer_token_refresh);
    (*cbs).oauthbearer_token_refresh = 0 as *mut kafka_conf_callback;
}
unsafe extern "C" fn kafka_conf_callback_copy(
    mut to: *mut *mut kafka_conf_callback,
    mut from: *mut kafka_conf_callback,
) {
    if !from.is_null() {
        *to = (if 0 != 0 {
            if ::std::mem::size_of::<kafka_conf_callback>() as libc::c_ulong
                <= 8 as libc::c_int as libc::c_ulong
            {
                _emalloc_8()
            } else if ::std::mem::size_of::<kafka_conf_callback>() as libc::c_ulong
                <= 16 as libc::c_int as libc::c_ulong
            {
                _emalloc_16()
            } else if ::std::mem::size_of::<kafka_conf_callback>() as libc::c_ulong
                <= 24 as libc::c_int as libc::c_ulong
            {
                _emalloc_24()
            } else if ::std::mem::size_of::<kafka_conf_callback>() as libc::c_ulong
                <= 32 as libc::c_int as libc::c_ulong
            {
                _emalloc_32()
            } else if ::std::mem::size_of::<kafka_conf_callback>() as libc::c_ulong
                <= 40 as libc::c_int as libc::c_ulong
            {
                _emalloc_40()
            } else if ::std::mem::size_of::<kafka_conf_callback>() as libc::c_ulong
                <= 48 as libc::c_int as libc::c_ulong
            {
                _emalloc_48()
            } else if ::std::mem::size_of::<kafka_conf_callback>() as libc::c_ulong
                <= 56 as libc::c_int as libc::c_ulong
            {
                _emalloc_56()
            } else if ::std::mem::size_of::<kafka_conf_callback>() as libc::c_ulong
                <= 64 as libc::c_int as libc::c_ulong
            {
                _emalloc_64()
            } else if ::std::mem::size_of::<kafka_conf_callback>() as libc::c_ulong
                <= 80 as libc::c_int as libc::c_ulong
            {
                _emalloc_80()
            } else if ::std::mem::size_of::<kafka_conf_callback>() as libc::c_ulong
                <= 96 as libc::c_int as libc::c_ulong
            {
                _emalloc_96()
            } else if ::std::mem::size_of::<kafka_conf_callback>() as libc::c_ulong
                <= 112 as libc::c_int as libc::c_ulong
            {
                _emalloc_112()
            } else if ::std::mem::size_of::<kafka_conf_callback>() as libc::c_ulong
                <= 128 as libc::c_int as libc::c_ulong
            {
                _emalloc_128()
            } else if ::std::mem::size_of::<kafka_conf_callback>() as libc::c_ulong
                <= 160 as libc::c_int as libc::c_ulong
            {
                _emalloc_160()
            } else if ::std::mem::size_of::<kafka_conf_callback>() as libc::c_ulong
                <= 192 as libc::c_int as libc::c_ulong
            {
                _emalloc_192()
            } else if ::std::mem::size_of::<kafka_conf_callback>() as libc::c_ulong
                <= 224 as libc::c_int as libc::c_ulong
            {
                _emalloc_224()
            } else if ::std::mem::size_of::<kafka_conf_callback>() as libc::c_ulong
                <= 256 as libc::c_int as libc::c_ulong
            {
                _emalloc_256()
            } else if ::std::mem::size_of::<kafka_conf_callback>() as libc::c_ulong
                <= 320 as libc::c_int as libc::c_ulong
            {
                _emalloc_320()
            } else if ::std::mem::size_of::<kafka_conf_callback>() as libc::c_ulong
                <= 384 as libc::c_int as libc::c_ulong
            {
                _emalloc_384()
            } else if ::std::mem::size_of::<kafka_conf_callback>() as libc::c_ulong
                <= 448 as libc::c_int as libc::c_ulong
            {
                _emalloc_448()
            } else if ::std::mem::size_of::<kafka_conf_callback>() as libc::c_ulong
                <= 512 as libc::c_int as libc::c_ulong
            {
                _emalloc_512()
            } else if ::std::mem::size_of::<kafka_conf_callback>() as libc::c_ulong
                <= 640 as libc::c_int as libc::c_ulong
            {
                _emalloc_640()
            } else if ::std::mem::size_of::<kafka_conf_callback>() as libc::c_ulong
                <= 768 as libc::c_int as libc::c_ulong
            {
                _emalloc_768()
            } else if ::std::mem::size_of::<kafka_conf_callback>() as libc::c_ulong
                <= 896 as libc::c_int as libc::c_ulong
            {
                _emalloc_896()
            } else if ::std::mem::size_of::<kafka_conf_callback>() as libc::c_ulong
                <= 1024 as libc::c_int as libc::c_ulong
            {
                _emalloc_1024()
            } else if ::std::mem::size_of::<kafka_conf_callback>() as libc::c_ulong
                <= 1280 as libc::c_int as libc::c_ulong
            {
                _emalloc_1280()
            } else if ::std::mem::size_of::<kafka_conf_callback>() as libc::c_ulong
                <= 1536 as libc::c_int as libc::c_ulong
            {
                _emalloc_1536()
            } else if ::std::mem::size_of::<kafka_conf_callback>() as libc::c_ulong
                <= 1792 as libc::c_int as libc::c_ulong
            {
                _emalloc_1792()
            } else if ::std::mem::size_of::<kafka_conf_callback>() as libc::c_ulong
                <= 2048 as libc::c_int as libc::c_ulong
            {
                _emalloc_2048()
            } else if ::std::mem::size_of::<kafka_conf_callback>() as libc::c_ulong
                <= 2560 as libc::c_int as libc::c_ulong
            {
                _emalloc_2560()
            } else if ::std::mem::size_of::<kafka_conf_callback>() as libc::c_ulong
                <= 3072 as libc::c_int as libc::c_ulong
            {
                _emalloc_3072()
            } else if ::std::mem::size_of::<kafka_conf_callback>() as libc::c_ulong
                <= (2 as libc::c_int * 1024 as libc::c_int * 1024 as libc::c_int
                    - 4 as libc::c_int * 1024 as libc::c_int * 1 as libc::c_int)
                    as libc::c_ulong
            {
                _emalloc_large(
                    ::std::mem::size_of::<kafka_conf_callback>() as libc::c_ulong,
                )
            } else {
                _emalloc_huge(
                    ::std::mem::size_of::<kafka_conf_callback>() as libc::c_ulong,
                )
            }
        } else {
            _emalloc(::std::mem::size_of::<kafka_conf_callback>() as libc::c_ulong)
        }) as *mut kafka_conf_callback;
        **to = *from;
        zval_copy_ctor(&mut (**to).fci.function_name);
    }
}
pub unsafe extern "C" fn kafka_conf_callbacks_copy(
    mut to: *mut kafka_conf_callbacks,
    mut from: *mut kafka_conf_callbacks,
) {
    kafka_conf_callback_copy(&mut (*to).error, (*from).error);
    kafka_conf_callback_copy(&mut (*to).rebalance, (*from).rebalance);
    kafka_conf_callback_copy(&mut (*to).dr_msg, (*from).dr_msg);
    kafka_conf_callback_copy(&mut (*to).stats, (*from).stats);
    kafka_conf_callback_copy(&mut (*to).consume, (*from).consume);
    kafka_conf_callback_copy(&mut (*to).offset_commit, (*from).offset_commit);
    kafka_conf_callback_copy(&mut (*to).log, (*from).log);
}
unsafe extern "C" fn kafka_conf_free(mut object: *mut zend_object) {
    let mut intern: *mut kafka_conf_object = (object as *mut libc::c_char)
        .offset(-(96 as libc::c_ulong as isize)) as *mut kafka_conf_object;
    match (*intern).type_0 as libc::c_uint {
        1 => {
            if !((*intern).u.conf).is_null() {
                rd_kafka_conf_destroy((*intern).u.conf);
            }
            kafka_conf_callbacks_dtor(&mut (*intern).cbs);
        }
        2 => {
            if !((*intern).u.topic_conf).is_null() {
                rd_kafka_topic_conf_destroy((*intern).u.topic_conf);
            }
        }
        _ => {}
    }
    zend_object_std_dtor(&mut (*intern).std);
}
unsafe extern "C" fn kafka_conf_new(
    mut class_type: *mut zend_class_entry,
) -> *mut zend_object {
    let mut retval: *mut zend_object = 0 as *mut zend_object;
    let mut intern: *mut kafka_conf_object = 0 as *mut kafka_conf_object;
    intern = zend_object_alloc(
        ::std::mem::size_of::<kafka_conf_object>() as libc::c_ulong,
        class_type,
    ) as *mut kafka_conf_object;
    zend_object_std_init(&mut (*intern).std, class_type);
    object_properties_init(&mut (*intern).std, class_type);
    retval = &mut (*intern).std;
    (*retval).handlers = &mut handlers;
    return retval;
}
pub unsafe extern "C" fn get_kafka_conf_object(
    mut zconf: *mut zval,
) -> *mut kafka_conf_object {
    let mut oconf: *mut kafka_conf_object = ((*zconf).value.obj as *mut libc::c_char)
        .offset(-(96 as libc::c_ulong as isize)) as *mut kafka_conf_object;
    if (*oconf).type_0 as u64 == 0 {
        zend_throw_exception_ex(
            0 as *mut zend_class_entry,
            0 as libc::c_int as zend_long,
            b"RdKafka\\Conf::__construct() has not been called\0" as *const u8
                as *const libc::c_char,
        );
        return 0 as *mut kafka_conf_object;
    }
    return oconf;
}
unsafe extern "C" fn kafka_conf_error_cb(
    mut rk: *mut rd_kafka_t,
    mut err: libc::c_int,
    mut reason: *const libc::c_char,
    mut opaque: *mut libc::c_void,
) {
    let mut cbs: *mut kafka_conf_callbacks = opaque as *mut kafka_conf_callbacks;
    let mut args: [zval; 3] = [zval {
        value: _zend_value { lval: 0 },
        u1: C2RustUnnamed_1 {
            v: C2RustUnnamed_2 {
                type_0: 0,
                type_flags: 0,
                u: C2RustUnnamed_3 { extra: 0 },
            },
        },
        u2: C2RustUnnamed_0 { next: 0 },
    }; 3];
    if opaque.is_null() {
        return;
    }
    if ((*cbs).error).is_null() {
        return;
    }
    args[0 as libc::c_int as usize].u1.type_info = 1 as libc::c_int as uint32_t;
    args[1 as libc::c_int as usize].u1.type_info = 1 as libc::c_int as uint32_t;
    args[2 as libc::c_int as usize].u1.type_info = 1 as libc::c_int as uint32_t;
    let mut __z: *mut zval = &mut *args.as_mut_ptr().offset(0 as libc::c_int as isize)
        as *mut zval;
    let mut __zv: *mut zval = &mut (*cbs).zrk;
    if !(zval_get_type(__zv) as libc::c_int == 10 as libc::c_int) as libc::c_int
        as libc::c_long != 0
    {
        if 1 as libc::c_int != 0 && 0 as libc::c_int == 0 {
            let mut _z1: *mut zval = __z;
            let mut _z2: *const zval = __zv;
            let mut _gc: *mut zend_refcounted = (*_z2).value.counted;
            let mut _t: uint32_t = (*_z2).u1.type_info;
            (*_z1).value.counted = _gc;
            (*_z1).u1.type_info = _t;
            if _t & 0xff00 as libc::c_int as libc::c_uint
                != 0 as libc::c_int as libc::c_uint
            {
                zend_gc_addref(&mut (*_gc).gc);
            }
        } else {
            let mut _z1_0: *mut zval = __z;
            let mut _z2_0: *const zval = __zv;
            let mut _gc_0: *mut zend_refcounted = (*_z2_0).value.counted;
            let mut _t_0: uint32_t = (*_z2_0).u1.type_info;
            (*_z1_0).value.counted = _gc_0;
            (*_z1_0).u1.type_info = _t_0;
        }
    } else {
        let mut _z1_1: *mut zval = __z;
        let mut _z2_1: *const zval = &mut (*(*__zv).value.ref_0).val;
        let mut _gc_1: *mut zend_refcounted = (*_z2_1).value.counted;
        let mut _t_1: uint32_t = (*_z2_1).u1.type_info;
        (*_z1_1).value.counted = _gc_1;
        (*_z1_1).u1.type_info = _t_1;
        if _t_1 & 0xff00 as libc::c_int as libc::c_uint
            != 0 as libc::c_int as libc::c_uint
        {
            zend_gc_addref(&mut (*_gc_1).gc);
        }
        if 0 as libc::c_int != 0 || 1 as libc::c_int == 0 {
            zval_ptr_dtor(__zv);
        }
    }
    let mut __z_0: *mut zval = &mut *args.as_mut_ptr().offset(1 as libc::c_int as isize)
        as *mut zval;
    (*__z_0).value.lval = err as zend_long;
    (*__z_0).u1.type_info = 4 as libc::c_int as uint32_t;
    let mut _s: *const libc::c_char = reason;
    let mut __z_1: *mut zval = &mut *args.as_mut_ptr().offset(2 as libc::c_int as isize)
        as *mut zval;
    let mut __s: *mut zend_string = zend_string_init(_s, strlen(_s), 0 as libc::c_int);
    (*__z_1).value.str_0 = __s;
    (*__z_1)
        .u1
        .type_info = (6 as libc::c_int
        | ((1 as libc::c_int) << 0 as libc::c_int) << 8 as libc::c_int) as uint32_t;
    rdkafka_call_function(
        &mut (*(*cbs).error).fci,
        &mut (*(*cbs).error).fcc,
        0 as *mut zval,
        3 as libc::c_int as uint32_t,
        args.as_mut_ptr(),
    );
    zval_ptr_dtor(&mut *args.as_mut_ptr().offset(0 as libc::c_int as isize));
    zval_ptr_dtor(&mut *args.as_mut_ptr().offset(1 as libc::c_int as isize));
    zval_ptr_dtor(&mut *args.as_mut_ptr().offset(2 as libc::c_int as isize));
}
pub unsafe extern "C" fn kafka_conf_dr_msg_cb(
    mut rk: *mut rd_kafka_t,
    mut msg: *const rd_kafka_message_t,
    mut opaque: *mut libc::c_void,
) {
    let mut cbs: *mut kafka_conf_callbacks = opaque as *mut kafka_conf_callbacks;
    let mut msg_opaque: *mut zend_string = (*msg)._private as *mut zend_string;
    let mut args: [zval; 2] = [zval {
        value: _zend_value { lval: 0 },
        u1: C2RustUnnamed_1 {
            v: C2RustUnnamed_2 {
                type_0: 0,
                type_flags: 0,
                u: C2RustUnnamed_3 { extra: 0 },
            },
        },
        u2: C2RustUnnamed_0 { next: 0 },
    }; 2];
    if !cbs.is_null() && !((*cbs).dr_msg).is_null() {
        args[0 as libc::c_int as usize].u1.type_info = 1 as libc::c_int as uint32_t;
        args[1 as libc::c_int as usize].u1.type_info = 1 as libc::c_int as uint32_t;
        let mut __z: *mut zval = &mut *args
            .as_mut_ptr()
            .offset(0 as libc::c_int as isize) as *mut zval;
        let mut __zv: *mut zval = &mut (*cbs).zrk;
        if !(zval_get_type(__zv) as libc::c_int == 10 as libc::c_int) as libc::c_int
            as libc::c_long != 0
        {
            if 1 as libc::c_int != 0 && 0 as libc::c_int == 0 {
                let mut _z1: *mut zval = __z;
                let mut _z2: *const zval = __zv;
                let mut _gc: *mut zend_refcounted = (*_z2).value.counted;
                let mut _t: uint32_t = (*_z2).u1.type_info;
                (*_z1).value.counted = _gc;
                (*_z1).u1.type_info = _t;
                if _t & 0xff00 as libc::c_int as libc::c_uint
                    != 0 as libc::c_int as libc::c_uint
                {
                    zend_gc_addref(&mut (*_gc).gc);
                }
            } else {
                let mut _z1_0: *mut zval = __z;
                let mut _z2_0: *const zval = __zv;
                let mut _gc_0: *mut zend_refcounted = (*_z2_0).value.counted;
                let mut _t_0: uint32_t = (*_z2_0).u1.type_info;
                (*_z1_0).value.counted = _gc_0;
                (*_z1_0).u1.type_info = _t_0;
            }
        } else {
            let mut _z1_1: *mut zval = __z;
            let mut _z2_1: *const zval = &mut (*(*__zv).value.ref_0).val;
            let mut _gc_1: *mut zend_refcounted = (*_z2_1).value.counted;
            let mut _t_1: uint32_t = (*_z2_1).u1.type_info;
            (*_z1_1).value.counted = _gc_1;
            (*_z1_1).u1.type_info = _t_1;
            if _t_1 & 0xff00 as libc::c_int as libc::c_uint
                != 0 as libc::c_int as libc::c_uint
            {
                zend_gc_addref(&mut (*_gc_1).gc);
            }
            if 0 as libc::c_int != 0 || 1 as libc::c_int == 0 {
                zval_ptr_dtor(__zv);
            }
        }
        kafka_message_new(
            &mut *args.as_mut_ptr().offset(1 as libc::c_int as isize),
            msg,
            msg_opaque,
        );
        rdkafka_call_function(
            &mut (*(*cbs).dr_msg).fci,
            &mut (*(*cbs).dr_msg).fcc,
            0 as *mut zval,
            2 as libc::c_int as uint32_t,
            args.as_mut_ptr(),
        );
        zval_ptr_dtor(&mut *args.as_mut_ptr().offset(0 as libc::c_int as isize));
        zval_ptr_dtor(&mut *args.as_mut_ptr().offset(1 as libc::c_int as isize));
    }
    if !msg_opaque.is_null() {
        zend_string_release(msg_opaque);
    }
}
unsafe extern "C" fn kafka_conf_stats_cb(
    mut rk: *mut rd_kafka_t,
    mut json: *mut libc::c_char,
    mut json_len: size_t,
    mut opaque: *mut libc::c_void,
) -> libc::c_int {
    let mut cbs: *mut kafka_conf_callbacks = opaque as *mut kafka_conf_callbacks;
    let mut args: [zval; 3] = [zval {
        value: _zend_value { lval: 0 },
        u1: C2RustUnnamed_1 {
            v: C2RustUnnamed_2 {
                type_0: 0,
                type_flags: 0,
                u: C2RustUnnamed_3 { extra: 0 },
            },
        },
        u2: C2RustUnnamed_0 { next: 0 },
    }; 3];
    if opaque.is_null() {
        return 0 as libc::c_int;
    }
    if ((*cbs).stats).is_null() {
        return 0 as libc::c_int;
    }
    args[0 as libc::c_int as usize].u1.type_info = 1 as libc::c_int as uint32_t;
    args[1 as libc::c_int as usize].u1.type_info = 1 as libc::c_int as uint32_t;
    args[2 as libc::c_int as usize].u1.type_info = 1 as libc::c_int as uint32_t;
    let mut __z: *mut zval = &mut *args.as_mut_ptr().offset(0 as libc::c_int as isize)
        as *mut zval;
    let mut __zv: *mut zval = &mut (*cbs).zrk;
    if !(zval_get_type(__zv) as libc::c_int == 10 as libc::c_int) as libc::c_int
        as libc::c_long != 0
    {
        if 1 as libc::c_int != 0 && 0 as libc::c_int == 0 {
            let mut _z1: *mut zval = __z;
            let mut _z2: *const zval = __zv;
            let mut _gc: *mut zend_refcounted = (*_z2).value.counted;
            let mut _t: uint32_t = (*_z2).u1.type_info;
            (*_z1).value.counted = _gc;
            (*_z1).u1.type_info = _t;
            if _t & 0xff00 as libc::c_int as libc::c_uint
                != 0 as libc::c_int as libc::c_uint
            {
                zend_gc_addref(&mut (*_gc).gc);
            }
        } else {
            let mut _z1_0: *mut zval = __z;
            let mut _z2_0: *const zval = __zv;
            let mut _gc_0: *mut zend_refcounted = (*_z2_0).value.counted;
            let mut _t_0: uint32_t = (*_z2_0).u1.type_info;
            (*_z1_0).value.counted = _gc_0;
            (*_z1_0).u1.type_info = _t_0;
        }
    } else {
        let mut _z1_1: *mut zval = __z;
        let mut _z2_1: *const zval = &mut (*(*__zv).value.ref_0).val;
        let mut _gc_1: *mut zend_refcounted = (*_z2_1).value.counted;
        let mut _t_1: uint32_t = (*_z2_1).u1.type_info;
        (*_z1_1).value.counted = _gc_1;
        (*_z1_1).u1.type_info = _t_1;
        if _t_1 & 0xff00 as libc::c_int as libc::c_uint
            != 0 as libc::c_int as libc::c_uint
        {
            zend_gc_addref(&mut (*_gc_1).gc);
        }
        if 0 as libc::c_int != 0 || 1 as libc::c_int == 0 {
            zval_ptr_dtor(__zv);
        }
    }
    let mut _s: *const libc::c_char = json;
    let mut __z_0: *mut zval = &mut *args.as_mut_ptr().offset(1 as libc::c_int as isize)
        as *mut zval;
    let mut __s: *mut zend_string = zend_string_init(_s, strlen(_s), 0 as libc::c_int);
    (*__z_0).value.str_0 = __s;
    (*__z_0)
        .u1
        .type_info = (6 as libc::c_int
        | ((1 as libc::c_int) << 0 as libc::c_int) << 8 as libc::c_int) as uint32_t;
    let mut __z_1: *mut zval = &mut *args.as_mut_ptr().offset(2 as libc::c_int as isize)
        as *mut zval;
    (*__z_1).value.lval = json_len as zend_long;
    (*__z_1).u1.type_info = 4 as libc::c_int as uint32_t;
    rdkafka_call_function(
        &mut (*(*cbs).stats).fci,
        &mut (*(*cbs).stats).fcc,
        0 as *mut zval,
        3 as libc::c_int as uint32_t,
        args.as_mut_ptr(),
    );
    zval_ptr_dtor(&mut *args.as_mut_ptr().offset(0 as libc::c_int as isize));
    zval_ptr_dtor(&mut *args.as_mut_ptr().offset(1 as libc::c_int as isize));
    zval_ptr_dtor(&mut *args.as_mut_ptr().offset(2 as libc::c_int as isize));
    return 0 as libc::c_int;
}
unsafe extern "C" fn kafka_conf_rebalance_cb(
    mut rk: *mut rd_kafka_t,
    mut err: rd_kafka_resp_err_t,
    mut partitions: *mut rd_kafka_topic_partition_list_t,
    mut opaque: *mut libc::c_void,
) {
    let mut cbs: *mut kafka_conf_callbacks = opaque as *mut kafka_conf_callbacks;
    let mut args: [zval; 3] = [zval {
        value: _zend_value { lval: 0 },
        u1: C2RustUnnamed_1 {
            v: C2RustUnnamed_2 {
                type_0: 0,
                type_flags: 0,
                u: C2RustUnnamed_3 { extra: 0 },
            },
        },
        u2: C2RustUnnamed_0 { next: 0 },
    }; 3];
    if opaque.is_null() {
        return;
    }
    if ((*cbs).rebalance).is_null() {
        err = rd_kafka_assign(rk, 0 as *const rd_kafka_topic_partition_list_t);
        if err as libc::c_int != RD_KAFKA_RESP_ERR_NO_ERROR as libc::c_int {
            zend_throw_exception(
                ce_kafka_exception,
                rd_kafka_err2str(err),
                err as zend_long,
            );
            return;
        }
        return;
    }
    args[0 as libc::c_int as usize].u1.type_info = 1 as libc::c_int as uint32_t;
    args[1 as libc::c_int as usize].u1.type_info = 1 as libc::c_int as uint32_t;
    args[2 as libc::c_int as usize].u1.type_info = 1 as libc::c_int as uint32_t;
    let mut __z: *mut zval = &mut *args.as_mut_ptr().offset(0 as libc::c_int as isize)
        as *mut zval;
    let mut __zv: *mut zval = &mut (*cbs).zrk;
    if !(zval_get_type(__zv) as libc::c_int == 10 as libc::c_int) as libc::c_int
        as libc::c_long != 0
    {
        if 1 as libc::c_int != 0 && 0 as libc::c_int == 0 {
            let mut _z1: *mut zval = __z;
            let mut _z2: *const zval = __zv;
            let mut _gc: *mut zend_refcounted = (*_z2).value.counted;
            let mut _t: uint32_t = (*_z2).u1.type_info;
            (*_z1).value.counted = _gc;
            (*_z1).u1.type_info = _t;
            if _t & 0xff00 as libc::c_int as libc::c_uint
                != 0 as libc::c_int as libc::c_uint
            {
                zend_gc_addref(&mut (*_gc).gc);
            }
        } else {
            let mut _z1_0: *mut zval = __z;
            let mut _z2_0: *const zval = __zv;
            let mut _gc_0: *mut zend_refcounted = (*_z2_0).value.counted;
            let mut _t_0: uint32_t = (*_z2_0).u1.type_info;
            (*_z1_0).value.counted = _gc_0;
            (*_z1_0).u1.type_info = _t_0;
        }
    } else {
        let mut _z1_1: *mut zval = __z;
        let mut _z2_1: *const zval = &mut (*(*__zv).value.ref_0).val;
        let mut _gc_1: *mut zend_refcounted = (*_z2_1).value.counted;
        let mut _t_1: uint32_t = (*_z2_1).u1.type_info;
        (*_z1_1).value.counted = _gc_1;
        (*_z1_1).u1.type_info = _t_1;
        if _t_1 & 0xff00 as libc::c_int as libc::c_uint
            != 0 as libc::c_int as libc::c_uint
        {
            zend_gc_addref(&mut (*_gc_1).gc);
        }
        if 0 as libc::c_int != 0 || 1 as libc::c_int == 0 {
            zval_ptr_dtor(__zv);
        }
    }
    let mut __z_0: *mut zval = &mut *args.as_mut_ptr().offset(1 as libc::c_int as isize)
        as *mut zval;
    (*__z_0).value.lval = err as zend_long;
    (*__z_0).u1.type_info = 4 as libc::c_int as uint32_t;
    kafka_topic_partition_list_to_array(
        &mut *args.as_mut_ptr().offset(2 as libc::c_int as isize),
        partitions,
    );
    rdkafka_call_function(
        &mut (*(*cbs).rebalance).fci,
        &mut (*(*cbs).rebalance).fcc,
        0 as *mut zval,
        3 as libc::c_int as uint32_t,
        args.as_mut_ptr(),
    );
    zval_ptr_dtor(&mut *args.as_mut_ptr().offset(0 as libc::c_int as isize));
    zval_ptr_dtor(&mut *args.as_mut_ptr().offset(1 as libc::c_int as isize));
    zval_ptr_dtor(&mut *args.as_mut_ptr().offset(2 as libc::c_int as isize));
}
unsafe extern "C" fn kafka_conf_consume_cb(
    mut msg: *mut rd_kafka_message_t,
    mut opaque: *mut libc::c_void,
) {
    let mut cbs: *mut kafka_conf_callbacks = opaque as *mut kafka_conf_callbacks;
    let mut args: [zval; 2] = [zval {
        value: _zend_value { lval: 0 },
        u1: C2RustUnnamed_1 {
            v: C2RustUnnamed_2 {
                type_0: 0,
                type_flags: 0,
                u: C2RustUnnamed_3 { extra: 0 },
            },
        },
        u2: C2RustUnnamed_0 { next: 0 },
    }; 2];
    if opaque.is_null() {
        return;
    }
    if ((*cbs).consume).is_null() {
        return;
    }
    args[0 as libc::c_int as usize].u1.type_info = 1 as libc::c_int as uint32_t;
    args[1 as libc::c_int as usize].u1.type_info = 1 as libc::c_int as uint32_t;
    kafka_message_new(
        &mut *args.as_mut_ptr().offset(0 as libc::c_int as isize),
        msg,
        0 as *mut zend_string,
    );
    let mut __z: *mut zval = &mut *args.as_mut_ptr().offset(1 as libc::c_int as isize)
        as *mut zval;
    let mut __zv: *mut zval = &mut (*cbs).zrk;
    if !(zval_get_type(__zv) as libc::c_int == 10 as libc::c_int) as libc::c_int
        as libc::c_long != 0
    {
        if 1 as libc::c_int != 0 && 0 as libc::c_int == 0 {
            let mut _z1: *mut zval = __z;
            let mut _z2: *const zval = __zv;
            let mut _gc: *mut zend_refcounted = (*_z2).value.counted;
            let mut _t: uint32_t = (*_z2).u1.type_info;
            (*_z1).value.counted = _gc;
            (*_z1).u1.type_info = _t;
            if _t & 0xff00 as libc::c_int as libc::c_uint
                != 0 as libc::c_int as libc::c_uint
            {
                zend_gc_addref(&mut (*_gc).gc);
            }
        } else {
            let mut _z1_0: *mut zval = __z;
            let mut _z2_0: *const zval = __zv;
            let mut _gc_0: *mut zend_refcounted = (*_z2_0).value.counted;
            let mut _t_0: uint32_t = (*_z2_0).u1.type_info;
            (*_z1_0).value.counted = _gc_0;
            (*_z1_0).u1.type_info = _t_0;
        }
    } else {
        let mut _z1_1: *mut zval = __z;
        let mut _z2_1: *const zval = &mut (*(*__zv).value.ref_0).val;
        let mut _gc_1: *mut zend_refcounted = (*_z2_1).value.counted;
        let mut _t_1: uint32_t = (*_z2_1).u1.type_info;
        (*_z1_1).value.counted = _gc_1;
        (*_z1_1).u1.type_info = _t_1;
        if _t_1 & 0xff00 as libc::c_int as libc::c_uint
            != 0 as libc::c_int as libc::c_uint
        {
            zend_gc_addref(&mut (*_gc_1).gc);
        }
        if 0 as libc::c_int != 0 || 1 as libc::c_int == 0 {
            zval_ptr_dtor(__zv);
        }
    }
    rdkafka_call_function(
        &mut (*(*cbs).consume).fci,
        &mut (*(*cbs).consume).fcc,
        0 as *mut zval,
        2 as libc::c_int as uint32_t,
        args.as_mut_ptr(),
    );
    zval_ptr_dtor(&mut *args.as_mut_ptr().offset(0 as libc::c_int as isize));
    zval_ptr_dtor(&mut *args.as_mut_ptr().offset(1 as libc::c_int as isize));
}
unsafe extern "C" fn kafka_conf_offset_commit_cb(
    mut rk: *mut rd_kafka_t,
    mut err: rd_kafka_resp_err_t,
    mut partitions: *mut rd_kafka_topic_partition_list_t,
    mut opaque: *mut libc::c_void,
) {
    let mut cbs: *mut kafka_conf_callbacks = opaque as *mut kafka_conf_callbacks;
    let mut args: [zval; 3] = [zval {
        value: _zend_value { lval: 0 },
        u1: C2RustUnnamed_1 {
            v: C2RustUnnamed_2 {
                type_0: 0,
                type_flags: 0,
                u: C2RustUnnamed_3 { extra: 0 },
            },
        },
        u2: C2RustUnnamed_0 { next: 0 },
    }; 3];
    if opaque.is_null() {
        return;
    }
    if ((*cbs).offset_commit).is_null() {
        return;
    }
    args[0 as libc::c_int as usize].u1.type_info = 1 as libc::c_int as uint32_t;
    args[1 as libc::c_int as usize].u1.type_info = 1 as libc::c_int as uint32_t;
    args[2 as libc::c_int as usize].u1.type_info = 1 as libc::c_int as uint32_t;
    let mut __z: *mut zval = &mut *args.as_mut_ptr().offset(0 as libc::c_int as isize)
        as *mut zval;
    let mut __zv: *mut zval = &mut (*cbs).zrk;
    if !(zval_get_type(__zv) as libc::c_int == 10 as libc::c_int) as libc::c_int
        as libc::c_long != 0
    {
        if 1 as libc::c_int != 0 && 0 as libc::c_int == 0 {
            let mut _z1: *mut zval = __z;
            let mut _z2: *const zval = __zv;
            let mut _gc: *mut zend_refcounted = (*_z2).value.counted;
            let mut _t: uint32_t = (*_z2).u1.type_info;
            (*_z1).value.counted = _gc;
            (*_z1).u1.type_info = _t;
            if _t & 0xff00 as libc::c_int as libc::c_uint
                != 0 as libc::c_int as libc::c_uint
            {
                zend_gc_addref(&mut (*_gc).gc);
            }
        } else {
            let mut _z1_0: *mut zval = __z;
            let mut _z2_0: *const zval = __zv;
            let mut _gc_0: *mut zend_refcounted = (*_z2_0).value.counted;
            let mut _t_0: uint32_t = (*_z2_0).u1.type_info;
            (*_z1_0).value.counted = _gc_0;
            (*_z1_0).u1.type_info = _t_0;
        }
    } else {
        let mut _z1_1: *mut zval = __z;
        let mut _z2_1: *const zval = &mut (*(*__zv).value.ref_0).val;
        let mut _gc_1: *mut zend_refcounted = (*_z2_1).value.counted;
        let mut _t_1: uint32_t = (*_z2_1).u1.type_info;
        (*_z1_1).value.counted = _gc_1;
        (*_z1_1).u1.type_info = _t_1;
        if _t_1 & 0xff00 as libc::c_int as libc::c_uint
            != 0 as libc::c_int as libc::c_uint
        {
            zend_gc_addref(&mut (*_gc_1).gc);
        }
        if 0 as libc::c_int != 0 || 1 as libc::c_int == 0 {
            zval_ptr_dtor(__zv);
        }
    }
    let mut __z_0: *mut zval = &mut *args.as_mut_ptr().offset(1 as libc::c_int as isize)
        as *mut zval;
    (*__z_0).value.lval = err as zend_long;
    (*__z_0).u1.type_info = 4 as libc::c_int as uint32_t;
    kafka_topic_partition_list_to_array(
        &mut *args.as_mut_ptr().offset(2 as libc::c_int as isize),
        partitions,
    );
    rdkafka_call_function(
        &mut (*(*cbs).offset_commit).fci,
        &mut (*(*cbs).offset_commit).fcc,
        0 as *mut zval,
        3 as libc::c_int as uint32_t,
        args.as_mut_ptr(),
    );
    zval_ptr_dtor(&mut *args.as_mut_ptr().offset(0 as libc::c_int as isize));
    zval_ptr_dtor(&mut *args.as_mut_ptr().offset(1 as libc::c_int as isize));
    zval_ptr_dtor(&mut *args.as_mut_ptr().offset(2 as libc::c_int as isize));
}
unsafe extern "C" fn kafka_conf_log_cb(
    mut rk: *const rd_kafka_t,
    mut level: libc::c_int,
    mut facility: *const libc::c_char,
    mut message: *const libc::c_char,
) {
    let mut args: [zval; 4] = [zval {
        value: _zend_value { lval: 0 },
        u1: C2RustUnnamed_1 {
            v: C2RustUnnamed_2 {
                type_0: 0,
                type_flags: 0,
                u: C2RustUnnamed_3 { extra: 0 },
            },
        },
        u2: C2RustUnnamed_0 { next: 0 },
    }; 4];
    let mut cbs: *mut kafka_conf_callbacks = rd_kafka_opaque(rk)
        as *mut kafka_conf_callbacks;
    if ((*cbs).log).is_null() {
        return;
    }
    args[0 as libc::c_int as usize].u1.type_info = 1 as libc::c_int as uint32_t;
    args[1 as libc::c_int as usize].u1.type_info = 1 as libc::c_int as uint32_t;
    args[2 as libc::c_int as usize].u1.type_info = 1 as libc::c_int as uint32_t;
    args[3 as libc::c_int as usize].u1.type_info = 1 as libc::c_int as uint32_t;
    let mut __z: *mut zval = &mut *args.as_mut_ptr().offset(0 as libc::c_int as isize)
        as *mut zval;
    let mut __zv: *mut zval = &mut (*cbs).zrk;
    if !(zval_get_type(__zv) as libc::c_int == 10 as libc::c_int) as libc::c_int
        as libc::c_long != 0
    {
        if 1 as libc::c_int != 0 && 0 as libc::c_int == 0 {
            let mut _z1: *mut zval = __z;
            let mut _z2: *const zval = __zv;
            let mut _gc: *mut zend_refcounted = (*_z2).value.counted;
            let mut _t: uint32_t = (*_z2).u1.type_info;
            (*_z1).value.counted = _gc;
            (*_z1).u1.type_info = _t;
            if _t & 0xff00 as libc::c_int as libc::c_uint
                != 0 as libc::c_int as libc::c_uint
            {
                zend_gc_addref(&mut (*_gc).gc);
            }
        } else {
            let mut _z1_0: *mut zval = __z;
            let mut _z2_0: *const zval = __zv;
            let mut _gc_0: *mut zend_refcounted = (*_z2_0).value.counted;
            let mut _t_0: uint32_t = (*_z2_0).u1.type_info;
            (*_z1_0).value.counted = _gc_0;
            (*_z1_0).u1.type_info = _t_0;
        }
    } else {
        let mut _z1_1: *mut zval = __z;
        let mut _z2_1: *const zval = &mut (*(*__zv).value.ref_0).val;
        let mut _gc_1: *mut zend_refcounted = (*_z2_1).value.counted;
        let mut _t_1: uint32_t = (*_z2_1).u1.type_info;
        (*_z1_1).value.counted = _gc_1;
        (*_z1_1).u1.type_info = _t_1;
        if _t_1 & 0xff00 as libc::c_int as libc::c_uint
            != 0 as libc::c_int as libc::c_uint
        {
            zend_gc_addref(&mut (*_gc_1).gc);
        }
        if 0 as libc::c_int != 0 || 1 as libc::c_int == 0 {
            zval_ptr_dtor(__zv);
        }
    }
    let mut __z_0: *mut zval = &mut *args.as_mut_ptr().offset(1 as libc::c_int as isize)
        as *mut zval;
    (*__z_0).value.lval = level as zend_long;
    (*__z_0).u1.type_info = 4 as libc::c_int as uint32_t;
    let mut _s: *const libc::c_char = facility;
    let mut __z_1: *mut zval = &mut *args.as_mut_ptr().offset(2 as libc::c_int as isize)
        as *mut zval;
    let mut __s: *mut zend_string = zend_string_init(_s, strlen(_s), 0 as libc::c_int);
    (*__z_1).value.str_0 = __s;
    (*__z_1)
        .u1
        .type_info = (6 as libc::c_int
        | ((1 as libc::c_int) << 0 as libc::c_int) << 8 as libc::c_int) as uint32_t;
    let mut _s_0: *const libc::c_char = message;
    let mut __z_2: *mut zval = &mut *args.as_mut_ptr().offset(3 as libc::c_int as isize)
        as *mut zval;
    let mut __s_0: *mut zend_string = zend_string_init(
        _s_0,
        strlen(_s_0),
        0 as libc::c_int,
    );
    (*__z_2).value.str_0 = __s_0;
    (*__z_2)
        .u1
        .type_info = (6 as libc::c_int
        | ((1 as libc::c_int) << 0 as libc::c_int) << 8 as libc::c_int) as uint32_t;
    rdkafka_call_function(
        &mut (*(*cbs).log).fci,
        &mut (*(*cbs).log).fcc,
        0 as *mut zval,
        4 as libc::c_int as uint32_t,
        args.as_mut_ptr(),
    );
    zval_ptr_dtor(&mut *args.as_mut_ptr().offset(0 as libc::c_int as isize));
    zval_ptr_dtor(&mut *args.as_mut_ptr().offset(1 as libc::c_int as isize));
    zval_ptr_dtor(&mut *args.as_mut_ptr().offset(2 as libc::c_int as isize));
    zval_ptr_dtor(&mut *args.as_mut_ptr().offset(3 as libc::c_int as isize));
}
unsafe extern "C" fn kafka_conf_set_oauthbearer_token_refresh_cb(
    mut rk: *mut rd_kafka_t,
    mut oauthbearer_config: *const libc::c_char,
    mut opaque: *mut libc::c_void,
) {
    let mut cbs: *mut kafka_conf_callbacks = opaque as *mut kafka_conf_callbacks;
    let mut args: [zval; 2] = [zval {
        value: _zend_value { lval: 0 },
        u1: C2RustUnnamed_1 {
            v: C2RustUnnamed_2 {
                type_0: 0,
                type_flags: 0,
                u: C2RustUnnamed_3 { extra: 0 },
            },
        },
        u2: C2RustUnnamed_0 { next: 0 },
    }; 2];
    if opaque.is_null() {
        return;
    }
    if ((*cbs).oauthbearer_token_refresh).is_null() {
        return;
    }
    args[0 as libc::c_int as usize].u1.type_info = 1 as libc::c_int as uint32_t;
    args[1 as libc::c_int as usize].u1.type_info = 1 as libc::c_int as uint32_t;
    let mut __z: *mut zval = &mut *args.as_mut_ptr().offset(0 as libc::c_int as isize)
        as *mut zval;
    let mut __zv: *mut zval = &mut (*cbs).zrk;
    if !(zval_get_type(__zv) as libc::c_int == 10 as libc::c_int) as libc::c_int
        as libc::c_long != 0
    {
        if 1 as libc::c_int != 0 && 0 as libc::c_int == 0 {
            let mut _z1: *mut zval = __z;
            let mut _z2: *const zval = __zv;
            let mut _gc: *mut zend_refcounted = (*_z2).value.counted;
            let mut _t: uint32_t = (*_z2).u1.type_info;
            (*_z1).value.counted = _gc;
            (*_z1).u1.type_info = _t;
            if _t & 0xff00 as libc::c_int as libc::c_uint
                != 0 as libc::c_int as libc::c_uint
            {
                zend_gc_addref(&mut (*_gc).gc);
            }
        } else {
            let mut _z1_0: *mut zval = __z;
            let mut _z2_0: *const zval = __zv;
            let mut _gc_0: *mut zend_refcounted = (*_z2_0).value.counted;
            let mut _t_0: uint32_t = (*_z2_0).u1.type_info;
            (*_z1_0).value.counted = _gc_0;
            (*_z1_0).u1.type_info = _t_0;
        }
    } else {
        let mut _z1_1: *mut zval = __z;
        let mut _z2_1: *const zval = &mut (*(*__zv).value.ref_0).val;
        let mut _gc_1: *mut zend_refcounted = (*_z2_1).value.counted;
        let mut _t_1: uint32_t = (*_z2_1).u1.type_info;
        (*_z1_1).value.counted = _gc_1;
        (*_z1_1).u1.type_info = _t_1;
        if _t_1 & 0xff00 as libc::c_int as libc::c_uint
            != 0 as libc::c_int as libc::c_uint
        {
            zend_gc_addref(&mut (*_gc_1).gc);
        }
        if 0 as libc::c_int != 0 || 1 as libc::c_int == 0 {
            zval_ptr_dtor(__zv);
        }
    }
    let mut _s: *const libc::c_char = oauthbearer_config;
    let mut __z_0: *mut zval = &mut *args.as_mut_ptr().offset(1 as libc::c_int as isize)
        as *mut zval;
    let mut __s: *mut zend_string = zend_string_init(_s, strlen(_s), 0 as libc::c_int);
    (*__z_0).value.str_0 = __s;
    (*__z_0)
        .u1
        .type_info = (6 as libc::c_int
        | ((1 as libc::c_int) << 0 as libc::c_int) << 8 as libc::c_int) as uint32_t;
    rdkafka_call_function(
        &mut (*(*cbs).oauthbearer_token_refresh).fci,
        &mut (*(*cbs).oauthbearer_token_refresh).fcc,
        0 as *mut zval,
        2 as libc::c_int as uint32_t,
        args.as_mut_ptr(),
    );
    zval_ptr_dtor(&mut *args.as_mut_ptr().offset(0 as libc::c_int as isize));
    zval_ptr_dtor(&mut *args.as_mut_ptr().offset(1 as libc::c_int as isize));
}
pub unsafe extern "C" fn zim_RdKafka_Conf___construct(
    mut execute_data: *mut zend_execute_data,
    mut return_value: *mut zval,
) {
    let mut intern: *mut kafka_conf_object = 0 as *mut kafka_conf_object;
    let mut error_handling: zend_error_handling = zend_error_handling {
        handling: EH_NORMAL,
        exception: 0 as *mut zend_class_entry,
        user_handler: zval {
            value: _zend_value { lval: 0 },
            u1: C2RustUnnamed_1 {
                v: C2RustUnnamed_2 {
                    type_0: 0,
                    type_flags: 0,
                    u: C2RustUnnamed_3 { extra: 0 },
                },
            },
            u2: C2RustUnnamed_0 { next: 0 },
        },
    };
    zend_replace_error_handling(
        EH_THROW,
        spl_ce_InvalidArgumentException,
        &mut error_handling,
    );
    if zend_parse_parameters(
        (*execute_data).This.u2.num_args as libc::c_int,
        b"\0" as *const u8 as *const libc::c_char,
    ) == FAILURE as libc::c_int
    {
        zend_restore_error_handling(&mut error_handling);
        return;
    }
    intern = ((*(if zval_get_type(&mut (*execute_data).This) as libc::c_int
        == 8 as libc::c_int
    {
        &mut (*execute_data).This as *mut zval
    } else {
        0 as *mut zval
    }))
        .value
        .obj as *mut libc::c_char)
        .offset(-(96 as libc::c_ulong as isize)) as *mut kafka_conf_object;
    (*intern).type_0 = KAFKA_CONF;
    (*intern).u.conf = rd_kafka_conf_new();
    zend_restore_error_handling(&mut error_handling);
}
pub unsafe extern "C" fn zim_RdKafka_Conf_dump(
    mut execute_data: *mut zend_execute_data,
    mut return_value: *mut zval,
) {
    let mut cntp: size_t = 0;
    let mut dump: *mut *const libc::c_char = 0 as *mut *const libc::c_char;
    let mut intern: *mut kafka_conf_object = 0 as *mut kafka_conf_object;
    let mut i: size_t = 0;
    if zend_parse_parameters(
        (*execute_data).This.u2.num_args as libc::c_int,
        b"\0" as *const u8 as *const libc::c_char,
    ) == FAILURE as libc::c_int
    {
        return;
    }
    intern = get_kafka_conf_object(
        if zval_get_type(&mut (*execute_data).This) as libc::c_int == 8 as libc::c_int {
            &mut (*execute_data).This
        } else {
            0 as *mut zval
        },
    );
    if intern.is_null() {
        return;
    }
    match (*intern).type_0 as libc::c_uint {
        1 => {
            dump = rd_kafka_conf_dump((*intern).u.conf, &mut cntp);
        }
        2 => {
            dump = rd_kafka_topic_conf_dump((*intern).u.topic_conf, &mut cntp);
        }
        _ => return,
    }
    let mut __arr: *mut zend_array = if 0 != 0 {
        if 0 as libc::c_int as uint32_t <= 8 as libc::c_int as libc::c_uint {
            _zend_new_array_0()
        } else {
            _zend_new_array(0 as libc::c_int as uint32_t)
        }
    } else {
        _zend_new_array(0 as libc::c_int as uint32_t)
    };
    let mut __z: *mut zval = return_value;
    (*__z).value.arr = __arr;
    (*__z)
        .u1
        .type_info = (7 as libc::c_int
        | ((1 as libc::c_int) << 0 as libc::c_int) << 8 as libc::c_int
        | ((1 as libc::c_int) << 1 as libc::c_int) << 8 as libc::c_int) as uint32_t;
    i = 0 as libc::c_int as size_t;
    while i < cntp {
        let mut key: *const libc::c_char = *dump.offset(i as isize);
        let mut value: *const libc::c_char = *dump
            .offset(i.wrapping_add(1 as libc::c_int as libc::c_ulong) as isize);
        add_assoc_string_ex(
            return_value,
            key as *mut libc::c_char,
            strlen(key as *mut libc::c_char),
            value as *mut libc::c_char,
        );
        i = (i as libc::c_ulong).wrapping_add(2 as libc::c_int as libc::c_ulong)
            as size_t as size_t;
    }
    rd_kafka_conf_dump_free(dump, cntp);
}
pub unsafe extern "C" fn zim_RdKafka_Conf_set(
    mut execute_data: *mut zend_execute_data,
    mut return_value: *mut zval,
) {
    let mut name: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut name_len: size_t = 0;
    let mut value: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut value_len: size_t = 0;
    let mut intern: *mut kafka_conf_object = 0 as *mut kafka_conf_object;
    let mut ret: rd_kafka_conf_res_t = RD_KAFKA_CONF_OK;
    let mut errstr: [libc::c_char; 512] = [0; 512];
    if zend_parse_parameters(
        (*execute_data).This.u2.num_args as libc::c_int,
        b"ss\0" as *const u8 as *const libc::c_char,
        &mut name as *mut *mut libc::c_char,
        &mut name_len as *mut size_t,
        &mut value as *mut *mut libc::c_char,
        &mut value_len as *mut size_t,
    ) == FAILURE as libc::c_int
    {
        return;
    }
    intern = get_kafka_conf_object(
        if zval_get_type(&mut (*execute_data).This) as libc::c_int == 8 as libc::c_int {
            &mut (*execute_data).This
        } else {
            0 as *mut zval
        },
    );
    if intern.is_null() {
        return;
    }
    errstr[0 as libc::c_int as usize] = '\0' as i32 as libc::c_char;
    match (*intern).type_0 as libc::c_uint {
        1 => {
            ret = rd_kafka_conf_set(
                (*intern).u.conf,
                name,
                value,
                errstr.as_mut_ptr(),
                ::std::mem::size_of::<[libc::c_char; 512]>() as libc::c_ulong,
            );
        }
        2 => {
            ret = rd_kafka_topic_conf_set(
                (*intern).u.topic_conf,
                name,
                value,
                errstr.as_mut_ptr(),
                ::std::mem::size_of::<[libc::c_char; 512]>() as libc::c_ulong,
            );
        }
        _ => {}
    }
    match ret as libc::c_int {
        -2 => {
            zend_throw_exception(
                ce_kafka_exception,
                errstr.as_mut_ptr(),
                RD_KAFKA_CONF_UNKNOWN as libc::c_int as zend_long,
            );
            return;
        }
        -1 => {
            zend_throw_exception(
                ce_kafka_exception,
                errstr.as_mut_ptr(),
                RD_KAFKA_CONF_INVALID as libc::c_int as zend_long,
            );
            return;
        }
        0 | _ => {}
    };
}
pub unsafe extern "C" fn zim_RdKafka_Conf_setDefaultTopicConf(
    mut execute_data: *mut zend_execute_data,
    mut return_value: *mut zval,
) {
    let mut ztopic_conf: *mut zval = 0 as *mut zval;
    let mut intern: *mut kafka_conf_object = 0 as *mut kafka_conf_object;
    let mut topic_conf_intern: *mut kafka_conf_object = 0 as *mut kafka_conf_object;
    let mut topic_conf: *mut rd_kafka_topic_conf_t = 0 as *mut rd_kafka_topic_conf_t;
    if zend_parse_parameters(
        (*execute_data).This.u2.num_args as libc::c_int,
        b"O\0" as *const u8 as *const libc::c_char,
        &mut ztopic_conf as *mut *mut zval,
        ce_kafka_topic_conf,
    ) == FAILURE as libc::c_int
    {
        return;
    }
    intern = get_kafka_conf_object(
        if zval_get_type(&mut (*execute_data).This) as libc::c_int == 8 as libc::c_int {
            &mut (*execute_data).This
        } else {
            0 as *mut zval
        },
    );
    if intern.is_null() {
        return;
    }
    topic_conf_intern = get_kafka_conf_object(ztopic_conf);
    if topic_conf_intern.is_null() {
        return;
    }
    topic_conf = rd_kafka_topic_conf_dup((*topic_conf_intern).u.topic_conf);
    rd_kafka_conf_set_default_topic_conf((*intern).u.conf, topic_conf);
}
pub unsafe extern "C" fn zim_RdKafka_Conf_setErrorCb(
    mut execute_data: *mut zend_execute_data,
    mut return_value: *mut zval,
) {
    let mut fci: zend_fcall_info = zend_fcall_info {
        size: 0,
        function_name: zval {
            value: _zend_value { lval: 0 },
            u1: C2RustUnnamed_1 {
                v: C2RustUnnamed_2 {
                    type_0: 0,
                    type_flags: 0,
                    u: C2RustUnnamed_3 { extra: 0 },
                },
            },
            u2: C2RustUnnamed_0 { next: 0 },
        },
        retval: 0 as *mut zval,
        params: 0 as *mut zval,
        object: 0 as *mut zend_object,
        no_separation: 0,
        param_count: 0,
    };
    let mut fcc: zend_fcall_info_cache = zend_fcall_info_cache {
        function_handler: 0 as *mut zend_function,
        calling_scope: 0 as *mut zend_class_entry,
        called_scope: 0 as *mut zend_class_entry,
        object: 0 as *mut zend_object,
    };
    let mut intern: *mut kafka_conf_object = 0 as *mut kafka_conf_object;
    if zend_parse_parameters(
        (*execute_data).This.u2.num_args as libc::c_int,
        b"f\0" as *const u8 as *const libc::c_char,
        &mut fci as *mut zend_fcall_info,
        &mut fcc as *mut zend_fcall_info_cache,
    ) == FAILURE as libc::c_int
    {
        return;
    }
    intern = get_kafka_conf_object(
        if zval_get_type(&mut (*execute_data).This) as libc::c_int == 8 as libc::c_int {
            &mut (*execute_data).This
        } else {
            0 as *mut zval
        },
    );
    if intern.is_null() {
        return;
    }
    zval_addref_p(&mut fci.function_name);
    if !((*intern).cbs.error).is_null() {
        zval_ptr_dtor(&mut (*(*intern).cbs.error).fci.function_name);
    } else {
        (*intern)
            .cbs
            .error = _ecalloc(
            1 as libc::c_int as size_t,
            ::std::mem::size_of::<kafka_conf_callback>() as libc::c_ulong,
        ) as *mut kafka_conf_callback;
    }
    (*(*intern).cbs.error).fci = fci;
    (*(*intern).cbs.error).fcc = fcc;
    rd_kafka_conf_set_error_cb(
        (*intern).u.conf,
        Some(
            kafka_conf_error_cb
                as unsafe extern "C" fn(
                    *mut rd_kafka_t,
                    libc::c_int,
                    *const libc::c_char,
                    *mut libc::c_void,
                ) -> (),
        ),
    );
}
pub unsafe extern "C" fn zim_RdKafka_Conf_setDrMsgCb(
    mut execute_data: *mut zend_execute_data,
    mut return_value: *mut zval,
) {
    let mut fci: zend_fcall_info = zend_fcall_info {
        size: 0,
        function_name: zval {
            value: _zend_value { lval: 0 },
            u1: C2RustUnnamed_1 {
                v: C2RustUnnamed_2 {
                    type_0: 0,
                    type_flags: 0,
                    u: C2RustUnnamed_3 { extra: 0 },
                },
            },
            u2: C2RustUnnamed_0 { next: 0 },
        },
        retval: 0 as *mut zval,
        params: 0 as *mut zval,
        object: 0 as *mut zend_object,
        no_separation: 0,
        param_count: 0,
    };
    let mut fcc: zend_fcall_info_cache = zend_fcall_info_cache {
        function_handler: 0 as *mut zend_function,
        calling_scope: 0 as *mut zend_class_entry,
        called_scope: 0 as *mut zend_class_entry,
        object: 0 as *mut zend_object,
    };
    let mut intern: *mut kafka_conf_object = 0 as *mut kafka_conf_object;
    if zend_parse_parameters(
        (*execute_data).This.u2.num_args as libc::c_int,
        b"f\0" as *const u8 as *const libc::c_char,
        &mut fci as *mut zend_fcall_info,
        &mut fcc as *mut zend_fcall_info_cache,
    ) == FAILURE as libc::c_int
    {
        return;
    }
    intern = get_kafka_conf_object(
        if zval_get_type(&mut (*execute_data).This) as libc::c_int == 8 as libc::c_int {
            &mut (*execute_data).This
        } else {
            0 as *mut zval
        },
    );
    if intern.is_null() {
        return;
    }
    zval_addref_p(&mut fci.function_name);
    if !((*intern).cbs.dr_msg).is_null() {
        zval_ptr_dtor(&mut (*(*intern).cbs.dr_msg).fci.function_name);
    } else {
        (*intern)
            .cbs
            .dr_msg = _ecalloc(
            1 as libc::c_int as size_t,
            ::std::mem::size_of::<kafka_conf_callback>() as libc::c_ulong,
        ) as *mut kafka_conf_callback;
    }
    (*(*intern).cbs.dr_msg).fci = fci;
    (*(*intern).cbs.dr_msg).fcc = fcc;
    rd_kafka_conf_set_dr_msg_cb(
        (*intern).u.conf,
        Some(
            kafka_conf_dr_msg_cb
                as unsafe extern "C" fn(
                    *mut rd_kafka_t,
                    *const rd_kafka_message_t,
                    *mut libc::c_void,
                ) -> (),
        ),
    );
}
pub unsafe extern "C" fn zim_RdKafka_Conf_setStatsCb(
    mut execute_data: *mut zend_execute_data,
    mut return_value: *mut zval,
) {
    let mut fci: zend_fcall_info = zend_fcall_info {
        size: 0,
        function_name: zval {
            value: _zend_value { lval: 0 },
            u1: C2RustUnnamed_1 {
                v: C2RustUnnamed_2 {
                    type_0: 0,
                    type_flags: 0,
                    u: C2RustUnnamed_3 { extra: 0 },
                },
            },
            u2: C2RustUnnamed_0 { next: 0 },
        },
        retval: 0 as *mut zval,
        params: 0 as *mut zval,
        object: 0 as *mut zend_object,
        no_separation: 0,
        param_count: 0,
    };
    let mut fcc: zend_fcall_info_cache = zend_fcall_info_cache {
        function_handler: 0 as *mut zend_function,
        calling_scope: 0 as *mut zend_class_entry,
        called_scope: 0 as *mut zend_class_entry,
        object: 0 as *mut zend_object,
    };
    let mut intern: *mut kafka_conf_object = 0 as *mut kafka_conf_object;
    if zend_parse_parameters(
        (*execute_data).This.u2.num_args as libc::c_int,
        b"f\0" as *const u8 as *const libc::c_char,
        &mut fci as *mut zend_fcall_info,
        &mut fcc as *mut zend_fcall_info_cache,
    ) == FAILURE as libc::c_int
    {
        return;
    }
    intern = get_kafka_conf_object(
        if zval_get_type(&mut (*execute_data).This) as libc::c_int == 8 as libc::c_int {
            &mut (*execute_data).This
        } else {
            0 as *mut zval
        },
    );
    if intern.is_null() {
        return;
    }
    zval_addref_p(&mut fci.function_name);
    if !((*intern).cbs.stats).is_null() {
        zval_ptr_dtor(&mut (*(*intern).cbs.stats).fci.function_name);
    } else {
        (*intern)
            .cbs
            .stats = _ecalloc(
            1 as libc::c_int as size_t,
            ::std::mem::size_of::<kafka_conf_callback>() as libc::c_ulong,
        ) as *mut kafka_conf_callback;
    }
    (*(*intern).cbs.stats).fci = fci;
    (*(*intern).cbs.stats).fcc = fcc;
    rd_kafka_conf_set_stats_cb(
        (*intern).u.conf,
        Some(
            kafka_conf_stats_cb
                as unsafe extern "C" fn(
                    *mut rd_kafka_t,
                    *mut libc::c_char,
                    size_t,
                    *mut libc::c_void,
                ) -> libc::c_int,
        ),
    );
}
pub unsafe extern "C" fn zim_RdKafka_Conf_setRebalanceCb(
    mut execute_data: *mut zend_execute_data,
    mut return_value: *mut zval,
) {
    let mut fci: zend_fcall_info = zend_fcall_info {
        size: 0,
        function_name: zval {
            value: _zend_value { lval: 0 },
            u1: C2RustUnnamed_1 {
                v: C2RustUnnamed_2 {
                    type_0: 0,
                    type_flags: 0,
                    u: C2RustUnnamed_3 { extra: 0 },
                },
            },
            u2: C2RustUnnamed_0 { next: 0 },
        },
        retval: 0 as *mut zval,
        params: 0 as *mut zval,
        object: 0 as *mut zend_object,
        no_separation: 0,
        param_count: 0,
    };
    let mut fcc: zend_fcall_info_cache = zend_fcall_info_cache {
        function_handler: 0 as *mut zend_function,
        calling_scope: 0 as *mut zend_class_entry,
        called_scope: 0 as *mut zend_class_entry,
        object: 0 as *mut zend_object,
    };
    let mut intern: *mut kafka_conf_object = 0 as *mut kafka_conf_object;
    if zend_parse_parameters(
        (*execute_data).This.u2.num_args as libc::c_int,
        b"f\0" as *const u8 as *const libc::c_char,
        &mut fci as *mut zend_fcall_info,
        &mut fcc as *mut zend_fcall_info_cache,
    ) == FAILURE as libc::c_int
    {
        return;
    }
    intern = get_kafka_conf_object(
        if zval_get_type(&mut (*execute_data).This) as libc::c_int == 8 as libc::c_int {
            &mut (*execute_data).This
        } else {
            0 as *mut zval
        },
    );
    if intern.is_null() {
        return;
    }
    zval_addref_p(&mut fci.function_name);
    if !((*intern).cbs.rebalance).is_null() {
        zval_ptr_dtor(&mut (*(*intern).cbs.rebalance).fci.function_name);
    } else {
        (*intern)
            .cbs
            .rebalance = _ecalloc(
            1 as libc::c_int as size_t,
            ::std::mem::size_of::<kafka_conf_callback>() as libc::c_ulong,
        ) as *mut kafka_conf_callback;
    }
    (*(*intern).cbs.rebalance).fci = fci;
    (*(*intern).cbs.rebalance).fcc = fcc;
    rd_kafka_conf_set_rebalance_cb(
        (*intern).u.conf,
        Some(
            kafka_conf_rebalance_cb
                as unsafe extern "C" fn(
                    *mut rd_kafka_t,
                    rd_kafka_resp_err_t,
                    *mut rd_kafka_topic_partition_list_t,
                    *mut libc::c_void,
                ) -> (),
        ),
    );
}
pub unsafe extern "C" fn zim_RdKafka_Conf_setConsumeCb(
    mut execute_data: *mut zend_execute_data,
    mut return_value: *mut zval,
) {
    let mut fci: zend_fcall_info = zend_fcall_info {
        size: 0,
        function_name: zval {
            value: _zend_value { lval: 0 },
            u1: C2RustUnnamed_1 {
                v: C2RustUnnamed_2 {
                    type_0: 0,
                    type_flags: 0,
                    u: C2RustUnnamed_3 { extra: 0 },
                },
            },
            u2: C2RustUnnamed_0 { next: 0 },
        },
        retval: 0 as *mut zval,
        params: 0 as *mut zval,
        object: 0 as *mut zend_object,
        no_separation: 0,
        param_count: 0,
    };
    let mut fcc: zend_fcall_info_cache = zend_fcall_info_cache {
        function_handler: 0 as *mut zend_function,
        calling_scope: 0 as *mut zend_class_entry,
        called_scope: 0 as *mut zend_class_entry,
        object: 0 as *mut zend_object,
    };
    let mut intern: *mut kafka_conf_object = 0 as *mut kafka_conf_object;
    if zend_parse_parameters(
        (*execute_data).This.u2.num_args as libc::c_int,
        b"f\0" as *const u8 as *const libc::c_char,
        &mut fci as *mut zend_fcall_info,
        &mut fcc as *mut zend_fcall_info_cache,
    ) == FAILURE as libc::c_int
    {
        return;
    }
    intern = get_kafka_conf_object(
        if zval_get_type(&mut (*execute_data).This) as libc::c_int == 8 as libc::c_int {
            &mut (*execute_data).This
        } else {
            0 as *mut zval
        },
    );
    if intern.is_null() {
        return;
    }
    zval_addref_p(&mut fci.function_name);
    if !((*intern).cbs.consume).is_null() {
        zval_ptr_dtor(&mut (*(*intern).cbs.consume).fci.function_name);
    } else {
        (*intern)
            .cbs
            .consume = _ecalloc(
            1 as libc::c_int as size_t,
            ::std::mem::size_of::<kafka_conf_callback>() as libc::c_ulong,
        ) as *mut kafka_conf_callback;
    }
    (*(*intern).cbs.consume).fci = fci;
    (*(*intern).cbs.consume).fcc = fcc;
    rd_kafka_conf_set_consume_cb(
        (*intern).u.conf,
        Some(
            kafka_conf_consume_cb
                as unsafe extern "C" fn(*mut rd_kafka_message_t, *mut libc::c_void) -> (),
        ),
    );
}
pub unsafe extern "C" fn zim_RdKafka_Conf_setOffsetCommitCb(
    mut execute_data: *mut zend_execute_data,
    mut return_value: *mut zval,
) {
    let mut fci: zend_fcall_info = zend_fcall_info {
        size: 0,
        function_name: zval {
            value: _zend_value { lval: 0 },
            u1: C2RustUnnamed_1 {
                v: C2RustUnnamed_2 {
                    type_0: 0,
                    type_flags: 0,
                    u: C2RustUnnamed_3 { extra: 0 },
                },
            },
            u2: C2RustUnnamed_0 { next: 0 },
        },
        retval: 0 as *mut zval,
        params: 0 as *mut zval,
        object: 0 as *mut zend_object,
        no_separation: 0,
        param_count: 0,
    };
    let mut fcc: zend_fcall_info_cache = zend_fcall_info_cache {
        function_handler: 0 as *mut zend_function,
        calling_scope: 0 as *mut zend_class_entry,
        called_scope: 0 as *mut zend_class_entry,
        object: 0 as *mut zend_object,
    };
    let mut intern: *mut kafka_conf_object = 0 as *mut kafka_conf_object;
    if zend_parse_parameters(
        (*execute_data).This.u2.num_args as libc::c_int,
        b"f\0" as *const u8 as *const libc::c_char,
        &mut fci as *mut zend_fcall_info,
        &mut fcc as *mut zend_fcall_info_cache,
    ) == FAILURE as libc::c_int
    {
        return;
    }
    intern = get_kafka_conf_object(
        if zval_get_type(&mut (*execute_data).This) as libc::c_int == 8 as libc::c_int {
            &mut (*execute_data).This
        } else {
            0 as *mut zval
        },
    );
    if intern.is_null() {
        return;
    }
    zval_addref_p(&mut fci.function_name);
    if !((*intern).cbs.offset_commit).is_null() {
        zval_ptr_dtor(&mut (*(*intern).cbs.offset_commit).fci.function_name);
    } else {
        (*intern)
            .cbs
            .offset_commit = _ecalloc(
            1 as libc::c_int as size_t,
            ::std::mem::size_of::<kafka_conf_callback>() as libc::c_ulong,
        ) as *mut kafka_conf_callback;
    }
    (*(*intern).cbs.offset_commit).fci = fci;
    (*(*intern).cbs.offset_commit).fcc = fcc;
    rd_kafka_conf_set_offset_commit_cb(
        (*intern).u.conf,
        Some(
            kafka_conf_offset_commit_cb
                as unsafe extern "C" fn(
                    *mut rd_kafka_t,
                    rd_kafka_resp_err_t,
                    *mut rd_kafka_topic_partition_list_t,
                    *mut libc::c_void,
                ) -> (),
        ),
    );
}
pub unsafe extern "C" fn zim_RdKafka_Conf_setLogCb(
    mut execute_data: *mut zend_execute_data,
    mut return_value: *mut zval,
) {
    let mut fci: zend_fcall_info = zend_fcall_info {
        size: 0,
        function_name: zval {
            value: _zend_value { lval: 0 },
            u1: C2RustUnnamed_1 {
                v: C2RustUnnamed_2 {
                    type_0: 0,
                    type_flags: 0,
                    u: C2RustUnnamed_3 { extra: 0 },
                },
            },
            u2: C2RustUnnamed_0 { next: 0 },
        },
        retval: 0 as *mut zval,
        params: 0 as *mut zval,
        object: 0 as *mut zend_object,
        no_separation: 0,
        param_count: 0,
    };
    let mut fcc: zend_fcall_info_cache = zend_fcall_info_cache {
        function_handler: 0 as *mut zend_function,
        calling_scope: 0 as *mut zend_class_entry,
        called_scope: 0 as *mut zend_class_entry,
        object: 0 as *mut zend_object,
    };
    let mut conf: *mut kafka_conf_object = 0 as *mut kafka_conf_object;
    let mut errstr: [libc::c_char; 512] = [0; 512];
    if zend_parse_parameters(
        (*execute_data).This.u2.num_args as libc::c_int,
        b"f\0" as *const u8 as *const libc::c_char,
        &mut fci as *mut zend_fcall_info,
        &mut fcc as *mut zend_fcall_info_cache,
    ) == FAILURE as libc::c_int
    {
        return;
    }
    conf = get_kafka_conf_object(
        if zval_get_type(&mut (*execute_data).This) as libc::c_int == 8 as libc::c_int {
            &mut (*execute_data).This
        } else {
            0 as *mut zval
        },
    );
    if conf.is_null() {
        return;
    }
    zval_addref_p(&mut fci.function_name);
    if !((*conf).cbs.log).is_null() {
        zval_ptr_dtor(&mut (*(*conf).cbs.log).fci.function_name);
    } else {
        (*conf)
            .cbs
            .log = _ecalloc(
            1 as libc::c_int as size_t,
            ::std::mem::size_of::<kafka_conf_callback>() as libc::c_ulong,
        ) as *mut kafka_conf_callback;
    }
    (*(*conf).cbs.log).fci = fci;
    (*(*conf).cbs.log).fcc = fcc;
    rd_kafka_conf_set_log_cb(
        (*conf).u.conf,
        Some(
            kafka_conf_log_cb
                as unsafe extern "C" fn(
                    *const rd_kafka_t,
                    libc::c_int,
                    *const libc::c_char,
                    *const libc::c_char,
                ) -> (),
        ),
    );
    rd_kafka_conf_set(
        (*conf).u.conf,
        b"log.queue\0" as *const u8 as *const libc::c_char,
        b"true\0" as *const u8 as *const libc::c_char,
        errstr.as_mut_ptr(),
        ::std::mem::size_of::<[libc::c_char; 512]>() as libc::c_ulong,
    );
}
pub unsafe extern "C" fn zim_RdKafka_Conf_setOauthbearerTokenRefreshCb(
    mut execute_data: *mut zend_execute_data,
    mut return_value: *mut zval,
) {
    let mut fci: zend_fcall_info = zend_fcall_info {
        size: 0,
        function_name: zval {
            value: _zend_value { lval: 0 },
            u1: C2RustUnnamed_1 {
                v: C2RustUnnamed_2 {
                    type_0: 0,
                    type_flags: 0,
                    u: C2RustUnnamed_3 { extra: 0 },
                },
            },
            u2: C2RustUnnamed_0 { next: 0 },
        },
        retval: 0 as *mut zval,
        params: 0 as *mut zval,
        object: 0 as *mut zend_object,
        no_separation: 0,
        param_count: 0,
    };
    let mut fcc: zend_fcall_info_cache = zend_fcall_info_cache {
        function_handler: 0 as *mut zend_function,
        calling_scope: 0 as *mut zend_class_entry,
        called_scope: 0 as *mut zend_class_entry,
        object: 0 as *mut zend_object,
    };
    let mut conf: *mut kafka_conf_object = 0 as *mut kafka_conf_object;
    if zend_parse_parameters(
        (*execute_data).This.u2.num_args as libc::c_int,
        b"f\0" as *const u8 as *const libc::c_char,
        &mut fci as *mut zend_fcall_info,
        &mut fcc as *mut zend_fcall_info_cache,
    ) == FAILURE as libc::c_int
    {
        return;
    }
    conf = get_kafka_conf_object(
        if zval_get_type(&mut (*execute_data).This) as libc::c_int == 8 as libc::c_int {
            &mut (*execute_data).This
        } else {
            0 as *mut zval
        },
    );
    if conf.is_null() {
        return;
    }
    zval_addref_p(&mut fci.function_name);
    if !((*conf).cbs.oauthbearer_token_refresh).is_null() {
        zval_ptr_dtor(&mut (*(*conf).cbs.oauthbearer_token_refresh).fci.function_name);
    } else {
        (*conf)
            .cbs
            .oauthbearer_token_refresh = _ecalloc(
            1 as libc::c_int as size_t,
            ::std::mem::size_of::<kafka_conf_callback>() as libc::c_ulong,
        ) as *mut kafka_conf_callback;
    }
    (*(*conf).cbs.oauthbearer_token_refresh).fci = fci;
    (*(*conf).cbs.oauthbearer_token_refresh).fcc = fcc;
    rd_kafka_conf_set_oauthbearer_token_refresh_cb(
        (*conf).u.conf,
        Some(
            kafka_conf_set_oauthbearer_token_refresh_cb
                as unsafe extern "C" fn(
                    *mut rd_kafka_t,
                    *const libc::c_char,
                    *mut libc::c_void,
                ) -> (),
        ),
    );
}
pub unsafe extern "C" fn zim_RdKafka_TopicConf___construct(
    mut execute_data: *mut zend_execute_data,
    mut return_value: *mut zval,
) {
    let mut intern: *mut kafka_conf_object = 0 as *mut kafka_conf_object;
    let mut error_handling: zend_error_handling = zend_error_handling {
        handling: EH_NORMAL,
        exception: 0 as *mut zend_class_entry,
        user_handler: zval {
            value: _zend_value { lval: 0 },
            u1: C2RustUnnamed_1 {
                v: C2RustUnnamed_2 {
                    type_0: 0,
                    type_flags: 0,
                    u: C2RustUnnamed_3 { extra: 0 },
                },
            },
            u2: C2RustUnnamed_0 { next: 0 },
        },
    };
    zend_replace_error_handling(
        EH_THROW,
        spl_ce_InvalidArgumentException,
        &mut error_handling,
    );
    if zend_parse_parameters(
        (*execute_data).This.u2.num_args as libc::c_int,
        b"\0" as *const u8 as *const libc::c_char,
    ) == FAILURE as libc::c_int
    {
        zend_restore_error_handling(&mut error_handling);
        return;
    }
    intern = ((*(if zval_get_type(&mut (*execute_data).This) as libc::c_int
        == 8 as libc::c_int
    {
        &mut (*execute_data).This as *mut zval
    } else {
        0 as *mut zval
    }))
        .value
        .obj as *mut libc::c_char)
        .offset(-(96 as libc::c_ulong as isize)) as *mut kafka_conf_object;
    (*intern).type_0 = KAFKA_TOPIC_CONF;
    (*intern).u.topic_conf = rd_kafka_topic_conf_new();
    zend_restore_error_handling(&mut error_handling);
}
pub unsafe extern "C" fn zim_RdKafka_TopicConf_setPartitioner(
    mut execute_data: *mut zend_execute_data,
    mut return_value: *mut zval,
) {
    let mut intern: *mut kafka_conf_object = 0 as *mut kafka_conf_object;
    let mut id: zend_long = 0;
    let mut partitioner: Option::<
        unsafe extern "C" fn(
            *const rd_kafka_topic_t,
            *const libc::c_void,
            size_t,
            int32_t,
            *mut libc::c_void,
            *mut libc::c_void,
        ) -> int32_t,
    > = None;
    if zend_parse_parameters(
        (*execute_data).This.u2.num_args as libc::c_int,
        b"l\0" as *const u8 as *const libc::c_char,
        &mut id as *mut zend_long,
    ) == FAILURE as libc::c_int
    {
        return;
    }
    intern = get_kafka_conf_object(
        if zval_get_type(&mut (*execute_data).This) as libc::c_int == 8 as libc::c_int {
            &mut (*execute_data).This
        } else {
            0 as *mut zval
        },
    );
    if intern.is_null() {
        return;
    }
    match id {
        2 => {
            partitioner = Some(
                rd_kafka_msg_partitioner_random
                    as unsafe extern "C" fn(
                        *const rd_kafka_topic_t,
                        *const libc::c_void,
                        size_t,
                        int32_t,
                        *mut libc::c_void,
                        *mut libc::c_void,
                    ) -> int32_t,
            );
        }
        3 => {
            partitioner = Some(
                rd_kafka_msg_partitioner_consistent
                    as unsafe extern "C" fn(
                        *const rd_kafka_topic_t,
                        *const libc::c_void,
                        size_t,
                        int32_t,
                        *mut libc::c_void,
                        *mut libc::c_void,
                    ) -> int32_t,
            );
        }
        4 => {
            partitioner = Some(
                rd_kafka_msg_partitioner_consistent_random
                    as unsafe extern "C" fn(
                        *const rd_kafka_topic_t,
                        *const libc::c_void,
                        size_t,
                        int32_t,
                        *mut libc::c_void,
                        *mut libc::c_void,
                    ) -> int32_t,
            );
        }
        5 => {
            partitioner = Some(
                rd_kafka_msg_partitioner_murmur2
                    as unsafe extern "C" fn(
                        *const rd_kafka_topic_t,
                        *const libc::c_void,
                        size_t,
                        int32_t,
                        *mut libc::c_void,
                        *mut libc::c_void,
                    ) -> int32_t,
            );
        }
        6 => {
            partitioner = Some(
                rd_kafka_msg_partitioner_murmur2_random
                    as unsafe extern "C" fn(
                        *const rd_kafka_topic_t,
                        *const libc::c_void,
                        size_t,
                        int32_t,
                        *mut libc::c_void,
                        *mut libc::c_void,
                    ) -> int32_t,
            );
        }
        _ => {
            zend_throw_exception_ex(
                spl_ce_InvalidArgumentException,
                0 as libc::c_int as zend_long,
                b"Invalid partitioner given\0" as *const u8 as *const libc::c_char,
            );
            return;
        }
    }
    rd_kafka_topic_conf_set_partitioner_cb((*intern).u.topic_conf, partitioner);
}
pub unsafe extern "C" fn kafka_conf_minit(
    mut type_0: libc::c_int,
    mut module_number: libc::c_int,
) {
    handlers = kafka_default_object_handlers;
    handlers
        .free_obj = Some(
        kafka_conf_free as unsafe extern "C" fn(*mut zend_object) -> (),
    );
    handlers.offset = 96 as libc::c_ulong as libc::c_int;
    ce_kafka_conf = register_class_RdKafka_Conf();
    (*ce_kafka_conf)
        .c2rust_unnamed_0
        .create_object = Some(
        kafka_conf_new as unsafe extern "C" fn(*mut zend_class_entry) -> *mut zend_object,
    );
    ce_kafka_topic_conf = register_class_RdKafka_TopicConf();
    (*ce_kafka_topic_conf)
        .c2rust_unnamed_0
        .create_object = Some(
        kafka_conf_new as unsafe extern "C" fn(*mut zend_class_entry) -> *mut zend_object,
    );
}
unsafe extern "C" fn run_static_initializers() {
    class_RdKafka_Conf_methods = [
        {
            let mut init = _zend_function_entry {
                fname: b"__construct\0" as *const u8 as *const libc::c_char,
                handler: Some(
                    zim_RdKafka_Conf___construct
                        as unsafe extern "C" fn(*mut zend_execute_data, *mut zval) -> (),
                ),
                arg_info: arginfo_class_RdKafka_Conf___construct.as_ptr(),
                num_args: (::std::mem::size_of::<[zend_internal_arg_info; 1]>()
                    as libc::c_ulong)
                    .wrapping_div(
                        ::std::mem::size_of::<_zend_internal_arg_info>() as libc::c_ulong,
                    )
                    .wrapping_sub(1 as libc::c_int as libc::c_ulong) as uint32_t,
                flags: ((1 as libc::c_int) << 0 as libc::c_int) as uint32_t,
            };
            init
        },
        {
            let mut init = _zend_function_entry {
                fname: b"dump\0" as *const u8 as *const libc::c_char,
                handler: Some(
                    zim_RdKafka_Conf_dump
                        as unsafe extern "C" fn(*mut zend_execute_data, *mut zval) -> (),
                ),
                arg_info: arginfo_class_RdKafka_Conf_dump.as_ptr(),
                num_args: (::std::mem::size_of::<[zend_internal_arg_info; 1]>()
                    as libc::c_ulong)
                    .wrapping_div(
                        ::std::mem::size_of::<_zend_internal_arg_info>() as libc::c_ulong,
                    )
                    .wrapping_sub(1 as libc::c_int as libc::c_ulong) as uint32_t,
                flags: ((1 as libc::c_int) << 0 as libc::c_int) as uint32_t,
            };
            init
        },
        {
            let mut init = _zend_function_entry {
                fname: b"set\0" as *const u8 as *const libc::c_char,
                handler: Some(
                    zim_RdKafka_Conf_set
                        as unsafe extern "C" fn(*mut zend_execute_data, *mut zval) -> (),
                ),
                arg_info: arginfo_class_RdKafka_Conf_set.as_ptr(),
                num_args: (::std::mem::size_of::<[zend_internal_arg_info; 3]>()
                    as libc::c_ulong)
                    .wrapping_div(
                        ::std::mem::size_of::<_zend_internal_arg_info>() as libc::c_ulong,
                    )
                    .wrapping_sub(1 as libc::c_int as libc::c_ulong) as uint32_t,
                flags: ((1 as libc::c_int) << 0 as libc::c_int) as uint32_t,
            };
            init
        },
        {
            let mut init = _zend_function_entry {
                fname: b"setDefaultTopicConf\0" as *const u8 as *const libc::c_char,
                handler: Some(
                    zim_RdKafka_Conf_setDefaultTopicConf
                        as unsafe extern "C" fn(*mut zend_execute_data, *mut zval) -> (),
                ),
                arg_info: arginfo_class_RdKafka_Conf_setDefaultTopicConf.as_ptr(),
                num_args: (::std::mem::size_of::<[zend_internal_arg_info; 2]>()
                    as libc::c_ulong)
                    .wrapping_div(
                        ::std::mem::size_of::<_zend_internal_arg_info>() as libc::c_ulong,
                    )
                    .wrapping_sub(1 as libc::c_int as libc::c_ulong) as uint32_t,
                flags: ((1 as libc::c_int) << 0 as libc::c_int
                    | (1 as libc::c_int) << 11 as libc::c_int) as uint32_t,
            };
            init
        },
        {
            let mut init = _zend_function_entry {
                fname: b"setErrorCb\0" as *const u8 as *const libc::c_char,
                handler: Some(
                    zim_RdKafka_Conf_setErrorCb
                        as unsafe extern "C" fn(*mut zend_execute_data, *mut zval) -> (),
                ),
                arg_info: arginfo_class_RdKafka_Conf_setErrorCb.as_ptr(),
                num_args: (::std::mem::size_of::<[zend_internal_arg_info; 2]>()
                    as libc::c_ulong)
                    .wrapping_div(
                        ::std::mem::size_of::<_zend_internal_arg_info>() as libc::c_ulong,
                    )
                    .wrapping_sub(1 as libc::c_int as libc::c_ulong) as uint32_t,
                flags: ((1 as libc::c_int) << 0 as libc::c_int) as uint32_t,
            };
            init
        },
        {
            let mut init = _zend_function_entry {
                fname: b"setDrMsgCb\0" as *const u8 as *const libc::c_char,
                handler: Some(
                    zim_RdKafka_Conf_setDrMsgCb
                        as unsafe extern "C" fn(*mut zend_execute_data, *mut zval) -> (),
                ),
                arg_info: arginfo_class_RdKafka_Conf_setErrorCb.as_ptr(),
                num_args: (::std::mem::size_of::<[zend_internal_arg_info; 2]>()
                    as libc::c_ulong)
                    .wrapping_div(
                        ::std::mem::size_of::<_zend_internal_arg_info>() as libc::c_ulong,
                    )
                    .wrapping_sub(1 as libc::c_int as libc::c_ulong) as uint32_t,
                flags: ((1 as libc::c_int) << 0 as libc::c_int) as uint32_t,
            };
            init
        },
        {
            let mut init = _zend_function_entry {
                fname: b"setStatsCb\0" as *const u8 as *const libc::c_char,
                handler: Some(
                    zim_RdKafka_Conf_setStatsCb
                        as unsafe extern "C" fn(*mut zend_execute_data, *mut zval) -> (),
                ),
                arg_info: arginfo_class_RdKafka_Conf_setErrorCb.as_ptr(),
                num_args: (::std::mem::size_of::<[zend_internal_arg_info; 2]>()
                    as libc::c_ulong)
                    .wrapping_div(
                        ::std::mem::size_of::<_zend_internal_arg_info>() as libc::c_ulong,
                    )
                    .wrapping_sub(1 as libc::c_int as libc::c_ulong) as uint32_t,
                flags: ((1 as libc::c_int) << 0 as libc::c_int) as uint32_t,
            };
            init
        },
        {
            let mut init = _zend_function_entry {
                fname: b"setRebalanceCb\0" as *const u8 as *const libc::c_char,
                handler: Some(
                    zim_RdKafka_Conf_setRebalanceCb
                        as unsafe extern "C" fn(*mut zend_execute_data, *mut zval) -> (),
                ),
                arg_info: arginfo_class_RdKafka_Conf_setErrorCb.as_ptr(),
                num_args: (::std::mem::size_of::<[zend_internal_arg_info; 2]>()
                    as libc::c_ulong)
                    .wrapping_div(
                        ::std::mem::size_of::<_zend_internal_arg_info>() as libc::c_ulong,
                    )
                    .wrapping_sub(1 as libc::c_int as libc::c_ulong) as uint32_t,
                flags: ((1 as libc::c_int) << 0 as libc::c_int) as uint32_t,
            };
            init
        },
        {
            let mut init = _zend_function_entry {
                fname: b"setConsumeCb\0" as *const u8 as *const libc::c_char,
                handler: Some(
                    zim_RdKafka_Conf_setConsumeCb
                        as unsafe extern "C" fn(*mut zend_execute_data, *mut zval) -> (),
                ),
                arg_info: arginfo_class_RdKafka_Conf_setErrorCb.as_ptr(),
                num_args: (::std::mem::size_of::<[zend_internal_arg_info; 2]>()
                    as libc::c_ulong)
                    .wrapping_div(
                        ::std::mem::size_of::<_zend_internal_arg_info>() as libc::c_ulong,
                    )
                    .wrapping_sub(1 as libc::c_int as libc::c_ulong) as uint32_t,
                flags: ((1 as libc::c_int) << 0 as libc::c_int) as uint32_t,
            };
            init
        },
        {
            let mut init = _zend_function_entry {
                fname: b"setOffsetCommitCb\0" as *const u8 as *const libc::c_char,
                handler: Some(
                    zim_RdKafka_Conf_setOffsetCommitCb
                        as unsafe extern "C" fn(*mut zend_execute_data, *mut zval) -> (),
                ),
                arg_info: arginfo_class_RdKafka_Conf_setErrorCb.as_ptr(),
                num_args: (::std::mem::size_of::<[zend_internal_arg_info; 2]>()
                    as libc::c_ulong)
                    .wrapping_div(
                        ::std::mem::size_of::<_zend_internal_arg_info>() as libc::c_ulong,
                    )
                    .wrapping_sub(1 as libc::c_int as libc::c_ulong) as uint32_t,
                flags: ((1 as libc::c_int) << 0 as libc::c_int) as uint32_t,
            };
            init
        },
        {
            let mut init = _zend_function_entry {
                fname: b"setLogCb\0" as *const u8 as *const libc::c_char,
                handler: Some(
                    zim_RdKafka_Conf_setLogCb
                        as unsafe extern "C" fn(*mut zend_execute_data, *mut zval) -> (),
                ),
                arg_info: arginfo_class_RdKafka_Conf_setErrorCb.as_ptr(),
                num_args: (::std::mem::size_of::<[zend_internal_arg_info; 2]>()
                    as libc::c_ulong)
                    .wrapping_div(
                        ::std::mem::size_of::<_zend_internal_arg_info>() as libc::c_ulong,
                    )
                    .wrapping_sub(1 as libc::c_int as libc::c_ulong) as uint32_t,
                flags: ((1 as libc::c_int) << 0 as libc::c_int) as uint32_t,
            };
            init
        },
        {
            let mut init = _zend_function_entry {
                fname: b"setOauthbearerTokenRefreshCb\0" as *const u8
                    as *const libc::c_char,
                handler: Some(
                    zim_RdKafka_Conf_setOauthbearerTokenRefreshCb
                        as unsafe extern "C" fn(*mut zend_execute_data, *mut zval) -> (),
                ),
                arg_info: arginfo_class_RdKafka_Conf_setErrorCb.as_ptr(),
                num_args: (::std::mem::size_of::<[zend_internal_arg_info; 2]>()
                    as libc::c_ulong)
                    .wrapping_div(
                        ::std::mem::size_of::<_zend_internal_arg_info>() as libc::c_ulong,
                    )
                    .wrapping_sub(1 as libc::c_int as libc::c_ulong) as uint32_t,
                flags: ((1 as libc::c_int) << 0 as libc::c_int) as uint32_t,
            };
            init
        },
        {
            let mut init = _zend_function_entry {
                fname: 0 as *const libc::c_char,
                handler: None,
                arg_info: 0 as *const _zend_internal_arg_info,
                num_args: 0 as libc::c_int as uint32_t,
                flags: 0 as libc::c_int as uint32_t,
            };
            init
        },
    ];
    class_RdKafka_TopicConf_methods = [
        {
            let mut init = _zend_function_entry {
                fname: b"__construct\0" as *const u8 as *const libc::c_char,
                handler: Some(
                    zim_RdKafka_TopicConf___construct
                        as unsafe extern "C" fn(*mut zend_execute_data, *mut zval) -> (),
                ),
                arg_info: arginfo_class_RdKafka_Conf___construct.as_ptr(),
                num_args: (::std::mem::size_of::<[zend_internal_arg_info; 1]>()
                    as libc::c_ulong)
                    .wrapping_div(
                        ::std::mem::size_of::<_zend_internal_arg_info>() as libc::c_ulong,
                    )
                    .wrapping_sub(1 as libc::c_int as libc::c_ulong) as uint32_t,
                flags: ((1 as libc::c_int) << 0 as libc::c_int) as uint32_t,
            };
            init
        },
        {
            let mut init = _zend_function_entry {
                fname: b"dump\0" as *const u8 as *const libc::c_char,
                handler: Some(
                    zim_RdKafka_Conf_dump
                        as unsafe extern "C" fn(*mut zend_execute_data, *mut zval) -> (),
                ),
                arg_info: arginfo_class_RdKafka_Conf_dump.as_ptr(),
                num_args: (::std::mem::size_of::<[zend_internal_arg_info; 1]>()
                    as libc::c_ulong)
                    .wrapping_div(
                        ::std::mem::size_of::<_zend_internal_arg_info>() as libc::c_ulong,
                    )
                    .wrapping_sub(1 as libc::c_int as libc::c_ulong) as uint32_t,
                flags: ((1 as libc::c_int) << 0 as libc::c_int) as uint32_t,
            };
            init
        },
        {
            let mut init = _zend_function_entry {
                fname: b"set\0" as *const u8 as *const libc::c_char,
                handler: Some(
                    zim_RdKafka_Conf_set
                        as unsafe extern "C" fn(*mut zend_execute_data, *mut zval) -> (),
                ),
                arg_info: arginfo_class_RdKafka_Conf_set.as_ptr(),
                num_args: (::std::mem::size_of::<[zend_internal_arg_info; 3]>()
                    as libc::c_ulong)
                    .wrapping_div(
                        ::std::mem::size_of::<_zend_internal_arg_info>() as libc::c_ulong,
                    )
                    .wrapping_sub(1 as libc::c_int as libc::c_ulong) as uint32_t,
                flags: ((1 as libc::c_int) << 0 as libc::c_int) as uint32_t,
            };
            init
        },
        {
            let mut init = _zend_function_entry {
                fname: b"setPartitioner\0" as *const u8 as *const libc::c_char,
                handler: Some(
                    zim_RdKafka_TopicConf_setPartitioner
                        as unsafe extern "C" fn(*mut zend_execute_data, *mut zval) -> (),
                ),
                arg_info: arginfo_class_RdKafka_TopicConf_setPartitioner.as_ptr(),
                num_args: (::std::mem::size_of::<[zend_internal_arg_info; 2]>()
                    as libc::c_ulong)
                    .wrapping_div(
                        ::std::mem::size_of::<_zend_internal_arg_info>() as libc::c_ulong,
                    )
                    .wrapping_sub(1 as libc::c_int as libc::c_ulong) as uint32_t,
                flags: ((1 as libc::c_int) << 0 as libc::c_int) as uint32_t,
            };
            init
        },
        {
            let mut init = _zend_function_entry {
                fname: 0 as *const libc::c_char,
                handler: None,
                arg_info: 0 as *const _zend_internal_arg_info,
                num_args: 0 as libc::c_int as uint32_t,
                flags: 0 as libc::c_int as uint32_t,
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
