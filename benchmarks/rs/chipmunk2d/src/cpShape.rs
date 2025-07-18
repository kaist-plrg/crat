use ::libc;
extern "C" {
    pub type cpHashSet;
    pub type cpContactBufferHeader;
    fn sqrt(_: libc::c_double) -> libc::c_double;
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
    fn cpBodyActivate(body: *mut cpBody);
    fn cpCollide(
        a: *const cpShape,
        b: *const cpShape,
        id: cpCollisionID,
        contacts: *mut cpContact,
    ) -> cpCollisionInfo;
    fn cpBodyAccumulateMassFromShapes(body: *mut cpBody);
    fn cpAreaForCircle(r1: cpFloat, r2: cpFloat) -> cpFloat;
    fn cpMomentForCircle(
        m: cpFloat,
        r1: cpFloat,
        r2: cpFloat,
        offset: cpVect,
    ) -> cpFloat;
    fn cpAreaForSegment(a: cpVect, b: cpVect, radius: cpFloat) -> cpFloat;
    fn cpMomentForBox(m: cpFloat, width: cpFloat, height: cpFloat) -> cpFloat;
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
pub struct cpCircleShape {
    pub shape: cpShape,
    pub c: cpVect,
    pub tc: cpVect,
    pub r: cpFloat,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct cpSegmentShape {
    pub shape: cpShape,
    pub a: cpVect,
    pub b: cpVect,
    pub n: cpVect,
    pub ta: cpVect,
    pub tb: cpVect,
    pub tn: cpVect,
    pub r: cpFloat,
    pub a_tangent: cpVect,
    pub b_tangent: cpVect,
}
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
unsafe extern "C" fn cpfclamp01(mut f: cpFloat) -> cpFloat {
    return cpfmax(0.0f32 as cpFloat, cpfmin(f, 1.0f32 as cpFloat));
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
unsafe extern "C" fn cpvrperp(v: cpVect) -> cpVect {
    return cpv(v.y, -v.x);
}
#[inline]
unsafe extern "C" fn cpvlengthsq(v: cpVect) -> cpFloat {
    return cpvdot(v, v);
}
#[inline]
unsafe extern "C" fn cpvlength(v: cpVect) -> cpFloat {
    return sqrt(cpvdot(v, v));
}
#[inline]
unsafe extern "C" fn cpvlerp(v1: cpVect, v2: cpVect, t: cpFloat) -> cpVect {
    return cpvadd(cpvmult(v1, 1.0f32 as libc::c_double - t), cpvmult(v2, t));
}
#[inline]
unsafe extern "C" fn cpvnormalize(v: cpVect) -> cpVect {
    return cpvmult(
        v,
        1.0f32 as libc::c_double / (cpvlength(v) + 2.2250738585072014e-308f64),
    );
}
#[inline]
unsafe extern "C" fn cpvdist(v1: cpVect, v2: cpVect) -> cpFloat {
    return cpvlength(cpvsub(v1, v2));
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
unsafe extern "C" fn cpBBNewForExtents(c: cpVect, hw: cpFloat, hh: cpFloat) -> cpBB {
    return cpBBNew(c.x - hw, c.y - hh, c.x + hw, c.y + hh);
}
#[inline]
unsafe extern "C" fn cpBBNewForCircle(p: cpVect, r: cpFloat) -> cpBB {
    return cpBBNewForExtents(p, r, r);
}
#[inline]
unsafe extern "C" fn cpTransformPoint(mut t: cpTransform, mut p: cpVect) -> cpVect {
    return cpv(t.a * p.x + t.c * p.y + t.tx, t.b * p.x + t.d * p.y + t.ty);
}
#[inline]
unsafe extern "C" fn cpTransformVect(mut t: cpTransform, mut v: cpVect) -> cpVect {
    return cpv(t.a * v.x + t.c * v.y, t.b * v.x + t.d * v.y);
}
#[inline]
unsafe extern "C" fn cpShapeActive(mut shape: *mut cpShape) -> cpBool {
    return (!((*shape).prev).is_null()
        || !((*shape).body).is_null() && (*(*shape).body).shapeList == shape)
        as libc::c_int as cpBool;
}
#[inline]
unsafe extern "C" fn CircleSegmentQuery(
    mut shape: *mut cpShape,
    mut center: cpVect,
    mut r1: cpFloat,
    mut a: cpVect,
    mut b: cpVect,
    mut r2: cpFloat,
    mut info: *mut cpSegmentQueryInfo,
) {
    let mut da: cpVect = cpvsub(a, center);
    let mut db: cpVect = cpvsub(b, center);
    let mut rsum: cpFloat = r1 + r2;
    let mut qa: cpFloat = cpvdot(da, da) - 2.0f32 as libc::c_double * cpvdot(da, db)
        + cpvdot(db, db);
    let mut qb: cpFloat = cpvdot(da, db) - cpvdot(da, da);
    let mut det: cpFloat = qb * qb - qa * (cpvdot(da, da) - rsum * rsum);
    if det >= 0.0f32 as libc::c_double {
        let mut t: cpFloat = (-qb - sqrt(det)) / qa;
        if 0.0f32 as libc::c_double <= t && t <= 1.0f32 as libc::c_double {
            let mut n: cpVect = cpvnormalize(cpvlerp(da, db, t));
            (*info).shape = shape;
            (*info).point = cpvsub(cpvlerp(a, b, t), cpvmult(n, r2));
            (*info).normal = n;
            (*info).alpha = t;
        }
    }
}
#[inline]
unsafe extern "C" fn cpClosetPointOnSegment(p: cpVect, a: cpVect, b: cpVect) -> cpVect {
    let mut delta: cpVect = cpvsub(a, b);
    let mut t: cpFloat = cpfclamp01(cpvdot(delta, cpvsub(p, b)) / cpvlengthsq(delta));
    return cpvadd(b, cpvmult(delta, t));
}
pub unsafe extern "C" fn cpShapeInit(
    mut shape: *mut cpShape,
    mut klass: *const cpShapeClass,
    mut body: *mut cpBody,
    mut massInfo: cpShapeMassInfo,
) -> *mut cpShape {
    (*shape).klass = klass;
    (*shape).body = body;
    (*shape).massInfo = massInfo;
    (*shape).sensor = 0 as libc::c_int as cpBool;
    (*shape).e = 0.0f32 as cpFloat;
    (*shape).u = 0.0f32 as cpFloat;
    (*shape).surfaceV = cpvzero;
    (*shape).type_0 = 0 as libc::c_int as cpCollisionType;
    (*shape).filter.group = 0 as libc::c_int as cpGroup;
    (*shape).filter.categories = !(0 as libc::c_int as cpBitmask);
    (*shape).filter.mask = !(0 as libc::c_int as cpBitmask);
    (*shape).userData = 0 as *mut libc::c_void;
    (*shape).space = 0 as *mut cpSpace;
    (*shape).next = 0 as *mut cpShape;
    (*shape).prev = 0 as *mut cpShape;
    return shape;
}
pub unsafe extern "C" fn cpShapeDestroy(mut shape: *mut cpShape) {
    if !((*shape).klass).is_null() && ((*(*shape).klass).destroy).is_some() {
        ((*(*shape).klass).destroy).unwrap()(shape);
    }
}
pub unsafe extern "C" fn cpShapeFree(mut shape: *mut cpShape) {
    if !shape.is_null() {
        cpShapeDestroy(shape);
        free(shape as *mut libc::c_void);
    }
}
pub unsafe extern "C" fn cpShapeGetSpace(mut shape: *const cpShape) -> *mut cpSpace {
    return (*shape).space;
}
pub unsafe extern "C" fn cpShapeGetBody(mut shape: *const cpShape) -> *mut cpBody {
    return (*shape).body;
}
pub unsafe extern "C" fn cpShapeSetBody(mut shape: *mut cpShape, mut body: *mut cpBody) {
    if cpShapeActive(shape) != 0 {
        cpMessage(
            b"!cpShapeActive(shape)\0" as *const u8 as *const libc::c_char,
            b"../../src/cpShape.c\0" as *const u8 as *const libc::c_char,
            90 as libc::c_int,
            1 as libc::c_int,
            1 as libc::c_int,
            b"You cannot change the body on an active shape. You must remove the shape from the space before changing the body.\0"
                as *const u8 as *const libc::c_char,
        );
        abort();
    }
    (*shape).body = body;
}
pub unsafe extern "C" fn cpShapeGetMass(mut shape: *mut cpShape) -> cpFloat {
    return (*shape).massInfo.m;
}
pub unsafe extern "C" fn cpShapeSetMass(mut shape: *mut cpShape, mut mass: cpFloat) {
    let mut body: *mut cpBody = (*shape).body;
    cpBodyActivate(body);
    (*shape).massInfo.m = mass;
    cpBodyAccumulateMassFromShapes(body);
}
pub unsafe extern "C" fn cpShapeGetDensity(mut shape: *mut cpShape) -> cpFloat {
    return (*shape).massInfo.m / (*shape).massInfo.area;
}
pub unsafe extern "C" fn cpShapeSetDensity(
    mut shape: *mut cpShape,
    mut density: cpFloat,
) {
    cpShapeSetMass(shape, density * (*shape).massInfo.area);
}
pub unsafe extern "C" fn cpShapeGetMoment(mut shape: *mut cpShape) -> cpFloat {
    return (*shape).massInfo.m * (*shape).massInfo.i;
}
pub unsafe extern "C" fn cpShapeGetArea(mut shape: *mut cpShape) -> cpFloat {
    return (*shape).massInfo.area;
}
pub unsafe extern "C" fn cpShapeGetCenterOfGravity(mut shape: *mut cpShape) -> cpVect {
    return (*shape).massInfo.cog;
}
pub unsafe extern "C" fn cpShapeGetBB(mut shape: *const cpShape) -> cpBB {
    return (*shape).bb;
}
pub unsafe extern "C" fn cpShapeGetSensor(mut shape: *const cpShape) -> cpBool {
    return (*shape).sensor;
}
pub unsafe extern "C" fn cpShapeSetSensor(mut shape: *mut cpShape, mut sensor: cpBool) {
    cpBodyActivate((*shape).body);
    (*shape).sensor = sensor;
}
pub unsafe extern "C" fn cpShapeGetElasticity(mut shape: *const cpShape) -> cpFloat {
    return (*shape).e;
}
pub unsafe extern "C" fn cpShapeSetElasticity(
    mut shape: *mut cpShape,
    mut elasticity: cpFloat,
) {
    if !(elasticity >= 0.0f32 as libc::c_double) {
        cpMessage(
            b"elasticity >= 0.0f\0" as *const u8 as *const libc::c_char,
            b"../../src/cpShape.c\0" as *const u8 as *const libc::c_char,
            140 as libc::c_int,
            1 as libc::c_int,
            1 as libc::c_int,
            b"Elasticity must be positive.\0" as *const u8 as *const libc::c_char,
        );
        abort();
    }
    cpBodyActivate((*shape).body);
    (*shape).e = elasticity;
}
pub unsafe extern "C" fn cpShapeGetFriction(mut shape: *const cpShape) -> cpFloat {
    return (*shape).u;
}
pub unsafe extern "C" fn cpShapeSetFriction(
    mut shape: *mut cpShape,
    mut friction: cpFloat,
) {
    if !(friction >= 0.0f32 as libc::c_double) {
        cpMessage(
            b"friction >= 0.0f\0" as *const u8 as *const libc::c_char,
            b"../../src/cpShape.c\0" as *const u8 as *const libc::c_char,
            154 as libc::c_int,
            1 as libc::c_int,
            1 as libc::c_int,
            b"Friction must be postive.\0" as *const u8 as *const libc::c_char,
        );
        abort();
    }
    cpBodyActivate((*shape).body);
    (*shape).u = friction;
}
pub unsafe extern "C" fn cpShapeGetSurfaceVelocity(mut shape: *const cpShape) -> cpVect {
    return (*shape).surfaceV;
}
pub unsafe extern "C" fn cpShapeSetSurfaceVelocity(
    mut shape: *mut cpShape,
    mut surfaceVelocity: cpVect,
) {
    cpBodyActivate((*shape).body);
    (*shape).surfaceV = surfaceVelocity;
}
pub unsafe extern "C" fn cpShapeGetUserData(mut shape: *const cpShape) -> cpDataPointer {
    return (*shape).userData;
}
pub unsafe extern "C" fn cpShapeSetUserData(
    mut shape: *mut cpShape,
    mut userData: cpDataPointer,
) {
    (*shape).userData = userData;
}
pub unsafe extern "C" fn cpShapeGetCollisionType(
    mut shape: *const cpShape,
) -> cpCollisionType {
    return (*shape).type_0;
}
pub unsafe extern "C" fn cpShapeSetCollisionType(
    mut shape: *mut cpShape,
    mut collisionType: cpCollisionType,
) {
    cpBodyActivate((*shape).body);
    (*shape).type_0 = collisionType;
}
pub unsafe extern "C" fn cpShapeGetFilter(mut shape: *const cpShape) -> cpShapeFilter {
    return (*shape).filter;
}
pub unsafe extern "C" fn cpShapeSetFilter(
    mut shape: *mut cpShape,
    mut filter: cpShapeFilter,
) {
    cpBodyActivate((*shape).body);
    (*shape).filter = filter;
}
pub unsafe extern "C" fn cpShapeCacheBB(mut shape: *mut cpShape) -> cpBB {
    return cpShapeUpdate(shape, (*(*shape).body).transform);
}
pub unsafe extern "C" fn cpShapeUpdate(
    mut shape: *mut cpShape,
    mut transform: cpTransform,
) -> cpBB {
    (*shape).bb = ((*(*shape).klass).cacheData).unwrap()(shape, transform);
    return (*shape).bb;
}
pub unsafe extern "C" fn cpShapePointQuery(
    mut shape: *const cpShape,
    mut p: cpVect,
    mut info: *mut cpPointQueryInfo,
) -> cpFloat {
    let mut blank: cpPointQueryInfo = {
        let mut init = cpPointQueryInfo {
            shape: 0 as *const cpShape,
            point: cpvzero,
            distance: ::std::f32::INFINITY as cpFloat,
            gradient: cpvzero,
        };
        init
    };
    if !info.is_null() {
        *info = blank;
    } else {
        info = &mut blank;
    }
    ((*(*shape).klass).pointQuery).unwrap()(shape, p, info);
    return (*info).distance;
}
pub unsafe extern "C" fn cpShapeSegmentQuery(
    mut shape: *const cpShape,
    mut a: cpVect,
    mut b: cpVect,
    mut radius: cpFloat,
    mut info: *mut cpSegmentQueryInfo,
) -> cpBool {
    let mut blank: cpSegmentQueryInfo = {
        let mut init = cpSegmentQueryInfo {
            shape: 0 as *const cpShape,
            point: b,
            normal: cpvzero,
            alpha: 1.0f32 as cpFloat,
        };
        init
    };
    if !info.is_null() {
        *info = blank;
    } else {
        info = &mut blank;
    }
    let mut nearest: cpPointQueryInfo = cpPointQueryInfo {
        shape: 0 as *const cpShape,
        point: cpVect { x: 0., y: 0. },
        distance: 0.,
        gradient: cpVect { x: 0., y: 0. },
    };
    ((*(*shape).klass).pointQuery).unwrap()(shape, a, &mut nearest);
    if nearest.distance <= radius {
        (*info).shape = shape;
        (*info).alpha = 0.0f64;
        (*info).normal = cpvnormalize(cpvsub(a, nearest.point));
    } else {
        ((*(*shape).klass).segmentQuery).unwrap()(shape, a, b, radius, info);
    }
    return ((*info).shape != 0 as *mut libc::c_void as *const cpShape) as libc::c_int
        as cpBool;
}
pub unsafe extern "C" fn cpShapesCollide(
    mut a: *const cpShape,
    mut b: *const cpShape,
) -> cpContactPointSet {
    let mut contacts: [cpContact; 2] = [cpContact {
        r1: cpVect { x: 0., y: 0. },
        r2: cpVect { x: 0., y: 0. },
        nMass: 0.,
        tMass: 0.,
        bounce: 0.,
        jnAcc: 0.,
        jtAcc: 0.,
        jBias: 0.,
        bias: 0.,
        hash: 0,
    }; 2];
    let mut info: cpCollisionInfo = cpCollide(
        a,
        b,
        0 as libc::c_int as cpCollisionID,
        contacts.as_mut_ptr(),
    );
    let mut set: cpContactPointSet = cpContactPointSet {
        count: 0,
        normal: cpVect { x: 0., y: 0. },
        points: [C2RustUnnamed_0 {
            pointA: cpVect { x: 0., y: 0. },
            pointB: cpVect { x: 0., y: 0. },
            distance: 0.,
        }; 2],
    };
    set.count = info.count;
    let mut swapped: cpBool = (a != info.a) as libc::c_int as cpBool;
    set.normal = if swapped as libc::c_int != 0 { cpvneg(info.n) } else { info.n };
    let mut i: libc::c_int = 0 as libc::c_int;
    while i < info.count {
        let mut p1: cpVect = contacts[i as usize].r1;
        let mut p2: cpVect = contacts[i as usize].r2;
        set
            .points[i as usize]
            .pointA = if swapped as libc::c_int != 0 { p2 } else { p1 };
        set
            .points[i as usize]
            .pointB = if swapped as libc::c_int != 0 { p1 } else { p2 };
        set.points[i as usize].distance = cpvdot(cpvsub(p2, p1), set.normal);
        i += 1;
        i;
    }
    return set;
}
pub unsafe extern "C" fn cpCircleShapeAlloc() -> *mut cpCircleShape {
    return calloc(
        1 as libc::c_int as libc::c_ulong,
        ::std::mem::size_of::<cpCircleShape>() as libc::c_ulong,
    ) as *mut cpCircleShape;
}
unsafe extern "C" fn cpCircleShapeCacheData(
    mut circle: *mut cpCircleShape,
    mut transform: cpTransform,
) -> cpBB {
    (*circle).tc = cpTransformPoint(transform, (*circle).c);
    let mut c: cpVect = (*circle).tc;
    return cpBBNewForCircle(c, (*circle).r);
}
unsafe extern "C" fn cpCircleShapePointQuery(
    mut circle: *mut cpCircleShape,
    mut p: cpVect,
    mut info: *mut cpPointQueryInfo,
) {
    let mut delta: cpVect = cpvsub(p, (*circle).tc);
    let mut d: cpFloat = cpvlength(delta);
    let mut r: cpFloat = (*circle).r;
    (*info).shape = circle as *mut cpShape;
    let mut r_over_d: cpFloat = if d > 0.0f32 as libc::c_double { r / d } else { r };
    (*info).point = cpvadd((*circle).tc, cpvmult(delta, r_over_d));
    (*info).distance = d - r;
    (*info)
        .gradient = if d > 1e-5f64 {
        cpvmult(delta, 1.0f32 as libc::c_double / d)
    } else {
        cpv(0.0f32 as cpFloat, 1.0f32 as cpFloat)
    };
}
unsafe extern "C" fn cpCircleShapeSegmentQuery(
    mut circle: *mut cpCircleShape,
    mut a: cpVect,
    mut b: cpVect,
    mut radius: cpFloat,
    mut info: *mut cpSegmentQueryInfo,
) {
    CircleSegmentQuery(
        circle as *mut cpShape,
        (*circle).tc,
        (*circle).r,
        a,
        b,
        radius,
        info,
    );
}
unsafe extern "C" fn cpCircleShapeMassInfo(
    mut mass: cpFloat,
    mut radius: cpFloat,
    mut center: cpVect,
) -> cpShapeMassInfo {
    let mut info: cpShapeMassInfo = {
        let mut init = cpShapeMassInfo {
            m: mass,
            i: cpMomentForCircle(1.0f32 as cpFloat, 0.0f32 as cpFloat, radius, cpvzero),
            cog: center,
            area: cpAreaForCircle(0.0f32 as cpFloat, radius),
        };
        init
    };
    return info;
}
static mut cpCircleShapeClass: cpShapeClass = unsafe {
    {
        let mut init = cpShapeClass {
            type_0: CP_CIRCLE_SHAPE,
            cacheData: ::std::mem::transmute::<
                Option::<unsafe extern "C" fn(*mut cpCircleShape, cpTransform) -> cpBB>,
                cpShapeCacheDataImpl,
            >(
                Some(
                    cpCircleShapeCacheData
                        as unsafe extern "C" fn(*mut cpCircleShape, cpTransform) -> cpBB,
                ),
            ),
            destroy: None,
            pointQuery: ::std::mem::transmute::<
                Option::<
                    unsafe extern "C" fn(
                        *mut cpCircleShape,
                        cpVect,
                        *mut cpPointQueryInfo,
                    ) -> (),
                >,
                cpShapePointQueryImpl,
            >(
                Some(
                    cpCircleShapePointQuery
                        as unsafe extern "C" fn(
                            *mut cpCircleShape,
                            cpVect,
                            *mut cpPointQueryInfo,
                        ) -> (),
                ),
            ),
            segmentQuery: ::std::mem::transmute::<
                Option::<
                    unsafe extern "C" fn(
                        *mut cpCircleShape,
                        cpVect,
                        cpVect,
                        cpFloat,
                        *mut cpSegmentQueryInfo,
                    ) -> (),
                >,
                cpShapeSegmentQueryImpl,
            >(
                Some(
                    cpCircleShapeSegmentQuery
                        as unsafe extern "C" fn(
                            *mut cpCircleShape,
                            cpVect,
                            cpVect,
                            cpFloat,
                            *mut cpSegmentQueryInfo,
                        ) -> (),
                ),
            ),
        };
        init
    }
};
pub unsafe extern "C" fn cpCircleShapeInit(
    mut circle: *mut cpCircleShape,
    mut body: *mut cpBody,
    mut radius: cpFloat,
    mut offset: cpVect,
) -> *mut cpCircleShape {
    (*circle).c = offset;
    (*circle).r = radius;
    cpShapeInit(
        circle as *mut cpShape,
        &cpCircleShapeClass,
        body,
        cpCircleShapeMassInfo(0.0f32 as cpFloat, radius, offset),
    );
    return circle;
}
pub unsafe extern "C" fn cpCircleShapeNew(
    mut body: *mut cpBody,
    mut radius: cpFloat,
    mut offset: cpVect,
) -> *mut cpShape {
    return cpCircleShapeInit(cpCircleShapeAlloc(), body, radius, offset) as *mut cpShape;
}
pub unsafe extern "C" fn cpCircleShapeGetOffset(mut shape: *const cpShape) -> cpVect {
    if !((*shape).klass == &cpCircleShapeClass as *const cpShapeClass) {
        cpMessage(
            b"shape->klass == &cpCircleShapeClass\0" as *const u8 as *const libc::c_char,
            b"../../src/cpShape.c\0" as *const u8 as *const libc::c_char,
            360 as libc::c_int,
            1 as libc::c_int,
            1 as libc::c_int,
            b"Shape is not a circle shape.\0" as *const u8 as *const libc::c_char,
        );
        abort();
    }
    return (*(shape as *mut cpCircleShape)).c;
}
pub unsafe extern "C" fn cpCircleShapeGetRadius(mut shape: *const cpShape) -> cpFloat {
    if !((*shape).klass == &cpCircleShapeClass as *const cpShapeClass) {
        cpMessage(
            b"shape->klass == &cpCircleShapeClass\0" as *const u8 as *const libc::c_char,
            b"../../src/cpShape.c\0" as *const u8 as *const libc::c_char,
            367 as libc::c_int,
            1 as libc::c_int,
            1 as libc::c_int,
            b"Shape is not a circle shape.\0" as *const u8 as *const libc::c_char,
        );
        abort();
    }
    return (*(shape as *mut cpCircleShape)).r;
}
pub unsafe extern "C" fn cpSegmentShapeAlloc() -> *mut cpSegmentShape {
    return calloc(
        1 as libc::c_int as libc::c_ulong,
        ::std::mem::size_of::<cpSegmentShape>() as libc::c_ulong,
    ) as *mut cpSegmentShape;
}
unsafe extern "C" fn cpSegmentShapeCacheData(
    mut seg: *mut cpSegmentShape,
    mut transform: cpTransform,
) -> cpBB {
    (*seg).ta = cpTransformPoint(transform, (*seg).a);
    (*seg).tb = cpTransformPoint(transform, (*seg).b);
    (*seg).tn = cpTransformVect(transform, (*seg).n);
    let mut l: cpFloat = 0.;
    let mut r: cpFloat = 0.;
    let mut b: cpFloat = 0.;
    let mut t: cpFloat = 0.;
    if (*seg).ta.x < (*seg).tb.x {
        l = (*seg).ta.x;
        r = (*seg).tb.x;
    } else {
        l = (*seg).tb.x;
        r = (*seg).ta.x;
    }
    if (*seg).ta.y < (*seg).tb.y {
        b = (*seg).ta.y;
        t = (*seg).tb.y;
    } else {
        b = (*seg).tb.y;
        t = (*seg).ta.y;
    }
    let mut rad: cpFloat = (*seg).r;
    return cpBBNew(l - rad, b - rad, r + rad, t + rad);
}
unsafe extern "C" fn cpSegmentShapePointQuery(
    mut seg: *mut cpSegmentShape,
    mut p: cpVect,
    mut info: *mut cpPointQueryInfo,
) {
    let mut closest: cpVect = cpClosetPointOnSegment(p, (*seg).ta, (*seg).tb);
    let mut delta: cpVect = cpvsub(p, closest);
    let mut d: cpFloat = cpvlength(delta);
    let mut r: cpFloat = (*seg).r;
    let mut g: cpVect = cpvmult(delta, 1.0f32 as libc::c_double / d);
    (*info).shape = seg as *mut cpShape;
    (*info).point = if d != 0. { cpvadd(closest, cpvmult(g, r)) } else { closest };
    (*info).distance = d - r;
    (*info).gradient = if d > 1e-5f64 { g } else { (*seg).n };
}
unsafe extern "C" fn cpSegmentShapeSegmentQuery(
    mut seg: *mut cpSegmentShape,
    mut a: cpVect,
    mut b: cpVect,
    mut r2: cpFloat,
    mut info: *mut cpSegmentQueryInfo,
) {
    let mut n: cpVect = (*seg).tn;
    let mut d: cpFloat = cpvdot(cpvsub((*seg).ta, a), n);
    let mut r: cpFloat = (*seg).r + r2;
    let mut flipped_n: cpVect = if d > 0.0f32 as libc::c_double { cpvneg(n) } else { n };
    let mut seg_offset: cpVect = cpvsub(cpvmult(flipped_n, r), a);
    let mut seg_a: cpVect = cpvadd((*seg).ta, seg_offset);
    let mut seg_b: cpVect = cpvadd((*seg).tb, seg_offset);
    let mut delta: cpVect = cpvsub(b, a);
    if cpvcross(delta, seg_a) * cpvcross(delta, seg_b) <= 0.0f32 as libc::c_double {
        let mut d_offset: cpFloat = d
            + (if d > 0.0f32 as libc::c_double { -r } else { r });
        let mut ad: cpFloat = -d_offset;
        let mut bd: cpFloat = cpvdot(delta, n) - d_offset;
        if ad * bd < 0.0f32 as libc::c_double {
            let mut t: cpFloat = ad / (ad - bd);
            (*info).shape = seg as *mut cpShape;
            (*info).point = cpvsub(cpvlerp(a, b, t), cpvmult(flipped_n, r2));
            (*info).normal = flipped_n;
            (*info).alpha = t;
        }
    } else if r != 0.0f32 as libc::c_double {
        let mut info1: cpSegmentQueryInfo = {
            let mut init = cpSegmentQueryInfo {
                shape: 0 as *const cpShape,
                point: b,
                normal: cpvzero,
                alpha: 1.0f32 as cpFloat,
            };
            init
        };
        let mut info2: cpSegmentQueryInfo = {
            let mut init = cpSegmentQueryInfo {
                shape: 0 as *const cpShape,
                point: b,
                normal: cpvzero,
                alpha: 1.0f32 as cpFloat,
            };
            init
        };
        CircleSegmentQuery(
            seg as *mut cpShape,
            (*seg).ta,
            (*seg).r,
            a,
            b,
            r2,
            &mut info1,
        );
        CircleSegmentQuery(
            seg as *mut cpShape,
            (*seg).tb,
            (*seg).r,
            a,
            b,
            r2,
            &mut info2,
        );
        if info1.alpha < info2.alpha {
            *info = info1;
        } else {
            *info = info2;
        }
    }
}
unsafe extern "C" fn cpSegmentShapeMassInfo(
    mut mass: cpFloat,
    mut a: cpVect,
    mut b: cpVect,
    mut r: cpFloat,
) -> cpShapeMassInfo {
    let mut info: cpShapeMassInfo = {
        let mut init = cpShapeMassInfo {
            m: mass,
            i: cpMomentForBox(
                1.0f32 as cpFloat,
                cpvdist(a, b) + 2.0f32 as libc::c_double * r,
                2.0f32 as libc::c_double * r,
            ),
            cog: cpvlerp(a, b, 0.5f32 as cpFloat),
            area: cpAreaForSegment(a, b, r),
        };
        init
    };
    return info;
}
static mut cpSegmentShapeClass: cpShapeClass = unsafe {
    {
        let mut init = cpShapeClass {
            type_0: CP_SEGMENT_SHAPE,
            cacheData: ::std::mem::transmute::<
                Option::<unsafe extern "C" fn(*mut cpSegmentShape, cpTransform) -> cpBB>,
                cpShapeCacheDataImpl,
            >(
                Some(
                    cpSegmentShapeCacheData
                        as unsafe extern "C" fn(*mut cpSegmentShape, cpTransform) -> cpBB,
                ),
            ),
            destroy: None,
            pointQuery: ::std::mem::transmute::<
                Option::<
                    unsafe extern "C" fn(
                        *mut cpSegmentShape,
                        cpVect,
                        *mut cpPointQueryInfo,
                    ) -> (),
                >,
                cpShapePointQueryImpl,
            >(
                Some(
                    cpSegmentShapePointQuery
                        as unsafe extern "C" fn(
                            *mut cpSegmentShape,
                            cpVect,
                            *mut cpPointQueryInfo,
                        ) -> (),
                ),
            ),
            segmentQuery: ::std::mem::transmute::<
                Option::<
                    unsafe extern "C" fn(
                        *mut cpSegmentShape,
                        cpVect,
                        cpVect,
                        cpFloat,
                        *mut cpSegmentQueryInfo,
                    ) -> (),
                >,
                cpShapeSegmentQueryImpl,
            >(
                Some(
                    cpSegmentShapeSegmentQuery
                        as unsafe extern "C" fn(
                            *mut cpSegmentShape,
                            cpVect,
                            cpVect,
                            cpFloat,
                            *mut cpSegmentQueryInfo,
                        ) -> (),
                ),
            ),
        };
        init
    }
};
pub unsafe extern "C" fn cpSegmentShapeInit(
    mut seg: *mut cpSegmentShape,
    mut body: *mut cpBody,
    mut a: cpVect,
    mut b: cpVect,
    mut r: cpFloat,
) -> *mut cpSegmentShape {
    (*seg).a = a;
    (*seg).b = b;
    (*seg).n = cpvrperp(cpvnormalize(cpvsub(b, a)));
    (*seg).r = r;
    (*seg).a_tangent = cpvzero;
    (*seg).b_tangent = cpvzero;
    cpShapeInit(
        seg as *mut cpShape,
        &cpSegmentShapeClass,
        body,
        cpSegmentShapeMassInfo(0.0f32 as cpFloat, a, b, r),
    );
    return seg;
}
pub unsafe extern "C" fn cpSegmentShapeNew(
    mut body: *mut cpBody,
    mut a: cpVect,
    mut b: cpVect,
    mut r: cpFloat,
) -> *mut cpShape {
    return cpSegmentShapeInit(cpSegmentShapeAlloc(), body, a, b, r) as *mut cpShape;
}
pub unsafe extern "C" fn cpSegmentShapeGetA(mut shape: *const cpShape) -> cpVect {
    if !((*shape).klass == &cpSegmentShapeClass as *const cpShapeClass) {
        cpMessage(
            b"shape->klass == &cpSegmentShapeClass\0" as *const u8
                as *const libc::c_char,
            b"../../src/cpShape.c\0" as *const u8 as *const libc::c_char,
            513 as libc::c_int,
            1 as libc::c_int,
            1 as libc::c_int,
            b"Shape is not a segment shape.\0" as *const u8 as *const libc::c_char,
        );
        abort();
    }
    return (*(shape as *mut cpSegmentShape)).a;
}
pub unsafe extern "C" fn cpSegmentShapeGetB(mut shape: *const cpShape) -> cpVect {
    if !((*shape).klass == &cpSegmentShapeClass as *const cpShapeClass) {
        cpMessage(
            b"shape->klass == &cpSegmentShapeClass\0" as *const u8
                as *const libc::c_char,
            b"../../src/cpShape.c\0" as *const u8 as *const libc::c_char,
            520 as libc::c_int,
            1 as libc::c_int,
            1 as libc::c_int,
            b"Shape is not a segment shape.\0" as *const u8 as *const libc::c_char,
        );
        abort();
    }
    return (*(shape as *mut cpSegmentShape)).b;
}
pub unsafe extern "C" fn cpSegmentShapeGetNormal(mut shape: *const cpShape) -> cpVect {
    if !((*shape).klass == &cpSegmentShapeClass as *const cpShapeClass) {
        cpMessage(
            b"shape->klass == &cpSegmentShapeClass\0" as *const u8
                as *const libc::c_char,
            b"../../src/cpShape.c\0" as *const u8 as *const libc::c_char,
            527 as libc::c_int,
            1 as libc::c_int,
            1 as libc::c_int,
            b"Shape is not a segment shape.\0" as *const u8 as *const libc::c_char,
        );
        abort();
    }
    return (*(shape as *mut cpSegmentShape)).n;
}
pub unsafe extern "C" fn cpSegmentShapeGetRadius(mut shape: *const cpShape) -> cpFloat {
    if !((*shape).klass == &cpSegmentShapeClass as *const cpShapeClass) {
        cpMessage(
            b"shape->klass == &cpSegmentShapeClass\0" as *const u8
                as *const libc::c_char,
            b"../../src/cpShape.c\0" as *const u8 as *const libc::c_char,
            534 as libc::c_int,
            1 as libc::c_int,
            1 as libc::c_int,
            b"Shape is not a segment shape.\0" as *const u8 as *const libc::c_char,
        );
        abort();
    }
    return (*(shape as *mut cpSegmentShape)).r;
}
pub unsafe extern "C" fn cpSegmentShapeSetNeighbors(
    mut shape: *mut cpShape,
    mut prev: cpVect,
    mut next: cpVect,
) {
    if !((*shape).klass == &cpSegmentShapeClass as *const cpShapeClass) {
        cpMessage(
            b"shape->klass == &cpSegmentShapeClass\0" as *const u8
                as *const libc::c_char,
            b"../../src/cpShape.c\0" as *const u8 as *const libc::c_char,
            541 as libc::c_int,
            1 as libc::c_int,
            1 as libc::c_int,
            b"Shape is not a segment shape.\0" as *const u8 as *const libc::c_char,
        );
        abort();
    }
    let mut seg: *mut cpSegmentShape = shape as *mut cpSegmentShape;
    (*seg).a_tangent = cpvsub(prev, (*seg).a);
    (*seg).b_tangent = cpvsub(next, (*seg).b);
}
pub unsafe extern "C" fn cpCircleShapeSetRadius(
    mut shape: *mut cpShape,
    mut radius: cpFloat,
) {
    if !((*shape).klass == &cpCircleShapeClass as *const cpShapeClass) {
        cpMessage(
            b"shape->klass == &cpCircleShapeClass\0" as *const u8 as *const libc::c_char,
            b"../../src/cpShape.c\0" as *const u8 as *const libc::c_char,
            555 as libc::c_int,
            1 as libc::c_int,
            1 as libc::c_int,
            b"Shape is not a circle shape.\0" as *const u8 as *const libc::c_char,
        );
        abort();
    }
    let mut circle: *mut cpCircleShape = shape as *mut cpCircleShape;
    (*circle).r = radius;
    let mut mass: cpFloat = (*shape).massInfo.m;
    (*shape).massInfo = cpCircleShapeMassInfo(mass, (*circle).r, (*circle).c);
    if mass > 0.0f32 as libc::c_double {
        cpBodyAccumulateMassFromShapes((*shape).body);
    }
}
pub unsafe extern "C" fn cpCircleShapeSetOffset(
    mut shape: *mut cpShape,
    mut offset: cpVect,
) {
    if !((*shape).klass == &cpCircleShapeClass as *const cpShapeClass) {
        cpMessage(
            b"shape->klass == &cpCircleShapeClass\0" as *const u8 as *const libc::c_char,
            b"../../src/cpShape.c\0" as *const u8 as *const libc::c_char,
            568 as libc::c_int,
            1 as libc::c_int,
            1 as libc::c_int,
            b"Shape is not a circle shape.\0" as *const u8 as *const libc::c_char,
        );
        abort();
    }
    let mut circle: *mut cpCircleShape = shape as *mut cpCircleShape;
    (*circle).c = offset;
    let mut mass: cpFloat = (*shape).massInfo.m;
    (*shape)
        .massInfo = cpCircleShapeMassInfo((*shape).massInfo.m, (*circle).r, (*circle).c);
    if mass > 0.0f32 as libc::c_double {
        cpBodyAccumulateMassFromShapes((*shape).body);
    }
}
pub unsafe extern "C" fn cpSegmentShapeSetEndpoints(
    mut shape: *mut cpShape,
    mut a: cpVect,
    mut b: cpVect,
) {
    if !((*shape).klass == &cpSegmentShapeClass as *const cpShapeClass) {
        cpMessage(
            b"shape->klass == &cpSegmentShapeClass\0" as *const u8
                as *const libc::c_char,
            b"../../src/cpShape.c\0" as *const u8 as *const libc::c_char,
            581 as libc::c_int,
            1 as libc::c_int,
            1 as libc::c_int,
            b"Shape is not a segment shape.\0" as *const u8 as *const libc::c_char,
        );
        abort();
    }
    let mut seg: *mut cpSegmentShape = shape as *mut cpSegmentShape;
    (*seg).a = a;
    (*seg).b = b;
    (*seg).n = cpvperp(cpvnormalize(cpvsub(b, a)));
    let mut mass: cpFloat = (*shape).massInfo.m;
    (*shape)
        .massInfo = cpSegmentShapeMassInfo(
        (*shape).massInfo.m,
        (*seg).a,
        (*seg).b,
        (*seg).r,
    );
    if mass > 0.0f32 as libc::c_double {
        cpBodyAccumulateMassFromShapes((*shape).body);
    }
}
pub unsafe extern "C" fn cpSegmentShapeSetRadius(
    mut shape: *mut cpShape,
    mut radius: cpFloat,
) {
    if !((*shape).klass == &cpSegmentShapeClass as *const cpShapeClass) {
        cpMessage(
            b"shape->klass == &cpSegmentShapeClass\0" as *const u8
                as *const libc::c_char,
            b"../../src/cpShape.c\0" as *const u8 as *const libc::c_char,
            596 as libc::c_int,
            1 as libc::c_int,
            1 as libc::c_int,
            b"Shape is not a segment shape.\0" as *const u8 as *const libc::c_char,
        );
        abort();
    }
    let mut seg: *mut cpSegmentShape = shape as *mut cpSegmentShape;
    (*seg).r = radius;
    let mut mass: cpFloat = (*shape).massInfo.m;
    (*shape)
        .massInfo = cpSegmentShapeMassInfo(
        (*shape).massInfo.m,
        (*seg).a,
        (*seg).b,
        (*seg).r,
    );
    if mass > 0.0f32 as libc::c_double {
        cpBodyAccumulateMassFromShapes((*shape).body);
    }
}
