use ::libc;
extern "C" {
    pub type cpHashSet;
    pub type cpContactBufferHeader;
    fn pow(_: libc::c_double, _: libc::c_double) -> libc::c_double;
    fn sqrt(_: libc::c_double) -> libc::c_double;
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
    fn cpBodyWorldToLocal(body: *const cpBody, point: cpVect) -> cpVect;
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
pub struct cpMat2x2 {
    pub a: cpFloat,
    pub b: cpFloat,
    pub c: cpFloat,
    pub d: cpFloat,
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
pub struct cpPivotJoint {
    pub constraint: cpConstraint,
    pub anchorA: cpVect,
    pub anchorB: cpVect,
    pub r1: cpVect,
    pub r2: cpVect,
    pub k: cpMat2x2,
    pub jAcc: cpVect,
    pub bias: cpVect,
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
unsafe extern "C" fn cpvlength(v: cpVect) -> cpFloat {
    return sqrt(cpvdot(v, v));
}
#[inline]
unsafe extern "C" fn cpvnormalize(v: cpVect) -> cpVect {
    return cpvmult(
        v,
        1.0f32 as libc::c_double / (cpvlength(v) + 2.2250738585072014e-308f64),
    );
}
#[inline]
unsafe extern "C" fn cpvclamp(v: cpVect, len: cpFloat) -> cpVect {
    return if cpvdot(v, v) > len * len { cpvmult(cpvnormalize(v), len) } else { v };
}
#[inline]
unsafe extern "C" fn cpMat2x2New(
    mut a: cpFloat,
    mut b: cpFloat,
    mut c: cpFloat,
    mut d: cpFloat,
) -> cpMat2x2 {
    let mut m: cpMat2x2 = {
        let mut init = cpMat2x2 { a: a, b: b, c: c, d: d };
        init
    };
    return m;
}
#[inline]
unsafe extern "C" fn cpMat2x2Transform(mut m: cpMat2x2, mut v: cpVect) -> cpVect {
    return cpv(v.x * m.a + v.y * m.b, v.x * m.c + v.y * m.d);
}
#[inline]
unsafe extern "C" fn cpTransformVect(mut t: cpTransform, mut v: cpVect) -> cpVect {
    return cpv(t.a * v.x + t.c * v.y, t.b * v.x + t.d * v.y);
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
unsafe extern "C" fn bias_coef(mut errorBias: cpFloat, mut dt: cpFloat) -> cpFloat {
    return 1.0f32 as libc::c_double - pow(errorBias, dt);
}
#[inline]
unsafe extern "C" fn k_tensor(
    mut a: *mut cpBody,
    mut b: *mut cpBody,
    mut r1: cpVect,
    mut r2: cpVect,
) -> cpMat2x2 {
    let mut m_sum: cpFloat = (*a).m_inv + (*b).m_inv;
    let mut k11: cpFloat = m_sum;
    let mut k12: cpFloat = 0.0f32 as cpFloat;
    let mut k21: cpFloat = 0.0f32 as cpFloat;
    let mut k22: cpFloat = m_sum;
    let mut a_i_inv: cpFloat = (*a).i_inv;
    let mut r1xsq: cpFloat = r1.x * r1.x * a_i_inv;
    let mut r1ysq: cpFloat = r1.y * r1.y * a_i_inv;
    let mut r1nxy: cpFloat = -r1.x * r1.y * a_i_inv;
    k11 += r1ysq;
    k12 += r1nxy;
    k21 += r1nxy;
    k22 += r1xsq;
    let mut b_i_inv: cpFloat = (*b).i_inv;
    let mut r2xsq: cpFloat = r2.x * r2.x * b_i_inv;
    let mut r2ysq: cpFloat = r2.y * r2.y * b_i_inv;
    let mut r2nxy: cpFloat = -r2.x * r2.y * b_i_inv;
    k11 += r2ysq;
    k12 += r2nxy;
    k21 += r2nxy;
    k22 += r2xsq;
    let mut det: cpFloat = k11 * k22 - k12 * k21;
    let mut det_inv: cpFloat = 1.0f32 as libc::c_double / det;
    return cpMat2x2New(k22 * det_inv, -k12 * det_inv, -k21 * det_inv, k11 * det_inv);
}
#[inline]
unsafe extern "C" fn cpConstraintActivateBodies(mut constraint: *mut cpConstraint) {
    let mut a: *mut cpBody = (*constraint).a;
    cpBodyActivate(a);
    let mut b: *mut cpBody = (*constraint).b;
    cpBodyActivate(b);
}
unsafe extern "C" fn preStep(mut joint: *mut cpPivotJoint, mut dt: cpFloat) {
    let mut a: *mut cpBody = (*joint).constraint.a;
    let mut b: *mut cpBody = (*joint).constraint.b;
    (*joint).r1 = cpTransformVect((*a).transform, cpvsub((*joint).anchorA, (*a).cog));
    (*joint).r2 = cpTransformVect((*b).transform, cpvsub((*joint).anchorB, (*b).cog));
    (*joint).k = k_tensor(a, b, (*joint).r1, (*joint).r2);
    let mut delta: cpVect = cpvsub(
        cpvadd((*b).p, (*joint).r2),
        cpvadd((*a).p, (*joint).r1),
    );
    (*joint)
        .bias = cpvclamp(
        cpvmult(delta, -bias_coef((*joint).constraint.errorBias, dt) / dt),
        (*joint).constraint.maxBias,
    );
}
unsafe extern "C" fn applyCachedImpulse(
    mut joint: *mut cpPivotJoint,
    mut dt_coef: cpFloat,
) {
    let mut a: *mut cpBody = (*joint).constraint.a;
    let mut b: *mut cpBody = (*joint).constraint.b;
    apply_impulses(a, b, (*joint).r1, (*joint).r2, cpvmult((*joint).jAcc, dt_coef));
}
unsafe extern "C" fn applyImpulse(mut joint: *mut cpPivotJoint, mut dt: cpFloat) {
    let mut a: *mut cpBody = (*joint).constraint.a;
    let mut b: *mut cpBody = (*joint).constraint.b;
    let mut r1: cpVect = (*joint).r1;
    let mut r2: cpVect = (*joint).r2;
    let mut vr: cpVect = relative_velocity(a, b, r1, r2);
    let mut j: cpVect = cpMat2x2Transform((*joint).k, cpvsub((*joint).bias, vr));
    let mut jOld: cpVect = (*joint).jAcc;
    (*joint)
        .jAcc = cpvclamp(cpvadd((*joint).jAcc, j), (*joint).constraint.maxForce * dt);
    j = cpvsub((*joint).jAcc, jOld);
    apply_impulses(a, b, (*joint).r1, (*joint).r2, j);
}
unsafe extern "C" fn getImpulse(mut joint: *mut cpConstraint) -> cpFloat {
    return cpvlength((*(joint as *mut cpPivotJoint)).jAcc);
}
static mut klass: cpConstraintClass = unsafe {
    {
        let mut init = cpConstraintClass {
            preStep: ::std::mem::transmute::<
                Option::<unsafe extern "C" fn(*mut cpPivotJoint, cpFloat) -> ()>,
                cpConstraintPreStepImpl,
            >(Some(preStep as unsafe extern "C" fn(*mut cpPivotJoint, cpFloat) -> ())),
            applyCachedImpulse: ::std::mem::transmute::<
                Option::<unsafe extern "C" fn(*mut cpPivotJoint, cpFloat) -> ()>,
                cpConstraintApplyCachedImpulseImpl,
            >(
                Some(
                    applyCachedImpulse
                        as unsafe extern "C" fn(*mut cpPivotJoint, cpFloat) -> (),
                ),
            ),
            applyImpulse: ::std::mem::transmute::<
                Option::<unsafe extern "C" fn(*mut cpPivotJoint, cpFloat) -> ()>,
                cpConstraintApplyImpulseImpl,
            >(
                Some(
                    applyImpulse
                        as unsafe extern "C" fn(*mut cpPivotJoint, cpFloat) -> (),
                ),
            ),
            getImpulse: ::std::mem::transmute::<
                Option::<unsafe extern "C" fn(*mut cpConstraint) -> cpFloat>,
                cpConstraintGetImpulseImpl,
            >(Some(getImpulse as unsafe extern "C" fn(*mut cpConstraint) -> cpFloat)),
        };
        init
    }
};
pub unsafe extern "C" fn cpPivotJointAlloc() -> *mut cpPivotJoint {
    return calloc(
        1 as libc::c_int as libc::c_ulong,
        ::std::mem::size_of::<cpPivotJoint>() as libc::c_ulong,
    ) as *mut cpPivotJoint;
}
pub unsafe extern "C" fn cpPivotJointInit(
    mut joint: *mut cpPivotJoint,
    mut a: *mut cpBody,
    mut b: *mut cpBody,
    mut anchorA: cpVect,
    mut anchorB: cpVect,
) -> *mut cpPivotJoint {
    cpConstraintInit(joint as *mut cpConstraint, &klass, a, b);
    (*joint).anchorA = anchorA;
    (*joint).anchorB = anchorB;
    (*joint).jAcc = cpvzero;
    return joint;
}
pub unsafe extern "C" fn cpPivotJointNew2(
    mut a: *mut cpBody,
    mut b: *mut cpBody,
    mut anchorA: cpVect,
    mut anchorB: cpVect,
) -> *mut cpConstraint {
    return cpPivotJointInit(cpPivotJointAlloc(), a, b, anchorA, anchorB)
        as *mut cpConstraint;
}
pub unsafe extern "C" fn cpPivotJointNew(
    mut a: *mut cpBody,
    mut b: *mut cpBody,
    mut pivot: cpVect,
) -> *mut cpConstraint {
    let mut anchorA: cpVect = if !a.is_null() {
        cpBodyWorldToLocal(a, pivot)
    } else {
        pivot
    };
    let mut anchorB: cpVect = if !b.is_null() {
        cpBodyWorldToLocal(b, pivot)
    } else {
        pivot
    };
    return cpPivotJointNew2(a, b, anchorA, anchorB);
}
pub unsafe extern "C" fn cpConstraintIsPivotJoint(
    mut constraint: *const cpConstraint,
) -> cpBool {
    return ((*constraint).klass == &klass as *const cpConstraintClass) as libc::c_int
        as cpBool;
}
pub unsafe extern "C" fn cpPivotJointGetAnchorA(
    mut constraint: *const cpConstraint,
) -> cpVect {
    if cpConstraintIsPivotJoint(constraint) == 0 {
        cpMessage(
            b"cpConstraintIsPivotJoint(constraint)\0" as *const u8
                as *const libc::c_char,
            b"../../src/cpPivotJoint.c\0" as *const u8 as *const libc::c_char,
            127 as libc::c_int,
            1 as libc::c_int,
            1 as libc::c_int,
            b"Constraint is not a pivot joint.\0" as *const u8 as *const libc::c_char,
        );
        abort();
    }
    return (*(constraint as *mut cpPivotJoint)).anchorA;
}
pub unsafe extern "C" fn cpPivotJointSetAnchorA(
    mut constraint: *mut cpConstraint,
    mut anchorA: cpVect,
) {
    if cpConstraintIsPivotJoint(constraint) == 0 {
        cpMessage(
            b"cpConstraintIsPivotJoint(constraint)\0" as *const u8
                as *const libc::c_char,
            b"../../src/cpPivotJoint.c\0" as *const u8 as *const libc::c_char,
            134 as libc::c_int,
            1 as libc::c_int,
            1 as libc::c_int,
            b"Constraint is not a pivot joint.\0" as *const u8 as *const libc::c_char,
        );
        abort();
    }
    cpConstraintActivateBodies(constraint);
    (*(constraint as *mut cpPivotJoint)).anchorA = anchorA;
}
pub unsafe extern "C" fn cpPivotJointGetAnchorB(
    mut constraint: *const cpConstraint,
) -> cpVect {
    if cpConstraintIsPivotJoint(constraint) == 0 {
        cpMessage(
            b"cpConstraintIsPivotJoint(constraint)\0" as *const u8
                as *const libc::c_char,
            b"../../src/cpPivotJoint.c\0" as *const u8 as *const libc::c_char,
            142 as libc::c_int,
            1 as libc::c_int,
            1 as libc::c_int,
            b"Constraint is not a pivot joint.\0" as *const u8 as *const libc::c_char,
        );
        abort();
    }
    return (*(constraint as *mut cpPivotJoint)).anchorB;
}
pub unsafe extern "C" fn cpPivotJointSetAnchorB(
    mut constraint: *mut cpConstraint,
    mut anchorB: cpVect,
) {
    if cpConstraintIsPivotJoint(constraint) == 0 {
        cpMessage(
            b"cpConstraintIsPivotJoint(constraint)\0" as *const u8
                as *const libc::c_char,
            b"../../src/cpPivotJoint.c\0" as *const u8 as *const libc::c_char,
            149 as libc::c_int,
            1 as libc::c_int,
            1 as libc::c_int,
            b"Constraint is not a pivot joint.\0" as *const u8 as *const libc::c_char,
        );
        abort();
    }
    cpConstraintActivateBodies(constraint);
    (*(constraint as *mut cpPivotJoint)).anchorB = anchorB;
}
