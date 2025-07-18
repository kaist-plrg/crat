use ::libc;
extern "C" {
    pub type cpBody;
    pub type cpShape;
    pub type cpConstraint;
    pub type cpSpace;
    fn sqrt(_: libc::c_double) -> libc::c_double;
    fn cpBodyNew(mass: cpFloat, moment: cpFloat) -> *mut cpBody;
    fn cpBodySetPosition(body: *mut cpBody, pos: cpVect);
    fn cpShapeSetFilter(shape: *mut cpShape, filter: cpShapeFilter);
    fn cpSegmentShapeNew(
        body: *mut cpBody,
        a: cpVect,
        b: cpVect,
        radius: cpFloat,
    ) -> *mut cpShape;
    fn cpPivotJointNew2(
        a: *mut cpBody,
        b: *mut cpBody,
        anchorA: cpVect,
        anchorB: cpVect,
    ) -> *mut cpConstraint;
    fn cpDampedSpringNew(
        a: *mut cpBody,
        b: *mut cpBody,
        anchorA: cpVect,
        anchorB: cpVect,
        restLength: cpFloat,
        stiffness: cpFloat,
        damping: cpFloat,
    ) -> *mut cpConstraint;
    fn cpDampedSpringGetRestLength(constraint: *const cpConstraint) -> cpFloat;
    fn cpDampedSpringGetStiffness(constraint: *const cpConstraint) -> cpFloat;
    fn cpDampedSpringSetSpringForceFunc(
        constraint: *mut cpConstraint,
        springForceFunc: cpDampedSpringForceFunc,
    );
    fn cpSpaceNew() -> *mut cpSpace;
    fn cpSpaceFree(space: *mut cpSpace);
    fn cpSpaceGetStaticBody(space: *const cpSpace) -> *mut cpBody;
    fn cpSpaceAddShape(space: *mut cpSpace, shape: *mut cpShape) -> *mut cpShape;
    fn cpSpaceAddBody(space: *mut cpSpace, body: *mut cpBody) -> *mut cpBody;
    fn cpSpaceAddConstraint(
        space: *mut cpSpace,
        constraint: *mut cpConstraint,
    ) -> *mut cpConstraint;
    fn cpSpaceStep(space: *mut cpSpace, dt: cpFloat);
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
pub type cpDampedSpringForceFunc = Option::<
    unsafe extern "C" fn(*mut cpConstraint, cpFloat) -> cpFloat,
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
unsafe extern "C" fn cpfmax(mut a: cpFloat, mut b: cpFloat) -> cpFloat {
    return if a > b { a } else { b };
}
#[inline]
unsafe extern "C" fn cpfmin(mut a: cpFloat, mut b: cpFloat) -> cpFloat {
    return if a < b { a } else { b };
}
#[inline]
unsafe extern "C" fn cpfclamp(
    mut f: cpFloat,
    mut min: cpFloat,
    mut max: cpFloat,
) -> cpFloat {
    return cpfmin(cpfmax(f, min), max);
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
unsafe extern "C" fn cpvmult(v: cpVect, s: cpFloat) -> cpVect {
    return cpv(v.x * s, v.y * s);
}
#[inline]
unsafe extern "C" fn cpvdot(v1: cpVect, v2: cpVect) -> cpFloat {
    return v1.x * v2.x + v1.y * v2.y;
}
#[inline]
unsafe extern "C" fn cpvlength(v: cpVect) -> cpFloat {
    return sqrt(cpvdot(v, v));
}
#[inline]
unsafe extern "C" fn cpShapeFilterNew(
    mut group: cpGroup,
    mut categories: cpBitmask,
    mut mask: cpBitmask,
) -> cpShapeFilter {
    let mut filter: cpShapeFilter = {
        let mut init = cpShapeFilter {
            group: group,
            categories: categories,
            mask: mask,
        };
        init
    };
    return filter;
}
unsafe extern "C" fn springForce(
    mut spring: *mut cpConstraint,
    mut dist: cpFloat,
) -> cpFloat {
    let mut clamp: cpFloat = 20.0f32 as cpFloat;
    return cpfclamp(cpDampedSpringGetRestLength(spring) - dist, -clamp, clamp)
        * cpDampedSpringGetStiffness(spring);
}
unsafe extern "C" fn new_spring(
    mut a: *mut cpBody,
    mut b: *mut cpBody,
    mut anchorA: cpVect,
    mut anchorB: cpVect,
    mut restLength: cpFloat,
    mut stiff: cpFloat,
    mut damp: cpFloat,
) -> *mut cpConstraint {
    let mut spring: *mut cpConstraint = cpDampedSpringNew(
        a,
        b,
        anchorA,
        anchorB,
        restLength,
        stiff,
        damp,
    );
    cpDampedSpringSetSpringForceFunc(
        spring,
        Some(springForce as unsafe extern "C" fn(*mut cpConstraint, cpFloat) -> cpFloat),
    );
    return spring;
}
unsafe extern "C" fn update(mut space: *mut cpSpace, mut dt: libc::c_double) {
    cpSpaceStep(space, dt);
}
unsafe extern "C" fn add_bar(
    mut space: *mut cpSpace,
    mut a: cpVect,
    mut b: cpVect,
    mut group: libc::c_int,
) -> *mut cpBody {
    let mut center: cpVect = cpvmult(cpvadd(a, b), (1.0f32 / 2.0f32) as cpFloat);
    let mut length: cpFloat = cpvlength(cpvsub(b, a));
    let mut mass: cpFloat = length / 160.0f32 as libc::c_double;
    let mut body: *mut cpBody = cpSpaceAddBody(
        space,
        cpBodyNew(mass, mass * length * length / 12.0f32 as libc::c_double),
    );
    cpBodySetPosition(body, center);
    let mut shape: *mut cpShape = cpSpaceAddShape(
        space,
        cpSegmentShapeNew(body, cpvsub(a, center), cpvsub(b, center), 10.0f32 as cpFloat),
    );
    cpShapeSetFilter(
        shape,
        cpShapeFilterNew(
            group as cpGroup,
            !(0 as libc::c_int as cpBitmask),
            !(0 as libc::c_int as cpBitmask),
        ),
    );
    return body;
}
unsafe extern "C" fn init() -> *mut cpSpace {
    let mut space: *mut cpSpace = cpSpaceNew();
    let mut staticBody: *mut cpBody = cpSpaceGetStaticBody(space);
    let mut body1: *mut cpBody = add_bar(
        space,
        cpv(-(240 as libc::c_int) as cpFloat, 160 as libc::c_int as cpFloat),
        cpv(-(160 as libc::c_int) as cpFloat, 80 as libc::c_int as cpFloat),
        1 as libc::c_int,
    );
    let mut body2: *mut cpBody = add_bar(
        space,
        cpv(-(160 as libc::c_int) as cpFloat, 80 as libc::c_int as cpFloat),
        cpv(-(80 as libc::c_int) as cpFloat, 160 as libc::c_int as cpFloat),
        1 as libc::c_int,
    );
    let mut body3: *mut cpBody = add_bar(
        space,
        cpv(0 as libc::c_int as cpFloat, 160 as libc::c_int as cpFloat),
        cpv(80 as libc::c_int as cpFloat, 0 as libc::c_int as cpFloat),
        0 as libc::c_int,
    );
    let mut body4: *mut cpBody = add_bar(
        space,
        cpv(160 as libc::c_int as cpFloat, 160 as libc::c_int as cpFloat),
        cpv(240 as libc::c_int as cpFloat, 160 as libc::c_int as cpFloat),
        0 as libc::c_int,
    );
    let mut body5: *mut cpBody = add_bar(
        space,
        cpv(-(240 as libc::c_int) as cpFloat, 0 as libc::c_int as cpFloat),
        cpv(-(160 as libc::c_int) as cpFloat, -(80 as libc::c_int) as cpFloat),
        2 as libc::c_int,
    );
    let mut body6: *mut cpBody = add_bar(
        space,
        cpv(-(160 as libc::c_int) as cpFloat, -(80 as libc::c_int) as cpFloat),
        cpv(-(80 as libc::c_int) as cpFloat, 0 as libc::c_int as cpFloat),
        2 as libc::c_int,
    );
    let mut body7: *mut cpBody = add_bar(
        space,
        cpv(-(80 as libc::c_int) as cpFloat, 0 as libc::c_int as cpFloat),
        cpv(0 as libc::c_int as cpFloat, 0 as libc::c_int as cpFloat),
        2 as libc::c_int,
    );
    let mut body8: *mut cpBody = add_bar(
        space,
        cpv(0 as libc::c_int as cpFloat, -(80 as libc::c_int) as cpFloat),
        cpv(80 as libc::c_int as cpFloat, -(80 as libc::c_int) as cpFloat),
        0 as libc::c_int,
    );
    let mut body9: *mut cpBody = add_bar(
        space,
        cpv(240 as libc::c_int as cpFloat, 80 as libc::c_int as cpFloat),
        cpv(160 as libc::c_int as cpFloat, 0 as libc::c_int as cpFloat),
        3 as libc::c_int,
    );
    let mut body10: *mut cpBody = add_bar(
        space,
        cpv(160 as libc::c_int as cpFloat, 0 as libc::c_int as cpFloat),
        cpv(240 as libc::c_int as cpFloat, -(80 as libc::c_int) as cpFloat),
        3 as libc::c_int,
    );
    let mut body11: *mut cpBody = add_bar(
        space,
        cpv(-(240 as libc::c_int) as cpFloat, -(80 as libc::c_int) as cpFloat),
        cpv(-(160 as libc::c_int) as cpFloat, -(160 as libc::c_int) as cpFloat),
        4 as libc::c_int,
    );
    let mut body12: *mut cpBody = add_bar(
        space,
        cpv(-(160 as libc::c_int) as cpFloat, -(160 as libc::c_int) as cpFloat),
        cpv(-(80 as libc::c_int) as cpFloat, -(160 as libc::c_int) as cpFloat),
        4 as libc::c_int,
    );
    let mut body13: *mut cpBody = add_bar(
        space,
        cpv(0 as libc::c_int as cpFloat, -(160 as libc::c_int) as cpFloat),
        cpv(80 as libc::c_int as cpFloat, -(160 as libc::c_int) as cpFloat),
        0 as libc::c_int,
    );
    let mut body14: *mut cpBody = add_bar(
        space,
        cpv(160 as libc::c_int as cpFloat, -(160 as libc::c_int) as cpFloat),
        cpv(240 as libc::c_int as cpFloat, -(160 as libc::c_int) as cpFloat),
        0 as libc::c_int,
    );
    cpSpaceAddConstraint(
        space,
        cpPivotJointNew2(
            body1,
            body2,
            cpv(40 as libc::c_int as cpFloat, -(40 as libc::c_int) as cpFloat),
            cpv(-(40 as libc::c_int) as cpFloat, -(40 as libc::c_int) as cpFloat),
        ),
    );
    cpSpaceAddConstraint(
        space,
        cpPivotJointNew2(
            body5,
            body6,
            cpv(40 as libc::c_int as cpFloat, -(40 as libc::c_int) as cpFloat),
            cpv(-(40 as libc::c_int) as cpFloat, -(40 as libc::c_int) as cpFloat),
        ),
    );
    cpSpaceAddConstraint(
        space,
        cpPivotJointNew2(
            body6,
            body7,
            cpv(40 as libc::c_int as cpFloat, 40 as libc::c_int as cpFloat),
            cpv(-(40 as libc::c_int) as cpFloat, 0 as libc::c_int as cpFloat),
        ),
    );
    cpSpaceAddConstraint(
        space,
        cpPivotJointNew2(
            body9,
            body10,
            cpv(-(40 as libc::c_int) as cpFloat, -(40 as libc::c_int) as cpFloat),
            cpv(-(40 as libc::c_int) as cpFloat, 40 as libc::c_int as cpFloat),
        ),
    );
    cpSpaceAddConstraint(
        space,
        cpPivotJointNew2(
            body11,
            body12,
            cpv(40 as libc::c_int as cpFloat, -(40 as libc::c_int) as cpFloat),
            cpv(-(40 as libc::c_int) as cpFloat, 0 as libc::c_int as cpFloat),
        ),
    );
    let mut stiff: cpFloat = 100.0f32 as cpFloat;
    let mut damp: cpFloat = 0.5f32 as cpFloat;
    cpSpaceAddConstraint(
        space,
        new_spring(
            staticBody,
            body1,
            cpv(-(320 as libc::c_int) as cpFloat, 240 as libc::c_int as cpFloat),
            cpv(-(40 as libc::c_int) as cpFloat, 40 as libc::c_int as cpFloat),
            0.0f32 as cpFloat,
            stiff,
            damp,
        ),
    );
    cpSpaceAddConstraint(
        space,
        new_spring(
            staticBody,
            body1,
            cpv(-(320 as libc::c_int) as cpFloat, 80 as libc::c_int as cpFloat),
            cpv(-(40 as libc::c_int) as cpFloat, 40 as libc::c_int as cpFloat),
            0.0f32 as cpFloat,
            stiff,
            damp,
        ),
    );
    cpSpaceAddConstraint(
        space,
        new_spring(
            staticBody,
            body1,
            cpv(-(160 as libc::c_int) as cpFloat, 240 as libc::c_int as cpFloat),
            cpv(-(40 as libc::c_int) as cpFloat, 40 as libc::c_int as cpFloat),
            0.0f32 as cpFloat,
            stiff,
            damp,
        ),
    );
    cpSpaceAddConstraint(
        space,
        new_spring(
            staticBody,
            body2,
            cpv(-(160 as libc::c_int) as cpFloat, 240 as libc::c_int as cpFloat),
            cpv(40 as libc::c_int as cpFloat, 40 as libc::c_int as cpFloat),
            0.0f32 as cpFloat,
            stiff,
            damp,
        ),
    );
    cpSpaceAddConstraint(
        space,
        new_spring(
            staticBody,
            body2,
            cpv(0 as libc::c_int as cpFloat, 240 as libc::c_int as cpFloat),
            cpv(40 as libc::c_int as cpFloat, 40 as libc::c_int as cpFloat),
            0.0f32 as cpFloat,
            stiff,
            damp,
        ),
    );
    cpSpaceAddConstraint(
        space,
        new_spring(
            staticBody,
            body3,
            cpv(80 as libc::c_int as cpFloat, 240 as libc::c_int as cpFloat),
            cpv(-(40 as libc::c_int) as cpFloat, 80 as libc::c_int as cpFloat),
            0.0f32 as cpFloat,
            stiff,
            damp,
        ),
    );
    cpSpaceAddConstraint(
        space,
        new_spring(
            staticBody,
            body4,
            cpv(80 as libc::c_int as cpFloat, 240 as libc::c_int as cpFloat),
            cpv(-(40 as libc::c_int) as cpFloat, 0 as libc::c_int as cpFloat),
            0.0f32 as cpFloat,
            stiff,
            damp,
        ),
    );
    cpSpaceAddConstraint(
        space,
        new_spring(
            staticBody,
            body4,
            cpv(320 as libc::c_int as cpFloat, 240 as libc::c_int as cpFloat),
            cpv(40 as libc::c_int as cpFloat, 0 as libc::c_int as cpFloat),
            0.0f32 as cpFloat,
            stiff,
            damp,
        ),
    );
    cpSpaceAddConstraint(
        space,
        new_spring(
            staticBody,
            body5,
            cpv(-(320 as libc::c_int) as cpFloat, 80 as libc::c_int as cpFloat),
            cpv(-(40 as libc::c_int) as cpFloat, 40 as libc::c_int as cpFloat),
            0.0f32 as cpFloat,
            stiff,
            damp,
        ),
    );
    cpSpaceAddConstraint(
        space,
        new_spring(
            staticBody,
            body9,
            cpv(320 as libc::c_int as cpFloat, 80 as libc::c_int as cpFloat),
            cpv(40 as libc::c_int as cpFloat, 40 as libc::c_int as cpFloat),
            0.0f32 as cpFloat,
            stiff,
            damp,
        ),
    );
    cpSpaceAddConstraint(
        space,
        new_spring(
            staticBody,
            body10,
            cpv(320 as libc::c_int as cpFloat, 0 as libc::c_int as cpFloat),
            cpv(40 as libc::c_int as cpFloat, -(40 as libc::c_int) as cpFloat),
            0.0f32 as cpFloat,
            stiff,
            damp,
        ),
    );
    cpSpaceAddConstraint(
        space,
        new_spring(
            staticBody,
            body10,
            cpv(320 as libc::c_int as cpFloat, -(160 as libc::c_int) as cpFloat),
            cpv(40 as libc::c_int as cpFloat, -(40 as libc::c_int) as cpFloat),
            0.0f32 as cpFloat,
            stiff,
            damp,
        ),
    );
    cpSpaceAddConstraint(
        space,
        new_spring(
            staticBody,
            body11,
            cpv(-(320 as libc::c_int) as cpFloat, -(160 as libc::c_int) as cpFloat),
            cpv(-(40 as libc::c_int) as cpFloat, 40 as libc::c_int as cpFloat),
            0.0f32 as cpFloat,
            stiff,
            damp,
        ),
    );
    cpSpaceAddConstraint(
        space,
        new_spring(
            staticBody,
            body12,
            cpv(-(240 as libc::c_int) as cpFloat, -(240 as libc::c_int) as cpFloat),
            cpv(-(40 as libc::c_int) as cpFloat, 0 as libc::c_int as cpFloat),
            0.0f32 as cpFloat,
            stiff,
            damp,
        ),
    );
    cpSpaceAddConstraint(
        space,
        new_spring(
            staticBody,
            body12,
            cpv(0 as libc::c_int as cpFloat, -(240 as libc::c_int) as cpFloat),
            cpv(40 as libc::c_int as cpFloat, 0 as libc::c_int as cpFloat),
            0.0f32 as cpFloat,
            stiff,
            damp,
        ),
    );
    cpSpaceAddConstraint(
        space,
        new_spring(
            staticBody,
            body13,
            cpv(0 as libc::c_int as cpFloat, -(240 as libc::c_int) as cpFloat),
            cpv(-(40 as libc::c_int) as cpFloat, 0 as libc::c_int as cpFloat),
            0.0f32 as cpFloat,
            stiff,
            damp,
        ),
    );
    cpSpaceAddConstraint(
        space,
        new_spring(
            staticBody,
            body13,
            cpv(80 as libc::c_int as cpFloat, -(240 as libc::c_int) as cpFloat),
            cpv(40 as libc::c_int as cpFloat, 0 as libc::c_int as cpFloat),
            0.0f32 as cpFloat,
            stiff,
            damp,
        ),
    );
    cpSpaceAddConstraint(
        space,
        new_spring(
            staticBody,
            body14,
            cpv(80 as libc::c_int as cpFloat, -(240 as libc::c_int) as cpFloat),
            cpv(-(40 as libc::c_int) as cpFloat, 0 as libc::c_int as cpFloat),
            0.0f32 as cpFloat,
            stiff,
            damp,
        ),
    );
    cpSpaceAddConstraint(
        space,
        new_spring(
            staticBody,
            body14,
            cpv(240 as libc::c_int as cpFloat, -(240 as libc::c_int) as cpFloat),
            cpv(40 as libc::c_int as cpFloat, 0 as libc::c_int as cpFloat),
            0.0f32 as cpFloat,
            stiff,
            damp,
        ),
    );
    cpSpaceAddConstraint(
        space,
        new_spring(
            staticBody,
            body14,
            cpv(320 as libc::c_int as cpFloat, -(160 as libc::c_int) as cpFloat),
            cpv(40 as libc::c_int as cpFloat, 0 as libc::c_int as cpFloat),
            0.0f32 as cpFloat,
            stiff,
            damp,
        ),
    );
    cpSpaceAddConstraint(
        space,
        new_spring(
            body1,
            body5,
            cpv(40 as libc::c_int as cpFloat, -(40 as libc::c_int) as cpFloat),
            cpv(-(40 as libc::c_int) as cpFloat, 40 as libc::c_int as cpFloat),
            0.0f32 as cpFloat,
            stiff,
            damp,
        ),
    );
    cpSpaceAddConstraint(
        space,
        new_spring(
            body1,
            body6,
            cpv(40 as libc::c_int as cpFloat, -(40 as libc::c_int) as cpFloat),
            cpv(40 as libc::c_int as cpFloat, 40 as libc::c_int as cpFloat),
            0.0f32 as cpFloat,
            stiff,
            damp,
        ),
    );
    cpSpaceAddConstraint(
        space,
        new_spring(
            body2,
            body3,
            cpv(40 as libc::c_int as cpFloat, 40 as libc::c_int as cpFloat),
            cpv(-(40 as libc::c_int) as cpFloat, 80 as libc::c_int as cpFloat),
            0.0f32 as cpFloat,
            stiff,
            damp,
        ),
    );
    cpSpaceAddConstraint(
        space,
        new_spring(
            body3,
            body4,
            cpv(-(40 as libc::c_int) as cpFloat, 80 as libc::c_int as cpFloat),
            cpv(-(40 as libc::c_int) as cpFloat, 0 as libc::c_int as cpFloat),
            0.0f32 as cpFloat,
            stiff,
            damp,
        ),
    );
    cpSpaceAddConstraint(
        space,
        new_spring(
            body3,
            body4,
            cpv(40 as libc::c_int as cpFloat, -(80 as libc::c_int) as cpFloat),
            cpv(-(40 as libc::c_int) as cpFloat, 0 as libc::c_int as cpFloat),
            0.0f32 as cpFloat,
            stiff,
            damp,
        ),
    );
    cpSpaceAddConstraint(
        space,
        new_spring(
            body3,
            body7,
            cpv(40 as libc::c_int as cpFloat, -(80 as libc::c_int) as cpFloat),
            cpv(40 as libc::c_int as cpFloat, 0 as libc::c_int as cpFloat),
            0.0f32 as cpFloat,
            stiff,
            damp,
        ),
    );
    cpSpaceAddConstraint(
        space,
        new_spring(
            body3,
            body7,
            cpv(-(40 as libc::c_int) as cpFloat, 80 as libc::c_int as cpFloat),
            cpv(-(40 as libc::c_int) as cpFloat, 0 as libc::c_int as cpFloat),
            0.0f32 as cpFloat,
            stiff,
            damp,
        ),
    );
    cpSpaceAddConstraint(
        space,
        new_spring(
            body3,
            body8,
            cpv(40 as libc::c_int as cpFloat, -(80 as libc::c_int) as cpFloat),
            cpv(40 as libc::c_int as cpFloat, 0 as libc::c_int as cpFloat),
            0.0f32 as cpFloat,
            stiff,
            damp,
        ),
    );
    cpSpaceAddConstraint(
        space,
        new_spring(
            body3,
            body9,
            cpv(40 as libc::c_int as cpFloat, -(80 as libc::c_int) as cpFloat),
            cpv(-(40 as libc::c_int) as cpFloat, -(40 as libc::c_int) as cpFloat),
            0.0f32 as cpFloat,
            stiff,
            damp,
        ),
    );
    cpSpaceAddConstraint(
        space,
        new_spring(
            body4,
            body9,
            cpv(40 as libc::c_int as cpFloat, 0 as libc::c_int as cpFloat),
            cpv(40 as libc::c_int as cpFloat, 40 as libc::c_int as cpFloat),
            0.0f32 as cpFloat,
            stiff,
            damp,
        ),
    );
    cpSpaceAddConstraint(
        space,
        new_spring(
            body5,
            body11,
            cpv(-(40 as libc::c_int) as cpFloat, 40 as libc::c_int as cpFloat),
            cpv(-(40 as libc::c_int) as cpFloat, 40 as libc::c_int as cpFloat),
            0.0f32 as cpFloat,
            stiff,
            damp,
        ),
    );
    cpSpaceAddConstraint(
        space,
        new_spring(
            body5,
            body11,
            cpv(40 as libc::c_int as cpFloat, -(40 as libc::c_int) as cpFloat),
            cpv(40 as libc::c_int as cpFloat, -(40 as libc::c_int) as cpFloat),
            0.0f32 as cpFloat,
            stiff,
            damp,
        ),
    );
    cpSpaceAddConstraint(
        space,
        new_spring(
            body7,
            body8,
            cpv(40 as libc::c_int as cpFloat, 0 as libc::c_int as cpFloat),
            cpv(-(40 as libc::c_int) as cpFloat, 0 as libc::c_int as cpFloat),
            0.0f32 as cpFloat,
            stiff,
            damp,
        ),
    );
    cpSpaceAddConstraint(
        space,
        new_spring(
            body8,
            body12,
            cpv(-(40 as libc::c_int) as cpFloat, 0 as libc::c_int as cpFloat),
            cpv(40 as libc::c_int as cpFloat, 0 as libc::c_int as cpFloat),
            0.0f32 as cpFloat,
            stiff,
            damp,
        ),
    );
    cpSpaceAddConstraint(
        space,
        new_spring(
            body8,
            body13,
            cpv(-(40 as libc::c_int) as cpFloat, 0 as libc::c_int as cpFloat),
            cpv(-(40 as libc::c_int) as cpFloat, 0 as libc::c_int as cpFloat),
            0.0f32 as cpFloat,
            stiff,
            damp,
        ),
    );
    cpSpaceAddConstraint(
        space,
        new_spring(
            body8,
            body13,
            cpv(40 as libc::c_int as cpFloat, 0 as libc::c_int as cpFloat),
            cpv(40 as libc::c_int as cpFloat, 0 as libc::c_int as cpFloat),
            0.0f32 as cpFloat,
            stiff,
            damp,
        ),
    );
    cpSpaceAddConstraint(
        space,
        new_spring(
            body8,
            body14,
            cpv(40 as libc::c_int as cpFloat, 0 as libc::c_int as cpFloat),
            cpv(-(40 as libc::c_int) as cpFloat, 0 as libc::c_int as cpFloat),
            0.0f32 as cpFloat,
            stiff,
            damp,
        ),
    );
    cpSpaceAddConstraint(
        space,
        new_spring(
            body10,
            body14,
            cpv(40 as libc::c_int as cpFloat, -(40 as libc::c_int) as cpFloat),
            cpv(-(40 as libc::c_int) as cpFloat, 0 as libc::c_int as cpFloat),
            0.0f32 as cpFloat,
            stiff,
            damp,
        ),
    );
    cpSpaceAddConstraint(
        space,
        new_spring(
            body10,
            body14,
            cpv(40 as libc::c_int as cpFloat, -(40 as libc::c_int) as cpFloat),
            cpv(-(40 as libc::c_int) as cpFloat, 0 as libc::c_int as cpFloat),
            0.0f32 as cpFloat,
            stiff,
            damp,
        ),
    );
    return space;
}
unsafe extern "C" fn destroy(mut space: *mut cpSpace) {
    ChipmunkDemoFreeSpaceChildren(space);
    cpSpaceFree(space);
}
pub static mut Springies: ChipmunkDemo = unsafe {
    {
        let mut init = ChipmunkDemo {
            name: b"Springies\0" as *const u8 as *const libc::c_char,
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
