use ::libc;
extern "C" {
    pub type Remotery;
    fn rand() -> libc::c_int;
    fn sin(_: libc::c_double) -> libc::c_double;
    fn signal(__sig: libc::c_int, __handler: __sighandler_t) -> __sighandler_t;
    fn printf(_: *const libc::c_char, _: ...) -> libc::c_int;
    fn _rmt_CreateGlobalInstance(remotery: *mut *mut Remotery) -> rmtError;
    fn _rmt_DestroyGlobalInstance(remotery: *mut Remotery);
    fn _rmt_LogText(text: rmtPStr);
    fn _rmt_BeginCPUSample(name: rmtPStr, flags: rmtU32, hash_cache: *mut rmtU32);
    fn _rmt_EndCPUSample();
}
pub type __sighandler_t = Option::<unsafe extern "C" fn(libc::c_int) -> ()>;
pub type rmtU32 = libc::c_uint;
pub type rmtPStr = *const libc::c_char;
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
pub type rmtSampleFlags = libc::c_uint;
pub const RMTSF_SendOnClose: rmtSampleFlags = 8;
pub const RMTSF_Root: rmtSampleFlags = 4;
pub const RMTSF_Recursive: rmtSampleFlags = 2;
pub const RMTSF_Aggregate: rmtSampleFlags = 1;
pub const RMTSF_None: rmtSampleFlags = 0;
pub unsafe extern "C" fn aggregateFunction() {
    static mut rmt_sample_hash_aggregate: rmtU32 = 0 as libc::c_int as rmtU32;
    _rmt_BeginCPUSample(
        b"aggregate\0" as *const u8 as *const libc::c_char,
        RMTSF_Aggregate as libc::c_int as rmtU32,
        &mut rmt_sample_hash_aggregate,
    );
    _rmt_EndCPUSample();
}
pub unsafe extern "C" fn recursiveFunction(mut depth: libc::c_int) {
    static mut rmt_sample_hash_recursive: rmtU32 = 0 as libc::c_int as rmtU32;
    _rmt_BeginCPUSample(
        b"recursive\0" as *const u8 as *const libc::c_char,
        RMTSF_Recursive as libc::c_int as rmtU32,
        &mut rmt_sample_hash_recursive,
    );
    if depth < 5 as libc::c_int {
        recursiveFunction(depth + 1 as libc::c_int);
    }
    _rmt_EndCPUSample();
}
pub unsafe extern "C" fn delay() -> libc::c_double {
    let mut i: libc::c_int = 0;
    let mut end: libc::c_int = 0;
    let mut j: libc::c_double = 0 as libc::c_int as libc::c_double;
    static mut rmt_sample_hash_delay: rmtU32 = 0 as libc::c_int as rmtU32;
    _rmt_BeginCPUSample(
        b"delay\0" as *const u8 as *const libc::c_char,
        0 as libc::c_int as rmtU32,
        &mut rmt_sample_hash_delay,
    );
    i = 0 as libc::c_int;
    end = rand() / 100 as libc::c_int;
    while i < end {
        j += sin(i as libc::c_double);
        i += 1;
        i;
    }
    recursiveFunction(0 as libc::c_int);
    aggregateFunction();
    aggregateFunction();
    aggregateFunction();
    _rmt_EndCPUSample();
    return j;
}
pub static mut sig: libc::c_int = 0 as libc::c_int;
pub unsafe extern "C" fn sigintHandler(mut sig_num: libc::c_int) {
    sig = sig_num;
    printf(b"Interrupted\n\0" as *const u8 as *const libc::c_char);
}
unsafe fn main_0() -> libc::c_int {
    let mut rmt: *mut Remotery = 0 as *mut Remotery;
    let mut error: rmtError = RMT_ERROR_NONE;
    signal(
        2 as libc::c_int,
        Some(sigintHandler as unsafe extern "C" fn(libc::c_int) -> ()),
    );
    error = _rmt_CreateGlobalInstance(&mut rmt);
    if RMT_ERROR_NONE as libc::c_int as libc::c_uint != error as libc::c_uint {
        printf(
            b"Error launching Remotery %d\n\0" as *const u8 as *const libc::c_char,
            error as libc::c_uint,
        );
        return -(1 as libc::c_int);
    }
    while sig == 0 as libc::c_int {
        _rmt_LogText(b"start profiling\0" as *const u8 as *const libc::c_char);
        delay();
        _rmt_LogText(b"end profiling\0" as *const u8 as *const libc::c_char);
    }
    _rmt_DestroyGlobalInstance(rmt);
    printf(b"Cleaned up and quit\n\0" as *const u8 as *const libc::c_char);
    return 0 as libc::c_int;
}
pub fn main() {
    unsafe { ::std::process::exit(main_0() as i32) }
}
