use ::libc;
extern "C" {
    pub type cpHashSet;
    pub type cpContactBufferHeader;
    fn sqrt(_: libc::c_double) -> libc::c_double;
    fn cpConstraintIsPinJoint(constraint: *const cpConstraint) -> cpBool;
    fn cpConstraintIsSlideJoint(constraint: *const cpConstraint) -> cpBool;
    fn cpConstraintIsPivotJoint(constraint: *const cpConstraint) -> cpBool;
    fn cpConstraintIsGrooveJoint(constraint: *const cpConstraint) -> cpBool;
    fn cpConstraintIsDampedSpring(constraint: *const cpConstraint) -> cpBool;
    fn cpSpaceEachShape(
        space: *mut cpSpace,
        func: cpSpaceShapeIteratorFunc,
        data: *mut libc::c_void,
    );
    fn cpSpaceEachConstraint(
        space: *mut cpSpace,
        func: cpSpaceConstraintIteratorFunc,
        data: *mut libc::c_void,
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
pub struct cpPinJoint {
    pub constraint: cpConstraint,
    pub anchorA: cpVect,
    pub anchorB: cpVect,
    pub dist: cpFloat,
    pub r1: cpVect,
    pub r2: cpVect,
    pub n: cpVect,
    pub nMass: cpFloat,
    pub jnAcc: cpFloat,
    pub bias: cpFloat,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct cpSlideJoint {
    pub constraint: cpConstraint,
    pub anchorA: cpVect,
    pub anchorB: cpVect,
    pub min: cpFloat,
    pub max: cpFloat,
    pub r1: cpVect,
    pub r2: cpVect,
    pub n: cpVect,
    pub nMass: cpFloat,
    pub jnAcc: cpFloat,
    pub bias: cpFloat,
}
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct cpGrooveJoint {
    pub constraint: cpConstraint,
    pub grv_n: cpVect,
    pub grv_a: cpVect,
    pub grv_b: cpVect,
    pub anchorB: cpVect,
    pub grv_tn: cpVect,
    pub clamp: cpFloat,
    pub r1: cpVect,
    pub r2: cpVect,
    pub k: cpMat2x2,
    pub jAcc: cpVect,
    pub bias: cpVect,
}
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
pub type cpSpaceShapeIteratorFunc = Option::<
    unsafe extern "C" fn(*mut cpShape, *mut libc::c_void) -> (),
>;
pub type cpSpaceConstraintIteratorFunc = Option::<
    unsafe extern "C" fn(*mut cpConstraint, *mut libc::c_void) -> (),
>;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct cpSpaceDebugColor {
    pub r: libc::c_float,
    pub g: libc::c_float,
    pub b: libc::c_float,
    pub a: libc::c_float,
}
pub type cpSpaceDebugDrawCircleImpl = Option::<
    unsafe extern "C" fn(
        cpVect,
        cpFloat,
        cpFloat,
        cpSpaceDebugColor,
        cpSpaceDebugColor,
        cpDataPointer,
    ) -> (),
>;
pub type cpSpaceDebugDrawSegmentImpl = Option::<
    unsafe extern "C" fn(cpVect, cpVect, cpSpaceDebugColor, cpDataPointer) -> (),
>;
pub type cpSpaceDebugDrawFatSegmentImpl = Option::<
    unsafe extern "C" fn(
        cpVect,
        cpVect,
        cpFloat,
        cpSpaceDebugColor,
        cpSpaceDebugColor,
        cpDataPointer,
    ) -> (),
>;
pub type cpSpaceDebugDrawPolygonImpl = Option::<
    unsafe extern "C" fn(
        libc::c_int,
        *const cpVect,
        cpFloat,
        cpSpaceDebugColor,
        cpSpaceDebugColor,
        cpDataPointer,
    ) -> (),
>;
pub type cpSpaceDebugDrawDotImpl = Option::<
    unsafe extern "C" fn(cpFloat, cpVect, cpSpaceDebugColor, cpDataPointer) -> (),
>;
pub type cpSpaceDebugDrawColorForShapeImpl = Option::<
    unsafe extern "C" fn(*mut cpShape, cpDataPointer) -> cpSpaceDebugColor,
>;
pub type cpSpaceDebugDrawFlags = libc::c_uint;
pub const CP_SPACE_DEBUG_DRAW_COLLISION_POINTS: cpSpaceDebugDrawFlags = 4;
pub const CP_SPACE_DEBUG_DRAW_CONSTRAINTS: cpSpaceDebugDrawFlags = 2;
pub const CP_SPACE_DEBUG_DRAW_SHAPES: cpSpaceDebugDrawFlags = 1;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct cpSpaceDebugDrawOptions {
    pub drawCircle: cpSpaceDebugDrawCircleImpl,
    pub drawSegment: cpSpaceDebugDrawSegmentImpl,
    pub drawFatSegment: cpSpaceDebugDrawFatSegmentImpl,
    pub drawPolygon: cpSpaceDebugDrawPolygonImpl,
    pub drawDot: cpSpaceDebugDrawDotImpl,
    pub flags: cpSpaceDebugDrawFlags,
    pub shapeOutlineColor: cpSpaceDebugColor,
    pub colorForShape: cpSpaceDebugDrawColorForShapeImpl,
    pub constraintColor: cpSpaceDebugColor,
    pub collisionPointColor: cpSpaceDebugColor,
    pub data: cpDataPointer,
}
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
unsafe extern "C" fn cpvlength(v: cpVect) -> cpFloat {
    return sqrt(cpvdot(v, v));
}
#[inline]
unsafe extern "C" fn cpTransformPoint(mut t: cpTransform, mut p: cpVect) -> cpVect {
    return cpv(t.a * p.x + t.c * p.y + t.tx, t.b * p.x + t.d * p.y + t.ty);
}
unsafe extern "C" fn cpSpaceDebugDrawShape(
    mut shape: *mut cpShape,
    mut options: *mut cpSpaceDebugDrawOptions,
) {
    let mut body: *mut cpBody = (*shape).body;
    let mut data: cpDataPointer = (*options).data;
    let mut outline_color: cpSpaceDebugColor = (*options).shapeOutlineColor;
    let mut fill_color: cpSpaceDebugColor = ((*options).colorForShape)
        .unwrap()(shape, data);
    match (*(*shape).klass).type_0 as libc::c_uint {
        0 => {
            let mut circle: *mut cpCircleShape = shape as *mut cpCircleShape;
            ((*options).drawCircle)
                .unwrap()(
                (*circle).tc,
                (*body).a,
                (*circle).r,
                outline_color,
                fill_color,
                data,
            );
        }
        1 => {
            let mut seg: *mut cpSegmentShape = shape as *mut cpSegmentShape;
            ((*options).drawFatSegment)
                .unwrap()(
                (*seg).ta,
                (*seg).tb,
                (*seg).r,
                outline_color,
                fill_color,
                data,
            );
        }
        2 => {
            let mut poly: *mut cpPolyShape = shape as *mut cpPolyShape;
            let mut count: libc::c_int = (*poly).count;
            let mut planes: *mut cpSplittingPlane = (*poly).planes;
            let mut fresh0 = ::std::vec::from_elem(
                0,
                (count as libc::c_ulong)
                    .wrapping_mul(::std::mem::size_of::<cpVect>() as libc::c_ulong)
                    as usize,
            );
            let mut verts: *mut cpVect = fresh0.leak().as_mut_ptr() as *mut cpVect;
            let mut i: libc::c_int = 0 as libc::c_int;
            while i < count {
                *verts.offset(i as isize) = (*planes.offset(i as isize)).v0;
                i += 1;
                i;
            }
            ((*options).drawPolygon)
                .unwrap()(count, verts, (*poly).r, outline_color, fill_color, data);
        }
        _ => {}
    };
}
static mut spring_verts: [cpVect; 15] = [
    {
        let mut init = cpVect {
            x: 0.00f32 as cpFloat,
            y: 0.0f32 as cpFloat,
        };
        init
    },
    {
        let mut init = cpVect {
            x: 0.20f32 as cpFloat,
            y: 0.0f32 as cpFloat,
        };
        init
    },
    {
        let mut init = cpVect {
            x: 0.25f32 as cpFloat,
            y: 3.0f32 as cpFloat,
        };
        init
    },
    {
        let mut init = cpVect {
            x: 0.30f32 as cpFloat,
            y: -6.0f32 as cpFloat,
        };
        init
    },
    {
        let mut init = cpVect {
            x: 0.35f32 as cpFloat,
            y: 6.0f32 as cpFloat,
        };
        init
    },
    {
        let mut init = cpVect {
            x: 0.40f32 as cpFloat,
            y: -6.0f32 as cpFloat,
        };
        init
    },
    {
        let mut init = cpVect {
            x: 0.45f32 as cpFloat,
            y: 6.0f32 as cpFloat,
        };
        init
    },
    {
        let mut init = cpVect {
            x: 0.50f32 as cpFloat,
            y: -6.0f32 as cpFloat,
        };
        init
    },
    {
        let mut init = cpVect {
            x: 0.55f32 as cpFloat,
            y: 6.0f32 as cpFloat,
        };
        init
    },
    {
        let mut init = cpVect {
            x: 0.60f32 as cpFloat,
            y: -6.0f32 as cpFloat,
        };
        init
    },
    {
        let mut init = cpVect {
            x: 0.65f32 as cpFloat,
            y: 6.0f32 as cpFloat,
        };
        init
    },
    {
        let mut init = cpVect {
            x: 0.70f32 as cpFloat,
            y: -3.0f32 as cpFloat,
        };
        init
    },
    {
        let mut init = cpVect {
            x: 0.75f32 as cpFloat,
            y: 6.0f32 as cpFloat,
        };
        init
    },
    {
        let mut init = cpVect {
            x: 0.80f32 as cpFloat,
            y: 0.0f32 as cpFloat,
        };
        init
    },
    {
        let mut init = cpVect {
            x: 1.00f32 as cpFloat,
            y: 0.0f32 as cpFloat,
        };
        init
    },
];
static mut spring_count: libc::c_int = 0;
unsafe extern "C" fn cpSpaceDebugDrawConstraint(
    mut constraint: *mut cpConstraint,
    mut options: *mut cpSpaceDebugDrawOptions,
) {
    let mut data: cpDataPointer = (*options).data;
    let mut color: cpSpaceDebugColor = (*options).constraintColor;
    let mut body_a: *mut cpBody = (*constraint).a;
    let mut body_b: *mut cpBody = (*constraint).b;
    if cpConstraintIsPinJoint(constraint) != 0 {
        let mut joint: *mut cpPinJoint = constraint as *mut cpPinJoint;
        let mut a: cpVect = cpTransformPoint((*body_a).transform, (*joint).anchorA);
        let mut b: cpVect = cpTransformPoint((*body_b).transform, (*joint).anchorB);
        ((*options).drawDot).unwrap()(5 as libc::c_int as cpFloat, a, color, data);
        ((*options).drawDot).unwrap()(5 as libc::c_int as cpFloat, b, color, data);
        ((*options).drawSegment).unwrap()(a, b, color, data);
    } else if cpConstraintIsSlideJoint(constraint) != 0 {
        let mut joint_0: *mut cpSlideJoint = constraint as *mut cpSlideJoint;
        let mut a_0: cpVect = cpTransformPoint((*body_a).transform, (*joint_0).anchorA);
        let mut b_0: cpVect = cpTransformPoint((*body_b).transform, (*joint_0).anchorB);
        ((*options).drawDot).unwrap()(5 as libc::c_int as cpFloat, a_0, color, data);
        ((*options).drawDot).unwrap()(5 as libc::c_int as cpFloat, b_0, color, data);
        ((*options).drawSegment).unwrap()(a_0, b_0, color, data);
    } else if cpConstraintIsPivotJoint(constraint) != 0 {
        let mut joint_1: *mut cpPivotJoint = constraint as *mut cpPivotJoint;
        let mut a_1: cpVect = cpTransformPoint((*body_a).transform, (*joint_1).anchorA);
        let mut b_1: cpVect = cpTransformPoint((*body_b).transform, (*joint_1).anchorB);
        ((*options).drawDot).unwrap()(5 as libc::c_int as cpFloat, a_1, color, data);
        ((*options).drawDot).unwrap()(5 as libc::c_int as cpFloat, b_1, color, data);
    } else if cpConstraintIsGrooveJoint(constraint) != 0 {
        let mut joint_2: *mut cpGrooveJoint = constraint as *mut cpGrooveJoint;
        let mut a_2: cpVect = cpTransformPoint((*body_a).transform, (*joint_2).grv_a);
        let mut b_2: cpVect = cpTransformPoint((*body_a).transform, (*joint_2).grv_b);
        let mut c: cpVect = cpTransformPoint((*body_b).transform, (*joint_2).anchorB);
        ((*options).drawDot).unwrap()(5 as libc::c_int as cpFloat, c, color, data);
        ((*options).drawSegment).unwrap()(a_2, b_2, color, data);
    } else if cpConstraintIsDampedSpring(constraint) != 0 {
        let mut spring: *mut cpDampedSpring = constraint as *mut cpDampedSpring;
        let mut a_3: cpVect = cpTransformPoint((*body_a).transform, (*spring).anchorA);
        let mut b_3: cpVect = cpTransformPoint((*body_b).transform, (*spring).anchorB);
        ((*options).drawDot).unwrap()(5 as libc::c_int as cpFloat, a_3, color, data);
        ((*options).drawDot).unwrap()(5 as libc::c_int as cpFloat, b_3, color, data);
        let mut delta: cpVect = cpvsub(b_3, a_3);
        let mut cos: cpFloat = delta.x;
        let mut sin: cpFloat = delta.y;
        let mut s: cpFloat = 1.0f32 as libc::c_double / cpvlength(delta);
        let mut r1: cpVect = cpv(cos, -sin * s);
        let mut r2: cpVect = cpv(sin, cos * s);
        let mut fresh1 = ::std::vec::from_elem(
            0,
            (spring_count as libc::c_ulong)
                .wrapping_mul(::std::mem::size_of::<cpVect>() as libc::c_ulong) as usize,
        );
        let mut verts: *mut cpVect = fresh1.leak().as_mut_ptr() as *mut cpVect;
        let mut i: libc::c_int = 0 as libc::c_int;
        while i < spring_count {
            let mut v: cpVect = spring_verts[i as usize];
            *verts
                .offset(i as isize) = cpv(cpvdot(v, r1) + a_3.x, cpvdot(v, r2) + a_3.y);
            i += 1;
            i;
        }
        let mut i_0: libc::c_int = 0 as libc::c_int;
        while i_0 < spring_count - 1 as libc::c_int {
            ((*options).drawSegment)
                .unwrap()(
                *verts.offset(i_0 as isize),
                *verts.offset((i_0 + 1 as libc::c_int) as isize),
                color,
                data,
            );
            i_0 += 1;
            i_0;
        }
    }
}
pub unsafe extern "C" fn cpSpaceDebugDraw(
    mut space: *mut cpSpace,
    mut options: *mut cpSpaceDebugDrawOptions,
) {
    if (*options).flags as libc::c_uint
        & CP_SPACE_DEBUG_DRAW_SHAPES as libc::c_int as libc::c_uint != 0
    {
        cpSpaceEachShape(
            space,
            ::std::mem::transmute::<
                Option::<
                    unsafe extern "C" fn(
                        *mut cpShape,
                        *mut cpSpaceDebugDrawOptions,
                    ) -> (),
                >,
                cpSpaceShapeIteratorFunc,
            >(
                Some(
                    cpSpaceDebugDrawShape
                        as unsafe extern "C" fn(
                            *mut cpShape,
                            *mut cpSpaceDebugDrawOptions,
                        ) -> (),
                ),
            ),
            options as *mut libc::c_void,
        );
    }
    if (*options).flags as libc::c_uint
        & CP_SPACE_DEBUG_DRAW_CONSTRAINTS as libc::c_int as libc::c_uint != 0
    {
        cpSpaceEachConstraint(
            space,
            ::std::mem::transmute::<
                Option::<
                    unsafe extern "C" fn(
                        *mut cpConstraint,
                        *mut cpSpaceDebugDrawOptions,
                    ) -> (),
                >,
                cpSpaceConstraintIteratorFunc,
            >(
                Some(
                    cpSpaceDebugDrawConstraint
                        as unsafe extern "C" fn(
                            *mut cpConstraint,
                            *mut cpSpaceDebugDrawOptions,
                        ) -> (),
                ),
            ),
            options as *mut libc::c_void,
        );
    }
    if (*options).flags as libc::c_uint
        & CP_SPACE_DEBUG_DRAW_COLLISION_POINTS as libc::c_int as libc::c_uint != 0
    {
        let mut arbiters: *mut cpArray = (*space).arbiters;
        let mut color: cpSpaceDebugColor = (*options).collisionPointColor;
        let mut draw_seg: cpSpaceDebugDrawSegmentImpl = (*options).drawSegment;
        let mut data: cpDataPointer = (*options).data;
        let mut i: libc::c_int = 0 as libc::c_int;
        while i < (*arbiters).num {
            let mut arb: *mut cpArbiter = *((*arbiters).arr).offset(i as isize)
                as *mut cpArbiter;
            let mut n: cpVect = (*arb).n;
            let mut j: libc::c_int = 0 as libc::c_int;
            while j < (*arb).count {
                let mut p1: cpVect = cpvadd(
                    (*(*arb).body_a).p,
                    (*((*arb).contacts).offset(j as isize)).r1,
                );
                let mut p2: cpVect = cpvadd(
                    (*(*arb).body_b).p,
                    (*((*arb).contacts).offset(j as isize)).r2,
                );
                let mut d: cpFloat = 2.0f32 as cpFloat;
                let mut a: cpVect = cpvadd(p1, cpvmult(n, -d));
                let mut b: cpVect = cpvadd(p2, cpvmult(n, d));
                draw_seg.unwrap()(a, b, color, data);
                j += 1;
                j;
            }
            i += 1;
            i;
        }
    }
}
unsafe extern "C" fn run_static_initializers() {
    spring_count = (::std::mem::size_of::<[cpVect; 15]>() as libc::c_ulong)
        .wrapping_div(::std::mem::size_of::<cpVect>() as libc::c_ulong) as libc::c_int;
}
#[used]
#[cfg_attr(target_os = "linux", link_section = ".init_array")]
#[cfg_attr(target_os = "windows", link_section = ".CRT$XIB")]
#[cfg_attr(target_os = "macos", link_section = "__DATA,__mod_init_func")]
static INIT_ARRAY: [unsafe extern "C" fn(); 1] = [run_static_initializers];
