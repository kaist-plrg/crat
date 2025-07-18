use ::libc;
extern "C" {
    pub type cpBody;
    pub type cpShape;
    pub type cpSpace;
    pub type cpArbiter;
    fn sqrt(_: libc::c_double) -> libc::c_double;
    fn cpMomentForBox(m: cpFloat, width: cpFloat, height: cpFloat) -> cpFloat;
    fn cpMomentForCircle(
        m: cpFloat,
        r1: cpFloat,
        r2: cpFloat,
        offset: cpVect,
    ) -> cpFloat;
    fn cpArbiterTotalImpulse(arb: *const cpArbiter) -> cpVect;
    fn cpArbiterGetShapes(
        arb: *const cpArbiter,
        a: *mut *mut cpShape,
        b: *mut *mut cpShape,
    );
    fn cpBodyNew(mass: cpFloat, moment: cpFloat) -> *mut cpBody;
    fn cpBodyNewStatic() -> *mut cpBody;
    fn cpBodySetPosition(body: *mut cpBody, pos: cpVect);
    fn cpBodyEachArbiter(
        body: *mut cpBody,
        func: cpBodyArbiterIteratorFunc,
        data: *mut libc::c_void,
    );
    fn cpShapeGetBB(shape: *const cpShape) -> cpBB;
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
    fn cpSpaceGetGravity(space: *const cpSpace) -> cpVect;
    fn cpSpaceSetGravity(space: *mut cpSpace, gravity: cpVect);
    fn cpSpaceSetSleepTimeThreshold(space: *mut cpSpace, sleepTimeThreshold: cpFloat);
    fn cpSpaceSetCollisionSlop(space: *mut cpSpace, collisionSlop: cpFloat);
    fn cpSpaceGetStaticBody(space: *const cpSpace) -> *mut cpBody;
    fn cpSpaceAddShape(space: *mut cpSpace, shape: *mut cpShape) -> *mut cpShape;
    fn cpSpaceAddBody(space: *mut cpSpace, body: *mut cpBody) -> *mut cpBody;
    fn cpSpaceStep(space: *mut cpSpace, dt: cpFloat);
    fn ChipmunkDemoPrintString(fmt: *const libc::c_char, _: ...);
    static mut NOT_GRABBABLE_FILTER: cpShapeFilter;
    fn ChipmunkDemoDefaultDrawImpl(space: *mut cpSpace);
    fn ChipmunkDemoFreeSpaceChildren(space: *mut cpSpace);
    fn ChipmunkDebugDrawBB(bb: cpBB, outlineColor: cpSpaceDebugColor);
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
pub type cpBodyArbiterIteratorFunc = Option::<
    unsafe extern "C" fn(*mut cpBody, *mut cpArbiter, *mut libc::c_void) -> (),
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
pub struct CrushingContext {
    pub magnitudeSum: cpFloat,
    pub vectorSum: cpVect,
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
unsafe extern "C" fn cpvdot(v1: cpVect, v2: cpVect) -> cpFloat {
    return v1.x * v2.x + v1.y * v2.y;
}
#[inline]
unsafe extern "C" fn cpvlengthsq(v: cpVect) -> cpFloat {
    return cpvdot(v, v);
}
#[inline]
unsafe extern "C" fn cpvlength(v: cpVect) -> cpFloat {
    return sqrt(cpvdot(v, v));
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
static mut scaleStaticBody: *mut cpBody = 0 as *const cpBody as *mut cpBody;
static mut ballBody: *mut cpBody = 0 as *const cpBody as *mut cpBody;
unsafe extern "C" fn ScaleIterator(
    mut body: *mut cpBody,
    mut arb: *mut cpArbiter,
    mut sum: *mut cpVect,
) {
    *sum = cpvadd(*sum, cpArbiterTotalImpulse(arb));
}
unsafe extern "C" fn BallIterator(
    mut body: *mut cpBody,
    mut arb: *mut cpArbiter,
    mut count: *mut libc::c_int,
) {
    let mut ball: *mut cpShape = 0 as *mut cpShape;
    let mut other: *mut cpShape = 0 as *mut cpShape;
    cpArbiterGetShapes(arb, &mut ball, &mut other);
    ChipmunkDebugDrawBB(
        cpShapeGetBB(other),
        RGBAColor(
            1 as libc::c_int as libc::c_float,
            0 as libc::c_int as libc::c_float,
            0 as libc::c_int as libc::c_float,
            1 as libc::c_int as libc::c_float,
        ),
    );
    *count += 1;
    *count;
}
unsafe extern "C" fn EstimateCrushing(
    mut body: *mut cpBody,
    mut arb: *mut cpArbiter,
    mut context: *mut CrushingContext,
) {
    let mut j: cpVect = cpArbiterTotalImpulse(arb);
    (*context).magnitudeSum += cpvlength(j);
    (*context).vectorSum = cpvadd((*context).vectorSum, j);
}
unsafe extern "C" fn update(mut space: *mut cpSpace, mut dt: libc::c_double) {
    cpSpaceStep(space, dt);
    ChipmunkDemoPrintString(
        b"Place objects on the scale to weigh them. The ball marks the shapes it's sitting on.\n\0"
            as *const u8 as *const libc::c_char,
    );
    let mut impulseSum: cpVect = cpvzero;
    cpBodyEachArbiter(
        scaleStaticBody,
        ::std::mem::transmute::<
            Option::<
                unsafe extern "C" fn(*mut cpBody, *mut cpArbiter, *mut cpVect) -> (),
            >,
            cpBodyArbiterIteratorFunc,
        >(
            Some(
                ScaleIterator
                    as unsafe extern "C" fn(
                        *mut cpBody,
                        *mut cpArbiter,
                        *mut cpVect,
                    ) -> (),
            ),
        ),
        &mut impulseSum as *mut cpVect as *mut libc::c_void,
    );
    let mut force: cpFloat = cpvlength(impulseSum) / dt;
    let mut g: cpVect = cpSpaceGetGravity(space);
    let mut weight: cpFloat = cpvdot(g, impulseSum) / (cpvlengthsq(g) * dt);
    ChipmunkDemoPrintString(
        b"Total force: %5.2f, Total weight: %5.2f. \0" as *const u8
            as *const libc::c_char,
        force,
        weight,
    );
    let mut count: libc::c_int = 0 as libc::c_int;
    cpBodyEachArbiter(
        ballBody,
        ::std::mem::transmute::<
            Option::<
                unsafe extern "C" fn(*mut cpBody, *mut cpArbiter, *mut libc::c_int) -> (),
            >,
            cpBodyArbiterIteratorFunc,
        >(
            Some(
                BallIterator
                    as unsafe extern "C" fn(
                        *mut cpBody,
                        *mut cpArbiter,
                        *mut libc::c_int,
                    ) -> (),
            ),
        ),
        &mut count as *mut libc::c_int as *mut libc::c_void,
    );
    ChipmunkDemoPrintString(
        b"The ball is touching %d shapes.\n\0" as *const u8 as *const libc::c_char,
        count,
    );
    let mut crush: CrushingContext = {
        let mut init = CrushingContext {
            magnitudeSum: 0.0f32 as cpFloat,
            vectorSum: cpvzero,
        };
        init
    };
    cpBodyEachArbiter(
        ballBody,
        ::std::mem::transmute::<
            Option::<
                unsafe extern "C" fn(
                    *mut cpBody,
                    *mut cpArbiter,
                    *mut CrushingContext,
                ) -> (),
            >,
            cpBodyArbiterIteratorFunc,
        >(
            Some(
                EstimateCrushing
                    as unsafe extern "C" fn(
                        *mut cpBody,
                        *mut cpArbiter,
                        *mut CrushingContext,
                    ) -> (),
            ),
        ),
        &mut crush as *mut CrushingContext as *mut libc::c_void,
    );
    let mut crushForce: cpFloat = (crush.magnitudeSum - cpvlength(crush.vectorSum)) * dt;
    if crushForce > 10.0f32 as libc::c_double {
        ChipmunkDemoPrintString(
            b"The ball is being crushed. (f: %.2f)\0" as *const u8
                as *const libc::c_char,
            crushForce,
        );
    } else {
        ChipmunkDemoPrintString(
            b"The ball is not being crushed. (f: %.2f)\0" as *const u8
                as *const libc::c_char,
            crushForce,
        );
    };
}
unsafe extern "C" fn init() -> *mut cpSpace {
    let mut space: *mut cpSpace = cpSpaceNew();
    cpSpaceSetIterations(space, 30 as libc::c_int);
    cpSpaceSetGravity(
        space,
        cpv(0 as libc::c_int as cpFloat, -(300 as libc::c_int) as cpFloat),
    );
    cpSpaceSetCollisionSlop(space, 0.5f64);
    cpSpaceSetSleepTimeThreshold(space, 1.0f32 as cpFloat);
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
    scaleStaticBody = cpSpaceAddBody(space, cpBodyNewStatic());
    shape = cpSpaceAddShape(
        space,
        cpSegmentShapeNew(
            scaleStaticBody,
            cpv(-(240 as libc::c_int) as cpFloat, -(180 as libc::c_int) as cpFloat),
            cpv(-(140 as libc::c_int) as cpFloat, -(180 as libc::c_int) as cpFloat),
            4.0f32 as cpFloat,
        ),
    );
    cpShapeSetElasticity(shape, 1.0f32 as cpFloat);
    cpShapeSetFriction(shape, 1.0f32 as cpFloat);
    cpShapeSetFilter(shape, NOT_GRABBABLE_FILTER);
    let mut i: libc::c_int = 0 as libc::c_int;
    while i < 5 as libc::c_int {
        body = cpSpaceAddBody(
            space,
            cpBodyNew(
                1.0f32 as cpFloat,
                cpMomentForBox(1.0f32 as cpFloat, 30.0f32 as cpFloat, 30.0f32 as cpFloat),
            ),
        );
        cpBodySetPosition(
            body,
            cpv(
                0 as libc::c_int as cpFloat,
                (i * 32 as libc::c_int - 220 as libc::c_int) as cpFloat,
            ),
        );
        shape = cpSpaceAddShape(
            space,
            cpBoxShapeNew(body, 30.0f32 as cpFloat, 30.0f32 as cpFloat, 0.0f64),
        );
        cpShapeSetElasticity(shape, 0.0f32 as cpFloat);
        cpShapeSetFriction(shape, 0.8f32 as cpFloat);
        i += 1;
        i;
    }
    let mut radius: cpFloat = 15.0f32 as cpFloat;
    ballBody = cpSpaceAddBody(
        space,
        cpBodyNew(
            10.0f32 as cpFloat,
            cpMomentForCircle(10.0f32 as cpFloat, 0.0f32 as cpFloat, radius, cpvzero),
        ),
    );
    cpBodySetPosition(
        ballBody,
        cpv(
            120 as libc::c_int as cpFloat,
            -(240 as libc::c_int) as libc::c_double + radius
                + 5 as libc::c_int as libc::c_double,
        ),
    );
    shape = cpSpaceAddShape(space, cpCircleShapeNew(ballBody, radius, cpvzero));
    cpShapeSetElasticity(shape, 0.0f32 as cpFloat);
    cpShapeSetFriction(shape, 0.9f32 as cpFloat);
    return space;
}
unsafe extern "C" fn destroy(mut space: *mut cpSpace) {
    ChipmunkDemoFreeSpaceChildren(space);
    cpSpaceFree(space);
}
pub static mut ContactGraph: ChipmunkDemo = unsafe {
    {
        let mut init = ChipmunkDemo {
            name: b"Contact Graph\0" as *const u8 as *const libc::c_char,
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
