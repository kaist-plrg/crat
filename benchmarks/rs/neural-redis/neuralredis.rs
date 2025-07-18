use ::libc;
extern "C" {
    pub type RedisModuleCtx;
    pub type RedisModuleKey;
    pub type RedisModuleString;
    pub type RedisModuleCallReply;
    pub type RedisModuleIO;
    pub type RedisModuleType;
    pub type RedisModuleDigest;
    pub type RedisModuleBlockedClient;
    fn snprintf(
        _: *mut libc::c_char,
        _: libc::c_ulong,
        _: *const libc::c_char,
        _: ...
    ) -> libc::c_int;
    fn rand() -> libc::c_int;
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
    fn strcmp(_: *const libc::c_char, _: *const libc::c_char) -> libc::c_int;
    fn strcasecmp(__s1: *const libc::c_char, __s2: *const libc::c_char) -> libc::c_int;
    fn pthread_create(
        __newthread: *mut pthread_t,
        __attr: *const pthread_attr_t,
        __start_routine: Option::<
            unsafe extern "C" fn(*mut libc::c_void) -> *mut libc::c_void,
        >,
        __arg: *mut libc::c_void,
    ) -> libc::c_int;
    fn pthread_mutex_lock(__mutex: *mut pthread_mutex_t) -> libc::c_int;
    fn pthread_mutex_unlock(__mutex: *mut pthread_mutex_t) -> libc::c_int;
    fn gettimeofday(__tv: *mut timeval, __tz: *mut libc::c_void) -> libc::c_int;
    fn fabs(_: libc::c_double) -> libc::c_double;
    fn AnnFree(net: *mut Ann);
    fn AnnCreateNet(layers: libc::c_int, units: *mut libc::c_int) -> *mut Ann;
    fn AnnClone(net: *mut Ann) -> *mut Ann;
    fn AnnCountWeights(net: *mut Ann) -> size_t;
    fn AnnSimulate(net: *mut Ann);
    fn AnnSetRandomWeights(net: *mut Ann);
    fn AnnTrain(
        net: *mut Ann,
        input: *mut libc::c_float,
        desidered: *mut libc::c_float,
        maxerr: libc::c_float,
        maxepochs: libc::c_int,
        setlen: libc::c_int,
        algo: libc::c_int,
    ) -> libc::c_float;
    fn AnnTestError(
        net: *mut Ann,
        input: *mut libc::c_float,
        desired: *mut libc::c_float,
        setlen: libc::c_int,
        avgerr: *mut libc::c_float,
        classerr: *mut libc::c_float,
    );
}
pub type __uint32_t = libc::c_uint;
pub type __int64_t = libc::c_long;
pub type __uint64_t = libc::c_ulong;
pub type __time_t = libc::c_long;
pub type __suseconds_t = libc::c_long;
pub type size_t = libc::c_ulong;
pub type int64_t = __int64_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct timeval {
    pub tv_sec: __time_t,
    pub tv_usec: __suseconds_t,
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
pub type pthread_t = libc::c_ulong;
#[derive(Copy, Clone)]
#[repr(C)]
pub union pthread_attr_t {
    pub __size: [libc::c_char; 56],
    pub __align: libc::c_long,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union pthread_mutex_t {
    pub __data: __pthread_mutex_s,
    pub __size: [libc::c_char; 40],
    pub __align: libc::c_long,
}
pub type uint32_t = __uint32_t;
pub type uint64_t = __uint64_t;
pub type mstime_t = libc::c_longlong;
pub type RedisModuleCmdFunc = Option::<
    unsafe extern "C" fn(
        *mut RedisModuleCtx,
        *mut *mut RedisModuleString,
        libc::c_int,
    ) -> libc::c_int,
>;
pub type RedisModuleTypeLoadFunc = Option::<
    unsafe extern "C" fn(*mut RedisModuleIO, libc::c_int) -> *mut libc::c_void,
>;
pub type RedisModuleTypeSaveFunc = Option::<
    unsafe extern "C" fn(*mut RedisModuleIO, *mut libc::c_void) -> (),
>;
pub type RedisModuleTypeRewriteFunc = Option::<
    unsafe extern "C" fn(
        *mut RedisModuleIO,
        *mut RedisModuleString,
        *mut libc::c_void,
    ) -> (),
>;
pub type RedisModuleTypeMemUsageFunc = Option::<
    unsafe extern "C" fn(*mut libc::c_void) -> size_t,
>;
pub type RedisModuleTypeDigestFunc = Option::<
    unsafe extern "C" fn(*mut RedisModuleDigest, *mut libc::c_void) -> (),
>;
pub type RedisModuleTypeFreeFunc = Option::<
    unsafe extern "C" fn(*mut libc::c_void) -> (),
>;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct RedisModuleTypeMethods {
    pub version: uint64_t,
    pub rdb_load: RedisModuleTypeLoadFunc,
    pub rdb_save: RedisModuleTypeSaveFunc,
    pub aof_rewrite: RedisModuleTypeRewriteFunc,
    pub mem_usage: RedisModuleTypeMemUsageFunc,
    pub digest: RedisModuleTypeDigestFunc,
    pub free: RedisModuleTypeFreeFunc,
}
pub type C2RustUnnamed = libc::c_uint;
pub const PTHREAD_MUTEX_DEFAULT: C2RustUnnamed = 0;
pub const PTHREAD_MUTEX_ERRORCHECK: C2RustUnnamed = 2;
pub const PTHREAD_MUTEX_RECURSIVE: C2RustUnnamed = 1;
pub const PTHREAD_MUTEX_NORMAL: C2RustUnnamed = 0;
pub const PTHREAD_MUTEX_ADAPTIVE_NP: C2RustUnnamed = 3;
pub const PTHREAD_MUTEX_ERRORCHECK_NP: C2RustUnnamed = 2;
pub const PTHREAD_MUTEX_RECURSIVE_NP: C2RustUnnamed = 1;
pub const PTHREAD_MUTEX_TIMED_NP: C2RustUnnamed = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct AnnLayer {
    pub output: *mut libc::c_float,
    pub error: *mut libc::c_float,
    pub weight: *mut libc::c_float,
    pub gradient: *mut libc::c_float,
    pub sgradient: *mut libc::c_float,
    pub pgradient: *mut libc::c_float,
    pub delta: *mut libc::c_float,
    pub units: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct Ann {
    pub flags: libc::c_int,
    pub layers: libc::c_int,
    pub rprop_nminus: libc::c_float,
    pub rprop_nplus: libc::c_float,
    pub rprop_maxupdate: libc::c_float,
    pub rprop_minupdate: libc::c_float,
    pub learn_rate: libc::c_float,
    pub _filler_: libc::c_float,
    pub layer: *mut AnnLayer,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct NRDataset {
    pub len: uint32_t,
    pub maxlen: uint32_t,
    pub inputs: *mut libc::c_float,
    pub outputs: *mut libc::c_float,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct NRTypeObject {
    pub id: uint64_t,
    pub training_total_steps: uint64_t,
    pub training_total_ms: uint64_t,
    pub training_max_cycles: uint64_t,
    pub training_max_ms: uint64_t,
    pub flags: uint32_t,
    pub epochs: uint32_t,
    pub nn: *mut Ann,
    pub dataset: NRDataset,
    pub test: NRDataset,
    pub dataset_error: libc::c_float,
    pub test_error: libc::c_float,
    pub test_class_error: libc::c_float,
    pub inorm: *mut libc::c_float,
    pub onorm: *mut libc::c_float,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct NRPendingTraining {
    pub key: *mut RedisModuleString,
    pub db_id: libc::c_int,
    pub tid: pthread_t,
    pub in_progress: libc::c_int,
    pub nr: *mut NRTypeObject,
    pub dataset_error: libc::c_float,
    pub test_error: libc::c_float,
    pub class_error: libc::c_float,
    pub curcycle: libc::c_int,
}
pub static mut RedisModule_Alloc: Option::<
    unsafe extern "C" fn(size_t) -> *mut libc::c_void,
> = None;
pub static mut RedisModule_Realloc: Option::<
    unsafe extern "C" fn(*mut libc::c_void, size_t) -> *mut libc::c_void,
> = None;
pub static mut RedisModule_Free: Option::<
    unsafe extern "C" fn(*mut libc::c_void) -> (),
> = None;
pub static mut RedisModule_Calloc: Option::<
    unsafe extern "C" fn(size_t, size_t) -> *mut libc::c_void,
> = None;
pub static mut RedisModule_Strdup: Option::<
    unsafe extern "C" fn(*const libc::c_char) -> *mut libc::c_char,
> = None;
pub static mut RedisModule_GetApi: Option::<
    unsafe extern "C" fn(*const libc::c_char, *mut libc::c_void) -> libc::c_int,
> = None;
pub static mut RedisModule_CreateCommand: Option::<
    unsafe extern "C" fn(
        *mut RedisModuleCtx,
        *const libc::c_char,
        RedisModuleCmdFunc,
        *const libc::c_char,
        libc::c_int,
        libc::c_int,
        libc::c_int,
    ) -> libc::c_int,
> = None;
pub static mut RedisModule_SetModuleAttribs: Option::<
    unsafe extern "C" fn(
        *mut RedisModuleCtx,
        *const libc::c_char,
        libc::c_int,
        libc::c_int,
    ) -> libc::c_int,
> = None;
pub static mut RedisModule_WrongArity: Option::<
    unsafe extern "C" fn(*mut RedisModuleCtx) -> libc::c_int,
> = None;
pub static mut RedisModule_ReplyWithLongLong: Option::<
    unsafe extern "C" fn(*mut RedisModuleCtx, libc::c_longlong) -> libc::c_int,
> = None;
pub static mut RedisModule_GetSelectedDb: Option::<
    unsafe extern "C" fn(*mut RedisModuleCtx) -> libc::c_int,
> = None;
pub static mut RedisModule_SelectDb: Option::<
    unsafe extern "C" fn(*mut RedisModuleCtx, libc::c_int) -> libc::c_int,
> = None;
pub static mut RedisModule_OpenKey: Option::<
    unsafe extern "C" fn(
        *mut RedisModuleCtx,
        *mut RedisModuleString,
        libc::c_int,
    ) -> *mut libc::c_void,
> = None;
pub static mut RedisModule_CloseKey: Option::<
    unsafe extern "C" fn(*mut RedisModuleKey) -> (),
> = None;
pub static mut RedisModule_KeyType: Option::<
    unsafe extern "C" fn(*mut RedisModuleKey) -> libc::c_int,
> = None;
pub static mut RedisModule_ValueLength: Option::<
    unsafe extern "C" fn(*mut RedisModuleKey) -> size_t,
> = None;
pub static mut RedisModule_ListPush: Option::<
    unsafe extern "C" fn(
        *mut RedisModuleKey,
        libc::c_int,
        *mut RedisModuleString,
    ) -> libc::c_int,
> = None;
pub static mut RedisModule_ListPop: Option::<
    unsafe extern "C" fn(*mut RedisModuleKey, libc::c_int) -> *mut RedisModuleString,
> = None;
pub static mut RedisModule_Call: Option::<
    unsafe extern "C" fn(
        *mut RedisModuleCtx,
        *const libc::c_char,
        *const libc::c_char,
        ...
    ) -> *mut RedisModuleCallReply,
> = None;
pub static mut RedisModule_CallReplyProto: Option::<
    unsafe extern "C" fn(*mut RedisModuleCallReply, *mut size_t) -> *const libc::c_char,
> = None;
pub static mut RedisModule_FreeCallReply: Option::<
    unsafe extern "C" fn(*mut RedisModuleCallReply) -> (),
> = None;
pub static mut RedisModule_CallReplyType: Option::<
    unsafe extern "C" fn(*mut RedisModuleCallReply) -> libc::c_int,
> = None;
pub static mut RedisModule_CallReplyInteger: Option::<
    unsafe extern "C" fn(*mut RedisModuleCallReply) -> libc::c_longlong,
> = None;
pub static mut RedisModule_CallReplyLength: Option::<
    unsafe extern "C" fn(*mut RedisModuleCallReply) -> size_t,
> = None;
pub static mut RedisModule_CallReplyArrayElement: Option::<
    unsafe extern "C" fn(*mut RedisModuleCallReply, size_t) -> *mut RedisModuleCallReply,
> = None;
pub static mut RedisModule_ZsetScore: Option::<
    unsafe extern "C" fn(
        *mut RedisModuleKey,
        *mut RedisModuleString,
        *mut libc::c_double,
    ) -> libc::c_int,
> = None;
pub static mut RedisModule_ZsetRangeStop: Option::<
    unsafe extern "C" fn(*mut RedisModuleKey) -> (),
> = None;
pub static mut RedisModule_ZsetRem: Option::<
    unsafe extern "C" fn(
        *mut RedisModuleKey,
        *mut RedisModuleString,
        *mut libc::c_int,
    ) -> libc::c_int,
> = None;
pub static mut RedisModule_CreateString: Option::<
    unsafe extern "C" fn(
        *mut RedisModuleCtx,
        *const libc::c_char,
        size_t,
    ) -> *mut RedisModuleString,
> = None;
pub static mut RedisModule_CreateStringFromLongLong: Option::<
    unsafe extern "C" fn(*mut RedisModuleCtx, libc::c_longlong) -> *mut RedisModuleString,
> = None;
pub static mut RedisModule_CreateStringFromString: Option::<
    unsafe extern "C" fn(
        *mut RedisModuleCtx,
        *const RedisModuleString,
    ) -> *mut RedisModuleString,
> = None;
pub static mut RedisModule_CreateStringPrintf: Option::<
    unsafe extern "C" fn(
        *mut RedisModuleCtx,
        *const libc::c_char,
        ...
    ) -> *mut RedisModuleString,
> = None;
pub static mut RedisModule_FreeString: Option::<
    unsafe extern "C" fn(*mut RedisModuleCtx, *mut RedisModuleString) -> (),
> = None;
pub static mut RedisModule_StringPtrLen: Option::<
    unsafe extern "C" fn(*const RedisModuleString, *mut size_t) -> *const libc::c_char,
> = None;
pub static mut RedisModule_ReplyWithError: Option::<
    unsafe extern "C" fn(*mut RedisModuleCtx, *const libc::c_char) -> libc::c_int,
> = None;
pub static mut RedisModule_ReplyWithSimpleString: Option::<
    unsafe extern "C" fn(*mut RedisModuleCtx, *const libc::c_char) -> libc::c_int,
> = None;
pub static mut RedisModule_ReplyWithArray: Option::<
    unsafe extern "C" fn(*mut RedisModuleCtx, libc::c_long) -> libc::c_int,
> = None;
pub static mut RedisModule_ReplySetArrayLength: Option::<
    unsafe extern "C" fn(*mut RedisModuleCtx, libc::c_long) -> (),
> = None;
pub static mut RedisModule_ReplyWithStringBuffer: Option::<
    unsafe extern "C" fn(*mut RedisModuleCtx, *const libc::c_char, size_t) -> libc::c_int,
> = None;
pub static mut RedisModule_ReplyWithString: Option::<
    unsafe extern "C" fn(*mut RedisModuleCtx, *mut RedisModuleString) -> libc::c_int,
> = None;
pub static mut RedisModule_ReplyWithNull: Option::<
    unsafe extern "C" fn(*mut RedisModuleCtx) -> libc::c_int,
> = None;
pub static mut RedisModule_ReplyWithDouble: Option::<
    unsafe extern "C" fn(*mut RedisModuleCtx, libc::c_double) -> libc::c_int,
> = None;
pub static mut RedisModule_ReplyWithCallReply: Option::<
    unsafe extern "C" fn(*mut RedisModuleCtx, *mut RedisModuleCallReply) -> libc::c_int,
> = None;
pub static mut RedisModule_StringToLongLong: Option::<
    unsafe extern "C" fn(*const RedisModuleString, *mut libc::c_longlong) -> libc::c_int,
> = None;
pub static mut RedisModule_StringToDouble: Option::<
    unsafe extern "C" fn(*const RedisModuleString, *mut libc::c_double) -> libc::c_int,
> = None;
pub static mut RedisModule_AutoMemory: Option::<
    unsafe extern "C" fn(*mut RedisModuleCtx) -> (),
> = None;
pub static mut RedisModule_Replicate: Option::<
    unsafe extern "C" fn(
        *mut RedisModuleCtx,
        *const libc::c_char,
        *const libc::c_char,
        ...
    ) -> libc::c_int,
> = None;
pub static mut RedisModule_ReplicateVerbatim: Option::<
    unsafe extern "C" fn(*mut RedisModuleCtx) -> libc::c_int,
> = None;
pub static mut RedisModule_CallReplyStringPtr: Option::<
    unsafe extern "C" fn(*mut RedisModuleCallReply, *mut size_t) -> *const libc::c_char,
> = None;
pub static mut RedisModule_CreateStringFromCallReply: Option::<
    unsafe extern "C" fn(*mut RedisModuleCallReply) -> *mut RedisModuleString,
> = None;
pub static mut RedisModule_DeleteKey: Option::<
    unsafe extern "C" fn(*mut RedisModuleKey) -> libc::c_int,
> = None;
pub static mut RedisModule_StringSet: Option::<
    unsafe extern "C" fn(*mut RedisModuleKey, *mut RedisModuleString) -> libc::c_int,
> = None;
pub static mut RedisModule_StringDMA: Option::<
    unsafe extern "C" fn(
        *mut RedisModuleKey,
        *mut size_t,
        libc::c_int,
    ) -> *mut libc::c_char,
> = None;
pub static mut RedisModule_StringTruncate: Option::<
    unsafe extern "C" fn(*mut RedisModuleKey, size_t) -> libc::c_int,
> = None;
pub static mut RedisModule_GetExpire: Option::<
    unsafe extern "C" fn(*mut RedisModuleKey) -> mstime_t,
> = None;
pub static mut RedisModule_SetExpire: Option::<
    unsafe extern "C" fn(*mut RedisModuleKey, mstime_t) -> libc::c_int,
> = None;
pub static mut RedisModule_ZsetAdd: Option::<
    unsafe extern "C" fn(
        *mut RedisModuleKey,
        libc::c_double,
        *mut RedisModuleString,
        *mut libc::c_int,
    ) -> libc::c_int,
> = None;
pub static mut RedisModule_ZsetIncrby: Option::<
    unsafe extern "C" fn(
        *mut RedisModuleKey,
        libc::c_double,
        *mut RedisModuleString,
        *mut libc::c_int,
        *mut libc::c_double,
    ) -> libc::c_int,
> = None;
pub static mut RedisModule_ZsetFirstInScoreRange: Option::<
    unsafe extern "C" fn(
        *mut RedisModuleKey,
        libc::c_double,
        libc::c_double,
        libc::c_int,
        libc::c_int,
    ) -> libc::c_int,
> = None;
pub static mut RedisModule_ZsetLastInScoreRange: Option::<
    unsafe extern "C" fn(
        *mut RedisModuleKey,
        libc::c_double,
        libc::c_double,
        libc::c_int,
        libc::c_int,
    ) -> libc::c_int,
> = None;
pub static mut RedisModule_ZsetFirstInLexRange: Option::<
    unsafe extern "C" fn(
        *mut RedisModuleKey,
        *mut RedisModuleString,
        *mut RedisModuleString,
    ) -> libc::c_int,
> = None;
pub static mut RedisModule_ZsetLastInLexRange: Option::<
    unsafe extern "C" fn(
        *mut RedisModuleKey,
        *mut RedisModuleString,
        *mut RedisModuleString,
    ) -> libc::c_int,
> = None;
pub static mut RedisModule_ZsetRangeCurrentElement: Option::<
    unsafe extern "C" fn(
        *mut RedisModuleKey,
        *mut libc::c_double,
    ) -> *mut RedisModuleString,
> = None;
pub static mut RedisModule_ZsetRangeNext: Option::<
    unsafe extern "C" fn(*mut RedisModuleKey) -> libc::c_int,
> = None;
pub static mut RedisModule_ZsetRangePrev: Option::<
    unsafe extern "C" fn(*mut RedisModuleKey) -> libc::c_int,
> = None;
pub static mut RedisModule_ZsetRangeEndReached: Option::<
    unsafe extern "C" fn(*mut RedisModuleKey) -> libc::c_int,
> = None;
pub static mut RedisModule_HashSet: Option::<
    unsafe extern "C" fn(*mut RedisModuleKey, libc::c_int, ...) -> libc::c_int,
> = None;
pub static mut RedisModule_HashGet: Option::<
    unsafe extern "C" fn(*mut RedisModuleKey, libc::c_int, ...) -> libc::c_int,
> = None;
pub static mut RedisModule_IsKeysPositionRequest: Option::<
    unsafe extern "C" fn(*mut RedisModuleCtx) -> libc::c_int,
> = None;
pub static mut RedisModule_KeyAtPos: Option::<
    unsafe extern "C" fn(*mut RedisModuleCtx, libc::c_int) -> (),
> = None;
pub static mut RedisModule_GetClientId: Option::<
    unsafe extern "C" fn(*mut RedisModuleCtx) -> libc::c_ulonglong,
> = None;
pub static mut RedisModule_PoolAlloc: Option::<
    unsafe extern "C" fn(*mut RedisModuleCtx, size_t) -> *mut libc::c_void,
> = None;
pub static mut RedisModule_CreateDataType: Option::<
    unsafe extern "C" fn(
        *mut RedisModuleCtx,
        *const libc::c_char,
        libc::c_int,
        *mut RedisModuleTypeMethods,
    ) -> *mut RedisModuleType,
> = None;
pub static mut RedisModule_ModuleTypeSetValue: Option::<
    unsafe extern "C" fn(
        *mut RedisModuleKey,
        *mut RedisModuleType,
        *mut libc::c_void,
    ) -> libc::c_int,
> = None;
pub static mut RedisModule_ModuleTypeGetType: Option::<
    unsafe extern "C" fn(*mut RedisModuleKey) -> *mut RedisModuleType,
> = None;
pub static mut RedisModule_ModuleTypeGetValue: Option::<
    unsafe extern "C" fn(*mut RedisModuleKey) -> *mut libc::c_void,
> = None;
pub static mut RedisModule_SaveUnsigned: Option::<
    unsafe extern "C" fn(*mut RedisModuleIO, uint64_t) -> (),
> = None;
pub static mut RedisModule_LoadUnsigned: Option::<
    unsafe extern "C" fn(*mut RedisModuleIO) -> uint64_t,
> = None;
pub static mut RedisModule_SaveSigned: Option::<
    unsafe extern "C" fn(*mut RedisModuleIO, int64_t) -> (),
> = None;
pub static mut RedisModule_LoadSigned: Option::<
    unsafe extern "C" fn(*mut RedisModuleIO) -> int64_t,
> = None;
pub static mut RedisModule_EmitAOF: Option::<
    unsafe extern "C" fn(
        *mut RedisModuleIO,
        *const libc::c_char,
        *const libc::c_char,
        ...
    ) -> (),
> = None;
pub static mut RedisModule_SaveString: Option::<
    unsafe extern "C" fn(*mut RedisModuleIO, *mut RedisModuleString) -> (),
> = None;
pub static mut RedisModule_SaveStringBuffer: Option::<
    unsafe extern "C" fn(*mut RedisModuleIO, *const libc::c_char, size_t) -> (),
> = None;
pub static mut RedisModule_LoadString: Option::<
    unsafe extern "C" fn(*mut RedisModuleIO) -> *mut RedisModuleString,
> = None;
pub static mut RedisModule_LoadStringBuffer: Option::<
    unsafe extern "C" fn(*mut RedisModuleIO, *mut size_t) -> *mut libc::c_char,
> = None;
pub static mut RedisModule_SaveDouble: Option::<
    unsafe extern "C" fn(*mut RedisModuleIO, libc::c_double) -> (),
> = None;
pub static mut RedisModule_LoadDouble: Option::<
    unsafe extern "C" fn(*mut RedisModuleIO) -> libc::c_double,
> = None;
pub static mut RedisModule_SaveFloat: Option::<
    unsafe extern "C" fn(*mut RedisModuleIO, libc::c_float) -> (),
> = None;
pub static mut RedisModule_LoadFloat: Option::<
    unsafe extern "C" fn(*mut RedisModuleIO) -> libc::c_float,
> = None;
pub static mut RedisModule_Log: Option::<
    unsafe extern "C" fn(
        *mut RedisModuleCtx,
        *const libc::c_char,
        *const libc::c_char,
        ...
    ) -> (),
> = None;
pub static mut RedisModule_LogIOError: Option::<
    unsafe extern "C" fn(
        *mut RedisModuleIO,
        *const libc::c_char,
        *const libc::c_char,
        ...
    ) -> (),
> = None;
pub static mut RedisModule_StringAppendBuffer: Option::<
    unsafe extern "C" fn(
        *mut RedisModuleCtx,
        *mut RedisModuleString,
        *const libc::c_char,
        size_t,
    ) -> libc::c_int,
> = None;
pub static mut RedisModule_RetainString: Option::<
    unsafe extern "C" fn(*mut RedisModuleCtx, *mut RedisModuleString) -> (),
> = None;
pub static mut RedisModule_StringCompare: Option::<
    unsafe extern "C" fn(*mut RedisModuleString, *mut RedisModuleString) -> libc::c_int,
> = None;
pub static mut RedisModule_GetContextFromIO: Option::<
    unsafe extern "C" fn(*mut RedisModuleIO) -> *mut RedisModuleCtx,
> = None;
pub static mut RedisModule_BlockClient: Option::<
    unsafe extern "C" fn(
        *mut RedisModuleCtx,
        RedisModuleCmdFunc,
        RedisModuleCmdFunc,
        Option::<unsafe extern "C" fn(*mut libc::c_void) -> ()>,
        libc::c_longlong,
    ) -> *mut RedisModuleBlockedClient,
> = None;
pub static mut RedisModule_UnblockClient: Option::<
    unsafe extern "C" fn(*mut RedisModuleBlockedClient, *mut libc::c_void) -> libc::c_int,
> = None;
pub static mut RedisModule_IsBlockedReplyRequest: Option::<
    unsafe extern "C" fn(*mut RedisModuleCtx) -> libc::c_int,
> = None;
pub static mut RedisModule_IsBlockedTimeoutRequest: Option::<
    unsafe extern "C" fn(*mut RedisModuleCtx) -> libc::c_int,
> = None;
pub static mut RedisModule_GetBlockedClientPrivateData: Option::<
    unsafe extern "C" fn(*mut RedisModuleCtx) -> *mut libc::c_void,
> = None;
pub static mut RedisModule_AbortBlock: Option::<
    unsafe extern "C" fn(*mut RedisModuleBlockedClient) -> libc::c_int,
> = None;
pub static mut RedisModule_Milliseconds: Option::<
    unsafe extern "C" fn() -> libc::c_longlong,
> = None;
unsafe extern "C" fn RedisModule_Init(
    mut ctx: *mut RedisModuleCtx,
    mut name: *const libc::c_char,
    mut ver: libc::c_int,
    mut apiver: libc::c_int,
) -> libc::c_int {
    let mut getapifuncptr: *mut libc::c_void = *(ctx as *mut *mut libc::c_void)
        .offset(0 as libc::c_int as isize);
    RedisModule_GetApi = ::std::mem::transmute::<
        libc::intptr_t,
        Option::<
            unsafe extern "C" fn(*const libc::c_char, *mut libc::c_void) -> libc::c_int,
        >,
    >(getapifuncptr as libc::c_ulong as libc::intptr_t);
    RedisModule_GetApi
        .unwrap()(
        b"RedisModule_Alloc\0" as *const u8 as *const libc::c_char,
        &mut RedisModule_Alloc
            as *mut Option::<unsafe extern "C" fn(size_t) -> *mut libc::c_void>
            as *mut *mut libc::c_void as *mut libc::c_void,
    );
    RedisModule_GetApi
        .unwrap()(
        b"RedisModule_Calloc\0" as *const u8 as *const libc::c_char,
        &mut RedisModule_Calloc
            as *mut Option::<unsafe extern "C" fn(size_t, size_t) -> *mut libc::c_void>
            as *mut *mut libc::c_void as *mut libc::c_void,
    );
    RedisModule_GetApi
        .unwrap()(
        b"RedisModule_Free\0" as *const u8 as *const libc::c_char,
        &mut RedisModule_Free
            as *mut Option::<unsafe extern "C" fn(*mut libc::c_void) -> ()>
            as *mut *mut libc::c_void as *mut libc::c_void,
    );
    RedisModule_GetApi
        .unwrap()(
        b"RedisModule_Realloc\0" as *const u8 as *const libc::c_char,
        &mut RedisModule_Realloc
            as *mut Option::<
                unsafe extern "C" fn(*mut libc::c_void, size_t) -> *mut libc::c_void,
            > as *mut *mut libc::c_void as *mut libc::c_void,
    );
    RedisModule_GetApi
        .unwrap()(
        b"RedisModule_Strdup\0" as *const u8 as *const libc::c_char,
        &mut RedisModule_Strdup
            as *mut Option::<
                unsafe extern "C" fn(*const libc::c_char) -> *mut libc::c_char,
            > as *mut *mut libc::c_void as *mut libc::c_void,
    );
    RedisModule_GetApi
        .unwrap()(
        b"RedisModule_CreateCommand\0" as *const u8 as *const libc::c_char,
        &mut RedisModule_CreateCommand
            as *mut Option::<
                unsafe extern "C" fn(
                    *mut RedisModuleCtx,
                    *const libc::c_char,
                    RedisModuleCmdFunc,
                    *const libc::c_char,
                    libc::c_int,
                    libc::c_int,
                    libc::c_int,
                ) -> libc::c_int,
            > as *mut *mut libc::c_void as *mut libc::c_void,
    );
    RedisModule_GetApi
        .unwrap()(
        b"RedisModule_SetModuleAttribs\0" as *const u8 as *const libc::c_char,
        &mut RedisModule_SetModuleAttribs
            as *mut Option::<
                unsafe extern "C" fn(
                    *mut RedisModuleCtx,
                    *const libc::c_char,
                    libc::c_int,
                    libc::c_int,
                ) -> libc::c_int,
            > as *mut *mut libc::c_void as *mut libc::c_void,
    );
    RedisModule_GetApi
        .unwrap()(
        b"RedisModule_WrongArity\0" as *const u8 as *const libc::c_char,
        &mut RedisModule_WrongArity
            as *mut Option::<unsafe extern "C" fn(*mut RedisModuleCtx) -> libc::c_int>
            as *mut *mut libc::c_void as *mut libc::c_void,
    );
    RedisModule_GetApi
        .unwrap()(
        b"RedisModule_ReplyWithLongLong\0" as *const u8 as *const libc::c_char,
        &mut RedisModule_ReplyWithLongLong
            as *mut Option::<
                unsafe extern "C" fn(
                    *mut RedisModuleCtx,
                    libc::c_longlong,
                ) -> libc::c_int,
            > as *mut *mut libc::c_void as *mut libc::c_void,
    );
    RedisModule_GetApi
        .unwrap()(
        b"RedisModule_ReplyWithError\0" as *const u8 as *const libc::c_char,
        &mut RedisModule_ReplyWithError
            as *mut Option::<
                unsafe extern "C" fn(
                    *mut RedisModuleCtx,
                    *const libc::c_char,
                ) -> libc::c_int,
            > as *mut *mut libc::c_void as *mut libc::c_void,
    );
    RedisModule_GetApi
        .unwrap()(
        b"RedisModule_ReplyWithSimpleString\0" as *const u8 as *const libc::c_char,
        &mut RedisModule_ReplyWithSimpleString
            as *mut Option::<
                unsafe extern "C" fn(
                    *mut RedisModuleCtx,
                    *const libc::c_char,
                ) -> libc::c_int,
            > as *mut *mut libc::c_void as *mut libc::c_void,
    );
    RedisModule_GetApi
        .unwrap()(
        b"RedisModule_ReplyWithArray\0" as *const u8 as *const libc::c_char,
        &mut RedisModule_ReplyWithArray
            as *mut Option::<
                unsafe extern "C" fn(*mut RedisModuleCtx, libc::c_long) -> libc::c_int,
            > as *mut *mut libc::c_void as *mut libc::c_void,
    );
    RedisModule_GetApi
        .unwrap()(
        b"RedisModule_ReplySetArrayLength\0" as *const u8 as *const libc::c_char,
        &mut RedisModule_ReplySetArrayLength
            as *mut Option::<
                unsafe extern "C" fn(*mut RedisModuleCtx, libc::c_long) -> (),
            > as *mut *mut libc::c_void as *mut libc::c_void,
    );
    RedisModule_GetApi
        .unwrap()(
        b"RedisModule_ReplyWithStringBuffer\0" as *const u8 as *const libc::c_char,
        &mut RedisModule_ReplyWithStringBuffer
            as *mut Option::<
                unsafe extern "C" fn(
                    *mut RedisModuleCtx,
                    *const libc::c_char,
                    size_t,
                ) -> libc::c_int,
            > as *mut *mut libc::c_void as *mut libc::c_void,
    );
    RedisModule_GetApi
        .unwrap()(
        b"RedisModule_ReplyWithString\0" as *const u8 as *const libc::c_char,
        &mut RedisModule_ReplyWithString
            as *mut Option::<
                unsafe extern "C" fn(
                    *mut RedisModuleCtx,
                    *mut RedisModuleString,
                ) -> libc::c_int,
            > as *mut *mut libc::c_void as *mut libc::c_void,
    );
    RedisModule_GetApi
        .unwrap()(
        b"RedisModule_ReplyWithNull\0" as *const u8 as *const libc::c_char,
        &mut RedisModule_ReplyWithNull
            as *mut Option::<unsafe extern "C" fn(*mut RedisModuleCtx) -> libc::c_int>
            as *mut *mut libc::c_void as *mut libc::c_void,
    );
    RedisModule_GetApi
        .unwrap()(
        b"RedisModule_ReplyWithCallReply\0" as *const u8 as *const libc::c_char,
        &mut RedisModule_ReplyWithCallReply
            as *mut Option::<
                unsafe extern "C" fn(
                    *mut RedisModuleCtx,
                    *mut RedisModuleCallReply,
                ) -> libc::c_int,
            > as *mut *mut libc::c_void as *mut libc::c_void,
    );
    RedisModule_GetApi
        .unwrap()(
        b"RedisModule_ReplyWithDouble\0" as *const u8 as *const libc::c_char,
        &mut RedisModule_ReplyWithDouble
            as *mut Option::<
                unsafe extern "C" fn(*mut RedisModuleCtx, libc::c_double) -> libc::c_int,
            > as *mut *mut libc::c_void as *mut libc::c_void,
    );
    RedisModule_GetApi
        .unwrap()(
        b"RedisModule_ReplySetArrayLength\0" as *const u8 as *const libc::c_char,
        &mut RedisModule_ReplySetArrayLength
            as *mut Option::<
                unsafe extern "C" fn(*mut RedisModuleCtx, libc::c_long) -> (),
            > as *mut *mut libc::c_void as *mut libc::c_void,
    );
    RedisModule_GetApi
        .unwrap()(
        b"RedisModule_GetSelectedDb\0" as *const u8 as *const libc::c_char,
        &mut RedisModule_GetSelectedDb
            as *mut Option::<unsafe extern "C" fn(*mut RedisModuleCtx) -> libc::c_int>
            as *mut *mut libc::c_void as *mut libc::c_void,
    );
    RedisModule_GetApi
        .unwrap()(
        b"RedisModule_SelectDb\0" as *const u8 as *const libc::c_char,
        &mut RedisModule_SelectDb
            as *mut Option::<
                unsafe extern "C" fn(*mut RedisModuleCtx, libc::c_int) -> libc::c_int,
            > as *mut *mut libc::c_void as *mut libc::c_void,
    );
    RedisModule_GetApi
        .unwrap()(
        b"RedisModule_OpenKey\0" as *const u8 as *const libc::c_char,
        &mut RedisModule_OpenKey
            as *mut Option::<
                unsafe extern "C" fn(
                    *mut RedisModuleCtx,
                    *mut RedisModuleString,
                    libc::c_int,
                ) -> *mut libc::c_void,
            > as *mut *mut libc::c_void as *mut libc::c_void,
    );
    RedisModule_GetApi
        .unwrap()(
        b"RedisModule_CloseKey\0" as *const u8 as *const libc::c_char,
        &mut RedisModule_CloseKey
            as *mut Option::<unsafe extern "C" fn(*mut RedisModuleKey) -> ()>
            as *mut *mut libc::c_void as *mut libc::c_void,
    );
    RedisModule_GetApi
        .unwrap()(
        b"RedisModule_KeyType\0" as *const u8 as *const libc::c_char,
        &mut RedisModule_KeyType
            as *mut Option::<unsafe extern "C" fn(*mut RedisModuleKey) -> libc::c_int>
            as *mut *mut libc::c_void as *mut libc::c_void,
    );
    RedisModule_GetApi
        .unwrap()(
        b"RedisModule_ValueLength\0" as *const u8 as *const libc::c_char,
        &mut RedisModule_ValueLength
            as *mut Option::<unsafe extern "C" fn(*mut RedisModuleKey) -> size_t>
            as *mut *mut libc::c_void as *mut libc::c_void,
    );
    RedisModule_GetApi
        .unwrap()(
        b"RedisModule_ListPush\0" as *const u8 as *const libc::c_char,
        &mut RedisModule_ListPush
            as *mut Option::<
                unsafe extern "C" fn(
                    *mut RedisModuleKey,
                    libc::c_int,
                    *mut RedisModuleString,
                ) -> libc::c_int,
            > as *mut *mut libc::c_void as *mut libc::c_void,
    );
    RedisModule_GetApi
        .unwrap()(
        b"RedisModule_ListPop\0" as *const u8 as *const libc::c_char,
        &mut RedisModule_ListPop
            as *mut Option::<
                unsafe extern "C" fn(
                    *mut RedisModuleKey,
                    libc::c_int,
                ) -> *mut RedisModuleString,
            > as *mut *mut libc::c_void as *mut libc::c_void,
    );
    RedisModule_GetApi
        .unwrap()(
        b"RedisModule_StringToLongLong\0" as *const u8 as *const libc::c_char,
        &mut RedisModule_StringToLongLong
            as *mut Option::<
                unsafe extern "C" fn(
                    *const RedisModuleString,
                    *mut libc::c_longlong,
                ) -> libc::c_int,
            > as *mut *mut libc::c_void as *mut libc::c_void,
    );
    RedisModule_GetApi
        .unwrap()(
        b"RedisModule_StringToDouble\0" as *const u8 as *const libc::c_char,
        &mut RedisModule_StringToDouble
            as *mut Option::<
                unsafe extern "C" fn(
                    *const RedisModuleString,
                    *mut libc::c_double,
                ) -> libc::c_int,
            > as *mut *mut libc::c_void as *mut libc::c_void,
    );
    RedisModule_GetApi
        .unwrap()(
        b"RedisModule_Call\0" as *const u8 as *const libc::c_char,
        &mut RedisModule_Call
            as *mut Option::<
                unsafe extern "C" fn(
                    *mut RedisModuleCtx,
                    *const libc::c_char,
                    *const libc::c_char,
                    ...
                ) -> *mut RedisModuleCallReply,
            > as *mut *mut libc::c_void as *mut libc::c_void,
    );
    RedisModule_GetApi
        .unwrap()(
        b"RedisModule_CallReplyProto\0" as *const u8 as *const libc::c_char,
        &mut RedisModule_CallReplyProto
            as *mut Option::<
                unsafe extern "C" fn(
                    *mut RedisModuleCallReply,
                    *mut size_t,
                ) -> *const libc::c_char,
            > as *mut *mut libc::c_void as *mut libc::c_void,
    );
    RedisModule_GetApi
        .unwrap()(
        b"RedisModule_FreeCallReply\0" as *const u8 as *const libc::c_char,
        &mut RedisModule_FreeCallReply
            as *mut Option::<unsafe extern "C" fn(*mut RedisModuleCallReply) -> ()>
            as *mut *mut libc::c_void as *mut libc::c_void,
    );
    RedisModule_GetApi
        .unwrap()(
        b"RedisModule_CallReplyInteger\0" as *const u8 as *const libc::c_char,
        &mut RedisModule_CallReplyInteger
            as *mut Option::<
                unsafe extern "C" fn(*mut RedisModuleCallReply) -> libc::c_longlong,
            > as *mut *mut libc::c_void as *mut libc::c_void,
    );
    RedisModule_GetApi
        .unwrap()(
        b"RedisModule_CallReplyType\0" as *const u8 as *const libc::c_char,
        &mut RedisModule_CallReplyType
            as *mut Option::<
                unsafe extern "C" fn(*mut RedisModuleCallReply) -> libc::c_int,
            > as *mut *mut libc::c_void as *mut libc::c_void,
    );
    RedisModule_GetApi
        .unwrap()(
        b"RedisModule_CallReplyLength\0" as *const u8 as *const libc::c_char,
        &mut RedisModule_CallReplyLength
            as *mut Option::<unsafe extern "C" fn(*mut RedisModuleCallReply) -> size_t>
            as *mut *mut libc::c_void as *mut libc::c_void,
    );
    RedisModule_GetApi
        .unwrap()(
        b"RedisModule_CallReplyArrayElement\0" as *const u8 as *const libc::c_char,
        &mut RedisModule_CallReplyArrayElement
            as *mut Option::<
                unsafe extern "C" fn(
                    *mut RedisModuleCallReply,
                    size_t,
                ) -> *mut RedisModuleCallReply,
            > as *mut *mut libc::c_void as *mut libc::c_void,
    );
    RedisModule_GetApi
        .unwrap()(
        b"RedisModule_CallReplyStringPtr\0" as *const u8 as *const libc::c_char,
        &mut RedisModule_CallReplyStringPtr
            as *mut Option::<
                unsafe extern "C" fn(
                    *mut RedisModuleCallReply,
                    *mut size_t,
                ) -> *const libc::c_char,
            > as *mut *mut libc::c_void as *mut libc::c_void,
    );
    RedisModule_GetApi
        .unwrap()(
        b"RedisModule_CreateStringFromCallReply\0" as *const u8 as *const libc::c_char,
        &mut RedisModule_CreateStringFromCallReply
            as *mut Option::<
                unsafe extern "C" fn(*mut RedisModuleCallReply) -> *mut RedisModuleString,
            > as *mut *mut libc::c_void as *mut libc::c_void,
    );
    RedisModule_GetApi
        .unwrap()(
        b"RedisModule_CreateString\0" as *const u8 as *const libc::c_char,
        &mut RedisModule_CreateString
            as *mut Option::<
                unsafe extern "C" fn(
                    *mut RedisModuleCtx,
                    *const libc::c_char,
                    size_t,
                ) -> *mut RedisModuleString,
            > as *mut *mut libc::c_void as *mut libc::c_void,
    );
    RedisModule_GetApi
        .unwrap()(
        b"RedisModule_CreateStringFromLongLong\0" as *const u8 as *const libc::c_char,
        &mut RedisModule_CreateStringFromLongLong
            as *mut Option::<
                unsafe extern "C" fn(
                    *mut RedisModuleCtx,
                    libc::c_longlong,
                ) -> *mut RedisModuleString,
            > as *mut *mut libc::c_void as *mut libc::c_void,
    );
    RedisModule_GetApi
        .unwrap()(
        b"RedisModule_CreateStringFromString\0" as *const u8 as *const libc::c_char,
        &mut RedisModule_CreateStringFromString
            as *mut Option::<
                unsafe extern "C" fn(
                    *mut RedisModuleCtx,
                    *const RedisModuleString,
                ) -> *mut RedisModuleString,
            > as *mut *mut libc::c_void as *mut libc::c_void,
    );
    RedisModule_GetApi
        .unwrap()(
        b"RedisModule_CreateStringPrintf\0" as *const u8 as *const libc::c_char,
        &mut RedisModule_CreateStringPrintf
            as *mut Option::<
                unsafe extern "C" fn(
                    *mut RedisModuleCtx,
                    *const libc::c_char,
                    ...
                ) -> *mut RedisModuleString,
            > as *mut *mut libc::c_void as *mut libc::c_void,
    );
    RedisModule_GetApi
        .unwrap()(
        b"RedisModule_FreeString\0" as *const u8 as *const libc::c_char,
        &mut RedisModule_FreeString
            as *mut Option::<
                unsafe extern "C" fn(*mut RedisModuleCtx, *mut RedisModuleString) -> (),
            > as *mut *mut libc::c_void as *mut libc::c_void,
    );
    RedisModule_GetApi
        .unwrap()(
        b"RedisModule_StringPtrLen\0" as *const u8 as *const libc::c_char,
        &mut RedisModule_StringPtrLen
            as *mut Option::<
                unsafe extern "C" fn(
                    *const RedisModuleString,
                    *mut size_t,
                ) -> *const libc::c_char,
            > as *mut *mut libc::c_void as *mut libc::c_void,
    );
    RedisModule_GetApi
        .unwrap()(
        b"RedisModule_AutoMemory\0" as *const u8 as *const libc::c_char,
        &mut RedisModule_AutoMemory
            as *mut Option::<unsafe extern "C" fn(*mut RedisModuleCtx) -> ()>
            as *mut *mut libc::c_void as *mut libc::c_void,
    );
    RedisModule_GetApi
        .unwrap()(
        b"RedisModule_Replicate\0" as *const u8 as *const libc::c_char,
        &mut RedisModule_Replicate
            as *mut Option::<
                unsafe extern "C" fn(
                    *mut RedisModuleCtx,
                    *const libc::c_char,
                    *const libc::c_char,
                    ...
                ) -> libc::c_int,
            > as *mut *mut libc::c_void as *mut libc::c_void,
    );
    RedisModule_GetApi
        .unwrap()(
        b"RedisModule_ReplicateVerbatim\0" as *const u8 as *const libc::c_char,
        &mut RedisModule_ReplicateVerbatim
            as *mut Option::<unsafe extern "C" fn(*mut RedisModuleCtx) -> libc::c_int>
            as *mut *mut libc::c_void as *mut libc::c_void,
    );
    RedisModule_GetApi
        .unwrap()(
        b"RedisModule_DeleteKey\0" as *const u8 as *const libc::c_char,
        &mut RedisModule_DeleteKey
            as *mut Option::<unsafe extern "C" fn(*mut RedisModuleKey) -> libc::c_int>
            as *mut *mut libc::c_void as *mut libc::c_void,
    );
    RedisModule_GetApi
        .unwrap()(
        b"RedisModule_StringSet\0" as *const u8 as *const libc::c_char,
        &mut RedisModule_StringSet
            as *mut Option::<
                unsafe extern "C" fn(
                    *mut RedisModuleKey,
                    *mut RedisModuleString,
                ) -> libc::c_int,
            > as *mut *mut libc::c_void as *mut libc::c_void,
    );
    RedisModule_GetApi
        .unwrap()(
        b"RedisModule_StringDMA\0" as *const u8 as *const libc::c_char,
        &mut RedisModule_StringDMA
            as *mut Option::<
                unsafe extern "C" fn(
                    *mut RedisModuleKey,
                    *mut size_t,
                    libc::c_int,
                ) -> *mut libc::c_char,
            > as *mut *mut libc::c_void as *mut libc::c_void,
    );
    RedisModule_GetApi
        .unwrap()(
        b"RedisModule_StringTruncate\0" as *const u8 as *const libc::c_char,
        &mut RedisModule_StringTruncate
            as *mut Option::<
                unsafe extern "C" fn(*mut RedisModuleKey, size_t) -> libc::c_int,
            > as *mut *mut libc::c_void as *mut libc::c_void,
    );
    RedisModule_GetApi
        .unwrap()(
        b"RedisModule_GetExpire\0" as *const u8 as *const libc::c_char,
        &mut RedisModule_GetExpire
            as *mut Option::<unsafe extern "C" fn(*mut RedisModuleKey) -> mstime_t>
            as *mut *mut libc::c_void as *mut libc::c_void,
    );
    RedisModule_GetApi
        .unwrap()(
        b"RedisModule_SetExpire\0" as *const u8 as *const libc::c_char,
        &mut RedisModule_SetExpire
            as *mut Option::<
                unsafe extern "C" fn(*mut RedisModuleKey, mstime_t) -> libc::c_int,
            > as *mut *mut libc::c_void as *mut libc::c_void,
    );
    RedisModule_GetApi
        .unwrap()(
        b"RedisModule_ZsetAdd\0" as *const u8 as *const libc::c_char,
        &mut RedisModule_ZsetAdd
            as *mut Option::<
                unsafe extern "C" fn(
                    *mut RedisModuleKey,
                    libc::c_double,
                    *mut RedisModuleString,
                    *mut libc::c_int,
                ) -> libc::c_int,
            > as *mut *mut libc::c_void as *mut libc::c_void,
    );
    RedisModule_GetApi
        .unwrap()(
        b"RedisModule_ZsetIncrby\0" as *const u8 as *const libc::c_char,
        &mut RedisModule_ZsetIncrby
            as *mut Option::<
                unsafe extern "C" fn(
                    *mut RedisModuleKey,
                    libc::c_double,
                    *mut RedisModuleString,
                    *mut libc::c_int,
                    *mut libc::c_double,
                ) -> libc::c_int,
            > as *mut *mut libc::c_void as *mut libc::c_void,
    );
    RedisModule_GetApi
        .unwrap()(
        b"RedisModule_ZsetScore\0" as *const u8 as *const libc::c_char,
        &mut RedisModule_ZsetScore
            as *mut Option::<
                unsafe extern "C" fn(
                    *mut RedisModuleKey,
                    *mut RedisModuleString,
                    *mut libc::c_double,
                ) -> libc::c_int,
            > as *mut *mut libc::c_void as *mut libc::c_void,
    );
    RedisModule_GetApi
        .unwrap()(
        b"RedisModule_ZsetRem\0" as *const u8 as *const libc::c_char,
        &mut RedisModule_ZsetRem
            as *mut Option::<
                unsafe extern "C" fn(
                    *mut RedisModuleKey,
                    *mut RedisModuleString,
                    *mut libc::c_int,
                ) -> libc::c_int,
            > as *mut *mut libc::c_void as *mut libc::c_void,
    );
    RedisModule_GetApi
        .unwrap()(
        b"RedisModule_ZsetRangeStop\0" as *const u8 as *const libc::c_char,
        &mut RedisModule_ZsetRangeStop
            as *mut Option::<unsafe extern "C" fn(*mut RedisModuleKey) -> ()>
            as *mut *mut libc::c_void as *mut libc::c_void,
    );
    RedisModule_GetApi
        .unwrap()(
        b"RedisModule_ZsetFirstInScoreRange\0" as *const u8 as *const libc::c_char,
        &mut RedisModule_ZsetFirstInScoreRange
            as *mut Option::<
                unsafe extern "C" fn(
                    *mut RedisModuleKey,
                    libc::c_double,
                    libc::c_double,
                    libc::c_int,
                    libc::c_int,
                ) -> libc::c_int,
            > as *mut *mut libc::c_void as *mut libc::c_void,
    );
    RedisModule_GetApi
        .unwrap()(
        b"RedisModule_ZsetLastInScoreRange\0" as *const u8 as *const libc::c_char,
        &mut RedisModule_ZsetLastInScoreRange
            as *mut Option::<
                unsafe extern "C" fn(
                    *mut RedisModuleKey,
                    libc::c_double,
                    libc::c_double,
                    libc::c_int,
                    libc::c_int,
                ) -> libc::c_int,
            > as *mut *mut libc::c_void as *mut libc::c_void,
    );
    RedisModule_GetApi
        .unwrap()(
        b"RedisModule_ZsetFirstInLexRange\0" as *const u8 as *const libc::c_char,
        &mut RedisModule_ZsetFirstInLexRange
            as *mut Option::<
                unsafe extern "C" fn(
                    *mut RedisModuleKey,
                    *mut RedisModuleString,
                    *mut RedisModuleString,
                ) -> libc::c_int,
            > as *mut *mut libc::c_void as *mut libc::c_void,
    );
    RedisModule_GetApi
        .unwrap()(
        b"RedisModule_ZsetLastInLexRange\0" as *const u8 as *const libc::c_char,
        &mut RedisModule_ZsetLastInLexRange
            as *mut Option::<
                unsafe extern "C" fn(
                    *mut RedisModuleKey,
                    *mut RedisModuleString,
                    *mut RedisModuleString,
                ) -> libc::c_int,
            > as *mut *mut libc::c_void as *mut libc::c_void,
    );
    RedisModule_GetApi
        .unwrap()(
        b"RedisModule_ZsetRangeCurrentElement\0" as *const u8 as *const libc::c_char,
        &mut RedisModule_ZsetRangeCurrentElement
            as *mut Option::<
                unsafe extern "C" fn(
                    *mut RedisModuleKey,
                    *mut libc::c_double,
                ) -> *mut RedisModuleString,
            > as *mut *mut libc::c_void as *mut libc::c_void,
    );
    RedisModule_GetApi
        .unwrap()(
        b"RedisModule_ZsetRangeNext\0" as *const u8 as *const libc::c_char,
        &mut RedisModule_ZsetRangeNext
            as *mut Option::<unsafe extern "C" fn(*mut RedisModuleKey) -> libc::c_int>
            as *mut *mut libc::c_void as *mut libc::c_void,
    );
    RedisModule_GetApi
        .unwrap()(
        b"RedisModule_ZsetRangePrev\0" as *const u8 as *const libc::c_char,
        &mut RedisModule_ZsetRangePrev
            as *mut Option::<unsafe extern "C" fn(*mut RedisModuleKey) -> libc::c_int>
            as *mut *mut libc::c_void as *mut libc::c_void,
    );
    RedisModule_GetApi
        .unwrap()(
        b"RedisModule_ZsetRangeEndReached\0" as *const u8 as *const libc::c_char,
        &mut RedisModule_ZsetRangeEndReached
            as *mut Option::<unsafe extern "C" fn(*mut RedisModuleKey) -> libc::c_int>
            as *mut *mut libc::c_void as *mut libc::c_void,
    );
    RedisModule_GetApi
        .unwrap()(
        b"RedisModule_HashSet\0" as *const u8 as *const libc::c_char,
        &mut RedisModule_HashSet
            as *mut Option::<
                unsafe extern "C" fn(
                    *mut RedisModuleKey,
                    libc::c_int,
                    ...
                ) -> libc::c_int,
            > as *mut *mut libc::c_void as *mut libc::c_void,
    );
    RedisModule_GetApi
        .unwrap()(
        b"RedisModule_HashGet\0" as *const u8 as *const libc::c_char,
        &mut RedisModule_HashGet
            as *mut Option::<
                unsafe extern "C" fn(
                    *mut RedisModuleKey,
                    libc::c_int,
                    ...
                ) -> libc::c_int,
            > as *mut *mut libc::c_void as *mut libc::c_void,
    );
    RedisModule_GetApi
        .unwrap()(
        b"RedisModule_IsKeysPositionRequest\0" as *const u8 as *const libc::c_char,
        &mut RedisModule_IsKeysPositionRequest
            as *mut Option::<unsafe extern "C" fn(*mut RedisModuleCtx) -> libc::c_int>
            as *mut *mut libc::c_void as *mut libc::c_void,
    );
    RedisModule_GetApi
        .unwrap()(
        b"RedisModule_KeyAtPos\0" as *const u8 as *const libc::c_char,
        &mut RedisModule_KeyAtPos
            as *mut Option::<
                unsafe extern "C" fn(*mut RedisModuleCtx, libc::c_int) -> (),
            > as *mut *mut libc::c_void as *mut libc::c_void,
    );
    RedisModule_GetApi
        .unwrap()(
        b"RedisModule_GetClientId\0" as *const u8 as *const libc::c_char,
        &mut RedisModule_GetClientId
            as *mut Option::<
                unsafe extern "C" fn(*mut RedisModuleCtx) -> libc::c_ulonglong,
            > as *mut *mut libc::c_void as *mut libc::c_void,
    );
    RedisModule_GetApi
        .unwrap()(
        b"RedisModule_PoolAlloc\0" as *const u8 as *const libc::c_char,
        &mut RedisModule_PoolAlloc
            as *mut Option::<
                unsafe extern "C" fn(*mut RedisModuleCtx, size_t) -> *mut libc::c_void,
            > as *mut *mut libc::c_void as *mut libc::c_void,
    );
    RedisModule_GetApi
        .unwrap()(
        b"RedisModule_CreateDataType\0" as *const u8 as *const libc::c_char,
        &mut RedisModule_CreateDataType
            as *mut Option::<
                unsafe extern "C" fn(
                    *mut RedisModuleCtx,
                    *const libc::c_char,
                    libc::c_int,
                    *mut RedisModuleTypeMethods,
                ) -> *mut RedisModuleType,
            > as *mut *mut libc::c_void as *mut libc::c_void,
    );
    RedisModule_GetApi
        .unwrap()(
        b"RedisModule_ModuleTypeSetValue\0" as *const u8 as *const libc::c_char,
        &mut RedisModule_ModuleTypeSetValue
            as *mut Option::<
                unsafe extern "C" fn(
                    *mut RedisModuleKey,
                    *mut RedisModuleType,
                    *mut libc::c_void,
                ) -> libc::c_int,
            > as *mut *mut libc::c_void as *mut libc::c_void,
    );
    RedisModule_GetApi
        .unwrap()(
        b"RedisModule_ModuleTypeGetType\0" as *const u8 as *const libc::c_char,
        &mut RedisModule_ModuleTypeGetType
            as *mut Option::<
                unsafe extern "C" fn(*mut RedisModuleKey) -> *mut RedisModuleType,
            > as *mut *mut libc::c_void as *mut libc::c_void,
    );
    RedisModule_GetApi
        .unwrap()(
        b"RedisModule_ModuleTypeGetValue\0" as *const u8 as *const libc::c_char,
        &mut RedisModule_ModuleTypeGetValue
            as *mut Option::<
                unsafe extern "C" fn(*mut RedisModuleKey) -> *mut libc::c_void,
            > as *mut *mut libc::c_void as *mut libc::c_void,
    );
    RedisModule_GetApi
        .unwrap()(
        b"RedisModule_SaveUnsigned\0" as *const u8 as *const libc::c_char,
        &mut RedisModule_SaveUnsigned
            as *mut Option::<unsafe extern "C" fn(*mut RedisModuleIO, uint64_t) -> ()>
            as *mut *mut libc::c_void as *mut libc::c_void,
    );
    RedisModule_GetApi
        .unwrap()(
        b"RedisModule_LoadUnsigned\0" as *const u8 as *const libc::c_char,
        &mut RedisModule_LoadUnsigned
            as *mut Option::<unsafe extern "C" fn(*mut RedisModuleIO) -> uint64_t>
            as *mut *mut libc::c_void as *mut libc::c_void,
    );
    RedisModule_GetApi
        .unwrap()(
        b"RedisModule_SaveSigned\0" as *const u8 as *const libc::c_char,
        &mut RedisModule_SaveSigned
            as *mut Option::<unsafe extern "C" fn(*mut RedisModuleIO, int64_t) -> ()>
            as *mut *mut libc::c_void as *mut libc::c_void,
    );
    RedisModule_GetApi
        .unwrap()(
        b"RedisModule_LoadSigned\0" as *const u8 as *const libc::c_char,
        &mut RedisModule_LoadSigned
            as *mut Option::<unsafe extern "C" fn(*mut RedisModuleIO) -> int64_t>
            as *mut *mut libc::c_void as *mut libc::c_void,
    );
    RedisModule_GetApi
        .unwrap()(
        b"RedisModule_SaveString\0" as *const u8 as *const libc::c_char,
        &mut RedisModule_SaveString
            as *mut Option::<
                unsafe extern "C" fn(*mut RedisModuleIO, *mut RedisModuleString) -> (),
            > as *mut *mut libc::c_void as *mut libc::c_void,
    );
    RedisModule_GetApi
        .unwrap()(
        b"RedisModule_SaveStringBuffer\0" as *const u8 as *const libc::c_char,
        &mut RedisModule_SaveStringBuffer
            as *mut Option::<
                unsafe extern "C" fn(
                    *mut RedisModuleIO,
                    *const libc::c_char,
                    size_t,
                ) -> (),
            > as *mut *mut libc::c_void as *mut libc::c_void,
    );
    RedisModule_GetApi
        .unwrap()(
        b"RedisModule_LoadString\0" as *const u8 as *const libc::c_char,
        &mut RedisModule_LoadString
            as *mut Option::<
                unsafe extern "C" fn(*mut RedisModuleIO) -> *mut RedisModuleString,
            > as *mut *mut libc::c_void as *mut libc::c_void,
    );
    RedisModule_GetApi
        .unwrap()(
        b"RedisModule_LoadStringBuffer\0" as *const u8 as *const libc::c_char,
        &mut RedisModule_LoadStringBuffer
            as *mut Option::<
                unsafe extern "C" fn(
                    *mut RedisModuleIO,
                    *mut size_t,
                ) -> *mut libc::c_char,
            > as *mut *mut libc::c_void as *mut libc::c_void,
    );
    RedisModule_GetApi
        .unwrap()(
        b"RedisModule_SaveDouble\0" as *const u8 as *const libc::c_char,
        &mut RedisModule_SaveDouble
            as *mut Option::<
                unsafe extern "C" fn(*mut RedisModuleIO, libc::c_double) -> (),
            > as *mut *mut libc::c_void as *mut libc::c_void,
    );
    RedisModule_GetApi
        .unwrap()(
        b"RedisModule_LoadDouble\0" as *const u8 as *const libc::c_char,
        &mut RedisModule_LoadDouble
            as *mut Option::<unsafe extern "C" fn(*mut RedisModuleIO) -> libc::c_double>
            as *mut *mut libc::c_void as *mut libc::c_void,
    );
    RedisModule_GetApi
        .unwrap()(
        b"RedisModule_SaveFloat\0" as *const u8 as *const libc::c_char,
        &mut RedisModule_SaveFloat
            as *mut Option::<
                unsafe extern "C" fn(*mut RedisModuleIO, libc::c_float) -> (),
            > as *mut *mut libc::c_void as *mut libc::c_void,
    );
    RedisModule_GetApi
        .unwrap()(
        b"RedisModule_LoadFloat\0" as *const u8 as *const libc::c_char,
        &mut RedisModule_LoadFloat
            as *mut Option::<unsafe extern "C" fn(*mut RedisModuleIO) -> libc::c_float>
            as *mut *mut libc::c_void as *mut libc::c_void,
    );
    RedisModule_GetApi
        .unwrap()(
        b"RedisModule_EmitAOF\0" as *const u8 as *const libc::c_char,
        &mut RedisModule_EmitAOF
            as *mut Option::<
                unsafe extern "C" fn(
                    *mut RedisModuleIO,
                    *const libc::c_char,
                    *const libc::c_char,
                    ...
                ) -> (),
            > as *mut *mut libc::c_void as *mut libc::c_void,
    );
    RedisModule_GetApi
        .unwrap()(
        b"RedisModule_Log\0" as *const u8 as *const libc::c_char,
        &mut RedisModule_Log
            as *mut Option::<
                unsafe extern "C" fn(
                    *mut RedisModuleCtx,
                    *const libc::c_char,
                    *const libc::c_char,
                    ...
                ) -> (),
            > as *mut *mut libc::c_void as *mut libc::c_void,
    );
    RedisModule_GetApi
        .unwrap()(
        b"RedisModule_LogIOError\0" as *const u8 as *const libc::c_char,
        &mut RedisModule_LogIOError
            as *mut Option::<
                unsafe extern "C" fn(
                    *mut RedisModuleIO,
                    *const libc::c_char,
                    *const libc::c_char,
                    ...
                ) -> (),
            > as *mut *mut libc::c_void as *mut libc::c_void,
    );
    RedisModule_GetApi
        .unwrap()(
        b"RedisModule_StringAppendBuffer\0" as *const u8 as *const libc::c_char,
        &mut RedisModule_StringAppendBuffer
            as *mut Option::<
                unsafe extern "C" fn(
                    *mut RedisModuleCtx,
                    *mut RedisModuleString,
                    *const libc::c_char,
                    size_t,
                ) -> libc::c_int,
            > as *mut *mut libc::c_void as *mut libc::c_void,
    );
    RedisModule_GetApi
        .unwrap()(
        b"RedisModule_RetainString\0" as *const u8 as *const libc::c_char,
        &mut RedisModule_RetainString
            as *mut Option::<
                unsafe extern "C" fn(*mut RedisModuleCtx, *mut RedisModuleString) -> (),
            > as *mut *mut libc::c_void as *mut libc::c_void,
    );
    RedisModule_GetApi
        .unwrap()(
        b"RedisModule_StringCompare\0" as *const u8 as *const libc::c_char,
        &mut RedisModule_StringCompare
            as *mut Option::<
                unsafe extern "C" fn(
                    *mut RedisModuleString,
                    *mut RedisModuleString,
                ) -> libc::c_int,
            > as *mut *mut libc::c_void as *mut libc::c_void,
    );
    RedisModule_GetApi
        .unwrap()(
        b"RedisModule_GetContextFromIO\0" as *const u8 as *const libc::c_char,
        &mut RedisModule_GetContextFromIO
            as *mut Option::<
                unsafe extern "C" fn(*mut RedisModuleIO) -> *mut RedisModuleCtx,
            > as *mut *mut libc::c_void as *mut libc::c_void,
    );
    RedisModule_GetApi
        .unwrap()(
        b"RedisModule_BlockClient\0" as *const u8 as *const libc::c_char,
        &mut RedisModule_BlockClient
            as *mut Option::<
                unsafe extern "C" fn(
                    *mut RedisModuleCtx,
                    RedisModuleCmdFunc,
                    RedisModuleCmdFunc,
                    Option::<unsafe extern "C" fn(*mut libc::c_void) -> ()>,
                    libc::c_longlong,
                ) -> *mut RedisModuleBlockedClient,
            > as *mut *mut libc::c_void as *mut libc::c_void,
    );
    RedisModule_GetApi
        .unwrap()(
        b"RedisModule_UnblockClient\0" as *const u8 as *const libc::c_char,
        &mut RedisModule_UnblockClient
            as *mut Option::<
                unsafe extern "C" fn(
                    *mut RedisModuleBlockedClient,
                    *mut libc::c_void,
                ) -> libc::c_int,
            > as *mut *mut libc::c_void as *mut libc::c_void,
    );
    RedisModule_GetApi
        .unwrap()(
        b"RedisModule_IsBlockedReplyRequest\0" as *const u8 as *const libc::c_char,
        &mut RedisModule_IsBlockedReplyRequest
            as *mut Option::<unsafe extern "C" fn(*mut RedisModuleCtx) -> libc::c_int>
            as *mut *mut libc::c_void as *mut libc::c_void,
    );
    RedisModule_GetApi
        .unwrap()(
        b"RedisModule_IsBlockedTimeoutRequest\0" as *const u8 as *const libc::c_char,
        &mut RedisModule_IsBlockedTimeoutRequest
            as *mut Option::<unsafe extern "C" fn(*mut RedisModuleCtx) -> libc::c_int>
            as *mut *mut libc::c_void as *mut libc::c_void,
    );
    RedisModule_GetApi
        .unwrap()(
        b"RedisModule_GetBlockedClientPrivateData\0" as *const u8 as *const libc::c_char,
        &mut RedisModule_GetBlockedClientPrivateData
            as *mut Option::<
                unsafe extern "C" fn(*mut RedisModuleCtx) -> *mut libc::c_void,
            > as *mut *mut libc::c_void as *mut libc::c_void,
    );
    RedisModule_GetApi
        .unwrap()(
        b"RedisModule_AbortBlock\0" as *const u8 as *const libc::c_char,
        &mut RedisModule_AbortBlock
            as *mut Option::<
                unsafe extern "C" fn(*mut RedisModuleBlockedClient) -> libc::c_int,
            > as *mut *mut libc::c_void as *mut libc::c_void,
    );
    RedisModule_GetApi
        .unwrap()(
        b"RedisModule_Milliseconds\0" as *const u8 as *const libc::c_char,
        &mut RedisModule_Milliseconds
            as *mut Option::<unsafe extern "C" fn() -> libc::c_longlong>
            as *mut *mut libc::c_void as *mut libc::c_void,
    );
    RedisModule_SetModuleAttribs.unwrap()(ctx, name, ver, apiver);
    return 0 as libc::c_int;
}
static mut NRType: *mut RedisModuleType = 0 as *const RedisModuleType
    as *mut RedisModuleType;
pub static mut NRNextId: uint64_t = 1 as libc::c_int as uint64_t;
static mut NRPendingTrainingMutex: pthread_mutex_t = pthread_mutex_t {
    __data: {
        let mut init = __pthread_mutex_s {
            __lock: 0 as libc::c_int,
            __count: 0 as libc::c_int as libc::c_uint,
            __owner: 0 as libc::c_int,
            __nusers: 0 as libc::c_int as libc::c_uint,
            __kind: PTHREAD_MUTEX_TIMED_NP as libc::c_int,
            __spins: 0 as libc::c_int as libc::c_short,
            __elision: 0 as libc::c_int as libc::c_short,
            __list: {
                let mut init = __pthread_internal_list {
                    __prev: 0 as *const __pthread_internal_list
                        as *mut __pthread_internal_list,
                    __next: 0 as *const __pthread_internal_list
                        as *mut __pthread_internal_list,
                };
                init
            },
        };
        init
    },
};
static mut NRTrainings: [NRPendingTraining; 32] = [NRPendingTraining {
    key: 0 as *const RedisModuleString as *mut RedisModuleString,
    db_id: 0,
    tid: 0,
    in_progress: 0,
    nr: 0 as *const NRTypeObject as *mut NRTypeObject,
    dataset_error: 0.,
    test_error: 0.,
    class_error: 0.,
    curcycle: 0,
}; 32];
static mut NRPendingTrainingCount: libc::c_int = 0 as libc::c_int;
pub unsafe extern "C" fn NRMilliseconds() -> libc::c_longlong {
    let mut tv: timeval = timeval { tv_sec: 0, tv_usec: 0 };
    let mut ust: libc::c_longlong = 0;
    gettimeofday(&mut tv, 0 as *mut libc::c_void);
    ust = tv.tv_sec as libc::c_longlong * 1000000 as libc::c_int as libc::c_longlong;
    ust += tv.tv_usec as libc::c_longlong;
    return ust / 1000 as libc::c_int as libc::c_longlong;
}
pub unsafe extern "C" fn createNRTypeObject(
    mut flags: libc::c_int,
    mut layers: *mut libc::c_int,
    mut numlayers: libc::c_int,
    mut dset_len: libc::c_int,
    mut test_len: libc::c_int,
) -> *mut NRTypeObject {
    let mut o: *mut NRTypeObject = 0 as *mut NRTypeObject;
    o = RedisModule_Calloc
        .unwrap()(
        1 as libc::c_int as size_t,
        ::std::mem::size_of::<NRTypeObject>() as libc::c_ulong,
    ) as *mut NRTypeObject;
    let fresh0 = NRNextId;
    NRNextId = NRNextId.wrapping_add(1);
    (*o).id = fresh0;
    (*o).flags = flags as uint32_t;
    (*o).nn = AnnCreateNet(numlayers, layers);
    (*o).dataset.maxlen = dset_len as uint32_t;
    (*o).test.maxlen = test_len as uint32_t;
    let mut ilen: libc::c_int = (*((*(*o).nn).layer)
        .offset(((*(*o).nn).layers - 1 as libc::c_int) as isize))
        .units - 1 as libc::c_int;
    let mut olen: libc::c_int = (*((*(*o).nn).layer).offset(0 as libc::c_int as isize))
        .units;
    (*o)
        .inorm = RedisModule_Calloc
        .unwrap()(
        1 as libc::c_int as size_t,
        (::std::mem::size_of::<libc::c_float>() as libc::c_ulong)
            .wrapping_mul(ilen as libc::c_ulong),
    ) as *mut libc::c_float;
    (*o)
        .onorm = RedisModule_Calloc
        .unwrap()(
        1 as libc::c_int as size_t,
        (::std::mem::size_of::<libc::c_float>() as libc::c_ulong)
            .wrapping_mul(olen as libc::c_ulong),
    ) as *mut libc::c_float;
    let mut j: libc::c_int = 0 as libc::c_int;
    while j < ilen {
        *((*o).inorm).offset(j as isize) = 1 as libc::c_int as libc::c_float;
        j += 1;
        j;
    }
    let mut j_0: libc::c_int = 0 as libc::c_int;
    while j_0 < olen {
        *((*o).onorm).offset(j_0 as isize) = 1 as libc::c_int as libc::c_float;
        j_0 += 1;
        j_0;
    }
    return o;
}
pub unsafe extern "C" fn NRTypeInsertData(
    mut o: *mut NRTypeObject,
    mut inputs: *mut libc::c_float,
    mut outputs: *mut libc::c_float,
    mut target_ds: libc::c_int,
) {
    let mut target: *mut NRDataset = 0 as *mut NRDataset;
    if (*o).dataset.maxlen == 0 as libc::c_int as libc::c_uint
        && (*o).test.maxlen == 0 as libc::c_int as libc::c_uint
    {
        return;
    }
    if target_ds == 1 as libc::c_int {
        target = &mut (*o).dataset;
    } else if target_ds == 2 as libc::c_int {
        target = &mut (*o).test;
    }
    if (*o).dataset.maxlen == 0 as libc::c_int as libc::c_uint {
        target = &mut (*o).test;
    } else if (*o).test.maxlen == 0 as libc::c_int as libc::c_uint {
        target = &mut (*o).dataset;
    }
    if target.is_null() {
        if (*o).dataset.len != (*o).dataset.maxlen || (*o).test.len != (*o).dataset.len {
            let mut fill_a: libc::c_float = (*o).dataset.len as libc::c_float
                / (*o).dataset.maxlen as libc::c_float;
            let mut fill_b: libc::c_float = (*o).test.len as libc::c_float
                / (*o).test.maxlen as libc::c_float;
            target = if fill_a <= fill_b { &mut (*o).dataset } else { &mut (*o).test };
        } else {
            let mut r: libc::c_double = (rand() / 2147483647 as libc::c_int)
                as libc::c_double;
            let mut sumlen: libc::c_double = ((*o).dataset.maxlen)
                .wrapping_add((*o).test.maxlen) as libc::c_double;
            if r < (*o).dataset.maxlen as libc::c_double / sumlen {
                target = &mut (*o).dataset;
            } else {
                target = &mut (*o).test;
            }
        }
    }
    let mut idx: size_t = 0;
    let mut j: libc::c_int = 0;
    let mut numin: libc::c_int = (*((*(*o).nn).layer)
        .offset(((*(*o).nn).layers - 1 as libc::c_int) as isize))
        .units - 1 as libc::c_int;
    let mut numout: libc::c_int = (*((*(*o).nn).layer).offset(0 as libc::c_int as isize))
        .units;
    if (*target).maxlen == (*target).len {
        idx = (rand() as libc::c_uint).wrapping_rem((*target).maxlen) as size_t;
    } else {
        idx = (*target).len as size_t;
        (*target).len = ((*target).len).wrapping_add(1);
        (*target).len;
        (*target)
            .inputs = RedisModule_Realloc
            .unwrap()(
            (*target).inputs as *mut libc::c_void,
            (::std::mem::size_of::<libc::c_float>() as libc::c_ulong)
                .wrapping_mul(numin as libc::c_ulong)
                .wrapping_mul((*target).len as libc::c_ulong),
        ) as *mut libc::c_float;
        (*target)
            .outputs = RedisModule_Realloc
            .unwrap()(
            (*target).outputs as *mut libc::c_void,
            (::std::mem::size_of::<libc::c_float>() as libc::c_ulong)
                .wrapping_mul(numout as libc::c_ulong)
                .wrapping_mul((*target).len as libc::c_ulong),
        ) as *mut libc::c_float;
    }
    j = 0 as libc::c_int;
    while j < numin {
        *((*target).inputs)
            .offset(
                idx.wrapping_mul(numin as libc::c_ulong).wrapping_add(j as libc::c_ulong)
                    as isize,
            ) = *inputs.offset(j as isize);
        j += 1;
        j;
    }
    j = 0 as libc::c_int;
    while j < numout {
        *((*target).outputs)
            .offset(
                idx
                    .wrapping_mul(numout as libc::c_ulong)
                    .wrapping_add(j as libc::c_ulong) as isize,
            ) = *outputs.offset(j as isize);
        j += 1;
        j;
    }
}
pub unsafe extern "C" fn NRDatasetFree(mut dset: *mut NRDataset) {
    RedisModule_Free.unwrap()((*dset).inputs as *mut libc::c_void);
    RedisModule_Free.unwrap()((*dset).outputs as *mut libc::c_void);
}
pub unsafe extern "C" fn NRTypeReleaseObject(mut o: *mut NRTypeObject) {
    AnnFree((*o).nn);
    NRDatasetFree(&mut (*o).dataset);
    NRDatasetFree(&mut (*o).test);
    RedisModule_Free.unwrap()((*o).inorm as *mut libc::c_void);
    RedisModule_Free.unwrap()((*o).onorm as *mut libc::c_void);
    RedisModule_Free.unwrap()(o as *mut libc::c_void);
}
pub unsafe extern "C" fn NRClone(
    mut o: *mut NRTypeObject,
    mut newid: libc::c_int,
) -> *mut NRTypeObject {
    let mut copy: *mut NRTypeObject = 0 as *mut NRTypeObject;
    copy = RedisModule_Calloc
        .unwrap()(
        1 as libc::c_int as size_t,
        ::std::mem::size_of::<NRTypeObject>() as libc::c_ulong,
    ) as *mut NRTypeObject;
    *copy = *o;
    if newid != 0 {
        let fresh1 = NRNextId;
        NRNextId = NRNextId.wrapping_add(1);
        (*copy).id = fresh1;
    }
    (*copy).nn = AnnClone((*o).nn);
    (*copy).dataset = (*o).dataset;
    (*copy).test = (*o).test;
    let mut ilen: libc::c_int = (*((*(*o).nn).layer)
        .offset(((*(*o).nn).layers - 1 as libc::c_int) as isize))
        .units - 1 as libc::c_int;
    let mut olen: libc::c_int = (*((*(*o).nn).layer).offset(0 as libc::c_int as isize))
        .units;
    (*copy)
        .dataset
        .inputs = RedisModule_Alloc
        .unwrap()(
        (::std::mem::size_of::<libc::c_float>() as libc::c_ulong)
            .wrapping_mul(ilen as libc::c_ulong)
            .wrapping_mul((*o).dataset.len as libc::c_ulong),
    ) as *mut libc::c_float;
    (*copy)
        .dataset
        .outputs = RedisModule_Alloc
        .unwrap()(
        (::std::mem::size_of::<libc::c_float>() as libc::c_ulong)
            .wrapping_mul(olen as libc::c_ulong)
            .wrapping_mul((*o).dataset.len as libc::c_ulong),
    ) as *mut libc::c_float;
    (*copy)
        .test
        .inputs = RedisModule_Alloc
        .unwrap()(
        (::std::mem::size_of::<libc::c_float>() as libc::c_ulong)
            .wrapping_mul(ilen as libc::c_ulong)
            .wrapping_mul((*o).test.len as libc::c_ulong),
    ) as *mut libc::c_float;
    (*copy)
        .test
        .outputs = RedisModule_Alloc
        .unwrap()(
        (::std::mem::size_of::<libc::c_float>() as libc::c_ulong)
            .wrapping_mul(olen as libc::c_ulong)
            .wrapping_mul((*o).test.len as libc::c_ulong),
    ) as *mut libc::c_float;
    memcpy(
        (*copy).dataset.inputs as *mut libc::c_void,
        (*o).dataset.inputs as *const libc::c_void,
        (::std::mem::size_of::<libc::c_float>() as libc::c_ulong)
            .wrapping_mul(ilen as libc::c_ulong)
            .wrapping_mul((*o).dataset.len as libc::c_ulong),
    );
    memcpy(
        (*copy).dataset.outputs as *mut libc::c_void,
        (*o).dataset.outputs as *const libc::c_void,
        (::std::mem::size_of::<libc::c_float>() as libc::c_ulong)
            .wrapping_mul(olen as libc::c_ulong)
            .wrapping_mul((*o).dataset.len as libc::c_ulong),
    );
    memcpy(
        (*copy).test.inputs as *mut libc::c_void,
        (*o).test.inputs as *const libc::c_void,
        (::std::mem::size_of::<libc::c_float>() as libc::c_ulong)
            .wrapping_mul(ilen as libc::c_ulong)
            .wrapping_mul((*o).test.len as libc::c_ulong),
    );
    memcpy(
        (*copy).test.outputs as *mut libc::c_void,
        (*o).test.outputs as *const libc::c_void,
        (::std::mem::size_of::<libc::c_float>() as libc::c_ulong)
            .wrapping_mul(olen as libc::c_ulong)
            .wrapping_mul((*o).test.len as libc::c_ulong),
    );
    (*copy)
        .inorm = RedisModule_Alloc
        .unwrap()(
        (::std::mem::size_of::<libc::c_float>() as libc::c_ulong)
            .wrapping_mul(ilen as libc::c_ulong),
    ) as *mut libc::c_float;
    (*copy)
        .onorm = RedisModule_Alloc
        .unwrap()(
        (::std::mem::size_of::<libc::c_float>() as libc::c_ulong)
            .wrapping_mul(olen as libc::c_ulong),
    ) as *mut libc::c_float;
    memcpy(
        (*copy).inorm as *mut libc::c_void,
        (*o).inorm as *const libc::c_void,
        (::std::mem::size_of::<libc::c_float>() as libc::c_ulong)
            .wrapping_mul(ilen as libc::c_ulong),
    );
    memcpy(
        (*copy).onorm as *mut libc::c_void,
        (*o).onorm as *const libc::c_void,
        (::std::mem::size_of::<libc::c_float>() as libc::c_ulong)
            .wrapping_mul(olen as libc::c_ulong),
    );
    return copy;
}
pub unsafe extern "C" fn NRTransferWeights(
    mut ctx: *mut RedisModuleCtx,
    mut dst: *mut NRTypeObject,
    mut src: *mut NRTypeObject,
) {
    if (*dst).id != (*src).id {
        RedisModule_Log
            .unwrap()(
            ctx,
            b"warning\0" as *const u8 as *const libc::c_char,
            b"NSTransferWeight(): source and destination neural network IDs don't match. This is unexpected, probably a bug inside the module. Weights not transferred back to the origina NN.\0"
                as *const u8 as *const libc::c_char,
        );
        return;
    }
    AnnFree((*dst).nn);
    (*dst).nn = AnnClone((*src).nn);
    (*dst).training_total_steps = (*src).training_total_steps;
    (*dst).training_total_ms = (*src).training_total_ms;
    (*dst).dataset_error = (*src).dataset_error;
    (*dst).test_error = (*src).test_error;
    (*dst).test_class_error = (*src).test_class_error;
    (*dst).flags
        |= (*src).flags & ((1 as libc::c_int) << 5 as libc::c_int) as libc::c_uint;
    let mut ilen: libc::c_int = (*((*(*src).nn).layer)
        .offset(((*(*src).nn).layers - 1 as libc::c_int) as isize))
        .units - 1 as libc::c_int;
    let mut olen: libc::c_int = (*((*(*src).nn).layer).offset(0 as libc::c_int as isize))
        .units;
    memcpy(
        (*dst).inorm as *mut libc::c_void,
        (*src).inorm as *const libc::c_void,
        (::std::mem::size_of::<libc::c_float>() as libc::c_ulong)
            .wrapping_mul(ilen as libc::c_ulong),
    );
    memcpy(
        (*dst).onorm as *mut libc::c_void,
        (*src).onorm as *const libc::c_void,
        (::std::mem::size_of::<libc::c_float>() as libc::c_ulong)
            .wrapping_mul(olen as libc::c_ulong),
    );
}
pub unsafe extern "C" fn NRTrainingThreadMain(
    mut arg: *mut libc::c_void,
) -> *mut libc::c_void {
    let mut pt: *mut NRPendingTraining = arg as *mut NRPendingTraining;
    let mut nr: *mut NRTypeObject = (*pt).nr;
    let mut training_iterations: libc::c_int = 1 as libc::c_int;
    let mut train_error: libc::c_float = 0 as libc::c_int as libc::c_float;
    let mut test_error: libc::c_float = 0 as libc::c_int as libc::c_float;
    let mut class_error: libc::c_float = 0 as libc::c_int as libc::c_float;
    let mut past_train_error: libc::c_float = (1.0f64 / 0.0f64) as libc::c_float;
    let mut past_test_error: libc::c_float = (1.0f64 / 0.0f64) as libc::c_float;
    let mut auto_stop: libc::c_int = ((*nr).flags
        & ((1 as libc::c_int) << 4 as libc::c_int) as libc::c_uint) as libc::c_int;
    let mut backtrack: libc::c_int = ((*nr).flags
        & ((1 as libc::c_int) << 6 as libc::c_int) as libc::c_uint) as libc::c_int;
    let mut cycles: uint64_t = 0 as libc::c_int as uint64_t;
    let mut start: libc::c_longlong = NRMilliseconds();
    let mut cycle_time: libc::c_longlong = 0;
    let mut overfitting_count: libc::c_int = 0 as libc::c_int;
    let mut overfitting_limit: libc::c_int = 5 as libc::c_int;
    let mut best_test_error: libc::c_float = (1.0f64 / 0.0f64) as libc::c_float;
    (*nr).flags &= !((1 as libc::c_int) << 5 as libc::c_int) as libc::c_uint;
    if (*nr).flags & ((1 as libc::c_int) << 3 as libc::c_int) as libc::c_uint != 0
        && (*nr).dataset.len != 0
    {
        let mut ilen: libc::c_int = (*((*(*nr).nn).layer)
            .offset(((*(*nr).nn).layers - 1 as libc::c_int) as isize))
            .units - 1 as libc::c_int;
        let mut olen: libc::c_int = (*((*(*nr).nn).layer)
            .offset(0 as libc::c_int as isize))
            .units;
        let mut imax: *mut libc::c_float = (*nr).inorm;
        let mut omax: *mut libc::c_float = (*nr).onorm;
        let mut inputs: *mut libc::c_float = (*nr).dataset.inputs;
        let mut outputs: *mut libc::c_float = (*nr).dataset.outputs;
        let mut i: libc::c_int = 0 as libc::c_int;
        while i < ilen {
            *imax.offset(i as isize) = 1 as libc::c_int as libc::c_float;
            i += 1;
            i;
        }
        let mut i_0: libc::c_int = 0 as libc::c_int;
        while i_0 < olen {
            *omax.offset(i_0 as isize) = 1 as libc::c_int as libc::c_float;
            i_0 += 1;
            i_0;
        }
        let mut j: uint32_t = 0 as libc::c_int as uint32_t;
        while j < (*nr).dataset.len {
            let mut i_1: libc::c_int = 0 as libc::c_int;
            while i_1 < ilen {
                if fabs(*inputs.offset(i_1 as isize) as libc::c_double)
                    > *imax.offset(i_1 as isize) as libc::c_double
                {
                    *imax
                        .offset(
                            i_1 as isize,
                        ) = fabs(*inputs.offset(i_1 as isize) as libc::c_double)
                        as libc::c_float;
                }
                i_1 += 1;
                i_1;
            }
            let mut i_2: libc::c_int = 0 as libc::c_int;
            while i_2 < olen {
                if fabs(*outputs.offset(i_2 as isize) as libc::c_double)
                    > *omax.offset(i_2 as isize) as libc::c_double
                {
                    *omax
                        .offset(
                            i_2 as isize,
                        ) = fabs(*outputs.offset(i_2 as isize) as libc::c_double)
                        as libc::c_float;
                }
                i_2 += 1;
                i_2;
            }
            inputs = inputs.offset(ilen as isize);
            outputs = outputs.offset(olen as isize);
            j = j.wrapping_add(1);
            j;
        }
        let mut i_3: libc::c_int = 0 as libc::c_int;
        while i_3 < ilen {
            if *imax.offset(i_3 as isize) != 1 as libc::c_int as libc::c_float {
                let ref mut fresh2 = *imax.offset(i_3 as isize);
                *fresh2 = (*fresh2 as libc::c_double * 1.2f64) as libc::c_float;
            }
            i_3 += 1;
            i_3;
        }
        let mut i_4: libc::c_int = 0 as libc::c_int;
        while i_4 < olen {
            if *omax.offset(i_4 as isize) != 1 as libc::c_int as libc::c_float {
                let ref mut fresh3 = *omax.offset(i_4 as isize);
                *fresh3 = (*fresh3 as libc::c_double * 1.2f64) as libc::c_float;
            }
            i_4 += 1;
            i_4;
        }
        inputs = (*nr).dataset.inputs;
        outputs = (*nr).dataset.outputs;
        let mut j_0: uint32_t = 0 as libc::c_int as uint32_t;
        while j_0 < (*nr).dataset.len {
            let mut i_5: libc::c_int = 0 as libc::c_int;
            while i_5 < ilen {
                *inputs.offset(i_5 as isize) /= *((*nr).inorm).offset(i_5 as isize);
                i_5 += 1;
                i_5;
            }
            if (*nr).flags & ((1 as libc::c_int) << 2 as libc::c_int) as libc::c_uint
                == 0
            {
                let mut i_6: libc::c_int = 0 as libc::c_int;
                while i_6 < olen {
                    *outputs.offset(i_6 as isize) /= *((*nr).onorm).offset(i_6 as isize);
                    i_6 += 1;
                    i_6;
                }
            }
            inputs = inputs.offset(ilen as isize);
            outputs = outputs.offset(olen as isize);
            j_0 = j_0.wrapping_add(1);
            j_0;
        }
        inputs = (*nr).test.inputs;
        outputs = (*nr).test.outputs;
        let mut j_1: uint32_t = 0 as libc::c_int as uint32_t;
        while j_1 < (*nr).test.len {
            let mut i_7: libc::c_int = 0 as libc::c_int;
            while i_7 < ilen {
                *inputs.offset(i_7 as isize) /= *((*nr).inorm).offset(i_7 as isize);
                i_7 += 1;
                i_7;
            }
            if (*nr).flags & ((1 as libc::c_int) << 2 as libc::c_int) as libc::c_uint
                == 0
            {
                let mut i_8: libc::c_int = 0 as libc::c_int;
                while i_8 < olen {
                    *outputs.offset(i_8 as isize) /= *((*nr).onorm).offset(i_8 as isize);
                    i_8 += 1;
                    i_8;
                }
            }
            inputs = inputs.offset(ilen as isize);
            outputs = outputs.offset(olen as isize);
            j_1 = j_1.wrapping_add(1);
            j_1;
        }
    }
    let mut saved: *mut Ann = 0 as *mut Ann;
    let mut saved_error: libc::c_float = 0.;
    let mut saved_train_error: libc::c_float = 0.;
    let mut saved_class_error: libc::c_float = 0.;
    loop {
        let mut cycle_start: libc::c_longlong = NRMilliseconds();
        train_error = AnnTrain(
            (*nr).nn,
            (*nr).dataset.inputs,
            (*nr).dataset.outputs,
            0 as libc::c_int as libc::c_float,
            training_iterations,
            (*nr).dataset.len as libc::c_int,
            0 as libc::c_int,
        );
        cycle_time = NRMilliseconds() - cycle_start;
        (*nr)
            .training_total_steps = ((*nr).training_total_steps as libc::c_ulong)
            .wrapping_add(
                ((*nr).dataset.len).wrapping_mul(training_iterations as libc::c_uint)
                    as libc::c_ulong,
            ) as uint64_t as uint64_t;
        if auto_stop != 0 {
            AnnTestError(
                (*nr).nn,
                (*nr).test.inputs,
                (*nr).test.outputs,
                (*nr).test.len as libc::c_int,
                &mut test_error,
                &mut class_error,
            );
            if train_error < past_train_error && test_error > past_test_error {
                overfitting_count += 1;
                overfitting_count;
                if overfitting_count == overfitting_limit {
                    (*nr).flags
                        |= ((1 as libc::c_int) << 5 as libc::c_int) as libc::c_uint;
                    break;
                }
            } else if overfitting_count > 0 as libc::c_int {
                overfitting_count -= 1;
                overfitting_count;
            }
            if backtrack != 0 && (saved.is_null() || test_error < saved_error) {
                saved_error = test_error;
                saved_train_error = train_error;
                saved_class_error = class_error;
                if !saved.is_null() {
                    AnnFree(saved);
                }
                saved = AnnClone((*nr).nn);
            }
            if test_error < best_test_error {
                overfitting_count = 0 as libc::c_int;
                best_test_error = test_error;
            }
            if (train_error as libc::c_double) < 0.000000000000001f64
                && (test_error as libc::c_double) < 0.000000000000001f64
            {
                break;
            }
        }
        cycles = cycles.wrapping_add(1);
        cycles;
        let mut total_time: libc::c_longlong = NRMilliseconds() - start;
        if (*nr).training_max_cycles != 0 && cycles == (*nr).training_max_cycles {
            break;
        }
        if (*nr).training_max_ms != 0
            && total_time > (*nr).training_max_ms as libc::c_longlong
        {
            break;
        }
        if total_time > 10000 as libc::c_int as libc::c_longlong
            && cycle_time < 100 as libc::c_int as libc::c_longlong
        {
            training_iterations += 1;
            training_iterations;
        }
        past_train_error = train_error;
        past_test_error = test_error;
        pthread_mutex_lock(&mut NRPendingTrainingMutex);
        (*pt).dataset_error = train_error;
        (*pt).test_error = test_error;
        if (*nr).flags & ((1 as libc::c_int) << 2 as libc::c_int) as libc::c_uint != 0 {
            (*pt).class_error = class_error;
        }
        (*pt).curcycle = cycles as libc::c_int;
        pthread_mutex_unlock(&mut NRPendingTrainingMutex);
    }
    if auto_stop == 0 {
        AnnTestError(
            (*nr).nn,
            (*nr).test.inputs,
            (*nr).test.outputs,
            (*nr).test.len as libc::c_int,
            &mut test_error,
            &mut class_error,
        );
    }
    if auto_stop != 0 && backtrack != 0 {
        if !saved.is_null() && saved_error < test_error {
            AnnFree((*nr).nn);
            (*nr).nn = saved;
            test_error = saved_error;
            train_error = saved_train_error;
            class_error = saved_class_error;
        } else if !saved.is_null() {
            AnnFree(saved);
        }
    }
    if (*nr).flags & ((1 as libc::c_int) << 2 as libc::c_int) as libc::c_uint != 0 {
        (*nr).test_class_error = class_error;
    }
    (*nr).dataset_error = train_error;
    (*nr).test_error = test_error;
    (*nr)
        .training_total_ms = ((*nr).training_total_ms as libc::c_ulonglong)
        .wrapping_add((NRMilliseconds() - start) as libc::c_ulonglong) as uint64_t
        as uint64_t;
    pthread_mutex_lock(&mut NRPendingTrainingMutex);
    (*pt).in_progress = 0 as libc::c_int;
    pthread_mutex_unlock(&mut NRPendingTrainingMutex);
    return 0 as *mut libc::c_void;
}
pub unsafe extern "C" fn NRStartTraining(
    mut ctx: *mut RedisModuleCtx,
    mut key: *mut RedisModuleString,
    mut dbid: libc::c_int,
    mut nr: *mut NRTypeObject,
) -> libc::c_int {
    pthread_mutex_lock(&mut NRPendingTrainingMutex);
    if NRPendingTrainingCount == 32 as libc::c_int {
        pthread_mutex_unlock(&mut NRPendingTrainingMutex);
        return 1 as libc::c_int;
    }
    let mut pt: *mut NRPendingTraining = &mut *NRTrainings
        .as_mut_ptr()
        .offset(NRPendingTrainingCount as isize) as *mut NRPendingTraining;
    (*pt).key = RedisModule_CreateStringFromString.unwrap()(ctx, key);
    RedisModule_RetainString.unwrap()(ctx, (*pt).key);
    (*pt).db_id = dbid;
    (*pt).in_progress = 1 as libc::c_int;
    (*pt).nr = NRClone(nr, 0 as libc::c_int);
    (*pt).dataset_error = 0 as libc::c_int as libc::c_float;
    (*pt).test_error = 0 as libc::c_int as libc::c_float;
    (*pt).class_error = 0 as libc::c_int as libc::c_float;
    (*pt).curcycle = 0 as libc::c_int;
    if pthread_create(
        &mut (*pt).tid,
        0 as *const pthread_attr_t,
        Some(
            NRTrainingThreadMain
                as unsafe extern "C" fn(*mut libc::c_void) -> *mut libc::c_void,
        ),
        pt as *mut libc::c_void,
    ) != 0 as libc::c_int
    {
        RedisModule_Log
            .unwrap()(
            ctx,
            b"warning\0" as *const u8 as *const libc::c_char,
            b"Unable to create a new pthread in NRStartTraining()\0" as *const u8
                as *const libc::c_char,
        );
        RedisModule_FreeString.unwrap()(ctx, (*pt).key);
        (*pt).key = 0 as *mut RedisModuleString;
        NRTypeReleaseObject((*pt).nr);
        pthread_mutex_unlock(&mut NRPendingTrainingMutex);
        return 1 as libc::c_int;
    }
    NRPendingTrainingCount += 1;
    NRPendingTrainingCount;
    (*nr).flags |= ((1 as libc::c_int) << 0 as libc::c_int) as libc::c_uint;
    (*nr).flags &= !((1 as libc::c_int) << 5 as libc::c_int) as libc::c_uint;
    pthread_mutex_unlock(&mut NRPendingTrainingMutex);
    return 0 as libc::c_int;
}
pub unsafe extern "C" fn NRCollectThreads(mut ctx: *mut RedisModuleCtx) -> libc::c_int {
    let mut collected: libc::c_int = 0 as libc::c_int;
    pthread_mutex_lock(&mut NRPendingTrainingMutex);
    let mut j: libc::c_int = 0 as libc::c_int;
    while j < NRPendingTrainingCount {
        let mut pt: *mut NRPendingTraining = &mut *NRTrainings
            .as_mut_ptr()
            .offset(j as isize) as *mut NRPendingTraining;
        if (*pt).in_progress == 0 as libc::c_int {
            let mut orig_id: libc::c_int = RedisModule_GetSelectedDb.unwrap()(ctx);
            if orig_id != (*pt).db_id {
                RedisModule_SelectDb.unwrap()(ctx, (*pt).db_id);
            }
            let mut key: *mut RedisModuleKey = RedisModule_OpenKey
                .unwrap()(
                ctx,
                (*pt).key,
                (1 as libc::c_int) << 0 as libc::c_int
                    | (1 as libc::c_int) << 1 as libc::c_int,
            ) as *mut RedisModuleKey;
            if RedisModule_ModuleTypeGetType.unwrap()(key) == NRType {
                let mut nr: *mut NRTypeObject = RedisModule_ModuleTypeGetValue
                    .unwrap()(key) as *mut NRTypeObject;
                if (*nr).id == (*(*pt).nr).id {
                    NRTransferWeights(ctx, nr, (*pt).nr);
                    (*nr).flags
                        &= !((1 as libc::c_int) << 0 as libc::c_int) as libc::c_uint;
                }
                RedisModule_FreeString.unwrap()(ctx, (*pt).key);
                (*pt).key = 0 as *mut RedisModuleString;
                NRTypeReleaseObject((*pt).nr);
                NRPendingTrainingCount -= 1;
                NRPendingTrainingCount;
                memcpy(
                    &mut *NRTrainings.as_mut_ptr().offset(j as isize)
                        as *mut NRPendingTraining as *mut libc::c_void,
                    &mut *NRTrainings
                        .as_mut_ptr()
                        .offset((j + 1 as libc::c_int) as isize)
                        as *mut NRPendingTraining as *const libc::c_void,
                    ((NRPendingTrainingCount - j) as libc::c_ulong)
                        .wrapping_mul(
                            ::std::mem::size_of::<NRPendingTraining>() as libc::c_ulong,
                        ),
                );
            }
            if orig_id != (*pt).db_id {
                RedisModule_SelectDb.unwrap()(ctx, orig_id);
            }
            collected += 1;
            collected;
        }
        j += 1;
        j;
    }
    pthread_mutex_unlock(&mut NRPendingTrainingMutex);
    return collected;
}
pub unsafe extern "C" fn NRCreate_RedisCommand(
    mut ctx: *mut RedisModuleCtx,
    mut argv: *mut *mut RedisModuleString,
    mut argc: libc::c_int,
) -> libc::c_int {
    let mut dset_size: libc::c_longlong = 0 as libc::c_int as libc::c_longlong;
    let mut test_size: libc::c_longlong = 0 as libc::c_int as libc::c_longlong;
    let mut layers: [libc::c_int; 32] = [0; 32];
    let mut num_layers: libc::c_int = 0 as libc::c_int;
    let mut flags: libc::c_int = 0 as libc::c_int;
    RedisModule_AutoMemory.unwrap()(ctx);
    NRCollectThreads(ctx);
    if argc < 6 as libc::c_int {
        return RedisModule_WrongArity.unwrap()(ctx);
    }
    let mut nntype: *const libc::c_char = RedisModule_StringPtrLen
        .unwrap()(*argv.offset(2 as libc::c_int as isize), 0 as *mut size_t);
    if strcasecmp(nntype, b"classifier\0" as *const u8 as *const libc::c_char) == 0 {
        flags |= (1 as libc::c_int) << 2 as libc::c_int;
    } else if strcasecmp(nntype, b"regressor\0" as *const u8 as *const libc::c_char) == 0
    {
        flags |= (1 as libc::c_int) << 1 as libc::c_int;
    } else {
        return RedisModule_ReplyWithError
            .unwrap()(
            ctx,
            b"ERR invalid neural network type. Must be CLASSIFIER or REGRESSOR\0"
                as *const u8 as *const libc::c_char,
        )
    }
    let mut j: libc::c_int = 3 as libc::c_int;
    let mut stop: libc::c_int = 0 as libc::c_int;
    while j < argc {
        let mut u: *const libc::c_char = RedisModule_StringPtrLen
            .unwrap()(*argv.offset(j as isize), 0 as *mut size_t);
        let mut units: libc::c_longlong = 0;
        if strcmp(u, b"->\0" as *const u8 as *const libc::c_char) == 0 {
            stop = 1 as libc::c_int;
            j += 1;
            j;
        } else {
            if RedisModule_StringToLongLong
                .unwrap()(*argv.offset(j as isize), &mut units) != 0 as libc::c_int
                || units <= 0 as libc::c_int as libc::c_longlong
            {
                return RedisModule_ReplyWithError
                    .unwrap()(
                    ctx,
                    b"ERR invalid units count\0" as *const u8 as *const libc::c_char,
                );
            }
            let fresh4 = num_layers;
            num_layers = num_layers + 1;
            layers[fresh4 as usize] = units as libc::c_int;
            j += 1;
            j;
            if stop != 0 {
                break;
            }
        }
    }
    let mut i: libc::c_int = 0 as libc::c_int;
    while i < num_layers / 2 as libc::c_int {
        let mut t: libc::c_int = layers[i as usize];
        layers[i as usize] = layers[(num_layers - 1 as libc::c_int - i) as usize];
        layers[(num_layers - 1 as libc::c_int - i) as usize] = t;
        i += 1;
        i;
    }
    while j < argc {
        let mut o: *const libc::c_char = RedisModule_StringPtrLen
            .unwrap()(*argv.offset(j as isize), 0 as *mut size_t);
        let mut v: libc::c_longlong = 0;
        let mut lastarg: libc::c_int = (j == argc - 1 as libc::c_int) as libc::c_int;
        if (strcasecmp(o, b"dataset\0" as *const u8 as *const libc::c_char) == 0
            || strcasecmp(o, b"test\0" as *const u8 as *const libc::c_char) == 0)
            && lastarg == 0
        {
            if RedisModule_StringToLongLong
                .unwrap()(*argv.offset((j + 1 as libc::c_int) as isize), &mut v)
                != 0 as libc::c_int || v < 0 as libc::c_int as libc::c_longlong
            {
                return RedisModule_ReplyWithError
                    .unwrap()(
                    ctx,
                    b"ERR invalid dataset size\0" as *const u8 as *const libc::c_char,
                );
            }
            if strcasecmp(o, b"dataset\0" as *const u8 as *const libc::c_char) == 0 {
                dset_size = v;
            } else {
                test_size = v;
            }
            j += 1;
            j;
        } else if strcasecmp(o, b"normalize\0" as *const u8 as *const libc::c_char) == 0
        {
            flags |= (1 as libc::c_int) << 3 as libc::c_int;
        } else {
            return RedisModule_ReplyWithError
                .unwrap()(
                ctx,
                b"ERR Syntax error in NR.CREATE\0" as *const u8 as *const libc::c_char,
            )
        }
        j += 1;
        j;
    }
    let mut key: *mut RedisModuleKey = RedisModule_OpenKey
        .unwrap()(
        ctx,
        *argv.offset(1 as libc::c_int as isize),
        (1 as libc::c_int) << 0 as libc::c_int | (1 as libc::c_int) << 1 as libc::c_int,
    ) as *mut RedisModuleKey;
    let mut type_0: libc::c_int = RedisModule_KeyType.unwrap()(key);
    if type_0 != 0 as libc::c_int {
        return RedisModule_ReplyWithError
            .unwrap()(
            ctx,
            b"ERR the key name is busy\0" as *const u8 as *const libc::c_char,
        );
    }
    let mut nr: *mut NRTypeObject = createNRTypeObject(
        flags,
        layers.as_mut_ptr(),
        num_layers,
        dset_size as libc::c_int,
        test_size as libc::c_int,
    );
    RedisModule_ModuleTypeSetValue.unwrap()(key, NRType, nr as *mut libc::c_void);
    RedisModule_ReplyWithLongLong
        .unwrap()(ctx, AnnCountWeights((*nr).nn) as libc::c_longlong);
    RedisModule_ReplicateVerbatim.unwrap()(ctx);
    return 0 as libc::c_int;
}
pub unsafe extern "C" fn NRGenericRun_RedisCommand(
    mut ctx: *mut RedisModuleCtx,
    mut argv: *mut *mut RedisModuleString,
    mut argc: libc::c_int,
    mut output_class: libc::c_int,
) -> libc::c_int {
    RedisModule_AutoMemory.unwrap()(ctx);
    NRCollectThreads(ctx);
    if argc < 3 as libc::c_int {
        return RedisModule_WrongArity.unwrap()(ctx);
    }
    let mut key: *mut RedisModuleKey = RedisModule_OpenKey
        .unwrap()(
        ctx,
        *argv.offset(1 as libc::c_int as isize),
        (1 as libc::c_int) << 0 as libc::c_int,
    ) as *mut RedisModuleKey;
    if RedisModule_ModuleTypeGetType.unwrap()(key) != NRType {
        return RedisModule_ReplyWithError
            .unwrap()(
            ctx,
            b"WRONGTYPE Operation against a key holding the wrong kind of value\0"
                as *const u8 as *const libc::c_char,
        );
    }
    let mut nr: *mut NRTypeObject = RedisModule_ModuleTypeGetValue.unwrap()(key)
        as *mut NRTypeObject;
    if output_class != 0
        && (*nr).flags & ((1 as libc::c_int) << 2 as libc::c_int) as libc::c_uint == 0
    {
        return RedisModule_ReplyWithError
            .unwrap()(
            ctx,
            b"ERR you can't call NR.CLASS with a regressor network. Use this command with a classifier network\0"
                as *const u8 as *const libc::c_char,
        );
    }
    let mut ilen: libc::c_int = (*((*(*nr).nn).layer)
        .offset(((*(*nr).nn).layers - 1 as libc::c_int) as isize))
        .units - 1 as libc::c_int;
    if argc != ilen + 2 as libc::c_int {
        return RedisModule_ReplyWithError
            .unwrap()(
            ctx,
            b"ERR number of arguments does not match the number of inputs in the neural network\0"
                as *const u8 as *const libc::c_char,
        );
    }
    let mut j: libc::c_int = 0 as libc::c_int;
    while j < ilen {
        let mut input: libc::c_double = 0.;
        if RedisModule_StringToDouble
            .unwrap()(*argv.offset((j + 2 as libc::c_int) as isize), &mut input)
            != 0 as libc::c_int
        {
            return RedisModule_ReplyWithError
                .unwrap()(
                ctx,
                b"ERR invalid neural network input: must be a valid float precision floating point number\0"
                    as *const u8 as *const libc::c_char,
            );
        }
        if (*nr).flags & ((1 as libc::c_int) << 3 as libc::c_int) as libc::c_uint != 0 {
            input /= *((*nr).inorm).offset(j as isize) as libc::c_double;
        }
        *((*((*(*nr).nn).layer).offset(((*(*nr).nn).layers - 1 as libc::c_int) as isize))
            .output)
            .offset(j as isize) = input as libc::c_float;
        j += 1;
        j;
    }
    AnnSimulate((*nr).nn);
    let mut olen: libc::c_int = (*((*(*nr).nn).layer).offset(0 as libc::c_int as isize))
        .units;
    if output_class != 0 {
        let mut max: libc::c_float = *((*((*(*nr).nn).layer)
            .offset(0 as libc::c_int as isize))
            .output)
            .offset(0 as libc::c_int as isize);
        let mut max_class: libc::c_int = 0 as libc::c_int;
        let mut j_0: libc::c_int = 1 as libc::c_int;
        while j_0 < olen {
            let mut output: libc::c_float = *((*((*(*nr).nn).layer)
                .offset(0 as libc::c_int as isize))
                .output)
                .offset(j_0 as isize);
            if output > max {
                max = output;
                max_class = j_0;
            }
            j_0 += 1;
            j_0;
        }
        RedisModule_ReplyWithLongLong.unwrap()(ctx, max_class as libc::c_longlong);
    } else {
        RedisModule_ReplyWithArray.unwrap()(ctx, olen as libc::c_long);
        let mut j_1: libc::c_int = 0 as libc::c_int;
        while j_1 < olen {
            let mut output_0: libc::c_float = *((*((*(*nr).nn).layer)
                .offset(0 as libc::c_int as isize))
                .output)
                .offset(j_1 as isize);
            if (*nr).flags & ((1 as libc::c_int) << 2 as libc::c_int) as libc::c_uint
                == 0
                && (*nr).flags & ((1 as libc::c_int) << 3 as libc::c_int) as libc::c_uint
                    != 0
            {
                output_0 *= *((*nr).onorm).offset(j_1 as isize);
            }
            RedisModule_ReplyWithDouble.unwrap()(ctx, output_0 as libc::c_double);
            j_1 += 1;
            j_1;
        }
    }
    return 0 as libc::c_int;
}
pub unsafe extern "C" fn NRRun_RedisCommand(
    mut ctx: *mut RedisModuleCtx,
    mut argv: *mut *mut RedisModuleString,
    mut argc: libc::c_int,
) -> libc::c_int {
    return NRGenericRun_RedisCommand(ctx, argv, argc, 0 as libc::c_int);
}
pub unsafe extern "C" fn NRClass_RedisCommand(
    mut ctx: *mut RedisModuleCtx,
    mut argv: *mut *mut RedisModuleString,
    mut argc: libc::c_int,
) -> libc::c_int {
    return NRGenericRun_RedisCommand(ctx, argv, argc, 1 as libc::c_int);
}
pub unsafe extern "C" fn NRObserve_RedisCommand(
    mut ctx: *mut RedisModuleCtx,
    mut argv: *mut *mut RedisModuleString,
    mut argc: libc::c_int,
) -> libc::c_int {
    RedisModule_AutoMemory.unwrap()(ctx);
    NRCollectThreads(ctx);
    if argc < 3 as libc::c_int {
        return RedisModule_WrongArity.unwrap()(ctx);
    }
    let mut key: *mut RedisModuleKey = RedisModule_OpenKey
        .unwrap()(
        ctx,
        *argv.offset(1 as libc::c_int as isize),
        (1 as libc::c_int) << 0 as libc::c_int | (1 as libc::c_int) << 1 as libc::c_int,
    ) as *mut RedisModuleKey;
    if RedisModule_ModuleTypeGetType.unwrap()(key) != NRType {
        return RedisModule_ReplyWithError
            .unwrap()(
            ctx,
            b"WRONGTYPE Operation against a key holding the wrong kind of value\0"
                as *const u8 as *const libc::c_char,
        );
    }
    let mut nr: *mut NRTypeObject = RedisModule_ModuleTypeGetValue.unwrap()(key)
        as *mut NRTypeObject;
    let mut ilen: libc::c_int = (*((*(*nr).nn).layer)
        .offset(((*(*nr).nn).layers - 1 as libc::c_int) as isize))
        .units - 1 as libc::c_int;
    let mut olen: libc::c_int = (*((*(*nr).nn).layer).offset(0 as libc::c_int as isize))
        .units;
    let mut oargs: libc::c_int = if (*nr).flags
        & ((1 as libc::c_int) << 2 as libc::c_int) as libc::c_uint != 0
    {
        1 as libc::c_int
    } else {
        olen
    };
    let mut target: libc::c_int = 0 as libc::c_int;
    if strcasecmp(
        RedisModule_StringPtrLen
            .unwrap()(
            *argv.offset((argc - 1 as libc::c_int) as isize),
            0 as *mut size_t,
        ),
        b"train\0" as *const u8 as *const libc::c_char,
    ) == 0
    {
        target = 1 as libc::c_int;
        argc -= 1;
        argc;
    } else if strcasecmp(
        RedisModule_StringPtrLen
            .unwrap()(
            *argv.offset((argc - 1 as libc::c_int) as isize),
            0 as *mut size_t,
        ),
        b"test\0" as *const u8 as *const libc::c_char,
    ) == 0
    {
        target = 2 as libc::c_int;
        argc -= 1;
        argc;
    }
    if argc != oargs + ilen + 3 as libc::c_int {
        return RedisModule_ReplyWithError
            .unwrap()(
            ctx,
            b"ERR number of arguments does not match the number of inputs and outputs in the neural network\0"
                as *const u8 as *const libc::c_char,
        );
    }
    let mut sep: *const libc::c_char = RedisModule_StringPtrLen
        .unwrap()(*argv.offset((ilen + 2 as libc::c_int) as isize), 0 as *mut size_t);
    if strcmp(sep, b"->\0" as *const u8 as *const libc::c_char) != 0 {
        return RedisModule_ReplyWithError
            .unwrap()(
            ctx,
            b"ERR no '->' separtor in the correct position between inputs and outputs: are you sure your training data is correct?\0"
                as *const u8 as *const libc::c_char,
        );
    }
    let mut inputs: *mut libc::c_float = RedisModule_Alloc
        .unwrap()(
        (::std::mem::size_of::<libc::c_float>() as libc::c_ulong)
            .wrapping_mul(ilen as libc::c_ulong),
    ) as *mut libc::c_float;
    let mut outputs: *mut libc::c_float = RedisModule_Alloc
        .unwrap()(
        (::std::mem::size_of::<libc::c_float>() as libc::c_ulong)
            .wrapping_mul(olen as libc::c_ulong),
    ) as *mut libc::c_float;
    let mut j: libc::c_int = 2 as libc::c_int;
    while j < argc {
        let mut val: libc::c_double = 0.;
        if !(j == ilen + 2 as libc::c_int) {
            if RedisModule_StringToDouble.unwrap()(*argv.offset(j as isize), &mut val)
                != 0 as libc::c_int
            {
                RedisModule_Free.unwrap()(inputs as *mut libc::c_void);
                RedisModule_Free.unwrap()(outputs as *mut libc::c_void);
                return RedisModule_ReplyWithError
                    .unwrap()(
                    ctx,
                    b"ERR invalid neural network input: must be a valid float precision floating point number\0"
                        as *const u8 as *const libc::c_char,
                );
            }
            if j < ilen + 2 as libc::c_int {
                *inputs.offset((j - 2 as libc::c_int) as isize) = val as libc::c_float;
            } else if (*nr).flags
                & ((1 as libc::c_int) << 2 as libc::c_int) as libc::c_uint != 0
            {
                let mut classid: libc::c_int = val as libc::c_int;
                if classid as libc::c_double != val || val >= olen as libc::c_double
                    || val < 0 as libc::c_int as libc::c_double
                {
                    RedisModule_Free.unwrap()(inputs as *mut libc::c_void);
                    RedisModule_Free.unwrap()(outputs as *mut libc::c_void);
                    return RedisModule_ReplyWithError
                        .unwrap()(
                        ctx,
                        b"ERR classifier network output must be an integer in the range from 0 to outputs-1.\0"
                            as *const u8 as *const libc::c_char,
                    );
                }
                memset(
                    outputs as *mut libc::c_void,
                    0 as libc::c_int,
                    (::std::mem::size_of::<libc::c_float>() as libc::c_ulong)
                        .wrapping_mul(olen as libc::c_ulong),
                );
                *outputs.offset(classid as isize) = 1 as libc::c_int as libc::c_float;
            } else {
                *outputs
                    .offset(
                        (j - ilen - 3 as libc::c_int) as isize,
                    ) = val as libc::c_float;
            }
        }
        j += 1;
        j;
    }
    NRTypeInsertData(nr, inputs, outputs, target);
    RedisModule_Free.unwrap()(inputs as *mut libc::c_void);
    RedisModule_Free.unwrap()(outputs as *mut libc::c_void);
    RedisModule_ReplyWithArray.unwrap()(ctx, 2 as libc::c_int as libc::c_long);
    RedisModule_ReplyWithLongLong.unwrap()(ctx, (*nr).dataset.len as libc::c_longlong);
    RedisModule_ReplyWithLongLong.unwrap()(ctx, (*nr).test.len as libc::c_longlong);
    return 0 as libc::c_int;
}
pub unsafe extern "C" fn NRTrain_RedisCommand(
    mut ctx: *mut RedisModuleCtx,
    mut argv: *mut *mut RedisModuleString,
    mut argc: libc::c_int,
) -> libc::c_int {
    RedisModule_AutoMemory.unwrap()(ctx);
    NRCollectThreads(ctx);
    if argc < 2 as libc::c_int {
        return RedisModule_WrongArity.unwrap()(ctx);
    }
    let mut key: *mut RedisModuleKey = RedisModule_OpenKey
        .unwrap()(
        ctx,
        *argv.offset(1 as libc::c_int as isize),
        (1 as libc::c_int) << 0 as libc::c_int | (1 as libc::c_int) << 1 as libc::c_int,
    ) as *mut RedisModuleKey;
    if RedisModule_ModuleTypeGetType.unwrap()(key) != NRType {
        return RedisModule_ReplyWithError
            .unwrap()(
            ctx,
            b"WRONGTYPE Operation against a key holding the wrong kind of value\0"
                as *const u8 as *const libc::c_char,
        );
    }
    let mut nr: *mut NRTypeObject = RedisModule_ModuleTypeGetValue.unwrap()(key)
        as *mut NRTypeObject;
    if (*nr).flags & ((1 as libc::c_int) << 0 as libc::c_int) as libc::c_uint != 0 {
        return RedisModule_ReplyWithError
            .unwrap()(
            ctx,
            b"ERR neural network training already in progress\0" as *const u8
                as *const libc::c_char,
        );
    }
    (*nr).training_max_cycles = 0 as libc::c_int as uint64_t;
    (*nr).training_max_ms = 10000 as libc::c_int as uint64_t;
    (*nr).flags
        &= !((1 as libc::c_int) << 4 as libc::c_int
            | (1 as libc::c_int) << 6 as libc::c_int) as libc::c_uint;
    let mut j: libc::c_int = 2 as libc::c_int;
    while j < argc {
        let mut o: *const libc::c_char = RedisModule_StringPtrLen
            .unwrap()(*argv.offset(j as isize), 0 as *mut size_t);
        let mut v: libc::c_longlong = 0;
        let mut lastarg: libc::c_int = (j == argc - 1 as libc::c_int) as libc::c_int;
        if strcasecmp(o, b"autostop\0" as *const u8 as *const libc::c_char) == 0 {
            (*nr).flags |= ((1 as libc::c_int) << 4 as libc::c_int) as libc::c_uint;
        } else if strcasecmp(o, b"backtrack\0" as *const u8 as *const libc::c_char) == 0
        {
            (*nr).flags |= ((1 as libc::c_int) << 6 as libc::c_int) as libc::c_uint;
        } else if strcasecmp(o, b"maxcycles\0" as *const u8 as *const libc::c_char) == 0
            && lastarg == 0
        {
            j += 1;
            if RedisModule_StringToLongLong.unwrap()(*argv.offset(j as isize), &mut v)
                != 0 as libc::c_int
            {
                return RedisModule_ReplyWithError
                    .unwrap()(
                    ctx,
                    b"ERR invalid number of cycles\0" as *const u8 as *const libc::c_char,
                );
            }
            (*nr).training_max_cycles = v as uint64_t;
        } else if strcasecmp(o, b"maxtime\0" as *const u8 as *const libc::c_char) == 0
            && lastarg == 0
        {
            j += 1;
            if RedisModule_StringToLongLong.unwrap()(*argv.offset(j as isize), &mut v)
                != 0 as libc::c_int
            {
                return RedisModule_ReplyWithError
                    .unwrap()(
                    ctx,
                    b"ERR invalid number of milliseconds of time\0" as *const u8
                        as *const libc::c_char,
                );
            }
            (*nr).training_max_ms = v as uint64_t;
        } else {
            return RedisModule_ReplyWithError
                .unwrap()(
                ctx,
                b"ERR Syntax error in NR.TRAIN\0" as *const u8 as *const libc::c_char,
            )
        }
        j += 1;
        j;
    }
    if (*nr).flags & ((1 as libc::c_int) << 4 as libc::c_int) as libc::c_uint != 0
        && (*nr).test.len == 0 as libc::c_int as libc::c_uint
    {
        return RedisModule_ReplyWithError
            .unwrap()(
            ctx,
            b"ERR Can't start training with AUTOSTOP option: overfitting detection requires a non zero length testing dataset\0"
                as *const u8 as *const libc::c_char,
        );
    }
    if NRStartTraining(
        ctx,
        *argv.offset(1 as libc::c_int as isize),
        RedisModule_GetSelectedDb.unwrap()(ctx),
        nr,
    ) == 1 as libc::c_int
    {
        return RedisModule_ReplyWithError
            .unwrap()(
            ctx,
            b"ERR Can't train the neural network: too many NNs already training\0"
                as *const u8 as *const libc::c_char,
        )
    } else {
        return RedisModule_ReplyWithSimpleString
            .unwrap()(ctx, b"Training has started\0" as *const u8 as *const libc::c_char)
    };
}
pub unsafe extern "C" fn NRReset_RedisCommand(
    mut ctx: *mut RedisModuleCtx,
    mut argv: *mut *mut RedisModuleString,
    mut argc: libc::c_int,
) -> libc::c_int {
    RedisModule_AutoMemory.unwrap()(ctx);
    NRCollectThreads(ctx);
    if argc != 2 as libc::c_int {
        return RedisModule_WrongArity.unwrap()(ctx);
    }
    let mut key: *mut RedisModuleKey = RedisModule_OpenKey
        .unwrap()(
        ctx,
        *argv.offset(1 as libc::c_int as isize),
        (1 as libc::c_int) << 0 as libc::c_int | (1 as libc::c_int) << 1 as libc::c_int,
    ) as *mut RedisModuleKey;
    if RedisModule_ModuleTypeGetType.unwrap()(key) != NRType {
        return RedisModule_ReplyWithError
            .unwrap()(
            ctx,
            b"WRONGTYPE Operation against a key holding the wrong kind of value\0"
                as *const u8 as *const libc::c_char,
        );
    }
    let mut nr: *mut NRTypeObject = RedisModule_ModuleTypeGetValue.unwrap()(key)
        as *mut NRTypeObject;
    let fresh5 = NRNextId;
    NRNextId = NRNextId.wrapping_add(1);
    (*nr).id = fresh5;
    (*nr).training_total_steps = 0 as libc::c_int as uint64_t;
    (*nr).training_total_ms = 0 as libc::c_int as uint64_t;
    (*nr).training_max_cycles = 0 as libc::c_int as uint64_t;
    (*nr).training_max_ms = 0 as libc::c_int as uint64_t;
    (*nr).dataset_error = 0 as libc::c_int as libc::c_float;
    (*nr).test_error = 0 as libc::c_int as libc::c_float;
    (*nr).test_class_error = 0 as libc::c_int as libc::c_float;
    AnnSetRandomWeights((*nr).nn);
    return RedisModule_ReplyWithSimpleString
        .unwrap()(ctx, b"OK\0" as *const u8 as *const libc::c_char);
}
pub unsafe extern "C" fn NRInfo_RedisCommand(
    mut ctx: *mut RedisModuleCtx,
    mut argv: *mut *mut RedisModuleString,
    mut argc: libc::c_int,
) -> libc::c_int {
    let mut buf: [libc::c_char; 128] = [0; 128];
    RedisModule_AutoMemory.unwrap()(ctx);
    NRCollectThreads(ctx);
    if argc != 2 as libc::c_int {
        return RedisModule_WrongArity.unwrap()(ctx);
    }
    let mut key: *mut RedisModuleKey = RedisModule_OpenKey
        .unwrap()(
        ctx,
        *argv.offset(1 as libc::c_int as isize),
        (1 as libc::c_int) << 0 as libc::c_int,
    ) as *mut RedisModuleKey;
    if RedisModule_ModuleTypeGetType.unwrap()(key) != NRType {
        return RedisModule_ReplyWithError
            .unwrap()(
            ctx,
            b"WRONGTYPE Operation against a key holding the wrong kind of value\0"
                as *const u8 as *const libc::c_char,
        );
    }
    let mut nr: *mut NRTypeObject = RedisModule_ModuleTypeGetValue.unwrap()(key)
        as *mut NRTypeObject;
    let mut fields: libc::c_int = 15 as libc::c_int;
    if (*nr).flags & ((1 as libc::c_int) << 2 as libc::c_int) as libc::c_uint != 0 {
        fields += 1;
        fields;
    }
    RedisModule_ReplyWithArray
        .unwrap()(ctx, (fields * 2 as libc::c_int) as libc::c_long);
    RedisModule_ReplyWithSimpleString
        .unwrap()(ctx, b"id\0" as *const u8 as *const libc::c_char);
    RedisModule_ReplyWithLongLong.unwrap()(ctx, (*nr).id as libc::c_longlong);
    RedisModule_ReplyWithSimpleString
        .unwrap()(ctx, b"type\0" as *const u8 as *const libc::c_char);
    RedisModule_ReplyWithSimpleString
        .unwrap()(
        ctx,
        if (*nr).flags & ((1 as libc::c_int) << 2 as libc::c_int) as libc::c_uint != 0 {
            b"classifier\0" as *const u8 as *const libc::c_char
        } else {
            b"regressor\0" as *const u8 as *const libc::c_char
        },
    );
    RedisModule_ReplyWithSimpleString
        .unwrap()(ctx, b"auto-normalization\0" as *const u8 as *const libc::c_char);
    RedisModule_ReplyWithLongLong
        .unwrap()(
        ctx,
        ((*nr).flags & ((1 as libc::c_int) << 3 as libc::c_int) as libc::c_uint != 0)
            as libc::c_int as libc::c_longlong,
    );
    RedisModule_ReplyWithSimpleString
        .unwrap()(ctx, b"training\0" as *const u8 as *const libc::c_char);
    RedisModule_ReplyWithLongLong
        .unwrap()(
        ctx,
        ((*nr).flags & ((1 as libc::c_int) << 0 as libc::c_int) as libc::c_uint != 0)
            as libc::c_int as libc::c_longlong,
    );
    RedisModule_ReplyWithSimpleString
        .unwrap()(ctx, b"layout\0" as *const u8 as *const libc::c_char);
    RedisModule_ReplyWithArray.unwrap()(ctx, (*(*nr).nn).layers as libc::c_long);
    let mut i: libc::c_int = (*(*nr).nn).layers - 1 as libc::c_int;
    while i >= 0 as libc::c_int {
        let mut units: libc::c_int = (*((*(*nr).nn).layer).offset(i as isize)).units;
        if i != 0 as libc::c_int {
            units -= 1;
            units;
        }
        RedisModule_ReplyWithLongLong.unwrap()(ctx, units as libc::c_longlong);
        i -= 1;
        i;
    }
    RedisModule_ReplyWithSimpleString
        .unwrap()(ctx, b"training-dataset-maxlen\0" as *const u8 as *const libc::c_char);
    RedisModule_ReplyWithLongLong
        .unwrap()(ctx, (*nr).dataset.maxlen as libc::c_longlong);
    RedisModule_ReplyWithSimpleString
        .unwrap()(ctx, b"training-dataset-len\0" as *const u8 as *const libc::c_char);
    RedisModule_ReplyWithLongLong.unwrap()(ctx, (*nr).dataset.len as libc::c_longlong);
    RedisModule_ReplyWithSimpleString
        .unwrap()(ctx, b"test-dataset-maxlen\0" as *const u8 as *const libc::c_char);
    RedisModule_ReplyWithLongLong.unwrap()(ctx, (*nr).test.maxlen as libc::c_longlong);
    RedisModule_ReplyWithSimpleString
        .unwrap()(ctx, b"test-dataset-len\0" as *const u8 as *const libc::c_char);
    RedisModule_ReplyWithLongLong.unwrap()(ctx, (*nr).test.len as libc::c_longlong);
    RedisModule_ReplyWithSimpleString
        .unwrap()(ctx, b"training-total-steps\0" as *const u8 as *const libc::c_char);
    RedisModule_ReplyWithLongLong
        .unwrap()(ctx, (*nr).training_total_steps as libc::c_longlong);
    RedisModule_ReplyWithSimpleString
        .unwrap()(ctx, b"training-total-cycles\0" as *const u8 as *const libc::c_char);
    RedisModule_ReplyWithLongLong
        .unwrap()(
        ctx,
        (if (*nr).dataset.len != 0 {
            ((*nr).training_total_steps).wrapping_div((*nr).dataset.len as libc::c_ulong)
        } else {
            0 as libc::c_int as libc::c_ulong
        }) as libc::c_longlong,
    );
    RedisModule_ReplyWithSimpleString
        .unwrap()(ctx, b"training-total-seconds\0" as *const u8 as *const libc::c_char);
    snprintf(
        buf.as_mut_ptr(),
        ::std::mem::size_of::<[libc::c_char; 128]>() as libc::c_ulong,
        b"%.02f\0" as *const u8 as *const libc::c_char,
        ((*nr).training_total_ms as libc::c_float / 1000 as libc::c_int as libc::c_float)
            as libc::c_double,
    );
    RedisModule_ReplyWithSimpleString.unwrap()(ctx, buf.as_mut_ptr());
    RedisModule_ReplyWithSimpleString
        .unwrap()(ctx, b"dataset-error\0" as *const u8 as *const libc::c_char);
    RedisModule_ReplyWithDouble.unwrap()(ctx, (*nr).dataset_error as libc::c_double);
    RedisModule_ReplyWithSimpleString
        .unwrap()(ctx, b"test-error\0" as *const u8 as *const libc::c_char);
    RedisModule_ReplyWithDouble.unwrap()(ctx, (*nr).test_error as libc::c_double);
    if (*nr).flags & ((1 as libc::c_int) << 2 as libc::c_int) as libc::c_uint != 0 {
        RedisModule_ReplyWithSimpleString
            .unwrap()(
            ctx,
            b"classification-errors-perc\0" as *const u8 as *const libc::c_char,
        );
        snprintf(
            buf.as_mut_ptr(),
            ::std::mem::size_of::<[libc::c_char; 128]>() as libc::c_ulong,
            b"%.02f\0" as *const u8 as *const libc::c_char,
            (*nr).test_class_error as libc::c_double,
        );
        RedisModule_ReplyWithSimpleString.unwrap()(ctx, buf.as_mut_ptr());
    }
    RedisModule_ReplyWithSimpleString
        .unwrap()(ctx, b"overfitting-detected\0" as *const u8 as *const libc::c_char);
    RedisModule_ReplyWithSimpleString
        .unwrap()(
        ctx,
        if (*nr).flags & ((1 as libc::c_int) << 5 as libc::c_int) as libc::c_uint != 0 {
            b"yes\0" as *const u8 as *const libc::c_char
        } else {
            b"no\0" as *const u8 as *const libc::c_char
        },
    );
    return 0 as libc::c_int;
}
pub unsafe extern "C" fn NRThreads_RedisCommand(
    mut ctx: *mut RedisModuleCtx,
    mut argv: *mut *mut RedisModuleString,
    mut argc: libc::c_int,
) -> libc::c_int {
    RedisModule_AutoMemory.unwrap()(ctx);
    NRCollectThreads(ctx);
    if argc != 1 as libc::c_int {
        return RedisModule_WrongArity.unwrap()(ctx);
    }
    pthread_mutex_lock(&mut NRPendingTrainingMutex);
    RedisModule_ReplyWithArray.unwrap()(ctx, NRPendingTrainingCount as libc::c_long);
    let mut j: libc::c_int = 0 as libc::c_int;
    while j < NRPendingTrainingCount {
        let mut buf: [libc::c_char; 1024] = [0; 1024];
        let mut pt: *mut NRPendingTraining = &mut *NRTrainings
            .as_mut_ptr()
            .offset(j as isize) as *mut NRPendingTraining;
        let mut keyname: *const libc::c_char = RedisModule_StringPtrLen
            .unwrap()((*pt).key, 0 as *mut size_t);
        snprintf(
            buf.as_mut_ptr(),
            ::std::mem::size_of::<[libc::c_char; 1024]>() as libc::c_ulong,
            b"nn_id=%llu cycle=%d key=%s db=%d maxtime=%llu maxcycles=%llu trainerr=%f testerr=%f classerr=%f\0"
                as *const u8 as *const libc::c_char,
            (*(*pt).nr).id as libc::c_ulonglong,
            (*pt).curcycle,
            keyname,
            (*pt).db_id,
            (*(*pt).nr).training_max_ms as libc::c_ulonglong,
            (*(*pt).nr).training_max_cycles as libc::c_ulonglong,
            (*pt).dataset_error as libc::c_double,
            (*pt).test_error as libc::c_double,
            (*pt).class_error as libc::c_double,
        );
        RedisModule_ReplyWithSimpleString.unwrap()(ctx, buf.as_mut_ptr());
        j += 1;
        j;
    }
    pthread_mutex_unlock(&mut NRPendingTrainingMutex);
    return 0 as libc::c_int;
}
pub unsafe extern "C" fn NRGetdata_RedisCommand(
    mut ctx: *mut RedisModuleCtx,
    mut argv: *mut *mut RedisModuleString,
    mut argc: libc::c_int,
) -> libc::c_int {
    RedisModule_AutoMemory.unwrap()(ctx);
    NRCollectThreads(ctx);
    if argc != 4 as libc::c_int {
        return RedisModule_WrongArity.unwrap()(ctx);
    }
    let mut key: *mut RedisModuleKey = RedisModule_OpenKey
        .unwrap()(
        ctx,
        *argv.offset(1 as libc::c_int as isize),
        (1 as libc::c_int) << 0 as libc::c_int,
    ) as *mut RedisModuleKey;
    if RedisModule_ModuleTypeGetType.unwrap()(key) != NRType {
        return RedisModule_ReplyWithError
            .unwrap()(
            ctx,
            b"WRONGTYPE Operation against a key holding the wrong kind of value\0"
                as *const u8 as *const libc::c_char,
        );
    }
    let mut nr: *mut NRTypeObject = RedisModule_ModuleTypeGetValue.unwrap()(key)
        as *mut NRTypeObject;
    let mut ilen: libc::c_int = (*((*(*nr).nn).layer)
        .offset(((*(*nr).nn).layers - 1 as libc::c_int) as isize))
        .units - 1 as libc::c_int;
    let mut olen: libc::c_int = (*((*(*nr).nn).layer).offset(0 as libc::c_int as isize))
        .units;
    let mut target: *mut NRDataset = 0 as *mut NRDataset;
    let mut idx: libc::c_longlong = 0;
    if strcasecmp(
        RedisModule_StringPtrLen
            .unwrap()(*argv.offset(2 as libc::c_int as isize), 0 as *mut size_t),
        b"train\0" as *const u8 as *const libc::c_char,
    ) == 0
    {
        target = &mut (*nr).dataset;
    } else if strcasecmp(
        RedisModule_StringPtrLen
            .unwrap()(*argv.offset(2 as libc::c_int as isize), 0 as *mut size_t),
        b"test\0" as *const u8 as *const libc::c_char,
    ) == 0
    {
        target = &mut (*nr).test;
    } else {
        return RedisModule_ReplyWithError
            .unwrap()(
            ctx,
            b"ERR please specify as source either TRAIN or TEST\0" as *const u8
                as *const libc::c_char,
        )
    }
    if RedisModule_StringToLongLong
        .unwrap()(*argv.offset(3 as libc::c_int as isize), &mut idx) != 0 as libc::c_int
        || idx < 0 as libc::c_int as libc::c_longlong
    {
        return RedisModule_ReplyWithError
            .unwrap()(
            ctx,
            b"ERR invalid row specified\0" as *const u8 as *const libc::c_char,
        )
    } else if idx >= (*target).maxlen as libc::c_longlong {
        return RedisModule_ReplyWithNull.unwrap()(ctx)
    }
    RedisModule_ReplyWithArray.unwrap()(ctx, 2 as libc::c_int as libc::c_long);
    RedisModule_ReplyWithArray.unwrap()(ctx, ilen as libc::c_long);
    let mut j: libc::c_int = 0 as libc::c_int;
    while j < ilen {
        let mut input: libc::c_double = *((*target).inputs)
            .offset((ilen as libc::c_longlong * idx + j as libc::c_longlong) as isize)
            as libc::c_double;
        RedisModule_ReplyWithDouble.unwrap()(ctx, input);
        j += 1;
        j;
    }
    RedisModule_ReplyWithArray.unwrap()(ctx, olen as libc::c_long);
    let mut j_0: libc::c_int = 0 as libc::c_int;
    while j_0 < olen {
        let mut output: libc::c_double = *((*target).outputs)
            .offset((olen as libc::c_longlong * idx + j_0 as libc::c_longlong) as isize)
            as libc::c_double;
        RedisModule_ReplyWithDouble.unwrap()(ctx, output);
        j_0 += 1;
        j_0;
    }
    return 0 as libc::c_int;
}
pub unsafe extern "C" fn NRTypeRdbSaveDataset(
    mut rdb: *mut RedisModuleIO,
    mut ds: *mut NRDataset,
    mut ilen: uint32_t,
    mut olen: uint32_t,
) {
    RedisModule_SaveUnsigned.unwrap()(rdb, (*ds).len as uint64_t);
    RedisModule_SaveUnsigned.unwrap()(rdb, (*ds).maxlen as uint64_t);
    let mut j: uint32_t = 0 as libc::c_int as uint32_t;
    while j < ilen.wrapping_mul((*ds).len) {
        RedisModule_SaveFloat.unwrap()(rdb, *((*ds).inputs).offset(j as isize));
        j = j.wrapping_add(1);
        j;
    }
    let mut j_0: uint32_t = 0 as libc::c_int as uint32_t;
    while j_0 < olen.wrapping_mul((*ds).len) {
        RedisModule_SaveFloat.unwrap()(rdb, *((*ds).outputs).offset(j_0 as isize));
        j_0 = j_0.wrapping_add(1);
        j_0;
    }
}
pub unsafe extern "C" fn NRTypeRdbSave(
    mut rdb: *mut RedisModuleIO,
    mut value: *mut libc::c_void,
) {
    let mut nr: *mut NRTypeObject = value as *mut NRTypeObject;
    RedisModule_SaveUnsigned.unwrap()(rdb, (*(*nr).nn).layers as uint64_t);
    let mut j: libc::c_int = 0 as libc::c_int;
    while j < (*(*nr).nn).layers {
        let mut units: libc::c_int = (*((*(*nr).nn).layer).offset(j as isize)).units;
        if j != 0 as libc::c_int {
            units -= 1;
            units;
        }
        RedisModule_SaveUnsigned.unwrap()(rdb, units as uint64_t);
        j += 1;
        j;
    }
    RedisModule_SaveUnsigned
        .unwrap()(
        rdb,
        ((*nr).flags
            & ((1 as libc::c_int) << 1 as libc::c_int
                | (1 as libc::c_int) << 2 as libc::c_int
                | (1 as libc::c_int) << 3 as libc::c_int
                | (1 as libc::c_int) << 5 as libc::c_int) as libc::c_uint) as uint64_t,
    );
    RedisModule_SaveUnsigned.unwrap()(rdb, (*nr).id);
    RedisModule_SaveUnsigned.unwrap()(rdb, (*nr).training_total_steps);
    RedisModule_SaveUnsigned.unwrap()(rdb, (*nr).training_total_ms);
    RedisModule_SaveUnsigned.unwrap()(rdb, (*nr).training_max_cycles);
    RedisModule_SaveUnsigned.unwrap()(rdb, (*nr).training_max_ms);
    RedisModule_SaveFloat.unwrap()(rdb, (*nr).dataset_error);
    RedisModule_SaveFloat.unwrap()(rdb, (*nr).test_error);
    RedisModule_SaveFloat.unwrap()(rdb, (*nr).test_class_error);
    let mut j_0: libc::c_int = 1 as libc::c_int;
    while j_0 < (*(*nr).nn).layers {
        let mut weights: libc::c_int = (*((*(*nr).nn).layer).offset(j_0 as isize)).units
            * (*((*(*nr).nn).layer).offset((j_0 - 1 as libc::c_int) as isize)).units;
        let mut i: libc::c_int = 0 as libc::c_int;
        while i < weights {
            RedisModule_SaveFloat
                .unwrap()(
                rdb,
                *((*((*(*nr).nn).layer).offset(j_0 as isize)).weight).offset(i as isize),
            );
            i += 1;
            i;
        }
        let mut i_0: libc::c_int = 0 as libc::c_int;
        while i_0 < weights {
            RedisModule_SaveFloat
                .unwrap()(
                rdb,
                *((*((*(*nr).nn).layer).offset(j_0 as isize)).delta).offset(i_0 as isize),
            );
            i_0 += 1;
            i_0;
        }
        let mut i_1: libc::c_int = 0 as libc::c_int;
        while i_1 < weights {
            RedisModule_SaveFloat
                .unwrap()(
                rdb,
                *((*((*(*nr).nn).layer).offset(j_0 as isize)).pgradient)
                    .offset(i_1 as isize),
            );
            i_1 += 1;
            i_1;
        }
        j_0 += 1;
        j_0;
    }
    let mut ilen: uint32_t = ((*((*(*nr).nn).layer)
        .offset(((*(*nr).nn).layers - 1 as libc::c_int) as isize))
        .units - 1 as libc::c_int) as uint32_t;
    let mut olen: uint32_t = (*((*(*nr).nn).layer).offset(0 as libc::c_int as isize))
        .units as uint32_t;
    let mut j_1: uint32_t = 0 as libc::c_int as uint32_t;
    while j_1 < ilen {
        RedisModule_SaveFloat.unwrap()(rdb, *((*nr).inorm).offset(j_1 as isize));
        j_1 = j_1.wrapping_add(1);
        j_1;
    }
    let mut j_2: uint32_t = 0 as libc::c_int as uint32_t;
    while j_2 < olen {
        RedisModule_SaveFloat.unwrap()(rdb, *((*nr).onorm).offset(j_2 as isize));
        j_2 = j_2.wrapping_add(1);
        j_2;
    }
    NRTypeRdbSaveDataset(rdb, &mut (*nr).dataset, ilen, olen);
    NRTypeRdbSaveDataset(rdb, &mut (*nr).test, ilen, olen);
}
pub unsafe extern "C" fn NRTypeRdbLoadDataset(
    mut rdb: *mut RedisModuleIO,
    mut ds: *mut NRDataset,
    mut ilen: uint32_t,
    mut olen: uint32_t,
) {
    (*ds).len = RedisModule_LoadUnsigned.unwrap()(rdb) as uint32_t;
    (*ds).maxlen = RedisModule_LoadUnsigned.unwrap()(rdb) as uint32_t;
    if (*ds).len == 0 as libc::c_int as libc::c_uint {
        return;
    }
    (*ds)
        .inputs = RedisModule_Alloc
        .unwrap()(
        (ilen.wrapping_mul((*ds).len) as libc::c_ulong)
            .wrapping_mul(::std::mem::size_of::<libc::c_float>() as libc::c_ulong),
    ) as *mut libc::c_float;
    (*ds)
        .outputs = RedisModule_Alloc
        .unwrap()(
        (olen.wrapping_mul((*ds).len) as libc::c_ulong)
            .wrapping_mul(::std::mem::size_of::<libc::c_float>() as libc::c_ulong),
    ) as *mut libc::c_float;
    let mut j: uint32_t = 0 as libc::c_int as uint32_t;
    while j < ilen.wrapping_mul((*ds).len) {
        *((*ds).inputs).offset(j as isize) = RedisModule_LoadFloat.unwrap()(rdb);
        j = j.wrapping_add(1);
        j;
    }
    let mut j_0: uint32_t = 0 as libc::c_int as uint32_t;
    while j_0 < olen.wrapping_mul((*ds).len) {
        *((*ds).outputs).offset(j_0 as isize) = RedisModule_LoadFloat.unwrap()(rdb);
        j_0 = j_0.wrapping_add(1);
        j_0;
    }
}
pub unsafe extern "C" fn NRTypeRdbLoad(
    mut rdb: *mut RedisModuleIO,
    mut encver: libc::c_int,
) -> *mut libc::c_void {
    if encver != 2 as libc::c_int {
        RedisModule_LogIOError
            .unwrap()(
            rdb,
            b"warning\0" as *const u8 as *const libc::c_char,
            b"Sorry the Neural Redis module only supports RDB files written with the encoding version %d. This file has encoding version %d, and was likely written by a previous version of this module that is now deprecated. Once the module will be stable we'll start supporting older versions of the encodings, in case we switch to newer encodings.\0"
                as *const u8 as *const libc::c_char,
            2 as libc::c_int,
            encver,
        );
        return 0 as *mut libc::c_void;
    }
    let mut numlayers: uint64_t = RedisModule_LoadUnsigned.unwrap()(rdb);
    let mut layers: *mut libc::c_int = RedisModule_Alloc
        .unwrap()(
        (::std::mem::size_of::<libc::c_int>() as libc::c_ulong).wrapping_mul(numlayers),
    ) as *mut libc::c_int;
    let mut j: uint32_t = 0 as libc::c_int as uint32_t;
    while (j as libc::c_ulong) < numlayers {
        *layers
            .offset(j as isize) = RedisModule_LoadUnsigned.unwrap()(rdb) as libc::c_int;
        j = j.wrapping_add(1);
        j;
    }
    let mut flags: uint32_t = RedisModule_LoadUnsigned.unwrap()(rdb) as uint32_t;
    let mut nr: *mut NRTypeObject = createNRTypeObject(
        flags as libc::c_int,
        layers,
        numlayers as libc::c_int,
        0 as libc::c_int,
        0 as libc::c_int,
    );
    RedisModule_Free.unwrap()(layers as *mut libc::c_void);
    (*nr).id = RedisModule_LoadUnsigned.unwrap()(rdb);
    (*nr).training_total_steps = RedisModule_LoadUnsigned.unwrap()(rdb);
    (*nr).training_total_ms = RedisModule_LoadUnsigned.unwrap()(rdb);
    (*nr).training_max_cycles = RedisModule_LoadUnsigned.unwrap()(rdb);
    (*nr).training_max_ms = RedisModule_LoadUnsigned.unwrap()(rdb);
    (*nr).dataset_error = RedisModule_LoadFloat.unwrap()(rdb);
    (*nr).test_error = RedisModule_LoadFloat.unwrap()(rdb);
    (*nr).test_class_error = RedisModule_LoadFloat.unwrap()(rdb);
    let mut j_0: libc::c_int = 1 as libc::c_int;
    while j_0 < (*(*nr).nn).layers {
        let mut weights: libc::c_int = (*((*(*nr).nn).layer).offset(j_0 as isize)).units
            * (*((*(*nr).nn).layer).offset((j_0 - 1 as libc::c_int) as isize)).units;
        let mut i: libc::c_int = 0 as libc::c_int;
        while i < weights {
            *((*((*(*nr).nn).layer).offset(j_0 as isize)).weight)
                .offset(i as isize) = RedisModule_LoadFloat.unwrap()(rdb);
            i += 1;
            i;
        }
        let mut i_0: libc::c_int = 0 as libc::c_int;
        while i_0 < weights {
            *((*((*(*nr).nn).layer).offset(j_0 as isize)).delta)
                .offset(i_0 as isize) = RedisModule_LoadFloat.unwrap()(rdb);
            i_0 += 1;
            i_0;
        }
        let mut i_1: libc::c_int = 0 as libc::c_int;
        while i_1 < weights {
            *((*((*(*nr).nn).layer).offset(j_0 as isize)).pgradient)
                .offset(i_1 as isize) = RedisModule_LoadFloat.unwrap()(rdb);
            i_1 += 1;
            i_1;
        }
        j_0 += 1;
        j_0;
    }
    let mut ilen: uint32_t = ((*((*(*nr).nn).layer)
        .offset(((*(*nr).nn).layers - 1 as libc::c_int) as isize))
        .units - 1 as libc::c_int) as uint32_t;
    let mut olen: uint32_t = (*((*(*nr).nn).layer).offset(0 as libc::c_int as isize))
        .units as uint32_t;
    let mut j_1: uint32_t = 0 as libc::c_int as uint32_t;
    while j_1 < ilen {
        *((*nr).inorm).offset(j_1 as isize) = RedisModule_LoadFloat.unwrap()(rdb);
        j_1 = j_1.wrapping_add(1);
        j_1;
    }
    let mut j_2: uint32_t = 0 as libc::c_int as uint32_t;
    while j_2 < olen {
        *((*nr).onorm).offset(j_2 as isize) = RedisModule_LoadFloat.unwrap()(rdb);
        j_2 = j_2.wrapping_add(1);
        j_2;
    }
    NRTypeRdbLoadDataset(rdb, &mut (*nr).dataset, ilen, olen);
    NRTypeRdbLoadDataset(rdb, &mut (*nr).test, ilen, olen);
    return nr as *mut libc::c_void;
}
pub unsafe extern "C" fn NRTypeAofRewrite(
    mut aof: *mut RedisModuleIO,
    mut key: *mut RedisModuleString,
    mut value: *mut libc::c_void,
) {}
pub unsafe extern "C" fn NRTypeFree(mut value: *mut libc::c_void) {
    NRTypeReleaseObject(value as *mut NRTypeObject);
}
pub unsafe extern "C" fn RedisModule_OnLoad(
    mut ctx: *mut RedisModuleCtx,
    mut argv: *mut *mut RedisModuleString,
    mut argc: libc::c_int,
) -> libc::c_int {
    if RedisModule_Init(
        ctx,
        b"neuralredis\0" as *const u8 as *const libc::c_char,
        1 as libc::c_int,
        1 as libc::c_int,
    ) == 1 as libc::c_int
    {
        return 1 as libc::c_int;
    }
    let mut tm: RedisModuleTypeMethods = {
        let mut init = RedisModuleTypeMethods {
            version: 1 as libc::c_int as uint64_t,
            rdb_load: Some(
                NRTypeRdbLoad
                    as unsafe extern "C" fn(
                        *mut RedisModuleIO,
                        libc::c_int,
                    ) -> *mut libc::c_void,
            ),
            rdb_save: Some(
                NRTypeRdbSave
                    as unsafe extern "C" fn(*mut RedisModuleIO, *mut libc::c_void) -> (),
            ),
            aof_rewrite: Some(
                NRTypeAofRewrite
                    as unsafe extern "C" fn(
                        *mut RedisModuleIO,
                        *mut RedisModuleString,
                        *mut libc::c_void,
                    ) -> (),
            ),
            mem_usage: None,
            digest: None,
            free: Some(NRTypeFree as unsafe extern "C" fn(*mut libc::c_void) -> ()),
        };
        init
    };
    NRType = RedisModule_CreateDataType
        .unwrap()(
        ctx,
        b"neural-NN\0" as *const u8 as *const libc::c_char,
        2 as libc::c_int,
        &mut tm,
    );
    if NRType.is_null() {
        return 1 as libc::c_int;
    }
    if RedisModule_CreateCommand
        .unwrap()(
        ctx,
        b"nr.create\0" as *const u8 as *const libc::c_char,
        Some(
            NRCreate_RedisCommand
                as unsafe extern "C" fn(
                    *mut RedisModuleCtx,
                    *mut *mut RedisModuleString,
                    libc::c_int,
                ) -> libc::c_int,
        ),
        b"write deny-oom\0" as *const u8 as *const libc::c_char,
        1 as libc::c_int,
        1 as libc::c_int,
        1 as libc::c_int,
    ) == 1 as libc::c_int
    {
        return 1 as libc::c_int;
    }
    if RedisModule_CreateCommand
        .unwrap()(
        ctx,
        b"nr.run\0" as *const u8 as *const libc::c_char,
        Some(
            NRRun_RedisCommand
                as unsafe extern "C" fn(
                    *mut RedisModuleCtx,
                    *mut *mut RedisModuleString,
                    libc::c_int,
                ) -> libc::c_int,
        ),
        b"readonly\0" as *const u8 as *const libc::c_char,
        1 as libc::c_int,
        1 as libc::c_int,
        1 as libc::c_int,
    ) == 1 as libc::c_int
    {
        return 1 as libc::c_int;
    }
    if RedisModule_CreateCommand
        .unwrap()(
        ctx,
        b"nr.class\0" as *const u8 as *const libc::c_char,
        Some(
            NRClass_RedisCommand
                as unsafe extern "C" fn(
                    *mut RedisModuleCtx,
                    *mut *mut RedisModuleString,
                    libc::c_int,
                ) -> libc::c_int,
        ),
        b"readonly\0" as *const u8 as *const libc::c_char,
        1 as libc::c_int,
        1 as libc::c_int,
        1 as libc::c_int,
    ) == 1 as libc::c_int
    {
        return 1 as libc::c_int;
    }
    if RedisModule_CreateCommand
        .unwrap()(
        ctx,
        b"nr.observe\0" as *const u8 as *const libc::c_char,
        Some(
            NRObserve_RedisCommand
                as unsafe extern "C" fn(
                    *mut RedisModuleCtx,
                    *mut *mut RedisModuleString,
                    libc::c_int,
                ) -> libc::c_int,
        ),
        b"write deny-oom\0" as *const u8 as *const libc::c_char,
        1 as libc::c_int,
        1 as libc::c_int,
        1 as libc::c_int,
    ) == 1 as libc::c_int
    {
        return 1 as libc::c_int;
    }
    if RedisModule_CreateCommand
        .unwrap()(
        ctx,
        b"nr.info\0" as *const u8 as *const libc::c_char,
        Some(
            NRInfo_RedisCommand
                as unsafe extern "C" fn(
                    *mut RedisModuleCtx,
                    *mut *mut RedisModuleString,
                    libc::c_int,
                ) -> libc::c_int,
        ),
        b"readonly\0" as *const u8 as *const libc::c_char,
        1 as libc::c_int,
        1 as libc::c_int,
        1 as libc::c_int,
    ) == 1 as libc::c_int
    {
        return 1 as libc::c_int;
    }
    if RedisModule_CreateCommand
        .unwrap()(
        ctx,
        b"nr.train\0" as *const u8 as *const libc::c_char,
        Some(
            NRTrain_RedisCommand
                as unsafe extern "C" fn(
                    *mut RedisModuleCtx,
                    *mut *mut RedisModuleString,
                    libc::c_int,
                ) -> libc::c_int,
        ),
        b"write\0" as *const u8 as *const libc::c_char,
        1 as libc::c_int,
        1 as libc::c_int,
        1 as libc::c_int,
    ) == 1 as libc::c_int
    {
        return 1 as libc::c_int;
    }
    if RedisModule_CreateCommand
        .unwrap()(
        ctx,
        b"nr.reset\0" as *const u8 as *const libc::c_char,
        Some(
            NRReset_RedisCommand
                as unsafe extern "C" fn(
                    *mut RedisModuleCtx,
                    *mut *mut RedisModuleString,
                    libc::c_int,
                ) -> libc::c_int,
        ),
        b"write\0" as *const u8 as *const libc::c_char,
        1 as libc::c_int,
        1 as libc::c_int,
        1 as libc::c_int,
    ) == 1 as libc::c_int
    {
        return 1 as libc::c_int;
    }
    if RedisModule_CreateCommand
        .unwrap()(
        ctx,
        b"nr.threads\0" as *const u8 as *const libc::c_char,
        Some(
            NRThreads_RedisCommand
                as unsafe extern "C" fn(
                    *mut RedisModuleCtx,
                    *mut *mut RedisModuleString,
                    libc::c_int,
                ) -> libc::c_int,
        ),
        b"\0" as *const u8 as *const libc::c_char,
        1 as libc::c_int,
        1 as libc::c_int,
        1 as libc::c_int,
    ) == 1 as libc::c_int
    {
        return 1 as libc::c_int;
    }
    if RedisModule_CreateCommand
        .unwrap()(
        ctx,
        b"nr.getdata\0" as *const u8 as *const libc::c_char,
        Some(
            NRGetdata_RedisCommand
                as unsafe extern "C" fn(
                    *mut RedisModuleCtx,
                    *mut *mut RedisModuleString,
                    libc::c_int,
                ) -> libc::c_int,
        ),
        b"readonly\0" as *const u8 as *const libc::c_char,
        1 as libc::c_int,
        1 as libc::c_int,
        1 as libc::c_int,
    ) == 1 as libc::c_int
    {
        return 1 as libc::c_int;
    }
    return 0 as libc::c_int;
}
