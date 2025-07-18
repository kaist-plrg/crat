use ::libc;
extern "C" {
    pub type _IO_wide_data;
    pub type _IO_codecvt;
    pub type _IO_marker;
    pub type cpHashSet;
    pub type cpContactBufferHeader;
    static mut stdout: *mut FILE;
    fn fflush(__stream: *mut FILE) -> libc::c_int;
    fn printf(_: *const libc::c_char, _: ...) -> libc::c_int;
    fn sprintf(_: *mut libc::c_char, _: *const libc::c_char, _: ...) -> libc::c_int;
    fn vsnprintf(
        _: *mut libc::c_char,
        _: libc::c_ulong,
        _: *const libc::c_char,
        _: ::std::ffi::VaList,
    ) -> libc::c_int;
    fn strcmp(_: *const libc::c_char, _: *const libc::c_char) -> libc::c_int;
    fn memcpy(
        _: *mut libc::c_void,
        _: *const libc::c_void,
        _: libc::c_ulong,
    ) -> *mut libc::c_void;
    fn pow(_: libc::c_double, _: libc::c_double) -> libc::c_double;
    fn exit(_: libc::c_int) -> !;
    fn abort() -> !;
    fn srand(__seed: libc::c_uint);
    fn cpMessage(
        condition: *const libc::c_char,
        file: *const libc::c_char,
        line: libc::c_int,
        isError: libc::c_int,
        isHardError: libc::c_int,
        message: *const libc::c_char,
        _: ...
    );
    fn cpBodyNewKinematic() -> *mut cpBody;
    fn cpBodyFree(body: *mut cpBody);
    fn cpBodyIsSleeping(body: *const cpBody) -> cpBool;
    fn cpBodyGetMass(body: *const cpBody) -> cpFloat;
    fn cpBodyWorldToLocal(body: *const cpBody, point: cpVect) -> cpVect;
    fn cpConstraintFree(constraint: *mut cpConstraint);
    fn cpPivotJointNew2(
        a: *mut cpBody,
        b: *mut cpBody,
        anchorA: cpVect,
        anchorB: cpVect,
    ) -> *mut cpConstraint;
    fn cpSpaceAddConstraint(
        space_0: *mut cpSpace,
        constraint: *mut cpConstraint,
    ) -> *mut cpConstraint;
    fn cpSpaceRemoveShape(space_0: *mut cpSpace, shape: *mut cpShape);
    fn cpSpaceRemoveBody(space_0: *mut cpSpace, body: *mut cpBody);
    fn cpSpaceRemoveConstraint(space_0: *mut cpSpace, constraint: *mut cpConstraint);
    fn cpSpaceAddPostStepCallback(
        space_0: *mut cpSpace,
        func: cpPostStepFunc,
        key: *mut libc::c_void,
        data: *mut libc::c_void,
    ) -> cpBool;
    fn cpSpacePointQueryNearest(
        space_0: *mut cpSpace,
        point: cpVect,
        maxDistance: cpFloat,
        filter: cpShapeFilter,
        out: *mut cpPointQueryInfo,
    ) -> *mut cpShape;
    fn cpSpaceEachBody(
        space_0: *mut cpSpace,
        func: cpSpaceBodyIteratorFunc,
        data: *mut libc::c_void,
    );
    fn cpSpaceEachShape(
        space_0: *mut cpSpace,
        func: cpSpaceShapeIteratorFunc,
        data: *mut libc::c_void,
    );
    fn cpSpaceEachConstraint(
        space_0: *mut cpSpace,
        func: cpSpaceConstraintIteratorFunc,
        data: *mut libc::c_void,
    );
    fn cpSpaceDebugDraw(space_0: *mut cpSpace, options: *mut cpSpaceDebugDrawOptions);
    fn cpShapeFree(shape: *mut cpShape);
    fn cpShapeGetSensor(shape: *const cpShape) -> cpBool;
    fn cpShapeGetBody(shape: *const cpShape) -> *mut cpBody;
    fn ChipmunkDebugDrawInit();
    static mut ChipmunkDebugDrawVPMatrix: cpTransform;
    static mut ChipmunkDebugDrawPointLineScale: libc::c_float;
    fn ChipmunkDebugDrawCircle(
        pos: cpVect,
        angle: cpFloat,
        radius: cpFloat,
        outlineColor: cpSpaceDebugColor,
        fillColor: cpSpaceDebugColor,
    );
    fn ChipmunkDebugDrawSegment(a: cpVect, b: cpVect, color: cpSpaceDebugColor);
    fn ChipmunkDebugDrawFatSegment(
        a: cpVect,
        b: cpVect,
        radius: cpFloat,
        outlineColor: cpSpaceDebugColor,
        fillColor: cpSpaceDebugColor,
    );
    fn ChipmunkDebugDrawClearRenderer();
    fn ChipmunkDebugDrawFlushRenderer();
    fn ChipmunkDebugDrawPushRenderer();
    fn ChipmunkDebugDrawPopRenderer();
    fn ChipmunkDebugDrawPolygon(
        count: libc::c_int,
        verts: *const cpVect,
        radius: cpFloat,
        outlineColor: cpSpaceDebugColor,
        fillColor: cpSpaceDebugColor,
    );
    fn ChipmunkDebugDrawDot(size: cpFloat, pos: cpVect, fillColor: cpSpaceDebugColor);
    static mut ChipmunkDemoTextMatrix: cpTransform;
    fn ChipmunkDemoTextInit();
    fn ChipmunkDemoTextDrawString(pos: cpVect, str: *const libc::c_char);
    fn ChipmunkDemoTextFlushRenderer();
    fn ChipmunkDemoTextClearRenderer();
    fn ChipmunkDemoTextPushRenderer();
    fn ChipmunkDemoTextPopRenderer();
    fn sapp_width() -> libc::c_int;
    fn sapp_height() -> libc::c_int;
    fn stm_now() -> uint64_t;
    fn stm_sec(ticks: uint64_t) -> libc::c_double;
    fn stm_setup();
    fn sg_commit();
    fn sg_setup(desc: *const sg_desc);
    fn sg_isvalid() -> bool;
    fn sg_begin_default_pass(
        pass_action: *const sg_pass_action,
        width: libc::c_int,
        height: libc::c_int,
    );
    fn sg_end_pass();
    fn sg_shutdown();
    static mut LogoSmash: ChipmunkDemo;
    static mut PyramidStack: ChipmunkDemo;
    static mut Plink: ChipmunkDemo;
    static mut BouncyHexagons: ChipmunkDemo;
    static mut Tumble: ChipmunkDemo;
    static mut PyramidTopple: ChipmunkDemo;
    static mut Planet: ChipmunkDemo;
    static mut Springies: ChipmunkDemo;
    static mut Pump: ChipmunkDemo;
    static mut TheoJansen: ChipmunkDemo;
    static mut Query: ChipmunkDemo;
    static mut OneWay: ChipmunkDemo;
    static mut Player: ChipmunkDemo;
    static mut Joints: ChipmunkDemo;
    static mut Tank: ChipmunkDemo;
    static mut Chains: ChipmunkDemo;
    static mut Crane: ChipmunkDemo;
    static mut Buoyancy: ChipmunkDemo;
    static mut ContactGraph: ChipmunkDemo;
    static mut Slice: ChipmunkDemo;
    static mut Convex: ChipmunkDemo;
    static mut Unicycle: ChipmunkDemo;
    static mut Sticky: ChipmunkDemo;
    static mut Shatter: ChipmunkDemo;
    static mut bench_list: [ChipmunkDemo; 0];
    static mut bench_count: libc::c_int;
}
pub type __builtin_va_list = [__va_list_tag; 1];
#[derive(Copy, Clone)]
#[repr(C)]
pub struct __va_list_tag {
    pub gp_offset: libc::c_uint,
    pub fp_offset: libc::c_uint,
    pub overflow_arg_area: *mut libc::c_void,
    pub reg_save_area: *mut libc::c_void,
}
pub type size_t = libc::c_ulong;
pub type va_list = __builtin_va_list;
pub type __uint8_t = libc::c_uchar;
pub type __uint32_t = libc::c_uint;
pub type __uint64_t = libc::c_ulong;
pub type __off_t = libc::c_long;
pub type __off64_t = libc::c_long;
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
pub type FILE = _IO_FILE;
pub type uint8_t = __uint8_t;
pub type uint32_t = __uint32_t;
pub type uint64_t = __uint64_t;
pub type uintptr_t = libc::c_ulong;
pub type cpFloat = libc::c_double;
pub type cpHashValue = uintptr_t;
pub type cpCollisionID = uint32_t;
pub type cpBool = libc::c_uchar;
pub type cpDataPointer = *mut libc::c_void;
pub type cpCollisionType = uintptr_t;
pub type cpGroup = uintptr_t;
pub type cpBitmask = libc::c_uint;
pub type cpTimestamp = libc::c_uint;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct cpVect {
    pub x: cpFloat,
    pub y: cpFloat,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct cpTransform {
    pub a: cpFloat,
    pub b: cpFloat,
    pub c: cpFloat,
    pub d: cpFloat,
    pub tx: cpFloat,
    pub ty: cpFloat,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct cpArray {
    pub num: libc::c_int,
    pub max: libc::c_int,
    pub arr: *mut *mut libc::c_void,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct cpBody {
    pub velocity_func: cpBodyVelocityFunc,
    pub position_func: cpBodyPositionFunc,
    pub m: cpFloat,
    pub m_inv: cpFloat,
    pub i: cpFloat,
    pub i_inv: cpFloat,
    pub cog: cpVect,
    pub p: cpVect,
    pub v: cpVect,
    pub f: cpVect,
    pub a: cpFloat,
    pub w: cpFloat,
    pub t: cpFloat,
    pub transform: cpTransform,
    pub userData: cpDataPointer,
    pub v_bias: cpVect,
    pub w_bias: cpFloat,
    pub space: *mut cpSpace,
    pub shapeList: *mut cpShape,
    pub arbiterList: *mut cpArbiter,
    pub constraintList: *mut cpConstraint,
    pub sleeping: C2RustUnnamed,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed {
    pub root: *mut cpBody,
    pub next: *mut cpBody,
    pub idleTime: cpFloat,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct cpConstraint {
    pub klass: *const cpConstraintClass,
    pub space: *mut cpSpace,
    pub a: *mut cpBody,
    pub b: *mut cpBody,
    pub next_a: *mut cpConstraint,
    pub next_b: *mut cpConstraint,
    pub maxForce: cpFloat,
    pub errorBias: cpFloat,
    pub maxBias: cpFloat,
    pub collideBodies: cpBool,
    pub preSolve: cpConstraintPreSolveFunc,
    pub postSolve: cpConstraintPostSolveFunc,
    pub userData: cpDataPointer,
}
pub type cpConstraintPostSolveFunc = Option::<
    unsafe extern "C" fn(*mut cpConstraint, *mut cpSpace) -> (),
>;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct cpSpace {
    pub iterations: libc::c_int,
    pub gravity: cpVect,
    pub damping: cpFloat,
    pub idleSpeedThreshold: cpFloat,
    pub sleepTimeThreshold: cpFloat,
    pub collisionSlop: cpFloat,
    pub collisionBias: cpFloat,
    pub collisionPersistence: cpTimestamp,
    pub userData: cpDataPointer,
    pub stamp: cpTimestamp,
    pub curr_dt: cpFloat,
    pub dynamicBodies: *mut cpArray,
    pub staticBodies: *mut cpArray,
    pub rousedBodies: *mut cpArray,
    pub sleepingComponents: *mut cpArray,
    pub shapeIDCounter: cpHashValue,
    pub staticShapes: *mut cpSpatialIndex,
    pub dynamicShapes: *mut cpSpatialIndex,
    pub constraints: *mut cpArray,
    pub arbiters: *mut cpArray,
    pub contactBuffersHead: *mut cpContactBufferHeader,
    pub cachedArbiters: *mut cpHashSet,
    pub pooledArbiters: *mut cpArray,
    pub allocatedBuffers: *mut cpArray,
    pub locked: libc::c_int,
    pub usesWildcards: cpBool,
    pub collisionHandlers: *mut cpHashSet,
    pub defaultHandler: cpCollisionHandler,
    pub skipPostStep: cpBool,
    pub postStepCallbacks: *mut cpArray,
    pub staticBody: *mut cpBody,
    pub _staticBody: cpBody,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct cpCollisionHandler {
    pub typeA: cpCollisionType,
    pub typeB: cpCollisionType,
    pub beginFunc: cpCollisionBeginFunc,
    pub preSolveFunc: cpCollisionPreSolveFunc,
    pub postSolveFunc: cpCollisionPostSolveFunc,
    pub separateFunc: cpCollisionSeparateFunc,
    pub userData: cpDataPointer,
}
pub type cpCollisionSeparateFunc = Option::<
    unsafe extern "C" fn(*mut cpArbiter, *mut cpSpace, cpDataPointer) -> (),
>;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct cpArbiter {
    pub e: cpFloat,
    pub u: cpFloat,
    pub surface_vr: cpVect,
    pub data: cpDataPointer,
    pub a: *const cpShape,
    pub b: *const cpShape,
    pub body_a: *mut cpBody,
    pub body_b: *mut cpBody,
    pub thread_a: cpArbiterThread,
    pub thread_b: cpArbiterThread,
    pub count: libc::c_int,
    pub contacts: *mut cpContact,
    pub n: cpVect,
    pub handler: *mut cpCollisionHandler,
    pub handlerA: *mut cpCollisionHandler,
    pub handlerB: *mut cpCollisionHandler,
    pub swapped: cpBool,
    pub stamp: cpTimestamp,
    pub state: cpArbiterState,
}
pub type cpArbiterState = libc::c_uint;
pub const CP_ARBITER_STATE_INVALIDATED: cpArbiterState = 4;
pub const CP_ARBITER_STATE_CACHED: cpArbiterState = 3;
pub const CP_ARBITER_STATE_IGNORE: cpArbiterState = 2;
pub const CP_ARBITER_STATE_NORMAL: cpArbiterState = 1;
pub const CP_ARBITER_STATE_FIRST_COLLISION: cpArbiterState = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct cpContact {
    pub r1: cpVect,
    pub r2: cpVect,
    pub nMass: cpFloat,
    pub tMass: cpFloat,
    pub bounce: cpFloat,
    pub jnAcc: cpFloat,
    pub jtAcc: cpFloat,
    pub jBias: cpFloat,
    pub bias: cpFloat,
    pub hash: cpHashValue,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct cpArbiterThread {
    pub next: *mut cpArbiter,
    pub prev: *mut cpArbiter,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct cpShape {
    pub klass: *const cpShapeClass,
    pub space: *mut cpSpace,
    pub body: *mut cpBody,
    pub massInfo: cpShapeMassInfo,
    pub bb: cpBB,
    pub sensor: cpBool,
    pub e: cpFloat,
    pub u: cpFloat,
    pub surfaceV: cpVect,
    pub userData: cpDataPointer,
    pub type_0: cpCollisionType,
    pub filter: cpShapeFilter,
    pub next: *mut cpShape,
    pub prev: *mut cpShape,
    pub hashid: cpHashValue,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct cpShapeFilter {
    pub group: cpGroup,
    pub categories: cpBitmask,
    pub mask: cpBitmask,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct cpBB {
    pub l: cpFloat,
    pub b: cpFloat,
    pub r: cpFloat,
    pub t: cpFloat,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct cpShapeMassInfo {
    pub m: cpFloat,
    pub i: cpFloat,
    pub cog: cpVect,
    pub area: cpFloat,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct cpShapeClass {
    pub type_0: cpShapeType,
    pub cacheData: cpShapeCacheDataImpl,
    pub destroy: cpShapeDestroyImpl,
    pub pointQuery: cpShapePointQueryImpl,
    pub segmentQuery: cpShapeSegmentQueryImpl,
}
pub type cpShapeSegmentQueryImpl = Option::<
    unsafe extern "C" fn(
        *const cpShape,
        cpVect,
        cpVect,
        cpFloat,
        *mut cpSegmentQueryInfo,
    ) -> (),
>;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct cpSegmentQueryInfo {
    pub shape: *const cpShape,
    pub point: cpVect,
    pub normal: cpVect,
    pub alpha: cpFloat,
}
pub type cpShapePointQueryImpl = Option::<
    unsafe extern "C" fn(*const cpShape, cpVect, *mut cpPointQueryInfo) -> (),
>;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct cpPointQueryInfo {
    pub shape: *const cpShape,
    pub point: cpVect,
    pub distance: cpFloat,
    pub gradient: cpVect,
}
pub type cpShapeDestroyImpl = Option::<unsafe extern "C" fn(*mut cpShape) -> ()>;
pub type cpShapeCacheDataImpl = Option::<
    unsafe extern "C" fn(*mut cpShape, cpTransform) -> cpBB,
>;
pub type cpShapeType = libc::c_uint;
pub const CP_NUM_SHAPES: cpShapeType = 3;
pub const CP_POLY_SHAPE: cpShapeType = 2;
pub const CP_SEGMENT_SHAPE: cpShapeType = 1;
pub const CP_CIRCLE_SHAPE: cpShapeType = 0;
pub type cpCollisionPostSolveFunc = Option::<
    unsafe extern "C" fn(*mut cpArbiter, *mut cpSpace, cpDataPointer) -> (),
>;
pub type cpCollisionPreSolveFunc = Option::<
    unsafe extern "C" fn(*mut cpArbiter, *mut cpSpace, cpDataPointer) -> cpBool,
>;
pub type cpCollisionBeginFunc = Option::<
    unsafe extern "C" fn(*mut cpArbiter, *mut cpSpace, cpDataPointer) -> cpBool,
>;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct cpSpatialIndex {
    pub klass: *mut cpSpatialIndexClass,
    pub bbfunc: cpSpatialIndexBBFunc,
    pub staticIndex: *mut cpSpatialIndex,
    pub dynamicIndex: *mut cpSpatialIndex,
}
pub type cpSpatialIndexBBFunc = Option::<
    unsafe extern "C" fn(*mut libc::c_void) -> cpBB,
>;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct cpSpatialIndexClass {
    pub destroy: cpSpatialIndexDestroyImpl,
    pub count: cpSpatialIndexCountImpl,
    pub each: cpSpatialIndexEachImpl,
    pub contains: cpSpatialIndexContainsImpl,
    pub insert: cpSpatialIndexInsertImpl,
    pub remove: cpSpatialIndexRemoveImpl,
    pub reindex: cpSpatialIndexReindexImpl,
    pub reindexObject: cpSpatialIndexReindexObjectImpl,
    pub reindexQuery: cpSpatialIndexReindexQueryImpl,
    pub query: cpSpatialIndexQueryImpl,
    pub segmentQuery: cpSpatialIndexSegmentQueryImpl,
}
pub type cpSpatialIndexSegmentQueryImpl = Option::<
    unsafe extern "C" fn(
        *mut cpSpatialIndex,
        *mut libc::c_void,
        cpVect,
        cpVect,
        cpFloat,
        cpSpatialIndexSegmentQueryFunc,
        *mut libc::c_void,
    ) -> (),
>;
pub type cpSpatialIndexSegmentQueryFunc = Option::<
    unsafe extern "C" fn(
        *mut libc::c_void,
        *mut libc::c_void,
        *mut libc::c_void,
    ) -> cpFloat,
>;
pub type cpSpatialIndexQueryImpl = Option::<
    unsafe extern "C" fn(
        *mut cpSpatialIndex,
        *mut libc::c_void,
        cpBB,
        cpSpatialIndexQueryFunc,
        *mut libc::c_void,
    ) -> (),
>;
pub type cpSpatialIndexQueryFunc = Option::<
    unsafe extern "C" fn(
        *mut libc::c_void,
        *mut libc::c_void,
        cpCollisionID,
        *mut libc::c_void,
    ) -> cpCollisionID,
>;
pub type cpSpatialIndexReindexQueryImpl = Option::<
    unsafe extern "C" fn(
        *mut cpSpatialIndex,
        cpSpatialIndexQueryFunc,
        *mut libc::c_void,
    ) -> (),
>;
pub type cpSpatialIndexReindexObjectImpl = Option::<
    unsafe extern "C" fn(*mut cpSpatialIndex, *mut libc::c_void, cpHashValue) -> (),
>;
pub type cpSpatialIndexReindexImpl = Option::<
    unsafe extern "C" fn(*mut cpSpatialIndex) -> (),
>;
pub type cpSpatialIndexRemoveImpl = Option::<
    unsafe extern "C" fn(*mut cpSpatialIndex, *mut libc::c_void, cpHashValue) -> (),
>;
pub type cpSpatialIndexInsertImpl = Option::<
    unsafe extern "C" fn(*mut cpSpatialIndex, *mut libc::c_void, cpHashValue) -> (),
>;
pub type cpSpatialIndexContainsImpl = Option::<
    unsafe extern "C" fn(*mut cpSpatialIndex, *mut libc::c_void, cpHashValue) -> cpBool,
>;
pub type cpSpatialIndexEachImpl = Option::<
    unsafe extern "C" fn(
        *mut cpSpatialIndex,
        cpSpatialIndexIteratorFunc,
        *mut libc::c_void,
    ) -> (),
>;
pub type cpSpatialIndexIteratorFunc = Option::<
    unsafe extern "C" fn(*mut libc::c_void, *mut libc::c_void) -> (),
>;
pub type cpSpatialIndexCountImpl = Option::<
    unsafe extern "C" fn(*mut cpSpatialIndex) -> libc::c_int,
>;
pub type cpSpatialIndexDestroyImpl = Option::<
    unsafe extern "C" fn(*mut cpSpatialIndex) -> (),
>;
pub type cpConstraintPreSolveFunc = Option::<
    unsafe extern "C" fn(*mut cpConstraint, *mut cpSpace) -> (),
>;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct cpConstraintClass {
    pub preStep: cpConstraintPreStepImpl,
    pub applyCachedImpulse: cpConstraintApplyCachedImpulseImpl,
    pub applyImpulse: cpConstraintApplyImpulseImpl,
    pub getImpulse: cpConstraintGetImpulseImpl,
}
pub type cpConstraintGetImpulseImpl = Option::<
    unsafe extern "C" fn(*mut cpConstraint) -> cpFloat,
>;
pub type cpConstraintApplyImpulseImpl = Option::<
    unsafe extern "C" fn(*mut cpConstraint, cpFloat) -> (),
>;
pub type cpConstraintApplyCachedImpulseImpl = Option::<
    unsafe extern "C" fn(*mut cpConstraint, cpFloat) -> (),
>;
pub type cpConstraintPreStepImpl = Option::<
    unsafe extern "C" fn(*mut cpConstraint, cpFloat) -> (),
>;
pub type cpBodyPositionFunc = Option::<unsafe extern "C" fn(*mut cpBody, cpFloat) -> ()>;
pub type cpBodyVelocityFunc = Option::<
    unsafe extern "C" fn(*mut cpBody, cpVect, cpFloat, cpFloat) -> (),
>;
pub type cpPostStepFunc = Option::<
    unsafe extern "C" fn(*mut cpSpace, *mut libc::c_void, *mut libc::c_void) -> (),
>;
pub type cpSpaceBodyIteratorFunc = Option::<
    unsafe extern "C" fn(*mut cpBody, *mut libc::c_void) -> (),
>;
pub type cpSpaceShapeIteratorFunc = Option::<
    unsafe extern "C" fn(*mut cpShape, *mut libc::c_void) -> (),
>;
pub type cpSpaceConstraintIteratorFunc = Option::<
    unsafe extern "C" fn(*mut cpConstraint, *mut libc::c_void) -> (),
>;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct cpSpaceDebugColor {
    pub r: libc::c_float,
    pub g: libc::c_float,
    pub b: libc::c_float,
    pub a: libc::c_float,
}
pub type cpSpaceDebugDrawCircleImpl = Option::<
    unsafe extern "C" fn(
        cpVect,
        cpFloat,
        cpFloat,
        cpSpaceDebugColor,
        cpSpaceDebugColor,
        cpDataPointer,
    ) -> (),
>;
pub type cpSpaceDebugDrawSegmentImpl = Option::<
    unsafe extern "C" fn(cpVect, cpVect, cpSpaceDebugColor, cpDataPointer) -> (),
>;
pub type cpSpaceDebugDrawFatSegmentImpl = Option::<
    unsafe extern "C" fn(
        cpVect,
        cpVect,
        cpFloat,
        cpSpaceDebugColor,
        cpSpaceDebugColor,
        cpDataPointer,
    ) -> (),
>;
pub type cpSpaceDebugDrawPolygonImpl = Option::<
    unsafe extern "C" fn(
        libc::c_int,
        *const cpVect,
        cpFloat,
        cpSpaceDebugColor,
        cpSpaceDebugColor,
        cpDataPointer,
    ) -> (),
>;
pub type cpSpaceDebugDrawDotImpl = Option::<
    unsafe extern "C" fn(cpFloat, cpVect, cpSpaceDebugColor, cpDataPointer) -> (),
>;
pub type cpSpaceDebugDrawColorForShapeImpl = Option::<
    unsafe extern "C" fn(*mut cpShape, cpDataPointer) -> cpSpaceDebugColor,
>;
pub type cpSpaceDebugDrawFlags = libc::c_uint;
pub const CP_SPACE_DEBUG_DRAW_COLLISION_POINTS: cpSpaceDebugDrawFlags = 4;
pub const CP_SPACE_DEBUG_DRAW_CONSTRAINTS: cpSpaceDebugDrawFlags = 2;
pub const CP_SPACE_DEBUG_DRAW_SHAPES: cpSpaceDebugDrawFlags = 1;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct cpSpaceDebugDrawOptions {
    pub drawCircle: cpSpaceDebugDrawCircleImpl,
    pub drawSegment: cpSpaceDebugDrawSegmentImpl,
    pub drawFatSegment: cpSpaceDebugDrawFatSegmentImpl,
    pub drawPolygon: cpSpaceDebugDrawPolygonImpl,
    pub drawDot: cpSpaceDebugDrawDotImpl,
    pub flags: cpSpaceDebugDrawFlags,
    pub shapeOutlineColor: cpSpaceDebugColor,
    pub colorForShape: cpSpaceDebugDrawColorForShapeImpl,
    pub constraintColor: cpSpaceDebugColor,
    pub collisionPointColor: cpSpaceDebugColor,
    pub data: cpDataPointer,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ChipmunkDemo {
    pub name: *const libc::c_char,
    pub timestep: libc::c_double,
    pub initFunc: ChipmunkDemoInitFunc,
    pub updateFunc: ChipmunkDemoUpdateFunc,
    pub drawFunc: ChipmunkDemoDrawFunc,
    pub destroyFunc: ChipmunkDemoDestroyFunc,
}
pub type ChipmunkDemoDestroyFunc = Option::<unsafe extern "C" fn(*mut cpSpace) -> ()>;
pub type ChipmunkDemoDrawFunc = Option::<unsafe extern "C" fn(*mut cpSpace) -> ()>;
pub type ChipmunkDemoUpdateFunc = Option::<
    unsafe extern "C" fn(*mut cpSpace, libc::c_double) -> (),
>;
pub type ChipmunkDemoInitFunc = Option::<unsafe extern "C" fn() -> *mut cpSpace>;
pub type sapp_event_type = libc::c_uint;
pub const _SAPP_EVENTTYPE_FORCE_U32: sapp_event_type = 134217727;
pub const _SAPP_EVENTTYPE_NUM: sapp_event_type = 20;
pub const SAPP_EVENTTYPE_UPDATE_CURSOR: sapp_event_type = 19;
pub const SAPP_EVENTTYPE_RESUMED: sapp_event_type = 18;
pub const SAPP_EVENTTYPE_SUSPENDED: sapp_event_type = 17;
pub const SAPP_EVENTTYPE_RESTORED: sapp_event_type = 16;
pub const SAPP_EVENTTYPE_ICONIFIED: sapp_event_type = 15;
pub const SAPP_EVENTTYPE_RESIZED: sapp_event_type = 14;
pub const SAPP_EVENTTYPE_TOUCHES_CANCELLED: sapp_event_type = 13;
pub const SAPP_EVENTTYPE_TOUCHES_ENDED: sapp_event_type = 12;
pub const SAPP_EVENTTYPE_TOUCHES_MOVED: sapp_event_type = 11;
pub const SAPP_EVENTTYPE_TOUCHES_BEGAN: sapp_event_type = 10;
pub const SAPP_EVENTTYPE_MOUSE_LEAVE: sapp_event_type = 9;
pub const SAPP_EVENTTYPE_MOUSE_ENTER: sapp_event_type = 8;
pub const SAPP_EVENTTYPE_MOUSE_MOVE: sapp_event_type = 7;
pub const SAPP_EVENTTYPE_MOUSE_SCROLL: sapp_event_type = 6;
pub const SAPP_EVENTTYPE_MOUSE_UP: sapp_event_type = 5;
pub const SAPP_EVENTTYPE_MOUSE_DOWN: sapp_event_type = 4;
pub const SAPP_EVENTTYPE_CHAR: sapp_event_type = 3;
pub const SAPP_EVENTTYPE_KEY_UP: sapp_event_type = 2;
pub const SAPP_EVENTTYPE_KEY_DOWN: sapp_event_type = 1;
pub const SAPP_EVENTTYPE_INVALID: sapp_event_type = 0;
pub type sapp_keycode = libc::c_uint;
pub const SAPP_KEYCODE_MENU: sapp_keycode = 348;
pub const SAPP_KEYCODE_RIGHT_SUPER: sapp_keycode = 347;
pub const SAPP_KEYCODE_RIGHT_ALT: sapp_keycode = 346;
pub const SAPP_KEYCODE_RIGHT_CONTROL: sapp_keycode = 345;
pub const SAPP_KEYCODE_RIGHT_SHIFT: sapp_keycode = 344;
pub const SAPP_KEYCODE_LEFT_SUPER: sapp_keycode = 343;
pub const SAPP_KEYCODE_LEFT_ALT: sapp_keycode = 342;
pub const SAPP_KEYCODE_LEFT_CONTROL: sapp_keycode = 341;
pub const SAPP_KEYCODE_LEFT_SHIFT: sapp_keycode = 340;
pub const SAPP_KEYCODE_KP_EQUAL: sapp_keycode = 336;
pub const SAPP_KEYCODE_KP_ENTER: sapp_keycode = 335;
pub const SAPP_KEYCODE_KP_ADD: sapp_keycode = 334;
pub const SAPP_KEYCODE_KP_SUBTRACT: sapp_keycode = 333;
pub const SAPP_KEYCODE_KP_MULTIPLY: sapp_keycode = 332;
pub const SAPP_KEYCODE_KP_DIVIDE: sapp_keycode = 331;
pub const SAPP_KEYCODE_KP_DECIMAL: sapp_keycode = 330;
pub const SAPP_KEYCODE_KP_9: sapp_keycode = 329;
pub const SAPP_KEYCODE_KP_8: sapp_keycode = 328;
pub const SAPP_KEYCODE_KP_7: sapp_keycode = 327;
pub const SAPP_KEYCODE_KP_6: sapp_keycode = 326;
pub const SAPP_KEYCODE_KP_5: sapp_keycode = 325;
pub const SAPP_KEYCODE_KP_4: sapp_keycode = 324;
pub const SAPP_KEYCODE_KP_3: sapp_keycode = 323;
pub const SAPP_KEYCODE_KP_2: sapp_keycode = 322;
pub const SAPP_KEYCODE_KP_1: sapp_keycode = 321;
pub const SAPP_KEYCODE_KP_0: sapp_keycode = 320;
pub const SAPP_KEYCODE_F25: sapp_keycode = 314;
pub const SAPP_KEYCODE_F24: sapp_keycode = 313;
pub const SAPP_KEYCODE_F23: sapp_keycode = 312;
pub const SAPP_KEYCODE_F22: sapp_keycode = 311;
pub const SAPP_KEYCODE_F21: sapp_keycode = 310;
pub const SAPP_KEYCODE_F20: sapp_keycode = 309;
pub const SAPP_KEYCODE_F19: sapp_keycode = 308;
pub const SAPP_KEYCODE_F18: sapp_keycode = 307;
pub const SAPP_KEYCODE_F17: sapp_keycode = 306;
pub const SAPP_KEYCODE_F16: sapp_keycode = 305;
pub const SAPP_KEYCODE_F15: sapp_keycode = 304;
pub const SAPP_KEYCODE_F14: sapp_keycode = 303;
pub const SAPP_KEYCODE_F13: sapp_keycode = 302;
pub const SAPP_KEYCODE_F12: sapp_keycode = 301;
pub const SAPP_KEYCODE_F11: sapp_keycode = 300;
pub const SAPP_KEYCODE_F10: sapp_keycode = 299;
pub const SAPP_KEYCODE_F9: sapp_keycode = 298;
pub const SAPP_KEYCODE_F8: sapp_keycode = 297;
pub const SAPP_KEYCODE_F7: sapp_keycode = 296;
pub const SAPP_KEYCODE_F6: sapp_keycode = 295;
pub const SAPP_KEYCODE_F5: sapp_keycode = 294;
pub const SAPP_KEYCODE_F4: sapp_keycode = 293;
pub const SAPP_KEYCODE_F3: sapp_keycode = 292;
pub const SAPP_KEYCODE_F2: sapp_keycode = 291;
pub const SAPP_KEYCODE_F1: sapp_keycode = 290;
pub const SAPP_KEYCODE_PAUSE: sapp_keycode = 284;
pub const SAPP_KEYCODE_PRINT_SCREEN: sapp_keycode = 283;
pub const SAPP_KEYCODE_NUM_LOCK: sapp_keycode = 282;
pub const SAPP_KEYCODE_SCROLL_LOCK: sapp_keycode = 281;
pub const SAPP_KEYCODE_CAPS_LOCK: sapp_keycode = 280;
pub const SAPP_KEYCODE_END: sapp_keycode = 269;
pub const SAPP_KEYCODE_HOME: sapp_keycode = 268;
pub const SAPP_KEYCODE_PAGE_DOWN: sapp_keycode = 267;
pub const SAPP_KEYCODE_PAGE_UP: sapp_keycode = 266;
pub const SAPP_KEYCODE_UP: sapp_keycode = 265;
pub const SAPP_KEYCODE_DOWN: sapp_keycode = 264;
pub const SAPP_KEYCODE_LEFT: sapp_keycode = 263;
pub const SAPP_KEYCODE_RIGHT: sapp_keycode = 262;
pub const SAPP_KEYCODE_DELETE: sapp_keycode = 261;
pub const SAPP_KEYCODE_INSERT: sapp_keycode = 260;
pub const SAPP_KEYCODE_BACKSPACE: sapp_keycode = 259;
pub const SAPP_KEYCODE_TAB: sapp_keycode = 258;
pub const SAPP_KEYCODE_ENTER: sapp_keycode = 257;
pub const SAPP_KEYCODE_ESCAPE: sapp_keycode = 256;
pub const SAPP_KEYCODE_WORLD_2: sapp_keycode = 162;
pub const SAPP_KEYCODE_WORLD_1: sapp_keycode = 161;
pub const SAPP_KEYCODE_GRAVE_ACCENT: sapp_keycode = 96;
pub const SAPP_KEYCODE_RIGHT_BRACKET: sapp_keycode = 93;
pub const SAPP_KEYCODE_BACKSLASH: sapp_keycode = 92;
pub const SAPP_KEYCODE_LEFT_BRACKET: sapp_keycode = 91;
pub const SAPP_KEYCODE_Z: sapp_keycode = 90;
pub const SAPP_KEYCODE_Y: sapp_keycode = 89;
pub const SAPP_KEYCODE_X: sapp_keycode = 88;
pub const SAPP_KEYCODE_W: sapp_keycode = 87;
pub const SAPP_KEYCODE_V: sapp_keycode = 86;
pub const SAPP_KEYCODE_U: sapp_keycode = 85;
pub const SAPP_KEYCODE_T: sapp_keycode = 84;
pub const SAPP_KEYCODE_S: sapp_keycode = 83;
pub const SAPP_KEYCODE_R: sapp_keycode = 82;
pub const SAPP_KEYCODE_Q: sapp_keycode = 81;
pub const SAPP_KEYCODE_P: sapp_keycode = 80;
pub const SAPP_KEYCODE_O: sapp_keycode = 79;
pub const SAPP_KEYCODE_N: sapp_keycode = 78;
pub const SAPP_KEYCODE_M: sapp_keycode = 77;
pub const SAPP_KEYCODE_L: sapp_keycode = 76;
pub const SAPP_KEYCODE_K: sapp_keycode = 75;
pub const SAPP_KEYCODE_J: sapp_keycode = 74;
pub const SAPP_KEYCODE_I: sapp_keycode = 73;
pub const SAPP_KEYCODE_H: sapp_keycode = 72;
pub const SAPP_KEYCODE_G: sapp_keycode = 71;
pub const SAPP_KEYCODE_F: sapp_keycode = 70;
pub const SAPP_KEYCODE_E: sapp_keycode = 69;
pub const SAPP_KEYCODE_D: sapp_keycode = 68;
pub const SAPP_KEYCODE_C: sapp_keycode = 67;
pub const SAPP_KEYCODE_B: sapp_keycode = 66;
pub const SAPP_KEYCODE_A: sapp_keycode = 65;
pub const SAPP_KEYCODE_EQUAL: sapp_keycode = 61;
pub const SAPP_KEYCODE_SEMICOLON: sapp_keycode = 59;
pub const SAPP_KEYCODE_9: sapp_keycode = 57;
pub const SAPP_KEYCODE_8: sapp_keycode = 56;
pub const SAPP_KEYCODE_7: sapp_keycode = 55;
pub const SAPP_KEYCODE_6: sapp_keycode = 54;
pub const SAPP_KEYCODE_5: sapp_keycode = 53;
pub const SAPP_KEYCODE_4: sapp_keycode = 52;
pub const SAPP_KEYCODE_3: sapp_keycode = 51;
pub const SAPP_KEYCODE_2: sapp_keycode = 50;
pub const SAPP_KEYCODE_1: sapp_keycode = 49;
pub const SAPP_KEYCODE_0: sapp_keycode = 48;
pub const SAPP_KEYCODE_SLASH: sapp_keycode = 47;
pub const SAPP_KEYCODE_PERIOD: sapp_keycode = 46;
pub const SAPP_KEYCODE_MINUS: sapp_keycode = 45;
pub const SAPP_KEYCODE_COMMA: sapp_keycode = 44;
pub const SAPP_KEYCODE_APOSTROPHE: sapp_keycode = 39;
pub const SAPP_KEYCODE_SPACE: sapp_keycode = 32;
pub const SAPP_KEYCODE_INVALID: sapp_keycode = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct sapp_touchpoint {
    pub identifier: uintptr_t,
    pub pos_x: libc::c_float,
    pub pos_y: libc::c_float,
    pub changed: bool,
}
pub type sapp_mousebutton = libc::c_int;
pub const SAPP_MOUSEBUTTON_MIDDLE: sapp_mousebutton = 2;
pub const SAPP_MOUSEBUTTON_RIGHT: sapp_mousebutton = 1;
pub const SAPP_MOUSEBUTTON_LEFT: sapp_mousebutton = 0;
pub const SAPP_MOUSEBUTTON_INVALID: sapp_mousebutton = -1;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct sapp_event {
    pub type_0: sapp_event_type,
    pub frame_count: uint32_t,
    pub key_code: sapp_keycode,
    pub char_code: uint32_t,
    pub key_repeat: bool,
    pub modifiers: uint32_t,
    pub mouse_button: sapp_mousebutton,
    pub mouse_x: libc::c_float,
    pub mouse_y: libc::c_float,
    pub scroll_x: libc::c_float,
    pub scroll_y: libc::c_float,
    pub num_touches: libc::c_int,
    pub touches: [sapp_touchpoint; 8],
    pub window_width: libc::c_int,
    pub window_height: libc::c_int,
    pub framebuffer_width: libc::c_int,
    pub framebuffer_height: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct sapp_desc {
    pub init_cb: Option::<unsafe extern "C" fn() -> ()>,
    pub frame_cb: Option::<unsafe extern "C" fn() -> ()>,
    pub cleanup_cb: Option::<unsafe extern "C" fn() -> ()>,
    pub event_cb: Option::<unsafe extern "C" fn(*const sapp_event) -> ()>,
    pub fail_cb: Option::<unsafe extern "C" fn(*const libc::c_char) -> ()>,
    pub user_data: *mut libc::c_void,
    pub init_userdata_cb: Option::<unsafe extern "C" fn(*mut libc::c_void) -> ()>,
    pub frame_userdata_cb: Option::<unsafe extern "C" fn(*mut libc::c_void) -> ()>,
    pub cleanup_userdata_cb: Option::<unsafe extern "C" fn(*mut libc::c_void) -> ()>,
    pub event_userdata_cb: Option::<
        unsafe extern "C" fn(*const sapp_event, *mut libc::c_void) -> (),
    >,
    pub fail_userdata_cb: Option::<
        unsafe extern "C" fn(*const libc::c_char, *mut libc::c_void) -> (),
    >,
    pub width: libc::c_int,
    pub height: libc::c_int,
    pub sample_count: libc::c_int,
    pub swap_interval: libc::c_int,
    pub high_dpi: bool,
    pub fullscreen: bool,
    pub alpha: bool,
    pub window_title: *const libc::c_char,
    pub user_cursor: bool,
    pub html5_canvas_name: *const libc::c_char,
    pub html5_canvas_resize: bool,
    pub html5_preserve_drawing_buffer: bool,
    pub html5_premultiplied_alpha: bool,
    pub ios_keyboard_resizes_canvas: bool,
    pub gl_force_gles2: bool,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct sg_pass_action {
    pub _start_canary: uint32_t,
    pub colors: [sg_color_attachment_action; 4],
    pub depth: sg_depth_attachment_action,
    pub stencil: sg_stencil_attachment_action,
    pub _end_canary: uint32_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct sg_stencil_attachment_action {
    pub action: sg_action,
    pub val: uint8_t,
}
pub type sg_action = libc::c_uint;
pub const _SG_ACTION_FORCE_U32: sg_action = 2147483647;
pub const _SG_ACTION_NUM: sg_action = 4;
pub const SG_ACTION_DONTCARE: sg_action = 3;
pub const SG_ACTION_LOAD: sg_action = 2;
pub const SG_ACTION_CLEAR: sg_action = 1;
pub const _SG_ACTION_DEFAULT: sg_action = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct sg_depth_attachment_action {
    pub action: sg_action,
    pub val: libc::c_float,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct sg_color_attachment_action {
    pub action: sg_action,
    pub val: [libc::c_float; 4],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct sg_desc {
    pub _start_canary: uint32_t,
    pub buffer_pool_size: libc::c_int,
    pub image_pool_size: libc::c_int,
    pub shader_pool_size: libc::c_int,
    pub pipeline_pool_size: libc::c_int,
    pub pass_pool_size: libc::c_int,
    pub context_pool_size: libc::c_int,
    pub gl_force_gles2: bool,
    pub mtl_device: *const libc::c_void,
    pub mtl_renderpass_descriptor_cb: Option::<
        unsafe extern "C" fn() -> *const libc::c_void,
    >,
    pub mtl_drawable_cb: Option::<unsafe extern "C" fn() -> *const libc::c_void>,
    pub mtl_global_uniform_buffer_size: libc::c_int,
    pub mtl_sampler_cache_size: libc::c_int,
    pub d3d11_device: *const libc::c_void,
    pub d3d11_device_context: *const libc::c_void,
    pub d3d11_render_target_view_cb: Option::<
        unsafe extern "C" fn() -> *const libc::c_void,
    >,
    pub d3d11_depth_stencil_view_cb: Option::<
        unsafe extern "C" fn() -> *const libc::c_void,
    >,
    pub _end_canary: uint32_t,
}
#[inline]
unsafe extern "C" fn cpfmin(mut a: cpFloat, mut b: cpFloat) -> cpFloat {
    return if a < b { a } else { b };
}
static mut cpvzero: cpVect = {
    let mut init = cpVect {
        x: 0.0f32 as cpFloat,
        y: 0.0f32 as cpFloat,
    };
    init
};
#[inline]
unsafe extern "C" fn cpv(x: cpFloat, y: cpFloat) -> cpVect {
    let mut v: cpVect = {
        let mut init = cpVect { x: x, y: y };
        init
    };
    return v;
}
#[inline]
unsafe extern "C" fn cpvadd(v1: cpVect, v2: cpVect) -> cpVect {
    return cpv(v1.x + v2.x, v1.y + v2.y);
}
#[inline]
unsafe extern "C" fn cpvsub(v1: cpVect, v2: cpVect) -> cpVect {
    return cpv(v1.x - v2.x, v1.y - v2.y);
}
#[inline]
unsafe extern "C" fn cpvmult(v: cpVect, s: cpFloat) -> cpVect {
    return cpv(v.x * s, v.y * s);
}
#[inline]
unsafe extern "C" fn cpvdot(v1: cpVect, v2: cpVect) -> cpFloat {
    return v1.x * v2.x + v1.y * v2.y;
}
#[inline]
unsafe extern "C" fn cpvlerp(v1: cpVect, v2: cpVect, t: cpFloat) -> cpVect {
    return cpvadd(cpvmult(v1, 1.0f32 as libc::c_double - t), cpvmult(v2, t));
}
#[inline]
unsafe extern "C" fn cpBBNew(l: cpFloat, b: cpFloat, r: cpFloat, t: cpFloat) -> cpBB {
    let mut bb: cpBB = {
        let mut init = cpBB { l: l, b: b, r: r, t: t };
        init
    };
    return bb;
}
#[inline]
unsafe extern "C" fn cpTransformNewTranspose(
    mut a: cpFloat,
    mut c: cpFloat,
    mut tx: cpFloat,
    mut b: cpFloat,
    mut d: cpFloat,
    mut ty: cpFloat,
) -> cpTransform {
    let mut t: cpTransform = {
        let mut init = cpTransform {
            a: a,
            b: b,
            c: c,
            d: d,
            tx: tx,
            ty: ty,
        };
        init
    };
    return t;
}
#[inline]
unsafe extern "C" fn cpTransformInverse(mut t: cpTransform) -> cpTransform {
    let mut inv_det: cpFloat = 1.0f64 / (t.a * t.d - t.c * t.b);
    return cpTransformNewTranspose(
        t.d * inv_det,
        -t.c * inv_det,
        (t.c * t.ty - t.tx * t.d) * inv_det,
        -t.b * inv_det,
        t.a * inv_det,
        (t.tx * t.b - t.a * t.ty) * inv_det,
    );
}
#[inline]
unsafe extern "C" fn cpTransformMult(
    mut t1: cpTransform,
    mut t2: cpTransform,
) -> cpTransform {
    return cpTransformNewTranspose(
        t1.a * t2.a + t1.c * t2.b,
        t1.a * t2.c + t1.c * t2.d,
        t1.a * t2.tx + t1.c * t2.ty + t1.tx,
        t1.b * t2.a + t1.d * t2.b,
        t1.b * t2.c + t1.d * t2.d,
        t1.b * t2.tx + t1.d * t2.ty + t1.ty,
    );
}
#[inline]
unsafe extern "C" fn cpTransformPoint(mut t: cpTransform, mut p: cpVect) -> cpVect {
    return cpv(t.a * p.x + t.c * p.y + t.tx, t.b * p.x + t.d * p.y + t.ty);
}
#[inline]
unsafe extern "C" fn cpTransformTranslate(mut translate: cpVect) -> cpTransform {
    return cpTransformNewTranspose(
        1.0f64,
        0.0f64,
        translate.x,
        0.0f64,
        1.0f64,
        translate.y,
    );
}
#[inline]
unsafe extern "C" fn cpTransformScale(
    mut scaleX: cpFloat,
    mut scaleY: cpFloat,
) -> cpTransform {
    return cpTransformNewTranspose(scaleX, 0.0f64, 0.0f64, 0.0f64, scaleY, 0.0f64);
}
#[inline]
unsafe extern "C" fn cpTransformOrtho(mut bb: cpBB) -> cpTransform {
    return cpTransformNewTranspose(
        2.0f64 / (bb.r - bb.l),
        0.0f64,
        -(bb.r + bb.l) / (bb.r - bb.l),
        0.0f64,
        2.0f64 / (bb.t - bb.b),
        -(bb.t + bb.b) / (bb.t - bb.b),
    );
}
#[inline]
unsafe extern "C" fn RGBAColor(
    mut r: libc::c_float,
    mut g: libc::c_float,
    mut b: libc::c_float,
    mut a: libc::c_float,
) -> cpSpaceDebugColor {
    let mut color: cpSpaceDebugColor = {
        let mut init = cpSpaceDebugColor {
            r: r,
            g: g,
            b: b,
            a: a,
        };
        init
    };
    return color;
}
#[inline]
unsafe extern "C" fn LAColor(
    mut l: libc::c_float,
    mut a: libc::c_float,
) -> cpSpaceDebugColor {
    let mut color: cpSpaceDebugColor = {
        let mut init = cpSpaceDebugColor {
            r: l,
            g: l,
            b: l,
            a: a,
        };
        init
    };
    return color;
}
static mut demos: [ChipmunkDemo; 32] = [ChipmunkDemo {
    name: 0 as *const libc::c_char,
    timestep: 0.,
    initFunc: None,
    updateFunc: None,
    drawFunc: None,
    destroyFunc: None,
}; 32];
static mut demo_count: libc::c_int = 0;
static mut demo_index: libc::c_int = 0;
static mut paused: cpBool = 0 as libc::c_int as cpBool;
static mut step: cpBool = 0 as libc::c_int as cpBool;
static mut space: *mut cpSpace = 0 as *const cpSpace as *mut cpSpace;
static mut Accumulator: libc::c_double = 0.;
static mut LastTime: libc::c_double = 0.;
pub static mut ChipmunkDemoTicks: libc::c_int = 0;
pub static mut ChipmunkDemoTime: libc::c_double = 0.;
pub static mut ChipmunkDemoMouse: cpVect = cpVect { x: 0., y: 0. };
pub static mut ChipmunkDemoRightClick: cpBool = 0;
pub static mut ChipmunkDemoRightDown: cpBool = 0;
pub static mut ChipmunkDemoKeyboard: cpVect = cpVect { x: 0., y: 0. };
static mut mouse_body: *mut cpBody = 0 as *const cpBody as *mut cpBody;
static mut mouse_joint: *mut cpConstraint = 0 as *const cpConstraint
    as *mut cpConstraint;
pub static mut ChipmunkDemoMessageString: *const libc::c_char = 0 as *const libc::c_char;
pub static mut GRAB_FILTER: cpShapeFilter = {
    let mut init = cpShapeFilter {
        group: 0 as libc::c_int as cpGroup,
        categories: ((1 as libc::c_int) << 31 as libc::c_int) as cpBitmask,
        mask: ((1 as libc::c_int) << 31 as libc::c_int) as cpBitmask,
    };
    init
};
pub static mut NOT_GRABBABLE_FILTER: cpShapeFilter = {
    let mut init = cpShapeFilter {
        group: 0 as libc::c_int as cpGroup,
        categories: !((1 as libc::c_int) << 31 as libc::c_int) as cpBitmask,
        mask: !((1 as libc::c_int) << 31 as libc::c_int) as cpBitmask,
    };
    init
};
pub static mut view_translate: cpVect = {
    let mut init = cpVect {
        x: 0 as libc::c_int as cpFloat,
        y: 0 as libc::c_int as cpFloat,
    };
    init
};
pub static mut view_scale: cpFloat = 1.0f64;
unsafe extern "C" fn ShapeFreeWrap(
    mut space_0: *mut cpSpace,
    mut shape: *mut cpShape,
    mut unused: *mut libc::c_void,
) {
    cpSpaceRemoveShape(space_0, shape);
    cpShapeFree(shape);
}
unsafe extern "C" fn PostShapeFree(mut shape: *mut cpShape, mut space_0: *mut cpSpace) {
    cpSpaceAddPostStepCallback(
        space_0,
        ::std::mem::transmute::<
            Option::<
                unsafe extern "C" fn(*mut cpSpace, *mut cpShape, *mut libc::c_void) -> (),
            >,
            cpPostStepFunc,
        >(
            Some(
                ShapeFreeWrap
                    as unsafe extern "C" fn(
                        *mut cpSpace,
                        *mut cpShape,
                        *mut libc::c_void,
                    ) -> (),
            ),
        ),
        shape as *mut libc::c_void,
        0 as *mut libc::c_void,
    );
}
unsafe extern "C" fn ConstraintFreeWrap(
    mut space_0: *mut cpSpace,
    mut constraint: *mut cpConstraint,
    mut unused: *mut libc::c_void,
) {
    cpSpaceRemoveConstraint(space_0, constraint);
    cpConstraintFree(constraint);
}
unsafe extern "C" fn PostConstraintFree(
    mut constraint: *mut cpConstraint,
    mut space_0: *mut cpSpace,
) {
    cpSpaceAddPostStepCallback(
        space_0,
        ::std::mem::transmute::<
            Option::<
                unsafe extern "C" fn(
                    *mut cpSpace,
                    *mut cpConstraint,
                    *mut libc::c_void,
                ) -> (),
            >,
            cpPostStepFunc,
        >(
            Some(
                ConstraintFreeWrap
                    as unsafe extern "C" fn(
                        *mut cpSpace,
                        *mut cpConstraint,
                        *mut libc::c_void,
                    ) -> (),
            ),
        ),
        constraint as *mut libc::c_void,
        0 as *mut libc::c_void,
    );
}
unsafe extern "C" fn BodyFreeWrap(
    mut space_0: *mut cpSpace,
    mut body: *mut cpBody,
    mut unused: *mut libc::c_void,
) {
    cpSpaceRemoveBody(space_0, body);
    cpBodyFree(body);
}
unsafe extern "C" fn PostBodyFree(mut body: *mut cpBody, mut space_0: *mut cpSpace) {
    cpSpaceAddPostStepCallback(
        space_0,
        ::std::mem::transmute::<
            Option::<
                unsafe extern "C" fn(*mut cpSpace, *mut cpBody, *mut libc::c_void) -> (),
            >,
            cpPostStepFunc,
        >(
            Some(
                BodyFreeWrap
                    as unsafe extern "C" fn(
                        *mut cpSpace,
                        *mut cpBody,
                        *mut libc::c_void,
                    ) -> (),
            ),
        ),
        body as *mut libc::c_void,
        0 as *mut libc::c_void,
    );
}
pub unsafe extern "C" fn ChipmunkDemoFreeSpaceChildren(mut space_0: *mut cpSpace) {
    cpSpaceEachShape(
        space_0,
        ::std::mem::transmute::<
            Option::<unsafe extern "C" fn(*mut cpShape, *mut cpSpace) -> ()>,
            cpSpaceShapeIteratorFunc,
        >(Some(PostShapeFree as unsafe extern "C" fn(*mut cpShape, *mut cpSpace) -> ())),
        space_0 as *mut libc::c_void,
    );
    cpSpaceEachConstraint(
        space_0,
        ::std::mem::transmute::<
            Option::<unsafe extern "C" fn(*mut cpConstraint, *mut cpSpace) -> ()>,
            cpSpaceConstraintIteratorFunc,
        >(
            Some(
                PostConstraintFree
                    as unsafe extern "C" fn(*mut cpConstraint, *mut cpSpace) -> (),
            ),
        ),
        space_0 as *mut libc::c_void,
    );
    cpSpaceEachBody(
        space_0,
        ::std::mem::transmute::<
            Option::<unsafe extern "C" fn(*mut cpBody, *mut cpSpace) -> ()>,
            cpSpaceBodyIteratorFunc,
        >(Some(PostBodyFree as unsafe extern "C" fn(*mut cpBody, *mut cpSpace) -> ())),
        space_0 as *mut libc::c_void,
    );
}
unsafe extern "C" fn DrawCircle(
    mut p: cpVect,
    mut a: cpFloat,
    mut r: cpFloat,
    mut outline: cpSpaceDebugColor,
    mut fill: cpSpaceDebugColor,
    mut data: cpDataPointer,
) {
    ChipmunkDebugDrawCircle(p, a, r, outline, fill);
}
unsafe extern "C" fn DrawSegment(
    mut a: cpVect,
    mut b: cpVect,
    mut color: cpSpaceDebugColor,
    mut data: cpDataPointer,
) {
    ChipmunkDebugDrawSegment(a, b, color);
}
unsafe extern "C" fn DrawFatSegment(
    mut a: cpVect,
    mut b: cpVect,
    mut r: cpFloat,
    mut outline: cpSpaceDebugColor,
    mut fill: cpSpaceDebugColor,
    mut data: cpDataPointer,
) {
    ChipmunkDebugDrawFatSegment(a, b, r, outline, fill);
}
unsafe extern "C" fn DrawPolygon(
    mut count: libc::c_int,
    mut verts: *const cpVect,
    mut r: cpFloat,
    mut outline: cpSpaceDebugColor,
    mut fill: cpSpaceDebugColor,
    mut data: cpDataPointer,
) {
    ChipmunkDebugDrawPolygon(count, verts, r, outline, fill);
}
unsafe extern "C" fn DrawDot(
    mut size: cpFloat,
    mut pos: cpVect,
    mut color: cpSpaceDebugColor,
    mut data: cpDataPointer,
) {
    ChipmunkDebugDrawDot(size, pos, color);
}
static mut Colors: [cpSpaceDebugColor; 8] = [
    {
        let mut init = cpSpaceDebugColor {
            r: 0xb5 as libc::c_int as libc::c_float / 255.0f32,
            g: 0x89 as libc::c_int as libc::c_float / 255.0f32,
            b: 0 as libc::c_int as libc::c_float / 255.0f32,
            a: 1.0f32,
        };
        init
    },
    {
        let mut init = cpSpaceDebugColor {
            r: 0xcb as libc::c_int as libc::c_float / 255.0f32,
            g: 0x4b as libc::c_int as libc::c_float / 255.0f32,
            b: 0x16 as libc::c_int as libc::c_float / 255.0f32,
            a: 1.0f32,
        };
        init
    },
    {
        let mut init = cpSpaceDebugColor {
            r: 0xdc as libc::c_int as libc::c_float / 255.0f32,
            g: 0x32 as libc::c_int as libc::c_float / 255.0f32,
            b: 0x2f as libc::c_int as libc::c_float / 255.0f32,
            a: 1.0f32,
        };
        init
    },
    {
        let mut init = cpSpaceDebugColor {
            r: 0xd3 as libc::c_int as libc::c_float / 255.0f32,
            g: 0x36 as libc::c_int as libc::c_float / 255.0f32,
            b: 0x82 as libc::c_int as libc::c_float / 255.0f32,
            a: 1.0f32,
        };
        init
    },
    {
        let mut init = cpSpaceDebugColor {
            r: 0x6c as libc::c_int as libc::c_float / 255.0f32,
            g: 0x71 as libc::c_int as libc::c_float / 255.0f32,
            b: 0xc4 as libc::c_int as libc::c_float / 255.0f32,
            a: 1.0f32,
        };
        init
    },
    {
        let mut init = cpSpaceDebugColor {
            r: 0x26 as libc::c_int as libc::c_float / 255.0f32,
            g: 0x8b as libc::c_int as libc::c_float / 255.0f32,
            b: 0xd2 as libc::c_int as libc::c_float / 255.0f32,
            a: 1.0f32,
        };
        init
    },
    {
        let mut init = cpSpaceDebugColor {
            r: 0x2a as libc::c_int as libc::c_float / 255.0f32,
            g: 0xa1 as libc::c_int as libc::c_float / 255.0f32,
            b: 0x98 as libc::c_int as libc::c_float / 255.0f32,
            a: 1.0f32,
        };
        init
    },
    {
        let mut init = cpSpaceDebugColor {
            r: 0x85 as libc::c_int as libc::c_float / 255.0f32,
            g: 0x99 as libc::c_int as libc::c_float / 255.0f32,
            b: 0 as libc::c_int as libc::c_float / 255.0f32,
            a: 1.0f32,
        };
        init
    },
];
unsafe extern "C" fn ColorForShape(
    mut shape: *mut cpShape,
    mut data: cpDataPointer,
) -> cpSpaceDebugColor {
    if cpShapeGetSensor(shape) != 0 {
        return LAColor(1.0f32, 0.1f32)
    } else {
        let mut body: *mut cpBody = cpShapeGetBody(shape);
        if cpBodyIsSleeping(body) != 0 {
            return RGBAColor(
                0x58 as libc::c_int as libc::c_float / 255.0f32,
                0x6e as libc::c_int as libc::c_float / 255.0f32,
                0x75 as libc::c_int as libc::c_float / 255.0f32,
                1.0f32,
            )
        } else if (*body).sleeping.idleTime > (*(*shape).space).sleepTimeThreshold {
            return RGBAColor(
                0x93 as libc::c_int as libc::c_float / 255.0f32,
                0xa1 as libc::c_int as libc::c_float / 255.0f32,
                0xa1 as libc::c_int as libc::c_float / 255.0f32,
                1.0f32,
            )
        } else {
            let mut val: uint32_t = (*shape).hashid as uint32_t;
            val = val
                .wrapping_add(0x7ed55d16 as libc::c_int as libc::c_uint)
                .wrapping_add(val << 12 as libc::c_int);
            val = val ^ 0xc761c23c as libc::c_uint ^ val >> 19 as libc::c_int;
            val = val
                .wrapping_add(0x165667b1 as libc::c_int as libc::c_uint)
                .wrapping_add(val << 5 as libc::c_int);
            val = val.wrapping_add(0xd3a2646c as libc::c_uint) ^ val << 9 as libc::c_int;
            val = val
                .wrapping_add(0xfd7046c5 as libc::c_uint)
                .wrapping_add(val << 3 as libc::c_int);
            val = val ^ 0xb55a4f09 as libc::c_uint ^ val >> 16 as libc::c_int;
            return Colors[(val & 0x7 as libc::c_int as libc::c_uint) as usize];
        }
    };
}
pub unsafe extern "C" fn ChipmunkDemoDefaultDrawImpl(mut space_0: *mut cpSpace) {
    let mut drawOptions: cpSpaceDebugDrawOptions = {
        let mut init = cpSpaceDebugDrawOptions {
            drawCircle: Some(
                DrawCircle
                    as unsafe extern "C" fn(
                        cpVect,
                        cpFloat,
                        cpFloat,
                        cpSpaceDebugColor,
                        cpSpaceDebugColor,
                        cpDataPointer,
                    ) -> (),
            ),
            drawSegment: Some(
                DrawSegment
                    as unsafe extern "C" fn(
                        cpVect,
                        cpVect,
                        cpSpaceDebugColor,
                        cpDataPointer,
                    ) -> (),
            ),
            drawFatSegment: Some(
                DrawFatSegment
                    as unsafe extern "C" fn(
                        cpVect,
                        cpVect,
                        cpFloat,
                        cpSpaceDebugColor,
                        cpSpaceDebugColor,
                        cpDataPointer,
                    ) -> (),
            ),
            drawPolygon: Some(
                DrawPolygon
                    as unsafe extern "C" fn(
                        libc::c_int,
                        *const cpVect,
                        cpFloat,
                        cpSpaceDebugColor,
                        cpSpaceDebugColor,
                        cpDataPointer,
                    ) -> (),
            ),
            drawDot: Some(
                DrawDot
                    as unsafe extern "C" fn(
                        cpFloat,
                        cpVect,
                        cpSpaceDebugColor,
                        cpDataPointer,
                    ) -> (),
            ),
            flags: (CP_SPACE_DEBUG_DRAW_SHAPES as libc::c_int
                | CP_SPACE_DEBUG_DRAW_CONSTRAINTS as libc::c_int
                | CP_SPACE_DEBUG_DRAW_COLLISION_POINTS as libc::c_int)
                as cpSpaceDebugDrawFlags,
            shapeOutlineColor: {
                let mut init = cpSpaceDebugColor {
                    r: 0xee as libc::c_int as libc::c_float / 255.0f32,
                    g: 0xe8 as libc::c_int as libc::c_float / 255.0f32,
                    b: 0xd5 as libc::c_int as libc::c_float / 255.0f32,
                    a: 1.0f32,
                };
                init
            },
            colorForShape: Some(
                ColorForShape
                    as unsafe extern "C" fn(
                        *mut cpShape,
                        cpDataPointer,
                    ) -> cpSpaceDebugColor,
            ),
            constraintColor: {
                let mut init = cpSpaceDebugColor {
                    r: 0.0f32,
                    g: 0.75f32,
                    b: 0.0f32,
                    a: 1.0f32,
                };
                init
            },
            collisionPointColor: {
                let mut init = cpSpaceDebugColor {
                    r: 1.0f32,
                    g: 0.0f32,
                    b: 0.0f32,
                    a: 1.0f32,
                };
                init
            },
            data: 0 as *mut libc::c_void,
        };
        init
    };
    cpSpaceDebugDraw(space_0, &mut drawOptions);
}
unsafe extern "C" fn DrawInstructions() {
    static mut title: [libc::c_char; 1024] = [0; 1024];
    sprintf(
        title.as_mut_ptr(),
        b"Demo(%c): %s\0" as *const u8 as *const libc::c_char,
        'A' as i32 + demo_index,
        demos[demo_index as usize].name,
    );
    ChipmunkDemoTextDrawString(
        cpv(-(300 as libc::c_int) as cpFloat, 220 as libc::c_int as cpFloat),
        title.as_mut_ptr(),
    );
    ChipmunkDemoTextDrawString(
        cpv(-(300 as libc::c_int) as cpFloat, 200 as libc::c_int as cpFloat),
        b"Controls:\nA - Z Switch demos. (return restarts)\nUse the mouse to grab objects.\n\0"
            as *const u8 as *const libc::c_char,
    );
}
static mut max_arbiters: libc::c_int = 0 as libc::c_int;
static mut max_points: libc::c_int = 0 as libc::c_int;
static mut max_constraints: libc::c_int = 0 as libc::c_int;
unsafe extern "C" fn DrawInfo() {
    let mut arbiters: libc::c_int = (*(*space).arbiters).num;
    let mut points: libc::c_int = 0 as libc::c_int;
    let mut i: libc::c_int = 0 as libc::c_int;
    while i < arbiters {
        points
            += (*(*((*(*space).arbiters).arr).offset(i as isize) as *mut cpArbiter))
                .count;
        i += 1;
        i;
    }
    let mut constraints: libc::c_int = ((*(*space).constraints).num + points)
        * (*space).iterations;
    max_arbiters = if arbiters > max_arbiters { arbiters } else { max_arbiters };
    max_points = if points > max_points { points } else { max_points };
    max_constraints = if constraints > max_constraints {
        constraints
    } else {
        max_constraints
    };
    let mut buffer: [libc::c_char; 1024] = [0; 1024];
    let mut format: *const libc::c_char = b"Arbiters: %d (%d) - Contact Points: %d (%d)\nOther Constraints: %d, Iterations: %d\nConstraints x Iterations: %d (%d)\nTime:% 5.2fs, KE:% 5.2e\0"
        as *const u8 as *const libc::c_char;
    let mut bodies: *mut cpArray = (*space).dynamicBodies;
    let mut ke: cpFloat = 0.0f32 as cpFloat;
    let mut i_0: libc::c_int = 0 as libc::c_int;
    while i_0 < (*bodies).num {
        let mut body: *mut cpBody = *((*bodies).arr).offset(i_0 as isize) as *mut cpBody;
        if !((*body).m == ::std::f32::INFINITY as libc::c_double
            || (*body).i == ::std::f32::INFINITY as libc::c_double)
        {
            ke
                += (*body).m * cpvdot((*body).v, (*body).v)
                    + (*body).i * (*body).w * (*body).w;
        }
        i_0 += 1;
        i_0;
    }
    sprintf(
        buffer.as_mut_ptr(),
        format,
        arbiters,
        max_arbiters,
        points,
        max_points,
        (*(*space).constraints).num,
        (*space).iterations,
        constraints,
        max_constraints,
        ChipmunkDemoTime,
        if ke < 1e-10f32 as libc::c_double { 0.0f32 as libc::c_double } else { ke },
    );
    ChipmunkDemoTextDrawString(
        cpv(0 as libc::c_int as cpFloat, 220 as libc::c_int as cpFloat),
        buffer.as_mut_ptr(),
    );
}
static mut PrintStringBuffer: [libc::c_char; 8192] = [0; 8192];
static mut PrintStringCursor: *mut libc::c_char = 0 as *const libc::c_char
    as *mut libc::c_char;
pub unsafe extern "C" fn ChipmunkDemoPrintString(
    mut fmt: *const libc::c_char,
    mut args: ...
) {
    if PrintStringCursor.is_null() {
        return;
    }
    ChipmunkDemoMessageString = PrintStringBuffer.as_mut_ptr();
    let mut args_0: ::std::ffi::VaListImpl;
    args_0 = args.clone();
    let mut remaining: libc::c_int = (::std::mem::size_of::<[libc::c_char; 8192]>()
        as libc::c_ulong)
        .wrapping_sub(
            PrintStringCursor.offset_from(PrintStringBuffer.as_mut_ptr()) as libc::c_long
                as libc::c_ulong,
        ) as libc::c_int;
    let mut would_write: libc::c_int = vsnprintf(
        PrintStringCursor,
        remaining as libc::c_ulong,
        fmt,
        args_0.as_va_list(),
    );
    if would_write > 0 as libc::c_int && would_write < remaining {
        PrintStringCursor = PrintStringCursor.offset(would_write as isize);
    } else {
        PrintStringCursor = 0 as *mut libc::c_char;
    };
}
unsafe extern "C" fn Tick(mut dt: libc::c_double) {
    if paused == 0 || step as libc::c_int != 0 {
        PrintStringBuffer[0 as libc::c_int as usize] = 0 as libc::c_int as libc::c_char;
        PrintStringCursor = PrintStringBuffer.as_mut_ptr();
        ChipmunkDebugDrawClearRenderer();
        ChipmunkDemoTextClearRenderer();
        let mut new_point: cpVect = cpvlerp(
            (*mouse_body).p,
            ChipmunkDemoMouse,
            0.25f32 as cpFloat,
        );
        (*mouse_body)
            .v = cpvmult(cpvsub(new_point, (*mouse_body).p), 60.0f32 as cpFloat);
        (*mouse_body).p = new_point;
        (demos[demo_index as usize].updateFunc).unwrap()(space, dt);
        ChipmunkDemoTicks += 1;
        ChipmunkDemoTicks;
        ChipmunkDemoTime += dt;
        step = 0 as libc::c_int as cpBool;
        ChipmunkDemoRightDown = 0 as libc::c_int as cpBool;
        ChipmunkDemoTextDrawString(
            cpv(-(300 as libc::c_int) as cpFloat, -(200 as libc::c_int) as cpFloat),
            ChipmunkDemoMessageString,
        );
    }
}
unsafe extern "C" fn Update() {
    let mut time: libc::c_double = stm_sec(stm_now());
    let mut dt: libc::c_double = time - LastTime;
    if dt > 0.2f64 {
        dt = 0.2f64;
    }
    let mut fixed_dt: libc::c_double = demos[demo_index as usize].timestep;
    Accumulator += dt;
    while Accumulator > fixed_dt {
        Tick(fixed_dt);
        Accumulator -= fixed_dt;
    }
    LastTime = time;
}
unsafe extern "C" fn Display() {
    let mut screen_size: cpVect = {
        let mut init = cpVect {
            x: sapp_width() as cpFloat,
            y: sapp_height() as cpFloat,
        };
        init
    };
    let mut view_matrix: cpTransform = cpTransformMult(
        cpTransformScale(view_scale, view_scale),
        cpTransformTranslate(view_translate),
    );
    let mut screen_scale: libc::c_float = cpfmin(
        screen_size.x / 640.0f64,
        screen_size.y / 480.0f64,
    ) as libc::c_float;
    let mut hw: libc::c_float = screen_size.x as libc::c_float * (0.5f32 / screen_scale);
    let mut hh: libc::c_float = screen_size.y as libc::c_float * (0.5f32 / screen_scale);
    let mut projection_matrix: cpTransform = cpTransformOrtho(
        cpBBNew(-hw as cpFloat, -hh as cpFloat, hw as cpFloat, hh as cpFloat),
    );
    ChipmunkDebugDrawPointLineScale = 1.0f32 / view_scale as libc::c_float;
    ChipmunkDebugDrawVPMatrix = cpTransformMult(projection_matrix, view_matrix);
    Update();
    ChipmunkDebugDrawPushRenderer();
    ChipmunkDemoTextPushRenderer();
    (demos[demo_index as usize].drawFunc).unwrap()(space);
    let mut action: sg_pass_action = {
        let mut init = sg_pass_action {
            _start_canary: 0,
            colors: [
                {
                    let mut init = sg_color_attachment_action {
                        action: SG_ACTION_CLEAR,
                        val: [
                            0x7 as libc::c_int as libc::c_float / 255.0f32,
                            0x36 as libc::c_int as libc::c_float / 255.0f32,
                            0x42 as libc::c_int as libc::c_float / 255.0f32,
                            0.,
                        ],
                    };
                    init
                },
                sg_color_attachment_action {
                    action: _SG_ACTION_DEFAULT,
                    val: [0.; 4],
                },
                sg_color_attachment_action {
                    action: _SG_ACTION_DEFAULT,
                    val: [0.; 4],
                },
                sg_color_attachment_action {
                    action: _SG_ACTION_DEFAULT,
                    val: [0.; 4],
                },
            ],
            depth: sg_depth_attachment_action {
                action: _SG_ACTION_DEFAULT,
                val: 0.,
            },
            stencil: sg_stencil_attachment_action {
                action: _SG_ACTION_DEFAULT,
                val: 0,
            },
            _end_canary: 0,
        };
        init
    };
    sg_begin_default_pass(
        &mut action,
        screen_size.x as libc::c_int,
        screen_size.y as libc::c_int,
    );
    ChipmunkDebugDrawFlushRenderer();
    DrawInstructions();
    DrawInfo();
    ChipmunkDemoTextMatrix = projection_matrix;
    ChipmunkDemoTextFlushRenderer();
    ChipmunkDebugDrawPopRenderer();
    ChipmunkDemoTextPopRenderer();
    sg_end_pass();
    sg_commit();
}
unsafe extern "C" fn RunDemo(mut index: libc::c_int) {
    srand(45073 as libc::c_int as libc::c_uint);
    demo_index = index;
    ChipmunkDemoTicks = 0 as libc::c_int;
    ChipmunkDemoTime = 0.0f64;
    Accumulator = 0.0f64;
    LastTime = stm_sec(stm_now());
    mouse_joint = 0 as *mut cpConstraint;
    ChipmunkDemoMessageString = b"\0" as *const u8 as *const libc::c_char;
    max_arbiters = 0 as libc::c_int;
    max_points = 0 as libc::c_int;
    max_constraints = 0 as libc::c_int;
    space = (demos[demo_index as usize].initFunc).unwrap()();
}
unsafe extern "C" fn Keyboard(mut event: *const sapp_event) {
    let mut translate_increment: libc::c_float = 50.0f32 / view_scale as libc::c_float;
    let mut scale_increment: libc::c_float = 1.2f32;
    if (*event).type_0 as libc::c_uint
        == SAPP_EVENTTYPE_CHAR as libc::c_int as libc::c_uint && !(*event).key_repeat
    {
        let mut index: libc::c_int = ((*event).char_code)
            .wrapping_sub('a' as i32 as libc::c_uint) as libc::c_int;
        if 0 as libc::c_int <= index && index < demo_count {
            (demos[demo_index as usize].destroyFunc).unwrap()(space);
            RunDemo(index);
        }
    } else if (*event).type_0 as libc::c_uint
        == SAPP_EVENTTYPE_KEY_DOWN as libc::c_int as libc::c_uint
    {
        match (*event).key_code as libc::c_uint {
            32 => {
                if !(*event).key_repeat {
                    (demos[demo_index as usize].destroyFunc).unwrap()(space);
                    RunDemo(demo_index);
                }
            }
            96 => {
                if !(*event).key_repeat {
                    paused = (paused == 0) as libc::c_int as cpBool;
                }
            }
            49 => {
                step = 1 as libc::c_int as cpBool;
            }
            324 => {
                view_translate.x += translate_increment as libc::c_double;
            }
            326 => {
                view_translate.x -= translate_increment as libc::c_double;
            }
            322 => {
                view_translate.y += translate_increment as libc::c_double;
            }
            328 => {
                view_translate.y -= translate_increment as libc::c_double;
            }
            327 => {
                view_scale /= scale_increment as libc::c_double;
            }
            329 => {
                view_scale *= scale_increment as libc::c_double;
            }
            325 => {
                view_translate.x = 0.0f32 as cpFloat;
                view_translate.y = 0.0f32 as cpFloat;
                view_scale = 1.0f32 as cpFloat;
            }
            _ => {}
        }
    }
    if !(*event).key_repeat {
        match (*event).key_code as libc::c_uint {
            265 => {
                ChipmunkDemoKeyboard.y
                    += if (*event).type_0 as libc::c_uint
                        == SAPP_EVENTTYPE_KEY_DOWN as libc::c_int as libc::c_uint
                    {
                        1.0f64
                    } else {
                        -1.0f64
                    };
            }
            264 => {
                ChipmunkDemoKeyboard.y
                    += if (*event).type_0 as libc::c_uint
                        == SAPP_EVENTTYPE_KEY_DOWN as libc::c_int as libc::c_uint
                    {
                        -1.0f64
                    } else {
                        1.0f64
                    };
            }
            263 => {
                ChipmunkDemoKeyboard.x
                    += if (*event).type_0 as libc::c_uint
                        == SAPP_EVENTTYPE_KEY_DOWN as libc::c_int as libc::c_uint
                    {
                        -1.0f64
                    } else {
                        1.0f64
                    };
            }
            262 => {
                ChipmunkDemoKeyboard.x
                    += if (*event).type_0 as libc::c_uint
                        == SAPP_EVENTTYPE_KEY_DOWN as libc::c_int as libc::c_uint
                    {
                        1.0f64
                    } else {
                        -1.0f64
                    };
            }
            _ => {}
        }
    }
}
unsafe extern "C" fn MouseToSpace(mut event: *const sapp_event) -> cpVect {
    let mut screen_size: cpVect = cpv(sapp_width() as cpFloat, sapp_height() as cpFloat);
    let mut clip_coord: cpVect = cpv(
        (2 as libc::c_int as libc::c_float * (*event).mouse_x) as libc::c_double
            / screen_size.x - 1 as libc::c_int as libc::c_double,
        1 as libc::c_int as libc::c_double
            - (2 as libc::c_int as libc::c_float * (*event).mouse_y) as libc::c_double
                / screen_size.y,
    );
    let mut vp_inverse: cpTransform = cpTransformInverse(ChipmunkDebugDrawVPMatrix);
    return cpTransformPoint(vp_inverse, clip_coord);
}
unsafe extern "C" fn Click(mut event: *const sapp_event) {
    let mut mouse_pos: cpVect = MouseToSpace(event);
    if (*event).mouse_button as libc::c_int == SAPP_MOUSEBUTTON_LEFT as libc::c_int {
        if (*event).type_0 as libc::c_uint
            == SAPP_EVENTTYPE_MOUSE_DOWN as libc::c_int as libc::c_uint
        {
            let mut radius: cpFloat = 5.0f64;
            let mut info: cpPointQueryInfo = {
                let mut init = cpPointQueryInfo {
                    shape: 0 as *const cpShape,
                    point: cpVect { x: 0., y: 0. },
                    distance: 0.,
                    gradient: cpVect { x: 0., y: 0. },
                };
                init
            };
            let mut shape: *mut cpShape = cpSpacePointQueryNearest(
                space,
                mouse_pos,
                radius,
                GRAB_FILTER,
                &mut info,
            );
            if !shape.is_null()
                && cpBodyGetMass(cpShapeGetBody(shape))
                    < ::std::f32::INFINITY as libc::c_double
            {
                let mut nearest: cpVect = if info.distance > 0.0f32 as libc::c_double {
                    info.point
                } else {
                    mouse_pos
                };
                let mut body: *mut cpBody = cpShapeGetBody(shape);
                mouse_joint = cpPivotJointNew2(
                    mouse_body,
                    body,
                    cpvzero,
                    cpBodyWorldToLocal(body, nearest),
                );
                (*mouse_joint).maxForce = 50000.0f32 as cpFloat;
                (*mouse_joint)
                    .errorBias = pow(
                    (1.0f32 - 0.15f32) as libc::c_double,
                    60.0f32 as libc::c_double,
                );
                cpSpaceAddConstraint(space, mouse_joint);
            }
        } else if !mouse_joint.is_null() {
            cpSpaceRemoveConstraint(space, mouse_joint);
            cpConstraintFree(mouse_joint);
            mouse_joint = 0 as *mut cpConstraint;
        }
    } else if (*event).mouse_button as libc::c_int
        == SAPP_MOUSEBUTTON_RIGHT as libc::c_int
    {
        ChipmunkDemoRightClick = ((*event).type_0 as libc::c_uint
            == SAPP_EVENTTYPE_MOUSE_DOWN as libc::c_int as libc::c_uint) as libc::c_int
            as cpBool;
        ChipmunkDemoRightDown = ChipmunkDemoRightClick;
    }
}
unsafe extern "C" fn Event(mut event: *const sapp_event) {
    match (*event).type_0 as libc::c_uint {
        3 | 2 | 1 => {
            Keyboard(event);
        }
        7 => {
            ChipmunkDemoMouse = MouseToSpace(event);
        }
        5 | 4 => {
            Click(event);
        }
        _ => {}
    };
}
unsafe extern "C" fn TimeTrial(mut index: libc::c_int, mut count: libc::c_int) {
    space = (demos[index as usize].initFunc).unwrap()();
    let mut start_time: libc::c_double = stm_sec(stm_now());
    let mut dt: libc::c_double = demos[index as usize].timestep;
    let mut i: libc::c_int = 0 as libc::c_int;
    while i < count {
        (demos[index as usize].updateFunc).unwrap()(space, dt);
        i += 1;
        i;
    }
    let mut end_time: libc::c_double = stm_sec(stm_now());
    (demos[index as usize].destroyFunc).unwrap()(space);
    printf(
        b"Time(%c) = %8.2f ms (%s)\n\0" as *const u8 as *const libc::c_char,
        index + 'a' as i32,
        (end_time - start_time) * 1e3f32 as libc::c_double,
        demos[index as usize].name,
    );
    fflush(stdout);
}
unsafe extern "C" fn Init() {
    let mut desc: sg_desc = {
        let mut init = sg_desc {
            _start_canary: 0 as libc::c_int as uint32_t,
            buffer_pool_size: 0,
            image_pool_size: 0,
            shader_pool_size: 0,
            pipeline_pool_size: 0,
            pass_pool_size: 0,
            context_pool_size: 0,
            gl_force_gles2: false,
            mtl_device: 0 as *const libc::c_void,
            mtl_renderpass_descriptor_cb: None,
            mtl_drawable_cb: None,
            mtl_global_uniform_buffer_size: 0,
            mtl_sampler_cache_size: 0,
            d3d11_device: 0 as *const libc::c_void,
            d3d11_device_context: 0 as *const libc::c_void,
            d3d11_render_target_view_cb: None,
            d3d11_depth_stencil_view_cb: None,
            _end_canary: 0,
        };
        init
    };
    sg_setup(&mut desc);
    if !sg_isvalid() {
        cpMessage(
            b"sg_isvalid()\0" as *const u8 as *const libc::c_char,
            b"../../demo/ChipmunkDemo.c\0" as *const u8 as *const libc::c_char,
            574 as libc::c_int,
            1 as libc::c_int,
            1 as libc::c_int,
            b"Could not init Sokol GFX.\0" as *const u8 as *const libc::c_char,
        );
        abort();
    }
    ChipmunkDebugDrawInit();
    ChipmunkDemoTextInit();
    mouse_body = cpBodyNewKinematic();
    RunDemo(demo_index);
}
pub unsafe extern "C" fn Cleanup() {
    sg_shutdown();
}
pub unsafe extern "C" fn sokol_main(
    mut argc: libc::c_int,
    mut argv: *mut *mut libc::c_char,
) -> sapp_desc {
    demos[0 as libc::c_int as usize] = LogoSmash;
    demos[1 as libc::c_int as usize] = PyramidStack;
    demos[2 as libc::c_int as usize] = Plink;
    demos[3 as libc::c_int as usize] = BouncyHexagons;
    demos[4 as libc::c_int as usize] = Tumble;
    demos[5 as libc::c_int as usize] = PyramidTopple;
    demos[6 as libc::c_int as usize] = Planet;
    demos[7 as libc::c_int as usize] = Springies;
    demos[8 as libc::c_int as usize] = Pump;
    demos[9 as libc::c_int as usize] = TheoJansen;
    demos[10 as libc::c_int as usize] = Query;
    demos[11 as libc::c_int as usize] = OneWay;
    demos[12 as libc::c_int as usize] = Joints;
    demos[13 as libc::c_int as usize] = Tank;
    demos[14 as libc::c_int as usize] = Chains;
    demos[15 as libc::c_int as usize] = Crane;
    demos[16 as libc::c_int as usize] = ContactGraph;
    demos[17 as libc::c_int as usize] = Buoyancy;
    demos[18 as libc::c_int as usize] = Player;
    demos[19 as libc::c_int as usize] = Slice;
    demos[20 as libc::c_int as usize] = Convex;
    demos[21 as libc::c_int as usize] = Unicycle;
    demos[22 as libc::c_int as usize] = Sticky;
    demos[23 as libc::c_int as usize] = Shatter;
    demo_count = 24 as libc::c_int;
    let mut trial: libc::c_int = 0 as libc::c_int;
    let mut i: libc::c_int = 0 as libc::c_int;
    while i < argc {
        if strcmp(
            *argv.offset(i as isize),
            b"-bench\0" as *const u8 as *const libc::c_char,
        ) == 0 as libc::c_int
        {
            memcpy(
                demos.as_mut_ptr() as *mut libc::c_void,
                bench_list.as_mut_ptr() as *const libc::c_void,
                (bench_count as libc::c_ulong)
                    .wrapping_mul(::std::mem::size_of::<ChipmunkDemo>() as libc::c_ulong),
            );
            demo_count = bench_count;
        } else if strcmp(
            *argv.offset(i as isize),
            b"-trial\0" as *const u8 as *const libc::c_char,
        ) == 0 as libc::c_int
        {
            trial = 1 as libc::c_int;
        }
        i += 1;
        i;
    }
    stm_setup();
    if trial != 0 {
        let mut i_0: libc::c_int = 0 as libc::c_int;
        while i_0 < demo_count {
            TimeTrial(i_0, 1000 as libc::c_int);
            i_0 += 1;
            i_0;
        }
        exit(0 as libc::c_int);
    } else {
        return {
            let mut init = sapp_desc {
                init_cb: Some(Init as unsafe extern "C" fn() -> ()),
                frame_cb: Some(Display as unsafe extern "C" fn() -> ()),
                cleanup_cb: Some(Cleanup as unsafe extern "C" fn() -> ()),
                event_cb: Some(Event as unsafe extern "C" fn(*const sapp_event) -> ()),
                fail_cb: None,
                user_data: 0 as *mut libc::c_void,
                init_userdata_cb: None,
                frame_userdata_cb: None,
                cleanup_userdata_cb: None,
                event_userdata_cb: None,
                fail_userdata_cb: None,
                width: 1024 as libc::c_int,
                height: 768 as libc::c_int,
                sample_count: 0,
                swap_interval: 0,
                high_dpi: 1 as libc::c_int != 0,
                fullscreen: false,
                alpha: false,
                window_title: b"Chipmunk2D\0" as *const u8 as *const libc::c_char,
                user_cursor: false,
                html5_canvas_name: 0 as *const libc::c_char,
                html5_canvas_resize: false,
                html5_preserve_drawing_buffer: false,
                html5_premultiplied_alpha: false,
                ios_keyboard_resizes_canvas: false,
                gl_force_gles2: false,
            };
            init
        }
    };
}
