use ::libc;
extern "C" {
    pub type cpBody;
    pub type cpShape;
    pub type cpConstraint;
    pub type cpSpace;
    fn cpMomentForBox(m: cpFloat, width: cpFloat, height: cpFloat) -> cpFloat;
    fn cpMomentForSegment(m: cpFloat, a: cpVect, b: cpVect, radius: cpFloat) -> cpFloat;
    fn cpMomentForCircle(
        m: cpFloat,
        r1: cpFloat,
        r2: cpFloat,
        offset: cpVect,
    ) -> cpFloat;
    fn cpBodyNew(mass: cpFloat, moment: cpFloat) -> *mut cpBody;
    fn cpBodySetPosition(body: *mut cpBody, pos: cpVect);
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
    fn cpBoxShapeNew(
        body: *mut cpBody,
        width: cpFloat,
        height: cpFloat,
        radius: cpFloat,
    ) -> *mut cpShape;
    fn cpPinJointNew(
        a: *mut cpBody,
        b: *mut cpBody,
        anchorA: cpVect,
        anchorB: cpVect,
    ) -> *mut cpConstraint;
    fn cpSlideJointNew(
        a: *mut cpBody,
        b: *mut cpBody,
        anchorA: cpVect,
        anchorB: cpVect,
        min: cpFloat,
        max: cpFloat,
    ) -> *mut cpConstraint;
    fn cpPivotJointNew(
        a: *mut cpBody,
        b: *mut cpBody,
        pivot: cpVect,
    ) -> *mut cpConstraint;
    fn cpGrooveJointNew(
        a: *mut cpBody,
        b: *mut cpBody,
        groove_a: cpVect,
        groove_b: cpVect,
        anchorB: cpVect,
    ) -> *mut cpConstraint;
    fn cpDampedSpringNew(
        a: *mut cpBody,
        b: *mut cpBody,
        anchorA: cpVect,
        anchorB: cpVect,
        restLength: cpFloat,
        stiffness: cpFloat,
        damping: cpFloat,
    ) -> *mut cpConstraint;
    fn cpDampedRotarySpringNew(
        a: *mut cpBody,
        b: *mut cpBody,
        restAngle: cpFloat,
        stiffness: cpFloat,
        damping: cpFloat,
    ) -> *mut cpConstraint;
    fn cpRotaryLimitJointNew(
        a: *mut cpBody,
        b: *mut cpBody,
        min: cpFloat,
        max: cpFloat,
    ) -> *mut cpConstraint;
    fn cpRatchetJointNew(
        a: *mut cpBody,
        b: *mut cpBody,
        phase: cpFloat,
        ratchet: cpFloat,
    ) -> *mut cpConstraint;
    fn cpGearJointNew(
        a: *mut cpBody,
        b: *mut cpBody,
        phase: cpFloat,
        ratio: cpFloat,
    ) -> *mut cpConstraint;
    fn cpSimpleMotorNew(
        a: *mut cpBody,
        b: *mut cpBody,
        rate: cpFloat,
    ) -> *mut cpConstraint;
    fn cpSpaceNew() -> *mut cpSpace;
    fn cpSpaceFree(space: *mut cpSpace);
    fn cpSpaceSetIterations(space: *mut cpSpace, iterations: libc::c_int);
    fn cpSpaceSetGravity(space: *mut cpSpace, gravity: cpVect);
    fn cpSpaceSetSleepTimeThreshold(space: *mut cpSpace, sleepTimeThreshold: cpFloat);
    fn cpSpaceGetStaticBody(space: *const cpSpace) -> *mut cpBody;
    fn cpSpaceAddShape(space: *mut cpSpace, shape: *mut cpShape) -> *mut cpShape;
    fn cpSpaceAddBody(space: *mut cpSpace, body: *mut cpBody) -> *mut cpBody;
    fn cpSpaceAddConstraint(
        space: *mut cpSpace,
        constraint: *mut cpConstraint,
    ) -> *mut cpConstraint;
    fn cpSpaceStep(space: *mut cpSpace, dt: cpFloat);
    static mut NOT_GRABBABLE_FILTER: cpShapeFilter;
    fn ChipmunkDemoDefaultDrawImpl(space: *mut cpSpace);
    fn ChipmunkDemoFreeSpaceChildren(space: *mut cpSpace);
}
pub type uintptr_t = libc::c_ulong;
pub type cpFloat = libc::c_double;
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
#[inline]
unsafe extern "C" fn cpvadd(v1: cpVect, v2: cpVect) -> cpVect {
    return cpv(v1.x + v2.x, v1.y + v2.y);
}
#[inline]
unsafe extern "C" fn cpShapeFilterNew(
    mut group: cpGroup,
    mut categories: cpBitmask,
    mut mask: cpBitmask,
) -> cpShapeFilter {
    let mut filter: cpShapeFilter = {
        let mut init = cpShapeFilter {
            group: group,
            categories: categories,
            mask: mask,
        };
        init
    };
    return filter;
}
unsafe extern "C" fn addBall(
    mut space: *mut cpSpace,
    mut pos: cpVect,
    mut boxOffset: cpVect,
) -> *mut cpBody {
    let mut radius: cpFloat = 15.0f32 as cpFloat;
    let mut mass: cpFloat = 1.0f32 as cpFloat;
    let mut body: *mut cpBody = cpSpaceAddBody(
        space,
        cpBodyNew(mass, cpMomentForCircle(mass, 0.0f32 as cpFloat, radius, cpvzero)),
    );
    cpBodySetPosition(body, cpvadd(pos, boxOffset));
    let mut shape: *mut cpShape = cpSpaceAddShape(
        space,
        cpCircleShapeNew(body, radius, cpvzero),
    );
    cpShapeSetElasticity(shape, 0.0f32 as cpFloat);
    cpShapeSetFriction(shape, 0.7f32 as cpFloat);
    return body;
}
unsafe extern "C" fn addLever(
    mut space: *mut cpSpace,
    mut pos: cpVect,
    mut boxOffset: cpVect,
) -> *mut cpBody {
    let mut mass: cpFloat = 1.0f32 as cpFloat;
    let mut a: cpVect = cpv(0 as libc::c_int as cpFloat, 15 as libc::c_int as cpFloat);
    let mut b: cpVect = cpv(
        0 as libc::c_int as cpFloat,
        -(15 as libc::c_int) as cpFloat,
    );
    let mut body: *mut cpBody = cpSpaceAddBody(
        space,
        cpBodyNew(mass, cpMomentForSegment(mass, a, b, 0.0f32 as cpFloat)),
    );
    cpBodySetPosition(
        body,
        cpvadd(
            pos,
            cpvadd(
                boxOffset,
                cpv(0 as libc::c_int as cpFloat, -(15 as libc::c_int) as cpFloat),
            ),
        ),
    );
    let mut shape: *mut cpShape = cpSpaceAddShape(
        space,
        cpSegmentShapeNew(body, a, b, 5.0f32 as cpFloat),
    );
    cpShapeSetElasticity(shape, 0.0f32 as cpFloat);
    cpShapeSetFriction(shape, 0.7f32 as cpFloat);
    return body;
}
unsafe extern "C" fn addBar(
    mut space: *mut cpSpace,
    mut pos: cpVect,
    mut boxOffset: cpVect,
) -> *mut cpBody {
    let mut mass: cpFloat = 2.0f32 as cpFloat;
    let mut a: cpVect = cpv(0 as libc::c_int as cpFloat, 30 as libc::c_int as cpFloat);
    let mut b: cpVect = cpv(
        0 as libc::c_int as cpFloat,
        -(30 as libc::c_int) as cpFloat,
    );
    let mut body: *mut cpBody = cpSpaceAddBody(
        space,
        cpBodyNew(mass, cpMomentForSegment(mass, a, b, 0.0f32 as cpFloat)),
    );
    cpBodySetPosition(body, cpvadd(pos, boxOffset));
    let mut shape: *mut cpShape = cpSpaceAddShape(
        space,
        cpSegmentShapeNew(body, a, b, 5.0f32 as cpFloat),
    );
    cpShapeSetElasticity(shape, 0.0f32 as cpFloat);
    cpShapeSetFriction(shape, 0.7f32 as cpFloat);
    cpShapeSetFilter(
        shape,
        cpShapeFilterNew(
            1 as libc::c_int as cpGroup,
            !(0 as libc::c_int as cpBitmask),
            !(0 as libc::c_int as cpBitmask),
        ),
    );
    return body;
}
unsafe extern "C" fn addWheel(
    mut space: *mut cpSpace,
    mut pos: cpVect,
    mut boxOffset: cpVect,
) -> *mut cpBody {
    let mut radius: cpFloat = 15.0f32 as cpFloat;
    let mut mass: cpFloat = 1.0f32 as cpFloat;
    let mut body: *mut cpBody = cpSpaceAddBody(
        space,
        cpBodyNew(mass, cpMomentForCircle(mass, 0.0f32 as cpFloat, radius, cpvzero)),
    );
    cpBodySetPosition(body, cpvadd(pos, boxOffset));
    let mut shape: *mut cpShape = cpSpaceAddShape(
        space,
        cpCircleShapeNew(body, radius, cpvzero),
    );
    cpShapeSetElasticity(shape, 0.0f32 as cpFloat);
    cpShapeSetFriction(shape, 0.7f32 as cpFloat);
    cpShapeSetFilter(
        shape,
        cpShapeFilterNew(
            1 as libc::c_int as cpGroup,
            !(0 as libc::c_int as cpBitmask),
            !(0 as libc::c_int as cpBitmask),
        ),
    );
    return body;
}
unsafe extern "C" fn addChassis(
    mut space: *mut cpSpace,
    mut pos: cpVect,
    mut boxOffset: cpVect,
) -> *mut cpBody {
    let mut mass: cpFloat = 5.0f32 as cpFloat;
    let mut width: cpFloat = 80 as libc::c_int as cpFloat;
    let mut height: cpFloat = 30 as libc::c_int as cpFloat;
    let mut body: *mut cpBody = cpSpaceAddBody(
        space,
        cpBodyNew(mass, cpMomentForBox(mass, width, height)),
    );
    cpBodySetPosition(body, cpvadd(pos, boxOffset));
    let mut shape: *mut cpShape = cpSpaceAddShape(
        space,
        cpBoxShapeNew(body, width, height, 0.0f64),
    );
    cpShapeSetElasticity(shape, 0.0f32 as cpFloat);
    cpShapeSetFriction(shape, 0.7f32 as cpFloat);
    cpShapeSetFilter(
        shape,
        cpShapeFilterNew(
            1 as libc::c_int as cpGroup,
            !(0 as libc::c_int as cpBitmask),
            !(0 as libc::c_int as cpBitmask),
        ),
    );
    return body;
}
unsafe extern "C" fn init() -> *mut cpSpace {
    let mut space: *mut cpSpace = cpSpaceNew();
    cpSpaceSetIterations(space, 10 as libc::c_int);
    cpSpaceSetGravity(
        space,
        cpv(0 as libc::c_int as cpFloat, -(100 as libc::c_int) as cpFloat),
    );
    cpSpaceSetSleepTimeThreshold(space, 0.5f32 as cpFloat);
    let mut staticBody: *mut cpBody = cpSpaceGetStaticBody(space);
    let mut shape: *mut cpShape = 0 as *mut cpShape;
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
    shape = cpSpaceAddShape(
        space,
        cpSegmentShapeNew(
            staticBody,
            cpv(-(320 as libc::c_int) as cpFloat, 120 as libc::c_int as cpFloat),
            cpv(320 as libc::c_int as cpFloat, 120 as libc::c_int as cpFloat),
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
            cpv(-(320 as libc::c_int) as cpFloat, 0 as libc::c_int as cpFloat),
            cpv(320 as libc::c_int as cpFloat, 0 as libc::c_int as cpFloat),
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
            cpv(-(320 as libc::c_int) as cpFloat, -(120 as libc::c_int) as cpFloat),
            cpv(320 as libc::c_int as cpFloat, -(120 as libc::c_int) as cpFloat),
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
            cpv(-(160 as libc::c_int) as cpFloat, -(240 as libc::c_int) as cpFloat),
            cpv(-(160 as libc::c_int) as cpFloat, 240 as libc::c_int as cpFloat),
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
            cpv(0 as libc::c_int as cpFloat, -(240 as libc::c_int) as cpFloat),
            cpv(0 as libc::c_int as cpFloat, 240 as libc::c_int as cpFloat),
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
            cpv(160 as libc::c_int as cpFloat, -(240 as libc::c_int) as cpFloat),
            cpv(160 as libc::c_int as cpFloat, 240 as libc::c_int as cpFloat),
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
    let mut boxOffset: cpVect = cpVect { x: 0., y: 0. };
    let mut body1: *mut cpBody = 0 as *mut cpBody;
    let mut body2: *mut cpBody = 0 as *mut cpBody;
    let mut posA: cpVect = cpv(
        50 as libc::c_int as cpFloat,
        60 as libc::c_int as cpFloat,
    );
    let mut posB: cpVect = cpv(
        110 as libc::c_int as cpFloat,
        60 as libc::c_int as cpFloat,
    );
    boxOffset = cpv(-(320 as libc::c_int) as cpFloat, -(240 as libc::c_int) as cpFloat);
    body1 = addBall(space, posA, boxOffset);
    body2 = addBall(space, posB, boxOffset);
    cpSpaceAddConstraint(
        space,
        cpPinJointNew(
            body1,
            body2,
            cpv(15 as libc::c_int as cpFloat, 0 as libc::c_int as cpFloat),
            cpv(-(15 as libc::c_int) as cpFloat, 0 as libc::c_int as cpFloat),
        ),
    );
    boxOffset = cpv(-(160 as libc::c_int) as cpFloat, -(240 as libc::c_int) as cpFloat);
    body1 = addBall(space, posA, boxOffset);
    body2 = addBall(space, posB, boxOffset);
    cpSpaceAddConstraint(
        space,
        cpSlideJointNew(
            body1,
            body2,
            cpv(15 as libc::c_int as cpFloat, 0 as libc::c_int as cpFloat),
            cpv(-(15 as libc::c_int) as cpFloat, 0 as libc::c_int as cpFloat),
            20.0f32 as cpFloat,
            40.0f32 as cpFloat,
        ),
    );
    boxOffset = cpv(0 as libc::c_int as cpFloat, -(240 as libc::c_int) as cpFloat);
    body1 = addBall(space, posA, boxOffset);
    body2 = addBall(space, posB, boxOffset);
    cpSpaceAddConstraint(
        space,
        cpPivotJointNew(
            body1,
            body2,
            cpvadd(
                boxOffset,
                cpv(80 as libc::c_int as cpFloat, 60 as libc::c_int as cpFloat),
            ),
        ),
    );
    boxOffset = cpv(160 as libc::c_int as cpFloat, -(240 as libc::c_int) as cpFloat);
    body1 = addBall(space, posA, boxOffset);
    body2 = addBall(space, posB, boxOffset);
    cpSpaceAddConstraint(
        space,
        cpGrooveJointNew(
            body1,
            body2,
            cpv(30 as libc::c_int as cpFloat, 30 as libc::c_int as cpFloat),
            cpv(30 as libc::c_int as cpFloat, -(30 as libc::c_int) as cpFloat),
            cpv(-(30 as libc::c_int) as cpFloat, 0 as libc::c_int as cpFloat),
        ),
    );
    boxOffset = cpv(-(320 as libc::c_int) as cpFloat, -(120 as libc::c_int) as cpFloat);
    body1 = addBall(space, posA, boxOffset);
    body2 = addBall(space, posB, boxOffset);
    cpSpaceAddConstraint(
        space,
        cpDampedSpringNew(
            body1,
            body2,
            cpv(15 as libc::c_int as cpFloat, 0 as libc::c_int as cpFloat),
            cpv(-(15 as libc::c_int) as cpFloat, 0 as libc::c_int as cpFloat),
            20.0f32 as cpFloat,
            5.0f32 as cpFloat,
            0.3f32 as cpFloat,
        ),
    );
    boxOffset = cpv(-(160 as libc::c_int) as cpFloat, -(120 as libc::c_int) as cpFloat);
    body1 = addBar(space, posA, boxOffset);
    body2 = addBar(space, posB, boxOffset);
    cpSpaceAddConstraint(
        space,
        cpPivotJointNew(body1, staticBody, cpvadd(boxOffset, posA)),
    );
    cpSpaceAddConstraint(
        space,
        cpPivotJointNew(body2, staticBody, cpvadd(boxOffset, posB)),
    );
    cpSpaceAddConstraint(
        space,
        cpDampedRotarySpringNew(
            body1,
            body2,
            0.0f32 as cpFloat,
            3000.0f32 as cpFloat,
            60.0f32 as cpFloat,
        ),
    );
    boxOffset = cpv(0 as libc::c_int as cpFloat, -(120 as libc::c_int) as cpFloat);
    body1 = addLever(space, posA, boxOffset);
    body2 = addLever(space, posB, boxOffset);
    cpSpaceAddConstraint(
        space,
        cpPivotJointNew(body1, staticBody, cpvadd(boxOffset, posA)),
    );
    cpSpaceAddConstraint(
        space,
        cpPivotJointNew(body2, staticBody, cpvadd(boxOffset, posB)),
    );
    cpSpaceAddConstraint(
        space,
        cpRotaryLimitJointNew(
            body1,
            body2,
            -3.14159265358979323846264338327950288f64 / 2.0f32 as libc::c_double,
            3.14159265358979323846264338327950288f64 / 2.0f32 as libc::c_double,
        ),
    );
    boxOffset = cpv(160 as libc::c_int as cpFloat, -(120 as libc::c_int) as cpFloat);
    body1 = addLever(space, posA, boxOffset);
    body2 = addLever(space, posB, boxOffset);
    cpSpaceAddConstraint(
        space,
        cpPivotJointNew(body1, staticBody, cpvadd(boxOffset, posA)),
    );
    cpSpaceAddConstraint(
        space,
        cpPivotJointNew(body2, staticBody, cpvadd(boxOffset, posB)),
    );
    cpSpaceAddConstraint(
        space,
        cpRatchetJointNew(
            body1,
            body2,
            0.0f32 as cpFloat,
            3.14159265358979323846264338327950288f64 / 2.0f32 as libc::c_double,
        ),
    );
    boxOffset = cpv(-(320 as libc::c_int) as cpFloat, 0 as libc::c_int as cpFloat);
    body1 = addBar(space, posA, boxOffset);
    body2 = addBar(space, posB, boxOffset);
    cpSpaceAddConstraint(
        space,
        cpPivotJointNew(body1, staticBody, cpvadd(boxOffset, posA)),
    );
    cpSpaceAddConstraint(
        space,
        cpPivotJointNew(body2, staticBody, cpvadd(boxOffset, posB)),
    );
    cpSpaceAddConstraint(
        space,
        cpGearJointNew(body1, body2, 0.0f32 as cpFloat, 2.0f32 as cpFloat),
    );
    boxOffset = cpv(-(160 as libc::c_int) as cpFloat, 0 as libc::c_int as cpFloat);
    body1 = addBar(space, posA, boxOffset);
    body2 = addBar(space, posB, boxOffset);
    cpSpaceAddConstraint(
        space,
        cpPivotJointNew(body1, staticBody, cpvadd(boxOffset, posA)),
    );
    cpSpaceAddConstraint(
        space,
        cpPivotJointNew(body2, staticBody, cpvadd(boxOffset, posB)),
    );
    cpSpaceAddConstraint(
        space,
        cpSimpleMotorNew(body1, body2, 3.14159265358979323846264338327950288f64),
    );
    boxOffset = cpv(0 as libc::c_int as cpFloat, 0 as libc::c_int as cpFloat);
    let mut wheel1: *mut cpBody = addWheel(space, posA, boxOffset);
    let mut wheel2: *mut cpBody = addWheel(space, posB, boxOffset);
    let mut chassis: *mut cpBody = addChassis(
        space,
        cpv(80 as libc::c_int as cpFloat, 100 as libc::c_int as cpFloat),
        boxOffset,
    );
    cpSpaceAddConstraint(
        space,
        cpGrooveJointNew(
            chassis,
            wheel1,
            cpv(-(30 as libc::c_int) as cpFloat, -(10 as libc::c_int) as cpFloat),
            cpv(-(30 as libc::c_int) as cpFloat, -(40 as libc::c_int) as cpFloat),
            cpvzero,
        ),
    );
    cpSpaceAddConstraint(
        space,
        cpGrooveJointNew(
            chassis,
            wheel2,
            cpv(30 as libc::c_int as cpFloat, -(10 as libc::c_int) as cpFloat),
            cpv(30 as libc::c_int as cpFloat, -(40 as libc::c_int) as cpFloat),
            cpvzero,
        ),
    );
    cpSpaceAddConstraint(
        space,
        cpDampedSpringNew(
            chassis,
            wheel1,
            cpv(-(30 as libc::c_int) as cpFloat, 0 as libc::c_int as cpFloat),
            cpvzero,
            50.0f32 as cpFloat,
            20.0f32 as cpFloat,
            10.0f32 as cpFloat,
        ),
    );
    cpSpaceAddConstraint(
        space,
        cpDampedSpringNew(
            chassis,
            wheel2,
            cpv(30 as libc::c_int as cpFloat, 0 as libc::c_int as cpFloat),
            cpvzero,
            50.0f32 as cpFloat,
            20.0f32 as cpFloat,
            10.0f32 as cpFloat,
        ),
    );
    return space;
}
unsafe extern "C" fn update(mut space: *mut cpSpace, mut dt: libc::c_double) {
    cpSpaceStep(space, dt);
}
unsafe extern "C" fn destroy(mut space: *mut cpSpace) {
    ChipmunkDemoFreeSpaceChildren(space);
    cpSpaceFree(space);
}
pub static mut Joints: ChipmunkDemo = unsafe {
    {
        let mut init = ChipmunkDemo {
            name: b"Joints and Constraints\0" as *const u8 as *const libc::c_char,
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
