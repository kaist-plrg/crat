use ::libc;
extern "C" {
    pub type cpBody;
    pub type cpShape;
    pub type cpSpace;
    pub type cpArbiter;
    fn cpMomentForCircle(
        m: cpFloat,
        r1: cpFloat,
        r2: cpFloat,
        offset: cpVect,
    ) -> cpFloat;
    fn cpArbiterIgnore(arb: *mut cpArbiter) -> cpBool;
    fn cpArbiterGetShapes(
        arb: *const cpArbiter,
        a: *mut *mut cpShape,
        b: *mut *mut cpShape,
    );
    fn cpArbiterGetNormal(arb: *const cpArbiter) -> cpVect;
    fn cpBodyNew(mass: cpFloat, moment: cpFloat) -> *mut cpBody;
    fn cpBodySetPosition(body: *mut cpBody, pos: cpVect);
    fn cpBodySetVelocity(body: *mut cpBody, velocity: cpVect);
    fn cpShapeSetElasticity(shape: *mut cpShape, elasticity: cpFloat);
    fn cpShapeSetFriction(shape: *mut cpShape, friction: cpFloat);
    fn cpShapeGetUserData(shape: *const cpShape) -> cpDataPointer;
    fn cpShapeSetUserData(shape: *mut cpShape, userData: cpDataPointer);
    fn cpShapeSetCollisionType(shape: *mut cpShape, collisionType: cpCollisionType);
    fn cpShapeSetFilter(shape: *mut cpShape, filter: cpShapeFilter);
    fn cpCircleShapeNew(
        body: *mut cpBody,
        radius: cpFloat,
        offset: cpVect,
    ) -> *mut cpShape;
    fn cpSegmentShapeNew(
        body: *mut cpBody,
        a: cpVect,
        b: cpVect,
        radius: cpFloat,
    ) -> *mut cpShape;
    fn cpSpaceNew() -> *mut cpSpace;
    fn cpSpaceFree(space: *mut cpSpace);
    fn cpSpaceSetIterations(space: *mut cpSpace, iterations: libc::c_int);
    fn cpSpaceSetGravity(space: *mut cpSpace, gravity: cpVect);
    fn cpSpaceGetStaticBody(space: *const cpSpace) -> *mut cpBody;
    fn cpSpaceAddWildcardHandler(
        space: *mut cpSpace,
        type_0: cpCollisionType,
    ) -> *mut cpCollisionHandler;
    fn cpSpaceAddShape(space: *mut cpSpace, shape: *mut cpShape) -> *mut cpShape;
    fn cpSpaceAddBody(space: *mut cpSpace, body: *mut cpBody) -> *mut cpBody;
    fn cpSpaceStep(space: *mut cpSpace, dt: cpFloat);
    static mut ChipmunkDemoMessageString: *const libc::c_char;
    fn ChipmunkDemoFreeSpaceChildren(space: *mut cpSpace);
    fn ChipmunkDemoDefaultDrawImpl(space: *mut cpSpace);
    static mut NOT_GRABBABLE_FILTER: cpShapeFilter;
}
pub type uintptr_t = libc::c_ulong;
pub type cpFloat = libc::c_double;
pub type cpBool = libc::c_uchar;
pub type cpDataPointer = *mut libc::c_void;
pub type cpCollisionType = uintptr_t;
pub type cpGroup = uintptr_t;
pub type cpBitmask = libc::c_uint;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct cpVect {
    pub x: cpFloat,
    pub y: cpFloat,
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
pub struct cpShapeFilter {
    pub group: cpGroup,
    pub categories: cpBitmask,
    pub mask: cpBitmask,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ChipmunkDemo {
    pub name: *const libc::c_char,
    pub timestep: libc::c_double,
    pub initFunc: ChipmunkDemoInitFunc,
    pub updateFunc: ChipmunkDemoUpdateFunc,
    pub drawFunc: ChipmunkDemoDrawFunc,
    pub destroyFunc: ChipmunkDemoDestroyFunc,
}
pub type ChipmunkDemoDestroyFunc = Option::<unsafe extern "C" fn(*mut cpSpace) -> ()>;
pub type ChipmunkDemoDrawFunc = Option::<unsafe extern "C" fn(*mut cpSpace) -> ()>;
pub type ChipmunkDemoUpdateFunc = Option::<
    unsafe extern "C" fn(*mut cpSpace, libc::c_double) -> (),
>;
pub type ChipmunkDemoInitFunc = Option::<unsafe extern "C" fn() -> *mut cpSpace>;
pub type CollisionTypes = libc::c_uint;
pub const COLLISION_TYPE_ONE_WAY: CollisionTypes = 1;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct OneWayPlatform {
    pub n: cpVect,
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
unsafe extern "C" fn cpvdot(v1: cpVect, v2: cpVect) -> cpFloat {
    return v1.x * v2.x + v1.y * v2.y;
}
static mut platformInstance: OneWayPlatform = OneWayPlatform {
    n: cpVect { x: 0., y: 0. },
};
unsafe extern "C" fn PreSolve(
    mut arb: *mut cpArbiter,
    mut space: *mut cpSpace,
    mut ignore: *mut libc::c_void,
) -> cpBool {
    let mut a: *mut cpShape = 0 as *mut cpShape;
    let mut b: *mut cpShape = 0 as *mut cpShape;
    cpArbiterGetShapes(arb, &mut a, &mut b);
    let mut platform: *mut OneWayPlatform = cpShapeGetUserData(a) as *mut OneWayPlatform;
    if cpvdot(cpArbiterGetNormal(arb), (*platform).n)
        < 0 as libc::c_int as libc::c_double
    {
        return cpArbiterIgnore(arb);
    }
    return 1 as libc::c_int as cpBool;
}
unsafe extern "C" fn update(mut space: *mut cpSpace, mut dt: libc::c_double) {
    cpSpaceStep(space, dt);
}
unsafe extern "C" fn init() -> *mut cpSpace {
    ChipmunkDemoMessageString = b"One way platforms are trivial in Chipmunk using a very simple collision callback.\0"
        as *const u8 as *const libc::c_char;
    let mut space: *mut cpSpace = cpSpaceNew();
    cpSpaceSetIterations(space, 10 as libc::c_int);
    cpSpaceSetGravity(
        space,
        cpv(0 as libc::c_int as cpFloat, -(100 as libc::c_int) as cpFloat),
    );
    let mut body: *mut cpBody = 0 as *mut cpBody;
    let mut staticBody: *mut cpBody = cpSpaceGetStaticBody(space);
    let mut shape: *mut cpShape = 0 as *mut cpShape;
    shape = cpSpaceAddShape(
        space,
        cpSegmentShapeNew(
            staticBody,
            cpv(-(320 as libc::c_int) as cpFloat, -(240 as libc::c_int) as cpFloat),
            cpv(-(320 as libc::c_int) as cpFloat, 240 as libc::c_int as cpFloat),
            0.0f32 as cpFloat,
        ),
    );
    cpShapeSetElasticity(shape, 1.0f32 as cpFloat);
    cpShapeSetFriction(shape, 1.0f32 as cpFloat);
    cpShapeSetFilter(shape, NOT_GRABBABLE_FILTER);
    shape = cpSpaceAddShape(
        space,
        cpSegmentShapeNew(
            staticBody,
            cpv(320 as libc::c_int as cpFloat, -(240 as libc::c_int) as cpFloat),
            cpv(320 as libc::c_int as cpFloat, 240 as libc::c_int as cpFloat),
            0.0f32 as cpFloat,
        ),
    );
    cpShapeSetElasticity(shape, 1.0f32 as cpFloat);
    cpShapeSetFriction(shape, 1.0f32 as cpFloat);
    cpShapeSetFilter(shape, NOT_GRABBABLE_FILTER);
    shape = cpSpaceAddShape(
        space,
        cpSegmentShapeNew(
            staticBody,
            cpv(-(320 as libc::c_int) as cpFloat, -(240 as libc::c_int) as cpFloat),
            cpv(320 as libc::c_int as cpFloat, -(240 as libc::c_int) as cpFloat),
            0.0f32 as cpFloat,
        ),
    );
    cpShapeSetElasticity(shape, 1.0f32 as cpFloat);
    cpShapeSetFriction(shape, 1.0f32 as cpFloat);
    cpShapeSetFilter(shape, NOT_GRABBABLE_FILTER);
    shape = cpSpaceAddShape(
        space,
        cpSegmentShapeNew(
            staticBody,
            cpv(-(160 as libc::c_int) as cpFloat, -(100 as libc::c_int) as cpFloat),
            cpv(160 as libc::c_int as cpFloat, -(100 as libc::c_int) as cpFloat),
            10.0f32 as cpFloat,
        ),
    );
    cpShapeSetElasticity(shape, 1.0f32 as cpFloat);
    cpShapeSetFriction(shape, 1.0f32 as cpFloat);
    cpShapeSetCollisionType(
        shape,
        COLLISION_TYPE_ONE_WAY as libc::c_int as cpCollisionType,
    );
    cpShapeSetFilter(shape, NOT_GRABBABLE_FILTER);
    platformInstance.n = cpv(0 as libc::c_int as cpFloat, 1 as libc::c_int as cpFloat);
    cpShapeSetUserData(
        shape,
        &mut platformInstance as *mut OneWayPlatform as cpDataPointer,
    );
    let mut radius: cpFloat = 15.0f32 as cpFloat;
    body = cpSpaceAddBody(
        space,
        cpBodyNew(
            10.0f32 as cpFloat,
            cpMomentForCircle(10.0f32 as cpFloat, 0.0f32 as cpFloat, radius, cpvzero),
        ),
    );
    cpBodySetPosition(
        body,
        cpv(0 as libc::c_int as cpFloat, -(200 as libc::c_int) as cpFloat),
    );
    cpBodySetVelocity(
        body,
        cpv(0 as libc::c_int as cpFloat, 170 as libc::c_int as cpFloat),
    );
    shape = cpSpaceAddShape(space, cpCircleShapeNew(body, radius, cpvzero));
    cpShapeSetElasticity(shape, 0.0f32 as cpFloat);
    cpShapeSetFriction(shape, 0.9f32 as cpFloat);
    cpShapeSetCollisionType(shape, 2 as libc::c_int as cpCollisionType);
    let mut handler: *mut cpCollisionHandler = cpSpaceAddWildcardHandler(
        space,
        COLLISION_TYPE_ONE_WAY as libc::c_int as cpCollisionType,
    );
    (*handler)
        .preSolveFunc = Some(
        PreSolve
            as unsafe extern "C" fn(
                *mut cpArbiter,
                *mut cpSpace,
                *mut libc::c_void,
            ) -> cpBool,
    );
    return space;
}
unsafe extern "C" fn destroy(mut space: *mut cpSpace) {
    ChipmunkDemoFreeSpaceChildren(space);
    cpSpaceFree(space);
}
pub static mut OneWay: ChipmunkDemo = unsafe {
    {
        let mut init = ChipmunkDemo {
            name: b"One Way Platforms\0" as *const u8 as *const libc::c_char,
            timestep: 1.0f64 / 60.0f64,
            initFunc: Some(init as unsafe extern "C" fn() -> *mut cpSpace),
            updateFunc: Some(
                update as unsafe extern "C" fn(*mut cpSpace, libc::c_double) -> (),
            ),
            drawFunc: Some(
                ChipmunkDemoDefaultDrawImpl as unsafe extern "C" fn(*mut cpSpace) -> (),
            ),
            destroyFunc: Some(destroy as unsafe extern "C" fn(*mut cpSpace) -> ()),
        };
        init
    }
};
