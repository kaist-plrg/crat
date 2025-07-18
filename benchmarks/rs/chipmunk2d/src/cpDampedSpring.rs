use ::libc;
extern "C" {
    pub type cpHashSet;
    pub type cpContactBufferHeader;
    fn exp(_: libc::c_double) -> libc::c_double;
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
pub struct cpDampedSpring {
    pub constraint: cpConstraint,
    pub anchorA: cpVect,
    pub anchorB: cpVect,
    pub restLength: cpFloat,
    pub stiffness: cpFloat,
    pub damping: cpFloat,
    pub springForceFunc: cpDampedSpringForceFunc,
    pub target_vrn: cpFloat,
    pub v_coef: cpFloat,
    pub r1: cpVect,
    pub r2: cpVect,
    pub nMass: cpFloat,
    pub n: cpVect,
    pub jAcc: cpFloat,
}
pub type cpDampedSpringForceFunc = Option::<
    unsafe extern "C" fn(*mut cpConstraint, cpFloat) -> cpFloat,
>;
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
unsafe extern "C" fn cpTransformVect(mut t: cpTransform, mut v: cpVect) -> cpVect {
    return cpv(t.a * v.x + t.c * v.y, t.b * v.x + t.d * v.y);
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
unsafe extern "C" fn cpConstraintActivateBodies(mut constraint: *mut cpConstraint) {
    let mut a: *mut cpBody = (*constraint).a;
    cpBodyActivate(a);
    let mut b: *mut cpBody = (*constraint).b;
    cpBodyActivate(b);
}
unsafe extern "C" fn defaultSpringForce(
    mut spring: *mut cpDampedSpring,
    mut dist: cpFloat,
) -> cpFloat {
    return ((*spring).restLength - dist) * (*spring).stiffness;
}
unsafe extern "C" fn preStep(mut spring: *mut cpDampedSpring, mut dt: cpFloat) {
    let mut a: *mut cpBody = (*spring).constraint.a;
    let mut b: *mut cpBody = (*spring).constraint.b;
    (*spring).r1 = cpTransformVect((*a).transform, cpvsub((*spring).anchorA, (*a).cog));
    (*spring).r2 = cpTransformVect((*b).transform, cpvsub((*spring).anchorB, (*b).cog));
    let mut delta: cpVect = cpvsub(
        cpvadd((*b).p, (*spring).r2),
        cpvadd((*a).p, (*spring).r1),
    );
    let mut dist: cpFloat = cpvlength(delta);
    (*spring)
        .n = cpvmult(
        delta,
        1.0f32 as libc::c_double
            / (if dist != 0. { dist } else { ::std::f32::INFINITY as libc::c_double }),
    );
    let mut k: cpFloat = k_scalar(a, b, (*spring).r1, (*spring).r2, (*spring).n);
    (*spring).nMass = 1.0f32 as libc::c_double / k;
    (*spring).target_vrn = 0.0f32 as cpFloat;
    (*spring).v_coef = 1.0f32 as libc::c_double - exp(-(*spring).damping * dt * k);
    let mut f_spring: cpFloat = ((*spring).springForceFunc)
        .unwrap()(spring as *mut cpConstraint, dist);
    (*spring).jAcc = f_spring * dt;
    let mut j_spring: cpFloat = (*spring).jAcc;
    apply_impulses(a, b, (*spring).r1, (*spring).r2, cpvmult((*spring).n, j_spring));
}
unsafe extern "C" fn applyCachedImpulse(
    mut spring: *mut cpDampedSpring,
    mut dt_coef: cpFloat,
) {}
unsafe extern "C" fn applyImpulse(mut spring: *mut cpDampedSpring, mut dt: cpFloat) {
    let mut a: *mut cpBody = (*spring).constraint.a;
    let mut b: *mut cpBody = (*spring).constraint.b;
    let mut n: cpVect = (*spring).n;
    let mut r1: cpVect = (*spring).r1;
    let mut r2: cpVect = (*spring).r2;
    let mut vrn: cpFloat = normal_relative_velocity(a, b, r1, r2, n);
    let mut v_damp: cpFloat = ((*spring).target_vrn - vrn) * (*spring).v_coef;
    (*spring).target_vrn = vrn + v_damp;
    let mut j_damp: cpFloat = v_damp * (*spring).nMass;
    (*spring).jAcc += j_damp;
    apply_impulses(a, b, (*spring).r1, (*spring).r2, cpvmult((*spring).n, j_damp));
}
unsafe extern "C" fn getImpulse(mut spring: *mut cpDampedSpring) -> cpFloat {
    return (*spring).jAcc;
}
static mut klass: cpConstraintClass = unsafe {
    {
        let mut init = cpConstraintClass {
            preStep: ::std::mem::transmute::<
                Option::<unsafe extern "C" fn(*mut cpDampedSpring, cpFloat) -> ()>,
                cpConstraintPreStepImpl,
            >(Some(preStep as unsafe extern "C" fn(*mut cpDampedSpring, cpFloat) -> ())),
            applyCachedImpulse: ::std::mem::transmute::<
                Option::<unsafe extern "C" fn(*mut cpDampedSpring, cpFloat) -> ()>,
                cpConstraintApplyCachedImpulseImpl,
            >(
                Some(
                    applyCachedImpulse
                        as unsafe extern "C" fn(*mut cpDampedSpring, cpFloat) -> (),
                ),
            ),
            applyImpulse: ::std::mem::transmute::<
                Option::<unsafe extern "C" fn(*mut cpDampedSpring, cpFloat) -> ()>,
                cpConstraintApplyImpulseImpl,
            >(
                Some(
                    applyImpulse
                        as unsafe extern "C" fn(*mut cpDampedSpring, cpFloat) -> (),
                ),
            ),
            getImpulse: ::std::mem::transmute::<
                Option::<unsafe extern "C" fn(*mut cpDampedSpring) -> cpFloat>,
                cpConstraintGetImpulseImpl,
            >(Some(getImpulse as unsafe extern "C" fn(*mut cpDampedSpring) -> cpFloat)),
        };
        init
    }
};
pub unsafe extern "C" fn cpDampedSpringAlloc() -> *mut cpDampedSpring {
    return calloc(
        1 as libc::c_int as libc::c_ulong,
        ::std::mem::size_of::<cpDampedSpring>() as libc::c_ulong,
    ) as *mut cpDampedSpring;
}
pub unsafe extern "C" fn cpDampedSpringInit(
    mut spring: *mut cpDampedSpring,
    mut a: *mut cpBody,
    mut b: *mut cpBody,
    mut anchorA: cpVect,
    mut anchorB: cpVect,
    mut restLength: cpFloat,
    mut stiffness: cpFloat,
    mut damping: cpFloat,
) -> *mut cpDampedSpring {
    cpConstraintInit(spring as *mut cpConstraint, &klass, a, b);
    (*spring).anchorA = anchorA;
    (*spring).anchorB = anchorB;
    (*spring).restLength = restLength;
    (*spring).stiffness = stiffness;
    (*spring).damping = damping;
    (*spring)
        .springForceFunc = ::std::mem::transmute::<
        Option::<unsafe extern "C" fn(*mut cpDampedSpring, cpFloat) -> cpFloat>,
        cpDampedSpringForceFunc,
    >(
        Some(
            defaultSpringForce
                as unsafe extern "C" fn(*mut cpDampedSpring, cpFloat) -> cpFloat,
        ),
    );
    (*spring).jAcc = 0.0f32 as cpFloat;
    return spring;
}
pub unsafe extern "C" fn cpDampedSpringNew(
    mut a: *mut cpBody,
    mut b: *mut cpBody,
    mut anchorA: cpVect,
    mut anchorB: cpVect,
    mut restLength: cpFloat,
    mut stiffness: cpFloat,
    mut damping: cpFloat,
) -> *mut cpConstraint {
    return cpDampedSpringInit(
        cpDampedSpringAlloc(),
        a,
        b,
        anchorA,
        anchorB,
        restLength,
        stiffness,
        damping,
    ) as *mut cpConstraint;
}
pub unsafe extern "C" fn cpConstraintIsDampedSpring(
    mut constraint: *const cpConstraint,
) -> cpBool {
    return ((*constraint).klass == &klass as *const cpConstraintClass) as libc::c_int
        as cpBool;
}
pub unsafe extern "C" fn cpDampedSpringGetAnchorA(
    mut constraint: *const cpConstraint,
) -> cpVect {
    if cpConstraintIsDampedSpring(constraint) == 0 {
        cpMessage(
            b"cpConstraintIsDampedSpring(constraint)\0" as *const u8
                as *const libc::c_char,
            b"../../src/cpDampedSpring.c\0" as *const u8 as *const libc::c_char,
            131 as libc::c_int,
            1 as libc::c_int,
            1 as libc::c_int,
            b"Constraint is not a damped spring.\0" as *const u8 as *const libc::c_char,
        );
        abort();
    }
    return (*(constraint as *mut cpDampedSpring)).anchorA;
}
pub unsafe extern "C" fn cpDampedSpringSetAnchorA(
    mut constraint: *mut cpConstraint,
    mut anchorA: cpVect,
) {
    if cpConstraintIsDampedSpring(constraint) == 0 {
        cpMessage(
            b"cpConstraintIsDampedSpring(constraint)\0" as *const u8
                as *const libc::c_char,
            b"../../src/cpDampedSpring.c\0" as *const u8 as *const libc::c_char,
            138 as libc::c_int,
            1 as libc::c_int,
            1 as libc::c_int,
            b"Constraint is not a damped spring.\0" as *const u8 as *const libc::c_char,
        );
        abort();
    }
    cpConstraintActivateBodies(constraint);
    (*(constraint as *mut cpDampedSpring)).anchorA = anchorA;
}
pub unsafe extern "C" fn cpDampedSpringGetAnchorB(
    mut constraint: *const cpConstraint,
) -> cpVect {
    if cpConstraintIsDampedSpring(constraint) == 0 {
        cpMessage(
            b"cpConstraintIsDampedSpring(constraint)\0" as *const u8
                as *const libc::c_char,
            b"../../src/cpDampedSpring.c\0" as *const u8 as *const libc::c_char,
            146 as libc::c_int,
            1 as libc::c_int,
            1 as libc::c_int,
            b"Constraint is not a damped spring.\0" as *const u8 as *const libc::c_char,
        );
        abort();
    }
    return (*(constraint as *mut cpDampedSpring)).anchorB;
}
pub unsafe extern "C" fn cpDampedSpringSetAnchorB(
    mut constraint: *mut cpConstraint,
    mut anchorB: cpVect,
) {
    if cpConstraintIsDampedSpring(constraint) == 0 {
        cpMessage(
            b"cpConstraintIsDampedSpring(constraint)\0" as *const u8
                as *const libc::c_char,
            b"../../src/cpDampedSpring.c\0" as *const u8 as *const libc::c_char,
            153 as libc::c_int,
            1 as libc::c_int,
            1 as libc::c_int,
            b"Constraint is not a damped spring.\0" as *const u8 as *const libc::c_char,
        );
        abort();
    }
    cpConstraintActivateBodies(constraint);
    (*(constraint as *mut cpDampedSpring)).anchorB = anchorB;
}
pub unsafe extern "C" fn cpDampedSpringGetRestLength(
    mut constraint: *const cpConstraint,
) -> cpFloat {
    if cpConstraintIsDampedSpring(constraint) == 0 {
        cpMessage(
            b"cpConstraintIsDampedSpring(constraint)\0" as *const u8
                as *const libc::c_char,
            b"../../src/cpDampedSpring.c\0" as *const u8 as *const libc::c_char,
            161 as libc::c_int,
            1 as libc::c_int,
            1 as libc::c_int,
            b"Constraint is not a damped spring.\0" as *const u8 as *const libc::c_char,
        );
        abort();
    }
    return (*(constraint as *mut cpDampedSpring)).restLength;
}
pub unsafe extern "C" fn cpDampedSpringSetRestLength(
    mut constraint: *mut cpConstraint,
    mut restLength: cpFloat,
) {
    if cpConstraintIsDampedSpring(constraint) == 0 {
        cpMessage(
            b"cpConstraintIsDampedSpring(constraint)\0" as *const u8
                as *const libc::c_char,
            b"../../src/cpDampedSpring.c\0" as *const u8 as *const libc::c_char,
            168 as libc::c_int,
            1 as libc::c_int,
            1 as libc::c_int,
            b"Constraint is not a damped spring.\0" as *const u8 as *const libc::c_char,
        );
        abort();
    }
    cpConstraintActivateBodies(constraint);
    (*(constraint as *mut cpDampedSpring)).restLength = restLength;
}
pub unsafe extern "C" fn cpDampedSpringGetStiffness(
    mut constraint: *const cpConstraint,
) -> cpFloat {
    if cpConstraintIsDampedSpring(constraint) == 0 {
        cpMessage(
            b"cpConstraintIsDampedSpring(constraint)\0" as *const u8
                as *const libc::c_char,
            b"../../src/cpDampedSpring.c\0" as *const u8 as *const libc::c_char,
            176 as libc::c_int,
            1 as libc::c_int,
            1 as libc::c_int,
            b"Constraint is not a damped spring.\0" as *const u8 as *const libc::c_char,
        );
        abort();
    }
    return (*(constraint as *mut cpDampedSpring)).stiffness;
}
pub unsafe extern "C" fn cpDampedSpringSetStiffness(
    mut constraint: *mut cpConstraint,
    mut stiffness: cpFloat,
) {
    if cpConstraintIsDampedSpring(constraint) == 0 {
        cpMessage(
            b"cpConstraintIsDampedSpring(constraint)\0" as *const u8
                as *const libc::c_char,
            b"../../src/cpDampedSpring.c\0" as *const u8 as *const libc::c_char,
            183 as libc::c_int,
            1 as libc::c_int,
            1 as libc::c_int,
            b"Constraint is not a damped spring.\0" as *const u8 as *const libc::c_char,
        );
        abort();
    }
    cpConstraintActivateBodies(constraint);
    (*(constraint as *mut cpDampedSpring)).stiffness = stiffness;
}
pub unsafe extern "C" fn cpDampedSpringGetDamping(
    mut constraint: *const cpConstraint,
) -> cpFloat {
    if cpConstraintIsDampedSpring(constraint) == 0 {
        cpMessage(
            b"cpConstraintIsDampedSpring(constraint)\0" as *const u8
                as *const libc::c_char,
            b"../../src/cpDampedSpring.c\0" as *const u8 as *const libc::c_char,
            191 as libc::c_int,
            1 as libc::c_int,
            1 as libc::c_int,
            b"Constraint is not a damped spring.\0" as *const u8 as *const libc::c_char,
        );
        abort();
    }
    return (*(constraint as *mut cpDampedSpring)).damping;
}
pub unsafe extern "C" fn cpDampedSpringSetDamping(
    mut constraint: *mut cpConstraint,
    mut damping: cpFloat,
) {
    if cpConstraintIsDampedSpring(constraint) == 0 {
        cpMessage(
            b"cpConstraintIsDampedSpring(constraint)\0" as *const u8
                as *const libc::c_char,
            b"../../src/cpDampedSpring.c\0" as *const u8 as *const libc::c_char,
            198 as libc::c_int,
            1 as libc::c_int,
            1 as libc::c_int,
            b"Constraint is not a damped spring.\0" as *const u8 as *const libc::c_char,
        );
        abort();
    }
    cpConstraintActivateBodies(constraint);
    (*(constraint as *mut cpDampedSpring)).damping = damping;
}
pub unsafe extern "C" fn cpDampedSpringGetSpringForceFunc(
    mut constraint: *const cpConstraint,
) -> cpDampedSpringForceFunc {
    if cpConstraintIsDampedSpring(constraint) == 0 {
        cpMessage(
            b"cpConstraintIsDampedSpring(constraint)\0" as *const u8
                as *const libc::c_char,
            b"../../src/cpDampedSpring.c\0" as *const u8 as *const libc::c_char,
            206 as libc::c_int,
            1 as libc::c_int,
            1 as libc::c_int,
            b"Constraint is not a damped spring.\0" as *const u8 as *const libc::c_char,
        );
        abort();
    }
    return (*(constraint as *mut cpDampedSpring)).springForceFunc;
}
pub unsafe extern "C" fn cpDampedSpringSetSpringForceFunc(
    mut constraint: *mut cpConstraint,
    mut springForceFunc: cpDampedSpringForceFunc,
) {
    if cpConstraintIsDampedSpring(constraint) == 0 {
        cpMessage(
            b"cpConstraintIsDampedSpring(constraint)\0" as *const u8
                as *const libc::c_char,
            b"../../src/cpDampedSpring.c\0" as *const u8 as *const libc::c_char,
            213 as libc::c_int,
            1 as libc::c_int,
            1 as libc::c_int,
            b"Constraint is not a damped spring.\0" as *const u8 as *const libc::c_char,
        );
        abort();
    }
    cpConstraintActivateBodies(constraint);
    let ref mut fresh0 = (*(constraint as *mut cpDampedSpring)).springForceFunc;
    *fresh0 = springForceFunc;
}
