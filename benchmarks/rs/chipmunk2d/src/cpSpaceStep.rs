use ::libc;
extern "C" {
    pub type cpHashSet;
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
    fn cpArbiterIgnore(arb: *mut cpArbiter) -> cpBool;
    fn cpBodyIsSleeping(body: *const cpBody) -> cpBool;
    fn cpBodyGetType(body: *mut cpBody) -> cpBodyType;
    fn cpShapeCacheBB(shape: *mut cpShape) -> cpBB;
    fn cpArrayPush(arr: *mut cpArray, object: *mut libc::c_void);
    fn cpSpaceActivateBody(space: *mut cpSpace, body: *mut cpBody);
    fn cpArbiterApplyImpulse(arb: *mut cpArbiter);
    fn cpArbiterApplyCachedImpulse(arb: *mut cpArbiter, dt_coef: cpFloat);
    fn cpArbiterPreStep(arb: *mut cpArbiter, dt: cpFloat, bias: cpFloat, slop: cpFloat);
    fn cpHashSetFilter(
        set: *mut cpHashSet,
        func: cpHashSetFilterFunc,
        data: *mut libc::c_void,
    );
    fn cpSpaceProcessComponents(space: *mut cpSpace, dt: cpFloat);
    fn cpCollide(
        a: *const cpShape,
        b: *const cpShape,
        id: cpCollisionID,
        contacts: *mut cpContact,
    ) -> cpCollisionInfo;
    fn cpArrayPop(arr: *mut cpArray) -> *mut libc::c_void;
    fn cpArbiterInit(
        arb: *mut cpArbiter,
        a: *mut cpShape,
        b: *mut cpShape,
    ) -> *mut cpArbiter;
    fn cpHashSetInsert(
        set: *mut cpHashSet,
        hash: cpHashValue,
        ptr: *const libc::c_void,
        trans: cpHashSetTransFunc,
        data: *mut libc::c_void,
    ) -> *const libc::c_void;
    fn cpArbiterUpdate(
        arb: *mut cpArbiter,
        info: *mut cpCollisionInfo,
        space: *mut cpSpace,
    );
    fn cpArbiterUnthread(arb: *mut cpArbiter);
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
pub struct cpContactBufferHeader {
    pub stamp: cpTimestamp,
    pub next: *mut cpContactBufferHeader,
    pub numContacts: libc::c_uint,
}
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
pub type cpBodyType = libc::c_uint;
pub const CP_BODY_TYPE_STATIC: cpBodyType = 2;
pub const CP_BODY_TYPE_KINEMATIC: cpBodyType = 1;
pub const CP_BODY_TYPE_DYNAMIC: cpBodyType = 0;
pub type cpPostStepFunc = Option::<
    unsafe extern "C" fn(*mut cpSpace, *mut libc::c_void, *mut libc::c_void) -> (),
>;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct cpPostStepCallback {
    pub func: cpPostStepFunc,
    pub key: *mut libc::c_void,
    pub data: *mut libc::c_void,
}
pub type cpHashSetFilterFunc = Option::<
    unsafe extern "C" fn(*mut libc::c_void, *mut libc::c_void) -> cpBool,
>;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct cpCollisionInfo {
    pub a: *const cpShape,
    pub b: *const cpShape,
    pub id: cpCollisionID,
    pub n: cpVect,
    pub count: libc::c_int,
    pub arr: *mut cpContact,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct cpContactBuffer {
    pub header: cpContactBufferHeader,
    pub contacts: [cpContact; 341],
}
pub type cpHashSetTransFunc = Option::<
    unsafe extern "C" fn(*const libc::c_void, *mut libc::c_void) -> *mut libc::c_void,
>;
#[inline]
unsafe extern "C" fn cpBBIntersects(a: cpBB, b: cpBB) -> cpBool {
    return (a.l <= b.r && b.l <= a.r && a.b <= b.t && b.b <= a.t) as libc::c_int
        as cpBool;
}
#[inline]
unsafe extern "C" fn cpSpatialIndexEach(
    mut index: *mut cpSpatialIndex,
    mut func: cpSpatialIndexIteratorFunc,
    mut data: *mut libc::c_void,
) {
    ((*(*index).klass).each).unwrap()(index, func, data);
}
#[inline]
unsafe extern "C" fn cpSpatialIndexReindexQuery(
    mut index: *mut cpSpatialIndex,
    mut func: cpSpatialIndexQueryFunc,
    mut data: *mut libc::c_void,
) {
    ((*(*index).klass).reindexQuery).unwrap()(index, func, data);
}
#[inline]
unsafe extern "C" fn cpConstraintNext(
    mut node: *mut cpConstraint,
    mut body: *mut cpBody,
) -> *mut cpConstraint {
    return if (*node).a == body { (*node).next_a } else { (*node).next_b };
}
#[inline]
unsafe extern "C" fn cpShapeFilterReject(
    mut a: cpShapeFilter,
    mut b: cpShapeFilter,
) -> cpBool {
    return (a.group != 0 as libc::c_int as libc::c_ulong && a.group == b.group
        || a.categories & b.mask == 0 as libc::c_int as libc::c_uint
        || b.categories & a.mask == 0 as libc::c_int as libc::c_uint) as libc::c_int
        as cpBool;
}
pub unsafe extern "C" fn cpSpaceGetPostStepCallback(
    mut space: *mut cpSpace,
    mut key: *mut libc::c_void,
) -> *mut cpPostStepCallback {
    let mut arr: *mut cpArray = (*space).postStepCallbacks;
    let mut i: libc::c_int = 0 as libc::c_int;
    while i < (*arr).num {
        let mut callback: *mut cpPostStepCallback = *((*arr).arr).offset(i as isize)
            as *mut cpPostStepCallback;
        if !callback.is_null() && (*callback).key == key {
            return callback;
        }
        i += 1;
        i;
    }
    return 0 as *mut cpPostStepCallback;
}
unsafe extern "C" fn PostStepDoNothing(
    mut space: *mut cpSpace,
    mut obj: *mut libc::c_void,
    mut data: *mut libc::c_void,
) {}
pub unsafe extern "C" fn cpSpaceAddPostStepCallback(
    mut space: *mut cpSpace,
    mut func: cpPostStepFunc,
    mut key: *mut libc::c_void,
    mut data: *mut libc::c_void,
) -> cpBool {
    if (cpSpaceGetPostStepCallback(space, key)).is_null() {
        let mut callback: *mut cpPostStepCallback = calloc(
            1 as libc::c_int as libc::c_ulong,
            ::std::mem::size_of::<cpPostStepCallback>() as libc::c_ulong,
        ) as *mut cpPostStepCallback;
        (*callback)
            .func = if func.is_some() {
            func
        } else {
            Some(
                PostStepDoNothing
                    as unsafe extern "C" fn(
                        *mut cpSpace,
                        *mut libc::c_void,
                        *mut libc::c_void,
                    ) -> (),
            )
        };
        (*callback).key = key;
        (*callback).data = data;
        cpArrayPush((*space).postStepCallbacks, callback as *mut libc::c_void);
        return 1 as libc::c_int as cpBool;
    } else {
        return 0 as libc::c_int as cpBool
    };
}
pub unsafe extern "C" fn cpSpaceLock(mut space: *mut cpSpace) {
    (*space).locked += 1;
    (*space).locked;
}
pub unsafe extern "C" fn cpSpaceUnlock(
    mut space: *mut cpSpace,
    mut runPostStep: cpBool,
) {
    (*space).locked -= 1;
    (*space).locked;
    if !((*space).locked >= 0 as libc::c_int) {
        cpMessage(
            b"space->locked >= 0\0" as *const u8 as *const libc::c_char,
            b"../../src/cpSpaceStep.c\0" as *const u8 as *const libc::c_char,
            72 as libc::c_int,
            1 as libc::c_int,
            1 as libc::c_int,
            b"Internal Error: Space lock underflow.\0" as *const u8
                as *const libc::c_char,
        );
        abort();
    }
    if (*space).locked == 0 as libc::c_int {
        let mut waking: *mut cpArray = (*space).rousedBodies;
        let mut i: libc::c_int = 0 as libc::c_int;
        let mut count: libc::c_int = (*waking).num;
        while i < count {
            cpSpaceActivateBody(
                space,
                *((*waking).arr).offset(i as isize) as *mut cpBody,
            );
            let ref mut fresh0 = *((*waking).arr).offset(i as isize);
            *fresh0 = 0 as *mut libc::c_void;
            i += 1;
            i;
        }
        (*waking).num = 0 as libc::c_int;
        if (*space).locked == 0 as libc::c_int && runPostStep as libc::c_int != 0
            && (*space).skipPostStep == 0
        {
            (*space).skipPostStep = 1 as libc::c_int as cpBool;
            let mut arr: *mut cpArray = (*space).postStepCallbacks;
            let mut i_0: libc::c_int = 0 as libc::c_int;
            while i_0 < (*arr).num {
                let mut callback: *mut cpPostStepCallback = *((*arr).arr)
                    .offset(i_0 as isize) as *mut cpPostStepCallback;
                let mut func: cpPostStepFunc = (*callback).func;
                (*callback).func = None;
                if func.is_some() {
                    func.unwrap()(space, (*callback).key, (*callback).data);
                }
                let ref mut fresh1 = *((*arr).arr).offset(i_0 as isize);
                *fresh1 = 0 as *mut libc::c_void;
                free(callback as *mut libc::c_void);
                i_0 += 1;
                i_0;
            }
            (*arr).num = 0 as libc::c_int;
            (*space).skipPostStep = 0 as libc::c_int as cpBool;
        }
    }
}
unsafe extern "C" fn cpSpaceAllocContactBuffer(
    mut space: *mut cpSpace,
) -> *mut cpContactBufferHeader {
    let mut buffer: *mut cpContactBuffer = calloc(
        1 as libc::c_int as libc::c_ulong,
        ::std::mem::size_of::<cpContactBuffer>() as libc::c_ulong,
    ) as *mut cpContactBuffer;
    cpArrayPush((*space).allocatedBuffers, buffer as *mut libc::c_void);
    return buffer as *mut cpContactBufferHeader;
}
unsafe extern "C" fn cpContactBufferHeaderInit(
    mut header: *mut cpContactBufferHeader,
    mut stamp: cpTimestamp,
    mut splice: *mut cpContactBufferHeader,
) -> *mut cpContactBufferHeader {
    (*header).stamp = stamp;
    (*header).next = if !splice.is_null() { (*splice).next } else { header };
    (*header).numContacts = 0 as libc::c_int as libc::c_uint;
    return header;
}
pub unsafe extern "C" fn cpSpacePushFreshContactBuffer(mut space: *mut cpSpace) {
    let mut stamp: cpTimestamp = (*space).stamp;
    let mut head: *mut cpContactBufferHeader = (*space).contactBuffersHead;
    if head.is_null() {
        (*space)
            .contactBuffersHead = cpContactBufferHeaderInit(
            cpSpaceAllocContactBuffer(space),
            stamp,
            0 as *mut cpContactBufferHeader,
        );
    } else if stamp.wrapping_sub((*(*head).next).stamp) > (*space).collisionPersistence {
        let mut tail: *mut cpContactBufferHeader = (*head).next;
        (*space).contactBuffersHead = cpContactBufferHeaderInit(tail, stamp, tail);
    } else {
        let mut buffer: *mut cpContactBufferHeader = cpContactBufferHeaderInit(
            cpSpaceAllocContactBuffer(space),
            stamp,
            head,
        );
        (*head).next = buffer;
        (*space).contactBuffersHead = (*head).next;
    };
}
pub unsafe extern "C" fn cpContactBufferGetArray(
    mut space: *mut cpSpace,
) -> *mut cpContact {
    if ((*(*space).contactBuffersHead).numContacts)
        .wrapping_add(2 as libc::c_int as libc::c_uint) as libc::c_ulong
        > ((32 as libc::c_int * 1024 as libc::c_int) as libc::c_ulong)
            .wrapping_sub(
                ::std::mem::size_of::<cpContactBufferHeader>() as libc::c_ulong,
            )
            .wrapping_div(::std::mem::size_of::<cpContact>() as libc::c_ulong)
    {
        cpSpacePushFreshContactBuffer(space);
    }
    let mut head: *mut cpContactBufferHeader = (*space).contactBuffersHead;
    return ((*(head as *mut cpContactBuffer)).contacts)
        .as_mut_ptr()
        .offset((*head).numContacts as isize);
}
pub unsafe extern "C" fn cpSpacePushContacts(
    mut space: *mut cpSpace,
    mut count: libc::c_int,
) {
    if !(count <= 2 as libc::c_int) {
        cpMessage(
            b"count <= CP_MAX_CONTACTS_PER_ARBITER\0" as *const u8
                as *const libc::c_char,
            b"../../src/cpSpaceStep.c\0" as *const u8 as *const libc::c_char,
            176 as libc::c_int,
            1 as libc::c_int,
            1 as libc::c_int,
            b"Internal Error: Contact buffer overflow!\0" as *const u8
                as *const libc::c_char,
        );
        abort();
    }
    (*(*space).contactBuffersHead)
        .numContacts = ((*(*space).contactBuffersHead).numContacts)
        .wrapping_add(count as libc::c_uint);
}
unsafe extern "C" fn cpSpacePopContacts(
    mut space: *mut cpSpace,
    mut count: libc::c_int,
) {
    (*(*space).contactBuffersHead)
        .numContacts = ((*(*space).contactBuffersHead).numContacts)
        .wrapping_sub(count as libc::c_uint);
}
unsafe extern "C" fn cpSpaceArbiterSetTrans(
    mut shapes: *mut *mut cpShape,
    mut space: *mut cpSpace,
) -> *mut libc::c_void {
    if (*(*space).pooledArbiters).num == 0 as libc::c_int {
        let mut count: libc::c_int = ((32 as libc::c_int * 1024 as libc::c_int)
            as libc::c_ulong)
            .wrapping_div(::std::mem::size_of::<cpArbiter>() as libc::c_ulong)
            as libc::c_int;
        if count == 0 {
            cpMessage(
                b"count\0" as *const u8 as *const libc::c_char,
                b"../../src/cpSpaceStep.c\0" as *const u8 as *const libc::c_char,
                193 as libc::c_int,
                1 as libc::c_int,
                1 as libc::c_int,
                b"Internal Error: Buffer size too small.\0" as *const u8
                    as *const libc::c_char,
            );
            abort();
        }
        let mut buffer: *mut cpArbiter = calloc(
            1 as libc::c_int as libc::c_ulong,
            (32 as libc::c_int * 1024 as libc::c_int) as libc::c_ulong,
        ) as *mut cpArbiter;
        cpArrayPush((*space).allocatedBuffers, buffer as *mut libc::c_void);
        let mut i: libc::c_int = 0 as libc::c_int;
        while i < count {
            cpArrayPush(
                (*space).pooledArbiters,
                buffer.offset(i as isize) as *mut libc::c_void,
            );
            i += 1;
            i;
        }
    }
    return cpArbiterInit(
        cpArrayPop((*space).pooledArbiters) as *mut cpArbiter,
        *shapes.offset(0 as libc::c_int as isize),
        *shapes.offset(1 as libc::c_int as isize),
    ) as *mut libc::c_void;
}
#[inline]
unsafe extern "C" fn QueryRejectConstraint(
    mut a: *mut cpBody,
    mut b: *mut cpBody,
) -> cpBool {
    let mut constraint: *mut cpConstraint = (*a).constraintList;
    while !constraint.is_null() {
        if (*constraint).collideBodies == 0
            && ((*constraint).a == a && (*constraint).b == b
                || (*constraint).a == b && (*constraint).b == a)
        {
            return 1 as libc::c_int as cpBool;
        }
        constraint = cpConstraintNext(constraint, a);
    }
    return 0 as libc::c_int as cpBool;
}
#[inline]
unsafe extern "C" fn QueryReject(mut a: *mut cpShape, mut b: *mut cpShape) -> cpBool {
    return (cpBBIntersects((*a).bb, (*b).bb) == 0 || (*a).body == (*b).body
        || cpShapeFilterReject((*a).filter, (*b).filter) as libc::c_int != 0
        || QueryRejectConstraint((*a).body, (*b).body) as libc::c_int != 0)
        as libc::c_int as cpBool;
}
pub unsafe extern "C" fn cpSpaceCollideShapes(
    mut a: *mut cpShape,
    mut b: *mut cpShape,
    mut id: cpCollisionID,
    mut space: *mut cpSpace,
) -> cpCollisionID {
    if QueryReject(a, b) != 0 {
        return id;
    }
    let mut info: cpCollisionInfo = cpCollide(a, b, id, cpContactBufferGetArray(space));
    if info.count == 0 as libc::c_int {
        return info.id;
    }
    cpSpacePushContacts(space, info.count);
    let mut shape_pair: [*const cpShape; 2] = [info.a, info.b];
    let mut arbHashID: cpHashValue = (info.a as cpHashValue)
        .wrapping_mul(3344921057 as libc::c_ulong)
        ^ (info.b as cpHashValue).wrapping_mul(3344921057 as libc::c_ulong);
    let mut arb: *mut cpArbiter = cpHashSetInsert(
        (*space).cachedArbiters,
        arbHashID,
        shape_pair.as_mut_ptr() as *const libc::c_void,
        ::std::mem::transmute::<
            Option::<
                unsafe extern "C" fn(
                    *mut *mut cpShape,
                    *mut cpSpace,
                ) -> *mut libc::c_void,
            >,
            cpHashSetTransFunc,
        >(
            Some(
                cpSpaceArbiterSetTrans
                    as unsafe extern "C" fn(
                        *mut *mut cpShape,
                        *mut cpSpace,
                    ) -> *mut libc::c_void,
            ),
        ),
        space as *mut libc::c_void,
    ) as *mut cpArbiter;
    cpArbiterUpdate(arb, &mut info, space);
    let mut handler: *mut cpCollisionHandler = (*arb).handler;
    if (*arb).state as libc::c_uint
        == CP_ARBITER_STATE_FIRST_COLLISION as libc::c_int as libc::c_uint
        && ((*handler).beginFunc).unwrap()(arb, space, (*handler).userData) == 0
    {
        cpArbiterIgnore(arb);
    }
    if (*arb).state as libc::c_uint
        != CP_ARBITER_STATE_IGNORE as libc::c_int as libc::c_uint
        && ((*handler).preSolveFunc).unwrap()(arb, space, (*handler).userData)
            as libc::c_int != 0
        && (*arb).state as libc::c_uint
            != CP_ARBITER_STATE_IGNORE as libc::c_int as libc::c_uint
        && !((*a).sensor as libc::c_int != 0 || (*b).sensor as libc::c_int != 0)
        && !((*(*a).body).m == ::std::f32::INFINITY as libc::c_double
            && (*(*b).body).m == ::std::f32::INFINITY as libc::c_double)
    {
        cpArrayPush((*space).arbiters, arb as *mut libc::c_void);
    } else {
        cpSpacePopContacts(space, info.count);
        (*arb).contacts = 0 as *mut cpContact;
        (*arb).count = 0 as libc::c_int;
        if (*arb).state as libc::c_uint
            != CP_ARBITER_STATE_IGNORE as libc::c_int as libc::c_uint
        {
            (*arb).state = CP_ARBITER_STATE_NORMAL;
        }
    }
    (*arb).stamp = (*space).stamp;
    return info.id;
}
pub unsafe extern "C" fn cpSpaceArbiterSetFilter(
    mut arb: *mut cpArbiter,
    mut space: *mut cpSpace,
) -> cpBool {
    let mut ticks: cpTimestamp = ((*space).stamp).wrapping_sub((*arb).stamp);
    let mut a: *mut cpBody = (*arb).body_a;
    let mut b: *mut cpBody = (*arb).body_b;
    if (cpBodyGetType(a) as libc::c_uint
        == CP_BODY_TYPE_STATIC as libc::c_int as libc::c_uint
        || cpBodyIsSleeping(a) as libc::c_int != 0)
        && (cpBodyGetType(b) as libc::c_uint
            == CP_BODY_TYPE_STATIC as libc::c_int as libc::c_uint
            || cpBodyIsSleeping(b) as libc::c_int != 0)
    {
        return 1 as libc::c_int as cpBool;
    }
    if ticks >= 1 as libc::c_int as libc::c_uint
        && (*arb).state as libc::c_uint
            != CP_ARBITER_STATE_CACHED as libc::c_int as libc::c_uint
    {
        (*arb).state = CP_ARBITER_STATE_CACHED;
        let mut handler: *mut cpCollisionHandler = (*arb).handler;
        ((*handler).separateFunc).unwrap()(arb, space, (*handler).userData);
    }
    if ticks >= (*space).collisionPersistence {
        (*arb).contacts = 0 as *mut cpContact;
        (*arb).count = 0 as libc::c_int;
        cpArrayPush((*space).pooledArbiters, arb as *mut libc::c_void);
        return 0 as libc::c_int as cpBool;
    }
    return 1 as libc::c_int as cpBool;
}
pub unsafe extern "C" fn cpShapeUpdateFunc(
    mut shape: *mut cpShape,
    mut unused: *mut libc::c_void,
) {
    cpShapeCacheBB(shape);
}
pub unsafe extern "C" fn cpSpaceStep(mut space: *mut cpSpace, mut dt: cpFloat) {
    if dt == 0.0f32 as libc::c_double {
        return;
    }
    (*space).stamp = ((*space).stamp).wrapping_add(1);
    (*space).stamp;
    let mut prev_dt: cpFloat = (*space).curr_dt;
    (*space).curr_dt = dt;
    let mut bodies: *mut cpArray = (*space).dynamicBodies;
    let mut constraints: *mut cpArray = (*space).constraints;
    let mut arbiters: *mut cpArray = (*space).arbiters;
    let mut i: libc::c_int = 0 as libc::c_int;
    while i < (*arbiters).num {
        let mut arb: *mut cpArbiter = *((*arbiters).arr).offset(i as isize)
            as *mut cpArbiter;
        (*arb).state = CP_ARBITER_STATE_NORMAL;
        if cpBodyIsSleeping((*arb).body_a) == 0 && cpBodyIsSleeping((*arb).body_b) == 0 {
            cpArbiterUnthread(arb);
        }
        i += 1;
        i;
    }
    (*arbiters).num = 0 as libc::c_int;
    cpSpaceLock(space);
    let mut i_0: libc::c_int = 0 as libc::c_int;
    while i_0 < (*bodies).num {
        let mut body: *mut cpBody = *((*bodies).arr).offset(i_0 as isize) as *mut cpBody;
        ((*body).position_func).unwrap()(body, dt);
        i_0 += 1;
        i_0;
    }
    cpSpacePushFreshContactBuffer(space);
    cpSpatialIndexEach(
        (*space).dynamicShapes,
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
    cpSpatialIndexReindexQuery(
        (*space).dynamicShapes,
        ::std::mem::transmute::<
            Option::<
                unsafe extern "C" fn(
                    *mut cpShape,
                    *mut cpShape,
                    cpCollisionID,
                    *mut cpSpace,
                ) -> cpCollisionID,
            >,
            cpSpatialIndexQueryFunc,
        >(
            Some(
                cpSpaceCollideShapes
                    as unsafe extern "C" fn(
                        *mut cpShape,
                        *mut cpShape,
                        cpCollisionID,
                        *mut cpSpace,
                    ) -> cpCollisionID,
            ),
        ),
        space as *mut libc::c_void,
    );
    cpSpaceUnlock(space, 0 as libc::c_int as cpBool);
    cpSpaceProcessComponents(space, dt);
    cpSpaceLock(space);
    cpHashSetFilter(
        (*space).cachedArbiters,
        ::std::mem::transmute::<
            Option::<unsafe extern "C" fn(*mut cpArbiter, *mut cpSpace) -> cpBool>,
            cpHashSetFilterFunc,
        >(
            Some(
                cpSpaceArbiterSetFilter
                    as unsafe extern "C" fn(*mut cpArbiter, *mut cpSpace) -> cpBool,
            ),
        ),
        space as *mut libc::c_void,
    );
    let mut slop: cpFloat = (*space).collisionSlop;
    let mut biasCoef: cpFloat = 1.0f32 as libc::c_double
        - pow((*space).collisionBias, dt);
    let mut i_1: libc::c_int = 0 as libc::c_int;
    while i_1 < (*arbiters).num {
        cpArbiterPreStep(
            *((*arbiters).arr).offset(i_1 as isize) as *mut cpArbiter,
            dt,
            slop,
            biasCoef,
        );
        i_1 += 1;
        i_1;
    }
    let mut i_2: libc::c_int = 0 as libc::c_int;
    while i_2 < (*constraints).num {
        let mut constraint: *mut cpConstraint = *((*constraints).arr)
            .offset(i_2 as isize) as *mut cpConstraint;
        let mut preSolve: cpConstraintPreSolveFunc = (*constraint).preSolve;
        if preSolve.is_some() {
            preSolve.unwrap()(constraint, space);
        }
        ((*(*constraint).klass).preStep).unwrap()(constraint, dt);
        i_2 += 1;
        i_2;
    }
    let mut damping: cpFloat = pow((*space).damping, dt);
    let mut gravity: cpVect = (*space).gravity;
    let mut i_3: libc::c_int = 0 as libc::c_int;
    while i_3 < (*bodies).num {
        let mut body_0: *mut cpBody = *((*bodies).arr).offset(i_3 as isize)
            as *mut cpBody;
        ((*body_0).velocity_func).unwrap()(body_0, gravity, damping, dt);
        i_3 += 1;
        i_3;
    }
    let mut dt_coef: cpFloat = if prev_dt == 0.0f32 as libc::c_double {
        0.0f32 as libc::c_double
    } else {
        dt / prev_dt
    };
    let mut i_4: libc::c_int = 0 as libc::c_int;
    while i_4 < (*arbiters).num {
        cpArbiterApplyCachedImpulse(
            *((*arbiters).arr).offset(i_4 as isize) as *mut cpArbiter,
            dt_coef,
        );
        i_4 += 1;
        i_4;
    }
    let mut i_5: libc::c_int = 0 as libc::c_int;
    while i_5 < (*constraints).num {
        let mut constraint_0: *mut cpConstraint = *((*constraints).arr)
            .offset(i_5 as isize) as *mut cpConstraint;
        ((*(*constraint_0).klass).applyCachedImpulse).unwrap()(constraint_0, dt_coef);
        i_5 += 1;
        i_5;
    }
    let mut i_6: libc::c_int = 0 as libc::c_int;
    while i_6 < (*space).iterations {
        let mut j: libc::c_int = 0 as libc::c_int;
        while j < (*arbiters).num {
            cpArbiterApplyImpulse(
                *((*arbiters).arr).offset(j as isize) as *mut cpArbiter,
            );
            j += 1;
            j;
        }
        let mut j_0: libc::c_int = 0 as libc::c_int;
        while j_0 < (*constraints).num {
            let mut constraint_1: *mut cpConstraint = *((*constraints).arr)
                .offset(j_0 as isize) as *mut cpConstraint;
            ((*(*constraint_1).klass).applyImpulse).unwrap()(constraint_1, dt);
            j_0 += 1;
            j_0;
        }
        i_6 += 1;
        i_6;
    }
    let mut i_7: libc::c_int = 0 as libc::c_int;
    while i_7 < (*constraints).num {
        let mut constraint_2: *mut cpConstraint = *((*constraints).arr)
            .offset(i_7 as isize) as *mut cpConstraint;
        let mut postSolve: cpConstraintPostSolveFunc = (*constraint_2).postSolve;
        if postSolve.is_some() {
            postSolve.unwrap()(constraint_2, space);
        }
        i_7 += 1;
        i_7;
    }
    let mut i_8: libc::c_int = 0 as libc::c_int;
    while i_8 < (*arbiters).num {
        let mut arb_0: *mut cpArbiter = *((*arbiters).arr).offset(i_8 as isize)
            as *mut cpArbiter;
        let mut handler: *mut cpCollisionHandler = (*arb_0).handler;
        ((*handler).postSolveFunc).unwrap()(arb_0, space, (*handler).userData);
        i_8 += 1;
        i_8;
    }
    cpSpaceUnlock(space, 1 as libc::c_int as cpBool);
}
