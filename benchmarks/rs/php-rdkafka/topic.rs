use ::libc;
extern "C" {
    pub type _zend_unserialize_data;
    pub type _zend_serialize_data;
    pub type rd_kafka_s;
    pub type rd_kafka_topic_s;
    pub type rd_kafka_queue_s;
    pub type rd_kafka_headers_s;
    static mut zend_string_init_interned: zend_string_init_interned_func_t;
    fn malloc(_: libc::c_ulong) -> *mut libc::c_void;
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
    fn _emalloc(size: size_t) -> *mut libc::c_void;
    fn _efree(ptr: *mut libc::c_void);
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
    fn zend_hash_index_del(ht: *mut HashTable, h: zend_ulong) -> libc::c_int;
    fn zend_hash_move_forward_ex(
        ht: *mut HashTable,
        pos: *mut HashPosition,
    ) -> libc::c_int;
    fn zend_hash_get_current_key_ex(
        ht: *const HashTable,
        str_index: *mut *mut zend_string,
        num_index: *mut zend_ulong,
        pos: *mut HashPosition,
    ) -> libc::c_int;
    fn zend_hash_get_current_data_ex(
        ht: *mut HashTable,
        pos: *mut HashPosition,
    ) -> *mut zval;
    fn zend_hash_internal_pointer_reset_ex(ht: *mut HashTable, pos: *mut HashPosition);
    fn zend_array_dup(source: *mut HashTable) -> *mut HashTable;
    fn zval_ptr_dtor(zval_ptr: *mut zval);
    static std_object_handlers: zend_object_handlers;
    fn _convert_to_string(op: *mut zval);
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
    fn zend_call_function(
        fci: *mut zend_fcall_info,
        fci_cache: *mut zend_fcall_info_cache,
    ) -> libc::c_int;
    fn zend_wrong_parameters_none_error() -> libc::c_int;
    fn zend_wrong_parameters_count_error(
        min_num_args: libc::c_int,
        max_num_args: libc::c_int,
    );
    fn zend_wrong_parameters_count_exception(
        min_num_args: libc::c_int,
        max_num_args: libc::c_int,
    );
    fn zend_wrong_parameter_type_error(
        num: libc::c_int,
        expected_type: zend_expected_type,
        arg: *mut zval,
    );
    fn zend_wrong_parameter_type_exception(
        num: libc::c_int,
        expected_type: zend_expected_type,
        arg: *mut zval,
    );
    fn zend_wrong_parameter_class_error(
        num: libc::c_int,
        name: *mut libc::c_char,
        arg: *mut zval,
    );
    fn zend_wrong_parameter_class_exception(
        num: libc::c_int,
        name: *mut libc::c_char,
        arg: *mut zval,
    );
    fn zend_wrong_callback_error(num: libc::c_int, error: *mut libc::c_char);
    fn zend_wrong_callback_exception(num: libc::c_int, error: *mut libc::c_char);
    fn zend_parse_arg_long_slow(arg: *mut zval, dest: *mut zend_long) -> libc::c_int;
    fn zend_parse_arg_long_cap_slow(arg: *mut zval, dest: *mut zend_long) -> libc::c_int;
    fn zend_parse_arg_str_slow(
        arg: *mut zval,
        dest: *mut *mut zend_string,
    ) -> libc::c_int;
    fn rd_kafka_err2str(err: rd_kafka_resp_err_t) -> *const libc::c_char;
    fn rd_kafka_last_error() -> rd_kafka_resp_err_t;
    fn rd_kafka_headers_new(initial_count: size_t) -> *mut rd_kafka_headers_t;
    fn rd_kafka_headers_destroy(hdrs: *mut rd_kafka_headers_t);
    fn rd_kafka_header_add(
        hdrs: *mut rd_kafka_headers_t,
        name: *const libc::c_char,
        name_size: ssize_t,
        value: *const libc::c_void,
        value_size: ssize_t,
    ) -> rd_kafka_resp_err_t;
    fn rd_kafka_message_destroy(rkmessage: *mut rd_kafka_message_t);
    fn rd_kafka_topic_name(rkt: *const rd_kafka_topic_t) -> *const libc::c_char;
    fn rd_kafka_consume_start(
        rkt: *mut rd_kafka_topic_t,
        partition: int32_t,
        offset: int64_t,
    ) -> libc::c_int;
    fn rd_kafka_consume_start_queue(
        rkt: *mut rd_kafka_topic_t,
        partition: int32_t,
        offset: int64_t,
        rkqu: *mut rd_kafka_queue_t,
    ) -> libc::c_int;
    fn rd_kafka_consume_stop(
        rkt: *mut rd_kafka_topic_t,
        partition: int32_t,
    ) -> libc::c_int;
    fn rd_kafka_consume(
        rkt: *mut rd_kafka_topic_t,
        partition: int32_t,
        timeout_ms: libc::c_int,
    ) -> *mut rd_kafka_message_t;
    fn rd_kafka_consume_batch(
        rkt: *mut rd_kafka_topic_t,
        partition: int32_t,
        timeout_ms: libc::c_int,
        rkmessages: *mut *mut rd_kafka_message_t,
        rkmessages_size: size_t,
    ) -> ssize_t;
    fn rd_kafka_consume_callback(
        rkt: *mut rd_kafka_topic_t,
        partition: int32_t,
        timeout_ms: libc::c_int,
        consume_cb: Option::<
            unsafe extern "C" fn(*mut rd_kafka_message_t, *mut libc::c_void) -> (),
        >,
        commit_opaque: *mut libc::c_void,
    ) -> libc::c_int;
    fn rd_kafka_offset_store(
        rkt: *mut rd_kafka_topic_t,
        partition: int32_t,
        offset: int64_t,
    ) -> rd_kafka_resp_err_t;
    fn rd_kafka_produce(
        rkt: *mut rd_kafka_topic_t,
        partition: int32_t,
        msgflags: libc::c_int,
        payload: *mut libc::c_void,
        len: size_t,
        key: *const libc::c_void,
        keylen: size_t,
        msg_opaque: *mut libc::c_void,
    ) -> libc::c_int;
    fn rd_kafka_producev(rk: *mut rd_kafka_t, _: ...) -> rd_kafka_resp_err_t;
    fn zim_RdKafka___construct(
        execute_data: *mut zend_execute_data,
        return_value: *mut zval,
    );
    static mut ce_kafka_exception: *mut zend_class_entry;
    fn get_kafka_object(zrk: *mut zval) -> *mut kafka_object;
    fn add_consuming_toppar(
        intern: *mut kafka_object,
        rkt: *mut rd_kafka_topic_t,
        partition: int32_t,
    );
    fn del_consuming_toppar(
        intern: *mut kafka_object,
        rkt: *mut rd_kafka_topic_t,
        partition: int32_t,
    );
    fn is_consuming_toppar(
        intern: *mut kafka_object,
        rkt: *mut rd_kafka_topic_t,
        partition: int32_t,
    ) -> libc::c_int;
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
    fn get_kafka_queue_object(zrkqu: *mut zval) -> *mut kafka_queue_object;
    static mut ce_kafka_queue: *mut zend_class_entry;
    fn kafka_message_new(
        return_value: *mut zval,
        message: *const rd_kafka_message_t,
        msg_opaque: *mut zend_string,
    );
    fn kafka_message_list_to_array(
        return_value: *mut zval,
        messages: *mut *mut rd_kafka_message_t,
        size: libc::c_long,
    );
}
pub type size_t = libc::c_ulong;
pub type __uint8_t = libc::c_uchar;
pub type __uint16_t = libc::c_ushort;
pub type __int32_t = libc::c_int;
pub type __uint32_t = libc::c_uint;
pub type __int64_t = libc::c_long;
pub type __uint64_t = libc::c_ulong;
pub type __ssize_t = libc::c_long;
pub type ssize_t = __ssize_t;
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
pub type HashPosition = uint32_t;
pub type zend_string_init_interned_func_t = Option::<
    unsafe extern "C" fn(*const libc::c_char, size_t, libc::c_int) -> *mut zend_string,
>;
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
pub type _zend_expected_type = libc::c_uint;
pub const Z_EXPECTED_LAST: _zend_expected_type = 9;
pub const Z_EXPECTED_DOUBLE: _zend_expected_type = 8;
pub const Z_EXPECTED_OBJECT: _zend_expected_type = 7;
pub const Z_EXPECTED_PATH: _zend_expected_type = 6;
pub const Z_EXPECTED_RESOURCE: _zend_expected_type = 5;
pub const Z_EXPECTED_FUNC: _zend_expected_type = 4;
pub const Z_EXPECTED_ARRAY: _zend_expected_type = 3;
pub const Z_EXPECTED_STRING: _zend_expected_type = 2;
pub const Z_EXPECTED_BOOL: _zend_expected_type = 1;
pub const Z_EXPECTED_LONG: _zend_expected_type = 0;
pub type zend_expected_type = _zend_expected_type;
pub type rd_kafka_type_t = libc::c_uint;
pub const RD_KAFKA_CONSUMER: rd_kafka_type_t = 1;
pub const RD_KAFKA_PRODUCER: rd_kafka_type_t = 0;
pub type rd_kafka_t = rd_kafka_s;
pub type rd_kafka_topic_t = rd_kafka_topic_s;
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
pub type rd_kafka_vtype_t = libc::c_uint;
pub const RD_KAFKA_VTYPE_HEADERS: rd_kafka_vtype_t = 10;
pub const RD_KAFKA_VTYPE_HEADER: rd_kafka_vtype_t = 9;
pub const RD_KAFKA_VTYPE_TIMESTAMP: rd_kafka_vtype_t = 8;
pub const RD_KAFKA_VTYPE_MSGFLAGS: rd_kafka_vtype_t = 7;
pub const RD_KAFKA_VTYPE_OPAQUE: rd_kafka_vtype_t = 6;
pub const RD_KAFKA_VTYPE_KEY: rd_kafka_vtype_t = 5;
pub const RD_KAFKA_VTYPE_VALUE: rd_kafka_vtype_t = 4;
pub const RD_KAFKA_VTYPE_PARTITION: rd_kafka_vtype_t = 3;
pub const RD_KAFKA_VTYPE_RKT: rd_kafka_vtype_t = 2;
pub const RD_KAFKA_VTYPE_TOPIC: rd_kafka_vtype_t = 1;
pub const RD_KAFKA_VTYPE_END: rd_kafka_vtype_t = 0;
pub type rd_kafka_headers_t = rd_kafka_headers_s;
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _kafka_topic_object {
    pub rkt: *mut rd_kafka_topic_t,
    pub zrk: zval,
    pub std: zend_object,
}
pub type kafka_topic_object = _kafka_topic_object;
pub type php_callback = _php_callback;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _php_callback {
    pub fci: zend_fcall_info,
    pub fcc: zend_fcall_info_cache,
}
pub type kafka_queue_object = _kafka_queue_object;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _kafka_queue_object {
    pub rkqu: *mut rd_kafka_queue_t,
    pub zrk: zval,
    pub std: zend_object,
}
#[inline(always)]
unsafe extern "C" fn zend_string_addref(mut s: *mut zend_string) -> uint32_t {
    if zval_gc_flags((*s).gc.u.type_info)
        & ((1 as libc::c_int) << 6 as libc::c_int) as libc::c_uint == 0
    {
        return zend_gc_addref(&mut (*s).gc);
    }
    return 1 as libc::c_int as uint32_t;
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
unsafe extern "C" fn zval_get_type(mut pz: *const zval) -> zend_uchar {
    return (*pz).u1.v.type_0;
}
#[inline(always)]
unsafe extern "C" fn zval_gc_flags(mut gc_type_info: uint32_t) -> uint32_t {
    return gc_type_info >> 0 as libc::c_int
        & (0x3f0 as libc::c_int >> 0 as libc::c_int) as libc::c_uint;
}
#[inline(always)]
unsafe extern "C" fn zend_gc_refcount(mut p: *const zend_refcounted_h) -> uint32_t {
    return (*p).refcount;
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
unsafe extern "C" fn zval_refcount_p(mut pz: *const zval) -> uint32_t {
    return zend_gc_refcount(&mut (*(*pz).value.counted).gc);
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
unsafe extern "C" fn zval_delref_p(mut pz: *mut zval) -> uint32_t {
    if !((*pz).u1.v.type_flags as libc::c_int != 0 as libc::c_int) as libc::c_int
        as libc::c_long != 0
    {
        unreachable!();
    }
    return zend_gc_delref(&mut (*(*pz).value.counted).gc);
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
#[inline(always)]
unsafe extern "C" fn zend_parse_arg_long(
    mut arg: *mut zval,
    mut dest: *mut zend_long,
    mut is_null: *mut zend_bool,
    mut check_null: libc::c_int,
    mut cap: libc::c_int,
) -> libc::c_int {
    if check_null != 0 {
        *is_null = 0 as libc::c_int as zend_bool;
    }
    if (zval_get_type(arg) as libc::c_int == 4 as libc::c_int) as libc::c_int
        as libc::c_long != 0
    {
        *dest = (*arg).value.lval;
    } else if check_null != 0 && zval_get_type(arg) as libc::c_int == 1 as libc::c_int {
        *is_null = 1 as libc::c_int as zend_bool;
        *dest = 0 as libc::c_int as zend_long;
    } else if cap != 0 {
        return zend_parse_arg_long_cap_slow(arg, dest)
    } else {
        return zend_parse_arg_long_slow(arg, dest)
    }
    return 1 as libc::c_int;
}
#[inline(always)]
unsafe extern "C" fn zend_parse_arg_str(
    mut arg: *mut zval,
    mut dest: *mut *mut zend_string,
    mut check_null: libc::c_int,
) -> libc::c_int {
    if (zval_get_type(arg) as libc::c_int == 6 as libc::c_int) as libc::c_int
        as libc::c_long != 0
    {
        *dest = (*arg).value.str_0;
    } else if check_null != 0 && zval_get_type(arg) as libc::c_int == 1 as libc::c_int {
        *dest = 0 as *mut zend_string;
    } else {
        return zend_parse_arg_str_slow(arg, dest)
    }
    return 1 as libc::c_int;
}
#[inline(always)]
unsafe extern "C" fn zend_parse_arg_string(
    mut arg: *mut zval,
    mut dest: *mut *mut libc::c_char,
    mut dest_len: *mut size_t,
    mut check_null: libc::c_int,
) -> libc::c_int {
    let mut str: *mut zend_string = 0 as *mut zend_string;
    if zend_parse_arg_str(arg, &mut str, check_null) == 0 {
        return 0 as libc::c_int;
    }
    if check_null != 0 && str.is_null() as libc::c_int as libc::c_long != 0 {
        *dest = 0 as *mut libc::c_char;
        *dest_len = 0 as libc::c_int as size_t;
    } else {
        *dest = ((*str).val).as_mut_ptr();
        *dest_len = (*str).len;
    }
    return 1 as libc::c_int;
}
#[inline(always)]
unsafe extern "C" fn zend_parse_arg_array_ht(
    mut arg: *mut zval,
    mut dest: *mut *mut HashTable,
    mut check_null: libc::c_int,
    mut or_object: libc::c_int,
    mut separate: libc::c_int,
) -> libc::c_int {
    if (zval_get_type(arg) as libc::c_int == 7 as libc::c_int) as libc::c_int
        as libc::c_long != 0
    {
        *dest = (*arg).value.arr;
    } else if or_object != 0
        && (zval_get_type(arg) as libc::c_int == 8 as libc::c_int) as libc::c_int
            as libc::c_long != 0
    {
        if separate != 0 && !((*(*arg).value.obj).properties).is_null()
            && (zend_gc_refcount(&mut (*(*(*arg).value.obj).properties).gc)
                > 1 as libc::c_int as libc::c_uint) as libc::c_int as libc::c_long != 0
        {
            if (zval_gc_flags((*(*(*arg).value.obj).properties).gc.u.type_info)
                & ((1 as libc::c_int) << 6 as libc::c_int) as libc::c_uint == 0)
                as libc::c_int as libc::c_long != 0
            {
                zend_gc_delref(&mut (*(*(*arg).value.obj).properties).gc);
            }
            (*(*arg).value.obj)
                .properties = zend_array_dup((*(*arg).value.obj).properties);
        }
        *dest = ((*(*(*arg).value.obj).handlers).get_properties).unwrap()(arg);
    } else if check_null != 0
        && (zval_get_type(arg) as libc::c_int == 1 as libc::c_int) as libc::c_int
            as libc::c_long != 0
    {
        *dest = 0 as *mut HashTable;
    } else {
        return 0 as libc::c_int
    }
    return 1 as libc::c_int;
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
#[inline]
unsafe extern "C" fn rdkafka_hash_get_current_key_ex(
    mut ht: *mut HashTable,
    mut pos: *mut HashPosition,
) -> *mut libc::c_char {
    let mut key: *mut zend_string = 0 as *mut zend_string;
    let mut index: zend_ulong = 0;
    if zend_hash_get_current_key_ex(ht, &mut key, &mut index, pos) == 1 as libc::c_int {
        return ((*key).val).as_mut_ptr();
    }
    return 0 as *mut libc::c_char;
}
static mut arginfo_class_RdKafka_Topic_getName: [zend_internal_arg_info; 1] = [
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
static mut arginfo_class_RdKafka_ConsumerTopic___construct: [zend_internal_arg_info; 1] = [
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
static mut arginfo_class_RdKafka_ConsumerTopic_consumeQueueStart: [zend_internal_arg_info; 4] = unsafe {
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
                name: b"partition\0" as *const u8 as *const libc::c_char,
                type_0: 0 as libc::c_int as zend_type,
                pass_by_reference: 0 as libc::c_int as zend_uchar,
                is_variadic: 0 as libc::c_int as zend_bool,
            };
            init
        },
        {
            let mut init = _zend_internal_arg_info {
                name: b"offset\0" as *const u8 as *const libc::c_char,
                type_0: 0 as libc::c_int as zend_type,
                pass_by_reference: 0 as libc::c_int as zend_uchar,
                is_variadic: 0 as libc::c_int as zend_bool,
            };
            init
        },
        {
            let mut init = _zend_internal_arg_info {
                name: b"queue\0" as *const u8 as *const libc::c_char,
                type_0: 0 as libc::c_int as zend_type,
                pass_by_reference: 0 as libc::c_int as zend_uchar,
                is_variadic: 0 as libc::c_int as zend_bool,
            };
            init
        },
    ]
};
static mut arginfo_class_RdKafka_ConsumerTopic_consumeCallback: [zend_internal_arg_info; 4] = unsafe {
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
                name: b"partition\0" as *const u8 as *const libc::c_char,
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
static mut arginfo_class_RdKafka_ConsumerTopic_consumeStart: [zend_internal_arg_info; 3] = unsafe {
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
                name: b"partition\0" as *const u8 as *const libc::c_char,
                type_0: 0 as libc::c_int as zend_type,
                pass_by_reference: 0 as libc::c_int as zend_uchar,
                is_variadic: 0 as libc::c_int as zend_bool,
            };
            init
        },
        {
            let mut init = _zend_internal_arg_info {
                name: b"offset\0" as *const u8 as *const libc::c_char,
                type_0: 0 as libc::c_int as zend_type,
                pass_by_reference: 0 as libc::c_int as zend_uchar,
                is_variadic: 0 as libc::c_int as zend_bool,
            };
            init
        },
    ]
};
static mut arginfo_class_RdKafka_ConsumerTopic_consumeStop: [zend_internal_arg_info; 2] = unsafe {
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
                name: b"partition\0" as *const u8 as *const libc::c_char,
                type_0: 0 as libc::c_int as zend_type,
                pass_by_reference: 0 as libc::c_int as zend_uchar,
                is_variadic: 0 as libc::c_int as zend_bool,
            };
            init
        },
    ]
};
static mut arginfo_class_RdKafka_ConsumerTopic_consume: [zend_internal_arg_info; 3] = unsafe {
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
                name: b"partition\0" as *const u8 as *const libc::c_char,
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
static mut arginfo_class_RdKafka_ConsumerTopic_consumeBatch: [zend_internal_arg_info; 4] = unsafe {
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
                name: b"partition\0" as *const u8 as *const libc::c_char,
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
        {
            let mut init = _zend_internal_arg_info {
                name: b"batch_size\0" as *const u8 as *const libc::c_char,
                type_0: 0 as libc::c_int as zend_type,
                pass_by_reference: 0 as libc::c_int as zend_uchar,
                is_variadic: 0 as libc::c_int as zend_bool,
            };
            init
        },
    ]
};
static mut arginfo_class_RdKafka_ProducerTopic_produce: [zend_internal_arg_info; 6] = unsafe {
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
                name: b"partition\0" as *const u8 as *const libc::c_char,
                type_0: 0 as libc::c_int as zend_type,
                pass_by_reference: 0 as libc::c_int as zend_uchar,
                is_variadic: 0 as libc::c_int as zend_bool,
            };
            init
        },
        {
            let mut init = _zend_internal_arg_info {
                name: b"msgflags\0" as *const u8 as *const libc::c_char,
                type_0: 0 as libc::c_int as zend_type,
                pass_by_reference: 0 as libc::c_int as zend_uchar,
                is_variadic: 0 as libc::c_int as zend_bool,
            };
            init
        },
        {
            let mut init = _zend_internal_arg_info {
                name: b"payload\0" as *const u8 as *const libc::c_char,
                type_0: 0 as libc::c_int as zend_type,
                pass_by_reference: 0 as libc::c_int as zend_uchar,
                is_variadic: 0 as libc::c_int as zend_bool,
            };
            init
        },
        {
            let mut init = _zend_internal_arg_info {
                name: b"key\0" as *const u8 as *const libc::c_char,
                type_0: 0 as libc::c_int as zend_type,
                pass_by_reference: 0 as libc::c_int as zend_uchar,
                is_variadic: 0 as libc::c_int as zend_bool,
            };
            init
        },
        {
            let mut init = _zend_internal_arg_info {
                name: b"msg_opaque\0" as *const u8 as *const libc::c_char,
                type_0: 0 as libc::c_int as zend_type,
                pass_by_reference: 0 as libc::c_int as zend_uchar,
                is_variadic: 0 as libc::c_int as zend_bool,
            };
            init
        },
    ]
};
static mut arginfo_class_RdKafka_ProducerTopic_producev: [zend_internal_arg_info; 8] = unsafe {
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
                name: b"partition\0" as *const u8 as *const libc::c_char,
                type_0: 0 as libc::c_int as zend_type,
                pass_by_reference: 0 as libc::c_int as zend_uchar,
                is_variadic: 0 as libc::c_int as zend_bool,
            };
            init
        },
        {
            let mut init = _zend_internal_arg_info {
                name: b"msgflags\0" as *const u8 as *const libc::c_char,
                type_0: 0 as libc::c_int as zend_type,
                pass_by_reference: 0 as libc::c_int as zend_uchar,
                is_variadic: 0 as libc::c_int as zend_bool,
            };
            init
        },
        {
            let mut init = _zend_internal_arg_info {
                name: b"payload\0" as *const u8 as *const libc::c_char,
                type_0: 0 as libc::c_int as zend_type,
                pass_by_reference: 0 as libc::c_int as zend_uchar,
                is_variadic: 0 as libc::c_int as zend_bool,
            };
            init
        },
        {
            let mut init = _zend_internal_arg_info {
                name: b"key\0" as *const u8 as *const libc::c_char,
                type_0: 0 as libc::c_int as zend_type,
                pass_by_reference: 0 as libc::c_int as zend_uchar,
                is_variadic: 0 as libc::c_int as zend_bool,
            };
            init
        },
        {
            let mut init = _zend_internal_arg_info {
                name: b"headers\0" as *const u8 as *const libc::c_char,
                type_0: 0 as libc::c_int as zend_type,
                pass_by_reference: 0 as libc::c_int as zend_uchar,
                is_variadic: 0 as libc::c_int as zend_bool,
            };
            init
        },
        {
            let mut init = _zend_internal_arg_info {
                name: b"timestamp_ms\0" as *const u8 as *const libc::c_char,
                type_0: 0 as libc::c_int as zend_type,
                pass_by_reference: 0 as libc::c_int as zend_uchar,
                is_variadic: 0 as libc::c_int as zend_bool,
            };
            init
        },
        {
            let mut init = _zend_internal_arg_info {
                name: b"msg_opaque\0" as *const u8 as *const libc::c_char,
                type_0: 0 as libc::c_int as zend_type,
                pass_by_reference: 0 as libc::c_int as zend_uchar,
                is_variadic: 0 as libc::c_int as zend_bool,
            };
            init
        },
    ]
};
static mut class_RdKafka_Topic_methods: [zend_function_entry; 2] = [zend_function_entry {
    fname: 0 as *const libc::c_char,
    handler: None,
    arg_info: 0 as *const _zend_internal_arg_info,
    num_args: 0,
    flags: 0,
}; 2];
static mut class_RdKafka_ConsumerTopic_methods: [zend_function_entry; 9] = [zend_function_entry {
    fname: 0 as *const libc::c_char,
    handler: None,
    arg_info: 0 as *const _zend_internal_arg_info,
    num_args: 0,
    flags: 0,
}; 9];
static mut class_RdKafka_KafkaConsumerTopic_methods: [zend_function_entry; 3] = [zend_function_entry {
    fname: 0 as *const libc::c_char,
    handler: None,
    arg_info: 0 as *const _zend_internal_arg_info,
    num_args: 0,
    flags: 0,
}; 3];
static mut class_RdKafka_ProducerTopic_methods: [zend_function_entry; 4] = [zend_function_entry {
    fname: 0 as *const libc::c_char,
    handler: None,
    arg_info: 0 as *const _zend_internal_arg_info,
    num_args: 0,
    flags: 0,
}; 4];
unsafe extern "C" fn register_class_RdKafka_Topic() -> *mut zend_class_entry {
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
        b"RdKafka\\Topic\0" as *const u8 as *const libc::c_char,
        (::std::mem::size_of::<[libc::c_char; 14]>() as libc::c_ulong)
            .wrapping_sub(1 as libc::c_int as libc::c_ulong),
        1 as libc::c_int,
    );
    ce.info.internal.builtin_functions = class_RdKafka_Topic_methods.as_ptr();
    class_entry = zend_register_internal_class_ex(&mut ce, 0 as *mut zend_class_entry);
    (*class_entry).ce_flags |= ((1 as libc::c_int) << 6 as libc::c_int) as libc::c_uint;
    return class_entry;
}
unsafe extern "C" fn register_class_RdKafka_ConsumerTopic(
    mut class_entry_RdKafka_Topic: *mut zend_class_entry,
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
        b"RdKafka\\ConsumerTopic\0" as *const u8 as *const libc::c_char,
        (::std::mem::size_of::<[libc::c_char; 22]>() as libc::c_ulong)
            .wrapping_sub(1 as libc::c_int as libc::c_ulong),
        1 as libc::c_int,
    );
    ce.info.internal.builtin_functions = class_RdKafka_ConsumerTopic_methods.as_ptr();
    class_entry = zend_register_internal_class_ex(&mut ce, class_entry_RdKafka_Topic);
    return class_entry;
}
unsafe extern "C" fn register_class_RdKafka_KafkaConsumerTopic(
    mut class_entry_RdKafka_Topic: *mut zend_class_entry,
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
        b"RdKafka\\KafkaConsumerTopic\0" as *const u8 as *const libc::c_char,
        (::std::mem::size_of::<[libc::c_char; 27]>() as libc::c_ulong)
            .wrapping_sub(1 as libc::c_int as libc::c_ulong),
        1 as libc::c_int,
    );
    ce
        .info
        .internal
        .builtin_functions = class_RdKafka_KafkaConsumerTopic_methods.as_ptr();
    class_entry = zend_register_internal_class_ex(&mut ce, class_entry_RdKafka_Topic);
    return class_entry;
}
unsafe extern "C" fn register_class_RdKafka_ProducerTopic(
    mut class_entry_RdKafka_Topic: *mut zend_class_entry,
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
        b"RdKafka\\ProducerTopic\0" as *const u8 as *const libc::c_char,
        (::std::mem::size_of::<[libc::c_char; 22]>() as libc::c_ulong)
            .wrapping_sub(1 as libc::c_int as libc::c_ulong),
        1 as libc::c_int,
    );
    ce.info.internal.builtin_functions = class_RdKafka_ProducerTopic_methods.as_ptr();
    class_entry = zend_register_internal_class_ex(&mut ce, class_entry_RdKafka_Topic);
    return class_entry;
}
static mut object_handlers: zend_object_handlers = zend_object_handlers {
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
pub static mut ce_kafka_consumer_topic: *mut zend_class_entry = 0
    as *const zend_class_entry as *mut zend_class_entry;
pub static mut ce_kafka_kafka_consumer_topic: *mut zend_class_entry = 0
    as *const zend_class_entry as *mut zend_class_entry;
pub static mut ce_kafka_producer_topic: *mut zend_class_entry = 0
    as *const zend_class_entry as *mut zend_class_entry;
pub static mut ce_kafka_topic: *mut zend_class_entry = 0 as *const zend_class_entry
    as *mut zend_class_entry;
unsafe extern "C" fn kafka_topic_free(mut object: *mut zend_object) {
    let mut intern: *mut kafka_topic_object = (object as *mut libc::c_char)
        .offset(-(24 as libc::c_ulong as isize)) as *mut kafka_topic_object;
    if zval_get_type(&mut (*intern).zrk) as libc::c_int != 0 as libc::c_int
        && !((*intern).rkt).is_null()
    {
        let mut kafka_intern: *mut kafka_object = get_kafka_object(&mut (*intern).zrk);
        if !kafka_intern.is_null() {
            zend_hash_index_del(&mut (*kafka_intern).topics, intern as zend_ulong);
        }
    }
    zend_object_std_dtor(&mut (*intern).std);
}
unsafe extern "C" fn kafka_topic_new(
    mut class_type: *mut zend_class_entry,
) -> *mut zend_object {
    let mut retval: *mut zend_object = 0 as *mut zend_object;
    let mut intern: *mut kafka_topic_object = 0 as *mut kafka_topic_object;
    intern = zend_object_alloc(
        ::std::mem::size_of::<kafka_topic_object>() as libc::c_ulong,
        class_type,
    ) as *mut kafka_topic_object;
    zend_object_std_init(&mut (*intern).std, class_type);
    object_properties_init(&mut (*intern).std, class_type);
    retval = &mut (*intern).std;
    (*retval).handlers = &mut object_handlers;
    return retval;
}
unsafe extern "C" fn consume_callback(
    mut msg: *mut rd_kafka_message_t,
    mut opaque: *mut libc::c_void,
) {
    let mut cb: *mut php_callback = opaque as *mut php_callback;
    let mut args: [zval; 1] = [zval {
        value: _zend_value { lval: 0 },
        u1: C2RustUnnamed_1 {
            v: C2RustUnnamed_2 {
                type_0: 0,
                type_flags: 0,
                u: C2RustUnnamed_3 { extra: 0 },
            },
        },
        u2: C2RustUnnamed_0 { next: 0 },
    }; 1];
    if opaque.is_null() {
        return;
    }
    if cb.is_null() {
        return;
    }
    args[0 as libc::c_int as usize].u1.type_info = 1 as libc::c_int as uint32_t;
    kafka_message_new(
        &mut *args.as_mut_ptr().offset(0 as libc::c_int as isize),
        msg,
        0 as *mut zend_string,
    );
    rdkafka_call_function(
        &mut (*cb).fci,
        &mut (*cb).fcc,
        0 as *mut zval,
        1 as libc::c_int as uint32_t,
        args.as_mut_ptr(),
    );
    zval_ptr_dtor(&mut *args.as_mut_ptr().offset(0 as libc::c_int as isize));
}
pub unsafe extern "C" fn get_kafka_topic_object(
    mut zrkt: *mut zval,
) -> *mut kafka_topic_object {
    let mut orkt: *mut kafka_topic_object = ((*zrkt).value.obj as *mut libc::c_char)
        .offset(-(24 as libc::c_ulong as isize)) as *mut kafka_topic_object;
    if ((*orkt).rkt).is_null() {
        zend_throw_exception_ex(
            0 as *mut zend_class_entry,
            0 as libc::c_int as zend_long,
            b"RdKafka\\Topic::__construct() has not been called\0" as *const u8
                as *const libc::c_char,
        );
        return 0 as *mut kafka_topic_object;
    }
    return orkt;
}
pub unsafe extern "C" fn zim_RdKafka_ConsumerTopic_consumeCallback(
    mut execute_data: *mut zend_execute_data,
    mut return_value: *mut zval,
) {
    let mut cb: php_callback = php_callback {
        fci: zend_fcall_info {
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
        },
        fcc: zend_fcall_info_cache {
            function_handler: 0 as *mut zend_function,
            calling_scope: 0 as *mut zend_class_entry,
            called_scope: 0 as *mut zend_class_entry,
            object: 0 as *mut zend_object,
        },
    };
    let mut partition: zend_long = 0;
    let mut timeout_ms: zend_long = 0;
    let mut result: libc::c_long = 0;
    let mut intern: *mut kafka_topic_object = 0 as *mut kafka_topic_object;
    if zend_parse_parameters(
        (*execute_data).This.u2.num_args as libc::c_int,
        b"llf\0" as *const u8 as *const libc::c_char,
        &mut partition as *mut zend_long,
        &mut timeout_ms as *mut zend_long,
        &mut cb.fci as *mut zend_fcall_info,
        &mut cb.fcc as *mut zend_fcall_info_cache,
    ) == FAILURE as libc::c_int
    {
        return;
    }
    if partition < 0 as libc::c_int as libc::c_long
        || partition > 0x7fffffff as libc::c_int as libc::c_long
    {
        zend_throw_exception_ex(
            spl_ce_InvalidArgumentException,
            0 as libc::c_int as zend_long,
            b"Out of range value '%ld' for $partition\0" as *const u8
                as *const libc::c_char,
            partition,
        );
        return;
    }
    intern = get_kafka_topic_object(
        if zval_get_type(&mut (*execute_data).This) as libc::c_int == 8 as libc::c_int {
            &mut (*execute_data).This
        } else {
            0 as *mut zval
        },
    );
    if intern.is_null() {
        return;
    }
    zval_addref_p(&mut cb.fci.function_name);
    result = rd_kafka_consume_callback(
        (*intern).rkt,
        partition as int32_t,
        timeout_ms as libc::c_int,
        Some(
            consume_callback
                as unsafe extern "C" fn(*mut rd_kafka_message_t, *mut libc::c_void) -> (),
        ),
        &mut cb as *mut php_callback as *mut libc::c_void,
    ) as libc::c_long;
    zval_ptr_dtor(&mut cb.fci.function_name);
    let mut __z: *mut zval = return_value;
    (*__z).value.lval = result;
    (*__z).u1.type_info = 4 as libc::c_int as uint32_t;
}
pub unsafe extern "C" fn zim_RdKafka_ConsumerTopic_consumeQueueStart(
    mut execute_data: *mut zend_execute_data,
    mut return_value: *mut zval,
) {
    let mut zrkqu: *mut zval = 0 as *mut zval;
    let mut intern: *mut kafka_topic_object = 0 as *mut kafka_topic_object;
    let mut queue_intern: *mut kafka_queue_object = 0 as *mut kafka_queue_object;
    let mut partition: zend_long = 0;
    let mut offset: zend_long = 0;
    let mut ret: libc::c_int = 0;
    let mut err: rd_kafka_resp_err_t = RD_KAFKA_RESP_ERR_NO_ERROR;
    let mut kafka_intern: *mut kafka_object = 0 as *mut kafka_object;
    if zend_parse_parameters(
        (*execute_data).This.u2.num_args as libc::c_int,
        b"llO\0" as *const u8 as *const libc::c_char,
        &mut partition as *mut zend_long,
        &mut offset as *mut zend_long,
        &mut zrkqu as *mut *mut zval,
        ce_kafka_queue,
    ) == FAILURE as libc::c_int
    {
        return;
    }
    if partition != -(1 as libc::c_int) as libc::c_long
        && (partition < 0 as libc::c_int as libc::c_long
            || partition > 0x7fffffff as libc::c_int as libc::c_long)
    {
        zend_throw_exception_ex(
            spl_ce_InvalidArgumentException,
            0 as libc::c_int as zend_long,
            b"Out of range value '%ld' for $partition\0" as *const u8
                as *const libc::c_char,
            partition,
        );
        return;
    }
    intern = get_kafka_topic_object(
        if zval_get_type(&mut (*execute_data).This) as libc::c_int == 8 as libc::c_int {
            &mut (*execute_data).This
        } else {
            0 as *mut zval
        },
    );
    if intern.is_null() {
        return;
    }
    queue_intern = get_kafka_queue_object(zrkqu);
    if queue_intern.is_null() {
        return;
    }
    kafka_intern = get_kafka_object(&mut (*intern).zrk);
    if kafka_intern.is_null() {
        return;
    }
    if is_consuming_toppar(kafka_intern, (*intern).rkt, partition as int32_t) != 0 {
        zend_throw_exception_ex(
            ce_kafka_exception,
            0 as libc::c_int as zend_long,
            b"%s:%ld is already being consumed by the same Consumer instance\0"
                as *const u8 as *const libc::c_char,
            rd_kafka_topic_name((*intern).rkt),
            partition,
        );
        return;
    }
    ret = rd_kafka_consume_start_queue(
        (*intern).rkt,
        partition as int32_t,
        offset,
        (*queue_intern).rkqu,
    );
    if ret == -(1 as libc::c_int) {
        err = rd_kafka_last_error();
        zend_throw_exception(
            ce_kafka_exception,
            rd_kafka_err2str(err),
            err as zend_long,
        );
        return;
    }
    add_consuming_toppar(kafka_intern, (*intern).rkt, partition as int32_t);
}
pub unsafe extern "C" fn zim_RdKafka_ConsumerTopic_consumeStart(
    mut execute_data: *mut zend_execute_data,
    mut return_value: *mut zval,
) {
    let mut intern: *mut kafka_topic_object = 0 as *mut kafka_topic_object;
    let mut partition: zend_long = 0;
    let mut offset: zend_long = 0;
    let mut ret: libc::c_int = 0;
    let mut err: rd_kafka_resp_err_t = RD_KAFKA_RESP_ERR_NO_ERROR;
    let mut kafka_intern: *mut kafka_object = 0 as *mut kafka_object;
    if zend_parse_parameters(
        (*execute_data).This.u2.num_args as libc::c_int,
        b"ll\0" as *const u8 as *const libc::c_char,
        &mut partition as *mut zend_long,
        &mut offset as *mut zend_long,
    ) == FAILURE as libc::c_int
    {
        return;
    }
    if partition != -(1 as libc::c_int) as libc::c_long
        && (partition < 0 as libc::c_int as libc::c_long
            || partition > 0x7fffffff as libc::c_int as libc::c_long)
    {
        zend_throw_exception_ex(
            spl_ce_InvalidArgumentException,
            0 as libc::c_int as zend_long,
            b"Out of range value '%ld' for $partition\0" as *const u8
                as *const libc::c_char,
            partition,
        );
        return;
    }
    intern = get_kafka_topic_object(
        if zval_get_type(&mut (*execute_data).This) as libc::c_int == 8 as libc::c_int {
            &mut (*execute_data).This
        } else {
            0 as *mut zval
        },
    );
    if intern.is_null() {
        return;
    }
    kafka_intern = get_kafka_object(&mut (*intern).zrk);
    if kafka_intern.is_null() {
        return;
    }
    if is_consuming_toppar(kafka_intern, (*intern).rkt, partition as int32_t) != 0 {
        zend_throw_exception_ex(
            ce_kafka_exception,
            0 as libc::c_int as zend_long,
            b"%s:%ld is already being consumed by the same Consumer instance\0"
                as *const u8 as *const libc::c_char,
            rd_kafka_topic_name((*intern).rkt),
            partition,
        );
        return;
    }
    ret = rd_kafka_consume_start((*intern).rkt, partition as int32_t, offset);
    if ret == -(1 as libc::c_int) {
        err = rd_kafka_last_error();
        zend_throw_exception(
            ce_kafka_exception,
            rd_kafka_err2str(err),
            err as zend_long,
        );
        return;
    }
    add_consuming_toppar(kafka_intern, (*intern).rkt, partition as int32_t);
}
pub unsafe extern "C" fn zim_RdKafka_ConsumerTopic_consumeStop(
    mut execute_data: *mut zend_execute_data,
    mut return_value: *mut zval,
) {
    let mut intern: *mut kafka_topic_object = 0 as *mut kafka_topic_object;
    let mut partition: zend_long = 0;
    let mut ret: libc::c_int = 0;
    let mut err: rd_kafka_resp_err_t = RD_KAFKA_RESP_ERR_NO_ERROR;
    let mut kafka_intern: *mut kafka_object = 0 as *mut kafka_object;
    if zend_parse_parameters(
        (*execute_data).This.u2.num_args as libc::c_int,
        b"l\0" as *const u8 as *const libc::c_char,
        &mut partition as *mut zend_long,
    ) == FAILURE as libc::c_int
    {
        return;
    }
    if partition != -(1 as libc::c_int) as libc::c_long
        && (partition < 0 as libc::c_int as libc::c_long
            || partition > 0x7fffffff as libc::c_int as libc::c_long)
    {
        zend_throw_exception_ex(
            spl_ce_InvalidArgumentException,
            0 as libc::c_int as zend_long,
            b"Out of range value '%ld' for $partition\0" as *const u8
                as *const libc::c_char,
            partition,
        );
        return;
    }
    intern = get_kafka_topic_object(
        if zval_get_type(&mut (*execute_data).This) as libc::c_int == 8 as libc::c_int {
            &mut (*execute_data).This
        } else {
            0 as *mut zval
        },
    );
    if intern.is_null() {
        return;
    }
    kafka_intern = get_kafka_object(&mut (*intern).zrk);
    if kafka_intern.is_null() {
        return;
    }
    ret = rd_kafka_consume_stop((*intern).rkt, partition as int32_t);
    if ret == -(1 as libc::c_int) {
        err = rd_kafka_last_error();
        zend_throw_exception(
            ce_kafka_exception,
            rd_kafka_err2str(err),
            err as zend_long,
        );
        return;
    }
    del_consuming_toppar(kafka_intern, (*intern).rkt, partition as int32_t);
}
pub unsafe extern "C" fn zim_RdKafka_ConsumerTopic_consume(
    mut execute_data: *mut zend_execute_data,
    mut return_value: *mut zval,
) {
    let mut intern: *mut kafka_topic_object = 0 as *mut kafka_topic_object;
    let mut partition: zend_long = 0;
    let mut timeout_ms: zend_long = 0;
    let mut message: *mut rd_kafka_message_t = 0 as *mut rd_kafka_message_t;
    let mut err: rd_kafka_resp_err_t = RD_KAFKA_RESP_ERR_NO_ERROR;
    if zend_parse_parameters(
        (*execute_data).This.u2.num_args as libc::c_int,
        b"ll\0" as *const u8 as *const libc::c_char,
        &mut partition as *mut zend_long,
        &mut timeout_ms as *mut zend_long,
    ) == FAILURE as libc::c_int
    {
        return;
    }
    if partition != -(1 as libc::c_int) as libc::c_long
        && (partition < 0 as libc::c_int as libc::c_long
            || partition > 0x7fffffff as libc::c_int as libc::c_long)
    {
        zend_throw_exception_ex(
            spl_ce_InvalidArgumentException,
            0 as libc::c_int as zend_long,
            b"Out of range value '%ld' for $partition\0" as *const u8
                as *const libc::c_char,
            partition,
        );
        return;
    }
    intern = get_kafka_topic_object(
        if zval_get_type(&mut (*execute_data).This) as libc::c_int == 8 as libc::c_int {
            &mut (*execute_data).This
        } else {
            0 as *mut zval
        },
    );
    if intern.is_null() {
        return;
    }
    message = rd_kafka_consume(
        (*intern).rkt,
        partition as int32_t,
        timeout_ms as libc::c_int,
    );
    if message.is_null() {
        err = rd_kafka_last_error();
        if err as libc::c_int == RD_KAFKA_RESP_ERR__TIMED_OUT as libc::c_int {
            return;
        }
        zend_throw_exception(
            ce_kafka_exception,
            rd_kafka_err2str(err),
            err as zend_long,
        );
        return;
    }
    kafka_message_new(return_value, message, 0 as *mut zend_string);
    rd_kafka_message_destroy(message);
}
pub unsafe extern "C" fn zim_RdKafka_ConsumerTopic_consumeBatch(
    mut execute_data: *mut zend_execute_data,
    mut return_value: *mut zval,
) {
    let mut intern: *mut kafka_topic_object = 0 as *mut kafka_topic_object;
    let mut partition: zend_long = 0;
    let mut timeout_ms: zend_long = 0;
    let mut batch_size: zend_long = 0;
    let mut result: libc::c_long = 0;
    let mut i: libc::c_long = 0;
    let mut rkmessages: *mut *mut rd_kafka_message_t = 0 as *mut *mut rd_kafka_message_t;
    let mut err: rd_kafka_resp_err_t = RD_KAFKA_RESP_ERR_NO_ERROR;
    if zend_parse_parameters(
        (*execute_data).This.u2.num_args as libc::c_int,
        b"lll\0" as *const u8 as *const libc::c_char,
        &mut partition as *mut zend_long,
        &mut timeout_ms as *mut zend_long,
        &mut batch_size as *mut zend_long,
    ) == FAILURE as libc::c_int
    {
        return;
    }
    if 0 as libc::c_int as libc::c_long >= batch_size {
        zend_throw_exception_ex(
            spl_ce_InvalidArgumentException,
            0 as libc::c_int as zend_long,
            b"Out of range value '%ld' for batch_size\0" as *const u8
                as *const libc::c_char,
            batch_size,
        );
        return;
    }
    if partition != -(1 as libc::c_int) as libc::c_long
        && (partition < 0 as libc::c_int as libc::c_long
            || partition > 0x7fffffff as libc::c_int as libc::c_long)
    {
        zend_throw_exception_ex(
            spl_ce_InvalidArgumentException,
            0 as libc::c_int as zend_long,
            b"Out of range value '%ld' for $partition\0" as *const u8
                as *const libc::c_char,
            partition,
        );
        return;
    }
    intern = get_kafka_topic_object(
        if zval_get_type(&mut (*execute_data).This) as libc::c_int == 8 as libc::c_int {
            &mut (*execute_data).This
        } else {
            0 as *mut zval
        },
    );
    if intern.is_null() {
        return;
    }
    rkmessages = malloc(
        (::std::mem::size_of::<*mut rd_kafka_message_t>() as libc::c_ulong)
            .wrapping_mul(batch_size as libc::c_ulong),
    ) as *mut *mut rd_kafka_message_t;
    result = rd_kafka_consume_batch(
        (*intern).rkt,
        partition as int32_t,
        timeout_ms as libc::c_int,
        rkmessages,
        batch_size as size_t,
    );
    if result == -(1 as libc::c_int) as libc::c_long {
        free(rkmessages as *mut libc::c_void);
        err = rd_kafka_last_error();
        zend_throw_exception(
            ce_kafka_exception,
            rd_kafka_err2str(err),
            err as zend_long,
        );
        return;
    }
    if result >= 0 as libc::c_int as libc::c_long {
        kafka_message_list_to_array(return_value, rkmessages, result);
        i = 0 as libc::c_int as libc::c_long;
        while i < result {
            rd_kafka_message_destroy(*rkmessages.offset(i as isize));
            i += 1;
            i;
        }
    }
    free(rkmessages as *mut libc::c_void);
}
pub unsafe extern "C" fn zim_RdKafka_ConsumerTopic_offsetStore(
    mut execute_data: *mut zend_execute_data,
    mut return_value: *mut zval,
) {
    let mut intern: *mut kafka_topic_object = 0 as *mut kafka_topic_object;
    let mut partition: zend_long = 0;
    let mut offset: zend_long = 0;
    let mut err: rd_kafka_resp_err_t = RD_KAFKA_RESP_ERR_NO_ERROR;
    if zend_parse_parameters(
        (*execute_data).This.u2.num_args as libc::c_int,
        b"ll\0" as *const u8 as *const libc::c_char,
        &mut partition as *mut zend_long,
        &mut offset as *mut zend_long,
    ) == FAILURE as libc::c_int
    {
        return;
    }
    if partition < 0 as libc::c_int as libc::c_long
        || partition > 0x7fffffff as libc::c_int as libc::c_long
    {
        zend_throw_exception_ex(
            spl_ce_InvalidArgumentException,
            0 as libc::c_int as zend_long,
            b"Out of range value '%ld' for $partition\0" as *const u8
                as *const libc::c_char,
            partition,
        );
        return;
    }
    intern = get_kafka_topic_object(
        if zval_get_type(&mut (*execute_data).This) as libc::c_int == 8 as libc::c_int {
            &mut (*execute_data).This
        } else {
            0 as *mut zval
        },
    );
    if intern.is_null() {
        return;
    }
    err = rd_kafka_offset_store((*intern).rkt, partition as int32_t, offset);
    if err as libc::c_int != RD_KAFKA_RESP_ERR_NO_ERROR as libc::c_int {
        zend_throw_exception(
            ce_kafka_exception,
            rd_kafka_err2str(err),
            err as zend_long,
        );
        return;
    }
}
pub unsafe extern "C" fn zim_RdKafka_ProducerTopic_produce(
    mut execute_data: *mut zend_execute_data,
    mut return_value: *mut zval,
) {
    let mut partition: zend_long = 0;
    let mut msgflags: zend_long = 0;
    let mut payload: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut payload_len: size_t = 0 as libc::c_int as size_t;
    let mut key: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut key_len: size_t = 0 as libc::c_int as size_t;
    let mut opaque: *mut zend_string = 0 as *mut zend_string;
    let mut ret: libc::c_int = 0;
    let mut err: rd_kafka_resp_err_t = RD_KAFKA_RESP_ERR_NO_ERROR;
    let mut intern: *mut kafka_topic_object = 0 as *mut kafka_topic_object;
    let _flags: libc::c_int = 0 as libc::c_int;
    let mut _min_num_args: libc::c_int = 2 as libc::c_int;
    let mut _max_num_args: libc::c_int = 5 as libc::c_int;
    let mut _num_args: libc::c_int = (*execute_data).This.u2.num_args as libc::c_int;
    let mut _i: libc::c_int = 0 as libc::c_int;
    let mut _real_arg: *mut zval = 0 as *mut zval;
    let mut _arg: *mut zval = 0 as *mut zval;
    let mut _expected_type: zend_expected_type = Z_EXPECTED_LONG;
    let mut _error: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut _dummy: zend_bool = 0;
    let mut _optional: zend_bool = 0 as libc::c_int as zend_bool;
    let mut _error_code: libc::c_int = 0 as libc::c_int;
    let mut current_block_216: u64;
    if (_num_args < _min_num_args) as libc::c_int as libc::c_long != 0
        || (_num_args > _max_num_args) as libc::c_int as libc::c_long != 0
            && (_max_num_args >= 0 as libc::c_int) as libc::c_int as libc::c_long != 0
    {
        if _flags & (1 as libc::c_int) << 1 as libc::c_int == 0 {
            if _flags & (1 as libc::c_int) << 2 as libc::c_int != 0 {
                zend_wrong_parameters_count_exception(_min_num_args, _max_num_args);
            } else {
                zend_wrong_parameters_count_error(_min_num_args, _max_num_args);
            }
        }
        _error_code = 1 as libc::c_int;
    } else {
        _real_arg = (execute_data as *mut zval)
            .offset(
                (((::std::mem::size_of::<zend_execute_data>() as libc::c_ulong)
                    .wrapping_add(8 as libc::c_int as libc::c_ulong)
                    .wrapping_sub(1 as libc::c_int as libc::c_ulong)
                    & !(8 as libc::c_int - 1 as libc::c_int) as libc::c_ulong)
                    .wrapping_add(
                        (::std::mem::size_of::<zval>() as libc::c_ulong)
                            .wrapping_add(8 as libc::c_int as libc::c_ulong)
                            .wrapping_sub(1 as libc::c_int as libc::c_ulong)
                            & !(8 as libc::c_int - 1 as libc::c_int) as libc::c_ulong,
                    )
                    .wrapping_sub(1 as libc::c_int as libc::c_ulong)
                    .wrapping_div(
                        (::std::mem::size_of::<zval>() as libc::c_ulong)
                            .wrapping_add(8 as libc::c_int as libc::c_ulong)
                            .wrapping_sub(1 as libc::c_int as libc::c_ulong)
                            & !(8 as libc::c_int - 1 as libc::c_int) as libc::c_ulong,
                    ) as libc::c_int + (0 as libc::c_int - 1 as libc::c_int)) as isize,
            );
        _i += 1;
        _i;
        if !(_i <= _min_num_args || _optional as libc::c_int == 1 as libc::c_int)
            as libc::c_int as libc::c_long != 0
        {
            unreachable!();
        }
        if !(_i > _min_num_args || _optional as libc::c_int == 0 as libc::c_int)
            as libc::c_int as libc::c_long != 0
        {
            unreachable!();
        }
        if _optional != 0 {
            if (_i > _num_args) as libc::c_int as libc::c_long != 0 {
                current_block_216 = 16046443856252641925;
            } else {
                current_block_216 = 652864300344834934;
            }
        } else {
            current_block_216 = 652864300344834934;
        }
        match current_block_216 {
            16046443856252641925 => {}
            _ => {
                _real_arg = _real_arg.offset(1);
                _real_arg;
                _arg = _real_arg;
                if (zend_parse_arg_long(
                    _arg,
                    &mut partition,
                    &mut _dummy,
                    0 as libc::c_int,
                    0 as libc::c_int,
                ) == 0) as libc::c_int as libc::c_long != 0
                {
                    _expected_type = Z_EXPECTED_LONG;
                    _error_code = 4 as libc::c_int;
                } else {
                    _i += 1;
                    _i;
                    if !(_i <= _min_num_args
                        || _optional as libc::c_int == 1 as libc::c_int) as libc::c_int
                        as libc::c_long != 0
                    {
                        unreachable!();
                    }
                    if !(_i > _min_num_args
                        || _optional as libc::c_int == 0 as libc::c_int) as libc::c_int
                        as libc::c_long != 0
                    {
                        unreachable!();
                    }
                    if _optional != 0 {
                        if (_i > _num_args) as libc::c_int as libc::c_long != 0 {
                            current_block_216 = 16046443856252641925;
                        } else {
                            current_block_216 = 13484060386966298149;
                        }
                    } else {
                        current_block_216 = 13484060386966298149;
                    }
                    match current_block_216 {
                        16046443856252641925 => {}
                        _ => {
                            _real_arg = _real_arg.offset(1);
                            _real_arg;
                            _arg = _real_arg;
                            if (zend_parse_arg_long(
                                _arg,
                                &mut msgflags,
                                &mut _dummy,
                                0 as libc::c_int,
                                0 as libc::c_int,
                            ) == 0) as libc::c_int as libc::c_long != 0
                            {
                                _expected_type = Z_EXPECTED_LONG;
                                _error_code = 4 as libc::c_int;
                            } else {
                                _optional = 1 as libc::c_int as zend_bool;
                                _i += 1;
                                _i;
                                if !(_i <= _min_num_args
                                    || _optional as libc::c_int == 1 as libc::c_int)
                                    as libc::c_int as libc::c_long != 0
                                {
                                    unreachable!();
                                }
                                if !(_i > _min_num_args
                                    || _optional as libc::c_int == 0 as libc::c_int)
                                    as libc::c_int as libc::c_long != 0
                                {
                                    unreachable!();
                                }
                                if _optional != 0 {
                                    if (_i > _num_args) as libc::c_int as libc::c_long != 0 {
                                        current_block_216 = 16046443856252641925;
                                    } else {
                                        current_block_216 = 7639320476250304355;
                                    }
                                } else {
                                    current_block_216 = 7639320476250304355;
                                }
                                match current_block_216 {
                                    16046443856252641925 => {}
                                    _ => {
                                        _real_arg = _real_arg.offset(1);
                                        _real_arg;
                                        _arg = _real_arg;
                                        if (zend_parse_arg_string(
                                            _arg,
                                            &mut payload,
                                            &mut payload_len,
                                            1 as libc::c_int,
                                        ) == 0) as libc::c_int as libc::c_long != 0
                                        {
                                            _expected_type = Z_EXPECTED_STRING;
                                            _error_code = 4 as libc::c_int;
                                        } else {
                                            _i += 1;
                                            _i;
                                            if !(_i <= _min_num_args
                                                || _optional as libc::c_int == 1 as libc::c_int)
                                                as libc::c_int as libc::c_long != 0
                                            {
                                                unreachable!();
                                            }
                                            if !(_i > _min_num_args
                                                || _optional as libc::c_int == 0 as libc::c_int)
                                                as libc::c_int as libc::c_long != 0
                                            {
                                                unreachable!();
                                            }
                                            if _optional != 0 {
                                                if (_i > _num_args) as libc::c_int as libc::c_long != 0 {
                                                    current_block_216 = 16046443856252641925;
                                                } else {
                                                    current_block_216 = 7627602990488000394;
                                                }
                                            } else {
                                                current_block_216 = 7627602990488000394;
                                            }
                                            match current_block_216 {
                                                16046443856252641925 => {}
                                                _ => {
                                                    _real_arg = _real_arg.offset(1);
                                                    _real_arg;
                                                    _arg = _real_arg;
                                                    if (zend_parse_arg_string(
                                                        _arg,
                                                        &mut key,
                                                        &mut key_len,
                                                        1 as libc::c_int,
                                                    ) == 0) as libc::c_int as libc::c_long != 0
                                                    {
                                                        _expected_type = Z_EXPECTED_STRING;
                                                        _error_code = 4 as libc::c_int;
                                                    } else {
                                                        _i += 1;
                                                        _i;
                                                        if !(_i <= _min_num_args
                                                            || _optional as libc::c_int == 1 as libc::c_int)
                                                            as libc::c_int as libc::c_long != 0
                                                        {
                                                            unreachable!();
                                                        }
                                                        if !(_i > _min_num_args
                                                            || _optional as libc::c_int == 0 as libc::c_int)
                                                            as libc::c_int as libc::c_long != 0
                                                        {
                                                            unreachable!();
                                                        }
                                                        if _optional != 0 {
                                                            if (_i > _num_args) as libc::c_int as libc::c_long != 0 {
                                                                current_block_216 = 16046443856252641925;
                                                            } else {
                                                                current_block_216 = 12387625063048049585;
                                                            }
                                                        } else {
                                                            current_block_216 = 12387625063048049585;
                                                        }
                                                        match current_block_216 {
                                                            16046443856252641925 => {}
                                                            _ => {
                                                                _real_arg = _real_arg.offset(1);
                                                                _real_arg;
                                                                _arg = _real_arg;
                                                                if (zend_parse_arg_str(_arg, &mut opaque, 1 as libc::c_int)
                                                                    == 0) as libc::c_int as libc::c_long != 0
                                                                {
                                                                    _expected_type = Z_EXPECTED_STRING;
                                                                    _error_code = 4 as libc::c_int;
                                                                }
                                                            }
                                                        }
                                                    }
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
    if (_error_code != 0 as libc::c_int) as libc::c_int as libc::c_long != 0 {
        if _flags & (1 as libc::c_int) << 1 as libc::c_int == 0 {
            if _error_code == 2 as libc::c_int {
                if _flags & (1 as libc::c_int) << 2 as libc::c_int != 0 {
                    zend_wrong_callback_exception(_i, _error);
                } else {
                    zend_wrong_callback_error(_i, _error);
                }
            } else if _error_code == 3 as libc::c_int {
                if _flags & (1 as libc::c_int) << 2 as libc::c_int != 0 {
                    zend_wrong_parameter_class_exception(_i, _error, _arg);
                } else {
                    zend_wrong_parameter_class_error(_i, _error, _arg);
                }
            } else if _error_code == 4 as libc::c_int {
                if _flags & (1 as libc::c_int) << 2 as libc::c_int != 0 {
                    zend_wrong_parameter_type_exception(_i, _expected_type, _arg);
                } else {
                    zend_wrong_parameter_type_error(_i, _expected_type, _arg);
                }
            }
        }
        return;
    }
    if partition != -(1 as libc::c_int) as libc::c_long
        && (partition < 0 as libc::c_int as libc::c_long
            || partition > 0x7fffffff as libc::c_int as libc::c_long)
    {
        zend_throw_exception_ex(
            spl_ce_InvalidArgumentException,
            0 as libc::c_int as zend_long,
            b"Out of range value '%ld' for $partition\0" as *const u8
                as *const libc::c_char,
            partition,
        );
        return;
    }
    if msgflags != 0 as libc::c_int as libc::c_long
        && msgflags != 0x4 as libc::c_int as libc::c_long
    {
        zend_throw_exception_ex(
            spl_ce_InvalidArgumentException,
            0 as libc::c_int as zend_long,
            b"Invalid value '%ld' for $msgflags\0" as *const u8 as *const libc::c_char,
            msgflags,
        );
        return;
    }
    intern = get_kafka_topic_object(
        if zval_get_type(&mut (*execute_data).This) as libc::c_int == 8 as libc::c_int {
            &mut (*execute_data).This
        } else {
            0 as *mut zval
        },
    );
    if !opaque.is_null() {
        zend_string_addref(opaque);
    }
    ret = rd_kafka_produce(
        (*intern).rkt,
        partition as int32_t,
        (msgflags | 0x2 as libc::c_int as libc::c_long) as libc::c_int,
        payload as *mut libc::c_void,
        payload_len,
        key as *const libc::c_void,
        key_len,
        opaque as *mut libc::c_void,
    );
    if ret == -(1 as libc::c_int) {
        if !opaque.is_null() {
            zend_string_release(opaque);
        }
        err = rd_kafka_last_error();
        zend_throw_exception(
            ce_kafka_exception,
            rd_kafka_err2str(err),
            err as zend_long,
        );
        return;
    }
}
pub unsafe extern "C" fn zim_RdKafka_ProducerTopic_producev(
    mut execute_data: *mut zend_execute_data,
    mut return_value: *mut zval,
) {
    let mut partition: zend_long = 0;
    let mut msgflags: zend_long = 0;
    let mut payload: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut payload_len: size_t = 0 as libc::c_int as size_t;
    let mut key: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut key_len: size_t = 0 as libc::c_int as size_t;
    let mut err: rd_kafka_resp_err_t = RD_KAFKA_RESP_ERR_NO_ERROR;
    let mut intern: *mut kafka_topic_object = 0 as *mut kafka_topic_object;
    let mut kafka_intern: *mut kafka_object = 0 as *mut kafka_object;
    let mut headersParam: *mut HashTable = 0 as *mut HashTable;
    let mut headersParamPos: HashPosition = 0;
    let mut header_key: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut header_value: *mut zval = 0 as *mut zval;
    let mut headers: *mut rd_kafka_headers_t = 0 as *mut rd_kafka_headers_t;
    let mut timestamp_ms: zend_long = 0 as libc::c_int as zend_long;
    let mut timestamp_ms_is_null: zend_bool = 0 as libc::c_int as zend_bool;
    let mut opaque: *mut zend_string = 0 as *mut zend_string;
    let _flags: libc::c_int = 0 as libc::c_int;
    let mut _min_num_args: libc::c_int = 2 as libc::c_int;
    let mut _max_num_args: libc::c_int = 7 as libc::c_int;
    let mut _num_args: libc::c_int = (*execute_data).This.u2.num_args as libc::c_int;
    let mut _i: libc::c_int = 0 as libc::c_int;
    let mut _real_arg: *mut zval = 0 as *mut zval;
    let mut _arg: *mut zval = 0 as *mut zval;
    let mut _expected_type: zend_expected_type = Z_EXPECTED_LONG;
    let mut _error: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut _dummy: zend_bool = 0;
    let mut _optional: zend_bool = 0 as libc::c_int as zend_bool;
    let mut _error_code: libc::c_int = 0 as libc::c_int;
    let mut current_block_296: u64;
    if (_num_args < _min_num_args) as libc::c_int as libc::c_long != 0
        || (_num_args > _max_num_args) as libc::c_int as libc::c_long != 0
            && (_max_num_args >= 0 as libc::c_int) as libc::c_int as libc::c_long != 0
    {
        if _flags & (1 as libc::c_int) << 1 as libc::c_int == 0 {
            if _flags & (1 as libc::c_int) << 2 as libc::c_int != 0 {
                zend_wrong_parameters_count_exception(_min_num_args, _max_num_args);
            } else {
                zend_wrong_parameters_count_error(_min_num_args, _max_num_args);
            }
        }
        _error_code = 1 as libc::c_int;
    } else {
        _real_arg = (execute_data as *mut zval)
            .offset(
                (((::std::mem::size_of::<zend_execute_data>() as libc::c_ulong)
                    .wrapping_add(8 as libc::c_int as libc::c_ulong)
                    .wrapping_sub(1 as libc::c_int as libc::c_ulong)
                    & !(8 as libc::c_int - 1 as libc::c_int) as libc::c_ulong)
                    .wrapping_add(
                        (::std::mem::size_of::<zval>() as libc::c_ulong)
                            .wrapping_add(8 as libc::c_int as libc::c_ulong)
                            .wrapping_sub(1 as libc::c_int as libc::c_ulong)
                            & !(8 as libc::c_int - 1 as libc::c_int) as libc::c_ulong,
                    )
                    .wrapping_sub(1 as libc::c_int as libc::c_ulong)
                    .wrapping_div(
                        (::std::mem::size_of::<zval>() as libc::c_ulong)
                            .wrapping_add(8 as libc::c_int as libc::c_ulong)
                            .wrapping_sub(1 as libc::c_int as libc::c_ulong)
                            & !(8 as libc::c_int - 1 as libc::c_int) as libc::c_ulong,
                    ) as libc::c_int + (0 as libc::c_int - 1 as libc::c_int)) as isize,
            );
        _i += 1;
        _i;
        if !(_i <= _min_num_args || _optional as libc::c_int == 1 as libc::c_int)
            as libc::c_int as libc::c_long != 0
        {
            unreachable!();
        }
        if !(_i > _min_num_args || _optional as libc::c_int == 0 as libc::c_int)
            as libc::c_int as libc::c_long != 0
        {
            unreachable!();
        }
        if _optional != 0 {
            if (_i > _num_args) as libc::c_int as libc::c_long != 0 {
                current_block_296 = 316278526493857137;
            } else {
                current_block_296 = 7333393191927787629;
            }
        } else {
            current_block_296 = 7333393191927787629;
        }
        match current_block_296 {
            316278526493857137 => {}
            _ => {
                _real_arg = _real_arg.offset(1);
                _real_arg;
                _arg = _real_arg;
                if (zend_parse_arg_long(
                    _arg,
                    &mut partition,
                    &mut _dummy,
                    0 as libc::c_int,
                    0 as libc::c_int,
                ) == 0) as libc::c_int as libc::c_long != 0
                {
                    _expected_type = Z_EXPECTED_LONG;
                    _error_code = 4 as libc::c_int;
                } else {
                    _i += 1;
                    _i;
                    if !(_i <= _min_num_args
                        || _optional as libc::c_int == 1 as libc::c_int) as libc::c_int
                        as libc::c_long != 0
                    {
                        unreachable!();
                    }
                    if !(_i > _min_num_args
                        || _optional as libc::c_int == 0 as libc::c_int) as libc::c_int
                        as libc::c_long != 0
                    {
                        unreachable!();
                    }
                    if _optional != 0 {
                        if (_i > _num_args) as libc::c_int as libc::c_long != 0 {
                            current_block_296 = 316278526493857137;
                        } else {
                            current_block_296 = 2723324002591448311;
                        }
                    } else {
                        current_block_296 = 2723324002591448311;
                    }
                    match current_block_296 {
                        316278526493857137 => {}
                        _ => {
                            _real_arg = _real_arg.offset(1);
                            _real_arg;
                            _arg = _real_arg;
                            if (zend_parse_arg_long(
                                _arg,
                                &mut msgflags,
                                &mut _dummy,
                                0 as libc::c_int,
                                0 as libc::c_int,
                            ) == 0) as libc::c_int as libc::c_long != 0
                            {
                                _expected_type = Z_EXPECTED_LONG;
                                _error_code = 4 as libc::c_int;
                            } else {
                                _optional = 1 as libc::c_int as zend_bool;
                                _i += 1;
                                _i;
                                if !(_i <= _min_num_args
                                    || _optional as libc::c_int == 1 as libc::c_int)
                                    as libc::c_int as libc::c_long != 0
                                {
                                    unreachable!();
                                }
                                if !(_i > _min_num_args
                                    || _optional as libc::c_int == 0 as libc::c_int)
                                    as libc::c_int as libc::c_long != 0
                                {
                                    unreachable!();
                                }
                                if _optional != 0 {
                                    if (_i > _num_args) as libc::c_int as libc::c_long != 0 {
                                        current_block_296 = 316278526493857137;
                                    } else {
                                        current_block_296 = 7019009297990327870;
                                    }
                                } else {
                                    current_block_296 = 7019009297990327870;
                                }
                                match current_block_296 {
                                    316278526493857137 => {}
                                    _ => {
                                        _real_arg = _real_arg.offset(1);
                                        _real_arg;
                                        _arg = _real_arg;
                                        if (zend_parse_arg_string(
                                            _arg,
                                            &mut payload,
                                            &mut payload_len,
                                            1 as libc::c_int,
                                        ) == 0) as libc::c_int as libc::c_long != 0
                                        {
                                            _expected_type = Z_EXPECTED_STRING;
                                            _error_code = 4 as libc::c_int;
                                        } else {
                                            _i += 1;
                                            _i;
                                            if !(_i <= _min_num_args
                                                || _optional as libc::c_int == 1 as libc::c_int)
                                                as libc::c_int as libc::c_long != 0
                                            {
                                                unreachable!();
                                            }
                                            if !(_i > _min_num_args
                                                || _optional as libc::c_int == 0 as libc::c_int)
                                                as libc::c_int as libc::c_long != 0
                                            {
                                                unreachable!();
                                            }
                                            if _optional != 0 {
                                                if (_i > _num_args) as libc::c_int as libc::c_long != 0 {
                                                    current_block_296 = 316278526493857137;
                                                } else {
                                                    current_block_296 = 14184516523743666873;
                                                }
                                            } else {
                                                current_block_296 = 14184516523743666873;
                                            }
                                            match current_block_296 {
                                                316278526493857137 => {}
                                                _ => {
                                                    _real_arg = _real_arg.offset(1);
                                                    _real_arg;
                                                    _arg = _real_arg;
                                                    if (zend_parse_arg_string(
                                                        _arg,
                                                        &mut key,
                                                        &mut key_len,
                                                        1 as libc::c_int,
                                                    ) == 0) as libc::c_int as libc::c_long != 0
                                                    {
                                                        _expected_type = Z_EXPECTED_STRING;
                                                        _error_code = 4 as libc::c_int;
                                                    } else {
                                                        _i += 1;
                                                        _i;
                                                        if !(_i <= _min_num_args
                                                            || _optional as libc::c_int == 1 as libc::c_int)
                                                            as libc::c_int as libc::c_long != 0
                                                        {
                                                            unreachable!();
                                                        }
                                                        if !(_i > _min_num_args
                                                            || _optional as libc::c_int == 0 as libc::c_int)
                                                            as libc::c_int as libc::c_long != 0
                                                        {
                                                            unreachable!();
                                                        }
                                                        if _optional != 0 {
                                                            if (_i > _num_args) as libc::c_int as libc::c_long != 0 {
                                                                current_block_296 = 316278526493857137;
                                                            } else {
                                                                current_block_296 = 10710847385228534238;
                                                            }
                                                        } else {
                                                            current_block_296 = 10710847385228534238;
                                                        }
                                                        match current_block_296 {
                                                            316278526493857137 => {}
                                                            _ => {
                                                                _real_arg = _real_arg.offset(1);
                                                                _real_arg;
                                                                _arg = _real_arg;
                                                                if (zend_parse_arg_array_ht(
                                                                    _arg,
                                                                    &mut headersParam,
                                                                    1 as libc::c_int,
                                                                    0 as libc::c_int,
                                                                    0 as libc::c_int,
                                                                ) == 0) as libc::c_int as libc::c_long != 0
                                                                {
                                                                    _expected_type = Z_EXPECTED_ARRAY;
                                                                    _error_code = 4 as libc::c_int;
                                                                } else {
                                                                    _i += 1;
                                                                    _i;
                                                                    if !(_i <= _min_num_args
                                                                        || _optional as libc::c_int == 1 as libc::c_int)
                                                                        as libc::c_int as libc::c_long != 0
                                                                    {
                                                                        unreachable!();
                                                                    }
                                                                    if !(_i > _min_num_args
                                                                        || _optional as libc::c_int == 0 as libc::c_int)
                                                                        as libc::c_int as libc::c_long != 0
                                                                    {
                                                                        unreachable!();
                                                                    }
                                                                    if _optional != 0 {
                                                                        if (_i > _num_args) as libc::c_int as libc::c_long != 0 {
                                                                            current_block_296 = 316278526493857137;
                                                                        } else {
                                                                            current_block_296 = 16517180880614114163;
                                                                        }
                                                                    } else {
                                                                        current_block_296 = 16517180880614114163;
                                                                    }
                                                                    match current_block_296 {
                                                                        316278526493857137 => {}
                                                                        _ => {
                                                                            _real_arg = _real_arg.offset(1);
                                                                            _real_arg;
                                                                            _arg = _real_arg;
                                                                            if (zend_parse_arg_long(
                                                                                _arg,
                                                                                &mut timestamp_ms,
                                                                                &mut timestamp_ms_is_null,
                                                                                1 as libc::c_int,
                                                                                0 as libc::c_int,
                                                                            ) == 0) as libc::c_int as libc::c_long != 0
                                                                            {
                                                                                _expected_type = Z_EXPECTED_LONG;
                                                                                _error_code = 4 as libc::c_int;
                                                                            } else {
                                                                                _i += 1;
                                                                                _i;
                                                                                if !(_i <= _min_num_args
                                                                                    || _optional as libc::c_int == 1 as libc::c_int)
                                                                                    as libc::c_int as libc::c_long != 0
                                                                                {
                                                                                    unreachable!();
                                                                                }
                                                                                if !(_i > _min_num_args
                                                                                    || _optional as libc::c_int == 0 as libc::c_int)
                                                                                    as libc::c_int as libc::c_long != 0
                                                                                {
                                                                                    unreachable!();
                                                                                }
                                                                                if _optional != 0 {
                                                                                    if (_i > _num_args) as libc::c_int as libc::c_long != 0 {
                                                                                        current_block_296 = 316278526493857137;
                                                                                    } else {
                                                                                        current_block_296 = 4308757698705929541;
                                                                                    }
                                                                                } else {
                                                                                    current_block_296 = 4308757698705929541;
                                                                                }
                                                                                match current_block_296 {
                                                                                    316278526493857137 => {}
                                                                                    _ => {
                                                                                        _real_arg = _real_arg.offset(1);
                                                                                        _real_arg;
                                                                                        _arg = _real_arg;
                                                                                        if (zend_parse_arg_str(_arg, &mut opaque, 1 as libc::c_int)
                                                                                            == 0) as libc::c_int as libc::c_long != 0
                                                                                        {
                                                                                            _expected_type = Z_EXPECTED_STRING;
                                                                                            _error_code = 4 as libc::c_int;
                                                                                        }
                                                                                    }
                                                                                }
                                                                            }
                                                                        }
                                                                    }
                                                                }
                                                            }
                                                        }
                                                    }
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
    if (_error_code != 0 as libc::c_int) as libc::c_int as libc::c_long != 0 {
        if _flags & (1 as libc::c_int) << 1 as libc::c_int == 0 {
            if _error_code == 2 as libc::c_int {
                if _flags & (1 as libc::c_int) << 2 as libc::c_int != 0 {
                    zend_wrong_callback_exception(_i, _error);
                } else {
                    zend_wrong_callback_error(_i, _error);
                }
            } else if _error_code == 3 as libc::c_int {
                if _flags & (1 as libc::c_int) << 2 as libc::c_int != 0 {
                    zend_wrong_parameter_class_exception(_i, _error, _arg);
                } else {
                    zend_wrong_parameter_class_error(_i, _error, _arg);
                }
            } else if _error_code == 4 as libc::c_int {
                if _flags & (1 as libc::c_int) << 2 as libc::c_int != 0 {
                    zend_wrong_parameter_type_exception(_i, _expected_type, _arg);
                } else {
                    zend_wrong_parameter_type_error(_i, _expected_type, _arg);
                }
            }
        }
        return;
    }
    if partition != -(1 as libc::c_int) as libc::c_long
        && (partition < 0 as libc::c_int as libc::c_long
            || partition > 0x7fffffff as libc::c_int as libc::c_long)
    {
        zend_throw_exception_ex(
            spl_ce_InvalidArgumentException,
            0 as libc::c_int as zend_long,
            b"Out of range value '%ld' for $partition\0" as *const u8
                as *const libc::c_char,
            partition,
        );
        return;
    }
    if msgflags != 0 as libc::c_int as libc::c_long
        && msgflags != 0x4 as libc::c_int as libc::c_long
    {
        zend_throw_exception_ex(
            spl_ce_InvalidArgumentException,
            0 as libc::c_int as zend_long,
            b"Invalid value '%ld' for $msgflags\0" as *const u8 as *const libc::c_char,
            msgflags,
        );
        return;
    }
    if timestamp_ms_is_null as libc::c_int == 1 as libc::c_int {
        timestamp_ms = 0 as libc::c_int as zend_long;
    }
    intern = get_kafka_topic_object(
        if zval_get_type(&mut (*execute_data).This) as libc::c_int == 8 as libc::c_int {
            &mut (*execute_data).This
        } else {
            0 as *mut zval
        },
    );
    if !opaque.is_null() {
        zend_string_addref(opaque);
    }
    if !headersParam.is_null()
        && (*headersParam).nNumOfElements > 0 as libc::c_int as libc::c_uint
    {
        headers = rd_kafka_headers_new((*headersParam).nNumOfElements as size_t);
        zend_hash_internal_pointer_reset_ex(headersParam, &mut headersParamPos);
        loop {
            header_value = zend_hash_get_current_data_ex(
                headersParam,
                &mut headersParamPos,
            );
            if !(!header_value.is_null()
                && {
                    header_key = rdkafka_hash_get_current_key_ex(
                        headersParam,
                        &mut headersParamPos,
                    );
                    !header_key.is_null()
                })
            {
                break;
            }
            if zval_get_type(header_value) as libc::c_int != 6 as libc::c_int {
                if zval_get_type(header_value) as libc::c_int != 6 as libc::c_int {
                    _convert_to_string(header_value);
                }
            }
            rd_kafka_header_add(
                headers,
                header_key,
                -(1 as libc::c_int) as ssize_t,
                ((*(*header_value).value.str_0).val).as_mut_ptr() as *const libc::c_void,
                (*(*header_value).value.str_0).len as ssize_t,
            );
            zend_hash_move_forward_ex(headersParam, &mut headersParamPos);
        }
    } else {
        headers = rd_kafka_headers_new(0 as libc::c_int as size_t);
    }
    kafka_intern = get_kafka_object(&mut (*intern).zrk);
    if kafka_intern.is_null() {
        return;
    }
    err = rd_kafka_producev(
        (*kafka_intern).rk,
        ({ RD_KAFKA_VTYPE_RKT as libc::c_int }),
        (*intern).rkt,
        ({ RD_KAFKA_VTYPE_PARTITION as libc::c_int }),
        partition as int32_t,
        ({ RD_KAFKA_VTYPE_MSGFLAGS as libc::c_int }),
        msgflags as libc::c_int | 0x2 as libc::c_int,
        ({ RD_KAFKA_VTYPE_VALUE as libc::c_int }),
        payload as *mut libc::c_void,
        payload_len,
        ({ RD_KAFKA_VTYPE_KEY as libc::c_int }),
        key as *mut libc::c_void,
        key_len,
        ({ RD_KAFKA_VTYPE_TIMESTAMP as libc::c_int }),
        timestamp_ms,
        ({ RD_KAFKA_VTYPE_HEADERS as libc::c_int }),
        headers,
        ({ RD_KAFKA_VTYPE_OPAQUE as libc::c_int }),
        opaque as *mut libc::c_void,
        RD_KAFKA_VTYPE_END as libc::c_int,
    );
    if err as libc::c_int != RD_KAFKA_RESP_ERR_NO_ERROR as libc::c_int {
        rd_kafka_headers_destroy(headers);
        if !opaque.is_null() {
            zend_string_release(opaque);
        }
        zend_throw_exception(
            ce_kafka_exception,
            rd_kafka_err2str(err),
            err as zend_long,
        );
        return;
    }
}
pub unsafe extern "C" fn zim_RdKafka_Topic_getName(
    mut execute_data: *mut zend_execute_data,
    mut return_value: *mut zval,
) {
    let mut intern: *mut kafka_topic_object = 0 as *mut kafka_topic_object;
    if (if ((*execute_data).This.u2.num_args == 0 as libc::c_int as libc::c_uint)
        as libc::c_int as libc::c_long != 0
    {
        SUCCESS as libc::c_int
    } else {
        zend_wrong_parameters_none_error();
        FAILURE as libc::c_int
    }) == FAILURE as libc::c_int
    {
        return;
    }
    intern = get_kafka_topic_object(
        if zval_get_type(&mut (*execute_data).This) as libc::c_int == 8 as libc::c_int {
            &mut (*execute_data).This
        } else {
            0 as *mut zval
        },
    );
    if intern.is_null() {
        return;
    }
    let mut _s: *const libc::c_char = rd_kafka_topic_name((*intern).rkt);
    let mut __z: *mut zval = return_value;
    let mut __s: *mut zend_string = zend_string_init(_s, strlen(_s), 0 as libc::c_int);
    (*__z).value.str_0 = __s;
    (*__z)
        .u1
        .type_info = (6 as libc::c_int
        | ((1 as libc::c_int) << 0 as libc::c_int) << 8 as libc::c_int) as uint32_t;
}
pub unsafe extern "C" fn kafka_topic_minit(
    mut type_0: libc::c_int,
    mut module_number: libc::c_int,
) {
    memcpy(
        &mut object_handlers as *mut zend_object_handlers as *mut libc::c_void,
        &std_object_handlers as *const zend_object_handlers as *const libc::c_void,
        ::std::mem::size_of::<zend_object_handlers>() as libc::c_ulong,
    );
    object_handlers.clone_obj = None;
    object_handlers
        .free_obj = Some(
        kafka_topic_free as unsafe extern "C" fn(*mut zend_object) -> (),
    );
    object_handlers.offset = 24 as libc::c_ulong as libc::c_int;
    ce_kafka_topic = register_class_RdKafka_Topic();
    (*ce_kafka_topic)
        .c2rust_unnamed_0
        .create_object = Some(
        kafka_topic_new
            as unsafe extern "C" fn(*mut zend_class_entry) -> *mut zend_object,
    );
    ce_kafka_consumer_topic = register_class_RdKafka_ConsumerTopic(ce_kafka_topic);
    ce_kafka_kafka_consumer_topic = register_class_RdKafka_KafkaConsumerTopic(
        ce_kafka_topic,
    );
    ce_kafka_producer_topic = register_class_RdKafka_ProducerTopic(ce_kafka_topic);
}
unsafe extern "C" fn run_static_initializers() {
    class_RdKafka_Topic_methods = [
        {
            let mut init = _zend_function_entry {
                fname: b"getName\0" as *const u8 as *const libc::c_char,
                handler: Some(
                    zim_RdKafka_Topic_getName
                        as unsafe extern "C" fn(*mut zend_execute_data, *mut zval) -> (),
                ),
                arg_info: arginfo_class_RdKafka_Topic_getName.as_ptr(),
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
    class_RdKafka_ConsumerTopic_methods = [
        {
            let mut init = _zend_function_entry {
                fname: b"__construct\0" as *const u8 as *const libc::c_char,
                handler: Some(
                    zim_RdKafka___construct
                        as unsafe extern "C" fn(*mut zend_execute_data, *mut zval) -> (),
                ),
                arg_info: arginfo_class_RdKafka_ConsumerTopic___construct.as_ptr(),
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
                fname: b"consumeQueueStart\0" as *const u8 as *const libc::c_char,
                handler: Some(
                    zim_RdKafka_ConsumerTopic_consumeQueueStart
                        as unsafe extern "C" fn(*mut zend_execute_data, *mut zval) -> (),
                ),
                arg_info: arginfo_class_RdKafka_ConsumerTopic_consumeQueueStart.as_ptr(),
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
                fname: b"consumeCallback\0" as *const u8 as *const libc::c_char,
                handler: Some(
                    zim_RdKafka_ConsumerTopic_consumeCallback
                        as unsafe extern "C" fn(*mut zend_execute_data, *mut zval) -> (),
                ),
                arg_info: arginfo_class_RdKafka_ConsumerTopic_consumeCallback.as_ptr(),
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
                fname: b"consumeStart\0" as *const u8 as *const libc::c_char,
                handler: Some(
                    zim_RdKafka_ConsumerTopic_consumeStart
                        as unsafe extern "C" fn(*mut zend_execute_data, *mut zval) -> (),
                ),
                arg_info: arginfo_class_RdKafka_ConsumerTopic_consumeStart.as_ptr(),
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
                fname: b"consumeStop\0" as *const u8 as *const libc::c_char,
                handler: Some(
                    zim_RdKafka_ConsumerTopic_consumeStop
                        as unsafe extern "C" fn(*mut zend_execute_data, *mut zval) -> (),
                ),
                arg_info: arginfo_class_RdKafka_ConsumerTopic_consumeStop.as_ptr(),
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
                fname: b"consume\0" as *const u8 as *const libc::c_char,
                handler: Some(
                    zim_RdKafka_ConsumerTopic_consume
                        as unsafe extern "C" fn(*mut zend_execute_data, *mut zval) -> (),
                ),
                arg_info: arginfo_class_RdKafka_ConsumerTopic_consume.as_ptr(),
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
                fname: b"consumeBatch\0" as *const u8 as *const libc::c_char,
                handler: Some(
                    zim_RdKafka_ConsumerTopic_consumeBatch
                        as unsafe extern "C" fn(*mut zend_execute_data, *mut zval) -> (),
                ),
                arg_info: arginfo_class_RdKafka_ConsumerTopic_consumeBatch.as_ptr(),
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
                fname: b"offsetStore\0" as *const u8 as *const libc::c_char,
                handler: Some(
                    zim_RdKafka_ConsumerTopic_offsetStore
                        as unsafe extern "C" fn(*mut zend_execute_data, *mut zval) -> (),
                ),
                arg_info: arginfo_class_RdKafka_ConsumerTopic_consumeStart.as_ptr(),
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
                fname: 0 as *const libc::c_char,
                handler: None,
                arg_info: 0 as *const _zend_internal_arg_info,
                num_args: 0 as libc::c_int as uint32_t,
                flags: 0 as libc::c_int as uint32_t,
            };
            init
        },
    ];
    class_RdKafka_KafkaConsumerTopic_methods = [
        {
            let mut init = _zend_function_entry {
                fname: b"__construct\0" as *const u8 as *const libc::c_char,
                handler: Some(
                    zim_RdKafka___construct
                        as unsafe extern "C" fn(*mut zend_execute_data, *mut zval) -> (),
                ),
                arg_info: arginfo_class_RdKafka_ConsumerTopic___construct.as_ptr(),
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
                fname: b"offsetStore\0" as *const u8 as *const libc::c_char,
                handler: Some(
                    zim_RdKafka_ConsumerTopic_offsetStore
                        as unsafe extern "C" fn(*mut zend_execute_data, *mut zval) -> (),
                ),
                arg_info: arginfo_class_RdKafka_ConsumerTopic_consumeStart.as_ptr(),
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
                fname: 0 as *const libc::c_char,
                handler: None,
                arg_info: 0 as *const _zend_internal_arg_info,
                num_args: 0 as libc::c_int as uint32_t,
                flags: 0 as libc::c_int as uint32_t,
            };
            init
        },
    ];
    class_RdKafka_ProducerTopic_methods = [
        {
            let mut init = _zend_function_entry {
                fname: b"__construct\0" as *const u8 as *const libc::c_char,
                handler: Some(
                    zim_RdKafka___construct
                        as unsafe extern "C" fn(*mut zend_execute_data, *mut zval) -> (),
                ),
                arg_info: arginfo_class_RdKafka_ConsumerTopic___construct.as_ptr(),
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
                fname: b"produce\0" as *const u8 as *const libc::c_char,
                handler: Some(
                    zim_RdKafka_ProducerTopic_produce
                        as unsafe extern "C" fn(*mut zend_execute_data, *mut zval) -> (),
                ),
                arg_info: arginfo_class_RdKafka_ProducerTopic_produce.as_ptr(),
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
                fname: b"producev\0" as *const u8 as *const libc::c_char,
                handler: Some(
                    zim_RdKafka_ProducerTopic_producev
                        as unsafe extern "C" fn(*mut zend_execute_data, *mut zval) -> (),
                ),
                arg_info: arginfo_class_RdKafka_ProducerTopic_producev.as_ptr(),
                num_args: (::std::mem::size_of::<[zend_internal_arg_info; 8]>()
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
