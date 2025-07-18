use ::libc;
extern "C" {
    pub type cpHashSet;
    pub type cpContactBufferHeader;
    fn calloc(_: libc::c_ulong, _: libc::c_ulong) -> *mut libc::c_void;
    fn pthread_create(
        __newthread: *mut pthread_t,
        __attr: *const pthread_attr_t,
        __start_routine: Option::<
            unsafe extern "C" fn(*mut libc::c_void) -> *mut libc::c_void,
        >,
        __arg: *mut libc::c_void,
    ) -> libc::c_int;
    fn pthread_join(
        __th: pthread_t,
        __thread_return: *mut *mut libc::c_void,
    ) -> libc::c_int;
    fn pthread_mutex_init(
        __mutex: *mut pthread_mutex_t,
        __mutexattr: *const pthread_mutexattr_t,
    ) -> libc::c_int;
    fn pthread_mutex_destroy(__mutex: *mut pthread_mutex_t) -> libc::c_int;
    fn pthread_mutex_lock(__mutex: *mut pthread_mutex_t) -> libc::c_int;
    fn pthread_mutex_unlock(__mutex: *mut pthread_mutex_t) -> libc::c_int;
    fn pthread_cond_init(
        __cond: *mut pthread_cond_t,
        __cond_attr: *const pthread_condattr_t,
    ) -> libc::c_int;
    fn pthread_cond_destroy(__cond: *mut pthread_cond_t) -> libc::c_int;
    fn pthread_cond_signal(__cond: *mut pthread_cond_t) -> libc::c_int;
    fn pthread_cond_broadcast(__cond: *mut pthread_cond_t) -> libc::c_int;
    fn pthread_cond_wait(
        __cond: *mut pthread_cond_t,
        __mutex: *mut pthread_mutex_t,
    ) -> libc::c_int;
    fn pow(_: libc::c_double, _: libc::c_double) -> libc::c_double;
    fn cpBodyIsSleeping(body: *const cpBody) -> cpBool;
    fn cpSpaceInit(space: *mut cpSpace) -> *mut cpSpace;
    fn cpSpaceFree(space: *mut cpSpace);
    fn cpHashSetFilter(
        set: *mut cpHashSet,
        func: cpHashSetFilterFunc,
        data: *mut libc::c_void,
    );
    fn cpArbiterUnthread(arb: *mut cpArbiter);
    fn cpArbiterPreStep(arb: *mut cpArbiter, dt: cpFloat, bias: cpFloat, slop: cpFloat);
    fn cpArbiterApplyCachedImpulse(arb: *mut cpArbiter, dt_coef: cpFloat);
    fn cpArbiterApplyImpulse(arb: *mut cpArbiter);
    fn cpSpaceProcessComponents(space: *mut cpSpace, dt: cpFloat);
    fn cpSpacePushFreshContactBuffer(space: *mut cpSpace);
    fn cpSpaceArbiterSetFilter(arb: *mut cpArbiter, space: *mut cpSpace) -> cpBool;
    fn cpSpaceLock(space: *mut cpSpace);
    fn cpSpaceUnlock(space: *mut cpSpace, runPostStep: cpBool);
    fn cpShapeUpdateFunc(shape: *mut cpShape, unused: *mut libc::c_void);
    fn cpSpaceCollideShapes(
        a: *mut cpShape,
        b: *mut cpShape,
        id: cpCollisionID,
        space: *mut cpSpace,
    ) -> cpCollisionID;
}
pub type __uint32_t = libc::c_uint;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct __pthread_internal_list {
    pub __prev: *mut __pthread_internal_list,
    pub __next: *mut __pthread_internal_list,
}
pub type __pthread_list_t = __pthread_internal_list;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct __pthread_mutex_s {
    pub __lock: libc::c_int,
    pub __count: libc::c_uint,
    pub __owner: libc::c_int,
    pub __nusers: libc::c_uint,
    pub __kind: libc::c_int,
    pub __spins: libc::c_short,
    pub __elision: libc::c_short,
    pub __list: __pthread_list_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct __pthread_cond_s {
    pub c2rust_unnamed: C2RustUnnamed_1,
    pub c2rust_unnamed_0: C2RustUnnamed,
    pub __g_refs: [libc::c_uint; 2],
    pub __g_size: [libc::c_uint; 2],
    pub __g1_orig_size: libc::c_uint,
    pub __wrefs: libc::c_uint,
    pub __g_signals: [libc::c_uint; 2],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed {
    pub __g1_start: libc::c_ulonglong,
    pub __g1_start32: C2RustUnnamed_0,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_0 {
    pub __low: libc::c_uint,
    pub __high: libc::c_uint,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_1 {
    pub __wseq: libc::c_ulonglong,
    pub __wseq32: C2RustUnnamed_2,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_2 {
    pub __low: libc::c_uint,
    pub __high: libc::c_uint,
}
pub type pthread_t = libc::c_ulong;
#[derive(Copy, Clone)]
#[repr(C)]
pub union pthread_mutexattr_t {
    pub __size: [libc::c_char; 4],
    pub __align: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union pthread_condattr_t {
    pub __size: [libc::c_char; 4],
    pub __align: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union pthread_attr_t {
    pub __size: [libc::c_char; 56],
    pub __align: libc::c_long,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union pthread_mutex_t {
    pub __data: __pthread_mutex_s,
    pub __size: [libc::c_char; 40],
    pub __align: libc::c_long,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union pthread_cond_t {
    pub __data: __pthread_cond_s,
    pub __size: [libc::c_char; 48],
    pub __align: libc::c_longlong,
}
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
    pub sleeping: C2RustUnnamed_3,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_3 {
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
pub type cpHashSetFilterFunc = Option::<
    unsafe extern "C" fn(*mut libc::c_void, *mut libc::c_void) -> cpBool,
>;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct cpHastySpace {
    pub space: cpSpace,
    pub num_threads: libc::c_ulong,
    pub num_working: libc::c_ulong,
    pub constraint_count_threshold: libc::c_ulong,
    pub mutex: pthread_mutex_t,
    pub cond_work: pthread_cond_t,
    pub cond_resume: pthread_cond_t,
    pub work: cpHastySpaceWorkFunction,
    pub workers: [ThreadContext; 1],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ThreadContext {
    pub thread: pthread_t,
    pub space: *mut cpHastySpace,
    pub thread_num: libc::c_ulong,
}
pub type cpHastySpaceWorkFunction = Option::<
    unsafe extern "C" fn(*mut cpSpace, libc::c_ulong, libc::c_ulong) -> (),
>;
#[inline]
unsafe extern "C" fn cpSpatialIndexEach(
    mut index: *mut cpSpatialIndex,
    mut func: cpSpatialIndexIteratorFunc,
    mut data: *mut libc::c_void,
) {
    ((*(*index).klass).each).unwrap()(index, func, data);
}
#[inline]
unsafe extern "C" fn cpSpatialIndexReindexQuery(
    mut index: *mut cpSpatialIndex,
    mut func: cpSpatialIndexQueryFunc,
    mut data: *mut libc::c_void,
) {
    ((*(*index).klass).reindexQuery).unwrap()(index, func, data);
}
unsafe extern "C" fn WorkerThreadLoop(
    mut context: *mut ThreadContext,
) -> *mut libc::c_void {
    let mut hasty: *mut cpHastySpace = (*context).space;
    let mut thread: libc::c_ulong = (*context).thread_num;
    let mut num_threads: libc::c_ulong = (*hasty).num_threads;
    loop {
        pthread_mutex_lock(&mut (*hasty).mutex);
        (*hasty).num_working = ((*hasty).num_working).wrapping_sub(1);
        if (*hasty).num_working == 0 as libc::c_int as libc::c_ulong {
            pthread_cond_signal(&mut (*hasty).cond_resume);
        }
        pthread_cond_wait(&mut (*hasty).cond_work, &mut (*hasty).mutex);
        pthread_mutex_unlock(&mut (*hasty).mutex);
        let mut func: cpHastySpaceWorkFunction = (*hasty).work;
        if !func.is_some() {
            break;
        }
        ((*hasty).work).unwrap()(&mut (*hasty).space, thread, num_threads);
    }
    return 0 as *mut libc::c_void;
}
unsafe extern "C" fn RunWorkers(
    mut hasty: *mut cpHastySpace,
    mut func: cpHastySpaceWorkFunction,
) {
    (*hasty)
        .num_working = ((*hasty).num_threads)
        .wrapping_sub(1 as libc::c_int as libc::c_ulong);
    (*hasty).work = func;
    if (*hasty).num_working > 0 as libc::c_int as libc::c_ulong {
        pthread_mutex_lock(&mut (*hasty).mutex);
        pthread_cond_broadcast(&mut (*hasty).cond_work);
        pthread_mutex_unlock(&mut (*hasty).mutex);
        func
            .unwrap()(
            hasty as *mut cpSpace,
            0 as libc::c_int as libc::c_ulong,
            (*hasty).num_threads,
        );
        pthread_mutex_lock(&mut (*hasty).mutex);
        if (*hasty).num_working > 0 as libc::c_int as libc::c_ulong {
            pthread_cond_wait(&mut (*hasty).cond_resume, &mut (*hasty).mutex);
        }
        pthread_mutex_unlock(&mut (*hasty).mutex);
    } else {
        func
            .unwrap()(
            hasty as *mut cpSpace,
            0 as libc::c_int as libc::c_ulong,
            (*hasty).num_threads,
        );
    }
    (*hasty).work = None;
}
unsafe extern "C" fn Solver(
    mut space: *mut cpSpace,
    mut worker: libc::c_ulong,
    mut worker_count: libc::c_ulong,
) {
    let mut constraints: *mut cpArray = (*space).constraints;
    let mut arbiters: *mut cpArray = (*space).arbiters;
    let mut dt: cpFloat = (*space).curr_dt;
    let mut iterations: libc::c_ulong = ((*space).iterations as libc::c_ulong)
        .wrapping_add(worker_count)
        .wrapping_sub(1 as libc::c_int as libc::c_ulong)
        .wrapping_div(worker_count);
    let mut i: libc::c_ulong = 0 as libc::c_int as libc::c_ulong;
    while i < iterations {
        let mut j: libc::c_int = 0 as libc::c_int;
        while j < (*arbiters).num {
            let mut arb: *mut cpArbiter = *((*arbiters).arr).offset(j as isize)
                as *mut cpArbiter;
            cpArbiterApplyImpulse(arb);
            j += 1;
            j;
        }
        let mut j_0: libc::c_int = 0 as libc::c_int;
        while j_0 < (*constraints).num {
            let mut constraint: *mut cpConstraint = *((*constraints).arr)
                .offset(j_0 as isize) as *mut cpConstraint;
            ((*(*constraint).klass).applyImpulse).unwrap()(constraint, dt);
            j_0 += 1;
            j_0;
        }
        i = i.wrapping_add(1);
        i;
    }
}
unsafe extern "C" fn HaltThreads(mut hasty: *mut cpHastySpace) {
    let mut mutex: *mut pthread_mutex_t = &mut (*hasty).mutex;
    pthread_mutex_lock(mutex);
    (*hasty).work = None;
    pthread_cond_broadcast(&mut (*hasty).cond_work);
    pthread_mutex_unlock(mutex);
    let mut i: libc::c_ulong = 0 as libc::c_int as libc::c_ulong;
    while i < ((*hasty).num_threads).wrapping_sub(1 as libc::c_int as libc::c_ulong) {
        pthread_join(
            (*((*hasty).workers).as_mut_ptr().offset(i as isize)).thread,
            0 as *mut *mut libc::c_void,
        );
        i = i.wrapping_add(1);
        i;
    }
}
pub unsafe extern "C" fn cpHastySpaceSetThreads(
    mut space: *mut cpSpace,
    mut threads: libc::c_ulong,
) {
    let mut hasty: *mut cpHastySpace = space as *mut cpHastySpace;
    HaltThreads(hasty);
    if threads == 0 as libc::c_int as libc::c_ulong {
        threads = 1 as libc::c_int as libc::c_ulong;
    }
    (*hasty)
        .num_threads = if threads < 2 as libc::c_int as libc::c_ulong {
        threads
    } else {
        2 as libc::c_int as libc::c_ulong
    };
    (*hasty)
        .num_working = ((*hasty).num_threads)
        .wrapping_sub(1 as libc::c_int as libc::c_ulong);
    if (*hasty).num_working > 0 as libc::c_int as libc::c_ulong {
        pthread_mutex_lock(&mut (*hasty).mutex);
        let mut i: libc::c_ulong = 0 as libc::c_int as libc::c_ulong;
        while i < ((*hasty).num_threads).wrapping_sub(1 as libc::c_int as libc::c_ulong)
        {
            let ref mut fresh0 = (*((*hasty).workers).as_mut_ptr().offset(i as isize))
                .space;
            *fresh0 = hasty;
            (*((*hasty).workers).as_mut_ptr().offset(i as isize))
                .thread_num = i.wrapping_add(1 as libc::c_int as libc::c_ulong);
            pthread_create(
                &mut (*((*hasty).workers).as_mut_ptr().offset(i as isize)).thread,
                0 as *const pthread_attr_t,
                ::std::mem::transmute::<
                    Option::<
                        unsafe extern "C" fn(*mut ThreadContext) -> *mut libc::c_void,
                    >,
                    Option::<
                        unsafe extern "C" fn(*mut libc::c_void) -> *mut libc::c_void,
                    >,
                >(
                    Some(
                        WorkerThreadLoop
                            as unsafe extern "C" fn(
                                *mut ThreadContext,
                            ) -> *mut libc::c_void,
                    ),
                ),
                &mut *((*hasty).workers).as_mut_ptr().offset(i as isize)
                    as *mut ThreadContext as *mut libc::c_void,
            );
            i = i.wrapping_add(1);
            i;
        }
        pthread_cond_wait(&mut (*hasty).cond_resume, &mut (*hasty).mutex);
        pthread_mutex_unlock(&mut (*hasty).mutex);
    }
}
pub unsafe extern "C" fn cpHastySpaceGetThreads(
    mut space: *mut cpSpace,
) -> libc::c_ulong {
    return (*(space as *mut cpHastySpace)).num_threads;
}
pub unsafe extern "C" fn cpHastySpaceNew() -> *mut cpSpace {
    let mut hasty: *mut cpHastySpace = calloc(
        1 as libc::c_int as libc::c_ulong,
        ::std::mem::size_of::<cpHastySpace>() as libc::c_ulong,
    ) as *mut cpHastySpace;
    cpSpaceInit(hasty as *mut cpSpace);
    pthread_mutex_init(&mut (*hasty).mutex, 0 as *const pthread_mutexattr_t);
    pthread_cond_init(&mut (*hasty).cond_work, 0 as *const pthread_condattr_t);
    pthread_cond_init(&mut (*hasty).cond_resume, 0 as *const pthread_condattr_t);
    (*hasty).constraint_count_threshold = 50 as libc::c_int as libc::c_ulong;
    (*hasty).num_threads = 1 as libc::c_int as libc::c_ulong;
    cpHastySpaceSetThreads(hasty as *mut cpSpace, 1 as libc::c_int as libc::c_ulong);
    return hasty as *mut cpSpace;
}
pub unsafe extern "C" fn cpHastySpaceFree(mut space: *mut cpSpace) {
    let mut hasty: *mut cpHastySpace = space as *mut cpHastySpace;
    HaltThreads(hasty);
    pthread_mutex_destroy(&mut (*hasty).mutex);
    pthread_cond_destroy(&mut (*hasty).cond_work);
    pthread_cond_destroy(&mut (*hasty).cond_resume);
    cpSpaceFree(space);
}
pub unsafe extern "C" fn cpHastySpaceStep(mut space: *mut cpSpace, mut dt: cpFloat) {
    if dt == 0.0f32 as libc::c_double {
        return;
    }
    (*space).stamp = ((*space).stamp).wrapping_add(1);
    (*space).stamp;
    let mut prev_dt: cpFloat = (*space).curr_dt;
    (*space).curr_dt = dt;
    let mut bodies: *mut cpArray = (*space).dynamicBodies;
    let mut constraints: *mut cpArray = (*space).constraints;
    let mut arbiters: *mut cpArray = (*space).arbiters;
    let mut i: libc::c_int = 0 as libc::c_int;
    while i < (*arbiters).num {
        let mut arb: *mut cpArbiter = *((*arbiters).arr).offset(i as isize)
            as *mut cpArbiter;
        (*arb).state = CP_ARBITER_STATE_NORMAL;
        if cpBodyIsSleeping((*arb).body_a) == 0 && cpBodyIsSleeping((*arb).body_b) == 0 {
            cpArbiterUnthread(arb);
        }
        i += 1;
        i;
    }
    (*arbiters).num = 0 as libc::c_int;
    cpSpaceLock(space);
    let mut i_0: libc::c_int = 0 as libc::c_int;
    while i_0 < (*bodies).num {
        let mut body: *mut cpBody = *((*bodies).arr).offset(i_0 as isize) as *mut cpBody;
        ((*body).position_func).unwrap()(body, dt);
        i_0 += 1;
        i_0;
    }
    cpSpacePushFreshContactBuffer(space);
    cpSpatialIndexEach(
        (*space).dynamicShapes,
        ::std::mem::transmute::<
            Option::<unsafe extern "C" fn(*mut cpShape, *mut libc::c_void) -> ()>,
            cpSpatialIndexIteratorFunc,
        >(
            Some(
                cpShapeUpdateFunc
                    as unsafe extern "C" fn(*mut cpShape, *mut libc::c_void) -> (),
            ),
        ),
        0 as *mut libc::c_void,
    );
    cpSpatialIndexReindexQuery(
        (*space).dynamicShapes,
        ::std::mem::transmute::<
            Option::<
                unsafe extern "C" fn(
                    *mut cpShape,
                    *mut cpShape,
                    cpCollisionID,
                    *mut cpSpace,
                ) -> cpCollisionID,
            >,
            cpSpatialIndexQueryFunc,
        >(
            Some(
                cpSpaceCollideShapes
                    as unsafe extern "C" fn(
                        *mut cpShape,
                        *mut cpShape,
                        cpCollisionID,
                        *mut cpSpace,
                    ) -> cpCollisionID,
            ),
        ),
        space as *mut libc::c_void,
    );
    cpSpaceUnlock(space, 0 as libc::c_int as cpBool);
    cpSpaceProcessComponents(space, dt);
    cpSpaceLock(space);
    cpHashSetFilter(
        (*space).cachedArbiters,
        ::std::mem::transmute::<
            Option::<unsafe extern "C" fn(*mut cpArbiter, *mut cpSpace) -> cpBool>,
            cpHashSetFilterFunc,
        >(
            Some(
                cpSpaceArbiterSetFilter
                    as unsafe extern "C" fn(*mut cpArbiter, *mut cpSpace) -> cpBool,
            ),
        ),
        space as *mut libc::c_void,
    );
    let mut slop: cpFloat = (*space).collisionSlop;
    let mut biasCoef: cpFloat = 1.0f32 as libc::c_double
        - pow((*space).collisionBias, dt);
    let mut i_1: libc::c_int = 0 as libc::c_int;
    while i_1 < (*arbiters).num {
        cpArbiterPreStep(
            *((*arbiters).arr).offset(i_1 as isize) as *mut cpArbiter,
            dt,
            slop,
            biasCoef,
        );
        i_1 += 1;
        i_1;
    }
    let mut i_2: libc::c_int = 0 as libc::c_int;
    while i_2 < (*constraints).num {
        let mut constraint: *mut cpConstraint = *((*constraints).arr)
            .offset(i_2 as isize) as *mut cpConstraint;
        let mut preSolve: cpConstraintPreSolveFunc = (*constraint).preSolve;
        if preSolve.is_some() {
            preSolve.unwrap()(constraint, space);
        }
        ((*(*constraint).klass).preStep).unwrap()(constraint, dt);
        i_2 += 1;
        i_2;
    }
    let mut damping: cpFloat = pow((*space).damping, dt);
    let mut gravity: cpVect = (*space).gravity;
    let mut i_3: libc::c_int = 0 as libc::c_int;
    while i_3 < (*bodies).num {
        let mut body_0: *mut cpBody = *((*bodies).arr).offset(i_3 as isize)
            as *mut cpBody;
        ((*body_0).velocity_func).unwrap()(body_0, gravity, damping, dt);
        i_3 += 1;
        i_3;
    }
    let mut dt_coef: cpFloat = if prev_dt == 0.0f32 as libc::c_double {
        0.0f32 as libc::c_double
    } else {
        dt / prev_dt
    };
    let mut i_4: libc::c_int = 0 as libc::c_int;
    while i_4 < (*arbiters).num {
        cpArbiterApplyCachedImpulse(
            *((*arbiters).arr).offset(i_4 as isize) as *mut cpArbiter,
            dt_coef,
        );
        i_4 += 1;
        i_4;
    }
    let mut i_5: libc::c_int = 0 as libc::c_int;
    while i_5 < (*constraints).num {
        let mut constraint_0: *mut cpConstraint = *((*constraints).arr)
            .offset(i_5 as isize) as *mut cpConstraint;
        ((*(*constraint_0).klass).applyCachedImpulse).unwrap()(constraint_0, dt_coef);
        i_5 += 1;
        i_5;
    }
    let mut hasty: *mut cpHastySpace = space as *mut cpHastySpace;
    if ((*arbiters).num + (*constraints).num) as libc::c_ulong
        > (*hasty).constraint_count_threshold
    {
        RunWorkers(
            hasty,
            Some(
                Solver
                    as unsafe extern "C" fn(
                        *mut cpSpace,
                        libc::c_ulong,
                        libc::c_ulong,
                    ) -> (),
            ),
        );
    } else {
        Solver(
            space,
            0 as libc::c_int as libc::c_ulong,
            1 as libc::c_int as libc::c_ulong,
        );
    }
    let mut i_6: libc::c_int = 0 as libc::c_int;
    while i_6 < (*constraints).num {
        let mut constraint_1: *mut cpConstraint = *((*constraints).arr)
            .offset(i_6 as isize) as *mut cpConstraint;
        let mut postSolve: cpConstraintPostSolveFunc = (*constraint_1).postSolve;
        if postSolve.is_some() {
            postSolve.unwrap()(constraint_1, space);
        }
        i_6 += 1;
        i_6;
    }
    let mut i_7: libc::c_int = 0 as libc::c_int;
    while i_7 < (*arbiters).num {
        let mut arb_0: *mut cpArbiter = *((*arbiters).arr).offset(i_7 as isize)
            as *mut cpArbiter;
        let mut handler: *mut cpCollisionHandler = (*arb_0).handler;
        ((*handler).postSolveFunc).unwrap()(arb_0, space, (*handler).userData);
        i_7 += 1;
        i_7;
    }
    cpSpaceUnlock(space, 1 as libc::c_int as cpBool);
}
