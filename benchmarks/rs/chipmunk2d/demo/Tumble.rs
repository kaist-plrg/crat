use ::libc;
extern "C" {
    pub type cpBody;
    pub type cpShape;
    pub type cpSpace;
    fn rand() -> libc::c_int;
    fn cpMomentForBox(m: cpFloat, width: cpFloat, height: cpFloat) -> cpFloat;
    fn cpMomentForCircle(
        m: cpFloat,
        r1: cpFloat,
        r2: cpFloat,
        offset: cpVect,
    ) -> cpFloat;
    fn cpBodyNew(mass: cpFloat, moment: cpFloat) -> *mut cpBody;
    fn cpBodyNewKinematic() -> *mut cpBody;
    fn cpBodySetPosition(body: *mut cpBody, pos: cpVect);
    fn cpBodySetAngularVelocity(body: *mut cpBody, angularVelocity: cpFloat);
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
    fn cpSpaceSetGravity(space: *mut cpSpace, gravity: cpVect);
    fn cpSpaceAddShape(space: *mut cpSpace, shape: *mut cpShape) -> *mut cpShape;
    fn cpSpaceAddBody(space: *mut cpSpace, body: *mut cpBody) -> *mut cpBody;
    fn cpSpaceStep(space: *mut cpSpace, dt: cpFloat);
    fn ChipmunkDemoFreeSpaceChildren(space: *mut cpSpace);
    static mut NOT_GRABBABLE_FILTER: cpShapeFilter;
    fn ChipmunkDemoDefaultDrawImpl(space: *mut cpSpace);
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
static mut KinematicBoxBody: *mut cpBody = 0 as *const cpBody as *mut cpBody;
unsafe extern "C" fn update(mut space: *mut cpSpace, mut dt: libc::c_double) {
    cpSpaceStep(space, dt);
}
unsafe extern "C" fn AddBox(
    mut space: *mut cpSpace,
    mut pos: cpVect,
    mut mass: cpFloat,
    mut width: cpFloat,
    mut height: cpFloat,
) {
    let mut body: *mut cpBody = cpSpaceAddBody(
        space,
        cpBodyNew(mass, cpMomentForBox(mass, width, height)),
    );
    cpBodySetPosition(body, pos);
    let mut shape: *mut cpShape = cpSpaceAddShape(
        space,
        cpBoxShapeNew(body, width, height, 0.0f64),
    );
    cpShapeSetElasticity(shape, 0.0f32 as cpFloat);
    cpShapeSetFriction(shape, 0.7f32 as cpFloat);
}
unsafe extern "C" fn AddSegment(
    mut space: *mut cpSpace,
    mut pos: cpVect,
    mut mass: cpFloat,
    mut width: cpFloat,
    mut height: cpFloat,
) {
    let mut body: *mut cpBody = cpSpaceAddBody(
        space,
        cpBodyNew(mass, cpMomentForBox(mass, width, height)),
    );
    cpBodySetPosition(body, pos);
    let mut shape: *mut cpShape = cpSpaceAddShape(
        space,
        cpSegmentShapeNew(
            body,
            cpv(0.0f64, (height - width) / 2.0f64),
            cpv(0.0f64, (width - height) / 2.0f64),
            width / 2.0f64,
        ),
    );
    cpShapeSetElasticity(shape, 0.0f32 as cpFloat);
    cpShapeSetFriction(shape, 0.7f32 as cpFloat);
}
unsafe extern "C" fn AddCircle(
    mut space: *mut cpSpace,
    mut pos: cpVect,
    mut mass: cpFloat,
    mut radius: cpFloat,
) {
    let mut body: *mut cpBody = cpSpaceAddBody(
        space,
        cpBodyNew(mass, cpMomentForCircle(mass, 0.0f64, radius, cpvzero)),
    );
    cpBodySetPosition(body, pos);
    let mut shape: *mut cpShape = cpSpaceAddShape(
        space,
        cpCircleShapeNew(body, radius, cpvzero),
    );
    cpShapeSetElasticity(shape, 0.0f32 as cpFloat);
    cpShapeSetFriction(shape, 0.7f32 as cpFloat);
}
unsafe extern "C" fn init() -> *mut cpSpace {
    let mut space: *mut cpSpace = cpSpaceNew();
    cpSpaceSetGravity(
        space,
        cpv(0 as libc::c_int as cpFloat, -(600 as libc::c_int) as cpFloat),
    );
    let mut shape: *mut cpShape = 0 as *mut cpShape;
    KinematicBoxBody = cpSpaceAddBody(space, cpBodyNewKinematic());
    cpBodySetAngularVelocity(KinematicBoxBody, 0.4f32 as cpFloat);
    let mut a: cpVect = cpv(
        -(200 as libc::c_int) as cpFloat,
        -(200 as libc::c_int) as cpFloat,
    );
    let mut b: cpVect = cpv(
        -(200 as libc::c_int) as cpFloat,
        200 as libc::c_int as cpFloat,
    );
    let mut c: cpVect = cpv(
        200 as libc::c_int as cpFloat,
        200 as libc::c_int as cpFloat,
    );
    let mut d: cpVect = cpv(
        200 as libc::c_int as cpFloat,
        -(200 as libc::c_int) as cpFloat,
    );
    shape = cpSpaceAddShape(
        space,
        cpSegmentShapeNew(KinematicBoxBody, a, b, 0.0f32 as cpFloat),
    );
    cpShapeSetElasticity(shape, 1.0f32 as cpFloat);
    cpShapeSetFriction(shape, 1.0f32 as cpFloat);
    cpShapeSetFilter(shape, NOT_GRABBABLE_FILTER);
    shape = cpSpaceAddShape(
        space,
        cpSegmentShapeNew(KinematicBoxBody, b, c, 0.0f32 as cpFloat),
    );
    cpShapeSetElasticity(shape, 1.0f32 as cpFloat);
    cpShapeSetFriction(shape, 1.0f32 as cpFloat);
    cpShapeSetFilter(shape, NOT_GRABBABLE_FILTER);
    shape = cpSpaceAddShape(
        space,
        cpSegmentShapeNew(KinematicBoxBody, c, d, 0.0f32 as cpFloat),
    );
    cpShapeSetElasticity(shape, 1.0f32 as cpFloat);
    cpShapeSetFriction(shape, 1.0f32 as cpFloat);
    cpShapeSetFilter(shape, NOT_GRABBABLE_FILTER);
    shape = cpSpaceAddShape(
        space,
        cpSegmentShapeNew(KinematicBoxBody, d, a, 0.0f32 as cpFloat),
    );
    cpShapeSetElasticity(shape, 1.0f32 as cpFloat);
    cpShapeSetFriction(shape, 1.0f32 as cpFloat);
    cpShapeSetFilter(shape, NOT_GRABBABLE_FILTER);
    let mut mass: cpFloat = 1 as libc::c_int as cpFloat;
    let mut width: cpFloat = 30 as libc::c_int as cpFloat;
    let mut height: cpFloat = width * 2 as libc::c_int as libc::c_double;
    let mut i: libc::c_int = 0 as libc::c_int;
    while i < 7 as libc::c_int {
        let mut j: libc::c_int = 0 as libc::c_int;
        while j < 3 as libc::c_int {
            let mut pos: cpVect = cpv(
                i as libc::c_double * width - 150 as libc::c_int as libc::c_double,
                j as libc::c_double * height - 150 as libc::c_int as libc::c_double,
            );
            let mut type_0: libc::c_int = rand() % 3000 as libc::c_int
                / 1000 as libc::c_int;
            if type_0 == 0 as libc::c_int {
                AddBox(space, pos, mass, width, height);
            } else if type_0 == 1 as libc::c_int {
                AddSegment(space, pos, mass, width, height);
            } else {
                AddCircle(
                    space,
                    cpvadd(pos, cpv(0.0f64, (height - width) / 2.0f64)),
                    mass,
                    width / 2.0f64,
                );
                AddCircle(
                    space,
                    cpvadd(pos, cpv(0.0f64, (width - height) / 2.0f64)),
                    mass,
                    width / 2.0f64,
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
pub static mut Tumble: ChipmunkDemo = unsafe {
    {
        let mut init = ChipmunkDemo {
            name: b"Tumble\0" as *const u8 as *const libc::c_char,
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
