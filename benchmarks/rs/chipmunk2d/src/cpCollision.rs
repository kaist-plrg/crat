use ::libc;
extern "C" {
    pub type cpHashSet;
    pub type cpContactBufferHeader;
    fn sqrt(_: libc::c_double) -> libc::c_double;
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
    fn cpBodyGetRotation(body: *const cpBody) -> cpVect;
    fn cpCheckPointGreater(a: cpVect, b: cpVect, c: cpVect) -> cpBool;
    fn cpCheckAxis(v0: cpVect, v1: cpVect, p: cpVect, n: cpVect) -> cpBool;
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
pub type CollisionFunc = Option::<
    unsafe extern "C" fn(*const cpShape, *const cpShape, *mut cpCollisionInfo) -> (),
>;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ClosestPoints {
    pub a: cpVect,
    pub b: cpVect,
    pub n: cpVect,
    pub d: cpFloat,
    pub id: cpCollisionID,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct SupportContext {
    pub shape1: *const cpShape,
    pub shape2: *const cpShape,
    pub func1: SupportPointFunc,
    pub func2: SupportPointFunc,
}
pub type SupportPointFunc = Option::<
    unsafe extern "C" fn(*const cpShape, cpVect) -> SupportPoint,
>;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct SupportPoint {
    pub p: cpVect,
    pub index: cpCollisionID,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct MinkowskiPoint {
    pub a: cpVect,
    pub b: cpVect,
    pub ab: cpVect,
    pub id: cpCollisionID,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct Edge {
    pub a: EdgePoint,
    pub b: EdgePoint,
    pub r: cpFloat,
    pub n: cpVect,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct EdgePoint {
    pub p: cpVect,
    pub hash: cpHashValue,
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
unsafe extern "C" fn cpveql(v1: cpVect, v2: cpVect) -> cpBool {
    return (v1.x == v2.x && v1.y == v2.y) as libc::c_int as cpBool;
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
unsafe extern "C" fn cpvrotate(v1: cpVect, v2: cpVect) -> cpVect {
    return cpv(v1.x * v2.x - v1.y * v2.y, v1.x * v2.y + v1.y * v2.x);
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
unsafe extern "C" fn cpBBCenter(mut bb: cpBB) -> cpVect {
    return cpvlerp(cpv(bb.l, bb.b), cpv(bb.r, bb.t), 0.5f32 as cpFloat);
}
#[inline]
unsafe extern "C" fn cpCollisionInfoPushContact(
    mut info: *mut cpCollisionInfo,
    mut p1: cpVect,
    mut p2: cpVect,
    mut hash: cpHashValue,
) {
    let mut con: *mut cpContact = &mut *((*info).arr).offset((*info).count as isize)
        as *mut cpContact;
    (*con).r1 = p1;
    (*con).r2 = p2;
    (*con).hash = hash;
    (*info).count += 1;
    (*info).count;
}
#[inline]
unsafe extern "C" fn PolySupportPointIndex(
    count: libc::c_int,
    mut planes: *const cpSplittingPlane,
    n: cpVect,
) -> libc::c_int {
    let mut max: cpFloat = -::std::f32::INFINITY as cpFloat;
    let mut index: libc::c_int = 0 as libc::c_int;
    let mut i: libc::c_int = 0 as libc::c_int;
    while i < count {
        let mut v: cpVect = (*planes.offset(i as isize)).v0;
        let mut d: cpFloat = cpvdot(v, n);
        if d > max {
            max = d;
            index = i;
        }
        i += 1;
        i;
    }
    return index;
}
#[inline]
unsafe extern "C" fn SupportPointNew(
    mut p: cpVect,
    mut index: cpCollisionID,
) -> SupportPoint {
    let mut point: SupportPoint = {
        let mut init = SupportPoint { p: p, index: index };
        init
    };
    return point;
}
#[inline]
unsafe extern "C" fn CircleSupportPoint(
    mut circle: *const cpCircleShape,
    n: cpVect,
) -> SupportPoint {
    return SupportPointNew((*circle).tc, 0 as libc::c_int as cpCollisionID);
}
#[inline]
unsafe extern "C" fn SegmentSupportPoint(
    mut seg: *const cpSegmentShape,
    n: cpVect,
) -> SupportPoint {
    if cpvdot((*seg).ta, n) > cpvdot((*seg).tb, n) {
        return SupportPointNew((*seg).ta, 0 as libc::c_int as cpCollisionID)
    } else {
        return SupportPointNew((*seg).tb, 1 as libc::c_int as cpCollisionID)
    };
}
#[inline]
unsafe extern "C" fn PolySupportPoint(
    mut poly: *const cpPolyShape,
    n: cpVect,
) -> SupportPoint {
    let mut planes: *const cpSplittingPlane = (*poly).planes;
    let mut i: libc::c_int = PolySupportPointIndex((*poly).count, planes, n);
    return SupportPointNew((*planes.offset(i as isize)).v0, i as cpCollisionID);
}
#[inline]
unsafe extern "C" fn MinkowskiPointNew(
    a: SupportPoint,
    b: SupportPoint,
) -> MinkowskiPoint {
    let mut point: MinkowskiPoint = {
        let mut init = MinkowskiPoint {
            a: a.p,
            b: b.p,
            ab: cpvsub(b.p, a.p),
            id: (a.index & 0xff as libc::c_int as libc::c_uint) << 8 as libc::c_int
                | b.index & 0xff as libc::c_int as libc::c_uint,
        };
        init
    };
    return point;
}
#[inline]
unsafe extern "C" fn Support(
    mut ctx: *const SupportContext,
    n: cpVect,
) -> MinkowskiPoint {
    let mut a: SupportPoint = ((*ctx).func1).unwrap()((*ctx).shape1, cpvneg(n));
    let mut b: SupportPoint = ((*ctx).func2).unwrap()((*ctx).shape2, n);
    return MinkowskiPointNew(a, b);
}
unsafe extern "C" fn SupportEdgeForPoly(
    mut poly: *const cpPolyShape,
    n: cpVect,
) -> Edge {
    let mut count: libc::c_int = (*poly).count;
    let mut i1: libc::c_int = PolySupportPointIndex((*poly).count, (*poly).planes, n);
    let mut i0: libc::c_int = (i1 - 1 as libc::c_int + count) % count;
    let mut i2: libc::c_int = (i1 + 1 as libc::c_int) % count;
    let mut planes: *const cpSplittingPlane = (*poly).planes;
    let mut hashid: cpHashValue = (*poly).shape.hashid;
    if cpvdot(n, (*planes.offset(i1 as isize)).n)
        > cpvdot(n, (*planes.offset(i2 as isize)).n)
    {
        let mut edge: Edge = {
            let mut init = Edge {
                a: {
                    let mut init = EdgePoint {
                        p: (*planes.offset(i0 as isize)).v0,
                        hash: hashid.wrapping_mul(3344921057 as libc::c_ulong)
                            ^ (i0 as cpHashValue)
                                .wrapping_mul(3344921057 as libc::c_ulong),
                    };
                    init
                },
                b: {
                    let mut init = EdgePoint {
                        p: (*planes.offset(i1 as isize)).v0,
                        hash: hashid.wrapping_mul(3344921057 as libc::c_ulong)
                            ^ (i1 as cpHashValue)
                                .wrapping_mul(3344921057 as libc::c_ulong),
                    };
                    init
                },
                r: (*poly).r,
                n: (*planes.offset(i1 as isize)).n,
            };
            init
        };
        return edge;
    } else {
        let mut edge_0: Edge = {
            let mut init = Edge {
                a: {
                    let mut init = EdgePoint {
                        p: (*planes.offset(i1 as isize)).v0,
                        hash: hashid.wrapping_mul(3344921057 as libc::c_ulong)
                            ^ (i1 as cpHashValue)
                                .wrapping_mul(3344921057 as libc::c_ulong),
                    };
                    init
                },
                b: {
                    let mut init = EdgePoint {
                        p: (*planes.offset(i2 as isize)).v0,
                        hash: hashid.wrapping_mul(3344921057 as libc::c_ulong)
                            ^ (i2 as cpHashValue)
                                .wrapping_mul(3344921057 as libc::c_ulong),
                    };
                    init
                },
                r: (*poly).r,
                n: (*planes.offset(i2 as isize)).n,
            };
            init
        };
        return edge_0;
    };
}
unsafe extern "C" fn SupportEdgeForSegment(
    mut seg: *const cpSegmentShape,
    n: cpVect,
) -> Edge {
    let mut hashid: cpHashValue = (*seg).shape.hashid;
    if cpvdot((*seg).tn, n) > 0.0f64 {
        let mut edge: Edge = {
            let mut init = Edge {
                a: {
                    let mut init = EdgePoint {
                        p: (*seg).ta,
                        hash: hashid.wrapping_mul(3344921057 as libc::c_ulong)
                            ^ (0 as libc::c_int as cpHashValue)
                                .wrapping_mul(3344921057 as libc::c_ulong),
                    };
                    init
                },
                b: {
                    let mut init = EdgePoint {
                        p: (*seg).tb,
                        hash: hashid.wrapping_mul(3344921057 as libc::c_ulong)
                            ^ (1 as libc::c_int as cpHashValue)
                                .wrapping_mul(3344921057 as libc::c_ulong),
                    };
                    init
                },
                r: (*seg).r,
                n: (*seg).tn,
            };
            init
        };
        return edge;
    } else {
        let mut edge_0: Edge = {
            let mut init = Edge {
                a: {
                    let mut init = EdgePoint {
                        p: (*seg).tb,
                        hash: hashid.wrapping_mul(3344921057 as libc::c_ulong)
                            ^ (1 as libc::c_int as cpHashValue)
                                .wrapping_mul(3344921057 as libc::c_ulong),
                    };
                    init
                },
                b: {
                    let mut init = EdgePoint {
                        p: (*seg).ta,
                        hash: hashid.wrapping_mul(3344921057 as libc::c_ulong)
                            ^ (0 as libc::c_int as cpHashValue)
                                .wrapping_mul(3344921057 as libc::c_ulong),
                    };
                    init
                },
                r: (*seg).r,
                n: cpvneg((*seg).tn),
            };
            init
        };
        return edge_0;
    };
}
#[inline]
unsafe extern "C" fn ClosestT(a: cpVect, b: cpVect) -> cpFloat {
    let mut delta: cpVect = cpvsub(b, a);
    return -cpfclamp(
        cpvdot(delta, cpvadd(a, b)) / (cpvlengthsq(delta) + 2.2250738585072014e-308f64),
        -1.0f32 as cpFloat,
        1.0f32 as cpFloat,
    );
}
#[inline]
unsafe extern "C" fn LerpT(a: cpVect, b: cpVect, t: cpFloat) -> cpVect {
    let mut ht: cpFloat = 0.5f32 as libc::c_double * t;
    return cpvadd(
        cpvmult(a, 0.5f32 as libc::c_double - ht),
        cpvmult(b, 0.5f32 as libc::c_double + ht),
    );
}
#[inline]
unsafe extern "C" fn ClosestPointsNew(
    v0: MinkowskiPoint,
    v1: MinkowskiPoint,
) -> ClosestPoints {
    let mut t: cpFloat = ClosestT(v0.ab, v1.ab);
    let mut p: cpVect = LerpT(v0.ab, v1.ab, t);
    let mut pa: cpVect = LerpT(v0.a, v1.a, t);
    let mut pb: cpVect = LerpT(v0.b, v1.b, t);
    let mut id: cpCollisionID = (v0.id & 0xffff as libc::c_int as libc::c_uint)
        << 16 as libc::c_int | v1.id & 0xffff as libc::c_int as libc::c_uint;
    let mut delta: cpVect = cpvsub(v1.ab, v0.ab);
    let mut n: cpVect = cpvnormalize(cpvrperp(delta));
    let mut d: cpFloat = cpvdot(n, p);
    if d <= 0.0f32 as libc::c_double
        || (-1.0f32 as libc::c_double) < t && t < 1.0f32 as libc::c_double
    {
        let mut points: ClosestPoints = {
            let mut init = ClosestPoints {
                a: pa,
                b: pb,
                n: n,
                d: d,
                id: id,
            };
            init
        };
        return points;
    } else {
        let mut d2: cpFloat = cpvlength(p);
        let mut n2: cpVect = cpvmult(
            p,
            1.0f32 as libc::c_double / (d2 + 2.2250738585072014e-308f64),
        );
        let mut points_0: ClosestPoints = {
            let mut init = ClosestPoints {
                a: pa,
                b: pb,
                n: n2,
                d: d2,
                id: id,
            };
            init
        };
        return points_0;
    };
}
#[inline]
unsafe extern "C" fn ClosestDist(v0: cpVect, v1: cpVect) -> cpFloat {
    return cpvlengthsq(LerpT(v0, v1, ClosestT(v0, v1)));
}
unsafe extern "C" fn EPARecurse(
    mut ctx: *const SupportContext,
    count: libc::c_int,
    mut hull: *const MinkowskiPoint,
    iteration: libc::c_int,
) -> ClosestPoints {
    let mut mini: libc::c_int = 0 as libc::c_int;
    let mut minDist: cpFloat = ::std::f32::INFINITY as cpFloat;
    let mut j: libc::c_int = 0 as libc::c_int;
    let mut i: libc::c_int = count - 1 as libc::c_int;
    while j < count {
        let mut d: cpFloat = ClosestDist(
            (*hull.offset(i as isize)).ab,
            (*hull.offset(j as isize)).ab,
        );
        if d < minDist {
            minDist = d;
            mini = i;
        }
        i = j;
        j += 1;
        j;
    }
    let mut v0: MinkowskiPoint = *hull.offset(mini as isize);
    let mut v1: MinkowskiPoint = *hull
        .offset(((mini + 1 as libc::c_int) % count) as isize);
    let mut p: MinkowskiPoint = Support(ctx, cpvperp(cpvsub(v1.ab, v0.ab)));
    let mut duplicate: cpBool = (p.id == v0.id || p.id == v1.id) as libc::c_int
        as cpBool;
    if duplicate == 0 && cpCheckPointGreater(v0.ab, v1.ab, p.ab) as libc::c_int != 0
        && iteration < 30 as libc::c_int
    {
        let mut fresh0 = ::std::vec::from_elem(
            0,
            ((count + 1 as libc::c_int) as libc::c_ulong)
                .wrapping_mul(::std::mem::size_of::<MinkowskiPoint>() as libc::c_ulong)
                as usize,
        );
        let mut hull2: *mut MinkowskiPoint = fresh0.leak().as_mut_ptr()
            as *mut MinkowskiPoint;
        let mut count2: libc::c_int = 1 as libc::c_int;
        *hull2.offset(0 as libc::c_int as isize) = p;
        let mut i_0: libc::c_int = 0 as libc::c_int;
        while i_0 < count {
            let mut index: libc::c_int = (mini + 1 as libc::c_int + i_0) % count;
            let mut h0: cpVect = (*hull2.offset((count2 - 1 as libc::c_int) as isize))
                .ab;
            let mut h1: cpVect = (*hull.offset(index as isize)).ab;
            let mut h2: cpVect = if (i_0 + 1 as libc::c_int) < count {
                *hull.offset(((index + 1 as libc::c_int) % count) as isize)
            } else {
                p
            }
                .ab;
            if cpCheckPointGreater(h0, h2, h1) != 0 {
                *hull2.offset(count2 as isize) = *hull.offset(index as isize);
                count2 += 1;
                count2;
            }
            i_0 += 1;
            i_0;
        }
        return EPARecurse(ctx, count2, hull2, iteration + 1 as libc::c_int);
    } else {
        return ClosestPointsNew(v0, v1)
    };
}
unsafe extern "C" fn EPA(
    mut ctx: *const SupportContext,
    v0: MinkowskiPoint,
    v1: MinkowskiPoint,
    v2: MinkowskiPoint,
) -> ClosestPoints {
    let mut hull: [MinkowskiPoint; 3] = [v0, v1, v2];
    return EPARecurse(ctx, 3 as libc::c_int, hull.as_mut_ptr(), 1 as libc::c_int);
}
#[inline]
unsafe extern "C" fn GJKRecurse(
    mut ctx: *const SupportContext,
    v0: MinkowskiPoint,
    v1: MinkowskiPoint,
    iteration: libc::c_int,
) -> ClosestPoints {
    if iteration > 30 as libc::c_int {
        return ClosestPointsNew(v0, v1);
    }
    if cpCheckPointGreater(v1.ab, v0.ab, cpvzero) != 0 {
        return GJKRecurse(ctx, v1, v0, iteration)
    } else {
        let mut t: cpFloat = ClosestT(v0.ab, v1.ab);
        let mut n: cpVect = if (-1.0f32 as libc::c_double) < t
            && t < 1.0f32 as libc::c_double
        {
            cpvperp(cpvsub(v1.ab, v0.ab))
        } else {
            cpvneg(LerpT(v0.ab, v1.ab, t))
        };
        let mut p: MinkowskiPoint = Support(ctx, n);
        if cpCheckPointGreater(p.ab, v0.ab, cpvzero) as libc::c_int != 0
            && cpCheckPointGreater(v1.ab, p.ab, cpvzero) as libc::c_int != 0
        {
            return EPA(ctx, v0, p, v1)
        } else if cpCheckAxis(v0.ab, v1.ab, p.ab, n) != 0 {
            return ClosestPointsNew(v0, v1)
        } else if ClosestDist(v0.ab, p.ab) < ClosestDist(p.ab, v1.ab) {
            return GJKRecurse(ctx, v0, p, iteration + 1 as libc::c_int)
        } else {
            return GJKRecurse(ctx, p, v1, iteration + 1 as libc::c_int)
        }
    };
}
unsafe extern "C" fn ShapePoint(
    mut shape: *const cpShape,
    i: libc::c_int,
) -> SupportPoint {
    match (*(*shape).klass).type_0 as libc::c_uint {
        0 => {
            return SupportPointNew(
                (*(shape as *mut cpCircleShape)).tc,
                0 as libc::c_int as cpCollisionID,
            );
        }
        1 => {
            let mut seg: *mut cpSegmentShape = shape as *mut cpSegmentShape;
            return SupportPointNew(
                if i == 0 as libc::c_int { (*seg).ta } else { (*seg).tb },
                i as cpCollisionID,
            );
        }
        2 => {
            let mut poly: *mut cpPolyShape = shape as *mut cpPolyShape;
            let mut index: libc::c_int = if i < (*poly).count {
                i
            } else {
                0 as libc::c_int
            };
            return SupportPointNew(
                (*((*poly).planes).offset(index as isize)).v0,
                index as cpCollisionID,
            );
        }
        _ => return SupportPointNew(cpvzero, 0 as libc::c_int as cpCollisionID),
    };
}
unsafe extern "C" fn GJK(
    mut ctx: *const SupportContext,
    mut id: *mut cpCollisionID,
) -> ClosestPoints {
    let mut v0: MinkowskiPoint = MinkowskiPoint {
        a: cpVect { x: 0., y: 0. },
        b: cpVect { x: 0., y: 0. },
        ab: cpVect { x: 0., y: 0. },
        id: 0,
    };
    let mut v1: MinkowskiPoint = MinkowskiPoint {
        a: cpVect { x: 0., y: 0. },
        b: cpVect { x: 0., y: 0. },
        ab: cpVect { x: 0., y: 0. },
        id: 0,
    };
    if *id != 0 {
        v0 = MinkowskiPointNew(
            ShapePoint(
                (*ctx).shape1,
                (*id >> 24 as libc::c_int & 0xff as libc::c_int as libc::c_uint)
                    as libc::c_int,
            ),
            ShapePoint(
                (*ctx).shape2,
                (*id >> 16 as libc::c_int & 0xff as libc::c_int as libc::c_uint)
                    as libc::c_int,
            ),
        );
        v1 = MinkowskiPointNew(
            ShapePoint(
                (*ctx).shape1,
                (*id >> 8 as libc::c_int & 0xff as libc::c_int as libc::c_uint)
                    as libc::c_int,
            ),
            ShapePoint(
                (*ctx).shape2,
                (*id & 0xff as libc::c_int as libc::c_uint) as libc::c_int,
            ),
        );
    } else {
        let mut axis: cpVect = cpvperp(
            cpvsub(cpBBCenter((*(*ctx).shape1).bb), cpBBCenter((*(*ctx).shape2).bb)),
        );
        v0 = Support(ctx, axis);
        v1 = Support(ctx, cpvneg(axis));
    }
    let mut points: ClosestPoints = GJKRecurse(ctx, v0, v1, 1 as libc::c_int);
    *id = points.id;
    return points;
}
#[inline]
unsafe extern "C" fn ContactPoints(
    e1: Edge,
    e2: Edge,
    points: ClosestPoints,
    mut info: *mut cpCollisionInfo,
) {
    let mut mindist: cpFloat = e1.r + e2.r;
    if points.d <= mindist {
        (*info).n = points.n;
        let mut n: cpVect = (*info).n;
        let mut d_e1_a: cpFloat = cpvcross(e1.a.p, n);
        let mut d_e1_b: cpFloat = cpvcross(e1.b.p, n);
        let mut d_e2_a: cpFloat = cpvcross(e2.a.p, n);
        let mut d_e2_b: cpFloat = cpvcross(e2.b.p, n);
        let mut e1_denom: cpFloat = 1.0f32 as libc::c_double
            / (d_e1_b - d_e1_a + 2.2250738585072014e-308f64);
        let mut e2_denom: cpFloat = 1.0f32 as libc::c_double
            / (d_e2_b - d_e2_a + 2.2250738585072014e-308f64);
        let mut p1: cpVect = cpvadd(
            cpvmult(n, e1.r),
            cpvlerp(e1.a.p, e1.b.p, cpfclamp01((d_e2_b - d_e1_a) * e1_denom)),
        );
        let mut p2: cpVect = cpvadd(
            cpvmult(n, -e2.r),
            cpvlerp(e2.a.p, e2.b.p, cpfclamp01((d_e1_a - d_e2_a) * e2_denom)),
        );
        let mut dist: cpFloat = cpvdot(cpvsub(p2, p1), n);
        if dist <= 0.0f32 as libc::c_double {
            let mut hash_1a2b: cpHashValue = (e1.a.hash)
                .wrapping_mul(3344921057 as libc::c_ulong)
                ^ (e2.b.hash).wrapping_mul(3344921057 as libc::c_ulong);
            cpCollisionInfoPushContact(info, p1, p2, hash_1a2b);
        }
        let mut p1_0: cpVect = cpvadd(
            cpvmult(n, e1.r),
            cpvlerp(e1.a.p, e1.b.p, cpfclamp01((d_e2_a - d_e1_a) * e1_denom)),
        );
        let mut p2_0: cpVect = cpvadd(
            cpvmult(n, -e2.r),
            cpvlerp(e2.a.p, e2.b.p, cpfclamp01((d_e1_b - d_e2_a) * e2_denom)),
        );
        let mut dist_0: cpFloat = cpvdot(cpvsub(p2_0, p1_0), n);
        if dist_0 <= 0.0f32 as libc::c_double {
            let mut hash_1b2a: cpHashValue = (e1.b.hash)
                .wrapping_mul(3344921057 as libc::c_ulong)
                ^ (e2.a.hash).wrapping_mul(3344921057 as libc::c_ulong);
            cpCollisionInfoPushContact(info, p1_0, p2_0, hash_1b2a);
        }
    }
}
unsafe extern "C" fn CircleToCircle(
    mut c1: *const cpCircleShape,
    mut c2: *const cpCircleShape,
    mut info: *mut cpCollisionInfo,
) {
    let mut mindist: cpFloat = (*c1).r + (*c2).r;
    let mut delta: cpVect = cpvsub((*c2).tc, (*c1).tc);
    let mut distsq: cpFloat = cpvlengthsq(delta);
    if distsq < mindist * mindist {
        let mut dist: cpFloat = sqrt(distsq);
        (*info)
            .n = if dist != 0. {
            cpvmult(delta, 1.0f32 as libc::c_double / dist)
        } else {
            cpv(1.0f32 as cpFloat, 0.0f32 as cpFloat)
        };
        let mut n: cpVect = (*info).n;
        cpCollisionInfoPushContact(
            info,
            cpvadd((*c1).tc, cpvmult(n, (*c1).r)),
            cpvadd((*c2).tc, cpvmult(n, -(*c2).r)),
            0 as libc::c_int as cpHashValue,
        );
    }
}
unsafe extern "C" fn CircleToSegment(
    mut circle: *const cpCircleShape,
    mut segment: *const cpSegmentShape,
    mut info: *mut cpCollisionInfo,
) {
    let mut seg_a: cpVect = (*segment).ta;
    let mut seg_b: cpVect = (*segment).tb;
    let mut center: cpVect = (*circle).tc;
    let mut seg_delta: cpVect = cpvsub(seg_b, seg_a);
    let mut closest_t: cpFloat = cpfclamp01(
        cpvdot(seg_delta, cpvsub(center, seg_a)) / cpvlengthsq(seg_delta),
    );
    let mut closest: cpVect = cpvadd(seg_a, cpvmult(seg_delta, closest_t));
    let mut mindist: cpFloat = (*circle).r + (*segment).r;
    let mut delta: cpVect = cpvsub(closest, center);
    let mut distsq: cpFloat = cpvlengthsq(delta);
    if distsq < mindist * mindist {
        let mut dist: cpFloat = sqrt(distsq);
        (*info)
            .n = if dist != 0. {
            cpvmult(delta, 1.0f32 as libc::c_double / dist)
        } else {
            (*segment).tn
        };
        let mut n: cpVect = (*info).n;
        let mut rot: cpVect = cpBodyGetRotation((*segment).shape.body);
        if (closest_t != 0.0f32 as libc::c_double
            || cpvdot(n, cpvrotate((*segment).a_tangent, rot)) >= 0.0f64)
            && (closest_t != 1.0f32 as libc::c_double
                || cpvdot(n, cpvrotate((*segment).b_tangent, rot)) >= 0.0f64)
        {
            cpCollisionInfoPushContact(
                info,
                cpvadd(center, cpvmult(n, (*circle).r)),
                cpvadd(closest, cpvmult(n, -(*segment).r)),
                0 as libc::c_int as cpHashValue,
            );
        }
    }
}
unsafe extern "C" fn SegmentToSegment(
    mut seg1: *const cpSegmentShape,
    mut seg2: *const cpSegmentShape,
    mut info: *mut cpCollisionInfo,
) {
    let mut context: SupportContext = {
        let mut init = SupportContext {
            shape1: seg1 as *mut cpShape,
            shape2: seg2 as *mut cpShape,
            func1: ::std::mem::transmute::<
                Option::<
                    unsafe extern "C" fn(*const cpSegmentShape, cpVect) -> SupportPoint,
                >,
                SupportPointFunc,
            >(
                Some(
                    SegmentSupportPoint
                        as unsafe extern "C" fn(
                            *const cpSegmentShape,
                            cpVect,
                        ) -> SupportPoint,
                ),
            ),
            func2: ::std::mem::transmute::<
                Option::<
                    unsafe extern "C" fn(*const cpSegmentShape, cpVect) -> SupportPoint,
                >,
                SupportPointFunc,
            >(
                Some(
                    SegmentSupportPoint
                        as unsafe extern "C" fn(
                            *const cpSegmentShape,
                            cpVect,
                        ) -> SupportPoint,
                ),
            ),
        };
        init
    };
    let mut points: ClosestPoints = GJK(&mut context, &mut (*info).id);
    let mut n: cpVect = points.n;
    let mut rot1: cpVect = cpBodyGetRotation((*seg1).shape.body);
    let mut rot2: cpVect = cpBodyGetRotation((*seg2).shape.body);
    if points.d <= (*seg1).r + (*seg2).r
        && ((cpveql(points.a, (*seg1).ta) == 0
            || cpvdot(n, cpvrotate((*seg1).a_tangent, rot1)) <= 0.0f64)
            && (cpveql(points.a, (*seg1).tb) == 0
                || cpvdot(n, cpvrotate((*seg1).b_tangent, rot1)) <= 0.0f64)
            && (cpveql(points.b, (*seg2).ta) == 0
                || cpvdot(n, cpvrotate((*seg2).a_tangent, rot2)) >= 0.0f64)
            && (cpveql(points.b, (*seg2).tb) == 0
                || cpvdot(n, cpvrotate((*seg2).b_tangent, rot2)) >= 0.0f64))
    {
        ContactPoints(
            SupportEdgeForSegment(seg1, n),
            SupportEdgeForSegment(seg2, cpvneg(n)),
            points,
            info,
        );
    }
}
unsafe extern "C" fn PolyToPoly(
    mut poly1: *const cpPolyShape,
    mut poly2: *const cpPolyShape,
    mut info: *mut cpCollisionInfo,
) {
    let mut context: SupportContext = {
        let mut init = SupportContext {
            shape1: poly1 as *mut cpShape,
            shape2: poly2 as *mut cpShape,
            func1: ::std::mem::transmute::<
                Option::<
                    unsafe extern "C" fn(*const cpPolyShape, cpVect) -> SupportPoint,
                >,
                SupportPointFunc,
            >(
                Some(
                    PolySupportPoint
                        as unsafe extern "C" fn(
                            *const cpPolyShape,
                            cpVect,
                        ) -> SupportPoint,
                ),
            ),
            func2: ::std::mem::transmute::<
                Option::<
                    unsafe extern "C" fn(*const cpPolyShape, cpVect) -> SupportPoint,
                >,
                SupportPointFunc,
            >(
                Some(
                    PolySupportPoint
                        as unsafe extern "C" fn(
                            *const cpPolyShape,
                            cpVect,
                        ) -> SupportPoint,
                ),
            ),
        };
        init
    };
    let mut points: ClosestPoints = GJK(&mut context, &mut (*info).id);
    if points.d - (*poly1).r - (*poly2).r <= 0.0f64 {
        ContactPoints(
            SupportEdgeForPoly(poly1, points.n),
            SupportEdgeForPoly(poly2, cpvneg(points.n)),
            points,
            info,
        );
    }
}
unsafe extern "C" fn SegmentToPoly(
    mut seg: *const cpSegmentShape,
    mut poly: *const cpPolyShape,
    mut info: *mut cpCollisionInfo,
) {
    let mut context: SupportContext = {
        let mut init = SupportContext {
            shape1: seg as *mut cpShape,
            shape2: poly as *mut cpShape,
            func1: ::std::mem::transmute::<
                Option::<
                    unsafe extern "C" fn(*const cpSegmentShape, cpVect) -> SupportPoint,
                >,
                SupportPointFunc,
            >(
                Some(
                    SegmentSupportPoint
                        as unsafe extern "C" fn(
                            *const cpSegmentShape,
                            cpVect,
                        ) -> SupportPoint,
                ),
            ),
            func2: ::std::mem::transmute::<
                Option::<
                    unsafe extern "C" fn(*const cpPolyShape, cpVect) -> SupportPoint,
                >,
                SupportPointFunc,
            >(
                Some(
                    PolySupportPoint
                        as unsafe extern "C" fn(
                            *const cpPolyShape,
                            cpVect,
                        ) -> SupportPoint,
                ),
            ),
        };
        init
    };
    let mut points: ClosestPoints = GJK(&mut context, &mut (*info).id);
    let mut n: cpVect = points.n;
    let mut rot: cpVect = cpBodyGetRotation((*seg).shape.body);
    if points.d - (*seg).r - (*poly).r <= 0.0f64
        && ((cpveql(points.a, (*seg).ta) == 0
            || cpvdot(n, cpvrotate((*seg).a_tangent, rot)) <= 0.0f64)
            && (cpveql(points.a, (*seg).tb) == 0
                || cpvdot(n, cpvrotate((*seg).b_tangent, rot)) <= 0.0f64))
    {
        ContactPoints(
            SupportEdgeForSegment(seg, n),
            SupportEdgeForPoly(poly, cpvneg(n)),
            points,
            info,
        );
    }
}
unsafe extern "C" fn CircleToPoly(
    mut circle: *const cpCircleShape,
    mut poly: *const cpPolyShape,
    mut info: *mut cpCollisionInfo,
) {
    let mut context: SupportContext = {
        let mut init = SupportContext {
            shape1: circle as *mut cpShape,
            shape2: poly as *mut cpShape,
            func1: ::std::mem::transmute::<
                Option::<
                    unsafe extern "C" fn(*const cpCircleShape, cpVect) -> SupportPoint,
                >,
                SupportPointFunc,
            >(
                Some(
                    CircleSupportPoint
                        as unsafe extern "C" fn(
                            *const cpCircleShape,
                            cpVect,
                        ) -> SupportPoint,
                ),
            ),
            func2: ::std::mem::transmute::<
                Option::<
                    unsafe extern "C" fn(*const cpPolyShape, cpVect) -> SupportPoint,
                >,
                SupportPointFunc,
            >(
                Some(
                    PolySupportPoint
                        as unsafe extern "C" fn(
                            *const cpPolyShape,
                            cpVect,
                        ) -> SupportPoint,
                ),
            ),
        };
        init
    };
    let mut points: ClosestPoints = GJK(&mut context, &mut (*info).id);
    if points.d <= (*circle).r + (*poly).r {
        (*info).n = points.n;
        let mut n: cpVect = (*info).n;
        cpCollisionInfoPushContact(
            info,
            cpvadd(points.a, cpvmult(n, (*circle).r)),
            cpvadd(points.b, cpvmult(n, -(*poly).r)),
            0 as libc::c_int as cpHashValue,
        );
    }
}
unsafe extern "C" fn CollisionError(
    mut circle: *const cpShape,
    mut poly: *const cpShape,
    mut info: *mut cpCollisionInfo,
) {
    if 0 as libc::c_int == 0 {
        cpMessage(
            b"cpFalse\0" as *const u8 as *const libc::c_char,
            b"../../src/cpCollision.c\0" as *const u8 as *const libc::c_char,
            684 as libc::c_int,
            1 as libc::c_int,
            1 as libc::c_int,
            b"Internal Error: Shape types are not sorted.\0" as *const u8
                as *const libc::c_char,
        );
        abort();
    }
}
static mut BuiltinCollisionFuncs: [CollisionFunc; 9] = unsafe {
    [
        ::std::mem::transmute::<
            Option::<
                unsafe extern "C" fn(
                    *const cpCircleShape,
                    *const cpCircleShape,
                    *mut cpCollisionInfo,
                ) -> (),
            >,
            CollisionFunc,
        >(
            Some(
                CircleToCircle
                    as unsafe extern "C" fn(
                        *const cpCircleShape,
                        *const cpCircleShape,
                        *mut cpCollisionInfo,
                    ) -> (),
            ),
        ),
        Some(
            CollisionError
                as unsafe extern "C" fn(
                    *const cpShape,
                    *const cpShape,
                    *mut cpCollisionInfo,
                ) -> (),
        ),
        Some(
            CollisionError
                as unsafe extern "C" fn(
                    *const cpShape,
                    *const cpShape,
                    *mut cpCollisionInfo,
                ) -> (),
        ),
        ::std::mem::transmute::<
            Option::<
                unsafe extern "C" fn(
                    *const cpCircleShape,
                    *const cpSegmentShape,
                    *mut cpCollisionInfo,
                ) -> (),
            >,
            CollisionFunc,
        >(
            Some(
                CircleToSegment
                    as unsafe extern "C" fn(
                        *const cpCircleShape,
                        *const cpSegmentShape,
                        *mut cpCollisionInfo,
                    ) -> (),
            ),
        ),
        ::std::mem::transmute::<
            Option::<
                unsafe extern "C" fn(
                    *const cpSegmentShape,
                    *const cpSegmentShape,
                    *mut cpCollisionInfo,
                ) -> (),
            >,
            CollisionFunc,
        >(
            Some(
                SegmentToSegment
                    as unsafe extern "C" fn(
                        *const cpSegmentShape,
                        *const cpSegmentShape,
                        *mut cpCollisionInfo,
                    ) -> (),
            ),
        ),
        Some(
            CollisionError
                as unsafe extern "C" fn(
                    *const cpShape,
                    *const cpShape,
                    *mut cpCollisionInfo,
                ) -> (),
        ),
        ::std::mem::transmute::<
            Option::<
                unsafe extern "C" fn(
                    *const cpCircleShape,
                    *const cpPolyShape,
                    *mut cpCollisionInfo,
                ) -> (),
            >,
            CollisionFunc,
        >(
            Some(
                CircleToPoly
                    as unsafe extern "C" fn(
                        *const cpCircleShape,
                        *const cpPolyShape,
                        *mut cpCollisionInfo,
                    ) -> (),
            ),
        ),
        ::std::mem::transmute::<
            Option::<
                unsafe extern "C" fn(
                    *const cpSegmentShape,
                    *const cpPolyShape,
                    *mut cpCollisionInfo,
                ) -> (),
            >,
            CollisionFunc,
        >(
            Some(
                SegmentToPoly
                    as unsafe extern "C" fn(
                        *const cpSegmentShape,
                        *const cpPolyShape,
                        *mut cpCollisionInfo,
                    ) -> (),
            ),
        ),
        ::std::mem::transmute::<
            Option::<
                unsafe extern "C" fn(
                    *const cpPolyShape,
                    *const cpPolyShape,
                    *mut cpCollisionInfo,
                ) -> (),
            >,
            CollisionFunc,
        >(
            Some(
                PolyToPoly
                    as unsafe extern "C" fn(
                        *const cpPolyShape,
                        *const cpPolyShape,
                        *mut cpCollisionInfo,
                    ) -> (),
            ),
        ),
    ]
};
static mut CollisionFuncs: *const CollisionFunc = unsafe {
    BuiltinCollisionFuncs.as_ptr()
};
pub unsafe extern "C" fn cpCollide(
    mut a: *const cpShape,
    mut b: *const cpShape,
    mut id: cpCollisionID,
    mut contacts: *mut cpContact,
) -> cpCollisionInfo {
    let mut info: cpCollisionInfo = {
        let mut init = cpCollisionInfo {
            a: a,
            b: b,
            id: id,
            n: cpvzero,
            count: 0 as libc::c_int,
            arr: contacts,
        };
        init
    };
    if (*(*a).klass).type_0 as libc::c_uint > (*(*b).klass).type_0 as libc::c_uint {
        info.a = b;
        info.b = a;
    }
    (*CollisionFuncs
        .offset(
            ((*(*info.a).klass).type_0 as libc::c_uint)
                .wrapping_add(
                    ((*(*info.b).klass).type_0 as libc::c_uint)
                        .wrapping_mul(CP_NUM_SHAPES as libc::c_int as libc::c_uint),
                ) as isize,
        ))
        .unwrap()(info.a, info.b, &mut info);
    return info;
}
