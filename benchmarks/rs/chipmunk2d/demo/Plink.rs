use ::libc;
extern "C" {
    pub type cpBody;
    pub type cpShape;
    pub type cpSpace;
    fn cos(_: libc::c_double) -> libc::c_double;
    fn sin(_: libc::c_double) -> libc::c_double;
    fn rand() -> libc::c_int;
    fn cpMomentForPoly(
        m: cpFloat,
        count: libc::c_int,
        verts: *const cpVect,
        offset: cpVect,
        radius: cpFloat,
    ) -> cpFloat;
    fn cpBodyNew(mass: cpFloat, moment: cpFloat) -> *mut cpBody;
    fn cpBodyGetType(body: *mut cpBody) -> cpBodyType;
    fn cpBodySetType(body: *mut cpBody, type_0: cpBodyType);
    fn cpBodySetMass(body: *mut cpBody, m: cpFloat);
    fn cpBodySetMoment(body: *mut cpBody, i: cpFloat);
    fn cpBodyGetPosition(body: *const cpBody) -> cpVect;
    fn cpBodySetPosition(body: *mut cpBody, pos: cpVect);
    fn cpShapeGetBody(shape: *const cpShape) -> *mut cpBody;
    fn cpShapeSetElasticity(shape: *mut cpShape, elasticity: cpFloat);
    fn cpShapeSetFriction(shape: *mut cpShape, friction: cpFloat);
    fn cpShapeSetFilter(shape: *mut cpShape, filter: cpShapeFilter);
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
    fn cpSpaceSetGravity(space: *mut cpSpace, gravity: cpVect);
    fn cpSpaceGetStaticBody(space: *const cpSpace) -> *mut cpBody;
    fn cpSpaceAddShape(space: *mut cpSpace, shape: *mut cpShape) -> *mut cpShape;
    fn cpSpaceAddBody(space: *mut cpSpace, body: *mut cpBody) -> *mut cpBody;
    fn cpSpacePointQueryNearest(
        space: *mut cpSpace,
        point: cpVect,
        maxDistance: cpFloat,
        filter: cpShapeFilter,
        out: *mut cpPointQueryInfo,
    ) -> *mut cpShape;
    fn cpSpaceEachBody(
        space: *mut cpSpace,
        func: cpSpaceBodyIteratorFunc,
        data: *mut libc::c_void,
    );
    fn cpSpaceStep(space: *mut cpSpace, dt: cpFloat);
    static mut ChipmunkDemoMouse: cpVect;
    fn ChipmunkDemoFreeSpaceChildren(space: *mut cpSpace);
    fn ChipmunkDemoDefaultDrawImpl(space: *mut cpSpace);
    static mut NOT_GRABBABLE_FILTER: cpShapeFilter;
    static mut GRAB_FILTER: cpShapeFilter;
    static mut ChipmunkDemoMessageString: *const libc::c_char;
    static mut ChipmunkDemoRightDown: cpBool;
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
pub struct cpTransform {
    pub a: cpFloat,
    pub b: cpFloat,
    pub c: cpFloat,
    pub d: cpFloat,
    pub tx: cpFloat,
    pub ty: cpFloat,
}
pub type cpBodyType = libc::c_uint;
pub const CP_BODY_TYPE_STATIC: cpBodyType = 2;
pub const CP_BODY_TYPE_KINEMATIC: cpBodyType = 1;
pub const CP_BODY_TYPE_DYNAMIC: cpBodyType = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct cpPointQueryInfo {
    pub shape: *const cpShape,
    pub point: cpVect,
    pub distance: cpFloat,
    pub gradient: cpVect,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct cpShapeFilter {
    pub group: cpGroup,
    pub categories: cpBitmask,
    pub mask: cpBitmask,
}
pub type cpSpaceBodyIteratorFunc = Option::<
    unsafe extern "C" fn(*mut cpBody, *mut libc::c_void) -> (),
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
#[inline]
unsafe extern "C" fn cpTransformNewTranspose(
    mut a: cpFloat,
    mut c: cpFloat,
    mut tx: cpFloat,
    mut b: cpFloat,
    mut d: cpFloat,
    mut ty: cpFloat,
) -> cpTransform {
    let mut t: cpTransform = {
        let mut init = cpTransform {
            a: a,
            b: b,
            c: c,
            d: d,
            tx: tx,
            ty: ty,
        };
        init
    };
    return t;
}
#[inline]
unsafe extern "C" fn cpfabs(mut f: cpFloat) -> cpFloat {
    return if f < 0 as libc::c_int as libc::c_double { -f } else { f };
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
unsafe extern "C" fn cpTransformTranslate(mut translate: cpVect) -> cpTransform {
    return cpTransformNewTranspose(
        1.0f64,
        0.0f64,
        translate.x,
        0.0f64,
        1.0f64,
        translate.y,
    );
}
static mut pentagon_mass: cpFloat = 0.0f32 as cpFloat;
static mut pentagon_moment: cpFloat = 0.0f32 as cpFloat;
unsafe extern "C" fn eachBody(mut body: *mut cpBody, mut unused: *mut libc::c_void) {
    let mut pos: cpVect = cpBodyGetPosition(body);
    if pos.y < -(260 as libc::c_int) as libc::c_double
        || cpfabs(pos.x) > 340 as libc::c_int as libc::c_double
    {
        let mut x: cpFloat = rand() as libc::c_double
            / 2147483647 as libc::c_int as cpFloat * 640 as libc::c_int as libc::c_double
            - 320 as libc::c_int as libc::c_double;
        cpBodySetPosition(body, cpv(x, 260 as libc::c_int as cpFloat));
    }
}
unsafe extern "C" fn update(mut space: *mut cpSpace, mut dt: libc::c_double) {
    if ChipmunkDemoRightDown != 0 {
        let mut nearest: *mut cpShape = cpSpacePointQueryNearest(
            space,
            ChipmunkDemoMouse,
            0.0f64,
            GRAB_FILTER,
            0 as *mut cpPointQueryInfo,
        );
        if !nearest.is_null() {
            let mut body: *mut cpBody = cpShapeGetBody(nearest);
            if cpBodyGetType(body) as libc::c_uint
                == CP_BODY_TYPE_STATIC as libc::c_int as libc::c_uint
            {
                cpBodySetType(body, CP_BODY_TYPE_DYNAMIC);
                cpBodySetMass(body, pentagon_mass);
                cpBodySetMoment(body, pentagon_moment);
            } else if cpBodyGetType(body) as libc::c_uint
                == CP_BODY_TYPE_DYNAMIC as libc::c_int as libc::c_uint
            {
                cpBodySetType(body, CP_BODY_TYPE_STATIC);
            }
        }
    }
    cpSpaceEachBody(
        space,
        Some(eachBody as unsafe extern "C" fn(*mut cpBody, *mut libc::c_void) -> ()),
        0 as *mut libc::c_void,
    );
    cpSpaceStep(space, dt);
}
unsafe extern "C" fn init() -> *mut cpSpace {
    ChipmunkDemoMessageString = b"Right click to make pentagons static/dynamic.\0"
        as *const u8 as *const libc::c_char;
    let mut space: *mut cpSpace = cpSpaceNew();
    cpSpaceSetIterations(space, 5 as libc::c_int);
    cpSpaceSetGravity(
        space,
        cpv(0 as libc::c_int as cpFloat, -(100 as libc::c_int) as cpFloat),
    );
    let mut body: *mut cpBody = 0 as *mut cpBody;
    let mut staticBody: *mut cpBody = cpSpaceGetStaticBody(space);
    let mut shape: *mut cpShape = 0 as *mut cpShape;
    let mut tris: [cpVect; 4] = [
        cpv(-(15 as libc::c_int) as cpFloat, -(15 as libc::c_int) as cpFloat),
        cpv(0 as libc::c_int as cpFloat, 10 as libc::c_int as cpFloat),
        cpv(15 as libc::c_int as cpFloat, -(15 as libc::c_int) as cpFloat),
        cpVect { x: 0., y: 0. },
    ];
    let mut i: libc::c_int = 0 as libc::c_int;
    while i < 9 as libc::c_int {
        let mut j: libc::c_int = 0 as libc::c_int;
        while j < 6 as libc::c_int {
            let mut stagger: cpFloat = (j % 2 as libc::c_int * 40 as libc::c_int)
                as cpFloat;
            let mut offset: cpVect = cpv(
                (i * 80 as libc::c_int - 320 as libc::c_int) as libc::c_double + stagger,
                (j * 70 as libc::c_int - 240 as libc::c_int) as cpFloat,
            );
            shape = cpSpaceAddShape(
                space,
                cpPolyShapeNew(
                    staticBody,
                    3 as libc::c_int,
                    tris.as_mut_ptr(),
                    cpTransformTranslate(offset),
                    0.0f64,
                ),
            );
            cpShapeSetElasticity(shape, 1.0f32 as cpFloat);
            cpShapeSetFriction(shape, 1.0f32 as cpFloat);
            cpShapeSetFilter(shape, NOT_GRABBABLE_FILTER);
            j += 1;
            j;
        }
        i += 1;
        i;
    }
    let mut verts: [cpVect; 5] = [cpVect { x: 0., y: 0. }; 5];
    let mut i_0: libc::c_int = 0 as libc::c_int;
    while i_0 < 5 as libc::c_int {
        let mut angle: cpFloat = -2.0f32 as libc::c_double
            * 3.14159265358979323846264338327950288f64 * i_0 as libc::c_double
            / 5 as libc::c_int as cpFloat;
        verts[i_0
            as usize] = cpv(
            10 as libc::c_int as libc::c_double * cos(angle),
            10 as libc::c_int as libc::c_double * sin(angle),
        );
        i_0 += 1;
        i_0;
    }
    pentagon_mass = 1.0f64;
    pentagon_moment = cpMomentForPoly(
        1.0f32 as cpFloat,
        5 as libc::c_int,
        verts.as_mut_ptr(),
        cpvzero,
        0.0f32 as cpFloat,
    );
    let mut i_1: libc::c_int = 0 as libc::c_int;
    while i_1 < 300 as libc::c_int {
        body = cpSpaceAddBody(space, cpBodyNew(pentagon_mass, pentagon_moment));
        let mut x: cpFloat = rand() as libc::c_double
            / 2147483647 as libc::c_int as cpFloat * 640 as libc::c_int as libc::c_double
            - 320 as libc::c_int as libc::c_double;
        cpBodySetPosition(body, cpv(x, 350 as libc::c_int as cpFloat));
        shape = cpSpaceAddShape(
            space,
            cpPolyShapeNew(
                body,
                5 as libc::c_int,
                verts.as_mut_ptr(),
                cpTransformIdentity,
                0.0f64,
            ),
        );
        cpShapeSetElasticity(shape, 0.0f32 as cpFloat);
        cpShapeSetFriction(shape, 0.4f32 as cpFloat);
        i_1 += 1;
        i_1;
    }
    return space;
}
unsafe extern "C" fn destroy(mut space: *mut cpSpace) {
    ChipmunkDemoFreeSpaceChildren(space);
    cpSpaceFree(space);
}
pub static mut Plink: ChipmunkDemo = unsafe {
    {
        let mut init = ChipmunkDemo {
            name: b"Plink\0" as *const u8 as *const libc::c_char,
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
