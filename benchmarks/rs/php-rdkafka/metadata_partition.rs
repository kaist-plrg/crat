use ::libc;
extern "C" {
    pub type _zend_unserialize_data;
    pub type _zend_serialize_data;
    static mut zend_string_init_interned: zend_string_init_interned_func_t;
    fn memset(
        _: *mut libc::c_void,
        _: libc::c_int,
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
    fn _zend_new_array_0() -> *mut HashTable;
    fn _zend_new_array(size: uint32_t) -> *mut HashTable;
    fn rc_dtor_func(p: *mut zend_refcounted);
    fn zval_ptr_dtor(zval_ptr: *mut zval);
    fn zend_object_std_init(object: *mut zend_object, ce_0: *mut zend_class_entry);
    fn zend_object_std_dtor(object: *mut zend_object);
    fn zend_register_internal_class_ex(
        class_entry: *mut zend_class_entry,
        parent_ce: *mut zend_class_entry,
    ) -> *mut zend_class_entry;
    fn object_init_ex(arg: *mut zval, ce_0: *mut zend_class_entry) -> libc::c_int;
    fn object_properties_init(
        object: *mut zend_object,
        class_type: *mut zend_class_entry,
    );
    fn add_assoc_long_ex(
        arg: *mut zval,
        key: *const libc::c_char,
        key_len: size_t,
        n: zend_long,
    ) -> libc::c_int;
    fn zend_wrong_parameters_none_error() -> libc::c_int;
    static mut kafka_default_object_handlers: zend_object_handlers;
    fn zim_RdKafka___construct(
        execute_data: *mut zend_execute_data,
        return_value: *mut zval,
    );
    fn zend_throw_exception_ex(
        exception_ce: *mut zend_class_entry,
        code: zend_long,
        format: *const libc::c_char,
        _: ...
    ) -> *mut zend_object;
    fn kafka_metadata_collection_init(
        return_value: *mut zval,
        zmetadata: *mut zval,
        items: *const libc::c_void,
        item_cnt: size_t,
        item_size: size_t,
        ctor: kafka_metadata_collection_ctor_t,
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
pub type zend_function_entry = _zend_function_entry;
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
pub struct rd_kafka_metadata_partition {
    pub id: int32_t,
    pub err: rd_kafka_resp_err_t,
    pub leader: int32_t,
    pub replica_cnt: libc::c_int,
    pub replicas: *mut int32_t,
    pub isr_cnt: libc::c_int,
    pub isrs: *mut int32_t,
}
pub type rd_kafka_metadata_partition_t = rd_kafka_metadata_partition;
pub type kafka_metadata_collection_ctor_t = Option::<
    unsafe extern "C" fn(*mut zval, *mut zval, *const libc::c_void) -> (),
>;
pub type object_intern = _object_intern;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _object_intern {
    pub zmetadata: zval,
    pub metadata_partition: *const rd_kafka_metadata_partition_t,
    pub std: zend_object,
}
#[inline(always)]
unsafe extern "C" fn zval_get_type(mut pz: *const zval) -> zend_uchar {
    return (*pz).u1.v.type_0;
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
unsafe extern "C" fn zval_delref_p(mut pz: *mut zval) -> uint32_t {
    if !((*pz).u1.v.type_flags as libc::c_int != 0 as libc::c_int) as libc::c_int
        as libc::c_long != 0
    {
        unreachable!();
    }
    return zend_gc_delref(&mut (*(*pz).value.counted).gc);
}
#[inline(always)]
unsafe extern "C" fn zval_ptr_dtor_nogc(mut zval_ptr: *mut zval) {
    if (*zval_ptr).u1.v.type_flags as libc::c_int != 0 as libc::c_int
        && zval_delref_p(zval_ptr) == 0
    {
        rc_dtor_func((*zval_ptr).value.counted);
    }
}
#[inline(always)]
unsafe extern "C" fn zend_object_properties_size(
    mut ce_0: *mut zend_class_entry,
) -> size_t {
    return (::std::mem::size_of::<zval>() as libc::c_ulong)
        .wrapping_mul(
            ((*ce_0).default_properties_count
                - (if (*ce_0).ce_flags
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
    mut ce_0: *mut zend_class_entry,
) -> *mut libc::c_void {
    let mut obj: *mut libc::c_void = if 0 != 0 {
        if obj_size.wrapping_add(zend_object_properties_size(ce_0))
            <= 8 as libc::c_int as libc::c_ulong
        {
            _emalloc_8()
        } else if obj_size.wrapping_add(zend_object_properties_size(ce_0))
            <= 16 as libc::c_int as libc::c_ulong
        {
            _emalloc_16()
        } else if obj_size.wrapping_add(zend_object_properties_size(ce_0))
            <= 24 as libc::c_int as libc::c_ulong
        {
            _emalloc_24()
        } else if obj_size.wrapping_add(zend_object_properties_size(ce_0))
            <= 32 as libc::c_int as libc::c_ulong
        {
            _emalloc_32()
        } else if obj_size.wrapping_add(zend_object_properties_size(ce_0))
            <= 40 as libc::c_int as libc::c_ulong
        {
            _emalloc_40()
        } else if obj_size.wrapping_add(zend_object_properties_size(ce_0))
            <= 48 as libc::c_int as libc::c_ulong
        {
            _emalloc_48()
        } else if obj_size.wrapping_add(zend_object_properties_size(ce_0))
            <= 56 as libc::c_int as libc::c_ulong
        {
            _emalloc_56()
        } else if obj_size.wrapping_add(zend_object_properties_size(ce_0))
            <= 64 as libc::c_int as libc::c_ulong
        {
            _emalloc_64()
        } else if obj_size.wrapping_add(zend_object_properties_size(ce_0))
            <= 80 as libc::c_int as libc::c_ulong
        {
            _emalloc_80()
        } else if obj_size.wrapping_add(zend_object_properties_size(ce_0))
            <= 96 as libc::c_int as libc::c_ulong
        {
            _emalloc_96()
        } else if obj_size.wrapping_add(zend_object_properties_size(ce_0))
            <= 112 as libc::c_int as libc::c_ulong
        {
            _emalloc_112()
        } else if obj_size.wrapping_add(zend_object_properties_size(ce_0))
            <= 128 as libc::c_int as libc::c_ulong
        {
            _emalloc_128()
        } else if obj_size.wrapping_add(zend_object_properties_size(ce_0))
            <= 160 as libc::c_int as libc::c_ulong
        {
            _emalloc_160()
        } else if obj_size.wrapping_add(zend_object_properties_size(ce_0))
            <= 192 as libc::c_int as libc::c_ulong
        {
            _emalloc_192()
        } else if obj_size.wrapping_add(zend_object_properties_size(ce_0))
            <= 224 as libc::c_int as libc::c_ulong
        {
            _emalloc_224()
        } else if obj_size.wrapping_add(zend_object_properties_size(ce_0))
            <= 256 as libc::c_int as libc::c_ulong
        {
            _emalloc_256()
        } else if obj_size.wrapping_add(zend_object_properties_size(ce_0))
            <= 320 as libc::c_int as libc::c_ulong
        {
            _emalloc_320()
        } else if obj_size.wrapping_add(zend_object_properties_size(ce_0))
            <= 384 as libc::c_int as libc::c_ulong
        {
            _emalloc_384()
        } else if obj_size.wrapping_add(zend_object_properties_size(ce_0))
            <= 448 as libc::c_int as libc::c_ulong
        {
            _emalloc_448()
        } else if obj_size.wrapping_add(zend_object_properties_size(ce_0))
            <= 512 as libc::c_int as libc::c_ulong
        {
            _emalloc_512()
        } else if obj_size.wrapping_add(zend_object_properties_size(ce_0))
            <= 640 as libc::c_int as libc::c_ulong
        {
            _emalloc_640()
        } else if obj_size.wrapping_add(zend_object_properties_size(ce_0))
            <= 768 as libc::c_int as libc::c_ulong
        {
            _emalloc_768()
        } else if obj_size.wrapping_add(zend_object_properties_size(ce_0))
            <= 896 as libc::c_int as libc::c_ulong
        {
            _emalloc_896()
        } else if obj_size.wrapping_add(zend_object_properties_size(ce_0))
            <= 1024 as libc::c_int as libc::c_ulong
        {
            _emalloc_1024()
        } else if obj_size.wrapping_add(zend_object_properties_size(ce_0))
            <= 1280 as libc::c_int as libc::c_ulong
        {
            _emalloc_1280()
        } else if obj_size.wrapping_add(zend_object_properties_size(ce_0))
            <= 1536 as libc::c_int as libc::c_ulong
        {
            _emalloc_1536()
        } else if obj_size.wrapping_add(zend_object_properties_size(ce_0))
            <= 1792 as libc::c_int as libc::c_ulong
        {
            _emalloc_1792()
        } else if obj_size.wrapping_add(zend_object_properties_size(ce_0))
            <= 2048 as libc::c_int as libc::c_ulong
        {
            _emalloc_2048()
        } else if obj_size.wrapping_add(zend_object_properties_size(ce_0))
            <= 2560 as libc::c_int as libc::c_ulong
        {
            _emalloc_2560()
        } else if obj_size.wrapping_add(zend_object_properties_size(ce_0))
            <= 3072 as libc::c_int as libc::c_ulong
        {
            _emalloc_3072()
        } else if obj_size.wrapping_add(zend_object_properties_size(ce_0))
            <= (2 as libc::c_int * 1024 as libc::c_int * 1024 as libc::c_int
                - 4 as libc::c_int * 1024 as libc::c_int * 1 as libc::c_int)
                as libc::c_ulong
        {
            _emalloc_large(obj_size.wrapping_add(zend_object_properties_size(ce_0)))
        } else {
            _emalloc_huge(obj_size.wrapping_add(zend_object_properties_size(ce_0)))
        }
    } else {
        _emalloc(obj_size.wrapping_add(zend_object_properties_size(ce_0)))
    };
    memset(
        obj,
        0 as libc::c_int,
        obj_size.wrapping_sub(::std::mem::size_of::<zval>() as libc::c_ulong),
    );
    return obj;
}
static mut arginfo_class_RdKafka_Metadata_Partition___construct: [zend_internal_arg_info; 1] = [
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
static mut arginfo_class_RdKafka_Metadata_Partition_getId: [zend_internal_arg_info; 1] = [
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
static mut class_RdKafka_Metadata_Partition_methods: [zend_function_entry; 7] = [zend_function_entry {
    fname: 0 as *const libc::c_char,
    handler: None,
    arg_info: 0 as *const _zend_internal_arg_info,
    num_args: 0,
    flags: 0,
}; 7];
unsafe extern "C" fn register_class_RdKafka_Metadata_Partition() -> *mut zend_class_entry {
    let mut ce_0: zend_class_entry = zend_class_entry {
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
        &mut ce_0 as *mut zend_class_entry as *mut libc::c_void,
        0 as libc::c_int,
        ::std::mem::size_of::<zend_class_entry>() as libc::c_ulong,
    );
    ce_0
        .name = zend_string_init_interned
        .unwrap()(
        b"RdKafka\\Metadata\\Partition\0" as *const u8 as *const libc::c_char,
        (::std::mem::size_of::<[libc::c_char; 27]>() as libc::c_ulong)
            .wrapping_sub(1 as libc::c_int as libc::c_ulong),
        1 as libc::c_int,
    );
    ce_0
        .info
        .internal
        .builtin_functions = class_RdKafka_Metadata_Partition_methods.as_ptr();
    class_entry = zend_register_internal_class_ex(&mut ce_0, 0 as *mut zend_class_entry);
    return class_entry;
}
static mut ce: *mut zend_class_entry = 0 as *const zend_class_entry
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
unsafe extern "C" fn free_object(mut object: *mut zend_object) {
    let mut intern: *mut object_intern = (object as *mut libc::c_char)
        .offset(-(24 as libc::c_ulong as isize)) as *mut object_intern;
    if !((*intern).metadata_partition).is_null() {
        zval_ptr_dtor_nogc(&mut (*intern).zmetadata);
    }
    zend_object_std_dtor(&mut (*intern).std);
}
unsafe extern "C" fn create_object(
    mut class_type: *mut zend_class_entry,
) -> *mut zend_object {
    let mut retval: *mut zend_object = 0 as *mut zend_object;
    let mut intern: *mut object_intern = 0 as *mut object_intern;
    intern = zend_object_alloc(
        ::std::mem::size_of::<object_intern>() as libc::c_ulong,
        class_type,
    ) as *mut object_intern;
    zend_object_std_init(&mut (*intern).std, class_type);
    object_properties_init(&mut (*intern).std, class_type);
    retval = &mut (*intern).std;
    (*retval).handlers = &mut handlers;
    return retval;
}
unsafe extern "C" fn get_object(mut zmt: *mut zval) -> *mut object_intern {
    let mut omt: *mut object_intern = ((*zmt).value.obj as *mut libc::c_char)
        .offset(-(24 as libc::c_ulong as isize)) as *mut object_intern;
    if ((*omt).metadata_partition).is_null() {
        zend_throw_exception_ex(
            0 as *mut zend_class_entry,
            0 as libc::c_int as zend_long,
            b"RdKafka\\Metadata\\Partition::__construct() has not been called\0"
                as *const u8 as *const libc::c_char,
        );
        return 0 as *mut object_intern;
    }
    return omt;
}
unsafe extern "C" fn get_debug_info(
    mut object: *mut zval,
    mut is_temp: *mut libc::c_int,
) -> *mut HashTable {
    let mut ary: zval = zval {
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
    let mut intern: *mut object_intern = 0 as *mut object_intern;
    *is_temp = 1 as libc::c_int;
    let mut __arr: *mut zend_array = if 0 != 0 {
        if 0 as libc::c_int as uint32_t <= 8 as libc::c_int as libc::c_uint {
            _zend_new_array_0()
        } else {
            _zend_new_array(0 as libc::c_int as uint32_t)
        }
    } else {
        _zend_new_array(0 as libc::c_int as uint32_t)
    };
    let mut __z: *mut zval = &mut ary;
    (*__z).value.arr = __arr;
    (*__z)
        .u1
        .type_info = (7 as libc::c_int
        | ((1 as libc::c_int) << 0 as libc::c_int) << 8 as libc::c_int
        | ((1 as libc::c_int) << 1 as libc::c_int) << 8 as libc::c_int) as uint32_t;
    intern = ((*object).value.obj as *mut libc::c_char)
        .offset(-(24 as libc::c_ulong as isize)) as *mut object_intern;
    if intern.is_null() || ((*intern).metadata_partition).is_null() {
        return ary.value.arr;
    }
    add_assoc_long_ex(
        &mut ary,
        b"id\0" as *const u8 as *const libc::c_char,
        strlen(b"id\0" as *const u8 as *const libc::c_char),
        (*(*intern).metadata_partition).id as zend_long,
    );
    add_assoc_long_ex(
        &mut ary,
        b"err\0" as *const u8 as *const libc::c_char,
        strlen(b"err\0" as *const u8 as *const libc::c_char),
        (*(*intern).metadata_partition).err as zend_long,
    );
    add_assoc_long_ex(
        &mut ary,
        b"leader\0" as *const u8 as *const libc::c_char,
        strlen(b"leader\0" as *const u8 as *const libc::c_char),
        (*(*intern).metadata_partition).leader as zend_long,
    );
    add_assoc_long_ex(
        &mut ary,
        b"replica_cnt\0" as *const u8 as *const libc::c_char,
        strlen(b"replica_cnt\0" as *const u8 as *const libc::c_char),
        (*(*intern).metadata_partition).replica_cnt as zend_long,
    );
    add_assoc_long_ex(
        &mut ary,
        b"isr_cnt\0" as *const u8 as *const libc::c_char,
        strlen(b"isr_cnt\0" as *const u8 as *const libc::c_char),
        (*(*intern).metadata_partition).isr_cnt as zend_long,
    );
    return ary.value.arr;
}
pub unsafe extern "C" fn zim_RdKafka_Metadata_Partition_getId(
    mut execute_data: *mut zend_execute_data,
    mut return_value: *mut zval,
) {
    let mut intern: *mut object_intern = 0 as *mut object_intern;
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
    intern = get_object(
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
    (*__z).value.lval = (*(*intern).metadata_partition).id as zend_long;
    (*__z).u1.type_info = 4 as libc::c_int as uint32_t;
}
pub unsafe extern "C" fn zim_RdKafka_Metadata_Partition_getErr(
    mut execute_data: *mut zend_execute_data,
    mut return_value: *mut zval,
) {
    let mut intern: *mut object_intern = 0 as *mut object_intern;
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
    intern = get_object(
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
    (*__z).value.lval = (*(*intern).metadata_partition).err as zend_long;
    (*__z).u1.type_info = 4 as libc::c_int as uint32_t;
}
pub unsafe extern "C" fn zim_RdKafka_Metadata_Partition_getLeader(
    mut execute_data: *mut zend_execute_data,
    mut return_value: *mut zval,
) {
    let mut intern: *mut object_intern = 0 as *mut object_intern;
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
    intern = get_object(
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
    (*__z).value.lval = (*(*intern).metadata_partition).leader as zend_long;
    (*__z).u1.type_info = 4 as libc::c_int as uint32_t;
}
pub unsafe extern "C" fn int32_ctor(
    mut return_value: *mut zval,
    mut zmetadata: *mut zval,
    mut data: *const libc::c_void,
) {
    let mut __z: *mut zval = return_value;
    (*__z).value.lval = *(data as *mut int32_t) as zend_long;
    (*__z).u1.type_info = 4 as libc::c_int as uint32_t;
}
pub unsafe extern "C" fn zim_RdKafka_Metadata_Partition_getReplicas(
    mut execute_data: *mut zend_execute_data,
    mut return_value: *mut zval,
) {
    let mut intern: *mut object_intern = 0 as *mut object_intern;
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
    intern = get_object(
        if zval_get_type(&mut (*execute_data).This) as libc::c_int == 8 as libc::c_int {
            &mut (*execute_data).This
        } else {
            0 as *mut zval
        },
    );
    if intern.is_null() {
        return;
    }
    kafka_metadata_collection_init(
        return_value,
        if zval_get_type(&mut (*execute_data).This) as libc::c_int == 8 as libc::c_int {
            &mut (*execute_data).This
        } else {
            0 as *mut zval
        },
        (*(*intern).metadata_partition).replicas as *const libc::c_void,
        (*(*intern).metadata_partition).replica_cnt as size_t,
        ::std::mem::size_of::<int32_t>() as libc::c_ulong,
        Some(
            int32_ctor
                as unsafe extern "C" fn(*mut zval, *mut zval, *const libc::c_void) -> (),
        ),
    );
}
pub unsafe extern "C" fn zim_RdKafka_Metadata_Partition_getIsrs(
    mut execute_data: *mut zend_execute_data,
    mut return_value: *mut zval,
) {
    let mut intern: *mut object_intern = 0 as *mut object_intern;
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
    intern = get_object(
        if zval_get_type(&mut (*execute_data).This) as libc::c_int == 8 as libc::c_int {
            &mut (*execute_data).This
        } else {
            0 as *mut zval
        },
    );
    if intern.is_null() {
        return;
    }
    kafka_metadata_collection_init(
        return_value,
        if zval_get_type(&mut (*execute_data).This) as libc::c_int == 8 as libc::c_int {
            &mut (*execute_data).This
        } else {
            0 as *mut zval
        },
        (*(*intern).metadata_partition).isrs as *const libc::c_void,
        (*(*intern).metadata_partition).isr_cnt as size_t,
        ::std::mem::size_of::<int32_t>() as libc::c_ulong,
        Some(
            int32_ctor
                as unsafe extern "C" fn(*mut zval, *mut zval, *const libc::c_void) -> (),
        ),
    );
}
pub unsafe extern "C" fn kafka_metadata_partition_minit(
    mut type_0: libc::c_int,
    mut module_number: libc::c_int,
) {
    ce = register_class_RdKafka_Metadata_Partition();
    (*ce)
        .c2rust_unnamed_0
        .create_object = Some(
        create_object as unsafe extern "C" fn(*mut zend_class_entry) -> *mut zend_object,
    );
    handlers = kafka_default_object_handlers;
    handlers
        .get_debug_info = Some(
        get_debug_info
            as unsafe extern "C" fn(*mut zval, *mut libc::c_int) -> *mut HashTable,
    );
    handlers
        .free_obj = Some(free_object as unsafe extern "C" fn(*mut zend_object) -> ());
    handlers.offset = 24 as libc::c_ulong as libc::c_int;
}
pub unsafe extern "C" fn kafka_metadata_partition_ctor(
    mut return_value: *mut zval,
    mut zmetadata: *mut zval,
    mut data: *const libc::c_void,
) {
    let mut metadata_partition: *mut rd_kafka_metadata_partition_t = data
        as *mut rd_kafka_metadata_partition_t;
    let mut intern: *mut object_intern = 0 as *mut object_intern;
    if object_init_ex(return_value, ce) != SUCCESS as libc::c_int {
        return;
    }
    intern = ((*return_value).value.obj as *mut libc::c_char)
        .offset(-(24 as libc::c_ulong as isize)) as *mut object_intern;
    if intern.is_null() {
        return;
    }
    let mut __z: *mut zval = &mut (*intern).zmetadata;
    let mut __zv: *mut zval = zmetadata;
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
    (*intern).metadata_partition = metadata_partition;
}
unsafe extern "C" fn run_static_initializers() {
    class_RdKafka_Metadata_Partition_methods = [
        {
            let mut init = _zend_function_entry {
                fname: b"__construct\0" as *const u8 as *const libc::c_char,
                handler: Some(
                    zim_RdKafka___construct
                        as unsafe extern "C" fn(*mut zend_execute_data, *mut zval) -> (),
                ),
                arg_info: arginfo_class_RdKafka_Metadata_Partition___construct.as_ptr(),
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
                fname: b"getId\0" as *const u8 as *const libc::c_char,
                handler: Some(
                    zim_RdKafka_Metadata_Partition_getId
                        as unsafe extern "C" fn(*mut zend_execute_data, *mut zval) -> (),
                ),
                arg_info: arginfo_class_RdKafka_Metadata_Partition_getId.as_ptr(),
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
                fname: b"getErr\0" as *const u8 as *const libc::c_char,
                handler: Some(
                    zim_RdKafka_Metadata_Partition_getErr
                        as unsafe extern "C" fn(*mut zend_execute_data, *mut zval) -> (),
                ),
                arg_info: arginfo_class_RdKafka_Metadata_Partition_getId.as_ptr(),
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
                fname: b"getLeader\0" as *const u8 as *const libc::c_char,
                handler: Some(
                    zim_RdKafka_Metadata_Partition_getLeader
                        as unsafe extern "C" fn(*mut zend_execute_data, *mut zval) -> (),
                ),
                arg_info: arginfo_class_RdKafka_Metadata_Partition_getId.as_ptr(),
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
                fname: b"getReplicas\0" as *const u8 as *const libc::c_char,
                handler: Some(
                    zim_RdKafka_Metadata_Partition_getReplicas
                        as unsafe extern "C" fn(*mut zend_execute_data, *mut zval) -> (),
                ),
                arg_info: arginfo_class_RdKafka_Metadata_Partition_getId.as_ptr(),
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
                fname: b"getIsrs\0" as *const u8 as *const libc::c_char,
                handler: Some(
                    zim_RdKafka_Metadata_Partition_getIsrs
                        as unsafe extern "C" fn(*mut zend_execute_data, *mut zval) -> (),
                ),
                arg_info: arginfo_class_RdKafka_Metadata_Partition_getId.as_ptr(),
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
}
#[used]
#[cfg_attr(target_os = "linux", link_section = ".init_array")]
#[cfg_attr(target_os = "windows", link_section = ".CRT$XIB")]
#[cfg_attr(target_os = "macos", link_section = "__DATA,__mod_init_func")]
static INIT_ARRAY: [unsafe extern "C" fn(); 1] = [run_static_initializers];
