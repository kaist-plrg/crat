use ::libc;
extern "C" {
    pub type cpBody;
    pub type cpShape;
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
    fn cpSpaceNew() -> *mut cpSpace;
    fn cpSpaceFree(space: *mut cpSpace);
    fn cpSpaceSetIterations(space: *mut cpSpace, iterations: libc::c_int);
    fn cpSpaceSetGravity(space: *mut cpSpace, gravity: cpVect);
    fn cpSpaceSetSleepTimeThreshold(space: *mut cpSpace, sleepTimeThreshold: cpFloat);
    fn cpSpaceSetCollisionSlop(space: *mut cpSpace, collisionSlop: cpFloat);
    fn cpSpaceGetStaticBody(space: *const cpSpace) -> *mut cpBody;
    fn cpSpaceAddShape(space: *mut cpSpace, shape: *mut cpShape) -> *mut cpShape;
    fn cpSpaceAddBody(space: *mut cpSpace, body: *mut cpBody) -> *mut cpBody;
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
    cpSpaceSetCollisionSlop(space, 0.5f32 as cpFloat);
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
    let mut i: libc::c_int = 0 as libc::c_int;
    while i < 14 as libc::c_int {
        let mut j: libc::c_int = 0 as libc::c_int;
        while j <= i {
            body = cpSpaceAddBody(
                space,
                cpBodyNew(
                    1.0f32 as cpFloat,
                    cpMomentForBox(
                        1.0f32 as cpFloat,
                        30.0f32 as cpFloat,
                        30.0f32 as cpFloat,
                    ),
                ),
            );
            cpBodySetPosition(
                body,
                cpv(
                    (j * 32 as libc::c_int - i * 16 as libc::c_int) as cpFloat,
                    (300 as libc::c_int - i * 32 as libc::c_int) as cpFloat,
                ),
            );
            shape = cpSpaceAddShape(
                space,
                cpBoxShapeNew(
                    body,
                    30.0f32 as cpFloat,
                    30.0f32 as cpFloat,
                    0.5f32 as cpFloat,
                ),
            );
            cpShapeSetElasticity(shape, 0.0f32 as cpFloat);
            cpShapeSetFriction(shape, 0.8f32 as cpFloat);
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
    shape = cpSpaceAddShape(space, cpCircleShapeNew(body, radius, cpvzero));
    cpShapeSetElasticity(shape, 0.0f32 as cpFloat);
    cpShapeSetFriction(shape, 0.9f32 as cpFloat);
    return space;
}
unsafe extern "C" fn destroy(mut space: *mut cpSpace) {
    ChipmunkDemoFreeSpaceChildren(space);
    cpSpaceFree(space);
}
pub static mut PyramidStack: ChipmunkDemo = unsafe {
    {
        let mut init = ChipmunkDemo {
            name: b"Pyramid Stack\0" as *const u8 as *const libc::c_char,
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
