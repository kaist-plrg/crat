use ::libc;
extern "C" {
    pub type cpHashSet;
    pub type cpContactBufferHeader;
    fn memcpy(
        _: *mut libc::c_void,
        _: *const libc::c_void,
        _: libc::c_ulong,
    ) -> *mut libc::c_void;
    fn calloc(_: libc::c_ulong, _: libc::c_ulong) -> *mut libc::c_void;
    fn free(__ptr: *mut libc::c_void);
    fn abort() -> !;
    fn cpMessage(
        condition: *const libc::c_char,
        file: *const libc::c_char,
        line: libc::c_int,
        isError: libc::c_int,
        isHardError: libc::c_int,
        message: *const libc::c_char,
        _: ...
    );
    fn cpBodyGetType(body: *mut cpBody) -> cpBodyType;
    fn cpArrayDeleteObj(arr: *mut cpArray, obj: *mut libc::c_void);
    fn cpArrayPush(arr: *mut cpArray, object: *mut libc::c_void);
    fn cpHashSetInsert(
        set: *mut cpHashSet,
        hash: cpHashValue,
        ptr: *const libc::c_void,
        trans: cpHashSetTransFunc,
        data: *mut libc::c_void,
    ) -> *const libc::c_void;
    fn cpSpacePushContacts(space: *mut cpSpace, count: libc::c_int);
    fn cpContactBufferGetArray(space: *mut cpSpace) -> *mut cpContact;
    fn cpArrayContains(arr: *mut cpArray, ptr: *mut libc::c_void) -> cpBool;
    fn cpBodyIsSleeping(body: *const cpBody) -> cpBool;
    fn cpHashSetRemove(
        set: *mut cpHashSet,
        hash: cpHashValue,
        ptr: *const libc::c_void,
    ) -> *const libc::c_void;
    fn cpShapeCacheBB(shape: *mut cpShape) -> cpBB;
    fn cpSpaceGetSleepTimeThreshold(space: *const cpSpace) -> cpFloat;
    fn cpSpaceIsLocked(space: *mut cpSpace) -> cpBool;
    fn cpBodyKineticEnergy(body: *const cpBody) -> cpFloat;
}
pub type size_t = libc::c_ulong;
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
pub type cpBodyType = libc::c_uint;
pub const CP_BODY_TYPE_STATIC: cpBodyType = 2;
pub const CP_BODY_TYPE_KINEMATIC: cpBodyType = 1;
pub const CP_BODY_TYPE_DYNAMIC: cpBodyType = 0;
pub type cpHashSetTransFunc = Option::<
    unsafe extern "C" fn(*const libc::c_void, *mut libc::c_void) -> *mut libc::c_void,
>;
#[inline]
unsafe extern "C" fn cpvdot(v1: cpVect, v2: cpVect) -> cpFloat {
    return v1.x * v2.x + v1.y * v2.y;
}
#[inline]
unsafe extern "C" fn cpvlengthsq(v: cpVect) -> cpFloat {
    return cpvdot(v, v);
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
unsafe extern "C" fn cpSpatialIndexRemove(
    mut index: *mut cpSpatialIndex,
    mut obj: *mut libc::c_void,
    mut hashid: cpHashValue,
) {
    ((*(*index).klass).remove).unwrap()(index, obj, hashid);
}
#[inline]
unsafe extern "C" fn cpArbiterNext(
    mut node: *mut cpArbiter,
    mut body: *mut cpBody,
) -> *mut cpArbiter {
    return if (*node).body_a == body {
        (*node).thread_a.next
    } else {
        (*node).thread_b.next
    };
}
#[inline]
unsafe extern "C" fn cpConstraintNext(
    mut node: *mut cpConstraint,
    mut body: *mut cpBody,
) -> *mut cpConstraint {
    return if (*node).a == body { (*node).next_a } else { (*node).next_b };
}
#[inline]
unsafe extern "C" fn cpSpaceUncacheArbiter(
    mut space: *mut cpSpace,
    mut arb: *mut cpArbiter,
) {
    let mut a: *const cpShape = (*arb).a;
    let mut b: *const cpShape = (*arb).b;
    let mut shape_pair: [*const cpShape; 2] = [a, b];
    let mut arbHashID: cpHashValue = (a as cpHashValue)
        .wrapping_mul(3344921057 as libc::c_ulong)
        ^ (b as cpHashValue).wrapping_mul(3344921057 as libc::c_ulong);
    cpHashSetRemove(
        (*space).cachedArbiters,
        arbHashID,
        shape_pair.as_mut_ptr() as *const libc::c_void,
    );
    cpArrayDeleteObj((*space).arbiters, arb as *mut libc::c_void);
}
#[inline]
unsafe extern "C" fn cpArbiterThreadForBody(
    mut arb: *mut cpArbiter,
    mut body: *mut cpBody,
) -> *mut cpArbiterThread {
    return if (*arb).body_a == body {
        &mut (*arb).thread_a
    } else {
        &mut (*arb).thread_b
    };
}
pub unsafe extern "C" fn cpSpaceActivateBody(
    mut space: *mut cpSpace,
    mut body: *mut cpBody,
) {
    if !(cpBodyGetType(body) as libc::c_uint
        == CP_BODY_TYPE_DYNAMIC as libc::c_int as libc::c_uint)
    {
        cpMessage(
            b"cpBodyGetType(body) == CP_BODY_TYPE_DYNAMIC\0" as *const u8
                as *const libc::c_char,
            b"../../src/cpSpaceComponent.c\0" as *const u8 as *const libc::c_char,
            31 as libc::c_int,
            1 as libc::c_int,
            1 as libc::c_int,
            b"Internal error: Attempting to activate a non-dynamic body.\0" as *const u8
                as *const libc::c_char,
        );
        abort();
    }
    if (*space).locked != 0 {
        if cpArrayContains((*space).rousedBodies, body as *mut libc::c_void) == 0 {
            cpArrayPush((*space).rousedBodies, body as *mut libc::c_void);
        }
    } else {
        cpArrayPush((*space).dynamicBodies, body as *mut libc::c_void);
        let mut shape: *mut cpShape = (*body).shapeList;
        while !shape.is_null() {
            cpSpatialIndexRemove(
                (*space).staticShapes,
                shape as *mut libc::c_void,
                (*shape).hashid,
            );
            cpSpatialIndexInsert(
                (*space).dynamicShapes,
                shape as *mut libc::c_void,
                (*shape).hashid,
            );
            shape = (*shape).next;
        }
        let mut arb: *mut cpArbiter = (*body).arbiterList;
        while !arb.is_null() {
            let mut bodyA: *mut cpBody = (*arb).body_a;
            if body == bodyA
                || cpBodyGetType(bodyA) as libc::c_uint
                    == CP_BODY_TYPE_STATIC as libc::c_int as libc::c_uint
            {
                let mut numContacts: libc::c_int = (*arb).count;
                let mut contacts: *mut cpContact = (*arb).contacts;
                (*arb).contacts = cpContactBufferGetArray(space);
                memcpy(
                    (*arb).contacts as *mut libc::c_void,
                    contacts as *const libc::c_void,
                    (numContacts as libc::c_ulong)
                        .wrapping_mul(
                            ::std::mem::size_of::<cpContact>() as libc::c_ulong,
                        ),
                );
                cpSpacePushContacts(space, numContacts);
                let mut a: *const cpShape = (*arb).a;
                let mut b: *const cpShape = (*arb).b;
                let mut shape_pair: [*const cpShape; 2] = [a, b];
                let mut arbHashID: cpHashValue = (a as cpHashValue)
                    .wrapping_mul(3344921057 as libc::c_ulong)
                    ^ (b as cpHashValue).wrapping_mul(3344921057 as libc::c_ulong);
                cpHashSetInsert(
                    (*space).cachedArbiters,
                    arbHashID,
                    shape_pair.as_mut_ptr() as *const libc::c_void,
                    None,
                    arb as *mut libc::c_void,
                );
                (*arb).stamp = (*space).stamp;
                cpArrayPush((*space).arbiters, arb as *mut libc::c_void);
                free(contacts as *mut libc::c_void);
            }
            arb = cpArbiterNext(arb, body);
        }
        let mut constraint: *mut cpConstraint = (*body).constraintList;
        while !constraint.is_null() {
            let mut bodyA_0: *mut cpBody = (*constraint).a;
            if body == bodyA_0
                || cpBodyGetType(bodyA_0) as libc::c_uint
                    == CP_BODY_TYPE_STATIC as libc::c_int as libc::c_uint
            {
                cpArrayPush((*space).constraints, constraint as *mut libc::c_void);
            }
            constraint = cpConstraintNext(constraint, body);
        }
    };
}
unsafe extern "C" fn cpSpaceDeactivateBody(
    mut space: *mut cpSpace,
    mut body: *mut cpBody,
) {
    if !(cpBodyGetType(body) as libc::c_uint
        == CP_BODY_TYPE_DYNAMIC as libc::c_int as libc::c_uint)
    {
        cpMessage(
            b"cpBodyGetType(body) == CP_BODY_TYPE_DYNAMIC\0" as *const u8
                as *const libc::c_char,
            b"../../src/cpSpaceComponent.c\0" as *const u8 as *const libc::c_char,
            85 as libc::c_int,
            1 as libc::c_int,
            1 as libc::c_int,
            b"Internal error: Attempting to deactivate a non-dynamic body.\0"
                as *const u8 as *const libc::c_char,
        );
        abort();
    }
    cpArrayDeleteObj((*space).dynamicBodies, body as *mut libc::c_void);
    let mut shape: *mut cpShape = (*body).shapeList;
    while !shape.is_null() {
        cpSpatialIndexRemove(
            (*space).dynamicShapes,
            shape as *mut libc::c_void,
            (*shape).hashid,
        );
        cpSpatialIndexInsert(
            (*space).staticShapes,
            shape as *mut libc::c_void,
            (*shape).hashid,
        );
        shape = (*shape).next;
    }
    let mut arb: *mut cpArbiter = (*body).arbiterList;
    while !arb.is_null() {
        let mut bodyA: *mut cpBody = (*arb).body_a;
        if body == bodyA
            || cpBodyGetType(bodyA) as libc::c_uint
                == CP_BODY_TYPE_STATIC as libc::c_int as libc::c_uint
        {
            cpSpaceUncacheArbiter(space, arb);
            let mut bytes: size_t = ((*arb).count as libc::c_ulong)
                .wrapping_mul(::std::mem::size_of::<cpContact>() as libc::c_ulong);
            let mut contacts: *mut cpContact = calloc(
                1 as libc::c_int as libc::c_ulong,
                bytes,
            ) as *mut cpContact;
            memcpy(
                contacts as *mut libc::c_void,
                (*arb).contacts as *const libc::c_void,
                bytes,
            );
            (*arb).contacts = contacts;
        }
        arb = cpArbiterNext(arb, body);
    }
    let mut constraint: *mut cpConstraint = (*body).constraintList;
    while !constraint.is_null() {
        let mut bodyA_0: *mut cpBody = (*constraint).a;
        if body == bodyA_0
            || cpBodyGetType(bodyA_0) as libc::c_uint
                == CP_BODY_TYPE_STATIC as libc::c_int as libc::c_uint
        {
            cpArrayDeleteObj((*space).constraints, constraint as *mut libc::c_void);
        }
        constraint = cpConstraintNext(constraint, body);
    }
}
#[inline]
unsafe extern "C" fn ComponentRoot(mut body: *mut cpBody) -> *mut cpBody {
    return if !body.is_null() { (*body).sleeping.root } else { 0 as *mut cpBody };
}
pub unsafe extern "C" fn cpBodyActivate(mut body: *mut cpBody) {
    if !body.is_null()
        && cpBodyGetType(body) as libc::c_uint
            == CP_BODY_TYPE_DYNAMIC as libc::c_int as libc::c_uint
    {
        (*body).sleeping.idleTime = 0.0f32 as cpFloat;
        let mut root: *mut cpBody = ComponentRoot(body);
        if !root.is_null() && cpBodyIsSleeping(root) as libc::c_int != 0 {
            let mut space: *mut cpSpace = (*root).space;
            let mut body_0: *mut cpBody = root;
            while !body_0.is_null() {
                let mut next: *mut cpBody = (*body_0).sleeping.next;
                (*body_0).sleeping.idleTime = 0.0f32 as cpFloat;
                (*body_0).sleeping.root = 0 as *mut cpBody;
                (*body_0).sleeping.next = 0 as *mut cpBody;
                cpSpaceActivateBody(space, body_0);
                body_0 = next;
            }
            cpArrayDeleteObj((*space).sleepingComponents, root as *mut libc::c_void);
        }
        let mut arb: *mut cpArbiter = (*body).arbiterList;
        while !arb.is_null() {
            let mut other: *mut cpBody = if (*arb).body_a == body {
                (*arb).body_b
            } else {
                (*arb).body_a
            };
            if cpBodyGetType(other) as libc::c_uint
                != CP_BODY_TYPE_STATIC as libc::c_int as libc::c_uint
            {
                (*other).sleeping.idleTime = 0.0f32 as cpFloat;
            }
            arb = cpArbiterNext(arb, body);
        }
    }
}
pub unsafe extern "C" fn cpBodyActivateStatic(
    mut body: *mut cpBody,
    mut filter: *mut cpShape,
) {
    if !(cpBodyGetType(body) as libc::c_uint
        == CP_BODY_TYPE_STATIC as libc::c_int as libc::c_uint)
    {
        cpMessage(
            b"cpBodyGetType(body) == CP_BODY_TYPE_STATIC\0" as *const u8
                as *const libc::c_char,
            b"../../src/cpSpaceComponent.c\0" as *const u8 as *const libc::c_char,
            158 as libc::c_int,
            1 as libc::c_int,
            1 as libc::c_int,
            b"cpBodyActivateStatic() called on a non-static body.\0" as *const u8
                as *const libc::c_char,
        );
        abort();
    }
    let mut arb: *mut cpArbiter = (*body).arbiterList;
    while !arb.is_null() {
        if filter.is_null() || filter == (*arb).a as *mut cpShape
            || filter == (*arb).b as *mut cpShape
        {
            cpBodyActivate(
                if (*arb).body_a == body { (*arb).body_b } else { (*arb).body_a },
            );
        }
        arb = cpArbiterNext(arb, body);
    }
}
#[inline]
unsafe extern "C" fn cpBodyPushArbiter(mut body: *mut cpBody, mut arb: *mut cpArbiter) {
    let mut next: *mut cpArbiter = (*body).arbiterList;
    let ref mut fresh0 = (*cpArbiterThreadForBody(arb, body)).next;
    *fresh0 = next;
    if !next.is_null() {
        let ref mut fresh1 = (*cpArbiterThreadForBody(next, body)).prev;
        *fresh1 = arb;
    }
    (*body).arbiterList = arb;
}
#[inline]
unsafe extern "C" fn ComponentAdd(mut root: *mut cpBody, mut body: *mut cpBody) {
    (*body).sleeping.root = root;
    if body != root {
        (*body).sleeping.next = (*root).sleeping.next;
        (*root).sleeping.next = body;
    }
}
#[inline]
unsafe extern "C" fn FloodFillComponent(mut root: *mut cpBody, mut body: *mut cpBody) {
    if cpBodyGetType(body) as libc::c_uint
        == CP_BODY_TYPE_DYNAMIC as libc::c_int as libc::c_uint
    {
        let mut other_root: *mut cpBody = ComponentRoot(body);
        if other_root.is_null() {
            ComponentAdd(root, body);
            let mut arb: *mut cpArbiter = (*body).arbiterList;
            while !arb.is_null() {
                FloodFillComponent(
                    root,
                    if body == (*arb).body_a { (*arb).body_b } else { (*arb).body_a },
                );
                arb = cpArbiterNext(arb, body);
            }
            let mut constraint: *mut cpConstraint = (*body).constraintList;
            while !constraint.is_null() {
                FloodFillComponent(
                    root,
                    if body == (*constraint).a {
                        (*constraint).b
                    } else {
                        (*constraint).a
                    },
                );
                constraint = cpConstraintNext(constraint, body);
            }
        }
    }
}
#[inline]
unsafe extern "C" fn ComponentActive(
    mut root: *mut cpBody,
    mut threshold: cpFloat,
) -> cpBool {
    let mut body: *mut cpBody = root;
    while !body.is_null() {
        if (*body).sleeping.idleTime < threshold {
            return 1 as libc::c_int as cpBool;
        }
        body = (*body).sleeping.next;
    }
    return 0 as libc::c_int as cpBool;
}
pub unsafe extern "C" fn cpSpaceProcessComponents(
    mut space: *mut cpSpace,
    mut dt: cpFloat,
) {
    let mut sleep: cpBool = ((*space).sleepTimeThreshold
        != ::std::f32::INFINITY as libc::c_double) as libc::c_int as cpBool;
    let mut bodies: *mut cpArray = (*space).dynamicBodies;
    if sleep != 0 {
        let mut dv: cpFloat = (*space).idleSpeedThreshold;
        let mut dvsq: cpFloat = if dv != 0. {
            dv * dv
        } else {
            cpvlengthsq((*space).gravity) * dt * dt
        };
        let mut i: libc::c_int = 0 as libc::c_int;
        while i < (*bodies).num {
            let mut body: *mut cpBody = *((*bodies).arr).offset(i as isize)
                as *mut cpBody;
            if !(cpBodyGetType(body) as libc::c_uint
                != CP_BODY_TYPE_DYNAMIC as libc::c_int as libc::c_uint)
            {
                let mut keThreshold: cpFloat = if dvsq != 0. {
                    (*body).m * dvsq
                } else {
                    0.0f32 as libc::c_double
                };
                (*body)
                    .sleeping
                    .idleTime = if cpBodyKineticEnergy(body) > keThreshold {
                    0.0f32 as libc::c_double
                } else {
                    (*body).sleeping.idleTime + dt
                };
            }
            i += 1;
            i;
        }
    }
    let mut arbiters: *mut cpArray = (*space).arbiters;
    let mut i_0: libc::c_int = 0 as libc::c_int;
    let mut count: libc::c_int = (*arbiters).num;
    while i_0 < count {
        let mut arb: *mut cpArbiter = *((*arbiters).arr).offset(i_0 as isize)
            as *mut cpArbiter;
        let mut a: *mut cpBody = (*arb).body_a;
        let mut b: *mut cpBody = (*arb).body_b;
        if sleep != 0 {
            if cpBodyGetType(b) as libc::c_uint
                == CP_BODY_TYPE_KINEMATIC as libc::c_int as libc::c_uint
                || cpBodyIsSleeping(a) as libc::c_int != 0
            {
                cpBodyActivate(a);
            }
            if cpBodyGetType(a) as libc::c_uint
                == CP_BODY_TYPE_KINEMATIC as libc::c_int as libc::c_uint
                || cpBodyIsSleeping(b) as libc::c_int != 0
            {
                cpBodyActivate(b);
            }
        }
        cpBodyPushArbiter(a, arb);
        cpBodyPushArbiter(b, arb);
        i_0 += 1;
        i_0;
    }
    if sleep != 0 {
        let mut constraints: *mut cpArray = (*space).constraints;
        let mut i_1: libc::c_int = 0 as libc::c_int;
        while i_1 < (*constraints).num {
            let mut constraint: *mut cpConstraint = *((*constraints).arr)
                .offset(i_1 as isize) as *mut cpConstraint;
            let mut a_0: *mut cpBody = (*constraint).a;
            let mut b_0: *mut cpBody = (*constraint).b;
            if cpBodyGetType(b_0) as libc::c_uint
                == CP_BODY_TYPE_KINEMATIC as libc::c_int as libc::c_uint
            {
                cpBodyActivate(a_0);
            }
            if cpBodyGetType(a_0) as libc::c_uint
                == CP_BODY_TYPE_KINEMATIC as libc::c_int as libc::c_uint
            {
                cpBodyActivate(b_0);
            }
            i_1 += 1;
            i_1;
        }
        let mut i_2: libc::c_int = 0 as libc::c_int;
        while i_2 < (*bodies).num {
            let mut body_0: *mut cpBody = *((*bodies).arr).offset(i_2 as isize)
                as *mut cpBody;
            if (ComponentRoot(body_0)).is_null() {
                FloodFillComponent(body_0, body_0);
                if ComponentActive(body_0, (*space).sleepTimeThreshold) == 0 {
                    cpArrayPush(
                        (*space).sleepingComponents,
                        body_0 as *mut libc::c_void,
                    );
                    let mut other: *mut cpBody = body_0;
                    while !other.is_null() {
                        cpSpaceDeactivateBody(space, other);
                        other = (*other).sleeping.next;
                    }
                    continue;
                }
            }
            i_2 += 1;
            i_2;
            (*body_0).sleeping.root = 0 as *mut cpBody;
            (*body_0).sleeping.next = 0 as *mut cpBody;
        }
    }
}
pub unsafe extern "C" fn cpBodySleep(mut body: *mut cpBody) {
    cpBodySleepWithGroup(body, 0 as *mut cpBody);
}
pub unsafe extern "C" fn cpBodySleepWithGroup(
    mut body: *mut cpBody,
    mut group: *mut cpBody,
) {
    if !(cpBodyGetType(body) as libc::c_uint
        == CP_BODY_TYPE_DYNAMIC as libc::c_int as libc::c_uint)
    {
        cpMessage(
            b"cpBodyGetType(body) == CP_BODY_TYPE_DYNAMIC\0" as *const u8
                as *const libc::c_char,
            b"../../src/cpSpaceComponent.c\0" as *const u8 as *const libc::c_char,
            317 as libc::c_int,
            1 as libc::c_int,
            1 as libc::c_int,
            b"Non-dynamic bodies cannot be put to sleep.\0" as *const u8
                as *const libc::c_char,
        );
        abort();
    }
    let mut space: *mut cpSpace = (*body).space;
    if cpSpaceIsLocked(space) != 0 {
        cpMessage(
            b"!cpSpaceIsLocked(space)\0" as *const u8 as *const libc::c_char,
            b"../../src/cpSpaceComponent.c\0" as *const u8 as *const libc::c_char,
            320 as libc::c_int,
            1 as libc::c_int,
            1 as libc::c_int,
            b"Bodies cannot be put to sleep during a query or a call to cpSpaceStep(). Put these calls into a post-step callback.\0"
                as *const u8 as *const libc::c_char,
        );
        abort();
    }
    if !(cpSpaceGetSleepTimeThreshold(space) < ::std::f32::INFINITY as libc::c_double) {
        cpMessage(
            b"cpSpaceGetSleepTimeThreshold(space) < INFINITY\0" as *const u8
                as *const libc::c_char,
            b"../../src/cpSpaceComponent.c\0" as *const u8 as *const libc::c_char,
            321 as libc::c_int,
            1 as libc::c_int,
            1 as libc::c_int,
            b"Sleeping is not enabled on the space. You cannot sleep a body without setting a sleep time threshold on the space.\0"
                as *const u8 as *const libc::c_char,
        );
        abort();
    }
    if !(group.is_null() || cpBodyIsSleeping(group) as libc::c_int != 0) {
        cpMessage(
            b"group == NULL || cpBodyIsSleeping(group)\0" as *const u8
                as *const libc::c_char,
            b"../../src/cpSpaceComponent.c\0" as *const u8 as *const libc::c_char,
            322 as libc::c_int,
            1 as libc::c_int,
            1 as libc::c_int,
            b"Cannot use a non-sleeping body as a group identifier.\0" as *const u8
                as *const libc::c_char,
        );
        abort();
    }
    if cpBodyIsSleeping(body) != 0 {
        if !(ComponentRoot(body) == ComponentRoot(group)) {
            cpMessage(
                b"ComponentRoot(body) == ComponentRoot(group)\0" as *const u8
                    as *const libc::c_char,
                b"../../src/cpSpaceComponent.c\0" as *const u8 as *const libc::c_char,
                325 as libc::c_int,
                1 as libc::c_int,
                1 as libc::c_int,
                b"The body is already sleeping and it's group cannot be reassigned.\0"
                    as *const u8 as *const libc::c_char,
            );
            abort();
        }
        return;
    }
    let mut shape: *mut cpShape = (*body).shapeList;
    while !shape.is_null() {
        cpShapeCacheBB(shape);
        shape = (*shape).next;
    }
    cpSpaceDeactivateBody(space, body);
    if !group.is_null() {
        let mut root: *mut cpBody = ComponentRoot(group);
        (*body).sleeping.root = root;
        (*body).sleeping.next = (*root).sleeping.next;
        (*body).sleeping.idleTime = 0.0f32 as cpFloat;
        (*root).sleeping.next = body;
    } else {
        (*body).sleeping.root = body;
        (*body).sleeping.next = 0 as *mut cpBody;
        (*body).sleeping.idleTime = 0.0f32 as cpFloat;
        cpArrayPush((*space).sleepingComponents, body as *mut libc::c_void);
    }
    cpArrayDeleteObj((*space).dynamicBodies, body as *mut libc::c_void);
}
