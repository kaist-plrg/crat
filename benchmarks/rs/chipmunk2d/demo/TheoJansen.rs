use ::libc;
extern "C" {
    pub type cpBody;
    pub type cpShape;
    pub type cpConstraint;
    pub type cpSpace;
    fn cos(_: libc::c_double) -> libc::c_double;
    fn sin(_: libc::c_double) -> libc::c_double;
    fn sqrt(_: libc::c_double) -> libc::c_double;
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
    fn cpConstraintSetMaxForce(constraint: *mut cpConstraint, maxForce: cpFloat);
    fn cpPinJointNew(
        a: *mut cpBody,
        b: *mut cpBody,
        anchorA: cpVect,
        anchorB: cpVect,
    ) -> *mut cpConstraint;
    fn cpPinJointSetDist(constraint: *mut cpConstraint, dist: cpFloat);
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
    fn cpSimpleMotorNew(
        a: *mut cpBody,
        b: *mut cpBody,
        rate: cpFloat,
    ) -> *mut cpConstraint;
    fn cpSimpleMotorSetRate(constraint: *mut cpConstraint, rate: cpFloat);
    fn cpSpaceNew() -> *mut cpSpace;
    fn cpSpaceFree(space: *mut cpSpace);
    fn cpSpaceSetIterations(space: *mut cpSpace, iterations: libc::c_int);
    fn cpSpaceSetGravity(space: *mut cpSpace, gravity: cpVect);
    fn cpSpaceGetStaticBody(space: *const cpSpace) -> *mut cpBody;
    fn cpSpaceAddShape(space: *mut cpSpace, shape: *mut cpShape) -> *mut cpShape;
    fn cpSpaceAddBody(space: *mut cpSpace, body: *mut cpBody) -> *mut cpBody;
    fn cpSpaceAddConstraint(
        space: *mut cpSpace,
        constraint: *mut cpConstraint,
    ) -> *mut cpConstraint;
    fn cpSpaceStep(space: *mut cpSpace, dt: cpFloat);
    static mut ChipmunkDemoKeyboard: cpVect;
    static mut ChipmunkDemoMessageString: *const libc::c_char;
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
unsafe extern "C" fn cpvmult(v: cpVect, s: cpFloat) -> cpVect {
    return cpv(v.x * s, v.y * s);
}
#[inline]
unsafe extern "C" fn cpvforangle(a: cpFloat) -> cpVect {
    return cpv(cos(a), sin(a));
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
static mut motor: *mut cpConstraint = 0 as *const cpConstraint as *mut cpConstraint;
unsafe extern "C" fn update(mut space: *mut cpSpace, mut dt: libc::c_double) {
    let mut coef: cpFloat = (2.0f32 as libc::c_double + ChipmunkDemoKeyboard.y)
        / 3.0f32 as libc::c_double;
    let mut rate: cpFloat = ChipmunkDemoKeyboard.x * 10.0f32 as libc::c_double * coef;
    cpSimpleMotorSetRate(motor, rate);
    cpConstraintSetMaxForce(
        motor,
        (if rate != 0. { 100000.0f32 } else { 0.0f32 }) as cpFloat,
    );
    cpSpaceStep(space, dt);
}
static mut seg_radius: cpFloat = 3.0f32 as cpFloat;
unsafe extern "C" fn make_leg(
    mut space: *mut cpSpace,
    mut side: cpFloat,
    mut offset: cpFloat,
    mut chassis: *mut cpBody,
    mut crank: *mut cpBody,
    mut anchor: cpVect,
) {
    let mut a: cpVect = cpVect { x: 0., y: 0. };
    let mut b: cpVect = cpVect { x: 0., y: 0. };
    let mut shape: *mut cpShape = 0 as *mut cpShape;
    let mut leg_mass: cpFloat = 1.0f32 as cpFloat;
    a = cpvzero;
    b = cpv(0.0f32 as cpFloat, side);
    let mut upper_leg: *mut cpBody = cpSpaceAddBody(
        space,
        cpBodyNew(leg_mass, cpMomentForSegment(leg_mass, a, b, 0.0f32 as cpFloat)),
    );
    cpBodySetPosition(upper_leg, cpv(offset, 0.0f32 as cpFloat));
    shape = cpSpaceAddShape(space, cpSegmentShapeNew(upper_leg, a, b, seg_radius));
    cpShapeSetFilter(
        shape,
        cpShapeFilterNew(
            1 as libc::c_int as cpGroup,
            !(0 as libc::c_int as cpBitmask),
            !(0 as libc::c_int as cpBitmask),
        ),
    );
    cpSpaceAddConstraint(
        space,
        cpPivotJointNew2(chassis, upper_leg, cpv(offset, 0.0f32 as cpFloat), cpvzero),
    );
    a = cpvzero;
    b = cpv(0.0f32 as cpFloat, -1.0f32 as libc::c_double * side);
    let mut lower_leg: *mut cpBody = cpSpaceAddBody(
        space,
        cpBodyNew(leg_mass, cpMomentForSegment(leg_mass, a, b, 0.0f32 as cpFloat)),
    );
    cpBodySetPosition(lower_leg, cpv(offset, -side));
    shape = cpSpaceAddShape(space, cpSegmentShapeNew(lower_leg, a, b, seg_radius));
    cpShapeSetFilter(
        shape,
        cpShapeFilterNew(
            1 as libc::c_int as cpGroup,
            !(0 as libc::c_int as cpBitmask),
            !(0 as libc::c_int as cpBitmask),
        ),
    );
    shape = cpSpaceAddShape(
        space,
        cpCircleShapeNew(lower_leg, seg_radius * 2.0f32 as libc::c_double, b),
    );
    cpShapeSetFilter(
        shape,
        cpShapeFilterNew(
            1 as libc::c_int as cpGroup,
            !(0 as libc::c_int as cpBitmask),
            !(0 as libc::c_int as cpBitmask),
        ),
    );
    cpShapeSetElasticity(shape, 0.0f32 as cpFloat);
    cpShapeSetFriction(shape, 1.0f32 as cpFloat);
    cpSpaceAddConstraint(
        space,
        cpPinJointNew(chassis, lower_leg, cpv(offset, 0.0f32 as cpFloat), cpvzero),
    );
    cpSpaceAddConstraint(
        space,
        cpGearJointNew(upper_leg, lower_leg, 0.0f32 as cpFloat, 1.0f32 as cpFloat),
    );
    let mut constraint: *mut cpConstraint = 0 as *mut cpConstraint;
    let mut diag: cpFloat = sqrt(side * side + offset * offset);
    constraint = cpSpaceAddConstraint(
        space,
        cpPinJointNew(crank, upper_leg, anchor, cpv(0.0f32 as cpFloat, side)),
    );
    cpPinJointSetDist(constraint, diag);
    constraint = cpSpaceAddConstraint(
        space,
        cpPinJointNew(crank, lower_leg, anchor, cpvzero),
    );
    cpPinJointSetDist(constraint, diag);
}
unsafe extern "C" fn init() -> *mut cpSpace {
    ChipmunkDemoMessageString = b"Use the arrow keys to control the machine.\0"
        as *const u8 as *const libc::c_char;
    let mut space: *mut cpSpace = cpSpaceNew();
    cpSpaceSetIterations(space, 20 as libc::c_int);
    cpSpaceSetGravity(
        space,
        cpv(0 as libc::c_int as cpFloat, -(500 as libc::c_int) as cpFloat),
    );
    let mut staticBody: *mut cpBody = cpSpaceGetStaticBody(space);
    let mut shape: *mut cpShape = 0 as *mut cpShape;
    let mut a: cpVect = cpVect { x: 0., y: 0. };
    let mut b: cpVect = cpVect { x: 0., y: 0. };
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
    let mut offset: cpFloat = 30.0f32 as cpFloat;
    let mut chassis_mass: cpFloat = 2.0f32 as cpFloat;
    a = cpv(-offset, 0.0f32 as cpFloat);
    b = cpv(offset, 0.0f32 as cpFloat);
    let mut chassis: *mut cpBody = cpSpaceAddBody(
        space,
        cpBodyNew(
            chassis_mass,
            cpMomentForSegment(chassis_mass, a, b, 0.0f32 as cpFloat),
        ),
    );
    shape = cpSpaceAddShape(space, cpSegmentShapeNew(chassis, a, b, seg_radius));
    cpShapeSetFilter(
        shape,
        cpShapeFilterNew(
            1 as libc::c_int as cpGroup,
            !(0 as libc::c_int as cpBitmask),
            !(0 as libc::c_int as cpBitmask),
        ),
    );
    let mut crank_mass: cpFloat = 1.0f32 as cpFloat;
    let mut crank_radius: cpFloat = 13.0f32 as cpFloat;
    let mut crank: *mut cpBody = cpSpaceAddBody(
        space,
        cpBodyNew(
            crank_mass,
            cpMomentForCircle(crank_mass, crank_radius, 0.0f32 as cpFloat, cpvzero),
        ),
    );
    shape = cpSpaceAddShape(space, cpCircleShapeNew(crank, crank_radius, cpvzero));
    cpShapeSetFilter(
        shape,
        cpShapeFilterNew(
            1 as libc::c_int as cpGroup,
            !(0 as libc::c_int as cpBitmask),
            !(0 as libc::c_int as cpBitmask),
        ),
    );
    cpSpaceAddConstraint(space, cpPivotJointNew2(chassis, crank, cpvzero, cpvzero));
    let mut side: cpFloat = 30.0f32 as cpFloat;
    let mut num_legs: libc::c_int = 2 as libc::c_int;
    let mut i: libc::c_int = 0 as libc::c_int;
    while i < num_legs {
        make_leg(
            space,
            side,
            offset,
            chassis,
            crank,
            cpvmult(
                cpvforangle(
                    (2 as libc::c_int * i + 0 as libc::c_int) as cpFloat
                        / num_legs as cpFloat * 3.14159265358979323846264338327950288f64,
                ),
                crank_radius,
            ),
        );
        make_leg(
            space,
            side,
            -offset,
            chassis,
            crank,
            cpvmult(
                cpvforangle(
                    (2 as libc::c_int * i + 1 as libc::c_int) as cpFloat
                        / num_legs as cpFloat * 3.14159265358979323846264338327950288f64,
                ),
                crank_radius,
            ),
        );
        i += 1;
        i;
    }
    motor = cpSpaceAddConstraint(
        space,
        cpSimpleMotorNew(chassis, crank, 6.0f32 as cpFloat),
    );
    return space;
}
unsafe extern "C" fn destroy(mut space: *mut cpSpace) {
    ChipmunkDemoFreeSpaceChildren(space);
    cpSpaceFree(space);
}
pub static mut TheoJansen: ChipmunkDemo = unsafe {
    {
        let mut init = ChipmunkDemo {
            name: b"Theo Jansen Machine\0" as *const u8 as *const libc::c_char,
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
