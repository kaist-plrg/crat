use ::libc;
extern "C" {
    pub type cpBody;
    pub type cpShape;
    pub type cpSpace;
    fn cpMomentForBox(m: cpFloat, width: cpFloat, height: cpFloat) -> cpFloat;
    fn cpBodyNew(mass: cpFloat, moment: cpFloat) -> *mut cpBody;
    fn cpBodySetPosition(body: *mut cpBody, pos: cpVect);
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
unsafe extern "C" fn update(mut space: *mut cpSpace, mut dt: libc::c_double) {
    cpSpaceStep(space, dt);
}
unsafe extern "C" fn add_domino(
    mut space: *mut cpSpace,
    mut pos: cpVect,
    mut flipped: cpBool,
) {
    let mut mass: cpFloat = 1.0f32 as cpFloat;
    let mut radius: cpFloat = 0.5f32 as cpFloat;
    let mut moment: cpFloat = cpMomentForBox(
        mass,
        4.0f32 as cpFloat,
        30.0f32 as cpFloat,
    );
    let mut body: *mut cpBody = cpSpaceAddBody(space, cpBodyNew(mass, moment));
    cpBodySetPosition(body, pos);
    let mut shape: *mut cpShape = if flipped as libc::c_int != 0 {
        cpBoxShapeNew(body, 30.0f32 as cpFloat, 4.0f32 as cpFloat, 0.0f64)
    } else {
        cpBoxShapeNew(
            body,
            4.0f32 as libc::c_double - radius * 2.0f32 as libc::c_double,
            30.0f32 as cpFloat,
            radius,
        )
    };
    cpSpaceAddShape(space, shape);
    cpShapeSetElasticity(shape, 0.0f32 as cpFloat);
    cpShapeSetFriction(shape, 0.6f32 as cpFloat);
}
unsafe extern "C" fn init() -> *mut cpSpace {
    let mut space: *mut cpSpace = cpSpaceNew();
    cpSpaceSetIterations(space, 30 as libc::c_int);
    cpSpaceSetGravity(
        space,
        cpv(0 as libc::c_int as cpFloat, -(300 as libc::c_int) as cpFloat),
    );
    cpSpaceSetSleepTimeThreshold(space, 0.5f32 as cpFloat);
    cpSpaceSetCollisionSlop(space, 0.5f32 as cpFloat);
    let mut shape: *mut cpShape = cpSpaceAddShape(
        space,
        cpSegmentShapeNew(
            cpSpaceGetStaticBody(space),
            cpv(-(600 as libc::c_int) as cpFloat, -(240 as libc::c_int) as cpFloat),
            cpv(600 as libc::c_int as cpFloat, -(240 as libc::c_int) as cpFloat),
            0.0f32 as cpFloat,
        ),
    );
    cpShapeSetElasticity(shape, 1.0f32 as cpFloat);
    cpShapeSetFriction(shape, 1.0f32 as cpFloat);
    cpShapeSetFilter(shape, NOT_GRABBABLE_FILTER);
    let mut n: libc::c_int = 12 as libc::c_int;
    let mut i: libc::c_int = 0 as libc::c_int;
    while i < n {
        let mut j: libc::c_int = 0 as libc::c_int;
        while j < n - i {
            let mut offset: cpVect = cpv(
                ((j as libc::c_float
                    - (n - 1 as libc::c_int - i) as libc::c_float * 0.5f32) * 1.5f32
                    * 30.0f32) as cpFloat,
                ((i as libc::c_float + 0.5f32)
                    * (30.0f32 + 2 as libc::c_int as libc::c_float * 4.0f32) - 4.0f32
                    - 240 as libc::c_int as libc::c_float) as cpFloat,
            );
            add_domino(space, offset, 0 as libc::c_int as cpBool);
            add_domino(
                space,
                cpvadd(
                    offset,
                    cpv(
                        0 as libc::c_int as cpFloat,
                        ((30.0f32 + 4.0f32) / 2.0f32) as cpFloat,
                    ),
                ),
                1 as libc::c_int as cpBool,
            );
            if j == 0 as libc::c_int {
                add_domino(
                    space,
                    cpvadd(
                        offset,
                        cpv(
                            (0.5f32 * (4.0f32 - 30.0f32)) as cpFloat,
                            (30.0f32 + 4.0f32) as cpFloat,
                        ),
                    ),
                    0 as libc::c_int as cpBool,
                );
            }
            if j != n - i - 1 as libc::c_int {
                add_domino(
                    space,
                    cpvadd(
                        offset,
                        cpv(
                            (30.0f32 * 0.75f32) as cpFloat,
                            ((30.0f32 + 3 as libc::c_int as libc::c_float * 4.0f32)
                                / 2.0f32) as cpFloat,
                        ),
                    ),
                    1 as libc::c_int as cpBool,
                );
            } else {
                add_domino(
                    space,
                    cpvadd(
                        offset,
                        cpv(
                            (0.5f32 * (30.0f32 - 4.0f32)) as cpFloat,
                            (30.0f32 + 4.0f32) as cpFloat,
                        ),
                    ),
                    0 as libc::c_int as cpBool,
                );
            }
            j += 1;
            j;
        }
        i += 1;
        i;
    }
    return space;
}
unsafe extern "C" fn destroy(mut space: *mut cpSpace) {
    ChipmunkDemoFreeSpaceChildren(space);
    cpSpaceFree(space);
}
pub static mut PyramidTopple: ChipmunkDemo = unsafe {
    {
        let mut init = ChipmunkDemo {
            name: b"Pyramid Topple\0" as *const u8 as *const libc::c_char,
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
