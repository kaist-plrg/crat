use ::libc;
extern "C" {
    pub type _zend_unserialize_data;
    pub type _zend_serialize_data;
    pub type rd_kafka_s;
    pub type rd_kafka_topic_s;
    pub type rd_kafka_conf_s;
    pub type rd_kafka_topic_conf_s;
    pub type rd_kafka_queue_s;
    fn _emalloc(size: size_t) -> *mut libc::c_void;
    fn _efree(ptr: *mut libc::c_void);
    fn _emalloc_896() -> *mut libc::c_void;
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
    fn _zend_hash_init(
        ht: *mut HashTable,
        nSize: uint32_t,
        pDestructor: dtor_func_t,
        persistent: zend_bool,
    );
    fn zend_hash_destroy(ht: *mut HashTable);
    fn zend_hash_str_add(
        ht: *mut HashTable,
        key: *const libc::c_char,
        len: size_t,
        pData: *mut zval,
    ) -> *mut zval;
    fn zend_hash_index_add(
        ht: *mut HashTable,
        h: zend_ulong,
        pData: *mut zval,
    ) -> *mut zval;
    fn zend_hash_apply(ht: *mut HashTable, apply_func: apply_func_t);
    fn zend_hash_str_del(
        ht: *mut HashTable,
        key: *const libc::c_char,
        len: size_t,
    ) -> libc::c_int;
    fn zend_hash_str_find(
        ht: *const HashTable,
        key: *const libc::c_char,
        len: size_t,
    ) -> *mut zval;
    fn zval_ptr_dtor(zval_ptr: *mut zval);
    fn zend_spprintf(
        message: *mut *mut libc::c_char,
        max_len: size_t,
        format: *const libc::c_char,
        _: ...
    ) -> size_t;
    fn zend_replace_error_handling(
        error_handling: zend_error_handling_t,
        exception_class: *mut zend_class_entry,
        current: *mut zend_error_handling,
    );
    fn zend_restore_error_handling(saved: *mut zend_error_handling);
    static std_object_handlers: zend_object_handlers;
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
    fn zend_declare_property_ex(
        ce: *mut zend_class_entry,
        name: *mut zend_string,
        property: *mut zval,
        access_type: libc::c_int,
        doc_comment: *mut zend_string,
    ) -> libc::c_int;
    fn object_init_ex(arg: *mut zval, ce: *mut zend_class_entry) -> libc::c_int;
    fn object_properties_init(
        object: *mut zend_object,
        class_type: *mut zend_class_entry,
    );
    fn ap_php_snprintf(
        _: *mut libc::c_char,
        _: size_t,
        _: *const libc::c_char,
        _: ...
    ) -> libc::c_int;
    fn zend_register_long_constant(
        name: *const libc::c_char,
        name_len: size_t,
        lval: zend_long,
        flags: libc::c_int,
        module_number: libc::c_int,
    );
    static mut zend_string_init_interned: zend_string_init_interned_func_t;
    fn php_info_print_table_row(num_cols: libc::c_int, _: ...);
    fn php_info_print_table_start();
    fn php_info_print_table_end();
    fn rd_kafka_version() -> libc::c_int;
    fn rd_kafka_version_str() -> *const libc::c_char;
    fn rd_kafka_get_err_descs(
        errdescs: *mut *const rd_kafka_err_desc,
        cntp: *mut size_t,
    );
    fn rd_kafka_err2str(err: rd_kafka_resp_err_t) -> *const libc::c_char;
    fn rd_kafka_topic_partition_list_destroy(
        rkparlist: *mut rd_kafka_topic_partition_list_t,
    );
    fn rd_kafka_conf_new() -> *mut rd_kafka_conf_t;
    fn rd_kafka_conf_dup(conf: *const rd_kafka_conf_t) -> *mut rd_kafka_conf_t;
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
    fn rd_kafka_conf_set_opaque(conf: *mut rd_kafka_conf_t, opaque: *mut libc::c_void);
    fn rd_kafka_topic_conf_dup(
        conf: *const rd_kafka_topic_conf_t,
    ) -> *mut rd_kafka_topic_conf_t;
    fn rd_kafka_new(
        type_0: rd_kafka_type_t,
        conf: *mut rd_kafka_conf_t,
        errstr: *mut libc::c_char,
        errstr_size: size_t,
    ) -> *mut rd_kafka_t;
    fn rd_kafka_destroy(rk: *mut rd_kafka_t);
    fn rd_kafka_topic_new(
        rk: *mut rd_kafka_t,
        topic: *const libc::c_char,
        conf: *mut rd_kafka_topic_conf_t,
    ) -> *mut rd_kafka_topic_t;
    fn rd_kafka_topic_destroy(rkt: *mut rd_kafka_topic_t);
    fn rd_kafka_topic_name(rkt: *const rd_kafka_topic_t) -> *const libc::c_char;
    fn rd_kafka_poll(rk: *mut rd_kafka_t, timeout_ms: libc::c_int) -> libc::c_int;
    fn rd_kafka_pause_partitions(
        rk: *mut rd_kafka_t,
        partitions: *mut rd_kafka_topic_partition_list_t,
    ) -> rd_kafka_resp_err_t;
    fn rd_kafka_query_watermark_offsets(
        rk: *mut rd_kafka_t,
        topic: *const libc::c_char,
        partition: int32_t,
        low: *mut int64_t,
        high: *mut int64_t,
        timeout_ms: libc::c_int,
    ) -> rd_kafka_resp_err_t;
    fn rd_kafka_offsets_for_times(
        rk: *mut rd_kafka_t,
        offsets: *mut rd_kafka_topic_partition_list_t,
        timeout_ms: libc::c_int,
    ) -> rd_kafka_resp_err_t;
    fn rd_kafka_queue_new(rk: *mut rd_kafka_t) -> *mut rd_kafka_queue_t;
    fn rd_kafka_queue_destroy(rkqu: *mut rd_kafka_queue_t);
    fn rd_kafka_set_log_queue(
        rk: *mut rd_kafka_t,
        rkqu: *mut rd_kafka_queue_t,
    ) -> rd_kafka_resp_err_t;
    fn rd_kafka_consume_stop(
        rkt: *mut rd_kafka_topic_t,
        partition: int32_t,
    ) -> libc::c_int;
    fn rd_kafka_flush(
        rk: *mut rd_kafka_t,
        timeout_ms: libc::c_int,
    ) -> rd_kafka_resp_err_t;
    fn rd_kafka_purge(
        rk: *mut rd_kafka_t,
        purge_flags: libc::c_int,
    ) -> rd_kafka_resp_err_t;
    fn rd_kafka_metadata(
        rk: *mut rd_kafka_t,
        all_topics: libc::c_int,
        only_rkt: *mut rd_kafka_topic_t,
        metadatap: *mut *const rd_kafka_metadata,
        timeout_ms: libc::c_int,
    ) -> rd_kafka_resp_err_t;
    fn rd_kafka_brokers_add(
        rk: *mut rd_kafka_t,
        brokerlist: *const libc::c_char,
    ) -> libc::c_int;
    fn rd_kafka_set_logger(
        rk: *mut rd_kafka_t,
        func: Option::<
            unsafe extern "C" fn(
                *const rd_kafka_t,
                libc::c_int,
                *const libc::c_char,
                *const libc::c_char,
            ) -> (),
        >,
    );
    fn rd_kafka_set_log_level(rk: *mut rd_kafka_t, level: libc::c_int);
    fn rd_kafka_log_print(
        rk: *const rd_kafka_t,
        level: libc::c_int,
        fac: *const libc::c_char,
        buf: *const libc::c_char,
    );
    fn rd_kafka_log_syslog(
        rk: *const rd_kafka_t,
        level: libc::c_int,
        fac: *const libc::c_char,
        buf: *const libc::c_char,
    );
    fn rd_kafka_outq_len(rk: *mut rd_kafka_t) -> libc::c_int;
    fn get_kafka_conf_object(zconf: *mut zval) -> *mut kafka_conf_object;
    fn kafka_conf_minit(type_0: libc::c_int, module_number: libc::c_int);
    fn kafka_conf_callbacks_dtor(cbs: *mut kafka_conf_callbacks);
    fn kafka_conf_callbacks_copy(
        to: *mut kafka_conf_callbacks,
        from: *mut kafka_conf_callbacks,
    );
    fn kafka_conf_dr_msg_cb(
        rk: *mut rd_kafka_t,
        msg: *const rd_kafka_message_t,
        opaque: *mut libc::c_void,
    );
    static mut ce_kafka_conf: *mut zend_class_entry;
    static mut ce_kafka_topic_conf: *mut zend_class_entry;
    static mut zend_ce_exception: *mut zend_class_entry;
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
    fn kafka_metadata_minit(type_0: libc::c_int, module_number: libc::c_int);
    fn kafka_metadata_init(
        return_value: *mut zval,
        metadata: *const rd_kafka_metadata_t,
    );
    fn kafka_topic_minit(type_0: libc::c_int, module_number: libc::c_int);
    fn get_kafka_topic_object(zrkt: *mut zval) -> *mut kafka_topic_object;
    static mut ce_kafka_consumer_topic: *mut zend_class_entry;
    static mut ce_kafka_producer_topic: *mut zend_class_entry;
    static mut ce_kafka_topic: *mut zend_class_entry;
    fn kafka_queue_minit(type_0: libc::c_int, module_number: libc::c_int);
    static mut ce_kafka_queue: *mut zend_class_entry;
    fn kafka_message_minit(type_0: libc::c_int, module_number: libc::c_int);
    fn kafka_kafka_consumer_minit(type_0: libc::c_int, module_number: libc::c_int);
    fn kafka_metadata_topic_partition_minit(
        type_0: libc::c_int,
        module_number: libc::c_int,
    );
    fn kafka_topic_partition_list_to_array(
        return_value: *mut zval,
        list: *mut rd_kafka_topic_partition_list_t,
    );
    fn array_arg_to_kafka_topic_partition_list(
        argnum: libc::c_int,
        ary: *mut HashTable,
    ) -> *mut rd_kafka_topic_partition_list_t;
    fn zif_rd_kafka_get_err_descs(
        execute_data: *mut zend_execute_data,
        return_value: *mut zval,
    );
    fn zif_rd_kafka_err2name(
        execute_data: *mut zend_execute_data,
        return_value: *mut zval,
    );
    fn zif_rd_kafka_err2str(
        execute_data: *mut zend_execute_data,
        return_value: *mut zval,
    );
    fn zif_rd_kafka_errno2err(
        execute_data: *mut zend_execute_data,
        return_value: *mut zval,
    );
    fn zif_rd_kafka_errno(execute_data: *mut zend_execute_data, return_value: *mut zval);
    fn zif_rd_kafka_offset_tail(
        execute_data: *mut zend_execute_data,
        return_value: *mut zval,
    );
    fn zif_rd_kafka_thread_cnt(
        execute_data: *mut zend_execute_data,
        return_value: *mut zval,
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
pub type apply_func_t = Option::<unsafe extern "C" fn(*mut zval) -> libc::c_int>;
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
pub type rd_kafka_type_t = libc::c_uint;
pub const RD_KAFKA_CONSUMER: rd_kafka_type_t = 1;
pub const RD_KAFKA_PRODUCER: rd_kafka_type_t = 0;
pub type rd_kafka_t = rd_kafka_s;
pub type rd_kafka_topic_t = rd_kafka_topic_s;
pub type rd_kafka_conf_t = rd_kafka_conf_s;
pub type rd_kafka_topic_conf_t = rd_kafka_topic_conf_s;
pub type rd_kafka_queue_t = rd_kafka_queue_s;
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
pub struct rd_kafka_err_desc {
    pub code: rd_kafka_resp_err_t,
    pub name: *const libc::c_char,
    pub desc: *const libc::c_char,
}
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
pub type C2RustUnnamed_15 = libc::c_int;
pub const RD_KAFKA_CONF_OK: C2RustUnnamed_15 = 0;
pub const RD_KAFKA_CONF_INVALID: C2RustUnnamed_15 = -1;
pub const RD_KAFKA_CONF_UNKNOWN: C2RustUnnamed_15 = -2;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct rd_kafka_metadata_broker {
    pub id: int32_t,
    pub host: *mut libc::c_char,
    pub port: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct rd_kafka_metadata_partition {
    pub id: int32_t,
    pub err: rd_kafka_resp_err_t,
    pub leader: int32_t,
    pub replica_cnt: libc::c_int,
    pub replicas: *mut int32_t,
    pub isr_cnt: libc::c_int,
    pub isrs: *mut int32_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct rd_kafka_metadata_topic {
    pub topic: *mut libc::c_char,
    pub partition_cnt: libc::c_int,
    pub partitions: *mut rd_kafka_metadata_partition,
    pub err: rd_kafka_resp_err_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct rd_kafka_metadata {
    pub broker_cnt: libc::c_int,
    pub brokers: *mut rd_kafka_metadata_broker,
    pub topic_cnt: libc::c_int,
    pub topics: *mut rd_kafka_metadata_topic,
    pub orig_broker_id: int32_t,
    pub orig_broker_name: *mut libc::c_char,
}
pub type rd_kafka_metadata_t = rd_kafka_metadata;
pub type C2RustUnnamed_16 = libc::c_uint;
pub const MSG_PARTITIONER_MURMUR2_RANDOM: C2RustUnnamed_16 = 6;
pub const MSG_PARTITIONER_MURMUR2: C2RustUnnamed_16 = 5;
pub const MSG_PARTITIONER_CONSISTENT_RANDOM: C2RustUnnamed_16 = 4;
pub const MSG_PARTITIONER_CONSISTENT: C2RustUnnamed_16 = 3;
pub const MSG_PARTITIONER_RANDOM: C2RustUnnamed_16 = 2;
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
    pub u: C2RustUnnamed_17,
    pub cbs: kafka_conf_callbacks,
    pub std: zend_object,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_17 {
    pub conf: *mut rd_kafka_conf_t,
    pub topic_conf: *mut rd_kafka_topic_conf_t,
}
pub type kafka_conf_object = _kafka_conf_object;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _kafka_object {
    pub type_0: rd_kafka_type_t,
    pub rk: *mut rd_kafka_t,
    pub cbs: kafka_conf_callbacks,
    pub consuming: HashTable,
    pub topics: HashTable,
    pub queues: HashTable,
    pub std: zend_object,
}
pub type kafka_object = _kafka_object;
pub type kafka_topic_object = _kafka_topic_object;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _kafka_topic_object {
    pub rkt: *mut rd_kafka_topic_t,
    pub zrk: zval,
    pub std: zend_object,
}
pub type kafka_queue_object = _kafka_queue_object;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _kafka_queue_object {
    pub rkqu: *mut rd_kafka_queue_t,
    pub zrk: zval,
    pub std: zend_object,
}
pub type toppar = _toppar;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _toppar {
    pub rkt: *mut rd_kafka_topic_t,
    pub partition: int32_t,
}
pub const RD_KAFKA_LOG_SYSLOG_PRINT: C2RustUnnamed_18 = 102;
pub const RD_KAFKA_LOG_SYSLOG: C2RustUnnamed_18 = 101;
pub const RD_KAFKA_LOG_PRINT: C2RustUnnamed_18 = 100;
pub type C2RustUnnamed_18 = libc::c_uint;
#[inline(always)]
#[no_mangle] unsafe extern "C" fn zval_get_type(mut pz: *const zval) -> zend_uchar {
    return (*pz).u1.v.type_0;
}
#[inline(always)]
#[no_mangle] unsafe extern "C" fn zval_gc_flags(mut gc_type_info: uint32_t) -> uint32_t {
    return gc_type_info >> 0 as libc::c_int
        & (0x3f0 as libc::c_int >> 0 as libc::c_int) as libc::c_uint;
}
#[inline(always)]
#[no_mangle] unsafe extern "C" fn zend_gc_set_refcount(
    mut p: *mut zend_refcounted_h,
    mut rc: uint32_t,
) -> uint32_t {
    (*p).refcount = rc;
    return (*p).refcount;
}
#[inline(always)]
#[no_mangle] unsafe extern "C" fn zend_gc_addref(mut p: *mut zend_refcounted_h) -> uint32_t {
    (*p).refcount = ((*p).refcount).wrapping_add(1);
    return (*p).refcount;
}
#[inline(always)]
#[no_mangle] unsafe extern "C" fn zend_gc_delref(mut p: *mut zend_refcounted_h) -> uint32_t {
    if !((*p).refcount > 0 as libc::c_int as libc::c_uint) as libc::c_int as libc::c_long
        != 0
    {
        unreachable!();
    }
    (*p).refcount = ((*p).refcount).wrapping_sub(1);
    return (*p).refcount;
}
#[inline(always)]
#[no_mangle] unsafe extern "C" fn zval_addref_p(mut pz: *mut zval) -> uint32_t {
    if !((*pz).u1.v.type_flags as libc::c_int != 0 as libc::c_int) as libc::c_int
        as libc::c_long != 0
    {
        unreachable!();
    }
    return zend_gc_addref(&mut (*(*pz).value.counted).gc);
}
#[inline(always)]
#[no_mangle] unsafe extern "C" fn zend_hash_str_exists(
    mut ht: *const HashTable,
    mut str: *const libc::c_char,
    mut len: size_t,
) -> zend_bool {
    return (zend_hash_str_find(ht, str, len) != 0 as *mut libc::c_void as *mut zval)
        as libc::c_int as zend_bool;
}
#[inline(always)]
#[no_mangle] unsafe extern "C" fn zend_hash_str_add_ptr(
    mut ht: *mut HashTable,
    mut str: *const libc::c_char,
    mut len: size_t,
    mut pData: *mut libc::c_void,
) -> *mut libc::c_void {
    let mut tmp: zval = zval {
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
    let mut zv: *mut zval = 0 as *mut zval;
    tmp.value.ptr = pData;
    tmp.u1.type_info = 14 as libc::c_int as uint32_t;
    zv = zend_hash_str_add(ht, str, len, &mut tmp);
    if !zv.is_null() {
        if ((*zv).value.ptr).is_null() as libc::c_int as libc::c_long != 0 {
            unreachable!();
        }
        return (*zv).value.ptr;
    } else {
        return 0 as *mut libc::c_void
    };
}
#[inline(always)]
#[no_mangle] unsafe extern "C" fn zend_hash_index_add_ptr(
    mut ht: *mut HashTable,
    mut h: zend_ulong,
    mut pData: *mut libc::c_void,
) -> *mut libc::c_void {
    let mut tmp: zval = zval {
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
    let mut zv: *mut zval = 0 as *mut zval;
    tmp.value.ptr = pData;
    tmp.u1.type_info = 14 as libc::c_int as uint32_t;
    zv = zend_hash_index_add(ht, h, &mut tmp);
    return if !zv.is_null() { (*zv).value.ptr } else { 0 as *mut libc::c_void };
}
#[inline(always)]
#[no_mangle] unsafe extern "C" fn zend_object_properties_size(
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
#[no_mangle] unsafe extern "C" fn zend_object_alloc(
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
#[inline(always)]
#[no_mangle] unsafe extern "C" fn zend_string_init(
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
#[no_mangle] unsafe extern "C" fn zend_string_alloc(
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
#[no_mangle] unsafe extern "C" fn zend_string_release(mut s: *mut zend_string) {
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
static mut arginfo_class_RdKafka___construct: [zend_internal_arg_info; 1] = [
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
static mut arginfo_class_RdKafka_addBrokers: [zend_internal_arg_info; 2] = unsafe {
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
                name: b"broker_list\0" as *const u8 as *const libc::c_char,
                type_0: 0 as libc::c_int as zend_type,
                pass_by_reference: 0 as libc::c_int as zend_uchar,
                is_variadic: 0 as libc::c_int as zend_bool,
            };
            init
        },
    ]
};
static mut arginfo_class_RdKafka_getMetadata: [zend_internal_arg_info; 4] = unsafe {
    [
        {
            let mut init = _zend_internal_arg_info {
                name: 3 as libc::c_int as zend_uintptr_t as *const libc::c_char,
                type_0: 0 as libc::c_int as zend_type,
                pass_by_reference: 0 as libc::c_int as zend_uchar,
                is_variadic: 0 as libc::c_int as zend_bool,
            };
            init
        },
        {
            let mut init = _zend_internal_arg_info {
                name: b"all_topics\0" as *const u8 as *const libc::c_char,
                type_0: 0 as libc::c_int as zend_type,
                pass_by_reference: 0 as libc::c_int as zend_uchar,
                is_variadic: 0 as libc::c_int as zend_bool,
            };
            init
        },
        {
            let mut init = _zend_internal_arg_info {
                name: b"only_topic\0" as *const u8 as *const libc::c_char,
                type_0: 0 as libc::c_int as zend_type,
                pass_by_reference: 0 as libc::c_int as zend_uchar,
                is_variadic: 0 as libc::c_int as zend_bool,
            };
            init
        },
        {
            let mut init = _zend_internal_arg_info {
                name: b"timeout_ms\0" as *const u8 as *const libc::c_char,
                type_0: 0 as libc::c_int as zend_type,
                pass_by_reference: 0 as libc::c_int as zend_uchar,
                is_variadic: 0 as libc::c_int as zend_bool,
            };
            init
        },
    ]
};
static mut arginfo_class_RdKafka_getOutQLen: [zend_internal_arg_info; 1] = [
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
static mut arginfo_class_RdKafka_setLogLevel: [zend_internal_arg_info; 2] = unsafe {
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
                name: b"level\0" as *const u8 as *const libc::c_char,
                type_0: 0 as libc::c_int as zend_type,
                pass_by_reference: 0 as libc::c_int as zend_uchar,
                is_variadic: 0 as libc::c_int as zend_bool,
            };
            init
        },
    ]
};
static mut arginfo_class_RdKafka_newTopic: [zend_internal_arg_info; 3] = unsafe {
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
                name: b"topic_name\0" as *const u8 as *const libc::c_char,
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
static mut arginfo_class_RdKafka_poll: [zend_internal_arg_info; 2] = unsafe {
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
                name: b"timeout_ms\0" as *const u8 as *const libc::c_char,
                type_0: 0 as libc::c_int as zend_type,
                pass_by_reference: 0 as libc::c_int as zend_uchar,
                is_variadic: 0 as libc::c_int as zend_bool,
            };
            init
        },
    ]
};
static mut arginfo_class_RdKafka_purge: [zend_internal_arg_info; 2] = unsafe {
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
                name: b"purge_flags\0" as *const u8 as *const libc::c_char,
                type_0: 0 as libc::c_int as zend_type,
                pass_by_reference: 0 as libc::c_int as zend_uchar,
                is_variadic: 0 as libc::c_int as zend_bool,
            };
            init
        },
    ]
};
static mut arginfo_class_RdKafka_setLogger: [zend_internal_arg_info; 2] = unsafe {
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
                name: b"logger\0" as *const u8 as *const libc::c_char,
                type_0: 0 as libc::c_int as zend_type,
                pass_by_reference: 0 as libc::c_int as zend_uchar,
                is_variadic: 0 as libc::c_int as zend_bool,
            };
            init
        },
    ]
};
static mut arginfo_class_RdKafka_queryWatermarkOffsets: [zend_internal_arg_info; 6] = unsafe {
    [
        {
            let mut init = _zend_internal_arg_info {
                name: 5 as libc::c_int as zend_uintptr_t as *const libc::c_char,
                type_0: 0 as libc::c_int as zend_type,
                pass_by_reference: 0 as libc::c_int as zend_uchar,
                is_variadic: 0 as libc::c_int as zend_bool,
            };
            init
        },
        {
            let mut init = _zend_internal_arg_info {
                name: b"topic\0" as *const u8 as *const libc::c_char,
                type_0: 0 as libc::c_int as zend_type,
                pass_by_reference: 0 as libc::c_int as zend_uchar,
                is_variadic: 0 as libc::c_int as zend_bool,
            };
            init
        },
        {
            let mut init = _zend_internal_arg_info {
                name: b"partition\0" as *const u8 as *const libc::c_char,
                type_0: 0 as libc::c_int as zend_type,
                pass_by_reference: 0 as libc::c_int as zend_uchar,
                is_variadic: 0 as libc::c_int as zend_bool,
            };
            init
        },
        {
            let mut init = _zend_internal_arg_info {
                name: b"low\0" as *const u8 as *const libc::c_char,
                type_0: 0 as libc::c_int as zend_type,
                pass_by_reference: 1 as libc::c_int as zend_uchar,
                is_variadic: 0 as libc::c_int as zend_bool,
            };
            init
        },
        {
            let mut init = _zend_internal_arg_info {
                name: b"high\0" as *const u8 as *const libc::c_char,
                type_0: 0 as libc::c_int as zend_type,
                pass_by_reference: 1 as libc::c_int as zend_uchar,
                is_variadic: 0 as libc::c_int as zend_bool,
            };
            init
        },
        {
            let mut init = _zend_internal_arg_info {
                name: b"timeout_ms\0" as *const u8 as *const libc::c_char,
                type_0: 0 as libc::c_int as zend_type,
                pass_by_reference: 0 as libc::c_int as zend_uchar,
                is_variadic: 0 as libc::c_int as zend_bool,
            };
            init
        },
    ]
};
static mut arginfo_class_RdKafka_offsetsForTimes: [zend_internal_arg_info; 3] = unsafe {
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
                name: b"topic_partitions\0" as *const u8 as *const libc::c_char,
                type_0: 0 as libc::c_int as zend_type,
                pass_by_reference: 0 as libc::c_int as zend_uchar,
                is_variadic: 0 as libc::c_int as zend_bool,
            };
            init
        },
        {
            let mut init = _zend_internal_arg_info {
                name: b"timeout_ms\0" as *const u8 as *const libc::c_char,
                type_0: 0 as libc::c_int as zend_type,
                pass_by_reference: 0 as libc::c_int as zend_uchar,
                is_variadic: 0 as libc::c_int as zend_bool,
            };
            init
        },
    ]
};
static mut arginfo_class_RdKafka_pausePartitions: [zend_internal_arg_info; 2] = unsafe {
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
                name: b"topic_partitions\0" as *const u8 as *const libc::c_char,
                type_0: 0 as libc::c_int as zend_type,
                pass_by_reference: 0 as libc::c_int as zend_uchar,
                is_variadic: 0 as libc::c_int as zend_bool,
            };
            init
        },
    ]
};
static mut arginfo_class_RdKafka_Consumer___construct: [zend_internal_arg_info; 2] = [
    {
        let mut init = _zend_internal_arg_info {
            name: 0 as *const libc::c_char,
            type_0: 0 as libc::c_int as zend_type,
            pass_by_reference: 0 as libc::c_int as zend_uchar,
            is_variadic: 0 as libc::c_int as zend_bool,
        };
        init
    },
    {
        let mut init = _zend_internal_arg_info {
            name: b"conf\0" as *const u8 as *const libc::c_char,
            type_0: 0 as libc::c_int as zend_type,
            pass_by_reference: 0 as libc::c_int as zend_uchar,
            is_variadic: 0 as libc::c_int as zend_bool,
        };
        init
    },
];
static mut class_RdKafka_methods: [zend_function_entry; 17] = [zend_function_entry {
    fname: 0 as *const libc::c_char,
    handler: None,
    arg_info: 0 as *const _zend_internal_arg_info,
    num_args: 0,
    flags: 0,
}; 17];
static mut class_RdKafka_Exception_methods: [zend_function_entry; 1] = [
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
static mut class_RdKafka_Consumer_methods: [zend_function_entry; 3] = [zend_function_entry {
    fname: 0 as *const libc::c_char,
    handler: None,
    arg_info: 0 as *const _zend_internal_arg_info,
    num_args: 0,
    flags: 0,
}; 3];
static mut class_RdKafka_Producer_methods: [zend_function_entry; 2] = [zend_function_entry {
    fname: 0 as *const libc::c_char,
    handler: None,
    arg_info: 0 as *const _zend_internal_arg_info,
    num_args: 0,
    flags: 0,
}; 2];
#[no_mangle] unsafe extern "C" fn register_class_RdKafka() -> *mut zend_class_entry {
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
        b"RdKafka\0" as *const u8 as *const libc::c_char,
        (::std::mem::size_of::<[libc::c_char; 8]>() as libc::c_ulong)
            .wrapping_sub(1 as libc::c_int as libc::c_ulong),
        1 as libc::c_int,
    );
    ce.info.internal.builtin_functions = class_RdKafka_methods.as_ptr();
    class_entry = zend_register_internal_class_ex(&mut ce, 0 as *mut zend_class_entry);
    (*class_entry).ce_flags |= ((1 as libc::c_int) << 6 as libc::c_int) as libc::c_uint;
    let mut property_error_cb_default_value: zval = zval {
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
    property_error_cb_default_value.u1.type_info = 1 as libc::c_int as uint32_t;
    let mut property_error_cb_name: *mut zend_string = zend_string_init(
        b"error_cb\0" as *const u8 as *const libc::c_char,
        (::std::mem::size_of::<[libc::c_char; 9]>() as libc::c_ulong)
            .wrapping_sub(1 as libc::c_int as libc::c_ulong),
        1 as libc::c_int,
    );
    zend_declare_property_ex(
        class_entry,
        property_error_cb_name,
        &mut property_error_cb_default_value,
        (1 as libc::c_int) << 2 as libc::c_int,
        0 as *mut zend_string,
    );
    zend_string_release(property_error_cb_name);
    let mut property_dr_cb_default_value: zval = zval {
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
    property_dr_cb_default_value.u1.type_info = 1 as libc::c_int as uint32_t;
    let mut property_dr_cb_name: *mut zend_string = zend_string_init(
        b"dr_cb\0" as *const u8 as *const libc::c_char,
        (::std::mem::size_of::<[libc::c_char; 6]>() as libc::c_ulong)
            .wrapping_sub(1 as libc::c_int as libc::c_ulong),
        1 as libc::c_int,
    );
    zend_declare_property_ex(
        class_entry,
        property_dr_cb_name,
        &mut property_dr_cb_default_value,
        (1 as libc::c_int) << 2 as libc::c_int,
        0 as *mut zend_string,
    );
    zend_string_release(property_dr_cb_name);
    return class_entry;
}
#[no_mangle] unsafe extern "C" fn register_class_RdKafka_Exception(
    mut class_entry_Exception: *mut zend_class_entry,
) -> *mut zend_class_entry {
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
        b"RdKafka\\Exception\0" as *const u8 as *const libc::c_char,
        (::std::mem::size_of::<[libc::c_char; 18]>() as libc::c_ulong)
            .wrapping_sub(1 as libc::c_int as libc::c_ulong),
        1 as libc::c_int,
    );
    ce.info.internal.builtin_functions = class_RdKafka_Exception_methods.as_ptr();
    class_entry = zend_register_internal_class_ex(&mut ce, class_entry_Exception);
    return class_entry;
}
#[no_mangle] unsafe extern "C" fn register_class_RdKafka_Consumer(
    mut class_entry_RdKafka: *mut zend_class_entry,
) -> *mut zend_class_entry {
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
        b"RdKafka\\Consumer\0" as *const u8 as *const libc::c_char,
        (::std::mem::size_of::<[libc::c_char; 17]>() as libc::c_ulong)
            .wrapping_sub(1 as libc::c_int as libc::c_ulong),
        1 as libc::c_int,
    );
    ce.info.internal.builtin_functions = class_RdKafka_Consumer_methods.as_ptr();
    class_entry = zend_register_internal_class_ex(&mut ce, class_entry_RdKafka);
    return class_entry;
}
#[no_mangle] unsafe extern "C" fn register_class_RdKafka_Producer(
    mut class_entry_RdKafka: *mut zend_class_entry,
) -> *mut zend_class_entry {
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
        b"RdKafka\\Producer\0" as *const u8 as *const libc::c_char,
        (::std::mem::size_of::<[libc::c_char; 17]>() as libc::c_ulong)
            .wrapping_sub(1 as libc::c_int as libc::c_ulong),
        1 as libc::c_int,
    );
    ce.info.internal.builtin_functions = class_RdKafka_Producer_methods.as_ptr();
    class_entry = zend_register_internal_class_ex(&mut ce, class_entry_RdKafka);
    return class_entry;
}
static mut arginfo_rd_kafka_get_err_descs: [zend_internal_arg_info; 1] = [
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
static mut arginfo_rd_kafka_err2name: [zend_internal_arg_info; 2] = unsafe {
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
                name: b"err\0" as *const u8 as *const libc::c_char,
                type_0: 0 as libc::c_int as zend_type,
                pass_by_reference: 0 as libc::c_int as zend_uchar,
                is_variadic: 0 as libc::c_int as zend_bool,
            };
            init
        },
    ]
};
static mut arginfo_rd_kafka_errno2err: [zend_internal_arg_info; 2] = unsafe {
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
                name: b"errnox\0" as *const u8 as *const libc::c_char,
                type_0: 0 as libc::c_int as zend_type,
                pass_by_reference: 0 as libc::c_int as zend_uchar,
                is_variadic: 0 as libc::c_int as zend_bool,
            };
            init
        },
    ]
};
static mut arginfo_rd_kafka_offset_tail: [zend_internal_arg_info; 2] = unsafe {
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
                name: b"cnt\0" as *const u8 as *const libc::c_char,
                type_0: 0 as libc::c_int as zend_type,
                pass_by_reference: 0 as libc::c_int as zend_uchar,
                is_variadic: 0 as libc::c_int as zend_bool,
            };
            init
        },
    ]
};
static mut ext_functions: [zend_function_entry; 8] = [zend_function_entry {
    fname: 0 as *const libc::c_char,
    handler: None,
    arg_info: 0 as *const _zend_internal_arg_info,
    num_args: 0,
    flags: 0,
}; 8];
static mut kafka_object_handlers: zend_object_handlers = zend_object_handlers {
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
pub static mut kafka_default_object_handlers: zend_object_handlers = zend_object_handlers {
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
static mut ce_kafka: *mut zend_class_entry = 0 as *const zend_class_entry
    as *mut zend_class_entry;
static mut ce_kafka_consumer: *mut zend_class_entry = 0 as *const zend_class_entry
    as *mut zend_class_entry;
pub static mut ce_kafka_exception: *mut zend_class_entry = 0 as *const zend_class_entry
    as *mut zend_class_entry;
static mut ce_kafka_producer: *mut zend_class_entry = 0 as *const zend_class_entry
    as *mut zend_class_entry;
#[no_mangle] unsafe extern "C" fn stop_consuming_toppar_pp(mut tp: *mut *mut toppar) {
    rd_kafka_consume_stop((**tp).rkt, (**tp).partition);
}
#[no_mangle] unsafe extern "C" fn stop_consuming(mut intern: *mut kafka_object) {
    zend_hash_apply(
        &mut (*intern).consuming,
        ::std::mem::transmute::<
            Option::<unsafe extern "C" fn(*mut *mut toppar) -> ()>,
            apply_func_t,
        >(Some(stop_consuming_toppar_pp as unsafe extern "C" fn(*mut *mut toppar) -> ())),
    );
}
#[no_mangle] unsafe extern "C" fn kafka_free(mut object: *mut zend_object) {
    let mut intern: *mut kafka_object = (object as *mut libc::c_char)
        .offset(-(264 as libc::c_ulong as isize)) as *mut kafka_object;
    kafka_conf_callbacks_dtor(&mut (*intern).cbs);
    if !((*intern).rk).is_null() {
        if (*intern).type_0 as libc::c_uint
            == RD_KAFKA_CONSUMER as libc::c_int as libc::c_uint
        {
            stop_consuming(intern);
            zend_hash_destroy(&mut (*intern).consuming);
            zend_hash_destroy(&mut (*intern).queues);
        } else if (*intern).type_0 as libc::c_uint
            == RD_KAFKA_PRODUCER as libc::c_int as libc::c_uint
        {
            rd_kafka_purge((*intern).rk, 0x1 as libc::c_int | 0x2 as libc::c_int);
            rd_kafka_flush((*intern).rk, 0 as libc::c_int);
        }
        zend_hash_destroy(&mut (*intern).topics);
        rd_kafka_destroy((*intern).rk);
        (*intern).rk = 0 as *mut rd_kafka_t;
    }
    zend_object_std_dtor(&mut (*intern).std);
}
#[no_mangle] unsafe extern "C" fn toppar_pp_dtor(mut tp: *mut *mut toppar) {
    _efree(*tp as *mut libc::c_void);
}
#[no_mangle] unsafe extern "C" fn kafka_queue_object_pre_free(mut pp: *mut *mut kafka_queue_object) {
    let mut intern: *mut kafka_queue_object = *pp;
    rd_kafka_queue_destroy((*intern).rkqu);
    (*intern).rkqu = 0 as *mut rd_kafka_queue_t;
    zval_ptr_dtor(&mut (*intern).zrk);
}
#[no_mangle] unsafe extern "C" fn kafka_topic_object_pre_free(mut pp: *mut *mut kafka_topic_object) {
    let mut intern: *mut kafka_topic_object = *pp;
    rd_kafka_topic_destroy((*intern).rkt);
    (*intern).rkt = 0 as *mut rd_kafka_topic_t;
    zval_ptr_dtor(&mut (*intern).zrk);
}
#[no_mangle] unsafe extern "C" fn kafka_init(
    mut this_ptr: *mut zval,
    mut type_0: rd_kafka_type_t,
    mut zconf: *mut zval,
) {
    let mut errstr: [libc::c_char; 512] = [0; 512];
    let mut rk: *mut rd_kafka_t = 0 as *mut rd_kafka_t;
    let mut intern: *mut kafka_object = 0 as *mut kafka_object;
    let mut conf_intern: *mut kafka_conf_object = 0 as *mut kafka_conf_object;
    let mut conf: *mut rd_kafka_conf_t = 0 as *mut rd_kafka_conf_t;
    intern = ((*this_ptr).value.obj as *mut libc::c_char)
        .offset(-(264 as libc::c_ulong as isize)) as *mut kafka_object;
    (*intern).type_0 = type_0;
    if !zconf.is_null() {
        conf_intern = get_kafka_conf_object(zconf);
        if !conf_intern.is_null() {
            conf = rd_kafka_conf_dup((*conf_intern).u.conf);
            kafka_conf_callbacks_copy(&mut (*intern).cbs, &mut (*conf_intern).cbs);
        }
    }
    if conf.is_null() {
        conf = rd_kafka_conf_new();
    }
    (*intern).cbs.zrk = *this_ptr;
    rd_kafka_conf_set_opaque(
        conf,
        &mut (*intern).cbs as *mut kafka_conf_callbacks as *mut libc::c_void,
    );
    if type_0 as libc::c_uint == RD_KAFKA_PRODUCER as libc::c_int as libc::c_uint {
        rd_kafka_conf_set_dr_msg_cb(
            conf,
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
    rk = rd_kafka_new(
        type_0,
        conf,
        errstr.as_mut_ptr(),
        ::std::mem::size_of::<[libc::c_char; 512]>() as libc::c_ulong,
    );
    if rk.is_null() {
        zend_throw_exception(
            ce_kafka_exception,
            errstr.as_mut_ptr(),
            0 as libc::c_int as zend_long,
        );
        return;
    }
    if !((*intern).cbs.log).is_null() {
        rd_kafka_set_log_queue(rk, 0 as *mut rd_kafka_queue_t);
    }
    (*intern).rk = rk;
    if type_0 as libc::c_uint == RD_KAFKA_CONSUMER as libc::c_int as libc::c_uint {
        _zend_hash_init(
            &mut (*intern).consuming,
            0 as libc::c_int as uint32_t,
            ::std::mem::transmute::<
                Option::<unsafe extern "C" fn(*mut *mut toppar) -> ()>,
                dtor_func_t,
            >(Some(toppar_pp_dtor as unsafe extern "C" fn(*mut *mut toppar) -> ())),
            0 as libc::c_int as zend_bool,
        );
        _zend_hash_init(
            &mut (*intern).queues,
            0 as libc::c_int as uint32_t,
            ::std::mem::transmute::<
                Option::<unsafe extern "C" fn(*mut *mut kafka_queue_object) -> ()>,
                dtor_func_t,
            >(
                Some(
                    kafka_queue_object_pre_free
                        as unsafe extern "C" fn(*mut *mut kafka_queue_object) -> (),
                ),
            ),
            0 as libc::c_int as zend_bool,
        );
    }
    _zend_hash_init(
        &mut (*intern).topics,
        0 as libc::c_int as uint32_t,
        ::std::mem::transmute::<
            Option::<unsafe extern "C" fn(*mut *mut kafka_topic_object) -> ()>,
            dtor_func_t,
        >(
            Some(
                kafka_topic_object_pre_free
                    as unsafe extern "C" fn(*mut *mut kafka_topic_object) -> (),
            ),
        ),
        0 as libc::c_int as zend_bool,
    );
}
#[no_mangle] unsafe extern "C" fn kafka_new(
    mut class_type: *mut zend_class_entry,
) -> *mut zend_object {
    let mut retval: *mut zend_object = 0 as *mut zend_object;
    let mut intern: *mut kafka_object = 0 as *mut kafka_object;
    intern = zend_object_alloc(
        ::std::mem::size_of::<kafka_object>() as libc::c_ulong,
        class_type,
    ) as *mut kafka_object;
    zend_object_std_init(&mut (*intern).std, class_type);
    object_properties_init(&mut (*intern).std, class_type);
    retval = &mut (*intern).std;
    (*retval).handlers = &mut kafka_object_handlers;
    return retval;
}
#[no_mangle] pub unsafe extern "C" fn get_kafka_object(mut zrk: *mut zval) -> *mut kafka_object {
    let mut ork: *mut kafka_object = ((*zrk).value.obj as *mut libc::c_char)
        .offset(-(264 as libc::c_ulong as isize)) as *mut kafka_object;
    if ((*ork).rk).is_null() {
        zend_throw_exception_ex(
            0 as *mut zend_class_entry,
            0 as libc::c_int as zend_long,
            b"RdKafka\\Kafka::__construct() has not been called\0" as *const u8
                as *const libc::c_char,
        );
        return 0 as *mut kafka_object;
    }
    return ork;
}
#[no_mangle] unsafe extern "C" fn kafka_log_syslog_print(
    mut rk: *const rd_kafka_t,
    mut level: libc::c_int,
    mut fac: *const libc::c_char,
    mut buf: *const libc::c_char,
) {
    rd_kafka_log_print(rk, level, fac, buf);
    rd_kafka_log_syslog(rk, level, fac, buf);
}
#[no_mangle] pub unsafe extern "C" fn add_consuming_toppar(
    mut intern: *mut kafka_object,
    mut rkt: *mut rd_kafka_topic_t,
    mut partition: int32_t,
) {
    let mut key: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut key_len: libc::c_int = 0;
    let mut topic_name: *const libc::c_char = rd_kafka_topic_name(rkt);
    let mut tp: *mut toppar = 0 as *mut toppar;
    tp = (if 0 != 0 {
        if ::std::mem::size_of::<toppar>() as libc::c_ulong
            <= 8 as libc::c_int as libc::c_ulong
        {
            _emalloc_8()
        } else if ::std::mem::size_of::<toppar>() as libc::c_ulong
            <= 16 as libc::c_int as libc::c_ulong
        {
            _emalloc_16()
        } else if ::std::mem::size_of::<toppar>() as libc::c_ulong
            <= 24 as libc::c_int as libc::c_ulong
        {
            _emalloc_24()
        } else if ::std::mem::size_of::<toppar>() as libc::c_ulong
            <= 32 as libc::c_int as libc::c_ulong
        {
            _emalloc_32()
        } else if ::std::mem::size_of::<toppar>() as libc::c_ulong
            <= 40 as libc::c_int as libc::c_ulong
        {
            _emalloc_40()
        } else if ::std::mem::size_of::<toppar>() as libc::c_ulong
            <= 48 as libc::c_int as libc::c_ulong
        {
            _emalloc_48()
        } else if ::std::mem::size_of::<toppar>() as libc::c_ulong
            <= 56 as libc::c_int as libc::c_ulong
        {
            _emalloc_56()
        } else if ::std::mem::size_of::<toppar>() as libc::c_ulong
            <= 64 as libc::c_int as libc::c_ulong
        {
            _emalloc_64()
        } else if ::std::mem::size_of::<toppar>() as libc::c_ulong
            <= 80 as libc::c_int as libc::c_ulong
        {
            _emalloc_80()
        } else if ::std::mem::size_of::<toppar>() as libc::c_ulong
            <= 96 as libc::c_int as libc::c_ulong
        {
            _emalloc_96()
        } else if ::std::mem::size_of::<toppar>() as libc::c_ulong
            <= 112 as libc::c_int as libc::c_ulong
        {
            _emalloc_112()
        } else if ::std::mem::size_of::<toppar>() as libc::c_ulong
            <= 128 as libc::c_int as libc::c_ulong
        {
            _emalloc_128()
        } else if ::std::mem::size_of::<toppar>() as libc::c_ulong
            <= 160 as libc::c_int as libc::c_ulong
        {
            _emalloc_160()
        } else if ::std::mem::size_of::<toppar>() as libc::c_ulong
            <= 192 as libc::c_int as libc::c_ulong
        {
            _emalloc_192()
        } else if ::std::mem::size_of::<toppar>() as libc::c_ulong
            <= 224 as libc::c_int as libc::c_ulong
        {
            _emalloc_224()
        } else if ::std::mem::size_of::<toppar>() as libc::c_ulong
            <= 256 as libc::c_int as libc::c_ulong
        {
            _emalloc_256()
        } else if ::std::mem::size_of::<toppar>() as libc::c_ulong
            <= 320 as libc::c_int as libc::c_ulong
        {
            _emalloc_320()
        } else if ::std::mem::size_of::<toppar>() as libc::c_ulong
            <= 384 as libc::c_int as libc::c_ulong
        {
            _emalloc_384()
        } else if ::std::mem::size_of::<toppar>() as libc::c_ulong
            <= 448 as libc::c_int as libc::c_ulong
        {
            _emalloc_448()
        } else if ::std::mem::size_of::<toppar>() as libc::c_ulong
            <= 512 as libc::c_int as libc::c_ulong
        {
            _emalloc_512()
        } else if ::std::mem::size_of::<toppar>() as libc::c_ulong
            <= 640 as libc::c_int as libc::c_ulong
        {
            _emalloc_640()
        } else if ::std::mem::size_of::<toppar>() as libc::c_ulong
            <= 768 as libc::c_int as libc::c_ulong
        {
            _emalloc_768()
        } else if ::std::mem::size_of::<toppar>() as libc::c_ulong
            <= 896 as libc::c_int as libc::c_ulong
        {
            _emalloc_896()
        } else if ::std::mem::size_of::<toppar>() as libc::c_ulong
            <= 1024 as libc::c_int as libc::c_ulong
        {
            _emalloc_1024()
        } else if ::std::mem::size_of::<toppar>() as libc::c_ulong
            <= 1280 as libc::c_int as libc::c_ulong
        {
            _emalloc_1280()
        } else if ::std::mem::size_of::<toppar>() as libc::c_ulong
            <= 1536 as libc::c_int as libc::c_ulong
        {
            _emalloc_1536()
        } else if ::std::mem::size_of::<toppar>() as libc::c_ulong
            <= 1792 as libc::c_int as libc::c_ulong
        {
            _emalloc_1792()
        } else if ::std::mem::size_of::<toppar>() as libc::c_ulong
            <= 2048 as libc::c_int as libc::c_ulong
        {
            _emalloc_2048()
        } else if ::std::mem::size_of::<toppar>() as libc::c_ulong
            <= 2560 as libc::c_int as libc::c_ulong
        {
            _emalloc_2560()
        } else if ::std::mem::size_of::<toppar>() as libc::c_ulong
            <= 3072 as libc::c_int as libc::c_ulong
        {
            _emalloc_3072()
        } else if ::std::mem::size_of::<toppar>() as libc::c_ulong
            <= (2 as libc::c_int * 1024 as libc::c_int * 1024 as libc::c_int
                - 4 as libc::c_int * 1024 as libc::c_int * 1 as libc::c_int)
                as libc::c_ulong
        {
            _emalloc_large(::std::mem::size_of::<toppar>() as libc::c_ulong)
        } else {
            _emalloc_huge(::std::mem::size_of::<toppar>() as libc::c_ulong)
        }
    } else {
        _emalloc(::std::mem::size_of::<toppar>() as libc::c_ulong)
    }) as *mut toppar;
    (*tp).rkt = rkt;
    (*tp).partition = partition;
    key_len = zend_spprintf(
        &mut key as *mut *mut libc::c_char,
        0 as libc::c_int as size_t,
        b"%s:%d\0" as *const u8 as *const libc::c_char,
        topic_name,
        partition,
    ) as libc::c_int;
    zend_hash_str_add_ptr(
        &mut (*intern).consuming,
        key,
        (key_len + 1 as libc::c_int) as size_t,
        tp as *mut libc::c_void,
    );
    _efree(key as *mut libc::c_void);
}
#[no_mangle] pub unsafe extern "C" fn del_consuming_toppar(
    mut intern: *mut kafka_object,
    mut rkt: *mut rd_kafka_topic_t,
    mut partition: int32_t,
) {
    let mut key: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut key_len: libc::c_int = 0;
    let mut topic_name: *const libc::c_char = rd_kafka_topic_name(rkt);
    key_len = zend_spprintf(
        &mut key as *mut *mut libc::c_char,
        0 as libc::c_int as size_t,
        b"%s:%d\0" as *const u8 as *const libc::c_char,
        topic_name,
        partition,
    ) as libc::c_int;
    zend_hash_str_del(
        &mut (*intern).consuming,
        key,
        (key_len + 1 as libc::c_int) as size_t,
    );
    _efree(key as *mut libc::c_void);
}
#[no_mangle] pub unsafe extern "C" fn is_consuming_toppar(
    mut intern: *mut kafka_object,
    mut rkt: *mut rd_kafka_topic_t,
    mut partition: int32_t,
) -> libc::c_int {
    let mut key: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut key_len: libc::c_int = 0;
    let mut topic_name: *const libc::c_char = rd_kafka_topic_name(rkt);
    let mut ret: libc::c_int = 0;
    key_len = zend_spprintf(
        &mut key as *mut *mut libc::c_char,
        0 as libc::c_int as size_t,
        b"%s:%d\0" as *const u8 as *const libc::c_char,
        topic_name,
        partition,
    ) as libc::c_int;
    ret = zend_hash_str_exists(
        &mut (*intern).consuming,
        key,
        (key_len + 1 as libc::c_int) as size_t,
    ) as libc::c_int;
    _efree(key as *mut libc::c_void);
    return ret;
}
#[no_mangle] pub unsafe extern "C" fn zim_RdKafka___construct(
    mut execute_data: *mut zend_execute_data,
    mut return_value: *mut zval,
) {
    zend_throw_exception(
        0 as *mut zend_class_entry,
        b"Private constructor\0" as *const u8 as *const libc::c_char,
        0 as libc::c_int as zend_long,
    );
}
#[no_mangle] pub unsafe extern "C" fn zim_RdKafka_Consumer___construct(
    mut execute_data: *mut zend_execute_data,
    mut return_value: *mut zval,
) {
    let mut zconf: *mut zval = 0 as *mut zval;
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
        b"|O!\0" as *const u8 as *const libc::c_char,
        &mut zconf as *mut *mut zval,
        ce_kafka_conf,
    ) == FAILURE as libc::c_int
    {
        zend_restore_error_handling(&mut error_handling);
        return;
    }
    kafka_init(
        if zval_get_type(&mut (*execute_data).This) as libc::c_int == 8 as libc::c_int {
            &mut (*execute_data).This
        } else {
            0 as *mut zval
        },
        RD_KAFKA_CONSUMER,
        zconf,
    );
    zend_restore_error_handling(&mut error_handling);
}
#[no_mangle] pub unsafe extern "C" fn zim_RdKafka_Consumer_newQueue(
    mut execute_data: *mut zend_execute_data,
    mut return_value: *mut zval,
) {
    let mut rkqu: *mut rd_kafka_queue_t = 0 as *mut rd_kafka_queue_t;
    let mut intern: *mut kafka_object = 0 as *mut kafka_object;
    let mut queue_intern: *mut kafka_queue_object = 0 as *mut kafka_queue_object;
    if zend_parse_parameters(
        (*execute_data).This.u2.num_args as libc::c_int,
        b"\0" as *const u8 as *const libc::c_char,
    ) == FAILURE as libc::c_int
    {
        return;
    }
    intern = get_kafka_object(
        if zval_get_type(&mut (*execute_data).This) as libc::c_int == 8 as libc::c_int {
            &mut (*execute_data).This
        } else {
            0 as *mut zval
        },
    );
    if intern.is_null() {
        return;
    }
    rkqu = rd_kafka_queue_new((*intern).rk);
    if rkqu.is_null() {
        return;
    }
    if object_init_ex(return_value, ce_kafka_queue) != SUCCESS as libc::c_int {
        return;
    }
    queue_intern = ((*return_value).value.obj as *mut libc::c_char)
        .offset(-(24 as libc::c_ulong as isize)) as *mut kafka_queue_object;
    if queue_intern.is_null() {
        return;
    }
    (*queue_intern).rkqu = rkqu;
    (*queue_intern)
        .zrk = *if zval_get_type(&mut (*execute_data).This) as libc::c_int
        == 8 as libc::c_int
    {
        &mut (*execute_data).This
    } else {
        0 as *mut zval
    };
    zval_addref_p(&mut (*queue_intern).zrk);
    zend_hash_index_add_ptr(
        &mut (*intern).queues,
        queue_intern as zend_ulong,
        queue_intern as *mut libc::c_void,
    );
}
#[no_mangle] pub unsafe extern "C" fn zim_RdKafka_addBrokers(
    mut execute_data: *mut zend_execute_data,
    mut return_value: *mut zval,
) {
    let mut broker_list: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut broker_list_len: size_t = 0;
    let mut intern: *mut kafka_object = 0 as *mut kafka_object;
    if zend_parse_parameters(
        (*execute_data).This.u2.num_args as libc::c_int,
        b"s\0" as *const u8 as *const libc::c_char,
        &mut broker_list as *mut *mut libc::c_char,
        &mut broker_list_len as *mut size_t,
    ) == FAILURE as libc::c_int
    {
        return;
    }
    intern = get_kafka_object(
        if zval_get_type(&mut (*execute_data).This) as libc::c_int == 8 as libc::c_int {
            &mut (*execute_data).This
        } else {
            0 as *mut zval
        },
    );
    if intern.is_null() {
        return;
    }
    let mut __z: *mut zval = return_value;
    (*__z).value.lval = rd_kafka_brokers_add((*intern).rk, broker_list) as zend_long;
    (*__z).u1.type_info = 4 as libc::c_int as uint32_t;
}
#[no_mangle] pub unsafe extern "C" fn zim_RdKafka_getMetadata(
    mut execute_data: *mut zend_execute_data,
    mut return_value: *mut zval,
) {
    let mut all_topics: zend_bool = 0;
    let mut only_zrkt: *mut zval = 0 as *mut zval;
    let mut timeout_ms: zend_long = 0;
    let mut err: rd_kafka_resp_err_t = RD_KAFKA_RESP_ERR_NO_ERROR;
    let mut intern: *mut kafka_object = 0 as *mut kafka_object;
    let mut metadata: *const rd_kafka_metadata_t = 0 as *const rd_kafka_metadata_t;
    let mut only_orkt: *mut kafka_topic_object = 0 as *mut kafka_topic_object;
    if zend_parse_parameters(
        (*execute_data).This.u2.num_args as libc::c_int,
        b"bO!l\0" as *const u8 as *const libc::c_char,
        &mut all_topics as *mut zend_bool,
        &mut only_zrkt as *mut *mut zval,
        ce_kafka_topic,
        &mut timeout_ms as *mut zend_long,
    ) == FAILURE as libc::c_int
    {
        return;
    }
    intern = get_kafka_object(
        if zval_get_type(&mut (*execute_data).This) as libc::c_int == 8 as libc::c_int {
            &mut (*execute_data).This
        } else {
            0 as *mut zval
        },
    );
    if intern.is_null() {
        return;
    }
    if !only_zrkt.is_null() {
        only_orkt = get_kafka_topic_object(only_zrkt);
        if only_orkt.is_null() {
            return;
        }
    }
    err = rd_kafka_metadata(
        (*intern).rk,
        all_topics as libc::c_int,
        if !only_orkt.is_null() { (*only_orkt).rkt } else { 0 as *mut rd_kafka_topic_t },
        &mut metadata,
        timeout_ms as libc::c_int,
    );
    if err as libc::c_int != RD_KAFKA_RESP_ERR_NO_ERROR as libc::c_int {
        zend_throw_exception(
            ce_kafka_exception,
            rd_kafka_err2str(err),
            err as zend_long,
        );
        return;
    }
    kafka_metadata_init(return_value, metadata);
}
#[no_mangle] pub unsafe extern "C" fn zim_RdKafka_setLogLevel(
    mut execute_data: *mut zend_execute_data,
    mut return_value: *mut zval,
) {
    let mut intern: *mut kafka_object = 0 as *mut kafka_object;
    let mut level: zend_long = 0;
    if zend_parse_parameters(
        (*execute_data).This.u2.num_args as libc::c_int,
        b"l\0" as *const u8 as *const libc::c_char,
        &mut level as *mut zend_long,
    ) == FAILURE as libc::c_int
    {
        return;
    }
    intern = get_kafka_object(
        if zval_get_type(&mut (*execute_data).This) as libc::c_int == 8 as libc::c_int {
            &mut (*execute_data).This
        } else {
            0 as *mut zval
        },
    );
    if intern.is_null() {
        return;
    }
    rd_kafka_set_log_level((*intern).rk, level as libc::c_int);
}
#[no_mangle] pub unsafe extern "C" fn zim_RdKafka_newTopic(
    mut execute_data: *mut zend_execute_data,
    mut return_value: *mut zval,
) {
    let mut topic: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut topic_len: size_t = 0;
    let mut rkt: *mut rd_kafka_topic_t = 0 as *mut rd_kafka_topic_t;
    let mut intern: *mut kafka_object = 0 as *mut kafka_object;
    let mut topic_intern: *mut kafka_topic_object = 0 as *mut kafka_topic_object;
    let mut topic_type: *mut zend_class_entry = 0 as *mut zend_class_entry;
    let mut zconf: *mut zval = 0 as *mut zval;
    let mut conf: *mut rd_kafka_topic_conf_t = 0 as *mut rd_kafka_topic_conf_t;
    let mut conf_intern: *mut kafka_conf_object = 0 as *mut kafka_conf_object;
    if zend_parse_parameters(
        (*execute_data).This.u2.num_args as libc::c_int,
        b"s|O!\0" as *const u8 as *const libc::c_char,
        &mut topic as *mut *mut libc::c_char,
        &mut topic_len as *mut size_t,
        &mut zconf as *mut *mut zval,
        ce_kafka_topic_conf,
    ) == FAILURE as libc::c_int
    {
        return;
    }
    intern = get_kafka_object(
        if zval_get_type(&mut (*execute_data).This) as libc::c_int == 8 as libc::c_int {
            &mut (*execute_data).This
        } else {
            0 as *mut zval
        },
    );
    if intern.is_null() {
        return;
    }
    if !zconf.is_null() {
        conf_intern = get_kafka_conf_object(zconf);
        if !conf_intern.is_null() {
            conf = rd_kafka_topic_conf_dup((*conf_intern).u.topic_conf);
        }
    }
    rkt = rd_kafka_topic_new((*intern).rk, topic, conf);
    if rkt.is_null() {
        return;
    }
    match (*intern).type_0 as libc::c_uint {
        1 => {
            topic_type = ce_kafka_consumer_topic;
        }
        0 => {
            topic_type = ce_kafka_producer_topic;
        }
        _ => return,
    }
    if object_init_ex(return_value, topic_type) != SUCCESS as libc::c_int {
        return;
    }
    topic_intern = ((*return_value).value.obj as *mut libc::c_char)
        .offset(-(24 as libc::c_ulong as isize)) as *mut kafka_topic_object;
    if topic_intern.is_null() {
        return;
    }
    (*topic_intern).rkt = rkt;
    (*topic_intern)
        .zrk = *if zval_get_type(&mut (*execute_data).This) as libc::c_int
        == 8 as libc::c_int
    {
        &mut (*execute_data).This
    } else {
        0 as *mut zval
    };
    zval_addref_p(&mut (*topic_intern).zrk);
    zend_hash_index_add_ptr(
        &mut (*intern).topics,
        topic_intern as zend_ulong,
        topic_intern as *mut libc::c_void,
    );
}
#[no_mangle] pub unsafe extern "C" fn zim_RdKafka_getOutQLen(
    mut execute_data: *mut zend_execute_data,
    mut return_value: *mut zval,
) {
    let mut intern: *mut kafka_object = 0 as *mut kafka_object;
    if zend_parse_parameters(
        (*execute_data).This.u2.num_args as libc::c_int,
        b"\0" as *const u8 as *const libc::c_char,
    ) == FAILURE as libc::c_int
    {
        return;
    }
    intern = get_kafka_object(
        if zval_get_type(&mut (*execute_data).This) as libc::c_int == 8 as libc::c_int {
            &mut (*execute_data).This
        } else {
            0 as *mut zval
        },
    );
    if intern.is_null() {
        return;
    }
    let mut __z: *mut zval = return_value;
    (*__z).value.lval = rd_kafka_outq_len((*intern).rk) as zend_long;
    (*__z).u1.type_info = 4 as libc::c_int as uint32_t;
}
#[no_mangle] pub unsafe extern "C" fn zim_RdKafka_poll(
    mut execute_data: *mut zend_execute_data,
    mut return_value: *mut zval,
) {
    let mut intern: *mut kafka_object = 0 as *mut kafka_object;
    let mut timeout: zend_long = 0;
    if zend_parse_parameters(
        (*execute_data).This.u2.num_args as libc::c_int,
        b"l\0" as *const u8 as *const libc::c_char,
        &mut timeout as *mut zend_long,
    ) == FAILURE as libc::c_int
    {
        return;
    }
    intern = get_kafka_object(
        if zval_get_type(&mut (*execute_data).This) as libc::c_int == 8 as libc::c_int {
            &mut (*execute_data).This
        } else {
            0 as *mut zval
        },
    );
    if intern.is_null() {
        return;
    }
    let mut __z: *mut zval = return_value;
    (*__z).value.lval = rd_kafka_poll((*intern).rk, timeout as libc::c_int) as zend_long;
    (*__z).u1.type_info = 4 as libc::c_int as uint32_t;
}
#[no_mangle] pub unsafe extern "C" fn zim_RdKafka_flush(
    mut execute_data: *mut zend_execute_data,
    mut return_value: *mut zval,
) {
    let mut intern: *mut kafka_object = 0 as *mut kafka_object;
    let mut timeout: zend_long = 0;
    if zend_parse_parameters(
        (*execute_data).This.u2.num_args as libc::c_int,
        b"l\0" as *const u8 as *const libc::c_char,
        &mut timeout as *mut zend_long,
    ) == FAILURE as libc::c_int
    {
        return;
    }
    intern = get_kafka_object(
        if zval_get_type(&mut (*execute_data).This) as libc::c_int == 8 as libc::c_int {
            &mut (*execute_data).This
        } else {
            0 as *mut zval
        },
    );
    if intern.is_null() {
        return;
    }
    let mut __z: *mut zval = return_value;
    (*__z)
        .value
        .lval = rd_kafka_flush((*intern).rk, timeout as libc::c_int) as zend_long;
    (*__z).u1.type_info = 4 as libc::c_int as uint32_t;
}
#[no_mangle] pub unsafe extern "C" fn zim_RdKafka_purge(
    mut execute_data: *mut zend_execute_data,
    mut return_value: *mut zval,
) {
    let mut intern: *mut kafka_object = 0 as *mut kafka_object;
    let mut purge_flags: zend_long = 0;
    if zend_parse_parameters(
        (*execute_data).This.u2.num_args as libc::c_int,
        b"l\0" as *const u8 as *const libc::c_char,
        &mut purge_flags as *mut zend_long,
    ) == FAILURE as libc::c_int
    {
        return;
    }
    intern = get_kafka_object(
        if zval_get_type(&mut (*execute_data).This) as libc::c_int == 8 as libc::c_int {
            &mut (*execute_data).This
        } else {
            0 as *mut zval
        },
    );
    if intern.is_null() {
        return;
    }
    let mut __z: *mut zval = return_value;
    (*__z)
        .value
        .lval = rd_kafka_purge((*intern).rk, purge_flags as libc::c_int) as zend_long;
    (*__z).u1.type_info = 4 as libc::c_int as uint32_t;
}
#[no_mangle] pub unsafe extern "C" fn zim_RdKafka_queryWatermarkOffsets(
    mut execute_data: *mut zend_execute_data,
    mut return_value: *mut zval,
) {
    let mut intern: *mut kafka_object = 0 as *mut kafka_object;
    let mut topic: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut topic_length: size_t = 0;
    let mut low: libc::c_long = 0;
    let mut high: libc::c_long = 0;
    let mut partition: zend_long = 0;
    let mut timeout: zend_long = 0;
    let mut lowResult: *mut zval = 0 as *mut zval;
    let mut highResult: *mut zval = 0 as *mut zval;
    let mut err: rd_kafka_resp_err_t = RD_KAFKA_RESP_ERR_NO_ERROR;
    if zend_parse_parameters(
        (*execute_data).This.u2.num_args as libc::c_int,
        b"slzzl\0" as *const u8 as *const libc::c_char,
        &mut topic as *mut *mut libc::c_char,
        &mut topic_length as *mut size_t,
        &mut partition as *mut zend_long,
        &mut lowResult as *mut *mut zval,
        &mut highResult as *mut *mut zval,
        &mut timeout as *mut zend_long,
    ) == FAILURE as libc::c_int
    {
        return;
    }
    if (zval_get_type(lowResult) as libc::c_int == 10 as libc::c_int) as libc::c_int
        as libc::c_long != 0
    {
        lowResult = &mut (*(*lowResult).value.ref_0).val;
    }
    if (zval_get_type(highResult) as libc::c_int == 10 as libc::c_int) as libc::c_int
        as libc::c_long != 0
    {
        highResult = &mut (*(*highResult).value.ref_0).val;
    }
    intern = get_kafka_object(
        if zval_get_type(&mut (*execute_data).This) as libc::c_int == 8 as libc::c_int {
            &mut (*execute_data).This
        } else {
            0 as *mut zval
        },
    );
    if intern.is_null() {
        return;
    }
    err = rd_kafka_query_watermark_offsets(
        (*intern).rk,
        topic,
        partition as int32_t,
        &mut low,
        &mut high,
        timeout as libc::c_int,
    );
    if err as libc::c_int != RD_KAFKA_RESP_ERR_NO_ERROR as libc::c_int {
        zend_throw_exception(
            ce_kafka_exception,
            rd_kafka_err2str(err),
            err as zend_long,
        );
        return;
    }
    let mut __z: *mut zval = lowResult;
    (*__z).value.lval = low;
    (*__z).u1.type_info = 4 as libc::c_int as uint32_t;
    let mut __z_0: *mut zval = highResult;
    (*__z_0).value.lval = high;
    (*__z_0).u1.type_info = 4 as libc::c_int as uint32_t;
}
#[no_mangle] pub unsafe extern "C" fn zim_RdKafka_offsetsForTimes(
    mut execute_data: *mut zend_execute_data,
    mut return_value: *mut zval,
) {
    let mut htopars: *mut HashTable = 0 as *mut HashTable;
    let mut intern: *mut kafka_object = 0 as *mut kafka_object;
    let mut topicPartitions: *mut rd_kafka_topic_partition_list_t = 0
        as *mut rd_kafka_topic_partition_list_t;
    let mut timeout_ms: zend_long = 0;
    let mut err: rd_kafka_resp_err_t = RD_KAFKA_RESP_ERR_NO_ERROR;
    if zend_parse_parameters(
        (*execute_data).This.u2.num_args as libc::c_int,
        b"hl\0" as *const u8 as *const libc::c_char,
        &mut htopars as *mut *mut HashTable,
        &mut timeout_ms as *mut zend_long,
    ) == FAILURE as libc::c_int
    {
        return;
    }
    intern = get_kafka_object(
        if zval_get_type(&mut (*execute_data).This) as libc::c_int == 8 as libc::c_int {
            &mut (*execute_data).This
        } else {
            0 as *mut zval
        },
    );
    if intern.is_null() {
        return;
    }
    topicPartitions = array_arg_to_kafka_topic_partition_list(1 as libc::c_int, htopars);
    if topicPartitions.is_null() {
        return;
    }
    err = rd_kafka_offsets_for_times(
        (*intern).rk,
        topicPartitions,
        timeout_ms as libc::c_int,
    );
    if err as libc::c_int != RD_KAFKA_RESP_ERR_NO_ERROR as libc::c_int {
        rd_kafka_topic_partition_list_destroy(topicPartitions);
        zend_throw_exception(
            ce_kafka_exception,
            rd_kafka_err2str(err),
            err as zend_long,
        );
        return;
    }
    kafka_topic_partition_list_to_array(return_value, topicPartitions);
    rd_kafka_topic_partition_list_destroy(topicPartitions);
}
#[no_mangle] pub unsafe extern "C" fn zim_RdKafka_setLogger(
    mut execute_data: *mut zend_execute_data,
    mut return_value: *mut zval,
) {
    let mut intern: *mut kafka_object = 0 as *mut kafka_object;
    let mut id: zend_long = 0;
    let mut logger: Option::<
        unsafe extern "C" fn(
            *const rd_kafka_t,
            libc::c_int,
            *const libc::c_char,
            *const libc::c_char,
        ) -> (),
    > = None;
    if zend_parse_parameters(
        (*execute_data).This.u2.num_args as libc::c_int,
        b"l\0" as *const u8 as *const libc::c_char,
        &mut id as *mut zend_long,
    ) == FAILURE as libc::c_int
    {
        return;
    }
    intern = get_kafka_object(
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
        100 => {
            logger = Some(
                rd_kafka_log_print
                    as unsafe extern "C" fn(
                        *const rd_kafka_t,
                        libc::c_int,
                        *const libc::c_char,
                        *const libc::c_char,
                    ) -> (),
            );
        }
        101 => {
            logger = Some(
                rd_kafka_log_syslog
                    as unsafe extern "C" fn(
                        *const rd_kafka_t,
                        libc::c_int,
                        *const libc::c_char,
                        *const libc::c_char,
                    ) -> (),
            );
        }
        102 => {
            logger = Some(
                kafka_log_syslog_print
                    as unsafe extern "C" fn(
                        *const rd_kafka_t,
                        libc::c_int,
                        *const libc::c_char,
                        *const libc::c_char,
                    ) -> (),
            );
        }
        _ => {
            zend_throw_exception_ex(
                0 as *mut zend_class_entry,
                0 as libc::c_int as zend_long,
                b"Invalid logger\0" as *const u8 as *const libc::c_char,
            );
            return;
        }
    }
    rd_kafka_set_logger((*intern).rk, logger);
}
#[no_mangle] pub unsafe extern "C" fn zim_RdKafka_pausePartitions(
    mut execute_data: *mut zend_execute_data,
    mut return_value: *mut zval,
) {
    let mut htopars: *mut HashTable = 0 as *mut HashTable;
    let mut topars: *mut rd_kafka_topic_partition_list_t = 0
        as *mut rd_kafka_topic_partition_list_t;
    let mut err: rd_kafka_resp_err_t = RD_KAFKA_RESP_ERR_NO_ERROR;
    let mut intern: *mut kafka_object = 0 as *mut kafka_object;
    if zend_parse_parameters(
        (*execute_data).This.u2.num_args as libc::c_int,
        b"h\0" as *const u8 as *const libc::c_char,
        &mut htopars as *mut *mut HashTable,
    ) == FAILURE as libc::c_int
    {
        return;
    }
    intern = get_kafka_object(
        if zval_get_type(&mut (*execute_data).This) as libc::c_int == 8 as libc::c_int {
            &mut (*execute_data).This
        } else {
            0 as *mut zval
        },
    );
    if intern.is_null() {
        return;
    }
    topars = array_arg_to_kafka_topic_partition_list(1 as libc::c_int, htopars);
    if topars.is_null() {
        return;
    }
    err = rd_kafka_pause_partitions((*intern).rk, topars);
    if err as libc::c_int != RD_KAFKA_RESP_ERR_NO_ERROR as libc::c_int {
        rd_kafka_topic_partition_list_destroy(topars);
        zend_throw_exception(
            ce_kafka_exception,
            rd_kafka_err2str(err),
            err as zend_long,
        );
        return;
    }
    kafka_topic_partition_list_to_array(return_value, topars);
    rd_kafka_topic_partition_list_destroy(topars);
}
#[no_mangle] pub unsafe extern "C" fn zim_RdKafka_resumePartitions(
    mut execute_data: *mut zend_execute_data,
    mut return_value: *mut zval,
) {
    let mut htopars: *mut HashTable = 0 as *mut HashTable;
    let mut topars: *mut rd_kafka_topic_partition_list_t = 0
        as *mut rd_kafka_topic_partition_list_t;
    let mut err: rd_kafka_resp_err_t = RD_KAFKA_RESP_ERR_NO_ERROR;
    let mut intern: *mut kafka_object = 0 as *mut kafka_object;
    if zend_parse_parameters(
        (*execute_data).This.u2.num_args as libc::c_int,
        b"h\0" as *const u8 as *const libc::c_char,
        &mut htopars as *mut *mut HashTable,
    ) == FAILURE as libc::c_int
    {
        return;
    }
    intern = get_kafka_object(
        if zval_get_type(&mut (*execute_data).This) as libc::c_int == 8 as libc::c_int {
            &mut (*execute_data).This
        } else {
            0 as *mut zval
        },
    );
    if intern.is_null() {
        return;
    }
    topars = array_arg_to_kafka_topic_partition_list(1 as libc::c_int, htopars);
    if topars.is_null() {
        return;
    }
    err = rd_kafka_pause_partitions((*intern).rk, topars);
    if err as libc::c_int != RD_KAFKA_RESP_ERR_NO_ERROR as libc::c_int {
        rd_kafka_topic_partition_list_destroy(topars);
        zend_throw_exception(
            ce_kafka_exception,
            rd_kafka_err2str(err),
            err as zend_long,
        );
        return;
    }
    kafka_topic_partition_list_to_array(return_value, topars);
    rd_kafka_topic_partition_list_destroy(topars);
}
#[no_mangle] pub unsafe extern "C" fn zim_RdKafka_Producer___construct(
    mut execute_data: *mut zend_execute_data,
    mut return_value: *mut zval,
) {
    let mut zconf: *mut zval = 0 as *mut zval;
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
        b"|O!\0" as *const u8 as *const libc::c_char,
        &mut zconf as *mut *mut zval,
        ce_kafka_conf,
    ) == FAILURE as libc::c_int
    {
        zend_restore_error_handling(&mut error_handling);
        return;
    }
    kafka_init(
        if zval_get_type(&mut (*execute_data).This) as libc::c_int == 8 as libc::c_int {
            &mut (*execute_data).This
        } else {
            0 as *mut zval
        },
        RD_KAFKA_PRODUCER,
        zconf,
    );
    zend_restore_error_handling(&mut error_handling);
}
#[no_mangle] pub unsafe extern "C" fn register_err_constants(
    mut type_0: libc::c_int,
    mut module_number: libc::c_int,
) {
    let mut errdescs: *const rd_kafka_err_desc = 0 as *const rd_kafka_err_desc;
    let mut cnt: size_t = 0;
    let mut i: size_t = 0;
    let mut buf: [libc::c_char; 128] = [0; 128];
    rd_kafka_get_err_descs(&mut errdescs, &mut cnt);
    i = 0 as libc::c_int as size_t;
    while i < cnt {
        let mut desc: *const rd_kafka_err_desc = &*errdescs.offset(i as isize)
            as *const rd_kafka_err_desc;
        let mut len: libc::c_int = 0;
        if !((*desc).name).is_null() {
            len = ap_php_snprintf(
                buf.as_mut_ptr(),
                ::std::mem::size_of::<[libc::c_char; 128]>() as libc::c_ulong,
                b"RD_KAFKA_RESP_ERR_%s\0" as *const u8 as *const libc::c_char,
                (*desc).name,
            );
            if len as size_t
                >= ::std::mem::size_of::<[libc::c_char; 128]>() as libc::c_ulong
            {
                len = (::std::mem::size_of::<[libc::c_char; 128]>() as libc::c_ulong)
                    .wrapping_sub(1 as libc::c_int as libc::c_ulong) as libc::c_int;
            }
            zend_register_long_constant(
                buf.as_mut_ptr(),
                len as size_t,
                (*desc).code as zend_long,
                (1 as libc::c_int) << 0 as libc::c_int
                    | (1 as libc::c_int) << 1 as libc::c_int,
                module_number,
            );
        }
        i = i.wrapping_add(1);
        i;
    }
}
#[no_mangle] pub unsafe extern "C" fn zm_startup_rdkafka(
    mut type_0: libc::c_int,
    mut module_number: libc::c_int,
) -> libc::c_int {
    zend_register_long_constant(
        b"RD_KAFKA_CONSUMER\0" as *const u8 as *const libc::c_char,
        (::std::mem::size_of::<[libc::c_char; 18]>() as libc::c_ulong)
            .wrapping_sub(1 as libc::c_int as libc::c_ulong),
        RD_KAFKA_CONSUMER as libc::c_int as zend_long,
        (1 as libc::c_int) << 0 as libc::c_int | (1 as libc::c_int) << 1 as libc::c_int,
        module_number,
    );
    zend_register_long_constant(
        b"RD_KAFKA_OFFSET_BEGINNING\0" as *const u8 as *const libc::c_char,
        (::std::mem::size_of::<[libc::c_char; 26]>() as libc::c_ulong)
            .wrapping_sub(1 as libc::c_int as libc::c_ulong),
        -(2 as libc::c_int) as zend_long,
        (1 as libc::c_int) << 0 as libc::c_int | (1 as libc::c_int) << 1 as libc::c_int,
        module_number,
    );
    zend_register_long_constant(
        b"RD_KAFKA_OFFSET_END\0" as *const u8 as *const libc::c_char,
        (::std::mem::size_of::<[libc::c_char; 20]>() as libc::c_ulong)
            .wrapping_sub(1 as libc::c_int as libc::c_ulong),
        -(1 as libc::c_int) as zend_long,
        (1 as libc::c_int) << 0 as libc::c_int | (1 as libc::c_int) << 1 as libc::c_int,
        module_number,
    );
    zend_register_long_constant(
        b"RD_KAFKA_OFFSET_STORED\0" as *const u8 as *const libc::c_char,
        (::std::mem::size_of::<[libc::c_char; 23]>() as libc::c_ulong)
            .wrapping_sub(1 as libc::c_int as libc::c_ulong),
        -(1000 as libc::c_int) as zend_long,
        (1 as libc::c_int) << 0 as libc::c_int | (1 as libc::c_int) << 1 as libc::c_int,
        module_number,
    );
    zend_register_long_constant(
        b"RD_KAFKA_PARTITION_UA\0" as *const u8 as *const libc::c_char,
        (::std::mem::size_of::<[libc::c_char; 22]>() as libc::c_ulong)
            .wrapping_sub(1 as libc::c_int as libc::c_ulong),
        -(1 as libc::c_int) as zend_long,
        (1 as libc::c_int) << 0 as libc::c_int | (1 as libc::c_int) << 1 as libc::c_int,
        module_number,
    );
    zend_register_long_constant(
        b"RD_KAFKA_PRODUCER\0" as *const u8 as *const libc::c_char,
        (::std::mem::size_of::<[libc::c_char; 18]>() as libc::c_ulong)
            .wrapping_sub(1 as libc::c_int as libc::c_ulong),
        RD_KAFKA_PRODUCER as libc::c_int as zend_long,
        (1 as libc::c_int) << 0 as libc::c_int | (1 as libc::c_int) << 1 as libc::c_int,
        module_number,
    );
    zend_register_long_constant(
        b"RD_KAFKA_MSG_F_BLOCK\0" as *const u8 as *const libc::c_char,
        (::std::mem::size_of::<[libc::c_char; 21]>() as libc::c_ulong)
            .wrapping_sub(1 as libc::c_int as libc::c_ulong),
        0x4 as libc::c_int as zend_long,
        (1 as libc::c_int) << 0 as libc::c_int | (1 as libc::c_int) << 1 as libc::c_int,
        module_number,
    );
    zend_register_long_constant(
        b"RD_KAFKA_PURGE_F_QUEUE\0" as *const u8 as *const libc::c_char,
        (::std::mem::size_of::<[libc::c_char; 23]>() as libc::c_ulong)
            .wrapping_sub(1 as libc::c_int as libc::c_ulong),
        0x1 as libc::c_int as zend_long,
        (1 as libc::c_int) << 0 as libc::c_int | (1 as libc::c_int) << 1 as libc::c_int,
        module_number,
    );
    zend_register_long_constant(
        b"RD_KAFKA_PURGE_F_INFLIGHT\0" as *const u8 as *const libc::c_char,
        (::std::mem::size_of::<[libc::c_char; 26]>() as libc::c_ulong)
            .wrapping_sub(1 as libc::c_int as libc::c_ulong),
        0x2 as libc::c_int as zend_long,
        (1 as libc::c_int) << 0 as libc::c_int | (1 as libc::c_int) << 1 as libc::c_int,
        module_number,
    );
    zend_register_long_constant(
        b"RD_KAFKA_PURGE_F_NON_BLOCKING\0" as *const u8 as *const libc::c_char,
        (::std::mem::size_of::<[libc::c_char; 30]>() as libc::c_ulong)
            .wrapping_sub(1 as libc::c_int as libc::c_ulong),
        0x4 as libc::c_int as zend_long,
        (1 as libc::c_int) << 0 as libc::c_int | (1 as libc::c_int) << 1 as libc::c_int,
        module_number,
    );
    zend_register_long_constant(
        b"RD_KAFKA_VERSION\0" as *const u8 as *const libc::c_char,
        (::std::mem::size_of::<[libc::c_char; 17]>() as libc::c_ulong)
            .wrapping_sub(1 as libc::c_int as libc::c_ulong),
        rd_kafka_version() as zend_long,
        (1 as libc::c_int) << 0 as libc::c_int | (1 as libc::c_int) << 1 as libc::c_int,
        module_number,
    );
    zend_register_long_constant(
        b"RD_KAFKA_BUILD_VERSION\0" as *const u8 as *const libc::c_char,
        (::std::mem::size_of::<[libc::c_char; 23]>() as libc::c_ulong)
            .wrapping_sub(1 as libc::c_int as libc::c_ulong),
        0x10201ff as libc::c_int as zend_long,
        (1 as libc::c_int) << 0 as libc::c_int | (1 as libc::c_int) << 1 as libc::c_int,
        module_number,
    );
    register_err_constants(type_0, module_number);
    zend_register_long_constant(
        b"RD_KAFKA_CONF_UNKNOWN\0" as *const u8 as *const libc::c_char,
        (::std::mem::size_of::<[libc::c_char; 22]>() as libc::c_ulong)
            .wrapping_sub(1 as libc::c_int as libc::c_ulong),
        RD_KAFKA_CONF_UNKNOWN as libc::c_int as zend_long,
        (1 as libc::c_int) << 0 as libc::c_int | (1 as libc::c_int) << 1 as libc::c_int,
        module_number,
    );
    zend_register_long_constant(
        b"RD_KAFKA_CONF_INVALID\0" as *const u8 as *const libc::c_char,
        (::std::mem::size_of::<[libc::c_char; 22]>() as libc::c_ulong)
            .wrapping_sub(1 as libc::c_int as libc::c_ulong),
        RD_KAFKA_CONF_INVALID as libc::c_int as zend_long,
        (1 as libc::c_int) << 0 as libc::c_int | (1 as libc::c_int) << 1 as libc::c_int,
        module_number,
    );
    zend_register_long_constant(
        b"RD_KAFKA_CONF_OK\0" as *const u8 as *const libc::c_char,
        (::std::mem::size_of::<[libc::c_char; 17]>() as libc::c_ulong)
            .wrapping_sub(1 as libc::c_int as libc::c_ulong),
        RD_KAFKA_CONF_OK as libc::c_int as zend_long,
        (1 as libc::c_int) << 0 as libc::c_int | (1 as libc::c_int) << 1 as libc::c_int,
        module_number,
    );
    zend_register_long_constant(
        b"RD_KAFKA_MSG_PARTITIONER_RANDOM\0" as *const u8 as *const libc::c_char,
        (::std::mem::size_of::<[libc::c_char; 32]>() as libc::c_ulong)
            .wrapping_sub(1 as libc::c_int as libc::c_ulong),
        MSG_PARTITIONER_RANDOM as libc::c_int as zend_long,
        (1 as libc::c_int) << 0 as libc::c_int | (1 as libc::c_int) << 1 as libc::c_int,
        module_number,
    );
    zend_register_long_constant(
        b"RD_KAFKA_MSG_PARTITIONER_CONSISTENT\0" as *const u8 as *const libc::c_char,
        (::std::mem::size_of::<[libc::c_char; 36]>() as libc::c_ulong)
            .wrapping_sub(1 as libc::c_int as libc::c_ulong),
        MSG_PARTITIONER_CONSISTENT as libc::c_int as zend_long,
        (1 as libc::c_int) << 0 as libc::c_int | (1 as libc::c_int) << 1 as libc::c_int,
        module_number,
    );
    zend_register_long_constant(
        b"RD_KAFKA_MSG_PARTITIONER_CONSISTENT_RANDOM\0" as *const u8
            as *const libc::c_char,
        (::std::mem::size_of::<[libc::c_char; 43]>() as libc::c_ulong)
            .wrapping_sub(1 as libc::c_int as libc::c_ulong),
        MSG_PARTITIONER_CONSISTENT_RANDOM as libc::c_int as zend_long,
        (1 as libc::c_int) << 0 as libc::c_int | (1 as libc::c_int) << 1 as libc::c_int,
        module_number,
    );
    zend_register_long_constant(
        b"RD_KAFKA_MSG_PARTITIONER_MURMUR2\0" as *const u8 as *const libc::c_char,
        (::std::mem::size_of::<[libc::c_char; 33]>() as libc::c_ulong)
            .wrapping_sub(1 as libc::c_int as libc::c_ulong),
        MSG_PARTITIONER_MURMUR2 as libc::c_int as zend_long,
        (1 as libc::c_int) << 0 as libc::c_int | (1 as libc::c_int) << 1 as libc::c_int,
        module_number,
    );
    zend_register_long_constant(
        b"RD_KAFKA_MSG_PARTITIONER_MURMUR2_RANDOM\0" as *const u8 as *const libc::c_char,
        (::std::mem::size_of::<[libc::c_char; 40]>() as libc::c_ulong)
            .wrapping_sub(1 as libc::c_int as libc::c_ulong),
        MSG_PARTITIONER_MURMUR2_RANDOM as libc::c_int as zend_long,
        (1 as libc::c_int) << 0 as libc::c_int | (1 as libc::c_int) << 1 as libc::c_int,
        module_number,
    );
    zend_register_long_constant(
        b"RD_KAFKA_LOG_PRINT\0" as *const u8 as *const libc::c_char,
        (::std::mem::size_of::<[libc::c_char; 19]>() as libc::c_ulong)
            .wrapping_sub(1 as libc::c_int as libc::c_ulong),
        RD_KAFKA_LOG_PRINT as libc::c_int as zend_long,
        (1 as libc::c_int) << 0 as libc::c_int | (1 as libc::c_int) << 1 as libc::c_int,
        module_number,
    );
    zend_register_long_constant(
        b"RD_KAFKA_LOG_SYSLOG\0" as *const u8 as *const libc::c_char,
        (::std::mem::size_of::<[libc::c_char; 20]>() as libc::c_ulong)
            .wrapping_sub(1 as libc::c_int as libc::c_ulong),
        RD_KAFKA_LOG_SYSLOG as libc::c_int as zend_long,
        (1 as libc::c_int) << 0 as libc::c_int | (1 as libc::c_int) << 1 as libc::c_int,
        module_number,
    );
    zend_register_long_constant(
        b"RD_KAFKA_LOG_SYSLOG_PRINT\0" as *const u8 as *const libc::c_char,
        (::std::mem::size_of::<[libc::c_char; 26]>() as libc::c_ulong)
            .wrapping_sub(1 as libc::c_int as libc::c_ulong),
        RD_KAFKA_LOG_SYSLOG_PRINT as libc::c_int as zend_long,
        (1 as libc::c_int) << 0 as libc::c_int | (1 as libc::c_int) << 1 as libc::c_int,
        module_number,
    );
    memcpy(
        &mut kafka_default_object_handlers as *mut zend_object_handlers
            as *mut libc::c_void,
        &std_object_handlers as *const zend_object_handlers as *const libc::c_void,
        ::std::mem::size_of::<zend_object_handlers>() as libc::c_ulong,
    );
    kafka_default_object_handlers.clone_obj = None;
    kafka_object_handlers = kafka_default_object_handlers;
    kafka_object_handlers
        .free_obj = Some(kafka_free as unsafe extern "C" fn(*mut zend_object) -> ());
    kafka_object_handlers.offset = 264 as libc::c_ulong as libc::c_int;
    ce_kafka = register_class_RdKafka();
    (*ce_kafka)
        .c2rust_unnamed_0
        .create_object = Some(
        kafka_new as unsafe extern "C" fn(*mut zend_class_entry) -> *mut zend_object,
    );
    ce_kafka_consumer = register_class_RdKafka_Consumer(ce_kafka);
    ce_kafka_producer = register_class_RdKafka_Producer(ce_kafka);
    ce_kafka_exception = register_class_RdKafka_Exception(zend_ce_exception);
    kafka_conf_minit(type_0, module_number);
    kafka_kafka_consumer_minit(type_0, module_number);
    kafka_message_minit(type_0, module_number);
    kafka_metadata_minit(type_0, module_number);
    kafka_metadata_topic_partition_minit(type_0, module_number);
    kafka_queue_minit(type_0, module_number);
    kafka_topic_minit(type_0, module_number);
    return SUCCESS as libc::c_int;
}
#[no_mangle] pub unsafe extern "C" fn zm_info_rdkafka(mut zend_module: *mut zend_module_entry) {
    let mut rd_kafka_version_0: *mut libc::c_char = 0 as *mut libc::c_char;
    php_info_print_table_start();
    php_info_print_table_row(
        2 as libc::c_int,
        b"rdkafka support\0" as *const u8 as *const libc::c_char,
        b"enabled\0" as *const u8 as *const libc::c_char,
    );
    php_info_print_table_row(
        2 as libc::c_int,
        b"version\0" as *const u8 as *const libc::c_char,
        b"6.0.3\0" as *const u8 as *const libc::c_char,
    );
    php_info_print_table_row(
        2 as libc::c_int,
        b"build date\0" as *const u8 as *const libc::c_char,
        b"Jul 11 2025 16:12:49\0" as *const u8 as *const libc::c_char,
    );
    zend_spprintf(
        &mut rd_kafka_version_0 as *mut *mut libc::c_char,
        0 as libc::c_int as size_t,
        b"%u.%u.%u.%u\0" as *const u8 as *const libc::c_char,
        (0x10201ff as libc::c_int as libc::c_uint & 0xff000000 as libc::c_uint)
            >> 24 as libc::c_int,
        (0x10201ff as libc::c_int & 0xff0000 as libc::c_int) >> 16 as libc::c_int,
        (0x10201ff as libc::c_int & 0xff00 as libc::c_int) >> 8 as libc::c_int,
        0x10201ff as libc::c_int & 0xff as libc::c_int,
    );
    php_info_print_table_row(
        2 as libc::c_int,
        b"librdkafka version (runtime)\0" as *const u8 as *const libc::c_char,
        rd_kafka_version_str(),
    );
    php_info_print_table_row(
        2 as libc::c_int,
        b"librdkafka version (build)\0" as *const u8 as *const libc::c_char,
        rd_kafka_version_0,
    );
    _efree(rd_kafka_version_0 as *mut libc::c_void);
    php_info_print_table_end();
}
pub static mut rdkafka_module_entry: zend_module_entry = unsafe {
    {
        let mut init = _zend_module_entry {
            size: ::std::mem::size_of::<zend_module_entry>() as libc::c_ulong
                as libc::c_ushort,
            zend_api: 20190902 as libc::c_int as libc::c_uint,
            zend_debug: 0 as libc::c_int as libc::c_uchar,
            zts: 0 as libc::c_int as libc::c_uchar,
            ini_entry: 0 as *const _zend_ini_entry,
            deps: 0 as *const _zend_module_dep,
            name: b"rdkafka\0" as *const u8 as *const libc::c_char,
            functions: ext_functions.as_ptr(),
            module_startup_func: Some(
                zm_startup_rdkafka
                    as unsafe extern "C" fn(libc::c_int, libc::c_int) -> libc::c_int,
            ),
            module_shutdown_func: None,
            request_startup_func: None,
            request_shutdown_func: None,
            info_func: Some(
                zm_info_rdkafka as unsafe extern "C" fn(*mut zend_module_entry) -> (),
            ),
            version: b"6.0.3\0" as *const u8 as *const libc::c_char,
            globals_size: 0 as libc::c_int as size_t,
            globals_ptr: 0 as *const libc::c_void as *mut libc::c_void,
            globals_ctor: None,
            globals_dtor: None,
            post_deactivate_func: None,
            module_started: 0 as libc::c_int,
            type_0: 0 as libc::c_int as libc::c_uchar,
            handle: 0 as *const libc::c_void as *mut libc::c_void,
            module_number: 0 as libc::c_int,
            build_id: b"API20190902,NTS\0" as *const u8 as *const libc::c_char,
        };
        init
    }
};
#[no_mangle] pub unsafe extern "C" fn get_module() -> *mut zend_module_entry {
    return &mut rdkafka_module_entry;
}
#[no_mangle] unsafe extern "C" fn run_static_initializers() {
    class_RdKafka_methods = [
        {
            let mut init = _zend_function_entry {
                fname: b"__construct\0" as *const u8 as *const libc::c_char,
                handler: Some(
                    zim_RdKafka___construct
                        as unsafe extern "C" fn(*mut zend_execute_data, *mut zval) -> (),
                ),
                arg_info: arginfo_class_RdKafka___construct.as_ptr(),
                num_args: (::std::mem::size_of::<[zend_internal_arg_info; 1]>()
                    as libc::c_ulong)
                    .wrapping_div(
                        ::std::mem::size_of::<_zend_internal_arg_info>() as libc::c_ulong,
                    )
                    .wrapping_sub(1 as libc::c_int as libc::c_ulong) as uint32_t,
                flags: ((1 as libc::c_int) << 2 as libc::c_int) as uint32_t,
            };
            init
        },
        {
            let mut init = _zend_function_entry {
                fname: b"addBrokers\0" as *const u8 as *const libc::c_char,
                handler: Some(
                    zim_RdKafka_addBrokers
                        as unsafe extern "C" fn(*mut zend_execute_data, *mut zval) -> (),
                ),
                arg_info: arginfo_class_RdKafka_addBrokers.as_ptr(),
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
                fname: b"getMetadata\0" as *const u8 as *const libc::c_char,
                handler: Some(
                    zim_RdKafka_getMetadata
                        as unsafe extern "C" fn(*mut zend_execute_data, *mut zval) -> (),
                ),
                arg_info: arginfo_class_RdKafka_getMetadata.as_ptr(),
                num_args: (::std::mem::size_of::<[zend_internal_arg_info; 4]>()
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
                fname: b"getOutQLen\0" as *const u8 as *const libc::c_char,
                handler: Some(
                    zim_RdKafka_getOutQLen
                        as unsafe extern "C" fn(*mut zend_execute_data, *mut zval) -> (),
                ),
                arg_info: arginfo_class_RdKafka_getOutQLen.as_ptr(),
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
                fname: b"metadata\0" as *const u8 as *const libc::c_char,
                handler: Some(
                    zim_RdKafka_getMetadata
                        as unsafe extern "C" fn(*mut zend_execute_data, *mut zval) -> (),
                ),
                arg_info: arginfo_class_RdKafka_getMetadata.as_ptr(),
                num_args: (::std::mem::size_of::<[zend_internal_arg_info; 4]>()
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
                fname: b"setLogLevel\0" as *const u8 as *const libc::c_char,
                handler: Some(
                    zim_RdKafka_setLogLevel
                        as unsafe extern "C" fn(*mut zend_execute_data, *mut zval) -> (),
                ),
                arg_info: arginfo_class_RdKafka_setLogLevel.as_ptr(),
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
                fname: b"newTopic\0" as *const u8 as *const libc::c_char,
                handler: Some(
                    zim_RdKafka_newTopic
                        as unsafe extern "C" fn(*mut zend_execute_data, *mut zval) -> (),
                ),
                arg_info: arginfo_class_RdKafka_newTopic.as_ptr(),
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
                fname: b"outqLen\0" as *const u8 as *const libc::c_char,
                handler: Some(
                    zim_RdKafka_getOutQLen
                        as unsafe extern "C" fn(*mut zend_execute_data, *mut zval) -> (),
                ),
                arg_info: arginfo_class_RdKafka_getOutQLen.as_ptr(),
                num_args: (::std::mem::size_of::<[zend_internal_arg_info; 1]>()
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
                fname: b"poll\0" as *const u8 as *const libc::c_char,
                handler: Some(
                    zim_RdKafka_poll
                        as unsafe extern "C" fn(*mut zend_execute_data, *mut zval) -> (),
                ),
                arg_info: arginfo_class_RdKafka_poll.as_ptr(),
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
                fname: b"flush\0" as *const u8 as *const libc::c_char,
                handler: Some(
                    zim_RdKafka_flush
                        as unsafe extern "C" fn(*mut zend_execute_data, *mut zval) -> (),
                ),
                arg_info: arginfo_class_RdKafka_poll.as_ptr(),
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
                fname: b"purge\0" as *const u8 as *const libc::c_char,
                handler: Some(
                    zim_RdKafka_purge
                        as unsafe extern "C" fn(*mut zend_execute_data, *mut zval) -> (),
                ),
                arg_info: arginfo_class_RdKafka_purge.as_ptr(),
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
                fname: b"setLogger\0" as *const u8 as *const libc::c_char,
                handler: Some(
                    zim_RdKafka_setLogger
                        as unsafe extern "C" fn(*mut zend_execute_data, *mut zval) -> (),
                ),
                arg_info: arginfo_class_RdKafka_setLogger.as_ptr(),
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
                fname: b"queryWatermarkOffsets\0" as *const u8 as *const libc::c_char,
                handler: Some(
                    zim_RdKafka_queryWatermarkOffsets
                        as unsafe extern "C" fn(*mut zend_execute_data, *mut zval) -> (),
                ),
                arg_info: arginfo_class_RdKafka_queryWatermarkOffsets.as_ptr(),
                num_args: (::std::mem::size_of::<[zend_internal_arg_info; 6]>()
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
                fname: b"offsetsForTimes\0" as *const u8 as *const libc::c_char,
                handler: Some(
                    zim_RdKafka_offsetsForTimes
                        as unsafe extern "C" fn(*mut zend_execute_data, *mut zval) -> (),
                ),
                arg_info: arginfo_class_RdKafka_offsetsForTimes.as_ptr(),
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
                fname: b"pausePartitions\0" as *const u8 as *const libc::c_char,
                handler: Some(
                    zim_RdKafka_pausePartitions
                        as unsafe extern "C" fn(*mut zend_execute_data, *mut zval) -> (),
                ),
                arg_info: arginfo_class_RdKafka_pausePartitions.as_ptr(),
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
                fname: b"resumePartitions\0" as *const u8 as *const libc::c_char,
                handler: Some(
                    zim_RdKafka_resumePartitions
                        as unsafe extern "C" fn(*mut zend_execute_data, *mut zval) -> (),
                ),
                arg_info: arginfo_class_RdKafka_pausePartitions.as_ptr(),
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
    class_RdKafka_Consumer_methods = [
        {
            let mut init = _zend_function_entry {
                fname: b"__construct\0" as *const u8 as *const libc::c_char,
                handler: Some(
                    zim_RdKafka_Consumer___construct
                        as unsafe extern "C" fn(*mut zend_execute_data, *mut zval) -> (),
                ),
                arg_info: arginfo_class_RdKafka_Consumer___construct.as_ptr(),
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
                fname: b"newQueue\0" as *const u8 as *const libc::c_char,
                handler: Some(
                    zim_RdKafka_Consumer_newQueue
                        as unsafe extern "C" fn(*mut zend_execute_data, *mut zval) -> (),
                ),
                arg_info: arginfo_class_RdKafka_getOutQLen.as_ptr(),
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
                fname: 0 as *const libc::c_char,
                handler: None,
                arg_info: 0 as *const _zend_internal_arg_info,
                num_args: 0 as libc::c_int as uint32_t,
                flags: 0 as libc::c_int as uint32_t,
            };
            init
        },
    ];
    class_RdKafka_Producer_methods = [
        {
            let mut init = _zend_function_entry {
                fname: b"__construct\0" as *const u8 as *const libc::c_char,
                handler: Some(
                    zim_RdKafka_Producer___construct
                        as unsafe extern "C" fn(*mut zend_execute_data, *mut zval) -> (),
                ),
                arg_info: arginfo_class_RdKafka_Consumer___construct.as_ptr(),
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
    ext_functions = [
        {
            let mut init = _zend_function_entry {
                fname: b"rd_kafka_get_err_descs\0" as *const u8 as *const libc::c_char,
                handler: Some(
                    zif_rd_kafka_get_err_descs
                        as unsafe extern "C" fn(*mut zend_execute_data, *mut zval) -> (),
                ),
                arg_info: arginfo_rd_kafka_get_err_descs.as_ptr(),
                num_args: (::std::mem::size_of::<[zend_internal_arg_info; 1]>()
                    as libc::c_ulong)
                    .wrapping_div(
                        ::std::mem::size_of::<_zend_internal_arg_info>() as libc::c_ulong,
                    )
                    .wrapping_sub(1 as libc::c_int as libc::c_ulong) as uint32_t,
                flags: 0 as libc::c_int as uint32_t,
            };
            init
        },
        {
            let mut init = _zend_function_entry {
                fname: b"rd_kafka_err2name\0" as *const u8 as *const libc::c_char,
                handler: Some(
                    zif_rd_kafka_err2name
                        as unsafe extern "C" fn(*mut zend_execute_data, *mut zval) -> (),
                ),
                arg_info: arginfo_rd_kafka_err2name.as_ptr(),
                num_args: (::std::mem::size_of::<[zend_internal_arg_info; 2]>()
                    as libc::c_ulong)
                    .wrapping_div(
                        ::std::mem::size_of::<_zend_internal_arg_info>() as libc::c_ulong,
                    )
                    .wrapping_sub(1 as libc::c_int as libc::c_ulong) as uint32_t,
                flags: 0 as libc::c_int as uint32_t,
            };
            init
        },
        {
            let mut init = _zend_function_entry {
                fname: b"rd_kafka_err2str\0" as *const u8 as *const libc::c_char,
                handler: Some(
                    zif_rd_kafka_err2str
                        as unsafe extern "C" fn(*mut zend_execute_data, *mut zval) -> (),
                ),
                arg_info: arginfo_rd_kafka_err2name.as_ptr(),
                num_args: (::std::mem::size_of::<[zend_internal_arg_info; 2]>()
                    as libc::c_ulong)
                    .wrapping_div(
                        ::std::mem::size_of::<_zend_internal_arg_info>() as libc::c_ulong,
                    )
                    .wrapping_sub(1 as libc::c_int as libc::c_ulong) as uint32_t,
                flags: 0 as libc::c_int as uint32_t,
            };
            init
        },
        {
            let mut init = _zend_function_entry {
                fname: b"rd_kafka_errno2err\0" as *const u8 as *const libc::c_char,
                handler: Some(
                    zif_rd_kafka_errno2err
                        as unsafe extern "C" fn(*mut zend_execute_data, *mut zval) -> (),
                ),
                arg_info: arginfo_rd_kafka_errno2err.as_ptr(),
                num_args: (::std::mem::size_of::<[zend_internal_arg_info; 2]>()
                    as libc::c_ulong)
                    .wrapping_div(
                        ::std::mem::size_of::<_zend_internal_arg_info>() as libc::c_ulong,
                    )
                    .wrapping_sub(1 as libc::c_int as libc::c_ulong) as uint32_t,
                flags: ((1 as libc::c_int) << 11 as libc::c_int) as uint32_t,
            };
            init
        },
        {
            let mut init = _zend_function_entry {
                fname: b"rd_kafka_errno\0" as *const u8 as *const libc::c_char,
                handler: Some(
                    zif_rd_kafka_errno
                        as unsafe extern "C" fn(*mut zend_execute_data, *mut zval) -> (),
                ),
                arg_info: arginfo_rd_kafka_get_err_descs.as_ptr(),
                num_args: (::std::mem::size_of::<[zend_internal_arg_info; 1]>()
                    as libc::c_ulong)
                    .wrapping_div(
                        ::std::mem::size_of::<_zend_internal_arg_info>() as libc::c_ulong,
                    )
                    .wrapping_sub(1 as libc::c_int as libc::c_ulong) as uint32_t,
                flags: ((1 as libc::c_int) << 11 as libc::c_int) as uint32_t,
            };
            init
        },
        {
            let mut init = _zend_function_entry {
                fname: b"rd_kafka_offset_tail\0" as *const u8 as *const libc::c_char,
                handler: Some(
                    zif_rd_kafka_offset_tail
                        as unsafe extern "C" fn(*mut zend_execute_data, *mut zval) -> (),
                ),
                arg_info: arginfo_rd_kafka_offset_tail.as_ptr(),
                num_args: (::std::mem::size_of::<[zend_internal_arg_info; 2]>()
                    as libc::c_ulong)
                    .wrapping_div(
                        ::std::mem::size_of::<_zend_internal_arg_info>() as libc::c_ulong,
                    )
                    .wrapping_sub(1 as libc::c_int as libc::c_ulong) as uint32_t,
                flags: 0 as libc::c_int as uint32_t,
            };
            init
        },
        {
            let mut init = _zend_function_entry {
                fname: b"rd_kafka_thread_cnt\0" as *const u8 as *const libc::c_char,
                handler: Some(
                    zif_rd_kafka_thread_cnt
                        as unsafe extern "C" fn(*mut zend_execute_data, *mut zval) -> (),
                ),
                arg_info: arginfo_rd_kafka_get_err_descs.as_ptr(),
                num_args: (::std::mem::size_of::<[zend_internal_arg_info; 1]>()
                    as libc::c_ulong)
                    .wrapping_div(
                        ::std::mem::size_of::<_zend_internal_arg_info>() as libc::c_ulong,
                    )
                    .wrapping_sub(1 as libc::c_int as libc::c_ulong) as uint32_t,
                flags: 0 as libc::c_int as uint32_t,
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
