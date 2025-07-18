use ::libc;
extern "C" {
    pub type cpBody;
    pub type cpShape;
    pub type cpSpace;
    fn cpConvexHull(
        count: libc::c_int,
        verts: *const cpVect,
        result: *mut cpVect,
        first: *mut libc::c_int,
        tol: cpFloat,
    ) -> libc::c_int;
    fn cpMomentForBox(m: cpFloat, width: cpFloat, height: cpFloat) -> cpFloat;
    fn cpCentroidForPoly(count: libc::c_int, verts: *const cpVect) -> cpVect;
    fn cpAreaForPoly(
        count: libc::c_int,
        verts: *const cpVect,
        radius: cpFloat,
    ) -> cpFloat;
    fn cpMomentForPoly(
        m: cpFloat,
        count: libc::c_int,
        verts: *const cpVect,
        offset: cpVect,
        radius: cpFloat,
    ) -> cpFloat;
    fn cpBodyNew(mass: cpFloat, moment: cpFloat) -> *mut cpBody;
    fn cpBodySetMass(body: *mut cpBody, m: cpFloat);
    fn cpBodySetMoment(body: *mut cpBody, i: cpFloat);
    fn cpBodySetPosition(body: *mut cpBody, pos: cpVect);
    fn cpBodyLocalToWorld(body: *const cpBody, point: cpVect) -> cpVect;
    fn cpBodyWorldToLocal(body: *const cpBody, point: cpVect) -> cpVect;
    fn cpShapePointQuery(
        shape_0: *const cpShape,
        p: cpVect,
        out: *mut cpPointQueryInfo,
    ) -> cpFloat;
    fn cpShapeGetBody(shape_0: *const cpShape) -> *mut cpBody;
    fn cpShapeSetElasticity(shape_0: *mut cpShape, elasticity: cpFloat);
    fn cpShapeSetFriction(shape_0: *mut cpShape, friction: cpFloat);
    fn cpShapeSetFilter(shape_0: *mut cpShape, filter: cpShapeFilter);
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
    fn cpPolyShapeGetCount(shape_0: *const cpShape) -> libc::c_int;
    fn cpPolyShapeGetVert(shape_0: *const cpShape, index: libc::c_int) -> cpVect;
    fn cpSpaceNew() -> *mut cpSpace;
    fn cpSpaceFree(space: *mut cpSpace);
    fn cpSpaceSetIterations(space: *mut cpSpace, iterations: libc::c_int);
    fn cpSpaceSetGravity(space: *mut cpSpace, gravity: cpVect);
    fn cpSpaceSetSleepTimeThreshold(space: *mut cpSpace, sleepTimeThreshold: cpFloat);
    fn cpSpaceSetCollisionSlop(space: *mut cpSpace, collisionSlop: cpFloat);
    fn cpSpaceGetStaticBody(space: *const cpSpace) -> *mut cpBody;
    fn cpSpaceAddShape(space: *mut cpSpace, shape_0: *mut cpShape) -> *mut cpShape;
    fn cpSpaceAddBody(space: *mut cpSpace, body: *mut cpBody) -> *mut cpBody;
    fn cpSpaceStep(space: *mut cpSpace, dt: cpFloat);
    fn cpPolyShapeSetVerts(
        shape_0: *mut cpShape,
        count: libc::c_int,
        verts: *mut cpVect,
        transform: cpTransform,
    );
    static mut ChipmunkDemoRightClick: cpBool;
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
unsafe extern "C" fn cpvneg(v: cpVect) -> cpVect {
    return cpv(-v.x, -v.y);
}
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
static mut shape: *mut cpShape = 0 as *const cpShape as *mut cpShape;
unsafe extern "C" fn update(mut space: *mut cpSpace, mut dt: libc::c_double) {
    let mut tolerance: cpFloat = 2.0f64;
    if ChipmunkDemoRightClick as libc::c_int != 0
        && cpShapePointQuery(shape, ChipmunkDemoMouse, 0 as *mut cpPointQueryInfo)
            > tolerance
    {
        let mut body: *mut cpBody = cpShapeGetBody(shape);
        let mut count: libc::c_int = cpPolyShapeGetCount(shape);
        let mut fresh0 = ::std::vec::from_elem(
            0,
            ((count + 1 as libc::c_int) as libc::c_ulong)
                .wrapping_mul(::std::mem::size_of::<cpVect>() as libc::c_ulong) as usize,
        );
        let mut verts: *mut cpVect = fresh0.leak().as_mut_ptr() as *mut cpVect;
        let mut i: libc::c_int = 0 as libc::c_int;
        while i < count {
            *verts.offset(i as isize) = cpPolyShapeGetVert(shape, i);
            i += 1;
            i;
        }
        *verts.offset(count as isize) = cpBodyWorldToLocal(body, ChipmunkDemoMouse);
        let mut hullCount: libc::c_int = cpConvexHull(
            count + 1 as libc::c_int,
            verts,
            verts,
            0 as *mut libc::c_int,
            tolerance,
        );
        let mut centroid: cpVect = cpCentroidForPoly(hullCount, verts);
        let mut mass: cpFloat = cpAreaForPoly(hullCount, verts, 0.0f32 as cpFloat)
            * (1.0f64 / 10000.0f64);
        cpBodySetMass(body, mass);
        cpBodySetMoment(
            body,
            cpMomentForPoly(mass, hullCount, verts, cpvneg(centroid), 0.0f32 as cpFloat),
        );
        cpBodySetPosition(body, cpBodyLocalToWorld(body, centroid));
        cpPolyShapeSetVerts(
            shape,
            hullCount,
            verts,
            cpTransformTranslate(cpvneg(centroid)),
        );
    }
    cpSpaceStep(space, dt);
}
unsafe extern "C" fn init() -> *mut cpSpace {
    ChipmunkDemoMessageString = b"Right click and drag to change the blocks's shape.\0"
        as *const u8 as *const libc::c_char;
    let mut space: *mut cpSpace = cpSpaceNew();
    cpSpaceSetIterations(space, 30 as libc::c_int);
    cpSpaceSetGravity(
        space,
        cpv(0 as libc::c_int as cpFloat, -(500 as libc::c_int) as cpFloat),
    );
    cpSpaceSetSleepTimeThreshold(space, 0.5f32 as cpFloat);
    cpSpaceSetCollisionSlop(space, 0.5f32 as cpFloat);
    let mut body: *mut cpBody = 0 as *mut cpBody;
    let mut staticBody: *mut cpBody = cpSpaceGetStaticBody(space);
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
    let mut width: cpFloat = 50.0f32 as cpFloat;
    let mut height: cpFloat = 70.0f32 as cpFloat;
    let mut mass: cpFloat = width * height * (1.0f64 / 10000.0f64);
    let mut moment: cpFloat = cpMomentForBox(mass, width, height);
    body = cpSpaceAddBody(space, cpBodyNew(mass, moment));
    shape = cpSpaceAddShape(space, cpBoxShapeNew(body, width, height, 0.0f64));
    cpShapeSetFriction(shape, 0.6f32 as cpFloat);
    return space;
}
unsafe extern "C" fn destroy(mut space: *mut cpSpace) {
    ChipmunkDemoFreeSpaceChildren(space);
    cpSpaceFree(space);
}
pub static mut Convex: ChipmunkDemo = unsafe {
    {
        let mut init = ChipmunkDemo {
            name: b"Convex.\0" as *const u8 as *const libc::c_char,
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
