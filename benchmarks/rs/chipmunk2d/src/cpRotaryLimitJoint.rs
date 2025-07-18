use ::libc;
extern "C" {
    pub type cpHashSet;
    pub type cpContactBufferHeader;
    fn pow(_: libc::c_double, _: libc::c_double) -> libc::c_double;
    fn abort() -> !;
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
    fn cpBodyActivate(body: *mut cpBody);
    fn cpConstraintInit(
        constraint: *mut cpConstraint,
        klass_0: *const cpConstraintClass,
        a: *mut cpBody,
        b: *mut cpBody,
    );
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct cpRotaryLimitJoint {
    pub constraint: cpConstraint,
    pub min: cpFloat,
    pub max: cpFloat,
    pub iSum: cpFloat,
    pub bias: cpFloat,
    pub jAcc: cpFloat,
}
#[inline]
unsafe extern "C" fn cpfmax(mut a: cpFloat, mut b: cpFloat) -> cpFloat {
    return if a > b { a } else { b };
}
#[inline]
unsafe extern "C" fn cpfmin(mut a: cpFloat, mut b: cpFloat) -> cpFloat {
    return if a < b { a } else { b };
}
#[inline]
unsafe extern "C" fn cpfabs(mut f: cpFloat) -> cpFloat {
    return if f < 0 as libc::c_int as libc::c_double { -f } else { f };
}
#[inline]
unsafe extern "C" fn cpfclamp(
    mut f: cpFloat,
    mut min: cpFloat,
    mut max: cpFloat,
) -> cpFloat {
    return cpfmin(cpfmax(f, min), max);
}
#[inline]
unsafe extern "C" fn bias_coef(mut errorBias: cpFloat, mut dt: cpFloat) -> cpFloat {
    return 1.0f32 as libc::c_double - pow(errorBias, dt);
}
#[inline]
unsafe extern "C" fn cpConstraintActivateBodies(mut constraint: *mut cpConstraint) {
    let mut a: *mut cpBody = (*constraint).a;
    cpBodyActivate(a);
    let mut b: *mut cpBody = (*constraint).b;
    cpBodyActivate(b);
}
unsafe extern "C" fn preStep(mut joint: *mut cpRotaryLimitJoint, mut dt: cpFloat) {
    let mut a: *mut cpBody = (*joint).constraint.a;
    let mut b: *mut cpBody = (*joint).constraint.b;
    let mut dist: cpFloat = (*b).a - (*a).a;
    let mut pdist: cpFloat = 0.0f32 as cpFloat;
    if dist > (*joint).max {
        pdist = (*joint).max - dist;
    } else if dist < (*joint).min {
        pdist = (*joint).min - dist;
    }
    (*joint).iSum = 1.0f32 as libc::c_double / ((*a).i_inv + (*b).i_inv);
    let mut maxBias: cpFloat = (*joint).constraint.maxBias;
    (*joint)
        .bias = cpfclamp(
        -bias_coef((*joint).constraint.errorBias, dt) * pdist / dt,
        -maxBias,
        maxBias,
    );
    if (*joint).bias == 0. {
        (*joint).jAcc = 0.0f32 as cpFloat;
    }
}
unsafe extern "C" fn applyCachedImpulse(
    mut joint: *mut cpRotaryLimitJoint,
    mut dt_coef: cpFloat,
) {
    let mut a: *mut cpBody = (*joint).constraint.a;
    let mut b: *mut cpBody = (*joint).constraint.b;
    let mut j: cpFloat = (*joint).jAcc * dt_coef;
    (*a).w -= j * (*a).i_inv;
    (*b).w += j * (*b).i_inv;
}
unsafe extern "C" fn applyImpulse(mut joint: *mut cpRotaryLimitJoint, mut dt: cpFloat) {
    if (*joint).bias == 0. {
        return;
    }
    let mut a: *mut cpBody = (*joint).constraint.a;
    let mut b: *mut cpBody = (*joint).constraint.b;
    let mut wr: cpFloat = (*b).w - (*a).w;
    let mut jMax: cpFloat = (*joint).constraint.maxForce * dt;
    let mut j: cpFloat = -((*joint).bias + wr) * (*joint).iSum;
    let mut jOld: cpFloat = (*joint).jAcc;
    if (*joint).bias < 0.0f32 as libc::c_double {
        (*joint).jAcc = cpfclamp(jOld + j, 0.0f32 as cpFloat, jMax);
    } else {
        (*joint).jAcc = cpfclamp(jOld + j, -jMax, 0.0f32 as cpFloat);
    }
    j = (*joint).jAcc - jOld;
    (*a).w -= j * (*a).i_inv;
    (*b).w += j * (*b).i_inv;
}
unsafe extern "C" fn getImpulse(mut joint: *mut cpRotaryLimitJoint) -> cpFloat {
    return cpfabs((*joint).jAcc);
}
static mut klass: cpConstraintClass = unsafe {
    {
        let mut init = cpConstraintClass {
            preStep: ::std::mem::transmute::<
                Option::<unsafe extern "C" fn(*mut cpRotaryLimitJoint, cpFloat) -> ()>,
                cpConstraintPreStepImpl,
            >(
                Some(
                    preStep
                        as unsafe extern "C" fn(*mut cpRotaryLimitJoint, cpFloat) -> (),
                ),
            ),
            applyCachedImpulse: ::std::mem::transmute::<
                Option::<unsafe extern "C" fn(*mut cpRotaryLimitJoint, cpFloat) -> ()>,
                cpConstraintApplyCachedImpulseImpl,
            >(
                Some(
                    applyCachedImpulse
                        as unsafe extern "C" fn(*mut cpRotaryLimitJoint, cpFloat) -> (),
                ),
            ),
            applyImpulse: ::std::mem::transmute::<
                Option::<unsafe extern "C" fn(*mut cpRotaryLimitJoint, cpFloat) -> ()>,
                cpConstraintApplyImpulseImpl,
            >(
                Some(
                    applyImpulse
                        as unsafe extern "C" fn(*mut cpRotaryLimitJoint, cpFloat) -> (),
                ),
            ),
            getImpulse: ::std::mem::transmute::<
                Option::<unsafe extern "C" fn(*mut cpRotaryLimitJoint) -> cpFloat>,
                cpConstraintGetImpulseImpl,
            >(
                Some(
                    getImpulse
                        as unsafe extern "C" fn(*mut cpRotaryLimitJoint) -> cpFloat,
                ),
            ),
        };
        init
    }
};
pub unsafe extern "C" fn cpRotaryLimitJointAlloc() -> *mut cpRotaryLimitJoint {
    return calloc(
        1 as libc::c_int as libc::c_ulong,
        ::std::mem::size_of::<cpRotaryLimitJoint>() as libc::c_ulong,
    ) as *mut cpRotaryLimitJoint;
}
pub unsafe extern "C" fn cpRotaryLimitJointInit(
    mut joint: *mut cpRotaryLimitJoint,
    mut a: *mut cpBody,
    mut b: *mut cpBody,
    mut min: cpFloat,
    mut max: cpFloat,
) -> *mut cpRotaryLimitJoint {
    cpConstraintInit(joint as *mut cpConstraint, &klass, a, b);
    (*joint).min = min;
    (*joint).max = max;
    (*joint).jAcc = 0.0f32 as cpFloat;
    return joint;
}
pub unsafe extern "C" fn cpRotaryLimitJointNew(
    mut a: *mut cpBody,
    mut b: *mut cpBody,
    mut min: cpFloat,
    mut max: cpFloat,
) -> *mut cpConstraint {
    return cpRotaryLimitJointInit(cpRotaryLimitJointAlloc(), a, b, min, max)
        as *mut cpConstraint;
}
pub unsafe extern "C" fn cpConstraintIsRotaryLimitJoint(
    mut constraint: *const cpConstraint,
) -> cpBool {
    return ((*constraint).klass == &klass as *const cpConstraintClass) as libc::c_int
        as cpBool;
}
pub unsafe extern "C" fn cpRotaryLimitJointGetMin(
    mut constraint: *const cpConstraint,
) -> cpFloat {
    if cpConstraintIsRotaryLimitJoint(constraint) == 0 {
        cpMessage(
            b"cpConstraintIsRotaryLimitJoint(constraint)\0" as *const u8
                as *const libc::c_char,
            b"../../src/cpRotaryLimitJoint.c\0" as *const u8 as *const libc::c_char,
            135 as libc::c_int,
            1 as libc::c_int,
            1 as libc::c_int,
            b"Constraint is not a rotary limit joint.\0" as *const u8
                as *const libc::c_char,
        );
        abort();
    }
    return (*(constraint as *mut cpRotaryLimitJoint)).min;
}
pub unsafe extern "C" fn cpRotaryLimitJointSetMin(
    mut constraint: *mut cpConstraint,
    mut min: cpFloat,
) {
    if cpConstraintIsRotaryLimitJoint(constraint) == 0 {
        cpMessage(
            b"cpConstraintIsRotaryLimitJoint(constraint)\0" as *const u8
                as *const libc::c_char,
            b"../../src/cpRotaryLimitJoint.c\0" as *const u8 as *const libc::c_char,
            142 as libc::c_int,
            1 as libc::c_int,
            1 as libc::c_int,
            b"Constraint is not a rotary limit joint.\0" as *const u8
                as *const libc::c_char,
        );
        abort();
    }
    cpConstraintActivateBodies(constraint);
    (*(constraint as *mut cpRotaryLimitJoint)).min = min;
}
pub unsafe extern "C" fn cpRotaryLimitJointGetMax(
    mut constraint: *const cpConstraint,
) -> cpFloat {
    if cpConstraintIsRotaryLimitJoint(constraint) == 0 {
        cpMessage(
            b"cpConstraintIsRotaryLimitJoint(constraint)\0" as *const u8
                as *const libc::c_char,
            b"../../src/cpRotaryLimitJoint.c\0" as *const u8 as *const libc::c_char,
            150 as libc::c_int,
            1 as libc::c_int,
            1 as libc::c_int,
            b"Constraint is not a rotary limit joint.\0" as *const u8
                as *const libc::c_char,
        );
        abort();
    }
    return (*(constraint as *mut cpRotaryLimitJoint)).max;
}
pub unsafe extern "C" fn cpRotaryLimitJointSetMax(
    mut constraint: *mut cpConstraint,
    mut max: cpFloat,
) {
    if cpConstraintIsRotaryLimitJoint(constraint) == 0 {
        cpMessage(
            b"cpConstraintIsRotaryLimitJoint(constraint)\0" as *const u8
                as *const libc::c_char,
            b"../../src/cpRotaryLimitJoint.c\0" as *const u8 as *const libc::c_char,
            157 as libc::c_int,
            1 as libc::c_int,
            1 as libc::c_int,
            b"Constraint is not a rotary limit joint.\0" as *const u8
                as *const libc::c_char,
        );
        abort();
    }
    cpConstraintActivateBodies(constraint);
    (*(constraint as *mut cpRotaryLimitJoint)).max = max;
}
