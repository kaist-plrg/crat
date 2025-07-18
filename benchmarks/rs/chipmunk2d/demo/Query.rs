use ::libc;
extern "C" {
    pub type cpBody;
    pub type cpShape;
    pub type cpSpace;
    fn cos(_: libc::c_double) -> libc::c_double;
    fn sin(_: libc::c_double) -> libc::c_double;
    fn sqrt(_: libc::c_double) -> libc::c_double;
    fn cpBodyNew(mass: cpFloat, moment: cpFloat) -> *mut cpBody;
    fn cpBodySetPosition(body: *mut cpBody, pos: cpVect);
    fn cpShapeGetBB(shape: *const cpShape) -> cpBB;
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
    fn cpSpaceSegmentQueryFirst(
        space: *mut cpSpace,
        start: cpVect,
        end: cpVect,
        radius: cpFloat,
        filter: cpShapeFilter,
        out: *mut cpSegmentQueryInfo,
    ) -> *mut cpShape;
    fn cpSpaceStep(space: *mut cpSpace, dt: cpFloat);
    fn cpMomentForCircle(
        m: cpFloat,
        r1: cpFloat,
        r2: cpFloat,
        offset: cpVect,
    ) -> cpFloat;
    fn cpMomentForSegment(m: cpFloat, a: cpVect, b: cpVect, radius: cpFloat) -> cpFloat;
    fn cpMomentForPoly(
        m: cpFloat,
        count: libc::c_int,
        verts: *const cpVect,
        offset: cpVect,
        radius: cpFloat,
    ) -> cpFloat;
    fn ChipmunkDebugDrawSegment(a: cpVect, b: cpVect, color: cpSpaceDebugColor);
    fn ChipmunkDebugDrawDot(size: cpFloat, pos: cpVect, fillColor: cpSpaceDebugColor);
    fn ChipmunkDebugDrawBB(bb: cpBB, outlineColor: cpSpaceDebugColor);
    fn ChipmunkDemoFreeSpaceChildren(space: *mut cpSpace);
    fn ChipmunkDemoDefaultDrawImpl(space: *mut cpSpace);
    fn ChipmunkDebugDrawFatSegment(
        a: cpVect,
        b: cpVect,
        radius: cpFloat,
        outlineColor: cpSpaceDebugColor,
        fillColor: cpSpaceDebugColor,
    );
    static mut ChipmunkDemoMouse: cpVect;
    static mut ChipmunkDemoRightClick: cpBool;
    fn ChipmunkDemoPrintString(fmt: *const libc::c_char, _: ...);
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
pub struct cpBB {
    pub l: cpFloat,
    pub b: cpFloat,
    pub r: cpFloat,
    pub t: cpFloat,
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
pub struct cpSegmentQueryInfo {
    pub shape: *const cpShape,
    pub point: cpVect,
    pub normal: cpVect,
    pub alpha: cpFloat,
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
unsafe extern "C" fn cpvdot(v1: cpVect, v2: cpVect) -> cpFloat {
    return v1.x * v2.x + v1.y * v2.y;
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
unsafe extern "C" fn cpvdist(v1: cpVect, v2: cpVect) -> cpFloat {
    return cpvlength(cpvsub(v1, v2));
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
static mut CP_SHAPE_FILTER_ALL: cpShapeFilter = {
    let mut init = cpShapeFilter {
        group: 0 as libc::c_int as cpGroup,
        categories: !(0 as libc::c_int as cpBitmask),
        mask: !(0 as libc::c_int as cpBitmask),
    };
    init
};
#[inline]
unsafe extern "C" fn LAColor(
    mut l: libc::c_float,
    mut a: libc::c_float,
) -> cpSpaceDebugColor {
    let mut color: cpSpaceDebugColor = {
        let mut init = cpSpaceDebugColor {
            r: l,
            g: l,
            b: l,
            a: a,
        };
        init
    };
    return color;
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
static mut QUERY_START: cpVect = {
    let mut init = cpVect {
        x: 0 as libc::c_int as cpFloat,
        y: 0 as libc::c_int as cpFloat,
    };
    init
};
unsafe extern "C" fn update(mut space: *mut cpSpace, mut dt: libc::c_double) {
    cpSpaceStep(space, dt);
    if ChipmunkDemoRightClick != 0 {
        QUERY_START = ChipmunkDemoMouse;
    }
    let mut start: cpVect = QUERY_START;
    let mut end: cpVect = ChipmunkDemoMouse;
    let mut radius: cpFloat = 10.0f64;
    ChipmunkDebugDrawSegment(
        start,
        end,
        RGBAColor(
            0 as libc::c_int as libc::c_float,
            1 as libc::c_int as libc::c_float,
            0 as libc::c_int as libc::c_float,
            1 as libc::c_int as libc::c_float,
        ),
    );
    ChipmunkDemoPrintString(
        b"Query: Dist(%f) Point(%5.2f, %5.2f), \0" as *const u8 as *const libc::c_char,
        cpvdist(start, end),
        end.x,
        end.y,
    );
    let mut segInfo: cpSegmentQueryInfo = {
        let mut init = cpSegmentQueryInfo {
            shape: 0 as *const cpShape,
            point: cpVect { x: 0., y: 0. },
            normal: cpVect { x: 0., y: 0. },
            alpha: 0.,
        };
        init
    };
    if !(cpSpaceSegmentQueryFirst(
        space,
        start,
        end,
        radius,
        CP_SHAPE_FILTER_ALL,
        &mut segInfo,
    ))
        .is_null()
    {
        let mut point: cpVect = segInfo.point;
        let mut n: cpVect = segInfo.normal;
        ChipmunkDebugDrawSegment(
            cpvlerp(start, end, segInfo.alpha),
            end,
            RGBAColor(
                0 as libc::c_int as libc::c_float,
                0 as libc::c_int as libc::c_float,
                1 as libc::c_int as libc::c_float,
                1 as libc::c_int as libc::c_float,
            ),
        );
        ChipmunkDebugDrawSegment(
            point,
            cpvadd(point, cpvmult(n, 16 as libc::c_int as cpFloat)),
            RGBAColor(
                1 as libc::c_int as libc::c_float,
                0 as libc::c_int as libc::c_float,
                0 as libc::c_int as libc::c_float,
                1 as libc::c_int as libc::c_float,
            ),
        );
        ChipmunkDebugDrawDot(
            3 as libc::c_int as cpFloat,
            point,
            RGBAColor(
                1 as libc::c_int as libc::c_float,
                0 as libc::c_int as libc::c_float,
                0 as libc::c_int as libc::c_float,
                1 as libc::c_int as libc::c_float,
            ),
        );
        ChipmunkDemoPrintString(
            b"Segment Query: Dist(%f) Normal(%5.2f, %5.2f)\0" as *const u8
                as *const libc::c_char,
            segInfo.alpha * cpvdist(start, end),
            n.x,
            n.y,
        );
    } else {
        ChipmunkDemoPrintString(
            b"Segment Query (None)\0" as *const u8 as *const libc::c_char,
        );
    }
    ChipmunkDebugDrawFatSegment(
        start,
        cpvlerp(start, end, segInfo.alpha),
        radius,
        RGBAColor(
            0 as libc::c_int as libc::c_float,
            1 as libc::c_int as libc::c_float,
            0 as libc::c_int as libc::c_float,
            1 as libc::c_int as libc::c_float,
        ),
        LAColor(0 as libc::c_int as libc::c_float, 0 as libc::c_int as libc::c_float),
    );
    let mut nearestInfo: cpPointQueryInfo = {
        let mut init = cpPointQueryInfo {
            shape: 0 as *const cpShape,
            point: cpVect { x: 0., y: 0. },
            distance: 0.,
            gradient: cpVect { x: 0., y: 0. },
        };
        init
    };
    cpSpacePointQueryNearest(
        space,
        ChipmunkDemoMouse,
        100.0f64,
        CP_SHAPE_FILTER_ALL,
        &mut nearestInfo,
    );
    if !(nearestInfo.shape).is_null() {
        ChipmunkDebugDrawDot(
            3 as libc::c_int as cpFloat,
            ChipmunkDemoMouse,
            RGBAColor(
                0.5f64 as libc::c_float,
                0.5f64 as libc::c_float,
                0.5f64 as libc::c_float,
                1.0f64 as libc::c_float,
            ),
        );
        ChipmunkDebugDrawSegment(
            ChipmunkDemoMouse,
            nearestInfo.point,
            RGBAColor(
                0.5f64 as libc::c_float,
                0.5f64 as libc::c_float,
                0.5f64 as libc::c_float,
                1.0f64 as libc::c_float,
            ),
        );
        if nearestInfo.distance < 0 as libc::c_int as libc::c_double {
            ChipmunkDebugDrawBB(
                cpShapeGetBB(nearestInfo.shape),
                RGBAColor(
                    1 as libc::c_int as libc::c_float,
                    0 as libc::c_int as libc::c_float,
                    0 as libc::c_int as libc::c_float,
                    1 as libc::c_int as libc::c_float,
                ),
            );
        }
    }
}
unsafe extern "C" fn init() -> *mut cpSpace {
    QUERY_START = cpvzero;
    let mut space: *mut cpSpace = cpSpaceNew();
    cpSpaceSetIterations(space, 5 as libc::c_int);
    let mut mass: cpFloat = 1.0f32 as cpFloat;
    let mut length: cpFloat = 100.0f32 as cpFloat;
    let mut a: cpVect = cpv(-length / 2.0f32 as libc::c_double, 0.0f32 as cpFloat);
    let mut b: cpVect = cpv(length / 2.0f32 as libc::c_double, 0.0f32 as cpFloat);
    let mut body: *mut cpBody = cpSpaceAddBody(
        space,
        cpBodyNew(mass, cpMomentForSegment(mass, a, b, 0.0f32 as cpFloat)),
    );
    cpBodySetPosition(body, cpv(0.0f32 as cpFloat, 100.0f32 as cpFloat));
    cpSpaceAddShape(space, cpSegmentShapeNew(body, a, b, 20.0f32 as cpFloat));
    cpSpaceAddShape(
        space,
        cpSegmentShapeNew(
            cpSpaceGetStaticBody(space),
            cpv(0 as libc::c_int as cpFloat, 300 as libc::c_int as cpFloat),
            cpv(300 as libc::c_int as cpFloat, 0 as libc::c_int as cpFloat),
            0.0f32 as cpFloat,
        ),
    );
    let mut mass_0: cpFloat = 1.0f32 as cpFloat;
    let mut verts: [cpVect; 5] = [cpVect { x: 0., y: 0. }; 5];
    let mut i: libc::c_int = 0 as libc::c_int;
    while i < 5 as libc::c_int {
        let mut angle: cpFloat = -2.0f32 as libc::c_double
            * 3.14159265358979323846264338327950288f64 * i as libc::c_double
            / 5 as libc::c_int as cpFloat;
        verts[i
            as usize] = cpv(
            30 as libc::c_int as libc::c_double * cos(angle),
            30 as libc::c_int as libc::c_double * sin(angle),
        );
        i += 1;
        i;
    }
    let mut body_0: *mut cpBody = cpSpaceAddBody(
        space,
        cpBodyNew(
            mass_0,
            cpMomentForPoly(
                mass_0,
                5 as libc::c_int,
                verts.as_mut_ptr(),
                cpvzero,
                0.0f32 as cpFloat,
            ),
        ),
    );
    cpBodySetPosition(body_0, cpv(50.0f32 as cpFloat, 30.0f32 as cpFloat));
    cpSpaceAddShape(
        space,
        cpPolyShapeNew(
            body_0,
            5 as libc::c_int,
            verts.as_mut_ptr(),
            cpTransformIdentity,
            10.0f32 as cpFloat,
        ),
    );
    let mut mass_1: cpFloat = 1.0f32 as cpFloat;
    let mut r: cpFloat = 20.0f32 as cpFloat;
    let mut body_1: *mut cpBody = cpSpaceAddBody(
        space,
        cpBodyNew(mass_1, cpMomentForCircle(mass_1, 0.0f32 as cpFloat, r, cpvzero)),
    );
    cpBodySetPosition(body_1, cpv(100.0f32 as cpFloat, 100.0f32 as cpFloat));
    cpSpaceAddShape(space, cpCircleShapeNew(body_1, r, cpvzero));
    return space;
}
unsafe extern "C" fn destroy(mut space: *mut cpSpace) {
    ChipmunkDemoFreeSpaceChildren(space);
    cpSpaceFree(space);
}
pub static mut Query: ChipmunkDemo = unsafe {
    {
        let mut init = ChipmunkDemo {
            name: b"Segment Query\0" as *const u8 as *const libc::c_char,
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
