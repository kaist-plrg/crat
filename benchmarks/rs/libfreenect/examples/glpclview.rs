use ::libc;
extern "C" {
    fn freenect_sync_get_video(
        video: *mut *mut libc::c_void,
        timestamp: *mut uint32_t,
        index: libc::c_int,
        fmt: freenect_video_format,
    ) -> libc::c_int;
    fn freenect_sync_get_depth(
        depth: *mut *mut libc::c_void,
        timestamp: *mut uint32_t,
        index: libc::c_int,
        fmt: freenect_depth_format,
    ) -> libc::c_int;
    fn freenect_sync_stop();
    fn printf(_: *const libc::c_char, _: ...) -> libc::c_int;
    fn exit(_: libc::c_int) -> !;
    fn glTranslatef(x: GLfloat, y: GLfloat, z: GLfloat);
    fn glScalef(x: GLfloat, y: GLfloat, z: GLfloat);
    fn glRotatef(angle: GLfloat, x: GLfloat, y: GLfloat, z: GLfloat);
    fn glMultMatrixf(m: *const GLfloat);
    fn glLoadIdentity();
    fn glPopMatrix();
    fn glPushMatrix();
    fn glViewport(x: GLint, y: GLint, width: GLsizei, height: GLsizei);
    fn glMatrixMode(mode: GLenum);
    fn glEnableClientState(cap: GLenum);
    fn glDisable(cap: GLenum);
    fn glEnable(cap: GLenum);
    fn glPointSize(size: GLfloat);
    fn glClear(mask: GLbitfield);
    fn glClearColor(red: GLclampf, green: GLclampf, blue: GLclampf, alpha: GLclampf);
    fn glutMotionFunc(
        callback: Option::<unsafe extern "C" fn(libc::c_int, libc::c_int) -> ()>,
    );
    fn glutMouseFunc(
        callback: Option::<
            unsafe extern "C" fn(
                libc::c_int,
                libc::c_int,
                libc::c_int,
                libc::c_int,
            ) -> (),
        >,
    );
    fn glutDisplayFunc(callback: Option::<unsafe extern "C" fn() -> ()>);
    fn glutReshapeFunc(
        callback: Option::<unsafe extern "C" fn(libc::c_int, libc::c_int) -> ()>,
    );
    fn glutKeyboardFunc(
        callback: Option::<
            unsafe extern "C" fn(libc::c_uchar, libc::c_int, libc::c_int) -> (),
        >,
    );
    fn glutIdleFunc(callback: Option::<unsafe extern "C" fn() -> ()>);
    fn glVertexPointer(
        size: GLint,
        type_0: GLenum,
        stride: GLsizei,
        ptr: *const libc::c_void,
    );
    fn glTexCoordPointer(
        size: GLint,
        type_0: GLenum,
        stride: GLsizei,
        ptr: *const libc::c_void,
    );
    fn glDrawElements(
        mode: GLenum,
        count: GLsizei,
        type_0: GLenum,
        indices: *const libc::c_void,
    );
    fn glTexParameteri(target: GLenum, pname: GLenum, param: GLint);
    fn glTexImage2D(
        target: GLenum,
        level: GLint,
        internalFormat: GLint,
        width: GLsizei,
        height: GLsizei,
        border: GLint,
        format: GLenum,
        type_0: GLenum,
        pixels: *const libc::c_void,
    );
    fn glGenTextures(n: GLsizei, textures: *mut GLuint);
    fn glBindTexture(target: GLenum, texture: GLuint);
    fn glutSwapBuffers();
    fn gluPerspective(fovy: GLdouble, aspect: GLdouble, zNear: GLdouble, zFar: GLdouble);
    fn glutInit(pargc: *mut libc::c_int, argv: *mut *mut libc::c_char);
    fn glutInitWindowPosition(x: libc::c_int, y: libc::c_int);
    fn glutInitWindowSize(width: libc::c_int, height: libc::c_int);
    fn glutInitDisplayMode(displayMode: libc::c_uint);
    fn glutMainLoop();
    fn glutCreateWindow(title: *const libc::c_char) -> libc::c_int;
    fn glutDestroyWindow(window_0: libc::c_int);
}
pub type __uint32_t = libc::c_uint;
pub type uint32_t = __uint32_t;
pub type freenect_video_format = libc::c_uint;
pub const FREENECT_VIDEO_DUMMY: freenect_video_format = 2147483647;
pub const FREENECT_VIDEO_YUV_RAW: freenect_video_format = 6;
pub const FREENECT_VIDEO_YUV_RGB: freenect_video_format = 5;
pub const FREENECT_VIDEO_IR_10BIT_PACKED: freenect_video_format = 4;
pub const FREENECT_VIDEO_IR_10BIT: freenect_video_format = 3;
pub const FREENECT_VIDEO_IR_8BIT: freenect_video_format = 2;
pub const FREENECT_VIDEO_BAYER: freenect_video_format = 1;
pub const FREENECT_VIDEO_RGB: freenect_video_format = 0;
pub type freenect_depth_format = libc::c_uint;
pub const FREENECT_DEPTH_DUMMY: freenect_depth_format = 2147483647;
pub const FREENECT_DEPTH_MM: freenect_depth_format = 5;
pub const FREENECT_DEPTH_REGISTERED: freenect_depth_format = 4;
pub const FREENECT_DEPTH_10BIT_PACKED: freenect_depth_format = 3;
pub const FREENECT_DEPTH_11BIT_PACKED: freenect_depth_format = 2;
pub const FREENECT_DEPTH_10BIT: freenect_depth_format = 1;
pub const FREENECT_DEPTH_11BIT: freenect_depth_format = 0;
pub type GLenum = libc::c_uint;
pub type GLbitfield = libc::c_uint;
pub type GLvoid = ();
pub type GLint = libc::c_int;
pub type GLuint = libc::c_uint;
pub type GLsizei = libc::c_int;
pub type GLfloat = libc::c_float;
pub type GLclampf = libc::c_float;
pub type GLdouble = libc::c_double;
pub static mut window: libc::c_int = 0;
pub static mut gl_rgb_tex: GLuint = 0;
pub static mut mx: libc::c_int = -(1 as libc::c_int);
pub static mut my: libc::c_int = -(1 as libc::c_int);
pub static mut rotangles: [libc::c_int; 2] = [0 as libc::c_int, 0];
pub static mut zoom: libc::c_float = 1 as libc::c_int as libc::c_float;
pub static mut color: libc::c_int = 1 as libc::c_int;
pub unsafe extern "C" fn LoadVertexMatrix() {
    let mut fx: libc::c_float = 594.21f32;
    let mut fy: libc::c_float = 591.04f32;
    let mut a: libc::c_float = -0.0030711f32;
    let mut b: libc::c_float = 3.3309495f32;
    let mut cx: libc::c_float = 339.5f32;
    let mut cy: libc::c_float = 242.7f32;
    let mut mat: [GLfloat; 16] = [
        1 as libc::c_int as libc::c_float / fx,
        0 as libc::c_int as GLfloat,
        0 as libc::c_int as GLfloat,
        0 as libc::c_int as GLfloat,
        0 as libc::c_int as GLfloat,
        -(1 as libc::c_int) as libc::c_float / fy,
        0 as libc::c_int as GLfloat,
        0 as libc::c_int as GLfloat,
        0 as libc::c_int as GLfloat,
        0 as libc::c_int as GLfloat,
        0 as libc::c_int as GLfloat,
        a,
        -cx / fx,
        cy / fy,
        -(1 as libc::c_int) as GLfloat,
        b,
    ];
    glMultMatrixf(mat.as_mut_ptr());
}
pub unsafe extern "C" fn LoadRGBMatrix() {
    let mut mat: [libc::c_float; 16] = [
        5.34866271e+02f64 as libc::c_float,
        3.89654806e+00f64 as libc::c_float,
        0.00000000e+00f64 as libc::c_float,
        1.74704200e-02f64 as libc::c_float,
        -4.70724694e+00f64 as libc::c_float,
        -5.28843603e+02f64 as libc::c_float,
        0.00000000e+00f64 as libc::c_float,
        -1.22753400e-02f64 as libc::c_float,
        -3.19670762e+02f64 as libc::c_float,
        -2.60999685e+02f64 as libc::c_float,
        0.00000000e+00f64 as libc::c_float,
        -9.99772000e-01f64 as libc::c_float,
        -6.98445586e+00f64 as libc::c_float,
        3.31139785e+00f64 as libc::c_float,
        0.00000000e+00f64 as libc::c_float,
        1.09167360e-02f64 as libc::c_float,
    ];
    glMultMatrixf(mat.as_mut_ptr());
}
pub unsafe extern "C" fn mouseMoved(mut x: libc::c_int, mut y: libc::c_int) {
    if mx >= 0 as libc::c_int && my >= 0 as libc::c_int {
        rotangles[0 as libc::c_int as usize] += y - my;
        rotangles[1 as libc::c_int as usize] += x - mx;
    }
    mx = x;
    my = y;
}
pub unsafe extern "C" fn mousePress(
    mut button: libc::c_int,
    mut state: libc::c_int,
    mut x: libc::c_int,
    mut y: libc::c_int,
) {
    if button == 0 as libc::c_int && state == 0 as libc::c_int {
        mx = x;
        my = y;
    }
    if button == 0 as libc::c_int && state == 0x1 as libc::c_int {
        mx = -(1 as libc::c_int);
        my = -(1 as libc::c_int);
    }
}
pub unsafe extern "C" fn no_kinect_quit() {
    printf(b"Error: Kinect not connected?\n\0" as *const u8 as *const libc::c_char);
    exit(1 as libc::c_int);
}
pub unsafe extern "C" fn DrawGLScene() {
    let mut depth: *mut libc::c_short = 0 as *mut libc::c_short;
    let mut rgb: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut ts: uint32_t = 0;
    if freenect_sync_get_depth(
        &mut depth as *mut *mut libc::c_short as *mut *mut libc::c_void,
        &mut ts,
        0 as libc::c_int,
        FREENECT_DEPTH_11BIT,
    ) < 0 as libc::c_int
    {
        no_kinect_quit();
    }
    if freenect_sync_get_video(
        &mut rgb as *mut *mut libc::c_char as *mut *mut libc::c_void,
        &mut ts,
        0 as libc::c_int,
        FREENECT_VIDEO_RGB,
    ) < 0 as libc::c_int
    {
        no_kinect_quit();
    }
    static mut indices: [[libc::c_uint; 640]; 480] = [[0; 640]; 480];
    static mut xyz: [[[libc::c_short; 3]; 640]; 480] = [[[0; 3]; 640]; 480];
    let mut i: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    i = 0 as libc::c_int;
    while i < 480 as libc::c_int {
        j = 0 as libc::c_int;
        while j < 640 as libc::c_int {
            xyz[i as usize][j as usize][0 as libc::c_int as usize] = j as libc::c_short;
            xyz[i as usize][j as usize][1 as libc::c_int as usize] = i as libc::c_short;
            xyz[i
                as usize][j
                as usize][2 as libc::c_int
                as usize] = *depth.offset((i * 640 as libc::c_int + j) as isize);
            indices[i
                as usize][j as usize] = (i * 640 as libc::c_int + j) as libc::c_uint;
            j += 1;
            j;
        }
        i += 1;
        i;
    }
    glClear((0x4000 as libc::c_int | 0x100 as libc::c_int) as GLbitfield);
    glLoadIdentity();
    glPushMatrix();
    glScalef(zoom, zoom, 1 as libc::c_int as GLfloat);
    glTranslatef(
        0 as libc::c_int as GLfloat,
        0 as libc::c_int as GLfloat,
        -3.5f64 as GLfloat,
    );
    glRotatef(
        rotangles[0 as libc::c_int as usize] as GLfloat,
        1 as libc::c_int as GLfloat,
        0 as libc::c_int as GLfloat,
        0 as libc::c_int as GLfloat,
    );
    glRotatef(
        rotangles[1 as libc::c_int as usize] as GLfloat,
        0 as libc::c_int as GLfloat,
        1 as libc::c_int as GLfloat,
        0 as libc::c_int as GLfloat,
    );
    glTranslatef(
        0 as libc::c_int as GLfloat,
        0 as libc::c_int as GLfloat,
        1.5f64 as GLfloat,
    );
    LoadVertexMatrix();
    glMatrixMode(0x1702 as libc::c_int as GLenum);
    glLoadIdentity();
    glScalef(
        1 as libc::c_int as libc::c_float / 640.0f32,
        1 as libc::c_int as libc::c_float / 480.0f32,
        1 as libc::c_int as GLfloat,
    );
    LoadRGBMatrix();
    LoadVertexMatrix();
    glMatrixMode(0x1700 as libc::c_int as GLenum);
    glPointSize(1 as libc::c_int as GLfloat);
    glEnableClientState(0x8074 as libc::c_int as GLenum);
    glVertexPointer(
        3 as libc::c_int,
        0x1402 as libc::c_int as GLenum,
        0 as libc::c_int,
        xyz.as_mut_ptr() as *const libc::c_void,
    );
    glEnableClientState(0x8078 as libc::c_int as GLenum);
    glTexCoordPointer(
        3 as libc::c_int,
        0x1402 as libc::c_int as GLenum,
        0 as libc::c_int,
        xyz.as_mut_ptr() as *const libc::c_void,
    );
    if color != 0 {
        glEnable(0xde1 as libc::c_int as GLenum);
    }
    glBindTexture(0xde1 as libc::c_int as GLenum, gl_rgb_tex);
    glTexImage2D(
        0xde1 as libc::c_int as GLenum,
        0 as libc::c_int,
        3 as libc::c_int,
        640 as libc::c_int,
        480 as libc::c_int,
        0 as libc::c_int,
        0x1907 as libc::c_int as GLenum,
        0x1401 as libc::c_int as GLenum,
        rgb as *const libc::c_void,
    );
    glPointSize(2.0f32);
    glDrawElements(
        0 as libc::c_int as GLenum,
        640 as libc::c_int * 480 as libc::c_int,
        0x1405 as libc::c_int as GLenum,
        indices.as_mut_ptr() as *const libc::c_void,
    );
    glPopMatrix();
    glDisable(0xde1 as libc::c_int as GLenum);
    glutSwapBuffers();
}
pub unsafe extern "C" fn keyPressed(
    mut key: libc::c_uchar,
    mut x: libc::c_int,
    mut y: libc::c_int,
) {
    if key as libc::c_int == 27 as libc::c_int {
        freenect_sync_stop();
        glutDestroyWindow(window);
        exit(0 as libc::c_int);
    }
    if key as libc::c_int == 'w' as i32 {
        zoom *= 1.1f32;
    }
    if key as libc::c_int == 's' as i32 {
        zoom /= 1.1f32;
    }
    if key as libc::c_int == 'c' as i32 {
        color = (color == 0) as libc::c_int;
    }
}
pub unsafe extern "C" fn ReSizeGLScene(mut Width: libc::c_int, mut Height: libc::c_int) {
    glViewport(0 as libc::c_int, 0 as libc::c_int, Width, Height);
    glMatrixMode(0x1701 as libc::c_int as GLenum);
    glLoadIdentity();
    gluPerspective(
        60 as libc::c_int as GLdouble,
        4 as libc::c_int as libc::c_double / 3.0f64,
        0.3f64,
        200 as libc::c_int as GLdouble,
    );
    glMatrixMode(0x1700 as libc::c_int as GLenum);
}
pub unsafe extern "C" fn InitGL(mut Width: libc::c_int, mut Height: libc::c_int) {
    glClearColor(0.0f32, 0.0f32, 0.0f32, 0.0f32);
    glEnable(0xb71 as libc::c_int as GLenum);
    glGenTextures(1 as libc::c_int, &mut gl_rgb_tex);
    glBindTexture(0xde1 as libc::c_int as GLenum, gl_rgb_tex);
    glTexParameteri(
        0xde1 as libc::c_int as GLenum,
        0x2801 as libc::c_int as GLenum,
        0x2601 as libc::c_int,
    );
    glTexParameteri(
        0xde1 as libc::c_int as GLenum,
        0x2800 as libc::c_int as GLenum,
        0x2601 as libc::c_int,
    );
    ReSizeGLScene(Width, Height);
}
unsafe fn main_0(
    mut argc: libc::c_int,
    mut argv: *mut *mut libc::c_char,
) -> libc::c_int {
    glutInit(&mut argc, argv);
    glutInitDisplayMode(
        (0 as libc::c_int | 0x2 as libc::c_int | 0x8 as libc::c_int
            | 0x10 as libc::c_int) as libc::c_uint,
    );
    glutInitWindowSize(640 as libc::c_int, 480 as libc::c_int);
    glutInitWindowPosition(0 as libc::c_int, 0 as libc::c_int);
    window = glutCreateWindow(b"LibFreenect\0" as *const u8 as *const libc::c_char);
    glutDisplayFunc(
        ::std::mem::transmute::<
            Option::<unsafe extern "C" fn() -> ()>,
            Option::<unsafe extern "C" fn() -> ()>,
        >(
            Some(
                ::std::mem::transmute::<
                    unsafe extern "C" fn() -> (),
                    unsafe extern "C" fn() -> (),
                >(DrawGLScene),
            ),
        ),
    );
    glutIdleFunc(
        ::std::mem::transmute::<
            Option::<unsafe extern "C" fn() -> ()>,
            Option::<unsafe extern "C" fn() -> ()>,
        >(
            Some(
                ::std::mem::transmute::<
                    unsafe extern "C" fn() -> (),
                    unsafe extern "C" fn() -> (),
                >(DrawGLScene),
            ),
        ),
    );
    glutReshapeFunc(
        Some(ReSizeGLScene as unsafe extern "C" fn(libc::c_int, libc::c_int) -> ()),
    );
    glutKeyboardFunc(
        Some(
            keyPressed
                as unsafe extern "C" fn(libc::c_uchar, libc::c_int, libc::c_int) -> (),
        ),
    );
    glutMotionFunc(
        Some(mouseMoved as unsafe extern "C" fn(libc::c_int, libc::c_int) -> ()),
    );
    glutMouseFunc(
        Some(
            mousePress
                as unsafe extern "C" fn(
                    libc::c_int,
                    libc::c_int,
                    libc::c_int,
                    libc::c_int,
                ) -> (),
        ),
    );
    InitGL(640 as libc::c_int, 480 as libc::c_int);
    glutMainLoop();
    return 0 as libc::c_int;
}
pub fn main() {
    let mut args: Vec::<*mut libc::c_char> = Vec::new();
    for arg in ::std::env::args_os() {
        args.push(
            (::std::ffi::CString::new(
                ::std::os::unix::ffi::OsStrExt::as_bytes(arg.as_os_str()),
            ))
                .unwrap()
                .into_raw(),
        );
    }
    args.push(::std::ptr::null_mut());
    unsafe {
        ::std::process::exit(
            main_0(
                (args.len() - 1) as libc::c_int,
                args.as_mut_ptr() as *mut *mut libc::c_char,
            ) as i32,
        )
    }
}
