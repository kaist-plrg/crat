use ::libc;
extern "C" {
    pub type cpBody;
    pub type cpShape;
    pub type cpSpace;
    fn memcpy(
        _: *mut libc::c_void,
        _: *const libc::c_void,
        _: libc::c_ulong,
    ) -> *mut libc::c_void;
    fn rand() -> libc::c_int;
    fn cpSpaceSetSleepTimeThreshold(space: *mut cpSpace, sleepTimeThreshold: cpFloat);
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
    fn cpShapeGetBB(shape: *const cpShape) -> cpBB;
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
    fn cpMomentForBox(m: cpFloat, width: cpFloat, height: cpFloat) -> cpFloat;
    fn cpCentroidForPoly(count: libc::c_int, verts: *const cpVect) -> cpVect;
    fn cpSpaceNew() -> *mut cpSpace;
    fn cpSpaceFree(space: *mut cpSpace);
    fn cpSpaceSetIterations(space: *mut cpSpace, iterations: libc::c_int);
    fn cpSpaceSetGravity(space: *mut cpSpace, gravity: cpVect);
    fn cpSpaceSetCollisionSlop(space: *mut cpSpace, collisionSlop: cpFloat);
    fn cpSpaceGetStaticBody(space: *const cpSpace) -> *mut cpBody;
    fn cpSpaceAddShape(space: *mut cpSpace, shape: *mut cpShape) -> *mut cpShape;
    fn cpSpaceAddBody(space: *mut cpSpace, body: *mut cpBody) -> *mut cpBody;
    fn cpSpaceRemoveShape(space: *mut cpSpace, shape: *mut cpShape);
    fn cpSpaceRemoveBody(space: *mut cpSpace, body: *mut cpBody);
    fn cpSpacePointQueryNearest(
        space: *mut cpSpace,
        point: cpVect,
        maxDistance: cpFloat,
        filter: cpShapeFilter,
        out: *mut cpPointQueryInfo,
    ) -> *mut cpShape;
    fn cpSpaceStep(space: *mut cpSpace, dt: cpFloat);
    fn cpMomentForPoly(
        m: cpFloat,
        count: libc::c_int,
        verts: *const cpVect,
        offset: cpVect,
        radius: cpFloat,
    ) -> cpFloat;
    fn cpAreaForPoly(
        count: libc::c_int,
        verts: *const cpVect,
        radius: cpFloat,
    ) -> cpFloat;
    static mut ChipmunkDemoMessageString: *const libc::c_char;
    static mut ChipmunkDemoMouse: cpVect;
    static mut ChipmunkDemoRightDown: cpBool;
    static mut GRAB_FILTER: cpShapeFilter;
    static mut NOT_GRABBABLE_FILTER: cpShapeFilter;
    fn ChipmunkDemoDefaultDrawImpl(space: *mut cpSpace);
    fn ChipmunkDemoFreeSpaceChildren(space: *mut cpSpace);
}
pub type __uint32_t = libc::c_uint;
pub type uint32_t = __uint32_t;
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct WorleyContex {
    pub seed: uint32_t,
    pub cellSize: cpFloat,
    pub width: libc::c_int,
    pub height: libc::c_int,
    pub bb: cpBB,
    pub focus: cpVect,
}
#[inline]
unsafe extern "C" fn cpfmax(mut a: cpFloat, mut b: cpFloat) -> cpFloat {
    return if a > b { a } else { b };
}
#[inline]
unsafe extern "C" fn cpfabs(mut f: cpFloat) -> cpFloat {
    return if f < 0 as libc::c_int as libc::c_double { -f } else { f };
}
#[inline]
unsafe extern "C" fn cpflerp(
    mut f1: cpFloat,
    mut f2: cpFloat,
    mut t: cpFloat,
) -> cpFloat {
    return f1 * (1.0f32 as libc::c_double - t) + f2 * t;
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
unsafe extern "C" fn cpvlerp(v1: cpVect, v2: cpVect, t: cpFloat) -> cpVect {
    return cpvadd(cpvmult(v1, 1.0f32 as libc::c_double - t), cpvmult(v2, t));
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
unsafe extern "C" fn HashVect(
    mut x: uint32_t,
    mut y: uint32_t,
    mut seed: uint32_t,
) -> cpVect {
    let mut border: cpFloat = 0.05f32 as cpFloat;
    let mut h: uint32_t = ((x.wrapping_mul(1640531513 as libc::c_int as libc::c_uint)
        as libc::c_long ^ y as libc::c_long * 2654435789 as libc::c_long)
        + seed as libc::c_long) as uint32_t;
    return cpv(
        cpflerp(
            border,
            1.0f32 as libc::c_double - border,
            (h & 0xffff as libc::c_int as libc::c_uint) as cpFloat
                / 0xffff as libc::c_int as cpFloat,
        ),
        cpflerp(
            border,
            1.0f32 as libc::c_double - border,
            (h >> 16 as libc::c_int & 0xffff as libc::c_int as libc::c_uint) as cpFloat
                / 0xffff as libc::c_int as cpFloat,
        ),
    );
}
unsafe extern "C" fn WorleyPoint(
    mut i: libc::c_int,
    mut j: libc::c_int,
    mut context: *mut WorleyContex,
) -> cpVect {
    let mut size: cpFloat = (*context).cellSize;
    let mut width: libc::c_int = (*context).width;
    let mut height: libc::c_int = (*context).height;
    let mut bb: cpBB = (*context).bb;
    let mut fv: cpVect = HashVect(i as uint32_t, j as uint32_t, (*context).seed);
    return cpv(
        cpflerp(bb.l, bb.r, 0.5f32 as cpFloat)
            + size
                * (i as libc::c_double + fv.x
                    - (width as libc::c_float * 0.5f32) as libc::c_double),
        cpflerp(bb.b, bb.t, 0.5f32 as cpFloat)
            + size
                * (j as libc::c_double + fv.y
                    - (height as libc::c_float * 0.5f32) as libc::c_double),
    );
}
unsafe extern "C" fn ClipCell(
    mut shape: *mut cpShape,
    mut center: cpVect,
    mut i: libc::c_int,
    mut j: libc::c_int,
    mut context: *mut WorleyContex,
    mut verts: *mut cpVect,
    mut clipped: *mut cpVect,
    mut count: libc::c_int,
) -> libc::c_int {
    let mut other: cpVect = WorleyPoint(i, j, context);
    if cpShapePointQuery(shape, other, 0 as *mut cpPointQueryInfo)
        > 0.0f32 as libc::c_double
    {
        memcpy(
            clipped as *mut libc::c_void,
            verts as *const libc::c_void,
            (count as libc::c_ulong)
                .wrapping_mul(::std::mem::size_of::<cpVect>() as libc::c_ulong),
        );
        return count;
    }
    let mut n: cpVect = cpvsub(other, center);
    let mut dist: cpFloat = cpvdot(n, cpvlerp(center, other, 0.5f32 as cpFloat));
    let mut clipped_count: libc::c_int = 0 as libc::c_int;
    let mut j_0: libc::c_int = 0 as libc::c_int;
    let mut i_0: libc::c_int = count - 1 as libc::c_int;
    while j_0 < count {
        let mut a: cpVect = *verts.offset(i_0 as isize);
        let mut a_dist: cpFloat = cpvdot(a, n) - dist;
        if a_dist <= 0.0f64 {
            *clipped.offset(clipped_count as isize) = a;
            clipped_count += 1;
            clipped_count;
        }
        let mut b: cpVect = *verts.offset(j_0 as isize);
        let mut b_dist: cpFloat = cpvdot(b, n) - dist;
        if a_dist * b_dist < 0.0f32 as libc::c_double {
            let mut t: cpFloat = cpfabs(a_dist) / (cpfabs(a_dist) + cpfabs(b_dist));
            *clipped.offset(clipped_count as isize) = cpvlerp(a, b, t);
            clipped_count += 1;
            clipped_count;
        }
        i_0 = j_0;
        j_0 += 1;
        j_0;
    }
    return clipped_count;
}
unsafe extern "C" fn ShatterCell(
    mut space: *mut cpSpace,
    mut shape: *mut cpShape,
    mut cell: cpVect,
    mut cell_i: libc::c_int,
    mut cell_j: libc::c_int,
    mut context: *mut WorleyContex,
) {
    let mut body: *mut cpBody = cpShapeGetBody(shape);
    let mut fresh0 = ::std::vec::from_elem(
        0,
        (16 as libc::c_int as libc::c_ulong)
            .wrapping_mul(::std::mem::size_of::<cpVect>() as libc::c_ulong) as usize,
    );
    let mut ping: *mut cpVect = fresh0.leak().as_mut_ptr() as *mut cpVect;
    let mut fresh1 = ::std::vec::from_elem(
        0,
        (16 as libc::c_int as libc::c_ulong)
            .wrapping_mul(::std::mem::size_of::<cpVect>() as libc::c_ulong) as usize,
    );
    let mut pong: *mut cpVect = fresh1.leak().as_mut_ptr() as *mut cpVect;
    let mut count: libc::c_int = cpPolyShapeGetCount(shape);
    count = if count > 16 as libc::c_int { 16 as libc::c_int } else { count };
    let mut i: libc::c_int = 0 as libc::c_int;
    while i < count {
        *ping
            .offset(i as isize) = cpBodyLocalToWorld(body, cpPolyShapeGetVert(shape, i));
        i += 1;
        i;
    }
    let mut i_0: libc::c_int = 0 as libc::c_int;
    while i_0 < (*context).width {
        let mut j: libc::c_int = 0 as libc::c_int;
        while j < (*context).height {
            if !(i_0 == cell_i && j == cell_j)
                && cpShapePointQuery(shape, cell, 0 as *mut cpPointQueryInfo)
                    < 0.0f32 as libc::c_double
            {
                count = ClipCell(shape, cell, i_0, j, context, ping, pong, count);
                memcpy(
                    ping as *mut libc::c_void,
                    pong as *const libc::c_void,
                    (count as libc::c_ulong)
                        .wrapping_mul(::std::mem::size_of::<cpVect>() as libc::c_ulong),
                );
            }
            j += 1;
            j;
        }
        i_0 += 1;
        i_0;
    }
    let mut centroid: cpVect = cpCentroidForPoly(count, ping);
    let mut mass: cpFloat = cpAreaForPoly(count, ping, 0.0f32 as cpFloat)
        * (1.0f64 / 10000.0f64);
    let mut moment: cpFloat = cpMomentForPoly(
        mass,
        count,
        ping,
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
        cpPolyShapeNew(new_body, count, ping, transform, 0.0f64),
    );
    cpShapeSetFriction(new_shape, cpShapeGetFriction(shape));
}
unsafe extern "C" fn ShatterShape(
    mut space: *mut cpSpace,
    mut shape: *mut cpShape,
    mut cellSize: cpFloat,
    mut focus: cpVect,
) {
    cpSpaceRemoveShape(space, shape);
    cpSpaceRemoveBody(space, cpShapeGetBody(shape));
    let mut bb: cpBB = cpShapeGetBB(shape);
    let mut width: libc::c_int = ((bb.r - bb.l) / cellSize) as libc::c_int
        + 1 as libc::c_int;
    let mut height: libc::c_int = ((bb.t - bb.b) / cellSize) as libc::c_int
        + 1 as libc::c_int;
    let mut context: WorleyContex = {
        let mut init = WorleyContex {
            seed: rand() as uint32_t,
            cellSize: cellSize,
            width: width,
            height: height,
            bb: bb,
            focus: focus,
        };
        init
    };
    let mut i: libc::c_int = 0 as libc::c_int;
    while i < context.width {
        let mut j: libc::c_int = 0 as libc::c_int;
        while j < context.height {
            let mut cell: cpVect = WorleyPoint(i, j, &mut context);
            if cpShapePointQuery(shape, cell, 0 as *mut cpPointQueryInfo)
                < 0.0f32 as libc::c_double
            {
                ShatterCell(space, shape, cell, i, j, &mut context);
            }
            j += 1;
            j;
        }
        i += 1;
        i;
    }
    cpBodyFree(cpShapeGetBody(shape));
    cpShapeFree(shape);
}
unsafe extern "C" fn update(mut space: *mut cpSpace, mut dt: libc::c_double) {
    cpSpaceStep(space, dt);
    if ChipmunkDemoRightDown != 0 {
        let mut info: cpPointQueryInfo = cpPointQueryInfo {
            shape: 0 as *const cpShape,
            point: cpVect { x: 0., y: 0. },
            distance: 0.,
            gradient: cpVect { x: 0., y: 0. },
        };
        if !(cpSpacePointQueryNearest(
            space,
            ChipmunkDemoMouse,
            0 as libc::c_int as cpFloat,
            GRAB_FILTER,
            &mut info,
        ))
            .is_null()
        {
            let mut bb: cpBB = cpShapeGetBB(info.shape);
            let mut cell_size: cpFloat = cpfmax(bb.r - bb.l, bb.t - bb.b)
                / 5.0f32 as libc::c_double;
            if cell_size > 5.0f32 as libc::c_double {
                ShatterShape(
                    space,
                    info.shape as *mut cpShape,
                    cell_size,
                    ChipmunkDemoMouse,
                );
            }
        }
    }
}
unsafe extern "C" fn init() -> *mut cpSpace {
    ChipmunkDemoMessageString = b"Right click something to shatter it.\0" as *const u8
        as *const libc::c_char;
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
    let mut height: cpFloat = 200.0f32 as cpFloat;
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
pub static mut Shatter: ChipmunkDemo = unsafe {
    {
        let mut init = ChipmunkDemo {
            name: b"Shatter.\0" as *const u8 as *const libc::c_char,
            timestep: (1.0f32 / 60.0f32) as libc::c_double,
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
