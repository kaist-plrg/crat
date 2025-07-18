use ::libc;
extern "C" {
    pub type cpBody;
    pub type cpShape;
    pub type cpSpace;
    pub type cpArbiter;
    fn cos(_: libc::c_double) -> libc::c_double;
    fn sin(_: libc::c_double) -> libc::c_double;
    fn pow(_: libc::c_double, _: libc::c_double) -> libc::c_double;
    fn abort() -> !;
    fn rand() -> libc::c_int;
    fn cpMomentForBox(m: cpFloat, width: cpFloat, height: cpFloat) -> cpFloat;
    fn cpMomentForPoly(
        m: cpFloat,
        count: libc::c_int,
        verts: *const cpVect,
        offset: cpVect,
        radius: cpFloat,
    ) -> cpFloat;
    fn cpMomentForCircle(
        m: cpFloat,
        r1: cpFloat,
        r2: cpFloat,
        offset: cpVect,
    ) -> cpFloat;
    fn cpBodyNew(mass: cpFloat, moment: cpFloat) -> *mut cpBody;
    fn cpBodySetPosition(body: *mut cpBody, pos: cpVect);
    fn cpBodySetVelocity(body: *mut cpBody, velocity: cpVect);
    fn cpShapeSetElasticity(shape: *mut cpShape, elasticity: cpFloat);
    fn cpShapeSetFriction(shape: *mut cpShape, friction: cpFloat);
    fn cpShapeSetCollisionType(shape: *mut cpShape, collisionType: cpCollisionType);
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
    fn cpBoxShapeNew(
        body: *mut cpBody,
        width: cpFloat,
        height: cpFloat,
        radius: cpFloat,
    ) -> *mut cpShape;
    fn cpSpaceNew() -> *mut cpSpace;
    fn cpSpaceFree(space: *mut cpSpace);
    fn cpSpaceSetIterations(space: *mut cpSpace, iterations: libc::c_int);
    fn cpSpaceSetGravity(space: *mut cpSpace, gravity: cpVect);
    fn cpSpaceSetCollisionSlop(space: *mut cpSpace, collisionSlop: cpFloat);
    fn cpSpaceGetStaticBody(space: *const cpSpace) -> *mut cpBody;
    fn cpSpaceAddCollisionHandler(
        space: *mut cpSpace,
        a: cpCollisionType,
        b: cpCollisionType,
    ) -> *mut cpCollisionHandler;
    fn cpSpaceAddShape(space: *mut cpSpace, shape: *mut cpShape) -> *mut cpShape;
    fn cpSpaceAddBody(space: *mut cpSpace, body: *mut cpBody) -> *mut cpBody;
    fn cpSpaceStep(space: *mut cpSpace, dt: cpFloat);
    fn cpPolyShapeSetRadius(shape: *mut cpShape, radius: cpFloat);
    fn ChipmunkDemoFreeSpaceChildren(space: *mut cpSpace);
    fn ChipmunkDemoDefaultDrawImpl(space: *mut cpSpace);
}
pub type uintptr_t = libc::c_ulong;
pub type cpFloat = libc::c_double;
pub type cpBool = libc::c_uchar;
pub type cpDataPointer = *mut libc::c_void;
pub type cpCollisionType = uintptr_t;
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
unsafe extern "C" fn cpvmult(v: cpVect, s: cpFloat) -> cpVect {
    return cpv(v.x * s, v.y * s);
}
#[inline]
unsafe extern "C" fn cpvdot(v1: cpVect, v2: cpVect) -> cpFloat {
    return v1.x * v2.x + v1.y * v2.y;
}
#[inline]
unsafe extern "C" fn cpvlengthsq(v: cpVect) -> cpFloat {
    return cpvdot(v, v);
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
unsafe extern "C" fn cpflerp(
    mut f1: cpFloat,
    mut f2: cpFloat,
    mut t: cpFloat,
) -> cpFloat {
    return f1 * (1.0f32 as libc::c_double - t) + f2 * t;
}
#[inline]
unsafe extern "C" fn frand() -> cpFloat {
    return rand() as cpFloat / 2147483647 as libc::c_int as cpFloat;
}
#[inline]
unsafe extern "C" fn frand_unit_circle() -> cpVect {
    let mut v: cpVect = cpv(
        frand() * 2.0f32 as libc::c_double - 1.0f32 as libc::c_double,
        frand() * 2.0f32 as libc::c_double - 1.0f32 as libc::c_double,
    );
    return if cpvlengthsq(v) < 1.0f32 as libc::c_double {
        v
    } else {
        frand_unit_circle()
    };
}
pub static mut bevel: cpFloat = 1.0f64;
static mut simple_terrain_verts: [cpVect; 48] = [
    {
        let mut init = cpVect {
            x: 350.00f64,
            y: 425.07f64,
        };
        init
    },
    {
        let mut init = cpVect {
            x: 336.00f64,
            y: 436.55f64,
        };
        init
    },
    {
        let mut init = cpVect {
            x: 272.00f64,
            y: 435.39f64,
        };
        init
    },
    {
        let mut init = cpVect {
            x: 258.00f64,
            y: 427.63f64,
        };
        init
    },
    {
        let mut init = cpVect {
            x: 225.28f64,
            y: 420.00f64,
        };
        init
    },
    {
        let mut init = cpVect {
            x: 202.82f64,
            y: 396.00f64,
        };
        init
    },
    {
        let mut init = cpVect {
            x: 191.81f64,
            y: 388.00f64,
        };
        init
    },
    {
        let mut init = cpVect {
            x: 189.00f64,
            y: 381.89f64,
        };
        init
    },
    {
        let mut init = cpVect {
            x: 173.00f64,
            y: 380.39f64,
        };
        init
    },
    {
        let mut init = cpVect {
            x: 162.59f64,
            y: 368.00f64,
        };
        init
    },
    {
        let mut init = cpVect {
            x: 150.47f64,
            y: 319.00f64,
        };
        init
    },
    {
        let mut init = cpVect {
            x: 128.00f64,
            y: 311.55f64,
        };
        init
    },
    {
        let mut init = cpVect {
            x: 119.14f64,
            y: 286.00f64,
        };
        init
    },
    {
        let mut init = cpVect {
            x: 126.84f64,
            y: 263.00f64,
        };
        init
    },
    {
        let mut init = cpVect {
            x: 120.56f64,
            y: 227.00f64,
        };
        init
    },
    {
        let mut init = cpVect {
            x: 141.14f64,
            y: 178.00f64,
        };
        init
    },
    {
        let mut init = cpVect {
            x: 137.52f64,
            y: 162.00f64,
        };
        init
    },
    {
        let mut init = cpVect {
            x: 146.51f64,
            y: 142.00f64,
        };
        init
    },
    {
        let mut init = cpVect {
            x: 156.23f64,
            y: 136.00f64,
        };
        init
    },
    {
        let mut init = cpVect {
            x: 158.00f64,
            y: 118.27f64,
        };
        init
    },
    {
        let mut init = cpVect {
            x: 170.00f64,
            y: 100.77f64,
        };
        init
    },
    {
        let mut init = cpVect {
            x: 208.43f64,
            y: 84.00f64,
        };
        init
    },
    {
        let mut init = cpVect {
            x: 224.00f64,
            y: 69.65f64,
        };
        init
    },
    {
        let mut init = cpVect {
            x: 249.30f64,
            y: 68.00f64,
        };
        init
    },
    {
        let mut init = cpVect {
            x: 257.00f64,
            y: 54.77f64,
        };
        init
    },
    {
        let mut init = cpVect {
            x: 363.00f64,
            y: 45.94f64,
        };
        init
    },
    {
        let mut init = cpVect {
            x: 374.15f64,
            y: 54.00f64,
        };
        init
    },
    {
        let mut init = cpVect {
            x: 386.00f64,
            y: 69.60f64,
        };
        init
    },
    {
        let mut init = cpVect {
            x: 413.00f64,
            y: 70.73f64,
        };
        init
    },
    {
        let mut init = cpVect {
            x: 456.00f64,
            y: 84.89f64,
        };
        init
    },
    {
        let mut init = cpVect {
            x: 468.09f64,
            y: 99.00f64,
        };
        init
    },
    {
        let mut init = cpVect {
            x: 467.09f64,
            y: 123.00f64,
        };
        init
    },
    {
        let mut init = cpVect {
            x: 464.92f64,
            y: 135.00f64,
        };
        init
    },
    {
        let mut init = cpVect {
            x: 469.00f64,
            y: 141.03f64,
        };
        init
    },
    {
        let mut init = cpVect {
            x: 497.00f64,
            y: 148.67f64,
        };
        init
    },
    {
        let mut init = cpVect {
            x: 513.85f64,
            y: 180.00f64,
        };
        init
    },
    {
        let mut init = cpVect {
            x: 509.56f64,
            y: 223.00f64,
        };
        init
    },
    {
        let mut init = cpVect {
            x: 523.51f64,
            y: 247.00f64,
        };
        init
    },
    {
        let mut init = cpVect {
            x: 523.00f64,
            y: 277.00f64,
        };
        init
    },
    {
        let mut init = cpVect {
            x: 497.79f64,
            y: 311.00f64,
        };
        init
    },
    {
        let mut init = cpVect {
            x: 478.67f64,
            y: 348.00f64,
        };
        init
    },
    {
        let mut init = cpVect {
            x: 467.90f64,
            y: 360.00f64,
        };
        init
    },
    {
        let mut init = cpVect {
            x: 456.76f64,
            y: 382.00f64,
        };
        init
    },
    {
        let mut init = cpVect {
            x: 432.95f64,
            y: 389.00f64,
        };
        init
    },
    {
        let mut init = cpVect {
            x: 417.00f64,
            y: 411.32f64,
        };
        init
    },
    {
        let mut init = cpVect {
            x: 373.00f64,
            y: 433.19f64,
        };
        init
    },
    {
        let mut init = cpVect {
            x: 361.00f64,
            y: 430.02f64,
        };
        init
    },
    {
        let mut init = cpVect {
            x: 350.00f64,
            y: 425.07f64,
        };
        init
    },
];
static mut simple_terrain_count: libc::c_int = 0;
unsafe extern "C" fn add_circle(
    mut space: *mut cpSpace,
    mut index: libc::c_int,
    mut radius: cpFloat,
) {
    let mut mass: cpFloat = radius * radius / 25.0f32 as libc::c_double;
    let mut body: *mut cpBody = cpSpaceAddBody(
        space,
        cpBodyNew(mass, cpMomentForCircle(mass, 0.0f32 as cpFloat, radius, cpvzero)),
    );
    cpBodySetPosition(body, cpvmult(frand_unit_circle(), 180.0f32 as cpFloat));
    let mut shape: *mut cpShape = cpSpaceAddShape(
        space,
        cpCircleShapeNew(body, radius, cpvzero),
    );
    cpShapeSetElasticity(shape, 0.0f64);
    cpShapeSetFriction(shape, 0.9f64);
}
unsafe extern "C" fn add_box(
    mut space: *mut cpSpace,
    mut index: libc::c_int,
    mut size: cpFloat,
) {
    let mut mass: cpFloat = size * size / 100.0f32 as libc::c_double;
    let mut body: *mut cpBody = cpSpaceAddBody(
        space,
        cpBodyNew(mass, cpMomentForBox(mass, size, size)),
    );
    cpBodySetPosition(body, cpvmult(frand_unit_circle(), 180.0f32 as cpFloat));
    let mut shape: *mut cpShape = cpSpaceAddShape(
        space,
        cpBoxShapeNew(
            body,
            size - bevel * 2 as libc::c_int as libc::c_double,
            size - bevel * 2 as libc::c_int as libc::c_double,
            0.0f64,
        ),
    );
    cpPolyShapeSetRadius(shape, bevel);
    cpShapeSetElasticity(shape, 0.0f64);
    cpShapeSetFriction(shape, 0.9f64);
}
unsafe extern "C" fn add_hexagon(
    mut space: *mut cpSpace,
    mut index: libc::c_int,
    mut radius: cpFloat,
) {
    let mut hexagon: [cpVect; 6] = [cpVect { x: 0., y: 0. }; 6];
    let mut i: libc::c_int = 0 as libc::c_int;
    while i < 6 as libc::c_int {
        let mut angle: cpFloat = -3.14159265358979323846264338327950288f64
            * 2.0f32 as libc::c_double * i as libc::c_double / 6.0f32 as libc::c_double;
        hexagon[i as usize] = cpvmult(cpv(cos(angle), sin(angle)), radius - bevel);
        i += 1;
        i;
    }
    let mut mass: cpFloat = radius * radius;
    let mut body: *mut cpBody = cpSpaceAddBody(
        space,
        cpBodyNew(
            mass,
            cpMomentForPoly(
                mass,
                6 as libc::c_int,
                hexagon.as_mut_ptr(),
                cpvzero,
                0.0f32 as cpFloat,
            ),
        ),
    );
    cpBodySetPosition(body, cpvmult(frand_unit_circle(), 180.0f32 as cpFloat));
    let mut shape: *mut cpShape = cpSpaceAddShape(
        space,
        cpPolyShapeNew(
            body,
            6 as libc::c_int,
            hexagon.as_mut_ptr(),
            cpTransformIdentity,
            bevel,
        ),
    );
    cpShapeSetElasticity(shape, 0.0f64);
    cpShapeSetFriction(shape, 0.9f64);
}
unsafe extern "C" fn SetupSpace_simpleTerrain() -> *mut cpSpace {
    let mut space: *mut cpSpace = cpSpaceNew();
    cpSpaceSetIterations(space, 10 as libc::c_int);
    cpSpaceSetGravity(
        space,
        cpv(0 as libc::c_int as cpFloat, -(100 as libc::c_int) as cpFloat),
    );
    cpSpaceSetCollisionSlop(space, 0.5f32 as cpFloat);
    let mut offset: cpVect = cpv(
        -(320 as libc::c_int) as cpFloat,
        -(240 as libc::c_int) as cpFloat,
    );
    let mut i: libc::c_int = 0 as libc::c_int;
    while i < simple_terrain_count - 1 as libc::c_int {
        let mut a: cpVect = simple_terrain_verts[i as usize];
        let mut b: cpVect = simple_terrain_verts[(i + 1 as libc::c_int) as usize];
        cpSpaceAddShape(
            space,
            cpSegmentShapeNew(
                cpSpaceGetStaticBody(space),
                cpvadd(a, offset),
                cpvadd(b, offset),
                0.0f32 as cpFloat,
            ),
        );
        i += 1;
        i;
    }
    return space;
}
unsafe extern "C" fn init_SimpleTerrainCircles_1000() -> *mut cpSpace {
    let mut space: *mut cpSpace = SetupSpace_simpleTerrain();
    let mut i: libc::c_int = 0 as libc::c_int;
    while i < 1000 as libc::c_int {
        add_circle(space, i, 5.0f32 as cpFloat);
        i += 1;
        i;
    }
    return space;
}
unsafe extern "C" fn init_SimpleTerrainCircles_500() -> *mut cpSpace {
    let mut space: *mut cpSpace = SetupSpace_simpleTerrain();
    let mut i: libc::c_int = 0 as libc::c_int;
    while i < 500 as libc::c_int {
        add_circle(space, i, 5.0f32 as cpFloat);
        i += 1;
        i;
    }
    return space;
}
unsafe extern "C" fn init_SimpleTerrainCircles_100() -> *mut cpSpace {
    let mut space: *mut cpSpace = SetupSpace_simpleTerrain();
    let mut i: libc::c_int = 0 as libc::c_int;
    while i < 100 as libc::c_int {
        add_circle(space, i, 5.0f32 as cpFloat);
        i += 1;
        i;
    }
    return space;
}
unsafe extern "C" fn init_SimpleTerrainBoxes_1000() -> *mut cpSpace {
    let mut space: *mut cpSpace = SetupSpace_simpleTerrain();
    let mut i: libc::c_int = 0 as libc::c_int;
    while i < 1000 as libc::c_int {
        add_box(space, i, 10.0f32 as cpFloat);
        i += 1;
        i;
    }
    return space;
}
unsafe extern "C" fn init_SimpleTerrainBoxes_500() -> *mut cpSpace {
    let mut space: *mut cpSpace = SetupSpace_simpleTerrain();
    let mut i: libc::c_int = 0 as libc::c_int;
    while i < 500 as libc::c_int {
        add_box(space, i, 10.0f32 as cpFloat);
        i += 1;
        i;
    }
    return space;
}
unsafe extern "C" fn init_SimpleTerrainBoxes_100() -> *mut cpSpace {
    let mut space: *mut cpSpace = SetupSpace_simpleTerrain();
    let mut i: libc::c_int = 0 as libc::c_int;
    while i < 100 as libc::c_int {
        add_box(space, i, 10.0f32 as cpFloat);
        i += 1;
        i;
    }
    return space;
}
unsafe extern "C" fn init_SimpleTerrainHexagons_1000() -> *mut cpSpace {
    let mut space: *mut cpSpace = SetupSpace_simpleTerrain();
    let mut i: libc::c_int = 0 as libc::c_int;
    while i < 1000 as libc::c_int {
        add_hexagon(space, i, 5.0f32 as cpFloat);
        i += 1;
        i;
    }
    return space;
}
unsafe extern "C" fn init_SimpleTerrainHexagons_500() -> *mut cpSpace {
    let mut space: *mut cpSpace = SetupSpace_simpleTerrain();
    let mut i: libc::c_int = 0 as libc::c_int;
    while i < 500 as libc::c_int {
        add_hexagon(space, i, 5.0f32 as cpFloat);
        i += 1;
        i;
    }
    return space;
}
unsafe extern "C" fn init_SimpleTerrainHexagons_100() -> *mut cpSpace {
    let mut space: *mut cpSpace = SetupSpace_simpleTerrain();
    let mut i: libc::c_int = 0 as libc::c_int;
    while i < 100 as libc::c_int {
        add_hexagon(space, i, 5.0f32 as cpFloat);
        i += 1;
        i;
    }
    return space;
}
unsafe extern "C" fn rand_size() -> cpFloat {
    return pow(1.5f64, cpflerp(-1.5f64, 3.5f64, frand()));
}
unsafe extern "C" fn init_SimpleTerrainVCircles_200() -> *mut cpSpace {
    let mut space: *mut cpSpace = SetupSpace_simpleTerrain();
    let mut i: libc::c_int = 0 as libc::c_int;
    while i < 200 as libc::c_int {
        add_circle(space, i, 5.0f32 as libc::c_double * rand_size());
        i += 1;
        i;
    }
    return space;
}
unsafe extern "C" fn init_SimpleTerrainVBoxes_200() -> *mut cpSpace {
    let mut space: *mut cpSpace = SetupSpace_simpleTerrain();
    let mut i: libc::c_int = 0 as libc::c_int;
    while i < 200 as libc::c_int {
        add_box(space, i, 8.0f32 as libc::c_double * rand_size());
        i += 1;
        i;
    }
    return space;
}
unsafe extern "C" fn init_SimpleTerrainVHexagons_200() -> *mut cpSpace {
    let mut space: *mut cpSpace = SetupSpace_simpleTerrain();
    let mut i: libc::c_int = 0 as libc::c_int;
    while i < 200 as libc::c_int {
        add_hexagon(space, i, 5.0f32 as libc::c_double * rand_size());
        i += 1;
        i;
    }
    return space;
}
static mut complex_terrain_verts: [cpVect; 254] = [
    {
        let mut init = cpVect {
            x: 46.78f64,
            y: 479.00f64,
        };
        init
    },
    {
        let mut init = cpVect {
            x: 35.00f64,
            y: 475.63f64,
        };
        init
    },
    {
        let mut init = cpVect {
            x: 27.52f64,
            y: 469.00f64,
        };
        init
    },
    {
        let mut init = cpVect {
            x: 23.52f64,
            y: 455.00f64,
        };
        init
    },
    {
        let mut init = cpVect {
            x: 23.78f64,
            y: 441.00f64,
        };
        init
    },
    {
        let mut init = cpVect {
            x: 28.41f64,
            y: 428.00f64,
        };
        init
    },
    {
        let mut init = cpVect {
            x: 49.61f64,
            y: 394.00f64,
        };
        init
    },
    {
        let mut init = cpVect {
            x: 59.00f64,
            y: 381.56f64,
        };
        init
    },
    {
        let mut init = cpVect {
            x: 80.00f64,
            y: 366.03f64,
        };
        init
    },
    {
        let mut init = cpVect {
            x: 81.46f64,
            y: 358.00f64,
        };
        init
    },
    {
        let mut init = cpVect {
            x: 86.31f64,
            y: 350.00f64,
        };
        init
    },
    {
        let mut init = cpVect {
            x: 77.74f64,
            y: 320.00f64,
        };
        init
    },
    {
        let mut init = cpVect {
            x: 70.26f64,
            y: 278.00f64,
        };
        init
    },
    {
        let mut init = cpVect {
            x: 67.51f64,
            y: 270.00f64,
        };
        init
    },
    {
        let mut init = cpVect {
            x: 58.86f64,
            y: 260.00f64,
        };
        init
    },
    {
        let mut init = cpVect {
            x: 57.19f64,
            y: 247.00f64,
        };
        init
    },
    {
        let mut init = cpVect {
            x: 38.00f64,
            y: 235.60f64,
        };
        init
    },
    {
        let mut init = cpVect {
            x: 25.76f64,
            y: 221.00f64,
        };
        init
    },
    {
        let mut init = cpVect {
            x: 24.58f64,
            y: 209.00f64,
        };
        init
    },
    {
        let mut init = cpVect {
            x: 27.63f64,
            y: 202.00f64,
        };
        init
    },
    {
        let mut init = cpVect {
            x: 31.28f64,
            y: 198.00f64,
        };
        init
    },
    {
        let mut init = cpVect {
            x: 40.00f64,
            y: 193.72f64,
        };
        init
    },
    {
        let mut init = cpVect {
            x: 48.00f64,
            y: 193.73f64,
        };
        init
    },
    {
        let mut init = cpVect {
            x: 55.00f64,
            y: 196.70f64,
        };
        init
    },
    {
        let mut init = cpVect {
            x: 62.10f64,
            y: 204.00f64,
        };
        init
    },
    {
        let mut init = cpVect {
            x: 71.00f64,
            y: 209.04f64,
        };
        init
    },
    {
        let mut init = cpVect {
            x: 79.00f64,
            y: 206.55f64,
        };
        init
    },
    {
        let mut init = cpVect {
            x: 88.00f64,
            y: 206.81f64,
        };
        init
    },
    {
        let mut init = cpVect {
            x: 95.88f64,
            y: 211.00f64,
        };
        init
    },
    {
        let mut init = cpVect {
            x: 103.00f64,
            y: 220.49f64,
        };
        init
    },
    {
        let mut init = cpVect {
            x: 131.00f64,
            y: 220.51f64,
        };
        init
    },
    {
        let mut init = cpVect {
            x: 137.00f64,
            y: 222.66f64,
        };
        init
    },
    {
        let mut init = cpVect {
            x: 143.08f64,
            y: 228.00f64,
        };
        init
    },
    {
        let mut init = cpVect {
            x: 146.22f64,
            y: 234.00f64,
        };
        init
    },
    {
        let mut init = cpVect {
            x: 147.08f64,
            y: 241.00f64,
        };
        init
    },
    {
        let mut init = cpVect {
            x: 145.45f64,
            y: 248.00f64,
        };
        init
    },
    {
        let mut init = cpVect {
            x: 142.31f64,
            y: 253.00f64,
        };
        init
    },
    {
        let mut init = cpVect {
            x: 132.00f64,
            y: 259.30f64,
        };
        init
    },
    {
        let mut init = cpVect {
            x: 115.00f64,
            y: 259.70f64,
        };
        init
    },
    {
        let mut init = cpVect {
            x: 109.28f64,
            y: 270.00f64,
        };
        init
    },
    {
        let mut init = cpVect {
            x: 112.91f64,
            y: 296.00f64,
        };
        init
    },
    {
        let mut init = cpVect {
            x: 119.69f64,
            y: 324.00f64,
        };
        init
    },
    {
        let mut init = cpVect {
            x: 129.00f64,
            y: 336.26f64,
        };
        init
    },
    {
        let mut init = cpVect {
            x: 141.00f64,
            y: 337.59f64,
        };
        init
    },
    {
        let mut init = cpVect {
            x: 153.00f64,
            y: 331.57f64,
        };
        init
    },
    {
        let mut init = cpVect {
            x: 175.00f64,
            y: 325.74f64,
        };
        init
    },
    {
        let mut init = cpVect {
            x: 188.00f64,
            y: 325.19f64,
        };
        init
    },
    {
        let mut init = cpVect {
            x: 235.00f64,
            y: 317.46f64,
        };
        init
    },
    {
        let mut init = cpVect {
            x: 250.00f64,
            y: 317.19f64,
        };
        init
    },
    {
        let mut init = cpVect {
            x: 255.00f64,
            y: 309.12f64,
        };
        init
    },
    {
        let mut init = cpVect {
            x: 262.62f64,
            y: 302.00f64,
        };
        init
    },
    {
        let mut init = cpVect {
            x: 262.21f64,
            y: 295.00f64,
        };
        init
    },
    {
        let mut init = cpVect {
            x: 248.00f64,
            y: 273.59f64,
        };
        init
    },
    {
        let mut init = cpVect {
            x: 229.00f64,
            y: 257.93f64,
        };
        init
    },
    {
        let mut init = cpVect {
            x: 221.00f64,
            y: 255.48f64,
        };
        init
    },
    {
        let mut init = cpVect {
            x: 215.00f64,
            y: 251.59f64,
        };
        init
    },
    {
        let mut init = cpVect {
            x: 210.79f64,
            y: 246.00f64,
        };
        init
    },
    {
        let mut init = cpVect {
            x: 207.47f64,
            y: 234.00f64,
        };
        init
    },
    {
        let mut init = cpVect {
            x: 203.25f64,
            y: 227.00f64,
        };
        init
    },
    {
        let mut init = cpVect {
            x: 179.00f64,
            y: 205.90f64,
        };
        init
    },
    {
        let mut init = cpVect {
            x: 148.00f64,
            y: 189.54f64,
        };
        init
    },
    {
        let mut init = cpVect {
            x: 136.00f64,
            y: 181.45f64,
        };
        init
    },
    {
        let mut init = cpVect {
            x: 120.00f64,
            y: 180.31f64,
        };
        init
    },
    {
        let mut init = cpVect {
            x: 110.00f64,
            y: 181.65f64,
        };
        init
    },
    {
        let mut init = cpVect {
            x: 95.00f64,
            y: 179.31f64,
        };
        init
    },
    {
        let mut init = cpVect {
            x: 63.00f64,
            y: 166.96f64,
        };
        init
    },
    {
        let mut init = cpVect {
            x: 50.00f64,
            y: 164.23f64,
        };
        init
    },
    {
        let mut init = cpVect {
            x: 31.00f64,
            y: 154.49f64,
        };
        init
    },
    {
        let mut init = cpVect {
            x: 19.76f64,
            y: 145.00f64,
        };
        init
    },
    {
        let mut init = cpVect {
            x: 15.96f64,
            y: 136.00f64,
        };
        init
    },
    {
        let mut init = cpVect {
            x: 16.65f64,
            y: 127.00f64,
        };
        init
    },
    {
        let mut init = cpVect {
            x: 20.57f64,
            y: 120.00f64,
        };
        init
    },
    {
        let mut init = cpVect {
            x: 28.00f64,
            y: 114.63f64,
        };
        init
    },
    {
        let mut init = cpVect {
            x: 40.00f64,
            y: 113.67f64,
        };
        init
    },
    {
        let mut init = cpVect {
            x: 65.00f64,
            y: 127.22f64,
        };
        init
    },
    {
        let mut init = cpVect {
            x: 73.00f64,
            y: 128.69f64,
        };
        init
    },
    {
        let mut init = cpVect {
            x: 81.96f64,
            y: 120.00f64,
        };
        init
    },
    {
        let mut init = cpVect {
            x: 77.58f64,
            y: 103.00f64,
        };
        init
    },
    {
        let mut init = cpVect { x: 78.18f64, y: 92.00f64 };
        init
    },
    {
        let mut init = cpVect { x: 59.11f64, y: 77.00f64 };
        init
    },
    {
        let mut init = cpVect { x: 52.00f64, y: 67.29f64 };
        init
    },
    {
        let mut init = cpVect { x: 31.29f64, y: 55.00f64 };
        init
    },
    {
        let mut init = cpVect { x: 25.67f64, y: 47.00f64 };
        init
    },
    {
        let mut init = cpVect { x: 24.65f64, y: 37.00f64 };
        init
    },
    {
        let mut init = cpVect { x: 27.82f64, y: 29.00f64 };
        init
    },
    {
        let mut init = cpVect { x: 35.00f64, y: 22.55f64 };
        init
    },
    {
        let mut init = cpVect { x: 44.00f64, y: 20.35f64 };
        init
    },
    {
        let mut init = cpVect { x: 49.00f64, y: 20.81f64 };
        init
    },
    {
        let mut init = cpVect { x: 61.00f64, y: 25.69f64 };
        init
    },
    {
        let mut init = cpVect { x: 79.00f64, y: 37.81f64 };
        init
    },
    {
        let mut init = cpVect { x: 88.00f64, y: 49.64f64 };
        init
    },
    {
        let mut init = cpVect { x: 97.00f64, y: 56.65f64 };
        init
    },
    {
        let mut init = cpVect {
            x: 109.00f64,
            y: 49.61f64,
        };
        init
    },
    {
        let mut init = cpVect {
            x: 143.00f64,
            y: 38.96f64,
        };
        init
    },
    {
        let mut init = cpVect {
            x: 197.00f64,
            y: 37.27f64,
        };
        init
    },
    {
        let mut init = cpVect {
            x: 215.00f64,
            y: 35.30f64,
        };
        init
    },
    {
        let mut init = cpVect {
            x: 222.00f64,
            y: 36.65f64,
        };
        init
    },
    {
        let mut init = cpVect {
            x: 228.42f64,
            y: 41.00f64,
        };
        init
    },
    {
        let mut init = cpVect {
            x: 233.30f64,
            y: 49.00f64,
        };
        init
    },
    {
        let mut init = cpVect {
            x: 234.14f64,
            y: 57.00f64,
        };
        init
    },
    {
        let mut init = cpVect {
            x: 231.00f64,
            y: 65.80f64,
        };
        init
    },
    {
        let mut init = cpVect {
            x: 224.00f64,
            y: 72.38f64,
        };
        init
    },
    {
        let mut init = cpVect {
            x: 218.00f64,
            y: 74.50f64,
        };
        init
    },
    {
        let mut init = cpVect {
            x: 197.00f64,
            y: 76.62f64,
        };
        init
    },
    {
        let mut init = cpVect {
            x: 145.00f64,
            y: 78.81f64,
        };
        init
    },
    {
        let mut init = cpVect {
            x: 123.00f64,
            y: 87.41f64,
        };
        init
    },
    {
        let mut init = cpVect {
            x: 117.59f64,
            y: 98.00f64,
        };
        init
    },
    {
        let mut init = cpVect {
            x: 117.79f64,
            y: 104.00f64,
        };
        init
    },
    {
        let mut init = cpVect {
            x: 119.00f64,
            y: 106.23f64,
        };
        init
    },
    {
        let mut init = cpVect {
            x: 138.73f64,
            y: 120.00f64,
        };
        init
    },
    {
        let mut init = cpVect {
            x: 148.00f64,
            y: 129.50f64,
        };
        init
    },
    {
        let mut init = cpVect {
            x: 158.50f64,
            y: 149.00f64,
        };
        init
    },
    {
        let mut init = cpVect {
            x: 203.93f64,
            y: 175.00f64,
        };
        init
    },
    {
        let mut init = cpVect {
            x: 229.00f64,
            y: 196.60f64,
        };
        init
    },
    {
        let mut init = cpVect {
            x: 238.16f64,
            y: 208.00f64,
        };
        init
    },
    {
        let mut init = cpVect {
            x: 245.20f64,
            y: 221.00f64,
        };
        init
    },
    {
        let mut init = cpVect {
            x: 275.45f64,
            y: 245.00f64,
        };
        init
    },
    {
        let mut init = cpVect {
            x: 289.00f64,
            y: 263.24f64,
        };
        init
    },
    {
        let mut init = cpVect {
            x: 303.60f64,
            y: 287.00f64,
        };
        init
    },
    {
        let mut init = cpVect {
            x: 312.00f64,
            y: 291.57f64,
        };
        init
    },
    {
        let mut init = cpVect {
            x: 339.25f64,
            y: 266.00f64,
        };
        init
    },
    {
        let mut init = cpVect {
            x: 366.33f64,
            y: 226.00f64,
        };
        init
    },
    {
        let mut init = cpVect {
            x: 363.43f64,
            y: 216.00f64,
        };
        init
    },
    {
        let mut init = cpVect {
            x: 364.13f64,
            y: 206.00f64,
        };
        init
    },
    {
        let mut init = cpVect {
            x: 353.00f64,
            y: 196.72f64,
        };
        init
    },
    {
        let mut init = cpVect {
            x: 324.00f64,
            y: 181.05f64,
        };
        init
    },
    {
        let mut init = cpVect {
            x: 307.00f64,
            y: 169.63f64,
        };
        init
    },
    {
        let mut init = cpVect {
            x: 274.93f64,
            y: 156.00f64,
        };
        init
    },
    {
        let mut init = cpVect {
            x: 256.00f64,
            y: 152.48f64,
        };
        init
    },
    {
        let mut init = cpVect {
            x: 228.00f64,
            y: 145.13f64,
        };
        init
    },
    {
        let mut init = cpVect {
            x: 221.09f64,
            y: 142.00f64,
        };
        init
    },
    {
        let mut init = cpVect {
            x: 214.87f64,
            y: 135.00f64,
        };
        init
    },
    {
        let mut init = cpVect {
            x: 212.67f64,
            y: 127.00f64,
        };
        init
    },
    {
        let mut init = cpVect {
            x: 213.81f64,
            y: 119.00f64,
        };
        init
    },
    {
        let mut init = cpVect {
            x: 219.32f64,
            y: 111.00f64,
        };
        init
    },
    {
        let mut init = cpVect {
            x: 228.00f64,
            y: 106.52f64,
        };
        init
    },
    {
        let mut init = cpVect {
            x: 236.00f64,
            y: 106.39f64,
        };
        init
    },
    {
        let mut init = cpVect {
            x: 290.00f64,
            y: 119.40f64,
        };
        init
    },
    {
        let mut init = cpVect {
            x: 299.33f64,
            y: 114.00f64,
        };
        init
    },
    {
        let mut init = cpVect {
            x: 300.52f64,
            y: 109.00f64,
        };
        init
    },
    {
        let mut init = cpVect {
            x: 300.30f64,
            y: 53.00f64,
        };
        init
    },
    {
        let mut init = cpVect {
            x: 301.46f64,
            y: 47.00f64,
        };
        init
    },
    {
        let mut init = cpVect {
            x: 305.00f64,
            y: 41.12f64,
        };
        init
    },
    {
        let mut init = cpVect {
            x: 311.00f64,
            y: 36.37f64,
        };
        init
    },
    {
        let mut init = cpVect {
            x: 317.00f64,
            y: 34.43f64,
        };
        init
    },
    {
        let mut init = cpVect {
            x: 325.00f64,
            y: 34.81f64,
        };
        init
    },
    {
        let mut init = cpVect {
            x: 334.90f64,
            y: 41.00f64,
        };
        init
    },
    {
        let mut init = cpVect {
            x: 339.45f64,
            y: 50.00f64,
        };
        init
    },
    {
        let mut init = cpVect {
            x: 339.82f64,
            y: 132.00f64,
        };
        init
    },
    {
        let mut init = cpVect {
            x: 346.09f64,
            y: 139.00f64,
        };
        init
    },
    {
        let mut init = cpVect {
            x: 350.00f64,
            y: 150.26f64,
        };
        init
    },
    {
        let mut init = cpVect {
            x: 380.00f64,
            y: 167.38f64,
        };
        init
    },
    {
        let mut init = cpVect {
            x: 393.00f64,
            y: 166.48f64,
        };
        init
    },
    {
        let mut init = cpVect {
            x: 407.00f64,
            y: 155.54f64,
        };
        init
    },
    {
        let mut init = cpVect {
            x: 430.00f64,
            y: 147.30f64,
        };
        init
    },
    {
        let mut init = cpVect {
            x: 437.78f64,
            y: 135.00f64,
        };
        init
    },
    {
        let mut init = cpVect {
            x: 433.13f64,
            y: 122.00f64,
        };
        init
    },
    {
        let mut init = cpVect {
            x: 410.23f64,
            y: 78.00f64,
        };
        init
    },
    {
        let mut init = cpVect {
            x: 401.59f64,
            y: 69.00f64,
        };
        init
    },
    {
        let mut init = cpVect {
            x: 393.48f64,
            y: 56.00f64,
        };
        init
    },
    {
        let mut init = cpVect {
            x: 392.80f64,
            y: 44.00f64,
        };
        init
    },
    {
        let mut init = cpVect {
            x: 395.50f64,
            y: 38.00f64,
        };
        init
    },
    {
        let mut init = cpVect {
            x: 401.00f64,
            y: 32.49f64,
        };
        init
    },
    {
        let mut init = cpVect {
            x: 409.00f64,
            y: 29.41f64,
        };
        init
    },
    {
        let mut init = cpVect {
            x: 420.00f64,
            y: 30.84f64,
        };
        init
    },
    {
        let mut init = cpVect {
            x: 426.92f64,
            y: 36.00f64,
        };
        init
    },
    {
        let mut init = cpVect {
            x: 432.32f64,
            y: 44.00f64,
        };
        init
    },
    {
        let mut init = cpVect {
            x: 439.49f64,
            y: 51.00f64,
        };
        init
    },
    {
        let mut init = cpVect {
            x: 470.13f64,
            y: 108.00f64,
        };
        init
    },
    {
        let mut init = cpVect {
            x: 475.71f64,
            y: 124.00f64,
        };
        init
    },
    {
        let mut init = cpVect {
            x: 483.00f64,
            y: 130.11f64,
        };
        init
    },
    {
        let mut init = cpVect {
            x: 488.00f64,
            y: 139.43f64,
        };
        init
    },
    {
        let mut init = cpVect {
            x: 529.00f64,
            y: 139.40f64,
        };
        init
    },
    {
        let mut init = cpVect {
            x: 536.00f64,
            y: 132.52f64,
        };
        init
    },
    {
        let mut init = cpVect {
            x: 543.73f64,
            y: 129.00f64,
        };
        init
    },
    {
        let mut init = cpVect {
            x: 540.47f64,
            y: 115.00f64,
        };
        init
    },
    {
        let mut init = cpVect {
            x: 541.11f64,
            y: 100.00f64,
        };
        init
    },
    {
        let mut init = cpVect {
            x: 552.18f64,
            y: 68.00f64,
        };
        init
    },
    {
        let mut init = cpVect {
            x: 553.78f64,
            y: 47.00f64,
        };
        init
    },
    {
        let mut init = cpVect {
            x: 559.00f64,
            y: 39.76f64,
        };
        init
    },
    {
        let mut init = cpVect {
            x: 567.00f64,
            y: 35.52f64,
        };
        init
    },
    {
        let mut init = cpVect {
            x: 577.00f64,
            y: 35.45f64,
        };
        init
    },
    {
        let mut init = cpVect {
            x: 585.00f64,
            y: 39.58f64,
        };
        init
    },
    {
        let mut init = cpVect {
            x: 591.38f64,
            y: 50.00f64,
        };
        init
    },
    {
        let mut init = cpVect {
            x: 591.67f64,
            y: 66.00f64,
        };
        init
    },
    {
        let mut init = cpVect {
            x: 590.31f64,
            y: 79.00f64,
        };
        init
    },
    {
        let mut init = cpVect {
            x: 579.76f64,
            y: 109.00f64,
        };
        init
    },
    {
        let mut init = cpVect {
            x: 582.25f64,
            y: 119.00f64,
        };
        init
    },
    {
        let mut init = cpVect {
            x: 583.66f64,
            y: 136.00f64,
        };
        init
    },
    {
        let mut init = cpVect {
            x: 586.45f64,
            y: 143.00f64,
        };
        init
    },
    {
        let mut init = cpVect {
            x: 586.44f64,
            y: 151.00f64,
        };
        init
    },
    {
        let mut init = cpVect {
            x: 580.42f64,
            y: 168.00f64,
        };
        init
    },
    {
        let mut init = cpVect {
            x: 577.15f64,
            y: 173.00f64,
        };
        init
    },
    {
        let mut init = cpVect {
            x: 572.00f64,
            y: 177.13f64,
        };
        init
    },
    {
        let mut init = cpVect {
            x: 564.00f64,
            y: 179.49f64,
        };
        init
    },
    {
        let mut init = cpVect {
            x: 478.00f64,
            y: 178.81f64,
        };
        init
    },
    {
        let mut init = cpVect {
            x: 443.00f64,
            y: 184.76f64,
        };
        init
    },
    {
        let mut init = cpVect {
            x: 427.10f64,
            y: 190.00f64,
        };
        init
    },
    {
        let mut init = cpVect {
            x: 424.00f64,
            y: 192.11f64,
        };
        init
    },
    {
        let mut init = cpVect {
            x: 415.94f64,
            y: 209.00f64,
        };
        init
    },
    {
        let mut init = cpVect {
            x: 408.82f64,
            y: 228.00f64,
        };
        init
    },
    {
        let mut init = cpVect {
            x: 405.82f64,
            y: 241.00f64,
        };
        init
    },
    {
        let mut init = cpVect {
            x: 411.00f64,
            y: 250.82f64,
        };
        init
    },
    {
        let mut init = cpVect {
            x: 415.00f64,
            y: 251.50f64,
        };
        init
    },
    {
        let mut init = cpVect {
            x: 428.00f64,
            y: 248.89f64,
        };
        init
    },
    {
        let mut init = cpVect {
            x: 469.00f64,
            y: 246.29f64,
        };
        init
    },
    {
        let mut init = cpVect {
            x: 505.00f64,
            y: 246.49f64,
        };
        init
    },
    {
        let mut init = cpVect {
            x: 533.00f64,
            y: 243.60f64,
        };
        init
    },
    {
        let mut init = cpVect {
            x: 541.87f64,
            y: 248.00f64,
        };
        init
    },
    {
        let mut init = cpVect {
            x: 547.55f64,
            y: 256.00f64,
        };
        init
    },
    {
        let mut init = cpVect {
            x: 548.48f64,
            y: 267.00f64,
        };
        init
    },
    {
        let mut init = cpVect {
            x: 544.00f64,
            y: 276.00f64,
        };
        init
    },
    {
        let mut init = cpVect {
            x: 534.00f64,
            y: 282.24f64,
        };
        init
    },
    {
        let mut init = cpVect {
            x: 513.00f64,
            y: 285.46f64,
        };
        init
    },
    {
        let mut init = cpVect {
            x: 468.00f64,
            y: 285.76f64,
        };
        init
    },
    {
        let mut init = cpVect {
            x: 402.00f64,
            y: 291.70f64,
        };
        init
    },
    {
        let mut init = cpVect {
            x: 392.00f64,
            y: 290.29f64,
        };
        init
    },
    {
        let mut init = cpVect {
            x: 377.00f64,
            y: 294.46f64,
        };
        init
    },
    {
        let mut init = cpVect {
            x: 367.00f64,
            y: 294.43f64,
        };
        init
    },
    {
        let mut init = cpVect {
            x: 356.44f64,
            y: 304.00f64,
        };
        init
    },
    {
        let mut init = cpVect {
            x: 354.22f64,
            y: 311.00f64,
        };
        init
    },
    {
        let mut init = cpVect {
            x: 362.00f64,
            y: 321.36f64,
        };
        init
    },
    {
        let mut init = cpVect {
            x: 390.00f64,
            y: 322.44f64,
        };
        init
    },
    {
        let mut init = cpVect {
            x: 433.00f64,
            y: 330.16f64,
        };
        init
    },
    {
        let mut init = cpVect {
            x: 467.00f64,
            y: 332.76f64,
        };
        init
    },
    {
        let mut init = cpVect {
            x: 508.00f64,
            y: 347.64f64,
        };
        init
    },
    {
        let mut init = cpVect {
            x: 522.00f64,
            y: 357.67f64,
        };
        init
    },
    {
        let mut init = cpVect {
            x: 528.00f64,
            y: 354.46f64,
        };
        init
    },
    {
        let mut init = cpVect {
            x: 536.00f64,
            y: 352.96f64,
        };
        init
    },
    {
        let mut init = cpVect {
            x: 546.06f64,
            y: 336.00f64,
        };
        init
    },
    {
        let mut init = cpVect {
            x: 553.47f64,
            y: 306.00f64,
        };
        init
    },
    {
        let mut init = cpVect {
            x: 564.19f64,
            y: 282.00f64,
        };
        init
    },
    {
        let mut init = cpVect {
            x: 567.84f64,
            y: 268.00f64,
        };
        init
    },
    {
        let mut init = cpVect {
            x: 578.72f64,
            y: 246.00f64,
        };
        init
    },
    {
        let mut init = cpVect {
            x: 585.00f64,
            y: 240.97f64,
        };
        init
    },
    {
        let mut init = cpVect {
            x: 592.00f64,
            y: 238.91f64,
        };
        init
    },
    {
        let mut init = cpVect {
            x: 600.00f64,
            y: 239.72f64,
        };
        init
    },
    {
        let mut init = cpVect {
            x: 606.00f64,
            y: 242.82f64,
        };
        init
    },
    {
        let mut init = cpVect {
            x: 612.36f64,
            y: 251.00f64,
        };
        init
    },
    {
        let mut init = cpVect {
            x: 613.35f64,
            y: 263.00f64,
        };
        init
    },
    {
        let mut init = cpVect {
            x: 588.75f64,
            y: 324.00f64,
        };
        init
    },
    {
        let mut init = cpVect {
            x: 583.25f64,
            y: 350.00f64,
        };
        init
    },
    {
        let mut init = cpVect {
            x: 572.12f64,
            y: 370.00f64,
        };
        init
    },
    {
        let mut init = cpVect {
            x: 575.45f64,
            y: 378.00f64,
        };
        init
    },
    {
        let mut init = cpVect {
            x: 575.20f64,
            y: 388.00f64,
        };
        init
    },
    {
        let mut init = cpVect {
            x: 589.00f64,
            y: 393.81f64,
        };
        init
    },
    {
        let mut init = cpVect {
            x: 599.20f64,
            y: 404.00f64,
        };
        init
    },
    {
        let mut init = cpVect {
            x: 607.14f64,
            y: 416.00f64,
        };
        init
    },
    {
        let mut init = cpVect {
            x: 609.96f64,
            y: 430.00f64,
        };
        init
    },
    {
        let mut init = cpVect {
            x: 615.45f64,
            y: 441.00f64,
        };
        init
    },
    {
        let mut init = cpVect {
            x: 613.44f64,
            y: 462.00f64,
        };
        init
    },
    {
        let mut init = cpVect {
            x: 610.48f64,
            y: 469.00f64,
        };
        init
    },
    {
        let mut init = cpVect {
            x: 603.00f64,
            y: 475.63f64,
        };
        init
    },
    {
        let mut init = cpVect {
            x: 590.96f64,
            y: 479.00f64,
        };
        init
    },
];
static mut complex_terrain_count: libc::c_int = 0;
unsafe extern "C" fn init_ComplexTerrainCircles_1000() -> *mut cpSpace {
    let mut space: *mut cpSpace = cpSpaceNew();
    cpSpaceSetIterations(space, 10 as libc::c_int);
    cpSpaceSetGravity(
        space,
        cpv(0 as libc::c_int as cpFloat, -(100 as libc::c_int) as cpFloat),
    );
    cpSpaceSetCollisionSlop(space, 0.5f32 as cpFloat);
    let mut offset: cpVect = cpv(
        -(320 as libc::c_int) as cpFloat,
        -(240 as libc::c_int) as cpFloat,
    );
    let mut i: libc::c_int = 0 as libc::c_int;
    while i < complex_terrain_count - 1 as libc::c_int {
        let mut a: cpVect = complex_terrain_verts[i as usize];
        let mut b: cpVect = complex_terrain_verts[(i + 1 as libc::c_int) as usize];
        cpSpaceAddShape(
            space,
            cpSegmentShapeNew(
                cpSpaceGetStaticBody(space),
                cpvadd(a, offset),
                cpvadd(b, offset),
                0.0f32 as cpFloat,
            ),
        );
        i += 1;
        i;
    }
    let mut i_0: libc::c_int = 0 as libc::c_int;
    while i_0 < 1000 as libc::c_int {
        let mut radius: cpFloat = 5.0f32 as cpFloat;
        let mut mass: cpFloat = radius * radius;
        let mut body: *mut cpBody = cpSpaceAddBody(
            space,
            cpBodyNew(mass, cpMomentForCircle(mass, 0.0f32 as cpFloat, radius, cpvzero)),
        );
        cpBodySetPosition(
            body,
            cpvadd(
                cpvmult(frand_unit_circle(), 180.0f32 as cpFloat),
                cpv(0.0f32 as cpFloat, 300.0f32 as cpFloat),
            ),
        );
        let mut shape: *mut cpShape = cpSpaceAddShape(
            space,
            cpCircleShapeNew(body, radius, cpvzero),
        );
        cpShapeSetElasticity(shape, 0.0f64);
        cpShapeSetFriction(shape, 0.0f64);
        i_0 += 1;
        i_0;
    }
    return space;
}
unsafe extern "C" fn init_ComplexTerrainHexagons_1000() -> *mut cpSpace {
    let mut space: *mut cpSpace = cpSpaceNew();
    cpSpaceSetIterations(space, 10 as libc::c_int);
    cpSpaceSetGravity(
        space,
        cpv(0 as libc::c_int as cpFloat, -(100 as libc::c_int) as cpFloat),
    );
    cpSpaceSetCollisionSlop(space, 0.5f32 as cpFloat);
    let mut offset: cpVect = cpv(
        -(320 as libc::c_int) as cpFloat,
        -(240 as libc::c_int) as cpFloat,
    );
    let mut i: libc::c_int = 0 as libc::c_int;
    while i < complex_terrain_count - 1 as libc::c_int {
        let mut a: cpVect = complex_terrain_verts[i as usize];
        let mut b: cpVect = complex_terrain_verts[(i + 1 as libc::c_int) as usize];
        cpSpaceAddShape(
            space,
            cpSegmentShapeNew(
                cpSpaceGetStaticBody(space),
                cpvadd(a, offset),
                cpvadd(b, offset),
                0.0f32 as cpFloat,
            ),
        );
        i += 1;
        i;
    }
    let mut radius: cpFloat = 5.0f32 as cpFloat;
    let mut hexagon: [cpVect; 6] = [cpVect { x: 0., y: 0. }; 6];
    let mut i_0: libc::c_int = 0 as libc::c_int;
    while i_0 < 6 as libc::c_int {
        let mut angle: cpFloat = -3.14159265358979323846264338327950288f64
            * 2.0f32 as libc::c_double * i_0 as libc::c_double
            / 6.0f32 as libc::c_double;
        hexagon[i_0 as usize] = cpvmult(cpv(cos(angle), sin(angle)), radius - bevel);
        i_0 += 1;
        i_0;
    }
    let mut i_1: libc::c_int = 0 as libc::c_int;
    while i_1 < 1000 as libc::c_int {
        let mut mass: cpFloat = radius * radius;
        let mut body: *mut cpBody = cpSpaceAddBody(
            space,
            cpBodyNew(
                mass,
                cpMomentForPoly(
                    mass,
                    6 as libc::c_int,
                    hexagon.as_mut_ptr(),
                    cpvzero,
                    0.0f32 as cpFloat,
                ),
            ),
        );
        cpBodySetPosition(
            body,
            cpvadd(
                cpvmult(frand_unit_circle(), 180.0f32 as cpFloat),
                cpv(0.0f32 as cpFloat, 300.0f32 as cpFloat),
            ),
        );
        let mut shape: *mut cpShape = cpSpaceAddShape(
            space,
            cpPolyShapeNew(
                body,
                6 as libc::c_int,
                hexagon.as_mut_ptr(),
                cpTransformIdentity,
                bevel,
            ),
        );
        cpShapeSetElasticity(shape, 0.0f64);
        cpShapeSetFriction(shape, 0.0f64);
        i_1 += 1;
        i_1;
    }
    return space;
}
static mut bouncy_terrain_verts: [cpVect; 516] = [
    {
        let mut init = cpVect {
            x: 537.18f64,
            y: 23.00f64,
        };
        init
    },
    {
        let mut init = cpVect {
            x: 520.50f64,
            y: 36.00f64,
        };
        init
    },
    {
        let mut init = cpVect {
            x: 501.53f64,
            y: 63.00f64,
        };
        init
    },
    {
        let mut init = cpVect {
            x: 496.14f64,
            y: 76.00f64,
        };
        init
    },
    {
        let mut init = cpVect {
            x: 498.86f64,
            y: 86.00f64,
        };
        init
    },
    {
        let mut init = cpVect {
            x: 504.00f64,
            y: 90.51f64,
        };
        init
    },
    {
        let mut init = cpVect {
            x: 508.00f64,
            y: 91.36f64,
        };
        init
    },
    {
        let mut init = cpVect {
            x: 508.77f64,
            y: 84.00f64,
        };
        init
    },
    {
        let mut init = cpVect {
            x: 513.00f64,
            y: 77.73f64,
        };
        init
    },
    {
        let mut init = cpVect {
            x: 519.00f64,
            y: 74.48f64,
        };
        init
    },
    {
        let mut init = cpVect {
            x: 530.00f64,
            y: 74.67f64,
        };
        init
    },
    {
        let mut init = cpVect {
            x: 545.00f64,
            y: 54.65f64,
        };
        init
    },
    {
        let mut init = cpVect {
            x: 554.00f64,
            y: 48.77f64,
        };
        init
    },
    {
        let mut init = cpVect {
            x: 562.00f64,
            y: 46.39f64,
        };
        init
    },
    {
        let mut init = cpVect {
            x: 568.00f64,
            y: 45.94f64,
        };
        init
    },
    {
        let mut init = cpVect {
            x: 568.61f64,
            y: 47.00f64,
        };
        init
    },
    {
        let mut init = cpVect {
            x: 567.94f64,
            y: 55.00f64,
        };
        init
    },
    {
        let mut init = cpVect {
            x: 571.27f64,
            y: 64.00f64,
        };
        init
    },
    {
        let mut init = cpVect {
            x: 572.92f64,
            y: 80.00f64,
        };
        init
    },
    {
        let mut init = cpVect {
            x: 572.00f64,
            y: 81.39f64,
        };
        init
    },
    {
        let mut init = cpVect {
            x: 563.00f64,
            y: 79.93f64,
        };
        init
    },
    {
        let mut init = cpVect {
            x: 556.00f64,
            y: 82.69f64,
        };
        init
    },
    {
        let mut init = cpVect {
            x: 551.49f64,
            y: 88.00f64,
        };
        init
    },
    {
        let mut init = cpVect {
            x: 549.00f64,
            y: 95.76f64,
        };
        init
    },
    {
        let mut init = cpVect {
            x: 538.00f64,
            y: 93.40f64,
        };
        init
    },
    {
        let mut init = cpVect {
            x: 530.00f64,
            y: 102.38f64,
        };
        init
    },
    {
        let mut init = cpVect {
            x: 523.00f64,
            y: 104.00f64,
        };
        init
    },
    {
        let mut init = cpVect {
            x: 517.00f64,
            y: 103.02f64,
        };
        init
    },
    {
        let mut init = cpVect {
            x: 516.22f64,
            y: 109.00f64,
        };
        init
    },
    {
        let mut init = cpVect {
            x: 518.96f64,
            y: 116.00f64,
        };
        init
    },
    {
        let mut init = cpVect {
            x: 526.00f64,
            y: 121.15f64,
        };
        init
    },
    {
        let mut init = cpVect {
            x: 534.00f64,
            y: 116.48f64,
        };
        init
    },
    {
        let mut init = cpVect {
            x: 543.00f64,
            y: 116.77f64,
        };
        init
    },
    {
        let mut init = cpVect {
            x: 549.28f64,
            y: 121.00f64,
        };
        init
    },
    {
        let mut init = cpVect {
            x: 554.00f64,
            y: 130.17f64,
        };
        init
    },
    {
        let mut init = cpVect {
            x: 564.00f64,
            y: 125.67f64,
        };
        init
    },
    {
        let mut init = cpVect {
            x: 575.60f64,
            y: 129.00f64,
        };
        init
    },
    {
        let mut init = cpVect {
            x: 573.31f64,
            y: 121.00f64,
        };
        init
    },
    {
        let mut init = cpVect {
            x: 567.77f64,
            y: 111.00f64,
        };
        init
    },
    {
        let mut init = cpVect {
            x: 575.00f64,
            y: 106.47f64,
        };
        init
    },
    {
        let mut init = cpVect {
            x: 578.51f64,
            y: 102.00f64,
        };
        init
    },
    {
        let mut init = cpVect {
            x: 580.25f64,
            y: 95.00f64,
        };
        init
    },
    {
        let mut init = cpVect {
            x: 577.98f64,
            y: 87.00f64,
        };
        init
    },
    {
        let mut init = cpVect {
            x: 582.00f64,
            y: 85.71f64,
        };
        init
    },
    {
        let mut init = cpVect {
            x: 597.00f64,
            y: 89.46f64,
        };
        init
    },
    {
        let mut init = cpVect {
            x: 604.80f64,
            y: 95.00f64,
        };
        init
    },
    {
        let mut init = cpVect {
            x: 609.28f64,
            y: 104.00f64,
        };
        init
    },
    {
        let mut init = cpVect {
            x: 610.55f64,
            y: 116.00f64,
        };
        init
    },
    {
        let mut init = cpVect {
            x: 609.30f64,
            y: 125.00f64,
        };
        init
    },
    {
        let mut init = cpVect {
            x: 600.80f64,
            y: 142.00f64,
        };
        init
    },
    {
        let mut init = cpVect {
            x: 597.31f64,
            y: 155.00f64,
        };
        init
    },
    {
        let mut init = cpVect {
            x: 584.00f64,
            y: 167.23f64,
        };
        init
    },
    {
        let mut init = cpVect {
            x: 577.86f64,
            y: 175.00f64,
        };
        init
    },
    {
        let mut init = cpVect {
            x: 583.52f64,
            y: 184.00f64,
        };
        init
    },
    {
        let mut init = cpVect {
            x: 582.64f64,
            y: 195.00f64,
        };
        init
    },
    {
        let mut init = cpVect {
            x: 591.00f64,
            y: 196.56f64,
        };
        init
    },
    {
        let mut init = cpVect {
            x: 597.81f64,
            y: 201.00f64,
        };
        init
    },
    {
        let mut init = cpVect {
            x: 607.45f64,
            y: 219.00f64,
        };
        init
    },
    {
        let mut init = cpVect {
            x: 607.51f64,
            y: 246.00f64,
        };
        init
    },
    {
        let mut init = cpVect {
            x: 600.00f64,
            y: 275.46f64,
        };
        init
    },
    {
        let mut init = cpVect {
            x: 588.00f64,
            y: 267.81f64,
        };
        init
    },
    {
        let mut init = cpVect {
            x: 579.00f64,
            y: 264.91f64,
        };
        init
    },
    {
        let mut init = cpVect {
            x: 557.00f64,
            y: 264.41f64,
        };
        init
    },
    {
        let mut init = cpVect {
            x: 552.98f64,
            y: 259.00f64,
        };
        init
    },
    {
        let mut init = cpVect {
            x: 548.00f64,
            y: 246.18f64,
        };
        init
    },
    {
        let mut init = cpVect {
            x: 558.00f64,
            y: 247.12f64,
        };
        init
    },
    {
        let mut init = cpVect {
            x: 565.98f64,
            y: 244.00f64,
        };
        init
    },
    {
        let mut init = cpVect {
            x: 571.10f64,
            y: 237.00f64,
        };
        init
    },
    {
        let mut init = cpVect {
            x: 571.61f64,
            y: 229.00f64,
        };
        init
    },
    {
        let mut init = cpVect {
            x: 568.25f64,
            y: 222.00f64,
        };
        init
    },
    {
        let mut init = cpVect {
            x: 562.00f64,
            y: 217.67f64,
        };
        init
    },
    {
        let mut init = cpVect {
            x: 544.00f64,
            y: 213.93f64,
        };
        init
    },
    {
        let mut init = cpVect {
            x: 536.73f64,
            y: 214.00f64,
        };
        init
    },
    {
        let mut init = cpVect {
            x: 535.60f64,
            y: 204.00f64,
        };
        init
    },
    {
        let mut init = cpVect {
            x: 539.69f64,
            y: 181.00f64,
        };
        init
    },
    {
        let mut init = cpVect {
            x: 542.84f64,
            y: 171.00f64,
        };
        init
    },
    {
        let mut init = cpVect {
            x: 550.43f64,
            y: 161.00f64,
        };
        init
    },
    {
        let mut init = cpVect {
            x: 540.00f64,
            y: 156.27f64,
        };
        init
    },
    {
        let mut init = cpVect {
            x: 536.62f64,
            y: 152.00f64,
        };
        init
    },
    {
        let mut init = cpVect {
            x: 534.70f64,
            y: 146.00f64,
        };
        init
    },
    {
        let mut init = cpVect {
            x: 527.00f64,
            y: 141.88f64,
        };
        init
    },
    {
        let mut init = cpVect {
            x: 518.59f64,
            y: 152.00f64,
        };
        init
    },
    {
        let mut init = cpVect {
            x: 514.51f64,
            y: 160.00f64,
        };
        init
    },
    {
        let mut init = cpVect {
            x: 510.33f64,
            y: 175.00f64,
        };
        init
    },
    {
        let mut init = cpVect {
            x: 519.38f64,
            y: 183.00f64,
        };
        init
    },
    {
        let mut init = cpVect {
            x: 520.52f64,
            y: 194.00f64,
        };
        init
    },
    {
        let mut init = cpVect {
            x: 516.00f64,
            y: 201.27f64,
        };
        init
    },
    {
        let mut init = cpVect {
            x: 505.25f64,
            y: 206.00f64,
        };
        init
    },
    {
        let mut init = cpVect {
            x: 507.57f64,
            y: 223.00f64,
        };
        init
    },
    {
        let mut init = cpVect {
            x: 519.90f64,
            y: 260.00f64,
        };
        init
    },
    {
        let mut init = cpVect {
            x: 529.00f64,
            y: 260.48f64,
        };
        init
    },
    {
        let mut init = cpVect {
            x: 534.00f64,
            y: 262.94f64,
        };
        init
    },
    {
        let mut init = cpVect {
            x: 538.38f64,
            y: 268.00f64,
        };
        init
    },
    {
        let mut init = cpVect {
            x: 540.00f64,
            y: 275.00f64,
        };
        init
    },
    {
        let mut init = cpVect {
            x: 537.06f64,
            y: 284.00f64,
        };
        init
    },
    {
        let mut init = cpVect {
            x: 530.00f64,
            y: 289.23f64,
        };
        init
    },
    {
        let mut init = cpVect {
            x: 520.00f64,
            y: 289.23f64,
        };
        init
    },
    {
        let mut init = cpVect {
            x: 513.00f64,
            y: 284.18f64,
        };
        init
    },
    {
        let mut init = cpVect {
            x: 509.71f64,
            y: 286.00f64,
        };
        init
    },
    {
        let mut init = cpVect {
            x: 501.69f64,
            y: 298.00f64,
        };
        init
    },
    {
        let mut init = cpVect {
            x: 501.56f64,
            y: 305.00f64,
        };
        init
    },
    {
        let mut init = cpVect {
            x: 504.30f64,
            y: 311.00f64,
        };
        init
    },
    {
        let mut init = cpVect {
            x: 512.00f64,
            y: 316.43f64,
        };
        init
    },
    {
        let mut init = cpVect {
            x: 521.00f64,
            y: 316.42f64,
        };
        init
    },
    {
        let mut init = cpVect {
            x: 525.67f64,
            y: 314.00f64,
        };
        init
    },
    {
        let mut init = cpVect {
            x: 535.00f64,
            y: 304.98f64,
        };
        init
    },
    {
        let mut init = cpVect {
            x: 562.00f64,
            y: 294.80f64,
        };
        init
    },
    {
        let mut init = cpVect {
            x: 573.00f64,
            y: 294.81f64,
        };
        init
    },
    {
        let mut init = cpVect {
            x: 587.52f64,
            y: 304.00f64,
        };
        init
    },
    {
        let mut init = cpVect {
            x: 600.89f64,
            y: 310.00f64,
        };
        init
    },
    {
        let mut init = cpVect {
            x: 596.96f64,
            y: 322.00f64,
        };
        init
    },
    {
        let mut init = cpVect {
            x: 603.28f64,
            y: 327.00f64,
        };
        init
    },
    {
        let mut init = cpVect {
            x: 606.52f64,
            y: 333.00f64,
        };
        init
    },
    {
        let mut init = cpVect {
            x: 605.38f64,
            y: 344.00f64,
        };
        init
    },
    {
        let mut init = cpVect {
            x: 597.65f64,
            y: 352.00f64,
        };
        init
    },
    {
        let mut init = cpVect {
            x: 606.36f64,
            y: 375.00f64,
        };
        init
    },
    {
        let mut init = cpVect {
            x: 607.16f64,
            y: 384.00f64,
        };
        init
    },
    {
        let mut init = cpVect {
            x: 603.40f64,
            y: 393.00f64,
        };
        init
    },
    {
        let mut init = cpVect {
            x: 597.00f64,
            y: 398.14f64,
        };
        init
    },
    {
        let mut init = cpVect {
            x: 577.00f64,
            y: 386.15f64,
        };
        init
    },
    {
        let mut init = cpVect {
            x: 564.35f64,
            y: 373.00f64,
        };
        init
    },
    {
        let mut init = cpVect {
            x: 565.21f64,
            y: 364.00f64,
        };
        init
    },
    {
        let mut init = cpVect {
            x: 562.81f64,
            y: 350.00f64,
        };
        init
    },
    {
        let mut init = cpVect {
            x: 553.00f64,
            y: 346.06f64,
        };
        init
    },
    {
        let mut init = cpVect {
            x: 547.48f64,
            y: 338.00f64,
        };
        init
    },
    {
        let mut init = cpVect {
            x: 547.48f64,
            y: 330.00f64,
        };
        init
    },
    {
        let mut init = cpVect {
            x: 550.00f64,
            y: 323.30f64,
        };
        init
    },
    {
        let mut init = cpVect {
            x: 544.00f64,
            y: 321.53f64,
        };
        init
    },
    {
        let mut init = cpVect {
            x: 537.00f64,
            y: 322.70f64,
        };
        init
    },
    {
        let mut init = cpVect {
            x: 532.00f64,
            y: 326.23f64,
        };
        init
    },
    {
        let mut init = cpVect {
            x: 528.89f64,
            y: 331.00f64,
        };
        init
    },
    {
        let mut init = cpVect {
            x: 527.83f64,
            y: 338.00f64,
        };
        init
    },
    {
        let mut init = cpVect {
            x: 533.02f64,
            y: 356.00f64,
        };
        init
    },
    {
        let mut init = cpVect {
            x: 542.00f64,
            y: 360.73f64,
        };
        init
    },
    {
        let mut init = cpVect {
            x: 546.68f64,
            y: 369.00f64,
        };
        init
    },
    {
        let mut init = cpVect {
            x: 545.38f64,
            y: 379.00f64,
        };
        init
    },
    {
        let mut init = cpVect {
            x: 537.58f64,
            y: 386.00f64,
        };
        init
    },
    {
        let mut init = cpVect {
            x: 537.63f64,
            y: 388.00f64,
        };
        init
    },
    {
        let mut init = cpVect {
            x: 555.00f64,
            y: 407.47f64,
        };
        init
    },
    {
        let mut init = cpVect {
            x: 563.00f64,
            y: 413.52f64,
        };
        init
    },
    {
        let mut init = cpVect {
            x: 572.57f64,
            y: 418.00f64,
        };
        init
    },
    {
        let mut init = cpVect {
            x: 582.72f64,
            y: 426.00f64,
        };
        init
    },
    {
        let mut init = cpVect {
            x: 578.00f64,
            y: 431.12f64,
        };
        init
    },
    {
        let mut init = cpVect {
            x: 563.21f64,
            y: 440.00f64,
        };
        init
    },
    {
        let mut init = cpVect {
            x: 558.00f64,
            y: 449.27f64,
        };
        init
    },
    {
        let mut init = cpVect {
            x: 549.00f64,
            y: 452.94f64,
        };
        init
    },
    {
        let mut init = cpVect {
            x: 541.00f64,
            y: 451.38f64,
        };
        init
    },
    {
        let mut init = cpVect {
            x: 536.73f64,
            y: 448.00f64,
        };
        init
    },
    {
        let mut init = cpVect {
            x: 533.00f64,
            y: 441.87f64,
        };
        init
    },
    {
        let mut init = cpVect {
            x: 520.00f64,
            y: 437.96f64,
        };
        init
    },
    {
        let mut init = cpVect {
            x: 514.00f64,
            y: 429.69f64,
        };
        init
    },
    {
        let mut init = cpVect {
            x: 490.00f64,
            y: 415.15f64,
        };
        init
    },
    {
        let mut init = cpVect {
            x: 472.89f64,
            y: 399.00f64,
        };
        init
    },
    {
        let mut init = cpVect {
            x: 472.03f64,
            y: 398.00f64,
        };
        init
    },
    {
        let mut init = cpVect {
            x: 474.00f64,
            y: 396.71f64,
        };
        init
    },
    {
        let mut init = cpVect {
            x: 486.00f64,
            y: 393.61f64,
        };
        init
    },
    {
        let mut init = cpVect {
            x: 492.00f64,
            y: 385.85f64,
        };
        init
    },
    {
        let mut init = cpVect {
            x: 492.00f64,
            y: 376.15f64,
        };
        init
    },
    {
        let mut init = cpVect {
            x: 489.04f64,
            y: 371.00f64,
        };
        init
    },
    {
        let mut init = cpVect {
            x: 485.00f64,
            y: 368.11f64,
        };
        init
    },
    {
        let mut init = cpVect {
            x: 480.00f64,
            y: 376.27f64,
        };
        init
    },
    {
        let mut init = cpVect {
            x: 472.00f64,
            y: 379.82f64,
        };
        init
    },
    {
        let mut init = cpVect {
            x: 463.00f64,
            y: 378.38f64,
        };
        init
    },
    {
        let mut init = cpVect {
            x: 455.08f64,
            y: 372.00f64,
        };
        init
    },
    {
        let mut init = cpVect {
            x: 446.00f64,
            y: 377.69f64,
        };
        init
    },
    {
        let mut init = cpVect {
            x: 439.00f64,
            y: 385.24f64,
        };
        init
    },
    {
        let mut init = cpVect {
            x: 436.61f64,
            y: 391.00f64,
        };
        init
    },
    {
        let mut init = cpVect {
            x: 437.52f64,
            y: 404.00f64,
        };
        init
    },
    {
        let mut init = cpVect {
            x: 440.00f64,
            y: 409.53f64,
        };
        init
    },
    {
        let mut init = cpVect {
            x: 463.53f64,
            y: 433.00f64,
        };
        init
    },
    {
        let mut init = cpVect {
            x: 473.80f64,
            y: 441.00f64,
        };
        init
    },
    {
        let mut init = cpVect {
            x: 455.00f64,
            y: 440.30f64,
        };
        init
    },
    {
        let mut init = cpVect {
            x: 443.00f64,
            y: 436.18f64,
        };
        init
    },
    {
        let mut init = cpVect {
            x: 436.00f64,
            y: 431.98f64,
        };
        init
    },
    {
        let mut init = cpVect {
            x: 412.00f64,
            y: 440.92f64,
        };
        init
    },
    {
        let mut init = cpVect {
            x: 397.00f64,
            y: 442.46f64,
        };
        init
    },
    {
        let mut init = cpVect {
            x: 393.59f64,
            y: 431.00f64,
        };
        init
    },
    {
        let mut init = cpVect {
            x: 393.71f64,
            y: 412.00f64,
        };
        init
    },
    {
        let mut init = cpVect {
            x: 400.00f64,
            y: 395.10f64,
        };
        init
    },
    {
        let mut init = cpVect {
            x: 407.32f64,
            y: 387.00f64,
        };
        init
    },
    {
        let mut init = cpVect {
            x: 408.54f64,
            y: 380.00f64,
        };
        init
    },
    {
        let mut init = cpVect {
            x: 407.42f64,
            y: 375.00f64,
        };
        init
    },
    {
        let mut init = cpVect {
            x: 403.97f64,
            y: 370.00f64,
        };
        init
    },
    {
        let mut init = cpVect {
            x: 399.00f64,
            y: 366.74f64,
        };
        init
    },
    {
        let mut init = cpVect {
            x: 393.00f64,
            y: 365.68f64,
        };
        init
    },
    {
        let mut init = cpVect {
            x: 391.23f64,
            y: 374.00f64,
        };
        init
    },
    {
        let mut init = cpVect {
            x: 387.00f64,
            y: 380.27f64,
        };
        init
    },
    {
        let mut init = cpVect {
            x: 381.00f64,
            y: 383.52f64,
        };
        init
    },
    {
        let mut init = cpVect {
            x: 371.56f64,
            y: 384.00f64,
        };
        init
    },
    {
        let mut init = cpVect {
            x: 364.98f64,
            y: 401.00f64,
        };
        init
    },
    {
        let mut init = cpVect {
            x: 362.96f64,
            y: 412.00f64,
        };
        init
    },
    {
        let mut init = cpVect {
            x: 363.63f64,
            y: 435.00f64,
        };
        init
    },
    {
        let mut init = cpVect {
            x: 345.00f64,
            y: 433.55f64,
        };
        init
    },
    {
        let mut init = cpVect {
            x: 344.52f64,
            y: 442.00f64,
        };
        init
    },
    {
        let mut init = cpVect {
            x: 342.06f64,
            y: 447.00f64,
        };
        init
    },
    {
        let mut init = cpVect {
            x: 337.00f64,
            y: 451.38f64,
        };
        init
    },
    {
        let mut init = cpVect {
            x: 330.00f64,
            y: 453.00f64,
        };
        init
    },
    {
        let mut init = cpVect {
            x: 325.00f64,
            y: 452.23f64,
        };
        init
    },
    {
        let mut init = cpVect {
            x: 318.00f64,
            y: 448.17f64,
        };
        init
    },
    {
        let mut init = cpVect {
            x: 298.00f64,
            y: 453.70f64,
        };
        init
    },
    {
        let mut init = cpVect {
            x: 284.00f64,
            y: 451.49f64,
        };
        init
    },
    {
        let mut init = cpVect {
            x: 278.62f64,
            y: 449.00f64,
        };
        init
    },
    {
        let mut init = cpVect {
            x: 291.47f64,
            y: 408.00f64,
        };
        init
    },
    {
        let mut init = cpVect {
            x: 291.77f64,
            y: 398.00f64,
        };
        init
    },
    {
        let mut init = cpVect {
            x: 301.00f64,
            y: 393.83f64,
        };
        init
    },
    {
        let mut init = cpVect {
            x: 305.00f64,
            y: 393.84f64,
        };
        init
    },
    {
        let mut init = cpVect {
            x: 305.60f64,
            y: 403.00f64,
        };
        init
    },
    {
        let mut init = cpVect {
            x: 310.00f64,
            y: 409.47f64,
        };
        init
    },
    {
        let mut init = cpVect {
            x: 318.00f64,
            y: 413.07f64,
        };
        init
    },
    {
        let mut init = cpVect {
            x: 325.00f64,
            y: 412.40f64,
        };
        init
    },
    {
        let mut init = cpVect {
            x: 332.31f64,
            y: 407.00f64,
        };
        init
    },
    {
        let mut init = cpVect {
            x: 335.07f64,
            y: 400.00f64,
        };
        init
    },
    {
        let mut init = cpVect {
            x: 334.40f64,
            y: 393.00f64,
        };
        init
    },
    {
        let mut init = cpVect {
            x: 329.00f64,
            y: 385.69f64,
        };
        init
    },
    {
        let mut init = cpVect {
            x: 319.00f64,
            y: 382.79f64,
        };
        init
    },
    {
        let mut init = cpVect {
            x: 301.00f64,
            y: 389.23f64,
        };
        init
    },
    {
        let mut init = cpVect {
            x: 289.00f64,
            y: 389.97f64,
        };
        init
    },
    {
        let mut init = cpVect {
            x: 265.00f64,
            y: 389.82f64,
        };
        init
    },
    {
        let mut init = cpVect {
            x: 251.00f64,
            y: 385.85f64,
        };
        init
    },
    {
        let mut init = cpVect {
            x: 245.00f64,
            y: 389.23f64,
        };
        init
    },
    {
        let mut init = cpVect {
            x: 239.00f64,
            y: 389.94f64,
        };
        init
    },
    {
        let mut init = cpVect {
            x: 233.00f64,
            y: 388.38f64,
        };
        init
    },
    {
        let mut init = cpVect {
            x: 226.00f64,
            y: 382.04f64,
        };
        init
    },
    {
        let mut init = cpVect {
            x: 206.00f64,
            y: 374.75f64,
        };
        init
    },
    {
        let mut init = cpVect {
            x: 206.00f64,
            y: 394.00f64,
        };
        init
    },
    {
        let mut init = cpVect {
            x: 204.27f64,
            y: 402.00f64,
        };
        init
    },
    {
        let mut init = cpVect {
            x: 197.00f64,
            y: 401.79f64,
        };
        init
    },
    {
        let mut init = cpVect {
            x: 191.00f64,
            y: 403.49f64,
        };
        init
    },
    {
        let mut init = cpVect {
            x: 186.53f64,
            y: 407.00f64,
        };
        init
    },
    {
        let mut init = cpVect {
            x: 183.60f64,
            y: 412.00f64,
        };
        init
    },
    {
        let mut init = cpVect {
            x: 183.60f64,
            y: 422.00f64,
        };
        init
    },
    {
        let mut init = cpVect {
            x: 189.00f64,
            y: 429.31f64,
        };
        init
    },
    {
        let mut init = cpVect {
            x: 196.00f64,
            y: 432.07f64,
        };
        init
    },
    {
        let mut init = cpVect {
            x: 203.00f64,
            y: 431.40f64,
        };
        init
    },
    {
        let mut init = cpVect {
            x: 209.47f64,
            y: 427.00f64,
        };
        init
    },
    {
        let mut init = cpVect {
            x: 213.00f64,
            y: 419.72f64,
        };
        init
    },
    {
        let mut init = cpVect {
            x: 220.00f64,
            y: 420.21f64,
        };
        init
    },
    {
        let mut init = cpVect {
            x: 227.00f64,
            y: 418.32f64,
        };
        init
    },
    {
        let mut init = cpVect {
            x: 242.00f64,
            y: 408.41f64,
        };
        init
    },
    {
        let mut init = cpVect {
            x: 258.98f64,
            y: 409.00f64,
        };
        init
    },
    {
        let mut init = cpVect {
            x: 250.00f64,
            y: 435.43f64,
        };
        init
    },
    {
        let mut init = cpVect {
            x: 239.00f64,
            y: 438.78f64,
        };
        init
    },
    {
        let mut init = cpVect {
            x: 223.00f64,
            y: 448.19f64,
        };
        init
    },
    {
        let mut init = cpVect {
            x: 209.00f64,
            y: 449.70f64,
        };
        init
    },
    {
        let mut init = cpVect {
            x: 205.28f64,
            y: 456.00f64,
        };
        init
    },
    {
        let mut init = cpVect {
            x: 199.00f64,
            y: 460.23f64,
        };
        init
    },
    {
        let mut init = cpVect {
            x: 190.00f64,
            y: 460.52f64,
        };
        init
    },
    {
        let mut init = cpVect {
            x: 182.73f64,
            y: 456.00f64,
        };
        init
    },
    {
        let mut init = cpVect {
            x: 178.00f64,
            y: 446.27f64,
        };
        init
    },
    {
        let mut init = cpVect {
            x: 160.00f64,
            y: 441.42f64,
        };
        init
    },
    {
        let mut init = cpVect {
            x: 148.35f64,
            y: 435.00f64,
        };
        init
    },
    {
        let mut init = cpVect {
            x: 149.79f64,
            y: 418.00f64,
        };
        init
    },
    {
        let mut init = cpVect {
            x: 157.72f64,
            y: 401.00f64,
        };
        init
    },
    {
        let mut init = cpVect {
            x: 161.00f64,
            y: 396.53f64,
        };
        init
    },
    {
        let mut init = cpVect {
            x: 177.00f64,
            y: 385.00f64,
        };
        init
    },
    {
        let mut init = cpVect {
            x: 180.14f64,
            y: 380.00f64,
        };
        init
    },
    {
        let mut init = cpVect {
            x: 181.11f64,
            y: 374.00f64,
        };
        init
    },
    {
        let mut init = cpVect {
            x: 180.00f64,
            y: 370.52f64,
        };
        init
    },
    {
        let mut init = cpVect {
            x: 170.00f64,
            y: 371.68f64,
        };
        init
    },
    {
        let mut init = cpVect {
            x: 162.72f64,
            y: 368.00f64,
        };
        init
    },
    {
        let mut init = cpVect {
            x: 158.48f64,
            y: 361.00f64,
        };
        init
    },
    {
        let mut init = cpVect {
            x: 159.56f64,
            y: 349.00f64,
        };
        init
    },
    {
        let mut init = cpVect {
            x: 154.00f64,
            y: 342.53f64,
        };
        init
    },
    {
        let mut init = cpVect {
            x: 146.00f64,
            y: 339.85f64,
        };
        init
    },
    {
        let mut init = cpVect {
            x: 136.09f64,
            y: 343.00f64,
        };
        init
    },
    {
        let mut init = cpVect {
            x: 130.64f64,
            y: 351.00f64,
        };
        init
    },
    {
        let mut init = cpVect {
            x: 131.74f64,
            y: 362.00f64,
        };
        init
    },
    {
        let mut init = cpVect {
            x: 140.61f64,
            y: 374.00f64,
        };
        init
    },
    {
        let mut init = cpVect {
            x: 130.68f64,
            y: 387.00f64,
        };
        init
    },
    {
        let mut init = cpVect {
            x: 120.75f64,
            y: 409.00f64,
        };
        init
    },
    {
        let mut init = cpVect {
            x: 118.09f64,
            y: 421.00f64,
        };
        init
    },
    {
        let mut init = cpVect {
            x: 117.92f64,
            y: 434.00f64,
        };
        init
    },
    {
        let mut init = cpVect {
            x: 100.00f64,
            y: 432.40f64,
        };
        init
    },
    {
        let mut init = cpVect {
            x: 87.00f64,
            y: 427.48f64,
        };
        init
    },
    {
        let mut init = cpVect {
            x: 81.59f64,
            y: 423.00f64,
        };
        init
    },
    {
        let mut init = cpVect {
            x: 73.64f64,
            y: 409.00f64,
        };
        init
    },
    {
        let mut init = cpVect {
            x: 72.57f64,
            y: 398.00f64,
        };
        init
    },
    {
        let mut init = cpVect {
            x: 74.62f64,
            y: 386.00f64,
        };
        init
    },
    {
        let mut init = cpVect {
            x: 78.80f64,
            y: 378.00f64,
        };
        init
    },
    {
        let mut init = cpVect {
            x: 88.00f64,
            y: 373.43f64,
        };
        init
    },
    {
        let mut init = cpVect {
            x: 92.49f64,
            y: 367.00f64,
        };
        init
    },
    {
        let mut init = cpVect {
            x: 93.32f64,
            y: 360.00f64,
        };
        init
    },
    {
        let mut init = cpVect {
            x: 91.30f64,
            y: 353.00f64,
        };
        init
    },
    {
        let mut init = cpVect {
            x: 103.00f64,
            y: 342.67f64,
        };
        init
    },
    {
        let mut init = cpVect {
            x: 109.00f64,
            y: 343.10f64,
        };
        init
    },
    {
        let mut init = cpVect {
            x: 116.00f64,
            y: 340.44f64,
        };
        init
    },
    {
        let mut init = cpVect {
            x: 127.33f64,
            y: 330.00f64,
        };
        init
    },
    {
        let mut init = cpVect {
            x: 143.00f64,
            y: 327.24f64,
        };
        init
    },
    {
        let mut init = cpVect {
            x: 154.30f64,
            y: 322.00f64,
        };
        init
    },
    {
        let mut init = cpVect {
            x: 145.00f64,
            y: 318.06f64,
        };
        init
    },
    {
        let mut init = cpVect {
            x: 139.77f64,
            y: 311.00f64,
        };
        init
    },
    {
        let mut init = cpVect {
            x: 139.48f64,
            y: 302.00f64,
        };
        init
    },
    {
        let mut init = cpVect {
            x: 144.95f64,
            y: 293.00f64,
        };
        init
    },
    {
        let mut init = cpVect {
            x: 143.00f64,
            y: 291.56f64,
        };
        init
    },
    {
        let mut init = cpVect {
            x: 134.00f64,
            y: 298.21f64,
        };
        init
    },
    {
        let mut init = cpVect {
            x: 118.00f64,
            y: 300.75f64,
        };
        init
    },
    {
        let mut init = cpVect {
            x: 109.40f64,
            y: 305.00f64,
        };
        init
    },
    {
        let mut init = cpVect {
            x: 94.67f64,
            y: 319.00f64,
        };
        init
    },
    {
        let mut init = cpVect {
            x: 88.00f64,
            y: 318.93f64,
        };
        init
    },
    {
        let mut init = cpVect {
            x: 81.00f64,
            y: 321.69f64,
        };
        init
    },
    {
        let mut init = cpVect {
            x: 67.24f64,
            y: 333.00f64,
        };
        init
    },
    {
        let mut init = cpVect {
            x: 56.68f64,
            y: 345.00f64,
        };
        init
    },
    {
        let mut init = cpVect {
            x: 53.00f64,
            y: 351.40f64,
        };
        init
    },
    {
        let mut init = cpVect {
            x: 47.34f64,
            y: 333.00f64,
        };
        init
    },
    {
        let mut init = cpVect {
            x: 50.71f64,
            y: 314.00f64,
        };
        init
    },
    {
        let mut init = cpVect {
            x: 56.57f64,
            y: 302.00f64,
        };
        init
    },
    {
        let mut init = cpVect {
            x: 68.00f64,
            y: 287.96f64,
        };
        init
    },
    {
        let mut init = cpVect {
            x: 91.00f64,
            y: 287.24f64,
        };
        init
    },
    {
        let mut init = cpVect {
            x: 110.00f64,
            y: 282.36f64,
        };
        init
    },
    {
        let mut init = cpVect {
            x: 133.80f64,
            y: 271.00f64,
        };
        init
    },
    {
        let mut init = cpVect {
            x: 147.34f64,
            y: 256.00f64,
        };
        init
    },
    {
        let mut init = cpVect {
            x: 156.47f64,
            y: 251.00f64,
        };
        init
    },
    {
        let mut init = cpVect {
            x: 157.26f64,
            y: 250.00f64,
        };
        init
    },
    {
        let mut init = cpVect {
            x: 154.18f64,
            y: 242.00f64,
        };
        init
    },
    {
        let mut init = cpVect {
            x: 154.48f64,
            y: 236.00f64,
        };
        init
    },
    {
        let mut init = cpVect {
            x: 158.72f64,
            y: 229.00f64,
        };
        init
    },
    {
        let mut init = cpVect {
            x: 166.71f64,
            y: 224.00f64,
        };
        init
    },
    {
        let mut init = cpVect {
            x: 170.15f64,
            y: 206.00f64,
        };
        init
    },
    {
        let mut init = cpVect {
            x: 170.19f64,
            y: 196.00f64,
        };
        init
    },
    {
        let mut init = cpVect {
            x: 167.24f64,
            y: 188.00f64,
        };
        init
    },
    {
        let mut init = cpVect {
            x: 160.00f64,
            y: 182.67f64,
        };
        init
    },
    {
        let mut init = cpVect {
            x: 150.00f64,
            y: 182.66f64,
        };
        init
    },
    {
        let mut init = cpVect {
            x: 143.60f64,
            y: 187.00f64,
        };
        init
    },
    {
        let mut init = cpVect {
            x: 139.96f64,
            y: 195.00f64,
        };
        init
    },
    {
        let mut init = cpVect {
            x: 139.50f64,
            y: 207.00f64,
        };
        init
    },
    {
        let mut init = cpVect {
            x: 136.45f64,
            y: 221.00f64,
        };
        init
    },
    {
        let mut init = cpVect {
            x: 136.52f64,
            y: 232.00f64,
        };
        init
    },
    {
        let mut init = cpVect {
            x: 133.28f64,
            y: 238.00f64,
        };
        init
    },
    {
        let mut init = cpVect {
            x: 129.00f64,
            y: 241.38f64,
        };
        init
    },
    {
        let mut init = cpVect {
            x: 119.00f64,
            y: 243.07f64,
        };
        init
    },
    {
        let mut init = cpVect {
            x: 115.00f64,
            y: 246.55f64,
        };
        init
    },
    {
        let mut init = cpVect {
            x: 101.00f64,
            y: 253.16f64,
        };
        init
    },
    {
        let mut init = cpVect {
            x: 86.00f64,
            y: 257.32f64,
        };
        init
    },
    {
        let mut init = cpVect {
            x: 63.00f64,
            y: 259.24f64,
        };
        init
    },
    {
        let mut init = cpVect {
            x: 57.00f64,
            y: 257.31f64,
        };
        init
    },
    {
        let mut init = cpVect {
            x: 50.54f64,
            y: 252.00f64,
        };
        init
    },
    {
        let mut init = cpVect {
            x: 47.59f64,
            y: 247.00f64,
        };
        init
    },
    {
        let mut init = cpVect {
            x: 46.30f64,
            y: 240.00f64,
        };
        init
    },
    {
        let mut init = cpVect {
            x: 47.58f64,
            y: 226.00f64,
        };
        init
    },
    {
        let mut init = cpVect {
            x: 50.00f64,
            y: 220.57f64,
        };
        init
    },
    {
        let mut init = cpVect {
            x: 58.00f64,
            y: 226.41f64,
        };
        init
    },
    {
        let mut init = cpVect {
            x: 69.00f64,
            y: 229.17f64,
        };
        init
    },
    {
        let mut init = cpVect {
            x: 79.00f64,
            y: 229.08f64,
        };
        init
    },
    {
        let mut init = cpVect {
            x: 94.50f64,
            y: 225.00f64,
        };
        init
    },
    {
        let mut init = cpVect {
            x: 100.21f64,
            y: 231.00f64,
        };
        init
    },
    {
        let mut init = cpVect {
            x: 107.00f64,
            y: 233.47f64,
        };
        init
    },
    {
        let mut init = cpVect {
            x: 107.48f64,
            y: 224.00f64,
        };
        init
    },
    {
        let mut init = cpVect {
            x: 109.94f64,
            y: 219.00f64,
        };
        init
    },
    {
        let mut init = cpVect {
            x: 115.00f64,
            y: 214.62f64,
        };
        init
    },
    {
        let mut init = cpVect {
            x: 122.57f64,
            y: 212.00f64,
        };
        init
    },
    {
        let mut init = cpVect {
            x: 116.00f64,
            y: 201.49f64,
        };
        init
    },
    {
        let mut init = cpVect {
            x: 104.00f64,
            y: 194.57f64,
        };
        init
    },
    {
        let mut init = cpVect {
            x: 90.00f64,
            y: 194.04f64,
        };
        init
    },
    {
        let mut init = cpVect {
            x: 79.00f64,
            y: 198.21f64,
        };
        init
    },
    {
        let mut init = cpVect {
            x: 73.00f64,
            y: 198.87f64,
        };
        init
    },
    {
        let mut init = cpVect {
            x: 62.68f64,
            y: 191.00f64,
        };
        init
    },
    {
        let mut init = cpVect {
            x: 62.58f64,
            y: 184.00f64,
        };
        init
    },
    {
        let mut init = cpVect {
            x: 64.42f64,
            y: 179.00f64,
        };
        init
    },
    {
        let mut init = cpVect {
            x: 75.00f64,
            y: 167.70f64,
        };
        init
    },
    {
        let mut init = cpVect {
            x: 80.39f64,
            y: 157.00f64,
        };
        init
    },
    {
        let mut init = cpVect {
            x: 68.79f64,
            y: 140.00f64,
        };
        init
    },
    {
        let mut init = cpVect {
            x: 61.67f64,
            y: 126.00f64,
        };
        init
    },
    {
        let mut init = cpVect {
            x: 61.47f64,
            y: 117.00f64,
        };
        init
    },
    {
        let mut init = cpVect {
            x: 64.43f64,
            y: 109.00f64,
        };
        init
    },
    {
        let mut init = cpVect { x: 63.10f64, y: 96.00f64 };
        init
    },
    {
        let mut init = cpVect { x: 56.48f64, y: 82.00f64 };
        init
    },
    {
        let mut init = cpVect { x: 48.00f64, y: 73.88f64 };
        init
    },
    {
        let mut init = cpVect { x: 43.81f64, y: 66.00f64 };
        init
    },
    {
        let mut init = cpVect { x: 43.81f64, y: 56.00f64 };
        init
    },
    {
        let mut init = cpVect { x: 50.11f64, y: 46.00f64 };
        init
    },
    {
        let mut init = cpVect { x: 59.00f64, y: 41.55f64 };
        init
    },
    {
        let mut init = cpVect { x: 71.00f64, y: 42.64f64 };
        init
    },
    {
        let mut init = cpVect { x: 78.00f64, y: 36.77f64 };
        init
    },
    {
        let mut init = cpVect { x: 83.00f64, y: 34.75f64 };
        init
    },
    {
        let mut init = cpVect { x: 99.00f64, y: 34.32f64 };
        init
    },
    {
        let mut init = cpVect {
            x: 117.00f64,
            y: 38.92f64,
        };
        init
    },
    {
        let mut init = cpVect {
            x: 133.00f64,
            y: 55.15f64,
        };
        init
    },
    {
        let mut init = cpVect {
            x: 142.00f64,
            y: 50.70f64,
        };
        init
    },
    {
        let mut init = cpVect {
            x: 149.74f64,
            y: 51.00f64,
        };
        init
    },
    {
        let mut init = cpVect {
            x: 143.55f64,
            y: 68.00f64,
        };
        init
    },
    {
        let mut init = cpVect {
            x: 153.28f64,
            y: 74.00f64,
        };
        init
    },
    {
        let mut init = cpVect {
            x: 156.23f64,
            y: 79.00f64,
        };
        init
    },
    {
        let mut init = cpVect {
            x: 157.00f64,
            y: 84.00f64,
        };
        init
    },
    {
        let mut init = cpVect {
            x: 156.23f64,
            y: 89.00f64,
        };
        init
    },
    {
        let mut init = cpVect {
            x: 153.28f64,
            y: 94.00f64,
        };
        init
    },
    {
        let mut init = cpVect {
            x: 144.58f64,
            y: 99.00f64,
        };
        init
    },
    {
        let mut init = cpVect {
            x: 151.52f64,
            y: 112.00f64,
        };
        init
    },
    {
        let mut init = cpVect {
            x: 151.51f64,
            y: 124.00f64,
        };
        init
    },
    {
        let mut init = cpVect {
            x: 150.00f64,
            y: 126.36f64,
        };
        init
    },
    {
        let mut init = cpVect {
            x: 133.00f64,
            y: 130.25f64,
        };
        init
    },
    {
        let mut init = cpVect {
            x: 126.71f64,
            y: 125.00f64,
        };
        init
    },
    {
        let mut init = cpVect {
            x: 122.00f64,
            y: 117.25f64,
        };
        init
    },
    {
        let mut init = cpVect {
            x: 114.00f64,
            y: 116.23f64,
        };
        init
    },
    {
        let mut init = cpVect {
            x: 107.73f64,
            y: 112.00f64,
        };
        init
    },
    {
        let mut init = cpVect {
            x: 104.48f64,
            y: 106.00f64,
        };
        init
    },
    {
        let mut init = cpVect {
            x: 104.32f64,
            y: 99.00f64,
        };
        init
    },
    {
        let mut init = cpVect {
            x: 106.94f64,
            y: 93.00f64,
        };
        init
    },
    {
        let mut init = cpVect {
            x: 111.24f64,
            y: 89.00f64,
        };
        init
    },
    {
        let mut init = cpVect {
            x: 111.60f64,
            y: 85.00f64,
        };
        init
    },
    {
        let mut init = cpVect {
            x: 107.24f64,
            y: 73.00f64,
        };
        init
    },
    {
        let mut init = cpVect {
            x: 102.00f64,
            y: 67.57f64,
        };
        init
    },
    {
        let mut init = cpVect { x: 99.79f64, y: 67.00f64 };
        init
    },
    {
        let mut init = cpVect { x: 99.23f64, y: 76.00f64 };
        init
    },
    {
        let mut init = cpVect { x: 95.00f64, y: 82.27f64 };
        init
    },
    {
        let mut init = cpVect { x: 89.00f64, y: 85.52f64 };
        init
    },
    {
        let mut init = cpVect { x: 79.84f64, y: 86.00f64 };
        init
    },
    {
        let mut init = cpVect {
            x: 86.73f64,
            y: 114.00f64,
        };
        init
    },
    {
        let mut init = cpVect {
            x: 98.00f64,
            y: 136.73f64,
        };
        init
    },
    {
        let mut init = cpVect {
            x: 99.00f64,
            y: 137.61f64,
        };
        init
    },
    {
        let mut init = cpVect {
            x: 109.00f64,
            y: 135.06f64,
        };
        init
    },
    {
        let mut init = cpVect {
            x: 117.00f64,
            y: 137.94f64,
        };
        init
    },
    {
        let mut init = cpVect {
            x: 122.52f64,
            y: 146.00f64,
        };
        init
    },
    {
        let mut init = cpVect {
            x: 122.94f64,
            y: 151.00f64,
        };
        init
    },
    {
        let mut init = cpVect {
            x: 121.00f64,
            y: 158.58f64,
        };
        init
    },
    {
        let mut init = cpVect {
            x: 134.00f64,
            y: 160.97f64,
        };
        init
    },
    {
        let mut init = cpVect {
            x: 153.00f64,
            y: 157.45f64,
        };
        init
    },
    {
        let mut init = cpVect {
            x: 171.30f64,
            y: 150.00f64,
        };
        init
    },
    {
        let mut init = cpVect {
            x: 169.06f64,
            y: 142.00f64,
        };
        init
    },
    {
        let mut init = cpVect {
            x: 169.77f64,
            y: 136.00f64,
        };
        init
    },
    {
        let mut init = cpVect {
            x: 174.00f64,
            y: 129.73f64,
        };
        init
    },
    {
        let mut init = cpVect {
            x: 181.46f64,
            y: 126.00f64,
        };
        init
    },
    {
        let mut init = cpVect {
            x: 182.22f64,
            y: 120.00f64,
        };
        init
    },
    {
        let mut init = cpVect {
            x: 182.20f64,
            y: 111.00f64,
        };
        init
    },
    {
        let mut init = cpVect {
            x: 180.06f64,
            y: 101.00f64,
        };
        init
    },
    {
        let mut init = cpVect {
            x: 171.28f64,
            y: 85.00f64,
        };
        init
    },
    {
        let mut init = cpVect {
            x: 171.75f64,
            y: 80.00f64,
        };
        init
    },
    {
        let mut init = cpVect {
            x: 182.30f64,
            y: 53.00f64,
        };
        init
    },
    {
        let mut init = cpVect {
            x: 189.47f64,
            y: 50.00f64,
        };
        init
    },
    {
        let mut init = cpVect {
            x: 190.62f64,
            y: 38.00f64,
        };
        init
    },
    {
        let mut init = cpVect {
            x: 194.00f64,
            y: 33.73f64,
        };
        init
    },
    {
        let mut init = cpVect {
            x: 199.00f64,
            y: 30.77f64,
        };
        init
    },
    {
        let mut init = cpVect {
            x: 208.00f64,
            y: 30.48f64,
        };
        init
    },
    {
        let mut init = cpVect {
            x: 216.00f64,
            y: 34.94f64,
        };
        init
    },
    {
        let mut init = cpVect {
            x: 224.00f64,
            y: 31.47f64,
        };
        init
    },
    {
        let mut init = cpVect {
            x: 240.00f64,
            y: 30.37f64,
        };
        init
    },
    {
        let mut init = cpVect {
            x: 247.00f64,
            y: 32.51f64,
        };
        init
    },
    {
        let mut init = cpVect {
            x: 249.77f64,
            y: 35.00f64,
        };
        init
    },
    {
        let mut init = cpVect {
            x: 234.75f64,
            y: 53.00f64,
        };
        init
    },
    {
        let mut init = cpVect {
            x: 213.81f64,
            y: 93.00f64,
        };
        init
    },
    {
        let mut init = cpVect {
            x: 212.08f64,
            y: 99.00f64,
        };
        init
    },
    {
        let mut init = cpVect {
            x: 213.00f64,
            y: 101.77f64,
        };
        init
    },
    {
        let mut init = cpVect {
            x: 220.00f64,
            y: 96.77f64,
        };
        init
    },
    {
        let mut init = cpVect {
            x: 229.00f64,
            y: 96.48f64,
        };
        init
    },
    {
        let mut init = cpVect {
            x: 236.28f64,
            y: 101.00f64,
        };
        init
    },
    {
        let mut init = cpVect {
            x: 240.00f64,
            y: 107.96f64,
        };
        init
    },
    {
        let mut init = cpVect {
            x: 245.08f64,
            y: 101.00f64,
        };
        init
    },
    {
        let mut init = cpVect {
            x: 263.00f64,
            y: 65.32f64,
        };
        init
    },
    {
        let mut init = cpVect {
            x: 277.47f64,
            y: 48.00f64,
        };
        init
    },
    {
        let mut init = cpVect {
            x: 284.00f64,
            y: 47.03f64,
        };
        init
    },
    {
        let mut init = cpVect {
            x: 286.94f64,
            y: 41.00f64,
        };
        init
    },
    {
        let mut init = cpVect {
            x: 292.00f64,
            y: 36.62f64,
        };
        init
    },
    {
        let mut init = cpVect {
            x: 298.00f64,
            y: 35.06f64,
        };
        init
    },
    {
        let mut init = cpVect {
            x: 304.00f64,
            y: 35.77f64,
        };
        init
    },
    {
        let mut init = cpVect {
            x: 314.00f64,
            y: 43.81f64,
        };
        init
    },
    {
        let mut init = cpVect {
            x: 342.00f64,
            y: 32.56f64,
        };
        init
    },
    {
        let mut init = cpVect {
            x: 359.00f64,
            y: 31.32f64,
        };
        init
    },
    {
        let mut init = cpVect {
            x: 365.00f64,
            y: 32.57f64,
        };
        init
    },
    {
        let mut init = cpVect {
            x: 371.00f64,
            y: 36.38f64,
        };
        init
    },
    {
        let mut init = cpVect {
            x: 379.53f64,
            y: 48.00f64,
        };
        init
    },
    {
        let mut init = cpVect {
            x: 379.70f64,
            y: 51.00f64,
        };
        init
    },
    {
        let mut init = cpVect {
            x: 356.00f64,
            y: 52.19f64,
        };
        init
    },
    {
        let mut init = cpVect {
            x: 347.00f64,
            y: 54.74f64,
        };
        init
    },
    {
        let mut init = cpVect {
            x: 344.38f64,
            y: 66.00f64,
        };
        init
    },
    {
        let mut init = cpVect {
            x: 341.00f64,
            y: 70.27f64,
        };
        init
    },
    {
        let mut init = cpVect {
            x: 335.00f64,
            y: 73.52f64,
        };
        init
    },
    {
        let mut init = cpVect {
            x: 324.00f64,
            y: 72.38f64,
        };
        init
    },
    {
        let mut init = cpVect {
            x: 317.00f64,
            y: 65.75f64,
        };
        init
    },
    {
        let mut init = cpVect {
            x: 313.00f64,
            y: 67.79f64,
        };
        init
    },
    {
        let mut init = cpVect {
            x: 307.57f64,
            y: 76.00f64,
        };
        init
    },
    {
        let mut init = cpVect {
            x: 315.00f64,
            y: 78.62f64,
        };
        init
    },
    {
        let mut init = cpVect {
            x: 319.28f64,
            y: 82.00f64,
        };
        init
    },
    {
        let mut init = cpVect {
            x: 322.23f64,
            y: 87.00f64,
        };
        init
    },
    {
        let mut init = cpVect {
            x: 323.00f64,
            y: 94.41f64,
        };
        init
    },
    {
        let mut init = cpVect {
            x: 334.00f64,
            y: 92.49f64,
        };
        init
    },
    {
        let mut init = cpVect {
            x: 347.00f64,
            y: 87.47f64,
        };
        init
    },
    {
        let mut init = cpVect {
            x: 349.62f64,
            y: 80.00f64,
        };
        init
    },
    {
        let mut init = cpVect {
            x: 353.00f64,
            y: 75.73f64,
        };
        init
    },
    {
        let mut init = cpVect {
            x: 359.00f64,
            y: 72.48f64,
        };
        init
    },
    {
        let mut init = cpVect {
            x: 366.00f64,
            y: 72.32f64,
        };
        init
    },
    {
        let mut init = cpVect {
            x: 372.00f64,
            y: 74.94f64,
        };
        init
    },
    {
        let mut init = cpVect {
            x: 377.00f64,
            y: 81.34f64,
        };
        init
    },
    {
        let mut init = cpVect {
            x: 382.00f64,
            y: 83.41f64,
        };
        init
    },
    {
        let mut init = cpVect {
            x: 392.00f64,
            y: 83.40f64,
        };
        init
    },
    {
        let mut init = cpVect {
            x: 399.00f64,
            y: 79.15f64,
        };
        init
    },
    {
        let mut init = cpVect {
            x: 404.00f64,
            y: 85.74f64,
        };
        init
    },
    {
        let mut init = cpVect {
            x: 411.00f64,
            y: 85.06f64,
        };
        init
    },
    {
        let mut init = cpVect {
            x: 417.00f64,
            y: 86.62f64,
        };
        init
    },
    {
        let mut init = cpVect {
            x: 423.38f64,
            y: 93.00f64,
        };
        init
    },
    {
        let mut init = cpVect {
            x: 425.05f64,
            y: 104.00f64,
        };
        init
    },
    {
        let mut init = cpVect {
            x: 438.00f64,
            y: 110.35f64,
        };
        init
    },
    {
        let mut init = cpVect {
            x: 450.00f64,
            y: 112.17f64,
        };
        init
    },
    {
        let mut init = cpVect {
            x: 452.62f64,
            y: 103.00f64,
        };
        init
    },
    {
        let mut init = cpVect {
            x: 456.00f64,
            y: 98.73f64,
        };
        init
    },
    {
        let mut init = cpVect {
            x: 462.00f64,
            y: 95.48f64,
        };
        init
    },
    {
        let mut init = cpVect {
            x: 472.00f64,
            y: 95.79f64,
        };
        init
    },
    {
        let mut init = cpVect {
            x: 471.28f64,
            y: 92.00f64,
        };
        init
    },
    {
        let mut init = cpVect {
            x: 464.00f64,
            y: 84.62f64,
        };
        init
    },
    {
        let mut init = cpVect {
            x: 445.00f64,
            y: 80.39f64,
        };
        init
    },
    {
        let mut init = cpVect {
            x: 436.00f64,
            y: 75.33f64,
        };
        init
    },
    {
        let mut init = cpVect {
            x: 428.00f64,
            y: 68.46f64,
        };
        init
    },
    {
        let mut init = cpVect {
            x: 419.00f64,
            y: 68.52f64,
        };
        init
    },
    {
        let mut init = cpVect {
            x: 413.00f64,
            y: 65.27f64,
        };
        init
    },
    {
        let mut init = cpVect {
            x: 408.48f64,
            y: 58.00f64,
        };
        init
    },
    {
        let mut init = cpVect {
            x: 409.87f64,
            y: 46.00f64,
        };
        init
    },
    {
        let mut init = cpVect {
            x: 404.42f64,
            y: 39.00f64,
        };
        init
    },
    {
        let mut init = cpVect {
            x: 408.00f64,
            y: 33.88f64,
        };
        init
    },
    {
        let mut init = cpVect {
            x: 415.00f64,
            y: 29.31f64,
        };
        init
    },
    {
        let mut init = cpVect {
            x: 429.00f64,
            y: 26.45f64,
        };
        init
    },
    {
        let mut init = cpVect {
            x: 455.00f64,
            y: 28.77f64,
        };
        init
    },
    {
        let mut init = cpVect {
            x: 470.00f64,
            y: 33.81f64,
        };
        init
    },
    {
        let mut init = cpVect {
            x: 482.00f64,
            y: 42.16f64,
        };
        init
    },
    {
        let mut init = cpVect {
            x: 494.00f64,
            y: 46.85f64,
        };
        init
    },
    {
        let mut init = cpVect {
            x: 499.65f64,
            y: 36.00f64,
        };
        init
    },
    {
        let mut init = cpVect {
            x: 513.00f64,
            y: 25.95f64,
        };
        init
    },
    {
        let mut init = cpVect {
            x: 529.00f64,
            y: 22.42f64,
        };
        init
    },
    {
        let mut init = cpVect {
            x: 537.18f64,
            y: 23.00f64,
        };
        init
    },
];
static mut bouncy_terrain_count: libc::c_int = 0;
unsafe extern "C" fn init_BouncyTerrainCircles_500() -> *mut cpSpace {
    let mut space: *mut cpSpace = cpSpaceNew();
    cpSpaceSetIterations(space, 10 as libc::c_int);
    let mut offset: cpVect = cpv(
        -(320 as libc::c_int) as cpFloat,
        -(240 as libc::c_int) as cpFloat,
    );
    let mut i: libc::c_int = 0 as libc::c_int;
    while i < bouncy_terrain_count - 1 as libc::c_int {
        let mut a: cpVect = bouncy_terrain_verts[i as usize];
        let mut b: cpVect = bouncy_terrain_verts[(i + 1 as libc::c_int) as usize];
        let mut shape: *mut cpShape = cpSpaceAddShape(
            space,
            cpSegmentShapeNew(
                cpSpaceGetStaticBody(space),
                cpvadd(a, offset),
                cpvadd(b, offset),
                0.0f32 as cpFloat,
            ),
        );
        cpShapeSetElasticity(shape, 1.0f64);
        i += 1;
        i;
    }
    let mut i_0: libc::c_int = 0 as libc::c_int;
    while i_0 < 500 as libc::c_int {
        let mut radius: cpFloat = 5.0f32 as cpFloat;
        let mut mass: cpFloat = radius * radius;
        let mut body: *mut cpBody = cpSpaceAddBody(
            space,
            cpBodyNew(mass, cpMomentForCircle(mass, 0.0f32 as cpFloat, radius, cpvzero)),
        );
        cpBodySetPosition(
            body,
            cpvadd(cpvmult(frand_unit_circle(), 130.0f32 as cpFloat), cpvzero),
        );
        cpBodySetVelocity(body, cpvmult(frand_unit_circle(), 50.0f32 as cpFloat));
        let mut shape_0: *mut cpShape = cpSpaceAddShape(
            space,
            cpCircleShapeNew(body, radius, cpvzero),
        );
        cpShapeSetElasticity(shape_0, 1.0f64);
        i_0 += 1;
        i_0;
    }
    return space;
}
unsafe extern "C" fn init_BouncyTerrainHexagons_500() -> *mut cpSpace {
    let mut space: *mut cpSpace = cpSpaceNew();
    cpSpaceSetIterations(space, 10 as libc::c_int);
    let mut offset: cpVect = cpv(
        -(320 as libc::c_int) as cpFloat,
        -(240 as libc::c_int) as cpFloat,
    );
    let mut i: libc::c_int = 0 as libc::c_int;
    while i < bouncy_terrain_count - 1 as libc::c_int {
        let mut a: cpVect = bouncy_terrain_verts[i as usize];
        let mut b: cpVect = bouncy_terrain_verts[(i + 1 as libc::c_int) as usize];
        let mut shape: *mut cpShape = cpSpaceAddShape(
            space,
            cpSegmentShapeNew(
                cpSpaceGetStaticBody(space),
                cpvadd(a, offset),
                cpvadd(b, offset),
                0.0f32 as cpFloat,
            ),
        );
        cpShapeSetElasticity(shape, 1.0f64);
        i += 1;
        i;
    }
    let mut radius: cpFloat = 5.0f32 as cpFloat;
    let mut hexagon: [cpVect; 6] = [cpVect { x: 0., y: 0. }; 6];
    let mut i_0: libc::c_int = 0 as libc::c_int;
    while i_0 < 6 as libc::c_int {
        let mut angle: cpFloat = -3.14159265358979323846264338327950288f64
            * 2.0f32 as libc::c_double * i_0 as libc::c_double
            / 6.0f32 as libc::c_double;
        hexagon[i_0 as usize] = cpvmult(cpv(cos(angle), sin(angle)), radius - bevel);
        i_0 += 1;
        i_0;
    }
    let mut i_1: libc::c_int = 0 as libc::c_int;
    while i_1 < 500 as libc::c_int {
        let mut mass: cpFloat = radius * radius;
        let mut body: *mut cpBody = cpSpaceAddBody(
            space,
            cpBodyNew(
                mass,
                cpMomentForPoly(
                    mass,
                    6 as libc::c_int,
                    hexagon.as_mut_ptr(),
                    cpvzero,
                    0.0f32 as cpFloat,
                ),
            ),
        );
        cpBodySetPosition(
            body,
            cpvadd(cpvmult(frand_unit_circle(), 130.0f32 as cpFloat), cpvzero),
        );
        cpBodySetVelocity(body, cpvmult(frand_unit_circle(), 50.0f32 as cpFloat));
        let mut shape_0: *mut cpShape = cpSpaceAddShape(
            space,
            cpPolyShapeNew(
                body,
                6 as libc::c_int,
                hexagon.as_mut_ptr(),
                cpTransformIdentity,
                bevel,
            ),
        );
        cpShapeSetElasticity(shape_0, 1.0f64);
        i_1 += 1;
        i_1;
    }
    return space;
}
unsafe extern "C" fn NoCollide_begin(
    mut arb: *mut cpArbiter,
    mut space: *mut cpSpace,
    mut data: *mut libc::c_void,
) -> cpBool {
    abort();
}
unsafe extern "C" fn init_NoCollide() -> *mut cpSpace {
    let mut space: *mut cpSpace = cpSpaceNew();
    cpSpaceSetIterations(space, 10 as libc::c_int);
    let mut handler: *mut cpCollisionHandler = cpSpaceAddCollisionHandler(
        space,
        2 as libc::c_int as cpCollisionType,
        2 as libc::c_int as cpCollisionType,
    );
    (*handler)
        .beginFunc = Some(
        NoCollide_begin
            as unsafe extern "C" fn(
                *mut cpArbiter,
                *mut cpSpace,
                *mut libc::c_void,
            ) -> cpBool,
    );
    let mut radius: libc::c_float = 4.5f32;
    cpShapeSetElasticity(
        cpSpaceAddShape(
            space,
            cpSegmentShapeNew(
                cpSpaceGetStaticBody(space),
                cpv(
                    (-(330 as libc::c_int) as libc::c_float - radius) as cpFloat,
                    (-(250 as libc::c_int) as libc::c_float - radius) as cpFloat,
                ),
                cpv(
                    (330 as libc::c_int as libc::c_float + radius) as cpFloat,
                    (-(250 as libc::c_int) as libc::c_float - radius) as cpFloat,
                ),
                0.0f32 as cpFloat,
            ),
        ),
        1.0f32 as cpFloat,
    );
    cpShapeSetElasticity(
        cpSpaceAddShape(
            space,
            cpSegmentShapeNew(
                cpSpaceGetStaticBody(space),
                cpv(
                    (330 as libc::c_int as libc::c_float + radius) as cpFloat,
                    (250 as libc::c_int as libc::c_float + radius) as cpFloat,
                ),
                cpv(
                    (330 as libc::c_int as libc::c_float + radius) as cpFloat,
                    (-(250 as libc::c_int) as libc::c_float - radius) as cpFloat,
                ),
                0.0f32 as cpFloat,
            ),
        ),
        1.0f32 as cpFloat,
    );
    cpShapeSetElasticity(
        cpSpaceAddShape(
            space,
            cpSegmentShapeNew(
                cpSpaceGetStaticBody(space),
                cpv(
                    (330 as libc::c_int as libc::c_float + radius) as cpFloat,
                    (250 as libc::c_int as libc::c_float + radius) as cpFloat,
                ),
                cpv(
                    (-(330 as libc::c_int) as libc::c_float - radius) as cpFloat,
                    (250 as libc::c_int as libc::c_float + radius) as cpFloat,
                ),
                0.0f32 as cpFloat,
            ),
        ),
        1.0f32 as cpFloat,
    );
    cpShapeSetElasticity(
        cpSpaceAddShape(
            space,
            cpSegmentShapeNew(
                cpSpaceGetStaticBody(space),
                cpv(
                    (-(330 as libc::c_int) as libc::c_float - radius) as cpFloat,
                    (-(250 as libc::c_int) as libc::c_float - radius) as cpFloat,
                ),
                cpv(
                    (-(330 as libc::c_int) as libc::c_float - radius) as cpFloat,
                    (250 as libc::c_int as libc::c_float + radius) as cpFloat,
                ),
                0.0f32 as cpFloat,
            ),
        ),
        1.0f32 as cpFloat,
    );
    let mut x: libc::c_int = -(320 as libc::c_int);
    while x <= 320 as libc::c_int {
        let mut y: libc::c_int = -(240 as libc::c_int);
        while y <= 240 as libc::c_int {
            cpSpaceAddShape(
                space,
                cpCircleShapeNew(
                    cpSpaceGetStaticBody(space),
                    radius as cpFloat,
                    cpv(x as cpFloat, y as cpFloat),
                ),
            );
            y += 20 as libc::c_int;
        }
        x += 20 as libc::c_int;
    }
    let mut y_0: libc::c_int = 10 as libc::c_int - 240 as libc::c_int;
    while y_0 <= 240 as libc::c_int {
        let mut mass: cpFloat = 7.0f32 as cpFloat;
        let mut body: *mut cpBody = cpSpaceAddBody(
            space,
            cpBodyNew(
                mass,
                cpMomentForCircle(mass, 0.0f32 as cpFloat, radius as cpFloat, cpvzero),
            ),
        );
        cpBodySetPosition(body, cpv(-320.0f32 as cpFloat, y_0 as cpFloat));
        cpBodySetVelocity(body, cpv(100.0f32 as cpFloat, 0.0f32 as cpFloat));
        let mut shape: *mut cpShape = cpSpaceAddShape(
            space,
            cpCircleShapeNew(body, radius as cpFloat, cpvzero),
        );
        cpShapeSetElasticity(shape, 1.0f64);
        cpShapeSetCollisionType(shape, 2 as libc::c_int as cpCollisionType);
        y_0 += 40 as libc::c_int;
    }
    let mut x_0: libc::c_int = 30 as libc::c_int - 320 as libc::c_int;
    while x_0 <= 320 as libc::c_int {
        let mut mass_0: cpFloat = 7.0f32 as cpFloat;
        let mut body_0: *mut cpBody = cpSpaceAddBody(
            space,
            cpBodyNew(
                mass_0,
                cpMomentForCircle(mass_0, 0.0f32 as cpFloat, radius as cpFloat, cpvzero),
            ),
        );
        cpBodySetPosition(body_0, cpv(x_0 as cpFloat, -240.0f32 as cpFloat));
        cpBodySetVelocity(body_0, cpv(0.0f32 as cpFloat, 100.0f32 as cpFloat));
        let mut shape_0: *mut cpShape = cpSpaceAddShape(
            space,
            cpCircleShapeNew(body_0, radius as cpFloat, cpvzero),
        );
        cpShapeSetElasticity(shape_0, 1.0f64);
        cpShapeSetCollisionType(shape_0, 2 as libc::c_int as cpCollisionType);
        x_0 += 40 as libc::c_int;
    }
    return space;
}
unsafe extern "C" fn update(mut space: *mut cpSpace, mut dt: libc::c_double) {
    cpSpaceStep(space, dt);
}
unsafe extern "C" fn destroy(mut space: *mut cpSpace) {
    ChipmunkDemoFreeSpaceChildren(space);
    cpSpaceFree(space);
}
pub static mut BouncyHexagons: ChipmunkDemo = unsafe {
    {
        let mut init = ChipmunkDemo {
            name: b"Bouncy Hexagons\0" as *const u8 as *const libc::c_char,
            timestep: 1.0f64 / 60.0f64,
            initFunc: Some(
                init_BouncyTerrainHexagons_500 as unsafe extern "C" fn() -> *mut cpSpace,
            ),
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
pub static mut bench_list: [ChipmunkDemo; 17] = unsafe {
    [
        {
            let mut init = ChipmunkDemo {
                name: b"benchmark - SimpleTerrainCircles_1000\0" as *const u8
                    as *const libc::c_char,
                timestep: 1.0f64 / 60.0f64,
                initFunc: Some(
                    init_SimpleTerrainCircles_1000
                        as unsafe extern "C" fn() -> *mut cpSpace,
                ),
                updateFunc: Some(
                    update as unsafe extern "C" fn(*mut cpSpace, libc::c_double) -> (),
                ),
                drawFunc: Some(
                    ChipmunkDemoDefaultDrawImpl
                        as unsafe extern "C" fn(*mut cpSpace) -> (),
                ),
                destroyFunc: Some(destroy as unsafe extern "C" fn(*mut cpSpace) -> ()),
            };
            init
        },
        {
            let mut init = ChipmunkDemo {
                name: b"benchmark - SimpleTerrainCircles_500\0" as *const u8
                    as *const libc::c_char,
                timestep: 1.0f64 / 60.0f64,
                initFunc: Some(
                    init_SimpleTerrainCircles_500
                        as unsafe extern "C" fn() -> *mut cpSpace,
                ),
                updateFunc: Some(
                    update as unsafe extern "C" fn(*mut cpSpace, libc::c_double) -> (),
                ),
                drawFunc: Some(
                    ChipmunkDemoDefaultDrawImpl
                        as unsafe extern "C" fn(*mut cpSpace) -> (),
                ),
                destroyFunc: Some(destroy as unsafe extern "C" fn(*mut cpSpace) -> ()),
            };
            init
        },
        {
            let mut init = ChipmunkDemo {
                name: b"benchmark - SimpleTerrainCircles_100\0" as *const u8
                    as *const libc::c_char,
                timestep: 1.0f64 / 60.0f64,
                initFunc: Some(
                    init_SimpleTerrainCircles_100
                        as unsafe extern "C" fn() -> *mut cpSpace,
                ),
                updateFunc: Some(
                    update as unsafe extern "C" fn(*mut cpSpace, libc::c_double) -> (),
                ),
                drawFunc: Some(
                    ChipmunkDemoDefaultDrawImpl
                        as unsafe extern "C" fn(*mut cpSpace) -> (),
                ),
                destroyFunc: Some(destroy as unsafe extern "C" fn(*mut cpSpace) -> ()),
            };
            init
        },
        {
            let mut init = ChipmunkDemo {
                name: b"benchmark - SimpleTerrainBoxes_1000\0" as *const u8
                    as *const libc::c_char,
                timestep: 1.0f64 / 60.0f64,
                initFunc: Some(
                    init_SimpleTerrainBoxes_1000
                        as unsafe extern "C" fn() -> *mut cpSpace,
                ),
                updateFunc: Some(
                    update as unsafe extern "C" fn(*mut cpSpace, libc::c_double) -> (),
                ),
                drawFunc: Some(
                    ChipmunkDemoDefaultDrawImpl
                        as unsafe extern "C" fn(*mut cpSpace) -> (),
                ),
                destroyFunc: Some(destroy as unsafe extern "C" fn(*mut cpSpace) -> ()),
            };
            init
        },
        {
            let mut init = ChipmunkDemo {
                name: b"benchmark - SimpleTerrainBoxes_500\0" as *const u8
                    as *const libc::c_char,
                timestep: 1.0f64 / 60.0f64,
                initFunc: Some(
                    init_SimpleTerrainBoxes_500 as unsafe extern "C" fn() -> *mut cpSpace,
                ),
                updateFunc: Some(
                    update as unsafe extern "C" fn(*mut cpSpace, libc::c_double) -> (),
                ),
                drawFunc: Some(
                    ChipmunkDemoDefaultDrawImpl
                        as unsafe extern "C" fn(*mut cpSpace) -> (),
                ),
                destroyFunc: Some(destroy as unsafe extern "C" fn(*mut cpSpace) -> ()),
            };
            init
        },
        {
            let mut init = ChipmunkDemo {
                name: b"benchmark - SimpleTerrainBoxes_100\0" as *const u8
                    as *const libc::c_char,
                timestep: 1.0f64 / 60.0f64,
                initFunc: Some(
                    init_SimpleTerrainBoxes_100 as unsafe extern "C" fn() -> *mut cpSpace,
                ),
                updateFunc: Some(
                    update as unsafe extern "C" fn(*mut cpSpace, libc::c_double) -> (),
                ),
                drawFunc: Some(
                    ChipmunkDemoDefaultDrawImpl
                        as unsafe extern "C" fn(*mut cpSpace) -> (),
                ),
                destroyFunc: Some(destroy as unsafe extern "C" fn(*mut cpSpace) -> ()),
            };
            init
        },
        {
            let mut init = ChipmunkDemo {
                name: b"benchmark - SimpleTerrainHexagons_1000\0" as *const u8
                    as *const libc::c_char,
                timestep: 1.0f64 / 60.0f64,
                initFunc: Some(
                    init_SimpleTerrainHexagons_1000
                        as unsafe extern "C" fn() -> *mut cpSpace,
                ),
                updateFunc: Some(
                    update as unsafe extern "C" fn(*mut cpSpace, libc::c_double) -> (),
                ),
                drawFunc: Some(
                    ChipmunkDemoDefaultDrawImpl
                        as unsafe extern "C" fn(*mut cpSpace) -> (),
                ),
                destroyFunc: Some(destroy as unsafe extern "C" fn(*mut cpSpace) -> ()),
            };
            init
        },
        {
            let mut init = ChipmunkDemo {
                name: b"benchmark - SimpleTerrainHexagons_500\0" as *const u8
                    as *const libc::c_char,
                timestep: 1.0f64 / 60.0f64,
                initFunc: Some(
                    init_SimpleTerrainHexagons_500
                        as unsafe extern "C" fn() -> *mut cpSpace,
                ),
                updateFunc: Some(
                    update as unsafe extern "C" fn(*mut cpSpace, libc::c_double) -> (),
                ),
                drawFunc: Some(
                    ChipmunkDemoDefaultDrawImpl
                        as unsafe extern "C" fn(*mut cpSpace) -> (),
                ),
                destroyFunc: Some(destroy as unsafe extern "C" fn(*mut cpSpace) -> ()),
            };
            init
        },
        {
            let mut init = ChipmunkDemo {
                name: b"benchmark - SimpleTerrainHexagons_100\0" as *const u8
                    as *const libc::c_char,
                timestep: 1.0f64 / 60.0f64,
                initFunc: Some(
                    init_SimpleTerrainHexagons_100
                        as unsafe extern "C" fn() -> *mut cpSpace,
                ),
                updateFunc: Some(
                    update as unsafe extern "C" fn(*mut cpSpace, libc::c_double) -> (),
                ),
                drawFunc: Some(
                    ChipmunkDemoDefaultDrawImpl
                        as unsafe extern "C" fn(*mut cpSpace) -> (),
                ),
                destroyFunc: Some(destroy as unsafe extern "C" fn(*mut cpSpace) -> ()),
            };
            init
        },
        {
            let mut init = ChipmunkDemo {
                name: b"benchmark - SimpleTerrainVCircles_200\0" as *const u8
                    as *const libc::c_char,
                timestep: 1.0f64 / 60.0f64,
                initFunc: Some(
                    init_SimpleTerrainVCircles_200
                        as unsafe extern "C" fn() -> *mut cpSpace,
                ),
                updateFunc: Some(
                    update as unsafe extern "C" fn(*mut cpSpace, libc::c_double) -> (),
                ),
                drawFunc: Some(
                    ChipmunkDemoDefaultDrawImpl
                        as unsafe extern "C" fn(*mut cpSpace) -> (),
                ),
                destroyFunc: Some(destroy as unsafe extern "C" fn(*mut cpSpace) -> ()),
            };
            init
        },
        {
            let mut init = ChipmunkDemo {
                name: b"benchmark - SimpleTerrainVBoxes_200\0" as *const u8
                    as *const libc::c_char,
                timestep: 1.0f64 / 60.0f64,
                initFunc: Some(
                    init_SimpleTerrainVBoxes_200
                        as unsafe extern "C" fn() -> *mut cpSpace,
                ),
                updateFunc: Some(
                    update as unsafe extern "C" fn(*mut cpSpace, libc::c_double) -> (),
                ),
                drawFunc: Some(
                    ChipmunkDemoDefaultDrawImpl
                        as unsafe extern "C" fn(*mut cpSpace) -> (),
                ),
                destroyFunc: Some(destroy as unsafe extern "C" fn(*mut cpSpace) -> ()),
            };
            init
        },
        {
            let mut init = ChipmunkDemo {
                name: b"benchmark - SimpleTerrainVHexagons_200\0" as *const u8
                    as *const libc::c_char,
                timestep: 1.0f64 / 60.0f64,
                initFunc: Some(
                    init_SimpleTerrainVHexagons_200
                        as unsafe extern "C" fn() -> *mut cpSpace,
                ),
                updateFunc: Some(
                    update as unsafe extern "C" fn(*mut cpSpace, libc::c_double) -> (),
                ),
                drawFunc: Some(
                    ChipmunkDemoDefaultDrawImpl
                        as unsafe extern "C" fn(*mut cpSpace) -> (),
                ),
                destroyFunc: Some(destroy as unsafe extern "C" fn(*mut cpSpace) -> ()),
            };
            init
        },
        {
            let mut init = ChipmunkDemo {
                name: b"benchmark - ComplexTerrainCircles_1000\0" as *const u8
                    as *const libc::c_char,
                timestep: 1.0f64 / 60.0f64,
                initFunc: Some(
                    init_ComplexTerrainCircles_1000
                        as unsafe extern "C" fn() -> *mut cpSpace,
                ),
                updateFunc: Some(
                    update as unsafe extern "C" fn(*mut cpSpace, libc::c_double) -> (),
                ),
                drawFunc: Some(
                    ChipmunkDemoDefaultDrawImpl
                        as unsafe extern "C" fn(*mut cpSpace) -> (),
                ),
                destroyFunc: Some(destroy as unsafe extern "C" fn(*mut cpSpace) -> ()),
            };
            init
        },
        {
            let mut init = ChipmunkDemo {
                name: b"benchmark - ComplexTerrainHexagons_1000\0" as *const u8
                    as *const libc::c_char,
                timestep: 1.0f64 / 60.0f64,
                initFunc: Some(
                    init_ComplexTerrainHexagons_1000
                        as unsafe extern "C" fn() -> *mut cpSpace,
                ),
                updateFunc: Some(
                    update as unsafe extern "C" fn(*mut cpSpace, libc::c_double) -> (),
                ),
                drawFunc: Some(
                    ChipmunkDemoDefaultDrawImpl
                        as unsafe extern "C" fn(*mut cpSpace) -> (),
                ),
                destroyFunc: Some(destroy as unsafe extern "C" fn(*mut cpSpace) -> ()),
            };
            init
        },
        {
            let mut init = ChipmunkDemo {
                name: b"benchmark - BouncyTerrainCircles_500\0" as *const u8
                    as *const libc::c_char,
                timestep: 1.0f64 / 60.0f64,
                initFunc: Some(
                    init_BouncyTerrainCircles_500
                        as unsafe extern "C" fn() -> *mut cpSpace,
                ),
                updateFunc: Some(
                    update as unsafe extern "C" fn(*mut cpSpace, libc::c_double) -> (),
                ),
                drawFunc: Some(
                    ChipmunkDemoDefaultDrawImpl
                        as unsafe extern "C" fn(*mut cpSpace) -> (),
                ),
                destroyFunc: Some(destroy as unsafe extern "C" fn(*mut cpSpace) -> ()),
            };
            init
        },
        {
            let mut init = ChipmunkDemo {
                name: b"benchmark - BouncyTerrainHexagons_500\0" as *const u8
                    as *const libc::c_char,
                timestep: 1.0f64 / 60.0f64,
                initFunc: Some(
                    init_BouncyTerrainHexagons_500
                        as unsafe extern "C" fn() -> *mut cpSpace,
                ),
                updateFunc: Some(
                    update as unsafe extern "C" fn(*mut cpSpace, libc::c_double) -> (),
                ),
                drawFunc: Some(
                    ChipmunkDemoDefaultDrawImpl
                        as unsafe extern "C" fn(*mut cpSpace) -> (),
                ),
                destroyFunc: Some(destroy as unsafe extern "C" fn(*mut cpSpace) -> ()),
            };
            init
        },
        {
            let mut init = ChipmunkDemo {
                name: b"benchmark - NoCollide\0" as *const u8 as *const libc::c_char,
                timestep: 1.0f64 / 60.0f64,
                initFunc: Some(init_NoCollide as unsafe extern "C" fn() -> *mut cpSpace),
                updateFunc: Some(
                    update as unsafe extern "C" fn(*mut cpSpace, libc::c_double) -> (),
                ),
                drawFunc: Some(
                    ChipmunkDemoDefaultDrawImpl
                        as unsafe extern "C" fn(*mut cpSpace) -> (),
                ),
                destroyFunc: Some(destroy as unsafe extern "C" fn(*mut cpSpace) -> ()),
            };
            init
        },
    ]
};
pub static mut bench_count: libc::c_int = 0;
unsafe extern "C" fn run_static_initializers() {
    simple_terrain_count = (::std::mem::size_of::<[cpVect; 48]>() as libc::c_ulong)
        .wrapping_div(::std::mem::size_of::<cpVect>() as libc::c_ulong) as libc::c_int;
    complex_terrain_count = (::std::mem::size_of::<[cpVect; 254]>() as libc::c_ulong)
        .wrapping_div(::std::mem::size_of::<cpVect>() as libc::c_ulong) as libc::c_int;
    bouncy_terrain_count = (::std::mem::size_of::<[cpVect; 516]>() as libc::c_ulong)
        .wrapping_div(::std::mem::size_of::<cpVect>() as libc::c_ulong) as libc::c_int;
    bench_count = (::std::mem::size_of::<[ChipmunkDemo; 17]>() as libc::c_ulong)
        .wrapping_div(::std::mem::size_of::<ChipmunkDemo>() as libc::c_ulong)
        as libc::c_int;
}
#[used]
#[cfg_attr(target_os = "linux", link_section = ".init_array")]
#[cfg_attr(target_os = "windows", link_section = ".CRT$XIB")]
#[cfg_attr(target_os = "macos", link_section = "__DATA,__mod_init_func")]
static INIT_ARRAY: [unsafe extern "C" fn(); 1] = [run_static_initializers];
