use ::libc;
extern "C" {
    pub type cpHashSet;
    pub type cpContactBufferHeader;
    fn cos(_: libc::c_double) -> libc::c_double;
    fn sin(_: libc::c_double) -> libc::c_double;
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
    fn cpArrayPush(arr: *mut cpArray, object: *mut libc::c_void);
    fn cpArrayDeleteObj(arr: *mut cpArray, obj: *mut libc::c_void);
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
pub type cpBodyType = libc::c_uint;
pub const CP_BODY_TYPE_STATIC: cpBodyType = 2;
pub const CP_BODY_TYPE_KINEMATIC: cpBodyType = 1;
pub const CP_BODY_TYPE_DYNAMIC: cpBodyType = 0;
pub type cpBodyShapeIteratorFunc = Option::<
    unsafe extern "C" fn(*mut cpBody, *mut cpShape, *mut libc::c_void) -> (),
>;
pub type cpBodyConstraintIteratorFunc = Option::<
    unsafe extern "C" fn(*mut cpBody, *mut cpConstraint, *mut libc::c_void) -> (),
>;
pub type cpBodyArbiterIteratorFunc = Option::<
    unsafe extern "C" fn(*mut cpBody, *mut cpArbiter, *mut libc::c_void) -> (),
>;
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
unsafe extern "C" fn cpvforangle(a: cpFloat) -> cpVect {
    return cpv(cos(a), sin(a));
}
#[inline]
unsafe extern "C" fn cpvlengthsq(v: cpVect) -> cpFloat {
    return cpvdot(v, v);
}
#[inline]
unsafe extern "C" fn cpvlerp(v1: cpVect, v2: cpVect, t: cpFloat) -> cpVect {
    return cpvadd(cpvmult(v1, 1.0f32 as libc::c_double - t), cpvmult(v2, t));
}
#[inline]
unsafe extern "C" fn cpvdistsq(v1: cpVect, v2: cpVect) -> cpFloat {
    return cpvlengthsq(cpvsub(v1, v2));
}
#[inline]
unsafe extern "C" fn cpTransformNewTranspose(
    mut a: cpFloat,
    mut c: cpFloat,
    mut tx: cpFloat,
    mut b: cpFloat,
    mut d: cpFloat,
    mut ty: cpFloat,
) -> cpTransform {
    let mut t: cpTransform = {
        let mut init = cpTransform {
            a: a,
            b: b,
            c: c,
            d: d,
            tx: tx,
            ty: ty,
        };
        init
    };
    return t;
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
unsafe extern "C" fn cpTransformRigidInverse(mut t: cpTransform) -> cpTransform {
    return cpTransformNewTranspose(
        t.d,
        -t.c,
        t.c * t.ty - t.tx * t.d,
        -t.b,
        t.a,
        t.tx * t.b - t.a * t.ty,
    );
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
unsafe extern "C" fn apply_impulse(mut body: *mut cpBody, mut j: cpVect, mut r: cpVect) {
    (*body).v = cpvadd((*body).v, cpvmult(j, (*body).m_inv));
    (*body).w += (*body).i_inv * cpvcross(r, j);
}
#[inline]
unsafe extern "C" fn cpConstraintNext(
    mut node: *mut cpConstraint,
    mut body: *mut cpBody,
) -> *mut cpConstraint {
    return if (*node).a == body { (*node).next_a } else { (*node).next_b };
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
pub unsafe extern "C" fn cpBodyAlloc() -> *mut cpBody {
    return calloc(
        1 as libc::c_int as libc::c_ulong,
        ::std::mem::size_of::<cpBody>() as libc::c_ulong,
    ) as *mut cpBody;
}
pub unsafe extern "C" fn cpBodyInit(
    mut body: *mut cpBody,
    mut mass: cpFloat,
    mut moment: cpFloat,
) -> *mut cpBody {
    (*body).space = 0 as *mut cpSpace;
    (*body).shapeList = 0 as *mut cpShape;
    (*body).arbiterList = 0 as *mut cpArbiter;
    (*body).constraintList = 0 as *mut cpConstraint;
    (*body)
        .velocity_func = Some(
        cpBodyUpdateVelocity
            as unsafe extern "C" fn(*mut cpBody, cpVect, cpFloat, cpFloat) -> (),
    );
    (*body)
        .position_func = Some(
        cpBodyUpdatePosition as unsafe extern "C" fn(*mut cpBody, cpFloat) -> (),
    );
    (*body).sleeping.root = 0 as *mut cpBody;
    (*body).sleeping.next = 0 as *mut cpBody;
    (*body).sleeping.idleTime = 0.0f32 as cpFloat;
    (*body).p = cpvzero;
    (*body).v = cpvzero;
    (*body).f = cpvzero;
    (*body).w = 0.0f32 as cpFloat;
    (*body).t = 0.0f32 as cpFloat;
    (*body).v_bias = cpvzero;
    (*body).w_bias = 0.0f32 as cpFloat;
    (*body).userData = 0 as *mut libc::c_void;
    cpBodySetMass(body, mass);
    cpBodySetMoment(body, moment);
    cpBodySetAngle(body, 0.0f32 as cpFloat);
    return body;
}
pub unsafe extern "C" fn cpBodyNew(
    mut mass: cpFloat,
    mut moment: cpFloat,
) -> *mut cpBody {
    return cpBodyInit(cpBodyAlloc(), mass, moment);
}
pub unsafe extern "C" fn cpBodyNewKinematic() -> *mut cpBody {
    let mut body: *mut cpBody = cpBodyNew(0.0f32 as cpFloat, 0.0f32 as cpFloat);
    cpBodySetType(body, CP_BODY_TYPE_KINEMATIC);
    return body;
}
pub unsafe extern "C" fn cpBodyNewStatic() -> *mut cpBody {
    let mut body: *mut cpBody = cpBodyNew(0.0f32 as cpFloat, 0.0f32 as cpFloat);
    cpBodySetType(body, CP_BODY_TYPE_STATIC);
    return body;
}
pub unsafe extern "C" fn cpBodyDestroy(mut body: *mut cpBody) {}
pub unsafe extern "C" fn cpBodyFree(mut body: *mut cpBody) {
    if !body.is_null() {
        cpBodyDestroy(body);
        free(body as *mut libc::c_void);
    }
}
pub unsafe extern "C" fn cpBodyIsSleeping(mut body: *const cpBody) -> cpBool {
    return ((*body).sleeping.root != 0 as *mut cpBody) as libc::c_int as cpBool;
}
pub unsafe extern "C" fn cpBodyGetType(mut body: *mut cpBody) -> cpBodyType {
    if (*body).sleeping.idleTime == ::std::f32::INFINITY as libc::c_double {
        return CP_BODY_TYPE_STATIC
    } else if (*body).m == ::std::f32::INFINITY as libc::c_double {
        return CP_BODY_TYPE_KINEMATIC
    } else {
        return CP_BODY_TYPE_DYNAMIC
    };
}
pub unsafe extern "C" fn cpBodySetType(mut body: *mut cpBody, mut type_0: cpBodyType) {
    let mut oldType: cpBodyType = cpBodyGetType(body);
    if oldType as libc::c_uint == type_0 as libc::c_uint {
        return;
    }
    (*body)
        .sleeping
        .idleTime = (if type_0 as libc::c_uint
        == CP_BODY_TYPE_STATIC as libc::c_int as libc::c_uint
    {
        ::std::f32::INFINITY
    } else {
        0.0f32
    }) as cpFloat;
    if type_0 as libc::c_uint == CP_BODY_TYPE_DYNAMIC as libc::c_int as libc::c_uint {
        (*body).i = 0.0f32 as cpFloat;
        (*body).m = (*body).i;
        (*body).i_inv = ::std::f32::INFINITY as cpFloat;
        (*body).m_inv = (*body).i_inv;
        cpBodyAccumulateMassFromShapes(body);
    } else {
        (*body).i = ::std::f32::INFINITY as cpFloat;
        (*body).m = (*body).i;
        (*body).i_inv = 0.0f32 as cpFloat;
        (*body).m_inv = (*body).i_inv;
        (*body).v = cpvzero;
        (*body).w = 0.0f32 as cpFloat;
    }
    let mut space: *mut cpSpace = cpBodyGetSpace(body);
    if !space.is_null() {
        if (*space).locked != 0 {
            cpMessage(
                b"!space->locked\0" as *const u8 as *const libc::c_char,
                b"../../src/cpBody.c\0" as *const u8 as *const libc::c_char,
                174 as libc::c_int,
                1 as libc::c_int,
                1 as libc::c_int,
                b"This operation cannot be done safely during a call to cpSpaceStep() or during a query. Put these calls into a post-step callback.\0"
                    as *const u8 as *const libc::c_char,
            );
            abort();
        }
        if !(oldType as libc::c_uint
            == CP_BODY_TYPE_STATIC as libc::c_int as libc::c_uint)
        {
            cpBodyActivate(body);
        }
        let mut fromArray: *mut cpArray = cpSpaceArrayForBodyType(space, oldType);
        let mut toArray: *mut cpArray = cpSpaceArrayForBodyType(space, type_0);
        if fromArray != toArray {
            cpArrayDeleteObj(fromArray, body as *mut libc::c_void);
            cpArrayPush(toArray, body as *mut libc::c_void);
        }
        let mut fromIndex: *mut cpSpatialIndex = if oldType as libc::c_uint
            == CP_BODY_TYPE_STATIC as libc::c_int as libc::c_uint
        {
            (*space).staticShapes
        } else {
            (*space).dynamicShapes
        };
        let mut toIndex: *mut cpSpatialIndex = if type_0 as libc::c_uint
            == CP_BODY_TYPE_STATIC as libc::c_int as libc::c_uint
        {
            (*space).staticShapes
        } else {
            (*space).dynamicShapes
        };
        if fromIndex != toIndex {
            let mut shape: *mut cpShape = (*body).shapeList;
            while !shape.is_null() {
                cpSpatialIndexRemove(
                    fromIndex,
                    shape as *mut libc::c_void,
                    (*shape).hashid,
                );
                cpSpatialIndexInsert(
                    toIndex,
                    shape as *mut libc::c_void,
                    (*shape).hashid,
                );
                shape = (*shape).next;
            }
        }
    }
}
pub unsafe extern "C" fn cpBodyAccumulateMassFromShapes(mut body: *mut cpBody) {
    if body.is_null()
        || cpBodyGetType(body) as libc::c_uint
            != CP_BODY_TYPE_DYNAMIC as libc::c_int as libc::c_uint
    {
        return;
    }
    (*body).i = 0.0f32 as cpFloat;
    (*body).m = (*body).i;
    (*body).cog = cpvzero;
    let mut pos: cpVect = cpBodyGetPosition(body);
    let mut shape: *mut cpShape = (*body).shapeList;
    while !shape.is_null() {
        let mut info: *mut cpShapeMassInfo = &mut (*shape).massInfo;
        let mut m: cpFloat = (*info).m;
        if m > 0.0f32 as libc::c_double {
            let mut msum: cpFloat = (*body).m + m;
            (*body).i
                += m * (*info).i
                    + cpvdistsq((*body).cog, (*info).cog) * (m * (*body).m) / msum;
            (*body).cog = cpvlerp((*body).cog, (*info).cog, m / msum);
            (*body).m = msum;
        }
        shape = (*shape).next;
    }
    (*body).m_inv = 1.0f32 as libc::c_double / (*body).m;
    (*body).i_inv = 1.0f32 as libc::c_double / (*body).i;
    cpBodySetPosition(body, pos);
}
pub unsafe extern "C" fn cpBodyGetSpace(mut body: *const cpBody) -> *mut cpSpace {
    return (*body).space;
}
pub unsafe extern "C" fn cpBodyGetMass(mut body: *const cpBody) -> cpFloat {
    return (*body).m;
}
pub unsafe extern "C" fn cpBodySetMass(mut body: *mut cpBody, mut mass: cpFloat) {
    if !(cpBodyGetType(body) as libc::c_uint
        == CP_BODY_TYPE_DYNAMIC as libc::c_int as libc::c_uint)
    {
        cpMessage(
            b"cpBodyGetType(body) == CP_BODY_TYPE_DYNAMIC\0" as *const u8
                as *const libc::c_char,
            b"../../src/cpBody.c\0" as *const u8 as *const libc::c_char,
            256 as libc::c_int,
            1 as libc::c_int,
            1 as libc::c_int,
            b"You cannot set the mass of kinematic or static bodies.\0" as *const u8
                as *const libc::c_char,
        );
        abort();
    }
    if !(0.0f32 as libc::c_double <= mass
        && mass < ::std::f32::INFINITY as libc::c_double)
    {
        cpMessage(
            b"0.0f <= mass && mass < INFINITY\0" as *const u8 as *const libc::c_char,
            b"../../src/cpBody.c\0" as *const u8 as *const libc::c_char,
            257 as libc::c_int,
            1 as libc::c_int,
            1 as libc::c_int,
            b"Mass must be positive and finite.\0" as *const u8 as *const libc::c_char,
        );
        abort();
    }
    cpBodyActivate(body);
    (*body).m = mass;
    (*body)
        .m_inv = if mass == 0.0f32 as libc::c_double {
        ::std::f32::INFINITY as libc::c_double
    } else {
        1.0f32 as libc::c_double / mass
    };
}
pub unsafe extern "C" fn cpBodyGetMoment(mut body: *const cpBody) -> cpFloat {
    return (*body).i;
}
pub unsafe extern "C" fn cpBodySetMoment(mut body: *mut cpBody, mut moment: cpFloat) {
    if !(moment >= 0.0f32 as libc::c_double) {
        cpMessage(
            b"moment >= 0.0f\0" as *const u8 as *const libc::c_char,
            b"../../src/cpBody.c\0" as *const u8 as *const libc::c_char,
            274 as libc::c_int,
            1 as libc::c_int,
            1 as libc::c_int,
            b"Moment of Inertia must be positive.\0" as *const u8 as *const libc::c_char,
        );
        abort();
    }
    cpBodyActivate(body);
    (*body).i = moment;
    (*body)
        .i_inv = if moment == 0.0f32 as libc::c_double {
        ::std::f32::INFINITY as libc::c_double
    } else {
        1.0f32 as libc::c_double / moment
    };
}
pub unsafe extern "C" fn cpBodyGetRotation(mut body: *const cpBody) -> cpVect {
    return cpv((*body).transform.a, (*body).transform.b);
}
pub unsafe extern "C" fn cpBodyAddShape(mut body: *mut cpBody, mut shape: *mut cpShape) {
    let mut next: *mut cpShape = (*body).shapeList;
    if !next.is_null() {
        (*next).prev = shape;
    }
    (*shape).next = next;
    (*body).shapeList = shape;
    if (*shape).massInfo.m > 0.0f32 as libc::c_double {
        cpBodyAccumulateMassFromShapes(body);
    }
}
pub unsafe extern "C" fn cpBodyRemoveShape(
    mut body: *mut cpBody,
    mut shape: *mut cpShape,
) {
    let mut prev: *mut cpShape = (*shape).prev;
    let mut next: *mut cpShape = (*shape).next;
    if !prev.is_null() {
        (*prev).next = next;
    } else {
        (*body).shapeList = next;
    }
    if !next.is_null() {
        (*next).prev = prev;
    }
    (*shape).prev = 0 as *mut cpShape;
    (*shape).next = 0 as *mut cpShape;
    if cpBodyGetType(body) as libc::c_uint
        == CP_BODY_TYPE_DYNAMIC as libc::c_int as libc::c_uint
        && (*shape).massInfo.m > 0.0f32 as libc::c_double
    {
        cpBodyAccumulateMassFromShapes(body);
    }
}
unsafe extern "C" fn filterConstraints(
    mut node: *mut cpConstraint,
    mut body: *mut cpBody,
    mut filter: *mut cpConstraint,
) -> *mut cpConstraint {
    if node == filter {
        return cpConstraintNext(node, body)
    } else if (*node).a == body {
        (*node).next_a = filterConstraints((*node).next_a, body, filter);
    } else {
        (*node).next_b = filterConstraints((*node).next_b, body, filter);
    }
    return node;
}
pub unsafe extern "C" fn cpBodyRemoveConstraint(
    mut body: *mut cpBody,
    mut constraint: *mut cpConstraint,
) {
    (*body).constraintList = filterConstraints((*body).constraintList, body, constraint);
}
unsafe extern "C" fn SetTransform(mut body: *mut cpBody, mut p: cpVect, mut a: cpFloat) {
    let mut rot: cpVect = cpvforangle(a);
    let mut c: cpVect = (*body).cog;
    (*body)
        .transform = cpTransformNewTranspose(
        rot.x,
        -rot.y,
        p.x - (c.x * rot.x - c.y * rot.y),
        rot.y,
        rot.x,
        p.y - (c.x * rot.y + c.y * rot.x),
    );
}
#[inline]
unsafe extern "C" fn SetAngle(mut body: *mut cpBody, mut a: cpFloat) -> cpFloat {
    (*body).a = a;
    return a;
}
pub unsafe extern "C" fn cpBodyGetPosition(mut body: *const cpBody) -> cpVect {
    return cpTransformPoint((*body).transform, cpvzero);
}
pub unsafe extern "C" fn cpBodySetPosition(mut body: *mut cpBody, mut position: cpVect) {
    cpBodyActivate(body);
    (*body).p = cpvadd(cpTransformVect((*body).transform, (*body).cog), position);
    let mut p: cpVect = (*body).p;
    SetTransform(body, p, (*body).a);
}
pub unsafe extern "C" fn cpBodyGetCenterOfGravity(mut body: *const cpBody) -> cpVect {
    return (*body).cog;
}
pub unsafe extern "C" fn cpBodySetCenterOfGravity(
    mut body: *mut cpBody,
    mut cog: cpVect,
) {
    cpBodyActivate(body);
    (*body).cog = cog;
}
pub unsafe extern "C" fn cpBodyGetVelocity(mut body: *const cpBody) -> cpVect {
    return (*body).v;
}
pub unsafe extern "C" fn cpBodySetVelocity(mut body: *mut cpBody, mut velocity: cpVect) {
    cpBodyActivate(body);
    (*body).v = velocity;
}
pub unsafe extern "C" fn cpBodyGetForce(mut body: *const cpBody) -> cpVect {
    return (*body).f;
}
pub unsafe extern "C" fn cpBodySetForce(mut body: *mut cpBody, mut force: cpVect) {
    cpBodyActivate(body);
    (*body).f = force;
}
pub unsafe extern "C" fn cpBodyGetAngle(mut body: *const cpBody) -> cpFloat {
    return (*body).a;
}
pub unsafe extern "C" fn cpBodySetAngle(mut body: *mut cpBody, mut angle: cpFloat) {
    cpBodyActivate(body);
    SetAngle(body, angle);
    SetTransform(body, (*body).p, angle);
}
pub unsafe extern "C" fn cpBodyGetAngularVelocity(mut body: *const cpBody) -> cpFloat {
    return (*body).w;
}
pub unsafe extern "C" fn cpBodySetAngularVelocity(
    mut body: *mut cpBody,
    mut angularVelocity: cpFloat,
) {
    cpBodyActivate(body);
    (*body).w = angularVelocity;
}
pub unsafe extern "C" fn cpBodyGetTorque(mut body: *const cpBody) -> cpFloat {
    return (*body).t;
}
pub unsafe extern "C" fn cpBodySetTorque(mut body: *mut cpBody, mut torque: cpFloat) {
    cpBodyActivate(body);
    (*body).t = torque;
}
pub unsafe extern "C" fn cpBodyGetUserData(mut body: *const cpBody) -> cpDataPointer {
    return (*body).userData;
}
pub unsafe extern "C" fn cpBodySetUserData(
    mut body: *mut cpBody,
    mut userData: cpDataPointer,
) {
    (*body).userData = userData;
}
pub unsafe extern "C" fn cpBodySetVelocityUpdateFunc(
    mut body: *mut cpBody,
    mut velocityFunc: cpBodyVelocityFunc,
) {
    (*body).velocity_func = velocityFunc;
}
pub unsafe extern "C" fn cpBodySetPositionUpdateFunc(
    mut body: *mut cpBody,
    mut positionFunc: cpBodyPositionFunc,
) {
    (*body).position_func = positionFunc;
}
pub unsafe extern "C" fn cpBodyUpdateVelocity(
    mut body: *mut cpBody,
    mut gravity: cpVect,
    mut damping: cpFloat,
    mut dt: cpFloat,
) {
    if cpBodyGetType(body) as libc::c_uint
        == CP_BODY_TYPE_KINEMATIC as libc::c_int as libc::c_uint
    {
        return;
    }
    (*body)
        .v = cpvadd(
        cpvmult((*body).v, damping),
        cpvmult(cpvadd(gravity, cpvmult((*body).f, (*body).m_inv)), dt),
    );
    (*body).w = (*body).w * damping + (*body).t * (*body).i_inv * dt;
    (*body).f = cpvzero;
    (*body).t = 0.0f32 as cpFloat;
}
pub unsafe extern "C" fn cpBodyUpdatePosition(mut body: *mut cpBody, mut dt: cpFloat) {
    (*body).p = cpvadd((*body).p, cpvmult(cpvadd((*body).v, (*body).v_bias), dt));
    let mut p: cpVect = (*body).p;
    let mut a: cpFloat = SetAngle(body, (*body).a + ((*body).w + (*body).w_bias) * dt);
    SetTransform(body, p, a);
    (*body).v_bias = cpvzero;
    (*body).w_bias = 0.0f32 as cpFloat;
}
pub unsafe extern "C" fn cpBodyLocalToWorld(
    mut body: *const cpBody,
    point: cpVect,
) -> cpVect {
    return cpTransformPoint((*body).transform, point);
}
pub unsafe extern "C" fn cpBodyWorldToLocal(
    mut body: *const cpBody,
    point: cpVect,
) -> cpVect {
    return cpTransformPoint(cpTransformRigidInverse((*body).transform), point);
}
pub unsafe extern "C" fn cpBodyApplyForceAtWorldPoint(
    mut body: *mut cpBody,
    mut force: cpVect,
    mut point: cpVect,
) {
    cpBodyActivate(body);
    (*body).f = cpvadd((*body).f, force);
    let mut r: cpVect = cpvsub(point, cpTransformPoint((*body).transform, (*body).cog));
    (*body).t += cpvcross(r, force);
}
pub unsafe extern "C" fn cpBodyApplyForceAtLocalPoint(
    mut body: *mut cpBody,
    mut force: cpVect,
    mut point: cpVect,
) {
    cpBodyApplyForceAtWorldPoint(
        body,
        cpTransformVect((*body).transform, force),
        cpTransformPoint((*body).transform, point),
    );
}
pub unsafe extern "C" fn cpBodyApplyImpulseAtWorldPoint(
    mut body: *mut cpBody,
    mut impulse: cpVect,
    mut point: cpVect,
) {
    cpBodyActivate(body);
    let mut r: cpVect = cpvsub(point, cpTransformPoint((*body).transform, (*body).cog));
    apply_impulse(body, impulse, r);
}
pub unsafe extern "C" fn cpBodyApplyImpulseAtLocalPoint(
    mut body: *mut cpBody,
    mut impulse: cpVect,
    mut point: cpVect,
) {
    cpBodyApplyImpulseAtWorldPoint(
        body,
        cpTransformVect((*body).transform, impulse),
        cpTransformPoint((*body).transform, point),
    );
}
pub unsafe extern "C" fn cpBodyGetVelocityAtLocalPoint(
    mut body: *const cpBody,
    mut point: cpVect,
) -> cpVect {
    let mut r: cpVect = cpTransformVect((*body).transform, cpvsub(point, (*body).cog));
    return cpvadd((*body).v, cpvmult(cpvperp(r), (*body).w));
}
pub unsafe extern "C" fn cpBodyGetVelocityAtWorldPoint(
    mut body: *const cpBody,
    mut point: cpVect,
) -> cpVect {
    let mut r: cpVect = cpvsub(point, cpTransformPoint((*body).transform, (*body).cog));
    return cpvadd((*body).v, cpvmult(cpvperp(r), (*body).w));
}
pub unsafe extern "C" fn cpBodyKineticEnergy(mut body: *const cpBody) -> cpFloat {
    let mut vsq: cpFloat = cpvdot((*body).v, (*body).v);
    let mut wsq: cpFloat = (*body).w * (*body).w;
    return (if vsq != 0. { vsq * (*body).m } else { 0.0f32 as libc::c_double })
        + (if wsq != 0. { wsq * (*body).i } else { 0.0f32 as libc::c_double });
}
pub unsafe extern "C" fn cpBodyEachShape(
    mut body: *mut cpBody,
    mut func: cpBodyShapeIteratorFunc,
    mut data: *mut libc::c_void,
) {
    let mut shape: *mut cpShape = (*body).shapeList;
    while !shape.is_null() {
        let mut next: *mut cpShape = (*shape).next;
        func.unwrap()(body, shape, data);
        shape = next;
    }
}
pub unsafe extern "C" fn cpBodyEachConstraint(
    mut body: *mut cpBody,
    mut func: cpBodyConstraintIteratorFunc,
    mut data: *mut libc::c_void,
) {
    let mut constraint: *mut cpConstraint = (*body).constraintList;
    while !constraint.is_null() {
        let mut next: *mut cpConstraint = cpConstraintNext(constraint, body);
        func.unwrap()(body, constraint, data);
        constraint = next;
    }
}
pub unsafe extern "C" fn cpBodyEachArbiter(
    mut body: *mut cpBody,
    mut func: cpBodyArbiterIteratorFunc,
    mut data: *mut libc::c_void,
) {
    let mut arb: *mut cpArbiter = (*body).arbiterList;
    while !arb.is_null() {
        let mut next: *mut cpArbiter = cpArbiterNext(arb, body);
        let mut swapped: cpBool = (*arb).swapped;
        (*arb).swapped = (body == (*arb).body_b) as libc::c_int as cpBool;
        func.unwrap()(body, arb, data);
        (*arb).swapped = swapped;
        arb = next;
    }
}
