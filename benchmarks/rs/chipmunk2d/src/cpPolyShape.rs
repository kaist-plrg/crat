use ::libc;
extern "C" {
    pub type cpHashSet;
    pub type cpContactBufferHeader;
    fn sqrt(_: libc::c_double) -> libc::c_double;
    fn free(__ptr: *mut libc::c_void);
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
    fn cpConvexHull(
        count: libc::c_int,
        verts: *const cpVect,
        result: *mut cpVect,
        first: *mut libc::c_int,
        tol: cpFloat,
    ) -> libc::c_int;
    fn cpAreaForPoly(
        count: libc::c_int,
        verts: *const cpVect,
        radius: cpFloat,
    ) -> cpFloat;
    fn cpCentroidForPoly(count: libc::c_int, verts: *const cpVect) -> cpVect;
    fn cpMomentForPoly(
        m: cpFloat,
        count: libc::c_int,
        verts: *const cpVect,
        offset: cpVect,
        radius: cpFloat,
    ) -> cpFloat;
    fn cpBodyAccumulateMassFromShapes(body: *mut cpBody);
    fn cpShapeInit(
        shape: *mut cpShape,
        klass: *const cpShapeClass,
        body: *mut cpBody,
        massInfo: cpShapeMassInfo,
    ) -> *mut cpShape;
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
pub struct cpPolyShape {
    pub shape: cpShape,
    pub r: cpFloat,
    pub count: libc::c_int,
    pub planes: *mut cpSplittingPlane,
    pub _planes: [cpSplittingPlane; 12],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct cpSplittingPlane {
    pub v0: cpVect,
    pub n: cpVect,
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
unsafe extern "C" fn cpTransformPoint(mut t: cpTransform, mut p: cpVect) -> cpVect {
    return cpv(t.a * p.x + t.c * p.y + t.tx, t.b * p.x + t.d * p.y + t.ty);
}
#[inline]
unsafe extern "C" fn cpTransformVect(mut t: cpTransform, mut v: cpVect) -> cpVect {
    return cpv(t.a * v.x + t.c * v.y, t.b * v.x + t.d * v.y);
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
pub unsafe extern "C" fn cpPolyShapeAlloc() -> *mut cpPolyShape {
    return calloc(
        1 as libc::c_int as libc::c_ulong,
        ::std::mem::size_of::<cpPolyShape>() as libc::c_ulong,
    ) as *mut cpPolyShape;
}
unsafe extern "C" fn cpPolyShapeDestroy(mut poly: *mut cpPolyShape) {
    if (*poly).count > 6 as libc::c_int {
        free((*poly).planes as *mut libc::c_void);
    }
}
unsafe extern "C" fn cpPolyShapeCacheData(
    mut poly: *mut cpPolyShape,
    mut transform: cpTransform,
) -> cpBB {
    let mut count: libc::c_int = (*poly).count;
    let mut dst: *mut cpSplittingPlane = (*poly).planes;
    let mut src: *mut cpSplittingPlane = dst.offset(count as isize);
    let mut l: cpFloat = ::std::f32::INFINITY as cpFloat;
    let mut r: cpFloat = -(::std::f32::INFINITY as cpFloat);
    let mut b: cpFloat = ::std::f32::INFINITY as cpFloat;
    let mut t: cpFloat = -(::std::f32::INFINITY as cpFloat);
    let mut i: libc::c_int = 0 as libc::c_int;
    while i < count {
        let mut v: cpVect = cpTransformPoint(transform, (*src.offset(i as isize)).v0);
        let mut n: cpVect = cpTransformVect(transform, (*src.offset(i as isize)).n);
        (*dst.offset(i as isize)).v0 = v;
        (*dst.offset(i as isize)).n = n;
        l = cpfmin(l, v.x);
        r = cpfmax(r, v.x);
        b = cpfmin(b, v.y);
        t = cpfmax(t, v.y);
        i += 1;
        i;
    }
    let mut radius: cpFloat = (*poly).r;
    (*poly).shape.bb = cpBBNew(l - radius, b - radius, r + radius, t + radius);
    return (*poly).shape.bb;
}
unsafe extern "C" fn cpPolyShapePointQuery(
    mut poly: *mut cpPolyShape,
    mut p: cpVect,
    mut info: *mut cpPointQueryInfo,
) {
    let mut count: libc::c_int = (*poly).count;
    let mut planes: *mut cpSplittingPlane = (*poly).planes;
    let mut r: cpFloat = (*poly).r;
    let mut v0: cpVect = (*planes.offset((count - 1 as libc::c_int) as isize)).v0;
    let mut minDist: cpFloat = ::std::f32::INFINITY as cpFloat;
    let mut closestPoint: cpVect = cpvzero;
    let mut closestNormal: cpVect = cpvzero;
    let mut outside: cpBool = 0 as libc::c_int as cpBool;
    let mut i: libc::c_int = 0 as libc::c_int;
    while i < count {
        let mut v1: cpVect = (*planes.offset(i as isize)).v0;
        outside = (outside as libc::c_int != 0
            || cpvdot((*planes.offset(i as isize)).n, cpvsub(p, v1))
                > 0.0f32 as libc::c_double) as libc::c_int as cpBool;
        let mut closest: cpVect = cpClosetPointOnSegment(p, v0, v1);
        let mut dist: cpFloat = cpvdist(p, closest);
        if dist < minDist {
            minDist = dist;
            closestPoint = closest;
            closestNormal = (*planes.offset(i as isize)).n;
        }
        v0 = v1;
        i += 1;
        i;
    }
    let mut dist_0: cpFloat = if outside as libc::c_int != 0 {
        minDist
    } else {
        -minDist
    };
    let mut g: cpVect = cpvmult(
        cpvsub(p, closestPoint),
        1.0f32 as libc::c_double / dist_0,
    );
    (*info).shape = poly as *mut cpShape;
    (*info).point = cpvadd(closestPoint, cpvmult(g, r));
    (*info).distance = dist_0 - r;
    (*info).gradient = if minDist > 1e-5f64 { g } else { closestNormal };
}
unsafe extern "C" fn cpPolyShapeSegmentQuery(
    mut poly: *mut cpPolyShape,
    mut a: cpVect,
    mut b: cpVect,
    mut r2: cpFloat,
    mut info: *mut cpSegmentQueryInfo,
) {
    let mut planes: *mut cpSplittingPlane = (*poly).planes;
    let mut count: libc::c_int = (*poly).count;
    let mut r: cpFloat = (*poly).r;
    let mut rsum: cpFloat = r + r2;
    let mut i: libc::c_int = 0 as libc::c_int;
    while i < count {
        let mut n: cpVect = (*planes.offset(i as isize)).n;
        let mut an: cpFloat = cpvdot(a, n);
        let mut d: cpFloat = an - cpvdot((*planes.offset(i as isize)).v0, n) - rsum;
        if !(d < 0.0f32 as libc::c_double) {
            let mut bn: cpFloat = cpvdot(b, n);
            let mut t: cpFloat = d / cpfmax(an - bn, 2.2250738585072014e-308f64);
            if !(t < 0.0f32 as libc::c_double || (1.0f32 as libc::c_double) < t) {
                let mut point: cpVect = cpvlerp(a, b, t);
                let mut dt: cpFloat = cpvcross(n, point);
                let mut dtMin: cpFloat = cpvcross(
                    n,
                    (*planes.offset(((i - 1 as libc::c_int + count) % count) as isize))
                        .v0,
                );
                let mut dtMax: cpFloat = cpvcross(n, (*planes.offset(i as isize)).v0);
                if dtMin <= dt && dt <= dtMax {
                    (*info).shape = poly as *mut cpShape;
                    (*info).point = cpvsub(cpvlerp(a, b, t), cpvmult(n, r2));
                    (*info).normal = n;
                    (*info).alpha = t;
                }
            }
        }
        i += 1;
        i;
    }
    if rsum > 0.0f32 as libc::c_double {
        let mut i_0: libc::c_int = 0 as libc::c_int;
        while i_0 < count {
            let mut circle_info: cpSegmentQueryInfo = {
                let mut init = cpSegmentQueryInfo {
                    shape: 0 as *const cpShape,
                    point: b,
                    normal: cpvzero,
                    alpha: 1.0f32 as cpFloat,
                };
                init
            };
            CircleSegmentQuery(
                &mut (*poly).shape,
                (*planes.offset(i_0 as isize)).v0,
                r,
                a,
                b,
                r2,
                &mut circle_info,
            );
            if circle_info.alpha < (*info).alpha {
                *info = circle_info;
            }
            i_0 += 1;
            i_0;
        }
    }
}
unsafe extern "C" fn SetVerts(
    mut poly: *mut cpPolyShape,
    mut count: libc::c_int,
    mut verts: *const cpVect,
) {
    (*poly).count = count;
    if count <= 6 as libc::c_int {
        (*poly).planes = ((*poly)._planes).as_mut_ptr();
    } else {
        (*poly)
            .planes = calloc(
            (2 as libc::c_int * count) as libc::c_ulong,
            ::std::mem::size_of::<cpSplittingPlane>() as libc::c_ulong,
        ) as *mut cpSplittingPlane;
    }
    let mut i: libc::c_int = 0 as libc::c_int;
    while i < count {
        let mut a: cpVect = *verts
            .offset(((i - 1 as libc::c_int + count) % count) as isize);
        let mut b: cpVect = *verts.offset(i as isize);
        let mut n: cpVect = cpvnormalize(cpvrperp(cpvsub(b, a)));
        (*((*poly).planes).offset((i + count) as isize)).v0 = b;
        (*((*poly).planes).offset((i + count) as isize)).n = n;
        i += 1;
        i;
    }
}
unsafe extern "C" fn cpPolyShapeMassInfo(
    mut mass: cpFloat,
    mut count: libc::c_int,
    mut verts: *const cpVect,
    mut radius: cpFloat,
) -> cpShapeMassInfo {
    let mut centroid: cpVect = cpCentroidForPoly(count, verts);
    let mut info: cpShapeMassInfo = {
        let mut init = cpShapeMassInfo {
            m: mass,
            i: cpMomentForPoly(
                1.0f32 as cpFloat,
                count,
                verts,
                cpvneg(centroid),
                radius,
            ),
            cog: centroid,
            area: cpAreaForPoly(count, verts, radius),
        };
        init
    };
    return info;
}
static mut polyClass: cpShapeClass = unsafe {
    {
        let mut init = cpShapeClass {
            type_0: CP_POLY_SHAPE,
            cacheData: ::std::mem::transmute::<
                Option::<unsafe extern "C" fn(*mut cpPolyShape, cpTransform) -> cpBB>,
                cpShapeCacheDataImpl,
            >(
                Some(
                    cpPolyShapeCacheData
                        as unsafe extern "C" fn(*mut cpPolyShape, cpTransform) -> cpBB,
                ),
            ),
            destroy: ::std::mem::transmute::<
                Option::<unsafe extern "C" fn(*mut cpPolyShape) -> ()>,
                cpShapeDestroyImpl,
            >(Some(cpPolyShapeDestroy as unsafe extern "C" fn(*mut cpPolyShape) -> ())),
            pointQuery: ::std::mem::transmute::<
                Option::<
                    unsafe extern "C" fn(
                        *mut cpPolyShape,
                        cpVect,
                        *mut cpPointQueryInfo,
                    ) -> (),
                >,
                cpShapePointQueryImpl,
            >(
                Some(
                    cpPolyShapePointQuery
                        as unsafe extern "C" fn(
                            *mut cpPolyShape,
                            cpVect,
                            *mut cpPointQueryInfo,
                        ) -> (),
                ),
            ),
            segmentQuery: ::std::mem::transmute::<
                Option::<
                    unsafe extern "C" fn(
                        *mut cpPolyShape,
                        cpVect,
                        cpVect,
                        cpFloat,
                        *mut cpSegmentQueryInfo,
                    ) -> (),
                >,
                cpShapeSegmentQueryImpl,
            >(
                Some(
                    cpPolyShapeSegmentQuery
                        as unsafe extern "C" fn(
                            *mut cpPolyShape,
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
pub unsafe extern "C" fn cpPolyShapeInit(
    mut poly: *mut cpPolyShape,
    mut body: *mut cpBody,
    mut count: libc::c_int,
    mut verts: *const cpVect,
    mut transform: cpTransform,
    mut radius: cpFloat,
) -> *mut cpPolyShape {
    let mut fresh0 = ::std::vec::from_elem(
        0,
        (count as libc::c_ulong)
            .wrapping_mul(::std::mem::size_of::<cpVect>() as libc::c_ulong) as usize,
    );
    let mut hullVerts: *mut cpVect = fresh0.leak().as_mut_ptr() as *mut cpVect;
    let mut i: libc::c_int = 0 as libc::c_int;
    while i < count {
        *hullVerts
            .offset(i as isize) = cpTransformPoint(transform, *verts.offset(i as isize));
        i += 1;
        i;
    }
    let mut hullCount: libc::c_uint = cpConvexHull(
        count,
        hullVerts,
        hullVerts,
        0 as *mut libc::c_int,
        0.0f64,
    ) as libc::c_uint;
    return cpPolyShapeInitRaw(poly, body, hullCount as libc::c_int, hullVerts, radius);
}
pub unsafe extern "C" fn cpPolyShapeInitRaw(
    mut poly: *mut cpPolyShape,
    mut body: *mut cpBody,
    mut count: libc::c_int,
    mut verts: *const cpVect,
    mut radius: cpFloat,
) -> *mut cpPolyShape {
    cpShapeInit(
        poly as *mut cpShape,
        &polyClass,
        body,
        cpPolyShapeMassInfo(0.0f32 as cpFloat, count, verts, radius),
    );
    SetVerts(poly, count, verts);
    (*poly).r = radius;
    return poly;
}
pub unsafe extern "C" fn cpPolyShapeNew(
    mut body: *mut cpBody,
    mut count: libc::c_int,
    mut verts: *const cpVect,
    mut transform: cpTransform,
    mut radius: cpFloat,
) -> *mut cpShape {
    return cpPolyShapeInit(cpPolyShapeAlloc(), body, count, verts, transform, radius)
        as *mut cpShape;
}
pub unsafe extern "C" fn cpPolyShapeNewRaw(
    mut body: *mut cpBody,
    mut count: libc::c_int,
    mut verts: *const cpVect,
    mut radius: cpFloat,
) -> *mut cpShape {
    return cpPolyShapeInitRaw(cpPolyShapeAlloc(), body, count, verts, radius)
        as *mut cpShape;
}
pub unsafe extern "C" fn cpBoxShapeInit(
    mut poly: *mut cpPolyShape,
    mut body: *mut cpBody,
    mut width: cpFloat,
    mut height: cpFloat,
    mut radius: cpFloat,
) -> *mut cpPolyShape {
    let mut hw: cpFloat = width / 2.0f32 as libc::c_double;
    let mut hh: cpFloat = height / 2.0f32 as libc::c_double;
    return cpBoxShapeInit2(poly, body, cpBBNew(-hw, -hh, hw, hh), radius);
}
pub unsafe extern "C" fn cpBoxShapeInit2(
    mut poly: *mut cpPolyShape,
    mut body: *mut cpBody,
    mut box_0: cpBB,
    mut radius: cpFloat,
) -> *mut cpPolyShape {
    let mut verts: [cpVect; 4] = [
        cpv(box_0.r, box_0.b),
        cpv(box_0.r, box_0.t),
        cpv(box_0.l, box_0.t),
        cpv(box_0.l, box_0.b),
    ];
    return cpPolyShapeInitRaw(poly, body, 4 as libc::c_int, verts.as_mut_ptr(), radius);
}
pub unsafe extern "C" fn cpBoxShapeNew(
    mut body: *mut cpBody,
    mut width: cpFloat,
    mut height: cpFloat,
    mut radius: cpFloat,
) -> *mut cpShape {
    return cpBoxShapeInit(cpPolyShapeAlloc(), body, width, height, radius)
        as *mut cpShape;
}
pub unsafe extern "C" fn cpBoxShapeNew2(
    mut body: *mut cpBody,
    mut box_0: cpBB,
    mut radius: cpFloat,
) -> *mut cpShape {
    return cpBoxShapeInit2(cpPolyShapeAlloc(), body, box_0, radius) as *mut cpShape;
}
pub unsafe extern "C" fn cpPolyShapeGetCount(mut shape: *const cpShape) -> libc::c_int {
    if !((*shape).klass == &polyClass as *const cpShapeClass) {
        cpMessage(
            b"shape->klass == &polyClass\0" as *const u8 as *const libc::c_char,
            b"../../src/cpPolyShape.c\0" as *const u8 as *const libc::c_char,
            262 as libc::c_int,
            1 as libc::c_int,
            1 as libc::c_int,
            b"Shape is not a poly shape.\0" as *const u8 as *const libc::c_char,
        );
        abort();
    }
    return (*(shape as *mut cpPolyShape)).count;
}
pub unsafe extern "C" fn cpPolyShapeGetVert(
    mut shape: *const cpShape,
    mut i: libc::c_int,
) -> cpVect {
    if !((*shape).klass == &polyClass as *const cpShapeClass) {
        cpMessage(
            b"shape->klass == &polyClass\0" as *const u8 as *const libc::c_char,
            b"../../src/cpPolyShape.c\0" as *const u8 as *const libc::c_char,
            269 as libc::c_int,
            1 as libc::c_int,
            1 as libc::c_int,
            b"Shape is not a poly shape.\0" as *const u8 as *const libc::c_char,
        );
        abort();
    }
    let mut count: libc::c_int = cpPolyShapeGetCount(shape);
    if !(0 as libc::c_int <= i && i < count) {
        cpMessage(
            b"0 <= i && i < count\0" as *const u8 as *const libc::c_char,
            b"../../src/cpPolyShape.c\0" as *const u8 as *const libc::c_char,
            272 as libc::c_int,
            1 as libc::c_int,
            1 as libc::c_int,
            b"Index out of range.\0" as *const u8 as *const libc::c_char,
        );
        abort();
    }
    return (*((*(shape as *mut cpPolyShape)).planes).offset((i + count) as isize)).v0;
}
pub unsafe extern "C" fn cpPolyShapeGetRadius(mut shape: *const cpShape) -> cpFloat {
    if !((*shape).klass == &polyClass as *const cpShapeClass) {
        cpMessage(
            b"shape->klass == &polyClass\0" as *const u8 as *const libc::c_char,
            b"../../src/cpPolyShape.c\0" as *const u8 as *const libc::c_char,
            280 as libc::c_int,
            1 as libc::c_int,
            1 as libc::c_int,
            b"Shape is not a poly shape.\0" as *const u8 as *const libc::c_char,
        );
        abort();
    }
    return (*(shape as *mut cpPolyShape)).r;
}
pub unsafe extern "C" fn cpPolyShapeSetVerts(
    mut shape: *mut cpShape,
    mut count: libc::c_int,
    mut verts: *mut cpVect,
    mut transform: cpTransform,
) {
    let mut fresh1 = ::std::vec::from_elem(
        0,
        (count as libc::c_ulong)
            .wrapping_mul(::std::mem::size_of::<cpVect>() as libc::c_ulong) as usize,
    );
    let mut hullVerts: *mut cpVect = fresh1.leak().as_mut_ptr() as *mut cpVect;
    let mut i: libc::c_int = 0 as libc::c_int;
    while i < count {
        *hullVerts
            .offset(i as isize) = cpTransformPoint(transform, *verts.offset(i as isize));
        i += 1;
        i;
    }
    let mut hullCount: libc::c_uint = cpConvexHull(
        count,
        hullVerts,
        hullVerts,
        0 as *mut libc::c_int,
        0.0f64,
    ) as libc::c_uint;
    cpPolyShapeSetVertsRaw(shape, hullCount as libc::c_int, hullVerts);
}
pub unsafe extern "C" fn cpPolyShapeSetVertsRaw(
    mut shape: *mut cpShape,
    mut count: libc::c_int,
    mut verts: *mut cpVect,
) {
    if !((*shape).klass == &polyClass as *const cpShapeClass) {
        cpMessage(
            b"shape->klass == &polyClass\0" as *const u8 as *const libc::c_char,
            b"../../src/cpPolyShape.c\0" as *const u8 as *const libc::c_char,
            301 as libc::c_int,
            1 as libc::c_int,
            1 as libc::c_int,
            b"Shape is not a poly shape.\0" as *const u8 as *const libc::c_char,
        );
        abort();
    }
    let mut poly: *mut cpPolyShape = shape as *mut cpPolyShape;
    cpPolyShapeDestroy(poly);
    SetVerts(poly, count, verts);
    let mut mass: cpFloat = (*shape).massInfo.m;
    (*shape)
        .massInfo = cpPolyShapeMassInfo((*shape).massInfo.m, count, verts, (*poly).r);
    if mass > 0.0f32 as libc::c_double {
        cpBodyAccumulateMassFromShapes((*shape).body);
    }
}
pub unsafe extern "C" fn cpPolyShapeSetRadius(
    mut shape: *mut cpShape,
    mut radius: cpFloat,
) {
    if !((*shape).klass == &polyClass as *const cpShapeClass) {
        cpMessage(
            b"shape->klass == &polyClass\0" as *const u8 as *const libc::c_char,
            b"../../src/cpPolyShape.c\0" as *const u8 as *const libc::c_char,
            315 as libc::c_int,
            1 as libc::c_int,
            1 as libc::c_int,
            b"Shape is not a poly shape.\0" as *const u8 as *const libc::c_char,
        );
        abort();
    }
    let mut poly: *mut cpPolyShape = shape as *mut cpPolyShape;
    (*poly).r = radius;
}
