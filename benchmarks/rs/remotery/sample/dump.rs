use ::libc;
extern "C" {
    pub type Msg_SampleTree;
    pub type Sample;
    pub type Remotery;
    fn rand() -> libc::c_int;
    fn sin(_: libc::c_double) -> libc::c_double;
    fn signal(__sig: libc::c_int, __handler: __sighandler_t) -> __sighandler_t;
    fn printf(_: *const libc::c_char, _: ...) -> libc::c_int;
    fn strcmp(_: *const libc::c_char, _: *const libc::c_char) -> libc::c_int;
    fn _rmt_PropertySetValue(property: *mut rmtProperty);
    fn _rmt_PropertyAddValue(property: *mut rmtProperty, add_value: rmtPropertyValue);
    fn _rmt_PropertySnapshotAll() -> rmtError;
    fn _rmt_PropertyFrameResetAll();
    fn _rmt_Settings() -> *mut rmtSettings;
    fn _rmt_CreateGlobalInstance(remotery: *mut *mut Remotery) -> rmtError;
    fn _rmt_DestroyGlobalInstance(remotery: *mut Remotery);
    fn _rmt_LogText(text: rmtPStr);
    fn _rmt_BeginCPUSample(name: rmtPStr, flags: rmtU32, hash_cache: *mut rmtU32);
    fn _rmt_EndCPUSample();
    fn _rmt_IterateChildren(iter: *mut rmtSampleIterator, sample: *mut rmtSample);
    fn _rmt_IterateNext(iter: *mut rmtSampleIterator) -> rmtBool;
    fn _rmt_SampleTreeGetThreadName(
        sample_tree: *mut rmtSampleTree,
    ) -> *const libc::c_char;
    fn _rmt_SampleTreeGetRootSample(sample_tree: *mut rmtSampleTree) -> *mut rmtSample;
    fn _rmt_SampleGetName(sample: *mut rmtSample) -> *const libc::c_char;
    fn _rmt_SampleGetCallCount(sample: *mut rmtSample) -> rmtU32;
    fn _rmt_SampleGetTime(sample: *mut rmtSample) -> rmtU64;
    fn _rmt_SampleGetSelfTime(sample: *mut rmtSample) -> rmtU64;
    fn _rmt_SampleGetColour(
        sample: *mut rmtSample,
        r: *mut rmtU8,
        g: *mut rmtU8,
        b: *mut rmtU8,
    );
    fn _rmt_SampleGetType(sample: *mut rmtSample) -> rmtSampleType;
    fn _rmt_PropertyIterateChildren(
        iter: *mut rmtPropertyIterator,
        property: *mut rmtProperty,
    );
    fn _rmt_PropertyIterateNext(iter: *mut rmtPropertyIterator) -> rmtBool;
    fn _rmt_PropertyGetType(property: *mut rmtProperty) -> rmtPropertyType;
    fn _rmt_PropertyGetName(property: *mut rmtProperty) -> *const libc::c_char;
    fn _rmt_PropertyGetValue(property: *mut rmtProperty) -> rmtPropertyValue;
}
pub type __sighandler_t = Option::<unsafe extern "C" fn(libc::c_int) -> ()>;
pub type rmtBool = libc::c_uint;
pub type rmtU8 = libc::c_uchar;
pub type rmtU16 = libc::c_ushort;
pub type rmtU32 = libc::c_uint;
pub type rmtU64 = libc::c_ulonglong;
pub type rmtS32 = libc::c_int;
pub type rmtS64 = libc::c_longlong;
pub type rmtF32 = libc::c_float;
pub type rmtF64 = libc::c_double;
pub type rmtPStr = *const libc::c_char;
pub type rmtSampleTree = Msg_SampleTree;
pub type rmtSample = Sample;
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
pub type rmtSampleType = libc::c_uint;
pub const RMT_SampleType_Count: rmtSampleType = 6;
pub const RMT_SampleType_Metal: rmtSampleType = 5;
pub const RMT_SampleType_OpenGL: rmtSampleType = 4;
pub const RMT_SampleType_D3D12: rmtSampleType = 3;
pub const RMT_SampleType_D3D11: rmtSampleType = 2;
pub const RMT_SampleType_CUDA: rmtSampleType = 1;
pub const RMT_SampleType_CPU: rmtSampleType = 0;
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
pub static mut Game: rmtProperty = {
    let mut init = rmtProperty {
        initialised: 0 as libc::c_int as rmtBool,
        type_0: RMT_PropertyType_rmtGroup,
        flags: RMT_PropertyFlags_NoFlags,
        value: rmtPropertyValue {
            Bool: 0 as libc::c_int as rmtBool,
        },
        lastFrameValue: rmtPropertyValue {
            Bool: 0 as libc::c_int as rmtBool,
        },
        prevValue: rmtPropertyValue {
            Bool: 0 as libc::c_int as rmtBool,
        },
        prevValueFrame: 0 as libc::c_int as rmtU32,
        name: b"Game\0" as *const u8 as *const libc::c_char,
        description: b"Game Properties\0" as *const u8 as *const libc::c_char,
        defaultValue: rmtPropertyValue {
            Bool: 0 as libc::c_int as rmtBool,
        },
        parent: 0 as *const rmtProperty as *mut rmtProperty,
        firstChild: 0 as *const rmtProperty as *mut rmtProperty,
        lastChild: 0 as *const rmtProperty as *mut rmtProperty,
        nextSibling: 0 as *const rmtProperty as *mut rmtProperty,
        nameHash: 0,
        uniqueID: 0,
    };
    init
};
pub static mut WasUpdated: rmtProperty = unsafe {
    {
        let mut init = rmtProperty {
            initialised: 0 as libc::c_int as rmtBool,
            type_0: RMT_PropertyType_rmtBool,
            flags: RMT_PropertyFlags_FrameReset,
            value: rmtPropertyValue {
                Bool: 0 as libc::c_int as rmtBool,
            },
            lastFrameValue: rmtPropertyValue {
                Bool: 0 as libc::c_int as rmtBool,
            },
            prevValue: rmtPropertyValue {
                Bool: 0 as libc::c_int as rmtBool,
            },
            prevValueFrame: 0 as libc::c_int as rmtU32,
            name: b"WasUpdated\0" as *const u8 as *const libc::c_char,
            description: b"Was the game loop executed this frame?\0" as *const u8
                as *const libc::c_char,
            defaultValue: rmtPropertyValue {
                Bool: 0 as libc::c_int as rmtBool,
            },
            parent: &Game as *const rmtProperty as *mut rmtProperty,
            firstChild: 0 as *const rmtProperty as *mut rmtProperty,
            lastChild: 0 as *const rmtProperty as *mut rmtProperty,
            nextSibling: 0 as *const rmtProperty as *mut rmtProperty,
            nameHash: 0,
            uniqueID: 0,
        };
        init
    }
};
pub static mut RecursiveDepth: rmtProperty = unsafe {
    {
        let mut init = rmtProperty {
            initialised: 0 as libc::c_int as rmtBool,
            type_0: RMT_PropertyType_rmtU32,
            flags: RMT_PropertyFlags_FrameReset,
            value: rmtPropertyValue {
                U32: 0 as libc::c_int as rmtU32,
            },
            lastFrameValue: rmtPropertyValue {
                U32: 0 as libc::c_int as rmtU32,
            },
            prevValue: rmtPropertyValue {
                U32: 0 as libc::c_int as rmtU32,
            },
            prevValueFrame: 0 as libc::c_int as rmtU32,
            name: b"RecursiveDepth\0" as *const u8 as *const libc::c_char,
            description: b"How deep did we go in recursiveFunction?\0" as *const u8
                as *const libc::c_char,
            defaultValue: rmtPropertyValue {
                U32: 0 as libc::c_int as rmtU32,
            },
            parent: &Game as *const rmtProperty as *mut rmtProperty,
            firstChild: 0 as *const rmtProperty as *mut rmtProperty,
            lastChild: 0 as *const rmtProperty as *mut rmtProperty,
            nextSibling: 0 as *const rmtProperty as *mut rmtProperty,
            nameHash: 0,
            uniqueID: 0,
        };
        init
    }
};
pub static mut Accumulated: rmtProperty = unsafe {
    {
        let mut init = rmtProperty {
            initialised: 0 as libc::c_int as rmtBool,
            type_0: RMT_PropertyType_rmtF32,
            flags: RMT_PropertyFlags_FrameReset,
            value: rmtPropertyValue {
                F32: 0 as libc::c_int as rmtF32,
            },
            lastFrameValue: rmtPropertyValue {
                F32: 0 as libc::c_int as rmtF32,
            },
            prevValue: rmtPropertyValue {
                F32: 0 as libc::c_int as rmtF32,
            },
            prevValueFrame: 0 as libc::c_int as rmtU32,
            name: b"Accumulated\0" as *const u8 as *const libc::c_char,
            description: b"What was the latest value?\0" as *const u8
                as *const libc::c_char,
            defaultValue: rmtPropertyValue {
                F32: 0 as libc::c_int as rmtF32,
            },
            parent: &Game as *const rmtProperty as *mut rmtProperty,
            firstChild: 0 as *const rmtProperty as *mut rmtProperty,
            lastChild: 0 as *const rmtProperty as *mut rmtProperty,
            nextSibling: 0 as *const rmtProperty as *mut rmtProperty,
            nameHash: 0,
            uniqueID: 0,
        };
        init
    }
};
pub static mut FrameCounter: rmtProperty = unsafe {
    {
        let mut init = rmtProperty {
            initialised: 0 as libc::c_int as rmtBool,
            type_0: RMT_PropertyType_rmtU32,
            flags: RMT_PropertyFlags_NoFlags,
            value: rmtPropertyValue {
                U32: 0 as libc::c_int as rmtU32,
            },
            lastFrameValue: rmtPropertyValue {
                U32: 0 as libc::c_int as rmtU32,
            },
            prevValue: rmtPropertyValue {
                U32: 0 as libc::c_int as rmtU32,
            },
            prevValueFrame: 0 as libc::c_int as rmtU32,
            name: b"FrameCounter\0" as *const u8 as *const libc::c_char,
            description: b"What is the current frame number?\0" as *const u8
                as *const libc::c_char,
            defaultValue: rmtPropertyValue {
                U32: 0 as libc::c_int as rmtU32,
            },
            parent: &Game as *const rmtProperty as *mut rmtProperty,
            firstChild: 0 as *const rmtProperty as *mut rmtProperty,
            lastChild: 0 as *const rmtProperty as *mut rmtProperty,
            nextSibling: 0 as *const rmtProperty as *mut rmtProperty,
            nameHash: 0,
            uniqueID: 0,
        };
        init
    }
};
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
    RecursiveDepth.value.U32 = depth as rmtU32;
    _rmt_PropertySetValue(&mut RecursiveDepth);
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
        let mut v: libc::c_double = sin(i as libc::c_double);
        j += v;
        Accumulated.value.F32 = (Accumulated.value.F32 as libc::c_double + v) as rmtF32;
        let mut delta_value: rmtPropertyValue = rmtPropertyValue {
            F32: v as rmtF32,
        };
        _rmt_PropertyAddValue(&mut Accumulated, delta_value);
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
pub unsafe extern "C" fn printIndent(mut indent: libc::c_int) {
    let mut i: libc::c_int = 0;
    i = 0 as libc::c_int;
    while i < indent {
        printf(b"  \0" as *const u8 as *const libc::c_char);
        i += 1;
        i;
    }
}
pub unsafe extern "C" fn printSample(
    mut sample: *mut rmtSample,
    mut indent: libc::c_int,
) {
    let mut name: *const libc::c_char = _rmt_SampleGetName(sample);
    let mut callcount: rmtU32 = _rmt_SampleGetCallCount(sample);
    let mut time: rmtU64 = _rmt_SampleGetTime(sample);
    let mut self_time: rmtU64 = _rmt_SampleGetSelfTime(sample);
    let mut type_0: rmtSampleType = _rmt_SampleGetType(sample);
    let mut r: rmtU8 = 0;
    let mut g: rmtU8 = 0;
    let mut b: rmtU8 = 0;
    _rmt_SampleGetColour(sample, &mut r, &mut g, &mut b);
    printIndent(indent);
    printf(
        b"%s %u  time: %llu  self: %llu type: %d  color: 0x%02x%02x%02x\n\0" as *const u8
            as *const libc::c_char,
        name,
        callcount,
        time,
        self_time,
        type_0 as libc::c_uint,
        r as libc::c_int,
        g as libc::c_int,
        b as libc::c_int,
    );
}
pub unsafe extern "C" fn printTree(mut sample: *mut rmtSample, mut indent: libc::c_int) {
    let mut iter: rmtSampleIterator = rmtSampleIterator {
        sample: 0 as *mut rmtSample,
        initial: 0 as *mut rmtSample,
    };
    printSample(sample, indent);
    _rmt_IterateChildren(&mut iter, sample);
    while _rmt_IterateNext(&mut iter) != 0 {
        printTree(iter.sample, indent + 1 as libc::c_int);
    }
}
pub unsafe extern "C" fn dumpTree(
    mut ctx: *mut libc::c_void,
    mut sample_tree: *mut rmtSampleTree,
) {
    let mut root: *mut rmtSample = _rmt_SampleTreeGetRootSample(sample_tree);
    let mut thread_name: *const libc::c_char = _rmt_SampleTreeGetThreadName(sample_tree);
    if strcmp(b"Remotery\0" as *const u8 as *const libc::c_char, thread_name)
        == 0 as libc::c_int
    {
        return;
    }
    printf(
        b"// ********************   DUMP TREE: %s   ************************\n\0"
            as *const u8 as *const libc::c_char,
        thread_name,
    );
    printTree(root, 0 as libc::c_int);
}
pub unsafe extern "C" fn printProperty(
    mut property: *mut rmtProperty,
    mut indent: libc::c_int,
) {
    let mut iter: rmtPropertyIterator = rmtPropertyIterator {
        property: 0 as *mut rmtProperty,
        initial: 0 as *mut rmtProperty,
    };
    let mut name: *const libc::c_char = _rmt_PropertyGetName(property);
    let mut type_0: rmtPropertyType = _rmt_PropertyGetType(property);
    let mut value: rmtPropertyValue = _rmt_PropertyGetValue(property);
    printIndent(indent);
    printf(b"%s: \0" as *const u8 as *const libc::c_char, name);
    match type_0 as libc::c_uint {
        1 => {
            printf(
                b"%s\n\0" as *const u8 as *const libc::c_char,
                if value.Bool != 0 {
                    b"true\0" as *const u8 as *const libc::c_char
                } else {
                    b"false\0" as *const u8 as *const libc::c_char
                },
            );
        }
        2 => {
            printf(b"%d\n\0" as *const u8 as *const libc::c_char, value.S32);
        }
        3 => {
            printf(b"%u\n\0" as *const u8 as *const libc::c_char, value.U32);
        }
        4 => {
            printf(
                b"%f\n\0" as *const u8 as *const libc::c_char,
                value.F32 as libc::c_double,
            );
        }
        5 => {
            printf(b"%lld\n\0" as *const u8 as *const libc::c_char, value.S64);
        }
        6 => {
            printf(b"%llu\n\0" as *const u8 as *const libc::c_char, value.U64);
        }
        7 => {
            printf(b"%g\n\0" as *const u8 as *const libc::c_char, value.F64);
        }
        0 => {
            printf(b"\n\0" as *const u8 as *const libc::c_char);
        }
        _ => {}
    }
    _rmt_PropertyIterateChildren(&mut iter, property);
    while _rmt_PropertyIterateNext(&mut iter) != 0 {
        printProperty(iter.property, indent + 1 as libc::c_int);
    }
}
pub unsafe extern "C" fn dumpProperties(
    mut ctx: *mut libc::c_void,
    mut root: *mut rmtProperty,
) {
    let mut iter: rmtPropertyIterator = rmtPropertyIterator {
        property: 0 as *mut rmtProperty,
        initial: 0 as *mut rmtProperty,
    };
    printf(
        b"// ********************   DUMP PROPERTIES:      ************************\n\0"
            as *const u8 as *const libc::c_char,
    );
    _rmt_PropertyIterateChildren(&mut iter, root);
    while _rmt_PropertyIterateNext(&mut iter) != 0 {
        printProperty(iter.property, 0 as libc::c_int);
    }
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
    let mut settings: *mut rmtSettings = _rmt_Settings();
    if !settings.is_null() {
        (*settings)
            .sampletree_handler = Some(
            dumpTree as unsafe extern "C" fn(*mut libc::c_void, *mut rmtSampleTree) -> (),
        );
        (*settings).sampletree_context = 0 as *mut libc::c_void;
        (*settings)
            .snapshot_callback = Some(
            dumpProperties
                as unsafe extern "C" fn(*mut libc::c_void, *mut rmtProperty) -> (),
        );
        (*settings).snapshot_context = 0 as *mut libc::c_void;
    }
    error = _rmt_CreateGlobalInstance(&mut rmt);
    if RMT_ERROR_NONE as libc::c_int as libc::c_uint != error as libc::c_uint {
        printf(
            b"Error launching Remotery %d\n\0" as *const u8 as *const libc::c_char,
            error as libc::c_uint,
        );
        return -(1 as libc::c_int);
    }
    let mut max_count: libc::c_int = 5 as libc::c_int;
    while sig == 0 as libc::c_int
        && {
            max_count -= 1;
            max_count > 0 as libc::c_int
        }
    {
        _rmt_LogText(b"start profiling\0" as *const u8 as *const libc::c_char);
        delay();
        _rmt_LogText(b"end profiling\0" as *const u8 as *const libc::c_char);
        WasUpdated.value.Bool = 1 as libc::c_int as rmtBool;
        _rmt_PropertySetValue(&mut WasUpdated);
        FrameCounter
            .value
            .U32 = (FrameCounter.value.U32 as libc::c_uint)
            .wrapping_add(1 as libc::c_int as libc::c_uint) as rmtU32 as rmtU32;
        let mut delta_value: rmtPropertyValue = rmtPropertyValue {
            U32: 1 as libc::c_int as rmtU32,
        };
        _rmt_PropertyAddValue(&mut FrameCounter, delta_value);
        _rmt_PropertySnapshotAll();
        _rmt_PropertyFrameResetAll();
    }
    _rmt_DestroyGlobalInstance(rmt);
    printf(b"Cleaned up and quit\n\0" as *const u8 as *const libc::c_char);
    return 0 as libc::c_int;
}
pub fn main() {
    unsafe { ::std::process::exit(main_0() as i32) }
}
