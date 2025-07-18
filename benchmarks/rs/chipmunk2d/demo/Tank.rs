use ::libc;
extern "C" {
    pub type cpBody;
    pub type cpShape;
    pub type cpConstraint;
    pub type cpSpace;
    fn atan2(_: libc::c_double, _: libc::c_double) -> libc::c_double;
    fn sqrt(_: libc::c_double) -> libc::c_double;
    fn rand() -> libc::c_int;
    fn cpBodyNew(mass: cpFloat, moment: cpFloat) -> *mut cpBody;
    fn cpBodyNewKinematic() -> *mut cpBody;
    fn cpBodyGetPosition(body: *const cpBody) -> cpVect;
    fn cpBodySetPosition(body: *mut cpBody, pos: cpVect);
    fn cpBodySetVelocity(body: *mut cpBody, velocity: cpVect);
    fn cpBodyGetAngle(body: *const cpBody) -> cpFloat;
    fn cpBodySetAngle(body: *mut cpBody, a: cpFloat);
    fn cpBodyGetRotation(body: *const cpBody) -> cpVect;
    fn cpShapeSetElasticity(shape: *mut cpShape, elasticity: cpFloat);
    fn cpShapeSetFriction(shape: *mut cpShape, friction: cpFloat);
    fn cpShapeSetFilter(shape: *mut cpShape, filter: cpShapeFilter);
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
    fn cpConstraintSetMaxForce(constraint: *mut cpConstraint, maxForce: cpFloat);
    fn cpConstraintSetErrorBias(constraint: *mut cpConstraint, errorBias: cpFloat);
    fn cpConstraintSetMaxBias(constraint: *mut cpConstraint, maxBias: cpFloat);
    fn cpPivotJointNew2(
        a: *mut cpBody,
        b: *mut cpBody,
        anchorA: cpVect,
        anchorB: cpVect,
    ) -> *mut cpConstraint;
    fn cpGearJointNew(
        a: *mut cpBody,
        b: *mut cpBody,
        phase: cpFloat,
        ratio: cpFloat,
    ) -> *mut cpConstraint;
    fn cpSpaceNew() -> *mut cpSpace;
    fn cpSpaceFree(space: *mut cpSpace);
    fn cpSpaceSetIterations(space: *mut cpSpace, iterations: libc::c_int);
    fn cpSpaceSetSleepTimeThreshold(space: *mut cpSpace, sleepTimeThreshold: cpFloat);
    fn cpSpaceGetStaticBody(space: *const cpSpace) -> *mut cpBody;
    fn cpSpaceAddShape(space: *mut cpSpace, shape: *mut cpShape) -> *mut cpShape;
    fn cpSpaceAddBody(space: *mut cpSpace, body: *mut cpBody) -> *mut cpBody;
    fn cpSpaceAddConstraint(
        space: *mut cpSpace,
        constraint: *mut cpConstraint,
    ) -> *mut cpConstraint;
    fn cpSpaceStep(space: *mut cpSpace, dt: cpFloat);
    fn cpMomentForBox(m: cpFloat, width: cpFloat, height: cpFloat) -> cpFloat;
    static mut ChipmunkDemoMouse: cpVect;
    static mut ChipmunkDemoMessageString: *const libc::c_char;
    static mut NOT_GRABBABLE_FILTER: cpShapeFilter;
    fn ChipmunkDemoDefaultDrawImpl(space: *mut cpSpace);
    fn ChipmunkDemoFreeSpaceChildren(space: *mut cpSpace);
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
unsafe extern "C" fn cpvsub(v1: cpVect, v2: cpVect) -> cpVect {
    return cpv(v1.x - v2.x, v1.y - v2.y);
}
#[inline]
unsafe extern "C" fn cpvdot(v1: cpVect, v2: cpVect) -> cpFloat {
    return v1.x * v2.x + v1.y * v2.y;
}
#[inline]
unsafe extern "C" fn cpvtoangle(v: cpVect) -> cpFloat {
    return atan2(v.y, v.x);
}
#[inline]
unsafe extern "C" fn cpvrotate(v1: cpVect, v2: cpVect) -> cpVect {
    return cpv(v1.x * v2.x - v1.y * v2.y, v1.x * v2.y + v1.y * v2.x);
}
#[inline]
unsafe extern "C" fn cpvunrotate(v1: cpVect, v2: cpVect) -> cpVect {
    return cpv(v1.x * v2.x + v1.y * v2.y, v1.y * v2.x - v1.x * v2.y);
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
unsafe extern "C" fn cpvdistsq(v1: cpVect, v2: cpVect) -> cpFloat {
    return cpvlengthsq(cpvsub(v1, v2));
}
#[inline]
unsafe extern "C" fn cpvnear(v1: cpVect, v2: cpVect, dist: cpFloat) -> cpBool {
    return (cpvdistsq(v1, v2) < dist * dist) as libc::c_int as cpBool;
}
#[inline]
unsafe extern "C" fn frand() -> cpFloat {
    return rand() as cpFloat / 2147483647 as libc::c_int as cpFloat;
}
static mut tankBody: *mut cpBody = 0 as *const cpBody as *mut cpBody;
static mut tankControlBody: *mut cpBody = 0 as *const cpBody as *mut cpBody;
unsafe extern "C" fn update(mut space: *mut cpSpace, mut dt: libc::c_double) {
    let mut mouseDelta: cpVect = cpvsub(ChipmunkDemoMouse, cpBodyGetPosition(tankBody));
    let mut turn: cpFloat = cpvtoangle(
        cpvunrotate(cpBodyGetRotation(tankBody), mouseDelta),
    );
    cpBodySetAngle(tankControlBody, cpBodyGetAngle(tankBody) - turn);
    if cpvnear(ChipmunkDemoMouse, cpBodyGetPosition(tankBody), 30.0f64) != 0 {
        cpBodySetVelocity(tankControlBody, cpvzero);
    } else {
        let mut direction: cpFloat = if cpvdot(mouseDelta, cpBodyGetRotation(tankBody))
            > 0.0f64
        {
            1.0f64
        } else {
            -1.0f64
        };
        cpBodySetVelocity(
            tankControlBody,
            cpvrotate(
                cpBodyGetRotation(tankBody),
                cpv(30.0f32 as libc::c_double * direction, 0.0f32 as cpFloat),
            ),
        );
    }
    cpSpaceStep(space, dt);
}
unsafe extern "C" fn add_box(
    mut space: *mut cpSpace,
    mut size: cpFloat,
    mut mass: cpFloat,
) -> *mut cpBody {
    let mut radius: cpFloat = cpvlength(cpv(size, size));
    let mut body: *mut cpBody = cpSpaceAddBody(
        space,
        cpBodyNew(mass, cpMomentForBox(mass, size, size)),
    );
    cpBodySetPosition(
        body,
        cpv(
            frand()
                * (640 as libc::c_int as libc::c_double
                    - 2 as libc::c_int as libc::c_double * radius)
                - (320 as libc::c_int as libc::c_double - radius),
            frand()
                * (480 as libc::c_int as libc::c_double
                    - 2 as libc::c_int as libc::c_double * radius)
                - (240 as libc::c_int as libc::c_double - radius),
        ),
    );
    let mut shape: *mut cpShape = cpSpaceAddShape(
        space,
        cpBoxShapeNew(body, size, size, 0.0f64),
    );
    cpShapeSetElasticity(shape, 0.0f32 as cpFloat);
    cpShapeSetFriction(shape, 0.7f32 as cpFloat);
    return body;
}
unsafe extern "C" fn init() -> *mut cpSpace {
    ChipmunkDemoMessageString = b"Use the mouse to drive the tank, it will follow the cursor.\0"
        as *const u8 as *const libc::c_char;
    let mut space: *mut cpSpace = cpSpaceNew();
    cpSpaceSetIterations(space, 10 as libc::c_int);
    cpSpaceSetSleepTimeThreshold(space, 0.5f32 as cpFloat);
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
    let mut i: libc::c_int = 0 as libc::c_int;
    while i < 50 as libc::c_int {
        let mut body: *mut cpBody = add_box(
            space,
            20 as libc::c_int as cpFloat,
            1 as libc::c_int as cpFloat,
        );
        let mut pivot: *mut cpConstraint = cpSpaceAddConstraint(
            space,
            cpPivotJointNew2(staticBody, body, cpvzero, cpvzero),
        );
        cpConstraintSetMaxBias(pivot, 0 as libc::c_int as cpFloat);
        cpConstraintSetMaxForce(pivot, 1000.0f32 as cpFloat);
        let mut gear: *mut cpConstraint = cpSpaceAddConstraint(
            space,
            cpGearJointNew(staticBody, body, 0.0f32 as cpFloat, 1.0f32 as cpFloat),
        );
        cpConstraintSetMaxBias(gear, 0 as libc::c_int as cpFloat);
        cpConstraintSetMaxForce(gear, 5000.0f32 as cpFloat);
        i += 1;
        i;
    }
    tankControlBody = cpSpaceAddBody(space, cpBodyNewKinematic());
    tankBody = add_box(
        space,
        30 as libc::c_int as cpFloat,
        10 as libc::c_int as cpFloat,
    );
    let mut pivot_0: *mut cpConstraint = cpSpaceAddConstraint(
        space,
        cpPivotJointNew2(tankControlBody, tankBody, cpvzero, cpvzero),
    );
    cpConstraintSetMaxBias(pivot_0, 0 as libc::c_int as cpFloat);
    cpConstraintSetMaxForce(pivot_0, 10000.0f32 as cpFloat);
    let mut gear_0: *mut cpConstraint = cpSpaceAddConstraint(
        space,
        cpGearJointNew(tankControlBody, tankBody, 0.0f32 as cpFloat, 1.0f32 as cpFloat),
    );
    cpConstraintSetErrorBias(gear_0, 0 as libc::c_int as cpFloat);
    cpConstraintSetMaxBias(gear_0, 1.2f32 as cpFloat);
    cpConstraintSetMaxForce(gear_0, 50000.0f32 as cpFloat);
    return space;
}
unsafe extern "C" fn destroy(mut space: *mut cpSpace) {
    ChipmunkDemoFreeSpaceChildren(space);
    cpSpaceFree(space);
}
pub static mut Tank: ChipmunkDemo = unsafe {
    {
        let mut init = ChipmunkDemo {
            name: b"Tank\0" as *const u8 as *const libc::c_char,
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
