use ::libc;
extern "C" {
    pub type cpBody;
    pub type cpShape;
    pub type cpConstraint;
    pub type cpSpace;
    pub type cpArbiter;
    fn rand() -> libc::c_int;
    fn cpMomentForCircle(
        m: cpFloat,
        r1: cpFloat,
        r2: cpFloat,
        offset: cpVect,
    ) -> cpFloat;
    fn cpArbiterGetUserData(arb: *const cpArbiter) -> cpDataPointer;
    fn cpArbiterSetUserData(arb: *mut cpArbiter, userData: cpDataPointer);
    fn cpArbiterGetBodies(
        arb: *const cpArbiter,
        a: *mut *mut cpBody,
        b: *mut *mut cpBody,
    );
    fn cpArbiterGetContactPointSet(arb: *const cpArbiter) -> cpContactPointSet;
    fn cpArbiterSetContactPointSet(arb: *mut cpArbiter, set: *mut cpContactPointSet);
    fn cpBodyNew(mass: cpFloat, moment: cpFloat) -> *mut cpBody;
    fn cpBodySetPosition(body: *mut cpBody, pos: cpVect);
    fn cpBodyWorldToLocal(body: *const cpBody, point: cpVect) -> cpVect;
    fn cpShapeSetElasticity(shape: *mut cpShape, elasticity: cpFloat);
    fn cpShapeSetFriction(shape: *mut cpShape, friction: cpFloat);
    fn cpShapeSetCollisionType(shape: *mut cpShape, collisionType: cpCollisionType);
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
    fn cpConstraintFree(constraint: *mut cpConstraint);
    fn cpConstraintSetMaxForce(constraint: *mut cpConstraint, maxForce: cpFloat);
    fn cpPivotJointNew2(
        a: *mut cpBody,
        b: *mut cpBody,
        anchorA: cpVect,
        anchorB: cpVect,
    ) -> *mut cpConstraint;
    fn cpSpaceNew() -> *mut cpSpace;
    fn cpSpaceFree(space: *mut cpSpace);
    fn cpSpaceSetIterations(space: *mut cpSpace, iterations: libc::c_int);
    fn cpSpaceSetGravity(space: *mut cpSpace, gravity: cpVect);
    fn cpSpaceSetCollisionSlop(space: *mut cpSpace, collisionSlop: cpFloat);
    fn cpSpaceGetStaticBody(space: *const cpSpace) -> *mut cpBody;
    fn cpSpaceAddWildcardHandler(
        space: *mut cpSpace,
        type_0: cpCollisionType,
    ) -> *mut cpCollisionHandler;
    fn cpSpaceAddShape(space: *mut cpSpace, shape: *mut cpShape) -> *mut cpShape;
    fn cpSpaceAddBody(space: *mut cpSpace, body: *mut cpBody) -> *mut cpBody;
    fn cpSpaceAddConstraint(
        space: *mut cpSpace,
        constraint: *mut cpConstraint,
    ) -> *mut cpConstraint;
    fn cpSpaceRemoveConstraint(space: *mut cpSpace, constraint: *mut cpConstraint);
    fn cpSpaceAddPostStepCallback(
        space: *mut cpSpace,
        func: cpPostStepFunc,
        key: *mut libc::c_void,
        data: *mut libc::c_void,
    ) -> cpBool;
    fn cpSpaceStep(space: *mut cpSpace, dt: cpFloat);
    static mut ChipmunkDemoMessageString: *const libc::c_char;
    static mut NOT_GRABBABLE_FILTER: cpShapeFilter;
    fn ChipmunkDemoDefaultDrawImpl(space: *mut cpSpace);
    fn ChipmunkDemoFreeSpaceChildren(space: *mut cpSpace);
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
pub struct cpContactPointSet {
    pub count: libc::c_int,
    pub normal: cpVect,
    pub points: [C2RustUnnamed; 2],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed {
    pub pointA: cpVect,
    pub pointB: cpVect,
    pub distance: cpFloat,
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
pub type C2RustUnnamed_0 = libc::c_uint;
pub const COLLISION_TYPE_STICKY: C2RustUnnamed_0 = 1;
#[inline]
unsafe extern "C" fn cpfmin(mut a: cpFloat, mut b: cpFloat) -> cpFloat {
    return if a < b { a } else { b };
}
#[inline]
unsafe extern "C" fn cpflerp(
    mut f1: cpFloat,
    mut f2: cpFloat,
    mut t: cpFloat,
) -> cpFloat {
    return f1 * (1.0f32 as libc::c_double - t) + f2 * t;
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
unsafe extern "C" fn cpvsub(v1: cpVect, v2: cpVect) -> cpVect {
    return cpv(v1.x - v2.x, v1.y - v2.y);
}
#[inline]
unsafe extern "C" fn cpvmult(v: cpVect, s: cpFloat) -> cpVect {
    return cpv(v.x * s, v.y * s);
}
#[inline]
unsafe extern "C" fn frand() -> cpFloat {
    return rand() as cpFloat / 2147483647 as libc::c_int as cpFloat;
}
unsafe extern "C" fn PostStepAddJoint(
    mut space: *mut cpSpace,
    mut key: *mut libc::c_void,
    mut data: *mut libc::c_void,
) {
    let mut joint: *mut cpConstraint = key as *mut cpConstraint;
    cpSpaceAddConstraint(space, joint);
}
unsafe extern "C" fn StickyPreSolve(
    mut arb: *mut cpArbiter,
    mut space: *mut cpSpace,
    mut data: *mut libc::c_void,
) -> cpBool {
    let mut deepest: cpFloat = ::std::f32::INFINITY as cpFloat;
    let mut contacts: cpContactPointSet = cpArbiterGetContactPointSet(arb);
    let mut i: libc::c_int = 0 as libc::c_int;
    while i < contacts.count {
        contacts
            .points[i as usize]
            .pointA = cpvsub(
            contacts.points[i as usize].pointA,
            cpvmult(contacts.normal, 2.5f32 as cpFloat),
        );
        contacts
            .points[i as usize]
            .pointB = cpvadd(
            contacts.points[i as usize].pointB,
            cpvmult(contacts.normal, 2.5f32 as cpFloat),
        );
        deepest = cpfmin(deepest, contacts.points[i as usize].distance);
        i += 1;
        i;
    }
    cpArbiterSetContactPointSet(arb, &mut contacts);
    if (cpArbiterGetUserData(arb)).is_null() && deepest <= 0.0f32 as libc::c_double {
        let mut bodyA: *mut cpBody = 0 as *mut cpBody;
        let mut bodyB: *mut cpBody = 0 as *mut cpBody;
        cpArbiterGetBodies(arb, &mut bodyA, &mut bodyB);
        let mut anchorA: cpVect = cpBodyWorldToLocal(
            bodyA,
            contacts.points[0 as libc::c_int as usize].pointA,
        );
        let mut anchorB: cpVect = cpBodyWorldToLocal(
            bodyB,
            contacts.points[0 as libc::c_int as usize].pointB,
        );
        let mut joint: *mut cpConstraint = cpPivotJointNew2(
            bodyA,
            bodyB,
            anchorA,
            anchorB,
        );
        cpConstraintSetMaxForce(joint, 3e3f64);
        cpSpaceAddPostStepCallback(
            space,
            Some(
                PostStepAddJoint
                    as unsafe extern "C" fn(
                        *mut cpSpace,
                        *mut libc::c_void,
                        *mut libc::c_void,
                    ) -> (),
            ),
            joint as *mut libc::c_void,
            0 as *mut libc::c_void,
        );
        cpArbiterSetUserData(arb, joint as cpDataPointer);
    }
    return (deepest <= 0.0f32 as libc::c_double) as libc::c_int as cpBool;
}
unsafe extern "C" fn PostStepRemoveJoint(
    mut space: *mut cpSpace,
    mut key: *mut libc::c_void,
    mut data: *mut libc::c_void,
) {
    let mut joint: *mut cpConstraint = key as *mut cpConstraint;
    cpSpaceRemoveConstraint(space, joint);
    cpConstraintFree(joint);
}
unsafe extern "C" fn StickySeparate(
    mut arb: *mut cpArbiter,
    mut space: *mut cpSpace,
    mut data: *mut libc::c_void,
) {
    let mut joint: *mut cpConstraint = cpArbiterGetUserData(arb) as *mut cpConstraint;
    if !joint.is_null() {
        cpConstraintSetMaxForce(joint, 0.0f32 as cpFloat);
        cpSpaceAddPostStepCallback(
            space,
            Some(
                PostStepRemoveJoint
                    as unsafe extern "C" fn(
                        *mut cpSpace,
                        *mut libc::c_void,
                        *mut libc::c_void,
                    ) -> (),
            ),
            joint as *mut libc::c_void,
            0 as *mut libc::c_void,
        );
        cpArbiterSetUserData(arb, 0 as *mut libc::c_void);
    }
}
unsafe extern "C" fn update(mut space: *mut cpSpace, mut dt: libc::c_double) {
    cpSpaceStep(space, dt);
}
unsafe extern "C" fn init() -> *mut cpSpace {
    ChipmunkDemoMessageString = b"Sticky collisions using the cpArbiter data pointer.\0"
        as *const u8 as *const libc::c_char;
    let mut space: *mut cpSpace = cpSpaceNew();
    cpSpaceSetIterations(space, 10 as libc::c_int);
    cpSpaceSetGravity(
        space,
        cpv(0 as libc::c_int as cpFloat, -(1000 as libc::c_int) as cpFloat),
    );
    cpSpaceSetCollisionSlop(space, 2.0f64);
    let mut staticBody: *mut cpBody = cpSpaceGetStaticBody(space);
    let mut shape: *mut cpShape = 0 as *mut cpShape;
    shape = cpSpaceAddShape(
        space,
        cpSegmentShapeNew(
            staticBody,
            cpv(-(340 as libc::c_int) as cpFloat, -(260 as libc::c_int) as cpFloat),
            cpv(-(340 as libc::c_int) as cpFloat, 260 as libc::c_int as cpFloat),
            20.0f32 as cpFloat,
        ),
    );
    cpShapeSetElasticity(shape, 1.0f32 as cpFloat);
    cpShapeSetFriction(shape, 1.0f32 as cpFloat);
    cpShapeSetFilter(shape, NOT_GRABBABLE_FILTER);
    shape = cpSpaceAddShape(
        space,
        cpSegmentShapeNew(
            staticBody,
            cpv(340 as libc::c_int as cpFloat, -(260 as libc::c_int) as cpFloat),
            cpv(340 as libc::c_int as cpFloat, 260 as libc::c_int as cpFloat),
            20.0f32 as cpFloat,
        ),
    );
    cpShapeSetElasticity(shape, 1.0f32 as cpFloat);
    cpShapeSetFriction(shape, 1.0f32 as cpFloat);
    cpShapeSetFilter(shape, NOT_GRABBABLE_FILTER);
    shape = cpSpaceAddShape(
        space,
        cpSegmentShapeNew(
            staticBody,
            cpv(-(340 as libc::c_int) as cpFloat, -(260 as libc::c_int) as cpFloat),
            cpv(340 as libc::c_int as cpFloat, -(260 as libc::c_int) as cpFloat),
            20.0f32 as cpFloat,
        ),
    );
    cpShapeSetElasticity(shape, 1.0f32 as cpFloat);
    cpShapeSetFriction(shape, 1.0f32 as cpFloat);
    cpShapeSetFilter(shape, NOT_GRABBABLE_FILTER);
    shape = cpSpaceAddShape(
        space,
        cpSegmentShapeNew(
            staticBody,
            cpv(-(340 as libc::c_int) as cpFloat, 260 as libc::c_int as cpFloat),
            cpv(340 as libc::c_int as cpFloat, 260 as libc::c_int as cpFloat),
            20.0f32 as cpFloat,
        ),
    );
    cpShapeSetElasticity(shape, 1.0f32 as cpFloat);
    cpShapeSetFriction(shape, 1.0f32 as cpFloat);
    cpShapeSetFilter(shape, NOT_GRABBABLE_FILTER);
    let mut i: libc::c_int = 0 as libc::c_int;
    while i < 200 as libc::c_int {
        let mut mass: cpFloat = 0.15f32 as cpFloat;
        let mut radius: cpFloat = 10.0f32 as cpFloat;
        let mut body: *mut cpBody = cpSpaceAddBody(
            space,
            cpBodyNew(mass, cpMomentForCircle(mass, 0.0f32 as cpFloat, radius, cpvzero)),
        );
        cpBodySetPosition(
            body,
            cpv(
                cpflerp(-150.0f32 as cpFloat, 150.0f32 as cpFloat, frand()),
                cpflerp(-150.0f32 as cpFloat, 150.0f32 as cpFloat, frand()),
            ),
        );
        let mut shape_0: *mut cpShape = cpSpaceAddShape(
            space,
            cpCircleShapeNew(body, radius + 2.5f32 as libc::c_double, cpvzero),
        );
        cpShapeSetFriction(shape_0, 0.9f32 as cpFloat);
        cpShapeSetCollisionType(
            shape_0,
            COLLISION_TYPE_STICKY as libc::c_int as cpCollisionType,
        );
        i += 1;
        i;
    }
    let mut handler: *mut cpCollisionHandler = cpSpaceAddWildcardHandler(
        space,
        COLLISION_TYPE_STICKY as libc::c_int as cpCollisionType,
    );
    (*handler)
        .preSolveFunc = Some(
        StickyPreSolve
            as unsafe extern "C" fn(
                *mut cpArbiter,
                *mut cpSpace,
                *mut libc::c_void,
            ) -> cpBool,
    );
    (*handler)
        .separateFunc = Some(
        StickySeparate
            as unsafe extern "C" fn(
                *mut cpArbiter,
                *mut cpSpace,
                *mut libc::c_void,
            ) -> (),
    );
    return space;
}
unsafe extern "C" fn destroy(mut space: *mut cpSpace) {
    ChipmunkDemoFreeSpaceChildren(space);
    cpSpaceFree(space);
}
pub static mut Sticky: ChipmunkDemo = unsafe {
    {
        let mut init = ChipmunkDemo {
            name: b"Sticky Surfaces\0" as *const u8 as *const libc::c_char,
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
