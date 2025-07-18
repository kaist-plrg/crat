use ::libc;
extern "C" {
    pub type cpBody;
    pub type cpShape;
    pub type cpConstraint;
    pub type cpSpace;
    fn cpMomentForBox(m: cpFloat, width: cpFloat, height: cpFloat) -> cpFloat;
    fn cpMomentForCircle(
        m: cpFloat,
        r1: cpFloat,
        r2: cpFloat,
        offset: cpVect,
    ) -> cpFloat;
    fn cpBodyNew(mass: cpFloat, moment: cpFloat) -> *mut cpBody;
    fn cpBodySetPosition(body: *mut cpBody, pos: cpVect);
    fn cpBodySetVelocity(body: *mut cpBody, velocity: cpVect);
    fn cpShapeSetElasticity(shape: *mut cpShape, elasticity: cpFloat);
    fn cpShapeSetFriction(shape: *mut cpShape, friction: cpFloat);
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
    fn cpConstraintFree(constraint: *mut cpConstraint);
    fn cpConstraintGetMaxForce(constraint: *const cpConstraint) -> cpFloat;
    fn cpConstraintSetMaxForce(constraint: *mut cpConstraint, maxForce: cpFloat);
    fn cpConstraintSetCollideBodies(
        constraint: *mut cpConstraint,
        collideBodies: cpBool,
    );
    fn cpConstraintSetPostSolveFunc(
        constraint: *mut cpConstraint,
        postSolveFunc: cpConstraintPostSolveFunc,
    );
    fn cpConstraintGetImpulse(constraint: *mut cpConstraint) -> cpFloat;
    fn cpSlideJointNew(
        a: *mut cpBody,
        b: *mut cpBody,
        anchorA: cpVect,
        anchorB: cpVect,
        min: cpFloat,
        max: cpFloat,
    ) -> *mut cpConstraint;
    fn cpSpaceNew() -> *mut cpSpace;
    fn cpSpaceFree(space: *mut cpSpace);
    fn cpSpaceSetIterations(space: *mut cpSpace, iterations: libc::c_int);
    fn cpSpaceSetGravity(space: *mut cpSpace, gravity: cpVect);
    fn cpSpaceSetSleepTimeThreshold(space: *mut cpSpace, sleepTimeThreshold: cpFloat);
    fn cpSpaceGetStaticBody(space: *const cpSpace) -> *mut cpBody;
    fn cpSpaceGetCurrentTimeStep(space: *const cpSpace) -> cpFloat;
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
    fn ChipmunkDemoFreeSpaceChildren(space: *mut cpSpace);
    fn ChipmunkDemoDefaultDrawImpl(space: *mut cpSpace);
    static mut NOT_GRABBABLE_FILTER: cpShapeFilter;
}
pub type uintptr_t = libc::c_ulong;
pub type cpFloat = libc::c_double;
pub type cpBool = libc::c_uchar;
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
pub struct cpShapeFilter {
    pub group: cpGroup,
    pub categories: cpBitmask,
    pub mask: cpBitmask,
}
pub type cpConstraintPostSolveFunc = Option::<
    unsafe extern "C" fn(*mut cpConstraint, *mut cpSpace) -> (),
>;
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
unsafe extern "C" fn BreakablejointPostStepRemove(
    mut space: *mut cpSpace,
    mut joint: *mut cpConstraint,
    mut unused: *mut libc::c_void,
) {
    cpSpaceRemoveConstraint(space, joint);
    cpConstraintFree(joint);
}
unsafe extern "C" fn BreakableJointPostSolve(
    mut joint: *mut cpConstraint,
    mut space: *mut cpSpace,
) {
    let mut dt: cpFloat = cpSpaceGetCurrentTimeStep(space);
    let mut force: cpFloat = cpConstraintGetImpulse(joint) / dt;
    let mut maxForce: cpFloat = cpConstraintGetMaxForce(joint);
    if force > 0.9f64 * maxForce {
        cpSpaceAddPostStepCallback(
            space,
            ::std::mem::transmute::<
                Option::<
                    unsafe extern "C" fn(
                        *mut cpSpace,
                        *mut cpConstraint,
                        *mut libc::c_void,
                    ) -> (),
                >,
                cpPostStepFunc,
            >(
                Some(
                    BreakablejointPostStepRemove
                        as unsafe extern "C" fn(
                            *mut cpSpace,
                            *mut cpConstraint,
                            *mut libc::c_void,
                        ) -> (),
                ),
            ),
            joint as *mut libc::c_void,
            0 as *mut libc::c_void,
        );
    }
}
unsafe extern "C" fn update(mut space: *mut cpSpace, mut dt: libc::c_double) {
    cpSpaceStep(space, dt);
}
unsafe extern "C" fn init() -> *mut cpSpace {
    let mut space: *mut cpSpace = cpSpaceNew();
    cpSpaceSetIterations(space, 30 as libc::c_int);
    cpSpaceSetGravity(
        space,
        cpv(0 as libc::c_int as cpFloat, -(100 as libc::c_int) as cpFloat),
    );
    cpSpaceSetSleepTimeThreshold(space, 0.5f32 as cpFloat);
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
            cpv(-(320 as libc::c_int) as cpFloat, 240 as libc::c_int as cpFloat),
            cpv(320 as libc::c_int as cpFloat, 240 as libc::c_int as cpFloat),
            0.0f32 as cpFloat,
        ),
    );
    cpShapeSetElasticity(shape, 1.0f32 as cpFloat);
    cpShapeSetFriction(shape, 1.0f32 as cpFloat);
    cpShapeSetFilter(shape, NOT_GRABBABLE_FILTER);
    let mut mass: cpFloat = 1 as libc::c_int as cpFloat;
    let mut width: cpFloat = 20 as libc::c_int as cpFloat;
    let mut height: cpFloat = 30 as libc::c_int as cpFloat;
    let mut spacing: cpFloat = width * 0.3f64;
    let mut i: libc::c_int = 0 as libc::c_int;
    while i < 8 as libc::c_int {
        let mut prev: *mut cpBody = 0 as *mut cpBody;
        let mut j: libc::c_int = 0 as libc::c_int;
        while j < 10 as libc::c_int {
            let mut pos: cpVect = cpv(
                40 as libc::c_int as libc::c_double
                    * (i as libc::c_double
                        - (8 as libc::c_int - 1 as libc::c_int) as libc::c_double
                            / 2.0f64),
                240 as libc::c_int as libc::c_double
                    - (j as libc::c_double + 0.5f64) * height
                    - (j + 1 as libc::c_int) as libc::c_double * spacing,
            );
            body = cpSpaceAddBody(
                space,
                cpBodyNew(mass, cpMomentForBox(mass, width, height)),
            );
            cpBodySetPosition(body, pos);
            shape = cpSpaceAddShape(
                space,
                cpSegmentShapeNew(
                    body,
                    cpv(0 as libc::c_int as cpFloat, (height - width) / 2.0f64),
                    cpv(0 as libc::c_int as cpFloat, (width - height) / 2.0f64),
                    width / 2.0f64,
                ),
            );
            cpShapeSetFriction(shape, 0.8f32 as cpFloat);
            let mut breakingForce: cpFloat = 80000 as libc::c_int as cpFloat;
            let mut constraint: *mut cpConstraint = 0 as *mut cpConstraint;
            if prev.is_null() {
                constraint = cpSpaceAddConstraint(
                    space,
                    cpSlideJointNew(
                        body,
                        staticBody,
                        cpv(
                            0 as libc::c_int as cpFloat,
                            height / 2 as libc::c_int as libc::c_double,
                        ),
                        cpv(pos.x, 240 as libc::c_int as cpFloat),
                        0 as libc::c_int as cpFloat,
                        spacing,
                    ),
                );
            } else {
                constraint = cpSpaceAddConstraint(
                    space,
                    cpSlideJointNew(
                        body,
                        prev,
                        cpv(
                            0 as libc::c_int as cpFloat,
                            height / 2 as libc::c_int as libc::c_double,
                        ),
                        cpv(
                            0 as libc::c_int as cpFloat,
                            -height / 2 as libc::c_int as libc::c_double,
                        ),
                        0 as libc::c_int as cpFloat,
                        spacing,
                    ),
                );
            }
            cpConstraintSetMaxForce(constraint, breakingForce);
            cpConstraintSetPostSolveFunc(
                constraint,
                Some(
                    BreakableJointPostSolve
                        as unsafe extern "C" fn(*mut cpConstraint, *mut cpSpace) -> (),
                ),
            );
            cpConstraintSetCollideBodies(constraint, 0 as libc::c_int as cpBool);
            prev = body;
            j += 1;
            j;
        }
        i += 1;
        i;
    }
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
        cpv(
            0 as libc::c_int as cpFloat,
            -(240 as libc::c_int) as libc::c_double + radius
                + 5 as libc::c_int as libc::c_double,
        ),
    );
    cpBodySetVelocity(
        body,
        cpv(0 as libc::c_int as cpFloat, 300 as libc::c_int as cpFloat),
    );
    shape = cpSpaceAddShape(space, cpCircleShapeNew(body, radius, cpvzero));
    cpShapeSetElasticity(shape, 0.0f32 as cpFloat);
    cpShapeSetFriction(shape, 0.9f32 as cpFloat);
    return space;
}
unsafe extern "C" fn destroy(mut space: *mut cpSpace) {
    ChipmunkDemoFreeSpaceChildren(space);
    cpSpaceFree(space);
}
pub static mut Chains: ChipmunkDemo = unsafe {
    {
        let mut init = ChipmunkDemo {
            name: b"Breakable Chains\0" as *const u8 as *const libc::c_char,
            timestep: 1.0f64 / 180.0f64,
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
