use ::libc;
extern "C" {
    pub type cpBody;
    pub type cpShape;
    pub type cpConstraint;
    pub type cpSpace;
    fn asin(_: libc::c_double) -> libc::c_double;
    fn cos(_: libc::c_double) -> libc::c_double;
    fn sin(_: libc::c_double) -> libc::c_double;
    fn pow(_: libc::c_double, _: libc::c_double) -> libc::c_double;
    fn cpMomentForBox2(m: cpFloat, box_0: cpBB) -> cpFloat;
    fn cpMomentForBox(m: cpFloat, width: cpFloat, height: cpFloat) -> cpFloat;
    fn cpMomentForCircle(
        m: cpFloat,
        r1: cpFloat,
        r2: cpFloat,
        offset: cpVect,
    ) -> cpFloat;
    fn cpBodyNew(mass: cpFloat, moment: cpFloat) -> *mut cpBody;
    fn cpBodyGetPosition(body: *const cpBody) -> cpVect;
    fn cpBodySetPosition(body: *mut cpBody, pos: cpVect);
    fn cpBodyGetVelocity(body: *const cpBody) -> cpVect;
    fn cpBodyGetAngularVelocity(body: *const cpBody) -> cpFloat;
    fn cpBodyGetRotation(body: *const cpBody) -> cpVect;
    fn cpBodyWorldToLocal(body: *const cpBody, point: cpVect) -> cpVect;
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
    fn cpBoxShapeNew2(body: *mut cpBody, box_0: cpBB, radius: cpFloat) -> *mut cpShape;
    fn cpConstraintSetMaxForce(constraint: *mut cpConstraint, maxForce: cpFloat);
    fn cpConstraintSetPreSolveFunc(
        constraint: *mut cpConstraint,
        preSolveFunc: cpConstraintPreSolveFunc,
    );
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
    fn cpSpaceGetCurrentTimeStep(space: *const cpSpace) -> cpFloat;
    fn cpSpaceAddShape(space: *mut cpSpace, shape: *mut cpShape) -> *mut cpShape;
    fn cpSpaceAddBody(space: *mut cpSpace, body: *mut cpBody) -> *mut cpBody;
    fn cpSpaceAddConstraint(
        space: *mut cpSpace,
        constraint: *mut cpConstraint,
    ) -> *mut cpConstraint;
    fn cpSpaceStep(space: *mut cpSpace, dt: cpFloat);
    static mut ChipmunkDemoMouse: cpVect;
    static mut ChipmunkDemoMessageString: *const libc::c_char;
    fn ChipmunkDebugDrawSegment(a: cpVect, b: cpVect, color: cpSpaceDebugColor);
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
pub struct cpBB {
    pub l: cpFloat,
    pub b: cpFloat,
    pub r: cpFloat,
    pub t: cpFloat,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct cpShapeFilter {
    pub group: cpGroup,
    pub categories: cpBitmask,
    pub mask: cpBitmask,
}
pub type cpConstraintPreSolveFunc = Option::<
    unsafe extern "C" fn(*mut cpConstraint, *mut cpSpace) -> (),
>;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct cpSpaceDebugColor {
    pub r: libc::c_float,
    pub g: libc::c_float,
    pub b: libc::c_float,
    pub a: libc::c_float,
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
#[inline]
unsafe extern "C" fn cpfmax(mut a: cpFloat, mut b: cpFloat) -> cpFloat {
    return if a > b { a } else { b };
}
#[inline]
unsafe extern "C" fn cpfmin(mut a: cpFloat, mut b: cpFloat) -> cpFloat {
    return if a < b { a } else { b };
}
#[inline]
unsafe extern "C" fn cpfclamp(
    mut f: cpFloat,
    mut min: cpFloat,
    mut max: cpFloat,
) -> cpFloat {
    return cpfmin(cpfmax(f, min), max);
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
unsafe extern "C" fn cpvadd(v1: cpVect, v2: cpVect) -> cpVect {
    return cpv(v1.x + v2.x, v1.y + v2.y);
}
#[inline]
unsafe extern "C" fn cpvcross(v1: cpVect, v2: cpVect) -> cpFloat {
    return v1.x * v2.y - v1.y * v2.x;
}
#[inline]
unsafe extern "C" fn cpvforangle(a: cpFloat) -> cpVect {
    return cpv(cos(a), sin(a));
}
#[inline]
unsafe extern "C" fn cpBBNew(l: cpFloat, b: cpFloat, r: cpFloat, t: cpFloat) -> cpBB {
    let mut bb: cpBB = {
        let mut init = cpBB { l: l, b: b, r: r, t: t };
        init
    };
    return bb;
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
#[inline]
unsafe extern "C" fn RGBAColor(
    mut r: libc::c_float,
    mut g: libc::c_float,
    mut b: libc::c_float,
    mut a: libc::c_float,
) -> cpSpaceDebugColor {
    let mut color: cpSpaceDebugColor = {
        let mut init = cpSpaceDebugColor {
            r: r,
            g: g,
            b: b,
            a: a,
        };
        init
    };
    return color;
}
static mut balance_body: *mut cpBody = 0 as *const cpBody as *mut cpBody;
static mut balance_sin: cpFloat = 0.0f64;
static mut wheel_body: *mut cpBody = 0 as *const cpBody as *mut cpBody;
static mut motor: *mut cpConstraint = 0 as *const cpConstraint as *mut cpConstraint;
#[inline]
unsafe extern "C" fn bias_coef(mut errorBias: cpFloat, mut dt: cpFloat) -> cpFloat {
    return 1.0f32 as libc::c_double - pow(errorBias, dt);
}
unsafe extern "C" fn motor_preSolve(
    mut motor_0: *mut cpConstraint,
    mut space: *mut cpSpace,
) {
    let mut dt: cpFloat = cpSpaceGetCurrentTimeStep(space);
    let mut target_x: cpFloat = ChipmunkDemoMouse.x;
    ChipmunkDebugDrawSegment(
        cpv(target_x, -1000.0f64),
        cpv(target_x, 1000.0f64),
        RGBAColor(
            1.0f64 as libc::c_float,
            0.0f64 as libc::c_float,
            0.0f64 as libc::c_float,
            1.0f64 as libc::c_float,
        ),
    );
    let mut max_v: cpFloat = 500.0f64;
    let mut target_v: cpFloat = cpfclamp(
        bias_coef(0.5f64, dt / 1.2f64) * (target_x - (cpBodyGetPosition(balance_body)).x)
            / dt,
        -max_v,
        max_v,
    );
    let mut error_v: cpFloat = target_v - (cpBodyGetVelocity(balance_body)).x;
    let mut target_sin: cpFloat = 3.0e-3f64 * bias_coef(0.1f64, dt) * error_v / dt;
    let mut max_sin: cpFloat = sin(0.6f64);
    balance_sin = cpfclamp(
        balance_sin - 6.0e-5f64 * bias_coef(0.2f64, dt) * error_v / dt,
        -max_sin,
        max_sin,
    );
    let mut target_a: cpFloat = asin(
        cpfclamp(-target_sin + balance_sin, -max_sin, max_sin),
    );
    let mut angular_diff: cpFloat = asin(
        cpvcross(cpBodyGetRotation(balance_body), cpvforangle(target_a)),
    );
    let mut target_w: cpFloat = bias_coef(0.1f64, dt / 0.4f64) * angular_diff / dt;
    let mut max_rate: cpFloat = 50.0f64;
    let mut rate: cpFloat = cpfclamp(
        cpBodyGetAngularVelocity(wheel_body) + cpBodyGetAngularVelocity(balance_body)
            - target_w,
        -max_rate,
        max_rate,
    );
    cpSimpleMotorSetRate(motor_0, cpfclamp(rate, -max_rate, max_rate));
    cpConstraintSetMaxForce(motor_0, 8.0e4f64);
}
unsafe extern "C" fn update(mut space: *mut cpSpace, mut dt: libc::c_double) {
    cpSpaceStep(space, dt);
}
unsafe extern "C" fn init() -> *mut cpSpace {
    ChipmunkDemoMessageString = b"This unicycle is completely driven and balanced by a single cpSimpleMotor.\nMove the mouse to make the unicycle follow it.\0"
        as *const u8 as *const libc::c_char;
    let mut space: *mut cpSpace = cpSpaceNew();
    cpSpaceSetIterations(space, 30 as libc::c_int);
    cpSpaceSetGravity(
        space,
        cpv(0 as libc::c_int as cpFloat, -(500 as libc::c_int) as cpFloat),
    );
    let mut shape: *mut cpShape = 0 as *mut cpShape;
    let mut staticBody: *mut cpBody = cpSpaceGetStaticBody(space);
    shape = cpSpaceAddShape(
        space,
        cpSegmentShapeNew(
            staticBody,
            cpv(-(3200 as libc::c_int) as cpFloat, -(240 as libc::c_int) as cpFloat),
            cpv(3200 as libc::c_int as cpFloat, -(240 as libc::c_int) as cpFloat),
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
            cpv(0 as libc::c_int as cpFloat, -(200 as libc::c_int) as cpFloat),
            cpv(240 as libc::c_int as cpFloat, -(240 as libc::c_int) as cpFloat),
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
            cpv(-(240 as libc::c_int) as cpFloat, -(240 as libc::c_int) as cpFloat),
            cpv(0 as libc::c_int as cpFloat, -(200 as libc::c_int) as cpFloat),
            0.0f32 as cpFloat,
        ),
    );
    cpShapeSetElasticity(shape, 1.0f32 as cpFloat);
    cpShapeSetFriction(shape, 1.0f32 as cpFloat);
    cpShapeSetFilter(shape, NOT_GRABBABLE_FILTER);
    let mut radius: cpFloat = 20.0f64;
    let mut mass: cpFloat = 1.0f64;
    let mut moment: cpFloat = cpMomentForCircle(mass, 0.0f64, radius, cpvzero);
    wheel_body = cpSpaceAddBody(space, cpBodyNew(mass, moment));
    cpBodySetPosition(wheel_body, cpv(0.0f64, -160.0f64 + radius));
    let mut shape_0: *mut cpShape = cpSpaceAddShape(
        space,
        cpCircleShapeNew(wheel_body, radius, cpvzero),
    );
    cpShapeSetFriction(shape_0, 0.7f64);
    cpShapeSetFilter(
        shape_0,
        cpShapeFilterNew(
            1 as libc::c_int as cpGroup,
            !(0 as libc::c_int as cpBitmask),
            !(0 as libc::c_int as cpBitmask),
        ),
    );
    let mut cog_offset: cpFloat = 30.0f64;
    let mut bb1: cpBB = cpBBNew(
        -5.0f64,
        0.0f64 - cog_offset,
        5.0f64,
        cog_offset * 1.2f64 - cog_offset,
    );
    let mut bb2: cpBB = cpBBNew(-25.0f64, bb1.t, 25.0f64, bb1.t + 10.0f64);
    let mut mass_0: cpFloat = 3.0f64;
    let mut moment_0: cpFloat = cpMomentForBox2(mass_0, bb1)
        + cpMomentForBox2(mass_0, bb2);
    balance_body = cpSpaceAddBody(space, cpBodyNew(mass_0, moment_0));
    cpBodySetPosition(
        balance_body,
        cpv(0.0f64, (cpBodyGetPosition(wheel_body)).y + cog_offset),
    );
    let mut shape_1: *mut cpShape = 0 as *mut cpShape;
    shape_1 = cpSpaceAddShape(space, cpBoxShapeNew2(balance_body, bb1, 0.0f64));
    cpShapeSetFriction(shape_1, 1.0f64);
    cpShapeSetFilter(
        shape_1,
        cpShapeFilterNew(
            1 as libc::c_int as cpGroup,
            !(0 as libc::c_int as cpBitmask),
            !(0 as libc::c_int as cpBitmask),
        ),
    );
    shape_1 = cpSpaceAddShape(space, cpBoxShapeNew2(balance_body, bb2, 0.0f64));
    cpShapeSetFriction(shape_1, 1.0f64);
    cpShapeSetFilter(
        shape_1,
        cpShapeFilterNew(
            1 as libc::c_int as cpGroup,
            !(0 as libc::c_int as cpBitmask),
            !(0 as libc::c_int as cpBitmask),
        ),
    );
    let mut anchorA: cpVect = cpBodyWorldToLocal(
        balance_body,
        cpBodyGetPosition(wheel_body),
    );
    let mut groove_a: cpVect = cpvadd(anchorA, cpv(0.0f64, 30.0f64));
    let mut groove_b: cpVect = cpvadd(anchorA, cpv(0.0f64, -10.0f64));
    cpSpaceAddConstraint(
        space,
        cpGrooveJointNew(balance_body, wheel_body, groove_a, groove_b, cpvzero),
    );
    cpSpaceAddConstraint(
        space,
        cpDampedSpringNew(
            balance_body,
            wheel_body,
            anchorA,
            cpvzero,
            0.0f64,
            6.0e2f64,
            30.0f64,
        ),
    );
    motor = cpSpaceAddConstraint(
        space,
        cpSimpleMotorNew(wheel_body, balance_body, 0.0f64),
    );
    cpConstraintSetPreSolveFunc(
        motor,
        Some(
            motor_preSolve as unsafe extern "C" fn(*mut cpConstraint, *mut cpSpace) -> (),
        ),
    );
    let mut width: cpFloat = 100.0f64;
    let mut height: cpFloat = 20.0f64;
    let mut mass_1: cpFloat = 3.0f64;
    let mut boxBody: *mut cpBody = cpSpaceAddBody(
        space,
        cpBodyNew(mass_1, cpMomentForBox(mass_1, width, height)),
    );
    cpBodySetPosition(
        boxBody,
        cpv(200 as libc::c_int as cpFloat, -(100 as libc::c_int) as cpFloat),
    );
    let mut shape_2: *mut cpShape = cpSpaceAddShape(
        space,
        cpBoxShapeNew(boxBody, width, height, 0.0f64),
    );
    cpShapeSetFriction(shape_2, 0.7f64);
    return space;
}
unsafe extern "C" fn destroy(mut space: *mut cpSpace) {
    ChipmunkDemoFreeSpaceChildren(space);
    cpSpaceFree(space);
}
pub static mut Unicycle: ChipmunkDemo = unsafe {
    {
        let mut init = ChipmunkDemo {
            name: b"Unicycle\0" as *const u8 as *const libc::c_char,
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
