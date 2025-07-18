use ::libc;
extern "C" {
    pub type cpHashSet;
    pub type cpContactBufferHeader;
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
    fn cpHashSetFind(
        set: *mut cpHashSet,
        hash: cpHashValue,
        ptr: *const libc::c_void,
    ) -> *const libc::c_void;
    static mut cpCollisionHandlerDoNothing: cpCollisionHandler;
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
pub struct cpContactPointSet {
    pub count: libc::c_int,
    pub normal: cpVect,
    pub points: [C2RustUnnamed_0; 2],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_0 {
    pub pointA: cpVect,
    pub pointB: cpVect,
    pub distance: cpFloat,
}
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
#[inline]
unsafe extern "C" fn cpfmax(mut a: cpFloat, mut b: cpFloat) -> cpFloat {
    return if a > b { a } else { b };
}
#[inline]
unsafe extern "C" fn cpfmin(mut a: cpFloat, mut b: cpFloat) -> cpFloat {
    return if a < b { a } else { b };
}
#[inline]
unsafe extern "C" fn cpfclamp(
    mut f: cpFloat,
    mut min: cpFloat,
    mut max: cpFloat,
) -> cpFloat {
    return cpfmin(cpfmax(f, min), max);
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
unsafe extern "C" fn cpvneg(v: cpVect) -> cpVect {
    return cpv(-v.x, -v.y);
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
unsafe extern "C" fn cpvcross(v1: cpVect, v2: cpVect) -> cpFloat {
    return v1.x * v2.y - v1.y * v2.x;
}
#[inline]
unsafe extern "C" fn cpvperp(v: cpVect) -> cpVect {
    return cpv(-v.y, v.x);
}
#[inline]
unsafe extern "C" fn cpvrotate(v1: cpVect, v2: cpVect) -> cpVect {
    return cpv(v1.x * v2.x - v1.y * v2.y, v1.x * v2.y + v1.y * v2.x);
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
#[inline]
unsafe extern "C" fn normal_relative_velocity(
    mut a: *mut cpBody,
    mut b: *mut cpBody,
    mut r1: cpVect,
    mut r2: cpVect,
    mut n: cpVect,
) -> cpFloat {
    return cpvdot(relative_velocity(a, b, r1, r2), n);
}
#[inline]
unsafe extern "C" fn relative_velocity(
    mut a: *mut cpBody,
    mut b: *mut cpBody,
    mut r1: cpVect,
    mut r2: cpVect,
) -> cpVect {
    let mut v1_sum: cpVect = cpvadd((*a).v, cpvmult(cpvperp(r1), (*a).w));
    let mut v2_sum: cpVect = cpvadd((*b).v, cpvmult(cpvperp(r2), (*b).w));
    return cpvsub(v2_sum, v1_sum);
}
#[inline]
unsafe extern "C" fn k_scalar(
    mut a: *mut cpBody,
    mut b: *mut cpBody,
    mut r1: cpVect,
    mut r2: cpVect,
    mut n: cpVect,
) -> cpFloat {
    let mut value: cpFloat = k_scalar_body(a, r1, n) + k_scalar_body(b, r2, n);
    return value;
}
#[inline]
unsafe extern "C" fn k_scalar_body(
    mut body: *mut cpBody,
    mut r: cpVect,
    mut n: cpVect,
) -> cpFloat {
    let mut rcn: cpFloat = cpvcross(r, n);
    return (*body).m_inv + (*body).i_inv * rcn * rcn;
}
#[inline]
unsafe extern "C" fn apply_impulses(
    mut a: *mut cpBody,
    mut b: *mut cpBody,
    mut r1: cpVect,
    mut r2: cpVect,
    mut j: cpVect,
) {
    apply_impulse(a, cpvneg(j), r1);
    apply_impulse(b, j, r2);
}
#[inline]
unsafe extern "C" fn apply_impulse(mut body: *mut cpBody, mut j: cpVect, mut r: cpVect) {
    (*body).v = cpvadd((*body).v, cpvmult(j, (*body).m_inv));
    (*body).w += (*body).i_inv * cpvcross(r, j);
}
#[inline]
unsafe extern "C" fn apply_bias_impulses(
    mut a: *mut cpBody,
    mut b: *mut cpBody,
    mut r1: cpVect,
    mut r2: cpVect,
    mut j: cpVect,
) {
    apply_bias_impulse(a, cpvneg(j), r1);
    apply_bias_impulse(b, j, r2);
}
#[inline]
unsafe extern "C" fn apply_bias_impulse(
    mut body: *mut cpBody,
    mut j: cpVect,
    mut r: cpVect,
) {
    (*body).v_bias = cpvadd((*body).v_bias, cpvmult(j, (*body).m_inv));
    (*body).w_bias += (*body).i_inv * cpvcross(r, j);
}
#[inline]
unsafe extern "C" fn unthreadHelper(mut arb: *mut cpArbiter, mut body: *mut cpBody) {
    let mut thread: *mut cpArbiterThread = cpArbiterThreadForBody(arb, body);
    let mut prev: *mut cpArbiter = (*thread).prev;
    let mut next: *mut cpArbiter = (*thread).next;
    if !prev.is_null() {
        let ref mut fresh0 = (*cpArbiterThreadForBody(prev, body)).next;
        *fresh0 = next;
    } else if (*body).arbiterList == arb {
        (*body).arbiterList = next;
    }
    if !next.is_null() {
        let ref mut fresh1 = (*cpArbiterThreadForBody(next, body)).prev;
        *fresh1 = prev;
    }
    (*thread).prev = 0 as *mut cpArbiter;
    (*thread).next = 0 as *mut cpArbiter;
}
pub unsafe extern "C" fn cpArbiterUnthread(mut arb: *mut cpArbiter) {
    unthreadHelper(arb, (*arb).body_a);
    unthreadHelper(arb, (*arb).body_b);
}
pub unsafe extern "C" fn cpArbiterIsFirstContact(mut arb: *const cpArbiter) -> cpBool {
    return ((*arb).state as libc::c_uint
        == CP_ARBITER_STATE_FIRST_COLLISION as libc::c_int as libc::c_uint)
        as libc::c_int as cpBool;
}
pub unsafe extern "C" fn cpArbiterIsRemoval(mut arb: *const cpArbiter) -> cpBool {
    return ((*arb).state as libc::c_uint
        == CP_ARBITER_STATE_INVALIDATED as libc::c_int as libc::c_uint) as libc::c_int
        as cpBool;
}
pub unsafe extern "C" fn cpArbiterGetCount(mut arb: *const cpArbiter) -> libc::c_int {
    return if ((*arb).state as libc::c_uint)
        < CP_ARBITER_STATE_CACHED as libc::c_int as libc::c_uint
    {
        (*arb).count
    } else {
        0 as libc::c_int
    };
}
pub unsafe extern "C" fn cpArbiterGetNormal(mut arb: *const cpArbiter) -> cpVect {
    return cpvmult(
        (*arb).n,
        if (*arb).swapped as libc::c_int != 0 {
            -1.0f32 as libc::c_double
        } else {
            1.0f64
        },
    );
}
pub unsafe extern "C" fn cpArbiterGetPointA(
    mut arb: *const cpArbiter,
    mut i: libc::c_int,
) -> cpVect {
    if !(0 as libc::c_int <= i && i < cpArbiterGetCount(arb)) {
        cpMessage(
            b"0 <= i && i < cpArbiterGetCount(arb)\0" as *const u8
                as *const libc::c_char,
            b"../../src/cpArbiter.c\0" as *const u8 as *const libc::c_char,
            79 as libc::c_int,
            1 as libc::c_int,
            1 as libc::c_int,
            b"Index error: The specified contact index is invalid for this arbiter\0"
                as *const u8 as *const libc::c_char,
        );
        abort();
    }
    return cpvadd((*(*arb).body_a).p, (*((*arb).contacts).offset(i as isize)).r1);
}
pub unsafe extern "C" fn cpArbiterGetPointB(
    mut arb: *const cpArbiter,
    mut i: libc::c_int,
) -> cpVect {
    if !(0 as libc::c_int <= i && i < cpArbiterGetCount(arb)) {
        cpMessage(
            b"0 <= i && i < cpArbiterGetCount(arb)\0" as *const u8
                as *const libc::c_char,
            b"../../src/cpArbiter.c\0" as *const u8 as *const libc::c_char,
            86 as libc::c_int,
            1 as libc::c_int,
            1 as libc::c_int,
            b"Index error: The specified contact index is invalid for this arbiter\0"
                as *const u8 as *const libc::c_char,
        );
        abort();
    }
    return cpvadd((*(*arb).body_b).p, (*((*arb).contacts).offset(i as isize)).r2);
}
pub unsafe extern "C" fn cpArbiterGetDepth(
    mut arb: *const cpArbiter,
    mut i: libc::c_int,
) -> cpFloat {
    if !(0 as libc::c_int <= i && i < cpArbiterGetCount(arb)) {
        cpMessage(
            b"0 <= i && i < cpArbiterGetCount(arb)\0" as *const u8
                as *const libc::c_char,
            b"../../src/cpArbiter.c\0" as *const u8 as *const libc::c_char,
            93 as libc::c_int,
            1 as libc::c_int,
            1 as libc::c_int,
            b"Index error: The specified contact index is invalid for this arbiter\0"
                as *const u8 as *const libc::c_char,
        );
        abort();
    }
    let mut con: *mut cpContact = &mut *((*arb).contacts).offset(i as isize)
        as *mut cpContact;
    return cpvdot(
        cpvadd(
            cpvsub((*con).r2, (*con).r1),
            cpvsub((*(*arb).body_b).p, (*(*arb).body_a).p),
        ),
        (*arb).n,
    );
}
pub unsafe extern "C" fn cpArbiterGetContactPointSet(
    mut arb: *const cpArbiter,
) -> cpContactPointSet {
    let mut set: cpContactPointSet = cpContactPointSet {
        count: 0,
        normal: cpVect { x: 0., y: 0. },
        points: [C2RustUnnamed_0 {
            pointA: cpVect { x: 0., y: 0. },
            pointB: cpVect { x: 0., y: 0. },
            distance: 0.,
        }; 2],
    };
    set.count = cpArbiterGetCount(arb);
    let mut swapped: cpBool = (*arb).swapped;
    let mut n: cpVect = (*arb).n;
    set.normal = if swapped as libc::c_int != 0 { cpvneg(n) } else { n };
    let mut i: libc::c_int = 0 as libc::c_int;
    while i < set.count {
        let mut p1: cpVect = cpvadd(
            (*(*arb).body_a).p,
            (*((*arb).contacts).offset(i as isize)).r1,
        );
        let mut p2: cpVect = cpvadd(
            (*(*arb).body_b).p,
            (*((*arb).contacts).offset(i as isize)).r2,
        );
        set
            .points[i as usize]
            .pointA = if swapped as libc::c_int != 0 { p2 } else { p1 };
        set
            .points[i as usize]
            .pointB = if swapped as libc::c_int != 0 { p1 } else { p2 };
        set.points[i as usize].distance = cpvdot(cpvsub(p2, p1), n);
        i += 1;
        i;
    }
    return set;
}
pub unsafe extern "C" fn cpArbiterSetContactPointSet(
    mut arb: *mut cpArbiter,
    mut set: *mut cpContactPointSet,
) {
    let mut count: libc::c_int = (*set).count;
    if !(count == (*arb).count) {
        cpMessage(
            b"count == arb->count\0" as *const u8 as *const libc::c_char,
            b"../../src/cpArbiter.c\0" as *const u8 as *const libc::c_char,
            126 as libc::c_int,
            1 as libc::c_int,
            1 as libc::c_int,
            b"The number of contact points cannot be changed.\0" as *const u8
                as *const libc::c_char,
        );
        abort();
    }
    let mut swapped: cpBool = (*arb).swapped;
    (*arb)
        .n = if swapped as libc::c_int != 0 {
        cpvneg((*set).normal)
    } else {
        (*set).normal
    };
    let mut i: libc::c_int = 0 as libc::c_int;
    while i < count {
        let mut p1: cpVect = (*set).points[i as usize].pointA;
        let mut p2: cpVect = (*set).points[i as usize].pointB;
        (*((*arb).contacts).offset(i as isize))
            .r1 = cpvsub(
            if swapped as libc::c_int != 0 { p2 } else { p1 },
            (*(*arb).body_a).p,
        );
        (*((*arb).contacts).offset(i as isize))
            .r2 = cpvsub(
            if swapped as libc::c_int != 0 { p1 } else { p2 },
            (*(*arb).body_b).p,
        );
        i += 1;
        i;
    }
}
pub unsafe extern "C" fn cpArbiterTotalImpulse(mut arb: *const cpArbiter) -> cpVect {
    let mut contacts: *mut cpContact = (*arb).contacts;
    let mut n: cpVect = (*arb).n;
    let mut sum: cpVect = cpvzero;
    let mut i: libc::c_int = 0 as libc::c_int;
    let mut count: libc::c_int = cpArbiterGetCount(arb);
    while i < count {
        let mut con: *mut cpContact = &mut *contacts.offset(i as isize)
            as *mut cpContact;
        sum = cpvadd(sum, cpvrotate(n, cpv((*con).jnAcc, (*con).jtAcc)));
        i += 1;
        i;
    }
    return if (*arb).swapped as libc::c_int != 0 { sum } else { cpvneg(sum) };
}
pub unsafe extern "C" fn cpArbiterTotalKE(mut arb: *const cpArbiter) -> cpFloat {
    let mut eCoef: cpFloat = (1 as libc::c_int as libc::c_double - (*arb).e)
        / (1 as libc::c_int as libc::c_double + (*arb).e);
    let mut sum: cpFloat = 0.0f64;
    let mut contacts: *mut cpContact = (*arb).contacts;
    let mut i: libc::c_int = 0 as libc::c_int;
    let mut count: libc::c_int = cpArbiterGetCount(arb);
    while i < count {
        let mut con: *mut cpContact = &mut *contacts.offset(i as isize)
            as *mut cpContact;
        let mut jnAcc: cpFloat = (*con).jnAcc;
        let mut jtAcc: cpFloat = (*con).jtAcc;
        sum += eCoef * jnAcc * jnAcc / (*con).nMass + jtAcc * jtAcc / (*con).tMass;
        i += 1;
        i;
    }
    return sum;
}
pub unsafe extern "C" fn cpArbiterIgnore(mut arb: *mut cpArbiter) -> cpBool {
    (*arb).state = CP_ARBITER_STATE_IGNORE;
    return 0 as libc::c_int as cpBool;
}
pub unsafe extern "C" fn cpArbiterGetRestitution(mut arb: *const cpArbiter) -> cpFloat {
    return (*arb).e;
}
pub unsafe extern "C" fn cpArbiterSetRestitution(
    mut arb: *mut cpArbiter,
    mut restitution: cpFloat,
) {
    (*arb).e = restitution;
}
pub unsafe extern "C" fn cpArbiterGetFriction(mut arb: *const cpArbiter) -> cpFloat {
    return (*arb).u;
}
pub unsafe extern "C" fn cpArbiterSetFriction(
    mut arb: *mut cpArbiter,
    mut friction: cpFloat,
) {
    (*arb).u = friction;
}
pub unsafe extern "C" fn cpArbiterGetSurfaceVelocity(mut arb: *mut cpArbiter) -> cpVect {
    return cpvmult(
        (*arb).surface_vr,
        if (*arb).swapped as libc::c_int != 0 {
            -1.0f32 as libc::c_double
        } else {
            1.0f64
        },
    );
}
pub unsafe extern "C" fn cpArbiterSetSurfaceVelocity(
    mut arb: *mut cpArbiter,
    mut vr: cpVect,
) {
    (*arb)
        .surface_vr = cpvmult(
        vr,
        if (*arb).swapped as libc::c_int != 0 {
            -1.0f32 as libc::c_double
        } else {
            1.0f64
        },
    );
}
pub unsafe extern "C" fn cpArbiterGetUserData(
    mut arb: *const cpArbiter,
) -> cpDataPointer {
    return (*arb).data;
}
pub unsafe extern "C" fn cpArbiterSetUserData(
    mut arb: *mut cpArbiter,
    mut userData: cpDataPointer,
) {
    (*arb).data = userData;
}
pub unsafe extern "C" fn cpArbiterGetShapes(
    mut arb: *const cpArbiter,
    mut a: *mut *mut cpShape,
    mut b: *mut *mut cpShape,
) {
    if (*arb).swapped != 0 {
        *a = (*arb).b as *mut cpShape;
        *b = (*arb).a as *mut cpShape;
    } else {
        *a = (*arb).a as *mut cpShape;
        *b = (*arb).b as *mut cpShape;
    };
}
pub unsafe extern "C" fn cpArbiterGetBodies(
    mut arb: *const cpArbiter,
    mut a: *mut *mut cpBody,
    mut b: *mut *mut cpBody,
) {
    let mut shape_a: *mut cpShape = 0 as *mut cpShape;
    let mut shape_b: *mut cpShape = 0 as *mut cpShape;
    cpArbiterGetShapes(arb, &mut shape_a, &mut shape_b);
    *a = (*shape_a).body;
    *b = (*shape_b).body;
}
pub unsafe extern "C" fn cpArbiterCallWildcardBeginA(
    mut arb: *mut cpArbiter,
    mut space: *mut cpSpace,
) -> cpBool {
    let mut handler: *mut cpCollisionHandler = (*arb).handlerA;
    return ((*handler).beginFunc).unwrap()(arb, space, (*handler).userData);
}
pub unsafe extern "C" fn cpArbiterCallWildcardBeginB(
    mut arb: *mut cpArbiter,
    mut space: *mut cpSpace,
) -> cpBool {
    let mut handler: *mut cpCollisionHandler = (*arb).handlerB;
    (*arb).swapped = ((*arb).swapped == 0) as libc::c_int as cpBool;
    let mut retval: cpBool = ((*handler).beginFunc)
        .unwrap()(arb, space, (*handler).userData);
    (*arb).swapped = ((*arb).swapped == 0) as libc::c_int as cpBool;
    return retval;
}
pub unsafe extern "C" fn cpArbiterCallWildcardPreSolveA(
    mut arb: *mut cpArbiter,
    mut space: *mut cpSpace,
) -> cpBool {
    let mut handler: *mut cpCollisionHandler = (*arb).handlerA;
    return ((*handler).preSolveFunc).unwrap()(arb, space, (*handler).userData);
}
pub unsafe extern "C" fn cpArbiterCallWildcardPreSolveB(
    mut arb: *mut cpArbiter,
    mut space: *mut cpSpace,
) -> cpBool {
    let mut handler: *mut cpCollisionHandler = (*arb).handlerB;
    (*arb).swapped = ((*arb).swapped == 0) as libc::c_int as cpBool;
    let mut retval: cpBool = ((*handler).preSolveFunc)
        .unwrap()(arb, space, (*handler).userData);
    (*arb).swapped = ((*arb).swapped == 0) as libc::c_int as cpBool;
    return retval;
}
pub unsafe extern "C" fn cpArbiterCallWildcardPostSolveA(
    mut arb: *mut cpArbiter,
    mut space: *mut cpSpace,
) {
    let mut handler: *mut cpCollisionHandler = (*arb).handlerA;
    ((*handler).postSolveFunc).unwrap()(arb, space, (*handler).userData);
}
pub unsafe extern "C" fn cpArbiterCallWildcardPostSolveB(
    mut arb: *mut cpArbiter,
    mut space: *mut cpSpace,
) {
    let mut handler: *mut cpCollisionHandler = (*arb).handlerB;
    (*arb).swapped = ((*arb).swapped == 0) as libc::c_int as cpBool;
    ((*handler).postSolveFunc).unwrap()(arb, space, (*handler).userData);
    (*arb).swapped = ((*arb).swapped == 0) as libc::c_int as cpBool;
}
pub unsafe extern "C" fn cpArbiterCallWildcardSeparateA(
    mut arb: *mut cpArbiter,
    mut space: *mut cpSpace,
) {
    let mut handler: *mut cpCollisionHandler = (*arb).handlerA;
    ((*handler).separateFunc).unwrap()(arb, space, (*handler).userData);
}
pub unsafe extern "C" fn cpArbiterCallWildcardSeparateB(
    mut arb: *mut cpArbiter,
    mut space: *mut cpSpace,
) {
    let mut handler: *mut cpCollisionHandler = (*arb).handlerB;
    (*arb).swapped = ((*arb).swapped == 0) as libc::c_int as cpBool;
    ((*handler).separateFunc).unwrap()(arb, space, (*handler).userData);
    (*arb).swapped = ((*arb).swapped == 0) as libc::c_int as cpBool;
}
pub unsafe extern "C" fn cpArbiterInit(
    mut arb: *mut cpArbiter,
    mut a: *mut cpShape,
    mut b: *mut cpShape,
) -> *mut cpArbiter {
    (*arb).handler = 0 as *mut cpCollisionHandler;
    (*arb).swapped = 0 as libc::c_int as cpBool;
    (*arb).handler = 0 as *mut cpCollisionHandler;
    (*arb).handlerA = 0 as *mut cpCollisionHandler;
    (*arb).handlerB = 0 as *mut cpCollisionHandler;
    (*arb).e = 0.0f32 as cpFloat;
    (*arb).u = 0.0f32 as cpFloat;
    (*arb).surface_vr = cpvzero;
    (*arb).count = 0 as libc::c_int;
    (*arb).contacts = 0 as *mut cpContact;
    (*arb).a = a;
    (*arb).body_a = (*a).body;
    (*arb).b = b;
    (*arb).body_b = (*b).body;
    (*arb).thread_a.next = 0 as *mut cpArbiter;
    (*arb).thread_b.next = 0 as *mut cpArbiter;
    (*arb).thread_a.prev = 0 as *mut cpArbiter;
    (*arb).thread_b.prev = 0 as *mut cpArbiter;
    (*arb).stamp = 0 as libc::c_int as cpTimestamp;
    (*arb).state = CP_ARBITER_STATE_FIRST_COLLISION;
    (*arb).data = 0 as *mut libc::c_void;
    return arb;
}
#[inline]
unsafe extern "C" fn cpSpaceLookupHandler(
    mut space: *mut cpSpace,
    mut a: cpCollisionType,
    mut b: cpCollisionType,
    mut defaultValue: *mut cpCollisionHandler,
) -> *mut cpCollisionHandler {
    let mut types: [cpCollisionType; 2] = [a, b];
    let mut handler: *mut cpCollisionHandler = cpHashSetFind(
        (*space).collisionHandlers,
        a.wrapping_mul(3344921057 as libc::c_ulong)
            ^ b.wrapping_mul(3344921057 as libc::c_ulong),
        types.as_mut_ptr() as *const libc::c_void,
    ) as *mut cpCollisionHandler;
    return if !handler.is_null() { handler } else { defaultValue };
}
pub unsafe extern "C" fn cpArbiterUpdate(
    mut arb: *mut cpArbiter,
    mut info: *mut cpCollisionInfo,
    mut space: *mut cpSpace,
) {
    let mut a: *const cpShape = (*info).a;
    let mut b: *const cpShape = (*info).b;
    (*arb).a = a;
    (*arb).body_a = (*a).body;
    (*arb).b = b;
    (*arb).body_b = (*b).body;
    let mut i: libc::c_int = 0 as libc::c_int;
    while i < (*info).count {
        let mut con: *mut cpContact = &mut *((*info).arr).offset(i as isize)
            as *mut cpContact;
        (*con).r1 = cpvsub((*con).r1, (*(*a).body).p);
        (*con).r2 = cpvsub((*con).r2, (*(*b).body).p);
        (*con).jtAcc = 0.0f32 as cpFloat;
        (*con).jnAcc = (*con).jtAcc;
        let mut j: libc::c_int = 0 as libc::c_int;
        while j < (*arb).count {
            let mut old: *mut cpContact = &mut *((*arb).contacts).offset(j as isize)
                as *mut cpContact;
            if (*con).hash == (*old).hash {
                (*con).jnAcc = (*old).jnAcc;
                (*con).jtAcc = (*old).jtAcc;
            }
            j += 1;
            j;
        }
        i += 1;
        i;
    }
    (*arb).contacts = (*info).arr;
    (*arb).count = (*info).count;
    (*arb).n = (*info).n;
    (*arb).e = (*a).e * (*b).e;
    (*arb).u = (*a).u * (*b).u;
    let mut surface_vr: cpVect = cpvsub((*b).surfaceV, (*a).surfaceV);
    (*arb)
        .surface_vr = cpvsub(
        surface_vr,
        cpvmult((*info).n, cpvdot(surface_vr, (*info).n)),
    );
    let mut typeA: cpCollisionType = (*(*info).a).type_0;
    let mut typeB: cpCollisionType = (*(*info).b).type_0;
    let mut defaultHandler: *mut cpCollisionHandler = &mut (*space).defaultHandler;
    (*arb).handler = cpSpaceLookupHandler(space, typeA, typeB, defaultHandler);
    let mut handler: *mut cpCollisionHandler = (*arb).handler;
    (*arb)
        .swapped = (typeA != (*handler).typeA
        && (*handler).typeA != !(0 as libc::c_int as cpCollisionType)) as libc::c_int
        as cpBool;
    let mut swapped: cpBool = (*arb).swapped;
    if handler != defaultHandler || (*space).usesWildcards as libc::c_int != 0 {
        (*arb)
            .handlerA = cpSpaceLookupHandler(
            space,
            if swapped as libc::c_int != 0 { typeB } else { typeA },
            !(0 as libc::c_int as cpCollisionType),
            &mut cpCollisionHandlerDoNothing,
        );
        (*arb)
            .handlerB = cpSpaceLookupHandler(
            space,
            if swapped as libc::c_int != 0 { typeA } else { typeB },
            !(0 as libc::c_int as cpCollisionType),
            &mut cpCollisionHandlerDoNothing,
        );
    }
    if (*arb).state as libc::c_uint
        == CP_ARBITER_STATE_CACHED as libc::c_int as libc::c_uint
    {
        (*arb).state = CP_ARBITER_STATE_FIRST_COLLISION;
    }
}
pub unsafe extern "C" fn cpArbiterPreStep(
    mut arb: *mut cpArbiter,
    mut dt: cpFloat,
    mut slop: cpFloat,
    mut bias: cpFloat,
) {
    let mut a: *mut cpBody = (*arb).body_a;
    let mut b: *mut cpBody = (*arb).body_b;
    let mut n: cpVect = (*arb).n;
    let mut body_delta: cpVect = cpvsub((*b).p, (*a).p);
    let mut i: libc::c_int = 0 as libc::c_int;
    while i < (*arb).count {
        let mut con: *mut cpContact = &mut *((*arb).contacts).offset(i as isize)
            as *mut cpContact;
        (*con)
            .nMass = 1.0f32 as libc::c_double / k_scalar(a, b, (*con).r1, (*con).r2, n);
        (*con)
            .tMass = 1.0f32 as libc::c_double
            / k_scalar(a, b, (*con).r1, (*con).r2, cpvperp(n));
        let mut dist: cpFloat = cpvdot(
            cpvadd(cpvsub((*con).r2, (*con).r1), body_delta),
            n,
        );
        (*con).bias = -bias * cpfmin(0.0f32 as cpFloat, dist + slop) / dt;
        (*con).jBias = 0.0f32 as cpFloat;
        (*con)
            .bounce = normal_relative_velocity(a, b, (*con).r1, (*con).r2, n) * (*arb).e;
        i += 1;
        i;
    }
}
pub unsafe extern "C" fn cpArbiterApplyCachedImpulse(
    mut arb: *mut cpArbiter,
    mut dt_coef: cpFloat,
) {
    if cpArbiterIsFirstContact(arb) != 0 {
        return;
    }
    let mut a: *mut cpBody = (*arb).body_a;
    let mut b: *mut cpBody = (*arb).body_b;
    let mut n: cpVect = (*arb).n;
    let mut i: libc::c_int = 0 as libc::c_int;
    while i < (*arb).count {
        let mut con: *mut cpContact = &mut *((*arb).contacts).offset(i as isize)
            as *mut cpContact;
        let mut j: cpVect = cpvrotate(n, cpv((*con).jnAcc, (*con).jtAcc));
        apply_impulses(a, b, (*con).r1, (*con).r2, cpvmult(j, dt_coef));
        i += 1;
        i;
    }
}
pub unsafe extern "C" fn cpArbiterApplyImpulse(mut arb: *mut cpArbiter) {
    let mut a: *mut cpBody = (*arb).body_a;
    let mut b: *mut cpBody = (*arb).body_b;
    let mut n: cpVect = (*arb).n;
    let mut surface_vr: cpVect = (*arb).surface_vr;
    let mut friction: cpFloat = (*arb).u;
    let mut i: libc::c_int = 0 as libc::c_int;
    while i < (*arb).count {
        let mut con: *mut cpContact = &mut *((*arb).contacts).offset(i as isize)
            as *mut cpContact;
        let mut nMass: cpFloat = (*con).nMass;
        let mut r1: cpVect = (*con).r1;
        let mut r2: cpVect = (*con).r2;
        let mut vb1: cpVect = cpvadd((*a).v_bias, cpvmult(cpvperp(r1), (*a).w_bias));
        let mut vb2: cpVect = cpvadd((*b).v_bias, cpvmult(cpvperp(r2), (*b).w_bias));
        let mut vr: cpVect = cpvadd(relative_velocity(a, b, r1, r2), surface_vr);
        let mut vbn: cpFloat = cpvdot(cpvsub(vb2, vb1), n);
        let mut vrn: cpFloat = cpvdot(vr, n);
        let mut vrt: cpFloat = cpvdot(vr, cpvperp(n));
        let mut jbn: cpFloat = ((*con).bias - vbn) * nMass;
        let mut jbnOld: cpFloat = (*con).jBias;
        (*con).jBias = cpfmax(jbnOld + jbn, 0.0f32 as cpFloat);
        let mut jn: cpFloat = -((*con).bounce + vrn) * nMass;
        let mut jnOld: cpFloat = (*con).jnAcc;
        (*con).jnAcc = cpfmax(jnOld + jn, 0.0f32 as cpFloat);
        let mut jtMax: cpFloat = friction * (*con).jnAcc;
        let mut jt: cpFloat = -vrt * (*con).tMass;
        let mut jtOld: cpFloat = (*con).jtAcc;
        (*con).jtAcc = cpfclamp(jtOld + jt, -jtMax, jtMax);
        apply_bias_impulses(a, b, r1, r2, cpvmult(n, (*con).jBias - jbnOld));
        apply_impulses(
            a,
            b,
            r1,
            r2,
            cpvrotate(n, cpv((*con).jnAcc - jnOld, (*con).jtAcc - jtOld)),
        );
        i += 1;
        i;
    }
}
