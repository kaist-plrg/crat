use ::libc;
extern "C" {
    pub type cpHashSet;
    pub type cpContactBufferHeader;
    fn cpSpaceUnlock(space: *mut cpSpace, runPostStep: cpBool);
    fn cpSpaceLock(space: *mut cpSpace);
    fn cpShapeUpdate(shape: *mut cpShape, transform: cpTransform) -> cpBB;
    fn cpShapePointQuery(
        shape: *const cpShape,
        p: cpVect,
        out: *mut cpPointQueryInfo,
    ) -> cpFloat;
    fn cpShapeSegmentQuery(
        shape: *const cpShape,
        a: cpVect,
        b: cpVect,
        radius: cpFloat,
        info: *mut cpSegmentQueryInfo,
    ) -> cpBool;
    fn cpShapesCollide(a: *const cpShape, b: *const cpShape) -> cpContactPointSet;
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
pub type cpSpacePointQueryFunc = Option::<
    unsafe extern "C" fn(*mut cpShape, cpVect, cpFloat, cpVect, *mut libc::c_void) -> (),
>;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct PointQueryContext {
    pub point: cpVect,
    pub maxDistance: cpFloat,
    pub filter: cpShapeFilter,
    pub func: cpSpacePointQueryFunc,
}
pub type cpSpaceSegmentQueryFunc = Option::<
    unsafe extern "C" fn(*mut cpShape, cpVect, cpVect, cpFloat, *mut libc::c_void) -> (),
>;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct SegmentQueryContext {
    pub start: cpVect,
    pub end: cpVect,
    pub radius: cpFloat,
    pub filter: cpShapeFilter,
    pub func: cpSpaceSegmentQueryFunc,
}
pub type cpSpaceBBQueryFunc = Option::<
    unsafe extern "C" fn(*mut cpShape, *mut libc::c_void) -> (),
>;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct BBQueryContext {
    pub bb: cpBB,
    pub filter: cpShapeFilter,
    pub func: cpSpaceBBQueryFunc,
}
pub type cpSpaceShapeQueryFunc = Option::<
    unsafe extern "C" fn(*mut cpShape, *mut cpContactPointSet, *mut libc::c_void) -> (),
>;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ShapeQueryContext {
    pub func: cpSpaceShapeQueryFunc,
    pub data: *mut libc::c_void,
    pub anyCollision: cpBool,
}
#[inline]
unsafe extern "C" fn cpfmax(mut a: cpFloat, mut b: cpFloat) -> cpFloat {
    return if a > b { a } else { b };
}
static mut cpvzero: cpVect = {
    let mut init = cpVect {
        x: 0.0f32 as cpFloat,
        y: 0.0f32 as cpFloat,
    };
    init
};
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
unsafe extern "C" fn cpBBIntersects(a: cpBB, b: cpBB) -> cpBool {
    return (a.l <= b.r && b.l <= a.r && a.b <= b.t && b.b <= a.t) as libc::c_int
        as cpBool;
}
#[inline]
unsafe extern "C" fn cpSpatialIndexQuery(
    mut index: *mut cpSpatialIndex,
    mut obj: *mut libc::c_void,
    mut bb: cpBB,
    mut func: cpSpatialIndexQueryFunc,
    mut data: *mut libc::c_void,
) {
    ((*(*index).klass).query).unwrap()(index, obj, bb, func, data);
}
#[inline]
unsafe extern "C" fn cpSpatialIndexSegmentQuery(
    mut index: *mut cpSpatialIndex,
    mut obj: *mut libc::c_void,
    mut a: cpVect,
    mut b: cpVect,
    mut t_exit: cpFloat,
    mut func: cpSpatialIndexSegmentQueryFunc,
    mut data: *mut libc::c_void,
) {
    ((*(*index).klass).segmentQuery).unwrap()(index, obj, a, b, t_exit, func, data);
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
unsafe extern "C" fn NearestPointQuery(
    mut context: *mut PointQueryContext,
    mut shape: *mut cpShape,
    mut id: cpCollisionID,
    mut data: *mut libc::c_void,
) -> cpCollisionID {
    if cpShapeFilterReject((*shape).filter, (*context).filter) == 0 {
        let mut info: cpPointQueryInfo = cpPointQueryInfo {
            shape: 0 as *const cpShape,
            point: cpVect { x: 0., y: 0. },
            distance: 0.,
            gradient: cpVect { x: 0., y: 0. },
        };
        cpShapePointQuery(shape, (*context).point, &mut info);
        if !(info.shape).is_null() && info.distance < (*context).maxDistance {
            ((*context).func)
                .unwrap()(shape, info.point, info.distance, info.gradient, data);
        }
    }
    return id;
}
pub unsafe extern "C" fn cpSpacePointQuery(
    mut space: *mut cpSpace,
    mut point: cpVect,
    mut maxDistance: cpFloat,
    mut filter: cpShapeFilter,
    mut func: cpSpacePointQueryFunc,
    mut data: *mut libc::c_void,
) {
    let mut context: PointQueryContext = {
        let mut init = PointQueryContext {
            point: point,
            maxDistance: maxDistance,
            filter: filter,
            func: func,
        };
        init
    };
    let mut bb: cpBB = cpBBNewForCircle(point, cpfmax(maxDistance, 0.0f32 as cpFloat));
    cpSpaceLock(space);
    cpSpatialIndexQuery(
        (*space).dynamicShapes,
        &mut context as *mut PointQueryContext as *mut libc::c_void,
        bb,
        ::std::mem::transmute::<
            Option::<
                unsafe extern "C" fn(
                    *mut PointQueryContext,
                    *mut cpShape,
                    cpCollisionID,
                    *mut libc::c_void,
                ) -> cpCollisionID,
            >,
            cpSpatialIndexQueryFunc,
        >(
            Some(
                NearestPointQuery
                    as unsafe extern "C" fn(
                        *mut PointQueryContext,
                        *mut cpShape,
                        cpCollisionID,
                        *mut libc::c_void,
                    ) -> cpCollisionID,
            ),
        ),
        data,
    );
    cpSpatialIndexQuery(
        (*space).staticShapes,
        &mut context as *mut PointQueryContext as *mut libc::c_void,
        bb,
        ::std::mem::transmute::<
            Option::<
                unsafe extern "C" fn(
                    *mut PointQueryContext,
                    *mut cpShape,
                    cpCollisionID,
                    *mut libc::c_void,
                ) -> cpCollisionID,
            >,
            cpSpatialIndexQueryFunc,
        >(
            Some(
                NearestPointQuery
                    as unsafe extern "C" fn(
                        *mut PointQueryContext,
                        *mut cpShape,
                        cpCollisionID,
                        *mut libc::c_void,
                    ) -> cpCollisionID,
            ),
        ),
        data,
    );
    cpSpaceUnlock(space, 1 as libc::c_int as cpBool);
}
unsafe extern "C" fn NearestPointQueryNearest(
    mut context: *mut PointQueryContext,
    mut shape: *mut cpShape,
    mut id: cpCollisionID,
    mut out: *mut cpPointQueryInfo,
) -> cpCollisionID {
    if cpShapeFilterReject((*shape).filter, (*context).filter) == 0
        && (*shape).sensor == 0
    {
        let mut info: cpPointQueryInfo = cpPointQueryInfo {
            shape: 0 as *const cpShape,
            point: cpVect { x: 0., y: 0. },
            distance: 0.,
            gradient: cpVect { x: 0., y: 0. },
        };
        cpShapePointQuery(shape, (*context).point, &mut info);
        if info.distance < (*out).distance {
            *out = info;
        }
    }
    return id;
}
pub unsafe extern "C" fn cpSpacePointQueryNearest(
    mut space: *mut cpSpace,
    mut point: cpVect,
    mut maxDistance: cpFloat,
    mut filter: cpShapeFilter,
    mut out: *mut cpPointQueryInfo,
) -> *mut cpShape {
    let mut info: cpPointQueryInfo = {
        let mut init = cpPointQueryInfo {
            shape: 0 as *const cpShape,
            point: cpvzero,
            distance: maxDistance,
            gradient: cpvzero,
        };
        init
    };
    if !out.is_null() {
        *out = info;
    } else {
        out = &mut info;
    }
    let mut context: PointQueryContext = {
        let mut init = PointQueryContext {
            point: point,
            maxDistance: maxDistance,
            filter: filter,
            func: None,
        };
        init
    };
    let mut bb: cpBB = cpBBNewForCircle(point, cpfmax(maxDistance, 0.0f32 as cpFloat));
    cpSpatialIndexQuery(
        (*space).dynamicShapes,
        &mut context as *mut PointQueryContext as *mut libc::c_void,
        bb,
        ::std::mem::transmute::<
            Option::<
                unsafe extern "C" fn(
                    *mut PointQueryContext,
                    *mut cpShape,
                    cpCollisionID,
                    *mut cpPointQueryInfo,
                ) -> cpCollisionID,
            >,
            cpSpatialIndexQueryFunc,
        >(
            Some(
                NearestPointQueryNearest
                    as unsafe extern "C" fn(
                        *mut PointQueryContext,
                        *mut cpShape,
                        cpCollisionID,
                        *mut cpPointQueryInfo,
                    ) -> cpCollisionID,
            ),
        ),
        out as *mut libc::c_void,
    );
    cpSpatialIndexQuery(
        (*space).staticShapes,
        &mut context as *mut PointQueryContext as *mut libc::c_void,
        bb,
        ::std::mem::transmute::<
            Option::<
                unsafe extern "C" fn(
                    *mut PointQueryContext,
                    *mut cpShape,
                    cpCollisionID,
                    *mut cpPointQueryInfo,
                ) -> cpCollisionID,
            >,
            cpSpatialIndexQueryFunc,
        >(
            Some(
                NearestPointQueryNearest
                    as unsafe extern "C" fn(
                        *mut PointQueryContext,
                        *mut cpShape,
                        cpCollisionID,
                        *mut cpPointQueryInfo,
                    ) -> cpCollisionID,
            ),
        ),
        out as *mut libc::c_void,
    );
    return (*out).shape as *mut cpShape;
}
unsafe extern "C" fn SegmentQuery(
    mut context: *mut SegmentQueryContext,
    mut shape: *mut cpShape,
    mut data: *mut libc::c_void,
) -> cpFloat {
    let mut info: cpSegmentQueryInfo = cpSegmentQueryInfo {
        shape: 0 as *const cpShape,
        point: cpVect { x: 0., y: 0. },
        normal: cpVect { x: 0., y: 0. },
        alpha: 0.,
    };
    if cpShapeFilterReject((*shape).filter, (*context).filter) == 0
        && cpShapeSegmentQuery(
            shape,
            (*context).start,
            (*context).end,
            (*context).radius,
            &mut info,
        ) as libc::c_int != 0
    {
        ((*context).func).unwrap()(shape, info.point, info.normal, info.alpha, data);
    }
    return 1.0f32 as cpFloat;
}
pub unsafe extern "C" fn cpSpaceSegmentQuery(
    mut space: *mut cpSpace,
    mut start: cpVect,
    mut end: cpVect,
    mut radius: cpFloat,
    mut filter: cpShapeFilter,
    mut func: cpSpaceSegmentQueryFunc,
    mut data: *mut libc::c_void,
) {
    let mut context: SegmentQueryContext = {
        let mut init = SegmentQueryContext {
            start: start,
            end: end,
            radius: radius,
            filter: filter,
            func: func,
        };
        init
    };
    cpSpaceLock(space);
    cpSpatialIndexSegmentQuery(
        (*space).staticShapes,
        &mut context as *mut SegmentQueryContext as *mut libc::c_void,
        start,
        end,
        1.0f32 as cpFloat,
        ::std::mem::transmute::<
            Option::<
                unsafe extern "C" fn(
                    *mut SegmentQueryContext,
                    *mut cpShape,
                    *mut libc::c_void,
                ) -> cpFloat,
            >,
            cpSpatialIndexSegmentQueryFunc,
        >(
            Some(
                SegmentQuery
                    as unsafe extern "C" fn(
                        *mut SegmentQueryContext,
                        *mut cpShape,
                        *mut libc::c_void,
                    ) -> cpFloat,
            ),
        ),
        data,
    );
    cpSpatialIndexSegmentQuery(
        (*space).dynamicShapes,
        &mut context as *mut SegmentQueryContext as *mut libc::c_void,
        start,
        end,
        1.0f32 as cpFloat,
        ::std::mem::transmute::<
            Option::<
                unsafe extern "C" fn(
                    *mut SegmentQueryContext,
                    *mut cpShape,
                    *mut libc::c_void,
                ) -> cpFloat,
            >,
            cpSpatialIndexSegmentQueryFunc,
        >(
            Some(
                SegmentQuery
                    as unsafe extern "C" fn(
                        *mut SegmentQueryContext,
                        *mut cpShape,
                        *mut libc::c_void,
                    ) -> cpFloat,
            ),
        ),
        data,
    );
    cpSpaceUnlock(space, 1 as libc::c_int as cpBool);
}
unsafe extern "C" fn SegmentQueryFirst(
    mut context: *mut SegmentQueryContext,
    mut shape: *mut cpShape,
    mut out: *mut cpSegmentQueryInfo,
) -> cpFloat {
    let mut info: cpSegmentQueryInfo = cpSegmentQueryInfo {
        shape: 0 as *const cpShape,
        point: cpVect { x: 0., y: 0. },
        normal: cpVect { x: 0., y: 0. },
        alpha: 0.,
    };
    if cpShapeFilterReject((*shape).filter, (*context).filter) == 0
        && (*shape).sensor == 0
        && cpShapeSegmentQuery(
            shape,
            (*context).start,
            (*context).end,
            (*context).radius,
            &mut info,
        ) as libc::c_int != 0 && info.alpha < (*out).alpha
    {
        *out = info;
    }
    return (*out).alpha;
}
pub unsafe extern "C" fn cpSpaceSegmentQueryFirst(
    mut space: *mut cpSpace,
    mut start: cpVect,
    mut end: cpVect,
    mut radius: cpFloat,
    mut filter: cpShapeFilter,
    mut out: *mut cpSegmentQueryInfo,
) -> *mut cpShape {
    let mut info: cpSegmentQueryInfo = {
        let mut init = cpSegmentQueryInfo {
            shape: 0 as *const cpShape,
            point: end,
            normal: cpvzero,
            alpha: 1.0f32 as cpFloat,
        };
        init
    };
    if !out.is_null() {
        *out = info;
    } else {
        out = &mut info;
    }
    let mut context: SegmentQueryContext = {
        let mut init = SegmentQueryContext {
            start: start,
            end: end,
            radius: radius,
            filter: filter,
            func: None,
        };
        init
    };
    cpSpatialIndexSegmentQuery(
        (*space).staticShapes,
        &mut context as *mut SegmentQueryContext as *mut libc::c_void,
        start,
        end,
        1.0f32 as cpFloat,
        ::std::mem::transmute::<
            Option::<
                unsafe extern "C" fn(
                    *mut SegmentQueryContext,
                    *mut cpShape,
                    *mut cpSegmentQueryInfo,
                ) -> cpFloat,
            >,
            cpSpatialIndexSegmentQueryFunc,
        >(
            Some(
                SegmentQueryFirst
                    as unsafe extern "C" fn(
                        *mut SegmentQueryContext,
                        *mut cpShape,
                        *mut cpSegmentQueryInfo,
                    ) -> cpFloat,
            ),
        ),
        out as *mut libc::c_void,
    );
    cpSpatialIndexSegmentQuery(
        (*space).dynamicShapes,
        &mut context as *mut SegmentQueryContext as *mut libc::c_void,
        start,
        end,
        (*out).alpha,
        ::std::mem::transmute::<
            Option::<
                unsafe extern "C" fn(
                    *mut SegmentQueryContext,
                    *mut cpShape,
                    *mut cpSegmentQueryInfo,
                ) -> cpFloat,
            >,
            cpSpatialIndexSegmentQueryFunc,
        >(
            Some(
                SegmentQueryFirst
                    as unsafe extern "C" fn(
                        *mut SegmentQueryContext,
                        *mut cpShape,
                        *mut cpSegmentQueryInfo,
                    ) -> cpFloat,
            ),
        ),
        out as *mut libc::c_void,
    );
    return (*out).shape as *mut cpShape;
}
unsafe extern "C" fn BBQuery(
    mut context: *mut BBQueryContext,
    mut shape: *mut cpShape,
    mut id: cpCollisionID,
    mut data: *mut libc::c_void,
) -> cpCollisionID {
    if cpShapeFilterReject((*shape).filter, (*context).filter) == 0
        && cpBBIntersects((*context).bb, (*shape).bb) as libc::c_int != 0
    {
        ((*context).func).unwrap()(shape, data);
    }
    return id;
}
pub unsafe extern "C" fn cpSpaceBBQuery(
    mut space: *mut cpSpace,
    mut bb: cpBB,
    mut filter: cpShapeFilter,
    mut func: cpSpaceBBQueryFunc,
    mut data: *mut libc::c_void,
) {
    let mut context: BBQueryContext = {
        let mut init = BBQueryContext {
            bb: bb,
            filter: filter,
            func: func,
        };
        init
    };
    cpSpaceLock(space);
    cpSpatialIndexQuery(
        (*space).dynamicShapes,
        &mut context as *mut BBQueryContext as *mut libc::c_void,
        bb,
        ::std::mem::transmute::<
            Option::<
                unsafe extern "C" fn(
                    *mut BBQueryContext,
                    *mut cpShape,
                    cpCollisionID,
                    *mut libc::c_void,
                ) -> cpCollisionID,
            >,
            cpSpatialIndexQueryFunc,
        >(
            Some(
                BBQuery
                    as unsafe extern "C" fn(
                        *mut BBQueryContext,
                        *mut cpShape,
                        cpCollisionID,
                        *mut libc::c_void,
                    ) -> cpCollisionID,
            ),
        ),
        data,
    );
    cpSpatialIndexQuery(
        (*space).staticShapes,
        &mut context as *mut BBQueryContext as *mut libc::c_void,
        bb,
        ::std::mem::transmute::<
            Option::<
                unsafe extern "C" fn(
                    *mut BBQueryContext,
                    *mut cpShape,
                    cpCollisionID,
                    *mut libc::c_void,
                ) -> cpCollisionID,
            >,
            cpSpatialIndexQueryFunc,
        >(
            Some(
                BBQuery
                    as unsafe extern "C" fn(
                        *mut BBQueryContext,
                        *mut cpShape,
                        cpCollisionID,
                        *mut libc::c_void,
                    ) -> cpCollisionID,
            ),
        ),
        data,
    );
    cpSpaceUnlock(space, 1 as libc::c_int as cpBool);
}
unsafe extern "C" fn ShapeQuery(
    mut a: *mut cpShape,
    mut b: *mut cpShape,
    mut id: cpCollisionID,
    mut context: *mut ShapeQueryContext,
) -> cpCollisionID {
    if cpShapeFilterReject((*a).filter, (*b).filter) as libc::c_int != 0 || a == b {
        return id;
    }
    let mut set: cpContactPointSet = cpShapesCollide(a, b);
    if set.count != 0 {
        if ((*context).func).is_some() {
            ((*context).func).unwrap()(b, &mut set, (*context).data);
        }
        (*context)
            .anyCollision = !((*a).sensor as libc::c_int != 0
            || (*b).sensor as libc::c_int != 0) as libc::c_int as cpBool;
    }
    return id;
}
pub unsafe extern "C" fn cpSpaceShapeQuery(
    mut space: *mut cpSpace,
    mut shape: *mut cpShape,
    mut func: cpSpaceShapeQueryFunc,
    mut data: *mut libc::c_void,
) -> cpBool {
    let mut body: *mut cpBody = (*shape).body;
    let mut bb: cpBB = if !body.is_null() {
        cpShapeUpdate(shape, (*body).transform)
    } else {
        (*shape).bb
    };
    let mut context: ShapeQueryContext = {
        let mut init = ShapeQueryContext {
            func: func,
            data: data,
            anyCollision: 0 as libc::c_int as cpBool,
        };
        init
    };
    cpSpaceLock(space);
    cpSpatialIndexQuery(
        (*space).dynamicShapes,
        shape as *mut libc::c_void,
        bb,
        ::std::mem::transmute::<
            Option::<
                unsafe extern "C" fn(
                    *mut cpShape,
                    *mut cpShape,
                    cpCollisionID,
                    *mut ShapeQueryContext,
                ) -> cpCollisionID,
            >,
            cpSpatialIndexQueryFunc,
        >(
            Some(
                ShapeQuery
                    as unsafe extern "C" fn(
                        *mut cpShape,
                        *mut cpShape,
                        cpCollisionID,
                        *mut ShapeQueryContext,
                    ) -> cpCollisionID,
            ),
        ),
        &mut context as *mut ShapeQueryContext as *mut libc::c_void,
    );
    cpSpatialIndexQuery(
        (*space).staticShapes,
        shape as *mut libc::c_void,
        bb,
        ::std::mem::transmute::<
            Option::<
                unsafe extern "C" fn(
                    *mut cpShape,
                    *mut cpShape,
                    cpCollisionID,
                    *mut ShapeQueryContext,
                ) -> cpCollisionID,
            >,
            cpSpatialIndexQueryFunc,
        >(
            Some(
                ShapeQuery
                    as unsafe extern "C" fn(
                        *mut cpShape,
                        *mut cpShape,
                        cpCollisionID,
                        *mut ShapeQueryContext,
                    ) -> cpCollisionID,
            ),
        ),
        &mut context as *mut ShapeQueryContext as *mut libc::c_void,
    );
    cpSpaceUnlock(space, 1 as libc::c_int as cpBool);
    return context.anyCollision;
}
