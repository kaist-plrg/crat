use ::libc;
extern "C" {
    pub type cpHashSet;
    pub type cpContactBufferHeader;
    fn memcpy(
        _: *mut libc::c_void,
        _: *const libc::c_void,
        _: libc::c_ulong,
    ) -> *mut libc::c_void;
    fn pow(_: libc::c_double, _: libc::c_double) -> libc::c_double;
    fn abort() -> !;
    fn free(__ptr: *mut libc::c_void);
    fn calloc(_: libc::c_ulong, _: libc::c_ulong) -> *mut libc::c_void;
    fn cpMessage(
        condition: *const libc::c_char,
        file: *const libc::c_char,
        line: libc::c_int,
        isError: libc::c_int,
        isHardError: libc::c_int,
        message: *const libc::c_char,
        _: ...
    );
    fn cpSpaceHashNew(
        celldim: cpFloat,
        cells: libc::c_int,
        bbfunc: cpSpatialIndexBBFunc,
        staticIndex: *mut cpSpatialIndex,
    ) -> *mut cpSpatialIndex;
    fn cpBBTreeNew(
        bbfunc: cpSpatialIndexBBFunc,
        staticIndex: *mut cpSpatialIndex,
    ) -> *mut cpSpatialIndex;
    fn cpBBTreeSetVelocityFunc(index: *mut cpSpatialIndex, func: cpBBTreeVelocityFunc);
    fn cpSpatialIndexFree(index: *mut cpSpatialIndex);
    fn cpArbiterCallWildcardBeginA(arb: *mut cpArbiter, space: *mut cpSpace) -> cpBool;
    fn cpArbiterCallWildcardBeginB(arb: *mut cpArbiter, space: *mut cpSpace) -> cpBool;
    fn cpArbiterCallWildcardPreSolveA(
        arb: *mut cpArbiter,
        space: *mut cpSpace,
    ) -> cpBool;
    fn cpArbiterCallWildcardPreSolveB(
        arb: *mut cpArbiter,
        space: *mut cpSpace,
    ) -> cpBool;
    fn cpArbiterCallWildcardPostSolveA(arb: *mut cpArbiter, space: *mut cpSpace);
    fn cpArbiterCallWildcardPostSolveB(arb: *mut cpArbiter, space: *mut cpSpace);
    fn cpArbiterCallWildcardSeparateA(arb: *mut cpArbiter, space: *mut cpSpace);
    fn cpArbiterCallWildcardSeparateB(arb: *mut cpArbiter, space: *mut cpSpace);
    fn cpBodyInit(body: *mut cpBody, mass: cpFloat, moment: cpFloat) -> *mut cpBody;
    fn cpBodyActivate(body: *mut cpBody);
    fn cpBodyActivateStatic(body: *mut cpBody, filter: *mut cpShape);
    fn cpBodyGetType(body: *mut cpBody) -> cpBodyType;
    fn cpBodySetType(body: *mut cpBody, type_0: cpBodyType);
    fn cpShapeCacheBB(shape: *mut cpShape) -> cpBB;
    fn cpShapeUpdate(shape: *mut cpShape, transform: cpTransform) -> cpBB;
    fn cpShapeGetBB(shape: *const cpShape) -> cpBB;
    fn cpArrayNew(size: libc::c_int) -> *mut cpArray;
    fn cpHashSetNew(size: libc::c_int, eqlFunc: cpHashSetEqlFunc) -> *mut cpHashSet;
    fn cpHashSetFree(set: *mut cpHashSet);
    fn cpHashSetEach(
        set: *mut cpHashSet,
        func: cpHashSetIteratorFunc,
        data: *mut libc::c_void,
    );
    fn cpArrayFree(arr: *mut cpArray);
    fn cpArrayFreeEach(
        arr: *mut cpArray,
        freeFunc: Option::<unsafe extern "C" fn(*mut libc::c_void) -> ()>,
    );
    fn cpSpaceUnlock(space: *mut cpSpace, runPostStep: cpBool);
    fn cpSpaceLock(space: *mut cpSpace);
    fn cpHashSetInsert(
        set: *mut cpHashSet,
        hash: cpHashValue,
        ptr: *const libc::c_void,
        trans: cpHashSetTransFunc,
        data: *mut libc::c_void,
    ) -> *const libc::c_void;
    fn cpBodyAddShape(body: *mut cpBody, shape: *mut cpShape);
    fn cpArrayPush(arr: *mut cpArray, object: *mut libc::c_void);
    fn cpArrayDeleteObj(arr: *mut cpArray, obj: *mut libc::c_void);
    fn cpArbiterUnthread(arb: *mut cpArbiter);
    fn cpHashSetFilter(
        set: *mut cpHashSet,
        func: cpHashSetFilterFunc,
        data: *mut libc::c_void,
    );
    fn cpBodyRemoveShape(body: *mut cpBody, shape: *mut cpShape);
    fn cpBodyRemoveConstraint(body: *mut cpBody, constraint: *mut cpConstraint);
    fn cpShapeUpdateFunc(shape: *mut cpShape, unused: *mut libc::c_void);
}
pub type __uint32_t = libc::c_uint;
pub type uint32_t = __uint32_t;
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
pub type cpBBTreeVelocityFunc = Option::<
    unsafe extern "C" fn(*mut libc::c_void) -> cpVect,
>;
pub type cpBodyType = libc::c_uint;
pub const CP_BODY_TYPE_STATIC: cpBodyType = 2;
pub const CP_BODY_TYPE_KINEMATIC: cpBodyType = 1;
pub const CP_BODY_TYPE_DYNAMIC: cpBodyType = 0;
pub type cpHashSetEqlFunc = Option::<
    unsafe extern "C" fn(*const libc::c_void, *const libc::c_void) -> cpBool,
>;
pub type cpHashSetIteratorFunc = Option::<
    unsafe extern "C" fn(*mut libc::c_void, *mut libc::c_void) -> (),
>;
pub type cpSpaceBodyIteratorFunc = Option::<
    unsafe extern "C" fn(*mut cpBody, *mut libc::c_void) -> (),
>;
pub type cpHashSetTransFunc = Option::<
    unsafe extern "C" fn(*const libc::c_void, *mut libc::c_void) -> *mut libc::c_void,
>;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct arbiterFilterContext {
    pub space: *mut cpSpace,
    pub body: *mut cpBody,
    pub shape: *mut cpShape,
}
pub type cpHashSetFilterFunc = Option::<
    unsafe extern "C" fn(*mut libc::c_void, *mut libc::c_void) -> cpBool,
>;
pub type cpSpaceShapeIteratorFunc = Option::<
    unsafe extern "C" fn(*mut cpShape, *mut libc::c_void) -> (),
>;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct spaceShapeContext {
    pub func: cpSpaceShapeIteratorFunc,
    pub data: *mut libc::c_void,
}
pub type cpSpaceConstraintIteratorFunc = Option::<
    unsafe extern "C" fn(*mut cpConstraint, *mut libc::c_void) -> (),
>;
#[inline]
unsafe extern "C" fn cpSpatialIndexRemove(
    mut index: *mut cpSpatialIndex,
    mut obj: *mut libc::c_void,
    mut hashid: cpHashValue,
) {
    ((*(*index).klass).remove).unwrap()(index, obj, hashid);
}
static mut cpvzero: cpVect = {
    let mut init = cpVect {
        x: 0.0f32 as cpFloat,
        y: 0.0f32 as cpFloat,
    };
    init
};
#[inline]
unsafe extern "C" fn cpSpatialIndexEach(
    mut index: *mut cpSpatialIndex,
    mut func: cpSpatialIndexIteratorFunc,
    mut data: *mut libc::c_void,
) {
    ((*(*index).klass).each).unwrap()(index, func, data);
}
#[inline]
unsafe extern "C" fn cpSpatialIndexInsert(
    mut index: *mut cpSpatialIndex,
    mut obj: *mut libc::c_void,
    mut hashid: cpHashValue,
) {
    ((*(*index).klass).insert).unwrap()(index, obj, hashid);
}
#[inline]
unsafe extern "C" fn cpSpaceArrayForBodyType(
    mut space: *mut cpSpace,
    mut type_0: cpBodyType,
) -> *mut cpArray {
    return if type_0 as libc::c_uint
        == CP_BODY_TYPE_STATIC as libc::c_int as libc::c_uint
    {
        (*space).staticBodies
    } else {
        (*space).dynamicBodies
    };
}
#[inline]
unsafe extern "C" fn cpSpatialIndexReindex(mut index: *mut cpSpatialIndex) {
    ((*(*index).klass).reindex).unwrap()(index);
}
#[inline]
unsafe extern "C" fn cpSpatialIndexReindexObject(
    mut index: *mut cpSpatialIndex,
    mut obj: *mut libc::c_void,
    mut hashid: cpHashValue,
) {
    ((*(*index).klass).reindexObject).unwrap()(index, obj, hashid);
}
unsafe extern "C" fn arbiterSetEql(
    mut shapes: *mut *mut cpShape,
    mut arb: *mut cpArbiter,
) -> cpBool {
    let mut a: *mut cpShape = *shapes.offset(0 as libc::c_int as isize);
    let mut b: *mut cpShape = *shapes.offset(1 as libc::c_int as isize);
    return (a == (*arb).a as *mut cpShape && b == (*arb).b as *mut cpShape
        || b == (*arb).a as *mut cpShape && a == (*arb).b as *mut cpShape) as libc::c_int
        as cpBool;
}
unsafe extern "C" fn handlerSetEql(
    mut check: *mut cpCollisionHandler,
    mut pair: *mut cpCollisionHandler,
) -> cpBool {
    return ((*check).typeA == (*pair).typeA && (*check).typeB == (*pair).typeB
        || (*check).typeB == (*pair).typeA && (*check).typeA == (*pair).typeB)
        as libc::c_int as cpBool;
}
unsafe extern "C" fn handlerSetTrans(
    mut handler: *mut cpCollisionHandler,
    mut unused: *mut libc::c_void,
) -> *mut libc::c_void {
    let mut copy: *mut cpCollisionHandler = calloc(
        1 as libc::c_int as libc::c_ulong,
        ::std::mem::size_of::<cpCollisionHandler>() as libc::c_ulong,
    ) as *mut cpCollisionHandler;
    memcpy(
        copy as *mut libc::c_void,
        handler as *const libc::c_void,
        ::std::mem::size_of::<cpCollisionHandler>() as libc::c_ulong,
    );
    return copy as *mut libc::c_void;
}
unsafe extern "C" fn DefaultBegin(
    mut arb: *mut cpArbiter,
    mut space: *mut cpSpace,
    mut data: cpDataPointer,
) -> cpBool {
    let mut retA: cpBool = cpArbiterCallWildcardBeginA(arb, space);
    let mut retB: cpBool = cpArbiterCallWildcardBeginB(arb, space);
    return (retA as libc::c_int != 0 && retB as libc::c_int != 0) as libc::c_int
        as cpBool;
}
unsafe extern "C" fn DefaultPreSolve(
    mut arb: *mut cpArbiter,
    mut space: *mut cpSpace,
    mut data: cpDataPointer,
) -> cpBool {
    let mut retA: cpBool = cpArbiterCallWildcardPreSolveA(arb, space);
    let mut retB: cpBool = cpArbiterCallWildcardPreSolveB(arb, space);
    return (retA as libc::c_int != 0 && retB as libc::c_int != 0) as libc::c_int
        as cpBool;
}
unsafe extern "C" fn DefaultPostSolve(
    mut arb: *mut cpArbiter,
    mut space: *mut cpSpace,
    mut data: cpDataPointer,
) {
    cpArbiterCallWildcardPostSolveA(arb, space);
    cpArbiterCallWildcardPostSolveB(arb, space);
}
unsafe extern "C" fn DefaultSeparate(
    mut arb: *mut cpArbiter,
    mut space: *mut cpSpace,
    mut data: cpDataPointer,
) {
    cpArbiterCallWildcardSeparateA(arb, space);
    cpArbiterCallWildcardSeparateB(arb, space);
}
static mut cpCollisionHandlerDefault: cpCollisionHandler = unsafe {
    {
        let mut init = cpCollisionHandler {
            typeA: !(0 as libc::c_int as cpCollisionType),
            typeB: !(0 as libc::c_int as cpCollisionType),
            beginFunc: Some(
                DefaultBegin
                    as unsafe extern "C" fn(
                        *mut cpArbiter,
                        *mut cpSpace,
                        cpDataPointer,
                    ) -> cpBool,
            ),
            preSolveFunc: Some(
                DefaultPreSolve
                    as unsafe extern "C" fn(
                        *mut cpArbiter,
                        *mut cpSpace,
                        cpDataPointer,
                    ) -> cpBool,
            ),
            postSolveFunc: Some(
                DefaultPostSolve
                    as unsafe extern "C" fn(
                        *mut cpArbiter,
                        *mut cpSpace,
                        cpDataPointer,
                    ) -> (),
            ),
            separateFunc: Some(
                DefaultSeparate
                    as unsafe extern "C" fn(
                        *mut cpArbiter,
                        *mut cpSpace,
                        cpDataPointer,
                    ) -> (),
            ),
            userData: 0 as *const libc::c_void as *mut libc::c_void,
        };
        init
    }
};
unsafe extern "C" fn AlwaysCollide(
    mut arb: *mut cpArbiter,
    mut space: *mut cpSpace,
    mut data: cpDataPointer,
) -> cpBool {
    return 1 as libc::c_int as cpBool;
}
unsafe extern "C" fn DoNothing(
    mut arb: *mut cpArbiter,
    mut space: *mut cpSpace,
    mut data: cpDataPointer,
) {}
pub static mut cpCollisionHandlerDoNothing: cpCollisionHandler = unsafe {
    {
        let mut init = cpCollisionHandler {
            typeA: !(0 as libc::c_int as cpCollisionType),
            typeB: !(0 as libc::c_int as cpCollisionType),
            beginFunc: Some(
                AlwaysCollide
                    as unsafe extern "C" fn(
                        *mut cpArbiter,
                        *mut cpSpace,
                        cpDataPointer,
                    ) -> cpBool,
            ),
            preSolveFunc: Some(
                AlwaysCollide
                    as unsafe extern "C" fn(
                        *mut cpArbiter,
                        *mut cpSpace,
                        cpDataPointer,
                    ) -> cpBool,
            ),
            postSolveFunc: Some(
                DoNothing
                    as unsafe extern "C" fn(
                        *mut cpArbiter,
                        *mut cpSpace,
                        cpDataPointer,
                    ) -> (),
            ),
            separateFunc: Some(
                DoNothing
                    as unsafe extern "C" fn(
                        *mut cpArbiter,
                        *mut cpSpace,
                        cpDataPointer,
                    ) -> (),
            ),
            userData: 0 as *const libc::c_void as *mut libc::c_void,
        };
        init
    }
};
unsafe extern "C" fn ShapeVelocityFunc(mut shape: *mut cpShape) -> cpVect {
    return (*(*shape).body).v;
}
unsafe extern "C" fn FreeWrap(
    mut ptr: *mut libc::c_void,
    mut unused: *mut libc::c_void,
) {
    free(ptr);
}
pub unsafe extern "C" fn cpSpaceAlloc() -> *mut cpSpace {
    return calloc(
        1 as libc::c_int as libc::c_ulong,
        ::std::mem::size_of::<cpSpace>() as libc::c_ulong,
    ) as *mut cpSpace;
}
pub unsafe extern "C" fn cpSpaceInit(mut space: *mut cpSpace) -> *mut cpSpace {
    (*space).iterations = 10 as libc::c_int;
    (*space).gravity = cpvzero;
    (*space).damping = 1.0f32 as cpFloat;
    (*space).collisionSlop = 0.1f32 as cpFloat;
    (*space)
        .collisionBias = pow(
        (1.0f32 - 0.1f32) as libc::c_double,
        60.0f32 as libc::c_double,
    );
    (*space).collisionPersistence = 3 as libc::c_int as cpTimestamp;
    (*space).locked = 0 as libc::c_int;
    (*space).stamp = 0 as libc::c_int as cpTimestamp;
    (*space).shapeIDCounter = 0 as libc::c_int as cpHashValue;
    (*space)
        .staticShapes = cpBBTreeNew(
        ::std::mem::transmute::<
            Option::<unsafe extern "C" fn(*const cpShape) -> cpBB>,
            cpSpatialIndexBBFunc,
        >(Some(cpShapeGetBB as unsafe extern "C" fn(*const cpShape) -> cpBB)),
        0 as *mut cpSpatialIndex,
    );
    (*space)
        .dynamicShapes = cpBBTreeNew(
        ::std::mem::transmute::<
            Option::<unsafe extern "C" fn(*const cpShape) -> cpBB>,
            cpSpatialIndexBBFunc,
        >(Some(cpShapeGetBB as unsafe extern "C" fn(*const cpShape) -> cpBB)),
        (*space).staticShapes,
    );
    cpBBTreeSetVelocityFunc(
        (*space).dynamicShapes,
        ::std::mem::transmute::<
            Option::<unsafe extern "C" fn(*mut cpShape) -> cpVect>,
            cpBBTreeVelocityFunc,
        >(Some(ShapeVelocityFunc as unsafe extern "C" fn(*mut cpShape) -> cpVect)),
    );
    (*space).allocatedBuffers = cpArrayNew(0 as libc::c_int);
    (*space).dynamicBodies = cpArrayNew(0 as libc::c_int);
    (*space).staticBodies = cpArrayNew(0 as libc::c_int);
    (*space).sleepingComponents = cpArrayNew(0 as libc::c_int);
    (*space).rousedBodies = cpArrayNew(0 as libc::c_int);
    (*space).sleepTimeThreshold = ::std::f32::INFINITY as cpFloat;
    (*space).idleSpeedThreshold = 0.0f32 as cpFloat;
    (*space).arbiters = cpArrayNew(0 as libc::c_int);
    (*space).pooledArbiters = cpArrayNew(0 as libc::c_int);
    (*space).contactBuffersHead = 0 as *mut cpContactBufferHeader;
    (*space)
        .cachedArbiters = cpHashSetNew(
        0 as libc::c_int,
        ::std::mem::transmute::<
            Option::<unsafe extern "C" fn(*mut *mut cpShape, *mut cpArbiter) -> cpBool>,
            cpHashSetEqlFunc,
        >(
            Some(
                arbiterSetEql
                    as unsafe extern "C" fn(*mut *mut cpShape, *mut cpArbiter) -> cpBool,
            ),
        ),
    );
    (*space).constraints = cpArrayNew(0 as libc::c_int);
    (*space).usesWildcards = 0 as libc::c_int as cpBool;
    memcpy(
        &mut (*space).defaultHandler as *mut cpCollisionHandler as *mut libc::c_void,
        &mut cpCollisionHandlerDoNothing as *mut cpCollisionHandler
            as *const libc::c_void,
        ::std::mem::size_of::<cpCollisionHandler>() as libc::c_ulong,
    );
    (*space)
        .collisionHandlers = cpHashSetNew(
        0 as libc::c_int,
        ::std::mem::transmute::<
            Option::<
                unsafe extern "C" fn(
                    *mut cpCollisionHandler,
                    *mut cpCollisionHandler,
                ) -> cpBool,
            >,
            cpHashSetEqlFunc,
        >(
            Some(
                handlerSetEql
                    as unsafe extern "C" fn(
                        *mut cpCollisionHandler,
                        *mut cpCollisionHandler,
                    ) -> cpBool,
            ),
        ),
    );
    (*space).postStepCallbacks = cpArrayNew(0 as libc::c_int);
    (*space).skipPostStep = 0 as libc::c_int as cpBool;
    let mut staticBody: *mut cpBody = cpBodyInit(
        &mut (*space)._staticBody,
        0.0f32 as cpFloat,
        0.0f32 as cpFloat,
    );
    cpBodySetType(staticBody, CP_BODY_TYPE_STATIC);
    cpSpaceSetStaticBody(space, staticBody);
    return space;
}
pub unsafe extern "C" fn cpSpaceNew() -> *mut cpSpace {
    return cpSpaceInit(cpSpaceAlloc());
}
unsafe extern "C" fn cpBodyActivateWrap(
    mut body: *mut cpBody,
    mut unused: *mut libc::c_void,
) {
    cpBodyActivate(body);
}
pub unsafe extern "C" fn cpSpaceDestroy(mut space: *mut cpSpace) {
    cpSpaceEachBody(
        space,
        ::std::mem::transmute::<
            Option::<unsafe extern "C" fn(*mut cpBody, *mut libc::c_void) -> ()>,
            cpSpaceBodyIteratorFunc,
        >(
            Some(
                cpBodyActivateWrap
                    as unsafe extern "C" fn(*mut cpBody, *mut libc::c_void) -> (),
            ),
        ),
        0 as *mut libc::c_void,
    );
    cpSpatialIndexFree((*space).staticShapes);
    cpSpatialIndexFree((*space).dynamicShapes);
    cpArrayFree((*space).dynamicBodies);
    cpArrayFree((*space).staticBodies);
    cpArrayFree((*space).sleepingComponents);
    cpArrayFree((*space).rousedBodies);
    cpArrayFree((*space).constraints);
    cpHashSetFree((*space).cachedArbiters);
    cpArrayFree((*space).arbiters);
    cpArrayFree((*space).pooledArbiters);
    if !((*space).allocatedBuffers).is_null() {
        cpArrayFreeEach(
            (*space).allocatedBuffers,
            Some(free as unsafe extern "C" fn(*mut libc::c_void) -> ()),
        );
        cpArrayFree((*space).allocatedBuffers);
    }
    if !((*space).postStepCallbacks).is_null() {
        cpArrayFreeEach(
            (*space).postStepCallbacks,
            Some(free as unsafe extern "C" fn(*mut libc::c_void) -> ()),
        );
        cpArrayFree((*space).postStepCallbacks);
    }
    if !((*space).collisionHandlers).is_null() {
        cpHashSetEach(
            (*space).collisionHandlers,
            Some(
                FreeWrap
                    as unsafe extern "C" fn(*mut libc::c_void, *mut libc::c_void) -> (),
            ),
            0 as *mut libc::c_void,
        );
    }
    cpHashSetFree((*space).collisionHandlers);
}
pub unsafe extern "C" fn cpSpaceFree(mut space: *mut cpSpace) {
    if !space.is_null() {
        cpSpaceDestroy(space);
        free(space as *mut libc::c_void);
    }
}
pub unsafe extern "C" fn cpSpaceGetIterations(mut space: *const cpSpace) -> libc::c_int {
    return (*space).iterations;
}
pub unsafe extern "C" fn cpSpaceSetIterations(
    mut space: *mut cpSpace,
    mut iterations: libc::c_int,
) {
    if !(iterations > 0 as libc::c_int) {
        cpMessage(
            b"iterations > 0\0" as *const u8 as *const libc::c_char,
            b"../../src/cpSpace.c\0" as *const u8 as *const libc::c_char,
            243 as libc::c_int,
            1 as libc::c_int,
            1 as libc::c_int,
            b"Iterations must be positive and non-zero.\0" as *const u8
                as *const libc::c_char,
        );
        abort();
    }
    (*space).iterations = iterations;
}
pub unsafe extern "C" fn cpSpaceGetGravity(mut space: *const cpSpace) -> cpVect {
    return (*space).gravity;
}
pub unsafe extern "C" fn cpSpaceSetGravity(
    mut space: *mut cpSpace,
    mut gravity: cpVect,
) {
    (*space).gravity = gravity;
    let mut components: *mut cpArray = (*space).sleepingComponents;
    let mut i: libc::c_int = 0 as libc::c_int;
    while i < (*components).num {
        cpBodyActivate(*((*components).arr).offset(i as isize) as *mut cpBody);
        i += 1;
        i;
    }
}
pub unsafe extern "C" fn cpSpaceGetDamping(mut space: *const cpSpace) -> cpFloat {
    return (*space).damping;
}
pub unsafe extern "C" fn cpSpaceSetDamping(
    mut space: *mut cpSpace,
    mut damping: cpFloat,
) {
    if !(damping >= 0.0f64) {
        cpMessage(
            b"damping >= 0.0\0" as *const u8 as *const libc::c_char,
            b"../../src/cpSpace.c\0" as *const u8 as *const libc::c_char,
            274 as libc::c_int,
            1 as libc::c_int,
            1 as libc::c_int,
            b"Damping must be positive.\0" as *const u8 as *const libc::c_char,
        );
        abort();
    }
    (*space).damping = damping;
}
pub unsafe extern "C" fn cpSpaceGetIdleSpeedThreshold(
    mut space: *const cpSpace,
) -> cpFloat {
    return (*space).idleSpeedThreshold;
}
pub unsafe extern "C" fn cpSpaceSetIdleSpeedThreshold(
    mut space: *mut cpSpace,
    mut idleSpeedThreshold: cpFloat,
) {
    (*space).idleSpeedThreshold = idleSpeedThreshold;
}
pub unsafe extern "C" fn cpSpaceGetSleepTimeThreshold(
    mut space: *const cpSpace,
) -> cpFloat {
    return (*space).sleepTimeThreshold;
}
pub unsafe extern "C" fn cpSpaceSetSleepTimeThreshold(
    mut space: *mut cpSpace,
    mut sleepTimeThreshold: cpFloat,
) {
    (*space).sleepTimeThreshold = sleepTimeThreshold;
}
pub unsafe extern "C" fn cpSpaceGetCollisionSlop(mut space: *const cpSpace) -> cpFloat {
    return (*space).collisionSlop;
}
pub unsafe extern "C" fn cpSpaceSetCollisionSlop(
    mut space: *mut cpSpace,
    mut collisionSlop: cpFloat,
) {
    (*space).collisionSlop = collisionSlop;
}
pub unsafe extern "C" fn cpSpaceGetCollisionBias(mut space: *const cpSpace) -> cpFloat {
    return (*space).collisionBias;
}
pub unsafe extern "C" fn cpSpaceSetCollisionBias(
    mut space: *mut cpSpace,
    mut collisionBias: cpFloat,
) {
    (*space).collisionBias = collisionBias;
}
pub unsafe extern "C" fn cpSpaceGetCollisionPersistence(
    mut space: *const cpSpace,
) -> cpTimestamp {
    return (*space).collisionPersistence;
}
pub unsafe extern "C" fn cpSpaceSetCollisionPersistence(
    mut space: *mut cpSpace,
    mut collisionPersistence: cpTimestamp,
) {
    (*space).collisionPersistence = collisionPersistence;
}
pub unsafe extern "C" fn cpSpaceGetUserData(mut space: *const cpSpace) -> cpDataPointer {
    return (*space).userData;
}
pub unsafe extern "C" fn cpSpaceSetUserData(
    mut space: *mut cpSpace,
    mut userData: cpDataPointer,
) {
    (*space).userData = userData;
}
pub unsafe extern "C" fn cpSpaceGetStaticBody(mut space: *const cpSpace) -> *mut cpBody {
    return (*space).staticBody;
}
pub unsafe extern "C" fn cpSpaceGetCurrentTimeStep(
    mut space: *const cpSpace,
) -> cpFloat {
    return (*space).curr_dt;
}
pub unsafe extern "C" fn cpSpaceSetStaticBody(
    mut space: *mut cpSpace,
    mut body: *mut cpBody,
) {
    if !((*space).staticBody).is_null() {
        if !((*(*space).staticBody).shapeList).is_null() {
            cpMessage(
                b"space->staticBody->shapeList == NULL\0" as *const u8
                    as *const libc::c_char,
                b"../../src/cpSpace.c\0" as *const u8 as *const libc::c_char,
                366 as libc::c_int,
                1 as libc::c_int,
                1 as libc::c_int,
                b"Internal Error: Changing the designated static body while the old one still had shapes attached.\0"
                    as *const u8 as *const libc::c_char,
            );
            abort();
        }
        (*(*space).staticBody).space = 0 as *mut cpSpace;
    }
    (*space).staticBody = body;
    (*body).space = space;
}
pub unsafe extern "C" fn cpSpaceIsLocked(mut space: *mut cpSpace) -> cpBool {
    return ((*space).locked > 0 as libc::c_int) as libc::c_int as cpBool;
}
unsafe extern "C" fn cpSpaceUseWildcardDefaultHandler(mut space: *mut cpSpace) {
    if (*space).usesWildcards == 0 {
        (*space).usesWildcards = 1 as libc::c_int as cpBool;
        memcpy(
            &mut (*space).defaultHandler as *mut cpCollisionHandler as *mut libc::c_void,
            &mut cpCollisionHandlerDefault as *mut cpCollisionHandler
                as *const libc::c_void,
            ::std::mem::size_of::<cpCollisionHandler>() as libc::c_ulong,
        );
    }
}
pub unsafe extern "C" fn cpSpaceAddDefaultCollisionHandler(
    mut space: *mut cpSpace,
) -> *mut cpCollisionHandler {
    cpSpaceUseWildcardDefaultHandler(space);
    return &mut (*space).defaultHandler;
}
pub unsafe extern "C" fn cpSpaceAddCollisionHandler(
    mut space: *mut cpSpace,
    mut a: cpCollisionType,
    mut b: cpCollisionType,
) -> *mut cpCollisionHandler {
    let mut hash: cpHashValue = a.wrapping_mul(3344921057 as libc::c_ulong)
        ^ b.wrapping_mul(3344921057 as libc::c_ulong);
    let mut handler: cpCollisionHandler = {
        let mut init = cpCollisionHandler {
            typeA: a,
            typeB: b,
            beginFunc: Some(
                DefaultBegin
                    as unsafe extern "C" fn(
                        *mut cpArbiter,
                        *mut cpSpace,
                        cpDataPointer,
                    ) -> cpBool,
            ),
            preSolveFunc: Some(
                DefaultPreSolve
                    as unsafe extern "C" fn(
                        *mut cpArbiter,
                        *mut cpSpace,
                        cpDataPointer,
                    ) -> cpBool,
            ),
            postSolveFunc: Some(
                DefaultPostSolve
                    as unsafe extern "C" fn(
                        *mut cpArbiter,
                        *mut cpSpace,
                        cpDataPointer,
                    ) -> (),
            ),
            separateFunc: Some(
                DefaultSeparate
                    as unsafe extern "C" fn(
                        *mut cpArbiter,
                        *mut cpSpace,
                        cpDataPointer,
                    ) -> (),
            ),
            userData: 0 as *mut libc::c_void,
        };
        init
    };
    return cpHashSetInsert(
        (*space).collisionHandlers,
        hash,
        &mut handler as *mut cpCollisionHandler as *const libc::c_void,
        ::std::mem::transmute::<
            Option::<
                unsafe extern "C" fn(
                    *mut cpCollisionHandler,
                    *mut libc::c_void,
                ) -> *mut libc::c_void,
            >,
            cpHashSetTransFunc,
        >(
            Some(
                handlerSetTrans
                    as unsafe extern "C" fn(
                        *mut cpCollisionHandler,
                        *mut libc::c_void,
                    ) -> *mut libc::c_void,
            ),
        ),
        0 as *mut libc::c_void,
    ) as *mut cpCollisionHandler;
}
pub unsafe extern "C" fn cpSpaceAddWildcardHandler(
    mut space: *mut cpSpace,
    mut type_0: cpCollisionType,
) -> *mut cpCollisionHandler {
    cpSpaceUseWildcardDefaultHandler(space);
    let mut hash: cpHashValue = type_0.wrapping_mul(3344921057 as libc::c_ulong)
        ^ (!(0 as libc::c_int as cpCollisionType))
            .wrapping_mul(3344921057 as libc::c_ulong);
    let mut handler: cpCollisionHandler = {
        let mut init = cpCollisionHandler {
            typeA: type_0,
            typeB: !(0 as libc::c_int as cpCollisionType),
            beginFunc: Some(
                AlwaysCollide
                    as unsafe extern "C" fn(
                        *mut cpArbiter,
                        *mut cpSpace,
                        cpDataPointer,
                    ) -> cpBool,
            ),
            preSolveFunc: Some(
                AlwaysCollide
                    as unsafe extern "C" fn(
                        *mut cpArbiter,
                        *mut cpSpace,
                        cpDataPointer,
                    ) -> cpBool,
            ),
            postSolveFunc: Some(
                DoNothing
                    as unsafe extern "C" fn(
                        *mut cpArbiter,
                        *mut cpSpace,
                        cpDataPointer,
                    ) -> (),
            ),
            separateFunc: Some(
                DoNothing
                    as unsafe extern "C" fn(
                        *mut cpArbiter,
                        *mut cpSpace,
                        cpDataPointer,
                    ) -> (),
            ),
            userData: 0 as *mut libc::c_void,
        };
        init
    };
    return cpHashSetInsert(
        (*space).collisionHandlers,
        hash,
        &mut handler as *mut cpCollisionHandler as *const libc::c_void,
        ::std::mem::transmute::<
            Option::<
                unsafe extern "C" fn(
                    *mut cpCollisionHandler,
                    *mut libc::c_void,
                ) -> *mut libc::c_void,
            >,
            cpHashSetTransFunc,
        >(
            Some(
                handlerSetTrans
                    as unsafe extern "C" fn(
                        *mut cpCollisionHandler,
                        *mut libc::c_void,
                    ) -> *mut libc::c_void,
            ),
        ),
        0 as *mut libc::c_void,
    ) as *mut cpCollisionHandler;
}
pub unsafe extern "C" fn cpSpaceAddShape(
    mut space: *mut cpSpace,
    mut shape: *mut cpShape,
) -> *mut cpShape {
    if !((*shape).space != space) {
        cpMessage(
            b"shape->space != space\0" as *const u8 as *const libc::c_char,
            b"../../src/cpSpace.c\0" as *const u8 as *const libc::c_char,
            420 as libc::c_int,
            1 as libc::c_int,
            1 as libc::c_int,
            b"You have already added this shape to this space. You must not add it a second time.\0"
                as *const u8 as *const libc::c_char,
        );
        abort();
    }
    if !((*shape).space).is_null() {
        cpMessage(
            b"!shape->space\0" as *const u8 as *const libc::c_char,
            b"../../src/cpSpace.c\0" as *const u8 as *const libc::c_char,
            421 as libc::c_int,
            1 as libc::c_int,
            1 as libc::c_int,
            b"You have already added this shape to another space. You cannot add it to a second.\0"
                as *const u8 as *const libc::c_char,
        );
        abort();
    }
    if ((*shape).body).is_null() {
        cpMessage(
            b"shape->body\0" as *const u8 as *const libc::c_char,
            b"../../src/cpSpace.c\0" as *const u8 as *const libc::c_char,
            422 as libc::c_int,
            1 as libc::c_int,
            1 as libc::c_int,
            b"The shape's body is not defined.\0" as *const u8 as *const libc::c_char,
        );
        abort();
    }
    if !((*(*shape).body).space == space) {
        cpMessage(
            b"shape->body->space == space\0" as *const u8 as *const libc::c_char,
            b"../../src/cpSpace.c\0" as *const u8 as *const libc::c_char,
            423 as libc::c_int,
            1 as libc::c_int,
            1 as libc::c_int,
            b"The shape's body must be added to the space before the shape.\0"
                as *const u8 as *const libc::c_char,
        );
        abort();
    }
    if (*space).locked != 0 {
        cpMessage(
            b"!space->locked\0" as *const u8 as *const libc::c_char,
            b"../../src/cpSpace.c\0" as *const u8 as *const libc::c_char,
            424 as libc::c_int,
            1 as libc::c_int,
            1 as libc::c_int,
            b"This operation cannot be done safely during a call to cpSpaceStep() or during a query. Put these calls into a post-step callback.\0"
                as *const u8 as *const libc::c_char,
        );
        abort();
    }
    let mut body: *mut cpBody = (*shape).body;
    let mut isStatic: cpBool = (cpBodyGetType(body) as libc::c_uint
        == CP_BODY_TYPE_STATIC as libc::c_int as libc::c_uint) as libc::c_int as cpBool;
    if isStatic == 0 {
        cpBodyActivate(body);
    }
    cpBodyAddShape(body, shape);
    let fresh0 = (*space).shapeIDCounter;
    (*space).shapeIDCounter = ((*space).shapeIDCounter).wrapping_add(1);
    (*shape).hashid = fresh0;
    cpShapeUpdate(shape, (*body).transform);
    cpSpatialIndexInsert(
        if isStatic as libc::c_int != 0 {
            (*space).staticShapes
        } else {
            (*space).dynamicShapes
        },
        shape as *mut libc::c_void,
        (*shape).hashid,
    );
    (*shape).space = space;
    return shape;
}
pub unsafe extern "C" fn cpSpaceAddBody(
    mut space: *mut cpSpace,
    mut body: *mut cpBody,
) -> *mut cpBody {
    if !((*body).space != space) {
        cpMessage(
            b"body->space != space\0" as *const u8 as *const libc::c_char,
            b"../../src/cpSpace.c\0" as *const u8 as *const libc::c_char,
            443 as libc::c_int,
            1 as libc::c_int,
            1 as libc::c_int,
            b"You have already added this body to this space. You must not add it a second time.\0"
                as *const u8 as *const libc::c_char,
        );
        abort();
    }
    if !((*body).space).is_null() {
        cpMessage(
            b"!body->space\0" as *const u8 as *const libc::c_char,
            b"../../src/cpSpace.c\0" as *const u8 as *const libc::c_char,
            444 as libc::c_int,
            1 as libc::c_int,
            1 as libc::c_int,
            b"You have already added this body to another space. You cannot add it to a second.\0"
                as *const u8 as *const libc::c_char,
        );
        abort();
    }
    if (*space).locked != 0 {
        cpMessage(
            b"!space->locked\0" as *const u8 as *const libc::c_char,
            b"../../src/cpSpace.c\0" as *const u8 as *const libc::c_char,
            445 as libc::c_int,
            1 as libc::c_int,
            1 as libc::c_int,
            b"This operation cannot be done safely during a call to cpSpaceStep() or during a query. Put these calls into a post-step callback.\0"
                as *const u8 as *const libc::c_char,
        );
        abort();
    }
    cpArrayPush(
        cpSpaceArrayForBodyType(space, cpBodyGetType(body)),
        body as *mut libc::c_void,
    );
    (*body).space = space;
    return body;
}
pub unsafe extern "C" fn cpSpaceAddConstraint(
    mut space: *mut cpSpace,
    mut constraint: *mut cpConstraint,
) -> *mut cpConstraint {
    if !((*constraint).space != space) {
        cpMessage(
            b"constraint->space != space\0" as *const u8 as *const libc::c_char,
            b"../../src/cpSpace.c\0" as *const u8 as *const libc::c_char,
            456 as libc::c_int,
            1 as libc::c_int,
            1 as libc::c_int,
            b"You have already added this constraint to this space. You must not add it a second time.\0"
                as *const u8 as *const libc::c_char,
        );
        abort();
    }
    if !((*constraint).space).is_null() {
        cpMessage(
            b"!constraint->space\0" as *const u8 as *const libc::c_char,
            b"../../src/cpSpace.c\0" as *const u8 as *const libc::c_char,
            457 as libc::c_int,
            1 as libc::c_int,
            1 as libc::c_int,
            b"You have already added this constraint to another space. You cannot add it to a second.\0"
                as *const u8 as *const libc::c_char,
        );
        abort();
    }
    if (*space).locked != 0 {
        cpMessage(
            b"!space->locked\0" as *const u8 as *const libc::c_char,
            b"../../src/cpSpace.c\0" as *const u8 as *const libc::c_char,
            458 as libc::c_int,
            1 as libc::c_int,
            1 as libc::c_int,
            b"This operation cannot be done safely during a call to cpSpaceStep() or during a query. Put these calls into a post-step callback.\0"
                as *const u8 as *const libc::c_char,
        );
        abort();
    }
    let mut a: *mut cpBody = (*constraint).a;
    let mut b: *mut cpBody = (*constraint).b;
    if !(!a.is_null() && !b.is_null()) {
        cpMessage(
            b"a != NULL && b != NULL\0" as *const u8 as *const libc::c_char,
            b"../../src/cpSpace.c\0" as *const u8 as *const libc::c_char,
            461 as libc::c_int,
            1 as libc::c_int,
            1 as libc::c_int,
            b"Constraint is attached to a NULL body.\0" as *const u8
                as *const libc::c_char,
        );
        abort();
    }
    cpBodyActivate(a);
    cpBodyActivate(b);
    cpArrayPush((*space).constraints, constraint as *mut libc::c_void);
    (*constraint).next_a = (*a).constraintList;
    (*a).constraintList = constraint;
    (*constraint).next_b = (*b).constraintList;
    (*b).constraintList = constraint;
    (*constraint).space = space;
    return constraint;
}
unsafe extern "C" fn cachedArbitersFilter(
    mut arb: *mut cpArbiter,
    mut context: *mut arbiterFilterContext,
) -> cpBool {
    let mut shape: *mut cpShape = (*context).shape;
    let mut body: *mut cpBody = (*context).body;
    if body == (*arb).body_a && (shape == (*arb).a as *mut cpShape || shape.is_null())
        || body == (*arb).body_b
            && (shape == (*arb).b as *mut cpShape || shape.is_null())
    {
        if !shape.is_null()
            && (*arb).state as libc::c_uint
                != CP_ARBITER_STATE_CACHED as libc::c_int as libc::c_uint
        {
            (*arb).state = CP_ARBITER_STATE_INVALIDATED;
            let mut handler: *mut cpCollisionHandler = (*arb).handler;
            ((*handler).separateFunc)
                .unwrap()(arb, (*context).space, (*handler).userData);
        }
        cpArbiterUnthread(arb);
        cpArrayDeleteObj((*(*context).space).arbiters, arb as *mut libc::c_void);
        cpArrayPush((*(*context).space).pooledArbiters, arb as *mut libc::c_void);
        return 0 as libc::c_int as cpBool;
    }
    return 1 as libc::c_int as cpBool;
}
pub unsafe extern "C" fn cpSpaceFilterArbiters(
    mut space: *mut cpSpace,
    mut body: *mut cpBody,
    mut filter: *mut cpShape,
) {
    cpSpaceLock(space);
    let mut context: arbiterFilterContext = {
        let mut init = arbiterFilterContext {
            space: space,
            body: body,
            shape: filter,
        };
        init
    };
    cpHashSetFilter(
        (*space).cachedArbiters,
        ::std::mem::transmute::<
            Option::<
                unsafe extern "C" fn(*mut cpArbiter, *mut arbiterFilterContext) -> cpBool,
            >,
            cpHashSetFilterFunc,
        >(
            Some(
                cachedArbitersFilter
                    as unsafe extern "C" fn(
                        *mut cpArbiter,
                        *mut arbiterFilterContext,
                    ) -> cpBool,
            ),
        ),
        &mut context as *mut arbiterFilterContext as *mut libc::c_void,
    );
    cpSpaceUnlock(space, 1 as libc::c_int as cpBool);
}
pub unsafe extern "C" fn cpSpaceRemoveShape(
    mut space: *mut cpSpace,
    mut shape: *mut cpShape,
) {
    let mut body: *mut cpBody = (*shape).body;
    if cpSpaceContainsShape(space, shape) == 0 {
        cpMessage(
            b"cpSpaceContainsShape(space, shape)\0" as *const u8 as *const libc::c_char,
            b"../../src/cpSpace.c\0" as *const u8 as *const libc::c_char,
            526 as libc::c_int,
            1 as libc::c_int,
            1 as libc::c_int,
            b"Cannot remove a shape that was not added to the space. (Removed twice maybe?)\0"
                as *const u8 as *const libc::c_char,
        );
        abort();
    }
    if (*space).locked != 0 {
        cpMessage(
            b"!space->locked\0" as *const u8 as *const libc::c_char,
            b"../../src/cpSpace.c\0" as *const u8 as *const libc::c_char,
            527 as libc::c_int,
            1 as libc::c_int,
            1 as libc::c_int,
            b"This operation cannot be done safely during a call to cpSpaceStep() or during a query. Put these calls into a post-step callback.\0"
                as *const u8 as *const libc::c_char,
        );
        abort();
    }
    let mut isStatic: cpBool = (cpBodyGetType(body) as libc::c_uint
        == CP_BODY_TYPE_STATIC as libc::c_int as libc::c_uint) as libc::c_int as cpBool;
    if isStatic != 0 {
        cpBodyActivateStatic(body, shape);
    } else {
        cpBodyActivate(body);
    }
    cpBodyRemoveShape(body, shape);
    cpSpaceFilterArbiters(space, body, shape);
    cpSpatialIndexRemove(
        if isStatic as libc::c_int != 0 {
            (*space).staticShapes
        } else {
            (*space).dynamicShapes
        },
        shape as *mut libc::c_void,
        (*shape).hashid,
    );
    (*shape).space = 0 as *mut cpSpace;
    (*shape).hashid = 0 as libc::c_int as cpHashValue;
}
pub unsafe extern "C" fn cpSpaceRemoveBody(
    mut space: *mut cpSpace,
    mut body: *mut cpBody,
) {
    if !(body != cpSpaceGetStaticBody(space)) {
        cpMessage(
            b"body != cpSpaceGetStaticBody(space)\0" as *const u8 as *const libc::c_char,
            b"../../src/cpSpace.c\0" as *const u8 as *const libc::c_char,
            546 as libc::c_int,
            1 as libc::c_int,
            1 as libc::c_int,
            b"Cannot remove the designated static body for the space.\0" as *const u8
                as *const libc::c_char,
        );
        abort();
    }
    if cpSpaceContainsBody(space, body) == 0 {
        cpMessage(
            b"cpSpaceContainsBody(space, body)\0" as *const u8 as *const libc::c_char,
            b"../../src/cpSpace.c\0" as *const u8 as *const libc::c_char,
            547 as libc::c_int,
            1 as libc::c_int,
            1 as libc::c_int,
            b"Cannot remove a body that was not added to the space. (Removed twice maybe?)\0"
                as *const u8 as *const libc::c_char,
        );
        abort();
    }
    if (*space).locked != 0 {
        cpMessage(
            b"!space->locked\0" as *const u8 as *const libc::c_char,
            b"../../src/cpSpace.c\0" as *const u8 as *const libc::c_char,
            550 as libc::c_int,
            1 as libc::c_int,
            1 as libc::c_int,
            b"This operation cannot be done safely during a call to cpSpaceStep() or during a query. Put these calls into a post-step callback.\0"
                as *const u8 as *const libc::c_char,
        );
        abort();
    }
    cpBodyActivate(body);
    cpArrayDeleteObj(
        cpSpaceArrayForBodyType(space, cpBodyGetType(body)),
        body as *mut libc::c_void,
    );
    (*body).space = 0 as *mut cpSpace;
}
pub unsafe extern "C" fn cpSpaceRemoveConstraint(
    mut space: *mut cpSpace,
    mut constraint: *mut cpConstraint,
) {
    if cpSpaceContainsConstraint(space, constraint) == 0 {
        cpMessage(
            b"cpSpaceContainsConstraint(space, constraint)\0" as *const u8
                as *const libc::c_char,
            b"../../src/cpSpace.c\0" as *const u8 as *const libc::c_char,
            561 as libc::c_int,
            1 as libc::c_int,
            1 as libc::c_int,
            b"Cannot remove a constraint that was not added to the space. (Removed twice maybe?)\0"
                as *const u8 as *const libc::c_char,
        );
        abort();
    }
    if (*space).locked != 0 {
        cpMessage(
            b"!space->locked\0" as *const u8 as *const libc::c_char,
            b"../../src/cpSpace.c\0" as *const u8 as *const libc::c_char,
            562 as libc::c_int,
            1 as libc::c_int,
            1 as libc::c_int,
            b"This operation cannot be done safely during a call to cpSpaceStep() or during a query. Put these calls into a post-step callback.\0"
                as *const u8 as *const libc::c_char,
        );
        abort();
    }
    cpBodyActivate((*constraint).a);
    cpBodyActivate((*constraint).b);
    cpArrayDeleteObj((*space).constraints, constraint as *mut libc::c_void);
    cpBodyRemoveConstraint((*constraint).a, constraint);
    cpBodyRemoveConstraint((*constraint).b, constraint);
    (*constraint).space = 0 as *mut cpSpace;
}
pub unsafe extern "C" fn cpSpaceContainsShape(
    mut space: *mut cpSpace,
    mut shape: *mut cpShape,
) -> cpBool {
    return ((*shape).space == space) as libc::c_int as cpBool;
}
pub unsafe extern "C" fn cpSpaceContainsBody(
    mut space: *mut cpSpace,
    mut body: *mut cpBody,
) -> cpBool {
    return ((*body).space == space) as libc::c_int as cpBool;
}
pub unsafe extern "C" fn cpSpaceContainsConstraint(
    mut space: *mut cpSpace,
    mut constraint: *mut cpConstraint,
) -> cpBool {
    return ((*constraint).space == space) as libc::c_int as cpBool;
}
pub unsafe extern "C" fn cpSpaceEachBody(
    mut space: *mut cpSpace,
    mut func: cpSpaceBodyIteratorFunc,
    mut data: *mut libc::c_void,
) {
    cpSpaceLock(space);
    let mut bodies: *mut cpArray = (*space).dynamicBodies;
    let mut i: libc::c_int = 0 as libc::c_int;
    while i < (*bodies).num {
        func.unwrap()(*((*bodies).arr).offset(i as isize) as *mut cpBody, data);
        i += 1;
        i;
    }
    let mut otherBodies: *mut cpArray = (*space).staticBodies;
    let mut i_0: libc::c_int = 0 as libc::c_int;
    while i_0 < (*otherBodies).num {
        func.unwrap()(*((*otherBodies).arr).offset(i_0 as isize) as *mut cpBody, data);
        i_0 += 1;
        i_0;
    }
    let mut components: *mut cpArray = (*space).sleepingComponents;
    let mut i_1: libc::c_int = 0 as libc::c_int;
    while i_1 < (*components).num {
        let mut root: *mut cpBody = *((*components).arr).offset(i_1 as isize)
            as *mut cpBody;
        let mut body: *mut cpBody = root;
        while !body.is_null() {
            let mut next: *mut cpBody = (*body).sleeping.next;
            func.unwrap()(body, data);
            body = next;
        }
        i_1 += 1;
        i_1;
    }
    cpSpaceUnlock(space, 1 as libc::c_int as cpBool);
}
unsafe extern "C" fn spaceEachShapeIterator(
    mut shape: *mut cpShape,
    mut context: *mut spaceShapeContext,
) {
    ((*context).func).unwrap()(shape, (*context).data);
}
pub unsafe extern "C" fn cpSpaceEachShape(
    mut space: *mut cpSpace,
    mut func: cpSpaceShapeIteratorFunc,
    mut data: *mut libc::c_void,
) {
    cpSpaceLock(space);
    let mut context: spaceShapeContext = {
        let mut init = spaceShapeContext {
            func: func,
            data: data,
        };
        init
    };
    cpSpatialIndexEach(
        (*space).dynamicShapes,
        ::std::mem::transmute::<
            Option::<unsafe extern "C" fn(*mut cpShape, *mut spaceShapeContext) -> ()>,
            cpSpatialIndexIteratorFunc,
        >(
            Some(
                spaceEachShapeIterator
                    as unsafe extern "C" fn(*mut cpShape, *mut spaceShapeContext) -> (),
            ),
        ),
        &mut context as *mut spaceShapeContext as *mut libc::c_void,
    );
    cpSpatialIndexEach(
        (*space).staticShapes,
        ::std::mem::transmute::<
            Option::<unsafe extern "C" fn(*mut cpShape, *mut spaceShapeContext) -> ()>,
            cpSpatialIndexIteratorFunc,
        >(
            Some(
                spaceEachShapeIterator
                    as unsafe extern "C" fn(*mut cpShape, *mut spaceShapeContext) -> (),
            ),
        ),
        &mut context as *mut spaceShapeContext as *mut libc::c_void,
    );
    cpSpaceUnlock(space, 1 as libc::c_int as cpBool);
}
pub unsafe extern "C" fn cpSpaceEachConstraint(
    mut space: *mut cpSpace,
    mut func: cpSpaceConstraintIteratorFunc,
    mut data: *mut libc::c_void,
) {
    cpSpaceLock(space);
    let mut constraints: *mut cpArray = (*space).constraints;
    let mut i: libc::c_int = 0 as libc::c_int;
    while i < (*constraints).num {
        func
            .unwrap()(
            *((*constraints).arr).offset(i as isize) as *mut cpConstraint,
            data,
        );
        i += 1;
        i;
    }
    cpSpaceUnlock(space, 1 as libc::c_int as cpBool);
}
pub unsafe extern "C" fn cpSpaceReindexStatic(mut space: *mut cpSpace) {
    if (*space).locked != 0 {
        cpMessage(
            b"!space->locked\0" as *const u8 as *const libc::c_char,
            b"../../src/cpSpace.c\0" as *const u8 as *const libc::c_char,
            656 as libc::c_int,
            1 as libc::c_int,
            1 as libc::c_int,
            b"You cannot manually reindex objects while the space is locked. Wait until the current query or step is complete.\0"
                as *const u8 as *const libc::c_char,
        );
        abort();
    }
    cpSpatialIndexEach(
        (*space).staticShapes,
        ::std::mem::transmute::<
            Option::<unsafe extern "C" fn(*mut cpShape, *mut libc::c_void) -> ()>,
            cpSpatialIndexIteratorFunc,
        >(
            Some(
                cpShapeUpdateFunc
                    as unsafe extern "C" fn(*mut cpShape, *mut libc::c_void) -> (),
            ),
        ),
        0 as *mut libc::c_void,
    );
    cpSpatialIndexReindex((*space).staticShapes);
}
pub unsafe extern "C" fn cpSpaceReindexShape(
    mut space: *mut cpSpace,
    mut shape: *mut cpShape,
) {
    if (*space).locked != 0 {
        cpMessage(
            b"!space->locked\0" as *const u8 as *const libc::c_char,
            b"../../src/cpSpace.c\0" as *const u8 as *const libc::c_char,
            665 as libc::c_int,
            1 as libc::c_int,
            1 as libc::c_int,
            b"You cannot manually reindex objects while the space is locked. Wait until the current query or step is complete.\0"
                as *const u8 as *const libc::c_char,
        );
        abort();
    }
    cpShapeCacheBB(shape);
    cpSpatialIndexReindexObject(
        (*space).dynamicShapes,
        shape as *mut libc::c_void,
        (*shape).hashid,
    );
    cpSpatialIndexReindexObject(
        (*space).staticShapes,
        shape as *mut libc::c_void,
        (*shape).hashid,
    );
}
pub unsafe extern "C" fn cpSpaceReindexShapesForBody(
    mut space: *mut cpSpace,
    mut body: *mut cpBody,
) {
    let mut shape: *mut cpShape = (*body).shapeList;
    while !shape.is_null() {
        cpSpaceReindexShape(space, shape);
        shape = (*shape).next;
    }
}
unsafe extern "C" fn copyShapes(
    mut shape: *mut cpShape,
    mut index: *mut cpSpatialIndex,
) {
    cpSpatialIndexInsert(index, shape as *mut libc::c_void, (*shape).hashid);
}
pub unsafe extern "C" fn cpSpaceUseSpatialHash(
    mut space: *mut cpSpace,
    mut dim: cpFloat,
    mut count: libc::c_int,
) {
    let mut staticShapes: *mut cpSpatialIndex = cpSpaceHashNew(
        dim,
        count,
        ::std::mem::transmute::<
            Option::<unsafe extern "C" fn(*const cpShape) -> cpBB>,
            cpSpatialIndexBBFunc,
        >(Some(cpShapeGetBB as unsafe extern "C" fn(*const cpShape) -> cpBB)),
        0 as *mut cpSpatialIndex,
    );
    let mut dynamicShapes: *mut cpSpatialIndex = cpSpaceHashNew(
        dim,
        count,
        ::std::mem::transmute::<
            Option::<unsafe extern "C" fn(*const cpShape) -> cpBB>,
            cpSpatialIndexBBFunc,
        >(Some(cpShapeGetBB as unsafe extern "C" fn(*const cpShape) -> cpBB)),
        staticShapes,
    );
    cpSpatialIndexEach(
        (*space).staticShapes,
        ::std::mem::transmute::<
            Option::<unsafe extern "C" fn(*mut cpShape, *mut cpSpatialIndex) -> ()>,
            cpSpatialIndexIteratorFunc,
        >(
            Some(
                copyShapes
                    as unsafe extern "C" fn(*mut cpShape, *mut cpSpatialIndex) -> (),
            ),
        ),
        staticShapes as *mut libc::c_void,
    );
    cpSpatialIndexEach(
        (*space).dynamicShapes,
        ::std::mem::transmute::<
            Option::<unsafe extern "C" fn(*mut cpShape, *mut cpSpatialIndex) -> ()>,
            cpSpatialIndexIteratorFunc,
        >(
            Some(
                copyShapes
                    as unsafe extern "C" fn(*mut cpShape, *mut cpSpatialIndex) -> (),
            ),
        ),
        dynamicShapes as *mut libc::c_void,
    );
    cpSpatialIndexFree((*space).staticShapes);
    cpSpatialIndexFree((*space).dynamicShapes);
    (*space).staticShapes = staticShapes;
    (*space).dynamicShapes = dynamicShapes;
}
