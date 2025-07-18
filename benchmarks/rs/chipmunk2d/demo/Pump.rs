use ::libc;
extern "C" {
    pub type cpBody;
    pub type cpShape;
    pub type cpConstraint;
    pub type cpSpace;
    fn cpMomentForSegment(m: cpFloat, a: cpVect, b: cpVect, radius: cpFloat) -> cpFloat;
    fn cpMomentForCircle(
        m: cpFloat,
        r1: cpFloat,
        r2: cpFloat,
        offset: cpVect,
    ) -> cpFloat;
    fn cpBodyNew(mass: cpFloat, moment: cpFloat) -> *mut cpBody;
    fn cpBodyGetPosition(body: *const cpBody) -> cpVect;
    fn cpBodySetPosition(body: *mut cpBody, pos: cpVect);
    fn cpBodySetVelocity(body: *mut cpBody, velocity: cpVect);
    fn cpBodySetAngle(body: *mut cpBody, a: cpFloat);
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
    fn cpPolyShapeNew(
        body: *mut cpBody,
        count: libc::c_int,
        verts: *const cpVect,
        transform: cpTransform,
        radius: cpFloat,
    ) -> *mut cpShape;
    fn cpConstraintSetMaxForce(constraint: *mut cpConstraint, maxForce: cpFloat);
    fn cpPinJointNew(
        a: *mut cpBody,
        b: *mut cpBody,
        anchorA: cpVect,
        anchorB: cpVect,
    ) -> *mut cpConstraint;
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
    fn ChipmunkDemoFreeSpaceChildren(space: *mut cpSpace);
    fn ChipmunkDemoDefaultDrawImpl(space: *mut cpSpace);
    static mut NOT_GRABBABLE_FILTER: cpShapeFilter;
    static mut GRAB_FILTER: cpShapeFilter;
    static mut ChipmunkDemoMessageString: *const libc::c_char;
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
static mut cpTransformIdentity: cpTransform = {
    let mut init = cpTransform {
        a: 1.0f32 as cpFloat,
        b: 0.0f32 as cpFloat,
        c: 0.0f32 as cpFloat,
        d: 1.0f32 as cpFloat,
        tx: 0.0f32 as cpFloat,
        ty: 0.0f32 as cpFloat,
    };
    init
};
static mut CP_SHAPE_FILTER_NONE: cpShapeFilter = {
    let mut init = cpShapeFilter {
        group: 0 as libc::c_int as cpGroup,
        categories: !!(0 as libc::c_int as cpBitmask),
        mask: !!(0 as libc::c_int as cpBitmask),
    };
    init
};
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
static mut balls: [*mut cpBody; 5] = [0 as *const cpBody as *mut cpBody; 5];
unsafe extern "C" fn update(mut space: *mut cpSpace, mut dt: libc::c_double) {
    let mut coef: cpFloat = (2.0f32 as libc::c_double + ChipmunkDemoKeyboard.y)
        / 3.0f32 as libc::c_double;
    let mut rate: cpFloat = ChipmunkDemoKeyboard.x * 30.0f32 as libc::c_double * coef;
    cpSimpleMotorSetRate(motor, rate);
    cpConstraintSetMaxForce(
        motor,
        (if rate != 0. { 1000000.0f32 } else { 0.0f32 }) as cpFloat,
    );
    cpSpaceStep(space, dt);
    let mut i: libc::c_int = 0 as libc::c_int;
    while i < 5 as libc::c_int {
        let mut ball: *mut cpBody = balls[i as usize];
        let mut pos: cpVect = cpBodyGetPosition(ball);
        if pos.x > 320.0f32 as libc::c_double {
            cpBodySetVelocity(ball, cpvzero);
            cpBodySetPosition(ball, cpv(-224.0f32 as cpFloat, 200.0f32 as cpFloat));
        }
        i += 1;
        i;
    }
}
unsafe extern "C" fn add_ball(mut space: *mut cpSpace, mut pos: cpVect) -> *mut cpBody {
    let mut body: *mut cpBody = cpSpaceAddBody(
        space,
        cpBodyNew(
            1.0f32 as cpFloat,
            cpMomentForCircle(
                1.0f32 as cpFloat,
                30 as libc::c_int as cpFloat,
                0 as libc::c_int as cpFloat,
                cpvzero,
            ),
        ),
    );
    cpBodySetPosition(body, pos);
    let mut shape: *mut cpShape = cpSpaceAddShape(
        space,
        cpCircleShapeNew(body, 30 as libc::c_int as cpFloat, cpvzero),
    );
    cpShapeSetElasticity(shape, 0.0f32 as cpFloat);
    cpShapeSetFriction(shape, 0.5f32 as cpFloat);
    return body;
}
unsafe extern "C" fn init() -> *mut cpSpace {
    ChipmunkDemoMessageString = b"Use the arrow keys to control the machine.\0"
        as *const u8 as *const libc::c_char;
    let mut space: *mut cpSpace = cpSpaceNew();
    cpSpaceSetGravity(
        space,
        cpv(0 as libc::c_int as cpFloat, -(600 as libc::c_int) as cpFloat),
    );
    let mut staticBody: *mut cpBody = cpSpaceGetStaticBody(space);
    let mut shape: *mut cpShape = 0 as *mut cpShape;
    shape = cpSpaceAddShape(
        space,
        cpSegmentShapeNew(
            staticBody,
            cpv(-(256 as libc::c_int) as cpFloat, 16 as libc::c_int as cpFloat),
            cpv(-(256 as libc::c_int) as cpFloat, 300 as libc::c_int as cpFloat),
            2.0f32 as cpFloat,
        ),
    );
    cpShapeSetElasticity(shape, 0.0f32 as cpFloat);
    cpShapeSetFriction(shape, 0.5f32 as cpFloat);
    cpShapeSetFilter(shape, NOT_GRABBABLE_FILTER);
    shape = cpSpaceAddShape(
        space,
        cpSegmentShapeNew(
            staticBody,
            cpv(-(256 as libc::c_int) as cpFloat, 16 as libc::c_int as cpFloat),
            cpv(-(192 as libc::c_int) as cpFloat, 0 as libc::c_int as cpFloat),
            2.0f32 as cpFloat,
        ),
    );
    cpShapeSetElasticity(shape, 0.0f32 as cpFloat);
    cpShapeSetFriction(shape, 0.5f32 as cpFloat);
    cpShapeSetFilter(shape, NOT_GRABBABLE_FILTER);
    shape = cpSpaceAddShape(
        space,
        cpSegmentShapeNew(
            staticBody,
            cpv(-(192 as libc::c_int) as cpFloat, 0 as libc::c_int as cpFloat),
            cpv(-(192 as libc::c_int) as cpFloat, -(64 as libc::c_int) as cpFloat),
            2.0f32 as cpFloat,
        ),
    );
    cpShapeSetElasticity(shape, 0.0f32 as cpFloat);
    cpShapeSetFriction(shape, 0.5f32 as cpFloat);
    cpShapeSetFilter(shape, NOT_GRABBABLE_FILTER);
    shape = cpSpaceAddShape(
        space,
        cpSegmentShapeNew(
            staticBody,
            cpv(-(128 as libc::c_int) as cpFloat, -(64 as libc::c_int) as cpFloat),
            cpv(-(128 as libc::c_int) as cpFloat, 144 as libc::c_int as cpFloat),
            2.0f32 as cpFloat,
        ),
    );
    cpShapeSetElasticity(shape, 0.0f32 as cpFloat);
    cpShapeSetFriction(shape, 0.5f32 as cpFloat);
    cpShapeSetFilter(shape, NOT_GRABBABLE_FILTER);
    shape = cpSpaceAddShape(
        space,
        cpSegmentShapeNew(
            staticBody,
            cpv(-(192 as libc::c_int) as cpFloat, 80 as libc::c_int as cpFloat),
            cpv(-(192 as libc::c_int) as cpFloat, 176 as libc::c_int as cpFloat),
            2.0f32 as cpFloat,
        ),
    );
    cpShapeSetElasticity(shape, 0.0f32 as cpFloat);
    cpShapeSetFriction(shape, 0.5f32 as cpFloat);
    cpShapeSetFilter(shape, NOT_GRABBABLE_FILTER);
    shape = cpSpaceAddShape(
        space,
        cpSegmentShapeNew(
            staticBody,
            cpv(-(192 as libc::c_int) as cpFloat, 176 as libc::c_int as cpFloat),
            cpv(-(128 as libc::c_int) as cpFloat, 240 as libc::c_int as cpFloat),
            2.0f32 as cpFloat,
        ),
    );
    cpShapeSetElasticity(shape, 0.0f32 as cpFloat);
    cpShapeSetFriction(shape, 0.5f32 as cpFloat);
    cpShapeSetFilter(shape, NOT_GRABBABLE_FILTER);
    shape = cpSpaceAddShape(
        space,
        cpSegmentShapeNew(
            staticBody,
            cpv(-(128 as libc::c_int) as cpFloat, 144 as libc::c_int as cpFloat),
            cpv(192 as libc::c_int as cpFloat, 64 as libc::c_int as cpFloat),
            2.0f32 as cpFloat,
        ),
    );
    cpShapeSetElasticity(shape, 0.0f32 as cpFloat);
    cpShapeSetFriction(shape, 0.5f32 as cpFloat);
    cpShapeSetFilter(shape, NOT_GRABBABLE_FILTER);
    let mut verts: [cpVect; 4] = [
        cpv(-(30 as libc::c_int) as cpFloat, -(80 as libc::c_int) as cpFloat),
        cpv(-(30 as libc::c_int) as cpFloat, 80 as libc::c_int as cpFloat),
        cpv(30 as libc::c_int as cpFloat, 64 as libc::c_int as cpFloat),
        cpv(30 as libc::c_int as cpFloat, -(80 as libc::c_int) as cpFloat),
    ];
    let mut plunger: *mut cpBody = cpSpaceAddBody(
        space,
        cpBodyNew(1.0f32 as cpFloat, ::std::f32::INFINITY as cpFloat),
    );
    cpBodySetPosition(
        plunger,
        cpv(-(160 as libc::c_int) as cpFloat, -(80 as libc::c_int) as cpFloat),
    );
    shape = cpSpaceAddShape(
        space,
        cpPolyShapeNew(
            plunger,
            4 as libc::c_int,
            verts.as_mut_ptr(),
            cpTransformIdentity,
            0.0f64,
        ),
    );
    cpShapeSetElasticity(shape, 1.0f32 as cpFloat);
    cpShapeSetFriction(shape, 0.5f32 as cpFloat);
    cpShapeSetFilter(
        shape,
        cpShapeFilterNew(
            0 as libc::c_int as cpGroup,
            1 as libc::c_int as cpBitmask,
            1 as libc::c_int as cpBitmask,
        ),
    );
    let mut i: libc::c_int = 0 as libc::c_int;
    while i < 5 as libc::c_int {
        balls[i
            as usize] = add_ball(
            space,
            cpv(
                (-(224 as libc::c_int) + i) as cpFloat,
                (80 as libc::c_int + 64 as libc::c_int * i) as cpFloat,
            ),
        );
        i += 1;
        i;
    }
    let mut smallGear: *mut cpBody = cpSpaceAddBody(
        space,
        cpBodyNew(
            10.0f32 as cpFloat,
            cpMomentForCircle(
                10.0f32 as cpFloat,
                80 as libc::c_int as cpFloat,
                0 as libc::c_int as cpFloat,
                cpvzero,
            ),
        ),
    );
    cpBodySetPosition(
        smallGear,
        cpv(-(160 as libc::c_int) as cpFloat, -(160 as libc::c_int) as cpFloat),
    );
    cpBodySetAngle(
        smallGear,
        -3.14159265358979323846264338327950288f64 / 2.0f32 as libc::c_double,
    );
    shape = cpSpaceAddShape(
        space,
        cpCircleShapeNew(smallGear, 80.0f32 as cpFloat, cpvzero),
    );
    cpShapeSetFilter(shape, CP_SHAPE_FILTER_NONE);
    cpSpaceAddConstraint(
        space,
        cpPivotJointNew2(
            staticBody,
            smallGear,
            cpv(-(160 as libc::c_int) as cpFloat, -(160 as libc::c_int) as cpFloat),
            cpvzero,
        ),
    );
    let mut bigGear: *mut cpBody = cpSpaceAddBody(
        space,
        cpBodyNew(
            40.0f32 as cpFloat,
            cpMomentForCircle(
                40.0f32 as cpFloat,
                160 as libc::c_int as cpFloat,
                0 as libc::c_int as cpFloat,
                cpvzero,
            ),
        ),
    );
    cpBodySetPosition(
        bigGear,
        cpv(80 as libc::c_int as cpFloat, -(160 as libc::c_int) as cpFloat),
    );
    cpBodySetAngle(
        bigGear,
        3.14159265358979323846264338327950288f64 / 2.0f32 as libc::c_double,
    );
    shape = cpSpaceAddShape(
        space,
        cpCircleShapeNew(bigGear, 160.0f32 as cpFloat, cpvzero),
    );
    cpShapeSetFilter(shape, CP_SHAPE_FILTER_NONE);
    cpSpaceAddConstraint(
        space,
        cpPivotJointNew2(
            staticBody,
            bigGear,
            cpv(80 as libc::c_int as cpFloat, -(160 as libc::c_int) as cpFloat),
            cpvzero,
        ),
    );
    cpSpaceAddConstraint(
        space,
        cpPinJointNew(
            smallGear,
            plunger,
            cpv(80 as libc::c_int as cpFloat, 0 as libc::c_int as cpFloat),
            cpv(0 as libc::c_int as cpFloat, 0 as libc::c_int as cpFloat),
        ),
    );
    cpSpaceAddConstraint(
        space,
        cpGearJointNew(
            smallGear,
            bigGear,
            -3.14159265358979323846264338327950288f64 / 2.0f32 as libc::c_double,
            -2.0f32 as cpFloat,
        ),
    );
    let mut bottom: cpFloat = -300.0f32 as cpFloat;
    let mut top: cpFloat = 32.0f32 as cpFloat;
    let mut feeder: *mut cpBody = cpSpaceAddBody(
        space,
        cpBodyNew(
            1.0f32 as cpFloat,
            cpMomentForSegment(
                1.0f32 as cpFloat,
                cpv(-224.0f32 as cpFloat, bottom),
                cpv(-224.0f32 as cpFloat, top),
                0.0f32 as cpFloat,
            ),
        ),
    );
    cpBodySetPosition(
        feeder,
        cpv(-(224 as libc::c_int) as cpFloat, (bottom + top) / 2.0f32 as libc::c_double),
    );
    let mut len: cpFloat = top - bottom;
    shape = cpSpaceAddShape(
        space,
        cpSegmentShapeNew(
            feeder,
            cpv(0.0f32 as cpFloat, len / 2.0f32 as libc::c_double),
            cpv(0.0f32 as cpFloat, -len / 2.0f32 as libc::c_double),
            20.0f32 as cpFloat,
        ),
    );
    cpShapeSetFilter(shape, GRAB_FILTER);
    cpSpaceAddConstraint(
        space,
        cpPivotJointNew2(
            staticBody,
            feeder,
            cpv(-224.0f32 as cpFloat, bottom),
            cpv(0.0f32 as cpFloat, -len / 2.0f32 as libc::c_double),
        ),
    );
    let mut anchr: cpVect = cpBodyWorldToLocal(
        feeder,
        cpv(-224.0f32 as cpFloat, -160.0f32 as cpFloat),
    );
    cpSpaceAddConstraint(
        space,
        cpPinJointNew(
            feeder,
            smallGear,
            anchr,
            cpv(0.0f32 as cpFloat, 80.0f32 as cpFloat),
        ),
    );
    motor = cpSpaceAddConstraint(
        space,
        cpSimpleMotorNew(staticBody, bigGear, 3.0f32 as cpFloat),
    );
    return space;
}
unsafe extern "C" fn destroy(mut space: *mut cpSpace) {
    ChipmunkDemoFreeSpaceChildren(space);
    cpSpaceFree(space);
}
pub static mut Pump: ChipmunkDemo = unsafe {
    {
        let mut init = ChipmunkDemo {
            name: b"Pump\0" as *const u8 as *const libc::c_char,
            timestep: 1.0f64 / 120.0f64,
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
