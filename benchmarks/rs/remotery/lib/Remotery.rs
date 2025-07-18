use ::libc;
use ::c2rust_asm_casts;
use std::arch::asm;
use c2rust_asm_casts::AsmCastTrait;
extern "C" {
    pub type _IO_wide_data;
    pub type _IO_codecvt;
    pub type _IO_marker;
    fn fopen(_: *const libc::c_char, _: *const libc::c_char) -> *mut FILE;
    fn fclose(__stream: *mut FILE) -> libc::c_int;
    fn realloc(_: *mut libc::c_void, _: libc::c_ulong) -> *mut libc::c_void;
    fn free(__ptr: *mut libc::c_void);
    fn malloc(_: libc::c_ulong) -> *mut libc::c_void;
    fn fwrite(
        _: *const libc::c_void,
        _: libc::c_ulong,
        _: libc::c_ulong,
        _: *mut FILE,
    ) -> libc::c_ulong;
    fn __assert_fail(
        __assertion: *const libc::c_char,
        __file: *const libc::c_char,
        __line: libc::c_uint,
        __function: *const libc::c_char,
    ) -> !;
    fn clock() -> clock_t;
    fn time(__timer: *mut time_t) -> time_t;
    fn gmtime(__timer: *const time_t) -> *mut tm;
    fn clock_gettime(__clock_id: clockid_t, __tp: *mut timespec) -> libc::c_int;
    fn mkstemp(__template: *mut libc::c_char) -> libc::c_int;
    fn select(
        __nfds: libc::c_int,
        __readfds: *mut fd_set,
        __writefds: *mut fd_set,
        __exceptfds: *mut fd_set,
        __timeout: *mut timeval,
    ) -> libc::c_int;
    fn abs(_: libc::c_int) -> libc::c_int;
    fn mbstowcs(__pwcs: *mut wchar_t, __s: *const libc::c_char, __n: size_t) -> size_t;
    fn fabs(_: libc::c_double) -> libc::c_double;
    fn fmod(_: libc::c_double, _: libc::c_double) -> libc::c_double;
    fn prctl(__option: libc::c_int, _: ...) -> libc::c_int;
    fn pthread_create(
        __newthread: *mut pthread_t,
        __attr: *const pthread_attr_t,
        __start_routine: Option::<
            unsafe extern "C" fn(*mut libc::c_void) -> *mut libc::c_void,
        >,
        __arg: *mut libc::c_void,
    ) -> libc::c_int;
    fn pthread_join(
        __th: pthread_t,
        __thread_return: *mut *mut libc::c_void,
    ) -> libc::c_int;
    fn pthread_self() -> pthread_t;
    fn pthread_equal(__thread1: pthread_t, __thread2: pthread_t) -> libc::c_int;
    fn pthread_mutex_init(
        __mutex: *mut pthread_mutex_t,
        __mutexattr: *const pthread_mutexattr_t,
    ) -> libc::c_int;
    fn pthread_mutex_destroy(__mutex: *mut pthread_mutex_t) -> libc::c_int;
    fn pthread_mutex_lock(__mutex: *mut pthread_mutex_t) -> libc::c_int;
    fn pthread_mutex_unlock(__mutex: *mut pthread_mutex_t) -> libc::c_int;
    fn pthread_key_create(
        __key: *mut pthread_key_t,
        __destr_function: Option::<unsafe extern "C" fn(*mut libc::c_void) -> ()>,
    ) -> libc::c_int;
    fn pthread_key_delete(__key: pthread_key_t) -> libc::c_int;
    fn pthread_getspecific(__key: pthread_key_t) -> *mut libc::c_void;
    fn pthread_setspecific(
        __key: pthread_key_t,
        __pointer: *const libc::c_void,
    ) -> libc::c_int;
    fn close(__fd: libc::c_int) -> libc::c_int;
    fn usleep(__useconds: __useconds_t) -> libc::c_int;
    fn unlink(__name: *const libc::c_char) -> libc::c_int;
    fn ftruncate(__fd: libc::c_int, __length: __off_t) -> libc::c_int;
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
    fn memcmp(
        _: *const libc::c_void,
        _: *const libc::c_void,
        _: libc::c_ulong,
    ) -> libc::c_int;
    fn strlen(_: *const libc::c_char) -> libc::c_ulong;
    fn socket(
        __domain: libc::c_int,
        __type: libc::c_int,
        __protocol: libc::c_int,
    ) -> libc::c_int;
    fn bind(__fd: libc::c_int, __addr: *const sockaddr, __len: socklen_t) -> libc::c_int;
    fn send(
        __fd: libc::c_int,
        __buf: *const libc::c_void,
        __n: size_t,
        __flags: libc::c_int,
    ) -> ssize_t;
    fn recv(
        __fd: libc::c_int,
        __buf: *mut libc::c_void,
        __n: size_t,
        __flags: libc::c_int,
    ) -> ssize_t;
    fn setsockopt(
        __fd: libc::c_int,
        __level: libc::c_int,
        __optname: libc::c_int,
        __optval: *const libc::c_void,
        __optlen: socklen_t,
    ) -> libc::c_int;
    fn listen(__fd: libc::c_int, __n: libc::c_int) -> libc::c_int;
    fn accept(
        __fd: libc::c_int,
        __addr: *mut sockaddr,
        __addr_len: *mut socklen_t,
    ) -> libc::c_int;
    fn shutdown(__fd: libc::c_int, __how: libc::c_int) -> libc::c_int;
    fn mmap(
        __addr: *mut libc::c_void,
        __len: size_t,
        __prot: libc::c_int,
        __flags: libc::c_int,
        __fd: libc::c_int,
        __offset: __off_t,
    ) -> *mut libc::c_void;
    fn munmap(__addr: *mut libc::c_void, __len: size_t) -> libc::c_int;
    fn htonl(__hostlong: uint32_t) -> uint32_t;
    fn htons(__hostshort: uint16_t) -> uint16_t;
    fn fcntl(__fd: libc::c_int, __cmd: libc::c_int, _: ...) -> libc::c_int;
    fn __errno_location() -> *mut libc::c_int;
}
pub type rmtBool = libc::c_uint;
pub type rmtU8 = libc::c_uchar;
pub type rmtU16 = libc::c_ushort;
pub type rmtU32 = libc::c_uint;
pub type rmtU64 = libc::c_ulonglong;
pub type rmtS8 = libc::c_char;
pub type rmtS32 = libc::c_int;
pub type rmtS64 = libc::c_longlong;
pub type rmtF32 = libc::c_float;
pub type rmtF64 = libc::c_double;
pub type rmtPStr = *const libc::c_char;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct Msg_SampleTree {
    pub rootSample: *mut Sample,
    pub allocator: *mut ObjectAllocator,
    pub threadName: rmtPStr,
    pub userData: rmtU32,
    pub partialTree: rmtBool,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ObjectAllocator {
    pub object_size: rmtU32,
    pub constructor: ObjConstructor,
    pub destructor: ObjDestructor,
    pub nb_free: rmtAtomicS32,
    pub nb_inuse: rmtAtomicS32,
    pub nb_allocated: rmtAtomicS32,
    pub first_free: *mut ObjectLink,
}
pub type ObjectLink = ObjectLink_s;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ObjectLink_s {
    pub next: *mut ObjectLink_s,
}
pub type rmtAtomicS32 = rmtS32;
pub type ObjDestructor = Option::<unsafe extern "C" fn(*mut libc::c_void) -> ()>;
pub type ObjConstructor = Option::<unsafe extern "C" fn(*mut libc::c_void) -> rmtError>;
pub type rmtError = libc::c_uint;
pub const RMT_ERROR_CUDA_UNKNOWN: rmtError = 43;
pub const RMT_ERROR_OPENGL_ERROR: rmtError = 42;
pub const RMT_ERROR_D3D11_FAILED_TO_CREATE_QUERY: rmtError = 41;
pub const RMT_ERROR_ERROR_NOT_READY: rmtError = 40;
pub const RMT_ERROR_CUDA_OUT_OF_MEMORY: rmtError = 39;
pub const RMT_ERROR_CUDA_INVALID_HANDLE: rmtError = 38;
pub const RMT_ERROR_CUDA_INVALID_VALUE: rmtError = 37;
pub const RMT_ERROR_CUDA_INVALID_CONTEXT: rmtError = 36;
pub const RMT_ERROR_CUDA_NOT_INITIALIZED: rmtError = 35;
pub const RMT_ERROR_CUDA_DEINITIALIZED: rmtError = 34;
pub const RMT_ERROR_SEND_ON_INCOMPLETE_PROFILE: rmtError = 33;
pub const RMT_ERROR_REMOTERY_NOT_CREATED: rmtError = 32;
pub const RMT_ERROR_WEBSOCKET_RECEIVE_TIMEOUT: rmtError = 31;
pub const RMT_ERROR_WEBSOCKET_BAD_FRAME_HEADER_MASK: rmtError = 30;
pub const RMT_ERROR_WEBSOCKET_BAD_FRAME_HEADER_SIZE: rmtError = 29;
pub const RMT_ERROR_WEBSOCKET_BAD_FRAME_HEADER: rmtError = 28;
pub const RMT_ERROR_WEBSOCKET_DISCONNECTED: rmtError = 27;
pub const RMT_ERROR_WEBSOCKET_HANDSHAKE_STRING_FAIL: rmtError = 26;
pub const RMT_ERROR_WEBSOCKET_HANDSHAKE_BAD_KEY: rmtError = 25;
pub const RMT_ERROR_WEBSOCKET_HANDSHAKE_NO_KEY: rmtError = 24;
pub const RMT_ERROR_WEBSOCKET_HANDSHAKE_BAD_HOST: rmtError = 23;
pub const RMT_ERROR_WEBSOCKET_HANDSHAKE_NO_HOST: rmtError = 22;
pub const RMT_ERROR_WEBSOCKET_HANDSHAKE_BAD_VERSION: rmtError = 21;
pub const RMT_ERROR_WEBSOCKET_HANDSHAKE_NO_VERSION: rmtError = 20;
pub const RMT_ERROR_WEBSOCKET_HANDSHAKE_NOT_GET: rmtError = 19;
pub const RMT_ERROR_SOCKET_RECV_FAILED: rmtError = 18;
pub const RMT_ERROR_SOCKET_RECV_TIMEOUT: rmtError = 17;
pub const RMT_ERROR_SOCKET_RECV_NO_DATA: rmtError = 16;
pub const RMT_ERROR_SOCKET_SEND_FAIL: rmtError = 15;
pub const RMT_ERROR_SOCKET_POLL_ERRORS: rmtError = 14;
pub const RMT_ERROR_SOCKET_SELECT_FAIL: rmtError = 13;
pub const RMT_ERROR_SOCKET_INVALID_POLL: rmtError = 12;
pub const RMT_ERROR_OPEN_THREAD_HANDLE_FAIL: rmtError = 11;
pub const RMT_ERROR_CREATE_THREAD_FAIL: rmtError = 10;
pub const RMT_ERROR_VIRTUAL_MEMORY_BUFFER_FAIL: rmtError = 9;
pub const RMT_ERROR_TLS_ALLOC_FAIL: rmtError = 8;
pub const RMT_ERROR_MALLOC_FAIL: rmtError = 7;
pub const RMT_ERROR_TIMEOUT: rmtError = 6;
pub const RMT_ERROR_RESOURCE_ACCESS_FAIL: rmtError = 5;
pub const RMT_ERROR_RESOURCE_CREATE_FAIL: rmtError = 4;
pub const RMT_ERROR_INVALID_INPUT: rmtError = 3;
pub const RMT_ERROR_UNKNOWN: rmtError = 2;
pub const RMT_ERROR_RECURSIVE_SAMPLE: rmtError = 1;
pub const RMT_ERROR_NONE: rmtError = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct Sample {
    pub Link: ObjectLink,
    pub type_0: rmtSampleType,
    pub name_hash: rmtU32,
    pub unique_id: rmtU32,
    pub uniqueColour: [rmtU8; 3],
    pub parent: *mut Sample,
    pub first_child: *mut Sample,
    pub last_child: *mut Sample,
    pub next_sibling: *mut Sample,
    pub nb_children: rmtU32,
    pub us_start: rmtU64,
    pub us_end: rmtU64,
    pub us_length: rmtU64,
    pub us_sampled_length: rmtU64,
    pub usGpuIssueOnCpu: rmtU64,
    pub call_count: rmtU32,
    pub recurse_depth: rmtU16,
    pub max_recurse_depth: rmtU16,
}
pub type rmtSampleType = libc::c_uint;
pub const RMT_SampleType_Count: rmtSampleType = 6;
pub const RMT_SampleType_Metal: rmtSampleType = 5;
pub const RMT_SampleType_OpenGL: rmtSampleType = 4;
pub const RMT_SampleType_D3D12: rmtSampleType = 3;
pub const RMT_SampleType_D3D11: rmtSampleType = 2;
pub const RMT_SampleType_CUDA: rmtSampleType = 1;
pub const RMT_SampleType_CPU: rmtSampleType = 0;
pub type rmtSampleTree = Msg_SampleTree;
pub type rmtSample = Sample;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct Remotery {
    pub server: *mut Server,
    pub timer: usTimer,
    pub mq_to_rmt_thread: *mut rmtMessageQueue,
    pub thread: *mut rmtThread,
    pub string_table: *mut StringTable,
    pub logfile: *mut FILE,
    pub map_message_queue_fn: Option::<
        unsafe extern "C" fn(*mut Remotery, *mut Message) -> (),
    >,
    pub map_message_queue_data: *mut libc::c_void,
    pub threadProfilers: *mut ThreadProfilers,
    pub propertyMutex: rmtMutex,
    pub rootProperty: rmtProperty,
    pub propertyAllocator: *mut ObjectAllocator,
    pub propertyFrame: rmtU32,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct rmtProperty {
    pub initialised: rmtBool,
    pub type_0: rmtPropertyType,
    pub flags: rmtPropertyFlags,
    pub value: rmtPropertyValue,
    pub lastFrameValue: rmtPropertyValue,
    pub prevValue: rmtPropertyValue,
    pub prevValueFrame: rmtU32,
    pub name: *const libc::c_char,
    pub description: *const libc::c_char,
    pub defaultValue: rmtPropertyValue,
    pub parent: *mut rmtProperty,
    pub firstChild: *mut rmtProperty,
    pub lastChild: *mut rmtProperty,
    pub nextSibling: *mut rmtProperty,
    pub nameHash: rmtU32,
    pub uniqueID: rmtU32,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union rmtPropertyValue {
    pub Bool: rmtBool,
    pub S32: rmtS32,
    pub U32: rmtU32,
    pub F32: rmtF32,
    pub S64: rmtS64,
    pub U64: rmtU64,
    pub F64: rmtF64,
}
pub type rmtPropertyFlags = libc::c_uint;
pub const RMT_PropertyFlags_FrameReset: rmtPropertyFlags = 1;
pub const RMT_PropertyFlags_NoFlags: rmtPropertyFlags = 0;
pub type rmtPropertyType = libc::c_uint;
pub const RMT_PropertyType_rmtF64: rmtPropertyType = 7;
pub const RMT_PropertyType_rmtU64: rmtPropertyType = 6;
pub const RMT_PropertyType_rmtS64: rmtPropertyType = 5;
pub const RMT_PropertyType_rmtF32: rmtPropertyType = 4;
pub const RMT_PropertyType_rmtU32: rmtPropertyType = 3;
pub const RMT_PropertyType_rmtS32: rmtPropertyType = 2;
pub const RMT_PropertyType_rmtBool: rmtPropertyType = 1;
pub const RMT_PropertyType_rmtGroup: rmtPropertyType = 0;
pub type pthread_mutex_t = rmtMutex;
#[derive(Copy, Clone)]
#[repr(C)]
pub union rmtMutex {
    pub __data: __pthread_mutex_s,
    pub __size: [libc::c_char; 40],
    pub __align: libc::c_long,
}
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
pub type __pthread_list_t = __pthread_internal_list;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct __pthread_internal_list {
    pub __prev: *mut __pthread_internal_list,
    pub __next: *mut __pthread_internal_list,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ThreadProfilers {
    pub timer: *mut usTimer,
    pub mqToRmtThread: *mut rmtMessageQueue,
    pub compiledSampleFn: *mut libc::c_void,
    pub compiledSampleFnSize: rmtU32,
    pub threadProfilerTlsHandle: rmtTLS,
    pub threadProfilers: [ThreadProfiler; 256],
    pub nbThreadProfilers: rmtAtomicU32,
    pub maxNbThreadProfilers: rmtU32,
    pub threadProfilerMutex: rmtMutex,
    pub threadSampleThread: *mut rmtThread,
    pub threadGatherThread: *mut rmtThread,
}
pub type rmtThread = Thread_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct Thread_t {
    pub handle: rmtThreadHandle,
    pub callback: ThreadProc,
    pub param: *mut libc::c_void,
    pub error: rmtError,
    pub request_exit: rmtBool,
}
pub type ThreadProc = Option::<unsafe extern "C" fn(*mut rmtThread) -> rmtError>;
pub type rmtThreadHandle = pthread_t;
pub type pthread_t = libc::c_ulong;
pub type rmtAtomicU32 = rmtU32;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ThreadProfiler {
    pub registerBackup0: rmtU64,
    pub registerBackup1: rmtU64,
    pub registerBackup2: rmtU64,
    pub nbSamplesWithoutCallback: rmtAtomicS32,
    pub processorIndex: rmtU32,
    pub lastProcessorIndex: rmtU32,
    pub threadId: rmtThreadId,
    pub threadHandle: rmtThreadHandle,
    pub threadName: [libc::c_char; 64],
    pub threadNameHash: rmtU32,
    pub sampleTrees: [*mut SampleTree; 6],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct SampleTree {
    pub allocator: *mut ObjectAllocator,
    pub root: *mut Sample,
    pub currentParent: *mut Sample,
    pub msLastTreeSendTime: rmtAtomicU32,
    pub treeBeingModified: rmtAtomicU32,
    pub sendSampleOnClose: *mut Sample,
}
pub type rmtThreadId = uintptr_t;
pub type uintptr_t = libc::c_ulong;
pub type rmtTLS = pthread_key_t;
pub type pthread_key_t = libc::c_uint;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct rmtMessageQueue {
    pub size: rmtU32,
    pub data: *mut VirtualMirrorBuffer,
    pub read_pos: rmtAtomicU32,
    pub write_pos: rmtAtomicU32,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct VirtualMirrorBuffer {
    pub size: rmtU32,
    pub ptr: *mut rmtU8,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct usTimer {
    pub counter_start: LARGE_INTEGER,
    pub counter_scale: libc::c_double,
}
pub type LARGE_INTEGER = rmtU64;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct Message {
    pub id: MessageID,
    pub payload_size: rmtU32,
    pub threadProfiler: *mut ThreadProfiler,
    pub payload: [rmtU8; 1],
}
pub type MessageID = libc::c_uint;
pub const MsgID_Force32Bits: MessageID = 4294967295;
pub const MsgID_PropertySnapshot: MessageID = 6;
pub const MsgID_None: MessageID = 5;
pub const MsgID_ProcessorThreads: MessageID = 4;
pub const MsgID_SampleTree: MessageID = 3;
pub const MsgID_LogText: MessageID = 2;
pub const MsgID_AddToStringTable: MessageID = 1;
pub const MsgID_NotReady: MessageID = 0;
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
pub type size_t = libc::c_ulong;
pub type __off64_t = libc::c_long;
pub type _IO_lock_t = ();
pub type __off_t = libc::c_long;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct StringTable {
    pub text: *mut Buffer,
    pub text_map: *mut rmtHashTable,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct rmtHashTable {
    pub maxNbSlots: rmtU32,
    pub nbSlots: rmtU32,
    pub slots: *mut HashSlot,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct HashSlot {
    pub key: rmtU32,
    pub value: rmtU64,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct Buffer {
    pub alloc_granularity: rmtU32,
    pub bytes_allocated: rmtU32,
    pub bytes_used: rmtU32,
    pub data: *mut rmtU8,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct Server {
    pub listen_socket: *mut WebSocket,
    pub client_socket: *mut WebSocket,
    pub last_ping_time: rmtU32,
    pub port: rmtU16,
    pub reuse_open_port: rmtBool,
    pub limit_connections_to_localhost: rmtBool,
    pub bin_buf: *mut Buffer,
    pub receive_handler: Server_ReceiveHandler,
    pub receive_handler_context: *mut libc::c_void,
}
pub type Server_ReceiveHandler = Option::<
    unsafe extern "C" fn(*mut libc::c_void, *mut libc::c_char, rmtU32) -> rmtError,
>;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct WebSocket {
    pub tcp_socket: *mut TCPSocket,
    pub mode: WebSocketMode,
    pub frame_bytes_remaining: rmtU32,
    pub mask_offset: rmtU32,
    pub data: C2RustUnnamed,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed {
    pub mask: [rmtU8; 4],
    pub mask_u32: rmtU32,
}
pub type WebSocketMode = libc::c_uint;
pub const WEBSOCKET_BINARY: WebSocketMode = 2;
pub const WEBSOCKET_TEXT: WebSocketMode = 1;
pub const WEBSOCKET_NONE: WebSocketMode = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct TCPSocket {
    pub socket: SOCKET,
}
pub type SOCKET = libc::c_int;
pub type rmtMallocPtr = Option::<
    unsafe extern "C" fn(*mut libc::c_void, rmtU32) -> *mut libc::c_void,
>;
pub type rmtReallocPtr = Option::<
    unsafe extern "C" fn(
        *mut libc::c_void,
        *mut libc::c_void,
        rmtU32,
    ) -> *mut libc::c_void,
>;
pub type rmtFreePtr = Option::<
    unsafe extern "C" fn(*mut libc::c_void, *mut libc::c_void) -> (),
>;
pub type rmtInputHandlerPtr = Option::<
    unsafe extern "C" fn(*const libc::c_char, *mut libc::c_void) -> (),
>;
pub type rmtSampleTreeHandlerPtr = Option::<
    unsafe extern "C" fn(*mut libc::c_void, *mut rmtSampleTree) -> (),
>;
pub type rmtPropertyHandlerPtr = Option::<
    unsafe extern "C" fn(*mut libc::c_void, *mut rmtProperty) -> (),
>;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct rmtSettings {
    pub port: rmtU16,
    pub reuse_open_port: rmtBool,
    pub limit_connections_to_localhost: rmtBool,
    pub enableThreadSampler: rmtBool,
    pub msSleepBetweenServerUpdates: rmtU32,
    pub messageQueueSizeInBytes: rmtU32,
    pub maxNbMessagesPerUpdate: rmtU32,
    pub malloc: rmtMallocPtr,
    pub realloc: rmtReallocPtr,
    pub free: rmtFreePtr,
    pub mm_context: *mut libc::c_void,
    pub input_handler: rmtInputHandlerPtr,
    pub sampletree_handler: rmtSampleTreeHandlerPtr,
    pub sampletree_context: *mut libc::c_void,
    pub snapshot_callback: rmtPropertyHandlerPtr,
    pub snapshot_context: *mut libc::c_void,
    pub input_handler_context: *mut libc::c_void,
    pub logPath: rmtPStr,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct Msg_AddToStringTable {
    pub hash: rmtU32,
    pub length: rmtU32,
}
pub type r_size_t = libc::c_uint;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct Msg_PropertySnapshot {
    pub rootSnapshot: *mut PropertySnapshot,
    pub nbSnapshots: rmtU32,
    pub propertyFrame: rmtU32,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct PropertySnapshot {
    pub Link: ObjectLink,
    pub type_0: rmtPropertyType,
    pub value: rmtPropertyValue,
    pub prevValue: rmtPropertyValue,
    pub prevValueFrame: rmtU32,
    pub nameHash: rmtU32,
    pub uniqueID: rmtU32,
    pub depth: rmtU8,
    pub nbChildren: rmtU32,
    pub nextSnapshot: *mut PropertySnapshot,
}
pub type rmtSampleFlags = libc::c_uint;
pub const RMTSF_SendOnClose: rmtSampleFlags = 8;
pub const RMTSF_Root: rmtSampleFlags = 4;
pub const RMTSF_Recursive: rmtSampleFlags = 2;
pub const RMTSF_Aggregate: rmtSampleFlags = 1;
pub const RMTSF_None: rmtSampleFlags = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct rmtSampleIterator {
    pub sample: *mut rmtSample,
    pub initial: *mut rmtSample,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct rmtPropertyIterator {
    pub property: *mut rmtProperty,
    pub initial: *mut rmtProperty,
}
pub type __useconds_t = libc::c_uint;
pub type __syscall_slong_t = libc::c_long;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct timespec {
    pub tv_sec: __time_t,
    pub tv_nsec: __syscall_slong_t,
}
pub type __time_t = libc::c_long;
pub type clockid_t = __clockid_t;
pub type __clockid_t = libc::c_int;
pub type errno_t = libc::c_int;
pub type __clock_t = libc::c_long;
pub type clock_t = __clock_t;
pub type ssize_t = __ssize_t;
pub type __ssize_t = libc::c_long;
pub const MSG_NOSIGNAL: C2RustUnnamed_2 = 16384;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct SocketStatus {
    pub can_read: rmtBool,
    pub can_write: rmtBool,
    pub error_state: rmtError,
}
pub type __fd_mask = libc::c_long;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct fd_set {
    pub __fds_bits: [__fd_mask; 16],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct timeval {
    pub tv_sec: __time_t,
    pub tv_usec: __suseconds_t,
}
pub type __suseconds_t = libc::c_long;
pub const SHUT_WR: C2RustUnnamed_3 = 1;
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_0 {
    pub d: libc::c_double,
    pub c: [libc::c_uchar; 8],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_1 {
    pub i: libc::c_uint,
    pub c: [libc::c_uchar; 4],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct Processor {
    pub threadProfiler: *mut ThreadProfiler,
    pub sampleCount: rmtU32,
    pub sampleTime: rmtU64,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct Msg_ProcessorThreads {
    pub messageIndex: rmtU64,
    pub nbProcessors: rmtU32,
    pub processors: [Processor; 1],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct SHA1 {
    pub data: [rmtU8; 20],
}
pub type socklen_t = __socklen_t;
pub type __socklen_t = libc::c_uint;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct sockaddr {
    pub sa_family: sa_family_t,
    pub sa_data: [libc::c_char; 14],
}
pub type sa_family_t = libc::c_ushort;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct sockaddr_in {
    pub sin_family: sa_family_t,
    pub sin_port: in_port_t,
    pub sin_addr: in_addr,
    pub sin_zero: [libc::c_uchar; 8],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct in_addr {
    pub s_addr: in_addr_t,
}
pub type in_addr_t = uint32_t;
pub type uint32_t = __uint32_t;
pub type __uint32_t = libc::c_uint;
pub type in_port_t = uint16_t;
pub type uint16_t = __uint16_t;
pub type __uint16_t = libc::c_ushort;
pub const IPPROTO_TCP: C2RustUnnamed_4 = 6;
pub const SOCK_STREAM: __socket_type = 1;
pub type int32_t = __int32_t;
pub type __int32_t = libc::c_int;
#[derive(Copy, Clone)]
#[repr(C)]
pub union pthread_attr_t {
    pub __size: [libc::c_char; 56],
    pub __align: libc::c_long,
}
pub type rmtCpuContext = libc::c_int;
pub type time_t = __time_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub union pthread_mutexattr_t {
    pub __size: [libc::c_char; 4],
    pub __align: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct tm {
    pub tm_sec: libc::c_int,
    pub tm_min: libc::c_int,
    pub tm_hour: libc::c_int,
    pub tm_mday: libc::c_int,
    pub tm_mon: libc::c_int,
    pub tm_year: libc::c_int,
    pub tm_wday: libc::c_int,
    pub tm_yday: libc::c_int,
    pub tm_isdst: libc::c_int,
    pub tm_gmtoff: libc::c_long,
    pub tm_zone: *const libc::c_char,
}
pub type wchar_t = libc::c_int;
pub type __socket_type = libc::c_uint;
pub const SOCK_NONBLOCK: __socket_type = 2048;
pub const SOCK_CLOEXEC: __socket_type = 524288;
pub const SOCK_PACKET: __socket_type = 10;
pub const SOCK_DCCP: __socket_type = 6;
pub const SOCK_SEQPACKET: __socket_type = 5;
pub const SOCK_RDM: __socket_type = 4;
pub const SOCK_RAW: __socket_type = 3;
pub const SOCK_DGRAM: __socket_type = 2;
pub type C2RustUnnamed_2 = libc::c_uint;
pub const MSG_CMSG_CLOEXEC: C2RustUnnamed_2 = 1073741824;
pub const MSG_FASTOPEN: C2RustUnnamed_2 = 536870912;
pub const MSG_ZEROCOPY: C2RustUnnamed_2 = 67108864;
pub const MSG_BATCH: C2RustUnnamed_2 = 262144;
pub const MSG_WAITFORONE: C2RustUnnamed_2 = 65536;
pub const MSG_MORE: C2RustUnnamed_2 = 32768;
pub const MSG_ERRQUEUE: C2RustUnnamed_2 = 8192;
pub const MSG_RST: C2RustUnnamed_2 = 4096;
pub const MSG_CONFIRM: C2RustUnnamed_2 = 2048;
pub const MSG_SYN: C2RustUnnamed_2 = 1024;
pub const MSG_FIN: C2RustUnnamed_2 = 512;
pub const MSG_WAITALL: C2RustUnnamed_2 = 256;
pub const MSG_EOR: C2RustUnnamed_2 = 128;
pub const MSG_DONTWAIT: C2RustUnnamed_2 = 64;
pub const MSG_TRUNC: C2RustUnnamed_2 = 32;
pub const MSG_PROXY: C2RustUnnamed_2 = 16;
pub const MSG_CTRUNC: C2RustUnnamed_2 = 8;
pub const MSG_DONTROUTE: C2RustUnnamed_2 = 4;
pub const MSG_PEEK: C2RustUnnamed_2 = 2;
pub const MSG_OOB: C2RustUnnamed_2 = 1;
pub type C2RustUnnamed_3 = libc::c_uint;
pub const SHUT_RDWR: C2RustUnnamed_3 = 2;
pub const SHUT_RD: C2RustUnnamed_3 = 0;
pub type C2RustUnnamed_4 = libc::c_uint;
pub const IPPROTO_MAX: C2RustUnnamed_4 = 256;
pub const IPPROTO_RAW: C2RustUnnamed_4 = 255;
pub const IPPROTO_MPLS: C2RustUnnamed_4 = 137;
pub const IPPROTO_UDPLITE: C2RustUnnamed_4 = 136;
pub const IPPROTO_SCTP: C2RustUnnamed_4 = 132;
pub const IPPROTO_COMP: C2RustUnnamed_4 = 108;
pub const IPPROTO_PIM: C2RustUnnamed_4 = 103;
pub const IPPROTO_ENCAP: C2RustUnnamed_4 = 98;
pub const IPPROTO_BEETPH: C2RustUnnamed_4 = 94;
pub const IPPROTO_MTP: C2RustUnnamed_4 = 92;
pub const IPPROTO_AH: C2RustUnnamed_4 = 51;
pub const IPPROTO_ESP: C2RustUnnamed_4 = 50;
pub const IPPROTO_GRE: C2RustUnnamed_4 = 47;
pub const IPPROTO_RSVP: C2RustUnnamed_4 = 46;
pub const IPPROTO_IPV6: C2RustUnnamed_4 = 41;
pub const IPPROTO_DCCP: C2RustUnnamed_4 = 33;
pub const IPPROTO_TP: C2RustUnnamed_4 = 29;
pub const IPPROTO_IDP: C2RustUnnamed_4 = 22;
pub const IPPROTO_UDP: C2RustUnnamed_4 = 17;
pub const IPPROTO_PUP: C2RustUnnamed_4 = 12;
pub const IPPROTO_EGP: C2RustUnnamed_4 = 8;
pub const IPPROTO_IPIP: C2RustUnnamed_4 = 4;
pub const IPPROTO_IGMP: C2RustUnnamed_4 = 2;
pub const IPPROTO_ICMP: C2RustUnnamed_4 = 1;
pub const IPPROTO_IP: C2RustUnnamed_4 = 0;
static mut g_Settings: rmtSettings = rmtSettings {
    port: 0,
    reuse_open_port: 0,
    limit_connections_to_localhost: 0,
    enableThreadSampler: 0,
    msSleepBetweenServerUpdates: 0,
    messageQueueSizeInBytes: 0,
    maxNbMessagesPerUpdate: 0,
    malloc: None,
    realloc: None,
    free: None,
    mm_context: 0 as *const libc::c_void as *mut libc::c_void,
    input_handler: None,
    sampletree_handler: None,
    sampletree_context: 0 as *const libc::c_void as *mut libc::c_void,
    snapshot_callback: None,
    snapshot_context: 0 as *const libc::c_void as *mut libc::c_void,
    input_handler_context: 0 as *const libc::c_void as *mut libc::c_void,
    logPath: 0 as *const libc::c_char,
};
static mut g_SettingsInitialized: rmtBool = 0 as libc::c_int as rmtBool;
unsafe extern "C" fn maxU16(mut a: rmtU16, mut b: rmtU16) -> rmtU16 {
    return (if a as libc::c_int > b as libc::c_int {
        a as libc::c_int
    } else {
        b as libc::c_int
    }) as rmtU16;
}
unsafe extern "C" fn minS32(mut a: rmtS32, mut b: rmtS32) -> rmtS32 {
    return if a < b { a } else { b };
}
unsafe extern "C" fn maxS32(mut a: rmtS32, mut b: rmtS32) -> rmtS32 {
    return if a > b { a } else { b };
}
unsafe extern "C" fn minU32(mut a: rmtU32, mut b: rmtU32) -> rmtU32 {
    return if a < b { a } else { b };
}
unsafe extern "C" fn maxS64(mut a: rmtS64, mut b: rmtS64) -> rmtS64 {
    return if a > b { a } else { b };
}
unsafe extern "C" fn rmtMalloc(mut size: rmtU32) -> *mut libc::c_void {
    return (g_Settings.malloc).unwrap()(g_Settings.mm_context, size);
}
unsafe extern "C" fn rmtRealloc(
    mut ptr: *mut libc::c_void,
    mut size: rmtU32,
) -> *mut libc::c_void {
    return (g_Settings.realloc).unwrap()(g_Settings.mm_context, ptr, size);
}
unsafe extern "C" fn rmtFree(mut ptr: *mut libc::c_void) {
    (g_Settings.free).unwrap()(g_Settings.mm_context, ptr);
}
unsafe extern "C" fn rmtOpenFile(
    mut filename: *const libc::c_char,
    mut mode: *const libc::c_char,
) -> *mut FILE {
    return fopen(filename, mode);
}
pub unsafe extern "C" fn rmtCloseFile(mut fp: *mut FILE) {
    if !fp.is_null() {
        fclose(fp);
    }
}
pub unsafe extern "C" fn rmtWriteFile(
    mut fp: *mut FILE,
    mut data: *const libc::c_void,
    mut size: rmtU32,
) -> rmtBool {
    if !fp.is_null() {} else {
        __assert_fail(
            b"fp != NULL\0" as *const u8 as *const libc::c_char,
            b"lib/Remotery.c\0" as *const u8 as *const libc::c_char,
            251 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 51],
                &[libc::c_char; 51],
            >(b"rmtBool rmtWriteFile(FILE *, const void *, rmtU32)\0"))
                .as_ptr(),
        );
    }
    'c_8759: {
        if !fp.is_null() {} else {
            __assert_fail(
                b"fp != NULL\0" as *const u8 as *const libc::c_char,
                b"lib/Remotery.c\0" as *const u8 as *const libc::c_char,
                251 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 51],
                    &[libc::c_char; 51],
                >(b"rmtBool rmtWriteFile(FILE *, const void *, rmtU32)\0"))
                    .as_ptr(),
            );
        }
    };
    return if fwrite(data, size as libc::c_ulong, 1 as libc::c_int as libc::c_ulong, fp)
        == size as libc::c_ulong
    {
        1 as libc::c_int as rmtBool
    } else {
        0 as libc::c_int as rmtBool
    };
}
unsafe extern "C" fn msTimer_Get() -> rmtU32 {
    let mut time_0: clock_t = clock();
    let mut msTime: rmtU32 = (time_0
        / (1000000 as libc::c_int as __clock_t / 1000 as libc::c_int as libc::c_long))
        as rmtU32;
    return msTime;
}
unsafe extern "C" fn usTimer_Init(mut timer: *mut usTimer) {
    let mut tv: timespec = timespec { tv_sec: 0, tv_nsec: 0 };
    clock_gettime(0 as libc::c_int, &mut tv);
    (*timer)
        .counter_start = (tv.tv_sec as libc::c_ulonglong)
        .wrapping_mul(1000000 as libc::c_int as rmtU64)
        .wrapping_add((tv.tv_nsec as libc::c_double * 0.001f64) as rmtU64);
}
unsafe extern "C" fn usTimer_Get(mut timer: *mut usTimer) -> rmtU64 {
    let mut tv: timespec = timespec { tv_sec: 0, tv_nsec: 0 };
    clock_gettime(0 as libc::c_int, &mut tv);
    return (tv.tv_sec as libc::c_ulonglong)
        .wrapping_mul(1000000 as libc::c_int as rmtU64)
        .wrapping_add((tv.tv_nsec as libc::c_double * 0.001f64) as rmtU64)
        .wrapping_sub((*timer).counter_start);
}
unsafe extern "C" fn msSleep(mut time_ms: rmtU32) {
    usleep(time_ms.wrapping_mul(1000 as libc::c_int as libc::c_uint));
}
unsafe extern "C" fn TimeDateNow() -> *mut tm {
    let mut time_now: time_t = time(0 as *mut time_t);
    return gmtime(&mut time_now);
}
unsafe extern "C" fn tlsAlloc(mut handle: *mut rmtTLS) -> rmtError {
    if !handle.is_null() {} else {
        __assert_fail(
            b"handle != NULL\0" as *const u8 as *const libc::c_char,
            b"lib/Remotery.c\0" as *const u8 as *const libc::c_char,
            439 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 28],
                &[libc::c_char; 28],
            >(b"rmtError tlsAlloc(rmtTLS *)\0"))
                .as_ptr(),
        );
    }
    'c_9142: {
        if !handle.is_null() {} else {
            __assert_fail(
                b"handle != NULL\0" as *const u8 as *const libc::c_char,
                b"lib/Remotery.c\0" as *const u8 as *const libc::c_char,
                439 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 28],
                    &[libc::c_char; 28],
                >(b"rmtError tlsAlloc(rmtTLS *)\0"))
                    .as_ptr(),
            );
        }
    };
    if pthread_key_create(handle, None) != 0 as libc::c_int {
        *handle = 0xffffffff as libc::c_uint;
        return RMT_ERROR_TLS_ALLOC_FAIL;
    }
    return RMT_ERROR_NONE;
}
unsafe extern "C" fn tlsFree(mut handle: rmtTLS) {
    if handle != 0xffffffff as libc::c_uint {} else {
        __assert_fail(
            b"handle != TLS_INVALID_HANDLE\0" as *const u8 as *const libc::c_char,
            b"lib/Remotery.c\0" as *const u8 as *const libc::c_char,
            461 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 21],
                &[libc::c_char; 21],
            >(b"void tlsFree(rmtTLS)\0"))
                .as_ptr(),
        );
    }
    'c_26090: {
        if handle != 0xffffffff as libc::c_uint {} else {
            __assert_fail(
                b"handle != TLS_INVALID_HANDLE\0" as *const u8 as *const libc::c_char,
                b"lib/Remotery.c\0" as *const u8 as *const libc::c_char,
                461 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 21],
                    &[libc::c_char; 21],
                >(b"void tlsFree(rmtTLS)\0"))
                    .as_ptr(),
            );
        }
    };
    pthread_key_delete(handle);
}
unsafe extern "C" fn tlsSet(mut handle: rmtTLS, mut value: *mut libc::c_void) {
    if handle != 0xffffffff as libc::c_uint {} else {
        __assert_fail(
            b"handle != TLS_INVALID_HANDLE\0" as *const u8 as *const libc::c_char,
            b"lib/Remotery.c\0" as *const u8 as *const libc::c_char,
            471 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 28],
                &[libc::c_char; 28],
            >(b"void tlsSet(rmtTLS, void *)\0"))
                .as_ptr(),
        );
    }
    'c_6447: {
        if handle != 0xffffffff as libc::c_uint {} else {
            __assert_fail(
                b"handle != TLS_INVALID_HANDLE\0" as *const u8 as *const libc::c_char,
                b"lib/Remotery.c\0" as *const u8 as *const libc::c_char,
                471 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 28],
                    &[libc::c_char; 28],
                >(b"void tlsSet(rmtTLS, void *)\0"))
                    .as_ptr(),
            );
        }
    };
    pthread_setspecific(handle, value);
}
unsafe extern "C" fn tlsGet(mut handle: rmtTLS) -> *mut libc::c_void {
    if handle != 0xffffffff as libc::c_uint {} else {
        __assert_fail(
            b"handle != TLS_INVALID_HANDLE\0" as *const u8 as *const libc::c_char,
            b"lib/Remotery.c\0" as *const u8 as *const libc::c_char,
            481 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 21],
                &[libc::c_char; 21],
            >(b"void *tlsGet(rmtTLS)\0"))
                .as_ptr(),
        );
    }
    'c_638: {
        if handle != 0xffffffff as libc::c_uint {} else {
            __assert_fail(
                b"handle != TLS_INVALID_HANDLE\0" as *const u8 as *const libc::c_char,
                b"lib/Remotery.c\0" as *const u8 as *const libc::c_char,
                481 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 21],
                    &[libc::c_char; 21],
                >(b"void *tlsGet(rmtTLS)\0"))
                    .as_ptr(),
            );
        }
    };
    return pthread_getspecific(handle);
}
static mut g_lastErrorMessageTlsHandle: rmtTLS = 0xffffffff as libc::c_uint;
static mut g_errorMessageSize: rmtU32 = 1024 as libc::c_int as rmtU32;
unsafe extern "C" fn rmtMakeError(
    mut error: rmtError,
    mut error_message: rmtPStr,
) -> rmtError {
    let mut thread_message_ptr: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut error_len: rmtU32 = 0;
    if g_lastErrorMessageTlsHandle == 0xffffffff as libc::c_uint {
        let mut error_0: rmtError = tlsAlloc(&mut g_lastErrorMessageTlsHandle);
        if error_0 as libc::c_uint != RMT_ERROR_NONE as libc::c_int as libc::c_uint {
            return error_0;
        }
    }
    thread_message_ptr = tlsGet(g_lastErrorMessageTlsHandle) as *mut libc::c_char;
    if thread_message_ptr.is_null() {
        thread_message_ptr = rmtMalloc(g_errorMessageSize) as *mut libc::c_char;
        if thread_message_ptr.is_null() {
            return RMT_ERROR_MALLOC_FAIL;
        }
        tlsSet(g_lastErrorMessageTlsHandle, thread_message_ptr as *mut libc::c_void);
    }
    error_len = strlen(error_message) as rmtU32;
    error_len = if error_len >= g_errorMessageSize {
        g_errorMessageSize.wrapping_sub(1 as libc::c_int as libc::c_uint)
    } else {
        error_len
    };
    memcpy(
        thread_message_ptr as *mut libc::c_void,
        error_message as *const libc::c_void,
        error_len as libc::c_ulong,
    );
    *thread_message_ptr.offset(error_len as isize) = 0 as libc::c_int as libc::c_char;
    return error;
}
pub unsafe extern "C" fn rmt_GetLastErrorMessage() -> rmtPStr {
    let mut thread_message_ptr: rmtPStr = 0 as *const libc::c_char;
    if g_lastErrorMessageTlsHandle == 0xffffffff as libc::c_uint {
        return b"No error message\0" as *const u8 as *const libc::c_char;
    }
    thread_message_ptr = tlsGet(g_lastErrorMessageTlsHandle) as rmtPStr;
    if thread_message_ptr.is_null() {
        return b"No error message\0" as *const u8 as *const libc::c_char;
    }
    return thread_message_ptr;
}
unsafe extern "C" fn mtxInit(mut mutex: *mut rmtMutex) {
    if !mutex.is_null() {} else {
        __assert_fail(
            b"mutex != NULL\0" as *const u8 as *const libc::c_char,
            b"lib/Remotery.c\0" as *const u8 as *const libc::c_char,
            571 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 25],
                &[libc::c_char; 25],
            >(b"void mtxInit(rmtMutex *)\0"))
                .as_ptr(),
        );
    }
    'c_25868: {
        if !mutex.is_null() {} else {
            __assert_fail(
                b"mutex != NULL\0" as *const u8 as *const libc::c_char,
                b"lib/Remotery.c\0" as *const u8 as *const libc::c_char,
                571 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 25],
                    &[libc::c_char; 25],
                >(b"void mtxInit(rmtMutex *)\0"))
                    .as_ptr(),
            );
        }
    };
    pthread_mutex_init(mutex, 0 as *const pthread_mutexattr_t);
}
unsafe extern "C" fn mtxLock(mut mutex: *mut rmtMutex) {
    if !mutex.is_null() {} else {
        __assert_fail(
            b"mutex != NULL\0" as *const u8 as *const libc::c_char,
            b"lib/Remotery.c\0" as *const u8 as *const libc::c_char,
            581 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 25],
                &[libc::c_char; 25],
            >(b"void mtxLock(rmtMutex *)\0"))
                .as_ptr(),
        );
    }
    'c_2020: {
        if !mutex.is_null() {} else {
            __assert_fail(
                b"mutex != NULL\0" as *const u8 as *const libc::c_char,
                b"lib/Remotery.c\0" as *const u8 as *const libc::c_char,
                581 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 25],
                    &[libc::c_char; 25],
                >(b"void mtxLock(rmtMutex *)\0"))
                    .as_ptr(),
            );
        }
    };
    pthread_mutex_lock(mutex);
}
unsafe extern "C" fn mtxUnlock(mut mutex: *mut rmtMutex) {
    if !mutex.is_null() {} else {
        __assert_fail(
            b"mutex != NULL\0" as *const u8 as *const libc::c_char,
            b"lib/Remotery.c\0" as *const u8 as *const libc::c_char,
            591 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 27],
                &[libc::c_char; 27],
            >(b"void mtxUnlock(rmtMutex *)\0"))
                .as_ptr(),
        );
    }
    'c_827: {
        if !mutex.is_null() {} else {
            __assert_fail(
                b"mutex != NULL\0" as *const u8 as *const libc::c_char,
                b"lib/Remotery.c\0" as *const u8 as *const libc::c_char,
                591 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 27],
                    &[libc::c_char; 27],
                >(b"void mtxUnlock(rmtMutex *)\0"))
                    .as_ptr(),
            );
        }
    };
    pthread_mutex_unlock(mutex);
}
unsafe extern "C" fn mtxDelete(mut mutex: *mut rmtMutex) {
    if !mutex.is_null() {} else {
        __assert_fail(
            b"mutex != NULL\0" as *const u8 as *const libc::c_char,
            b"lib/Remotery.c\0" as *const u8 as *const libc::c_char,
            601 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 27],
                &[libc::c_char; 27],
            >(b"void mtxDelete(rmtMutex *)\0"))
                .as_ptr(),
        );
    }
    'c_26021: {
        if !mutex.is_null() {} else {
            __assert_fail(
                b"mutex != NULL\0" as *const u8 as *const libc::c_char,
                b"lib/Remotery.c\0" as *const u8 as *const libc::c_char,
                601 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 27],
                    &[libc::c_char; 27],
                >(b"void mtxDelete(rmtMutex *)\0"))
                    .as_ptr(),
            );
        }
    };
    pthread_mutex_destroy(mutex);
}
unsafe extern "C" fn AtomicCompareAndSwapU32(
    mut val: *mut rmtU32,
    mut old_val: libc::c_long,
    mut new_val: libc::c_long,
) -> rmtBool {
    return if (::std::intrinsics::atomic_cxchg_seqcst_seqcst(
        val,
        old_val as rmtU32,
        new_val as rmtU32,
    ))
        .1 as libc::c_int != 0
    {
        1 as libc::c_int as rmtBool
    } else {
        0 as libc::c_int as rmtBool
    };
}
unsafe extern "C" fn AtomicCompareAndSwapPointer(
    mut ptr: *mut *mut libc::c_long,
    mut old_ptr: *mut libc::c_long,
    mut new_ptr: *mut libc::c_long,
) -> rmtBool {
    return if (::std::intrinsics::atomic_cxchg_seqcst_seqcst(ptr, old_ptr, new_ptr)).1
        as libc::c_int != 0
    {
        1 as libc::c_int as rmtBool
    } else {
        0 as libc::c_int as rmtBool
    };
}
unsafe extern "C" fn AtomicAddS32(
    mut value: *mut rmtAtomicS32,
    mut add: rmtS32,
) -> rmtS32 {
    return ::std::intrinsics::atomic_xadd_seqcst(value, add);
}
unsafe extern "C" fn AtomicSubS32(mut value: *mut rmtAtomicS32, mut sub: rmtS32) {
    AtomicAddS32(value, -sub);
}
unsafe extern "C" fn CompilerWriteFence() {
    asm!("", options(preserves_flags, att_syntax));
}
unsafe extern "C" fn CompilerReadFence() {
    asm!("", options(preserves_flags, att_syntax));
}
unsafe extern "C" fn LoadAcquire(mut address: *mut rmtAtomicU32) -> rmtU32 {
    let mut value: rmtU32 = *address;
    CompilerReadFence();
    return value;
}
unsafe extern "C" fn LoadAcquirePointer(
    mut ptr: *mut *mut libc::c_long,
) -> *mut libc::c_long {
    let mut value: *mut libc::c_long = *ptr;
    CompilerReadFence();
    return value;
}
unsafe extern "C" fn StoreRelease(mut address: *mut rmtAtomicU32, mut value: rmtU32) {
    CompilerWriteFence();
    ::std::ptr::write_volatile(address, value);
}
unsafe extern "C" fn StoreReleasePointer(
    mut ptr: *mut *mut libc::c_long,
    mut value: *mut libc::c_long,
) {
    CompilerWriteFence();
    ::std::ptr::write_volatile(ptr, value);
}
static mut Well512_State: [rmtU32; 16] = [0; 16];
static mut Well512_Index: rmtU32 = 0;
unsafe extern "C" fn Well512_Init(mut seed: rmtU32) {
    let mut i: rmtU32 = 0;
    Well512_State[0 as libc::c_int as usize] = seed;
    i = 1 as libc::c_int as rmtU32;
    while i < 16 as libc::c_int as libc::c_uint {
        let mut prev: rmtU32 = Well512_State[i
            .wrapping_sub(1 as libc::c_int as libc::c_uint) as usize];
        Well512_State[i
            as usize] = (1812433253 as libc::c_int as libc::c_uint)
            .wrapping_mul(prev ^ prev >> 30 as libc::c_int)
            .wrapping_add(i);
        i = i.wrapping_add(1);
        i;
    }
    Well512_Index = 0 as libc::c_int as rmtU32;
}
unsafe extern "C" fn Well512_RandomU32() -> rmtU32 {
    let mut a: rmtU32 = 0;
    let mut b: rmtU32 = 0;
    let mut c: rmtU32 = 0;
    let mut d: rmtU32 = 0;
    a = Well512_State[Well512_Index as usize];
    c = Well512_State[(Well512_Index.wrapping_add(13 as libc::c_int as libc::c_uint)
        & 15 as libc::c_int as libc::c_uint) as usize];
    b = a ^ c ^ a << 16 as libc::c_int ^ c << 15 as libc::c_int;
    c = Well512_State[(Well512_Index.wrapping_add(9 as libc::c_int as libc::c_uint)
        & 15 as libc::c_int as libc::c_uint) as usize];
    c ^= c >> 11 as libc::c_int;
    Well512_State[Well512_Index as usize] = b ^ c;
    a = Well512_State[Well512_Index as usize];
    d = (a as libc::c_ulong
        ^ (a << 5 as libc::c_int) as libc::c_ulong & 0xda442d24 as libc::c_ulong)
        as rmtU32;
    Well512_Index = Well512_Index.wrapping_add(15 as libc::c_int as libc::c_uint)
        & 15 as libc::c_int as libc::c_uint;
    a = Well512_State[Well512_Index as usize];
    Well512_State[Well512_Index
        as usize] = a ^ b ^ d ^ a << 2 as libc::c_int ^ b << 18 as libc::c_int
        ^ c << 28 as libc::c_int;
    return Well512_State[Well512_Index as usize];
}
unsafe extern "C" fn Well512_RandomOpenLimit(mut limit: rmtU32) -> rmtU32 {
    let mut bucket_size: rmtU32 = (2147483647 as libc::c_int as libc::c_uint)
        .wrapping_mul(2 as libc::c_uint)
        .wrapping_add(1 as libc::c_uint)
        .wrapping_div(limit);
    let mut bucket_limit: rmtU32 = bucket_size.wrapping_mul(limit);
    let mut r: rmtU32 = 0;
    loop {
        r = Well512_RandomU32();
        if !(r >= bucket_limit) {
            break;
        }
    }
    return r.wrapping_div(bucket_size);
}
unsafe extern "C" fn Log2i(mut x: rmtU32) -> rmtU32 {
    static mut MultiplyDeBruijnBitPosition: [rmtU8; 32] = [
        0 as libc::c_int as rmtU8,
        9 as libc::c_int as rmtU8,
        1 as libc::c_int as rmtU8,
        10 as libc::c_int as rmtU8,
        13 as libc::c_int as rmtU8,
        21 as libc::c_int as rmtU8,
        2 as libc::c_int as rmtU8,
        29 as libc::c_int as rmtU8,
        11 as libc::c_int as rmtU8,
        14 as libc::c_int as rmtU8,
        16 as libc::c_int as rmtU8,
        18 as libc::c_int as rmtU8,
        22 as libc::c_int as rmtU8,
        25 as libc::c_int as rmtU8,
        3 as libc::c_int as rmtU8,
        30 as libc::c_int as rmtU8,
        8 as libc::c_int as rmtU8,
        12 as libc::c_int as rmtU8,
        20 as libc::c_int as rmtU8,
        28 as libc::c_int as rmtU8,
        15 as libc::c_int as rmtU8,
        17 as libc::c_int as rmtU8,
        24 as libc::c_int as rmtU8,
        7 as libc::c_int as rmtU8,
        19 as libc::c_int as rmtU8,
        27 as libc::c_int as rmtU8,
        23 as libc::c_int as rmtU8,
        6 as libc::c_int as rmtU8,
        26 as libc::c_int as rmtU8,
        5 as libc::c_int as rmtU8,
        4 as libc::c_int as rmtU8,
        31 as libc::c_int as rmtU8,
    ];
    x |= x >> 1 as libc::c_int;
    x |= x >> 2 as libc::c_int;
    x |= x >> 4 as libc::c_int;
    x |= x >> 8 as libc::c_int;
    x |= x >> 16 as libc::c_int;
    return MultiplyDeBruijnBitPosition[(x.wrapping_mul(0x7c4acdd as libc::c_uint)
        >> 27 as libc::c_int) as usize] as rmtU32;
}
unsafe extern "C" fn GaloisLFSRMask(mut table_size_log2: rmtU32) -> rmtU32 {
    static mut XORMasks: [rmtU8; 7] = [
        ((1 as libc::c_int) << 0 as libc::c_int | (1 as libc::c_int) << 1 as libc::c_int)
            as rmtU8,
        ((1 as libc::c_int) << 1 as libc::c_int | (1 as libc::c_int) << 2 as libc::c_int)
            as rmtU8,
        ((1 as libc::c_int) << 2 as libc::c_int | (1 as libc::c_int) << 3 as libc::c_int)
            as rmtU8,
        ((1 as libc::c_int) << 2 as libc::c_int | (1 as libc::c_int) << 4 as libc::c_int)
            as rmtU8,
        ((1 as libc::c_int) << 4 as libc::c_int | (1 as libc::c_int) << 5 as libc::c_int)
            as rmtU8,
        ((1 as libc::c_int) << 5 as libc::c_int | (1 as libc::c_int) << 6 as libc::c_int)
            as rmtU8,
        ((1 as libc::c_int) << 3 as libc::c_int | (1 as libc::c_int) << 4 as libc::c_int
            | (1 as libc::c_int) << 5 as libc::c_int
            | (1 as libc::c_int) << 7 as libc::c_int) as rmtU8,
    ];
    if table_size_log2 >= 2 as libc::c_int as libc::c_uint {} else {
        __assert_fail(
            b"table_size_log2 >= 2\0" as *const u8 as *const libc::c_char,
            b"lib/Remotery.c\0" as *const u8 as *const libc::c_char,
            853 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 30],
                &[libc::c_char; 30],
            >(b"rmtU32 GaloisLFSRMask(rmtU32)\0"))
                .as_ptr(),
        );
    }
    'c_24963: {
        if table_size_log2 >= 2 as libc::c_int as libc::c_uint {} else {
            __assert_fail(
                b"table_size_log2 >= 2\0" as *const u8 as *const libc::c_char,
                b"lib/Remotery.c\0" as *const u8 as *const libc::c_char,
                853 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 30],
                    &[libc::c_char; 30],
                >(b"rmtU32 GaloisLFSRMask(rmtU32)\0"))
                    .as_ptr(),
            );
        }
    };
    if table_size_log2 <= 8 as libc::c_int as libc::c_uint {} else {
        __assert_fail(
            b"table_size_log2 <= 8\0" as *const u8 as *const libc::c_char,
            b"lib/Remotery.c\0" as *const u8 as *const libc::c_char,
            854 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 30],
                &[libc::c_char; 30],
            >(b"rmtU32 GaloisLFSRMask(rmtU32)\0"))
                .as_ptr(),
        );
    }
    'c_24925: {
        if table_size_log2 <= 8 as libc::c_int as libc::c_uint {} else {
            __assert_fail(
                b"table_size_log2 <= 8\0" as *const u8 as *const libc::c_char,
                b"lib/Remotery.c\0" as *const u8 as *const libc::c_char,
                854 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 30],
                    &[libc::c_char; 30],
                >(b"rmtU32 GaloisLFSRMask(rmtU32)\0"))
                    .as_ptr(),
            );
        }
    };
    return XORMasks[table_size_log2.wrapping_sub(2 as libc::c_int as libc::c_uint)
        as usize] as rmtU32;
}
unsafe extern "C" fn GaloisLFSRNext(mut value: rmtU32, mut xor_mask: rmtU32) -> rmtU32 {
    let mut lsb: rmtU32 = value & 1 as libc::c_int as libc::c_uint;
    value >>= 1 as libc::c_int;
    if lsb != 0 as libc::c_int as libc::c_uint {
        value ^= xor_mask;
    }
    return value;
}
unsafe extern "C" fn VirtualMirrorBuffer_Constructor(
    mut buffer: *mut VirtualMirrorBuffer,
    mut size: rmtU32,
    mut nb_attempts: libc::c_int,
) -> rmtError {
    static mut k_64: rmtU32 = (64 as libc::c_int * 1024 as libc::c_int) as rmtU32;
    if 1 as libc::c_int != 0 {} else {};
    let mut path: [libc::c_char; 28] = *::std::mem::transmute::<
        &[u8; 28],
        &mut [libc::c_char; 28],
    >(b"/dev/shm/ring-buffer-XXXXXX\0");
    let mut file_descriptor: libc::c_int = 0;
    size = size
        .wrapping_add(k_64)
        .wrapping_sub(1 as libc::c_int as libc::c_uint)
        .wrapping_div(k_64)
        .wrapping_mul(k_64);
    (*buffer).size = size;
    (*buffer).ptr = 0 as *mut rmtU8;
    file_descriptor = mkstemp(path.as_mut_ptr());
    if file_descriptor < 0 as libc::c_int {
        return RMT_ERROR_VIRTUAL_MEMORY_BUFFER_FAIL;
    }
    if unlink(path.as_mut_ptr()) != 0 {
        return RMT_ERROR_VIRTUAL_MEMORY_BUFFER_FAIL;
    }
    if ftruncate(
        file_descriptor,
        size.wrapping_mul(2 as libc::c_int as libc::c_uint) as __off_t,
    ) != 0
    {
        return RMT_ERROR_VIRTUAL_MEMORY_BUFFER_FAIL;
    }
    (*buffer)
        .ptr = mmap(
        0 as *mut libc::c_void,
        size.wrapping_mul(2 as libc::c_int as libc::c_uint) as size_t,
        0 as libc::c_int,
        0x20 as libc::c_int | 0x2 as libc::c_int,
        -(1 as libc::c_int),
        0 as libc::c_int as __off_t,
    ) as *mut rmtU8;
    if (*buffer).ptr == -(1 as libc::c_int) as *mut libc::c_void as *mut rmtU8 {
        (*buffer).ptr = 0 as *mut rmtU8;
        return RMT_ERROR_VIRTUAL_MEMORY_BUFFER_FAIL;
    }
    if mmap(
        (*buffer).ptr as *mut libc::c_void,
        size as size_t,
        0x1 as libc::c_int | 0x2 as libc::c_int,
        0x10 as libc::c_int | 0x1 as libc::c_int,
        file_descriptor,
        0 as libc::c_int as __off_t,
    ) != (*buffer).ptr as *mut libc::c_void
        || mmap(
            ((*buffer).ptr).offset(size as isize) as *mut libc::c_void,
            size as size_t,
            0x1 as libc::c_int | 0x2 as libc::c_int,
            0x10 as libc::c_int | 0x1 as libc::c_int,
            file_descriptor,
            0 as libc::c_int as __off_t,
        ) != ((*buffer).ptr).offset(size as isize) as *mut libc::c_void
    {
        return RMT_ERROR_VIRTUAL_MEMORY_BUFFER_FAIL;
    }
    if ((*buffer).ptr).is_null() {
        return RMT_ERROR_VIRTUAL_MEMORY_BUFFER_FAIL;
    }
    return RMT_ERROR_NONE;
}
unsafe extern "C" fn VirtualMirrorBuffer_Destructor(
    mut buffer: *mut VirtualMirrorBuffer,
) {
    if !buffer.is_null() {} else {
        __assert_fail(
            b"buffer != 0\0" as *const u8 as *const libc::c_char,
            b"lib/Remotery.c\0" as *const u8 as *const libc::c_char,
            1351 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 59],
                &[libc::c_char; 59],
            >(b"void VirtualMirrorBuffer_Destructor(VirtualMirrorBuffer *)\0"))
                .as_ptr(),
        );
    }
    'c_27909: {
        if !buffer.is_null() {} else {
            __assert_fail(
                b"buffer != 0\0" as *const u8 as *const libc::c_char,
                b"lib/Remotery.c\0" as *const u8 as *const libc::c_char,
                1351 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 59],
                    &[libc::c_char; 59],
                >(b"void VirtualMirrorBuffer_Destructor(VirtualMirrorBuffer *)\0"))
                    .as_ptr(),
            );
        }
    };
    if !((*buffer).ptr).is_null() {
        munmap(
            (*buffer).ptr as *mut libc::c_void,
            ((*buffer).size).wrapping_mul(2 as libc::c_int as libc::c_uint) as size_t,
        );
    }
    (*buffer).ptr = 0 as *mut rmtU8;
}
unsafe extern "C" fn strnlen_s_safe_c(
    mut dest: *const libc::c_char,
    mut dmax: r_size_t,
) -> r_size_t {
    let mut count: r_size_t = 0;
    if dest.is_null() {
        return 0 as libc::c_int as r_size_t;
    }
    if dmax == 0 as libc::c_int as libc::c_uint {
        return 0 as libc::c_int as r_size_t;
    }
    if dmax as libc::c_ulong > (4 as libc::c_ulong) << 10 as libc::c_int {
        return 0 as libc::c_int as r_size_t;
    }
    count = 0 as libc::c_int as r_size_t;
    while *dest as libc::c_int != 0 && dmax != 0 {
        count = count.wrapping_add(1);
        count;
        dmax = dmax.wrapping_sub(1);
        dmax;
        dest = dest.offset(1);
        dest;
    }
    return count;
}
unsafe extern "C" fn strstr_s(
    mut dest: *mut libc::c_char,
    mut dmax: r_size_t,
    mut src: *const libc::c_char,
    mut slen: r_size_t,
    mut substring: *mut *mut libc::c_char,
) -> errno_t {
    let mut len: r_size_t = 0;
    let mut dlen: r_size_t = 0;
    let mut i: libc::c_int = 0;
    if substring.is_null() {
        return 400 as libc::c_int;
    }
    *substring = 0 as *mut libc::c_char;
    if dest.is_null() {
        return 400 as libc::c_int;
    }
    if dmax == 0 as libc::c_int as libc::c_uint {
        return 401 as libc::c_int;
    }
    if dmax as libc::c_ulong > (4 as libc::c_ulong) << 10 as libc::c_int {
        return 403 as libc::c_int;
    }
    if src.is_null() {
        return 400 as libc::c_int;
    }
    if slen == 0 as libc::c_int as libc::c_uint {
        return 401 as libc::c_int;
    }
    if slen as libc::c_ulong > (4 as libc::c_ulong) << 10 as libc::c_int {
        return 403 as libc::c_int;
    }
    if *src as libc::c_int == '\0' as i32 || dest == src as *mut libc::c_char {
        *substring = dest;
        return 0 as libc::c_int;
    }
    while *dest as libc::c_int != 0 && dmax != 0 {
        i = 0 as libc::c_int;
        len = slen;
        dlen = dmax;
        while *src.offset(i as isize) as libc::c_int != 0 && dlen != 0 {
            if *dest.offset(i as isize) as libc::c_int
                != *src.offset(i as isize) as libc::c_int
            {
                break;
            }
            i += 1;
            i;
            len = len.wrapping_sub(1);
            len;
            dlen = dlen.wrapping_sub(1);
            dlen;
            if *src.offset(i as isize) as libc::c_int == '\0' as i32 || len == 0 {
                *substring = dest;
                return 0 as libc::c_int;
            }
        }
        dest = dest.offset(1);
        dest;
        dmax = dmax.wrapping_sub(1);
        dmax;
    }
    *substring = 0 as *mut libc::c_char;
    return 409 as libc::c_int;
}
unsafe extern "C" fn strncat_s_safe_c(
    mut dest: *mut libc::c_char,
    mut dmax: r_size_t,
    mut src: *const libc::c_char,
    mut slen: r_size_t,
) -> errno_t {
    let mut overlap_bumper: *const libc::c_char = 0 as *const libc::c_char;
    if dest.is_null() {
        return 400 as libc::c_int;
    }
    if src.is_null() {
        return 400 as libc::c_int;
    }
    if slen as libc::c_ulong > (4 as libc::c_ulong) << 10 as libc::c_int {
        return 403 as libc::c_int;
    }
    if dmax == 0 as libc::c_int as libc::c_uint {
        return 401 as libc::c_int;
    }
    if dmax as libc::c_ulong > (4 as libc::c_ulong) << 10 as libc::c_int {
        return 403 as libc::c_int;
    }
    if dest < src as *mut libc::c_char {
        overlap_bumper = src;
        while *dest as libc::c_int != '\0' as i32 {
            if dest == overlap_bumper as *mut libc::c_char {
                return 404 as libc::c_int;
            }
            dest = dest.offset(1);
            dest;
            dmax = dmax.wrapping_sub(1);
            dmax;
            if dmax == 0 as libc::c_int as libc::c_uint {
                return 407 as libc::c_int;
            }
        }
        while dmax > 0 as libc::c_int as libc::c_uint {
            if dest == overlap_bumper as *mut libc::c_char {
                return 404 as libc::c_int;
            }
            if slen == 0 as libc::c_int as libc::c_uint {
                *dest = '\0' as i32 as libc::c_char;
                return 0 as libc::c_int;
            }
            *dest = *src;
            if *dest as libc::c_int == '\0' as i32 {
                return 0 as libc::c_int;
            }
            dmax = dmax.wrapping_sub(1);
            dmax;
            slen = slen.wrapping_sub(1);
            slen;
            dest = dest.offset(1);
            dest;
            src = src.offset(1);
            src;
        }
    } else {
        overlap_bumper = dest;
        while *dest as libc::c_int != '\0' as i32 {
            dest = dest.offset(1);
            dest;
            dmax = dmax.wrapping_sub(1);
            dmax;
            if dmax == 0 as libc::c_int as libc::c_uint {
                return 407 as libc::c_int;
            }
        }
        while dmax > 0 as libc::c_int as libc::c_uint {
            if src == overlap_bumper {
                return 404 as libc::c_int;
            }
            if slen == 0 as libc::c_int as libc::c_uint {
                *dest = '\0' as i32 as libc::c_char;
                return 0 as libc::c_int;
            }
            *dest = *src;
            if *dest as libc::c_int == '\0' as i32 {
                return 0 as libc::c_int;
            }
            dmax = dmax.wrapping_sub(1);
            dmax;
            slen = slen.wrapping_sub(1);
            slen;
            dest = dest.offset(1);
            dest;
            src = src.offset(1);
            src;
        }
    }
    return 406 as libc::c_int;
}
pub unsafe extern "C" fn strcpy_s_safe_c(
    mut dest: *mut libc::c_char,
    mut dmax: r_size_t,
    mut src: *const libc::c_char,
) -> errno_t {
    let mut overlap_bumper: *const libc::c_char = 0 as *const libc::c_char;
    if dest.is_null() {
        return 400 as libc::c_int;
    }
    if dmax == 0 as libc::c_int as libc::c_uint {
        return 401 as libc::c_int;
    }
    if dmax as libc::c_ulong > (4 as libc::c_ulong) << 10 as libc::c_int {
        return 403 as libc::c_int;
    }
    if src.is_null() {
        *dest = '\0' as i32 as libc::c_char;
        return 400 as libc::c_int;
    }
    if dest == src as *mut libc::c_char {
        return 0 as libc::c_int;
    }
    if dest < src as *mut libc::c_char {
        overlap_bumper = src;
        while dmax > 0 as libc::c_int as libc::c_uint {
            if dest == overlap_bumper as *mut libc::c_char {
                return 404 as libc::c_int;
            }
            *dest = *src;
            if *dest as libc::c_int == '\0' as i32 {
                return 0 as libc::c_int;
            }
            dmax = dmax.wrapping_sub(1);
            dmax;
            dest = dest.offset(1);
            dest;
            src = src.offset(1);
            src;
        }
    } else {
        overlap_bumper = dest;
        while dmax > 0 as libc::c_int as libc::c_uint {
            if src == overlap_bumper {
                return 404 as libc::c_int;
            }
            *dest = *src;
            if *dest as libc::c_int == '\0' as i32 {
                return 0 as libc::c_int;
            }
            dmax = dmax.wrapping_sub(1);
            dmax;
            dest = dest.offset(1);
            dest;
            src = src.offset(1);
            src;
        }
    }
    return 406 as libc::c_int;
}
static mut hex_encoding_table: *const libc::c_char = b"0123456789ABCDEF\0" as *const u8
    as *const libc::c_char;
unsafe extern "C" fn itoahex_s(
    mut dest: *mut libc::c_char,
    mut dmax: r_size_t,
    mut value: rmtS32,
) {
    let mut len: r_size_t = 0;
    let mut halfbytepos: rmtS32 = 0;
    halfbytepos = 8 as libc::c_int;
    while halfbytepos > 1 as libc::c_int {
        halfbytepos -= 1;
        halfbytepos;
        if !(value >> 4 as libc::c_int * halfbytepos & 0xf as libc::c_int != 0) {
            continue;
        }
        halfbytepos += 1;
        halfbytepos;
        break;
    }
    len = 0 as libc::c_int as r_size_t;
    while len.wrapping_add(1 as libc::c_int as libc::c_uint) < dmax
        && halfbytepos > 0 as libc::c_int
    {
        halfbytepos -= 1;
        halfbytepos;
        *dest
            .offset(
                len as isize,
            ) = *hex_encoding_table
            .offset(
                (value >> 4 as libc::c_int * halfbytepos & 0xf as libc::c_int) as isize,
            );
        len = len.wrapping_add(1);
        len;
    }
    if len < dmax {
        *dest.offset(len as isize) = 0 as libc::c_int as libc::c_char;
    }
}
unsafe extern "C" fn itoa_s(mut value: rmtS32) -> *const libc::c_char {
    static mut temp_dest: [libc::c_char; 12] = [
        0 as libc::c_int as libc::c_char,
        0 as libc::c_int as libc::c_char,
        0 as libc::c_int as libc::c_char,
        0 as libc::c_int as libc::c_char,
        0 as libc::c_int as libc::c_char,
        0 as libc::c_int as libc::c_char,
        0 as libc::c_int as libc::c_char,
        0 as libc::c_int as libc::c_char,
        0 as libc::c_int as libc::c_char,
        0 as libc::c_int as libc::c_char,
        0 as libc::c_int as libc::c_char,
        0 as libc::c_int as libc::c_char,
    ];
    let mut pos: libc::c_int = 10 as libc::c_int;
    let mut abs_value: rmtS32 = abs(value);
    while abs_value > 0 as libc::c_int {
        let fresh0 = pos;
        pos = pos - 1;
        temp_dest[fresh0
            as usize] = ('0' as i32 + abs_value % 10 as libc::c_int) as libc::c_char;
        abs_value /= 10 as libc::c_int;
    }
    if value < 0 as libc::c_int {
        let fresh1 = pos;
        pos = pos - 1;
        temp_dest[fresh1 as usize] = '-' as i32 as libc::c_char;
    }
    return temp_dest.as_mut_ptr().offset(pos as isize).offset(1 as libc::c_int as isize);
}
unsafe extern "C" fn rmtGetNbProcessors() -> rmtU32 {
    return 0 as libc::c_int as rmtU32;
}
unsafe extern "C" fn rmtGetCurrentThreadId() -> rmtThreadId {
    return pthread_self();
}
unsafe extern "C" fn rmtSuspendThread(mut thread_handle: rmtThreadHandle) -> rmtBool {
    return 0 as libc::c_int as rmtBool;
}
unsafe extern "C" fn rmtResumeThread(mut thread_handle: rmtThreadHandle) {}
unsafe extern "C" fn rmtGetUserModeThreadContext(
    mut thread: rmtThreadHandle,
    mut context: *mut rmtCpuContext,
) -> rmtBool {
    return 0 as libc::c_int as rmtBool;
}
unsafe extern "C" fn rmtSetThreadContext(
    mut thread_handle: rmtThreadHandle,
    mut context: *mut rmtCpuContext,
) {}
unsafe extern "C" fn rmtOpenThreadHandle(
    mut thread_id: rmtThreadId,
    mut out_thread_handle: *mut rmtThreadHandle,
) -> rmtError {
    return RMT_ERROR_NONE;
}
unsafe extern "C" fn rmtCloseThreadHandle(mut thread_handle: rmtThreadHandle) {}
unsafe extern "C" fn rmtGetThreadNameFallback(
    mut out_thread_name: *mut libc::c_char,
    mut thread_name_size: rmtU32,
) {
    static mut countThreads: rmtS32 = 0 as libc::c_int;
    *out_thread_name
        .offset(0 as libc::c_int as isize) = 0 as libc::c_int as libc::c_char;
    strncat_s_safe_c(
        out_thread_name,
        thread_name_size,
        b"Thread\0" as *const u8 as *const libc::c_char,
        6 as libc::c_int as r_size_t,
    );
    itoahex_s(
        out_thread_name.offset(6 as libc::c_int as isize),
        thread_name_size.wrapping_sub(6 as libc::c_int as libc::c_uint),
        AtomicAddS32(
            &mut countThreads as *mut rmtS32 as *mut rmtAtomicS32,
            1 as libc::c_int,
        ),
    );
}
unsafe extern "C" fn rmtGetThreadName(
    mut thread_id: rmtThreadId,
    mut thread_handle: rmtThreadHandle,
    mut out_thread_name: *mut libc::c_char,
    mut thread_name_size: rmtU32,
) {
    rmtGetThreadNameFallback(out_thread_name, thread_name_size);
}
unsafe extern "C" fn StartFunc(mut pArgs: *mut libc::c_void) -> *mut libc::c_void {
    let mut thread: *mut rmtThread = pArgs as *mut rmtThread;
    if !thread.is_null() {} else {
        __assert_fail(
            b"thread != NULL\0" as *const u8 as *const libc::c_char,
            b"lib/Remotery.c\0" as *const u8 as *const libc::c_char,
            2156 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 24],
                &[libc::c_char; 24],
            >(b"void *StartFunc(void *)\0"))
                .as_ptr(),
        );
    }
    'c_22852: {
        if !thread.is_null() {} else {
            __assert_fail(
                b"thread != NULL\0" as *const u8 as *const libc::c_char,
                b"lib/Remotery.c\0" as *const u8 as *const libc::c_char,
                2156 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 24],
                    &[libc::c_char; 24],
                >(b"void *StartFunc(void *)\0"))
                    .as_ptr(),
            );
        }
    };
    (*thread).error = ((*thread).callback).unwrap()(thread);
    return 0 as *mut libc::c_void;
}
unsafe extern "C" fn rmtThread_Valid(mut thread: *mut rmtThread) -> libc::c_int {
    if !thread.is_null() {} else {
        __assert_fail(
            b"thread != NULL\0" as *const u8 as *const libc::c_char,
            b"lib/Remotery.c\0" as *const u8 as *const libc::c_char,
            2164 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 33],
                &[libc::c_char; 33],
            >(b"int rmtThread_Valid(rmtThread *)\0"))
                .as_ptr(),
        );
    }
    'c_23103: {
        if !thread.is_null() {} else {
            __assert_fail(
                b"thread != NULL\0" as *const u8 as *const libc::c_char,
                b"lib/Remotery.c\0" as *const u8 as *const libc::c_char,
                2164 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 33],
                    &[libc::c_char; 33],
                >(b"int rmtThread_Valid(rmtThread *)\0"))
                    .as_ptr(),
            );
        }
    };
    return (pthread_equal((*thread).handle, pthread_self()) == 0) as libc::c_int;
}
unsafe extern "C" fn rmtThread_Constructor(
    mut thread: *mut rmtThread,
    mut callback: ThreadProc,
    mut param: *mut libc::c_void,
) -> rmtError {
    if !thread.is_null() {} else {
        __assert_fail(
            b"thread != NULL\0" as *const u8 as *const libc::c_char,
            b"lib/Remotery.c\0" as *const u8 as *const libc::c_char,
            2175 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 64],
                &[libc::c_char; 64],
            >(b"rmtError rmtThread_Constructor(rmtThread *, ThreadProc, void *)\0"))
                .as_ptr(),
        );
    }
    'c_22945: {
        if !thread.is_null() {} else {
            __assert_fail(
                b"thread != NULL\0" as *const u8 as *const libc::c_char,
                b"lib/Remotery.c\0" as *const u8 as *const libc::c_char,
                2175 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 64],
                    &[libc::c_char; 64],
                >(b"rmtError rmtThread_Constructor(rmtThread *, ThreadProc, void *)\0"))
                    .as_ptr(),
            );
        }
    };
    (*thread).callback = callback;
    (*thread).param = param;
    (*thread).error = RMT_ERROR_NONE;
    ::std::ptr::write_volatile(
        &mut (*thread).request_exit as *mut rmtBool,
        0 as libc::c_int as rmtBool,
    );
    let mut error: int32_t = pthread_create(
        &mut (*thread).handle,
        0 as *const pthread_attr_t,
        Some(StartFunc as unsafe extern "C" fn(*mut libc::c_void) -> *mut libc::c_void),
        thread as *mut libc::c_void,
    );
    if error != 0 {
        (*thread).handle = pthread_self();
        return RMT_ERROR_CREATE_THREAD_FAIL;
    }
    return RMT_ERROR_NONE;
}
unsafe extern "C" fn rmtThread_RequestExit(mut thread: *mut rmtThread) {
    if !thread.is_null() {} else {
        __assert_fail(
            b"thread != NULL\0" as *const u8 as *const libc::c_char,
            b"lib/Remotery.c\0" as *const u8 as *const libc::c_char,
            2215 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 40],
                &[libc::c_char; 40],
            >(b"void rmtThread_RequestExit(rmtThread *)\0"))
                .as_ptr(),
        );
    }
    'c_23173: {
        if !thread.is_null() {} else {
            __assert_fail(
                b"thread != NULL\0" as *const u8 as *const libc::c_char,
                b"lib/Remotery.c\0" as *const u8 as *const libc::c_char,
                2215 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 40],
                    &[libc::c_char; 40],
                >(b"void rmtThread_RequestExit(rmtThread *)\0"))
                    .as_ptr(),
            );
        }
    };
    ::std::ptr::write_volatile(
        &mut (*thread).request_exit as *mut rmtBool,
        1 as libc::c_int as rmtBool,
    );
}
unsafe extern "C" fn rmtThread_Join(mut thread: *mut rmtThread) {
    if rmtThread_Valid(thread) != 0 {} else {
        __assert_fail(
            b"rmtThread_Valid(thread)\0" as *const u8 as *const libc::c_char,
            b"lib/Remotery.c\0" as *const u8 as *const libc::c_char,
            2221 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 33],
                &[libc::c_char; 33],
            >(b"void rmtThread_Join(rmtThread *)\0"))
                .as_ptr(),
        );
    }
    'c_23052: {
        if rmtThread_Valid(thread) != 0 {} else {
            __assert_fail(
                b"rmtThread_Valid(thread)\0" as *const u8 as *const libc::c_char,
                b"lib/Remotery.c\0" as *const u8 as *const libc::c_char,
                2221 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 33],
                    &[libc::c_char; 33],
                >(b"void rmtThread_Join(rmtThread *)\0"))
                    .as_ptr(),
            );
        }
    };
    pthread_join((*thread).handle, 0 as *mut *mut libc::c_void);
}
unsafe extern "C" fn rmtThread_Destructor(mut thread: *mut rmtThread) {
    if !thread.is_null() {} else {
        __assert_fail(
            b"thread != NULL\0" as *const u8 as *const libc::c_char,
            b"lib/Remotery.c\0" as *const u8 as *const libc::c_char,
            2232 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 39],
                &[libc::c_char; 39],
            >(b"void rmtThread_Destructor(rmtThread *)\0"))
                .as_ptr(),
        );
    }
    'c_23220: {
        if !thread.is_null() {} else {
            __assert_fail(
                b"thread != NULL\0" as *const u8 as *const libc::c_char,
                b"lib/Remotery.c\0" as *const u8 as *const libc::c_char,
                2232 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 39],
                    &[libc::c_char; 39],
                >(b"void rmtThread_Destructor(rmtThread *)\0"))
                    .as_ptr(),
            );
        }
    };
    if rmtThread_Valid(thread) != 0 {
        rmtThread_RequestExit(thread);
        rmtThread_Join(thread);
    }
}
unsafe extern "C" fn ObjectLink_Constructor(mut link: *mut ObjectLink) {
    if !link.is_null() {} else {
        __assert_fail(
            b"link != NULL\0" as *const u8 as *const libc::c_char,
            b"lib/Remotery.c\0" as *const u8 as *const libc::c_char,
            2267 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 42],
                &[libc::c_char; 42],
            >(b"void ObjectLink_Constructor(ObjectLink *)\0"))
                .as_ptr(),
        );
    }
    'c_7233: {
        if !link.is_null() {} else {
            __assert_fail(
                b"link != NULL\0" as *const u8 as *const libc::c_char,
                b"lib/Remotery.c\0" as *const u8 as *const libc::c_char,
                2267 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 42],
                    &[libc::c_char; 42],
                >(b"void ObjectLink_Constructor(ObjectLink *)\0"))
                    .as_ptr(),
            );
        }
    };
    ::std::ptr::write_volatile(
        &mut (*link).next as *mut *mut ObjectLink_s,
        0 as *mut ObjectLink_s,
    );
}
unsafe extern "C" fn ObjectAllocator_Constructor(
    mut allocator: *mut ObjectAllocator,
    mut object_size: rmtU32,
    mut constructor: ObjConstructor,
    mut destructor: ObjDestructor,
) -> rmtError {
    (*allocator).object_size = object_size;
    (*allocator).constructor = constructor;
    (*allocator).destructor = destructor;
    ::std::ptr::write_volatile(
        &mut (*allocator).nb_free as *mut rmtAtomicS32,
        0 as libc::c_int,
    );
    ::std::ptr::write_volatile(
        &mut (*allocator).nb_inuse as *mut rmtAtomicS32,
        0 as libc::c_int,
    );
    ::std::ptr::write_volatile(
        &mut (*allocator).nb_allocated as *mut rmtAtomicS32,
        0 as libc::c_int,
    );
    (*allocator).first_free = 0 as *mut ObjectLink;
    return RMT_ERROR_NONE;
}
unsafe extern "C" fn ObjectAllocator_Destructor(mut allocator: *mut ObjectAllocator) {
    if !allocator.is_null() {} else {
        __assert_fail(
            b"allocator != NULL\0" as *const u8 as *const libc::c_char,
            b"lib/Remotery.c\0" as *const u8 as *const libc::c_char,
            2309 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 51],
                &[libc::c_char; 51],
            >(b"void ObjectAllocator_Destructor(ObjectAllocator *)\0"))
                .as_ptr(),
        );
    }
    'c_6826: {
        if !allocator.is_null() {} else {
            __assert_fail(
                b"allocator != NULL\0" as *const u8 as *const libc::c_char,
                b"lib/Remotery.c\0" as *const u8 as *const libc::c_char,
                2309 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 51],
                    &[libc::c_char; 51],
                >(b"void ObjectAllocator_Destructor(ObjectAllocator *)\0"))
                    .as_ptr(),
            );
        }
    };
    if (*allocator).nb_inuse == 0 as libc::c_int {} else {
        __assert_fail(
            b"allocator->nb_inuse == 0\0" as *const u8 as *const libc::c_char,
            b"lib/Remotery.c\0" as *const u8 as *const libc::c_char,
            2310 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 51],
                &[libc::c_char; 51],
            >(b"void ObjectAllocator_Destructor(ObjectAllocator *)\0"))
                .as_ptr(),
        );
    }
    'c_6786: {
        if (*allocator).nb_inuse == 0 as libc::c_int {} else {
            __assert_fail(
                b"allocator->nb_inuse == 0\0" as *const u8 as *const libc::c_char,
                b"lib/Remotery.c\0" as *const u8 as *const libc::c_char,
                2310 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 51],
                    &[libc::c_char; 51],
                >(b"void ObjectAllocator_Destructor(ObjectAllocator *)\0"))
                    .as_ptr(),
            );
        }
    };
    while !((*allocator).first_free).is_null() {
        let mut next: *mut ObjectLink = (*(*allocator).first_free).next;
        if ((*allocator).destructor).is_some() {} else {
            __assert_fail(
                b"allocator->destructor != NULL\0" as *const u8 as *const libc::c_char,
                b"lib/Remotery.c\0" as *const u8 as *const libc::c_char,
                2316 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 51],
                    &[libc::c_char; 51],
                >(b"void ObjectAllocator_Destructor(ObjectAllocator *)\0"))
                    .as_ptr(),
            );
        }
        'c_6732: {
            if ((*allocator).destructor).is_some() {} else {
                __assert_fail(
                    b"allocator->destructor != NULL\0" as *const u8
                        as *const libc::c_char,
                    b"lib/Remotery.c\0" as *const u8 as *const libc::c_char,
                    2316 as libc::c_int as libc::c_uint,
                    (*::std::mem::transmute::<
                        &[u8; 51],
                        &[libc::c_char; 51],
                    >(b"void ObjectAllocator_Destructor(ObjectAllocator *)\0"))
                        .as_ptr(),
                );
            }
        };
        ((*allocator).destructor).unwrap()((*allocator).first_free as *mut libc::c_void);
        rmtFree((*allocator).first_free as *mut libc::c_void);
        (*allocator).first_free = next;
    }
}
unsafe extern "C" fn ObjectAllocator_Push(
    mut allocator: *mut ObjectAllocator,
    mut start: *mut ObjectLink,
    mut end: *mut ObjectLink,
) {
    if !allocator.is_null() {} else {
        __assert_fail(
            b"allocator != NULL\0" as *const u8 as *const libc::c_char,
            b"lib/Remotery.c\0" as *const u8 as *const libc::c_char,
            2325 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 73],
                &[libc::c_char; 73],
            >(
                b"void ObjectAllocator_Push(ObjectAllocator *, ObjectLink *, ObjectLink *)\0",
            ))
                .as_ptr(),
        );
    }
    'c_2474: {
        if !allocator.is_null() {} else {
            __assert_fail(
                b"allocator != NULL\0" as *const u8 as *const libc::c_char,
                b"lib/Remotery.c\0" as *const u8 as *const libc::c_char,
                2325 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 73],
                    &[libc::c_char; 73],
                >(
                    b"void ObjectAllocator_Push(ObjectAllocator *, ObjectLink *, ObjectLink *)\0",
                ))
                    .as_ptr(),
            );
        }
    };
    if !start.is_null() {} else {
        __assert_fail(
            b"start != NULL\0" as *const u8 as *const libc::c_char,
            b"lib/Remotery.c\0" as *const u8 as *const libc::c_char,
            2326 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 73],
                &[libc::c_char; 73],
            >(
                b"void ObjectAllocator_Push(ObjectAllocator *, ObjectLink *, ObjectLink *)\0",
            ))
                .as_ptr(),
        );
    }
    'c_2432: {
        if !start.is_null() {} else {
            __assert_fail(
                b"start != NULL\0" as *const u8 as *const libc::c_char,
                b"lib/Remotery.c\0" as *const u8 as *const libc::c_char,
                2326 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 73],
                    &[libc::c_char; 73],
                >(
                    b"void ObjectAllocator_Push(ObjectAllocator *, ObjectLink *, ObjectLink *)\0",
                ))
                    .as_ptr(),
            );
        }
    };
    if !end.is_null() {} else {
        __assert_fail(
            b"end != NULL\0" as *const u8 as *const libc::c_char,
            b"lib/Remotery.c\0" as *const u8 as *const libc::c_char,
            2327 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 73],
                &[libc::c_char; 73],
            >(
                b"void ObjectAllocator_Push(ObjectAllocator *, ObjectLink *, ObjectLink *)\0",
            ))
                .as_ptr(),
        );
    }
    'c_2388: {
        if !end.is_null() {} else {
            __assert_fail(
                b"end != NULL\0" as *const u8 as *const libc::c_char,
                b"lib/Remotery.c\0" as *const u8 as *const libc::c_char,
                2327 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 73],
                    &[libc::c_char; 73],
                >(
                    b"void ObjectAllocator_Push(ObjectAllocator *, ObjectLink *, ObjectLink *)\0",
                ))
                    .as_ptr(),
            );
        }
    };
    loop {
        let mut old_link: *mut ObjectLink = (*allocator).first_free;
        ::std::ptr::write_volatile(&mut (*end).next as *mut *mut ObjectLink_s, old_link);
        if AtomicCompareAndSwapPointer(
            &mut (*allocator).first_free as *mut *mut ObjectLink
                as *mut *mut libc::c_long,
            old_link as *mut libc::c_long,
            start as *mut libc::c_long,
        ) == 1 as libc::c_int as rmtBool
        {
            break;
        }
    };
}
unsafe extern "C" fn ObjectAllocator_Pop(
    mut allocator: *mut ObjectAllocator,
) -> *mut ObjectLink {
    let mut link: *mut ObjectLink = 0 as *mut ObjectLink;
    if !allocator.is_null() {} else {
        __assert_fail(
            b"allocator != NULL\0" as *const u8 as *const libc::c_char,
            b"lib/Remotery.c\0" as *const u8 as *const libc::c_char,
            2344 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 51],
                &[libc::c_char; 51],
            >(b"ObjectLink *ObjectAllocator_Pop(ObjectAllocator *)\0"))
                .as_ptr(),
        );
    }
    'c_3234: {
        if !allocator.is_null() {} else {
            __assert_fail(
                b"allocator != NULL\0" as *const u8 as *const libc::c_char,
                b"lib/Remotery.c\0" as *const u8 as *const libc::c_char,
                2344 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 51],
                    &[libc::c_char; 51],
                >(b"ObjectLink *ObjectAllocator_Pop(ObjectAllocator *)\0"))
                    .as_ptr(),
            );
        }
    };
    loop {
        let mut old_link: *mut ObjectLink = (*allocator).first_free;
        if old_link.is_null() {
            return 0 as *mut ObjectLink;
        }
        let mut next_link: *mut ObjectLink = (*old_link).next;
        if !(AtomicCompareAndSwapPointer(
            &mut (*allocator).first_free as *mut *mut ObjectLink
                as *mut *mut libc::c_long,
            old_link as *mut libc::c_long,
            next_link as *mut libc::c_long,
        ) == 1 as libc::c_int as rmtBool)
        {
            continue;
        }
        link = old_link;
        break;
    }
    ::std::ptr::write_volatile(
        &mut (*link).next as *mut *mut ObjectLink_s,
        0 as *mut ObjectLink_s,
    );
    return link;
}
unsafe extern "C" fn ObjectAllocator_Alloc(
    mut allocator: *mut ObjectAllocator,
    mut object: *mut *mut libc::c_void,
) -> rmtError {
    if !allocator.is_null() {} else {
        __assert_fail(
            b"allocator != NULL\0" as *const u8 as *const libc::c_char,
            b"lib/Remotery.c\0" as *const u8 as *const libc::c_char,
            2372 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 59],
                &[libc::c_char; 59],
            >(b"rmtError ObjectAllocator_Alloc(ObjectAllocator *, void **)\0"))
                .as_ptr(),
        );
    }
    'c_3321: {
        if !allocator.is_null() {} else {
            __assert_fail(
                b"allocator != NULL\0" as *const u8 as *const libc::c_char,
                b"lib/Remotery.c\0" as *const u8 as *const libc::c_char,
                2372 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 59],
                    &[libc::c_char; 59],
                >(b"rmtError ObjectAllocator_Alloc(ObjectAllocator *, void **)\0"))
                    .as_ptr(),
            );
        }
    };
    if !object.is_null() {} else {
        __assert_fail(
            b"object != NULL\0" as *const u8 as *const libc::c_char,
            b"lib/Remotery.c\0" as *const u8 as *const libc::c_char,
            2373 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 59],
                &[libc::c_char; 59],
            >(b"rmtError ObjectAllocator_Alloc(ObjectAllocator *, void **)\0"))
                .as_ptr(),
        );
    }
    'c_3279: {
        if !object.is_null() {} else {
            __assert_fail(
                b"object != NULL\0" as *const u8 as *const libc::c_char,
                b"lib/Remotery.c\0" as *const u8 as *const libc::c_char,
                2373 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 59],
                    &[libc::c_char; 59],
                >(b"rmtError ObjectAllocator_Alloc(ObjectAllocator *, void **)\0"))
                    .as_ptr(),
            );
        }
    };
    *object = ObjectAllocator_Pop(allocator) as *mut libc::c_void;
    if (*object).is_null() {
        let mut error: rmtError = RMT_ERROR_NONE;
        *object = rmtMalloc((*allocator).object_size);
        if (*object).is_null() {
            return RMT_ERROR_MALLOC_FAIL;
        }
        if ((*allocator).constructor).is_some() {} else {
            __assert_fail(
                b"allocator->constructor != NULL\0" as *const u8 as *const libc::c_char,
                b"lib/Remotery.c\0" as *const u8 as *const libc::c_char,
                2387 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 59],
                    &[libc::c_char; 59],
                >(b"rmtError ObjectAllocator_Alloc(ObjectAllocator *, void **)\0"))
                    .as_ptr(),
            );
        }
        'c_3060: {
            if ((*allocator).constructor).is_some() {} else {
                __assert_fail(
                    b"allocator->constructor != NULL\0" as *const u8
                        as *const libc::c_char,
                    b"lib/Remotery.c\0" as *const u8 as *const libc::c_char,
                    2387 as libc::c_int as libc::c_uint,
                    (*::std::mem::transmute::<
                        &[u8; 59],
                        &[libc::c_char; 59],
                    >(b"rmtError ObjectAllocator_Alloc(ObjectAllocator *, void **)\0"))
                        .as_ptr(),
                );
            }
        };
        error = ((*allocator).constructor).unwrap()(*object);
        if error as libc::c_uint != RMT_ERROR_NONE as libc::c_int as libc::c_uint {
            if ((*allocator).destructor).is_some() {} else {
                __assert_fail(
                    b"allocator->destructor != NULL\0" as *const u8
                        as *const libc::c_char,
                    b"lib/Remotery.c\0" as *const u8 as *const libc::c_char,
                    2392 as libc::c_int as libc::c_uint,
                    (*::std::mem::transmute::<
                        &[u8; 59],
                        &[libc::c_char; 59],
                    >(b"rmtError ObjectAllocator_Alloc(ObjectAllocator *, void **)\0"))
                        .as_ptr(),
                );
            }
            'c_2996: {
                if ((*allocator).destructor).is_some() {} else {
                    __assert_fail(
                        b"allocator->destructor != NULL\0" as *const u8
                            as *const libc::c_char,
                        b"lib/Remotery.c\0" as *const u8 as *const libc::c_char,
                        2392 as libc::c_int as libc::c_uint,
                        (*::std::mem::transmute::<
                            &[u8; 59],
                            &[libc::c_char; 59],
                        >(
                            b"rmtError ObjectAllocator_Alloc(ObjectAllocator *, void **)\0",
                        ))
                            .as_ptr(),
                    );
                }
            };
            ((*allocator).destructor).unwrap()(*object);
            rmtFree(*object);
            return error;
        }
        AtomicAddS32(&mut (*allocator).nb_allocated, 1 as libc::c_int);
    } else {
        AtomicSubS32(&mut (*allocator).nb_free, 1 as libc::c_int);
    }
    AtomicAddS32(&mut (*allocator).nb_inuse, 1 as libc::c_int);
    return RMT_ERROR_NONE;
}
unsafe extern "C" fn ObjectAllocator_Free(
    mut allocator: *mut ObjectAllocator,
    mut object: *mut libc::c_void,
) {
    if !allocator.is_null() {} else {
        __assert_fail(
            b"allocator != NULL\0" as *const u8 as *const libc::c_char,
            b"lib/Remotery.c\0" as *const u8 as *const libc::c_char,
            2413 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 53],
                &[libc::c_char; 53],
            >(b"void ObjectAllocator_Free(ObjectAllocator *, void *)\0"))
                .as_ptr(),
        );
    }
    'c_2517: {
        if !allocator.is_null() {} else {
            __assert_fail(
                b"allocator != NULL\0" as *const u8 as *const libc::c_char,
                b"lib/Remotery.c\0" as *const u8 as *const libc::c_char,
                2413 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 53],
                    &[libc::c_char; 53],
                >(b"void ObjectAllocator_Free(ObjectAllocator *, void *)\0"))
                    .as_ptr(),
            );
        }
    };
    ObjectAllocator_Push(
        allocator,
        object as *mut ObjectLink,
        object as *mut ObjectLink,
    );
    AtomicSubS32(&mut (*allocator).nb_inuse, 1 as libc::c_int);
    AtomicAddS32(&mut (*allocator).nb_free, 1 as libc::c_int);
}
unsafe extern "C" fn ObjectAllocator_FreeRange(
    mut allocator: *mut ObjectAllocator,
    mut start: *mut libc::c_void,
    mut end: *mut libc::c_void,
    mut count: rmtU32,
) {
    if !allocator.is_null() {} else {
        __assert_fail(
            b"allocator != NULL\0" as *const u8 as *const libc::c_char,
            b"lib/Remotery.c\0" as *const u8 as *const libc::c_char,
            2421 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 74],
                &[libc::c_char; 74],
            >(
                b"void ObjectAllocator_FreeRange(ObjectAllocator *, void *, void *, rmtU32)\0",
            ))
                .as_ptr(),
        );
    }
    'c_4674: {
        if !allocator.is_null() {} else {
            __assert_fail(
                b"allocator != NULL\0" as *const u8 as *const libc::c_char,
                b"lib/Remotery.c\0" as *const u8 as *const libc::c_char,
                2421 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 74],
                    &[libc::c_char; 74],
                >(
                    b"void ObjectAllocator_FreeRange(ObjectAllocator *, void *, void *, rmtU32)\0",
                ))
                    .as_ptr(),
            );
        }
    };
    ObjectAllocator_Push(allocator, start as *mut ObjectLink, end as *mut ObjectLink);
    AtomicSubS32(&mut (*allocator).nb_inuse, count as rmtS32);
    AtomicAddS32(&mut (*allocator).nb_free, count as rmtS32);
}
unsafe extern "C" fn Buffer_Constructor(
    mut buffer: *mut Buffer,
    mut alloc_granularity: rmtU32,
) -> rmtError {
    if !buffer.is_null() {} else {
        __assert_fail(
            b"buffer != NULL\0" as *const u8 as *const libc::c_char,
            b"lib/Remotery.c\0" as *const u8 as *const libc::c_char,
            2447 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 46],
                &[libc::c_char; 46],
            >(b"rmtError Buffer_Constructor(Buffer *, rmtU32)\0"))
                .as_ptr(),
        );
    }
    'c_27052: {
        if !buffer.is_null() {} else {
            __assert_fail(
                b"buffer != NULL\0" as *const u8 as *const libc::c_char,
                b"lib/Remotery.c\0" as *const u8 as *const libc::c_char,
                2447 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 46],
                    &[libc::c_char; 46],
                >(b"rmtError Buffer_Constructor(Buffer *, rmtU32)\0"))
                    .as_ptr(),
            );
        }
    };
    (*buffer).alloc_granularity = alloc_granularity;
    (*buffer).bytes_allocated = 0 as libc::c_int as rmtU32;
    (*buffer).bytes_used = 0 as libc::c_int as rmtU32;
    (*buffer).data = 0 as *mut rmtU8;
    return RMT_ERROR_NONE;
}
unsafe extern "C" fn Buffer_Destructor(mut buffer: *mut Buffer) {
    if !buffer.is_null() {} else {
        __assert_fail(
            b"buffer != NULL\0" as *const u8 as *const libc::c_char,
            b"lib/Remotery.c\0" as *const u8 as *const libc::c_char,
            2457 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 33],
                &[libc::c_char; 33],
            >(b"void Buffer_Destructor(Buffer *)\0"))
                .as_ptr(),
        );
    }
    'c_27156: {
        if !buffer.is_null() {} else {
            __assert_fail(
                b"buffer != NULL\0" as *const u8 as *const libc::c_char,
                b"lib/Remotery.c\0" as *const u8 as *const libc::c_char,
                2457 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 33],
                    &[libc::c_char; 33],
                >(b"void Buffer_Destructor(Buffer *)\0"))
                    .as_ptr(),
            );
        }
    };
    if !((*buffer).data).is_null() {
        rmtFree((*buffer).data as *mut libc::c_void);
        (*buffer).data = 0 as *mut rmtU8;
    }
}
unsafe extern "C" fn Buffer_Grow(
    mut buffer: *mut Buffer,
    mut length: rmtU32,
) -> rmtError {
    let mut granularity: rmtU32 = (*buffer).alloc_granularity;
    let mut allocate: rmtU32 = ((*buffer).bytes_allocated).wrapping_add(length);
    allocate = allocate
        .wrapping_add(
            granularity
                .wrapping_sub(1 as libc::c_int as libc::c_uint)
                .wrapping_sub(
                    allocate
                        .wrapping_sub(1 as libc::c_int as libc::c_uint)
                        .wrapping_rem(granularity),
                ),
        );
    (*buffer).bytes_allocated = allocate;
    (*buffer)
        .data = rmtRealloc(
        (*buffer).data as *mut libc::c_void,
        (*buffer).bytes_allocated,
    ) as *mut rmtU8;
    if ((*buffer).data).is_null() {
        return RMT_ERROR_MALLOC_FAIL;
    }
    return RMT_ERROR_NONE;
}
unsafe extern "C" fn Buffer_Pad(
    mut buffer: *mut Buffer,
    mut length: rmtU32,
) -> rmtError {
    if !buffer.is_null() {} else {
        __assert_fail(
            b"buffer != NULL\0" as *const u8 as *const libc::c_char,
            b"lib/Remotery.c\0" as *const u8 as *const libc::c_char,
            2483 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 38],
                &[libc::c_char; 38],
            >(b"rmtError Buffer_Pad(Buffer *, rmtU32)\0"))
                .as_ptr(),
        );
    }
    'c_12279: {
        if !buffer.is_null() {} else {
            __assert_fail(
                b"buffer != NULL\0" as *const u8 as *const libc::c_char,
                b"lib/Remotery.c\0" as *const u8 as *const libc::c_char,
                2483 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 38],
                    &[libc::c_char; 38],
                >(b"rmtError Buffer_Pad(Buffer *, rmtU32)\0"))
                    .as_ptr(),
            );
        }
    };
    if ((*buffer).bytes_used).wrapping_add(length) > (*buffer).bytes_allocated {
        let mut error: rmtError = Buffer_Grow(buffer, length);
        if error as libc::c_uint != RMT_ERROR_NONE as libc::c_int as libc::c_uint {
            return error;
        }
    }
    (*buffer)
        .bytes_used = ((*buffer).bytes_used as libc::c_uint).wrapping_add(length)
        as rmtU32 as rmtU32;
    return RMT_ERROR_NONE;
}
unsafe extern "C" fn Buffer_AlignedPad(
    mut buffer: *mut Buffer,
    mut start_pos: rmtU32,
) -> rmtError {
    return Buffer_Pad(
        buffer,
        (4 as libc::c_int as libc::c_uint)
            .wrapping_sub(
                ((*buffer).bytes_used).wrapping_sub(start_pos)
                    & 3 as libc::c_int as libc::c_uint,
            ) & 3 as libc::c_int as libc::c_uint,
    );
}
unsafe extern "C" fn Buffer_Write(
    mut buffer: *mut Buffer,
    mut data: *const libc::c_void,
    mut length: rmtU32,
) -> rmtError {
    if !buffer.is_null() {} else {
        __assert_fail(
            b"buffer != NULL\0" as *const u8 as *const libc::c_char,
            b"lib/Remotery.c\0" as *const u8 as *const libc::c_char,
            2504 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 54],
                &[libc::c_char; 54],
            >(b"rmtError Buffer_Write(Buffer *, const void *, rmtU32)\0"))
                .as_ptr(),
        );
    }
    'c_13343: {
        if !buffer.is_null() {} else {
            __assert_fail(
                b"buffer != NULL\0" as *const u8 as *const libc::c_char,
                b"lib/Remotery.c\0" as *const u8 as *const libc::c_char,
                2504 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 54],
                    &[libc::c_char; 54],
                >(b"rmtError Buffer_Write(Buffer *, const void *, rmtU32)\0"))
                    .as_ptr(),
            );
        }
    };
    if ((*buffer).bytes_used).wrapping_add(length) > (*buffer).bytes_allocated {
        let mut error: rmtError = Buffer_Grow(buffer, length);
        if error as libc::c_uint != RMT_ERROR_NONE as libc::c_int as libc::c_uint {
            return error;
        }
    }
    memcpy(
        ((*buffer).data).offset((*buffer).bytes_used as isize) as *mut libc::c_void,
        data,
        length as libc::c_ulong,
    );
    (*buffer)
        .bytes_used = ((*buffer).bytes_used as libc::c_uint).wrapping_add(length)
        as rmtU32 as rmtU32;
    return RMT_ERROR_NONE;
}
unsafe extern "C" fn Buffer_WriteStringZ(
    mut buffer: *mut Buffer,
    mut string: rmtPStr,
) -> rmtError {
    if !string.is_null() {} else {
        __assert_fail(
            b"string != NULL\0" as *const u8 as *const libc::c_char,
            b"lib/Remotery.c\0" as *const u8 as *const libc::c_char,
            2521 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 48],
                &[libc::c_char; 48],
            >(b"rmtError Buffer_WriteStringZ(Buffer *, rmtPStr)\0"))
                .as_ptr(),
        );
    }
    'c_16860: {
        if !string.is_null() {} else {
            __assert_fail(
                b"string != NULL\0" as *const u8 as *const libc::c_char,
                b"lib/Remotery.c\0" as *const u8 as *const libc::c_char,
                2521 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 48],
                    &[libc::c_char; 48],
                >(b"rmtError Buffer_WriteStringZ(Buffer *, rmtPStr)\0"))
                    .as_ptr(),
            );
        }
    };
    return Buffer_Write(
        buffer,
        string as *mut libc::c_void,
        (strnlen_s_safe_c(string, 2048 as libc::c_int as r_size_t))
            .wrapping_add(1 as libc::c_int as libc::c_uint),
    );
}
unsafe extern "C" fn U32ToByteArray(mut dest: *mut rmtU8, mut value: rmtU32) {
    *dest
        .offset(
            0 as libc::c_int as isize,
        ) = (value & 255 as libc::c_int as libc::c_uint) as rmtU8;
    *dest
        .offset(
            1 as libc::c_int as isize,
        ) = (value >> 8 as libc::c_int & 255 as libc::c_int as libc::c_uint) as rmtU8;
    *dest
        .offset(
            2 as libc::c_int as isize,
        ) = (value >> 16 as libc::c_int & 255 as libc::c_int as libc::c_uint) as rmtU8;
    *dest.offset(3 as libc::c_int as isize) = (value >> 24 as libc::c_int) as rmtU8;
}
unsafe extern "C" fn Buffer_WriteU32(
    mut buffer: *mut Buffer,
    mut value: rmtU32,
) -> rmtError {
    if !buffer.is_null() {} else {
        __assert_fail(
            b"buffer != NULL\0" as *const u8 as *const libc::c_char,
            b"lib/Remotery.c\0" as *const u8 as *const libc::c_char,
            2541 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 43],
                &[libc::c_char; 43],
            >(b"rmtError Buffer_WriteU32(Buffer *, rmtU32)\0"))
                .as_ptr(),
        );
    }
    'c_12438: {
        if !buffer.is_null() {} else {
            __assert_fail(
                b"buffer != NULL\0" as *const u8 as *const libc::c_char,
                b"lib/Remotery.c\0" as *const u8 as *const libc::c_char,
                2541 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 43],
                    &[libc::c_char; 43],
                >(b"rmtError Buffer_WriteU32(Buffer *, rmtU32)\0"))
                    .as_ptr(),
            );
        }
    };
    if ((*buffer).bytes_used as libc::c_ulong)
        .wrapping_add(::std::mem::size_of::<rmtU32>() as libc::c_ulong)
        > (*buffer).bytes_allocated as libc::c_ulong
    {
        let mut error: rmtError = Buffer_Grow(
            buffer,
            ::std::mem::size_of::<rmtU32>() as libc::c_ulong as rmtU32,
        );
        if error as libc::c_uint != RMT_ERROR_NONE as libc::c_int as libc::c_uint {
            return error;
        }
    }
    U32ToByteArray(((*buffer).data).offset((*buffer).bytes_used as isize), value);
    (*buffer)
        .bytes_used = ((*buffer).bytes_used as libc::c_ulong)
        .wrapping_add(::std::mem::size_of::<rmtU32>() as libc::c_ulong) as rmtU32
        as rmtU32;
    return RMT_ERROR_NONE;
}
unsafe extern "C" fn IsLittleEndian() -> rmtBool {
    let mut u: C2RustUnnamed_1 = C2RustUnnamed_1 { i: 0 };
    u.i = 1 as libc::c_int as libc::c_uint;
    return if u.c[0 as libc::c_int as usize] as libc::c_int == 1 as libc::c_int {
        1 as libc::c_int as rmtBool
    } else {
        0 as libc::c_int as rmtBool
    };
}
unsafe extern "C" fn Buffer_WriteF64(
    mut buffer: *mut Buffer,
    mut value: rmtF64,
) -> rmtError {
    if !buffer.is_null() {} else {
        __assert_fail(
            b"buffer != NULL\0" as *const u8 as *const libc::c_char,
            b"lib/Remotery.c\0" as *const u8 as *const libc::c_char,
            2575 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 43],
                &[libc::c_char; 43],
            >(b"rmtError Buffer_WriteF64(Buffer *, rmtF64)\0"))
                .as_ptr(),
        );
    }
    'c_12876: {
        if !buffer.is_null() {} else {
            __assert_fail(
                b"buffer != NULL\0" as *const u8 as *const libc::c_char,
                b"lib/Remotery.c\0" as *const u8 as *const libc::c_char,
                2575 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 43],
                    &[libc::c_char; 43],
                >(b"rmtError Buffer_WriteF64(Buffer *, rmtF64)\0"))
                    .as_ptr(),
            );
        }
    };
    if ((*buffer).bytes_used as libc::c_ulong)
        .wrapping_add(::std::mem::size_of::<rmtF64>() as libc::c_ulong)
        > (*buffer).bytes_allocated as libc::c_ulong
    {
        let mut error: rmtError = Buffer_Grow(
            buffer,
            ::std::mem::size_of::<rmtF64>() as libc::c_ulong as rmtU32,
        );
        if error as libc::c_uint != RMT_ERROR_NONE as libc::c_int as libc::c_uint {
            return error;
        }
    }
    let mut u: C2RustUnnamed_0 = C2RustUnnamed_0 { d: 0. };
    let mut dest: *mut rmtU8 = ((*buffer).data).offset((*buffer).bytes_used as isize);
    u.d = value;
    if IsLittleEndian() != 0 {
        *dest.offset(0 as libc::c_int as isize) = u.c[0 as libc::c_int as usize];
        *dest.offset(1 as libc::c_int as isize) = u.c[1 as libc::c_int as usize];
        *dest.offset(2 as libc::c_int as isize) = u.c[2 as libc::c_int as usize];
        *dest.offset(3 as libc::c_int as isize) = u.c[3 as libc::c_int as usize];
        *dest.offset(4 as libc::c_int as isize) = u.c[4 as libc::c_int as usize];
        *dest.offset(5 as libc::c_int as isize) = u.c[5 as libc::c_int as usize];
        *dest.offset(6 as libc::c_int as isize) = u.c[6 as libc::c_int as usize];
        *dest.offset(7 as libc::c_int as isize) = u.c[7 as libc::c_int as usize];
    } else {
        *dest.offset(0 as libc::c_int as isize) = u.c[7 as libc::c_int as usize];
        *dest.offset(1 as libc::c_int as isize) = u.c[6 as libc::c_int as usize];
        *dest.offset(2 as libc::c_int as isize) = u.c[5 as libc::c_int as usize];
        *dest.offset(3 as libc::c_int as isize) = u.c[4 as libc::c_int as usize];
        *dest.offset(4 as libc::c_int as isize) = u.c[3 as libc::c_int as usize];
        *dest.offset(5 as libc::c_int as isize) = u.c[2 as libc::c_int as usize];
        *dest.offset(6 as libc::c_int as isize) = u.c[1 as libc::c_int as usize];
        *dest.offset(7 as libc::c_int as isize) = u.c[0 as libc::c_int as usize];
    }
    (*buffer)
        .bytes_used = ((*buffer).bytes_used as libc::c_ulong)
        .wrapping_add(::std::mem::size_of::<rmtF64>() as libc::c_ulong) as rmtU32
        as rmtU32;
    return RMT_ERROR_NONE;
}
unsafe extern "C" fn Buffer_WriteU64(
    mut buffer: *mut Buffer,
    mut value: rmtU64,
) -> rmtError {
    return Buffer_WriteF64(buffer, value as libc::c_double);
}
unsafe extern "C" fn Buffer_WriteStringWithLength(
    mut buffer: *mut Buffer,
    mut string: rmtPStr,
) -> rmtError {
    let mut length: rmtU32 = strnlen_s_safe_c(string, 2048 as libc::c_int as r_size_t);
    let mut error: rmtError = Buffer_WriteU32(buffer, length);
    if error as libc::c_uint != RMT_ERROR_NONE as libc::c_int as libc::c_uint {
        return error;
    }
    return Buffer_Write(buffer, string as *mut libc::c_void, length);
}
unsafe extern "C" fn rmtHashTable_Constructor(
    mut table: *mut rmtHashTable,
    mut max_nb_slots: rmtU32,
) -> rmtError {
    if !table.is_null() {} else {
        __assert_fail(
            b"table != NULL\0" as *const u8 as *const libc::c_char,
            b"lib/Remotery.c\0" as *const u8 as *const libc::c_char,
            2669 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 58],
                &[libc::c_char; 58],
            >(b"rmtError rmtHashTable_Constructor(rmtHashTable *, rmtU32)\0"))
                .as_ptr(),
        );
    }
    'c_26807: {
        if !table.is_null() {} else {
            __assert_fail(
                b"table != NULL\0" as *const u8 as *const libc::c_char,
                b"lib/Remotery.c\0" as *const u8 as *const libc::c_char,
                2669 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 58],
                    &[libc::c_char; 58],
                >(b"rmtError rmtHashTable_Constructor(rmtHashTable *, rmtU32)\0"))
                    .as_ptr(),
            );
        }
    };
    (*table).maxNbSlots = max_nb_slots;
    (*table).nbSlots = 0 as libc::c_int as rmtU32;
    (*table)
        .slots = rmtMalloc(
        ((*table).maxNbSlots as libc::c_ulong)
            .wrapping_mul(::std::mem::size_of::<HashSlot>() as libc::c_ulong) as rmtU32,
    ) as *mut HashSlot;
    if ((*table).slots).is_null() {
        return RMT_ERROR_MALLOC_FAIL;
    }
    memset(
        (*table).slots as *mut libc::c_void,
        0 as libc::c_int,
        ((*table).maxNbSlots as libc::c_ulong)
            .wrapping_mul(::std::mem::size_of::<HashSlot>() as libc::c_ulong),
    );
    return RMT_ERROR_NONE;
}
unsafe extern "C" fn rmtHashTable_Destructor(mut table: *mut rmtHashTable) {
    if !table.is_null() {} else {
        __assert_fail(
            b"table != NULL\0" as *const u8 as *const libc::c_char,
            b"lib/Remotery.c\0" as *const u8 as *const libc::c_char,
            2682 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 45],
                &[libc::c_char; 45],
            >(b"void rmtHashTable_Destructor(rmtHashTable *)\0"))
                .as_ptr(),
        );
    }
    'c_26912: {
        if !table.is_null() {} else {
            __assert_fail(
                b"table != NULL\0" as *const u8 as *const libc::c_char,
                b"lib/Remotery.c\0" as *const u8 as *const libc::c_char,
                2682 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 45],
                    &[libc::c_char; 45],
                >(b"void rmtHashTable_Destructor(rmtHashTable *)\0"))
                    .as_ptr(),
            );
        }
    };
    if !((*table).slots).is_null() {
        rmtFree((*table).slots as *mut libc::c_void);
        (*table).slots = 0 as *mut HashSlot;
    }
}
unsafe extern "C" fn rmtHashTable_Insert(
    mut table: *mut rmtHashTable,
    mut key: rmtU32,
    mut value: rmtU64,
) -> rmtError {
    let mut slot: *mut HashSlot = 0 as *mut HashSlot;
    let mut error: rmtError = RMT_ERROR_NONE;
    let mut index_mask: rmtU32 = ((*table).maxNbSlots)
        .wrapping_sub(1 as libc::c_int as libc::c_uint);
    let mut index: rmtU32 = key & index_mask;
    if key != 0 as libc::c_int as libc::c_uint {} else {
        __assert_fail(
            b"key != 0\0" as *const u8 as *const libc::c_char,
            b"lib/Remotery.c\0" as *const u8 as *const libc::c_char,
            2702 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 61],
                &[libc::c_char; 61],
            >(b"rmtError rmtHashTable_Insert(rmtHashTable *, rmtU32, rmtU64)\0"))
                .as_ptr(),
        );
    }
    'c_16787: {
        if key != 0 as libc::c_int as libc::c_uint {} else {
            __assert_fail(
                b"key != 0\0" as *const u8 as *const libc::c_char,
                b"lib/Remotery.c\0" as *const u8 as *const libc::c_char,
                2702 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 61],
                    &[libc::c_char; 61],
                >(b"rmtError rmtHashTable_Insert(rmtHashTable *, rmtU32, rmtU64)\0"))
                    .as_ptr(),
            );
        }
    };
    if value != 0xffffffffffffffff as libc::c_ulong as libc::c_ulonglong {} else {
        __assert_fail(
            b"value != RMT_NOT_FOUND\0" as *const u8 as *const libc::c_char,
            b"lib/Remotery.c\0" as *const u8 as *const libc::c_char,
            2703 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 61],
                &[libc::c_char; 61],
            >(b"rmtError rmtHashTable_Insert(rmtHashTable *, rmtU32, rmtU64)\0"))
                .as_ptr(),
        );
    }
    'c_16749: {
        if value != 0xffffffffffffffff as libc::c_ulong as libc::c_ulonglong {} else {
            __assert_fail(
                b"value != RMT_NOT_FOUND\0" as *const u8 as *const libc::c_char,
                b"lib/Remotery.c\0" as *const u8 as *const libc::c_char,
                2703 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 61],
                    &[libc::c_char; 61],
                >(b"rmtError rmtHashTable_Insert(rmtHashTable *, rmtU32, rmtU64)\0"))
                    .as_ptr(),
            );
        }
    };
    while (*((*table).slots).offset(index as isize)).key != 0 {
        if (*((*table).slots).offset(index as isize)).key == key {
            (*table).nbSlots = ((*table).nbSlots).wrapping_sub(1);
            (*table).nbSlots;
            break;
        } else {
            index = index.wrapping_add(1 as libc::c_int as libc::c_uint) & index_mask;
        }
    }
    if index < (*table).maxNbSlots {} else {
        __assert_fail(
            b"index < table->maxNbSlots\0" as *const u8 as *const libc::c_char,
            b"lib/Remotery.c\0" as *const u8 as *const libc::c_char,
            2720 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 61],
                &[libc::c_char; 61],
            >(b"rmtError rmtHashTable_Insert(rmtHashTable *, rmtU32, rmtU64)\0"))
                .as_ptr(),
        );
    }
    'c_16664: {
        if index < (*table).maxNbSlots {} else {
            __assert_fail(
                b"index < table->maxNbSlots\0" as *const u8 as *const libc::c_char,
                b"lib/Remotery.c\0" as *const u8 as *const libc::c_char,
                2720 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 61],
                    &[libc::c_char; 61],
                >(b"rmtError rmtHashTable_Insert(rmtHashTable *, rmtU32, rmtU64)\0"))
                    .as_ptr(),
            );
        }
    };
    slot = ((*table).slots).offset(index as isize);
    (*slot).key = key;
    (*slot).value = value;
    (*table).nbSlots = ((*table).nbSlots).wrapping_add(1);
    (*table).nbSlots;
    if (*table).nbSlots
        > ((*table).maxNbSlots)
            .wrapping_mul(2 as libc::c_int as libc::c_uint)
            .wrapping_div(3 as libc::c_int as libc::c_uint)
    {
        error = rmtHashTable_Resize(table);
    }
    return error;
}
unsafe extern "C" fn rmtHashTable_Resize(mut table: *mut rmtHashTable) -> rmtError {
    let mut old_max_nb_slots: rmtU32 = (*table).maxNbSlots;
    let mut new_slots: *mut HashSlot = 0 as *mut HashSlot;
    let mut old_slots: *mut HashSlot = (*table).slots;
    let mut i: rmtU32 = 0;
    let mut new_max_nb_slots: rmtU32 = (*table).maxNbSlots;
    if new_max_nb_slots < (8192 as libc::c_int * 4 as libc::c_int) as libc::c_uint {
        new_max_nb_slots = (new_max_nb_slots as libc::c_uint)
            .wrapping_mul(4 as libc::c_int as libc::c_uint) as rmtU32 as rmtU32;
    } else {
        new_max_nb_slots = (new_max_nb_slots as libc::c_uint)
            .wrapping_mul(2 as libc::c_int as libc::c_uint) as rmtU32 as rmtU32;
    }
    new_slots = rmtMalloc(
        (new_max_nb_slots as libc::c_ulong)
            .wrapping_mul(::std::mem::size_of::<HashSlot>() as libc::c_ulong) as rmtU32,
    ) as *mut HashSlot;
    if new_slots.is_null() {
        return RMT_ERROR_MALLOC_FAIL;
    }
    memset(
        new_slots as *mut libc::c_void,
        0 as libc::c_int,
        (new_max_nb_slots as libc::c_ulong)
            .wrapping_mul(::std::mem::size_of::<HashSlot>() as libc::c_ulong),
    );
    (*table).slots = new_slots;
    (*table).maxNbSlots = new_max_nb_slots;
    (*table).nbSlots = 0 as libc::c_int as rmtU32;
    i = 0 as libc::c_int as rmtU32;
    while i < old_max_nb_slots {
        let mut slot: *mut HashSlot = old_slots.offset(i as isize);
        if (*slot).key != 0 as libc::c_int as libc::c_uint {
            rmtHashTable_Insert(table, (*slot).key, (*slot).value);
        }
        i = i.wrapping_add(1);
        i;
    }
    rmtFree(old_slots as *mut libc::c_void);
    return RMT_ERROR_NONE;
}
unsafe extern "C" fn rmtHashTable_Find(
    mut table: *mut rmtHashTable,
    mut key: rmtU32,
) -> rmtU64 {
    let mut index_mask: rmtU32 = ((*table).maxNbSlots)
        .wrapping_sub(1 as libc::c_int as libc::c_uint);
    let mut index: rmtU32 = key & index_mask;
    while (*((*table).slots).offset(index as isize)).key != 0 {
        let mut slot: *mut HashSlot = ((*table).slots).offset(index as isize);
        if (*slot).key == key {
            return (*slot).value;
        }
        index = index.wrapping_add(1 as libc::c_int as libc::c_uint) & index_mask;
    }
    return 0xffffffffffffffff as libc::c_ulong as rmtU64;
}
unsafe extern "C" fn StringTable_Constructor(mut table: *mut StringTable) -> rmtError {
    if !table.is_null() {} else {
        __assert_fail(
            b"table != NULL\0" as *const u8 as *const libc::c_char,
            b"lib/Remotery.c\0" as *const u8 as *const libc::c_char,
            2821 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 48],
                &[libc::c_char; 48],
            >(b"rmtError StringTable_Constructor(StringTable *)\0"))
                .as_ptr(),
        );
    }
    'c_27250: {
        if !table.is_null() {} else {
            __assert_fail(
                b"table != NULL\0" as *const u8 as *const libc::c_char,
                b"lib/Remotery.c\0" as *const u8 as *const libc::c_char,
                2821 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 48],
                    &[libc::c_char; 48],
                >(b"rmtError StringTable_Constructor(StringTable *)\0"))
                    .as_ptr(),
            );
        }
    };
    (*table).text = 0 as *mut Buffer;
    (*table).text_map = 0 as *mut rmtHashTable;
    (*table)
        .text = rmtMalloc(::std::mem::size_of::<Buffer>() as libc::c_ulong as rmtU32)
        as *mut Buffer;
    if ((*table).text).is_null() {
        return RMT_ERROR_MALLOC_FAIL;
    }
    let mut error: rmtError = Buffer_Constructor(
        (*table).text,
        (8 as libc::c_int * 1024 as libc::c_int) as rmtU32,
    );
    if error as libc::c_uint != RMT_ERROR_NONE as libc::c_int as libc::c_uint {
        if !((*table).text).is_null() {
            Buffer_Destructor((*table).text);
            rmtFree((*table).text as *mut libc::c_void);
            (*table).text = 0 as *mut Buffer;
        }
        return error;
    }
    (*table)
        .text_map = rmtMalloc(
        ::std::mem::size_of::<rmtHashTable>() as libc::c_ulong as rmtU32,
    ) as *mut rmtHashTable;
    if ((*table).text_map).is_null() {
        return RMT_ERROR_MALLOC_FAIL;
    }
    let mut error_0: rmtError = rmtHashTable_Constructor(
        (*table).text_map,
        (1 as libc::c_int * 1024 as libc::c_int) as rmtU32,
    );
    if error_0 as libc::c_uint != RMT_ERROR_NONE as libc::c_int as libc::c_uint {
        if !((*table).text_map).is_null() {
            rmtHashTable_Destructor((*table).text_map);
            rmtFree((*table).text_map as *mut libc::c_void);
            (*table).text_map = 0 as *mut rmtHashTable;
        }
        return error_0;
    }
    return RMT_ERROR_NONE;
}
unsafe extern "C" fn StringTable_Destructor(mut table: *mut StringTable) {
    if !table.is_null() {} else {
        __assert_fail(
            b"table != NULL\0" as *const u8 as *const libc::c_char,
            b"lib/Remotery.c\0" as *const u8 as *const libc::c_char,
            2834 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 43],
                &[libc::c_char; 43],
            >(b"void StringTable_Destructor(StringTable *)\0"))
                .as_ptr(),
        );
    }
    'c_27402: {
        if !table.is_null() {} else {
            __assert_fail(
                b"table != NULL\0" as *const u8 as *const libc::c_char,
                b"lib/Remotery.c\0" as *const u8 as *const libc::c_char,
                2834 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 43],
                    &[libc::c_char; 43],
                >(b"void StringTable_Destructor(StringTable *)\0"))
                    .as_ptr(),
            );
        }
    };
    if !((*table).text_map).is_null() {
        rmtHashTable_Destructor((*table).text_map);
        rmtFree((*table).text_map as *mut libc::c_void);
        (*table).text_map = 0 as *mut rmtHashTable;
    }
    if !((*table).text).is_null() {
        Buffer_Destructor((*table).text);
        rmtFree((*table).text as *mut libc::c_void);
        (*table).text = 0 as *mut Buffer;
    }
}
unsafe extern "C" fn StringTable_Find(
    mut table: *mut StringTable,
    mut name_hash: rmtU32,
) -> rmtPStr {
    let mut text_offset: rmtU64 = rmtHashTable_Find((*table).text_map, name_hash);
    if text_offset != 0xffffffffffffffff as libc::c_ulong as libc::c_ulonglong {
        return ((*(*table).text).data).offset(text_offset as isize) as rmtPStr;
    }
    return 0 as rmtPStr;
}
unsafe extern "C" fn StringTable_Insert(
    mut table: *mut StringTable,
    mut name_hash: rmtU32,
    mut name: rmtPStr,
) -> rmtBool {
    let mut text_offset: rmtU64 = rmtHashTable_Find((*table).text_map, name_hash);
    if text_offset == 0xffffffffffffffff as libc::c_ulong as libc::c_ulonglong {
        text_offset = (*(*table).text).bytes_used as rmtU64;
        Buffer_WriteStringZ((*table).text, name);
        rmtHashTable_Insert((*table).text_map, name_hash, text_offset);
        return 1 as libc::c_int as rmtBool;
    }
    return 0 as libc::c_int as rmtBool;
}
unsafe extern "C" fn InitialiseNetwork() -> rmtError {
    return RMT_ERROR_NONE;
}
unsafe extern "C" fn ShutdownNetwork() {}
unsafe extern "C" fn TCPSocket_Constructor(mut tcp_socket: *mut TCPSocket) -> rmtError {
    if !tcp_socket.is_null() {} else {
        __assert_fail(
            b"tcp_socket != NULL\0" as *const u8 as *const libc::c_char,
            b"lib/Remotery.c\0" as *const u8 as *const libc::c_char,
            2931 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 44],
                &[libc::c_char; 44],
            >(b"rmtError TCPSocket_Constructor(TCPSocket *)\0"))
                .as_ptr(),
        );
    }
    'c_18728: {
        if !tcp_socket.is_null() {} else {
            __assert_fail(
                b"tcp_socket != NULL\0" as *const u8 as *const libc::c_char,
                b"lib/Remotery.c\0" as *const u8 as *const libc::c_char,
                2931 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 44],
                    &[libc::c_char; 44],
                >(b"rmtError TCPSocket_Constructor(TCPSocket *)\0"))
                    .as_ptr(),
            );
        }
    };
    (*tcp_socket).socket = -(1 as libc::c_int);
    return InitialiseNetwork();
}
unsafe extern "C" fn TCPSocket_Destructor(mut tcp_socket: *mut TCPSocket) {
    if !tcp_socket.is_null() {} else {
        __assert_fail(
            b"tcp_socket != NULL\0" as *const u8 as *const libc::c_char,
            b"lib/Remotery.c\0" as *const u8 as *const libc::c_char,
            2938 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 39],
                &[libc::c_char; 39],
            >(b"void TCPSocket_Destructor(TCPSocket *)\0"))
                .as_ptr(),
        );
    }
    'c_10657: {
        if !tcp_socket.is_null() {} else {
            __assert_fail(
                b"tcp_socket != NULL\0" as *const u8 as *const libc::c_char,
                b"lib/Remotery.c\0" as *const u8 as *const libc::c_char,
                2938 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 39],
                    &[libc::c_char; 39],
                >(b"void TCPSocket_Destructor(TCPSocket *)\0"))
                    .as_ptr(),
            );
        }
    };
    TCPSocket_Close(tcp_socket);
    ShutdownNetwork();
}
unsafe extern "C" fn TCPSocket_RunServer(
    mut tcp_socket: *mut TCPSocket,
    mut port: rmtU16,
    mut reuse_open_port: rmtBool,
    mut limit_connections_to_localhost: rmtBool,
) -> rmtError {
    let mut s: SOCKET = -(1 as libc::c_int);
    let mut sin: sockaddr_in = sockaddr_in {
        sin_family: 0,
        sin_port: 0,
        sin_addr: in_addr { s_addr: 0 },
        sin_zero: [0; 8],
    };
    memset(
        &mut sin as *mut sockaddr_in as *mut libc::c_void,
        0 as libc::c_int,
        ::std::mem::size_of::<sockaddr_in>() as libc::c_ulong,
    );
    if !tcp_socket.is_null() {} else {
        __assert_fail(
            b"tcp_socket != NULL\0" as *const u8 as *const libc::c_char,
            b"lib/Remotery.c\0" as *const u8 as *const libc::c_char,
            2953 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 68],
                &[libc::c_char; 68],
            >(b"rmtError TCPSocket_RunServer(TCPSocket *, rmtU16, rmtBool, rmtBool)\0"))
                .as_ptr(),
        );
    }
    'c_22126: {
        if !tcp_socket.is_null() {} else {
            __assert_fail(
                b"tcp_socket != NULL\0" as *const u8 as *const libc::c_char,
                b"lib/Remotery.c\0" as *const u8 as *const libc::c_char,
                2953 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 68],
                    &[libc::c_char; 68],
                >(
                    b"rmtError TCPSocket_RunServer(TCPSocket *, rmtU16, rmtBool, rmtBool)\0",
                ))
                    .as_ptr(),
            );
        }
    };
    s = socket(2 as libc::c_int, SOCK_STREAM as libc::c_int, IPPROTO_TCP as libc::c_int);
    if s == -(1 as libc::c_int) {
        return rmtMakeError(
            RMT_ERROR_RESOURCE_CREATE_FAIL,
            b"Can't create a socket for connection to the remote viewer\0" as *const u8
                as *const libc::c_char,
        );
    }
    if reuse_open_port != 0 {
        let mut enable: libc::c_int = 1 as libc::c_int;
        setsockopt(
            s,
            1 as libc::c_int,
            2 as libc::c_int,
            &mut enable as *mut libc::c_int as *const libc::c_void,
            ::std::mem::size_of::<libc::c_int>() as libc::c_ulong as socklen_t,
        );
    }
    sin.sin_family = 2 as libc::c_int as sa_family_t;
    sin
        .sin_addr
        .s_addr = htonl(
        if limit_connections_to_localhost != 0 {
            0x7f000001 as libc::c_int as in_addr_t
        } else {
            0 as libc::c_int as in_addr_t
        },
    );
    sin.sin_port = htons(port);
    if bind(
        s,
        &mut sin as *mut sockaddr_in as *mut sockaddr,
        ::std::mem::size_of::<sockaddr_in>() as libc::c_ulong as socklen_t,
    ) == -(1 as libc::c_int)
    {
        return rmtMakeError(
            RMT_ERROR_RESOURCE_ACCESS_FAIL,
            b"Can't bind a socket for the server\0" as *const u8 as *const libc::c_char,
        );
    }
    (*tcp_socket).socket = s;
    if listen(s, 1 as libc::c_int) == -(1 as libc::c_int) {
        return rmtMakeError(
            RMT_ERROR_RESOURCE_ACCESS_FAIL,
            b"Created server socket failed to enter a listen state\0" as *const u8
                as *const libc::c_char,
        );
    }
    if fcntl((*tcp_socket).socket, 4 as libc::c_int, 0o4000 as libc::c_int)
        == -(1 as libc::c_int)
    {
        return rmtMakeError(
            RMT_ERROR_RESOURCE_ACCESS_FAIL,
            b"Created server socket failed to switch to a non-blocking state\0"
                as *const u8 as *const libc::c_char,
        );
    }
    return RMT_ERROR_NONE;
}
unsafe extern "C" fn TCPSocket_Close(mut tcp_socket: *mut TCPSocket) {
    if !tcp_socket.is_null() {} else {
        __assert_fail(
            b"tcp_socket != NULL\0" as *const u8 as *const libc::c_char,
            b"lib/Remotery.c\0" as *const u8 as *const libc::c_char,
            3017 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 34],
                &[libc::c_char; 34],
            >(b"void TCPSocket_Close(TCPSocket *)\0"))
                .as_ptr(),
        );
    }
    'c_10614: {
        if !tcp_socket.is_null() {} else {
            __assert_fail(
                b"tcp_socket != NULL\0" as *const u8 as *const libc::c_char,
                b"lib/Remotery.c\0" as *const u8 as *const libc::c_char,
                3017 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 34],
                    &[libc::c_char; 34],
                >(b"void TCPSocket_Close(TCPSocket *)\0"))
                    .as_ptr(),
            );
        }
    };
    if (*tcp_socket).socket != -(1 as libc::c_int) {
        let mut result: libc::c_int = shutdown(
            (*tcp_socket).socket,
            SHUT_WR as libc::c_int,
        );
        if result != -(1 as libc::c_int) {
            let mut total: libc::c_int = 0 as libc::c_int;
            let mut temp_buf: [libc::c_char; 128] = [0; 128];
            while result > 0 as libc::c_int {
                result = recv(
                    (*tcp_socket).socket,
                    temp_buf.as_mut_ptr() as *mut libc::c_void,
                    ::std::mem::size_of::<[libc::c_char; 128]>() as libc::c_ulong,
                    0 as libc::c_int,
                ) as libc::c_int;
                total += result;
            }
        }
        close((*tcp_socket).socket);
        (*tcp_socket).socket = -(1 as libc::c_int);
    }
}
unsafe extern "C" fn TCPSocket_PollStatus(
    mut tcp_socket: *mut TCPSocket,
) -> SocketStatus {
    let mut status: SocketStatus = SocketStatus {
        can_read: 0,
        can_write: 0,
        error_state: RMT_ERROR_NONE,
    };
    let mut fd_read: fd_set = fd_set { __fds_bits: [0; 16] };
    let mut fd_write: fd_set = fd_set { __fds_bits: [0; 16] };
    let mut fd_errors: fd_set = fd_set { __fds_bits: [0; 16] };
    let mut tv: timeval = timeval { tv_sec: 0, tv_usec: 0 };
    status.can_read = 0 as libc::c_int as rmtBool;
    status.can_write = 0 as libc::c_int as rmtBool;
    status.error_state = RMT_ERROR_NONE;
    if !tcp_socket.is_null() {} else {
        __assert_fail(
            b"tcp_socket != NULL\0" as *const u8 as *const libc::c_char,
            b"lib/Remotery.c\0" as *const u8 as *const libc::c_char,
            3051 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 47],
                &[libc::c_char; 47],
            >(b"SocketStatus TCPSocket_PollStatus(TCPSocket *)\0"))
                .as_ptr(),
        );
    }
    'c_9885: {
        if !tcp_socket.is_null() {} else {
            __assert_fail(
                b"tcp_socket != NULL\0" as *const u8 as *const libc::c_char,
                b"lib/Remotery.c\0" as *const u8 as *const libc::c_char,
                3051 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 47],
                    &[libc::c_char; 47],
                >(b"SocketStatus TCPSocket_PollStatus(TCPSocket *)\0"))
                    .as_ptr(),
            );
        }
    };
    if (*tcp_socket).socket == -(1 as libc::c_int) {
        status.error_state = RMT_ERROR_SOCKET_INVALID_POLL;
        return status;
    }
    let mut __d0: libc::c_int = 0;
    let mut __d1: libc::c_int = 0;
    let fresh2 = &mut __d0;
    let fresh3;
    let fresh4 = (::std::mem::size_of::<fd_set>() as libc::c_ulong)
        .wrapping_div(::std::mem::size_of::<__fd_mask>() as libc::c_ulong);
    let fresh5 = &mut __d1;
    let fresh6;
    let fresh7 = &mut *(fd_read.__fds_bits)
        .as_mut_ptr()
        .offset(0 as libc::c_int as isize) as *mut __fd_mask;
    asm!(
        "cld; rep; stosq", inlateout("cx") c2rust_asm_casts::AsmCast::cast_in(fresh2,
        fresh4) => fresh3, inlateout("di") c2rust_asm_casts::AsmCast::cast_in(fresh5,
        fresh7) => fresh6, inlateout("ax") 0 as libc::c_int => _,
        options(preserves_flags, att_syntax)
    );
    c2rust_asm_casts::AsmCast::cast_out(fresh2, fresh4, fresh3);
    c2rust_asm_casts::AsmCast::cast_out(fresh5, fresh7, fresh6);
    let mut __d0_0: libc::c_int = 0;
    let mut __d1_0: libc::c_int = 0;
    let fresh8 = &mut __d0_0;
    let fresh9;
    let fresh10 = (::std::mem::size_of::<fd_set>() as libc::c_ulong)
        .wrapping_div(::std::mem::size_of::<__fd_mask>() as libc::c_ulong);
    let fresh11 = &mut __d1_0;
    let fresh12;
    let fresh13 = &mut *(fd_write.__fds_bits)
        .as_mut_ptr()
        .offset(0 as libc::c_int as isize) as *mut __fd_mask;
    asm!(
        "cld; rep; stosq", inlateout("cx") c2rust_asm_casts::AsmCast::cast_in(fresh8,
        fresh10) => fresh9, inlateout("di") c2rust_asm_casts::AsmCast::cast_in(fresh11,
        fresh13) => fresh12, inlateout("ax") 0 as libc::c_int => _,
        options(preserves_flags, att_syntax)
    );
    c2rust_asm_casts::AsmCast::cast_out(fresh8, fresh10, fresh9);
    c2rust_asm_casts::AsmCast::cast_out(fresh11, fresh13, fresh12);
    let mut __d0_1: libc::c_int = 0;
    let mut __d1_1: libc::c_int = 0;
    let fresh14 = &mut __d0_1;
    let fresh15;
    let fresh16 = (::std::mem::size_of::<fd_set>() as libc::c_ulong)
        .wrapping_div(::std::mem::size_of::<__fd_mask>() as libc::c_ulong);
    let fresh17 = &mut __d1_1;
    let fresh18;
    let fresh19 = &mut *(fd_errors.__fds_bits)
        .as_mut_ptr()
        .offset(0 as libc::c_int as isize) as *mut __fd_mask;
    asm!(
        "cld; rep; stosq", inlateout("cx") c2rust_asm_casts::AsmCast::cast_in(fresh14,
        fresh16) => fresh15, inlateout("di") c2rust_asm_casts::AsmCast::cast_in(fresh17,
        fresh19) => fresh18, inlateout("ax") 0 as libc::c_int => _,
        options(preserves_flags, att_syntax)
    );
    c2rust_asm_casts::AsmCast::cast_out(fresh14, fresh16, fresh15);
    c2rust_asm_casts::AsmCast::cast_out(fresh17, fresh19, fresh18);
    fd_read
        .__fds_bits[((*tcp_socket).socket
        / (8 as libc::c_int
            * ::std::mem::size_of::<__fd_mask>() as libc::c_ulong as libc::c_int))
        as usize]
        |= ((1 as libc::c_ulong)
            << (*tcp_socket).socket
                % (8 as libc::c_int
                    * ::std::mem::size_of::<__fd_mask>() as libc::c_ulong
                        as libc::c_int)) as __fd_mask;
    fd_write
        .__fds_bits[((*tcp_socket).socket
        / (8 as libc::c_int
            * ::std::mem::size_of::<__fd_mask>() as libc::c_ulong as libc::c_int))
        as usize]
        |= ((1 as libc::c_ulong)
            << (*tcp_socket).socket
                % (8 as libc::c_int
                    * ::std::mem::size_of::<__fd_mask>() as libc::c_ulong
                        as libc::c_int)) as __fd_mask;
    fd_errors
        .__fds_bits[((*tcp_socket).socket
        / (8 as libc::c_int
            * ::std::mem::size_of::<__fd_mask>() as libc::c_ulong as libc::c_int))
        as usize]
        |= ((1 as libc::c_ulong)
            << (*tcp_socket).socket
                % (8 as libc::c_int
                    * ::std::mem::size_of::<__fd_mask>() as libc::c_ulong
                        as libc::c_int)) as __fd_mask;
    tv.tv_sec = 0 as libc::c_int as __time_t;
    tv.tv_usec = 0 as libc::c_int as __suseconds_t;
    if select(
        (*tcp_socket).socket + 1 as libc::c_int,
        &mut fd_read,
        &mut fd_write,
        &mut fd_errors,
        &mut tv,
    ) == -(1 as libc::c_int)
    {
        status.error_state = RMT_ERROR_SOCKET_SELECT_FAIL;
        return status;
    }
    status
        .can_read = if (fd_read
        .__fds_bits[((*tcp_socket).socket
        / (8 as libc::c_int
            * ::std::mem::size_of::<__fd_mask>() as libc::c_ulong as libc::c_int))
        as usize]
        & ((1 as libc::c_ulong)
            << (*tcp_socket).socket
                % (8 as libc::c_int
                    * ::std::mem::size_of::<__fd_mask>() as libc::c_ulong
                        as libc::c_int)) as __fd_mask
        != 0 as libc::c_int as libc::c_long) as libc::c_int != 0 as libc::c_int
    {
        1 as libc::c_int as rmtBool
    } else {
        0 as libc::c_int as rmtBool
    };
    status
        .can_write = if (fd_write
        .__fds_bits[((*tcp_socket).socket
        / (8 as libc::c_int
            * ::std::mem::size_of::<__fd_mask>() as libc::c_ulong as libc::c_int))
        as usize]
        & ((1 as libc::c_ulong)
            << (*tcp_socket).socket
                % (8 as libc::c_int
                    * ::std::mem::size_of::<__fd_mask>() as libc::c_ulong
                        as libc::c_int)) as __fd_mask
        != 0 as libc::c_int as libc::c_long) as libc::c_int != 0 as libc::c_int
    {
        1 as libc::c_int as rmtBool
    } else {
        0 as libc::c_int as rmtBool
    };
    status
        .error_state = (if (fd_errors
        .__fds_bits[((*tcp_socket).socket
        / (8 as libc::c_int
            * ::std::mem::size_of::<__fd_mask>() as libc::c_ulong as libc::c_int))
        as usize]
        & ((1 as libc::c_ulong)
            << (*tcp_socket).socket
                % (8 as libc::c_int
                    * ::std::mem::size_of::<__fd_mask>() as libc::c_ulong
                        as libc::c_int)) as __fd_mask
        != 0 as libc::c_int as libc::c_long) as libc::c_int != 0 as libc::c_int
    {
        RMT_ERROR_SOCKET_POLL_ERRORS as libc::c_int
    } else {
        RMT_ERROR_NONE as libc::c_int
    }) as rmtError;
    return status;
}
unsafe extern "C" fn TCPSocket_AcceptConnection(
    mut tcp_socket: *mut TCPSocket,
    mut client_socket: *mut *mut TCPSocket,
) -> rmtError {
    let mut status: SocketStatus = SocketStatus {
        can_read: 0,
        can_write: 0,
        error_state: RMT_ERROR_NONE,
    };
    let mut s: SOCKET = 0;
    if !tcp_socket.is_null() {} else {
        __assert_fail(
            b"tcp_socket != NULL\0" as *const u8 as *const libc::c_char,
            b"lib/Remotery.c\0" as *const u8 as *const libc::c_char,
            3094 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 63],
                &[libc::c_char; 63],
            >(b"rmtError TCPSocket_AcceptConnection(TCPSocket *, TCPSocket **)\0"))
                .as_ptr(),
        );
    }
    'c_21692: {
        if !tcp_socket.is_null() {} else {
            __assert_fail(
                b"tcp_socket != NULL\0" as *const u8 as *const libc::c_char,
                b"lib/Remotery.c\0" as *const u8 as *const libc::c_char,
                3094 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 63],
                    &[libc::c_char; 63],
                >(b"rmtError TCPSocket_AcceptConnection(TCPSocket *, TCPSocket **)\0"))
                    .as_ptr(),
            );
        }
    };
    status = TCPSocket_PollStatus(tcp_socket);
    if status.error_state as libc::c_uint
        != RMT_ERROR_NONE as libc::c_int as libc::c_uint || status.can_read == 0
    {
        return status.error_state;
    }
    s = accept((*tcp_socket).socket, 0 as *mut sockaddr, 0 as *mut socklen_t);
    if s == -(1 as libc::c_int) {
        return rmtMakeError(
            RMT_ERROR_RESOURCE_CREATE_FAIL,
            b"Server failed to accept connection from client\0" as *const u8
                as *const libc::c_char,
        );
    }
    if !client_socket.is_null() {} else {
        __assert_fail(
            b"client_socket != NULL\0" as *const u8 as *const libc::c_char,
            b"lib/Remotery.c\0" as *const u8 as *const libc::c_char,
            3119 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 63],
                &[libc::c_char; 63],
            >(b"rmtError TCPSocket_AcceptConnection(TCPSocket *, TCPSocket **)\0"))
                .as_ptr(),
        );
    }
    'c_21579: {
        if !client_socket.is_null() {} else {
            __assert_fail(
                b"client_socket != NULL\0" as *const u8 as *const libc::c_char,
                b"lib/Remotery.c\0" as *const u8 as *const libc::c_char,
                3119 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 63],
                    &[libc::c_char; 63],
                >(b"rmtError TCPSocket_AcceptConnection(TCPSocket *, TCPSocket **)\0"))
                    .as_ptr(),
            );
        }
    };
    *client_socket = rmtMalloc(
        ::std::mem::size_of::<TCPSocket>() as libc::c_ulong as rmtU32,
    ) as *mut TCPSocket;
    if (*client_socket).is_null() {
        return RMT_ERROR_MALLOC_FAIL;
    }
    let mut error: rmtError = TCPSocket_Constructor(*client_socket);
    if error as libc::c_uint != RMT_ERROR_NONE as libc::c_int as libc::c_uint {
        if !(*client_socket).is_null() {
            TCPSocket_Destructor(*client_socket);
            rmtFree(*client_socket as *mut libc::c_void);
            *client_socket = 0 as *mut TCPSocket;
        }
        return error;
    }
    (**client_socket).socket = s;
    return RMT_ERROR_NONE;
}
unsafe extern "C" fn TCPTryAgain() -> libc::c_int {
    return (*__errno_location() == 11 as libc::c_int) as libc::c_int;
}
unsafe extern "C" fn TCPSocket_Send(
    mut tcp_socket: *mut TCPSocket,
    mut data: *const libc::c_void,
    mut length: rmtU32,
    mut timeout_ms: rmtU32,
) -> rmtError {
    let mut status: SocketStatus = SocketStatus {
        can_read: 0,
        can_write: 0,
        error_state: RMT_ERROR_NONE,
    };
    let mut cur_data: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut end_data: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut start_ms: rmtU32 = 0 as libc::c_int as rmtU32;
    let mut cur_ms: rmtU32 = 0 as libc::c_int as rmtU32;
    if !tcp_socket.is_null() {} else {
        __assert_fail(
            b"tcp_socket != NULL\0" as *const u8 as *const libc::c_char,
            b"lib/Remotery.c\0" as *const u8 as *const libc::c_char,
            3148 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 67],
                &[libc::c_char; 67],
            >(b"rmtError TCPSocket_Send(TCPSocket *, const void *, rmtU32, rmtU32)\0"))
                .as_ptr(),
        );
    }
    'c_9958: {
        if !tcp_socket.is_null() {} else {
            __assert_fail(
                b"tcp_socket != NULL\0" as *const u8 as *const libc::c_char,
                b"lib/Remotery.c\0" as *const u8 as *const libc::c_char,
                3148 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 67],
                    &[libc::c_char; 67],
                >(
                    b"rmtError TCPSocket_Send(TCPSocket *, const void *, rmtU32, rmtU32)\0",
                ))
                    .as_ptr(),
            );
        }
    };
    start_ms = msTimer_Get();
    status.can_write = 0 as libc::c_int as rmtBool;
    while status.can_write == 0 {
        status = TCPSocket_PollStatus(tcp_socket);
        if status.error_state as libc::c_uint
            != RMT_ERROR_NONE as libc::c_int as libc::c_uint
        {
            return status.error_state;
        }
        cur_ms = msTimer_Get();
        if cur_ms.wrapping_sub(start_ms) > timeout_ms {
            return rmtMakeError(
                RMT_ERROR_TIMEOUT,
                b"Timed out trying to send data\0" as *const u8 as *const libc::c_char,
            );
        }
    }
    cur_data = data as *mut libc::c_char;
    end_data = cur_data.offset(length as isize);
    while cur_data < end_data {
        let mut bytes_sent: libc::c_int = 0;
        let mut send_flags: libc::c_int = 0 as libc::c_int;
        send_flags = MSG_NOSIGNAL as libc::c_int;
        bytes_sent = send(
            (*tcp_socket).socket,
            cur_data as *const libc::c_void,
            end_data.offset_from(cur_data) as libc::c_long as libc::c_int as size_t,
            send_flags,
        ) as libc::c_int;
        if bytes_sent == -(1 as libc::c_int) || bytes_sent == 0 as libc::c_int {
            if bytes_sent != 0 as libc::c_int && TCPTryAgain() == 0 {
                return RMT_ERROR_SOCKET_SEND_FAIL;
            }
            cur_ms = msTimer_Get();
            if cur_ms < start_ms {
                start_ms = cur_ms;
            } else if cur_ms.wrapping_sub(start_ms) > timeout_ms {
                return rmtMakeError(
                    RMT_ERROR_TIMEOUT,
                    b"Timed out trying to send data\0" as *const u8
                        as *const libc::c_char,
                )
            }
        } else {
            cur_data = cur_data.offset(bytes_sent as isize);
        }
    }
    return RMT_ERROR_NONE;
}
unsafe extern "C" fn TCPSocket_Receive(
    mut tcp_socket: *mut TCPSocket,
    mut data: *mut libc::c_void,
    mut length: rmtU32,
    mut timeout_ms: rmtU32,
) -> rmtError {
    let mut status: SocketStatus = SocketStatus {
        can_read: 0,
        can_write: 0,
        error_state: RMT_ERROR_NONE,
    };
    let mut cur_data: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut end_data: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut start_ms: rmtU32 = 0 as libc::c_int as rmtU32;
    let mut cur_ms: rmtU32 = 0 as libc::c_int as rmtU32;
    if !tcp_socket.is_null() {} else {
        __assert_fail(
            b"tcp_socket != NULL\0" as *const u8 as *const libc::c_char,
            b"lib/Remotery.c\0" as *const u8 as *const libc::c_char,
            3230 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 64],
                &[libc::c_char; 64],
            >(b"rmtError TCPSocket_Receive(TCPSocket *, void *, rmtU32, rmtU32)\0"))
                .as_ptr(),
        );
    }
    'c_17507: {
        if !tcp_socket.is_null() {} else {
            __assert_fail(
                b"tcp_socket != NULL\0" as *const u8 as *const libc::c_char,
                b"lib/Remotery.c\0" as *const u8 as *const libc::c_char,
                3230 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 64],
                    &[libc::c_char; 64],
                >(b"rmtError TCPSocket_Receive(TCPSocket *, void *, rmtU32, rmtU32)\0"))
                    .as_ptr(),
            );
        }
    };
    status = TCPSocket_PollStatus(tcp_socket);
    if status.error_state as libc::c_uint
        != RMT_ERROR_NONE as libc::c_int as libc::c_uint
    {
        return status.error_state;
    }
    if status.can_read == 0 {
        return RMT_ERROR_SOCKET_RECV_NO_DATA;
    }
    cur_data = data as *mut libc::c_char;
    end_data = cur_data.offset(length as isize);
    start_ms = msTimer_Get();
    while cur_data < end_data {
        let mut bytes_received: libc::c_int = recv(
            (*tcp_socket).socket,
            cur_data as *mut libc::c_void,
            end_data.offset_from(cur_data) as libc::c_long as libc::c_int as size_t,
            0 as libc::c_int,
        ) as libc::c_int;
        if bytes_received == -(1 as libc::c_int) || bytes_received == 0 as libc::c_int {
            if bytes_received != 0 as libc::c_int && TCPTryAgain() == 0 {
                return RMT_ERROR_SOCKET_RECV_FAILED;
            }
            cur_ms = msTimer_Get();
            if cur_ms < start_ms {
                start_ms = cur_ms;
            } else if cur_ms.wrapping_sub(start_ms) > timeout_ms {
                return RMT_ERROR_SOCKET_RECV_TIMEOUT
            }
        } else {
            cur_data = cur_data.offset(bytes_received as isize);
        }
    }
    return RMT_ERROR_NONE;
}
unsafe extern "C" fn rol(value: libc::c_uint, steps: libc::c_uint) -> libc::c_uint {
    return value << steps
        | value >> (32 as libc::c_int as libc::c_uint).wrapping_sub(steps);
}
unsafe extern "C" fn clearWBuffert(mut buffert: *mut libc::c_uint) {
    let mut pos: libc::c_int = 0;
    pos = 16 as libc::c_int;
    loop {
        pos -= 1;
        if !(pos >= 0 as libc::c_int) {
            break;
        }
        *buffert.offset(pos as isize) = 0 as libc::c_int as libc::c_uint;
    };
}
unsafe extern "C" fn innerHash(mut result: *mut libc::c_uint, mut w: *mut libc::c_uint) {
    let mut a: libc::c_uint = *result.offset(0 as libc::c_int as isize);
    let mut b: libc::c_uint = *result.offset(1 as libc::c_int as isize);
    let mut c: libc::c_uint = *result.offset(2 as libc::c_int as isize);
    let mut d: libc::c_uint = *result.offset(3 as libc::c_int as isize);
    let mut e: libc::c_uint = *result.offset(4 as libc::c_int as isize);
    let mut round: libc::c_int = 0 as libc::c_int;
    while round < 16 as libc::c_int {
        let t: libc::c_uint = (rol(a, 5 as libc::c_int as libc::c_uint))
            .wrapping_add(b & c | !b & d)
            .wrapping_add(e)
            .wrapping_add(0x5a827999 as libc::c_int as libc::c_uint)
            .wrapping_add(*w.offset(round as isize));
        e = d;
        d = c;
        c = rol(b, 30 as libc::c_int as libc::c_uint);
        b = a;
        a = t;
        round += 1;
        round;
    }
    while round < 20 as libc::c_int {
        *w
            .offset(
                round as isize,
            ) = rol(
            *w.offset((round - 3 as libc::c_int) as isize)
                ^ *w.offset((round - 8 as libc::c_int) as isize)
                ^ *w.offset((round - 14 as libc::c_int) as isize)
                ^ *w.offset((round - 16 as libc::c_int) as isize),
            1 as libc::c_int as libc::c_uint,
        );
        let t_0: libc::c_uint = (rol(a, 5 as libc::c_int as libc::c_uint))
            .wrapping_add(b & c | !b & d)
            .wrapping_add(e)
            .wrapping_add(0x5a827999 as libc::c_int as libc::c_uint)
            .wrapping_add(*w.offset(round as isize));
        e = d;
        d = c;
        c = rol(b, 30 as libc::c_int as libc::c_uint);
        b = a;
        a = t_0;
        round += 1;
        round;
    }
    while round < 40 as libc::c_int {
        *w
            .offset(
                round as isize,
            ) = rol(
            *w.offset((round - 3 as libc::c_int) as isize)
                ^ *w.offset((round - 8 as libc::c_int) as isize)
                ^ *w.offset((round - 14 as libc::c_int) as isize)
                ^ *w.offset((round - 16 as libc::c_int) as isize),
            1 as libc::c_int as libc::c_uint,
        );
        let t_1: libc::c_uint = (rol(a, 5 as libc::c_int as libc::c_uint))
            .wrapping_add(b ^ c ^ d)
            .wrapping_add(e)
            .wrapping_add(0x6ed9eba1 as libc::c_int as libc::c_uint)
            .wrapping_add(*w.offset(round as isize));
        e = d;
        d = c;
        c = rol(b, 30 as libc::c_int as libc::c_uint);
        b = a;
        a = t_1;
        round += 1;
        round;
    }
    while round < 60 as libc::c_int {
        *w
            .offset(
                round as isize,
            ) = rol(
            *w.offset((round - 3 as libc::c_int) as isize)
                ^ *w.offset((round - 8 as libc::c_int) as isize)
                ^ *w.offset((round - 14 as libc::c_int) as isize)
                ^ *w.offset((round - 16 as libc::c_int) as isize),
            1 as libc::c_int as libc::c_uint,
        );
        let t_2: libc::c_uint = (rol(a, 5 as libc::c_int as libc::c_uint))
            .wrapping_add(b & c | b & d | c & d)
            .wrapping_add(e)
            .wrapping_add(0x8f1bbcdc as libc::c_uint)
            .wrapping_add(*w.offset(round as isize));
        e = d;
        d = c;
        c = rol(b, 30 as libc::c_int as libc::c_uint);
        b = a;
        a = t_2;
        round += 1;
        round;
    }
    while round < 80 as libc::c_int {
        *w
            .offset(
                round as isize,
            ) = rol(
            *w.offset((round - 3 as libc::c_int) as isize)
                ^ *w.offset((round - 8 as libc::c_int) as isize)
                ^ *w.offset((round - 14 as libc::c_int) as isize)
                ^ *w.offset((round - 16 as libc::c_int) as isize),
            1 as libc::c_int as libc::c_uint,
        );
        let t_3: libc::c_uint = (rol(a, 5 as libc::c_int as libc::c_uint))
            .wrapping_add(b ^ c ^ d)
            .wrapping_add(e)
            .wrapping_add(0xca62c1d6 as libc::c_uint)
            .wrapping_add(*w.offset(round as isize));
        e = d;
        d = c;
        c = rol(b, 30 as libc::c_int as libc::c_uint);
        b = a;
        a = t_3;
        round += 1;
        round;
    }
    let ref mut fresh20 = *result.offset(0 as libc::c_int as isize);
    *fresh20 = (*fresh20).wrapping_add(a);
    let ref mut fresh21 = *result.offset(1 as libc::c_int as isize);
    *fresh21 = (*fresh21).wrapping_add(b);
    let ref mut fresh22 = *result.offset(2 as libc::c_int as isize);
    *fresh22 = (*fresh22).wrapping_add(c);
    let ref mut fresh23 = *result.offset(3 as libc::c_int as isize);
    *fresh23 = (*fresh23).wrapping_add(d);
    let ref mut fresh24 = *result.offset(4 as libc::c_int as isize);
    *fresh24 = (*fresh24).wrapping_add(e);
}
unsafe extern "C" fn calc(
    mut src: *const libc::c_void,
    bytelength: libc::c_int,
    mut hash: *mut libc::c_uchar,
) {
    let mut roundPos: libc::c_int = 0;
    let mut lastBlockBytes: libc::c_int = 0;
    let mut hashByte: libc::c_int = 0;
    let mut result: [libc::c_uint; 5] = [
        0x67452301 as libc::c_int as libc::c_uint,
        0xefcdab89 as libc::c_uint,
        0x98badcfe as libc::c_uint,
        0x10325476 as libc::c_int as libc::c_uint,
        0xc3d2e1f0 as libc::c_uint,
    ];
    let mut sarray: *const libc::c_uchar = src as *const libc::c_uchar;
    let mut w: [libc::c_uint; 80] = [0; 80];
    let endOfFullBlocks: libc::c_int = bytelength - 64 as libc::c_int;
    let mut endCurrentBlock: libc::c_int = 0;
    let mut currentBlock: libc::c_int = 0 as libc::c_int;
    while currentBlock <= endOfFullBlocks {
        endCurrentBlock = currentBlock + 64 as libc::c_int;
        roundPos = 0 as libc::c_int;
        while currentBlock < endCurrentBlock {
            let fresh25 = roundPos;
            roundPos = roundPos + 1;
            w[fresh25
                as usize] = *sarray.offset((currentBlock + 3 as libc::c_int) as isize)
                as libc::c_uint
                | (*sarray.offset((currentBlock + 2 as libc::c_int) as isize)
                    as libc::c_uint) << 8 as libc::c_int
                | (*sarray.offset((currentBlock + 1 as libc::c_int) as isize)
                    as libc::c_uint) << 16 as libc::c_int
                | (*sarray.offset(currentBlock as isize) as libc::c_uint)
                    << 24 as libc::c_int;
            currentBlock += 4 as libc::c_int;
        }
        innerHash(result.as_mut_ptr(), w.as_mut_ptr());
    }
    endCurrentBlock = bytelength - currentBlock;
    clearWBuffert(w.as_mut_ptr());
    lastBlockBytes = 0 as libc::c_int;
    while lastBlockBytes < endCurrentBlock {
        w[(lastBlockBytes >> 2 as libc::c_int) as usize]
            |= (*sarray.offset((lastBlockBytes + currentBlock) as isize) as libc::c_uint)
                << (3 as libc::c_int - (lastBlockBytes & 3 as libc::c_int)
                    << 3 as libc::c_int);
        lastBlockBytes += 1;
        lastBlockBytes;
    }
    w[(lastBlockBytes >> 2 as libc::c_int) as usize]
        |= (0x80 as libc::c_uint)
            << (3 as libc::c_int - (lastBlockBytes & 3 as libc::c_int)
                << 3 as libc::c_int);
    if endCurrentBlock >= 56 as libc::c_int {
        innerHash(result.as_mut_ptr(), w.as_mut_ptr());
        clearWBuffert(w.as_mut_ptr());
    }
    w[15 as libc::c_int as usize] = (bytelength << 3 as libc::c_int) as libc::c_uint;
    innerHash(result.as_mut_ptr(), w.as_mut_ptr());
    hashByte = 20 as libc::c_int;
    loop {
        hashByte -= 1;
        if !(hashByte >= 0 as libc::c_int) {
            break;
        }
        *hash
            .offset(
                hashByte as isize,
            ) = (result[(hashByte >> 2 as libc::c_int) as usize]
            >> ((3 as libc::c_int - hashByte & 0x3 as libc::c_int) << 3 as libc::c_int)
            & 0xff as libc::c_int as libc::c_uint) as libc::c_uchar;
    };
}
unsafe extern "C" fn SHA1_Calculate(
    mut src: *const libc::c_void,
    mut length: libc::c_uint,
) -> SHA1 {
    let mut hash: SHA1 = SHA1 { data: [0; 20] };
    if length as libc::c_int >= 0 as libc::c_int {} else {
        __assert_fail(
            b"(int)length >= 0\0" as *const u8 as *const libc::c_char,
            b"lib/Remotery.c\0" as *const u8 as *const libc::c_char,
            3476 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 48],
                &[libc::c_char; 48],
            >(b"SHA1 SHA1_Calculate(const void *, unsigned int)\0"))
                .as_ptr(),
        );
    }
    'c_20560: {
        if length as libc::c_int >= 0 as libc::c_int {} else {
            __assert_fail(
                b"(int)length >= 0\0" as *const u8 as *const libc::c_char,
                b"lib/Remotery.c\0" as *const u8 as *const libc::c_char,
                3476 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 48],
                    &[libc::c_char; 48],
                >(b"SHA1 SHA1_Calculate(const void *, unsigned int)\0"))
                    .as_ptr(),
            );
        }
    };
    calc(src, length as libc::c_int, (hash.data).as_mut_ptr());
    return hash;
}
static mut b64_encoding_table: *const libc::c_char = b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/\0"
    as *const u8 as *const libc::c_char;
unsafe extern "C" fn Base64_CalculateEncodedLength(mut length: rmtU32) -> rmtU32 {
    return (4 as libc::c_int as libc::c_uint)
        .wrapping_mul(
            length
                .wrapping_add(2 as libc::c_int as libc::c_uint)
                .wrapping_div(3 as libc::c_int as libc::c_uint),
        );
}
unsafe extern "C" fn Base64_Encode(
    mut in_bytes: *const rmtU8,
    mut length: rmtU32,
    mut out_bytes: *mut rmtU8,
) {
    let mut i: rmtU32 = 0;
    let mut encoded_length: rmtU32 = 0;
    let mut remaining_bytes: rmtU32 = 0;
    let mut optr: *mut rmtU8 = out_bytes;
    i = 0 as libc::c_int as rmtU32;
    while i < length {
        let mut c0: rmtU32 = (if i < length {
            let fresh26 = i;
            i = i.wrapping_add(1);
            *in_bytes.offset(fresh26 as isize) as libc::c_int
        } else {
            0 as libc::c_int
        }) as rmtU32;
        let mut c1: rmtU32 = (if i < length {
            let fresh27 = i;
            i = i.wrapping_add(1);
            *in_bytes.offset(fresh27 as isize) as libc::c_int
        } else {
            0 as libc::c_int
        }) as rmtU32;
        let mut c2: rmtU32 = (if i < length {
            let fresh28 = i;
            i = i.wrapping_add(1);
            *in_bytes.offset(fresh28 as isize) as libc::c_int
        } else {
            0 as libc::c_int
        }) as rmtU32;
        let mut triple: rmtU32 = (c0 << 0x10 as libc::c_int)
            .wrapping_add(c1 << 0x8 as libc::c_int)
            .wrapping_add(c2);
        let fresh29 = optr;
        optr = optr.offset(1);
        *fresh29 = *b64_encoding_table
            .offset(
                (triple >> 3 as libc::c_int * 6 as libc::c_int
                    & 0x3f as libc::c_int as libc::c_uint) as isize,
            ) as rmtU8;
        let fresh30 = optr;
        optr = optr.offset(1);
        *fresh30 = *b64_encoding_table
            .offset(
                (triple >> 2 as libc::c_int * 6 as libc::c_int
                    & 0x3f as libc::c_int as libc::c_uint) as isize,
            ) as rmtU8;
        let fresh31 = optr;
        optr = optr.offset(1);
        *fresh31 = *b64_encoding_table
            .offset(
                (triple >> 1 as libc::c_int * 6 as libc::c_int
                    & 0x3f as libc::c_int as libc::c_uint) as isize,
            ) as rmtU8;
        let fresh32 = optr;
        optr = optr.offset(1);
        *fresh32 = *b64_encoding_table
            .offset(
                (triple >> 0 as libc::c_int * 6 as libc::c_int
                    & 0x3f as libc::c_int as libc::c_uint) as isize,
            ) as rmtU8;
    }
    encoded_length = Base64_CalculateEncodedLength(length);
    remaining_bytes = (3 as libc::c_int as libc::c_uint)
        .wrapping_sub(
            length
                .wrapping_add(2 as libc::c_int as libc::c_uint)
                .wrapping_rem(3 as libc::c_int as libc::c_uint),
        )
        .wrapping_sub(1 as libc::c_int as libc::c_uint);
    i = 0 as libc::c_int as rmtU32;
    while i < remaining_bytes {
        *out_bytes
            .offset(
                encoded_length
                    .wrapping_sub(1 as libc::c_int as libc::c_uint)
                    .wrapping_sub(i) as isize,
            ) = '=' as i32 as rmtU8;
        i = i.wrapping_add(1);
        i;
    }
    *out_bytes.offset(encoded_length as isize) = 0 as libc::c_int as rmtU8;
}
unsafe extern "C" fn rotl32(mut x: rmtU32, mut r: rmtS8) -> rmtU32 {
    return x << r as libc::c_int | x >> 32 as libc::c_int - r as libc::c_int;
}
unsafe extern "C" fn getblock32(mut p: *const rmtU32, mut i: libc::c_int) -> rmtU32 {
    let mut result: rmtU32 = 0;
    let mut src: *const rmtU8 = (p as *const rmtU8)
        .offset(
            (i * ::std::mem::size_of::<rmtU32>() as libc::c_ulong as libc::c_int)
                as isize,
        );
    memcpy(
        &mut result as *mut rmtU32 as *mut libc::c_void,
        src as *const libc::c_void,
        ::std::mem::size_of::<rmtU32>() as libc::c_ulong,
    );
    return result;
}
unsafe extern "C" fn fmix32(mut h: rmtU32) -> rmtU32 {
    h ^= h >> 16 as libc::c_int;
    h = (h as libc::c_uint).wrapping_mul(0x85ebca6b as libc::c_uint) as rmtU32 as rmtU32;
    h ^= h >> 13 as libc::c_int;
    h = (h as libc::c_uint).wrapping_mul(0xc2b2ae35 as libc::c_uint) as rmtU32 as rmtU32;
    h ^= h >> 16 as libc::c_int;
    return h;
}
unsafe extern "C" fn MurmurHash3_x86_32(
    mut key: *const libc::c_void,
    mut len: libc::c_int,
    mut seed: rmtU32,
) -> rmtU32 {
    let mut data: *const rmtU8 = key as *const rmtU8;
    let nblocks: libc::c_int = len / 4 as libc::c_int;
    let mut h1: rmtU32 = seed;
    let c1: rmtU32 = 0xcc9e2d51 as libc::c_uint;
    let c2: rmtU32 = 0x1b873593 as libc::c_int as rmtU32;
    let mut i: libc::c_int = 0;
    let mut blocks: *const rmtU32 = data.offset((nblocks * 4 as libc::c_int) as isize)
        as *const rmtU32;
    let mut tail: *const rmtU8 = data.offset((nblocks * 4 as libc::c_int) as isize);
    let mut k1: rmtU32 = 0 as libc::c_int as rmtU32;
    i = -nblocks;
    while i != 0 {
        let mut k2: rmtU32 = getblock32(blocks, i);
        k2 = (k2 as libc::c_uint).wrapping_mul(c1) as rmtU32 as rmtU32;
        k2 = rotl32(k2, 15 as libc::c_int as rmtS8);
        k2 = (k2 as libc::c_uint).wrapping_mul(c2) as rmtU32 as rmtU32;
        h1 ^= k2;
        h1 = rotl32(h1, 13 as libc::c_int as rmtS8);
        h1 = h1
            .wrapping_mul(5 as libc::c_int as libc::c_uint)
            .wrapping_add(0xe6546b64 as libc::c_uint);
        i += 1;
        i;
    }
    let mut current_block_15: u64;
    match len & 3 as libc::c_int {
        3 => {
            k1
                ^= ((*tail.offset(2 as libc::c_int as isize) as libc::c_int)
                    << 16 as libc::c_int) as libc::c_uint;
            current_block_15 = 17264445587133170540;
        }
        2 => {
            current_block_15 = 17264445587133170540;
        }
        1 => {
            current_block_15 = 3232300654658835597;
        }
        _ => {
            current_block_15 = 15976848397966268834;
        }
    }
    match current_block_15 {
        17264445587133170540 => {
            k1
                ^= ((*tail.offset(1 as libc::c_int as isize) as libc::c_int)
                    << 8 as libc::c_int) as libc::c_uint;
            current_block_15 = 3232300654658835597;
        }
        _ => {}
    }
    match current_block_15 {
        3232300654658835597 => {
            k1 ^= *tail.offset(0 as libc::c_int as isize) as libc::c_uint;
            k1 = (k1 as libc::c_uint).wrapping_mul(c1) as rmtU32 as rmtU32;
            k1 = rotl32(k1, 15 as libc::c_int as rmtS8);
            k1 = (k1 as libc::c_uint).wrapping_mul(c2) as rmtU32 as rmtU32;
            h1 ^= k1;
        }
        _ => {}
    }
    h1 ^= len as libc::c_uint;
    h1 = fmix32(h1);
    return h1;
}
pub unsafe extern "C" fn _rmt_HashString32(
    mut s: *const libc::c_char,
    mut len: libc::c_int,
    mut seed: rmtU32,
) -> rmtU32 {
    return MurmurHash3_x86_32(s as *const libc::c_void, len, seed);
}
unsafe extern "C" fn GetField(
    mut buffer: *mut libc::c_char,
    mut buffer_length: r_size_t,
    mut field_name: rmtPStr,
) -> *mut libc::c_char {
    let mut field: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut buffer_end: *mut libc::c_char = buffer
        .offset(buffer_length as isize)
        .offset(-(1 as libc::c_int as isize));
    let mut field_length: r_size_t = strnlen_s_safe_c(field_name, buffer_length);
    if field_length == 0 as libc::c_int as libc::c_uint {
        return 0 as *mut libc::c_char;
    }
    if strstr_s(buffer, buffer_length, field_name, field_length, &mut field)
        != 0 as libc::c_int
    {
        return 0 as *mut libc::c_char;
    }
    field = field.offset(strlen(field_name) as isize);
    while *field as libc::c_int == ' ' as i32 {
        if field >= buffer_end {
            return 0 as *mut libc::c_char;
        }
        field = field.offset(1);
        field;
    }
    return field;
}
static mut websocket_guid: [libc::c_char; 37] = unsafe {
    *::std::mem::transmute::<
        &[u8; 37],
        &[libc::c_char; 37],
    >(b"258EAFA5-E914-47DA-95CA-C5AB0DC85B11\0")
};
static mut websocket_response: [libc::c_char; 98] = unsafe {
    *::std::mem::transmute::<
        &[u8; 98],
        &[libc::c_char; 98],
    >(
        b"HTTP/1.1 101 Switching Protocols\r\nUpgrade: websocket\r\nConnection: Upgrade\r\nSec-WebSocket-Accept: \0",
    )
};
unsafe extern "C" fn WebSocketHandshake(
    mut tcp_socket: *mut TCPSocket,
    mut limit_host: rmtPStr,
) -> rmtError {
    let mut start_ms: rmtU32 = 0;
    let mut now_ms: rmtU32 = 0;
    let mut buffer: [libc::c_char; 1024] = [0; 1024];
    let mut buffer_ptr: *mut libc::c_char = buffer.as_mut_ptr();
    let mut buffer_len: libc::c_int = (::std::mem::size_of::<[libc::c_char; 1024]>()
        as libc::c_ulong)
        .wrapping_sub(1 as libc::c_int as libc::c_ulong) as libc::c_int;
    let mut buffer_end: *mut libc::c_char = buffer
        .as_mut_ptr()
        .offset(buffer_len as isize);
    let mut response_buffer: [libc::c_char; 256] = [0; 256];
    let mut response_buffer_len: libc::c_int = (::std::mem::size_of::<
        [libc::c_char; 256],
    >() as libc::c_ulong)
        .wrapping_sub(1 as libc::c_int as libc::c_ulong) as libc::c_int;
    let mut version: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut host: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut key: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut key_end: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut hash: SHA1 = SHA1 { data: [0; 20] };
    if !tcp_socket.is_null() {} else {
        __assert_fail(
            b"tcp_socket != NULL\0" as *const u8 as *const libc::c_char,
            b"lib/Remotery.c\0" as *const u8 as *const libc::c_char,
            3731 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 50],
                &[libc::c_char; 50],
            >(b"rmtError WebSocketHandshake(TCPSocket *, rmtPStr)\0"))
                .as_ptr(),
        );
    }
    'c_21392: {
        if !tcp_socket.is_null() {} else {
            __assert_fail(
                b"tcp_socket != NULL\0" as *const u8 as *const libc::c_char,
                b"lib/Remotery.c\0" as *const u8 as *const libc::c_char,
                3731 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 50],
                    &[libc::c_char; 50],
                >(b"rmtError WebSocketHandshake(TCPSocket *, rmtPStr)\0"))
                    .as_ptr(),
            );
        }
    };
    start_ms = msTimer_Get();
    while (buffer_ptr.offset_from(buffer.as_mut_ptr()) as libc::c_long)
        < buffer_len as libc::c_long
    {
        let mut error: rmtError = TCPSocket_Receive(
            tcp_socket,
            buffer_ptr as *mut libc::c_void,
            1 as libc::c_int as rmtU32,
            20 as libc::c_int as rmtU32,
        );
        if error as libc::c_uint
            == RMT_ERROR_SOCKET_RECV_FAILED as libc::c_int as libc::c_uint
        {
            return error;
        }
        if error as libc::c_uint
            == RMT_ERROR_SOCKET_RECV_NO_DATA as libc::c_int as libc::c_uint
            || error as libc::c_uint
                == RMT_ERROR_SOCKET_RECV_TIMEOUT as libc::c_int as libc::c_uint
        {
            now_ms = msTimer_Get();
            if now_ms.wrapping_sub(start_ms) > 1000 as libc::c_int as libc::c_uint {
                return RMT_ERROR_SOCKET_RECV_TIMEOUT;
            }
        } else {
            if error as libc::c_uint == RMT_ERROR_NONE as libc::c_int as libc::c_uint
            {} else {
                __assert_fail(
                    b"error == RMT_ERROR_NONE\0" as *const u8 as *const libc::c_char,
                    b"lib/Remotery.c\0" as *const u8 as *const libc::c_char,
                    3754 as libc::c_int as libc::c_uint,
                    (*::std::mem::transmute::<
                        &[u8; 50],
                        &[libc::c_char; 50],
                    >(b"rmtError WebSocketHandshake(TCPSocket *, rmtPStr)\0"))
                        .as_ptr(),
                );
            }
            'c_21281: {
                if error as libc::c_uint == RMT_ERROR_NONE as libc::c_int as libc::c_uint
                {} else {
                    __assert_fail(
                        b"error == RMT_ERROR_NONE\0" as *const u8 as *const libc::c_char,
                        b"lib/Remotery.c\0" as *const u8 as *const libc::c_char,
                        3754 as libc::c_int as libc::c_uint,
                        (*::std::mem::transmute::<
                            &[u8; 50],
                            &[libc::c_char; 50],
                        >(b"rmtError WebSocketHandshake(TCPSocket *, rmtPStr)\0"))
                            .as_ptr(),
                    );
                }
            };
            if buffer_ptr.offset_from(buffer.as_mut_ptr()) as libc::c_long
                >= 4 as libc::c_int as libc::c_long
            {
                if *buffer_ptr.offset(-(3 as libc::c_int as isize)) as libc::c_int
                    == '\r' as i32
                    && *buffer_ptr.offset(-(2 as libc::c_int as isize)) as libc::c_int
                        == '\n' as i32
                    && *buffer_ptr.offset(-(1 as libc::c_int as isize)) as libc::c_int
                        == '\r' as i32
                    && *buffer_ptr.offset(-(0 as libc::c_int as isize)) as libc::c_int
                        == '\n' as i32
                {
                    break;
                }
            }
            buffer_ptr = buffer_ptr.offset(1);
            buffer_ptr;
        }
    }
    *buffer_ptr = 0 as libc::c_int as libc::c_char;
    if memcmp(
        buffer.as_mut_ptr() as *const libc::c_void,
        b"GET\0" as *const u8 as *const libc::c_char as *const libc::c_void,
        3 as libc::c_int as libc::c_ulong,
    ) != 0 as libc::c_int
    {
        return RMT_ERROR_WEBSOCKET_HANDSHAKE_NOT_GET;
    }
    version = GetField(
        buffer.as_mut_ptr(),
        buffer_len as r_size_t,
        b"Sec-WebSocket-Version:\0" as *const u8 as *const libc::c_char,
    );
    if version.is_null() {
        return RMT_ERROR_WEBSOCKET_HANDSHAKE_NO_VERSION;
    }
    if (buffer_end.offset_from(version) as libc::c_long)
        < 2 as libc::c_int as libc::c_long
        || *version.offset(0 as libc::c_int as isize) as libc::c_int != '8' as i32
            && (*version.offset(0 as libc::c_int as isize) as libc::c_int != '1' as i32
                || *version.offset(1 as libc::c_int as isize) as libc::c_int
                    != '3' as i32)
    {
        return RMT_ERROR_WEBSOCKET_HANDSHAKE_BAD_VERSION;
    }
    host = GetField(
        buffer.as_mut_ptr(),
        buffer_len as r_size_t,
        b"Host:\0" as *const u8 as *const libc::c_char,
    );
    if host.is_null() {
        return RMT_ERROR_WEBSOCKET_HANDSHAKE_NO_HOST;
    }
    if !limit_host.is_null() {
        let mut limit_host_len: r_size_t = strnlen_s_safe_c(
            limit_host,
            128 as libc::c_int as r_size_t,
        );
        let mut found: *mut libc::c_char = 0 as *mut libc::c_char;
        if strstr_s(
            host,
            buffer_end.offset_from(host) as libc::c_long as r_size_t,
            limit_host,
            limit_host_len,
            &mut found,
        ) != 0 as libc::c_int
        {
            return RMT_ERROR_WEBSOCKET_HANDSHAKE_BAD_HOST;
        }
    }
    key = GetField(
        buffer.as_mut_ptr(),
        buffer_len as r_size_t,
        b"Sec-WebSocket-Key:\0" as *const u8 as *const libc::c_char,
    );
    if key.is_null() {
        return RMT_ERROR_WEBSOCKET_HANDSHAKE_NO_KEY;
    }
    if strstr_s(
        key,
        buffer_end.offset_from(key) as libc::c_long as r_size_t,
        b"\r\n\0" as *const u8 as *const libc::c_char,
        2 as libc::c_int as r_size_t,
        &mut key_end,
    ) != 0 as libc::c_int
    {
        return RMT_ERROR_WEBSOCKET_HANDSHAKE_BAD_KEY;
    }
    *key_end = 0 as libc::c_int as libc::c_char;
    buffer[0 as libc::c_int as usize] = 0 as libc::c_int as libc::c_char;
    if strncat_s_safe_c(
        buffer.as_mut_ptr(),
        buffer_len as r_size_t,
        key,
        key_end.offset_from(key) as libc::c_long as r_size_t,
    ) != 0 as libc::c_int
    {
        return RMT_ERROR_WEBSOCKET_HANDSHAKE_STRING_FAIL;
    }
    if strncat_s_safe_c(
        buffer.as_mut_ptr(),
        buffer_len as r_size_t,
        websocket_guid.as_ptr(),
        ::std::mem::size_of::<[libc::c_char; 37]>() as libc::c_ulong as r_size_t,
    ) != 0 as libc::c_int
    {
        return RMT_ERROR_WEBSOCKET_HANDSHAKE_STRING_FAIL;
    }
    hash = SHA1_Calculate(
        buffer.as_mut_ptr() as *const libc::c_void,
        strnlen_s_safe_c(buffer.as_mut_ptr(), buffer_len as r_size_t),
    );
    Base64_Encode(
        (hash.data).as_mut_ptr(),
        ::std::mem::size_of::<[rmtU8; 20]>() as libc::c_ulong as rmtU32,
        buffer.as_mut_ptr() as *mut rmtU8,
    );
    response_buffer[0 as libc::c_int as usize] = 0 as libc::c_int as libc::c_char;
    if strncat_s_safe_c(
        response_buffer.as_mut_ptr(),
        response_buffer_len as r_size_t,
        websocket_response.as_ptr(),
        ::std::mem::size_of::<[libc::c_char; 98]>() as libc::c_ulong as r_size_t,
    ) != 0 as libc::c_int
    {
        return RMT_ERROR_WEBSOCKET_HANDSHAKE_STRING_FAIL;
    }
    if strncat_s_safe_c(
        response_buffer.as_mut_ptr(),
        response_buffer_len as r_size_t,
        buffer.as_mut_ptr(),
        buffer_len as r_size_t,
    ) != 0 as libc::c_int
    {
        return RMT_ERROR_WEBSOCKET_HANDSHAKE_STRING_FAIL;
    }
    if strncat_s_safe_c(
        response_buffer.as_mut_ptr(),
        response_buffer_len as r_size_t,
        b"\r\n\r\n\0" as *const u8 as *const libc::c_char,
        4 as libc::c_int as r_size_t,
    ) != 0 as libc::c_int
    {
        return RMT_ERROR_WEBSOCKET_HANDSHAKE_STRING_FAIL;
    }
    return TCPSocket_Send(
        tcp_socket,
        response_buffer.as_mut_ptr() as *const libc::c_void,
        strnlen_s_safe_c(response_buffer.as_mut_ptr(), response_buffer_len as r_size_t),
        1000 as libc::c_int as rmtU32,
    );
}
unsafe extern "C" fn WebSocket_Constructor(
    mut web_socket: *mut WebSocket,
    mut tcp_socket: *mut TCPSocket,
) -> rmtError {
    let mut error: rmtError = RMT_ERROR_NONE;
    if !web_socket.is_null() {} else {
        __assert_fail(
            b"web_socket != NULL\0" as *const u8 as *const libc::c_char,
            b"lib/Remotery.c\0" as *const u8 as *const libc::c_char,
            3824 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 57],
                &[libc::c_char; 57],
            >(b"rmtError WebSocket_Constructor(WebSocket *, TCPSocket *)\0"))
                .as_ptr(),
        );
    }
    'c_18906: {
        if !web_socket.is_null() {} else {
            __assert_fail(
                b"web_socket != NULL\0" as *const u8 as *const libc::c_char,
                b"lib/Remotery.c\0" as *const u8 as *const libc::c_char,
                3824 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 57],
                    &[libc::c_char; 57],
                >(b"rmtError WebSocket_Constructor(WebSocket *, TCPSocket *)\0"))
                    .as_ptr(),
            );
        }
    };
    (*web_socket).tcp_socket = tcp_socket;
    (*web_socket).mode = WEBSOCKET_NONE;
    (*web_socket).frame_bytes_remaining = 0 as libc::c_int as rmtU32;
    (*web_socket).mask_offset = 0 as libc::c_int as rmtU32;
    (*web_socket).data.mask[0 as libc::c_int as usize] = 0 as libc::c_int as rmtU8;
    (*web_socket).data.mask[1 as libc::c_int as usize] = 0 as libc::c_int as rmtU8;
    (*web_socket).data.mask[2 as libc::c_int as usize] = 0 as libc::c_int as rmtU8;
    (*web_socket).data.mask[3 as libc::c_int as usize] = 0 as libc::c_int as rmtU8;
    if ((*web_socket).tcp_socket).is_null() {
        (*web_socket)
            .tcp_socket = rmtMalloc(
            ::std::mem::size_of::<TCPSocket>() as libc::c_ulong as rmtU32,
        ) as *mut TCPSocket;
        if ((*web_socket).tcp_socket).is_null() {
            return RMT_ERROR_MALLOC_FAIL;
        }
        let mut error_0: rmtError = TCPSocket_Constructor((*web_socket).tcp_socket);
        if error_0 as libc::c_uint != RMT_ERROR_NONE as libc::c_int as libc::c_uint {
            if !((*web_socket).tcp_socket).is_null() {
                TCPSocket_Destructor((*web_socket).tcp_socket);
                rmtFree((*web_socket).tcp_socket as *mut libc::c_void);
                (*web_socket).tcp_socket = 0 as *mut TCPSocket;
            }
            return error_0;
        }
    }
    return error;
}
unsafe extern "C" fn WebSocket_Destructor(mut web_socket: *mut WebSocket) {
    WebSocket_Close(web_socket);
}
unsafe extern "C" fn WebSocket_RunServer(
    mut web_socket: *mut WebSocket,
    mut port: rmtU16,
    mut reuse_open_port: rmtBool,
    mut limit_connections_to_localhost: rmtBool,
    mut mode: WebSocketMode,
) -> rmtError {
    if !web_socket.is_null() {} else {
        __assert_fail(
            b"web_socket != NULL\0" as *const u8 as *const libc::c_char,
            b"lib/Remotery.c\0" as *const u8 as *const libc::c_char,
            3850 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 88],
                &[libc::c_char; 88],
            >(
                b"rmtError WebSocket_RunServer(WebSocket *, rmtU16, rmtBool, rmtBool, enum WebSocketMode)\0",
            ))
                .as_ptr(),
        );
    }
    'c_22185: {
        if !web_socket.is_null() {} else {
            __assert_fail(
                b"web_socket != NULL\0" as *const u8 as *const libc::c_char,
                b"lib/Remotery.c\0" as *const u8 as *const libc::c_char,
                3850 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 88],
                    &[libc::c_char; 88],
                >(
                    b"rmtError WebSocket_RunServer(WebSocket *, rmtU16, rmtBool, rmtBool, enum WebSocketMode)\0",
                ))
                    .as_ptr(),
            );
        }
    };
    (*web_socket).mode = mode;
    return TCPSocket_RunServer(
        (*web_socket).tcp_socket,
        port,
        reuse_open_port,
        limit_connections_to_localhost,
    );
}
unsafe extern "C" fn WebSocket_Close(mut web_socket: *mut WebSocket) {
    if !web_socket.is_null() {} else {
        __assert_fail(
            b"web_socket != NULL\0" as *const u8 as *const libc::c_char,
            b"lib/Remotery.c\0" as *const u8 as *const libc::c_char,
            3857 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 34],
                &[libc::c_char; 34],
            >(b"void WebSocket_Close(WebSocket *)\0"))
                .as_ptr(),
        );
    }
    'c_10708: {
        if !web_socket.is_null() {} else {
            __assert_fail(
                b"web_socket != NULL\0" as *const u8 as *const libc::c_char,
                b"lib/Remotery.c\0" as *const u8 as *const libc::c_char,
                3857 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 34],
                    &[libc::c_char; 34],
                >(b"void WebSocket_Close(WebSocket *)\0"))
                    .as_ptr(),
            );
        }
    };
    if !((*web_socket).tcp_socket).is_null() {
        TCPSocket_Destructor((*web_socket).tcp_socket);
        rmtFree((*web_socket).tcp_socket as *mut libc::c_void);
        (*web_socket).tcp_socket = 0 as *mut TCPSocket;
    }
}
unsafe extern "C" fn WebSocket_PollStatus(
    mut web_socket: *mut WebSocket,
) -> SocketStatus {
    if !web_socket.is_null() {} else {
        __assert_fail(
            b"web_socket != NULL\0" as *const u8 as *const libc::c_char,
            b"lib/Remotery.c\0" as *const u8 as *const libc::c_char,
            3863 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 47],
                &[libc::c_char; 47],
            >(b"SocketStatus WebSocket_PollStatus(WebSocket *)\0"))
                .as_ptr(),
        );
    }
    'c_10283: {
        if !web_socket.is_null() {} else {
            __assert_fail(
                b"web_socket != NULL\0" as *const u8 as *const libc::c_char,
                b"lib/Remotery.c\0" as *const u8 as *const libc::c_char,
                3863 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 47],
                    &[libc::c_char; 47],
                >(b"SocketStatus WebSocket_PollStatus(WebSocket *)\0"))
                    .as_ptr(),
            );
        }
    };
    return TCPSocket_PollStatus((*web_socket).tcp_socket);
}
unsafe extern "C" fn WebSocket_AcceptConnection(
    mut web_socket: *mut WebSocket,
    mut client_socket: *mut *mut WebSocket,
) -> rmtError {
    let mut tcp_socket: *mut TCPSocket = 0 as *mut TCPSocket;
    if !web_socket.is_null() {} else {
        __assert_fail(
            b"web_socket != NULL\0" as *const u8 as *const libc::c_char,
            b"lib/Remotery.c\0" as *const u8 as *const libc::c_char,
            3872 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 63],
                &[libc::c_char; 63],
            >(b"rmtError WebSocket_AcceptConnection(WebSocket *, WebSocket **)\0"))
                .as_ptr(),
        );
    }
    'c_21739: {
        if !web_socket.is_null() {} else {
            __assert_fail(
                b"web_socket != NULL\0" as *const u8 as *const libc::c_char,
                b"lib/Remotery.c\0" as *const u8 as *const libc::c_char,
                3872 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 63],
                    &[libc::c_char; 63],
                >(b"rmtError WebSocket_AcceptConnection(WebSocket *, WebSocket **)\0"))
                    .as_ptr(),
            );
        }
    };
    let mut error: rmtError = TCPSocket_AcceptConnection(
        (*web_socket).tcp_socket,
        &mut tcp_socket,
    );
    if error as libc::c_uint != RMT_ERROR_NONE as libc::c_int as libc::c_uint {
        return error;
    }
    if tcp_socket.is_null() {
        return RMT_ERROR_NONE;
    }
    let mut error_0: rmtError = WebSocketHandshake(tcp_socket, 0 as rmtPStr);
    if error_0 as libc::c_uint != RMT_ERROR_NONE as libc::c_int as libc::c_uint {
        return error_0;
    }
    if !client_socket.is_null() {} else {
        __assert_fail(
            b"client_socket != NULL\0" as *const u8 as *const libc::c_char,
            b"lib/Remotery.c\0" as *const u8 as *const libc::c_char,
            3882 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 63],
                &[libc::c_char; 63],
            >(b"rmtError WebSocket_AcceptConnection(WebSocket *, WebSocket **)\0"))
                .as_ptr(),
        );
    }
    'c_19012: {
        if !client_socket.is_null() {} else {
            __assert_fail(
                b"client_socket != NULL\0" as *const u8 as *const libc::c_char,
                b"lib/Remotery.c\0" as *const u8 as *const libc::c_char,
                3882 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 63],
                    &[libc::c_char; 63],
                >(b"rmtError WebSocket_AcceptConnection(WebSocket *, WebSocket **)\0"))
                    .as_ptr(),
            );
        }
    };
    *client_socket = rmtMalloc(
        ::std::mem::size_of::<WebSocket>() as libc::c_ulong as rmtU32,
    ) as *mut WebSocket;
    if (*client_socket).is_null() {
        return RMT_ERROR_MALLOC_FAIL;
    }
    let mut error_1: rmtError = WebSocket_Constructor(*client_socket, tcp_socket);
    if error_1 as libc::c_uint != RMT_ERROR_NONE as libc::c_int as libc::c_uint {
        if !(*client_socket).is_null() {
            WebSocket_Destructor(*client_socket);
            rmtFree(*client_socket as *mut libc::c_void);
            *client_socket = 0 as *mut WebSocket;
        }
        return error_1;
    }
    (**client_socket).mode = (*web_socket).mode;
    return RMT_ERROR_NONE;
}
unsafe extern "C" fn WriteSize(
    mut size: rmtU32,
    mut dest: *mut rmtU8,
    mut dest_size: rmtU32,
    mut dest_offset: rmtU32,
) {
    let mut size_size: libc::c_int = dest_size.wrapping_sub(dest_offset) as libc::c_int;
    let mut i: rmtU32 = 0;
    i = 0 as libc::c_int as rmtU32;
    while i < dest_size {
        let mut j: libc::c_int = i.wrapping_sub(dest_offset) as libc::c_int;
        *dest
            .offset(
                i as isize,
            ) = (if j < 0 as libc::c_int {
            0 as libc::c_int as libc::c_uint
        } else {
            size >> (size_size - j - 1 as libc::c_int) * 8 as libc::c_int
                & 0xff as libc::c_int as libc::c_uint
        }) as rmtU8;
        i = i.wrapping_add(1);
        i;
    }
}
unsafe extern "C" fn WebSocket_PrepareBuffer(mut buffer: *mut Buffer) {
    let mut empty_frame_header: [libc::c_char; 10] = [0; 10];
    if !buffer.is_null() {} else {
        __assert_fail(
            b"buffer != NULL\0" as *const u8 as *const libc::c_char,
            b"lib/Remotery.c\0" as *const u8 as *const libc::c_char,
            3908 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 39],
                &[libc::c_char; 39],
            >(b"void WebSocket_PrepareBuffer(Buffer *)\0"))
                .as_ptr(),
        );
    }
    'c_13685: {
        if !buffer.is_null() {} else {
            __assert_fail(
                b"buffer != NULL\0" as *const u8 as *const libc::c_char,
                b"lib/Remotery.c\0" as *const u8 as *const libc::c_char,
                3908 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 39],
                    &[libc::c_char; 39],
                >(b"void WebSocket_PrepareBuffer(Buffer *)\0"))
                    .as_ptr(),
            );
        }
    };
    (*buffer).bytes_used = 0 as libc::c_int as rmtU32;
    Buffer_Write(
        buffer,
        empty_frame_header.as_mut_ptr() as *const libc::c_void,
        ::std::mem::size_of::<[libc::c_char; 10]>() as libc::c_ulong as rmtU32,
    );
}
unsafe extern "C" fn WebSocket_FrameHeaderSize(mut length: rmtU32) -> rmtU32 {
    if length <= 125 as libc::c_int as libc::c_uint {
        return 2 as libc::c_int as rmtU32;
    }
    if length <= 65535 as libc::c_int as libc::c_uint {
        return 4 as libc::c_int as rmtU32;
    }
    return 10 as libc::c_int as rmtU32;
}
unsafe extern "C" fn WebSocket_WriteFrameHeader(
    mut web_socket: *mut WebSocket,
    mut dest: *mut rmtU8,
    mut length: rmtU32,
) {
    let mut final_fragment: rmtU8 = ((0x1 as libc::c_int) << 7 as libc::c_int) as rmtU8;
    let mut frame_type: rmtU8 = (*web_socket).mode as rmtU8;
    *dest
        .offset(
            0 as libc::c_int as isize,
        ) = (final_fragment as libc::c_int | frame_type as libc::c_int) as rmtU8;
    if length <= 125 as libc::c_int as libc::c_uint {
        *dest.offset(1 as libc::c_int as isize) = length as rmtU8;
    } else if length <= 65535 as libc::c_int as libc::c_uint {
        *dest.offset(1 as libc::c_int as isize) = 126 as libc::c_int as rmtU8;
        WriteSize(
            length,
            dest.offset(2 as libc::c_int as isize),
            2 as libc::c_int as rmtU32,
            0 as libc::c_int as rmtU32,
        );
    } else {
        *dest.offset(1 as libc::c_int as isize) = 127 as libc::c_int as rmtU8;
        WriteSize(
            length,
            dest.offset(2 as libc::c_int as isize),
            8 as libc::c_int as rmtU32,
            4 as libc::c_int as rmtU32,
        );
    };
}
unsafe extern "C" fn WebSocket_Send(
    mut web_socket: *mut WebSocket,
    mut data: *const libc::c_void,
    mut length: rmtU32,
    mut timeout_ms: rmtU32,
) -> rmtError {
    let mut error: rmtError = RMT_ERROR_NONE;
    let mut status: SocketStatus = SocketStatus {
        can_read: 0,
        can_write: 0,
        error_state: RMT_ERROR_NONE,
    };
    let mut payload_length: rmtU32 = 0;
    let mut frame_header_size: rmtU32 = 0;
    let mut delta: rmtU32 = 0;
    if !web_socket.is_null() {} else {
        __assert_fail(
            b"web_socket != NULL\0" as *const u8 as *const libc::c_char,
            b"lib/Remotery.c\0" as *const u8 as *const libc::c_char,
            3956 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 67],
                &[libc::c_char; 67],
            >(b"rmtError WebSocket_Send(WebSocket *, const void *, rmtU32, rmtU32)\0"))
                .as_ptr(),
        );
    }
    'c_10367: {
        if !web_socket.is_null() {} else {
            __assert_fail(
                b"web_socket != NULL\0" as *const u8 as *const libc::c_char,
                b"lib/Remotery.c\0" as *const u8 as *const libc::c_char,
                3956 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 67],
                    &[libc::c_char; 67],
                >(
                    b"rmtError WebSocket_Send(WebSocket *, const void *, rmtU32, rmtU32)\0",
                ))
                    .as_ptr(),
            );
        }
    };
    if !data.is_null() {} else {
        __assert_fail(
            b"data != NULL\0" as *const u8 as *const libc::c_char,
            b"lib/Remotery.c\0" as *const u8 as *const libc::c_char,
            3957 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 67],
                &[libc::c_char; 67],
            >(b"rmtError WebSocket_Send(WebSocket *, const void *, rmtU32, rmtU32)\0"))
                .as_ptr(),
        );
    }
    'c_10325: {
        if !data.is_null() {} else {
            __assert_fail(
                b"data != NULL\0" as *const u8 as *const libc::c_char,
                b"lib/Remotery.c\0" as *const u8 as *const libc::c_char,
                3957 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 67],
                    &[libc::c_char; 67],
                >(
                    b"rmtError WebSocket_Send(WebSocket *, const void *, rmtU32, rmtU32)\0",
                ))
                    .as_ptr(),
            );
        }
    };
    status = WebSocket_PollStatus(web_socket);
    if status.error_state as libc::c_uint
        != RMT_ERROR_NONE as libc::c_int as libc::c_uint
    {
        return status.error_state;
    }
    payload_length = length.wrapping_sub(10 as libc::c_int as libc::c_uint);
    frame_header_size = WebSocket_FrameHeaderSize(payload_length);
    delta = (10 as libc::c_int as libc::c_uint).wrapping_sub(frame_header_size);
    data = (data as *mut rmtU8).offset(delta as isize) as *mut libc::c_void;
    length = (length as libc::c_uint).wrapping_sub(delta) as rmtU32 as rmtU32;
    WebSocket_WriteFrameHeader(web_socket, data as *mut rmtU8, payload_length);
    error = TCPSocket_Send((*web_socket).tcp_socket, data, length, timeout_ms);
    return error;
}
unsafe extern "C" fn ReceiveFrameHeader(mut web_socket: *mut WebSocket) -> rmtError {
    let mut msg_header: [rmtU8; 2] = [
        0 as libc::c_int as rmtU8,
        0 as libc::c_int as rmtU8,
    ];
    let mut msg_length: libc::c_int = 0;
    let mut size_bytes_remaining: libc::c_int = 0;
    let mut i: libc::c_int = 0;
    let mut mask_present: rmtBool = 0;
    if !web_socket.is_null() {} else {
        __assert_fail(
            b"web_socket != NULL\0" as *const u8 as *const libc::c_char,
            b"lib/Remotery.c\0" as *const u8 as *const libc::c_char,
            3985 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 41],
                &[libc::c_char; 41],
            >(b"rmtError ReceiveFrameHeader(WebSocket *)\0"))
                .as_ptr(),
        );
    }
    'c_17866: {
        if !web_socket.is_null() {} else {
            __assert_fail(
                b"web_socket != NULL\0" as *const u8 as *const libc::c_char,
                b"lib/Remotery.c\0" as *const u8 as *const libc::c_char,
                3985 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 41],
                    &[libc::c_char; 41],
                >(b"rmtError ReceiveFrameHeader(WebSocket *)\0"))
                    .as_ptr(),
            );
        }
    };
    let mut error: rmtError = TCPSocket_Receive(
        (*web_socket).tcp_socket,
        msg_header.as_mut_ptr() as *mut libc::c_void,
        2 as libc::c_int as rmtU32,
        20 as libc::c_int as rmtU32,
    );
    if error as libc::c_uint != RMT_ERROR_NONE as libc::c_int as libc::c_uint {
        return error;
    }
    if msg_header[0 as libc::c_int as usize] as libc::c_int == 0x88 as libc::c_int {
        return RMT_ERROR_WEBSOCKET_DISCONNECTED;
    }
    if msg_header[0 as libc::c_int as usize] as libc::c_int != 0x81 as libc::c_int
        && msg_header[0 as libc::c_int as usize] as libc::c_int != 0x82 as libc::c_int
    {
        return RMT_ERROR_WEBSOCKET_BAD_FRAME_HEADER;
    }
    msg_length = msg_header[1 as libc::c_int as usize] as libc::c_int
        & 0x7f as libc::c_int;
    size_bytes_remaining = 0 as libc::c_int;
    match msg_length {
        126 => {
            size_bytes_remaining = 2 as libc::c_int;
        }
        127 => {
            size_bytes_remaining = 8 as libc::c_int;
        }
        _ => {}
    }
    if size_bytes_remaining > 0 as libc::c_int {
        let mut size_bytes: [rmtU8; 8] = [0; 8];
        let mut error_0: rmtError = TCPSocket_Receive(
            (*web_socket).tcp_socket,
            size_bytes.as_mut_ptr() as *mut libc::c_void,
            size_bytes_remaining as rmtU32,
            20 as libc::c_int as rmtU32,
        );
        if error_0 as libc::c_uint != RMT_ERROR_NONE as libc::c_int as libc::c_uint {
            return error_0;
        }
        msg_length = 0 as libc::c_int;
        i = 0 as libc::c_int;
        while i < size_bytes_remaining {
            msg_length
                |= (size_bytes[i as usize] as libc::c_int)
                    << (size_bytes_remaining - 1 as libc::c_int - i) * 8 as libc::c_int;
            i += 1;
            i;
        }
    }
    mask_present = if msg_header[1 as libc::c_int as usize] as libc::c_int
        & 0x80 as libc::c_int != 0 as libc::c_int
    {
        1 as libc::c_int as rmtBool
    } else {
        0 as libc::c_int as rmtBool
    };
    if mask_present != 0 {
        let mut error_1: rmtError = TCPSocket_Receive(
            (*web_socket).tcp_socket,
            ((*web_socket).data.mask).as_mut_ptr() as *mut libc::c_void,
            4 as libc::c_int as rmtU32,
            20 as libc::c_int as rmtU32,
        );
        if error_1 as libc::c_uint != RMT_ERROR_NONE as libc::c_int as libc::c_uint {
            return error_1;
        }
    }
    (*web_socket).frame_bytes_remaining = msg_length as rmtU32;
    (*web_socket).mask_offset = 0 as libc::c_int as rmtU32;
    return RMT_ERROR_NONE;
}
unsafe extern "C" fn WebSocket_Receive(
    mut web_socket: *mut WebSocket,
    mut data: *mut libc::c_void,
    mut msg_len: *mut rmtU32,
    mut length: rmtU32,
    mut timeout_ms: rmtU32,
) -> rmtError {
    let mut status: SocketStatus = SocketStatus {
        can_read: 0,
        can_write: 0,
        error_state: RMT_ERROR_NONE,
    };
    let mut cur_data: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut end_data: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut start_ms: rmtU32 = 0;
    let mut now_ms: rmtU32 = 0;
    let mut bytes_to_read: rmtU32 = 0;
    let mut error: rmtError = RMT_ERROR_NONE;
    if !web_socket.is_null() {} else {
        __assert_fail(
            b"web_socket != NULL\0" as *const u8 as *const libc::c_char,
            b"lib/Remotery.c\0" as *const u8 as *const libc::c_char,
            4045 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 74],
                &[libc::c_char; 74],
            >(
                b"rmtError WebSocket_Receive(WebSocket *, void *, rmtU32 *, rmtU32, rmtU32)\0",
            ))
                .as_ptr(),
        );
    }
    'c_17960: {
        if !web_socket.is_null() {} else {
            __assert_fail(
                b"web_socket != NULL\0" as *const u8 as *const libc::c_char,
                b"lib/Remotery.c\0" as *const u8 as *const libc::c_char,
                4045 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 74],
                    &[libc::c_char; 74],
                >(
                    b"rmtError WebSocket_Receive(WebSocket *, void *, rmtU32 *, rmtU32, rmtU32)\0",
                ))
                    .as_ptr(),
            );
        }
    };
    status = WebSocket_PollStatus(web_socket);
    if status.error_state as libc::c_uint
        != RMT_ERROR_NONE as libc::c_int as libc::c_uint
    {
        return status.error_state;
    }
    cur_data = data as *mut libc::c_char;
    end_data = cur_data.offset(length as isize);
    start_ms = msTimer_Get();
    while cur_data < end_data {
        if (*web_socket).frame_bytes_remaining == 0 as libc::c_int as libc::c_uint {
            let mut error_0: rmtError = ReceiveFrameHeader(web_socket);
            if error_0 as libc::c_uint != RMT_ERROR_NONE as libc::c_int as libc::c_uint {
                return error_0;
            }
            if !msg_len.is_null() {
                *msg_len = (*web_socket).frame_bytes_remaining;
            }
        }
        bytes_to_read = if (*web_socket).frame_bytes_remaining < length {
            (*web_socket).frame_bytes_remaining
        } else {
            length
        };
        error = TCPSocket_Receive(
            (*web_socket).tcp_socket,
            cur_data as *mut libc::c_void,
            bytes_to_read,
            20 as libc::c_int as rmtU32,
        );
        if error as libc::c_uint
            == RMT_ERROR_SOCKET_RECV_FAILED as libc::c_int as libc::c_uint
        {
            return error;
        }
        if error as libc::c_uint
            == RMT_ERROR_SOCKET_RECV_NO_DATA as libc::c_int as libc::c_uint
            || error as libc::c_uint
                == RMT_ERROR_SOCKET_RECV_TIMEOUT as libc::c_int as libc::c_uint
        {
            now_ms = msTimer_Get();
            if now_ms.wrapping_sub(start_ms) > timeout_ms {
                return RMT_ERROR_SOCKET_RECV_TIMEOUT;
            }
        } else {
            if (*web_socket).data.mask_u32 != 0 as libc::c_int as libc::c_uint {
                let mut i: rmtU32 = 0;
                i = 0 as libc::c_int as rmtU32;
                while i < bytes_to_read {
                    let ref mut fresh33 = *(cur_data as *mut rmtU8).offset(i as isize);
                    *fresh33 = (*fresh33 as libc::c_int
                        ^ (*web_socket)
                            .data
                            .mask[((*web_socket).mask_offset
                            & 3 as libc::c_int as libc::c_uint) as usize] as libc::c_int)
                        as rmtU8;
                    (*web_socket)
                        .mask_offset = ((*web_socket).mask_offset).wrapping_add(1);
                    (*web_socket).mask_offset;
                    i = i.wrapping_add(1);
                    i;
                }
            }
            cur_data = cur_data.offset(bytes_to_read as isize);
            (*web_socket)
                .frame_bytes_remaining = ((*web_socket).frame_bytes_remaining
                as libc::c_uint)
                .wrapping_sub(bytes_to_read) as rmtU32 as rmtU32;
        }
    }
    return RMT_ERROR_NONE;
}
unsafe extern "C" fn rmtMessageQueue_Constructor(
    mut queue: *mut rmtMessageQueue,
    mut size: rmtU32,
) -> rmtError {
    if !queue.is_null() {} else {
        __assert_fail(
            b"queue != NULL\0" as *const u8 as *const libc::c_char,
            b"lib/Remotery.c\0" as *const u8 as *const libc::c_char,
            4153 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 64],
                &[libc::c_char; 64],
            >(b"rmtError rmtMessageQueue_Constructor(rmtMessageQueue *, rmtU32)\0"))
                .as_ptr(),
        );
    }
    'c_28009: {
        if !queue.is_null() {} else {
            __assert_fail(
                b"queue != NULL\0" as *const u8 as *const libc::c_char,
                b"lib/Remotery.c\0" as *const u8 as *const libc::c_char,
                4153 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 64],
                    &[libc::c_char; 64],
                >(b"rmtError rmtMessageQueue_Constructor(rmtMessageQueue *, rmtU32)\0"))
                    .as_ptr(),
            );
        }
    };
    (*queue).size = 0 as libc::c_int as rmtU32;
    (*queue).data = 0 as *mut VirtualMirrorBuffer;
    ::std::ptr::write_volatile(
        &mut (*queue).read_pos as *mut rmtAtomicU32,
        0 as libc::c_int as rmtU32,
    );
    ::std::ptr::write_volatile(
        &mut (*queue).write_pos as *mut rmtAtomicU32,
        0 as libc::c_int as rmtU32,
    );
    (*queue)
        .data = rmtMalloc(
        ::std::mem::size_of::<VirtualMirrorBuffer>() as libc::c_ulong as rmtU32,
    ) as *mut VirtualMirrorBuffer;
    if ((*queue).data).is_null() {
        return RMT_ERROR_MALLOC_FAIL;
    }
    let mut error: rmtError = VirtualMirrorBuffer_Constructor(
        (*queue).data,
        size,
        10 as libc::c_int,
    );
    if error as libc::c_uint != RMT_ERROR_NONE as libc::c_int as libc::c_uint {
        if !((*queue).data).is_null() {
            VirtualMirrorBuffer_Destructor((*queue).data);
            rmtFree((*queue).data as *mut libc::c_void);
            (*queue).data = 0 as *mut VirtualMirrorBuffer;
        }
        return error;
    }
    (*queue).size = (*(*queue).data).size;
    memset(
        (*(*queue).data).ptr as *mut libc::c_void,
        MsgID_NotReady as libc::c_int,
        (*queue).size as libc::c_ulong,
    );
    return RMT_ERROR_NONE;
}
unsafe extern "C" fn rmtMessageQueue_Destructor(mut queue: *mut rmtMessageQueue) {
    if !queue.is_null() {} else {
        __assert_fail(
            b"queue != NULL\0" as *const u8 as *const libc::c_char,
            b"lib/Remotery.c\0" as *const u8 as *const libc::c_char,
            4175 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 51],
                &[libc::c_char; 51],
            >(b"void rmtMessageQueue_Destructor(rmtMessageQueue *)\0"))
                .as_ptr(),
        );
    }
    'c_28123: {
        if !queue.is_null() {} else {
            __assert_fail(
                b"queue != NULL\0" as *const u8 as *const libc::c_char,
                b"lib/Remotery.c\0" as *const u8 as *const libc::c_char,
                4175 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 51],
                    &[libc::c_char; 51],
                >(b"void rmtMessageQueue_Destructor(rmtMessageQueue *)\0"))
                    .as_ptr(),
            );
        }
    };
    if !((*queue).data).is_null() {
        VirtualMirrorBuffer_Destructor((*queue).data);
        rmtFree((*queue).data as *mut libc::c_void);
        (*queue).data = 0 as *mut VirtualMirrorBuffer;
    }
}
unsafe extern "C" fn rmtMessageQueue_SizeForPayload(mut payload_size: rmtU32) -> rmtU32 {
    let mut size: rmtU32 = (::std::mem::size_of::<Message>() as libc::c_ulong)
        .wrapping_add(payload_size as libc::c_ulong) as rmtU32;
    size = size.wrapping_add(3 as libc::c_int as libc::c_uint) & !(3 as libc::c_uint);
    return size;
}
unsafe extern "C" fn rmtMessageQueue_AllocMessage(
    mut queue: *mut rmtMessageQueue,
    mut payload_size: rmtU32,
    mut thread_profiler: *mut ThreadProfiler,
) -> *mut Message {
    let mut msg: *mut Message = 0 as *mut Message;
    let mut write_size: rmtU32 = rmtMessageQueue_SizeForPayload(payload_size);
    if !queue.is_null() {} else {
        __assert_fail(
            b"queue != NULL\0" as *const u8 as *const libc::c_char,
            b"lib/Remotery.c\0" as *const u8 as *const libc::c_char,
            4198 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 90],
                &[libc::c_char; 90],
            >(
                b"Message *rmtMessageQueue_AllocMessage(rmtMessageQueue *, rmtU32, struct ThreadProfiler *)\0",
            ))
                .as_ptr(),
        );
    }
    'c_1248: {
        if !queue.is_null() {} else {
            __assert_fail(
                b"queue != NULL\0" as *const u8 as *const libc::c_char,
                b"lib/Remotery.c\0" as *const u8 as *const libc::c_char,
                4198 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 90],
                    &[libc::c_char; 90],
                >(
                    b"Message *rmtMessageQueue_AllocMessage(rmtMessageQueue *, rmtU32, struct ThreadProfiler *)\0",
                ))
                    .as_ptr(),
            );
        }
    };
    loop {
        let mut s: rmtU32 = (*queue).size;
        let mut w: rmtU32 = LoadAcquire(&mut (*queue).write_pos);
        let mut r: rmtU32 = LoadAcquire(&mut (*queue).read_pos);
        if w.wrapping_sub(r) as libc::c_int > s.wrapping_sub(write_size) as libc::c_int {
            return 0 as *mut Message;
        }
        msg = ((*(*queue).data).ptr)
            .offset((w & s.wrapping_sub(1 as libc::c_int as libc::c_uint)) as isize)
            as *mut Message;
        if !(AtomicCompareAndSwapU32(
            &mut (*queue).write_pos,
            w as libc::c_long,
            w.wrapping_add(write_size) as libc::c_long,
        ) == 1 as libc::c_int as rmtBool)
        {
            continue;
        }
        (*msg).payload_size = payload_size;
        (*msg).threadProfiler = thread_profiler;
        break;
    }
    return msg;
}
unsafe extern "C" fn rmtMessageQueue_CommitMessage(
    mut message: *mut Message,
    mut id: MessageID,
) {
    if !message.is_null() {} else {
        __assert_fail(
            b"message != NULL\0" as *const u8 as *const libc::c_char,
            b"lib/Remotery.c\0" as *const u8 as *const libc::c_char,
            4229 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 57],
                &[libc::c_char; 57],
            >(b"void rmtMessageQueue_CommitMessage(Message *, MessageID)\0"))
                .as_ptr(),
        );
    }
    'c_1389: {
        if !message.is_null() {} else {
            __assert_fail(
                b"message != NULL\0" as *const u8 as *const libc::c_char,
                b"lib/Remotery.c\0" as *const u8 as *const libc::c_char,
                4229 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 57],
                    &[libc::c_char; 57],
                >(b"void rmtMessageQueue_CommitMessage(Message *, MessageID)\0"))
                    .as_ptr(),
            );
        }
    };
    if LoadAcquire(
        &mut (*message).id as *mut MessageID as *mut rmtU32 as *mut rmtAtomicU32,
    ) == MsgID_NotReady as libc::c_int as libc::c_uint
    {} else {
        __assert_fail(
            b"LoadAcquire((rmtU32*)&message->id) == MsgID_NotReady\0" as *const u8
                as *const libc::c_char,
            b"lib/Remotery.c\0" as *const u8 as *const libc::c_char,
            4232 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 57],
                &[libc::c_char; 57],
            >(b"void rmtMessageQueue_CommitMessage(Message *, MessageID)\0"))
                .as_ptr(),
        );
    }
    'c_1335: {
        if LoadAcquire(
            &mut (*message).id as *mut MessageID as *mut rmtU32 as *mut rmtAtomicU32,
        ) == MsgID_NotReady as libc::c_int as libc::c_uint
        {} else {
            __assert_fail(
                b"LoadAcquire((rmtU32*)&message->id) == MsgID_NotReady\0" as *const u8
                    as *const libc::c_char,
                b"lib/Remotery.c\0" as *const u8 as *const libc::c_char,
                4232 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 57],
                    &[libc::c_char; 57],
                >(b"void rmtMessageQueue_CommitMessage(Message *, MessageID)\0"))
                    .as_ptr(),
            );
        }
    };
    StoreRelease(
        &mut (*message).id as *mut MessageID as *mut rmtU32 as *mut rmtAtomicU32,
        id as rmtU32,
    );
}
pub unsafe extern "C" fn rmtMessageQueue_PeekNextMessage(
    mut queue: *mut rmtMessageQueue,
) -> *mut Message {
    let mut ptr: *mut Message = 0 as *mut Message;
    let mut r: rmtU32 = 0;
    let mut w: rmtU32 = 0;
    let mut id: MessageID = MsgID_NotReady;
    if !queue.is_null() {} else {
        __assert_fail(
            b"queue != NULL\0" as *const u8 as *const libc::c_char,
            b"lib/Remotery.c\0" as *const u8 as *const libc::c_char,
            4242 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 60],
                &[libc::c_char; 60],
            >(b"Message *rmtMessageQueue_PeekNextMessage(rmtMessageQueue *)\0"))
                .as_ptr(),
        );
    }
    'c_4148: {
        if !queue.is_null() {} else {
            __assert_fail(
                b"queue != NULL\0" as *const u8 as *const libc::c_char,
                b"lib/Remotery.c\0" as *const u8 as *const libc::c_char,
                4242 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 60],
                    &[libc::c_char; 60],
                >(b"Message *rmtMessageQueue_PeekNextMessage(rmtMessageQueue *)\0"))
                    .as_ptr(),
            );
        }
    };
    w = LoadAcquire(&mut (*queue).write_pos);
    r = (*queue).read_pos;
    if w.wrapping_sub(r) == 0 as libc::c_int as libc::c_uint {
        return 0 as *mut Message;
    }
    r = r & ((*queue).size).wrapping_sub(1 as libc::c_int as libc::c_uint);
    ptr = ((*(*queue).data).ptr).offset(r as isize) as *mut Message;
    id = LoadAcquire(
        &mut (*ptr).id as *mut MessageID as *mut rmtU32 as *mut rmtAtomicU32,
    ) as MessageID;
    if id as libc::c_uint != MsgID_NotReady as libc::c_int as libc::c_uint {
        return ptr;
    }
    return 0 as *mut Message;
}
unsafe extern "C" fn rmtMessageQueue_ConsumeNextMessage(
    mut queue: *mut rmtMessageQueue,
    mut message: *mut Message,
) {
    let mut message_size: rmtU32 = 0;
    let mut read_pos: rmtU32 = 0;
    if !queue.is_null() {} else {
        __assert_fail(
            b"queue != NULL\0" as *const u8 as *const libc::c_char,
            b"lib/Remotery.c\0" as *const u8 as *const libc::c_char,
            4266 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 70],
                &[libc::c_char; 70],
            >(
                b"void rmtMessageQueue_ConsumeNextMessage(rmtMessageQueue *, Message *)\0",
            ))
                .as_ptr(),
        );
    }
    'c_4296: {
        if !queue.is_null() {} else {
            __assert_fail(
                b"queue != NULL\0" as *const u8 as *const libc::c_char,
                b"lib/Remotery.c\0" as *const u8 as *const libc::c_char,
                4266 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 70],
                    &[libc::c_char; 70],
                >(
                    b"void rmtMessageQueue_ConsumeNextMessage(rmtMessageQueue *, Message *)\0",
                ))
                    .as_ptr(),
            );
        }
    };
    if !message.is_null() {} else {
        __assert_fail(
            b"message != NULL\0" as *const u8 as *const libc::c_char,
            b"lib/Remotery.c\0" as *const u8 as *const libc::c_char,
            4267 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 70],
                &[libc::c_char; 70],
            >(
                b"void rmtMessageQueue_ConsumeNextMessage(rmtMessageQueue *, Message *)\0",
            ))
                .as_ptr(),
        );
    }
    'c_4253: {
        if !message.is_null() {} else {
            __assert_fail(
                b"message != NULL\0" as *const u8 as *const libc::c_char,
                b"lib/Remotery.c\0" as *const u8 as *const libc::c_char,
                4267 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 70],
                    &[libc::c_char; 70],
                >(
                    b"void rmtMessageQueue_ConsumeNextMessage(rmtMessageQueue *, Message *)\0",
                ))
                    .as_ptr(),
            );
        }
    };
    message_size = rmtMessageQueue_SizeForPayload((*message).payload_size);
    memset(
        message as *mut libc::c_void,
        MsgID_NotReady as libc::c_int,
        message_size as libc::c_ulong,
    );
    read_pos = ((*queue).read_pos).wrapping_add(message_size);
    StoreRelease(&mut (*queue).read_pos, read_pos);
}
unsafe extern "C" fn Server_CreateListenSocket(
    mut server: *mut Server,
    mut port: rmtU16,
    mut reuse_open_port: rmtBool,
    mut limit_connections_to_localhost: rmtBool,
) -> rmtError {
    (*server)
        .listen_socket = rmtMalloc(
        ::std::mem::size_of::<WebSocket>() as libc::c_ulong as rmtU32,
    ) as *mut WebSocket;
    if ((*server).listen_socket).is_null() {
        return RMT_ERROR_MALLOC_FAIL;
    }
    let mut error: rmtError = WebSocket_Constructor(
        (*server).listen_socket,
        0 as *mut TCPSocket,
    );
    if error as libc::c_uint != RMT_ERROR_NONE as libc::c_int as libc::c_uint {
        if !((*server).listen_socket).is_null() {
            WebSocket_Destructor((*server).listen_socket);
            rmtFree((*server).listen_socket as *mut libc::c_void);
            (*server).listen_socket = 0 as *mut WebSocket;
        }
        return error;
    }
    let mut error_0: rmtError = WebSocket_RunServer(
        (*server).listen_socket,
        port,
        reuse_open_port,
        limit_connections_to_localhost,
        WEBSOCKET_BINARY,
    );
    if error_0 as libc::c_uint != RMT_ERROR_NONE as libc::c_int as libc::c_uint {
        return error_0;
    }
    return RMT_ERROR_NONE;
}
unsafe extern "C" fn Server_Constructor(
    mut server: *mut Server,
    mut port: rmtU16,
    mut reuse_open_port: rmtBool,
    mut limit_connections_to_localhost: rmtBool,
) -> rmtError {
    if !server.is_null() {} else {
        __assert_fail(
            b"server != NULL\0" as *const u8 as *const libc::c_char,
            b"lib/Remotery.c\0" as *const u8 as *const libc::c_char,
            4328 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 64],
                &[libc::c_char; 64],
            >(b"rmtError Server_Constructor(Server *, rmtU16, rmtBool, rmtBool)\0"))
                .as_ptr(),
        );
    }
    'c_28703: {
        if !server.is_null() {} else {
            __assert_fail(
                b"server != NULL\0" as *const u8 as *const libc::c_char,
                b"lib/Remotery.c\0" as *const u8 as *const libc::c_char,
                4328 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 64],
                    &[libc::c_char; 64],
                >(b"rmtError Server_Constructor(Server *, rmtU16, rmtBool, rmtBool)\0"))
                    .as_ptr(),
            );
        }
    };
    (*server).listen_socket = 0 as *mut WebSocket;
    (*server).client_socket = 0 as *mut WebSocket;
    (*server).last_ping_time = 0 as libc::c_int as rmtU32;
    (*server).port = port;
    (*server).reuse_open_port = reuse_open_port;
    (*server).limit_connections_to_localhost = limit_connections_to_localhost;
    (*server).bin_buf = 0 as *mut Buffer;
    (*server).receive_handler = None;
    (*server).receive_handler_context = 0 as *mut libc::c_void;
    (*server)
        .bin_buf = rmtMalloc(::std::mem::size_of::<Buffer>() as libc::c_ulong as rmtU32)
        as *mut Buffer;
    if ((*server).bin_buf).is_null() {
        return RMT_ERROR_MALLOC_FAIL;
    }
    let mut error: rmtError = Buffer_Constructor(
        (*server).bin_buf,
        4096 as libc::c_int as rmtU32,
    );
    if error as libc::c_uint != RMT_ERROR_NONE as libc::c_int as libc::c_uint {
        if !((*server).bin_buf).is_null() {
            Buffer_Destructor((*server).bin_buf);
            rmtFree((*server).bin_buf as *mut libc::c_void);
            (*server).bin_buf = 0 as *mut Buffer;
        }
        return error;
    }
    return Server_CreateListenSocket(
        server,
        port,
        reuse_open_port,
        limit_connections_to_localhost,
    );
}
unsafe extern "C" fn Server_Destructor(mut server: *mut Server) {
    if !server.is_null() {} else {
        __assert_fail(
            b"server != NULL\0" as *const u8 as *const libc::c_char,
            b"lib/Remotery.c\0" as *const u8 as *const libc::c_char,
            4348 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 33],
                &[libc::c_char; 33],
            >(b"void Server_Destructor(Server *)\0"))
                .as_ptr(),
        );
    }
    'c_28891: {
        if !server.is_null() {} else {
            __assert_fail(
                b"server != NULL\0" as *const u8 as *const libc::c_char,
                b"lib/Remotery.c\0" as *const u8 as *const libc::c_char,
                4348 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 33],
                    &[libc::c_char; 33],
                >(b"void Server_Destructor(Server *)\0"))
                    .as_ptr(),
            );
        }
    };
    if !((*server).client_socket).is_null() {
        WebSocket_Destructor((*server).client_socket);
        rmtFree((*server).client_socket as *mut libc::c_void);
        (*server).client_socket = 0 as *mut WebSocket;
    }
    if !((*server).listen_socket).is_null() {
        WebSocket_Destructor((*server).listen_socket);
        rmtFree((*server).listen_socket as *mut libc::c_void);
        (*server).listen_socket = 0 as *mut WebSocket;
    }
    if !((*server).bin_buf).is_null() {
        Buffer_Destructor((*server).bin_buf);
        rmtFree((*server).bin_buf as *mut libc::c_void);
        (*server).bin_buf = 0 as *mut Buffer;
    }
}
unsafe extern "C" fn Server_IsClientConnected(mut server: *mut Server) -> rmtBool {
    if !server.is_null() {} else {
        __assert_fail(
            b"server != NULL\0" as *const u8 as *const libc::c_char,
            b"lib/Remotery.c\0" as *const u8 as *const libc::c_char,
            4356 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 43],
                &[libc::c_char; 43],
            >(b"rmtBool Server_IsClientConnected(Server *)\0"))
                .as_ptr(),
        );
    }
    'c_10847: {
        if !server.is_null() {} else {
            __assert_fail(
                b"server != NULL\0" as *const u8 as *const libc::c_char,
                b"lib/Remotery.c\0" as *const u8 as *const libc::c_char,
                4356 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 43],
                    &[libc::c_char; 43],
                >(b"rmtBool Server_IsClientConnected(Server *)\0"))
                    .as_ptr(),
            );
        }
    };
    return if !((*server).client_socket).is_null() {
        1 as libc::c_int as rmtBool
    } else {
        0 as libc::c_int as rmtBool
    };
}
unsafe extern "C" fn Server_DisconnectClient(mut server: *mut Server) {
    let mut client_socket: *mut WebSocket = 0 as *mut WebSocket;
    if !server.is_null() {} else {
        __assert_fail(
            b"server != NULL\0" as *const u8 as *const libc::c_char,
            b"lib/Remotery.c\0" as *const u8 as *const libc::c_char,
            4364 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 39],
                &[libc::c_char; 39],
            >(b"void Server_DisconnectClient(Server *)\0"))
                .as_ptr(),
        );
    }
    'c_10773: {
        if !server.is_null() {} else {
            __assert_fail(
                b"server != NULL\0" as *const u8 as *const libc::c_char,
                b"lib/Remotery.c\0" as *const u8 as *const libc::c_char,
                4364 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 39],
                    &[libc::c_char; 39],
                >(b"void Server_DisconnectClient(Server *)\0"))
                    .as_ptr(),
            );
        }
    };
    client_socket = (*server).client_socket;
    (*server).client_socket = 0 as *mut WebSocket;
    CompilerWriteFence();
    if !client_socket.is_null() {
        WebSocket_Destructor(client_socket);
        rmtFree(client_socket as *mut libc::c_void);
        client_socket = 0 as *mut WebSocket;
    }
}
unsafe extern "C" fn Server_Send(
    mut server: *mut Server,
    mut data: *const libc::c_void,
    mut length: rmtU32,
    mut timeout: rmtU32,
) -> rmtError {
    if !server.is_null() {} else {
        __assert_fail(
            b"server != NULL\0" as *const u8 as *const libc::c_char,
            b"lib/Remotery.c\0" as *const u8 as *const libc::c_char,
            4375 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 61],
                &[libc::c_char; 61],
            >(b"rmtError Server_Send(Server *, const void *, rmtU32, rmtU32)\0"))
                .as_ptr(),
        );
    }
    'c_10890: {
        if !server.is_null() {} else {
            __assert_fail(
                b"server != NULL\0" as *const u8 as *const libc::c_char,
                b"lib/Remotery.c\0" as *const u8 as *const libc::c_char,
                4375 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 61],
                    &[libc::c_char; 61],
                >(b"rmtError Server_Send(Server *, const void *, rmtU32, rmtU32)\0"))
                    .as_ptr(),
            );
        }
    };
    if Server_IsClientConnected(server) != 0 {
        let mut error: rmtError = WebSocket_Send(
            (*server).client_socket,
            data,
            length,
            timeout,
        );
        if error as libc::c_uint
            == RMT_ERROR_SOCKET_SEND_FAIL as libc::c_int as libc::c_uint
        {
            Server_DisconnectClient(server);
        }
        return error;
    }
    return RMT_ERROR_NONE;
}
unsafe extern "C" fn Server_ReceiveMessage(
    mut server: *mut Server,
    mut message_first_byte: libc::c_char,
    mut message_length: rmtU32,
) -> rmtError {
    let mut message_data: [libc::c_char; 1024] = [0; 1024];
    if message_length as libc::c_ulong
        >= (::std::mem::size_of::<[libc::c_char; 1024]>() as libc::c_ulong)
            .wrapping_sub(1 as libc::c_int as libc::c_ulong)
    {
        _rmt_LogText(
            b"Ignoring console input bigger than internal receive buffer (1024 bytes)\0"
                as *const u8 as *const libc::c_char,
        );
        return RMT_ERROR_NONE;
    }
    message_data[0 as libc::c_int as usize] = message_first_byte;
    let mut error: rmtError = WebSocket_Receive(
        (*server).client_socket,
        message_data.as_mut_ptr().offset(1 as libc::c_int as isize) as *mut libc::c_void,
        0 as *mut rmtU32,
        message_length.wrapping_sub(1 as libc::c_int as libc::c_uint),
        100 as libc::c_int as rmtU32,
    );
    if error as libc::c_uint != RMT_ERROR_NONE as libc::c_int as libc::c_uint {
        return error;
    }
    message_data[message_length as usize] = 0 as libc::c_int as libc::c_char;
    if message_length < 4 as libc::c_int as libc::c_uint {
        return RMT_ERROR_NONE;
    }
    if ((*server).receive_handler).is_some() {
        let mut error_0: rmtError = ((*server).receive_handler)
            .unwrap()(
            (*server).receive_handler_context,
            message_data.as_mut_ptr(),
            message_length,
        );
        if error_0 as libc::c_uint != RMT_ERROR_NONE as libc::c_int as libc::c_uint {
            return error_0;
        }
    }
    return RMT_ERROR_NONE;
}
unsafe extern "C" fn bin_MessageHeader(
    mut buffer: *mut Buffer,
    mut id: *const libc::c_char,
    mut out_write_start_offset: *mut rmtU32,
) -> rmtError {
    *out_write_start_offset = (*buffer).bytes_used;
    let mut error: rmtError = Buffer_Write(
        buffer,
        id as *mut libc::c_void,
        4 as libc::c_int as rmtU32,
    );
    if error as libc::c_uint != RMT_ERROR_NONE as libc::c_int as libc::c_uint {
        return error;
    }
    let mut error_0: rmtError = Buffer_Write(
        buffer,
        b"    \0" as *const u8 as *const libc::c_char as *mut libc::c_void,
        4 as libc::c_int as rmtU32,
    );
    if error_0 as libc::c_uint != RMT_ERROR_NONE as libc::c_int as libc::c_uint {
        return error_0;
    }
    return RMT_ERROR_NONE;
}
unsafe extern "C" fn bin_MessageFooter(
    mut buffer: *mut Buffer,
    mut write_start_offset: rmtU32,
) -> rmtError {
    let mut error: rmtError = Buffer_AlignedPad(buffer, write_start_offset);
    if error as libc::c_uint != RMT_ERROR_NONE as libc::c_int as libc::c_uint {
        return error;
    }
    U32ToByteArray(
        ((*buffer).data)
            .offset(write_start_offset as isize)
            .offset(4 as libc::c_int as isize),
        ((*buffer).bytes_used).wrapping_sub(write_start_offset),
    );
    return RMT_ERROR_NONE;
}
unsafe extern "C" fn Server_Update(mut server: *mut Server) {
    let mut cur_time: rmtU32 = 0;
    if !server.is_null() {} else {
        __assert_fail(
            b"server != NULL\0" as *const u8 as *const libc::c_char,
            b"lib/Remotery.c\0" as *const u8 as *const libc::c_char,
            4439 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 29],
                &[libc::c_char; 29],
            >(b"void Server_Update(Server *)\0"))
                .as_ptr(),
        );
    }
    'c_22328: {
        if !server.is_null() {} else {
            __assert_fail(
                b"server != NULL\0" as *const u8 as *const libc::c_char,
                b"lib/Remotery.c\0" as *const u8 as *const libc::c_char,
                4439 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 29],
                    &[libc::c_char; 29],
                >(b"void Server_Update(Server *)\0"))
                    .as_ptr(),
            );
        }
    };
    if ((*server).listen_socket).is_null() {
        Server_CreateListenSocket(
            server,
            (*server).port,
            (*server).reuse_open_port,
            (*server).limit_connections_to_localhost,
        );
    }
    if !((*server).listen_socket).is_null() && ((*server).client_socket).is_null() {
        let mut client_socket: *mut WebSocket = 0 as *mut WebSocket;
        let mut error: rmtError = WebSocket_AcceptConnection(
            (*server).listen_socket,
            &mut client_socket,
        );
        if error as libc::c_uint == RMT_ERROR_NONE as libc::c_int as libc::c_uint {
            (*server).client_socket = client_socket;
        } else if !((*server).listen_socket).is_null() {
            WebSocket_Destructor((*server).listen_socket);
            rmtFree((*server).listen_socket as *mut libc::c_void);
            (*server).listen_socket = 0 as *mut WebSocket;
        }
    } else {
        loop {
            let mut message_first_byte: libc::c_char = 0;
            let mut message_length: rmtU32 = 0;
            let mut error_0: rmtError = WebSocket_Receive(
                (*server).client_socket,
                &mut message_first_byte as *mut libc::c_char as *mut libc::c_void,
                &mut message_length,
                1 as libc::c_int as rmtU32,
                0 as libc::c_int as rmtU32,
            );
            if error_0 as libc::c_uint == RMT_ERROR_NONE as libc::c_int as libc::c_uint {
                error_0 = Server_ReceiveMessage(
                    server,
                    message_first_byte,
                    message_length,
                );
                if error_0 as libc::c_uint
                    != RMT_ERROR_NONE as libc::c_int as libc::c_uint
                {
                    Server_DisconnectClient(server);
                    break;
                }
            } else if error_0 as libc::c_uint
                == RMT_ERROR_SOCKET_RECV_NO_DATA as libc::c_int as libc::c_uint
            {
                break;
            } else {
                if error_0 as libc::c_uint
                    == RMT_ERROR_SOCKET_RECV_TIMEOUT as libc::c_int as libc::c_uint
                {
                    break;
                }
                Server_DisconnectClient(server);
                break;
            }
        }
    }
    cur_time = msTimer_Get();
    if cur_time.wrapping_sub((*server).last_ping_time)
        > 1000 as libc::c_int as libc::c_uint
    {
        let mut bin_buf: *mut Buffer = (*server).bin_buf;
        let mut write_start_offset: rmtU32 = 0;
        WebSocket_PrepareBuffer(bin_buf);
        bin_MessageHeader(
            bin_buf,
            b"PING\0" as *const u8 as *const libc::c_char,
            &mut write_start_offset,
        );
        bin_MessageFooter(bin_buf, write_start_offset);
        Server_Send(
            server,
            (*bin_buf).data as *const libc::c_void,
            (*bin_buf).bytes_used,
            10 as libc::c_int as rmtU32,
        );
        (*server).last_ping_time = cur_time;
    }
}
unsafe extern "C" fn Sample_Constructor(mut sample: *mut Sample) -> rmtError {
    if !sample.is_null() {} else {
        __assert_fail(
            b"sample != NULL\0" as *const u8 as *const libc::c_char,
            b"lib/Remotery.c\0" as *const u8 as *const libc::c_char,
            4577 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 38],
                &[libc::c_char; 38],
            >(b"rmtError Sample_Constructor(Sample *)\0"))
                .as_ptr(),
        );
    }
    'c_7275: {
        if !sample.is_null() {} else {
            __assert_fail(
                b"sample != NULL\0" as *const u8 as *const libc::c_char,
                b"lib/Remotery.c\0" as *const u8 as *const libc::c_char,
                4577 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 38],
                    &[libc::c_char; 38],
                >(b"rmtError Sample_Constructor(Sample *)\0"))
                    .as_ptr(),
            );
        }
    };
    ObjectLink_Constructor(sample as *mut ObjectLink);
    (*sample).type_0 = RMT_SampleType_CPU;
    (*sample).name_hash = 0 as libc::c_int as rmtU32;
    (*sample).unique_id = 0 as libc::c_int as rmtU32;
    (*sample).uniqueColour[0 as libc::c_int as usize] = 0 as libc::c_int as rmtU8;
    (*sample).uniqueColour[1 as libc::c_int as usize] = 0 as libc::c_int as rmtU8;
    (*sample).uniqueColour[2 as libc::c_int as usize] = 0 as libc::c_int as rmtU8;
    (*sample).parent = 0 as *mut Sample;
    (*sample).first_child = 0 as *mut Sample;
    (*sample).last_child = 0 as *mut Sample;
    (*sample).next_sibling = 0 as *mut Sample;
    (*sample).nb_children = 0 as libc::c_int as rmtU32;
    (*sample).us_start = 0 as libc::c_int as rmtU64;
    (*sample).us_end = 0 as libc::c_int as rmtU64;
    (*sample).us_length = 0 as libc::c_int as rmtU64;
    (*sample).us_sampled_length = 0 as libc::c_int as rmtU64;
    (*sample).usGpuIssueOnCpu = 0 as libc::c_int as rmtU64;
    (*sample).call_count = 0 as libc::c_int as rmtU32;
    (*sample).recurse_depth = 0 as libc::c_int as rmtU16;
    (*sample).max_recurse_depth = 0 as libc::c_int as rmtU16;
    return RMT_ERROR_NONE;
}
unsafe extern "C" fn Sample_Destructor(mut sample: *mut Sample) {
    if 1 as libc::c_int != 0 {} else {};
}
unsafe extern "C" fn Sample_Prepare(
    mut sample: *mut Sample,
    mut name_hash: rmtU32,
    mut parent: *mut Sample,
) {
    (*sample).name_hash = name_hash;
    (*sample).unique_id = 0 as libc::c_int as rmtU32;
    (*sample).parent = parent;
    (*sample).first_child = 0 as *mut Sample;
    (*sample).last_child = 0 as *mut Sample;
    (*sample).next_sibling = 0 as *mut Sample;
    (*sample).nb_children = 0 as libc::c_int as rmtU32;
    (*sample).us_start = 0 as libc::c_int as rmtU64;
    (*sample).us_end = 0 as libc::c_int as rmtU64;
    (*sample).us_length = 0 as libc::c_int as rmtU64;
    (*sample).us_sampled_length = 0 as libc::c_int as rmtU64;
    (*sample).usGpuIssueOnCpu = 0 as libc::c_int as rmtU64;
    (*sample).call_count = 1 as libc::c_int as rmtU32;
    (*sample).recurse_depth = 0 as libc::c_int as rmtU16;
    (*sample).max_recurse_depth = 0 as libc::c_int as rmtU16;
}
unsafe extern "C" fn Sample_Close(mut sample: *mut Sample, mut us_end: rmtS64) {
    let mut us_length: rmtS64 = 0 as libc::c_int as rmtS64;
    if (*sample).call_count > 1 as libc::c_int as libc::c_uint
        && (*sample).max_recurse_depth as libc::c_int == 0 as libc::c_int
    {
        us_length = maxS64(
            (us_end as libc::c_ulonglong).wrapping_sub((*sample).us_end) as rmtS64,
            0 as libc::c_int as rmtS64,
        );
    } else {
        us_length = maxS64(
            (us_end as libc::c_ulonglong).wrapping_sub((*sample).us_start) as rmtS64,
            0 as libc::c_int as rmtS64,
        );
    }
    (*sample)
        .us_length = ((*sample).us_length as libc::c_ulonglong)
        .wrapping_add(us_length as libc::c_ulonglong) as rmtU64 as rmtU64;
    if !((*sample).parent).is_null() {
        (*(*sample).parent)
            .us_sampled_length = ((*(*sample).parent).us_sampled_length
            as libc::c_ulonglong)
            .wrapping_add(us_length as libc::c_ulonglong) as rmtU64 as rmtU64;
    }
}
unsafe extern "C" fn Sample_CopyState(
    mut dst_sample: *mut Sample,
    mut src_sample: *const Sample,
) {
    (*dst_sample).type_0 = (*src_sample).type_0;
    (*dst_sample).name_hash = (*src_sample).name_hash;
    (*dst_sample).unique_id = (*src_sample).unique_id;
    (*dst_sample).nb_children = (*src_sample).nb_children;
    (*dst_sample).us_start = (*src_sample).us_start;
    (*dst_sample).us_end = (*src_sample).us_end;
    (*dst_sample).us_length = (*src_sample).us_length;
    (*dst_sample).us_sampled_length = (*src_sample).us_sampled_length;
    (*dst_sample).usGpuIssueOnCpu = (*src_sample).usGpuIssueOnCpu;
    (*dst_sample).call_count = (*src_sample).call_count;
    (*dst_sample).recurse_depth = (*src_sample).recurse_depth;
    (*dst_sample).max_recurse_depth = (*src_sample).max_recurse_depth;
    (*dst_sample).parent = 0 as *mut Sample;
    (*dst_sample).first_child = 0 as *mut Sample;
    (*dst_sample).last_child = 0 as *mut Sample;
    (*dst_sample).next_sibling = 0 as *mut Sample;
}
unsafe extern "C" fn bin_Sample(
    mut buffer: *mut Buffer,
    mut sample: *mut Sample,
    mut depth: rmtU8,
) -> rmtError {
    if !sample.is_null() {} else {
        __assert_fail(
            b"sample != NULL\0" as *const u8 as *const libc::c_char,
            b"lib/Remotery.c\0" as *const u8 as *const libc::c_char,
            4678 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 47],
                &[libc::c_char; 47],
            >(b"rmtError bin_Sample(Buffer *, Sample *, rmtU8)\0"))
                .as_ptr(),
        );
    }
    'c_14625: {
        if !sample.is_null() {} else {
            __assert_fail(
                b"sample != NULL\0" as *const u8 as *const libc::c_char,
                b"lib/Remotery.c\0" as *const u8 as *const libc::c_char,
                4678 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 47],
                    &[libc::c_char; 47],
                >(b"rmtError bin_Sample(Buffer *, Sample *, rmtU8)\0"))
                    .as_ptr(),
            );
        }
    };
    let mut error: rmtError = Buffer_WriteU32(buffer, (*sample).name_hash);
    if error as libc::c_uint != RMT_ERROR_NONE as libc::c_int as libc::c_uint {
        return error;
    }
    let mut error_0: rmtError = Buffer_WriteU32(buffer, (*sample).unique_id);
    if error_0 as libc::c_uint != RMT_ERROR_NONE as libc::c_int as libc::c_uint {
        return error_0;
    }
    let mut error_1: rmtError = Buffer_Write(
        buffer,
        ((*sample).uniqueColour).as_mut_ptr() as *const libc::c_void,
        3 as libc::c_int as rmtU32,
    );
    if error_1 as libc::c_uint != RMT_ERROR_NONE as libc::c_int as libc::c_uint {
        return error_1;
    }
    let mut error_2: rmtError = Buffer_Write(
        buffer,
        &mut depth as *mut rmtU8 as *const libc::c_void,
        1 as libc::c_int as rmtU32,
    );
    if error_2 as libc::c_uint != RMT_ERROR_NONE as libc::c_int as libc::c_uint {
        return error_2;
    }
    let mut error_3: rmtError = Buffer_WriteU64(buffer, (*sample).us_start);
    if error_3 as libc::c_uint != RMT_ERROR_NONE as libc::c_int as libc::c_uint {
        return error_3;
    }
    let mut error_4: rmtError = Buffer_WriteU64(buffer, (*sample).us_length);
    if error_4 as libc::c_uint != RMT_ERROR_NONE as libc::c_int as libc::c_uint {
        return error_4;
    }
    let mut error_5: rmtError = Buffer_WriteU64(
        buffer,
        maxS64(
            ((*sample).us_length).wrapping_sub((*sample).us_sampled_length) as rmtS64,
            0 as libc::c_int as rmtS64,
        ) as rmtU64,
    );
    if error_5 as libc::c_uint != RMT_ERROR_NONE as libc::c_int as libc::c_uint {
        return error_5;
    }
    let mut error_6: rmtError = Buffer_WriteU64(buffer, (*sample).usGpuIssueOnCpu);
    if error_6 as libc::c_uint != RMT_ERROR_NONE as libc::c_int as libc::c_uint {
        return error_6;
    }
    let mut error_7: rmtError = Buffer_WriteU32(buffer, (*sample).call_count);
    if error_7 as libc::c_uint != RMT_ERROR_NONE as libc::c_int as libc::c_uint {
        return error_7;
    }
    let mut error_8: rmtError = Buffer_WriteU32(
        buffer,
        (*sample).max_recurse_depth as rmtU32,
    );
    if error_8 as libc::c_uint != RMT_ERROR_NONE as libc::c_int as libc::c_uint {
        return error_8;
    }
    let mut error_9: rmtError = bin_SampleArray(
        buffer,
        sample,
        (depth as libc::c_int + 1 as libc::c_int) as rmtU8,
    );
    if error_9 as libc::c_uint != RMT_ERROR_NONE as libc::c_int as libc::c_uint {
        return error_9;
    }
    return RMT_ERROR_NONE;
}
unsafe extern "C" fn bin_SampleArray(
    mut buffer: *mut Buffer,
    mut parent_sample: *mut Sample,
    mut depth: rmtU8,
) -> rmtError {
    let mut sample: *mut Sample = 0 as *mut Sample;
    let mut error: rmtError = Buffer_WriteU32(buffer, (*parent_sample).nb_children);
    if error as libc::c_uint != RMT_ERROR_NONE as libc::c_int as libc::c_uint {
        return error;
    }
    sample = (*parent_sample).first_child;
    while !sample.is_null() {
        let mut error_0: rmtError = bin_Sample(buffer, sample, depth);
        if error_0 as libc::c_uint != RMT_ERROR_NONE as libc::c_int as libc::c_uint {
            return error_0;
        }
        sample = (*sample).next_sibling;
    }
    return RMT_ERROR_NONE;
}
unsafe extern "C" fn SampleTree_Constructor(
    mut tree: *mut SampleTree,
    mut sample_size: rmtU32,
    mut constructor: ObjConstructor,
    mut destructor: ObjDestructor,
) -> rmtError {
    if !tree.is_null() {} else {
        __assert_fail(
            b"tree != NULL\0" as *const u8 as *const libc::c_char,
            b"lib/Remotery.c\0" as *const u8 as *const libc::c_char,
            4745 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 85],
                &[libc::c_char; 85],
            >(
                b"rmtError SampleTree_Constructor(SampleTree *, rmtU32, ObjConstructor, ObjDestructor)\0",
            ))
                .as_ptr(),
        );
    }
    'c_7710: {
        if !tree.is_null() {} else {
            __assert_fail(
                b"tree != NULL\0" as *const u8 as *const libc::c_char,
                b"lib/Remotery.c\0" as *const u8 as *const libc::c_char,
                4745 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 85],
                    &[libc::c_char; 85],
                >(
                    b"rmtError SampleTree_Constructor(SampleTree *, rmtU32, ObjConstructor, ObjDestructor)\0",
                ))
                    .as_ptr(),
            );
        }
    };
    (*tree).allocator = 0 as *mut ObjectAllocator;
    (*tree).root = 0 as *mut Sample;
    (*tree).currentParent = 0 as *mut Sample;
    StoreRelease(&mut (*tree).msLastTreeSendTime, 0 as libc::c_int as rmtU32);
    StoreRelease(&mut (*tree).treeBeingModified, 0 as libc::c_int as rmtU32);
    (*tree).sendSampleOnClose = 0 as *mut Sample;
    (*tree)
        .allocator = rmtMalloc(
        ::std::mem::size_of::<ObjectAllocator>() as libc::c_ulong as rmtU32,
    ) as *mut ObjectAllocator;
    if ((*tree).allocator).is_null() {
        return RMT_ERROR_MALLOC_FAIL;
    }
    let mut error: rmtError = ObjectAllocator_Constructor(
        (*tree).allocator,
        sample_size,
        constructor,
        destructor,
    );
    if error as libc::c_uint != RMT_ERROR_NONE as libc::c_int as libc::c_uint {
        if !((*tree).allocator).is_null() {
            ObjectAllocator_Destructor((*tree).allocator);
            rmtFree((*tree).allocator as *mut libc::c_void);
            (*tree).allocator = 0 as *mut ObjectAllocator;
        }
        return error;
    }
    let mut error_0: rmtError = ObjectAllocator_Alloc(
        (*tree).allocator,
        &mut (*tree).root as *mut *mut Sample as *mut *mut libc::c_void,
    );
    if error_0 as libc::c_uint != RMT_ERROR_NONE as libc::c_int as libc::c_uint {
        return error_0;
    }
    Sample_Prepare((*tree).root, 0 as libc::c_int as rmtU32, 0 as *mut Sample);
    (*tree).currentParent = (*tree).root;
    return RMT_ERROR_NONE;
}
unsafe extern "C" fn SampleTree_Destructor(mut tree: *mut SampleTree) {
    if !tree.is_null() {} else {
        __assert_fail(
            b"tree != NULL\0" as *const u8 as *const libc::c_char,
            b"lib/Remotery.c\0" as *const u8 as *const libc::c_char,
            4767 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 41],
                &[libc::c_char; 41],
            >(b"void SampleTree_Destructor(SampleTree *)\0"))
                .as_ptr(),
        );
    }
    'c_6908: {
        if !tree.is_null() {} else {
            __assert_fail(
                b"tree != NULL\0" as *const u8 as *const libc::c_char,
                b"lib/Remotery.c\0" as *const u8 as *const libc::c_char,
                4767 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 41],
                    &[libc::c_char; 41],
                >(b"void SampleTree_Destructor(SampleTree *)\0"))
                    .as_ptr(),
            );
        }
    };
    if !((*tree).root).is_null() {
        ObjectAllocator_Free((*tree).allocator, (*tree).root as *mut libc::c_void);
        (*tree).root = 0 as *mut Sample;
    }
    if !((*tree).allocator).is_null() {
        ObjectAllocator_Destructor((*tree).allocator);
        rmtFree((*tree).allocator as *mut libc::c_void);
        (*tree).allocator = 0 as *mut ObjectAllocator;
    }
}
unsafe extern "C" fn HashCombine(mut hash_a: rmtU32, mut hash_b: rmtU32) -> rmtU32 {
    static mut random_bits: rmtU32 = 0x9e3779b9 as libc::c_uint;
    hash_a
        ^= hash_b
            .wrapping_add(random_bits)
            .wrapping_add(hash_a << 6 as libc::c_int)
            .wrapping_add(hash_a >> 2 as libc::c_int);
    return hash_a;
}
unsafe extern "C" fn SampleTree_Push(
    mut tree: *mut SampleTree,
    mut name_hash: rmtU32,
    mut flags: rmtU32,
    mut sample: *mut *mut Sample,
) -> rmtError {
    let mut parent: *mut Sample = 0 as *mut Sample;
    let mut unique_id: rmtU32 = 0;
    if !tree.is_null() {} else {
        __assert_fail(
            b"tree != NULL\0" as *const u8 as *const libc::c_char,
            b"lib/Remotery.c\0" as *const u8 as *const libc::c_char,
            4795 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 66],
                &[libc::c_char; 66],
            >(b"rmtError SampleTree_Push(SampleTree *, rmtU32, rmtU32, Sample **)\0"))
                .as_ptr(),
        );
    }
    'c_11864: {
        if !tree.is_null() {} else {
            __assert_fail(
                b"tree != NULL\0" as *const u8 as *const libc::c_char,
                b"lib/Remotery.c\0" as *const u8 as *const libc::c_char,
                4795 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 66],
                    &[libc::c_char; 66],
                >(
                    b"rmtError SampleTree_Push(SampleTree *, rmtU32, rmtU32, Sample **)\0",
                ))
                    .as_ptr(),
            );
        }
    };
    if !((*tree).currentParent).is_null() {} else {
        __assert_fail(
            b"tree->currentParent != NULL\0" as *const u8 as *const libc::c_char,
            b"lib/Remotery.c\0" as *const u8 as *const libc::c_char,
            4796 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 66],
                &[libc::c_char; 66],
            >(b"rmtError SampleTree_Push(SampleTree *, rmtU32, rmtU32, Sample **)\0"))
                .as_ptr(),
        );
    }
    'c_11818: {
        if !((*tree).currentParent).is_null() {} else {
            __assert_fail(
                b"tree->currentParent != NULL\0" as *const u8 as *const libc::c_char,
                b"lib/Remotery.c\0" as *const u8 as *const libc::c_char,
                4796 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 66],
                    &[libc::c_char; 66],
                >(
                    b"rmtError SampleTree_Push(SampleTree *, rmtU32, rmtU32, Sample **)\0",
                ))
                    .as_ptr(),
            );
        }
    };
    parent = (*tree).currentParent;
    if flags != 0 as libc::c_int as libc::c_uint {
        if flags & RMTSF_Root as libc::c_int as libc::c_uint
            != 0 as libc::c_int as libc::c_uint
        {
            if ((*parent).parent).is_null() {} else {
                __assert_fail(
                    b"parent->parent == NULL\0" as *const u8 as *const libc::c_char,
                    b"lib/Remotery.c\0" as *const u8 as *const libc::c_char,
                    4805 as libc::c_int as libc::c_uint,
                    (*::std::mem::transmute::<
                        &[u8; 66],
                        &[libc::c_char; 66],
                    >(
                        b"rmtError SampleTree_Push(SampleTree *, rmtU32, rmtU32, Sample **)\0",
                    ))
                        .as_ptr(),
                );
            }
            'c_11753: {
                if ((*parent).parent).is_null() {} else {
                    __assert_fail(
                        b"parent->parent == NULL\0" as *const u8 as *const libc::c_char,
                        b"lib/Remotery.c\0" as *const u8 as *const libc::c_char,
                        4805 as libc::c_int as libc::c_uint,
                        (*::std::mem::transmute::<
                            &[u8; 66],
                            &[libc::c_char; 66],
                        >(
                            b"rmtError SampleTree_Push(SampleTree *, rmtU32, rmtU32, Sample **)\0",
                        ))
                            .as_ptr(),
                    );
                }
            };
        }
        if flags & RMTSF_Aggregate as libc::c_int as libc::c_uint
            != 0 as libc::c_int as libc::c_uint
        {
            let mut sibling: *mut Sample = 0 as *mut Sample;
            sibling = (*parent).first_child;
            while !sibling.is_null() {
                if (*sibling).name_hash == name_hash {
                    (*tree).currentParent = sibling;
                    (*sibling).call_count = ((*sibling).call_count).wrapping_add(1);
                    (*sibling).call_count;
                    *sample = sibling;
                    return RMT_ERROR_NONE;
                }
                sibling = (*sibling).next_sibling;
            }
        }
        if flags & RMTSF_Recursive as libc::c_int as libc::c_uint
            != 0 as libc::c_int as libc::c_uint && (*parent).name_hash == name_hash
        {
            (*parent).recurse_depth = ((*parent).recurse_depth).wrapping_add(1);
            (*parent).recurse_depth;
            (*parent)
                .max_recurse_depth = maxU16(
                (*parent).max_recurse_depth,
                (*parent).recurse_depth,
            );
            (*parent).call_count = ((*parent).call_count).wrapping_add(1);
            (*parent).call_count;
            *sample = parent;
            return RMT_ERROR_RECURSIVE_SAMPLE;
        }
        let mut error: rmtError = ObjectAllocator_Alloc(
            (*tree).allocator,
            sample as *mut *mut libc::c_void,
        );
        if error as libc::c_uint != RMT_ERROR_NONE as libc::c_int as libc::c_uint {
            return error;
        }
        Sample_Prepare(*sample, name_hash, parent);
        if flags & RMTSF_SendOnClose as libc::c_int as libc::c_uint
            != 0 as libc::c_int as libc::c_uint
        {
            if !((*tree).currentParent).is_null() {} else {
                __assert_fail(
                    b"tree->currentParent != NULL\0" as *const u8 as *const libc::c_char,
                    b"lib/Remotery.c\0" as *const u8 as *const libc::c_char,
                    4841 as libc::c_int as libc::c_uint,
                    (*::std::mem::transmute::<
                        &[u8; 66],
                        &[libc::c_char; 66],
                    >(
                        b"rmtError SampleTree_Push(SampleTree *, rmtU32, rmtU32, Sample **)\0",
                    ))
                        .as_ptr(),
                );
            }
            'c_11522: {
                if !((*tree).currentParent).is_null() {} else {
                    __assert_fail(
                        b"tree->currentParent != NULL\0" as *const u8
                            as *const libc::c_char,
                        b"lib/Remotery.c\0" as *const u8 as *const libc::c_char,
                        4841 as libc::c_int as libc::c_uint,
                        (*::std::mem::transmute::<
                            &[u8; 66],
                            &[libc::c_char; 66],
                        >(
                            b"rmtError SampleTree_Push(SampleTree *, rmtU32, rmtU32, Sample **)\0",
                        ))
                            .as_ptr(),
                    );
                }
            };
            if ((*tree).sendSampleOnClose).is_null() {} else {
                __assert_fail(
                    b"tree->sendSampleOnClose == NULL\0" as *const u8
                        as *const libc::c_char,
                    b"lib/Remotery.c\0" as *const u8 as *const libc::c_char,
                    4842 as libc::c_int as libc::c_uint,
                    (*::std::mem::transmute::<
                        &[u8; 66],
                        &[libc::c_char; 66],
                    >(
                        b"rmtError SampleTree_Push(SampleTree *, rmtU32, rmtU32, Sample **)\0",
                    ))
                        .as_ptr(),
                );
            }
            'c_11475: {
                if ((*tree).sendSampleOnClose).is_null() {} else {
                    __assert_fail(
                        b"tree->sendSampleOnClose == NULL\0" as *const u8
                            as *const libc::c_char,
                        b"lib/Remotery.c\0" as *const u8 as *const libc::c_char,
                        4842 as libc::c_int as libc::c_uint,
                        (*::std::mem::transmute::<
                            &[u8; 66],
                            &[libc::c_char; 66],
                        >(
                            b"rmtError SampleTree_Push(SampleTree *, rmtU32, rmtU32, Sample **)\0",
                        ))
                            .as_ptr(),
                    );
                }
            };
            (*tree).sendSampleOnClose = *sample;
        }
    } else {
        let mut error_0: rmtError = ObjectAllocator_Alloc(
            (*tree).allocator,
            sample as *mut *mut libc::c_void,
        );
        if error_0 as libc::c_uint != RMT_ERROR_NONE as libc::c_int as libc::c_uint {
            return error_0;
        }
        Sample_Prepare(*sample, name_hash, parent);
    }
    unique_id = (*parent).unique_id;
    unique_id = HashCombine(unique_id, (**sample).name_hash);
    unique_id = HashCombine(unique_id, (*parent).nb_children);
    (**sample).unique_id = unique_id;
    (*parent).nb_children = ((*parent).nb_children).wrapping_add(1);
    (*parent).nb_children;
    if ((*parent).first_child).is_null() {
        (*parent).first_child = *sample;
        (*parent).last_child = *sample;
    } else {
        if !((*parent).last_child).is_null() {} else {
            __assert_fail(
                b"parent->last_child != NULL\0" as *const u8 as *const libc::c_char,
                b"lib/Remotery.c\0" as *const u8 as *const libc::c_char,
                4869 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 66],
                    &[libc::c_char; 66],
                >(
                    b"rmtError SampleTree_Push(SampleTree *, rmtU32, rmtU32, Sample **)\0",
                ))
                    .as_ptr(),
            );
        }
        'c_11298: {
            if !((*parent).last_child).is_null() {} else {
                __assert_fail(
                    b"parent->last_child != NULL\0" as *const u8 as *const libc::c_char,
                    b"lib/Remotery.c\0" as *const u8 as *const libc::c_char,
                    4869 as libc::c_int as libc::c_uint,
                    (*::std::mem::transmute::<
                        &[u8; 66],
                        &[libc::c_char; 66],
                    >(
                        b"rmtError SampleTree_Push(SampleTree *, rmtU32, rmtU32, Sample **)\0",
                    ))
                        .as_ptr(),
                );
            }
        };
        (*(*parent).last_child).next_sibling = *sample;
        (*parent).last_child = *sample;
    }
    (*tree).currentParent = *sample;
    return RMT_ERROR_NONE;
}
unsafe extern "C" fn SampleTree_Pop(mut tree: *mut SampleTree, mut sample: *mut Sample) {
    if !tree.is_null() {} else {
        __assert_fail(
            b"tree != NULL\0" as *const u8 as *const libc::c_char,
            b"lib/Remotery.c\0" as *const u8 as *const libc::c_char,
            4882 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 44],
                &[libc::c_char; 44],
            >(b"void SampleTree_Pop(SampleTree *, Sample *)\0"))
                .as_ptr(),
        );
    }
    'c_6255: {
        if !tree.is_null() {} else {
            __assert_fail(
                b"tree != NULL\0" as *const u8 as *const libc::c_char,
                b"lib/Remotery.c\0" as *const u8 as *const libc::c_char,
                4882 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 44],
                    &[libc::c_char; 44],
                >(b"void SampleTree_Pop(SampleTree *, Sample *)\0"))
                    .as_ptr(),
            );
        }
    };
    if !sample.is_null() {} else {
        __assert_fail(
            b"sample != NULL\0" as *const u8 as *const libc::c_char,
            b"lib/Remotery.c\0" as *const u8 as *const libc::c_char,
            4883 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 44],
                &[libc::c_char; 44],
            >(b"void SampleTree_Pop(SampleTree *, Sample *)\0"))
                .as_ptr(),
        );
    }
    'c_6213: {
        if !sample.is_null() {} else {
            __assert_fail(
                b"sample != NULL\0" as *const u8 as *const libc::c_char,
                b"lib/Remotery.c\0" as *const u8 as *const libc::c_char,
                4883 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 44],
                    &[libc::c_char; 44],
                >(b"void SampleTree_Pop(SampleTree *, Sample *)\0"))
                    .as_ptr(),
            );
        }
    };
    if sample != (*tree).root {} else {
        __assert_fail(
            b"sample != tree->root\0" as *const u8 as *const libc::c_char,
            b"lib/Remotery.c\0" as *const u8 as *const libc::c_char,
            4884 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 44],
                &[libc::c_char; 44],
            >(b"void SampleTree_Pop(SampleTree *, Sample *)\0"))
                .as_ptr(),
        );
    }
    'c_6171: {
        if sample != (*tree).root {} else {
            __assert_fail(
                b"sample != tree->root\0" as *const u8 as *const libc::c_char,
                b"lib/Remotery.c\0" as *const u8 as *const libc::c_char,
                4884 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 44],
                    &[libc::c_char; 44],
                >(b"void SampleTree_Pop(SampleTree *, Sample *)\0"))
                    .as_ptr(),
            );
        }
    };
    (*tree).currentParent = (*sample).parent;
}
unsafe extern "C" fn FlattenSamples(
    mut sample: *mut Sample,
    mut nb_samples: *mut rmtU32,
) -> *mut ObjectLink {
    let mut child: *mut Sample = 0 as *mut Sample;
    let mut cur_link: *mut ObjectLink = &mut (*sample).Link;
    if !sample.is_null() {} else {
        __assert_fail(
            b"sample != NULL\0" as *const u8 as *const libc::c_char,
            b"lib/Remotery.c\0" as *const u8 as *const libc::c_char,
            4893 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 47],
                &[libc::c_char; 47],
            >(b"ObjectLink *FlattenSamples(Sample *, rmtU32 *)\0"))
                .as_ptr(),
        );
    }
    'c_4585: {
        if !sample.is_null() {} else {
            __assert_fail(
                b"sample != NULL\0" as *const u8 as *const libc::c_char,
                b"lib/Remotery.c\0" as *const u8 as *const libc::c_char,
                4893 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 47],
                    &[libc::c_char; 47],
                >(b"ObjectLink *FlattenSamples(Sample *, rmtU32 *)\0"))
                    .as_ptr(),
            );
        }
    };
    if !nb_samples.is_null() {} else {
        __assert_fail(
            b"nb_samples != NULL\0" as *const u8 as *const libc::c_char,
            b"lib/Remotery.c\0" as *const u8 as *const libc::c_char,
            4894 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 47],
                &[libc::c_char; 47],
            >(b"ObjectLink *FlattenSamples(Sample *, rmtU32 *)\0"))
                .as_ptr(),
        );
    }
    'c_4541: {
        if !nb_samples.is_null() {} else {
            __assert_fail(
                b"nb_samples != NULL\0" as *const u8 as *const libc::c_char,
                b"lib/Remotery.c\0" as *const u8 as *const libc::c_char,
                4894 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 47],
                    &[libc::c_char; 47],
                >(b"ObjectLink *FlattenSamples(Sample *, rmtU32 *)\0"))
                    .as_ptr(),
            );
        }
    };
    *nb_samples = (*nb_samples as libc::c_uint)
        .wrapping_add(1 as libc::c_int as libc::c_uint) as rmtU32 as rmtU32;
    ::std::ptr::write_volatile(
        &mut (*sample).Link.next as *mut *mut ObjectLink_s,
        (*sample).first_child as *mut ObjectLink,
    );
    child = (*sample).first_child;
    while !child.is_null() {
        let mut last_link: *mut ObjectLink = FlattenSamples(child, nb_samples);
        ::std::ptr::write_volatile(
            &mut (*last_link).next as *mut *mut ObjectLink_s,
            (*child).next_sibling as *mut ObjectLink,
        );
        cur_link = last_link;
        child = (*child).next_sibling;
    }
    (*sample).first_child = 0 as *mut Sample;
    (*sample).last_child = 0 as *mut Sample;
    (*sample).nb_children = 0 as libc::c_int as rmtU32;
    return cur_link;
}
unsafe extern "C" fn FreeSamples(
    mut sample: *mut Sample,
    mut allocator: *mut ObjectAllocator,
) {
    let mut nb_cleared_samples: rmtU32 = 0 as libc::c_int as rmtU32;
    let mut last_link: *mut ObjectLink = FlattenSamples(sample, &mut nb_cleared_samples);
    if !((*sample).Link.next).is_null() {
        ObjectAllocator_FreeRange(
            allocator,
            sample as *mut libc::c_void,
            last_link as *mut libc::c_void,
            nb_cleared_samples,
        );
    } else {
        ObjectAllocator_Free(allocator, sample as *mut libc::c_void);
    };
}
unsafe extern "C" fn SampleTree_CopySample(
    mut out_dst_sample: *mut *mut Sample,
    mut dst_parent_sample: *mut Sample,
    mut allocator: *mut ObjectAllocator,
    mut src_sample: *const Sample,
) -> rmtError {
    let mut src_child: *mut Sample = 0 as *mut Sample;
    let mut dst_sample: *mut Sample = 0 as *mut Sample;
    let mut error: rmtError = ObjectAllocator_Alloc(
        allocator,
        &mut dst_sample as *mut *mut Sample as *mut *mut libc::c_void,
    );
    if error as libc::c_uint != RMT_ERROR_NONE as libc::c_int as libc::c_uint {
        return error;
    }
    Sample_CopyState(dst_sample, src_sample);
    if !dst_parent_sample.is_null() {
        if ((*dst_parent_sample).first_child).is_null() {
            (*dst_parent_sample).first_child = dst_sample;
            (*dst_parent_sample).last_child = dst_sample;
        } else {
            if !((*dst_parent_sample).last_child).is_null() {} else {
                __assert_fail(
                    b"dst_parent_sample->last_child != NULL\0" as *const u8
                        as *const libc::c_char,
                    b"lib/Remotery.c\0" as *const u8 as *const libc::c_char,
                    4952 as libc::c_int as libc::c_uint,
                    (*::std::mem::transmute::<
                        &[u8; 87],
                        &[libc::c_char; 87],
                    >(
                        b"rmtError SampleTree_CopySample(Sample **, Sample *, ObjectAllocator *, const Sample *)\0",
                    ))
                        .as_ptr(),
                );
            }
            'c_5743: {
                if !((*dst_parent_sample).last_child).is_null() {} else {
                    __assert_fail(
                        b"dst_parent_sample->last_child != NULL\0" as *const u8
                            as *const libc::c_char,
                        b"lib/Remotery.c\0" as *const u8 as *const libc::c_char,
                        4952 as libc::c_int as libc::c_uint,
                        (*::std::mem::transmute::<
                            &[u8; 87],
                            &[libc::c_char; 87],
                        >(
                            b"rmtError SampleTree_CopySample(Sample **, Sample *, ObjectAllocator *, const Sample *)\0",
                        ))
                            .as_ptr(),
                    );
                }
            };
            (*(*dst_parent_sample).last_child).next_sibling = dst_sample;
            (*dst_parent_sample).last_child = dst_sample;
        }
    }
    src_child = (*src_sample).first_child;
    while !src_child.is_null() {
        let mut dst_child: *mut Sample = 0 as *mut Sample;
        let mut error_0: rmtError = SampleTree_CopySample(
            &mut dst_child,
            dst_sample,
            allocator,
            src_child,
        );
        if error_0 as libc::c_uint != RMT_ERROR_NONE as libc::c_int as libc::c_uint {
            return error_0;
        }
        src_child = (*src_child).next_sibling;
    }
    *out_dst_sample = dst_sample;
    return RMT_ERROR_NONE;
}
unsafe extern "C" fn SampleTree_Copy(
    mut dst_tree: *mut SampleTree,
    mut src_tree: *const SampleTree,
) -> rmtError {
    let mut allocator: *mut ObjectAllocator = (*src_tree).allocator;
    (*dst_tree).allocator = allocator;
    let mut error: rmtError = SampleTree_CopySample(
        &mut (*dst_tree).root,
        0 as *mut Sample,
        allocator,
        (*src_tree).root,
    );
    if error as libc::c_uint != RMT_ERROR_NONE as libc::c_int as libc::c_uint {
        return error;
    }
    (*dst_tree).currentParent = (*dst_tree).root;
    return RMT_ERROR_NONE;
}
unsafe extern "C" fn QueueSampleTree(
    mut queue: *mut rmtMessageQueue,
    mut sample: *mut Sample,
    mut allocator: *mut ObjectAllocator,
    mut thread_name: rmtPStr,
    mut user_data: rmtU32,
    mut thread_profiler: *mut ThreadProfiler,
    mut partial_tree: rmtBool,
) {
    let mut payload: *mut Msg_SampleTree = 0 as *mut Msg_SampleTree;
    let mut message: *mut Message = rmtMessageQueue_AllocMessage(
        queue,
        ::std::mem::size_of::<Msg_SampleTree>() as libc::c_ulong as rmtU32,
        thread_profiler,
    );
    if message.is_null() {
        FreeSamples(sample, allocator);
        return;
    }
    payload = ((*message).payload).as_mut_ptr() as *mut Msg_SampleTree;
    (*payload).rootSample = sample;
    (*payload).allocator = allocator;
    (*payload).threadName = thread_name;
    (*payload).userData = user_data;
    (*payload).partialTree = partial_tree;
    rmtMessageQueue_CommitMessage(message, MsgID_SampleTree);
}
unsafe extern "C" fn QueueAddToStringTable(
    mut queue: *mut rmtMessageQueue,
    mut hash: rmtU32,
    mut string: *const libc::c_char,
    mut length: size_t,
    mut thread_profiler: *mut ThreadProfiler,
) -> rmtBool {
    let mut payload: *mut Msg_AddToStringTable = 0 as *mut Msg_AddToStringTable;
    let mut nb_string_bytes: size_t = length
        .wrapping_add(1 as libc::c_int as libc::c_ulong);
    let mut message: *mut Message = rmtMessageQueue_AllocMessage(
        queue,
        (::std::mem::size_of::<Msg_AddToStringTable>() as libc::c_ulong)
            .wrapping_add(nb_string_bytes) as rmtU32,
        thread_profiler,
    );
    if message.is_null() {
        return 0 as libc::c_int as rmtBool;
    }
    payload = ((*message).payload).as_mut_ptr() as *mut Msg_AddToStringTable;
    (*payload).hash = hash;
    (*payload).length = length as rmtU32;
    memcpy(
        payload.offset(1 as libc::c_int as isize) as *mut libc::c_void,
        string as *const libc::c_void,
        nb_string_bytes,
    );
    rmtMessageQueue_CommitMessage(message, MsgID_AddToStringTable);
    return 1 as libc::c_int as rmtBool;
}
unsafe extern "C" fn ThreadProfiler_Constructor(
    mut mq_to_rmt: *mut rmtMessageQueue,
    mut thread_profiler: *mut ThreadProfiler,
    mut thread_id: rmtThreadId,
) -> rmtError {
    let mut name_length: rmtU32 = 0;
    ::std::ptr::write_volatile(
        &mut (*thread_profiler).nbSamplesWithoutCallback as *mut rmtAtomicS32,
        0 as libc::c_int,
    );
    (*thread_profiler).processorIndex = -(1 as libc::c_int) as rmtU32;
    (*thread_profiler).lastProcessorIndex = -(1 as libc::c_int) as rmtU32;
    (*thread_profiler).threadId = thread_id;
    memset(
        ((*thread_profiler).sampleTrees).as_mut_ptr() as *mut libc::c_void,
        0 as libc::c_int,
        ::std::mem::size_of::<[*mut SampleTree; 6]>() as libc::c_ulong,
    );
    let mut error: rmtError = rmtOpenThreadHandle(
        thread_id,
        &mut (*thread_profiler).threadHandle,
    );
    if error as libc::c_uint != RMT_ERROR_NONE as libc::c_int as libc::c_uint {
        return error;
    }
    rmtGetThreadName(
        thread_id,
        (*thread_profiler).threadHandle,
        ((*thread_profiler).threadName).as_mut_ptr(),
        ::std::mem::size_of::<[libc::c_char; 64]>() as libc::c_ulong as rmtU32,
    );
    name_length = strnlen_s_safe_c(
        ((*thread_profiler).threadName).as_mut_ptr(),
        64 as libc::c_int as r_size_t,
    );
    (*thread_profiler)
        .threadNameHash = _rmt_HashString32(
        ((*thread_profiler).threadName).as_mut_ptr(),
        name_length as libc::c_int,
        0 as libc::c_int as rmtU32,
    );
    QueueAddToStringTable(
        mq_to_rmt,
        (*thread_profiler).threadNameHash,
        ((*thread_profiler).threadName).as_mut_ptr(),
        name_length as size_t,
        thread_profiler,
    );
    (*thread_profiler)
        .sampleTrees[RMT_SampleType_CPU as libc::c_int
        as usize] = rmtMalloc(
        ::std::mem::size_of::<SampleTree>() as libc::c_ulong as rmtU32,
    ) as *mut SampleTree;
    if ((*thread_profiler).sampleTrees[RMT_SampleType_CPU as libc::c_int as usize])
        .is_null()
    {
        return RMT_ERROR_MALLOC_FAIL;
    }
    let mut error_0: rmtError = SampleTree_Constructor(
        (*thread_profiler).sampleTrees[RMT_SampleType_CPU as libc::c_int as usize],
        ::std::mem::size_of::<Sample>() as libc::c_ulong as rmtU32,
        ::std::mem::transmute::<
            Option::<unsafe extern "C" fn(*mut Sample) -> rmtError>,
            ObjConstructor,
        >(Some(Sample_Constructor as unsafe extern "C" fn(*mut Sample) -> rmtError)),
        ::std::mem::transmute::<
            Option::<unsafe extern "C" fn(*mut Sample) -> ()>,
            ObjDestructor,
        >(Some(Sample_Destructor as unsafe extern "C" fn(*mut Sample) -> ())),
    );
    if error_0 as libc::c_uint != RMT_ERROR_NONE as libc::c_int as libc::c_uint {
        if !((*thread_profiler).sampleTrees[RMT_SampleType_CPU as libc::c_int as usize])
            .is_null()
        {
            SampleTree_Destructor(
                (*thread_profiler)
                    .sampleTrees[RMT_SampleType_CPU as libc::c_int as usize],
            );
            rmtFree(
                (*thread_profiler)
                    .sampleTrees[RMT_SampleType_CPU as libc::c_int as usize]
                    as *mut libc::c_void,
            );
            (*thread_profiler)
                .sampleTrees[RMT_SampleType_CPU as libc::c_int
                as usize] = 0 as *mut SampleTree;
        }
        return error_0;
    }
    return RMT_ERROR_NONE;
}
unsafe extern "C" fn ThreadProfiler_Destructor(
    mut thread_profiler: *mut ThreadProfiler,
) {
    let mut index: rmtU32 = 0;
    index = 0 as libc::c_int as rmtU32;
    while index < RMT_SampleType_Count as libc::c_int as libc::c_uint {
        if !((*thread_profiler).sampleTrees[index as usize]).is_null() {
            SampleTree_Destructor((*thread_profiler).sampleTrees[index as usize]);
            rmtFree((*thread_profiler).sampleTrees[index as usize] as *mut libc::c_void);
            (*thread_profiler).sampleTrees[index as usize] = 0 as *mut SampleTree;
        }
        index = index.wrapping_add(1);
        index;
    }
    rmtCloseThreadHandle((*thread_profiler).threadHandle);
}
unsafe extern "C" fn ThreadProfiler_Push(
    mut tree: *mut SampleTree,
    mut name_hash: rmtU32,
    mut flags: rmtU32,
    mut sample: *mut *mut Sample,
) -> rmtError {
    let mut error: rmtError = RMT_ERROR_NONE;
    StoreRelease(&mut (*tree).treeBeingModified, 1 as libc::c_int as rmtU32);
    error = SampleTree_Push(tree, name_hash, flags, sample);
    StoreRelease(&mut (*tree).treeBeingModified, 0 as libc::c_int as rmtU32);
    return error;
}
unsafe extern "C" fn CloseOpenSamples(
    mut sample: *mut Sample,
    mut sample_time_us: rmtU64,
    mut parents_are_last: rmtU32,
) {
    let mut child_sample: *mut Sample = 0 as *mut Sample;
    child_sample = (*sample).first_child;
    while !child_sample.is_null() {
        let mut is_last: rmtU32 = parents_are_last
            & (if child_sample == (*sample).last_child {
                1 as libc::c_int
            } else {
                0 as libc::c_int
            }) as libc::c_uint;
        CloseOpenSamples(child_sample, sample_time_us, is_last);
        child_sample = (*child_sample).next_sibling;
    }
    if parents_are_last > 0 as libc::c_int as libc::c_uint {
        Sample_Close(sample, sample_time_us as rmtS64);
    }
}
unsafe extern "C" fn MakePartialTreeCopy(
    mut sample_tree: *mut SampleTree,
    mut sample_time_us: rmtU64,
    mut out_sample_tree_copy: *mut SampleTree,
) -> rmtError {
    let mut sample_time_s: rmtU32 = sample_time_us
        .wrapping_div(1000 as libc::c_int as libc::c_ulonglong) as rmtU32;
    StoreRelease(&mut (*sample_tree).msLastTreeSendTime, sample_time_s);
    let mut error: rmtError = SampleTree_Copy(out_sample_tree_copy, sample_tree);
    if error as libc::c_uint != RMT_ERROR_NONE as libc::c_int as libc::c_uint {
        return error;
    }
    CloseOpenSamples(
        (*out_sample_tree_copy).root,
        sample_time_us,
        1 as libc::c_int as rmtU32,
    );
    return RMT_ERROR_NONE;
}
unsafe extern "C" fn ThreadProfiler_Pop(
    mut thread_profiler: *mut ThreadProfiler,
    mut queue: *mut rmtMessageQueue,
    mut sample: *mut Sample,
    mut msg_user_data: rmtU32,
) -> rmtBool {
    let mut tree: *mut SampleTree = (*thread_profiler)
        .sampleTrees[(*sample).type_0 as usize];
    SampleTree_Pop(tree, sample);
    if (*tree).currentParent == (*tree).root {
        let mut root: *mut Sample = 0 as *mut Sample;
        StoreRelease(&mut (*tree).treeBeingModified, 1 as libc::c_int as rmtU32);
        root = (*tree).root;
        (*root).first_child = 0 as *mut Sample;
        (*root).last_child = 0 as *mut Sample;
        (*root).nb_children = 0 as libc::c_int as rmtU32;
        StoreRelease(&mut (*tree).treeBeingModified, 0 as libc::c_int as rmtU32);
        QueueSampleTree(
            queue,
            sample,
            (*tree).allocator,
            ((*thread_profiler).threadName).as_mut_ptr() as rmtPStr,
            msg_user_data,
            thread_profiler,
            0 as libc::c_int as rmtBool,
        );
        StoreRelease(
            &mut (*tree).msLastTreeSendTime,
            ((*sample).us_end).wrapping_div(1000 as libc::c_int as libc::c_ulonglong)
                as rmtU32,
        );
        return 1 as libc::c_int as rmtBool;
    }
    if (*tree).sendSampleOnClose == sample {
        let mut partial_tree: SampleTree = SampleTree {
            allocator: 0 as *mut ObjectAllocator,
            root: 0 as *mut Sample,
            currentParent: 0 as *mut Sample,
            msLastTreeSendTime: 0,
            treeBeingModified: 0,
            sendSampleOnClose: 0 as *mut Sample,
        };
        if MakePartialTreeCopy(
            tree,
            ((*sample).us_start).wrapping_add((*sample).us_length),
            &mut partial_tree,
        ) as libc::c_uint == RMT_ERROR_NONE as libc::c_int as libc::c_uint
        {
            let mut sample_0: *mut Sample = (*partial_tree.root).first_child;
            if !sample_0.is_null() {} else {
                __assert_fail(
                    b"sample != NULL\0" as *const u8 as *const libc::c_char,
                    b"lib/Remotery.c\0" as *const u8 as *const libc::c_char,
                    5241 as libc::c_int as libc::c_uint,
                    (*::std::mem::transmute::<
                        &[u8; 82],
                        &[libc::c_char; 82],
                    >(
                        b"rmtBool ThreadProfiler_Pop(ThreadProfiler *, rmtMessageQueue *, Sample *, rmtU32)\0",
                    ))
                        .as_ptr(),
                );
            }
            'c_5304: {
                if !sample_0.is_null() {} else {
                    __assert_fail(
                        b"sample != NULL\0" as *const u8 as *const libc::c_char,
                        b"lib/Remotery.c\0" as *const u8 as *const libc::c_char,
                        5241 as libc::c_int as libc::c_uint,
                        (*::std::mem::transmute::<
                            &[u8; 82],
                            &[libc::c_char; 82],
                        >(
                            b"rmtBool ThreadProfiler_Pop(ThreadProfiler *, rmtMessageQueue *, Sample *, rmtU32)\0",
                        ))
                            .as_ptr(),
                    );
                }
            };
            QueueSampleTree(
                queue,
                sample_0,
                partial_tree.allocator,
                ((*thread_profiler).threadName).as_mut_ptr() as rmtPStr,
                msg_user_data,
                thread_profiler,
                1 as libc::c_int as rmtBool,
            );
        }
        if !(partial_tree.root).is_null() {
            FreeSamples(partial_tree.root, partial_tree.allocator);
        }
        (*tree).sendSampleOnClose = 0 as *mut Sample;
    }
    return 0 as libc::c_int as rmtBool;
}
unsafe extern "C" fn ThreadProfiler_GetNameHash(
    mut thread_profiler: *mut ThreadProfiler,
    mut queue: *mut rmtMessageQueue,
    mut name: rmtPStr,
    mut hash_cache: *mut rmtU32,
) -> rmtU32 {
    let mut name_len: size_t = 0;
    let mut name_hash: rmtU32 = 0;
    if !hash_cache.is_null() {
        name_hash = *hash_cache;
        if name_hash == 0 as libc::c_int as libc::c_uint {
            if !name.is_null() {} else {
                __assert_fail(
                    b"name != NULL\0" as *const u8 as *const libc::c_char,
                    b"lib/Remotery.c\0" as *const u8 as *const libc::c_char,
                    5269 as libc::c_int as libc::c_uint,
                    (*::std::mem::transmute::<
                        &[u8; 90],
                        &[libc::c_char; 90],
                    >(
                        b"rmtU32 ThreadProfiler_GetNameHash(ThreadProfiler *, rmtMessageQueue *, rmtPStr, rmtU32 *)\0",
                    ))
                        .as_ptr(),
                );
            }
            'c_11138: {
                if !name.is_null() {} else {
                    __assert_fail(
                        b"name != NULL\0" as *const u8 as *const libc::c_char,
                        b"lib/Remotery.c\0" as *const u8 as *const libc::c_char,
                        5269 as libc::c_int as libc::c_uint,
                        (*::std::mem::transmute::<
                            &[u8; 90],
                            &[libc::c_char; 90],
                        >(
                            b"rmtU32 ThreadProfiler_GetNameHash(ThreadProfiler *, rmtMessageQueue *, rmtPStr, rmtU32 *)\0",
                        ))
                            .as_ptr(),
                    );
                }
            };
            name_len = strnlen_s_safe_c(name, 256 as libc::c_int as r_size_t) as size_t;
            name_hash = _rmt_HashString32(
                name,
                name_len as libc::c_int,
                0 as libc::c_int as rmtU32,
            );
            if QueueAddToStringTable(queue, name_hash, name, name_len, thread_profiler)
                == 1 as libc::c_int as rmtBool
            {
                *hash_cache = name_hash;
            }
        }
        return name_hash;
    }
    name_len = strnlen_s_safe_c(name, 256 as libc::c_int as r_size_t) as size_t;
    name_hash = _rmt_HashString32(
        name,
        name_len as libc::c_int,
        0 as libc::c_int as rmtU32,
    );
    QueueAddToStringTable(queue, name_hash, name, name_len, thread_profiler);
    return name_hash;
}
unsafe extern "C" fn ThreadProfilers_Constructor(
    mut thread_profilers: *mut ThreadProfilers,
    mut timer: *mut usTimer,
    mut mq_to_rmt_thread: *mut rmtMessageQueue,
) -> rmtError {
    (*thread_profilers).timer = timer;
    (*thread_profilers).mqToRmtThread = mq_to_rmt_thread;
    (*thread_profilers).compiledSampleFn = 0 as *mut libc::c_void;
    (*thread_profilers).compiledSampleFnSize = 0 as libc::c_int as rmtU32;
    (*thread_profilers).threadProfilerTlsHandle = 0xffffffff as libc::c_uint;
    ::std::ptr::write_volatile(
        &mut (*thread_profilers).nbThreadProfilers as *mut rmtAtomicU32,
        0 as libc::c_int as rmtU32,
    );
    (*thread_profilers)
        .maxNbThreadProfilers = (::std::mem::size_of::<[ThreadProfiler; 256]>()
        as libc::c_ulong)
        .wrapping_div(::std::mem::size_of::<ThreadProfiler>() as libc::c_ulong)
        as rmtU32;
    mtxInit(&mut (*thread_profilers).threadProfilerMutex);
    (*thread_profilers).threadSampleThread = 0 as *mut rmtThread;
    (*thread_profilers).threadGatherThread = 0 as *mut rmtThread;
    let mut error: rmtError = tlsAlloc(&mut (*thread_profilers).threadProfilerTlsHandle);
    if error as libc::c_uint != RMT_ERROR_NONE as libc::c_int as libc::c_uint {
        return error;
    }
    if g_Settings.enableThreadSampler == 1 as libc::c_int as rmtBool {
        (*thread_profilers)
            .threadSampleThread = rmtMalloc(
            ::std::mem::size_of::<rmtThread>() as libc::c_ulong as rmtU32,
        ) as *mut rmtThread;
        if ((*thread_profilers).threadSampleThread).is_null() {
            return RMT_ERROR_MALLOC_FAIL;
        }
        let mut error_0: rmtError = rmtThread_Constructor(
            (*thread_profilers).threadSampleThread,
            Some(SampleThreadsLoop as unsafe extern "C" fn(*mut rmtThread) -> rmtError),
            thread_profilers as *mut libc::c_void,
        );
        if error_0 as libc::c_uint != RMT_ERROR_NONE as libc::c_int as libc::c_uint {
            if !((*thread_profilers).threadSampleThread).is_null() {
                rmtThread_Destructor((*thread_profilers).threadSampleThread);
                rmtFree((*thread_profilers).threadSampleThread as *mut libc::c_void);
                (*thread_profilers).threadSampleThread = 0 as *mut rmtThread;
            }
            return error_0;
        }
    }
    return RMT_ERROR_NONE;
}
unsafe extern "C" fn ThreadProfilers_Destructor(
    mut thread_profilers: *mut ThreadProfilers,
) {
    let mut thread_index: rmtU32 = 0;
    if !((*thread_profilers).threadSampleThread).is_null() {
        rmtThread_Destructor((*thread_profilers).threadSampleThread);
        rmtFree((*thread_profilers).threadSampleThread as *mut libc::c_void);
        (*thread_profilers).threadSampleThread = 0 as *mut rmtThread;
    }
    thread_index = 0 as libc::c_int as rmtU32;
    while thread_index < (*thread_profilers).nbThreadProfilers {
        let mut thread_profiler: *mut ThreadProfiler = ((*thread_profilers)
            .threadProfilers)
            .as_mut_ptr()
            .offset(thread_index as isize);
        ThreadProfiler_Destructor(thread_profiler);
        thread_index = thread_index.wrapping_add(1);
        thread_index;
    }
    if (*thread_profilers).threadProfilerTlsHandle != 0xffffffff as libc::c_uint {
        tlsFree((*thread_profilers).threadProfilerTlsHandle);
    }
    mtxDelete(&mut (*thread_profilers).threadProfilerMutex);
}
unsafe extern "C" fn ThreadProfilers_GetThreadProfiler(
    mut thread_profilers: *mut ThreadProfilers,
    mut thread_id: rmtThreadId,
    mut out_thread_profiler: *mut *mut ThreadProfiler,
) -> rmtError {
    let mut profiler_index: rmtU32 = 0;
    let mut thread_profiler: *mut ThreadProfiler = 0 as *mut ThreadProfiler;
    let mut error: rmtError = RMT_ERROR_NONE;
    mtxLock(&mut (*thread_profilers).threadProfilerMutex);
    profiler_index = 0 as libc::c_int as rmtU32;
    while profiler_index < (*thread_profilers).nbThreadProfilers {
        thread_profiler = ((*thread_profilers).threadProfilers)
            .as_mut_ptr()
            .offset(profiler_index as isize);
        if (*thread_profiler).threadId == thread_id {
            *out_thread_profiler = thread_profiler;
            mtxUnlock(&mut (*thread_profilers).threadProfilerMutex);
            return RMT_ERROR_NONE;
        }
        profiler_index = profiler_index.wrapping_add(1);
        profiler_index;
    }
    thread_profiler = ((*thread_profilers).threadProfilers)
        .as_mut_ptr()
        .offset((*thread_profilers).nbThreadProfilers as isize);
    error = ThreadProfiler_Constructor(
        (*thread_profilers).mqToRmtThread,
        thread_profiler,
        thread_id,
    );
    if error as libc::c_uint != RMT_ERROR_NONE as libc::c_int as libc::c_uint {
        ThreadProfiler_Destructor(thread_profiler);
        mtxUnlock(&mut (*thread_profilers).threadProfilerMutex);
        return error;
    }
    *out_thread_profiler = thread_profiler;
    StoreRelease(
        &mut (*thread_profilers).nbThreadProfilers,
        ((*thread_profilers).nbThreadProfilers)
            .wrapping_add(1 as libc::c_int as libc::c_uint),
    );
    mtxUnlock(&mut (*thread_profilers).threadProfilerMutex);
    return RMT_ERROR_NONE;
}
unsafe extern "C" fn ThreadProfilers_GetCurrentThreadProfiler(
    mut thread_profilers: *mut ThreadProfilers,
    mut out_thread_profiler: *mut *mut ThreadProfiler,
) -> rmtError {
    *out_thread_profiler = tlsGet((*thread_profilers).threadProfilerTlsHandle)
        as *mut ThreadProfiler;
    if (*out_thread_profiler).is_null() {
        let mut error: rmtError = ThreadProfilers_GetThreadProfiler(
            thread_profilers,
            rmtGetCurrentThreadId(),
            out_thread_profiler,
        );
        if error as libc::c_uint != RMT_ERROR_NONE as libc::c_int as libc::c_uint {
            return error;
        }
        tlsSet(
            (*thread_profilers).threadProfilerTlsHandle,
            *out_thread_profiler as *mut libc::c_void,
        );
    }
    return RMT_ERROR_NONE;
}
unsafe extern "C" fn ThreadProfilers_ThreadInCallback(
    mut thread_profilers: *mut ThreadProfilers,
    mut context: *mut rmtCpuContext,
) -> rmtBool {
    return 0 as libc::c_int as rmtBool;
}
unsafe extern "C" fn GatherThreads(mut thread_profilers: *mut ThreadProfilers) {
    let mut handle: rmtThreadHandle = 0;
    if !thread_profilers.is_null() {} else {
        __assert_fail(
            b"thread_profilers != NULL\0" as *const u8 as *const libc::c_char,
            b"lib/Remotery.c\0" as *const u8 as *const libc::c_char,
            5486 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 38],
                &[libc::c_char; 38],
            >(b"void GatherThreads(ThreadProfilers *)\0"))
                .as_ptr(),
        );
    }
    'c_25525: {
        if !thread_profilers.is_null() {} else {
            __assert_fail(
                b"thread_profilers != NULL\0" as *const u8 as *const libc::c_char,
                b"lib/Remotery.c\0" as *const u8 as *const libc::c_char,
                5486 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 38],
                    &[libc::c_char; 38],
                >(b"void GatherThreads(ThreadProfilers *)\0"))
                    .as_ptr(),
            );
        }
    };
}
unsafe extern "C" fn GatherThreadsLoop(mut thread: *mut rmtThread) -> rmtError {
    let mut thread_profilers: *mut ThreadProfilers = (*thread).param
        as *mut ThreadProfilers;
    let mut sleep_time: rmtU32 = 100 as libc::c_int as rmtU32;
    if !thread_profilers.is_null() {} else {
        __assert_fail(
            b"thread_profilers != NULL\0" as *const u8 as *const libc::c_char,
            b"lib/Remotery.c\0" as *const u8 as *const libc::c_char,
            5529 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 40],
                &[libc::c_char; 40],
            >(b"rmtError GatherThreadsLoop(rmtThread *)\0"))
                .as_ptr(),
        );
    }
    'c_25580: {
        if !thread_profilers.is_null() {} else {
            __assert_fail(
                b"thread_profilers != NULL\0" as *const u8 as *const libc::c_char,
                b"lib/Remotery.c\0" as *const u8 as *const libc::c_char,
                5529 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 40],
                    &[libc::c_char; 40],
                >(b"rmtError GatherThreadsLoop(rmtThread *)\0"))
                    .as_ptr(),
            );
        }
    };
    _rmt_SetCurrentThreadName(
        b"RemoteryGatherThreads\0" as *const u8 as *const libc::c_char,
    );
    while (*thread).request_exit == 0 as libc::c_int as rmtBool {
        GatherThreads(thread_profilers);
        msSleep(sleep_time);
        sleep_time = minU32(
            sleep_time.wrapping_mul(2 as libc::c_int as libc::c_uint),
            2000 as libc::c_int as rmtU32,
        );
    }
    return RMT_ERROR_NONE;
}
unsafe extern "C" fn QueueProcessorThreads(
    mut queue: *mut rmtMessageQueue,
    mut message_index: rmtU64,
    mut nb_processors: rmtU32,
    mut processors: *mut Processor,
) {
    let mut payload: *mut Msg_ProcessorThreads = 0 as *mut Msg_ProcessorThreads;
    let mut array_size: rmtU32 = (nb_processors
        .wrapping_sub(1 as libc::c_int as libc::c_uint) as libc::c_ulong)
        .wrapping_mul(::std::mem::size_of::<Processor>() as libc::c_ulong) as rmtU32;
    let mut message: *mut Message = rmtMessageQueue_AllocMessage(
        queue,
        (::std::mem::size_of::<Msg_ProcessorThreads>() as libc::c_ulong)
            .wrapping_add(array_size as libc::c_ulong) as rmtU32,
        0 as *mut ThreadProfiler,
    );
    if message.is_null() {
        return;
    }
    payload = ((*message).payload).as_mut_ptr() as *mut Msg_ProcessorThreads;
    (*payload).messageIndex = message_index;
    (*payload).nbProcessors = nb_processors;
    memcpy(
        ((*payload).processors).as_mut_ptr() as *mut libc::c_void,
        processors as *const libc::c_void,
        (nb_processors as libc::c_ulong)
            .wrapping_mul(::std::mem::size_of::<Processor>() as libc::c_ulong),
    );
    rmtMessageQueue_CommitMessage(message, MsgID_ProcessorThreads);
}
unsafe extern "C" fn CheckForStallingSamples(
    mut stalling_sample_tree: *mut SampleTree,
    mut thread_profiler: *mut ThreadProfiler,
    mut sample_time_us: rmtU64,
) -> rmtError {
    let mut sample_tree: *mut SampleTree = 0 as *mut SampleTree;
    let mut sample_time_s: rmtU32 = sample_time_us
        .wrapping_div(1000 as libc::c_int as libc::c_ulonglong) as rmtU32;
    (*stalling_sample_tree).root = 0 as *mut Sample;
    (*stalling_sample_tree).allocator = 0 as *mut ObjectAllocator;
    sample_tree = (*thread_profiler)
        .sampleTrees[RMT_SampleType_CPU as libc::c_int as usize];
    if LoadAcquire(&mut (*sample_tree).treeBeingModified)
        != 0 as libc::c_int as libc::c_uint
    {
        return RMT_ERROR_NONE;
    }
    if !sample_tree.is_null() {
        let mut root_sample: *mut Sample = (*sample_tree).root;
        if !root_sample.is_null()
            && (*root_sample).nb_children > 0 as libc::c_int as libc::c_uint
        {
            if sample_time_s
                .wrapping_sub(LoadAcquire(&mut (*sample_tree).msLastTreeSendTime))
                > 1000 as libc::c_int as libc::c_uint
            {
                let mut error: rmtError = MakePartialTreeCopy(
                    sample_tree,
                    sample_time_us,
                    stalling_sample_tree,
                );
                if error as libc::c_uint != RMT_ERROR_NONE as libc::c_int as libc::c_uint
                {
                    return error;
                }
            }
        }
    }
    return RMT_ERROR_NONE;
}
unsafe extern "C" fn InitThreadSampling(
    mut thread_profilers: *mut ThreadProfilers,
) -> rmtError {
    _rmt_SetCurrentThreadName(
        b"RemoterySampleThreads\0" as *const u8 as *const libc::c_char,
    );
    GatherThreads(thread_profilers);
    (*thread_profilers)
        .threadGatherThread = rmtMalloc(
        ::std::mem::size_of::<rmtThread>() as libc::c_ulong as rmtU32,
    ) as *mut rmtThread;
    if ((*thread_profilers).threadGatherThread).is_null() {
        return RMT_ERROR_MALLOC_FAIL;
    }
    let mut error: rmtError = rmtThread_Constructor(
        (*thread_profilers).threadGatherThread,
        Some(GatherThreadsLoop as unsafe extern "C" fn(*mut rmtThread) -> rmtError),
        thread_profilers as *mut libc::c_void,
    );
    if error as libc::c_uint != RMT_ERROR_NONE as libc::c_int as libc::c_uint {
        if !((*thread_profilers).threadGatherThread).is_null() {
            rmtThread_Destructor((*thread_profilers).threadGatherThread);
            rmtFree((*thread_profilers).threadGatherThread as *mut libc::c_void);
            (*thread_profilers).threadGatherThread = 0 as *mut rmtThread;
        }
        return error;
    }
    Well512_Init(time(0 as *mut time_t) as rmtU32);
    return RMT_ERROR_NONE;
}
unsafe extern "C" fn SampleThreadsLoop(mut rmt_thread: *mut rmtThread) -> rmtError {
    let mut context: rmtCpuContext = 0;
    let mut processor_message_index: rmtU32 = 0 as libc::c_int as rmtU32;
    let mut nb_processors: rmtU32 = 0;
    let mut processors: *mut Processor = 0 as *mut Processor;
    let mut processor_index: rmtU32 = 0;
    let mut thread_profilers: *mut ThreadProfilers = (*rmt_thread).param
        as *mut ThreadProfilers;
    nb_processors = rmtGetNbProcessors();
    if nb_processors == 0 as libc::c_int as libc::c_uint {
        return RMT_ERROR_UNKNOWN;
    }
    let mut error: rmtError = InitThreadSampling(thread_profilers);
    if error as libc::c_uint != RMT_ERROR_NONE as libc::c_int as libc::c_uint {
        return error;
    }
    processors = rmtMalloc(
        (nb_processors as libc::c_ulong)
            .wrapping_mul(::std::mem::size_of::<Processor>() as libc::c_ulong) as rmtU32,
    ) as *mut Processor;
    if processors.is_null() {
        return RMT_ERROR_MALLOC_FAIL;
    }
    processor_index = 0 as libc::c_int as rmtU32;
    while processor_index < nb_processors {
        let ref mut fresh34 = (*processors.offset(processor_index as isize))
            .threadProfiler;
        *fresh34 = 0 as *mut ThreadProfiler;
        (*processors.offset(processor_index as isize))
            .sampleTime = 0 as libc::c_int as rmtU64;
        processor_index = processor_index.wrapping_add(1);
        processor_index;
    }
    while (*rmt_thread).request_exit == 0 as libc::c_int as rmtBool {
        let mut lfsr_seed: rmtU32 = 0;
        let mut lfsr_value: rmtU32 = 0;
        let mut nb_thread_profilers: rmtU32 = LoadAcquire(
            &mut (*thread_profilers).nbThreadProfilers,
        );
        let mut highest_bit_set: rmtU32 = Log2i(nb_thread_profilers);
        let mut table_size_log2: rmtU32 = highest_bit_set
            .wrapping_add(1 as libc::c_int as libc::c_uint);
        let mut xor_mask: rmtU32 = GaloisLFSRMask(table_size_log2);
        lfsr_seed = Well512_RandomOpenLimit(nb_thread_profilers);
        lfsr_value = lfsr_seed;
        loop {
            let mut thread_index: rmtU32 = 0;
            let mut thread_id: rmtThreadId = 0;
            let mut thread_profiler: *mut ThreadProfiler = 0 as *mut ThreadProfiler;
            let mut thread_handle: rmtThreadHandle = 0;
            let mut sample_time_us: rmtU64 = 0;
            let mut sample_count: rmtU32 = 0;
            let mut stalling_sample_tree: SampleTree = SampleTree {
                allocator: 0 as *mut ObjectAllocator,
                root: 0 as *mut Sample,
                currentParent: 0 as *mut Sample,
                msLastTreeSendTime: 0,
                treeBeingModified: 0,
                sendSampleOnClose: 0 as *mut Sample,
            };
            lfsr_value = GaloisLFSRNext(lfsr_value, xor_mask);
            thread_index = lfsr_value.wrapping_sub(1 as libc::c_int as libc::c_uint);
            if !(thread_index >= nb_thread_profilers) {
                thread_id = rmtGetCurrentThreadId();
                thread_profiler = ((*thread_profilers).threadProfilers)
                    .as_mut_ptr()
                    .offset(thread_index as isize);
                if !((*thread_profiler).threadId == thread_id) {
                    thread_handle = (*thread_profiler).threadHandle;
                    if !(rmtSuspendThread(thread_handle) == 0 as libc::c_int as rmtBool)
                    {
                        sample_time_us = usTimer_Get((*thread_profilers).timer);
                        sample_count = AtomicAddS32(
                            &mut (*thread_profiler).nbSamplesWithoutCallback,
                            1 as libc::c_int,
                        ) as rmtU32;
                        processor_index = (*thread_profiler).processorIndex;
                        if processor_index != -(1 as libc::c_int) as rmtU32 {
                            if processor_index < nb_processors {} else {
                                __assert_fail(
                                    b"processor_index < nb_processors\0" as *const u8
                                        as *const libc::c_char,
                                    b"lib/Remotery.c\0" as *const u8 as *const libc::c_char,
                                    5888 as libc::c_int as libc::c_uint,
                                    (*::std::mem::transmute::<
                                        &[u8; 40],
                                        &[libc::c_char; 40],
                                    >(b"rmtError SampleThreadsLoop(rmtThread *)\0"))
                                        .as_ptr(),
                                );
                            }
                            'c_24529: {
                                if processor_index < nb_processors {} else {
                                    __assert_fail(
                                        b"processor_index < nb_processors\0" as *const u8
                                            as *const libc::c_char,
                                        b"lib/Remotery.c\0" as *const u8 as *const libc::c_char,
                                        5888 as libc::c_int as libc::c_uint,
                                        (*::std::mem::transmute::<
                                            &[u8; 40],
                                            &[libc::c_char; 40],
                                        >(b"rmtError SampleThreadsLoop(rmtThread *)\0"))
                                            .as_ptr(),
                                    );
                                }
                            };
                            let ref mut fresh35 = (*processors
                                .offset(processor_index as isize))
                                .threadProfiler;
                            *fresh35 = thread_profiler;
                            (*processors.offset(processor_index as isize))
                                .sampleCount = sample_count;
                            (*processors.offset(processor_index as isize))
                                .sampleTime = sample_time_us;
                        }
                        if sample_count == 0 as libc::c_int as libc::c_uint {
                            if rmtGetUserModeThreadContext(thread_handle, &mut context)
                                == 1 as libc::c_int as rmtBool
                                && ThreadProfilers_ThreadInCallback(
                                    thread_profilers,
                                    &mut context,
                                ) == 0 as libc::c_int as rmtBool
                            {
                                rmtSetThreadContext(thread_handle, &mut context);
                            } else {
                                AtomicAddS32(
                                    &mut (*thread_profiler).nbSamplesWithoutCallback,
                                    -(1 as libc::c_int),
                                );
                            }
                        }
                        if RMT_ERROR_NONE as libc::c_int as libc::c_uint
                            != CheckForStallingSamples(
                                &mut stalling_sample_tree,
                                thread_profiler,
                                sample_time_us,
                            ) as libc::c_uint
                        {
                            if !(stalling_sample_tree.allocator).is_null() {} else {
                                __assert_fail(
                                    b"stalling_sample_tree.allocator != NULL\0" as *const u8
                                        as *const libc::c_char,
                                    b"lib/Remotery.c\0" as *const u8 as *const libc::c_char,
                                    5936 as libc::c_int as libc::c_uint,
                                    (*::std::mem::transmute::<
                                        &[u8; 40],
                                        &[libc::c_char; 40],
                                    >(b"rmtError SampleThreadsLoop(rmtThread *)\0"))
                                        .as_ptr(),
                                );
                            }
                            'c_24224: {
                                if !(stalling_sample_tree.allocator).is_null() {} else {
                                    __assert_fail(
                                        b"stalling_sample_tree.allocator != NULL\0" as *const u8
                                            as *const libc::c_char,
                                        b"lib/Remotery.c\0" as *const u8 as *const libc::c_char,
                                        5936 as libc::c_int as libc::c_uint,
                                        (*::std::mem::transmute::<
                                            &[u8; 40],
                                            &[libc::c_char; 40],
                                        >(b"rmtError SampleThreadsLoop(rmtThread *)\0"))
                                            .as_ptr(),
                                    );
                                }
                            };
                            if !(stalling_sample_tree.root).is_null() {
                                FreeSamples(
                                    stalling_sample_tree.root,
                                    stalling_sample_tree.allocator,
                                );
                            }
                        }
                        rmtResumeThread(thread_handle);
                        if !(stalling_sample_tree.root).is_null() {
                            let mut sample: *mut Sample = (*stalling_sample_tree.root)
                                .first_child;
                            if !sample.is_null() {} else {
                                __assert_fail(
                                    b"sample != NULL\0" as *const u8 as *const libc::c_char,
                                    b"lib/Remotery.c\0" as *const u8 as *const libc::c_char,
                                    5951 as libc::c_int as libc::c_uint,
                                    (*::std::mem::transmute::<
                                        &[u8; 40],
                                        &[libc::c_char; 40],
                                    >(b"rmtError SampleThreadsLoop(rmtThread *)\0"))
                                        .as_ptr(),
                                );
                            }
                            'c_24143: {
                                if !sample.is_null() {} else {
                                    __assert_fail(
                                        b"sample != NULL\0" as *const u8 as *const libc::c_char,
                                        b"lib/Remotery.c\0" as *const u8 as *const libc::c_char,
                                        5951 as libc::c_int as libc::c_uint,
                                        (*::std::mem::transmute::<
                                            &[u8; 40],
                                            &[libc::c_char; 40],
                                        >(b"rmtError SampleThreadsLoop(rmtThread *)\0"))
                                            .as_ptr(),
                                    );
                                }
                            };
                            QueueSampleTree(
                                (*thread_profilers).mqToRmtThread,
                                sample,
                                stalling_sample_tree.allocator,
                                ((*thread_profiler).threadName).as_mut_ptr() as rmtPStr,
                                0 as libc::c_int as rmtU32,
                                thread_profiler,
                                1 as libc::c_int as rmtBool,
                            );
                            (*stalling_sample_tree.root).first_child = 0 as *mut Sample;
                            (*stalling_sample_tree.root).last_child = 0 as *mut Sample;
                            (*stalling_sample_tree.root)
                                .nb_children = 0 as libc::c_int as rmtU32;
                            if !(stalling_sample_tree.allocator).is_null() {} else {
                                __assert_fail(
                                    b"stalling_sample_tree.allocator != NULL\0" as *const u8
                                        as *const libc::c_char,
                                    b"lib/Remotery.c\0" as *const u8 as *const libc::c_char,
                                    5960 as libc::c_int as libc::c_uint,
                                    (*::std::mem::transmute::<
                                        &[u8; 40],
                                        &[libc::c_char; 40],
                                    >(b"rmtError SampleThreadsLoop(rmtThread *)\0"))
                                        .as_ptr(),
                                );
                            }
                            'c_24043: {
                                if !(stalling_sample_tree.allocator).is_null() {} else {
                                    __assert_fail(
                                        b"stalling_sample_tree.allocator != NULL\0" as *const u8
                                            as *const libc::c_char,
                                        b"lib/Remotery.c\0" as *const u8 as *const libc::c_char,
                                        5960 as libc::c_int as libc::c_uint,
                                        (*::std::mem::transmute::<
                                            &[u8; 40],
                                            &[libc::c_char; 40],
                                        >(b"rmtError SampleThreadsLoop(rmtThread *)\0"))
                                            .as_ptr(),
                                    );
                                }
                            };
                            FreeSamples(
                                stalling_sample_tree.root,
                                stalling_sample_tree.allocator,
                            );
                        }
                    }
                }
            }
            if !(lfsr_value != lfsr_seed) {
                break;
            }
        }
        processor_index = 0 as libc::c_int as rmtU32;
        while processor_index < nb_processors {
            let mut processor: *mut Processor = processors
                .offset(processor_index as isize);
            let mut thread_profiler_0: *mut ThreadProfiler = (*processor).threadProfiler;
            if !thread_profiler_0.is_null() {
                let mut last_processor_index: rmtU32 = (*thread_profiler_0)
                    .lastProcessorIndex;
                if last_processor_index != -(1 as libc::c_int) as rmtU32
                    && last_processor_index != processor_index
                {
                    if last_processor_index < nb_processors {} else {
                        __assert_fail(
                            b"last_processor_index < nb_processors\0" as *const u8
                                as *const libc::c_char,
                            b"lib/Remotery.c\0" as *const u8 as *const libc::c_char,
                            5980 as libc::c_int as libc::c_uint,
                            (*::std::mem::transmute::<
                                &[u8; 40],
                                &[libc::c_char; 40],
                            >(b"rmtError SampleThreadsLoop(rmtThread *)\0"))
                                .as_ptr(),
                        );
                    }
                    'c_23928: {
                        if last_processor_index < nb_processors {} else {
                            __assert_fail(
                                b"last_processor_index < nb_processors\0" as *const u8
                                    as *const libc::c_char,
                                b"lib/Remotery.c\0" as *const u8 as *const libc::c_char,
                                5980 as libc::c_int as libc::c_uint,
                                (*::std::mem::transmute::<
                                    &[u8; 40],
                                    &[libc::c_char; 40],
                                >(b"rmtError SampleThreadsLoop(rmtThread *)\0"))
                                    .as_ptr(),
                            );
                        }
                    };
                    if (*processors.offset(last_processor_index as isize)).threadProfiler
                        == thread_profiler_0
                    {
                        let ref mut fresh36 = (*processors
                            .offset(last_processor_index as isize))
                            .threadProfiler;
                        *fresh36 = 0 as *mut ThreadProfiler;
                    }
                } else if (*processor).sampleCount > 1 as libc::c_int as libc::c_uint {
                    (*processor).threadProfiler = 0 as *mut ThreadProfiler;
                }
                (*thread_profiler_0)
                    .lastProcessorIndex = (*thread_profiler_0).processorIndex;
            }
            processor_index = processor_index.wrapping_add(1);
            processor_index;
        }
        let fresh37 = processor_message_index;
        processor_message_index = processor_message_index.wrapping_add(1);
        QueueProcessorThreads(
            (*thread_profilers).mqToRmtThread,
            fresh37 as rmtU64,
            nb_processors,
            processors,
        );
    }
    if !((*thread_profilers).threadGatherThread).is_null() {
        rmtThread_Destructor((*thread_profilers).threadGatherThread);
        rmtFree((*thread_profilers).threadGatherThread as *mut libc::c_void);
        (*thread_profilers).threadGatherThread = 0 as *mut rmtThread;
    }
    rmtFree(processors as *mut libc::c_void);
    return RMT_ERROR_NONE;
}
unsafe extern "C" fn PropertySnapshot_Constructor(
    mut snapshot: *mut PropertySnapshot,
) -> rmtError {
    if !snapshot.is_null() {} else {
        __assert_fail(
            b"snapshot != NULL\0" as *const u8 as *const libc::c_char,
            b"lib/Remotery.c\0" as *const u8 as *const libc::c_char,
            6063 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 58],
                &[libc::c_char; 58],
            >(b"rmtError PropertySnapshot_Constructor(PropertySnapshot *)\0"))
                .as_ptr(),
        );
    }
    'c_23461: {
        if !snapshot.is_null() {} else {
            __assert_fail(
                b"snapshot != NULL\0" as *const u8 as *const libc::c_char,
                b"lib/Remotery.c\0" as *const u8 as *const libc::c_char,
                6063 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 58],
                    &[libc::c_char; 58],
                >(b"rmtError PropertySnapshot_Constructor(PropertySnapshot *)\0"))
                    .as_ptr(),
            );
        }
    };
    ObjectLink_Constructor(snapshot as *mut ObjectLink);
    (*snapshot).type_0 = RMT_PropertyType_rmtBool;
    (*snapshot).value.Bool = 0 as libc::c_int as rmtBool;
    (*snapshot).nameHash = 0 as libc::c_int as rmtU32;
    (*snapshot).uniqueID = 0 as libc::c_int as rmtU32;
    (*snapshot).nbChildren = 0 as libc::c_int as rmtU32;
    (*snapshot).depth = 0 as libc::c_int as rmtU8;
    (*snapshot).nextSnapshot = 0 as *mut PropertySnapshot;
    return RMT_ERROR_NONE;
}
unsafe extern "C" fn PropertySnapshot_Destructor(mut snapshot: *mut PropertySnapshot) {
    if 1 as libc::c_int != 0 {} else {};
}
static mut g_Remotery: *mut Remotery = 0 as *const Remotery as *mut Remotery;
static mut g_RemoteryCreated: rmtBool = 0 as libc::c_int as rmtBool;
unsafe extern "C" fn saturate(mut v: libc::c_double) -> libc::c_double {
    if v < 0 as libc::c_int as libc::c_double {
        return 0 as libc::c_int as libc::c_double;
    }
    if v > 1 as libc::c_int as libc::c_double {
        return 1 as libc::c_int as libc::c_double;
    }
    return v;
}
unsafe extern "C" fn PostProcessSamples(
    mut sample: *mut Sample,
    mut nb_samples: *mut rmtU32,
) {
    let mut child: *mut Sample = 0 as *mut Sample;
    if !sample.is_null() {} else {
        __assert_fail(
            b"sample != NULL\0" as *const u8 as *const libc::c_char,
            b"lib/Remotery.c\0" as *const u8 as *const libc::c_char,
            6165 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 44],
                &[libc::c_char; 44],
            >(b"void PostProcessSamples(Sample *, rmtU32 *)\0"))
                .as_ptr(),
        );
    }
    'c_15258: {
        if !sample.is_null() {} else {
            __assert_fail(
                b"sample != NULL\0" as *const u8 as *const libc::c_char,
                b"lib/Remotery.c\0" as *const u8 as *const libc::c_char,
                6165 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 44],
                    &[libc::c_char; 44],
                >(b"void PostProcessSamples(Sample *, rmtU32 *)\0"))
                    .as_ptr(),
            );
        }
    };
    if !nb_samples.is_null() {} else {
        __assert_fail(
            b"nb_samples != NULL\0" as *const u8 as *const libc::c_char,
            b"lib/Remotery.c\0" as *const u8 as *const libc::c_char,
            6166 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 44],
                &[libc::c_char; 44],
            >(b"void PostProcessSamples(Sample *, rmtU32 *)\0"))
                .as_ptr(),
        );
    }
    'c_15216: {
        if !nb_samples.is_null() {} else {
            __assert_fail(
                b"nb_samples != NULL\0" as *const u8 as *const libc::c_char,
                b"lib/Remotery.c\0" as *const u8 as *const libc::c_char,
                6166 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 44],
                    &[libc::c_char; 44],
                >(b"void PostProcessSamples(Sample *, rmtU32 *)\0"))
                    .as_ptr(),
            );
        }
    };
    *nb_samples = (*nb_samples).wrapping_add(1);
    *nb_samples;
    let mut h: libc::c_double = (*sample).name_hash as libc::c_double
        / 0xffffffff as libc::c_uint as libc::c_double;
    let mut r: libc::c_double = saturate(
        fabs(
            fmod(
                h * 6 as libc::c_int as libc::c_double
                    + 0 as libc::c_int as libc::c_double,
                6 as libc::c_int as libc::c_double,
            ) - 3 as libc::c_int as libc::c_double,
        ) - 1 as libc::c_int as libc::c_double,
    );
    let mut g: libc::c_double = saturate(
        fabs(
            fmod(
                h * 6 as libc::c_int as libc::c_double
                    + 4 as libc::c_int as libc::c_double,
                6 as libc::c_int as libc::c_double,
            ) - 3 as libc::c_int as libc::c_double,
        ) - 1 as libc::c_int as libc::c_double,
    );
    let mut b: libc::c_double = saturate(
        fabs(
            fmod(
                h * 6 as libc::c_int as libc::c_double
                    + 2 as libc::c_int as libc::c_double,
                6 as libc::c_int as libc::c_double,
            ) - 3 as libc::c_int as libc::c_double,
        ) - 1 as libc::c_int as libc::c_double,
    );
    r = r * r
        * (3 as libc::c_int as libc::c_double - 2 as libc::c_int as libc::c_double * r);
    g = g * g
        * (3 as libc::c_int as libc::c_double - 2 as libc::c_int as libc::c_double * g);
    b = b * b
        * (3 as libc::c_int as libc::c_double - 2 as libc::c_int as libc::c_double * b);
    let mut k: libc::c_double = 0.4f64;
    r = r * k + (1 as libc::c_int as libc::c_double - k);
    g = g * k + (1 as libc::c_int as libc::c_double - k);
    b = b * k + (1 as libc::c_int as libc::c_double - k);
    (*sample)
        .uniqueColour[0 as libc::c_int
        as usize] = maxS32(
        minS32((r * 255 as libc::c_int as libc::c_double) as rmtS32, 255 as libc::c_int),
        0 as libc::c_int,
    ) as rmtU8;
    (*sample)
        .uniqueColour[1 as libc::c_int
        as usize] = maxS32(
        minS32((g * 255 as libc::c_int as libc::c_double) as rmtS32, 255 as libc::c_int),
        0 as libc::c_int,
    ) as rmtU8;
    (*sample)
        .uniqueColour[2 as libc::c_int
        as usize] = maxS32(
        minS32((b * 255 as libc::c_int as libc::c_double) as rmtS32, 255 as libc::c_int),
        0 as libc::c_int,
    ) as rmtU8;
    child = (*sample).first_child;
    while !child.is_null() {
        PostProcessSamples(child, nb_samples);
        child = (*child).next_sibling;
    }
}
unsafe extern "C" fn Remotery_SendLogTextMessage(
    mut rmt: *mut Remotery,
    mut message: *mut Message,
) -> rmtError {
    let mut error: rmtError = RMT_ERROR_NONE;
    let mut bin_buf: *mut Buffer = 0 as *mut Buffer;
    let mut write_start_offset: rmtU32 = 0;
    if !rmt.is_null() {} else {
        __assert_fail(
            b"rmt != NULL\0" as *const u8 as *const libc::c_char,
            b"lib/Remotery.c\0" as *const u8 as *const libc::c_char,
            6213 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 60],
                &[libc::c_char; 60],
            >(b"rmtError Remotery_SendLogTextMessage(Remotery *, Message *)\0"))
                .as_ptr(),
        );
    }
    'c_15993: {
        if !rmt.is_null() {} else {
            __assert_fail(
                b"rmt != NULL\0" as *const u8 as *const libc::c_char,
                b"lib/Remotery.c\0" as *const u8 as *const libc::c_char,
                6213 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 60],
                    &[libc::c_char; 60],
                >(b"rmtError Remotery_SendLogTextMessage(Remotery *, Message *)\0"))
                    .as_ptr(),
            );
        }
    };
    if !message.is_null() {} else {
        __assert_fail(
            b"message != NULL\0" as *const u8 as *const libc::c_char,
            b"lib/Remotery.c\0" as *const u8 as *const libc::c_char,
            6214 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 60],
                &[libc::c_char; 60],
            >(b"rmtError Remotery_SendLogTextMessage(Remotery *, Message *)\0"))
                .as_ptr(),
        );
    }
    'c_15951: {
        if !message.is_null() {} else {
            __assert_fail(
                b"message != NULL\0" as *const u8 as *const libc::c_char,
                b"lib/Remotery.c\0" as *const u8 as *const libc::c_char,
                6214 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 60],
                    &[libc::c_char; 60],
                >(b"rmtError Remotery_SendLogTextMessage(Remotery *, Message *)\0"))
                    .as_ptr(),
            );
        }
    };
    bin_buf = (*(*rmt).server).bin_buf;
    WebSocket_PrepareBuffer(bin_buf);
    let mut error_0: rmtError = bin_MessageHeader(
        bin_buf,
        b"LOGM\0" as *const u8 as *const libc::c_char,
        &mut write_start_offset,
    );
    if error_0 as libc::c_uint != RMT_ERROR_NONE as libc::c_int as libc::c_uint {
        return error_0;
    }
    let mut error_1: rmtError = Buffer_Write(
        bin_buf,
        ((*message).payload).as_mut_ptr() as *const libc::c_void,
        (*message).payload_size,
    );
    if error_1 as libc::c_uint != RMT_ERROR_NONE as libc::c_int as libc::c_uint {
        return error_1;
    }
    let mut error_2: rmtError = bin_MessageFooter(bin_buf, write_start_offset);
    if error_2 as libc::c_uint != RMT_ERROR_NONE as libc::c_int as libc::c_uint {
        return error_2;
    }
    if Server_IsClientConnected((*rmt).server) == 1 as libc::c_int as rmtBool {
        error = Server_Send(
            (*rmt).server,
            (*bin_buf).data as *const libc::c_void,
            (*bin_buf).bytes_used,
            20 as libc::c_int as rmtU32,
        );
    }
    if !((*rmt).logfile).is_null() {
        rmtWriteFile(
            (*rmt).logfile,
            ((*bin_buf).data).offset(10 as libc::c_int as isize) as *const libc::c_void,
            ((*bin_buf).bytes_used).wrapping_sub(10 as libc::c_int as libc::c_uint),
        );
    }
    return error;
}
unsafe extern "C" fn bin_SampleName(
    mut buffer: *mut Buffer,
    mut name: *const libc::c_char,
    mut name_hash: rmtU32,
    mut name_length: rmtU32,
) -> rmtError {
    let mut write_start_offset: rmtU32 = 0;
    let mut error: rmtError = bin_MessageHeader(
        buffer,
        b"SSMP\0" as *const u8 as *const libc::c_char,
        &mut write_start_offset,
    );
    if error as libc::c_uint != RMT_ERROR_NONE as libc::c_int as libc::c_uint {
        return error;
    }
    let mut error_0: rmtError = Buffer_WriteU32(buffer, name_hash);
    if error_0 as libc::c_uint != RMT_ERROR_NONE as libc::c_int as libc::c_uint {
        return error_0;
    }
    let mut error_1: rmtError = Buffer_WriteU32(buffer, name_length);
    if error_1 as libc::c_uint != RMT_ERROR_NONE as libc::c_int as libc::c_uint {
        return error_1;
    }
    let mut error_2: rmtError = Buffer_Write(
        buffer,
        name as *mut libc::c_void,
        name_length,
    );
    if error_2 as libc::c_uint != RMT_ERROR_NONE as libc::c_int as libc::c_uint {
        return error_2;
    }
    let mut error_3: rmtError = bin_MessageFooter(buffer, write_start_offset);
    if error_3 as libc::c_uint != RMT_ERROR_NONE as libc::c_int as libc::c_uint {
        return error_3;
    }
    return RMT_ERROR_NONE;
}
unsafe extern "C" fn Remotery_AddToStringTable(
    mut rmt: *mut Remotery,
    mut message: *mut Message,
) -> rmtError {
    let mut payload: *mut Msg_AddToStringTable = ((*message).payload).as_mut_ptr()
        as *mut Msg_AddToStringTable;
    let mut name: *const libc::c_char = payload.offset(1 as libc::c_int as isize)
        as *const libc::c_char;
    let mut name_inserted: rmtBool = StringTable_Insert(
        (*rmt).string_table,
        (*payload).hash,
        name,
    );
    if name_inserted == 1 as libc::c_int as rmtBool && !((*rmt).logfile).is_null() {
        let mut bin_buf: *mut Buffer = (*(*rmt).server).bin_buf;
        (*bin_buf).bytes_used = 0 as libc::c_int as rmtU32;
        let mut error: rmtError = bin_SampleName(
            bin_buf,
            name,
            (*payload).hash,
            (*payload).length,
        );
        if error as libc::c_uint != RMT_ERROR_NONE as libc::c_int as libc::c_uint {
            return error;
        }
        rmtWriteFile(
            (*rmt).logfile,
            (*bin_buf).data as *const libc::c_void,
            (*bin_buf).bytes_used,
        );
    }
    return RMT_ERROR_NONE;
}
unsafe extern "C" fn bin_SampleTree(
    mut buffer: *mut Buffer,
    mut msg: *mut Msg_SampleTree,
) -> rmtError {
    let mut root_sample: *mut Sample = 0 as *mut Sample;
    let mut thread_name: [libc::c_char; 256] = [0; 256];
    let mut nb_samples: rmtU32 = 0 as libc::c_int as rmtU32;
    let mut write_start_offset: rmtU32 = 0 as libc::c_int as rmtU32;
    if !buffer.is_null() {} else {
        __assert_fail(
            b"buffer != NULL\0" as *const u8 as *const libc::c_char,
            b"lib/Remotery.c\0" as *const u8 as *const libc::c_char,
            6273 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 52],
                &[libc::c_char; 52],
            >(b"rmtError bin_SampleTree(Buffer *, Msg_SampleTree *)\0"))
                .as_ptr(),
        );
    }
    'c_15539: {
        if !buffer.is_null() {} else {
            __assert_fail(
                b"buffer != NULL\0" as *const u8 as *const libc::c_char,
                b"lib/Remotery.c\0" as *const u8 as *const libc::c_char,
                6273 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 52],
                    &[libc::c_char; 52],
                >(b"rmtError bin_SampleTree(Buffer *, Msg_SampleTree *)\0"))
                    .as_ptr(),
            );
        }
    };
    if !msg.is_null() {} else {
        __assert_fail(
            b"msg != NULL\0" as *const u8 as *const libc::c_char,
            b"lib/Remotery.c\0" as *const u8 as *const libc::c_char,
            6274 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 52],
                &[libc::c_char; 52],
            >(b"rmtError bin_SampleTree(Buffer *, Msg_SampleTree *)\0"))
                .as_ptr(),
        );
    }
    'c_15497: {
        if !msg.is_null() {} else {
            __assert_fail(
                b"msg != NULL\0" as *const u8 as *const libc::c_char,
                b"lib/Remotery.c\0" as *const u8 as *const libc::c_char,
                6274 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 52],
                    &[libc::c_char; 52],
                >(b"rmtError bin_SampleTree(Buffer *, Msg_SampleTree *)\0"))
                    .as_ptr(),
            );
        }
    };
    root_sample = (*msg).rootSample;
    if !root_sample.is_null() {} else {
        __assert_fail(
            b"root_sample != NULL\0" as *const u8 as *const libc::c_char,
            b"lib/Remotery.c\0" as *const u8 as *const libc::c_char,
            6278 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 52],
                &[libc::c_char; 52],
            >(b"rmtError bin_SampleTree(Buffer *, Msg_SampleTree *)\0"))
                .as_ptr(),
        );
    }
    'c_15447: {
        if !root_sample.is_null() {} else {
            __assert_fail(
                b"root_sample != NULL\0" as *const u8 as *const libc::c_char,
                b"lib/Remotery.c\0" as *const u8 as *const libc::c_char,
                6278 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 52],
                    &[libc::c_char; 52],
                >(b"rmtError bin_SampleTree(Buffer *, Msg_SampleTree *)\0"))
                    .as_ptr(),
            );
        }
    };
    thread_name[0 as libc::c_int as usize] = 0 as libc::c_int as libc::c_char;
    strncat_s_safe_c(
        thread_name.as_mut_ptr(),
        ::std::mem::size_of::<[libc::c_char; 256]>() as libc::c_ulong as r_size_t,
        (*msg).threadName,
        strnlen_s_safe_c((*msg).threadName, 255 as libc::c_int as r_size_t),
    );
    if (*root_sample).type_0 as libc::c_uint
        == RMT_SampleType_CUDA as libc::c_int as libc::c_uint
    {
        strncat_s_safe_c(
            thread_name.as_mut_ptr(),
            ::std::mem::size_of::<[libc::c_char; 256]>() as libc::c_ulong as r_size_t,
            b" (CUDA)\0" as *const u8 as *const libc::c_char,
            7 as libc::c_int as r_size_t,
        );
    }
    if (*root_sample).type_0 as libc::c_uint
        == RMT_SampleType_D3D11 as libc::c_int as libc::c_uint
    {
        strncat_s_safe_c(
            thread_name.as_mut_ptr(),
            ::std::mem::size_of::<[libc::c_char; 256]>() as libc::c_ulong as r_size_t,
            b" (D3D11)\0" as *const u8 as *const libc::c_char,
            8 as libc::c_int as r_size_t,
        );
    }
    if (*root_sample).type_0 as libc::c_uint
        == RMT_SampleType_D3D12 as libc::c_int as libc::c_uint
    {
        strncat_s_safe_c(
            thread_name.as_mut_ptr(),
            ::std::mem::size_of::<[libc::c_char; 256]>() as libc::c_ulong as r_size_t,
            b" (D3D12)\0" as *const u8 as *const libc::c_char,
            8 as libc::c_int as r_size_t,
        );
    }
    if (*root_sample).type_0 as libc::c_uint
        == RMT_SampleType_OpenGL as libc::c_int as libc::c_uint
    {
        strncat_s_safe_c(
            thread_name.as_mut_ptr(),
            ::std::mem::size_of::<[libc::c_char; 256]>() as libc::c_ulong as r_size_t,
            b" (OpenGL)\0" as *const u8 as *const libc::c_char,
            9 as libc::c_int as r_size_t,
        );
    }
    if (*root_sample).type_0 as libc::c_uint
        == RMT_SampleType_Metal as libc::c_int as libc::c_uint
    {
        strncat_s_safe_c(
            thread_name.as_mut_ptr(),
            ::std::mem::size_of::<[libc::c_char; 256]>() as libc::c_ulong as r_size_t,
            b" (Metal)\0" as *const u8 as *const libc::c_char,
            8 as libc::c_int as r_size_t,
        );
    }
    PostProcessSamples(root_sample, &mut nb_samples);
    let mut error: rmtError = bin_MessageHeader(
        buffer,
        b"SMPL\0" as *const u8 as *const libc::c_char,
        &mut write_start_offset,
    );
    if error as libc::c_uint != RMT_ERROR_NONE as libc::c_int as libc::c_uint {
        return error;
    }
    let mut error_0: rmtError = Buffer_WriteStringWithLength(
        buffer,
        thread_name.as_mut_ptr() as rmtPStr,
    );
    if error_0 as libc::c_uint != RMT_ERROR_NONE as libc::c_int as libc::c_uint {
        return error_0;
    }
    let mut error_1: rmtError = Buffer_WriteU32(buffer, nb_samples);
    if error_1 as libc::c_uint != RMT_ERROR_NONE as libc::c_int as libc::c_uint {
        return error_1;
    }
    let mut error_2: rmtError = Buffer_WriteU32(
        buffer,
        (if (*msg).partialTree != 0 { 1 as libc::c_int } else { 0 as libc::c_int })
            as rmtU32,
    );
    if error_2 as libc::c_uint != RMT_ERROR_NONE as libc::c_int as libc::c_uint {
        return error_2;
    }
    let mut error_3: rmtError = Buffer_AlignedPad(buffer, write_start_offset);
    if error_3 as libc::c_uint != RMT_ERROR_NONE as libc::c_int as libc::c_uint {
        return error_3;
    }
    let mut error_4: rmtError = bin_Sample(
        buffer,
        root_sample,
        0 as libc::c_int as rmtU8,
    );
    if error_4 as libc::c_uint != RMT_ERROR_NONE as libc::c_int as libc::c_uint {
        return error_4;
    }
    let mut error_5: rmtError = bin_MessageFooter(buffer, write_start_offset);
    if error_5 as libc::c_uint != RMT_ERROR_NONE as libc::c_int as libc::c_uint {
        return error_5;
    }
    return RMT_ERROR_NONE;
}
unsafe extern "C" fn Remotery_SendToViewerAndLog(
    mut rmt: *mut Remotery,
    mut bin_buf: *mut Buffer,
    mut timeout: rmtU32,
) -> rmtError {
    let mut error: rmtError = RMT_ERROR_NONE;
    if Server_IsClientConnected((*rmt).server) == 1 as libc::c_int as rmtBool {
        static mut rmt_sample_hash_Server_Send: rmtU32 = 0 as libc::c_int as rmtU32;
        _rmt_BeginCPUSample(
            b"Server_Send\0" as *const u8 as *const libc::c_char,
            RMTSF_Aggregate as libc::c_int as rmtU32,
            &mut rmt_sample_hash_Server_Send,
        );
        error = Server_Send(
            (*rmt).server,
            (*bin_buf).data as *const libc::c_void,
            (*bin_buf).bytes_used,
            timeout,
        );
        _rmt_EndCPUSample();
    }
    if !((*rmt).logfile).is_null() {
        rmtWriteFile(
            (*rmt).logfile,
            ((*bin_buf).data).offset(10 as libc::c_int as isize) as *const libc::c_void,
            ((*bin_buf).bytes_used).wrapping_sub(10 as libc::c_int as libc::c_uint),
        );
    }
    return error;
}
unsafe extern "C" fn Remotery_SendSampleTreeMessage(
    mut rmt: *mut Remotery,
    mut message: *mut Message,
) -> rmtError {
    let mut error: rmtError = RMT_ERROR_NONE;
    let mut sample_tree: *mut Msg_SampleTree = 0 as *mut Msg_SampleTree;
    let mut sample: *mut Sample = 0 as *mut Sample;
    let mut bin_buf: *mut Buffer = 0 as *mut Buffer;
    if !rmt.is_null() {} else {
        __assert_fail(
            b"rmt != NULL\0" as *const u8 as *const libc::c_char,
            b"lib/Remotery.c\0" as *const u8 as *const libc::c_char,
            6357 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 63],
                &[libc::c_char; 63],
            >(b"rmtError Remotery_SendSampleTreeMessage(Remotery *, Message *)\0"))
                .as_ptr(),
        );
    }
    'c_15706: {
        if !rmt.is_null() {} else {
            __assert_fail(
                b"rmt != NULL\0" as *const u8 as *const libc::c_char,
                b"lib/Remotery.c\0" as *const u8 as *const libc::c_char,
                6357 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 63],
                    &[libc::c_char; 63],
                >(b"rmtError Remotery_SendSampleTreeMessage(Remotery *, Message *)\0"))
                    .as_ptr(),
            );
        }
    };
    if !message.is_null() {} else {
        __assert_fail(
            b"message != NULL\0" as *const u8 as *const libc::c_char,
            b"lib/Remotery.c\0" as *const u8 as *const libc::c_char,
            6358 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 63],
                &[libc::c_char; 63],
            >(b"rmtError Remotery_SendSampleTreeMessage(Remotery *, Message *)\0"))
                .as_ptr(),
        );
    }
    'c_15664: {
        if !message.is_null() {} else {
            __assert_fail(
                b"message != NULL\0" as *const u8 as *const libc::c_char,
                b"lib/Remotery.c\0" as *const u8 as *const libc::c_char,
                6358 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 63],
                    &[libc::c_char; 63],
                >(b"rmtError Remotery_SendSampleTreeMessage(Remotery *, Message *)\0"))
                    .as_ptr(),
            );
        }
    };
    sample_tree = ((*message).payload).as_mut_ptr() as *mut Msg_SampleTree;
    sample = (*sample_tree).rootSample;
    if !sample.is_null() {} else {
        __assert_fail(
            b"sample != NULL\0" as *const u8 as *const libc::c_char,
            b"lib/Remotery.c\0" as *const u8 as *const libc::c_char,
            6363 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 63],
                &[libc::c_char; 63],
            >(b"rmtError Remotery_SendSampleTreeMessage(Remotery *, Message *)\0"))
                .as_ptr(),
        );
    }
    'c_15608: {
        if !sample.is_null() {} else {
            __assert_fail(
                b"sample != NULL\0" as *const u8 as *const libc::c_char,
                b"lib/Remotery.c\0" as *const u8 as *const libc::c_char,
                6363 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 63],
                    &[libc::c_char; 63],
                >(b"rmtError Remotery_SendSampleTreeMessage(Remotery *, Message *)\0"))
                    .as_ptr(),
            );
        }
    };
    bin_buf = (*(*rmt).server).bin_buf;
    WebSocket_PrepareBuffer(bin_buf);
    static mut rmt_sample_hash_bin_SampleTree: rmtU32 = 0 as libc::c_int as rmtU32;
    _rmt_BeginCPUSample(
        b"bin_SampleTree\0" as *const u8 as *const libc::c_char,
        RMTSF_Aggregate as libc::c_int as rmtU32,
        &mut rmt_sample_hash_bin_SampleTree,
    );
    error = bin_SampleTree(bin_buf, sample_tree);
    _rmt_EndCPUSample();
    if (g_Settings.sampletree_handler).is_some() {
        (g_Settings.sampletree_handler)
            .unwrap()(g_Settings.sampletree_context, sample_tree);
    }
    FreeSamples(sample, (*sample_tree).allocator);
    if error as libc::c_uint != RMT_ERROR_NONE as libc::c_int as libc::c_uint {
        return error;
    }
    return Remotery_SendToViewerAndLog(rmt, bin_buf, 50000 as libc::c_int as rmtU32);
}
unsafe extern "C" fn Remotery_SendProcessorThreads(
    mut rmt: *mut Remotery,
    mut message: *mut Message,
) -> rmtError {
    let mut processor_index: rmtU32 = 0;
    let mut error: rmtError = RMT_ERROR_NONE;
    let mut processor_threads: *mut Msg_ProcessorThreads = ((*message).payload)
        .as_mut_ptr() as *mut Msg_ProcessorThreads;
    let mut bin_buf: *mut Buffer = 0 as *mut Buffer;
    let mut write_start_offset: rmtU32 = 0;
    bin_buf = (*(*rmt).server).bin_buf;
    WebSocket_PrepareBuffer(bin_buf);
    let mut error_0: rmtError = bin_MessageHeader(
        bin_buf,
        b"PRTH\0" as *const u8 as *const libc::c_char,
        &mut write_start_offset,
    );
    if error_0 as libc::c_uint != RMT_ERROR_NONE as libc::c_int as libc::c_uint {
        return error_0;
    }
    let mut error_1: rmtError = Buffer_WriteU32(
        bin_buf,
        (*processor_threads).nbProcessors,
    );
    if error_1 as libc::c_uint != RMT_ERROR_NONE as libc::c_int as libc::c_uint {
        return error_1;
    }
    let mut error_2: rmtError = Buffer_WriteU64(
        bin_buf,
        (*processor_threads).messageIndex,
    );
    if error_2 as libc::c_uint != RMT_ERROR_NONE as libc::c_int as libc::c_uint {
        return error_2;
    }
    processor_index = 0 as libc::c_int as rmtU32;
    while processor_index < (*processor_threads).nbProcessors {
        let mut processor: *mut Processor = ((*processor_threads).processors)
            .as_mut_ptr()
            .offset(processor_index as isize);
        if !((*processor).threadProfiler).is_null() {
            let mut error_3: rmtError = Buffer_WriteU32(
                bin_buf,
                (*(*processor).threadProfiler).threadId as rmtU32,
            );
            if error_3 as libc::c_uint != RMT_ERROR_NONE as libc::c_int as libc::c_uint {
                return error_3;
            }
            let mut error_4: rmtError = Buffer_WriteU32(
                bin_buf,
                (*(*processor).threadProfiler).threadNameHash,
            );
            if error_4 as libc::c_uint != RMT_ERROR_NONE as libc::c_int as libc::c_uint {
                return error_4;
            }
            let mut error_5: rmtError = Buffer_WriteU64(
                bin_buf,
                (*processor).sampleTime,
            );
            if error_5 as libc::c_uint != RMT_ERROR_NONE as libc::c_int as libc::c_uint {
                return error_5;
            }
        } else {
            let mut error_6: rmtError = Buffer_WriteU32(
                bin_buf,
                -(1 as libc::c_int) as rmtU32,
            );
            if error_6 as libc::c_uint != RMT_ERROR_NONE as libc::c_int as libc::c_uint {
                return error_6;
            }
            let mut error_7: rmtError = Buffer_WriteU32(
                bin_buf,
                0 as libc::c_int as rmtU32,
            );
            if error_7 as libc::c_uint != RMT_ERROR_NONE as libc::c_int as libc::c_uint {
                return error_7;
            }
            let mut error_8: rmtError = Buffer_WriteU64(
                bin_buf,
                0 as libc::c_int as rmtU64,
            );
            if error_8 as libc::c_uint != RMT_ERROR_NONE as libc::c_int as libc::c_uint {
                return error_8;
            }
        }
        processor_index = processor_index.wrapping_add(1);
        processor_index;
    }
    let mut error_9: rmtError = bin_MessageFooter(bin_buf, write_start_offset);
    if error_9 as libc::c_uint != RMT_ERROR_NONE as libc::c_int as libc::c_uint {
        return error_9;
    }
    return Remotery_SendToViewerAndLog(rmt, bin_buf, 50 as libc::c_int as rmtU32);
}
unsafe extern "C" fn FreePropertySnapshots(mut snapshot: *mut PropertySnapshot) {
    if snapshot.is_null() {
        return;
    }
    if !((*snapshot).nextSnapshot).is_null() {
        FreePropertySnapshots((*snapshot).nextSnapshot);
    }
    ObjectAllocator_Free((*g_Remotery).propertyAllocator, snapshot as *mut libc::c_void);
}
unsafe extern "C" fn Remotery_SerialisePropertySnapshots(
    mut bin_buf: *mut Buffer,
    mut msg_snapshot: *mut Msg_PropertySnapshot,
) -> rmtError {
    let mut snapshot: *mut PropertySnapshot = 0 as *mut PropertySnapshot;
    let mut empty_group: [rmtU8; 16] = [
        0 as libc::c_int as rmtU8,
        0 as libc::c_int as rmtU8,
        0 as libc::c_int as rmtU8,
        0 as libc::c_int as rmtU8,
        0 as libc::c_int as rmtU8,
        0 as libc::c_int as rmtU8,
        0 as libc::c_int as rmtU8,
        0 as libc::c_int as rmtU8,
        0 as libc::c_int as rmtU8,
        0 as libc::c_int as rmtU8,
        0 as libc::c_int as rmtU8,
        0 as libc::c_int as rmtU8,
        0 as libc::c_int as rmtU8,
        0 as libc::c_int as rmtU8,
        0 as libc::c_int as rmtU8,
        0 as libc::c_int as rmtU8,
    ];
    let mut write_start_offset: rmtU32 = 0;
    let mut error: rmtError = bin_MessageHeader(
        bin_buf,
        b"PSNP\0" as *const u8 as *const libc::c_char,
        &mut write_start_offset,
    );
    if error as libc::c_uint != RMT_ERROR_NONE as libc::c_int as libc::c_uint {
        return error;
    }
    let mut error_0: rmtError = Buffer_WriteU32(bin_buf, (*msg_snapshot).nbSnapshots);
    if error_0 as libc::c_uint != RMT_ERROR_NONE as libc::c_int as libc::c_uint {
        return error_0;
    }
    let mut error_1: rmtError = Buffer_WriteU32(bin_buf, (*msg_snapshot).propertyFrame);
    if error_1 as libc::c_uint != RMT_ERROR_NONE as libc::c_int as libc::c_uint {
        return error_1;
    }
    snapshot = (*msg_snapshot).rootSnapshot;
    while !snapshot.is_null() {
        let mut colour_depth: [rmtU8; 4] = [
            0 as libc::c_int as rmtU8,
            0 as libc::c_int as rmtU8,
            0 as libc::c_int as rmtU8,
            0,
        ];
        let mut error_2: rmtError = Buffer_WriteU32(bin_buf, (*snapshot).nameHash);
        if error_2 as libc::c_uint != RMT_ERROR_NONE as libc::c_int as libc::c_uint {
            return error_2;
        }
        let mut error_3: rmtError = Buffer_WriteU32(bin_buf, (*snapshot).uniqueID);
        if error_3 as libc::c_uint != RMT_ERROR_NONE as libc::c_int as libc::c_uint {
            return error_3;
        }
        colour_depth[3 as libc::c_int as usize] = (*snapshot).depth;
        let mut error_4: rmtError = Buffer_Write(
            bin_buf,
            colour_depth.as_mut_ptr() as *const libc::c_void,
            4 as libc::c_int as rmtU32,
        );
        if error_4 as libc::c_uint != RMT_ERROR_NONE as libc::c_int as libc::c_uint {
            return error_4;
        }
        let mut error_5: rmtError = Buffer_WriteU32(
            bin_buf,
            (*snapshot).type_0 as rmtU32,
        );
        if error_5 as libc::c_uint != RMT_ERROR_NONE as libc::c_int as libc::c_uint {
            return error_5;
        }
        match (*snapshot).type_0 as libc::c_uint {
            0 => {
                let mut error_6: rmtError = Buffer_Write(
                    bin_buf,
                    empty_group.as_mut_ptr() as *const libc::c_void,
                    16 as libc::c_int as rmtU32,
                );
                if error_6 as libc::c_uint
                    != RMT_ERROR_NONE as libc::c_int as libc::c_uint
                {
                    return error_6;
                }
            }
            1 => {
                let mut error_7: rmtError = Buffer_WriteF64(
                    bin_buf,
                    (*snapshot).value.Bool as rmtF64,
                );
                if error_7 as libc::c_uint
                    != RMT_ERROR_NONE as libc::c_int as libc::c_uint
                {
                    return error_7;
                }
                let mut error_8: rmtError = Buffer_WriteF64(
                    bin_buf,
                    (*snapshot).prevValue.Bool as rmtF64,
                );
                if error_8 as libc::c_uint
                    != RMT_ERROR_NONE as libc::c_int as libc::c_uint
                {
                    return error_8;
                }
            }
            2 => {
                let mut error_9: rmtError = Buffer_WriteF64(
                    bin_buf,
                    (*snapshot).value.S32 as rmtF64,
                );
                if error_9 as libc::c_uint
                    != RMT_ERROR_NONE as libc::c_int as libc::c_uint
                {
                    return error_9;
                }
                let mut error_10: rmtError = Buffer_WriteF64(
                    bin_buf,
                    (*snapshot).prevValue.S32 as rmtF64,
                );
                if error_10 as libc::c_uint
                    != RMT_ERROR_NONE as libc::c_int as libc::c_uint
                {
                    return error_10;
                }
            }
            3 => {
                let mut error_11: rmtError = Buffer_WriteF64(
                    bin_buf,
                    (*snapshot).value.U32 as rmtF64,
                );
                if error_11 as libc::c_uint
                    != RMT_ERROR_NONE as libc::c_int as libc::c_uint
                {
                    return error_11;
                }
                let mut error_12: rmtError = Buffer_WriteF64(
                    bin_buf,
                    (*snapshot).prevValue.U32 as rmtF64,
                );
                if error_12 as libc::c_uint
                    != RMT_ERROR_NONE as libc::c_int as libc::c_uint
                {
                    return error_12;
                }
            }
            4 => {
                let mut error_13: rmtError = Buffer_WriteF64(
                    bin_buf,
                    (*snapshot).value.F32 as rmtF64,
                );
                if error_13 as libc::c_uint
                    != RMT_ERROR_NONE as libc::c_int as libc::c_uint
                {
                    return error_13;
                }
                let mut error_14: rmtError = Buffer_WriteF64(
                    bin_buf,
                    (*snapshot).prevValue.F32 as rmtF64,
                );
                if error_14 as libc::c_uint
                    != RMT_ERROR_NONE as libc::c_int as libc::c_uint
                {
                    return error_14;
                }
            }
            5 | 6 => {
                let mut error_15: rmtError = Buffer_WriteU64(
                    bin_buf,
                    (*snapshot).value.U64,
                );
                if error_15 as libc::c_uint
                    != RMT_ERROR_NONE as libc::c_int as libc::c_uint
                {
                    return error_15;
                }
                let mut error_16: rmtError = Buffer_WriteU64(
                    bin_buf,
                    (*snapshot).prevValue.U64,
                );
                if error_16 as libc::c_uint
                    != RMT_ERROR_NONE as libc::c_int as libc::c_uint
                {
                    return error_16;
                }
            }
            7 => {
                let mut error_17: rmtError = Buffer_WriteF64(
                    bin_buf,
                    (*snapshot).value.F64,
                );
                if error_17 as libc::c_uint
                    != RMT_ERROR_NONE as libc::c_int as libc::c_uint
                {
                    return error_17;
                }
                let mut error_18: rmtError = Buffer_WriteF64(
                    bin_buf,
                    (*snapshot).prevValue.F64,
                );
                if error_18 as libc::c_uint
                    != RMT_ERROR_NONE as libc::c_int as libc::c_uint
                {
                    return error_18;
                }
            }
            _ => {}
        }
        let mut error_19: rmtError = Buffer_WriteU32(
            bin_buf,
            (*snapshot).prevValueFrame,
        );
        if error_19 as libc::c_uint != RMT_ERROR_NONE as libc::c_int as libc::c_uint {
            return error_19;
        }
        let mut error_20: rmtError = Buffer_WriteU32(bin_buf, (*snapshot).nbChildren);
        if error_20 as libc::c_uint != RMT_ERROR_NONE as libc::c_int as libc::c_uint {
            return error_20;
        }
        snapshot = (*snapshot).nextSnapshot;
    }
    let mut error_21: rmtError = bin_MessageFooter(bin_buf, write_start_offset);
    if error_21 as libc::c_uint != RMT_ERROR_NONE as libc::c_int as libc::c_uint {
        return error_21;
    }
    return RMT_ERROR_NONE;
}
unsafe extern "C" fn Remotery_SendPropertySnapshot(
    mut rmt: *mut Remotery,
    mut message: *mut Message,
) -> rmtError {
    let mut msg_snapshot: *mut Msg_PropertySnapshot = ((*message).payload).as_mut_ptr()
        as *mut Msg_PropertySnapshot;
    let mut error: rmtError = RMT_ERROR_NONE;
    let mut bin_buf: *mut Buffer = 0 as *mut Buffer;
    bin_buf = (*(*rmt).server).bin_buf;
    WebSocket_PrepareBuffer(bin_buf);
    error = Remotery_SerialisePropertySnapshots(bin_buf, msg_snapshot);
    if error as libc::c_uint == RMT_ERROR_NONE as libc::c_int as libc::c_uint {
        error = Remotery_SendToViewerAndLog(rmt, bin_buf, 50 as libc::c_int as rmtU32);
    }
    FreePropertySnapshots((*msg_snapshot).rootSnapshot);
    return error;
}
unsafe extern "C" fn Remotery_ConsumeMessageQueue(mut rmt: *mut Remotery) -> rmtError {
    let mut nb_messages_sent: rmtU32 = 0 as libc::c_int as rmtU32;
    let maxNbMessagesPerUpdate: rmtU32 = g_Settings.maxNbMessagesPerUpdate;
    if !rmt.is_null() {} else {
        __assert_fail(
            b"rmt != NULL\0" as *const u8 as *const libc::c_char,
            b"lib/Remotery.c\0" as *const u8 as *const libc::c_char,
            6572 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 50],
                &[libc::c_char; 50],
            >(b"rmtError Remotery_ConsumeMessageQueue(Remotery *)\0"))
                .as_ptr(),
        );
    }
    'c_16977: {
        if !rmt.is_null() {} else {
            __assert_fail(
                b"rmt != NULL\0" as *const u8 as *const libc::c_char,
                b"lib/Remotery.c\0" as *const u8 as *const libc::c_char,
                6572 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 50],
                    &[libc::c_char; 50],
                >(b"rmtError Remotery_ConsumeMessageQueue(Remotery *)\0"))
                    .as_ptr(),
            );
        }
    };
    while nb_messages_sent < maxNbMessagesPerUpdate {
        let mut error: rmtError = RMT_ERROR_NONE;
        let mut message: *mut Message = rmtMessageQueue_PeekNextMessage(
            (*rmt).mq_to_rmt_thread,
        );
        if message.is_null() {
            break;
        }
        match (*message).id as libc::c_uint {
            0 => {
                if 0 as libc::c_int as rmtBool != 0 {} else {
                    __assert_fail(
                        b"RMT_FALSE\0" as *const u8 as *const libc::c_char,
                        b"lib/Remotery.c\0" as *const u8 as *const libc::c_char,
                        6587 as libc::c_int as libc::c_uint,
                        (*::std::mem::transmute::<
                            &[u8; 50],
                            &[libc::c_char; 50],
                        >(b"rmtError Remotery_ConsumeMessageQueue(Remotery *)\0"))
                            .as_ptr(),
                    );
                }
                'c_16920: {
                    if 0 as libc::c_int as rmtBool != 0 {} else {
                        __assert_fail(
                            b"RMT_FALSE\0" as *const u8 as *const libc::c_char,
                            b"lib/Remotery.c\0" as *const u8 as *const libc::c_char,
                            6587 as libc::c_int as libc::c_uint,
                            (*::std::mem::transmute::<
                                &[u8; 50],
                                &[libc::c_char; 50],
                            >(b"rmtError Remotery_ConsumeMessageQueue(Remotery *)\0"))
                                .as_ptr(),
                        );
                    }
                };
            }
            1 => {
                error = Remotery_AddToStringTable(rmt, message);
            }
            2 => {
                error = Remotery_SendLogTextMessage(rmt, message);
                nb_messages_sent = nb_messages_sent.wrapping_add(1);
                nb_messages_sent;
            }
            3 => {
                static mut rmt_sample_hash_SendSampleTreeMessage: rmtU32 = 0
                    as libc::c_int as rmtU32;
                _rmt_BeginCPUSample(
                    b"SendSampleTreeMessage\0" as *const u8 as *const libc::c_char,
                    RMTSF_Aggregate as libc::c_int as rmtU32,
                    &mut rmt_sample_hash_SendSampleTreeMessage,
                );
                error = Remotery_SendSampleTreeMessage(rmt, message);
                nb_messages_sent = nb_messages_sent.wrapping_add(1);
                nb_messages_sent;
                _rmt_EndCPUSample();
            }
            4 => {
                Remotery_SendProcessorThreads(rmt, message);
                nb_messages_sent = nb_messages_sent.wrapping_add(1);
                nb_messages_sent;
            }
            6 => {
                error = Remotery_SendPropertySnapshot(rmt, message);
            }
            _ => {}
        }
        rmtMessageQueue_ConsumeNextMessage((*rmt).mq_to_rmt_thread, message);
        if error as libc::c_uint != RMT_ERROR_NONE as libc::c_int as libc::c_uint {
            return error;
        }
    }
    return RMT_ERROR_NONE;
}
unsafe extern "C" fn Remotery_FlushMessageQueue(mut rmt: *mut Remotery) {
    if !rmt.is_null() {} else {
        __assert_fail(
            b"rmt != NULL\0" as *const u8 as *const libc::c_char,
            b"lib/Remotery.c\0" as *const u8 as *const libc::c_char,
            6629 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 44],
                &[libc::c_char; 44],
            >(b"void Remotery_FlushMessageQueue(Remotery *)\0"))
                .as_ptr(),
        );
    }
    'c_4752: {
        if !rmt.is_null() {} else {
            __assert_fail(
                b"rmt != NULL\0" as *const u8 as *const libc::c_char,
                b"lib/Remotery.c\0" as *const u8 as *const libc::c_char,
                6629 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 44],
                    &[libc::c_char; 44],
                >(b"void Remotery_FlushMessageQueue(Remotery *)\0"))
                    .as_ptr(),
            );
        }
    };
    loop {
        let mut message: *mut Message = rmtMessageQueue_PeekNextMessage(
            (*rmt).mq_to_rmt_thread,
        );
        if message.is_null() {
            break;
        }
        match (*message).id as libc::c_uint {
            3 => {
                let mut sample_tree: *mut Msg_SampleTree = ((*message).payload)
                    .as_mut_ptr() as *mut Msg_SampleTree;
                FreeSamples((*sample_tree).rootSample, (*sample_tree).allocator);
            }
            6 => {
                let mut msg_snapshot: *mut Msg_PropertySnapshot = ((*message).payload)
                    .as_mut_ptr() as *mut Msg_PropertySnapshot;
                FreePropertySnapshots((*msg_snapshot).rootSnapshot);
            }
            0 | 1 | 2 | _ => {}
        }
        rmtMessageQueue_ConsumeNextMessage((*rmt).mq_to_rmt_thread, message);
    };
}
unsafe extern "C" fn Remotery_MapMessageQueue(mut rmt: *mut Remotery) {
    let mut read_pos: rmtU32 = 0;
    let mut write_pos: rmtU32 = 0;
    let mut queue: *mut rmtMessageQueue = 0 as *mut rmtMessageQueue;
    if !rmt.is_null() {} else {
        __assert_fail(
            b"rmt != NULL\0" as *const u8 as *const libc::c_char,
            b"lib/Remotery.c\0" as *const u8 as *const libc::c_char,
            6672 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 42],
                &[libc::c_char; 42],
            >(b"void Remotery_MapMessageQueue(Remotery *)\0"))
                .as_ptr(),
        );
    }
    'c_5016: {
        if !rmt.is_null() {} else {
            __assert_fail(
                b"rmt != NULL\0" as *const u8 as *const libc::c_char,
                b"lib/Remotery.c\0" as *const u8 as *const libc::c_char,
                6672 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 42],
                    &[libc::c_char; 42],
                >(b"void Remotery_MapMessageQueue(Remotery *)\0"))
                    .as_ptr(),
            );
        }
    };
    while (LoadAcquirePointer(
        &mut (*rmt).map_message_queue_data as *mut *mut libc::c_void
            as *mut *mut libc::c_long,
    ))
        .is_null()
    {
        msSleep(1 as libc::c_int as rmtU32);
    }
    queue = (*rmt).mq_to_rmt_thread;
    write_pos = LoadAcquire(&mut (*queue).write_pos);
    read_pos = (*queue).read_pos;
    while read_pos < write_pos {
        let mut r: rmtU32 = read_pos
            & ((*queue).size).wrapping_sub(1 as libc::c_int as libc::c_uint);
        let mut message: *mut Message = ((*(*queue).data).ptr).offset(r as isize)
            as *mut Message;
        let mut message_size: rmtU32 = rmtMessageQueue_SizeForPayload(
            (*message).payload_size,
        );
        ((*rmt).map_message_queue_fn).unwrap()(rmt, message);
        read_pos = (read_pos as libc::c_uint).wrapping_add(message_size) as rmtU32
            as rmtU32;
    }
    StoreReleasePointer(
        &mut (*rmt).map_message_queue_data as *mut *mut libc::c_void
            as *mut *mut libc::c_long,
        0 as *mut libc::c_long,
    );
}
unsafe extern "C" fn Remotery_ThreadMain(mut thread: *mut rmtThread) -> rmtError {
    let mut rmt: *mut Remotery = (*thread).param as *mut Remotery;
    if !rmt.is_null() {} else {
        __assert_fail(
            b"rmt != NULL\0" as *const u8 as *const libc::c_char,
            b"lib/Remotery.c\0" as *const u8 as *const libc::c_char,
            6700 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 42],
                &[libc::c_char; 42],
            >(b"rmtError Remotery_ThreadMain(rmtThread *)\0"))
                .as_ptr(),
        );
    }
    'c_22734: {
        if !rmt.is_null() {} else {
            __assert_fail(
                b"rmt != NULL\0" as *const u8 as *const libc::c_char,
                b"lib/Remotery.c\0" as *const u8 as *const libc::c_char,
                6700 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 42],
                    &[libc::c_char; 42],
                >(b"rmtError Remotery_ThreadMain(rmtThread *)\0"))
                    .as_ptr(),
            );
        }
    };
    _rmt_SetCurrentThreadName(b"Remotery\0" as *const u8 as *const libc::c_char);
    while (*thread).request_exit == 0 as libc::c_int as rmtBool {
        static mut rmt_sample_hash_Wakeup: rmtU32 = 0 as libc::c_int as rmtU32;
        _rmt_BeginCPUSample(
            b"Wakeup\0" as *const u8 as *const libc::c_char,
            0 as libc::c_int as rmtU32,
            &mut rmt_sample_hash_Wakeup,
        );
        static mut rmt_sample_hash_ServerUpdate: rmtU32 = 0 as libc::c_int as rmtU32;
        _rmt_BeginCPUSample(
            b"ServerUpdate\0" as *const u8 as *const libc::c_char,
            0 as libc::c_int as rmtU32,
            &mut rmt_sample_hash_ServerUpdate,
        );
        Server_Update((*rmt).server);
        _rmt_EndCPUSample();
        static mut rmt_sample_hash_ConsumeMessageQueue: rmtU32 = 0 as libc::c_int
            as rmtU32;
        _rmt_BeginCPUSample(
            b"ConsumeMessageQueue\0" as *const u8 as *const libc::c_char,
            0 as libc::c_int as rmtU32,
            &mut rmt_sample_hash_ConsumeMessageQueue,
        );
        Remotery_ConsumeMessageQueue(rmt);
        _rmt_EndCPUSample();
        _rmt_EndCPUSample();
        if !(LoadAcquirePointer(
            &mut (*rmt).map_message_queue_fn
                as *mut Option::<unsafe extern "C" fn(*mut Remotery, *mut Message) -> ()>
                as *mut *mut libc::c_long,
        ))
            .is_null()
        {
            Remotery_MapMessageQueue(rmt);
            StoreReleasePointer(
                &mut (*rmt).map_message_queue_fn
                    as *mut Option::<
                        unsafe extern "C" fn(*mut Remotery, *mut Message) -> (),
                    > as *mut *mut libc::c_long,
                0 as *mut libc::c_long,
            );
        }
        msSleep(g_Settings.msSleepBetweenServerUpdates);
    }
    Remotery_FlushMessageQueue(rmt);
    return RMT_ERROR_NONE;
}
unsafe extern "C" fn Remotery_ReceiveMessage(
    mut context: *mut libc::c_void,
    mut message_data: *mut libc::c_char,
    mut message_length: rmtU32,
) -> rmtError {
    let mut rmt: *mut Remotery = context as *mut Remotery;
    let mut message_id: rmtU32 = *(message_data as *mut rmtU32);
    match message_id {
        1229868867 => {
            _rmt_LogText(
                b"Console message received...\0" as *const u8 as *const libc::c_char,
            );
            _rmt_LogText(message_data.offset(4 as libc::c_int as isize) as rmtPStr);
            if (g_Settings.input_handler).is_some() {
                (g_Settings.input_handler)
                    .unwrap()(
                    message_data.offset(4 as libc::c_int as isize),
                    g_Settings.input_handler_context,
                );
            }
        }
        1347244871 => {
            let mut name: rmtPStr = 0 as *const libc::c_char;
            let mut name_hash: rmtU32 = 0 as libc::c_int as rmtU32;
            let mut cur: *const libc::c_char = message_data
                .offset(4 as libc::c_int as isize);
            let mut end: *const libc::c_char = cur
                .offset(message_length as isize)
                .offset(-(4 as libc::c_int as isize));
            while cur < end {
                let fresh38 = cur;
                cur = cur.offset(1);
                name_hash = name_hash
                    .wrapping_mul(10 as libc::c_int as libc::c_uint)
                    .wrapping_add(*fresh38 as libc::c_uint)
                    .wrapping_sub('0' as i32 as libc::c_uint);
            }
            name = StringTable_Find((*rmt).string_table, name_hash);
            if !name.is_null() {
                let mut name_length: rmtU32 = strnlen_s_safe_c(
                    name,
                    (256 as libc::c_int - 12 as libc::c_int) as r_size_t,
                );
                let mut bin_buf: *mut Buffer = (*(*rmt).server).bin_buf;
                WebSocket_PrepareBuffer(bin_buf);
                bin_SampleName(bin_buf, name, name_hash, name_length);
                return Server_Send(
                    (*rmt).server,
                    (*bin_buf).data as *const libc::c_void,
                    (*bin_buf).bytes_used,
                    10 as libc::c_int as rmtU32,
                );
            }
        }
        _ => {}
    }
    return RMT_ERROR_NONE;
}
unsafe extern "C" fn Remotery_Constructor(mut rmt: *mut Remotery) -> rmtError {
    if !rmt.is_null() {} else {
        __assert_fail(
            b"rmt != NULL\0" as *const u8 as *const libc::c_char,
            b"lib/Remotery.c\0" as *const u8 as *const libc::c_char,
            6802 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 42],
                &[libc::c_char; 42],
            >(b"rmtError Remotery_Constructor(Remotery *)\0"))
                .as_ptr(),
        );
    }
    'c_29194: {
        if !rmt.is_null() {} else {
            __assert_fail(
                b"rmt != NULL\0" as *const u8 as *const libc::c_char,
                b"lib/Remotery.c\0" as *const u8 as *const libc::c_char,
                6802 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 42],
                    &[libc::c_char; 42],
                >(b"rmtError Remotery_Constructor(Remotery *)\0"))
                    .as_ptr(),
            );
        }
    };
    (*rmt).server = 0 as *mut Server;
    (*rmt).mq_to_rmt_thread = 0 as *mut rmtMessageQueue;
    (*rmt).thread = 0 as *mut rmtThread;
    (*rmt).string_table = 0 as *mut StringTable;
    (*rmt).logfile = 0 as *mut FILE;
    (*rmt).map_message_queue_fn = None;
    (*rmt).map_message_queue_data = 0 as *mut libc::c_void;
    (*rmt).threadProfilers = 0 as *mut ThreadProfilers;
    mtxInit(&mut (*rmt).propertyMutex);
    (*rmt).propertyAllocator = 0 as *mut ObjectAllocator;
    (*rmt).propertyFrame = 0 as libc::c_int as rmtU32;
    let mut root_property: *mut rmtProperty = &mut (*rmt).rootProperty;
    (*root_property).initialised = 1 as libc::c_int as rmtBool;
    (*root_property).type_0 = RMT_PropertyType_rmtGroup;
    (*root_property).value.Bool = 0 as libc::c_int as rmtBool;
    (*root_property).flags = RMT_PropertyFlags_NoFlags;
    (*root_property).name = b"Root Property\0" as *const u8 as *const libc::c_char;
    (*root_property).description = b"\0" as *const u8 as *const libc::c_char;
    (*root_property).defaultValue.Bool = 0 as libc::c_int as rmtBool;
    (*root_property).parent = 0 as *mut rmtProperty;
    (*root_property).firstChild = 0 as *mut rmtProperty;
    (*root_property).lastChild = 0 as *mut rmtProperty;
    (*root_property).nextSibling = 0 as *mut rmtProperty;
    (*root_property).nameHash = 0 as libc::c_int as rmtU32;
    (*root_property).uniqueID = 0 as libc::c_int as rmtU32;
    usTimer_Init(&mut (*rmt).timer);
    (*rmt)
        .server = rmtMalloc(::std::mem::size_of::<Server>() as libc::c_ulong as rmtU32)
        as *mut Server;
    if ((*rmt).server).is_null() {
        return RMT_ERROR_MALLOC_FAIL;
    }
    let mut error: rmtError = Server_Constructor(
        (*rmt).server,
        g_Settings.port,
        g_Settings.reuse_open_port,
        g_Settings.limit_connections_to_localhost,
    );
    if error as libc::c_uint != RMT_ERROR_NONE as libc::c_int as libc::c_uint {
        if !((*rmt).server).is_null() {
            Server_Destructor((*rmt).server);
            rmtFree((*rmt).server as *mut libc::c_void);
            (*rmt).server = 0 as *mut Server;
        }
        return error;
    }
    (*(*rmt).server)
        .receive_handler = Some(
        Remotery_ReceiveMessage
            as unsafe extern "C" fn(
                *mut libc::c_void,
                *mut libc::c_char,
                rmtU32,
            ) -> rmtError,
    );
    (*(*rmt).server).receive_handler_context = rmt as *mut libc::c_void;
    (*rmt)
        .mq_to_rmt_thread = rmtMalloc(
        ::std::mem::size_of::<rmtMessageQueue>() as libc::c_ulong as rmtU32,
    ) as *mut rmtMessageQueue;
    if ((*rmt).mq_to_rmt_thread).is_null() {
        return RMT_ERROR_MALLOC_FAIL;
    }
    let mut error_0: rmtError = rmtMessageQueue_Constructor(
        (*rmt).mq_to_rmt_thread,
        g_Settings.messageQueueSizeInBytes,
    );
    if error_0 as libc::c_uint != RMT_ERROR_NONE as libc::c_int as libc::c_uint {
        if !((*rmt).mq_to_rmt_thread).is_null() {
            rmtMessageQueue_Destructor((*rmt).mq_to_rmt_thread);
            rmtFree((*rmt).mq_to_rmt_thread as *mut libc::c_void);
            (*rmt).mq_to_rmt_thread = 0 as *mut rmtMessageQueue;
        }
        return error_0;
    }
    (*rmt)
        .string_table = rmtMalloc(
        ::std::mem::size_of::<StringTable>() as libc::c_ulong as rmtU32,
    ) as *mut StringTable;
    if ((*rmt).string_table).is_null() {
        return RMT_ERROR_MALLOC_FAIL;
    }
    let mut error_1: rmtError = StringTable_Constructor((*rmt).string_table);
    if error_1 as libc::c_uint != RMT_ERROR_NONE as libc::c_int as libc::c_uint {
        if !((*rmt).string_table).is_null() {
            StringTable_Destructor((*rmt).string_table);
            rmtFree((*rmt).string_table as *mut libc::c_void);
            (*rmt).string_table = 0 as *mut StringTable;
        }
        return error_1;
    }
    if !(g_Settings.logPath).is_null() {
        let mut now_tm: *mut tm = TimeDateNow();
        let mut filename: [libc::c_char; 512] = [
            0 as libc::c_int as libc::c_char,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
        ];
        strncat_s_safe_c(
            filename.as_mut_ptr(),
            ::std::mem::size_of::<[libc::c_char; 512]>() as libc::c_ulong as r_size_t,
            g_Settings.logPath,
            512 as libc::c_int as r_size_t,
        );
        strncat_s_safe_c(
            filename.as_mut_ptr(),
            ::std::mem::size_of::<[libc::c_char; 512]>() as libc::c_ulong as r_size_t,
            b"/remotery-log-\0" as *const u8 as *const libc::c_char,
            14 as libc::c_int as r_size_t,
        );
        strncat_s_safe_c(
            filename.as_mut_ptr(),
            ::std::mem::size_of::<[libc::c_char; 512]>() as libc::c_ulong as r_size_t,
            itoa_s((*now_tm).tm_year + 1900 as libc::c_int),
            11 as libc::c_int as r_size_t,
        );
        strncat_s_safe_c(
            filename.as_mut_ptr(),
            ::std::mem::size_of::<[libc::c_char; 512]>() as libc::c_ulong as r_size_t,
            b"-\0" as *const u8 as *const libc::c_char,
            1 as libc::c_int as r_size_t,
        );
        strncat_s_safe_c(
            filename.as_mut_ptr(),
            ::std::mem::size_of::<[libc::c_char; 512]>() as libc::c_ulong as r_size_t,
            itoa_s((*now_tm).tm_mon + 1 as libc::c_int),
            11 as libc::c_int as r_size_t,
        );
        strncat_s_safe_c(
            filename.as_mut_ptr(),
            ::std::mem::size_of::<[libc::c_char; 512]>() as libc::c_ulong as r_size_t,
            b"-\0" as *const u8 as *const libc::c_char,
            1 as libc::c_int as r_size_t,
        );
        strncat_s_safe_c(
            filename.as_mut_ptr(),
            ::std::mem::size_of::<[libc::c_char; 512]>() as libc::c_ulong as r_size_t,
            itoa_s((*now_tm).tm_mday),
            11 as libc::c_int as r_size_t,
        );
        strncat_s_safe_c(
            filename.as_mut_ptr(),
            ::std::mem::size_of::<[libc::c_char; 512]>() as libc::c_ulong as r_size_t,
            b"-\0" as *const u8 as *const libc::c_char,
            1 as libc::c_int as r_size_t,
        );
        strncat_s_safe_c(
            filename.as_mut_ptr(),
            ::std::mem::size_of::<[libc::c_char; 512]>() as libc::c_ulong as r_size_t,
            itoa_s((*now_tm).tm_hour),
            11 as libc::c_int as r_size_t,
        );
        strncat_s_safe_c(
            filename.as_mut_ptr(),
            ::std::mem::size_of::<[libc::c_char; 512]>() as libc::c_ulong as r_size_t,
            b"-\0" as *const u8 as *const libc::c_char,
            1 as libc::c_int as r_size_t,
        );
        strncat_s_safe_c(
            filename.as_mut_ptr(),
            ::std::mem::size_of::<[libc::c_char; 512]>() as libc::c_ulong as r_size_t,
            itoa_s((*now_tm).tm_min),
            11 as libc::c_int as r_size_t,
        );
        strncat_s_safe_c(
            filename.as_mut_ptr(),
            ::std::mem::size_of::<[libc::c_char; 512]>() as libc::c_ulong as r_size_t,
            b"-\0" as *const u8 as *const libc::c_char,
            1 as libc::c_int as r_size_t,
        );
        strncat_s_safe_c(
            filename.as_mut_ptr(),
            ::std::mem::size_of::<[libc::c_char; 512]>() as libc::c_ulong as r_size_t,
            itoa_s((*now_tm).tm_sec),
            11 as libc::c_int as r_size_t,
        );
        strncat_s_safe_c(
            filename.as_mut_ptr(),
            ::std::mem::size_of::<[libc::c_char; 512]>() as libc::c_ulong as r_size_t,
            b".rbin\0" as *const u8 as *const libc::c_char,
            5 as libc::c_int as r_size_t,
        );
        (*rmt)
            .logfile = rmtOpenFile(
            filename.as_mut_ptr(),
            b"w\0" as *const u8 as *const libc::c_char,
        );
        if !((*rmt).logfile).is_null() {
            rmtWriteFile(
                (*rmt).logfile,
                b"RMTBLOGF\0" as *const u8 as *const libc::c_char as *const libc::c_void,
                8 as libc::c_int as rmtU32,
            );
        }
    }
    (*rmt)
        .threadProfilers = rmtMalloc(
        ::std::mem::size_of::<ThreadProfilers>() as libc::c_ulong as rmtU32,
    ) as *mut ThreadProfilers;
    if ((*rmt).threadProfilers).is_null() {
        return RMT_ERROR_MALLOC_FAIL;
    }
    let mut error_2: rmtError = ThreadProfilers_Constructor(
        (*rmt).threadProfilers,
        &mut (*rmt).timer,
        (*rmt).mq_to_rmt_thread,
    );
    if error_2 as libc::c_uint != RMT_ERROR_NONE as libc::c_int as libc::c_uint {
        if !((*rmt).threadProfilers).is_null() {
            ThreadProfilers_Destructor((*rmt).threadProfilers);
            rmtFree((*rmt).threadProfilers as *mut libc::c_void);
            (*rmt).threadProfilers = 0 as *mut ThreadProfilers;
        }
        return error_2;
    }
    (*rmt)
        .propertyAllocator = rmtMalloc(
        ::std::mem::size_of::<ObjectAllocator>() as libc::c_ulong as rmtU32,
    ) as *mut ObjectAllocator;
    if ((*rmt).propertyAllocator).is_null() {
        return RMT_ERROR_MALLOC_FAIL;
    }
    let mut error_3: rmtError = ObjectAllocator_Constructor(
        (*rmt).propertyAllocator,
        ::std::mem::size_of::<PropertySnapshot>() as libc::c_ulong as rmtU32,
        ::std::mem::transmute::<
            Option::<unsafe extern "C" fn(*mut PropertySnapshot) -> rmtError>,
            ObjConstructor,
        >(
            Some(
                PropertySnapshot_Constructor
                    as unsafe extern "C" fn(*mut PropertySnapshot) -> rmtError,
            ),
        ),
        ::std::mem::transmute::<
            Option::<unsafe extern "C" fn(*mut PropertySnapshot) -> ()>,
            ObjDestructor,
        >(
            Some(
                PropertySnapshot_Destructor
                    as unsafe extern "C" fn(*mut PropertySnapshot) -> (),
            ),
        ),
    );
    if error_3 as libc::c_uint != RMT_ERROR_NONE as libc::c_int as libc::c_uint {
        if !((*rmt).propertyAllocator).is_null() {
            ObjectAllocator_Destructor((*rmt).propertyAllocator);
            rmtFree((*rmt).propertyAllocator as *mut libc::c_void);
            (*rmt).propertyAllocator = 0 as *mut ObjectAllocator;
        }
        return error_3;
    }
    if g_Remotery.is_null() {} else {
        __assert_fail(
            b"g_Remotery == NULL\0" as *const u8 as *const libc::c_char,
            b"lib/Remotery.c\0" as *const u8 as *const libc::c_char,
            6922 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 42],
                &[libc::c_char; 42],
            >(b"rmtError Remotery_Constructor(Remotery *)\0"))
                .as_ptr(),
        );
    }
    'c_23314: {
        if g_Remotery.is_null() {} else {
            __assert_fail(
                b"g_Remotery == NULL\0" as *const u8 as *const libc::c_char,
                b"lib/Remotery.c\0" as *const u8 as *const libc::c_char,
                6922 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 42],
                    &[libc::c_char; 42],
                >(b"rmtError Remotery_Constructor(Remotery *)\0"))
                    .as_ptr(),
            );
        }
    };
    g_Remotery = rmt;
    g_RemoteryCreated = 1 as libc::c_int as rmtBool;
    CompilerWriteFence();
    (*rmt)
        .thread = rmtMalloc(
        ::std::mem::size_of::<rmtThread>() as libc::c_ulong as rmtU32,
    ) as *mut rmtThread;
    if ((*rmt).thread).is_null() {
        return RMT_ERROR_MALLOC_FAIL;
    }
    let mut error_4: rmtError = rmtThread_Constructor(
        (*rmt).thread,
        Some(Remotery_ThreadMain as unsafe extern "C" fn(*mut rmtThread) -> rmtError),
        rmt as *mut libc::c_void,
    );
    if error_4 as libc::c_uint != RMT_ERROR_NONE as libc::c_int as libc::c_uint {
        if !((*rmt).thread).is_null() {
            rmtThread_Destructor((*rmt).thread);
            rmtFree((*rmt).thread as *mut libc::c_void);
            (*rmt).thread = 0 as *mut rmtThread;
        }
        return error_4;
    }
    return RMT_ERROR_NONE;
}
unsafe extern "C" fn Remotery_Destructor(mut rmt: *mut Remotery) {
    if !rmt.is_null() {} else {
        __assert_fail(
            b"rmt != NULL\0" as *const u8 as *const libc::c_char,
            b"lib/Remotery.c\0" as *const u8 as *const libc::c_char,
            6937 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 37],
                &[libc::c_char; 37],
            >(b"void Remotery_Destructor(Remotery *)\0"))
                .as_ptr(),
        );
    }
    'c_29569: {
        if !rmt.is_null() {} else {
            __assert_fail(
                b"rmt != NULL\0" as *const u8 as *const libc::c_char,
                b"lib/Remotery.c\0" as *const u8 as *const libc::c_char,
                6937 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 37],
                    &[libc::c_char; 37],
                >(b"void Remotery_Destructor(Remotery *)\0"))
                    .as_ptr(),
            );
        }
    };
    if !((*rmt).thread).is_null() {
        rmtThread_Destructor((*rmt).thread);
        rmtFree((*rmt).thread as *mut libc::c_void);
        (*rmt).thread = 0 as *mut rmtThread;
    }
    if g_RemoteryCreated != 0 {
        g_Remotery = 0 as *mut Remotery;
        g_RemoteryCreated = 0 as libc::c_int as rmtBool;
    }
    if !((*rmt).propertyAllocator).is_null() {
        ObjectAllocator_Destructor((*rmt).propertyAllocator);
        rmtFree((*rmt).propertyAllocator as *mut libc::c_void);
        (*rmt).propertyAllocator = 0 as *mut ObjectAllocator;
    }
    if !((*rmt).threadProfilers).is_null() {
        ThreadProfilers_Destructor((*rmt).threadProfilers);
        rmtFree((*rmt).threadProfilers as *mut libc::c_void);
        (*rmt).threadProfilers = 0 as *mut ThreadProfilers;
    }
    rmtCloseFile((*rmt).logfile);
    if !((*rmt).string_table).is_null() {
        StringTable_Destructor((*rmt).string_table);
        rmtFree((*rmt).string_table as *mut libc::c_void);
        (*rmt).string_table = 0 as *mut StringTable;
    }
    if !((*rmt).mq_to_rmt_thread).is_null() {
        rmtMessageQueue_Destructor((*rmt).mq_to_rmt_thread);
        rmtFree((*rmt).mq_to_rmt_thread as *mut libc::c_void);
        (*rmt).mq_to_rmt_thread = 0 as *mut rmtMessageQueue;
    }
    if !((*rmt).server).is_null() {
        Server_Destructor((*rmt).server);
        rmtFree((*rmt).server as *mut libc::c_void);
        (*rmt).server = 0 as *mut Server;
    }
    if g_lastErrorMessageTlsHandle != 0xffffffff as libc::c_uint {
        tlsFree(g_lastErrorMessageTlsHandle);
        g_lastErrorMessageTlsHandle = 0xffffffff as libc::c_uint;
    }
    mtxDelete(&mut (*rmt).propertyMutex);
}
unsafe extern "C" fn CRTMalloc(
    mut mm_context: *mut libc::c_void,
    mut size: rmtU32,
) -> *mut libc::c_void {
    if 1 as libc::c_int != 0 {} else {};
    return malloc(size as size_t);
}
unsafe extern "C" fn CRTFree(
    mut mm_context: *mut libc::c_void,
    mut ptr: *mut libc::c_void,
) {
    if 1 as libc::c_int != 0 {} else {};
    free(ptr);
}
unsafe extern "C" fn CRTRealloc(
    mut mm_context: *mut libc::c_void,
    mut ptr: *mut libc::c_void,
    mut size: rmtU32,
) -> *mut libc::c_void {
    if 1 as libc::c_int != 0 {} else {};
    return realloc(ptr, size as libc::c_ulong);
}
pub unsafe extern "C" fn _rmt_Settings() -> *mut rmtSettings {
    if g_SettingsInitialized == 0 as libc::c_int as rmtBool {
        g_Settings.port = 0x4597 as libc::c_int as rmtU16;
        g_Settings.reuse_open_port = 0 as libc::c_int as rmtBool;
        g_Settings.limit_connections_to_localhost = 0 as libc::c_int as rmtBool;
        g_Settings.enableThreadSampler = 1 as libc::c_int as rmtBool;
        g_Settings.msSleepBetweenServerUpdates = 4 as libc::c_int as rmtU32;
        g_Settings
            .messageQueueSizeInBytes = (1024 as libc::c_int * 1024 as libc::c_int)
            as rmtU32;
        g_Settings.maxNbMessagesPerUpdate = 1000 as libc::c_int as rmtU32;
        g_Settings
            .malloc = Some(
            CRTMalloc
                as unsafe extern "C" fn(*mut libc::c_void, rmtU32) -> *mut libc::c_void,
        );
        g_Settings
            .free = Some(
            CRTFree as unsafe extern "C" fn(*mut libc::c_void, *mut libc::c_void) -> (),
        );
        g_Settings
            .realloc = Some(
            CRTRealloc
                as unsafe extern "C" fn(
                    *mut libc::c_void,
                    *mut libc::c_void,
                    rmtU32,
                ) -> *mut libc::c_void,
        );
        g_Settings.input_handler = None;
        g_Settings.input_handler_context = 0 as *mut libc::c_void;
        g_Settings.logPath = 0 as rmtPStr;
        g_Settings.sampletree_handler = None;
        g_Settings.sampletree_context = 0 as *mut libc::c_void;
        g_Settings.snapshot_callback = None;
        g_Settings.snapshot_context = 0 as *mut libc::c_void;
        g_SettingsInitialized = 1 as libc::c_int as rmtBool;
    }
    return &mut g_Settings;
}
pub unsafe extern "C" fn _rmt_CreateGlobalInstance(
    mut remotery: *mut *mut Remotery,
) -> rmtError {
    if ::std::mem::size_of::<MessageID>() as libc::c_ulong
        == ::std::mem::size_of::<rmtU32>() as libc::c_ulong
    {} else {
        __assert_fail(
            b"sizeof(MessageID) == sizeof(rmtU32)\0" as *const u8 as *const libc::c_char,
            b"lib/Remotery.c\0" as *const u8 as *const libc::c_char,
            7037 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 53],
                &[libc::c_char; 53],
            >(b"enum rmtError _rmt_CreateGlobalInstance(Remotery **)\0"))
                .as_ptr(),
        );
    }
    'c_29693: {
        if ::std::mem::size_of::<MessageID>() as libc::c_ulong
            == ::std::mem::size_of::<rmtU32>() as libc::c_ulong
        {} else {
            __assert_fail(
                b"sizeof(MessageID) == sizeof(rmtU32)\0" as *const u8
                    as *const libc::c_char,
                b"lib/Remotery.c\0" as *const u8 as *const libc::c_char,
                7037 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 53],
                    &[libc::c_char; 53],
                >(b"enum rmtError _rmt_CreateGlobalInstance(Remotery **)\0"))
                    .as_ptr(),
            );
        }
    };
    _rmt_Settings();
    if !remotery.is_null() {} else {
        __assert_fail(
            b"remotery != NULL\0" as *const u8 as *const libc::c_char,
            b"lib/Remotery.c\0" as *const u8 as *const libc::c_char,
            7043 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 53],
                &[libc::c_char; 53],
            >(b"enum rmtError _rmt_CreateGlobalInstance(Remotery **)\0"))
                .as_ptr(),
        );
    }
    'c_29647: {
        if !remotery.is_null() {} else {
            __assert_fail(
                b"remotery != NULL\0" as *const u8 as *const libc::c_char,
                b"lib/Remotery.c\0" as *const u8 as *const libc::c_char,
                7043 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 53],
                    &[libc::c_char; 53],
                >(b"enum rmtError _rmt_CreateGlobalInstance(Remotery **)\0"))
                    .as_ptr(),
            );
        }
    };
    *remotery = rmtMalloc(::std::mem::size_of::<Remotery>() as libc::c_ulong as rmtU32)
        as *mut Remotery;
    if (*remotery).is_null() {
        return RMT_ERROR_MALLOC_FAIL;
    }
    let mut error: rmtError = Remotery_Constructor(*remotery);
    if error as libc::c_uint != RMT_ERROR_NONE as libc::c_int as libc::c_uint {
        if !(*remotery).is_null() {
            Remotery_Destructor(*remotery);
            rmtFree(*remotery as *mut libc::c_void);
            *remotery = 0 as *mut Remotery;
        }
        return error;
    }
    return RMT_ERROR_NONE;
}
pub unsafe extern "C" fn _rmt_DestroyGlobalInstance(mut remotery: *mut Remotery) {
    if g_RemoteryCreated == 1 as libc::c_int as rmtBool {} else {
        __assert_fail(
            b"g_RemoteryCreated == RMT_TRUE\0" as *const u8 as *const libc::c_char,
            b"lib/Remotery.c\0" as *const u8 as *const libc::c_char,
            7051 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 44],
                &[libc::c_char; 44],
            >(b"void _rmt_DestroyGlobalInstance(Remotery *)\0"))
                .as_ptr(),
        );
    }
    'c_29801: {
        if g_RemoteryCreated == 1 as libc::c_int as rmtBool {} else {
            __assert_fail(
                b"g_RemoteryCreated == RMT_TRUE\0" as *const u8 as *const libc::c_char,
                b"lib/Remotery.c\0" as *const u8 as *const libc::c_char,
                7051 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 44],
                    &[libc::c_char; 44],
                >(b"void _rmt_DestroyGlobalInstance(Remotery *)\0"))
                    .as_ptr(),
            );
        }
    };
    if g_Remotery == remotery {} else {
        __assert_fail(
            b"g_Remotery == remotery\0" as *const u8 as *const libc::c_char,
            b"lib/Remotery.c\0" as *const u8 as *const libc::c_char,
            7052 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 44],
                &[libc::c_char; 44],
            >(b"void _rmt_DestroyGlobalInstance(Remotery *)\0"))
                .as_ptr(),
        );
    }
    'c_29763: {
        if g_Remotery == remotery {} else {
            __assert_fail(
                b"g_Remotery == remotery\0" as *const u8 as *const libc::c_char,
                b"lib/Remotery.c\0" as *const u8 as *const libc::c_char,
                7052 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 44],
                    &[libc::c_char; 44],
                >(b"void _rmt_DestroyGlobalInstance(Remotery *)\0"))
                    .as_ptr(),
            );
        }
    };
    if !remotery.is_null() {
        Remotery_Destructor(remotery);
        rmtFree(remotery as *mut libc::c_void);
        remotery = 0 as *mut Remotery;
    }
}
pub unsafe extern "C" fn _rmt_SetGlobalInstance(mut remotery: *mut Remotery) {
    _rmt_Settings();
    g_Remotery = remotery;
}
pub unsafe extern "C" fn _rmt_GetGlobalInstance() -> *mut Remotery {
    return g_Remotery;
}
pub unsafe extern "C" fn MakeWideString(
    mut string: *const libc::c_char,
) -> *mut wchar_t {
    let mut wlen: size_t = 0;
    let mut wstr: *mut wchar_t = 0 as *mut wchar_t;
    wlen = mbstowcs(0 as *mut wchar_t, string, 256 as libc::c_int as size_t);
    wstr = rmtMalloc(
        wlen
            .wrapping_add(1 as libc::c_int as libc::c_ulong)
            .wrapping_mul(::std::mem::size_of::<wchar_t>() as libc::c_ulong) as rmtU32,
    ) as *mut wchar_t;
    if wstr.is_null() {
        return 0 as *mut wchar_t;
    }
    if mbstowcs(wstr, string, wlen.wrapping_add(1 as libc::c_int as libc::c_ulong))
        != wlen
    {
        rmtFree(wstr as *mut libc::c_void);
        return 0 as *mut wchar_t;
    }
    return wstr;
}
unsafe extern "C" fn SetDebuggerThreadName(mut name: *const libc::c_char) {
    if 1 as libc::c_int != 0 {} else {};
    let mut name_clamp: [libc::c_char; 16] = [0; 16];
    name_clamp[0 as libc::c_int as usize] = 0 as libc::c_int as libc::c_char;
    strncat_s_safe_c(
        name_clamp.as_mut_ptr(),
        ::std::mem::size_of::<[libc::c_char; 16]>() as libc::c_ulong as r_size_t,
        name,
        15 as libc::c_int as r_size_t,
    );
    prctl(
        15 as libc::c_int,
        name_clamp.as_mut_ptr(),
        0 as libc::c_int,
        0 as libc::c_int,
        0 as libc::c_int,
    );
}
pub unsafe extern "C" fn _rmt_SetCurrentThreadName(mut thread_name: rmtPStr) {
    let mut thread_profiler: *mut ThreadProfiler = 0 as *mut ThreadProfiler;
    let mut name_length: rmtU32 = 0;
    if g_Remotery.is_null() {
        return;
    }
    if ThreadProfilers_GetCurrentThreadProfiler(
        (*g_Remotery).threadProfilers,
        &mut thread_profiler,
    ) as libc::c_uint != RMT_ERROR_NONE as libc::c_int as libc::c_uint
    {
        return;
    }
    strcpy_s_safe_c(
        ((*thread_profiler).threadName).as_mut_ptr(),
        ::std::mem::size_of::<[libc::c_char; 64]>() as libc::c_ulong as r_size_t,
        thread_name,
    );
    (*thread_profiler)
        .threadNameHash = _rmt_HashString32(
        thread_name,
        strnlen_s_safe_c(thread_name, 64 as libc::c_int as r_size_t) as libc::c_int,
        0 as libc::c_int as rmtU32,
    );
    SetDebuggerThreadName(thread_name);
}
unsafe extern "C" fn QueueLine(
    mut queue: *mut rmtMessageQueue,
    mut text: *mut libc::c_uchar,
    mut size: rmtU32,
    mut thread_profiler: *mut ThreadProfiler,
) -> rmtBool {
    let mut message: *mut Message = 0 as *mut Message;
    let mut text_size: rmtU32 = 0;
    if !queue.is_null() {} else {
        __assert_fail(
            b"queue != NULL\0" as *const u8 as *const libc::c_char,
            b"lib/Remotery.c\0" as *const u8 as *const libc::c_char,
            7207 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 87],
                &[libc::c_char; 87],
            >(
                b"rmtBool QueueLine(rmtMessageQueue *, unsigned char *, rmtU32, struct ThreadProfiler *)\0",
            ))
                .as_ptr(),
        );
    }
    'c_18285: {
        if !queue.is_null() {} else {
            __assert_fail(
                b"queue != NULL\0" as *const u8 as *const libc::c_char,
                b"lib/Remotery.c\0" as *const u8 as *const libc::c_char,
                7207 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 87],
                    &[libc::c_char; 87],
                >(
                    b"rmtBool QueueLine(rmtMessageQueue *, unsigned char *, rmtU32, struct ThreadProfiler *)\0",
                ))
                    .as_ptr(),
            );
        }
    };
    text_size = size.wrapping_sub(4 as libc::c_int as libc::c_uint);
    U32ToByteArray(text, text_size);
    message = rmtMessageQueue_AllocMessage(queue, size, thread_profiler);
    if message.is_null() {
        return 0 as libc::c_int as rmtBool;
    }
    memcpy(
        ((*message).payload).as_mut_ptr() as *mut libc::c_void,
        text as *const libc::c_void,
        size as libc::c_ulong,
    );
    rmtMessageQueue_CommitMessage(message, MsgID_LogText);
    return 1 as libc::c_int as rmtBool;
}
pub unsafe extern "C" fn _rmt_LogText(mut text: rmtPStr) {
    let mut start_offset: libc::c_int = 0;
    let mut offset: libc::c_int = 0;
    let mut i: libc::c_int = 0;
    let mut line_buffer: [libc::c_uchar; 1024] = [
        0 as libc::c_int as libc::c_uchar,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
    ];
    let mut thread_profiler: *mut ThreadProfiler = 0 as *mut ThreadProfiler;
    if g_Remotery.is_null() {
        return;
    }
    if ThreadProfilers_GetCurrentThreadProfiler(
        (*g_Remotery).threadProfilers,
        &mut thread_profiler,
    ) as libc::c_uint != RMT_ERROR_NONE as libc::c_int as libc::c_uint
    {
        return;
    }
    line_buffer[0 as libc::c_int as usize] = ' ' as i32 as libc::c_uchar;
    line_buffer[1 as libc::c_int as usize] = ' ' as i32 as libc::c_uchar;
    line_buffer[2 as libc::c_int as usize] = ' ' as i32 as libc::c_uchar;
    line_buffer[3 as libc::c_int as usize] = ' ' as i32 as libc::c_uchar;
    start_offset = 4 as libc::c_int;
    offset = start_offset;
    let mut current_block_15: u64;
    i = 0 as libc::c_int;
    while *text.offset(i as isize) as libc::c_int != 0 as libc::c_int {
        let mut c: libc::c_char = *text.offset(i as isize);
        if offset as libc::c_ulong
            == (::std::mem::size_of::<[libc::c_uchar; 1024]>() as libc::c_ulong)
                .wrapping_sub(1 as libc::c_int as libc::c_ulong)
            || c as libc::c_int == '\n' as i32
        {
            if QueueLine(
                (*g_Remotery).mq_to_rmt_thread,
                line_buffer.as_mut_ptr(),
                offset as rmtU32,
                thread_profiler,
            ) == 0 as libc::c_int as rmtBool
            {
                return;
            }
            offset = start_offset;
            if c as libc::c_int == '\n' as i32 {
                current_block_15 = 13513818773234778473;
            } else {
                current_block_15 = 10048703153582371463;
            }
        } else {
            current_block_15 = 10048703153582371463;
        }
        match current_block_15 {
            10048703153582371463 => {
                let fresh39 = offset;
                offset = offset + 1;
                line_buffer[fresh39 as usize] = c as libc::c_uchar;
            }
            _ => {}
        }
        i += 1;
        i;
    }
    if offset > start_offset {
        if offset
            < ::std::mem::size_of::<[libc::c_uchar; 1024]>() as libc::c_ulong
                as libc::c_int
        {} else {
            __assert_fail(
                b"offset < (int)sizeof(line_buffer)\0" as *const u8
                    as *const libc::c_char,
                b"lib/Remotery.c\0" as *const u8 as *const libc::c_char,
                7276 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 27],
                    &[libc::c_char; 27],
                >(b"void _rmt_LogText(rmtPStr)\0"))
                    .as_ptr(),
            );
        }
        'c_18327: {
            if offset
                < ::std::mem::size_of::<[libc::c_uchar; 1024]>() as libc::c_ulong
                    as libc::c_int
            {} else {
                __assert_fail(
                    b"offset < (int)sizeof(line_buffer)\0" as *const u8
                        as *const libc::c_char,
                    b"lib/Remotery.c\0" as *const u8 as *const libc::c_char,
                    7276 as libc::c_int as libc::c_uint,
                    (*::std::mem::transmute::<
                        &[u8; 27],
                        &[libc::c_char; 27],
                    >(b"void _rmt_LogText(rmtPStr)\0"))
                        .as_ptr(),
                );
            }
        };
        QueueLine(
            (*g_Remotery).mq_to_rmt_thread,
            line_buffer.as_mut_ptr(),
            offset as rmtU32,
            thread_profiler,
        );
    }
}
pub unsafe extern "C" fn _rmt_BeginCPUSample(
    mut name: rmtPStr,
    mut flags: rmtU32,
    mut hash_cache: *mut rmtU32,
) {
    let mut thread_profiler: *mut ThreadProfiler = 0 as *mut ThreadProfiler;
    if g_Remotery.is_null() {
        return;
    }
    if ThreadProfilers_GetCurrentThreadProfiler(
        (*g_Remotery).threadProfilers,
        &mut thread_profiler,
    ) as libc::c_uint == RMT_ERROR_NONE as libc::c_int as libc::c_uint
    {
        let mut sample: *mut Sample = 0 as *mut Sample;
        let mut name_hash: rmtU32 = ThreadProfiler_GetNameHash(
            thread_profiler,
            (*g_Remotery).mq_to_rmt_thread,
            name,
            hash_cache,
        );
        if ThreadProfiler_Push(
            (*thread_profiler).sampleTrees[RMT_SampleType_CPU as libc::c_int as usize],
            name_hash,
            flags,
            &mut sample,
        ) as libc::c_uint == RMT_ERROR_NONE as libc::c_int as libc::c_uint
        {
            if (*sample).call_count > 1 as libc::c_int as libc::c_uint {
                (*sample).us_end = usTimer_Get(&mut (*g_Remotery).timer);
            } else {
                (*sample).us_start = usTimer_Get(&mut (*g_Remotery).timer);
            }
        }
    }
}
pub unsafe extern "C" fn _rmt_EndCPUSample() {
    let mut thread_profiler: *mut ThreadProfiler = 0 as *mut ThreadProfiler;
    if g_Remotery.is_null() {
        return;
    }
    if ThreadProfilers_GetCurrentThreadProfiler(
        (*g_Remotery).threadProfilers,
        &mut thread_profiler,
    ) as libc::c_uint == RMT_ERROR_NONE as libc::c_int as libc::c_uint
    {
        let mut sample: *mut Sample = (*(*thread_profiler)
            .sampleTrees[RMT_SampleType_CPU as libc::c_int as usize])
            .currentParent;
        if (*sample).recurse_depth as libc::c_int > 0 as libc::c_int {
            (*sample).recurse_depth = ((*sample).recurse_depth).wrapping_sub(1);
            (*sample).recurse_depth;
        } else {
            let mut us_end: rmtU64 = usTimer_Get(&mut (*g_Remotery).timer);
            Sample_Close(sample, us_end as rmtS64);
            ThreadProfiler_Pop(
                thread_profiler,
                (*g_Remotery).mq_to_rmt_thread,
                sample,
                0 as libc::c_int as rmtU32,
            );
        }
    }
}
pub unsafe extern "C" fn _rmt_MarkFrame() -> rmtError {
    if g_Remotery.is_null() {
        return RMT_ERROR_REMOTERY_NOT_CREATED;
    }
    return RMT_ERROR_NONE;
}
pub unsafe extern "C" fn _rmt_IterateChildren(
    mut iterator: *mut rmtSampleIterator,
    mut sample: *mut rmtSample,
) {
    (*iterator).sample = 0 as *mut rmtSample;
    (*iterator)
        .initial = if !sample.is_null() {
        (*sample).first_child
    } else {
        0 as *mut Sample
    };
}
pub unsafe extern "C" fn _rmt_IterateNext(mut iter: *mut rmtSampleIterator) -> rmtBool {
    if !((*iter).initial).is_null() {
        (*iter).sample = (*iter).initial;
        (*iter).initial = 0 as *mut rmtSample;
    } else if !((*iter).sample).is_null() {
        (*iter).sample = (*(*iter).sample).next_sibling;
    }
    return if !((*iter).sample).is_null() {
        1 as libc::c_int as rmtBool
    } else {
        0 as libc::c_int as rmtBool
    };
}
pub unsafe extern "C" fn _rmt_SampleTreeGetThreadName(
    mut sample_tree: *mut rmtSampleTree,
) -> *const libc::c_char {
    return (*sample_tree).threadName;
}
pub unsafe extern "C" fn _rmt_SampleTreeGetRootSample(
    mut sample_tree: *mut rmtSampleTree,
) -> *mut rmtSample {
    return (*sample_tree).rootSample;
}
pub unsafe extern "C" fn _rmt_SampleGetName(
    mut sample: *mut rmtSample,
) -> *const libc::c_char {
    let mut name: *const libc::c_char = StringTable_Find(
        (*g_Remotery).string_table,
        (*sample).name_hash,
    );
    if name.is_null() {
        return b"null\0" as *const u8 as *const libc::c_char;
    }
    return name;
}
pub unsafe extern "C" fn _rmt_SampleGetNameHash(mut sample: *mut rmtSample) -> rmtU32 {
    return (*sample).name_hash;
}
pub unsafe extern "C" fn _rmt_SampleGetCallCount(mut sample: *mut rmtSample) -> rmtU32 {
    return (*sample).call_count;
}
pub unsafe extern "C" fn _rmt_SampleGetStart(mut sample: *mut rmtSample) -> rmtU64 {
    return (*sample).us_start;
}
pub unsafe extern "C" fn _rmt_SampleGetTime(mut sample: *mut rmtSample) -> rmtU64 {
    return (*sample).us_length;
}
pub unsafe extern "C" fn _rmt_SampleGetSelfTime(mut sample: *mut rmtSample) -> rmtU64 {
    return maxS64(
        ((*sample).us_length).wrapping_sub((*sample).us_sampled_length) as rmtS64,
        0 as libc::c_int as rmtS64,
    ) as rmtU64;
}
pub unsafe extern "C" fn _rmt_SampleGetType(
    mut sample: *mut rmtSample,
) -> rmtSampleType {
    return (*sample).type_0;
}
pub unsafe extern "C" fn _rmt_SampleGetColour(
    mut sample: *mut rmtSample,
    mut r: *mut rmtU8,
    mut g: *mut rmtU8,
    mut b: *mut rmtU8,
) {
    *r = (*sample).uniqueColour[0 as libc::c_int as usize];
    *g = (*sample).uniqueColour[1 as libc::c_int as usize];
    *b = (*sample).uniqueColour[2 as libc::c_int as usize];
}
pub unsafe extern "C" fn _rmt_PropertyIterateChildren(
    mut iterator: *mut rmtPropertyIterator,
    mut property: *mut rmtProperty,
) {
    (*iterator).property = 0 as *mut rmtProperty;
    (*iterator)
        .initial = if !property.is_null() {
        (*property).firstChild
    } else {
        0 as *mut rmtProperty
    };
}
pub unsafe extern "C" fn _rmt_PropertyIterateNext(
    mut iter: *mut rmtPropertyIterator,
) -> rmtBool {
    if !((*iter).initial).is_null() {
        (*iter).property = (*iter).initial;
        (*iter).initial = 0 as *mut rmtProperty;
    } else if !((*iter).property).is_null() {
        (*iter).property = (*(*iter).property).nextSibling;
    }
    return if !((*iter).property).is_null() {
        1 as libc::c_int as rmtBool
    } else {
        0 as libc::c_int as rmtBool
    };
}
pub unsafe extern "C" fn _rmt_PropertyGetName(
    mut property: *mut rmtProperty,
) -> *const libc::c_char {
    return (*property).name;
}
pub unsafe extern "C" fn _rmt_PropertyGetDescription(
    mut property: *mut rmtProperty,
) -> *const libc::c_char {
    return (*property).description;
}
pub unsafe extern "C" fn _rmt_PropertyGetNameHash(
    mut property: *mut rmtProperty,
) -> rmtU32 {
    return (*property).nameHash;
}
pub unsafe extern "C" fn _rmt_PropertyGetType(
    mut property: *mut rmtProperty,
) -> rmtPropertyType {
    return (*property).type_0;
}
pub unsafe extern "C" fn _rmt_PropertyGetValue(
    mut property: *mut rmtProperty,
) -> rmtPropertyValue {
    return (*property).value;
}
unsafe extern "C" fn RegisterProperty(
    mut property: *mut rmtProperty,
    mut can_lock: rmtBool,
) {
    if (*property).initialised == 0 as libc::c_int as rmtBool {
        if can_lock != 0 {
            mtxLock(&mut (*g_Remotery).propertyMutex);
        }
        if (*property).initialised == 0 as libc::c_int as rmtBool {
            let mut name_len: rmtU32 = 0;
            let mut parent_property: *mut rmtProperty = (*property).parent;
            if parent_property.is_null() {
                (*property).parent = &mut (*g_Remotery).rootProperty;
                parent_property = (*property).parent;
            }
            RegisterProperty(parent_property, 0 as libc::c_int as rmtBool);
            if !((*parent_property).firstChild).is_null() {
                (*(*parent_property).lastChild).nextSibling = property;
                (*parent_property).lastChild = property;
            } else {
                (*parent_property).firstChild = property;
                (*parent_property).lastChild = property;
            }
            name_len = strnlen_s_safe_c(
                (*property).name,
                256 as libc::c_int as r_size_t,
            );
            (*property)
                .nameHash = _rmt_HashString32(
                (*property).name,
                name_len as libc::c_int,
                0 as libc::c_int as rmtU32,
            );
            QueueAddToStringTable(
                (*g_Remotery).mq_to_rmt_thread,
                (*property).nameHash,
                (*property).name,
                name_len as size_t,
                0 as *mut ThreadProfiler,
            );
            (*property).uniqueID = (*parent_property).uniqueID;
            (*property)
                .uniqueID = HashCombine((*property).uniqueID, (*property).nameHash);
            (*property).initialised = 1 as libc::c_int as rmtBool;
        }
        if can_lock != 0 {
            mtxUnlock(&mut (*g_Remotery).propertyMutex);
        }
    }
}
pub unsafe extern "C" fn _rmt_PropertySetValue(mut property: *mut rmtProperty) {
    if g_Remotery.is_null() {
        return;
    }
    RegisterProperty(property, 1 as libc::c_int as rmtBool);
}
pub unsafe extern "C" fn _rmt_PropertyAddValue(
    mut property: *mut rmtProperty,
    mut add_value: rmtPropertyValue,
) {
    if g_Remotery.is_null() {
        return;
    }
    RegisterProperty(property, 1 as libc::c_int as rmtBool);
}
unsafe extern "C" fn TakePropertySnapshot(
    mut property: *mut rmtProperty,
    mut parent_snapshot: *mut PropertySnapshot,
    mut first_snapshot: *mut *mut PropertySnapshot,
    mut prev_snapshot: *mut *mut PropertySnapshot,
    mut depth: rmtU32,
) -> rmtError {
    let mut error: rmtError = RMT_ERROR_NONE;
    let mut child_property: *mut rmtProperty = 0 as *mut rmtProperty;
    let mut snapshot: *mut PropertySnapshot = 0 as *mut PropertySnapshot;
    error = ObjectAllocator_Alloc(
        (*g_Remotery).propertyAllocator,
        &mut snapshot as *mut *mut PropertySnapshot as *mut *mut libc::c_void,
    );
    if error as libc::c_uint != RMT_ERROR_NONE as libc::c_int as libc::c_uint {
        return error;
    }
    (*snapshot).type_0 = (*property).type_0;
    (*snapshot).value = (*property).value;
    (*snapshot).prevValue = (*property).prevValue;
    (*snapshot).prevValueFrame = (*property).prevValueFrame;
    (*snapshot).nameHash = (*property).nameHash;
    (*snapshot).uniqueID = (*property).uniqueID;
    (*snapshot).nbChildren = 0 as libc::c_int as rmtU32;
    (*snapshot).depth = depth as rmtU8;
    (*snapshot).nextSnapshot = 0 as *mut PropertySnapshot;
    if !parent_snapshot.is_null() {
        (*parent_snapshot).nbChildren = ((*parent_snapshot).nbChildren).wrapping_add(1);
        (*parent_snapshot).nbChildren;
    }
    if (*first_snapshot).is_null() {
        *first_snapshot = snapshot;
    }
    if !(*prev_snapshot).is_null() {
        (**prev_snapshot).nextSnapshot = snapshot;
    }
    *prev_snapshot = snapshot;
    child_property = (*property).firstChild;
    while !child_property.is_null() {
        error = TakePropertySnapshot(
            child_property,
            snapshot,
            first_snapshot,
            prev_snapshot,
            depth.wrapping_add(1 as libc::c_int as libc::c_uint),
        );
        if error as libc::c_uint != RMT_ERROR_NONE as libc::c_int as libc::c_uint {
            return error;
        }
        child_property = (*child_property).nextSibling;
    }
    return RMT_ERROR_NONE;
}
pub unsafe extern "C" fn _rmt_PropertySnapshotAll() -> rmtError {
    let mut error: rmtError = RMT_ERROR_NONE;
    let mut first_snapshot: *mut PropertySnapshot = 0 as *mut PropertySnapshot;
    let mut prev_snapshot: *mut PropertySnapshot = 0 as *mut PropertySnapshot;
    let mut payload: *mut Msg_PropertySnapshot = 0 as *mut Msg_PropertySnapshot;
    let mut message: *mut Message = 0 as *mut Message;
    let mut nb_snapshot_allocs: rmtU32 = 0;
    if g_Remotery.is_null() {
        return RMT_ERROR_REMOTERY_NOT_CREATED;
    }
    if ((*g_Remotery).rootProperty.firstChild).is_null() {
        return RMT_ERROR_NONE;
    }
    nb_snapshot_allocs = (*(*g_Remotery).propertyAllocator).nb_inuse as rmtU32;
    first_snapshot = 0 as *mut PropertySnapshot;
    prev_snapshot = 0 as *mut PropertySnapshot;
    mtxLock(&mut (*g_Remotery).propertyMutex);
    error = TakePropertySnapshot(
        &mut (*g_Remotery).rootProperty,
        0 as *mut PropertySnapshot,
        &mut first_snapshot,
        &mut prev_snapshot,
        0 as libc::c_int as rmtU32,
    );
    if (g_Settings.snapshot_callback).is_some() {
        (g_Settings.snapshot_callback)
            .unwrap()(g_Settings.snapshot_context, &mut (*g_Remotery).rootProperty);
    }
    mtxUnlock(&mut (*g_Remotery).propertyMutex);
    if error as libc::c_uint != RMT_ERROR_NONE as libc::c_int as libc::c_uint {
        FreePropertySnapshots(first_snapshot);
        return error;
    }
    message = rmtMessageQueue_AllocMessage(
        (*g_Remotery).mq_to_rmt_thread,
        ::std::mem::size_of::<Msg_PropertySnapshot>() as libc::c_ulong as rmtU32,
        0 as *mut ThreadProfiler,
    );
    if message.is_null() {
        FreePropertySnapshots(first_snapshot);
        return RMT_ERROR_UNKNOWN;
    }
    payload = ((*message).payload).as_mut_ptr() as *mut Msg_PropertySnapshot;
    (*payload).rootSnapshot = first_snapshot;
    (*payload)
        .nbSnapshots = ((*(*g_Remotery).propertyAllocator).nb_inuse as libc::c_uint)
        .wrapping_sub(nb_snapshot_allocs);
    (*payload).propertyFrame = (*g_Remotery).propertyFrame;
    rmtMessageQueue_CommitMessage(message, MsgID_PropertySnapshot);
    return RMT_ERROR_NONE;
}
unsafe extern "C" fn PropertyFrameReset(
    mut rmt: *mut Remotery,
    mut first_property: *mut rmtProperty,
) {
    let mut property: *mut rmtProperty = 0 as *mut rmtProperty;
    property = first_property;
    while !property.is_null() {
        PropertyFrameReset(rmt, (*property).firstChild);
        let mut changed: rmtBool = 0 as libc::c_int as rmtBool;
        match (*property).type_0 as libc::c_uint {
            1 => {
                changed = ((*property).lastFrameValue.Bool != (*property).value.Bool)
                    as libc::c_int as rmtBool;
            }
            2 | 3 | 4 => {
                changed = ((*property).lastFrameValue.U32 != (*property).value.U32)
                    as libc::c_int as rmtBool;
            }
            5 | 6 | 7 => {
                changed = ((*property).lastFrameValue.U64 != (*property).value.U64)
                    as libc::c_int as rmtBool;
            }
            0 | _ => {}
        }
        if changed != 0 {
            (*property).prevValue = (*property).lastFrameValue;
            (*property).prevValueFrame = (*rmt).propertyFrame;
        }
        (*property).lastFrameValue = (*property).value;
        if (*property).flags as libc::c_uint
            & RMT_PropertyFlags_FrameReset as libc::c_int as libc::c_uint
            != 0 as libc::c_int as libc::c_uint
        {
            (*property).value = (*property).defaultValue;
        }
        property = (*property).nextSibling;
    }
}
pub unsafe extern "C" fn _rmt_PropertyFrameResetAll() {
    if g_Remotery.is_null() {
        return;
    }
    mtxLock(&mut (*g_Remotery).propertyMutex);
    PropertyFrameReset(g_Remotery, (*g_Remotery).rootProperty.firstChild);
    mtxUnlock(&mut (*g_Remotery).propertyMutex);
    (*g_Remotery).propertyFrame = ((*g_Remotery).propertyFrame).wrapping_add(1);
    (*g_Remotery).propertyFrame;
}
