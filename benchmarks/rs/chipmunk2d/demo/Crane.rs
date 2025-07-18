use ::libc;
extern "C" {
    pub type cpBody;
    pub type cpShape;
    pub type cpConstraint;
    pub type cpSpace;
    pub type cpArbiter;
    fn cpMomentForBox(m: cpFloat, width: cpFloat, height: cpFloat) -> cpFloat;
    fn cpArbiterGetBodies(
        arb: *const cpArbiter,
        a: *mut *mut cpBody,
        b: *mut *mut cpBody,
    );
    fn cpBodyNew(mass: cpFloat, moment: cpFloat) -> *mut cpBody;
    fn cpBodyGetPosition(body: *const cpBody) -> cpVect;
    fn cpBodySetPosition(body: *mut cpBody, pos: cpVect);
    fn cpShapeSetSensor(shape: *mut cpShape, sensor: cpBool);
    fn cpShapeSetElasticity(shape: *mut cpShape, elasticity: cpFloat);
    fn cpShapeSetFriction(shape: *mut cpShape, friction: cpFloat);
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
    fn cpBoxShapeNew(
        body: *mut cpBody,
        width: cpFloat,
        height: cpFloat,
        radius: cpFloat,
    ) -> *mut cpShape;
    fn cpConstraintFree(constraint: *mut cpConstraint);
    fn cpConstraintSetMaxForce(constraint: *mut cpConstraint, maxForce: cpFloat);
    fn cpConstraintSetMaxBias(constraint: *mut cpConstraint, maxBias: cpFloat);
    fn cpSlideJointNew(
        a: *mut cpBody,
        b: *mut cpBody,
        anchorA: cpVect,
        anchorB: cpVect,
        min: cpFloat,
        max: cpFloat,
    ) -> *mut cpConstraint;
    fn cpSlideJointSetMax(constraint: *mut cpConstraint, max: cpFloat);
    fn cpPivotJointNew(
        a: *mut cpBody,
        b: *mut cpBody,
        pivot: cpVect,
    ) -> *mut cpConstraint;
    fn cpPivotJointSetAnchorA(constraint: *mut cpConstraint, anchorA: cpVect);
    fn cpGrooveJointNew(
        a: *mut cpBody,
        b: *mut cpBody,
        groove_a: cpVect,
        groove_b: cpVect,
        anchorB: cpVect,
    ) -> *mut cpConstraint;
    fn cpSpaceNew() -> *mut cpSpace;
    fn cpSpaceFree(space: *mut cpSpace);
    fn cpSpaceSetIterations(space: *mut cpSpace, iterations: libc::c_int);
    fn cpSpaceSetGravity(space: *mut cpSpace, gravity: cpVect);
    fn cpSpaceSetDamping(space: *mut cpSpace, damping: cpFloat);
    fn cpSpaceGetStaticBody(space: *const cpSpace) -> *mut cpBody;
    fn cpSpaceAddCollisionHandler(
        space: *mut cpSpace,
        a: cpCollisionType,
        b: cpCollisionType,
    ) -> *mut cpCollisionHandler;
    fn cpSpaceAddShape(space: *mut cpSpace, shape: *mut cpShape) -> *mut cpShape;
    fn cpSpaceAddBody(space: *mut cpSpace, body: *mut cpBody) -> *mut cpBody;
    fn cpSpaceAddConstraint(
        space: *mut cpSpace,
        constraint: *mut cpConstraint,
    ) -> *mut cpConstraint;
    fn cpSpaceRemoveConstraint(space: *mut cpSpace, constraint: *mut cpConstraint);
    fn cpSpaceAddPostStepCallback(
        space: *mut cpSpace,
        func: cpPostStepFunc,
        key: *mut libc::c_void,
        data: *mut libc::c_void,
    ) -> cpBool;
    fn cpSpaceStep(space: *mut cpSpace, dt: cpFloat);
    static mut ChipmunkDemoMouse: cpVect;
    static mut ChipmunkDemoRightClick: cpBool;
    static mut ChipmunkDemoMessageString: *const libc::c_char;
    static mut NOT_GRABBABLE_FILTER: cpShapeFilter;
    fn ChipmunkDemoDefaultDrawImpl(space: *mut cpSpace);
    fn ChipmunkDemoFreeSpaceChildren(space: *mut cpSpace);
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
pub type cpPostStepFunc = Option::<
    unsafe extern "C" fn(*mut cpSpace, *mut libc::c_void, *mut libc::c_void) -> (),
>;
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
pub type COLLISION_TYPES = libc::c_uint;
pub const CRATE: COLLISION_TYPES = 2;
pub const HOOK_SENSOR: COLLISION_TYPES = 1;
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
unsafe extern "C" fn cpv(x: cpFloat, y: cpFloat) -> cpVect {
    let mut v: cpVect = {
        let mut init = cpVect { x: x, y: y };
        init
    };
    return v;
}
static mut dollyBody: *mut cpBody = 0 as *const cpBody as *mut cpBody;
static mut dollyServo: *mut cpConstraint = 0 as *const cpConstraint as *mut cpConstraint;
static mut winchServo: *mut cpConstraint = 0 as *const cpConstraint as *mut cpConstraint;
static mut hookJoint: *mut cpConstraint = 0 as *const cpConstraint as *mut cpConstraint;
unsafe extern "C" fn update(mut space: *mut cpSpace, mut dt: libc::c_double) {
    cpPivotJointSetAnchorA(
        dollyServo,
        cpv(ChipmunkDemoMouse.x, 100 as libc::c_int as cpFloat),
    );
    cpSlideJointSetMax(
        winchServo,
        cpfmax(
            100 as libc::c_int as libc::c_double - ChipmunkDemoMouse.y,
            50 as libc::c_int as cpFloat,
        ),
    );
    if !hookJoint.is_null() && ChipmunkDemoRightClick as libc::c_int != 0 {
        cpSpaceRemoveConstraint(space, hookJoint);
        cpConstraintFree(hookJoint);
        hookJoint = 0 as *mut cpConstraint;
    }
    cpSpaceStep(space, dt);
}
unsafe extern "C" fn AttachHook(
    mut space: *mut cpSpace,
    mut hook: *mut cpBody,
    mut crate_0: *mut cpBody,
) {
    hookJoint = cpSpaceAddConstraint(
        space,
        cpPivotJointNew(hook, crate_0, cpBodyGetPosition(hook)),
    );
}
unsafe extern "C" fn HookCrate(
    mut arb: *mut cpArbiter,
    mut space: *mut cpSpace,
    mut data: *mut libc::c_void,
) -> cpBool {
    if hookJoint.is_null() {
        let mut hook: *mut cpBody = 0 as *mut cpBody;
        let mut crate_0: *mut cpBody = 0 as *mut cpBody;
        cpArbiterGetBodies(arb, &mut hook, &mut crate_0);
        cpSpaceAddPostStepCallback(
            space,
            ::std::mem::transmute::<
                Option::<
                    unsafe extern "C" fn(*mut cpSpace, *mut cpBody, *mut cpBody) -> (),
                >,
                cpPostStepFunc,
            >(
                Some(
                    AttachHook
                        as unsafe extern "C" fn(
                            *mut cpSpace,
                            *mut cpBody,
                            *mut cpBody,
                        ) -> (),
                ),
            ),
            hook as *mut libc::c_void,
            crate_0 as *mut libc::c_void,
        );
    }
    return 1 as libc::c_int as cpBool;
}
unsafe extern "C" fn init() -> *mut cpSpace {
    ChipmunkDemoMessageString = b"Control the crane by moving the mouse. Right click to release.\0"
        as *const u8 as *const libc::c_char;
    let mut space: *mut cpSpace = cpSpaceNew();
    cpSpaceSetIterations(space, 30 as libc::c_int);
    cpSpaceSetGravity(
        space,
        cpv(0 as libc::c_int as cpFloat, -(100 as libc::c_int) as cpFloat),
    );
    cpSpaceSetDamping(space, 0.8f64);
    let mut staticBody: *mut cpBody = cpSpaceGetStaticBody(space);
    let mut shape: *mut cpShape = 0 as *mut cpShape;
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
    dollyBody = cpSpaceAddBody(
        space,
        cpBodyNew(10 as libc::c_int as cpFloat, ::std::f32::INFINITY as cpFloat),
    );
    cpBodySetPosition(
        dollyBody,
        cpv(0 as libc::c_int as cpFloat, 100 as libc::c_int as cpFloat),
    );
    cpSpaceAddShape(
        space,
        cpBoxShapeNew(
            dollyBody,
            30 as libc::c_int as cpFloat,
            30 as libc::c_int as cpFloat,
            0.0f64,
        ),
    );
    cpSpaceAddConstraint(
        space,
        cpGrooveJointNew(
            staticBody,
            dollyBody,
            cpv(-(250 as libc::c_int) as cpFloat, 100 as libc::c_int as cpFloat),
            cpv(250 as libc::c_int as cpFloat, 100 as libc::c_int as cpFloat),
            cpvzero,
        ),
    );
    dollyServo = cpSpaceAddConstraint(
        space,
        cpPivotJointNew(staticBody, dollyBody, cpBodyGetPosition(dollyBody)),
    );
    cpConstraintSetMaxForce(dollyServo, 10000 as libc::c_int as cpFloat);
    cpConstraintSetMaxBias(dollyServo, 100 as libc::c_int as cpFloat);
    let mut hookBody: *mut cpBody = cpSpaceAddBody(
        space,
        cpBodyNew(1 as libc::c_int as cpFloat, ::std::f32::INFINITY as cpFloat),
    );
    cpBodySetPosition(
        hookBody,
        cpv(0 as libc::c_int as cpFloat, 50 as libc::c_int as cpFloat),
    );
    shape = cpSpaceAddShape(
        space,
        cpCircleShapeNew(hookBody, 10 as libc::c_int as cpFloat, cpvzero),
    );
    cpShapeSetSensor(shape, 1 as libc::c_int as cpBool);
    cpShapeSetCollisionType(shape, HOOK_SENSOR as libc::c_int as cpCollisionType);
    winchServo = cpSpaceAddConstraint(
        space,
        cpSlideJointNew(
            dollyBody,
            hookBody,
            cpvzero,
            cpvzero,
            0 as libc::c_int as cpFloat,
            ::std::f32::INFINITY as cpFloat,
        ),
    );
    cpConstraintSetMaxForce(winchServo, 30000 as libc::c_int as cpFloat);
    cpConstraintSetMaxBias(winchServo, 60 as libc::c_int as cpFloat);
    let mut boxBody: *mut cpBody = cpSpaceAddBody(
        space,
        cpBodyNew(
            30 as libc::c_int as cpFloat,
            cpMomentForBox(
                30 as libc::c_int as cpFloat,
                50 as libc::c_int as cpFloat,
                50 as libc::c_int as cpFloat,
            ),
        ),
    );
    cpBodySetPosition(
        boxBody,
        cpv(200 as libc::c_int as cpFloat, -(200 as libc::c_int) as cpFloat),
    );
    shape = cpSpaceAddShape(
        space,
        cpBoxShapeNew(
            boxBody,
            50 as libc::c_int as cpFloat,
            50 as libc::c_int as cpFloat,
            0.0f64,
        ),
    );
    cpShapeSetFriction(shape, 0.7f64);
    cpShapeSetCollisionType(shape, CRATE as libc::c_int as cpCollisionType);
    let mut handler: *mut cpCollisionHandler = cpSpaceAddCollisionHandler(
        space,
        HOOK_SENSOR as libc::c_int as cpCollisionType,
        CRATE as libc::c_int as cpCollisionType,
    );
    (*handler)
        .beginFunc = ::std::mem::transmute::<
        Option::<
            unsafe extern "C" fn(
                *mut cpArbiter,
                *mut cpSpace,
                *mut libc::c_void,
            ) -> cpBool,
        >,
        cpCollisionBeginFunc,
    >(
        Some(
            HookCrate
                as unsafe extern "C" fn(
                    *mut cpArbiter,
                    *mut cpSpace,
                    *mut libc::c_void,
                ) -> cpBool,
        ),
    );
    return space;
}
unsafe extern "C" fn destroy(mut space: *mut cpSpace) {
    ChipmunkDemoFreeSpaceChildren(space);
    cpSpaceFree(space);
}
pub static mut Crane: ChipmunkDemo = unsafe {
    {
        let mut init = ChipmunkDemo {
            name: b"Crane\0" as *const u8 as *const libc::c_char,
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
