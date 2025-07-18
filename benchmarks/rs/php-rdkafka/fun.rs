use ::libc;
extern "C" {
    pub type _zend_unserialize_data;
    pub type _zend_serialize_data;
    fn memcpy(
        _: *mut libc::c_void,
        _: *const libc::c_void,
        _: libc::c_ulong,
    ) -> *mut libc::c_void;
    fn strlen(_: *const libc::c_char) -> libc::c_ulong;
    fn _emalloc(size: size_t) -> *mut libc::c_void;
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
    fn zend_hash_next_index_insert(ht: *mut HashTable, pData: *mut zval) -> *mut zval;
    fn _zend_new_array_0() -> *mut HashTable;
    fn _zend_new_array(size: uint32_t) -> *mut HashTable;
    fn __errno_location() -> *mut libc::c_int;
    fn zend_parse_parameters(
        num_args: libc::c_int,
        type_spec: *const libc::c_char,
        _: ...
    ) -> libc::c_int;
    fn add_assoc_long_ex(
        arg: *mut zval,
        key: *const libc::c_char,
        key_len: size_t,
        n: zend_long,
    ) -> libc::c_int;
    fn add_assoc_null_ex(
        arg: *mut zval,
        key: *const libc::c_char,
        key_len: size_t,
    ) -> libc::c_int;
    fn add_assoc_string_ex(
        arg: *mut zval,
        key: *const libc::c_char,
        key_len: size_t,
        str: *const libc::c_char,
    ) -> libc::c_int;
    fn zend_wrong_parameters_none_error() -> libc::c_int;
    fn rd_kafka_get_err_descs(
        errdescs: *mut *const rd_kafka_err_desc,
        cntp: *mut size_t,
    );
    fn rd_kafka_err2str(err: rd_kafka_resp_err_t) -> *const libc::c_char;
    fn rd_kafka_err2name(err: rd_kafka_resp_err_t) -> *const libc::c_char;
    fn rd_kafka_errno2err(errnox: libc::c_int) -> rd_kafka_resp_err_t;
    fn rd_kafka_thread_cnt() -> libc::c_int;
}
pub type size_t = libc::c_ulong;
pub type __uint8_t = libc::c_uchar;
pub type __uint16_t = libc::c_ushort;
pub type __uint32_t = libc::c_uint;
pub type __int64_t = libc::c_long;
pub type __uint64_t = libc::c_ulong;
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
#[inline(always)]
unsafe extern "C" fn zend_gc_set_refcount(
    mut p: *mut zend_refcounted_h,
    mut rc: uint32_t,
) -> uint32_t {
    (*p).refcount = rc;
    return (*p).refcount;
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
unsafe extern "C" fn add_next_index_zval(
    mut arg: *mut zval,
    mut value: *mut zval,
) -> libc::c_int {
    return if !(zend_hash_next_index_insert((*arg).value.arr, value)).is_null() {
        SUCCESS as libc::c_int
    } else {
        FAILURE as libc::c_int
    };
}
pub unsafe extern "C" fn zif_rd_kafka_get_err_descs(
    mut execute_data: *mut zend_execute_data,
    mut return_value: *mut zval,
) {
    let mut errdescs: *const rd_kafka_err_desc = 0 as *const rd_kafka_err_desc;
    let mut cnt: size_t = 0;
    let mut i: size_t = 0;
    let mut seen_zero: libc::c_int = 0 as libc::c_int;
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
    rd_kafka_get_err_descs(&mut errdescs, &mut cnt);
    let mut __arr: *mut zend_array = if 0 != 0 {
        if cnt as uint32_t <= 8 as libc::c_int as libc::c_uint {
            _zend_new_array_0()
        } else {
            _zend_new_array(cnt as uint32_t)
        }
    } else {
        _zend_new_array(cnt as uint32_t)
    };
    let mut __z: *mut zval = return_value;
    (*__z).value.arr = __arr;
    (*__z)
        .u1
        .type_info = (7 as libc::c_int
        | ((1 as libc::c_int) << 0 as libc::c_int) << 8 as libc::c_int
        | ((1 as libc::c_int) << 1 as libc::c_int) << 8 as libc::c_int) as uint32_t;
    let mut current_block_28: u64;
    i = 0 as libc::c_int as size_t;
    while i < cnt {
        let mut desc: *const rd_kafka_err_desc = &*errdescs.offset(i as isize)
            as *const rd_kafka_err_desc;
        let mut el: zval = zval {
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
        if (*desc).code as libc::c_int == 0 as libc::c_int {
            if seen_zero != 0 {
                current_block_28 = 13513818773234778473;
            } else {
                seen_zero = 1 as libc::c_int;
                current_block_28 = 3512920355445576850;
            }
        } else {
            current_block_28 = 3512920355445576850;
        }
        match current_block_28 {
            3512920355445576850 => {
                el.u1.type_info = 1 as libc::c_int as uint32_t;
                let mut __arr_0: *mut zend_array = if 0 != 0 {
                    if 0 as libc::c_int as uint32_t <= 8 as libc::c_int as libc::c_uint {
                        _zend_new_array_0()
                    } else {
                        _zend_new_array(0 as libc::c_int as uint32_t)
                    }
                } else {
                    _zend_new_array(0 as libc::c_int as uint32_t)
                };
                let mut __z_0: *mut zval = &mut el;
                (*__z_0).value.arr = __arr_0;
                (*__z_0)
                    .u1
                    .type_info = (7 as libc::c_int
                    | ((1 as libc::c_int) << 0 as libc::c_int) << 8 as libc::c_int
                    | ((1 as libc::c_int) << 1 as libc::c_int) << 8 as libc::c_int)
                    as uint32_t;
                add_assoc_long_ex(
                    &mut el,
                    b"code\0" as *const u8 as *const libc::c_char,
                    strlen(b"code\0" as *const u8 as *const libc::c_char),
                    (*desc).code as zend_long,
                );
                if !((*desc).name).is_null() {
                    add_assoc_string_ex(
                        &mut el,
                        b"name\0" as *const u8 as *const libc::c_char,
                        strlen(b"name\0" as *const u8 as *const libc::c_char),
                        (*desc).name as *mut libc::c_char,
                    );
                } else {
                    add_assoc_null_ex(
                        &mut el,
                        b"name\0" as *const u8 as *const libc::c_char,
                        strlen(b"name\0" as *const u8 as *const libc::c_char),
                    );
                }
                if !((*desc).desc).is_null() {
                    add_assoc_string_ex(
                        &mut el,
                        b"desc\0" as *const u8 as *const libc::c_char,
                        strlen(b"desc\0" as *const u8 as *const libc::c_char),
                        (*desc).desc as *mut libc::c_char,
                    );
                } else {
                    add_assoc_null_ex(
                        &mut el,
                        b"desc\0" as *const u8 as *const libc::c_char,
                        strlen(b"desc\0" as *const u8 as *const libc::c_char),
                    );
                }
                add_next_index_zval(return_value, &mut el);
            }
            _ => {}
        }
        i = i.wrapping_add(1);
        i;
    }
}
pub unsafe extern "C" fn zif_rd_kafka_err2name(
    mut execute_data: *mut zend_execute_data,
    mut return_value: *mut zval,
) {
    let mut err: zend_long = 0;
    let mut name: *const libc::c_char = 0 as *const libc::c_char;
    if zend_parse_parameters(
        (*execute_data).This.u2.num_args as libc::c_int,
        b"l\0" as *const u8 as *const libc::c_char,
        &mut err as *mut zend_long,
    ) == FAILURE as libc::c_int
    {
        return;
    }
    name = rd_kafka_err2name(err as rd_kafka_resp_err_t);
    if !name.is_null() {
        let mut _s: *const libc::c_char = name;
        let mut __z: *mut zval = return_value;
        let mut __s: *mut zend_string = zend_string_init(
            _s,
            strlen(_s),
            0 as libc::c_int,
        );
        (*__z).value.str_0 = __s;
        (*__z)
            .u1
            .type_info = (6 as libc::c_int
            | ((1 as libc::c_int) << 0 as libc::c_int) << 8 as libc::c_int) as uint32_t;
        return;
    }
}
pub unsafe extern "C" fn zif_rd_kafka_err2str(
    mut execute_data: *mut zend_execute_data,
    mut return_value: *mut zval,
) {
    let mut err: zend_long = 0;
    let mut errstr: *const libc::c_char = 0 as *const libc::c_char;
    if zend_parse_parameters(
        (*execute_data).This.u2.num_args as libc::c_int,
        b"l\0" as *const u8 as *const libc::c_char,
        &mut err as *mut zend_long,
    ) == FAILURE as libc::c_int
    {
        return;
    }
    errstr = rd_kafka_err2str(err as rd_kafka_resp_err_t);
    if !errstr.is_null() {
        let mut _s: *const libc::c_char = errstr;
        let mut __z: *mut zval = return_value;
        let mut __s: *mut zend_string = zend_string_init(
            _s,
            strlen(_s),
            0 as libc::c_int,
        );
        (*__z).value.str_0 = __s;
        (*__z)
            .u1
            .type_info = (6 as libc::c_int
            | ((1 as libc::c_int) << 0 as libc::c_int) << 8 as libc::c_int) as uint32_t;
        return;
    }
}
pub unsafe extern "C" fn zif_rd_kafka_errno(
    mut execute_data: *mut zend_execute_data,
    mut return_value: *mut zval,
) {
    if zend_parse_parameters(
        (*execute_data).This.u2.num_args as libc::c_int,
        b"\0" as *const u8 as *const libc::c_char,
    ) == FAILURE as libc::c_int
    {
        return;
    }
    let mut __z: *mut zval = return_value;
    (*__z).value.lval = *__errno_location() as zend_long;
    (*__z).u1.type_info = 4 as libc::c_int as uint32_t;
}
pub unsafe extern "C" fn zif_rd_kafka_errno2err(
    mut execute_data: *mut zend_execute_data,
    mut return_value: *mut zval,
) {
    let mut errnox: zend_long = 0;
    if zend_parse_parameters(
        (*execute_data).This.u2.num_args as libc::c_int,
        b"l\0" as *const u8 as *const libc::c_char,
        &mut errnox as *mut zend_long,
    ) == FAILURE as libc::c_int
    {
        return;
    }
    let mut __z: *mut zval = return_value;
    (*__z).value.lval = rd_kafka_errno2err(errnox as libc::c_int) as zend_long;
    (*__z).u1.type_info = 4 as libc::c_int as uint32_t;
}
pub unsafe extern "C" fn zif_rd_kafka_thread_cnt(
    mut execute_data: *mut zend_execute_data,
    mut return_value: *mut zval,
) {
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
    let mut __z: *mut zval = return_value;
    (*__z).value.lval = rd_kafka_thread_cnt() as zend_long;
    (*__z).u1.type_info = 4 as libc::c_int as uint32_t;
}
pub unsafe extern "C" fn zif_rd_kafka_offset_tail(
    mut execute_data: *mut zend_execute_data,
    mut return_value: *mut zval,
) {
    let mut cnt: zend_long = 0;
    if zend_parse_parameters(
        (*execute_data).This.u2.num_args as libc::c_int,
        b"l\0" as *const u8 as *const libc::c_char,
        &mut cnt as *mut zend_long,
    ) == FAILURE as libc::c_int
    {
        return;
    }
    let mut __z: *mut zval = return_value;
    (*__z).value.lval = -(2000 as libc::c_int) as libc::c_long - cnt;
    (*__z).u1.type_info = 4 as libc::c_int as uint32_t;
}
