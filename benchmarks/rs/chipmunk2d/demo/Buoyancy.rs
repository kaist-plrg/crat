use ::libc;
extern "C" {
    pub type cpBody;
    pub type cpShape;
    pub type cpSpace;
    pub type cpArbiter;
    fn exp(_: libc::c_double) -> libc::c_double;
    fn sqrt(_: libc::c_double) -> libc::c_double;
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
    fn cpArbiterGetShapes(
        arb: *const cpArbiter,
        a: *mut *mut cpShape,
        b: *mut *mut cpShape,
    );
    fn cpBodyNew(mass: cpFloat, moment: cpFloat) -> *mut cpBody;
    fn cpBodyGetMass(body: *const cpBody) -> cpFloat;
    fn cpBodyGetMoment(body: *const cpBody) -> cpFloat;
    fn cpBodyGetPosition(body: *const cpBody) -> cpVect;
    fn cpBodySetPosition(body: *mut cpBody, pos: cpVect);
    fn cpBodyGetCenterOfGravity(body: *const cpBody) -> cpVect;
    fn cpBodySetVelocity(body: *mut cpBody, velocity: cpVect);
    fn cpBodyGetAngularVelocity(body: *const cpBody) -> cpFloat;
    fn cpBodySetAngularVelocity(body: *mut cpBody, angularVelocity: cpFloat);
    fn cpBodyLocalToWorld(body: *const cpBody, point: cpVect) -> cpVect;
    fn cpBodyApplyImpulseAtWorldPoint(body: *mut cpBody, impulse: cpVect, point: cpVect);
    fn cpBodyGetVelocityAtWorldPoint(body: *const cpBody, point: cpVect) -> cpVect;
    fn cpShapeGetBody(shape: *const cpShape) -> *mut cpBody;
    fn cpShapeGetBB(shape: *const cpShape) -> cpBB;
    fn cpShapeSetSensor(shape: *mut cpShape, sensor: cpBool);
    fn cpShapeSetElasticity(shape: *mut cpShape, elasticity: cpFloat);
    fn cpShapeSetFriction(shape: *mut cpShape, friction: cpFloat);
    fn cpShapeSetCollisionType(shape: *mut cpShape, collisionType: cpCollisionType);
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
    fn cpBoxShapeNew2(body: *mut cpBody, box_0: cpBB, radius: cpFloat) -> *mut cpShape;
    fn cpPolyShapeGetCount(shape: *const cpShape) -> libc::c_int;
    fn cpPolyShapeGetVert(shape: *const cpShape, index: libc::c_int) -> cpVect;
    fn cpSpaceNew() -> *mut cpSpace;
    fn cpSpaceFree(space: *mut cpSpace);
    fn cpSpaceSetIterations(space: *mut cpSpace, iterations: libc::c_int);
    fn cpSpaceGetGravity(space: *const cpSpace) -> cpVect;
    fn cpSpaceSetGravity(space: *mut cpSpace, gravity: cpVect);
    fn cpSpaceSetSleepTimeThreshold(space: *mut cpSpace, sleepTimeThreshold: cpFloat);
    fn cpSpaceSetCollisionSlop(space: *mut cpSpace, collisionSlop: cpFloat);
    fn cpSpaceGetStaticBody(space: *const cpSpace) -> *mut cpBody;
    fn cpSpaceGetCurrentTimeStep(space: *const cpSpace) -> cpFloat;
    fn cpSpaceAddCollisionHandler(
        space: *mut cpSpace,
        a: cpCollisionType,
        b: cpCollisionType,
    ) -> *mut cpCollisionHandler;
    fn cpSpaceAddShape(space: *mut cpSpace, shape: *mut cpShape) -> *mut cpShape;
    fn cpSpaceAddBody(space: *mut cpSpace, body: *mut cpBody) -> *mut cpBody;
    fn cpSpaceStep(space: *mut cpSpace, dt: cpFloat);
    fn ChipmunkDemoFreeSpaceChildren(space: *mut cpSpace);
    fn ChipmunkDebugDrawPolygon(
        count: libc::c_int,
        verts: *const cpVect,
        radius: cpFloat,
        outlineColor: cpSpaceDebugColor,
        fillColor: cpSpaceDebugColor,
    );
    static mut ChipmunkDemoMessageString: *const libc::c_char;
    fn ChipmunkDemoDefaultDrawImpl(space: *mut cpSpace);
    fn ChipmunkDebugDrawDot(size: cpFloat, pos: cpVect, fillColor: cpSpaceDebugColor);
    static mut NOT_GRABBABLE_FILTER: cpShapeFilter;
}
pub type uintptr_t = libc::c_ulong;
pub type cpFloat = libc::c_double;
pub type cpBool = libc::c_uchar;
pub type cpDataPointer = *mut libc::c_void;
pub type cpCollisionType = uintptr_t;
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
pub struct cpCollisionHandler {
    pub typeA: cpCollisionType,
    pub typeB: cpCollisionType,
    pub beginFunc: cpCollisionBeginFunc,
    pub preSolveFunc: cpCollisionPreSolveFunc,
    pub postSolveFunc: cpCollisionPostSolveFunc,
    pub separateFunc: cpCollisionSeparateFunc,
    pub userData: cpDataPointer,
}
pub type cpCollisionSeparateFunc = Option::<
    unsafe extern "C" fn(*mut cpArbiter, *mut cpSpace, cpDataPointer) -> (),
>;
pub type cpCollisionPostSolveFunc = Option::<
    unsafe extern "C" fn(*mut cpArbiter, *mut cpSpace, cpDataPointer) -> (),
>;
pub type cpCollisionPreSolveFunc = Option::<
    unsafe extern "C" fn(*mut cpArbiter, *mut cpSpace, cpDataPointer) -> cpBool,
>;
pub type cpCollisionBeginFunc = Option::<
    unsafe extern "C" fn(*mut cpArbiter, *mut cpSpace, cpDataPointer) -> cpBool,
>;
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
unsafe extern "C" fn cpfabs(mut f: cpFloat) -> cpFloat {
    return if f < 0 as libc::c_int as libc::c_double { -f } else { f };
}
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
unsafe extern "C" fn cpvsub(v1: cpVect, v2: cpVect) -> cpVect {
    return cpv(v1.x - v2.x, v1.y - v2.y);
}
#[inline]
unsafe extern "C" fn cpvneg(v: cpVect) -> cpVect {
    return cpv(-v.x, -v.y);
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
unsafe extern "C" fn cpvcross(v1: cpVect, v2: cpVect) -> cpFloat {
    return v1.x * v2.y - v1.y * v2.x;
}
#[inline]
unsafe extern "C" fn cpvlength(v: cpVect) -> cpFloat {
    return sqrt(cpvdot(v, v));
}
#[inline]
unsafe extern "C" fn cpvlerp(v1: cpVect, v2: cpVect, t: cpFloat) -> cpVect {
    return cpvadd(cpvmult(v1, 1.0f32 as libc::c_double - t), cpvmult(v2, t));
}
#[inline]
unsafe extern "C" fn cpvnormalize(v: cpVect) -> cpVect {
    return cpvmult(
        v,
        1.0f32 as libc::c_double / (cpvlength(v) + 2.2250738585072014e-308f64),
    );
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
unsafe extern "C" fn update(mut space: *mut cpSpace, mut dt: libc::c_double) {
    cpSpaceStep(space, dt);
}
pub static mut messageBuffer: [libc::c_char; 1024] = [0; 1024];
#[inline]
unsafe extern "C" fn k_scalar_body(
    mut body: *mut cpBody,
    mut point: cpVect,
    mut n: cpVect,
) -> cpFloat {
    let mut rcn: cpFloat = cpvcross(cpvsub(point, cpBodyGetPosition(body)), n);
    return 1.0f32 as libc::c_double / cpBodyGetMass(body)
        + rcn * rcn / cpBodyGetMoment(body);
}
unsafe extern "C" fn waterPreSolve(
    mut arb: *mut cpArbiter,
    mut space: *mut cpSpace,
    mut ptr: *mut libc::c_void,
) -> cpBool {
    let mut water: *mut cpShape = 0 as *mut cpShape;
    let mut poly: *mut cpShape = 0 as *mut cpShape;
    cpArbiterGetShapes(arb, &mut water, &mut poly);
    let mut body: *mut cpBody = cpShapeGetBody(poly);
    let mut level: cpFloat = (cpShapeGetBB(water)).t;
    let mut count: libc::c_int = cpPolyShapeGetCount(poly);
    let mut clippedCount: libc::c_int = 0 as libc::c_int;
    let vla = (count + 1 as libc::c_int) as usize;
    let mut clipped: Vec::<cpVect> = ::std::vec::from_elem(cpVect { x: 0., y: 0. }, vla);
    let mut i: libc::c_int = 0 as libc::c_int;
    let mut j: libc::c_int = count - 1 as libc::c_int;
    while i < count {
        let mut a: cpVect = cpBodyLocalToWorld(body, cpPolyShapeGetVert(poly, j));
        let mut b: cpVect = cpBodyLocalToWorld(body, cpPolyShapeGetVert(poly, i));
        if a.y < level {
            *clipped.as_mut_ptr().offset(clippedCount as isize) = a;
            clippedCount += 1;
            clippedCount;
        }
        let mut a_level: cpFloat = a.y - level;
        let mut b_level: cpFloat = b.y - level;
        if a_level * b_level < 0.0f32 as libc::c_double {
            let mut t: cpFloat = cpfabs(a_level) / (cpfabs(a_level) + cpfabs(b_level));
            *clipped.as_mut_ptr().offset(clippedCount as isize) = cpvlerp(a, b, t);
            clippedCount += 1;
            clippedCount;
        }
        j = i;
        i += 1;
        i;
    }
    let mut clippedArea: cpFloat = cpAreaForPoly(
        clippedCount,
        clipped.as_mut_ptr(),
        0.0f32 as cpFloat,
    );
    let mut displacedMass: cpFloat = clippedArea * 0.00014f64;
    let mut centroid: cpVect = cpCentroidForPoly(clippedCount, clipped.as_mut_ptr());
    ChipmunkDebugDrawPolygon(
        clippedCount,
        clipped.as_mut_ptr(),
        5.0f32 as cpFloat,
        RGBAColor(
            0 as libc::c_int as libc::c_float,
            0 as libc::c_int as libc::c_float,
            1 as libc::c_int as libc::c_float,
            1 as libc::c_int as libc::c_float,
        ),
        RGBAColor(
            0 as libc::c_int as libc::c_float,
            0 as libc::c_int as libc::c_float,
            1 as libc::c_int as libc::c_float,
            0.1f32,
        ),
    );
    ChipmunkDebugDrawDot(
        5 as libc::c_int as cpFloat,
        centroid,
        RGBAColor(
            0 as libc::c_int as libc::c_float,
            0 as libc::c_int as libc::c_float,
            1 as libc::c_int as libc::c_float,
            1 as libc::c_int as libc::c_float,
        ),
    );
    let mut dt: cpFloat = cpSpaceGetCurrentTimeStep(space);
    let mut g: cpVect = cpSpaceGetGravity(space);
    cpBodyApplyImpulseAtWorldPoint(body, cpvmult(g, -displacedMass * dt), centroid);
    let mut v_centroid: cpVect = cpBodyGetVelocityAtWorldPoint(body, centroid);
    let mut k: cpFloat = k_scalar_body(body, centroid, cpvnormalize(v_centroid));
    let mut damping: cpFloat = clippedArea * 2.0f64 * 0.00014f64;
    let mut v_coef: cpFloat = exp(-damping * dt * k);
    cpBodyApplyImpulseAtWorldPoint(
        body,
        cpvmult(cpvsub(cpvmult(v_centroid, v_coef), v_centroid), 1.0f64 / k),
        centroid,
    );
    let mut cog: cpVect = cpBodyLocalToWorld(body, cpBodyGetCenterOfGravity(body));
    let mut w_damping: cpFloat = cpMomentForPoly(
        2.0f64 * 0.00014f64 * clippedArea,
        clippedCount,
        clipped.as_mut_ptr(),
        cpvneg(cog),
        0.0f32 as cpFloat,
    );
    cpBodySetAngularVelocity(
        body,
        cpBodyGetAngularVelocity(body) * exp(-w_damping * dt / cpBodyGetMoment(body)),
    );
    return 1 as libc::c_int as cpBool;
}
unsafe extern "C" fn init() -> *mut cpSpace {
    ChipmunkDemoMessageString = messageBuffer.as_mut_ptr();
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
    let mut bb: cpBB = cpBBNew(
        -(300 as libc::c_int) as cpFloat,
        -(200 as libc::c_int) as cpFloat,
        100 as libc::c_int as cpFloat,
        0 as libc::c_int as cpFloat,
    );
    let mut radius: cpFloat = 5.0f32 as cpFloat;
    shape = cpSpaceAddShape(
        space,
        cpSegmentShapeNew(staticBody, cpv(bb.l, bb.b), cpv(bb.l, bb.t), radius),
    );
    cpShapeSetElasticity(shape, 1.0f32 as cpFloat);
    cpShapeSetFriction(shape, 1.0f32 as cpFloat);
    cpShapeSetFilter(shape, NOT_GRABBABLE_FILTER);
    shape = cpSpaceAddShape(
        space,
        cpSegmentShapeNew(staticBody, cpv(bb.r, bb.b), cpv(bb.r, bb.t), radius),
    );
    cpShapeSetElasticity(shape, 1.0f32 as cpFloat);
    cpShapeSetFriction(shape, 1.0f32 as cpFloat);
    cpShapeSetFilter(shape, NOT_GRABBABLE_FILTER);
    shape = cpSpaceAddShape(
        space,
        cpSegmentShapeNew(staticBody, cpv(bb.l, bb.b), cpv(bb.r, bb.b), radius),
    );
    cpShapeSetElasticity(shape, 1.0f32 as cpFloat);
    cpShapeSetFriction(shape, 1.0f32 as cpFloat);
    cpShapeSetFilter(shape, NOT_GRABBABLE_FILTER);
    shape = cpSpaceAddShape(space, cpBoxShapeNew2(staticBody, bb, 0.0f64));
    cpShapeSetSensor(shape, 1 as libc::c_int as cpBool);
    cpShapeSetCollisionType(shape, 1 as libc::c_int as cpCollisionType);
    let mut width: cpFloat = 200.0f32 as cpFloat;
    let mut height: cpFloat = 50.0f32 as cpFloat;
    let mut mass: cpFloat = 0.3f64 * 0.00014f64 * width * height;
    let mut moment: cpFloat = cpMomentForBox(mass, width, height);
    body = cpSpaceAddBody(space, cpBodyNew(mass, moment));
    cpBodySetPosition(
        body,
        cpv(-(50 as libc::c_int) as cpFloat, -(100 as libc::c_int) as cpFloat),
    );
    cpBodySetVelocity(
        body,
        cpv(0 as libc::c_int as cpFloat, -(100 as libc::c_int) as cpFloat),
    );
    cpBodySetAngularVelocity(body, 1 as libc::c_int as cpFloat);
    shape = cpSpaceAddShape(space, cpBoxShapeNew(body, width, height, 0.0f64));
    cpShapeSetFriction(shape, 0.8f32 as cpFloat);
    let mut width_0: cpFloat = 40.0f32 as cpFloat;
    let mut height_0: cpFloat = width_0 * 2 as libc::c_int as libc::c_double;
    let mut mass_0: cpFloat = 0.3f64 * 0.00014f64 * width_0 * height_0;
    let mut moment_0: cpFloat = cpMomentForBox(mass_0, width_0, height_0);
    body = cpSpaceAddBody(space, cpBodyNew(mass_0, moment_0));
    cpBodySetPosition(
        body,
        cpv(-(200 as libc::c_int) as cpFloat, -(50 as libc::c_int) as cpFloat),
    );
    cpBodySetVelocity(
        body,
        cpv(0 as libc::c_int as cpFloat, -(100 as libc::c_int) as cpFloat),
    );
    cpBodySetAngularVelocity(body, 1 as libc::c_int as cpFloat);
    shape = cpSpaceAddShape(space, cpBoxShapeNew(body, width_0, height_0, 0.0f64));
    cpShapeSetFriction(shape, 0.8f32 as cpFloat);
    let mut handler: *mut cpCollisionHandler = cpSpaceAddCollisionHandler(
        space,
        1 as libc::c_int as cpCollisionType,
        0 as libc::c_int as cpCollisionType,
    );
    (*handler)
        .preSolveFunc = ::std::mem::transmute::<
        Option::<
            unsafe extern "C" fn(
                *mut cpArbiter,
                *mut cpSpace,
                *mut libc::c_void,
            ) -> cpBool,
        >,
        cpCollisionPreSolveFunc,
    >(
        Some(
            waterPreSolve
                as unsafe extern "C" fn(
                    *mut cpArbiter,
                    *mut cpSpace,
                    *mut libc::c_void,
                ) -> cpBool,
        ),
    );
    return space;
}
unsafe extern "C" fn destroy(mut space: *mut cpSpace) {
    ChipmunkDemoFreeSpaceChildren(space);
    cpSpaceFree(space);
}
pub static mut Buoyancy: ChipmunkDemo = unsafe {
    {
        let mut init = ChipmunkDemo {
            name: b"Simple Sensor based fluids.\0" as *const u8 as *const libc::c_char,
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
