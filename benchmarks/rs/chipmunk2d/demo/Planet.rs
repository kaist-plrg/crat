use ::libc;
extern "C" {
    pub type cpBody;
    pub type cpShape;
    pub type cpSpace;
    fn atan2(_: libc::c_double, _: libc::c_double) -> libc::c_double;
    fn sqrt(_: libc::c_double) -> libc::c_double;
    fn rand() -> libc::c_int;
    fn cpMomentForPoly(
        m: cpFloat,
        count: libc::c_int,
        verts: *const cpVect,
        offset: cpVect,
        radius: cpFloat,
    ) -> cpFloat;
    fn cpBodyNew(mass: cpFloat, moment: cpFloat) -> *mut cpBody;
    fn cpBodyNewKinematic() -> *mut cpBody;
    fn cpBodyGetPosition(body: *const cpBody) -> cpVect;
    fn cpBodySetPosition(body: *mut cpBody, pos: cpVect);
    fn cpBodySetVelocity(body: *mut cpBody, velocity: cpVect);
    fn cpBodySetAngle(body: *mut cpBody, a: cpFloat);
    fn cpBodySetAngularVelocity(body: *mut cpBody, angularVelocity: cpFloat);
    fn cpBodySetVelocityUpdateFunc(body: *mut cpBody, velocityFunc: cpBodyVelocityFunc);
    fn cpBodyUpdateVelocity(
        body: *mut cpBody,
        gravity: cpVect,
        damping: cpFloat,
        dt: cpFloat,
    );
    fn cpShapeSetElasticity(shape: *mut cpShape, elasticity: cpFloat);
    fn cpShapeSetFriction(shape: *mut cpShape, friction: cpFloat);
    fn cpShapeSetFilter(shape: *mut cpShape, filter: cpShapeFilter);
    fn cpCircleShapeNew(
        body: *mut cpBody,
        radius: cpFloat,
        offset: cpVect,
    ) -> *mut cpShape;
    fn cpPolyShapeNew(
        body: *mut cpBody,
        count: libc::c_int,
        verts: *const cpVect,
        transform: cpTransform,
        radius: cpFloat,
    ) -> *mut cpShape;
    fn cpSpaceNew() -> *mut cpSpace;
    fn cpSpaceFree(space: *mut cpSpace);
    fn cpSpaceSetIterations(space: *mut cpSpace, iterations: libc::c_int);
    fn cpSpaceAddShape(space: *mut cpSpace, shape: *mut cpShape) -> *mut cpShape;
    fn cpSpaceAddBody(space: *mut cpSpace, body: *mut cpBody) -> *mut cpBody;
    fn cpSpaceStep(space: *mut cpSpace, dt: cpFloat);
    fn ChipmunkDemoDefaultDrawImpl(space: *mut cpSpace);
    static mut NOT_GRABBABLE_FILTER: cpShapeFilter;
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
pub struct cpTransform {
    pub a: cpFloat,
    pub b: cpFloat,
    pub c: cpFloat,
    pub d: cpFloat,
    pub tx: cpFloat,
    pub ty: cpFloat,
}
pub type cpBodyVelocityFunc = Option::<
    unsafe extern "C" fn(*mut cpBody, cpVect, cpFloat, cpFloat) -> (),
>;
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
unsafe extern "C" fn cpvdot(v1: cpVect, v2: cpVect) -> cpFloat {
    return v1.x * v2.x + v1.y * v2.y;
}
#[inline]
unsafe extern "C" fn cpvperp(v: cpVect) -> cpVect {
    return cpv(-v.y, v.x);
}
#[inline]
unsafe extern "C" fn cpvlengthsq(v: cpVect) -> cpFloat {
    return cpvdot(v, v);
}
#[inline]
unsafe extern "C" fn cpvlength(v: cpVect) -> cpFloat {
    return sqrt(cpvdot(v, v));
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
#[inline]
unsafe extern "C" fn frand() -> cpFloat {
    return rand() as cpFloat / 2147483647 as libc::c_int as cpFloat;
}
static mut planetBody: *mut cpBody = 0 as *const cpBody as *mut cpBody;
static mut gravityStrength: cpFloat = 5.0e6f32 as cpFloat;
unsafe extern "C" fn update(mut space: *mut cpSpace, mut dt: libc::c_double) {
    cpSpaceStep(space, dt);
}
unsafe extern "C" fn planetGravityVelocityFunc(
    mut body: *mut cpBody,
    mut gravity: cpVect,
    mut damping: cpFloat,
    mut dt: cpFloat,
) {
    let mut p: cpVect = cpBodyGetPosition(body);
    let mut sqdist: cpFloat = cpvlengthsq(p);
    let mut g: cpVect = cpvmult(p, -gravityStrength / (sqdist * sqrt(sqdist)));
    cpBodyUpdateVelocity(body, g, damping, dt);
}
unsafe extern "C" fn rand_pos(mut radius: cpFloat) -> cpVect {
    let mut v: cpVect = cpVect { x: 0., y: 0. };
    loop {
        v = cpv(
            frand()
                * (640 as libc::c_int as libc::c_double
                    - 2 as libc::c_int as libc::c_double * radius)
                - (320 as libc::c_int as libc::c_double - radius),
            frand()
                * (480 as libc::c_int as libc::c_double
                    - 2 as libc::c_int as libc::c_double * radius)
                - (240 as libc::c_int as libc::c_double - radius),
        );
        if !(cpvlength(v) < 85.0f32 as libc::c_double) {
            break;
        }
    }
    return v;
}
unsafe extern "C" fn add_box(mut space: *mut cpSpace) {
    let size: cpFloat = 10.0f32 as cpFloat;
    let mass: cpFloat = 1.0f32 as cpFloat;
    let mut verts: [cpVect; 4] = [
        cpv(-size, -size),
        cpv(-size, size),
        cpv(size, size),
        cpv(size, -size),
    ];
    let mut radius: cpFloat = cpvlength(cpv(size, size));
    let mut pos: cpVect = rand_pos(radius);
    let mut body: *mut cpBody = cpSpaceAddBody(
        space,
        cpBodyNew(
            mass,
            cpMomentForPoly(
                mass,
                4 as libc::c_int,
                verts.as_mut_ptr(),
                cpvzero,
                0.0f32 as cpFloat,
            ),
        ),
    );
    cpBodySetVelocityUpdateFunc(
        body,
        Some(
            planetGravityVelocityFunc
                as unsafe extern "C" fn(*mut cpBody, cpVect, cpFloat, cpFloat) -> (),
        ),
    );
    cpBodySetPosition(body, pos);
    let mut r: cpFloat = cpvlength(pos);
    let mut v: cpFloat = sqrt(gravityStrength / r) / r;
    cpBodySetVelocity(body, cpvmult(cpvperp(pos), v));
    cpBodySetAngularVelocity(body, v);
    cpBodySetAngle(body, atan2(pos.y, pos.x));
    let mut shape: *mut cpShape = cpSpaceAddShape(
        space,
        cpPolyShapeNew(
            body,
            4 as libc::c_int,
            verts.as_mut_ptr(),
            cpTransformIdentity,
            0.0f64,
        ),
    );
    cpShapeSetElasticity(shape, 0.0f32 as cpFloat);
    cpShapeSetFriction(shape, 0.7f32 as cpFloat);
}
unsafe extern "C" fn init() -> *mut cpSpace {
    let mut space: *mut cpSpace = cpSpaceNew();
    cpSpaceSetIterations(space, 20 as libc::c_int);
    planetBody = cpSpaceAddBody(space, cpBodyNewKinematic());
    cpBodySetAngularVelocity(planetBody, 0.2f32 as cpFloat);
    let mut i: libc::c_int = 0 as libc::c_int;
    while i < 30 as libc::c_int {
        add_box(space);
        i += 1;
        i;
    }
    let mut shape: *mut cpShape = cpSpaceAddShape(
        space,
        cpCircleShapeNew(planetBody, 70.0f32 as cpFloat, cpvzero),
    );
    cpShapeSetElasticity(shape, 1.0f32 as cpFloat);
    cpShapeSetFriction(shape, 1.0f32 as cpFloat);
    cpShapeSetFilter(shape, NOT_GRABBABLE_FILTER);
    return space;
}
unsafe extern "C" fn destroy(mut space: *mut cpSpace) {
    ChipmunkDemoFreeSpaceChildren(space);
    cpSpaceFree(space);
}
pub static mut Planet: ChipmunkDemo = unsafe {
    {
        let mut init = ChipmunkDemo {
            name: b"Planet\0" as *const u8 as *const libc::c_char,
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
