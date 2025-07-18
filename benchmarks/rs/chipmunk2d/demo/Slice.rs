use ::libc;
extern "C" {
    pub type cpBody;
    pub type cpShape;
    pub type cpSpace;
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
    fn cpBodyNew(mass: cpFloat, moment: cpFloat) -> *mut cpBody;
    fn cpBodyFree(body: *mut cpBody);
    fn cpBodySetPosition(body: *mut cpBody, pos: cpVect);
    fn cpBodySetVelocity(body: *mut cpBody, velocity: cpVect);
    fn cpBodyGetAngularVelocity(body: *const cpBody) -> cpFloat;
    fn cpBodySetAngularVelocity(body: *mut cpBody, angularVelocity: cpFloat);
    fn cpBodyLocalToWorld(body: *const cpBody, point: cpVect) -> cpVect;
    fn cpBodyGetVelocityAtWorldPoint(body: *const cpBody, point: cpVect) -> cpVect;
    fn cpShapeFree(shape: *mut cpShape);
    fn cpShapePointQuery(
        shape: *const cpShape,
        p: cpVect,
        out: *mut cpPointQueryInfo,
    ) -> cpFloat;
    fn cpShapeGetBody(shape: *const cpShape) -> *mut cpBody;
    fn cpShapeSetElasticity(shape: *mut cpShape, elasticity: cpFloat);
    fn cpShapeGetFriction(shape: *const cpShape) -> cpFloat;
    fn cpShapeSetFriction(shape: *mut cpShape, friction: cpFloat);
    fn cpShapeSetFilter(shape: *mut cpShape, filter: cpShapeFilter);
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
    fn cpBoxShapeNew(
        body: *mut cpBody,
        width: cpFloat,
        height: cpFloat,
        radius: cpFloat,
    ) -> *mut cpShape;
    fn cpPolyShapeGetCount(shape: *const cpShape) -> libc::c_int;
    fn cpPolyShapeGetVert(shape: *const cpShape, index: libc::c_int) -> cpVect;
    fn cpSpaceNew() -> *mut cpSpace;
    fn cpSpaceFree(space: *mut cpSpace);
    fn cpSpaceSetIterations(space: *mut cpSpace, iterations: libc::c_int);
    fn cpSpaceSetGravity(space: *mut cpSpace, gravity: cpVect);
    fn cpSpaceSetSleepTimeThreshold(space: *mut cpSpace, sleepTimeThreshold: cpFloat);
    fn cpSpaceSetCollisionSlop(space: *mut cpSpace, collisionSlop: cpFloat);
    fn cpSpaceGetStaticBody(space: *const cpSpace) -> *mut cpBody;
    fn cpSpaceAddShape(space: *mut cpSpace, shape: *mut cpShape) -> *mut cpShape;
    fn cpSpaceAddBody(space: *mut cpSpace, body: *mut cpBody) -> *mut cpBody;
    fn cpSpaceRemoveShape(space: *mut cpSpace, shape: *mut cpShape);
    fn cpSpaceRemoveBody(space: *mut cpSpace, body: *mut cpBody);
    fn cpSpaceAddPostStepCallback(
        space: *mut cpSpace,
        func: cpPostStepFunc,
        key: *mut libc::c_void,
        data: *mut libc::c_void,
    ) -> cpBool;
    fn cpSpaceSegmentQuery(
        space: *mut cpSpace,
        start: cpVect,
        end: cpVect,
        radius: cpFloat,
        filter: cpShapeFilter,
        func: cpSpaceSegmentQueryFunc,
        data: *mut libc::c_void,
    );
    fn cpSpaceStep(space: *mut cpSpace, dt: cpFloat);
    static mut ChipmunkDemoMouse: cpVect;
    static mut ChipmunkDemoRightClick: cpBool;
    static mut ChipmunkDemoMessageString: *const libc::c_char;
    fn ChipmunkDemoFreeSpaceChildren(space: *mut cpSpace);
    fn ChipmunkDemoDefaultDrawImpl(space: *mut cpSpace);
    static mut NOT_GRABBABLE_FILTER: cpShapeFilter;
    static mut GRAB_FILTER: cpShapeFilter;
    fn ChipmunkDebugDrawSegment(a: cpVect, b: cpVect, color: cpSpaceDebugColor);
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
pub type cpPostStepFunc = Option::<
    unsafe extern "C" fn(*mut cpSpace, *mut libc::c_void, *mut libc::c_void) -> (),
>;
pub type cpSpaceSegmentQueryFunc = Option::<
    unsafe extern "C" fn(*mut cpShape, cpVect, cpVect, cpFloat, *mut libc::c_void) -> (),
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct SliceContext {
    pub a: cpVect,
    pub b: cpVect,
    pub space: *mut cpSpace,
}
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
unsafe extern "C" fn cpvperp(v: cpVect) -> cpVect {
    return cpv(-v.y, v.x);
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
unsafe extern "C" fn ClipPoly(
    mut space: *mut cpSpace,
    mut shape: *mut cpShape,
    mut n: cpVect,
    mut dist: cpFloat,
) {
    let mut body: *mut cpBody = cpShapeGetBody(shape);
    let mut count: libc::c_int = cpPolyShapeGetCount(shape);
    let mut clippedCount: libc::c_int = 0 as libc::c_int;
    let mut fresh0 = ::std::vec::from_elem(
        0,
        ((count + 1 as libc::c_int) as libc::c_ulong)
            .wrapping_mul(::std::mem::size_of::<cpVect>() as libc::c_ulong) as usize,
    );
    let mut clipped: *mut cpVect = fresh0.leak().as_mut_ptr() as *mut cpVect;
    let mut i: libc::c_int = 0 as libc::c_int;
    let mut j: libc::c_int = count - 1 as libc::c_int;
    while i < count {
        let mut a: cpVect = cpBodyLocalToWorld(body, cpPolyShapeGetVert(shape, j));
        let mut a_dist: cpFloat = cpvdot(a, n) - dist;
        if a_dist < 0.0f64 {
            *clipped.offset(clippedCount as isize) = a;
            clippedCount += 1;
            clippedCount;
        }
        let mut b: cpVect = cpBodyLocalToWorld(body, cpPolyShapeGetVert(shape, i));
        let mut b_dist: cpFloat = cpvdot(b, n) - dist;
        if a_dist * b_dist < 0.0f32 as libc::c_double {
            let mut t: cpFloat = cpfabs(a_dist) / (cpfabs(a_dist) + cpfabs(b_dist));
            *clipped.offset(clippedCount as isize) = cpvlerp(a, b, t);
            clippedCount += 1;
            clippedCount;
        }
        j = i;
        i += 1;
        i;
    }
    let mut centroid: cpVect = cpCentroidForPoly(clippedCount, clipped);
    let mut mass: cpFloat = cpAreaForPoly(clippedCount, clipped, 0.0f32 as cpFloat)
        * (1.0f64 / 10000.0f64);
    let mut moment: cpFloat = cpMomentForPoly(
        mass,
        clippedCount,
        clipped,
        cpvneg(centroid),
        0.0f32 as cpFloat,
    );
    let mut new_body: *mut cpBody = cpSpaceAddBody(space, cpBodyNew(mass, moment));
    cpBodySetPosition(new_body, centroid);
    cpBodySetVelocity(new_body, cpBodyGetVelocityAtWorldPoint(body, centroid));
    cpBodySetAngularVelocity(new_body, cpBodyGetAngularVelocity(body));
    let mut transform: cpTransform = cpTransformTranslate(cpvneg(centroid));
    let mut new_shape: *mut cpShape = cpSpaceAddShape(
        space,
        cpPolyShapeNew(new_body, clippedCount, clipped, transform, 0.0f64),
    );
    cpShapeSetFriction(new_shape, cpShapeGetFriction(shape));
}
unsafe extern "C" fn SliceShapePostStep(
    mut space: *mut cpSpace,
    mut shape: *mut cpShape,
    mut context: *mut SliceContext,
) {
    let mut a: cpVect = (*context).a;
    let mut b: cpVect = (*context).b;
    let mut n: cpVect = cpvnormalize(cpvperp(cpvsub(b, a)));
    let mut dist: cpFloat = cpvdot(a, n);
    ClipPoly(space, shape, n, dist);
    ClipPoly(space, shape, cpvneg(n), -dist);
    let mut body: *mut cpBody = cpShapeGetBody(shape);
    cpSpaceRemoveShape(space, shape);
    cpSpaceRemoveBody(space, body);
    cpShapeFree(shape);
    cpBodyFree(body);
}
unsafe extern "C" fn SliceQuery(
    mut shape: *mut cpShape,
    mut point: cpVect,
    mut normal: cpVect,
    mut alpha: cpFloat,
    mut context: *mut SliceContext,
) {
    let mut a: cpVect = (*context).a;
    let mut b: cpVect = (*context).b;
    if cpShapePointQuery(shape, a, 0 as *mut cpPointQueryInfo) > 0.0f32 as libc::c_double
        && cpShapePointQuery(shape, b, 0 as *mut cpPointQueryInfo)
            > 0.0f32 as libc::c_double
    {
        cpSpaceAddPostStepCallback(
            (*context).space,
            ::std::mem::transmute::<
                Option::<
                    unsafe extern "C" fn(
                        *mut cpSpace,
                        *mut cpShape,
                        *mut SliceContext,
                    ) -> (),
                >,
                cpPostStepFunc,
            >(
                Some(
                    SliceShapePostStep
                        as unsafe extern "C" fn(
                            *mut cpSpace,
                            *mut cpShape,
                            *mut SliceContext,
                        ) -> (),
                ),
            ),
            shape as *mut libc::c_void,
            context as *mut libc::c_void,
        );
    }
}
unsafe extern "C" fn update(mut space: *mut cpSpace, mut dt: libc::c_double) {
    cpSpaceStep(space, dt);
    static mut lastClickState: cpBool = 0 as libc::c_int as cpBool;
    static mut sliceStart: cpVect = {
        let mut init = cpVect { x: 0.0f64, y: 0.0f64 };
        init
    };
    if ChipmunkDemoRightClick as libc::c_int != lastClickState as libc::c_int {
        if ChipmunkDemoRightClick != 0 {
            sliceStart = ChipmunkDemoMouse;
        } else {
            let mut context: SliceContext = {
                let mut init = SliceContext {
                    a: sliceStart,
                    b: ChipmunkDemoMouse,
                    space: space,
                };
                init
            };
            cpSpaceSegmentQuery(
                space,
                sliceStart,
                ChipmunkDemoMouse,
                0.0f64,
                GRAB_FILTER,
                ::std::mem::transmute::<
                    Option::<
                        unsafe extern "C" fn(
                            *mut cpShape,
                            cpVect,
                            cpVect,
                            cpFloat,
                            *mut SliceContext,
                        ) -> (),
                    >,
                    cpSpaceSegmentQueryFunc,
                >(
                    Some(
                        SliceQuery
                            as unsafe extern "C" fn(
                                *mut cpShape,
                                cpVect,
                                cpVect,
                                cpFloat,
                                *mut SliceContext,
                            ) -> (),
                    ),
                ),
                &mut context as *mut SliceContext as *mut libc::c_void,
            );
        }
        lastClickState = ChipmunkDemoRightClick;
    }
    if ChipmunkDemoRightClick != 0 {
        ChipmunkDebugDrawSegment(
            sliceStart,
            ChipmunkDemoMouse,
            RGBAColor(
                1 as libc::c_int as libc::c_float,
                0 as libc::c_int as libc::c_float,
                0 as libc::c_int as libc::c_float,
                1 as libc::c_int as libc::c_float,
            ),
        );
    }
}
unsafe extern "C" fn init() -> *mut cpSpace {
    ChipmunkDemoMessageString = b"Right click and drag to slice up the block.\0"
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
    let mut shape: *mut cpShape = 0 as *mut cpShape;
    shape = cpSpaceAddShape(
        space,
        cpSegmentShapeNew(
            staticBody,
            cpv(-(1000 as libc::c_int) as cpFloat, -(240 as libc::c_int) as cpFloat),
            cpv(1000 as libc::c_int as cpFloat, -(240 as libc::c_int) as cpFloat),
            0.0f32 as cpFloat,
        ),
    );
    cpShapeSetElasticity(shape, 1.0f32 as cpFloat);
    cpShapeSetFriction(shape, 1.0f32 as cpFloat);
    cpShapeSetFilter(shape, NOT_GRABBABLE_FILTER);
    let mut width: cpFloat = 200.0f32 as cpFloat;
    let mut height: cpFloat = 300.0f32 as cpFloat;
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
pub static mut Slice: ChipmunkDemo = unsafe {
    {
        let mut init = ChipmunkDemo {
            name: b"Slice.\0" as *const u8 as *const libc::c_char,
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
